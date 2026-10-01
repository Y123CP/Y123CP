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
fn as_bytes(s: &[u64]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, std::mem::size_of_val(s)) }
}

                                              
fn run_encode(pcm: &[u8], iters: i64, hd: i32, op: &str) {
    let n = pcm.len();
    let cap = n + n / 2 + 4096;
    let mut out = vec![0u8; cap];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let ctx = libcall::init(hd);
            if ctx.is_null() {
                eprintln!("aptx_init failed");
                exit(3);
            }
            let mut written: usize = 0;
            let consumed = libcall::encode(ctx, pcm.as_ptr(), n, out.as_mut_ptr(), cap, &mut written);
            let mut meta = [consumed as u64, written as u64];
            h = digest64(h, &out[..written]);
            let mut fin_written: usize = 0;
            if libcall::encode_finish(ctx, out.as_mut_ptr(), cap, &mut fin_written) != 0 && fin_written != 0 {
                h = digest64(h, &out[..fin_written]);
            }
            meta[1] += fin_written as u64;
            h = digest64(h, as_bytes(&meta));
            total = meta[1];
            libcall::finish(ctx);
        }
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, total, iters, h);
}

                                              
fn run_decode(ap: &[u8], iters: i64, hd: i32, op: &str) {
    let n = ap.len();
    let cap = n * 8 + 4096;
    let mut out = vec![0u8; cap];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let ctx = libcall::init(hd);
            let mut written: usize = 0;
            let consumed = libcall::decode(ctx, ap.as_ptr(), n, out.as_mut_ptr(), cap, &mut written);
            if consumed == 0 {
                eprintln!("decode consumed 0");
                exit(3);
            }
            h = digest64(h, &out[..written]);
            let meta = [consumed as u64, written as u64];
            h = digest64(h, as_bytes(&meta));
            total = written as u64;
            libcall::finish(ctx);
        }
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, total, iters, h);
}

                                                        
fn run_decode_sync(ap: &[u8], iters: i64) {
    let n = ap.len();
    if n < 1024 {
        eprintln!("stream too short");
        exit(2);
    }
    let mut broken = ap.to_vec();
    for k in 0..37 {
        broken[k] ^= 0x5A;
    }
    for k in (n / 2)..(n / 2 + 53) {
        broken[k] ^= 0xA5;
    }
    let cap = n * 8 + 4096;
    let mut out = vec![0u8; cap];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let ctx = libcall::init(0);
            let mut written: usize = 0;
            let mut synced: i32 = 0;
            let mut dropped: usize = 0;
            let consumed = libcall::decode_sync(
                ctx, broken.as_ptr(), n, out.as_mut_ptr(), cap,
                &mut written, &mut synced, &mut dropped,
            );
            h = digest64(h, &out[..written]);
            let meta = [consumed as u64, written as u64, synced as u64, dropped as u64];
            h = digest64(h, as_bytes(&meta));
            let fdropped = libcall::decode_sync_finish(ctx);
            let fd = [fdropped as u64];
            h = digest64(h, as_bytes(&fd));
            total = written as u64;
            libcall::finish(ctx);
        }
    }
    println!("op=decode_sync in={} out={} iters={} digest={:016x}", n, total, iters, h);
}

                                                          
fn run_reset_stream(pcm: &[u8], iters: i64) {
    let n = pcm.len();
    const CHUNK: usize = 65536 * 3; // multiple of the 24-byte frame
    let cap = CHUNK * 2 + 4096;
    let mut out = vec![0u8; cap];
    let mut h: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let ctx = libcall::init(0);
            let mut off = 0usize;
            while off + CHUNK <= n {
                let mut written: usize = 0;
                libcall::encode(ctx, pcm[off..].as_ptr(), CHUNK, out.as_mut_ptr(), cap, &mut written);
                h = digest64(h, &out[..written]);
                libcall::reset(ctx);
                off += CHUNK;
            }
            libcall::finish(ctx);
        }
    }
    println!("op=reset_stream in={} out=0 iters={} digest={:016x}", n, iters, h);
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
    let buf = std::fs::read(&args[2]).unwrap_or_else(|_| {
        eprintln!("cannot read {}", args[2]);
        exit(2);
    });

    match op {
        "encode" => run_encode(&buf, iters, 0, op),
        "encode_hd" => run_encode(&buf, iters, 1, op),
        "decode" => run_decode(&buf, iters, 0, op),
        "decode_hd" => run_decode(&buf, iters, 1, op),
        "decode_sync" => run_decode_sync(&buf, iters),
        "reset_stream" => run_reset_stream(&buf, iters),
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
