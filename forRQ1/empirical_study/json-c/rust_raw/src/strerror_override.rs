extern "C" {
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn getenv(name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub errno_value: ::core::ffi::c_int,
    pub errno_str: *const ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut errno_list: [C2RustUnnamed; 36] = [C2RustUnnamed {
    errno_value: 0,
    errno_str: ::core::ptr::null::<::core::ffi::c_char>(),
}; 36];
static mut _json_c_strerror_enable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut errno_buf: [::core::ffi::c_char; 128] = unsafe {
    ::core::mem::transmute::<
        [u8; 128],
        [::core::ffi::c_char; 128],
    >(
        *b"ERRNO=\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn _json_c_strerror(
    mut errno_in: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut start_idx: ::core::ffi::c_int = 0;
    let mut digbuf: [::core::ffi::c_char; 20] = [0; 20];
    let mut ii: ::core::ffi::c_int = 0;
    let mut jj: ::core::ffi::c_int = 0;
    if _json_c_strerror_enable == 0 {
        _json_c_strerror_enable =
            if getenv(b"_JSON_C_STRERROR_ENABLE\0" as *const u8 as *const ::core::ffi::c_char)
                .is_null()
            {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
    }
    if _json_c_strerror_enable == -(1 as ::core::ffi::c_int) {
        return strerror(errno_in);
    }
    ii = 0 as ::core::ffi::c_int;
    while !errno_list[ii as usize].errno_str.is_null() {
        let mut errno_str: *const ::core::ffi::c_char = errno_list[ii as usize].errno_str;
        if errno_list[ii as usize].errno_value != errno_in {
            ii += 1;
        } else {
            start_idx = (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int;
            jj = 0 as ::core::ffi::c_int;
            while *errno_str.offset(jj as isize) as ::core::ffi::c_int != '\0' as i32 {
                errno_buf[start_idx as usize] = *errno_str.offset(jj as isize);
                jj += 1;
                start_idx += 1;
            }
            errno_buf[start_idx as usize] = '\0' as i32 as ::core::ffi::c_char;
            return &raw mut errno_buf as *mut ::core::ffi::c_char;
        }
    }
    ii = 0 as ::core::ffi::c_int;
    while errno_in >= 10 as ::core::ffi::c_int {
        digbuf[ii as usize] =
            ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"0123456789\0")
                [(errno_in % 10 as ::core::ffi::c_int) as usize];
        errno_in /= 10 as ::core::ffi::c_int;
        ii += 1;
    }
    digbuf[ii as usize] =
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"0123456789\0")
            [(errno_in % 10 as ::core::ffi::c_int) as usize];
    start_idx = (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
    while ii >= 0 as ::core::ffi::c_int {
        errno_buf[start_idx as usize] = digbuf[ii as usize];
        ii -= 1;
        start_idx += 1;
    }
    errno_buf[start_idx as usize] = '\0' as i32 as ::core::ffi::c_char;
    return &raw mut errno_buf as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn run_static_initializers() {
    errno_list = [
        C2RustUnnamed {
            errno_value: 1 as ::core::ffi::c_int,
            errno_str: (b"undef_EPERM\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 2 as ::core::ffi::c_int,
            errno_str: (b"undef_ENOENT\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 3 as ::core::ffi::c_int,
            errno_str: (b"undef_ESRCH\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 4 as ::core::ffi::c_int,
            errno_str: (b"undef_EINTR\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 5 as ::core::ffi::c_int,
            errno_str: (b"undef_EIO\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 6 as ::core::ffi::c_int,
            errno_str: (b"undef_ENXIO\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 7 as ::core::ffi::c_int,
            errno_str: (b"undef_E2BIG\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 8 as ::core::ffi::c_int,
            errno_str: (b"undef_ENOEXEC\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 9 as ::core::ffi::c_int,
            errno_str: (b"undef_EBADF\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 10 as ::core::ffi::c_int,
            errno_str: (b"undef_ECHILD\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 35 as ::core::ffi::c_int,
            errno_str: (b"undef_EDEADLK\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 12 as ::core::ffi::c_int,
            errno_str: (b"undef_ENOMEM\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 13 as ::core::ffi::c_int,
            errno_str: (b"undef_EACCES\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 14 as ::core::ffi::c_int,
            errno_str: (b"undef_EFAULT\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 15 as ::core::ffi::c_int,
            errno_str: (b"undef_ENOTBLK\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 16 as ::core::ffi::c_int,
            errno_str: (b"undef_EBUSY\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 17 as ::core::ffi::c_int,
            errno_str: (b"undef_EEXIST\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 18 as ::core::ffi::c_int,
            errno_str: (b"undef_EXDEV\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 19 as ::core::ffi::c_int,
            errno_str: (b"undef_ENODEV\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 20 as ::core::ffi::c_int,
            errno_str: (b"undef_ENOTDIR\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 21 as ::core::ffi::c_int,
            errno_str: (b"undef_EISDIR\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 22 as ::core::ffi::c_int,
            errno_str: (b"undef_EINVAL\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 23 as ::core::ffi::c_int,
            errno_str: (b"undef_ENFILE\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 24 as ::core::ffi::c_int,
            errno_str: (b"undef_EMFILE\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 25 as ::core::ffi::c_int,
            errno_str: (b"undef_ENOTTY\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 26 as ::core::ffi::c_int,
            errno_str: (b"undef_ETXTBSY\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 27 as ::core::ffi::c_int,
            errno_str: (b"undef_EFBIG\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 28 as ::core::ffi::c_int,
            errno_str: (b"undef_ENOSPC\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 29 as ::core::ffi::c_int,
            errno_str: (b"undef_ESPIPE\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 30 as ::core::ffi::c_int,
            errno_str: (b"undef_EROFS\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 31 as ::core::ffi::c_int,
            errno_str: (b"undef_EMLINK\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 32 as ::core::ffi::c_int,
            errno_str: (b"undef_EPIPE\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 33 as ::core::ffi::c_int,
            errno_str: (b"undef_EDOM\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 34 as ::core::ffi::c_int,
            errno_str: (b"undef_ERANGE\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 11 as ::core::ffi::c_int,
            errno_str: (b"undef_EAGAIN\0" as *const u8 as *const ::core::ffi::c_char)
                .offset(6 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
        },
        C2RustUnnamed {
            errno_value: 0 as ::core::ffi::c_int,
            errno_str: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
