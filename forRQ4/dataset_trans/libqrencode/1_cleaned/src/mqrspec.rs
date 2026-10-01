use core::ffi::*;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MQRspec_Capacity {
    pub width: c_int,
    pub ec: [c_int; 4],
}

static mut mqrspecCapacity: [MQRspec_Capacity; 5] = [
    MQRspec_Capacity {
        width: 0 as c_int,
        ec: [
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
        ],
    },
    MQRspec_Capacity {
        width: 11 as c_int,
        ec: [
            2 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
        ],
    },
    MQRspec_Capacity {
        width: 13 as c_int,
        ec: [
            5 as c_int,
            6 as c_int,
            0 as c_int,
            0 as c_int,
        ],
    },
    MQRspec_Capacity {
        width: 15 as c_int,
        ec: [
            6 as c_int,
            8 as c_int,
            0 as c_int,
            0 as c_int,
        ],
    },
    MQRspec_Capacity {
        width: 17 as c_int,
        ec: [
            8 as c_int,
            10 as c_int,
            14 as c_int,
            0 as c_int,
        ],
    },
];
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getDataLengthBit(
    mut version: c_int,
    mut level: QRecLevel,
) -> c_int {
    let mut w: c_int = 0;
    let mut ecc: c_int = 0;
    w = mqrspecCapacity[version as usize].width - 1 as c_int;
    ecc = mqrspecCapacity[version as usize].ec[level as usize];
    if ecc == 0 as c_int {
        return 0 as c_int;
    }
    return w * w - 64 as c_int - ecc * 8 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getDataLength(
    mut version: c_int,
    mut level: QRecLevel,
) -> c_int {
    return (MQRspec_getDataLengthBit(version, level) + 4 as c_int)
        / 8 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getECCLength(
    mut version: c_int,
    mut level: QRecLevel,
) -> c_int {
    return mqrspecCapacity[version as usize].ec[level as usize];
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getWidth(mut version: c_int) -> c_int {
    return mqrspecCapacity[version as usize].width;
}
static mut lengthTableBits: [[c_int; 4]; 4] = [
    [
        3 as c_int,
        4 as c_int,
        5 as c_int,
        6 as c_int,
    ],
    [
        0 as c_int,
        3 as c_int,
        4 as c_int,
        5 as c_int,
    ],
    [
        0 as c_int,
        0 as c_int,
        4 as c_int,
        5 as c_int,
    ],
    [
        0 as c_int,
        0 as c_int,
        3 as c_int,
        4 as c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn MQRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version: c_int,
) -> c_int {
    return lengthTableBits[mode as usize][(version - 1 as c_int) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version: c_int,
) -> c_int {
    let mut bits: c_int = 0;
    let mut words: c_int = 0;
    bits = lengthTableBits[mode as usize][(version - 1 as c_int) as usize];
    words = ((1 as c_int) << bits) - 1 as c_int;
    if mode as c_int == QR_MODE_KANJI as c_int {
        words *= 2 as c_int;
    }
    return words;
}
static mut formatInfo: [[c_uint; 8]; 4] = [
    [
        0x4445 as c_int as c_uint,
        0x55ae as c_int as c_uint,
        0x6793 as c_int as c_uint,
        0x7678 as c_int as c_uint,
        0x6de as c_int as c_uint,
        0x1735 as c_int as c_uint,
        0x2508 as c_int as c_uint,
        0x34e3 as c_int as c_uint,
    ],
    [
        0x4172 as c_int as c_uint,
        0x5099 as c_int as c_uint,
        0x62a4 as c_int as c_uint,
        0x734f as c_int as c_uint,
        0x3e9 as c_int as c_uint,
        0x1202 as c_int as c_uint,
        0x203f as c_int as c_uint,
        0x31d4 as c_int as c_uint,
    ],
    [
        0x4e2b as c_int as c_uint,
        0x5fc0 as c_int as c_uint,
        0x6dfd as c_int as c_uint,
        0x7c16 as c_int as c_uint,
        0xcb0 as c_int as c_uint,
        0x1d5b as c_int as c_uint,
        0x2f66 as c_int as c_uint,
        0x3e8d as c_int as c_uint,
    ],
    [
        0x4b1c as c_int as c_uint,
        0x5af7 as c_int as c_uint,
        0x68ca as c_int as c_uint,
        0x7921 as c_int as c_uint,
        0x987 as c_int as c_uint,
        0x186c as c_int as c_uint,
        0x2a51 as c_int as c_uint,
        0x3bba as c_int as c_uint,
    ],
];
static mut typeTable: [[c_int; 3]; 5] = [
    [
        -(1 as c_int),
        -(1 as c_int),
        -(1 as c_int),
    ],
    [
        0 as c_int,
        -(1 as c_int),
        -(1 as c_int),
    ],
    [
        1 as c_int,
        2 as c_int,
        -(1 as c_int),
    ],
    [
        3 as c_int,
        4 as c_int,
        -(1 as c_int),
    ],
    [
        5 as c_int,
        6 as c_int,
        7 as c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn MQRspec_getFormatInfo(
    mut mask: c_int,
    mut version: c_int,
    mut level: QRecLevel,
) -> c_uint {
    let mut type_0: c_int = 0;
    if mask < 0 as c_int || mask > 3 as c_int {
        return 0 as c_uint;
    }
    if version <= 0 as c_int || version > MQRSPEC_VERSION_MAX {
        return 0 as c_uint;
    }
    if level as c_uint == QR_ECLEVEL_H as c_int as c_uint {
        return 0 as c_uint;
    }
    type_0 = typeTable[version as usize][level as usize];
    if type_0 < 0 as c_int {
        return 0 as c_uint;
    }
    return formatInfo[mask as usize][type_0 as usize];
}

unsafe extern "C" fn MQRspec_createFrame(
    mut version: c_int,
) -> *mut c_uchar {
    let mut frame: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut q: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut width: c_int = 0;
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    width = mqrspecCapacity[version as usize].width;
    frame = malloc((width * width) as size_t) as *mut c_uchar;
    if frame.is_null() {
        return ::core::ptr::null_mut::<c_uchar>();
    }
    memset(
        frame as *mut c_void,
        0 as c_int,
        (width * width) as size_t,
    );
    putFinderPattern(
        frame,
        width,
        0 as c_int,
        0 as c_int,
    );
    p = frame;
    y = 0 as c_int;
    while y < 7 as c_int {
        *p.offset(7 as c_int as isize) = 0xc0 as c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    memset(
        frame.offset((width * 7 as c_int) as isize) as *mut c_void,
        0xc0 as c_int,
        8 as size_t,
    );
    memset(
        frame
            .offset((width * 8 as c_int) as isize)
            .offset(1 as c_int as isize) as *mut c_void,
        0x84 as c_int,
        8 as size_t,
    );
    p = frame
        .offset(width as isize)
        .offset(8 as c_int as isize);
    y = 0 as c_int;
    while y < 7 as c_int {
        *p = 0x84 as c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame.offset(8 as c_int as isize);
    q = frame.offset((width * 8 as c_int) as isize);
    x = 1 as c_int;
    while x < width - 7 as c_int {
        *p = (0x90 as c_int | x & 1 as c_int) as c_uchar;
        *q = (0x90 as c_int | x & 1 as c_int) as c_uchar;
        p = p.offset(1);
        q = q.offset(width as isize);
        x += 1;
    }
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn MQRspec_newFrame(
    mut version: c_int,
) -> *mut c_uchar {
    if version < 1 as c_int || version > MQRSPEC_VERSION_MAX {
        return ::core::ptr::null_mut::<c_uchar>();
    }
    return MQRspec_createFrame(version);
}
