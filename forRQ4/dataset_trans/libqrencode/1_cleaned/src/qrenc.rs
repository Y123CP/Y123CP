#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
use core::ffi::*;
use ::libqrencode_cleaned::src::ffi::*;
use ::libqrencode_cleaned::src::c_consts::*;
use ::libqrencode_cleaned::src::c_structs::*;
use ::libqrencode_cleaned::src::c_types::*;
#[allow(unused_imports)]
use ::libqrencode_cleaned;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> c_int;
    fn fopen(
        __filename: *const c_char,
        __modes: *const c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const c_char,
        ...
    ) -> c_int;
    fn sscanf(
        __s: *const c_char,
        __format: *const c_char,
        ...
    ) -> c_int;
    fn fputc(__c: c_int, __stream: *mut FILE) -> c_int;
    fn fputs(__s: *const c_char, __stream: *mut FILE) -> c_int;
    fn fread(
        __ptr: *mut c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> c_ulong;
    fn feof(__stream: *mut FILE) -> c_int;
    fn strcasecmp(
        __s1: *const c_char,
        __s2: *const c_char,
    ) -> c_int;
    fn getopt_long(
        ___argc: c_int,
        ___argv: *const *mut c_char,
        __shortopts: *const c_char,
        __longopts: *const option,
        __longind: *mut c_int,
    ) -> c_int;
    fn QRcode_encodeString(
        string: *const c_char,
        version_0: c_int,
        level_0: QRecLevel,
        hint_0: QRencodeMode,
        casesensitive_0: c_int,
    ) -> *mut QRcode;
    fn QRcode_encodeStringMQR(
        string: *const c_char,
        version_0: c_int,
        level_0: QRecLevel,
        hint_0: QRencodeMode,
        casesensitive_0: c_int,
    ) -> *mut QRcode;
    fn QRcode_encodeData(
        size_0: c_int,
        data: *const c_uchar,
        version_0: c_int,
        level_0: QRecLevel,
    ) -> *mut QRcode;
    fn QRcode_encodeDataMQR(
        size_0: c_int,
        data: *const c_uchar,
        version_0: c_int,
        level_0: QRecLevel,
    ) -> *mut QRcode;
    fn QRcode_free(qrcode: *mut QRcode);
    fn QRcode_encodeStringStructured(
        string: *const c_char,
        version_0: c_int,
        level_0: QRecLevel,
        hint_0: QRencodeMode,
        casesensitive_0: c_int,
    ) -> *mut QRcode_List;
    fn QRcode_encodeDataStructured(
        size_0: c_int,
        data: *const c_uchar,
        version_0: c_int,
        level_0: QRecLevel,
    ) -> *mut QRcode_List;
    fn QRcode_List_free(qrlist: *mut QRcode_List);
    fn QRcode_APIVersionString() -> *mut c_char;
}

pub type __off_t = c_long;
pub type __off64_t = c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: __off_t,
    pub _cur_column: c_ushort,
    pub _vtable_offset: c_schar,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub __pad5: size_t,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

pub type imageType = c_uint;
pub const ANSIUTF8i_TYPE: imageType = 13;
pub const UTF8i_TYPE: imageType = 12;
pub const ANSI256UTF8_TYPE: imageType = 11;
pub const ANSIUTF8_TYPE: imageType = 10;
pub const UTF8_TYPE: imageType = 9;
pub const ASCIIi_TYPE: imageType = 8;
pub const ASCII_TYPE: imageType = 7;
pub const ANSI256_TYPE: imageType = 6;
pub const ANSI_TYPE: imageType = 5;
pub const XPM_TYPE: imageType = 4;
pub const SVG_TYPE: imageType = 3;
pub const EPS_TYPE: imageType = 2;
pub const PNG32_TYPE: imageType = 1;
pub const PNG_TYPE: imageType = 0;

pub const NULL_0: *mut c_void = ::core::ptr::null_mut::<c_void>();
pub const FILENAME_MAX: c_int = 4096 as c_int;
pub const EXIT_FAILURE: c_int = 1 as c_int;
pub const EXIT_SUCCESS: c_int = 0 as c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const c_char) -> c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut c_char,
        10 as c_int,
    ) as c_int;
}

pub const no_argument: c_int = 0 as c_int;
pub const required_argument: c_int = 1 as c_int;

pub const INCHES_PER_METER: c_double = 100.0f64 / 2.54f64;
static mut casesensitive: c_int = 1 as c_int;
static mut eightbit: c_int = 0 as c_int;
static mut version: c_int = 0 as c_int;
static mut size: c_int = 3 as c_int;
static mut margin: c_int = -(1 as c_int);
static mut dpi: c_int = 72 as c_int;
static mut structured: c_int = 0 as c_int;
static mut rle: c_int = 0 as c_int;
static mut svg_path: c_int = 0 as c_int;
static mut micro: c_int = 0 as c_int;
static mut inline_svg: c_int = 0 as c_int;
static mut strict_versioning: c_int = 0 as c_int;
static mut level: QRecLevel = QR_ECLEVEL_L;
static mut hint: QRencodeMode = QR_MODE_8;
static mut fg_color: [c_uchar; 4] = [
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    255 as c_int as c_uchar,
];
static mut bg_color: [c_uchar; 4] = [
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
    255 as c_int as c_uchar,
];
static mut verbose: c_int = 0 as c_int;
static mut image_type: imageType = PNG_TYPE;
static mut options: [option; 24] = unsafe {
    [
        option {
            name: b"help\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'h' as i32,
        },
        option {
            name: b"output\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'o' as i32,
        },
        option {
            name: b"read-from\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'r' as i32,
        },
        option {
            name: b"level\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'l' as i32,
        },
        option {
            name: b"size\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 's' as i32,
        },
        option {
            name: b"symversion\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'v' as i32,
        },
        option {
            name: b"margin\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'm' as i32,
        },
        option {
            name: b"dpi\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'd' as i32,
        },
        option {
            name: b"type\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 't' as i32,
        },
        option {
            name: b"structured\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'S' as i32,
        },
        option {
            name: b"kanji\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'k' as i32,
        },
        option {
            name: b"casesensitive\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'c' as i32,
        },
        option {
            name: b"ignorecase\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'i' as i32,
        },
        option {
            name: b"8bit\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: '8' as i32,
        },
        option {
            name: b"micro\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'M' as i32,
        },
        option {
            name: b"rle\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: &raw const rle as *mut c_int,
            val: 1 as c_int,
        },
        option {
            name: b"svg-path\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: &raw const svg_path as *mut c_int,
            val: 1 as c_int,
        },
        option {
            name: b"inline\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: &raw const inline_svg as *mut c_int,
            val: 1 as c_int,
        },
        option {
            name: b"strict-version\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: &raw const strict_versioning as *mut c_int,
            val: 1 as c_int,
        },
        option {
            name: b"foreground\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'f' as i32,
        },
        option {
            name: b"background\0" as *const u8 as *const c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'b' as i32,
        },
        option {
            name: b"version\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 'V' as i32,
        },
        option {
            name: b"verbose\0" as *const u8 as *const c_char,
            has_arg: no_argument,
            flag: &raw const verbose as *mut c_int,
            val: 1 as c_int,
        },
        option {
            name: ::core::ptr::null::<c_char>(),
            has_arg: 0 as c_int,
            flag: ::core::ptr::null::<c_int>() as *mut c_int,
            val: 0 as c_int,
        },
    ]
};
static mut optstring: *mut c_char = b"ho:r:l:s:v:m:d:t:Skci8MV\0" as *const u8
    as *const c_char
    as *mut c_char;
