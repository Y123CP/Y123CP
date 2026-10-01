#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(raw_ref_op)]
#![feature(stdsimd)]
use core::ffi::*;
use ::brotli_cleaned::src::ffi::*;
use ::brotli_cleaned::src::c_consts::*;
use ::brotli_cleaned::src::c_types::*;
use ::brotli_cleaned::src::c_extern_types::*;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type BrotliDecoderStateStruct;
    pub type BrotliEncoderStateStruct;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> c_int;
    fn fflush(__stream: *mut FILE) -> c_int;
    fn fopen(
        __filename: *const c_char,
        __modes: *const c_char,
    ) -> *mut FILE;
    fn fdopen(__fd: c_int, __modes: *const c_char) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const c_char,
        ...
    ) -> c_int;
    fn fgetc(__stream: *mut FILE) -> c_int;
    fn fread(
        __ptr: *mut c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> c_ulong;
    fn fwrite(
        __ptr: *const c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: c_long,
        __whence: c_int,
    ) -> c_int;
    fn ftell(__stream: *mut FILE) -> c_long;
    fn feof(__stream: *mut FILE) -> c_int;
    fn ferror(__stream: *mut FILE) -> c_int;
    fn fileno(__stream: *mut FILE) -> c_int;
    fn fchmod(__fd: c_int, __mode: __mode_t) -> c_int;
    fn futimens(__fd: c_int, __times: *const timespec) -> c_int;
    fn __xstat(
        __ver: c_int,
        __filename: *const c_char,
        __stat_buf: *mut stat,
    ) -> c_int;
    fn clock() -> clock_t;
    fn BrotliDecoderSetParameter(
        state: *mut BrotliDecoderState,
        param: BrotliDecoderParameter,
        value: uint32_t,
    ) -> c_int;
    fn BrotliDecoderAttachDictionary(
        state: *mut BrotliDecoderState,
        type_0: BrotliSharedDictionaryType,
        data_size: size_t,
        data: *const uint8_t,
    ) -> c_int;
    fn BrotliDecoderCreateInstance(
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut c_void,
    ) -> *mut BrotliDecoderState;
    fn BrotliDecoderDestroyInstance(state: *mut BrotliDecoderState);
    fn BrotliDecoderDecompressStream(
        state: *mut BrotliDecoderState,
        available_in: *mut size_t,
        next_in: *mut *const uint8_t,
        available_out: *mut size_t,
        next_out: *mut *mut uint8_t,
        total_out: *mut size_t,
    ) -> BrotliDecoderResult;
    fn BrotliDecoderGetErrorCode(state: *const BrotliDecoderState) -> BrotliDecoderErrorCode;
    fn BrotliDecoderErrorString(c: BrotliDecoderErrorCode) -> *const c_char;
    fn BrotliDecoderSetMetadataCallbacks(
        state: *mut BrotliDecoderState,
        start_func: brotli_decoder_metadata_start_func,
        chunk_func: brotli_decoder_metadata_chunk_func,
        opaque: *mut c_void,
    );
    fn BrotliEncoderSetParameter(
        state: *mut BrotliEncoderState,
        param: BrotliEncoderParameter,
        value: uint32_t,
    ) -> c_int;
    fn BrotliEncoderCreateInstance(
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut c_void,
    ) -> *mut BrotliEncoderState;
    fn BrotliEncoderDestroyInstance(state: *mut BrotliEncoderState);
    fn BrotliEncoderPrepareDictionary(
        type_0: BrotliSharedDictionaryType,
        data_size: size_t,
        data: *const uint8_t,
        quality: c_int,
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut c_void,
    ) -> *mut BrotliEncoderPreparedDictionary;
    fn BrotliEncoderDestroyPreparedDictionary(dictionary: *mut BrotliEncoderPreparedDictionary);
    fn BrotliEncoderAttachPreparedDictionary(
        state: *mut BrotliEncoderState,
        dictionary: *const BrotliEncoderPreparedDictionary,
    ) -> c_int;
    fn BrotliEncoderCompressStream(
        state: *mut BrotliEncoderState,
        op: BrotliEncoderOperation,
        available_in: *mut size_t,
        next_in: *mut *const uint8_t,
        available_out: *mut size_t,
        next_out: *mut *mut uint8_t,
        total_out: *mut size_t,
    ) -> c_int;
    fn BrotliEncoderIsFinished(state: *mut BrotliEncoderState) -> c_int;
    fn fchown(__fd: c_int, __owner: __uid_t, __group: __gid_t) -> c_int;
}

pub type __dev_t = c_ulong;
pub type __uid_t = c_uint;
pub type __gid_t = c_uint;
pub type __ino_t = c_ulong;
pub type __mode_t = c_uint;
pub type __nlink_t = c_ulong;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type __clock_t = c_long;
pub type __time_t = c_long;
pub type __blksize_t = c_long;
pub type __blkcnt_t = c_long;
pub type __syscall_slong_t = c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type clock_t = __clock_t;
pub type BrotliDecoderState = BrotliDecoderStateStruct;

pub type BrotliEncoderState = BrotliEncoderStateStruct;

pub type Command = c_uint;
pub const COMMAND_VERSION: Command = 6;
pub const COMMAND_NOOP: Command = 5;
pub const COMMAND_TEST_INTEGRITY: Command = 4;
pub const COMMAND_INVALID: Command = 3;
pub const COMMAND_HELP: Command = 2;
pub const COMMAND_DECOMPRESS: Command = 1;
pub const COMMAND_COMPRESS: Command = 0;
pub type CommentState = c_uint;
pub const COMMENT_BAD: CommentState = 3;
pub const COMMENT_OK: CommentState = 2;
pub const COMMENT_READ: CommentState = 1;
pub const COMMENT_INIT: CommentState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Context {
    pub quality: c_int,
    pub lgwin: c_int,
    pub verbosity: c_int,
    pub force_overwrite: c_int,
    pub junk_source: c_int,
    pub reject_uncompressible: c_int,
    pub copy_stat: c_int,
    pub write_to_stdout: c_int,
    pub test_integrity: c_int,
    pub decompress: c_int,
    pub large_window: c_int,
    pub allow_concatenated: c_int,
    pub output_path: *const c_char,
    pub dictionary_path: *const c_char,
    pub suffix: *const c_char,
    pub comment: [uint8_t; 80],
    pub comment_len: size_t,
    pub comment_pos: size_t,
    pub comment_state: CommentState,
    pub not_input_indices: [c_int; 24],
    pub longest_path_len: size_t,
    pub input_count: size_t,
    pub argc: c_int,
    pub argv: *mut *mut c_char,
    pub dictionary: *mut uint8_t,
    pub dictionary_size: size_t,
    pub prepared_dictionary: *mut BrotliEncoderPreparedDictionary,
    pub decoder: *mut BrotliDecoderState,
    pub modified_path: *mut c_char,
    pub iterator: c_int,
    pub ignore: c_int,
    pub iterator_error: c_int,
    pub buffer: *mut uint8_t,
    pub input: *mut uint8_t,
    pub output: *mut uint8_t,
    pub current_input_path: *const c_char,
    pub current_output_path: *const c_char,
    pub input_file_length: int64_t,
    pub fin: *mut FILE,
    pub fout: *mut FILE,
    pub available_in: size_t,
    pub next_in: *const uint8_t,
    pub available_out: size_t,
    pub next_out: *mut uint8_t,
    pub total_in: size_t,
    pub total_out: size_t,
    pub start_time: clock_t,
    pub end_time: clock_t,
}
static mut kMaxDictionarySize: c_int = 0;

pub const O_WRONLY: c_int = 0o1 as c_int;
pub const O_CREAT: c_int = 0o100 as c_int;
pub const O_EXCL: c_int = 0o200 as c_int;
pub const O_TRUNC: c_int = 0o1000 as c_int;

pub const _STAT_VER_LINUX: c_int = 1 as c_int;
pub const _STAT_VER: c_int = _STAT_VER_LINUX;
pub const __S_IREAD: c_int = 0o400 as c_int;
pub const __S_IWRITE: c_int = 0o200 as c_int;
pub const __S_IEXEC: c_int = 0o100 as c_int;
pub const EOF: c_int = -(1 as c_int);
pub const SEEK_END: c_int = 2 as c_int;
pub const CLOCKS_PER_SEC: __clock_t = 1000000 as c_int as __clock_t;
pub const S_IRUSR: c_int = __S_IREAD;
pub const S_IWUSR: c_int = __S_IWRITE;
pub const S_IRWXU: c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const S_IRWXG: c_int = S_IRWXU >> 3 as c_int;
pub const S_IRWXO: c_int = S_IRWXG >> 3 as c_int;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const c_char,
    mut __statbuf: *mut stat,
) -> c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
pub const BROTLI_VERSION_MAJOR: c_int = 1 as c_int;
pub const BROTLI_VERSION_MINOR: c_int = 2 as c_int;
pub const BROTLI_VERSION_PATCH: c_int = 0 as c_int;

pub const BROTLI_MIN_QUALITY: c_int = 0 as c_int;

