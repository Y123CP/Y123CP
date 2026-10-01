#![allow(unused_imports, unused_variables, clippy::missing_safety_doc)]

use std::fs;
use std::mem::MaybeUninit;
use std::ptr;

// LZ4 core
use lz4_raw::src::lz4::{
    LZ4_compress, LZ4_compressBound, LZ4_compress_default, LZ4_compress_destSize,
    LZ4_compress_destSize_extState, LZ4_compress_fast, LZ4_compress_fast_continue,
    LZ4_compress_fast_extState, LZ4_compress_fast_extState_fastReset,
    LZ4_compress_forceExtDict, LZ4_compress_limitedOutput,
    LZ4_compress_limitedOutput_continue, LZ4_compress_limitedOutput_withState,
    LZ4_compress_continue, LZ4_compress_withState,
    LZ4_create, LZ4_createStream, LZ4_createStreamDecode,
    LZ4_decompress_fast, LZ4_decompress_fast_continue, LZ4_decompress_fast_usingDict,
    LZ4_decompress_fast_withPrefix64k,
    LZ4_decompress_safe, LZ4_decompress_safe_continue,
    LZ4_decompress_safe_partial, LZ4_decompress_safe_usingDict,
    LZ4_decompress_safe_partial_usingDict,
    LZ4_decompress_safe_withPrefix64k,
    LZ4_decompress_safe_forceExtDict,
    LZ4_decompress_safe_partial_forceExtDict,
    LZ4_decoderRingBufferSize,
    LZ4_freeStream, LZ4_freeStreamDecode,
    LZ4_initStream, LZ4_loadDict, LZ4_loadDictSlow,
    LZ4_attach_dictionary,
    LZ4_resetStream, LZ4_resetStream_fast, LZ4_saveDict,
    LZ4_setStreamDecode,
    LZ4_sizeofState, LZ4_sizeofStreamState,
    LZ4_slideInputBuffer, LZ4_resetStreamState,
    LZ4_uncompress, LZ4_uncompress_unknownOutputSize,
    LZ4_versionNumber, LZ4_versionString,
    LZ4_stream_t, LZ4_streamDecode_t,
};

// LZ4 HC
use lz4_raw::src::lz4hc::{
    LZ4_compress_HC, LZ4_compress_HC_continue, LZ4_compress_HC_continue_destSize,
    LZ4_compress_HC_destSize, LZ4_compress_HC_extStateHC,
    LZ4_compress_HC_extStateHC_fastReset,
    LZ4_compressHC, LZ4_compressHC2, LZ4_compressHC2_limitedOutput,
    LZ4_compressHC_continue, LZ4_compressHC_limitedOutput,
    LZ4_compressHC_limitedOutput_continue, LZ4_compressHC_limitedOutput_withStateHC,
    LZ4_compressHC_withStateHC,
    LZ4_compressHC2_continue, LZ4_compressHC2_limitedOutput_continue,
    LZ4_compressHC2_withStateHC, LZ4_compressHC2_limitedOutput_withStateHC,
    LZ4_createHC, LZ4_createStreamHC, LZ4_freeStreamHC,
    LZ4_attach_HC_dictionary,
    LZ4_favorDecompressionSpeed, LZ4_initStreamHC,
    LZ4_loadDictHC, LZ4_resetStreamHC, LZ4_resetStreamHC_fast,
    LZ4_saveDictHC, LZ4_setCompressionLevel,
    LZ4_sizeofStateHC, LZ4_sizeofStreamStateHC,
    LZ4_freeHC, LZ4_resetStreamStateHC, LZ4_slideInputBufferHC,
    LZ4_streamHC_t,
};

// LZ4 Frame
use lz4_raw::src::lz4frame::{
    LZ4F_cctx, LZ4F_dctx,
    LZ4F_CDict, LZ4F_CustomMem,
    LZ4F_compressBegin, LZ4F_compressBegin_usingCDict, LZ4F_compressBegin_usingDict,
    LZ4F_compressBound, LZ4F_compressEnd, LZ4F_compressFrame,
    LZ4F_compressFrame_usingCDict, LZ4F_compressFrameBound,
    LZ4F_compressUpdate, LZ4F_uncompressedUpdate,
    LZ4F_compressionLevel_max,
    LZ4F_createCDict, LZ4F_createCDict_advanced,
    LZ4F_createCompressionContext, LZ4F_createCompressionContext_advanced,
    LZ4F_createDecompressionContext, LZ4F_createDecompressionContext_advanced,
    LZ4F_cctx_size, LZ4F_dctx_size,
    LZ4F_decompress, LZ4F_decompress_usingDict,
    LZ4F_flush,
    LZ4F_freeCDict, LZ4F_freeCompressionContext, LZ4F_freeDecompressionContext,
    LZ4F_getBlockSize, LZ4F_getErrorCode, LZ4F_getErrorName, LZ4F_getFrameInfo,
    LZ4F_getVersion, LZ4F_headerSize,
    LZ4F_isError,
    LZ4F_resetDecompressionContext,
    LZ4F_compressOptions_t, LZ4F_decompressOptions_t,
    LZ4F_frameInfo_t, LZ4F_preferences_t,
};

// LZ4 File I/O
use lz4_raw::src::lz4file::{
    LZ4F_readOpen, LZ4F_read, LZ4F_readClose,
    LZ4F_writeOpen, LZ4F_write, LZ4F_writeClose,
    LZ4_readFile_t, LZ4_writeFile_t,
};

// xxhash
use lz4_raw::src::xxhash::{
    XXH32, XXH32_canonicalFromHash, XXH32_canonical_t, XXH32_copyState, XXH32_createState,
    XXH32_digest, XXH32_freeState, XXH32_hashFromCanonical, XXH32_reset, XXH32_update,
    XXH64, XXH64_canonicalFromHash, XXH64_canonical_t, XXH64_copyState, XXH64_createState,
    XXH64_digest, XXH64_freeState, XXH64_hashFromCanonical, XXH64_reset, XXH64_update,
    XXH_versionNumber,
};

// C standard library functions we need for file I/O
extern "C" {
    fn fopen(filename: *const std::os::raw::c_char, mode: *const std::os::raw::c_char) -> *mut std::ffi::c_void;
    fn fclose(stream: *mut std::ffi::c_void) -> std::os::raw::c_int;
    fn unlink(pathname: *const std::os::raw::c_char) -> std::os::raw::c_int;
}

fn fold(acc: u64, x: u64) -> u64 {
    acc.rotate_left(5) ^ x
}

fn xor_fold_bytes(data: &[u8]) -> u64 {
    let mut h: u64 = 0;
    for (i, &b) in data.iter().enumerate() {
        h = h.wrapping_add((b as u64).wrapping_mul(i as u64).wrapping_add(1));
    }
    h
}

// ── op: compress_decompress ──
pub fn op_compress_decompress(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { last_err = -1; digest = fold(digest, 0); continue; }
            let mut compressed = vec![0u8; bound as usize];
            let comp_size = LZ4_compress_default(
                data.as_ptr() as *const i8, compressed.as_mut_ptr() as *mut i8, src_size, bound,
            );
            if comp_size <= 0 { last_err = comp_size; digest = fold(digest, comp_size as u64); continue; }
            let mut decompressed = vec![0u8; src_size as usize];
            let decomp_size = LZ4_decompress_safe(
                compressed.as_ptr() as *const i8, decompressed.as_mut_ptr() as *mut i8, comp_size, src_size,
            );
            if decomp_size < 0 { last_err = decomp_size; digest = fold(digest, decomp_size as u64); continue; }
            let partial_target = std::cmp::max(1, src_size / 2);
            let mut partial_buf = vec![0u8; src_size as usize];
            let partial_size = LZ4_decompress_safe_partial(
                compressed.as_ptr() as *const i8, partial_buf.as_mut_ptr() as *mut i8,
                comp_size, partial_target, src_size,
            );
            let iter_digest = fold(
                fold(fold(comp_size as u64, decomp_size as u64), xor_fold_bytes(&decompressed[..decomp_size as usize])),
                partial_size as u64,
            );
            digest = fold(digest, iter_digest);
        }
    }
    format!("op=compress_decompress err={} digest={:016x}", last_err, digest)
}

// ── op: compress_hc_decompress ──
pub fn op_compress_hc_decompress(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { last_err = -1; digest = fold(digest, 0); continue; }
            let max_level = LZ4F_compressionLevel_max();
            let levels = [1, 2, 4, 6, 9, 12, max_level];
            let mut iter_d: u64 = 0;
            for &lvl in &levels {
                let mut compressed = vec![0u8; bound as usize];
                let comp_size = LZ4_compress_HC(
                    data.as_ptr() as *const i8, compressed.as_mut_ptr() as *mut i8, src_size, bound, lvl,
                );
                if comp_size <= 0 { last_err = comp_size; iter_d = fold(iter_d, comp_size as u64); continue; }
                let mut decompressed = vec![0u8; src_size as usize];
                let decomp_size = LZ4_decompress_safe(
                    compressed.as_ptr() as *const i8, decompressed.as_mut_ptr() as *mut i8, comp_size, src_size,
                );
                iter_d = fold(iter_d, fold(comp_size as u64, decomp_size as u64));
            }
            digest = fold(digest, iter_d);
        }
    }
    format!("op=compress_hc_decompress err={} digest={:016x}", last_err, digest)
}

