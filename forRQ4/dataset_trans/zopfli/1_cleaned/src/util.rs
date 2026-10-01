use core::ffi::*;
pub use crate::src::c_structs::*;

#[no_mangle]
pub unsafe extern "C" fn ZopfliInitOptions(mut options: *mut ZopfliOptions) {
    (*options).verbose = 0 as c_int;
    (*options).verbose_more = 0 as c_int;
    (*options).numiterations = 15 as c_int;
    (*options).blocksplitting = 1 as c_int;
    (*options).blocksplittinglast = 0 as c_int;
    (*options).blocksplittingmax = 15 as c_int;
}
