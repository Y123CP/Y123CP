// ============================================================================
                                                                     
                                                            
                                                           
                                   
                                                                  
                                                 
//   MODE_GENERIC=0 MODE_TEXT=1 | PARAM_QUALITY=1 PARAM_LGWIN=2
//   OP_PROCESS=0 OP_FINISH=2 OP_EMIT_METADATA=3 | DEC_PARAM_LARGE_WINDOW=1
//   DEC_RESULT_ERROR=0 SUCCESS=1 NEEDS_MORE_INPUT=2 | SHARED_DICTIONARY_RAW=0
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
fn u64s_bytes(s: &[u64]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, std::mem::size_of_val(s)) }
}

/* ------------------------- compress: q{0,1,3,5,9} ------------------------- */
fn run_compress(src: &[u8], iters: i64) {
    const QS: [i32; 5] = [0, 1, 3, 5, 9];
    let n = src.len();
    let cap = libcall::encoder_max_compressed_size(n);
    let mut out = vec![0u8; cap];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    for _ in 0..iters {
        total = 0;
        for q in QS {
            let mut outlen = cap;
            let ok = unsafe {
                libcall::encoder_compress(q, 22, 0 /*MODE_GENERIC*/, n,
                                          src.as_ptr(), &mut outlen, out.as_mut_ptr())
            };
            if ok == 0 { eprintln!("compress q{} failed", q); exit(3); }
            h = digest64(h, &out[..outlen]);
            total += outlen as u64;
        }
    }
    println!("op=compress in={} out={} iters={} digest={:016x}", n, total, iters, h);
}

/* --------------------- compress_hq: q10 zopfli + q11 ---------------------- */
fn run_compress_hq(src: &[u8], iters: i64) {
    let n = src.len();
    let cap = libcall::encoder_max_compressed_size(n);
    let mut out = vec![0u8; cap];
    let mut h: u64 = 0;
    let mut outlen: usize = 0;
    for _ in 0..iters {
        for q in 10..=11i32 {
            outlen = cap;
            let ok = unsafe {
                libcall::encoder_compress(q, 24, 1 /*MODE_TEXT*/, n,
                                          src.as_ptr(), &mut outlen, out.as_mut_ptr())
            };
            if ok == 0 { eprintln!("compress q{} failed", q); exit(3); }
            h = digest64(h, &out[..outlen]);
        }
    }
    println!("op=compress_hq in={} out={} iters={} digest={:016x}", n, outlen, iters, h);
}

/* ----------- decompress: .br file = u64 LE orig size + stream ------------- */
fn run_decompress(file: &[u8], iters: i64) {
    let file_n = file.len();
    if file_n < 8 { eprintln!("bad .br input"); exit(2); }
    let orig = u64::from_le_bytes(file[..8].try_into().unwrap());
    let mut out = vec![0u8; orig as usize];
    let mut h: u64 = 0;
    for _ in 0..iters {
        let mut outlen = orig as usize;
        let r = unsafe {
            libcall::decoder_decompress(file_n - 8, file.as_ptr().add(8),
                                        &mut outlen, out.as_mut_ptr())
        };
        if r != 1 /*SUCCESS*/ || outlen as u64 != orig {
            eprintln!("decompress failed"); exit(3);
        }
        h = digest64(h, &out[..outlen]);
    }
    println!("op=decompress in={} out={} iters={} digest={:016x}", file_n, orig, iters, h);
}

/* -------- metadata callbacks: fold sizes/bytes into the accumulator ------- */
static mut G_META_H: u64 = 0;
unsafe extern "C" fn meta_start(_opaque: *mut core::ffi::c_void, size: usize) {
    let s = size as u64;
    G_META_H = digest64(G_META_H, &s.to_ne_bytes());
}
unsafe extern "C" fn meta_chunk(_opaque: *mut core::ffi::c_void, data: *const u8, size: usize) {
    G_META_H = digest64(G_META_H, std::slice::from_raw_parts(data, size));
}

