#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]

extern crate libc;

pub mod src {
    pub mod lz4;
    pub mod lz4file;
    pub mod lz4frame;
    pub mod lz4hc;
    pub mod xxhash;
} // mod src
