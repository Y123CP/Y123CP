// W1 harness for heman — exercise distance-field + colorize compute path.
//
// Mirrors lodepng_roundtrip / csv_count pattern. Input: seed_512.bin
// (8-byte header `w,h:u32 LE` + w*h grayscale bytes). We run:
//   import_u8 → distance_create_df → color_create_gradient →
//   color_apply_gradient → export_u8 (RGB result) → sha256 → stdout
// The hash is the W1 oracle. Stage A's lifts must preserve byte-equality.

#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_float, c_int};
use std::{env, fs, process};

// Import lib's heman_image type so harness shares the SAME nominal type
// with the staged lib (post-`_unify_duplicate_pub_types`). Defining a
// local `struct heman_image` here would diverge from lib's after Stage A
// E1 strips extern "C" — call sites then fail with E0308 "expected
// `heman_image_s`, found a different `heman_image_s`".
use heman_raw::src::src::ops::heman_image;

// (sha2 removed — empirical study emits raw RGB to stdout, driver hashes it.)

// OMP stub — for fair compare with single-threaded c2rust output, run heman
// without OpenMP threading. The C-side build uses an identical stub
// (-Iomp.h shim). All `#pragma omp parallel for` decay to serial loops; the
// only runtime call is heman_get_num_threads() which we hardcode to 1.
#[no_mangle]
pub extern "C" fn omp_get_max_threads() -> core::ffi::c_int {
    1
}

type heman_byte = u8;
type heman_color = u32;

extern "C" {
    fn heman_import_u8(
        width: c_int,
        height: c_int,
        nbands: c_int,
        source: *const heman_byte,
        minval: c_float,
        maxval: c_float,
    ) -> *mut heman_image;

    fn heman_distance_create_df(monochrome: *mut heman_image) -> *mut heman_image;

    fn heman_color_create_gradient(
        width: c_int,
        num_colors: c_int,
        cp_locations: *const c_int,
        cp_values: *const heman_color,
    ) -> *mut heman_image;

    fn heman_color_apply_gradient(
        heightmap: *mut heman_image,
        minheight: c_float,
        maxheight: c_float,
        gradient: *mut heman_image,
    ) -> *mut heman_image;

    fn heman_image_info(
        img: *mut heman_image,
        width: *mut c_int,
        height: *mut c_int,
        nbands: *mut c_int,
    );

    fn heman_export_u8(
        source: *mut heman_image,
        minv: c_float,
        maxv: c_float,
        outp: *mut heman_byte,
    );

    // OPERATION 3 — GENERATE: synthesize an island heightmap from OpenSimplex
    // noise + an internal distance field (all of generate.c + noise.c, both 0%
    // under the pipeline/lighting ops). heman's headline feature + heaviest FP.
    fn heman_generate_island_heightmap(
        width: c_int,
        height: c_int,
        seed: c_int,
    ) -> *mut heman_image;

    // OPERATION 2 — LIGHTING: internally computes surface normals + ambient
    // occlusion + diffuse shading (all of lighting.c, untouched by the
    // distance+color pipeline). New module → big coverage lift.
    fn heman_lighting_apply(
        heightmap: *mut heman_image,
        colorbuffer: *mut heman_image,
        occlusion: c_float,
        diffuse: c_float,
        diffuse_softening: c_float,
        light_position: *const c_float,
    ) -> *mut heman_image;

    fn heman_image_destroy(img: *mut heman_image);
}

// OPERATION 3 — GENERATE: island heightmap from OpenSimplex noise, GEN_N times
// (~1s), export the last. Self-contained (no seed file — generate synthesizes its
// own data from fixed params). Exercises generate.c + noise.c (both 0% before).
const GEN_SZ: c_int = 512;
const GEN_SEED: c_int = 42;
const GEN_N: usize = 6; // iterations (each ~190ms; ~1.1s total)

