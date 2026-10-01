use core::ffi::*;
use crate::src::qrinput::QRinput_isSplittableMode;
use crate::src::c_inlined_fns::putFinderPattern;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRspec_Capacity {
    pub width: c_int,
    pub words: c_int,
    pub remainder: c_int,
    pub ec: [c_int; 4],
}

static mut qrspecCapacity: [QRspec_Capacity; 41] = [
    QRspec_Capacity {
        width: 0 as c_int,
        words: 0 as c_int,
        remainder: 0 as c_int,
        ec: [
            0 as c_int,
            0 as c_int,
            0 as c_int,
            0 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 21 as c_int,
        words: 26 as c_int,
        remainder: 0 as c_int,
        ec: [
            7 as c_int,
            10 as c_int,
            13 as c_int,
            17 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 25 as c_int,
        words: 44 as c_int,
        remainder: 7 as c_int,
        ec: [
            10 as c_int,
            16 as c_int,
            22 as c_int,
            28 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 29 as c_int,
        words: 70 as c_int,
        remainder: 7 as c_int,
        ec: [
            15 as c_int,
            26 as c_int,
            36 as c_int,
            44 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 33 as c_int,
        words: 100 as c_int,
        remainder: 7 as c_int,
        ec: [
            20 as c_int,
            36 as c_int,
            52 as c_int,
            64 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 37 as c_int,
        words: 134 as c_int,
        remainder: 7 as c_int,
        ec: [
            26 as c_int,
            48 as c_int,
            72 as c_int,
            88 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 41 as c_int,
        words: 172 as c_int,
        remainder: 7 as c_int,
        ec: [
            36 as c_int,
            64 as c_int,
            96 as c_int,
            112 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 45 as c_int,
        words: 196 as c_int,
        remainder: 0 as c_int,
        ec: [
            40 as c_int,
            72 as c_int,
            108 as c_int,
            130 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 49 as c_int,
        words: 242 as c_int,
        remainder: 0 as c_int,
        ec: [
            48 as c_int,
            88 as c_int,
            132 as c_int,
            156 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 53 as c_int,
        words: 292 as c_int,
        remainder: 0 as c_int,
        ec: [
            60 as c_int,
            110 as c_int,
            160 as c_int,
            192 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 57 as c_int,
        words: 346 as c_int,
        remainder: 0 as c_int,
        ec: [
            72 as c_int,
            130 as c_int,
            192 as c_int,
            224 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 61 as c_int,
        words: 404 as c_int,
        remainder: 0 as c_int,
        ec: [
            80 as c_int,
            150 as c_int,
            224 as c_int,
            264 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 65 as c_int,
        words: 466 as c_int,
        remainder: 0 as c_int,
        ec: [
            96 as c_int,
            176 as c_int,
            260 as c_int,
            308 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 69 as c_int,
        words: 532 as c_int,
        remainder: 0 as c_int,
        ec: [
            104 as c_int,
            198 as c_int,
            288 as c_int,
            352 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 73 as c_int,
        words: 581 as c_int,
        remainder: 3 as c_int,
        ec: [
            120 as c_int,
            216 as c_int,
            320 as c_int,
            384 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 77 as c_int,
        words: 655 as c_int,
        remainder: 3 as c_int,
        ec: [
            132 as c_int,
            240 as c_int,
            360 as c_int,
            432 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 81 as c_int,
        words: 733 as c_int,
        remainder: 3 as c_int,
        ec: [
            144 as c_int,
            280 as c_int,
            408 as c_int,
            480 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 85 as c_int,
        words: 815 as c_int,
        remainder: 3 as c_int,
        ec: [
            168 as c_int,
            308 as c_int,
            448 as c_int,
            532 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 89 as c_int,
        words: 901 as c_int,
        remainder: 3 as c_int,
        ec: [
            180 as c_int,
            338 as c_int,
            504 as c_int,
            588 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 93 as c_int,
        words: 991 as c_int,
        remainder: 3 as c_int,
        ec: [
            196 as c_int,
            364 as c_int,
            546 as c_int,
            650 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 97 as c_int,
        words: 1085 as c_int,
        remainder: 3 as c_int,
        ec: [
            224 as c_int,
            416 as c_int,
            600 as c_int,
            700 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 101 as c_int,
        words: 1156 as c_int,
        remainder: 4 as c_int,
        ec: [
            224 as c_int,
            442 as c_int,
            644 as c_int,
            750 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 105 as c_int,
        words: 1258 as c_int,
        remainder: 4 as c_int,
        ec: [
            252 as c_int,
            476 as c_int,
            690 as c_int,
            816 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 109 as c_int,
        words: 1364 as c_int,
        remainder: 4 as c_int,
        ec: [
            270 as c_int,
            504 as c_int,
            750 as c_int,
            900 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 113 as c_int,
        words: 1474 as c_int,
        remainder: 4 as c_int,
        ec: [
            300 as c_int,
            560 as c_int,
            810 as c_int,
            960 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 117 as c_int,
        words: 1588 as c_int,
        remainder: 4 as c_int,
        ec: [
            312 as c_int,
            588 as c_int,
            870 as c_int,
            1050 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 121 as c_int,
        words: 1706 as c_int,
        remainder: 4 as c_int,
        ec: [
            336 as c_int,
            644 as c_int,
            952 as c_int,
            1110 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 125 as c_int,
        words: 1828 as c_int,
        remainder: 4 as c_int,
        ec: [
            360 as c_int,
            700 as c_int,
            1020 as c_int,
            1200 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 129 as c_int,
        words: 1921 as c_int,
        remainder: 3 as c_int,
        ec: [
            390 as c_int,
            728 as c_int,
            1050 as c_int,
            1260 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 133 as c_int,
        words: 2051 as c_int,
        remainder: 3 as c_int,
        ec: [
            420 as c_int,
            784 as c_int,
            1140 as c_int,
            1350 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 137 as c_int,
        words: 2185 as c_int,
        remainder: 3 as c_int,
        ec: [
            450 as c_int,
            812 as c_int,
            1200 as c_int,
            1440 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 141 as c_int,
        words: 2323 as c_int,
        remainder: 3 as c_int,
        ec: [
            480 as c_int,
            868 as c_int,
            1290 as c_int,
            1530 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 145 as c_int,
        words: 2465 as c_int,
        remainder: 3 as c_int,
        ec: [
            510 as c_int,
            924 as c_int,
            1350 as c_int,
            1620 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 149 as c_int,
        words: 2611 as c_int,
        remainder: 3 as c_int,
        ec: [
            540 as c_int,
            980 as c_int,
            1440 as c_int,
            1710 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 153 as c_int,
        words: 2761 as c_int,
        remainder: 3 as c_int,
        ec: [
            570 as c_int,
            1036 as c_int,
            1530 as c_int,
            1800 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 157 as c_int,
        words: 2876 as c_int,
        remainder: 0 as c_int,
        ec: [
            570 as c_int,
            1064 as c_int,
            1590 as c_int,
            1890 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 161 as c_int,
        words: 3034 as c_int,
        remainder: 0 as c_int,
        ec: [
            600 as c_int,
            1120 as c_int,
            1680 as c_int,
            1980 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 165 as c_int,
        words: 3196 as c_int,
        remainder: 0 as c_int,
        ec: [
            630 as c_int,
            1204 as c_int,
            1770 as c_int,
            2100 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 169 as c_int,
        words: 3362 as c_int,
        remainder: 0 as c_int,
        ec: [
            660 as c_int,
            1260 as c_int,
            1860 as c_int,
            2220 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 173 as c_int,
        words: 3532 as c_int,
        remainder: 0 as c_int,
        ec: [
            720 as c_int,
            1316 as c_int,
            1950 as c_int,
            2310 as c_int,
        ],
    },
    QRspec_Capacity {
        width: 177 as c_int,
        words: 3706 as c_int,
        remainder: 0 as c_int,
        ec: [
            750 as c_int,
            1372 as c_int,
            2040 as c_int,
            2430 as c_int,
        ],
    },
];
#[inline]
pub fn QRspec_getDataLength(
    mut version: c_int,
    mut level: QRecLevel,
) -> c_int { unsafe {
    return qrspecCapacity[version as usize].words
        - qrspecCapacity[version as usize].ec[level as usize];
} }
#[inline]
pub fn QRspec_getECCLength(
    mut version: c_int,
    mut level: QRecLevel,
) -> c_int { unsafe {
    return qrspecCapacity[version as usize].ec[level as usize];
} }
#[inline]
pub fn QRspec_getMinimumVersion(
    mut size: c_int,
    mut level: QRecLevel,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut words: c_int = 0;
    i = 1 as c_int;
    while i <= QRSPEC_VERSION_MAX {
        words = qrspecCapacity[i as usize].words - qrspecCapacity[i as usize].ec[level as usize];
        if words >= size {
            return i;
        }
        i += 1;
    }
    return QRSPEC_VERSION_MAX;
} }
#[inline]
pub fn QRspec_getWidth(mut version: c_int) -> c_int { unsafe {
    return qrspecCapacity[version as usize].width;
} }
#[inline]
pub fn QRspec_getRemainder(
    mut version: c_int,
) -> c_int { unsafe {
    return qrspecCapacity[version as usize].remainder;
} }
static mut lengthTableBits: [[c_int; 3]; 4] = [
    [
        10 as c_int,
        12 as c_int,
        14 as c_int,
    ],
    [
        9 as c_int,
        11 as c_int,
        13 as c_int,
    ],
    [
        8 as c_int,
        16 as c_int,
        16 as c_int,
    ],
    [
        8 as c_int,
        10 as c_int,
        12 as c_int,
    ],
];
#[inline]
pub fn QRspec_lengthIndicator(
    mut mode: QRencodeMode,
    mut version: c_int,
) -> c_int { unsafe {
    let mut l: c_int = 0;
    if QRinput_isSplittableMode(mode) == 0 {
        return 0 as c_int;
    }
    if version <= 9 as c_int {
        l = 0 as c_int;
    } else if version <= 26 as c_int {
        l = 1 as c_int;
    } else {
        l = 2 as c_int;
    }
    return lengthTableBits[mode as usize][l as usize];
} }
#[inline]
pub fn QRspec_maximumWords(
    mut mode: QRencodeMode,
    mut version: c_int,
) -> c_int { unsafe {
    let mut l: c_int = 0;
    let mut bits: c_int = 0;
    let mut words: c_int = 0;
    if QRinput_isSplittableMode(mode) == 0 {
        return 0 as c_int;
    }
    if version <= 9 as c_int {
        l = 0 as c_int;
    } else if version <= 26 as c_int {
        l = 1 as c_int;
    } else {
        l = 2 as c_int;
    }
    bits = lengthTableBits[mode as usize][l as usize];
    words = ((1 as c_int) << bits) - 1 as c_int;
    if mode as c_int == QR_MODE_KANJI as c_int {
        words *= 2 as c_int;
    }
    return words;
} }
static mut eccTable: [[[c_int; 2]; 4]; 41] = [
    [
        [0 as c_int, 0 as c_int],
        [0 as c_int, 0 as c_int],
        [0 as c_int, 0 as c_int],
        [0 as c_int, 0 as c_int],
    ],
    [
        [1 as c_int, 0 as c_int],
        [1 as c_int, 0 as c_int],
        [1 as c_int, 0 as c_int],
        [1 as c_int, 0 as c_int],
    ],
    [
        [1 as c_int, 0 as c_int],
        [1 as c_int, 0 as c_int],
        [1 as c_int, 0 as c_int],
        [1 as c_int, 0 as c_int],
    ],
    [
        [1 as c_int, 0 as c_int],
        [1 as c_int, 0 as c_int],
        [2 as c_int, 0 as c_int],
        [2 as c_int, 0 as c_int],
    ],
    [
        [1 as c_int, 0 as c_int],
        [2 as c_int, 0 as c_int],
        [2 as c_int, 0 as c_int],
        [4 as c_int, 0 as c_int],
    ],
    [
        [1 as c_int, 0 as c_int],
        [2 as c_int, 0 as c_int],
        [2 as c_int, 2 as c_int],
        [2 as c_int, 2 as c_int],
    ],
    [
        [2 as c_int, 0 as c_int],
        [4 as c_int, 0 as c_int],
        [4 as c_int, 0 as c_int],
        [4 as c_int, 0 as c_int],
    ],
    [
        [2 as c_int, 0 as c_int],
        [4 as c_int, 0 as c_int],
        [2 as c_int, 4 as c_int],
        [4 as c_int, 1 as c_int],
    ],
    [
        [2 as c_int, 0 as c_int],
        [2 as c_int, 2 as c_int],
        [4 as c_int, 2 as c_int],
        [4 as c_int, 2 as c_int],
    ],
    [
        [2 as c_int, 0 as c_int],
        [3 as c_int, 2 as c_int],
        [4 as c_int, 4 as c_int],
        [4 as c_int, 4 as c_int],
    ],
    [
        [2 as c_int, 2 as c_int],
        [4 as c_int, 1 as c_int],
        [6 as c_int, 2 as c_int],
        [6 as c_int, 2 as c_int],
    ],
    [
        [4 as c_int, 0 as c_int],
        [1 as c_int, 4 as c_int],
        [4 as c_int, 4 as c_int],
        [3 as c_int, 8 as c_int],
    ],
    [
        [2 as c_int, 2 as c_int],
        [6 as c_int, 2 as c_int],
        [4 as c_int, 6 as c_int],
        [7 as c_int, 4 as c_int],
    ],
    [
        [4 as c_int, 0 as c_int],
        [8 as c_int, 1 as c_int],
        [8 as c_int, 4 as c_int],
        [12 as c_int, 4 as c_int],
    ],
    [
        [3 as c_int, 1 as c_int],
        [4 as c_int, 5 as c_int],
        [11 as c_int, 5 as c_int],
        [11 as c_int, 5 as c_int],
    ],
    [
        [5 as c_int, 1 as c_int],
        [5 as c_int, 5 as c_int],
        [5 as c_int, 7 as c_int],
        [11 as c_int, 7 as c_int],
    ],
    [
        [5 as c_int, 1 as c_int],
        [7 as c_int, 3 as c_int],
        [15 as c_int, 2 as c_int],
        [3 as c_int, 13 as c_int],
    ],
    [
        [1 as c_int, 5 as c_int],
        [10 as c_int, 1 as c_int],
        [1 as c_int, 15 as c_int],
        [2 as c_int, 17 as c_int],
    ],
    [
        [5 as c_int, 1 as c_int],
        [9 as c_int, 4 as c_int],
        [17 as c_int, 1 as c_int],
        [2 as c_int, 19 as c_int],
    ],
    [
        [3 as c_int, 4 as c_int],
        [3 as c_int, 11 as c_int],
        [17 as c_int, 4 as c_int],
        [9 as c_int, 16 as c_int],
    ],
    [
        [3 as c_int, 5 as c_int],
        [3 as c_int, 13 as c_int],
        [15 as c_int, 5 as c_int],
        [15 as c_int, 10 as c_int],
    ],
    [
        [4 as c_int, 4 as c_int],
        [17 as c_int, 0 as c_int],
        [17 as c_int, 6 as c_int],
        [19 as c_int, 6 as c_int],
    ],
    [
        [2 as c_int, 7 as c_int],
        [17 as c_int, 0 as c_int],
        [7 as c_int, 16 as c_int],
        [34 as c_int, 0 as c_int],
    ],
    [
        [4 as c_int, 5 as c_int],
        [4 as c_int, 14 as c_int],
        [11 as c_int, 14 as c_int],
        [16 as c_int, 14 as c_int],
    ],
    [
        [6 as c_int, 4 as c_int],
        [6 as c_int, 14 as c_int],
        [11 as c_int, 16 as c_int],
        [30 as c_int, 2 as c_int],
    ],
    [
        [8 as c_int, 4 as c_int],
        [8 as c_int, 13 as c_int],
        [7 as c_int, 22 as c_int],
        [22 as c_int, 13 as c_int],
    ],
    [
        [10 as c_int, 2 as c_int],
        [19 as c_int, 4 as c_int],
        [28 as c_int, 6 as c_int],
        [33 as c_int, 4 as c_int],
    ],
    [
        [8 as c_int, 4 as c_int],
        [22 as c_int, 3 as c_int],
        [8 as c_int, 26 as c_int],
        [12 as c_int, 28 as c_int],
    ],
    [
        [3 as c_int, 10 as c_int],
        [3 as c_int, 23 as c_int],
        [4 as c_int, 31 as c_int],
        [11 as c_int, 31 as c_int],
    ],
    [
        [7 as c_int, 7 as c_int],
        [21 as c_int, 7 as c_int],
        [1 as c_int, 37 as c_int],
        [19 as c_int, 26 as c_int],
    ],
    [
        [5 as c_int, 10 as c_int],
        [19 as c_int, 10 as c_int],
        [15 as c_int, 25 as c_int],
        [23 as c_int, 25 as c_int],
    ],
    [
        [13 as c_int, 3 as c_int],
        [2 as c_int, 29 as c_int],
        [42 as c_int, 1 as c_int],
        [23 as c_int, 28 as c_int],
    ],
    [
        [17 as c_int, 0 as c_int],
        [10 as c_int, 23 as c_int],
        [10 as c_int, 35 as c_int],
        [19 as c_int, 35 as c_int],
    ],
    [
        [17 as c_int, 1 as c_int],
        [14 as c_int, 21 as c_int],
        [29 as c_int, 19 as c_int],
        [11 as c_int, 46 as c_int],
    ],
    [
        [13 as c_int, 6 as c_int],
        [14 as c_int, 23 as c_int],
        [44 as c_int, 7 as c_int],
        [59 as c_int, 1 as c_int],
    ],
    [
        [12 as c_int, 7 as c_int],
        [12 as c_int, 26 as c_int],
        [39 as c_int, 14 as c_int],
        [22 as c_int, 41 as c_int],
    ],
    [
        [6 as c_int, 14 as c_int],
        [6 as c_int, 34 as c_int],
        [46 as c_int, 10 as c_int],
        [2 as c_int, 64 as c_int],
    ],
    [
        [17 as c_int, 4 as c_int],
        [29 as c_int, 14 as c_int],
        [49 as c_int, 10 as c_int],
        [24 as c_int, 46 as c_int],
    ],
    [
        [4 as c_int, 18 as c_int],
        [13 as c_int, 32 as c_int],
        [48 as c_int, 14 as c_int],
        [42 as c_int, 32 as c_int],
    ],
    [
        [20 as c_int, 4 as c_int],
        [40 as c_int, 7 as c_int],
        [43 as c_int, 22 as c_int],
        [10 as c_int, 67 as c_int],
    ],
    [
        [19 as c_int, 6 as c_int],
        [18 as c_int, 31 as c_int],
        [34 as c_int, 34 as c_int],
        [20 as c_int, 61 as c_int],
    ],
];
#[inline]
pub unsafe fn QRspec_getEccSpec(
    mut version: c_int,
    mut level: QRecLevel,
    mut spec: *mut c_int,
) {
    let mut b1: c_int = 0;
    let mut b2: c_int = 0;
    let mut data: c_int = 0;
    let mut ecc: c_int = 0;
    b1 = eccTable[version as usize][level as usize][0 as c_int as usize];
    b2 = eccTable[version as usize][level as usize][1 as c_int as usize];
    data = QRspec_getDataLength(version, level);
    ecc = QRspec_getECCLength(version, level);
    if b2 == 0 as c_int {
        *spec.offset(0 as c_int as isize) = b1;
        *spec.offset(1 as c_int as isize) = data / b1;
        *spec.offset(2 as c_int as isize) = ecc / b1;
        let ref mut fresh0 = *spec.offset(4 as c_int as isize);
        *fresh0 = 0 as c_int;
        *spec.offset(3 as c_int as isize) = *fresh0;
    } else {
        *spec.offset(0 as c_int as isize) = b1;
        *spec.offset(1 as c_int as isize) = data / (b1 + b2);
        *spec.offset(2 as c_int as isize) = ecc / (b1 + b2);
        *spec.offset(3 as c_int as isize) = b2;
        *spec.offset(4 as c_int as isize) =
            *spec.offset(1 as c_int as isize) + 1 as c_int;
    };
}
static mut alignmentPattern: [[c_int; 2]; 41] = [
    [0 as c_int, 0 as c_int],
    [0 as c_int, 0 as c_int],
    [18 as c_int, 0 as c_int],
    [22 as c_int, 0 as c_int],
    [26 as c_int, 0 as c_int],
    [30 as c_int, 0 as c_int],
    [34 as c_int, 0 as c_int],
    [22 as c_int, 38 as c_int],
    [24 as c_int, 42 as c_int],
    [26 as c_int, 46 as c_int],
    [28 as c_int, 50 as c_int],
    [30 as c_int, 54 as c_int],
    [32 as c_int, 58 as c_int],
    [34 as c_int, 62 as c_int],
    [26 as c_int, 46 as c_int],
    [26 as c_int, 48 as c_int],
    [26 as c_int, 50 as c_int],
    [30 as c_int, 54 as c_int],
    [30 as c_int, 56 as c_int],
    [30 as c_int, 58 as c_int],
    [34 as c_int, 62 as c_int],
    [28 as c_int, 50 as c_int],
    [26 as c_int, 50 as c_int],
    [30 as c_int, 54 as c_int],
    [28 as c_int, 54 as c_int],
    [32 as c_int, 58 as c_int],
    [30 as c_int, 58 as c_int],
    [34 as c_int, 62 as c_int],
    [26 as c_int, 50 as c_int],
    [30 as c_int, 54 as c_int],
    [26 as c_int, 52 as c_int],
    [30 as c_int, 56 as c_int],
    [34 as c_int, 60 as c_int],
    [30 as c_int, 58 as c_int],
    [34 as c_int, 62 as c_int],
    [30 as c_int, 54 as c_int],
    [24 as c_int, 50 as c_int],
    [28 as c_int, 54 as c_int],
    [32 as c_int, 58 as c_int],
    [26 as c_int, 54 as c_int],
    [30 as c_int, 58 as c_int],
];
unsafe fn QRspec_putAlignmentMarker(
    mut frame: *mut c_uchar,
    mut width: c_int,
    mut ox: c_int,
    mut oy: c_int,
) {
    static mut finder: [c_uchar; 25] = [
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa0 as c_int as c_uchar,
        0xa0 as c_int as c_uchar,
        0xa0 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa0 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa0 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa0 as c_int as c_uchar,
        0xa0 as c_int as c_uchar,
        0xa0 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
        0xa1 as c_int as c_uchar,
    ];
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut s: *const c_uchar = ::core::ptr::null::<c_uchar>();
    frame = frame
        .offset(((oy - 2 as c_int) * width + ox - 2 as c_int) as isize);
    s = &raw const finder as *const c_uchar;
    y = 0 as c_int;
    while y < 5 as c_int {
        x = 0 as c_int;
        while x < 5 as c_int {
            *frame.offset(x as isize) = *s.offset(x as isize);
            x += 1;
        }
        frame = frame.offset(width as isize);
        s = s.offset(5 as c_int as isize);
        y += 1;
    }
}
unsafe fn QRspec_putAlignmentPattern(
    mut version: c_int,
    mut frame: *mut c_uchar,
    mut width: c_int,
) {
    let mut d: c_int = 0;
    let mut w: c_int = 0;
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut cx: c_int = 0;
    let mut cy: c_int = 0;
    if version < 2 as c_int {
        return;
    }
    d = alignmentPattern[version as usize][1 as c_int as usize]
        - alignmentPattern[version as usize][0 as c_int as usize];
    if d < 0 as c_int {
        w = 2 as c_int;
    } else {
        w = (width - alignmentPattern[version as usize][0 as c_int as usize]) / d
            + 2 as c_int;
    }
    if w * w - 3 as c_int == 1 as c_int {
        x = alignmentPattern[version as usize][0 as c_int as usize];
        y = alignmentPattern[version as usize][0 as c_int as usize];
        QRspec_putAlignmentMarker(frame, width, x, y);
        return;
    }
    cx = alignmentPattern[version as usize][0 as c_int as usize];
    x = 1 as c_int;
    while x < w - 1 as c_int {
        QRspec_putAlignmentMarker(frame, width, 6 as c_int, cx);
        QRspec_putAlignmentMarker(frame, width, cx, 6 as c_int);
        cx += d;
        x += 1;
    }
    cy = alignmentPattern[version as usize][0 as c_int as usize];
    y = 0 as c_int;
    while y < w - 1 as c_int {
        cx = alignmentPattern[version as usize][0 as c_int as usize];
        x = 0 as c_int;
        while x < w - 1 as c_int {
            QRspec_putAlignmentMarker(frame, width, cx, cy);
            cx += d;
            x += 1;
        }
        cy += d;
        y += 1;
    }
}
static mut versionPattern: [c_uint; 34] = [
    0x7c94 as c_int as c_uint,
    0x85bc as c_int as c_uint,
    0x9a99 as c_int as c_uint,
    0xa4d3 as c_int as c_uint,
    0xbbf6 as c_int as c_uint,
    0xc762 as c_int as c_uint,
    0xd847 as c_int as c_uint,
    0xe60d as c_int as c_uint,
    0xf928 as c_int as c_uint,
    0x10b78 as c_int as c_uint,
    0x1145d as c_int as c_uint,
    0x12a17 as c_int as c_uint,
    0x13532 as c_int as c_uint,
    0x149a6 as c_int as c_uint,
    0x15683 as c_int as c_uint,
    0x168c9 as c_int as c_uint,
    0x177ec as c_int as c_uint,
    0x18ec4 as c_int as c_uint,
    0x191e1 as c_int as c_uint,
    0x1afab as c_int as c_uint,
    0x1b08e as c_int as c_uint,
    0x1cc1a as c_int as c_uint,
    0x1d33f as c_int as c_uint,
    0x1ed75 as c_int as c_uint,
    0x1f250 as c_int as c_uint,
    0x209d5 as c_int as c_uint,
    0x216f0 as c_int as c_uint,
    0x228ba as c_int as c_uint,
    0x2379f as c_int as c_uint,
    0x24b0b as c_int as c_uint,
    0x2542e as c_int as c_uint,
    0x26a64 as c_int as c_uint,
    0x27541 as c_int as c_uint,
    0x28c69 as c_int as c_uint,
];
#[inline]
pub fn QRspec_getVersionPattern(
    mut version: c_int,
) -> c_uint { unsafe {
    if version < 7 as c_int || version > QRSPEC_VERSION_MAX {
        return 0 as c_uint;
    }
    return versionPattern[(version - 7 as c_int) as usize];
} }
static mut formatInfo: [[c_uint; 8]; 4] = [
    [
        0x77c4 as c_int as c_uint,
        0x72f3 as c_int as c_uint,
        0x7daa as c_int as c_uint,
        0x789d as c_int as c_uint,
        0x662f as c_int as c_uint,
        0x6318 as c_int as c_uint,
        0x6c41 as c_int as c_uint,
        0x6976 as c_int as c_uint,
    ],
    [
        0x5412 as c_int as c_uint,
        0x5125 as c_int as c_uint,
        0x5e7c as c_int as c_uint,
        0x5b4b as c_int as c_uint,
        0x45f9 as c_int as c_uint,
        0x40ce as c_int as c_uint,
        0x4f97 as c_int as c_uint,
        0x4aa0 as c_int as c_uint,
    ],
    [
        0x355f as c_int as c_uint,
        0x3068 as c_int as c_uint,
        0x3f31 as c_int as c_uint,
        0x3a06 as c_int as c_uint,
        0x24b4 as c_int as c_uint,
        0x2183 as c_int as c_uint,
        0x2eda as c_int as c_uint,
        0x2bed as c_int as c_uint,
    ],
    [
        0x1689 as c_int as c_uint,
        0x13be as c_int as c_uint,
        0x1ce7 as c_int as c_uint,
        0x19d0 as c_int as c_uint,
        0x762 as c_int as c_uint,
        0x255 as c_int as c_uint,
        0xd0c as c_int as c_uint,
        0x83b as c_int as c_uint,
    ],
];
#[inline]
pub fn QRspec_getFormatInfo(
    mut mask: c_int,
    mut level: QRecLevel,
) -> c_uint { unsafe {
    if mask < 0 as c_int || mask > 7 as c_int {
        return 0 as c_uint;
    }
    return formatInfo[level as usize][mask as usize];
} }

fn QRspec_createFrame(
    mut version: c_int,
) -> *mut c_uchar { unsafe {
    let mut frame: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut p: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut q: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut width: c_int = 0;
    let mut x: c_int = 0;
    let mut y: c_int = 0;
    let mut verinfo: c_uint = 0;
    let mut v: c_uint = 0;
    width = qrspecCapacity[version as usize].width;
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
    putFinderPattern(
        frame,
        width,
        width - 7 as c_int,
        0 as c_int,
    );
    putFinderPattern(
        frame,
        width,
        0 as c_int,
        width - 7 as c_int,
    );
    p = frame;
    q = frame.offset((width * (width - 7 as c_int)) as isize);
    y = 0 as c_int;
    while y < 7 as c_int {
        *p.offset(7 as c_int as isize) = 0xc0 as c_uchar;
        *p.offset((width - 8 as c_int) as isize) = 0xc0 as c_uchar;
        *q.offset(7 as c_int as isize) = 0xc0 as c_uchar;
        p = p.offset(width as isize);
        q = q.offset(width as isize);
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
            .offset(-(8 as c_int as isize)) as *mut c_void,
        0xc0 as c_int,
        8 as size_t,
    );
    memset(
        frame.offset((width * (width - 8 as c_int)) as isize)
            as *mut c_void,
        0xc0 as c_int,
        8 as size_t,
    );
    memset(
        frame.offset((width * 8 as c_int) as isize) as *mut c_void,
        0x84 as c_int,
        9 as size_t,
    );
    memset(
        frame
            .offset((width * 9 as c_int) as isize)
            .offset(-(8 as c_int as isize)) as *mut c_void,
        0x84 as c_int,
        8 as size_t,
    );
    p = frame.offset(8 as c_int as isize);
    y = 0 as c_int;
    while y < 8 as c_int {
        *p = 0x84 as c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame
        .offset((width * (width - 7 as c_int)) as isize)
        .offset(8 as c_int as isize);
    y = 0 as c_int;
    while y < 7 as c_int {
        *p = 0x84 as c_uchar;
        p = p.offset(width as isize);
        y += 1;
    }
    p = frame
        .offset((width * 6 as c_int) as isize)
        .offset(8 as c_int as isize);
    q = frame
        .offset((width * 8 as c_int) as isize)
        .offset(6 as c_int as isize);
    x = 1 as c_int;
    while x < width - 15 as c_int {
        *p = (0x90 as c_int | x & 1 as c_int) as c_uchar;
        *q = (0x90 as c_int | x & 1 as c_int) as c_uchar;
        p = p.offset(1);
        q = q.offset(width as isize);
        x += 1;
    }
    QRspec_putAlignmentPattern(version, frame, width);
    if version >= 7 as c_int {
        verinfo = QRspec_getVersionPattern(version);
        p = frame.offset((width * (width - 11 as c_int)) as isize);
        v = verinfo;
        x = 0 as c_int;
        while x < 6 as c_int {
            y = 0 as c_int;
            while y < 3 as c_int {
                *p.offset((width * y + x) as isize) = (0x88 as c_uint
                    | v & 1 as c_uint)
                    as c_uchar;
                v = v >> 1 as c_int;
                y += 1;
            }
            x += 1;
        }
        p = frame
            .offset(width as isize)
            .offset(-(11 as c_int as isize));
        v = verinfo;
        y = 0 as c_int;
        while y < 6 as c_int {
            x = 0 as c_int;
            while x < 3 as c_int {
                *p.offset(x as isize) = (0x88 as c_uint | v & 1 as c_uint)
                    as c_uchar;
                v = v >> 1 as c_int;
                x += 1;
            }
            p = p.offset(width as isize);
            y += 1;
        }
    }
    *frame.offset((width * (width - 8 as c_int) + 8 as c_int) as isize) =
        0x81 as c_uchar;
    return frame;
} }
#[inline]
pub fn QRspec_newFrame(
    mut version: c_int,
) -> *mut c_uchar { {
    if version < 1 as c_int || version > QRSPEC_VERSION_MAX {
        return ::core::ptr::null_mut::<c_uchar>();
    }
    return QRspec_createFrame(version);
} }
