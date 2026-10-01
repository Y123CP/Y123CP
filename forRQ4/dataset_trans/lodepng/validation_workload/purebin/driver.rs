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
fn u64_bytes(s: &[u64]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, std::mem::size_of_val(s)) }
}

#[inline]
unsafe fn buf_slice<'a>(p: *const u8, n: usize) -> &'a [u8] {
    std::slice::from_raw_parts(p, n)
}

type CChar = core::ffi::c_char;

/* PNG -> RGBA32 */
fn run_decode32(png: &[u8], n: usize, iters: i64) {
    let mut h: u64 = 0;
    let (mut w, mut ht): (u32, u32) = (0, 0);
    let mut outbytes = 0usize;
    for _ in 0..iters {
        unsafe {
            let mut img: *mut u8 = std::ptr::null_mut();
            let err = libcall::lodepng_decode32(&mut img, &mut w, &mut ht, png.as_ptr(), n);
            if err != 0 {
                eprintln!("decode32 failed: {}", err);
                exit(3);
            }
            outbytes = (w as usize) * (ht as usize) * 4;
            h = digest64(h, buf_slice(img, outbytes));
            libcall::free(img as *mut core::ffi::c_void);
        }
    }
    println!("op=decode32 in={} out={} iters={} digest={:016x}", n, outbytes, iters, h);
}

/* PNG -> raw color type (no conversion), via state-based decoder */
fn run_decode_keep(png: &[u8], n: usize, iters: i64) {
    let mut h: u64 = 0;
    let mut outbytes = 0usize;
    for _ in 0..iters {
        unsafe {
            let mut state: libcall::LodePNGState = std::mem::zeroed();
            libcall::lodepng_state_init(&mut state);
            state.decoder.color_convert = 0;
            let mut img: *mut u8 = std::ptr::null_mut();
            let (mut w, mut ht): (u32, u32) = (0, 0);
            let err = libcall::lodepng_decode(&mut img, &mut w, &mut ht, &mut state, png.as_ptr(), n);
            if err != 0 {
                eprintln!("decode_keep failed: {}", err);
                exit(3);
            }
            outbytes = libcall::lodepng_get_raw_size(w, ht, &state.info_raw);
            h = digest64(h, buf_slice(img, outbytes));
            libcall::free(img as *mut core::ffi::c_void);
            libcall::lodepng_state_cleanup(&mut state);
        }
    }
    println!("op=decode_keep in={} out={} iters={} digest={:016x}", n, outbytes, iters, h);
}

/* .rgba file: u32 LE w, u32 LE h, then w*h*4 bytes of RGBA pixels */
fn rgba_header(file: &[u8]) -> (u32, u32) {
    if file.len() < 8 {
        eprintln!("bad rgba input");
        exit(2);
    }
    let w = u32::from_le_bytes(file[0..4].try_into().unwrap());
    let ht = u32::from_le_bytes(file[4..8].try_into().unwrap());
    (w, ht)
}

fn run_encode32(file: &[u8], file_n: usize, iters: i64) {
    let (w, ht) = rgba_header(file);
    if file_n != 8 + (w as usize) * (ht as usize) * 4 {
        eprintln!("rgba size mismatch");
        exit(2);
    }
    let pixels = &file[8..];
    let mut h: u64 = 0;
    let mut outsize = 0usize;
    for _ in 0..iters {
        unsafe {
            let mut png: *mut u8 = std::ptr::null_mut();
            let err = libcall::lodepng_encode32(&mut png, &mut outsize, pixels.as_ptr(), w, ht);
            if err != 0 {
                eprintln!("encode32 failed: {}", err);
                exit(3);
            }
            h = digest64(h, buf_slice(png, outsize));
            libcall::free(png as *mut core::ffi::c_void);
        }
    }
    println!("op=encode32 in={} out={} iters={} digest={:016x}", file_n, outsize, iters, h);
}

