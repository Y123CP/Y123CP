extern "C" {
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub arg: *const ::core::ffi::c_char,
    pub flag: ::core::ffi::c_int,
}
static mut format_args: [C2RustUnnamed; 4] = [
    C2RustUnnamed {
        arg: b"plain\0" as *const u8 as *const ::core::ffi::c_char,
        flag: JSON_C_TO_STRING_PLAIN,
    },
    C2RustUnnamed {
        arg: b"spaced\0" as *const u8 as *const ::core::ffi::c_char,
        flag: JSON_C_TO_STRING_SPACED,
    },
    C2RustUnnamed {
        arg: b"pretty\0" as *const u8 as *const ::core::ffi::c_char,
        flag: JSON_C_TO_STRING_PRETTY,
    },
    C2RustUnnamed {
        arg: b"pretty_tab\0" as *const u8 as *const ::core::ffi::c_char,
        flag: JSON_C_TO_STRING_PRETTY_TAB,
    },
];
#[no_mangle]
pub unsafe extern "C" fn parse_flags(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut arg_idx: ::core::ffi::c_int = 0;
    let mut sflags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    arg_idx = 1 as ::core::ffi::c_int;
    while arg_idx < argc {
        let mut jj: ::core::ffi::c_int = 0;
        jj = 0 as ::core::ffi::c_int;
        while jj
            < (::core::mem::size_of::<[C2RustUnnamed; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
                as ::core::ffi::c_int
        {
            if strcasecmp(*argv.offset(arg_idx as isize), format_args[jj as usize].arg)
                == 0 as ::core::ffi::c_int
            {
                sflags |= format_args[jj as usize].flag;
                break;
            } else {
                jj += 1;
            }
        }
        if jj as usize
            == (::core::mem::size_of::<[C2RustUnnamed; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize)
        {
            printf(
                b"Unknown arg: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(arg_idx as isize),
            );
            exit(1 as ::core::ffi::c_int);
        }
        arg_idx += 1;
    }
    return sflags;
}
pub const JSON_C_TO_STRING_PLAIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_SPACED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_PRETTY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const JSON_C_TO_STRING_PRETTY_TAB: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
