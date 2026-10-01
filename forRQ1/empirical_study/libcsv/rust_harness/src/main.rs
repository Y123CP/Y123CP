// csv_count — workload harness for the staged libcsv_safe library.
//
// Mirrors gap_analyze/harness/libcsv/c/main.c (the C reference harness)
// but calls into the staged Rust libcsv_safe via its #[no_mangle]
// extern "C" symbols.
//
// Usage:   csv_count <file.csv>
// Output:  rows=<N> fields=<M>\n
//
// The output format MUST match the C harness so W1 stdout_hash applies
// equally to (C csv_count, c2rust raw csv_count, our csv_count). All
// three must produce identical bytes for functional equivalence.
//
// Required nightly features come from the libcsv_safe dependency's
// own lib.rs — this crate doesn't enable them directly.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env;
use std::ffi::c_void;
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicU64, Ordering};

use core::ffi::{c_int, c_uchar};
// 0_raw puts all types in src::libcsv (no c_structs/c_types split — that's a
// Stage-1 cleanup artifact). Empirical study links raw 0_raw, so import from libcsv.
use libcsv_safe::src::libcsv::csv_parser;
use libcsv_safe::src::libcsv::size_t;

                                                                              
                                                           
                                                                   
use libcsv_safe::src::libcsv::csv_parse;
use libcsv_safe::src::libcsv::csv_init;
use libcsv_safe::src::libcsv::csv_fini;
use libcsv_safe::src::libcsv::csv_free;
// OPERATION 2 — WRITE: exercises libcsv's field-escaping/quoting path (csv_write),
// which the parse-only workload never touched (parse cov was 41% line / 22% fn).
// Feature-gated so the default (parse-only) binary keeps original single-op codegen.
#[cfg(feature = "op_write")]
use libcsv_safe::src::libcsv::csv_write;
// OPERATION 3 — COV: exercise the remaining public API (opts/delim/quote get+set,
// blk_size, buffer_size, space/term/realloc/free func setters, error, strerror) so
// libcsv fn coverage reaches ≥85%. Deterministic; C and Rust MUST match stdout.
use libcsv_safe::src::libcsv::{
    csv_error, csv_get_buffer_size, csv_get_delim, csv_get_opts, csv_get_quote,
    csv_set_blk_size, csv_set_delim, csv_set_free_func, csv_set_opts, csv_set_quote,
    csv_set_realloc_func, csv_set_space_func, csv_set_term_func, csv_strerror,
};

const CSV_STRICT: c_int = 1;
const CSV_APPEND_NULL: c_int = 8;