/* parse utilities: chunk walk + inspect + crc32 over the whole file */
fn run_chunk_crc(png: &[u8], n: usize, iters: i64) {
    let mut h: u64 = 0;
    let base = png.as_ptr();
    for _ in 0..iters {
        unsafe {
            let mut acc = [0u64; 8];
            let mut state: libcall::LodePNGState = std::mem::zeroed();
            libcall::lodepng_state_init(&mut state);
            let (mut w, mut ht): (u32, u32) = (0, 0);
            let err = libcall::lodepng_inspect(&mut w, &mut ht, &mut state, base, n);
            if err != 0 {
                eprintln!("inspect failed: {}", err);
                exit(3);
            }
            acc[0] = w as u64;
            acc[1] = ht as u64;
            acc[2] = state.info_png.color.colortype as u64;
            acc[3] = state.info_png.color.bitdepth as u64;
            libcall::lodepng_state_cleanup(&mut state);
            if n > 8 {
                let mut chunk = base.add(8);
                let end = base.add(n);
                while chunk.add(12) <= end {
                    let len = libcall::lodepng_chunk_length(chunk);
                    let mut ctype = [0 as CChar; 5];
                    libcall::lodepng_chunk_type(ctype.as_mut_ptr(), chunk);
                    acc[4] += len as u64;
                    acc[5] += libcall::lodepng_chunk_ancillary(chunk) as u64;
                    acc[6] += libcall::lodepng_chunk_check_crc(chunk) as u64;
                    let t = std::slice::from_raw_parts(ctype.as_ptr() as *const u8, 4);
                    if t == b"IEND" {
                        break;
                    }
                    if chunk.add(12 + len as usize) > end {
                        break;
                    }
                    chunk = libcall::lodepng_chunk_next_const(chunk, end);
                    if chunk.is_null() {
                        break;
                    }
                }
            }
            acc[7] = libcall::lodepng_crc32(base, n) as u64;
            h = digest64(h, u64_bytes(&acc));

            /* chunk surgery + misc utilities (deterministic, folded into digest) */
            let mut acc2 = [0u64; 8];
            let end = base.add(n);
            let ihdr = libcall::lodepng_chunk_find_const(base.add(8), end, b"IHDR\0".as_ptr() as *const CChar);
            if !ihdr.is_null() {
                acc2[0] = libcall::lodepng_chunk_length(ihdr) as u64;
                acc2[1] = libcall::lodepng_chunk_private(ihdr) as u64
                    | ((libcall::lodepng_chunk_safetocopy(ihdr) as u64) << 1);
                let mut st2: libcall::LodePNGState = std::mem::zeroed();
                libcall::lodepng_state_init(&mut st2);
                let (mut w2, mut h2): (u32, u32) = (0, 0);
                libcall::lodepng_inspect(&mut w2, &mut h2, &mut st2, base, n);
                /* inspect_chunk on the first chunk after IHDR */
                let nxt = libcall::lodepng_chunk_next_const(ihdr, end);
                if !nxt.is_null() && nxt < end {
                    let _ = libcall::lodepng_inspect_chunk(&mut st2, nxt.offset_from(base) as usize, base, n);
                }
                let mut st3: libcall::LodePNGState = std::mem::zeroed();
                libcall::lodepng_state_init(&mut st3);
                libcall::lodepng_state_copy(&mut st3, &st2);
                acc2[2] = st3.info_png.color.colortype as u64;
                libcall::lodepng_state_cleanup(&mut st3);
                libcall::lodepng_state_cleanup(&mut st2);
            }
            /* build a tiny chunk buffer: create + append + find + mutate */
            {
                let mut buf2: *mut u8 = std::ptr::null_mut();
                let mut bn = 0usize;
                let payload: [u8; 4] = [1, 2, 3, 4];
                if libcall::lodepng_chunk_create(&mut buf2, &mut bn, 4,
                        b"tEXt\0".as_ptr() as *const CChar, payload.as_ptr()) == 0
                    && !buf2.is_null()
                {
                    let c = buf2;
                    acc2[3] = libcall::lodepng_chunk_length(c) as u64;
                    acc2[4] = libcall::lodepng_chunk_type_equals(c, b"tEXt\0".as_ptr() as *const CChar) as u64
                        | ((libcall::lodepng_chunk_ancillary(c) as u64) << 2);
                    let data = libcall::lodepng_chunk_data(c);
                    if !data.is_null() {
                        acc2[5] = *data as u64;
                    }
                    let find = libcall::lodepng_chunk_find(buf2, buf2.add(bn), b"tEXt\0".as_ptr() as *const CChar);
                    acc2[6] = (find == buf2) as u64;
                    if !find.is_null() {
                        let _nx = libcall::lodepng_chunk_next(find, buf2.add(bn));
                    }
                    libcall::lodepng_chunk_generate_crc(c);
                    acc2[7] = libcall::lodepng_chunk_check_crc(c) as u64;
                    let _ = libcall::lodepng_chunk_append(&mut buf2, &mut bn, c);
                    libcall::free(buf2 as *mut core::ffi::c_void);
                }
            }
            h = digest64(h, u64_bytes(&acc2));

            /* color-mode + error-text helpers */
            let mut acc3 = [0u64; 6];
            let mut pal = libcall::lodepng_color_mode_make(libcall::LCT_PALETTE, 8);
            for k in 0u32..4 {
                libcall::lodepng_palette_add(&mut pal, k as u8, (k * 2) as u8, (k * 3) as u8, 255);
            }
            acc3[0] = libcall::lodepng_get_bpp(&pal) as u64;
            acc3[1] = libcall::lodepng_get_channels(&pal) as u64;
            acc3[2] = libcall::lodepng_is_palette_type(&pal) as u64
                | ((libcall::lodepng_has_palette_alpha(&pal) as u64) << 1)
                | ((libcall::lodepng_is_alpha_type(&pal) as u64) << 2)
                | ((libcall::lodepng_is_greyscale_type(&pal) as u64) << 3);
            let mut palcopy: libcall::LodePNGColorMode = std::mem::zeroed();
            libcall::lodepng_color_mode_init(&mut palcopy);
            libcall::lodepng_color_mode_copy(&mut palcopy, &pal);
            acc3[3] = libcall::lodepng_can_have_alpha(&palcopy) as u64;
            libcall::lodepng_color_mode_cleanup(&mut palcopy);
            libcall::lodepng_color_mode_cleanup(&mut pal);
            let mut elen: u64 = 0;
            for e in 0u32..96 {
                let s = libcall::lodepng_error_text(e);
                elen += std::ffi::CStr::from_ptr(s).to_bytes().len() as u64;
            }
            acc3[4] = elen;
            h = digest64(h, u64_bytes(&acc3));
        }
    }
    println!("op=chunk_crc in={} out=0 iters={} digest={:016x}", n, iters, h);
}

