extern "C" {
    pub type internal_state;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn zcalloc(opaque: voidpf, items: ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> voidpf;
    fn zcfree(opaque: voidpf, ptr: voidpf);
    fn inflate_table(
        type_0: codetype,
        lens: *mut ::core::ffi::c_ushort,
        codes: ::core::ffi::c_uint,
        table: *mut *mut code,
        bits: *mut ::core::ffi::c_uint,
        work: *mut ::core::ffi::c_ushort,
    ) -> ::core::ffi::c_int;
    fn inflate_fast(strm: z_streamp, start: ::core::ffi::c_uint);
}
pub type size_t = usize;
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
pub type in_func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uint,
>;
pub type out_func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_uchar,
        ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int,
>;
pub const BAD: inflate_mode = 16209;
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
pub type codetype = ::core::ffi::c_uint;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
pub const ZLIB_VERSION: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"1.2.11-optipng\0") };
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Z_STREAM_ERROR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const Z_DATA_ERROR: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const Z_MEM_ERROR: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const Z_BUF_ERROR: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
pub const Z_VERSION_ERROR: ::core::ffi::c_int = -(6 as ::core::ffi::c_int);
pub const Z_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn inflateBackInit_(
    mut strm: z_streamp,
    mut windowBits: ::core::ffi::c_int,
    mut window: *mut ::core::ffi::c_uchar,
    mut version: *const ::core::ffi::c_char,
    mut stream_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    if version.is_null()
        || *version.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != ZLIB_VERSION[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        || stream_size != ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int
    {
        return Z_VERSION_ERROR;
    }
    if strm.is_null()
        || window.is_null()
        || windowBits < 8 as ::core::ffi::c_int
        || windowBits > 15 as ::core::ffi::c_int
    {
        return Z_STREAM_ERROR;
    }
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*strm).zalloc.is_none() {
        (*strm).zalloc = Some(
            zcalloc
                as unsafe extern "C" fn(voidpf, ::core::ffi::c_uint, ::core::ffi::c_uint) -> voidpf,
        ) as alloc_func;
        (*strm).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
    (*state).dmax = 32768 as ::core::ffi::c_uint;
    (*state).wbits = windowBits as uInt as ::core::ffi::c_uint;
    (*state).wsize = (1 as ::core::ffi::c_uint) << windowBits;
    (*state).window = window;
    (*state).wnext = 0 as ::core::ffi::c_uint;
    (*state).whave = 0 as ::core::ffi::c_uint;
    return Z_OK;
}
unsafe extern "C" fn fixedtables(mut state: *mut inflate_state) {
    static mut lenfix: [code; 512] = [
        code {
            op: 96 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 80 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 16 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 115 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 112 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 48 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 192 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 96 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 32 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 160 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 128 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 64 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 224 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 88 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 24 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 144 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 120 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 56 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 208 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 104 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 40 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 176 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 136 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 72 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 240 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 84 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 20 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 227 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 116 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 52 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 200 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 100 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 36 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 168 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 132 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 68 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 232 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 92 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 28 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 152 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 124 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 60 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 216 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 108 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 44 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 184 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 12 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 140 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 76 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 248 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 82 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 18 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 163 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 114 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 50 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 196 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 98 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 34 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 164 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 2 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 130 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 66 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 228 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 90 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 26 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 148 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 122 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 58 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 212 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 106 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 42 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 180 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 138 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 74 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 244 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 86 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 22 as ::core::ffi::c_ushort,
        },
        code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 118 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 54 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 204 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 102 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 38 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 172 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 134 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 70 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 236 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 94 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 30 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 156 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 126 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 62 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 220 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 110 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 46 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 188 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 14 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 142 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 78 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 252 as ::core::ffi::c_ushort,
        },
        code {
            op: 96 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 81 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 131 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 113 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 49 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 194 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 97 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 33 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 162 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 1 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 129 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 65 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 226 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 89 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 25 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 146 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 121 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 57 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 210 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 105 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 41 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 178 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 137 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 73 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 242 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 85 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 21 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 258 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 117 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 53 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 202 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 101 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 37 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 170 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 133 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 69 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 234 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 93 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 29 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 154 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 125 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 61 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 218 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 109 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 45 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 186 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 141 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 77 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 250 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 195 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 115 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 198 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 166 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 131 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 230 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 91 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 150 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 123 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 214 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 107 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 182 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 139 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 75 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 246 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 87 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 119 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 55 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 206 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 103 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 39 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 174 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 135 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 71 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 238 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 95 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 158 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 127 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 63 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 222 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 111 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 47 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 190 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 143 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 79 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 254 as ::core::ffi::c_ushort,
        },
        code {
            op: 96 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 80 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 16 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 115 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 112 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 48 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 193 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 96 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 32 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 161 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 128 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 64 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 225 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 88 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 24 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 145 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 120 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 56 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 209 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 104 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 40 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 177 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 136 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 72 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 241 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 84 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 20 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 227 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 116 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 52 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 201 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 100 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 36 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 169 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 132 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 68 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 233 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 92 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 28 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 153 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 124 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 60 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 217 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 108 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 44 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 185 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 12 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 140 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 76 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 249 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 82 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 18 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 163 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 114 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 50 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 197 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 98 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 34 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 165 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 2 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 130 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 66 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 229 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 90 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 26 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 149 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 122 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 58 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 213 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 106 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 42 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 181 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 138 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 74 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 245 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 86 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 22 as ::core::ffi::c_ushort,
        },
        code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 118 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 54 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 205 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 102 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 38 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 173 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 134 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 70 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 237 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 94 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 30 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 157 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 126 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 62 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 221 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 110 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 46 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 189 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 14 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 142 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 78 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 253 as ::core::ffi::c_ushort,
        },
        code {
            op: 96 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 81 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 131 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 113 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 49 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 195 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 10 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 97 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 33 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 163 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 1 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 129 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 65 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 227 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 6 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 89 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 25 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 147 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 121 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 57 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 211 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 105 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 41 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 179 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 137 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 73 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 243 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 85 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 21 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 258 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 117 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 53 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 203 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 101 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 37 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 171 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 133 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 69 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 235 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 8 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 93 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 29 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 155 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 125 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 61 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 219 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 109 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 45 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 187 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 141 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 77 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 251 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 83 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 195 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 115 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 199 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 35 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 167 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 131 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 231 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 91 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 151 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 67 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 123 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 59 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 215 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 19 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 107 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 43 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 183 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 11 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 139 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 75 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 247 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 87 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 23 as ::core::ffi::c_ushort,
        },
        code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 51 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 119 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 55 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 207 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 103 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 39 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 175 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 135 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 71 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 239 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 95 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 31 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 159 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 99 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 127 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 63 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 223 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 7 as ::core::ffi::c_uchar,
            val: 27 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 111 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 47 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 191 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 15 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 143 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 8 as ::core::ffi::c_uchar,
            val: 79 as ::core::ffi::c_ushort,
        },
        code {
            op: 0 as ::core::ffi::c_uchar,
            bits: 9 as ::core::ffi::c_uchar,
            val: 255 as ::core::ffi::c_ushort,
        },
    ];
    static mut distfix: [code; 32] = [
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 1 as ::core::ffi::c_ushort,
        },
        code {
            op: 23 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 257 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 17 as ::core::ffi::c_ushort,
        },
        code {
            op: 27 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 4097 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 5 as ::core::ffi::c_ushort,
        },
        code {
            op: 25 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 1025 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 65 as ::core::ffi::c_ushort,
        },
        code {
            op: 29 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 16385 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 3 as ::core::ffi::c_ushort,
        },
        code {
            op: 24 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 513 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 33 as ::core::ffi::c_ushort,
        },
        code {
            op: 28 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 8193 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 9 as ::core::ffi::c_ushort,
        },
        code {
            op: 26 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 2049 as ::core::ffi::c_ushort,
        },
        code {
            op: 22 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 129 as ::core::ffi::c_ushort,
        },
        code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 2 as ::core::ffi::c_ushort,
        },
        code {
            op: 23 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 385 as ::core::ffi::c_ushort,
        },
        code {
            op: 19 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 25 as ::core::ffi::c_ushort,
        },
        code {
            op: 27 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 6145 as ::core::ffi::c_ushort,
        },
        code {
            op: 17 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 7 as ::core::ffi::c_ushort,
        },
        code {
            op: 25 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 1537 as ::core::ffi::c_ushort,
        },
        code {
            op: 21 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 97 as ::core::ffi::c_ushort,
        },
        code {
            op: 29 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 24577 as ::core::ffi::c_ushort,
        },
        code {
            op: 16 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 4 as ::core::ffi::c_ushort,
        },
        code {
            op: 24 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 769 as ::core::ffi::c_ushort,
        },
        code {
            op: 20 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 49 as ::core::ffi::c_ushort,
        },
        code {
            op: 28 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 12289 as ::core::ffi::c_ushort,
        },
        code {
            op: 18 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 13 as ::core::ffi::c_ushort,
        },
        code {
            op: 26 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 3073 as ::core::ffi::c_ushort,
        },
        code {
            op: 22 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 193 as ::core::ffi::c_ushort,
        },
        code {
            op: 64 as ::core::ffi::c_uchar,
            bits: 5 as ::core::ffi::c_uchar,
            val: 0 as ::core::ffi::c_ushort,
        },
    ];
    (*state).lencode = &raw const lenfix as *const code;
    (*state).lenbits = 9 as ::core::ffi::c_uint;
    (*state).distcode = &raw const distfix as *const code;
    (*state).distbits = 5 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn inflateBack(
    mut strm: z_streamp,
    mut in_0: in_func,
    mut in_desc: *mut ::core::ffi::c_void,
    mut out: out_func,
    mut out_desc: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut state: *mut inflate_state = ::core::ptr::null_mut::<inflate_state>();
    let mut next: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut put: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut have: ::core::ffi::c_uint = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut hold: ::core::ffi::c_ulong = 0;
    let mut bits: ::core::ffi::c_uint = 0;
    let mut copy: ::core::ffi::c_uint = 0;
    let mut from: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
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
    let mut len: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int = 0;
    static mut order: [::core::ffi::c_ushort; 19] = [
        16 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        17 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        18 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        0 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        8 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        7 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        9 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        6 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        10 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        5 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        11 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        4 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        12 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        3 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        13 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        2 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        14 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        1 as ::core::ffi::c_int as ::core::ffi::c_ushort,
        15 as ::core::ffi::c_int as ::core::ffi::c_ushort,
    ];
    if strm.is_null() || (*strm).state.is_null() {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    (*strm).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).mode = TYPE;
    (*state).last = 0 as ::core::ffi::c_int;
    (*state).whave = 0 as ::core::ffi::c_uint;
    next = (*strm).next_in as *mut ::core::ffi::c_uchar;
    have = if !next.is_null() {
        (*strm).avail_in as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    };
    hold = 0 as ::core::ffi::c_ulong;
    bits = 0 as ::core::ffi::c_uint;
    put = (*state).window;
    left = (*state).wsize;
    's_69: loop {
        match (*state).mode as ::core::ffi::c_uint {
            16191 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as ::core::ffi::c_uint;
                    bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                    (*state).mode = DONE;
                    continue;
                } else {
                    while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh0 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    (*state).last = (hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint))
                        as ::core::ffi::c_int;
                    hold >>= 1 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(1 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    match hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                            .wrapping_sub(1 as ::core::ffi::c_uint)
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
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = BAD;
                        }
                        _ => {}
                    }
                    hold >>= 2 as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(2 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    continue;
                }
            }
            16193 => {
                hold >>= bits & 7 as ::core::ffi::c_uint;
                bits = bits.wrapping_sub(bits & 7 as ::core::ffi::c_uint);
                while bits < 32 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh1 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if hold & 0xffff as ::core::ffi::c_ulong
                    != hold >> 16 as ::core::ffi::c_int ^ 0xffff as ::core::ffi::c_ulong
                {
                    (*strm).msg = b"invalid stored block lengths\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).length = hold as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint;
                    hold = 0 as ::core::ffi::c_ulong;
                    bits = 0 as ::core::ffi::c_uint;
                    while (*state).length != 0 as ::core::ffi::c_uint {
                        copy = (*state).length;
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        if left == 0 as ::core::ffi::c_uint {
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
                            put as *mut ::core::ffi::c_void,
                            next as *const ::core::ffi::c_void,
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
                while bits < 14 as ::core::ffi::c_int as ::core::ffi::c_uint {
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh2 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh2 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                (*state).nlen = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(257 as ::core::ffi::c_uint);
                hold >>= 5 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
                (*state).ndist = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 5 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(1 as ::core::ffi::c_uint);
                hold >>= 5 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(5 as ::core::ffi::c_int as ::core::ffi::c_uint);
                (*state).ncode = (hold as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int)
                        .wrapping_sub(1 as ::core::ffi::c_uint))
                .wrapping_add(4 as ::core::ffi::c_uint);
                hold >>= 4 as ::core::ffi::c_int;
                bits = bits.wrapping_sub(4 as ::core::ffi::c_int as ::core::ffi::c_uint);
                if (*state).nlen > 286 as ::core::ffi::c_uint
                    || (*state).ndist > 30 as ::core::ffi::c_uint
                {
                    (*strm).msg = b"too many length or distance symbols\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).have = 0 as ::core::ffi::c_uint;
                    while (*state).have < (*state).ncode {
                        while bits < 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
                            if have == 0 as ::core::ffi::c_uint {
                                have = in_0.expect("non-null function pointer")(
                                    in_desc,
                                    &raw mut next,
                                );
                                if have == 0 as ::core::ffi::c_uint {
                                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    ret = Z_BUF_ERROR;
                                    break 's_69;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let fresh3 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh3 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        let fresh4 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[order[fresh4 as usize] as usize] = (hold
                            as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as ::core::ffi::c_ushort;
                        hold >>= 3 as ::core::ffi::c_int;
                        bits = bits.wrapping_sub(3 as ::core::ffi::c_int as ::core::ffi::c_uint);
                    }
                    while (*state).have < 19 as ::core::ffi::c_uint {
                        let fresh5 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[order[fresh5 as usize] as usize] = 0 as ::core::ffi::c_ushort;
                    }
                    (*state).next = &raw mut (*state).codes as *mut code;
                    (*state).lencode = (*state).next as *const code;
                    (*state).lenbits = 7 as ::core::ffi::c_uint;
                    ret = inflate_table(
                        CODES,
                        &raw mut (*state).lens as *mut ::core::ffi::c_ushort,
                        19 as ::core::ffi::c_uint,
                        &raw mut (*state).next,
                        &raw mut (*state).lenbits,
                        &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                    );
                    if ret != 0 {
                        (*strm).msg = b"invalid code lengths set\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        (*state).have = 0 as ::core::ffi::c_uint;
                        while (*state).have < (*state).nlen.wrapping_add((*state).ndist) {
                            loop {
                                here = *(*state).lencode.offset(
                                    (hold as ::core::ffi::c_uint
                                        & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                                            .wrapping_sub(1 as ::core::ffi::c_uint))
                                        as isize,
                                );
                                if here.bits as ::core::ffi::c_uint <= bits {
                                    break;
                                }
                                if have == 0 as ::core::ffi::c_uint {
                                    have = in_0.expect("non-null function pointer")(
                                        in_desc,
                                        &raw mut next,
                                    );
                                    if have == 0 as ::core::ffi::c_uint {
                                        next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                        ret = Z_BUF_ERROR;
                                        break 's_69;
                                    }
                                }
                                have = have.wrapping_sub(1);
                                let fresh6 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh6 as ::core::ffi::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                            }
                            if (here.val as ::core::ffi::c_int) < 16 as ::core::ffi::c_int {
                                hold >>= here.bits as ::core::ffi::c_int;
                                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                let fresh7 = (*state).have;
                                (*state).have = (*state).have.wrapping_add(1);
                                (*state).lens[fresh7 as usize] = here.val;
                            } else {
                                if here.val as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 2 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        if have == 0 as ::core::ffi::c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as ::core::ffi::c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                ret = Z_BUF_ERROR;
                                                break 's_69;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let fresh8 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*fresh8 as ::core::ffi::c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    if (*state).have == 0 as ::core::ffi::c_uint {
                                        (*strm).msg = b"invalid bit length repeat\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char;
                                        (*state).mode = BAD;
                                        break;
                                    } else {
                                        len = (*state).lens[(*state)
                                            .have
                                            .wrapping_sub(1 as ::core::ffi::c_uint)
                                            as usize]
                                            as ::core::ffi::c_uint;
                                        copy = (3 as ::core::ffi::c_uint).wrapping_add(
                                            hold as ::core::ffi::c_uint
                                                & ((1 as ::core::ffi::c_uint)
                                                    << 2 as ::core::ffi::c_int)
                                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                                        );
                                        hold >>= 2 as ::core::ffi::c_int;
                                        bits = bits.wrapping_sub(
                                            2 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        );
                                    }
                                } else if here.val as ::core::ffi::c_int == 17 as ::core::ffi::c_int
                                {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 3 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        if have == 0 as ::core::ffi::c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as ::core::ffi::c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                ret = Z_BUF_ERROR;
                                                break 's_69;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let fresh9 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*fresh9 as ::core::ffi::c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    len = 0 as ::core::ffi::c_uint;
                                    copy = (3 as ::core::ffi::c_uint).wrapping_add(
                                        hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint)
                                                << 3 as ::core::ffi::c_int)
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                    );
                                    hold >>= 3 as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(
                                        3 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    );
                                } else {
                                    while bits
                                        < (here.bits as ::core::ffi::c_int
                                            + 7 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint
                                    {
                                        if have == 0 as ::core::ffi::c_uint {
                                            have = in_0.expect("non-null function pointer")(
                                                in_desc,
                                                &raw mut next,
                                            );
                                            if have == 0 as ::core::ffi::c_uint {
                                                next =
                                                    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                ret = Z_BUF_ERROR;
                                                break 's_69;
                                            }
                                        }
                                        have = have.wrapping_sub(1);
                                        let fresh10 = next;
                                        next = next.offset(1);
                                        hold = hold.wrapping_add(
                                            (*fresh10 as ::core::ffi::c_ulong) << bits,
                                        );
                                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                                    }
                                    hold >>= here.bits as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                                    len = 0 as ::core::ffi::c_uint;
                                    copy = (11 as ::core::ffi::c_uint).wrapping_add(
                                        hold as ::core::ffi::c_uint
                                            & ((1 as ::core::ffi::c_uint)
                                                << 7 as ::core::ffi::c_int)
                                                .wrapping_sub(1 as ::core::ffi::c_uint),
                                    );
                                    hold >>= 7 as ::core::ffi::c_int;
                                    bits = bits.wrapping_sub(
                                        7 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                    );
                                }
                                if (*state).have.wrapping_add(copy)
                                    > (*state).nlen.wrapping_add((*state).ndist)
                                {
                                    (*strm).msg = b"invalid bit length repeat\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
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
                                            len as ::core::ffi::c_ushort;
                                    }
                                }
                            }
                        }
                        if (*state).mode as ::core::ffi::c_uint
                            == BAD as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            continue;
                        }
                        if (*state).lens[256 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            (*strm).msg = b"invalid code -- missing end-of-block\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                            (*state).mode = BAD;
                            continue;
                        } else {
                            (*state).next = &raw mut (*state).codes as *mut code;
                            (*state).lencode = (*state).next as *const code;
                            (*state).lenbits = 9 as ::core::ffi::c_uint;
                            ret = inflate_table(
                                LENS,
                                &raw mut (*state).lens as *mut ::core::ffi::c_ushort,
                                (*state).nlen,
                                &raw mut (*state).next,
                                &raw mut (*state).lenbits,
                                &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                            );
                            if ret != 0 {
                                (*strm).msg = b"invalid literal/lengths set\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char;
                                (*state).mode = BAD;
                                continue;
                            } else {
                                (*state).distcode = (*state).next as *const code;
                                (*state).distbits = 6 as ::core::ffi::c_uint;
                                ret = inflate_table(
                                    DISTS,
                                    (&raw mut (*state).lens as *mut ::core::ffi::c_ushort)
                                        .offset((*state).nlen as isize),
                                    (*state).ndist,
                                    &raw mut (*state).next,
                                    &raw mut (*state).distbits,
                                    &raw mut (*state).work as *mut ::core::ffi::c_ushort,
                                );
                                if ret != 0 {
                                    (*strm).msg = b"invalid distances set\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
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
        if have >= 6 as ::core::ffi::c_uint && left >= 258 as ::core::ffi::c_uint {
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
            put = (*strm).next_out as *mut ::core::ffi::c_uchar;
            left = (*strm).avail_out as ::core::ffi::c_uint;
            next = (*strm).next_in as *mut ::core::ffi::c_uchar;
            have = (*strm).avail_in as ::core::ffi::c_uint;
            hold = (*state).hold;
            bits = (*state).bits;
        } else {
            loop {
                here = *(*state).lencode.offset(
                    (hold as ::core::ffi::c_uint
                        & ((1 as ::core::ffi::c_uint) << (*state).lenbits)
                            .wrapping_sub(1 as ::core::ffi::c_uint)) as isize,
                );
                if here.bits as ::core::ffi::c_uint <= bits {
                    break;
                }
                if have == 0 as ::core::ffi::c_uint {
                    have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                    if have == 0 as ::core::ffi::c_uint {
                        next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        ret = Z_BUF_ERROR;
                        break 's_69;
                    }
                }
                have = have.wrapping_sub(1);
                let fresh13 = next;
                next = next.offset(1);
                hold = hold.wrapping_add((*fresh13 as ::core::ffi::c_ulong) << bits);
                bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
            }
            if here.op as ::core::ffi::c_int != 0
                && here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                last = here;
                loop {
                    here = *(*state).lencode.offset(
                        (last.val as ::core::ffi::c_uint).wrapping_add(
                            (hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint)
                                    << last.bits as ::core::ffi::c_int
                                        + last.op as ::core::ffi::c_int)
                                    .wrapping_sub(1 as ::core::ffi::c_uint))
                                >> last.bits as ::core::ffi::c_int,
                        ) as isize,
                    );
                    if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        <= bits
                    {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh14 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh14 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                hold >>= last.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
            }
            hold >>= here.bits as ::core::ffi::c_int;
            bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
            (*state).length = here.val as ::core::ffi::c_uint;
            if here.op as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if left == 0 as ::core::ffi::c_uint {
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
                *fresh15 = (*state).length as ::core::ffi::c_uchar;
                left = left.wrapping_sub(1);
                (*state).mode = LEN;
            } else if here.op as ::core::ffi::c_int & 32 as ::core::ffi::c_int != 0 {
                (*state).mode = TYPE;
            } else if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                (*strm).msg = b"invalid literal/length code\0" as *const u8
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                (*state).mode = BAD;
            } else {
                (*state).extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                if (*state).extra != 0 as ::core::ffi::c_uint {
                    while bits < (*state).extra {
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh16 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh16 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    (*state).length = (*state).length.wrapping_add(
                        hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                .wrapping_sub(1 as ::core::ffi::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                }
                loop {
                    here = *(*state).distcode.offset(
                        (hold as ::core::ffi::c_uint
                            & ((1 as ::core::ffi::c_uint) << (*state).distbits)
                                .wrapping_sub(1 as ::core::ffi::c_uint))
                            as isize,
                    );
                    if here.bits as ::core::ffi::c_uint <= bits {
                        break;
                    }
                    if have == 0 as ::core::ffi::c_uint {
                        have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                        if have == 0 as ::core::ffi::c_uint {
                            next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                            ret = Z_BUF_ERROR;
                            break 's_69;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh17 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh17 as ::core::ffi::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                }
                if here.op as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    last = here;
                    loop {
                        here = *(*state).distcode.offset(
                            (last.val as ::core::ffi::c_uint).wrapping_add(
                                (hold as ::core::ffi::c_uint
                                    & ((1 as ::core::ffi::c_uint)
                                        << last.bits as ::core::ffi::c_int
                                            + last.op as ::core::ffi::c_int)
                                        .wrapping_sub(1 as ::core::ffi::c_uint))
                                    >> last.bits as ::core::ffi::c_int,
                            ) as isize,
                        );
                        if (last.bits as ::core::ffi::c_int + here.bits as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            <= bits
                        {
                            break;
                        }
                        if have == 0 as ::core::ffi::c_uint {
                            have = in_0.expect("non-null function pointer")(in_desc, &raw mut next);
                            if have == 0 as ::core::ffi::c_uint {
                                next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                ret = Z_BUF_ERROR;
                                break 's_69;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh18 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh18 as ::core::ffi::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                    }
                    hold >>= last.bits as ::core::ffi::c_int;
                    bits = bits.wrapping_sub(last.bits as ::core::ffi::c_uint);
                }
                hold >>= here.bits as ::core::ffi::c_int;
                bits = bits.wrapping_sub(here.bits as ::core::ffi::c_uint);
                if here.op as ::core::ffi::c_int & 64 as ::core::ffi::c_int != 0 {
                    (*strm).msg = b"invalid distance code\0" as *const u8
                        as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    (*state).mode = BAD;
                } else {
                    (*state).offset = here.val as ::core::ffi::c_uint;
                    (*state).extra = here.op as ::core::ffi::c_uint & 15 as ::core::ffi::c_uint;
                    if (*state).extra != 0 as ::core::ffi::c_uint {
                        while bits < (*state).extra {
                            if have == 0 as ::core::ffi::c_uint {
                                have = in_0.expect("non-null function pointer")(
                                    in_desc,
                                    &raw mut next,
                                );
                                if have == 0 as ::core::ffi::c_uint {
                                    next = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    ret = Z_BUF_ERROR;
                                    break 's_69;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let fresh19 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh19 as ::core::ffi::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as ::core::ffi::c_uint);
                        }
                        (*state).offset = (*state).offset.wrapping_add(
                            hold as ::core::ffi::c_uint
                                & ((1 as ::core::ffi::c_uint) << (*state).extra)
                                    .wrapping_sub(1 as ::core::ffi::c_uint),
                        );
                        hold >>= (*state).extra;
                        bits = bits.wrapping_sub((*state).extra);
                    }
                    if (*state).offset
                        > (*state).wsize.wrapping_sub(
                            (if (*state).whave < (*state).wsize {
                                left
                            } else {
                                0 as ::core::ffi::c_uint
                            }),
                        )
                    {
                        (*strm).msg = b"invalid distance too far back\0" as *const u8
                            as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char;
                        (*state).mode = BAD;
                    } else {
                        loop {
                            if left == 0 as ::core::ffi::c_uint {
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
                            if !((*state).length != 0 as ::core::ffi::c_uint) {
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
#[no_mangle]
pub unsafe extern "C" fn inflateBackEnd(mut strm: z_streamp) -> ::core::ffi::c_int {
    if strm.is_null() || (*strm).state.is_null() || (*strm).zfree.is_none() {
        return Z_STREAM_ERROR;
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<internal_state>();
    return Z_OK;
}
