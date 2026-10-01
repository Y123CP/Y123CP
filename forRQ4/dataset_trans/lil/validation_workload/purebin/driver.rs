// ============================================================================
                                                                  
                                                            
                                                      
                                                                  
                                                                   
                                               
// ============================================================================

use core::ffi::c_char;
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

                                                          
fn read_file(path: &str) -> (Vec<u8>, usize) {
    let mut buf = std::fs::read(path).unwrap_or_else(|_| {
        eprintln!("cannot open {}", path);
        exit(2);
    });
    let n = buf.len();
    buf.push(0);
    (buf, n)
}

unsafe fn cstr_bytes<'a>(p: *const c_char) -> &'a [u8] {
    std::ffi::CStr::from_ptr(p).to_bytes()
}

/* WRITE callback: fold script output into the accumulator attached via
 *                                                 */
#[repr(C)]
struct Acc {
    h: u64,
    writes: u64,
}

unsafe extern "C" fn write_cb(lil: libcall::Lil, msg: *const c_char) {
    let a = libcall::lil_get_data(lil) as *mut Acc;
    if !a.is_null() && !msg.is_null() {
        (*a).h = digest64((*a).h, cstr_bytes(msg));
        (*a).writes += 1;
    }
}

fn run_script(path: &str, iters: i64) {
    let (code, n) = read_file(path);
    let mut h: u64 = 0;
    let mut writes: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let mut a = Acc { h: 0, writes: 0 };
            let lil = libcall::lil_new();
            if lil.is_null() {
                eprintln!("lil_new failed");
                exit(3);
            }
            libcall::lil_set_data(lil, &mut a as *mut Acc as *mut core::ffi::c_void);
            libcall::lil_callback_write(lil, write_cb);
            let v = libcall::lil_parse(lil, code.as_ptr() as *const c_char, n, 0);
            let s = libcall::lil_to_string(v);
            if !s.is_null() {
                h = digest64(h, cstr_bytes(s));
            }
            libcall::lil_free_value(v);
            let mut errmsg: *const c_char = std::ptr::null();
            let mut errpos: usize = 0;
            let haderr = libcall::lil_error(lil, &mut errmsg, &mut errpos) as i64 as u64;
            h = digest64(h, &haderr.to_ne_bytes());
            if haderr != 0 && !errmsg.is_null() {
                h = digest64(h, cstr_bytes(errmsg));
            }
            h = digest64(h, &a.h.to_ne_bytes());
            writes = a.writes;
            libcall::lil_free(lil);
        }
    }
    println!("op=run_script in={} out={} iters={} digest={:016x}", n, writes, iters, h);
}

/* run every script listed in a manifest (one path per line), each in a
 *                                                        */
fn run_suite(listpath: &str, iters: i64) {
    let (list, ln) = read_file(listpath);
    let mut h: u64 = 0;
    let mut scripts: u64 = 0;
    for _ in 0..iters {
        scripts = 0;
        for line in list[..ln].split(|&b| b == b'\n') {
            if line.is_empty() {
                continue;
            }
            let path = std::str::from_utf8(line).unwrap_or_else(|_| {
                eprintln!("bad path in {}", listpath);
                exit(2);
            });
            let (code, n) = read_file(path);
            unsafe {
                let mut a = Acc { h: 0, writes: 0 };
                let lil = libcall::lil_new();
                libcall::lil_set_data(lil, &mut a as *mut Acc as *mut core::ffi::c_void);
                libcall::lil_callback_write(lil, write_cb);
                let v = libcall::lil_parse(lil, code.as_ptr() as *const c_char, n, 0);
                let s = libcall::lil_to_string(v);
                if !s.is_null() {
                    h = digest64(h, cstr_bytes(s));
                }
                libcall::lil_free_value(v);
                h = digest64(h, &a.h.to_ne_bytes());
                libcall::lil_free(lil);
            }
            scripts += 1;
        }
    }
    println!("op=run_suite in={} out={} iters={} digest={:016x}", ln, scripts, iters, h);
}

/*                                                                   */
unsafe extern "C" fn native_sum(
    _lil: libcall::Lil,
    argc: usize,
    argv: *mut libcall::Value,
) -> libcall::Value {
    let mut sum: i64 = 0;
    let mut i = 0usize;
    while i < argc {
        sum = sum.wrapping_add(libcall::lil_to_integer(libcall::lil_arg(argv, i)));
        i += 1;
    }
    libcall::lil_alloc_integer(sum)
}

