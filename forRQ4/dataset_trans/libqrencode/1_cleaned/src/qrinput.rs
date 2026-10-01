use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    fn MQRspec_lengthIndicator(
        mode: QRencodeMode,
        version: c_int,
    ) -> c_int;
    fn MQRspec_maximumWords(mode: QRencodeMode, version: c_int) -> c_int;
    fn BitStream_new() -> *mut BitStream;
    fn BitStream_appendNum(
        bstream: *mut BitStream,
        bits: size_t,
        num: c_uint,
    ) -> c_int;
    fn BitStream_appendBytes(
        bstream: *mut BitStream,
        size: size_t,
        data: *mut c_uchar,
    ) -> c_int;
    fn BitStream_toByte(bstream: *mut BitStream) -> *mut c_uchar;
    fn BitStream_free(bstream: *mut BitStream);
    fn QRspec_getDataLength(version: c_int, level: QRecLevel) -> c_int;
    fn QRspec_getMinimumVersion(size: c_int, level: QRecLevel) -> c_int;
    fn QRspec_maximumWords(mode: QRencodeMode, version: c_int) -> c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput {
    pub version: c_int,
    pub level: QRecLevel,
    pub head: *mut QRinput_List,
    pub tail: *mut QRinput_List,
    pub mqr: c_int,
    pub fnc1: c_int,
    pub appid: c_uchar,
}
pub type QRinput_List = _QRinput_List;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_List {
    pub mode: QRencodeMode,
    pub size: c_int,
    pub data: *mut c_uchar,
    pub bstream: *mut BitStream,
    pub next: *mut QRinput_List,
}

pub type QRinput = _QRinput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_Struct {
    pub size: c_int,
    pub parity: c_int,
    pub head: *mut QRinput_InputList,
    pub tail: *mut QRinput_InputList,
}
pub type QRinput_InputList = _QRinput_InputList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_InputList {
    pub input: *mut QRinput,
    pub next: *mut QRinput_InputList,
}
pub type QRinput_Struct = _QRinput_Struct;

