// ============================================================================
                                                                    
                                                            
                                        
                                                                  
// ============================================================================

use core::ffi::{c_char, c_int, c_void};
use std::process::exit;

#[inline]
fn digest64(seed: u64, p: &[u8]) -> u64 {
    let mut l0 = seed ^ 0xcbf2_9ce4_8422_2325u64;
    let mut l1 = 0x8422_2325_cbf2_9ce4u64;
    let mut l2 = 0x9ce4_8422_2325_cbf2u64;
    let mut l3 = 0x2325_cbf2_9ce4_8422u64;
    let n = p.len();
    let mut i = 0usize;
    while i + 32 <= n {
        let w0 = u64::from_ne_bytes(p[i..i + 8].try_into().unwrap());
        let w1 = u64::from_ne_bytes(p[i + 8..i + 16].try_into().unwrap());
        let w2 = u64::from_ne_bytes(p[i + 16..i + 24].try_into().unwrap());
        let w3 = u64::from_ne_bytes(p[i + 24..i + 32].try_into().unwrap());
        l0 = (l0 ^ w0).rotate_left(17);
        l1 = (l1 ^ w1).rotate_left(19);
        l2 = (l2 ^ w2).rotate_left(23);
        l3 = (l3 ^ w3).rotate_left(29);
        i += 32;
    }
    while i < n {
        l0 = (l0 ^ p[i] as u64).rotate_left(11);
        i += 1;
    }
    let mut h = l0.wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
    h ^= l1.rotate_left(1);
    h = h.wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
    h ^= l2.rotate_left(2);
    h = h.wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
    h ^= l3.rotate_left(3);
    h = h.wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
    h
}

                                                
#[inline]
unsafe fn digest_raw(h: u64, p: *const c_void, n: usize) -> u64 {
    digest64(h, std::slice::from_raw_parts(p as *const u8, n))
}

                                                       
#[inline]
fn u64_bytes(s: &[u64]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, std::mem::size_of_val(s)) }
}

const SLICE: usize = 65536;

/*                                                       */
static mut FIXED_TIME: libcall::time_t = 1_750_000_000;

unsafe fn zip_add_slice(
    zip: *mut libcall::mz_zip_archive,
    name: *const c_char,
    data: *const u8,
    size: usize,
    level: u32,
) -> c_int {
    libcall::mz_zip_writer_add_mem_ex_v2(
        zip,
        name,
        data as *const c_void,
        size,
        std::ptr::null(),
        0,
        level,
        0,
        0,
        std::ptr::addr_of_mut!(FIXED_TIME),
        std::ptr::null(),
        0,
        std::ptr::null(),
        0,
    )
}

                           
macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

fn read_file_or_die(path: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("cannot read {}", path);
            exit(2);
        }
    }
}

/* -------------------------------------------------------------------- */

fn run_compress(src: &[u8], n: usize, iters: i64) {
    unsafe {
        let bound = libcall::mz_compressBound(n as u64);
        let mut dst = vec![0u8; bound as usize];
        let mut h: u64 = 0;
        let mut outlen: u64 = 0;
        for i in 0..iters {
            outlen = bound;
            /* alternate the two equivalent one-shot entry points (same output) */
            let r = if i & 1 != 0 {
                libcall::mz_compress(dst.as_mut_ptr(), &mut outlen, src.as_ptr(), n as u64)
            } else {
                libcall::mz_compress2(
                    dst.as_mut_ptr(),
                    &mut outlen,
                    src.as_ptr(),
                    n as u64,
                    libcall::MZ_DEFAULT_LEVEL as c_int,
                )
            };
            if r != libcall::MZ_OK as c_int {
                eprintln!("compress failed: {}", r);
                exit(3);
            }
            h = digest64(h, &dst[..outlen as usize]);
        }
        println!("op=compress in={} out={} iters={} digest={:016x}", n, outlen, iters, h);
    }
}

/* .z file: u64 LE orig size + zlib stream */
fn run_uncompress(file: &[u8], file_n: usize, iters: i64) {
    if file_n < 8 {
        eprintln!("bad .z input");
        exit(2);
    }
    let orig = u64::from_ne_bytes(file[0..8].try_into().unwrap());
    let mut dst = vec![0u8; orig as usize];
    let mut h: u64 = 0;
    unsafe {
        for _ in 0..iters {
            let mut dlen: u64 = orig;
            let r = libcall::mz_uncompress(
                dst.as_mut_ptr(),
                &mut dlen,
                file.as_ptr().add(8),
                (file_n - 8) as u64,
            );
            if r != libcall::MZ_OK as c_int || dlen != orig {
                eprintln!("uncompress failed: {}", r);
                exit(3);
            }
            h = digest64(h, &dst[..dlen as usize]);
        }
    }
    println!("op=uncompress in={} out={} iters={} digest={:016x}", file_n, orig, iters, h);
}

