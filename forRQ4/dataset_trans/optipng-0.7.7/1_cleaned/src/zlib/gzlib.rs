use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type internal_state;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;

pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub x: gzFile_s,
    pub mode: c_int,
    pub fd: c_int,
    pub path: *mut c_char,
    pub size: c_uint,
    pub want: c_uint,
    pub in_0: *mut c_uchar,
    pub out: *mut c_uchar,
    pub direct: c_int,
    pub how: c_int,
    pub start: off_t,
    pub eof: c_int,
    pub past: c_int,
    pub level: c_int,
    pub strategy: c_int,
    pub skip: off_t,
    pub seek: c_int,
    pub err: c_int,
    pub msg: *mut c_char,
    pub strm: z_stream,
}

pub const O_RDONLY: c_int = 0 as c_int;
pub const O_WRONLY: c_int = 0o1 as c_int;
pub const O_CREAT: c_int = 0o100 as c_int;
pub const O_EXCL: c_int = 0o200 as c_int;
pub const O_TRUNC: c_int = 0o1000 as c_int;
pub const O_APPEND: c_int = 0o2000 as c_int;
pub const __O_CLOEXEC: c_int = 0o2000000 as c_int;
pub const O_CLOEXEC: c_int = __O_CLOEXEC;
pub const Z_OK: c_int = 0 as c_int;
pub const Z_MEM_ERROR: c_int = -(4 as c_int);
pub const Z_BUF_ERROR: c_int = -(5 as c_int);

