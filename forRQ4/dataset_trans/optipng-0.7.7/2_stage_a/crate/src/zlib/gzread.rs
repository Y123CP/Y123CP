use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::zlib::deflate::internal_state;
extern "C" {
    fn inflate(strm: z_streamp, flush: c_int) -> c_int;
    fn inflateEnd(strm: z_streamp) -> c_int;
    fn inflateReset(strm: z_streamp) -> c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
    fn gz_error(_: gz_statep, _: c_int, _: *const c_char);
    fn memchr(
        __s: *const c_void,
        __c: c_int,
        __n: size_t,
    ) -> *mut c_void;
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

pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub x: gzFile_s,
    pub mode: c_int,
    pub fd: c_int,
    pub path: *mut c_char,
    pub size: c_uint,
    pub want: c_uint,
    pub in_0: *mut c_uchar,
    pub out: *mut c_uchar,
    pub direct: c_int,
    pub how: c_int,
    pub start: off_t,
    pub eof: c_int,
    pub past: c_int,
    pub level: c_int,
    pub strategy: c_int,
    pub skip: off_t,
    pub seek: c_int,
    pub err: c_int,
    pub msg: *mut c_char,
    pub strm: z_stream,
}

pub const ZLIB_VERSION: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"1.2.11-optipng\0") };

pub const Z_OK: c_int = 0 as c_int;
pub const Z_STREAM_END: c_int = 1 as c_int;
pub const Z_NEED_DICT: c_int = 2 as c_int;
pub const Z_ERRNO: c_int = -(1 as c_int);

pub const Z_DATA_ERROR: c_int = -(3 as c_int);
pub const Z_MEM_ERROR: c_int = -(4 as c_int);
pub const Z_BUF_ERROR: c_int = -(5 as c_int);

