use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;

unsafe fn BrotliParseAsUTF8(
    mut symbol: *mut c_int,
    mut input: *const uint8_t,
    mut size: size_t,
) -> size_t {
    if *input.offset(0 as c_int as isize) as c_int
        & 0x80 as c_int
        == 0 as c_int
    {
        *symbol = *input.offset(0 as c_int as isize) as c_int;
        if *symbol > 0 as c_int {
            return 1 as size_t;
        }
    }
    if size > 1 as size_t
        && *input.offset(0 as c_int as isize) as c_int
            & 0xe0 as c_int
            == 0xc0 as c_int
        && *input.offset(1 as c_int as isize) as c_int
            & 0xc0 as c_int
            == 0x80 as c_int
    {
        *symbol = (*input.offset(0 as c_int as isize) as c_int
            & 0x1f as c_int)
            << 6 as c_int
            | *input.offset(1 as c_int as isize) as c_int
                & 0x3f as c_int;
        if *symbol > 0x7f as c_int {
            return 2 as size_t;
        }
    }
    if size > 2 as size_t
        && *input.offset(0 as c_int as isize) as c_int
            & 0xf0 as c_int
            == 0xe0 as c_int
        && *input.offset(1 as c_int as isize) as c_int
            & 0xc0 as c_int
            == 0x80 as c_int
        && *input.offset(2 as c_int as isize) as c_int
            & 0xc0 as c_int
            == 0x80 as c_int
    {
        *symbol = (*input.offset(0 as c_int as isize) as c_int
            & 0xf as c_int)
            << 12 as c_int
            | (*input.offset(1 as c_int as isize) as c_int
                & 0x3f as c_int)
                << 6 as c_int
            | *input.offset(2 as c_int as isize) as c_int
                & 0x3f as c_int;
        if *symbol > 0x7ff as c_int {
            return 3 as size_t;
        }
    }
    if size > 3 as size_t
        && *input.offset(0 as c_int as isize) as c_int
            & 0xf8 as c_int
            == 0xf0 as c_int
        && *input.offset(1 as c_int as isize) as c_int
            & 0xc0 as c_int
            == 0x80 as c_int
        && *input.offset(2 as c_int as isize) as c_int
            & 0xc0 as c_int
            == 0x80 as c_int
        && *input.offset(3 as c_int as isize) as c_int
            & 0xc0 as c_int
            == 0x80 as c_int
    {
        *symbol = (*input.offset(0 as c_int as isize) as c_int
            & 0x7 as c_int)
            << 18 as c_int
            | (*input.offset(1 as c_int as isize) as c_int
                & 0x3f as c_int)
                << 12 as c_int
            | (*input.offset(2 as c_int as isize) as c_int
                & 0x3f as c_int)
                << 6 as c_int
            | *input.offset(3 as c_int as isize) as c_int
                & 0x3f as c_int;
        if *symbol > 0xffff as c_int && *symbol <= 0x10ffff as c_int {
            return 4 as size_t;
        }
    }
    *symbol = 0x110000 as c_int
        | *input.offset(0 as c_int as isize) as c_int;
    return 1 as size_t;
}
#[inline]
pub unsafe fn BrotliIsMostlyUTF8(
    mut data: *const uint8_t,
    pos: size_t,
    mask: size_t,
    length: size_t,
    min_fraction: c_double,
) -> c_int {
    let mut size_utf8: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;

    while i < length {
        let remaining = length.wrapping_sub(i);
        let p = {
            let idx = pos.wrapping_add(i) & mask;
            // SAFETY: Preserves the original function's pointer arithmetic exactly:
            // caller guarantees `data` points to the ring buffer and `idx` is the
            // intended wrapped offset within that buffer.
            data.offset(idx as isize) as *const uint8_t
        };

        let mut symbol: c_int = 0;
        let bytes_read = BrotliParseAsUTF8(&raw mut symbol, p, remaining);
        i = i.wrapping_add(bytes_read);

        if symbol < 0x110000 as c_int {
            size_utf8 = size_utf8.wrapping_add(bytes_read);
        }
    }

    if (size_utf8 as c_double) > min_fraction * (length as c_double) {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    }
}
