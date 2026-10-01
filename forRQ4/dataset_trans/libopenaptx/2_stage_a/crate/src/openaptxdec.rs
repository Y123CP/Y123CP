#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
use core::ffi::*;
use ::libopenaptx_cleaned::src::ffi::*;
use ::libopenaptx_cleaned::src::c_consts::*;
use ::libopenaptx_cleaned::src::c_structs::*;
use ::libopenaptx_cleaned::src::c_types::*;
use ::libopenaptx_cleaned::src::c_extern_types::*;
#[allow(unused_imports)]
use ::libopenaptx_cleaned;
use ::libopenaptx_cleaned::src::openaptx::aptx_decode_sync;
use ::libopenaptx_cleaned::src::openaptx::aptx_decode_sync_finish;
use ::libopenaptx_cleaned::src::openaptx::aptx_finish;
use ::libopenaptx_cleaned::src::openaptx::aptx_init;
pub use libopenaptx_cleaned::src::openaptx::aptx_context;

static mut input_buffer: [c_uchar; 3072] = [0; 3072];
static mut output_buffer: [c_uchar; 18456] = [0; 18456];
unsafe fn main_0(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) -> c_int {
    let mut i: c_int = 0;
    let mut hd: c_int = 0;
    let mut ret: c_int = 0;
    let mut length: size_t = 0;
    let mut processed: size_t = 0;
    let mut written: size_t = 0;
    let mut dropped: size_t = 0;
    let mut synced: c_int = 0;
    let mut syncing: c_int = 0;
    let mut ctx: *mut aptx_context = ::core::ptr::null_mut::<aptx_context>();
    hd = 0 as c_int;
    i = 1 as c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const c_char,
        ) == 0 as c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--help\0" as *const u8 as *const c_char,
            ) == 0 as c_int
        {
            fprintf(
                stderr,
                b"aptX decoder utility %d.%d.%d (using libopenaptx %d.%d.%d)\n\0" as *const u8
                    as *const c_char,
                OPENAPTX_MAJOR,
                OPENAPTX_MINOR,
                OPENAPTX_PATCH,
                aptx_major,
                aptx_minor,
                aptx_patch,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"This utility decodes aptX or aptX HD audio stream\n\0" as *const u8
                    as *const c_char,
            );
            fprintf(
                stderr,
                b"from stdin to a raw 24 bit signed stereo on stdout\n\0" as *const u8
                    as *const c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"When input is damaged it tries to synchronize and recover\n\0" as *const u8
                    as *const c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"Non-zero return value indicates that input was damaged\n\0" as *const u8
                    as *const c_char,
            );
            fprintf(
                stderr,
                b"and some bytes from input aptX audio stream were dropped\n\0" as *const u8
                    as *const c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"Usage:\n\0" as *const u8 as *const c_char,
            );
            fprintf(
                stderr,
                b"        %s [options]\n\0" as *const u8 as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"Options:\n\0" as *const u8 as *const c_char,
            );
            fprintf(
                stderr,
                b"        -h, --help   Display this help\n\0" as *const u8
                    as *const c_char,
            );
            fprintf(
                stderr,
                b"        --hd         Decode from aptX HD\n\0" as *const u8
                    as *const c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"Examples:\n\0" as *const u8 as *const c_char,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"        %s < sample.aptx > sample.s24le\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"        %s --hd < sample.aptxhd > sample.s24le\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"        %s < sample.aptx | play -t raw -r 44.1k -L -e s -b 24 -c 2 -\n\0"
                    as *const u8 as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            return 1 as c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--hd\0" as *const u8 as *const c_char,
        ) == 0 as c_int
        {
            hd = 1 as c_int;
        } else {
            fprintf(
                stderr,
                b"%s: Invalid option %s\n\0" as *const u8 as *const c_char,
                *argv.offset(0 as c_int as isize),
                *argv.offset(i as isize),
            );
            return 1 as c_int;
        }
        i += 1;
    }
    ctx = aptx_init(hd);
    if ctx.is_null() {
        fprintf(
            stderr,
            b"%s: Cannot initialize aptX decoder\n\0" as *const u8 as *const c_char,
            *argv.offset(0 as c_int as isize),
        );
        return 1 as c_int;
    }
    length = fread(
        &raw mut input_buffer as *mut c_uchar as *mut c_void,
        1 as size_t,
        6 as size_t,
        stdin,
    ) as size_t;
    if length >= 4 as size_t
        && memcmp(
            &raw mut input_buffer as *mut c_uchar as *const c_void,
            b"K\xBFK\xBF\0" as *const u8 as *const c_char
                as *const c_void,
            4 as size_t,
        ) == 0 as c_int
    {
        if hd != 0 {
            fprintf(
                stderr,
                b"%s: Input looks like start of aptX audio stream (not aptX HD), try without --hd\n\0"
                    as *const u8 as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
        }
    } else if length >= 6 as size_t
        && memcmp(
            &raw mut input_buffer as *mut c_uchar as *const c_void,
            b"s\xBE\xFFs\xBE\xFF\0" as *const u8 as *const c_char
                as *const c_void,
            6 as size_t,
        ) == 0 as c_int
    {
        if hd == 0 {
            fprintf(
                stderr,
                b"%s: Input looks like start of aptX HD audio stream, try with --hd\n\0"
                    as *const u8 as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
        }
    } else if length >= 4 as size_t
        && memcmp(
            &raw mut input_buffer as *mut c_uchar as *const c_void,
            b"k\xBFk\xBF\0" as *const u8 as *const c_char
                as *const c_void,
            4 as size_t,
        ) == 0 as c_int
    {
        fprintf(
            stderr,
            b"%s: Input looks like start of standard aptX audio stream, which is not supported yet\n\0"
                as *const u8 as *const c_char,
            *argv.offset(0 as c_int as isize),
        );
    } else {
        fprintf(
            stderr,
            b"%s: Input does not look like start of aptX nor aptX HD audio stream\n\0" as *const u8
                as *const c_char,
            *argv.offset(0 as c_int as isize),
        );
    }
    ret = 0 as c_int;
    syncing = 0 as c_int;
    while length > 0 as size_t {
        processed = aptx_decode_sync(
            ctx,
            &raw mut input_buffer as *mut c_uchar,
            length,
            &raw mut output_buffer as *mut c_uchar,
            ::core::mem::size_of::<[c_uchar; 18456]>() as size_t,
            &raw mut written,
            &raw mut synced,
            &raw mut dropped,
        );
        if synced == 0 {
            if syncing == 0 {
                fprintf(
                    stderr,
                    b"%s: aptX decoding failed, synchronizing\n\0" as *const u8
                        as *const c_char,
                    *argv.offset(0 as c_int as isize),
                );
                syncing = 1 as c_int;
                ret = 1 as c_int;
            }
            if dropped != 0 {
                fprintf(
                    stderr,
                    b"%s: aptX synchronization successful, dropped %lu byte%s\n\0" as *const u8
                        as *const c_char,
                    *argv.offset(0 as c_int as isize),
                    dropped as c_ulong,
                    if dropped != 1 as size_t {
                        b"s\0" as *const u8 as *const c_char
                    } else {
                        b"\0" as *const u8 as *const c_char
                    },
                );
                syncing = 0 as c_int;
                ret = 1 as c_int;
            }
            if syncing == 0 {
                fprintf(
                    stderr,
                    b"%s: aptX decoding failed, synchronizing\n\0" as *const u8
                        as *const c_char,
                    *argv.offset(0 as c_int as isize),
                );
                syncing = 1 as c_int;
                ret = 1 as c_int;
            }
        } else if dropped != 0 {
            if syncing == 0 {
                fprintf(
                    stderr,
                    b"%s: aptX decoding failed, synchronizing\n\0" as *const u8
                        as *const c_char,
                    *argv.offset(0 as c_int as isize),
                );
            }
            fprintf(
                stderr,
                b"%s: aptX synchronization successful, dropped %lu byte%s\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
                dropped as c_ulong,
                if dropped != 1 as size_t {
                    b"s\0" as *const u8 as *const c_char
                } else {
                    b"\0" as *const u8 as *const c_char
                },
            );
            syncing = 0 as c_int;
            ret = 1 as c_int;
        } else if syncing != 0 {
            fprintf(
                stderr,
                b"%s: aptX synchronization successful\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            syncing = 0 as c_int;
            ret = 1 as c_int;
        }
        if processed != length {
            fprintf(
                stderr,
                b"%s: aptX decoding failed\n\0" as *const u8 as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            ret = 1 as c_int;
            break;
        } else {
            if feof(stdin) == 0 {
                length = fread(
                    &raw mut input_buffer as *mut c_uchar as *mut c_void,
                    1 as size_t,
                    ::core::mem::size_of::<[c_uchar; 3072]>() as size_t,
                    stdin,
                ) as size_t;
                if ferror(stdin) != 0 {
                    fprintf(
                        stderr,
                        b"%s: aptX decoding failed to read input data\n\0" as *const u8
                            as *const c_char,
                        *argv.offset(0 as c_int as isize),
                    );
                    ret = 1 as c_int;
                    length = 0 as size_t;
                }
            } else {
                length = 0 as size_t;
            }
            if length == 0 as size_t
                && ferror(stdin) == 0
                && written >= (6 as c_int * 2 as c_int) as size_t
            {
                written = written
                    .wrapping_sub((6 as c_int * 2 as c_int) as size_t);
            }
            if !(written > 0 as size_t) {
                continue;
            }
            if !(fwrite(
                &raw mut output_buffer as *mut c_uchar as *const c_void,
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
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            ret = 1 as c_int;
            break;
        }
    }
    dropped = aptx_decode_sync_finish(ctx);
    if dropped != 0 && syncing == 0 {
        fprintf(
            stderr,
            b"%s: aptX decoding stopped in the middle of the sample, dropped %lu byte%s\n\0"
                as *const u8 as *const c_char,
            *argv.offset(0 as c_int as isize),
            dropped as c_ulong,
            if dropped != 1 as size_t {
                b"s\0" as *const u8 as *const c_char
            } else {
                b"\0" as *const u8 as *const c_char
            },
        );
        ret = 1 as c_int;
    } else if syncing != 0 {
        fprintf(
            stderr,
            b"%s: aptX synchronization failed\n\0" as *const u8 as *const c_char,
            *argv.offset(0 as c_int as isize),
        );
        ret = 1 as c_int;
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