unsafe extern "C" fn usage(
    mut help: c_int,
    mut longopt: c_int,
    mut status: c_int,
) {
    let mut out: *mut FILE = if status != 0 { stderr } else { stdout };
    fprintf(
        out,
        b"qrencode version %s\nCopyright (C) 2006-2017 Kentaro Fukuchi\n\0" as *const u8
            as *const c_char,
        QRcode_APIVersionString(),
    );
    if help != 0 {
        if longopt != 0 {
            fprintf(
                out,
                b"Usage: qrencode [-o FILENAME] [OPTION]... [STRING]\nEncode input data in a QR Code and save as a PNG or EPS image.\n\n  -h, --help   display the help message. -h displays only the help of short\n               options.\n\n  -o FILENAME, --output=FILENAME\n               write image to FILENAME. If '-' is specified, the result\n               will be output to standard output. If -S is given, structured\n               symbols are written to FILENAME-01.png, FILENAME-02.png, ...\n               (suffix is removed from FILENAME, if specified)\n\n  -r FILENAME, --read-from=FILENAME\n               read input data from FILENAME.\n\n  -s NUMBER, --size=NUMBER\n               specify module size in dots (pixels). (default=3)\n\n  -l {LMQH}, --level={LMQH}\n               specify error correction level from L (lowest) to H (highest).\n               (default=L)\n\n  -v NUMBER, --symversion=NUMBER\n               specify the minimum version of the symbol. See SYMBOL VERSIONS\n               for more information. (default=auto)\n\n  -m NUMBER, --margin=NUMBER\n               specify the width of the margins. (default=4 (2 for Micro QR)))\n\n  -d NUMBER, --dpi=NUMBER\n               specify the DPI of the generated PNG. (default=72)\n\n  -t {PNG,PNG32,EPS,SVG,XPM,ANSI,ANSI256,ASCII,ASCIIi,UTF8,UTF8i,ANSIUTF8,ANSIUTF8i,ANSI256UTF8},\n  --type={PNG,PNG32,EPS,SVG,XPM,ANSI,ANSI256,ASCII,ASCIIi,UTF8,UTF8i,ANSIUTF8,ANSIUTF8i,ANSI256UTF8}\n               specify the type of the generated image. (default=PNG)\n\n  -S, --structured\n               make structured symbols. Version must be specified with '-v'.\n\n  -k, --kanji  assume that the input text contains kanji (shift-jis).\n\n  -c, --casesensitive\n               encode lower-case alphabet characters in 8-bit mode. (default)\n\n  -i, --ignorecase\n               ignore case distinctions and use only upper-case characters.\n\n  -8, --8bit   encode entire data in 8-bit mode. -k, -c and -i will be ignored.\n\n  -M, --micro  encode in a Micro QR Code.\n\n      --rle    enable run-length encoding for SVG.\n\n      --svg-path\n               use single path to draw modules for SVG.\n\n      --inline only useful for SVG output, generates an SVG without the XML tag.\n\n      --foreground=RRGGBB[AA]\n      --background=RRGGBB[AA]\n               specify foreground/background color in hexadecimal notation.\n               6-digit (RGB) or 8-digit (RGBA) form are supported.\n               Color output support available only in PNG, EPS and SVG.\n\n      --strict-version\n               disable automatic version number adjustment. If the input data is\n               too large for the specified version, the program exits with the\n               code of 1.\n\n  -V, --version\n               display the version number and copyrights of the qrencode.\n\n      --verbose\n               display verbose information to stderr.\n\n  [STRING]     input data. If it is not specified, data will be taken from\n               standard input.\n\nSYMBOL VERSIONS\n               The symbol versions of QR Code range from Version 1 to Version\n               40. Each version has a different module configuration or number\n               of modules, ranging from Version 1 (21 x 21 modules) up to\n               Version 40 (177 x 177 modules). Each higher version number\n               comprises 4 additional modules per side by default. See\n               http://www.qrcode.com/en/about/version.html for a detailed\n               version list.\n\0"
                    as *const u8 as *const c_char,
            );
        } else {
            fprintf(
                out,
                b"Usage: qrencode [-o FILENAME] [OPTION]... [STRING]\nEncode input data in a QR Code and save as a PNG or EPS image.\n\n  -h           display this message.\n  --help       display the usage of long options.\n  -o FILENAME  write image to FILENAME. If '-' is specified, the result\n               will be output to standard output. If -S is given, structured\n               symbols are written to FILENAME-01.png, FILENAME-02.png, ...\n               (suffix is removed from FILENAME, if specified)\n  -r FILENAME  read input data from FILENAME.\n  -s NUMBER    specify module size in dots (pixels). (default=3)\n  -l {LMQH}    specify error correction level from L (lowest) to H (highest).\n               (default=L)\n  -v NUMBER    specify the minimum version of the symbol. (default=auto)\n  -m NUMBER    specify the width of the margins. (default=4 (2 for Micro))\n  -d NUMBER    specify the DPI of the generated PNG. (default=72)\n  -t {PNG,PNG32,EPS,SVG,XPM,ANSI,ANSI256,ASCII,ASCIIi,UTF8,UTF8i,ANSIUTF8,ANSIUTF8i,ANSI256UTF8}\n               specify the type of the generated image. (default=PNG)\n  -S           make structured symbols. Version number must be specified with '-v'.\n  -k           assume that the input text contains kanji (shift-jis).\n  -c           encode lower-case alphabet characters in 8-bit mode. (default)\n  -i           ignore case distinctions and use only upper-case characters.\n  -8           encode entire data in 8-bit mode. -k, -c and -i will be ignored.\n  -M           encode in a Micro QR Code.\n  -V           display the version number and copyrights of the qrencode.\n  [STRING]     input data. If it is not specified, data will be taken from\n               standard input.\n\n  Try \"qrencode --help\" for more options.\n\0"
                    as *const u8 as *const c_char,
            );
        }
    }
}
unsafe extern "C" fn color_set(
    mut color: *mut c_uchar,
    mut value: *const c_char,
) -> c_int {
    let mut len: c_int = strlen(value) as c_int;
    let mut i: c_int = 0;
    let mut count: c_int = 0;
    let mut col: [c_uint; 4] = [0; 4];
    if len == 6 as c_int {
        count = sscanf(
            value,
            b"%02x%02x%02x%n\0" as *const u8 as *const c_char,
            (&raw mut col as *mut c_uint).offset(0 as c_int as isize)
                as *mut c_uint,
            (&raw mut col as *mut c_uint).offset(1 as c_int as isize)
                as *mut c_uint,
            (&raw mut col as *mut c_uint).offset(2 as c_int as isize)
                as *mut c_uint,
            &raw mut len,
        );
        if count < 3 as c_int || len != 6 as c_int {
            return -(1 as c_int);
        }
        i = 0 as c_int;
        while i < 3 as c_int {
            *color.offset(i as isize) = col[i as usize] as c_uchar;
            i += 1;
        }
        *color.offset(3 as c_int as isize) = 255 as c_uchar;
    } else if len == 8 as c_int {
        count = sscanf(
            value,
            b"%02x%02x%02x%02x%n\0" as *const u8 as *const c_char,
            (&raw mut col as *mut c_uint).offset(0 as c_int as isize)
                as *mut c_uint,
            (&raw mut col as *mut c_uint).offset(1 as c_int as isize)
                as *mut c_uint,
            (&raw mut col as *mut c_uint).offset(2 as c_int as isize)
                as *mut c_uint,
            (&raw mut col as *mut c_uint).offset(3 as c_int as isize)
                as *mut c_uint,
            &raw mut len,
        );
        if count < 4 as c_int || len != 8 as c_int {
            return -(1 as c_int);
        }
        i = 0 as c_int;
        while i < 4 as c_int {
            *color.offset(i as isize) = col[i as usize] as c_uchar;
            i += 1;
        }
    } else {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
pub const MAX_DATA_SIZE: c_int = 7090 as c_int * 2 as c_int;
static mut data_buffer: [c_uchar; 14180] = [0; 14180];
unsafe extern "C" fn readFile(
    mut fp: *mut FILE,
    mut length: *mut c_int,
) -> *mut c_uchar {
    let mut ret: c_int = 0;
    ret = fread(
        &raw mut data_buffer as *mut c_uchar as *mut c_void,
        1 as size_t,
        MAX_DATA_SIZE as size_t,
        fp,
    ) as c_int;
    if ret == 0 as c_int {
        fprintf(
            stderr,
            b"No input data.\n\0" as *const u8 as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    if feof(fp) == 0 as c_int {
        fprintf(
            stderr,
            b"Input data is too large.\n\0" as *const u8 as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    data_buffer[ret as usize] = '\0' as i32 as c_uchar;
    *length = ret;
    return &raw mut data_buffer as *mut c_uchar;
}
unsafe extern "C" fn openFile(mut outfile: *const c_char) -> *mut FILE {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    if outfile.is_null()
        || *outfile.offset(0 as c_int as isize) as c_int == '-' as i32
            && *outfile.offset(1 as c_int as isize) as c_int
                == '\0' as i32
    {
        fp = stdout;
    } else {
        fp = fopen(outfile, b"wb\0" as *const u8 as *const c_char) as *mut FILE;
        if fp.is_null() {
            fprintf(
                stderr,
                b"Failed to create file: %s\n\0" as *const u8 as *const c_char,
                outfile,
            );
            perror(::core::ptr::null::<c_char>());
            exit(EXIT_FAILURE);
        }
    }
    return fp;
}
unsafe extern "C" fn writePNG(
    mut qrcode: *const QRcode,
    mut outfile: *const c_char,
    mut type_0: imageType,
) -> c_int {
    fputs(
        b"PNG output is disabled at compile time. No output generated.\n\0" as *const u8
            as *const c_char,
        stderr,
    );
    return 0 as c_int;
}
unsafe extern "C" fn writeEPS(
    mut qrcode: *const QRcode,
    mut outfile: *const c_char,
) -> c_int {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut row: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut yy: c_int = 0;
    let mut realwidth: c_int = 0;
    fp = openFile(outfile);
    realwidth = ((*qrcode).width + margin * 2 as c_int) * size;
    fprintf(
        fp,
        b"%%!PS-Adobe-2.0 EPSF-1.2\n%%%%BoundingBox: 0 0 %d %d\n%%%%Pages: 1 1\n%%%%EndComments\n\0"
            as *const u8 as *const c_char,
        realwidth,
        realwidth,
    );
    fprintf(
        fp,
        b"/p { moveto 0 1 rlineto 1 0 rlineto 0 -1 rlineto fill } bind def\n\0" as *const u8
            as *const c_char,
    );
    fprintf(fp, b"gsave\n\0" as *const u8 as *const c_char);
    fprintf(
        fp,
        b"%f %f %f setrgbcolor\n\0" as *const u8 as *const c_char,
        (bg_color[0 as c_int as usize] as c_float
            / 255 as c_int as c_float) as c_double,
        (bg_color[1 as c_int as usize] as c_float
            / 255 as c_int as c_float) as c_double,
        (bg_color[2 as c_int as usize] as c_float
            / 255 as c_int as c_float) as c_double,
    );
    fprintf(
        fp,
        b"%d %d scale\n\0" as *const u8 as *const c_char,
        realwidth,
        realwidth,
    );
    fprintf(
        fp,
        b"0 0 p\ngrestore\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        fp,
        b"%f %f %f setrgbcolor\n\0" as *const u8 as *const c_char,
        (fg_color[0 as c_int as usize] as c_float
            / 255 as c_int as c_float) as c_double,
        (fg_color[1 as c_int as usize] as c_float
            / 255 as c_int as c_float) as c_double,
        (fg_color[2 as c_int as usize] as c_float
            / 255 as c_int as c_float) as c_double,
    );
    fprintf(
        fp,
        b"%d %d scale\n\0" as *const u8 as *const c_char,
        size,
        size,
    );
    p = (*qrcode).data;
    y = 0 as c_int;
    while y < (*qrcode).width {
        row = p.offset((y * (*qrcode).width) as isize);
        yy = margin + (*qrcode).width - y - 1 as c_int;
        x = 0 as c_int;
        while x < (*qrcode).width {
            if *row.offset(x as isize) as c_int & 0x1 as c_int != 0 {
                fprintf(
                    fp,
                    b"%d %d p \0" as *const u8 as *const c_char,
                    margin + x,
                    yy,
                );
            }
            x += 1;
        }
        y += 1;
    }
    fprintf(
        fp,
        b"\n%%%%EOF\n\0" as *const u8 as *const c_char,
    );
    fclose(fp);
    return 0 as c_int;
}
unsafe extern "C" fn writeSVG_drawModules(
    mut fp: *mut FILE,
    mut x: c_int,
    mut y: c_int,
    mut width: c_int,
    mut col: *const c_char,
    mut opacity: c_float,
) {
    if svg_path != 0 {
        fprintf(
            fp,
            b"M%d,%dh%d\0" as *const u8 as *const c_char,
            x,
            y,
            width,
        );
    } else if fg_color[3 as c_int as usize] as c_int
        != 255 as c_int
    {
        fprintf(
            fp,
            b"\t\t\t<rect x=\"%d\" y=\"%d\" width=\"%d\" height=\"1\" fill=\"#%s\" fill-opacity=\"%f\"/>\n\0"
                as *const u8 as *const c_char,
            x,
            y,
            width,
            col,
            opacity as c_double,
        );
    } else {
        fprintf(
            fp,
            b"\t\t\t<rect x=\"%d\" y=\"%d\" width=\"%d\" height=\"1\" fill=\"#%s\"/>\n\0"
                as *const u8 as *const c_char,
            x,
            y,
            width,
            col,
        );
    };
}
unsafe extern "C" fn writeSVG(
    mut qrcode: *const QRcode,
    mut outfile: *const c_char,
) -> c_int {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut row: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut x0: c_int = 0;
    let mut pen: c_int = 0;
    let mut symwidth: c_int = 0;
    let mut realwidth: c_int = 0;
    let mut scale: c_float = 0.;
    let mut fg: [c_char; 7] = [0; 7];
    let mut bg: [c_char; 7] = [0; 7];
    let mut fg_opacity: c_float = 0.;
    let mut bg_opacity: c_float = 0.;
    fp = openFile(outfile);
    scale = (dpi as c_double * INCHES_PER_METER / 100.0f64) as c_float;
    symwidth = (*qrcode).width + margin * 2 as c_int;
    realwidth = symwidth * size;
    snprintf(
        &raw mut fg as *mut c_char,
        7 as size_t,
        b"%02x%02x%02x\0" as *const u8 as *const c_char,
        fg_color[0 as c_int as usize] as c_int,
        fg_color[1 as c_int as usize] as c_int,
        fg_color[2 as c_int as usize] as c_int,
    );
    snprintf(
        &raw mut bg as *mut c_char,
        7 as size_t,
        b"%02x%02x%02x\0" as *const u8 as *const c_char,
        bg_color[0 as c_int as usize] as c_int,
        bg_color[1 as c_int as usize] as c_int,
        bg_color[2 as c_int as usize] as c_int,
    );
    fg_opacity = fg_color[3 as c_int as usize] as c_float
        / 255 as c_int as c_float;
    bg_opacity = bg_color[3 as c_int as usize] as c_float
        / 255 as c_int as c_float;
    if inline_svg == 0 {
        fputs(
            b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n\0" as *const u8
                as *const c_char,
            fp,
        );
    }
    fprintf(
        fp,
        b"<!-- Created with qrencode %s (https://fukuchi.org/works/qrencode/index.html) -->\n\0"
            as *const u8 as *const c_char,
        QRcode_APIVersionString(),
    );
    fprintf(
        fp,
        b"<svg width=\"%.2fcm\" height=\"%.2fcm\" viewBox=\"0 0 %d %d\" preserveAspectRatio=\"none\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">\n\0"
            as *const u8 as *const c_char,
        (realwidth as c_float / scale) as c_double,
        (realwidth as c_float / scale) as c_double,
        symwidth,
        symwidth,
    );
    fputs(
        b"\t<g id=\"QRcode\">\n\0" as *const u8 as *const c_char,
        fp,
    );
    if bg_color[3 as c_int as usize] as c_int != 255 as c_int
    {
        fprintf(
            fp,
            b"\t\t<rect x=\"0\" y=\"0\" width=\"%d\" height=\"%d\" fill=\"#%s\" fill-opacity=\"%f\"/>\n\0"
                as *const u8 as *const c_char,
            symwidth,
            symwidth,
            &raw mut bg as *mut c_char,
            bg_opacity as c_double,
        );
    } else {
        fprintf(
            fp,
            b"\t\t<rect x=\"0\" y=\"0\" width=\"%d\" height=\"%d\" fill=\"#%s\"/>\n\0" as *const u8
                as *const c_char,
            symwidth,
            symwidth,
            &raw mut bg as *mut c_char,
        );
    }
    if svg_path != 0 {
        if fg_color[3 as c_int as usize] as c_int
            != 255 as c_int
        {
            fprintf(
                fp,
                b"\t\t<path style=\"stroke:#%s;stroke-opacity:%f\" transform=\"translate(%d,%d.5)\" d=\"\0"
                    as *const u8 as *const c_char,
                &raw mut fg as *mut c_char,
                fg_opacity as c_double,
                margin,
                margin,
            );
        } else {
            fprintf(
                fp,
                b"\t\t<path style=\"stroke:#%s\" transform=\"translate(%d,%d.5)\" d=\"\0"
                    as *const u8 as *const c_char,
                &raw mut fg as *mut c_char,
                margin,
                margin,
            );
        }
    } else {
        fprintf(
            fp,
            b"\t\t<g id=\"Pattern\" transform=\"translate(%d,%d)\">\n\0" as *const u8
                as *const c_char,
            margin,
            margin,
        );
    }
    p = (*qrcode).data;
    y = 0 as c_int;
    while y < (*qrcode).width {
        row = p.offset((y * (*qrcode).width) as isize);
        if rle == 0 {
            x = 0 as c_int;
            while x < (*qrcode).width {
                if *row.offset(x as isize) as c_int & 0x1 as c_int != 0 {
                    writeSVG_drawModules(
                        fp,
                        x,
                        y,
                        1 as c_int,
                        &raw mut fg as *mut c_char,
                        fg_opacity,
                    );
                }
                x += 1;
            }
        } else {
            pen = 0 as c_int;
            x0 = 0 as c_int;
            x = 0 as c_int;
            while x < (*qrcode).width {
                if pen == 0 {
                    pen = *row.offset(x as isize) as c_int & 0x1 as c_int;
                    x0 = x;
                } else if *row.offset(x as isize) as c_int & 0x1 as c_int
                    == 0
                {
                    writeSVG_drawModules(
                        fp,
                        x0,
                        y,
                        x - x0,
                        &raw mut fg as *mut c_char,
                        fg_opacity,
                    );
                    pen = 0 as c_int;
                }
                x += 1;
            }
            if pen != 0 {
                writeSVG_drawModules(
                    fp,
                    x0,
                    y,
                    (*qrcode).width - x0,
                    &raw mut fg as *mut c_char,
                    fg_opacity,
                );
            }
        }
        y += 1;
    }
    if svg_path != 0 {
        fputs(b"\"/>\n\0" as *const u8 as *const c_char, fp);
    } else {
        fputs(
            b"\t\t</g>\n\0" as *const u8 as *const c_char,
            fp,
        );
    }
    fputs(b"\t</g>\n\0" as *const u8 as *const c_char, fp);
    fputs(b"</svg>\n\0" as *const u8 as *const c_char, fp);
    fclose(fp);
    return 0 as c_int;
}
unsafe extern "C" fn writeXPM(
    mut qrcode: *const QRcode,
    mut outfile: *const c_char,
) -> c_int {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut x: c_int = 0;
    let mut xx: c_int = 0;
    let mut y: c_int = 0;
    let mut yy: c_int = 0;
    let mut realwidth: c_int = 0;
    let mut realmargin: c_int = 0;
    let mut row: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut fg: [c_char; 7] = [0; 7];
    let mut bg: [c_char; 7] = [0; 7];
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    fp = openFile(outfile);
    realwidth = ((*qrcode).width + margin * 2 as c_int) * size;
    realmargin = margin * size;
    row = malloc((realwidth as size_t).wrapping_add(1 as size_t)) as *mut c_char;
    if row.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory.\n\0" as *const u8 as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    snprintf(
        &raw mut fg as *mut c_char,
        7 as size_t,
        b"%02x%02x%02x\0" as *const u8 as *const c_char,
        fg_color[0 as c_int as usize] as c_int,
        fg_color[1 as c_int as usize] as c_int,
        fg_color[2 as c_int as usize] as c_int,
    );
    snprintf(
        &raw mut bg as *mut c_char,
        7 as size_t,
        b"%02x%02x%02x\0" as *const u8 as *const c_char,
        bg_color[0 as c_int as usize] as c_int,
        bg_color[1 as c_int as usize] as c_int,
        bg_color[2 as c_int as usize] as c_int,
    );
    fputs(
        b"/* XPM */\n\0" as *const u8 as *const c_char,
        fp,
    );
    fputs(
        b"static const char *const qrcode_xpm[] = {\n\0" as *const u8 as *const c_char,
        fp,
    );
    fputs(
        b"/* width height ncolors chars_per_pixel */\n\0" as *const u8
            as *const c_char,
        fp,
    );
    fprintf(
        fp,
        b"\"%d %d 2 1\",\n\0" as *const u8 as *const c_char,
        realwidth,
        realwidth,
    );
    fputs(
        b"/* colors */\n\0" as *const u8 as *const c_char,
        fp,
    );
    fprintf(
        fp,
        b"\"F c #%s\",\n\0" as *const u8 as *const c_char,
        &raw mut fg as *mut c_char,
    );
    fprintf(
        fp,
        b"\"B c #%s\",\n\0" as *const u8 as *const c_char,
        &raw mut bg as *mut c_char,
    );
    fputs(
        b"/* pixels */\n\0" as *const u8 as *const c_char,
        fp,
    );
    memset(
        row as *mut c_void,
        'B' as i32,
        realwidth as size_t,
    );
    *row.offset(realwidth as isize) = '\0' as i32 as c_char;
    y = 0 as c_int;
    while y < realmargin {
        fprintf(
            fp,
            b"\"%s\",\n\0" as *const u8 as *const c_char,
            row,
        );
        y += 1;
    }
    p = (*qrcode).data;
    y = 0 as c_int;
    while y < (*qrcode).width {
        yy = 0 as c_int;
        while yy < size {
            fputs(b"\"\0" as *const u8 as *const c_char, fp);
            x = 0 as c_int;
            while x < margin {
                xx = 0 as c_int;
                while xx < size {
                    fputs(b"B\0" as *const u8 as *const c_char, fp);
                    xx += 1;
                }
                x += 1;
            }
            x = 0 as c_int;
            while x < (*qrcode).width {
                xx = 0 as c_int;
                while xx < size {
                    if *p.offset((y * (*qrcode).width + x) as isize) as c_int
                        & 0x1 as c_int
                        != 0
                    {
                        fputs(b"F\0" as *const u8 as *const c_char, fp);
                    } else {
                        fputs(b"B\0" as *const u8 as *const c_char, fp);
                    }
                    xx += 1;
                }
                x += 1;
            }
            x = 0 as c_int;
            while x < margin {
                xx = 0 as c_int;
                while xx < size {
                    fputs(b"B\0" as *const u8 as *const c_char, fp);
                    xx += 1;
                }
                x += 1;
            }
            fputs(b"\",\n\0" as *const u8 as *const c_char, fp);
            yy += 1;
        }
        y += 1;
    }
    y = 0 as c_int;
    while y < realmargin {
        fprintf(
            fp,
            b"\"%s\"%s\n\0" as *const u8 as *const c_char,
            row,
            if y < size - 1 as c_int {
                b",\0" as *const u8 as *const c_char
            } else {
                b"};\0" as *const u8 as *const c_char
            },
        );
        y += 1;
    }
    free(row as *mut c_void);
    fclose(fp);
    return 0 as c_int;
}
unsafe extern "C" fn writeANSI_margin(
    mut fp: *mut FILE,
    mut realwidth: c_int,
    mut buffer: *mut c_char,
    mut white: *const c_char,
    mut white_s: c_int,
) {
    let mut y: c_int = 0;
    strncpy(buffer, white, white_s as size_t);
    memset(
        buffer.offset(white_s as isize) as *mut c_void,
        ' ' as i32,
        (realwidth as size_t).wrapping_mul(2 as size_t),
    );
    strcpy(
        buffer
            .offset(white_s as isize)
            .offset((realwidth * 2 as c_int) as isize),
        b"\x1B[0m\n\0" as *const u8 as *const c_char,
    );
    y = 0 as c_int;
    while y < margin {
        fputs(buffer, fp);
        y += 1;
    }
}
unsafe extern "C" fn writeANSI(
    mut qrcode: *const QRcode,
    mut outfile: *const c_char,
) -> c_int {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut row: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut realwidth: c_int = 0;
    let mut last: c_int = 0;
    let mut white: *const c_char = ::core::ptr::null::<c_char>();
    let mut black: *const c_char = ::core::ptr::null::<c_char>();
    let mut buffer: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut white_s: c_int = 0;
    let mut black_s: c_int = 0;
    let mut buffer_s: c_int = 0;
    if image_type as c_uint
        == ANSI256_TYPE as c_int as c_uint
    {
        white = b"\x1B[48;5;231m\0" as *const u8 as *const c_char;
        white_s = 11 as c_int;
        black = b"\x1B[48;5;16m\0" as *const u8 as *const c_char;
        black_s = 10 as c_int;
    } else {
        white = b"\x1B[47m\0" as *const u8 as *const c_char;
        white_s = 5 as c_int;
        black = b"\x1B[40m\0" as *const u8 as *const c_char;
        black_s = 5 as c_int;
    }
    size = 1 as c_int;
    fp = openFile(outfile);
    realwidth = ((*qrcode).width + margin * 2 as c_int) * size;
    buffer_s = realwidth * white_s * 2 as c_int;
    buffer = malloc(buffer_s as size_t) as *mut c_char;
    if buffer.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory.\n\0" as *const u8 as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    writeANSI_margin(fp, realwidth, buffer, white, white_s);
    p = (*qrcode).data;
    y = 0 as c_int;
    while y < (*qrcode).width {
        row = p.offset((y * (*qrcode).width) as isize);
        memset(
            buffer as *mut c_void,
            0 as c_int,
            buffer_s as size_t,
        );
        strncpy(buffer, white, white_s as size_t);
        x = 0 as c_int;
        while x < margin {
            strncat(
                buffer,
                b"  \0" as *const u8 as *const c_char,
                2 as size_t,
            );
            x += 1;
        }
        last = 0 as c_int;
        x = 0 as c_int;
        while x < (*qrcode).width {
            if *row.offset(x as isize) as c_int & 0x1 as c_int != 0 {
                if last != 1 as c_int {
                    strncat(buffer, black, black_s as size_t);
                    last = 1 as c_int;
                }
            } else if last != 0 as c_int {
                strncat(buffer, white, white_s as size_t);
                last = 0 as c_int;
            }
            strncat(
                buffer,
                b"  \0" as *const u8 as *const c_char,
                2 as size_t,
            );
            x += 1;
        }
        if last != 0 as c_int {
            strncat(buffer, white, white_s as size_t);
        }
        x = 0 as c_int;
        while x < margin {
            strncat(
                buffer,
                b"  \0" as *const u8 as *const c_char,
                2 as size_t,
            );
            x += 1;
        }
        strncat(
            buffer,
            b"\x1B[0m\n\0" as *const u8 as *const c_char,
            5 as size_t,
        );
        fputs(buffer, fp);
        y += 1;
    }
    writeANSI_margin(fp, realwidth, buffer, white, white_s);
    fclose(fp);
    free(buffer as *mut c_void);
    return 0 as c_int;
}
unsafe extern "C" fn writeUTF8_margin(
    mut fp: *mut FILE,
    mut realwidth: c_int,
    mut white: *const c_char,
    mut reset: *const c_char,
    mut full: *const c_char,
) {
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    y = 0 as c_int;
    while y < margin / 2 as c_int {
        fputs(white, fp);
        x = 0 as c_int;
        while x < realwidth {
            fputs(full, fp);
            x += 1;
        }
        fputs(reset, fp);
        fputc('\n' as i32, fp);
        y += 1;
    }
}
unsafe extern "C" fn writeUTF8(
    mut qrcode: *const QRcode,
    mut outfile: *const c_char,
    mut use_ansi: c_int,
    mut invert: c_int,
) -> c_int {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut realwidth: c_int = 0;
    let mut white: *const c_char = ::core::ptr::null::<c_char>();
    let mut reset: *const c_char = ::core::ptr::null::<c_char>();
    let mut empty: *const c_char = ::core::ptr::null::<c_char>();
    let mut lowhalf: *const c_char = ::core::ptr::null::<c_char>();
    let mut uphalf: *const c_char = ::core::ptr::null::<c_char>();
    let mut full: *const c_char = ::core::ptr::null::<c_char>();
    empty = b" \0" as *const u8 as *const c_char;
    lowhalf = b"\xE2\x96\x84\0" as *const u8 as *const c_char;
    uphalf = b"\xE2\x96\x80\0" as *const u8 as *const c_char;
    full = b"\xE2\x96\x88\0" as *const u8 as *const c_char;
    if invert != 0 {
        let mut tmp: *const c_char = ::core::ptr::null::<c_char>();
        tmp = empty;
        empty = full;
        full = tmp;
        tmp = lowhalf;
        lowhalf = uphalf;
        uphalf = tmp;
    }
    if use_ansi != 0 {
        if use_ansi == 2 as c_int {
            white = b"\x1B[38;5;231m\x1B[48;5;16m\0" as *const u8 as *const c_char;
        } else {
            white = b"\x1B[40;37;1m\0" as *const u8 as *const c_char;
        }
        reset = b"\x1B[0m\0" as *const u8 as *const c_char;
    } else {
        white = b"\0" as *const u8 as *const c_char;
        reset = b"\0" as *const u8 as *const c_char;
    }
    fp = openFile(outfile);
    realwidth = (*qrcode).width + margin * 2 as c_int;
    writeUTF8_margin(fp, realwidth, white, reset, full);
    y = 0 as c_int;
    while y < (*qrcode).width {
        let mut row1: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
        let mut row2: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
        row1 = (*qrcode).data.offset((y * (*qrcode).width) as isize);
        row2 = row1.offset((*qrcode).width as isize);
        fputs(white, fp);
        x = 0 as c_int;
        while x < margin {
            fputs(full, fp);
            x += 1;
        }
        x = 0 as c_int;
        while x < (*qrcode).width {
            if *row1.offset(x as isize) as c_int & 1 as c_int != 0 {
                if y < (*qrcode).width - 1 as c_int
                    && *row2.offset(x as isize) as c_int & 1 as c_int != 0
                {
                    fputs(empty, fp);
                } else {
                    fputs(lowhalf, fp);
                }
            } else if y < (*qrcode).width - 1 as c_int
                && *row2.offset(x as isize) as c_int & 1 as c_int != 0
            {
                fputs(uphalf, fp);
            } else {
                fputs(full, fp);
            }
            x += 1;
        }
        x = 0 as c_int;
        while x < margin {
            fputs(full, fp);
            x += 1;
        }
        fputs(reset, fp);
        fputc('\n' as i32, fp);
        y += 2 as c_int;
    }
    writeUTF8_margin(fp, realwidth, white, reset, full);
    fclose(fp);
    return 0 as c_int;
}
unsafe extern "C" fn writeASCII_margin(
    mut fp: *mut FILE,
    mut realwidth: c_int,
    mut buffer: *mut c_char,
    mut invert: c_int,
) {
    let mut y: c_int = 0;
    let mut h: c_int = 0;
    h = margin;
    memset(
        buffer as *mut c_void,
        if invert != 0 { '#' as i32 } else { ' ' as i32 },
        realwidth as size_t,
    );
    *buffer.offset(realwidth as isize) = '\n' as i32 as c_char;
    *buffer.offset((realwidth + 1 as c_int) as isize) =
        '\0' as i32 as c_char;
    y = 0 as c_int;
    while y < h {
        fputs(buffer, fp);
        y += 1;
    }
}
unsafe extern "C" fn writeASCII(
    mut qrcode: *const QRcode,
    mut outfile: *const c_char,
    mut invert: c_int,
) -> c_int {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut row: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut realwidth: c_int = 0;
    let mut buffer: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut p: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut buffer_s: c_int = 0;
    let mut black: c_char = '#' as i32 as c_char;
    let mut white: c_char = ' ' as i32 as c_char;
    if invert != 0 {
        black = ' ' as i32 as c_char;
        white = '#' as i32 as c_char;
    }
    size = 1 as c_int;
    fp = openFile(outfile);
    realwidth = ((*qrcode).width + margin * 2 as c_int) * 2 as c_int;
    buffer_s = realwidth + 2 as c_int;
    buffer = malloc(buffer_s as size_t) as *mut c_char;
    if buffer.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory.\n\0" as *const u8 as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    writeASCII_margin(fp, realwidth, buffer, invert);
    y = 0 as c_int;
    while y < (*qrcode).width {
        row = (*qrcode).data.offset((y * (*qrcode).width) as isize);
        p = buffer;
        memset(
            p as *mut c_void,
            white as c_int,
            (margin as size_t).wrapping_mul(2 as size_t),
        );
        p = p.offset((margin * 2 as c_int) as isize);
        x = 0 as c_int;
        while x < (*qrcode).width {
            if *row.offset(x as isize) as c_int & 0x1 as c_int != 0 {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = black;
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = black;
            } else {
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = white;
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = white;
            }
            x += 1;
        }
        memset(
            p as *mut c_void,
            white as c_int,
            (margin as size_t).wrapping_mul(2 as size_t),
        );
        p = p.offset((margin * 2 as c_int) as isize);
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = '\n' as i32 as c_char;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '\0' as i32 as c_char;
        fputs(buffer, fp);
        y += 1;
    }
    writeASCII_margin(fp, realwidth, buffer, invert);
    fclose(fp);
    free(buffer as *mut c_void);
    return 0 as c_int;
}
unsafe extern "C" fn encode(
    mut intext: *const c_uchar,
    mut length: c_int,
) -> *mut QRcode {
    let mut code: *mut QRcode = ::core::ptr::null_mut::<QRcode>();
    if micro != 0 {
        if eightbit != 0 {
            code = QRcode_encodeDataMQR(length, intext, version, level);
        } else {
            code = QRcode_encodeStringMQR(
                intext as *mut c_char,
                version,
                level,
                hint,
                casesensitive,
            );
        }
    } else if eightbit != 0 {
        code = QRcode_encodeData(length, intext, version, level);
    } else {
        code = QRcode_encodeString(
            intext as *mut c_char,
            version,
            level,
            hint,
            casesensitive,
        );
    }
    return code;
}
unsafe extern "C" fn qrencode(
    mut intext: *const c_uchar,
    mut length: c_int,
    mut outfile: *const c_char,
) {
    let mut qrcode: *mut QRcode = ::core::ptr::null_mut::<QRcode>();
    qrcode = encode(intext, length);
    if qrcode.is_null() {
        if *__errno_location() == ERANGE {
            fprintf(
                stderr,
                b"Failed to encode the input data: Input data too large\n\0" as *const u8
                    as *const c_char,
            );
        } else {
            perror(b"Failed to encode the input data\0" as *const u8 as *const c_char);
        }
        exit(EXIT_FAILURE);
    }
    if strict_versioning != 0 && version > 0 as c_int && (*qrcode).version != version {
        fprintf(
            stderr,
            b"Failed to encode the input data: Input data too large\n\0" as *const u8
                as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    if verbose != 0 {
        fprintf(
            stderr,
            b"File: %s, Version: %d\n\0" as *const u8 as *const c_char,
            if !outfile.is_null() {
                outfile
            } else {
                b"(stdout)\0" as *const u8 as *const c_char
            },
            (*qrcode).version,
        );
    }
    match image_type as c_uint {
        0 | 1 => {
            writePNG(qrcode, outfile, image_type);
        }
        2 => {
            writeEPS(qrcode, outfile);
        }
        3 => {
            writeSVG(qrcode, outfile);
        }
        4 => {
            writeXPM(qrcode, outfile);
        }
        5 | 6 => {
            writeANSI(qrcode, outfile);
        }
        8 => {
            writeASCII(qrcode, outfile, 1 as c_int);
        }
        7 => {
            writeASCII(qrcode, outfile, 0 as c_int);
        }
        9 => {
            writeUTF8(
                qrcode,
                outfile,
                0 as c_int,
                0 as c_int,
            );
        }
        10 => {
            writeUTF8(
                qrcode,
                outfile,
                1 as c_int,
                0 as c_int,
            );
        }
        11 => {
            writeUTF8(
                qrcode,
                outfile,
                2 as c_int,
                0 as c_int,
            );
        }
        12 => {
            writeUTF8(
                qrcode,
                outfile,
                0 as c_int,
                1 as c_int,
            );
        }
        13 => {
            writeUTF8(
                qrcode,
                outfile,
                1 as c_int,
                1 as c_int,
            );
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown image type.\n\0" as *const u8 as *const c_char,
            );
            exit(EXIT_FAILURE);
        }
    }
    QRcode_free(qrcode);
}
unsafe extern "C" fn encodeStructured(
    mut intext: *const c_uchar,
    mut length: c_int,
) -> *mut QRcode_List {
    let mut list: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    if eightbit != 0 {
        list = QRcode_encodeDataStructured(length, intext, version, level);
    } else {
        list = QRcode_encodeStringStructured(
            intext as *mut c_char,
            version,
            level,
            hint,
            casesensitive,
        );
    }
    return list;
}
unsafe extern "C" fn qrencodeStructured(
    mut intext: *const c_uchar,
    mut length: c_int,
    mut outfile: *const c_char,
) {
    let mut qrlist: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    let mut p: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    let mut filename: [c_char; 4096] = [0; 4096];
    let mut base: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut q: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut suffix: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut type_suffix: *const c_char = ::core::ptr::null::<c_char>();
    let mut i: c_int = 1 as c_int;
    let mut suffix_size: size_t = 0;
    match image_type as c_uint {
        0 => {
            type_suffix = b".png\0" as *const u8 as *const c_char;
        }
        2 => {
            type_suffix = b".eps\0" as *const u8 as *const c_char;
        }
        3 => {
            type_suffix = b".svg\0" as *const u8 as *const c_char;
        }
        4 => {
            type_suffix = b".xpm\0" as *const u8 as *const c_char;
        }
        5 | 6 | 7 | 9 | 10 | 12 | 13 => {
            type_suffix = b".txt\0" as *const u8 as *const c_char;
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown image type.\n\0" as *const u8 as *const c_char,
            );
            exit(EXIT_FAILURE);
        }
    }
    if outfile.is_null() {
        fprintf(
            stderr,
            b"An output filename must be specified to store the structured images.\n\0" as *const u8
                as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    base = strdup(outfile);
    if base.is_null() {
        fprintf(
            stderr,
            b"Failed to allocate memory.\n\0" as *const u8 as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    suffix_size = strlen(type_suffix);
    if strlen(base) > suffix_size {
        q = base
            .offset(strlen(base) as isize)
            .offset(-(suffix_size as isize));
        if strcasecmp(type_suffix, q) == 0 as c_int {
            suffix = strdup(q);
            *q = '\0' as i32 as c_char;
        }
    }
    qrlist = encodeStructured(intext, length);
    if qrlist.is_null() {
        if *__errno_location() == ERANGE {
            fprintf(
                stderr,
                b"Failed to encode the input data: Input data too large\n\0" as *const u8
                    as *const c_char,
            );
        } else {
            perror(b"Failed to encode the input data\0" as *const u8 as *const c_char);
        }
        exit(EXIT_FAILURE);
    }
    p = qrlist;
    while !p.is_null() {
        if (*p).code.is_null() {
            fprintf(
                stderr,
                b"Failed to encode the input data.\n\0" as *const u8 as *const c_char,
            );
            exit(EXIT_FAILURE);
        }
        if !suffix.is_null() {
            snprintf(
                &raw mut filename as *mut c_char,
                FILENAME_MAX as size_t,
                b"%s-%02d%s\0" as *const u8 as *const c_char,
                base,
                i,
                suffix,
            );
        } else {
            snprintf(
                &raw mut filename as *mut c_char,
                FILENAME_MAX as size_t,
                b"%s-%02d\0" as *const u8 as *const c_char,
                base,
                i,
            );
        }
        if verbose != 0 {
            fprintf(
                stderr,
                b"File: %s, Version: %d\n\0" as *const u8 as *const c_char,
                &raw mut filename as *mut c_char,
                (*(*p).code).version,
            );
        }
        match image_type as c_uint {
            0 | 1 => {
                writePNG(
                    (*p).code,
                    &raw mut filename as *mut c_char,
                    image_type,
                );
            }
            2 => {
                writeEPS((*p).code, &raw mut filename as *mut c_char);
            }
            3 => {
                writeSVG((*p).code, &raw mut filename as *mut c_char);
            }
            4 => {
                writeXPM((*p).code, &raw mut filename as *mut c_char);
            }
            5 | 6 => {
                writeANSI((*p).code, &raw mut filename as *mut c_char);
            }
            8 => {
                writeASCII(
                    (*p).code,
                    &raw mut filename as *mut c_char,
                    1 as c_int,
                );
            }
            7 => {
                writeASCII(
                    (*p).code,
                    &raw mut filename as *mut c_char,
                    0 as c_int,
                );
            }
            9 => {
                writeUTF8(
                    (*p).code,
                    &raw mut filename as *mut c_char,
                    0 as c_int,
                    0 as c_int,
                );
            }
            10 => {
                writeUTF8(
                    (*p).code,
                    &raw mut filename as *mut c_char,
                    0 as c_int,
                    0 as c_int,
                );
            }
            11 => {
                writeUTF8(
                    (*p).code,
                    &raw mut filename as *mut c_char,
                    0 as c_int,
                    0 as c_int,
                );
            }
            12 => {
                writeUTF8(
                    (*p).code,
                    &raw mut filename as *mut c_char,
                    0 as c_int,
                    1 as c_int,
                );
            }
            13 => {
                writeUTF8(
                    (*p).code,
                    &raw mut filename as *mut c_char,
                    0 as c_int,
                    1 as c_int,
                );
            }
            _ => {
                fprintf(
                    stderr,
                    b"Unknown image type.\n\0" as *const u8 as *const c_char,
                );
                exit(EXIT_FAILURE);
            }
        }
        i += 1;
        p = (*p).next as *mut QRcode_List;
    }
    free(base as *mut c_void);
    if !suffix.is_null() {
        free(suffix as *mut c_void);
    }
    QRcode_List_free(qrlist);
}
unsafe fn main_0(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) -> c_int {
    let mut opt: c_int = 0;
    let mut lindex: c_int = -(1 as c_int);
    let mut outfile: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut infile: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut intext: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut length: c_int = 0 as c_int;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    loop {
        opt = getopt_long(
            argc,
            argv,
            optstring,
            &raw const options as *const option,
            &raw mut lindex,
        );
        if !(opt != -(1 as c_int)) {
            break;
        }
        match opt {
            104 => {
                if lindex == 0 as c_int {
                    usage(
                        1 as c_int,
                        1 as c_int,
                        EXIT_SUCCESS,
                    );
                } else {
                    usage(
                        1 as c_int,
                        0 as c_int,
                        EXIT_SUCCESS,
                    );
                }
                exit(EXIT_SUCCESS);
            }
            111 => {
                outfile = optarg;
            }
            114 => {
                infile = optarg;
            }
            115 => {
                size = atoi(optarg);
                if size <= 0 as c_int {
                    fprintf(
                        stderr,
                        b"Invalid size: %d\n\0" as *const u8 as *const c_char,
                        size,
                    );
                    exit(EXIT_FAILURE);
                }
            }
            118 => {
                version = atoi(optarg);
                if version < 0 as c_int {
                    fprintf(
                        stderr,
                        b"Invalid version: %d\n\0" as *const u8 as *const c_char,
                        version,
                    );
                    exit(EXIT_FAILURE);
                }
            }
            108 => match *optarg as c_int {
                108 | 76 => {
                    level = QR_ECLEVEL_L;
                }
                109 | 77 => {
                    level = QR_ECLEVEL_M;
                }
                113 | 81 => {
                    level = QR_ECLEVEL_Q;
                }
                104 | 72 => {
                    level = QR_ECLEVEL_H;
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Invalid level: %s\n\0" as *const u8 as *const c_char,
                        optarg,
                    );
                    exit(EXIT_FAILURE);
                }
            },
            109 => {
                margin = atoi(optarg);
                if margin < 0 as c_int {
                    fprintf(
                        stderr,
                        b"Invalid margin: %d\n\0" as *const u8 as *const c_char,
                        margin,
                    );
                    exit(EXIT_FAILURE);
                }
            }
            100 => {
                dpi = atoi(optarg);
                if dpi < 0 as c_int {
                    fprintf(
                        stderr,
                        b"Invalid DPI: %d\n\0" as *const u8 as *const c_char,
                        dpi,
                    );
                    exit(EXIT_FAILURE);
                }
            }
            116 => {
                if strcasecmp(
                    optarg,
                    b"png32\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    image_type = PNG32_TYPE;
                } else if strcasecmp(optarg, b"png\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    image_type = PNG_TYPE;
                } else if strcasecmp(optarg, b"eps\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    image_type = EPS_TYPE;
                } else if strcasecmp(optarg, b"svg\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    image_type = SVG_TYPE;
                } else if strcasecmp(optarg, b"xpm\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    image_type = XPM_TYPE;
                } else if strcasecmp(optarg, b"ansi\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    image_type = ANSI_TYPE;
                } else if strcasecmp(
                    optarg,
                    b"ansi256\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    image_type = ANSI256_TYPE;
                } else if strcasecmp(
                    optarg,
                    b"asciii\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    image_type = ASCIIi_TYPE;
                } else if strcasecmp(
                    optarg,
                    b"ascii\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    image_type = ASCII_TYPE;
                } else if strcasecmp(optarg, b"utf8\0" as *const u8 as *const c_char)
                    == 0 as c_int
                {
                    image_type = UTF8_TYPE;
                } else if strcasecmp(
                    optarg,
                    b"ansiutf8\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    image_type = ANSIUTF8_TYPE;
                } else if strcasecmp(
                    optarg,
                    b"ansi256utf8\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    image_type = ANSI256UTF8_TYPE;
                } else if strcasecmp(
                    optarg,
                    b"utf8i\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    image_type = UTF8i_TYPE;
                } else if strcasecmp(
                    optarg,
                    b"ansiutf8i\0" as *const u8 as *const c_char,
                ) == 0 as c_int
                {
                    image_type = ANSIUTF8i_TYPE;
                } else {
                    fprintf(
                        stderr,
                        b"Invalid image type: %s\n\0" as *const u8 as *const c_char,
                        optarg,
                    );
                    exit(EXIT_FAILURE);
                }
            }
            83 => {
                structured = 1 as c_int;
            }
            107 => {
                hint = QR_MODE_KANJI;
            }
            99 => {
                casesensitive = 1 as c_int;
            }
            105 => {
                casesensitive = 0 as c_int;
            }
            56 => {
                eightbit = 1 as c_int;
            }
            77 => {
                micro = 1 as c_int;
            }
            102 => {
                if color_set(&raw mut fg_color as *mut c_uchar, optarg) != 0 {
                    fprintf(
                        stderr,
                        b"Invalid foreground color value.\n\0" as *const u8
                            as *const c_char,
                    );
                    exit(EXIT_FAILURE);
                }
            }
            98 => {
                if color_set(&raw mut bg_color as *mut c_uchar, optarg) != 0 {
                    fprintf(
                        stderr,
                        b"Invalid background color value.\n\0" as *const u8
                            as *const c_char,
                    );
                    exit(EXIT_FAILURE);
                }
            }
            86 => {
                usage(
                    0 as c_int,
                    0 as c_int,
                    EXIT_SUCCESS,
                );
                exit(EXIT_SUCCESS);
            }
            0 => {}
            _ => {
                fprintf(
                    stderr,
                    b"Try \"qrencode --help\" for more information.\n\0" as *const u8
                        as *const c_char,
                );
                exit(EXIT_FAILURE);
            }
        }
    }
    if argc == 1 as c_int {
        usage(
            1 as c_int,
            0 as c_int,
            EXIT_FAILURE,
        );
        exit(EXIT_FAILURE);
    }
    if outfile.is_null()
        && image_type as c_uint
            == PNG_TYPE as c_int as c_uint
    {
        fprintf(
            stderr,
            b"No output filename is given.\n\0" as *const u8 as *const c_char,
        );
        exit(EXIT_FAILURE);
    }
    if optind < argc {
        intext = *argv.offset(optind as isize) as *mut c_uchar;
        length = strlen(intext as *mut c_char) as c_int;
    }
    if intext.is_null() {
        fp = (if infile.is_null() {
            stdin as *mut FILE
        } else {
            fopen(infile, b"r\0" as *const u8 as *const c_char)
        }) as *mut FILE;
        if fp.is_null() {
            fprintf(
                stderr,
                b"Cannot read input file %s.\n\0" as *const u8 as *const c_char,
                infile,
            );
            exit(EXIT_FAILURE);
        }
        intext = readFile(fp, &raw mut length);
    }
    if micro != 0 && version > MQRSPEC_VERSION_MAX {
        fprintf(
            stderr,
            b"Version number should be less or equal to %d.\n\0" as *const u8
                as *const c_char,
            MQRSPEC_VERSION_MAX,
        );
        exit(EXIT_FAILURE);
    } else if micro == 0 && version > QRSPEC_VERSION_MAX {
        fprintf(
            stderr,
            b"Version number should be less or equal to %d.\n\0" as *const u8
                as *const c_char,
            QRSPEC_VERSION_MAX,
        );
        exit(EXIT_FAILURE);
    }
    if margin < 0 as c_int {
        if micro != 0 {
            margin = 2 as c_int;
        } else {
            margin = 4 as c_int;
        }
    }
    if micro != 0 {
        if structured != 0 {
            fprintf(
                stderr,
                b"Micro QR Code does not support structured symbols.\n\0" as *const u8
                    as *const c_char,
            );
            exit(EXIT_FAILURE);
        }
    }
    if structured != 0 {
        if version == 0 as c_int {
            fprintf(
                stderr,
                b"Version number must be specified to encode structured symbols.\n\0" as *const u8
                    as *const c_char,
            );
            exit(EXIT_FAILURE);
        }
        qrencodeStructured(intext, length, outfile);
    } else {
        qrencode(intext, length, outfile);
    }
    return 0 as c_int;
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
