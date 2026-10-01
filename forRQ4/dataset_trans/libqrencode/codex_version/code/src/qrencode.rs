extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn QRinput_new2(version: ::core::ffi::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_newMQR(version: ::core::ffi::c_int, level: QRecLevel) -> *mut QRinput;
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: ::core::ffi::c_int,
        data: *const ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn QRinput_free(input: *mut QRinput);
    fn QRinput_Struct_free(s: *mut QRinput_Struct);
    fn QRinput_splitQRinputToStruct(input: *mut QRinput) -> *mut QRinput_Struct;
    fn QRinput_getByteStream(input: *mut QRinput) -> *mut ::core::ffi::c_uchar;
    fn MQRspec_getDataLengthBit(
        version: ::core::ffi::c_int,
        level: QRecLevel,
    ) -> ::core::ffi::c_int;
    fn MQRspec_getDataLength(version: ::core::ffi::c_int, level: QRecLevel) -> ::core::ffi::c_int;
    fn MQRspec_getECCLength(version: ::core::ffi::c_int, level: QRecLevel) -> ::core::ffi::c_int;
    fn MQRspec_getWidth(version: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn MQRspec_newFrame(version: ::core::ffi::c_int) -> *mut ::core::ffi::c_uchar;
    fn RSECC_encode(
        data_length: size_t,
        ecc_length: size_t,
        data: *const ::core::ffi::c_uchar,
        ecc: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn Split_splitStringToQRinput(
        string: *const ::core::ffi::c_char,
        input: *mut QRinput,
        hint: QRencodeMode,
        casesensitive: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Mask_makeMask(
        width: ::core::ffi::c_int,
        frame: *mut ::core::ffi::c_uchar,
        mask: ::core::ffi::c_int,
        level: QRecLevel,
    ) -> *mut ::core::ffi::c_uchar;
    fn Mask_mask(
        width: ::core::ffi::c_int,
        frame: *mut ::core::ffi::c_uchar,
        level: QRecLevel,
    ) -> *mut ::core::ffi::c_uchar;
    fn MMask_makeMask(
        version: ::core::ffi::c_int,
        frame: *mut ::core::ffi::c_uchar,
        mask: ::core::ffi::c_int,
        level: QRecLevel,
    ) -> *mut ::core::ffi::c_uchar;
    fn MMask_mask(
        version: ::core::ffi::c_int,
        frame: *mut ::core::ffi::c_uchar,
        level: QRecLevel,
    ) -> *mut ::core::ffi::c_uchar;
    fn QRspec_getWidth(version: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn QRspec_getRemainder(version: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn QRspec_getEccSpec(
        version: ::core::ffi::c_int,
        level: QRecLevel,
        spec: *mut ::core::ffi::c_int,
    );
    fn QRspec_newFrame(version: ::core::ffi::c_int) -> *mut ::core::ffi::c_uchar;
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
pub struct _QRinput {
    pub version: ::core::ffi::c_int,
    pub level: QRecLevel,
    pub head: *mut QRinput_List,
    pub tail: *mut QRinput_List,
    pub mqr: ::core::ffi::c_int,
    pub fnc1: ::core::ffi::c_int,
    pub appid: ::core::ffi::c_uchar,
}
pub type QRinput_List = _QRinput_List;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_List {
    pub mode: QRencodeMode,
    pub size: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_uchar,
    pub bstream: *mut BitStream,
    pub next: *mut QRinput_List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub length: size_t,
    pub datasize: size_t,
    pub data: *mut ::core::ffi::c_uchar,
}
pub type QRinput = _QRinput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_Struct {
    pub size: ::core::ffi::c_int,
    pub parity: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRcode {
    pub version: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRcode_List {
    pub code: *mut QRcode,
    pub next: *mut _QRcode_List,
}
pub type QRcode_List = _QRcode_List;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QRRawCode {
    pub version: ::core::ffi::c_int,
    pub dataLength: ::core::ffi::c_int,
    pub eccLength: ::core::ffi::c_int,
    pub datacode: *mut ::core::ffi::c_uchar,
    pub ecccode: *mut ::core::ffi::c_uchar,
    pub b1: ::core::ffi::c_int,
    pub blocks: ::core::ffi::c_int,
    pub rsblock: *mut RSblock,
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RSblock {
    pub dataLength: ::core::ffi::c_int,
    pub eccLength: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_uchar,
    pub ecc: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameFiller {
    pub width: ::core::ffi::c_int,
    pub frame: *mut ::core::ffi::c_uchar,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub dir: ::core::ffi::c_int,
    pub bit: ::core::ffi::c_int,
    pub mqr: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MQRRawCode {
    pub version: ::core::ffi::c_int,
    pub dataLength: ::core::ffi::c_int,
    pub eccLength: ::core::ffi::c_int,
    pub datacode: *mut ::core::ffi::c_uchar,
    pub ecccode: *mut ::core::ffi::c_uchar,
    pub rsblock: *mut RSblock,
    pub oddbits: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const QRSPEC_VERSION_MAX: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const MQRSPEC_VERSION_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn RSblock_initBlock(
    mut block: *mut RSblock,
    mut dl: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_uchar,
    mut el: ::core::ffi::c_int,
    mut ecc: *mut ::core::ffi::c_uchar,
) {
    (*block).dataLength = dl;
    (*block).data = data;
    (*block).eccLength = el;
    (*block).ecc = ecc;
    RSECC_encode(dl as size_t, el as size_t, data, ecc);
}
unsafe extern "C" fn RSblock_init(
    mut blocks: *mut RSblock,
    mut spec: *mut ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_uchar,
    mut ecc: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut block: *mut RSblock = ::core::ptr::null_mut::<RSblock>();
    let mut dp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut ep: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut el: ::core::ffi::c_int = 0;
    let mut dl: ::core::ffi::c_int = 0;
    dl = *spec.offset(1 as ::core::ffi::c_int as isize);
    el = *spec.offset(2 as ::core::ffi::c_int as isize);
    block = blocks;
    dp = data;
    ep = ecc;
    i = 0 as ::core::ffi::c_int;
    while i < *spec.offset(0 as ::core::ffi::c_int as isize) {
        RSblock_initBlock(block, dl, dp, el, ep);
        dp = dp.offset(dl as isize);
        ep = ep.offset(el as isize);
        block = block.offset(1);
        i += 1;
    }
    if *spec.offset(3 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    dl = *spec.offset(4 as ::core::ffi::c_int as isize);
    el = *spec.offset(2 as ::core::ffi::c_int as isize);
    i = 0 as ::core::ffi::c_int;
    while i < *spec.offset(3 as ::core::ffi::c_int as isize) {
        RSblock_initBlock(block, dl, dp, el, ep);
        dp = dp.offset(dl as isize);
        ep = ep.offset(el as isize);
        block = block.offset(1);
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn QRraw_new(mut input: *mut QRinput) -> *mut QRRawCode {
    let mut raw: *mut QRRawCode = ::core::ptr::null_mut::<QRRawCode>();
    let mut spec: [::core::ffi::c_int; 5] = [0; 5];
    let mut ret: ::core::ffi::c_int = 0;
    raw = malloc(::core::mem::size_of::<QRRawCode>() as size_t) as *mut QRRawCode;
    if raw.is_null() {
        return ::core::ptr::null_mut::<QRRawCode>();
    }
    (*raw).datacode = QRinput_getByteStream(input);
    if (*raw).datacode.is_null() {
        free(raw as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<QRRawCode>();
    }
    QRspec_getEccSpec(
        (*input).version,
        (*input).level,
        &raw mut spec as *mut ::core::ffi::c_int,
    );
    (*raw).version = (*input).version;
    (*raw).b1 = spec[0 as ::core::ffi::c_int as usize];
    (*raw).dataLength = spec[0 as ::core::ffi::c_int as usize]
        * spec[1 as ::core::ffi::c_int as usize]
        + spec[3 as ::core::ffi::c_int as usize] * spec[4 as ::core::ffi::c_int as usize];
    (*raw).eccLength = (spec[0 as ::core::ffi::c_int as usize]
        + spec[3 as ::core::ffi::c_int as usize])
        * spec[2 as ::core::ffi::c_int as usize];
    (*raw).ecccode = malloc((*raw).eccLength as size_t) as *mut ::core::ffi::c_uchar;
    if (*raw).ecccode.is_null() {
        free((*raw).datacode as *mut ::core::ffi::c_void);
        free(raw as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<QRRawCode>();
    }
    (*raw).blocks = spec[0 as ::core::ffi::c_int as usize] + spec[3 as ::core::ffi::c_int as usize];
    (*raw).rsblock = calloc(
        (*raw).blocks as size_t,
        ::core::mem::size_of::<RSblock>() as size_t,
    ) as *mut RSblock;
    if (*raw).rsblock.is_null() {
        QRraw_free(raw);
        return ::core::ptr::null_mut::<QRRawCode>();
    }
    ret = RSblock_init(
        (*raw).rsblock,
        &raw mut spec as *mut ::core::ffi::c_int,
        (*raw).datacode,
        (*raw).ecccode,
    );
    if ret < 0 as ::core::ffi::c_int {
        QRraw_free(raw);
        return ::core::ptr::null_mut::<QRRawCode>();
    }
    (*raw).count = 0 as ::core::ffi::c_int;
    return raw;
}
unsafe extern "C" fn QRraw_getCode(mut raw: *mut QRRawCode) -> ::core::ffi::c_uchar {
    let mut col: ::core::ffi::c_int = 0;
    let mut row: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_uchar = 0;
    if (*raw).count < (*raw).dataLength {
        row = (*raw).count % (*raw).blocks;
        col = (*raw).count / (*raw).blocks;
        if col >= (*(*raw).rsblock.offset(0 as ::core::ffi::c_int as isize)).dataLength {
            row += (*raw).b1;
        }
        ret = *(*(*raw).rsblock.offset(row as isize))
            .data
            .offset(col as isize);
    } else if (*raw).count < (*raw).dataLength + (*raw).eccLength {
        row = ((*raw).count - (*raw).dataLength) % (*raw).blocks;
        col = ((*raw).count - (*raw).dataLength) / (*raw).blocks;
        ret = *(*(*raw).rsblock.offset(row as isize))
            .ecc
            .offset(col as isize);
    } else {
        return 0 as ::core::ffi::c_uchar;
    }
    (*raw).count += 1;
    return ret;
}
unsafe extern "C" fn QRraw_free(mut raw: *mut QRRawCode) {
    if !raw.is_null() {
        free((*raw).datacode as *mut ::core::ffi::c_void);
        free((*raw).ecccode as *mut ::core::ffi::c_void);
        free((*raw).rsblock as *mut ::core::ffi::c_void);
        free(raw as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn MQRraw_new(mut input: *mut QRinput) -> *mut MQRRawCode {
    let mut raw: *mut MQRRawCode = ::core::ptr::null_mut::<MQRRawCode>();
    raw = malloc(::core::mem::size_of::<MQRRawCode>() as size_t) as *mut MQRRawCode;
    if raw.is_null() {
        return ::core::ptr::null_mut::<MQRRawCode>();
    }
    (*raw).version = (*input).version;
    (*raw).dataLength = MQRspec_getDataLength((*input).version, (*input).level);
    (*raw).eccLength = MQRspec_getECCLength((*input).version, (*input).level);
    (*raw).oddbits = (*raw).dataLength * 8 as ::core::ffi::c_int
        - MQRspec_getDataLengthBit((*input).version, (*input).level);
    (*raw).datacode = QRinput_getByteStream(input);
    if (*raw).datacode.is_null() {
        free(raw as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<MQRRawCode>();
    }
    (*raw).ecccode = malloc((*raw).eccLength as size_t) as *mut ::core::ffi::c_uchar;
    if (*raw).ecccode.is_null() {
        free((*raw).datacode as *mut ::core::ffi::c_void);
        free(raw as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<MQRRawCode>();
    }
    (*raw).rsblock =
        calloc(1 as size_t, ::core::mem::size_of::<RSblock>() as size_t) as *mut RSblock;
    if (*raw).rsblock.is_null() {
        MQRraw_free(raw);
        return ::core::ptr::null_mut::<MQRRawCode>();
    }
    RSblock_initBlock(
        (*raw).rsblock,
        (*raw).dataLength,
        (*raw).datacode,
        (*raw).eccLength,
        (*raw).ecccode,
    );
    (*raw).count = 0 as ::core::ffi::c_int;
    return raw;
}
unsafe extern "C" fn MQRraw_getCode(mut raw: *mut MQRRawCode) -> ::core::ffi::c_uchar {
    let mut ret: ::core::ffi::c_uchar = 0;
    if (*raw).count < (*raw).dataLength {
        ret = *(*raw).datacode.offset((*raw).count as isize);
    } else if (*raw).count < (*raw).dataLength + (*raw).eccLength {
        ret = *(*raw)
            .ecccode
            .offset(((*raw).count - (*raw).dataLength) as isize);
    } else {
        return 0 as ::core::ffi::c_uchar;
    }
    (*raw).count += 1;
    return ret;
}
unsafe extern "C" fn MQRraw_free(mut raw: *mut MQRRawCode) {
    if !raw.is_null() {
        free((*raw).datacode as *mut ::core::ffi::c_void);
        free((*raw).ecccode as *mut ::core::ffi::c_void);
        free((*raw).rsblock as *mut ::core::ffi::c_void);
        free(raw as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn FrameFiller_set(
    mut filler: *mut FrameFiller,
    mut width: ::core::ffi::c_int,
    mut frame: *mut ::core::ffi::c_uchar,
    mut mqr: ::core::ffi::c_int,
) {
    (*filler).width = width;
    (*filler).frame = frame;
    (*filler).x = width - 1 as ::core::ffi::c_int;
    (*filler).y = width - 1 as ::core::ffi::c_int;
    (*filler).dir = -(1 as ::core::ffi::c_int);
    (*filler).bit = -(1 as ::core::ffi::c_int);
    (*filler).mqr = mqr;
}
unsafe extern "C" fn FrameFiller_next(mut filler: *mut FrameFiller) -> *mut ::core::ffi::c_uchar {
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    if (*filler).bit == -(1 as ::core::ffi::c_int) {
        (*filler).bit = 0 as ::core::ffi::c_int;
        return (*filler)
            .frame
            .offset(((*filler).y * (*filler).width) as isize)
            .offset((*filler).x as isize);
    }
    x = (*filler).x;
    y = (*filler).y;
    p = (*filler).frame;
    w = (*filler).width;
    if (*filler).bit == 0 as ::core::ffi::c_int {
        x -= 1;
        (*filler).bit += 1;
    } else {
        x += 1;
        y += (*filler).dir;
        (*filler).bit -= 1;
    }
    if (*filler).dir < 0 as ::core::ffi::c_int {
        if y < 0 as ::core::ffi::c_int {
            y = 0 as ::core::ffi::c_int;
            x -= 2 as ::core::ffi::c_int;
            (*filler).dir = 1 as ::core::ffi::c_int;
            if (*filler).mqr == 0 && x == 6 as ::core::ffi::c_int {
                x -= 1;
                y = 9 as ::core::ffi::c_int;
            }
        }
    } else if y == w {
        y = w - 1 as ::core::ffi::c_int;
        x -= 2 as ::core::ffi::c_int;
        (*filler).dir = -(1 as ::core::ffi::c_int);
        if (*filler).mqr == 0 && x == 6 as ::core::ffi::c_int {
            x -= 1;
            y -= 8 as ::core::ffi::c_int;
        }
    }
    if x < 0 as ::core::ffi::c_int || y < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    (*filler).x = x;
    (*filler).y = y;
    if *p.offset((y * w + x) as isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        return FrameFiller_next(filler);
    }
    return p.offset((y * w + x) as isize) as *mut ::core::ffi::c_uchar;
}
unsafe extern "C" fn QRcode_new(
    mut version: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_uchar,
) -> *mut QRcode {
    let mut qrcode: *mut QRcode = ::core::ptr::null_mut::<QRcode>();
    qrcode = malloc(::core::mem::size_of::<QRcode>() as size_t) as *mut QRcode;
    if qrcode.is_null() {
        return ::core::ptr::null_mut::<QRcode>();
    }
    (*qrcode).version = version;
    (*qrcode).width = width;
    (*qrcode).data = data;
    return qrcode;
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_free(mut qrcode: *mut QRcode) {
    if !qrcode.is_null() {
        free((*qrcode).data as *mut ::core::ffi::c_void);
        free(qrcode as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn QRcode_encodeMask(
    mut input: *mut QRinput,
    mut mask: ::core::ffi::c_int,
) -> *mut QRcode {
    let mut current_block: u64;
    let mut width: ::core::ffi::c_int = 0;
    let mut version: ::core::ffi::c_int = 0;
    let mut raw: *mut QRRawCode = ::core::ptr::null_mut::<QRRawCode>();
    let mut frame: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut masked: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut code: ::core::ffi::c_uchar = 0;
    let mut bit: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut qrcode: *mut QRcode = ::core::ptr::null_mut::<QRcode>();
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    if (*input).mqr != 0 {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    if (*input).version < 0 as ::core::ffi::c_int || (*input).version > QRSPEC_VERSION_MAX {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    if !((*input).level as ::core::ffi::c_uint
        >= QR_ECLEVEL_L as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*input).level as ::core::ffi::c_uint
            <= QR_ECLEVEL_H as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    raw = QRraw_new(input);
    if raw.is_null() {
        return ::core::ptr::null_mut::<QRcode>();
    }
    version = (*raw).version;
    width = QRspec_getWidth(version);
    frame = QRspec_newFrame(version);
    if frame.is_null() {
        QRraw_free(raw);
        return ::core::ptr::null_mut::<QRcode>();
    }
    FrameFiller_set(&raw mut filler, width, frame, 0 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    's_88: loop {
        if !(i < (*raw).dataLength) {
            current_block = 4068382217303356765;
            break;
        }
        code = QRraw_getCode(raw);
        bit = 0x80 as ::core::ffi::c_uchar;
        j = 0 as ::core::ffi::c_int;
        while j < 8 as ::core::ffi::c_int {
            p = FrameFiller_next(&raw mut filler);
            if p.is_null() {
                current_block = 13866549931906050429;
                break 's_88;
            }
            *p = (bit as ::core::ffi::c_int & code as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uchar;
            bit = (bit as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            j += 1;
        }
        i += 1;
    }
    match current_block {
        4068382217303356765 => {
            i = 0 as ::core::ffi::c_int;
            's_134: loop {
                if !(i < (*raw).eccLength) {
                    current_block = 7245201122033322888;
                    break;
                }
                code = QRraw_getCode(raw);
                bit = 0x80 as ::core::ffi::c_uchar;
                j = 0 as ::core::ffi::c_int;
                while j < 8 as ::core::ffi::c_int {
                    p = FrameFiller_next(&raw mut filler);
                    if p.is_null() {
                        current_block = 13866549931906050429;
                        break 's_134;
                    }
                    *p = (0x2 as ::core::ffi::c_int
                        | (bit as ::core::ffi::c_int & code as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    bit = (bit as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    j += 1;
                }
                i += 1;
            }
            match current_block {
                13866549931906050429 => {}
                _ => {
                    QRraw_free(raw);
                    raw = ::core::ptr::null_mut::<QRRawCode>();
                    j = QRspec_getRemainder(version);
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i < j) {
                            current_block = 4567019141635105728;
                            break;
                        }
                        p = FrameFiller_next(&raw mut filler);
                        if p.is_null() {
                            current_block = 13866549931906050429;
                            break;
                        }
                        *p = 0x2 as ::core::ffi::c_uchar;
                        i += 1;
                    }
                    match current_block {
                        13866549931906050429 => {}
                        _ => {
                            if mask == -(2 as ::core::ffi::c_int) {
                                masked =
                                    malloc((width * width) as size_t) as *mut ::core::ffi::c_uchar;
                                memcpy(
                                    masked as *mut ::core::ffi::c_void,
                                    frame as *const ::core::ffi::c_void,
                                    (width * width) as size_t,
                                );
                            } else if mask < 0 as ::core::ffi::c_int {
                                masked = Mask_mask(width, frame, (*input).level);
                            } else {
                                masked = Mask_makeMask(width, frame, mask, (*input).level);
                            }
                            if !masked.is_null() {
                                qrcode = QRcode_new(version, width, masked);
                                if qrcode.is_null() {
                                    free(masked as *mut ::core::ffi::c_void);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    QRraw_free(raw);
    free(frame as *mut ::core::ffi::c_void);
    return qrcode;
}
unsafe extern "C" fn QRcode_encodeMaskMQR(
    mut input: *mut QRinput,
    mut mask: ::core::ffi::c_int,
) -> *mut QRcode {
    let mut current_block: u64;
    let mut width: ::core::ffi::c_int = 0;
    let mut version: ::core::ffi::c_int = 0;
    let mut raw: *mut MQRRawCode = ::core::ptr::null_mut::<MQRRawCode>();
    let mut frame: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut masked: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut code: ::core::ffi::c_uchar = 0;
    let mut bit: ::core::ffi::c_uchar = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut qrcode: *mut QRcode = ::core::ptr::null_mut::<QRcode>();
    let mut filler: FrameFiller = FrameFiller {
        width: 0,
        frame: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        x: 0,
        y: 0,
        dir: 0,
        bit: 0,
        mqr: 0,
    };
    if (*input).mqr == 0 {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    if (*input).version <= 0 as ::core::ffi::c_int || (*input).version > MQRSPEC_VERSION_MAX {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    if !((*input).level as ::core::ffi::c_uint
        >= QR_ECLEVEL_L as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*input).level as ::core::ffi::c_uint
            <= QR_ECLEVEL_Q as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    raw = MQRraw_new(input);
    if raw.is_null() {
        return ::core::ptr::null_mut::<QRcode>();
    }
    version = (*raw).version;
    width = MQRspec_getWidth(version);
    frame = MQRspec_newFrame(version);
    if frame.is_null() {
        MQRraw_free(raw);
        return ::core::ptr::null_mut::<QRcode>();
    }
    FrameFiller_set(&raw mut filler, width, frame, 1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    's_88: loop {
        if !(i < (*raw).dataLength) {
            current_block = 17788412896529399552;
            break;
        }
        code = MQRraw_getCode(raw);
        bit = 0x80 as ::core::ffi::c_uchar;
        if (*raw).oddbits != 0 && i == (*raw).dataLength - 1 as ::core::ffi::c_int {
            length = (*raw).oddbits;
        } else {
            length = 8 as ::core::ffi::c_int;
        }
        j = 0 as ::core::ffi::c_int;
        while j < length {
            p = FrameFiller_next(&raw mut filler);
            if p.is_null() {
                current_block = 10267158886153388035;
                break 's_88;
            }
            *p = (bit as ::core::ffi::c_int & code as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_uchar;
            bit = (bit as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            j += 1;
        }
        i += 1;
    }
    match current_block {
        17788412896529399552 => {
            i = 0 as ::core::ffi::c_int;
            's_154: loop {
                if !(i < (*raw).eccLength) {
                    current_block = 15090052786889560393;
                    break;
                }
                code = MQRraw_getCode(raw);
                bit = 0x80 as ::core::ffi::c_uchar;
                length = 8 as ::core::ffi::c_int;
                j = 0 as ::core::ffi::c_int;
                while j < length {
                    p = FrameFiller_next(&raw mut filler);
                    if p.is_null() {
                        current_block = 10267158886153388035;
                        break 's_154;
                    }
                    *p = (0x2 as ::core::ffi::c_int
                        | (bit as ::core::ffi::c_int & code as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    bit = (bit as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    j += 1;
                }
                i += 1;
            }
            match current_block {
                10267158886153388035 => {}
                _ => {
                    MQRraw_free(raw);
                    raw = ::core::ptr::null_mut::<MQRRawCode>();
                    if mask == -(2 as ::core::ffi::c_int) {
                        masked = malloc((width * width) as size_t) as *mut ::core::ffi::c_uchar;
                        memcpy(
                            masked as *mut ::core::ffi::c_void,
                            frame as *const ::core::ffi::c_void,
                            (width * width) as size_t,
                        );
                    } else if mask < 0 as ::core::ffi::c_int {
                        masked = MMask_mask(version, frame, (*input).level);
                    } else {
                        masked = MMask_makeMask(version, frame, mask, (*input).level);
                    }
                    if !masked.is_null() {
                        qrcode = QRcode_new(version, width, masked);
                        if qrcode.is_null() {
                            free(masked as *mut ::core::ffi::c_void);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    MQRraw_free(raw);
    free(frame as *mut ::core::ffi::c_void);
    return qrcode;
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeInput(mut input: *mut QRinput) -> *mut QRcode {
    if (*input).mqr != 0 {
        return QRcode_encodeMaskMQR(input, -(1 as ::core::ffi::c_int));
    } else {
        return QRcode_encodeMask(input, -(1 as ::core::ffi::c_int));
    };
}
unsafe extern "C" fn QRcode_encodeStringReal(
    mut string: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
    mut mqr: ::core::ffi::c_int,
    mut hint: QRencodeMode,
    mut casesensitive: ::core::ffi::c_int,
) -> *mut QRcode {
    let mut input: *mut QRinput = ::core::ptr::null_mut::<QRinput>();
    let mut code: *mut QRcode = ::core::ptr::null_mut::<QRcode>();
    let mut ret: ::core::ffi::c_int = 0;
    if string.is_null() {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    if hint as ::core::ffi::c_int != QR_MODE_8 as ::core::ffi::c_int
        && hint as ::core::ffi::c_int != QR_MODE_KANJI as ::core::ffi::c_int
    {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    if mqr != 0 {
        input = QRinput_newMQR(version, level);
    } else {
        input = QRinput_new2(version, level);
    }
    if input.is_null() {
        return ::core::ptr::null_mut::<QRcode>();
    }
    ret = Split_splitStringToQRinput(string, input, hint, casesensitive);
    if ret < 0 as ::core::ffi::c_int {
        QRinput_free(input);
        return ::core::ptr::null_mut::<QRcode>();
    }
    code = QRcode_encodeInput(input);
    QRinput_free(input);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeString(
    mut string: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: ::core::ffi::c_int,
) -> *mut QRcode {
    return QRcode_encodeStringReal(
        string,
        version,
        level,
        0 as ::core::ffi::c_int,
        hint,
        casesensitive,
    );
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeStringMQR(
    mut string: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: ::core::ffi::c_int,
) -> *mut QRcode {
    let mut i: ::core::ffi::c_int = 0;
    if version == 0 as ::core::ffi::c_int {
        version = 1 as ::core::ffi::c_int;
    }
    i = version;
    while i <= MQRSPEC_VERSION_MAX {
        let mut code: *mut QRcode = QRcode_encodeStringReal(
            string,
            i,
            level,
            1 as ::core::ffi::c_int,
            hint,
            casesensitive,
        );
        if !code.is_null() {
            return code;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<QRcode>();
}
unsafe extern "C" fn QRcode_encodeDataReal(
    mut data: *const ::core::ffi::c_uchar,
    mut length: ::core::ffi::c_int,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
    mut mqr: ::core::ffi::c_int,
) -> *mut QRcode {
    let mut input: *mut QRinput = ::core::ptr::null_mut::<QRinput>();
    let mut code: *mut QRcode = ::core::ptr::null_mut::<QRcode>();
    let mut ret: ::core::ffi::c_int = 0;
    if data.is_null() || length == 0 as ::core::ffi::c_int {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    if mqr != 0 {
        input = QRinput_newMQR(version, level);
    } else {
        input = QRinput_new2(version, level);
    }
    if input.is_null() {
        return ::core::ptr::null_mut::<QRcode>();
    }
    ret = QRinput_append(input, QR_MODE_8, length, data);
    if ret < 0 as ::core::ffi::c_int {
        QRinput_free(input);
        return ::core::ptr::null_mut::<QRcode>();
    }
    code = QRcode_encodeInput(input);
    QRinput_free(input);
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeData(
    mut size: ::core::ffi::c_int,
    mut data: *const ::core::ffi::c_uchar,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    return QRcode_encodeDataReal(data, size, version, level, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeString8bit(
    mut string: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    if string.is_null() {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    return QRcode_encodeDataReal(
        string as *mut ::core::ffi::c_uchar,
        strlen(string) as ::core::ffi::c_int,
        version,
        level,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeDataMQR(
    mut size: ::core::ffi::c_int,
    mut data: *const ::core::ffi::c_uchar,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    let mut i: ::core::ffi::c_int = 0;
    if version == 0 as ::core::ffi::c_int {
        version = 1 as ::core::ffi::c_int;
    }
    i = version;
    while i <= MQRSPEC_VERSION_MAX {
        let mut code: *mut QRcode =
            QRcode_encodeDataReal(data, size, i, level, 1 as ::core::ffi::c_int);
        if !code.is_null() {
            return code;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<QRcode>();
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeString8bitMQR(
    mut string: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> *mut QRcode {
    let mut i: ::core::ffi::c_int = 0;
    if string.is_null() {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode>();
    }
    if version == 0 as ::core::ffi::c_int {
        version = 1 as ::core::ffi::c_int;
    }
    i = version;
    while i <= MQRSPEC_VERSION_MAX {
        let mut code: *mut QRcode = QRcode_encodeDataReal(
            string as *mut ::core::ffi::c_uchar,
            strlen(string) as ::core::ffi::c_int,
            i,
            level,
            1 as ::core::ffi::c_int,
        );
        if !code.is_null() {
            return code;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<QRcode>();
}
unsafe extern "C" fn QRcode_List_newEntry() -> *mut QRcode_List {
    let mut entry: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    entry = malloc(::core::mem::size_of::<QRcode_List>() as size_t) as *mut QRcode_List;
    if entry.is_null() {
        return ::core::ptr::null_mut::<QRcode_List>();
    }
    (*entry).next = ::core::ptr::null_mut::<_QRcode_List>();
    (*entry).code = ::core::ptr::null_mut::<QRcode>();
    return entry;
}
unsafe extern "C" fn QRcode_List_freeEntry(mut entry: *mut QRcode_List) {
    if !entry.is_null() {
        QRcode_free((*entry).code);
        free(entry as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_List_free(mut qrlist: *mut QRcode_List) {
    let mut list: *mut QRcode_List = qrlist;
    let mut next: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    while !list.is_null() {
        next = (*list).next as *mut QRcode_List;
        QRcode_List_freeEntry(list);
        list = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_List_size(mut qrlist: *mut QRcode_List) -> ::core::ffi::c_int {
    let mut list: *mut QRcode_List = qrlist;
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !list.is_null() {
        size += 1;
        list = (*list).next as *mut QRcode_List;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeInputStructured(
    mut s: *mut QRinput_Struct,
) -> *mut QRcode_List {
    let mut current_block: u64;
    let mut head: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    let mut tail: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    let mut entry: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    let mut list: *mut QRinput_InputList = (*s).head;
    loop {
        if list.is_null() {
            current_block = 12800627514080957624;
            break;
        }
        if head.is_null() {
            entry = QRcode_List_newEntry();
            if entry.is_null() {
                current_block = 17062122413114542564;
                break;
            }
            head = entry;
            tail = head;
        } else {
            entry = QRcode_List_newEntry();
            if entry.is_null() {
                current_block = 17062122413114542564;
                break;
            }
            (*tail).next = entry as *mut _QRcode_List;
            tail = (*tail).next as *mut QRcode_List;
        }
        (*tail).code = QRcode_encodeInput((*list).input);
        if (*tail).code.is_null() {
            current_block = 17062122413114542564;
            break;
        }
        list = (*list).next;
    }
    match current_block {
        12800627514080957624 => return head,
        _ => {
            QRcode_List_free(head);
            return ::core::ptr::null_mut::<QRcode_List>();
        }
    };
}
unsafe extern "C" fn QRcode_encodeInputToStructured(mut input: *mut QRinput) -> *mut QRcode_List {
    let mut s: *mut QRinput_Struct = ::core::ptr::null_mut::<QRinput_Struct>();
    let mut codes: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    s = QRinput_splitQRinputToStruct(input);
    if s.is_null() {
        return ::core::ptr::null_mut::<QRcode_List>();
    }
    codes = QRcode_encodeInputStructured(s);
    QRinput_Struct_free(s);
    return codes;
}
unsafe extern "C" fn QRcode_encodeDataStructuredReal(
    mut size: ::core::ffi::c_int,
    mut data: *const ::core::ffi::c_uchar,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
    mut eightbit: ::core::ffi::c_int,
    mut hint: QRencodeMode,
    mut casesensitive: ::core::ffi::c_int,
) -> *mut QRcode_List {
    let mut input: *mut QRinput = ::core::ptr::null_mut::<QRinput>();
    let mut codes: *mut QRcode_List = ::core::ptr::null_mut::<QRcode_List>();
    let mut ret: ::core::ffi::c_int = 0;
    if version <= 0 as ::core::ffi::c_int {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode_List>();
    }
    if eightbit == 0
        && (hint as ::core::ffi::c_int != QR_MODE_8 as ::core::ffi::c_int
            && hint as ::core::ffi::c_int != QR_MODE_KANJI as ::core::ffi::c_int)
    {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode_List>();
    }
    input = QRinput_new2(version, level);
    if input.is_null() {
        return ::core::ptr::null_mut::<QRcode_List>();
    }
    if eightbit != 0 {
        ret = QRinput_append(input, QR_MODE_8, size, data);
    } else {
        ret = Split_splitStringToQRinput(
            data as *mut ::core::ffi::c_char,
            input,
            hint,
            casesensitive,
        );
    }
    if ret < 0 as ::core::ffi::c_int {
        QRinput_free(input);
        return ::core::ptr::null_mut::<QRcode_List>();
    }
    codes = QRcode_encodeInputToStructured(input);
    QRinput_free(input);
    return codes;
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeDataStructured(
    mut size: ::core::ffi::c_int,
    mut data: *const ::core::ffi::c_uchar,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> *mut QRcode_List {
    return QRcode_encodeDataStructuredReal(
        size,
        data,
        version,
        level,
        1 as ::core::ffi::c_int,
        QR_MODE_NUL,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeString8bitStructured(
    mut string: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
) -> *mut QRcode_List {
    if string.is_null() {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode_List>();
    }
    return QRcode_encodeDataStructured(
        strlen(string) as ::core::ffi::c_int,
        string as *mut ::core::ffi::c_uchar,
        version,
        level,
    );
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_encodeStringStructured(
    mut string: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
    mut level: QRecLevel,
    mut hint: QRencodeMode,
    mut casesensitive: ::core::ffi::c_int,
) -> *mut QRcode_List {
    if string.is_null() {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<QRcode_List>();
    }
    return QRcode_encodeDataStructuredReal(
        strlen(string) as ::core::ffi::c_int,
        string as *mut ::core::ffi::c_uchar,
        version,
        level,
        0 as ::core::ffi::c_int,
        hint,
        casesensitive,
    );
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_APIVersion(
    mut major_version: *mut ::core::ffi::c_int,
    mut minor_version: *mut ::core::ffi::c_int,
    mut micro_version: *mut ::core::ffi::c_int,
) {
    if !major_version.is_null() {
        *major_version = MAJOR_VERSION;
    }
    if !minor_version.is_null() {
        *minor_version = MINOR_VERSION;
    }
    if !micro_version.is_null() {
        *micro_version = MICRO_VERSION;
    }
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_APIVersionString() -> *mut ::core::ffi::c_char {
    return VERSION.as_ptr() as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn QRcode_clearCache() {}
pub const MAJOR_VERSION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MICRO_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"4.1.1\0") };
