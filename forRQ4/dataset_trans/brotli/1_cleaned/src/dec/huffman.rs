use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;

#[inline(always)]
unsafe extern "C" fn ConstructHuffmanCode(bits: uint8_t, value: uint16_t) -> HuffmanCode {
    let mut h: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
    h.bits = bits;
    h.value = value;
    return h;
}
pub const BROTLI_REVERSE_BITS_MAX: c_int = 8 as c_int;
pub const BROTLI_REVERSE_BITS_BASE: c_int = 0 as c_int;
static mut kReverseBits: [uint8_t; 256] = [
    0 as c_int as uint8_t,
    0x80 as c_int as uint8_t,
    0x40 as c_int as uint8_t,
    0xc0 as c_int as uint8_t,
    0x20 as c_int as uint8_t,
    0xa0 as c_int as uint8_t,
    0x60 as c_int as uint8_t,
    0xe0 as c_int as uint8_t,
    0x10 as c_int as uint8_t,
    0x90 as c_int as uint8_t,
    0x50 as c_int as uint8_t,
    0xd0 as c_int as uint8_t,
    0x30 as c_int as uint8_t,
    0xb0 as c_int as uint8_t,
    0x70 as c_int as uint8_t,
    0xf0 as c_int as uint8_t,
    0x8 as c_int as uint8_t,
    0x88 as c_int as uint8_t,
    0x48 as c_int as uint8_t,
    0xc8 as c_int as uint8_t,
    0x28 as c_int as uint8_t,
    0xa8 as c_int as uint8_t,
    0x68 as c_int as uint8_t,
    0xe8 as c_int as uint8_t,
    0x18 as c_int as uint8_t,
    0x98 as c_int as uint8_t,
    0x58 as c_int as uint8_t,
    0xd8 as c_int as uint8_t,
    0x38 as c_int as uint8_t,
    0xb8 as c_int as uint8_t,
    0x78 as c_int as uint8_t,
    0xf8 as c_int as uint8_t,
    0x4 as c_int as uint8_t,
    0x84 as c_int as uint8_t,
    0x44 as c_int as uint8_t,
    0xc4 as c_int as uint8_t,
    0x24 as c_int as uint8_t,
    0xa4 as c_int as uint8_t,
    0x64 as c_int as uint8_t,
    0xe4 as c_int as uint8_t,
    0x14 as c_int as uint8_t,
    0x94 as c_int as uint8_t,
    0x54 as c_int as uint8_t,
    0xd4 as c_int as uint8_t,
    0x34 as c_int as uint8_t,
    0xb4 as c_int as uint8_t,
    0x74 as c_int as uint8_t,
    0xf4 as c_int as uint8_t,
    0xc as c_int as uint8_t,
    0x8c as c_int as uint8_t,
    0x4c as c_int as uint8_t,
    0xcc as c_int as uint8_t,
    0x2c as c_int as uint8_t,
    0xac as c_int as uint8_t,
    0x6c as c_int as uint8_t,
    0xec as c_int as uint8_t,
    0x1c as c_int as uint8_t,
    0x9c as c_int as uint8_t,
    0x5c as c_int as uint8_t,
    0xdc as c_int as uint8_t,
    0x3c as c_int as uint8_t,
    0xbc as c_int as uint8_t,
    0x7c as c_int as uint8_t,
    0xfc as c_int as uint8_t,
    0x2 as c_int as uint8_t,
    0x82 as c_int as uint8_t,
    0x42 as c_int as uint8_t,
    0xc2 as c_int as uint8_t,
    0x22 as c_int as uint8_t,
    0xa2 as c_int as uint8_t,
    0x62 as c_int as uint8_t,
    0xe2 as c_int as uint8_t,
    0x12 as c_int as uint8_t,
    0x92 as c_int as uint8_t,
    0x52 as c_int as uint8_t,
    0xd2 as c_int as uint8_t,
    0x32 as c_int as uint8_t,
    0xb2 as c_int as uint8_t,
    0x72 as c_int as uint8_t,
    0xf2 as c_int as uint8_t,
    0xa as c_int as uint8_t,
    0x8a as c_int as uint8_t,
    0x4a as c_int as uint8_t,
    0xca as c_int as uint8_t,
    0x2a as c_int as uint8_t,
    0xaa as c_int as uint8_t,
    0x6a as c_int as uint8_t,
    0xea as c_int as uint8_t,
    0x1a as c_int as uint8_t,
    0x9a as c_int as uint8_t,
    0x5a as c_int as uint8_t,
    0xda as c_int as uint8_t,
    0x3a as c_int as uint8_t,
    0xba as c_int as uint8_t,
    0x7a as c_int as uint8_t,
    0xfa as c_int as uint8_t,
    0x6 as c_int as uint8_t,
    0x86 as c_int as uint8_t,
    0x46 as c_int as uint8_t,
    0xc6 as c_int as uint8_t,
    0x26 as c_int as uint8_t,
    0xa6 as c_int as uint8_t,
    0x66 as c_int as uint8_t,
    0xe6 as c_int as uint8_t,
    0x16 as c_int as uint8_t,
    0x96 as c_int as uint8_t,
    0x56 as c_int as uint8_t,
    0xd6 as c_int as uint8_t,
    0x36 as c_int as uint8_t,
    0xb6 as c_int as uint8_t,
    0x76 as c_int as uint8_t,
    0xf6 as c_int as uint8_t,
    0xe as c_int as uint8_t,
    0x8e as c_int as uint8_t,
    0x4e as c_int as uint8_t,
    0xce as c_int as uint8_t,
    0x2e as c_int as uint8_t,
    0xae as c_int as uint8_t,
    0x6e as c_int as uint8_t,
    0xee as c_int as uint8_t,
    0x1e as c_int as uint8_t,
    0x9e as c_int as uint8_t,
    0x5e as c_int as uint8_t,
    0xde as c_int as uint8_t,
    0x3e as c_int as uint8_t,
    0xbe as c_int as uint8_t,
    0x7e as c_int as uint8_t,
    0xfe as c_int as uint8_t,
    0x1 as c_int as uint8_t,
    0x81 as c_int as uint8_t,
    0x41 as c_int as uint8_t,
    0xc1 as c_int as uint8_t,
    0x21 as c_int as uint8_t,
    0xa1 as c_int as uint8_t,
    0x61 as c_int as uint8_t,
    0xe1 as c_int as uint8_t,
    0x11 as c_int as uint8_t,
    0x91 as c_int as uint8_t,
    0x51 as c_int as uint8_t,
    0xd1 as c_int as uint8_t,
    0x31 as c_int as uint8_t,
    0xb1 as c_int as uint8_t,
    0x71 as c_int as uint8_t,
    0xf1 as c_int as uint8_t,
    0x9 as c_int as uint8_t,
    0x89 as c_int as uint8_t,
    0x49 as c_int as uint8_t,
    0xc9 as c_int as uint8_t,
    0x29 as c_int as uint8_t,
    0xa9 as c_int as uint8_t,
    0x69 as c_int as uint8_t,
    0xe9 as c_int as uint8_t,
    0x19 as c_int as uint8_t,
    0x99 as c_int as uint8_t,
    0x59 as c_int as uint8_t,
    0xd9 as c_int as uint8_t,
    0x39 as c_int as uint8_t,
    0xb9 as c_int as uint8_t,
    0x79 as c_int as uint8_t,
    0xf9 as c_int as uint8_t,
    0x5 as c_int as uint8_t,
    0x85 as c_int as uint8_t,
    0x45 as c_int as uint8_t,
    0xc5 as c_int as uint8_t,
    0x25 as c_int as uint8_t,
    0xa5 as c_int as uint8_t,
    0x65 as c_int as uint8_t,
    0xe5 as c_int as uint8_t,
    0x15 as c_int as uint8_t,
    0x95 as c_int as uint8_t,
    0x55 as c_int as uint8_t,
    0xd5 as c_int as uint8_t,
    0x35 as c_int as uint8_t,
    0xb5 as c_int as uint8_t,
    0x75 as c_int as uint8_t,
    0xf5 as c_int as uint8_t,
    0xd as c_int as uint8_t,
    0x8d as c_int as uint8_t,
    0x4d as c_int as uint8_t,
    0xcd as c_int as uint8_t,
    0x2d as c_int as uint8_t,
    0xad as c_int as uint8_t,
    0x6d as c_int as uint8_t,
    0xed as c_int as uint8_t,
    0x1d as c_int as uint8_t,
    0x9d as c_int as uint8_t,
    0x5d as c_int as uint8_t,
    0xdd as c_int as uint8_t,
    0x3d as c_int as uint8_t,
    0xbd as c_int as uint8_t,
    0x7d as c_int as uint8_t,
    0xfd as c_int as uint8_t,
    0x3 as c_int as uint8_t,
    0x83 as c_int as uint8_t,
    0x43 as c_int as uint8_t,
    0xc3 as c_int as uint8_t,
    0x23 as c_int as uint8_t,
    0xa3 as c_int as uint8_t,
    0x63 as c_int as uint8_t,
    0xe3 as c_int as uint8_t,
    0x13 as c_int as uint8_t,
    0x93 as c_int as uint8_t,
    0x53 as c_int as uint8_t,
    0xd3 as c_int as uint8_t,
    0x33 as c_int as uint8_t,
    0xb3 as c_int as uint8_t,
    0x73 as c_int as uint8_t,
    0xf3 as c_int as uint8_t,
    0xb as c_int as uint8_t,
    0x8b as c_int as uint8_t,
    0x4b as c_int as uint8_t,
    0xcb as c_int as uint8_t,
    0x2b as c_int as uint8_t,
    0xab as c_int as uint8_t,
    0x6b as c_int as uint8_t,
    0xeb as c_int as uint8_t,
    0x1b as c_int as uint8_t,
    0x9b as c_int as uint8_t,
    0x5b as c_int as uint8_t,
    0xdb as c_int as uint8_t,
    0x3b as c_int as uint8_t,
    0xbb as c_int as uint8_t,
    0x7b as c_int as uint8_t,
    0xfb as c_int as uint8_t,
    0x7 as c_int as uint8_t,
    0x87 as c_int as uint8_t,
    0x47 as c_int as uint8_t,
    0xc7 as c_int as uint8_t,
    0x27 as c_int as uint8_t,
    0xa7 as c_int as uint8_t,
    0x67 as c_int as uint8_t,
    0xe7 as c_int as uint8_t,
    0x17 as c_int as uint8_t,
    0x97 as c_int as uint8_t,
    0x57 as c_int as uint8_t,
    0xd7 as c_int as uint8_t,
    0x37 as c_int as uint8_t,
    0xb7 as c_int as uint8_t,
    0x77 as c_int as uint8_t,
    0xf7 as c_int as uint8_t,
    0xf as c_int as uint8_t,
    0x8f as c_int as uint8_t,
    0x4f as c_int as uint8_t,
    0xcf as c_int as uint8_t,
    0x2f as c_int as uint8_t,
    0xaf as c_int as uint8_t,
    0x6f as c_int as uint8_t,
    0xef as c_int as uint8_t,
    0x1f as c_int as uint8_t,
    0x9f as c_int as uint8_t,
    0x5f as c_int as uint8_t,
    0xdf as c_int as uint8_t,
    0x3f as c_int as uint8_t,
    0xbf as c_int as uint8_t,
    0x7f as c_int as uint8_t,
    0xff as c_int as uint8_t,
];
pub const BROTLI_REVERSE_BITS_LOWEST: uint64_t = (1 as c_int as uint64_t)
    << BROTLI_REVERSE_BITS_MAX - 1 as c_int + BROTLI_REVERSE_BITS_BASE;
