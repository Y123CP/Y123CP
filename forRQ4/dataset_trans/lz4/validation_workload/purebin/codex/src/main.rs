#![feature(extern_types)]
                                                             
                                                          
                                                
                                                                 
                                                                  
mod libcall {
    use core::ffi::{c_char, c_int, c_void};
    use lz4lib::src::{lz4, lz4hc};

    pub type Stream = *mut lz4::LZ4_stream_t;
    pub type StreamHC = *mut lz4hc::LZ4_streamHC_t;
    pub type StreamDecode = *mut lz4::LZ4_streamDecode_t;

    #[inline] pub unsafe fn version_number() -> i32 { lz4::LZ4_versionNumber() }
    #[inline] pub unsafe fn compress_bound(n: i32) -> i32 { lz4::LZ4_compressBound(n) }
    #[inline] pub unsafe fn sizeof_state() -> i32 { lz4::LZ4_sizeofState() }
    #[inline] pub unsafe fn sizeof_state_hc() -> i32 { lz4hc::LZ4_sizeofStateHC() }

    #[inline] pub unsafe fn compress_default(src: *const u8, dst: *mut u8, n: i32, cap: i32) -> i32 {
        lz4::LZ4_compress_default(src as *const c_char, dst as *mut c_char, n, cap)
    }
    #[inline] pub unsafe fn compress_hc(src: *const u8, dst: *mut u8, n: i32, cap: i32, level: i32) -> i32 {
        lz4hc::LZ4_compress_HC(src as *const c_char, dst as *mut c_char, n, cap, level)
    }
    #[inline] pub unsafe fn decompress_safe(src: *const u8, dst: *mut u8, comp: i32, cap: i32) -> i32 {
        lz4::LZ4_decompress_safe(src as *const c_char, dst as *mut c_char, comp, cap)
    }
    #[inline] pub unsafe fn decompress_safe_partial(src: *const u8, dst: *mut u8, comp: i32, target: i32, cap: i32) -> i32 {
        lz4::LZ4_decompress_safe_partial(src as *const c_char, dst as *mut c_char, comp, target, cap)
    }
    #[inline] pub unsafe fn decompress_safe_using_dict(src: *const u8, dst: *mut u8, comp: i32, cap: i32, dict: *const u8, dict_n: i32) -> i32 {
        lz4::LZ4_decompress_safe_usingDict(src as *const c_char, dst as *mut c_char, comp, cap, dict as *const c_char, dict_n)
    }
    #[inline] pub unsafe fn decompress_safe_partial_using_dict(src: *const u8, dst: *mut u8, comp: i32, target: i32, max_out: i32, dict: *const u8, dict_n: i32) -> i32 {
        lz4::LZ4_decompress_safe_partial_usingDict(src as *const c_char, dst as *mut c_char, comp, target, max_out, dict as *const c_char, dict_n)
    }
    #[inline] pub unsafe fn decompress_safe_continue(sd: StreamDecode, src: *const u8, dst: *mut u8, comp: i32, cap: i32) -> i32 {
        lz4::LZ4_decompress_safe_continue(sd, src as *const c_char, dst as *mut c_char, comp, cap)
    }

    // ---- fast stream ----
    #[inline] pub unsafe fn create_stream() -> Stream { lz4::LZ4_createStream() }
    #[inline] pub unsafe fn free_stream(s: Stream) -> i32 { lz4::LZ4_freeStream(s) }
    #[inline] pub unsafe fn load_dict(s: Stream, dict: *const u8, n: i32) -> i32 {
        lz4::LZ4_loadDict(s, dict as *const c_char, n)
    }
    #[inline] pub unsafe fn save_dict(s: Stream, buf: *mut u8, n: i32) -> i32 {
        lz4::LZ4_saveDict(s, buf as *mut c_char, n)
    }
    #[inline] pub unsafe fn reset_stream_fast(s: Stream) { lz4::LZ4_resetStream_fast(s) }
    #[inline] pub unsafe fn attach_dictionary(work: Stream, dict: Stream) { lz4::LZ4_attach_dictionary(work, dict) }
    #[inline] pub unsafe fn compress_fast_continue(s: Stream, src: *const u8, dst: *mut u8, n: i32, cap: i32, accel: i32) -> i32 {
        lz4::LZ4_compress_fast_continue(s, src as *const c_char, dst as *mut c_char, n, cap, accel)
    }
    #[inline] pub unsafe fn init_stream(buf: *mut u8, size: usize) -> Stream {
        lz4::LZ4_initStream(buf as *mut c_void, size)
    }
    #[inline] pub unsafe fn compress_fast_extstate_fastreset(state: *mut u8, src: *const u8, dst: *mut u8, n: i32, cap: i32, accel: i32) -> i32 {
        lz4::LZ4_compress_fast_extState_fastReset(state as *mut c_void, src as *const c_char, dst as *mut c_char, n, cap, accel)
    }

    // ---- HC stream ----
    #[inline] pub unsafe fn create_stream_hc() -> StreamHC { lz4hc::LZ4_createStreamHC() }
    #[inline] pub unsafe fn free_stream_hc(s: StreamHC) -> i32 { lz4hc::LZ4_freeStreamHC(s) }
    #[inline] pub unsafe fn load_dict_hc(s: StreamHC, dict: *const u8, n: i32) -> i32 {
        lz4hc::LZ4_loadDictHC(s, dict as *const c_char, n)
    }
    #[inline] pub unsafe fn save_dict_hc(s: StreamHC, buf: *mut u8, n: i32) -> i32 {
        lz4hc::LZ4_saveDictHC(s, buf as *mut c_char, n)
    }
    #[inline] pub unsafe fn reset_stream_hc_fast(s: StreamHC, level: i32) { lz4hc::LZ4_resetStreamHC_fast(s, level) }
    #[inline] pub unsafe fn attach_hc_dictionary(work: StreamHC, dict: StreamHC) { lz4hc::LZ4_attach_HC_dictionary(work, dict) }
    #[inline] pub unsafe fn compress_hc_continue(s: StreamHC, src: *const u8, dst: *mut u8, n: i32, cap: i32) -> i32 {
        lz4hc::LZ4_compress_HC_continue(s, src as *const c_char, dst as *mut c_char, n, cap)
    }

    // ---- destSize variants ----
    #[inline] pub unsafe fn compress_destsize(src: *const u8, dst: *mut u8, src_size: *mut i32, target: i32) -> i32 {
        lz4::LZ4_compress_destSize(src as *const c_char, dst as *mut c_char, src_size as *mut c_int, target)
    }
    #[inline] pub unsafe fn compress_destsize_extstate(state: *mut u8, src: *const u8, dst: *mut u8, src_size: *mut i32, target: i32, accel: i32) -> i32 {
        lz4::LZ4_compress_destSize_extState(state as *mut c_void, src as *const c_char, dst as *mut c_char, src_size as *mut c_int, target, accel)
    }
    #[inline] pub unsafe fn compress_hc_destsize(state: *mut u8, src: *const u8, dst: *mut u8, src_size: *mut i32, target: i32, level: i32) -> i32 {
        lz4hc::LZ4_compress_HC_destSize(state as *mut c_void, src as *const c_char, dst as *mut c_char, src_size as *mut c_int, target, level)
    }

    // ---- decode stream ----
    #[inline] pub unsafe fn create_stream_decode() -> StreamDecode { lz4::LZ4_createStreamDecode() }
    #[inline] pub unsafe fn free_stream_decode(sd: StreamDecode) -> i32 { lz4::LZ4_freeStreamDecode(sd) }
    #[inline] pub unsafe fn set_stream_decode(sd: StreamDecode, dict: *const u8, n: i32) -> i32 {
        lz4::LZ4_setStreamDecode(sd, dict as *const c_char, n)
    }
}

include!("../../driver.rs");
