// ============================================================================
                                                             
                                      
                                                      
                                                  
                                                               
//
                                                                      
                                                                      
// ============================================================================

use std::process::exit;

                                                                        
#[inline]
fn digest64(seed: u64, p: &[u8]) -> u64 {
    let mut l0 = seed ^ 0xcbf2_9ce4_8422_2325u64;
    let mut l1 = 0x8422_2325_cbf2_9ce4u64;
    let mut l2 = 0x9ce4_8422_2325_cbf2u64;
    let mut l3 = 0x2325_cbf2_9ce4_8422u64;
    // `chunks_exact` carries the length in the type, so the four 8-byte reads
    // need no bounds check and the loop vectorises — driver.c gets this from
    // plain `memcpy(&w, p+i, 8)`. The indexed form this replaces compiled to
    // four `ja <panic>` branches per 32-byte block and stayed scalar, which
    // put the driver's own digest on the critical path of every decompress
    // measurement. Arithmetic is unchanged, so the digest is bit-identical.
    let mut blocks = p.chunks_exact(32);
    for c in blocks.by_ref() {
        let (b0, r0) = c.split_at(8);
        let (b1, r1) = r0.split_at(8);
        let (b2, b3) = r1.split_at(8);
        let w0 = u64::from_ne_bytes(b0.try_into().unwrap());
        let w1 = u64::from_ne_bytes(b1.try_into().unwrap());
        let w2 = u64::from_ne_bytes(b2.try_into().unwrap());
        let w3 = u64::from_ne_bytes(b3.try_into().unwrap());
        l0 = (l0 ^ w0).rotate_left(17);
        l1 = (l1 ^ w1).rotate_left(19);
        l2 = (l2 ^ w2).rotate_left(23);
        l3 = (l3 ^ w3).rotate_left(29);
    }
    for &b in blocks.remainder() {
        l0 = (l0 ^ b as u64).rotate_left(11);
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

const DICT_SIZE: usize = 65536;
const DICT_MSG: usize = 4096;
const STREAM_CHUNK: usize = 65536;

// ---- run_compress(driver.c run_compress)—— level 0=fast, >0=HC ----
fn run_compress(op: &str, src: &[u8], iters: i64, level: i32) {
    let n = src.len();
    let bound = unsafe { libcall::compress_bound(n as i32) };
    let mut dst = vec![0u8; bound as usize];
    let mut h: u64 = 0;
    let mut outlen: i32 = 0;
    for _ in 0..iters {
        outlen = unsafe {
            if level != 0 {
                libcall::compress_hc(src.as_ptr(), dst.as_mut_ptr(), n as i32, bound, level)
            } else {
                libcall::compress_default(src.as_ptr(), dst.as_mut_ptr(), n as i32, bound)
            }
        };
        if outlen <= 0 {
            eprintln!("compress failed");
            exit(3);
        }
        h = digest64(h, &dst[..outlen as usize]);
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, outlen, iters, h);
}

                                                                                 
fn run_compress_dict(op: &str, src: &[u8], iters: i64) {
    let n = src.len();
    if n < DICT_SIZE + DICT_MSG {
        eprintln!("input too small for dict op");
        exit(2);
    }
    let dn = n - DICT_SIZE;
    let bound = unsafe { libcall::compress_bound(DICT_MSG as i32) };
    let mut dst = vec![0u8; bound as usize];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    unsafe {
        let dict_s = libcall::create_stream();
        let work_s = libcall::create_stream();
        if dict_s.is_null() || work_s.is_null() {
            eprintln!("oom");
            exit(2);
        }
        libcall::load_dict(dict_s, src.as_ptr(), DICT_SIZE as i32);
                                                                   
        let mut scratch = vec![0u8; DICT_SIZE];
        libcall::save_dict(dict_s, scratch.as_mut_ptr(), DICT_SIZE as i32);
        for _ in 0..iters {
            total = 0;
            let mut off = 0usize;
            while off < dn {
                let msg = if dn - off < DICT_MSG { (dn - off) as i32 } else { DICT_MSG as i32 };
                libcall::reset_stream_fast(work_s);
                libcall::attach_dictionary(work_s, dict_s);
                let outlen = libcall::compress_fast_continue(
                    work_s, src[DICT_SIZE + off..].as_ptr(), dst.as_mut_ptr(), msg, bound, 1);
                if outlen <= 0 {
                    eprintln!("dict compress failed");
                    exit(3);
                }
                h = digest64(h, &dst[..outlen as usize]);
                total += outlen as u64;
                off += DICT_MSG;
            }
        }
        libcall::free_stream(dict_s);
        libcall::free_stream(work_s);
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, total, iters, h);
}

// ---- run_compress_dict_hc(driver.c)—— HC preset-dict ----
fn run_compress_dict_hc(op: &str, src: &[u8], iters: i64) {
    let n = src.len();
    if n < DICT_SIZE + DICT_MSG {
        eprintln!("input too small for dict op");
        exit(2);
    }
    let dn = n - DICT_SIZE;
    let bound = unsafe { libcall::compress_bound(DICT_MSG as i32) };
    let mut dst = vec![0u8; bound as usize];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    unsafe {
        let dict_s = libcall::create_stream_hc();
        let work_s = libcall::create_stream_hc();
        if dict_s.is_null() || work_s.is_null() {
            eprintln!("oom");
            exit(2);
        }
        libcall::load_dict_hc(dict_s, src.as_ptr(), DICT_SIZE as i32);
        let mut scratch = vec![0u8; DICT_SIZE];
        libcall::save_dict_hc(dict_s, scratch.as_mut_ptr(), DICT_SIZE as i32);
        for _ in 0..iters {
            total = 0;
            let mut off = 0usize;
            while off < dn {
                let msg = if dn - off < DICT_MSG { (dn - off) as i32 } else { DICT_MSG as i32 };
                libcall::reset_stream_hc_fast(work_s, 9);
                libcall::attach_hc_dictionary(work_s, dict_s);
                let outlen = libcall::compress_hc_continue(
                    work_s, src[DICT_SIZE + off..].as_ptr(), dst.as_mut_ptr(), msg, bound);
                if outlen <= 0 {
                    eprintln!("hc dict compress failed");
                    exit(3);
                }
                h = digest64(h, &dst[..outlen as usize]);
                total += outlen as u64;
                off += DICT_MSG;
            }
        }
        libcall::free_stream_hc(dict_s);
        libcall::free_stream_hc(work_s);
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, total, iters, h);
}

                                                          
fn run_compress_destsize(op: &str, src: &[u8], iters: i64, hc: bool) {
    const BUDGET: i32 = 4096;
    let n = src.len();
    let mut dst = vec![0u8; BUDGET as usize];
    let ssz = unsafe { if hc { libcall::sizeof_state_hc() } else { libcall::sizeof_state() } };
    let mut state = vec![0u8; ssz as usize];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    for _ in 0..iters {
        total = 0;
        let mut pos = 0usize;
        let mut page: i64 = 0;
        while pos < n {
            let mut src_size = if n - pos > i32::MAX as usize { i32::MAX } else { (n - pos) as i32 };
            let outlen = unsafe {
                if hc {
                    libcall::compress_hc_destsize(
                        state.as_mut_ptr(), src[pos..].as_ptr(), dst.as_mut_ptr(),
                        &mut src_size, BUDGET, 9)
                } else if page & 1 != 0 {
                    libcall::compress_destsize_extstate(
                        state.as_mut_ptr(), src[pos..].as_ptr(), dst.as_mut_ptr(),
                        &mut src_size, BUDGET, 1)
                } else {
                    libcall::compress_destsize(
                        src[pos..].as_ptr(), dst.as_mut_ptr(), &mut src_size, BUDGET)
                }
            };
            if outlen <= 0 || src_size <= 0 {
                eprintln!("destSize failed");
                exit(3);
            }
            h = digest64(h, &dst[..outlen as usize]);
            total += outlen as u64;
            pos += src_size as usize;
            page += 1;
        }
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, total, iters, h);
}

// ---- run_compress_extstate(driver.c)—— caller-managed state, fastReset ----
fn run_compress_extstate(op: &str, src: &[u8], iters: i64) {
    let n = src.len();
    let ssz = unsafe { libcall::sizeof_state() };
    let mut state = vec![0u8; ssz as usize];
    let bound = unsafe { libcall::compress_bound(n as i32) };
    let mut dst = vec![0u8; bound as usize];
    let mut h: u64 = 0;
    let mut outlen: i32 = 0;
    unsafe {
        if libcall::init_stream(state.as_mut_ptr(), ssz as usize).is_null() {
            eprintln!("initStream failed (alignment)");
            exit(2);
        }
        for _ in 0..iters {
            outlen = libcall::compress_fast_extstate_fastreset(
                state.as_mut_ptr(), src.as_ptr(), dst.as_mut_ptr(), n as i32, bound, 1);
            if outlen <= 0 {
                eprintln!("extState compress failed");
                exit(3);
            }
            h = digest64(h, &dst[..outlen as usize]);
        }
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, outlen, iters, h);
}

// ---- run_decompress(driver.c)—— u64 LE orig size + LZ4 block ----
fn run_decompress(file: &[u8], iters: i64) {
    if file.len() < 8 {
        eprintln!("bad decompress input");
        exit(2);
    }
    let orig = u64::from_ne_bytes(file[0..8].try_into().unwrap());
    let comp = &file[8..];
    let comp_n = comp.len();
    let mut dst = vec![0u8; orig as usize];
    let mut h: u64 = 0;
    for _ in 0..iters {
        let r = unsafe {
            libcall::decompress_safe(comp.as_ptr(), dst.as_mut_ptr(), comp_n as i32, orig as i32)
        };
        if r != orig as i32 {
            eprintln!("decompress failed: {}", r);
            exit(3);
        }
        h = digest64(h, &dst[..orig as usize]);
    }
    println!("op=decompress in={} out={} iters={} digest={:016x}", file.len(), orig, iters, h);
}

                                                         
fn run_decompress_partial(file: &[u8], iters: i64) {
    if file.len() < 8 {
        eprintln!("bad decompress input");
        exit(2);
    }
    let orig = u64::from_ne_bytes(file[0..8].try_into().unwrap());
    let comp = &file[8..];
    let comp_n = comp.len();
    let target = (orig / 2) as i32;
    let mut dst = vec![0u8; orig as usize];
    let mut h: u64 = 0;
    for _ in 0..iters {
        let r = unsafe {
            libcall::decompress_safe_partial(
                comp.as_ptr(), dst.as_mut_ptr(), comp_n as i32, target, orig as i32)
        };
        if r != target {
            eprintln!("partial failed: {}", r);
            exit(3);
        }
        h = digest64(h, &dst[..r as usize]);
    }
    println!("op=decompress_partial in={} out={} iters={} digest={:016x}", file.len(), target, iters, h);
}

// ---- run_decompress_dict(driver.c)—— u32 dict + dict + u64 total + msgs ----
fn run_decompress_dict(file: &[u8], iters: i64) {
    if file.len() < 12 {
        eprintln!("bad dict input");
        exit(2);
    }
    let dict_n = u32::from_ne_bytes(file[0..4].try_into().unwrap());
    let dict = &file[4..];
    let pos0_total = 4 + dict_n as usize;
    let total = u64::from_ne_bytes(file[pos0_total..pos0_total + 8].try_into().unwrap());
    let pos0 = pos0_total + 8;
    let mut out = vec![0u8; DICT_MSG];
    let mut h: u64 = 0;
    for _ in 0..iters {
        let mut pos = pos0;
        let mut off: u64 = 0;
        while pos < file.len() {
            let comp = u32::from_ne_bytes(file[pos..pos + 4].try_into().unwrap());
            let orig = u32::from_ne_bytes(file[pos + 4..pos + 8].try_into().unwrap());
            pos += 8;
            let r = unsafe {
                libcall::decompress_safe_using_dict(
                    file[pos..].as_ptr(), out.as_mut_ptr(), comp as i32, orig as i32,
                    dict.as_ptr(), dict_n as i32)
            };
            if r != orig as i32 {
                eprintln!("dict decompress failed: {}", r);
                exit(3);
            }
            h = digest64(h, &out[..orig as usize]);
            pos += comp as usize;
            off += orig as u64;
        }
        if off != total {
            eprintln!("dict size mismatch");
            exit(3);
        }
    }
    println!("op=decompress_dict in={} out={} iters={} digest={:016x}", file.len(), total, iters, h);
}

// ---- run_decompress_partial_dict(driver.c)—— partial + external dict ----
fn run_decompress_partial_dict(file: &[u8], iters: i64) {
    if file.len() < 12 {
        eprintln!("bad dict input");
        exit(2);
    }
    let dict_n = u32::from_ne_bytes(file[0..4].try_into().unwrap());
    let dict = &file[4..];
    let pos0 = 4 + dict_n as usize + 8;
    let mut out = vec![0u8; DICT_MSG];
    let mut h: u64 = 0;
    let mut produced: u64 = 0;
    for _ in 0..iters {
        let mut pos = pos0;
        produced = 0;
        while pos < file.len() {
            let comp = u32::from_ne_bytes(file[pos..pos + 4].try_into().unwrap());
            let orig = u32::from_ne_bytes(file[pos + 4..pos + 8].try_into().unwrap());
            pos += 8;
            let target = (orig / 2) as i32;
            let r = unsafe {
                libcall::decompress_safe_partial_using_dict(
                    file[pos..].as_ptr(), out.as_mut_ptr(), comp as i32, target, orig as i32,
                    dict.as_ptr(), dict_n as i32)
            };
            if r != target {
                eprintln!("partial dict failed: {}", r);
                exit(3);
            }
            h = digest64(h, &out[..r as usize]);
            produced += r as u64;
            pos += comp as usize;
        }
    }
    println!("op=decompress_partial_dict in={} out={} iters={} digest={:016x}", file.len(), produced, iters, h);
}

// ---- run_compress_stream(driver.c)—— 64KB chunk, *_continue ----
fn run_compress_stream(op: &str, src: &[u8], iters: i64, hc: bool) {
    let n = src.len();
    let bound = unsafe { libcall::compress_bound(STREAM_CHUNK as i32) };
    let mut dst = vec![0u8; bound as usize];
    let mut h: u64 = 0;
    let mut total_out: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let s = if hc { std::ptr::null_mut() } else { libcall::create_stream() };
            let sh = if hc { libcall::create_stream_hc() } else { std::ptr::null_mut() };
            if s.is_null() && sh.is_null() {
                eprintln!("stream alloc failed");
                exit(2);
            }
            total_out = 0;
            let mut off = 0usize;
            while off < n {
                let csize = if n - off < STREAM_CHUNK { (n - off) as i32 } else { STREAM_CHUNK as i32 };
                let outlen = if hc {
                    libcall::compress_hc_continue(sh, src[off..].as_ptr(), dst.as_mut_ptr(), csize, bound)
                } else {
                    libcall::compress_fast_continue(s, src[off..].as_ptr(), dst.as_mut_ptr(), csize, bound, 1)
                };
                if outlen <= 0 {
                    eprintln!("stream compress failed");
                    exit(3);
                }
                h = digest64(h, &dst[..outlen as usize]);
                total_out += outlen as u64;
                off += STREAM_CHUNK;
            }
            if hc { libcall::free_stream_hc(sh); } else { libcall::free_stream(s); }
        }
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, total_out, iters, h);
}

