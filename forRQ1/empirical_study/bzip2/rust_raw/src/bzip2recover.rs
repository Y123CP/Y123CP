#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(linkage)]
#![feature(raw_ref_op)]
#[allow(unused_imports)]
use ::bzip2_1_0_8_raw;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub type MaybeUInt64 = ::core::ffi::c_ulonglong;
pub type UInt32 = ::core::ffi::c_uint;
pub type Int32 = ::core::ffi::c_int;
pub type UChar = ::core::ffi::c_uchar;
pub type Char = ::core::ffi::c_char;
pub type Bool = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub handle: *mut FILE,
    pub buffer: Int32,
    pub buffLive: Int32,
    pub mode: Char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const True: Bool = 1 as ::core::ffi::c_int as Bool;
pub const False: Bool = 0 as ::core::ffi::c_int as Bool;
pub const BZ_MAX_FILENAME: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
#[no_mangle]
pub static mut inFileName: [Char; 2000] = [0; 2000];
#[no_mangle]
pub static mut outFileName: [Char; 2000] = [0; 2000];
#[no_mangle]
pub static mut progName: [Char; 2000] = [0; 2000];
#[no_mangle]
pub static mut bytesOut: MaybeUInt64 = 0 as MaybeUInt64;
#[no_mangle]
pub static mut bytesIn: MaybeUInt64 = 0 as MaybeUInt64;
pub const BZ_HDR_B: ::core::ffi::c_int = 0x42 as ::core::ffi::c_int;
pub const BZ_HDR_Z: ::core::ffi::c_int = 0x5a as ::core::ffi::c_int;
pub const BZ_HDR_h: ::core::ffi::c_int = 0x68 as ::core::ffi::c_int;
pub const BZ_HDR_0: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;
unsafe extern "C" fn readError() {
    fprintf(
        stderr,
        b"%s: I/O error reading `%s', possible reason follows.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
        &raw mut inFileName as *mut Char,
    );
    perror(&raw mut progName as *mut Char);
    fprintf(
        stderr,
        b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn writeError() {
    fprintf(
        stderr,
        b"%s: I/O error reading `%s', possible reason follows.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
        &raw mut inFileName as *mut Char,
    );
    perror(&raw mut progName as *mut Char);
    fprintf(
        stderr,
        b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn mallocFail(mut n: Int32) {
    fprintf(
        stderr,
        b"%s: malloc failed on request for %d bytes.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
        n,
    );
    fprintf(
        stderr,
        b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn tooManyBlocks(mut max_handled_blocks: Int32) {
    fprintf(
        stderr,
        b"%s: `%s' appears to contain more than %d blocks\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
        &raw mut inFileName as *mut Char,
        max_handled_blocks,
    );
    fprintf(
        stderr,
        b"%s: and cannot be handled.  To fix, increase\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
    );
    fprintf(
        stderr,
        b"%s: BZ_MAX_HANDLED_BLOCKS in bzip2recover.c, and recompile.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
    );
    exit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn bsOpenReadStream(mut stream: *mut FILE) -> *mut BitStream {
    let mut bs: *mut BitStream =
        malloc(::core::mem::size_of::<BitStream>() as size_t) as *mut BitStream;
    if bs.is_null() {
        mallocFail(::core::mem::size_of::<BitStream>() as Int32);
    }
    (*bs).handle = stream;
    (*bs).buffer = 0 as ::core::ffi::c_int as Int32;
    (*bs).buffLive = 0 as ::core::ffi::c_int as Int32;
    (*bs).mode = 'r' as i32 as Char;
    return bs;
}
unsafe extern "C" fn bsOpenWriteStream(mut stream: *mut FILE) -> *mut BitStream {
    let mut bs: *mut BitStream =
        malloc(::core::mem::size_of::<BitStream>() as size_t) as *mut BitStream;
    if bs.is_null() {
        mallocFail(::core::mem::size_of::<BitStream>() as Int32);
    }
    (*bs).handle = stream;
    (*bs).buffer = 0 as ::core::ffi::c_int as Int32;
    (*bs).buffLive = 0 as ::core::ffi::c_int as Int32;
    (*bs).mode = 'w' as i32 as Char;
    return bs;
}
unsafe extern "C" fn bsPutBit(mut bs: *mut BitStream, mut bit: Int32) {
    if (*bs).buffLive == 8 as ::core::ffi::c_int {
        let mut retVal: Int32 =
            putc((*bs).buffer as UChar as ::core::ffi::c_int, (*bs).handle) as Int32;
        if retVal == EOF {
            writeError();
        }
        bytesOut = bytesOut.wrapping_add(1);
        (*bs).buffLive = 1 as ::core::ffi::c_int as Int32;
        (*bs).buffer = (bit as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int) as Int32;
    } else {
        (*bs).buffer = (((*bs).buffer as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | bit as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int)
            as Int32;
        (*bs).buffLive += 1;
    };
}
unsafe extern "C" fn bsGetBit(mut bs: *mut BitStream) -> Int32 {
    if (*bs).buffLive > 0 as ::core::ffi::c_int {
        (*bs).buffLive -= 1;
        return (*bs).buffer >> (*bs).buffLive & 0x1 as Int32;
    } else {
        let mut retVal: Int32 = getc((*bs).handle) as Int32;
        if retVal == EOF {
            if *__errno_location() != 0 as ::core::ffi::c_int {
                readError();
            }
            return 2 as Int32;
        }
        (*bs).buffLive = 7 as ::core::ffi::c_int as Int32;
        (*bs).buffer = retVal;
        return (*bs).buffer >> 7 as ::core::ffi::c_int & 0x1 as Int32;
    };
}
unsafe extern "C" fn bsClose(mut bs: *mut BitStream) {
    let mut retVal: Int32 = 0;
    if (*bs).mode as ::core::ffi::c_int == 'w' as i32 {
        while (*bs).buffLive < 8 as ::core::ffi::c_int {
            (*bs).buffLive += 1;
            (*bs).buffer <<= 1 as ::core::ffi::c_int;
        }
        retVal = putc((*bs).buffer as UChar as ::core::ffi::c_int, (*bs).handle) as Int32;
        if retVal == EOF {
            writeError();
        }
        bytesOut = bytesOut.wrapping_add(1);
        retVal = fflush((*bs).handle) as Int32;
        if retVal == EOF {
            writeError();
        }
    }
    retVal = fclose((*bs).handle) as Int32;
    if retVal == EOF {
        if (*bs).mode as ::core::ffi::c_int == 'w' as i32 {
            writeError();
        } else {
            readError();
        }
    }
    free(bs as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn bsPutUChar(mut bs: *mut BitStream, mut c: UChar) {
    let mut i: Int32 = 0;
    i = 7 as ::core::ffi::c_int as Int32;
    while i >= 0 as ::core::ffi::c_int {
        bsPutBit(
            bs,
            (c as ::core::ffi::c_uint >> i & 0x1 as ::core::ffi::c_uint) as Int32,
        );
        i -= 1;
    }
}
unsafe extern "C" fn bsPutUInt32(mut bs: *mut BitStream, mut c: UInt32) {
    let mut i: Int32 = 0;
    i = 31 as ::core::ffi::c_int as Int32;
    while i >= 0 as ::core::ffi::c_int {
        bsPutBit(
            bs,
            (c as ::core::ffi::c_uint >> i & 0x1 as ::core::ffi::c_uint) as Int32,
        );
        i -= 1;
    }
}
unsafe extern "C" fn endsInBz2(mut name: *mut Char) -> Bool {
    let mut n: Int32 = strlen(name) as Int32;
    if n <= 4 as ::core::ffi::c_int {
        return False;
    }
    return (*name.offset((n as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        == '.' as i32
        && *name.offset((n as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            == 'b' as i32
        && *name.offset((n as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            == 'z' as i32
        && *name.offset((n as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            == '2' as i32) as ::core::ffi::c_int as Bool;
}
pub const BZ_SPLIT_SYM: ::core::ffi::c_int = '/' as i32;
pub const BLOCK_HEADER_HI: ::core::ffi::c_ulong = 0x3141 as ::core::ffi::c_ulong;
pub const BLOCK_HEADER_LO: ::core::ffi::c_ulong = 0x59265359 as ::core::ffi::c_ulong;
pub const BLOCK_ENDMARK_HI: ::core::ffi::c_ulong = 0x1772 as ::core::ffi::c_ulong;
pub const BLOCK_ENDMARK_LO: ::core::ffi::c_ulong = 0x45385090 as ::core::ffi::c_ulong;
pub const BZ_MAX_HANDLED_BLOCKS: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;
#[no_mangle]
pub static mut bStart: [MaybeUInt64; 50000] = [0; 50000];
#[no_mangle]
pub static mut bEnd: [MaybeUInt64; 50000] = [0; 50000];
#[no_mangle]
pub static mut rbStart: [MaybeUInt64; 50000] = [0; 50000];
#[no_mangle]
pub static mut rbEnd: [MaybeUInt64; 50000] = [0; 50000];
unsafe fn main_0(mut argc: Int32, mut argv: *mut *mut Char) -> Int32 {
    let mut inFile: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut outFile: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut bsIn: *mut BitStream = ::core::ptr::null_mut::<BitStream>();
    let mut bsWr: *mut BitStream = ::core::ptr::null_mut::<BitStream>();
    let mut b: Int32 = 0;
    let mut wrBlock: Int32 = 0;
    let mut currBlock: Int32 = 0;
    let mut rbCtr: Int32 = 0;
    let mut bitsRead: MaybeUInt64 = 0;
    let mut buffHi: UInt32 = 0;
    let mut buffLo: UInt32 = 0;
    let mut blockCRC: UInt32 = 0;
    let mut p: *mut Char = ::core::ptr::null_mut::<Char>();
    strncpy(
        &raw mut progName as *mut ::core::ffi::c_char,
        *argv.offset(0 as ::core::ffi::c_int as isize),
        (BZ_MAX_FILENAME - 1 as ::core::ffi::c_int) as size_t,
    );
    progName[(BZ_MAX_FILENAME - 1 as ::core::ffi::c_int) as usize] = '\0' as i32 as Char;
    outFileName[0 as ::core::ffi::c_int as usize] = 0 as Char;
    inFileName[0 as ::core::ffi::c_int as usize] = outFileName[0 as ::core::ffi::c_int as usize];
    fprintf(
        stderr,
        b"bzip2recover 1.0.8: extracts blocks from damaged .bz2 files.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"%s: usage is `%s damaged_file_name'.\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut progName as *mut Char,
            &raw mut progName as *mut Char,
        );
        match ::core::mem::size_of::<MaybeUInt64>() {
            8 => {
                fprintf(
                    stderr,
                    b"\trestrictions on size of recovered file: None\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            4 => {
                fprintf(
                    stderr,
                    b"\trestrictions on size of recovered file: 512 MB\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fprintf(
                    stderr,
                    b"\tto circumvent, recompile with MaybeUInt64 as an\n\tunsigned 64-bit int.\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            _ => {
                fprintf(
                    stderr,
                    b"\tsizeof(MaybeUInt64) is not 4 or 8 -- configuration error.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
        exit(1 as ::core::ffi::c_int);
    }
    if strlen(*argv.offset(1 as ::core::ffi::c_int as isize))
        >= (BZ_MAX_FILENAME - 20 as ::core::ffi::c_int) as size_t
    {
        fprintf(
            stderr,
            b"%s: supplied filename is suspiciously (>= %d chars) long.  Bye!\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut progName as *mut Char,
            strlen(*argv.offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int,
        );
        exit(1 as ::core::ffi::c_int);
    }
    strcpy(
        &raw mut inFileName as *mut ::core::ffi::c_char,
        *argv.offset(1 as ::core::ffi::c_int as isize),
    );
    inFile = fopen(
        &raw mut inFileName as *mut Char,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if inFile.is_null() {
        fprintf(
            stderr,
            b"%s: can't read `%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut progName as *mut Char,
            &raw mut inFileName as *mut Char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    bsIn = bsOpenReadStream(inFile);
    fprintf(
        stderr,
        b"%s: searching for block boundaries ...\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
    );
    bitsRead = 0 as MaybeUInt64;
    buffLo = 0 as UInt32;
    buffHi = buffLo;
    currBlock = 0 as ::core::ffi::c_int as Int32;
    bStart[currBlock as usize] = 0 as MaybeUInt64;
    rbCtr = 0 as ::core::ffi::c_int as Int32;
    while True != 0 {
        b = bsGetBit(bsIn);
        bitsRead = bitsRead.wrapping_add(1);
        if b == 2 as ::core::ffi::c_int {
            if bitsRead >= bStart[currBlock as usize]
                && bitsRead.wrapping_sub(bStart[currBlock as usize])
                    >= 40 as ::core::ffi::c_ulonglong
            {
                bEnd[currBlock as usize] = (bitsRead as ::core::ffi::c_ulonglong)
                    .wrapping_sub(1 as ::core::ffi::c_ulonglong)
                    as MaybeUInt64;
                if currBlock > 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"   block %d runs from %Lu to %Lu (incomplete)\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        currBlock,
                        bStart[currBlock as usize],
                        bEnd[currBlock as usize],
                    );
                }
            } else {
                currBlock -= 1;
            }
            break;
        } else {
            buffHi = buffHi << 1 as ::core::ffi::c_int | buffLo >> 31 as ::core::ffi::c_int;
            buffLo = ((buffLo as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int
                | (b as ::core::ffi::c_int & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as UInt32;
            if (buffHi as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint)
                as ::core::ffi::c_ulong
                == BLOCK_HEADER_HI
                && buffLo as ::core::ffi::c_ulong == BLOCK_HEADER_LO
                || (buffHi as ::core::ffi::c_uint & 0xffff as ::core::ffi::c_uint)
                    as ::core::ffi::c_ulong
                    == BLOCK_ENDMARK_HI
                    && buffLo as ::core::ffi::c_ulong == BLOCK_ENDMARK_LO
            {
                if bitsRead > 49 as ::core::ffi::c_ulonglong {
                    bEnd[currBlock as usize] = (bitsRead as ::core::ffi::c_ulonglong)
                        .wrapping_sub(49 as ::core::ffi::c_ulonglong)
                        as MaybeUInt64;
                } else {
                    bEnd[currBlock as usize] = 0 as MaybeUInt64;
                }
                if currBlock > 0 as ::core::ffi::c_int
                    && bEnd[currBlock as usize].wrapping_sub(bStart[currBlock as usize])
                        >= 130 as ::core::ffi::c_ulonglong
                {
                    fprintf(
                        stderr,
                        b"   block %d runs from %Lu to %Lu\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        rbCtr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                        bStart[currBlock as usize],
                        bEnd[currBlock as usize],
                    );
                    rbStart[rbCtr as usize] = bStart[currBlock as usize];
                    rbEnd[rbCtr as usize] = bEnd[currBlock as usize];
                    rbCtr += 1;
                }
                if currBlock >= BZ_MAX_HANDLED_BLOCKS {
                    tooManyBlocks(BZ_MAX_HANDLED_BLOCKS);
                }
                currBlock += 1;
                bStart[currBlock as usize] = bitsRead;
            }
        }
    }
    bsClose(bsIn);
    if rbCtr < 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"%s: sorry, I couldn't find any block boundaries.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut progName as *mut Char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    fprintf(
        stderr,
        b"%s: splitting into blocks\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
    );
    inFile = fopen(
        &raw mut inFileName as *mut Char,
        b"rb\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if inFile.is_null() {
        fprintf(
            stderr,
            b"%s: can't open `%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut progName as *mut Char,
            &raw mut inFileName as *mut Char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    bsIn = bsOpenReadStream(inFile);
    blockCRC = 0 as UInt32;
    bsWr = ::core::ptr::null_mut::<BitStream>();
    bitsRead = 0 as MaybeUInt64;
    outFile = ::core::ptr::null_mut::<FILE>();
    wrBlock = 0 as ::core::ffi::c_int as Int32;
    while True != 0 {
        b = bsGetBit(bsIn);
        if b == 2 as ::core::ffi::c_int {
            break;
        }
        buffHi = buffHi << 1 as ::core::ffi::c_int | buffLo >> 31 as ::core::ffi::c_int;
        buffLo = ((buffLo as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int
            | (b as ::core::ffi::c_int & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as UInt32;
        if bitsRead == (47 as MaybeUInt64).wrapping_add(rbStart[wrBlock as usize]) {
            blockCRC = buffHi << 16 as ::core::ffi::c_int | buffLo >> 16 as ::core::ffi::c_int;
        }
        if !outFile.is_null()
            && bitsRead >= rbStart[wrBlock as usize]
            && bitsRead <= rbEnd[wrBlock as usize]
        {
            bsPutBit(bsWr, b);
        }
        bitsRead = bitsRead.wrapping_add(1);
        if bitsRead == rbEnd[wrBlock as usize].wrapping_add(1 as ::core::ffi::c_ulonglong) {
            if !outFile.is_null() {
                bsPutUChar(bsWr, 0x17 as UChar);
                bsPutUChar(bsWr, 0x72 as UChar);
                bsPutUChar(bsWr, 0x45 as UChar);
                bsPutUChar(bsWr, 0x38 as UChar);
                bsPutUChar(bsWr, 0x50 as UChar);
                bsPutUChar(bsWr, 0x90 as UChar);
                bsPutUInt32(bsWr, blockCRC);
                bsClose(bsWr);
                outFile = ::core::ptr::null_mut::<FILE>();
            }
            if wrBlock >= rbCtr {
                break;
            }
            wrBlock += 1;
        } else if bitsRead == rbStart[wrBlock as usize] {
            let mut split: *mut Char = ::core::ptr::null_mut::<Char>();
            let mut ofs: Int32 = 0;
            let mut k: Int32 = 0;
            k = 0 as ::core::ffi::c_int as Int32;
            while k < BZ_MAX_FILENAME {
                outFileName[k as usize] = 0 as Char;
                k += 1;
            }
            strcpy(
                &raw mut outFileName as *mut ::core::ffi::c_char,
                &raw mut inFileName as *mut Char,
            );
            split = strrchr(&raw mut outFileName as *mut Char, BZ_SPLIT_SYM) as *mut Char;
            if split.is_null() {
                split = &raw mut outFileName as *mut Char;
            } else {
                split = split.offset(1);
            }
            ofs = split.offset_from(&raw mut outFileName as *mut Char) as ::core::ffi::c_long
                as Int32;
            sprintf(
                split as *mut ::core::ffi::c_char,
                b"rec%5d\0" as *const u8 as *const ::core::ffi::c_char,
                wrBlock as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
            );
            p = split;
            while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if *p as ::core::ffi::c_int == ' ' as i32 {
                    *p = '0' as i32 as Char;
                }
                p = p.offset(1);
            }
            strcat(
                &raw mut outFileName as *mut ::core::ffi::c_char,
                (&raw mut inFileName as *mut Char).offset(ofs as isize),
            );
            if endsInBz2(&raw mut outFileName as *mut Char) == 0 {
                strcat(
                    &raw mut outFileName as *mut ::core::ffi::c_char,
                    b".bz2\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            fprintf(
                stderr,
                b"   writing block %d to `%s' ...\n\0" as *const u8 as *const ::core::ffi::c_char,
                wrBlock as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                &raw mut outFileName as *mut Char,
            );
            outFile = fopen(
                &raw mut outFileName as *mut Char,
                b"wb\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if outFile.is_null() {
                fprintf(
                    stderr,
                    b"%s: can't write `%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut progName as *mut Char,
                    &raw mut outFileName as *mut Char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            bsWr = bsOpenWriteStream(outFile);
            bsPutUChar(bsWr, BZ_HDR_B as UChar);
            bsPutUChar(bsWr, BZ_HDR_Z as UChar);
            bsPutUChar(bsWr, BZ_HDR_h as UChar);
            bsPutUChar(bsWr, (BZ_HDR_0 + 9 as ::core::ffi::c_int) as UChar);
            bsPutUChar(bsWr, 0x31 as UChar);
            bsPutUChar(bsWr, 0x41 as UChar);
            bsPutUChar(bsWr, 0x59 as UChar);
            bsPutUChar(bsWr, 0x26 as UChar);
            bsPutUChar(bsWr, 0x53 as UChar);
            bsPutUChar(bsWr, 0x59 as UChar);
        }
    }
    fprintf(
        stderr,
        b"%s: finished\n\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut progName as *mut Char,
    );
    return 0 as Int32;
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
            (args_ptrs.len() - 1) as Int32,
            args_ptrs.as_mut_ptr() as *mut *mut Char,
        ) as i32)
    }
}