// ── op: frame_round_trip ──
pub fn op_frame_round_trip(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len();
            let prefs: LZ4F_preferences_t = std::mem::zeroed();
            let frame_bound = LZ4F_compressFrameBound(src_size, &prefs);
            if frame_bound == 0 { digest = fold(digest, 0); continue; }
            let mut compressed = vec![0u8; frame_bound];
            let comp_size = LZ4F_compressFrame(
                compressed.as_mut_ptr() as *mut _, frame_bound,
                data.as_ptr() as *const _, src_size, &prefs,
            );
            if LZ4F_isError(comp_size) != 0 { last_err = comp_size as u64; digest = fold(digest, comp_size as u64); continue; }
            let hdr_sz = LZ4F_headerSize(compressed.as_ptr() as *const _, comp_size);
            let mut dctx: *mut LZ4F_dctx = ptr::null_mut();
            let ret = LZ4F_createDecompressionContext(&mut dctx, 100);
            if LZ4F_isError(ret) != 0 || dctx.is_null() { last_err = ret as u64; digest = fold(digest, ret as u64); continue; }
            let mut frame_info: LZ4F_frameInfo_t = std::mem::zeroed();
            let mut fi_src_size = comp_size;
            let _fi_ret = LZ4F_getFrameInfo(dctx, &mut frame_info, compressed.as_ptr() as *const _, &mut fi_src_size);
            LZ4F_resetDecompressionContext(dctx);
            let decomp_cap = src_size + 1024;
            let mut decompressed = vec![0u8; decomp_cap];
            let mut dst_size = decomp_cap;
            let mut src_consumed = comp_size;
            let opts: LZ4F_decompressOptions_t = std::mem::zeroed();
            let result = LZ4F_decompress(
                dctx, decompressed.as_mut_ptr() as *mut _, &mut dst_size,
                compressed.as_ptr() as *const _, &mut src_consumed, &opts,
            );
            if LZ4F_isError(result) != 0 { last_err = result as u64; }
            let dctx_sz = LZ4F_dctx_size(dctx);
            let iter_digest = fold(
                fold(fold(comp_size as u64, dst_size as u64), xor_fold_bytes(&decompressed[..dst_size])),
                fold(hdr_sz as u64, dctx_sz as u64),
            );
            digest = fold(digest, iter_digest);
            LZ4F_freeDecompressionContext(dctx);
        }
    }
    format!("op=frame_round_trip err={} digest={:016x}", last_err, digest)
}

// ── op: streaming_compress ──
pub fn op_streaming_compress(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    let chunk_size: usize = 4096;
    for _ in 0..iters {
        unsafe {
            let stream = LZ4_createStream();
            if stream.is_null() { last_err = -1; digest = fold(digest, 0); continue; }
            LZ4_resetStream(stream);
            let mut total_comp_size: u64 = 0;
            let mut all_decomp_xor: u64 = 0;
            let mut save_buf = vec![0u8; 65536];
            let mut had_error = false;
            let mut offset = 0usize;
            while offset < data.len() {
                let end = std::cmp::min(offset + chunk_size, data.len());
                let chunk = &data[offset..end];
                let chunk_len = chunk.len() as i32;
                let bound = LZ4_compressBound(chunk_len);
                if bound <= 0 { last_err = -2; had_error = true; break; }
                let mut comp_buf = vec![0u8; bound as usize];
                let comp_size = LZ4_compress_fast_continue(
                    stream, chunk.as_ptr() as *const i8, comp_buf.as_mut_ptr() as *mut i8,
                    chunk_len, bound, 1,
                );
                if comp_size <= 0 { last_err = comp_size; had_error = true; break; }
                total_comp_size = total_comp_size.wrapping_add(comp_size as u64);
                let mut decomp_buf = vec![0u8; chunk_len as usize];
                let decomp_size = LZ4_decompress_safe(
                    comp_buf.as_ptr() as *const i8, decomp_buf.as_mut_ptr() as *mut i8,
                    comp_size, chunk_len,
                );
                if decomp_size < 0 { last_err = decomp_size; had_error = true; break; }
                all_decomp_xor = fold(all_decomp_xor, xor_fold_bytes(&decomp_buf[..decomp_size as usize]));
                LZ4_saveDict(stream, save_buf.as_mut_ptr() as *mut i8, save_buf.len() as i32);
                offset = end;
            }
            LZ4_freeStream(stream);
            if had_error { digest = fold(digest, last_err as u64); }
            else { digest = fold(digest, fold(total_comp_size, all_decomp_xor)); }
        }
    }
    format!("op=streaming_compress err={} digest={:016x}", last_err, digest)
}


// ── op: xxhash_digest ──
pub fn op_xxhash_digest(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let seed32: u32 = 0;
            let seed64: u64 = 0;
            let h32_oneshot = XXH32(data.as_ptr() as *const _, data.len(), seed32);
            let h64_oneshot = XXH64(data.as_ptr() as *const _, data.len(), seed64);
            let state32 = XXH32_createState();
            let h32_streaming = if !state32.is_null() {
                XXH32_reset(state32, seed32);
                XXH32_update(state32, data.as_ptr() as *const _, data.len());
                let d = XXH32_digest(state32);
                let state32b = XXH32_createState();
                if !state32b.is_null() {
                    XXH32_copyState(state32b, state32);
                    XXH32_freeState(state32b);
                }
                XXH32_freeState(state32);
                d
            } else { last_err = -1; 0 };
            let state64 = XXH64_createState();
            let h64_streaming = if !state64.is_null() {
                XXH64_reset(state64, seed64);
                XXH64_update(state64, data.as_ptr() as *const _, data.len());
                let d = XXH64_digest(state64);
                let state64b = XXH64_createState();
                if !state64b.is_null() {
                    XXH64_copyState(state64b, state64);
                    XXH64_freeState(state64b);
                }
                XXH64_freeState(state64);
                d
            } else { last_err = -2; 0 };
            let mut canon32 = MaybeUninit::<XXH32_canonical_t>::zeroed();
            XXH32_canonicalFromHash(canon32.as_mut_ptr(), h32_oneshot);
            let h32_rt = XXH32_hashFromCanonical(canon32.as_ptr());
            let mut canon64 = MaybeUninit::<XXH64_canonical_t>::zeroed();
            XXH64_canonicalFromHash(canon64.as_mut_ptr(), h64_oneshot);
            let h64_rt = XXH64_hashFromCanonical(canon64.as_ptr());
            let ver = XXH_versionNumber();
            let iter_digest = fold(
                fold(fold(h32_oneshot as u64, h32_streaming as u64), h64_oneshot as u64),
                fold(fold(h64_streaming as u64, h32_rt as u64), fold(h64_rt as u64, ver as u64)),
            );
            digest = fold(digest, iter_digest);
        }
    }
    format!("op=xxhash_digest err={} digest={:016x}", last_err, digest)
}

// ── op: legacy_compress ──
pub fn op_legacy_compress(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }
            let mut comp1 = vec![0u8; bound as usize];
            let c1 = LZ4_compress(data.as_ptr() as *const i8, comp1.as_mut_ptr() as *mut i8, src_size);
            let mut comp2 = vec![0u8; bound as usize];
            let c2 = LZ4_compress_limitedOutput(
                data.as_ptr() as *const i8, comp2.as_mut_ptr() as *mut i8, src_size, bound,
            );
            let state_size = LZ4_sizeofState();
            let mut state_buf = vec![0u8; state_size as usize + 16];
            let state_ptr = state_buf.as_mut_ptr() as *mut std::ffi::c_void;
            let mut comp3 = vec![0u8; bound as usize];
            let c3 = LZ4_compress_withState(
                state_ptr, data.as_ptr() as *const i8, comp3.as_mut_ptr() as *mut i8, src_size,
            );
            let mut comp4 = vec![0u8; bound as usize];
            let c4 = LZ4_compress_limitedOutput_withState(
                state_ptr, data.as_ptr() as *const i8, comp4.as_mut_ptr() as *mut i8, src_size, bound,
            );
            let mut comp5 = vec![0u8; bound as usize];
            let c5 = LZ4_compress_fast(
                data.as_ptr() as *const i8, comp5.as_mut_ptr() as *mut i8, src_size, bound, 2,
            );
            let mut decomp = vec![0u8; src_size as usize];
            let d1 = if c1 > 0 {
                LZ4_uncompress(comp1.as_ptr() as *const i8, decomp.as_mut_ptr() as *mut i8, src_size)
            } else { -1 };
            let mut decomp2 = vec![0u8; src_size as usize];
            let d2 = if c2 > 0 {
                LZ4_uncompress_unknownOutputSize(
                    comp2.as_ptr() as *const i8, decomp2.as_mut_ptr() as *mut i8, c2, src_size,
                )
            } else { -1 };
            let iter_d = fold(
                fold(fold(c1 as u64, c2 as u64), fold(c3 as u64, c4 as u64)),
                fold(fold(c5 as u64, d1 as u64), d2 as u64),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=legacy_compress err={} digest={:016x}", last_err, digest)
}

// ── op: compress_destsize ──
pub fn op_compress_destsize(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }
            let target = std::cmp::max(bound / 2, 66);
            let mut src_consumed = src_size;
            let mut comp = vec![0u8; target as usize];
            let c = LZ4_compress_destSize(
                data.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8, &mut src_consumed, target,
            );
            let state_size = LZ4_sizeofState();
            let mut state_buf = vec![0u8; state_size as usize + 16];
            let mut src_consumed2 = src_size;
            let mut comp2 = vec![0u8; target as usize];
            let c2 = LZ4_compress_destSize_extState(
                state_buf.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                comp2.as_mut_ptr() as *mut i8, &mut src_consumed2, target, 1,
            );
            let hc_state_size = LZ4_sizeofStateHC();
            let mut hc_state = vec![0u8; hc_state_size as usize + 16];
            let mut src_consumed3 = src_size;
            let mut comp3 = vec![0u8; target as usize];
            let c3 = LZ4_compress_HC_destSize(
                hc_state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                comp3.as_mut_ptr() as *mut i8, &mut src_consumed3, target, 9,
            );
            let iter_d = fold(
                fold(c as u64, fold(src_consumed as u64, c2 as u64)),
                fold(src_consumed2 as u64, fold(c3 as u64, src_consumed3 as u64)),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=compress_destsize err=0 digest={:016x}", digest)
}

// ── op: decompress_fast_roundtrip ──
pub fn op_decompress_fast_roundtrip(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }
            let mut compressed = vec![0u8; bound as usize];
            let comp_size = LZ4_compress_default(
                data.as_ptr() as *const i8, compressed.as_mut_ptr() as *mut i8, src_size, bound,
            );
            if comp_size <= 0 { last_err = comp_size; digest = fold(digest, comp_size as u64); continue; }
            let mut decomp = vec![0u8; src_size as usize];
            let d1 = LZ4_decompress_fast(
                compressed.as_ptr() as *const i8, decomp.as_mut_ptr() as *mut i8, src_size,
            );
            let dict: [u8; 4] = [0, 0, 0, 0];
            let mut decomp2 = vec![0u8; src_size as usize];
            let d2 = LZ4_decompress_fast_usingDict(
                compressed.as_ptr() as *const i8, decomp2.as_mut_ptr() as *mut i8,
                src_size, dict.as_ptr() as *const i8, 0,
            );
            let prefix_size = 65536usize;
            let mut big_buf = vec![0u8; prefix_size + src_size as usize];
            let d3 = LZ4_decompress_fast_withPrefix64k(
                compressed.as_ptr() as *const i8,
                big_buf.as_mut_ptr().add(prefix_size) as *mut i8,
                src_size,
            );
            let ring_sz = LZ4_decoderRingBufferSize(src_size);
            let iter_d = fold(
                fold(d1 as u64, d2 as u64),
                fold(d3 as u64, ring_sz as u64),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=decompress_fast_roundtrip err={} digest={:016x}", last_err, digest)
}

