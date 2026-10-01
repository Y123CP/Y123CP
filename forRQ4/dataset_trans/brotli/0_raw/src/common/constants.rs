pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliPrefixCodeRange {
    pub offset: uint16_t,
    pub nbits: uint8_t,
}
#[no_mangle]
pub static mut _kBrotliPrefixCodeRanges: [BrotliPrefixCodeRange; 26] = [
    BrotliPrefixCodeRange {
        offset: 1 as uint16_t,
        nbits: 2 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 5 as uint16_t,
        nbits: 2 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 9 as uint16_t,
        nbits: 2 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 13 as uint16_t,
        nbits: 2 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 17 as uint16_t,
        nbits: 3 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 25 as uint16_t,
        nbits: 3 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 33 as uint16_t,
        nbits: 3 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 41 as uint16_t,
        nbits: 3 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 49 as uint16_t,
        nbits: 4 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 65 as uint16_t,
        nbits: 4 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 81 as uint16_t,
        nbits: 4 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 97 as uint16_t,
        nbits: 4 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 113 as uint16_t,
        nbits: 5 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 145 as uint16_t,
        nbits: 5 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 177 as uint16_t,
        nbits: 5 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 209 as uint16_t,
        nbits: 5 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 241 as uint16_t,
        nbits: 6 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 305 as uint16_t,
        nbits: 6 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 369 as uint16_t,
        nbits: 7 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 497 as uint16_t,
        nbits: 8 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 753 as uint16_t,
        nbits: 9 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 1265 as uint16_t,
        nbits: 10 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 2289 as uint16_t,
        nbits: 11 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 4337 as uint16_t,
        nbits: 12 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 8433 as uint16_t,
        nbits: 13 as uint8_t,
    },
    BrotliPrefixCodeRange {
        offset: 16625 as uint16_t,
        nbits: 24 as uint8_t,
    },
];