/* ------------------ stream_roundtrip: TakeOutput windows ------------------ */
fn run_stream_roundtrip(src: &[u8], iters: i64) {
    const IN_CHUNK: usize = 65536;
    const OUT_WIN: usize = 4096;
    const DEC_CHUNK: usize = 7;
    // C: static const uint8_t metadata[] = "vw-metadata-block";
                                                      
    const METADATA: &[u8] = b"vw-metadata-block";
    let n = src.len();
    let cap = libcall::encoder_max_compressed_size(n) + 4096;
    let mut comp = vec![0u8; cap];
    let mut back = vec![0u8; n];
    let mut outwin = [0u8; OUT_WIN];
    let mut h: u64 = 0;
    let mut comp_n: usize = 0;
    for _ in 0..iters {
        /* --- chunked encode via small window + TakeOutput drain --- */
        let es = unsafe { libcall::encoder_create_instance() };
        unsafe {
            libcall::encoder_set_parameter(es, 1 /*QUALITY*/, 5);
            libcall::encoder_set_parameter(es, 2 /*LGWIN*/, 22);
        }
        comp_n = 0;
        let mut pos: usize = 0;
        let mut meta_sent = false;
        loop {
            let c = if n - pos < IN_CHUNK { n - pos } else { IN_CHUNK };
            let mut next_in: *const u8;
            let mut avail_in: usize;
            let eop: u32;
            if !meta_sent && pos >= n / 2 {
                next_in = METADATA.as_ptr();
                avail_in = METADATA.len();
                eop = 3; /*EMIT_METADATA*/
                meta_sent = true;
            } else {
                next_in = unsafe { src.as_ptr().add(pos) };
                avail_in = c;
                pos += c;
                eop = if pos >= n { 2 /*FINISH*/ } else { 0 /*PROCESS*/ };
            }
            loop {
                let mut next_out = outwin.as_mut_ptr();
                let mut avail_out = OUT_WIN;
                let ok = unsafe {
                    libcall::encoder_compress_stream(es, eop, &mut avail_in, &mut next_in,
                                                     &mut avail_out, &mut next_out,
                                                     std::ptr::null_mut())
                };
                if ok == 0 { eprintln!("stream compress failed"); exit(3); }
                let got = OUT_WIN - avail_out;
                comp[comp_n..comp_n + got].copy_from_slice(&outwin[..got]);
                comp_n += got;
                while unsafe { libcall::encoder_has_more_output(es) } != 0 {
                    let mut take: usize = 0;
                    let tp = unsafe { libcall::encoder_take_output(es, &mut take) };
                    if tp.is_null() || take == 0 { break; }
                    unsafe {
                        std::ptr::copy_nonoverlapping(tp, comp.as_mut_ptr().add(comp_n), take);
                    }
                    comp_n += take;
                }
                if avail_in == 0 { break; } // C: do {} while (avail_in > 0)
            }
            if unsafe { libcall::encoder_is_finished(es) } != 0 { break; }
        }
        h = digest64(h, &comp[..comp_n]);
        unsafe { libcall::encoder_destroy_instance(es); }

        /* --- tiny-chunk decode (safe bit-reader paths) + metadata cbs --- */
        let ds = unsafe { libcall::decoder_create_instance() };
        unsafe {
            libcall::decoder_set_parameter(ds, 1 /*LARGE_WINDOW*/, 0);
            G_META_H = 0;
            libcall::decoder_set_metadata_callbacks(ds, Some(meta_start), Some(meta_chunk),
                                                    std::ptr::null_mut());
        }
        let mut back_n: usize = 0;
        let mut dpos: usize = 0;
        let mut din: usize = 0;
        let mut dnext_in: *const u8 = comp.as_ptr();
        let mut r: u32 = 2; /*NEEDS_MORE_INPUT*/
        while r != 1 /*SUCCESS*/ {
            if r == 2 /*NEEDS_MORE_INPUT*/ {
                if dpos >= comp_n { eprintln!("stream decode starved"); exit(3); }
                let c = if comp_n - dpos < DEC_CHUNK { comp_n - dpos } else { DEC_CHUNK };
                dnext_in = unsafe { comp.as_ptr().add(dpos) };
                din = c;
                dpos += c;
            }
            let mut dnext_out = outwin.as_mut_ptr();
            let mut dout = OUT_WIN;
            r = unsafe {
                libcall::decoder_decompress_stream(ds, &mut din, &mut dnext_in,
                                                   &mut dout, &mut dnext_out,
                                                   std::ptr::null_mut())
            };
            if r == 0 /*ERROR*/ {
                let es_ = unsafe {
                    libcall::decoder_error_string(libcall::decoder_get_error_code(ds))
                };
                let msg = unsafe { std::ffi::CStr::from_ptr(es_) };
                eprintln!("stream decode failed: {}", msg.to_string_lossy());
                exit(3);
            }
            let got = OUT_WIN - dout;
            back[back_n..back_n + got].copy_from_slice(&outwin[..got]);
            back_n += got;
            while unsafe { libcall::decoder_has_more_output(ds) } != 0 {
                let mut take: usize = 0;
                let tp = unsafe { libcall::decoder_take_output(ds, &mut take) };
                if tp.is_null() || take == 0 { break; }
                unsafe {
                    std::ptr::copy_nonoverlapping(tp, back.as_mut_ptr().add(back_n), take);
                }
                back_n += take;
            }
        }
        let used: [u64; 3] = [
            unsafe { libcall::decoder_is_used(ds) } as u64,
            unsafe { libcall::decoder_is_finished(ds) } as u64,
            unsafe { G_META_H },
        ];
        h = digest64(h, u64s_bytes(&used));
        if back_n != n || &back[..back_n] != src {
            eprintln!("stream roundtrip mismatch ({}/{})", back_n, n); exit(3);
        }
        h = digest64(h, &back[..back_n]);
        unsafe { libcall::decoder_destroy_instance(ds); }
    }
    println!("op=stream_roundtrip in={} out={} iters={} digest={:016x}", n, comp_n, iters, h);
}