fn run_deflate_stream(src: &[u8], n: usize, iters: i64) {
    unsafe {
        let mut out = vec![0u8; SLICE];
        let mut h: u64 = 0;
        let mut total: u64 = 0;
        let mut s: libcall::mz_stream = std::mem::zeroed();
        if libcall::mz_deflateInit(&mut s, libcall::MZ_DEFAULT_LEVEL as c_int)
            != libcall::MZ_OK as c_int
        {
            eprintln!("deflateInit failed");
            exit(3);
        }
        for _ in 0..iters {
            if libcall::mz_deflateReset(&mut s) != libcall::MZ_OK as c_int {
                eprintln!("deflateReset failed");
                exit(3);
            }
            total = 0;
            let mut pos = 0usize;
            loop {
                let chunk = if n - pos < SLICE { n - pos } else { SLICE };
                s.next_in = src.as_ptr().add(pos);
                s.avail_in = chunk as u32;
                pos += chunk;
                let flush = if pos >= n {
                    libcall::MZ_FINISH as c_int
                } else {
                    libcall::MZ_NO_FLUSH as c_int
                };
                loop {
                    s.next_out = out.as_mut_ptr();
                    s.avail_out = SLICE as u32;
                    let r = libcall::mz_deflate(&mut s, flush);
                    if r != libcall::MZ_OK as c_int && r != libcall::MZ_STREAM_END as c_int {
                        eprintln!("deflate failed: {}", r);
                        exit(3);
                    }
                    let got = SLICE - s.avail_out as usize;
                    h = digest64(h, &out[..got]);
                    total += got as u64;
                    if s.avail_out != 0 {
                        break;
                    }
                }
                if flush == libcall::MZ_FINISH as c_int {
                    break;
                }
            }
        }
        libcall::mz_deflateEnd(&mut s);
        println!("op=deflate_stream in={} out={} iters={} digest={:016x}", n, total, iters, h);
    }
}

fn run_inflate_stream(file: &[u8], file_n: usize, iters: i64) {
    if file_n < 8 {
        eprintln!("bad .z input");
        exit(2);
    }
    let orig = u64::from_ne_bytes(file[0..8].try_into().unwrap());
    unsafe {
        let mut out = vec![0u8; SLICE];
        let mut h: u64 = 0;
        let mut total: u64 = 0;
        let mut s: libcall::mz_stream = std::mem::zeroed();
        if libcall::mz_inflateInit(&mut s) != libcall::MZ_OK as c_int {
            eprintln!("inflateInit failed");
            exit(3);
        }
        for _ in 0..iters {
            if libcall::mz_inflateReset(&mut s) != libcall::MZ_OK as c_int {
                eprintln!("inflateReset failed");
                exit(3);
            }
            total = 0;
            s.next_in = file.as_ptr().add(8);
            s.avail_in = (file_n - 8) as u32;
            loop {
                s.next_out = out.as_mut_ptr();
                s.avail_out = SLICE as u32;
                let r = libcall::mz_inflate(&mut s, libcall::MZ_NO_FLUSH as c_int);
                if r != libcall::MZ_OK as c_int && r != libcall::MZ_STREAM_END as c_int {
                    eprintln!("inflate failed: {}", r);
                    exit(3);
                }
                let got = SLICE - s.avail_out as usize;
                h = digest64(h, &out[..got]);
                total += got as u64;
                if r == libcall::MZ_STREAM_END as c_int {
                    break;
                }
            }
            if total != orig {
                eprintln!("inflate size mismatch");
                exit(3);
            }
        }
        libcall::mz_inflateEnd(&mut s);
        println!("op=inflate_stream in={} out={} iters={} digest={:016x}", file_n, total, iters, h);
    }
}

