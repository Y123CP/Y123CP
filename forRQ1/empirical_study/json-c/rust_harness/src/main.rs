// json_roundtrip — workload harness for the staged json_c_cleaned library.
//
// Reads a .json file, parses it via json_tokener_parse(), reports the
// top-level container's element count and the canonical re-serialized
// string length. Output format matches the C reference harness
// (dataset_source/json-c/exhaustive_driver.c canonical-print path).
//
// Usage:   json_roundtrip <input.json>
// Output:  length=<N> keys=<M>\n
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;

use json_c_cleaned::src::json_tokener::json_tokener_parse;
use json_c_cleaned::src::json_object::{
    json_object_object_length, json_object_to_json_string, json_object_put,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <input.json> [parse|serialize]", args[0]);
        std::process::exit(1);
    }

    let mut buf = Vec::new();
    File::open(&args[1])
        .and_then(|mut f| f.read_to_end(&mut buf))
        .unwrap_or_else(|e| {
            eprintln!("read {}: {}", &args[1], e);
            std::process::exit(1);
        });
    // json_tokener_parse needs a NUL-terminated C string.
    let cstr = CString::new(buf).unwrap_or_else(|_| {
        eprintln!("input contains interior NUL byte");
        std::process::exit(1);
    });

    // OPERATION split: parse (text→tree) vs serialize (tree→text), each looped
    // JC_N times (~1s). argv[2] selects the mode; default = roundtrip. Bridge the
    // per-module distinct `json_object` types via *mut c_void.
    const JC_N: usize = 20;
    const JC_N_SER: usize = 80;   // serialize ~3x faster/iter than parse → scale to clear the 100ms floor even on twitter (digest = length, loop-independent)
    let mode = args.get(2).map(|s| s.as_str()).unwrap_or("roundtrip");
    use std::os::raw::c_void;
    unsafe {
        match mode {
            "parse" => {
                let mut keys: i64 = 0;
                for _ in 0..JC_N {
                    let root = json_tokener_parse(cstr.as_ptr());
                    if root.is_null() {
                        eprintln!("parse NULL");
                        std::process::exit(2);
                    }
                    let opaque = root as *mut c_void;
                    keys = json_object_object_length(opaque as *const _) as i64;
                    json_object_put(opaque as *mut _);
                }
                println!("keys={}", keys);
            }
            "serialize" => {
                let root = json_tokener_parse(cstr.as_ptr());
                if root.is_null() {
                    eprintln!("parse NULL");
                    std::process::exit(2);
                }
                let opaque = root as *mut c_void;
                let mut s_len = 0;
                for _ in 0..JC_N_SER {
                    let s_ptr = json_object_to_json_string(opaque as *mut _);
                    s_len = if s_ptr.is_null() { 0 } else { CStr::from_ptr(s_ptr).to_bytes().len() };
                }
                println!("length={}", s_len);
                json_object_put(opaque as *mut _);
            }
            _ => {
                let root = json_tokener_parse(cstr.as_ptr());
                if root.is_null() {
                    eprintln!("json_tokener_parse returned NULL");
                    std::process::exit(2);
                }
                let opaque = root as *mut c_void;
                let keys = json_object_object_length(opaque as *const _) as i64;
                let s_ptr = json_object_to_json_string(opaque as *mut _);
                let s_len = if s_ptr.is_null() { 0 } else { CStr::from_ptr(s_ptr).to_bytes().len() };
                println!("length={} keys={}", s_len, keys);
                json_object_put(opaque as *mut _);
            }
        }
    }
}
