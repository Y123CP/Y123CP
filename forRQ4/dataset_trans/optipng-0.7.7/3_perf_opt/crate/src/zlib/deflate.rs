use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::zlib::trees::static_tree_desc_s;
extern "C" {
    pub    static z_errmsg: [*mut c_char; 10];
    fn _tr_init(s: *mut deflate_state);
    fn _tr_flush_block(
        s: *mut deflate_state,
        buf: *mut charf,
        stored_len: ulg,
        last: c_int,
    );
    fn _tr_flush_bits(s: *mut deflate_state);
    fn _tr_align(s: *mut deflate_state);
    fn _tr_stored_block(
        s: *mut deflate_state,
        buf: *mut charf,
        stored_len: ulg,
        last: c_int,
    );
    static _length_code: [uch; 0];
    static _dist_code: [uch; 0];
}

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
pub type deflate_state = internal_state;
pub const block_done: block_state = 1;
pub type block_state = c_uint;
pub const finish_done: block_state = 3;
pub const finish_started: block_state = 2;
pub const need_more: block_state = 0;
pub type compress_func =
    Option<unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state>;
pub type config = config_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub good_length: ush,
    pub max_lazy: ush,
    pub nice_length: ush,
    pub max_chain: ush,
    pub func: compress_func,
}
pub const MAX_MEM_LEVEL: c_int = 9 as c_int;

pub const ZLIB_VERSION: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"1.2.11-optipng\0") };

pub const Z_PARTIAL_FLUSH: c_int = 1 as c_int;
pub const Z_FULL_FLUSH: c_int = 3 as c_int;

pub const Z_OK: c_int = 0 as c_int;
pub const Z_STREAM_END: c_int = 1 as c_int;
pub const Z_NEED_DICT: c_int = 2 as c_int;

pub const Z_DATA_ERROR: c_int = -(3 as c_int);
pub const Z_MEM_ERROR: c_int = -(4 as c_int);
pub const Z_BUF_ERROR: c_int = -(5 as c_int);
pub const Z_VERSION_ERROR: c_int = -(6 as c_int);

pub const DEF_MEM_LEVEL: c_int = 8 as c_int;
pub const MIN_MATCH: c_int = 3 as c_int;
pub const MAX_MATCH: c_int = 258 as c_int;
pub const PRESET_DICT: c_int = 0x20 as c_int;

