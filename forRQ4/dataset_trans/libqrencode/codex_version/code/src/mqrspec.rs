extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type QRencodeMode = ::core::ffi::c_int;
pub const QR_MODE_FNC1SECOND: QRencodeMode = 7;
pub const QR_MODE_FNC1FIRST: QRencodeMode = 6;
pub const QR_MODE_ECI: QRencodeMode = 5;
pub const QR_MODE_STRUCTURE: QRencodeMode = 4;
pub const QR_MODE_KANJI: QRencodeMode = 3;
pub const QR_MODE_8: QRencodeMode = 2;
pub const QR_MODE_AN: QRencodeMode = 1;
pub const QR_MODE_NUM: QRencodeMode = 0;
pub const QR_MODE_NUL: QRencodeMode = -1;
pub type QRecLevel = ::core::ffi::c_uint;
pub const QR_ECLEVEL_H: QRecLevel = 3;
pub const QR_ECLEVEL_Q: QRecLevel = 2;
pub const QR_ECLEVEL_M: QRecLevel = 1;
pub const QR_ECLEVEL_L: QRecLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MQRspec_Capacity {
    pub width: ::core::ffi::c_int,
    pub ec: [::core::ffi::c_int; 4],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MQRSPEC_VERSION_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut mqrspecCapacity: [MQRspec_Capacity; 5] = [
    MQRspec_Capacity {
        width: 0 as ::core::ffi::c_int,
        ec: [
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
    MQRspec_Capacity {
        width: 11 as ::core::ffi::c_int,
        ec: [
            2 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
    MQRspec_Capacity {
        width: 13 as ::core::ffi::c_int,
        ec: [
            5 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
    MQRspec_Capacity {
        width: 15 as ::core::ffi::c_int,
        ec: [
            6 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
    MQRspec_Capacity {
        width: 17 as ::core::ffi::c_int,
        ec: [
            8 as ::core::ffi::c_int,
            10 as ::core::ffi::c_int,
            14 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
];
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getDataLengthBit(
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_int {
    let mut w: ::core::ffi::c_int = 0;
    let mut ecc: ::core::ffi::c_int = 0;
    w = mqrspecCapacity[version as usize].width - 1 as ::core::ffi::c_int;
    ecc = mqrspecCapacity[version as usize].ec[level as usize];
    if ecc == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return w * w - 64 as ::core::ffi::c_int - ecc * 8 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getDataLength(
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_int {
    return (MQRspec_getDataLengthBit(version, level) + 4 as ::core::ffi::c_int)
        / 8 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getECCLength(
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_int {
    return mqrspecCapacity[version as usize].ec[level as usize];
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getWidth(mut version: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return mqrspecCapacity[version as usize].width;
}
static mut lengthTableBits: [[::core::ffi::c_int; 4]; 4] = [
    [
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
    ],
    [
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
    ],
    [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
    ],
    [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn MQRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return lengthTableBits[mode as usize][(version - 1 as ::core::ffi::c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bits: ::core::ffi::c_int = 0;
    let mut words: ::core::ffi::c_int = 0;
    bits = lengthTableBits[mode as usize][(version - 1 as ::core::ffi::c_int) as usize];
    words = ((1 as ::core::ffi::c_int) << bits) - 1 as ::core::ffi::c_int;
    if mode as ::core::ffi::c_int == QR_MODE_KANJI as ::core::ffi::c_int {
        words *= 2 as ::core::ffi::c_int;
    }
    return words;
}
static mut formatInfo: [[::core::ffi::c_uint; 8]; 4] = [
    [
        0x4445 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x55ae as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x6793 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7678 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x6de as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1735 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2508 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x34e3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    ],
    [
        0x4172 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x5099 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x62a4 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x734f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3e9 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1202 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x203f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x31d4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    ],
    [
        0x4e2b as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x5fc0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x6dfd as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7c16 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xcb0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1d5b as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2f66 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3e8d as ::core::ffi::c_int as ::core::ffi::c_uint,
    ],
    [
        0x4b1c as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x5af7 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x68ca as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7921 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x987 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x186c as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2a51 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3bba as ::core::ffi::c_int as ::core::ffi::c_uint,
    ],
];
static mut typeTable: [[::core::ffi::c_int; 3]; 5] = [
    [
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
    ],
    [
        0 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
        -(1 as ::core::ffi::c_int),
    ],
    [
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
    ],
    [
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        -(1 as ::core::ffi::c_int),
    ],
    [
        5 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
        7 as ::core::ffi::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getFormatInfo(
    mut mask: ::core::ffi::c_int,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_uint {
    let mut type_0: ::core::ffi::c_int = 0;
    if mask < 0 as ::core::ffi::c_int || mask > 3 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_uint;
    }
    if version <= 0 as ::core::ffi::c_int || version > MQRSPEC_VERSION_MAX {
        return 0 as ::core::ffi::c_uint;
    }
    if level as ::core::ffi::c_uint == QR_ECLEVEL_H as ::core::ffi::c_int as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_uint;
    }
    type_0 = typeTable[version as usize][level as usize];
    if type_0 < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_uint;
    }
    return formatInfo[mask as usize][type_0 as usize];
}
unsafe extern "C" fn putFinderPattern(
    mut frame: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut ox: ::core::ffi::c_int,
    mut oy: ::core::ffi::c_int,
) {
    static mut finder: [::core::ffi::c_uchar; 49] = [
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut s: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    frame = frame.offset((oy * width + ox) as isize);
    s = &raw const finder as *const ::core::ffi::c_uchar;
    y = 0 as ::core::ffi::c_int;
    while y < 7 as ::core::ffi::c_int {
        x = 0 as ::core::ffi::c_int;
        while x < 7 as ::core::ffi::c_int {
            *frame.offset(x as isize) = *s.offset(x as isize);
            x += 1;
        }
        frame = frame.offset(width as isize);
        s = s.offset(7 as ::core::ffi::c_int as isize);
        y += 1;
    }
}
unsafe extern "C" fn MQRspec_createFrame(
    mut version: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut frame: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut q: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut width: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    width = mqrspecCapacity[version as usize].width;
    frame = malloc((width * width) as size_t) as *mut ::core::ffi::c_uchar;
    if frame.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    memset(
        frame as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (width * width) as size_t,
    );
    putFinderPattern(
        frame,
        width,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    p = frame;
    y = 0 as ::core::ffi::c_int;
    while y < 7 as ::core::ffi::c_int {
        *p.offset(7 as ::core::ffi::c_int as isize) = 0xc0 as ::core::ffi::c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    memset(
        frame.offset((width * 7 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_void,
        0xc0 as ::core::ffi::c_int,
        8 as size_t,
    );
    memset(
        frame
            .offset((width * 8 as ::core::ffi::c_int) as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        0x84 as ::core::ffi::c_int,
        8 as size_t,
    );
    p = frame
        .offset(width as isize)
        .offset(8 as ::core::ffi::c_int as isize);
    y = 0 as ::core::ffi::c_int;
    while y < 7 as ::core::ffi::c_int {
        *p = 0x84 as ::core::ffi::c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame.offset(8 as ::core::ffi::c_int as isize);
    q = frame.offset((width * 8 as ::core::ffi::c_int) as isize);
    x = 1 as ::core::ffi::c_int;
    while x < width - 7 as ::core::ffi::c_int {
        *p = (0x90 as ::core::ffi::c_int | x & 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *q = (0x90 as ::core::ffi::c_int | x & 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        p = p.offset(1);
        q = q.offset(width as isize);
        x += 1;
    }
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_newFrame(
    mut version: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    if version < 1 as ::core::ffi::c_int || version > MQRSPEC_VERSION_MAX {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return MQRspec_createFrame(version);
}
