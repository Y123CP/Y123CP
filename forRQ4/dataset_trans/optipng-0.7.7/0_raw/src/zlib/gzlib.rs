extern "C" {
    pub type internal_state;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type z_size_t = size_t;
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut ::core::ffi::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: ::core::ffi::c_uint,
    pub next: *mut ::core::ffi::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub x: gzFile_s,
    pub mode: ::core::ffi::c_int,
    pub fd: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub size: ::core::ffi::c_uint,
    pub want: ::core::ffi::c_uint,
    pub in_0: *mut ::core::ffi::c_uchar,
    pub out: *mut ::core::ffi::c_uchar,
    pub direct: ::core::ffi::c_int,
    pub how: ::core::ffi::c_int,
    pub start: off_t,
    pub eof: ::core::ffi::c_int,
    pub past: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub strategy: ::core::ffi::c_int,
    pub skip: off_t,
    pub seek: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
    pub msg: *mut ::core::ffi::c_char,
    pub strm: z_stream,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_MEM_ERROR: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const Z_BUF_ERROR: ::core::ffi::c_int = -(5 as ::core::ffi::c_int);
pub const Z_DEFAULT_COMPRESSION: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const Z_FILTERED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Z_HUFFMAN_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Z_RLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const Z_FIXED: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Z_DEFAULT_STRATEGY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn gz_reset(mut state: gz_statep) {
    (*state).x.have = 0 as ::core::ffi::c_uint;
    if (*state).mode == GZ_READ {
        (*state).eof = 0 as ::core::ffi::c_int;
        (*state).past = 0 as ::core::ffi::c_int;
        (*state).how = LOOK;
    }
    (*state).seek = 0 as ::core::ffi::c_int;
    gz_error(state, Z_OK, ::core::ptr::null::<::core::ffi::c_char>());
    (*state).x.pos = 0 as off_t;
    (*state).strm.avail_in = 0 as uInt;
}
unsafe extern "C" fn gz_open(
    mut path: *const ::core::ffi::c_void,
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> gzFile {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    let mut len: z_size_t = 0;
    let mut oflag: ::core::ffi::c_int = 0;
    let mut cloexec: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut exclusive: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if path.is_null() {
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    state = malloc(::core::mem::size_of::<gz_state>() as size_t) as gz_statep;
    if state.is_null() {
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    (*state).size = 0 as ::core::ffi::c_uint;
    (*state).want = GZBUFSIZE as ::core::ffi::c_uint;
    (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).mode = GZ_NONE;
    (*state).level = Z_DEFAULT_COMPRESSION;
    (*state).strategy = Z_DEFAULT_STRATEGY;
    (*state).direct = 0 as ::core::ffi::c_int;
    while *mode != 0 {
        if *mode as ::core::ffi::c_int >= '0' as i32 && *mode as ::core::ffi::c_int <= '9' as i32 {
            (*state).level = *mode as ::core::ffi::c_int - '0' as i32;
        } else {
            match *mode as ::core::ffi::c_int {
                114 => {
                    (*state).mode = GZ_READ;
                }
                43 => {
                    free(state as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<gzFile_s>();
                }
                101 => {
                    cloexec = 1 as ::core::ffi::c_int;
                }
                120 => {
                    exclusive = 1 as ::core::ffi::c_int;
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
                    (*state).direct = 1 as ::core::ffi::c_int;
                }
                98 | _ => {}
            }
        }
        mode = mode.offset(1);
    }
    if (*state).mode == GZ_NONE {
        free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    if (*state).mode == GZ_READ {
        if (*state).direct != 0 {
            free(state as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<gzFile_s>();
        }
        (*state).direct = 1 as ::core::ffi::c_int;
    }
    len = strlen(path as *const ::core::ffi::c_char) as z_size_t;
    (*state).path = malloc((len as size_t).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if (*state).path.is_null() {
        free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    snprintf(
        (*state).path,
        (len as size_t).wrapping_add(1 as size_t),
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        path as *const ::core::ffi::c_char,
    );
    oflag = (if cloexec != 0 {
        O_CLOEXEC
    } else {
        0 as ::core::ffi::c_int
    }) | (if (*state).mode == GZ_READ {
        O_RDONLY
    } else {
        O_WRONLY
            | O_CREAT
            | (if exclusive != 0 {
                O_EXCL
            } else {
                0 as ::core::ffi::c_int
            })
            | (if (*state).mode == GZ_WRITE {
                O_TRUNC
            } else {
                O_APPEND
            })
    });
    (*state).fd = if fd > -(1 as ::core::ffi::c_int) {
        fd
    } else {
        open(
            path as *const ::core::ffi::c_char,
            oflag,
            0o666 as ::core::ffi::c_int,
        )
    };
    if (*state).fd == -(1 as ::core::ffi::c_int) {
        free((*state).path as *mut ::core::ffi::c_void);
        free(state as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    if (*state).mode == GZ_APPEND {
        lseek((*state).fd, 0 as __off_t, SEEK_END);
        (*state).mode = GZ_WRITE;
    }
    if (*state).mode == GZ_READ {
        (*state).start = lseek((*state).fd, 0 as __off_t, SEEK_CUR) as off_t;
        if (*state).start == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
            (*state).start = 0 as off_t;
        }
    }
    gz_reset(state);
    return state as gzFile;
}
#[no_mangle]
pub unsafe extern "C" fn gzopen(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> gzFile {
    return gz_open(
        path as *const ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        mode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gzopen64(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> gzFile {
    return gz_open(
        path as *const ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        mode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn gzdopen(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> gzFile {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gz: gzFile = ::core::ptr::null_mut::<gzFile_s>();
    if fd == -(1 as ::core::ffi::c_int) || {
        path = malloc((7 as size_t).wrapping_add(
            (3 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        )) as *mut ::core::ffi::c_char;
        path.is_null()
    } {
        return ::core::ptr::null_mut::<gzFile_s>();
    }
    snprintf(
        path,
        (7 as size_t).wrapping_add(
            (3 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        ),
        b"<fd:%d>\0" as *const u8 as *const ::core::ffi::c_char,
        fd,
    );
    gz = gz_open(path as *const ::core::ffi::c_void, fd, mode);
    free(path as *mut ::core::ffi::c_void);
    return gz;
}
#[no_mangle]
pub unsafe extern "C" fn gzbuffer(
    mut file: gzFile,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as ::core::ffi::c_int);
    }
    if (*state).size != 0 as ::core::ffi::c_uint {
        return -(1 as ::core::ffi::c_int);
    }
    if (size << 1 as ::core::ffi::c_int) < size {
        return -(1 as ::core::ffi::c_int);
    }
    if size < 2 as ::core::ffi::c_uint {
        size = 2 as ::core::ffi::c_uint;
    }
    (*state).want = size;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gzrewind(mut file: gzFile) -> ::core::ffi::c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ || (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as ::core::ffi::c_int);
    }
    if lseek((*state).fd, (*state).start as __off_t, SEEK_SET)
        == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
    {
        return -(1 as ::core::ffi::c_int);
    }
    gz_reset(state);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gzseek64(
    mut file: gzFile,
    mut offset: off_t,
    mut whence: ::core::ffi::c_int,
) -> off_t {
    let mut n: ::core::ffi::c_uint = 0;
    let mut ret: off_t = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as ::core::ffi::c_int) as off_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as ::core::ffi::c_int) as off_t;
    }
    if (*state).err != Z_OK && (*state).err != Z_BUF_ERROR {
        return -(1 as ::core::ffi::c_int) as off_t;
    }
    if whence != SEEK_SET && whence != SEEK_CUR {
        return -(1 as ::core::ffi::c_int) as off_t;
    }
    if whence == SEEK_SET {
        offset -= (*state).x.pos as ::core::ffi::c_long;
    } else if (*state).seek != 0 {
        offset += (*state).skip as ::core::ffi::c_long;
    }
    (*state).seek = 0 as ::core::ffi::c_int;
    if (*state).mode == GZ_READ
        && (*state).how == COPY
        && (*state).x.pos + offset >= 0 as ::core::ffi::c_long
    {
        ret = lseek(
            (*state).fd,
            offset as __off_t - (*state).x.have as __off_t,
            SEEK_CUR,
        ) as off_t;
        if ret == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
            return -(1 as ::core::ffi::c_int) as off_t;
        }
        (*state).x.have = 0 as ::core::ffi::c_uint;
        (*state).eof = 0 as ::core::ffi::c_int;
        (*state).past = 0 as ::core::ffi::c_int;
        (*state).seek = 0 as ::core::ffi::c_int;
        gz_error(state, Z_OK, ::core::ptr::null::<::core::ffi::c_char>());
        (*state).strm.avail_in = 0 as uInt;
        (*state).x.pos += offset as ::core::ffi::c_long;
        return (*state).x.pos;
    }
    if offset < 0 as ::core::ffi::c_long {
        if (*state).mode != GZ_READ {
            return -(1 as ::core::ffi::c_int) as off_t;
        }
        offset += (*state).x.pos as ::core::ffi::c_long;
        if offset < 0 as ::core::ffi::c_long {
            return -(1 as ::core::ffi::c_int) as off_t;
        }
        if gzrewind(file) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int) as off_t;
        }
    }
    if (*state).mode == GZ_READ {
        n = if ::core::mem::size_of::<::core::ffi::c_int>() as usize
            == ::core::mem::size_of::<off_t>() as usize
            && (*state).x.have > INT_MAX as ::core::ffi::c_uint
            || (*state).x.have as off_t > offset
        {
            offset as ::core::ffi::c_uint
        } else {
            (*state).x.have
        };
        (*state).x.have = (*state).x.have.wrapping_sub(n);
        (*state).x.next = (*state).x.next.offset(n as isize);
        (*state).x.pos += n as ::core::ffi::c_long;
        offset -= n as ::core::ffi::c_long;
    }
    if offset != 0 {
        (*state).seek = 1 as ::core::ffi::c_int;
        (*state).skip = offset;
    }
    return (*state).x.pos + offset;
}
#[no_mangle]
pub unsafe extern "C" fn gzseek(
    mut file: gzFile,
    mut offset: off_t,
    mut whence: ::core::ffi::c_int,
) -> off_t {
    let mut ret: off_t = 0;
    ret = gzseek64(file, offset, whence);
    return if ret == ret {
        ret
    } else {
        -(1 as ::core::ffi::c_int) as off_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gztell64(mut file: gzFile) -> off_t {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as ::core::ffi::c_int) as off_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as ::core::ffi::c_int) as off_t;
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
        -(1 as ::core::ffi::c_int) as off_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzoffset64(mut file: gzFile) -> off_t {
    let mut offset: off_t = 0;
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return -(1 as ::core::ffi::c_int) as off_t;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return -(1 as ::core::ffi::c_int) as off_t;
    }
    offset = lseek((*state).fd, 0 as __off_t, SEEK_CUR) as off_t;
    if offset == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
        return -(1 as ::core::ffi::c_int) as off_t;
    }
    if (*state).mode == GZ_READ {
        offset -= (*state).strm.avail_in as ::core::ffi::c_long;
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
        -(1 as ::core::ffi::c_int) as off_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzeof(mut file: gzFile) -> ::core::ffi::c_int {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return 0 as ::core::ffi::c_int;
    }
    return if (*state).mode == GZ_READ {
        (*state).past
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn gzerror(
    mut file: gzFile,
    mut errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut state: gz_statep = ::core::ptr::null_mut::<gz_state>();
    if file.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    state = file as gz_statep;
    if (*state).mode != GZ_READ && (*state).mode != GZ_WRITE {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !errnum.is_null() {
        *errnum = (*state).err;
    }
    return if (*state).err == Z_MEM_ERROR {
        b"out of memory\0" as *const u8 as *const ::core::ffi::c_char
    } else if (*state).msg.is_null() {
        b"\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        (*state).msg as *const ::core::ffi::c_char
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
        (*state).eof = 0 as ::core::ffi::c_int;
        (*state).past = 0 as ::core::ffi::c_int;
    }
    gz_error(state, Z_OK, ::core::ptr::null::<::core::ffi::c_char>());
}
#[no_mangle]
pub unsafe extern "C" fn gz_error(
    mut state: gz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    if !(*state).msg.is_null() {
        if (*state).err != Z_MEM_ERROR {
            free((*state).msg as *mut ::core::ffi::c_void);
        }
        (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if err != Z_OK && err != Z_BUF_ERROR {
        (*state).x.have = 0 as ::core::ffi::c_uint;
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
    ) as *mut ::core::ffi::c_char;
    if (*state).msg.is_null() {
        (*state).err = Z_MEM_ERROR;
        return;
    }
    snprintf(
        (*state).msg,
        strlen((*state).path)
            .wrapping_add(strlen(msg))
            .wrapping_add(3 as size_t),
        b"%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        (*state).path,
        b": \0" as *const u8 as *const ::core::ffi::c_char,
        msg,
    );
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GZBUFSIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const GZ_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GZ_READ: ::core::ffi::c_int = 7247 as ::core::ffi::c_int;
pub const GZ_WRITE: ::core::ffi::c_int = 31153 as ::core::ffi::c_int;
pub const GZ_APPEND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOOK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COPY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
