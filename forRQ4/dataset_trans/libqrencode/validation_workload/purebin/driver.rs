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
fn u64s(s: &[u64]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, std::mem::size_of_val(s)) }
}

                                                              
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

unsafe fn fold_qr(h: u64, qr: *mut libcall::Qr) -> u64 {
    if qr.is_null() {
        return digest64(h, b"null");
    }
    let meta = [(*qr).version as u64, (*qr).width as u64];
    let mut hh = digest64(h, u64s(&meta));
    let w = (*qr).width as usize;
    hh = digest64(hh, std::slice::from_raw_parts((*qr).data, w * w));
    hh
}

const LEVELS: [i32; 4] = [0, 1, 2, 3]; // QR_ECLEVEL_{L,M,Q,H}
const MODE_NUM: i32 = 0;
const MODE_AN: i32 = 1;
const MODE_8: i32 = 2;
const MODE_KANJI: i32 = 3;

fn run_encode_matrix(buf: &mut [u8], n: usize, iters: i64) {
    let lines = split_lines(buf, n);
    let mut h: u64 = 0;
    let mut symbols: u64 = 0;
    for _ in 0..iters {
        symbols = 0;
        for (j, &line) in lines.iter().enumerate() {
            for lv in 0..4 {
                let version = if j % 2 == 1 { 5 } else { 0 };
                unsafe {
                    let qr = libcall::encode_string(line, version, LEVELS[lv], MODE_8, 1);
                    if qr.is_null() {
                        eprintln!("encodeString failed on line {}", j);
                        exit(3);
                    }
                    h = fold_qr(h, qr);
                    libcall::qr_free(qr);
                }
                symbols += 1;
            }
        }
    }
    println!("op=encode_matrix in={} out={} iters={} digest={:016x}", n, symbols, iters, h);
}

fn run_encode_8bit(buf: &mut [u8], n: usize, iters: i64) {
    let lines = split_lines(buf, n);
    let cnt = lines.len();
    let mut h: u64 = 0;
    for _ in 0..iters {
        for (j, &line) in lines.iter().enumerate() {
            unsafe {
                let qr = libcall::encode_string_8bit(line, 0, LEVELS[j % 4]);
                if qr.is_null() {
                    eprintln!("encodeString8bit failed");
                    exit(3);
                }
                h = fold_qr(h, qr);
                libcall::qr_free(qr);
                let len = std::ffi::CStr::from_ptr(line).to_bytes().len();
                let qd = libcall::encode_data(len as i32, line as *const u8, 0, 1); // QR_ECLEVEL_M
                if qd.is_null() {
                    eprintln!("encodeData failed");
                    exit(3);
                }
                h = fold_qr(h, qd);
                libcall::qr_free(qd);
            }
        }
    }
    println!("op=encode_8bit in={} out={} iters={} digest={:016x}", n, cnt, iters, h);
}

fn run_encode_mqr(buf: &mut [u8], n: usize, iters: i64) {
    let lines = split_lines(buf, n);
    let mut h: u64 = 0;
    let mut done: u64 = 0;
    for _ in 0..iters {
        done = 0;
        for &line in &lines {
                                                                       
            let mut shorty = [0u8; 16];
            unsafe {
                let src = std::ffi::CStr::from_ptr(line).to_bytes();
                let l = src.len().min(10);
                shorty[..l].copy_from_slice(&src[..l]);
                let sp = shorty.as_ptr() as *const i8;
                let qr = libcall::encode_string_mqr(sp, 3, 0, MODE_8, 1);
                if !qr.is_null() {
                    h = fold_qr(h, qr);
                    libcall::qr_free(qr);
                    done += 1;
                }
                let q8 = libcall::encode_string_8bit_mqr(sp, 4, 0);
                if !q8.is_null() {
                    h = fold_qr(h, q8);
                    libcall::qr_free(q8);
                    done += 1;
                }
                let qd = libcall::encode_data_mqr(6, shorty.as_ptr(), 4, 0);
                if !qd.is_null() {
                    h = fold_qr(h, qd);
                    libcall::qr_free(qd);
                    done += 1;
                }
            }
        }
    }
    println!("op=encode_mqr in={} out={} iters={} digest={:016x}", n, done, iters, h);
}

unsafe fn fold_list(mut h: u64, list: *mut libcall::QrList) -> u64 {
    let mut e = list;
    while !e.is_null() {
        h = fold_qr(h, (*e).code);
        e = (*e).next;
    }
    h
}

