use core::ffi::*;
pub use crate::src::c_structs::*;

#[inline]
pub unsafe fn ZopfliInitOptions(mut options: *mut ZopfliOptions) {
    let options_view: &mut ZopfliOptions = unsafe { &mut *options };
    options_view.verbose = 0 as c_int;
    options_view.verbose_more = 0 as c_int;
    options_view.numiterations = 15 as c_int;
    options_view.blocksplitting = 1 as c_int;
    options_view.blocksplittinglast = 0 as c_int;
    options_view.blocksplittingmax = 15 as c_int;
}