// ---- run_decompress_stream(driver.c)—— u64 total + per chunk ----
fn run_decompress_stream(file: &[u8], iters: i64) {
    if file.len() < 8 {
        eprintln!("bad stream input");
        exit(2);
    }
    let total = u64::from_ne_bytes(file[0..8].try_into().unwrap());
    let mut out = vec![0u8; total as usize];
    let mut h: u64 = 0;
    unsafe {
        let sd = libcall::create_stream_decode();
        if sd.is_null() {
            eprintln!("stream alloc failed");
            exit(2);
        }
        for _ in 0..iters {
            if libcall::set_stream_decode(sd, std::ptr::null(), 0) == 0 {
                eprintln!("setStreamDecode failed");
                exit(2);
            }
            let mut pos = 8usize;
            let mut off = 0usize;
            while pos < file.len() {
                let comp = u32::from_ne_bytes(file[pos..pos + 4].try_into().unwrap());
                let orig = u32::from_ne_bytes(file[pos + 4..pos + 8].try_into().unwrap());
                pos += 8;
                let r = libcall::decompress_safe_continue(
                    sd, file[pos..].as_ptr(), out[off..].as_mut_ptr(), comp as i32, orig as i32);
                if r != orig as i32 {
                    eprintln!("stream decompress failed: {}", r);
                    exit(3);
                }
                h = digest64(h, &out[off..off + orig as usize]);
                pos += comp as usize;
                off += orig as usize;
            }
            if off != total as usize {
                eprintln!("stream size mismatch");
                exit(3);
            }
        }
        libcall::free_stream_decode(sd);
    }
    println!("op=decompress_stream in={} out={} iters={} digest={:016x}", file.len(), total, iters, h);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: {} <op> <input> <iters>", args.get(0).map(|s| s.as_str()).unwrap_or("driver"));
        exit(1);
    }
    let op = args[1].as_str();
    let iters: i64 = args[3].parse().unwrap_or(0);
    if iters <= 0 {
        eprintln!("bad iters");
        exit(1);
    }
                                                  
    if unsafe { libcall::version_number() } < 10000 {
        eprintln!("unexpected lz4 version");
        exit(1);
    }
    let buf = std::fs::read(&args[2]).unwrap_or_else(|_| {
        eprintln!("cannot read {}", args[2]);
        exit(2);
    });

    match op {
        "compress_fast" => run_compress(op, &buf, iters, 0),
        "compress_hc" => run_compress(op, &buf, iters, 9),
        "compress_hc_min" => run_compress(op, &buf, iters, 2),
        "compress_hc_max" => run_compress(op, &buf, iters, 12),
        "decompress" => run_decompress(&buf, iters),
        "compress_stream" => run_compress_stream(op, &buf, iters, false),
        "compress_stream_hc" => run_compress_stream(op, &buf, iters, true),
        "decompress_stream" => run_decompress_stream(&buf, iters),
        "compress_dict" => run_compress_dict(op, &buf, iters),
        "decompress_dict" => run_decompress_dict(&buf, iters),
        "compress_dict_hc" => run_compress_dict_hc(op, &buf, iters),
        "compress_destsize" => run_compress_destsize(op, &buf, iters, false),
        "compress_destsize_hc" => run_compress_destsize(op, &buf, iters, true),
        "compress_extstate" => run_compress_extstate(op, &buf, iters),
        "decompress_partial" => run_decompress_partial(&buf, iters),
        "decompress_partial_dict" => run_decompress_partial_dict(&buf, iters),
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
