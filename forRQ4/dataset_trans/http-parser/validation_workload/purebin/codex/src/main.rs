                                                           
                                                               
                                      
mod libcall {
    use core::ffi::{c_char, c_int};
    use httplib::src::http_parser as hp;

    pub type Parser = hp::http_parser;
    pub type Settings = hp::http_parser_settings;
    pub type ParsedUrl = hp::http_parser_url;
    pub use hp::{HPE_OK, HPE_PAUSED, HPE_CLOSED_CONNECTION};

    #[inline]
    pub unsafe fn init(p: *mut Parser, t: u32) {
        hp::http_parser_init(p, t)
    }
    #[inline]
    pub unsafe fn settings_init(s: *mut Settings) {
        hp::http_parser_settings_init(s)
    }
    #[inline]
    pub unsafe fn execute(p: *mut Parser, s: *const Settings, data: *const c_char, len: usize) -> usize {
        hp::http_parser_execute(p, s, data, len as hp::size_t) as usize
    }
    #[inline]
    pub unsafe fn should_keep_alive(p: *const Parser) -> i32 {
        hp::http_should_keep_alive(p) as i32
    }
    #[inline]
    pub unsafe fn body_is_final(p: *const Parser) -> i32 {
        hp::http_body_is_final(p) as i32
    }
    #[inline]
    pub unsafe fn pause(p: *mut Parser, paused: i32) {
        hp::http_parser_pause(p, paused as c_int)
    }
    #[inline]
    pub unsafe fn url_init(u: *mut ParsedUrl) {
        hp::http_parser_url_init(u)
    }
    #[inline]
    pub unsafe fn parse_url(buf: *const c_char, len: usize, is_connect: i32, u: *mut ParsedUrl) -> i32 {
        hp::http_parser_parse_url(buf, len as hp::size_t, is_connect as c_int, u) as i32
    }
}

include!("../../driver.rs");
