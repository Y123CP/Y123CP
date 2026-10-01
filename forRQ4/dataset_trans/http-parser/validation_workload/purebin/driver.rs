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

                                         
#[repr(C)]
struct Acc {
    h: u64,
    msgs: u64,
    hdrs: u64,
    body_bytes: u64,
    chunks: u64,
    pause_armed: i32,
    parser: *mut libcall::Parser,
}

unsafe fn acc_of(p: *mut libcall::Parser) -> &'static mut Acc {
    &mut *((*p).data as *mut Acc)
}

unsafe extern "C" fn cb_message_begin(p: *mut libcall::Parser) -> i32 {
    acc_of(p).msgs += 1;
    0
}
unsafe extern "C" fn cb_url(p: *mut libcall::Parser, at: *const i8, n: usize) -> i32 {
    let a = acc_of(p);
    a.h = digest64(a.h, std::slice::from_raw_parts(at as *const u8, n));
    0
}
unsafe extern "C" fn cb_status(p: *mut libcall::Parser, at: *const i8, n: usize) -> i32 {
    let a = acc_of(p);
    a.h = digest64(a.h, std::slice::from_raw_parts(at as *const u8, n));
    0
}
unsafe extern "C" fn cb_header_field(p: *mut libcall::Parser, at: *const i8, n: usize) -> i32 {
    let a = acc_of(p);
    a.hdrs += 1;
    a.h = digest64(a.h, std::slice::from_raw_parts(at as *const u8, n));
    0
}
unsafe extern "C" fn cb_header_value(p: *mut libcall::Parser, at: *const i8, n: usize) -> i32 {
    let a = acc_of(p);
    a.h = digest64(a.h, std::slice::from_raw_parts(at as *const u8, n));
    0
}
unsafe extern "C" fn cb_headers_complete(p: *mut libcall::Parser) -> i32 {
    let a = acc_of(p);
    let meta = [
        if (*p).type_0() == 0 { (*p).method() as u64 } else { (*p).status_code() as u64 },
        ((*p).http_major as u64) << 8 | (*p).http_minor as u64,
        libcall::should_keep_alive(p) as u64,
        (*p).content_length,
    ];
    a.h = digest64(a.h, u64s(&meta));
    0
}
unsafe extern "C" fn cb_body(p: *mut libcall::Parser, at: *const i8, n: usize) -> i32 {
    let a = acc_of(p);
    a.body_bytes += n as u64;
    a.h = digest64(a.h, std::slice::from_raw_parts(at as *const u8, n));
    let _ = libcall::body_is_final(p);
    0
}
unsafe extern "C" fn cb_message_complete(p: *mut libcall::Parser) -> i32 {
    let a = acc_of(p);
    let fin = [libcall::should_keep_alive(p) as u64, 0xC0DAu64];
    a.h = digest64(a.h, u64s(&fin));
    if a.pause_armed != 0 {
        libcall::pause(a.parser, 1);
    }
    0
}
unsafe extern "C" fn cb_chunk_header(p: *mut libcall::Parser) -> i32 {
    let a = acc_of(p);
    a.chunks += 1;
    let cl = (*p).content_length;
    a.h = digest64(a.h, &cl.to_ne_bytes());
    0
}
unsafe extern "C" fn cb_chunk_complete(p: *mut libcall::Parser) -> i32 {
    let a = acc_of(p);
    a.h = digest64(a.h, b"ck");
    0
}

unsafe fn settings_fill(s: *mut libcall::Settings) {
    libcall::settings_init(s);
    (*s).on_message_begin = Some(cb_message_begin);
    (*s).on_url = Some(cb_url);
    (*s).on_status = Some(cb_status);
    (*s).on_header_field = Some(cb_header_field);
    (*s).on_header_value = Some(cb_header_value);
    (*s).on_headers_complete = Some(cb_headers_complete);
    (*s).on_body = Some(cb_body);
    (*s).on_message_complete = Some(cb_message_complete);
    (*s).on_chunk_header = Some(cb_chunk_header);
    (*s).on_chunk_complete = Some(cb_chunk_complete);
}