/* low-level tdefl over a flag matrix: probe counts, greedy, RLE, zlib header */
fn run_tdefl_matrix(src: &[u8], n: usize, iters: i64) {
    let flag_sets: [c_int; 5] = [
        1,                                                     /* 1 probe, fastest */
        128 | libcall::TDEFL_GREEDY_PARSING_FLAG as c_int,     /* greedy, mid probes */
        libcall::TDEFL_WRITE_ZLIB_HEADER as c_int | 256,       /* zlib wrapper */
        libcall::TDEFL_RLE_MATCHES as c_int | 1,               /* RLE-only matches */
        libcall::TDEFL_FORCE_ALL_STATIC_BLOCKS as c_int | 64,  /* static-huffman blocks */
    ];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    unsafe {
        for _ in 0..iters {
            total = 0;
            for k in 0..5usize {
                let mut outlen: usize = 0;
                let out = libcall::tdefl_compress_mem_to_heap(
                    src.as_ptr() as *const c_void,
                    n,
                    &mut outlen,
                    flag_sets[k],
                );
                if out.is_null() {
                    eprintln!("tdefl failed");
                    exit(3);
                }
                h = digest_raw(h, out, outlen);
                total += outlen as u64;
                libcall::mz_free(out);
            }
            /* fixed-buffer variant (same first flag set, output must fit) */
            {
                let mut fixed = vec![0u8; n + 1024];
                let got = libcall::tdefl_compress_mem_to_mem(
                    fixed.as_mut_ptr() as *mut c_void,
                    n + 1024,
                    src.as_ptr() as *const c_void,
                    n,
                    1,
                );
                if got == 0 {
                    eprintln!("tdefl mem_to_mem failed");
                    exit(3);
                }
                h = digest64(h, &fixed[..got]);
            }
            /* heap compressor lifecycle + status query */
            {
                let c = libcall::tdefl_compressor_alloc();
                if c.is_null() {
                    eprintln!("tdefl alloc failed");
                    exit(3);
                }
                libcall::tdefl_init(c, None, std::ptr::null_mut(), 1);
                let st = libcall::tdefl_get_prev_return_status(c) as u64;
                h = digest64(h, &st.to_ne_bytes());
                libcall::tdefl_compressor_free(c);
            }
            /* PNG writer (tdefl-based, deterministic from input bytes) */
            {
                let mut png_n: usize = 0;
                let png = libcall::tdefl_write_image_to_png_file_in_memory_ex(
                    src.as_ptr() as *const c_void,
                    64,
                    64,
                    3,
                    &mut png_n,
                    6,
                    0,
                );
                if png.is_null() {
                    eprintln!("tdefl png failed");
                    exit(3);
                }
                h = digest_raw(h, png, png_n);
                libcall::mz_free(png);
                let mut png2_n: usize = 0;
                let png2 = libcall::tdefl_write_image_to_png_file_in_memory(
                    src.as_ptr() as *const c_void,
                    64,
                    64,
                    3,
                    &mut png2_n,
                );
                if png2.is_null() {
                    eprintln!("tdefl png2 failed");
                    exit(3);
                }
                h = digest_raw(h, png2, png2_n);
                libcall::mz_free(png2);
            }
        }
    }
    println!("op=tdefl_matrix in={} out={} iters={} digest={:016x}", n, total, iters, h);
}

unsafe extern "C" fn tinfl_count_cb(_pbuf: *const c_void, len: c_int, puser: *mut c_void) -> c_int {
    *(puser as *mut u64) += len as u64;
    1
}

/* mz_file_write_func-shaped counter for the extract_to_callback variants */
unsafe extern "C" fn zip_count_cb(puser: *mut c_void, _ofs: u64, _pbuf: *const c_void, n: usize) -> usize {
    *(puser as *mut u64) += n as u64;
    n
}

/* mz_file_read_func-shaped memory reader for add_read_buf_callback */
#[repr(C)]
struct MemReader {
    p: *const u8,
    n: usize,
}
unsafe extern "C" fn zip_read_cb(puser: *mut c_void, ofs: u64, pbuf: *mut c_void, mut n: usize) -> usize {
    let m = &mut *(puser as *mut MemReader);
    if ofs >= m.n as u64 {
        return 0;
    }
    let avail = m.n - ofs as usize;
    if n > avail {
        n = avail;
    }
    std::ptr::copy_nonoverlapping(m.p.add(ofs as usize), pbuf as *mut u8, n);
    n
}

/* .defl file: u64 LE orig size + raw deflate stream */
fn run_tinfl_lowlevel(file: &[u8], file_n: usize, iters: i64) {
    if file_n < 8 {
        eprintln!("bad .defl input");
        exit(2);
    }
    let orig = u64::from_ne_bytes(file[0..8].try_into().unwrap());
    let mut h: u64 = 0;
    let mut fixed = vec![0u8; orig as usize];
    unsafe {
        for _ in 0..iters {
            let mut outlen: usize = 0;
            let out = libcall::tinfl_decompress_mem_to_heap(
                file.as_ptr().add(8) as *const c_void,
                file_n - 8,
                &mut outlen,
                0,
            );
            if out.is_null() || outlen as u64 != orig {
                eprintln!("tinfl failed");
                exit(3);
            }
            h = digest_raw(h, out, outlen);
            libcall::mz_free(out);
            /* fixed-buffer variant */
            let got = libcall::tinfl_decompress_mem_to_mem(
                fixed.as_mut_ptr() as *mut c_void,
                orig as usize,
                file.as_ptr().add(8) as *const c_void,
                file_n - 8,
                0,
            );
            if got as u64 != orig {
                eprintln!("tinfl mem_to_mem failed");
                exit(3);
            }
            h = digest64(h, &fixed[..got]);
        }
        /* callback variant + decompressor lifecycle (once; folds via counter) */
        {
            let mut cb_total: u64 = 0;
            let mut in_n: usize = file_n - 8;
            if libcall::tinfl_decompress_mem_to_callback(
                file.as_ptr().add(8) as *const c_void,
                &mut in_n,
                Some(tinfl_count_cb),
                &mut cb_total as *mut u64 as *mut c_void,
                0,
            ) == 0
            {
                eprintln!("tinfl callback failed");
                exit(3);
            }
            h = digest64(h, &cb_total.to_ne_bytes());
            let d = libcall::tinfl_decompressor_alloc();
            if d.is_null() {
                eprintln!("tinfl alloc failed");
                exit(3);
            }
            libcall::tinfl_decompressor_free(d);
        }
    }
    println!("op=tinfl_lowlevel in={} out={} iters={} digest={:016x}", file_n, orig, iters, h);
}

