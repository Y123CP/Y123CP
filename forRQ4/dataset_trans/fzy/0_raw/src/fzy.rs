#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#[allow(unused_imports)]
use ::fzy_raw;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tty_init(tty: *mut tty_t, tty_filename: *const ::core::ffi::c_char);
    fn tty_getheight(tty: *mut tty_t) -> size_t;
    fn options_parse(
        options: *mut options_t,
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    );
    fn choices_init(c: *mut choices_t, options: *mut options_t);
    fn choices_fread(c: *mut choices_t, file: *mut FILE, input_delimiter: ::core::ffi::c_char);
    fn choices_destroy(c: *mut choices_t);
    fn choices_available(c: *mut choices_t) -> size_t;
    fn choices_search(c: *mut choices_t, search: *const ::core::ffi::c_char);
    fn choices_get(c: *mut choices_t, n: size_t) -> *const ::core::ffi::c_char;
    fn choices_getscore(c: *mut choices_t, n: size_t) -> score_t;
    fn tty_interface_init(
        state: *mut tty_interface_t,
        tty: *mut tty_t,
        choices: *mut choices_t,
        options: *mut options_t,
    );
    fn tty_interface_run(state: *mut tty_interface_t) -> ::core::ffi::c_int;
}
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
pub type score_t = ::core::ffi::c_double;
pub type cc_t = ::core::ffi::c_uchar;
pub type speed_t = ::core::ffi::c_uint;
pub type tcflag_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_t {
    pub fdin: ::core::ffi::c_int,
    pub fout: *mut FILE,
    pub original_termios: termios,
    pub fgcolor: ::core::ffi::c_int,
    pub maxwidth: size_t,
    pub maxheight: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_t {
    pub benchmark: ::core::ffi::c_int,
    pub filter: *const ::core::ffi::c_char,
    pub init_search: *const ::core::ffi::c_char,
    pub tty_filename: *const ::core::ffi::c_char,
    pub show_scores: ::core::ffi::c_int,
    pub num_lines: ::core::ffi::c_uint,
    pub scrolloff: ::core::ffi::c_uint,
    pub prompt: *const ::core::ffi::c_char,
    pub workers: ::core::ffi::c_uint,
    pub input_delimiter: ::core::ffi::c_char,
    pub show_info: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scored_result {
    pub score: score_t,
    pub str_0: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct choices_t {
    pub buffer: *mut ::core::ffi::c_char,
    pub buffer_size: size_t,
    pub capacity: size_t,
    pub size: size_t,
    pub strings: *mut *const ::core::ffi::c_char,
    pub results: *mut scored_result,
    pub available: size_t,
    pub selection: size_t,
    pub worker_count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_interface_t {
    pub tty: *mut tty_t,
    pub choices: *mut choices_t,
    pub options: *mut options_t,
    pub search: [::core::ffi::c_char; 4097],
    pub last_search: [::core::ffi::c_char; 4097],
    pub cursor: size_t,
    pub ambiguous_key_pending: ::core::ffi::c_int,
    pub input: [::core::ffi::c_char; 32],
    pub exit: ::core::ffi::c_int,
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut options: options_t = options_t {
        benchmark: 0,
        filter: ::core::ptr::null::<::core::ffi::c_char>(),
        init_search: ::core::ptr::null::<::core::ffi::c_char>(),
        tty_filename: ::core::ptr::null::<::core::ffi::c_char>(),
        show_scores: 0,
        num_lines: 0,
        scrolloff: 0,
        prompt: ::core::ptr::null::<::core::ffi::c_char>(),
        workers: 0,
        input_delimiter: 0,
        show_info: 0,
    };
    options_parse(&raw mut options, argc, argv);
    let mut choices: choices_t = choices_t {
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        buffer_size: 0,
        capacity: 0,
        size: 0,
        strings: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        results: ::core::ptr::null_mut::<scored_result>(),
        available: 0,
        selection: 0,
        worker_count: 0,
    };
    choices_init(&raw mut choices, &raw mut options);
    if options.benchmark != 0 {
        if options.filter.is_null() {
            fprintf(
                stderr,
                b"Must specify -e/--show-matches with --benchmark\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            exit(EXIT_FAILURE);
        }
        choices_fread(&raw mut choices, stdin, options.input_delimiter);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < options.benchmark {
            choices_search(&raw mut choices, options.filter);
            i += 1;
        }
    } else if !options.filter.is_null() {
        choices_fread(&raw mut choices, stdin, options.input_delimiter);
        choices_search(&raw mut choices, options.filter);
        let mut i_0: size_t = 0 as size_t;
        while i_0 < choices_available(&raw mut choices) {
            if options.show_scores != 0 {
                printf(
                    b"%f\t\0" as *const u8 as *const ::core::ffi::c_char,
                    choices_getscore(&raw mut choices, i_0),
                );
            }
            printf(
                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                choices_get(&raw mut choices, i_0),
            );
            i_0 = i_0.wrapping_add(1);
        }
    } else {
        if isatty(STDIN_FILENO) != 0 {
            choices_fread(&raw mut choices, stdin, options.input_delimiter);
        }
        let mut tty: tty_t = tty_t {
            fdin: 0,
            fout: ::core::ptr::null_mut::<FILE>(),
            original_termios: termios {
                c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,
            },
            fgcolor: 0,
            maxwidth: 0,
            maxheight: 0,
        };
        tty_init(&raw mut tty, options.tty_filename);
        if isatty(STDIN_FILENO) == 0 {
            choices_fread(&raw mut choices, stdin, options.input_delimiter);
        }
        if options.num_lines as size_t > choices.size {
            options.num_lines = choices.size as ::core::ffi::c_uint;
        }
        let mut num_lines_adjustment: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if options.show_info != 0 {
            num_lines_adjustment += 1;
        }
        if options
            .num_lines
            .wrapping_add(num_lines_adjustment as ::core::ffi::c_uint) as size_t
            > tty_getheight(&raw mut tty)
        {
            options.num_lines = tty_getheight(&raw mut tty)
                .wrapping_sub(num_lines_adjustment as size_t)
                as ::core::ffi::c_uint;
        }
        let mut tty_interface: tty_interface_t = tty_interface_t {
            tty: ::core::ptr::null_mut::<tty_t>(),
            choices: ::core::ptr::null_mut::<choices_t>(),
            options: ::core::ptr::null_mut::<options_t>(),
            search: [0; 4097],
            last_search: [0; 4097],
            cursor: 0,
            ambiguous_key_pending: 0,
            input: [0; 32],
            exit: 0,
        };
        tty_interface_init(
            &raw mut tty_interface,
            &raw mut tty,
            &raw mut choices,
            &raw mut options,
        );
        ret = tty_interface_run(&raw mut tty_interface);
    }
    choices_destroy(&raw mut choices);
    return ret;
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
