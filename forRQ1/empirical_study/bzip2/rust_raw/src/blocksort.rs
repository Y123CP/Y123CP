extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn BZ2_bz__AssertH__fail(errcode: ::core::ffi::c_int);
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut ::core::ffi::c_char,
    pub avail_in: ::core::ffi::c_uint,
    pub total_in_lo32: ::core::ffi::c_uint,
    pub total_in_hi32: ::core::ffi::c_uint,
    pub next_out: *mut ::core::ffi::c_char,
    pub avail_out: ::core::ffi::c_uint,
    pub total_out_lo32: ::core::ffi::c_uint,
    pub total_out_hi32: ::core::ffi::c_uint,
    pub state: *mut ::core::ffi::c_void,
    pub bzalloc: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub bzfree:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type Bool = ::core::ffi::c_uchar;
pub type UChar = ::core::ffi::c_uchar;
pub type Int32 = ::core::ffi::c_int;
pub type UInt32 = ::core::ffi::c_uint;
pub type UInt16 = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub strm: *mut bz_stream,
    pub mode: Int32,
    pub state: Int32,
    pub avail_in_expect: UInt32,
    pub arr1: *mut UInt32,
    pub arr2: *mut UInt32,
    pub ftab: *mut UInt32,
    pub origPtr: Int32,
    pub ptr: *mut UInt32,
    pub block: *mut UChar,
    pub mtfv: *mut UInt16,
    pub zbits: *mut UChar,
    pub workFactor: Int32,
    pub state_in_ch: UInt32,
    pub state_in_len: Int32,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub nblock: Int32,
    pub nblockMAX: Int32,
    pub numZ: Int32,
    pub state_out_pos: Int32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub unseqToSeq: [UChar; 256],
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockCRC: UInt32,
    pub combinedCRC: UInt32,
    pub verbosity: Int32,
    pub blockNo: Int32,
    pub blockSize100k: Int32,
    pub nMTF: Int32,
    pub mtfFreq: [Int32; 258],
    pub selector: [UChar; 18002],
    pub selectorMtf: [UChar; 18002],
    pub len: [[UChar; 258]; 6],
    pub code: [[Int32; 258]; 6],
    pub rfreq: [[Int32; 258]; 6],
    pub len_pack: [[UInt32; 4]; 258],
}
pub const True: Bool = 1 as ::core::ffi::c_int as Bool;
pub const False: Bool = 0 as ::core::ffi::c_int as Bool;
pub const BZ_N_RADIX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BZ_N_QSORT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const BZ_N_SHELL: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const BZ_N_OVERSHOOT: ::core::ffi::c_int =
    BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn fallbackSimpleSort(
    mut fmap: *mut UInt32,
    mut eclass: *mut UInt32,
    mut lo: Int32,
    mut hi: Int32,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: Int32 = 0;
    let mut ec_tmp: UInt32 = 0;
    if lo == hi {
        return;
    }
    if hi - lo > 3 as ::core::ffi::c_int {
        i = (hi as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as Int32;
        while i >= lo {
            tmp = *fmap.offset(i as isize) as Int32;
            ec_tmp = *eclass.offset(tmp as isize);
            j = (i as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as Int32;
            while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                *fmap.offset((j as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize) =
                    *fmap.offset(j as isize);
                j += 4 as ::core::ffi::c_int;
            }
            *fmap.offset((j as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize) =
                tmp as UInt32;
            i -= 1;
        }
    }
    i = (hi as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as Int32;
    while i >= lo {
        tmp = *fmap.offset(i as isize) as Int32;
        ec_tmp = *eclass.offset(tmp as isize);
        j = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as Int32;
        while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
            *fmap.offset((j as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                *fmap.offset(j as isize);
            j += 1;
        }
        *fmap.offset((j as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) = tmp as UInt32;
        i -= 1;
    }
}
pub const FALLBACK_QSORT_SMALL_THRESH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn fallbackQSort3(
    mut fmap: *mut UInt32,
    mut eclass: *mut UInt32,
    mut loSt: Int32,
    mut hiSt: Int32,
) {
    let mut unLo: Int32 = 0;
    let mut unHi: Int32 = 0;
    let mut ltLo: Int32 = 0;
    let mut gtHi: Int32 = 0;
    let mut n: Int32 = 0;
    let mut m: Int32 = 0;
    let mut sp: Int32 = 0;
    let mut lo: Int32 = 0;
    let mut hi: Int32 = 0;
    let mut med: UInt32 = 0;
    let mut r: UInt32 = 0;
    let mut r3: UInt32 = 0;
    let mut stackLo: [Int32; 100] = [0; 100];
    let mut stackHi: [Int32; 100] = [0; 100];
    r = 0 as UInt32;
    sp = 0 as ::core::ffi::c_int as Int32;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    sp += 1;
    while sp > 0 as ::core::ffi::c_int {
        if !(sp < 100 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) {
            BZ2_bz__AssertH__fail(1004 as ::core::ffi::c_int);
        }
        sp -= 1;
        lo = stackLo[sp as usize];
        hi = stackHi[sp as usize];
        if hi - lo < FALLBACK_QSORT_SMALL_THRESH {
            fallbackSimpleSort(fmap, eclass, lo, hi);
        } else {
            r = (r as ::core::ffi::c_uint)
                .wrapping_mul(7621 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_rem(32768 as ::core::ffi::c_uint) as UInt32;
            r3 = (r as ::core::ffi::c_uint).wrapping_rem(3 as ::core::ffi::c_uint) as UInt32;
            if r3 == 0 as ::core::ffi::c_uint {
                med = *eclass.offset(*fmap.offset(lo as isize) as isize);
            } else if r3 == 1 as ::core::ffi::c_uint {
                med = *eclass
                    .offset(*fmap.offset((lo + hi >> 1 as ::core::ffi::c_int) as isize) as isize);
            } else {
                med = *eclass.offset(*fmap.offset(hi as isize) as isize);
            }
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            loop {
                while !(unLo > unHi) {
                    n = *eclass.offset(*fmap.offset(unLo as isize) as isize) as Int32
                        - med as Int32;
                    if n == 0 as ::core::ffi::c_int {
                        let mut zztmp: Int32 = *fmap.offset(unLo as isize) as Int32;
                        *fmap.offset(unLo as isize) = *fmap.offset(ltLo as isize);
                        *fmap.offset(ltLo as isize) = zztmp as UInt32;
                        ltLo += 1;
                        unLo += 1;
                    } else {
                        if n > 0 as ::core::ffi::c_int {
                            break;
                        }
                        unLo += 1;
                    }
                }
                while !(unLo > unHi) {
                    n = *eclass.offset(*fmap.offset(unHi as isize) as isize) as Int32
                        - med as Int32;
                    if n == 0 as ::core::ffi::c_int {
                        let mut zztmp_0: Int32 = *fmap.offset(unHi as isize) as Int32;
                        *fmap.offset(unHi as isize) = *fmap.offset(gtHi as isize);
                        *fmap.offset(gtHi as isize) = zztmp_0 as UInt32;
                        gtHi -= 1;
                        unHi -= 1;
                    } else {
                        if n < 0 as ::core::ffi::c_int {
                            break;
                        }
                        unHi -= 1;
                    }
                }
                if unLo > unHi {
                    break;
                }
                let mut zztmp_1: Int32 = *fmap.offset(unLo as isize) as Int32;
                *fmap.offset(unLo as isize) = *fmap.offset(unHi as isize);
                *fmap.offset(unHi as isize) = zztmp_1 as UInt32;
                unLo += 1;
                unHi -= 1;
            }
            if gtHi < ltLo {
                continue;
            }
            n = (if ltLo - lo < unLo - ltLo {
                ltLo as ::core::ffi::c_int - lo as ::core::ffi::c_int
            } else {
                unLo as ::core::ffi::c_int - ltLo as ::core::ffi::c_int
            }) as Int32;
            let mut yyp1: Int32 = lo;
            let mut yyp2: Int32 = unLo - n;
            let mut yyn: Int32 = n;
            while yyn > 0 as ::core::ffi::c_int {
                let mut zztmp_2: Int32 = *fmap.offset(yyp1 as isize) as Int32;
                *fmap.offset(yyp1 as isize) = *fmap.offset(yyp2 as isize);
                *fmap.offset(yyp2 as isize) = zztmp_2 as UInt32;
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1;
            }
            m = (if hi - gtHi < gtHi - unHi {
                hi as ::core::ffi::c_int - gtHi as ::core::ffi::c_int
            } else {
                gtHi as ::core::ffi::c_int - unHi as ::core::ffi::c_int
            }) as Int32;
            let mut yyp1_0: Int32 = unLo;
            let mut yyp2_0: Int32 = hi - m + 1 as Int32;
            let mut yyn_0: Int32 = m;
            while yyn_0 > 0 as ::core::ffi::c_int {
                let mut zztmp_3: Int32 = *fmap.offset(yyp1_0 as isize) as Int32;
                *fmap.offset(yyp1_0 as isize) = *fmap.offset(yyp2_0 as isize);
                *fmap.offset(yyp2_0 as isize) = zztmp_3 as UInt32;
                yyp1_0 += 1;
                yyp2_0 += 1;
                yyn_0 -= 1;
            }
            n = (lo as ::core::ffi::c_int + unLo as ::core::ffi::c_int
                - ltLo as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as Int32;
            m = (hi as ::core::ffi::c_int
                - (gtHi as ::core::ffi::c_int - unHi as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as Int32;
            if n - lo > hi - m {
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = n;
                sp += 1;
                stackLo[sp as usize] = m;
                stackHi[sp as usize] = hi;
                sp += 1;
            } else {
                stackLo[sp as usize] = m;
                stackHi[sp as usize] = hi;
                sp += 1;
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = n;
                sp += 1;
            }
        }
    }
}
unsafe extern "C" fn fallbackSort(
    mut fmap: *mut UInt32,
    mut eclass: *mut UInt32,
    mut bhtab: *mut UInt32,
    mut nblock: Int32,
    mut verb: Int32,
) {
    let mut ftab: [Int32; 257] = [0; 257];
    let mut ftabCopy: [Int32; 256] = [0; 256];
    let mut H: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut l: Int32 = 0;
    let mut r: Int32 = 0;
    let mut cc: Int32 = 0;
    let mut cc1: Int32 = 0;
    let mut nNotDone: Int32 = 0;
    let mut nBhtab: Int32 = 0;
    let mut eclass8: *mut UChar = eclass as *mut UChar;
    if verb >= 4 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"        bucket sorting ...\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i < 257 as ::core::ffi::c_int {
        ftab[i as usize] = 0 as ::core::ffi::c_int as Int32;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i < nblock {
        ftab[*eclass8.offset(i as isize) as usize] += 1;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i < 256 as ::core::ffi::c_int {
        ftabCopy[i as usize] = ftab[i as usize];
        i += 1;
    }
    i = 1 as ::core::ffi::c_int as Int32;
    while i < 257 as ::core::ffi::c_int {
        ftab[i as usize] += ftab[(i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i < nblock {
        j = *eclass8.offset(i as isize) as Int32;
        k = (ftab[j as usize] - 1 as ::core::ffi::c_int) as Int32;
        ftab[j as usize] = k;
        *fmap.offset(k as isize) = i as UInt32;
        i += 1;
    }
    nBhtab = (2 as ::core::ffi::c_int + nblock as ::core::ffi::c_int / 32 as ::core::ffi::c_int)
        as Int32;
    i = 0 as ::core::ffi::c_int as Int32;
    while i < nBhtab {
        *bhtab.offset(i as isize) = 0 as UInt32;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i < 256 as ::core::ffi::c_int {
        let ref mut fresh0 = *bhtab.offset((ftab[i as usize] >> 5 as ::core::ffi::c_int) as isize);
        *fresh0 |= ((1 as ::core::ffi::c_int as UInt32)
            << (ftab[i as usize] & 31 as ::core::ffi::c_int))
            as ::core::ffi::c_uint;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i < 32 as ::core::ffi::c_int {
        let ref mut fresh1 =
            *bhtab.offset((nblock + 2 as Int32 * i >> 5 as ::core::ffi::c_int) as isize);
        *fresh1 |= ((1 as ::core::ffi::c_int as UInt32)
            << (nblock as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i as ::core::ffi::c_int
                & 31 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
        let ref mut fresh2 = *bhtab.offset(
            (nblock as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * i as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int) as isize,
        );
        *fresh2 &= !((1 as ::core::ffi::c_int as UInt32)
            << (nblock as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * i as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                & 31 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
        i += 1;
    }
    H = 1 as ::core::ffi::c_int as Int32;
    loop {
        if verb >= 4 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"        depth %6d has \0" as *const u8 as *const ::core::ffi::c_char,
                H,
            );
        }
        j = 0 as ::core::ffi::c_int as Int32;
        i = 0 as ::core::ffi::c_int as Int32;
        while i < nblock {
            if *bhtab.offset((i >> 5 as ::core::ffi::c_int) as isize)
                & (1 as ::core::ffi::c_int as UInt32)
                    << (i as ::core::ffi::c_int & 31 as ::core::ffi::c_int)
                != 0
            {
                j = i;
            }
            k = (*fmap.offset(i as isize) as ::core::ffi::c_uint)
                .wrapping_sub(H as ::core::ffi::c_uint) as Int32;
            if k < 0 as ::core::ffi::c_int {
                k += nblock as ::core::ffi::c_int;
            }
            *eclass.offset(k as isize) = j as UInt32;
            i += 1;
        }
        nNotDone = 0 as ::core::ffi::c_int as Int32;
        r = -(1 as ::core::ffi::c_int) as Int32;
        loop {
            k = (r as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as Int32;
            while *bhtab.offset((k >> 5 as ::core::ffi::c_int) as isize)
                & (1 as ::core::ffi::c_int as UInt32)
                    << (k as ::core::ffi::c_int & 31 as ::core::ffi::c_int)
                != 0
                && k as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int != 0
            {
                k += 1;
            }
            if *bhtab.offset((k >> 5 as ::core::ffi::c_int) as isize)
                & (1 as ::core::ffi::c_int as UInt32)
                    << (k as ::core::ffi::c_int & 31 as ::core::ffi::c_int)
                != 0
            {
                while *bhtab.offset((k >> 5 as ::core::ffi::c_int) as isize)
                    == 0xffffffff as ::core::ffi::c_uint
                {
                    k += 32 as ::core::ffi::c_int;
                }
                while *bhtab.offset((k >> 5 as ::core::ffi::c_int) as isize)
                    & (1 as ::core::ffi::c_int as UInt32)
                        << (k as ::core::ffi::c_int & 31 as ::core::ffi::c_int)
                    != 0
                {
                    k += 1;
                }
            }
            l = (k as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as Int32;
            if l >= nblock {
                break;
            }
            while *bhtab.offset((k >> 5 as ::core::ffi::c_int) as isize)
                & (1 as ::core::ffi::c_int as UInt32)
                    << (k as ::core::ffi::c_int & 31 as ::core::ffi::c_int)
                == 0
                && k as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int != 0
            {
                k += 1;
            }
            if *bhtab.offset((k >> 5 as ::core::ffi::c_int) as isize)
                & (1 as ::core::ffi::c_int as UInt32)
                    << (k as ::core::ffi::c_int & 31 as ::core::ffi::c_int)
                == 0
            {
                while *bhtab.offset((k >> 5 as ::core::ffi::c_int) as isize)
                    == 0 as ::core::ffi::c_uint
                {
                    k += 32 as ::core::ffi::c_int;
                }
                while *bhtab.offset((k >> 5 as ::core::ffi::c_int) as isize)
                    & (1 as ::core::ffi::c_int as UInt32)
                        << (k as ::core::ffi::c_int & 31 as ::core::ffi::c_int)
                    == 0
                {
                    k += 1;
                }
            }
            r = (k as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as Int32;
            if r >= nblock {
                break;
            }
            if r > l {
                nNotDone +=
                    r as ::core::ffi::c_int - l as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
                fallbackQSort3(fmap, eclass, l, r);
                cc = -(1 as ::core::ffi::c_int) as Int32;
                i = l;
                while i <= r {
                    cc1 = *eclass.offset(*fmap.offset(i as isize) as isize) as Int32;
                    if cc != cc1 {
                        let ref mut fresh3 = *bhtab.offset((i >> 5 as ::core::ffi::c_int) as isize);
                        *fresh3 |= ((1 as ::core::ffi::c_int as UInt32)
                            << (i as ::core::ffi::c_int & 31 as ::core::ffi::c_int))
                            as ::core::ffi::c_uint;
                        cc = cc1;
                    }
                    i += 1;
                }
            }
        }
        if verb >= 4 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"%6d unresolved strings\n\0" as *const u8 as *const ::core::ffi::c_char,
                nNotDone,
            );
        }
        H *= 2 as ::core::ffi::c_int;
        if H > nblock || nNotDone == 0 as ::core::ffi::c_int {
            break;
        }
    }
    if verb >= 4 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"        reconstructing block ...\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    j = 0 as ::core::ffi::c_int as Int32;
    i = 0 as ::core::ffi::c_int as Int32;
    while i < nblock {
        while ftabCopy[j as usize] == 0 as ::core::ffi::c_int {
            j += 1;
        }
        ftabCopy[j as usize] -= 1;
        *eclass8.offset(*fmap.offset(i as isize) as isize) = j as UChar;
        i += 1;
    }
    if !(j < 256 as ::core::ffi::c_int) {
        BZ2_bz__AssertH__fail(1005 as ::core::ffi::c_int);
    }
}
#[inline]
unsafe extern "C" fn mainGtU(
    mut i1: UInt32,
    mut i2: UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut nblock: UInt32,
    mut budget: *mut Int32,
) -> Bool {
    let mut k: Int32 = 0;
    let mut c1: UChar = 0;
    let mut c2: UChar = 0;
    let mut s1: UInt16 = 0;
    let mut s2: UInt16 = 0;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
        return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    k = (nblock as ::core::ffi::c_uint).wrapping_add(8 as ::core::ffi::c_uint) as Int32;
    loop {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as ::core::ffi::c_int != s2 as ::core::ffi::c_int {
            return (s1 as ::core::ffi::c_int > s2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as ::core::ffi::c_int != s2 as ::core::ffi::c_int {
            return (s1 as ::core::ffi::c_int > s2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as ::core::ffi::c_int != s2 as ::core::ffi::c_int {
            return (s1 as ::core::ffi::c_int > s2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as ::core::ffi::c_int != s2 as ::core::ffi::c_int {
            return (s1 as ::core::ffi::c_int > s2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as ::core::ffi::c_int != s2 as ::core::ffi::c_int {
            return (s1 as ::core::ffi::c_int > s2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as ::core::ffi::c_int != s2 as ::core::ffi::c_int {
            return (s1 as ::core::ffi::c_int > s2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as ::core::ffi::c_int != s2 as ::core::ffi::c_int {
            return (s1 as ::core::ffi::c_int > s2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return (c1 as ::core::ffi::c_int > c2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as ::core::ffi::c_int != s2 as ::core::ffi::c_int {
            return (s1 as ::core::ffi::c_int > s2 as ::core::ffi::c_int) as ::core::ffi::c_int
                as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        if i1 >= nblock {
            i1 = (i1 as ::core::ffi::c_uint).wrapping_sub(nblock as ::core::ffi::c_uint) as UInt32
                as UInt32;
        }
        if i2 >= nblock {
            i2 = (i2 as ::core::ffi::c_uint).wrapping_sub(nblock as ::core::ffi::c_uint) as UInt32
                as UInt32;
        }
        k -= 8 as ::core::ffi::c_int;
        *budget -= 1;
        if !(k >= 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return False;
}
static mut incs: [Int32; 14] = [
    1 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    13 as ::core::ffi::c_int,
    40 as ::core::ffi::c_int,
    121 as ::core::ffi::c_int,
    364 as ::core::ffi::c_int,
    1093 as ::core::ffi::c_int,
    3280 as ::core::ffi::c_int,
    9841 as ::core::ffi::c_int,
    29524 as ::core::ffi::c_int,
    88573 as ::core::ffi::c_int,
    265720 as ::core::ffi::c_int,
    797161 as ::core::ffi::c_int,
    2391484 as ::core::ffi::c_int,
];
unsafe extern "C" fn mainSimpleSort(
    mut ptr: *mut UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut nblock: Int32,
    mut lo: Int32,
    mut hi: Int32,
    mut d: Int32,
    mut budget: *mut Int32,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut h: Int32 = 0;
    let mut bigN: Int32 = 0;
    let mut hp: Int32 = 0;
    let mut v: UInt32 = 0;
    bigN = (hi as ::core::ffi::c_int - lo as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as Int32;
    if bigN < 2 as ::core::ffi::c_int {
        return;
    }
    hp = 0 as ::core::ffi::c_int as Int32;
    while incs[hp as usize] < bigN {
        hp += 1;
    }
    hp -= 1;
    while hp >= 0 as ::core::ffi::c_int {
        h = incs[hp as usize];
        i = lo + h;
        while True != 0 {
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as UInt32),
                v.wrapping_add(d as UInt32),
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo as ::core::ffi::c_int + h as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as UInt32),
                v.wrapping_add(d as UInt32),
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo as ::core::ffi::c_int + h as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as UInt32),
                v.wrapping_add(d as UInt32),
                block,
                quadrant,
                nblock as UInt32,
                budget,
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo as ::core::ffi::c_int + h as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            if *budget < 0 as ::core::ffi::c_int {
                return;
            }
        }
        hp -= 1;
    }
}
#[inline]
unsafe extern "C" fn mmed3(mut a: UChar, mut b: UChar, mut c: UChar) -> UChar {
    let mut t: UChar = 0;
    if a as ::core::ffi::c_int > b as ::core::ffi::c_int {
        t = a;
        a = b;
        b = t;
    }
    if b as ::core::ffi::c_int > c as ::core::ffi::c_int {
        b = c;
        if a as ::core::ffi::c_int > b as ::core::ffi::c_int {
            b = a;
        }
    }
    return b;
}
pub const MAIN_QSORT_SMALL_THRESH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAIN_QSORT_DEPTH_THRESH: ::core::ffi::c_int = BZ_N_RADIX + BZ_N_QSORT;
unsafe extern "C" fn mainQSort3(
    mut ptr: *mut UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut nblock: Int32,
    mut loSt: Int32,
    mut hiSt: Int32,
    mut dSt: Int32,
    mut budget: *mut Int32,
) {
    let mut unLo: Int32 = 0;
    let mut unHi: Int32 = 0;
    let mut ltLo: Int32 = 0;
    let mut gtHi: Int32 = 0;
    let mut n: Int32 = 0;
    let mut m: Int32 = 0;
    let mut med: Int32 = 0;
    let mut sp: Int32 = 0;
    let mut lo: Int32 = 0;
    let mut hi: Int32 = 0;
    let mut d: Int32 = 0;
    let mut stackLo: [Int32; 100] = [0; 100];
    let mut stackHi: [Int32; 100] = [0; 100];
    let mut stackD: [Int32; 100] = [0; 100];
    let mut nextLo: [Int32; 3] = [0; 3];
    let mut nextHi: [Int32; 3] = [0; 3];
    let mut nextD: [Int32; 3] = [0; 3];
    sp = 0 as ::core::ffi::c_int as Int32;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    stackD[sp as usize] = dSt;
    sp += 1;
    while sp > 0 as ::core::ffi::c_int {
        if !(sp < 100 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) {
            BZ2_bz__AssertH__fail(1001 as ::core::ffi::c_int);
        }
        sp -= 1;
        lo = stackLo[sp as usize];
        hi = stackHi[sp as usize];
        d = stackD[sp as usize];
        if hi - lo < MAIN_QSORT_SMALL_THRESH || d > MAIN_QSORT_DEPTH_THRESH {
            mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 as ::core::ffi::c_int {
                return;
            }
        } else {
            med = mmed3(
                *block.offset(
                    (*ptr.offset(lo as isize) as ::core::ffi::c_uint)
                        .wrapping_add(d as ::core::ffi::c_uint) as isize,
                ),
                *block.offset(
                    (*ptr.offset(hi as isize) as ::core::ffi::c_uint)
                        .wrapping_add(d as ::core::ffi::c_uint) as isize,
                ),
                *block.offset(
                    (*ptr.offset((lo + hi >> 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_uint)
                        .wrapping_add(d as ::core::ffi::c_uint) as isize,
                ),
            ) as Int32;
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            while True != 0 {
                while True != 0 {
                    if unLo > unHi {
                        break;
                    }
                    n = *block.offset(
                        (*ptr.offset(unLo as isize) as ::core::ffi::c_uint)
                            .wrapping_add(d as ::core::ffi::c_uint)
                            as isize,
                    ) as Int32
                        - med;
                    if n == 0 as ::core::ffi::c_int {
                        let mut zztmp: Int32 = *ptr.offset(unLo as isize) as Int32;
                        *ptr.offset(unLo as isize) = *ptr.offset(ltLo as isize);
                        *ptr.offset(ltLo as isize) = zztmp as UInt32;
                        ltLo += 1;
                        unLo += 1;
                    } else {
                        if n > 0 as ::core::ffi::c_int {
                            break;
                        }
                        unLo += 1;
                    }
                }
                while True != 0 {
                    if unLo > unHi {
                        break;
                    }
                    n = *block.offset(
                        (*ptr.offset(unHi as isize) as ::core::ffi::c_uint)
                            .wrapping_add(d as ::core::ffi::c_uint)
                            as isize,
                    ) as Int32
                        - med;
                    if n == 0 as ::core::ffi::c_int {
                        let mut zztmp_0: Int32 = *ptr.offset(unHi as isize) as Int32;
                        *ptr.offset(unHi as isize) = *ptr.offset(gtHi as isize);
                        *ptr.offset(gtHi as isize) = zztmp_0 as UInt32;
                        gtHi -= 1;
                        unHi -= 1;
                    } else {
                        if n < 0 as ::core::ffi::c_int {
                            break;
                        }
                        unHi -= 1;
                    }
                }
                if unLo > unHi {
                    break;
                }
                let mut zztmp_1: Int32 = *ptr.offset(unLo as isize) as Int32;
                *ptr.offset(unLo as isize) = *ptr.offset(unHi as isize);
                *ptr.offset(unHi as isize) = zztmp_1 as UInt32;
                unLo += 1;
                unHi -= 1;
            }
            if gtHi < ltLo {
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = hi;
                stackD[sp as usize] = (d as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as Int32;
                sp += 1;
            } else {
                n = (if ltLo - lo < unLo - ltLo {
                    ltLo as ::core::ffi::c_int - lo as ::core::ffi::c_int
                } else {
                    unLo as ::core::ffi::c_int - ltLo as ::core::ffi::c_int
                }) as Int32;
                let mut yyp1: Int32 = lo;
                let mut yyp2: Int32 = unLo - n;
                let mut yyn: Int32 = n;
                while yyn > 0 as ::core::ffi::c_int {
                    let mut zztmp_2: Int32 = *ptr.offset(yyp1 as isize) as Int32;
                    *ptr.offset(yyp1 as isize) = *ptr.offset(yyp2 as isize);
                    *ptr.offset(yyp2 as isize) = zztmp_2 as UInt32;
                    yyp1 += 1;
                    yyp2 += 1;
                    yyn -= 1;
                }
                m = (if hi - gtHi < gtHi - unHi {
                    hi as ::core::ffi::c_int - gtHi as ::core::ffi::c_int
                } else {
                    gtHi as ::core::ffi::c_int - unHi as ::core::ffi::c_int
                }) as Int32;
                let mut yyp1_0: Int32 = unLo;
                let mut yyp2_0: Int32 = hi - m + 1 as Int32;
                let mut yyn_0: Int32 = m;
                while yyn_0 > 0 as ::core::ffi::c_int {
                    let mut zztmp_3: Int32 = *ptr.offset(yyp1_0 as isize) as Int32;
                    *ptr.offset(yyp1_0 as isize) = *ptr.offset(yyp2_0 as isize);
                    *ptr.offset(yyp2_0 as isize) = zztmp_3 as UInt32;
                    yyp1_0 += 1;
                    yyp2_0 += 1;
                    yyn_0 -= 1;
                }
                n = (lo as ::core::ffi::c_int + unLo as ::core::ffi::c_int
                    - ltLo as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as Int32;
                m = (hi as ::core::ffi::c_int
                    - (gtHi as ::core::ffi::c_int - unHi as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int) as Int32;
                nextLo[0 as ::core::ffi::c_int as usize] = lo;
                nextHi[0 as ::core::ffi::c_int as usize] = n;
                nextD[0 as ::core::ffi::c_int as usize] = d;
                nextLo[1 as ::core::ffi::c_int as usize] = m;
                nextHi[1 as ::core::ffi::c_int as usize] = hi;
                nextD[1 as ::core::ffi::c_int as usize] = d;
                nextLo[2 as ::core::ffi::c_int as usize] =
                    (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as Int32;
                nextHi[2 as ::core::ffi::c_int as usize] =
                    (m as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as Int32;
                nextD[2 as ::core::ffi::c_int as usize] =
                    (d as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as Int32;
                if nextHi[0 as ::core::ffi::c_int as usize]
                    - nextLo[0 as ::core::ffi::c_int as usize]
                    < nextHi[1 as ::core::ffi::c_int as usize]
                        - nextLo[1 as ::core::ffi::c_int as usize]
                {
                    let mut tz: Int32 = 0;
                    tz = nextLo[0 as ::core::ffi::c_int as usize];
                    nextLo[0 as ::core::ffi::c_int as usize] =
                        nextLo[1 as ::core::ffi::c_int as usize];
                    nextLo[1 as ::core::ffi::c_int as usize] = tz;
                    tz = nextHi[0 as ::core::ffi::c_int as usize];
                    nextHi[0 as ::core::ffi::c_int as usize] =
                        nextHi[1 as ::core::ffi::c_int as usize];
                    nextHi[1 as ::core::ffi::c_int as usize] = tz;
                    tz = nextD[0 as ::core::ffi::c_int as usize];
                    nextD[0 as ::core::ffi::c_int as usize] =
                        nextD[1 as ::core::ffi::c_int as usize];
                    nextD[1 as ::core::ffi::c_int as usize] = tz;
                }
                if nextHi[1 as ::core::ffi::c_int as usize]
                    - nextLo[1 as ::core::ffi::c_int as usize]
                    < nextHi[2 as ::core::ffi::c_int as usize]
                        - nextLo[2 as ::core::ffi::c_int as usize]
                {
                    let mut tz_0: Int32 = 0;
                    tz_0 = nextLo[1 as ::core::ffi::c_int as usize];
                    nextLo[1 as ::core::ffi::c_int as usize] =
                        nextLo[2 as ::core::ffi::c_int as usize];
                    nextLo[2 as ::core::ffi::c_int as usize] = tz_0;
                    tz_0 = nextHi[1 as ::core::ffi::c_int as usize];
                    nextHi[1 as ::core::ffi::c_int as usize] =
                        nextHi[2 as ::core::ffi::c_int as usize];
                    nextHi[2 as ::core::ffi::c_int as usize] = tz_0;
                    tz_0 = nextD[1 as ::core::ffi::c_int as usize];
                    nextD[1 as ::core::ffi::c_int as usize] =
                        nextD[2 as ::core::ffi::c_int as usize];
                    nextD[2 as ::core::ffi::c_int as usize] = tz_0;
                }
                if nextHi[0 as ::core::ffi::c_int as usize]
                    - nextLo[0 as ::core::ffi::c_int as usize]
                    < nextHi[1 as ::core::ffi::c_int as usize]
                        - nextLo[1 as ::core::ffi::c_int as usize]
                {
                    let mut tz_1: Int32 = 0;
                    tz_1 = nextLo[0 as ::core::ffi::c_int as usize];
                    nextLo[0 as ::core::ffi::c_int as usize] =
                        nextLo[1 as ::core::ffi::c_int as usize];
                    nextLo[1 as ::core::ffi::c_int as usize] = tz_1;
                    tz_1 = nextHi[0 as ::core::ffi::c_int as usize];
                    nextHi[0 as ::core::ffi::c_int as usize] =
                        nextHi[1 as ::core::ffi::c_int as usize];
                    nextHi[1 as ::core::ffi::c_int as usize] = tz_1;
                    tz_1 = nextD[0 as ::core::ffi::c_int as usize];
                    nextD[0 as ::core::ffi::c_int as usize] =
                        nextD[1 as ::core::ffi::c_int as usize];
                    nextD[1 as ::core::ffi::c_int as usize] = tz_1;
                }
                stackLo[sp as usize] = nextLo[0 as ::core::ffi::c_int as usize];
                stackHi[sp as usize] = nextHi[0 as ::core::ffi::c_int as usize];
                stackD[sp as usize] = nextD[0 as ::core::ffi::c_int as usize];
                sp += 1;
                stackLo[sp as usize] = nextLo[1 as ::core::ffi::c_int as usize];
                stackHi[sp as usize] = nextHi[1 as ::core::ffi::c_int as usize];
                stackD[sp as usize] = nextD[1 as ::core::ffi::c_int as usize];
                sp += 1;
                stackLo[sp as usize] = nextLo[2 as ::core::ffi::c_int as usize];
                stackHi[sp as usize] = nextHi[2 as ::core::ffi::c_int as usize];
                stackD[sp as usize] = nextD[2 as ::core::ffi::c_int as usize];
                sp += 1;
            }
        }
    }
}
pub const SETMASK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int;
pub const CLEARMASK: ::core::ffi::c_int = !((1 as ::core::ffi::c_int) << 21 as ::core::ffi::c_int);
unsafe extern "C" fn mainSort(
    mut ptr: *mut UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut ftab: *mut UInt32,
    mut nblock: Int32,
    mut verb: Int32,
    mut budget: *mut Int32,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut ss: Int32 = 0;
    let mut sb: Int32 = 0;
    let mut runningOrder: [Int32; 256] = [0; 256];
    let mut bigDone: [Bool; 256] = [0; 256];
    let mut copyStart: [Int32; 256] = [0; 256];
    let mut copyEnd: [Int32; 256] = [0; 256];
    let mut c1: UChar = 0;
    let mut numQSorted: Int32 = 0;
    let mut s: UInt16 = 0;
    if verb >= 4 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"        main sort initialise ...\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = 65536 as ::core::ffi::c_int as Int32;
    while i >= 0 as ::core::ffi::c_int {
        *ftab.offset(i as isize) = 0 as UInt32;
        i -= 1;
    }
    j = ((*block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int) as Int32;
    i = (nblock as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as Int32;
    while i >= 3 as ::core::ffi::c_int {
        *quadrant.offset(i as isize) = 0 as UInt16;
        j = (j as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset(i as isize) as UInt16 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as Int32;
        let ref mut fresh4 = *ftab.offset(j as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        *quadrant.offset((i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
            0 as UInt16;
        j = (j as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset((i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) as UInt16
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as Int32;
        let ref mut fresh5 = *ftab.offset(j as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        *quadrant.offset((i as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize) =
            0 as UInt16;
        j = (j as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset((i as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize) as UInt16
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as Int32;
        let ref mut fresh6 = *ftab.offset(j as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        *quadrant.offset((i as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize) =
            0 as UInt16;
        j = (j as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset((i as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize) as UInt16
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as Int32;
        let ref mut fresh7 = *ftab.offset(j as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
        i -= 4 as ::core::ffi::c_int;
    }
    while i >= 0 as ::core::ffi::c_int {
        *quadrant.offset(i as isize) = 0 as UInt16;
        j = (j as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset(i as isize) as UInt16 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as Int32;
        let ref mut fresh8 = *ftab.offset(j as isize);
        *fresh8 = (*fresh8).wrapping_add(1);
        i -= 1;
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i < BZ_N_OVERSHOOT {
        *block.offset((nblock + i) as isize) = *block.offset(i as isize);
        *quadrant.offset((nblock + i) as isize) = 0 as UInt16;
        i += 1;
    }
    if verb >= 4 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"        bucket sorting ...\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = 1 as ::core::ffi::c_int as Int32;
    while i <= 65536 as ::core::ffi::c_int {
        let ref mut fresh9 = *ftab.offset(i as isize);
        *fresh9 = (*fresh9 as ::core::ffi::c_uint).wrapping_add(
            *ftab.offset((i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint,
        ) as UInt32 as UInt32;
        i += 1;
    }
    s = ((*block.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int) as UInt16;
    i = (nblock as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as Int32;
    while i >= 3 as ::core::ffi::c_int {
        s = (s as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset(i as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as UInt16;
        j = (*ftab.offset(s as isize) as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        s = (s as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset((i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as UInt16;
        j = (*ftab.offset(s as isize) as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as UInt32;
        s = (s as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset((i as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as UInt16;
        j = (*ftab.offset(s as isize) as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as UInt32;
        s = (s as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset((i as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int) as UInt16;
        j = (*ftab.offset(s as isize) as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as UInt32;
        i -= 4 as ::core::ffi::c_int;
    }
    while i >= 0 as ::core::ffi::c_int {
        s = (s as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            | (*block.offset(i as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
            as UInt16;
        j = (*ftab.offset(s as isize) as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
            as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        i -= 1;
    }
    i = 0 as ::core::ffi::c_int as Int32;
    while i <= 255 as ::core::ffi::c_int {
        bigDone[i as usize] = False;
        runningOrder[i as usize] = i;
        i += 1;
    }
    let mut vv: Int32 = 0;
    let mut h: Int32 = 1 as Int32;
    loop {
        h = (3 as ::core::ffi::c_int * h as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as Int32;
        if !(h <= 256 as ::core::ffi::c_int) {
            break;
        }
    }
    loop {
        h = (h as ::core::ffi::c_int / 3 as ::core::ffi::c_int) as Int32;
        i = h;
        while i <= 255 as ::core::ffi::c_int {
            vv = runningOrder[i as usize];
            j = i;
            while (*ftab.offset(
                ((runningOrder[(j - h) as usize] + 1 as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int) as isize,
            ))
            .wrapping_sub(
                *ftab.offset((runningOrder[(j - h) as usize] << 8 as ::core::ffi::c_int) as isize),
            ) > (*ftab.offset(
                ((vv as ::core::ffi::c_int + 1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as isize,
            ))
            .wrapping_sub(*ftab.offset((vv << 8 as ::core::ffi::c_int) as isize))
            {
                runningOrder[j as usize] = runningOrder[(j - h) as usize];
                j = j - h;
                if j <= h as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
                    break;
                }
            }
            runningOrder[j as usize] = vv;
            i += 1;
        }
        if !(h != 1 as ::core::ffi::c_int) {
            break;
        }
    }
    numQSorted = 0 as ::core::ffi::c_int as Int32;
    i = 0 as ::core::ffi::c_int as Int32;
    while i <= 255 as ::core::ffi::c_int {
        ss = runningOrder[i as usize];
        j = 0 as ::core::ffi::c_int as Int32;
        while j <= 255 as ::core::ffi::c_int {
            if j != ss {
                sb = (ss << 8 as ::core::ffi::c_int) + j;
                if *ftab.offset(sb as isize) as ::core::ffi::c_uint & SETMASK as ::core::ffi::c_uint
                    == 0
                {
                    let mut lo: Int32 = (*ftab.offset(sb as isize) as ::core::ffi::c_uint
                        & CLEARMASK as ::core::ffi::c_uint)
                        as Int32;
                    let mut hi: Int32 = (*ftab
                        .offset((sb as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_uint
                        & CLEARMASK as ::core::ffi::c_uint)
                        .wrapping_sub(1 as ::core::ffi::c_uint)
                        as Int32;
                    if hi > lo {
                        if verb >= 4 as ::core::ffi::c_int {
                            fprintf(
                                stderr,
                                b"        qsort [0x%x, 0x%x]   done %d   this %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                ss,
                                j,
                                numQSorted,
                                hi as ::core::ffi::c_int - lo as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int,
                            );
                        }
                        mainQSort3(ptr, block, quadrant, nblock, lo, hi, BZ_N_RADIX, budget);
                        numQSorted += hi as ::core::ffi::c_int - lo as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int;
                        if *budget < 0 as ::core::ffi::c_int {
                            return;
                        }
                    }
                }
                let ref mut fresh10 = *ftab.offset(sb as isize);
                *fresh10 |= SETMASK as ::core::ffi::c_uint;
            }
            j += 1;
        }
        if bigDone[ss as usize] != 0 {
            BZ2_bz__AssertH__fail(1006 as ::core::ffi::c_int);
        }
        j = 0 as ::core::ffi::c_int as Int32;
        while j <= 255 as ::core::ffi::c_int {
            copyStart[j as usize] = (*ftab.offset(((j << 8 as ::core::ffi::c_int) + ss) as isize)
                as ::core::ffi::c_uint
                & CLEARMASK as ::core::ffi::c_uint) as Int32;
            copyEnd[j as usize] = (*ftab.offset(
                (((j as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    + ss as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_uint
                & CLEARMASK as ::core::ffi::c_uint)
                .wrapping_sub(1 as ::core::ffi::c_uint) as Int32;
            j += 1;
        }
        j = (*ftab.offset((ss << 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_uint
            & CLEARMASK as ::core::ffi::c_uint) as Int32;
        while j < copyStart[ss as usize] {
            k = (*ptr.offset(j as isize) as ::core::ffi::c_uint)
                .wrapping_sub(1 as ::core::ffi::c_uint) as Int32;
            if k < 0 as ::core::ffi::c_int {
                k += nblock as ::core::ffi::c_int;
            }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh11 = copyStart[c1 as usize];
                copyStart[c1 as usize] = copyStart[c1 as usize] + 1;
                *ptr.offset(fresh11 as isize) = k as UInt32;
            }
            j += 1;
        }
        j = (*ftab.offset(
            ((ss as ::core::ffi::c_int + 1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint
            & CLEARMASK as ::core::ffi::c_uint)
            .wrapping_sub(1 as ::core::ffi::c_uint) as Int32;
        while j > copyEnd[ss as usize] {
            k = (*ptr.offset(j as isize) as ::core::ffi::c_uint)
                .wrapping_sub(1 as ::core::ffi::c_uint) as Int32;
            if k < 0 as ::core::ffi::c_int {
                k += nblock as ::core::ffi::c_int;
            }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh12 = copyEnd[c1 as usize];
                copyEnd[c1 as usize] = copyEnd[c1 as usize] - 1;
                *ptr.offset(fresh12 as isize) = k as UInt32;
            }
            j -= 1;
        }
        if !(copyStart[ss as usize] - 1 as ::core::ffi::c_int == copyEnd[ss as usize]
            || copyStart[ss as usize] == 0 as ::core::ffi::c_int
                && copyEnd[ss as usize] == nblock as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        {
            BZ2_bz__AssertH__fail(1007 as ::core::ffi::c_int);
        }
        j = 0 as ::core::ffi::c_int as Int32;
        while j <= 255 as ::core::ffi::c_int {
            let ref mut fresh13 = *ftab.offset(((j << 8 as ::core::ffi::c_int) + ss) as isize);
            *fresh13 |= SETMASK as ::core::ffi::c_uint;
            j += 1;
        }
        bigDone[ss as usize] = True;
        if i < 255 as ::core::ffi::c_int {
            let mut bbStart: Int32 = (*ftab.offset((ss << 8 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_uint
                & CLEARMASK as ::core::ffi::c_uint) as Int32;
            let mut bbSize: Int32 = (*ftab.offset(
                ((ss as ::core::ffi::c_int + 1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_uint
                & CLEARMASK as ::core::ffi::c_uint)
                .wrapping_sub(bbStart as ::core::ffi::c_uint)
                as Int32;
            let mut shifts: Int32 = 0 as Int32;
            while bbSize >> shifts > 65534 as ::core::ffi::c_int {
                shifts += 1;
            }
            j = (bbSize as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as Int32;
            while j >= 0 as ::core::ffi::c_int {
                let mut a2update: Int32 = *ptr.offset((bbStart + j) as isize) as Int32;
                let mut qVal: UInt16 = (j >> shifts) as UInt16;
                *quadrant.offset(a2update as isize) = qVal;
                if a2update < BZ_N_OVERSHOOT {
                    *quadrant.offset((a2update + nblock) as isize) = qVal;
                }
                j -= 1;
            }
            if !(bbSize as ::core::ffi::c_int - 1 as ::core::ffi::c_int >> shifts
                <= 65535 as ::core::ffi::c_int)
            {
                BZ2_bz__AssertH__fail(1002 as ::core::ffi::c_int);
            }
        }
        i += 1;
    }
    if verb >= 4 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"        %d pointers, %d sorted, %d scanned\n\0" as *const u8
                as *const ::core::ffi::c_char,
            nblock,
            numQSorted,
            nblock - numQSorted,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_blockSort(mut s: *mut EState) {
    let mut ptr: *mut UInt32 = (*s).ptr;
    let mut block: *mut UChar = (*s).block;
    let mut ftab: *mut UInt32 = (*s).ftab;
    let mut nblock: Int32 = (*s).nblock;
    let mut verb: Int32 = (*s).verbosity;
    let mut wfact: Int32 = (*s).workFactor;
    let mut quadrant: *mut UInt16 = ::core::ptr::null_mut::<UInt16>();
    let mut budget: Int32 = 0;
    let mut budgetInit: Int32 = 0;
    let mut i: Int32 = 0;
    if nblock < 10000 as ::core::ffi::c_int {
        fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
    } else {
        i = (nblock as ::core::ffi::c_int + BZ_N_OVERSHOOT) as Int32;
        if i as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            i += 1;
        }
        quadrant = block.offset(i as isize) as *mut UChar as *mut UInt16;
        if wfact < 1 as ::core::ffi::c_int {
            wfact = 1 as ::core::ffi::c_int as Int32;
        }
        if wfact > 100 as ::core::ffi::c_int {
            wfact = 100 as ::core::ffi::c_int as Int32;
        }
        budgetInit = (nblock as ::core::ffi::c_int
            * ((wfact as ::core::ffi::c_int - 1 as ::core::ffi::c_int) / 3 as ::core::ffi::c_int))
            as Int32;
        budget = budgetInit;
        mainSort(ptr, block, quadrant, ftab, nblock, verb, &raw mut budget);
        if verb >= 3 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"      %d work, %d block, ratio %5.2f\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                budgetInit - budget,
                nblock,
                ((budgetInit - budget) as ::core::ffi::c_float
                    / (if nblock == 0 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int
                    } else {
                        nblock as ::core::ffi::c_int
                    }) as ::core::ffi::c_float) as ::core::ffi::c_double,
            );
        }
        if budget < 0 as ::core::ffi::c_int {
            if verb >= 2 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"    too repetitive; using fallback sorting algorithm\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
        }
    }
    (*s).origPtr = -(1 as ::core::ffi::c_int) as Int32;
    i = 0 as ::core::ffi::c_int as Int32;
    while i < (*s).nblock {
        if *ptr.offset(i as isize) == 0 as ::core::ffi::c_uint {
            (*s).origPtr = i;
            break;
        } else {
            i += 1;
        }
    }
    if !((*s).origPtr != -(1 as ::core::ffi::c_int)) {
        BZ2_bz__AssertH__fail(1003 as ::core::ffi::c_int);
    }
}
