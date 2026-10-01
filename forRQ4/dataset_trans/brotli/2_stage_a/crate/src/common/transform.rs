use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

static mut kPrefixSuffix: [c_char; 217] = unsafe {
    ::core::mem::transmute::<
        [u8; 217],
        [c_char; 217],
    >(
        *b"\x01 \x02, \x08 of the \x04 of \x02s \x01.\x05 and \x04 in \x01\"\x04 to \x02\">\x01\n\x02. \x01]\x05 for \x03 a \x06 that \x01'\x06 with \x06 from \x04 by \x01(\x06. The \x04 on \x04 as \x04 is \x04ing \x02\n\t\x01:\x03ed \x02=\"\x04 at \x03ly \x01,\x02='\x05.com/\x07. This \x05 not \x03er \x03al \x04ful \x04ive \x05less \x04est \x04ize \x02\xC2\xA0\x04ous \x05 the \x02e \0",
    )
};
static mut kPrefixSuffixMap: [uint16_t; 50] = [
    0 as c_int as uint16_t,
    0x2 as c_int as uint16_t,
    0x5 as c_int as uint16_t,
    0xe as c_int as uint16_t,
    0x13 as c_int as uint16_t,
    0x16 as c_int as uint16_t,
    0x18 as c_int as uint16_t,
    0x1e as c_int as uint16_t,
    0x23 as c_int as uint16_t,
    0x25 as c_int as uint16_t,
    0x2a as c_int as uint16_t,
    0x2d as c_int as uint16_t,
    0x2f as c_int as uint16_t,
    0x32 as c_int as uint16_t,
    0x34 as c_int as uint16_t,
    0x3a as c_int as uint16_t,
    0x3e as c_int as uint16_t,
    0x45 as c_int as uint16_t,
    0x47 as c_int as uint16_t,
    0x4e as c_int as uint16_t,
    0x55 as c_int as uint16_t,
    0x5a as c_int as uint16_t,
    0x5c as c_int as uint16_t,
    0x63 as c_int as uint16_t,
    0x68 as c_int as uint16_t,
    0x6d as c_int as uint16_t,
    0x72 as c_int as uint16_t,
    0x77 as c_int as uint16_t,
    0x7a as c_int as uint16_t,
    0x7c as c_int as uint16_t,
    0x80 as c_int as uint16_t,
    0x83 as c_int as uint16_t,
    0x88 as c_int as uint16_t,
    0x8c as c_int as uint16_t,
    0x8e as c_int as uint16_t,
    0x91 as c_int as uint16_t,
    0x97 as c_int as uint16_t,
    0x9f as c_int as uint16_t,
    0xa5 as c_int as uint16_t,
    0xa9 as c_int as uint16_t,
    0xad as c_int as uint16_t,
    0xb2 as c_int as uint16_t,
    0xb7 as c_int as uint16_t,
    0xbd as c_int as uint16_t,
    0xc2 as c_int as uint16_t,
    0xc7 as c_int as uint16_t,
    0xca as c_int as uint16_t,
    0xcf as c_int as uint16_t,
    0xd5 as c_int as uint16_t,
    0xd8 as c_int as uint16_t,
];
static mut kTransformsData: [uint8_t; 363] = [
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    0 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_1 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    47 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    49 as c_int as uint8_t,
    4 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    3 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    6 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_2 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_1 as c_int as uint8_t,
    49 as c_int as uint8_t,
    1 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    1 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    7 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    9 as c_int as uint8_t,
    48 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    8 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    5 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    10 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    11 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_3 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    13 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    14 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_3 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_2 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    15 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    16 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    12 as c_int as uint8_t,
    5 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    49 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    1 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_4 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    18 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    17 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    19 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    20 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_5 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_6 as c_int as uint8_t,
    49 as c_int as uint8_t,
    47 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_4 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    22 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    23 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    24 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    25 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_7 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_1 as c_int as uint8_t,
    26 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    27 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    28 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    12 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    29 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_9 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_FIRST_7 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_6 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    21 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    1 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_8 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    31 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    32 as c_int as uint8_t,
    47 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    3 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_5 as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_OMIT_LAST_9 as c_int as uint8_t,
    49 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    1 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    8 as c_int as uint8_t,
    5 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    21 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    10 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    30 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    5 as c_int as uint8_t,
    35 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    49 as c_int as uint8_t,
    47 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    2 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    17 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    36 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    33 as c_int as uint8_t,
    5 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    21 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    5 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    37 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    30 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    38 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    0 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    39 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    49 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    34 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    8 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    12 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    21 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    40 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    12 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    41 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    42 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    17 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    43 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    5 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    10 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    34 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    33 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    44 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    5 as c_int as uint8_t,
    45 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    49 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    33 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    30 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    30 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_IDENTITY as c_int as uint8_t,
    46 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    1 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    34 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    33 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    30 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    1 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    33 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    21 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    12 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    5 as c_int as uint8_t,
    49 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    34 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    12 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    30 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_ALL as c_int as uint8_t,
    34 as c_int as uint8_t,
    0 as c_int as uint8_t,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int as uint8_t,
    34 as c_int as uint8_t,
];
static mut kBrotliTransforms: BrotliTransforms = BrotliTransforms {
    prefix_suffix_size: 0,
    prefix_suffix: ::core::ptr::null::<uint8_t>(),
    prefix_suffix_map: ::core::ptr::null::<uint16_t>(),
    num_transforms: 0,
    transforms: ::core::ptr::null::<uint8_t>(),
    params: ::core::ptr::null::<uint8_t>(),
    cutOffTransforms: [0; 10],
};
#[inline]
pub fn BrotliGetTransforms() -> *const BrotliTransforms { unsafe {
    return &raw const kBrotliTransforms;
} }
unsafe fn ToUpperCase(mut p: *mut uint8_t) -> c_int {
    if (*p.offset(0 as c_int as isize) as c_int)
        < 0xc0 as c_int
    {
        if *p.offset(0 as c_int as isize) as c_int >= 'a' as i32
            && *p.offset(0 as c_int as isize) as c_int <= 'z' as i32
        {
            let ref mut fresh10 = *p.offset(0 as c_int as isize);
            *fresh10 = (*fresh10 as c_int ^ 32 as c_int) as uint8_t;
        }
        return 1 as c_int;
    }
    if (*p.offset(0 as c_int as isize) as c_int)
        < 0xe0 as c_int
    {
        let ref mut fresh11 = *p.offset(1 as c_int as isize);
        *fresh11 = (*fresh11 as c_int ^ 32 as c_int) as uint8_t;
        return 2 as c_int;
    }
    let ref mut fresh12 = *p.offset(2 as c_int as isize);
    *fresh12 = (*fresh12 as c_int ^ 5 as c_int) as uint8_t;
    return 3 as c_int;
}
unsafe fn Shift(
    mut word: *mut uint8_t,
    mut word_len: c_int,
    mut parameter: uint16_t,
) -> c_int {
    let mut scalar: uint32_t = (parameter as uint32_t & 0x7fff as uint32_t).wrapping_add(
        (0x1000000 as uint32_t).wrapping_sub(parameter as uint32_t & 0x8000 as uint32_t),
    );
    if (*word.offset(0 as c_int as isize) as c_int)
        < 0x80 as c_int
    {
        scalar =
            (scalar as c_uint)
                .wrapping_add(*word.offset(0 as c_int as isize) as uint32_t
                    as c_uint) as uint32_t as uint32_t;
        *word.offset(0 as c_int as isize) = (scalar & 0x7f as uint32_t) as uint8_t;
        return 1 as c_int;
    } else if (*word.offset(0 as c_int as isize) as c_int)
        < 0xc0 as c_int
    {
        return 1 as c_int;
    } else if (*word.offset(0 as c_int as isize) as c_int)
        < 0xe0 as c_int
    {
        if word_len < 2 as c_int {
            return 1 as c_int;
        }
        scalar = (scalar as c_uint).wrapping_add(
            (*word.offset(1 as c_int as isize) as c_uint
                & 0x3f as c_uint
                | (*word.offset(0 as c_int as isize) as c_uint
                    & 0x1f as c_uint)
                    << 6 as c_uint) as uint32_t as c_uint,
        ) as uint32_t as uint32_t;
        *word.offset(0 as c_int as isize) =
            (0xc0 as uint32_t | scalar >> 6 as c_uint & 0x1f as uint32_t) as uint8_t;
        *word.offset(1 as c_int as isize) =
            ((*word.offset(1 as c_int as isize) as c_int
                & 0xc0 as c_int) as uint32_t
                | scalar & 0x3f as uint32_t) as uint8_t;
        return 2 as c_int;
    } else if (*word.offset(0 as c_int as isize) as c_int)
        < 0xf0 as c_int
    {
        if word_len < 3 as c_int {
            return word_len;
        }
        scalar = (scalar as c_uint).wrapping_add(
            (*word.offset(2 as c_int as isize) as c_uint
                & 0x3f as c_uint
                | (*word.offset(1 as c_int as isize) as c_uint
                    & 0x3f as c_uint)
                    << 6 as c_uint
                | (*word.offset(0 as c_int as isize) as c_uint
                    & 0xf as c_uint)
                    << 12 as c_uint) as uint32_t as c_uint,
        ) as uint32_t as uint32_t;
        *word.offset(0 as c_int as isize) =
            (0xe0 as uint32_t | scalar >> 12 as c_uint & 0xf as uint32_t) as uint8_t;
        *word.offset(1 as c_int as isize) =
            ((*word.offset(1 as c_int as isize) as c_int
                & 0xc0 as c_int) as uint32_t
                | scalar >> 6 as c_uint & 0x3f as uint32_t) as uint8_t;
        *word.offset(2 as c_int as isize) =
            ((*word.offset(2 as c_int as isize) as c_int
                & 0xc0 as c_int) as uint32_t
                | scalar & 0x3f as uint32_t) as uint8_t;
        return 3 as c_int;
    } else if (*word.offset(0 as c_int as isize) as c_int)
        < 0xf8 as c_int
    {
        if word_len < 4 as c_int {
            return word_len;
        }
        scalar = (scalar as c_uint).wrapping_add(
            (*word.offset(3 as c_int as isize) as c_uint
                & 0x3f as c_uint
                | (*word.offset(2 as c_int as isize) as c_uint
                    & 0x3f as c_uint)
                    << 6 as c_uint
                | (*word.offset(1 as c_int as isize) as c_uint
                    & 0x3f as c_uint)
                    << 12 as c_uint
                | (*word.offset(0 as c_int as isize) as c_uint
                    & 0x7 as c_uint)
                    << 18 as c_uint) as uint32_t as c_uint,
        ) as uint32_t as uint32_t;
        *word.offset(0 as c_int as isize) =
            (0xf0 as uint32_t | scalar >> 18 as c_uint & 0x7 as uint32_t) as uint8_t;
        *word.offset(1 as c_int as isize) =
            ((*word.offset(1 as c_int as isize) as c_int
                & 0xc0 as c_int) as uint32_t
                | scalar >> 12 as c_uint & 0x3f as uint32_t) as uint8_t;
        *word.offset(2 as c_int as isize) =
            ((*word.offset(2 as c_int as isize) as c_int
                & 0xc0 as c_int) as uint32_t
                | scalar >> 6 as c_uint & 0x3f as uint32_t) as uint8_t;
        *word.offset(3 as c_int as isize) =
            ((*word.offset(3 as c_int as isize) as c_int
                & 0xc0 as c_int) as uint32_t
                | scalar & 0x3f as uint32_t) as uint8_t;
        return 4 as c_int;
    }
    return 1 as c_int;
}
#[inline]
pub unsafe fn BrotliTransformDictionaryWord(
    mut dst: *mut uint8_t,
    mut word: *const uint8_t,
    mut len: c_int,
    mut transforms: *const BrotliTransforms,
    mut transform_idx: c_int,
) -> c_int {
    let mut idx: c_int = 0 as c_int;
    let mut prefix: *const uint8_t =
        (*transforms)
            .prefix_suffix
            .offset(
                *(*transforms)
                    .prefix_suffix_map
                    .offset(*(*transforms).transforms.offset(
                        (transform_idx * 3 as c_int + 0 as c_int)
                            as isize,
                    ) as isize) as isize,
            ) as *const uint8_t;
    let mut type_0: uint8_t = *(*transforms)
        .transforms
        .offset((transform_idx * 3 as c_int + 1 as c_int) as isize);
    let mut suffix: *const uint8_t =
        (*transforms)
            .prefix_suffix
            .offset(
                *(*transforms)
                    .prefix_suffix_map
                    .offset(*(*transforms).transforms.offset(
                        (transform_idx * 3 as c_int + 2 as c_int)
                            as isize,
                    ) as isize) as isize,
            ) as *const uint8_t;
    let fresh0 = prefix;
    prefix = prefix.offset(1);
    let mut prefix_len: c_int = *fresh0 as c_int;
    loop {
        let fresh1 = prefix_len;
        prefix_len = prefix_len - 1;
        if !(fresh1 != 0) {
            break;
        }
        let fresh2 = prefix;
        prefix = prefix.offset(1);
        let fresh3 = idx;
        idx = idx + 1;
        *dst.offset(fresh3 as isize) = *fresh2;
    }
    let t: c_int = type_0 as c_int;
    let mut i: c_int = 0 as c_int;
    if t <= BROTLI_TRANSFORM_OMIT_LAST_9 as c_int {
        len -= t;
    } else if t >= BROTLI_TRANSFORM_OMIT_FIRST_1 as c_int
        && t <= BROTLI_TRANSFORM_OMIT_FIRST_9 as c_int
    {
        let mut skip: c_int =
            t - (BROTLI_TRANSFORM_OMIT_FIRST_1 as c_int - 1 as c_int);
        word = word.offset(skip as isize);
        len -= skip;
    }
    while i < len {
        let fresh4 = i;
        i = i + 1;
        let fresh5 = idx;
        idx = idx + 1;
        *dst.offset(fresh5 as isize) = *word.offset(fresh4 as isize);
    }
    if t == BROTLI_TRANSFORM_UPPERCASE_FIRST as c_int {
        ToUpperCase(dst.offset((idx - len) as isize) as *mut uint8_t);
    } else if t == BROTLI_TRANSFORM_UPPERCASE_ALL as c_int {
        let mut uppercase: *mut uint8_t = dst.offset((idx - len) as isize) as *mut uint8_t;
        while len > 0 as c_int {
            let mut step: c_int = ToUpperCase(uppercase);
            uppercase = uppercase.offset(step as isize);
            len -= step;
        }
    } else if t == BROTLI_TRANSFORM_SHIFT_FIRST as c_int {
        let mut param: uint16_t = (*(*transforms)
            .params
            .offset((transform_idx * 2 as c_int) as isize)
            as c_int
            + ((*(*transforms).params.offset(
                (transform_idx * 2 as c_int + 1 as c_int) as isize,
            ) as c_int)
                << 8 as c_uint)) as uint16_t;
        Shift(dst.offset((idx - len) as isize) as *mut uint8_t, len, param);
    } else if t == BROTLI_TRANSFORM_SHIFT_ALL as c_int {
        let mut param_0: uint16_t = (*(*transforms)
            .params
            .offset((transform_idx * 2 as c_int) as isize)
            as c_int
            + ((*(*transforms).params.offset(
                (transform_idx * 2 as c_int + 1 as c_int) as isize,
            ) as c_int)
                << 8 as c_uint)) as uint16_t;
        let mut shift: *mut uint8_t = dst.offset((idx - len) as isize) as *mut uint8_t;
        while len > 0 as c_int {
            let mut step_0: c_int = Shift(shift, len, param_0);
            shift = shift.offset(step_0 as isize);
            len -= step_0;
        }
    }
    let fresh6 = suffix;
    suffix = suffix.offset(1);
    let mut suffix_len: c_int = *fresh6 as c_int;
    loop {
        let fresh7 = suffix_len;
        suffix_len = suffix_len - 1;
        if !(fresh7 != 0) {
            break;
        }
        let fresh8 = suffix;
        suffix = suffix.offset(1);
        let fresh9 = idx;
        idx = idx + 1;
        *dst.offset(fresh9 as isize) = *fresh8;
    }
    return idx;
}
extern "C" fn run_static_initializers() { unsafe {
    kBrotliTransforms = BrotliTransforms {
        prefix_suffix_size: ::core::mem::size_of::<[c_char; 217]>() as uint16_t,
        prefix_suffix: &raw const kPrefixSuffix as *const c_char as *const uint8_t,
        prefix_suffix_map: &raw const kPrefixSuffixMap as *const uint16_t,
        num_transforms: (::core::mem::size_of::<[uint8_t; 363]>() as usize)
            .wrapping_div((3 as usize).wrapping_mul(::core::mem::size_of::<uint8_t>() as usize))
            as uint32_t,
        transforms: &raw const kTransformsData as *const uint8_t,
        params: ::core::ptr::null::<uint8_t>(),
        cutOffTransforms: [
            0 as c_int as int16_t,
            12 as c_int as int16_t,
            27 as c_int as int16_t,
            23 as c_int as int16_t,
            42 as c_int as int16_t,
            63 as c_int as int16_t,
            56 as c_int as int16_t,
            48 as c_int as int16_t,
            59 as c_int as int16_t,
            64 as c_int as int16_t,
        ],
    };
} }
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
