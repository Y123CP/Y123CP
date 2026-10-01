use core::ffi::*;
use crate::src::deflate::ZopfliDeflate;
use crate::src::gzip_container::ZopfliGzipCompress;
use crate::src::zlib_container::ZopfliZlibCompress;
pub use crate::src::ffi::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

pub type ZopfliFormat = c_uint;
pub const ZOPFLI_FORMAT_DEFLATE: ZopfliFormat = 2;
pub const ZOPFLI_FORMAT_ZLIB: ZopfliFormat = 1;
pub const ZOPFLI_FORMAT_GZIP: ZopfliFormat = 0;
#[inline]
pub unsafe fn ZopfliCompress(
    mut options: *const ZopfliOptions,
    mut output_type: ZopfliFormat,
    mut in_0: *const c_uchar,
    mut insize: size_t,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    if output_type as c_uint
        == ZOPFLI_FORMAT_GZIP as c_int as c_uint
    {
        ZopfliGzipCompress(options, in_0, insize, out, outsize);
    } else if output_type as c_uint
        == ZOPFLI_FORMAT_ZLIB as c_int as c_uint
    {
        ZopfliZlibCompress(options, in_0, insize, out, outsize);
    } else if output_type as c_uint
        == ZOPFLI_FORMAT_DEFLATE as c_int as c_uint
    {
        let mut bp: c_uchar = 0 as c_uchar;
        ZopfliDeflate(
            options,
            2 as c_int,
            1 as c_int,
            in_0,
            insize,
            &raw mut bp,
            out,
            outsize,
        );
    }
}