fn run_checksums(src: &[u8], n: usize, iters: i64) {
    let mut h: u64 = 0;
    unsafe {
        for _ in 0..iters {
            let mut acc = [0u64; 3];
            acc[0] = libcall::mz_crc32(libcall::MZ_CRC32_INIT as u64, src.as_ptr(), n);
            acc[1] = libcall::mz_adler32(libcall::MZ_ADLER32_INIT as u64, src.as_ptr(), n);
            acc[2] = std::ffi::CStr::from_ptr(libcall::mz_error(libcall::MZ_MEM_ERROR as c_int))
                .to_bytes()
                .len() as u64;
            h = digest64(h, u64_bytes(&acc));
        }
    }
    println!("op=checksums in={} out=0 iters={} digest={:016x}", n, iters, h);
}

/* build an in-memory ZIP from 64KB slices of the input, then re-read it */
fn run_zip_write(src: &[u8], n: usize, iters: i64) {
    let mut h: u64 = 0;
    let mut zipsize: usize = 0;
    unsafe {
        for _ in 0..iters {
            let mut zip: libcall::mz_zip_archive = std::mem::zeroed();
            libcall::mz_zip_zero_struct(&mut zip);
            if libcall::mz_zip_writer_init_heap(&mut zip, 0, 1 << 16) == 0 {
                eprintln!("zip init failed");
                exit(3);
            }
            let mut idx: u32 = 0;
            let mut off = 0usize;
            while off < n {
                let chunk = if n - off < SLICE { n - off } else { SLICE };
                let name = format!("slice_{:04}.bin\0", idx);
                if zip_add_slice(
                    &mut zip,
                    name.as_ptr() as *const c_char,
                    src.as_ptr().add(off),
                    chunk,
                    libcall::MZ_DEFAULT_LEVEL as u32,
                ) == 0
                {
                    eprintln!("zip add failed");
                    exit(3);
                }
                off += SLICE;
                idx += 1;
            }
            let mut buf: *mut c_void = std::ptr::null_mut();
            if libcall::mz_zip_writer_finalize_heap_archive(&mut zip, &mut buf, &mut zipsize) == 0 {
                eprintln!("zip finalize failed");
                exit(3);
            }
            libcall::mz_zip_writer_end(&mut zip);
            h = digest_raw(h, buf, zipsize);
            libcall::mz_free(buf);
        }
    }
    println!("op=zip_write in={} out={} iters={} digest={:016x}", n, zipsize, iters, h);
}

