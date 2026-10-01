#![feature(extern_types)]
                                                             
                                                         
                                                
                                                                   
                                                                
                                                              
mod libcall {
    #![allow(dead_code)]
    use core::ffi::{c_char, c_void};
    use brlib::src::dec::{decode, static_init as dec_si};
    use brlib::src::enc::{encode, static_init as enc_si};

    pub type EncState = *mut encode::BrotliEncoderState;
    pub type DecState = *mut decode::BrotliDecoderState;
    pub type PreparedDict = *mut encode::BrotliEncoderPreparedDictionary;
    pub type MetaStartFn = Option<unsafe extern "C" fn(*mut c_void, usize)>;
    pub type MetaChunkFn = Option<unsafe extern "C" fn(*mut c_void, *const u8, usize)>;

    // ---- static init (c2rust de-globalization shim; C tree has it too) ----
    #[inline] pub unsafe fn encoder_ensure_static_init() -> i32 { enc_si::BrotliEncoderEnsureStaticInit() }
    #[inline] pub unsafe fn decoder_ensure_static_init() -> i32 { dec_si::BrotliDecoderEnsureStaticInit() }

    // ---- encoder: one-shot ----
    #[inline] pub fn encoder_max_compressed_size(n: usize) -> usize {
        unsafe { encode::BrotliEncoderMaxCompressedSize(n) }
    }
    #[inline] pub unsafe fn encoder_compress(quality: i32, lgwin: i32, mode: u32, n: usize,
                                             src: *const u8, outlen: *mut usize, out: *mut u8) -> i32 {
        encode::BrotliEncoderCompress(quality, lgwin, mode, n, src, outlen, out)
    }

    // ---- encoder: streaming handle ----
    #[inline] pub unsafe fn encoder_create_instance() -> EncState {
        encode::BrotliEncoderCreateInstance(None, None, core::ptr::null_mut())
    }
    #[inline] pub unsafe fn encoder_destroy_instance(s: EncState) { encode::BrotliEncoderDestroyInstance(s) }
    #[inline] pub unsafe fn encoder_set_parameter(s: EncState, p: u32, v: u32) -> i32 {
        encode::BrotliEncoderSetParameter(s, p, v)
    }
    #[inline] pub unsafe fn encoder_compress_stream(s: EncState, op: u32,
                                                    avail_in: *mut usize, next_in: *mut *const u8,
                                                    avail_out: *mut usize, next_out: *mut *mut u8,
                                                    total_out: *mut usize) -> i32 {
        encode::BrotliEncoderCompressStream(s, op, avail_in, next_in, avail_out, next_out, total_out)
    }
    #[inline] pub unsafe fn encoder_has_more_output(s: EncState) -> i32 { encode::BrotliEncoderHasMoreOutput(s) }
    #[inline] pub unsafe fn encoder_take_output(s: EncState, size: *mut usize) -> *const u8 {
        encode::BrotliEncoderTakeOutput(s, size)
    }
    #[inline] pub unsafe fn encoder_is_finished(s: EncState) -> i32 { encode::BrotliEncoderIsFinished(s) }

    // ---- encoder: shared raw dictionary ----
    #[inline] pub unsafe fn encoder_prepare_dictionary(ty: u32, size: usize, data: *const u8,
                                                       quality: i32) -> PreparedDict {
        encode::BrotliEncoderPrepareDictionary(ty, size, data, quality, None, None, core::ptr::null_mut())
    }
    #[inline] pub unsafe fn encoder_get_prepared_dictionary_size(pd: PreparedDict) -> usize {
        encode::BrotliEncoderGetPreparedDictionarySize(pd)
    }
    #[inline] pub unsafe fn encoder_attach_prepared_dictionary(s: EncState, pd: PreparedDict) -> i32 {
        encode::BrotliEncoderAttachPreparedDictionary(s, pd)
    }
    #[inline] pub unsafe fn encoder_destroy_prepared_dictionary(pd: PreparedDict) {
        encode::BrotliEncoderDestroyPreparedDictionary(pd)
    }

    // ---- decoder ----
    #[inline] pub unsafe fn decoder_decompress(enc_n: usize, enc_buf: *const u8,
                                               dec_n: *mut usize, dec_buf: *mut u8) -> u32 {
        decode::BrotliDecoderDecompress(enc_n, enc_buf, dec_n, dec_buf)
    }
    #[inline] pub unsafe fn decoder_create_instance() -> DecState {
        decode::BrotliDecoderCreateInstance(None, None, core::ptr::null_mut())
    }
    #[inline] pub unsafe fn decoder_destroy_instance(s: DecState) { decode::BrotliDecoderDestroyInstance(s) }
    #[inline] pub unsafe fn decoder_set_parameter(s: DecState, p: u32, v: u32) -> i32 {
        decode::BrotliDecoderSetParameter(s, p, v)
    }
    #[inline] pub unsafe fn decoder_set_metadata_callbacks(s: DecState, start: MetaStartFn,
                                                           chunk: MetaChunkFn, opaque: *mut c_void) {
        decode::BrotliDecoderSetMetadataCallbacks(s, start, chunk, opaque)
    }
    #[inline] pub unsafe fn decoder_decompress_stream(s: DecState,
                                                      avail_in: *mut usize, next_in: *mut *const u8,
                                                      avail_out: *mut usize, next_out: *mut *mut u8,
                                                      total_out: *mut usize) -> u32 {
        decode::BrotliDecoderDecompressStream(s, avail_in, next_in, avail_out, next_out, total_out)
    }
    #[inline] pub unsafe fn decoder_has_more_output(s: DecState) -> i32 { decode::BrotliDecoderHasMoreOutput(s) }
    #[inline] pub unsafe fn decoder_take_output(s: DecState, size: *mut usize) -> *const u8 {
        decode::BrotliDecoderTakeOutput(s, size)
    }
    #[inline] pub unsafe fn decoder_is_used(s: DecState) -> i32 { decode::BrotliDecoderIsUsed(s) }
    #[inline] pub unsafe fn decoder_is_finished(s: DecState) -> i32 { decode::BrotliDecoderIsFinished(s) }
    #[inline] pub unsafe fn decoder_get_error_code(s: DecState) -> i32 { decode::BrotliDecoderGetErrorCode(s) }
    #[inline] pub unsafe fn decoder_error_string(c: i32) -> *const c_char { decode::BrotliDecoderErrorString(c) }
    #[inline] pub unsafe fn decoder_attach_dictionary(s: DecState, ty: u32, size: usize,
                                                      data: *const u8) -> i32 {
        decode::BrotliDecoderAttachDictionary(s, ty, size, data)
    }
}

include!("../../driver.rs");