fn run_api_surface(iters: i64) {
    let mut h: u64 = 0;
    for _ in 0..iters {
        unsafe {
            let mut a = Acc { h: 0, writes: 0 };
            let lil = libcall::lil_new();
            libcall::lil_set_data(lil, &mut a as *mut Acc as *mut core::ffi::c_void);
            libcall::lil_callback_write(lil, write_cb);

            /* values */
            let vs = libcall::lil_alloc_string(b"hello lil\0".as_ptr() as *const c_char);
            let vi = libcall::lil_alloc_integer(424242);
            let vd = libcall::lil_alloc_double(3.5);
            let vc = libcall::lil_clone_value(vs);
            libcall::lil_append_char(vc, b'!' as c_char);
            libcall::lil_append_string(vc, b" world\0".as_ptr() as *const c_char);
            libcall::lil_append_val(vc, vi);
            let nums: [u64; 3] = [
                libcall::lil_to_integer(vi) as u64,
                libcall::lil_to_boolean(vd) as i64 as u64,
                (libcall::lil_to_double(vd) * 1000.0) as u64,
            ];
            let mut numbytes = [0u8; 24];
            numbytes[..8].copy_from_slice(&nums[0].to_ne_bytes());
            numbytes[8..16].copy_from_slice(&nums[1].to_ne_bytes());
            numbytes[16..].copy_from_slice(&nums[2].to_ne_bytes());
            h = digest64(h, &numbytes);
            h = digest64(h, cstr_bytes(libcall::lil_to_string(vc)));

            /* lists */
            let list = libcall::lil_alloc_list();
            libcall::lil_list_append(list, libcall::lil_clone_value(vs));
            libcall::lil_list_append(list, libcall::lil_clone_value(vi));
            libcall::lil_list_append(list, libcall::lil_clone_value(vd));
            let lsz = libcall::lil_list_size(list) as u64;
            h = digest64(h, &lsz.to_ne_bytes());
            let item = libcall::lil_list_get(list, 1);
            h = digest64(h, cstr_bytes(libcall::lil_to_string(item)));
            let joined = libcall::lil_list_to_value(list, 1);
            h = digest64(h, cstr_bytes(libcall::lil_to_string(joined)));
            libcall::lil_free_value(joined);
            libcall::lil_free_list(list);

            /* variables + env */
            libcall::lil_set_var(lil, b"answer\0".as_ptr() as *const c_char, vi, 0); // LIL_SETVAR_GLOBAL
            let got = libcall::lil_get_var(lil, b"answer\0".as_ptr() as *const c_char);
            h = digest64(h, cstr_bytes(libcall::lil_to_string(got)));
            let dflt = libcall::lil_get_var_or(lil, b"missing\0".as_ptr() as *const c_char, vd);
            h = digest64(h, cstr_bytes(libcall::lil_to_string(dflt)));
            let _pushed = libcall::lil_push_env(lil);
            libcall::lil_set_var(lil, b"inner\0".as_ptr() as *const c_char, vs, 2); // LIL_SETVAR_LOCAL_NEW
            libcall::lil_pop_env(lil);
            let standalone = libcall::lil_alloc_env(std::ptr::null_mut());
            libcall::lil_free_env(standalone);

            /* register + call native, expression eval, substitution */
            libcall::lil_register(lil, b"natsum\0".as_ptr() as *const c_char, native_sum);
            let mut args = [libcall::lil_alloc_integer(40), libcall::lil_alloc_integer(2)];
            let called = libcall::lil_call(lil, b"natsum\0".as_ptr() as *const c_char, 2, args.as_mut_ptr());
            h = digest64(h, cstr_bytes(libcall::lil_to_string(called)));
            libcall::lil_free_value(called);
            libcall::lil_free_value(args[0]);
            libcall::lil_free_value(args[1]);
            let exprsrc = libcall::lil_alloc_string(b"(3 + 4) * 5 - 1\0".as_ptr() as *const c_char);
            let exprv = libcall::lil_eval_expr(lil, exprsrc);
            if !exprv.is_null() {
                h = digest64(h, cstr_bytes(libcall::lil_to_string(exprv)));
                libcall::lil_free_value(exprv);
            }
            libcall::lil_free_value(exprsrc);
            let pv = libcall::lil_parse_value(lil, vs, 0);
            libcall::lil_free_value(pv);
            let substsrc = libcall::lil_alloc_string(b"val is $answer\0".as_ptr() as *const c_char);
            let sv = libcall::lil_subst_to_value(lil, substsrc);
            if !sv.is_null() {
                h = digest64(h, cstr_bytes(libcall::lil_to_string(sv)));
                libcall::lil_free_value(sv);
            }
            let sl = libcall::lil_subst_to_list(lil, substsrc);
            if !sl.is_null() {
                let ssz = libcall::lil_list_size(sl) as u64;
                h = digest64(h, &ssz.to_ne_bytes());
                libcall::lil_free_list(sl);
            }
            libcall::lil_free_value(substsrc);

            /* error surface + misc */
            libcall::lil_set_error(lil, b"synthetic error\0".as_ptr() as *const c_char);
            let mut errmsg: *const c_char = std::ptr::null();
            let mut errpos: usize = 0;
            let hade = libcall::lil_error(lil, &mut errmsg, &mut errpos) as i64 as u64;
            h = digest64(h, &hade.to_ne_bytes());
            libcall::lil_set_error_at(lil, 7, b"positioned error\0".as_ptr() as *const c_char);
            let _ = libcall::lil_error(lil, &mut errmsg, &mut errpos);
            let un = libcall::lil_unused_name(lil, b"tmp\0".as_ptr() as *const c_char);
            if !un.is_null() {
                h = digest64(h, cstr_bytes(libcall::lil_to_string(un)));
                libcall::lil_free_value(un);
            }
            libcall::lil_write(lil, b"written via lil_write\0".as_ptr() as *const c_char);
            h = digest64(h, &a.h.to_ne_bytes());

            libcall::lil_free_value(vs);
            libcall::lil_free_value(vi);
            libcall::lil_free_value(vd);
            libcall::lil_free_value(vc);
            libcall::lil_free(lil);
            libcall::lil_freemem(libcall::strdup(b"x\0".as_ptr() as *const c_char)
                as *mut core::ffi::c_void); /* allocator-matched free helper */
        }
    }
    println!("op=api_surface in=0 out=0 iters={} digest={:016x}", iters, h);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "usage: {} <run_script|api_surface> <input> <iters>",
            args.get(0).map(|s| s.as_str()).unwrap_or("driver")
        );
        exit(1);
    }
    let iters: i64 = args[3].parse().unwrap_or(0);
    if iters <= 0 {
        eprintln!("bad iters");
        exit(1);
    }
    match args[1].as_str() {
        "run_script" => run_script(&args[2], iters),
        "run_suite" => run_suite(&args[2], iters),
        "api_surface" => run_api_surface(iters),
        other => {
            eprintln!("unknown op {}", other);
            exit(1);
        }
    }
}