/* .zip file: stat + locate + extract every entry */
fn run_zip_read(file: &[u8], file_n: usize, iters: i64) {
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    unsafe {
        for _ in 0..iters {
            let mut zip: libcall::mz_zip_archive = std::mem::zeroed();
            libcall::mz_zip_zero_struct(&mut zip);
            if libcall::mz_zip_reader_init_mem(
                &mut zip,
                file.as_ptr() as *const c_void,
                file_n,
                0,
            ) == 0
            {
                eprintln!("zip reader init failed");
                exit(3);
            }
            let nfiles = libcall::mz_zip_reader_get_num_files(&mut zip);
            total = 0;
            for f in 0..nfiles {
                let mut st: libcall::mz_zip_archive_file_stat = std::mem::zeroed();
                if libcall::mz_zip_reader_file_stat(&mut zip, f, &mut st) == 0 {
                    eprintln!("zip stat failed");
                    exit(3);
                }
                let meta: [u64; 3] = [st.m_uncomp_size, st.m_comp_size, st.m_crc32 as u64];
                h = digest64(h, u64_bytes(&meta));
                if libcall::mz_zip_reader_is_file_a_directory(&mut zip, f) != 0 {
                    continue;
                }
                let loc = libcall::mz_zip_reader_locate_file(
                    &mut zip,
                    st.m_filename.as_ptr(),
                    std::ptr::null(),
                    0,
                );
                if loc != f as c_int {
                    eprintln!("zip locate mismatch");
                    exit(3);
                }
                let mut outlen: usize = 0;
                let out = libcall::mz_zip_reader_extract_to_heap(&mut zip, f, &mut outlen, 0);
                if out.is_null() {
                    eprintln!("zip extract failed");
                    exit(3);
                }
                h = digest_raw(h, out, outlen);
                total += outlen as u64;
                libcall::mz_free(out);
            }

            /* archive-level queries + error surface */
            {
                let mut q = [0u64; 8];
                q[0] = libcall::mz_zip_get_archive_size(&mut zip);
                q[1] = libcall::mz_zip_get_central_dir_size(&mut zip) as u64;
                q[2] = libcall::mz_zip_get_archive_file_start_offset(&mut zip);
                q[3] = libcall::mz_zip_is_zip64(&mut zip) as u64;
                q[4] = libcall::mz_zip_get_mode(&mut zip) as u64
                    | ((libcall::mz_zip_get_type(&mut zip) as u64) << 8);
                q[5] = libcall::mz_zip_peek_last_error(&mut zip) as u64;
                q[6] = libcall::mz_zip_clear_last_error(&mut zip) as u64;
                q[7] = libcall::mz_zip_get_last_error(&mut zip) as u64
                    + std::ffi::CStr::from_ptr(libcall::mz_zip_get_error_string(
                        libcall::MZ_ZIP_UNDEFINED_ERROR,
                    ))
                    .to_bytes()
                    .len() as u64;
                h = digest64(h, u64_bytes(&q));
                let _ = libcall::mz_zip_get_cfile(&mut zip);
                let mut hdr = [0u8; 64];
                let got = libcall::mz_zip_read_archive_data(
                    &mut zip,
                    0,
                    hdr.as_mut_ptr() as *mut c_void,
                    hdr.len(),
                );
                h = digest64(h, &hdr[..got]);
                if libcall::mz_zip_validate_archive(&mut zip, 0) == 0 {
                    eprintln!("zip validate failed");
                    exit(3);
                }
            }

            /* entry-0 extract variants (all must agree with extract_to_heap) */
            if nfiles > 0 {
                let mut st0: libcall::mz_zip_archive_file_stat = std::mem::zeroed();
                libcall::mz_zip_reader_file_stat(&mut zip, 0, &mut st0);
                let mut fname = [0 as c_char; 260];
                libcall::mz_zip_reader_get_filename(
                    &mut zip,
                    0,
                    fname.as_mut_ptr(),
                    fname.len() as u32,
                );
                let sz0 = st0.m_uncomp_size as usize;
                let mut buf0 = vec![0u8; sz0];
                if libcall::mz_zip_reader_extract_to_mem(
                    &mut zip,
                    0,
                    buf0.as_mut_ptr() as *mut c_void,
                    sz0,
                    0,
                ) == 0
                {
                    eprintln!("extract_to_mem failed");
                    exit(3);
                }
                h = digest64(h, &buf0[..sz0]);
                if libcall::mz_zip_reader_extract_file_to_mem(
                    &mut zip,
                    fname.as_ptr(),
                    buf0.as_mut_ptr() as *mut c_void,
                    sz0,
                    0,
                ) == 0
                {
                    eprintln!("extract_file_to_mem failed");
                    exit(3);
                }
                h = digest64(h, &buf0[..sz0]);
                if libcall::mz_zip_reader_extract_to_mem_no_alloc(
                    &mut zip,
                    0,
                    buf0.as_mut_ptr() as *mut c_void,
                    sz0,
                    0,
                    std::ptr::null_mut(),
                    0,
                ) == 0
                {
                    eprintln!("no_alloc failed");
                    exit(3);
                }
                if libcall::mz_zip_reader_extract_file_to_mem_no_alloc(
                    &mut zip,
                    fname.as_ptr(),
                    buf0.as_mut_ptr() as *mut c_void,
                    sz0,
                    0,
                    std::ptr::null_mut(),
                    0,
                ) == 0
                {
                    eprintln!("file no_alloc failed");
                    exit(3);
                }
                let mut hn: usize = 0;
                let hp = libcall::mz_zip_reader_extract_file_to_heap(
                    &mut zip,
                    fname.as_ptr(),
                    &mut hn,
                    0,
                );
                if hp.is_null() {
                    eprintln!("extract_file_to_heap failed");
                    exit(3);
                }
                h = digest_raw(h, hp, hn);
                libcall::mz_free(hp);
                let mut cbt: u64 = 0;
                if libcall::mz_zip_reader_extract_to_callback(
                    &mut zip,
                    0,
                    Some(zip_count_cb),
                    &mut cbt as *mut u64 as *mut c_void,
                    0,
                ) == 0
                {
                    eprintln!("extract cb failed");
                    exit(3);
                }
                if libcall::mz_zip_reader_extract_file_to_callback(
                    &mut zip,
                    fname.as_ptr(),
                    Some(zip_count_cb),
                    &mut cbt as *mut u64 as *mut c_void,
                    0,
                ) == 0
                {
                    eprintln!("extract file cb failed");
                    exit(3);
                }
                h = digest64(h, &cbt.to_ne_bytes());
                let it = libcall::mz_zip_reader_extract_iter_new(&mut zip, 0, 0);
                if it.is_null() {
                    eprintln!("iter new failed");
                    exit(3);
                }
                let mut itot: u64 = 0;
                let mut ib = [0u8; 4096];
                loop {
                    let r = libcall::mz_zip_reader_extract_iter_read(
                        it,
                        ib.as_mut_ptr() as *mut c_void,
                        ib.len(),
                    );
                    if r == 0 {
                        break;
                    }
                    h = digest64(h, &ib[..r]);
                    itot += r as u64;
                }
                libcall::mz_zip_reader_extract_iter_free(it);
                let it2 = libcall::mz_zip_reader_extract_file_iter_new(&mut zip, fname.as_ptr(), 0);
                if !it2.is_null() {
                    let _ = libcall::mz_zip_reader_extract_iter_read(
                        it2,
                        ib.as_mut_ptr() as *mut c_void,
                        ib.len(),
                    );
                    libcall::mz_zip_reader_extract_iter_free(it2);
                }
                h = digest64(h, &itot.to_ne_bytes());
            }
            libcall::mz_zip_end(&mut zip); /* generic end (reader mode) */
        }
        /* standalone validate helper over the raw archive bytes */
        {
            let mut zerr: libcall::mz_zip_error = libcall::MZ_ZIP_NO_ERROR;
            if libcall::mz_zip_validate_mem_archive(
                file.as_ptr() as *const c_void,
                file_n,
                0,
                &mut zerr,
            ) == 0
            {
                eprintln!("validate_mem failed");
                exit(3);
            }
        }
    }
    println!("op=zip_read in={} out={} iters={} digest={:016x}", file_n, total, iters, h);
}

