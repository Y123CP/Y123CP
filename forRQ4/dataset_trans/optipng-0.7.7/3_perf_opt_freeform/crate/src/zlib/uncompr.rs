use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;
pub use crate::src::zlib::deflate::internal_state;
extern "C" {
    fn inflate(strm: z_streamp, flush: c_int) -> c_int;
    fn inflateEnd(strm: z_streamp) -> c_int;
    fn inflateInit_(
        strm: z_streamp,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int;
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
pub const ZLIB_VERSION: [c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [c_char; 15]>(*b"1.2.11-optipng\0") };

pub const Z_OK: c_int = 0 as c_int;
pub const Z_STREAM_END: c_int = 1 as c_int;
pub const Z_NEED_DICT: c_int = 2 as c_int;
pub const Z_DATA_ERROR: c_int = -(3 as c_int);
pub const Z_BUF_ERROR: c_int = -(5 as c_int);
#[inline]
pub unsafe fn uncompress2(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: *mut uLong,
) -> c_int {
    let destLen_view: &mut uLongf = unsafe { &mut *destLen };
    let mut stream: z_stream = z_stream {
        next_in: ::core::ptr::null_mut::<Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<c_char>(),
        state: ::core::ptr::null_mut::<internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut err: c_int = 0;
    let max: uInt = -(1 as c_int) as uInt;
    let mut len: uLong = 0;
    let mut left: uLong = 0;
    let mut buf: [Byte; 1] = [0; 1];
    len = *sourceLen;
    if *destLen_view != 0 {
        left = *destLen_view as uLong;
        *destLen_view = 0 as uLongf;
    } else {
        left = 1 as uLong;
        dest = &raw mut buf as *mut Byte as *mut Bytef;
    }
    stream.next_in = source as *mut Bytef;
    stream.avail_in = 0 as uInt;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<c_void>();
    err = inflateInit_(
        &raw mut stream,
        ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<z_stream>() as c_int,
    );
    if err != Z_OK {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0 as uInt;
    loop {
        if stream.avail_out == 0 as c_uint {
            stream.avail_out = (if left > max as uLong {
                max as c_uint
            } else {
                left as c_uint
            }) as uInt;
            left = (left as c_ulong)
                .wrapping_sub(stream.avail_out as c_ulong) as uLong
                as uLong;
        }
        if stream.avail_in == 0 as c_uint {
            stream.avail_in = (if len > max as uLong {
                max as c_uint
            } else {
                len as c_uint
            }) as uInt;
            len = (len as c_ulong)
                .wrapping_sub(stream.avail_in as c_ulong) as uLong
                as uLong;
        }
        err = inflate(&raw mut stream, Z_NO_FLUSH);
        if !(err == Z_OK) {
            break;
        }
    }
    *sourceLen = (*sourceLen as c_ulong).wrapping_sub(
        (len as c_ulong).wrapping_add(stream.avail_in as c_ulong),
    ) as uLong as uLong;
    if dest != &raw mut buf as *mut Byte {
        *destLen_view = stream.total_out as uLongf;
    } else if stream.total_out != 0 && err == Z_BUF_ERROR {
        left = 1 as uLong;
    }
    inflateEnd(&raw mut stream);
    return if err == Z_STREAM_END {
        Z_OK
    } else if err == Z_NEED_DICT {
        Z_DATA_ERROR
    } else if err == Z_BUF_ERROR
        && (left as c_ulong).wrapping_add(stream.avail_out as c_ulong)
            != 0
    {
        Z_DATA_ERROR
    } else {
        err
    };
}
#[inline]
pub unsafe fn uncompress(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
) -> c_int {
    return uncompress2(dest, destLen, source, &raw mut sourceLen);
}
