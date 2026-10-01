extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_struct_def;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::core::ffi::c_char,
        __modes: ::core::ffi::c_int,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn abort() -> !;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn opng_initialize(options_0: *const opng_options, ui: *const opng_ui) -> ::core::ffi::c_int;
    fn opng_optimize(infile_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn opng_finalize() -> ::core::ffi::c_int;
    fn opng_strparse_rangeset_to_bitset(
        out_set: *mut opng_bitset_t,
        rangeset_str: *const ::core::ffi::c_char,
        mask_set: opng_bitset_t,
    ) -> ::core::ffi::c_int;
    fn png_get_libpng_ver(png_ptr: png_const_structrp) -> png_const_charp;
    fn zlibVersion() -> *const ::core::ffi::c_char;
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
pub type __int32_t = i32;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type va_list = __builtin_va_list;
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
pub type opng_bitset_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_options {
    pub backup: ::core::ffi::c_int,
    pub clobber: ::core::ffi::c_int,
    pub debug: ::core::ffi::c_int,
    pub fix: ::core::ffi::c_int,
    pub force: ::core::ffi::c_int,
    pub full: ::core::ffi::c_int,
    pub preserve: ::core::ffi::c_int,
    pub quiet: ::core::ffi::c_int,
    pub simulate: ::core::ffi::c_int,
    pub verbose: ::core::ffi::c_int,
    pub out_name: *const ::core::ffi::c_char,
    pub dir_name: *const ::core::ffi::c_char,
    pub log_name: *const ::core::ffi::c_char,
    pub interlace: ::core::ffi::c_int,
    pub nb: ::core::ffi::c_int,
    pub nc: ::core::ffi::c_int,
    pub np: ::core::ffi::c_int,
    pub nz: ::core::ffi::c_int,
    pub optim_level: ::core::ffi::c_int,
    pub compr_level_set: opng_bitset_t,
    pub mem_level_set: opng_bitset_t,
    pub strategy_set: opng_bitset_t,
    pub filter_set: opng_bitset_t,
    pub window_bits: ::core::ffi::c_int,
    pub snip: ::core::ffi::c_int,
    pub strip_all: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ui {
    pub printf_fn: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>,
    pub print_cntrl_fn: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    pub progress_fn: Option<unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_ulong) -> ()>,
    pub panic_fn: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>,
}
pub type png_const_charp = *const ::core::ffi::c_char;
pub type png_struct = png_struct_def;
pub type png_const_structrp = *const png_struct;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const OP_SHOW_VERSION: C2RustUnnamed_0 = 2;
pub const OP_SHOW_HELP: C2RustUnnamed_0 = 1;
pub const OP_RUN: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub help: ::core::ffi::c_int,
    pub version: ::core::ffi::c_int,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
#[inline]
unsafe extern "C" fn tolower(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const _IOLBF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BUFSIZ: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPNG_COMPR_LEVEL_SET_MASK: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
    << 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
pub const OPNG_MEM_LEVEL_SET_MASK: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
    << 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    - ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
pub const OPNG_STRATEGY_SET_MASK: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
    << 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    - ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
pub const OPNG_FILTER_SET_MASK: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
    << 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
    - ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int);
pub const OPNG_BITSET_EMPTY: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
static mut msg_intro: *const ::core::ffi::c_char =
    b"OptiPNG version 0.7.7\nCopyright (C) 2001-2017 Cosmin Truta and the Contributing Authors.\n\0"
        as *const u8 as *const ::core::ffi::c_char;
static mut msg_license: *const ::core::ffi::c_char = b"This program is open-source software. See LICENSE for more details.\n\nPortions of this software are based in part on the work of:\n  Jean-loup Gailly and Mark Adler (zlib)\n  Glenn Randers-Pehrson and the PNG Development Group (libpng)\n  Miyasaka Masaru (BMP support)\n  David Koblas (GIF support)\n\0"
    as *const u8 as *const ::core::ffi::c_char;
static mut msg_help_synopsis: *const ::core::ffi::c_char = b"Synopsis:\n    optipng [options] files ...\nFiles:\n    Image files of type: PNG, BMP, GIF, PNM or TIFF\n\0"
    as *const u8 as *const ::core::ffi::c_char;
static mut msg_help_basic_options: *const ::core::ffi::c_char = b"Basic options:\n    -?, -h, -help\tshow the extended help\n    -o <level>\t\toptimization level (0-7)\t\t[default: 2]\n    -v\t\t\trun in verbose mode / show copyright and version info\n\0"
    as *const u8 as *const ::core::ffi::c_char;
static mut msg_help_options: *const ::core::ffi::c_char = b"Basic options:\n    -?, -h, -help\tshow this help\n    -o <level>\t\toptimization level (0-7)\t\t[default: 2]\n    -v\t\t\trun in verbose mode / show copyright and version info\nGeneral options:\n    -backup, -keep\tkeep a backup of the modified files\n    -clobber\t\toverwrite existing files\n    -fix\t\tenable error recovery\n    -force\t\tenforce writing of a new output file\n    -preserve\t\tpreserve file attributes if possible\n    -quiet, -silent\trun in quiet mode\n    -simulate\t\trun in simulation mode\n    -out <file>\t\twrite output file to <file>\n    -dir <directory>\twrite output file(s) to <directory>\n    -log <file>\t\tlog messages to <file>\n    --\t\t\tstop option switch parsing\nOptimization options:\n    -f <filters>\tPNG delta filters (0-5)\t\t\t[default: 0,5]\n    -i <type>\t\tPNG interlace type (0-1)\n    -zc <levels>\tzlib compression levels (1-9)\t\t[default: 9]\n    -zm <levels>\tzlib memory levels (1-9)\t\t[default: 8]\n    -zs <strategies>\tzlib compression strategies (0-3)\t[default: 0-3]\n    -zw <size>\t\tzlib window size (256,512,1k,2k,4k,8k,16k,32k)\n    -full\t\tproduce a full report on IDAT (might reduce speed)\n    -nb\t\t\tno bit depth reduction\n    -nc\t\t\tno color type reduction\n    -np\t\t\tno palette reduction\n    -nx\t\t\tno reductions\n    -nz\t\t\tno IDAT recoding\nEditing options:\n    -snip\t\tcut one image out of multi-image or animation files\n    -strip <objects>\tstrip metadata objects (e.g. \"all\")\nOptimization levels:\n    -o0\t\t<=>\t-o1 -nx -nz\t\t\t\t(0 or 1 trials)\n    -o1\t\t<=>\t-zc9 -zm8 -zs0 -f0\t\t\t(1 trial)\n    \t\t(or...)\t-zc9 -zm8 -zs1 -f5\t\t\t(1 trial)\n    -o2\t\t<=>\t-zc9 -zm8 -zs0-3 -f0,5\t\t\t(8 trials)\n    -o3\t\t<=>\t-zc9 -zm8-9 -zs0-3 -f0,5\t\t(16 trials)\n    -o4\t\t<=>\t-zc9 -zm8 -zs0-3 -f0-5\t\t\t(24 trials)\n    -o5\t\t<=>\t-zc9 -zm8-9 -zs0-3 -f0-5\t\t(48 trials)\n    -o6\t\t<=>\t-zc1-9 -zm8 -zs0-3 -f0-5\t\t(120 trials)\n    -o7\t\t<=>\t-zc1-9 -zm8-9 -zs0-3 -f0-5\t\t(240 trials)\n    -o7 -zm1-9\t<=>\t-zc1-9 -zm1-9 -zs0-3 -f0-5\t\t(1080 trials)\nNotes:\n    The combination for -o1 is chosen heuristically.\n    Exhaustive combinations such as \"-o7 -zm1-9\" are not generally recommended.\n\0"
    as *const u8 as *const ::core::ffi::c_char;
static mut msg_help_examples: *const ::core::ffi::c_char = b"Examples:\n    optipng file.png\t\t\t\t\t\t(default speed)\n    optipng -o5 file.png\t\t\t\t\t(slow)\n    optipng -o7 file.png\t\t\t\t\t(very slow)\n\0"
    as *const u8 as *const ::core::ffi::c_char;
static mut msg_help_more: *const ::core::ffi::c_char =
    b"Type \"optipng -h\" for extended help.\n\0" as *const u8 as *const ::core::ffi::c_char;
static mut operation: C2RustUnnamed_0 = OP_RUN;
static mut local_options: C2RustUnnamed_1 = C2RustUnnamed_1 {
    help: 0,
    version: 0,
};
static mut options: opng_options = opng_options {
    backup: 0,
    clobber: 0,
    debug: 0,
    fix: 0,
    force: 0,
    full: 0,
    preserve: 0,
    quiet: 0,
    simulate: 0,
    verbose: 0,
    out_name: ::core::ptr::null::<::core::ffi::c_char>(),
    dir_name: ::core::ptr::null::<::core::ffi::c_char>(),
    log_name: ::core::ptr::null::<::core::ffi::c_char>(),
    interlace: 0,
    nb: 0,
    nc: 0,
    np: 0,
    nz: 0,
    optim_level: 0,
    compr_level_set: 0,
    mem_level_set: 0,
    strategy_set: 0,
    filter_set: 0,
    window_bits: 0,
    snip: 0,
    strip_all: 0,
};
static mut con_file: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut log_file: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut start_of_line: ::core::ffi::c_int = 0;
unsafe extern "C" fn error(mut fmt: *const ::core::ffi::c_char, mut args: ...) {
    let mut arg_ptr: ::core::ffi::VaListImpl;
    fprintf(
        stderr,
        b"** Error: \0" as *const u8 as *const ::core::ffi::c_char,
    );
    arg_ptr = args.clone();
    vfprintf(stderr, fmt, arg_ptr.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    exit(EXIT_FAILURE);
}
unsafe extern "C" fn panic(mut msg: *const ::core::ffi::c_char) {
    fprintf(
        stderr,
        b"\n** INTERNAL ERROR: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        msg,
    );
    fprintf(
        stderr,
        b"Please submit a defect report.\nhttp://optipng.sourceforge.net/\n\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    if options.debug != 0 {
        abort();
    } else {
        fprintf(
            stderr,
            b"The execution of this program has been terminated abnormally.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(70 as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn opng_strcasecmp(
    mut str1: *const ::core::ffi::c_char,
    mut str2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ch1: ::core::ffi::c_int = 0;
    let mut ch2: ::core::ffi::c_int = 0;
    loop {
        ch1 = ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let fresh0 = str1;
                    str1 = str1.offset(1);
                    let mut __c: ::core::ffi::c_int = *fresh0 as ::core::ffi::c_int;
                    __res =
                        (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                } else {
                    let fresh1 = str1;
                    str1 = str1.offset(1);
                    __res = tolower(*fresh1 as ::core::ffi::c_int);
                }
            } else {
                let fresh2 = str1;
                str1 = str1.offset(1);
                __res = *(*__ctype_tolower_loc()).offset(*fresh2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            }
            __res
        });
        ch2 = ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let fresh3 = str2;
                    str2 = str2.offset(1);
                    let mut __c: ::core::ffi::c_int = *fresh3 as ::core::ffi::c_int;
                    __res =
                        (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                } else {
                    let fresh4 = str2;
                    str2 = str2.offset(1);
                    __res = tolower(*fresh4 as ::core::ffi::c_int);
                }
            } else {
                let fresh5 = str2;
                str2 = str2.offset(1);
                __res = *(*__ctype_tolower_loc()).offset(*fresh5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            }
            __res
        });
        if ch1 != ch2 {
            return ch1 - ch2;
        }
        if ch1 == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn opng_strltrim(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    while *(*__ctype_b_loc()).offset(*str as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        str = str.offset(1);
    }
    return str as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn opng_strtail(
    mut str: *const ::core::ffi::c_char,
    mut num: size_t,
) -> *mut ::core::ffi::c_char {
    let mut len: size_t = 0;
    len = strlen(str);
    if len <= num {
        return str as *mut ::core::ffi::c_char;
    }
    return (str as *mut ::core::ffi::c_char)
        .offset(len as isize)
        .offset(-(num as isize));
}
unsafe extern "C" fn opng_strpbrk_digit(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    loop {
        if *str as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if *(*__ctype_b_loc()).offset(*str as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            return str as *mut ::core::ffi::c_char;
        }
        str = str.offset(1);
    }
}
unsafe extern "C" fn opng_str2ulong(
    mut out_val: *mut ::core::ffi::c_ulong,
    mut in_str: *const ::core::ffi::c_char,
    mut allow_multiplier: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut begin_ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end_ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut multiplier: ::core::ffi::c_ulong = 0;
    end_ptr = opng_strltrim(in_str);
    begin_ptr = end_ptr;
    if *begin_ptr as ::core::ffi::c_int >= '0' as i32
        && *begin_ptr as ::core::ffi::c_int <= '9' as i32
    {
        *out_val = strtoul(begin_ptr, &raw mut end_ptr, 10 as ::core::ffi::c_int);
    }
    if begin_ptr == end_ptr as *const ::core::ffi::c_char {
        *__errno_location() = EINVAL;
        *out_val = 0 as ::core::ffi::c_ulong;
        return -(1 as ::core::ffi::c_int);
    }
    if allow_multiplier != 0 {
        if *end_ptr as ::core::ffi::c_int == 'k' as i32
            || *end_ptr as ::core::ffi::c_int == 'K' as i32
        {
            end_ptr = end_ptr.offset(1);
            multiplier = 1024 as ::core::ffi::c_ulong;
        } else if *end_ptr as ::core::ffi::c_int == 'M' as i32 {
            end_ptr = end_ptr.offset(1);
            multiplier = (1024 as ::core::ffi::c_ulong).wrapping_mul(1024 as ::core::ffi::c_ulong);
        } else if *end_ptr as ::core::ffi::c_int == 'G' as i32 {
            end_ptr = end_ptr.offset(1);
            multiplier = (1024 as ::core::ffi::c_ulong)
                .wrapping_mul(1024 as ::core::ffi::c_ulong)
                .wrapping_mul(1024 as ::core::ffi::c_ulong);
        } else {
            multiplier = 1 as ::core::ffi::c_ulong;
        }
        if multiplier > 1 as ::core::ffi::c_ulong {
            if *out_val > ULONG_MAX.wrapping_div(multiplier) {
                *__errno_location() = ERANGE;
                *out_val = ULONG_MAX;
            } else {
                *out_val = (*out_val).wrapping_mul(multiplier);
            }
        }
    }
    if *opng_strltrim(end_ptr) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn err_option_arg(
    mut opt: *const ::core::ffi::c_char,
    mut opt_arg: *const ::core::ffi::c_char,
) {
    if opt_arg.is_null() || *opng_strltrim(opt_arg) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        error(
            b"Missing argument for option %s\0" as *const u8 as *const ::core::ffi::c_char,
            opt,
        );
    } else {
        error(
            b"Invalid argument for option %s: %s\0" as *const u8 as *const ::core::ffi::c_char,
            opt,
            opt_arg,
        );
    };
}
unsafe extern "C" fn check_num_option(
    mut opt: *const ::core::ffi::c_char,
    mut opt_arg: *const ::core::ffi::c_char,
    mut lowest: ::core::ffi::c_int,
    mut highest: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_ulong = 0;
    if opng_str2ulong(&raw mut value, opt_arg, 0 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int
        || value > INT_MAX as ::core::ffi::c_ulong
        || (value as ::core::ffi::c_int) < lowest
        || value as ::core::ffi::c_int > highest
    {
        err_option_arg(opt, opt_arg);
    }
    return value as ::core::ffi::c_int;
}
unsafe extern "C" fn check_power2_option(
    mut opt: *const ::core::ffi::c_char,
    mut opt_arg: *const ::core::ffi::c_char,
    mut lowest: ::core::ffi::c_int,
    mut highest: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_ulong = 0;
    let mut result: ::core::ffi::c_int = 0;
    if opng_str2ulong(&raw mut value, opt_arg, 1 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        if lowest < 0 as ::core::ffi::c_int {
            lowest = 0 as ::core::ffi::c_int;
        }
        if highest
            > (CHAR_BIT as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
                .wrapping_sub(2 as usize) as ::core::ffi::c_int
        {
            highest = (CHAR_BIT as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as usize)
                .wrapping_sub(2 as usize) as ::core::ffi::c_int;
        }
        result = lowest;
        while result <= highest {
            if (1 as ::core::ffi::c_ulong) << result == value {
                return result;
            }
            result += 1;
        }
    }
    err_option_arg(opt, opt_arg);
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn check_rangeset_option(
    mut opt: *const ::core::ffi::c_char,
    mut opt_arg: *const ::core::ffi::c_char,
    mut result_mask: opng_bitset_t,
) -> opng_bitset_t {
    let mut result: opng_bitset_t = 0;
    if opng_strparse_rangeset_to_bitset(&raw mut result, opt_arg, result_mask)
        != 0 as ::core::ffi::c_int
    {
        result = OPNG_BITSET_EMPTY as opng_bitset_t;
    }
    if result & result_mask != result {
        result = OPNG_BITSET_EMPTY as opng_bitset_t;
    }
    if result == OPNG_BITSET_EMPTY {
        err_option_arg(opt, opt_arg);
    }
    return result;
}
unsafe extern "C" fn check_obj_option(
    mut opt: *const ::core::ffi::c_char,
    mut opt_arg: *const ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_uint = 0;
    if strcmp(b"all\0" as *const u8 as *const ::core::ffi::c_char, opt_arg)
        == 0 as ::core::ffi::c_int
    {
        return;
    }
    i = 0 as ::core::ffi::c_uint;
    while i < 4 as ::core::ffi::c_uint {
        if !(*opt_arg.offset(i as isize) as ::core::ffi::c_int >= 'A' as i32
            && *opt_arg.offset(i as isize) as ::core::ffi::c_int <= 'Z' as i32
            || *opt_arg.offset(i as isize) as ::core::ffi::c_int >= 'a' as i32
                && *opt_arg.offset(i as isize) as ::core::ffi::c_int <= 'z' as i32)
        {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == 4 as ::core::ffi::c_uint
        && *opt_arg.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        error(
            b"Manipulation of individual chunks is not implemented\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        err_option_arg(opt, opt_arg);
    };
}
unsafe extern "C" fn scan_option(
    mut str: *const ::core::ffi::c_char,
    mut opt_buf: *mut ::core::ffi::c_char,
    mut opt_buf_size: size_t,
    mut opt_arg_ptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut opt_len: ::core::ffi::c_uint = 0;
    if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32
        || *str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    opt_len = 0 as ::core::ffi::c_uint;
    ptr = str.offset(1 as ::core::ffi::c_int as isize);
    while *ptr as ::core::ffi::c_int == '-' as i32 {
        ptr = ptr.offset(1);
    }
    if *ptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        ptr = ptr.offset(-1);
    }
    loop {
        if (opt_len as size_t) < opt_buf_size {
            *opt_buf.offset(opt_len as isize) = ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<::core::ffi::c_char>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: ::core::ffi::c_int = *ptr as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = tolower(*ptr as ::core::ffi::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*ptr as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int;
                }
                __res
            }) as ::core::ffi::c_char;
        }
        opt_len = opt_len.wrapping_add(1);
        ptr = ptr.offset(1);
        if *ptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *(*__ctype_b_loc()).offset(*ptr as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
        {
            while *(*__ctype_b_loc()).offset(*ptr as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
            {
                ptr = ptr.offset(1);
            }
            *opt_arg_ptr = if *ptr as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                ptr
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            };
            break;
        } else {
            if !(*ptr as ::core::ffi::c_int == '=' as i32) {
                continue;
            }
            ptr = ptr.offset(1);
            *opt_arg_ptr = ptr;
            break;
        }
    }
    if opt_buf_size > 0 as size_t {
        if (opt_len as size_t) < opt_buf_size {
            *opt_buf.offset(opt_len as isize) = '\0' as i32 as ::core::ffi::c_char;
        } else {
            *opt_buf.offset(opt_buf_size.wrapping_sub(1 as size_t) as isize) =
                '\0' as i32 as ::core::ffi::c_char;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_args(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut arg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut opt: [::core::ffi::c_char; 16] = [0; 16];
    let mut opt_len: size_t = 0;
    let mut xopt: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut simple_opt: ::core::ffi::c_int = 0;
    let mut stop_switch: ::core::ffi::c_int = 0;
    let mut set: opng_bitset_t = 0;
    let mut val: ::core::ffi::c_int = 0;
    let mut file_count: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_int = 0;
    memset(
        &raw mut options as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<opng_options>() as size_t,
    );
    options.optim_level = -(1 as ::core::ffi::c_int);
    options.interlace = -(1 as ::core::ffi::c_int);
    file_count = 0 as ::core::ffi::c_uint;
    stop_switch = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < argc {
        arg = *argv.offset(i as isize);
        if stop_switch != 0
            || scan_option(
                arg,
                &raw mut opt as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
                &raw mut xopt,
            ) < 1 as ::core::ffi::c_int
        {
            file_count = file_count.wrapping_add(1);
        } else {
            opt_len = strlen(&raw mut opt as *mut ::core::ffi::c_char);
            let ref mut fresh6 = *argv.offset(i as isize);
            *fresh6 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !strchr(
                b"fio\0" as *const u8 as *const ::core::ffi::c_char,
                opt[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            )
            .is_null()
                && *(*__ctype_b_loc())
                    .offset(opt[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
                || opt[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 'z' as i32
                    && *(*__ctype_b_loc()).offset(
                        opt[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                        & _ISalpha as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0
                    && *(*__ctype_b_loc()).offset(
                        opt[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0
            {
                opt_len = opng_strpbrk_digit(&raw mut opt as *mut ::core::ffi::c_char)
                    .offset_from(&raw mut opt as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as size_t;
                opt[opt_len as usize] = '\0' as i32 as ::core::ffi::c_char;
                xopt = opng_strpbrk_digit(arg);
            }
            simple_opt = 1 as ::core::ffi::c_int;
            if strcmp(
                b"-\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                stop_switch = 1 as ::core::ffi::c_int;
            } else if strcmp(
                b"?\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strncmp(
                    b"help\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut opt as *mut ::core::ffi::c_char,
                    opt_len,
                ) == 0 as ::core::ffi::c_int
            {
                local_options.help = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"backup\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                || strncmp(
                    b"keep\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut opt as *mut ::core::ffi::c_char,
                    opt_len,
                ) == 0 as ::core::ffi::c_int
            {
                options.backup = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"clobber\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
            {
                options.clobber = 1 as ::core::ffi::c_int;
            } else if strcmp(
                b"debug\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                options.debug = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"fix\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 2 as size_t
            {
                options.fix = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"force\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 2 as size_t
            {
                options.force = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"full\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 2 as size_t
            {
                options.full = 1 as ::core::ffi::c_int;
            } else if strcmp(
                b"nb\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                options.nb = 1 as ::core::ffi::c_int;
            } else if strcmp(
                b"nc\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                options.nc = 1 as ::core::ffi::c_int;
            } else if strcmp(
                b"np\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                options.np = 1 as ::core::ffi::c_int;
            } else if strcmp(
                b"nx\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                options.np = 1 as ::core::ffi::c_int;
                options.nc = options.np;
                options.nb = options.nc;
            } else if strcmp(
                b"nz\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                options.nz = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"preserve\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
            {
                options.preserve = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"quiet\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                || strncmp(
                    b"silent\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut opt as *mut ::core::ffi::c_char,
                    opt_len,
                ) == 0 as ::core::ffi::c_int
                    && opt_len >= 3 as size_t
            {
                options.quiet = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"simulate\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 3 as size_t
            {
                options.simulate = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"snip\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 2 as size_t
            {
                options.snip = 1 as ::core::ffi::c_int;
            } else if strcmp(
                b"v\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                options.verbose = 1 as ::core::ffi::c_int;
                local_options.version = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 4 as size_t
            {
                options.verbose = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 4 as size_t
            {
                local_options.version = 1 as ::core::ffi::c_int;
            } else {
                simple_opt = 0 as ::core::ffi::c_int;
                if xopt.is_null() {
                    i += 1;
                    if i < argc {
                        xopt = *argv.offset(i as isize);
                        let ref mut fresh7 = *argv.offset(i as isize);
                        *fresh7 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    } else {
                        xopt = b"\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
            }
            if simple_opt != 0 {
                if !xopt.is_null() {
                    error(
                        b"No argument allowed for option: %s\0" as *const u8
                            as *const ::core::ffi::c_char,
                        arg,
                    );
                }
            } else if strcmp(
                b"o\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                val = check_num_option(
                    b"-o\0" as *const u8 as *const ::core::ffi::c_char,
                    xopt,
                    0 as ::core::ffi::c_int,
                    INT_MAX,
                );
                if options.optim_level < 0 as ::core::ffi::c_int {
                    options.optim_level = val;
                } else if options.optim_level != val {
                    error(
                        b"Multiple optimization levels are not permitted\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(
                b"i\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                val = check_num_option(
                    b"-i\0" as *const u8 as *const ::core::ffi::c_char,
                    xopt,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                if options.interlace < 0 as ::core::ffi::c_int {
                    options.interlace = val;
                } else if options.interlace != val {
                    error(
                        b"Multiple interlace types are not permitted\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(
                b"f\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                set = check_rangeset_option(
                    b"-f\0" as *const u8 as *const ::core::ffi::c_char,
                    xopt,
                    OPNG_FILTER_SET_MASK as opng_bitset_t,
                );
                options.filter_set |= set as ::core::ffi::c_uint;
            } else if strcmp(
                b"zc\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                set = check_rangeset_option(
                    b"-zc\0" as *const u8 as *const ::core::ffi::c_char,
                    xopt,
                    OPNG_COMPR_LEVEL_SET_MASK as opng_bitset_t,
                );
                options.compr_level_set |= set as ::core::ffi::c_uint;
            } else if strcmp(
                b"zm\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                set = check_rangeset_option(
                    b"-zm\0" as *const u8 as *const ::core::ffi::c_char,
                    xopt,
                    OPNG_MEM_LEVEL_SET_MASK as opng_bitset_t,
                );
                options.mem_level_set |= set as ::core::ffi::c_uint;
            } else if strcmp(
                b"zs\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                set = check_rangeset_option(
                    b"-zs\0" as *const u8 as *const ::core::ffi::c_char,
                    xopt,
                    OPNG_STRATEGY_SET_MASK as opng_bitset_t,
                );
                options.strategy_set |= set as ::core::ffi::c_uint;
            } else if strcmp(
                b"zw\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                val = check_power2_option(
                    b"-zw\0" as *const u8 as *const ::core::ffi::c_char,
                    xopt,
                    8 as ::core::ffi::c_int,
                    15 as ::core::ffi::c_int,
                );
                if options.window_bits == 0 as ::core::ffi::c_int {
                    options.window_bits = val;
                } else if options.window_bits != val {
                    error(
                        b"Multiple window sizes are not permitted\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if strncmp(
                b"strip\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 2 as size_t
            {
                check_obj_option(b"-strip\0" as *const u8 as *const ::core::ffi::c_char, xopt);
                options.strip_all = 1 as ::core::ffi::c_int;
            } else if strncmp(
                b"out\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
                && opt_len >= 2 as size_t
            {
                if !options.out_name.is_null() {
                    error(
                        b"Multiple output file names are not permitted\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if *xopt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    err_option_arg(
                        b"-out\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
                options.out_name = xopt;
            } else if strncmp(
                b"dir\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
            {
                if !options.dir_name.is_null() {
                    error(
                        b"Multiple output dir names are not permitted\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if *xopt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    err_option_arg(
                        b"-dir\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
                options.dir_name = xopt;
            } else if strncmp(
                b"log\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut opt as *mut ::core::ffi::c_char,
                opt_len,
            ) == 0 as ::core::ffi::c_int
            {
                if !options.log_name.is_null() {
                    error(
                        b"Multiple log file names are not permitted\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if *xopt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    err_option_arg(
                        b"-log\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
                options.log_name = xopt;
            } else {
                error(
                    b"Unrecognized option: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    arg,
                );
            }
        }
        i += 1;
    }
    if !options.out_name.is_null() {
        if file_count > 1 as ::core::ffi::c_uint {
            error(
                b"The option -out requires one input file\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if !options.dir_name.is_null() {
            error(
                b"The options -out and -dir are mutually exclusive\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if !options.log_name.is_null() {
        if opng_strcasecmp(
            b".log\0" as *const u8 as *const ::core::ffi::c_char,
            opng_strtail(options.log_name, 4 as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            error(
                b"To prevent accidental data corruption, the log file name must end with \".log\"\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if local_options.help != 0 {
        operation = OP_SHOW_HELP;
    } else if file_count != 0 as ::core::ffi::c_uint {
        operation = OP_RUN;
    } else if local_options.version != 0 {
        operation = OP_SHOW_VERSION;
    } else {
        operation = OP_SHOW_HELP;
    };
}
unsafe extern "C" fn app_printf(mut fmt: *const ::core::ffi::c_char, mut args: ...) {
    let mut arg_ptr: ::core::ffi::VaListImpl;
    if *fmt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return;
    }
    start_of_line = if *fmt.offset(strlen(fmt).wrapping_sub(1 as size_t) as isize)
        as ::core::ffi::c_int
        == '\n' as i32
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if !con_file.is_null() {
        arg_ptr = args.clone();
        vfprintf(con_file, fmt, arg_ptr.as_va_list());
    }
    if !log_file.is_null() {
        arg_ptr = args.clone();
        vfprintf(log_file, fmt, arg_ptr.as_va_list());
    }
}
unsafe extern "C" fn app_print_cntrl(mut cntrl_code: ::core::ffi::c_int) {
    let mut con_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut log_str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if cntrl_code == '\r' as i32 {
        con_str = b"\r\0" as *const u8 as *const ::core::ffi::c_char;
        log_str = b"\n\0" as *const u8 as *const ::core::ffi::c_char;
        start_of_line = 1 as ::core::ffi::c_int;
    } else if cntrl_code == '\u{b}' as i32 {
        if start_of_line == 0 {
            log_str = b"\n\0" as *const u8 as *const ::core::ffi::c_char;
            con_str = log_str;
            start_of_line = 1 as ::core::ffi::c_int;
        } else {
            log_str = b"\0" as *const u8 as *const ::core::ffi::c_char;
            con_str = log_str;
        }
    } else if cntrl_code < 0 as ::core::ffi::c_int
        && cntrl_code > -(80 as ::core::ffi::c_int)
        && start_of_line != 0
    {
        if !con_file.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i > cntrl_code {
                fputc(' ' as i32, con_file);
                i -= 1;
            }
        }
        con_str = b"\r\0" as *const u8 as *const ::core::ffi::c_char;
        log_str = b"\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        log_str = b"<?>\0" as *const u8 as *const ::core::ffi::c_char;
        con_str = log_str;
    }
    if !con_file.is_null() {
        fputs(con_str, con_file);
    }
    if !log_file.is_null() {
        fputs(log_str, log_file);
    }
}
unsafe extern "C" fn app_progress(
    mut current_step: ::core::ffi::c_ulong,
    mut total_steps: ::core::ffi::c_ulong,
) {
    if !con_file.is_null() {
        fflush(con_file);
    }
    if current_step != 0 && total_steps != 0 {
        return;
    }
}
unsafe extern "C" fn app_init() {
    start_of_line = 1 as ::core::ffi::c_int;
    if operation as ::core::ffi::c_uint == OP_SHOW_HELP as ::core::ffi::c_int as ::core::ffi::c_uint
        || operation as ::core::ffi::c_uint
            == OP_SHOW_VERSION as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con_file = stdout;
    } else if options.quiet == 0 {
        con_file = stderr;
    } else {
        con_file = ::core::ptr::null_mut::<FILE>();
    }
    if !options.log_name.is_null() {
        log_file = fopen(
            options.log_name,
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if log_file.is_null() {
            error(
                b"Can't open log file: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                options.log_name,
            );
        }
        setvbuf(
            log_file,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            _IOLBF,
            BUFSIZ as size_t,
        );
        app_printf(
            b"** Warning: %s\n\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"The option -log is deprecated; use shell redirection\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn app_finish() {
    if !log_file.is_null() {
        fclose(log_file);
    }
}
unsafe extern "C" fn process_files(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    let mut ui: opng_ui = opng_ui {
        printf_fn: None,
        print_cntrl_fn: None,
        progress_fn: None,
        panic_fn: None,
    };
    let mut i: ::core::ffi::c_int = 0;
    ui.printf_fn = Some(app_printf as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
        as Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>;
    ui.print_cntrl_fn = Some(app_print_cntrl as unsafe extern "C" fn(::core::ffi::c_int) -> ())
        as Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
    ui.progress_fn = Some(
        app_progress as unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_ulong) -> (),
    )
        as Option<unsafe extern "C" fn(::core::ffi::c_ulong, ::core::ffi::c_ulong) -> ()>;
    ui.panic_fn = Some(panic as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ())
        as Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>;
    if opng_initialize(&raw mut options, &raw mut ui) != 0 as ::core::ffi::c_int {
        panic(b"Can't initialize optimization engine\0" as *const u8 as *const ::core::ffi::c_char);
    }
    result = EXIT_SUCCESS;
    i = 1 as ::core::ffi::c_int;
    while i < argc {
        if !((*argv.offset(i as isize)).is_null()
            || *(*argv.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int)
        {
            if opng_optimize(*argv.offset(i as isize)) != 0 as ::core::ffi::c_int {
                result = EXIT_FAILURE;
            }
        }
        i += 1;
    }
    if opng_finalize() != 0 as ::core::ffi::c_int {
        panic(b"Can't finalize optimization engine\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return result;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    parse_args(argc, argv);
    app_init();
    result = EXIT_SUCCESS;
    if local_options.version != 0 {
        app_printf(
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            msg_intro,
        );
    }
    match operation as ::core::ffi::c_uint {
        0 => {
            result = process_files(argc, argv);
        }
        1 => {
            if local_options.help != 0 {
                app_printf(
                    b"%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg_help_synopsis,
                    msg_help_options,
                    msg_help_examples,
                );
            } else {
                app_printf(
                    b"%s%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    msg_help_synopsis,
                    msg_help_basic_options,
                    msg_help_examples,
                    msg_help_more,
                );
            }
        }
        2 => {
            app_printf(
                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                msg_license,
            );
            app_printf(
                b"Using libpng version %s and zlib version %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                png_get_libpng_ver(::core::ptr::null::<png_struct>()),
                zlibVersion(),
            );
        }
        _ => {
            result = -(1 as ::core::ffi::c_int);
        }
    }
    app_finish();
    return result;
}
pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
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
