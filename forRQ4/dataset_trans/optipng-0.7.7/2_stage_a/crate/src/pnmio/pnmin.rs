use core::ffi::*;
use crate::src::pnmio::pnmutil::pnm_is_valid;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;

pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const PNM_P7: C2RustUnnamed_htdd24ee73 = 7;
pub const PNM_P6: C2RustUnnamed_htdd24ee73 = 6;
pub const PNM_P5: C2RustUnnamed_htdd24ee73 = 5;
pub const PNM_P4: C2RustUnnamed_htdd24ee73 = 4;
pub const PNM_P3: C2RustUnnamed_htdd24ee73 = 3;
pub const PNM_P2: C2RustUnnamed_htdd24ee73 = 2;
pub const PNM_P1: C2RustUnnamed_htdd24ee73 = 1;

pub const UINT_MAX: c_uint = (__INT_MAX__ as c_uint)
    .wrapping_mul(2 as c_uint)
    .wrapping_add(1 as c_uint);

unsafe fn pnm_fget_char(mut stream: *mut FILE) -> c_int {
    let mut ch: c_int = getc(stream);
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
unsafe fn pnm_fscan_uint(
    mut stream: *mut FILE,
    mut value: *mut c_uint,
) -> c_int {
    let value_view: &mut c_uint = unsafe { &mut *value };
    let mut ch: c_int = 0;
    let mut tmp: c_uint = 0;
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
        return 0 as c_int;
    }
    *value_view = 0 as c_uint;
    loop {
        tmp = value_view
            .wrapping_mul(10 as c_uint)
            .wrapping_add((ch - '0' as i32) as c_uint);
        if tmp >= *value_view {
            *value_view = tmp;
        } else {
            *value_view = UINT_MAX;
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
    return 1 as c_int;
}
#[inline]
pub unsafe fn pnm_fget_header(
    mut pnm_ptr: *mut pnm_struct,
    mut stream: *mut FILE,
) -> c_int {
    let mut format: c_uint = 0;
    let mut ch: c_int = 0;
    memset(
        pnm_ptr as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<pnm_struct>() as size_t,
    );
    ch = getc(stream);
    if ch == EOF {
        return -(1 as c_int);
    }
    if ch != 'P' as i32 {
        return -(1 as c_int);
    }
    ch = getc(stream);
    if ch < '1' as i32 || ch > '9' as i32 {
        return -(1 as c_int);
    }
    format = (ch - '0' as i32) as c_uint;
    ch = pnm_fget_char(stream);
    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32) {
        return -(1 as c_int);
    }
    (*pnm_ptr).format = format;
    if format >= PNM_P1 as c_int as c_uint
        && format <= PNM_P6 as c_int as c_uint
    {
        (*pnm_ptr).depth = (if format == PNM_P3 as c_int as c_uint
            || format == PNM_P6 as c_int as c_uint
        {
            3 as c_int
        } else {
            1 as c_int
        }) as c_uint;
        if pnm_fscan_uint(stream, &raw mut (*pnm_ptr).width) != 1 as c_int
            || pnm_fscan_uint(stream, &raw mut (*pnm_ptr).height) != 1 as c_int
        {
            return -(1 as c_int);
        }
        if format == PNM_P1 as c_int as c_uint
            || format == PNM_P4 as c_int as c_uint
        {
            (*pnm_ptr).maxval = 1 as c_uint;
        } else if pnm_fscan_uint(stream, &raw mut (*pnm_ptr).maxval) != 1 as c_int {
            return -(1 as c_int);
        }
        return if pnm_is_valid(pnm_ptr) != 0 {
            1 as c_int
        } else {
            0 as c_int
        };
    } else {
        return -(1 as c_int);
    };
}
#[inline]
pub unsafe fn pnm_fget_values(
    mut pnm_ptr: *const pnm_struct,
    mut sample_values: *mut c_uint,
    mut num_rows: c_uint,
    mut stream: *mut FILE,
) -> c_int {
    let pnm_ptr_view: &pnm_struct = unsafe { &*pnm_ptr };
    let mut format: c_uint = pnm_ptr_view.format;
    let mut depth: c_uint = pnm_ptr_view.depth;
    let mut width: c_uint = pnm_ptr_view.width;
    let mut maxval: c_uint = pnm_ptr_view.maxval;
    let mut row_length: size_t = (depth as size_t).wrapping_mul(width as size_t);
    let mut num_samples: size_t = (num_rows as size_t).wrapping_mul(row_length);
    let mut ch: c_int = 0;
    let mut ch8: c_int = 0;
    let mut ch16: c_int = 0;
    let mut ch24: c_int = 0;
    let mut mask: c_int = 0;
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
                        1 as c_int
                    } else {
                        0 as c_int
                    })
                        as c_uint;
                    i = i.wrapping_add(1);
                }
            }
        }
        2 | 3 => {
            i = 0 as size_t;
            while i < num_samples {
                if pnm_fscan_uint(
                    stream,
                    sample_values.offset(i as isize) as *mut c_uint,
                ) != 1 as c_int
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
                mask = 0x80 as c_int;
                while mask != 0 as c_int {
                    let fresh0 = i;
                    i = i.wrapping_add(1);
                    *sample_values.offset(fresh0 as isize) = (if ch & mask != 0 {
                        0 as c_int
                    } else {
                        1 as c_int
                    })
                        as c_uint;
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as size_t;
                        break;
                    } else {
                        mask >>= 1 as c_int;
                    }
                }
            }
        }
        5 | 6 | 7 => {
            if maxval <= 0xff as c_uint {
                i = 0 as size_t;
                while i < num_samples {
                    ch = getc(stream);
                    if ch == EOF {
                        break;
                    }
                    *sample_values.offset(i as isize) = ch as c_uint;
                    i = i.wrapping_add(1);
                }
            } else if maxval <= 0xffff as c_uint {
                i = 0 as size_t;
                while i < num_samples {
                    ch8 = getc(stream);
                    ch = getc(stream);
                    if ch == EOF {
                        break;
                    }
                    *sample_values.offset(i as isize) = ((ch8 as c_uint)
                        << 8 as c_int)
                        .wrapping_add(ch as c_uint);
                    i = i.wrapping_add(1);
                }
            } else if maxval <= 0xffffffff as c_uint {
                ch24 = 0 as c_int;
                i = 0 as size_t;
                while i < num_samples {
                    if maxval > 0xffffff as c_uint {
                        ch24 = getc(stream);
                    }
                    ch16 = getc(stream);
                    ch8 = getc(stream);
                    ch = getc(stream);
                    if ch == EOF {
                        break;
                    }
                    *sample_values.offset(i as isize) = ((ch24 as c_uint)
                        << 24 as c_int)
                        .wrapping_add((ch16 as c_uint) << 16 as c_int)
                        .wrapping_add((ch8 as c_uint) << 8 as c_int)
                        .wrapping_add(ch as c_uint);
                    i = i.wrapping_add(1);
                }
            } else {
                *__errno_location() = EINVAL;
                return 0 as c_int;
            }
        }
        _ => {
            *__errno_location() = EINVAL;
            return 0 as c_int;
        }
    }
    if i < num_samples {
        memset(
            sample_values.offset(i as isize) as *mut c_void,
            0 as c_int,
            num_samples
                .wrapping_sub(i)
                .wrapping_mul(::core::mem::size_of::<c_uint>() as size_t),
        );
        return -(1 as c_int);
    }
    return 1 as c_int;
}
#[inline]
pub unsafe fn pnm_fget_bytes(
    mut pnm_ptr: *const pnm_struct,
    mut sample_bytes: *mut c_uchar,
    mut sample_size: size_t,
    mut num_rows: c_uint,
    mut stream: *mut FILE,
) -> c_int {
    let pnm_ptr_view: &pnm_struct = unsafe { &*pnm_ptr };
    let mut format: c_uint = pnm_ptr_view.format;
    let mut depth: c_uint = pnm_ptr_view.depth;
    let mut width: c_uint = pnm_ptr_view.width;
    let mut maxval: c_uint = pnm_ptr_view.maxval;
    let mut row_length: size_t = (depth as size_t).wrapping_mul(width as size_t);
    let mut num_samples: size_t = (num_rows as size_t).wrapping_mul(row_length);
    let mut raw_sample_size: size_t = 0;
    let mut ch: c_int = 0;
    let mut mask: c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if maxval <= 0xff as c_uint {
        raw_sample_size = 1 as size_t;
    } else if maxval <= 0xffff as c_uint {
        raw_sample_size = 2 as size_t;
    } else if maxval <= 0xffffff as c_uint {
        raw_sample_size = 3 as size_t;
    } else if maxval <= 0xffffffff as c_uint {
        raw_sample_size = 4 as size_t;
    } else {
        raw_sample_size = (sample_size == 0) as c_int as size_t;
    }
    if raw_sample_size != sample_size {
        *__errno_location() = EINVAL;
        return 0 as c_int;
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
                mask = 0x80 as c_int;
                while mask != 0 as c_int {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    *sample_bytes.offset(fresh1 as isize) = (if ch & mask != 0 {
                        0 as c_int
                    } else {
                        1 as c_int
                    })
                        as c_uchar;
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as size_t;
                        break;
                    } else {
                        mask >>= 1 as c_int;
                    }
                }
            }
        }
        5 | 6 | 7 => {
            i = fread(
                sample_bytes as *mut c_void,
                sample_size,
                num_samples,
                stream,
            ) as size_t;
        }
        _ => {
            *__errno_location() = EINVAL;
            return 0 as c_int;
        }
    }
    if i < num_samples {
        memset(
            sample_bytes.offset(i as isize) as *mut c_void,
            0 as c_int,
            sample_size.wrapping_mul(num_samples).wrapping_sub(i),
        );
        return -(1 as c_int);
    }
    return 1 as c_int;
}

