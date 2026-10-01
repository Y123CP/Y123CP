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

#[inline]
pub unsafe fn pnm_fput_header(
    mut pnm_ptr: *const pnm_struct,
    mut stream: *mut FILE,
) -> c_int {
    let pnm_ptr_view: &pnm_struct = unsafe { &*pnm_ptr };
    let mut format: c_uint = pnm_ptr_view.format;
    let mut depth: c_uint = pnm_ptr_view.depth;
    let mut width: c_uint = pnm_ptr_view.width;
    let mut height: c_uint = pnm_ptr_view.height;
    let mut maxval: c_uint = pnm_ptr_view.maxval;
    let mut result: c_int = 0;
    if pnm_is_valid(pnm_ptr) == 0 {
        return 0 as c_int;
    }
    match format {
        1 | 4 => {
            result = fprintf(
                stream,
                b"P%c\n%u %u\n\0" as *const u8 as *const c_char,
                format.wrapping_add('0' as i32 as c_uint),
                width,
                height,
            );
        }
        2 | 3 | 5 | 6 => {
            result = fprintf(
                stream,
                b"P%c\n%u %u\n%u\n\0" as *const u8 as *const c_char,
                format.wrapping_add('0' as i32 as c_uint),
                width,
                height,
                maxval,
            );
        }
        7 => {
            result = fprintf(
                stream,
                b"P7\nDEPTH %u\nWIDTH %u\nHEIGHT %u\nMAXVAL %u\nENDHDR\n\0" as *const u8
                    as *const c_char,
                depth,
                width,
                height,
                maxval,
            );
        }
        _ => {
            *__errno_location() = EINVAL;
            return 0 as c_int;
        }
    }
    return if result > 0 as c_int {
        1 as c_int
    } else {
        -(1 as c_int)
    };
}
#[inline]
pub unsafe fn pnm_fput_values(
    mut pnm_ptr: *const pnm_struct,
    mut sample_values: *const c_uint,
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
    let mut mask: c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    match format {
        1 => {
            j = 0 as size_t;
            i = j;
            while i < num_samples {
                if putc(
                    (if *sample_values.offset(i as isize) != 0 as c_uint {
                        '0' as i32
                    } else {
                        '1' as i32
                    }),
                    stream,
                ) == EOF
                {
                    break;
                }
                j = j.wrapping_add(1);
                if j == row_length {
                    j = 0 as size_t;
                    if putc('\n' as i32, stream) == EOF {
                        break;
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        2 | 3 => {
            j = 0 as size_t;
            i = j;
            while i < num_samples {
                j = j.wrapping_add(1);
                if j == row_length {
                    j = 0 as size_t;
                }
                if fprintf(
                    stream,
                    (if j == 0 as size_t {
                        b"%u\n\0" as *const u8 as *const c_char
                    } else {
                        b"%u \0" as *const u8 as *const c_char
                    }),
                    *sample_values.offset(i as isize),
                ) <= 0 as c_int
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
                ch = 0 as c_int;
                mask = 0x80 as c_int;
                while mask != 0 as c_int {
                    let fresh0 = i;
                    i = i.wrapping_add(1);
                    if *sample_values.offset(fresh0 as isize) == 0 as c_uint {
                        ch |= mask;
                    }
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as size_t;
                        break;
                    } else {
                        mask >>= 1 as c_int;
                    }
                }
                if putc(ch, stream) == EOF {
                    break;
                }
            }
        }
        5 | 6 | 7 => {
            if maxval <= 0xff as c_uint {
                i = 0 as size_t;
                while i < num_samples {
                    if putc(
                        (*sample_values.offset(i as isize) & 0xff as c_uint)
                            as c_int,
                        stream,
                    ) == EOF
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
            } else if maxval <= 0xffff as c_uint {
                i = 0 as size_t;
                while i < num_samples {
                    if putc(
                        (*sample_values.offset(i as isize) >> 8 as c_int
                            & 0xff as c_uint)
                            as c_int,
                        stream,
                    ) == EOF
                        || putc(
                            (*sample_values.offset(i as isize) & 0xff as c_uint)
                                as c_int,
                            stream,
                        ) == EOF
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
            } else if maxval <= 0xffffffff as c_uint {
                i = 0 as size_t;
                while i < num_samples {
                    if maxval > 0xffffff as c_uint {
                        if putc(
                            (*sample_values.offset(i as isize) >> 24 as c_int
                                & 0xff as c_uint)
                                as c_int,
                            stream,
                        ) == EOF
                        {
                            break;
                        }
                    }
                    if putc(
                        (*sample_values.offset(i as isize) >> 16 as c_int
                            & 0xff as c_uint)
                            as c_int,
                        stream,
                    ) == EOF
                        || putc(
                            (*sample_values.offset(i as isize) >> 8 as c_int
                                & 0xff as c_uint)
                                as c_int,
                            stream,
                        ) == EOF
                        || putc(
                            (*sample_values.offset(i as isize) & 0xff as c_uint)
                                as c_int,
                            stream,
                        ) == EOF
                    {
                        break;
                    }
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
    return if i == num_samples {
        1 as c_int
    } else {
        -(1 as c_int)
    };
}
#[inline]
pub unsafe fn pnm_fput_bytes(
    mut pnm_ptr: *const pnm_struct,
    mut sample_bytes: *const c_uchar,
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
                ch = 0 as c_int;
                mask = 0x80 as c_int;
                while mask != 0 as c_int {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    if *sample_bytes.offset(fresh1 as isize) as c_int
                        == 0 as c_int
                    {
                        ch |= mask;
                    }
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as size_t;
                        break;
                    } else {
                        mask >>= 1 as c_int;
                    }
                }
                if putc(ch, stream) == EOF {
                    break;
                }
            }
        }
        5 | 6 | 7 => {
            i = fwrite(
                sample_bytes as *const c_void,
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
    return if i == num_samples {
        1 as c_int
    } else {
        -(1 as c_int)
    };
}