/* PNG file path -> RGBA32 via the _file entry points */
fn run_decode_file(path: &str, iters: i64) {
    let cpath = std::ffi::CString::new(path).unwrap();
    let mut h: u64 = 0;
    let (mut w, mut ht): (u32, u32) = (0, 0);
    let mut outbytes = 0usize;
    for _ in 0..iters {
        unsafe {
            let mut img: *mut u8 = std::ptr::null_mut();
            let err = libcall::lodepng_decode32_file(&mut img, &mut w, &mut ht, cpath.as_ptr());
            if err != 0 {
                eprintln!("decode32_file failed: {}", err);
                exit(3);
            }
            outbytes = (w as usize) * (ht as usize) * 4;
            h = digest64(h, buf_slice(img, outbytes));
            libcall::free(img as *mut core::ffi::c_void);
        }
    }
    println!("op=decode_file in=0 out={} iters={} digest={:016x}", outbytes, iters, h);
}

unsafe fn set_meta(state: &mut libcall::LodePNGState) {
    let info = &mut state.info_png;
    libcall::lodepng_add_text(info, b"Title\0".as_ptr() as *const CChar,
        b"validation workload\0".as_ptr() as *const CChar);
    libcall::lodepng_add_text(info, b"Author\0".as_ptr() as *const CChar,
        b"reference encoder\0".as_ptr() as *const CChar);
    libcall::lodepng_add_itext(info, b"Comment\0".as_ptr() as *const CChar,
        b"en\0".as_ptr() as *const CChar, b"comment\0".as_ptr() as *const CChar,
        b"three-way perf validation\0".as_ptr() as *const CChar);
    info.gama_defined = 1; info.gama_gamma = 45455;
    info.phys_defined = 1; info.phys_x = 2835; info.phys_y = 2835; info.phys_unit = 1;
    info.time_defined = 1;
    info.time.year = 2026; info.time.month = 7; info.time.day = 16;
    info.time.hour = 12; info.time.minute = 0; info.time.second = 0;
    info.background_defined = 1;
    info.background_r = 255; info.background_g = 255; info.background_b = 255;
    info.srgb_defined = 1; info.srgb_intent = 0;
    info.chrm_defined = 1;
    info.chrm_white_x = 31270; info.chrm_white_y = 32900;
    info.chrm_red_x = 64000; info.chrm_red_y = 33000;
    info.chrm_green_x = 30000; info.chrm_green_y = 60000;
    info.chrm_blue_x = 15000; info.chrm_blue_y = 6000;
    info.sbit_defined = 1; info.sbit_r = 8; info.sbit_g = 8; info.sbit_b = 8; info.sbit_a = 8;
}