fn run_structured(buf: &mut [u8], n: usize, iters: i64) {
    let lines = split_lines(buf, n);
    let mut big = [0u8; 1024];
    let mut off = 0usize;
    for &line in &lines {
        if off >= 800 {
            break;
        }
        let l = unsafe { std::ffi::CStr::from_ptr(line).to_bytes().len() };
        if off + l + 1 >= 800 {
            break;
        }
        unsafe {
            big[off..off + l].copy_from_slice(std::ffi::CStr::from_ptr(line).to_bytes());
        }
        off += l;
        big[off] = b' ';
        off += 1;
    }
    big[off] = 0;
    let bp = big.as_ptr() as *const i8;
    let mut h: u64 = 0;
    let mut syms: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let list = libcall::encode_string_8bit_structured(bp, 6, 0);
            if list.is_null() {
                eprintln!("structured failed");
                exit(3);
            }
            syms = libcall::list_size(list) as u64;
            h = fold_list(h, list);
            libcall::list_free(list);
            let l2 = libcall::encode_string_structured(bp, 7, 1, MODE_8, 1);
            if !l2.is_null() {
                let s2 = libcall::list_size(l2) as u64;
                h = digest64(h, &s2.to_ne_bytes());
                h = fold_list(h, l2);
                libcall::list_free(l2);
            }
            let l3 = libcall::encode_data_structured(off as i32, big.as_ptr(), 6, 0);
            if !l3.is_null() {
                h = fold_list(h, l3);
                libcall::list_free(l3);
            }
        }
    }
    println!("op=structured in={} out={} iters={} digest={:016x}", n, syms, iters, h);
}

fn run_input_builder(iters: i64) {
    let kanji: [u8; 4] = [0x93, 0x5f, 0x93, 0x5f];
    let mut h: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let inp = libcall::input_new2(0, 2); // QR_ECLEVEL_Q
            if inp.is_null() {
                eprintln!("QRinput_new2 failed");
                exit(3);
            }
            libcall::input_append(inp, MODE_NUM, 10, b"0123456789".as_ptr());
            libcall::input_append(inp, MODE_AN, 8, b"AC-42$/+".as_ptr());
            libcall::input_append(inp, MODE_8, 12, b"binary\x01\x02:-)#".as_ptr());
            libcall::input_append(inp, MODE_KANJI, 4, kanji.as_ptr());
            let est = [
                libcall::estimate_bits_num(10) as u64,
                libcall::estimate_bits_an(8) as u64,
                libcall::estimate_bits_8(12) as u64,
                libcall::estimate_bits_kanji(4) as u64,
                libcall::input_check(MODE_AN, 8, b"AC-42$/+".as_ptr()) as u64,
            ];
            h = digest64(h, u64s(&est));
            libcall::input_set_ver_ecl(inp, 10, 3); // QR_ECLEVEL_H
            let got = [libcall::input_get_version(inp) as u64, libcall::input_get_ecl(inp) as u64];
            h = digest64(h, u64s(&got));
            let dup = libcall::input_dup(inp);
            let qr = libcall::encode_input(dup);
            if !qr.is_null() {
                h = fold_qr(h, qr);
                libcall::qr_free(qr);
            }
            libcall::input_free(dup);

            // structured-append via builder
            let big = libcall::input_new2(0, 0); // QR_ECLEVEL_L
            let mut blob = [0u8; 800];
            for (b, x) in blob.iter_mut().enumerate() {
                *x = (b.wrapping_mul(7).wrapping_add(1)) as u8;
            }
            libcall::input_append(big, MODE_8, blob.len() as i32, blob.as_ptr());
            libcall::input_set_version(big, 5);
            libcall::input_set_ecl(big, 0);
            let st = libcall::input_split_to_struct(big);
            if !st.is_null() {
                libcall::struct_set_parity(st, 0x42);
                libcall::struct_insert_headers(st);
                let list = libcall::encode_input_structured(st);
                if !list.is_null() {
                    let sz = libcall::list_size(list) as u64;
                    h = digest64(h, &sz.to_ne_bytes());
                    h = fold_list(h, list);
                    libcall::list_free(list);
                }
                libcall::struct_free(st);
            }
            libcall::input_free(big);

            // FNC1 + ECI + MQR input + misc
            let f1 = libcall::input_new();
            libcall::input_set_fnc1_first(f1);
            libcall::input_append(f1, MODE_NUM, 4, b"1234".as_ptr());
            let qf = libcall::encode_input(f1);
            if !qf.is_null() {
                h = fold_qr(h, qf);
                libcall::qr_free(qf);
            }
            libcall::input_free(f1);
            let f2 = libcall::input_new2(0, 1); // QR_ECLEVEL_M
            libcall::input_set_fnc1_second(f2, b'A' as i8);
            libcall::input_append_eci(f2, 26);
            libcall::input_append(f2, MODE_8, 5, b"hello".as_ptr());
            let qe = libcall::encode_input(f2);
            if !qe.is_null() {
                h = fold_qr(h, qe);
                libcall::qr_free(qe);
            }
            libcall::input_free(f2);
            let mq = libcall::input_new_mqr(3, 0);
            if !mq.is_null() {
                libcall::input_append(mq, MODE_NUM, 6, b"424242".as_ptr());
                let sp = libcall::is_splittable(MODE_8) as u64;
                h = digest64(h, &sp.to_ne_bytes());
                let bs = libcall::input_get_byte_stream(inp);
                if !bs.is_null() {
                    libcall::free(bs as *mut core::ffi::c_void);
                }
                libcall::input_free(mq);
            }
            libcall::input_free(inp);
            let mut vmaj = 0i32;
            let mut vmin = 0i32;
            let mut vmic = 0i32;
            libcall::api_version(&mut vmaj, &mut vmin, &mut vmic);
            let vs = std::ffi::CStr::from_ptr(libcall::api_version_string()).to_bytes().len();
            let api = [(vmaj as u64) << 16 | (vmin as u64) << 8 | vmic as u64, vs as u64];
            h = digest64(h, u64s(&api));
        }
    }
    unsafe { libcall::clear_cache() };
    println!("op=input_builder in=0 out=0 iters={} digest={:016x}", iters, h);
}

