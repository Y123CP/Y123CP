#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
use core::ffi::*;
use ::lil_cleaned::src::ffi::*;
use ::lil_cleaned::src::c_consts::*;
use ::lil_cleaned::src::c_structs::*;
use ::lil_cleaned::src::c_types::*;
use ::lil_cleaned::src::c_extern_types::*;
#[allow(unused_imports)]
use ::lil_cleaned;
use ::lil_cleaned::src::lil::lil_alloc_list;
use ::lil_cleaned::src::lil::lil_alloc_string;
use ::lil_cleaned::src::lil::lil_callback;
use ::lil_cleaned::src::lil::lil_error;
use ::lil_cleaned::src::lil::lil_free;
use ::lil_cleaned::src::lil::lil_free_list;
use ::lil_cleaned::src::lil::lil_free_value;
use ::lil_cleaned::src::lil::lil_list_append;
use ::lil_cleaned::src::lil::lil_list_to_value;
use ::lil_cleaned::src::lil::lil_new;
use ::lil_cleaned::src::lil::lil_parse;
use ::lil_cleaned::src::lil::lil_register;
use ::lil_cleaned::src::lil::lil_set_var;
use ::lil_cleaned::src::lil::lil_to_integer;
use ::lil_cleaned::src::lil::lil_to_string;
pub use lil_cleaned::src::lil::_lil_value_t;
pub use lil_cleaned::src::lil::_lil_var_t;
pub use lil_cleaned::src::lil::_lil_list_t;
pub use lil_cleaned::src::lil::_lil_t;
extern "C" {
    fn popen(
        __command: *const c_char,
        __modes: *const c_char,
    ) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> c_int;
}

pub type __ssize_t = c_long;
pub type ssize_t = __ssize_t;

pub type lil_value_t = *mut _lil_value_t;
pub type lil_var_t = *mut _lil_var_t;
pub type lil_list_t = *mut _lil_list_t;
pub type lil_t = *mut _lil_t;
pub type lil_func_proc_t =
    Option<unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t>;

pub const EOF: c_int = -(1 as c_int);

