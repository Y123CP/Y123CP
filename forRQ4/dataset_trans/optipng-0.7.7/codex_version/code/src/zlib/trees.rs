extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type charf = ::core::ffi::c_char;
pub type intf = ::core::ffi::c_int;
pub type voidpf = *mut ::core::ffi::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub strm: z_streamp,
    pub status: ::core::ffi::c_int,
    pub pending_buf: *mut Bytef,
    pub pending_buf_size: ulg,
    pub pending_out: *mut Bytef,
    pub pending: ulg,
    pub wrap: ::core::ffi::c_int,
    pub gzhead: gz_headerp,
    pub gzindex: ulg,
    pub method: Byte,
    pub last_flush: ::core::ffi::c_int,
    pub w_size: uInt,
    pub w_bits: uInt,
    pub w_mask: uInt,
    pub window: *mut Bytef,
    pub window_size: ulg,
    pub prev: *mut Posf,
    pub head: *mut Posf,
    pub ins_h: uInt,
    pub hash_size: uInt,
    pub hash_bits: uInt,
    pub hash_mask: uInt,
    pub hash_shift: uInt,
    pub block_start: ::core::ffi::c_long,
    pub match_length: uInt,
    pub prev_match: IPos,
    pub match_available: ::core::ffi::c_int,
    pub strstart: uInt,
    pub match_start: uInt,
    pub lookahead: uInt,
    pub prev_length: uInt,
    pub max_chain_length: uInt,
    pub max_lazy_match: uInt,
    pub level: ::core::ffi::c_int,
    pub strategy: ::core::ffi::c_int,
    pub good_match: uInt,
    pub nice_match: ::core::ffi::c_int,
    pub dyn_ltree: [ct_data_s; 573],
    pub dyn_dtree: [ct_data_s; 61],
    pub bl_tree: [ct_data_s; 39],
    pub l_desc: tree_desc_s,
    pub d_desc: tree_desc_s,
    pub bl_desc: tree_desc_s,
    pub bl_count: [ush; 16],
    pub heap: [::core::ffi::c_int; 573],
    pub heap_len: ::core::ffi::c_int,
    pub heap_max: ::core::ffi::c_int,
    pub depth: [uch; 573],
    pub l_buf: *mut uchf,
    pub lit_bufsize: uInt,
    pub last_lit: uInt,
    pub d_buf: *mut ushf,
    pub opt_len: ulg,
    pub static_len: ulg,
    pub matches: uInt,
    pub insert: uInt,
    pub bi_buf: ush,
    pub bi_valid: ::core::ffi::c_int,
    pub high_water: ulg,
}
pub type ulg = ::core::ffi::c_ulong;
pub type ush = ::core::ffi::c_ushort;
pub type ushf = ush;
pub type uchf = uch;
pub type uch = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc_s {
    pub dyn_tree: *mut ct_data,
    pub max_code: ::core::ffi::c_int,
    pub stat_desc: *const static_tree_desc,
}
pub type static_tree_desc = static_tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct static_tree_desc_s {
    pub static_tree: *const ct_data,
    pub extra_bits: *const intf,
    pub extra_base: ::core::ffi::c_int,
    pub elems: ::core::ffi::c_int,
    pub max_length: ::core::ffi::c_int,
}
pub type ct_data = ct_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data_s {
    pub fc: C2RustUnnamed_0,
    pub dl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dad: ush,
    pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub freq: ush,
    pub code: ush,
}
pub type IPos = ::core::ffi::c_uint;
pub type Posf = Pos;
pub type Pos = ush;
pub type gz_headerp = *mut gz_header;
pub type gz_header = gz_header_s;
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
pub type z_streamp = *mut z_stream;
pub type z_stream = z_stream_s;
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
pub type tree_desc = tree_desc_s;
pub type deflate_state = internal_state;
pub const Z_FIXED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Z_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_TEXT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Z_UNKNOWN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LENGTH_CODES: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const LITERALS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const L_CODES: ::core::ffi::c_int = LITERALS + 1 as ::core::ffi::c_int + LENGTH_CODES;
pub const D_CODES: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const BL_CODES: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const HEAP_SIZE: ::core::ffi::c_int =
    2 as ::core::ffi::c_int * L_CODES + 1 as ::core::ffi::c_int;