unsafe fn generate_workload() {
    let mut last: *mut heman_image = core::ptr::null_mut();
    for i in 0..GEN_N {
        let img = heman_generate_island_heightmap(GEN_SZ, GEN_SZ, GEN_SEED);
        if i + 1 < GEN_N {
            heman_image_destroy(img);
        } else {
            last = img;
        }
    }
    let mut cw: c_int = 0;
    let mut ch: c_int = 0;
    let mut cn: c_int = 0;
    heman_image_info(last, &mut cw, &mut ch, &mut cn);
    let nout = (cw as usize) * (ch as usize) * (cn as usize);
    let mut out = vec![0u8; nout];
    heman_export_u8(last, 0.0, 1.0, out.as_mut_ptr());
    heman_image_destroy(last);
    use std::io::Write;
    std::io::stdout().write_all(&out).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // OPERATION 3 — GENERATE: argv[1]=="generate", no seed file needed.
    if args.get(1).map(|s| s.as_str()) == Some("generate") {
        unsafe { generate_workload() };
        return;
    }
    if args.len() < 2 {
        eprintln!("usage: heman_pipeline <seed.bin | generate>");
        process::exit(2);
    }
    let bytes = match fs::read(&args[1]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("heman_pipeline: read {} failed: {}", args[1], e);
            process::exit(1);
        }
    };
    if bytes.len() < 8 {
        eprintln!("heman_pipeline: file too short (need 8-byte header)");
        process::exit(1);
    }
    let w = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as c_int;
    let h = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as c_int;
    let pixels = &bytes[8..];
    if pixels.len() != (w as usize) * (h as usize) {
        eprintln!(
            "heman_pipeline: pixel buffer size {} != w*h {}",
            pixels.len(),
            w * h
        );
        process::exit(1);
    }

    // PIPE_N repeats of the full pipeline to clear the wall-clock floor on small
    // seeds; export only the LAST buffer so stdout digest stays loop-independent,
    // all intermediates destroyed each iter (no leak). Mirrors heman_pipeline_c.c.
    const PIPE_N: usize = 3;
    let lighting_mode = args.get(2).map(|s| s.as_str()) == Some("lighting");
    let mut raw_rgb: Vec<u8> = Vec::new();
    for _ in 0..PIPE_N {
        raw_rgb = unsafe {
            // 1. Import grayscale bytes as heman_image (nbands=1, range [0,1]).
            let src_img = heman_import_u8(w, h, 1, pixels.as_ptr(), 0.0, 1.0);
            if src_img.is_null() {
                eprintln!("heman_import_u8 returned null");
                process::exit(1);
            }
            // 2. Distance-field: the hot compute path for image-stencil domain.
            let df = heman_distance_create_df(src_img);
            // 3. Build a fixed gradient (5 stops, deterministic).
            let cp_locs: [c_int; 5] = [0, 64, 128, 192, 255];
            let cp_vals: [heman_color; 5] = [0x000033, 0x0055AA, 0x33AA66, 0xCCAA22, 0xFFEECC];
            let gradient = heman_color_create_gradient(256, 5, cp_locs.as_ptr(), cp_vals.as_ptr());
            // 4. Apply gradient (df range is roughly [-1, 1] after default scaling).
            let colored = heman_color_apply_gradient(df, -1.0, 1.0, gradient);
            // 4b. OPERATION 2 — LIGHTING (fixed params) when requested.
            let final_img = if lighting_mode {
                let light_position: [c_float; 3] = [-0.5, 0.5, 1.0];
                heman_lighting_apply(src_img, colored, 1.0, 1.0, 0.5, light_position.as_ptr())
            } else {
                colored
            };
            // 5. Export RGB result to bytes.
            let mut cw: c_int = 0;
            let mut ch: c_int = 0;
            let mut cn: c_int = 0;
            heman_image_info(final_img, &mut cw, &mut ch, &mut cn);
            let nout = (cw as usize) * (ch as usize) * (cn as usize);
            let mut out = vec![0u8; nout];
            heman_export_u8(final_img, 0.0, 1.0, out.as_mut_ptr());
            // 6. Clean up all intermediates this iteration.
            if lighting_mode {
                heman_image_destroy(final_img);
            }
            heman_image_destroy(colored);
            heman_image_destroy(gradient);
            heman_image_destroy(df);
            heman_image_destroy(src_img);
            out
        };
    }
    // Empirical study: emit the raw RGB buffer to stdout; the driver hashes stdout
    // for the equivalence gate. sha256 was REMOVED from the timed region — it (a)
    // diluted heman's compute and (b) used a different impl per side (OpenSSL SHA256
    // vs the sha2 crate), an UNFAIR non-cancelling cost that inflated the gap.
    use std::io::Write;
    std::io::stdout().write_all(&raw_rgb).unwrap();
}
