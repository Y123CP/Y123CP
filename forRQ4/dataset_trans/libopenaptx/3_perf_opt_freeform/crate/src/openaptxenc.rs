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
use ::libopenaptx_cleaned::src::openaptx::aptx_encode;
use ::libopenaptx_cleaned::src::openaptx::aptx_encode_finish;
use ::libopenaptx_cleaned::src::openaptx::aptx_finish;
use ::libopenaptx_cleaned::src::openaptx::aptx_init;
pub use libopenaptx_cleaned::src::openaptx::aptx_context;

static mut input_buffer: [c_uchar; 12288] = [0; 12288];
static mut output_buffer: [c_uchar; 3072] = [0; 3072];
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
                b"aptX encoder utility %d.%d.%d (using libopenaptx %d.%d.%d)\n\0" as *const u8
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
                b"This utility encodes a raw 24 bit signed stereo\n\0" as *const u8
                    as *const c_char,
            );
            fprintf(
                stderr,
                b"samples from stdin to aptX or aptX HD on stdout\n\0" as *const u8
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
                b"        --hd         Encode to aptX HD\n\0" as *const u8
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
                b"        %s < sample.s24le > sample.aptx\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"        %s --hd < sample.s24le > sample.aptxhd\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            fprintf(
                stderr,
                b"        sox sample.wav -t raw -r 44.1k -L -e s -b 24 -c 2 - | %s > sample.aptx\n\0"
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
            b"%s: Cannot initialize aptX encoder\n\0" as *const u8 as *const c_char,
            *argv.offset(0 as c_int as isize),
        );
        return 1 as c_int;
    }
    ret = 0 as c_int;
    while feof(stdin) == 0 {
        length = fread(
            &raw mut input_buffer as *mut c_uchar as *mut c_void,
            1 as size_t,
            ::core::mem::size_of::<[c_uchar; 12288]>() as size_t,
            stdin,
        ) as size_t;
        if ferror(stdin) != 0 {
            fprintf(
                stderr,
                b"%s: aptX encoding failed to read input data\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            ret = 1 as c_int;
        }
        if length == 0 as size_t {
            break;
        }
        processed = aptx_encode(
            ctx,
            &raw mut input_buffer as *mut c_uchar,
            length,
            &raw mut output_buffer as *mut c_uchar,
            ::core::mem::size_of::<[c_uchar; 3072]>() as size_t,
            &raw mut written,
        );
        if processed != length {
            fprintf(
                stderr,
                b"%s: aptX encoding stopped in the middle of the sample, dropped %lu byte%s\n\0"
                    as *const u8 as *const c_char,
                *argv.offset(0 as c_int as isize),
                length.wrapping_sub(processed) as c_ulong,
                if length.wrapping_sub(processed) != 1 as size_t {
                    b"s\0" as *const u8 as *const c_char
                } else {
                    b"\0" as *const u8 as *const c_char
                },
            );
            ret = 1 as c_int;
        }
        if fwrite(
            &raw mut output_buffer as *mut c_uchar as *const c_void,
            1 as size_t,
            written,
            stdout,
        ) as size_t
            != written
        {
            fprintf(
                stderr,
                b"%s: aptX encoding failed to write encoded data\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            ret = 1 as c_int;
            break;
        } else if processed != length {
            break;
        }
    }
    if aptx_encode_finish(
        ctx,
        &raw mut output_buffer as *mut c_uchar,
        ::core::mem::size_of::<[c_uchar; 3072]>() as size_t,
        &raw mut written,
    ) != 0
    {
        if fwrite(
            &raw mut output_buffer as *mut c_uchar as *const c_void,
            1 as size_t,
            written,
            stdout,
        ) as size_t
            != written
        {
            fprintf(
                stderr,
                b"%s: aptX encoding failed to write encoded data\n\0" as *const u8
                    as *const c_char,
                *argv.offset(0 as c_int as isize),
            );
            ret = 1 as c_int;
        }
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