pub const MAX_BITS: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const Buf_size: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAX_BL_BITS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const END_BLOCK: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const REP_3_6: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const REPZ_3_10: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const REPZ_11_138: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
static mut extra_lbits: [::core::ffi::c_int; 29] = [
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
];
static mut extra_dbits: [::core::ffi::c_int; 30] = [
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    9 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    11 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
];
static mut extra_blbits: [::core::ffi::c_int; 19] = [
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
];
static mut bl_order: [uch; 19] = [
    16 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    0 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    7 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    6 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    5 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    4 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    3 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    2 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    1 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
];
static mut static_ltree: [ct_data; 288] = [
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 12 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 140 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 76 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 204 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 44 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 172 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 108 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 236 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 28 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 156 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 92 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 220 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 60 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 188 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 124 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 252 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 2 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 130 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 66 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 194 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 34 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 162 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 98 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 226 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 18 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 146 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 82 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 210 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 50 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 178 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 114 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 242 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 10 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 138 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 74 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 202 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 42 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 170 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 106 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 234 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 26 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 154 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 90 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 218 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 58 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 186 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 122 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 250 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 6 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 134 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 70 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 198 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 38 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 166 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 102 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 230 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 22 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 150 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 86 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 214 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 54 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 182 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 118 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 246 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 14 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 142 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 78 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 206 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 46 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 174 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 110 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 238 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 30 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 158 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 94 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 222 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 62 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 190 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 126 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 254 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 1 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 129 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 65 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 193 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 33 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 161 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 97 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 225 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 17 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 145 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 81 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 209 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 49 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 177 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 113 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 241 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 9 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 137 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 73 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 201 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 41 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 169 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 105 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 233 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 25 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 153 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 89 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 217 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 57 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 185 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 121 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 249 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 5 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 133 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 69 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 197 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 37 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 165 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 101 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 229 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 21 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 149 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 85 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 213 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 53 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 181 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 117 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 245 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 13 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 141 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 77 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 205 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 45 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 173 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 109 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 237 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 29 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 157 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 93 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 221 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 61 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 189 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 125 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 253 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 19 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 275 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 147 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 403 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 83 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 339 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 211 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 467 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 51 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 307 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 179 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 435 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 115 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 371 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 243 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 499 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 11 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 267 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 139 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 395 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 75 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 331 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 203 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 459 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 43 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 299 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 171 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 427 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 107 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 363 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 235 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 491 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 27 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 283 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 155 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 411 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 91 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 347 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 219 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 475 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 59 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 315 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 187 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 443 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 123 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 379 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 251 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 507 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 7 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 263 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 135 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 391 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 71 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 327 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 199 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 455 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 39 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 295 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 167 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 423 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 103 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 359 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 231 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 487 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 23 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 279 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 151 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 407 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 87 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 343 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 215 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 471 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 55 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 311 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 183 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 439 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 119 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 375 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 247 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 503 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 15 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 271 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 143 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 399 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 79 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 335 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 207 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 463 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 47 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 303 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 175 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 431 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 111 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 367 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 239 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 495 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 31 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 287 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 159 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 415 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 95 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 351 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 223 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 479 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 63 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 319 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 191 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 447 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 127 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 383 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 255 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 511 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 9 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 0 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 64 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 32 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 96 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 16 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 80 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 48 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 112 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 8 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 72 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 40 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 104 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 24 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 88 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 56 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 120 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 4 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 68 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 36 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 100 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 20 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 84 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 52 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 116 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 7 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 3 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 131 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 67 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 195 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 35 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 163 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 99 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 227 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 8 as ::core::ffi::c_int as ush,
        },
    },
];
static mut static_dtree: [ct_data; 30] = [
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 0 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 16 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 8 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 24 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 4 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 20 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 12 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 28 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 2 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 18 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 10 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 26 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 6 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 22 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 14 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 30 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 1 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 17 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 9 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 25 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 5 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 21 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 13 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 29 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 3 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 19 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 11 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 27 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 7 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_0 {
            freq: 23 as ::core::ffi::c_int as ush,
        },
        dl: C2RustUnnamed {
            dad: 5 as ::core::ffi::c_int as ush,
        },
    },
];
#[no_mangle]
pub static mut _dist_code: [uch; 512] = [
    0 as ::core::ffi::c_int as uch,
    1 as ::core::ffi::c_int as uch,
    2 as ::core::ffi::c_int as uch,
    3 as ::core::ffi::c_int as uch,
    4 as ::core::ffi::c_int as uch,
    4 as ::core::ffi::c_int as uch,
    5 as ::core::ffi::c_int as uch,
    5 as ::core::ffi::c_int as uch,
    6 as ::core::ffi::c_int as uch,
    6 as ::core::ffi::c_int as uch,
    6 as ::core::ffi::c_int as uch,
    6 as ::core::ffi::c_int as uch,
    7 as ::core::ffi::c_int as uch,
    7 as ::core::ffi::c_int as uch,
    7 as ::core::ffi::c_int as uch,
    7 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    0 as ::core::ffi::c_int as uch,
    0 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
    29 as ::core::ffi::c_int as uch,
];
#[no_mangle]
pub static mut _length_code: [uch; 256] = [
    0 as ::core::ffi::c_int as uch,
    1 as ::core::ffi::c_int as uch,
    2 as ::core::ffi::c_int as uch,
    3 as ::core::ffi::c_int as uch,
    4 as ::core::ffi::c_int as uch,
    5 as ::core::ffi::c_int as uch,
    6 as ::core::ffi::c_int as uch,
    7 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    8 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    9 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    10 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    11 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    12 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    13 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    14 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    15 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    16 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    17 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    18 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    19 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    20 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    21 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    22 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    23 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    24 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    25 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    26 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    27 as ::core::ffi::c_int as uch,
    28 as ::core::ffi::c_int as uch,
];
static mut base_length: [::core::ffi::c_int; 29] = [
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    5 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    7 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    10 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    14 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    20 as ::core::ffi::c_int,
    24 as ::core::ffi::c_int,
    28 as ::core::ffi::c_int,
    32 as ::core::ffi::c_int,
    40 as ::core::ffi::c_int,
    48 as ::core::ffi::c_int,
    56 as ::core::ffi::c_int,
    64 as ::core::ffi::c_int,
    80 as ::core::ffi::c_int,
    96 as ::core::ffi::c_int,
    112 as ::core::ffi::c_int,
    128 as ::core::ffi::c_int,
    160 as ::core::ffi::c_int,
    192 as ::core::ffi::c_int,
    224 as ::core::ffi::c_int,
    0 as ::core::ffi::c_int,
];
static mut base_dist: [::core::ffi::c_int; 30] = [
    0 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    2 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    6 as ::core::ffi::c_int,
    8 as ::core::ffi::c_int,
    12 as ::core::ffi::c_int,
    16 as ::core::ffi::c_int,
    24 as ::core::ffi::c_int,
    32 as ::core::ffi::c_int,
    48 as ::core::ffi::c_int,
    64 as ::core::ffi::c_int,
    96 as ::core::ffi::c_int,
    128 as ::core::ffi::c_int,
    192 as ::core::ffi::c_int,
    256 as ::core::ffi::c_int,
    384 as ::core::ffi::c_int,
    512 as ::core::ffi::c_int,
    768 as ::core::ffi::c_int,
    1024 as ::core::ffi::c_int,
    1536 as ::core::ffi::c_int,
    2048 as ::core::ffi::c_int,
    3072 as ::core::ffi::c_int,
    4096 as ::core::ffi::c_int,
    6144 as ::core::ffi::c_int,
    8192 as ::core::ffi::c_int,
    12288 as ::core::ffi::c_int,
    16384 as ::core::ffi::c_int,
    24576 as ::core::ffi::c_int,
];
static mut static_l_desc: static_tree_desc = unsafe {
    static_tree_desc_s {
        static_tree: &raw const static_ltree as *const ct_data,
        extra_bits: &raw const extra_lbits as *const intf,
        extra_base: LITERALS + 1 as ::core::ffi::c_int,
        elems: L_CODES,
        max_length: MAX_BITS,
    }
};
static mut static_d_desc: static_tree_desc = unsafe {
    static_tree_desc_s {
        static_tree: &raw const static_dtree as *const ct_data,
        extra_bits: &raw const extra_dbits as *const intf,
        extra_base: 0 as ::core::ffi::c_int,
        elems: D_CODES,
        max_length: MAX_BITS,
    }
};
static mut static_bl_desc: static_tree_desc = unsafe {
    static_tree_desc_s {
        static_tree: ::core::ptr::null::<ct_data>(),
        extra_bits: &raw const extra_blbits as *const intf,
        extra_base: 0 as ::core::ffi::c_int,
        elems: BL_CODES,
        max_length: MAX_BL_BITS,
    }
};
unsafe extern "C" fn tr_static_init() {}
#[no_mangle]
pub unsafe extern "C" fn _tr_init(mut s: *mut deflate_state) {
    tr_static_init();
    (*s).l_desc.dyn_tree = &raw mut (*s).dyn_ltree as *mut ct_data_s as *mut ct_data;
    (*s).l_desc.stat_desc = &raw const static_l_desc;
    (*s).d_desc.dyn_tree = &raw mut (*s).dyn_dtree as *mut ct_data_s as *mut ct_data;
    (*s).d_desc.stat_desc = &raw const static_d_desc;
    (*s).bl_desc.dyn_tree = &raw mut (*s).bl_tree as *mut ct_data_s as *mut ct_data;
    (*s).bl_desc.stat_desc = &raw const static_bl_desc;
    (*s).bi_buf = 0 as ush;
    (*s).bi_valid = 0 as ::core::ffi::c_int;
    init_block(s);
}
unsafe extern "C" fn init_block(mut s: *mut deflate_state) {
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while n < L_CODES {
        (*s).dyn_ltree[n as usize].fc.freq = 0 as ush;
        n += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n < D_CODES {
        (*s).dyn_dtree[n as usize].fc.freq = 0 as ush;
        n += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n < BL_CODES {
        (*s).bl_tree[n as usize].fc.freq = 0 as ush;
        n += 1;
    }
    (*s).dyn_ltree[END_BLOCK as usize].fc.freq = 1 as ush;
    (*s).static_len = 0 as ulg;
    (*s).opt_len = (*s).static_len;
    (*s).matches = 0 as uInt;
    (*s).last_lit = (*s).matches;
}
pub const SMALLEST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn pqdownheap(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut k: ::core::ffi::c_int,
) {
    let mut v: ::core::ffi::c_int = (*s).heap[k as usize];
    let mut j: ::core::ffi::c_int = k << 1 as ::core::ffi::c_int;
    while j <= (*s).heap_len {
        if j < (*s).heap_len
            && (((*tree.offset((*s).heap[(j + 1 as ::core::ffi::c_int) as usize] as isize))
                .fc
                .freq as ::core::ffi::c_int)
                < (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as ::core::ffi::c_int
                || (*tree.offset((*s).heap[(j + 1 as ::core::ffi::c_int) as usize] as isize))
                    .fc
                    .freq as ::core::ffi::c_int
                    == (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as ::core::ffi::c_int
                    && (*s).depth[(*s).heap[(j + 1 as ::core::ffi::c_int) as usize] as usize]
                        as ::core::ffi::c_int
                        <= (*s).depth[(*s).heap[j as usize] as usize] as ::core::ffi::c_int)
        {
            j += 1;
        }
        if ((*tree.offset(v as isize)).fc.freq as ::core::ffi::c_int)
            < (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as ::core::ffi::c_int
            || (*tree.offset(v as isize)).fc.freq as ::core::ffi::c_int
                == (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as ::core::ffi::c_int
                && (*s).depth[v as usize] as ::core::ffi::c_int
                    <= (*s).depth[(*s).heap[j as usize] as usize] as ::core::ffi::c_int
        {
            break;
        }
        (*s).heap[k as usize] = (*s).heap[j as usize];
        k = j;
        j <<= 1 as ::core::ffi::c_int;
    }
    (*s).heap[k as usize] = v;
}
unsafe extern "C" fn gen_bitlen(mut s: *mut deflate_state, mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut max_code: ::core::ffi::c_int = (*desc).max_code;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut extra: *const intf = (*(*desc).stat_desc).extra_bits;
    let mut base: ::core::ffi::c_int = (*(*desc).stat_desc).extra_base;
    let mut max_length: ::core::ffi::c_int = (*(*desc).stat_desc).max_length;
    let mut h: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut bits: ::core::ffi::c_int = 0;
    let mut xbits: ::core::ffi::c_int = 0;
    let mut f: ush = 0;
    let mut overflow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    bits = 0 as ::core::ffi::c_int;
    while bits <= MAX_BITS {
        (*s).bl_count[bits as usize] = 0 as ush;
        bits += 1;
    }
    (*tree.offset((*s).heap[(*s).heap_max as usize] as isize))
        .dl
        .len = 0 as ush;
    h = (*s).heap_max + 1 as ::core::ffi::c_int;
    while h < HEAP_SIZE {
        n = (*s).heap[h as usize];
        bits = (*tree.offset((*tree.offset(n as isize)).dl.dad as isize))
            .dl
            .len as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int;
        if bits > max_length {
            bits = max_length;
            overflow += 1;
        }
        (*tree.offset(n as isize)).dl.len = bits as ush;
        if !(n > max_code) {
            (*s).bl_count[bits as usize] = (*s).bl_count[bits as usize].wrapping_add(1);
            xbits = 0 as ::core::ffi::c_int;
            if n >= base {
                xbits = *extra.offset((n - base) as isize) as ::core::ffi::c_int;
            }
            f = (*tree.offset(n as isize)).fc.freq;
            (*s).opt_len = ((*s).opt_len as ::core::ffi::c_ulong).wrapping_add(
                (f as ::core::ffi::c_ulong)
                    .wrapping_mul((bits + xbits) as ::core::ffi::c_uint as ::core::ffi::c_ulong),
            ) as ulg as ulg;
            if !stree.is_null() {
                (*s).static_len = ((*s).static_len as ::core::ffi::c_ulong).wrapping_add(
                    (f as ::core::ffi::c_ulong).wrapping_mul(
                        ((*stree.offset(n as isize)).dl.len as ::core::ffi::c_int + xbits)
                            as ::core::ffi::c_uint as ::core::ffi::c_ulong,
                    ),
                ) as ulg as ulg;
            }
        }
        h += 1;
    }
    if overflow == 0 as ::core::ffi::c_int {
        return;
    }
    loop {
        bits = max_length - 1 as ::core::ffi::c_int;
        while (*s).bl_count[bits as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            bits -= 1;
        }
        (*s).bl_count[bits as usize] = (*s).bl_count[bits as usize].wrapping_sub(1);
        (*s).bl_count[(bits + 1 as ::core::ffi::c_int) as usize] =
            ((*s).bl_count[(bits + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as ush;
        (*s).bl_count[max_length as usize] = (*s).bl_count[max_length as usize].wrapping_sub(1);
        overflow -= 2 as ::core::ffi::c_int;
        if !(overflow > 0 as ::core::ffi::c_int) {
            break;
        }
    }
    bits = max_length;
    while bits != 0 as ::core::ffi::c_int {
        n = (*s).bl_count[bits as usize] as ::core::ffi::c_int;
        while n != 0 as ::core::ffi::c_int {
            h -= 1;
            m = (*s).heap[h as usize];
            if m > max_code {
                continue;
            }
            if (*tree.offset(m as isize)).dl.len as ::core::ffi::c_uint
                != bits as ::core::ffi::c_uint
            {
                (*s).opt_len = ((*s).opt_len as ::core::ffi::c_ulong).wrapping_add(
                    (bits as ::core::ffi::c_ulong)
                        .wrapping_sub((*tree.offset(m as isize)).dl.len as ::core::ffi::c_ulong)
                        .wrapping_mul((*tree.offset(m as isize)).fc.freq as ::core::ffi::c_ulong),
                ) as ulg as ulg;
                (*tree.offset(m as isize)).dl.len = bits as ush;
            }
            n -= 1;
        }
        bits -= 1;
    }
}
unsafe extern "C" fn gen_codes(
    mut tree: *mut ct_data,
    mut max_code: ::core::ffi::c_int,
    mut bl_count: *mut ushf,
) {
    let mut next_code: [ush; 16] = [0; 16];
    let mut code: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut bits: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    bits = 1 as ::core::ffi::c_int;
    while bits <= MAX_BITS {
        code = code.wrapping_add(
            *bl_count.offset((bits - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uint
        ) << 1 as ::core::ffi::c_int;
        next_code[bits as usize] = code as ush;
        bits += 1;
    }
    n = 0 as ::core::ffi::c_int;
    while n <= max_code {
        let mut len: ::core::ffi::c_int = (*tree.offset(n as isize)).dl.len as ::core::ffi::c_int;
        if !(len == 0 as ::core::ffi::c_int) {
            let fresh54 = next_code[len as usize];
            next_code[len as usize] = next_code[len as usize].wrapping_add(1);
            (*tree.offset(n as isize)).fc.code =
                bi_reverse(fresh54 as ::core::ffi::c_uint, len) as ush;
        }
        n += 1;
    }
}
unsafe extern "C" fn build_tree(mut s: *mut deflate_state, mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut elems: ::core::ffi::c_int = (*(*desc).stat_desc).elems;
    let mut n: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut max_code: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut node: ::core::ffi::c_int = 0;
    (*s).heap_len = 0 as ::core::ffi::c_int;
    (*s).heap_max = HEAP_SIZE;
    n = 0 as ::core::ffi::c_int;
    while n < elems {
        if (*tree.offset(n as isize)).fc.freq as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            max_code = n;
            (*s).heap_len += 1;
            (*s).heap[(*s).heap_len as usize] = max_code;
            (*s).depth[n as usize] = 0 as uch;
        } else {
            (*tree.offset(n as isize)).dl.len = 0 as ush;
        }
        n += 1;
    }
    while (*s).heap_len < 2 as ::core::ffi::c_int {
        (*s).heap_len += 1;
        (*s).heap[(*s).heap_len as usize] = if max_code < 2 as ::core::ffi::c_int {
            max_code += 1;
            max_code
        } else {
            0 as ::core::ffi::c_int
        };
        node = (*s).heap[(*s).heap_len as usize];
        (*tree.offset(node as isize)).fc.freq = 1 as ush;
        (*s).depth[node as usize] = 0 as uch;
        (*s).opt_len = (*s).opt_len.wrapping_sub(1);
        if !stree.is_null() {
            (*s).static_len = ((*s).static_len as ::core::ffi::c_ulong)
                .wrapping_sub((*stree.offset(node as isize)).dl.len as ::core::ffi::c_ulong)
                as ulg as ulg;
        }
    }
    (*desc).max_code = max_code;
    n = (*s).heap_len / 2 as ::core::ffi::c_int;
    while n >= 1 as ::core::ffi::c_int {
        pqdownheap(s, tree, n);
        n -= 1;
    }
    node = elems;
    loop {
        n = (*s).heap[SMALLEST as usize];
        let fresh51 = (*s).heap_len;
        (*s).heap_len = (*s).heap_len - 1;
        (*s).heap[SMALLEST as usize] = (*s).heap[fresh51 as usize];
        pqdownheap(s, tree, SMALLEST);
        m = (*s).heap[SMALLEST as usize];
        (*s).heap_max -= 1;
        (*s).heap[(*s).heap_max as usize] = n;
        (*s).heap_max -= 1;
        (*s).heap[(*s).heap_max as usize] = m;
        (*tree.offset(node as isize)).fc.freq =
            ((*tree.offset(n as isize)).fc.freq as ::core::ffi::c_int
                + (*tree.offset(m as isize)).fc.freq as ::core::ffi::c_int) as ush;
        (*s).depth[node as usize] = ((if (*s).depth[n as usize] as ::core::ffi::c_int
            >= (*s).depth[m as usize] as ::core::ffi::c_int
        {
            (*s).depth[n as usize] as ::core::ffi::c_int
        } else {
            (*s).depth[m as usize] as ::core::ffi::c_int
        }) + 1 as ::core::ffi::c_int) as uch;
        let ref mut fresh52 = (*tree.offset(m as isize)).dl.dad;
        *fresh52 = node as ush;
        (*tree.offset(n as isize)).dl.dad = *fresh52;
        let fresh53 = node;
        node = node + 1;
        (*s).heap[SMALLEST as usize] = fresh53;
        pqdownheap(s, tree, SMALLEST);
        if !((*s).heap_len >= 2 as ::core::ffi::c_int) {
            break;
        }
    }
    (*s).heap_max -= 1;
    (*s).heap[(*s).heap_max as usize] = (*s).heap[SMALLEST as usize];
    gen_bitlen(s, desc);
    gen_codes(tree, max_code, &raw mut (*s).bl_count as *mut ushf);
}
unsafe extern "C" fn scan_tree(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut max_code: ::core::ffi::c_int,
) {
    let mut n: ::core::ffi::c_int = 0;
    let mut prevlen: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut curlen: ::core::ffi::c_int = 0;
    let mut nextlen: ::core::ffi::c_int =
        (*tree.offset(0 as ::core::ffi::c_int as isize)).dl.len as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_count: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    let mut min_count: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    if nextlen == 0 as ::core::ffi::c_int {
        max_count = 138 as ::core::ffi::c_int;
        min_count = 3 as ::core::ffi::c_int;
    }
    (*tree.offset((max_code + 1 as ::core::ffi::c_int) as isize))
        .dl
        .len = 0xffff as ::core::ffi::c_int as ush;
    n = 0 as ::core::ffi::c_int;
    while n <= max_code {
        curlen = nextlen;
        nextlen = (*tree.offset((n + 1 as ::core::ffi::c_int) as isize))
            .dl
            .len as ::core::ffi::c_int;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                (*s).bl_tree[curlen as usize].fc.freq =
                    ((*s).bl_tree[curlen as usize].fc.freq as ::core::ffi::c_int + count) as ush;
            } else if curlen != 0 as ::core::ffi::c_int {
                if curlen != prevlen {
                    (*s).bl_tree[curlen as usize].fc.freq =
                        (*s).bl_tree[curlen as usize].fc.freq.wrapping_add(1);
                }
                (*s).bl_tree[REP_3_6 as usize].fc.freq =
                    (*s).bl_tree[REP_3_6 as usize].fc.freq.wrapping_add(1);
            } else if count <= 10 as ::core::ffi::c_int {
                (*s).bl_tree[REPZ_3_10 as usize].fc.freq =
                    (*s).bl_tree[REPZ_3_10 as usize].fc.freq.wrapping_add(1);
            } else {
                (*s).bl_tree[REPZ_11_138 as usize].fc.freq =
                    (*s).bl_tree[REPZ_11_138 as usize].fc.freq.wrapping_add(1);
            }
            count = 0 as ::core::ffi::c_int;
            prevlen = curlen;
            if nextlen == 0 as ::core::ffi::c_int {
                max_count = 138 as ::core::ffi::c_int;
                min_count = 3 as ::core::ffi::c_int;
            } else if curlen == nextlen {
                max_count = 6 as ::core::ffi::c_int;
                min_count = 3 as ::core::ffi::c_int;
            } else {
                max_count = 7 as ::core::ffi::c_int;
                min_count = 4 as ::core::ffi::c_int;
            }
        }
        n += 1;
    }
}
unsafe extern "C" fn send_tree(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut max_code: ::core::ffi::c_int,
) {
    let mut n: ::core::ffi::c_int = 0;
    let mut prevlen: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut curlen: ::core::ffi::c_int = 0;
    let mut nextlen: ::core::ffi::c_int =
        (*tree.offset(0 as ::core::ffi::c_int as isize)).dl.len as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_count: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    let mut min_count: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    if nextlen == 0 as ::core::ffi::c_int {
        max_count = 138 as ::core::ffi::c_int;
        min_count = 3 as ::core::ffi::c_int;
    }
    n = 0 as ::core::ffi::c_int;
    while n <= max_code {
        curlen = nextlen;
        nextlen = (*tree.offset((n + 1 as ::core::ffi::c_int) as isize))
            .dl
            .len as ::core::ffi::c_int;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                loop {
                    let mut len: ::core::ffi::c_int =
                        (*s).bl_tree[curlen as usize].dl.len as ::core::ffi::c_int;
                    if (*s).bi_valid > Buf_size - len {
                        let mut val: ::core::ffi::c_int =
                            (*s).bl_tree[curlen as usize].fc.code as ::core::ffi::c_int;
                        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                            | (val as ush as ::core::ffi::c_int) << (*s).bi_valid)
                            as ush;
                        let fresh29 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh29 as isize) =
                            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                        let fresh30 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh30 as isize) =
                            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                        (*s).bi_buf =
                            (val as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                        (*s).bi_valid += len - Buf_size;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                            | ((*s).bl_tree[curlen as usize].fc.code as ::core::ffi::c_int)
                                << (*s).bi_valid) as ush;
                        (*s).bi_valid += len;
                    }
                    count -= 1;
                    if !(count != 0 as ::core::ffi::c_int) {
                        break;
                    }
                }
            } else if curlen != 0 as ::core::ffi::c_int {
                if curlen != prevlen {
                    let mut len_0: ::core::ffi::c_int =
                        (*s).bl_tree[curlen as usize].dl.len as ::core::ffi::c_int;
                    if (*s).bi_valid > Buf_size - len_0 {
                        let mut val_0: ::core::ffi::c_int =
                            (*s).bl_tree[curlen as usize].fc.code as ::core::ffi::c_int;
                        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                            | (val_0 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                            as ush;
                        let fresh31 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh31 as isize) =
                            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                        let fresh32 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh32 as isize) =
                            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                        (*s).bi_buf =
                            (val_0 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                        (*s).bi_valid += len_0 - Buf_size;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                            | ((*s).bl_tree[curlen as usize].fc.code as ::core::ffi::c_int)
                                << (*s).bi_valid) as ush;
                        (*s).bi_valid += len_0;
                    }
                    count -= 1;
                }
                let mut len_1: ::core::ffi::c_int =
                    (*s).bl_tree[16 as ::core::ffi::c_int as usize].dl.len as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len_1 {
                    let mut val_1: ::core::ffi::c_int =
                        (*s).bl_tree[16 as ::core::ffi::c_int as usize].fc.code
                            as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val_1 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh33 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh33 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh34 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh34 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val_1 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_1 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((*s).bl_tree[16 as ::core::ffi::c_int as usize].fc.code
                            as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_1;
                }
                let mut len_2: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len_2 {
                    let mut val_2: ::core::ffi::c_int = count - 3 as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val_2 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh35 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh35 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh36 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh36 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val_2 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((count - 3 as ::core::ffi::c_int) as ush as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2;
                }
            } else if count <= 10 as ::core::ffi::c_int {
                let mut len_3: ::core::ffi::c_int =
                    (*s).bl_tree[17 as ::core::ffi::c_int as usize].dl.len as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len_3 {
                    let mut val_3: ::core::ffi::c_int =
                        (*s).bl_tree[17 as ::core::ffi::c_int as usize].fc.code
                            as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val_3 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh37 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh37 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh38 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh38 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val_3 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_3 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((*s).bl_tree[17 as ::core::ffi::c_int as usize].fc.code
                            as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_3;
                }
                let mut len_4: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len_4 {
                    let mut val_4: ::core::ffi::c_int = count - 3 as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val_4 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh39 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh39 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh40 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh40 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val_4 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_4 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((count - 3 as ::core::ffi::c_int) as ush as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_4;
                }
            } else {
                let mut len_5: ::core::ffi::c_int =
                    (*s).bl_tree[18 as ::core::ffi::c_int as usize].dl.len as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len_5 {
                    let mut val_5: ::core::ffi::c_int =
                        (*s).bl_tree[18 as ::core::ffi::c_int as usize].fc.code
                            as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val_5 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh41 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh41 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh42 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh42 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val_5 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_5 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((*s).bl_tree[18 as ::core::ffi::c_int as usize].fc.code
                            as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_5;
                }
                let mut len_6: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len_6 {
                    let mut val_6: ::core::ffi::c_int = count - 11 as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val_6 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh43 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh43 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh44 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh44 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val_6 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_6 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((count - 11 as ::core::ffi::c_int) as ush as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_6;
                }
            }
            count = 0 as ::core::ffi::c_int;
            prevlen = curlen;
            if nextlen == 0 as ::core::ffi::c_int {
                max_count = 138 as ::core::ffi::c_int;
                min_count = 3 as ::core::ffi::c_int;
            } else if curlen == nextlen {
                max_count = 6 as ::core::ffi::c_int;
                min_count = 3 as ::core::ffi::c_int;
            } else {
                max_count = 7 as ::core::ffi::c_int;
                min_count = 4 as ::core::ffi::c_int;
            }
        }
        n += 1;
    }
}
unsafe extern "C" fn build_bl_tree(mut s: *mut deflate_state) -> ::core::ffi::c_int {
    let mut max_blindex: ::core::ffi::c_int = 0;
    scan_tree(
        s,
        &raw mut (*s).dyn_ltree as *mut ct_data_s as *mut ct_data,
        (*s).l_desc.max_code,
    );
    scan_tree(
        s,
        &raw mut (*s).dyn_dtree as *mut ct_data_s as *mut ct_data,
        (*s).d_desc.max_code,
    );
    build_tree(s, &raw mut (*s).bl_desc as *mut tree_desc);
    max_blindex = BL_CODES - 1 as ::core::ffi::c_int;
    while max_blindex >= 3 as ::core::ffi::c_int {
        if (*s).bl_tree[bl_order[max_blindex as usize] as usize].dl.len as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            break;
        }
        max_blindex -= 1;
    }
    (*s).opt_len = ((*s).opt_len as ::core::ffi::c_ulong).wrapping_add(
        (3 as ::core::ffi::c_ulong)
            .wrapping_mul(
                (max_blindex as ::core::ffi::c_ulong).wrapping_add(1 as ::core::ffi::c_ulong),
            )
            .wrapping_add(5 as ::core::ffi::c_ulong)
            .wrapping_add(5 as ::core::ffi::c_ulong)
            .wrapping_add(4 as ::core::ffi::c_ulong),
    ) as ulg as ulg;
    return max_blindex;
}
unsafe extern "C" fn send_all_trees(
    mut s: *mut deflate_state,
    mut lcodes: ::core::ffi::c_int,
    mut dcodes: ::core::ffi::c_int,
    mut blcodes: ::core::ffi::c_int,
) {
    let mut rank: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if (*s).bi_valid > Buf_size - len {
        let mut val: ::core::ffi::c_int = lcodes - 257 as ::core::ffi::c_int;
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (val as ush as ::core::ffi::c_int) << (*s).bi_valid) as ush;
        let fresh21 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh21 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh22 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh22 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
        (*s).bi_buf = (val as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | ((lcodes - 257 as ::core::ffi::c_int) as ush as ::core::ffi::c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len;
    }
    let mut len_0: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    if (*s).bi_valid > Buf_size - len_0 {
        let mut val_0: ::core::ffi::c_int = dcodes - 1 as ::core::ffi::c_int;
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (val_0 as ush as ::core::ffi::c_int) << (*s).bi_valid) as ush;
        let fresh23 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh23 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh24 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh24 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
        (*s).bi_buf = (val_0 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len_0 - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | ((dcodes - 1 as ::core::ffi::c_int) as ush as ::core::ffi::c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len_0;
    }
    let mut len_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    if (*s).bi_valid > Buf_size - len_1 {
        let mut val_1: ::core::ffi::c_int = blcodes - 4 as ::core::ffi::c_int;
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (val_1 as ush as ::core::ffi::c_int) << (*s).bi_valid) as ush;
        let fresh25 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh25 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh26 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh26 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
        (*s).bi_buf = (val_1 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len_1 - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | ((blcodes - 4 as ::core::ffi::c_int) as ush as ::core::ffi::c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len_1;
    }
    rank = 0 as ::core::ffi::c_int;
    while rank < blcodes {
        let mut len_2: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
        if (*s).bi_valid > Buf_size - len_2 {
            let mut val_2: ::core::ffi::c_int =
                (*s).bl_tree[bl_order[rank as usize] as usize].dl.len as ::core::ffi::c_int;
            (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                | (val_2 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                as ush;
            let fresh27 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh27 as isize) =
                ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
            let fresh28 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh28 as isize) =
                ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
            (*s).bi_buf = (val_2 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
            (*s).bi_valid += len_2 - Buf_size;
        } else {
            (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                | ((*s).bl_tree[bl_order[rank as usize] as usize].dl.len as ::core::ffi::c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len_2;
        }
        rank += 1;
    }
    send_tree(
        s,
        &raw mut (*s).dyn_ltree as *mut ct_data_s as *mut ct_data,
        lcodes - 1 as ::core::ffi::c_int,
    );
    send_tree(
        s,
        &raw mut (*s).dyn_dtree as *mut ct_data_s as *mut ct_data,
        dcodes - 1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _tr_stored_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut stored_len: ulg,
    mut last: ::core::ffi::c_int,
) {
    let mut len: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    if (*s).bi_valid > Buf_size - len {
        let mut val: ::core::ffi::c_int =
            ((0 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) + last;
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (val as ush as ::core::ffi::c_int) << (*s).bi_valid) as ush;
        let fresh45 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh45 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh46 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh46 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
        (*s).bi_buf = (val as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | ((((0 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) + last) as ush
                as ::core::ffi::c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    bi_windup(s);
    let fresh47 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh47 as isize) =
        (stored_len as ush as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
    let fresh48 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh48 as isize) =
        (stored_len as ush as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
    let fresh49 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh49 as isize) =
        (!stored_len as ush as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
    let fresh50 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh50 as isize) =
        (!stored_len as ush as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
    memcpy(
        (*s).pending_buf.offset((*s).pending as isize) as *mut ::core::ffi::c_void,
        buf as *mut Bytef as *const ::core::ffi::c_void,
        stored_len as size_t,
    );
    (*s).pending = ((*s).pending as ::core::ffi::c_ulong)
        .wrapping_add(stored_len as ::core::ffi::c_ulong) as ulg as ulg;
}
#[no_mangle]
pub unsafe extern "C" fn _tr_flush_bits(mut s: *mut deflate_state) {
    bi_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn _tr_align(mut s: *mut deflate_state) {
    let mut len: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    if (*s).bi_valid > Buf_size - len {
        let mut val: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (val as ush as ::core::ffi::c_int) << (*s).bi_valid) as ush;
        let fresh58 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh58 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh59 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh59 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
        (*s).bi_buf = (val as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as ush as ::core::ffi::c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    let mut len_0: ::core::ffi::c_int =
        static_ltree[256 as ::core::ffi::c_int as usize].dl.len as ::core::ffi::c_int;
    if (*s).bi_valid > Buf_size - len_0 {
        let mut val_0: ::core::ffi::c_int =
            static_ltree[256 as ::core::ffi::c_int as usize].fc.code as ::core::ffi::c_int;
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (val_0 as ush as ::core::ffi::c_int) << (*s).bi_valid) as ush;
        let fresh60 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh60 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh61 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh61 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
        (*s).bi_buf = (val_0 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len_0 - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (static_ltree[256 as ::core::ffi::c_int as usize].fc.code as ::core::ffi::c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len_0;
    }
    bi_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn _tr_flush_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut stored_len: ulg,
    mut last: ::core::ffi::c_int,
) {
    let mut opt_lenb: ulg = 0;
    let mut static_lenb: ulg = 0;
    let mut max_blindex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*s).level > 0 as ::core::ffi::c_int {
        if (*(*s).strm).data_type == Z_UNKNOWN {
            (*(*s).strm).data_type = detect_data_type(s);
        }
        build_tree(s, &raw mut (*s).l_desc as *mut tree_desc);
        build_tree(s, &raw mut (*s).d_desc as *mut tree_desc);
        max_blindex = build_bl_tree(s);
        opt_lenb = (((*s).opt_len as ::core::ffi::c_ulong)
            .wrapping_add(3 as ::core::ffi::c_ulong)
            .wrapping_add(7 as ::core::ffi::c_ulong)
            >> 3 as ::core::ffi::c_int) as ulg;
        static_lenb = (((*s).static_len as ::core::ffi::c_ulong)
            .wrapping_add(3 as ::core::ffi::c_ulong)
            .wrapping_add(7 as ::core::ffi::c_ulong)
            >> 3 as ::core::ffi::c_int) as ulg;
        if static_lenb <= opt_lenb {
            opt_lenb = static_lenb;
        }
    } else {
        static_lenb =
            (stored_len as ::core::ffi::c_ulong).wrapping_add(5 as ::core::ffi::c_ulong) as ulg;
        opt_lenb = static_lenb;
    }
    if (stored_len as ::core::ffi::c_ulong).wrapping_add(4 as ::core::ffi::c_ulong) <= opt_lenb
        && !buf.is_null()
    {
        _tr_stored_block(s, buf, stored_len, last);
    } else if (*s).strategy == Z_FIXED || static_lenb == opt_lenb {
        let mut len: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
        if (*s).bi_valid > Buf_size - len {
            let mut val: ::core::ffi::c_int =
                ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) + last;
            (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                | (val as ush as ::core::ffi::c_int) << (*s).bi_valid)
                as ush;
            let fresh1 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh1 as isize) =
                ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
            let fresh2 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh2 as isize) =
                ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
            (*s).bi_buf = (val as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
            (*s).bi_valid += len - Buf_size;
        } else {
            (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                | ((((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) + last) as ush
                    as ::core::ffi::c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len;
        }
        compress_block(
            s,
            &raw const static_ltree as *const ct_data,
            &raw const static_dtree as *const ct_data,
        );
    } else {
        let mut len_0: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
        if (*s).bi_valid > Buf_size - len_0 {
            let mut val_0: ::core::ffi::c_int =
                ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) + last;
            (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                | (val_0 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                as ush;
            let fresh3 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh3 as isize) =
                ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
            let fresh4 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh4 as isize) =
                ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
            (*s).bi_buf = (val_0 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
            (*s).bi_valid += len_0 - Buf_size;
        } else {
            (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                | ((((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) + last) as ush
                    as ::core::ffi::c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len_0;
        }
        send_all_trees(
            s,
            (*s).l_desc.max_code + 1 as ::core::ffi::c_int,
            (*s).d_desc.max_code + 1 as ::core::ffi::c_int,
            max_blindex + 1 as ::core::ffi::c_int,
        );
        compress_block(
            s,
            &raw mut (*s).dyn_ltree as *mut ct_data_s as *const ct_data,
            &raw mut (*s).dyn_dtree as *mut ct_data_s as *const ct_data,
        );
    }
    init_block(s);
    if last != 0 {
        bi_windup(s);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _tr_tally(
    mut s: *mut deflate_state,
    mut dist: ::core::ffi::c_uint,
    mut lc: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    *(*s).d_buf.offset((*s).last_lit as isize) = dist as ush as ushf;
    let fresh0 = (*s).last_lit;
    (*s).last_lit = (*s).last_lit.wrapping_add(1);
    *(*s).l_buf.offset(fresh0 as isize) = lc as uch as uchf;
    if dist == 0 as ::core::ffi::c_uint {
        (*s).dyn_ltree[lc as usize].fc.freq = (*s).dyn_ltree[lc as usize].fc.freq.wrapping_add(1);
    } else {
        (*s).matches = (*s).matches.wrapping_add(1);
        dist = dist.wrapping_sub(1);
        (*s).dyn_ltree[(_length_code[lc as usize] as ::core::ffi::c_int
            + LITERALS
            + 1 as ::core::ffi::c_int) as usize]
            .fc
            .freq = (*s).dyn_ltree[(_length_code[lc as usize] as ::core::ffi::c_int
            + LITERALS
            + 1 as ::core::ffi::c_int) as usize]
            .fc
            .freq
            .wrapping_add(1);
        (*s).dyn_dtree[(if dist < 256 as ::core::ffi::c_uint {
            _dist_code[dist as usize] as ::core::ffi::c_int
        } else {
            _dist_code[(256 as ::core::ffi::c_uint).wrapping_add(dist >> 7 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int
        }) as usize]
            .fc
            .freq = (*s).dyn_dtree[(if dist < 256 as ::core::ffi::c_uint {
            _dist_code[dist as usize] as ::core::ffi::c_int
        } else {
            _dist_code[(256 as ::core::ffi::c_uint).wrapping_add(dist >> 7 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int
        }) as usize]
            .fc
            .freq
            .wrapping_add(1);
    }
    return ((*s).last_lit
        == ((*s).lit_bufsize as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn compress_block(
    mut s: *mut deflate_state,
    mut ltree: *const ct_data,
    mut dtree: *const ct_data,
) {
    let mut dist: ::core::ffi::c_uint = 0;
    let mut lc: ::core::ffi::c_int = 0;
    let mut lx: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut code: ::core::ffi::c_uint = 0;
    let mut extra: ::core::ffi::c_int = 0;
    if (*s).last_lit != 0 as ::core::ffi::c_uint {
        loop {
            dist = *(*s).d_buf.offset(lx as isize) as ::core::ffi::c_uint;
            let fresh8 = lx;
            lx = lx.wrapping_add(1);
            lc = *(*s).l_buf.offset(fresh8 as isize) as ::core::ffi::c_int;
            if dist == 0 as ::core::ffi::c_uint {
                let mut len: ::core::ffi::c_int =
                    (*ltree.offset(lc as isize)).dl.len as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len {
                    let mut val: ::core::ffi::c_int =
                        (*ltree.offset(lc as isize)).fc.code as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh9 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh9 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh10 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh10 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((*ltree.offset(lc as isize)).fc.code as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len;
                }
            } else {
                code = _length_code[lc as usize] as ::core::ffi::c_uint;
                let mut len_0: ::core::ffi::c_int = (*ltree.offset(
                    code.wrapping_add(256 as ::core::ffi::c_uint)
                        .wrapping_add(1 as ::core::ffi::c_uint) as isize,
                ))
                .dl
                .len as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len_0 {
                    let mut val_0: ::core::ffi::c_int = (*ltree.offset(
                        code.wrapping_add(256 as ::core::ffi::c_uint)
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            as isize,
                    ))
                    .fc
                    .code
                        as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val_0 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh11 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh11 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh12 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh12 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val_0 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_0 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((*ltree.offset(
                            code.wrapping_add(256 as ::core::ffi::c_uint)
                                .wrapping_add(1 as ::core::ffi::c_uint)
                                as isize,
                        ))
                        .fc
                        .code as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_0;
                }
                extra = extra_lbits[code as usize];
                if extra != 0 as ::core::ffi::c_int {
                    lc -= base_length[code as usize];
                    let mut len_1: ::core::ffi::c_int = extra;
                    if (*s).bi_valid > Buf_size - len_1 {
                        let mut val_1: ::core::ffi::c_int = lc;
                        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                            | (val_1 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                            as ush;
                        let fresh13 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh13 as isize) =
                            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                        let fresh14 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh14 as isize) =
                            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                        (*s).bi_buf =
                            (val_1 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                        (*s).bi_valid += len_1 - Buf_size;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                            | (lc as ush as ::core::ffi::c_int) << (*s).bi_valid)
                            as ush;
                        (*s).bi_valid += len_1;
                    }
                }
                dist = dist.wrapping_sub(1);
                code = (if dist < 256 as ::core::ffi::c_uint {
                    _dist_code[dist as usize] as ::core::ffi::c_int
                } else {
                    _dist_code[(256 as ::core::ffi::c_uint)
                        .wrapping_add(dist >> 7 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                }) as ::core::ffi::c_uint;
                let mut len_2: ::core::ffi::c_int =
                    (*dtree.offset(code as isize)).dl.len as ::core::ffi::c_int;
                if (*s).bi_valid > Buf_size - len_2 {
                    let mut val_2: ::core::ffi::c_int =
                        (*dtree.offset(code as isize)).fc.code as ::core::ffi::c_int;
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | (val_2 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                        as ush;
                    let fresh15 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh15 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                    let fresh16 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh16 as isize) =
                        ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                    (*s).bi_buf =
                        (val_2 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                        | ((*dtree.offset(code as isize)).fc.code as ::core::ffi::c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2;
                }
                extra = extra_dbits[code as usize];
                if extra != 0 as ::core::ffi::c_int {
                    dist = dist.wrapping_sub(base_dist[code as usize] as ::core::ffi::c_uint);
                    let mut len_3: ::core::ffi::c_int = extra;
                    if (*s).bi_valid > Buf_size - len_3 {
                        let mut val_3: ::core::ffi::c_int = dist as ::core::ffi::c_int;
                        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                            | (val_3 as ush as ::core::ffi::c_int) << (*s).bi_valid)
                            as ush;
                        let fresh17 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh17 as isize) =
                            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
                        let fresh18 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh18 as isize) =
                            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
                        (*s).bi_buf =
                            (val_3 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
                        (*s).bi_valid += len_3 - Buf_size;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
                            | (dist as ush as ::core::ffi::c_int) << (*s).bi_valid)
                            as ush;
                        (*s).bi_valid += len_3;
                    }
                }
            }
            if !(lx < (*s).last_lit) {
                break;
            }
        }
    }
    let mut len_4: ::core::ffi::c_int =
        (*ltree.offset(256 as ::core::ffi::c_int as isize)).dl.len as ::core::ffi::c_int;
    if (*s).bi_valid > Buf_size - len_4 {
        let mut val_4: ::core::ffi::c_int =
            (*ltree.offset(256 as ::core::ffi::c_int as isize)).fc.code as ::core::ffi::c_int;
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | (val_4 as ush as ::core::ffi::c_int) << (*s).bi_valid) as ush;
        let fresh19 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh19 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh20 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh20 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
        (*s).bi_buf = (val_4 as ush as ::core::ffi::c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len_4 - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int
            | ((*ltree.offset(256 as ::core::ffi::c_int as isize)).fc.code as ::core::ffi::c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len_4;
    };
}
unsafe extern "C" fn detect_data_type(mut s: *mut deflate_state) -> ::core::ffi::c_int {
    let mut black_mask: ::core::ffi::c_ulong = 0xf3ffc07f as ::core::ffi::c_ulong;
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    while n <= 31 as ::core::ffi::c_int {
        if black_mask & 1 as ::core::ffi::c_ulong != 0
            && (*s).dyn_ltree[n as usize].fc.freq as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            return Z_BINARY;
        }
        n += 1;
        black_mask >>= 1 as ::core::ffi::c_int;
    }
    if (*s).dyn_ltree[9 as ::core::ffi::c_int as usize].fc.freq as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || (*s).dyn_ltree[10 as ::core::ffi::c_int as usize].fc.freq as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        || (*s).dyn_ltree[13 as ::core::ffi::c_int as usize].fc.freq as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        return Z_TEXT;
    }
    n = 32 as ::core::ffi::c_int;
    while n < LITERALS {
        if (*s).dyn_ltree[n as usize].fc.freq as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            return Z_TEXT;
        }
        n += 1;
    }
    return Z_BINARY;
}
unsafe extern "C" fn bi_reverse(
    mut code: ::core::ffi::c_uint,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut res: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    loop {
        res |= code & 1 as ::core::ffi::c_uint;
        code >>= 1 as ::core::ffi::c_int;
        res <<= 1 as ::core::ffi::c_int;
        len -= 1;
        if !(len > 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return res >> 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn bi_flush(mut s: *mut deflate_state) {
    if (*s).bi_valid == 16 as ::core::ffi::c_int {
        let fresh55 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh55 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh56 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh56 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
        (*s).bi_buf = 0 as ush;
        (*s).bi_valid = 0 as ::core::ffi::c_int;
    } else if (*s).bi_valid >= 8 as ::core::ffi::c_int {
        let fresh57 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh57 as isize) = (*s).bi_buf as Byte;
        (*s).bi_buf = ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ush;
        (*s).bi_valid -= 8 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn bi_windup(mut s: *mut deflate_state) {
    if (*s).bi_valid > 8 as ::core::ffi::c_int {
        let fresh5 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh5 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uch;
        let fresh6 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh6 as isize) =
            ((*s).bi_buf as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uch;
    } else if (*s).bi_valid > 0 as ::core::ffi::c_int {
        let fresh7 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh7 as isize) = (*s).bi_buf as Byte;
    }
    (*s).bi_buf = 0 as ush;
    (*s).bi_valid = 0 as ::core::ffi::c_int;
}
