// libzahl_opsuite — workload harness for the staged libzahl_cleaned.
//
// Reads pairs of length-prefixed big-int byte strings from a .bin file
// (4-byte little-endian length followed by `len` raw bytes), runs the
// canonical op suite (add/sub/mul_ll) on each pair, accumulates the
// results, and prints `pairs=N final_bytes=M`. Output format matches
// the C reference harness (dataset_source/libzahl/libzahl_harness.c).
//
// Usage:   libzahl_opsuite <input.bin>
// Output:  pairs=<N> final_bytes=<M>\n
//
// Note: c2rust 0.22.1 emits a separate `pub struct zahl` per consuming
// module, so we cannot share a single Rust type across the imported fn
// signatures. We hold the zahl as a local `repr(C)` struct of matching
// layout and cast every call site to *mut c_void → *mut _.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env;
use std::ffi::c_void;
use std::fs::File;
use std::io::Read;
use std::mem::MaybeUninit;
use std::os::raw::c_int;

use libzahl_cleaned::src::zadd::zadd;
use libzahl_cleaned::src::zsub::zsub;
use libzahl_cleaned::src::zmul::zmul_ll;
use libzahl_cleaned::src::zload::zload;
use libzahl_cleaned::src::zfree::zfree;
use libzahl_cleaned::src::zsetup::zsetup;
use libzahl_cleaned::src::zunsetup::zunsetup;
// OPERATION 2 — DIVISION (zdivmod), the one heavy op the add/sub/mul suite omits.
use libzahl_cleaned::src::zdivmod::zdivmod;
// OPERATION 4 — MODULAR EXPONENTIATION (zmodpow), the crypto-relevant heavy kernel.
use libzahl_cleaned::src::zmodpow::zmodpow;

// Layout-identical to libzahl's zahl struct (sign, padding, used, alloced,
// chars). We own the storage; the lib mutates fields through pointer
// casts so layout is what matters, not Rust type identity.
#[repr(C)]
#[derive(Copy, Clone)]
struct ZahlStorage {
    sign: c_int,
    padding__: c_int,
    used: usize,
    alloced: usize,
    chars: *mut u64,
}

