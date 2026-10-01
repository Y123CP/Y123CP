use core::ffi::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    fn gzclose_r(file: gzFile) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn gzclose(mut file: gzFile) -> c_int {
    return gzclose_r(file);
}
