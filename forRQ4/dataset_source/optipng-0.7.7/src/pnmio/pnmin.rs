extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ungetc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn pnm_is_valid(pnm_ptr: *const pnm_struct) -> ::core::ffi::c_int;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pnm_struct {
    pub format: ::core::ffi::c_uint,
    pub depth: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub maxval: ::core::ffi::c_uint,
}
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn pnm_fget_char(mut stream: *mut FILE) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = getc(stream);
    if ch == '#' as i32 {
        loop {
            ch = getc(stream);
            if !(ch != EOF && ch != '\n' as i32 && ch != '\r' as i32) {
                break;
            }
        }
    }
    if ch == '\r' as i32 {
        ch = getc(stream);
        if ch != '\n' as i32 {
            ungetc(ch, stream);
            ch = '\n' as i32;
        }
    }
    return ch;
}
unsafe extern "C" fn pnm_fscan_uint(
    mut stream: *mut FILE,
    mut value: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    let mut tmp: ::core::ffi::c_uint = 0;
    loop {
        ch = pnm_fget_char(stream);
        if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
            break;
        }
    }
    if ch == EOF {
        return EOF;
    }
    if !(ch >= '0' as i32 && ch <= '9' as i32) {
        ungetc(ch, stream);
        return 0 as ::core::ffi::c_int;
    }
    *value = 0 as ::core::ffi::c_uint;
    loop {
        tmp = (*value)
            .wrapping_mul(10 as ::core::ffi::c_uint)
            .wrapping_add((ch - '0' as i32) as ::core::ffi::c_uint);
        if tmp >= *value {
            *value = tmp;
        } else {
            *value = UINT_MAX;
            *__errno_location() = ERANGE;
        }
        ch = getc(stream);
        if !(ch >= '0' as i32 && ch <= '9' as i32) {
            break;
        }
    }
    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
        ungetc(ch, stream);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_header(
    mut pnm_ptr: *mut pnm_struct,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut format: ::core::ffi::c_uint = 0;
    let mut ch: ::core::ffi::c_int = 0;
    memset(
        pnm_ptr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pnm_struct>() as size_t,
    );
    ch = getc(stream);
    if ch == EOF {
        return -(1 as ::core::ffi::c_int);
    }
    if ch != 'P' as i32 {
        return -(1 as ::core::ffi::c_int);
    }
    ch = getc(stream);
    if ch < '1' as i32 || ch > '9' as i32 {
        return -(1 as ::core::ffi::c_int);
    }
    format = (ch - '0' as i32) as ::core::ffi::c_uint;
    ch = pnm_fget_char(stream);
    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
        return -(1 as ::core::ffi::c_int);
    }
    (*pnm_ptr).format = format;
    if format >= PNM_P1 as ::core::ffi::c_int as ::core::ffi::c_uint
        && format <= PNM_P6 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*pnm_ptr).depth = (if format == PNM_P3 as ::core::ffi::c_int as ::core::ffi::c_uint
            || format == PNM_P6 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
        if pnm_fscan_uint(stream, &raw mut (*pnm_ptr).width) != 1 as ::core::ffi::c_int
            || pnm_fscan_uint(stream, &raw mut (*pnm_ptr).height) != 1 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        if format == PNM_P1 as ::core::ffi::c_int as ::core::ffi::c_uint
            || format == PNM_P4 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*pnm_ptr).maxval = 1 as ::core::ffi::c_uint;
        } else if pnm_fscan_uint(stream, &raw mut (*pnm_ptr).maxval) != 1 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        return if pnm_is_valid(pnm_ptr) != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_values(
    mut pnm_ptr: *const pnm_struct,
    mut sample_values: *mut ::core::ffi::c_uint,
    mut num_rows: ::core::ffi::c_uint,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut format: ::core::ffi::c_uint = (*pnm_ptr).format;
    let mut depth: ::core::ffi::c_uint = (*pnm_ptr).depth;
    let mut width: ::core::ffi::c_uint = (*pnm_ptr).width;
    let mut maxval: ::core::ffi::c_uint = (*pnm_ptr).maxval;
    let mut row_length: size_t = (depth as size_t).wrapping_mul(width as size_t);
    let mut num_samples: size_t = (num_rows as size_t).wrapping_mul(row_length);
    let mut ch: ::core::ffi::c_int = 0;
    let mut ch8: ::core::ffi::c_int = 0;
    let mut ch16: ::core::ffi::c_int = 0;
    let mut ch24: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    match format {
        1 => {
            i = 0 as size_t;
            while i < num_samples {
                loop {
                    ch = pnm_fget_char(stream);
                    if !(ch == ' ' as i32
                        || ch == '\t' as i32
                        || ch == '\n' as i32
                        || ch == '\r' as i32)
                    {
                        break;
                    }
                }
                if ch != '0' as i32 && ch != '1' as i32 {
                    ungetc(ch, stream);
                    break;
                } else {
                    *sample_values.offset(i as isize) = (if ch == '0' as i32 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                        as ::core::ffi::c_uint;
                    i = i.wrapping_add(1);
                }
            }
        }
        2 | 3 => {
            i = 0 as size_t;
            while i < num_samples {
                if pnm_fscan_uint(
                    stream,
                    sample_values.offset(i as isize) as *mut ::core::ffi::c_uint,
                ) != 1 as ::core::ffi::c_int
                {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        4 => {
            j = 0 as size_t;
            i = j;
            while i < num_samples {
                ch = getc(stream);
                if ch == EOF {
                    break;
                }
                mask = 0x80 as ::core::ffi::c_int;
                while mask != 0 as ::core::ffi::c_int {
                    let fresh0 = i;
                    i = i.wrapping_add(1);
                    *sample_values.offset(fresh0 as isize) = (if ch & mask != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                        as ::core::ffi::c_uint;
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as size_t;
                        break;
                    } else {
                        mask >>= 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
        5 | 6 | 7 => {
            if maxval <= 0xff as ::core::ffi::c_uint {
                i = 0 as size_t;
                while i < num_samples {
                    ch = getc(stream);
                    if ch == EOF {
                        break;
                    }
                    *sample_values.offset(i as isize) = ch as ::core::ffi::c_uint;
                    i = i.wrapping_add(1);
                }
            } else if maxval <= 0xffff as ::core::ffi::c_uint {
                i = 0 as size_t;
                while i < num_samples {
                    ch8 = getc(stream);
                    ch = getc(stream);
                    if ch == EOF {
                        break;
                    }
                    *sample_values.offset(i as isize) = ((ch8 as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int)
                        .wrapping_add(ch as ::core::ffi::c_uint);
                    i = i.wrapping_add(1);
                }
            } else if maxval <= 0xffffffff as ::core::ffi::c_uint {
                ch24 = 0 as ::core::ffi::c_int;
                i = 0 as size_t;
                while i < num_samples {
                    if maxval > 0xffffff as ::core::ffi::c_uint {
                        ch24 = getc(stream);
                    }
                    ch16 = getc(stream);
                    ch8 = getc(stream);
                    ch = getc(stream);
                    if ch == EOF {
                        break;
                    }
                    *sample_values.offset(i as isize) = ((ch24 as ::core::ffi::c_uint)
                        << 24 as ::core::ffi::c_int)
                        .wrapping_add((ch16 as ::core::ffi::c_uint) << 16 as ::core::ffi::c_int)
                        .wrapping_add((ch8 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
                        .wrapping_add(ch as ::core::ffi::c_uint);
                    i = i.wrapping_add(1);
                }
            } else {
                *__errno_location() = EINVAL;
                return 0 as ::core::ffi::c_int;
            }
        }
        _ => {
            *__errno_location() = EINVAL;
            return 0 as ::core::ffi::c_int;
        }
    }
    if i < num_samples {
        memset(
            sample_values.offset(i as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            num_samples
                .wrapping_sub(i)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_bytes(
    mut pnm_ptr: *const pnm_struct,
    mut sample_bytes: *mut ::core::ffi::c_uchar,
    mut sample_size: size_t,
    mut num_rows: ::core::ffi::c_uint,
    mut stream: *mut FILE,
) -> ::core::ffi::c_int {
    let mut format: ::core::ffi::c_uint = (*pnm_ptr).format;
    let mut depth: ::core::ffi::c_uint = (*pnm_ptr).depth;
    let mut width: ::core::ffi::c_uint = (*pnm_ptr).width;
    let mut maxval: ::core::ffi::c_uint = (*pnm_ptr).maxval;
    let mut row_length: size_t = (depth as size_t).wrapping_mul(width as size_t);
    let mut num_samples: size_t = (num_rows as size_t).wrapping_mul(row_length);
    let mut raw_sample_size: size_t = 0;
    let mut ch: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if maxval <= 0xff as ::core::ffi::c_uint {
        raw_sample_size = 1 as size_t;
    } else if maxval <= 0xffff as ::core::ffi::c_uint {
        raw_sample_size = 2 as size_t;
    } else if maxval <= 0xffffff as ::core::ffi::c_uint {
        raw_sample_size = 3 as size_t;
    } else if maxval <= 0xffffffff as ::core::ffi::c_uint {
        raw_sample_size = 4 as size_t;
    } else {
        raw_sample_size = (sample_size == 0) as ::core::ffi::c_int as size_t;
    }
    if raw_sample_size != sample_size {
        *__errno_location() = EINVAL;
        return 0 as ::core::ffi::c_int;
    }
    match format {
        4 => {
            j = 0 as size_t;
            i = j;
            while i < num_samples {
                ch = getc(stream);
                if ch == EOF {
                    break;
                }
                mask = 0x80 as ::core::ffi::c_int;
                while mask != 0 as ::core::ffi::c_int {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    *sample_bytes.offset(fresh1 as isize) = (if ch & mask != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                        as ::core::ffi::c_uchar;
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as size_t;
                        break;
                    } else {
                        mask >>= 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
        5 | 6 | 7 => {
            i = fread(
                sample_bytes as *mut ::core::ffi::c_void,
                sample_size,
                num_samples,
                stream,
            ) as size_t;
        }
        _ => {
            *__errno_location() = EINVAL;
            return 0 as ::core::ffi::c_int;
        }
    }
    if i < num_samples {
        memset(
            sample_bytes.offset(i as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            sample_size.wrapping_mul(num_samples).wrapping_sub(i),
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 1 as ::core::ffi::c_int;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