fn run_split_op(buf: &mut [u8], n: usize, iters: i64) {
    let lines = split_lines(buf, n);
    let cnt = lines.len();
    let sjis_line = b"QR\x93\x5f\x93\x5f 123ABC\0";
    let mut h: u64 = 0;
    for _ in 0..iters {
        unsafe {
            for &line in &lines {
                for mode in 0..2 {
                    let inp = libcall::input_new2(0, 1); // QR_ECLEVEL_M
                    let r = if mode == 1 {
                        libcall::split_string(line, inp, MODE_KANJI, 0)
                    } else {
                        libcall::split_string(line, inp, MODE_8, 1)
                    };
                    let rr = r as u64;
                    h = digest64(h, &rr.to_ne_bytes());
                    if r == 0 {
                        let qr = libcall::encode_input(inp);
                        if !qr.is_null() {
                            h = fold_qr(h, qr);
                            libcall::qr_free(qr);
                        }
                    }
                    libcall::input_free(inp);
                }
            }
            let ki = libcall::input_new2(0, 1);
            if libcall::split_string(sjis_line.as_ptr() as *const i8, ki, MODE_KANJI, 0) == 0 {
                let qk = libcall::encode_input(ki);
                if !qk.is_null() {
                    h = fold_qr(h, qk);
                    libcall::qr_free(qk);
                }
            }
            libcall::input_free(ki);

            // struct split WITHOUT explicit parity -> auto calcParity path
            let auto_in = libcall::input_new2(0, 0);
            let mut blob2 = [0u8; 600];
            for (b, x) in blob2.iter_mut().enumerate() {
                *x = (b.wrapping_mul(13).wrapping_add(5)) as u8;
            }
            libcall::input_append(auto_in, MODE_8, blob2.len() as i32, blob2.as_ptr());
            libcall::input_set_version(auto_in, 6);
            let st2 = libcall::input_split_to_struct(auto_in);
            if !st2.is_null() {
                libcall::struct_insert_headers(st2);
                let lst = libcall::encode_input_structured(st2);
                if !lst.is_null() {
                    h = fold_list(h, lst);
                    libcall::list_free(lst);
                }
                libcall::struct_free(st2);
            }
            libcall::input_free(auto_in);
        }
    }
    println!("op=split_op in={} out={} iters={} digest={:016x}", n, cnt, iters, h);
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

    if op == "input_builder" {
        run_input_builder(iters);
        return;
    }

    let mut buf = std::fs::read(&args[2]).unwrap_or_else(|_| {
        eprintln!("cannot read {}", args[2]);
        exit(2);
    });
    let n = buf.len();
    buf.push(0);

    match op {
        "encode_matrix" => run_encode_matrix(&mut buf, n, iters),
        "encode_8bit" => run_encode_8bit(&mut buf, n, iters),
        "encode_mqr" => run_encode_mqr(&mut buf, n, iters),
        "structured" => run_structured(&mut buf, n, iters),
        "split_op" => run_split_op(&mut buf, n, iters),
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