// ── op: decompress_usingdict ──
pub fn op_decompress_usingdict(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }
            let dict_data = b"dictionary_data_for_testing_1234567890";
            let stream = LZ4_createStream();
            if stream.is_null() { digest = fold(digest, 0); continue; }
            LZ4_loadDict(stream, dict_data.as_ptr() as *const i8, dict_data.len() as i32);
            let mut compressed = vec![0u8; bound as usize];
            let comp_size = LZ4_compress_fast_continue(
                stream, data.as_ptr() as *const i8, compressed.as_mut_ptr() as *mut i8,
                src_size, bound, 1,
            );
            LZ4_freeStream(stream);
            if comp_size <= 0 { last_err = comp_size; digest = fold(digest, comp_size as u64); continue; }
            let mut decomp = vec![0u8; src_size as usize];
            let d1 = LZ4_decompress_safe_usingDict(
                compressed.as_ptr() as *const i8, decomp.as_mut_ptr() as *mut i8,
                comp_size, src_size, dict_data.as_ptr() as *const i8, dict_data.len() as i32,
            );
            let partial_target = std::cmp::max(1, src_size / 2);
            let mut decomp2 = vec![0u8; src_size as usize];
            let d2 = LZ4_decompress_safe_partial_usingDict(
                compressed.as_ptr() as *const i8, decomp2.as_mut_ptr() as *mut i8,
                comp_size, partial_target, src_size,
                dict_data.as_ptr() as *const i8, dict_data.len() as i32,
            );
            let iter_d = fold(
                fold(comp_size as u64, d1 as u64),
                fold(d2 as u64, xor_fold_bytes(&decomp[..std::cmp::max(0, d1) as usize])),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=decompress_usingdict err={} digest={:016x}", last_err, digest)
}

// ── op: streaming_hc_compress ──
pub fn op_streaming_hc_compress(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    let chunk_size: usize = 2048;
    for _ in 0..iters {
        unsafe {
            let stream = LZ4_createStreamHC();
            if stream.is_null() { last_err = -1; digest = fold(digest, 0); continue; }
            LZ4_resetStreamHC(stream, 9);
            LZ4_setCompressionLevel(stream, 9);
            LZ4_favorDecompressionSpeed(stream, 1);
            let dict_data = b"hc_dictionary_test_data_abcdefghijklmnop";
            LZ4_loadDictHC(stream, dict_data.as_ptr() as *const i8, dict_data.len() as i32);
            let mut total: u64 = 0;
            let mut had_error = false;
            let mut save_buf = vec![0u8; 65536];
            let mut offset = 0usize;
            while offset < data.len() {
                let end = std::cmp::min(offset + chunk_size, data.len());
                let chunk = &data[offset..end];
                let chunk_len = chunk.len() as i32;
                let bound = LZ4_compressBound(chunk_len);
                if bound <= 0 { had_error = true; break; }
                let mut comp = vec![0u8; bound as usize];
                let c = LZ4_compress_HC_continue(
                    stream, chunk.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8,
                    chunk_len, bound,
                );
                if c <= 0 { last_err = c; had_error = true; break; }
                total = fold(total, c as u64);
                LZ4_saveDictHC(stream, save_buf.as_mut_ptr() as *mut i8, save_buf.len() as i32);
                offset = end;
            }
            LZ4_resetStreamHC_fast(stream, 6);
            if data.len() > 0 {
                let bound = LZ4_compressBound(data.len() as i32);
                if bound > 0 {
                    let target = std::cmp::max(bound / 2, 66);
                    let mut src_consumed = data.len() as i32;
                    let mut comp = vec![0u8; target as usize];
                    let c = LZ4_compress_HC_continue_destSize(
                        stream, data.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8,
                        &mut src_consumed, target,
                    );
                    total = fold(total, fold(c as u64, src_consumed as u64));
                }
            }
            LZ4_freeStreamHC(stream);
            if had_error { digest = fold(digest, last_err as u64); }
            else { digest = fold(digest, total); }
        }
    }
    format!("op=streaming_hc_compress err={} digest={:016x}", last_err, digest)
}

// ── op: frame_streaming_round_trip ──
pub fn op_frame_streaming_round_trip(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len();
            let mut cctx: *mut LZ4F_cctx = ptr::null_mut();
            let ret = LZ4F_createCompressionContext(&mut cctx, LZ4F_getVersion());
            if LZ4F_isError(ret) != 0 || cctx.is_null() {
                last_err = ret as u64;
                digest = fold(digest, ret as u64);
                continue;
            }
            let cctx_sz = LZ4F_cctx_size(cctx);
            let prefs: LZ4F_preferences_t = std::mem::zeroed();
            let header_bound = LZ4F_compressBound(src_size, &prefs);
            let total_bound = header_bound + 128 + src_size * 2;
            let mut compressed = vec![0u8; total_bound];
            let mut offset_out = 0usize;
            let hdr_size = LZ4F_compressBegin(cctx, compressed.as_mut_ptr() as *mut _, total_bound, &prefs);
            if LZ4F_isError(hdr_size) != 0 {
                last_err = hdr_size as u64;
                LZ4F_freeCompressionContext(cctx);
                digest = fold(digest, hdr_size as u64);
                continue;
            }
            offset_out += hdr_size;
            let chunk_size = 4096;
            let opts: LZ4F_compressOptions_t = std::mem::zeroed();
            let mut src_off = 0usize;
            let mut err_in_loop = false;
            while src_off < src_size {
                let end = std::cmp::min(src_off + chunk_size, src_size);
                let chunk = &data[src_off..end];
                let remaining = total_bound.saturating_sub(offset_out);
                let written = LZ4F_compressUpdate(
                    cctx, compressed.as_mut_ptr().add(offset_out) as *mut _, remaining,
                    chunk.as_ptr() as *const _, chunk.len(), &opts,
                );
                if LZ4F_isError(written) != 0 {
                    last_err = written as u64;
                    err_in_loop = true;
                    break;
                }
                offset_out += written;
                src_off = end;
            }
            if !err_in_loop {
                let remaining = total_bound.saturating_sub(offset_out);
                let flushed = LZ4F_flush(cctx, compressed.as_mut_ptr().add(offset_out) as *mut _, remaining, &opts);
                if LZ4F_isError(flushed) == 0 { offset_out += flushed; }
                let remaining = total_bound.saturating_sub(offset_out);
                let ended = LZ4F_compressEnd(cctx, compressed.as_mut_ptr().add(offset_out) as *mut _, remaining, &opts);
                if LZ4F_isError(ended) == 0 { offset_out += ended; }
                else { last_err = ended as u64; }
            }
            LZ4F_freeCompressionContext(cctx);
            let mut dctx: *mut LZ4F_dctx = ptr::null_mut();
            let ret2 = LZ4F_createDecompressionContext(&mut dctx, 100);
            if LZ4F_isError(ret2) != 0 || dctx.is_null() {
                last_err = ret2 as u64;
                digest = fold(digest, ret2 as u64);
                continue;
            }
            let decomp_cap = src_size + 4096;
            let mut decompressed = vec![0u8; decomp_cap];
            let mut dst_size = decomp_cap;
            let mut src_consumed = offset_out;
            let dopts: LZ4F_decompressOptions_t = std::mem::zeroed();
            let result = LZ4F_decompress(
                dctx, decompressed.as_mut_ptr() as *mut _, &mut dst_size,
                compressed.as_ptr() as *const _, &mut src_consumed, &dopts,
            );
            if LZ4F_isError(result) != 0 { last_err = result as u64; }
            let iter_d = fold(
                fold(offset_out as u64, dst_size as u64),
                fold(cctx_sz as u64, xor_fold_bytes(&decompressed[..dst_size])),
            );
            digest = fold(digest, iter_d);
            LZ4F_freeDecompressionContext(dctx);
        }
    }
    format!("op=frame_streaming_round_trip err={} digest={:016x}", last_err, digest)
}

