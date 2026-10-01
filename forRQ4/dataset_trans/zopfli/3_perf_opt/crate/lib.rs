#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

pub mod src {
    pub mod c_inlined_fns;
    pub mod ffi;
    pub mod c_consts;
    pub mod c_structs;
    pub mod c_types;
    pub mod c_extern_types;
    pub mod blocksplitter;
    pub mod cache;
    pub mod deflate;
    pub mod gzip_container;
    pub mod hash;
    pub mod katajainen;
    pub mod lz77;
    pub mod squeeze;
    pub mod tree;
    pub mod util;
    pub mod zlib_container;
    pub mod zopfli_lib;
} // mod src
