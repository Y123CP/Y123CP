use ::core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zadd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: ::core::ffi::c_int,
    pub padding__: ::core::ffi::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [zahl; 1];
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d.offset((20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((20 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1391058008256916972;
        }
        19 => {
            current_block_42 = 1391058008256916972;
        }
        18 => {
            current_block_42 = 6893763438629148997;
        }
        17 => {
            current_block_42 = 6421391391121710;
        }
        16 => {
            current_block_42 = 17162270685673637631;
        }
        15 => {
            current_block_42 = 451532537293855284;
        }
        14 => {
            current_block_42 = 11729455350843475820;
        }
        13 => {
            current_block_42 = 3088197500626375644;
        }
        12 => {
            current_block_42 = 4423598539606377485;
        }
        11 => {
            current_block_42 = 5993199832596135542;
        }
        10 => {
            current_block_42 = 4636481217339069480;
        }
        9 => {
            current_block_42 = 12386775860387733485;
        }
        8 => {
            current_block_42 = 14468891521630732394;
        }
        7 => {
            current_block_42 = 13031440908851014233;
        }
        6 => {
            current_block_42 = 8848739226222623272;
        }
        5 => {
            current_block_42 = 11538747596021838440;
        }
        4 => {
            current_block_42 = 1606853116260846100;
        }
        3 => {
            current_block_42 = 1919423070859572111;
        }
        2 => {
            current_block_42 = 8266687893124359221;
        }
        1 => {
            current_block_42 = 1890451546187205982;
        }
        0 => {
            current_block_42 = 1836292691772056875;
        }
        _ => {
            let mut t: zahl_char_t = 0;
            asm!(
                "\n", "    shlq $3, {3}\n", "    addq {1}, {3}\n", " 1:\n",
                "    movq 0({2}), {0}\n", "    movq {0}, 0({1})\n",
                "    movq 8({2}), {0}\n", "    movq {0}, 8({1})\n",
                "    movq 16({2}), {0}\n", "    movq {0}, 16({1})\n",
                "    movq 24({2}), {0}\n", "    movq {0}, 24({1})\n",
                "    addq $32, {2}\n", "    addq $32, {1}\n", "    cmpq {3}, {1}\n",
                "    jl 1b\n", lateout(reg) t, inlateout(reg) d, inlateout(reg) s,
                inlateout(reg) n, options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        1391058008256916972 => {
            *d.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((19 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 6893763438629148997;
        }
        _ => {}
    }
    match current_block_42 {
        6893763438629148997 => {
            *d.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((18 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 6421391391121710;
        }
        _ => {}
    }
    match current_block_42 {
        6421391391121710 => {
            *d.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((17 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 17162270685673637631;
        }
        _ => {}
    }
    match current_block_42 {
        17162270685673637631 => {
            *d.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 451532537293855284;
        }
        _ => {}
    }
    match current_block_42 {
        451532537293855284 => {
            *d.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 11729455350843475820;
        }
        _ => {}
    }
    match current_block_42 {
        11729455350843475820 => {
            *d.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 3088197500626375644;
        }
        _ => {}
    }
    match current_block_42 {
        3088197500626375644 => {
            *d.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 4423598539606377485;
        }
        _ => {}
    }
    match current_block_42 {
        4423598539606377485 => {
            *d.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 5993199832596135542;
        }
        _ => {}
    }
    match current_block_42 {
        5993199832596135542 => {
            *d.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 4636481217339069480;
        }
        _ => {}
    }
    match current_block_42 {
        4636481217339069480 => {
            *d.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 12386775860387733485;
        }
        _ => {}
    }
    match current_block_42 {
        12386775860387733485 => {
            *d.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 14468891521630732394;
        }
        _ => {}
    }
    match current_block_42 {
        14468891521630732394 => {
            *d.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 13031440908851014233;
        }
        _ => {}
    }
    match current_block_42 {
        13031440908851014233 => {
            *d.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((7 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 8848739226222623272;
        }
        _ => {}
    }
    match current_block_42 {
        8848739226222623272 => {
            *d.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 11538747596021838440;
        }
        _ => {}
    }
    match current_block_42 {
        11538747596021838440 => {
            *d.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1606853116260846100;
        }
        _ => {}
    }
    match current_block_42 {
        1606853116260846100 => {
            *d.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1919423070859572111;
        }
        _ => {}
    }
    match current_block_42 {
        1919423070859572111 => {
            *d.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 8266687893124359221;
        }
        _ => {}
    }
    match current_block_42 {
        8266687893124359221 => {
            *d.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
            current_block_42 = 1890451546187205982;
        }
        _ => {}
    }
    match current_block_42 {
        1890451546187205982 => {
            *d.offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *s.offset((1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> ::core::ffi::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = 0 as ::core::ffi::c_int;
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used {
            libzahl_realloc(a as *mut zahl, (*b).used);
        }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zmul(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut b_sign: ::core::ffi::c_int = 0;
    let mut c_sign: ::core::ffi::c_int = 0;
    b_sign = (*b).sign;
    (*b).sign *= b_sign;
    c_sign = (*c).sign;
    (*c).sign *= c_sign;
    zmul_ll(a, b, c);
    (*c).sign = c_sign;
    (*b).sign = b_sign;
    (*a).sign = zsignum(b) * zsignum(c);
}
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn zsets(
    mut a: *mut zahl,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut temp: ::core::ffi::c_ulonglong = 0 as ::core::ffi::c_ulonglong;
    let mut neg: ::core::ffi::c_int =
        (*str as ::core::ffi::c_int == '-' as i32) as ::core::ffi::c_int;
    let mut str_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    str = str.offset(
        (neg != 0 || *str as ::core::ffi::c_int == '+' as i32) as ::core::ffi::c_int as isize,
    );
    if (*str == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        *__errno_location() = EINVAL;
        return -(1 as ::core::ffi::c_int);
    }
    str_end = str;
    while *str_end != 0 {
        if (*(*__ctype_b_loc()).offset(*str_end as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            *__errno_location() = EINVAL;
            return -(1 as ::core::ffi::c_int);
        }
        str_end = str_end.offset(1);
    }
    (*a).sign = 0 as ::core::ffi::c_int;
    zset(
        &raw mut libzahl_tmp_str_num as *mut zahl,
        &raw mut libzahl_const_1e19 as *mut zahl,
    );
    's_183: {
        let mut current_block_36: u64;
        match str_end.offset_from(str) as ::core::ffi::c_long % 19 as ::core::ffi::c_long {
            0 => {
                current_block_36 = 435944291641368317;
            }
            18 => {
                current_block_36 = 15720265420250078001;
            }
            17 => {
                current_block_36 = 15498226013514017913;
            }
            16 => {
                current_block_36 = 9964442620838245131;
            }
            15 => {
                current_block_36 = 17685150860668569253;
            }
            14 => {
                current_block_36 = 15193246926733009155;
            }
            13 => {
                current_block_36 = 1100798251586648964;
            }
            12 => {
                current_block_36 = 17354110723917477434;
            }
            11 => {
                current_block_36 = 6757825451543678428;
            }
            10 => {
                current_block_36 = 17757625703761429344;
            }
            9 => {
                current_block_36 = 15546349514742412310;
            }
            8 => {
                current_block_36 = 5963766169110708753;
            }
            7 => {
                current_block_36 = 18118272086669285738;
            }
            6 => {
                current_block_36 = 9102062007102360857;
            }
            5 => {
                current_block_36 = 1209784605738317917;
            }
            4 => {
                current_block_36 = 2023459610091673929;
            }
            3 => {
                current_block_36 = 14827723604853618344;
            }
            2 => {
                current_block_36 = 13717851870030888558;
            }
            1 => {
                current_block_36 = 15093209673654042025;
            }
            _ => {
                current_block_36 = 10692455896603418738;
            }
        }
        loop {
            match current_block_36 {
                10692455896603418738 => {
                    break 's_183;
                }
                435944291641368317 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh0 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh0 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 15720265420250078001;
                }
                15720265420250078001 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh1 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 15498226013514017913;
                }
                15498226013514017913 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh2 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 9964442620838245131;
                }
                9964442620838245131 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh3 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 17685150860668569253;
                }
                17685150860668569253 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh4 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh4 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 15193246926733009155;
                }
                15193246926733009155 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh5 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh5 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 1100798251586648964;
                }
                1100798251586648964 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh6 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh6 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 17354110723917477434;
                }
                17354110723917477434 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh7 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh7 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 6757825451543678428;
                }
                6757825451543678428 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh8 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 17757625703761429344;
                }
                17757625703761429344 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh9 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh9 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 15546349514742412310;
                }
                15546349514742412310 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh10 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh10 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 5963766169110708753;
                }
                5963766169110708753 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh11 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh11 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 18118272086669285738;
                }
                18118272086669285738 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh12 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh12 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 9102062007102360857;
                }
                9102062007102360857 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh13 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh13 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 1209784605738317917;
                }
                1209784605738317917 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh14 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 2023459610091673929;
                }
                2023459610091673929 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh15 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh15 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 14827723604853618344;
                }
                14827723604853618344 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh16 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh16 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 13717851870030888558;
                }
                13717851870030888558 => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh17 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh17 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    current_block_36 = 15093209673654042025;
                }
                _ => {
                    temp = temp.wrapping_mul(10 as ::core::ffi::c_ulonglong);
                    let fresh18 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh18 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                            as ::core::ffi::c_ulonglong,
                    );
                    if !(temp == 0) {
                        *(*(&raw mut libzahl_tmp_str_num as *mut zahl))
                            .chars
                            .offset(0 as ::core::ffi::c_int as isize) = temp as zahl_char_t;
                        zadd(a, a, &raw mut libzahl_tmp_str_num as *mut zahl);
                    }
                    if !(*str != 0) {
                        current_block_36 = 10692455896603418738;
                        continue;
                    }
                    zmul(a, a, &raw mut libzahl_const_1e19 as *mut zahl);
                    temp = 0 as ::core::ffi::c_ulonglong;
                    current_block_36 = 435944291641368317;
                }
            }
        }
    }
    if (neg != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        (*a).sign = -zsignum(a);
    }
    return 0 as ::core::ffi::c_int;
}