pub const MODE_INDICATOR_SIZE: c_int = 4 as c_int;
pub const STRUCTURE_HEADER_SIZE: c_int = 20 as c_int;
pub const MAX_STRUCTURED_SYMBOLS: c_int = 16 as c_int;
pub const MQRSPEC_MODEID_NUM: c_int = 0 as c_int;
pub const MQRSPEC_MODEID_AN: c_int = 1 as c_int;
pub const MQRSPEC_MODEID_8: c_int = 2 as c_int;
pub const MQRSPEC_MODEID_KANJI: c_int = 3 as c_int;
#[no_mangle]
pub unsafe extern "C" fn QRinput_isSplittableMode(mut mode: QRencodeMode) -> c_int {
    return (mode as c_int >= QR_MODE_NUM as c_int
        && mode as c_int <= QR_MODE_KANJI as c_int)
        as c_int;
}
unsafe extern "C" fn QRinput_List_newEntry(
    mut mode: QRencodeMode,
    mut size: c_int,
    mut data: *const c_uchar,
) -> *mut QRinput_List {
    let mut entry: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    if QRinput_check(mode, size, data) != 0 {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRinput_List>();
    }
    entry = malloc(::core::mem::size_of::<QRinput_List>() as size_t) as *mut QRinput_List;
    if entry.is_null() {
        return ::core::ptr::null_mut::<QRinput_List>();
    }
    (*entry).mode = mode;
    (*entry).size = size;
    (*entry).data = ::core::ptr::null_mut::<c_uchar>();
    if size > 0 as c_int {
        (*entry).data = malloc(size as size_t) as *mut c_uchar;
        if (*entry).data.is_null() {
            free(entry as *mut c_void);
            return ::core::ptr::null_mut::<QRinput_List>();
        }
        memcpy(
            (*entry).data as *mut c_void,
            data as *const c_void,
            size as size_t,
        );
    }
    (*entry).bstream = ::core::ptr::null_mut::<BitStream>();
    (*entry).next = ::core::ptr::null_mut::<QRinput_List>();
    return entry;
}
unsafe extern "C" fn QRinput_List_freeEntry(mut entry: *mut QRinput_List) {
    if !entry.is_null() {
        free((*entry).data as *mut c_void);
        BitStream_free((*entry).bstream);
        free(entry as *mut c_void);
    }
}
unsafe extern "C" fn QRinput_List_dup(mut entry: *mut QRinput_List) -> *mut QRinput_List {
    let mut n: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    n = malloc(::core::mem::size_of::<QRinput_List>() as size_t) as *mut QRinput_List;
    if n.is_null() {
        return ::core::ptr::null_mut::<QRinput_List>();
    }
    (*n).mode = (*entry).mode;
    (*n).size = (*entry).size;
    (*n).data = malloc((*n).size as size_t) as *mut c_uchar;
    if (*n).data.is_null() {
        free(n as *mut c_void);
        return ::core::ptr::null_mut::<QRinput_List>();
    }
    memcpy(
        (*n).data as *mut c_void,
        (*entry).data as *const c_void,
        (*entry).size as size_t,
    );
    (*n).bstream = ::core::ptr::null_mut::<BitStream>();
    (*n).next = ::core::ptr::null_mut::<QRinput_List>();
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_new() -> *mut QRinput {
    return QRinput_new2(0 as c_int, QR_ECLEVEL_L);
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_new2(
    mut version: c_int,
    mut level: QRecLevel,
) -> *mut QRinput {
    let mut input: *mut QRinput = ::core::ptr::null_mut::<QRinput>();
    if version < 0 as c_int
        || version > QRSPEC_VERSION_MAX
        || (level as c_uint) < 0 as c_uint
        || level as c_uint > QR_ECLEVEL_H as c_int as c_uint
    {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRinput>();
    }
    input = malloc(::core::mem::size_of::<QRinput>() as size_t) as *mut QRinput;
    if input.is_null() {
        return ::core::ptr::null_mut::<QRinput>();
    }
    (*input).head = ::core::ptr::null_mut::<QRinput_List>();
    (*input).tail = ::core::ptr::null_mut::<QRinput_List>();
    (*input).version = version;
    (*input).level = level;
    (*input).mqr = 0 as c_int;
    (*input).fnc1 = 0 as c_int;
    return input;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_newMQR(
    mut version: c_int,
    mut level: QRecLevel,
) -> *mut QRinput {
    let mut input: *mut QRinput = ::core::ptr::null_mut::<QRinput>();
    if !(version <= 0 as c_int || version > MQRSPEC_VERSION_MAX) {
        if !(MQRspec_getECCLength(version, level) == 0 as c_int) {
            input = QRinput_new2(version, level);
            if input.is_null() {
                return ::core::ptr::null_mut::<QRinput>();
            }
            (*input).mqr = 1 as c_int;
            return input;
        }
    }
    *__errno_location() = EINVAL;
    return ::core::ptr::null_mut::<QRinput>();
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_getVersion(mut input: *mut QRinput) -> c_int {
    return (*input).version;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_setVersion(
    mut input: *mut QRinput,
    mut version: c_int,
) -> c_int {
    if (*input).mqr != 0 || version < 0 as c_int || version > QRSPEC_VERSION_MAX {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    (*input).version = version;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_getErrorCorrectionLevel(mut input: *mut QRinput) -> QRecLevel {
    return (*input).level;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_setErrorCorrectionLevel(
    mut input: *mut QRinput,
    mut level: QRecLevel,
) -> c_int {
    if (*input).mqr != 0
        || level as c_uint > QR_ECLEVEL_H as c_int as c_uint
    {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    (*input).level = level;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_setVersionAndErrorCorrectionLevel(
    mut input: *mut QRinput,
    mut version: c_int,
    mut level: QRecLevel,
) -> c_int {
    let mut current_block: u64;
    if (*input).mqr != 0 {
        if version <= 0 as c_int || version > MQRSPEC_VERSION_MAX {
            current_block = 15906867335992566915;
        } else if MQRspec_getECCLength(version, level) == 0 as c_int {
            current_block = 15906867335992566915;
        } else {
            current_block = 11875828834189669668;
        }
    } else if version < 0 as c_int || version > QRSPEC_VERSION_MAX {
        current_block = 15906867335992566915;
    } else if level as c_uint
        > QR_ECLEVEL_H as c_int as c_uint
    {
        current_block = 15906867335992566915;
    } else {
        current_block = 11875828834189669668;
    }
    match current_block {
        11875828834189669668 => {
            (*input).version = version;
            (*input).level = level;
            return 0 as c_int;
        }
        _ => {
            *__errno_location() = EINVAL;
            return -(1 as c_int);
        }
    };
}
unsafe extern "C" fn QRinput_appendEntry(mut input: *mut QRinput, mut entry: *mut QRinput_List) {
    if (*input).tail.is_null() {
        (*input).head = entry;
        (*input).tail = entry;
    } else {
        (*(*input).tail).next = entry;
        (*input).tail = entry;
    }
    (*entry).next = ::core::ptr::null_mut::<QRinput_List>();
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_append(
    mut input: *mut QRinput,
    mut mode: QRencodeMode,
    mut size: c_int,
    mut data: *const c_uchar,
) -> c_int {
    let mut entry: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    entry = QRinput_List_newEntry(mode, size, data);
    if entry.is_null() {
        return -(1 as c_int);
    }
    QRinput_appendEntry(input, entry);
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_insertStructuredAppendHeader(
    mut input: *mut QRinput,
    mut size: c_int,
    mut number: c_int,
    mut parity: c_uchar,
) -> c_int {
    let mut entry: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut buf: [c_uchar; 3] = [0; 3];
    if size > MAX_STRUCTURED_SYMBOLS {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    if number <= 0 as c_int || number > size {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    buf[0 as c_int as usize] = size as c_uchar;
    buf[1 as c_int as usize] = number as c_uchar;
    buf[2 as c_int as usize] = parity;
    entry = QRinput_List_newEntry(
        QR_MODE_STRUCTURE,
        3 as c_int,
        &raw mut buf as *mut c_uchar,
    );
    if entry.is_null() {
        return -(1 as c_int);
    }
    (*entry).next = (*input).head;
    (*input).head = entry;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_appendECIheader(
    mut input: *mut QRinput,
    mut ecinum: c_uint,
) -> c_int {
    let mut data: [c_uchar; 4] = [0; 4];
    if ecinum > 999999 as c_int as c_uint {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    data[0 as c_int as usize] =
        (ecinum & 0xff as c_uint) as c_uchar;
    data[1 as c_int as usize] =
        (ecinum >> 8 as c_int & 0xff as c_uint) as c_uchar;
    data[2 as c_int as usize] =
        (ecinum >> 16 as c_int & 0xff as c_uint) as c_uchar;
    data[3 as c_int as usize] =
        (ecinum >> 24 as c_int & 0xff as c_uint) as c_uchar;
    return QRinput_append(
        input,
        QR_MODE_ECI,
        4 as c_int,
        &raw mut data as *mut c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_free(mut input: *mut QRinput) {
    let mut list: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut next: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    if !input.is_null() {
        list = (*input).head;
        while !list.is_null() {
            next = (*list).next;
            QRinput_List_freeEntry(list);
            list = next;
        }
        free(input as *mut c_void);
    }
}
unsafe extern "C" fn QRinput_calcParity(mut input: *mut QRinput) -> c_uchar {
    let mut parity: c_uchar = 0 as c_uchar;
    let mut list: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut i: c_int = 0;
    list = (*input).head;
    while !list.is_null() {
        if (*list).mode as c_int != QR_MODE_STRUCTURE as c_int {
            i = (*list).size - 1 as c_int;
            while i >= 0 as c_int {
                parity = (parity as c_int
                    ^ *(*list).data.offset(i as isize) as c_int)
                    as c_uchar;
                i -= 1;
            }
        }
        list = (*list).next;
    }
    return parity;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_dup(mut input: *mut QRinput) -> *mut QRinput {
    let mut n: *mut QRinput = ::core::ptr::null_mut::<QRinput>();
    let mut list: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut e: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    if (*input).mqr != 0 {
        n = QRinput_newMQR((*input).version, (*input).level);
    } else {
        n = QRinput_new2((*input).version, (*input).level);
    }
    if n.is_null() {
        return ::core::ptr::null_mut::<QRinput>();
    }
    list = (*input).head;
    while !list.is_null() {
        e = QRinput_List_dup(list);
        if e.is_null() {
            QRinput_free(n);
            return ::core::ptr::null_mut::<QRinput>();
        }
        QRinput_appendEntry(n, e);
        list = (*list).next;
    }
    return n;
}
unsafe extern "C" fn QRinput_checkModeNum(
    mut size: c_int,
    mut data: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < size {
        if (*data.offset(i as isize) as c_int) < '0' as i32
            || *data.offset(i as isize) as c_int > '9' as i32
        {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_estimateBitsModeNum(
    mut size: c_int,
) -> c_int {
    let mut w: c_int = 0;
    let mut bits: c_int = 0;
    w = size / 3 as c_int;
    bits = w * 10 as c_int;
    match size - w * 3 as c_int {
        1 => {
            bits += 4 as c_int;
        }
        2 => {
            bits += 7 as c_int;
        }
        _ => {}
    }
    return bits;
}
unsafe extern "C" fn QRinput_encodeModeNum(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: c_int,
    mut mqr: c_int,
) -> c_int {
    let mut words: c_int = 0;
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    let mut val: c_uint = 0;
    if mqr != 0 {
        if version > 1 as c_int {
            ret = BitStream_appendNum(
                bstream,
                (version - 1 as c_int) as size_t,
                MQRSPEC_MODEID_NUM as c_uint,
            );
            if ret < 0 as c_int {
                return -(1 as c_int);
            }
        }
        ret = BitStream_appendNum(
            bstream,
            MQRspec_lengthIndicator(QR_MODE_NUM, version) as size_t,
            (*entry).size as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as size_t,
            QRSPEC_MODEID_NUM as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            QRspec_lengthIndicator(QR_MODE_NUM, version) as size_t,
            (*entry).size as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    }
    words = (*entry).size / 3 as c_int;
    i = 0 as c_int;
    while i < words {
        val = ((*(*entry).data.offset((i * 3 as c_int) as isize) as c_int
            - '0' as i32) as c_uint)
            .wrapping_mul(100 as c_uint);
        val = val.wrapping_add(
            ((*(*entry)
                .data
                .offset((i * 3 as c_int + 1 as c_int) as isize)
                as c_int
                - '0' as i32) as c_uint)
                .wrapping_mul(10 as c_uint),
        );
        val = val.wrapping_add(
            (*(*entry)
                .data
                .offset((i * 3 as c_int + 2 as c_int) as isize)
                as c_int
                - '0' as i32) as c_uint,
        );
        ret = BitStream_appendNum(bstream, 10 as size_t, val);
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        i += 1;
    }
    if (*entry).size - words * 3 as c_int == 1 as c_int {
        val = (*(*entry)
            .data
            .offset((words * 3 as c_int) as isize)
            as c_int
            - '0' as i32) as c_uint;
        ret = BitStream_appendNum(bstream, 4 as size_t, val);
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    } else if (*entry).size - words * 3 as c_int == 2 as c_int {
        val = ((*(*entry)
            .data
            .offset((words * 3 as c_int) as isize)
            as c_int
            - '0' as i32) as c_uint)
            .wrapping_mul(10 as c_uint);
        val = val.wrapping_add(
            (*(*entry)
                .data
                .offset((words * 3 as c_int + 1 as c_int) as isize)
                as c_int
                - '0' as i32) as c_uint,
        );
        ret = BitStream_appendNum(bstream, 7 as size_t, val);
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    }
    return 0 as c_int;
}
#[no_mangle]
pub static mut QRinput_anTable: [c_schar; 128] = [
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    36 as c_int as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    37 as c_int as c_schar,
    38 as c_int as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    39 as c_int as c_schar,
    40 as c_int as c_schar,
    -(1 as c_int) as c_schar,
    41 as c_int as c_schar,
    42 as c_int as c_schar,
    43 as c_int as c_schar,
    0 as c_int as c_schar,
    1 as c_int as c_schar,
    2 as c_int as c_schar,
    3 as c_int as c_schar,
    4 as c_int as c_schar,
    5 as c_int as c_schar,
    6 as c_int as c_schar,
    7 as c_int as c_schar,
    8 as c_int as c_schar,
    9 as c_int as c_schar,
    44 as c_int as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    10 as c_int as c_schar,
    11 as c_int as c_schar,
    12 as c_int as c_schar,
    13 as c_int as c_schar,
    14 as c_int as c_schar,
    15 as c_int as c_schar,
    16 as c_int as c_schar,
    17 as c_int as c_schar,
    18 as c_int as c_schar,
    19 as c_int as c_schar,
    20 as c_int as c_schar,
    21 as c_int as c_schar,
    22 as c_int as c_schar,
    23 as c_int as c_schar,
    24 as c_int as c_schar,
    25 as c_int as c_schar,
    26 as c_int as c_schar,
    27 as c_int as c_schar,
    28 as c_int as c_schar,
    29 as c_int as c_schar,
    30 as c_int as c_schar,
    31 as c_int as c_schar,
    32 as c_int as c_schar,
    33 as c_int as c_schar,
    34 as c_int as c_schar,
    35 as c_int as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
    -(1 as c_int) as c_schar,
];
unsafe extern "C" fn QRinput_checkModeAn(
    mut size: c_int,
    mut data: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < size {
        if (if *data.offset(i as isize) as c_int & 0x80 as c_int != 0 {
            -(1 as c_int)
        } else {
            QRinput_anTable[*data.offset(i as isize) as c_int as usize]
                as c_int
        }) < 0 as c_int
        {
            return -(1 as c_int);
        }
        i += 1;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_estimateBitsModeAn(
    mut size: c_int,
) -> c_int {
    let mut w: c_int = 0;
    let mut bits: c_int = 0;
    w = size / 2 as c_int;
    bits = w * 11 as c_int;
    if size & 1 as c_int != 0 {
        bits += 6 as c_int;
    }
    return bits;
}
unsafe extern "C" fn QRinput_encodeModeAn(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: c_int,
    mut mqr: c_int,
) -> c_int {
    let mut words: c_int = 0;
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    let mut val: c_uint = 0;
    if mqr != 0 {
        if version < 2 as c_int {
            *__errno_location() = ERANGE;
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as c_int) as size_t,
            MQRSPEC_MODEID_AN as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            MQRspec_lengthIndicator(QR_MODE_AN, version) as size_t,
            (*entry).size as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as size_t,
            QRSPEC_MODEID_AN as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            QRspec_lengthIndicator(QR_MODE_AN, version) as size_t,
            (*entry).size as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    }
    words = (*entry).size / 2 as c_int;
    i = 0 as c_int;
    while i < words {
        val = ((if *(*entry).data.offset((i * 2 as c_int) as isize)
            as c_int
            & 0x80 as c_int
            != 0
        {
            -(1 as c_int)
        } else {
            QRinput_anTable[*(*entry).data.offset((i * 2 as c_int) as isize)
                as c_int as usize] as c_int
        }) as c_uint)
            .wrapping_mul(45 as c_uint);
        val = val.wrapping_add(
            (if *(*entry)
                .data
                .offset((i * 2 as c_int + 1 as c_int) as isize)
                as c_int
                & 0x80 as c_int
                != 0
            {
                -(1 as c_int)
            } else {
                QRinput_anTable[*(*entry)
                    .data
                    .offset((i * 2 as c_int + 1 as c_int) as isize)
                    as c_int as usize] as c_int
            }) as c_uint,
        );
        ret = BitStream_appendNum(bstream, 11 as size_t, val);
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        i += 1;
    }
    if (*entry).size & 1 as c_int != 0 {
        val = (if *(*entry)
            .data
            .offset((words * 2 as c_int) as isize)
            as c_int
            & 0x80 as c_int
            != 0
        {
            -(1 as c_int)
        } else {
            QRinput_anTable[*(*entry)
                .data
                .offset((words * 2 as c_int) as isize)
                as c_int as usize] as c_int
        }) as c_uint;
        ret = BitStream_appendNum(bstream, 6 as size_t, val);
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_estimateBitsMode8(
    mut size: c_int,
) -> c_int {
    return size * 8 as c_int;
}
unsafe extern "C" fn QRinput_encodeMode8(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: c_int,
    mut mqr: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    if mqr != 0 {
        if version < 3 as c_int {
            *__errno_location() = ERANGE;
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as c_int) as size_t,
            MQRSPEC_MODEID_8 as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            MQRspec_lengthIndicator(QR_MODE_8, version) as size_t,
            (*entry).size as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    } else {
        ret = BitStream_appendNum(bstream, 4 as size_t, QRSPEC_MODEID_8 as c_uint);
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            QRspec_lengthIndicator(QR_MODE_8, version) as size_t,
            (*entry).size as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    }
    ret = BitStream_appendBytes(bstream, (*entry).size as size_t, (*entry).data);
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_estimateBitsModeKanji(
    mut size: c_int,
) -> c_int {
    return size / 2 as c_int * 13 as c_int;
}
unsafe extern "C" fn QRinput_checkModeKanji(
    mut size: c_int,
    mut data: *const c_uchar,
) -> c_int {
    let mut i: c_int = 0;
    let mut val: c_uint = 0;
    if size & 1 as c_int != 0 {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < size {
        val = (*data.offset(i as isize) as c_uint) << 8 as c_int
            | *data.offset((i + 1 as c_int) as isize) as c_uint;
        if val < 0x8140 as c_uint
            || val > 0x9ffc as c_uint && val < 0xe040 as c_uint
            || val > 0xebbf as c_uint
        {
            return -(1 as c_int);
        }
        i += 2 as c_int;
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_encodeModeKanji(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: c_int,
    mut mqr: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let mut i: c_int = 0;
    let mut val: c_uint = 0;
    let mut h: c_uint = 0;
    if mqr != 0 {
        if version < 2 as c_int {
            *__errno_location() = ERANGE;
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            (version - 1 as c_int) as size_t,
            MQRSPEC_MODEID_KANJI as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            MQRspec_lengthIndicator(QR_MODE_KANJI, version) as size_t,
            ((*entry).size as c_uint).wrapping_div(2 as c_uint),
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    } else {
        ret = BitStream_appendNum(
            bstream,
            4 as size_t,
            QRSPEC_MODEID_KANJI as c_uint,
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        ret = BitStream_appendNum(
            bstream,
            QRspec_lengthIndicator(QR_MODE_KANJI, version) as size_t,
            ((*entry).size as c_uint).wrapping_div(2 as c_uint),
        );
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    }
    i = 0 as c_int;
    while i < (*entry).size {
        val = (*(*entry).data.offset(i as isize) as c_uint) << 8 as c_int
            | *(*entry).data.offset((i + 1 as c_int) as isize) as c_uint;
        if val <= 0x9ffc as c_uint {
            val = val.wrapping_sub(0x8140 as c_uint);
        } else {
            val = val.wrapping_sub(0xc140 as c_uint);
        }
        h = (val >> 8 as c_int).wrapping_mul(0xc0 as c_uint);
        val = (val & 0xff as c_uint).wrapping_add(h);
        ret = BitStream_appendNum(bstream, 13 as size_t, val);
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
        i += 2 as c_int;
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_encodeModeStructure(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut mqr: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    if mqr != 0 {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as size_t,
        QRSPEC_MODEID_STRUCTURE as c_uint,
    );
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as size_t,
        (*(*entry).data.offset(1 as c_int as isize) as c_uint)
            .wrapping_sub(1 as c_uint),
    );
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as size_t,
        (*(*entry).data.offset(0 as c_int as isize) as c_uint)
            .wrapping_sub(1 as c_uint),
    );
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    ret = BitStream_appendNum(
        bstream,
        8 as size_t,
        *(*entry).data.offset(2 as c_int as isize) as c_uint,
    );
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_checkModeFNC1Second(
    mut size: c_int,
) -> c_int {
    if size != 1 as c_int {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_encodeModeFNC1Second(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
) -> c_int {
    let mut ret: c_int = 0;
    ret = BitStream_appendNum(
        bstream,
        4 as size_t,
        QRSPEC_MODEID_FNC1SECOND as c_uint,
    );
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    ret = BitStream_appendBytes(bstream, 1 as size_t, (*entry).data);
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_decodeECIfromByteArray(
    mut data: *mut c_uchar,
) -> c_uint {
    let mut i: c_int = 0;
    let mut ecinum: c_uint = 0;
    ecinum = 0 as c_uint;
    i = 0 as c_int;
    while i < 4 as c_int {
        ecinum = ecinum << 8 as c_int;
        ecinum |= *data.offset((3 as c_int - i) as isize) as c_uint;
        i += 1;
    }
    return ecinum;
}
unsafe extern "C" fn QRinput_estimateBitsModeECI(
    mut data: *mut c_uchar,
) -> c_int {
    let mut ecinum: c_uint = 0;
    ecinum = QRinput_decodeECIfromByteArray(data);
    if ecinum < 128 as c_uint {
        return MODE_INDICATOR_SIZE + 8 as c_int;
    } else if ecinum < 16384 as c_uint {
        return MODE_INDICATOR_SIZE + 16 as c_int;
    } else {
        return MODE_INDICATOR_SIZE + 24 as c_int;
    };
}
unsafe extern "C" fn QRinput_encodeModeECI(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
) -> c_int {
    let mut ret: c_int = 0;
    let mut words: c_int = 0;
    let mut ecinum: c_uint = 0;
    let mut code: c_uint = 0;
    ecinum = QRinput_decodeECIfromByteArray((*entry).data);
    if ecinum < 128 as c_uint {
        words = 1 as c_int;
        code = ecinum;
    } else if ecinum < 16384 as c_uint {
        words = 2 as c_int;
        code = (0x8000 as c_uint).wrapping_add(ecinum);
    } else {
        words = 3 as c_int;
        code = (0xc0000 as c_int as c_uint).wrapping_add(ecinum);
    }
    ret = BitStream_appendNum(
        bstream,
        4 as size_t,
        QRSPEC_MODEID_ECI as c_uint,
    );
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    ret = BitStream_appendNum(bstream, (words as size_t).wrapping_mul(8 as size_t), code);
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_check(
    mut mode: QRencodeMode,
    mut size: c_int,
    mut data: *const c_uchar,
) -> c_int {
    if mode as c_int == QR_MODE_FNC1FIRST as c_int
        && size < 0 as c_int
        || size <= 0 as c_int
    {
        return -(1 as c_int);
    }
    match mode as c_int {
        0 => return QRinput_checkModeNum(size, data as *const c_char),
        1 => return QRinput_checkModeAn(size, data as *const c_char),
        3 => return QRinput_checkModeKanji(size, data),
        2 => return 0 as c_int,
        4 => return 0 as c_int,
        5 => return 0 as c_int,
        6 => return 0 as c_int,
        7 => return QRinput_checkModeFNC1Second(size),
        -1 | _ => {}
    }
    return -(1 as c_int);
}
unsafe extern "C" fn QRinput_estimateBitStreamSizeOfEntry(
    mut entry: *mut QRinput_List,
    mut version: c_int,
    mut mqr: c_int,
) -> c_int {
    let mut bits: c_int = 0 as c_int;
    let mut l: c_int = 0;
    let mut m: c_int = 0;
    let mut num: c_int = 0;
    if version == 0 as c_int {
        version = 1 as c_int;
    }
    match (*entry).mode as c_int {
        0 => {
            bits = QRinput_estimateBitsModeNum((*entry).size);
        }
        1 => {
            bits = QRinput_estimateBitsModeAn((*entry).size);
        }
        2 => {
            bits = QRinput_estimateBitsMode8((*entry).size);
        }
        3 => {
            bits = QRinput_estimateBitsModeKanji((*entry).size);
        }
        4 => return STRUCTURE_HEADER_SIZE,
        5 => {
            bits = QRinput_estimateBitsModeECI((*entry).data);
        }
        6 => return MODE_INDICATOR_SIZE,
        7 => return MODE_INDICATOR_SIZE + 8 as c_int,
        _ => return 0 as c_int,
    }
    if mqr != 0 {
        l = MQRspec_lengthIndicator((*entry).mode, version);
        m = version - 1 as c_int;
        bits += l + m;
    } else {
        l = QRspec_lengthIndicator((*entry).mode, version);
        m = (1 as c_int) << l;
        if (*entry).mode as c_int == QR_MODE_KANJI as c_int {
            num = ((*entry).size / 2 as c_int + m - 1 as c_int) / m;
        } else {
            num = ((*entry).size + m - 1 as c_int) / m;
        }
        bits += num * (MODE_INDICATOR_SIZE + l);
    }
    return bits;
}
unsafe extern "C" fn QRinput_estimateBitStreamSize(
    mut input: *mut QRinput,
    mut version: c_int,
) -> c_int {
    let mut list: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut bits: c_int = 0 as c_int;
    list = (*input).head;
    while !list.is_null() {
        bits += QRinput_estimateBitStreamSizeOfEntry(list, version, (*input).mqr);
        list = (*list).next;
    }
    return bits;
}
unsafe extern "C" fn QRinput_estimateVersion(mut input: *mut QRinput) -> c_int {
    let mut bits: c_int = 0;
    let mut version: c_int = 0;
    let mut prev: c_int = 0;
    version = 0 as c_int;
    loop {
        prev = version;
        bits = QRinput_estimateBitStreamSize(input, prev);
        version = QRspec_getMinimumVersion(
            (bits + 7 as c_int) / 8 as c_int,
            (*input).level,
        );
        if prev == 0 as c_int && version > 1 as c_int {
            version -= 1;
        }
        if !(version > prev) {
            break;
        }
    }
    return version;
}
unsafe extern "C" fn QRinput_lengthOfCode(
    mut mode: QRencodeMode,
    mut version: c_int,
    mut bits: c_int,
) -> c_int {
    let mut payload: c_int = 0;
    let mut size: c_int = 0;
    let mut chunks: c_int = 0;
    let mut remain: c_int = 0;
    let mut maxsize: c_int = 0;
    payload = bits - 4 as c_int - QRspec_lengthIndicator(mode, version);
    match mode as c_int {
        0 => {
            chunks = payload / 10 as c_int;
            remain = payload - chunks * 10 as c_int;
            size = chunks * 3 as c_int;
            if remain >= 7 as c_int {
                size += 2 as c_int;
            } else if remain >= 4 as c_int {
                size += 1 as c_int;
            }
        }
        1 => {
            chunks = payload / 11 as c_int;
            remain = payload - chunks * 11 as c_int;
            size = chunks * 2 as c_int;
            if remain >= 6 as c_int {
                size += 1;
            }
        }
        2 => {
            size = payload / 8 as c_int;
        }
        3 => {
            size = payload / 13 as c_int * 2 as c_int;
        }
        4 => {
            size = payload / 8 as c_int;
        }
        _ => {
            size = 0 as c_int;
        }
    }
    maxsize = QRspec_maximumWords(mode, version);
    if size < 0 as c_int {
        size = 0 as c_int;
    }
    if maxsize > 0 as c_int && size > maxsize {
        size = maxsize;
    }
    return size;
}
unsafe extern "C" fn QRinput_encodeBitStream(
    mut entry: *mut QRinput_List,
    mut bstream: *mut BitStream,
    mut version: c_int,
    mut mqr: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut words: c_int = 0;
    let mut ret: c_int = 0;
    let mut st1: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut st2: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut prevsize: c_int = 0;
    prevsize = (*bstream).length as c_int;
    if mqr != 0 {
        words = MQRspec_maximumWords((*entry).mode, version);
    } else {
        words = QRspec_maximumWords((*entry).mode, version);
    }
    if words != 0 as c_int && (*entry).size > words {
        st1 = QRinput_List_newEntry((*entry).mode, words, (*entry).data);
        if st1.is_null() {
            current_block = 4454329335403284158;
        } else {
            st2 = QRinput_List_newEntry(
                (*entry).mode,
                (*entry).size - words,
                (*entry).data.offset(words as isize) as *mut c_uchar,
            );
            if st2.is_null() {
                current_block = 4454329335403284158;
            } else {
                ret = QRinput_encodeBitStream(st1, bstream, version, mqr);
                if ret < 0 as c_int {
                    current_block = 4454329335403284158;
                } else {
                    ret = QRinput_encodeBitStream(st2, bstream, version, mqr);
                    if ret < 0 as c_int {
                        current_block = 4454329335403284158;
                    } else {
                        QRinput_List_freeEntry(st1);
                        QRinput_List_freeEntry(st2);
                        current_block = 4775909272756257391;
                    }
                }
            }
        }
        match current_block {
            4775909272756257391 => {}
            _ => {
                QRinput_List_freeEntry(st1);
                QRinput_List_freeEntry(st2);
                return -(1 as c_int);
            }
        }
    } else {
        ret = 0 as c_int;
        match (*entry).mode as c_int {
            0 => {
                ret = QRinput_encodeModeNum(entry, bstream, version, mqr);
            }
            1 => {
                ret = QRinput_encodeModeAn(entry, bstream, version, mqr);
            }
            2 => {
                ret = QRinput_encodeMode8(entry, bstream, version, mqr);
            }
            3 => {
                ret = QRinput_encodeModeKanji(entry, bstream, version, mqr);
            }
            4 => {
                ret = QRinput_encodeModeStructure(entry, bstream, mqr);
            }
            5 => {
                ret = QRinput_encodeModeECI(entry, bstream);
            }
            7 => {
                ret = QRinput_encodeModeFNC1Second(entry, bstream);
            }
            _ => {}
        }
        if ret < 0 as c_int {
            return -(1 as c_int);
        }
    }
    return (*bstream).length as c_int - prevsize;
}
unsafe extern "C" fn QRinput_createBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> c_int {
    let mut list: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut bits: c_int = 0;
    let mut total: c_int = 0 as c_int;
    list = (*input).head;
    while !list.is_null() {
        bits = QRinput_encodeBitStream(list, bstream, (*input).version, (*input).mqr);
        if bits < 0 as c_int {
            return -(1 as c_int);
        }
        total += bits;
        list = (*list).next;
    }
    return total;
}
unsafe extern "C" fn QRinput_convertData(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> c_int {
    let mut bits: c_int = 0;
    let mut ver: c_int = 0;
    ver = QRinput_estimateVersion(input);
    if ver > QRinput_getVersion(input) {
        QRinput_setVersion(input, ver);
    }
    loop {
        (*bstream).length = 0 as size_t;
        bits = QRinput_createBitStream(input, bstream);
        if bits < 0 as c_int {
            return -(1 as c_int);
        }
        ver = QRspec_getMinimumVersion(
            (bits + 7 as c_int) / 8 as c_int,
            (*input).level,
        );
        if !(ver > QRinput_getVersion(input)) {
            break;
        }
        QRinput_setVersion(input, ver);
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_appendPaddingBit(
    mut bstream: *mut BitStream,
    mut input: *mut QRinput,
) -> c_int {
    let mut bits: c_int = 0;
    let mut maxbits: c_int = 0;
    let mut words: c_int = 0;
    let mut maxwords: c_int = 0;
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    let mut padlen: c_int = 0;
    bits = (*bstream).length as c_int;
    maxwords = QRspec_getDataLength((*input).version, (*input).level);
    maxbits = maxwords * 8 as c_int;
    if maxbits < bits {
        *__errno_location() = ERANGE;
        return -(1 as c_int);
    }
    if maxbits == bits {
        return 0 as c_int;
    }
    if maxbits - bits <= 4 as c_int {
        return BitStream_appendNum(
            bstream,
            (maxbits - bits) as size_t,
            0 as c_uint,
        );
    }
    words = (bits + 4 as c_int + 7 as c_int) / 8 as c_int;
    ret = BitStream_appendNum(
        bstream,
        (words * 8 as c_int - bits) as size_t,
        0 as c_uint,
    );
    if ret < 0 as c_int {
        return ret;
    }
    padlen = maxwords - words;
    if padlen > 0 as c_int {
        i = 0 as c_int;
        while i < padlen {
            ret = BitStream_appendNum(
                bstream,
                8 as size_t,
                (if i & 1 as c_int != 0 {
                    0x11 as c_int
                } else {
                    0xec as c_int
                }) as c_uint,
            );
            if ret < 0 as c_int {
                return ret;
            }
            i += 1;
        }
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_appendPaddingBitMQR(
    mut bstream: *mut BitStream,
    mut input: *mut QRinput,
) -> c_int {
    let mut bits: c_int = 0;
    let mut maxbits: c_int = 0;
    let mut words: c_int = 0;
    let mut maxwords: c_int = 0;
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    let mut termbits: c_int = 0;
    let mut padlen: c_int = 0;
    bits = (*bstream).length as c_int;
    maxbits = MQRspec_getDataLengthBit((*input).version, (*input).level);
    maxwords = maxbits / 8 as c_int;
    if maxbits < bits {
        *__errno_location() = ERANGE;
        return -(1 as c_int);
    }
    if maxbits == bits {
        return 0 as c_int;
    }
    termbits = (*input).version * 2 as c_int + 1 as c_int;
    if maxbits - bits <= termbits {
        return BitStream_appendNum(
            bstream,
            (maxbits - bits) as size_t,
            0 as c_uint,
        );
    }
    bits += termbits;
    words = (bits + 7 as c_int) / 8 as c_int;
    if maxbits - words * 8 as c_int > 0 as c_int {
        termbits += words * 8 as c_int - bits;
        if words == maxwords {
            termbits += maxbits - words * 8 as c_int;
        }
    } else {
        termbits += words * 8 as c_int - bits;
    }
    ret = BitStream_appendNum(bstream, termbits as size_t, 0 as c_uint);
    if ret < 0 as c_int {
        return ret;
    }
    padlen = maxwords - words;
    if padlen > 0 as c_int {
        i = 0 as c_int;
        while i < padlen {
            ret = BitStream_appendNum(
                bstream,
                8 as size_t,
                (if i & 1 as c_int != 0 {
                    0x11 as c_int
                } else {
                    0xec as c_int
                }) as c_uint,
            );
            if ret < 0 as c_int {
                return ret;
            }
            i += 1;
        }
        termbits = maxbits - maxwords * 8 as c_int;
        if termbits > 0 as c_int {
            ret = BitStream_appendNum(bstream, termbits as size_t, 0 as c_uint);
            if ret < 0 as c_int {
                return ret;
            }
        }
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_insertFNC1Header(mut input: *mut QRinput) -> c_int {
    let mut entry: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    if (*input).fnc1 == 1 as c_int {
        entry = QRinput_List_newEntry(
            QR_MODE_FNC1FIRST,
            0 as c_int,
            ::core::ptr::null::<c_uchar>(),
        );
    } else if (*input).fnc1 == 2 as c_int {
        entry = QRinput_List_newEntry(
            QR_MODE_FNC1SECOND,
            1 as c_int,
            &raw mut (*input).appid,
        );
    }
    if entry.is_null() {
        return -(1 as c_int);
    }
    if (*(*input).head).mode as c_int != QR_MODE_STRUCTURE as c_int
        && (*(*input).head).mode as c_int != QR_MODE_ECI as c_int
    {
        (*entry).next = (*input).head;
        (*input).head = entry;
    } else {
        (*entry).next = (*(*input).head).next;
        (*(*input).head).next = entry;
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_mergeBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> c_int {
    if (*input).mqr != 0 {
        if QRinput_createBitStream(input, bstream) < 0 as c_int {
            return -(1 as c_int);
        }
    } else {
        if (*input).fnc1 != 0 {
            if QRinput_insertFNC1Header(input) < 0 as c_int {
                return -(1 as c_int);
            }
        }
        if QRinput_convertData(input, bstream) < 0 as c_int {
            return -(1 as c_int);
        }
    }
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_getBitStream(
    mut input: *mut QRinput,
    mut bstream: *mut BitStream,
) -> c_int {
    let mut ret: c_int = 0;
    ret = QRinput_mergeBitStream(input, bstream);
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    if (*input).mqr != 0 {
        ret = QRinput_appendPaddingBitMQR(bstream, input);
    } else {
        ret = QRinput_appendPaddingBit(bstream, input);
    }
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_getByteStream(
    mut input: *mut QRinput,
) -> *mut c_uchar {
    let mut bstream: *mut BitStream = ::core::ptr::null_mut::<BitStream>();
    let mut array: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut ret: c_int = 0;
    bstream = BitStream_new();
    if bstream.is_null() {
        return ::core::ptr::null_mut::<c_uchar>();
    }
    ret = QRinput_getBitStream(input, bstream);
    if ret < 0 as c_int {
        BitStream_free(bstream);
        return ::core::ptr::null_mut::<c_uchar>();
    }
    array = BitStream_toByte(bstream);
    BitStream_free(bstream);
    return array;
}
unsafe extern "C" fn QRinput_InputList_newEntry(mut input: *mut QRinput) -> *mut QRinput_InputList {
    let mut entry: *mut QRinput_InputList = ::core::ptr::null_mut::<QRinput_InputList>();
    entry = malloc(::core::mem::size_of::<QRinput_InputList>() as size_t) as *mut QRinput_InputList;
    if entry.is_null() {
        return ::core::ptr::null_mut::<QRinput_InputList>();
    }
    (*entry).input = input;
    (*entry).next = ::core::ptr::null_mut::<QRinput_InputList>();
    return entry;
}
unsafe extern "C" fn QRinput_InputList_freeEntry(mut entry: *mut QRinput_InputList) {
    if !entry.is_null() {
        QRinput_free((*entry).input);
        free(entry as *mut c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_Struct_new() -> *mut QRinput_Struct {
    let mut s: *mut QRinput_Struct = ::core::ptr::null_mut::<QRinput_Struct>();
    s = malloc(::core::mem::size_of::<QRinput_Struct>() as size_t) as *mut QRinput_Struct;
    if s.is_null() {
        return ::core::ptr::null_mut::<QRinput_Struct>();
    }
    (*s).size = 0 as c_int;
    (*s).parity = -(1 as c_int);
    (*s).head = ::core::ptr::null_mut::<QRinput_InputList>();
    (*s).tail = ::core::ptr::null_mut::<QRinput_InputList>();
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_Struct_setParity(
    mut s: *mut QRinput_Struct,
    mut parity: c_uchar,
) {
    (*s).parity = parity as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_Struct_appendInput(
    mut s: *mut QRinput_Struct,
    mut input: *mut QRinput,
) -> c_int {
    let mut e: *mut QRinput_InputList = ::core::ptr::null_mut::<QRinput_InputList>();
    if (*input).mqr != 0 {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    e = QRinput_InputList_newEntry(input);
    if e.is_null() {
        return -(1 as c_int);
    }
    (*s).size += 1;
    if (*s).tail.is_null() {
        (*s).head = e;
        (*s).tail = e;
    } else {
        (*(*s).tail).next = e;
        (*s).tail = e;
    }
    return (*s).size;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_Struct_free(mut s: *mut QRinput_Struct) {
    let mut list: *mut QRinput_InputList = ::core::ptr::null_mut::<QRinput_InputList>();
    let mut next: *mut QRinput_InputList = ::core::ptr::null_mut::<QRinput_InputList>();
    if !s.is_null() {
        list = (*s).head;
        while !list.is_null() {
            next = (*list).next;
            QRinput_InputList_freeEntry(list);
            list = next;
        }
        free(s as *mut c_void);
    }
}
unsafe extern "C" fn QRinput_Struct_calcParity(mut s: *mut QRinput_Struct) -> c_uchar {
    let mut list: *mut QRinput_InputList = ::core::ptr::null_mut::<QRinput_InputList>();
    let mut parity: c_uchar = 0 as c_uchar;
    list = (*s).head;
    while !list.is_null() {
        parity = (parity as c_int
            ^ QRinput_calcParity((*list).input) as c_int)
            as c_uchar;
        list = (*list).next;
    }
    QRinput_Struct_setParity(s, parity);
    return parity;
}
unsafe extern "C" fn QRinput_List_shrinkEntry(
    mut entry: *mut QRinput_List,
    mut bytes: c_int,
) -> c_int {
    let mut data: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    data = malloc(bytes as size_t) as *mut c_uchar;
    if data.is_null() {
        return -(1 as c_int);
    }
    memcpy(
        data as *mut c_void,
        (*entry).data as *const c_void,
        bytes as size_t,
    );
    free((*entry).data as *mut c_void);
    (*entry).data = data;
    (*entry).size = bytes;
    return 0 as c_int;
}
unsafe extern "C" fn QRinput_splitEntry(
    mut entry: *mut QRinput_List,
    mut bytes: c_int,
) -> c_int {
    let mut e: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut ret: c_int = 0;
    e = QRinput_List_newEntry(
        (*entry).mode,
        (*entry).size - bytes,
        (*entry).data.offset(bytes as isize),
    );
    if e.is_null() {
        return -(1 as c_int);
    }
    ret = QRinput_List_shrinkEntry(entry, bytes);
    if ret < 0 as c_int {
        QRinput_List_freeEntry(e);
        return -(1 as c_int);
    }
    (*e).next = (*entry).next;
    (*entry).next = e;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_splitQRinputToStruct(
    mut input: *mut QRinput,
) -> *mut QRinput_Struct {
    let mut current_block: u64;
    let mut p: *mut QRinput = ::core::ptr::null_mut::<QRinput>();
    let mut s: *mut QRinput_Struct = ::core::ptr::null_mut::<QRinput_Struct>();
    let mut bits: c_int = 0;
    let mut maxbits: c_int = 0;
    let mut nextbits: c_int = 0;
    let mut bytes: c_int = 0;
    let mut ret: c_int = 0;
    let mut list: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut next: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut prev: *mut QRinput_List = ::core::ptr::null_mut::<QRinput_List>();
    let mut bstream: *mut BitStream = ::core::ptr::null_mut::<BitStream>();
    if (*input).mqr != 0 {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRinput_Struct>();
    }
    s = QRinput_Struct_new();
    if s.is_null() {
        return ::core::ptr::null_mut::<QRinput_Struct>();
    }
    input = QRinput_dup(input);
    if input.is_null() {
        QRinput_Struct_free(s);
        return ::core::ptr::null_mut::<QRinput_Struct>();
    }
    QRinput_Struct_setParity(s, QRinput_calcParity(input));
    maxbits = QRspec_getDataLength((*input).version, (*input).level) * 8 as c_int
        - STRUCTURE_HEADER_SIZE;
    if !(maxbits <= 0 as c_int) {
        bstream = BitStream_new();
        if !bstream.is_null() {
            bits = 0 as c_int;
            list = (*input).head;
            prev = ::core::ptr::null_mut::<QRinput_List>();
            loop {
                if list.is_null() {
                    current_block = 572715077006366937;
                    break;
                }
                nextbits =
                    QRinput_estimateBitStreamSizeOfEntry(list, (*input).version, (*input).mqr);
                if bits + nextbits <= maxbits {
                    (*bstream).length = 0 as size_t;
                    ret = QRinput_encodeBitStream(list, bstream, (*input).version, (*input).mqr);
                    if ret < 0 as c_int {
                        current_block = 10583721196169621243;
                        break;
                    }
                    bits += ret;
                    prev = list;
                    list = (*list).next;
                } else {
                    bytes = QRinput_lengthOfCode((*list).mode, (*input).version, maxbits - bits);
                    p = QRinput_new2((*input).version, (*input).level);
                    if p.is_null() {
                        current_block = 10583721196169621243;
                        break;
                    }
                    if bytes > 0 as c_int {
                        ret = QRinput_splitEntry(list, bytes);
                        if ret < 0 as c_int {
                            QRinput_free(p);
                            current_block = 10583721196169621243;
                            break;
                        } else {
                            next = (*list).next;
                            (*list).next = ::core::ptr::null_mut::<QRinput_List>();
                            (*p).head = next;
                            (*p).tail = (*input).tail;
                            (*input).tail = list;
                            prev = list;
                            list = next;
                        }
                    } else {
                        (*prev).next = ::core::ptr::null_mut::<QRinput_List>();
                        (*p).head = list;
                        (*p).tail = (*input).tail;
                        (*input).tail = prev;
                    }
                    ret = QRinput_Struct_appendInput(s, input);
                    if ret < 0 as c_int {
                        QRinput_free(p);
                        current_block = 10583721196169621243;
                        break;
                    } else {
                        input = p;
                        bits = 0 as c_int;
                    }
                }
            }
            match current_block {
                10583721196169621243 => {}
                _ => {
                    ret = QRinput_Struct_appendInput(s, input);
                    if !(ret < 0 as c_int) {
                        if (*s).size > MAX_STRUCTURED_SYMBOLS {
                            *__errno_location() = ERANGE;
                            QRinput_Struct_free(s);
                            BitStream_free(bstream);
                            return ::core::ptr::null_mut::<QRinput_Struct>();
                        }
                        ret = QRinput_Struct_insertStructuredAppendHeaders(s);
                        if ret < 0 as c_int {
                            QRinput_Struct_free(s);
                            BitStream_free(bstream);
                            return ::core::ptr::null_mut::<QRinput_Struct>();
                        }
                        BitStream_free(bstream);
                        return s;
                    }
                }
            }
        }
    }
    BitStream_free(bstream);
    QRinput_free(input);
    QRinput_Struct_free(s);
    return ::core::ptr::null_mut::<QRinput_Struct>();
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_Struct_insertStructuredAppendHeaders(
    mut s: *mut QRinput_Struct,
) -> c_int {
    let mut i: c_int = 0;
    let mut list: *mut QRinput_InputList = ::core::ptr::null_mut::<QRinput_InputList>();
    if (*s).size == 1 as c_int {
        return 0 as c_int;
    }
    if (*s).parity < 0 as c_int {
        QRinput_Struct_calcParity(s);
    }
    i = 1 as c_int;
    list = (*s).head;
    while !list.is_null() {
        if QRinput_insertStructuredAppendHeader(
            (*list).input,
            (*s).size,
            i,
            (*s).parity as c_uchar,
        ) != 0
        {
            return -(1 as c_int);
        }
        i += 1;
        list = (*list).next;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_setFNC1First(mut input: *mut QRinput) -> c_int {
    if (*input).mqr != 0 {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    (*input).fnc1 = 1 as c_int;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn QRinput_setFNC1Second(
    mut input: *mut QRinput,
    mut appid: c_uchar,
) -> c_int {
    if (*input).mqr != 0 {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    (*input).fnc1 = 2 as c_int;
    (*input).appid = appid;
    return 0 as c_int;
}
pub const QRSPEC_MODEID_ECI: c_int = 7 as c_int;
pub const QRSPEC_MODEID_NUM: c_int = 1 as c_int;
pub const QRSPEC_MODEID_AN: c_int = 2 as c_int;
pub const QRSPEC_MODEID_8: c_int = 4 as c_int;
pub const QRSPEC_MODEID_KANJI: c_int = 8 as c_int;
pub const QRSPEC_MODEID_FNC1SECOND: c_int = 9 as c_int;
pub const QRSPEC_MODEID_STRUCTURE: c_int = 3 as c_int;
