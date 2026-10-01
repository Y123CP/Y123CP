use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type internal_state;
    fn vsnprintf(
        __s: *mut c_char,
        __maxlen: size_t,
        __format: *const c_char,
        __arg: ::core::ffi::VaList,
    ) -> c_int;
    fn deflate(strm: z_streamp, flush: c_int) -> c_int;
    fn deflateEnd(strm: z_streamp) -> c_int;
    fn deflateReset(strm: z_streamp) -> c_int;
    fn deflateParams(
        strm: z_streamp,
        level: c_int,
        strategy: c_int,
    ) -> c_int;
    fn deflateInit2_(
        strm: z_streamp,
        level: c_int,
        method: c_int,
        windowBits: c_int,
        memLevel: c_int,
        strategy: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
    fn gz_error(_: gz_statep, _: c_int, _: *const c_char);
}

pub type voidpc = *const c_void;

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
pub const Z_ERRNO: c_int = -(1 as c_int);

pub const Z_DATA_ERROR: c_int = -(3 as c_int);
pub const Z_MEM_ERROR: c_int = -(4 as c_int);

unsafe extern "C" fn gz_init(mut state: gz_statep) -> c_int {
    let mut ret: c_int = 0;
    let mut strm: z_streamp = &raw mut (*state).strm;
    (*state).in_0 =
        malloc(((*state).want << 1 as c_int) as size_t) as *mut c_uchar;
    if (*state).in_0.is_null() {
        gz_error(
            state,
            Z_MEM_ERROR,
            b"out of memory\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if (*state).direct == 0 {
        (*state).out = malloc((*state).want as size_t) as *mut c_uchar;
        if (*state).out.is_null() {
            free((*state).in_0 as *mut c_void);
            gz_error(
                state,
                Z_MEM_ERROR,
                b"out of memory\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*strm).zalloc = None;
        (*strm).zfree = None;
        (*strm).opaque = ::core::ptr::null_mut::<c_void>();
        ret = deflateInit2_(
            strm,
            (*state).level,
            8 as c_int,
            15 as c_int + 16 as c_int,
            8 as c_int,
            (*state).strategy,
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as c_int,
        );
        if ret != Z_OK {
            free((*state).out as *mut c_void);
            free((*state).in_0 as *mut c_void);
            gz_error(
                state,
                Z_MEM_ERROR,
                b"out of memory\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*strm).next_in = ::core::ptr::null_mut::<Bytef>();
    }
    (*state).size = (*state).want;
    if (*state).direct == 0 {
        (*strm).avail_out = (*state).size as uInt;
        (*strm).next_out = (*state).out as *mut Bytef;
        (*state).x.next = (*strm).next_out as *mut c_uchar;
    }
    return 0 as c_int;
}
unsafe extern "C" fn gz_comp(
    mut state: gz_statep,
    mut flush: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let mut writ: c_int = 0;
    let mut have: c_uint = 0;
    let mut put: c_uint = 0;
    let mut max: c_uint = (-(1 as c_int) as c_uint
        >> 2 as c_int)
        .wrapping_add(1 as c_uint);
    let mut strm: z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as c_uint && gz_init(state) == -(1 as c_int) {
        return -(1 as c_int);
    }
    if (*state).direct != 0 {
        while (*strm).avail_in != 0 {
            put = if (*strm).avail_in > max {
                max
            } else {
                (*strm).avail_in as c_uint
            };
            writ = write(
                (*state).fd,
                (*strm).next_in as *const c_void,
                put as size_t,
            ) as c_int;
            if writ < 0 as c_int {
                gz_error(state, Z_ERRNO, strerror(*__errno_location()));
                return -(1 as c_int);
            }
            (*strm).avail_in = ((*strm).avail_in as c_uint)
                .wrapping_sub(writ as c_uint) as uInt
                as uInt;
            (*strm).next_in = (*strm).next_in.offset(writ as isize);
        }
        return 0 as c_int;
    }
    ret = Z_OK;
    loop {
        if (*strm).avail_out == 0 as c_uint
            || flush != Z_NO_FLUSH && (flush != Z_FINISH || ret == Z_STREAM_END)
        {
            while (*strm).next_out > (*state).x.next {
                put = if (*strm).next_out.offset_from((*state).x.next) as c_long
                    > max as c_int as c_long
                {
                    max
                } else {
                    (*strm).next_out.offset_from((*state).x.next) as c_long
                        as c_uint
                };
                writ = write(
                    (*state).fd,
                    (*state).x.next as *const c_void,
                    put as size_t,
                ) as c_int;
                if writ < 0 as c_int {
                    gz_error(state, Z_ERRNO, strerror(*__errno_location()));
                    return -(1 as c_int);
                }
                (*state).x.next = (*state).x.next.offset(writ as isize);
            }
            if (*strm).avail_out == 0 as c_uint {
                (*strm).avail_out = (*state).size as uInt;
                (*strm).next_out = (*state).out as *mut Bytef;
                (*state).x.next = (*state).out;
            }
        }
        have = (*strm).avail_out as c_uint;
        ret = deflate(strm, flush);
        if ret == Z_STREAM_ERROR {
            gz_error(
                state,
                Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0" as *const u8
                    as *const c_char,
            );
            return -(1 as c_int);
        }
        have = have.wrapping_sub((*strm).avail_out as c_uint);
        if !(have != 0) {
            break;
        }
    }
    if flush == Z_FINISH {
        deflateReset(strm);
    }
    return 0 as c_int;
}
unsafe extern "C" fn gz_zero(mut state: gz_statep, mut len: off_t) -> c_int {
    let mut first: c_int = 0;
    let mut n: c_uint = 0;
    let mut strm: z_streamp = &raw mut (*state).strm;
    if (*strm).avail_in != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as c_int) {
        return -(1 as c_int);
    }
    first = 1 as c_int;
    while len != 0 {
        n = if ::core::mem::size_of::<c_int>() as usize
            == ::core::mem::size_of::<off_t>() as usize
            && (*state).size > INT_MAX as c_uint
            || (*state).size as off_t > len
        {
            len as c_uint
        } else {
            (*state).size
        };
        if first != 0 {
            memset(
                (*state).in_0 as *mut c_void,
                0 as c_int,
                n as size_t,
            );
            first = 0 as c_int;
        }
        (*strm).avail_in = n as uInt;
        (*strm).next_in = (*state).in_0 as *mut Bytef;
        (*state).x.pos += n as c_long;
        if gz_comp(state, Z_NO_FLUSH) == -(1 as c_int) {
            return -(1 as c_int);
        }
        len -= n as c_long;
    }
    return 0 as c_int;
}
unsafe extern "C" fn gz_write(
    mut state: gz_statep,
    mut buf: voidpc,
    mut len: z_size_t,
) -> z_size_t {
    let mut put: z_size_t = len;
    if len == 0 as z_size_t {
        return 0 as z_size_t;
    }
    if (*state).size == 0 as c_uint && gz_init(state) == -(1 as c_int) {
        return 0 as z_size_t;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_zero(state, (*state).skip) == -(1 as c_int) {
            return 0 as z_size_t;
        }
    }
    if len < (*state).size as z_size_t {
        loop {
            let mut have: c_uint = 0;
            let mut copy: c_uint = 0;
            if (*state).strm.avail_in == 0 as c_uint {
                (*state).strm.next_in = (*state).in_0 as *mut Bytef;
            }
            have = (*state)
                .strm
                .next_in
                .offset((*state).strm.avail_in as isize)
                .offset_from((*state).in_0) as c_long
                as c_uint;
            copy = (*state).size.wrapping_sub(have);
            if copy as z_size_t > len {
                copy = len as c_uint;
            }
            memcpy(
                (*state).in_0.offset(have as isize) as *mut c_void,
                buf as *const c_void,
                copy as size_t,
            );
            (*state).strm.avail_in =
                ((*state).strm.avail_in as c_uint).wrapping_add(copy) as uInt as uInt;
            (*state).x.pos += copy as c_long;
            buf = (buf as *const c_char).offset(copy as isize) as voidpc;
            len = (len as c_ulong).wrapping_sub(copy as c_ulong)
                as z_size_t as z_size_t;
            if len != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as c_int) {
                return 0 as z_size_t;
            }
            if !(len != 0) {
                break;
            }
        }
    } else {
        if (*state).strm.avail_in != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as c_int) {
            return 0 as z_size_t;
        }
        (*state).strm.next_in = buf as *mut Bytef;
        loop {
            let mut n: c_uint = -(1 as c_int) as c_uint;
            if n as z_size_t > len {
                n = len as c_uint;
            }
            (*state).strm.avail_in = n as uInt;
            (*state).x.pos += n as c_long;
            if gz_comp(state, Z_NO_FLUSH) == -(1 as c_int) {
                return 0 as z_size_t;
            }
            len = (len as c_ulong).wrapping_sub(n as c_ulong) as z_size_t
                as z_size_t;
            if !(len != 0) {
                break;
            }
        }
    }
    return put;
}
#[no_mangle]
pub unsafe extern "C" fn gzwrite(
    mut file: gzFile,
    mut buf: voidpc,
    mut len: c_uint,
) -> c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return 0 as c_int;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return 0 as c_int;
    }
    if (len as c_int) < 0 as c_int {
        gz_error(
            state,
            Z_DATA_ERROR,
            b"requested length does not fit in int\0" as *const u8 as *const c_char,
        );
        return 0 as c_int;
    }
    return gz_write(state, buf, len as z_size_t) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gzfwrite(
    mut buf: voidpc,
    mut size: z_size_t,
    mut nitems: z_size_t,
    mut file: gzFile,
) -> z_size_t {
    let mut len: z_size_t = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return 0 as z_size_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
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
        gz_write(state, buf, len).wrapping_div(size)
    } else {
        0 as z_size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzputc(mut file: gzFile, mut c: c_int) -> c_int {
    let mut have: c_uint = 0;
    let mut buf: [c_uchar; 1] = [0; 1];
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    let mut strm: z_streamp = ::core::ptr::null_mut::<z_stream>();
    if file.is_null() {
        return -(1 as c_int);
    }
    state = file as gz_statep;
    strm = &raw mut (*state).strm as z_streamp;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return -(1 as c_int);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_zero(state, (*state).skip) == -(1 as c_int) {
            return -(1 as c_int);
        }
    }
    if (*state).size != 0 {
        if (*strm).avail_in == 0 as c_uint {
            (*strm).next_in = (*state).in_0 as *mut Bytef;
        }
        have = (*strm)
            .next_in
            .offset((*strm).avail_in as isize)
            .offset_from((*state).in_0) as c_long
            as c_uint;
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as c_uchar;
            (*strm).avail_in = (*strm).avail_in.wrapping_add(1);
            (*state).x.pos += 1;
            return c & 0xff as c_int;
        }
    }
    buf[0 as c_int as usize] = c as c_uchar;
    if gz_write(
        state,
        &raw mut buf as *mut c_uchar as voidpc,
        1 as z_size_t,
    ) != 1 as z_size_t
    {
        return -(1 as c_int);
    }
    return c & 0xff as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gzputs(
    mut file: gzFile,
    mut str: *const c_char,
) -> c_int {
    let mut ret: c_int = 0;
    let mut len: z_size_t = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return -(1 as c_int);
    }
    len = strlen(str) as z_size_t;
    ret = gz_write(state, str as voidpc, len) as c_int;
    return if ret == 0 as c_int && len != 0 as z_size_t {
        -(1 as c_int)
    } else {
        ret
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzvprintf(
    mut file: gzFile,
    mut format: *const c_char,
    mut va: ::core::ffi::VaList,
) -> c_int {
    let mut len: c_int = 0;
    let mut left: c_uint = 0;
    let mut next: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    let mut strm: z_streamp = ::core::ptr::null_mut::<z_stream>();
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    strm = &raw mut (*state).strm as z_streamp;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return Z_STREAM_ERROR;
    }
    if (*state).size == 0 as c_uint && gz_init(state) == -(1 as c_int) {
        return (*state).err;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_zero(state, (*state).skip) == -(1 as c_int) {
            return (*state).err;
        }
    }
    if (*strm).avail_in == 0 as c_uint {
        (*strm).next_in = (*state).in_0 as *mut Bytef;
    }
    next = (*state)
        .in_0
        .offset((*strm).next_in.offset_from((*state).in_0) as c_long as isize)
        .offset((*strm).avail_in as isize) as *mut c_char;
    *next.offset((*state).size.wrapping_sub(1 as c_uint) as isize) =
        0 as c_char;
    len = vsnprintf(next, (*state).size as size_t, format, va.as_va_list());
    if len == 0 as c_int
        || len as c_uint >= (*state).size
        || *next.offset((*state).size.wrapping_sub(1 as c_uint) as isize)
            as c_int
            != 0 as c_int
    {
        return 0 as c_int;
    }
    (*strm).avail_in = ((*strm).avail_in as c_uint)
        .wrapping_add(len as c_uint) as uInt as uInt;
    (*state).x.pos += len as c_long;
    if (*strm).avail_in >= (*state).size {
        left = ((*strm).avail_in as c_uint).wrapping_sub((*state).size);
        (*strm).avail_in = (*state).size as uInt;
        if gz_comp(state, Z_NO_FLUSH) == -(1 as c_int) {
            return (*state).err;
        }
        memcpy(
            (*state).in_0 as *mut c_void,
            (*state).in_0.offset((*state).size as isize) as *const c_void,
            left as size_t,
        );
        (*strm).next_in = (*state).in_0 as *mut Bytef;
        (*strm).avail_in = left as uInt;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn gzprintf(
    mut file: gzFile,
    mut format: *const c_char,
    mut args: ...
) -> c_int {
    let mut va: ::core::ffi::VaListImpl;
    let mut ret: c_int = 0;
    va = args.clone();
    ret = gzvprintf(file, format, va.as_va_list());
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn gzflush(
    mut file: gzFile,
    mut flush: c_int,
) -> c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return Z_STREAM_ERROR;
    }
    if flush < 0 as c_int || flush > Z_FINISH {
        return Z_STREAM_ERROR;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_zero(state, (*state).skip) == -(1 as c_int) {
            return (*state).err;
        }
    }
    gz_comp(state, flush);
    return (*state).err;
}
#[no_mangle]
pub unsafe extern "C" fn gzsetparams(
    mut file: gzFile,
    mut level: c_int,
    mut strategy: c_int,
) -> c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    let mut strm: z_streamp = ::core::ptr::null_mut::<z_stream>();
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    strm = &raw mut (*state).strm as z_streamp;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return Z_STREAM_ERROR;
    }
    if level == (*state).level && strategy == (*state).strategy {
        return Z_OK;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_zero(state, (*state).skip) == -(1 as c_int) {
            return (*state).err;
        }
    }
    if (*state).size != 0 {
        if (*strm).avail_in != 0 && gz_comp(state, Z_BLOCK) == -(1 as c_int) {
            return (*state).err;
        }
        deflateParams(strm, level, strategy);
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn gzclose_w(mut file: gzFile) -> c_int {
    let mut ret: c_int = Z_OK;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE {
        return Z_STREAM_ERROR;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as c_int;
        if gz_zero(state, (*state).skip) == -(1 as c_int) {
            ret = (*state).err;
        }
    }
    if gz_comp(state, Z_FINISH) == -(1 as c_int) {
        ret = (*state).err;
    }
    if (*state).size != 0 {
        if (*state).direct == 0 {
            deflateEnd(&raw mut (*state).strm);
            free((*state).out as *mut c_void);
        }
        free((*state).in_0 as *mut c_void);
    }
    gz_error(state, Z_OK, ::core::ptr::null::<c_char>());
    free((*state).path as *mut c_void);
    if close((*state).fd) == -(1 as c_int) {
        ret = Z_ERRNO;
    }
    free(state as *mut c_void);
    return ret;
}