#[inline(always)]
unsafe extern "C" fn BrotliReverseBits(mut num: uint64_t) -> uint64_t {
    return kReverseBits[num as usize] as uint64_t;
}
#[inline(always)]
unsafe extern "C" fn ReplicateValue(
    mut table: *mut HuffmanCode,
    mut step: c_int,
    mut end: c_int,
    mut code: HuffmanCode,
) {
    loop {
        end -= step;
        *table.offset(end as isize) = code;
        if !(end > 0 as c_int) {
            break;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn NextTableBitSize(
    count: *const uint16_t,
    mut len: c_int,
    mut root_bits: c_int,
) -> c_int {
    let mut left: c_int = (1 as c_int) << len - root_bits;
    while len < BROTLI_HUFFMAN_MAX_CODE_LENGTH {
        left -= *count.offset(len as isize) as c_int;
        if left <= 0 as c_int {
            break;
        }
        len += 1;
        left <<= 1 as c_int;
    }
    return len - root_bits;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildCodeLengthsHuffmanTable(
    mut table: *mut HuffmanCode,
    code_lengths: *const uint8_t,
    mut count: *mut uint16_t,
) {
    let mut code: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
    let mut symbol: c_int = 0;
    let mut key: uint64_t = 0;
    let mut key_step: uint64_t = 0;
    let mut step: c_int = 0;
    let mut table_size: c_int = 0;
    let mut sorted: [c_int; 18] = [0; 18];
    let mut offset: [c_int; 6] = [0; 6];
    let mut bits: c_int = 0;
    let mut bits_count: c_int = 0;
    symbol = -(1 as c_int);
    bits = 1 as c_int;
    symbol += *count.offset(bits as isize) as c_int;
    offset[bits as usize] = symbol;
    bits += 1;
    symbol += *count.offset(bits as isize) as c_int;
    offset[bits as usize] = symbol;
    bits += 1;
    symbol += *count.offset(bits as isize) as c_int;
    offset[bits as usize] = symbol;
    bits += 1;
    symbol += *count.offset(bits as isize) as c_int;
    offset[bits as usize] = symbol;
    bits += 1;
    symbol += *count.offset(bits as isize) as c_int;
    offset[bits as usize] = symbol;
    bits += 1;
    offset[0 as c_int as usize] = BROTLI_CODE_LENGTH_CODES - 1 as c_int;
    symbol = BROTLI_CODE_LENGTH_CODES;
    loop {
        symbol -= 1;
        let fresh0 = offset[*code_lengths.offset(symbol as isize) as usize];
        offset[*code_lengths.offset(symbol as isize) as usize] =
            offset[*code_lengths.offset(symbol as isize) as usize] - 1;
        sorted[fresh0 as usize] = symbol;
        symbol -= 1;
        let fresh1 = offset[*code_lengths.offset(symbol as isize) as usize];
        offset[*code_lengths.offset(symbol as isize) as usize] =
            offset[*code_lengths.offset(symbol as isize) as usize] - 1;
        sorted[fresh1 as usize] = symbol;
        symbol -= 1;
        let fresh2 = offset[*code_lengths.offset(symbol as isize) as usize];
        offset[*code_lengths.offset(symbol as isize) as usize] =
            offset[*code_lengths.offset(symbol as isize) as usize] - 1;
        sorted[fresh2 as usize] = symbol;
        symbol -= 1;
        let fresh3 = offset[*code_lengths.offset(symbol as isize) as usize];
        offset[*code_lengths.offset(symbol as isize) as usize] =
            offset[*code_lengths.offset(symbol as isize) as usize] - 1;
        sorted[fresh3 as usize] = symbol;
        symbol -= 1;
        let fresh4 = offset[*code_lengths.offset(symbol as isize) as usize];
        offset[*code_lengths.offset(symbol as isize) as usize] =
            offset[*code_lengths.offset(symbol as isize) as usize] - 1;
        sorted[fresh4 as usize] = symbol;
        symbol -= 1;
        let fresh5 = offset[*code_lengths.offset(symbol as isize) as usize];
        offset[*code_lengths.offset(symbol as isize) as usize] =
            offset[*code_lengths.offset(symbol as isize) as usize] - 1;
        sorted[fresh5 as usize] = symbol;
        if !(symbol != 0 as c_int) {
            break;
        }
    }
    table_size = (1 as c_int) << BROTLI_HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH;
    if offset[0 as c_int as usize] == 0 as c_int {
        code = ConstructHuffmanCode(
            0 as uint8_t,
            sorted[0 as c_int as usize] as uint16_t,
        );
        key = 0 as uint64_t;
        while key < table_size as uint64_t {
            *table.offset(key as isize) = code;
            key = key.wrapping_add(1);
        }
        return;
    }
    key = 0 as uint64_t;
    key_step = BROTLI_REVERSE_BITS_LOWEST;
    symbol = 0 as c_int;
    bits = 1 as c_int;
    step = 2 as c_int;
    loop {
        bits_count = *count.offset(bits as isize) as c_int;
        while bits_count != 0 as c_int {
            let fresh6 = symbol;
            symbol = symbol + 1;
            code = ConstructHuffmanCode(bits as uint8_t, sorted[fresh6 as usize] as uint16_t);
            ReplicateValue(
                table.offset(
                    (BrotliReverseBits as unsafe extern "C" fn(uint64_t) -> uint64_t)(key) as isize,
                ) as *mut HuffmanCode,
                step,
                table_size,
                code,
            );
            key = (key as c_ulong).wrapping_add(key_step as c_ulong)
                as uint64_t as uint64_t;
            bits_count -= 1;
        }
        step <<= 1 as c_int;
        key_step >>= 1 as c_int;
        bits += 1;
        if !(bits <= BROTLI_HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildHuffmanTable(
    mut root_table: *mut HuffmanCode,
    mut root_bits: c_int,
    symbol_lists: *const uint16_t,
    mut count: *mut uint16_t,
) -> uint32_t {
    let mut code: HuffmanCode = HuffmanCode { bits: 0, value: 0 };
    let mut table: *mut HuffmanCode = ::core::ptr::null_mut::<HuffmanCode>();
    let mut len: c_int = 0;
    let mut symbol: c_int = 0;
    let mut key: uint64_t = 0;
    let mut key_step: uint64_t = 0;
    let mut sub_key: uint64_t = 0;
    let mut sub_key_step: uint64_t = 0;
    let mut step: c_int = 0;
    let mut table_bits: c_int = 0;
    let mut table_size: c_int = 0;
    let mut total_size: c_int = 0;
    let mut max_length: c_int = -(1 as c_int);
    let mut bits: c_int = 0;
    let mut bits_count: c_int = 0;
    while *symbol_lists.offset(max_length as isize) as c_int
        == 0xffff as c_int
    {
        max_length -= 1;
    }
    max_length += BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1 as c_int;
    table = root_table;
    table_bits = root_bits;
    table_size = (1 as c_int) << table_bits;
    total_size = table_size;
    if table_bits > max_length {
        table_bits = max_length;
        table_size = (1 as c_int) << table_bits;
    }
    key = 0 as uint64_t;
    key_step = BROTLI_REVERSE_BITS_LOWEST;
    bits = 1 as c_int;
    step = 2 as c_int;
    loop {
        symbol = bits - (BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1 as c_int);
        bits_count = *count.offset(bits as isize) as c_int;
        while bits_count != 0 as c_int {
            symbol = *symbol_lists.offset(symbol as isize) as c_int;
            code = ConstructHuffmanCode(bits as uint8_t, symbol as uint16_t);
            ReplicateValue(
                table.offset(
                    (BrotliReverseBits as unsafe extern "C" fn(uint64_t) -> uint64_t)(key) as isize,
                ) as *mut HuffmanCode,
                step,
                table_size,
                code,
            );
            key = (key as c_ulong).wrapping_add(key_step as c_ulong)
                as uint64_t as uint64_t;
            bits_count -= 1;
        }
        step <<= 1 as c_int;
        key_step >>= 1 as c_int;
        bits += 1;
        if !(bits <= table_bits) {
            break;
        }
    }
    while total_size != table_size {
        memcpy(
            table.offset(table_size as isize) as *mut HuffmanCode as *mut c_void,
            table.offset(0 as c_int as isize) as *mut HuffmanCode
                as *const c_void,
            (table_size as size_t).wrapping_mul(::core::mem::size_of::<HuffmanCode>() as size_t),
        );
        table_size <<= 1 as c_int;
    }
    key_step = BROTLI_REVERSE_BITS_LOWEST >> root_bits - 1 as c_int;
    sub_key = BROTLI_REVERSE_BITS_LOWEST << 1 as c_int;
    sub_key_step = BROTLI_REVERSE_BITS_LOWEST;
    len = root_bits + 1 as c_int;
    step = 2 as c_int;
    while len <= max_length {
        symbol = len - (BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1 as c_int);
        while *count.offset(len as isize) as c_int != 0 as c_int {
            if sub_key == BROTLI_REVERSE_BITS_LOWEST << 1 as c_uint {
                table = table.offset(table_size as isize);
                table_bits = NextTableBitSize(count, len, root_bits);
                table_size = (1 as c_int) << table_bits;
                total_size += table_size;
                sub_key = BrotliReverseBits(key);
                key = (key as c_ulong).wrapping_add(key_step as c_ulong)
                    as uint64_t as uint64_t;
                *root_table.offset(sub_key as isize) = ConstructHuffmanCode(
                    (table_bits + root_bits) as uint8_t,
                    (table.offset_from(root_table) as c_long as size_t)
                        .wrapping_sub(sub_key as size_t) as uint16_t,
                );
                sub_key = 0 as uint64_t;
            }
            symbol = *symbol_lists.offset(symbol as isize) as c_int;
            code = ConstructHuffmanCode((len - root_bits) as uint8_t, symbol as uint16_t);
            ReplicateValue(
                table.offset(
                    (BrotliReverseBits as unsafe extern "C" fn(uint64_t) -> uint64_t)(sub_key)
                        as isize,
                ) as *mut HuffmanCode,
                step,
                table_size,
                code,
            );
            sub_key = (sub_key as c_ulong)
                .wrapping_add(sub_key_step as c_ulong)
                as uint64_t as uint64_t;
            let ref mut fresh7 = *count.offset(len as isize);
            *fresh7 = (*fresh7).wrapping_sub(1);
        }
        step <<= 1 as c_int;
        sub_key_step >>= 1 as c_int;
        len += 1;
    }
    return total_size as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn BrotliBuildSimpleHuffmanTable(
    mut table: *mut HuffmanCode,
    mut root_bits: c_int,
    mut val: *mut uint16_t,
    mut num_symbols: uint32_t,
) -> uint32_t {
    let mut table_size: uint32_t = 1 as uint32_t;
    let goal_size: uint32_t = (1 as uint32_t) << root_bits;
    match num_symbols {
        0 => {
            *table.offset(0 as c_int as isize) =
                ConstructHuffmanCode(0 as uint8_t, *val.offset(0 as c_int as isize));
        }
        1 => {
            if *val.offset(1 as c_int as isize) as c_int
                > *val.offset(0 as c_int as isize) as c_int
            {
                *table.offset(0 as c_int as isize) = ConstructHuffmanCode(
                    1 as uint8_t,
                    *val.offset(0 as c_int as isize),
                );
                *table.offset(1 as c_int as isize) = ConstructHuffmanCode(
                    1 as uint8_t,
                    *val.offset(1 as c_int as isize),
                );
            } else {
                *table.offset(0 as c_int as isize) = ConstructHuffmanCode(
                    1 as uint8_t,
                    *val.offset(1 as c_int as isize),
                );
                *table.offset(1 as c_int as isize) = ConstructHuffmanCode(
                    1 as uint8_t,
                    *val.offset(0 as c_int as isize),
                );
            }
            table_size = 2 as uint32_t;
        }
        2 => {
            *table.offset(0 as c_int as isize) =
                ConstructHuffmanCode(1 as uint8_t, *val.offset(0 as c_int as isize));
            *table.offset(2 as c_int as isize) =
                ConstructHuffmanCode(1 as uint8_t, *val.offset(0 as c_int as isize));
            if *val.offset(2 as c_int as isize) as c_int
                > *val.offset(1 as c_int as isize) as c_int
            {
                *table.offset(1 as c_int as isize) = ConstructHuffmanCode(
                    2 as uint8_t,
                    *val.offset(1 as c_int as isize),
                );
                *table.offset(3 as c_int as isize) = ConstructHuffmanCode(
                    2 as uint8_t,
                    *val.offset(2 as c_int as isize),
                );
            } else {
                *table.offset(1 as c_int as isize) = ConstructHuffmanCode(
                    2 as uint8_t,
                    *val.offset(2 as c_int as isize),
                );
                *table.offset(3 as c_int as isize) = ConstructHuffmanCode(
                    2 as uint8_t,
                    *val.offset(1 as c_int as isize),
                );
            }
            table_size = 4 as uint32_t;
        }
        3 => {
            let mut i: c_int = 0;
            let mut k: c_int = 0;
            i = 0 as c_int;
            while i < 3 as c_int {
                k = i + 1 as c_int;
                while k < 4 as c_int {
                    if (*val.offset(k as isize) as c_int)
                        < *val.offset(i as isize) as c_int
                    {
                        let mut t: uint16_t = *val.offset(k as isize);
                        *val.offset(k as isize) = *val.offset(i as isize);
                        *val.offset(i as isize) = t;
                    }
                    k += 1;
                }
                i += 1;
            }
            *table.offset(0 as c_int as isize) =
                ConstructHuffmanCode(2 as uint8_t, *val.offset(0 as c_int as isize));
            *table.offset(2 as c_int as isize) =
                ConstructHuffmanCode(2 as uint8_t, *val.offset(1 as c_int as isize));
            *table.offset(1 as c_int as isize) =
                ConstructHuffmanCode(2 as uint8_t, *val.offset(2 as c_int as isize));
            *table.offset(3 as c_int as isize) =
                ConstructHuffmanCode(2 as uint8_t, *val.offset(3 as c_int as isize));
            table_size = 4 as uint32_t;
        }
        4 => {
            if (*val.offset(3 as c_int as isize) as c_int)
                < *val.offset(2 as c_int as isize) as c_int
            {
                let mut t_0: uint16_t = *val.offset(3 as c_int as isize);
                *val.offset(3 as c_int as isize) =
                    *val.offset(2 as c_int as isize);
                *val.offset(2 as c_int as isize) = t_0;
            }
            *table.offset(0 as c_int as isize) =
                ConstructHuffmanCode(1 as uint8_t, *val.offset(0 as c_int as isize));
            *table.offset(1 as c_int as isize) =
                ConstructHuffmanCode(2 as uint8_t, *val.offset(1 as c_int as isize));
            *table.offset(2 as c_int as isize) =
                ConstructHuffmanCode(1 as uint8_t, *val.offset(0 as c_int as isize));
            *table.offset(3 as c_int as isize) =
                ConstructHuffmanCode(3 as uint8_t, *val.offset(2 as c_int as isize));
            *table.offset(4 as c_int as isize) =
                ConstructHuffmanCode(1 as uint8_t, *val.offset(0 as c_int as isize));
            *table.offset(5 as c_int as isize) =
                ConstructHuffmanCode(2 as uint8_t, *val.offset(1 as c_int as isize));
            *table.offset(6 as c_int as isize) =
                ConstructHuffmanCode(1 as uint8_t, *val.offset(0 as c_int as isize));
            *table.offset(7 as c_int as isize) =
                ConstructHuffmanCode(3 as uint8_t, *val.offset(3 as c_int as isize));
            table_size = 8 as uint32_t;
        }
        _ => {}
    }
    while table_size != goal_size {
        memcpy(
            table.offset(table_size as isize) as *mut HuffmanCode as *mut c_void,
            table.offset(0 as c_int as isize) as *mut HuffmanCode
                as *const c_void,
            (table_size as size_t).wrapping_mul(::core::mem::size_of::<HuffmanCode>() as size_t),
        );
        table_size <<= 1 as c_int;
    }
    return goal_size;
}

