#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#[allow(unused_imports)]
use ::libopenaptx_raw;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type aptx_context;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static aptx_major: ::core::ffi::c_int;
    static aptx_minor: ::core::ffi::c_int;
    static aptx_patch: ::core::ffi::c_int;
    fn aptx_init(hd: ::core::ffi::c_int) -> *mut aptx_context;
    fn aptx_finish(ctx: *mut aptx_context);
    fn aptx_decode_sync(
        ctx: *mut aptx_context,
        input: *const ::core::ffi::c_uchar,
        input_size: size_t,
        output: *mut ::core::ffi::c_uchar,
        output_size: size_t,
        written: *mut size_t,
        synced: *mut ::core::ffi::c_int,
        dropped: *mut size_t,
    ) -> size_t;
    fn aptx_decode_sync_finish(ctx: *mut aptx_context) -> size_t;
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
pub const OPENAPTX_MAJOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPENAPTX_MINOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OPENAPTX_PATCH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut input_buffer: [::core::ffi::c_uchar; 3072] = [0; 3072];
static mut output_buffer: [::core::ffi::c_uchar; 18456] = [0; 18456];
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut hd: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut length: size_t = 0;
    let mut processed: size_t = 0;
    let mut written: size_t = 0;
    let mut dropped: size_t = 0;
    let mut synced: ::core::ffi::c_int = 0;
    let mut syncing: ::core::ffi::c_int = 0;
    let mut ctx: *mut aptx_context = ::core::ptr::null_mut::<aptx_context>();
    hd = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--help\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"aptX decoder utility %d.%d.%d (using libopenaptx %d.%d.%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                OPENAPTX_MAJOR,
                OPENAPTX_MINOR,
                OPENAPTX_PATCH,
                aptx_major,
                aptx_minor,
                aptx_patch,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"This utility decodes aptX or aptX HD audio stream\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fprintf(
                stderr,
                b"from stdin to a raw 24 bit signed stereo on stdout\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"When input is damaged it tries to synchronize and recover\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"Non-zero return value indicates that input was damaged\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fprintf(
                stderr,
                b"and some bytes from input aptX audio stream were dropped\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"Usage:\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            fprintf(
                stderr,
                b"        %s [options]\n\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"Options:\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            fprintf(
                stderr,
                b"        -h, --help   Display this help\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fprintf(
                stderr,
                b"        --hd         Decode from aptX HD\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"Examples:\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"        %s < sample.aptx > sample.s24le\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"        %s --hd < sample.aptxhd > sample.s24le\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"        %s < sample.aptx | play -t raw -r 44.1k -L -e s -b 24 -c 2 -\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            return 1 as ::core::ffi::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--hd\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            hd = 1 as ::core::ffi::c_int;
        } else {
            fprintf(
                stderr,
                b"%s: Invalid option %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                *argv.offset(i as isize),
            );
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    ctx = aptx_init(hd);
    if ctx.is_null() {
        fprintf(
            stderr,
            b"%s: Cannot initialize aptX decoder\n\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        return 1 as ::core::ffi::c_int;
    }
    length = fread(
        &raw mut input_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        1 as size_t,
        6 as size_t,
        stdin,
    ) as size_t;
    if length >= 4 as size_t
        && memcmp(
            &raw mut input_buffer as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            b"K\xBFK\xBF\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        if hd != 0 {
            fprintf(
                stderr,
                b"%s: Input looks like start of aptX audio stream (not aptX HD), try without --hd\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
        }
    } else if length >= 6 as size_t
        && memcmp(
            &raw mut input_buffer as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            b"s\xBE\xFFs\xBE\xFF\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            6 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        if hd == 0 {
            fprintf(
                stderr,
                b"%s: Input looks like start of aptX HD audio stream, try with --hd\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
        }
    } else if length >= 4 as size_t
        && memcmp(
            &raw mut input_buffer as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            b"k\xBFk\xBF\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"%s: Input looks like start of standard aptX audio stream, which is not supported yet\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
    } else {
        fprintf(
            stderr,
            b"%s: Input does not look like start of aptX nor aptX HD audio stream\n\0" as *const u8
                as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
    }
    ret = 0 as ::core::ffi::c_int;
    syncing = 0 as ::core::ffi::c_int;
    while length > 0 as size_t {
        processed = aptx_decode_sync(
            ctx,
            &raw mut input_buffer as *mut ::core::ffi::c_uchar,
            length,
            &raw mut output_buffer as *mut ::core::ffi::c_uchar,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 18456]>() as size_t,
            &raw mut written,
            &raw mut synced,
            &raw mut dropped,
        );
        if synced == 0 {
            if syncing == 0 {
                fprintf(
                    stderr,
                    b"%s: aptX decoding failed, synchronizing\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                syncing = 1 as ::core::ffi::c_int;
                ret = 1 as ::core::ffi::c_int;
            }
            if dropped != 0 {
                fprintf(
                    stderr,
                    b"%s: aptX synchronization successful, dropped %lu byte%s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    dropped as ::core::ffi::c_ulong,
                    if dropped != 1 as size_t {
                        b"s\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"\0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
                syncing = 0 as ::core::ffi::c_int;
                ret = 1 as ::core::ffi::c_int;
            }
            if syncing == 0 {
                fprintf(
                    stderr,
                    b"%s: aptX decoding failed, synchronizing\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                syncing = 1 as ::core::ffi::c_int;
                ret = 1 as ::core::ffi::c_int;
            }
        } else if dropped != 0 {
            if syncing == 0 {
                fprintf(
                    stderr,
                    b"%s: aptX decoding failed, synchronizing\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
            }
            fprintf(
                stderr,
                b"%s: aptX synchronization successful, dropped %lu byte%s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
                dropped as ::core::ffi::c_ulong,
                if dropped != 1 as size_t {
                    b"s\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            syncing = 0 as ::core::ffi::c_int;
            ret = 1 as ::core::ffi::c_int;
        } else if syncing != 0 {
            fprintf(
                stderr,
                b"%s: aptX synchronization successful\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            syncing = 0 as ::core::ffi::c_int;
            ret = 1 as ::core::ffi::c_int;
        }
        if processed != length {
            fprintf(
                stderr,
                b"%s: aptX decoding failed\n\0" as *const u8 as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            ret = 1 as ::core::ffi::c_int;
            break;
        } else {
            if feof(stdin) == 0 {
                length = fread(
                    &raw mut input_buffer as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                    1 as size_t,
                    ::core::mem::size_of::<[::core::ffi::c_uchar; 3072]>() as size_t,
                    stdin,
                ) as size_t;
                if ferror(stdin) != 0 {
                    fprintf(
                        stderr,
                        b"%s: aptX decoding failed to read input data\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        *argv.offset(0 as ::core::ffi::c_int as isize),
                    );
                    ret = 1 as ::core::ffi::c_int;
                    length = 0 as size_t;
                }
            } else {
                length = 0 as size_t;
            }
            if length == 0 as size_t
                && ferror(stdin) == 0
                && written >= (6 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as size_t
            {
                written = written
                    .wrapping_sub((6 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as size_t);
            }
            if !(written > 0 as size_t) {
                continue;
            }
            if !(fwrite(
                &raw mut output_buffer as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                1 as size_t,
                written,
                stdout,
            ) as size_t
                != written)
            {
                continue;
            }
            fprintf(
                stderr,
                b"%s: aptX decoding failed to write decoded data\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *argv.offset(0 as ::core::ffi::c_int as isize),
            );
            ret = 1 as ::core::ffi::c_int;
            break;
        }
    }
    dropped = aptx_decode_sync_finish(ctx);
    if dropped != 0 && syncing == 0 {
        fprintf(
            stderr,
            b"%s: aptX decoding stopped in the middle of the sample, dropped %lu byte%s\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
            dropped as ::core::ffi::c_ulong,
            if dropped != 1 as size_t {
                b"s\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        ret = 1 as ::core::ffi::c_int;
    } else if syncing != 0 {
        fprintf(
            stderr,
            b"%s: aptX synchronization failed\n\0" as *const u8 as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        ret = 1 as ::core::ffi::c_int;
    }
    aptx_finish(ctx);
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
