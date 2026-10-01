use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    fn getopt_long(
        ___argc: c_int,
        ___argv: *const *mut c_char,
        __shortopts: *const c_char,
        __longopts: *const option,
        __longind: *mut c_int,
    ) -> c_int;
    fn sscanf(
        __s: *const c_char,
        __format: *const c_char,
        ...
    ) -> c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

pub const no_argument: c_int = 0 as c_int;
pub const required_argument: c_int = 1 as c_int;
pub const optional_argument: c_int = 2 as c_int;

pub const DEFAULT_TTY: [c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [c_char; 9]>(*b"/dev/tty\0") };
pub const DEFAULT_PROMPT: [c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [c_char; 3]>(*b"> \0") };
pub const DEFAULT_NUM_LINES: c_int = 10 as c_int;
pub const DEFAULT_WORKERS: c_int = 0 as c_int;
pub const DEFAULT_SHOW_INFO: c_int = 0 as c_int;

static mut usage_str: *const c_char = b"Usage: fzy [OPTION]...\n -l, --lines=LINES        Specify how many lines of results to show (default 10)\n -p, --prompt=PROMPT      Input prompt (default '> ')\n -q, --query=QUERY        Use QUERY as the initial search string\n -e, --show-matches=QUERY Output the sorted matches of QUERY\n -t, --tty=TTY            Specify file to use as TTY device (default /dev/tty)\n -s, --show-scores        Show the scores of each match\n -0, --read-null          Read input delimited by ASCII NUL characters\n -j, --workers NUM        Use NUM workers for searching. (default is # of CPUs)\n -i, --show-info          Show selection info line\n -h, --help     Display this help and exit\n -v, --version  Output version information and exit\n\0"
    as *const u8 as *const c_char;
unsafe extern "C" fn usage(mut argv0: *const c_char) {
    fprintf(stderr, usage_str, argv0);
}
static mut longopts: [option; 13] = [
    option {
        name: b"show-matches\0" as *const u8 as *const c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'e' as i32,
    },
    option {
        name: b"query\0" as *const u8 as *const c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'q' as i32,
    },
    option {
        name: b"lines\0" as *const u8 as *const c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'l' as i32,
    },
    option {
        name: b"tty\0" as *const u8 as *const c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 't' as i32,
    },
    option {
        name: b"prompt\0" as *const u8 as *const c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'p' as i32,
    },
    option {
        name: b"show-scores\0" as *const u8 as *const c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 's' as i32,
    },
    option {
        name: b"read-null\0" as *const u8 as *const c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: '0' as i32,
    },
    option {
        name: b"version\0" as *const u8 as *const c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'v' as i32,
    },
    option {
        name: b"benchmark\0" as *const u8 as *const c_char,
        has_arg: optional_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'b' as i32,
    },
    option {
        name: b"workers\0" as *const u8 as *const c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'j' as i32,
    },
    option {
        name: b"show-info\0" as *const u8 as *const c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'i' as i32,
    },
    option {
        name: b"help\0" as *const u8 as *const c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 'h' as i32,
    },
    option {
        name: ::core::ptr::null::<c_char>(),
        has_arg: 0 as c_int,
        flag: ::core::ptr::null::<c_int>() as *mut c_int,
        val: 0 as c_int,
    },
];
#[no_mangle]
pub unsafe extern "C" fn options_init(mut options: *mut options_t) {
    (*options).benchmark = 0 as c_int;
    (*options).filter = ::core::ptr::null::<c_char>();
    (*options).init_search = ::core::ptr::null::<c_char>();
    (*options).show_scores = 0 as c_int;
    (*options).scrolloff = 1 as c_uint;
    (*options).tty_filename = DEFAULT_TTY.as_ptr();
    (*options).num_lines = DEFAULT_NUM_LINES as c_uint;
    (*options).prompt = DEFAULT_PROMPT.as_ptr();
    (*options).workers = DEFAULT_WORKERS as c_uint;
    (*options).input_delimiter = '\n' as i32 as c_char;
    (*options).show_info = DEFAULT_SHOW_INFO;
}
#[no_mangle]
pub unsafe extern "C" fn options_parse(
    mut options: *mut options_t,
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) {
    options_init(options);
    let mut c: c_int = 0;
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut c_char,
            b"vhs0e:q:l:t:p:j:i\0" as *const u8 as *const c_char,
            &raw mut longopts as *mut option,
            ::core::ptr::null_mut::<c_int>(),
        );
        if !(c != -(1 as c_int)) {
            break;
        }
        match c {
            118 => {
                printf(
                    b"%s 1.1 \xC2\xA9 2014-2025 John Hawthorn\n\0" as *const u8
                        as *const c_char,
                    *argv.offset(0 as c_int as isize),
                );
                exit(EXIT_SUCCESS);
            }
            115 => {
                (*options).show_scores = 1 as c_int;
            }
            48 => {
                (*options).input_delimiter = '\0' as i32 as c_char;
            }
            113 => {
                (*options).init_search = optarg;
            }
            101 => {
                (*options).filter = optarg;
            }
            98 => {
                if !optarg.is_null() {
                    if sscanf(
                        optarg,
                        b"%d\0" as *const u8 as *const c_char,
                        &raw mut (*options).benchmark,
                    ) != 1 as c_int
                    {
                        usage(*argv.offset(0 as c_int as isize));
                        exit(EXIT_FAILURE);
                    }
                } else {
                    (*options).benchmark = 100 as c_int;
                }
            }
            116 => {
                (*options).tty_filename = optarg;
            }
            112 => {
                (*options).prompt = optarg;
            }
            106 => {
                if sscanf(
                    optarg,
                    b"%u\0" as *const u8 as *const c_char,
                    &raw mut (*options).workers,
                ) != 1 as c_int
                {
                    usage(*argv.offset(0 as c_int as isize));
                    exit(EXIT_FAILURE);
                }
            }
            108 => {
                let mut l: c_int = 0;
                if strcmp(optarg, b"max\0" as *const u8 as *const c_char) == 0 {
                    l = INT_MAX;
                } else if sscanf(
                    optarg,
                    b"%d\0" as *const u8 as *const c_char,
                    &raw mut l,
                ) != 1 as c_int
                    || l < 3 as c_int
                {
                    fprintf(
                        stderr,
                        b"Invalid format for --lines: %s\n\0" as *const u8
                            as *const c_char,
                        optarg,
                    );
                    fprintf(
                        stderr,
                        b"Must be integer in range 3..\n\0" as *const u8
                            as *const c_char,
                    );
                    usage(*argv.offset(0 as c_int as isize));
                    exit(EXIT_FAILURE);
                }
                (*options).num_lines = l as c_uint;
            }
            105 => {
                (*options).show_info = 1 as c_int;
            }
            104 | _ => {
                usage(*argv.offset(0 as c_int as isize));
                exit(EXIT_SUCCESS);
            }
        }
    }
    if optind != argc {
        usage(*argv.offset(0 as c_int as isize));
        exit(EXIT_FAILURE);
    }
}
pub const __INT_MAX__: c_int = 2147483647 as c_int;
pub const INT_MAX: c_int = __INT_MAX__;
