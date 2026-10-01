extern "C" {
    pub type internal_state;
    fn deflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn deflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn deflateInit_(
        strm: z_streamp,
        level: ::core::ffi::c_int,
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
pub const Z_FINISH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Z_DEFAULT_COMPRESSION: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn compress2(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
    mut level: ::core::ffi::c_int,
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
    let mut left: uLong = 0;
    left = *destLen as uLong;
    *destLen = 0 as uLongf;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
    err = deflateInit_(
        &raw mut stream,
        level,
        ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
    );
    if err != Z_OK {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0 as uInt;
    stream.next_in = source as *mut Bytef;
    stream.avail_in = 0 as uInt;
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
            stream.avail_in = (if sourceLen > max as uLong {
                max as ::core::ffi::c_uint
            } else {
                sourceLen as ::core::ffi::c_uint
            }) as uInt;
            sourceLen = (sourceLen as ::core::ffi::c_ulong)
                .wrapping_sub(stream.avail_in as ::core::ffi::c_ulong)
                as uLong as uLong;
        }
        err = deflate(
            &raw mut stream,
            if sourceLen != 0 { Z_NO_FLUSH } else { Z_FINISH },
        );
        if !(err == Z_OK) {
            break;
        }
    }
    *destLen = stream.total_out as uLongf;
    deflateEnd(&raw mut stream);
    return if err == Z_STREAM_END { Z_OK } else { err };
}
#[no_mangle]
pub unsafe extern "C" fn compress(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
) -> ::core::ffi::c_int {
    return compress2(dest, destLen, source, sourceLen, Z_DEFAULT_COMPRESSION);
}
#[no_mangle]
pub unsafe extern "C" fn compressBound(mut sourceLen: uLong) -> uLong {
    return sourceLen
        .wrapping_add(sourceLen >> 12 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 14 as ::core::ffi::c_int)
        .wrapping_add(sourceLen >> 25 as ::core::ffi::c_int)
        .wrapping_add(13 as uLong);
}
