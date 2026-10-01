extern "C" {
    pub type internal_state;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn deflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn deflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn deflateReset(strm: z_streamp) -> ::core::ffi::c_int;
    fn deflateParams(
        strm: z_streamp,
        level: ::core::ffi::c_int,
        strategy: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn deflateInit2_(
        strm: z_streamp,
        level: ::core::ffi::c_int,
        method: ::core::ffi::c_int,
        windowBits: ::core::ffi::c_int,
        memLevel: ::core::ffi::c_int,
        strategy: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn gz_error(_: gz_statep, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type va_list = __builtin_va_list;
pub type __off_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type z_size_t = size_t;
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type voidpc = *const ::core::ffi::c_void;
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
pub struct gzFile_s {
    pub have: ::core::ffi::c_uint,
    pub next: *mut ::core::ffi::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub x: gzFile_s,
    pub mode: ::core::ffi::c_int,
    pub fd: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub size: ::core::ffi::c_uint,
    pub want: ::core::ffi::c_uint,
    pub in_0: *mut ::core::ffi::c_uchar,
    pub out: *mut ::core::ffi::c_uchar,
    pub direct: ::core::ffi::c_int,
    pub how: ::core::ffi::c_int,
    pub start: off_t,
    pub eof: ::core::ffi::c_int,
    pub past: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub strategy: ::core::ffi::c_int,
    pub skip: off_t,
    pub seek: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
    pub msg: *mut ::core::ffi::c_char,
    pub strm: z_stream,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ZLIB_VERSION: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"1.2.11-optipng\0") };
pub const Z_NO_FLUSH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_FINISH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Z_BLOCK: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Z_ERRNO: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const Z_STREAM_ERROR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const Z_DATA_ERROR: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const Z_MEM_ERROR: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const Z_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn gz_init(mut state: gz_statep) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut strm: z_streamp = &raw mut (*state).strm;
    (*state).in_0 =
        malloc(((*state).want << 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_uchar;
    if (*state).in_0.is_null() {
        gz_error(
            state,
            Z_MEM_ERROR,
            b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*state).direct == 0 {
        (*state).out = malloc((*state).want as size_t) as *mut ::core::ffi::c_uchar;
        if (*state).out.is_null() {
            free((*state).in_0 as *mut ::core::ffi::c_void);
            gz_error(
                state,
                Z_MEM_ERROR,
                b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*strm).zalloc = None;
        (*strm).zfree = None;
        (*strm).opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        ret = deflateInit2_(
            strm,
            (*state).level,
            8 as ::core::ffi::c_int,
            15 as ::core::ffi::c_int + 16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (*state).strategy,
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
        );
        if ret != Z_OK {
            free((*state).out as *mut ::core::ffi::c_void);
            free((*state).in_0 as *mut ::core::ffi::c_void);
            gz_error(
                state,
                Z_MEM_ERROR,
                b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*strm).next_in = ::core::ptr::null_mut::<Bytef>();
    }
    (*state).size = (*state).want;
    if (*state).direct == 0 {
        (*strm).avail_out = (*state).size as uInt;
        (*strm).next_out = (*state).out as *mut Bytef;
        (*state).x.next = (*strm).next_out as *mut ::core::ffi::c_uchar;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn gz_comp(
    mut state: gz_statep,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut writ: ::core::ffi::c_int = 0;
    let mut have: ::core::ffi::c_uint = 0;
    let mut put: ::core::ffi::c_uint = 0;
    let mut max: ::core::ffi::c_uint = (-(1 as ::core::ffi::c_int) as ::core::ffi::c_uint
        >> 2 as ::core::ffi::c_int)
        .wrapping_add(1 as ::core::ffi::c_uint);
    let mut strm: z_streamp = &raw mut (*state).strm;
    if (*state).size == 0 as ::core::ffi::c_uint && gz_init(state) == -(1 as ::core::ffi::c_int) {
        return -(1 as ::core::ffi::c_int);
    }
    if (*state).direct != 0 {
        while (*strm).avail_in != 0 {
            put = if (*strm).avail_in > max {
                max
            } else {
                (*strm).avail_in as ::core::ffi::c_uint
            };
            writ = write(
                (*state).fd,
                (*strm).next_in as *const ::core::ffi::c_void,
                put as size_t,
            ) as ::core::ffi::c_int;
            if writ < 0 as ::core::ffi::c_int {
                gz_error(state, Z_ERRNO, strerror(*__errno_location()));
                return -(1 as ::core::ffi::c_int);
            }
            (*strm).avail_in = ((*strm).avail_in as ::core::ffi::c_uint)
                .wrapping_sub(writ as ::core::ffi::c_uint) as uInt
                as uInt;
            (*strm).next_in = (*strm).next_in.offset(writ as isize);
        }
        return 0 as ::core::ffi::c_int;
    }
    ret = Z_OK;
    loop {
        if (*strm).avail_out == 0 as ::core::ffi::c_uint
            || flush != Z_NO_FLUSH && (flush != Z_FINISH || ret == Z_STREAM_END)
        {
            while (*strm).next_out > (*state).x.next {
                put = if (*strm).next_out.offset_from((*state).x.next) as ::core::ffi::c_long
                    > max as ::core::ffi::c_int as ::core::ffi::c_long
                {
                    max
                } else {
                    (*strm).next_out.offset_from((*state).x.next) as ::core::ffi::c_long
                        as ::core::ffi::c_uint
                };
                writ = write(
                    (*state).fd,
                    (*state).x.next as *const ::core::ffi::c_void,
                    put as size_t,
                ) as ::core::ffi::c_int;
                if writ < 0 as ::core::ffi::c_int {
                    gz_error(state, Z_ERRNO, strerror(*__errno_location()));
                    return -(1 as ::core::ffi::c_int);
                }
                (*state).x.next = (*state).x.next.offset(writ as isize);
            }
            if (*strm).avail_out == 0 as ::core::ffi::c_uint {
                (*strm).avail_out = (*state).size as uInt;
                (*strm).next_out = (*state).out as *mut Bytef;
                (*state).x.next = (*state).out;
            }
        }
        have = (*strm).avail_out as ::core::ffi::c_uint;
        ret = deflate(strm, flush);
        if ret == Z_STREAM_ERROR {
            gz_error(
                state,
                Z_STREAM_ERROR,
                b"internal error: deflate stream corrupt\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        have = have.wrapping_sub((*strm).avail_out as ::core::ffi::c_uint);
        if !(have != 0) {
            break;
        }
    }
    if flush == Z_FINISH {
        deflateReset(strm);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn gz_zero(mut state: gz_statep, mut len: off_t) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut strm: z_streamp = &raw mut (*state).strm;
    if (*strm).avail_in != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as ::core::ffi::c_int) {
        return -(1 as ::core::ffi::c_int);
    }
    first = 1 as ::core::ffi::c_int;
    while len != 0 {
        n = if ::core::mem::size_of::<::core::ffi::c_int>() as usize
            == ::core::mem::size_of::<off_t>() as usize
            && (*state).size > INT_MAX as ::core::ffi::c_uint
            || (*state).size as off_t > len
        {
            len as ::core::ffi::c_uint
        } else {
            (*state).size
        };
        if first != 0 {
            memset(
                (*state).in_0 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                n as size_t,
            );
            first = 0 as ::core::ffi::c_int;
        }
        (*strm).avail_in = n as uInt;
        (*strm).next_in = (*state).in_0 as *mut Bytef;
        (*state).x.pos += n as ::core::ffi::c_long;
        if gz_comp(state, Z_NO_FLUSH) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
        len -= n as ::core::ffi::c_long;
    }
    return 0 as ::core::ffi::c_int;
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
    if (*state).size == 0 as ::core::ffi::c_uint && gz_init(state) == -(1 as ::core::ffi::c_int) {
        return 0 as z_size_t;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as ::core::ffi::c_int;
        if gz_zero(state, (*state).skip) == -(1 as ::core::ffi::c_int) {
            return 0 as z_size_t;
        }
    }
    if len < (*state).size as z_size_t {
        loop {
            let mut have: ::core::ffi::c_uint = 0;
            let mut copy: ::core::ffi::c_uint = 0;
            if (*state).strm.avail_in == 0 as ::core::ffi::c_uint {
                (*state).strm.next_in = (*state).in_0 as *mut Bytef;
            }
            have = (*state)
                .strm
                .next_in
                .offset((*state).strm.avail_in as isize)
                .offset_from((*state).in_0) as ::core::ffi::c_long
                as ::core::ffi::c_uint;
            copy = (*state).size.wrapping_sub(have);
            if copy as z_size_t > len {
                copy = len as ::core::ffi::c_uint;
            }
            memcpy(
                (*state).in_0.offset(have as isize) as *mut ::core::ffi::c_void,
                buf as *const ::core::ffi::c_void,
                copy as size_t,
            );
            (*state).strm.avail_in =
                ((*state).strm.avail_in as ::core::ffi::c_uint).wrapping_add(copy) as uInt as uInt;
            (*state).x.pos += copy as ::core::ffi::c_long;
            buf = (buf as *const ::core::ffi::c_char).offset(copy as isize) as voidpc;
            len = (len as ::core::ffi::c_ulong).wrapping_sub(copy as ::core::ffi::c_ulong)
                as z_size_t as z_size_t;
            if len != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as ::core::ffi::c_int) {
                return 0 as z_size_t;
            }
            if !(len != 0) {
                break;
            }
        }
    } else {
        if (*state).strm.avail_in != 0 && gz_comp(state, Z_NO_FLUSH) == -(1 as ::core::ffi::c_int) {
            return 0 as z_size_t;
        }
        (*state).strm.next_in = buf as *mut Bytef;
        loop {
            let mut n: ::core::ffi::c_uint = -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if n as z_size_t > len {
                n = len as ::core::ffi::c_uint;
            }
            (*state).strm.avail_in = n as uInt;
            (*state).x.pos += n as ::core::ffi::c_long;
            if gz_comp(state, Z_NO_FLUSH) == -(1 as ::core::ffi::c_int) {
                return 0 as z_size_t;
            }
            len = (len as ::core::ffi::c_ulong).wrapping_sub(n as ::core::ffi::c_ulong) as z_size_t
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
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return 0 as ::core::ffi::c_int;
    }
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        gz_error(
            state,
            Z_DATA_ERROR,
            b"requested length does not fit in int\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return gz_write(state, buf, len as z_size_t) as ::core::ffi::c_int;
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
            b"request does not fit in a size_t\0" as *const u8 as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn gzputc(mut file: gzFile, mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut have: ::core::ffi::c_uint = 0;
    let mut buf: [::core::ffi::c_uchar; 1] = [0; 1];
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    let mut strm: z_streamp = ::core::ptr::null_mut::<z_stream>();
    if file.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    state = file as gz_statep;
    strm = &raw mut (*state).strm as z_streamp;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return -(1 as ::core::ffi::c_int);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as ::core::ffi::c_int;
        if gz_zero(state, (*state).skip) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*state).size != 0 {
        if (*strm).avail_in == 0 as ::core::ffi::c_uint {
            (*strm).next_in = (*state).in_0 as *mut Bytef;
        }
        have = (*strm)
            .next_in
            .offset((*strm).avail_in as isize)
            .offset_from((*state).in_0) as ::core::ffi::c_long
            as ::core::ffi::c_uint;
        if have < (*state).size {
            *(*state).in_0.offset(have as isize) = c as ::core::ffi::c_uchar;
            (*strm).avail_in = (*strm).avail_in.wrapping_add(1);
            (*state).x.pos += 1;
            return c & 0xff as ::core::ffi::c_int;
        }
    }
    buf[0 as ::core::ffi::c_int as usize] = c as ::core::ffi::c_uchar;
    if gz_write(
        state,
        &raw mut buf as *mut ::core::ffi::c_uchar as voidpc,
        1 as z_size_t,
    ) != 1 as z_size_t
    {
        return -(1 as ::core::ffi::c_int);
    }
    return c & 0xff as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gzputs(
    mut file: gzFile,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut len: z_size_t = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return -(1 as ::core::ffi::c_int);
    }
    len = strlen(str) as z_size_t;
    ret = gz_write(state, str as voidpc, len) as ::core::ffi::c_int;
    return if ret == 0 as ::core::ffi::c_int && len != 0 as z_size_t {
        -(1 as ::core::ffi::c_int)
    } else {
        ret
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzvprintf(
    mut file: gzFile,
    mut format: *const ::core::ffi::c_char,
    mut va: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut left: ::core::ffi::c_uint = 0;
    let mut next: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    if (*state).size == 0 as ::core::ffi::c_uint && gz_init(state) == -(1 as ::core::ffi::c_int) {
        return (*state).err;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as ::core::ffi::c_int;
        if gz_zero(state, (*state).skip) == -(1 as ::core::ffi::c_int) {
            return (*state).err;
        }
    }
    if (*strm).avail_in == 0 as ::core::ffi::c_uint {
        (*strm).next_in = (*state).in_0 as *mut Bytef;
    }
    next = (*state)
        .in_0
        .offset((*strm).next_in.offset_from((*state).in_0) as ::core::ffi::c_long as isize)
        .offset((*strm).avail_in as isize) as *mut ::core::ffi::c_char;
    *next.offset((*state).size.wrapping_sub(1 as ::core::ffi::c_uint) as isize) =
        0 as ::core::ffi::c_char;
    len = vsnprintf(next, (*state).size as size_t, format, va.as_va_list());
    if len == 0 as ::core::ffi::c_int
        || len as ::core::ffi::c_uint >= (*state).size
        || *next.offset((*state).size.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    (*strm).avail_in = ((*strm).avail_in as ::core::ffi::c_uint)
        .wrapping_add(len as ::core::ffi::c_uint) as uInt as uInt;
    (*state).x.pos += len as ::core::ffi::c_long;
    if (*strm).avail_in >= (*state).size {
        left = ((*strm).avail_in as ::core::ffi::c_uint).wrapping_sub((*state).size);
        (*strm).avail_in = (*state).size as uInt;
        if gz_comp(state, Z_NO_FLUSH) == -(1 as ::core::ffi::c_int) {
            return (*state).err;
        }
        memcpy(
            (*state).in_0 as *mut ::core::ffi::c_void,
            (*state).in_0.offset((*state).size as isize) as *const ::core::ffi::c_void,
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
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut va: ::core::ffi::VaListImpl;
    let mut ret: ::core::ffi::c_int = 0;
    va = args.clone();
    ret = gzvprintf(file, format, va.as_va_list());
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn gzflush(
    mut file: gzFile,
    mut flush: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE || (*state).err != Z_OK {
        return Z_STREAM_ERROR;
    }
    if flush < 0 as ::core::ffi::c_int || flush > Z_FINISH {
        return Z_STREAM_ERROR;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as ::core::ffi::c_int;
        if gz_zero(state, (*state).skip) == -(1 as ::core::ffi::c_int) {
            return (*state).err;
        }
    }
    gz_comp(state, flush);
    return (*state).err;
}
#[no_mangle]
pub unsafe extern "C" fn gzsetparams(
    mut file: gzFile,
    mut level: ::core::ffi::c_int,
    mut strategy: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
        (*state).seek = 0 as ::core::ffi::c_int;
        if gz_zero(state, (*state).skip) == -(1 as ::core::ffi::c_int) {
            return (*state).err;
        }
    }
    if (*state).size != 0 {
        if (*strm).avail_in != 0 && gz_comp(state, Z_BLOCK) == -(1 as ::core::ffi::c_int) {
            return (*state).err;
        }
        deflateParams(strm, level, strategy);
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn gzclose_w(mut file: gzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = Z_OK;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return Z_STREAM_ERROR;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_WRITE {
        return Z_STREAM_ERROR;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as ::core::ffi::c_int;
        if gz_zero(state, (*state).skip) == -(1 as ::core::ffi::c_int) {
            ret = (*state).err;
        }
    }
    if gz_comp(state, Z_FINISH) == -(1 as ::core::ffi::c_int) {
        ret = (*state).err;
    }
    if (*state).size != 0 {
        if (*state).direct == 0 {
            deflateEnd(&raw mut (*state).strm);
            free((*state).out as *mut ::core::ffi::c_void);
        }
        free((*state).in_0 as *mut ::core::ffi::c_void);
    }
    gz_error(state, Z_OK, ::core::ptr::null::<::core::ffi::c_char>());
    free((*state).path as *mut ::core::ffi::c_void);
    if close((*state).fd) == -(1 as ::core::ffi::c_int) {
        ret = Z_ERRNO;
    }
    free(state as *mut ::core::ffi::c_void);
    return ret;
}
pub const GZ_WRITE: ::core::ffi::c_int = 31153 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
