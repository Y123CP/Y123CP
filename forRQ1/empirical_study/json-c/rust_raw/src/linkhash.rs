extern "C" {
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn json_c_get_random_seed() -> ::core::ffi::c_int;
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_entry {
    pub k: *const ::core::ffi::c_void,
    pub k_is_constant: ::core::ffi::c_int,
    pub v: *const ::core::ffi::c_void,
    pub next: *mut lh_entry,
    pub prev: *mut lh_entry,
}
pub type json_bool = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_table {
    pub size: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub head: *mut lh_entry,
    pub tail: *mut lh_entry,
    pub table: *mut lh_entry,
    pub free_fn: Option<lh_entry_free_fn>,
    pub hash_fn: Option<lh_hash_fn>,
    pub equal_fn: Option<lh_equal_fn>,
}
pub type lh_equal_fn = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub type lh_hash_fn = unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong;
pub type lh_entry_free_fn = unsafe extern "C" fn(*mut lh_entry) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ptr: *const ::core::ffi::c_void,
    pub i: size_t,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const JSON_C_OBJECT_ADD_CONSTANT_KEY: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const LH_PRIME: ::core::ffi::c_ulong = 0x9e370001 as ::core::ffi::c_ulong;
pub const LH_LOAD_FACTOR: ::core::ffi::c_double = 0.66f64;
pub const LH_EMPTY: *mut ::core::ffi::c_void =
    -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
pub const LH_FREED: *mut ::core::ffi::c_void =
    -(2 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
pub const JSON_C_STR_HASH_DFLT: ::core::ffi::c_int = 0;
pub const JSON_C_STR_HASH_PERLLIKE: ::core::ffi::c_int = 1;
#[inline]
unsafe extern "C" fn lh_get_hash(
    mut t: *const lh_table,
    mut k: *const ::core::ffi::c_void,
) -> ::core::ffi::c_ulong {
    return (*t).hash_fn.expect("non-null function pointer")(k);
}
#[inline]
unsafe extern "C" fn lh_entry_v(mut e: *const lh_entry) -> *mut ::core::ffi::c_void {
    return (*e).v as uintptr_t as *mut ::core::ffi::c_void;
}
static mut char_hash_fn: Option<lh_hash_fn> = unsafe {
    Some(lh_char_hash as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong)
};
#[no_mangle]
pub unsafe extern "C" fn json_global_set_string_hash(h: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match h {
        JSON_C_STR_HASH_DFLT => {
            char_hash_fn = Some(
                lh_char_hash
                    as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
            ) as Option<lh_hash_fn>;
        }
        JSON_C_STR_HASH_PERLLIKE => {
            char_hash_fn = Some(
                lh_perllike_str_hash
                    as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
            ) as Option<lh_hash_fn>;
        }
        _ => return -(1 as ::core::ffi::c_int),
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn lh_ptr_hash(mut k: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    return (k as ptrdiff_t as ::core::ffi::c_ulong).wrapping_mul(LH_PRIME)
        >> 4 as ::core::ffi::c_int
        & ULONG_MAX;
}
#[no_mangle]
pub unsafe extern "C" fn lh_ptr_equal(
    mut k1: *const ::core::ffi::c_void,
    mut k2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (k1 == k2) as ::core::ffi::c_int;
}
pub const HASH_LITTLE_ENDIAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn hashlittle(
    mut key: *const ::core::ffi::c_void,
    mut length: size_t,
    mut initval: uint32_t,
) -> uint32_t {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut u: C2RustUnnamed = C2RustUnnamed {
        ptr: ::core::ptr::null::<::core::ffi::c_void>(),
    };
    c = (0xdeadbeef as uint32_t)
        .wrapping_add(length as uint32_t)
        .wrapping_add(initval);
    b = c;
    a = b;
    u.ptr = key;
    if HASH_LITTLE_ENDIAN != 0 && u.i & 0x3 as size_t == 0 as size_t {
        let mut k: *const uint32_t = key as *const uint32_t;
        while length > 12 as size_t {
            a = (a as ::core::ffi::c_uint)
                .wrapping_add(*k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            b = (b as ::core::ffi::c_uint)
                .wrapping_add(*k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            c = (c as ::core::ffi::c_uint)
                .wrapping_add(*k.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_sub(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint
                ^ (c << 4 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_sub(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint
                ^ (a << 6 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_sub(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint
                ^ (b << 8 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_sub(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint
                ^ (c << 16 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_sub(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_sub(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint
                ^ (b << 4 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            length = (length as ::core::ffi::c_ulong).wrapping_sub(12 as ::core::ffi::c_ulong)
                as size_t as size_t;
            k = k.offset(3 as ::core::ffi::c_int as isize);
        }
        match length {
            12 => {
                c =
                    (c as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
                b =
                    (b as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            11 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(2 as ::core::ffi::c_int as isize) & 0xffffff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                b =
                    (b as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            10 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(2 as ::core::ffi::c_int as isize) & 0xffff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                b =
                    (b as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            9 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(2 as ::core::ffi::c_int as isize) & 0xff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                b =
                    (b as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            8 => {
                b =
                    (b as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            7 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(1 as ::core::ffi::c_int as isize) & 0xffffff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            6 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(1 as ::core::ffi::c_int as isize) & 0xffff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            5 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(1 as ::core::ffi::c_int as isize) & 0xff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            4 => {
                a =
                    (a as ::core::ffi::c_uint).wrapping_add(
                        *k.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                    ) as uint32_t as uint32_t;
            }
            3 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(0 as ::core::ffi::c_int as isize) & 0xffffff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            2 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(0 as ::core::ffi::c_int as isize) & 0xffff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            1 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    (*k.offset(0 as ::core::ffi::c_int as isize) & 0xff as uint32_t)
                        as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            0 => return c,
            _ => {}
        }
    } else if HASH_LITTLE_ENDIAN != 0 && u.i & 0x1 as size_t == 0 as size_t {
        let mut k_0: *const uint16_t = key as *const uint16_t;
        let mut k8: *const uint8_t = ::core::ptr::null::<uint8_t>();
        while length > 12 as size_t {
            a = (a as ::core::ffi::c_uint).wrapping_add(
                (*k_0.offset(0 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                    (*k_0.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(
                (*k_0.offset(2 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                    (*k_0.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(
                (*k_0.offset(4 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                    (*k_0.offset(5 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_sub(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint
                ^ (c << 4 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_sub(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint
                ^ (a << 6 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_sub(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint
                ^ (b << 8 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_sub(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint
                ^ (c << 16 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_sub(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_sub(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint
                ^ (b << 4 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            length = (length as ::core::ffi::c_ulong).wrapping_sub(12 as ::core::ffi::c_ulong)
                as size_t as size_t;
            k_0 = k_0.offset(6 as ::core::ffi::c_int as isize);
        }
        k8 = k_0 as *const uint8_t;
        let mut current_block_102: u64;
        match length {
            12 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(4 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(5 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(2 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(0 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_102 = 8062065914618164218;
            }
            11 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    ((*k8.offset(10 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_102 = 17010573868819109381;
            }
            10 => {
                current_block_102 = 17010573868819109381;
            }
            9 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    *k8.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_102 = 4524380190166585165;
            }
            8 => {
                current_block_102 = 4524380190166585165;
            }
            7 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    ((*k8.offset(6 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_102 = 15270969162102947270;
            }
            6 => {
                current_block_102 = 15270969162102947270;
            }
            5 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    *k8.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_102 = 10229552095456696480;
            }
            4 => {
                current_block_102 = 10229552095456696480;
            }
            3 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    ((*k8.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_102 = 2159893340451696585;
            }
            2 => {
                current_block_102 = 2159893340451696585;
            }
            1 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    *k8.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_102 = 8062065914618164218;
            }
            0 => return c,
            _ => {
                current_block_102 = 8062065914618164218;
            }
        }
        match current_block_102 {
            15270969162102947270 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    *k_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(0 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            4524380190166585165 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(2 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(0 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            17010573868819109381 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    *k_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(2 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(0 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            10229552095456696480 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    (*k_0.offset(0 as ::core::ffi::c_int as isize) as uint32_t).wrapping_add(
                        (*k_0.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                            << 16 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            2159893340451696585 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    *k_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            _ => {}
        }
    } else {
        let mut k_1: *const uint8_t = key as *const uint8_t;
        while length > 12 as size_t {
            a = (a as ::core::ffi::c_uint)
                .wrapping_add(*k_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                    << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            b = (b as ::core::ffi::c_uint)
                .wrapping_add(*k_1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(5 as ::core::ffi::c_int as isize) as uint32_t)
                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(6 as ::core::ffi::c_int as isize) as uint32_t)
                    << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(7 as ::core::ffi::c_int as isize) as uint32_t)
                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            c = (c as ::core::ffi::c_uint)
                .wrapping_add(*k_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
                as uint32_t as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(9 as ::core::ffi::c_int as isize) as uint32_t)
                    << 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(10 as ::core::ffi::c_int as isize) as uint32_t)
                    << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(
                ((*k_1.offset(11 as ::core::ffi::c_int as isize) as uint32_t)
                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as uint32_t as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_sub(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint
                ^ (c << 4 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_sub(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint
                ^ (a << 6 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_sub(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint
                ^ (b << 8 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_sub(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            a = (a as ::core::ffi::c_uint
                ^ (c << 16 as ::core::ffi::c_int
                    | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_add(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_sub(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            b = (b as ::core::ffi::c_uint
                ^ (a << 19 as ::core::ffi::c_int
                    | a >> 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            a = (a as ::core::ffi::c_uint).wrapping_add(c as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint).wrapping_sub(b as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            c = (c as ::core::ffi::c_uint
                ^ (b << 4 as ::core::ffi::c_int
                    | b >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as uint32_t;
            b = (b as ::core::ffi::c_uint).wrapping_add(a as ::core::ffi::c_uint) as uint32_t
                as uint32_t;
            length = (length as ::core::ffi::c_ulong).wrapping_sub(12 as ::core::ffi::c_ulong)
                as size_t as size_t;
            k_1 = k_1.offset(12 as ::core::ffi::c_int as isize);
        }
        let mut current_block_153: u64;
        match length {
            12 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(11 as ::core::ffi::c_int as isize) as uint32_t)
                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 12004229728070162418;
            }
            11 => {
                current_block_153 = 12004229728070162418;
            }
            10 => {
                current_block_153 = 13529034234584902216;
            }
            9 => {
                current_block_153 = 12799776824301345776;
            }
            8 => {
                current_block_153 = 2704717237347425666;
            }
            7 => {
                current_block_153 = 7930884878441681367;
            }
            6 => {
                current_block_153 = 14146705567854054825;
            }
            5 => {
                current_block_153 = 15150018934656932653;
            }
            4 => {
                current_block_153 = 17505504008781020068;
            }
            3 => {
                current_block_153 = 5951553903778974260;
            }
            2 => {
                current_block_153 = 2572363314455137002;
            }
            1 => {
                current_block_153 = 14793654828423728963;
            }
            0 => return c,
            _ => {
                current_block_153 = 7728257318064351663;
            }
        }
        match current_block_153 {
            12004229728070162418 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(10 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 13529034234584902216;
            }
            _ => {}
        }
        match current_block_153 {
            13529034234584902216 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(9 as ::core::ffi::c_int as isize) as uint32_t)
                        << 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 12799776824301345776;
            }
            _ => {}
        }
        match current_block_153 {
            12799776824301345776 => {
                c = (c as ::core::ffi::c_uint).wrapping_add(
                    *k_1.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 2704717237347425666;
            }
            _ => {}
        }
        match current_block_153 {
            2704717237347425666 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(7 as ::core::ffi::c_int as isize) as uint32_t)
                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 7930884878441681367;
            }
            _ => {}
        }
        match current_block_153 {
            7930884878441681367 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(6 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 14146705567854054825;
            }
            _ => {}
        }
        match current_block_153 {
            14146705567854054825 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(5 as ::core::ffi::c_int as isize) as uint32_t)
                        << 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 15150018934656932653;
            }
            _ => {}
        }
        match current_block_153 {
            15150018934656932653 => {
                b = (b as ::core::ffi::c_uint).wrapping_add(
                    *k_1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 17505504008781020068;
            }
            _ => {}
        }
        match current_block_153 {
            17505504008781020068 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
                        << 24 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 5951553903778974260;
            }
            _ => {}
        }
        match current_block_153 {
            5951553903778974260 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                        << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 2572363314455137002;
            }
            _ => {}
        }
        match current_block_153 {
            2572363314455137002 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    ((*k_1.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                        << 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
                current_block_153 = 14793654828423728963;
            }
            _ => {}
        }
        match current_block_153 {
            14793654828423728963 => {
                a = (a as ::core::ffi::c_uint).wrapping_add(
                    *k_1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint,
                ) as uint32_t as uint32_t;
            }
            _ => {}
        }
    }
    c = (c as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint) as uint32_t;
    c = (c as ::core::ffi::c_uint).wrapping_sub(
        (b << 14 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    a = (a as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint) as uint32_t;
    a = (a as ::core::ffi::c_uint).wrapping_sub(
        (c << 11 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    b = (b as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint) as uint32_t;
    b = (b as ::core::ffi::c_uint).wrapping_sub(
        (a << 25 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    c = (c as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint) as uint32_t;
    c = (c as ::core::ffi::c_uint).wrapping_sub(
        (b << 16 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    a = (a as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint) as uint32_t;
    a = (a as ::core::ffi::c_uint).wrapping_sub(
        (c << 4 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    b = (b as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint) as uint32_t;
    b = (b as ::core::ffi::c_uint).wrapping_sub(
        (a << 14 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    c = (c as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint) as uint32_t;
    c = (c as ::core::ffi::c_uint).wrapping_sub(
        (b << 24 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 24 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    return c;
}
unsafe extern "C" fn lh_perllike_str_hash(
    mut k: *const ::core::ffi::c_void,
) -> ::core::ffi::c_ulong {
    let mut rkey: *const ::core::ffi::c_char = k as *const ::core::ffi::c_char;
    let mut hashval: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    while *rkey != 0 {
        let fresh0 = rkey;
        rkey = rkey.offset(1);
        hashval = hashval
            .wrapping_mul(33 as ::core::ffi::c_uint)
            .wrapping_add(*fresh0 as ::core::ffi::c_uint);
    }
    return hashval as ::core::ffi::c_ulong;
}
unsafe extern "C" fn lh_char_hash(mut k: *const ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    static mut random_seed: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if random_seed == -(1 as ::core::ffi::c_int) {
        let mut seed: ::core::ffi::c_int = 0;
        loop {
            seed = json_c_get_random_seed();
            if !(seed == -(1 as ::core::ffi::c_int)) {
                break;
            }
        }
        ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
            &raw mut random_seed,
            -(1 as ::core::ffi::c_int),
            seed,
        )
        .0;
    }
    return hashlittle(
        k as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(k as *const ::core::ffi::c_char),
        random_seed as uint32_t,
    ) as ::core::ffi::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn lh_char_equal(
    mut k1: *const ::core::ffi::c_void,
    mut k2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (strcmp(
        k1 as *const ::core::ffi::c_char,
        k2 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_new(
    mut size: ::core::ffi::c_int,
    mut free_fn: Option<lh_entry_free_fn>,
    mut hash_fn: Option<lh_hash_fn>,
    mut equal_fn: Option<lh_equal_fn>,
) -> *mut lh_table {
    let mut i: ::core::ffi::c_int = 0;
    let mut t: *mut lh_table = ::core::ptr::null_mut::<lh_table>();
    t = calloc(1 as size_t, ::core::mem::size_of::<lh_table>() as size_t) as *mut lh_table;
    if t.is_null() {
        return ::core::ptr::null_mut::<lh_table>();
    }
    (*t).count = 0 as ::core::ffi::c_int;
    (*t).size = size;
    (*t).table =
        calloc(size as size_t, ::core::mem::size_of::<lh_entry>() as size_t) as *mut lh_entry;
    if (*t).table.is_null() {
        free(t as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<lh_table>();
    }
    (*t).free_fn = free_fn;
    (*t).hash_fn = hash_fn;
    (*t).equal_fn = equal_fn;
    i = 0 as ::core::ffi::c_int;
    while i < size {
        let ref mut fresh1 = (*(*t).table.offset(i as isize)).k;
        *fresh1 = LH_EMPTY;
        i += 1;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn lh_kchar_table_new(
    mut size: ::core::ffi::c_int,
    mut free_fn: Option<lh_entry_free_fn>,
) -> *mut lh_table {
    return lh_table_new(
        size,
        free_fn,
        char_hash_fn,
        Some(
            lh_char_equal
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lh_kptr_table_new(
    mut size: ::core::ffi::c_int,
    mut free_fn: Option<lh_entry_free_fn>,
) -> *mut lh_table {
    return lh_table_new(
        size,
        free_fn,
        Some(
            lh_ptr_hash as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ::core::ffi::c_ulong,
        ),
        Some(
            lh_ptr_equal
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_resize(
    mut t: *mut lh_table,
    mut new_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut new_t: *mut lh_table = ::core::ptr::null_mut::<lh_table>();
    let mut ent: *mut lh_entry = ::core::ptr::null_mut::<lh_entry>();
    new_t = lh_table_new(new_size, None, (*t).hash_fn, (*t).equal_fn);
    if new_t.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ent = (*t).head;
    while !ent.is_null() {
        let mut h: ::core::ffi::c_ulong = lh_get_hash(new_t, (*ent).k);
        let mut opts: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        if (*ent).k_is_constant != 0 {
            opts = JSON_C_OBJECT_ADD_CONSTANT_KEY as ::core::ffi::c_uint;
        }
        if lh_table_insert_w_hash(new_t, (*ent).k, (*ent).v, h, opts) != 0 as ::core::ffi::c_int {
            lh_table_free(new_t);
            return -(1 as ::core::ffi::c_int);
        }
        ent = (*ent).next;
    }
    free((*t).table as *mut ::core::ffi::c_void);
    (*t).table = (*new_t).table;
    (*t).size = new_size;
    (*t).head = (*new_t).head;
    (*t).tail = (*new_t).tail;
    free(new_t as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_free(mut t: *mut lh_table) {
    let mut c: *mut lh_entry = ::core::ptr::null_mut::<lh_entry>();
    if (*t).free_fn.is_some() {
        c = (*t).head;
        while !c.is_null() {
            (*t).free_fn.expect("non-null function pointer")(c);
            c = (*c).next;
        }
    }
    free((*t).table as *mut ::core::ffi::c_void);
    free(t as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_insert_w_hash(
    mut t: *mut lh_table,
    mut k: *const ::core::ffi::c_void,
    mut v: *const ::core::ffi::c_void,
    h: ::core::ffi::c_ulong,
    opts: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_ulong = 0;
    if (*t).count as ::core::ffi::c_double >= (*t).size as ::core::ffi::c_double * LH_LOAD_FACTOR {
        let mut new_size: ::core::ffi::c_int = if (*t).size > INT_MAX / 2 as ::core::ffi::c_int {
            INT_MAX
        } else {
            (*t).size * 2 as ::core::ffi::c_int
        };
        if (*t).size == INT_MAX || lh_table_resize(t, new_size) != 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    n = h.wrapping_rem((*t).size as ::core::ffi::c_ulong);
    while !((*(*t).table.offset(n as isize)).k == LH_EMPTY as *const ::core::ffi::c_void
        || (*(*t).table.offset(n as isize)).k == LH_FREED as *const ::core::ffi::c_void)
    {
        n = n.wrapping_add(1);
        if n as ::core::ffi::c_int == (*t).size {
            n = 0 as ::core::ffi::c_ulong;
        }
    }
    let ref mut fresh2 = (*(*t).table.offset(n as isize)).k;
    *fresh2 = k;
    (*(*t).table.offset(n as isize)).k_is_constant =
        (opts & JSON_C_OBJECT_ADD_CONSTANT_KEY as ::core::ffi::c_uint) as ::core::ffi::c_int;
    let ref mut fresh3 = (*(*t).table.offset(n as isize)).v;
    *fresh3 = v;
    (*t).count += 1;
    if (*t).head.is_null() {
        (*t).tail = (*t).table.offset(n as isize) as *mut lh_entry;
        (*t).head = (*t).tail;
        let ref mut fresh4 = (*(*t).table.offset(n as isize)).prev;
        *fresh4 = ::core::ptr::null_mut::<lh_entry>();
        let ref mut fresh5 = (*(*t).table.offset(n as isize)).next;
        *fresh5 = *fresh4;
    } else {
        (*(*t).tail).next = (*t).table.offset(n as isize) as *mut lh_entry;
        let ref mut fresh6 = (*(*t).table.offset(n as isize)).prev;
        *fresh6 = (*t).tail;
        let ref mut fresh7 = (*(*t).table.offset(n as isize)).next;
        *fresh7 = ::core::ptr::null_mut::<lh_entry>();
        (*t).tail = (*t).table.offset(n as isize) as *mut lh_entry;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_insert(
    mut t: *mut lh_table,
    mut k: *const ::core::ffi::c_void,
    mut v: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return lh_table_insert_w_hash(t, k, v, lh_get_hash(t, k), 0 as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_lookup_entry_w_hash(
    mut t: *mut lh_table,
    mut k: *const ::core::ffi::c_void,
    h: ::core::ffi::c_ulong,
) -> *mut lh_entry {
    let mut n: ::core::ffi::c_ulong = h.wrapping_rem((*t).size as ::core::ffi::c_ulong);
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count < (*t).size {
        if (*(*t).table.offset(n as isize)).k == LH_EMPTY as *const ::core::ffi::c_void {
            return ::core::ptr::null_mut::<lh_entry>();
        }
        if (*(*t).table.offset(n as isize)).k != LH_FREED as *const ::core::ffi::c_void
            && (*t).equal_fn.expect("non-null function pointer")(
                (*(*t).table.offset(n as isize)).k,
                k,
            ) != 0
        {
            return (*t).table.offset(n as isize) as *mut lh_entry;
        }
        n = n.wrapping_add(1);
        if n as ::core::ffi::c_int == (*t).size {
            n = 0 as ::core::ffi::c_ulong;
        }
        count += 1;
    }
    return ::core::ptr::null_mut::<lh_entry>();
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_lookup_entry(
    mut t: *mut lh_table,
    mut k: *const ::core::ffi::c_void,
) -> *mut lh_entry {
    return lh_table_lookup_entry_w_hash(t, k, lh_get_hash(t, k));
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_lookup_ex(
    mut t: *mut lh_table,
    mut k: *const ::core::ffi::c_void,
    mut v: *mut *mut ::core::ffi::c_void,
) -> json_bool {
    let mut e: *mut lh_entry = lh_table_lookup_entry(t, k);
    if !e.is_null() {
        if !v.is_null() {
            *v = lh_entry_v(e);
        }
        return 1 as json_bool;
    }
    if !v.is_null() {
        *v = NULL;
    }
    return 0 as json_bool;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_delete_entry(
    mut t: *mut lh_table,
    mut e: *mut lh_entry,
) -> ::core::ffi::c_int {
    let mut n: ptrdiff_t = e.offset_from((*t).table) as ::core::ffi::c_long as ptrdiff_t;
    if n < 0 as ptrdiff_t {
        return -(2 as ::core::ffi::c_int);
    }
    if (*(*t).table.offset(n as isize)).k == LH_EMPTY as *const ::core::ffi::c_void
        || (*(*t).table.offset(n as isize)).k == LH_FREED as *const ::core::ffi::c_void
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*t).count -= 1;
    if (*t).free_fn.is_some() {
        (*t).free_fn.expect("non-null function pointer")(e);
    }
    let ref mut fresh8 = (*(*t).table.offset(n as isize)).v;
    *fresh8 = ::core::ptr::null::<::core::ffi::c_void>();
    let ref mut fresh9 = (*(*t).table.offset(n as isize)).k;
    *fresh9 = LH_FREED;
    if (*t).tail == (*t).table.offset(n as isize) as *mut lh_entry
        && (*t).head == (*t).table.offset(n as isize) as *mut lh_entry
    {
        (*t).tail = ::core::ptr::null_mut::<lh_entry>();
        (*t).head = (*t).tail;
    } else if (*t).head == (*t).table.offset(n as isize) as *mut lh_entry {
        (*(*(*t).head).next).prev = ::core::ptr::null_mut::<lh_entry>();
        (*t).head = (*(*t).head).next;
    } else if (*t).tail == (*t).table.offset(n as isize) as *mut lh_entry {
        (*(*(*t).tail).prev).next = ::core::ptr::null_mut::<lh_entry>();
        (*t).tail = (*(*t).tail).prev;
    } else {
        let ref mut fresh10 = (*(*(*t).table.offset(n as isize)).prev).next;
        *fresh10 = (*(*t).table.offset(n as isize)).next;
        let ref mut fresh11 = (*(*(*t).table.offset(n as isize)).next).prev;
        *fresh11 = (*(*t).table.offset(n as isize)).prev;
    }
    let ref mut fresh12 = (*(*t).table.offset(n as isize)).prev;
    *fresh12 = ::core::ptr::null_mut::<lh_entry>();
    let ref mut fresh13 = (*(*t).table.offset(n as isize)).next;
    *fresh13 = *fresh12;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_delete(
    mut t: *mut lh_table,
    mut k: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut e: *mut lh_entry = lh_table_lookup_entry(t, k);
    if e.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return lh_table_delete_entry(t, e);
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_length(mut t: *mut lh_table) -> ::core::ffi::c_int {
    return (*t).count;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