// ── op: frame_dict_round_trip ──
pub fn op_frame_dict_round_trip(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let dict_data = b"frame_dictionary_for_testing_0123456789abcdef";
            let src_size = data.len();
            let cdict = LZ4F_createCDict(dict_data.as_ptr() as *const _, dict_data.len());
            if cdict.is_null() { digest = fold(digest, 0); continue; }
            let prefs: LZ4F_preferences_t = std::mem::zeroed();
            let frame_bound = LZ4F_compressFrameBound(src_size, &prefs);
            if frame_bound == 0 { LZ4F_freeCDict(cdict); digest = fold(digest, 0); continue; }
            let mut cctx: *mut LZ4F_cctx = ptr::null_mut();
            let ret = LZ4F_createCompressionContext(&mut cctx, LZ4F_getVersion());
            if LZ4F_isError(ret) != 0 || cctx.is_null() {
                LZ4F_freeCDict(cdict);
                last_err = ret as u64;
                digest = fold(digest, ret as u64);
                continue;
            }
            let mut compressed = vec![0u8; frame_bound + 256];
            let total_cap = compressed.len();
            let hdr = LZ4F_compressBegin_usingCDict(cctx, compressed.as_mut_ptr() as *mut _, total_cap, cdict, &prefs);
            if LZ4F_isError(hdr) != 0 {
                last_err = hdr as u64;
                LZ4F_freeCompressionContext(cctx);
                LZ4F_freeCDict(cdict);
                digest = fold(digest, hdr as u64);
                continue;
            }
            let mut off = hdr;
            let opts: LZ4F_compressOptions_t = std::mem::zeroed();
            if src_size > 0 {
                let remaining = total_cap.saturating_sub(off);
                let w = LZ4F_compressUpdate(
                    cctx, compressed.as_mut_ptr().add(off) as *mut _, remaining,
                    data.as_ptr() as *const _, src_size, &opts,
                );
                if LZ4F_isError(w) == 0 { off += w; }
            }
            let remaining = total_cap.saturating_sub(off);
            let ended = LZ4F_compressEnd(cctx, compressed.as_mut_ptr().add(off) as *mut _, remaining, &opts);
            if LZ4F_isError(ended) == 0 { off += ended; }
            LZ4F_freeCompressionContext(cctx);
            let mut cctx2: *mut LZ4F_cctx = ptr::null_mut();
            LZ4F_createCompressionContext(&mut cctx2, LZ4F_getVersion());
            let mut comp2 = vec![0u8; frame_bound + 256];
            let c2 = if !cctx2.is_null() {
                let r = LZ4F_compressFrame_usingCDict(
                    cctx2, comp2.as_mut_ptr() as *mut _, comp2.len(),
                    data.as_ptr() as *const _, src_size, cdict, &prefs,
                );
                LZ4F_freeCompressionContext(cctx2);
                if LZ4F_isError(r) == 0 { r } else { 0 }
            } else { 0 };
            LZ4F_freeCDict(cdict);
            let mut dctx: *mut LZ4F_dctx = ptr::null_mut();
            LZ4F_createDecompressionContext(&mut dctx, 100);
            let decomp_cap = src_size + 4096;
            let mut decompressed = vec![0u8; decomp_cap];
            let mut dst_size = decomp_cap;
            let mut src_consumed = off;
            let dopts: LZ4F_decompressOptions_t = std::mem::zeroed();
            let result = if !dctx.is_null() {
                let r = LZ4F_decompress_usingDict(
                    dctx, decompressed.as_mut_ptr() as *mut _, &mut dst_size,
                    compressed.as_ptr() as *const _, &mut src_consumed,
                    dict_data.as_ptr() as *const _, dict_data.len(), &dopts,
                );
                LZ4F_freeDecompressionContext(dctx);
                r
            } else { 0 };
            let iter_d = fold(
                fold(off as u64, c2 as u64),
                fold(dst_size as u64, result as u64),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=frame_dict_round_trip err={} digest={:016x}", last_err, digest)
}

// ── op: frame_decompress_usingdict ──
pub fn op_frame_decompress_usingdict(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len();
            let dict_data = b"decompress_dict_test_data";
            let mut cctx: *mut LZ4F_cctx = ptr::null_mut();
            let ret = LZ4F_createCompressionContext(&mut cctx, LZ4F_getVersion());
            if LZ4F_isError(ret) != 0 || cctx.is_null() {
                last_err = ret as u64;
                digest = fold(digest, ret as u64);
                continue;
            }
            let prefs: LZ4F_preferences_t = std::mem::zeroed();
            let bound = LZ4F_compressBound(src_size, &prefs) + 256;
            let mut compressed = vec![0u8; bound];
            let hdr = LZ4F_compressBegin_usingDict(
                cctx, compressed.as_mut_ptr() as *mut _, bound,
                dict_data.as_ptr() as *const _, dict_data.len(), &prefs,
            );
            if LZ4F_isError(hdr) != 0 {
                last_err = hdr as u64;
                LZ4F_freeCompressionContext(cctx);
                digest = fold(digest, hdr as u64);
                continue;
            }
            let mut off = hdr;
            let opts: LZ4F_compressOptions_t = std::mem::zeroed();
            if src_size > 0 {
                let remaining = bound.saturating_sub(off);
                let w = LZ4F_compressUpdate(
                    cctx, compressed.as_mut_ptr().add(off) as *mut _, remaining,
                    data.as_ptr() as *const _, src_size, &opts,
                );
                if LZ4F_isError(w) == 0 { off += w; }
            }
            let remaining = bound.saturating_sub(off);
            let ended = LZ4F_compressEnd(cctx, compressed.as_mut_ptr().add(off) as *mut _, remaining, &opts);
            if LZ4F_isError(ended) == 0 { off += ended; }
            LZ4F_freeCompressionContext(cctx);
            let mut dctx: *mut LZ4F_dctx = ptr::null_mut();
            LZ4F_createDecompressionContext(&mut dctx, 100);
            if dctx.is_null() { digest = fold(digest, 0); continue; }
            let decomp_cap = src_size + 4096;
            let mut decompressed = vec![0u8; decomp_cap];
            let mut dst_size = decomp_cap;
            let mut src_consumed = off;
            let dopts: LZ4F_decompressOptions_t = std::mem::zeroed();
            let result = LZ4F_decompress_usingDict(
                dctx, decompressed.as_mut_ptr() as *mut _, &mut dst_size,
                compressed.as_ptr() as *const _, &mut src_consumed,
                dict_data.as_ptr() as *const _, dict_data.len(), &dopts,
            );
            if LZ4F_isError(result) != 0 { last_err = result as u64; }
            let iter_d = fold(off as u64, fold(dst_size as u64, xor_fold_bytes(&decompressed[..dst_size])));
            digest = fold(digest, iter_d);
            LZ4F_freeDecompressionContext(dctx);
        }
    }
    format!("op=frame_decompress_usingdict err={} digest={:016x}", last_err, digest)
}

// ── op: streaming_decode ──
pub fn op_streaming_decode(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }
            let mut compressed = vec![0u8; bound as usize];
            let comp_size = LZ4_compress_default(
                data.as_ptr() as *const i8, compressed.as_mut_ptr() as *mut i8, src_size, bound,
            );
            if comp_size <= 0 { last_err = comp_size; digest = fold(digest, comp_size as u64); continue; }
            let sd = LZ4_createStreamDecode();
            if sd.is_null() { last_err = -1; digest = fold(digest, 0); continue; }
            LZ4_setStreamDecode(sd, ptr::null(), 0);
            let ring_sz = LZ4_decoderRingBufferSize(src_size);
            let mut ring_buf = vec![0u8; ring_sz as usize];
            let d1 = LZ4_decompress_safe_continue(
                sd, compressed.as_ptr() as *const i8, ring_buf.as_mut_ptr() as *mut i8,
                comp_size, src_size,
            );
            let sd2 = LZ4_createStreamDecode();
            let d2 = if !sd2.is_null() {
                LZ4_setStreamDecode(sd2, ptr::null(), 0);
                let mut ring_buf2 = vec![0u8; ring_sz as usize];
                let r = LZ4_decompress_fast_continue(
                    sd2, compressed.as_ptr() as *const i8, ring_buf2.as_mut_ptr() as *mut i8, src_size,
                );
                LZ4_freeStreamDecode(sd2);
                r
            } else { -1 };
            let iter_d = fold(
                fold(d1 as u64, d2 as u64),
                fold(ring_sz as u64, xor_fold_bytes(&ring_buf[..std::cmp::max(0, d1) as usize])),
            );
            digest = fold(digest, iter_d);
            LZ4_freeStreamDecode(sd);
        }
    }
    format!("op=streaming_decode err={} digest={:016x}", last_err, digest)
}

// ── op: attach_dictionary ──
pub fn op_attach_dictionary(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }
            let dict_data = b"attach_dict_test_data_0123456789abcdefghijklmnopqrstuvwxyz";
            let dict_stream = LZ4_createStream();
            if dict_stream.is_null() { digest = fold(digest, 0); continue; }
            LZ4_loadDict(dict_stream, dict_data.as_ptr() as *const i8, dict_data.len() as i32);
            let work_stream = LZ4_createStream();
            if work_stream.is_null() { LZ4_freeStream(dict_stream); digest = fold(digest, 0); continue; }
            LZ4_attach_dictionary(work_stream, dict_stream);
            let mut compressed = vec![0u8; bound as usize];
            let c = LZ4_compress_fast_continue(
                work_stream, data.as_ptr() as *const i8, compressed.as_mut_ptr() as *mut i8,
                src_size, bound, 1,
            );
            LZ4_freeStream(work_stream);
            LZ4_freeStream(dict_stream);
            let dict_hc = LZ4_createStreamHC();
            if dict_hc.is_null() { digest = fold(digest, c as u64); continue; }
            LZ4_resetStreamHC(dict_hc, 9);
            LZ4_loadDictHC(dict_hc, dict_data.as_ptr() as *const i8, dict_data.len() as i32);
            let work_hc = LZ4_createStreamHC();
            if work_hc.is_null() { LZ4_freeStreamHC(dict_hc); digest = fold(digest, c as u64); continue; }
            LZ4_resetStreamHC(work_hc, 9);
            LZ4_attach_HC_dictionary(work_hc, dict_hc);
            let mut comp_hc = vec![0u8; bound as usize];
            let c2 = LZ4_compress_HC_continue(
                work_hc, data.as_ptr() as *const i8, comp_hc.as_mut_ptr() as *mut i8,
                src_size, bound,
            );
            LZ4_freeStreamHC(work_hc);
            LZ4_freeStreamHC(dict_hc);
            let stream3 = LZ4_createStream();
            let c3 = if !stream3.is_null() {
                LZ4_loadDictSlow(stream3, dict_data.as_ptr() as *const i8, dict_data.len() as i32);
                let mut comp3 = vec![0u8; bound as usize];
                let r = LZ4_compress_fast_continue(
                    stream3, data.as_ptr() as *const i8, comp3.as_mut_ptr() as *mut i8,
                    src_size, bound, 1,
                );
                LZ4_freeStream(stream3);
                r
            } else { 0 };
            let iter_d = fold(fold(c as u64, c2 as u64), c3 as u64);
            digest = fold(digest, iter_d);
        }
    }
    format!("op=attach_dictionary err={} digest={:016x}", last_err, digest)
}

