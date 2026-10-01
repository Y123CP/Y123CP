#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]
#![feature(stdsimd)]

#[macro_use]
extern crate c2rust_bitfields;

pub mod src {
    pub mod c_inlined_fns;
    pub mod ffi;
    pub mod c_consts;
    pub mod c_structs;
    pub mod c_types;
    pub mod c_extern_types;
    pub mod common {
        pub mod constants;
        pub mod context;
        pub mod dictionary;
        pub mod platform;
        pub mod shared_dictionary;
        pub mod transform;
    } // mod common
    pub mod dec {
        pub mod bit_reader;
        pub mod decode;
        pub mod huffman;
        pub mod prefix;
        pub mod state;
        pub mod static_init;
    } // mod dec
    pub mod enc {
        pub mod backward_references;
        pub mod backward_references_hq;
        pub mod bit_cost;
        pub mod block_splitter;
        pub mod brotli_bit_stream;
        pub mod cluster;
        pub mod command;
        pub mod compound_dictionary;
        pub mod compress_fragment;
        pub mod compress_fragment_two_pass;
        pub mod dictionary_hash;
        pub mod encode;
        pub mod encoder_dict;
        pub mod entropy_encode;
        pub mod fast_log;
        pub mod histogram;
        pub mod literal_cost;
        pub mod memory;
        pub mod metablock;
        pub mod static_dict;
        pub mod static_dict_lut;
        pub mod static_init;
        pub mod utf8_util;
    } // mod enc
    // c2rust 0.22.1 bug fix: removed `pub mod tools { pub mod brotli; }`
    // because `src/tools/brotli.rs` is also the Cargo `[[bin]]` entry.
    // Same-file dual-compile (as lib module + as bin) prevents the bin
    // from importing the lib via `use ::brotli_raw::*` (cargo refuses
    // the auto bin→lib dep when the bin file is reachable as a lib
    // module). Stage1 cleanup needs that import to be valid so dedup
    // can splice `use ::<crate>::src::c_types::*;` into the bin.
} // mod src