fn run_parse(op: &str, ptype: u32, buf: &[u8], iters: i64) {
    let n = buf.len();
    let mut h: u64 = 0;
    let mut msgs: u64 = 0;
    unsafe {
        let mut st: libcall::Settings = std::mem::zeroed();
        settings_fill(&mut st);
        for _ in 0..iters {
            let mut a: Acc = std::mem::zeroed();
            let mut p: libcall::Parser = std::mem::zeroed();
            libcall::init(&mut p, ptype);
            p.data = &mut a as *mut Acc as *mut core::ffi::c_void;
            a.parser = &mut p;
            let mut pos = 0usize;
            while pos < n {
                let np = libcall::execute(&mut p, &st, buf[pos..].as_ptr() as *const i8, n - pos);
                pos += np;
                if p.http_errno() == libcall::HPE_OK {
                    if pos >= n {
                        break;
                    }
                    if p.upgrade() != 0 {
                        libcall::init(&mut p, ptype);
                        p.data = &mut a as *mut Acc as *mut core::ffi::c_void;
                        continue;
                    }
                    eprintln!("{} stalled at {}/{}", op, pos, n);
                    exit(3);
                }
                if p.http_errno() == libcall::HPE_CLOSED_CONNECTION {
                    libcall::init(&mut p, ptype);
                    p.data = &mut a as *mut Acc as *mut core::ffi::c_void;
                    continue;
                }
                eprintln!("{} failed at {}/{}", op, pos, n);
                exit(3);
            }
            libcall::execute(&mut p, &st, std::ptr::null(), 0); // EOF
            h = digest64(h, &a.h.to_ne_bytes());
            let cnts = [a.msgs, a.hdrs, a.body_bytes];
            h = digest64(h, u64s(&cnts));
            msgs = a.msgs;
        }
    }
    println!("op={} in={} out={} iters={} digest={:016x}", op, n, msgs, iters, h);
}

fn run_parse_incremental(buf: &[u8], iters: i64) {
    const CHUNK: usize = 7;
    let n = buf.len();
    let mut h: u64 = 0;
    let mut msgs: u64 = 0;
    unsafe {
        let mut st: libcall::Settings = std::mem::zeroed();
        settings_fill(&mut st);
        for _ in 0..iters {
            let mut a: Acc = std::mem::zeroed();
            a.pause_armed = 1;
            let mut p: libcall::Parser = std::mem::zeroed();
            libcall::init(&mut p, 0); // HTTP_REQUEST
            p.data = &mut a as *mut Acc as *mut core::ffi::c_void;
            a.parser = &mut p;
            let mut pos = 0usize;
            while pos < n {
                let c = if n - pos < CHUNK { n - pos } else { CHUNK };
                let np = libcall::execute(&mut p, &st, buf[pos..].as_ptr() as *const i8, c);
                if p.http_errno() == libcall::HPE_PAUSED {
                    libcall::pause(&mut p, 0);
                    pos += np;
                    continue;
                }
                if p.http_errno() != libcall::HPE_OK {
                    eprintln!("incremental failed");
                    exit(3);
                }
                pos += np;
            }
            h = digest64(h, &a.h.to_ne_bytes());
            msgs = a.msgs;
        }
    }
    println!("op=parse_incremental in={} out={} iters={} digest={:016x}", n, msgs, iters, h);
}

fn run_parse_url(buf: &[u8], iters: i64) {
    let n = buf.len();
    let mut h: u64 = 0;
    let mut urls: u64 = 0;
    for _ in 0..iters {
        urls = 0;
        let mut p = 0usize;
        while p < n {
            let nl = buf[p..].iter().position(|&b| b == b'\n').map(|k| p + k).unwrap_or(n);
            let len = nl - p;
            if len > 0 {
                unsafe {
                    let mut u: libcall::ParsedUrl = std::mem::zeroed();
                    libcall::url_init(&mut u);
                    let r = libcall::parse_url(buf[p..].as_ptr() as *const i8, len, 0, &mut u);
                    let acc = [r as u64, u.field_set as u64, u.port as u64];
                    h = digest64(h, u64s(&acc));
                    if r == 0 {
                        for f in 0..7usize {
                            if u.field_set & (1 << f) != 0 {
                                let fd = [u.field_data[f].off as u64, u.field_data[f].len as u64];
                                h = digest64(h, u64s(&fd));
                            }
                        }
                    }
                }
                urls += 1;
            }
            p = nl + 1;
        }
    }
    println!("op=parse_url in={} out={} iters={} digest={:016x}", n, urls, iters, h);
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
        "parse_requests" => run_parse(op, 0, &buf, iters),   // HTTP_REQUEST
        "parse_responses" => run_parse(op, 1, &buf, iters),  // HTTP_RESPONSE
        "parse_incremental" => run_parse_incremental(&buf, iters),
        "parse_url" => run_parse_url(&buf, iters),
        "proto_utils" => {
            eprintln!("op proto_utils is coverage_only (not timed in purebin)");
            exit(1);
        }
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