// ── op: hc_legacy ──
pub fn op_hc_legacy(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }
            let mut c1_buf = vec![0u8; bound as usize];
            let c1 = LZ4_compressHC(data.as_ptr() as *const i8, c1_buf.as_mut_ptr() as *mut i8, src_size);
            let mut c2_buf = vec![0u8; bound as usize];
            let c2 = LZ4_compressHC_limitedOutput(
                data.as_ptr() as *const i8, c2_buf.as_mut_ptr() as *mut i8, src_size, bound,
            );
            let mut c3_buf = vec![0u8; bound as usize];
            let c3 = LZ4_compressHC2(data.as_ptr() as *const i8, c3_buf.as_mut_ptr() as *mut i8, src_size, 6);
            let mut c4_buf = vec![0u8; bound as usize];
            let c4 = LZ4_compressHC2_limitedOutput(
                data.as_ptr() as *const i8, c4_buf.as_mut_ptr() as *mut i8, src_size, bound, 6,
            );
            let hc_state_size = LZ4_sizeofStateHC();
            let mut hc_state = vec![0u8; hc_state_size as usize + 16];
            let mut c5_buf = vec![0u8; bound as usize];
            let c5 = LZ4_compressHC_withStateHC(
                hc_state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                c5_buf.as_mut_ptr() as *mut i8, src_size,
            );
            let mut c6_buf = vec![0u8; bound as usize];
            let c6 = LZ4_compressHC_limitedOutput_withStateHC(
                hc_state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                c6_buf.as_mut_ptr() as *mut i8, src_size, bound,
            );
            let mut c7_buf = vec![0u8; bound as usize];
            let c7 = LZ4_compressHC2_withStateHC(
                hc_state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                c7_buf.as_mut_ptr() as *mut i8, src_size, 6,
            );
            let mut c8_buf = vec![0u8; bound as usize];
            let c8 = LZ4_compressHC2_limitedOutput_withStateHC(
                hc_state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                c8_buf.as_mut_ptr() as *mut i8, src_size, bound, 6,
            );
            let mut c9_buf = vec![0u8; bound as usize];
            let c9 = LZ4_compress_HC_extStateHC(
                hc_state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                c9_buf.as_mut_ptr() as *mut i8, src_size, bound, 9,
            );
            let mut c10_buf = vec![0u8; bound as usize];
            let c10 = LZ4_compress_HC_extStateHC_fastReset(
                hc_state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                c10_buf.as_mut_ptr() as *mut i8, src_size, bound, 9,
            );
            let hc_ctx = LZ4_createHC(data.as_ptr() as *const i8);
            let c11 = if !hc_ctx.is_null() {
                let mut c11_buf = vec![0u8; bound as usize];
                let r = LZ4_compressHC_continue(
                    hc_ctx as *mut LZ4_streamHC_t, data.as_ptr() as *const i8,
                    c11_buf.as_mut_ptr() as *mut i8, src_size,
                );
                let mut c12_buf = vec![0u8; bound as usize];
                let r2 = LZ4_compressHC_limitedOutput_continue(
                    hc_ctx as *mut LZ4_streamHC_t, data.as_ptr() as *const i8,
                    c12_buf.as_mut_ptr() as *mut i8, src_size, bound,
                );
                let mut c13_buf = vec![0u8; bound as usize];
                let r3 = LZ4_compressHC2_continue(
                    hc_ctx, data.as_ptr() as *const i8,
                    c13_buf.as_mut_ptr() as *mut i8, src_size, 6,
                );
                let mut c14_buf = vec![0u8; bound as usize];
                let r4 = LZ4_compressHC2_limitedOutput_continue(
                    hc_ctx, data.as_ptr() as *const i8,
                    c14_buf.as_mut_ptr() as *mut i8, src_size, bound, 6,
                );
                LZ4_freeHC(hc_ctx);
                fold(fold(r as u64, r2 as u64), fold(r3 as u64, r4 as u64))
            } else { 0 };
            let iter_d = fold(
                fold(fold(c1 as u64, c2 as u64), fold(c3 as u64, c4 as u64)),
                fold(fold(c5 as u64, c6 as u64), fold(fold(c7 as u64, c8 as u64), fold(fold(c9 as u64, c10 as u64), c11))),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=hc_legacy err={} digest={:016x}", last_err, digest)
}

// ── op: frame_misc ──
pub fn op_frame_misc(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let ver = LZ4F_getVersion();
            let max_lvl = LZ4F_compressionLevel_max();
            let lz4_ver = LZ4_versionNumber();
            let lz4_ver_str = LZ4_versionString();
            let ver_str_byte = if !lz4_ver_str.is_null() { *lz4_ver_str as u8 } else { 0 };
            let bs4 = LZ4F_getBlockSize(4);
            let bs5 = LZ4F_getBlockSize(5);
            let bs6 = LZ4F_getBlockSize(6);
            let bs7 = LZ4F_getBlockSize(7);
            let err_name = LZ4F_getErrorName(usize::MAX);
            let err_byte = if !err_name.is_null() { *err_name as u8 } else { 0 };
            let err_code = LZ4F_getErrorCode(usize::MAX);
            let ss1 = LZ4_sizeofState();
            let ss2 = LZ4_sizeofStateHC();
            let ss3 = LZ4_sizeofStreamState();
            let ss4 = LZ4_sizeofStreamStateHC();
            let src_size = data.len();
            let mut cctx: *mut LZ4F_cctx = ptr::null_mut();
            let ret = LZ4F_createCompressionContext(&mut cctx, ver);
            let uncomp_d = if LZ4F_isError(ret) == 0 && !cctx.is_null() {
                let prefs: LZ4F_preferences_t = std::mem::zeroed();
                let bound = LZ4F_compressBound(src_size, &prefs) + 256;
                let mut buf = vec![0u8; bound];
                let hdr = LZ4F_compressBegin(cctx, buf.as_mut_ptr() as *mut _, bound, &prefs);
                let mut off = if LZ4F_isError(hdr) == 0 { hdr } else { 0 };
                if src_size > 0 && off > 0 {
                    let opts: LZ4F_compressOptions_t = std::mem::zeroed();
                    let remaining = bound.saturating_sub(off);
                    let w = LZ4F_uncompressedUpdate(
                        cctx, buf.as_mut_ptr().add(off) as *mut _, remaining,
                        data.as_ptr() as *const _, src_size, &opts,
                    );
                    if LZ4F_isError(w) == 0 { off += w; }
                    let remaining = bound.saturating_sub(off);
                    let ended = LZ4F_compressEnd(cctx, buf.as_mut_ptr().add(off) as *mut _, remaining, &opts);
                    if LZ4F_isError(ended) == 0 { off += ended; }
                }
                LZ4F_freeCompressionContext(cctx);
                off as u64
            } else {
                if !cctx.is_null() { LZ4F_freeCompressionContext(cctx); }
                0u64
            };
            let force_d = if src_size > 0 {
                let stream = LZ4_createStream();
                if !stream.is_null() {
                    let dict = b"forceextdict_test";
                    LZ4_loadDict(stream, dict.as_ptr() as *const i8, dict.len() as i32);
                    let bound = LZ4_compressBound(src_size as i32);
                    if bound > 0 {
                        let mut comp = vec![0u8; bound as usize];
                        let c = LZ4_compress_forceExtDict(
                            stream, data.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8, src_size as i32,
                        );
                        LZ4_freeStream(stream);
                        c as u64
                    } else { LZ4_freeStream(stream); 0 }
                } else { 0 }
            } else { 0 };
            let cont_d = if src_size > 0 {
                let stream = LZ4_createStream();
                if !stream.is_null() {
                    LZ4_resetStream(stream);
                    let bound = LZ4_compressBound(src_size as i32);
                    if bound > 0 {
                        let mut comp = vec![0u8; bound as usize];
                        let c = LZ4_compress_continue(
                            stream, data.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8, src_size as i32,
                        );
                        let mut comp2 = vec![0u8; bound as usize];
                        let c2 = LZ4_compress_limitedOutput_continue(
                            stream, data.as_ptr() as *const i8, comp2.as_mut_ptr() as *mut i8,
                            src_size as i32, bound,
                        );
                        LZ4_freeStream(stream);
                        fold(c as u64, c2 as u64)
                    } else { LZ4_freeStream(stream); 0 }
                } else { 0 }
            } else { 0 };
            let create_d = if src_size > 0 {
                let mut input_buf = vec![0u8; 65536 + src_size];
                input_buf[..src_size].copy_from_slice(data);
                let ctx = LZ4_create(input_buf.as_mut_ptr() as *mut i8);
                if !ctx.is_null() {
                    let _slid = LZ4_slideInputBuffer(ctx);
                    LZ4_freeStream(ctx as *mut LZ4_stream_t);
                    1u64
                } else { 0 }
            } else { 0 };
            let ext_d = if src_size > 0 {
                let state_size = LZ4_sizeofState();
                let mut state = vec![0u8; state_size as usize + 16];
                let bound = LZ4_compressBound(src_size as i32);
                if bound > 0 {
                    let mut comp = vec![0u8; bound as usize];
                    let c1 = LZ4_compress_fast_extState(
                        state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                        comp.as_mut_ptr() as *mut i8, src_size as i32, bound, 1,
                    );
                    let mut comp2 = vec![0u8; bound as usize];
                    let c2 = LZ4_compress_fast_extState_fastReset(
                        state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                        comp2.as_mut_ptr() as *mut i8, src_size as i32, bound, 1,
                    );
                    fold(c1 as u64, c2 as u64)
                } else { 0 }
            } else { 0 };
            let init_d = {
                let mut stream_buf = vec![0u8; std::mem::size_of::<LZ4_stream_t>() + 16];
                let s = LZ4_initStream(stream_buf.as_mut_ptr() as *mut _, std::mem::size_of::<LZ4_stream_t>());
                if !s.is_null() {
                    LZ4_resetStream_fast(s);
                    1u64
                } else { 0 }
            };
            let init_hc_d = {
                let mut stream_buf = vec![0u8; std::mem::size_of::<LZ4_streamHC_t>() + 16];
                let s = LZ4_initStreamHC(stream_buf.as_mut_ptr() as *mut _, std::mem::size_of::<LZ4_streamHC_t>());
                if !s.is_null() {
                    LZ4_resetStreamHC_fast(s, 9);
                    1u64
                } else { 0 }
            };
            let force_ext_d = if src_size > 0 {
                let bound = LZ4_compressBound(src_size as i32);
                if bound > 0 {
                    let mut comp = vec![0u8; bound as usize];
                    let c = LZ4_compress_default(
                        data.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8, src_size as i32, bound,
                    );
                    if c > 0 {
                        let mut decomp = vec![0u8; src_size];
                        let d = LZ4_decompress_safe_forceExtDict(
                            comp.as_ptr() as *const i8, decomp.as_mut_ptr() as *mut i8,
                            c, src_size as i32, ptr::null(), 0,
                        );
                        d as u64
                    } else { 0 }
                } else { 0 }
            } else { 0 };
            let iter_d = fold(
                fold(
                    fold(ver as u64, fold(max_lvl as u64, lz4_ver as u64)),
                    fold(fold(bs4 as u64, bs5 as u64), fold(bs6 as u64, bs7 as u64)),
                ),
                fold(
                    fold(fold(err_byte as u64, err_code as u64), fold(ss1 as u64, ss2 as u64)),
                    fold(
                        fold(fold(ss3 as u64, ss4 as u64), fold(uncomp_d, force_d)),
                        fold(fold(cont_d, create_d), fold(fold(ext_d, init_d), fold(init_hc_d, fold(force_ext_d, ver_str_byte as u64)))),
                    ),
                ),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=frame_misc err=0 digest={:016x}", digest)
}

// ── op: compress_extstate ──
pub fn op_compress_extstate(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }
            let state_size = LZ4_sizeofState();
            let mut state = vec![0u8; state_size as usize + 16];
            let accels = [1, 2, 5, 10];
            let mut iter_d: u64 = 0;
            for &accel in &accels {
                let mut comp = vec![0u8; bound as usize];
                let c = LZ4_compress_fast_extState(
                    state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                    comp.as_mut_ptr() as *mut i8, src_size, bound, accel,
                );
                iter_d = fold(iter_d, c as u64);
                let mut comp2 = vec![0u8; bound as usize];
                let c2 = LZ4_compress_fast_extState_fastReset(
                    state.as_mut_ptr() as *mut _, data.as_ptr() as *const i8,
                    comp2.as_mut_ptr() as *mut i8, src_size, bound, accel,
                );
                iter_d = fold(iter_d, c2 as u64);
            }
            let mut input_buf = vec![0u8; 65536 + src_size as usize];
            input_buf[..src_size as usize].copy_from_slice(data);
            let _stream_state_size = LZ4_sizeofStreamState();
            let mut ss_buf = vec![0u8; _stream_state_size as usize + 16];
            let _rs = LZ4_resetStreamState(ss_buf.as_mut_ptr() as *mut _, input_buf.as_mut_ptr() as *mut i8);
            let _hc_ss_size = LZ4_sizeofStreamStateHC();
            let hc_ctx = LZ4_createHC(input_buf.as_ptr() as *const i8);
            if !hc_ctx.is_null() {
                let _slid = LZ4_slideInputBufferHC(hc_ctx);
                let _rs2 = LZ4_resetStreamStateHC(hc_ctx, input_buf.as_mut_ptr() as *mut i8);
                LZ4_freeHC(hc_ctx);
            }
            digest = fold(digest, iter_d);
        }
    }
    format!("op=compress_extstate err=0 digest={:016x}", digest)
}