unsafe fn gz_load(
    mut state: gz_statep,
    mut buf: *mut c_uchar,
    mut len: c_uint,
    mut have: *mut c_uint,
) -> c_int {
    let have_view: &mut c_uint = unsafe { &mut *have };
    let mut ret: c_int = 0;
    let mut get: c_uint = 0;
    let mut max: c_uint = (-(1 as c_int) as c_uint
        >> 2 as c_int)
        .wrapping_add(1 as c_uint);
    *have_view = 0 as c_uint;
    loop {
        get = len.wrapping_sub(*have_view);
        if get > max {
            get = max;
        }
        ret = read(
            (*state).fd,
            buf.offset(*have_view as isize) as *mut c_void,
            get as size_t,
        ) as c_int;
        if ret <= 0 as c_int {
            break;
        }
        *have_view = have_view.wrapping_add(ret as c_uint);
        if !(*have_view < len) {
            break;
        }
    }
    if ret < 0 as c_int {
        gz_error(state, Z_ERRNO, strerror(*__errno_location()));
        return -(1 as c_int);
    }
    if ret == 0 as c_int {
        (*state).eof = 1 as c_int;
    }
    return 0 as c_int;
}
fn gz_avail(mut state: gz_statep) -> c_int { unsafe {
    let mut got: c_uint = 0;
    let mut strm: z_streamp = &raw mut (*state).strm;
    if (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as c_int);
    }
    if (*state).eof == 0 as c_int {
        if (*strm).avail_in != 0 {
            let mut p: *mut c_uchar = (*state).in_0;
            let mut q: *const c_uchar = (*strm).next_in;
            let mut n: c_uint = (*strm).avail_in as c_uint;
            loop {
                let fresh0 = q;
                q = q.offset(1);
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = *fresh0;
                n = n.wrapping_sub(1);
                if !(n != 0) {
                    break;
                }
            }
        }
        if gz_load(
            state,
            (*state).in_0.offset((*strm).avail_in as isize),
            (*state)
                .size
                .wrapping_sub((*strm).avail_in as c_uint),
            &raw mut got,
        ) == -(1 as c_int)
        {
            return -(1 as c_int);
        }
        (*strm).avail_in =
            ((*strm).avail_in as c_uint).wrapping_add(got) as uInt as uInt;
        (*strm).next_in = (*state).in_0 as *mut Bytef;
    }
    return 0 as c_int;
} }
fn gz_look(mut state: gz_statep) -> c_int { unsafe {
    let mut strm: z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as c_uint {
        (*state).in_0 = malloc((*state).want as size_t) as *mut c_uchar;
        (*state).out = malloc(((*state).want << 1 as c_int) as size_t)
            as *mut c_uchar;
        if (*state).in_0.is_null() || (*state).out.is_null() {
            free((*state).out as *mut c_void);
            free((*state).in_0 as *mut c_void);
            gz_error(
                state,
                Z_MEM_ERROR,
                b"out of memory\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*state).size = (*state).want;
        (*state).strm.zalloc = None;
        (*state).strm.zfree = None;
        (*state).strm.opaque = ::core::ptr::null_mut::<c_void>();
        (*state).strm.avail_in = 0 as uInt;
        (*state).strm.next_in = ::core::ptr::null_mut::<Bytef>();
        if inflateInit2_(
            &raw mut (*state).strm,
            15 as c_int + 16 as c_int,
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as c_int,
        ) != Z_OK
        {
            free((*state).out as *mut c_void);
            free((*state).in_0 as *mut c_void);
            (*state).size = 0 as c_uint;
            gz_error(
                state,
                Z_MEM_ERROR,
                b"out of memory\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
    }
    if (*strm).avail_in < 2 as c_uint {
        if gz_avail(state) == -(1 as c_int) {
            return -(1 as c_int);
        }
        if (*strm).avail_in == 0 as c_uint {
            return 0 as c_int;
        }
    }
    if (*strm).avail_in > 1 as c_uint
        && *(*strm).next_in.offset(0 as c_int as isize) as c_int
            == 31 as c_int
        && *(*strm).next_in.offset(1 as c_int as isize) as c_int
            == 139 as c_int
    {
        inflateReset(strm);
        (*state).how = GZIP;
        (*state).direct = 0 as c_int;
        return 0 as c_int;
    }
    if (*state).direct == 0 as c_int {
        (*strm).avail_in = 0 as uInt;
        (*state).eof = 1 as c_int;
        (*state).x.have = 0 as c_uint;
        return 0 as c_int;
    }
    (*state).x.next = (*state).out;
    if (*strm).avail_in != 0 {
        memcpy(
            (*state).x.next as *mut c_void,
            (*strm).next_in as *const c_void,
            (*strm).avail_in as size_t,
        );
        (*state).x.have = (*strm).avail_in as c_uint;
        (*strm).avail_in = 0 as uInt;
    }
    (*state).how = COPY;
    (*state).direct = 1 as c_int;
    return 0 as c_int;
} }
fn gz_decomp(mut state: gz_statep) -> c_int { unsafe {
    let mut ret: c_int = Z_OK;
    let mut had: c_uint = 0;
    let mut strm: z_streamp = &raw mut (*state).strm;
    had = (*strm).avail_out as c_uint;
    loop {
        if (*strm).avail_in == 0 as c_uint
            && gz_avail(state) == -(1 as c_int)
        {
            return -(1 as c_int);
        }
        if (*strm).avail_in == 0 as c_uint {
            gz_error(
                state,
                Z_BUF_ERROR,
                b"unexpected end of file\0" as *const u8 as *const c_char,
            );
            break;
        } else {
            ret = inflate(strm, Z_NO_FLUSH);
            if ret == Z_STREAM_ERROR || ret == Z_NEED_DICT {
                gz_error(
                    state,
                    Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0" as *const u8
                        as *const c_char,
                );
                return -(1 as c_int);
            }
            if ret == Z_MEM_ERROR {
                gz_error(
                    state,
                    Z_MEM_ERROR,
                    b"out of memory\0" as *const u8 as *const c_char,
                );
                return -(1 as c_int);
            }
            if ret == Z_DATA_ERROR {
                gz_error(
                    state,
                    Z_DATA_ERROR,
                    if (*strm).msg.is_null() {
                        b"compressed data error\0" as *const u8 as *const c_char
                    } else {
                        (*strm).msg as *const c_char
                    },
                );
                return -(1 as c_int);
            }
            if !((*strm).avail_out != 0 && ret != Z_STREAM_END) {
                break;
            }
        }
    }
    (*state).x.have = (had as uInt).wrapping_sub((*strm).avail_out) as c_uint;
    (*state).x.next =
        (*strm).next_out.offset(-((*state).x.have as isize)) as *mut c_uchar;
    if ret == Z_STREAM_END {
        (*state).how = LOOK;
    }
    return 0 as c_int;
} }
fn gz_fetch(mut state: gz_statep) -> c_int { unsafe {
    let mut strm: z_streamp = &raw mut (*state).strm;
    loop {
        match (*state).how {
            LOOK => {
                if gz_look(state) == -(1 as c_int) {
                    return -(1 as c_int);
                }
                if (*state).how == LOOK {
                    return 0 as c_int;
                }
            }
            COPY => {
                if gz_load(
                    state,
                    (*state).out,
                    (*state).size << 1 as c_int,
                    &raw mut (*state).x.have,
                ) == -(1 as c_int)
                {
                    return -(1 as c_int);
                }
                (*state).x.next = (*state).out;
                return 0 as c_int;
            }
            GZIP => {
                (*strm).avail_out = ((*state).size << 1 as c_int) as uInt;
                (*strm).next_out = (*state).out as *mut Bytef;
                if gz_decomp(state) == -(1 as c_int) {
                    return -(1 as c_int);
                }
            }
            _ => {}
        }
        if !((*state).x.have == 0 as c_uint
            && ((*state).eof == 0 || (*strm).avail_in != 0))
        {
            break;
        }
    }
    return 0 as c_int;
} }
fn gz_skip(mut state: gz_statep, mut len: off_t) -> c_int { unsafe {
    let mut n: c_uint = 0;
    while len != 0 {
        if (*state).x.have != 0 {
            n = if ::core::mem::size_of::<c_int>() as usize
                == ::core::mem::size_of::<off_t>() as usize
                && (*state).x.have > INT_MAX as c_uint
                || (*state).x.have as off_t > len
            {
                len as c_uint
            } else {
                (*state).x.have
            };
            (*state).x.have = (*state).x.have.wrapping_sub(n);
            (*state).x.next = (*state).x.next.offset(n as isize);
            (*state).x.pos += n as c_long;
            len -= n as c_long;
        } else {
            if (*state).eof != 0 && (*state).strm.avail_in == 0 as c_uint {
                break;
            }
            if gz_fetch(state) == -(1 as c_int) {
                return -(1 as c_int);
            }
        }
    }
    return 0 as c_int;
} }
fn gz_read(mut state: gz_statep, mut buf: voidp, mut len: z_size_t) -> z_size_t { unsafe {
    let mut got: z_size_t = 0;
    let mut n: c_uint = 0;
    if len == 0 as z_size_t {
        return 0 as z_size_t;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_skip(state, (*state).skip) == -(1 as c_int) {
            return 0 as z_size_t;
        }
    }
    got = 0 as z_size_t;
    let mut current_block_34: u64;
    loop {
        n = -(1 as c_int) as c_uint;
        if n as z_size_t > len {
            n = len as c_uint;
        }
        if (*state).x.have != 0 {
            if (*state).x.have < n {
                n = (*state).x.have;
            }
            memcpy(
                buf as *mut c_void,
                (*state).x.next as *const c_void,
                n as size_t,
            );
            (*state).x.next = (*state).x.next.offset(n as isize);
            (*state).x.have = (*state).x.have.wrapping_sub(n);
            current_block_34 = 13550086250199790493;
        } else if (*state).eof != 0 && (*state).strm.avail_in == 0 as c_uint {
            (*state).past = 1 as c_int;
            break;
        } else if (*state).how == LOOK || n < (*state).size << 1 as c_int {
            if gz_fetch(state) == -(1 as c_int) {
                return 0 as z_size_t;
            }
            current_block_34 = 4906268039856690917;
        } else {
            if (*state).how == COPY {
                if gz_load(state, buf as *mut c_uchar, n, &raw mut n)
                    == -(1 as c_int)
                {
                    return 0 as z_size_t;
                }
            } else {
                (*state).strm.avail_out = n as uInt;
                (*state).strm.next_out = buf as *mut c_uchar as *mut Bytef;
                if gz_decomp(state) == -(1 as c_int) {
                    return 0 as z_size_t;
                }
                n = (*state).x.have;
                (*state).x.have = 0 as c_uint;
            }
            current_block_34 = 13550086250199790493;
        }
        match current_block_34 {
            13550086250199790493 => {
                len = (len as c_ulong).wrapping_sub(n as c_ulong)
                    as z_size_t as z_size_t;
                buf = (buf as *mut c_char).offset(n as isize) as voidp;
                got = (got as c_ulong).wrapping_add(n as c_ulong)
                    as z_size_t as z_size_t;
                (*state).x.pos += n as c_long;
            }
            _ => {}
        }
        if !(len != 0) {
            break;
        }
    }
    return got;
} }
#[no_mangle]
pub extern "C" fn gzread(
    mut file: gzFile,
    mut buf: voidp,
    mut len: c_uint,
) -> c_int { unsafe {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ || (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as c_int);
    }
    if (len as c_int) < 0 as c_int {
        gz_error(
            state,
            Z_STREAM_ERROR,
            b"request does not fit in an int\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    len = gz_read(state, buf, len as z_size_t) as c_uint;
    if len == 0 as c_uint && (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as c_int);
    }
    return len as c_int;
} }
#[inline]
pub fn gzfread(
    mut buf: voidp,
    mut size: z_size_t,
    mut nitems: z_size_t,
    mut file: gzFile,
) -> z_size_t { unsafe {
    let mut len: z_size_t = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return 0 as z_size_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ || (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return 0 as z_size_t;
    }
    len = nitems.wrapping_mul(size);
    if size != 0 && len.wrapping_div(size) != nitems {
        gz_error(
            state,
            Z_STREAM_ERROR,
            b"request does not fit in a size_t\0" as *const u8 as *const c_char,
        );
        return 0 as z_size_t;
    }
    return if len != 0 {
        gz_read(state, buf, len).wrapping_div(size)
    } else {
        0 as z_size_t
    };
} }
#[inline]
pub fn gzgetc(mut file: gzFile) -> c_int { unsafe {
    let mut ret: c_int = 0;
    let mut buf: [c_uchar; 1] = [0; 1];
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ || (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as c_int);
    }
    if (*state).x.have != 0 {
        (*state).x.have = (*state).x.have.wrapping_sub(1);
        (*state).x.pos += 1;
        let fresh2 = (*state).x.next;
        (*state).x.next = (*state).x.next.offset(1);
        return *fresh2 as c_int;
    }
    ret = gz_read(
        state,
        &raw mut buf as *mut c_uchar as voidp,
        1 as z_size_t,
    ) as c_int;
    return if ret < 1 as c_int {
        -(1 as c_int)
    } else {
        buf[0 as c_int as usize] as c_int
    };
} }
#[inline]
pub fn gzgetc_(mut file: gzFile) -> c_int { {
    return gzgetc(file);
} }
#[inline]
pub fn gzungetc(
    mut c: c_int,
    mut file: gzFile,
) -> c_int { unsafe {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ || (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as c_int);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_skip(state, (*state).skip) == -(1 as c_int) {
            return -(1 as c_int);
        }
    }
    if c < 0 as c_int {
        return -(1 as c_int);
    }
    if (*state).x.have == 0 as c_uint {
        (*state).x.have = 1 as c_uint;
        (*state).x.next = (*state)
            .out
            .offset(((*state).size << 1 as c_int) as isize)
            .offset(-(1 as c_int as isize));
        *(*state).x.next.offset(0 as c_int as isize) = c as c_uchar;
        (*state).x.pos -= 1;
        (*state).past = 0 as c_int;
        return c;
    }
    if (*state).x.have == (*state).size << 1 as c_int {
        gz_error(
            state,
            Z_DATA_ERROR,
            b"out of room to push characters\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if (*state).x.next == (*state).out {
        let mut src: *mut c_uchar = (*state).out.offset((*state).x.have as isize);
        let mut dest: *mut c_uchar = (*state)
            .out
            .offset(((*state).size << 1 as c_int) as isize);
        while src > (*state).out {
            src = src.offset(-1);
            dest = dest.offset(-1);
            *dest = *src;
        }
        (*state).x.next = dest;
    }
    (*state).x.have = (*state).x.have.wrapping_add(1);
    (*state).x.next = (*state).x.next.offset(-1);
    *(*state).x.next.offset(0 as c_int as isize) = c as c_uchar;
    (*state).x.pos -= 1;
    (*state).past = 0 as c_int;
    return c;
} }
#[inline]
pub unsafe fn gzgets(
    mut file: gzFile,
    mut buf: *mut c_char,
    mut len: c_int,
) -> *mut c_char {
    let mut left: c_uint = 0;
    let mut n: c_uint = 0;
    let mut str: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut eol: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() || buf.is_null() || len < 1 as c_int {
        return ::core::ptr::null_mut::<c_char>();
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ || (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return ::core::ptr::null_mut::<c_char>();
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_skip(state, (*state).skip) == -(1 as c_int) {
            return ::core::ptr::null_mut::<c_char>();
        }
    }
    str = buf;
    left = (len as c_uint).wrapping_sub(1 as c_uint);
    if left != 0 {
        loop {
            if (*state).x.have == 0 as c_uint
                && gz_fetch(state) == -(1 as c_int)
            {
                return ::core::ptr::null_mut::<c_char>();
            }
            if (*state).x.have == 0 as c_uint {
                (*state).past = 1 as c_int;
                break;
            } else {
                n = if (*state).x.have > left {
                    left
                } else {
                    (*state).x.have
                };
                eol = memchr(
                    (*state).x.next as *const c_void,
                    '\n' as i32,
                    n as size_t,
                ) as *mut c_uchar;
                if !eol.is_null() {
                    n = (eol.offset_from((*state).x.next) as c_long
                        as c_uint)
                        .wrapping_add(1 as c_uint);
                }
                memcpy(
                    buf as *mut c_void,
                    (*state).x.next as *const c_void,
                    n as size_t,
                );
                (*state).x.have = (*state).x.have.wrapping_sub(n);
                (*state).x.next = (*state).x.next.offset(n as isize);
                (*state).x.pos += n as c_long;
                left = left.wrapping_sub(n);
                buf = buf.offset(n as isize);
                if !(left != 0 && eol.is_null()) {
                    break;
                }
            }
        }
    }
    if buf == str {
        return ::core::ptr::null_mut::<c_char>();
    }
    *buf.offset(0 as c_int as isize) = 0 as c_char;
    return str;
}
#[inline]
pub fn gzdirect(mut file: gzFile) -> c_int { unsafe {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return 0 as c_int;
    }
    state = file as gz_statep;
    if (*state).mode == GZ_READ
        && (*state).how == LOOK
        && (*state).x.have == 0 as c_uint
    {
        gz_look(state);
    }
    return (*state).direct;
} }
#[inline]
pub fn gzclose_r(mut file: gzFile) -> c_int { unsafe {
    let mut ret: c_int = 0;
    let mut err: c_int = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ {
        return Z_STREAM_ERROR;
    }
    if (*state).size != 0 {
        inflateEnd(&raw mut (*state).strm);
        free((*state).out as *mut c_void);
        free((*state).in_0 as *mut c_void);
    }
    err = if (*state).err == Z_BUF_ERROR {
        Z_BUF_ERROR
    } else {
        Z_OK
    };
    gz_error(state, Z_OK, ::core::ptr::null::<c_char>());
    free((*state).path as *mut c_void);
    ret = close((*state).fd);
    free(state as *mut c_void);
    return if ret != 0 { Z_ERRNO } else { err };
} }

pub const COPY: c_int = 1 as c_int;
pub const GZIP: c_int = 2;

