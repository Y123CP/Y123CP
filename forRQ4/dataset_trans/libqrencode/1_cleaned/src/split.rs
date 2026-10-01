use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    fn QRinput_append(
        input: *mut QRinput,
        mode: QRencodeMode,
        size: c_int,
        data: *const c_uchar,
    ) -> c_int;
    fn QRinput_estimateBitsModeNum(size: c_int) -> c_int;
    fn QRinput_estimateBitsModeAn(size: c_int) -> c_int;
    fn QRinput_estimateBitsMode8(size: c_int) -> c_int;
    static QRinput_anTable: [c_schar; 128];
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

unsafe extern "C" fn Split_identifyMode(
    mut string: *const c_char,
    mut hint: QRencodeMode,
) -> QRencodeMode {
    let mut c: c_uchar = 0;
    let mut d: c_uchar = 0;
    let mut word: c_uint = 0;
    c = *string.offset(0 as c_int as isize) as c_uchar;
    if c as c_int == '\0' as i32 {
        return QR_MODE_NUL;
    }
    if ((c as c_schar as c_int - '0' as i32) as c_uchar
        as c_int)
        < 10 as c_int
    {
        return QR_MODE_NUM;
    } else if (if c as c_int & 0x80 as c_int != 0 {
        -(1 as c_int)
    } else {
        QRinput_anTable[c as c_int as usize] as c_int
    }) >= 0 as c_int
    {
        return QR_MODE_AN;
    } else if hint as c_int == QR_MODE_KANJI as c_int {
        d = *string.offset(1 as c_int as isize) as c_uchar;
        if d as c_int != '\0' as i32 {
            word = (c as c_uint) << 8 as c_int | d as c_uint;
            if word >= 0x8140 as c_uint && word <= 0x9ffc as c_uint
                || word >= 0xe040 as c_uint && word <= 0xebbf as c_uint
            {
                return QR_MODE_KANJI;
            }
        }
    }
    return QR_MODE_8;
}
unsafe extern "C" fn Split_eatNum(
    mut string: *const c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> c_int {
    let mut p: *const c_char = ::core::ptr::null::<c_char>();
    let mut ret: c_int = 0;
    let mut run: c_int = 0;
    let mut dif: c_int = 0;
    let mut ln: c_int = 0;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    p = string;
    while ((*p as c_schar as c_int - '0' as i32) as c_uchar
        as c_int)
        < 10 as c_int
    {
        p = p.offset(1);
    }
    run = p.offset_from(string) as c_long as c_int;
    mode = Split_identifyMode(p, hint);
    if mode as c_int == QR_MODE_8 as c_int {
        dif = QRinput_estimateBitsModeNum(run)
            + 4 as c_int
            + ln
            + QRinput_estimateBitsMode8(1 as c_int)
            - QRinput_estimateBitsMode8(run + 1 as c_int);
        if dif > 0 as c_int {
            return Split_eat8(string, input, hint);
        }
    }
    if mode as c_int == QR_MODE_AN as c_int {
        dif = QRinput_estimateBitsModeNum(run)
            + 4 as c_int
            + ln
            + QRinput_estimateBitsModeAn(1 as c_int)
            - QRinput_estimateBitsModeAn(run + 1 as c_int);
        if dif > 0 as c_int {
            return Split_eatAn(string, input, hint);
        }
    }
    ret = QRinput_append(input, QR_MODE_NUM, run, string as *mut c_uchar);
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eatAn(
    mut string: *const c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> c_int {
    let mut p: *const c_char = ::core::ptr::null::<c_char>();
    let mut q: *const c_char = ::core::ptr::null::<c_char>();
    let mut ret: c_int = 0;
    let mut run: c_int = 0;
    let mut dif: c_int = 0;
    let mut la: c_int = 0;
    let mut ln: c_int = 0;
    la = QRspec_lengthIndicator(QR_MODE_AN, (*input).version);
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    p = string;
    while (if *p as c_int & 0x80 as c_int != 0 {
        -(1 as c_int)
    } else {
        QRinput_anTable[*p as c_int as usize] as c_int
    }) >= 0 as c_int
    {
        if ((*p as c_schar as c_int - '0' as i32) as c_uchar
            as c_int)
            < 10 as c_int
        {
            q = p;
            while ((*q as c_schar as c_int - '0' as i32)
                as c_uchar as c_int)
                < 10 as c_int
            {
                q = q.offset(1);
            }
            dif = QRinput_estimateBitsModeAn(
                p.offset_from(string) as c_long as c_int
            ) + QRinput_estimateBitsModeNum(
                q.offset_from(p) as c_long as c_int
            ) + 4 as c_int
                + ln
                + (if (if *q as c_int & 0x80 as c_int != 0 {
                    -(1 as c_int)
                } else {
                    QRinput_anTable[*q as c_int as usize] as c_int
                }) >= 0 as c_int
                {
                    4 as c_int + ln
                } else {
                    0 as c_int
                })
                - QRinput_estimateBitsModeAn(
                    q.offset_from(string) as c_long as c_int
                );
            if dif < 0 as c_int {
                break;
            }
            p = q;
        } else {
            p = p.offset(1);
        }
    }
    run = p.offset_from(string) as c_long as c_int;
    if *p as c_int != 0
        && !((if *p as c_int & 0x80 as c_int != 0 {
            -(1 as c_int)
        } else {
            QRinput_anTable[*p as c_int as usize] as c_int
        }) >= 0 as c_int)
    {
        dif = QRinput_estimateBitsModeAn(run)
            + 4 as c_int
            + la
            + QRinput_estimateBitsMode8(1 as c_int)
            - QRinput_estimateBitsMode8(run + 1 as c_int);
        if dif > 0 as c_int {
            return Split_eat8(string, input, hint);
        }
    }
    ret = QRinput_append(input, QR_MODE_AN, run, string as *mut c_uchar);
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eatKanji(
    mut string: *const c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> c_int {
    let mut p: *const c_char = ::core::ptr::null::<c_char>();
    let mut ret: c_int = 0;
    let mut run: c_int = 0;
    p = string;
    while Split_identifyMode(p, hint) as c_int == QR_MODE_KANJI as c_int {
        p = p.offset(2 as c_int as isize);
    }
    run = p.offset_from(string) as c_long as c_int;
    ret = QRinput_append(
        input,
        QR_MODE_KANJI,
        run,
        string as *mut c_uchar,
    );
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return run;
}
unsafe extern "C" fn Split_eat8(
    mut string: *const c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> c_int {
    let mut p: *const c_char = ::core::ptr::null::<c_char>();
    let mut q: *const c_char = ::core::ptr::null::<c_char>();
    let mut mode: QRencodeMode = QR_MODE_NUM;
    let mut ret: c_int = 0;
    let mut run: c_int = 0;
    let mut dif: c_int = 0;
    let mut la: c_int = 0;
    let mut ln: c_int = 0;
    let mut l8: c_int = 0;
    let mut swcost: c_int = 0;
    la = QRspec_lengthIndicator(QR_MODE_AN, (*input).version);
    ln = QRspec_lengthIndicator(QR_MODE_NUM, (*input).version);
    l8 = QRspec_lengthIndicator(QR_MODE_8, (*input).version);
    p = string.offset(1 as c_int as isize);
    while *p as c_int != '\0' as i32 {
        mode = Split_identifyMode(p, hint);
        if mode as c_int == QR_MODE_KANJI as c_int {
            break;
        }
        if mode as c_int == QR_MODE_NUM as c_int {
            q = p;
            while ((*q as c_schar as c_int - '0' as i32)
                as c_uchar as c_int)
                < 10 as c_int
            {
                q = q.offset(1);
            }
            if Split_identifyMode(q, hint) as c_int == QR_MODE_8 as c_int
            {
                swcost = 4 as c_int + l8;
            } else {
                swcost = 0 as c_int;
            }
            dif = QRinput_estimateBitsMode8(
                p.offset_from(string) as c_long as c_int
            ) + QRinput_estimateBitsModeNum(
                q.offset_from(p) as c_long as c_int
            ) + 4 as c_int
                + ln
                + swcost
                - QRinput_estimateBitsMode8(
                    q.offset_from(string) as c_long as c_int
                );
            if dif < 0 as c_int {
                break;
            }
            p = q;
        } else if mode as c_int == QR_MODE_AN as c_int {
            q = p;
            while (if *q as c_int & 0x80 as c_int != 0 {
                -(1 as c_int)
            } else {
                QRinput_anTable[*q as c_int as usize] as c_int
            }) >= 0 as c_int
            {
                q = q.offset(1);
            }
            if Split_identifyMode(q, hint) as c_int == QR_MODE_8 as c_int
            {
                swcost = 4 as c_int + l8;
            } else {
                swcost = 0 as c_int;
            }
            dif = QRinput_estimateBitsMode8(
                p.offset_from(string) as c_long as c_int
            ) + QRinput_estimateBitsModeAn(
                q.offset_from(p) as c_long as c_int
            ) + 4 as c_int
                + la
                + swcost
                - QRinput_estimateBitsMode8(
                    q.offset_from(string) as c_long as c_int
                );
            if dif < 0 as c_int {
                break;
            }
            p = q;
        } else {
            p = p.offset(1);
        }
    }
    run = p.offset_from(string) as c_long as c_int;
    ret = QRinput_append(input, QR_MODE_8, run, string as *mut c_uchar);
    if ret < 0 as c_int {
        return -(1 as c_int);
    }
    return run;
}
unsafe extern "C" fn Split_splitString(
    mut string: *const c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
) -> c_int {
    let mut length: c_int = 0;
    let mut mode: QRencodeMode = QR_MODE_NUM;
    while *string as c_int != '\0' as i32 {
        mode = Split_identifyMode(string, hint);
        if mode as c_int == QR_MODE_NUM as c_int {
            length = Split_eatNum(string, input, hint);
        } else if mode as c_int == QR_MODE_AN as c_int {
            length = Split_eatAn(string, input, hint);
        } else if mode as c_int == QR_MODE_KANJI as c_int
            && hint as c_int == QR_MODE_KANJI as c_int
        {
            length = Split_eatKanji(string, input, hint);
        } else {
            length = Split_eat8(string, input, hint);
        }
        if length == 0 as c_int {
            break;
        }
        if length < 0 as c_int {
            return -(1 as c_int);
        }
        string = string.offset(length as isize);
    }
    return 0 as c_int;
}
unsafe extern "C" fn dupAndToUpper(
    mut str: *const c_char,
    mut hint: QRencodeMode,
) -> *mut c_char {
    let mut newstr: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut p: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut mode: QRencodeMode = QR_MODE_NUM;
    newstr = strdup(str);
    if newstr.is_null() {
        return ::core::ptr::null_mut::<c_char>();
    }
    p = newstr;
    while *p as c_int != '\0' as i32 {
        mode = Split_identifyMode(p, hint);
        if mode as c_int == QR_MODE_KANJI as c_int {
            p = p.offset(2 as c_int as isize);
        } else {
            if *p as c_int >= 'a' as i32 && *p as c_int <= 'z' as i32 {
                *p = (*p as c_int - 32 as c_int) as c_char;
            }
            p = p.offset(1);
        }
    }
    return newstr;
}
#[no_mangle]
pub unsafe extern "C" fn Split_splitStringToQRinput(
    mut string: *const c_char,
    mut input: *mut QRinput,
    mut hint: QRencodeMode,
    mut casesensitive: c_int,
) -> c_int {
    let mut newstr: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut ret: c_int = 0;
    if string.is_null() || *string as c_int == '\0' as i32 {
        *__errno_location() = EINVAL;
        return -(1 as c_int);
    }
    if casesensitive == 0 {
        newstr = dupAndToUpper(string, hint);
        if newstr.is_null() {
            return -(1 as c_int);
        }
        ret = Split_splitString(newstr, input, hint);
        free(newstr as *mut c_void);
    } else {
        ret = Split_splitString(string, input, hint);
    }
    return ret;
}