// ── op: file_io_round_trip ──
pub fn op_file_io_round_trip(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: u64 = 0;

    let tmp_path = std::ffi::CString::new("/tmp/lz4_harness_fileio.lz4").unwrap();
    let mode_wb = std::ffi::CString::new("wb").unwrap();
    let mode_rb = std::ffi::CString::new("rb").unwrap();

    for _ in 0..iters {
        unsafe {
            let src_size = data.len();

            // Write compressed file
            let fp_w = fopen(tmp_path.as_ptr(), mode_wb.as_ptr());
            if fp_w.is_null() {
                last_err = 1;
                digest = fold(digest, 1);
                continue;
            }

            let prefs: LZ4F_preferences_t = std::mem::zeroed();
            let mut lz4f_write: *mut LZ4_writeFile_t = ptr::null_mut();
            // LZ4F_writeOpen expects *const LZ4F_preferences_t from lz4file module
            // Cast the pointer to bridge potential type differences
            let ret_w = LZ4F_writeOpen(&mut lz4f_write, fp_w as *mut _, &prefs as *const _ as *const _);
            if LZ4F_isError(ret_w) != 0 || lz4f_write.is_null() {
                last_err = ret_w as u64;
                fclose(fp_w);
                digest = fold(digest, ret_w as u64);
                continue;
            }

            // Write data
            let written = if src_size > 0 {
                LZ4F_write(lz4f_write, data.as_ptr() as *const _, src_size)
            } else { 0 };

            let ret_wc = LZ4F_writeClose(lz4f_write);
            if LZ4F_isError(ret_wc) != 0 { last_err = ret_wc as u64; }
            fclose(fp_w);

            // Read back
            let fp_r = fopen(tmp_path.as_ptr(), mode_rb.as_ptr());
            if fp_r.is_null() {
                last_err = 2;
                digest = fold(digest, 2);
                continue;
            }

            let mut lz4f_read: *mut LZ4_readFile_t = ptr::null_mut();
            let ret_r = LZ4F_readOpen(&mut lz4f_read, fp_r as *mut _);
            if LZ4F_isError(ret_r) != 0 || lz4f_read.is_null() {
                last_err = ret_r as u64;
                fclose(fp_r);
                digest = fold(digest, ret_r as u64);
                continue;
            }

            let read_cap = src_size + 4096;
            let mut read_buf = vec![0u8; read_cap];
            let mut total_read: usize = 0;
            loop {
                let remaining = read_cap.saturating_sub(total_read);
                if remaining == 0 { break; }
                let n = LZ4F_read(lz4f_read, read_buf.as_mut_ptr().add(total_read) as *mut _, remaining);
                if LZ4F_isError(n) != 0 {
                    last_err = n as u64;
                    break;
                }
                if n == 0 { break; }
                total_read += n;
            }

            let ret_rc = LZ4F_readClose(lz4f_read);
            if LZ4F_isError(ret_rc) != 0 { last_err = ret_rc as u64; }
            fclose(fp_r);

            // Clean up temp file
            unlink(tmp_path.as_ptr());

            let iter_d = fold(
                fold(written as u64, total_read as u64),
                xor_fold_bytes(&read_buf[..total_read]),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=file_io_round_trip err={} digest={:016x}", last_err, digest)
}

// ── op: frame_hc_variants ──
pub fn op_frame_hc_variants(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len();
            let block_sizes: [u32; 4] = [4, 5, 6, 7];
            let levels: [i32; 4] = [3, 6, 9, 12];

            let mut iter_d: u64 = 0;

            for (bi, &bs) in block_sizes.iter().enumerate() {
                let lvl = levels[bi % levels.len()];
                let mut prefs: LZ4F_preferences_t = std::mem::zeroed();
                prefs.frameInfo.blockSizeID = bs;
                prefs.frameInfo.contentChecksumFlag = 1;
                prefs.frameInfo.blockChecksumFlag = 1;
                prefs.compressionLevel = lvl;
                prefs.frameInfo.contentSize = src_size as u64;

                let frame_bound = LZ4F_compressFrameBound(src_size, &prefs);
                if frame_bound == 0 { continue; }

                let mut compressed = vec![0u8; frame_bound];
                let comp_size = LZ4F_compressFrame(
                    compressed.as_mut_ptr() as *mut _, frame_bound,
                    data.as_ptr() as *const _, src_size, &prefs,
                );
                if LZ4F_isError(comp_size) != 0 {
                    last_err = comp_size as u64;
                    iter_d = fold(iter_d, comp_size as u64);
                    continue;
                }

                let mut dctx: *mut LZ4F_dctx = ptr::null_mut();
                let ret = LZ4F_createDecompressionContext(&mut dctx, 100);
                if LZ4F_isError(ret) != 0 || dctx.is_null() {
                    last_err = ret as u64;
                    iter_d = fold(iter_d, ret as u64);
                    continue;
                }

                let decomp_cap = src_size + 4096;
                let mut decompressed = vec![0u8; decomp_cap];
                let mut dst_size = decomp_cap;
                let mut src_consumed = comp_size;
                let dopts: LZ4F_decompressOptions_t = std::mem::zeroed();
                let result = LZ4F_decompress(
                    dctx, decompressed.as_mut_ptr() as *mut _, &mut dst_size,
                    compressed.as_ptr() as *const _, &mut src_consumed, &dopts,
                );
                if LZ4F_isError(result) != 0 { last_err = result as u64; }

                iter_d = fold(iter_d, fold(comp_size as u64, fold(dst_size as u64, bs as u64)));
                LZ4F_freeDecompressionContext(dctx);
            }

            // Also test streaming HC frame with autoFlush
            {
                let mut cctx: *mut LZ4F_cctx = ptr::null_mut();
                let ret = LZ4F_createCompressionContext(&mut cctx, LZ4F_getVersion());
                if LZ4F_isError(ret) == 0 && !cctx.is_null() {
                    let mut prefs: LZ4F_preferences_t = std::mem::zeroed();
                    prefs.compressionLevel = 9;
                    prefs.autoFlush = 1;
                    prefs.frameInfo.contentChecksumFlag = 1;
                    prefs.frameInfo.blockChecksumFlag = 1;
                    prefs.frameInfo.contentSize = src_size as u64;

                    let bound = LZ4F_compressBound(src_size, &prefs) + 256;
                    let mut buf = vec![0u8; bound];
                    let hdr = LZ4F_compressBegin(cctx, buf.as_mut_ptr() as *mut _, bound, &prefs);
                    if LZ4F_isError(hdr) == 0 {
                        let mut off = hdr;
                        let opts: LZ4F_compressOptions_t = std::mem::zeroed();
                        if src_size > 0 {
                            let remaining = bound.saturating_sub(off);
                            let w = LZ4F_compressUpdate(
                                cctx, buf.as_mut_ptr().add(off) as *mut _, remaining,
                                data.as_ptr() as *const _, src_size, &opts,
                            );
                            if LZ4F_isError(w) == 0 { off += w; }
                        }
                        let remaining = bound.saturating_sub(off);
                        let ended = LZ4F_compressEnd(cctx, buf.as_mut_ptr().add(off) as *mut _, remaining, &opts);
                        if LZ4F_isError(ended) == 0 { off += ended; }
                        iter_d = fold(iter_d, off as u64);
                    }
                    LZ4F_freeCompressionContext(cctx);
                }
            }

            digest = fold(digest, iter_d);
        }
    }
    format!("op=frame_hc_variants err={} digest={:016x}", last_err, digest)
}

// ── op: prefix64k_decompress ──
pub fn op_prefix64k_decompress(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }

            let mut compressed = vec![0u8; bound as usize];
            let comp_size = LZ4_compress_default(
                data.as_ptr() as *const i8, compressed.as_mut_ptr() as *mut i8, src_size, bound,
            );
            if comp_size <= 0 { last_err = comp_size; digest = fold(digest, comp_size as u64); continue; }

            let prefix_size = 65536usize;
            let mut big_buf = vec![0u8; prefix_size + src_size as usize];
            for i in 0..prefix_size {
                big_buf[i] = (i & 0xFF) as u8;
            }
            let d1 = LZ4_decompress_safe_withPrefix64k(
                compressed.as_ptr() as *const i8,
                big_buf.as_mut_ptr().add(prefix_size) as *mut i8,
                comp_size, src_size,
            );

            let mut big_buf2 = vec![0u8; prefix_size + src_size as usize];
            let d2 = LZ4_decompress_fast_withPrefix64k(
                compressed.as_ptr() as *const i8,
                big_buf2.as_mut_ptr().add(prefix_size) as *mut i8,
                src_size,
            );

            let iter_d = fold(
                fold(comp_size as u64, d1 as u64),
                fold(d2 as u64, xor_fold_bytes(&big_buf[prefix_size..prefix_size + std::cmp::max(0, d1) as usize])),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=prefix64k_decompress err={} digest={:016x}", last_err, digest)
}

// ── op: extdict_decompress ──
pub fn op_extdict_decompress(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len() as i32;
            if src_size == 0 { digest = fold(digest, 0); continue; }
            let bound = LZ4_compressBound(src_size);
            if bound <= 0 { digest = fold(digest, 0); continue; }

            let dict_data = b"extdict_test_dictionary_data_0123456789abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let stream = LZ4_createStream();
            if stream.is_null() { digest = fold(digest, 0); continue; }
            LZ4_loadDict(stream, dict_data.as_ptr() as *const i8, dict_data.len() as i32);
            let mut compressed = vec![0u8; bound as usize];
            let comp_size = LZ4_compress_fast_continue(
                stream, data.as_ptr() as *const i8, compressed.as_mut_ptr() as *mut i8,
                src_size, bound, 1,
            );
            LZ4_freeStream(stream);
            if comp_size <= 0 { last_err = comp_size; digest = fold(digest, comp_size as u64); continue; }

            let mut decomp1 = vec![0u8; src_size as usize];
            let d1 = LZ4_decompress_safe_forceExtDict(
                compressed.as_ptr() as *const i8, decomp1.as_mut_ptr() as *mut i8,
                comp_size, src_size,
                dict_data.as_ptr() as *const _, dict_data.len(),
            );

            let partial_target = std::cmp::max(1, src_size / 2);
            let mut decomp2 = vec![0u8; src_size as usize];
            let d2 = LZ4_decompress_safe_partial_forceExtDict(
                compressed.as_ptr() as *const i8, decomp2.as_mut_ptr() as *mut i8,
                comp_size, partial_target, src_size,
                dict_data.as_ptr() as *const _, dict_data.len(),
            );

            let mut decomp3 = vec![0u8; src_size as usize];
            let d3 = LZ4_decompress_fast_usingDict(
                compressed.as_ptr() as *const i8, decomp3.as_mut_ptr() as *mut i8,
                src_size, dict_data.as_ptr() as *const i8, dict_data.len() as i32,
            );

            let mut decomp4 = vec![0u8; src_size as usize];
            let d4 = LZ4_decompress_safe_usingDict(
                compressed.as_ptr() as *const i8, decomp4.as_mut_ptr() as *mut i8,
                comp_size, src_size, dict_data.as_ptr() as *const i8, dict_data.len() as i32,
            );

            let iter_d = fold(
                fold(fold(comp_size as u64, d1 as u64), fold(d2 as u64, d3 as u64)),
                fold(d4 as u64, xor_fold_bytes(&decomp1[..std::cmp::max(0, d1) as usize])),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=extdict_decompress err={} digest={:016x}", last_err, digest)
}

// ── op: streaming_multiblock ──
pub fn op_streaming_multiblock(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: i32 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len();
            if src_size == 0 { digest = fold(digest, 0); continue; }

            let chunk_size = 256usize;
            let dict_data = b"multiblock_dict_data_for_streaming_test_0123456789";

            let stream = LZ4_createStream();
            if stream.is_null() { digest = fold(digest, 0); continue; }
            LZ4_loadDict(stream, dict_data.as_ptr() as *const i8, dict_data.len() as i32);

            let mut chunks: Vec<(Vec<u8>, i32, i32)> = Vec::new();
            let mut offset = 0usize;
            let mut had_error = false;
            while offset < src_size {
                let end = std::cmp::min(offset + chunk_size, src_size);
                let chunk = &data[offset..end];
                let chunk_len = chunk.len() as i32;
                let bound = LZ4_compressBound(chunk_len);
                if bound <= 0 { had_error = true; break; }
                let mut comp = vec![0u8; bound as usize];
                let c = LZ4_compress_fast_continue(
                    stream, chunk.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8,
                    chunk_len, bound, 1,
                );
                if c <= 0 { last_err = c; had_error = true; break; }
                chunks.push((comp, c, chunk_len));
                offset = end;
            }
            LZ4_freeStream(stream);

            if had_error { digest = fold(digest, last_err as u64); continue; }

            let sd = LZ4_createStreamDecode();
            if sd.is_null() { digest = fold(digest, 0); continue; }
            LZ4_setStreamDecode(sd, dict_data.as_ptr() as *const i8, dict_data.len() as i32);

            let ring_sz = LZ4_decoderRingBufferSize(chunk_size as i32);
            let mut ring_buf = vec![0u8; ring_sz as usize];
            let mut ring_off = 0usize;
            let mut total_d: u64 = 0;

            for (comp, comp_size, orig_size) in &chunks {
                if ring_off + *orig_size as usize > ring_buf.len() {
                    ring_off = 0;
                }
                let d = LZ4_decompress_safe_continue(
                    sd, comp.as_ptr() as *const i8,
                    ring_buf.as_mut_ptr().add(ring_off) as *mut i8,
                    *comp_size, *orig_size,
                );
                if d < 0 { last_err = d; }
                total_d = fold(total_d, d as u64);
                if d > 0 { ring_off += d as usize; }
            }

            LZ4_freeStreamDecode(sd);

            let hc_stream = LZ4_createStreamHC();
            if !hc_stream.is_null() {
                LZ4_resetStreamHC(hc_stream, 4);
                LZ4_loadDictHC(hc_stream, dict_data.as_ptr() as *const i8, dict_data.len() as i32);

                let mut hc_total: u64 = 0;
                let mut offset2 = 0usize;
                while offset2 < src_size {
                    let end = std::cmp::min(offset2 + chunk_size, src_size);
                    let chunk = &data[offset2..end];
                    let chunk_len = chunk.len() as i32;
                    let bound = LZ4_compressBound(chunk_len);
                    if bound <= 0 { break; }
                    let mut comp = vec![0u8; bound as usize];
                    let c = LZ4_compress_HC_continue(
                        hc_stream, chunk.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8,
                        chunk_len, bound,
                    );
                    hc_total = fold(hc_total, c as u64);
                    offset2 = end;
                }
                LZ4_freeStreamHC(hc_stream);
                total_d = fold(total_d, hc_total);
            }

            let stream2 = LZ4_createStream();
            if !stream2.is_null() {
                LZ4_loadDict(stream2, dict_data.as_ptr() as *const i8, dict_data.len() as i32);
                let bound = LZ4_compressBound(src_size as i32);
                if bound > 0 {
                    let mut comp = vec![0u8; bound as usize];
                    let c = LZ4_compress_forceExtDict(
                        stream2, data.as_ptr() as *const i8, comp.as_mut_ptr() as *mut i8, src_size as i32,
                    );
                    total_d = fold(total_d, c as u64);
                }
                LZ4_freeStream(stream2);
            }

            digest = fold(digest, total_d);
        }
    }
    format!("op=streaming_multiblock err={} digest={:016x}", last_err, digest)
}

