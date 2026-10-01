#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#[allow(unused_imports)]
use ::lil_raw;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _lil_value_t;
    pub type _lil_var_t;
    pub type _lil_list_t;
    pub type _lil_t;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn popen(
        __command: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn lil_new() -> lil_t;
    fn lil_free(lil: lil_t);
    fn lil_register(
        lil: lil_t,
        name: *const ::core::ffi::c_char,
        proc_0: lil_func_proc_t,
    ) -> ::core::ffi::c_int;
    fn lil_parse(
        lil: lil_t,
        code: *const ::core::ffi::c_char,
        codelen: size_t,
        funclevel: ::core::ffi::c_int,
    ) -> lil_value_t;
    fn lil_callback(lil: lil_t, cb: ::core::ffi::c_int, proc_0: lil_callback_proc_t);
    fn lil_error(
        lil: lil_t,
        msg: *mut *const ::core::ffi::c_char,
        pos: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn lil_to_string(val: lil_value_t) -> *const ::core::ffi::c_char;
    fn lil_to_integer(val: lil_value_t) -> lilint_t;
    fn lil_alloc_string(str: *const ::core::ffi::c_char) -> lil_value_t;
    fn lil_free_value(val: lil_value_t);
    fn lil_alloc_list() -> lil_list_t;
    fn lil_free_list(list: lil_list_t);
    fn lil_list_append(list: lil_list_t, val: lil_value_t);
    fn lil_list_to_value(list: lil_list_t, do_escape: ::core::ffi::c_int) -> lil_value_t;
    fn lil_set_var(
        lil: lil_t,
        name: *const ::core::ffi::c_char,
        val: lil_value_t,
        local: ::core::ffi::c_int,
    ) -> lil_var_t;
}
pub type __int64_t = i64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = usize;
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
pub type int64_t = __int64_t;
pub type lilint_t = int64_t;
pub type lil_value_t = *mut _lil_value_t;
pub type lil_var_t = *mut _lil_var_t;
pub type lil_list_t = *mut _lil_list_t;
pub type lil_t = *mut _lil_t;
pub type lil_func_proc_t =
    Option<unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t>;
pub type lil_callback_proc_t = Option<unsafe extern "C" fn() -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const LIL_SETVAR_GLOBAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LIL_CALLBACK_EXIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut running: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut exit_code: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn do_exit(mut lil: lil_t, mut val: lil_value_t) {
    running = 0 as ::core::ffi::c_int;
    exit_code = lil_to_integer(val) as ::core::ffi::c_int;
}
unsafe extern "C" fn do_system(
    mut argc: size_t,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cmdlen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0;
    let mut p: *mut FILE = ::core::ptr::null_mut::<FILE>();
    i = 0 as size_t;
    while i < argc {
        let mut len: size_t = strlen(*argv.offset(i as isize));
        if i != 0 as size_t {
            cmd = realloc(
                cmd as *mut ::core::ffi::c_void,
                (cmdlen + 1 as ::core::ffi::c_int) as size_t,
            ) as *mut ::core::ffi::c_char;
            let fresh0 = cmdlen;
            cmdlen = cmdlen + 1;
            *cmd.offset(fresh0 as isize) = ' ' as i32 as ::core::ffi::c_char;
        }
        cmd = realloc(
            cmd as *mut ::core::ffi::c_void,
            (cmdlen as size_t).wrapping_add(len),
        ) as *mut ::core::ffi::c_char;
        memcpy(
            cmd.offset(cmdlen as isize) as *mut ::core::ffi::c_void,
            *argv.offset(i as isize) as *const ::core::ffi::c_void,
            len,
        );
        cmdlen = (cmdlen as ::core::ffi::c_ulong).wrapping_add(len as ::core::ffi::c_ulong)
            as ::core::ffi::c_int as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    cmd = realloc(
        cmd as *mut ::core::ffi::c_void,
        (cmdlen + 1 as ::core::ffi::c_int) as size_t,
    ) as *mut ::core::ffi::c_char;
    *cmd.offset(cmdlen as isize) = 0 as ::core::ffi::c_char;
    p = popen(cmd, b"r\0" as *const u8 as *const ::core::ffi::c_char);
    free(cmd as *mut ::core::ffi::c_void);
    if !p.is_null() {
        let mut retval: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut size: size_t = 0 as size_t;
        let mut buff: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut bytes: ssize_t = 0;
        loop {
            bytes = fread(
                &raw mut buff as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                1 as size_t,
                1024 as size_t,
                p,
            ) as ssize_t;
            if !(bytes != 0) {
                break;
            }
            retval = realloc(
                retval as *mut ::core::ffi::c_void,
                size.wrapping_add(bytes as size_t),
            ) as *mut ::core::ffi::c_char;
            memcpy(
                retval.offset(size as isize) as *mut ::core::ffi::c_void,
                &raw mut buff as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                bytes as size_t,
            );
            size = (size as ::core::ffi::c_ulong).wrapping_add(bytes as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        retval = realloc(
            retval as *mut ::core::ffi::c_void,
            size.wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        *retval.offset(size as isize) = 0 as ::core::ffi::c_char;
        pclose(p);
        return retval;
    } else {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
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
        b"%c\0" as *const u8 as *const ::core::ffi::c_char,
        lil_to_integer(*argv.offset(0 as ::core::ffi::c_int as isize)) as ::core::ffi::c_char
            as ::core::ffi::c_int,
    );
    return ::core::ptr::null_mut::<_lil_value_t>();
}
unsafe extern "C" fn fnc_system(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut sargv: *mut *const ::core::ffi::c_char = malloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
            .wrapping_mul(argc.wrapping_add(1 as size_t)),
    ) as *mut *const ::core::ffi::c_char;
    let mut r: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut rv: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: size_t = 0;
    if argc == 0 as size_t {
        return ::core::ptr::null_mut::<_lil_value_t>();
    }
    i = 0 as size_t;
    while i < argc {
        let ref mut fresh1 = *sargv.offset(i as isize);
        *fresh1 = lil_to_string(*argv.offset(i as isize));
        i = i.wrapping_add(1);
    }
    let ref mut fresh2 = *sargv.offset(argc as isize);
    *fresh2 = ::core::ptr::null::<::core::ffi::c_char>();
    rv = do_system(argc, sargv as *mut *mut ::core::ffi::c_char);
    if !rv.is_null() {
        r = lil_alloc_string(rv);
        free(rv as *mut ::core::ffi::c_void);
    }
    free(sargv as *mut ::core::ffi::c_void);
    return r;
}
unsafe extern "C" fn fnc_readline(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut len: size_t = 0 as size_t;
    let mut size: size_t = 64 as size_t;
    let mut buffer: *mut ::core::ffi::c_char = malloc(size) as *mut ::core::ffi::c_char;
    let mut ch: ::core::ffi::c_schar = 0;
    let mut retval: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    loop {
        ch = fgetc(stdin) as ::core::ffi::c_schar;
        if ch as ::core::ffi::c_int == EOF {
            break;
        }
        if ch as ::core::ffi::c_int == '\r' as i32 {
            continue;
        }
        if ch as ::core::ffi::c_int == '\n' as i32 {
            break;
        }
        if len < size {
            size = (size as ::core::ffi::c_ulong).wrapping_add(64 as ::core::ffi::c_ulong) as size_t
                as size_t;
            buffer = realloc(buffer as *mut ::core::ffi::c_void, size) as *mut ::core::ffi::c_char;
        }
        let fresh3 = len;
        len = len.wrapping_add(1);
        *buffer.offset(fresh3 as isize) = ch as ::core::ffi::c_char;
    }
    buffer = realloc(
        buffer as *mut ::core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    *buffer.offset(len as isize) = 0 as ::core::ffi::c_char;
    retval = lil_alloc_string(buffer);
    free(buffer as *mut ::core::ffi::c_void);
    return retval;
}
unsafe extern "C" fn repl() -> ::core::ffi::c_int {
    let mut buffer: [::core::ffi::c_char; 16384] = [0; 16384];
    let mut lil: lil_t = lil_new();
    lil_register(
        lil,
        b"writechar\0" as *const u8 as *const ::core::ffi::c_char,
        Some(fnc_writechar as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"system\0" as *const u8 as *const ::core::ffi::c_char,
        Some(fnc_system as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"readline\0" as *const u8 as *const ::core::ffi::c_char,
        Some(fnc_readline as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    printf(
        b"Little Interpreted Language Interactive Shell\n\0" as *const u8
            as *const ::core::ffi::c_char,
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
        let mut strres: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut err_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut pos: size_t = 0;
        buffer[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        printf(b"# \0" as *const u8 as *const ::core::ffi::c_char);
        if fgets(
            &raw mut buffer as *mut ::core::ffi::c_char,
            16384 as ::core::ffi::c_int,
            stdin,
        )
        .is_null()
        {
            break;
        }
        result = lil_parse(
            lil,
            &raw mut buffer as *mut ::core::ffi::c_char,
            0 as size_t,
            0 as ::core::ffi::c_int,
        );
        strres = lil_to_string(result);
        if *strres.offset(0 as ::core::ffi::c_int as isize) != 0 {
            printf(b"%s\n\0" as *const u8 as *const ::core::ffi::c_char, strres);
        }
        lil_free_value(result);
        if lil_error(lil, &raw mut err_msg, &raw mut pos) != 0 {
            printf(
                b"error at %i: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                pos as ::core::ffi::c_int,
                err_msg,
            );
        }
    }
    lil_free(lil);
    return exit_code;
}
unsafe extern "C" fn nonint(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut lil: lil_t = lil_new();
    let mut filename: *const ::core::ffi::c_char = *argv.offset(1 as ::core::ffi::c_int as isize);
    let mut err_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pos: size_t = 0;
    let mut arglist: lil_list_t = lil_alloc_list();
    let mut args: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut result: lil_value_t = ::core::ptr::null_mut::<_lil_value_t>();
    let mut tmpcode: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    lil_register(
        lil,
        b"writechar\0" as *const u8 as *const ::core::ffi::c_char,
        Some(fnc_writechar as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"system\0" as *const u8 as *const ::core::ffi::c_char,
        Some(fnc_system as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t),
    );
    i = 2 as ::core::ffi::c_int;
    while i < argc {
        lil_list_append(arglist, lil_alloc_string(*argv.offset(i as isize)));
        i += 1;
    }
    args = lil_list_to_value(arglist, 1 as ::core::ffi::c_int);
    lil_free_list(arglist);
    lil_set_var(
        lil,
        b"argv\0" as *const u8 as *const ::core::ffi::c_char,
        args,
        LIL_SETVAR_GLOBAL,
    );
    lil_free_value(args);
    tmpcode = malloc(strlen(filename).wrapping_add(256 as size_t)) as *mut ::core::ffi::c_char;
    sprintf(
        tmpcode,
        b"set __lilmain:code__ [read {%s}]\nif [streq $__lilmain:code__ ''] {print There is no code in the file or the file does not exist} {eval $__lilmain:code__}\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        filename,
    );
    result = lil_parse(lil, tmpcode, 0 as size_t, 1 as ::core::ffi::c_int);
    free(tmpcode as *mut ::core::ffi::c_void);
    lil_free_value(result);
    if lil_error(lil, &raw mut err_msg, &raw mut pos) != 0 {
        fprintf(
            stderr,
            b"lil: error at %i: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            pos as ::core::ffi::c_int,
            err_msg,
        );
    }
    lil_free(lil);
    return exit_code;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if argc < 2 as ::core::ffi::c_int {
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
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *const ::core::ffi::c_char,
        ) as i32)
    }
}
