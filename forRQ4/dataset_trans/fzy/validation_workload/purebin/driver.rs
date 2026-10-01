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

                                                          
#[inline]
fn usize_bytes(s: &[usize]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, std::mem::size_of_val(s)) }
}

const NEEDLES: [&[u8]; 6] = [b"a\0", b"src\0", b"main\0", b"test\0", b"make\0", b"cfg.h\0"];

                                                               
fn split_lines(buf: &mut [u8], n: usize) -> Vec<*const i8> {
    let mut lines = Vec::with_capacity(1024);
    let mut p = 0usize;
    while p < n {
        let nl = buf[p..n].iter().position(|&b| b == b'\n').map(|k| p + k).unwrap_or(n);
        buf[nl] = 0;                                          
        if buf[p] != 0 {
            lines.push(buf[p..].as_ptr() as *const i8);
        }
        p = nl + 1;
    }
    lines
}

fn run_match_batch(buf: &mut [u8], n: usize, iters: i64) {
    let lines = split_lines(buf, n);
    let cnt = lines.len();
    let _ = cnt;
    let mut h: u64 = 0;
    let mut matched: u64 = 0;
    for _ in 0..iters {
        matched = 0;
        for needle in NEEDLES {
            let nd = needle.as_ptr() as *const i8;
            for &line in &lines {
                unsafe {
                    if libcall::has_match(nd, line) != 0 {
                        let s = libcall::match_positions(nd, line, std::ptr::null_mut());
                        h = digest64(h, &s.to_ne_bytes());
                        matched += 1;
                    }
                }
            }
        }
    }
    println!("op=match_batch in={} out={} iters={} digest={:016x}", n, matched, iters, h);
}

fn run_match_positions_full(buf: &mut [u8], n: usize, iters: i64) {
    let lines = split_lines(buf, n);
    let cnt = if lines.len() > 2000 { 2000 } else { lines.len() };
    let mut pos = [0usize; 1024]; // MATCH_MAX_LEN
    let mut h: u64 = 0;
    for _ in 0..iters {
        for needle in NEEDLES {
            let nd = needle.as_ptr() as *const i8;
            let nl = needle.len() - 1;                  
            for &line in lines[..cnt].iter() {
                unsafe {
                    if libcall::has_match(nd, line) == 0 {
                        continue;
                    }
                    pos[..nl].fill(0);
                    let s = libcall::match_positions(nd, line, pos.as_mut_ptr());
                    h = digest64(h, &s.to_ne_bytes());
                    h = digest64(h, usize_bytes(&pos[..nl]));
                }
            }
        }
    }
    println!("op=match_positions_full in={} out={} iters={} digest={:016x}", n, cnt, iters, h);
}

unsafe fn cstr_bytes<'a>(p: *const i8) -> &'a [u8] {
    std::ffi::CStr::from_ptr(p).to_bytes()
}

fn run_choices_ops(buf: &mut [u8], n: usize, iters: i64) {
    let lines = split_lines(buf, n);
    let cnt = lines.len();
    let mut h: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let mut opt: libcall::Options = std::mem::zeroed();
            libcall::options_init(&mut opt);
            opt.workers = 1; // deterministic single-threaded search
            let mut c: libcall::Choices = std::mem::zeroed();
            libcall::choices_init(&mut c, &mut opt);
            for &line in &lines {
                libcall::choices_add(&mut c, line);
            }
            for needle in NEEDLES {
                let nd = needle.as_ptr() as *const i8;
                libcall::choices_search(&mut c, nd);
                let avail = libcall::choices_available(&mut c) as u64;
                h = digest64(h, &avail.to_ne_bytes());
                let top = if avail < 20 { avail as usize } else { 20 };
                for t in 0..top {
                    let s = libcall::choices_get(&mut c, t);
                    let sc = libcall::choices_getscore(&mut c, t);
                    if !s.is_null() {
                        h = digest64(h, cstr_bytes(s));
                    }
                    h = digest64(h, &sc.to_ne_bytes());
                }
                libcall::choices_next(&mut c);
                libcall::choices_next(&mut c);
                libcall::choices_prev(&mut c);
                let sel = c.selection as u64;
                h = digest64(h, &sel.to_ne_bytes());
            }
            libcall::choices_destroy(&mut c);

            // threaded search path (worker merge)
            let mut opt2: libcall::Options = std::mem::zeroed();
            libcall::options_init(&mut opt2);
            opt2.workers = 2;
            let mut c2: libcall::Choices = std::mem::zeroed();
            libcall::choices_init(&mut c2, &mut opt2);
            for &line in &lines {
                libcall::choices_add(&mut c2, line);
            }
            libcall::choices_search(&mut c2, b"src\0".as_ptr() as *const i8);
            let avail2 = libcall::choices_available(&mut c2) as u64;
            h = digest64(h, &avail2.to_ne_bytes());
            let top2 = if avail2 < 10 { avail2 as usize } else { 10 };
            for t in 0..top2 {
                let s = libcall::choices_get(&mut c2, t);
                let sc = libcall::choices_getscore(&mut c2, t);
                if !s.is_null() {
                    h = digest64(h, cstr_bytes(s));
                }
                h = digest64(h, &sc.to_ne_bytes());
            }
            libcall::choices_destroy(&mut c2);
        }
    }
    println!("op=choices_ops in={} out={} iters={} digest={:016x}", n, cnt, iters, h);
}

fn run_choices_fread(path: &str, iters: i64) {
    let cpath = std::ffi::CString::new(path).unwrap();
    let mut h: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let f = libcall::fopen(cpath.as_ptr(), b"rb\0".as_ptr() as *const i8);
            if f.is_null() {
                eprintln!("cannot open {}", path);
                exit(2);
            }
            let mut opt: libcall::Options = std::mem::zeroed();
            libcall::options_init(&mut opt);
            opt.workers = 1;
            let mut c: libcall::Choices = std::mem::zeroed();
            libcall::choices_init(&mut c, &mut opt);
            libcall::choices_fread(&mut c, f, b'\n' as i8);
            libcall::fclose(f);
            libcall::choices_search(&mut c, b"src\0".as_ptr() as *const i8);
            let acc = [c.size as usize, libcall::choices_available(&mut c)];
            h = digest64(h, usize_bytes(&acc));
            if libcall::choices_available(&mut c) > 0 {
                let s = libcall::choices_get(&mut c, 0);
                if !s.is_null() {
                    h = digest64(h, cstr_bytes(s));
                }
            }
            libcall::choices_destroy(&mut c);
        }
    }
    println!("op=choices_fread_op in=0 out=0 iters={} digest={:016x}", iters, h);
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

    if op == "choices_fread_op" {
        run_choices_fread(&args[2], iters);
        return;
    }

                                                              
    let mut buf = std::fs::read(&args[2]).unwrap_or_else(|_| {
        eprintln!("cannot read {}", args[2]);
        exit(2);
    });
    let n = buf.len();
    buf.push(0);

    match op {
        "match_batch" => run_match_batch(&mut buf, n, iters),
        "match_positions_full" => run_match_positions_full(&mut buf, n, iters),
        "choices_ops" => run_choices_ops(&mut buf, n, iters),
        "options_matrix" | "tty_ops" => {
            eprintln!("op {} is coverage_only (not timed in purebin)", op);
            exit(1);
        }
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