pub const STDIN_FILENO: c_int = 0 as c_int;
pub const STDOUT_FILENO: c_int = 1 as c_int;
pub const DEFAULT_LGWIN: c_int = 24 as c_int;
pub const DEFAULT_SUFFIX: [c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [c_char; 4]>(*b".br\0") };
pub const MAX_OPTIONS: c_int = 24 as c_int;
pub const MAX_COMMENT_LEN: c_int = 80 as c_int;
unsafe extern "C" fn ParseBase64(
    mut str: *const c_char,
    mut out: *mut uint8_t,
    mut out_len: *mut size_t,
) -> c_int {
    let mut in_len: size_t = strlen(str);
    let mut max_out_len: size_t = *out_len;
    let mut i: size_t = 0;
    let mut bit_count: size_t = 0 as size_t;
    let mut bits: uint32_t = 0 as uint32_t;
    let mut padding_count: size_t = 0 as size_t;
    let mut octet_count: size_t = 0 as size_t;
    i = 0 as size_t;
    while i < in_len {
        let mut c: c_char = *str.offset(i as isize);
        let mut sextet: c_int = 0 as c_int;
        if !(c as c_int == 9 as c_int
            || c as c_int == 10 as c_int
            || c as c_int == 13 as c_int
            || c as c_int == ' ' as i32)
        {
            if c as c_int == '=' as i32 {
                padding_count = padding_count.wrapping_add(1);
            } else {
                if padding_count != 0 {
                    return BROTLI_FALSE;
                }
                if c as c_int == '+' as i32 || c as c_int == '-' as i32 {
                    sextet = 62 as c_int;
                } else if c as c_int == '/' as i32
                    || c as c_int == '_' as i32
                {
                    sextet = 63 as c_int;
                } else if c as c_int >= 'A' as i32
                    && c as c_int <= 'Z' as i32
                {
                    sextet = c as c_int - 'A' as i32;
                } else if c as c_int >= 'a' as i32
                    && c as c_int <= 'z' as i32
                {
                    sextet = c as c_int - 'a' as i32 + 26 as c_int;
                } else if c as c_int >= '0' as i32
                    && c as c_int <= '9' as i32
                {
                    sextet = c as c_int - '0' as i32 + 52 as c_int;
                } else {
                    return BROTLI_FALSE;
                }
                bits = bits << 6 as c_int | sextet as uint32_t;
                bit_count = (bit_count as c_ulong)
                    .wrapping_add(6 as c_ulong) as size_t
                    as size_t;
                if bit_count >= 8 as size_t {
                    if octet_count == max_out_len {
                        return BROTLI_FALSE;
                    }
                    bit_count = (bit_count as c_ulong)
                        .wrapping_sub(8 as c_ulong)
                        as size_t as size_t;
                    let fresh0 = octet_count;
                    octet_count = octet_count.wrapping_add(1);
                    *out.offset(fresh0 as isize) =
                        (bits >> bit_count & 0xff as uint32_t) as uint8_t;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if padding_count > 2 as size_t {
        return BROTLI_FALSE;
    }
    *out_len = octet_count;
    return BROTLI_TRUE;
}
unsafe extern "C" fn ParseInt(
    mut s: *const c_char,
    mut low: c_int,
    mut high: c_int,
    mut result: *mut c_int,
) -> c_int {
    let mut value: c_int = 0 as c_int;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < 5 as c_int {
        let mut c: c_char = *s.offset(i as isize);
        if c as c_int == 0 as c_int {
            break;
        }
        if (*s.offset(i as isize) as c_int) < '0' as i32
            || *s.offset(i as isize) as c_int > '9' as i32
        {
            return BROTLI_FALSE;
        }
        value = 10 as c_int * value + (c as c_int - '0' as i32);
        i += 1;
    }
    if i == 0 as c_int {
        return BROTLI_FALSE;
    }
    if i > 1 as c_int
        && *s.offset(0 as c_int as isize) as c_int == '0' as i32
    {
        return BROTLI_FALSE;
    }
    if *s.offset(i as isize) as c_int != 0 as c_int {
        return BROTLI_FALSE;
    }
    if value < low || value > high {
        return BROTLI_FALSE;
    }
    *result = value;
    return BROTLI_TRUE;
}
unsafe extern "C" fn FileName(mut path: *const c_char) -> *const c_char {
    let mut separator_position: *const c_char = strrchr(path, '/' as i32);
    if !separator_position.is_null() {
        path = separator_position.offset(1 as c_int as isize);
    }
    separator_position = strrchr(path, '\\' as i32);
    if !separator_position.is_null() {
        path = separator_position.offset(1 as c_int as isize);
    }
    return path;
}
unsafe extern "C" fn CheckAlias(
    mut name: *const c_char,
    mut alias: *const c_char,
) -> c_int {
    let mut alias_len: size_t = strlen(alias);
    name = FileName(name);
    if strncmp(name, alias, alias_len) == 0 as c_int {
        let mut terminator: c_char = *name.offset(alias_len as isize);
        if terminator as c_int == 0 as c_int
            || terminator as c_int == '.' as i32
        {
            return BROTLI_TRUE;
        }
    }
    return BROTLI_FALSE;
}
unsafe extern "C" fn ParseParams(mut params: *mut Context) -> Command {
    let mut argc: c_int = (*params).argc;
    let mut argv: *mut *mut c_char = (*params).argv;
    let mut i: c_int = 0;
    let mut next_option_index: c_int = 0 as c_int;
    let mut input_count: size_t = 0 as size_t;
    let mut longest_path_len: size_t = 1 as size_t;
    let mut command_set: c_int = BROTLI_FALSE;
    let mut quality_set: c_int = BROTLI_FALSE;
    let mut output_set: c_int = BROTLI_FALSE;
    let mut keep_set: c_int = BROTLI_FALSE;
    let mut squash_set: c_int = BROTLI_FALSE;
    let mut lgwin_set: c_int = BROTLI_FALSE;
    let mut suffix_set: c_int = BROTLI_FALSE;
    let mut after_dash_dash: c_int = BROTLI_FALSE;
    let mut comment_set: c_int = BROTLI_FALSE;
    let mut concatenated_set: c_int = BROTLI_FALSE;
    let mut command: Command = COMMAND_COMPRESS;
    if CheckAlias(
        *argv.offset(0 as c_int as isize),
        b"brcat\0" as *const u8 as *const c_char,
    ) != 0
    {
        command_set = BROTLI_TRUE;
        command = COMMAND_DECOMPRESS;
        concatenated_set = BROTLI_TRUE;
        (*params).allow_concatenated = BROTLI_TRUE;
        output_set = BROTLI_TRUE;
        (*params).write_to_stdout = BROTLI_TRUE;
    } else if CheckAlias(
        *argv.offset(0 as c_int as isize),
        b"unbrotli\0" as *const u8 as *const c_char,
    ) != 0
    {
        command_set = BROTLI_TRUE;
        command = COMMAND_DECOMPRESS;
    }
    i = 1 as c_int;
    while i < argc {
        let mut arg: *const c_char = *argv.offset(i as isize);
        let mut arg_len: size_t = if !arg.is_null() {
            strlen(arg)
        } else {
            0 as size_t
        };
        if arg_len == 0 as size_t {
            let fresh1 = next_option_index;
            next_option_index = next_option_index + 1;
            (*params).not_input_indices[fresh1 as usize] = i;
        } else {
            if next_option_index > MAX_OPTIONS - 2 as c_int {
                fprintf(
                    stderr,
                    b"too many options passed\n\0" as *const u8 as *const c_char,
                );
                return COMMAND_INVALID;
            }
            if after_dash_dash != 0
                || *arg.offset(0 as c_int as isize) as c_int != '-' as i32
                || arg_len == 1 as size_t
            {
                input_count = input_count.wrapping_add(1);
                if longest_path_len < arg_len {
                    longest_path_len = arg_len;
                }
            } else {
                let fresh2 = next_option_index;
                next_option_index = next_option_index + 1;
                (*params).not_input_indices[fresh2 as usize] = i;
                if arg_len == 2 as size_t
                    && *arg.offset(1 as c_int as isize) as c_int
                        == '-' as i32
                {
                    after_dash_dash = BROTLI_TRUE;
                } else if *arg.offset(1 as c_int as isize) as c_int
                    != '-' as i32
                {
                    let mut j: size_t = 0;
                    j = 1 as size_t;
                    while j < arg_len {
                        let mut c: c_char = *arg.offset(j as isize);
                        if c as c_int >= '0' as i32
                            && c as c_int <= '9' as i32
                        {
                            if quality_set != 0 {
                                fprintf(
                                    stderr,
                                    b"quality already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            quality_set = BROTLI_TRUE;
                            (*params).quality = c as c_int - '0' as i32;
                        } else if c as c_int == 'c' as i32 {
                            if output_set != 0 {
                                fprintf(
                                    stderr,
                                    b"write to standard output already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            output_set = BROTLI_TRUE;
                            (*params).write_to_stdout = BROTLI_TRUE;
                        } else if c as c_int == 'd' as i32 {
                            if command_set != 0 {
                                fprintf(
                                    stderr,
                                    b"command already set when parsing -d\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            command_set = BROTLI_TRUE;
                            command = COMMAND_DECOMPRESS;
                        } else if c as c_int == 'f' as i32 {
                            if (*params).force_overwrite != 0 {
                                fprintf(
                                    stderr,
                                    b"force output overwrite already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params).force_overwrite = BROTLI_TRUE;
                        } else if c as c_int == 'h' as i32 {
                            return COMMAND_HELP;
                        } else if c as c_int == 'j' as i32
                            || c as c_int == 'k' as i32
                        {
                            if keep_set != 0 {
                                fprintf(
                                    stderr,
                                    b"argument --rm / -j or --keep / -k already set\n\0"
                                        as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            keep_set = BROTLI_TRUE;
                            (*params).junk_source = if c as c_int == 'j' as i32 {
                                BROTLI_TRUE
                            } else {
                                BROTLI_FALSE
                            };
                        } else if c as c_int == 'n' as i32 {
                            if (*params).copy_stat == 0 {
                                fprintf(
                                    stderr,
                                    b"argument --no-copy-stat / -n already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params).copy_stat = BROTLI_FALSE;
                        } else if c as c_int == 's' as i32 {
                            if squash_set != 0 {
                                fprintf(
                                    stderr,
                                    b"argument --squash / -s already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            squash_set = BROTLI_TRUE;
                            (*params).reject_uncompressible = BROTLI_TRUE;
                        } else if c as c_int == 't' as i32 {
                            if command_set != 0 {
                                fprintf(
                                    stderr,
                                    b"command already set when parsing -t\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            command_set = BROTLI_TRUE;
                            command = COMMAND_TEST_INTEGRITY;
                        } else if c as c_int == 'v' as i32 {
                            if (*params).verbosity > 0 as c_int {
                                fprintf(
                                    stderr,
                                    b"argument --verbose / -v already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params).verbosity = 1 as c_int;
                        } else if c as c_int == 'K' as i32 {
                            if concatenated_set != 0 {
                                fprintf(
                                    stderr,
                                    b"argument -K / --concatenated already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            concatenated_set = BROTLI_TRUE;
                            (*params).allow_concatenated = BROTLI_TRUE;
                        } else if c as c_int == 'V' as i32 {
                            return COMMAND_VERSION;
                        } else if c as c_int == 'Z' as i32 {
                            if quality_set != 0 {
                                fprintf(
                                    stderr,
                                    b"quality already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            quality_set = BROTLI_TRUE;
                            (*params).quality = 11 as c_int;
                        } else {
                            if c as c_int != 'o' as i32
                                && c as c_int != 'q' as i32
                                && c as c_int != 'w' as i32
                                && c as c_int != 'C' as i32
                                && c as c_int != 'D' as i32
                                && c as c_int != 'S' as i32
                            {
                                fprintf(
                                    stderr,
                                    b"invalid argument -%c\n\0" as *const u8
                                        as *const c_char,
                                    c as c_int,
                                );
                                return COMMAND_INVALID;
                            }
                            if j.wrapping_add(1 as size_t) != arg_len {
                                fprintf(
                                    stderr,
                                    b"expected parameter for argument -%c\n\0" as *const u8
                                        as *const c_char,
                                    c as c_int,
                                );
                                return COMMAND_INVALID;
                            }
                            i += 1;
                            if i == argc
                                || (*argv.offset(i as isize)).is_null()
                                || *(*argv.offset(i as isize))
                                    .offset(0 as c_int as isize)
                                    as c_int
                                    == 0 as c_int
                            {
                                fprintf(
                                    stderr,
                                    b"expected parameter for argument -%c\n\0" as *const u8
                                        as *const c_char,
                                    c as c_int,
                                );
                                return COMMAND_INVALID;
                            }
                            let fresh3 = next_option_index;
                            next_option_index = next_option_index + 1;
                            (*params).not_input_indices[fresh3 as usize] = i;
                            if c as c_int == 'o' as i32 {
                                if output_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"write to standard output already set (-o)\n\0"
                                            as *const u8
                                            as *const c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                (*params).output_path = *argv.offset(i as isize);
                            } else if c as c_int == 'q' as i32 {
                                if quality_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"quality already set\n\0" as *const u8
                                            as *const c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                quality_set = ParseInt(
                                    *argv.offset(i as isize),
                                    BROTLI_MIN_QUALITY,
                                    BROTLI_MAX_QUALITY,
                                    &raw mut (*params).quality,
                                );
                                if quality_set == 0 {
                                    fprintf(
                                        stderr,
                                        b"error parsing quality value [%s]\n\0" as *const u8
                                            as *const c_char,
                                        *argv.offset(i as isize),
                                    );
                                    return COMMAND_INVALID;
                                }
                            } else if c as c_int == 'w' as i32 {
                                if lgwin_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"lgwin parameter already set\n\0" as *const u8
                                            as *const c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                lgwin_set = ParseInt(
                                    *argv.offset(i as isize),
                                    0 as c_int,
                                    BROTLI_MAX_WINDOW_BITS,
                                    &raw mut (*params).lgwin,
                                );
                                if lgwin_set == 0 {
                                    fprintf(
                                        stderr,
                                        b"error parsing lgwin value [%s]\n\0" as *const u8
                                            as *const c_char,
                                        *argv.offset(i as isize),
                                    );
                                    return COMMAND_INVALID;
                                }
                                if (*params).lgwin != 0 as c_int
                                    && (*params).lgwin < BROTLI_MIN_WINDOW_BITS
                                {
                                    fprintf(
                                        stderr,
                                        b"lgwin parameter (%d) smaller than the minimum (%d)\n\0"
                                            as *const u8
                                            as *const c_char,
                                        (*params).lgwin,
                                        BROTLI_MIN_WINDOW_BITS,
                                    );
                                    return COMMAND_INVALID;
                                }
                            } else if c as c_int == 'C' as i32 {
                                if comment_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"comment already set\n\0" as *const u8
                                            as *const c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                (*params).comment_len = MAX_COMMENT_LEN as size_t;
                                if ParseBase64(
                                    *argv.offset(i as isize),
                                    &raw mut (*params).comment as *mut uint8_t,
                                    &raw mut (*params).comment_len,
                                ) == 0
                                {
                                    fprintf(
                                        stderr,
                                        b"invalid base64-encoded comment\n\0" as *const u8
                                            as *const c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                comment_set = BROTLI_TRUE;
                            } else if c as c_int == 'D' as i32 {
                                if !(*params).dictionary_path.is_null() {
                                    fprintf(
                                        stderr,
                                        b"dictionary path already set\n\0" as *const u8
                                            as *const c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                (*params).dictionary_path = *argv.offset(i as isize);
                            } else if c as c_int == 'S' as i32 {
                                if suffix_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"suffix already set\n\0" as *const u8
                                            as *const c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                suffix_set = BROTLI_TRUE;
                                (*params).suffix = *argv.offset(i as isize);
                            }
                        }
                        j = j.wrapping_add(1);
                    }
                } else {
                    arg =
                        arg.offset(2 as c_int as isize) as *const c_char;
                    if strcmp(b"best\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        if quality_set != 0 {
                            fprintf(
                                stderr,
                                b"quality already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        quality_set = BROTLI_TRUE;
                        (*params).quality = 11 as c_int;
                    } else if strcmp(
                        b"concatenated\0" as *const u8 as *const c_char,
                        arg,
                    ) == 0 as c_int
                    {
                        if concatenated_set != 0 {
                            fprintf(
                                stderr,
                                b"argument -K / --concatenated already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        concatenated_set = BROTLI_TRUE;
                        (*params).allow_concatenated = BROTLI_TRUE;
                    } else if strcmp(
                        b"decompress\0" as *const u8 as *const c_char,
                        arg,
                    ) == 0 as c_int
                    {
                        if command_set != 0 {
                            fprintf(
                                stderr,
                                b"command already set when parsing --decompress\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        command_set = BROTLI_TRUE;
                        command = COMMAND_DECOMPRESS;
                    } else if strcmp(b"force\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        if (*params).force_overwrite != 0 {
                            fprintf(
                                stderr,
                                b"force output overwrite already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        (*params).force_overwrite = BROTLI_TRUE;
                    } else if strcmp(b"help\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        return COMMAND_HELP;
                    } else if strcmp(b"keep\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        if keep_set != 0 {
                            fprintf(
                                stderr,
                                b"argument --rm / -j or --keep / -k already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        keep_set = BROTLI_TRUE;
                        (*params).junk_source = BROTLI_FALSE;
                    } else if strcmp(
                        b"no-copy-stat\0" as *const u8 as *const c_char,
                        arg,
                    ) == 0 as c_int
                    {
                        if (*params).copy_stat == 0 {
                            fprintf(
                                stderr,
                                b"argument --no-copy-stat / -n already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        (*params).copy_stat = BROTLI_FALSE;
                    } else if strcmp(b"rm\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        if keep_set != 0 {
                            fprintf(
                                stderr,
                                b"argument --rm / -j or --keep / -k already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        keep_set = BROTLI_TRUE;
                        (*params).junk_source = BROTLI_TRUE;
                    } else if strcmp(b"squash\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        if squash_set != 0 {
                            fprintf(
                                stderr,
                                b"argument --squash / -s already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        squash_set = BROTLI_TRUE;
                        (*params).reject_uncompressible = BROTLI_TRUE;
                    } else if strcmp(b"stdout\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        if output_set != 0 {
                            fprintf(
                                stderr,
                                b"write to standard output already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        output_set = BROTLI_TRUE;
                        (*params).write_to_stdout = BROTLI_TRUE;
                    } else if strcmp(b"test\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        if command_set != 0 {
                            fprintf(
                                stderr,
                                b"command already set when parsing --test\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        command_set = BROTLI_TRUE;
                        command = COMMAND_TEST_INTEGRITY;
                    } else if strcmp(b"verbose\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        if (*params).verbosity > 0 as c_int {
                            fprintf(
                                stderr,
                                b"argument --verbose / -v already set\n\0" as *const u8
                                    as *const c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        (*params).verbosity = 1 as c_int;
                    } else if strcmp(b"version\0" as *const u8 as *const c_char, arg)
                        == 0 as c_int
                    {
                        return COMMAND_VERSION;
                    } else {
                        let mut value: *const c_char = strchr(arg, '=' as i32);
                        let mut key_len: size_t = 0;
                        if value.is_null()
                            || *value.offset(1 as c_int as isize) as c_int
                                == 0 as c_int
                        {
                            fprintf(
                                stderr,
                                b"must pass the parameter as --%s=value\n\0" as *const u8
                                    as *const c_char,
                                arg,
                            );
                            return COMMAND_INVALID;
                        }
                        key_len = value.offset_from(arg) as c_long as size_t;
                        value = value.offset(1);
                        if strncmp(
                            b"comment\0" as *const u8 as *const c_char,
                            arg,
                            key_len,
                        ) == 0 as c_int
                        {
                            if comment_set != 0 {
                                fprintf(
                                    stderr,
                                    b"comment already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params).comment_len = MAX_COMMENT_LEN as size_t;
                            if ParseBase64(
                                value,
                                &raw mut (*params).comment as *mut uint8_t,
                                &raw mut (*params).comment_len,
                            ) == 0
                            {
                                fprintf(
                                    stderr,
                                    b"invalid base64-encoded comment\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            comment_set = BROTLI_TRUE;
                        } else if strncmp(
                            b"dictionary\0" as *const u8 as *const c_char,
                            arg,
                            key_len,
                        ) == 0 as c_int
                        {
                            if !(*params).dictionary_path.is_null() {
                                fprintf(
                                    stderr,
                                    b"dictionary path already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params).dictionary_path = value;
                        } else if strncmp(
                            b"lgwin\0" as *const u8 as *const c_char,
                            arg,
                            key_len,
                        ) == 0 as c_int
                        {
                            if lgwin_set != 0 {
                                fprintf(
                                    stderr,
                                    b"lgwin parameter already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            lgwin_set = ParseInt(
                                value,
                                0 as c_int,
                                BROTLI_MAX_WINDOW_BITS,
                                &raw mut (*params).lgwin,
                            );
                            if lgwin_set == 0 {
                                fprintf(
                                    stderr,
                                    b"error parsing lgwin value [%s]\n\0" as *const u8
                                        as *const c_char,
                                    value,
                                );
                                return COMMAND_INVALID;
                            }
                            if (*params).lgwin != 0 as c_int
                                && (*params).lgwin < BROTLI_MIN_WINDOW_BITS
                            {
                                fprintf(
                                    stderr,
                                    b"lgwin parameter (%d) smaller than the minimum (%d)\n\0"
                                        as *const u8
                                        as *const c_char,
                                    (*params).lgwin,
                                    BROTLI_MIN_WINDOW_BITS,
                                );
                                return COMMAND_INVALID;
                            }
                        } else if strncmp(
                            b"large_window\0" as *const u8 as *const c_char,
                            arg,
                            key_len,
                        ) == 0 as c_int
                        {
                            if lgwin_set != 0 {
                                fprintf(
                                    stderr,
                                    b"lgwin parameter already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            lgwin_set = ParseInt(
                                value,
                                0 as c_int,
                                BROTLI_LARGE_MAX_WINDOW_BITS,
                                &raw mut (*params).lgwin,
                            );
                            if lgwin_set == 0 {
                                fprintf(
                                    stderr,
                                    b"error parsing lgwin value [%s]\n\0" as *const u8
                                        as *const c_char,
                                    value,
                                );
                                return COMMAND_INVALID;
                            }
                            if (*params).lgwin != 0 as c_int
                                && (*params).lgwin < BROTLI_MIN_WINDOW_BITS
                            {
                                fprintf(
                                    stderr,
                                    b"lgwin parameter (%d) smaller than the minimum (%d)\n\0"
                                        as *const u8
                                        as *const c_char,
                                    (*params).lgwin,
                                    BROTLI_MIN_WINDOW_BITS,
                                );
                                return COMMAND_INVALID;
                            }
                        } else if strncmp(
                            b"output\0" as *const u8 as *const c_char,
                            arg,
                            key_len,
                        ) == 0 as c_int
                        {
                            if output_set != 0 {
                                fprintf(
                                    stderr,
                                    b"write to standard output already set (--output)\n\0"
                                        as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params).output_path = value;
                        } else if strncmp(
                            b"quality\0" as *const u8 as *const c_char,
                            arg,
                            key_len,
                        ) == 0 as c_int
                        {
                            if quality_set != 0 {
                                fprintf(
                                    stderr,
                                    b"quality already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            quality_set = ParseInt(
                                value,
                                BROTLI_MIN_QUALITY,
                                BROTLI_MAX_QUALITY,
                                &raw mut (*params).quality,
                            );
                            if quality_set == 0 {
                                fprintf(
                                    stderr,
                                    b"error parsing quality value [%s]\n\0" as *const u8
                                        as *const c_char,
                                    value,
                                );
                                return COMMAND_INVALID;
                            }
                        } else if strncmp(
                            b"suffix\0" as *const u8 as *const c_char,
                            arg,
                            key_len,
                        ) == 0 as c_int
                        {
                            if suffix_set != 0 {
                                fprintf(
                                    stderr,
                                    b"suffix already set\n\0" as *const u8
                                        as *const c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            suffix_set = BROTLI_TRUE;
                            (*params).suffix = value;
                        } else {
                            fprintf(
                                stderr,
                                b"invalid parameter: [%s]\n\0" as *const u8
                                    as *const c_char,
                                arg,
                            );
                            return COMMAND_INVALID;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    (*params).input_count = input_count;
    (*params).longest_path_len = longest_path_len;
    (*params).decompress = (command as c_uint
        == COMMAND_DECOMPRESS as c_int as c_uint)
        as c_int;
    (*params).test_integrity = (command as c_uint
        == COMMAND_TEST_INTEGRITY as c_int as c_uint)
        as c_int;
    if input_count > 1 as size_t && output_set != 0 {
        return COMMAND_INVALID;
    }
    if (*params).test_integrity != 0 {
        if !(*params).output_path.is_null() {
            return COMMAND_INVALID;
        }
        if (*params).write_to_stdout != 0 {
            return COMMAND_INVALID;
        }
    }
    if (*params).reject_uncompressible != 0 && (*params).write_to_stdout != 0 {
        return COMMAND_INVALID;
    }
    if !strchr((*params).suffix, '/' as i32).is_null()
        || !strchr((*params).suffix, '\\' as i32).is_null()
    {
        return COMMAND_INVALID;
    }
    if (*params).decompress == 0 && (*params).allow_concatenated != 0 {
        return COMMAND_INVALID;
    }
    if (*params).allow_concatenated != 0 && (*params).comment_len != 0 {
        return COMMAND_INVALID;
    }
    return command;
}
unsafe extern "C" fn PrintVersion() {
    let mut major: c_int = BROTLI_VERSION_MAJOR;
    let mut minor: c_int = BROTLI_VERSION_MINOR;
    let mut patch: c_int = BROTLI_VERSION_PATCH;
    fprintf(
        stdout,
        b"brotli %d.%d.%d\n\0" as *const u8 as *const c_char,
        major,
        minor,
        patch,
    );
}
unsafe extern "C" fn PrintHelp(
    mut name: *const c_char,
    mut error: c_int,
) {
    let mut media: *mut FILE = if error != 0 { stderr } else { stdout };
    fprintf(
        media,
        b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8 as *const c_char,
        name,
    );
    fprintf(
        media,
        b"Options:\n  -#                          compression level (0-9)\n  -c, --stdout                write on standard output\n  -d, --decompress            decompress\n  -f, --force                 force output file overwrite\n  -h, --help                  display this help and exit\n\0"
            as *const u8 as *const c_char,
    );
    fprintf(
        media,
        b"  -j, --rm                    remove source file(s)\n  -s, --squash                remove destination file if larger than source\n  -k, --keep                  keep source file(s) (default)\n  -n, --no-copy-stat          do not copy source file(s) attributes\n  -o FILE, --output=FILE      output file (only if 1 input file)\n\0"
            as *const u8 as *const c_char,
    );
    fprintf(
        media,
        b"  -q NUM, --quality=NUM       compression level (%d-%d)\n\0" as *const u8
            as *const c_char,
        BROTLI_MIN_QUALITY,
        BROTLI_MAX_QUALITY,
    );
    fprintf(
        media,
        b"  -t, --test                  test compressed file integrity\n  -v, --verbose               verbose mode\n\0"
            as *const u8 as *const c_char,
    );
    fprintf(
        media,
        b"  -w NUM, --lgwin=NUM         set LZ77 window size (0, %d-%d)\n                              window size = 2**NUM - 16\n                              0 lets compressor choose the optimal value\n\0"
            as *const u8 as *const c_char,
        BROTLI_MIN_WINDOW_BITS,
        BROTLI_MAX_WINDOW_BITS,
    );
    fprintf(
        media,
        b"  --large_window=NUM          use incompatible large-window brotli\n                              bitstream with window size (0, %d-%d)\n                              WARNING: this format is not compatible\n                              with brotli RFC 7932 and may not be\n                              decodable with regular brotli decoders\n\0"
            as *const u8 as *const c_char,
        BROTLI_MIN_WINDOW_BITS,
        BROTLI_LARGE_MAX_WINDOW_BITS,
    );
    fprintf(
        media,
        b"  -C B64, --comment=B64       set comment; argument is base64-decoded first;\n                              (maximal decoded length: %d)\n                              when decoding: check stream comment;\n                              when encoding: embed comment (fingerprint)\n\0"
            as *const u8 as *const c_char,
        MAX_COMMENT_LEN,
    );
    fprintf(
        media,
        b"  -D FILE, --dictionary=FILE  use FILE as raw (LZ77) dictionary\n  -K, --concatenated          allows concatenated brotli streams as input\n\0"
            as *const u8 as *const c_char,
    );
    fprintf(
        media,
        b"  -S SUF, --suffix=SUF        output file suffix (default:'%s')\n\0" as *const u8
            as *const c_char,
        DEFAULT_SUFFIX.as_ptr(),
    );
    fprintf(
        media,
        b"  -V, --version               display version and exit\n  -Z, --best                  use best compression level (11) (default)\nSimple options could be coalesced, i.e. '-9kf' is equivalent to '-9 -k -f'.\nWith no FILE, or when FILE is -, read standard input.\nAll arguments after '--' are treated as files.\n\0"
            as *const u8 as *const c_char,
    );
}
unsafe extern "C" fn PrintablePath(
    mut path: *const c_char,
) -> *const c_char {
    return if !path.is_null() {
        path
    } else {
        b"con\0" as *const u8 as *const c_char
    };
}
unsafe extern "C" fn OpenInputFile(
    mut input_path: *const c_char,
    mut f: *mut *mut FILE,
) -> c_int {
    *f = ::core::ptr::null_mut::<FILE>();
    if input_path.is_null() {
        *f = fdopen(
            0 as c_int,
            b"rb\0" as *const u8 as *const c_char,
        );
        return BROTLI_TRUE;
    }
    *f = fopen(
        input_path,
        b"rb\0" as *const u8 as *const c_char,
    );
    if (*f).is_null() {
        fprintf(
            stderr,
            b"failed to open input file [%s]: %s\n\0" as *const u8 as *const c_char,
            PrintablePath(input_path),
            strerror(*__errno_location()),
        );
        return BROTLI_FALSE;
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn OpenOutputFile(
    mut output_path: *const c_char,
    mut f: *mut *mut FILE,
    mut force: c_int,
) -> c_int {
    let mut fd: c_int = 0;
    *f = ::core::ptr::null_mut::<FILE>();
    if output_path.is_null() {
        *f = fdopen(
            1 as c_int,
            b"wb\0" as *const u8 as *const c_char,
        );
        return BROTLI_TRUE;
    }
    fd = open(
        output_path,
        O_CREAT
            | (if force != 0 {
                0 as c_int
            } else {
                O_EXCL
            })
            | O_WRONLY
            | O_TRUNC,
        S_IRUSR | S_IWUSR,
    );
    if fd < 0 as c_int {
        fprintf(
            stderr,
            b"failed to open output file [%s]: %s\n\0" as *const u8 as *const c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
        return BROTLI_FALSE;
    }
    *f = fdopen(fd, b"wb\0" as *const u8 as *const c_char);
    if (*f).is_null() {
        fprintf(
            stderr,
            b"failed to open output file [%s]: %s\n\0" as *const u8 as *const c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
        return BROTLI_FALSE;
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn FileSize(mut path: *const c_char) -> int64_t {
    let mut f: *mut FILE = fopen(path, b"rb\0" as *const u8 as *const c_char);
    let mut retval: int64_t = 0;
    if f.is_null() {
        return -(1 as c_int) as int64_t;
    }
    if fseek(f, 0 as c_long, SEEK_END) != 0 as c_int {
        fclose(f);
        return -(1 as c_int) as int64_t;
    }
    retval = ftell(f) as int64_t;
    if fclose(f) != 0 as c_int {
        return -(1 as c_int) as int64_t;
    }
    return retval;
}
unsafe extern "C" fn CopyTimeStat(
    mut statbuf: *const stat,
    mut output_path: *const c_char,
    mut fout: *mut FILE,
) -> c_int {
    let mut times: [timespec; 2] = [timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 2];
    let mut fd: c_int = fileno(fout);
    if fd < 0 as c_int {
        return -(1 as c_int);
    }
    times[0 as c_int as usize].tv_sec = (*statbuf).st_atim.tv_sec;
    times[0 as c_int as usize].tv_nsec = (*statbuf).st_atim.tv_nsec;
    times[1 as c_int as usize].tv_sec = (*statbuf).st_mtim.tv_sec;
    times[1 as c_int as usize].tv_nsec = (*statbuf).st_mtim.tv_nsec;
    return futimens(fd, &raw mut times as *mut timespec as *const timespec);
}
unsafe extern "C" fn CopyStat(
    mut input_path: *const c_char,
    mut output_path: *const c_char,
    mut fout: *mut FILE,
) {
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut fd: c_int = 0;
    let mut res: c_int = 0;
    if input_path.is_null() || output_path.is_null() || fout.is_null() {
        return;
    }
    fd = fileno(fout);
    if fd < 0 as c_int {
        return;
    }
    if stat(input_path, &raw mut statbuf) != 0 as c_int {
        return;
    }
    if fflush(fout) != 0 as c_int {
        return;
    }
    res = CopyTimeStat(&raw mut statbuf, output_path, fout);
    res = fchmod(
        fd,
        statbuf.st_mode & (S_IRWXU | S_IRWXG | S_IRWXO) as __mode_t,
    );
    if res != 0 as c_int {
        fprintf(
            stderr,
            b"setting access bits failed for [%s]: %s\n\0" as *const u8
                as *const c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
    }
    res = fchown(fd, -(1 as c_int) as __uid_t, statbuf.st_gid);
    if res != 0 as c_int {
        fprintf(
            stderr,
            b"setting group failed for [%s]: %s\n\0" as *const u8 as *const c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
    }
    res = fchown(fd, statbuf.st_uid, -(1 as c_int) as __gid_t);
    if res != 0 as c_int {
        fprintf(
            stderr,
            b"setting user failed for [%s]: %s\n\0" as *const u8 as *const c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn ReadDictionary(
    mut context: *mut Context,
    mut command: Command,
) -> c_int {
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut file_size_64: int64_t = 0;
    let mut buffer: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut bytes_read: size_t = 0;
    if (*context).dictionary_path.is_null() {
        return BROTLI_TRUE;
    }
    f = fopen(
        (*context).dictionary_path,
        b"rb\0" as *const u8 as *const c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"failed to open dictionary file [%s]: %s\n\0" as *const u8
                as *const c_char,
            PrintablePath((*context).dictionary_path),
            strerror(*__errno_location()),
        );
        return BROTLI_FALSE;
    }
    file_size_64 = FileSize((*context).dictionary_path);
    if file_size_64 == -(1 as c_int) as int64_t {
        fprintf(
            stderr,
            b"could not get size of dictionary file [%s]\0" as *const u8
                as *const c_char,
            PrintablePath((*context).dictionary_path),
        );
        fclose(f);
        return BROTLI_FALSE;
    }
    if file_size_64 > kMaxDictionarySize as int64_t {
        fprintf(
            stderr,
            b"dictionary [%s] is larger than maximum allowed: %d\n\0" as *const u8
                as *const c_char,
            PrintablePath((*context).dictionary_path),
            kMaxDictionarySize,
        );
        fclose(f);
        return BROTLI_FALSE;
    }
    (*context).dictionary_size = file_size_64 as size_t;
    buffer = malloc((*context).dictionary_size) as *mut uint8_t;
    if buffer.is_null() {
        fprintf(
            stderr,
            b"could not read dictionary: out of memory\n\0" as *const u8
                as *const c_char,
        );
        fclose(f);
        return BROTLI_FALSE;
    }
    bytes_read = fread(
        buffer as *mut c_void,
        ::core::mem::size_of::<uint8_t>() as size_t,
        (*context).dictionary_size,
        f,
    ) as size_t;
    if bytes_read != (*context).dictionary_size {
        free(buffer as *mut c_void);
        fprintf(
            stderr,
            b"failed to read dictionary [%s]: %s\n\0" as *const u8 as *const c_char,
            PrintablePath((*context).dictionary_path),
            strerror(*__errno_location()),
        );
        fclose(f);
        return BROTLI_FALSE;
    }
    fclose(f);
    (*context).dictionary = buffer;
    if command as c_uint
        == COMMAND_COMPRESS as c_int as c_uint
    {
        (*context).prepared_dictionary = BrotliEncoderPrepareDictionary(
            BROTLI_SHARED_DICTIONARY_RAW,
            (*context).dictionary_size,
            (*context).dictionary as *const uint8_t,
            BROTLI_MAX_QUALITY,
            None,
            None,
            NULL,
        );
        if (*context).prepared_dictionary.is_null() {
            fprintf(
                stderr,
                b"failed to prepare dictionary [%s]\n\0" as *const u8 as *const c_char,
                PrintablePath((*context).dictionary_path),
            );
            return BROTLI_FALSE;
        }
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn NextFile(mut context: *mut Context) -> c_int {
    let mut arg: *const c_char = ::core::ptr::null::<c_char>();
    let mut arg_len: size_t = 0;
    (*context).iterator += 1;
    (*context).input_file_length = -(1 as c_int) as int64_t;
    if (*context).input_count == 0 as size_t {
        if (*context).iterator > 1 as c_int {
            return BROTLI_FALSE;
        }
        (*context).current_input_path = ::core::ptr::null::<c_char>();
        (*context).current_output_path = (*context).output_path;
        return BROTLI_TRUE;
    }
    while (*context).iterator == (*context).not_input_indices[(*context).ignore as usize] {
        (*context).iterator += 1;
        (*context).ignore += 1;
    }
    if (*context).iterator >= (*context).argc {
        return BROTLI_FALSE;
    }
    arg = *(*context).argv.offset((*context).iterator as isize);
    arg_len = strlen(arg);
    if arg_len == 1 as size_t
        && *arg.offset(0 as c_int as isize) as c_int == '-' as i32
    {
        (*context).current_input_path = ::core::ptr::null::<c_char>();
        (*context).current_output_path = (*context).output_path;
        return BROTLI_TRUE;
    }
    (*context).current_input_path = arg;
    (*context).input_file_length = FileSize(arg);
    (*context).current_output_path = (*context).output_path;
    if !(*context).output_path.is_null() {
        return BROTLI_TRUE;
    }
    if (*context).write_to_stdout != 0 {
        return BROTLI_TRUE;
    }
    strcpy((*context).modified_path, arg);
    (*context).current_output_path = (*context).modified_path;
    if (*context).decompress != 0 {
        let mut suffix_len: size_t = strlen((*context).suffix);
        let mut name: *mut c_char =
            FileName((*context).modified_path) as *mut c_char;
        let mut name_suffix: *mut c_char =
            ::core::ptr::null_mut::<c_char>();
        let mut name_len: size_t = strlen(name);
        if name_len < suffix_len.wrapping_add(1 as size_t) {
            fprintf(
                stderr,
                b"empty output file name for [%s] input file\n\0" as *const u8
                    as *const c_char,
                PrintablePath(arg),
            );
            (*context).iterator_error = BROTLI_TRUE;
            return BROTLI_FALSE;
        }
        name_suffix = name
            .offset(name_len as isize)
            .offset(-(suffix_len as isize));
        if strcmp((*context).suffix, name_suffix) != 0 as c_int {
            fprintf(
                stderr,
                b"input file [%s] suffix mismatch\n\0" as *const u8 as *const c_char,
                PrintablePath(arg),
            );
            (*context).iterator_error = BROTLI_TRUE;
            return BROTLI_FALSE;
        }
        *name_suffix.offset(0 as c_int as isize) = 0 as c_char;
        return BROTLI_TRUE;
    } else {
        strcpy(
            (*context).modified_path.offset(arg_len as isize),
            (*context).suffix,
        );
        return BROTLI_TRUE;
    };
}
unsafe extern "C" fn OpenFiles(mut context: *mut Context) -> c_int {
    let mut is_ok: c_int =
        OpenInputFile((*context).current_input_path, &raw mut (*context).fin);
    if (*context).test_integrity == 0 && is_ok != 0 {
        is_ok = OpenOutputFile(
            (*context).current_output_path,
            &raw mut (*context).fout,
            (*context).force_overwrite,
        );
    }
    return is_ok;
}
unsafe extern "C" fn CloseFiles(
    mut context: *mut Context,
    mut rm_input: c_int,
    mut rm_output: c_int,
) -> c_int {
    let mut is_ok: c_int = BROTLI_TRUE;
    if (*context).test_integrity == 0 && !(*context).fout.is_null() {
        if rm_output == 0 && (*context).copy_stat != 0 && !(*context).current_output_path.is_null()
        {
            CopyStat(
                (*context).current_input_path,
                (*context).current_output_path,
                (*context).fout,
            );
        }
        if fclose((*context).fout) != 0 as c_int {
            if is_ok != 0 {
                fprintf(
                    stderr,
                    b"fclose failed [%s]: %s\n\0" as *const u8 as *const c_char,
                    PrintablePath((*context).current_output_path),
                    strerror(*__errno_location()),
                );
            }
            is_ok = BROTLI_FALSE;
        }
        if rm_output != 0 && !(*context).current_output_path.is_null() {
            unlink((*context).current_output_path);
        }
    }
    if !(*context).fin.is_null() {
        if fclose((*context).fin) != 0 as c_int {
            if is_ok != 0 {
                fprintf(
                    stderr,
                    b"fclose failed [%s]: %s\n\0" as *const u8 as *const c_char,
                    PrintablePath((*context).current_input_path),
                    strerror(*__errno_location()),
                );
            }
            is_ok = BROTLI_FALSE;
        }
    }
    if rm_input != 0 && !(*context).current_input_path.is_null() {
        unlink((*context).current_input_path);
    }
    (*context).fin = ::core::ptr::null_mut::<FILE>();
    (*context).fout = ::core::ptr::null_mut::<FILE>();
    return is_ok;
}
static mut kFileBufferSize: size_t =
    ((1 as c_int) << 19 as c_int) as size_t;
unsafe extern "C" fn InitializeBuffers(mut context: *mut Context) {
    (*context).available_in = 0 as size_t;
    (*context).next_in = ::core::ptr::null::<uint8_t>();
    (*context).available_out = kFileBufferSize;
    (*context).next_out = (*context).output;
    (*context).total_in = 0 as size_t;
    (*context).total_out = 0 as size_t;
    if (*context).verbosity > 0 as c_int {
        (*context).start_time = clock();
    }
}
unsafe extern "C" fn HasMoreInput(mut context: *mut Context) -> c_int {
    return if feof((*context).fin) != 0 {
        BROTLI_FALSE
    } else {
        BROTLI_TRUE
    };
}
unsafe extern "C" fn ProvideInput(mut context: *mut Context) -> c_int {
    (*context).available_in = fread(
        (*context).input as *mut c_void,
        1 as size_t,
        kFileBufferSize,
        (*context).fin,
    ) as size_t;
    (*context).total_in = ((*context).total_in as c_ulong)
        .wrapping_add((*context).available_in as c_ulong)
        as size_t as size_t;
    (*context).next_in = (*context).input;
    if ferror((*context).fin) != 0 {
        fprintf(
            stderr,
            b"failed to read input [%s]: %s\n\0" as *const u8 as *const c_char,
            PrintablePath((*context).current_input_path),
            strerror(*__errno_location()),
        );
        return BROTLI_FALSE;
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn WriteOutput(mut context: *mut Context) -> c_int {
    let mut out_size: size_t =
        (*context).next_out.offset_from((*context).output) as c_long as size_t;
    (*context).total_out = ((*context).total_out as c_ulong)
        .wrapping_add(out_size as c_ulong) as size_t
        as size_t;
    if out_size == 0 as size_t {
        return BROTLI_TRUE;
    }
    if (*context).test_integrity != 0 {
        return BROTLI_TRUE;
    }
    fwrite(
        (*context).output as *const c_void,
        1 as size_t,
        out_size,
        (*context).fout,
    );
    if ferror((*context).fout) != 0 {
        fprintf(
            stderr,
            b"failed to write output [%s]: %s\n\0" as *const u8 as *const c_char,
            PrintablePath((*context).current_output_path),
            strerror(*__errno_location()),
        );
        return BROTLI_FALSE;
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn ProvideOutput(mut context: *mut Context) -> c_int {
    if WriteOutput(context) == 0 {
        return BROTLI_FALSE;
    }
    (*context).available_out = kFileBufferSize;
    (*context).next_out = (*context).output;
    return BROTLI_TRUE;
}
unsafe extern "C" fn FlushOutput(mut context: *mut Context) -> c_int {
    if WriteOutput(context) == 0 {
        return BROTLI_FALSE;
    }
    (*context).available_out = 0 as size_t;
    (*context).next_out = (*context).output;
    return BROTLI_TRUE;
}
unsafe extern "C" fn PrintBytes(mut value: size_t) {
    if value < 1024 as size_t {
        fprintf(
            stderr,
            b"%d B\0" as *const u8 as *const c_char,
            value as c_int,
        );
    } else if value < 1048576 as c_int as size_t {
        fprintf(
            stderr,
            b"%0.3f KiB\0" as *const u8 as *const c_char,
            value as c_double / 1024.0f64,
        );
    } else if value < 1073741824 as c_int as size_t {
        fprintf(
            stderr,
            b"%0.3f MiB\0" as *const u8 as *const c_char,
            value as c_double / 1048576.0f64,
        );
    } else {
        fprintf(
            stderr,
            b"%0.3f GiB\0" as *const u8 as *const c_char,
            value as c_double / 1073741824.0f64,
        );
    };
}
unsafe extern "C" fn PrintFileProcessingProgress(mut context: *mut Context) {
    fprintf(
        stderr,
        b"[%s]: \0" as *const u8 as *const c_char,
        PrintablePath((*context).current_input_path),
    );
    PrintBytes((*context).total_in);
    fprintf(stderr, b" -> \0" as *const u8 as *const c_char);
    PrintBytes((*context).total_out);
    fprintf(
        stderr,
        b" in %1.2f sec\0" as *const u8 as *const c_char,
        ((*context).end_time - (*context).start_time) as c_double
            / CLOCKS_PER_SEC as c_double,
    );
}
unsafe extern "C" fn PrettyDecoderErrorString(
    mut code: BrotliDecoderErrorCode,
) -> *const c_char {
    let mut prefix: *const c_char =
        b"_ERROR_\0" as *const u8 as *const c_char;
    let mut prefix_len: size_t = strlen(prefix);
    let mut error_str: *const c_char = BrotliDecoderErrorString(code);
    let mut error_len: size_t = strlen(error_str);
    if error_len > prefix_len {
        if strncmp(error_str, prefix, prefix_len) == 0 as c_int {
            error_str = error_str.offset(prefix_len as isize);
        }
    }
    return error_str;
}
unsafe extern "C" fn OnMetadataStart(mut opaque: *mut c_void, mut size: size_t) {
    let mut context: *mut Context = opaque as *mut Context;
    if (*context).comment_state as c_uint
        == COMMENT_INIT as c_int as c_uint
    {
        if (*context).comment_len != size {
            (*context).comment_state = COMMENT_BAD;
            return;
        }
        (*context).comment_pos = 0 as size_t;
        (*context).comment_state = COMMENT_READ;
    }
}
unsafe extern "C" fn OnMetadataChunk(
    mut opaque: *mut c_void,
    mut data: *const uint8_t,
    mut size: size_t,
) {
    let mut context: *mut Context = opaque as *mut Context;
    if (*context).comment_state as c_uint
        == COMMENT_READ as c_int as c_uint
    {
        let mut i: size_t = 0;
        i = 0 as size_t;
        while i < size {
            if (*context).comment_pos >= (*context).comment_len {
                (*context).comment_state = COMMENT_BAD;
                return;
            }
            let fresh4 = (*context).comment_pos;
            (*context).comment_pos = (*context).comment_pos.wrapping_add(1);
            if (*context).comment[fresh4 as usize] as c_int
                != *data.offset(i as isize) as c_int
            {
                (*context).comment_state = COMMENT_BAD;
                return;
            }
            i = i.wrapping_add(1);
        }
        if (*context).comment_pos == (*context).comment_len {
            (*context).comment_state = COMMENT_OK;
        }
    }
}
unsafe extern "C" fn InitDecoder(mut context: *mut Context) -> c_int {
    (*context).decoder = BrotliDecoderCreateInstance(None, None, NULL);
    if (*context).decoder.is_null() {
        fprintf(
            stderr,
            b"out of memory\n\0" as *const u8 as *const c_char,
        );
        return BROTLI_FALSE;
    }
    BrotliDecoderSetParameter(
        (*context).decoder,
        BROTLI_DECODER_PARAM_LARGE_WINDOW,
        1 as uint32_t,
    );
    if !(*context).dictionary.is_null() {
        BrotliDecoderAttachDictionary(
            (*context).decoder,
            BROTLI_SHARED_DICTIONARY_RAW,
            (*context).dictionary_size,
            (*context).dictionary as *const uint8_t,
        );
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn DecompressFile(mut context: *mut Context) -> c_int {
    let mut s: *mut BrotliDecoderState = (*context).decoder;
    let mut result: BrotliDecoderResult = BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT;
    if (*context).comment_len != 0 {
        (*context).comment_state = COMMENT_INIT;
        BrotliDecoderSetMetadataCallbacks(
            s,
            Some(OnMetadataStart as unsafe extern "C" fn(*mut c_void, size_t) -> ()),
            Some(
                OnMetadataChunk
                    as unsafe extern "C" fn(*mut c_void, *const uint8_t, size_t) -> (),
            ),
            context as *mut c_void,
        );
    } else {
        (*context).comment_state = COMMENT_OK;
    }
    InitializeBuffers(context);
    loop {
        if (*context).comment_state as c_uint
            == COMMENT_BAD as c_int as c_uint
        {
            fprintf(
                stderr,
                b"corrupt input [%s]\n\0" as *const u8 as *const c_char,
                PrintablePath((*context).current_input_path),
            );
            if (*context).verbosity > 0 as c_int {
                fprintf(
                    stderr,
                    b"reason: comment mismatch\n\0" as *const u8 as *const c_char,
                );
            }
            return BROTLI_FALSE;
        }
        if result as c_uint
            == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT as c_int as c_uint
        {
            if HasMoreInput(context) == 0 {
                fprintf(
                    stderr,
                    b"corrupt input [%s]\n\0" as *const u8 as *const c_char,
                    PrintablePath((*context).current_input_path),
                );
                if (*context).verbosity > 0 as c_int {
                    fprintf(
                        stderr,
                        b"reason: truncated input\n\0" as *const u8 as *const c_char,
                    );
                }
                return BROTLI_FALSE;
            }
            if ProvideInput(context) == 0 {
                return BROTLI_FALSE;
            }
        } else if result as c_uint
            == BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT as c_int as c_uint
        {
            if ProvideOutput(context) == 0 {
                return BROTLI_FALSE;
            }
        } else if result as c_uint
            == BROTLI_DECODER_RESULT_SUCCESS as c_int as c_uint
        {
            if FlushOutput(context) == 0 {
                return BROTLI_FALSE;
            }
            let mut has_more_input: c_int =
                ((*context).available_in != 0 as size_t) as c_int;
            let mut extra_char: c_int = EOF;
            if has_more_input == 0 {
                extra_char = fgetc((*context).fin);
                if extra_char != EOF {
                    has_more_input = BROTLI_TRUE;
                    *(*context).input.offset(0 as c_int as isize) =
                        extra_char as uint8_t;
                    (*context).next_in = (*context).input;
                    (*context).available_in = 1 as size_t;
                }
            }
            if has_more_input != 0 {
                if (*context).allow_concatenated != 0 {
                    if (*context).verbosity > 0 as c_int {
                        fprintf(
                            stderr,
                            b"extra input\n\0" as *const u8 as *const c_char,
                        );
                    }
                    if ProvideOutput(context) == 0 {
                        return BROTLI_FALSE;
                    }
                    BrotliDecoderDestroyInstance((*context).decoder);
                    (*context).decoder = ::core::ptr::null_mut::<BrotliDecoderState>();
                    if InitDecoder(context) == 0 {
                        return BROTLI_FALSE;
                    }
                    s = (*context).decoder;
                } else {
                    fprintf(
                        stderr,
                        b"corrupt input [%s]\n\0" as *const u8 as *const c_char,
                        PrintablePath((*context).current_input_path),
                    );
                    if (*context).verbosity > 0 as c_int {
                        fprintf(
                            stderr,
                            b"reason: extra input\n\0" as *const u8 as *const c_char,
                        );
                    }
                    return BROTLI_FALSE;
                }
            } else {
                if (*context).verbosity > 0 as c_int {
                    (*context).end_time = clock();
                    fprintf(
                        stderr,
                        b"Decompressed \0" as *const u8 as *const c_char,
                    );
                    PrintFileProcessingProgress(context);
                    fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
                }
                if (*context).comment_state as c_uint
                    != COMMENT_OK as c_int as c_uint
                {
                    fprintf(
                        stderr,
                        b"corrupt input [%s]\n\0" as *const u8 as *const c_char,
                        PrintablePath((*context).current_input_path),
                    );
                    if (*context).verbosity > 0 as c_int {
                        fprintf(
                            stderr,
                            b"reason: comment mismatch\n\0" as *const u8
                                as *const c_char,
                        );
                    }
                    return BROTLI_FALSE;
                }
                return BROTLI_TRUE;
            }
        } else {
            fprintf(
                stderr,
                b"corrupt input [%s]\n\0" as *const u8 as *const c_char,
                PrintablePath((*context).current_input_path),
            );
            if (*context).verbosity > 0 as c_int {
                let mut error: BrotliDecoderErrorCode = BrotliDecoderGetErrorCode(s);
                let mut error_str: *const c_char = PrettyDecoderErrorString(error);
                fprintf(
                    stderr,
                    b"reason: %s (%d)\n\0" as *const u8 as *const c_char,
                    error_str,
                    error as c_int,
                );
            }
            return BROTLI_FALSE;
        }
        result = BrotliDecoderDecompressStream(
            s,
            &raw mut (*context).available_in,
            &raw mut (*context).next_in,
            &raw mut (*context).available_out,
            &raw mut (*context).next_out,
            ::core::ptr::null_mut::<size_t>(),
        );
    }
}
unsafe extern "C" fn DecompressFiles(mut context: *mut Context) -> c_int {
    while NextFile(context) != 0 {
        let mut is_ok: c_int = BROTLI_TRUE;
        let mut rm_input: c_int = BROTLI_FALSE;
        let mut rm_output: c_int = BROTLI_TRUE;
        if InitDecoder(context) == 0 {
            return BROTLI_FALSE;
        }
        is_ok = OpenFiles(context);
        if is_ok != 0
            && (*context).current_input_path.is_null()
            && (*context).force_overwrite == 0
            && isatty(STDIN_FILENO) != 0
        {
            fprintf(
                stderr,
                b"Use -h help. Use -f to force input from a terminal.\n\0" as *const u8
                    as *const c_char,
            );
            is_ok = BROTLI_FALSE;
        }
        if is_ok != 0 {
            is_ok = DecompressFile(context);
        }
        if !(*context).decoder.is_null() {
            BrotliDecoderDestroyInstance((*context).decoder);
        }
        (*context).decoder = ::core::ptr::null_mut::<BrotliDecoderState>();
        rm_output = (is_ok == 0) as c_int;
        rm_input = (rm_output == 0 && (*context).junk_source != 0) as c_int;
        if CloseFiles(context, rm_input, rm_output) == 0 {
            is_ok = BROTLI_FALSE;
        }
        if is_ok == 0 {
            return BROTLI_FALSE;
        }
    }
    return BROTLI_TRUE;
}
unsafe extern "C" fn CompressFile(
    mut context: *mut Context,
    mut s: *mut BrotliEncoderState,
) -> c_int {
    let mut is_eof: c_int = BROTLI_FALSE;
    let mut prologue: c_int = ((*context).comment_len != 0) as c_int;
    InitializeBuffers(context);
    loop {
        if (*context).available_in == 0 as size_t && is_eof == 0 {
            if ProvideInput(context) == 0 {
                return BROTLI_FALSE;
            }
            is_eof = (HasMoreInput(context) == 0) as c_int;
        }
        if prologue != 0 {
            prologue = BROTLI_FALSE;
            let mut next_meta: *const uint8_t = &raw mut (*context).comment as *mut uint8_t;
            let mut available_meta: size_t = (*context).comment_len;
            if BrotliEncoderCompressStream(
                s,
                BROTLI_OPERATION_EMIT_METADATA,
                &raw mut available_meta,
                &raw mut next_meta,
                &raw mut (*context).available_out,
                &raw mut (*context).next_out,
                ::core::ptr::null_mut::<size_t>(),
            ) == 0
            {
                fprintf(
                    stderr,
                    b"failed to emit metadata [%s]\n\0" as *const u8 as *const c_char,
                    PrintablePath((*context).current_input_path),
                );
                return BROTLI_FALSE;
            }
            if available_meta != 0 as size_t {
                fprintf(
                    stderr,
                    b"failed to emit metadata [%s]\n\0" as *const u8 as *const c_char,
                    PrintablePath((*context).current_input_path),
                );
                return BROTLI_FALSE;
            }
        } else if BrotliEncoderCompressStream(
            s,
            (if is_eof != 0 {
                BROTLI_OPERATION_FINISH as c_int
            } else {
                BROTLI_OPERATION_PROCESS as c_int
            }) as BrotliEncoderOperation,
            &raw mut (*context).available_in,
            &raw mut (*context).next_in,
            &raw mut (*context).available_out,
            &raw mut (*context).next_out,
            ::core::ptr::null_mut::<size_t>(),
        ) == 0
        {
            fprintf(
                stderr,
                b"failed to compress data [%s]\n\0" as *const u8 as *const c_char,
                PrintablePath((*context).current_input_path),
            );
            return BROTLI_FALSE;
        }
        if (*context).available_out == 0 as size_t {
            if ProvideOutput(context) == 0 {
                return BROTLI_FALSE;
            }
        }
        if BrotliEncoderIsFinished(s) != 0 {
            if FlushOutput(context) == 0 {
                return BROTLI_FALSE;
            }
            if (*context).verbosity > 0 as c_int {
                (*context).end_time = clock();
                fprintf(
                    stderr,
                    b"Compressed \0" as *const u8 as *const c_char,
                );
                PrintFileProcessingProgress(context);
                fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
            }
            return BROTLI_TRUE;
        }
    }
}
unsafe extern "C" fn CompressFiles(mut context: *mut Context) -> c_int {
    while NextFile(context) != 0 {
        let mut is_ok: c_int = BROTLI_TRUE;
        let mut rm_input: c_int = BROTLI_FALSE;
        let mut rm_output: c_int = BROTLI_TRUE;
        let mut s: *mut BrotliEncoderState = BrotliEncoderCreateInstance(None, None, NULL);
        if s.is_null() {
            fprintf(
                stderr,
                b"out of memory\n\0" as *const u8 as *const c_char,
            );
            return BROTLI_FALSE;
        }
        BrotliEncoderSetParameter(s, BROTLI_PARAM_QUALITY, (*context).quality as uint32_t);
        if (*context).lgwin > 0 as c_int {
            if (*context).lgwin > BROTLI_MAX_WINDOW_BITS {
                BrotliEncoderSetParameter(s, BROTLI_PARAM_LARGE_WINDOW, 1 as uint32_t);
            }
            BrotliEncoderSetParameter(s, BROTLI_PARAM_LGWIN, (*context).lgwin as uint32_t);
        } else {
            let mut lgwin: uint32_t = DEFAULT_LGWIN as uint32_t;
            if (*context).input_file_length >= 0 as int64_t {
                lgwin = BROTLI_MIN_WINDOW_BITS as uint32_t;
                while ((1 as c_int as size_t) << lgwin)
                    .wrapping_sub(BROTLI_WINDOW_GAP as size_t)
                    < (*context).input_file_length as size_t
                {
                    lgwin = lgwin.wrapping_add(1);
                    if lgwin == BROTLI_MAX_WINDOW_BITS as uint32_t {
                        break;
                    }
                }
            }
            BrotliEncoderSetParameter(s, BROTLI_PARAM_LGWIN, lgwin);
        }
        if (*context).input_file_length > 0 as int64_t {
            let mut size_hint: uint32_t = if (*context).input_file_length
                < ((1 as c_int) << 30 as c_int) as int64_t
            {
                (*context).input_file_length as uint32_t
            } else {
                (1 as uint32_t) << 30 as c_int
            };
            BrotliEncoderSetParameter(s, BROTLI_PARAM_SIZE_HINT, size_hint);
        }
        if !(*context).dictionary.is_null() {
            BrotliEncoderAttachPreparedDictionary(s, (*context).prepared_dictionary);
        }
        is_ok = OpenFiles(context);
        if is_ok != 0
            && (*context).current_output_path.is_null()
            && (*context).force_overwrite == 0
            && isatty(STDOUT_FILENO) != 0
        {
            fprintf(
                stderr,
                b"Use -h help. Use -f to force output to a terminal.\n\0" as *const u8
                    as *const c_char,
            );
            is_ok = BROTLI_FALSE;
        }
        if is_ok != 0 {
            is_ok = CompressFile(context, s);
        }
        BrotliEncoderDestroyInstance(s);
        rm_output = (is_ok == 0) as c_int;
        if is_ok != 0 && (*context).reject_uncompressible != 0 {
            if (*context).total_out >= (*context).total_in {
                rm_output = BROTLI_TRUE;
                if (*context).verbosity > 0 as c_int {
                    fprintf(
                        stderr,
                        b"Output is larger than input\n\0" as *const u8
                            as *const c_char,
                    );
                }
            }
        }
        rm_input = (rm_output == 0 && (*context).junk_source != 0) as c_int;
        if CloseFiles(context, rm_input, rm_output) == 0 {
            is_ok = BROTLI_FALSE;
        }
        if is_ok == 0 {
            return BROTLI_FALSE;
        }
    }
    return BROTLI_TRUE;
}
unsafe fn main_0(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) -> c_int {
    let mut command: Command = COMMAND_COMPRESS;
    let mut context: Context = Context {
        quality: 0,
        lgwin: 0,
        verbosity: 0,
        force_overwrite: 0,
        junk_source: 0,
        reject_uncompressible: 0,
        copy_stat: 0,
        write_to_stdout: 0,
        test_integrity: 0,
        decompress: 0,
        large_window: 0,
        allow_concatenated: 0,
        output_path: ::core::ptr::null::<c_char>(),
        dictionary_path: ::core::ptr::null::<c_char>(),
        suffix: ::core::ptr::null::<c_char>(),
        comment: [0; 80],
        comment_len: 0,
        comment_pos: 0,
        comment_state: COMMENT_INIT,
        not_input_indices: [0; 24],
        longest_path_len: 0,
        input_count: 0,
        argc: 0,
        argv: ::core::ptr::null_mut::<*mut c_char>(),
        dictionary: ::core::ptr::null_mut::<uint8_t>(),
        dictionary_size: 0,
        prepared_dictionary: ::core::ptr::null_mut::<BrotliEncoderPreparedDictionary>(),
        decoder: ::core::ptr::null_mut::<BrotliDecoderState>(),
        modified_path: ::core::ptr::null_mut::<c_char>(),
        iterator: 0,
        ignore: 0,
        iterator_error: 0,
        buffer: ::core::ptr::null_mut::<uint8_t>(),
        input: ::core::ptr::null_mut::<uint8_t>(),
        output: ::core::ptr::null_mut::<uint8_t>(),
        current_input_path: ::core::ptr::null::<c_char>(),
        current_output_path: ::core::ptr::null::<c_char>(),
        input_file_length: 0,
        fin: ::core::ptr::null_mut::<FILE>(),
        fout: ::core::ptr::null_mut::<FILE>(),
        available_in: 0,
        next_in: ::core::ptr::null::<uint8_t>(),
        available_out: 0,
        next_out: ::core::ptr::null_mut::<uint8_t>(),
        total_in: 0,
        total_out: 0,
        start_time: 0,
        end_time: 0,
    };
    let mut is_ok: c_int = BROTLI_TRUE;
    let mut i: c_int = 0;
    context.quality = 11 as c_int;
    context.lgwin = -(1 as c_int);
    context.verbosity = 0 as c_int;
    context.comment_len = 0 as size_t;
    context.force_overwrite = BROTLI_FALSE;
    context.junk_source = BROTLI_FALSE;
    context.reject_uncompressible = BROTLI_FALSE;
    context.copy_stat = BROTLI_TRUE;
    context.test_integrity = BROTLI_FALSE;
    context.write_to_stdout = BROTLI_FALSE;
    context.decompress = BROTLI_FALSE;
    context.large_window = BROTLI_FALSE;
    context.allow_concatenated = BROTLI_FALSE;
    context.output_path = ::core::ptr::null::<c_char>();
    context.dictionary_path = ::core::ptr::null::<c_char>();
    context.suffix = DEFAULT_SUFFIX.as_ptr();
    i = 0 as c_int;
    while i < MAX_OPTIONS {
        context.not_input_indices[i as usize] = 0 as c_int;
        i += 1;
    }
    context.longest_path_len = 1 as size_t;
    context.input_count = 0 as size_t;
    context.argc = argc;
    context.argv = argv;
    context.dictionary = ::core::ptr::null_mut::<uint8_t>();
    context.dictionary_size = 0 as size_t;
    context.decoder = ::core::ptr::null_mut::<BrotliDecoderState>();
    context.prepared_dictionary = ::core::ptr::null_mut::<BrotliEncoderPreparedDictionary>();
    context.modified_path = ::core::ptr::null_mut::<c_char>();
    context.iterator = 0 as c_int;
    context.ignore = 0 as c_int;
    context.iterator_error = BROTLI_FALSE;
    context.buffer = ::core::ptr::null_mut::<uint8_t>();
    context.current_input_path = ::core::ptr::null::<c_char>();
    context.current_output_path = ::core::ptr::null::<c_char>();
    context.fin = ::core::ptr::null_mut::<FILE>();
    context.fout = ::core::ptr::null_mut::<FILE>();
    command = ParseParams(&raw mut context);
    if command as c_uint
        == COMMAND_COMPRESS as c_int as c_uint
        || command as c_uint
            == COMMAND_DECOMPRESS as c_int as c_uint
        || command as c_uint
            == COMMAND_TEST_INTEGRITY as c_int as c_uint
    {
        if ReadDictionary(&raw mut context, command) == 0 {
            is_ok = BROTLI_FALSE;
        }
        if is_ok != 0 {
            let mut modified_path_len: size_t = context
                .longest_path_len
                .wrapping_add(strlen(context.suffix))
                .wrapping_add(1 as size_t);
            context.modified_path = malloc(modified_path_len) as *mut c_char;
            context.buffer = malloc(kFileBufferSize.wrapping_mul(2 as size_t)) as *mut uint8_t;
            if context.modified_path.is_null() || context.buffer.is_null() {
                fprintf(
                    stderr,
                    b"out of memory\n\0" as *const u8 as *const c_char,
                );
                is_ok = BROTLI_FALSE;
            } else {
                context.input = context.buffer;
                context.output = context.buffer.offset(kFileBufferSize as isize);
            }
        }
    }
    if is_ok == 0 {
        command = COMMAND_NOOP;
    }
    match command as c_uint {
        5 => {}
        6 => {
            PrintVersion();
        }
        0 => {
            is_ok = CompressFiles(&raw mut context);
        }
        1 | 4 => {
            is_ok = DecompressFiles(&raw mut context);
        }
        2 | 3 | _ => {
            is_ok = (command as c_uint
                == COMMAND_HELP as c_int as c_uint)
                as c_int;
            PrintHelp(
                FileName(*argv.offset(0 as c_int as isize)),
                is_ok,
            );
        }
    }
    if context.iterator_error != 0 {
        is_ok = BROTLI_FALSE;
    }
    BrotliEncoderDestroyPreparedDictionary(context.prepared_dictionary);
    free(context.dictionary as *mut c_void);
    free(context.modified_path as *mut c_void);
    free(context.buffer as *mut c_void);
    if is_ok == 0 {
        exit(1 as c_int);
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
unsafe extern "C" fn run_static_initializers() {
    kMaxDictionarySize = (BROTLI_MAX_DISTANCE as size_t).wrapping_sub(
        ((1 as c_int as size_t) << 24 as c_int)
            .wrapping_sub(BROTLI_WINDOW_GAP as size_t),
    ) as c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