/* .rgba -> PNG carrying the common ancillary chunks (zTXt via
 * text_compression=1) with auto color-model selection.
 *                                                               
 *                            /                                    
 *                       /c/                 */
fn run_encode_meta(file: &[u8], file_n: usize, iters: i64) {
    let (w, ht) = rgba_header(file);
    let pixels = &file[8..];
    let mut h: u64 = 0;
    let mut outsize = 0usize;
    for _ in 0..iters {
        unsafe {
            let mut state: libcall::LodePNGState = std::mem::zeroed();
            libcall::lodepng_state_init(&mut state);
            set_meta(&mut state);
            state.encoder.text_compression = 1; /* zTXt path */
            state.encoder.auto_convert = 1; /* color stats + model selection */
            state.encoder.filter_strategy = libcall::LFS_MINSUM;
            let mut png: *mut u8 = std::ptr::null_mut();
            let err = libcall::lodepng_encode(&mut png, &mut outsize, pixels.as_ptr(), w, ht, &mut state);
            if err != 0 {
                eprintln!("encode_meta failed: {}", err);
                exit(3);
            }
            h = digest64(h, buf_slice(png, outsize));
            libcall::free(png as *mut core::ffi::c_void);
            libcall::lodepng_state_cleanup(&mut state);
        }
    }
    println!("op=encode_meta in={} out={} iters={} digest={:016x}", file_n, outsize, iters, h);
}

/* .rgba -> PNG over a settings matrix: filter strategies x zlib btypes */
fn run_encode_settings(file: &[u8], file_n: usize, iters: i64) {
    let (w, ht) = rgba_header(file);
    let pixels = &file[8..];
    const STRATEGIES: [u32; 4] = [
        0, /* LFS_ZERO */ 5, /* LFS_MINSUM */ 6, /* LFS_ENTROPY */ 7, /* LFS_BRUTE_FORCE */
    ];
    const BTYPES: [u32; 3] = [0, 1, 2];
    let mut h: u64 = 0;
    let mut total: u64 = 0;
    for _ in 0..iters {
        total = 0;
        for s in 0..4 {
            for b in 0..3 {
                unsafe {
                    let mut state: libcall::LodePNGState = std::mem::zeroed();
                    libcall::lodepng_state_init(&mut state);
                    state.encoder.filter_strategy = STRATEGIES[s];
                    state.encoder.zlibsettings.btype = BTYPES[b];
                    let mut png: *mut u8 = std::ptr::null_mut();
                    let mut outsize = 0usize;
                    let err = libcall::lodepng_encode(&mut png, &mut outsize, pixels.as_ptr(), w, ht, &mut state);
                    if err != 0 {
                        eprintln!("encode_settings failed: {}", err);
                        exit(3);
                    }
                    h = digest64(h, buf_slice(png, outsize));
                    total += outsize as u64;
                    libcall::free(png as *mut core::ffi::c_void);
                    libcall::lodepng_state_cleanup(&mut state);
                }
            }
        }
    }
    println!("op=encode_settings in={} out={} iters={} digest={:016x}", file_n, total, iters, h);
}