extern "C" {
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

unsafe extern "C" fn cov_space(c: c_uchar) -> c_int {
    (c == b' ' || c == b'\t') as c_int
}
unsafe extern "C" fn cov_term(c: c_uchar) -> c_int {
    (c == b'\n') as c_int
}

unsafe fn cov_workload() {
    let mut storage = std::mem::MaybeUninit::<csv_parser>::zeroed();
    let p = storage.as_mut_ptr();
    csv_init(p, 0);
    csv_set_opts(p, (CSV_STRICT | CSV_APPEND_NULL) as c_uchar);
    let opts = csv_get_opts(p);
    csv_set_delim(p, b';');
    let d = csv_get_delim(p);
    csv_set_quote(p, b'\'');
    let q = csv_get_quote(p);
    csv_set_blk_size(p, 2048);
    let bufsz = csv_get_buffer_size(p);
    csv_set_space_func(p, Some(cov_space));
    csv_set_term_func(p, Some(cov_term));
    csv_set_realloc_func(p, Some(realloc));
    csv_set_free_func(p, Some(free));
    let data = b"a;b;'c;d'\nx;y;z\n";
    csv_parse(p, data.as_ptr() as *const c_void, data.len() as size_t, None, None, std::ptr::null_mut());
    let err = csv_error(p);
    let es = std::ffi::CStr::from_ptr(csv_strerror(err)).to_str().unwrap();
    csv_fini(p, None, None, std::ptr::null_mut());
    csv_free(p);
    println!("opts={} delim={} quote={} bufsz={} err={} es={}", opts, d, q, bufsz, err, es);
}


// csv_parse is single-threaded — Relaxed atomics are fine and avoid
// the cost of Mutex / unsafe-static-mut.
static FIELDS: AtomicU64 = AtomicU64::new(0);
static ROWS:   AtomicU64 = AtomicU64::new(0);

unsafe extern "C" fn cb_field(_s: *mut c_void, _len: size_t, _data: *mut c_void) {
    FIELDS.fetch_add(1, Ordering::Relaxed);
}

unsafe extern "C" fn cb_row(_c: c_int, _data: *mut c_void) {
    ROWS.fetch_add(1, Ordering::Relaxed);
}

// WRITE workload: build K fixed fields ONCE (each forced to contain a comma and a
// quote so csv_write must quote + double-escape), then loop N_W times escaping every
// field via csv_write, folding each result's (len, first, last) byte. Self-contained
// (argv[1]=="write"), so C and Rust MUST print identical "written=<total> checksum=HEX".
#[cfg(feature = "op_write")]
const CSV_K: usize = 10_000;   // distinct fields
#[cfg(feature = "op_write")]
const CSV_NW: usize = 300;     // iterations over the field set (wall-time ~1s)
#[cfg(feature = "op_write")]
const CSV_FLEN: usize = 40;    // bytes per field
#[cfg(feature = "op_write")]
const CSV_DEST: usize = 256;   // per-field output buffer

#[cfg(feature = "op_write")]
unsafe fn write_workload() {
    let mut fields = vec![0u8; CSV_K * CSV_FLEN];
    for i in 0..CSV_K {
        for j in 0..CSV_FLEN {
            fields[i * CSV_FLEN + j] =
                i.wrapping_mul(31).wrapping_add(j.wrapping_mul(7)) as u8;
        }
        fields[i * CSV_FLEN + 5] = b',';
        fields[i * CSV_FLEN + 10] = b'"';
    }
    let mut dest = [0u8; CSV_DEST];
    let mut checksum: u64 = 0;
    let mut total: u64 = 0;
    for _ in 0..CSV_NW {
        for i in 0..CSV_K {
            let src = fields.as_ptr().add(i * CSV_FLEN) as *const c_void;
            let sz = csv_write(
                dest.as_mut_ptr() as *mut c_void,
                CSV_DEST as size_t,
                src,
                CSV_FLEN as size_t,
            ) as usize;
            let last = if sz > 0 { dest[sz - 1] } else { 0 };
            checksum = checksum.rotate_left(1) ^ (sz as u64) ^ (dest[0] as u64) ^ (last as u64);
            total += sz as u64;
        }
    }
    println!("written={} checksum={:016x}", total, checksum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    #[cfg(feature = "op_write")]
    if args.get(1).map(|s| s.as_str()) == Some("write") {
        unsafe { write_workload() };
        return;
    }
    if args.get(1).map(|s| s.as_str()) == Some("cov") {
        unsafe { cov_workload() };
        return;
    }
    if args.len() != 2 {
        eprintln!("usage: {} <file.csv | write>", args[0]);
        std::process::exit(1);
    }

    let mut buf = Vec::new();
    File::open(&args[1])
        .and_then(|mut f| f.read_to_end(&mut buf))
        .unwrap_or_else(|e| {
            eprintln!("read {}: {}", &args[1], e);
            std::process::exit(1);
        });

    unsafe {
        // csv_parser is a caller-allocated, opaque handle — exactly the
        // C contract `struct csv_parser p; csv_init(&p);`: the caller
        // provides storage, csv_init does ALL field initialization.
        // The harness allocates zeroed storage and only ever holds a
        // *mut csv_parser — it never constructs or inspects a
        // csv_parser value itself. So it stays agnostic to how the
        // library is lifted (raw POD struct, or one owning a Vec):
        // verification hooks onto the stable boundary API, nothing else.
        let mut storage = std::mem::MaybeUninit::<csv_parser>::zeroed();
        let p = storage.as_mut_ptr();
        if csv_init(p, 0) != 0 {
            eprintln!("csv_init failed");
            std::process::exit(1);
        }
        // FN-PTR INTERFACE — c2rust-direct (0_raw / 1_cleaned) typed the
        // callback params as `Option<unsafe extern "C" fn(...)>`. We pass
        // `Some(<fn_name>)` directly; the closure form `move |...| ...`
        // (used by sibling csv_count/ harness for 2_stage_a) does not
        // type-check against the Option<fn-ptr> signature.
        csv_parse(
            p,
            buf.as_ptr() as *const c_void,
            buf.len() as size_t,
            Some(cb_field),
            Some(cb_row),
            std::ptr::null_mut(),
        );
        csv_fini(p, Some(cb_field), Some(cb_row), std::ptr::null_mut());
        csv_free(p);
    }

    println!(
        "rows={} fields={}",
        ROWS.load(Ordering::Relaxed),
        FIELDS.load(Ordering::Relaxed)
    );
}