// ── op: frame_content_checksum ──
pub fn op_frame_content_checksum(data: &[u8], iters: u64) -> String {
    let mut digest: u64 = 0;
    let mut last_err: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let src_size = data.len();

            let mut prefs: LZ4F_preferences_t = std::mem::zeroed();
            prefs.frameInfo.contentChecksumFlag = 1;
            prefs.frameInfo.blockChecksumFlag = 1;
            prefs.frameInfo.contentSize = src_size as u64;
            prefs.frameInfo.blockSizeID = 4;
            prefs.compressionLevel = 0;

            let frame_bound = LZ4F_compressFrameBound(src_size, &prefs);
            if frame_bound == 0 { digest = fold(digest, 0); continue; }

            let mut compressed = vec![0u8; frame_bound];
            let comp_size = LZ4F_compressFrame(
                compressed.as_mut_ptr() as *mut _, frame_bound,
                data.as_ptr() as *const _, src_size, &prefs,
            );
            if LZ4F_isError(comp_size) != 0 {
                last_err = comp_size as u64;
                digest = fold(digest, comp_size as u64);
                continue;
            }

            let mut dctx: *mut LZ4F_dctx = ptr::null_mut();
            let ret = LZ4F_createDecompressionContext(&mut dctx, 100);
            if LZ4F_isError(ret) != 0 || dctx.is_null() {
                last_err = ret as u64;
                digest = fold(digest, ret as u64);
                continue;
            }

            let decomp_cap = src_size + 4096;
            let mut decompressed = vec![0u8; decomp_cap];
            let mut total_dst = 0usize;
            let mut total_src = 0usize;
            let dopts: LZ4F_decompressOptions_t = std::mem::zeroed();

            let feed_chunk = 128;
            while total_src < comp_size {
                let src_remaining = comp_size - total_src;
                let mut src_chunk = std::cmp::min(feed_chunk, src_remaining);
                let dst_remaining = decomp_cap.saturating_sub(total_dst);
                let mut dst_chunk = dst_remaining;

                let result = LZ4F_decompress(
                    dctx,
                    decompressed.as_mut_ptr().add(total_dst) as *mut _, &mut dst_chunk,
                    compressed.as_ptr().add(total_src) as *const _, &mut src_chunk,
                    &dopts,
                );
                if LZ4F_isError(result) != 0 {
                    last_err = result as u64;
                    break;
                }
                total_src += src_chunk;
                total_dst += dst_chunk;
                if result == 0 { break; }
            }

            LZ4F_freeDecompressionContext(dctx);

            let mut prefs2: LZ4F_preferences_t = std::mem::zeroed();
            prefs2.frameInfo.contentChecksumFlag = 1;
            prefs2.frameInfo.blockChecksumFlag = 1;
            prefs2.frameInfo.contentSize = src_size as u64;
            prefs2.compressionLevel = 9;

            let frame_bound2 = LZ4F_compressFrameBound(src_size, &prefs2);
            let mut compressed2 = vec![0u8; frame_bound2];
            let comp_size2 = LZ4F_compressFrame(
                compressed2.as_mut_ptr() as *mut _, frame_bound2,
                data.as_ptr() as *const _, src_size, &prefs2,
            );
            let c2_val = if LZ4F_isError(comp_size2) == 0 { comp_size2 } else { 0 };

            let mut prefs3: LZ4F_preferences_t = std::mem::zeroed();
            prefs3.frameInfo.blockMode = 0;
            prefs3.frameInfo.contentChecksumFlag = 1;
            prefs3.compressionLevel = 6;

            let frame_bound3 = LZ4F_compressFrameBound(src_size, &prefs3);
            let mut compressed3 = vec![0u8; frame_bound3];
            let comp_size3 = LZ4F_compressFrame(
                compressed3.as_mut_ptr() as *mut _, frame_bound3,
                data.as_ptr() as *const _, src_size, &prefs3,
            );
            let c3_val = if LZ4F_isError(comp_size3) == 0 { comp_size3 } else { 0 };

            let iter_d = fold(
                fold(comp_size as u64, fold(total_dst as u64, c2_val as u64)),
                fold(c3_val as u64, xor_fold_bytes(&decompressed[..total_dst])),
            );
            digest = fold(digest, iter_d);
        }
    }
    format!("op=frame_content_checksum err={} digest={:016x}", last_err, digest)
}

