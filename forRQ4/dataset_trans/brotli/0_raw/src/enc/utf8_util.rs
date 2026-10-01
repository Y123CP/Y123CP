pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn BrotliParseAsUTF8(
    mut symbol: *mut ::core::ffi::c_int,
    mut input: *const uint8_t,
    mut size: size_t,
) -> size_t {
    if *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        *symbol = *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if *symbol > 0 as ::core::ffi::c_int {
            return 1 as size_t;
        }
    }
    if size > 1 as size_t
        && *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xe0 as ::core::ffi::c_int
            == 0xc0 as ::core::ffi::c_int
        && *input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
    {
        *symbol = (*input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x1f as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int
            | *input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int;
        if *symbol > 0x7f as ::core::ffi::c_int {
            return 2 as size_t;
        }
    }
    if size > 2 as size_t
        && *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf0 as ::core::ffi::c_int
            == 0xe0 as ::core::ffi::c_int
        && *input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
        && *input.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
    {
        *symbol = (*input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int)
            << 12 as ::core::ffi::c_int
            | (*input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int
            | *input.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int;
        if *symbol > 0x7ff as ::core::ffi::c_int {
            return 3 as size_t;
        }
    }
    if size > 3 as size_t
        && *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf8 as ::core::ffi::c_int
            == 0xf0 as ::core::ffi::c_int
        && *input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
        && *input.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
        && *input.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int
            == 0x80 as ::core::ffi::c_int
    {
        *symbol = (*input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x7 as ::core::ffi::c_int)
            << 18 as ::core::ffi::c_int
            | (*input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int)
                << 12 as ::core::ffi::c_int
            | (*input.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int
            | *input.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int;
        if *symbol > 0xffff as ::core::ffi::c_int && *symbol <= 0x10ffff as ::core::ffi::c_int {
            return 4 as size_t;
        }
    }
    *symbol = 0x110000 as ::core::ffi::c_int
        | *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    return 1 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliIsMostlyUTF8(
    mut data: *const uint8_t,
    pos: size_t,
    mask: size_t,
    length: size_t,
    min_fraction: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut size_utf8: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < length {
        let mut symbol: ::core::ffi::c_int = 0;
        let mut bytes_read: size_t = BrotliParseAsUTF8(
            &raw mut symbol,
            data.offset((pos.wrapping_add(i) & mask) as isize) as *const uint8_t,
            length.wrapping_sub(i),
        );
        i = (i as ::core::ffi::c_ulong).wrapping_add(bytes_read as ::core::ffi::c_ulong) as size_t
            as size_t;
        if symbol < 0x110000 as ::core::ffi::c_int {
            size_utf8 = (size_utf8 as ::core::ffi::c_ulong)
                .wrapping_add(bytes_read as ::core::ffi::c_ulong) as size_t
                as size_t;
        }
    }
    return if size_utf8 as ::core::ffi::c_double > min_fraction * length as ::core::ffi::c_double {
        BROTLI_TRUE
    } else {
        BROTLI_FALSE
    };
}
