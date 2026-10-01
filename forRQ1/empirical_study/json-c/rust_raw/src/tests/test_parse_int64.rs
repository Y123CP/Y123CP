extern "C" {
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn json_parse_int64(
        buf: *const ::core::ffi::c_char,
        retval: *mut int64_t,
    ) -> ::core::ffi::c_int;
    fn json_parse_uint64(
        buf: *const ::core::ffi::c_char,
        retval: *mut uint64_t,
    ) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn checkit(mut buf: *const ::core::ffi::c_char) {
    let mut cint64: int64_t = -(666 as ::core::ffi::c_int) as int64_t;
    let mut retval: ::core::ffi::c_int = json_parse_int64(buf, &raw mut cint64);
    printf(
        b"buf=%s parseit=%d, value=%ld \n\0" as *const u8 as *const ::core::ffi::c_char,
        buf,
        retval,
        cint64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn checkit_uint(mut buf: *const ::core::ffi::c_char) {
    let mut cuint64: uint64_t = 666 as uint64_t;
    let mut retval: ::core::ffi::c_int = json_parse_uint64(buf, &raw mut cuint64);
    printf(
        b"buf=%s parseit=%d, value=%lu \n\0" as *const u8 as *const ::core::ffi::c_char,
        buf,
        retval,
        cuint64,
    );
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 100] = [0; 100];
    printf(
        b"==========json_parse_int64() test===========\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    checkit(b"x\0" as *const u8 as *const ::core::ffi::c_char);
    checkit(b"0\0" as *const u8 as *const ::core::ffi::c_char);
    checkit(b"-0\0" as *const u8 as *const ::core::ffi::c_char);
    checkit(b"00000000\0" as *const u8 as *const ::core::ffi::c_char);
    checkit(b"-00000000\0" as *const u8 as *const ::core::ffi::c_char);
    checkit(b"1\0" as *const u8 as *const ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"2147483647\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-1\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"   -1\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"00001234\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"0001234x\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-00001234\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-00001234x\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"4294967295\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"4294967296\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"21474836470\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"31474836470\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-2147483647\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-2147483648\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-2147483649\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-21474836480\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"9223372036854775806\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"9223372036854775807\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"9223372036854775808\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-9223372036854775808\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-9223372036854775809\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"18446744073709551614\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"18446744073709551615\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"18446744073709551616\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-18446744073709551616\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"123\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit(&raw mut buf as *mut ::core::ffi::c_char);
    printf(
        b"\n==========json_parse_uint64() test===========\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    checkit_uint(b"x\0" as *const u8 as *const ::core::ffi::c_char);
    checkit_uint(b"0\0" as *const u8 as *const ::core::ffi::c_char);
    checkit_uint(b"-0\0" as *const u8 as *const ::core::ffi::c_char);
    checkit_uint(b"00000000\0" as *const u8 as *const ::core::ffi::c_char);
    checkit_uint(b"-00000000\0" as *const u8 as *const ::core::ffi::c_char);
    checkit_uint(b"1\0" as *const u8 as *const ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"2147483647\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-1\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"-9223372036854775808\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"   1\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"00001234\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"0001234x\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"4294967295\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"4294967296\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"21474836470\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"31474836470\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"9223372036854775806\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"9223372036854775807\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"9223372036854775808\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"18446744073709551614\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"18446744073709551615\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"18446744073709551616\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    strcpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"123\0" as *const u8 as *const ::core::ffi::c_char,
    );
    checkit_uint(&raw mut buf as *mut ::core::ffi::c_char);
    return 0 as ::core::ffi::c_int;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
