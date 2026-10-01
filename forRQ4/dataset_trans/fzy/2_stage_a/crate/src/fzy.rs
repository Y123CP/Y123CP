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
use core::ffi::*;
use ::fzy_cleaned::src::ffi::*;
use ::fzy_cleaned::src::c_consts::*;
use ::fzy_cleaned::src::c_structs::*;
use ::fzy_cleaned::src::c_types::*;
use ::fzy_cleaned::src::c_extern_types::*;
#[allow(unused_imports)]
use ::fzy_cleaned;
use ::fzy_cleaned::src::choices::choices_available;
use ::fzy_cleaned::src::choices::choices_destroy;
use ::fzy_cleaned::src::choices::choices_fread;
use ::fzy_cleaned::src::choices::choices_get;
use ::fzy_cleaned::src::choices::choices_getscore;
use ::fzy_cleaned::src::choices::choices_init;
use ::fzy_cleaned::src::choices::choices_search;
use ::fzy_cleaned::src::options::options_parse;
use ::fzy_cleaned::src::tty::tty_getheight;
use ::fzy_cleaned::src::tty::tty_init;
use ::fzy_cleaned::src::tty_interface::tty_interface_init;
use ::fzy_cleaned::src::tty_interface::tty_interface_run;

pub use fzy_cleaned::src::tty_interface::tty_t;


pub use fzy_cleaned::src::tty_interface::tty_interface_t;


pub const STDIN_FILENO: c_int = 0 as c_int;
unsafe fn main_0(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) -> c_int {
    let mut ret: c_int = 0 as c_int;
    let mut options: options_t = options_t {
        benchmark: 0,
        filter: ::core::ptr::null::<c_char>(),
        init_search: ::core::ptr::null::<c_char>(),
        tty_filename: ::core::ptr::null::<c_char>(),
        show_scores: 0,
        num_lines: 0,
        scrolloff: 0,
        prompt: ::core::ptr::null::<c_char>(),
        workers: 0,
        input_delimiter: 0,
        show_info: 0,
    };
    options_parse(&raw mut options, argc, argv);
    let mut choices: choices_t = choices_t {
        buffer: ::core::ptr::null_mut::<c_char>(),
        buffer_size: 0,
        capacity: 0,
        size: 0,
        strings: ::core::ptr::null_mut::<*const c_char>(),
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
                    as *const c_char,
            );
            exit(EXIT_FAILURE);
        }
        choices_fread(&raw mut choices, stdin, options.input_delimiter);
        let mut i: c_int = 0 as c_int;
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
                    b"%f\t\0" as *const u8 as *const c_char,
                    choices_getscore(&raw mut choices, i_0),
                );
            }
            printf(
                b"%s\n\0" as *const u8 as *const c_char,
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
            options.num_lines = choices.size as c_uint;
        }
        let mut num_lines_adjustment: c_int = 1 as c_int;
        if options.show_info != 0 {
            num_lines_adjustment += 1;
        }
        if options
            .num_lines
            .wrapping_add(num_lines_adjustment as c_uint) as size_t
            > tty_getheight(&raw mut tty)
        {
            options.num_lines = tty_getheight(&raw mut tty)
                .wrapping_sub(num_lines_adjustment as size_t)
                as c_uint;
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
    let mut args_ptrs: Vec<*mut c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as c_int,
            args_ptrs.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}