/* file/cfile/in-place/append entry points (page-cached scratch files).
 * DIGEST RULE: fold only extracted payloads and counts, never raw archive
 * bytes -- file-based writers stamp source mtimes into entry headers. */
fn run_zip_file(src: &[u8], n: usize, iters: i64, input_path: &str) {
    let scratch = cstr!("/tmp/miniz_vw_scratch.zip");
    let scratch2 = cstr!("/tmp/miniz_vw_scratch2.zip");
    let scratch3 = cstr!("/tmp/miniz_vw_scratch3.bin");
    let input_c = std::ffi::CString::new(input_path).unwrap();
    let mut h: u64 = 0;
    unsafe {
        for _ in 0..iters {
            /* 1. file-based writer: slices + whole input file + read-buf callback */
            let mut w: libcall::mz_zip_archive = std::mem::zeroed();
            libcall::mz_zip_zero_struct(&mut w);
            if libcall::mz_zip_writer_init_file(&mut w, scratch, 0) == 0 {
                eprintln!("zip file init failed");
                exit(3);
            }
            let part = n / 4;
            for k in 0..4usize {
                let name = format!("f{}.bin\0", k);
                if zip_add_slice(
                    &mut w,
                    name.as_ptr() as *const c_char,
                    src.as_ptr().add(k * part),
                    part,
                    libcall::MZ_BEST_SPEED as u32,
                ) == 0
                {
                    eprintln!("zip file add failed");
                    exit(3);
                }
            }
            if libcall::mz_zip_writer_add_file(
                &mut w,
                cstr!("whole.bin"),
                input_c.as_ptr(),
                std::ptr::null(),
                0,
                libcall::MZ_BEST_SPEED as u32,
            ) == 0
            {
                eprintln!("zip add_file failed");
                exit(3);
            }
            let mut mr = MemReader { p: src.as_ptr(), n: n / 8 };
            if libcall::mz_zip_writer_add_read_buf_callback(
                &mut w,
                cstr!("cb.bin"),
                Some(zip_read_cb),
                &mut mr as *mut MemReader as *mut c_void,
                (n / 8) as u64,
                std::ptr::addr_of!(FIXED_TIME),
                std::ptr::null(),
                0,
                libcall::MZ_BEST_SPEED as u32,
                std::ptr::null(),
                0,
                std::ptr::null(),
                0,
            ) == 0
            {
                eprintln!("zip add_read_buf failed");
                exit(3);
            }
            if libcall::mz_zip_writer_finalize_archive(&mut w) == 0 {
                eprintln!("zip file finalize failed");
                exit(3);
            }
            libcall::mz_zip_writer_end(&mut w);

            /* 2. in-place append helpers */
            if libcall::mz_zip_add_mem_to_archive_file_in_place(
                scratch,
                cstr!("extra1.bin"),
                src.as_ptr() as *const c_void,
                1024,
                std::ptr::null(),
                0,
                libcall::MZ_BEST_SPEED as u32,
            ) == 0
            {
                eprintln!("in_place failed");
                exit(3);
            }
            let mut zerr: libcall::mz_zip_error = libcall::MZ_ZIP_NO_ERROR;
            if libcall::mz_zip_add_mem_to_archive_file_in_place_v2(
                scratch,
                cstr!("extra2.bin"),
                src.as_ptr() as *const c_void,
                1024,
                std::ptr::null(),
                0,
                libcall::MZ_BEST_SPEED as u32,
                &mut zerr,
            ) == 0
            {
                eprintln!("in_place v2 failed");
                exit(3);
            }

            /* 3. reader -> writer append pattern */
            {
                let mut z: libcall::mz_zip_archive = std::mem::zeroed();
                libcall::mz_zip_zero_struct(&mut z);
                if libcall::mz_zip_reader_init_file(&mut z, scratch, 0) == 0 {
                    eprintln!("reader init failed");
                    exit(3);
                }
                if libcall::mz_zip_writer_init_from_reader_v2(&mut z, scratch, 0) == 0 {
                    eprintln!("init_from_reader failed");
                    exit(3);
                }
                if zip_add_slice(
                    &mut z,
                    cstr!("appended.bin"),
                    src.as_ptr(),
                    2048,
                    libcall::MZ_BEST_SPEED as u32,
                ) == 0
                {
                    eprintln!("append failed");
                    exit(3);
                }
                if libcall::mz_zip_writer_finalize_archive(&mut z) == 0 {
                    eprintln!("append finalize failed");
                    exit(3);
                }
                libcall::mz_zip_writer_end(&mut z);
            }

            /* 4. read back: file reader + extract to file / heap-by-name helpers */
            {
                let mut r: libcall::mz_zip_archive = std::mem::zeroed();
                libcall::mz_zip_zero_struct(&mut r);
                if libcall::mz_zip_reader_init_file(&mut r, scratch, 0) == 0 {
                    eprintln!("zip file reader failed");
                    exit(3);
                }
                let nf = libcall::mz_zip_reader_get_num_files(&mut r);
                for f in 0..nf {
                    let mut outlen: usize = 0;
                    let out = libcall::mz_zip_reader_extract_to_heap(&mut r, f, &mut outlen, 0);
                    if out.is_null() {
                        eprintln!("zip file extract failed");
                        exit(3);
                    }
                    h = digest_raw(h, out, outlen);
                    libcall::mz_free(out);
                }
                if libcall::mz_zip_reader_extract_to_file(&mut r, 0, scratch3, 0) == 0 {
                    eprintln!("extract_to_file failed");
                    exit(3);
                }
                if libcall::mz_zip_reader_extract_file_to_file(
                    &mut r,
                    cstr!("f1.bin"),
                    scratch3,
                    0,
                ) == 0
                {
                    eprintln!("extract_file_to_file failed");
                    exit(3);
                }
                let rb = read_file_or_die("/tmp/miniz_vw_scratch3.bin");
                h = digest64(h, &rb);
                drop(rb);
                let devnull = libcall::fopen(cstr!("/dev/null"), cstr!("wb"));
                if !devnull.is_null() {
                    if libcall::mz_zip_reader_extract_to_cfile(&mut r, 0, devnull, 0) == 0 {
                        eprintln!("extract_to_cfile failed");
                        exit(3);
                    }
                    if libcall::mz_zip_reader_extract_file_to_cfile(
                        &mut r,
                        cstr!("f1.bin"),
                        devnull,
                        0,
                    ) == 0
                    {
                        eprintln!("extract_file_to_cfile failed");
                        exit(3);
                    }
                    libcall::fclose(devnull);
                }
                libcall::mz_zip_reader_end(&mut r);
            }
            {
                let mut hn: usize = 0;
                let hp = libcall::mz_zip_extract_archive_file_to_heap(
                    scratch,
                    cstr!("f2.bin"),
                    &mut hn,
                    0,
                );
                if hp.is_null() {
                    eprintln!("archive_file_to_heap failed");
                    exit(3);
                }
                h = digest_raw(h, hp, hn);
                libcall::mz_free(hp);
                let mut e2: libcall::mz_zip_error = libcall::MZ_ZIP_NO_ERROR;
                let hp2 = libcall::mz_zip_extract_archive_file_to_heap_v2(
                    scratch,
                    cstr!("f3.bin"),
                    std::ptr::null(),
                    &mut hn,
                    0,
                    &mut e2,
                );
                if hp2.is_null() {
                    eprintln!("archive_file_to_heap_v2 failed");
                    exit(3);
                }
                h = digest_raw(h, hp2, hn);
                libcall::mz_free(hp2);
                if libcall::mz_zip_validate_file_archive(scratch, 0, &mut e2) == 0 {
                    eprintln!("validate_file failed");
                    exit(3);
                }
            }

            /* 5. cfile writer + reader */
            {
                let fo = libcall::fopen(scratch2, cstr!("w+b"));
                let fi = libcall::fopen(input_c.as_ptr(), cstr!("rb"));
                if fo.is_null() || fi.is_null() {
                    eprintln!("cfile open failed");
                    exit(3);
                }
                let mut cw: libcall::mz_zip_archive = std::mem::zeroed();
                libcall::mz_zip_zero_struct(&mut cw);
                if libcall::mz_zip_writer_init_cfile(&mut cw, fo, 0) == 0 {
                    eprintln!("writer_init_cfile failed");
                    exit(3);
                }
                libcall::rewind(fi);
                if libcall::mz_zip_writer_add_cfile(
                    &mut cw,
                    cstr!("c.bin"),
                    fi,
                    n as u64,
                    std::ptr::addr_of!(FIXED_TIME),
                    std::ptr::null(),
                    0,
                    libcall::MZ_BEST_SPEED as u32,
                    std::ptr::null(),
                    0,
                    std::ptr::null(),
                    0,
                ) == 0
                {
                    let es = std::ffi::CStr::from_ptr(libcall::mz_zip_get_error_string(
                        libcall::mz_zip_get_last_error(&mut cw),
                    ));
                    eprintln!("add_cfile failed: {}", es.to_string_lossy());
                    exit(3);
                }
                if libcall::mz_zip_writer_finalize_archive(&mut cw) == 0 {
                    eprintln!("cfile finalize failed");
                    exit(3);
                }
                let written = libcall::mz_zip_get_archive_size(&mut cw);
                libcall::mz_zip_writer_end(&mut cw);
                libcall::fclose(fi);
                libcall::fseek(fo, 0, 0 /* SEEK_SET */);
                let mut cr: libcall::mz_zip_archive = std::mem::zeroed();
                libcall::mz_zip_zero_struct(&mut cr);
                if libcall::mz_zip_reader_init_cfile(&mut cr, fo, written, 0) == 0 {
                    eprintln!("reader_init_cfile failed");
                    exit(3);
                }
                let mut cn: usize = 0;
                let cp = libcall::mz_zip_reader_extract_to_heap(&mut cr, 0, &mut cn, 0);
                if cp.is_null() {
                    eprintln!("cfile extract failed");
                    exit(3);
                }
                h = digest_raw(h, cp, cn);
                libcall::mz_free(cp);
                libcall::mz_zip_reader_end(&mut cr);
                libcall::fclose(fo);
            }
        }
        libcall::remove(scratch);
        libcall::remove(scratch2);
        libcall::remove(scratch3);
    }
    println!("op=zip_file in={} out=0 iters={} digest={:016x}", n, iters, h);
}

