#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(raw_ref_op)]

pub mod src {
    pub mod ffi;
    pub mod c_consts;
    pub mod c_structs;
    pub mod c_types;
    pub mod miniz;
    pub mod miniz_tdef;
    pub mod miniz_tinfl;
    pub mod miniz_zip;
} // mod src
