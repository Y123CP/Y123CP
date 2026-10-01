pub type size_t = usize;
pub type z_size_t = size_t;
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type __off_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub const Z_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BASE: ::core::ffi::c_uint = 65521 as ::core::ffi::c_uint;
pub const NMAX: ::core::ffi::c_int = 5552 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn adler32_z(
    mut adler: uLong,
    mut buf: *const Bytef,
    mut len: z_size_t,
) -> uLong {
    let mut sum2: ::core::ffi::c_ulong = 0;
    let mut n: ::core::ffi::c_uint = 0;
    sum2 =
        adler as ::core::ffi::c_ulong >> 16 as ::core::ffi::c_int & 0xffff as ::core::ffi::c_ulong;
    adler &= 0xffff as ::core::ffi::c_ulong;
    if len == 1 as z_size_t {
        adler = (adler as ::core::ffi::c_ulong)
            .wrapping_add(*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong)
            as uLong as uLong;
        if adler >= BASE as ::core::ffi::c_ulong {
            adler = (adler as ::core::ffi::c_ulong).wrapping_sub(BASE as ::core::ffi::c_ulong)
                as uLong as uLong;
        }
        sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
        if sum2 >= BASE as ::core::ffi::c_ulong {
            sum2 = sum2.wrapping_sub(BASE as ::core::ffi::c_ulong);
        }
        return adler | (sum2 as uLong) << 16 as ::core::ffi::c_int;
    }
    if buf.is_null() {
        return 1 as uLong;
    }
    if len < 16 as z_size_t {
        loop {
            let fresh0 = len;
            len = len.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            let fresh1 = buf;
            buf = buf.offset(1);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*fresh1 as ::core::ffi::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
        }
        if adler >= BASE as ::core::ffi::c_ulong {
            adler = (adler as ::core::ffi::c_ulong).wrapping_sub(BASE as ::core::ffi::c_ulong)
                as uLong as uLong;
        }
        sum2 = sum2.wrapping_rem(BASE as ::core::ffi::c_ulong);
        return adler | (sum2 as uLong) << 16 as ::core::ffi::c_int;
    }
    while len >= NMAX as z_size_t {
        len = (len as ::core::ffi::c_ulong).wrapping_sub(NMAX as ::core::ffi::c_ulong) as z_size_t
            as z_size_t;
        n = (NMAX / 16 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        loop {
            adler = (adler as ::core::ffi::c_ulong)
                .wrapping_add(*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong)
                .wrapping_add(*buf.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            buf = buf.offset(16 as ::core::ffi::c_int as isize);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        adler = (adler as ::core::ffi::c_ulong).wrapping_rem(BASE as ::core::ffi::c_ulong) as uLong
            as uLong;
        sum2 = sum2.wrapping_rem(BASE as ::core::ffi::c_ulong);
    }
    if len != 0 {
        while len >= 16 as z_size_t {
            len = (len as ::core::ffi::c_ulong).wrapping_sub(16 as ::core::ffi::c_ulong) as z_size_t
                as z_size_t;
            adler = (adler as ::core::ffi::c_ulong)
                .wrapping_add(*buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (0 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong)
                .wrapping_add(*buf.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(
                *buf.offset((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*buf.offset(
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            )
                as ::core::ffi::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
            buf = buf.offset(16 as ::core::ffi::c_int as isize);
        }
        loop {
            let fresh2 = len;
            len = len.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            let fresh3 = buf;
            buf = buf.offset(1);
            adler = (adler as ::core::ffi::c_ulong).wrapping_add(*fresh3 as ::core::ffi::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as ::core::ffi::c_ulong);
        }
        adler = (adler as ::core::ffi::c_ulong).wrapping_rem(BASE as ::core::ffi::c_ulong) as uLong
            as uLong;
        sum2 = sum2.wrapping_rem(BASE as ::core::ffi::c_ulong);
    }
    return adler | (sum2 as uLong) << 16 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn adler32(mut adler: uLong, mut buf: *const Bytef, mut len: uInt) -> uLong {
    return adler32_z(adler, buf, len as z_size_t);
}
unsafe extern "C" fn adler32_combine_(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: off_t,
) -> uLong {
    let mut sum1: ::core::ffi::c_ulong = 0;
    let mut sum2: ::core::ffi::c_ulong = 0;
    let mut rem: ::core::ffi::c_uint = 0;
    if len2 < 0 as ::core::ffi::c_long {
        return 0xffffffff as uLong;
    }
    len2 %= BASE as ::core::ffi::c_long;
    rem = len2 as ::core::ffi::c_uint;
    sum1 = adler1 as ::core::ffi::c_ulong & 0xffff as ::core::ffi::c_ulong;
    sum2 = (rem as ::core::ffi::c_ulong).wrapping_mul(sum1);
    sum2 = sum2.wrapping_rem(BASE as ::core::ffi::c_ulong);
    sum1 = sum1.wrapping_add(
        (adler2 as ::core::ffi::c_ulong & 0xffff as ::core::ffi::c_ulong)
            .wrapping_add(BASE as ::core::ffi::c_ulong)
            .wrapping_sub(1 as ::core::ffi::c_ulong),
    );
    sum2 = sum2.wrapping_add(
        (adler1 as ::core::ffi::c_ulong >> 16 as ::core::ffi::c_int
            & 0xffff as ::core::ffi::c_ulong)
            .wrapping_add(
                adler2 as ::core::ffi::c_ulong >> 16 as ::core::ffi::c_int
                    & 0xffff as ::core::ffi::c_ulong,
            )
            .wrapping_add(BASE as ::core::ffi::c_ulong)
            .wrapping_sub(rem as ::core::ffi::c_ulong),
    );
    if sum1 >= BASE as ::core::ffi::c_ulong {
        sum1 = sum1.wrapping_sub(BASE as ::core::ffi::c_ulong);
    }
    if sum1 >= BASE as ::core::ffi::c_ulong {
        sum1 = sum1.wrapping_sub(BASE as ::core::ffi::c_ulong);
    }
    if sum2 >= (BASE as ::core::ffi::c_ulong) << 1 as ::core::ffi::c_int {
        sum2 = sum2.wrapping_sub((BASE as ::core::ffi::c_ulong) << 1 as ::core::ffi::c_int);
    }
    if sum2 >= BASE as ::core::ffi::c_ulong {
        sum2 = sum2.wrapping_sub(BASE as ::core::ffi::c_ulong);
    }
    return sum1 as uLong | (sum2 as uLong) << 16 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn adler32_combine(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: off_t,
) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
#[no_mangle]
pub unsafe extern "C" fn adler32_combine64(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: off_t,
) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