unsafe extern "C" fn gz_reset(mut state: gz_statep) {
    (*state).x.have = 0 as c_uint;
    if (*state).mode == GZ_READ {
        (*state).eof = 0 as c_int;
        (*state).past = 0 as c_int;
        (*state).how = LOOK;
    }
    (*state).seek = 0 as c_int;
    gz_error(state, Z_OK, ::core::ptr::null::<c_char>());
    (*state).x.pos = 0 as off_t;
    (*state).strm.avail_in = 0 as uInt;
}
unsafe extern "C" fn gz_open(
    mut path: *const c_void,
    mut fd: c_int,
    mut mode: *const c_char,
) -> gzFile {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    let mut len: z_size_t = 0;
    let mut oflag: c_int = 0;
    let mut cloexec: c_int = 0 as c_int;
    let mut exclusive: c_int = 0 as c_int;
    if path.is_null() {
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    state = malloc(::core::mem::size_of::<gz_state>() as size_t) as gz_statep;
    if state.is_null() {
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    (*state).size = 0 as c_uint;
    (*state).want = GZBUFSIZE as c_uint;
    (*state).msg = ::core::ptr::null_mut::<c_char>();
    (*state).mode = GZ_NONE;
    (*state).level = Z_DEFAULT_COMPRESSION;
    (*state).strategy = Z_DEFAULT_STRATEGY;
    (*state).direct = 0 as c_int;
    while *mode != 0 {
        if *mode as c_int >= '0' as i32 && *mode as c_int <= '9' as i32 {
            (*state).level = *mode as c_int - '0' as i32;
        } else {
            match *mode as c_int {
                114 => {
                    (*state).mode = GZ_READ;
                }
                43 => {
                    free(state as *mut c_void);
                    return ::core::ptr::null_mut::<gzFile_s>();
                }
                101 => {
                    cloexec = 1 as c_int;
                }
                120 => {
                    exclusive = 1 as c_int;
                }
                102 => {
                    (*state).strategy = Z_FILTERED;
                }
                104 => {
                    (*state).strategy = Z_HUFFMAN_ONLY;
                }
                82 => {
                    (*state).strategy = Z_RLE;
                }
                70 => {
                    (*state).strategy = Z_FIXED;
                }
                84 => {
                    (*state).direct = 1 as c_int;
                }
                98 | _ => {}
            }
        }
        mode = mode.offset(1);
    }
    if (*state).mode == GZ_NONE {
        free(state as *mut c_void);
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    if (*state).mode == GZ_READ {
        if (*state).direct != 0 {
            free(state as *mut c_void);
            return ::core::ptr::null_mut::<gzFile_s>();
        }
        (*state).direct = 1 as c_int;
    }
    len = strlen(path as *const c_char) as z_size_t;
    (*state).path = malloc((len as size_t).wrapping_add(1 as size_t)) as *mut c_char;
    if (*state).path.is_null() {
        free(state as *mut c_void);
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    snprintf(
        (*state).path,
        (len as size_t).wrapping_add(1 as size_t),
        b"%s\0" as *const u8 as *const c_char,
        path as *const c_char,
    );
    oflag = (if cloexec != 0 {
        O_CLOEXEC
    } else {
        0 as c_int
    }) | (if (*state).mode == GZ_READ {
        O_RDONLY
    } else {
        O_WRONLY
            | O_CREAT
            | (if exclusive != 0 {
                O_EXCL
            } else {
                0 as c_int
            })
            | (if (*state).mode == GZ_WRITE {
                O_TRUNC
            } else {
                O_APPEND
            })
    });
    (*state).fd = if fd > -(1 as c_int) {
        fd
    } else {
        open(
            path as *const c_char,
            oflag,
            0o666 as c_int,
        )
    };
    if (*state).fd == -(1 as c_int) {
        free((*state).path as *mut c_void);
        free(state as *mut c_void);
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    if (*state).mode == GZ_APPEND {
        lseek((*state).fd, 0 as __off_t, SEEK_END);
        (*state).mode = GZ_WRITE;
    }
    if (*state).mode == GZ_READ {
        (*state).start = lseek((*state).fd, 0 as __off_t, SEEK_CUR) as off_t;
        if (*state).start == -(1 as c_int) as c_long {
            (*state).start = 0 as off_t;
        }
    }
    gz_reset(state);
    return state as gzFile;
}
#[no_mangle]
pub unsafe extern "C" fn gzopen(
    mut path: *const c_char,
    mut mode: *const c_char,
) -> gzFile {
    return gz_open(
        path as *const c_void,
        -(1 as c_int),
        mode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gzopen64(
    mut path: *const c_char,
    mut mode: *const c_char,
) -> gzFile {
    return gz_open(
        path as *const c_void,
        -(1 as c_int),
        mode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gzdopen(
    mut fd: c_int,
    mut mode: *const c_char,
) -> gzFile {
    let mut path: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut gz: gzFile = ::core::ptr::null_mut::<gzFile_s>();
    if fd == -(1 as c_int) || {
        path = malloc((7 as size_t).wrapping_add(
            (3 as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
        )) as *mut c_char;
        path.is_null()
    } {
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    snprintf(
        path,
        (7 as size_t).wrapping_add(
            (3 as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
        ),
        b"<fd:%d>\0" as *const u8 as *const c_char,
        fd,
    );
    gz = gz_open(path as *const c_void, fd, mode);
    free(path as *mut c_void);
    return gz;
}
#[no_mangle]
pub unsafe extern "C" fn gzbuffer(
    mut file: gzFile,
    mut size: c_uint,
) -> c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as c_int);
    }
    if (*state).size != 0 as c_uint {
        return -(1 as c_int);
    }
    if (size << 1 as c_int) < size {
        return -(1 as c_int);
    }
    if size < 2 as c_uint {
        size = 2 as c_uint;
    }
    (*state).want = size;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gzrewind(mut file: gzFile) -> c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ || (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as c_int);
    }
    if lseek((*state).fd, (*state).start as __off_t, SEEK_SET)
        == -(1 as c_int) as c_long
    {
        return -(1 as c_int);
    }
    gz_reset(state);
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gzseek64(
    mut file: gzFile,
    mut offset: off_t,
    mut whence: c_int,
) -> off_t {
    let mut n: c_uint = 0;
    let mut ret: off_t = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int) as off_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as c_int) as off_t;
    }
    if (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as c_int) as off_t;
    }
    if whence != SEEK_SET && whence != SEEK_CUR {
        return -(1 as c_int) as off_t;
    }
    if whence == SEEK_SET {
        offset -= (*state).x.pos as c_long;
    } else if (*state).seek != 0 {
        offset += (*state).skip as c_long;
    }
    (*state).seek = 0 as c_int;
    if (*state).mode == GZ_READ
        && (*state).how == COPY
        && (*state).x.pos + offset >= 0 as c_long
    {
        ret = lseek(
            (*state).fd,
            offset as __off_t - (*state).x.have as __off_t,
            SEEK_CUR,
        ) as off_t;
        if ret == -(1 as c_int) as c_long {
            return -(1 as c_int) as off_t;
        }
        (*state).x.have = 0 as c_uint;
        (*state).eof = 0 as c_int;
        (*state).past = 0 as c_int;
        (*state).seek = 0 as c_int;
        gz_error(state, Z_OK, ::core::ptr::null::<c_char>());
        (*state).strm.avail_in = 0 as uInt;
        (*state).x.pos += offset as c_long;
        return (*state).x.pos;
    }
    if offset < 0 as c_long {
        if (*state).mode != GZ_READ {
            return -(1 as c_int) as off_t;
        }
        offset += (*state).x.pos as c_long;
        if offset < 0 as c_long {
            return -(1 as c_int) as off_t;
        }
        if gzrewind(file) == -(1 as c_int) {
            return -(1 as c_int) as off_t;
        }
    }
    if (*state).mode == GZ_READ {
        n = if ::core::mem::size_of::<c_int>() as usize
            == ::core::mem::size_of::<off_t>() as usize
            && (*state).x.have > INT_MAX as c_uint
            || (*state).x.have as off_t > offset
        {
            offset as c_uint
        } else {
            (*state).x.have
        };
        (*state).x.have = (*state).x.have.wrapping_sub(n);
        (*state).x.next = (*state).x.next.offset(n as isize);
        (*state).x.pos += n as c_long;
        offset -= n as c_long;
    }
    if offset != 0 {
        (*state).seek = 1 as c_int;
        (*state).skip = offset;
    }
    return (*state).x.pos + offset;
}
#[no_mangle]
pub unsafe extern "C" fn gzseek(
    mut file: gzFile,
    mut offset: off_t,
    mut whence: c_int,
) -> off_t {
    let mut ret: off_t = 0;
    ret = gzseek64(file, offset, whence);
    return if ret == ret {
        ret
    } else {
        -(1 as c_int) as off_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gztell64(mut file: gzFile) -> off_t {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int) as off_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as c_int) as off_t;
    }
    return (*state).x.pos
        + (if (*state).seek != 0 {
            (*state).skip
        } else {
            0 as off_t
        });
}
#[no_mangle]
pub unsafe extern "C" fn gztell(mut file: gzFile) -> off_t {
    let mut ret: off_t = 0;
    ret = gztell64(file);
    return if ret == ret {
        ret
    } else {
        -(1 as c_int) as off_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzoffset64(mut file: gzFile) -> off_t {
    let mut offset: off_t = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as c_int) as off_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as c_int) as off_t;
    }
    offset = lseek((*state).fd, 0 as __off_t, SEEK_CUR) as off_t;
    if offset == -(1 as c_int) as c_long {
        return -(1 as c_int) as off_t;
    }
    if (*state).mode == GZ_READ {
        offset -= (*state).strm.avail_in as c_long;
    }
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn gzoffset(mut file: gzFile) -> off_t {
    let mut ret: off_t = 0;
    ret = gzoffset64(file);
    return if ret == ret {
        ret
    } else {
        -(1 as c_int) as off_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzeof(mut file: gzFile) -> c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return 0 as c_int;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return 0 as c_int;
    }
    return if (*state).mode == GZ_READ {
        (*state).past
    } else {
        0 as c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzerror(
    mut file: gzFile,
    mut errnum: *mut c_int,
) -> *const c_char {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return ::core::ptr::null::<c_char>();
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return ::core::ptr::null::<c_char>();
    }
    if !errnum.is_null() {
        *errnum = (*state).err;
    }
    return if (*state).err == Z_MEM_ERROR {
        b"out of memory\0" as *const u8 as *const c_char
    } else if (*state).msg.is_null() {
        b"\0" as *const u8 as *const c_char
    } else {
        (*state).msg as *const c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzclearerr(mut file: gzFile) {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return;
    }
    if (*state).mode == GZ_READ {
        (*state).eof = 0 as c_int;
        (*state).past = 0 as c_int;
    }
    gz_error(state, Z_OK, ::core::ptr::null::<c_char>());
}
#[no_mangle]
pub unsafe extern "C" fn gz_error(
    mut state: gz_statep,
    mut err: c_int,
    mut msg: *const c_char,
) {
    if !(*state).msg.is_null() {
        if (*state).err != Z_MEM_ERROR {
            free((*state).msg as *mut c_void);
        }
        (*state).msg = ::core::ptr::null_mut::<c_char>();
    }
    if err != Z_OK && err != Z_BUF_ERROR {
        (*state).x.have = 0 as c_uint;
    }
    (*state).err = err;
    if msg.is_null() {
        return;
    }
    if err == Z_MEM_ERROR {
        return;
    }
    (*state).msg = malloc(
        strlen((*state).path)
            .wrapping_add(strlen(msg))
            .wrapping_add(3 as size_t),
    ) as *mut c_char;
    if (*state).msg.is_null() {
        (*state).err = Z_MEM_ERROR;
        return;
    }
    snprintf(
        (*state).msg,
        strlen((*state).path)
            .wrapping_add(strlen(msg))
            .wrapping_add(3 as size_t),
        b"%s%s%s\0" as *const u8 as *const c_char,
        (*state).path,
        b": \0" as *const u8 as *const c_char,
        msg,
    );
}

pub const GZBUFSIZE: c_int = 8192 as c_int;
pub const GZ_NONE: c_int = 0 as c_int;

pub const GZ_APPEND: c_int = 1 as c_int;

pub const COPY: c_int = 1 as c_int;