extern "C" {
    fn setjmp(env: *mut c_void) -> c_int;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <input.bin>", args[0]);
        std::process::exit(1);
    }

    // argv[1]=="divmod" selects OPERATION 2 (division); otherwise it's an input
    // path for the op-suite (whose contents are ignored — operands are in-harness).
    let divmod_mode = args.get(1).map(|s| s.as_str()) == Some("divmod");
    let modpow_mode = args.get(1).map(|s| s.as_str()) == Some("modpow");
    let mut data = Vec::new();
    if !divmod_mode && !modpow_mode {
        File::open(&args[1])
            .and_then(|mut f| f.read_to_end(&mut data))
            .unwrap_or_else(|e| {
                eprintln!("read {}: {}", &args[1], e);
                std::process::exit(1);
            });
    }

    unsafe {
        // setjmp/longjmp target — libzahl_failure longjmps to here on
        // arithmetic errors (e.g. div-by-zero). 200 ints ≥ sizeof(jmp_buf).
        let mut jbuf: MaybeUninit<[c_int; 200]> = MaybeUninit::zeroed();
        let rc = setjmp(jbuf.as_mut_ptr() as *mut c_void);
        if rc != 0 {
            eprintln!("libzahl arithmetic error (longjmp code {})", rc);
            std::process::exit(2);
        }
        zsetup(jbuf.as_mut_ptr() as *mut _);

        let mut x = MaybeUninit::<ZahlStorage>::zeroed().assume_init();
        let mut y = MaybeUninit::<ZahlStorage>::zeroed().assume_init();
        let mut acc = MaybeUninit::<ZahlStorage>::zeroed().assume_init();
        let mut tmp = MaybeUninit::<ZahlStorage>::zeroed().assume_init();

        // EMPIRICAL STUDY FIX (2026-07-01): the ORIGINAL harness zload'd ZERO
        // buffers → x=y=0, so the op loop did 0+0, 0*0 on EMPTY bignums that
        // never grow — it measured call/pool overhead on empty operands, NOT
        // real multi-precision arithmetic (self-time was dominated by process
        // startup). We now zload two LARGE ~4096-bit operands, crafted
        // deterministically and IDENTICALLY to the C harness, so the loop
        // exercises real big-number add/sub/mul. Input file is still ignored
        // (pair_*.bin fixtures aren't in zload format); operands are generated
        // in-harness for reproducibility. zload buffer format (src/zload.c):
        //   [i64 sign][usize used][used × u64 limbs, little-endian].
        let _ = data;
        fn make_operand(seed: u64, nlimbs: usize) -> Vec<u8> {
            let mut buf = Vec::with_capacity(16 + nlimbs * 8);
            buf.extend_from_slice(&1i64.to_le_bytes());            // sign = +1
            buf.extend_from_slice(&(nlimbs as u64).to_le_bytes()); // used = nlimbs
            let mut s = seed;
            for i in 0..nlimbs {
                s = s.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(i as u64 + 1);
                let limb = if i + 1 == nlimbs { s | (1u64 << 63) } else { s | 1 };
                buf.extend_from_slice(&limb.to_le_bytes());
            }
            buf
        }
        let bx = make_operand(0xB2C3_D4E5_F607_1829, 64);   // ~4096-bit
        let by = make_operand(0x1357_9BDF_2468_ACE0, 64);   // ~4096-bit, distinct
        let zero_buf = [0u8; 16];                            // sign=0,used=0 → 0
        zload(&mut x as *mut _ as *mut _, bx.as_ptr() as *const c_void);
        zload(&mut y as *mut _ as *mut _, by.as_ptr() as *const c_void);
        zload(&mut acc as *mut _ as *mut _, zero_buf.as_ptr() as *const c_void);
        zload(&mut tmp as *mut _ as *mut _, zero_buf.as_ptr() as *const c_void);

        if divmod_mode {
            // OPERATION 2 — DIVISION: dividend = x*y (~8192-bit) ÷ y (~4096-bit),
            // N times → real schoolbook long division (quotient ~4096-bit). Exercises
            // zdivmod, the heavy op the add/sub/mul suite never touched.
            let mut big = MaybeUninit::<ZahlStorage>::zeroed().assume_init();
            let mut q = MaybeUninit::<ZahlStorage>::zeroed().assume_init();
            let mut r = MaybeUninit::<ZahlStorage>::zeroed().assume_init();
            zload(&mut big as *mut _ as *mut _, zero_buf.as_ptr() as *const c_void);
            zload(&mut q as *mut _ as *mut _, zero_buf.as_ptr() as *const c_void);
            zload(&mut r as *mut _ as *mut _, zero_buf.as_ptr() as *const c_void);
            zmul_ll(&mut big as *mut _ as *mut _, &mut x as *mut _ as *mut _, &mut y as *mut _ as *mut _);

            let mut pairs = 0u64;
            for _ in 0..1_500 {
                zdivmod(
                    &mut q as *mut _ as *mut _,
                    &mut r as *mut _ as *mut _,
                    &mut big as *mut _ as *mut _,
                    &mut y as *mut _ as *mut _,
                );
                pairs += 1;
            }
            let mut checksum: u64 = q.used as u64;
            for i in 0..q.used {
                checksum ^= (*q.chars.add(i)).rotate_left((i as u32) & 63);
            }
            println!("pairs={} used={} checksum={:016x}", pairs, q.used, checksum);
            zfree(&mut big as *mut _ as *mut _);
            zfree(&mut q as *mut _ as *mut _);
            zfree(&mut r as *mut _ as *mut _);
        } else if modpow_mode {
            // OPERATION 4 — MODULAR EXPONENTIATION: res = x^e mod y (y ~4096-bit odd,
            // e ~256-bit), N times → the crypto-relevant heavy kernel (square-and-
            // multiply over modular multiplies). Same operands/loop as the C harness.
            let mut e = MaybeUninit::<ZahlStorage>::zeroed().assume_init();
            let mut res = MaybeUninit::<ZahlStorage>::zeroed().assume_init();
            let be = make_operand(0x2468_ACE0_1357_9BDF, 4);   // ~256-bit exponent
            zload(&mut e as *mut _ as *mut _, be.as_ptr() as *const c_void);
            zload(&mut res as *mut _ as *mut _, zero_buf.as_ptr() as *const c_void);
            let mut pairs = 0u64;
            for _ in 0..3 {
                zmodpow(
                    &mut res as *mut _ as *mut _,
                    &mut x as *mut _ as *mut _,
                    &mut e as *mut _ as *mut _,
                    &mut y as *mut _ as *mut _,
                );
                pairs += 1;
            }
            let mut checksum: u64 = res.used as u64;
            for i in 0..res.used {
                checksum ^= (*res.chars.add(i)).rotate_left((i as u32) & 63);
            }
            println!("pairs={} used={} checksum={:016x}", pairs, res.used, checksum);
            zfree(&mut e as *mut _ as *mut _);
            zfree(&mut res as *mut _ as *mut _);
        } else {
            // OPERATION 1 — MULTIPLY: isolate the O(n²) schoolbook multiply kernel
            // (add/sub are O(n), too fast + diluted the signal). Fixed x,y → bounded.
            let mut pairs = 0u64;
            for _ in 0..1_500 {
                zmul_ll(&mut tmp as *mut _ as *mut _, &mut x as *mut _ as *mut _, &mut y as *mut _ as *mut _);
                pairs += 1;
            }
            let mut checksum: u64 = tmp.used as u64;
            for i in 0..tmp.used {
                checksum ^= (*tmp.chars.add(i)).rotate_left((i as u32) & 63);
            }
            println!("pairs={} used={} checksum={:016x}", pairs, tmp.used, checksum);
        }

        zfree(&mut x as *mut _ as *mut _);
        zfree(&mut y as *mut _ as *mut _);
        zfree(&mut acc as *mut _ as *mut _);
        zfree(&mut tmp as *mut _ as *mut _);
        zunsetup();
    }
}
