extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: ::core::ffi::c_int,
        data: *const ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn QRinput_estimateBitsModeNum(size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn QRinput_estimateBitsModeAn(size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn QRinput_estimateBitsMode8(size: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static QRinput_anTable: [::core::ffi::c_schar; 128];
    fn QRspec_lengthIndicator(
        mode: QRencodeMode,
        version: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
unsafe extern "C" fn Split_identifyMode(
    mut string: *const ::core::ffi::c_char,
    mut hint: QRencodeMode,
) -> QRencodeMode {
    let mut c: ::core::ffi::c_uchar = 0;
    let mut d: ::core::ffi::c_uchar = 0;
    let mut word: ::core::ffi::c_uint = 0;
    c = *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar;
    if c as ::core::ffi::c_int == '\0' as i32 {
        return QR_MODE_NUL;
    }
    if ((c as ::core::ffi::c_schar as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uchar
        as ::core::ffi::c_int)
        < 10 as ::core::ffi::c_int
    {
        return QR_MODE_NUM;
    } else if (if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        QRinput_anTable[c as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    }) >= 0 as ::core::ffi::c_int
    {
        return QR_MODE_AN;
    } else if hint as ::core::ffi::c_int == QR_MODE_KANJI as ::core::ffi::c_int {
        d = *string.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar;
        if d as ::core::ffi::c_int != '\0' as i32 {
            word = (c as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int | d as ::core::ffi::c_uint;
            if word >= 0x8140 as ::core::ffi::c_uint && word <= 0x9ffc as ::core::ffi::c_uint
                || word >= 0xe040 as ::core::ffi::c_uint && word <= 0xebbf as ::core::ffi::c_uint
            {
                return QR_MODE_KANJI;
            }
        }
    }
    return QR_MODE_8;
}
unsafe extern "C" fn Split_eatNum(
    mut string: *const ::core::ffi::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut run: ::core::ffi::c_int = 0;
    let mut dif: ::core::ffi::c_int = 0;
    let mut ln: ::core::ffi::c_int = 0;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    p = string;
    while ((*p as ::core::ffi::c_schar as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uchar
        as ::core::ffi::c_int)
        < 10 as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    run = p.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int;
    mode = Split_identifyMode(p, hint);
    if mode as ::core::ffi::c_int == QR_MODE_8 as ::core::ffi::c_int {
        dif = QRinput_estimateBitsModeNum(run)
            + 4 as ::core::ffi::c_int
            + ln
            + QRinput_estimateBitsMode8(1 as ::core::ffi::c_int)
            - QRinput_estimateBitsMode8(run + 1 as ::core::ffi::c_int);
        if dif > 0 as ::core::ffi::c_int {
            return Split_eat8(string, input, hint);
        }
    }
    if mode as ::core::ffi::c_int == QR_MODE_AN as ::core::ffi::c_int {
        dif = QRinput_estimateBitsModeNum(run)
            + 4 as ::core::ffi::c_int
            + ln
            + QRinput_estimateBitsModeAn(1 as ::core::ffi::c_int)
            - QRinput_estimateBitsModeAn(run + 1 as ::core::ffi::c_int);
        if dif > 0 as ::core::ffi::c_int {
            return Split_eatAn(string, input, hint);
        }
    }
    ret = QRinput_append(input, QR_MODE_NUM, run, string as *mut ::core::ffi::c_uchar);
    if ret < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eatAn(
    mut string: *const ::core::ffi::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut run: ::core::ffi::c_int = 0;
    let mut dif: ::core::ffi::c_int = 0;
    let mut la: ::core::ffi::c_int = 0;
    let mut ln: ::core::ffi::c_int = 0;
    la = QRspec_lengthIndicator(QR_MODE_AN, (*input).version);
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    p = string;
    while (if *p as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        QRinput_anTable[*p as ::core::ffi::c_int as usize] as ::core::ffi::c_int
    }) >= 0 as ::core::ffi::c_int
    {
        if ((*p as ::core::ffi::c_schar as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uchar
            as ::core::ffi::c_int)
            < 10 as ::core::ffi::c_int
        {
            q = p;
            while ((*q as ::core::ffi::c_schar as ::core::ffi::c_int - '0' as i32)
                as ::core::ffi::c_uchar as ::core::ffi::c_int)
                < 10 as ::core::ffi::c_int
            {
                q = q.offset(1);
            }
            dif = QRinput_estimateBitsModeAn(
                p.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int
            ) + QRinput_estimateBitsModeNum(
                q.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int
            ) + 4 as ::core::ffi::c_int
                + ln
                + (if (if *q as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                    -(1 as ::core::ffi::c_int)
                } else {
                    QRinput_anTable[*q as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                }) >= 0 as ::core::ffi::c_int
                {
                    4 as ::core::ffi::c_int + ln
                } else {
                    0 as ::core::ffi::c_int
                })
                - QRinput_estimateBitsModeAn(
                    q.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int
                );
            if dif < 0 as ::core::ffi::c_int {
                break;
            }
            p = q;
        } else {
            p = p.offset(1);
        }
    }
    run = p.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int;
    if *p as ::core::ffi::c_int != 0
        && !((if *p as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            -(1 as ::core::ffi::c_int)
        } else {
            QRinput_anTable[*p as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        }) >= 0 as ::core::ffi::c_int)
    {
        dif = QRinput_estimateBitsModeAn(run)
            + 4 as ::core::ffi::c_int
            + la
            + QRinput_estimateBitsMode8(1 as ::core::ffi::c_int)
            - QRinput_estimateBitsMode8(run + 1 as ::core::ffi::c_int);
        if dif > 0 as ::core::ffi::c_int {
            return Split_eat8(string, input, hint);
        }
    }
    ret = QRinput_append(input, QR_MODE_AN, run, string as *mut ::core::ffi::c_uchar);
    if ret < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eatKanji(
    mut string: *const ::core::ffi::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut run: ::core::ffi::c_int = 0;
    p = string;
    while Split_identifyMode(p, hint) as ::core::ffi::c_int == QR_MODE_KANJI as ::core::ffi::c_int {
        p = p.offset(2 as ::core::ffi::c_int as isize);
    }
    run = p.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int;
    ret = QRinput_append(
        input,
        QR_MODE_KANJI,
        run,
        string as *mut ::core::ffi::c_uchar,
    );
    if ret < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eat8(
    mut string: *const ::core::ffi::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mode: QRencodeMode = QR_MODE_NUM;
    let mut ret: ::core::ffi::c_int = 0;
    let mut run: ::core::ffi::c_int = 0;
    let mut dif: ::core::ffi::c_int = 0;
    let mut la: ::core::ffi::c_int = 0;
    let mut ln: ::core::ffi::c_int = 0;
    let mut l8: ::core::ffi::c_int = 0;
    let mut swcost: ::core::ffi::c_int = 0;
    la = QRspec_lengthIndicator(QR_MODE_AN, (*input).version);
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    l8 = QRspec_lengthIndicator(QR_MODE_8, (*input).version);
    p = string.offset(1 as ::core::ffi::c_int as isize);
    while *p as ::core::ffi::c_int != '\0' as i32 {
        mode = Split_identifyMode(p, hint);
        if mode as ::core::ffi::c_int == QR_MODE_KANJI as ::core::ffi::c_int {
            break;
        }
        if mode as ::core::ffi::c_int == QR_MODE_NUM as ::core::ffi::c_int {
            q = p;
            while ((*q as ::core::ffi::c_schar as ::core::ffi::c_int - '0' as i32)
                as ::core::ffi::c_uchar as ::core::ffi::c_int)
                < 10 as ::core::ffi::c_int
            {
                q = q.offset(1);
            }
            if Split_identifyMode(q, hint) as ::core::ffi::c_int == QR_MODE_8 as ::core::ffi::c_int
            {
                swcost = 4 as ::core::ffi::c_int + l8;
            } else {
                swcost = 0 as ::core::ffi::c_int;
            }
            dif = QRinput_estimateBitsMode8(
                p.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int
            ) + QRinput_estimateBitsModeNum(
                q.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int
            ) + 4 as ::core::ffi::c_int
                + ln
                + swcost
                - QRinput_estimateBitsMode8(
                    q.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int
                );
            if dif < 0 as ::core::ffi::c_int {
                break;
            }
            p = q;
        } else if mode as ::core::ffi::c_int == QR_MODE_AN as ::core::ffi::c_int {
            q = p;
            while (if *q as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                -(1 as ::core::ffi::c_int)
            } else {
                QRinput_anTable[*q as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            }) >= 0 as ::core::ffi::c_int
            {
                q = q.offset(1);
            }
            if Split_identifyMode(q, hint) as ::core::ffi::c_int == QR_MODE_8 as ::core::ffi::c_int
            {
                swcost = 4 as ::core::ffi::c_int + l8;
            } else {
                swcost = 0 as ::core::ffi::c_int;
            }
            dif = QRinput_estimateBitsMode8(
                p.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int
            ) + QRinput_estimateBitsModeAn(
                q.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int
            ) + 4 as ::core::ffi::c_int
                + la
                + swcost
                - QRinput_estimateBitsMode8(
                    q.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int
                );
            if dif < 0 as ::core::ffi::c_int {
                break;
            }
            p = q;
        } else {
            p = p.offset(1);
        }
    }
    run = p.offset_from(string) as ::core::ffi::c_long as ::core::ffi::c_int;
    ret = QRinput_append(input, QR_MODE_8, run, string as *mut ::core::ffi::c_uchar);
    if ret < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return run;
}
unsafe extern "C" fn Split_splitString(
    mut string: *const ::core::ffi::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> ::core::ffi::c_int {
    let mut length: ::core::ffi::c_int = 0;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    while *string as ::core::ffi::c_int != '\0' as i32 {
        mode = Split_identifyMode(string, hint);
        if mode as ::core::ffi::c_int == QR_MODE_NUM as ::core::ffi::c_int {
            length = Split_eatNum(string, input, hint);
        } else if mode as ::core::ffi::c_int == QR_MODE_AN as ::core::ffi::c_int {
            length = Split_eatAn(string, input, hint);
        } else if mode as ::core::ffi::c_int == QR_MODE_KANJI as ::core::ffi::c_int
            && hint as ::core::ffi::c_int == QR_MODE_KANJI as ::core::ffi::c_int
        {
            length = Split_eatKanji(string, input, hint);
        } else {
            length = Split_eat8(string, input, hint);
        }
        if length == 0 as ::core::ffi::c_int {
            break;
        }
        if length < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        string = string.offset(length as isize);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn dupAndToUpper(
    mut str: *const ::core::ffi::c_char,
    mut hint: QRencodeMode,
) -> *mut ::core::ffi::c_char {
    let mut newstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mode: QRencodeMode = QR_MODE_NUM;
    newstr = strdup(str);
    if newstr.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = newstr;
    while *p as ::core::ffi::c_int != '\0' as i32 {
        mode = Split_identifyMode(p, hint);
        if mode as ::core::ffi::c_int == QR_MODE_KANJI as ::core::ffi::c_int {
            p = p.offset(2 as ::core::ffi::c_int as isize);
        } else {
            if *p as ::core::ffi::c_int >= 'a' as i32 && *p as ::core::ffi::c_int <= 'z' as i32 {
                *p = (*p as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_char;
            }
            p = p.offset(1);
        }
    }
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn Split_splitStringToQRinput(
    mut string: *const ::core::ffi::c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
    mut casesensitive: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut newstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ret: ::core::ffi::c_int = 0;
    if string.is_null() || *string as ::core::ffi::c_int == '\0' as i32 {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    if casesensitive == 0 {
        newstr = dupAndToUpper(string, hint);
        if newstr.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        ret = Split_splitString(newstr, input, hint);
        free(newstr as *mut ::core::ffi::c_void);
    } else {
        ret = Split_splitString(string, input, hint);
    }
    return ret;
}
