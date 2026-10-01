#![feature(extern_types)]
                                                        
                                                     
                                                        
                                                             
                                                             
                                                               
mod libcall {
    use core::ffi::{c_char, c_int};
    use fzylib::src::{choices, options, r#match};

    pub type Options = choices::options_t;
    pub type Choices = choices::choices_t;
    type SizeT = fzylib::src::choices::size_t;

    #[inline]
    pub unsafe fn has_match(needle: *const c_char, haystack: *const c_char) -> i32 {
        r#match::has_match(needle, haystack) as i32
    }
    #[inline]
    pub unsafe fn match_positions(needle: *const c_char, haystack: *const c_char, positions: *mut usize) -> f64 {
        r#match::match_positions(needle, haystack, positions as *mut r#match::size_t)
    }

    #[inline]
    pub unsafe fn options_init(o: *mut Options) {
        options::options_init(o as *mut options::options_t)
    }

    #[inline]
    pub unsafe fn choices_init(c: *mut Choices, o: *mut Options) {
        choices::choices_init(c, o)
    }
    #[inline]
    pub unsafe fn choices_add(c: *mut Choices, s: *const c_char) {
        choices::choices_add(c, s)
    }
    #[inline]
    pub unsafe fn choices_search(c: *mut Choices, s: *const c_char) {
        choices::choices_search(c, s)
    }
    #[inline]
    pub unsafe fn choices_available(c: *mut Choices) -> usize {
        choices::choices_available(c) as usize
    }
    #[inline]
    pub unsafe fn choices_get(c: *mut Choices, n: usize) -> *const c_char {
        choices::choices_get(c, n as SizeT)
    }
    #[inline]
    pub unsafe fn choices_getscore(c: *mut Choices, n: usize) -> f64 {
        choices::choices_getscore(c, n as SizeT)
    }
    #[inline]
    pub unsafe fn choices_next(c: *mut Choices) {
        choices::choices_next(c)
    }
    #[inline]
    pub unsafe fn choices_prev(c: *mut Choices) {
        choices::choices_prev(c)
    }
    #[inline]
    pub unsafe fn choices_destroy(c: *mut Choices) {
        choices::choices_destroy(c)
    }
    #[inline]
    pub unsafe fn choices_fread(c: *mut Choices, f: *mut choices::FILE, delim: i8) {
        choices::choices_fread(c, f, delim as c_char)
    }

                                             
    extern "C" {
        pub fn fopen(path: *const c_char, mode: *const c_char) -> *mut choices::FILE;
        pub fn fclose(f: *mut choices::FILE) -> c_int;
    }
}

include!("../../driver.rs");
