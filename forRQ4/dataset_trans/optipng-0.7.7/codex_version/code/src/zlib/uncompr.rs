extern "C" {
    pub type internal_state;
    fn inflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn inflateInit_(
        strm: z_streamp,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type uLongf = uLong;
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
pub const ZLIB_VERSION: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"1.2.11-optipng\0") };
pub const Z_NO_FLUSH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Z_NEED_DICT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Z_DATA_ERROR: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const Z_BUF_ERROR: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn uncompress2(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: *mut uLong,
) -> ::core::ffi::c_int {
    let mut stream: z_stream = z_stream {
        next_in: ::core::ptr::null_mut::<Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: ::core::ptr::null_mut::<Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        state: ::core::ptr::null_mut::<internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut err: ::core::ffi::c_int = 0;
    let max: uInt = -(1 as ::core::ffi::c_int) as uInt;
    let mut len: uLong = 0;
    let mut left: uLong = 0;
    let mut buf: [Byte; 1] = [0; 1];
    len = *sourceLen;
    if *destLen != 0 {
        left = *destLen as uLong;
        *destLen = 0 as uLongf;
    } else {
        left = 1 as uLong;
        dest = &raw mut buf as *mut Byte as *mut Bytef;
    }
    stream.next_in = source as *mut Bytef;
    stream.avail_in = 0 as uInt;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = inflateInit_(
        &raw mut stream,
        ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
    );
    if err != Z_OK {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0 as uInt;
    loop {
        if stream.avail_out == 0 as ::core::ffi::c_uint {
            stream.avail_out = (if left > max as uLong {
                max as ::core::ffi::c_uint
            } else {
                left as ::core::ffi::c_uint
            }) as uInt;
            left = (left as ::core::ffi::c_ulong)
                .wrapping_sub(stream.avail_out as ::core::ffi::c_ulong) as uLong
                as uLong;
        }
        if stream.avail_in == 0 as ::core::ffi::c_uint {
            stream.avail_in = (if len > max as uLong {
                max as ::core::ffi::c_uint
            } else {
                len as ::core::ffi::c_uint
            }) as uInt;
            len = (len as ::core::ffi::c_ulong)
                .wrapping_sub(stream.avail_in as ::core::ffi::c_ulong) as uLong
                as uLong;
        }
        err = inflate(&raw mut stream, Z_NO_FLUSH);
        if !(err == Z_OK) {
            break;
        }
    }
    *sourceLen = (*sourceLen as ::core::ffi::c_ulong).wrapping_sub(
        (len as ::core::ffi::c_ulong).wrapping_add(stream.avail_in as ::core::ffi::c_ulong),
    ) as uLong as uLong;
    if dest != &raw mut buf as *mut Byte {
        *destLen = stream.total_out as uLongf;
    } else if stream.total_out != 0 && err == Z_BUF_ERROR {
        left = 1 as uLong;
    }
    inflateEnd(&raw mut stream);
    return if err == Z_STREAM_END {
        Z_OK
    } else if err == Z_NEED_DICT {
        Z_DATA_ERROR
    } else if err == Z_BUF_ERROR
        && (left as ::core::ffi::c_ulong).wrapping_add(stream.avail_out as ::core::ffi::c_ulong)
            != 0
    {
        Z_DATA_ERROR
    } else {
        err
    };
}
#[no_mangle]
pub unsafe extern "C" fn uncompress(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
) -> ::core::ffi::c_int {
    return uncompress2(dest, destLen, source, &raw mut sourceLen);
}
