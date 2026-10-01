use core::ffi::*;
use crate::src::zlib::gzread::gzclose_r;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[no_mangle]
pub extern "C" fn gzclose(mut file: gzFile) -> c_int { {
    return gzclose_r(file);
} }
