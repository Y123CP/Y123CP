extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn vsyslog(
        __pri: ::core::ffi::c_int,
        __fmt: *const ::core::ffi::c_char,
        __ap: ::core::ffi::VaList,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const ::core::ffi::c_char,
    mut __arg: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_INFO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LOG_DEBUG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
static mut _syslog: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut _debug: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn mc_set_debug(mut debug: ::core::ffi::c_int) {
    _debug = debug;
}
#[no_mangle]
pub unsafe extern "C" fn mc_get_debug() -> ::core::ffi::c_int {
    return _debug;
}
#[no_mangle]
pub unsafe extern "C" fn mc_set_syslog(mut syslog: ::core::ffi::c_int) {
    _syslog = syslog;
}
#[no_mangle]
pub unsafe extern "C" fn mc_debug(mut msg: *const ::core::ffi::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    if _debug != 0 {
        ap = args.clone();
        if _syslog != 0 {
            vsyslog(LOG_DEBUG, msg, ap.as_va_list());
        } else {
            vprintf(msg, ap.as_va_list());
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mc_error(mut msg: *const ::core::ffi::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    if _syslog != 0 {
        vsyslog(LOG_ERR, msg, ap.as_va_list());
    } else {
        vfprintf(stderr, msg, ap.as_va_list());
    };
}
#[no_mangle]
pub unsafe extern "C" fn mc_info(mut msg: *const ::core::ffi::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    if _syslog != 0 {
        vsyslog(LOG_INFO, msg, ap.as_va_list());
    } else {
        vfprintf(stderr, msg, ap.as_va_list());
    };
}