/* ------ dict_roundtrip: first 64KB is the dict, rest is the payload ------- */
fn run_dict_roundtrip(src: &[u8], iters: i64) {
    const DICT: usize = 65536;
    let n = src.len();
    if n <= DICT + 4096 { eprintln!("input too small"); exit(2); }
    let dict = &src[..DICT];
    let data = &src[DICT..];
    let dn = n - DICT;
    let cap = libcall::encoder_max_compressed_size(dn);
    let mut comp = vec![0u8; cap];
    let mut back = vec![0u8; dn];
    let mut h: u64 = 0;
    for _ in 0..iters {
        let pd = unsafe {
            libcall::encoder_prepare_dictionary(0 /*SHARED_DICTIONARY_RAW*/, DICT,
                                                dict.as_ptr(), 5)
        };
        if pd.is_null() { eprintln!("prepare dict failed"); exit(3); }
        let pds = unsafe { libcall::encoder_get_prepared_dictionary_size(pd) } as u64;
        h = digest64(h, &pds.to_ne_bytes());
        let es = unsafe { libcall::encoder_create_instance() };
        unsafe { libcall::encoder_set_parameter(es, 1 /*QUALITY*/, 5); }
        if unsafe { libcall::encoder_attach_prepared_dictionary(es, pd) } == 0 {
            eprintln!("attach dict failed"); exit(3);
        }
        let mut avail_in = dn;
        let mut avail_out = cap;
        let mut next_in = data.as_ptr();
        let mut next_out = comp.as_mut_ptr();
        let ok = unsafe {
            libcall::encoder_compress_stream(es, 2 /*FINISH*/, &mut avail_in, &mut next_in,
                                             &mut avail_out, &mut next_out,
                                             std::ptr::null_mut())
        };
        if ok == 0 || unsafe { libcall::encoder_is_finished(es) } == 0 {
            eprintln!("dict compress failed"); exit(3);
        }
        let comp_n = cap - avail_out;
        h = digest64(h, &comp[..comp_n]);
        unsafe {
            libcall::encoder_destroy_instance(es);
            libcall::encoder_destroy_prepared_dictionary(pd);
        }

        let ds = unsafe { libcall::decoder_create_instance() };
        if unsafe { libcall::decoder_attach_dictionary(ds, 0 /*RAW*/, DICT, dict.as_ptr()) } == 0 {
            eprintln!("decoder attach failed"); exit(3);
        }
        let mut din = comp_n;
        let mut dout = dn;
        let mut dnext_in = comp.as_ptr();
        let mut dnext_out = back.as_mut_ptr();
        let dr = unsafe {
            libcall::decoder_decompress_stream(ds, &mut din, &mut dnext_in,
                                               &mut dout, &mut dnext_out,
                                               std::ptr::null_mut())
        };
        if dr != 1 /*SUCCESS*/ || dout != 0 || &back[..dn] != data {
            eprintln!("dict roundtrip mismatch (r={})", dr); exit(3);
        }
        h = digest64(h, &back[..dn]);
        unsafe { libcall::decoder_destroy_instance(ds); }
    }
    println!("op=dict_roundtrip in={} out=0 iters={} digest={:016x}", n, iters, h);
}

fn main() {
    unsafe {
        libcall::encoder_ensure_static_init();
        libcall::decoder_ensure_static_init();
    }
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: {} <op> <input> <iters>",
                  args.get(0).map(|s| s.as_str()).unwrap_or("driver"));
        exit(1);
    }
    let op = args[1].as_str();
    let iters: i64 = args[3].parse().unwrap_or(0);
    if iters <= 0 { eprintln!("bad iters"); exit(1); }
    let buf = std::fs::read(&args[2]).unwrap_or_else(|_| {
        eprintln!("cannot read {}", args[2]);
        exit(2);
    });

    match op {
        "compress"         => run_compress(&buf, iters),
        "compress_hq"      => run_compress_hq(&buf, iters),
        "decompress"       => run_decompress(&buf, iters),
        "stream_roundtrip" => run_stream_roundtrip(&buf, iters),
        "dict_roundtrip"   => run_dict_roundtrip(&buf, iters),
        "misc" => {
            eprintln!("op misc is coverage_only (not timed in purebin)");
            exit(1);
        }
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
