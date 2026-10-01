use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub type intf = c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub strm: z_streamp,
    pub status: c_int,
    pub pending_buf: *mut Bytef,
    pub pending_buf_size: ulg,
    pub pending_out: *mut Bytef,
    pub pending: ulg,
    pub wrap: c_int,
    pub gzhead: gz_headerp,
    pub gzindex: ulg,
    pub method: Byte,
    pub last_flush: c_int,
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
    pub block_start: c_long,
    pub match_length: uInt,
    pub prev_match: IPos,
    pub match_available: c_int,
    pub strstart: uInt,
    pub match_start: uInt,
    pub lookahead: uInt,
    pub prev_length: uInt,
    pub max_chain_length: uInt,
    pub max_lazy_match: uInt,
    pub level: c_int,
    pub strategy: c_int,
    pub good_match: uInt,
    pub nice_match: c_int,
    pub dyn_ltree: [ct_data_s; 573],
    pub dyn_dtree: [ct_data_s; 61],
    pub bl_tree: [ct_data_s; 39],
    pub l_desc: tree_desc_s,
    pub d_desc: tree_desc_s,
    pub bl_desc: tree_desc_s,
    pub bl_count: [ush; 16],
    pub heap: [c_int; 573],
    pub heap_len: c_int,
    pub heap_max: c_int,
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
    pub bi_valid: c_int,
    pub high_water: ulg,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc_s {
    pub dyn_tree: *mut ct_data,
    pub max_code: c_int,
    pub stat_desc: *const static_tree_desc,
}
pub type static_tree_desc = static_tree_desc_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct static_tree_desc_s {
    pub static_tree: *const ct_data,
    pub extra_bits: *const intf,
    pub extra_base: c_int,
    pub elems: c_int,
    pub max_length: c_int,
}
pub type ct_data = ct_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data_s {
    pub fc: C2RustUnnamed_hu1f66356c,
    pub dl: C2RustUnnamed_hu165d5d84,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_hu165d5d84 {
    pub dad: ush,
    pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_hu1f66356c {
    pub freq: ush,
    pub code: ush,
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
    pub msg: *mut c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type tree_desc = tree_desc_s;
pub type deflate_state = internal_state;

pub const Z_BINARY: c_int = 0 as c_int;
pub const Z_TEXT: c_int = 1 as c_int;

pub const LENGTH_CODES: c_int = 29 as c_int;

pub const L_CODES: c_int = LITERALS + 1 as c_int + LENGTH_CODES;
pub const D_CODES: c_int = 30 as c_int;
pub const BL_CODES: c_int = 19 as c_int;
pub const HEAP_SIZE: c_int =
    2 as c_int * L_CODES + 1 as c_int;
pub const MAX_BITS: c_int = 15 as c_int;

pub const MAX_BL_BITS: c_int = 7 as c_int;
pub const END_BLOCK: c_int = 256 as c_int;
pub const REP_3_6: c_int = 16 as c_int;
pub const REPZ_3_10: c_int = 17 as c_int;
pub const REPZ_11_138: c_int = 18 as c_int;
static mut extra_lbits: [c_int; 29] = [
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    0 as c_int,
];
static mut extra_dbits: [c_int; 30] = [
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    1 as c_int,
    1 as c_int,
    2 as c_int,
    2 as c_int,
    3 as c_int,
    3 as c_int,
    4 as c_int,
    4 as c_int,
    5 as c_int,
    5 as c_int,
    6 as c_int,
    6 as c_int,
    7 as c_int,
    7 as c_int,
    8 as c_int,
    8 as c_int,
    9 as c_int,
    9 as c_int,
    10 as c_int,
    10 as c_int,
    11 as c_int,
    11 as c_int,
    12 as c_int,
    12 as c_int,
    13 as c_int,
    13 as c_int,
];
static mut extra_blbits: [c_int; 19] = [
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    0 as c_int,
    2 as c_int,
    3 as c_int,
    7 as c_int,
];
static mut bl_order: [uch; 19] = [
    16 as c_int as uch,
    17 as c_int as uch,
    18 as c_int as uch,
    0 as c_int as uch,
    8 as c_int as uch,
    7 as c_int as uch,
    9 as c_int as uch,
    6 as c_int as uch,
    10 as c_int as uch,
    5 as c_int as uch,
    11 as c_int as uch,
    4 as c_int as uch,
    12 as c_int as uch,
    3 as c_int as uch,
    13 as c_int as uch,
    2 as c_int as uch,
    14 as c_int as uch,
    1 as c_int as uch,
    15 as c_int as uch,
];
static mut static_ltree: [ct_data; 288] = [
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 12 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 140 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 76 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 204 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 44 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 172 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 108 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 236 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 28 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 156 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 92 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 220 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 60 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 188 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 124 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 252 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 2 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 130 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 66 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 194 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 34 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 162 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 98 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 226 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 18 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 146 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 82 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 210 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 50 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 178 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 114 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 242 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 10 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 138 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 74 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 202 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 42 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 170 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 106 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 234 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 26 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 154 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 90 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 218 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 58 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 186 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 122 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 250 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 6 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 134 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 70 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 198 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 38 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 166 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 102 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 230 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 22 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 150 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 86 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 214 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 54 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 182 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 118 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 246 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 14 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 142 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 78 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 206 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 46 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 174 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 110 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 238 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 30 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 158 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 94 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 222 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 62 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 190 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 126 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 254 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 1 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 129 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 65 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 193 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 33 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 161 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 97 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 225 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 17 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 145 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 81 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 209 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 49 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 177 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 113 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 241 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 9 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 137 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 73 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 201 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 41 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 169 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 105 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 233 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 25 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 153 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 89 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 217 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 57 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 185 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 121 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 249 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 5 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 133 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 69 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 197 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 37 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 165 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 101 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 229 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 21 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 149 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 85 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 213 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 53 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 181 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 117 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 245 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 13 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 141 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 77 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 205 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 45 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 173 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 109 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 237 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 29 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 157 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 93 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 221 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 61 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 189 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 125 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 253 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 19 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 275 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 147 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 403 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 83 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 339 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 211 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 467 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 51 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 307 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 179 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 435 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 115 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 371 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 243 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 499 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 11 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 267 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 139 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 395 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 75 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 331 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 203 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 459 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 43 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 299 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 171 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 427 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 107 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 363 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 235 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 491 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 27 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 283 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 155 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 411 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 91 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 347 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 219 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 475 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 59 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 315 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 187 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 443 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 123 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 379 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 251 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 507 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 7 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 263 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 135 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 391 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 71 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 327 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 199 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 455 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 39 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 295 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 167 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 423 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 103 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 359 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 231 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 487 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 23 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 279 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 151 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 407 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 87 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 343 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 215 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 471 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 55 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 311 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 183 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 439 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 119 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 375 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 247 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 503 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 15 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 271 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 143 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 399 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 79 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 335 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 207 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 463 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 47 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 303 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 175 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 431 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 111 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 367 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 239 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 495 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 31 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 287 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 159 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 415 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 95 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 351 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 223 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 479 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 63 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 319 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 191 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 447 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 127 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 383 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 255 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 511 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 9 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 0 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 64 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 32 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 96 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 16 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 80 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 48 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 112 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 8 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 72 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 40 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 104 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 24 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 88 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 56 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 120 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 4 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 68 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 36 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 100 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 20 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 84 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 52 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 116 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 7 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 3 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 131 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 67 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 195 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 35 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 163 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 99 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 227 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 8 as c_int as ush,
        },
    },
];
static mut static_dtree: [ct_data; 30] = [
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 0 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 16 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 8 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 24 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 4 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 20 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 12 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 28 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 2 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 18 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 10 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 26 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 6 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 22 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 14 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 30 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 1 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 17 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 9 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 25 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 5 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 21 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 13 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 29 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 3 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 19 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 11 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 27 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 7 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
    ct_data_s {
        fc: C2RustUnnamed_hu1f66356c {
            freq: 23 as c_int as ush,
        },
        dl: C2RustUnnamed_hu165d5d84 {
            dad: 5 as c_int as ush,
        },
    },
];
#[no_mangle]
pub static mut _dist_code: [uch; 512] = [
    0 as c_int as uch,
    1 as c_int as uch,
    2 as c_int as uch,
    3 as c_int as uch,
    4 as c_int as uch,
    4 as c_int as uch,
    5 as c_int as uch,
    5 as c_int as uch,
    6 as c_int as uch,
    6 as c_int as uch,
    6 as c_int as uch,
    6 as c_int as uch,
    7 as c_int as uch,
    7 as c_int as uch,
    7 as c_int as uch,
    7 as c_int as uch,
    8 as c_int as uch,
    8 as c_int as uch,
    8 as c_int as uch,
    8 as c_int as uch,
    8 as c_int as uch,
    8 as c_int as uch,
    8 as c_int as uch,
    8 as c_int as uch,
    9 as c_int as uch,
    9 as c_int as uch,
    9 as c_int as uch,
    9 as c_int as uch,
    9 as c_int as uch,
    9 as c_int as uch,
    9 as c_int as uch,
    9 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    0 as c_int as uch,
    0 as c_int as uch,
    16 as c_int as uch,
    17 as c_int as uch,
    18 as c_int as uch,
    18 as c_int as uch,
    19 as c_int as uch,
    19 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    28 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
    29 as c_int as uch,
];
#[no_mangle]
pub static mut _length_code: [uch; 256] = [
    0 as c_int as uch,
    1 as c_int as uch,
    2 as c_int as uch,
    3 as c_int as uch,
    4 as c_int as uch,
    5 as c_int as uch,
    6 as c_int as uch,
    7 as c_int as uch,
    8 as c_int as uch,
    8 as c_int as uch,
    9 as c_int as uch,
    9 as c_int as uch,
    10 as c_int as uch,
    10 as c_int as uch,
    11 as c_int as uch,
    11 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    12 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    13 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    14 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    15 as c_int as uch,
    16 as c_int as uch,
    16 as c_int as uch,
    16 as c_int as uch,
    16 as c_int as uch,
    16 as c_int as uch,
    16 as c_int as uch,
    16 as c_int as uch,
    16 as c_int as uch,
    17 as c_int as uch,
    17 as c_int as uch,
    17 as c_int as uch,
    17 as c_int as uch,
    17 as c_int as uch,
    17 as c_int as uch,
    17 as c_int as uch,
    17 as c_int as uch,
    18 as c_int as uch,
    18 as c_int as uch,
    18 as c_int as uch,
    18 as c_int as uch,
    18 as c_int as uch,
    18 as c_int as uch,
    18 as c_int as uch,
    18 as c_int as uch,
    19 as c_int as uch,
    19 as c_int as uch,
    19 as c_int as uch,
    19 as c_int as uch,
    19 as c_int as uch,
    19 as c_int as uch,
    19 as c_int as uch,
    19 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    20 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    21 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    22 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    23 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    24 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    25 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    26 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    27 as c_int as uch,
    28 as c_int as uch,
];
static mut base_length: [c_int; 29] = [
    0 as c_int,
    1 as c_int,
    2 as c_int,
    3 as c_int,
    4 as c_int,
    5 as c_int,
    6 as c_int,
    7 as c_int,
    8 as c_int,
    10 as c_int,
    12 as c_int,
    14 as c_int,
    16 as c_int,
    20 as c_int,
    24 as c_int,
    28 as c_int,
    32 as c_int,
    40 as c_int,
    48 as c_int,
    56 as c_int,
    64 as c_int,
    80 as c_int,
    96 as c_int,
    112 as c_int,
    128 as c_int,
    160 as c_int,
    192 as c_int,
    224 as c_int,
    0 as c_int,
];
static mut base_dist: [c_int; 30] = [
    0 as c_int,
    1 as c_int,
    2 as c_int,
    3 as c_int,
    4 as c_int,
    6 as c_int,
    8 as c_int,
    12 as c_int,
    16 as c_int,
    24 as c_int,
    32 as c_int,
    48 as c_int,
    64 as c_int,
    96 as c_int,
    128 as c_int,
    192 as c_int,
    256 as c_int,
    384 as c_int,
    512 as c_int,
    768 as c_int,
    1024 as c_int,
    1536 as c_int,
    2048 as c_int,
    3072 as c_int,
    4096 as c_int,
    6144 as c_int,
    8192 as c_int,
    12288 as c_int,
    16384 as c_int,
    24576 as c_int,
];
static mut static_l_desc: static_tree_desc = unsafe {
    static_tree_desc_s {
        static_tree: &raw const static_ltree as *const ct_data,
        extra_bits: &raw const extra_lbits as *const intf,
        extra_base: LITERALS + 1 as c_int,
        elems: L_CODES,
        max_length: MAX_BITS,
    }
};
static mut static_d_desc: static_tree_desc = unsafe {
    static_tree_desc_s {
        static_tree: &raw const static_dtree as *const ct_data,
        extra_bits: &raw const extra_dbits as *const intf,
        extra_base: 0 as c_int,
        elems: D_CODES,
        max_length: MAX_BITS,
    }
};
static mut static_bl_desc: static_tree_desc = unsafe {
    static_tree_desc_s {
        static_tree: ::core::ptr::null::<ct_data>(),
        extra_bits: &raw const extra_blbits as *const intf,
        extra_base: 0 as c_int,
        elems: BL_CODES,
        max_length: MAX_BL_BITS,
    }
};
fn tr_static_init() { {} }
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
    (*s).bi_valid = 0 as c_int;
    init_block(s);
}
unsafe fn init_block(mut s: *mut deflate_state) {
    let mut n: c_int = 0;
    n = 0 as c_int;
    while n < L_CODES {
        (*s).dyn_ltree[n as usize].fc.freq = 0 as ush;
        n += 1;
    }
    n = 0 as c_int;
    while n < D_CODES {
        (*s).dyn_dtree[n as usize].fc.freq = 0 as ush;
        n += 1;
    }
    n = 0 as c_int;
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
pub const SMALLEST: c_int = 1 as c_int;
unsafe fn pqdownheap(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut k: c_int,
) {
    let mut v: c_int = (*s).heap[k as usize];
    let mut j: c_int = k << 1 as c_int;
    while j <= (*s).heap_len {
        if j < (*s).heap_len
            && (((*tree.offset((*s).heap[(j + 1 as c_int) as usize] as isize))
                .fc
                .freq as c_int)
                < (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as c_int
                || (*tree.offset((*s).heap[(j + 1 as c_int) as usize] as isize))
                    .fc
                    .freq as c_int
                    == (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as c_int
                    && (*s).depth[(*s).heap[(j + 1 as c_int) as usize] as usize]
                        as c_int
                        <= (*s).depth[(*s).heap[j as usize] as usize] as c_int)
        {
            j += 1;
        }
        if ((*tree.offset(v as isize)).fc.freq as c_int)
            < (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as c_int
            || (*tree.offset(v as isize)).fc.freq as c_int
                == (*tree.offset((*s).heap[j as usize] as isize)).fc.freq as c_int
                && (*s).depth[v as usize] as c_int
                    <= (*s).depth[(*s).heap[j as usize] as usize] as c_int
        {
            break;
        }
        (*s).heap[k as usize] = (*s).heap[j as usize];
        k = j;
        j <<= 1 as c_int;
    }
    (*s).heap[k as usize] = v;
}
unsafe fn gen_bitlen(mut s: *mut deflate_state, mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut max_code: c_int = (*desc).max_code;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut extra: *const intf = (*(*desc).stat_desc).extra_bits;
    let mut base: c_int = (*(*desc).stat_desc).extra_base;
    let mut max_length: c_int = (*(*desc).stat_desc).max_length;
    let mut h: c_int = 0;
    let mut n: c_int = 0;
    let mut m: c_int = 0;
    let mut bits: c_int = 0;
    let mut xbits: c_int = 0;
    let mut f: ush = 0;
    let mut overflow: c_int = 0 as c_int;
    bits = 0 as c_int;
    while bits <= MAX_BITS {
        (*s).bl_count[bits as usize] = 0 as ush;
        bits += 1;
    }
    (*tree.offset((*s).heap[(*s).heap_max as usize] as isize))
        .dl
        .len = 0 as ush;
    h = (*s).heap_max + 1 as c_int;
    while h < HEAP_SIZE {
        n = (*s).heap[h as usize];
        bits = (*tree.offset((*tree.offset(n as isize)).dl.dad as isize))
            .dl
            .len as c_int
            + 1 as c_int;
        if bits > max_length {
            bits = max_length;
            overflow += 1;
        }
        (*tree.offset(n as isize)).dl.len = bits as ush;
        if !(n > max_code) {
            (*s).bl_count[bits as usize] = (*s).bl_count[bits as usize].wrapping_add(1);
            xbits = 0 as c_int;
            if n >= base {
                xbits = *extra.offset((n - base) as isize) as c_int;
            }
            f = (*tree.offset(n as isize)).fc.freq;
            (*s).opt_len = ((*s).opt_len as c_ulong).wrapping_add(
                (f as c_ulong)
                    .wrapping_mul((bits + xbits) as c_uint as c_ulong),
            ) as ulg as ulg;
            if !stree.is_null() {
                (*s).static_len = ((*s).static_len as c_ulong).wrapping_add(
                    (f as c_ulong).wrapping_mul(
                        ((*stree.offset(n as isize)).dl.len as c_int + xbits)
                            as c_uint as c_ulong,
                    ),
                ) as ulg as ulg;
            }
        }
        h += 1;
    }
    if overflow == 0 as c_int {
        return;
    }
    loop {
        bits = max_length - 1 as c_int;
        while (*s).bl_count[bits as usize] as c_int == 0 as c_int {
            bits -= 1;
        }
        (*s).bl_count[bits as usize] = (*s).bl_count[bits as usize].wrapping_sub(1);
        (*s).bl_count[(bits + 1 as c_int) as usize] =
            ((*s).bl_count[(bits + 1 as c_int) as usize] as c_int
                + 2 as c_int) as ush;
        (*s).bl_count[max_length as usize] = (*s).bl_count[max_length as usize].wrapping_sub(1);
        overflow -= 2 as c_int;
        if !(overflow > 0 as c_int) {
            break;
        }
    }
    bits = max_length;
    while bits != 0 as c_int {
        n = (*s).bl_count[bits as usize] as c_int;
        while n != 0 as c_int {
            h -= 1;
            m = (*s).heap[h as usize];
            if m > max_code {
                continue;
            }
            if (*tree.offset(m as isize)).dl.len as c_uint
                != bits as c_uint
            {
                (*s).opt_len = ((*s).opt_len as c_ulong).wrapping_add(
                    (bits as c_ulong)
                        .wrapping_sub((*tree.offset(m as isize)).dl.len as c_ulong)
                        .wrapping_mul((*tree.offset(m as isize)).fc.freq as c_ulong),
                ) as ulg as ulg;
                (*tree.offset(m as isize)).dl.len = bits as ush;
            }
            n -= 1;
        }
        bits -= 1;
    }
}
unsafe fn gen_codes(
    mut tree: *mut ct_data,
    mut max_code: c_int,
    mut bl_count: *mut ushf,
) {
    let mut next_code: [ush; 16] = [0; 16];
    let mut code: c_uint = 0 as c_uint;
    let mut bits: c_int = 0;
    let mut n: c_int = 0;
    bits = 1 as c_int;
    while bits <= MAX_BITS {
        code = code.wrapping_add(
            *bl_count.offset((bits - 1 as c_int) as isize) as c_uint
        ) << 1 as c_int;
        next_code[bits as usize] = code as ush;
        bits += 1;
    }
    n = 0 as c_int;
    while n <= max_code {
        let mut len: c_int = (*tree.offset(n as isize)).dl.len as c_int;
        if !(len == 0 as c_int) {
            let fresh54 = next_code[len as usize];
            next_code[len as usize] = next_code[len as usize].wrapping_add(1);
            (*tree.offset(n as isize)).fc.code =
                bi_reverse(fresh54 as c_uint, len) as ush;
        }
        n += 1;
    }
}
unsafe fn build_tree(mut s: *mut deflate_state, mut desc: *mut tree_desc) {
    let mut tree: *mut ct_data = (*desc).dyn_tree;
    let mut stree: *const ct_data = (*(*desc).stat_desc).static_tree;
    let mut elems: c_int = (*(*desc).stat_desc).elems;
    let mut n: c_int = 0;
    let mut m: c_int = 0;
    let mut max_code: c_int = -(1 as c_int);
    let mut node: c_int = 0;
    (*s).heap_len = 0 as c_int;
    (*s).heap_max = HEAP_SIZE;
    n = 0 as c_int;
    while n < elems {
        if (*tree.offset(n as isize)).fc.freq as c_int != 0 as c_int {
            max_code = n;
            (*s).heap_len += 1;
            (*s).heap[(*s).heap_len as usize] = max_code;
            (*s).depth[n as usize] = 0 as uch;
        } else {
            (*tree.offset(n as isize)).dl.len = 0 as ush;
        }
        n += 1;
    }
    while (*s).heap_len < 2 as c_int {
        (*s).heap_len += 1;
        (*s).heap[(*s).heap_len as usize] = if max_code < 2 as c_int {
            max_code += 1;
            max_code
        } else {
            0 as c_int
        };
        node = (*s).heap[(*s).heap_len as usize];
        (*tree.offset(node as isize)).fc.freq = 1 as ush;
        (*s).depth[node as usize] = 0 as uch;
        (*s).opt_len = (*s).opt_len.wrapping_sub(1);
        if !stree.is_null() {
            (*s).static_len = ((*s).static_len as c_ulong)
                .wrapping_sub((*stree.offset(node as isize)).dl.len as c_ulong)
                as ulg as ulg;
        }
    }
    (*desc).max_code = max_code;
    n = (*s).heap_len / 2 as c_int;
    while n >= 1 as c_int {
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
            ((*tree.offset(n as isize)).fc.freq as c_int
                + (*tree.offset(m as isize)).fc.freq as c_int) as ush;
        (*s).depth[node as usize] = ((if (*s).depth[n as usize] as c_int
            >= (*s).depth[m as usize] as c_int
        {
            (*s).depth[n as usize] as c_int
        } else {
            (*s).depth[m as usize] as c_int
        }) + 1 as c_int) as uch;
        let ref mut fresh52 = (*tree.offset(m as isize)).dl.dad;
        *fresh52 = node as ush;
        (*tree.offset(n as isize)).dl.dad = *fresh52;
        let fresh53 = node;
        node = node + 1;
        (*s).heap[SMALLEST as usize] = fresh53;
        pqdownheap(s, tree, SMALLEST);
        if !((*s).heap_len >= 2 as c_int) {
            break;
        }
    }
    (*s).heap_max -= 1;
    (*s).heap[(*s).heap_max as usize] = (*s).heap[SMALLEST as usize];
    gen_bitlen(s, desc);
    gen_codes(tree, max_code, &raw mut (*s).bl_count as *mut ushf);
}
unsafe fn scan_tree(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut max_code: c_int,
) {
    let mut n: c_int = 0;
    let mut prevlen: c_int = -(1 as c_int);
    let mut curlen: c_int = 0;
    let mut nextlen: c_int =
        (*tree.offset(0 as c_int as isize)).dl.len as c_int;
    let mut count: c_int = 0 as c_int;
    let mut max_count: c_int = 7 as c_int;
    let mut min_count: c_int = 4 as c_int;
    if nextlen == 0 as c_int {
        max_count = 138 as c_int;
        min_count = 3 as c_int;
    }
    (*tree.offset((max_code + 1 as c_int) as isize))
        .dl
        .len = 0xffff as c_int as ush;
    n = 0 as c_int;
    while n <= max_code {
        curlen = nextlen;
        nextlen = (*tree.offset((n + 1 as c_int) as isize))
            .dl
            .len as c_int;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                (*s).bl_tree[curlen as usize].fc.freq =
                    ((*s).bl_tree[curlen as usize].fc.freq as c_int + count) as ush;
            } else if curlen != 0 as c_int {
                if curlen != prevlen {
                    (*s).bl_tree[curlen as usize].fc.freq =
                        (*s).bl_tree[curlen as usize].fc.freq.wrapping_add(1);
                }
                (*s).bl_tree[REP_3_6 as usize].fc.freq =
                    (*s).bl_tree[REP_3_6 as usize].fc.freq.wrapping_add(1);
            } else if count <= 10 as c_int {
                (*s).bl_tree[REPZ_3_10 as usize].fc.freq =
                    (*s).bl_tree[REPZ_3_10 as usize].fc.freq.wrapping_add(1);
            } else {
                (*s).bl_tree[REPZ_11_138 as usize].fc.freq =
                    (*s).bl_tree[REPZ_11_138 as usize].fc.freq.wrapping_add(1);
            }
            count = 0 as c_int;
            prevlen = curlen;
            if nextlen == 0 as c_int {
                max_count = 138 as c_int;
                min_count = 3 as c_int;
            } else if curlen == nextlen {
                max_count = 6 as c_int;
                min_count = 3 as c_int;
            } else {
                max_count = 7 as c_int;
                min_count = 4 as c_int;
            }
        }
        n += 1;
    }
}
unsafe fn send_tree(
    mut s: *mut deflate_state,
    mut tree: *mut ct_data,
    mut max_code: c_int,
) {
    let mut n: c_int = 0;
    let mut prevlen: c_int = -(1 as c_int);
    let mut curlen: c_int = 0;
    let mut nextlen: c_int =
        (*tree.offset(0 as c_int as isize)).dl.len as c_int;
    let mut count: c_int = 0 as c_int;
    let mut max_count: c_int = 7 as c_int;
    let mut min_count: c_int = 4 as c_int;
    if nextlen == 0 as c_int {
        max_count = 138 as c_int;
        min_count = 3 as c_int;
    }
    n = 0 as c_int;
    while n <= max_code {
        curlen = nextlen;
        nextlen = (*tree.offset((n + 1 as c_int) as isize))
            .dl
            .len as c_int;
        count += 1;
        if !(count < max_count && curlen == nextlen) {
            if count < min_count {
                loop {
                    let mut len: c_int =
                        (*s).bl_tree[curlen as usize].dl.len as c_int;
                    if (*s).bi_valid > Buf_size - len {
                        let mut val: c_int =
                            (*s).bl_tree[curlen as usize].fc.code as c_int;
                        (*s).bi_buf = ((*s).bi_buf as c_int
                            | (val as ush as c_int) << (*s).bi_valid)
                            as ush;
                        let fresh29 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh29 as isize) =
                            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                        let fresh30 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh30 as isize) =
                            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                        (*s).bi_buf =
                            (val as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                        (*s).bi_valid += len - Buf_size;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as c_int
                            | ((*s).bl_tree[curlen as usize].fc.code as c_int)
                                << (*s).bi_valid) as ush;
                        (*s).bi_valid += len;
                    }
                    count -= 1;
                    if !(count != 0 as c_int) {
                        break;
                    }
                }
            } else if curlen != 0 as c_int {
                if curlen != prevlen {
                    let mut len_0: c_int =
                        (*s).bl_tree[curlen as usize].dl.len as c_int;
                    if (*s).bi_valid > Buf_size - len_0 {
                        let mut val_0: c_int =
                            (*s).bl_tree[curlen as usize].fc.code as c_int;
                        (*s).bi_buf = ((*s).bi_buf as c_int
                            | (val_0 as ush as c_int) << (*s).bi_valid)
                            as ush;
                        let fresh31 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh31 as isize) =
                            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                        let fresh32 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *(*s).pending_buf.offset(fresh32 as isize) =
                            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                        (*s).bi_buf =
                            (val_0 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                        (*s).bi_valid += len_0 - Buf_size;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as c_int
                            | ((*s).bl_tree[curlen as usize].fc.code as c_int)
                                << (*s).bi_valid) as ush;
                        (*s).bi_valid += len_0;
                    }
                    count -= 1;
                }
                let mut len_1: c_int =
                    (*s).bl_tree[16 as c_int as usize].dl.len as c_int;
                if (*s).bi_valid > Buf_size - len_1 {
                    let mut val_1: c_int =
                        (*s).bl_tree[16 as c_int as usize].fc.code
                            as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val_1 as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh33 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh33 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh34 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh34 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val_1 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_1 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((*s).bl_tree[16 as c_int as usize].fc.code
                            as c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_1;
                }
                let mut len_2: c_int = 2 as c_int;
                if (*s).bi_valid > Buf_size - len_2 {
                    let mut val_2: c_int = count - 3 as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val_2 as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh35 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh35 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh36 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh36 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val_2 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((count - 3 as c_int) as ush as c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2;
                }
            } else if count <= 10 as c_int {
                let mut len_3: c_int =
                    (*s).bl_tree[17 as c_int as usize].dl.len as c_int;
                if (*s).bi_valid > Buf_size - len_3 {
                    let mut val_3: c_int =
                        (*s).bl_tree[17 as c_int as usize].fc.code
                            as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val_3 as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh37 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh37 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh38 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh38 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val_3 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_3 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((*s).bl_tree[17 as c_int as usize].fc.code
                            as c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_3;
                }
                let mut len_4: c_int = 3 as c_int;
                if (*s).bi_valid > Buf_size - len_4 {
                    let mut val_4: c_int = count - 3 as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val_4 as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh39 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh39 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh40 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh40 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val_4 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_4 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((count - 3 as c_int) as ush as c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_4;
                }
            } else {
                let mut len_5: c_int =
                    (*s).bl_tree[18 as c_int as usize].dl.len as c_int;
                if (*s).bi_valid > Buf_size - len_5 {
                    let mut val_5: c_int =
                        (*s).bl_tree[18 as c_int as usize].fc.code
                            as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val_5 as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh41 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh41 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh42 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh42 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val_5 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_5 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((*s).bl_tree[18 as c_int as usize].fc.code
                            as c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_5;
                }
                let mut len_6: c_int = 7 as c_int;
                if (*s).bi_valid > Buf_size - len_6 {
                    let mut val_6: c_int = count - 11 as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val_6 as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh43 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh43 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh44 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *(*s).pending_buf.offset(fresh44 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val_6 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_6 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((count - 11 as c_int) as ush as c_int)
                            << (*s).bi_valid) as ush;
                    (*s).bi_valid += len_6;
                }
            }
            count = 0 as c_int;
            prevlen = curlen;
            if nextlen == 0 as c_int {
                max_count = 138 as c_int;
                min_count = 3 as c_int;
            } else if curlen == nextlen {
                max_count = 6 as c_int;
                min_count = 3 as c_int;
            } else {
                max_count = 7 as c_int;
                min_count = 4 as c_int;
            }
        }
        n += 1;
    }
}
unsafe fn build_bl_tree(mut s: *mut deflate_state) -> c_int {
    let mut max_blindex: c_int = 0;
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
    max_blindex = BL_CODES - 1 as c_int;
    while max_blindex >= 3 as c_int {
        if (*s).bl_tree[bl_order[max_blindex as usize] as usize].dl.len as c_int
            != 0 as c_int
        {
            break;
        }
        max_blindex -= 1;
    }
    (*s).opt_len = ((*s).opt_len as c_ulong).wrapping_add(
        (3 as c_ulong)
            .wrapping_mul(
                (max_blindex as c_ulong).wrapping_add(1 as c_ulong),
            )
            .wrapping_add(5 as c_ulong)
            .wrapping_add(5 as c_ulong)
            .wrapping_add(4 as c_ulong),
    ) as ulg as ulg;
    return max_blindex;
}
unsafe fn send_all_trees(
    mut s: *mut deflate_state,
    mut lcodes: c_int,
    mut dcodes: c_int,
    mut blcodes: c_int,
) {
    let mut rank: c_int = 0;
    let mut len: c_int = 5 as c_int;
    if (*s).bi_valid > Buf_size - len {
        let mut val: c_int = lcodes - 257 as c_int;
        (*s).bi_buf = ((*s).bi_buf as c_int
            | (val as ush as c_int) << (*s).bi_valid) as ush;
        let fresh21 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh21 as isize) =
            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
        let fresh22 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh22 as isize) =
            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
        (*s).bi_buf = (val as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as c_int
            | ((lcodes - 257 as c_int) as ush as c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len;
    }
    let mut len_0: c_int = 5 as c_int;
    if (*s).bi_valid > Buf_size - len_0 {
        let mut val_0: c_int = dcodes - 1 as c_int;
        (*s).bi_buf = ((*s).bi_buf as c_int
            | (val_0 as ush as c_int) << (*s).bi_valid) as ush;
        let fresh23 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh23 as isize) =
            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
        let fresh24 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh24 as isize) =
            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
        (*s).bi_buf = (val_0 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len_0 - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as c_int
            | ((dcodes - 1 as c_int) as ush as c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len_0;
    }
    let mut len_1: c_int = 4 as c_int;
    if (*s).bi_valid > Buf_size - len_1 {
        let mut val_1: c_int = blcodes - 4 as c_int;
        (*s).bi_buf = ((*s).bi_buf as c_int
            | (val_1 as ush as c_int) << (*s).bi_valid) as ush;
        let fresh25 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh25 as isize) =
            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
        let fresh26 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh26 as isize) =
            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
        (*s).bi_buf = (val_1 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len_1 - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as c_int
            | ((blcodes - 4 as c_int) as ush as c_int) << (*s).bi_valid)
            as ush;
        (*s).bi_valid += len_1;
    }
    rank = 0 as c_int;
    while rank < blcodes {
        let mut len_2: c_int = 3 as c_int;
        if (*s).bi_valid > Buf_size - len_2 {
            let mut val_2: c_int =
                (*s).bl_tree[bl_order[rank as usize] as usize].dl.len as c_int;
            (*s).bi_buf = ((*s).bi_buf as c_int
                | (val_2 as ush as c_int) << (*s).bi_valid)
                as ush;
            let fresh27 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh27 as isize) =
                ((*s).bi_buf as c_int & 0xff as c_int) as uch;
            let fresh28 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh28 as isize) =
                ((*s).bi_buf as c_int >> 8 as c_int) as uch;
            (*s).bi_buf = (val_2 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
            (*s).bi_valid += len_2 - Buf_size;
        } else {
            (*s).bi_buf = ((*s).bi_buf as c_int
                | ((*s).bl_tree[bl_order[rank as usize] as usize].dl.len as c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len_2;
        }
        rank += 1;
    }
    send_tree(
        s,
        &raw mut (*s).dyn_ltree as *mut ct_data_s as *mut ct_data,
        lcodes - 1 as c_int,
    );
    send_tree(
        s,
        &raw mut (*s).dyn_dtree as *mut ct_data_s as *mut ct_data,
        dcodes - 1 as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _tr_stored_block(
    mut s: *mut deflate_state,
    mut buf: *mut charf,
    mut stored_len: ulg,
    mut last: c_int,
) {
    let mut len: c_int = 3 as c_int;
    if (*s).bi_valid > Buf_size - len {
        let mut val: c_int =
            ((0 as c_int) << 1 as c_int) + last;
        (*s).bi_buf = ((*s).bi_buf as c_int
            | (val as ush as c_int) << (*s).bi_valid) as ush;
        let fresh45 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh45 as isize) =
            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
        let fresh46 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh46 as isize) =
            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
        (*s).bi_buf = (val as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as c_int
            | ((((0 as c_int) << 1 as c_int) + last) as ush
                as c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    bi_windup(s);
    let fresh47 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh47 as isize) =
        (stored_len as ush as c_int & 0xff as c_int) as uch;
    let fresh48 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh48 as isize) =
        (stored_len as ush as c_int >> 8 as c_int) as uch;
    let fresh49 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh49 as isize) =
        (!stored_len as ush as c_int & 0xff as c_int) as uch;
    let fresh50 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh50 as isize) =
        (!stored_len as ush as c_int >> 8 as c_int) as uch;
    memcpy(
        (*s).pending_buf.offset((*s).pending as isize) as *mut c_void,
        buf as *mut Bytef as *const c_void,
        stored_len as size_t,
    );
    (*s).pending = ((*s).pending as c_ulong)
        .wrapping_add(stored_len as c_ulong) as ulg as ulg;
}
#[no_mangle]
pub unsafe extern "C" fn _tr_flush_bits(mut s: *mut deflate_state) {
    bi_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn _tr_align(mut s: *mut deflate_state) {
    let mut len: c_int = 3 as c_int;
    if (*s).bi_valid > Buf_size - len {
        let mut val: c_int = (1 as c_int) << 1 as c_int;
        (*s).bi_buf = ((*s).bi_buf as c_int
            | (val as ush as c_int) << (*s).bi_valid) as ush;
        let fresh58 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh58 as isize) =
            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
        let fresh59 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh59 as isize) =
            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
        (*s).bi_buf = (val as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as c_int
            | (((1 as c_int) << 1 as c_int) as ush as c_int)
                << (*s).bi_valid) as ush;
        (*s).bi_valid += len;
    }
    let mut len_0: c_int =
        static_ltree[256 as c_int as usize].dl.len as c_int;
    if (*s).bi_valid > Buf_size - len_0 {
        let mut val_0: c_int =
            static_ltree[256 as c_int as usize].fc.code as c_int;
        (*s).bi_buf = ((*s).bi_buf as c_int
            | (val_0 as ush as c_int) << (*s).bi_valid) as ush;
        let fresh60 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh60 as isize) =
            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
        let fresh61 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh61 as isize) =
            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
        (*s).bi_buf = (val_0 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
        (*s).bi_valid += len_0 - Buf_size;
    } else {
        (*s).bi_buf = ((*s).bi_buf as c_int
            | (static_ltree[256 as c_int as usize].fc.code as c_int)
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
    mut last: c_int,
) {
    let mut opt_lenb: ulg = 0;
    let mut static_lenb: ulg = 0;
    let mut max_blindex: c_int = 0 as c_int;
    if (*s).level > 0 as c_int {
        if (*(*s).strm).data_type == Z_UNKNOWN {
            (*(*s).strm).data_type = detect_data_type(s);
        }
        build_tree(s, &raw mut (*s).l_desc as *mut tree_desc);
        build_tree(s, &raw mut (*s).d_desc as *mut tree_desc);
        max_blindex = build_bl_tree(s);
        opt_lenb = (((*s).opt_len as c_ulong)
            .wrapping_add(3 as c_ulong)
            .wrapping_add(7 as c_ulong)
            >> 3 as c_int) as ulg;
        static_lenb = (((*s).static_len as c_ulong)
            .wrapping_add(3 as c_ulong)
            .wrapping_add(7 as c_ulong)
            >> 3 as c_int) as ulg;
        if static_lenb <= opt_lenb {
            opt_lenb = static_lenb;
        }
    } else {
        static_lenb =
            (stored_len as c_ulong).wrapping_add(5 as c_ulong) as ulg;
        opt_lenb = static_lenb;
    }
    if (stored_len as c_ulong).wrapping_add(4 as c_ulong) <= opt_lenb
        && !buf.is_null()
    {
        _tr_stored_block(s, buf, stored_len, last);
    } else if (*s).strategy == Z_FIXED || static_lenb == opt_lenb {
        let mut len: c_int = 3 as c_int;
        if (*s).bi_valid > Buf_size - len {
            let mut val: c_int =
                ((1 as c_int) << 1 as c_int) + last;
            (*s).bi_buf = ((*s).bi_buf as c_int
                | (val as ush as c_int) << (*s).bi_valid)
                as ush;
            let fresh1 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh1 as isize) =
                ((*s).bi_buf as c_int & 0xff as c_int) as uch;
            let fresh2 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh2 as isize) =
                ((*s).bi_buf as c_int >> 8 as c_int) as uch;
            (*s).bi_buf = (val as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
            (*s).bi_valid += len - Buf_size;
        } else {
            (*s).bi_buf = ((*s).bi_buf as c_int
                | ((((1 as c_int) << 1 as c_int) + last) as ush
                    as c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len;
        }
        compress_block(
            s,
            &raw const static_ltree as *const ct_data,
            &raw const static_dtree as *const ct_data,
        );
    } else {
        let mut len_0: c_int = 3 as c_int;
        if (*s).bi_valid > Buf_size - len_0 {
            let mut val_0: c_int =
                ((2 as c_int) << 1 as c_int) + last;
            (*s).bi_buf = ((*s).bi_buf as c_int
                | (val_0 as ush as c_int) << (*s).bi_valid)
                as ush;
            let fresh3 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh3 as isize) =
                ((*s).bi_buf as c_int & 0xff as c_int) as uch;
            let fresh4 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh4 as isize) =
                ((*s).bi_buf as c_int >> 8 as c_int) as uch;
            (*s).bi_buf = (val_0 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
            (*s).bi_valid += len_0 - Buf_size;
        } else {
            (*s).bi_buf = ((*s).bi_buf as c_int
                | ((((2 as c_int) << 1 as c_int) + last) as ush
                    as c_int)
                    << (*s).bi_valid) as ush;
            (*s).bi_valid += len_0;
        }
        send_all_trees(
            s,
            (*s).l_desc.max_code + 1 as c_int,
            (*s).d_desc.max_code + 1 as c_int,
            max_blindex + 1 as c_int,
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
#[inline]
pub unsafe fn _tr_tally(
    mut s: *mut deflate_state,
    mut dist: c_uint,
    mut lc: c_uint,
) -> c_int {
    *(*s).d_buf.offset((*s).last_lit as isize) = dist as ush as ushf;
    let fresh0 = (*s).last_lit;
    (*s).last_lit = (*s).last_lit.wrapping_add(1);
    *(*s).l_buf.offset(fresh0 as isize) = lc as uch as uchf;
    if dist == 0 as c_uint {
        (*s).dyn_ltree[lc as usize].fc.freq = (*s).dyn_ltree[lc as usize].fc.freq.wrapping_add(1);
    } else {
        (*s).matches = (*s).matches.wrapping_add(1);
        dist = dist.wrapping_sub(1);
        (*s).dyn_ltree[(_length_code[lc as usize] as c_int
            + LITERALS
            + 1 as c_int) as usize]
            .fc
            .freq = (*s).dyn_ltree[(_length_code[lc as usize] as c_int
            + LITERALS
            + 1 as c_int) as usize]
            .fc
            .freq
            .wrapping_add(1);
        (*s).dyn_dtree[(if dist < 256 as c_uint {
            _dist_code[dist as usize] as c_int
        } else {
            _dist_code[(256 as c_uint).wrapping_add(dist >> 7 as c_int)
                as usize] as c_int
        }) as usize]
            .fc
            .freq = (*s).dyn_dtree[(if dist < 256 as c_uint {
            _dist_code[dist as usize] as c_int
        } else {
            _dist_code[(256 as c_uint).wrapping_add(dist >> 7 as c_int)
                as usize] as c_int
        }) as usize]
            .fc
            .freq
            .wrapping_add(1);
    }
    return ((*s).last_lit
        == ((*s).lit_bufsize as c_uint).wrapping_sub(1 as c_uint))
        as c_int;
}
unsafe fn compress_block(
    mut s: *mut deflate_state,
    mut ltree: *const ct_data,
    mut dtree: *const ct_data,
) {
    let mut dist: c_uint = 0;
    let mut lc: c_int = 0;
    let mut lx: c_uint = 0 as c_uint;
    let mut code: c_uint = 0;
    let mut extra: c_int = 0;
    let last_lit = (*s).last_lit;
    let l_buf = (*s).l_buf;
    let d_buf = (*s).d_buf;
    let pending_buf = (*s).pending_buf;
    if last_lit != 0 as c_uint {
        while lx < last_lit {
            dist = *d_buf.offset(lx as isize) as c_uint;
            lc = *l_buf.offset(lx as isize) as c_int;
            lx = lx.wrapping_add(1);
            if dist == 0 as c_uint {
                let ltree_lc = ltree.offset(lc as isize);
                let mut len: c_int = (*ltree_lc).dl.len as c_int;
                if (*s).bi_valid > Buf_size - len {
                    let mut val: c_int = (*ltree_lc).fc.code as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh9 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *pending_buf.offset(fresh9 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh10 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *pending_buf.offset(fresh10 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((*ltree_lc).fc.code as c_int) << (*s).bi_valid)
                        as ush;
                    (*s).bi_valid += len;
                }
            } else {
                code = _length_code[lc as usize] as c_uint;
                let lcode_idx = code.wrapping_add(256 as c_uint).wrapping_add(1 as c_uint);
                let ltree_code = ltree.offset(lcode_idx as isize);
                let mut len_0: c_int = (*ltree_code).dl.len as c_int;
                if (*s).bi_valid > Buf_size - len_0 {
                    let mut val_0: c_int = (*ltree_code).fc.code as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val_0 as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh11 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *pending_buf.offset(fresh11 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh12 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *pending_buf.offset(fresh12 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val_0 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_0 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((*ltree_code).fc.code as c_int) << (*s).bi_valid)
                        as ush;
                    (*s).bi_valid += len_0;
                }
                extra = extra_lbits[code as usize];
                if extra != 0 as c_int {
                    lc -= base_length[code as usize];
                    let mut len_1: c_int = extra;
                    if (*s).bi_valid > Buf_size - len_1 {
                        let mut val_1: c_int = lc;
                        (*s).bi_buf = ((*s).bi_buf as c_int
                            | (val_1 as ush as c_int) << (*s).bi_valid)
                            as ush;
                        let fresh13 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *pending_buf.offset(fresh13 as isize) =
                            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                        let fresh14 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *pending_buf.offset(fresh14 as isize) =
                            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                        (*s).bi_buf =
                            (val_1 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                        (*s).bi_valid += len_1 - Buf_size;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as c_int
                            | (lc as ush as c_int) << (*s).bi_valid)
                            as ush;
                        (*s).bi_valid += len_1;
                    }
                }
                dist = dist.wrapping_sub(1);
                code = (if dist < 256 as c_uint {
                    _dist_code[dist as usize] as c_int
                } else {
                    _dist_code[(256 as c_uint)
                        .wrapping_add(dist >> 7 as c_int)
                        as usize] as c_int
                }) as c_uint;
                let dtree_code = dtree.offset(code as isize);
                let mut len_2: c_int = (*dtree_code).dl.len as c_int;
                if (*s).bi_valid > Buf_size - len_2 {
                    let mut val_2: c_int = (*dtree_code).fc.code as c_int;
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | (val_2 as ush as c_int) << (*s).bi_valid)
                        as ush;
                    let fresh15 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *pending_buf.offset(fresh15 as isize) =
                        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                    let fresh16 = (*s).pending;
                    (*s).pending = (*s).pending.wrapping_add(1);
                    *pending_buf.offset(fresh16 as isize) =
                        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                    (*s).bi_buf =
                        (val_2 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                    (*s).bi_valid += len_2 - Buf_size;
                } else {
                    (*s).bi_buf = ((*s).bi_buf as c_int
                        | ((*dtree_code).fc.code as c_int) << (*s).bi_valid)
                        as ush;
                    (*s).bi_valid += len_2;
                }
                extra = extra_dbits[code as usize];
                if extra != 0 as c_int {
                    dist = dist.wrapping_sub(base_dist[code as usize] as c_uint);
                    let mut len_3: c_int = extra;
                    if (*s).bi_valid > Buf_size - len_3 {
                        let mut val_3: c_int = dist as c_int;
                        (*s).bi_buf = ((*s).bi_buf as c_int
                            | (val_3 as ush as c_int) << (*s).bi_valid)
                            as ush;
                        let fresh17 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *pending_buf.offset(fresh17 as isize) =
                            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
                        let fresh18 = (*s).pending;
                        (*s).pending = (*s).pending.wrapping_add(1);
                        *pending_buf.offset(fresh18 as isize) =
                            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
                        (*s).bi_buf =
                            (val_3 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
                        (*s).bi_valid += len_3 - Buf_size;
                    } else {
                        (*s).bi_buf = ((*s).bi_buf as c_int
                            | (dist as ush as c_int) << (*s).bi_valid)
                            as ush;
                        (*s).bi_valid += len_3;
                    }
                }
            }
        }
    }
    let end_code = ltree.offset(256 as isize);
let mut len_4: c_int = (*end_code).dl.len as c_int;
if (*s).bi_valid > Buf_size - len_4 {
    let mut val_4: c_int = (*end_code).fc.code as c_int;
    (*s).bi_buf = ((*s).bi_buf as c_int
        | (val_4 as ush as c_int) << (*s).bi_valid) as ush;
    let fresh19 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh19 as isize) =
        ((*s).bi_buf as c_int & 0xff as c_int) as uch;
    let fresh20 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh20 as isize) =
        ((*s).bi_buf as c_int >> 8 as c_int) as uch;
    (*s).bi_buf = (val_4 as ush as c_int >> Buf_size - (*s).bi_valid) as ush;
    (*s).bi_valid += len_4 - Buf_size;
} else {
    (*s).bi_buf = ((*s).bi_buf as c_int
        | ((*end_code).fc.code as c_int) << (*s).bi_valid) as ush;
    (*s).bi_valid += len_4;
};
}
unsafe fn detect_data_type(mut s: *mut deflate_state) -> c_int {
    let s_view: &deflate_state = unsafe { &*s };
    let mut black_mask: c_ulong = 0xf3ffc07f as c_ulong;
    let mut n: c_int = 0;
    n = 0 as c_int;
    while n <= 31 as c_int {
        if black_mask & 1 as c_ulong != 0
            && s_view.dyn_ltree[n as usize].fc.freq as c_int != 0 as c_int
        {
            return Z_BINARY;
        }
        n += 1;
        black_mask >>= 1 as c_int;
    }
    if s_view.dyn_ltree[9 as c_int as usize].fc.freq as c_int
        != 0 as c_int
        || s_view.dyn_ltree[10 as c_int as usize].fc.freq as c_int
            != 0 as c_int
        || s_view.dyn_ltree[13 as c_int as usize].fc.freq as c_int
            != 0 as c_int
    {
        return Z_TEXT;
    }
    n = 32 as c_int;
    while n < LITERALS {
        if s_view.dyn_ltree[n as usize].fc.freq as c_int != 0 as c_int {
            return Z_TEXT;
        }
        n += 1;
    }
    return Z_BINARY;
}
fn bi_reverse(
    mut code: c_uint,
    mut len: c_int,
) -> c_uint { {
    let mut res: c_uint = 0 as c_uint;
    loop {
        res |= code & 1 as c_uint;
        code >>= 1 as c_int;
        res <<= 1 as c_int;
        len -= 1;
        if !(len > 0 as c_int) {
            break;
        }
    }
    return res >> 1 as c_int;
} }
unsafe fn bi_flush(mut s: *mut deflate_state) {
    if (*s).bi_valid == 16 as c_int {
        let fresh55 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh55 as isize) =
            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
        let fresh56 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh56 as isize) =
            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
        (*s).bi_buf = 0 as ush;
        (*s).bi_valid = 0 as c_int;
    } else if (*s).bi_valid >= 8 as c_int {
        let fresh57 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh57 as isize) = (*s).bi_buf as Byte;
        (*s).bi_buf = ((*s).bi_buf as c_int >> 8 as c_int) as ush;
        (*s).bi_valid -= 8 as c_int;
    }
}
unsafe fn bi_windup(mut s: *mut deflate_state) {
    if (*s).bi_valid > 8 as c_int {
        let fresh5 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh5 as isize) =
            ((*s).bi_buf as c_int & 0xff as c_int) as uch;
        let fresh6 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh6 as isize) =
            ((*s).bi_buf as c_int >> 8 as c_int) as uch;
    } else if (*s).bi_valid > 0 as c_int {
        let fresh7 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh7 as isize) = (*s).bi_buf as Byte;
    }
    (*s).bi_buf = 0 as ush;
    (*s).bi_valid = 0 as c_int;
}
