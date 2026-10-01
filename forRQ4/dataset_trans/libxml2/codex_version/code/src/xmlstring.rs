extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlMallocAtomic: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
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
pub type size_t = usize;
pub type va_list = __builtin_va_list;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type xmlChar = ::core::ffi::c_uchar;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
#[no_mangle]
pub unsafe extern "C" fn xmlStrndup(
    mut cur: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if cur.is_null() || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlMallocAtomic.expect("non-null function pointer")(
        (len as size_t).wrapping_add(1 as size_t),
    ) as *mut xmlChar;
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    memcpy(
        ret as *mut ::core::ffi::c_void,
        cur as *const ::core::ffi::c_void,
        len as size_t,
    );
    *ret.offset(len as isize) = 0 as xmlChar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrdup(mut cur: *const xmlChar) -> *mut xmlChar {
    let mut len: size_t = 0;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    len = strlen(cur as *const ::core::ffi::c_char);
    if len > INT_MAX as size_t {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStrndup(cur, len as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCharStrndup(
    mut cur: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if cur.is_null() || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlMallocAtomic.expect("non-null function pointer")(
        (len as size_t).wrapping_add(1 as size_t),
    ) as *mut xmlChar;
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < len {
        *ret.offset(i as isize) = *cur.offset(i as isize) as xmlChar;
        if *ret.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return ret;
        }
        i += 1;
    }
    *ret.offset(len as isize) = 0 as xmlChar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCharStrdup(mut cur: *const ::core::ffi::c_char) -> *mut xmlChar {
    let mut len: size_t = 0;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    len = strlen(cur);
    if len > INT_MAX as size_t {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlCharStrndup(cur, len as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrcmp(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) -> ::core::ffi::c_int {
    if str1 == str2 {
        return 0 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        let fresh4 = str1;
        str1 = str1.offset(1);
        let mut tmp: ::core::ffi::c_int =
            *fresh4 as ::core::ffi::c_int - *str2 as ::core::ffi::c_int;
        if tmp != 0 as ::core::ffi::c_int {
            return tmp;
        }
        let fresh5 = str2;
        str2 = str2.offset(1);
        if !(*fresh5 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrEqual(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) -> ::core::ffi::c_int {
    if str1 == str2 {
        return 1 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if str2.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let fresh8 = str1;
        str1 = str1.offset(1);
        if *fresh8 as ::core::ffi::c_int != *str2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        let fresh9 = str2;
        str2 = str2.offset(1);
        if !(*fresh9 != 0) {
            break;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrQEqual(
    mut pref: *const xmlChar,
    mut name: *const xmlChar,
    mut str: *const xmlChar,
) -> ::core::ffi::c_int {
    if pref.is_null() {
        return xmlStrEqual(name, str);
    }
    if name.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if str.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let fresh10 = pref;
        pref = pref.offset(1);
        if *fresh10 as ::core::ffi::c_int != *str as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        let fresh11 = str;
        str = str.offset(1);
        if !(*fresh11 as ::core::ffi::c_int != 0 && *pref as ::core::ffi::c_int != 0) {
            break;
        }
    }
    let fresh12 = str;
    str = str.offset(1);
    if *fresh12 as ::core::ffi::c_int != ':' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let fresh13 = name;
        name = name.offset(1);
        if *fresh13 as ::core::ffi::c_int != *str as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        let fresh14 = str;
        str = str.offset(1);
        if !(*fresh14 != 0) {
            break;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrncmp(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if len <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if str1 == str2 {
        return 0 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        let fresh0 = str1;
        str1 = str1.offset(1);
        let mut tmp: ::core::ffi::c_int =
            *fresh0 as ::core::ffi::c_int - *str2 as ::core::ffi::c_int;
        if tmp != 0 as ::core::ffi::c_int || {
            len -= 1;
            len == 0 as ::core::ffi::c_int
        } {
            return tmp;
        }
        let fresh1 = str2;
        str2 = str2.offset(1);
        if !(*fresh1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
static mut casemap: [xmlChar; 256] = [
    0 as ::core::ffi::c_int as xmlChar,
    0x1 as ::core::ffi::c_int as xmlChar,
    0x2 as ::core::ffi::c_int as xmlChar,
    0x3 as ::core::ffi::c_int as xmlChar,
    0x4 as ::core::ffi::c_int as xmlChar,
    0x5 as ::core::ffi::c_int as xmlChar,
    0x6 as ::core::ffi::c_int as xmlChar,
    0x7 as ::core::ffi::c_int as xmlChar,
    0x8 as ::core::ffi::c_int as xmlChar,
    0x9 as ::core::ffi::c_int as xmlChar,
    0xa as ::core::ffi::c_int as xmlChar,
    0xb as ::core::ffi::c_int as xmlChar,
    0xc as ::core::ffi::c_int as xmlChar,
    0xd as ::core::ffi::c_int as xmlChar,
    0xe as ::core::ffi::c_int as xmlChar,
    0xf as ::core::ffi::c_int as xmlChar,
    0x10 as ::core::ffi::c_int as xmlChar,
    0x11 as ::core::ffi::c_int as xmlChar,
    0x12 as ::core::ffi::c_int as xmlChar,
    0x13 as ::core::ffi::c_int as xmlChar,
    0x14 as ::core::ffi::c_int as xmlChar,
    0x15 as ::core::ffi::c_int as xmlChar,
    0x16 as ::core::ffi::c_int as xmlChar,
    0x17 as ::core::ffi::c_int as xmlChar,
    0x18 as ::core::ffi::c_int as xmlChar,
    0x19 as ::core::ffi::c_int as xmlChar,
    0x1a as ::core::ffi::c_int as xmlChar,
    0x1b as ::core::ffi::c_int as xmlChar,
    0x1c as ::core::ffi::c_int as xmlChar,
    0x1d as ::core::ffi::c_int as xmlChar,
    0x1e as ::core::ffi::c_int as xmlChar,
    0x1f as ::core::ffi::c_int as xmlChar,
    0x20 as ::core::ffi::c_int as xmlChar,
    0x21 as ::core::ffi::c_int as xmlChar,
    0x22 as ::core::ffi::c_int as xmlChar,
    0x23 as ::core::ffi::c_int as xmlChar,
    0x24 as ::core::ffi::c_int as xmlChar,
    0x25 as ::core::ffi::c_int as xmlChar,
    0x26 as ::core::ffi::c_int as xmlChar,
    0x27 as ::core::ffi::c_int as xmlChar,
    0x28 as ::core::ffi::c_int as xmlChar,
    0x29 as ::core::ffi::c_int as xmlChar,
    0x2a as ::core::ffi::c_int as xmlChar,
    0x2b as ::core::ffi::c_int as xmlChar,
    0x2c as ::core::ffi::c_int as xmlChar,
    0x2d as ::core::ffi::c_int as xmlChar,
    0x2e as ::core::ffi::c_int as xmlChar,
    0x2f as ::core::ffi::c_int as xmlChar,
    0x30 as ::core::ffi::c_int as xmlChar,
    0x31 as ::core::ffi::c_int as xmlChar,
    0x32 as ::core::ffi::c_int as xmlChar,
    0x33 as ::core::ffi::c_int as xmlChar,
    0x34 as ::core::ffi::c_int as xmlChar,
    0x35 as ::core::ffi::c_int as xmlChar,
    0x36 as ::core::ffi::c_int as xmlChar,
    0x37 as ::core::ffi::c_int as xmlChar,
    0x38 as ::core::ffi::c_int as xmlChar,
    0x39 as ::core::ffi::c_int as xmlChar,
    0x3a as ::core::ffi::c_int as xmlChar,
    0x3b as ::core::ffi::c_int as xmlChar,
    0x3c as ::core::ffi::c_int as xmlChar,
    0x3d as ::core::ffi::c_int as xmlChar,
    0x3e as ::core::ffi::c_int as xmlChar,
    0x3f as ::core::ffi::c_int as xmlChar,
    0x40 as ::core::ffi::c_int as xmlChar,
    0x61 as ::core::ffi::c_int as xmlChar,
    0x62 as ::core::ffi::c_int as xmlChar,
    0x63 as ::core::ffi::c_int as xmlChar,
    0x64 as ::core::ffi::c_int as xmlChar,
    0x65 as ::core::ffi::c_int as xmlChar,
    0x66 as ::core::ffi::c_int as xmlChar,
    0x67 as ::core::ffi::c_int as xmlChar,
    0x68 as ::core::ffi::c_int as xmlChar,
    0x69 as ::core::ffi::c_int as xmlChar,
    0x6a as ::core::ffi::c_int as xmlChar,
    0x6b as ::core::ffi::c_int as xmlChar,
    0x6c as ::core::ffi::c_int as xmlChar,
    0x6d as ::core::ffi::c_int as xmlChar,
    0x6e as ::core::ffi::c_int as xmlChar,
    0x6f as ::core::ffi::c_int as xmlChar,
    0x70 as ::core::ffi::c_int as xmlChar,
    0x71 as ::core::ffi::c_int as xmlChar,
    0x72 as ::core::ffi::c_int as xmlChar,
    0x73 as ::core::ffi::c_int as xmlChar,
    0x74 as ::core::ffi::c_int as xmlChar,
    0x75 as ::core::ffi::c_int as xmlChar,
    0x76 as ::core::ffi::c_int as xmlChar,
    0x77 as ::core::ffi::c_int as xmlChar,
    0x78 as ::core::ffi::c_int as xmlChar,
    0x79 as ::core::ffi::c_int as xmlChar,
    0x7a as ::core::ffi::c_int as xmlChar,
    0x7b as ::core::ffi::c_int as xmlChar,
    0x5c as ::core::ffi::c_int as xmlChar,
    0x5d as ::core::ffi::c_int as xmlChar,
    0x5e as ::core::ffi::c_int as xmlChar,
    0x5f as ::core::ffi::c_int as xmlChar,
    0x60 as ::core::ffi::c_int as xmlChar,
    0x61 as ::core::ffi::c_int as xmlChar,
    0x62 as ::core::ffi::c_int as xmlChar,
    0x63 as ::core::ffi::c_int as xmlChar,
    0x64 as ::core::ffi::c_int as xmlChar,
    0x65 as ::core::ffi::c_int as xmlChar,
    0x66 as ::core::ffi::c_int as xmlChar,
    0x67 as ::core::ffi::c_int as xmlChar,
    0x68 as ::core::ffi::c_int as xmlChar,
    0x69 as ::core::ffi::c_int as xmlChar,
    0x6a as ::core::ffi::c_int as xmlChar,
    0x6b as ::core::ffi::c_int as xmlChar,
    0x6c as ::core::ffi::c_int as xmlChar,
    0x6d as ::core::ffi::c_int as xmlChar,
    0x6e as ::core::ffi::c_int as xmlChar,
    0x6f as ::core::ffi::c_int as xmlChar,
    0x70 as ::core::ffi::c_int as xmlChar,
    0x71 as ::core::ffi::c_int as xmlChar,
    0x72 as ::core::ffi::c_int as xmlChar,
    0x73 as ::core::ffi::c_int as xmlChar,
    0x74 as ::core::ffi::c_int as xmlChar,
    0x75 as ::core::ffi::c_int as xmlChar,
    0x76 as ::core::ffi::c_int as xmlChar,
    0x77 as ::core::ffi::c_int as xmlChar,
    0x78 as ::core::ffi::c_int as xmlChar,
    0x79 as ::core::ffi::c_int as xmlChar,
    0x7a as ::core::ffi::c_int as xmlChar,
    0x7b as ::core::ffi::c_int as xmlChar,
    0x7c as ::core::ffi::c_int as xmlChar,
    0x7d as ::core::ffi::c_int as xmlChar,
    0x7e as ::core::ffi::c_int as xmlChar,
    0x7f as ::core::ffi::c_int as xmlChar,
    0x80 as ::core::ffi::c_int as xmlChar,
    0x81 as ::core::ffi::c_int as xmlChar,
    0x82 as ::core::ffi::c_int as xmlChar,
    0x83 as ::core::ffi::c_int as xmlChar,
    0x84 as ::core::ffi::c_int as xmlChar,
    0x85 as ::core::ffi::c_int as xmlChar,
    0x86 as ::core::ffi::c_int as xmlChar,
    0x87 as ::core::ffi::c_int as xmlChar,
    0x88 as ::core::ffi::c_int as xmlChar,
    0x89 as ::core::ffi::c_int as xmlChar,
    0x8a as ::core::ffi::c_int as xmlChar,
    0x8b as ::core::ffi::c_int as xmlChar,
    0x8c as ::core::ffi::c_int as xmlChar,
    0x8d as ::core::ffi::c_int as xmlChar,
    0x8e as ::core::ffi::c_int as xmlChar,
    0x8f as ::core::ffi::c_int as xmlChar,
    0x90 as ::core::ffi::c_int as xmlChar,
    0x91 as ::core::ffi::c_int as xmlChar,
    0x92 as ::core::ffi::c_int as xmlChar,
    0x93 as ::core::ffi::c_int as xmlChar,
    0x94 as ::core::ffi::c_int as xmlChar,
    0x95 as ::core::ffi::c_int as xmlChar,
    0x96 as ::core::ffi::c_int as xmlChar,
    0x97 as ::core::ffi::c_int as xmlChar,
    0x98 as ::core::ffi::c_int as xmlChar,
    0x99 as ::core::ffi::c_int as xmlChar,
    0x9a as ::core::ffi::c_int as xmlChar,
    0x9b as ::core::ffi::c_int as xmlChar,
    0x9c as ::core::ffi::c_int as xmlChar,
    0x9d as ::core::ffi::c_int as xmlChar,
    0x9e as ::core::ffi::c_int as xmlChar,
    0x9f as ::core::ffi::c_int as xmlChar,
    0xa0 as ::core::ffi::c_int as xmlChar,
    0xa1 as ::core::ffi::c_int as xmlChar,
    0xa2 as ::core::ffi::c_int as xmlChar,
    0xa3 as ::core::ffi::c_int as xmlChar,
    0xa4 as ::core::ffi::c_int as xmlChar,
    0xa5 as ::core::ffi::c_int as xmlChar,
    0xa6 as ::core::ffi::c_int as xmlChar,
    0xa7 as ::core::ffi::c_int as xmlChar,
    0xa8 as ::core::ffi::c_int as xmlChar,
    0xa9 as ::core::ffi::c_int as xmlChar,
    0xaa as ::core::ffi::c_int as xmlChar,
    0xab as ::core::ffi::c_int as xmlChar,
    0xac as ::core::ffi::c_int as xmlChar,
    0xad as ::core::ffi::c_int as xmlChar,
    0xae as ::core::ffi::c_int as xmlChar,
    0xaf as ::core::ffi::c_int as xmlChar,
    0xb0 as ::core::ffi::c_int as xmlChar,
    0xb1 as ::core::ffi::c_int as xmlChar,
    0xb2 as ::core::ffi::c_int as xmlChar,
    0xb3 as ::core::ffi::c_int as xmlChar,
    0xb4 as ::core::ffi::c_int as xmlChar,
    0xb5 as ::core::ffi::c_int as xmlChar,
    0xb6 as ::core::ffi::c_int as xmlChar,
    0xb7 as ::core::ffi::c_int as xmlChar,
    0xb8 as ::core::ffi::c_int as xmlChar,
    0xb9 as ::core::ffi::c_int as xmlChar,
    0xba as ::core::ffi::c_int as xmlChar,
    0xbb as ::core::ffi::c_int as xmlChar,
    0xbc as ::core::ffi::c_int as xmlChar,
    0xbd as ::core::ffi::c_int as xmlChar,
    0xbe as ::core::ffi::c_int as xmlChar,
    0xbf as ::core::ffi::c_int as xmlChar,
    0xc0 as ::core::ffi::c_int as xmlChar,
    0xc1 as ::core::ffi::c_int as xmlChar,
    0xc2 as ::core::ffi::c_int as xmlChar,
    0xc3 as ::core::ffi::c_int as xmlChar,
    0xc4 as ::core::ffi::c_int as xmlChar,
    0xc5 as ::core::ffi::c_int as xmlChar,
    0xc6 as ::core::ffi::c_int as xmlChar,
    0xc7 as ::core::ffi::c_int as xmlChar,
    0xc8 as ::core::ffi::c_int as xmlChar,
    0xc9 as ::core::ffi::c_int as xmlChar,
    0xca as ::core::ffi::c_int as xmlChar,
    0xcb as ::core::ffi::c_int as xmlChar,
    0xcc as ::core::ffi::c_int as xmlChar,
    0xcd as ::core::ffi::c_int as xmlChar,
    0xce as ::core::ffi::c_int as xmlChar,
    0xcf as ::core::ffi::c_int as xmlChar,
    0xd0 as ::core::ffi::c_int as xmlChar,
    0xd1 as ::core::ffi::c_int as xmlChar,
    0xd2 as ::core::ffi::c_int as xmlChar,
    0xd3 as ::core::ffi::c_int as xmlChar,
    0xd4 as ::core::ffi::c_int as xmlChar,
    0xd5 as ::core::ffi::c_int as xmlChar,
    0xd6 as ::core::ffi::c_int as xmlChar,
    0xd7 as ::core::ffi::c_int as xmlChar,
    0xd8 as ::core::ffi::c_int as xmlChar,
    0xd9 as ::core::ffi::c_int as xmlChar,
    0xda as ::core::ffi::c_int as xmlChar,
    0xdb as ::core::ffi::c_int as xmlChar,
    0xdc as ::core::ffi::c_int as xmlChar,
    0xdd as ::core::ffi::c_int as xmlChar,
    0xde as ::core::ffi::c_int as xmlChar,
    0xdf as ::core::ffi::c_int as xmlChar,
    0xe0 as ::core::ffi::c_int as xmlChar,
    0xe1 as ::core::ffi::c_int as xmlChar,
    0xe2 as ::core::ffi::c_int as xmlChar,
    0xe3 as ::core::ffi::c_int as xmlChar,
    0xe4 as ::core::ffi::c_int as xmlChar,
    0xe5 as ::core::ffi::c_int as xmlChar,
    0xe6 as ::core::ffi::c_int as xmlChar,
    0xe7 as ::core::ffi::c_int as xmlChar,
    0xe8 as ::core::ffi::c_int as xmlChar,
    0xe9 as ::core::ffi::c_int as xmlChar,
    0xea as ::core::ffi::c_int as xmlChar,
    0xeb as ::core::ffi::c_int as xmlChar,
    0xec as ::core::ffi::c_int as xmlChar,
    0xed as ::core::ffi::c_int as xmlChar,
    0xee as ::core::ffi::c_int as xmlChar,
    0xef as ::core::ffi::c_int as xmlChar,
    0xf0 as ::core::ffi::c_int as xmlChar,
    0xf1 as ::core::ffi::c_int as xmlChar,
    0xf2 as ::core::ffi::c_int as xmlChar,
    0xf3 as ::core::ffi::c_int as xmlChar,
    0xf4 as ::core::ffi::c_int as xmlChar,
    0xf5 as ::core::ffi::c_int as xmlChar,
    0xf6 as ::core::ffi::c_int as xmlChar,
    0xf7 as ::core::ffi::c_int as xmlChar,
    0xf8 as ::core::ffi::c_int as xmlChar,
    0xf9 as ::core::ffi::c_int as xmlChar,
    0xfa as ::core::ffi::c_int as xmlChar,
    0xfb as ::core::ffi::c_int as xmlChar,
    0xfc as ::core::ffi::c_int as xmlChar,
    0xfd as ::core::ffi::c_int as xmlChar,
    0xfe as ::core::ffi::c_int as xmlChar,
    0xff as ::core::ffi::c_int as xmlChar,
];
#[no_mangle]
pub unsafe extern "C" fn xmlStrcasecmp(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_int = 0;
    if str1 == str2 {
        return 0 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        let fresh6 = str1;
        str1 = str1.offset(1);
        tmp = casemap[*fresh6 as usize] as ::core::ffi::c_int
            - casemap[*str2 as usize] as ::core::ffi::c_int;
        if tmp != 0 as ::core::ffi::c_int {
            return tmp;
        }
        let fresh7 = str2;
        str2 = str2.offset(1);
        if !(*fresh7 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrncasecmp(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp: ::core::ffi::c_int = 0;
    if len <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if str1 == str2 {
        return 0 as ::core::ffi::c_int;
    }
    if str1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if str2.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    loop {
        let fresh2 = str1;
        str1 = str1.offset(1);
        tmp = casemap[*fresh2 as usize] as ::core::ffi::c_int
            - casemap[*str2 as usize] as ::core::ffi::c_int;
        if tmp != 0 as ::core::ffi::c_int || {
            len -= 1;
            len == 0 as ::core::ffi::c_int
        } {
            return tmp;
        }
        let fresh3 = str2;
        str2 = str2.offset(1);
        if !(*fresh3 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrchr(mut str: *const xmlChar, mut val: xmlChar) -> *const xmlChar {
    if str.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    while *str as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *str as ::core::ffi::c_int == val as ::core::ffi::c_int {
            return str as *mut xmlChar;
        }
        str = str.offset(1);
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrstr(
    mut str: *const xmlChar,
    mut val: *const xmlChar,
) -> *const xmlChar {
    let mut n: ::core::ffi::c_int = 0;
    if str.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if val.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    n = xmlStrlen(val);
    if n == 0 as ::core::ffi::c_int {
        return str;
    }
    while *str as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *str as ::core::ffi::c_int == *val as ::core::ffi::c_int {
            if xmlStrncmp(str, val, n) == 0 {
                return str;
            }
        }
        str = str.offset(1);
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrcasestr(
    mut str: *const xmlChar,
    mut val: *const xmlChar,
) -> *const xmlChar {
    let mut n: ::core::ffi::c_int = 0;
    if str.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if val.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    n = xmlStrlen(val);
    if n == 0 as ::core::ffi::c_int {
        return str;
    }
    while *str as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if casemap[*str as usize] as ::core::ffi::c_int
            == casemap[*val as usize] as ::core::ffi::c_int
        {
            if xmlStrncasecmp(str, val, n) == 0 {
                return str;
            }
        }
        str = str.offset(1);
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrsub(
    mut str: *const xmlChar,
    mut start: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut i: ::core::ffi::c_int = 0;
    if str.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if start < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < start {
        if *str as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        str = str.offset(1);
        i += 1;
    }
    if *str as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStrndup(str, len);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrlen(mut str: *const xmlChar) -> ::core::ffi::c_int {
    let mut len: size_t = if !str.is_null() {
        strlen(str as *const ::core::ffi::c_char)
    } else {
        0 as size_t
    };
    return (if len > INT_MAX as size_t {
        0 as size_t
    } else {
        len
    }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrncat(
    mut cur: *mut xmlChar,
    mut add: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut size: ::core::ffi::c_int = 0;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if add.is_null() || len == 0 as ::core::ffi::c_int {
        return cur;
    }
    if len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if cur.is_null() {
        return xmlStrndup(add, len);
    }
    size = xmlStrlen(cur);
    if size < 0 as ::core::ffi::c_int || size > INT_MAX - len {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlRealloc.expect("non-null function pointer")(
        cur as *mut ::core::ffi::c_void,
        (size as size_t)
            .wrapping_add(len as size_t)
            .wrapping_add(1 as size_t),
    ) as *mut xmlChar;
    if ret.is_null() {
        return cur;
    }
    memcpy(
        ret.offset(size as isize) as *mut xmlChar as *mut ::core::ffi::c_void,
        add as *const ::core::ffi::c_void,
        len as size_t,
    );
    *ret.offset((size + len) as isize) = 0 as xmlChar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrncatNew(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut size: ::core::ffi::c_int = 0;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if len < 0 as ::core::ffi::c_int {
        len = xmlStrlen(str2);
        if len < 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<xmlChar>();
        }
    }
    if str2.is_null() || len == 0 as ::core::ffi::c_int {
        return xmlStrdup(str1);
    }
    if str1.is_null() {
        return xmlStrndup(str2, len);
    }
    size = xmlStrlen(str1);
    if size < 0 as ::core::ffi::c_int || size > INT_MAX - len {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        (size as size_t)
            .wrapping_add(len as size_t)
            .wrapping_add(1 as size_t),
    ) as *mut xmlChar;
    if ret.is_null() {
        return xmlStrndup(str1, size);
    }
    memcpy(
        ret as *mut ::core::ffi::c_void,
        str1 as *const ::core::ffi::c_void,
        size as size_t,
    );
    memcpy(
        ret.offset(size as isize) as *mut xmlChar as *mut ::core::ffi::c_void,
        str2 as *const ::core::ffi::c_void,
        len as size_t,
    );
    *ret.offset((size + len) as isize) = 0 as xmlChar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrcat(mut cur: *mut xmlChar, mut add: *const xmlChar) -> *mut xmlChar {
    let mut len: size_t = 0;
    if add.is_null() {
        return cur;
    }
    if cur.is_null() {
        return xmlStrdup(add);
    }
    len = strlen(add as *const ::core::ffi::c_char);
    if len > INT_MAX as size_t {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStrncat(cur, add, len as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrPrintf(
    mut buf: *mut xmlChar,
    mut len: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
    mut args: ...
) -> ::core::ffi::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut ret: ::core::ffi::c_int = 0;
    if buf.is_null() || msg.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    args_0 = args.clone();
    ret = vsnprintf(
        buf as *mut ::core::ffi::c_char,
        len as size_t,
        msg,
        args_0.as_va_list(),
    );
    *buf.offset((len - 1 as ::core::ffi::c_int) as isize) = 0 as xmlChar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStrVPrintf(
    mut buf: *mut xmlChar,
    mut len: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if buf.is_null() || msg.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ret = vsnprintf(
        buf as *mut ::core::ffi::c_char,
        len as size_t,
        msg,
        ap.as_va_list(),
    );
    *buf.offset((len - 1 as ::core::ffi::c_int) as isize) = 0 as xmlChar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Size(mut utf: *const xmlChar) -> ::core::ffi::c_int {
    let mut mask: xmlChar = 0;
    let mut len: ::core::ffi::c_int = 0;
    if utf.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*utf as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if *utf as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    len = 2 as ::core::ffi::c_int;
    mask = 0x20 as xmlChar;
    while mask as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *utf as ::core::ffi::c_int & mask as ::core::ffi::c_int == 0 {
            return len;
        }
        len += 1;
        mask = (mask as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as xmlChar;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Charcmp(
    mut utf1: *const xmlChar,
    mut utf2: *const xmlChar,
) -> ::core::ffi::c_int {
    if utf1.is_null() {
        if utf2.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return -(1 as ::core::ffi::c_int);
    }
    return xmlStrncmp(utf1, utf2, xmlUTF8Size(utf1));
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strlen(mut utf: *const xmlChar) -> ::core::ffi::c_int {
    let mut ret: size_t = 0 as size_t;
    if utf.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    while *utf as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *utf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            if *utf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            if *utf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                if *utf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return -(1 as ::core::ffi::c_int);
                }
                if *utf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xf0 as ::core::ffi::c_int
                {
                    if *utf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        != 0xf0 as ::core::ffi::c_int
                        || *utf.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            & 0xc0 as ::core::ffi::c_int
                            != 0x80 as ::core::ffi::c_int
                    {
                        return -(1 as ::core::ffi::c_int);
                    }
                    utf = utf.offset(4 as ::core::ffi::c_int as isize);
                } else {
                    utf = utf.offset(3 as ::core::ffi::c_int as isize);
                }
            } else {
                utf = utf.offset(2 as ::core::ffi::c_int as isize);
            }
        } else {
            utf = utf.offset(1);
        }
        ret = ret.wrapping_add(1);
    }
    return (if ret > INT_MAX as size_t {
        0 as size_t
    } else {
        ret
    }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetUTF8Char(
    mut utf: *const ::core::ffi::c_uchar,
    mut len: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut c: ::core::ffi::c_uint = 0;
    if !utf.is_null() {
        if !len.is_null() {
            c = *utf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
            if c < 0x80 as ::core::ffi::c_uint {
                if *len < 1 as ::core::ffi::c_int {
                    current_block = 8918797746248981988;
                } else {
                    *len = 1 as ::core::ffi::c_int;
                    current_block = 10043043949733653460;
                }
            } else if *len < 2 as ::core::ffi::c_int
                || *utf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                current_block = 8918797746248981988;
            } else if c < 0xe0 as ::core::ffi::c_uint {
                if c < 0xc2 as ::core::ffi::c_uint {
                    current_block = 8918797746248981988;
                } else {
                    *len = 2 as ::core::ffi::c_int;
                    c = (c & 0x1f as ::core::ffi::c_uint) << 6 as ::core::ffi::c_int;
                    c |= (*utf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        & 0x3f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint;
                    current_block = 10043043949733653460;
                }
            } else if *len < 3 as ::core::ffi::c_int
                || *utf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                current_block = 8918797746248981988;
            } else if c < 0xf0 as ::core::ffi::c_uint {
                *len = 3 as ::core::ffi::c_int;
                c = (c & 0xf as ::core::ffi::c_uint) << 12 as ::core::ffi::c_int;
                c |= ((*utf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                c |= (*utf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int) as ::core::ffi::c_uint;
                if c < 0x800 as ::core::ffi::c_uint
                    || c >= 0xd800 as ::core::ffi::c_uint && c < 0xe000 as ::core::ffi::c_uint
                {
                    current_block = 8918797746248981988;
                } else {
                    current_block = 10043043949733653460;
                }
            } else if *len < 4 as ::core::ffi::c_int
                || *utf.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                current_block = 8918797746248981988;
            } else {
                *len = 4 as ::core::ffi::c_int;
                c = (c & 0x7 as ::core::ffi::c_uint) << 18 as ::core::ffi::c_int;
                c |= ((*utf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int)
                    << 12 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                c |= ((*utf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                c |= (*utf.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x3f as ::core::ffi::c_int) as ::core::ffi::c_uint;
                if c < 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint
                    || c >= 0x110000 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    current_block = 8918797746248981988;
                } else {
                    current_block = 10043043949733653460;
                }
            }
            match current_block {
                8918797746248981988 => {}
                _ => return c as ::core::ffi::c_int,
            }
        }
    }
    if !len.is_null() {
        *len = 0 as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCheckUTF8(mut utf: *const ::core::ffi::c_uchar) -> ::core::ffi::c_int {
    let mut ix: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_uchar = 0;
    if utf.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        c = *utf.offset(0 as ::core::ffi::c_int as isize);
        if !(c != 0) {
            break;
        }
        ix = 0 as ::core::ffi::c_int;
        if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            ix = 1 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int
        {
            if *utf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            ix = 2 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int
        {
            if *utf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
                || *utf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            ix = 3 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int
        {
            if *utf.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
                || *utf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                || *utf.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            ix = 4 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int;
        }
        utf = utf.offset(ix as isize);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strsize(
    mut utf: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ptr: *const xmlChar = utf;
    let mut ch: ::core::ffi::c_int = 0;
    let mut ret: size_t = 0;
    if utf.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if len <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let fresh15 = len;
        len = len - 1;
        if !(fresh15 > 0 as ::core::ffi::c_int) {
            break;
        }
        if *ptr == 0 {
            break;
        }
        let fresh16 = ptr;
        ptr = ptr.offset(1);
        ch = *fresh16 as ::core::ffi::c_int;
        if ch & 0x80 as ::core::ffi::c_int != 0 {
            loop {
                ch <<= 1 as ::core::ffi::c_int;
                if !(ch & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
                if *ptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                ptr = ptr.offset(1);
            }
        }
    }
    ret = ptr.offset_from(utf) as ::core::ffi::c_long as size_t;
    return (if ret > INT_MAX as size_t {
        0 as size_t
    } else {
        ret
    }) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strndup(
    mut utf: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut i: ::core::ffi::c_int = 0;
    if utf.is_null() || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    i = xmlUTF8Strsize(utf, len);
    ret = xmlMallocAtomic.expect("non-null function pointer")(
        (i as size_t).wrapping_add(1 as size_t),
    ) as *mut xmlChar;
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    memcpy(
        ret as *mut ::core::ffi::c_void,
        utf as *const ::core::ffi::c_void,
        i as size_t,
    );
    *ret.offset(i as isize) = 0 as xmlChar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strpos(
    mut utf: *const xmlChar,
    mut pos: ::core::ffi::c_int,
) -> *const xmlChar {
    let mut ch: ::core::ffi::c_int = 0;
    if utf.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if pos < 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    loop {
        let fresh17 = pos;
        pos = pos - 1;
        if !(fresh17 != 0) {
            break;
        }
        let fresh18 = utf;
        utf = utf.offset(1);
        ch = *fresh18 as ::core::ffi::c_int;
        if ch == 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<xmlChar>();
        }
        if ch & 0x80 as ::core::ffi::c_int != 0 {
            if ch & 0xc0 as ::core::ffi::c_int != 0xc0 as ::core::ffi::c_int {
                return ::core::ptr::null::<xmlChar>();
            }
            loop {
                ch <<= 1 as ::core::ffi::c_int;
                if !(ch & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
                let fresh19 = utf;
                utf = utf.offset(1);
                if *fresh19 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return ::core::ptr::null::<xmlChar>();
                }
            }
        }
    }
    return utf as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strloc(
    mut utf: *const xmlChar,
    mut utfchar: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    if utf.is_null() || utfchar.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    size = xmlUTF8Strsize(utfchar, 1 as ::core::ffi::c_int);
    i = 0 as size_t;
    loop {
        ch = *utf as ::core::ffi::c_int;
        if !(ch != 0 as ::core::ffi::c_int) {
            break;
        }
        if xmlStrncmp(utf, utfchar, size) == 0 as ::core::ffi::c_int {
            return (if i > INT_MAX as size_t {
                0 as size_t
            } else {
                i
            }) as ::core::ffi::c_int;
        }
        utf = utf.offset(1);
        if ch & 0x80 as ::core::ffi::c_int != 0 {
            if ch & 0xc0 as ::core::ffi::c_int != 0xc0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            loop {
                ch <<= 1 as ::core::ffi::c_int;
                if !(ch & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
                let fresh20 = utf;
                utf = utf.offset(1);
                if *fresh20 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return -(1 as ::core::ffi::c_int);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUTF8Strsub(
    mut utf: *const xmlChar,
    mut start: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut i: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    if utf.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if start < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < start {
        let fresh21 = utf;
        utf = utf.offset(1);
        ch = *fresh21 as ::core::ffi::c_int;
        if ch == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if ch & 0x80 as ::core::ffi::c_int != 0 {
            if ch & 0xc0 as ::core::ffi::c_int != 0xc0 as ::core::ffi::c_int {
                return ::core::ptr::null_mut::<xmlChar>();
            }
            loop {
                ch <<= 1 as ::core::ffi::c_int;
                if !(ch & 0x80 as ::core::ffi::c_int != 0) {
                    break;
                }
                let fresh22 = utf;
                utf = utf.offset(1);
                if *fresh22 as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    return ::core::ptr::null_mut::<xmlChar>();
                }
            }
        }
        i += 1;
    }
    return xmlUTF8Strndup(utf, len);
}
#[no_mangle]
pub unsafe extern "C" fn xmlEscapeFormatString(mut msg: *mut *mut xmlChar) -> *mut xmlChar {
    let mut msgPtr: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut result: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut resultPtr: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut count: size_t = 0 as size_t;
    let mut msgLen: size_t = 0 as size_t;
    let mut resultLen: size_t = 0 as size_t;
    if msg.is_null() || (*msg).is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    msgPtr = *msg;
    while *msgPtr as ::core::ffi::c_int != '\0' as i32 {
        msgLen = msgLen.wrapping_add(1);
        if *msgPtr as ::core::ffi::c_int == '%' as i32 {
            count = count.wrapping_add(1);
        }
        msgPtr = msgPtr.offset(1);
    }
    if count == 0 as size_t {
        return *msg;
    }
    if count > INT_MAX as size_t || msgLen > (INT_MAX as size_t).wrapping_sub(count) {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    resultLen = msgLen.wrapping_add(count).wrapping_add(1 as size_t);
    result = xmlMallocAtomic.expect("non-null function pointer")(resultLen) as *mut xmlChar;
    if result.is_null() {
        xmlFree.expect("non-null function pointer")(*msg as *mut ::core::ffi::c_void);
        *msg = ::core::ptr::null_mut::<xmlChar>();
        return ::core::ptr::null_mut::<xmlChar>();
    }
    msgPtr = *msg;
    resultPtr = result;
    while *msgPtr as ::core::ffi::c_int != '\0' as i32 {
        *resultPtr = *msgPtr;
        if *msgPtr as ::core::ffi::c_int == '%' as i32 {
            resultPtr = resultPtr.offset(1);
            *resultPtr = '%' as i32 as xmlChar;
        }
        msgPtr = msgPtr.offset(1);
        resultPtr = resultPtr.offset(1);
    }
    *result.offset(resultLen.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as xmlChar;
    xmlFree.expect("non-null function pointer")(*msg as *mut ::core::ffi::c_void);
    *msg = result;
    return *msg;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
