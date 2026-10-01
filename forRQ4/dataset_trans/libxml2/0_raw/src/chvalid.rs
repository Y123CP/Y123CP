#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: ::core::ffi::c_ushort,
    pub high: ::core::ffi::c_ushort,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: ::core::ffi::c_uint,
    pub high: ::core::ffi::c_uint,
}
pub type xmlChLRange = _xmlChLRange;
pub type xmlChLRangePtr = *mut xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: ::core::ffi::c_int,
    pub nbLongRange: ::core::ffi::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChRangeGroup = _xmlChRangeGroup;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut xmlIsPubidChar_tab: [::core::ffi::c_uchar; 256] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
static mut xmlIsBaseChar_srng: [xmlChSRange; 197] = [
    _xmlChSRange {
        low: 0x100 as ::core::ffi::c_ushort,
        high: 0x131 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x134 as ::core::ffi::c_ushort,
        high: 0x13e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x141 as ::core::ffi::c_ushort,
        high: 0x148 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x14a as ::core::ffi::c_ushort,
        high: 0x17e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x180 as ::core::ffi::c_ushort,
        high: 0x1c3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1cd as ::core::ffi::c_ushort,
        high: 0x1f0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f4 as ::core::ffi::c_ushort,
        high: 0x1f5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fa as ::core::ffi::c_ushort,
        high: 0x217 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x250 as ::core::ffi::c_ushort,
        high: 0x2a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2bb as ::core::ffi::c_ushort,
        high: 0x2c1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x386 as ::core::ffi::c_ushort,
        high: 0x386 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x388 as ::core::ffi::c_ushort,
        high: 0x38a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x38c as ::core::ffi::c_ushort,
        high: 0x38c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x38e as ::core::ffi::c_ushort,
        high: 0x3a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3a3 as ::core::ffi::c_ushort,
        high: 0x3ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3d0 as ::core::ffi::c_ushort,
        high: 0x3d6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3da as ::core::ffi::c_ushort,
        high: 0x3da as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3dc as ::core::ffi::c_ushort,
        high: 0x3dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3de as ::core::ffi::c_ushort,
        high: 0x3de as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e0 as ::core::ffi::c_ushort,
        high: 0x3e0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e2 as ::core::ffi::c_ushort,
        high: 0x3f3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x401 as ::core::ffi::c_ushort,
        high: 0x40c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x40e as ::core::ffi::c_ushort,
        high: 0x44f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x451 as ::core::ffi::c_ushort,
        high: 0x45c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x45e as ::core::ffi::c_ushort,
        high: 0x481 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x490 as ::core::ffi::c_ushort,
        high: 0x4c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c7 as ::core::ffi::c_ushort,
        high: 0x4c8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4cb as ::core::ffi::c_ushort,
        high: 0x4cc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d0 as ::core::ffi::c_ushort,
        high: 0x4eb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ee as ::core::ffi::c_ushort,
        high: 0x4f5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f8 as ::core::ffi::c_ushort,
        high: 0x4f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x531 as ::core::ffi::c_ushort,
        high: 0x556 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x559 as ::core::ffi::c_ushort,
        high: 0x559 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x561 as ::core::ffi::c_ushort,
        high: 0x586 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5d0 as ::core::ffi::c_ushort,
        high: 0x5ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5f0 as ::core::ffi::c_ushort,
        high: 0x5f2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x621 as ::core::ffi::c_ushort,
        high: 0x63a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x641 as ::core::ffi::c_ushort,
        high: 0x64a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x671 as ::core::ffi::c_ushort,
        high: 0x6b7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6ba as ::core::ffi::c_ushort,
        high: 0x6be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6c0 as ::core::ffi::c_ushort,
        high: 0x6ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d0 as ::core::ffi::c_ushort,
        high: 0x6d3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d5 as ::core::ffi::c_ushort,
        high: 0x6d5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e5 as ::core::ffi::c_ushort,
        high: 0x6e6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x905 as ::core::ffi::c_ushort,
        high: 0x939 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93d as ::core::ffi::c_ushort,
        high: 0x93d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x958 as ::core::ffi::c_ushort,
        high: 0x961 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x985 as ::core::ffi::c_ushort,
        high: 0x98c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x98f as ::core::ffi::c_ushort,
        high: 0x990 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x993 as ::core::ffi::c_ushort,
        high: 0x9a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9aa as ::core::ffi::c_ushort,
        high: 0x9b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9b2 as ::core::ffi::c_ushort,
        high: 0x9b2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9b6 as ::core::ffi::c_ushort,
        high: 0x9b9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9dc as ::core::ffi::c_ushort,
        high: 0x9dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9df as ::core::ffi::c_ushort,
        high: 0x9e1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9f0 as ::core::ffi::c_ushort,
        high: 0x9f1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa05 as ::core::ffi::c_ushort,
        high: 0xa0a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa0f as ::core::ffi::c_ushort,
        high: 0xa10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa13 as ::core::ffi::c_ushort,
        high: 0xa28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa2a as ::core::ffi::c_ushort,
        high: 0xa30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa32 as ::core::ffi::c_ushort,
        high: 0xa33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa35 as ::core::ffi::c_ushort,
        high: 0xa36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa38 as ::core::ffi::c_ushort,
        high: 0xa39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa59 as ::core::ffi::c_ushort,
        high: 0xa5c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa5e as ::core::ffi::c_ushort,
        high: 0xa5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa72 as ::core::ffi::c_ushort,
        high: 0xa74 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa85 as ::core::ffi::c_ushort,
        high: 0xa8b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa8d as ::core::ffi::c_ushort,
        high: 0xa8d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa8f as ::core::ffi::c_ushort,
        high: 0xa91 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa93 as ::core::ffi::c_ushort,
        high: 0xaa8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xaaa as ::core::ffi::c_ushort,
        high: 0xab0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xab2 as ::core::ffi::c_ushort,
        high: 0xab3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xab5 as ::core::ffi::c_ushort,
        high: 0xab9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabd as ::core::ffi::c_ushort,
        high: 0xabd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae0 as ::core::ffi::c_ushort,
        high: 0xae0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb05 as ::core::ffi::c_ushort,
        high: 0xb0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb0f as ::core::ffi::c_ushort,
        high: 0xb10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb13 as ::core::ffi::c_ushort,
        high: 0xb28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb2a as ::core::ffi::c_ushort,
        high: 0xb30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb32 as ::core::ffi::c_ushort,
        high: 0xb33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb36 as ::core::ffi::c_ushort,
        high: 0xb39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3d as ::core::ffi::c_ushort,
        high: 0xb3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb5c as ::core::ffi::c_ushort,
        high: 0xb5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb5f as ::core::ffi::c_ushort,
        high: 0xb61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb85 as ::core::ffi::c_ushort,
        high: 0xb8a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb8e as ::core::ffi::c_ushort,
        high: 0xb90 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb92 as ::core::ffi::c_ushort,
        high: 0xb95 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb99 as ::core::ffi::c_ushort,
        high: 0xb9a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb9c as ::core::ffi::c_ushort,
        high: 0xb9c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb9e as ::core::ffi::c_ushort,
        high: 0xb9f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xba3 as ::core::ffi::c_ushort,
        high: 0xba4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xba8 as ::core::ffi::c_ushort,
        high: 0xbaa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbae as ::core::ffi::c_ushort,
        high: 0xbb5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbb7 as ::core::ffi::c_ushort,
        high: 0xbb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc05 as ::core::ffi::c_ushort,
        high: 0xc0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc0e as ::core::ffi::c_ushort,
        high: 0xc10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc12 as ::core::ffi::c_ushort,
        high: 0xc28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc2a as ::core::ffi::c_ushort,
        high: 0xc33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc35 as ::core::ffi::c_ushort,
        high: 0xc39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc60 as ::core::ffi::c_ushort,
        high: 0xc61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc85 as ::core::ffi::c_ushort,
        high: 0xc8c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc8e as ::core::ffi::c_ushort,
        high: 0xc90 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc92 as ::core::ffi::c_ushort,
        high: 0xca8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcaa as ::core::ffi::c_ushort,
        high: 0xcb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcb5 as ::core::ffi::c_ushort,
        high: 0xcb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcde as ::core::ffi::c_ushort,
        high: 0xcde as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xce0 as ::core::ffi::c_ushort,
        high: 0xce1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd05 as ::core::ffi::c_ushort,
        high: 0xd0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd0e as ::core::ffi::c_ushort,
        high: 0xd10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd12 as ::core::ffi::c_ushort,
        high: 0xd28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd2a as ::core::ffi::c_ushort,
        high: 0xd39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd60 as ::core::ffi::c_ushort,
        high: 0xd61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe01 as ::core::ffi::c_ushort,
        high: 0xe2e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe30 as ::core::ffi::c_ushort,
        high: 0xe30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe32 as ::core::ffi::c_ushort,
        high: 0xe33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe40 as ::core::ffi::c_ushort,
        high: 0xe45 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe81 as ::core::ffi::c_ushort,
        high: 0xe82 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe84 as ::core::ffi::c_ushort,
        high: 0xe84 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe87 as ::core::ffi::c_ushort,
        high: 0xe88 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe8a as ::core::ffi::c_ushort,
        high: 0xe8a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe8d as ::core::ffi::c_ushort,
        high: 0xe8d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe94 as ::core::ffi::c_ushort,
        high: 0xe97 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe99 as ::core::ffi::c_ushort,
        high: 0xe9f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea1 as ::core::ffi::c_ushort,
        high: 0xea3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea5 as ::core::ffi::c_ushort,
        high: 0xea5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea7 as ::core::ffi::c_ushort,
        high: 0xea7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeaa as ::core::ffi::c_ushort,
        high: 0xeab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xead as ::core::ffi::c_ushort,
        high: 0xeae as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb0 as ::core::ffi::c_ushort,
        high: 0xeb0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb2 as ::core::ffi::c_ushort,
        high: 0xeb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xebd as ::core::ffi::c_ushort,
        high: 0xebd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec0 as ::core::ffi::c_ushort,
        high: 0xec4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf40 as ::core::ffi::c_ushort,
        high: 0xf47 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf49 as ::core::ffi::c_ushort,
        high: 0xf69 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10a0 as ::core::ffi::c_ushort,
        high: 0x10c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10d0 as ::core::ffi::c_ushort,
        high: 0x10f6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1100 as ::core::ffi::c_ushort,
        high: 0x1100 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1102 as ::core::ffi::c_ushort,
        high: 0x1103 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1105 as ::core::ffi::c_ushort,
        high: 0x1107 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1109 as ::core::ffi::c_ushort,
        high: 0x1109 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x110b as ::core::ffi::c_ushort,
        high: 0x110c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x110e as ::core::ffi::c_ushort,
        high: 0x1112 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x113c as ::core::ffi::c_ushort,
        high: 0x113c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x113e as ::core::ffi::c_ushort,
        high: 0x113e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1140 as ::core::ffi::c_ushort,
        high: 0x1140 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x114c as ::core::ffi::c_ushort,
        high: 0x114c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x114e as ::core::ffi::c_ushort,
        high: 0x114e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1150 as ::core::ffi::c_ushort,
        high: 0x1150 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1154 as ::core::ffi::c_ushort,
        high: 0x1155 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1159 as ::core::ffi::c_ushort,
        high: 0x1159 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x115f as ::core::ffi::c_ushort,
        high: 0x1161 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1163 as ::core::ffi::c_ushort,
        high: 0x1163 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1165 as ::core::ffi::c_ushort,
        high: 0x1165 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1167 as ::core::ffi::c_ushort,
        high: 0x1167 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1169 as ::core::ffi::c_ushort,
        high: 0x1169 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x116d as ::core::ffi::c_ushort,
        high: 0x116e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1172 as ::core::ffi::c_ushort,
        high: 0x1173 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1175 as ::core::ffi::c_ushort,
        high: 0x1175 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x119e as ::core::ffi::c_ushort,
        high: 0x119e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11a8 as ::core::ffi::c_ushort,
        high: 0x11a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11ab as ::core::ffi::c_ushort,
        high: 0x11ab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11ae as ::core::ffi::c_ushort,
        high: 0x11af as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11b7 as ::core::ffi::c_ushort,
        high: 0x11b8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11ba as ::core::ffi::c_ushort,
        high: 0x11ba as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11bc as ::core::ffi::c_ushort,
        high: 0x11c2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11eb as ::core::ffi::c_ushort,
        high: 0x11eb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11f0 as ::core::ffi::c_ushort,
        high: 0x11f0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11f9 as ::core::ffi::c_ushort,
        high: 0x11f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e00 as ::core::ffi::c_ushort,
        high: 0x1e9b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea0 as ::core::ffi::c_ushort,
        high: 0x1ef9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f00 as ::core::ffi::c_ushort,
        high: 0x1f15 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f18 as ::core::ffi::c_ushort,
        high: 0x1f1d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f20 as ::core::ffi::c_ushort,
        high: 0x1f45 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f48 as ::core::ffi::c_ushort,
        high: 0x1f4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f50 as ::core::ffi::c_ushort,
        high: 0x1f57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f59 as ::core::ffi::c_ushort,
        high: 0x1f59 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5b as ::core::ffi::c_ushort,
        high: 0x1f5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5d as ::core::ffi::c_ushort,
        high: 0x1f5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5f as ::core::ffi::c_ushort,
        high: 0x1f7d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f80 as ::core::ffi::c_ushort,
        high: 0x1fb4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fb6 as ::core::ffi::c_ushort,
        high: 0x1fbc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbe as ::core::ffi::c_ushort,
        high: 0x1fbe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc2 as ::core::ffi::c_ushort,
        high: 0x1fc4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc6 as ::core::ffi::c_ushort,
        high: 0x1fcc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd0 as ::core::ffi::c_ushort,
        high: 0x1fd3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd6 as ::core::ffi::c_ushort,
        high: 0x1fdb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fe0 as ::core::ffi::c_ushort,
        high: 0x1fec as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff2 as ::core::ffi::c_ushort,
        high: 0x1ff4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff6 as ::core::ffi::c_ushort,
        high: 0x1ffc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2126 as ::core::ffi::c_ushort,
        high: 0x2126 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212a as ::core::ffi::c_ushort,
        high: 0x212b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212e as ::core::ffi::c_ushort,
        high: 0x212e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2180 as ::core::ffi::c_ushort,
        high: 0x2182 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3041 as ::core::ffi::c_ushort,
        high: 0x3094 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30a1 as ::core::ffi::c_ushort,
        high: 0x30fa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3105 as ::core::ffi::c_ushort,
        high: 0x312c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac00 as ::core::ffi::c_ushort,
        high: 0xd7a3 as ::core::ffi::c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsBaseCharGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 197 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlIsBaseChar_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
static mut xmlIsChar_srng: [xmlChSRange; 2] = [
    _xmlChSRange {
        low: 0x100 as ::core::ffi::c_ushort,
        high: 0xd7ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe000 as ::core::ffi::c_ushort,
        high: 0xfffd as ::core::ffi::c_ushort,
    },
];
static mut xmlIsChar_lrng: [xmlChLRange; 1] = [_xmlChLRange {
    low: 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    high: 0x10ffff as ::core::ffi::c_int as ::core::ffi::c_uint,
}];
#[no_mangle]
pub static mut xmlIsCharGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 2 as ::core::ffi::c_int,
        nbLongRange: 1 as ::core::ffi::c_int,
        shortRange: &raw const xmlIsChar_srng as *const xmlChSRange,
        longRange: &raw const xmlIsChar_lrng as *const xmlChLRange,
    }
};
static mut xmlIsCombining_srng: [xmlChSRange; 95] = [
    _xmlChSRange {
        low: 0x300 as ::core::ffi::c_ushort,
        high: 0x345 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x360 as ::core::ffi::c_ushort,
        high: 0x361 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x483 as ::core::ffi::c_ushort,
        high: 0x486 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x591 as ::core::ffi::c_ushort,
        high: 0x5a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5a3 as ::core::ffi::c_ushort,
        high: 0x5b9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5bb as ::core::ffi::c_ushort,
        high: 0x5bd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5bf as ::core::ffi::c_ushort,
        high: 0x5bf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c1 as ::core::ffi::c_ushort,
        high: 0x5c2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c4 as ::core::ffi::c_ushort,
        high: 0x5c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x64b as ::core::ffi::c_ushort,
        high: 0x652 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x670 as ::core::ffi::c_ushort,
        high: 0x670 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d6 as ::core::ffi::c_ushort,
        high: 0x6dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6dd as ::core::ffi::c_ushort,
        high: 0x6df as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e0 as ::core::ffi::c_ushort,
        high: 0x6e4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e7 as ::core::ffi::c_ushort,
        high: 0x6e8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6ea as ::core::ffi::c_ushort,
        high: 0x6ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x901 as ::core::ffi::c_ushort,
        high: 0x903 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93c as ::core::ffi::c_ushort,
        high: 0x93c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93e as ::core::ffi::c_ushort,
        high: 0x94c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x94d as ::core::ffi::c_ushort,
        high: 0x94d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x951 as ::core::ffi::c_ushort,
        high: 0x954 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x962 as ::core::ffi::c_ushort,
        high: 0x963 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x981 as ::core::ffi::c_ushort,
        high: 0x983 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9bc as ::core::ffi::c_ushort,
        high: 0x9bc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9be as ::core::ffi::c_ushort,
        high: 0x9be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9bf as ::core::ffi::c_ushort,
        high: 0x9bf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9c0 as ::core::ffi::c_ushort,
        high: 0x9c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9c7 as ::core::ffi::c_ushort,
        high: 0x9c8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9cb as ::core::ffi::c_ushort,
        high: 0x9cd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9d7 as ::core::ffi::c_ushort,
        high: 0x9d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9e2 as ::core::ffi::c_ushort,
        high: 0x9e3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa02 as ::core::ffi::c_ushort,
        high: 0xa02 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa3c as ::core::ffi::c_ushort,
        high: 0xa3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa3e as ::core::ffi::c_ushort,
        high: 0xa3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa3f as ::core::ffi::c_ushort,
        high: 0xa3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa40 as ::core::ffi::c_ushort,
        high: 0xa42 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa47 as ::core::ffi::c_ushort,
        high: 0xa48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa4b as ::core::ffi::c_ushort,
        high: 0xa4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa70 as ::core::ffi::c_ushort,
        high: 0xa71 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa81 as ::core::ffi::c_ushort,
        high: 0xa83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabc as ::core::ffi::c_ushort,
        high: 0xabc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabe as ::core::ffi::c_ushort,
        high: 0xac5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac7 as ::core::ffi::c_ushort,
        high: 0xac9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xacb as ::core::ffi::c_ushort,
        high: 0xacd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb01 as ::core::ffi::c_ushort,
        high: 0xb03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3c as ::core::ffi::c_ushort,
        high: 0xb3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3e as ::core::ffi::c_ushort,
        high: 0xb43 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb47 as ::core::ffi::c_ushort,
        high: 0xb48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb4b as ::core::ffi::c_ushort,
        high: 0xb4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb56 as ::core::ffi::c_ushort,
        high: 0xb57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb82 as ::core::ffi::c_ushort,
        high: 0xb83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbbe as ::core::ffi::c_ushort,
        high: 0xbc2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbc6 as ::core::ffi::c_ushort,
        high: 0xbc8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbca as ::core::ffi::c_ushort,
        high: 0xbcd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbd7 as ::core::ffi::c_ushort,
        high: 0xbd7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc01 as ::core::ffi::c_ushort,
        high: 0xc03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc3e as ::core::ffi::c_ushort,
        high: 0xc44 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc46 as ::core::ffi::c_ushort,
        high: 0xc48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc4a as ::core::ffi::c_ushort,
        high: 0xc4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc55 as ::core::ffi::c_ushort,
        high: 0xc56 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc82 as ::core::ffi::c_ushort,
        high: 0xc83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcbe as ::core::ffi::c_ushort,
        high: 0xcc4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcc6 as ::core::ffi::c_ushort,
        high: 0xcc8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcca as ::core::ffi::c_ushort,
        high: 0xccd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcd5 as ::core::ffi::c_ushort,
        high: 0xcd6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd02 as ::core::ffi::c_ushort,
        high: 0xd03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd3e as ::core::ffi::c_ushort,
        high: 0xd43 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd46 as ::core::ffi::c_ushort,
        high: 0xd48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd4a as ::core::ffi::c_ushort,
        high: 0xd4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd57 as ::core::ffi::c_ushort,
        high: 0xd57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe31 as ::core::ffi::c_ushort,
        high: 0xe31 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe34 as ::core::ffi::c_ushort,
        high: 0xe3a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe47 as ::core::ffi::c_ushort,
        high: 0xe4e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb1 as ::core::ffi::c_ushort,
        high: 0xeb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb4 as ::core::ffi::c_ushort,
        high: 0xeb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xebb as ::core::ffi::c_ushort,
        high: 0xebc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec8 as ::core::ffi::c_ushort,
        high: 0xecd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf18 as ::core::ffi::c_ushort,
        high: 0xf19 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf35 as ::core::ffi::c_ushort,
        high: 0xf35 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf37 as ::core::ffi::c_ushort,
        high: 0xf37 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf39 as ::core::ffi::c_ushort,
        high: 0xf39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3e as ::core::ffi::c_ushort,
        high: 0xf3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3f as ::core::ffi::c_ushort,
        high: 0xf3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf71 as ::core::ffi::c_ushort,
        high: 0xf84 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf86 as ::core::ffi::c_ushort,
        high: 0xf8b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf90 as ::core::ffi::c_ushort,
        high: 0xf95 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf97 as ::core::ffi::c_ushort,
        high: 0xf97 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf99 as ::core::ffi::c_ushort,
        high: 0xfad as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb1 as ::core::ffi::c_ushort,
        high: 0xfb7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb9 as ::core::ffi::c_ushort,
        high: 0xfb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20d0 as ::core::ffi::c_ushort,
        high: 0x20dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20e1 as ::core::ffi::c_ushort,
        high: 0x20e1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x302a as ::core::ffi::c_ushort,
        high: 0x302f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3099 as ::core::ffi::c_ushort,
        high: 0x3099 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x309a as ::core::ffi::c_ushort,
        high: 0x309a as ::core::ffi::c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsCombiningGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 95 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlIsCombining_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
static mut xmlIsDigit_srng: [xmlChSRange; 14] = [
    _xmlChSRange {
        low: 0x660 as ::core::ffi::c_ushort,
        high: 0x669 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6f0 as ::core::ffi::c_ushort,
        high: 0x6f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x966 as ::core::ffi::c_ushort,
        high: 0x96f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9e6 as ::core::ffi::c_ushort,
        high: 0x9ef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa66 as ::core::ffi::c_ushort,
        high: 0xa6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae6 as ::core::ffi::c_ushort,
        high: 0xaef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb66 as ::core::ffi::c_ushort,
        high: 0xb6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbe7 as ::core::ffi::c_ushort,
        high: 0xbef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc66 as ::core::ffi::c_ushort,
        high: 0xc6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xce6 as ::core::ffi::c_ushort,
        high: 0xcef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd66 as ::core::ffi::c_ushort,
        high: 0xd6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe50 as ::core::ffi::c_ushort,
        high: 0xe59 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xed0 as ::core::ffi::c_ushort,
        high: 0xed9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf20 as ::core::ffi::c_ushort,
        high: 0xf29 as ::core::ffi::c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsDigitGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 14 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlIsDigit_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
static mut xmlIsExtender_srng: [xmlChSRange; 10] = [
    _xmlChSRange {
        low: 0x2d0 as ::core::ffi::c_ushort,
        high: 0x2d0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2d1 as ::core::ffi::c_ushort,
        high: 0x2d1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x387 as ::core::ffi::c_ushort,
        high: 0x387 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x640 as ::core::ffi::c_ushort,
        high: 0x640 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe46 as ::core::ffi::c_ushort,
        high: 0xe46 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec6 as ::core::ffi::c_ushort,
        high: 0xec6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3005 as ::core::ffi::c_ushort,
        high: 0x3005 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3031 as ::core::ffi::c_ushort,
        high: 0x3035 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x309d as ::core::ffi::c_ushort,
        high: 0x309e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30fc as ::core::ffi::c_ushort,
        high: 0x30fe as ::core::ffi::c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsExtenderGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 10 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlIsExtender_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
static mut xmlIsIdeographic_srng: [xmlChSRange; 3] = [
    _xmlChSRange {
        low: 0x3007 as ::core::ffi::c_ushort,
        high: 0x3007 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3021 as ::core::ffi::c_ushort,
        high: 0x3029 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e00 as ::core::ffi::c_ushort,
        high: 0x9fa5 as ::core::ffi::c_ushort,
    },
];
#[no_mangle]
pub static mut xmlIsIdeographicGroup: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 3 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlIsIdeographic_srng as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>() as *mut xmlChLRange as *const xmlChLRange,
    }
};
#[no_mangle]
pub unsafe extern "C" fn xmlCharInRange(
    mut val: ::core::ffi::c_uint,
    mut rptr: *const xmlChRangeGroup,
) -> ::core::ffi::c_int {
    let mut low: ::core::ffi::c_int = 0;
    let mut high: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut sptr: *const xmlChSRange = ::core::ptr::null::<xmlChSRange>();
    let mut lptr: *const xmlChLRange = ::core::ptr::null::<xmlChLRange>();
    if rptr.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if val < 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint {
        if (*rptr).nbShortRange == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        low = 0 as ::core::ffi::c_int;
        high = (*rptr).nbShortRange - 1 as ::core::ffi::c_int;
        sptr = (*rptr).shortRange;
        while low <= high {
            mid = (low + high) / 2 as ::core::ffi::c_int;
            if (val as ::core::ffi::c_ushort as ::core::ffi::c_int)
                < (*sptr.offset(mid as isize)).low as ::core::ffi::c_int
            {
                high = mid - 1 as ::core::ffi::c_int;
            } else if val as ::core::ffi::c_ushort as ::core::ffi::c_int
                > (*sptr.offset(mid as isize)).high as ::core::ffi::c_int
            {
                low = mid + 1 as ::core::ffi::c_int;
            } else {
                return 1 as ::core::ffi::c_int;
            }
        }
    } else {
        if (*rptr).nbLongRange == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        low = 0 as ::core::ffi::c_int;
        high = (*rptr).nbLongRange - 1 as ::core::ffi::c_int;
        lptr = (*rptr).longRange;
        while low <= high {
            mid = (low + high) / 2 as ::core::ffi::c_int;
            if val < (*lptr.offset(mid as isize)).low {
                high = mid - 1 as ::core::ffi::c_int;
            } else if val > (*lptr.offset(mid as isize)).high {
                low = mid + 1 as ::core::ffi::c_int;
            } else {
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsBaseChar(mut ch: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    return if ch < 0x100 as ::core::ffi::c_uint {
        (0x41 as ::core::ffi::c_uint <= ch && ch <= 0x5a as ::core::ffi::c_uint
            || 0x61 as ::core::ffi::c_uint <= ch && ch <= 0x7a as ::core::ffi::c_uint
            || 0xc0 as ::core::ffi::c_uint <= ch && ch <= 0xd6 as ::core::ffi::c_uint
            || 0xd8 as ::core::ffi::c_uint <= ch && ch <= 0xf6 as ::core::ffi::c_uint
            || 0xf8 as ::core::ffi::c_uint <= ch) as ::core::ffi::c_int
    } else {
        xmlCharInRange(ch, &raw const xmlIsBaseCharGroup)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsBlank(mut ch: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    return if ch < 0x100 as ::core::ffi::c_uint {
        (ch == 0x20 as ::core::ffi::c_uint
            || 0x9 as ::core::ffi::c_uint <= ch && ch <= 0xa as ::core::ffi::c_uint
            || ch == 0xd as ::core::ffi::c_uint) as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsChar(mut ch: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    return if ch < 0x100 as ::core::ffi::c_uint {
        (0x9 as ::core::ffi::c_uint <= ch && ch <= 0xa as ::core::ffi::c_uint
            || ch == 0xd as ::core::ffi::c_uint
            || 0x20 as ::core::ffi::c_uint <= ch) as ::core::ffi::c_int
    } else {
        (0x100 as ::core::ffi::c_uint <= ch && ch <= 0xd7ff as ::core::ffi::c_uint
            || 0xe000 as ::core::ffi::c_uint <= ch && ch <= 0xfffd as ::core::ffi::c_uint
            || 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint <= ch
                && ch <= 0x10ffff as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsCombining(mut ch: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    return if ch < 0x100 as ::core::ffi::c_uint {
        0 as ::core::ffi::c_int
    } else {
        xmlCharInRange(ch, &raw const xmlIsCombiningGroup)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsDigit(mut ch: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    return if ch < 0x100 as ::core::ffi::c_uint {
        (0x30 as ::core::ffi::c_uint <= ch && ch <= 0x39 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    } else {
        xmlCharInRange(ch, &raw const xmlIsDigitGroup)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsExtender(mut ch: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    return if ch < 0x100 as ::core::ffi::c_uint {
        (ch == 0xb7 as ::core::ffi::c_uint) as ::core::ffi::c_int
    } else {
        xmlCharInRange(ch, &raw const xmlIsExtenderGroup)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsIdeographic(mut ch: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    return if ch < 0x100 as ::core::ffi::c_uint {
        0 as ::core::ffi::c_int
    } else {
        (0x4e00 as ::core::ffi::c_uint <= ch && ch <= 0x9fa5 as ::core::ffi::c_uint
            || ch == 0x3007 as ::core::ffi::c_uint
            || 0x3021 as ::core::ffi::c_uint <= ch && ch <= 0x3029 as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsPubidChar(mut ch: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    return if ch < 0x100 as ::core::ffi::c_uint {
        xmlIsPubidChar_tab[ch as usize] as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
