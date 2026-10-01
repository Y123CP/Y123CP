extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn QRinput_isSplittableMode(mode: QRencodeMode) -> ::core::ffi::c_int;
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
pub struct QRspec_Capacity {
    pub width: ::core::ffi::c_int,
    pub words: ::core::ffi::c_int,
    pub remainder: ::core::ffi::c_int,
    pub ec: [::core::ffi::c_int; 4],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const QRSPEC_VERSION_MAX: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
static mut qrspecCapacity: [QRspec_Capacity; 41] = [
    QRspec_Capacity {
        width: 0 as ::core::ffi::c_int,
        words: 0 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 21 as ::core::ffi::c_int,
        words: 26 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            7 as ::core::ffi::c_int,
            10 as ::core::ffi::c_int,
            13 as ::core::ffi::c_int,
            17 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 25 as ::core::ffi::c_int,
        words: 44 as ::core::ffi::c_int,
        remainder: 7 as ::core::ffi::c_int,
        ec: [
            10 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            22 as ::core::ffi::c_int,
            28 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 29 as ::core::ffi::c_int,
        words: 70 as ::core::ffi::c_int,
        remainder: 7 as ::core::ffi::c_int,
        ec: [
            15 as ::core::ffi::c_int,
            26 as ::core::ffi::c_int,
            36 as ::core::ffi::c_int,
            44 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 33 as ::core::ffi::c_int,
        words: 100 as ::core::ffi::c_int,
        remainder: 7 as ::core::ffi::c_int,
        ec: [
            20 as ::core::ffi::c_int,
            36 as ::core::ffi::c_int,
            52 as ::core::ffi::c_int,
            64 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 37 as ::core::ffi::c_int,
        words: 134 as ::core::ffi::c_int,
        remainder: 7 as ::core::ffi::c_int,
        ec: [
            26 as ::core::ffi::c_int,
            48 as ::core::ffi::c_int,
            72 as ::core::ffi::c_int,
            88 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 41 as ::core::ffi::c_int,
        words: 172 as ::core::ffi::c_int,
        remainder: 7 as ::core::ffi::c_int,
        ec: [
            36 as ::core::ffi::c_int,
            64 as ::core::ffi::c_int,
            96 as ::core::ffi::c_int,
            112 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 45 as ::core::ffi::c_int,
        words: 196 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            40 as ::core::ffi::c_int,
            72 as ::core::ffi::c_int,
            108 as ::core::ffi::c_int,
            130 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 49 as ::core::ffi::c_int,
        words: 242 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            48 as ::core::ffi::c_int,
            88 as ::core::ffi::c_int,
            132 as ::core::ffi::c_int,
            156 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 53 as ::core::ffi::c_int,
        words: 292 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            60 as ::core::ffi::c_int,
            110 as ::core::ffi::c_int,
            160 as ::core::ffi::c_int,
            192 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 57 as ::core::ffi::c_int,
        words: 346 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            72 as ::core::ffi::c_int,
            130 as ::core::ffi::c_int,
            192 as ::core::ffi::c_int,
            224 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 61 as ::core::ffi::c_int,
        words: 404 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            80 as ::core::ffi::c_int,
            150 as ::core::ffi::c_int,
            224 as ::core::ffi::c_int,
            264 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 65 as ::core::ffi::c_int,
        words: 466 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            96 as ::core::ffi::c_int,
            176 as ::core::ffi::c_int,
            260 as ::core::ffi::c_int,
            308 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 69 as ::core::ffi::c_int,
        words: 532 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            104 as ::core::ffi::c_int,
            198 as ::core::ffi::c_int,
            288 as ::core::ffi::c_int,
            352 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 73 as ::core::ffi::c_int,
        words: 581 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            120 as ::core::ffi::c_int,
            216 as ::core::ffi::c_int,
            320 as ::core::ffi::c_int,
            384 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 77 as ::core::ffi::c_int,
        words: 655 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            132 as ::core::ffi::c_int,
            240 as ::core::ffi::c_int,
            360 as ::core::ffi::c_int,
            432 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 81 as ::core::ffi::c_int,
        words: 733 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            144 as ::core::ffi::c_int,
            280 as ::core::ffi::c_int,
            408 as ::core::ffi::c_int,
            480 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 85 as ::core::ffi::c_int,
        words: 815 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            168 as ::core::ffi::c_int,
            308 as ::core::ffi::c_int,
            448 as ::core::ffi::c_int,
            532 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 89 as ::core::ffi::c_int,
        words: 901 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            180 as ::core::ffi::c_int,
            338 as ::core::ffi::c_int,
            504 as ::core::ffi::c_int,
            588 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 93 as ::core::ffi::c_int,
        words: 991 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            196 as ::core::ffi::c_int,
            364 as ::core::ffi::c_int,
            546 as ::core::ffi::c_int,
            650 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 97 as ::core::ffi::c_int,
        words: 1085 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            224 as ::core::ffi::c_int,
            416 as ::core::ffi::c_int,
            600 as ::core::ffi::c_int,
            700 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 101 as ::core::ffi::c_int,
        words: 1156 as ::core::ffi::c_int,
        remainder: 4 as ::core::ffi::c_int,
        ec: [
            224 as ::core::ffi::c_int,
            442 as ::core::ffi::c_int,
            644 as ::core::ffi::c_int,
            750 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 105 as ::core::ffi::c_int,
        words: 1258 as ::core::ffi::c_int,
        remainder: 4 as ::core::ffi::c_int,
        ec: [
            252 as ::core::ffi::c_int,
            476 as ::core::ffi::c_int,
            690 as ::core::ffi::c_int,
            816 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 109 as ::core::ffi::c_int,
        words: 1364 as ::core::ffi::c_int,
        remainder: 4 as ::core::ffi::c_int,
        ec: [
            270 as ::core::ffi::c_int,
            504 as ::core::ffi::c_int,
            750 as ::core::ffi::c_int,
            900 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 113 as ::core::ffi::c_int,
        words: 1474 as ::core::ffi::c_int,
        remainder: 4 as ::core::ffi::c_int,
        ec: [
            300 as ::core::ffi::c_int,
            560 as ::core::ffi::c_int,
            810 as ::core::ffi::c_int,
            960 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 117 as ::core::ffi::c_int,
        words: 1588 as ::core::ffi::c_int,
        remainder: 4 as ::core::ffi::c_int,
        ec: [
            312 as ::core::ffi::c_int,
            588 as ::core::ffi::c_int,
            870 as ::core::ffi::c_int,
            1050 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 121 as ::core::ffi::c_int,
        words: 1706 as ::core::ffi::c_int,
        remainder: 4 as ::core::ffi::c_int,
        ec: [
            336 as ::core::ffi::c_int,
            644 as ::core::ffi::c_int,
            952 as ::core::ffi::c_int,
            1110 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 125 as ::core::ffi::c_int,
        words: 1828 as ::core::ffi::c_int,
        remainder: 4 as ::core::ffi::c_int,
        ec: [
            360 as ::core::ffi::c_int,
            700 as ::core::ffi::c_int,
            1020 as ::core::ffi::c_int,
            1200 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 129 as ::core::ffi::c_int,
        words: 1921 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            390 as ::core::ffi::c_int,
            728 as ::core::ffi::c_int,
            1050 as ::core::ffi::c_int,
            1260 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 133 as ::core::ffi::c_int,
        words: 2051 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            420 as ::core::ffi::c_int,
            784 as ::core::ffi::c_int,
            1140 as ::core::ffi::c_int,
            1350 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 137 as ::core::ffi::c_int,
        words: 2185 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            450 as ::core::ffi::c_int,
            812 as ::core::ffi::c_int,
            1200 as ::core::ffi::c_int,
            1440 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 141 as ::core::ffi::c_int,
        words: 2323 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            480 as ::core::ffi::c_int,
            868 as ::core::ffi::c_int,
            1290 as ::core::ffi::c_int,
            1530 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 145 as ::core::ffi::c_int,
        words: 2465 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            510 as ::core::ffi::c_int,
            924 as ::core::ffi::c_int,
            1350 as ::core::ffi::c_int,
            1620 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 149 as ::core::ffi::c_int,
        words: 2611 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            540 as ::core::ffi::c_int,
            980 as ::core::ffi::c_int,
            1440 as ::core::ffi::c_int,
            1710 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 153 as ::core::ffi::c_int,
        words: 2761 as ::core::ffi::c_int,
        remainder: 3 as ::core::ffi::c_int,
        ec: [
            570 as ::core::ffi::c_int,
            1036 as ::core::ffi::c_int,
            1530 as ::core::ffi::c_int,
            1800 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 157 as ::core::ffi::c_int,
        words: 2876 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            570 as ::core::ffi::c_int,
            1064 as ::core::ffi::c_int,
            1590 as ::core::ffi::c_int,
            1890 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 161 as ::core::ffi::c_int,
        words: 3034 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            600 as ::core::ffi::c_int,
            1120 as ::core::ffi::c_int,
            1680 as ::core::ffi::c_int,
            1980 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 165 as ::core::ffi::c_int,
        words: 3196 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            630 as ::core::ffi::c_int,
            1204 as ::core::ffi::c_int,
            1770 as ::core::ffi::c_int,
            2100 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 169 as ::core::ffi::c_int,
        words: 3362 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            660 as ::core::ffi::c_int,
            1260 as ::core::ffi::c_int,
            1860 as ::core::ffi::c_int,
            2220 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 173 as ::core::ffi::c_int,
        words: 3532 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            720 as ::core::ffi::c_int,
            1316 as ::core::ffi::c_int,
            1950 as ::core::ffi::c_int,
            2310 as ::core::ffi::c_int,
        ],
    },
    QRspec_Capacity {
        width: 177 as ::core::ffi::c_int,
        words: 3706 as ::core::ffi::c_int,
        remainder: 0 as ::core::ffi::c_int,
        ec: [
            750 as ::core::ffi::c_int,
            1372 as ::core::ffi::c_int,
            2040 as ::core::ffi::c_int,
            2430 as ::core::ffi::c_int,
        ],
    },
];
#[no_mangle]
pub unsafe extern "C" fn QRspec_getDataLength(
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_int {
    return qrspecCapacity[version as usize].words
        - qrspecCapacity[version as usize].ec[level as usize];
}
#[no_mangle]
pub unsafe extern "C" fn QRspec_getECCLength(
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_int {
    return qrspecCapacity[version as usize].ec[level as usize];
}
#[no_mangle]
pub unsafe extern "C" fn QRspec_getMinimumVersion(
    mut size: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut words: ::core::ffi::c_int = 0;
    i = 1 as ::core::ffi::c_int;
    while i <= QRSPEC_VERSION_MAX {
        words = qrspecCapacity[i as usize].words - qrspecCapacity[i as usize].ec[level as usize];
        if words >= size {
            return i;
        }
        i += 1;
    }
    return QRSPEC_VERSION_MAX;
}
#[no_mangle]
pub unsafe extern "C" fn QRspec_getWidth(mut version: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return qrspecCapacity[version as usize].width;
}
#[no_mangle]
pub unsafe extern "C" fn QRspec_getRemainder(
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return qrspecCapacity[version as usize].remainder;
}
static mut lengthTableBits: [[::core::ffi::c_int; 3]; 4] = [
    [
        10 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
        14 as ::core::ffi::c_int,
    ],
    [
        9 as ::core::ffi::c_int,
        11 as ::core::ffi::c_int,
        13 as ::core::ffi::c_int,
    ],
    [
        8 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
    ],
    [
        8 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn QRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = 0;
    if QRinput_isSplittableMode(mode) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if version <= 9 as ::core::ffi::c_int {
        l = 0 as ::core::ffi::c_int;
    } else if version <= 26 as ::core::ffi::c_int {
        l = 1 as ::core::ffi::c_int;
    } else {
        l = 2 as ::core::ffi::c_int;
    }
    return lengthTableBits[mode as usize][l as usize];
}
#[no_mangle]
pub unsafe extern "C" fn QRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = 0;
    let mut bits: ::core::ffi::c_int = 0;
    let mut words: ::core::ffi::c_int = 0;
    if QRinput_isSplittableMode(mode) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if version <= 9 as ::core::ffi::c_int {
        l = 0 as ::core::ffi::c_int;
    } else if version <= 26 as ::core::ffi::c_int {
        l = 1 as ::core::ffi::c_int;
    } else {
        l = 2 as ::core::ffi::c_int;
    }
    bits = lengthTableBits[mode as usize][l as usize];
    words = ((1 as ::core::ffi::c_int) << bits) - 1 as ::core::ffi::c_int;
    if mode as ::core::ffi::c_int == QR_MODE_KANJI as ::core::ffi::c_int {
        words *= 2 as ::core::ffi::c_int;
    }
    return words;
}
static mut eccTable: [[[::core::ffi::c_int; 2]; 4]; 41] = [
    [
        [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    ],
    [
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    ],
    [
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    ],
    [
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    ],
    [
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    ],
    [
        [1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
    ],
    [
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    ],
    [
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
    ],
    [
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
    ],
    [
        [2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [3 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
    ],
    [
        [2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [6 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [6 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
    ],
    [
        [4 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [3 as ::core::ffi::c_int, 8 as ::core::ffi::c_int],
    ],
    [
        [2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [6 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 6 as ::core::ffi::c_int],
        [7 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
    ],
    [
        [4 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [8 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [8 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [12 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
    ],
    [
        [3 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 5 as ::core::ffi::c_int],
        [11 as ::core::ffi::c_int, 5 as ::core::ffi::c_int],
        [11 as ::core::ffi::c_int, 5 as ::core::ffi::c_int],
    ],
    [
        [5 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [5 as ::core::ffi::c_int, 5 as ::core::ffi::c_int],
        [5 as ::core::ffi::c_int, 7 as ::core::ffi::c_int],
        [11 as ::core::ffi::c_int, 7 as ::core::ffi::c_int],
    ],
    [
        [5 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [7 as ::core::ffi::c_int, 3 as ::core::ffi::c_int],
        [15 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [3 as ::core::ffi::c_int, 13 as ::core::ffi::c_int],
    ],
    [
        [1 as ::core::ffi::c_int, 5 as ::core::ffi::c_int],
        [10 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 15 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 17 as ::core::ffi::c_int],
    ],
    [
        [5 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [9 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [17 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 19 as ::core::ffi::c_int],
    ],
    [
        [3 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [3 as ::core::ffi::c_int, 11 as ::core::ffi::c_int],
        [17 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [9 as ::core::ffi::c_int, 16 as ::core::ffi::c_int],
    ],
    [
        [3 as ::core::ffi::c_int, 5 as ::core::ffi::c_int],
        [3 as ::core::ffi::c_int, 13 as ::core::ffi::c_int],
        [15 as ::core::ffi::c_int, 5 as ::core::ffi::c_int],
        [15 as ::core::ffi::c_int, 10 as ::core::ffi::c_int],
    ],
    [
        [4 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [17 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [17 as ::core::ffi::c_int, 6 as ::core::ffi::c_int],
        [19 as ::core::ffi::c_int, 6 as ::core::ffi::c_int],
    ],
    [
        [2 as ::core::ffi::c_int, 7 as ::core::ffi::c_int],
        [17 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [7 as ::core::ffi::c_int, 16 as ::core::ffi::c_int],
        [34 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    ],
    [
        [4 as ::core::ffi::c_int, 5 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 14 as ::core::ffi::c_int],
        [11 as ::core::ffi::c_int, 14 as ::core::ffi::c_int],
        [16 as ::core::ffi::c_int, 14 as ::core::ffi::c_int],
    ],
    [
        [6 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [6 as ::core::ffi::c_int, 14 as ::core::ffi::c_int],
        [11 as ::core::ffi::c_int, 16 as ::core::ffi::c_int],
        [30 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
    ],
    [
        [8 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [8 as ::core::ffi::c_int, 13 as ::core::ffi::c_int],
        [7 as ::core::ffi::c_int, 22 as ::core::ffi::c_int],
        [22 as ::core::ffi::c_int, 13 as ::core::ffi::c_int],
    ],
    [
        [10 as ::core::ffi::c_int, 2 as ::core::ffi::c_int],
        [19 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [28 as ::core::ffi::c_int, 6 as ::core::ffi::c_int],
        [33 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
    ],
    [
        [8 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [22 as ::core::ffi::c_int, 3 as ::core::ffi::c_int],
        [8 as ::core::ffi::c_int, 26 as ::core::ffi::c_int],
        [12 as ::core::ffi::c_int, 28 as ::core::ffi::c_int],
    ],
    [
        [3 as ::core::ffi::c_int, 10 as ::core::ffi::c_int],
        [3 as ::core::ffi::c_int, 23 as ::core::ffi::c_int],
        [4 as ::core::ffi::c_int, 31 as ::core::ffi::c_int],
        [11 as ::core::ffi::c_int, 31 as ::core::ffi::c_int],
    ],
    [
        [7 as ::core::ffi::c_int, 7 as ::core::ffi::c_int],
        [21 as ::core::ffi::c_int, 7 as ::core::ffi::c_int],
        [1 as ::core::ffi::c_int, 37 as ::core::ffi::c_int],
        [19 as ::core::ffi::c_int, 26 as ::core::ffi::c_int],
    ],
    [
        [5 as ::core::ffi::c_int, 10 as ::core::ffi::c_int],
        [19 as ::core::ffi::c_int, 10 as ::core::ffi::c_int],
        [15 as ::core::ffi::c_int, 25 as ::core::ffi::c_int],
        [23 as ::core::ffi::c_int, 25 as ::core::ffi::c_int],
    ],
    [
        [13 as ::core::ffi::c_int, 3 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 29 as ::core::ffi::c_int],
        [42 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [23 as ::core::ffi::c_int, 28 as ::core::ffi::c_int],
    ],
    [
        [17 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
        [10 as ::core::ffi::c_int, 23 as ::core::ffi::c_int],
        [10 as ::core::ffi::c_int, 35 as ::core::ffi::c_int],
        [19 as ::core::ffi::c_int, 35 as ::core::ffi::c_int],
    ],
    [
        [17 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
        [14 as ::core::ffi::c_int, 21 as ::core::ffi::c_int],
        [29 as ::core::ffi::c_int, 19 as ::core::ffi::c_int],
        [11 as ::core::ffi::c_int, 46 as ::core::ffi::c_int],
    ],
    [
        [13 as ::core::ffi::c_int, 6 as ::core::ffi::c_int],
        [14 as ::core::ffi::c_int, 23 as ::core::ffi::c_int],
        [44 as ::core::ffi::c_int, 7 as ::core::ffi::c_int],
        [59 as ::core::ffi::c_int, 1 as ::core::ffi::c_int],
    ],
    [
        [12 as ::core::ffi::c_int, 7 as ::core::ffi::c_int],
        [12 as ::core::ffi::c_int, 26 as ::core::ffi::c_int],
        [39 as ::core::ffi::c_int, 14 as ::core::ffi::c_int],
        [22 as ::core::ffi::c_int, 41 as ::core::ffi::c_int],
    ],
    [
        [6 as ::core::ffi::c_int, 14 as ::core::ffi::c_int],
        [6 as ::core::ffi::c_int, 34 as ::core::ffi::c_int],
        [46 as ::core::ffi::c_int, 10 as ::core::ffi::c_int],
        [2 as ::core::ffi::c_int, 64 as ::core::ffi::c_int],
    ],
    [
        [17 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [29 as ::core::ffi::c_int, 14 as ::core::ffi::c_int],
        [49 as ::core::ffi::c_int, 10 as ::core::ffi::c_int],
        [24 as ::core::ffi::c_int, 46 as ::core::ffi::c_int],
    ],
    [
        [4 as ::core::ffi::c_int, 18 as ::core::ffi::c_int],
        [13 as ::core::ffi::c_int, 32 as ::core::ffi::c_int],
        [48 as ::core::ffi::c_int, 14 as ::core::ffi::c_int],
        [42 as ::core::ffi::c_int, 32 as ::core::ffi::c_int],
    ],
    [
        [20 as ::core::ffi::c_int, 4 as ::core::ffi::c_int],
        [40 as ::core::ffi::c_int, 7 as ::core::ffi::c_int],
        [43 as ::core::ffi::c_int, 22 as ::core::ffi::c_int],
        [10 as ::core::ffi::c_int, 67 as ::core::ffi::c_int],
    ],
    [
        [19 as ::core::ffi::c_int, 6 as ::core::ffi::c_int],
        [18 as ::core::ffi::c_int, 31 as ::core::ffi::c_int],
        [34 as ::core::ffi::c_int, 34 as ::core::ffi::c_int],
        [20 as ::core::ffi::c_int, 61 as ::core::ffi::c_int],
    ],
];
#[no_mangle]
pub unsafe extern "C" fn QRspec_getEccSpec(
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
    mut spec: *mut ::core::ffi::c_int,
) {
    let mut b1: ::core::ffi::c_int = 0;
    let mut b2: ::core::ffi::c_int = 0;
    let mut data: ::core::ffi::c_int = 0;
    let mut ecc: ::core::ffi::c_int = 0;
    b1 = eccTable[version as usize][level as usize][0 as ::core::ffi::c_int as usize];
    b2 = eccTable[version as usize][level as usize][1 as ::core::ffi::c_int as usize];
    data = QRspec_getDataLength(version, level);
    ecc = QRspec_getECCLength(version, level);
    if b2 == 0 as ::core::ffi::c_int {
        *spec.offset(0 as ::core::ffi::c_int as isize) = b1;
        *spec.offset(1 as ::core::ffi::c_int as isize) = data / b1;
        *spec.offset(2 as ::core::ffi::c_int as isize) = ecc / b1;
        let ref mut fresh0 = *spec.offset(4 as ::core::ffi::c_int as isize);
        *fresh0 = 0 as ::core::ffi::c_int;
        *spec.offset(3 as ::core::ffi::c_int as isize) = *fresh0;
    } else {
        *spec.offset(0 as ::core::ffi::c_int as isize) = b1;
        *spec.offset(1 as ::core::ffi::c_int as isize) = data / (b1 + b2);
        *spec.offset(2 as ::core::ffi::c_int as isize) = ecc / (b1 + b2);
        *spec.offset(3 as ::core::ffi::c_int as isize) = b2;
        *spec.offset(4 as ::core::ffi::c_int as isize) =
            *spec.offset(1 as ::core::ffi::c_int as isize) + 1 as ::core::ffi::c_int;
    };
}
static mut alignmentPattern: [[::core::ffi::c_int; 2]; 41] = [
    [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    [0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    [18 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    [22 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    [34 as ::core::ffi::c_int, 0 as ::core::ffi::c_int],
    [22 as ::core::ffi::c_int, 38 as ::core::ffi::c_int],
    [24 as ::core::ffi::c_int, 42 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 46 as ::core::ffi::c_int],
    [28 as ::core::ffi::c_int, 50 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 54 as ::core::ffi::c_int],
    [32 as ::core::ffi::c_int, 58 as ::core::ffi::c_int],
    [34 as ::core::ffi::c_int, 62 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 46 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 48 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 50 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 54 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 56 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 58 as ::core::ffi::c_int],
    [34 as ::core::ffi::c_int, 62 as ::core::ffi::c_int],
    [28 as ::core::ffi::c_int, 50 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 50 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 54 as ::core::ffi::c_int],
    [28 as ::core::ffi::c_int, 54 as ::core::ffi::c_int],
    [32 as ::core::ffi::c_int, 58 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 58 as ::core::ffi::c_int],
    [34 as ::core::ffi::c_int, 62 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 50 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 54 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 52 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 56 as ::core::ffi::c_int],
    [34 as ::core::ffi::c_int, 60 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 58 as ::core::ffi::c_int],
    [34 as ::core::ffi::c_int, 62 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 54 as ::core::ffi::c_int],
    [24 as ::core::ffi::c_int, 50 as ::core::ffi::c_int],
    [28 as ::core::ffi::c_int, 54 as ::core::ffi::c_int],
    [32 as ::core::ffi::c_int, 58 as ::core::ffi::c_int],
    [26 as ::core::ffi::c_int, 54 as ::core::ffi::c_int],
    [30 as ::core::ffi::c_int, 58 as ::core::ffi::c_int],
];
unsafe extern "C" fn QRspec_putAlignmentMarker(
    mut frame: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut ox: ::core::ffi::c_int,
    mut oy: ::core::ffi::c_int,
) {
    static mut finder: [::core::ffi::c_uchar; 25] = [
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut s: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    frame = frame
        .offset(((oy - 2 as ::core::ffi::c_int) * width + ox - 2 as ::core::ffi::c_int) as isize);
    s = &raw const finder as *const ::core::ffi::c_uchar;
    y = 0 as ::core::ffi::c_int;
    while y < 5 as ::core::ffi::c_int {
        x = 0 as ::core::ffi::c_int;
        while x < 5 as ::core::ffi::c_int {
            *frame.offset(x as isize) = *s.offset(x as isize);
            x += 1;
        }
        frame = frame.offset(width as isize);
        s = s.offset(5 as ::core::ffi::c_int as isize);
        y += 1;
    }
}
unsafe extern "C" fn QRspec_putAlignmentPattern(
    mut version: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
) {
    let mut d: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut cx: ::core::ffi::c_int = 0;
    let mut cy: ::core::ffi::c_int = 0;
    if version < 2 as ::core::ffi::c_int {
        return;
    }
    d = alignmentPattern[version as usize][1 as ::core::ffi::c_int as usize]
        - alignmentPattern[version as usize][0 as ::core::ffi::c_int as usize];
    if d < 0 as ::core::ffi::c_int {
        w = 2 as ::core::ffi::c_int;
    } else {
        w = (width - alignmentPattern[version as usize][0 as ::core::ffi::c_int as usize]) / d
            + 2 as ::core::ffi::c_int;
    }
    if w * w - 3 as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        x = alignmentPattern[version as usize][0 as ::core::ffi::c_int as usize];
        y = alignmentPattern[version as usize][0 as ::core::ffi::c_int as usize];
        QRspec_putAlignmentMarker(frame, width, x, y);
        return;
    }
    cx = alignmentPattern[version as usize][0 as ::core::ffi::c_int as usize];
    x = 1 as ::core::ffi::c_int;
    while x < w - 1 as ::core::ffi::c_int {
        QRspec_putAlignmentMarker(frame, width, 6 as ::core::ffi::c_int, cx);
        QRspec_putAlignmentMarker(frame, width, cx, 6 as ::core::ffi::c_int);
        cx += d;
        x += 1;
    }
    cy = alignmentPattern[version as usize][0 as ::core::ffi::c_int as usize];
    y = 0 as ::core::ffi::c_int;
    while y < w - 1 as ::core::ffi::c_int {
        cx = alignmentPattern[version as usize][0 as ::core::ffi::c_int as usize];
        x = 0 as ::core::ffi::c_int;
        while x < w - 1 as ::core::ffi::c_int {
            QRspec_putAlignmentMarker(frame, width, cx, cy);
            cx += d;
            x += 1;
        }
        cy += d;
        y += 1;
    }
}
static mut versionPattern: [::core::ffi::c_uint; 34] = [
    0x7c94 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x85bc as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x9a99 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xa4d3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xbbf6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xc762 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xd847 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xe60d as ::core::ffi::c_int as ::core::ffi::c_uint,
    0xf928 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x10b78 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1145d as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x12a17 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x13532 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x149a6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x15683 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x168c9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x177ec as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x18ec4 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x191e1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1afab as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1b08e as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1cc1a as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1d33f as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1ed75 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x1f250 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x209d5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x216f0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x228ba as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x2379f as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x24b0b as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x2542e as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x26a64 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x27541 as ::core::ffi::c_int as ::core::ffi::c_uint,
    0x28c69 as ::core::ffi::c_int as ::core::ffi::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn QRspec_getVersionPattern(
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if version < 7 as ::core::ffi::c_int || version > QRSPEC_VERSION_MAX {
        return 0 as ::core::ffi::c_uint;
    }
    return versionPattern[(version - 7 as ::core::ffi::c_int) as usize];
}
static mut formatInfo: [[::core::ffi::c_uint; 8]; 4] = [
    [
        0x77c4 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x72f3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7daa as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x789d as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x662f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x6318 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x6c41 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x6976 as ::core::ffi::c_int as ::core::ffi::c_uint,
    ],
    [
        0x5412 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x5125 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x5e7c as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x5b4b as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x45f9 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x40ce as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x4f97 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x4aa0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    ],
    [
        0x355f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3068 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3f31 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3a06 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x24b4 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2183 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2eda as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2bed as ::core::ffi::c_int as ::core::ffi::c_uint,
    ],
    [
        0x1689 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x13be as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1ce7 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x19d0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x762 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x255 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xd0c as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x83b as ::core::ffi::c_int as ::core::ffi::c_uint,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn QRspec_getFormatInfo(
    mut mask: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> ::core::ffi::c_uint {
    if mask < 0 as ::core::ffi::c_int || mask > 7 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_uint;
    }
    return formatInfo[level as usize][mask as usize];
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
unsafe extern "C" fn QRspec_createFrame(
    mut version: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut frame: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut q: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut width: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut verinfo: ::core::ffi::c_uint = 0;
    let mut v: ::core::ffi::c_uint = 0;
    width = qrspecCapacity[version as usize].width;
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
    putFinderPattern(
        frame,
        width,
        width - 7 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    putFinderPattern(
        frame,
        width,
        0 as ::core::ffi::c_int,
        width - 7 as ::core::ffi::c_int,
    );
    p = frame;
    q = frame.offset((width * (width - 7 as ::core::ffi::c_int)) as isize);
    y = 0 as ::core::ffi::c_int;
    while y < 7 as ::core::ffi::c_int {
        *p.offset(7 as ::core::ffi::c_int as isize) = 0xc0 as ::core::ffi::c_uchar;
        *p.offset((width - 8 as ::core::ffi::c_int) as isize) = 0xc0 as ::core::ffi::c_uchar;
        *q.offset(7 as ::core::ffi::c_int as isize) = 0xc0 as ::core::ffi::c_uchar;
        p = p.offset(width as isize);
        q = q.offset(width as isize);
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
            .offset(-(8 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
        0xc0 as ::core::ffi::c_int,
        8 as size_t,
    );
    memset(
        frame.offset((width * (width - 8 as ::core::ffi::c_int)) as isize)
            as *mut ::core::ffi::c_void,
        0xc0 as ::core::ffi::c_int,
        8 as size_t,
    );
    memset(
        frame.offset((width * 8 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_void,
        0x84 as ::core::ffi::c_int,
        9 as size_t,
    );
    memset(
        frame
            .offset((width * 9 as ::core::ffi::c_int) as isize)
            .offset(-(8 as ::core::ffi::c_int as isize)) as *mut ::core::ffi::c_void,
        0x84 as ::core::ffi::c_int,
        8 as size_t,
    );
    p = frame.offset(8 as ::core::ffi::c_int as isize);
    y = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        *p = 0x84 as ::core::ffi::c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame
        .offset((width * (width - 7 as ::core::ffi::c_int)) as isize)
        .offset(8 as ::core::ffi::c_int as isize);
    y = 0 as ::core::ffi::c_int;
    while y < 7 as ::core::ffi::c_int {
        *p = 0x84 as ::core::ffi::c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame
        .offset((width * 6 as ::core::ffi::c_int) as isize)
        .offset(8 as ::core::ffi::c_int as isize);
    q = frame
        .offset((width * 8 as ::core::ffi::c_int) as isize)
        .offset(6 as ::core::ffi::c_int as isize);
    x = 1 as ::core::ffi::c_int;
    while x < width - 15 as ::core::ffi::c_int {
        *p = (0x90 as ::core::ffi::c_int | x & 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        *q = (0x90 as ::core::ffi::c_int | x & 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        p = p.offset(1);
        q = q.offset(width as isize);
        x += 1;
    }
    QRspec_putAlignmentPattern(version, frame, width);
    if version >= 7 as ::core::ffi::c_int {
        verinfo = QRspec_getVersionPattern(version);
        p = frame.offset((width * (width - 11 as ::core::ffi::c_int)) as isize);
        v = verinfo;
        x = 0 as ::core::ffi::c_int;
        while x < 6 as ::core::ffi::c_int {
            y = 0 as ::core::ffi::c_int;
            while y < 3 as ::core::ffi::c_int {
                *p.offset((width * y + x) as isize) = (0x88 as ::core::ffi::c_uint
                    | v & 1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_uchar;
                v = v >> 1 as ::core::ffi::c_int;
                y += 1;
            }
            x += 1;
        }
        p = frame
            .offset(width as isize)
            .offset(-(11 as ::core::ffi::c_int as isize));
        v = verinfo;
        y = 0 as ::core::ffi::c_int;
        while y < 6 as ::core::ffi::c_int {
            x = 0 as ::core::ffi::c_int;
            while x < 3 as ::core::ffi::c_int {
                *p.offset(x as isize) = (0x88 as ::core::ffi::c_uint | v & 1 as ::core::ffi::c_uint)
                    as ::core::ffi::c_uchar;
                v = v >> 1 as ::core::ffi::c_int;
                x += 1;
            }
            p = p.offset(width as isize);
            y += 1;
        }
    }
    *frame.offset((width * (width - 8 as ::core::ffi::c_int) + 8 as ::core::ffi::c_int) as isize) =
        0x81 as ::core::ffi::c_uchar;
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn QRspec_newFrame(
    mut version: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    if version < 1 as ::core::ffi::c_int || version > QRSPEC_VERSION_MAX {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return QRspec_createFrame(version);
}