static mut running: c_int = 1 as c_int;
static mut exit_code: c_int = 0 as c_int;
extern "C" fn do_exit(mut lil: lil_t, mut val: lil_value_t) { unsafe {
    running = 0 as c_int;
    exit_code = lil_to_integer(val) as c_int;
} }
unsafe extern "C" fn do_system(
    mut argc: size_t,
    mut argv: *mut *mut c_char,
) -> *mut c_char {
    let mut cmd: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut cmdlen: c_int = 0 as c_int;
    let mut i: size_t = 0;
    let mut p: *mut FILE = ::core::ptr::null_mut::<FILE>();
    i = 0 as size_t;
    while i < argc {
        let mut len: size_t = strlen(*argv.offset(i as isize));
        if i != 0 as size_t {
            cmd = realloc(
                cmd as *mut c_void,
                (cmdlen + 1 as c_int) as size_t,
            ) as *mut c_char;
            let fresh0 = cmdlen;
            cmdlen = cmdlen + 1;
            *cmd.offset(fresh0 as isize) = ' ' as i32 as c_char;
        }
        cmd = realloc(
            cmd as *mut c_void,
            (cmdlen as size_t).wrapping_add(len),
        ) as *mut c_char;
        memcpy(
            cmd.offset(cmdlen as isize) as *mut c_void,
            *argv.offset(i as isize) as *const c_void,
            len,
        );
        cmdlen = (cmdlen as c_ulong).wrapping_add(len as c_ulong)
            as c_int as c_int;
        i = i.wrapping_add(1);
    }
    cmd = realloc(
        cmd as *mut c_void,
        (cmdlen + 1 as c_int) as size_t,
    ) as *mut c_char;
    *cmd.offset(cmdlen as isize) = 0 as c_char;
    p = popen(cmd, b"r\0" as *const u8 as *const c_char);
    free(cmd as *mut c_void);
    if !p.is_null() {
        let mut retval: *mut c_char = ::core::ptr::null_mut::<c_char>();
        let mut size: size_t = 0 as size_t;
        let mut buff: [c_char; 1024] = [0; 1024];
        let mut bytes: ssize_t = 0;
        loop {
            bytes = fread(
                &raw mut buff as *mut c_char as *mut c_void,
                1 as size_t,
                1024 as size_t,
                p,
            ) as ssize_t;
            if !(bytes != 0) {
                break;
            }
            retval = realloc(
                retval as *mut c_void,
                size.wrapping_add(bytes as size_t),
            ) as *mut c_char;
            memcpy(
                retval.offset(size as isize) as *mut c_void,
                &raw mut buff as *mut c_char as *const c_void,
                bytes as size_t,
            );
            size = (size as c_ulong).wrapping_add(bytes as c_ulong)
                as size_t as size_t;
        }
        retval = realloc(
            retval as *mut c_void,
            size.wrapping_add(1 as size_t),
        ) as *mut c_char;
        *retval.offset(size as isize) = 0 as c_char;
        pclose(p);
        return retval;
    } else {
        return ::core::ptr::null_mut::<c_char>();
    };
}
unsafe extern "C" fn fnc_writechar(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc == 0 {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    printf(
        b"%c\0" as *const u8 as *const c_char,
        lil_to_integer(*argv.offset(0 as c_int as isize)) as c_char
            as c_int,
    );
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_system(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let argv_view: &[lil_value_t] = unsafe { core::slice::from_raw_parts(argv, (argc) as usize) };
    let mut sargv: *mut *const c_char = malloc(
        (::core::mem::size_of::<*mut c_char>() as size_t)
            .wrapping_mul(argc.wrapping_add(1 as size_t)),
    ) as *mut *const c_char;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut rv: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut i: size_t = 0;
    if argc == 0 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    i = 0 as size_t;
    while i < argc {
        let ref mut fresh1 = *sargv.offset(i as isize);
        *fresh1 = lil_to_string(argv_view[(i) as usize]);
        i = i.wrapping_add(1);
    }
    let ref mut fresh2 = *sargv.offset(argc as isize);
    *fresh2 = ::core::ptr::null::<c_char>();
    rv = do_system(argc, sargv as *mut *mut c_char);
    if !rv.is_null() {
        r = lil_alloc_string(rv);
        free(rv as *mut c_void);
    }
    free(sargv as *mut c_void);
    return r;
}
unsafe extern "C" fn fnc_readline(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut len: size_t = 0 as size_t;
    let mut size: size_t = 64 as size_t;
    let mut buffer: *mut c_char = malloc(size) as *mut c_char;
    let mut ch: c_schar = 0;
    let mut retval: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    loop {
        ch = fgetc(stdin) as c_schar;
        if ch as c_int == EOF {
            break;
        }
        if ch as c_int == '\r' as i32 {
            continue;
        }
        if ch as c_int == '\n' as i32 {
            break;
        }
        if len < size {
            size = (size as c_ulong).wrapping_add(64 as c_ulong) as size_t
                as size_t;
            buffer = realloc(buffer as *mut c_void, size) as *mut c_char;
        }
        let fresh3 = len;
        len = len.wrapping_add(1);
        *buffer.offset(fresh3 as isize) = ch as c_char;
    }
    buffer = realloc(
        buffer as *mut c_void,
        len.wrapping_add(1 as size_t),
    ) as *mut c_char;
    *buffer.offset(len as isize) = 0 as c_char;
    retval = lil_alloc_string(buffer);
    free(buffer as *mut c_void);
    return retval;
}
extern "C" fn repl() -> c_int { unsafe {
    let mut buffer: [c_char; 16384] = [0; 16384];
    let mut lil: lil_t = lil_new();
    lil_register(
        lil,
        b"writechar\0" as *const u8 as *const c_char,
        Some(fnc_writechar as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"system\0" as *const u8 as *const c_char,
        Some(fnc_system as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"readline\0" as *const u8 as *const c_char,
        Some(fnc_readline as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    printf(
        b"Little Interpreted Language Interactive Shell\n\0" as *const u8
            as *const c_char,
    );
    lil_callback(
        lil,
        LIL_CALLBACK_EXIT,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(lil_t, lil_value_t) -> ()>,
            lil_callback_proc_t,
        >(Some(
            do_exit as unsafe extern "C" fn(lil_t, lil_value_t) -> (),
        )),
    );
    while running != 0 {
        let mut result: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
        let mut strres: *const c_char = ::core::ptr::null::<c_char>();
        let mut err_msg: *const c_char = ::core::ptr::null::<c_char>();
        let mut pos: size_t = 0;
        buffer[0 as c_int as usize] = 0 as c_char;
        printf(b"# \0" as *const u8 as *const c_char);
        if fgets(
            &raw mut buffer as *mut c_char,
            16384 as c_int,
            stdin,
        )
        .is_null()
        {
            break;
        }
        result = lil_parse(
            lil,
            &raw mut buffer as *mut c_char,
            0 as size_t,
            0 as c_int,
        );
        strres = lil_to_string(result);
        if *strres.offset(0 as c_int as isize) != 0 {
            printf(b"%s\n\0" as *const u8 as *const c_char, strres);
        }
        lil_free_value(result);
        if lil_error(lil, &raw mut err_msg, &raw mut pos) != 0 {
            printf(
                b"error at %i: %s\n\0" as *const u8 as *const c_char,
                pos as c_int,
                err_msg,
            );
        }
    }
    lil_free(lil);
    return exit_code;
} }
unsafe extern "C" fn nonint(
    mut argc: c_int,
    mut argv: *mut *const c_char,
) -> c_int {
    let mut lil: lil_t = lil_new();
    let mut filename: *const c_char = *argv.offset(1 as c_int as isize);
    let mut err_msg: *const c_char = ::core::ptr::null::<c_char>();
    let mut pos: size_t = 0;
    let mut arglist: lil_list_t = lil_alloc_list();
    let mut args: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut result: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut tmpcode: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut i: c_int = 0;
    lil_register(
        lil,
        b"writechar\0" as *const u8 as *const c_char,
        Some(fnc_writechar as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"system\0" as *const u8 as *const c_char,
        Some(fnc_system as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    i = 2 as c_int;
    while i < argc {
        lil_list_append(arglist, lil_alloc_string(*argv.offset(i as isize)));
        i += 1;
    }
    args = lil_list_to_value(arglist, 1 as c_int);
    lil_free_list(arglist);
    lil_set_var(
        lil,
        b"argv\0" as *const u8 as *const c_char,
        args,
        LIL_SETVAR_GLOBAL,
    );
    lil_free_value(args);
    tmpcode = malloc(strlen(filename).wrapping_add(256 as size_t)) as *mut c_char;
    sprintf(
        tmpcode,
        b"set __lilmain:code__ [read {%s}]\nif [streq $__lilmain:code__ ''] {print There is no code in the file or the file does not exist} {eval $__lilmain:code__}\n\0"
            as *const u8 as *const c_char,
        filename,
    );
    result = lil_parse(lil, tmpcode, 0 as size_t, 1 as c_int);
    free(tmpcode as *mut c_void);
    lil_free_value(result);
    if lil_error(lil, &raw mut err_msg, &raw mut pos) != 0 {
        fprintf(
            stderr,
            b"lil: error at %i: %s\n\0" as *const u8 as *const c_char,
            pos as c_int,
            err_msg,
        );
    }
    lil_free(lil);
    return exit_code;
}
unsafe fn main_0(
    mut argc: c_int,
    mut argv: *mut *const c_char,
) -> c_int {
    if argc < 2 as c_int {
        return repl();
    } else {
        return nonint(argc, argv);
    };
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as c_int,
            args_ptrs.as_mut_ptr() as *mut *const c_char,
        ) as i32)
    }
}