/* zlib compress + decompress + raw-deflate inflate over the input bytes */
fn run_zlib_roundtrip(buf: &[u8], n: usize, iters: i64) {
    unsafe {
        let mut cs: libcall::LodePNGCompressSettings = std::mem::zeroed();
        libcall::lodepng_compress_settings_init(&mut cs);
        let mut ds: libcall::LodePNGDecompressSettings = std::mem::zeroed();
        libcall::lodepng_decompress_settings_init(&mut ds);
        let mut h: u64 = 0;
        for _ in 0..iters {
            let mut z: *mut u8 = std::ptr::null_mut();
            let mut zn = 0usize;
            if libcall::lodepng_zlib_compress(&mut z, &mut zn, buf.as_ptr(), n, &cs) != 0 {
                eprintln!("zlib compress failed");
                exit(3);
            }
            h = digest64(h, buf_slice(z, zn));
            let mut back: *mut u8 = std::ptr::null_mut();
            let mut bn = 0usize;
            if libcall::lodepng_zlib_decompress(&mut back, &mut bn, z, zn, &ds) != 0 || bn != n {
                eprintln!("zlib decompress failed");
                exit(3);
            }
            h = digest64(h, buf_slice(back, bn));
            /* raw deflate stream = zlib payload without the 2-byte header */
            let mut raw: *mut u8 = std::ptr::null_mut();
            let mut rn = 0usize;
            if libcall::lodepng_inflate(&mut raw, &mut rn, z.add(2), zn - 2, &ds) != 0 || rn != n {
                eprintln!("inflate failed");
                exit(3);
            }
            h = digest64(h, buf_slice(raw, rn));
            libcall::free(z as *mut core::ffi::c_void);
            libcall::free(back as *mut core::ffi::c_void);
            libcall::free(raw as *mut core::ffi::c_void);
        }
        println!("op=zlib_roundtrip in={} out={} iters={} digest={:016x}", n, n, iters, h);
    }
}

/* RGBA -> RGB / grey / grey16 / RGBA16 conversions (lodepng_convert) */
fn run_convert(file: &[u8], file_n: usize, iters: i64) {
    let (w, ht) = rgba_header(file);
    let pixels = &file[8..];
    unsafe {
        let mut in_mode: libcall::LodePNGColorMode = std::mem::zeroed();
        libcall::lodepng_color_mode_init(&mut in_mode);
        in_mode.colortype = libcall::LCT_RGBA;
        in_mode.bitdepth = 8;
        const OUTS: [(u32, u32); 4] = [
            (libcall::LCT_RGB, 8), (libcall::LCT_GREY, 8),
            (libcall::LCT_GREY, 16), (libcall::LCT_RGBA, 16),
        ];
        let mut h: u64 = 0;
        for _ in 0..iters {
            for k in 0..4 {
                let mut out_mode: libcall::LodePNGColorMode = std::mem::zeroed();
                libcall::lodepng_color_mode_init(&mut out_mode);
                out_mode.colortype = OUTS[k].0;
                out_mode.bitdepth = OUTS[k].1;
                let outbytes = libcall::lodepng_get_raw_size(w, ht, &out_mode);
                let out = libcall::malloc(outbytes) as *mut u8;
                if out.is_null() {
                    eprintln!("oom");
                    exit(2);
                }
                let err = libcall::lodepng_convert(out, pixels.as_ptr(), &out_mode, &in_mode, w, ht);
                if err != 0 {
                    eprintln!("convert failed: {}", err);
                    exit(3);
                }
                h = digest64(h, buf_slice(out, outbytes));
                libcall::free(out as *mut core::ffi::c_void);
                libcall::lodepng_color_mode_cleanup(&mut out_mode);
            }
        }
        println!("op=convert in={} out=0 iters={} digest={:016x}", file_n, iters, h);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 && (args[1] == "gen_rgba" || args[1] == "gen_tile" || args[1] == "gen_variant") {
        eprintln!("op {} is a C-side input generator (not replicated in purebin)", args[1]);
        exit(1);
    }
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
    let n = buf.len();

    match op {
        "decode32" => run_decode32(&buf, n, iters),
        "decode_keep" => run_decode_keep(&buf, n, iters),
        "decode_file" => run_decode_file(&args[2], iters),
        "encode32" => run_encode32(&buf, n, iters),
        "encode_meta" => run_encode_meta(&buf, n, iters),
        "encode_settings" => run_encode_settings(&buf, n, iters),
        "zlib_roundtrip" => run_zlib_roundtrip(&buf, n, iters),
        "convert" => run_convert(&buf, n, iters),
        "chunk_crc" => run_chunk_crc(&buf, n, iters),
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }
}
