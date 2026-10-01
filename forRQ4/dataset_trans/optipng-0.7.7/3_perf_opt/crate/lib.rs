#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]

pub mod src {
    pub mod ffi;
    pub mod c_consts;
    pub mod c_structs;
    pub mod c_types;
    pub mod c_extern_types;
    pub mod gifread {
        pub mod gifread;
    } // mod gifread
    pub mod libpng {
        pub mod png;
        pub mod pngerror;
        pub mod pngget;
        pub mod pngmem;
        pub mod pngpread;
        pub mod pngread;
        pub mod pngrio;
        pub mod pngrtran;
        pub mod pngrutil;
        pub mod pngset;
        pub mod pngtrans;
        pub mod pngwio;
        pub mod pngwrite;
        pub mod pngwtran;
        pub mod pngwutil;
    } // mod libpng
    pub mod minitiff {
        pub mod tiffread;
        pub mod tiffutil;
    } // mod minitiff
    pub mod opngreduc {
        pub mod opngreduc;
    } // mod opngreduc
    pub mod optipng {
        pub mod bitset;
        pub mod ioutil;
        pub mod optim;
        pub mod optipng;
        pub mod ratio;
        pub mod wildargs;
    } // mod optipng
    pub mod pngxtern {
        pub mod pngxio;
        pub mod pngxmem;
        pub mod pngxrbmp;
        pub mod pngxread;
        pub mod pngxrgif;
        pub mod pngxrjpg;
        pub mod pngxrpnm;
        pub mod pngxrtif;
        pub mod pngxset;
    } // mod pngxtern
    pub mod pnmio {
        pub mod pnmin;
        pub mod pnmout;
        pub mod pnmutil;
    } // mod pnmio
    pub mod zlib {
        pub mod adler32;
        pub mod compress;
        pub mod crc32;
        pub mod deflate;
        pub mod gzclose;
        pub mod gzlib;
        pub mod gzread;
        pub mod gzwrite;
        pub mod infback;
        pub mod inffast;
        pub mod inflate;
        pub mod inftrees;
        pub mod trees;
        pub mod uncompr;
        pub mod zutil;
    } // mod zlib
} // mod src