pub const INIT_STATE: c_int = 42 as c_int;
pub const EXTRA_STATE: c_int = 69 as c_int;
pub const NAME_STATE: c_int = 73 as c_int;
pub const COMMENT_STATE: c_int = 91 as c_int;
pub const HCRC_STATE: c_int = 103 as c_int;
pub const BUSY_STATE: c_int = 113 as c_int;
pub const FINISH_STATE: c_int = 666 as c_int;
pub const MIN_LOOKAHEAD: c_int = MAX_MATCH + MIN_MATCH + 1 as c_int;
pub const WIN_INIT: c_int = MAX_MATCH;
#[no_mangle]
pub static mut deflate_copyright: [c_char; 69] = unsafe {
    ::core::mem::transmute::<[u8; 69], [c_char; 69]>(
        *b" deflate 1.2.11 Copyright 1995-2017 Jean-loup Gailly and Mark Adler \0",
    )
};
pub const NIL: c_int = 0 as c_int;
static mut configuration_table: [config; 10] = [
        config_s {
            good_length: 0 as ush,
            max_lazy: 0 as ush,
            nice_length: 0 as ush,
            max_chain: 0 as ush,
            func: Some(
                deflate_stored
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 4 as ush,
            max_lazy: 4 as ush,
            nice_length: 8 as ush,
            max_chain: 4 as ush,
            func: Some(
                deflate_fast
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 4 as ush,
            max_lazy: 5 as ush,
            nice_length: 16 as ush,
            max_chain: 8 as ush,
            func: Some(
                deflate_fast
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 4 as ush,
            max_lazy: 6 as ush,
            nice_length: 32 as ush,
            max_chain: 32 as ush,
            func: Some(
                deflate_fast
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 4 as ush,
            max_lazy: 4 as ush,
            nice_length: 16 as ush,
            max_chain: 16 as ush,
            func: Some(
                deflate_slow
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 8 as ush,
            max_lazy: 16 as ush,
            nice_length: 32 as ush,
            max_chain: 32 as ush,
            func: Some(
                deflate_slow
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 8 as ush,
            max_lazy: 16 as ush,
            nice_length: 128 as ush,
            max_chain: 128 as ush,
            func: Some(
                deflate_slow
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 8 as ush,
            max_lazy: 32 as ush,
            nice_length: 128 as ush,
            max_chain: 256 as ush,
            func: Some(
                deflate_slow
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 32 as ush,
            max_lazy: 128 as ush,
            nice_length: 258 as ush,
            max_chain: 1024 as ush,
            func: Some(
                deflate_slow
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
        config_s {
            good_length: 32 as ush,
            max_lazy: 258 as ush,
            nice_length: 258 as ush,
            max_chain: 4096 as ush,
            func: Some(
                deflate_slow
                    as unsafe extern "C" fn(*mut deflate_state, c_int) -> block_state,
            ),
        },
    ];
unsafe fn slide_hash(mut s: *mut deflate_state) {
    let mut n: c_uint = 0;
    let mut m: c_uint = 0;
    let mut p: *mut Posf = ::core::ptr::null_mut::<Posf>();
    let mut wsize: uInt = (*s).w_size;
    n = (*s).hash_size as c_uint;
    p = (*s).head.offset(n as isize) as *mut Posf;
    loop {
        p = p.offset(-1);
        m = *p as c_uint;
        *p = (if m >= wsize {
            m.wrapping_sub(wsize as c_uint)
        } else {
            NIL as c_uint
        }) as Pos as Posf;
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
    n = wsize as c_uint;
    p = (*s).prev.offset(n as isize) as *mut Posf;
    loop {
        p = p.offset(-1);
        m = *p as c_uint;
        *p = (if m >= wsize {
            m.wrapping_sub(wsize as c_uint)
        } else {
            NIL as c_uint
        }) as Pos as Posf;
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn deflateInit_(
    mut strm: z_streamp,
    mut level: c_int,
    mut version: *const c_char,
    mut stream_size: c_int,
) -> c_int {
    return deflateInit2_(
        strm,
        level,
        Z_DEFLATED,
        MAX_WBITS,
        DEF_MEM_LEVEL,
        Z_DEFAULT_STRATEGY,
        version,
        stream_size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn deflateInit2_(
    mut strm: z_streamp,
    mut level: c_int,
    mut method: c_int,
    mut windowBits: c_int,
    mut memLevel: c_int,
    mut strategy: c_int,
    mut version: *const c_char,
    mut stream_size: c_int,
) -> c_int {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    let mut wrap: c_int = 1 as c_int;
    static mut my_version: [c_char; 15] = ZLIB_VERSION;
    let mut overlay: *mut ushf = ::core::ptr::null_mut::<ushf>();
    if version.is_null()
        || *version.offset(0 as c_int as isize) as c_int
            != my_version[0 as c_int as usize] as c_int
        || stream_size as usize != ::core::mem::size_of::<z_stream>() as usize
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
    if level == Z_DEFAULT_COMPRESSION {
        level = 6 as c_int;
    }
    if windowBits < 0 as c_int {
        wrap = 0 as c_int;
        windowBits = -windowBits;
    }
    if memLevel < 1 as c_int
        || memLevel > MAX_MEM_LEVEL
        || method != Z_DEFLATED
        || windowBits < 8 as c_int
        || windowBits > 15 as c_int
        || level < 0 as c_int
        || level > 9 as c_int
        || strategy < 0 as c_int
        || strategy > Z_FIXED
        || windowBits == 8 as c_int && wrap != 1 as c_int
    {
        return Z_STREAM_ERROR;
    }
    if windowBits == 8 as c_int {
        windowBits = 9 as c_int;
    }
    s = Some((*strm).zalloc.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        1 as uInt,
        ::core::mem::size_of::<deflate_state>() as uInt,
    ) as *mut deflate_state;
    if s.is_null() {
        return Z_MEM_ERROR;
    }
    (*strm).state = s as *mut internal_state;
    (*s).strm = strm;
    (*s).status = INIT_STATE;
    (*s).wrap = wrap;
    (*s).gzhead = ::core::ptr::null_mut::<gz_header>();
    (*s).w_bits = windowBits as uInt;
    (*s).w_size = ((1 as c_int) << (*s).w_bits) as uInt;
    (*s).w_mask =
        ((*s).w_size as c_uint).wrapping_sub(1 as c_uint) as uInt;
    (*s).hash_bits =
        (memLevel as c_uint).wrapping_add(7 as c_uint) as uInt;
    (*s).hash_size = ((1 as c_int) << (*s).hash_bits) as uInt;
    (*s).hash_mask =
        ((*s).hash_size as c_uint).wrapping_sub(1 as c_uint) as uInt;
    (*s).hash_shift = ((*s).hash_bits as c_uint)
        .wrapping_add(MIN_MATCH as c_uint)
        .wrapping_sub(1 as c_uint)
        .wrapping_div(MIN_MATCH as c_uint) as uInt;
    (*s).window = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<Byte>() as usize) as uInt,
    ) as *mut Bytef;
    (*s).prev = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).w_size,
        ::core::mem::size_of::<Pos>() as uInt,
    ) as *mut Posf;
    (*s).head = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).hash_size,
        ::core::mem::size_of::<Pos>() as uInt,
    ) as *mut Posf;
    (*s).high_water = 0 as ulg;
    (*s).lit_bufsize = ((1 as c_int) << memLevel + 6 as c_int) as uInt;
    overlay = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).lit_bufsize,
        (::core::mem::size_of::<ush>() as usize).wrapping_add(2 as usize) as uInt,
    ) as *mut ushf;
    (*s).pending_buf = overlay as *mut uchf as *mut Bytef;
    (*s).pending_buf_size = ((*s).lit_bufsize as usize)
        .wrapping_mul((::core::mem::size_of::<ush>() as usize).wrapping_add(2 as usize))
        as ulg;
    if (*s).window.is_null()
        || (*s).prev.is_null()
        || (*s).head.is_null()
        || (*s).pending_buf.is_null()
    {
        (*s).status = FINISH_STATE;
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(4 as c_int)) as usize];
        deflateEnd(strm);
        return Z_MEM_ERROR;
    }
    (*s).d_buf = overlay.offset(
        ((*s).lit_bufsize as usize).wrapping_div(::core::mem::size_of::<ush>() as usize) as isize,
    );
    (*s).l_buf = (*s).pending_buf.offset(
        (1 as usize)
            .wrapping_add(::core::mem::size_of::<ush>() as usize)
            .wrapping_mul((*s).lit_bufsize as usize) as isize,
    ) as *mut uchf;
    (*s).level = level;
    (*s).strategy = strategy;
    (*s).method = method as Byte;
    return deflateReset(strm);
}
fn deflateStateCheck(mut strm: z_streamp) -> c_int { unsafe {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as c_int;
    }
    s = (*strm).state as *mut deflate_state;
    if s.is_null()
        || (*s).strm != strm
        || (*s).status != INIT_STATE
            && (*s).status != EXTRA_STATE
            && (*s).status != NAME_STATE
            && (*s).status != COMMENT_STATE
            && (*s).status != HCRC_STATE
            && (*s).status != BUSY_STATE
            && (*s).status != FINISH_STATE
    {
        return 1 as c_int;
    }
    return 0 as c_int;
} }
#[inline]
pub unsafe fn deflateSetDictionary(
    mut strm: z_streamp,
    mut dictionary: *const Bytef,
    mut dictLength: uInt,
) -> c_int {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    let mut str: uInt = 0;
    let mut n: uInt = 0;
    let mut wrap: c_int = 0;
    let mut avail: c_uint = 0;
    let mut next: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    if deflateStateCheck(strm) != 0 || dictionary.is_null() {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut deflate_state;
    wrap = (*s).wrap;
    if wrap == 2 as c_int
        || wrap == 1 as c_int && (*s).status != INIT_STATE
        || (*s).lookahead != 0
    {
        return Z_STREAM_ERROR;
    }
    if wrap == 1 as c_int {
        (*strm).adler = adler32((*strm).adler, dictionary, dictLength);
    }
    (*s).wrap = 0 as c_int;
    if dictLength >= (*s).w_size {
        if wrap == 0 as c_int {
            *(*s).head.offset(
                ((*s).hash_size as c_uint).wrapping_sub(1 as c_uint)
                    as isize,
            ) = NIL as Posf;
            memset(
                (*s).head as *mut Bytef as *mut c_void,
                0 as c_int,
                (((*s).hash_size as c_uint).wrapping_sub(1 as c_uint)
                    as size_t)
                    .wrapping_mul(::core::mem::size_of::<Posf>() as size_t),
            );
            (*s).strstart = 0 as uInt;
            (*s).block_start = 0 as c_long;
            (*s).insert = 0 as uInt;
        }
        dictionary = dictionary.offset(dictLength.wrapping_sub((*s).w_size) as isize);
        dictLength = (*s).w_size;
    }
    avail = (*strm).avail_in as c_uint;
    next = (*strm).next_in as *mut c_uchar;
    (*strm).avail_in = dictLength;
    (*strm).next_in = dictionary as *mut Bytef;
    fill_window(s);
    while (*s).lookahead >= MIN_MATCH as c_uint {
        str = (*s).strstart;
        n = ((*s).lookahead as c_uint)
            .wrapping_sub((MIN_MATCH - 1 as c_int) as c_uint)
            as uInt;
        loop {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset(
                    (str as c_uint)
                        .wrapping_add(3 as c_uint)
                        .wrapping_sub(1 as c_uint) as isize,
                ) as uInt)
                & (*s).hash_mask;
            *(*s).prev.offset((str & (*s).w_mask) as isize) =
                *(*s).head.offset((*s).ins_h as isize);
            *(*s).head.offset((*s).ins_h as isize) = str as Pos as Posf;
            str = str.wrapping_add(1);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        (*s).strstart = str;
        (*s).lookahead = (MIN_MATCH - 1 as c_int) as uInt;
        fill_window(s);
    }
    (*s).strstart = ((*s).strstart as c_uint)
        .wrapping_add((*s).lookahead as c_uint) as uInt as uInt;
    (*s).block_start = (*s).strstart as c_long;
    (*s).insert = (*s).lookahead;
    (*s).lookahead = 0 as uInt;
    (*s).prev_length = (MIN_MATCH - 1 as c_int) as uInt;
    (*s).match_length = (*s).prev_length;
    (*s).match_available = 0 as c_int;
    (*strm).next_in = next as *mut Bytef;
    (*strm).avail_in = avail as uInt;
    (*s).wrap = wrap;
    return Z_OK;
}
#[inline]
pub unsafe fn deflateGetDictionary(
    mut strm: z_streamp,
    mut dictionary: *mut Bytef,
    mut dictLength: *mut uInt,
) -> c_int {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    let mut len: uInt = 0;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut deflate_state;
    len = (*s).strstart.wrapping_add((*s).lookahead);
    if len > (*s).w_size {
        len = (*s).w_size;
    }
    if !dictionary.is_null() && len != 0 {
        memcpy(
            dictionary as *mut c_void,
            (*s).window
                .offset((*s).strstart as isize)
                .offset((*s).lookahead as isize)
                .offset(-(len as isize)) as *const c_void,
            len as size_t,
        );
    }
    if !dictLength.is_null() {
        *dictLength = len;
    }
    return Z_OK;
}
#[inline]
pub fn deflateResetKeep(mut strm: z_streamp) -> c_int { unsafe {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    (*strm).total_out = 0 as uLong;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = ::core::ptr::null_mut::<c_char>();
    (*strm).data_type = Z_UNKNOWN;
    s = (*strm).state as *mut deflate_state;
    (*s).pending = 0 as ulg;
    (*s).pending_out = (*s).pending_buf;
    if (*s).wrap < 0 as c_int {
        (*s).wrap = -(*s).wrap;
    }
    (*s).status = if (*s).wrap != 0 {
        INIT_STATE
    } else {
        BUSY_STATE
    };
    (*strm).adler = adler32(0 as uLong, ::core::ptr::null::<Bytef>(), 0 as uInt);
    (*s).last_flush = Z_NO_FLUSH;
    _tr_init(s);
    return Z_OK;
} }
#[no_mangle]
pub extern "C" fn deflateReset(mut strm: z_streamp) -> c_int { unsafe {
    let mut ret: c_int = 0;
    ret = deflateResetKeep(strm);
    if ret == Z_OK {
        lm_init((*strm).state as *mut deflate_state);
    }
    return ret;
} }
#[inline]
pub fn deflateSetHeader(
    mut strm: z_streamp,
    mut head: gz_headerp,
) -> c_int { unsafe {
    if deflateStateCheck(strm) != 0 || (*(*strm).state).wrap != 2 as c_int {
        return Z_STREAM_ERROR;
    }
    (*(*strm).state).gzhead = head;
    return Z_OK;
} }
#[inline]
pub unsafe fn deflatePending(
    mut strm: z_streamp,
    mut pending: *mut c_uint,
    mut bits: *mut c_int,
) -> c_int {
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    if !pending.is_null() {
        *pending = (*(*strm).state).pending as c_uint;
    }
    if !bits.is_null() {
        *bits = (*(*strm).state).bi_valid;
    }
    return Z_OK;
}
#[inline]
pub fn deflatePrime(
    mut strm: z_streamp,
    mut bits: c_int,
    mut value: c_int,
) -> c_int { unsafe {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    let mut put: c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut deflate_state;
    if ((*s).d_buf as *mut Bytef)
        < (*s)
            .pending_out
            .offset((Buf_size + 7 as c_int >> 3 as c_int) as isize)
    {
        return Z_BUF_ERROR;
    }
    loop {
        put = Buf_size - (*s).bi_valid;
        if put > bits {
            put = bits;
        }
        (*s).bi_buf = ((*s).bi_buf as c_int
            | ((value & ((1 as c_int) << put) - 1 as c_int)
                << (*s).bi_valid) as ush as c_int) as ush;
        (*s).bi_valid += put;
        _tr_flush_bits(s);
        value >>= put;
        bits -= put;
        if !(bits != 0) {
            break;
        }
    }
    return Z_OK;
} }
#[no_mangle]
pub extern "C" fn deflateParams(
    mut strm: z_streamp,
    mut level: c_int,
    mut strategy: c_int,
) -> c_int { unsafe {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    let mut func: compress_func = None;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut deflate_state;
    if level == Z_DEFAULT_COMPRESSION {
        level = 6 as c_int;
    }
    if level < 0 as c_int
        || level > 9 as c_int
        || strategy < 0 as c_int
        || strategy > Z_FIXED
    {
        return Z_STREAM_ERROR;
    }
    func = configuration_table[(*s).level as usize].func;
    if (strategy != (*s).strategy || func != configuration_table[level as usize].func)
        && (*s).high_water != 0
    {
        let mut err: c_int = deflate(strm, Z_BLOCK);
        if err == Z_STREAM_ERROR {
            return err;
        }
        if (*strm).avail_out == 0 as c_uint {
            return Z_BUF_ERROR;
        }
    }
    if (*s).level != level {
        if (*s).level == 0 as c_int && (*s).matches != 0 as c_uint {
            if (*s).matches == 1 as c_uint {
                slide_hash(s);
            } else {
                *(*s).head.offset(
                    ((*s).hash_size as c_uint).wrapping_sub(1 as c_uint)
                        as isize,
                ) = NIL as Posf;
            }
            memset(
                (*s).head as *mut Bytef as *mut c_void,
                0 as c_int,
                (((*s).hash_size as c_uint).wrapping_sub(1 as c_uint)
                    as size_t)
                    .wrapping_mul(::core::mem::size_of::<Posf>() as size_t),
            );
            (*s).matches = 0 as uInt;
        }
        (*s).level = level;
        (*s).max_lazy_match = configuration_table[level as usize].max_lazy as uInt;
        (*s).good_match = configuration_table[level as usize].good_length as uInt;
        (*s).nice_match = configuration_table[level as usize].nice_length as c_int;
        (*s).max_chain_length = configuration_table[level as usize].max_chain as uInt;
    }
    (*s).strategy = strategy;
    return Z_OK;
} }
#[inline]
pub fn deflateTune(
    mut strm: z_streamp,
    mut good_length: c_int,
    mut max_lazy: c_int,
    mut nice_length: c_int,
    mut max_chain: c_int,
) -> c_int { unsafe {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut deflate_state;
    (*s).good_match = good_length as uInt;
    (*s).max_lazy_match = max_lazy as uInt;
    (*s).nice_match = nice_length;
    (*s).max_chain_length = max_chain as uInt;
    return Z_OK;
} }
#[inline]
pub fn deflateBound(mut strm: z_streamp, mut sourceLen: uLong) -> uLong { unsafe {
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    let mut complen: uLong = 0;
    let mut wraplen: uLong = 0;
    complen = (sourceLen as c_ulong)
        .wrapping_add(
            (sourceLen as c_ulong).wrapping_add(7 as c_ulong)
                >> 3 as c_int,
        )
        .wrapping_add(
            (sourceLen as c_ulong).wrapping_add(63 as c_ulong)
                >> 6 as c_int,
        )
        .wrapping_add(5 as c_ulong) as uLong;
    if deflateStateCheck(strm) != 0 {
        return complen.wrapping_add(6 as uLong);
    }
    s = (*strm).state as *mut deflate_state;
    match (*s).wrap {
        0 => {
            wraplen = 0 as uLong;
        }
        1 => {
            wraplen = (6 as c_int
                + (if (*s).strstart != 0 {
                    4 as c_int
                } else {
                    0 as c_int
                })) as uLong;
        }
        _ => {
            wraplen = 6 as uLong;
        }
    }
    if (*s).w_bits != 15 as c_uint
        || (*s).hash_bits
            != (8 as c_int + 7 as c_int) as c_uint
    {
        return complen.wrapping_add(wraplen);
    }
    return sourceLen
        .wrapping_add(sourceLen >> 12 as c_int)
        .wrapping_add(sourceLen >> 14 as c_int)
        .wrapping_add(sourceLen >> 25 as c_int)
        .wrapping_add(13 as uLong)
        .wrapping_sub(6 as uLong)
        .wrapping_add(wraplen);
} }
unsafe fn putShortMSB(mut s: *mut deflate_state, mut b: uInt) {
    let s_view: &mut deflate_state = unsafe { &mut *s };
    let fresh0 = s_view.pending;
    s_view.pending = s_view.pending.wrapping_add(1);
    *s_view.pending_buf.offset(fresh0 as isize) = (b >> 8 as c_int) as Byte;
    let fresh1 = s_view.pending;
    s_view.pending = s_view.pending.wrapping_add(1);
    *s_view.pending_buf.offset(fresh1 as isize) =
        (b as c_uint & 0xff as c_uint) as Byte;
}
fn flush_pending(mut strm: z_streamp) { unsafe {
    let mut len: c_uint = 0;
    let mut s: *mut deflate_state = (*strm).state as *mut deflate_state;
    _tr_flush_bits(s);
    len = (*s).pending as c_uint;
    if len > (*strm).avail_out {
        len = (*strm).avail_out as c_uint;
    }
    if len == 0 as c_uint {
        return;
    }
    memcpy(
        (*strm).next_out as *mut c_void,
        (*s).pending_out as *const c_void,
        len as size_t,
    );
    (*strm).next_out = (*strm).next_out.offset(len as isize);
    (*s).pending_out = (*s).pending_out.offset(len as isize);
    (*strm).total_out = ((*strm).total_out as c_ulong)
        .wrapping_add(len as c_ulong) as uLong as uLong;
    (*strm).avail_out =
        ((*strm).avail_out as c_uint).wrapping_sub(len) as uInt as uInt;
    (*s).pending = ((*s).pending as c_ulong).wrapping_sub(len as c_ulong)
        as ulg as ulg;
    if (*s).pending == 0 as c_ulong {
        (*s).pending_out = (*s).pending_buf;
    }
} }
#[no_mangle]
pub extern "C" fn deflate(
    mut strm: z_streamp,
    mut flush: c_int,
) -> c_int { unsafe {
    let mut old_flush: c_int = 0;
    let mut s: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    if deflateStateCheck(strm) != 0 || flush > Z_BLOCK || flush < 0 as c_int {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state as *mut deflate_state;
    if (*strm).next_out.is_null()
        || (*strm).avail_in != 0 as c_uint && (*strm).next_in.is_null()
        || (*s).status == FINISH_STATE && flush != Z_FINISH
    {
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(2 as c_int)) as usize];
        return -(2 as c_int);
    }
    if (*strm).avail_out == 0 as c_uint {
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(5 as c_int)) as usize];
        return -(5 as c_int);
    }
    old_flush = (*s).last_flush;
    (*s).last_flush = flush;
    if (*s).pending != 0 as c_ulong {
        flush_pending(strm);
        if (*strm).avail_out == 0 as c_uint {
            (*s).last_flush = -(1 as c_int);
            return Z_OK;
        }
    } else if (*strm).avail_in == 0 as c_uint
        && flush * 2 as c_int
            - (if flush > 4 as c_int {
                9 as c_int
            } else {
                0 as c_int
            })
            <= old_flush * 2 as c_int
                - (if old_flush > 4 as c_int {
                    9 as c_int
                } else {
                    0 as c_int
                })
        && flush != Z_FINISH
    {
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(5 as c_int)) as usize];
        return -(5 as c_int);
    }
    if (*s).status == FINISH_STATE && (*strm).avail_in != 0 as c_uint {
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(5 as c_int)) as usize];
        return -(5 as c_int);
    }
    if (*s).status == INIT_STATE {
        let mut header: uInt = (Z_DEFLATED as uInt)
            .wrapping_add((*s).w_bits.wrapping_sub(8 as uInt) << 4 as c_int)
            << 8 as c_int;
        let mut level_flags: uInt = 0;
        if (*s).strategy >= Z_HUFFMAN_ONLY || (*s).level < 2 as c_int {
            level_flags = 0 as uInt;
        } else if (*s).level < 6 as c_int {
            level_flags = 1 as uInt;
        } else if (*s).level == 6 as c_int {
            level_flags = 2 as uInt;
        } else {
            level_flags = 3 as uInt;
        }
        header |= (level_flags << 6 as c_int) as c_uint;
        if (*s).strstart != 0 as c_uint {
            header |= PRESET_DICT as c_uint;
        }
        header =
            (header as c_uint).wrapping_add((31 as c_uint).wrapping_sub(
                (header as c_uint).wrapping_rem(31 as c_uint),
            )) as uInt as uInt;
        putShortMSB(s, header);
        if (*s).strstart != 0 as c_uint {
            putShortMSB(s, ((*strm).adler >> 16 as c_int) as uInt);
            putShortMSB(
                s,
                ((*strm).adler as c_ulong & 0xffff as c_ulong) as uInt,
            );
        }
        (*strm).adler = adler32(0 as uLong, ::core::ptr::null::<Bytef>(), 0 as uInt);
        (*s).status = BUSY_STATE;
        flush_pending(strm);
        if (*s).pending != 0 as c_ulong {
            (*s).last_flush = -(1 as c_int);
            return Z_OK;
        }
    }
    if (*strm).avail_in != 0 as c_uint
        || (*s).lookahead != 0 as c_uint
        || flush != Z_NO_FLUSH && (*s).status != FINISH_STATE
    {
        let mut bstate: block_state = need_more;
        bstate = (if (*s).level == 0 as c_int {
            deflate_stored(s, flush) as c_uint
        } else if (*s).strategy == Z_HUFFMAN_ONLY {
            deflate_huff(s, flush) as c_uint
        } else if (*s).strategy == Z_RLE {
            deflate_rle(s, flush) as c_uint
        } else {
            Some(
                (*(&raw const configuration_table as *const config).offset((*s).level as isize))
                    .func
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(s, flush) as c_uint
        }) as block_state;
        if bstate as c_uint
            == finish_started as c_int as c_uint
            || bstate as c_uint
                == finish_done as c_int as c_uint
        {
            (*s).status = FINISH_STATE;
        }
        if bstate as c_uint == need_more as c_int as c_uint
            || bstate as c_uint
                == finish_started as c_int as c_uint
        {
            if (*strm).avail_out == 0 as c_uint {
                (*s).last_flush = -(1 as c_int);
            }
            return Z_OK;
        }
        if bstate as c_uint == block_done as c_int as c_uint
        {
            if flush == Z_PARTIAL_FLUSH {
                _tr_align(s);
            } else if flush != Z_BLOCK {
                _tr_stored_block(
                    s,
                    ::core::ptr::null_mut::<charf>(),
                    0 as ulg,
                    0 as c_int,
                );
                if flush == Z_FULL_FLUSH {
                    *(*s).head.offset(
                        ((*s).hash_size as c_uint)
                            .wrapping_sub(1 as c_uint)
                            as isize,
                    ) = NIL as Posf;
                    memset(
                        (*s).head as *mut Bytef as *mut c_void,
                        0 as c_int,
                        (((*s).hash_size as c_uint)
                            .wrapping_sub(1 as c_uint)
                            as size_t)
                            .wrapping_mul(::core::mem::size_of::<Posf>() as size_t),
                    );
                    if (*s).lookahead == 0 as c_uint {
                        (*s).strstart = 0 as uInt;
                        (*s).block_start = 0 as c_long;
                        (*s).insert = 0 as uInt;
                    }
                }
            }
            flush_pending(strm);
            if (*strm).avail_out == 0 as c_uint {
                (*s).last_flush = -(1 as c_int);
                return Z_OK;
            }
        }
    }
    if flush != Z_FINISH {
        return Z_OK;
    }
    if (*s).wrap <= 0 as c_int {
        return Z_STREAM_END;
    }
    putShortMSB(s, ((*strm).adler >> 16 as c_int) as uInt);
    putShortMSB(
        s,
        ((*strm).adler as c_ulong & 0xffff as c_ulong) as uInt,
    );
    flush_pending(strm);
    if (*s).wrap > 0 as c_int {
        (*s).wrap = -(*s).wrap;
    }
    return if (*s).pending != 0 as c_ulong {
        Z_OK
    } else {
        Z_STREAM_END
    };
} }
#[no_mangle]
pub extern "C" fn deflateEnd(mut strm: z_streamp) -> c_int { unsafe {
    let mut status: c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    status = (*(*strm).state).status;
    if !(*(*strm).state).pending_buf.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).pending_buf as voidpf,
        );
    }
    if !(*(*strm).state).head.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).head as voidpf,
        );
    }
    if !(*(*strm).state).prev.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).prev as voidpf,
        );
    }
    if !(*(*strm).state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).window as voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as voidpf,
    );
    (*strm).state = ::core::ptr::null_mut::<internal_state>();
    return if status == BUSY_STATE {
        Z_DATA_ERROR
    } else {
        Z_OK
    };
} }
#[inline]
pub fn deflateCopy(
    mut dest: z_streamp,
    mut source: z_streamp,
) -> c_int { unsafe {
    let mut ds: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    let mut ss: *mut deflate_state = ::core::ptr::null_mut::<deflate_state>();
    let mut overlay: *mut ushf = ::core::ptr::null_mut::<ushf>();
    if deflateStateCheck(source) != 0 || dest.is_null() {
        return Z_STREAM_ERROR;
    }
    ss = (*source).state as *mut deflate_state;
    memcpy(
        dest as *mut c_void,
        source as voidpf as *const c_void,
        ::core::mem::size_of::<z_stream>() as size_t,
    );
    ds = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        1 as uInt,
        ::core::mem::size_of::<deflate_state>() as uInt,
    ) as *mut deflate_state;
    if ds.is_null() {
        return Z_MEM_ERROR;
    }
    (*dest).state = ds as *mut internal_state;
    memcpy(
        ds as *mut c_void,
        ss as voidpf as *const c_void,
        ::core::mem::size_of::<deflate_state>() as size_t,
    );
    (*ds).strm = dest;
    (*ds).window = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).w_size,
        (2 as usize).wrapping_mul(::core::mem::size_of::<Byte>() as usize) as uInt,
    ) as *mut Bytef;
    (*ds).prev = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).w_size,
        ::core::mem::size_of::<Pos>() as uInt,
    ) as *mut Posf;
    (*ds).head = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).hash_size,
        ::core::mem::size_of::<Pos>() as uInt,
    ) as *mut Posf;
    overlay = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).lit_bufsize,
        (::core::mem::size_of::<ush>() as usize).wrapping_add(2 as usize) as uInt,
    ) as *mut ushf;
    (*ds).pending_buf = overlay as *mut uchf as *mut Bytef;
    if (*ds).window.is_null()
        || (*ds).prev.is_null()
        || (*ds).head.is_null()
        || (*ds).pending_buf.is_null()
    {
        deflateEnd(dest);
        return Z_MEM_ERROR;
    }
    memcpy(
        (*ds).window as *mut c_void,
        (*ss).window as *const c_void,
        (((*ds).w_size as c_uint).wrapping_mul(2 as c_uint) as size_t)
            .wrapping_mul(::core::mem::size_of::<Byte>() as size_t),
    );
    memcpy(
        (*ds).prev as *mut c_void,
        (*ss).prev as voidpf as *const c_void,
        ((*ds).w_size as size_t).wrapping_mul(::core::mem::size_of::<Pos>() as size_t),
    );
    memcpy(
        (*ds).head as *mut c_void,
        (*ss).head as voidpf as *const c_void,
        ((*ds).hash_size as size_t).wrapping_mul(::core::mem::size_of::<Pos>() as size_t),
    );
    memcpy(
        (*ds).pending_buf as *mut c_void,
        (*ss).pending_buf as *const c_void,
        (*ds).pending_buf_size as uInt as size_t,
    );
    (*ds).pending_out = (*ds)
        .pending_buf
        .offset((*ss).pending_out.offset_from((*ss).pending_buf) as c_long as isize);
    (*ds).d_buf = overlay.offset(
        ((*ds).lit_bufsize as usize).wrapping_div(::core::mem::size_of::<ush>() as usize) as isize,
    );
    (*ds).l_buf = (*ds).pending_buf.offset(
        (1 as usize)
            .wrapping_add(::core::mem::size_of::<ush>() as usize)
            .wrapping_mul((*ds).lit_bufsize as usize) as isize,
    ) as *mut uchf;
    (*ds).l_desc.dyn_tree = &raw mut (*ds).dyn_ltree as *mut ct_data_s as *mut ct_data;
    (*ds).d_desc.dyn_tree = &raw mut (*ds).dyn_dtree as *mut ct_data_s as *mut ct_data;
    (*ds).bl_desc.dyn_tree = &raw mut (*ds).bl_tree as *mut ct_data_s as *mut ct_data;
    return Z_OK;
} }
unsafe fn read_buf(
    mut strm: z_streamp,
    mut buf: *mut Bytef,
    mut size: c_uint,
) -> c_uint {
    let mut len: c_uint = (*strm).avail_in as c_uint;
    if len > size {
        len = size;
    }
    if len == 0 as c_uint {
        return 0 as c_uint;
    }
    (*strm).avail_in = ((*strm).avail_in as c_uint).wrapping_sub(len) as uInt as uInt;
    memcpy(
        buf as *mut c_void,
        (*strm).next_in as *const c_void,
        len as size_t,
    );
    if (*(*strm).state).wrap == 1 as c_int {
        (*strm).adler = adler32((*strm).adler, buf, len as uInt);
    }
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in = ((*strm).total_in as c_ulong)
        .wrapping_add(len as c_ulong) as uLong as uLong;
    return len;
}
unsafe fn lm_init(mut s: *mut deflate_state) {
    let s_view: &mut deflate_state = unsafe { &mut *s };
    s_view.window_size = (2 as c_long as c_ulong)
        .wrapping_mul(s_view.w_size as c_ulong) as ulg;
    *s_view.head.offset(
        (s_view.hash_size as c_uint).wrapping_sub(1 as c_uint) as isize,
    ) = NIL as Posf;
    memset(
        s_view.head as *mut Bytef as *mut c_void,
        0 as c_int,
        ((s_view.hash_size as c_uint).wrapping_sub(1 as c_uint) as size_t)
            .wrapping_mul(::core::mem::size_of::<Posf>() as size_t),
    );
    s_view.max_lazy_match = configuration_table[s_view.level as usize].max_lazy as uInt;
    s_view.good_match = configuration_table[s_view.level as usize].good_length as uInt;
    s_view.nice_match = configuration_table[s_view.level as usize].nice_length as c_int;
    s_view.max_chain_length = configuration_table[s_view.level as usize].max_chain as uInt;
    s_view.strstart = 0 as uInt;
    s_view.block_start = 0 as c_long;
    s_view.lookahead = 0 as uInt;
    s_view.insert = 0 as uInt;
    s_view.prev_length = (MIN_MATCH - 1 as c_int) as uInt;
    s_view.match_length = s_view.prev_length;
    s_view.match_available = 0 as c_int;
    s_view.ins_h = 0 as uInt;
}
unsafe fn longest_match(mut s: *mut deflate_state, mut cur_match: IPos) -> uInt {
    let s_view: &mut deflate_state = unsafe { &mut *s };
    let mut chain_length: c_uint = s_view.max_chain_length as c_uint;
    let mut scan: *mut Bytef = s_view.window.offset(s_view.strstart as isize);
    let mut match_0: *mut Bytef = ::core::ptr::null_mut::<Bytef>();
    let mut len: c_int = 0;
    let mut best_len: c_int = s_view.prev_length as c_int;
    let mut nice_match: c_int = s_view.nice_match;
    let mut limit: IPos = if s_view.strstart
        > (s_view.w_size as c_uint).wrapping_sub(MIN_LOOKAHEAD as c_uint)
    {
        (s_view.strstart as IPos).wrapping_sub(
            (s_view.w_size as c_uint).wrapping_sub(MIN_LOOKAHEAD as c_uint),
        )
    } else {
        NIL as IPos
    };
    let mut prev: *mut Posf = s_view.prev;
    let mut wmask: uInt = s_view.w_mask;
    let mut strend: *mut Bytef = s_view
        .window
        .offset(s_view.strstart as isize)
        .offset(MAX_MATCH as isize);
    let mut scan_end1: Byte = *scan.offset((best_len - 1 as c_int) as isize) as Byte;
    let mut scan_end: Byte = *scan.offset(best_len as isize) as Byte;
    if s_view.prev_length >= s_view.good_match {
        chain_length >>= 2 as c_int;
    }
    if nice_match as uInt > s_view.lookahead {
        nice_match = s_view.lookahead as c_int;
    }
    loop {
        match_0 = s_view.window.offset(cur_match as isize);
        if !(*match_0.offset(best_len as isize) as c_int
            != scan_end as c_int
            || *match_0.offset((best_len - 1 as c_int) as isize) as c_int
                != scan_end1 as c_int
            || *match_0 as c_int != *scan as c_int
            || {
                match_0 = match_0.offset(1);
                *match_0 as c_int
                    != *scan.offset(1 as c_int as isize) as c_int
            })
        {
            scan = scan.offset(2 as c_int as isize);
            match_0 = match_0.offset(1);
            loop {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                if !(*scan as c_int == *match_0 as c_int
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as c_int == *match_0 as c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as c_int == *match_0 as c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as c_int == *match_0 as c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as c_int == *match_0 as c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as c_int == *match_0 as c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as c_int == *match_0 as c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        *scan as c_int == *match_0 as c_int
                    }
                    && scan < strend)
                {
                    break;
                }
            }
            len = MAX_MATCH - strend.offset_from(scan) as c_long as c_int;
            scan = strend.offset(-(MAX_MATCH as isize));
            if len > best_len {
                s_view.match_start = cur_match as uInt;
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = *scan.offset((best_len - 1 as c_int) as isize) as Byte;
                scan_end = *scan.offset(best_len as isize) as Byte;
            }
        }
        cur_match = *prev.offset((cur_match as uInt & wmask) as isize) as IPos;
        if !(cur_match > limit && {
            chain_length = chain_length.wrapping_sub(1);
            chain_length != 0 as c_uint
        }) {
            break;
        }
    }
    if best_len as uInt <= s_view.lookahead {
        return best_len as uInt;
    }
    return s_view.lookahead;
}
unsafe fn fill_window(mut s: *mut deflate_state) {
    let mut n: c_uint = 0;
    let mut more: c_uint = 0;
    let mut wsize: uInt = (*s).w_size;
    loop {
        more = (*s)
            .window_size
            .wrapping_sub((*s).lookahead as ulg)
            .wrapping_sub((*s).strstart as ulg) as c_uint;
        if ::core::mem::size_of::<c_int>() as usize <= 2 as usize {
            if more == 0 as c_uint
                && (*s).strstart == 0 as c_uint
                && (*s).lookahead == 0 as c_uint
            {
                more = wsize as c_uint;
            } else if more == -(1 as c_int) as c_uint {
                more = more.wrapping_sub(1);
            }
        }
        if (*s).strstart
            >= (wsize as c_uint).wrapping_add(
                ((*s).w_size as c_uint)
                    .wrapping_sub(MIN_LOOKAHEAD as c_uint),
            )
        {
            memcpy(
                (*s).window as *mut c_void,
                (*s).window.offset(wsize as isize) as *const c_void,
                wsize.wrapping_sub(more) as size_t,
            );
            (*s).match_start = ((*s).match_start as c_uint)
                .wrapping_sub(wsize as c_uint) as uInt
                as uInt;
            (*s).strstart = ((*s).strstart as c_uint)
                .wrapping_sub(wsize as c_uint) as uInt
                as uInt;
            (*s).block_start -= wsize as c_long;
            slide_hash(s);
            more = more.wrapping_add(wsize as c_uint);
        }
        if (*(*s).strm).avail_in == 0 as c_uint {
            break;
        }
        n = read_buf(
            (*s).strm,
            (*s).window
                .offset((*s).strstart as isize)
                .offset((*s).lookahead as isize),
            more,
        );
        (*s).lookahead = ((*s).lookahead as c_uint).wrapping_add(n) as uInt as uInt;
        if (*s).lookahead.wrapping_add((*s).insert) >= MIN_MATCH as c_uint {
            let mut str: uInt = (*s).strstart.wrapping_sub((*s).insert);
            (*s).ins_h = *(*s).window.offset(str as isize) as uInt;
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset(
                    (str as c_uint).wrapping_add(1 as c_uint) as isize,
                ) as uInt)
                & (*s).hash_mask;
            while (*s).insert != 0 {
                (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                    ^ *(*s).window.offset(
                        (str as c_uint)
                            .wrapping_add(3 as c_uint)
                            .wrapping_sub(1 as c_uint)
                            as isize,
                    ) as uInt)
                    & (*s).hash_mask;
                *(*s).prev.offset((str & (*s).w_mask) as isize) =
                    *(*s).head.offset((*s).ins_h as isize);
                *(*s).head.offset((*s).ins_h as isize) = str as Pos as Posf;
                str = str.wrapping_add(1);
                (*s).insert = (*s).insert.wrapping_sub(1);
                if (*s).lookahead.wrapping_add((*s).insert) < MIN_MATCH as c_uint {
                    break;
                }
            }
        }
        if !((*s).lookahead < MIN_LOOKAHEAD as c_uint
            && (*(*s).strm).avail_in != 0 as c_uint)
        {
            break;
        }
    }
    if (*s).high_water < (*s).window_size {
        let mut curr: ulg = ((*s).strstart as ulg).wrapping_add((*s).lookahead as ulg);
        let mut init: ulg = 0;
        if (*s).high_water < curr {
            init = (*s).window_size.wrapping_sub(curr);
            if init > WIN_INIT as c_ulong {
                init = WIN_INIT as ulg;
            }
            memset(
                (*s).window.offset(curr as isize) as *mut c_void,
                0 as c_int,
                init as c_uint as size_t,
            );
            (*s).high_water = curr.wrapping_add(init);
        } else if (*s).high_water < curr.wrapping_add(WIN_INIT as c_ulong) {
            init = curr
                .wrapping_add(WIN_INIT as ulg)
                .wrapping_sub((*s).high_water);
            if init > (*s).window_size.wrapping_sub((*s).high_water) {
                init = (*s).window_size.wrapping_sub((*s).high_water);
            }
            memset(
                (*s).window.offset((*s).high_water as isize) as *mut c_void,
                0 as c_int,
                init as c_uint as size_t,
            );
            (*s).high_water = ((*s).high_water as c_ulong)
                .wrapping_add(init as c_ulong) as ulg
                as ulg;
        }
    }
}
pub const MAX_STORED: c_int = 65535 as c_int;
unsafe extern "C" fn deflate_stored(
    mut s: *mut deflate_state,
    mut flush: c_int,
) -> block_state {
    let mut min_block: c_uint = (if ((*s).pending_buf_size as c_ulong)
        .wrapping_sub(5 as c_ulong)
        > (*s).w_size as c_ulong
    {
        (*s).w_size as c_ulong
    } else {
        ((*s).pending_buf_size as c_ulong).wrapping_sub(5 as c_ulong)
    }) as c_uint;
    let mut len: c_uint = 0;
    let mut left: c_uint = 0;
    let mut have: c_uint = 0;
    let mut last: c_uint = 0 as c_uint;
    let mut used: c_uint = (*(*s).strm).avail_in as c_uint;
    loop {
        len = MAX_STORED as c_uint;
        have = ((*s).bi_valid + 42 as c_int >> 3 as c_int)
            as c_uint;
        if (*(*s).strm).avail_out < have {
            break;
        }
        have = ((*(*s).strm).avail_out as c_uint).wrapping_sub(have);
        left = ((*s).strstart as c_long - (*s).block_start) as c_uint;
        if len as c_ulong
            > (left as c_ulong)
                .wrapping_add((*(*s).strm).avail_in as c_ulong)
        {
            len = (left as uInt).wrapping_add((*(*s).strm).avail_in) as c_uint;
        }
        if len > have {
            len = have;
        }
        if len < min_block
            && (len == 0 as c_uint && flush != Z_FINISH
                || flush == Z_NO_FLUSH
                || len != (left as uInt).wrapping_add((*(*s).strm).avail_in))
        {
            break;
        }
        last = (if flush == Z_FINISH && len == (left as uInt).wrapping_add((*(*s).strm).avail_in) {
            1 as c_int
        } else {
            0 as c_int
        }) as c_uint;
        _tr_stored_block(
            s,
            ::core::ptr::null_mut::<charf>(),
            0 as ulg,
            last as c_int,
        );
        *(*s).pending_buf.offset(
            ((*s).pending as c_ulong).wrapping_sub(4 as c_ulong) as isize,
        ) = len as Bytef;
        *(*s).pending_buf.offset(
            ((*s).pending as c_ulong).wrapping_sub(3 as c_ulong) as isize,
        ) = (len >> 8 as c_int) as Bytef;
        *(*s).pending_buf.offset(
            ((*s).pending as c_ulong).wrapping_sub(2 as c_ulong) as isize,
        ) = !len as Bytef;
        *(*s).pending_buf.offset(
            ((*s).pending as c_ulong).wrapping_sub(1 as c_ulong) as isize,
        ) = (!len >> 8 as c_int) as Bytef;
        flush_pending((*s).strm);
        if left != 0 {
            if left > len {
                left = len;
            }
            memcpy(
                (*(*s).strm).next_out as *mut c_void,
                (*s).window.offset((*s).block_start as isize) as *const c_void,
                left as size_t,
            );
            (*(*s).strm).next_out = (*(*s).strm).next_out.offset(left as isize);
            (*(*s).strm).avail_out =
                ((*(*s).strm).avail_out as c_uint).wrapping_sub(left) as uInt as uInt;
            (*(*s).strm).total_out = ((*(*s).strm).total_out as c_ulong)
                .wrapping_add(left as c_ulong)
                as uLong as uLong;
            (*s).block_start += left as c_long;
            len = len.wrapping_sub(left);
        }
        if len != 0 {
            read_buf((*s).strm, (*(*s).strm).next_out, len);
            (*(*s).strm).next_out = (*(*s).strm).next_out.offset(len as isize);
            (*(*s).strm).avail_out =
                ((*(*s).strm).avail_out as c_uint).wrapping_sub(len) as uInt as uInt;
            (*(*s).strm).total_out = ((*(*s).strm).total_out as c_ulong)
                .wrapping_add(len as c_ulong)
                as uLong as uLong;
        }
        if !(last == 0 as c_uint) {
            break;
        }
    }
    used = used.wrapping_sub((*(*s).strm).avail_in as c_uint);
    if used != 0 {
        if used >= (*s).w_size {
            (*s).matches = 2 as uInt;
            memcpy(
                (*s).window as *mut c_void,
                (*(*s).strm).next_in.offset(-((*s).w_size as isize)) as *const c_void,
                (*s).w_size as size_t,
            );
            (*s).strstart = (*s).w_size;
        } else {
            if ((*s).window_size as c_ulong)
                .wrapping_sub((*s).strstart as c_ulong)
                <= used as c_ulong
            {
                (*s).strstart = ((*s).strstart as c_uint)
                    .wrapping_sub((*s).w_size as c_uint)
                    as uInt as uInt;
                memcpy(
                    (*s).window as *mut c_void,
                    (*s).window.offset((*s).w_size as isize) as *const c_void,
                    (*s).strstart as size_t,
                );
                if (*s).matches < 2 as c_uint {
                    (*s).matches = (*s).matches.wrapping_add(1);
                }
            }
            memcpy(
                (*s).window.offset((*s).strstart as isize) as *mut c_void,
                (*(*s).strm).next_in.offset(-(used as isize)) as *const c_void,
                used as size_t,
            );
            (*s).strstart =
                ((*s).strstart as c_uint).wrapping_add(used) as uInt as uInt;
        }
        (*s).block_start = (*s).strstart as c_long;
        (*s).insert = ((*s).insert as c_uint).wrapping_add(
            if used > (*s).w_size.wrapping_sub((*s).insert) {
                ((*s).w_size as c_uint)
                    .wrapping_sub((*s).insert as c_uint)
            } else {
                used
            },
        ) as uInt as uInt;
    }
    if (*s).high_water < (*s).strstart as c_ulong {
        (*s).high_water = (*s).strstart as ulg;
    }
    if last != 0 {
        return finish_done;
    }
    if flush != Z_NO_FLUSH
        && flush != Z_FINISH
        && (*(*s).strm).avail_in == 0 as c_uint
        && (*s).strstart as c_long == (*s).block_start
    {
        return block_done;
    }
    have = ((*s).window_size as c_ulong)
        .wrapping_sub((*s).strstart as c_ulong)
        .wrapping_sub(1 as c_ulong) as c_uint;
    if (*(*s).strm).avail_in > have && (*s).block_start >= (*s).w_size as c_long {
        (*s).block_start -= (*s).w_size as c_long;
        (*s).strstart = ((*s).strstart as c_uint)
            .wrapping_sub((*s).w_size as c_uint) as uInt
            as uInt;
        memcpy(
            (*s).window as *mut c_void,
            (*s).window.offset((*s).w_size as isize) as *const c_void,
            (*s).strstart as size_t,
        );
        if (*s).matches < 2 as c_uint {
            (*s).matches = (*s).matches.wrapping_add(1);
        }
        have = have.wrapping_add((*s).w_size as c_uint);
    }
    if have > (*(*s).strm).avail_in {
        have = (*(*s).strm).avail_in as c_uint;
    }
    if have != 0 {
        read_buf((*s).strm, (*s).window.offset((*s).strstart as isize), have);
        (*s).strstart = ((*s).strstart as c_uint).wrapping_add(have) as uInt as uInt;
    }
    if (*s).high_water < (*s).strstart as c_ulong {
        (*s).high_water = (*s).strstart as ulg;
    }
    have = ((*s).bi_valid + 42 as c_int >> 3 as c_int)
        as c_uint;
    have = (if ((*s).pending_buf_size as c_ulong)
        .wrapping_sub(have as c_ulong)
        > 65535 as c_ulong
    {
        65535 as c_ulong
    } else {
        ((*s).pending_buf_size as c_ulong).wrapping_sub(have as c_ulong)
    }) as c_uint;
    min_block = if have > (*s).w_size {
        (*s).w_size as c_uint
    } else {
        have
    };
    left = ((*s).strstart as c_long - (*s).block_start) as c_uint;
    if left >= min_block
        || (left != 0 || flush == Z_FINISH)
            && flush != Z_NO_FLUSH
            && (*(*s).strm).avail_in == 0 as c_uint
            && left <= have
    {
        len = if left > have { have } else { left };
        last = (if flush == Z_FINISH
            && (*(*s).strm).avail_in == 0 as c_uint
            && len == left
        {
            1 as c_int
        } else {
            0 as c_int
        }) as c_uint;
        _tr_stored_block(
            s,
            ((*s).window as *mut charf).offset((*s).block_start as isize),
            len as ulg,
            last as c_int,
        );
        (*s).block_start += len as c_long;
        flush_pending((*s).strm);
    }
    return (if last != 0 {
        finish_started as c_int
    } else {
        need_more as c_int
    }) as block_state;
}
unsafe extern "C" fn deflate_fast(
    mut s: *mut deflate_state,
    mut flush: c_int,
) -> block_state {
    let mut hash_head: IPos = 0;
    let mut bflush: c_int = 0;
    loop {
        if (*s).lookahead < MIN_LOOKAHEAD as c_uint {
            fill_window(s);
            if (*s).lookahead < MIN_LOOKAHEAD as c_uint && flush == Z_NO_FLUSH {
                return need_more;
            }
            if (*s).lookahead == 0 as c_uint {
                break;
            }
        }
        hash_head = NIL as IPos;
        if (*s).lookahead >= MIN_MATCH as c_uint {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s)
                    .window
                    .offset(((*s).strstart as c_uint).wrapping_add(
                        (3 as c_int - 1 as c_int) as c_uint,
                    ) as isize) as uInt)
                & (*s).hash_mask;
            let ref mut fresh7 = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
            *fresh7 = *(*s).head.offset((*s).ins_h as isize);
            hash_head = *fresh7 as IPos;
            *(*s).head.offset((*s).ins_h as isize) = (*s).strstart as Pos as Posf;
        }
        if hash_head != NIL as c_uint
            && ((*s).strstart as IPos).wrapping_sub(hash_head)
                <= ((*s).w_size as c_uint)
                    .wrapping_sub(MIN_LOOKAHEAD as c_uint)
        {
            (*s).match_length = longest_match(s, hash_head);
        }
        if (*s).match_length >= MIN_MATCH as c_uint {
            let mut len: uch = ((*s).match_length as c_uint)
                .wrapping_sub(3 as c_uint) as uch;
            let mut dist: ush = (*s).strstart.wrapping_sub((*s).match_start) as ush;
            *(*s).d_buf.offset((*s).last_lit as isize) = dist as ushf;
            let fresh8 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh8 as isize) = len as uchf;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*(&raw const _length_code as *const uch).offset(len as isize)
                as c_int
                + LITERALS
                + 1 as c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(*(&raw const _length_code as *const uch)
                .offset(len as isize) as c_int
                + LITERALS
                + 1 as c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[(if (dist as c_int) < 256 as c_int {
                *(&raw const _dist_code as *const uch).offset(dist as isize) as c_int
            } else {
                *(&raw const _dist_code as *const uch).offset(
                    (256 as c_int
                        + (dist as c_int >> 7 as c_int))
                        as isize,
                ) as c_int
            }) as usize]
                .fc
                .freq = (*s).dyn_dtree[(if (dist as c_int) < 256 as c_int
            {
                *(&raw const _dist_code as *const uch).offset(dist as isize) as c_int
            } else {
                *(&raw const _dist_code as *const uch).offset(
                    (256 as c_int
                        + (dist as c_int >> 7 as c_int))
                        as isize,
                ) as c_int
            }) as usize]
                .fc
                .freq
                .wrapping_add(1);
            bflush = ((*s).last_lit
                == ((*s).lit_bufsize as c_uint).wrapping_sub(1 as c_uint))
                as c_int;
            (*s).lookahead = ((*s).lookahead as c_uint)
                .wrapping_sub((*s).match_length as c_uint)
                as uInt as uInt;
            if (*s).match_length <= (*s).max_lazy_match
                && (*s).lookahead >= MIN_MATCH as c_uint
            {
                (*s).match_length = (*s).match_length.wrapping_sub(1);
                loop {
                    (*s).strstart = (*s).strstart.wrapping_add(1);
                    (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                        ^ *(*s)
                            .window
                            .offset(((*s).strstart as c_uint).wrapping_add(
                                (3 as c_int - 1 as c_int)
                                    as c_uint,
                            ) as isize) as uInt)
                        & (*s).hash_mask;
                    let ref mut fresh9 = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
                    *fresh9 = *(*s).head.offset((*s).ins_h as isize);
                    hash_head = *fresh9 as IPos;
                    *(*s).head.offset((*s).ins_h as isize) = (*s).strstart as Pos as Posf;
                    (*s).match_length = (*s).match_length.wrapping_sub(1);
                    if !((*s).match_length != 0 as c_uint) {
                        break;
                    }
                }
                (*s).strstart = (*s).strstart.wrapping_add(1);
            } else {
                (*s).strstart = ((*s).strstart as c_uint)
                    .wrapping_add((*s).match_length as c_uint)
                    as uInt as uInt;
                (*s).match_length = 0 as uInt;
                (*s).ins_h = *(*s).window.offset((*s).strstart as isize) as uInt;
                (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                    ^ *(*s).window.offset(
                        ((*s).strstart as c_uint)
                            .wrapping_add(1 as c_uint)
                            as isize,
                    ) as uInt)
                    & (*s).hash_mask;
            }
        } else {
            let mut cc: uch = *(*s).window.offset((*s).strstart as isize) as uch;
            *(*s).d_buf.offset((*s).last_lit as isize) = 0 as ushf;
            let fresh10 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh10 as isize) = cc as uchf;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
            bflush = ((*s).last_lit
                == ((*s).lit_bufsize as c_uint).wrapping_sub(1 as c_uint))
                as c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            (*s).strstart = (*s).strstart.wrapping_add(1);
        }
        if bflush != 0 {
            _tr_flush_block(
                s,
                if (*s).block_start >= 0 as c_long {
                    (*s).window
                        .offset((*s).block_start as c_uint as isize)
                        as *mut Bytef as *mut charf
                } else {
                    ::core::ptr::null_mut::<charf>()
                },
                ((*s).strstart as c_long - (*s).block_start) as ulg,
                0 as c_int,
            );
            (*s).block_start = (*s).strstart as c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as c_uint {
                return (if 0 as c_int != 0 {
                    finish_started as c_int
                } else {
                    need_more as c_int
                }) as block_state;
            }
        }
    }
    (*s).insert = (if (*s).strstart < (MIN_MATCH - 1 as c_int) as c_uint {
        (*s).strstart as c_uint
    } else {
        (MIN_MATCH - 1 as c_int) as c_uint
    }) as uInt;
    if flush == Z_FINISH {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as c_long {
                (*s).window
                    .offset((*s).block_start as c_uint as isize)
                    as *mut Bytef as *mut charf
            } else {
                ::core::ptr::null_mut::<charf>()
            },
            ((*s).strstart as c_long - (*s).block_start) as ulg,
            1 as c_int,
        );
        (*s).block_start = (*s).strstart as c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as c_uint {
            return (if 1 as c_int != 0 {
                finish_started as c_int
            } else {
                need_more as c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).last_lit != 0 {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as c_long {
                (*s).window
                    .offset((*s).block_start as c_uint as isize)
                    as *mut Bytef as *mut charf
            } else {
                ::core::ptr::null_mut::<charf>()
            },
            ((*s).strstart as c_long - (*s).block_start) as ulg,
            0 as c_int,
        );
        (*s).block_start = (*s).strstart as c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as c_uint {
            return (if 0 as c_int != 0 {
                finish_started as c_int
            } else {
                need_more as c_int
            }) as block_state;
        }
    }
    return block_done;
}
unsafe extern "C" fn deflate_slow(
    mut s: *mut deflate_state,
    mut flush: c_int,
) -> block_state {
    let mut hash_head: IPos = 0;
    let mut bflush: c_int = 0;
    let w_mask = (*s).w_mask;
    let hash_shift = (*s).hash_shift;
    let hash_mask = (*s).hash_mask;
    let max_lazy_match = (*s).max_lazy_match;
    let w_size = (*s).w_size;
    let strategy = (*s).strategy;
    let lit_bufsize_m1 = ((*s).lit_bufsize as c_uint).wrapping_sub(1 as c_uint);
    let window = (*s).window;
    let prev = (*s).prev;
    let head = (*s).head;
    let d_buf = (*s).d_buf;
    let l_buf = (*s).l_buf;
    loop {
        if (*s).lookahead < MIN_LOOKAHEAD as c_uint {
            fill_window(s);
            if (*s).lookahead < MIN_LOOKAHEAD as c_uint && flush == Z_NO_FLUSH {
                return need_more;
            }
            if (*s).lookahead == 0 as c_uint {
                break;
            }
        }
        hash_head = NIL as IPos;
        if (*s).lookahead >= MIN_MATCH as c_uint {
            (*s).ins_h = ((*s).ins_h << hash_shift
                ^ *window
                    .offset(((*s).strstart as c_uint).wrapping_add(
                        (3 as c_int - 1 as c_int) as c_uint,
                    ) as isize) as uInt)
                & hash_mask;
            let ref mut fresh2 = *prev.offset(((*s).strstart & w_mask) as isize);
            *fresh2 = *head.offset((*s).ins_h as isize);
            hash_head = *fresh2 as IPos;
            *head.offset((*s).ins_h as isize) = (*s).strstart as Pos as Posf;
        }
        (*s).prev_length = (*s).match_length;
        (*s).prev_match = (*s).match_start as IPos;
        (*s).match_length = (MIN_MATCH - 1 as c_int) as uInt;
        if hash_head != NIL as c_uint
            && (*s).prev_length < max_lazy_match
            && ((*s).strstart as IPos).wrapping_sub(hash_head)
                <= (w_size as c_uint).wrapping_sub(MIN_LOOKAHEAD as c_uint)
        {
            (*s).match_length = longest_match(s, hash_head);
            if (*s).match_length <= 5 as c_uint && strategy == Z_FILTERED {
                (*s).match_length = (MIN_MATCH - 1 as c_int) as uInt;
            }
        }
        if (*s).prev_length >= MIN_MATCH as c_uint
            && (*s).match_length <= (*s).prev_length
        {
            let mut max_insert: uInt = (*s)
                .strstart
                .wrapping_add((*s).lookahead)
                .wrapping_sub(MIN_MATCH as uInt);
            let mut len: uch = ((*s).prev_length as c_uint)
                .wrapping_sub(3 as c_uint) as uch;
            let mut dist: ush = ((*s).strstart as IPos)
                .wrapping_sub(1 as IPos)
                .wrapping_sub((*s).prev_match) as ush;
            *d_buf.offset((*s).last_lit as isize) = dist as ushf;
            let fresh3 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *l_buf.offset(fresh3 as isize) = len as uchf;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*(&raw const _length_code as *const uch).offset(len as isize)
                as c_int
                + LITERALS
                + 1 as c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(*(&raw const _length_code as *const uch)
                .offset(len as isize) as c_int
                + LITERALS
                + 1 as c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[(if (dist as c_int) < 256 as c_int {
                *(&raw const _dist_code as *const uch).offset(dist as isize) as c_int
            } else {
                *(&raw const _dist_code as *const uch).offset(
                    (256 as c_int
                        + (dist as c_int >> 7 as c_int))
                        as isize,
                ) as c_int
            }) as usize]
                .fc
                .freq = (*s).dyn_dtree[(if (dist as c_int) < 256 as c_int
            {
                *(&raw const _dist_code as *const uch).offset(dist as isize) as c_int
            } else {
                *(&raw const _dist_code as *const uch).offset(
                    (256 as c_int
                        + (dist as c_int >> 7 as c_int))
                        as isize,
                ) as c_int
            }) as usize]
                .fc
                .freq
                .wrapping_add(1);
            bflush = ((*s).last_lit == lit_bufsize_m1) as c_int;
            (*s).lookahead = ((*s).lookahead as c_uint).wrapping_sub(
                ((*s).prev_length as c_uint).wrapping_sub(1 as c_uint),
            ) as uInt as uInt;
            (*s).prev_length = ((*s).prev_length as c_uint)
                .wrapping_sub(2 as c_uint) as uInt
                as uInt;
            loop {
                (*s).strstart = (*s).strstart.wrapping_add(1);
                if (*s).strstart <= max_insert {
                    (*s).ins_h = ((*s).ins_h << hash_shift
                        ^ *window
                            .offset(((*s).strstart as c_uint).wrapping_add(
                                (3 as c_int - 1 as c_int)
                                    as c_uint,
                            ) as isize) as uInt)
                        & hash_mask;
                    let ref mut fresh4 = *prev.offset(((*s).strstart & w_mask) as isize);
                    *fresh4 = *head.offset((*s).ins_h as isize);
                    hash_head = *fresh4 as IPos;
                    *head.offset((*s).ins_h as isize) = (*s).strstart as Pos as Posf;
                }
                (*s).prev_length = (*s).prev_length.wrapping_sub(1);
                if !((*s).prev_length != 0 as c_uint) {
                    break;
                }
            }
            (*s).match_available = 0 as c_int;
            (*s).match_length = (MIN_MATCH - 1 as c_int) as uInt;
            (*s).strstart = (*s).strstart.wrapping_add(1);
            if bflush != 0 {
                _tr_flush_block(
                    s,
                    if (*s).block_start >= 0 as c_long {
                        window
                            .offset((*s).block_start as c_uint as isize)
                            as *mut Bytef as *mut charf
                    } else {
                        ::core::ptr::null_mut::<charf>()
                    },
                    ((*s).strstart as c_long - (*s).block_start) as ulg,
                    0 as c_int,
                );
                (*s).block_start = (*s).strstart as c_long;
                flush_pending((*s).strm);
                if (*(*s).strm).avail_out == 0 as c_uint {
                    return (if 0 as c_int != 0 {
                        finish_started as c_int
                    } else {
                        need_more as c_int
                    }) as block_state;
                }
            }
        } else if (*s).match_available != 0 {
            let mut cc: uch = *window.offset(
                ((*s).strstart as c_uint).wrapping_sub(1 as c_uint)
                    as isize,
            ) as uch;
            *d_buf.offset((*s).last_lit as isize) = 0 as ushf;
            let fresh5 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *l_buf.offset(fresh5 as isize) = cc as uchf;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
            bflush = ((*s).last_lit == lit_bufsize_m1) as c_int;
            if bflush != 0 {
                _tr_flush_block(
                    s,
                    if (*s).block_start >= 0 as c_long {
                        window
                            .offset((*s).block_start as c_uint as isize)
                            as *mut Bytef as *mut charf
                    } else {
                        ::core::ptr::null_mut::<charf>()
                    },
                    ((*s).strstart as c_long - (*s).block_start) as ulg,
                    0 as c_int,
                );
                (*s).block_start = (*s).strstart as c_long;
                flush_pending((*s).strm);
            }
            (*s).strstart = (*s).strstart.wrapping_add(1);
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            if (*(*s).strm).avail_out == 0 as c_uint {
                return need_more;
            }
        } else {
            (*s).match_available = 1 as c_int;
            (*s).strstart = (*s).strstart.wrapping_add(1);
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
        }
    }
    if (*s).match_available != 0 {
        let mut cc_0: uch = *(*s).window.offset(
            ((*s).strstart as c_uint).wrapping_sub(1 as c_uint) as isize,
        ) as uch;
        *(*s).d_buf.offset((*s).last_lit as isize) = 0 as ushf;
        let fresh6 = (*s).last_lit;
        (*s).last_lit = (*s).last_lit.wrapping_add(1);
        *(*s).l_buf.offset(fresh6 as isize) = cc_0 as uchf;
        (*s).dyn_ltree[cc_0 as usize].fc.freq =
            (*s).dyn_ltree[cc_0 as usize].fc.freq.wrapping_add(1);
        bflush = ((*s).last_lit
            == ((*s).lit_bufsize as c_uint).wrapping_sub(1 as c_uint))
            as c_int;
        (*s).match_available = 0 as c_int;
    }
    (*s).insert = (if (*s).strstart < (MIN_MATCH - 1 as c_int) as c_uint {
        (*s).strstart as c_uint
    } else {
        (MIN_MATCH - 1 as c_int) as c_uint
    }) as uInt;
    if flush == Z_FINISH {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as c_long {
                (*s).window
                    .offset((*s).block_start as c_uint as isize)
                    as *mut Bytef as *mut charf
            } else {
                ::core::ptr::null_mut::<charf>()
            },
            ((*s).strstart as c_long - (*s).block_start) as ulg,
            1 as c_int,
        );
        (*s).block_start = (*s).strstart as c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as c_uint {
            return (if 1 as c_int != 0 {
                finish_started as c_int
            } else {
                need_more as c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).last_lit != 0 {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as c_long {
                (*s).window
                    .offset((*s).block_start as c_uint as isize)
                    as *mut Bytef as *mut charf
            } else {
                ::core::ptr::null_mut::<charf>()
            },
            ((*s).strstart as c_long - (*s).block_start) as ulg,
            0 as c_int,
        );
        (*s).block_start = (*s).strstart as c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as c_uint {
            return (if 0 as c_int != 0 {
                finish_started as c_int
            } else {
                need_more as c_int
            }) as block_state;
        }
    }
    return block_done;
}
unsafe fn deflate_rle(
    mut s: *mut deflate_state,
    mut flush: c_int,
) -> block_state {
    let mut bflush: c_int = 0;
    let mut prev: uInt = 0;
    let mut scan: *mut Bytef = ::core::ptr::null_mut::<Bytef>();
    let mut strend: *mut Bytef = ::core::ptr::null_mut::<Bytef>();
    loop {
        if (*s).lookahead <= MAX_MATCH as c_uint {
            fill_window(s);
            if (*s).lookahead <= MAX_MATCH as c_uint && flush == Z_NO_FLUSH {
                return need_more;
            }
            if (*s).lookahead == 0 as c_uint {
                break;
            }
        }
        (*s).match_length = 0 as uInt;
        if (*s).lookahead >= MIN_MATCH as c_uint
            && (*s).strstart > 0 as c_uint
        {
            scan = (*s)
                .window
                .offset((*s).strstart as isize)
                .offset(-(1 as c_int as isize));
            prev = *scan as uInt;
            scan = scan.offset(1);
            if prev == *scan as c_uint
                && {
                    scan = scan.offset(1);
                    prev == *scan as c_uint
                }
                && {
                    scan = scan.offset(1);
                    prev == *scan as c_uint
                }
            {
                strend = (*s)
                    .window
                    .offset((*s).strstart as isize)
                    .offset(MAX_MATCH as isize);
                loop {
                    scan = scan.offset(1);
                    if !(prev == *scan as c_uint
                        && {
                            scan = scan.offset(1);
                            prev == *scan as c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            prev == *scan as c_uint
                        }
                        && scan < strend)
                    {
                        break;
                    }
                }
                (*s).match_length = (MAX_MATCH as uInt)
                    .wrapping_sub(strend.offset_from(scan) as c_long as uInt);
                if (*s).match_length > (*s).lookahead {
                    (*s).match_length = (*s).lookahead;
                }
            }
        }
        if (*s).match_length >= MIN_MATCH as c_uint {
            let mut len: uch = ((*s).match_length as c_uint)
                .wrapping_sub(3 as c_uint) as uch;
            let mut dist: ush = 1 as c_int as ush;
            *(*s).d_buf.offset((*s).last_lit as isize) = dist as ushf;
            let fresh11 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh11 as isize) = len as uchf;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*(&raw const _length_code as *const uch).offset(len as isize)
                as c_int
                + LITERALS
                + 1 as c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(*(&raw const _length_code as *const uch)
                .offset(len as isize) as c_int
                + LITERALS
                + 1 as c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[(if (dist as c_int) < 256 as c_int {
                *(&raw const _dist_code as *const uch).offset(dist as isize) as c_int
            } else {
                *(&raw const _dist_code as *const uch).offset(
                    (256 as c_int
                        + (dist as c_int >> 7 as c_int))
                        as isize,
                ) as c_int
            }) as usize]
                .fc
                .freq = (*s).dyn_dtree[(if (dist as c_int) < 256 as c_int
            {
                *(&raw const _dist_code as *const uch).offset(dist as isize) as c_int
            } else {
                *(&raw const _dist_code as *const uch).offset(
                    (256 as c_int
                        + (dist as c_int >> 7 as c_int))
                        as isize,
                ) as c_int
            }) as usize]
                .fc
                .freq
                .wrapping_add(1);
            bflush = ((*s).last_lit
                == ((*s).lit_bufsize as c_uint).wrapping_sub(1 as c_uint))
                as c_int;
            (*s).lookahead = ((*s).lookahead as c_uint)
                .wrapping_sub((*s).match_length as c_uint)
                as uInt as uInt;
            (*s).strstart = ((*s).strstart as c_uint)
                .wrapping_add((*s).match_length as c_uint)
                as uInt as uInt;
            (*s).match_length = 0 as uInt;
        } else {
            let mut cc: uch = *(*s).window.offset((*s).strstart as isize) as uch;
            *(*s).d_buf.offset((*s).last_lit as isize) = 0 as ushf;
            let fresh12 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh12 as isize) = cc as uchf;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
            bflush = ((*s).last_lit
                == ((*s).lit_bufsize as c_uint).wrapping_sub(1 as c_uint))
                as c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            (*s).strstart = (*s).strstart.wrapping_add(1);
        }
        if bflush != 0 {
            _tr_flush_block(
                s,
                if (*s).block_start >= 0 as c_long {
                    (*s).window
                        .offset((*s).block_start as c_uint as isize)
                        as *mut Bytef as *mut charf
                } else {
                    ::core::ptr::null_mut::<charf>()
                },
                ((*s).strstart as c_long - (*s).block_start) as ulg,
                0 as c_int,
            );
            (*s).block_start = (*s).strstart as c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as c_uint {
                return (if 0 as c_int != 0 {
                    finish_started as c_int
                } else {
                    need_more as c_int
                }) as block_state;
            }
        }
    }
    (*s).insert = 0 as uInt;
    if flush == Z_FINISH {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as c_long {
                (*s).window
                    .offset((*s).block_start as c_uint as isize)
                    as *mut Bytef as *mut charf
            } else {
                ::core::ptr::null_mut::<charf>()
            },
            ((*s).strstart as c_long - (*s).block_start) as ulg,
            1 as c_int,
        );
        (*s).block_start = (*s).strstart as c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as c_uint {
            return (if 1 as c_int != 0 {
                finish_started as c_int
            } else {
                need_more as c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).last_lit != 0 {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as c_long {
                (*s).window
                    .offset((*s).block_start as c_uint as isize)
                    as *mut Bytef as *mut charf
            } else {
                ::core::ptr::null_mut::<charf>()
            },
            ((*s).strstart as c_long - (*s).block_start) as ulg,
            0 as c_int,
        );
        (*s).block_start = (*s).strstart as c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as c_uint {
            return (if 0 as c_int != 0 {
                finish_started as c_int
            } else {
                need_more as c_int
            }) as block_state;
        }
    }
    return block_done;
}
unsafe fn deflate_huff(
    mut s: *mut deflate_state,
    mut flush: c_int,
) -> block_state {
    let mut bflush: c_int = 0;
    loop {
        if (*s).lookahead == 0 as c_uint {
            fill_window(s);
            if (*s).lookahead == 0 as c_uint {
                if flush == Z_NO_FLUSH {
                    return need_more;
                }
                break;
            }
        }
        (*s).match_length = 0 as uInt;
        let mut cc: uch = *(*s).window.offset((*s).strstart as isize) as uch;
        *(*s).d_buf.offset((*s).last_lit as isize) = 0 as ushf;
        let fresh13 = (*s).last_lit;
        (*s).last_lit = (*s).last_lit.wrapping_add(1);
        *(*s).l_buf.offset(fresh13 as isize) = cc as uchf;
        (*s).dyn_ltree[cc as usize].fc.freq = (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
        bflush = ((*s).last_lit
            == ((*s).lit_bufsize as c_uint).wrapping_sub(1 as c_uint))
            as c_int;
        (*s).lookahead = (*s).lookahead.wrapping_sub(1);
        (*s).strstart = (*s).strstart.wrapping_add(1);
        if bflush != 0 {
            _tr_flush_block(
                s,
                if (*s).block_start >= 0 as c_long {
                    (*s).window
                        .offset((*s).block_start as c_uint as isize)
                        as *mut Bytef as *mut charf
                } else {
                    ::core::ptr::null_mut::<charf>()
                },
                ((*s).strstart as c_long - (*s).block_start) as ulg,
                0 as c_int,
            );
            (*s).block_start = (*s).strstart as c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as c_uint {
                return (if 0 as c_int != 0 {
                    finish_started as c_int
                } else {
                    need_more as c_int
                }) as block_state;
            }
        }
    }
    (*s).insert = 0 as uInt;
    if flush == Z_FINISH {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as c_long {
                (*s).window
                    .offset((*s).block_start as c_uint as isize)
                    as *mut Bytef as *mut charf
            } else {
                ::core::ptr::null_mut::<charf>()
            },
            ((*s).strstart as c_long - (*s).block_start) as ulg,
            1 as c_int,
        );
        (*s).block_start = (*s).strstart as c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as c_uint {
            return (if 1 as c_int != 0 {
                finish_started as c_int
            } else {
                need_more as c_int
            }) as block_state;
        }
        return finish_done;
    }
    if (*s).last_lit != 0 {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as c_long {
                (*s).window
                    .offset((*s).block_start as c_uint as isize)
                    as *mut Bytef as *mut charf
            } else {
                ::core::ptr::null_mut::<charf>()
            },
            ((*s).strstart as c_long - (*s).block_start) as ulg,
            0 as c_int,
        );
        (*s).block_start = (*s).strstart as c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as c_uint {
            return (if 0 as c_int != 0 {
                finish_started as c_int
            } else {
                need_more as c_int
            }) as block_state;
        }
    }
    return block_done;
}
