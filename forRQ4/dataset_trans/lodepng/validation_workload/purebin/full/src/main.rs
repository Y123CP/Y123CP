                                                        
                                                     
                                                          
                                                          
                                                                 
                                                     
// LodePNGColorType/LodePNGFilterStrategy = c_uint = u32。
                                                           
                                                    
mod libcall {
    pub use pnglib::src::lodepng::{
        // decode / encode / state
        lodepng_decode32, lodepng_decode, lodepng_decode32_file,
        lodepng_encode32, lodepng_encode,
        lodepng_state_init, lodepng_state_cleanup, lodepng_state_copy,
        lodepng_inspect, lodepng_inspect_chunk, lodepng_get_raw_size,
        // chunk utilities
        lodepng_chunk_length, lodepng_chunk_type, lodepng_chunk_ancillary,
        lodepng_chunk_check_crc, lodepng_chunk_next_const, lodepng_chunk_find_const,
        lodepng_chunk_private, lodepng_chunk_safetocopy, lodepng_chunk_create,
        lodepng_chunk_type_equals, lodepng_chunk_data, lodepng_chunk_find,
        lodepng_chunk_next, lodepng_chunk_generate_crc, lodepng_chunk_append,
        lodepng_crc32,
        // color mode / info helpers
        lodepng_color_mode_make, lodepng_color_mode_init, lodepng_color_mode_copy,
        lodepng_color_mode_cleanup, lodepng_palette_add, lodepng_get_bpp,
        lodepng_get_channels, lodepng_is_palette_type, lodepng_has_palette_alpha,
        lodepng_is_alpha_type, lodepng_is_greyscale_type, lodepng_can_have_alpha,
        lodepng_error_text, lodepng_add_text, lodepng_add_itext,
        lodepng_clear_text, lodepng_clear_itext,
        // zlib
        lodepng_compress_settings_init, lodepng_decompress_settings_init,
        lodepng_zlib_compress, lodepng_zlib_decompress, lodepng_inflate,
        // convert
        lodepng_convert,
        // types + enum consts
        LodePNGState, LodePNGColorMode, LodePNGCompressSettings, LodePNGDecompressSettings,
        LCT_PALETTE, LCT_RGBA, LCT_RGB, LCT_GREY, LFS_MINSUM,
    };

                                                                 
    extern "C" {
        pub fn malloc(n: usize) -> *mut core::ffi::c_void;
        pub fn free(p: *mut core::ffi::c_void);
    }
}

include!("../../driver.rs");
