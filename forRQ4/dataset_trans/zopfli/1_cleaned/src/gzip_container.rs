use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;

static mut crc32_table: [c_ulong; 256] = [
    0 as c_uint as c_ulong,
    1996959894 as c_uint as c_ulong,
    3993919788 as c_uint as c_ulong,
    2567524794 as c_uint as c_ulong,
    124634137 as c_uint as c_ulong,
    1886057615 as c_uint as c_ulong,
    3915621685 as c_uint as c_ulong,
    2657392035 as c_uint as c_ulong,
    249268274 as c_uint as c_ulong,
    2044508324 as c_uint as c_ulong,
    3772115230 as c_uint as c_ulong,
    2547177864 as c_uint as c_ulong,
    162941995 as c_uint as c_ulong,
    2125561021 as c_uint as c_ulong,
    3887607047 as c_uint as c_ulong,
    2428444049 as c_uint as c_ulong,
    498536548 as c_uint as c_ulong,
    1789927666 as c_uint as c_ulong,
    4089016648 as c_uint as c_ulong,
    2227061214 as c_uint as c_ulong,
    450548861 as c_uint as c_ulong,
    1843258603 as c_uint as c_ulong,
    4107580753 as c_uint as c_ulong,
    2211677639 as c_uint as c_ulong,
    325883990 as c_uint as c_ulong,
    1684777152 as c_uint as c_ulong,
    4251122042 as c_uint as c_ulong,
    2321926636 as c_uint as c_ulong,
    335633487 as c_uint as c_ulong,
    1661365465 as c_uint as c_ulong,
    4195302755 as c_uint as c_ulong,
    2366115317 as c_uint as c_ulong,
    997073096 as c_uint as c_ulong,
    1281953886 as c_uint as c_ulong,
    3579855332 as c_uint as c_ulong,
    2724688242 as c_uint as c_ulong,
    1006888145 as c_uint as c_ulong,
    1258607687 as c_uint as c_ulong,
    3524101629 as c_uint as c_ulong,
    2768942443 as c_uint as c_ulong,
    901097722 as c_uint as c_ulong,
    1119000684 as c_uint as c_ulong,
    3686517206 as c_uint as c_ulong,
    2898065728 as c_uint as c_ulong,
    853044451 as c_uint as c_ulong,
    1172266101 as c_uint as c_ulong,
    3705015759 as c_uint as c_ulong,
    2882616665 as c_uint as c_ulong,
    651767980 as c_uint as c_ulong,
    1373503546 as c_uint as c_ulong,
    3369554304 as c_uint as c_ulong,
    3218104598 as c_uint as c_ulong,
    565507253 as c_uint as c_ulong,
    1454621731 as c_uint as c_ulong,
    3485111705 as c_uint as c_ulong,
    3099436303 as c_uint as c_ulong,
    671266974 as c_uint as c_ulong,
    1594198024 as c_uint as c_ulong,
    3322730930 as c_uint as c_ulong,
    2970347812 as c_uint as c_ulong,
    795835527 as c_uint as c_ulong,
    1483230225 as c_uint as c_ulong,
    3244367275 as c_uint as c_ulong,
    3060149565 as c_uint as c_ulong,
    1994146192 as c_uint as c_ulong,
    31158534 as c_uint as c_ulong,
    2563907772 as c_uint as c_ulong,
    4023717930 as c_uint as c_ulong,
    1907459465 as c_uint as c_ulong,
    112637215 as c_uint as c_ulong,
    2680153253 as c_uint as c_ulong,
    3904427059 as c_uint as c_ulong,
    2013776290 as c_uint as c_ulong,
    251722036 as c_uint as c_ulong,
    2517215374 as c_uint as c_ulong,
    3775830040 as c_uint as c_ulong,
    2137656763 as c_uint as c_ulong,
    141376813 as c_uint as c_ulong,
    2439277719 as c_uint as c_ulong,
    3865271297 as c_uint as c_ulong,
    1802195444 as c_uint as c_ulong,
    476864866 as c_uint as c_ulong,
    2238001368 as c_uint as c_ulong,
    4066508878 as c_uint as c_ulong,
    1812370925 as c_uint as c_ulong,
    453092731 as c_uint as c_ulong,
    2181625025 as c_uint as c_ulong,
    4111451223 as c_uint as c_ulong,
    1706088902 as c_uint as c_ulong,
    314042704 as c_uint as c_ulong,
    2344532202 as c_uint as c_ulong,
    4240017532 as c_uint as c_ulong,
    1658658271 as c_uint as c_ulong,
    366619977 as c_uint as c_ulong,
    2362670323 as c_uint as c_ulong,
    4224994405 as c_uint as c_ulong,
    1303535960 as c_uint as c_ulong,
    984961486 as c_uint as c_ulong,
    2747007092 as c_uint as c_ulong,
    3569037538 as c_uint as c_ulong,
    1256170817 as c_uint as c_ulong,
    1037604311 as c_uint as c_ulong,
    2765210733 as c_uint as c_ulong,
    3554079995 as c_uint as c_ulong,
    1131014506 as c_uint as c_ulong,
    879679996 as c_uint as c_ulong,
    2909243462 as c_uint as c_ulong,
    3663771856 as c_uint as c_ulong,
    1141124467 as c_uint as c_ulong,
    855842277 as c_uint as c_ulong,
    2852801631 as c_uint as c_ulong,
    3708648649 as c_uint as c_ulong,
    1342533948 as c_uint as c_ulong,
    654459306 as c_uint as c_ulong,
    3188396048 as c_uint as c_ulong,
    3373015174 as c_uint as c_ulong,
    1466479909 as c_uint as c_ulong,
    544179635 as c_uint as c_ulong,
    3110523913 as c_uint as c_ulong,
    3462522015 as c_uint as c_ulong,
    1591671054 as c_uint as c_ulong,
    702138776 as c_uint as c_ulong,
    2966460450 as c_uint as c_ulong,
    3352799412 as c_uint as c_ulong,
    1504918807 as c_uint as c_ulong,
    783551873 as c_uint as c_ulong,
    3082640443 as c_uint as c_ulong,
    3233442989 as c_uint as c_ulong,
    3988292384 as c_uint as c_ulong,
    2596254646 as c_uint as c_ulong,
    62317068 as c_uint as c_ulong,
    1957810842 as c_uint as c_ulong,
    3939845945 as c_uint as c_ulong,
    2647816111 as c_uint as c_ulong,
    81470997 as c_uint as c_ulong,
    1943803523 as c_uint as c_ulong,
    3814918930 as c_uint as c_ulong,
    2489596804 as c_uint as c_ulong,
    225274430 as c_uint as c_ulong,
    2053790376 as c_uint as c_ulong,
    3826175755 as c_uint as c_ulong,
    2466906013 as c_uint as c_ulong,
    167816743 as c_uint as c_ulong,
    2097651377 as c_uint as c_ulong,
    4027552580 as c_uint as c_ulong,
    2265490386 as c_uint as c_ulong,
    503444072 as c_uint as c_ulong,
    1762050814 as c_uint as c_ulong,
    4150417245 as c_uint as c_ulong,
    2154129355 as c_uint as c_ulong,
    426522225 as c_uint as c_ulong,
    1852507879 as c_uint as c_ulong,
    4275313526 as c_uint as c_ulong,
    2312317920 as c_uint as c_ulong,
    282753626 as c_uint as c_ulong,
    1742555852 as c_uint as c_ulong,
    4189708143 as c_uint as c_ulong,
    2394877945 as c_uint as c_ulong,
    397917763 as c_uint as c_ulong,
    1622183637 as c_uint as c_ulong,
    3604390888 as c_uint as c_ulong,
    2714866558 as c_uint as c_ulong,
    953729732 as c_uint as c_ulong,
    1340076626 as c_uint as c_ulong,
    3518719985 as c_uint as c_ulong,
    2797360999 as c_uint as c_ulong,
    1068828381 as c_uint as c_ulong,
    1219638859 as c_uint as c_ulong,
    3624741850 as c_uint as c_ulong,
    2936675148 as c_uint as c_ulong,
    906185462 as c_uint as c_ulong,
    1090812512 as c_uint as c_ulong,
    3747672003 as c_uint as c_ulong,
    2825379669 as c_uint as c_ulong,
    829329135 as c_uint as c_ulong,
    1181335161 as c_uint as c_ulong,
    3412177804 as c_uint as c_ulong,
    3160834842 as c_uint as c_ulong,
    628085408 as c_uint as c_ulong,
    1382605366 as c_uint as c_ulong,
    3423369109 as c_uint as c_ulong,
    3138078467 as c_uint as c_ulong,
    570562233 as c_uint as c_ulong,
    1426400815 as c_uint as c_ulong,
    3317316542 as c_uint as c_ulong,
    2998733608 as c_uint as c_ulong,
    733239954 as c_uint as c_ulong,
    1555261956 as c_uint as c_ulong,
    3268935591 as c_uint as c_ulong,
    3050360625 as c_uint as c_ulong,
    752459403 as c_uint as c_ulong,
    1541320221 as c_uint as c_ulong,
    2607071920 as c_uint as c_ulong,
    3965973030 as c_uint as c_ulong,
    1969922972 as c_uint as c_ulong,
    40735498 as c_uint as c_ulong,
    2617837225 as c_uint as c_ulong,
    3943577151 as c_uint as c_ulong,
    1913087877 as c_uint as c_ulong,
    83908371 as c_uint as c_ulong,
    2512341634 as c_uint as c_ulong,
    3803740692 as c_uint as c_ulong,
    2075208622 as c_uint as c_ulong,
    213261112 as c_uint as c_ulong,
    2463272603 as c_uint as c_ulong,
    3855990285 as c_uint as c_ulong,
    2094854071 as c_uint as c_ulong,
    198958881 as c_uint as c_ulong,
    2262029012 as c_uint as c_ulong,
    4057260610 as c_uint as c_ulong,
    1759359992 as c_uint as c_ulong,
    534414190 as c_uint as c_ulong,
    2176718541 as c_uint as c_ulong,
    4139329115 as c_uint as c_ulong,
    1873836001 as c_uint as c_ulong,
    414664567 as c_uint as c_ulong,
    2282248934 as c_uint as c_ulong,
    4279200368 as c_uint as c_ulong,
    1711684554 as c_uint as c_ulong,
    285281116 as c_uint as c_ulong,
    2405801727 as c_uint as c_ulong,
    4167216745 as c_uint as c_ulong,
    1634467795 as c_uint as c_ulong,
    376229701 as c_uint as c_ulong,
    2685067896 as c_uint as c_ulong,
    3608007406 as c_uint as c_ulong,
    1308918612 as c_uint as c_ulong,
    956543938 as c_uint as c_ulong,
    2808555105 as c_uint as c_ulong,
    3495958263 as c_uint as c_ulong,
    1231636301 as c_uint as c_ulong,
    1047427035 as c_uint as c_ulong,
    2932959818 as c_uint as c_ulong,
    3654703836 as c_uint as c_ulong,
    1088359270 as c_uint as c_ulong,
    936918000 as c_uint as c_ulong,
    2847714899 as c_uint as c_ulong,
    3736837829 as c_uint as c_ulong,
    1202900863 as c_uint as c_ulong,
    817233897 as c_uint as c_ulong,
    3183342108 as c_uint as c_ulong,
    3401237130 as c_uint as c_ulong,
    1404277552 as c_uint as c_ulong,
    615818150 as c_uint as c_ulong,
    3134207493 as c_uint as c_ulong,
    3453421203 as c_uint as c_ulong,
    1423857449 as c_uint as c_ulong,
    601450431 as c_uint as c_ulong,
    3009837614 as c_uint as c_ulong,
    3294710456 as c_uint as c_ulong,
    1567103746 as c_uint as c_ulong,
    711928724 as c_uint as c_ulong,
    3020668471 as c_uint as c_ulong,
    3272380065 as c_uint as c_ulong,
    1510334235 as c_uint as c_ulong,
    755167117 as c_uint as c_ulong,
];
unsafe extern "C" fn CRC(
    mut data: *const c_uchar,
    mut size: size_t,
) -> c_ulong {
    let mut result: c_ulong = 0xffffffff as c_ulong;
    while size > 0 as size_t {
        let fresh0 = data;
        data = data.offset(1);
        result = crc32_table
            [((result ^ *fresh0 as c_ulong) & 0xff as c_ulong) as usize]
            ^ result >> 8 as c_int;
        size = size.wrapping_sub(1);
    }
    return result ^ 0xffffffff as c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn ZopfliGzipCompress(
    mut options: *const ZopfliOptions,
    mut in_0: *const c_uchar,
    mut insize: size_t,
    mut out: *mut *mut c_uchar,
    mut outsize: *mut size_t,
) {
    let mut crcvalue: c_ulong = CRC(in_0, insize);
    let mut bp: c_uchar = 0 as c_uchar;
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 31 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 139 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 8 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 0 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 2 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = 3 as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    ZopfliDeflate(
        options,
        2 as c_int,
        1 as c_int,
        in_0,
        insize,
        &raw mut bp,
        out,
        outsize,
    );
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) =
        crcvalue.wrapping_rem(256 as c_ulong) as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = (crcvalue >> 8 as c_int)
        .wrapping_rem(256 as c_ulong)
        as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = (crcvalue >> 16 as c_int)
        .wrapping_rem(256 as c_ulong)
        as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = (crcvalue >> 24 as c_int)
        .wrapping_rem(256 as c_ulong)
        as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) = insize.wrapping_rem(256 as size_t) as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) =
        (insize >> 8 as c_int).wrapping_rem(256 as size_t) as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) =
        (insize >> 16 as c_int).wrapping_rem(256 as size_t) as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if *outsize & (*outsize).wrapping_sub(1 as size_t) == 0 {
        *out = (if *outsize == 0 as size_t {
            malloc(::core::mem::size_of::<c_uchar>() as size_t)
        } else {
            realloc(
                *out as *mut c_void,
                (*outsize)
                    .wrapping_mul(2 as size_t)
                    .wrapping_mul(::core::mem::size_of::<c_uchar>() as size_t),
            )
        }) as *mut c_uchar;
    }
    *(*out).offset(*outsize as isize) =
        (insize >> 24 as c_int).wrapping_rem(256 as size_t) as c_uchar;
    *outsize = (*outsize).wrapping_add(1);
    if (*options).verbose != 0 {
        fprintf(
            stderr,
            b"Original Size: %d, Gzip: %d, Compression: %f%% Removed\n\0" as *const u8
                as *const c_char,
            insize as c_int,
            *outsize as c_int,
            100.0f64 * insize.wrapping_sub(*outsize) as c_double
                / insize as c_double,
        );
    }
}
