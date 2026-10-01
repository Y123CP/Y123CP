use core::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;

pub const BASE: c_uint = 65521 as c_uint;
pub const NMAX: c_int = 5552 as c_int;
#[no_mangle]
pub unsafe extern "C" fn adler32_z(
    mut adler: uLong,
    mut buf: *const Bytef,
    mut len: z_size_t,
) -> uLong {
    let mut sum2: c_ulong = 0;
    let mut n: c_uint = 0;
    sum2 =
        adler as c_ulong >> 16 as c_int & 0xffff as c_ulong;
    adler &= 0xffff as c_ulong;
    if len == 1 as z_size_t {
        adler = (adler as c_ulong)
            .wrapping_add(*buf.offset(0 as c_int as isize) as c_ulong)
            as uLong as uLong;
        if adler >= BASE as c_ulong {
            adler = (adler as c_ulong).wrapping_sub(BASE as c_ulong)
                as uLong as uLong;
        }
        sum2 = sum2.wrapping_add(adler as c_ulong);
        if sum2 >= BASE as c_ulong {
            sum2 = sum2.wrapping_sub(BASE as c_ulong);
        }
        return adler | (sum2 as uLong) << 16 as c_int;
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
            adler = (adler as c_ulong).wrapping_add(*fresh1 as c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
        }
        if adler >= BASE as c_ulong {
            adler = (adler as c_ulong).wrapping_sub(BASE as c_ulong)
                as uLong as uLong;
        }
        sum2 = sum2.wrapping_rem(BASE as c_ulong);
        return adler | (sum2 as uLong) << 16 as c_int;
    }
    while len >= NMAX as z_size_t {
        len = (len as c_ulong).wrapping_sub(NMAX as c_ulong) as z_size_t
            as z_size_t;
        n = (NMAX / 16 as c_int) as c_uint;
        loop {
            adler = (adler as c_ulong)
                .wrapping_add(*buf.offset(0 as c_int as isize) as c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((0 as c_int + 1 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((0 as c_int + 2 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (0 as c_int + 2 as c_int + 1 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((0 as c_int + 4 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (0 as c_int + 4 as c_int + 1 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (0 as c_int + 4 as c_int + 2 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (0 as c_int
                    + 4 as c_int
                    + 2 as c_int
                    + 1 as c_int) as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong)
                .wrapping_add(*buf.offset(8 as c_int as isize) as c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((8 as c_int + 1 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((8 as c_int + 2 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (8 as c_int + 2 as c_int + 1 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((8 as c_int + 4 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (8 as c_int + 4 as c_int + 1 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (8 as c_int + 4 as c_int + 2 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (8 as c_int
                    + 4 as c_int
                    + 2 as c_int
                    + 1 as c_int) as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            buf = buf.offset(16 as c_int as isize);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        adler = (adler as c_ulong).wrapping_rem(BASE as c_ulong) as uLong
            as uLong;
        sum2 = sum2.wrapping_rem(BASE as c_ulong);
    }
    if len != 0 {
        while len >= 16 as z_size_t {
            len = (len as c_ulong).wrapping_sub(16 as c_ulong) as z_size_t
                as z_size_t;
            adler = (adler as c_ulong)
                .wrapping_add(*buf.offset(0 as c_int as isize) as c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((0 as c_int + 1 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((0 as c_int + 2 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (0 as c_int + 2 as c_int + 1 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((0 as c_int + 4 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (0 as c_int + 4 as c_int + 1 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (0 as c_int + 4 as c_int + 2 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (0 as c_int
                    + 4 as c_int
                    + 2 as c_int
                    + 1 as c_int) as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong)
                .wrapping_add(*buf.offset(8 as c_int as isize) as c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((8 as c_int + 1 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((8 as c_int + 2 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (8 as c_int + 2 as c_int + 1 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(
                *buf.offset((8 as c_int + 4 as c_int) as isize)
                    as c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (8 as c_int + 4 as c_int + 1 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (8 as c_int + 4 as c_int + 2 as c_int)
                    as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            adler = (adler as c_ulong).wrapping_add(*buf.offset(
                (8 as c_int
                    + 4 as c_int
                    + 2 as c_int
                    + 1 as c_int) as isize,
            )
                as c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
            buf = buf.offset(16 as c_int as isize);
        }
        loop {
            let fresh2 = len;
            len = len.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            let fresh3 = buf;
            buf = buf.offset(1);
            adler = (adler as c_ulong).wrapping_add(*fresh3 as c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler as c_ulong);
        }
        adler = (adler as c_ulong).wrapping_rem(BASE as c_ulong) as uLong
            as uLong;
        sum2 = sum2.wrapping_rem(BASE as c_ulong);
    }
    return adler | (sum2 as uLong) << 16 as c_int;
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
    let mut sum1: c_ulong = 0;
    let mut sum2: c_ulong = 0;
    let mut rem: c_uint = 0;
    if len2 < 0 as c_long {
        return 0xffffffff as uLong;
    }
    len2 %= BASE as c_long;
    rem = len2 as c_uint;
    sum1 = adler1 as c_ulong & 0xffff as c_ulong;
    sum2 = (rem as c_ulong).wrapping_mul(sum1);
    sum2 = sum2.wrapping_rem(BASE as c_ulong);
    sum1 = sum1.wrapping_add(
        (adler2 as c_ulong & 0xffff as c_ulong)
            .wrapping_add(BASE as c_ulong)
            .wrapping_sub(1 as c_ulong),
    );
    sum2 = sum2.wrapping_add(
        (adler1 as c_ulong >> 16 as c_int
            & 0xffff as c_ulong)
            .wrapping_add(
                adler2 as c_ulong >> 16 as c_int
                    & 0xffff as c_ulong,
            )
            .wrapping_add(BASE as c_ulong)
            .wrapping_sub(rem as c_ulong),
    );
    if sum1 >= BASE as c_ulong {
        sum1 = sum1.wrapping_sub(BASE as c_ulong);
    }
    if sum1 >= BASE as c_ulong {
        sum1 = sum1.wrapping_sub(BASE as c_ulong);
    }
    if sum2 >= (BASE as c_ulong) << 1 as c_int {
        sum2 = sum2.wrapping_sub((BASE as c_ulong) << 1 as c_int);
    }
    if sum2 >= BASE as c_ulong {
        sum2 = sum2.wrapping_sub(BASE as c_ulong);
    }
    return sum1 as uLong | (sum2 as uLong) << 16 as c_int;
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
