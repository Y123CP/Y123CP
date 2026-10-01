use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;
pub use crate::src::c_structs::*;

pub type xmlChLRangePtr = *mut xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: c_int,
    pub nbLongRange: c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChRangeGroup = _xmlChRangeGroup;

#[no_mangle]
pub static mut xmlIsPubidChar_tab: [c_uchar; 256] = [
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0x1 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
];
static mut xmlIsBaseChar_srng: [xmlChSRange; 197] = [
    _xmlChSRange {
        low: 0x100 as c_ushort,
        high: 0x131 as c_ushort,
    },
    _xmlChSRange {
        low: 0x134 as c_ushort,
        high: 0x13e as c_ushort,
    },
    _xmlChSRange {
        low: 0x141 as c_ushort,
        high: 0x148 as c_ushort,
    },
    _xmlChSRange {
        low: 0x14a as c_ushort,
        high: 0x17e as c_ushort,
    },
    _xmlChSRange {
        low: 0x180 as c_ushort,
        high: 0x1c3 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1cd as c_ushort,
        high: 0x1f0 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f4 as c_ushort,
        high: 0x1f5 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1fa as c_ushort,
        high: 0x217 as c_ushort,
    },
    _xmlChSRange {
        low: 0x250 as c_ushort,
        high: 0x2a8 as c_ushort,
    },
    _xmlChSRange {
        low: 0x2bb as c_ushort,
        high: 0x2c1 as c_ushort,
    },
    _xmlChSRange {
        low: 0x386 as c_ushort,
        high: 0x386 as c_ushort,
    },
    _xmlChSRange {
        low: 0x388 as c_ushort,
        high: 0x38a as c_ushort,
    },
    _xmlChSRange {
        low: 0x38c as c_ushort,
        high: 0x38c as c_ushort,
    },
    _xmlChSRange {
        low: 0x38e as c_ushort,
        high: 0x3a1 as c_ushort,
    },
    _xmlChSRange {
        low: 0x3a3 as c_ushort,
        high: 0x3ce as c_ushort,
    },
    _xmlChSRange {
        low: 0x3d0 as c_ushort,
        high: 0x3d6 as c_ushort,
    },
    _xmlChSRange {
        low: 0x3da as c_ushort,
        high: 0x3da as c_ushort,
    },
    _xmlChSRange {
        low: 0x3dc as c_ushort,
        high: 0x3dc as c_ushort,
    },
    _xmlChSRange {
        low: 0x3de as c_ushort,
        high: 0x3de as c_ushort,
    },
    _xmlChSRange {
        low: 0x3e0 as c_ushort,
        high: 0x3e0 as c_ushort,
    },
    _xmlChSRange {
        low: 0x3e2 as c_ushort,
        high: 0x3f3 as c_ushort,
    },
    _xmlChSRange {
        low: 0x401 as c_ushort,
        high: 0x40c as c_ushort,
    },
    _xmlChSRange {
        low: 0x40e as c_ushort,
        high: 0x44f as c_ushort,
    },
    _xmlChSRange {
        low: 0x451 as c_ushort,
        high: 0x45c as c_ushort,
    },
    _xmlChSRange {
        low: 0x45e as c_ushort,
        high: 0x481 as c_ushort,
    },
    _xmlChSRange {
        low: 0x490 as c_ushort,
        high: 0x4c4 as c_ushort,
    },
    _xmlChSRange {
        low: 0x4c7 as c_ushort,
        high: 0x4c8 as c_ushort,
    },
    _xmlChSRange {
        low: 0x4cb as c_ushort,
        high: 0x4cc as c_ushort,
    },
    _xmlChSRange {
        low: 0x4d0 as c_ushort,
        high: 0x4eb as c_ushort,
    },
    _xmlChSRange {
        low: 0x4ee as c_ushort,
        high: 0x4f5 as c_ushort,
    },
    _xmlChSRange {
        low: 0x4f8 as c_ushort,
        high: 0x4f9 as c_ushort,
    },
    _xmlChSRange {
        low: 0x531 as c_ushort,
        high: 0x556 as c_ushort,
    },
    _xmlChSRange {
        low: 0x559 as c_ushort,
        high: 0x559 as c_ushort,
    },
    _xmlChSRange {
        low: 0x561 as c_ushort,
        high: 0x586 as c_ushort,
    },
    _xmlChSRange {
        low: 0x5d0 as c_ushort,
        high: 0x5ea as c_ushort,
    },
    _xmlChSRange {
        low: 0x5f0 as c_ushort,
        high: 0x5f2 as c_ushort,
    },
    _xmlChSRange {
        low: 0x621 as c_ushort,
        high: 0x63a as c_ushort,
    },
    _xmlChSRange {
        low: 0x641 as c_ushort,
        high: 0x64a as c_ushort,
    },
    _xmlChSRange {
        low: 0x671 as c_ushort,
        high: 0x6b7 as c_ushort,
    },
    _xmlChSRange {
        low: 0x6ba as c_ushort,
        high: 0x6be as c_ushort,
    },
    _xmlChSRange {
        low: 0x6c0 as c_ushort,
        high: 0x6ce as c_ushort,
    },
    _xmlChSRange {
        low: 0x6d0 as c_ushort,
        high: 0x6d3 as c_ushort,
    },
    _xmlChSRange {
        low: 0x6d5 as c_ushort,
        high: 0x6d5 as c_ushort,
    },
    _xmlChSRange {
        low: 0x6e5 as c_ushort,
        high: 0x6e6 as c_ushort,
    },
    _xmlChSRange {
        low: 0x905 as c_ushort,
        high: 0x939 as c_ushort,
    },
    _xmlChSRange {
        low: 0x93d as c_ushort,
        high: 0x93d as c_ushort,
    },
    _xmlChSRange {
        low: 0x958 as c_ushort,
        high: 0x961 as c_ushort,
    },
    _xmlChSRange {
        low: 0x985 as c_ushort,
        high: 0x98c as c_ushort,
    },
    _xmlChSRange {
        low: 0x98f as c_ushort,
        high: 0x990 as c_ushort,
    },
    _xmlChSRange {
        low: 0x993 as c_ushort,
        high: 0x9a8 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9aa as c_ushort,
        high: 0x9b0 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9b2 as c_ushort,
        high: 0x9b2 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9b6 as c_ushort,
        high: 0x9b9 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9dc as c_ushort,
        high: 0x9dd as c_ushort,
    },
    _xmlChSRange {
        low: 0x9df as c_ushort,
        high: 0x9e1 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9f0 as c_ushort,
        high: 0x9f1 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa05 as c_ushort,
        high: 0xa0a as c_ushort,
    },
    _xmlChSRange {
        low: 0xa0f as c_ushort,
        high: 0xa10 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa13 as c_ushort,
        high: 0xa28 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa2a as c_ushort,
        high: 0xa30 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa32 as c_ushort,
        high: 0xa33 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa35 as c_ushort,
        high: 0xa36 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa38 as c_ushort,
        high: 0xa39 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa59 as c_ushort,
        high: 0xa5c as c_ushort,
    },
    _xmlChSRange {
        low: 0xa5e as c_ushort,
        high: 0xa5e as c_ushort,
    },
    _xmlChSRange {
        low: 0xa72 as c_ushort,
        high: 0xa74 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa85 as c_ushort,
        high: 0xa8b as c_ushort,
    },
    _xmlChSRange {
        low: 0xa8d as c_ushort,
        high: 0xa8d as c_ushort,
    },
    _xmlChSRange {
        low: 0xa8f as c_ushort,
        high: 0xa91 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa93 as c_ushort,
        high: 0xaa8 as c_ushort,
    },
    _xmlChSRange {
        low: 0xaaa as c_ushort,
        high: 0xab0 as c_ushort,
    },
    _xmlChSRange {
        low: 0xab2 as c_ushort,
        high: 0xab3 as c_ushort,
    },
    _xmlChSRange {
        low: 0xab5 as c_ushort,
        high: 0xab9 as c_ushort,
    },
    _xmlChSRange {
        low: 0xabd as c_ushort,
        high: 0xabd as c_ushort,
    },
    _xmlChSRange {
        low: 0xae0 as c_ushort,
        high: 0xae0 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb05 as c_ushort,
        high: 0xb0c as c_ushort,
    },
    _xmlChSRange {
        low: 0xb0f as c_ushort,
        high: 0xb10 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb13 as c_ushort,
        high: 0xb28 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb2a as c_ushort,
        high: 0xb30 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb32 as c_ushort,
        high: 0xb33 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb36 as c_ushort,
        high: 0xb39 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb3d as c_ushort,
        high: 0xb3d as c_ushort,
    },
    _xmlChSRange {
        low: 0xb5c as c_ushort,
        high: 0xb5d as c_ushort,
    },
    _xmlChSRange {
        low: 0xb5f as c_ushort,
        high: 0xb61 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb85 as c_ushort,
        high: 0xb8a as c_ushort,
    },
    _xmlChSRange {
        low: 0xb8e as c_ushort,
        high: 0xb90 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb92 as c_ushort,
        high: 0xb95 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb99 as c_ushort,
        high: 0xb9a as c_ushort,
    },
    _xmlChSRange {
        low: 0xb9c as c_ushort,
        high: 0xb9c as c_ushort,
    },
    _xmlChSRange {
        low: 0xb9e as c_ushort,
        high: 0xb9f as c_ushort,
    },
    _xmlChSRange {
        low: 0xba3 as c_ushort,
        high: 0xba4 as c_ushort,
    },
    _xmlChSRange {
        low: 0xba8 as c_ushort,
        high: 0xbaa as c_ushort,
    },
    _xmlChSRange {
        low: 0xbae as c_ushort,
        high: 0xbb5 as c_ushort,
    },
    _xmlChSRange {
        low: 0xbb7 as c_ushort,
        high: 0xbb9 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc05 as c_ushort,
        high: 0xc0c as c_ushort,
    },
    _xmlChSRange {
        low: 0xc0e as c_ushort,
        high: 0xc10 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc12 as c_ushort,
        high: 0xc28 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc2a as c_ushort,
        high: 0xc33 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc35 as c_ushort,
        high: 0xc39 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc60 as c_ushort,
        high: 0xc61 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc85 as c_ushort,
        high: 0xc8c as c_ushort,
    },
    _xmlChSRange {
        low: 0xc8e as c_ushort,
        high: 0xc90 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc92 as c_ushort,
        high: 0xca8 as c_ushort,
    },
    _xmlChSRange {
        low: 0xcaa as c_ushort,
        high: 0xcb3 as c_ushort,
    },
    _xmlChSRange {
        low: 0xcb5 as c_ushort,
        high: 0xcb9 as c_ushort,
    },
    _xmlChSRange {
        low: 0xcde as c_ushort,
        high: 0xcde as c_ushort,
    },
    _xmlChSRange {
        low: 0xce0 as c_ushort,
        high: 0xce1 as c_ushort,
    },
    _xmlChSRange {
        low: 0xd05 as c_ushort,
        high: 0xd0c as c_ushort,
    },
    _xmlChSRange {
        low: 0xd0e as c_ushort,
        high: 0xd10 as c_ushort,
    },
    _xmlChSRange {
        low: 0xd12 as c_ushort,
        high: 0xd28 as c_ushort,
    },
    _xmlChSRange {
        low: 0xd2a as c_ushort,
        high: 0xd39 as c_ushort,
    },
    _xmlChSRange {
        low: 0xd60 as c_ushort,
        high: 0xd61 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe01 as c_ushort,
        high: 0xe2e as c_ushort,
    },
    _xmlChSRange {
        low: 0xe30 as c_ushort,
        high: 0xe30 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe32 as c_ushort,
        high: 0xe33 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe40 as c_ushort,
        high: 0xe45 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe81 as c_ushort,
        high: 0xe82 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe84 as c_ushort,
        high: 0xe84 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe87 as c_ushort,
        high: 0xe88 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe8a as c_ushort,
        high: 0xe8a as c_ushort,
    },
    _xmlChSRange {
        low: 0xe8d as c_ushort,
        high: 0xe8d as c_ushort,
    },
    _xmlChSRange {
        low: 0xe94 as c_ushort,
        high: 0xe97 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe99 as c_ushort,
        high: 0xe9f as c_ushort,
    },
    _xmlChSRange {
        low: 0xea1 as c_ushort,
        high: 0xea3 as c_ushort,
    },
    _xmlChSRange {
        low: 0xea5 as c_ushort,
        high: 0xea5 as c_ushort,
    },
    _xmlChSRange {
        low: 0xea7 as c_ushort,
        high: 0xea7 as c_ushort,
    },
    _xmlChSRange {
        low: 0xeaa as c_ushort,
        high: 0xeab as c_ushort,
    },
    _xmlChSRange {
        low: 0xead as c_ushort,
        high: 0xeae as c_ushort,
    },
    _xmlChSRange {
        low: 0xeb0 as c_ushort,
        high: 0xeb0 as c_ushort,
    },
    _xmlChSRange {
        low: 0xeb2 as c_ushort,
        high: 0xeb3 as c_ushort,
    },
    _xmlChSRange {
        low: 0xebd as c_ushort,
        high: 0xebd as c_ushort,
    },
    _xmlChSRange {
        low: 0xec0 as c_ushort,
        high: 0xec4 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf40 as c_ushort,
        high: 0xf47 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf49 as c_ushort,
        high: 0xf69 as c_ushort,
    },
    _xmlChSRange {
        low: 0x10a0 as c_ushort,
        high: 0x10c5 as c_ushort,
    },
    _xmlChSRange {
        low: 0x10d0 as c_ushort,
        high: 0x10f6 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1100 as c_ushort,
        high: 0x1100 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1102 as c_ushort,
        high: 0x1103 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1105 as c_ushort,
        high: 0x1107 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1109 as c_ushort,
        high: 0x1109 as c_ushort,
    },
    _xmlChSRange {
        low: 0x110b as c_ushort,
        high: 0x110c as c_ushort,
    },
    _xmlChSRange {
        low: 0x110e as c_ushort,
        high: 0x1112 as c_ushort,
    },
    _xmlChSRange {
        low: 0x113c as c_ushort,
        high: 0x113c as c_ushort,
    },
    _xmlChSRange {
        low: 0x113e as c_ushort,
        high: 0x113e as c_ushort,
    },
    _xmlChSRange {
        low: 0x1140 as c_ushort,
        high: 0x1140 as c_ushort,
    },
    _xmlChSRange {
        low: 0x114c as c_ushort,
        high: 0x114c as c_ushort,
    },
    _xmlChSRange {
        low: 0x114e as c_ushort,
        high: 0x114e as c_ushort,
    },
    _xmlChSRange {
        low: 0x1150 as c_ushort,
        high: 0x1150 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1154 as c_ushort,
        high: 0x1155 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1159 as c_ushort,
        high: 0x1159 as c_ushort,
    },
    _xmlChSRange {
        low: 0x115f as c_ushort,
        high: 0x1161 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1163 as c_ushort,
        high: 0x1163 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1165 as c_ushort,
        high: 0x1165 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1167 as c_ushort,
        high: 0x1167 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1169 as c_ushort,
        high: 0x1169 as c_ushort,
    },
    _xmlChSRange {
        low: 0x116d as c_ushort,
        high: 0x116e as c_ushort,
    },
    _xmlChSRange {
        low: 0x1172 as c_ushort,
        high: 0x1173 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1175 as c_ushort,
        high: 0x1175 as c_ushort,
    },
    _xmlChSRange {
        low: 0x119e as c_ushort,
        high: 0x119e as c_ushort,
    },
    _xmlChSRange {
        low: 0x11a8 as c_ushort,
        high: 0x11a8 as c_ushort,
    },
    _xmlChSRange {
        low: 0x11ab as c_ushort,
        high: 0x11ab as c_ushort,
    },
    _xmlChSRange {
        low: 0x11ae as c_ushort,
        high: 0x11af as c_ushort,
    },
    _xmlChSRange {
        low: 0x11b7 as c_ushort,
        high: 0x11b8 as c_ushort,
    },
    _xmlChSRange {
        low: 0x11ba as c_ushort,
        high: 0x11ba as c_ushort,
    },
    _xmlChSRange {
        low: 0x11bc as c_ushort,
        high: 0x11c2 as c_ushort,
    },
    _xmlChSRange {
        low: 0x11eb as c_ushort,
        high: 0x11eb as c_ushort,
    },
    _xmlChSRange {
        low: 0x11f0 as c_ushort,
        high: 0x11f0 as c_ushort,
    },
    _xmlChSRange {
        low: 0x11f9 as c_ushort,
        high: 0x11f9 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1e00 as c_ushort,
        high: 0x1e9b as c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea0 as c_ushort,
        high: 0x1ef9 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f00 as c_ushort,
        high: 0x1f15 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f18 as c_ushort,
        high: 0x1f1d as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f20 as c_ushort,
        high: 0x1f45 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f48 as c_ushort,
        high: 0x1f4d as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f50 as c_ushort,
        high: 0x1f57 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f59 as c_ushort,
        high: 0x1f59 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5b as c_ushort,
        high: 0x1f5b as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5d as c_ushort,
        high: 0x1f5d as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5f as c_ushort,
        high: 0x1f7d as c_ushort,
    },
    _xmlChSRange {
        low: 0x1f80 as c_ushort,
        high: 0x1fb4 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1fb6 as c_ushort,
        high: 0x1fbc as c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbe as c_ushort,
        high: 0x1fbe as c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc2 as c_ushort,
        high: 0x1fc4 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc6 as c_ushort,
        high: 0x1fcc as c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd0 as c_ushort,
        high: 0x1fd3 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd6 as c_ushort,
        high: 0x1fdb as c_ushort,
    },
    _xmlChSRange {
        low: 0x1fe0 as c_ushort,
        high: 0x1fec as c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff2 as c_ushort,
        high: 0x1ff4 as c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff6 as c_ushort,
        high: 0x1ffc as c_ushort,
    },
    _xmlChSRange {
        low: 0x2126 as c_ushort,
        high: 0x2126 as c_ushort,
    },
    _xmlChSRange {
        low: 0x212a as c_ushort,
        high: 0x212b as c_ushort,
    },
    _xmlChSRange {
        low: 0x212e as c_ushort,
        high: 0x212e as c_ushort,
    },
    _xmlChSRange {
        low: 0x2180 as c_ushort,
        high: 0x2182 as c_ushort,
    },
    _xmlChSRange {
        low: 0x3041 as c_ushort,
        high: 0x3094 as c_ushort,
    },
    _xmlChSRange {
        low: 0x30a1 as c_ushort,
        high: 0x30fa as c_ushort,
    },
    _xmlChSRange {
        low: 0x3105 as c_ushort,
        high: 0x312c as c_ushort,
    },
    _xmlChSRange {
        low: 0xac00 as c_ushort,
        high: 0xd7a3 as c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsBaseCharGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 197 as c_int,
        nbLongRange: 0 as c_int,
        shortRange: &raw const xmlIsBaseChar_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
static mut xmlIsChar_srng: [xmlChSRange; 2] = [
    _xmlChSRange {
        low: 0x100 as c_ushort,
        high: 0xd7ff as c_ushort,
    },
    _xmlChSRange {
        low: 0xe000 as c_ushort,
        high: 0xfffd as c_ushort,
    },
];
static mut xmlIsChar_lrng: [xmlChLRange; 1] = [_xmlChLRange {
    low: 0x10000 as c_int as c_uint,
    high: 0x10ffff as c_int as c_uint,
}];
#[no_mangle]
pub static mut xmlIsCharGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 2 as c_int,
        nbLongRange: 1 as c_int,
        shortRange: &raw const xmlIsChar_srng as *const xmlChSRange,
        longRange: &raw const xmlIsChar_lrng as *const xmlChLRange,
    }
};
static mut xmlIsCombining_srng: [xmlChSRange; 95] = [
    _xmlChSRange {
        low: 0x300 as c_ushort,
        high: 0x345 as c_ushort,
    },
    _xmlChSRange {
        low: 0x360 as c_ushort,
        high: 0x361 as c_ushort,
    },
    _xmlChSRange {
        low: 0x483 as c_ushort,
        high: 0x486 as c_ushort,
    },
    _xmlChSRange {
        low: 0x591 as c_ushort,
        high: 0x5a1 as c_ushort,
    },
    _xmlChSRange {
        low: 0x5a3 as c_ushort,
        high: 0x5b9 as c_ushort,
    },
    _xmlChSRange {
        low: 0x5bb as c_ushort,
        high: 0x5bd as c_ushort,
    },
    _xmlChSRange {
        low: 0x5bf as c_ushort,
        high: 0x5bf as c_ushort,
    },
    _xmlChSRange {
        low: 0x5c1 as c_ushort,
        high: 0x5c2 as c_ushort,
    },
    _xmlChSRange {
        low: 0x5c4 as c_ushort,
        high: 0x5c4 as c_ushort,
    },
    _xmlChSRange {
        low: 0x64b as c_ushort,
        high: 0x652 as c_ushort,
    },
    _xmlChSRange {
        low: 0x670 as c_ushort,
        high: 0x670 as c_ushort,
    },
    _xmlChSRange {
        low: 0x6d6 as c_ushort,
        high: 0x6dc as c_ushort,
    },
    _xmlChSRange {
        low: 0x6dd as c_ushort,
        high: 0x6df as c_ushort,
    },
    _xmlChSRange {
        low: 0x6e0 as c_ushort,
        high: 0x6e4 as c_ushort,
    },
    _xmlChSRange {
        low: 0x6e7 as c_ushort,
        high: 0x6e8 as c_ushort,
    },
    _xmlChSRange {
        low: 0x6ea as c_ushort,
        high: 0x6ed as c_ushort,
    },
    _xmlChSRange {
        low: 0x901 as c_ushort,
        high: 0x903 as c_ushort,
    },
    _xmlChSRange {
        low: 0x93c as c_ushort,
        high: 0x93c as c_ushort,
    },
    _xmlChSRange {
        low: 0x93e as c_ushort,
        high: 0x94c as c_ushort,
    },
    _xmlChSRange {
        low: 0x94d as c_ushort,
        high: 0x94d as c_ushort,
    },
    _xmlChSRange {
        low: 0x951 as c_ushort,
        high: 0x954 as c_ushort,
    },
    _xmlChSRange {
        low: 0x962 as c_ushort,
        high: 0x963 as c_ushort,
    },
    _xmlChSRange {
        low: 0x981 as c_ushort,
        high: 0x983 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9bc as c_ushort,
        high: 0x9bc as c_ushort,
    },
    _xmlChSRange {
        low: 0x9be as c_ushort,
        high: 0x9be as c_ushort,
    },
    _xmlChSRange {
        low: 0x9bf as c_ushort,
        high: 0x9bf as c_ushort,
    },
    _xmlChSRange {
        low: 0x9c0 as c_ushort,
        high: 0x9c4 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9c7 as c_ushort,
        high: 0x9c8 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9cb as c_ushort,
        high: 0x9cd as c_ushort,
    },
    _xmlChSRange {
        low: 0x9d7 as c_ushort,
        high: 0x9d7 as c_ushort,
    },
    _xmlChSRange {
        low: 0x9e2 as c_ushort,
        high: 0x9e3 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa02 as c_ushort,
        high: 0xa02 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa3c as c_ushort,
        high: 0xa3c as c_ushort,
    },
    _xmlChSRange {
        low: 0xa3e as c_ushort,
        high: 0xa3e as c_ushort,
    },
    _xmlChSRange {
        low: 0xa3f as c_ushort,
        high: 0xa3f as c_ushort,
    },
    _xmlChSRange {
        low: 0xa40 as c_ushort,
        high: 0xa42 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa47 as c_ushort,
        high: 0xa48 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa4b as c_ushort,
        high: 0xa4d as c_ushort,
    },
    _xmlChSRange {
        low: 0xa70 as c_ushort,
        high: 0xa71 as c_ushort,
    },
    _xmlChSRange {
        low: 0xa81 as c_ushort,
        high: 0xa83 as c_ushort,
    },
    _xmlChSRange {
        low: 0xabc as c_ushort,
        high: 0xabc as c_ushort,
    },
    _xmlChSRange {
        low: 0xabe as c_ushort,
        high: 0xac5 as c_ushort,
    },
    _xmlChSRange {
        low: 0xac7 as c_ushort,
        high: 0xac9 as c_ushort,
    },
    _xmlChSRange {
        low: 0xacb as c_ushort,
        high: 0xacd as c_ushort,
    },
    _xmlChSRange {
        low: 0xb01 as c_ushort,
        high: 0xb03 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb3c as c_ushort,
        high: 0xb3c as c_ushort,
    },
    _xmlChSRange {
        low: 0xb3e as c_ushort,
        high: 0xb43 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb47 as c_ushort,
        high: 0xb48 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb4b as c_ushort,
        high: 0xb4d as c_ushort,
    },
    _xmlChSRange {
        low: 0xb56 as c_ushort,
        high: 0xb57 as c_ushort,
    },
    _xmlChSRange {
        low: 0xb82 as c_ushort,
        high: 0xb83 as c_ushort,
    },
    _xmlChSRange {
        low: 0xbbe as c_ushort,
        high: 0xbc2 as c_ushort,
    },
    _xmlChSRange {
        low: 0xbc6 as c_ushort,
        high: 0xbc8 as c_ushort,
    },
    _xmlChSRange {
        low: 0xbca as c_ushort,
        high: 0xbcd as c_ushort,
    },
    _xmlChSRange {
        low: 0xbd7 as c_ushort,
        high: 0xbd7 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc01 as c_ushort,
        high: 0xc03 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc3e as c_ushort,
        high: 0xc44 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc46 as c_ushort,
        high: 0xc48 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc4a as c_ushort,
        high: 0xc4d as c_ushort,
    },
    _xmlChSRange {
        low: 0xc55 as c_ushort,
        high: 0xc56 as c_ushort,
    },
    _xmlChSRange {
        low: 0xc82 as c_ushort,
        high: 0xc83 as c_ushort,
    },
    _xmlChSRange {
        low: 0xcbe as c_ushort,
        high: 0xcc4 as c_ushort,
    },
    _xmlChSRange {
        low: 0xcc6 as c_ushort,
        high: 0xcc8 as c_ushort,
    },
    _xmlChSRange {
        low: 0xcca as c_ushort,
        high: 0xccd as c_ushort,
    },
    _xmlChSRange {
        low: 0xcd5 as c_ushort,
        high: 0xcd6 as c_ushort,
    },
    _xmlChSRange {
        low: 0xd02 as c_ushort,
        high: 0xd03 as c_ushort,
    },
    _xmlChSRange {
        low: 0xd3e as c_ushort,
        high: 0xd43 as c_ushort,
    },
    _xmlChSRange {
        low: 0xd46 as c_ushort,
        high: 0xd48 as c_ushort,
    },
    _xmlChSRange {
        low: 0xd4a as c_ushort,
        high: 0xd4d as c_ushort,
    },
    _xmlChSRange {
        low: 0xd57 as c_ushort,
        high: 0xd57 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe31 as c_ushort,
        high: 0xe31 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe34 as c_ushort,
        high: 0xe3a as c_ushort,
    },
    _xmlChSRange {
        low: 0xe47 as c_ushort,
        high: 0xe4e as c_ushort,
    },
    _xmlChSRange {
        low: 0xeb1 as c_ushort,
        high: 0xeb1 as c_ushort,
    },
    _xmlChSRange {
        low: 0xeb4 as c_ushort,
        high: 0xeb9 as c_ushort,
    },
    _xmlChSRange {
        low: 0xebb as c_ushort,
        high: 0xebc as c_ushort,
    },
    _xmlChSRange {
        low: 0xec8 as c_ushort,
        high: 0xecd as c_ushort,
    },
    _xmlChSRange {
        low: 0xf18 as c_ushort,
        high: 0xf19 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf35 as c_ushort,
        high: 0xf35 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf37 as c_ushort,
        high: 0xf37 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf39 as c_ushort,
        high: 0xf39 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf3e as c_ushort,
        high: 0xf3e as c_ushort,
    },
    _xmlChSRange {
        low: 0xf3f as c_ushort,
        high: 0xf3f as c_ushort,
    },
    _xmlChSRange {
        low: 0xf71 as c_ushort,
        high: 0xf84 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf86 as c_ushort,
        high: 0xf8b as c_ushort,
    },
    _xmlChSRange {
        low: 0xf90 as c_ushort,
        high: 0xf95 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf97 as c_ushort,
        high: 0xf97 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf99 as c_ushort,
        high: 0xfad as c_ushort,
    },
    _xmlChSRange {
        low: 0xfb1 as c_ushort,
        high: 0xfb7 as c_ushort,
    },
    _xmlChSRange {
        low: 0xfb9 as c_ushort,
        high: 0xfb9 as c_ushort,
    },
    _xmlChSRange {
        low: 0x20d0 as c_ushort,
        high: 0x20dc as c_ushort,
    },
    _xmlChSRange {
        low: 0x20e1 as c_ushort,
        high: 0x20e1 as c_ushort,
    },
    _xmlChSRange {
        low: 0x302a as c_ushort,
        high: 0x302f as c_ushort,
    },
    _xmlChSRange {
        low: 0x3099 as c_ushort,
        high: 0x3099 as c_ushort,
    },
    _xmlChSRange {
        low: 0x309a as c_ushort,
        high: 0x309a as c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsCombiningGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 95 as c_int,
        nbLongRange: 0 as c_int,
        shortRange: &raw const xmlIsCombining_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
static mut xmlIsDigit_srng: [xmlChSRange; 14] = [
    _xmlChSRange {
        low: 0x660 as c_ushort,
        high: 0x669 as c_ushort,
    },
    _xmlChSRange {
        low: 0x6f0 as c_ushort,
        high: 0x6f9 as c_ushort,
    },
    _xmlChSRange {
        low: 0x966 as c_ushort,
        high: 0x96f as c_ushort,
    },
    _xmlChSRange {
        low: 0x9e6 as c_ushort,
        high: 0x9ef as c_ushort,
    },
    _xmlChSRange {
        low: 0xa66 as c_ushort,
        high: 0xa6f as c_ushort,
    },
    _xmlChSRange {
        low: 0xae6 as c_ushort,
        high: 0xaef as c_ushort,
    },
    _xmlChSRange {
        low: 0xb66 as c_ushort,
        high: 0xb6f as c_ushort,
    },
    _xmlChSRange {
        low: 0xbe7 as c_ushort,
        high: 0xbef as c_ushort,
    },
    _xmlChSRange {
        low: 0xc66 as c_ushort,
        high: 0xc6f as c_ushort,
    },
    _xmlChSRange {
        low: 0xce6 as c_ushort,
        high: 0xcef as c_ushort,
    },
    _xmlChSRange {
        low: 0xd66 as c_ushort,
        high: 0xd6f as c_ushort,
    },
    _xmlChSRange {
        low: 0xe50 as c_ushort,
        high: 0xe59 as c_ushort,
    },
    _xmlChSRange {
        low: 0xed0 as c_ushort,
        high: 0xed9 as c_ushort,
    },
    _xmlChSRange {
        low: 0xf20 as c_ushort,
        high: 0xf29 as c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsDigitGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 14 as c_int,
        nbLongRange: 0 as c_int,
        shortRange: &raw const xmlIsDigit_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
static mut xmlIsExtender_srng: [xmlChSRange; 10] = [
    _xmlChSRange {
        low: 0x2d0 as c_ushort,
        high: 0x2d0 as c_ushort,
    },
    _xmlChSRange {
        low: 0x2d1 as c_ushort,
        high: 0x2d1 as c_ushort,
    },
    _xmlChSRange {
        low: 0x387 as c_ushort,
        high: 0x387 as c_ushort,
    },
    _xmlChSRange {
        low: 0x640 as c_ushort,
        high: 0x640 as c_ushort,
    },
    _xmlChSRange {
        low: 0xe46 as c_ushort,
        high: 0xe46 as c_ushort,
    },
    _xmlChSRange {
        low: 0xec6 as c_ushort,
        high: 0xec6 as c_ushort,
    },
    _xmlChSRange {
        low: 0x3005 as c_ushort,
        high: 0x3005 as c_ushort,
    },
    _xmlChSRange {
        low: 0x3031 as c_ushort,
        high: 0x3035 as c_ushort,
    },
    _xmlChSRange {
        low: 0x309d as c_ushort,
        high: 0x309e as c_ushort,
    },
    _xmlChSRange {
        low: 0x30fc as c_ushort,
        high: 0x30fe as c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsExtenderGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 10 as c_int,
        nbLongRange: 0 as c_int,
        shortRange: &raw const xmlIsExtender_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
static mut xmlIsIdeographic_srng: [xmlChSRange; 3] = [
    _xmlChSRange {
        low: 0x3007 as c_ushort,
        high: 0x3007 as c_ushort,
    },
    _xmlChSRange {
        low: 0x3021 as c_ushort,
        high: 0x3029 as c_ushort,
    },
    _xmlChSRange {
        low: 0x4e00 as c_ushort,
        high: 0x9fa5 as c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsIdeographicGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 3 as c_int,
        nbLongRange: 0 as c_int,
        shortRange: &raw const xmlIsIdeographic_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
#[no_mangle]
pub unsafe extern "C" fn xmlCharInRange(
    mut val: c_uint,
    mut rptr: *const xmlChRangeGroup,
) -> c_int {
    let mut low: c_int = 0;
    let mut high: c_int = 0;
    let mut mid: c_int = 0;
    let mut sptr: *const xmlChSRange = ::core::ptr::null::<xmlChSRange>();
    let mut lptr: *const xmlChLRange = ::core::ptr::null::<xmlChLRange>();
    if rptr.is_null() {
        return 0 as c_int;
    }
    if val < 0x10000 as c_int as c_uint {
        if (*rptr).nbShortRange == 0 as c_int {
            return 0 as c_int;
        }
        low = 0 as c_int;
        high = (*rptr).nbShortRange - 1 as c_int;
        sptr = (*rptr).shortRange;
        while low <= high {
            mid = (low + high) / 2 as c_int;
            if (val as c_ushort as c_int)
                < (*sptr.offset(mid as isize)).low as c_int
            {
                high = mid - 1 as c_int;
            } else if val as c_ushort as c_int
                > (*sptr.offset(mid as isize)).high as c_int
            {
                low = mid + 1 as c_int;
            } else {
                return 1 as c_int;
            }
        }
    } else {
        if (*rptr).nbLongRange == 0 as c_int {
            return 0 as c_int;
        }
        low = 0 as c_int;
        high = (*rptr).nbLongRange - 1 as c_int;
        lptr = (*rptr).longRange;
        while low <= high {
            mid = (low + high) / 2 as c_int;
            if val < (*lptr.offset(mid as isize)).low {
                high = mid - 1 as c_int;
            } else if val > (*lptr.offset(mid as isize)).high {
                low = mid + 1 as c_int;
            } else {
                return 1 as c_int;
            }
        }
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsBaseChar(mut ch: c_uint) -> c_int {
    return if ch < 0x100 as c_uint {
        (0x41 as c_uint <= ch && ch <= 0x5a as c_uint
            || 0x61 as c_uint <= ch && ch <= 0x7a as c_uint
            || 0xc0 as c_uint <= ch && ch <= 0xd6 as c_uint
            || 0xd8 as c_uint <= ch && ch <= 0xf6 as c_uint
            || 0xf8 as c_uint <= ch) as c_int
    } else {
        xmlCharInRange(ch, &raw const xmlIsBaseCharGroup)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsBlank(mut ch: c_uint) -> c_int {
    return if ch < 0x100 as c_uint {
        (ch == 0x20 as c_uint
            || 0x9 as c_uint <= ch && ch <= 0xa as c_uint
            || ch == 0xd as c_uint) as c_int
    } else {
        0 as c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsChar(mut ch: c_uint) -> c_int {
    return if ch < 0x100 as c_uint {
        (0x9 as c_uint <= ch && ch <= 0xa as c_uint
            || ch == 0xd as c_uint
            || 0x20 as c_uint <= ch) as c_int
    } else {
        (0x100 as c_uint <= ch && ch <= 0xd7ff as c_uint
            || 0xe000 as c_uint <= ch && ch <= 0xfffd as c_uint
            || 0x10000 as c_int as c_uint <= ch
                && ch <= 0x10ffff as c_int as c_uint)
            as c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsCombining(mut ch: c_uint) -> c_int {
    return if ch < 0x100 as c_uint {
        0 as c_int
    } else {
        xmlCharInRange(ch, &raw const xmlIsCombiningGroup)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsDigit(mut ch: c_uint) -> c_int {
    return if ch < 0x100 as c_uint {
        (0x30 as c_uint <= ch && ch <= 0x39 as c_uint)
            as c_int
    } else {
        xmlCharInRange(ch, &raw const xmlIsDigitGroup)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsExtender(mut ch: c_uint) -> c_int {
    return if ch < 0x100 as c_uint {
        (ch == 0xb7 as c_uint) as c_int
    } else {
        xmlCharInRange(ch, &raw const xmlIsExtenderGroup)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsIdeographic(mut ch: c_uint) -> c_int {
    return if ch < 0x100 as c_uint {
        0 as c_int
    } else {
        (0x4e00 as c_uint <= ch && ch <= 0x9fa5 as c_uint
            || ch == 0x3007 as c_uint
            || 0x3021 as c_uint <= ch && ch <= 0x3029 as c_uint)
            as c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsPubidChar(mut ch: c_uint) -> c_int {
    return if ch < 0x100 as c_uint {
        xmlIsPubidChar_tab[ch as usize] as c_int
    } else {
        0 as c_int
    };
}
