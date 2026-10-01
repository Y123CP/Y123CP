                                                             
                                                                     
                                                                       
                                               
mod libcall {
    use core::ffi::c_void;
    use zopflilib::src::{util, zopfli_lib};

    pub type Options = zopfli_lib::ZopfliOptions;
    pub const FMT_GZIP: u32 = 0;
    pub const FMT_ZLIB: u32 = 1;
    pub const FMT_DEFLATE: u32 = 2;

    #[inline]
    pub unsafe fn init_options(o: *mut Options) {
        util::ZopfliInitOptions(o as *mut util::ZopfliOptions)
    }
    #[inline]
    pub unsafe fn compress(o: *const Options, fmt: u32, src: *const u8, n: usize,
                           out: *mut *mut u8, outsize: *mut usize) {
        zopfli_lib::ZopfliCompress(o, fmt, src, n,
                                   out as *mut *mut core::ffi::c_uchar,
                                   outsize as *mut zopfli_lib::size_t)
    }
    extern "C" {
        pub fn free(p: *mut c_void);
    }
}

include!("../../driver.rs");
