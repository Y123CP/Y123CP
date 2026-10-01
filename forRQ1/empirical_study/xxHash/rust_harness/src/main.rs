// xxh_bench — empirical-study harness for xxHash (0_raw / c2rust_raw).
//
// EMPIRICAL STUDY (2026-07-01): xxHash freshly STAGED (clone Cyan4973/xxHash →
// c2rust transpile → 0_raw). Two SOURCE ADAPTATIONS were needed for c2rust
// 0.22.1 (both semantically NO-OPs, applied to the source that BOTH C and Rust
// build from, so the comparison stays fair):
//   1. XXH_ASSUME → ((void)0)          — c2rust doesn't implement __builtin_assume
//   2. XXH_COMPILER_GUARD → ((void)0)  — c2rust mistranslates its barrier asm
//   plus -DXXH_VECTOR=0 (scalar path — c2rust can't do SIMD intrinsics, and
//   forcing scalar keeps C vs Rust on the same non-SIMD code = fair).
//
// Workload: hash a fixed 64 KiB buffer N times with varying seed (so the hash
// actually recomputes), XORing the digests. Exercises xxHash's scalar hash
// kernels (round/avalanche/mergeAccs). Input file (if any) is ignored — the
// buffer is constructed in-harness. argv[1] selects the variant:
//   xxh64 (default) | xxh3 (XXH3-64) | xxh32 . C and Rust MUST print identical
//   "hashes=N acc=HEX" (all -DXXH_VECTOR=0 scalar, since c2rust can't do SIMD).
#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::c_void;
use xxhash_raw::src::xxhash::{XXH64, XXH32, XXH3_64bits_withSeed};

const BUFSIZE: usize = 65536; // 64 KiB
const N: u64 = 50_000;        // iterations (tuned so wall-time ~1s)

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("xxh64");
    let mut buf = vec![0u8; BUFSIZE];
    for i in 0..BUFSIZE {
        buf[i] = (i.wrapping_mul(31).wrapping_add(7)) as u8;
    }
    let p = buf.as_ptr() as *const c_void;
    let mut acc: u64 = 0;
    match mode {
        "xxh3"  => for n in 0..N { acc ^= unsafe { XXH3_64bits_withSeed(p, BUFSIZE, n) }; },
        "xxh32" => for n in 0..N { acc ^= unsafe { XXH32(p, BUFSIZE, n as u32) } as u64; },
        _       => for n in 0..N { acc ^= unsafe { XXH64(p, BUFSIZE, n) }; },
    }
    println!("hashes={} acc={:016x}", N, acc);
}