/* -------------------------------------------------------------------- */

fn main() {
    /* provenance sanity check (stderr only, stdout unaffected) */
    unsafe {
        if libcall::mz_version().is_null() {
            eprintln!("bad miniz");
            exit(1);
        }
    }
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "usage: {} <op> <input> <iters>  |  gen_* only in the C driver",
            args.get(0).map(|s| s.as_str()).unwrap_or("driver")
        );
        exit(1);
    }
    let op = args[1].as_str();
    let iters: i64 = args[3].parse().unwrap_or(0);
    if iters <= 0 {
        eprintln!("bad iters");
        exit(1);
    }
    let buf = read_file_or_die(&args[2]);
    let n = buf.len();

    match op {
        "compress" => run_compress(&buf, n, iters),
        "uncompress" => run_uncompress(&buf, n, iters),
        "deflate_stream" => run_deflate_stream(&buf, n, iters),
        "inflate_stream" => run_inflate_stream(&buf, n, iters),
        "tdefl_matrix" => run_tdefl_matrix(&buf, n, iters),
        "tinfl_lowlevel" => run_tinfl_lowlevel(&buf, n, iters),
        "checksums" => run_checksums(&buf, n, iters),
        "zip_write" => run_zip_write(&buf, n, iters),
        "zip_read" => run_zip_read(&buf, n, iters),
        "zip_file" => run_zip_file(&buf, n, iters, &args[2]),
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