pub fn ops() -> Vec<(&'static str, fn(&[u8], u64) -> String)> {
    vec![
        ("compress_decompress", op_compress_decompress as fn(&[u8], u64) -> String),
        ("compress_hc_decompress", op_compress_hc_decompress as fn(&[u8], u64) -> String),
        ("frame_round_trip", op_frame_round_trip as fn(&[u8], u64) -> String),
        ("streaming_compress", op_streaming_compress as fn(&[u8], u64) -> String),
        ("xxhash_digest", op_xxhash_digest as fn(&[u8], u64) -> String),
        ("legacy_compress", op_legacy_compress as fn(&[u8], u64) -> String),
        ("compress_destsize", op_compress_destsize as fn(&[u8], u64) -> String),
        ("decompress_fast_roundtrip", op_decompress_fast_roundtrip as fn(&[u8], u64) -> String),
        ("decompress_usingdict", op_decompress_usingdict as fn(&[u8], u64) -> String),
        ("streaming_hc_compress", op_streaming_hc_compress as fn(&[u8], u64) -> String),
        ("frame_streaming_round_trip", op_frame_streaming_round_trip as fn(&[u8], u64) -> String),
        ("frame_dict_round_trip", op_frame_dict_round_trip as fn(&[u8], u64) -> String),
        ("frame_decompress_usingdict", op_frame_decompress_usingdict as fn(&[u8], u64) -> String),
        ("streaming_decode", op_streaming_decode as fn(&[u8], u64) -> String),
        ("attach_dictionary", op_attach_dictionary as fn(&[u8], u64) -> String),
        ("hc_legacy", op_hc_legacy as fn(&[u8], u64) -> String),
        ("frame_misc", op_frame_misc as fn(&[u8], u64) -> String),
        ("compress_extstate", op_compress_extstate as fn(&[u8], u64) -> String),
        ("file_io_round_trip", op_file_io_round_trip as fn(&[u8], u64) -> String),
        ("frame_hc_variants", op_frame_hc_variants as fn(&[u8], u64) -> String),
        ("prefix64k_decompress", op_prefix64k_decompress as fn(&[u8], u64) -> String),
        ("extdict_decompress", op_extdict_decompress as fn(&[u8], u64) -> String),
        ("streaming_multiblock", op_streaming_multiblock as fn(&[u8], u64) -> String),
        ("frame_content_checksum", op_frame_content_checksum as fn(&[u8], u64) -> String),
    ]
}

pub fn gen_seeds(dir: &str) -> Vec<String> {
    let mut lines = Vec::new();

    // Pattern 1: 1024 bytes, 0..255 repeated 4 times (compressible)
    let mut pat1 = Vec::with_capacity(1024);
    for _ in 0..4 { for b in 0u8..=255 { pat1.push(b); } }

    // Pattern 2: 2048 bytes of highly repetitive data
    let mut pat2 = Vec::with_capacity(2048);
    for _ in 0..2048 { pat2.push(b'A'); }

    // Pattern 3: semi-random (less compressible)
    let mut pat3 = Vec::with_capacity(4096);
    let mut val: u32 = 0xDEADBEEF;
    for _ in 0..4096 {
        val = val.wrapping_mul(1103515245).wrapping_add(12345);
        pat3.push((val >> 16) as u8);
    }

    // Pattern 4: data with repeating 4-byte patterns
    let mut pat4 = Vec::with_capacity(8192);
    let pattern: [u8; 4] = [0xAB, 0xCD, 0xEF, 0x01];
    for _ in 0..2048 { pat4.extend_from_slice(&pattern); }

    // Pattern 5: mixed
    let mut pat5 = Vec::with_capacity(16384);
    pat5.extend_from_slice(b"attach_dict_test_data_0123456789abcdefghijklmnopqrstuvwxyz");
    for _ in 0..200 { pat5.extend_from_slice(b"ABCDEFGHIJKLMNOP"); }
    let mut v2: u32 = 0x42424242;
    for _ in 0..4096 {
        v2 = v2.wrapping_mul(1103515245).wrapping_add(12345);
        pat5.push((v2 >> 16) as u8);
    }
    for _ in 0..200 { pat5.extend_from_slice(b"ZYXWVUTSRQPONMLK"); }

    // Pattern 6: large data > 64KB
    let mut pat6 = Vec::with_capacity(70000);
    let mut v3: u32 = 0x13579BDF;
    for i in 0..70000u32 {
        if i % 100 < 50 {
            pat6.push(b'X');
        } else {
            v3 = v3.wrapping_mul(1103515245).wrapping_add(12345);
            pat6.push((v3 >> 16) as u8);
        }
    }

    let op_names: Vec<&str> = ops().iter().map(|(n, _)| *n).collect();

    for op in &op_names {
        let path1 = format!("{}/{}.1.bin", dir, op);
        let path2 = format!("{}/{}.2.bin", dir, op);
        fs::write(&path1, &pat1).unwrap();
        fs::write(&path2, &pat3).unwrap();
        lines.push(format!("seed {} {}", op, path1));
        lines.push(format!("seed {} {}", op, path2));
    }

    let repetitive_ops = [
        "compress_decompress", "compress_hc_decompress", "frame_round_trip",
        "streaming_compress", "hc_legacy", "frame_hc_variants",
        "prefix64k_decompress", "extdict_decompress", "streaming_multiblock",
        "frame_content_checksum", "streaming_hc_compress",
    ];
    for op in &repetitive_ops {
        let path3 = format!("{}/{}.3.bin", dir, op);
        fs::write(&path3, &pat2).unwrap();
        lines.push(format!("seed {} {}", op, path3));
    }

    let pattern_ops = [
        "compress_hc_decompress", "hc_legacy", "streaming_hc_compress",
        "frame_hc_variants", "extdict_decompress", "streaming_multiblock",
    ];
    for op in &pattern_ops {
        let path4 = format!("{}/{}.4.bin", dir, op);
        fs::write(&path4, &pat4).unwrap();
        lines.push(format!("seed {} {}", op, path4));
    }

    let mixed_ops = [
        "compress_hc_decompress", "streaming_hc_compress", "frame_hc_variants",
        "attach_dictionary", "streaming_multiblock", "extdict_decompress",
    ];
    for op in &mixed_ops {
        let path5 = format!("{}/{}.5.bin", dir, op);
        fs::write(&path5, &pat5).unwrap();
        lines.push(format!("seed {} {}", op, path5));
    }

    let large_ops = [
        "extdict_decompress", "streaming_multiblock", "prefix64k_decompress",
        "streaming_compress", "streaming_hc_compress", "frame_content_checksum",
    ];
    for op in &large_ops {
        let path6 = format!("{}/{}.6.bin", dir, op);
        fs::write(&path6, &pat6).unwrap();
        lines.push(format!("seed {} {}", op, path6));
    }

    lines
}

pub fn gen_perf(dir: &str) -> Vec<String> {
    let mut lines = Vec::new();

                         
    //
                                                     
                                                           
                                                          
                                                     
                                             
    //
                                                   
                                      
    //
                                                      
                                                   
                                             
    let size = 512 * 1024;
    let mut val: u32 = 0x12345678;
    let mut rnd = move || {
        val = val.wrapping_mul(1103515245).wrapping_add(12345);
        ((val >> 16) & 0x7fff) as usize
    };

                                   
    let mut vocab: Vec<Vec<u8>> = Vec::with_capacity(96);
    for _ in 0..96 {
        let len = 3 + rnd() % 8;
        let mut w = Vec::with_capacity(len);
        for _ in 0..len { w.push(b'a' + (rnd() % 26) as u8); }
        vocab.push(w);
    }

    let mut big: Vec<u8> = Vec::with_capacity(size);
    while big.len() < size {
                                        
                                                         
        if big.len() > 4096 && rnd() % 24 == 0 {
            let span = 32 + rnd() % 200;
            let back = 1 + rnd() % (big.len() / 2);
            let start = big.len() - back;
            let end = core::cmp::min(start + span, big.len());
            let piece = big[start..end].to_vec();
            big.extend_from_slice(&piece);
            continue;
        }
                                               
        let i = if rnd() % 4 == 0 { rnd() % vocab.len() } else { rnd() % (vocab.len() / 4) };
        big.extend_from_slice(&vocab[i]);
        big.push(if rnd() % 16 == 0 { b'\n' } else { b' ' });
    }
    big.truncate(size);

    let perf_ops = [
        "compress_decompress", "compress_hc_decompress", "frame_round_trip",
        "streaming_compress", "xxhash_digest", "legacy_compress",
        "compress_destsize", "decompress_fast_roundtrip", "decompress_usingdict",
        "streaming_hc_compress", "frame_streaming_round_trip",
        "frame_dict_round_trip", "frame_decompress_usingdict",
        "streaming_decode", "attach_dictionary", "hc_legacy",
        "frame_misc", "compress_extstate",
        "file_io_round_trip", "frame_hc_variants",
        "prefix64k_decompress", "extdict_decompress",
        "streaming_multiblock", "frame_content_checksum",
    ];

    for op in &perf_ops {
        let path = format!("{}/{}.perf.bin", dir, op);
        fs::write(&path, &big).unwrap();
        lines.push(format!("perf {} {}", op, path));
    }

    lines
}
