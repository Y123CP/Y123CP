// ============================================================================
                                                                     
                                                    
                                                
                                                           
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

fn run_container(op: &str, fmt: u32, src: &[u8], iters: i64) {
    let n = src.len();
    let mut h: u64 = 0;
    let mut outsize: usize = 0;
    for _ in 0..iters {
        unsafe {
            let mut opt: libcall::Options = std::mem::zeroed();
            libcall::init_options(&mut opt);
            let mut out: *mut u8 = std::ptr::null_mut();
            outsize = 0;
            libcall::compress(&opt, fmt, src.as_ptr(), n, &mut out, &mut outsize);
            if out.is_null() || outsize == 0 {
                eprintln!("compress failed");
                exit(3);
            }
            h = digest64(h, std::slice::from_raw_parts(out, outsize));
            libcall::free(out as *mut core::ffi::c_void);
        }
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, outsize, iters, h);
}

fn run_options_matrix(src: &[u8], iters: i64) {
    let n = src.len();
    let its = [5i32, 15];
    let bsp = [0i32, 1];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    for _ in 0..iters {
        total = 0;
        for &a in &its {
            for &b in &bsp {
                unsafe {
                    let mut opt: libcall::Options = std::mem::zeroed();
                    libcall::init_options(&mut opt);
                    opt.numiterations = a;
                    opt.blocksplitting = b;
                    let mut out: *mut u8 = std::ptr::null_mut();
                    let mut outsize: usize = 0;
                    libcall::compress(&opt, libcall::FMT_GZIP, src.as_ptr(), n, &mut out, &mut outsize);
                    if out.is_null() || outsize == 0 {
                        eprintln!("compress failed");
                        exit(3);
                    }
                    h = digest64(h, std::slice::from_raw_parts(out, outsize));
                    total += outsize as u64;
                    libcall::free(out as *mut core::ffi::c_void);
                }
            }
        }
    }
    println!("op=options_matrix in={} out={} iters={} digest={:016x}", n, total, iters, h);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: {} <compress_gzip|compress_zlib|compress_deflate|options_matrix> <input> <iters>",
                  args.get(0).map(|s| s.as_str()).unwrap_or("driver"));
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
        "compress_gzip" => run_container(op, libcall::FMT_GZIP, &buf, iters),
        "compress_zlib" => run_container(op, libcall::FMT_ZLIB, &buf, iters),
        "compress_deflate" => run_container(op, libcall::FMT_DEFLATE, &buf, iters),
        "options_matrix" => run_options_matrix(&buf, iters),
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
