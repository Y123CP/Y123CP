// binn_roundtrip_raw — empirical-study harness for binn (0_raw / c2rust_raw).
//
// EMPIRICAL STUDY: two operations, dispatched by argv[1] (like bzip2 -z/-d), so
// ONE binary covers binn's WRITE and READ hot paths (raises RQ1 coverage past the
// encode-only 13%). Both are self-contained (input ignored); C and Rust MUST emit
// identical stdout.
//   default / "$INPUT"  → ENCODE: build a binn list of M sequential int32, serialize,
//                         loop N_ENC times, emit the serialized bytes of the last build.
//   "decode"            → DECODE: build+serialize the list ONCE, then loop N_DEC times
//                         iterating it via binn_iter_init + binn_list_next, summing each
//                         vint32; emit "sum=<i64>". Exercises binn's read path
//                         (binn_iter_init / binn_list_next / GetValue / AdvanceDataPos).
// set_* helpers are inline macros (not exported), so we call base binn_list_add with
// BINN_INT32. The union int field is c2rust_unnamed.vint32 (GetValue stores DWORDs there).
#![allow(non_camel_case_types, non_snake_case)]

use core::ffi::{c_int, c_void};
use std::env;
use std::io::Write;

use binn_cleaned::src::binn::{
    binn, binn_free, binn_iter, binn_iter_init, binn_list, binn_list_add, binn_list_next,
    binn_ptr, binn_size,
};

const BINN_INT32: c_int = 0x61; // binn.h
const BINN_LIST: c_int = 0xE0; // binn.h (storage container / list)
const M: usize = 100_000; // list length
const N_ENC: usize = 600; // encode iterations (wall-time ~1s)
const N_DEC: usize = 1000; // decode iterations (wall-time ~1s)

unsafe fn build_list() -> *mut binn {
    let list = binn_list();
    for i in 0..M {
        let mut v: i32 = i as i32;
        binn_list_add(list, BINN_INT32, &mut v as *mut i32 as *mut c_void, 0);
    }
    list
}

unsafe fn encode(emit: bool) {
    let list = build_list();
    let out = binn_ptr(list as *const c_void);
    let sz = binn_size(list as *const c_void);
    if emit {
        let bytes = std::slice::from_raw_parts(out as *const u8, sz as usize);
        std::io::stdout().write_all(bytes).unwrap();
    }
    binn_free(list);
}

unsafe fn decode() {
    // build+serialize once, copy the bytes into an owned buffer (setup, not the hot loop)
    let list = build_list();
    let out = binn_ptr(list as *const c_void);
    let sz = binn_size(list as *const c_void) as usize;
    let buf: Vec<u8> = std::slice::from_raw_parts(out as *const u8, sz).to_vec();
    binn_free(list);

    let mut acc: i64 = 0;
    for _ in 0..N_DEC {
        let mut iter: binn_iter = core::mem::zeroed();
        binn_iter_init(&mut iter, buf.as_ptr() as *const c_void, BINN_LIST);
        let mut value: binn = core::mem::zeroed();
        while binn_list_next(&mut iter, &mut value) != 0 {
            acc += value.c2rust_unnamed.vint32 as i64;
        }
    }
    println!("sum={}", acc);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");
    unsafe {
        if mode == "decode" {
            decode();
        } else {
            for _ in 0..(N_ENC - 1) {
                encode(false);
            }
            encode(true); // final build → emit serialized bytes (oracle)
        }
    }
}
