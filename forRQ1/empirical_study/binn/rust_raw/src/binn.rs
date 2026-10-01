use ::libc;
extern "C" {
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn atof(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type BOOL = ::core::ffi::c_int;
pub type int64 = ::core::ffi::c_longlong;
pub type uint64 = ::core::ffi::c_ulonglong;
pub type binn_mem_free = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_struct {
    pub header: ::core::ffi::c_int,
    pub allocated: BOOL,
    pub writable: BOOL,
    pub dirty: BOOL,
    pub pbuf: *mut ::core::ffi::c_void,
    pub pre_allocated: BOOL,
    pub alloc_size: ::core::ffi::c_int,
    pub used_size: ::core::ffi::c_int,
    pub type_0: ::core::ffi::c_int,
    pub ptr: *mut ::core::ffi::c_void,
    pub size: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub freefn: binn_mem_free,
    pub c2rust_unnamed: C2RustUnnamed,
    pub disable_int_compression: BOOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub vint8: ::core::ffi::c_schar,
    pub vint16: ::core::ffi::c_short,
    pub vint32: ::core::ffi::c_int,
    pub vint64: int64,
    pub vuint8: ::core::ffi::c_uchar,
    pub vuint16: ::core::ffi::c_ushort,
    pub vuint32: ::core::ffi::c_uint,
    pub vuint64: uint64,
    pub vchar: ::core::ffi::c_schar,
    pub vuchar: ::core::ffi::c_uchar,
    pub vshort: ::core::ffi::c_short,
    pub vushort: ::core::ffi::c_ushort,
    pub vint: ::core::ffi::c_int,
    pub vuint: ::core::ffi::c_uint,
    pub vfloat: ::core::ffi::c_float,
    pub vdouble: ::core::ffi::c_double,
    pub vbool: BOOL,
}
pub type binn = binn_struct;
pub type u32_0 = ::core::ffi::c_uint;
pub type u64_0 = ::core::ffi::c_ulonglong;
pub type u16_0 = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_iter_struct {
    pub pnext: *mut ::core::ffi::c_uchar,
    pub plimit: *mut ::core::ffi::c_uchar,
    pub type_0: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub current: ::core::ffi::c_int,
}
pub type binn_iter = binn_iter_struct;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT8_MIN: ::core::ffi::c_int = -(128 as ::core::ffi::c_int);
pub const INT16_MIN: ::core::ffi::c_int = -(32767 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const INT32_MIN: ::core::ffi::c_int =
    -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const INT64_MIN: ::core::ffi::c_long =
    -(9223372036854775807 as ::core::ffi::c_long) - 1 as ::core::ffi::c_long;
pub const INT8_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const INT16_MAX: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT8_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const BINN_MAGIC: ::core::ffi::c_int = 0x1f22b11f as ::core::ffi::c_int;
pub const MAX_BINN_HEADER: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MIN_BINN_SIZE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CHUNK_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const BINN_STRUCT: ::core::ffi::c_int = 1;
pub const BINN_BUFFER: ::core::ffi::c_int = 2;
#[no_mangle]
pub static mut malloc_fn: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void> = None;
#[no_mangle]
pub static mut realloc_fn: Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
> = None;
#[no_mangle]
pub static mut free_fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
unsafe extern "C" fn copy_be16(mut pdest: *mut u16_0, mut psource: *mut u16_0) {
    let mut source: *mut ::core::ffi::c_uchar = psource as *mut ::core::ffi::c_uchar;
    let mut dest: *mut ::core::ffi::c_uchar = pdest as *mut ::core::ffi::c_uchar;
    *dest.offset(0 as ::core::ffi::c_int as isize) =
        *source.offset(1 as ::core::ffi::c_int as isize);
    *dest.offset(1 as ::core::ffi::c_int as isize) =
        *source.offset(0 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn copy_be32(mut pdest: *mut u32_0, mut psource: *mut u32_0) {
    let mut source: *mut ::core::ffi::c_uchar = psource as *mut ::core::ffi::c_uchar;
    let mut dest: *mut ::core::ffi::c_uchar = pdest as *mut ::core::ffi::c_uchar;
    *dest.offset(0 as ::core::ffi::c_int as isize) =
        *source.offset(3 as ::core::ffi::c_int as isize);
    *dest.offset(1 as ::core::ffi::c_int as isize) =
        *source.offset(2 as ::core::ffi::c_int as isize);
    *dest.offset(2 as ::core::ffi::c_int as isize) =
        *source.offset(1 as ::core::ffi::c_int as isize);
    *dest.offset(3 as ::core::ffi::c_int as isize) =
        *source.offset(0 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn copy_be64(mut pdest: *mut u64_0, mut psource: *mut u64_0) {
    let mut source: *mut ::core::ffi::c_uchar = psource as *mut ::core::ffi::c_uchar;
    let mut dest: *mut ::core::ffi::c_uchar = pdest as *mut ::core::ffi::c_uchar;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        *dest.offset(i as isize) = *source.offset((7 as ::core::ffi::c_int - i) as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn binn_version() -> *mut ::core::ffi::c_char {
    return BINN_VERSION.as_ptr() as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_alloc_functions(
    mut new_malloc: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    mut new_realloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    >,
    mut new_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
) {
    malloc_fn = new_malloc;
    realloc_fn = new_realloc;
    free_fn = new_free;
}
unsafe extern "C" fn check_alloc_functions() {
    if malloc_fn.is_none() {
        malloc_fn = Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void)
            as Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
    }
    if realloc_fn.is_none() {
        realloc_fn = Some(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        )
            as Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
            >;
    }
    if free_fn.is_none() {
        free_fn = Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    }
}
unsafe extern "C" fn binn_malloc(mut size: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    check_alloc_functions();
    return malloc_fn.expect("non-null function pointer")(size as size_t);
}
unsafe extern "C" fn binn_memdup(
    mut src: *const ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut dest: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if src.is_null() || size <= 0 as ::core::ffi::c_int {
        return NULL;
    }
    dest = binn_malloc(size);
    if dest.is_null() {
        return NULL;
    }
    memcpy(dest, src, size as size_t);
    return dest;
}
unsafe extern "C" fn strlen2(mut str: *mut ::core::ffi::c_char) -> size_t {
    if str.is_null() {
        return 0 as size_t;
    }
    return strlen(str);
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_type(
    mut storage_type: ::core::ffi::c_int,
    mut data_type_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if data_type_index < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if storage_type < BINN_STORAGE_MIN || storage_type > BINN_STORAGE_MAX {
        return -(1 as ::core::ffi::c_int);
    }
    if data_type_index < 16 as ::core::ffi::c_int {
        return storage_type | data_type_index;
    } else if data_type_index < 4096 as ::core::ffi::c_int {
        storage_type |= BINN_STORAGE_HAS_MORE;
        storage_type <<= 8 as ::core::ffi::c_int;
        data_type_index >>= 4 as ::core::ffi::c_int;
        return storage_type | data_type_index;
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_type_info(
    mut long_type: ::core::ffi::c_int,
    mut pstorage_type: *mut ::core::ffi::c_int,
    mut pextra_type: *mut ::core::ffi::c_int,
) -> BOOL {
    let mut storage_type: ::core::ffi::c_int = 0;
    let mut extra_type: ::core::ffi::c_int = 0;
    let mut retval: BOOL = TRUE;
    let mut current_block_11: u64;
    loop {
        if long_type < 0 as ::core::ffi::c_int {
            current_block_11 = 2077634427133855796;
            break;
        }
        if long_type <= 0xff as ::core::ffi::c_int {
            storage_type = long_type & BINN_STORAGE_MASK;
            extra_type = long_type & BINN_TYPE_MASK;
            current_block_11 = 10048703153582371463;
            break;
        } else if long_type <= 0xffff as ::core::ffi::c_int {
            storage_type = long_type & BINN_STORAGE_MASK16;
            storage_type >>= 8 as ::core::ffi::c_int;
            extra_type = long_type & BINN_TYPE_MASK16;
            extra_type >>= 4 as ::core::ffi::c_int;
            current_block_11 = 10048703153582371463;
            break;
        } else {
            if !(long_type & BINN_STORAGE_VIRTUAL != 0) {
                current_block_11 = 2077634427133855796;
                break;
            }
            long_type &= 0xffff as ::core::ffi::c_int;
        }
    }
    match current_block_11 {
        2077634427133855796 => {
            storage_type = -(1 as ::core::ffi::c_int);
            extra_type = -(1 as ::core::ffi::c_int);
            retval = FALSE as BOOL;
        }
        _ => {}
    }
    if !pstorage_type.is_null() {
        *pstorage_type = storage_type;
    }
    if !pextra_type.is_null() {
        *pextra_type = extra_type;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_create(
    mut item: *mut binn,
    mut type_0: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut pointer: *mut ::core::ffi::c_void,
) -> BOOL {
    let mut current_block: u64;
    let mut retval: BOOL = FALSE;
    match type_0 {
        BINN_LIST | BINN_MAP | BINN_OBJECT => {
            if !(item.is_null() || size < 0 as ::core::ffi::c_int) {
                if size < MIN_BINN_SIZE {
                    if !pointer.is_null() {
                        current_block = 17338405409163857808;
                    } else {
                        size = 0 as ::core::ffi::c_int;
                        current_block = 10879442775620481940;
                    }
                } else {
                    current_block = 10879442775620481940;
                }
                match current_block {
                    17338405409163857808 => {}
                    _ => {
                        memset(
                            item as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<binn>() as size_t,
                        );
                        if !pointer.is_null() {
                            (*item).pre_allocated = TRUE as BOOL;
                        } else {
                            (*item).pre_allocated = FALSE as BOOL;
                            if size == 0 as ::core::ffi::c_int {
                                size = CHUNK_SIZE;
                            }
                            pointer = binn_malloc(size);
                            if pointer.is_null() {
                                return INVALID_BINN;
                            }
                        }
                        (*item).pbuf = pointer;
                        (*item).alloc_size = size;
                        (*item).header = BINN_MAGIC;
                        (*item).writable = TRUE as BOOL;
                        (*item).used_size = MAX_BINN_HEADER;
                        (*item).type_0 = type_0;
                        (*item).dirty = TRUE as BOOL;
                        retval = TRUE as BOOL;
                    }
                }
            }
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_new(
    mut type_0: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut pointer: *mut ::core::ffi::c_void,
) -> *mut binn {
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    item = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_create(item, type_0, size, pointer) == FALSE {
        free_fn.expect("non-null function pointer")(item as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*item).allocated = TRUE as BOOL;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_list(mut list: *mut binn) -> BOOL {
    return binn_create(list, BINN_LIST, 0 as ::core::ffi::c_int, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_map(mut map: *mut binn) -> BOOL {
    return binn_create(map, BINN_MAP, 0 as ::core::ffi::c_int, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_object(mut object: *mut binn) -> BOOL {
    return binn_create(object, BINN_OBJECT, 0 as ::core::ffi::c_int, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn binn_list() -> *mut binn {
    return binn_new(
        BINN_LIST,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_map() -> *mut binn {
    return binn_new(
        BINN_MAP,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object() -> *mut binn {
    return binn_new(
        BINN_OBJECT,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_copy(mut old: *const ::core::ffi::c_void) -> *mut binn {
    let mut type_0: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut header_size: ::core::ffi::c_int = 0;
    let mut old_ptr: *mut ::core::ffi::c_uchar = binn_ptr(old) as *mut ::core::ffi::c_uchar;
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    size = 0 as ::core::ffi::c_int;
    if IsValidBinnHeader(
        old_ptr as *const ::core::ffi::c_void,
        &raw mut type_0,
        &raw mut count,
        &raw mut size,
        &raw mut header_size,
    ) == 0
    {
        return ::core::ptr::null_mut::<binn>();
    }
    item = binn_new(type_0, size - header_size + MAX_BINN_HEADER, NULL);
    if !item.is_null() {
        let mut dest: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        dest = ((*item).pbuf as *mut ::core::ffi::c_uchar).offset(MAX_BINN_HEADER as isize);
        memcpy(
            dest as *mut ::core::ffi::c_void,
            old_ptr.offset(header_size as isize) as *const ::core::ffi::c_void,
            (size - header_size) as size_t,
        );
        (*item).used_size = MAX_BINN_HEADER + size - header_size;
        (*item).count = count;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_load(
    mut data: *const ::core::ffi::c_void,
    mut value: *mut binn,
) -> BOOL {
    if data.is_null() || value.is_null() {
        return FALSE;
    }
    memset(
        value as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<binn>() as size_t,
    );
    (*value).header = BINN_MAGIC;
    if binn_is_valid(
        data,
        &raw mut (*value).type_0,
        &raw mut (*value).count,
        &raw mut (*value).size,
    ) == FALSE
    {
        return FALSE;
    }
    (*value).ptr = data as *mut ::core::ffi::c_void;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_load_ex(
    mut data: *const ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
    mut value: *mut binn,
) -> BOOL {
    if data.is_null() || value.is_null() || size <= 0 as ::core::ffi::c_int {
        return FALSE;
    }
    memset(
        value as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<binn>() as size_t,
    );
    (*value).header = BINN_MAGIC;
    if binn_is_valid_ex(
        data,
        &raw mut (*value).type_0,
        &raw mut (*value).count,
        &raw mut size,
    ) == FALSE
    {
        return FALSE;
    }
    (*value).ptr = data as *mut ::core::ffi::c_void;
    (*value).size = size;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_open(mut data: *const ::core::ffi::c_void) -> *mut binn {
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    item = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_load(data, item) == FALSE {
        free_fn.expect("non-null function pointer")(item as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*item).allocated = TRUE as BOOL;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_open_ex(
    mut data: *const ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> *mut binn {
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    item = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_load_ex(data, size, item) == FALSE {
        free_fn.expect("non-null function pointer")(item as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*item).allocated = TRUE as BOOL;
    return item;
}
unsafe extern "C" fn binn_get_ptr_type(mut ptr: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    if ptr.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    match *(ptr as *mut ::core::ffi::c_uint) {
        522367263 => return BINN_STRUCT,
        _ => return BINN_BUFFER,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_struct(mut ptr: *const ::core::ffi::c_void) -> BOOL {
    if ptr.is_null() {
        return FALSE;
    }
    if *(ptr as *mut ::core::ffi::c_uint) == BINN_MAGIC as ::core::ffi::c_uint {
        return TRUE;
    } else {
        return FALSE;
    };
}
unsafe extern "C" fn CalcAllocation(
    mut needed_size: ::core::ffi::c_int,
    mut alloc_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut calc_size: ::core::ffi::c_int = 0;
    calc_size = alloc_size;
    while calc_size < needed_size {
        calc_size <<= 1 as ::core::ffi::c_int;
    }
    return calc_size;
}
unsafe extern "C" fn CheckAllocation(
    mut item: *mut binn,
    mut add_size: ::core::ffi::c_int,
) -> BOOL {
    let mut alloc_size: ::core::ffi::c_int = 0;
    let mut ptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*item).used_size + add_size > (*item).alloc_size {
        if (*item).pre_allocated != 0 {
            return FALSE;
        }
        alloc_size = CalcAllocation((*item).used_size + add_size, (*item).alloc_size);
        ptr = realloc_fn.expect("non-null function pointer")((*item).pbuf, alloc_size as size_t);
        if ptr.is_null() {
            return FALSE;
        }
        (*item).pbuf = ptr;
        (*item).alloc_size = alloc_size;
    }
    return TRUE;
}
unsafe extern "C" fn AdvanceDataPos(
    mut p: *mut ::core::ffi::c_uchar,
    mut plimit: *mut ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_uchar {
    let mut byte: ::core::ffi::c_uchar = 0;
    let mut storage_type: ::core::ffi::c_int = 0;
    let mut DataSize: ::core::ffi::c_int = 0;
    if p > plimit {
        return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    byte = *p;
    p = p.offset(1);
    storage_type = byte as ::core::ffi::c_int & BINN_STORAGE_MASK;
    if byte as ::core::ffi::c_int & BINN_STORAGE_HAS_MORE != 0 {
        p = p.offset(1);
    }
    match storage_type {
        BINN_STORAGE_NOBYTES => {}
        BINN_STORAGE_BYTE => {
            p = p.offset(1);
        }
        BINN_STORAGE_WORD => {
            p = p.offset(2 as ::core::ffi::c_int as isize);
        }
        BINN_STORAGE_DWORD => {
            p = p.offset(4 as ::core::ffi::c_int as isize);
        }
        BINN_STORAGE_QWORD => {
            p = p.offset(8 as ::core::ffi::c_int as isize);
        }
        BINN_STORAGE_BLOB | BINN_STORAGE_STRING => {
            if p > plimit {
                return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            DataSize = *p as ::core::ffi::c_int;
            if DataSize & 0x80 as ::core::ffi::c_int != 0 {
                if p.offset(::core::mem::size_of::<::core::ffi::c_int>() as usize as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    > plimit
                {
                    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
                copy_be32(&raw mut DataSize as *mut u32_0, p as *mut u32_0);
                DataSize &= 0x7fffffff as ::core::ffi::c_int;
                p = p.offset(4 as ::core::ffi::c_int as isize);
            } else {
                p = p.offset(1);
            }
            p = p.offset(DataSize as isize);
            if storage_type == BINN_STORAGE_STRING {
                p = p.offset(1);
            }
        }
        BINN_STORAGE_CONTAINER => {
            if p > plimit {
                return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
            DataSize = *p as ::core::ffi::c_int;
            if DataSize & 0x80 as ::core::ffi::c_int != 0 {
                if p.offset(::core::mem::size_of::<::core::ffi::c_int>() as usize as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    > plimit
                {
                    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
                copy_be32(&raw mut DataSize as *mut u32_0, p as *mut u32_0);
                DataSize &= 0x7fffffff as ::core::ffi::c_int;
            }
            DataSize -= 1;
            p = p.offset(DataSize as isize);
        }
        _ => return ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    }
    return p;
}
unsafe extern "C" fn read_map_id(
    mut pp: *mut *mut ::core::ffi::c_uchar,
    mut plimit: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut c: ::core::ffi::c_uchar = 0;
    let mut sign: ::core::ffi::c_uchar = 0;
    let mut type_0: ::core::ffi::c_uchar = 0;
    let mut id: ::core::ffi::c_int = 0;
    let mut extra_bytes: ::core::ffi::c_int = 0;
    p = *pp;
    if p > plimit {
        return 0 as ::core::ffi::c_int;
    }
    let fresh11 = p;
    p = p.offset(1);
    c = *fresh11;
    if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        extra_bytes = ((c as ::core::ffi::c_int & 0x60 as ::core::ffi::c_int)
            >> 5 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int;
        if p.offset(extra_bytes as isize) > plimit {
            *pp = p.offset(extra_bytes as isize);
            return 0 as ::core::ffi::c_int;
        }
    }
    type_0 = (c as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    sign = (c as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        sign = (c as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        id = c as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int;
    } else if type_0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int {
        id = c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int;
        let fresh12 = p;
        p = p.offset(1);
        id = id << 8 as ::core::ffi::c_int | *fresh12 as ::core::ffi::c_int;
    } else if type_0 as ::core::ffi::c_int == 0xa0 as ::core::ffi::c_int {
        id = c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int;
        let fresh13 = p;
        p = p.offset(1);
        id = id << 8 as ::core::ffi::c_int | *fresh13 as ::core::ffi::c_int;
        let fresh14 = p;
        p = p.offset(1);
        id = id << 8 as ::core::ffi::c_int | *fresh14 as ::core::ffi::c_int;
    } else if type_0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int {
        id = c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int;
        let fresh15 = p;
        p = p.offset(1);
        id = id << 8 as ::core::ffi::c_int | *fresh15 as ::core::ffi::c_int;
        let fresh16 = p;
        p = p.offset(1);
        id = id << 8 as ::core::ffi::c_int | *fresh16 as ::core::ffi::c_int;
        let fresh17 = p;
        p = p.offset(1);
        id = id << 8 as ::core::ffi::c_int | *fresh17 as ::core::ffi::c_int;
    } else if type_0 as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int {
        copy_be32(&raw mut id as *mut u32_0, p as *mut u32_0);
        p = p.offset(4 as ::core::ffi::c_int as isize);
    } else {
        *pp = plimit.offset(2 as ::core::ffi::c_int as isize);
        return 0 as ::core::ffi::c_int;
    }
    if sign != 0 {
        id = -id;
    }
    *pp = p;
    return id;
}
unsafe extern "C" fn SearchForID(
    mut p: *mut ::core::ffi::c_uchar,
    mut header_size: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut numitems: ::core::ffi::c_int,
    mut id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut plimit: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut base: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut i: ::core::ffi::c_int = 0;
    let mut int32: ::core::ffi::c_int = 0;
    base = p;
    plimit = p
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as ::core::ffi::c_int;
    while i < numitems {
        int32 = read_map_id(&raw mut p, plimit);
        if p > plimit {
            break;
        }
        if int32 == id {
            return p;
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            break;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
unsafe extern "C" fn SearchForKey(
    mut p: *mut ::core::ffi::c_uchar,
    mut header_size: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
    mut numitems: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_uchar {
    let mut len: ::core::ffi::c_uchar = 0;
    let mut plimit: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut base: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut i: ::core::ffi::c_int = 0;
    let mut keylen: ::core::ffi::c_int = 0;
    base = p;
    plimit = p
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    p = p.offset(header_size as isize);
    keylen = strlen(key) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < numitems {
        if p > plimit {
            break;
        }
        len = *p;
        p = p.offset(1);
        if p.offset(len as ::core::ffi::c_int as isize) > plimit {
            break;
        }
        if len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if strncasecmp(p as *mut ::core::ffi::c_char, key, len as size_t)
                == 0 as ::core::ffi::c_int
            {
                if keylen == len as ::core::ffi::c_int {
                    p = p.offset(len as ::core::ffi::c_int as isize);
                    return p;
                }
            }
            p = p.offset(len as ::core::ffi::c_int as isize);
        } else if len as ::core::ffi::c_int == keylen {
            return p;
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            break;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
unsafe extern "C" fn binn_list_add_raw(
    mut item: *mut binn,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> BOOL {
    if item.is_null() || (*item).type_0 != BINN_LIST || (*item).writable == FALSE {
        return FALSE;
    }
    if AddValue(item, type_0, pvalue, size) == FALSE {
        return FALSE;
    }
    (*item).count += 1;
    return TRUE;
}
unsafe extern "C" fn binn_object_set_raw(
    mut item: *mut binn,
    mut key: *const ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> BOOL {
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_uchar = 0;
    let mut int32: ::core::ffi::c_int = 0;
    if item.is_null() || (*item).type_0 != BINN_OBJECT || (*item).writable == FALSE {
        return FALSE;
    }
    if key.is_null() {
        return FALSE;
    }
    int32 = strlen(key) as ::core::ffi::c_int;
    if int32 > 255 as ::core::ffi::c_int {
        return FALSE;
    }
    p = SearchForKey(
        (*item).pbuf as *mut ::core::ffi::c_uchar,
        MAX_BINN_HEADER,
        (*item).used_size,
        (*item).count,
        key,
    );
    if !p.is_null() {
        return FALSE;
    }
    if CheckAllocation(item, 1 as ::core::ffi::c_int + int32) == FALSE {
        return FALSE;
    }
    p = ((*item).pbuf as *mut ::core::ffi::c_uchar).offset((*item).used_size as isize);
    len = int32 as ::core::ffi::c_uchar;
    *p = len;
    p = p.offset(1);
    memcpy(
        p as *mut ::core::ffi::c_void,
        key as *const ::core::ffi::c_void,
        int32 as size_t,
    );
    int32 += 1;
    (*item).used_size += int32;
    if AddValue(item, type_0, pvalue, size) == FALSE {
        (*item).used_size -= int32;
        return FALSE;
    }
    (*item).count += 1;
    return TRUE;
}
unsafe extern "C" fn binn_map_set_raw(
    mut item: *mut binn,
    mut id: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> BOOL {
    let mut base: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut sign: ::core::ffi::c_uchar = 0;
    let mut id_size: ::core::ffi::c_int = 0;
    if item.is_null() || (*item).type_0 != BINN_MAP || (*item).writable == FALSE {
        return FALSE;
    }
    p = SearchForID(
        (*item).pbuf as *mut ::core::ffi::c_uchar,
        MAX_BINN_HEADER,
        (*item).used_size,
        (*item).count,
        id,
    );
    if !p.is_null() {
        return FALSE;
    }
    if CheckAllocation(item, 5 as ::core::ffi::c_int) == FALSE {
        return FALSE;
    }
    base = ((*item).pbuf as *mut ::core::ffi::c_uchar).offset((*item).used_size as isize);
    p = base;
    sign = (id < 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uchar;
    if sign != 0 {
        id = -id;
    }
    if id <= 0x3f as ::core::ffi::c_int {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 =
            ((sign as ::core::ffi::c_int) << 6 as ::core::ffi::c_int | id) as ::core::ffi::c_uchar;
    } else if id <= 0xfff as ::core::ffi::c_int {
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = (0x80 as ::core::ffi::c_int
            | (sign as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (id & 0xf00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = (id & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    } else if id <= 0xfffff as ::core::ffi::c_int {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = (0xa0 as ::core::ffi::c_int
            | (sign as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (id & 0xf0000 as ::core::ffi::c_int) >> 16 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ((id & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = (id & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    } else if id <= 0xfffffff as ::core::ffi::c_int {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = (0xc0 as ::core::ffi::c_int
            | (sign as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (id & 0xf000000 as ::core::ffi::c_int) >> 24 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = ((id & 0xff0000 as ::core::ffi::c_int) >> 16 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = ((id & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = (id & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    } else {
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = 0xe0 as ::core::ffi::c_uchar;
        if sign != 0 {
            id = -id;
        }
        copy_be32(p as *mut u32_0, &raw mut id as *mut u32_0);
        p = p.offset(4 as ::core::ffi::c_int as isize);
    }
    id_size = p.offset_from(base) as ::core::ffi::c_long as ::core::ffi::c_int;
    (*item).used_size += id_size;
    if AddValue(item, type_0, pvalue, size) == FALSE {
        (*item).used_size -= id_size;
        return FALSE;
    }
    (*item).count += 1;
    return TRUE;
}
unsafe extern "C" fn compress_int(
    mut pstorage_type: *mut ::core::ffi::c_int,
    mut ptype: *mut ::core::ffi::c_int,
    mut psource: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut current_block: u64;
    let mut storage_type: ::core::ffi::c_int = 0;
    let mut storage_type2: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut type2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut vint: int64 = 0 as int64;
    let mut vuint: uint64 = 0;
    let mut pvalue: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    storage_type = *pstorage_type;
    if storage_type == BINN_STORAGE_BYTE {
        return psource;
    }
    type_0 = *ptype;
    match type_0 {
        BINN_INT64 => {
            vint = *(psource as *mut int64);
            current_block = 10020511525755771531;
        }
        BINN_INT32 => {
            vint = *(psource as *mut ::core::ffi::c_int) as int64;
            current_block = 10020511525755771531;
        }
        BINN_INT16 => {
            vint = *(psource as *mut ::core::ffi::c_short) as int64;
            current_block = 10020511525755771531;
        }
        BINN_UINT64 => {
            vuint = *(psource as *mut uint64);
            current_block = 11849129699931280770;
        }
        BINN_UINT32 => {
            vuint = *(psource as *mut ::core::ffi::c_uint) as uint64;
            current_block = 11849129699931280770;
        }
        BINN_UINT16 => {
            vuint = *(psource as *mut ::core::ffi::c_ushort) as uint64;
            current_block = 11849129699931280770;
        }
        _ => {
            current_block = 10020511525755771531;
        }
    }
    match current_block {
        10020511525755771531 => {
            if vint >= 0 as ::core::ffi::c_longlong {
                vuint = vint as uint64;
                current_block = 11849129699931280770;
            } else {
                if vint >= INT8_MIN as ::core::ffi::c_longlong {
                    type2 = BINN_INT8;
                } else if vint >= INT16_MIN as ::core::ffi::c_longlong {
                    type2 = BINN_INT16;
                } else if vint >= INT32_MIN as ::core::ffi::c_longlong {
                    type2 = BINN_INT32;
                }
                current_block = 8075753905672049457;
            }
        }
        _ => {}
    }
    match current_block {
        11849129699931280770 => {
            if vuint <= UINT8_MAX as ::core::ffi::c_ulonglong {
                type2 = BINN_UINT8;
            } else if vuint <= UINT16_MAX as ::core::ffi::c_ulonglong {
                type2 = BINN_UINT16;
            } else if vuint <= UINT32_MAX as ::core::ffi::c_ulonglong {
                type2 = BINN_UINT32;
            }
        }
        _ => {}
    }
    pvalue = psource as *mut ::core::ffi::c_char;
    if type2 != 0 && type2 != type_0 {
        *ptype = type2;
        storage_type2 = binn_get_write_storage(type2);
        *pstorage_type = storage_type2;
    }
    return pvalue as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn AddValue(
    mut item: *mut binn,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> BOOL {
    let mut int32: ::core::ffi::c_int = 0;
    let mut ArgSize: ::core::ffi::c_int = 0;
    let mut storage_type: ::core::ffi::c_int = 0;
    let mut extra_type: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    binn_get_type_info(type_0, &raw mut storage_type, &raw mut extra_type);
    if pvalue.is_null() {
        let mut current_block_1: u64;
        match storage_type {
            BINN_STORAGE_NOBYTES => {
                current_block_1 = 6873731126896040597;
            }
            BINN_STORAGE_BLOB | BINN_STORAGE_STRING => {
                if size == 0 as ::core::ffi::c_int {
                    current_block_1 = 6873731126896040597;
                } else {
                    current_block_1 = 18329605127095459026;
                }
            }
            _ => {
                current_block_1 = 18329605127095459026;
            }
        }
        match current_block_1 {
            18329605127095459026 => return FALSE,
            _ => {}
        }
    }
    if type_family(type_0) == BINN_FAMILY_INT && (*item).disable_int_compression == FALSE {
        pvalue = compress_int(&raw mut storage_type, &raw mut type_0, pvalue);
    }
    match storage_type {
        BINN_STORAGE_NOBYTES => {
            size = 0 as ::core::ffi::c_int;
            ArgSize = size;
        }
        BINN_STORAGE_BYTE => {
            size = 1 as ::core::ffi::c_int;
            ArgSize = size;
        }
        BINN_STORAGE_WORD => {
            size = 2 as ::core::ffi::c_int;
            ArgSize = size;
        }
        BINN_STORAGE_DWORD => {
            size = 4 as ::core::ffi::c_int;
            ArgSize = size;
        }
        BINN_STORAGE_QWORD => {
            size = 8 as ::core::ffi::c_int;
            ArgSize = size;
        }
        BINN_STORAGE_BLOB => {
            if size < 0 as ::core::ffi::c_int {
                return FALSE;
            }
            ArgSize = size + 4 as ::core::ffi::c_int;
        }
        BINN_STORAGE_STRING => {
            if size < 0 as ::core::ffi::c_int {
                return FALSE;
            }
            if size == 0 as ::core::ffi::c_int {
                size = strlen2(pvalue as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
            }
            ArgSize = size + 5 as ::core::ffi::c_int;
        }
        BINN_STORAGE_CONTAINER => {
            if size <= 0 as ::core::ffi::c_int {
                return FALSE;
            }
            ArgSize = size;
        }
        _ => return FALSE,
    }
    ArgSize += 2 as ::core::ffi::c_int;
    if CheckAllocation(item, ArgSize) == FALSE {
        return FALSE;
    }
    p = ((*item).pbuf as *mut ::core::ffi::c_uchar).offset((*item).used_size as isize);
    if storage_type != BINN_STORAGE_CONTAINER {
        if type_0 > 255 as ::core::ffi::c_int {
            let mut type16: u16_0 = type_0 as u16_0;
            copy_be16(p as *mut u16_0, &raw mut type16);
            p = p.offset(2 as ::core::ffi::c_int as isize);
            (*item).used_size += 2 as ::core::ffi::c_int;
        } else {
            *p = type_0 as ::core::ffi::c_uchar;
            p = p.offset(1);
            (*item).used_size += 1;
        }
    }
    match storage_type {
        BINN_STORAGE_BYTE => {
            *(p as *mut ::core::ffi::c_char) = *(pvalue as *mut ::core::ffi::c_char);
            (*item).used_size += 1 as ::core::ffi::c_int;
        }
        BINN_STORAGE_WORD => {
            copy_be16(p as *mut u16_0, pvalue as *mut u16_0);
            (*item).used_size += 2 as ::core::ffi::c_int;
        }
        BINN_STORAGE_DWORD => {
            copy_be32(p as *mut u32_0, pvalue as *mut u32_0);
            (*item).used_size += 4 as ::core::ffi::c_int;
        }
        BINN_STORAGE_QWORD => {
            copy_be64(p as *mut u64_0, pvalue as *mut u64_0);
            (*item).used_size += 8 as ::core::ffi::c_int;
        }
        BINN_STORAGE_BLOB | BINN_STORAGE_STRING => {
            if size > 127 as ::core::ffi::c_int {
                int32 = (size as ::core::ffi::c_uint | 0x80000000 as ::core::ffi::c_uint)
                    as ::core::ffi::c_int;
                copy_be32(p as *mut u32_0, &raw mut int32 as *mut u32_0);
                p = p.offset(4 as ::core::ffi::c_int as isize);
                (*item).used_size += 4 as ::core::ffi::c_int;
            } else {
                *p = size as ::core::ffi::c_uchar;
                p = p.offset(1);
                (*item).used_size += 1;
            }
            memcpy(p as *mut ::core::ffi::c_void, pvalue, size as size_t);
            if storage_type == BINN_STORAGE_STRING {
                p = p.offset(size as isize);
                *(p as *mut ::core::ffi::c_char) = 0 as ::core::ffi::c_int as ::core::ffi::c_char;
                size += 1;
            }
            (*item).used_size += size;
        }
        BINN_STORAGE_CONTAINER => {
            memcpy(p as *mut ::core::ffi::c_void, pvalue, size as size_t);
            (*item).used_size += size;
        }
        BINN_STORAGE_NOBYTES | _ => {}
    }
    (*item).dirty = TRUE as BOOL;
    return TRUE;
}
unsafe extern "C" fn binn_save_header(mut item: *mut binn) -> BOOL {
    let mut byte: ::core::ffi::c_uchar = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut int32: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    if item.is_null() {
        return FALSE;
    }
    p = ((*item).pbuf as *mut ::core::ffi::c_uchar).offset(MAX_BINN_HEADER as isize);
    size = (*item).used_size - MAX_BINN_HEADER + 3 as ::core::ffi::c_int;
    if (*item).count > 127 as ::core::ffi::c_int {
        p = p.offset(-(4 as ::core::ffi::c_int as isize));
        size += 3 as ::core::ffi::c_int;
        int32 = ((*item).count as ::core::ffi::c_uint | 0x80000000 as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
        copy_be32(p as *mut u32_0, &raw mut int32 as *mut u32_0);
    } else {
        p = p.offset(-1);
        *p = (*item).count as ::core::ffi::c_uchar;
    }
    if size > 127 as ::core::ffi::c_int {
        p = p.offset(-(4 as ::core::ffi::c_int as isize));
        size += 3 as ::core::ffi::c_int;
        int32 =
            (size as ::core::ffi::c_uint | 0x80000000 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        copy_be32(p as *mut u32_0, &raw mut int32 as *mut u32_0);
    } else {
        p = p.offset(-1);
        *p = size as ::core::ffi::c_uchar;
    }
    p = p.offset(-1);
    *p = (*item).type_0 as ::core::ffi::c_uchar;
    (*item).ptr = p as *mut ::core::ffi::c_void;
    (*item).size = size;
    (*item).dirty = FALSE as BOOL;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_free(mut item: *mut binn) {
    if item.is_null() {
        return;
    }
    if (*item).writable != 0 && (*item).pre_allocated == FALSE {
        free_fn.expect("non-null function pointer")((*item).pbuf);
    }
    if (*item).freefn.is_some() {
        (*item).freefn.expect("non-null function pointer")((*item).ptr);
    }
    if (*item).allocated != 0 {
        free_fn.expect("non-null function pointer")(item as *mut ::core::ffi::c_void);
    } else {
        memset(
            item as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<binn>() as size_t,
        );
        (*item).header = BINN_MAGIC;
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_release(mut item: *mut binn) -> *mut ::core::ffi::c_void {
    let mut data: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if item.is_null() {
        return NULL;
    }
    data = binn_ptr(item as *const ::core::ffi::c_void);
    if data > (*item).pbuf {
        memmove((*item).pbuf, data, (*item).size as size_t);
        data = (*item).pbuf;
    }
    if (*item).allocated != 0 {
        free_fn.expect("non-null function pointer")(item as *mut ::core::ffi::c_void);
    } else {
        memset(
            item as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<binn>() as size_t,
        );
        (*item).header = BINN_MAGIC;
    }
    return data;
}
unsafe extern "C" fn IsValidBinnHeader(
    mut pbuf: *const ::core::ffi::c_void,
    mut ptype: *mut ::core::ffi::c_int,
    mut pcount: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
    mut pheadersize: *mut ::core::ffi::c_int,
) -> BOOL {
    let mut byte: ::core::ffi::c_uchar = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut plimit: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut int32: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    if pbuf.is_null() {
        return FALSE;
    }
    p = pbuf as *mut ::core::ffi::c_uchar;
    if !psize.is_null() && *psize > 0 as ::core::ffi::c_int {
        if *psize < MIN_BINN_SIZE {
            return FALSE;
        }
        plimit = p
            .offset(*psize as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
    }
    byte = *p;
    p = p.offset(1);
    if byte as ::core::ffi::c_int & BINN_STORAGE_MASK != BINN_STORAGE_CONTAINER {
        return FALSE;
    }
    if byte as ::core::ffi::c_int & BINN_STORAGE_HAS_MORE != 0 {
        return FALSE;
    }
    type_0 = byte as ::core::ffi::c_int;
    match type_0 {
        BINN_LIST | BINN_MAP | BINN_OBJECT => {}
        _ => return FALSE,
    }
    if !plimit.is_null() && p > plimit {
        return FALSE;
    }
    int32 = *p as ::core::ffi::c_int;
    if int32 & 0x80 as ::core::ffi::c_int != 0 {
        if !plimit.is_null()
            && p.offset(::core::mem::size_of::<::core::ffi::c_int>() as usize as isize)
                .offset(-(1 as ::core::ffi::c_int as isize))
                > plimit
        {
            return FALSE;
        }
        copy_be32(&raw mut int32 as *mut u32_0, p as *mut u32_0);
        int32 &= 0x7fffffff as ::core::ffi::c_int;
        p = p.offset(4 as ::core::ffi::c_int as isize);
    } else {
        p = p.offset(1);
    }
    size = int32;
    if !plimit.is_null() && p > plimit {
        return FALSE;
    }
    int32 = *p as ::core::ffi::c_int;
    if int32 & 0x80 as ::core::ffi::c_int != 0 {
        if !plimit.is_null()
            && p.offset(::core::mem::size_of::<::core::ffi::c_int>() as usize as isize)
                .offset(-(1 as ::core::ffi::c_int as isize))
                > plimit
        {
            return FALSE;
        }
        copy_be32(&raw mut int32 as *mut u32_0, p as *mut u32_0);
        int32 &= 0x7fffffff as ::core::ffi::c_int;
        p = p.offset(4 as ::core::ffi::c_int as isize);
    } else {
        p = p.offset(1);
    }
    count = int32;
    if size < MIN_BINN_SIZE || count < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if !ptype.is_null() {
        *ptype = type_0;
    }
    if !pcount.is_null() {
        *pcount = count;
    }
    if !psize.is_null() {
        *psize = size;
    }
    if !pheadersize.is_null() {
        *pheadersize = p.offset_from(pbuf as *mut ::core::ffi::c_uchar) as ::core::ffi::c_long
            as ::core::ffi::c_int;
    }
    return TRUE;
}
unsafe extern "C" fn binn_buf_type(mut pbuf: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut type_0: ::core::ffi::c_int = 0;
    if IsValidBinnHeader(
        pbuf,
        &raw mut type_0,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) == 0
    {
        return INVALID_BINN;
    }
    return type_0;
}
unsafe extern "C" fn binn_buf_count(mut pbuf: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut nitems: ::core::ffi::c_int = 0;
    if IsValidBinnHeader(
        pbuf,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut nitems,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return nitems;
}
unsafe extern "C" fn binn_buf_size(mut pbuf: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if IsValidBinnHeader(
        pbuf,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut size,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn binn_ptr(mut ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    match binn_get_ptr_type(ptr) {
        BINN_STRUCT => {
            item = ptr as *mut binn;
            if (*item).writable != 0 && (*item).dirty != 0 {
                binn_save_header(item);
            }
            return (*item).ptr;
        }
        BINN_BUFFER => return ptr as *mut ::core::ffi::c_void,
        _ => return NULL,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_size(mut ptr: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    match binn_get_ptr_type(ptr) {
        BINN_STRUCT => {
            item = ptr as *mut binn;
            if (*item).writable != 0 && (*item).dirty != 0 {
                binn_save_header(item);
            }
            return (*item).size;
        }
        BINN_BUFFER => return binn_buf_size(ptr),
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_type(mut ptr: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    match binn_get_ptr_type(ptr) {
        BINN_STRUCT => {
            item = ptr as *mut binn;
            return (*item).type_0;
        }
        BINN_BUFFER => return binn_buf_type(ptr),
        _ => return -(1 as ::core::ffi::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_count(mut ptr: *const ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    match binn_get_ptr_type(ptr) {
        BINN_STRUCT => {
            item = ptr as *mut binn;
            return (*item).count;
        }
        BINN_BUFFER => return binn_buf_count(ptr),
        _ => return -(1 as ::core::ffi::c_int),
    };
}
unsafe extern "C" fn binn_is_valid_ex2(
    mut ptr: *const ::core::ffi::c_void,
    mut ptype: *mut ::core::ffi::c_int,
    mut pcount: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> BOOL {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut header_size: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut plimit: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut base: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_uchar = 0;
    if ptr.is_null() {
        return FALSE;
    }
    if !psize.is_null() && *psize > 0 as ::core::ffi::c_int {
        size = *psize;
    } else {
        size = 0 as ::core::ffi::c_int;
    }
    if IsValidBinnHeader(
        ptr,
        &raw mut type_0,
        &raw mut count,
        &raw mut size,
        &raw mut header_size,
    ) == 0
    {
        return FALSE;
    }
    if !psize.is_null() && *psize > 0 as ::core::ffi::c_int {
        if size > *psize {
            return FALSE;
        }
    }
    if !pcount.is_null() && *pcount > 0 as ::core::ffi::c_int {
        if count != *pcount {
            return FALSE;
        }
    }
    if !ptype.is_null() && *ptype != 0 as ::core::ffi::c_int {
        if type_0 != *ptype {
            return FALSE;
        }
    }
    p = ptr as *mut ::core::ffi::c_uchar;
    base = p;
    plimit = p
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as ::core::ffi::c_int;
    loop {
        if !(i < count) {
            current_block = 2604890879466389055;
            break;
        }
        match type_0 {
            BINN_OBJECT => {
                if p > plimit {
                    current_block = 17367276684611205500;
                    break;
                }
                len = *p;
                p = p.offset(1);
                p = p.offset(len as ::core::ffi::c_int as isize);
            }
            BINN_MAP => {
                read_map_id(&raw mut p, plimit);
            }
            BINN_LIST => {}
            _ => {
                current_block = 17367276684611205500;
                break;
            }
        }
        if p > plimit {
            current_block = 17367276684611205500;
            break;
        }
        if *p as ::core::ffi::c_int & BINN_STORAGE_MASK == BINN_STORAGE_CONTAINER {
            let mut size2: ::core::ffi::c_int = (plimit.offset_from(p) as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long)
                as ::core::ffi::c_int;
            if binn_is_valid_ex2(
                p as *const ::core::ffi::c_void,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                &raw mut size2,
            ) == FALSE
            {
                current_block = 17367276684611205500;
                break;
            }
            p = p.offset(size2 as isize);
        } else {
            p = AdvanceDataPos(p, plimit);
            if p.is_null() || p < base {
                current_block = 17367276684611205500;
                break;
            }
        }
        i += 1;
    }
    match current_block {
        17367276684611205500 => return FALSE,
        _ => {
            if !ptype.is_null() && *ptype == 0 as ::core::ffi::c_int {
                *ptype = type_0;
            }
            if !pcount.is_null() && *pcount == 0 as ::core::ffi::c_int {
                *pcount = count;
            }
            if !psize.is_null() {
                *psize = size;
            }
            return TRUE;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_valid_ex(
    mut ptr: *const ::core::ffi::c_void,
    mut ptype: *mut ::core::ffi::c_int,
    mut pcount: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> BOOL {
    let mut size: ::core::ffi::c_int = 0;
    if !psize.is_null() && *psize > 0 as ::core::ffi::c_int {
        size = *psize;
    } else {
        size = 0 as ::core::ffi::c_int;
    }
    if binn_is_valid_ex2(ptr, ptype, pcount, &raw mut size) == FALSE {
        return FALSE;
    }
    if !psize.is_null() {
        if *psize > 0 as ::core::ffi::c_int {
            if size != *psize {
                return FALSE;
            }
        } else if *psize == 0 as ::core::ffi::c_int {
            *psize = size;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_valid(
    mut ptr: *const ::core::ffi::c_void,
    mut ptype: *mut ::core::ffi::c_int,
    mut pcount: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> BOOL {
    if !ptype.is_null() {
        *ptype = 0 as ::core::ffi::c_int;
    }
    if !pcount.is_null() {
        *pcount = 0 as ::core::ffi::c_int;
    }
    if !psize.is_null() {
        *psize = 0 as ::core::ffi::c_int;
    }
    return binn_is_valid_ex(ptr, ptype, pcount, psize);
}
unsafe extern "C" fn GetValue(
    mut p: *mut ::core::ffi::c_uchar,
    mut plimit: *mut ::core::ffi::c_uchar,
    mut value: *mut binn,
) -> BOOL {
    let mut byte: ::core::ffi::c_uchar = 0;
    let mut data_type: ::core::ffi::c_int = 0;
    let mut storage_type: ::core::ffi::c_int = 0;
    let mut DataSize: ::core::ffi::c_int = 0;
    let mut p2: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if value.is_null() {
        return FALSE;
    }
    memset(
        value as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<binn>() as size_t,
    );
    (*value).header = BINN_MAGIC;
    p2 = p as *mut ::core::ffi::c_void;
    if p > plimit {
        return FALSE;
    }
    byte = *p;
    p = p.offset(1);
    storage_type = byte as ::core::ffi::c_int & BINN_STORAGE_MASK;
    if byte as ::core::ffi::c_int & BINN_STORAGE_HAS_MORE != 0 {
        data_type = (byte as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
        if p > plimit {
            return FALSE;
        }
        byte = *p;
        p = p.offset(1);
        data_type |= byte as ::core::ffi::c_int;
    } else {
        data_type = byte as ::core::ffi::c_int;
    }
    (*value).type_0 = data_type;
    match storage_type {
        BINN_STORAGE_NOBYTES => {}
        BINN_STORAGE_BYTE => {
            if p > plimit {
                return FALSE;
            }
            (*value).c2rust_unnamed.vuint8 = *p;
            (*value).ptr = p as *mut ::core::ffi::c_void;
        }
        BINN_STORAGE_WORD => {
            if p.offset(1 as ::core::ffi::c_int as isize) > plimit {
                return FALSE;
            }
            copy_be16(
                &raw mut (*value).c2rust_unnamed.vint16 as *mut u16_0,
                p as *mut u16_0,
            );
            (*value).ptr = &raw mut (*value).c2rust_unnamed.vint16 as *mut ::core::ffi::c_void;
        }
        BINN_STORAGE_DWORD => {
            if p.offset(3 as ::core::ffi::c_int as isize) > plimit {
                return FALSE;
            }
            copy_be32(
                &raw mut (*value).c2rust_unnamed.vint32 as *mut u32_0,
                p as *mut u32_0,
            );
            (*value).ptr = &raw mut (*value).c2rust_unnamed.vint32 as *mut ::core::ffi::c_void;
        }
        BINN_STORAGE_QWORD => {
            if p.offset(7 as ::core::ffi::c_int as isize) > plimit {
                return FALSE;
            }
            copy_be64(
                &raw mut (*value).c2rust_unnamed.vint64 as *mut u64_0,
                p as *mut u64_0,
            );
            (*value).ptr = &raw mut (*value).c2rust_unnamed.vint64 as *mut ::core::ffi::c_void;
        }
        BINN_STORAGE_BLOB | BINN_STORAGE_STRING => {
            if p > plimit {
                return FALSE;
            }
            DataSize = *p as ::core::ffi::c_int;
            if DataSize & 0x80 as ::core::ffi::c_int != 0 {
                if p.offset(3 as ::core::ffi::c_int as isize) > plimit {
                    return FALSE;
                }
                copy_be32(&raw mut DataSize as *mut u32_0, p as *mut u32_0);
                DataSize &= 0x7fffffff as ::core::ffi::c_int;
                p = p.offset(4 as ::core::ffi::c_int as isize);
            } else {
                p = p.offset(1);
            }
            if p.offset(DataSize as isize)
                .offset(-(1 as ::core::ffi::c_int as isize))
                > plimit
            {
                return FALSE;
            }
            (*value).size = DataSize;
            (*value).ptr = p as *mut ::core::ffi::c_void;
        }
        BINN_STORAGE_CONTAINER => {
            (*value).ptr = p2;
            if IsValidBinnHeader(
                p2,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                &raw mut (*value).count,
                &raw mut (*value).size,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ) == FALSE
            {
                return FALSE;
            }
        }
        _ => return FALSE,
    }
    match (*value).type_0 {
        BINN_TRUE => {
            (*value).type_0 = BINN_BOOL;
            (*value).c2rust_unnamed.vbool = TRUE as BOOL;
            (*value).ptr = &raw mut (*value).c2rust_unnamed.vbool as *mut ::core::ffi::c_void;
        }
        BINN_FALSE => {
            (*value).type_0 = BINN_BOOL;
            (*value).c2rust_unnamed.vbool = FALSE as BOOL;
            (*value).ptr = &raw mut (*value).c2rust_unnamed.vbool as *mut ::core::ffi::c_void;
        }
        _ => {}
    }
    return TRUE;
}
#[no_mangle]
pub static mut local_value: binn = binn {
    header: 0,
    allocated: 0,
    writable: 0,
    dirty: 0,
    pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    pre_allocated: 0,
    alloc_size: 0,
    used_size: 0,
    type_0: 0,
    ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    size: 0,
    count: 0,
    freefn: None,
    c2rust_unnamed: C2RustUnnamed { vint8: 0 },
    disable_int_compression: 0,
};
unsafe extern "C" fn store_value(mut value: *mut binn) -> *mut ::core::ffi::c_void {
    memcpy(
        &raw mut local_value as *mut ::core::ffi::c_void,
        value as *const ::core::ffi::c_void,
        ::core::mem::size_of::<binn>() as size_t,
    );
    match binn_get_read_storage((*value).type_0) {
        BINN_STORAGE_NOBYTES | BINN_STORAGE_WORD | BINN_STORAGE_DWORD | BINN_STORAGE_QWORD => {
            return &raw mut local_value.c2rust_unnamed.vint32 as *mut ::core::ffi::c_void;
        }
        _ => {}
    }
    return (*value).ptr;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get_value(
    mut ptr: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut binn,
) -> BOOL {
    let mut type_0: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut header_size: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut plimit: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    ptr = binn_ptr(ptr);
    if ptr.is_null() || key.is_null() || value.is_null() {
        return FALSE;
    }
    if IsValidBinnHeader(
        ptr,
        &raw mut type_0,
        &raw mut count,
        &raw mut size,
        &raw mut header_size,
    ) == FALSE
    {
        return FALSE;
    }
    if type_0 != BINN_OBJECT {
        return FALSE;
    }
    if count == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    p = ptr as *mut ::core::ffi::c_uchar;
    plimit = p
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    p = SearchForKey(p, header_size, size, count, key);
    if p.is_null() {
        return FALSE;
    }
    return GetValue(p, plimit, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get_value(
    mut ptr: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
    mut value: *mut binn,
) -> BOOL {
    let mut type_0: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut header_size: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut plimit: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    ptr = binn_ptr(ptr);
    if ptr.is_null() || value.is_null() {
        return FALSE;
    }
    if IsValidBinnHeader(
        ptr,
        &raw mut type_0,
        &raw mut count,
        &raw mut size,
        &raw mut header_size,
    ) == FALSE
    {
        return FALSE;
    }
    if type_0 != BINN_MAP {
        return FALSE;
    }
    if count == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    p = ptr as *mut ::core::ffi::c_uchar;
    plimit = p
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    p = SearchForID(p, header_size, size, count, id);
    if p.is_null() {
        return FALSE;
    }
    return GetValue(p, plimit, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_get_value(
    mut ptr: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut value: *mut binn,
) -> BOOL {
    let mut i: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut header_size: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut plimit: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut base: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    ptr = binn_ptr(ptr);
    if ptr.is_null() || value.is_null() {
        return FALSE;
    }
    if IsValidBinnHeader(
        ptr,
        &raw mut type_0,
        &raw mut count,
        &raw mut size,
        &raw mut header_size,
    ) == FALSE
    {
        return FALSE;
    }
    if type_0 != BINN_LIST {
        return FALSE;
    }
    if count == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if pos <= 0 as ::core::ffi::c_int || pos > count {
        return FALSE;
    }
    pos -= 1;
    p = ptr as *mut ::core::ffi::c_uchar;
    base = p;
    plimit = p
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as ::core::ffi::c_int;
    while i < pos {
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            return FALSE;
        }
        i += 1;
    }
    return GetValue(p, plimit, value);
}
unsafe extern "C" fn binn_read_pair(
    mut expected_type: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut pid: *mut ::core::ffi::c_int,
    mut pkey: *mut ::core::ffi::c_char,
    mut value: *mut binn,
) -> BOOL {
    let mut current_block: u64;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut header_size: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut int32: ::core::ffi::c_int = 0;
    let mut id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut counter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut plimit: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut base: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut key: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    ptr = binn_ptr(ptr);
    if IsValidBinnHeader(
        ptr,
        &raw mut type_0,
        &raw mut count,
        &raw mut size,
        &raw mut header_size,
    ) == FALSE
    {
        return FALSE;
    }
    if type_0 != expected_type
        || count == 0 as ::core::ffi::c_int
        || pos < 1 as ::core::ffi::c_int
        || pos > count
    {
        return FALSE;
    }
    p = ptr as *mut ::core::ffi::c_uchar;
    base = p;
    plimit = p
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as ::core::ffi::c_int;
    loop {
        if !(i < count) {
            current_block = 11298138898191919651;
            break;
        }
        match type_0 {
            BINN_MAP => {
                int32 = read_map_id(&raw mut p, plimit);
                if p > plimit {
                    return FALSE;
                }
                id = int32;
            }
            BINN_OBJECT => {
                len = *p;
                p = p.offset(1);
                if p > plimit {
                    return FALSE;
                }
                key = p;
                p = p.offset(len as ::core::ffi::c_int as isize);
                if p > plimit {
                    return FALSE;
                }
            }
            _ => {}
        }
        counter += 1;
        if counter == pos {
            current_block = 11959128111437687298;
            break;
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            return FALSE;
        }
        i += 1;
    }
    match current_block {
        11298138898191919651 => return FALSE,
        _ => {
            match type_0 {
                BINN_MAP => {
                    if !pid.is_null() {
                        *pid = id;
                    }
                }
                BINN_OBJECT => {
                    if !pkey.is_null() {
                        memcpy(
                            pkey as *mut ::core::ffi::c_void,
                            key as *const ::core::ffi::c_void,
                            len as size_t,
                        );
                        *pkey.offset(len as isize) = 0 as ::core::ffi::c_char;
                    }
                }
                _ => {}
            }
            return GetValue(p, plimit, value);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get_pair(
    mut ptr: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut pid: *mut ::core::ffi::c_int,
    mut value: *mut binn,
) -> BOOL {
    return binn_read_pair(
        BINN_MAP,
        ptr,
        pos,
        pid,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get_pair(
    mut ptr: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut pkey: *mut ::core::ffi::c_char,
    mut value: *mut binn,
) -> BOOL {
    return binn_read_pair(
        BINN_OBJECT,
        ptr,
        pos,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        pkey,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_pair(
    mut map: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut pid: *mut ::core::ffi::c_int,
) -> *mut binn {
    let mut value: *mut binn = ::core::ptr::null_mut::<binn>();
    value = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_read_pair(
        BINN_MAP,
        map,
        pos,
        pid,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value,
    ) == FALSE
    {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*value).allocated = TRUE as BOOL;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_pair(
    mut obj: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut pkey: *mut ::core::ffi::c_char,
) -> *mut binn {
    let mut value: *mut binn = ::core::ptr::null_mut::<binn>();
    value = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_read_pair(
        BINN_OBJECT,
        obj,
        pos,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        pkey,
        value,
    ) == FALSE
    {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*value).allocated = TRUE as BOOL;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read_pair(
    mut ptr: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut pid: *mut ::core::ffi::c_int,
    mut ptype: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_get_pair(ptr, pos, pid, &raw mut value) == FALSE {
        return NULL;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&raw mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read_pair(
    mut ptr: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut pkey: *mut ::core::ffi::c_char,
    mut ptype: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_get_pair(ptr, pos, pkey, &raw mut value) == FALSE {
        return NULL;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&raw mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_iter_init(
    mut iter: *mut binn_iter,
    mut ptr: *const ::core::ffi::c_void,
    mut expected_type: ::core::ffi::c_int,
) -> BOOL {
    let mut type_0: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut header_size: ::core::ffi::c_int = 0;
    ptr = binn_ptr(ptr);
    if ptr.is_null() || iter.is_null() {
        return FALSE;
    }
    memset(
        iter as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<binn_iter>() as size_t,
    );
    if IsValidBinnHeader(
        ptr,
        &raw mut type_0,
        &raw mut count,
        &raw mut size,
        &raw mut header_size,
    ) == FALSE
    {
        return FALSE;
    }
    if type_0 != expected_type {
        return FALSE;
    }
    (*iter).plimit = (ptr as *mut ::core::ffi::c_uchar)
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    (*iter).pnext = (ptr as *mut ::core::ffi::c_uchar).offset(header_size as isize);
    (*iter).count = count;
    (*iter).current = 0 as ::core::ffi::c_int;
    (*iter).type_0 = type_0;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_next(mut iter: *mut binn_iter, mut value: *mut binn) -> BOOL {
    let mut pnow: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if iter.is_null()
        || (*iter).pnext.is_null()
        || (*iter).pnext > (*iter).plimit
        || (*iter).current > (*iter).count
        || (*iter).type_0 != BINN_LIST
    {
        return FALSE;
    }
    (*iter).current += 1;
    if (*iter).current > (*iter).count {
        return FALSE;
    }
    pnow = (*iter).pnext;
    (*iter).pnext = AdvanceDataPos(pnow, (*iter).plimit);
    if !(*iter).pnext.is_null() && (*iter).pnext < pnow {
        return FALSE;
    }
    return GetValue(pnow, (*iter).plimit, value);
}
unsafe extern "C" fn binn_read_next_pair(
    mut expected_type: ::core::ffi::c_int,
    mut iter: *mut binn_iter,
    mut pid: *mut ::core::ffi::c_int,
    mut pkey: *mut ::core::ffi::c_char,
    mut value: *mut binn,
) -> BOOL {
    let mut int32: ::core::ffi::c_int = 0;
    let mut id: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut key: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: ::core::ffi::c_ushort = 0;
    if iter.is_null()
        || (*iter).pnext.is_null()
        || (*iter).pnext > (*iter).plimit
        || (*iter).current > (*iter).count
        || (*iter).type_0 != expected_type
    {
        return FALSE;
    }
    (*iter).current += 1;
    if (*iter).current > (*iter).count {
        return FALSE;
    }
    p = (*iter).pnext;
    match expected_type {
        BINN_MAP => {
            int32 = read_map_id(&raw mut p, (*iter).plimit);
            if p > (*iter).plimit {
                return FALSE;
            }
            id = int32;
            if !pid.is_null() {
                *pid = id;
            }
        }
        BINN_OBJECT => {
            len = *p as ::core::ffi::c_ushort;
            p = p.offset(1);
            key = p;
            p = p.offset(len as ::core::ffi::c_int as isize);
            if p > (*iter).plimit {
                return FALSE;
            }
            if !pkey.is_null() {
                memcpy(
                    pkey as *mut ::core::ffi::c_void,
                    key as *const ::core::ffi::c_void,
                    len as size_t,
                );
                *pkey.offset(len as isize) = 0 as ::core::ffi::c_char;
            }
        }
        _ => {}
    }
    (*iter).pnext = AdvanceDataPos(p, (*iter).plimit);
    if !(*iter).pnext.is_null() && (*iter).pnext < p {
        return FALSE;
    }
    return GetValue(p, (*iter).plimit, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_next(
    mut iter: *mut binn_iter,
    mut pid: *mut ::core::ffi::c_int,
    mut value: *mut binn,
) -> BOOL {
    return binn_read_next_pair(
        BINN_MAP,
        iter,
        pid,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_next(
    mut iter: *mut binn_iter,
    mut pkey: *mut ::core::ffi::c_char,
    mut value: *mut binn,
) -> BOOL {
    return binn_read_next_pair(
        BINN_OBJECT,
        iter,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        pkey,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_next_value(mut iter: *mut binn_iter) -> *mut binn {
    let mut value: *mut binn = ::core::ptr::null_mut::<binn>();
    value = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_list_next(iter, value) == FALSE {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*value).allocated = TRUE as BOOL;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_next_value(
    mut iter: *mut binn_iter,
    mut pid: *mut ::core::ffi::c_int,
) -> *mut binn {
    let mut value: *mut binn = ::core::ptr::null_mut::<binn>();
    value = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_map_next(iter, pid, value) == FALSE {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*value).allocated = TRUE as BOOL;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_next_value(
    mut iter: *mut binn_iter,
    mut pkey: *mut ::core::ffi::c_char,
) -> *mut binn {
    let mut value: *mut binn = ::core::ptr::null_mut::<binn>();
    value = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_object_next(iter, pkey, value) == FALSE {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*value).allocated = TRUE as BOOL;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_read_next(
    mut iter: *mut binn_iter,
    mut ptype: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_list_next(iter, &raw mut value) == FALSE {
        return NULL;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&raw mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read_next(
    mut iter: *mut binn_iter,
    mut pid: *mut ::core::ffi::c_int,
    mut ptype: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_next(iter, pid, &raw mut value) == FALSE {
        return NULL;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&raw mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read_next(
    mut iter: *mut binn_iter,
    mut pkey: *mut ::core::ffi::c_char,
    mut ptype: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_next(iter, pkey, &raw mut value) == FALSE {
        return NULL;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&raw mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_write_storage(
    mut type_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut storage_type: ::core::ffi::c_int = 0;
    match type_0 {
        BINN_SINGLE_STR | BINN_DOUBLE_STR => return BINN_STORAGE_STRING,
        BINN_BOOL => return BINN_STORAGE_NOBYTES,
        _ => {
            binn_get_type_info(
                type_0,
                &raw mut storage_type,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            return storage_type;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_read_storage(
    mut type_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut storage_type: ::core::ffi::c_int = 0;
    match type_0 {
        BINN_BOOL | BINN_TRUE | BINN_FALSE => return BINN_STORAGE_DWORD,
        _ => {
            binn_get_type_info(
                type_0,
                &raw mut storage_type,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
            return storage_type;
        }
    };
}
unsafe extern "C" fn GetWriteConvertedData(
    mut ptype: *mut ::core::ffi::c_int,
    mut ppvalue: *mut *mut ::core::ffi::c_void,
    mut psize: *mut ::core::ffi::c_int,
) -> BOOL {
    let mut type_0: ::core::ffi::c_int = 0;
    let mut f1: ::core::ffi::c_float = 0.;
    let mut d1: ::core::ffi::c_double = 0.;
    let mut pstr: [::core::ffi::c_char; 128] = [0; 128];
    type_0 = *ptype;
    if (*ppvalue).is_null() {
        let mut current_block_4: u64;
        match type_0 {
            BINN_NULL | BINN_TRUE | BINN_FALSE => {
                current_block_4 = 13513818773234778473;
            }
            BINN_STRING | BINN_BLOB => {
                if *psize == 0 as ::core::ffi::c_int {
                    current_block_4 = 13513818773234778473;
                } else {
                    current_block_4 = 15132177437536745126;
                }
            }
            _ => {
                current_block_4 = 15132177437536745126;
            }
        }
        match current_block_4 {
            15132177437536745126 => return FALSE,
            _ => {}
        }
    }
    match type_0 {
        BINN_DECIMAL | BINN_CURRENCYSTR => return TRUE,
        BINN_DATE | BINN_DATETIME | BINN_TIME => return TRUE,
        BINN_BOOL => {
            if **(ppvalue as *mut *mut BOOL) == FALSE {
                type_0 = BINN_FALSE;
            } else {
                type_0 = BINN_TRUE;
            }
            *ptype = type_0;
        }
        _ => {}
    }
    return TRUE;
}
unsafe extern "C" fn type_family(mut type_0: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match type_0 {
        BINN_LIST | BINN_MAP | BINN_OBJECT => return BINN_FAMILY_BINN,
        BINN_INT8 | BINN_INT16 | BINN_INT32 | BINN_INT64 | BINN_UINT8 | BINN_UINT16
        | BINN_UINT32 | BINN_UINT64 => return BINN_FAMILY_INT,
        BINN_FLOAT32 | BINN_FLOAT64 | BINN_SINGLE_STR | BINN_DOUBLE_STR => {
            return BINN_FAMILY_FLOAT;
        }
        BINN_STRING | BINN_HTML | BINN_CSS | BINN_XML | BINN_JSON | BINN_JAVASCRIPT => {
            return BINN_FAMILY_STRING;
        }
        BINN_BLOB | BINN_JPEG | BINN_GIF | BINN_PNG | BINN_BMP => return BINN_FAMILY_BLOB,
        BINN_DECIMAL | BINN_CURRENCY | BINN_DATE | BINN_TIME | BINN_DATETIME => {
            return BINN_FAMILY_STRING;
        }
        BINN_BOOL => return BINN_FAMILY_BOOL,
        BINN_NULL => return BINN_FAMILY_NULL,
        _ => return BINN_FAMILY_NONE,
    };
}
unsafe extern "C" fn int_type(mut type_0: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match type_0 {
        BINN_INT8 | BINN_INT16 | BINN_INT32 | BINN_INT64 => return BINN_SIGNED_INT,
        BINN_UINT8 | BINN_UINT16 | BINN_UINT32 | BINN_UINT64 => return BINN_UNSIGNED_INT,
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn copy_raw_value(
    mut psource: *const ::core::ffi::c_void,
    mut pdest: *mut ::core::ffi::c_void,
    mut data_store: ::core::ffi::c_int,
) -> BOOL {
    match data_store {
        BINN_STORAGE_NOBYTES => {}
        BINN_STORAGE_BYTE => {
            *(pdest as *mut ::core::ffi::c_char) = *(psource as *mut ::core::ffi::c_char);
        }
        BINN_STORAGE_WORD => {
            *(pdest as *mut ::core::ffi::c_short) = *(psource as *mut ::core::ffi::c_short);
        }
        BINN_STORAGE_DWORD => {
            *(pdest as *mut ::core::ffi::c_int) = *(psource as *mut ::core::ffi::c_int);
        }
        BINN_STORAGE_QWORD => {
            *(pdest as *mut uint64) = *(psource as *mut uint64);
        }
        BINN_STORAGE_BLOB | BINN_STORAGE_STRING | BINN_STORAGE_CONTAINER => {
            let ref mut fresh18 = *(pdest as *mut *mut ::core::ffi::c_char);
            *fresh18 = psource as *mut ::core::ffi::c_char;
        }
        _ => return FALSE,
    }
    return TRUE;
}
unsafe extern "C" fn copy_int_value(
    mut psource: *const ::core::ffi::c_void,
    mut pdest: *mut ::core::ffi::c_void,
    mut source_type: ::core::ffi::c_int,
    mut dest_type: ::core::ffi::c_int,
) -> BOOL {
    let mut vuint64: uint64 = 0 as uint64;
    let mut vint64: int64 = 0 as int64;
    match source_type {
        BINN_INT8 => {
            vint64 = *(psource as *mut ::core::ffi::c_schar) as int64;
        }
        BINN_INT16 => {
            vint64 = *(psource as *mut ::core::ffi::c_short) as int64;
        }
        BINN_INT32 => {
            vint64 = *(psource as *mut ::core::ffi::c_int) as int64;
        }
        BINN_INT64 => {
            vint64 = *(psource as *mut int64);
        }
        BINN_UINT8 => {
            vuint64 = *(psource as *mut ::core::ffi::c_uchar) as uint64;
        }
        BINN_UINT16 => {
            vuint64 = *(psource as *mut ::core::ffi::c_ushort) as uint64;
        }
        BINN_UINT32 => {
            vuint64 = *(psource as *mut ::core::ffi::c_uint) as uint64;
        }
        BINN_UINT64 => {
            vuint64 = *(psource as *mut uint64);
        }
        _ => return FALSE,
    }
    if int_type(source_type) == BINN_UNSIGNED_INT && int_type(dest_type) == BINN_SIGNED_INT {
        if vuint64 > INT64_MAX as ::core::ffi::c_ulonglong {
            return FALSE;
        }
        vint64 = vuint64 as int64;
    } else if int_type(source_type) == BINN_SIGNED_INT && int_type(dest_type) == BINN_UNSIGNED_INT {
        if vint64 < 0 as ::core::ffi::c_longlong {
            return FALSE;
        }
        vuint64 = vint64 as uint64;
    }
    match dest_type {
        BINN_INT8 => {
            if vint64 < INT8_MIN as ::core::ffi::c_longlong
                || vint64 > INT8_MAX as ::core::ffi::c_longlong
            {
                return FALSE;
            }
            *(pdest as *mut ::core::ffi::c_schar) = vint64 as ::core::ffi::c_schar;
        }
        BINN_INT16 => {
            if vint64 < INT16_MIN as ::core::ffi::c_longlong
                || vint64 > INT16_MAX as ::core::ffi::c_longlong
            {
                return FALSE;
            }
            *(pdest as *mut ::core::ffi::c_short) = vint64 as ::core::ffi::c_short;
        }
        BINN_INT32 => {
            if vint64 < INT32_MIN as ::core::ffi::c_longlong
                || vint64 > INT32_MAX as ::core::ffi::c_longlong
            {
                return FALSE;
            }
            *(pdest as *mut ::core::ffi::c_int) = vint64 as ::core::ffi::c_int;
        }
        BINN_INT64 => {
            *(pdest as *mut int64) = vint64;
        }
        BINN_UINT8 => {
            if vuint64 > UINT8_MAX as ::core::ffi::c_ulonglong {
                return FALSE;
            }
            *(pdest as *mut ::core::ffi::c_uchar) = vuint64 as ::core::ffi::c_uchar;
        }
        BINN_UINT16 => {
            if vuint64 > UINT16_MAX as ::core::ffi::c_ulonglong {
                return FALSE;
            }
            *(pdest as *mut ::core::ffi::c_ushort) = vuint64 as ::core::ffi::c_ushort;
        }
        BINN_UINT32 => {
            if vuint64 > UINT32_MAX as ::core::ffi::c_ulonglong {
                return FALSE;
            }
            *(pdest as *mut ::core::ffi::c_uint) = vuint64 as ::core::ffi::c_uint;
        }
        BINN_UINT64 => {
            *(pdest as *mut uint64) = vuint64;
        }
        _ => return FALSE,
    }
    return TRUE;
}
unsafe extern "C" fn copy_float_value(
    mut psource: *const ::core::ffi::c_void,
    mut pdest: *mut ::core::ffi::c_void,
    mut source_type: ::core::ffi::c_int,
    mut dest_type: ::core::ffi::c_int,
) -> BOOL {
    match source_type {
        BINN_FLOAT32 => {
            *(pdest as *mut ::core::ffi::c_double) =
                *(psource as *mut ::core::ffi::c_float) as ::core::ffi::c_double;
        }
        BINN_FLOAT64 => {
            *(pdest as *mut ::core::ffi::c_float) =
                *(psource as *mut ::core::ffi::c_double) as ::core::ffi::c_float;
        }
        _ => return FALSE,
    }
    return TRUE;
}
unsafe extern "C" fn zero_value(
    mut pvalue: *const ::core::ffi::c_void,
    mut type_0: ::core::ffi::c_int,
) {
    match binn_get_read_storage(type_0) {
        BINN_STORAGE_BYTE => {
            *(pvalue as *mut ::core::ffi::c_char) = 0 as ::core::ffi::c_char;
        }
        BINN_STORAGE_WORD => {
            *(pvalue as *mut ::core::ffi::c_short) = 0 as ::core::ffi::c_short;
        }
        BINN_STORAGE_DWORD => {
            *(pvalue as *mut ::core::ffi::c_int) = 0 as ::core::ffi::c_int;
        }
        BINN_STORAGE_QWORD => {
            *(pvalue as *mut uint64) = 0 as uint64;
        }
        BINN_STORAGE_BLOB | BINN_STORAGE_STRING | BINN_STORAGE_CONTAINER => {
            let ref mut fresh19 = *(pvalue as *mut *mut ::core::ffi::c_char);
            *fresh19 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        BINN_STORAGE_NOBYTES | _ => {}
    };
}
unsafe extern "C" fn copy_value(
    mut psource: *mut ::core::ffi::c_void,
    mut pdest: *mut ::core::ffi::c_void,
    mut source_type: ::core::ffi::c_int,
    mut dest_type: ::core::ffi::c_int,
    mut data_store: ::core::ffi::c_int,
) -> BOOL {
    if type_family(source_type) != type_family(dest_type) {
        return FALSE;
    }
    if type_family(source_type) == BINN_FAMILY_INT && source_type != dest_type {
        return copy_int_value(psource, pdest, source_type, dest_type);
    } else if type_family(source_type) == BINN_FAMILY_FLOAT && source_type != dest_type {
        return copy_float_value(psource, pdest, source_type, dest_type);
    } else {
        return copy_raw_value(psource, pdest, data_store);
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_add(
    mut list: *mut binn,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> BOOL {
    if GetWriteConvertedData(&raw mut type_0, &raw mut pvalue, &raw mut size) == FALSE {
        return FALSE;
    }
    return binn_list_add_raw(list, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_set(
    mut map: *mut binn,
    mut id: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> BOOL {
    if GetWriteConvertedData(&raw mut type_0, &raw mut pvalue, &raw mut size) == FALSE {
        return FALSE;
    }
    return binn_map_set_raw(map, id, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_set(
    mut obj: *mut binn,
    mut key: *const ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> BOOL {
    if GetWriteConvertedData(&raw mut type_0, &raw mut pvalue, &raw mut size) == FALSE {
        return FALSE;
    }
    return binn_object_set_raw(obj, key, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_add_value(
    mut item: *mut binn,
    mut binn_type_0: ::core::ffi::c_int,
    mut id: ::core::ffi::c_int,
    mut name: *mut ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
) -> BOOL {
    match binn_type_0 {
        BINN_LIST => return binn_list_add(item, type_0, pvalue, size),
        BINN_MAP => return binn_map_set(item, id, type_0, pvalue, size),
        BINN_OBJECT => return binn_object_set(item, name, type_0, pvalue, size),
        _ => return FALSE,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_add_new(mut list: *mut binn, mut value: *mut binn) -> BOOL {
    let mut retval: BOOL = 0;
    retval = binn_list_add_value(list, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_set_new(
    mut map: *mut binn,
    mut id: ::core::ffi::c_int,
    mut value: *mut binn,
) -> BOOL {
    let mut retval: BOOL = 0;
    retval = binn_map_set_value(map, id, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_set_new(
    mut obj: *mut binn,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut binn,
) -> BOOL {
    let mut retval: BOOL = 0;
    retval = binn_object_set_value(obj, key, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_value(
    mut ptr: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> *mut binn {
    let mut value: *mut binn = ::core::ptr::null_mut::<binn>();
    value = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_list_get_value(ptr, pos, value) == FALSE {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*value).allocated = TRUE as BOOL;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_value(
    mut ptr: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> *mut binn {
    let mut value: *mut binn = ::core::ptr::null_mut::<binn>();
    value = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_map_get_value(ptr, id, value) == FALSE {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*value).allocated = TRUE as BOOL;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_value(
    mut ptr: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> *mut binn {
    let mut value: *mut binn = ::core::ptr::null_mut::<binn>();
    value = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if binn_object_get_value(ptr, key, value) == FALSE {
        free_fn.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<binn>();
    }
    (*value).allocated = TRUE as BOOL;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_read(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut ptype: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_list_get_value(list, pos, &raw mut value) == FALSE {
        return NULL;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&raw mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
    mut ptype: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_get_value(map, id, &raw mut value) == FALSE {
        return NULL;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&raw mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
    mut ptype: *mut ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_get_value(obj, key, &raw mut value) == FALSE {
        return NULL;
    }
    if !ptype.is_null() {
        *ptype = value.type_0;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return store_value(&raw mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_get(
    mut ptr: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut psize: *mut ::core::ffi::c_int,
) -> BOOL {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: ::core::ffi::c_int = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != BINN_STORAGE_NOBYTES && pvalue.is_null() {
        return FALSE;
    }
    zero_value(pvalue, type_0);
    if binn_list_get_value(ptr, pos, &raw mut value) == FALSE {
        return FALSE;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type) == FALSE {
        return FALSE;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get(
    mut ptr: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut psize: *mut ::core::ffi::c_int,
) -> BOOL {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: ::core::ffi::c_int = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != BINN_STORAGE_NOBYTES && pvalue.is_null() {
        return FALSE;
    }
    zero_value(pvalue, type_0);
    if binn_map_get_value(ptr, id, &raw mut value) == FALSE {
        return FALSE;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type) == FALSE {
        return FALSE;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get(
    mut ptr: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut psize: *mut ::core::ffi::c_int,
) -> BOOL {
    let mut value: binn = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: ::core::ffi::c_int = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != BINN_STORAGE_NOBYTES && pvalue.is_null() {
        return FALSE;
    }
    zero_value(pvalue, type_0);
    if binn_object_get_value(ptr, key, &raw mut value) == FALSE {
        return FALSE;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type) == FALSE {
        return FALSE;
    }
    if !psize.is_null() {
        *psize = value.size;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int8(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_schar {
    let mut value: ::core::ffi::c_schar = 0;
    binn_list_get(
        list,
        pos,
        BINN_INT8,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int16(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_short {
    let mut value: ::core::ffi::c_short = 0;
    binn_list_get(
        list,
        pos,
        BINN_INT16,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int32(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_int = 0;
    binn_list_get(
        list,
        pos,
        BINN_INT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int64(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> int64 {
    let mut value: int64 = 0;
    binn_list_get(
        list,
        pos,
        BINN_INT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint8(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut value: ::core::ffi::c_uchar = 0;
    binn_list_get(
        list,
        pos,
        BINN_UINT8,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint16(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_ushort {
    let mut value: ::core::ffi::c_ushort = 0;
    binn_list_get(
        list,
        pos,
        BINN_UINT16,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint32(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut value: ::core::ffi::c_uint = 0;
    binn_list_get(
        list,
        pos,
        BINN_UINT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint64(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> uint64 {
    let mut value: uint64 = 0;
    binn_list_get(
        list,
        pos,
        BINN_UINT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_float(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut value: ::core::ffi::c_float = 0.;
    binn_list_get(
        list,
        pos,
        BINN_FLOAT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_double(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    binn_list_get(
        list,
        pos,
        BINN_FLOAT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_bool(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> BOOL {
    let mut value: BOOL = 0;
    binn_list_get(
        list,
        pos,
        BINN_BOOL,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_null(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> BOOL {
    return binn_list_get(
        list,
        pos,
        BINN_NULL,
        NULL,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_str(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    binn_list_get(
        list,
        pos,
        BINN_STRING,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_blob(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_list_get(
        list,
        pos,
        BINN_BLOB,
        &raw mut value as *mut ::core::ffi::c_void,
        psize,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_list(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_list_get(
        list,
        pos,
        BINN_LIST,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_map(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_list_get(
        list,
        pos,
        BINN_MAP,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_object(
    mut list: *const ::core::ffi::c_void,
    mut pos: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_list_get(
        list,
        pos,
        BINN_OBJECT,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int8(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_schar {
    let mut value: ::core::ffi::c_schar = 0;
    binn_map_get(
        map,
        id,
        BINN_INT8,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int16(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_short {
    let mut value: ::core::ffi::c_short = 0;
    binn_map_get(
        map,
        id,
        BINN_INT16,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int32(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_int = 0;
    binn_map_get(
        map,
        id,
        BINN_INT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int64(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> int64 {
    let mut value: int64 = 0;
    binn_map_get(
        map,
        id,
        BINN_INT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint8(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_uchar {
    let mut value: ::core::ffi::c_uchar = 0;
    binn_map_get(
        map,
        id,
        BINN_UINT8,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint16(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_ushort {
    let mut value: ::core::ffi::c_ushort = 0;
    binn_map_get(
        map,
        id,
        BINN_UINT16,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint32(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut value: ::core::ffi::c_uint = 0;
    binn_map_get(
        map,
        id,
        BINN_UINT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint64(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> uint64 {
    let mut value: uint64 = 0;
    binn_map_get(
        map,
        id,
        BINN_UINT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_float(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut value: ::core::ffi::c_float = 0.;
    binn_map_get(
        map,
        id,
        BINN_FLOAT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_double(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    binn_map_get(
        map,
        id,
        BINN_FLOAT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_bool(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> BOOL {
    let mut value: BOOL = 0;
    binn_map_get(
        map,
        id,
        BINN_BOOL,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_null(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> BOOL {
    return binn_map_get(
        map,
        id,
        BINN_NULL,
        NULL,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_str(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    binn_map_get(
        map,
        id,
        BINN_STRING,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_blob(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_map_get(
        map,
        id,
        BINN_BLOB,
        &raw mut value as *mut ::core::ffi::c_void,
        psize,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_list(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_map_get(
        map,
        id,
        BINN_LIST,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_map(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_map_get(
        map,
        id,
        BINN_MAP,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_object(
    mut map: *const ::core::ffi::c_void,
    mut id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_map_get(
        map,
        id,
        BINN_OBJECT,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int8(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_schar {
    let mut value: ::core::ffi::c_schar = 0;
    binn_object_get(
        obj,
        key,
        BINN_INT8,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int16(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_short {
    let mut value: ::core::ffi::c_short = 0;
    binn_object_get(
        obj,
        key,
        BINN_INT16,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int32(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_int = 0;
    binn_object_get(
        obj,
        key,
        BINN_INT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int64(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> int64 {
    let mut value: int64 = 0;
    binn_object_get(
        obj,
        key,
        BINN_INT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint8(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uchar {
    let mut value: ::core::ffi::c_uchar = 0;
    binn_object_get(
        obj,
        key,
        BINN_UINT8,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint16(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_ushort {
    let mut value: ::core::ffi::c_ushort = 0;
    binn_object_get(
        obj,
        key,
        BINN_UINT16,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint32(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut value: ::core::ffi::c_uint = 0;
    binn_object_get(
        obj,
        key,
        BINN_UINT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint64(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> uint64 {
    let mut value: uint64 = 0;
    binn_object_get(
        obj,
        key,
        BINN_UINT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_float(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_float {
    let mut value: ::core::ffi::c_float = 0.;
    binn_object_get(
        obj,
        key,
        BINN_FLOAT32,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_double(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.;
    binn_object_get(
        obj,
        key,
        BINN_FLOAT64,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_bool(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> BOOL {
    let mut value: BOOL = 0;
    binn_object_get(
        obj,
        key,
        BINN_BOOL,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_null(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> BOOL {
    return binn_object_get(
        obj,
        key,
        BINN_NULL,
        NULL,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_str(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    binn_object_get(
        obj,
        key,
        BINN_STRING,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_blob(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
    mut psize: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_object_get(
        obj,
        key,
        BINN_BLOB,
        &raw mut value as *mut ::core::ffi::c_void,
        psize,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_list(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_object_get(
        obj,
        key,
        BINN_LIST,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_map(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_object_get(
        obj,
        key,
        BINN_MAP,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_object(
    mut obj: *const ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut value: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    binn_object_get(
        obj,
        key,
        BINN_OBJECT,
        &raw mut value as *mut ::core::ffi::c_void,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    return value;
}
unsafe extern "C" fn binn_alloc_item() -> *mut binn {
    let mut item: *mut binn = ::core::ptr::null_mut::<binn>();
    item = binn_malloc(::core::mem::size_of::<binn>() as ::core::ffi::c_int) as *mut binn;
    if !item.is_null() {
        memset(
            item as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<binn>() as size_t,
        );
        (*item).header = BINN_MAGIC;
        (*item).allocated = TRUE as BOOL;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_value(
    mut type_0: ::core::ffi::c_int,
    mut pvalue: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
    mut freefn: binn_mem_free,
) -> *mut binn {
    let mut storage_type: ::core::ffi::c_int = 0;
    let mut item: *mut binn = binn_alloc_item();
    if !item.is_null() {
        (*item).type_0 = type_0;
        binn_get_type_info(
            type_0,
            &raw mut storage_type,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        let mut current_block_19: u64;
        match storage_type {
            BINN_STORAGE_NOBYTES => {
                current_block_19 = 2370887241019905314;
            }
            BINN_STORAGE_STRING => {
                if size == 0 as ::core::ffi::c_int {
                    size = strlen(pvalue as *mut ::core::ffi::c_char).wrapping_add(1 as size_t)
                        as ::core::ffi::c_int;
                }
                current_block_19 = 9960047145181982405;
            }
            BINN_STORAGE_BLOB | BINN_STORAGE_CONTAINER => {
                current_block_19 = 9960047145181982405;
            }
            _ => {
                (*item).ptr = &raw mut (*item).c2rust_unnamed.vint32 as *mut ::core::ffi::c_void;
                copy_raw_value(pvalue, (*item).ptr, storage_type);
                current_block_19 = 2370887241019905314;
            }
        }
        match current_block_19 {
            9960047145181982405 => {
                if freefn
                    == ::core::mem::transmute::<::libc::intptr_t, binn_mem_free>(
                        -(1 as ::core::ffi::c_int) as ::libc::intptr_t,
                    )
                {
                    (*item).ptr = binn_memdup(pvalue, size);
                    if (*item).ptr.is_null() {
                        free_fn.expect("non-null function pointer")(
                            item as *mut ::core::ffi::c_void,
                        );
                        return ::core::ptr::null_mut::<binn>();
                    }
                    (*item).freefn = free_fn as binn_mem_free;
                } else {
                    (*item).ptr = pvalue;
                    (*item).freefn = freefn;
                }
                if storage_type == BINN_STORAGE_STRING {
                    size -= 1;
                }
                (*item).size = size;
            }
            _ => {}
        }
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_string(
    mut item: *mut binn,
    mut str: *mut ::core::ffi::c_char,
    mut pfree: binn_mem_free,
) -> BOOL {
    if item.is_null() || str.is_null() {
        return FALSE;
    }
    if pfree
        == ::core::mem::transmute::<::libc::intptr_t, binn_mem_free>(
            -(1 as ::core::ffi::c_int) as ::libc::intptr_t,
        )
    {
        (*item).ptr = binn_memdup(
            str as *const ::core::ffi::c_void,
            strlen(str).wrapping_add(1 as size_t) as ::core::ffi::c_int,
        );
        if (*item).ptr.is_null() {
            return FALSE;
        }
        (*item).freefn = free_fn as binn_mem_free;
    } else {
        (*item).ptr = str as *mut ::core::ffi::c_void;
        (*item).freefn = pfree;
    }
    (*item).type_0 = BINN_STRING;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_blob(
    mut item: *mut binn,
    mut ptr: *mut ::core::ffi::c_void,
    mut size: ::core::ffi::c_int,
    mut pfree: binn_mem_free,
) -> BOOL {
    if item.is_null() || ptr.is_null() {
        return FALSE;
    }
    if pfree
        == ::core::mem::transmute::<::libc::intptr_t, binn_mem_free>(
            -(1 as ::core::ffi::c_int) as ::libc::intptr_t,
        )
    {
        (*item).ptr = binn_memdup(ptr, size);
        if (*item).ptr.is_null() {
            return FALSE;
        }
        (*item).freefn = free_fn as binn_mem_free;
    } else {
        (*item).ptr = ptr;
        (*item).freefn = pfree;
    }
    (*item).type_0 = BINN_BLOB;
    (*item).size = size;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn atoi64(mut str: *mut ::core::ffi::c_char) -> int64 {
    let mut retval: int64 = 0;
    let mut is_negative: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *str as ::core::ffi::c_int == '-' as i32 {
        is_negative = 1 as ::core::ffi::c_int;
        str = str.offset(1);
    }
    retval = 0 as int64;
    while *str != 0 {
        retval = (10 as ::core::ffi::c_longlong * retval as ::core::ffi::c_longlong
            + (*str as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_longlong)
            as int64;
        str = str.offset(1);
    }
    if is_negative != 0 {
        retval *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_longlong;
    }
    return retval;
}
unsafe extern "C" fn is_integer(mut p: *mut ::core::ffi::c_char) -> BOOL {
    let mut retval: BOOL = 0;
    if p.is_null() {
        return FALSE;
    }
    if *p as ::core::ffi::c_int == '-' as i32 {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    retval = TRUE as BOOL;
    while *p != 0 {
        if (*p as ::core::ffi::c_int) < '0' as i32 || *p as ::core::ffi::c_int > '9' as i32 {
            retval = FALSE as BOOL;
        }
        p = p.offset(1);
    }
    return retval;
}
unsafe extern "C" fn is_float(mut p: *mut ::core::ffi::c_char) -> BOOL {
    let mut retval: BOOL = 0;
    let mut number_found: BOOL = FALSE;
    if p.is_null() {
        return FALSE;
    }
    if *p as ::core::ffi::c_int == '-' as i32 {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    retval = TRUE as BOOL;
    while *p != 0 {
        if *p as ::core::ffi::c_int == '.' as i32 || *p as ::core::ffi::c_int == ',' as i32 {
            if number_found == 0 {
                retval = FALSE as BOOL;
            }
        } else if *p as ::core::ffi::c_int >= '0' as i32 && *p as ::core::ffi::c_int <= '9' as i32 {
            number_found = TRUE as BOOL;
        } else {
            return FALSE;
        }
        p = p.offset(1);
    }
    return retval;
}
unsafe extern "C" fn is_bool_str(mut str: *mut ::core::ffi::c_char, mut pbool: *mut BOOL) -> BOOL {
    let mut vint: int64 = 0;
    let mut vdouble: ::core::ffi::c_double = 0.;
    if str.is_null() || pbool.is_null() {
        return FALSE;
    }
    if !(strcasecmp(str, b"true\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int)
    {
        if !(strcasecmp(str, b"yes\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int)
        {
            if !(strcasecmp(str, b"on\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int)
            {
                if !(strcasecmp(str, b"false\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int)
                {
                    if !(strcasecmp(str, b"no\0" as *const u8 as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int)
                    {
                        if !(strcasecmp(str, b"off\0" as *const u8 as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int)
                        {
                            if is_integer(str) != 0 {
                                vint = atoi64(str);
                                *pbool = (if vint != 0 as ::core::ffi::c_longlong {
                                    TRUE
                                } else {
                                    FALSE
                                }) as BOOL;
                                return TRUE;
                            } else if is_float(str) != 0 {
                                vdouble = atof(str);
                                *pbool = (if vdouble
                                    != 0 as ::core::ffi::c_int as ::core::ffi::c_double
                                {
                                    TRUE
                                } else {
                                    FALSE
                                }) as BOOL;
                                return TRUE;
                            }
                            return FALSE;
                        }
                    }
                }
                *pbool = FALSE as BOOL;
                return TRUE;
            }
        }
    }
    *pbool = TRUE as BOOL;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_int32(
    mut value: *mut binn,
    mut pint: *mut ::core::ffi::c_int,
) -> BOOL {
    if value.is_null() || pint.is_null() {
        return FALSE;
    }
    if type_family((*value).type_0) == BINN_FAMILY_INT {
        return copy_int_value(
            (*value).ptr,
            pint as *mut ::core::ffi::c_void,
            (*value).type_0,
            BINN_INT32,
        );
    }
    match (*value).type_0 {
        BINN_FLOAT => {
            if (*value).c2rust_unnamed.vfloat < INT32_MIN as ::core::ffi::c_float
                || (*value).c2rust_unnamed.vfloat > INT32_MAX as ::core::ffi::c_float
            {
                return FALSE;
            }
            *pint = if (*value).c2rust_unnamed.vfloat as ::core::ffi::c_double >= 0.0f64 {
                ((*value).c2rust_unnamed.vfloat as ::core::ffi::c_double + 0.5f64)
                    as ::core::ffi::c_int
            } else if (*value).c2rust_unnamed.vfloat as ::core::ffi::c_double
                - (*value).c2rust_unnamed.vfloat as ::core::ffi::c_int as ::core::ffi::c_double
                <= -0.5f64
            {
                (*value).c2rust_unnamed.vfloat as ::core::ffi::c_int
            } else {
                ((*value).c2rust_unnamed.vfloat as ::core::ffi::c_double - 0.5f64)
                    as ::core::ffi::c_int
            };
        }
        BINN_DOUBLE => {
            if (*value).c2rust_unnamed.vdouble < INT32_MIN as ::core::ffi::c_double
                || (*value).c2rust_unnamed.vdouble > INT32_MAX as ::core::ffi::c_double
            {
                return FALSE;
            }
            *pint = if (*value).c2rust_unnamed.vdouble >= 0.0f64 {
                ((*value).c2rust_unnamed.vdouble + 0.5f64) as ::core::ffi::c_int
            } else if (*value).c2rust_unnamed.vdouble
                - (*value).c2rust_unnamed.vdouble as ::core::ffi::c_int as ::core::ffi::c_double
                <= -0.5f64
            {
                (*value).c2rust_unnamed.vdouble as ::core::ffi::c_int
            } else {
                ((*value).c2rust_unnamed.vdouble - 0.5f64) as ::core::ffi::c_int
            };
        }
        BINN_STRING => {
            if is_integer((*value).ptr as *mut ::core::ffi::c_char) != 0 {
                *pint = atoi((*value).ptr as *mut ::core::ffi::c_char);
            } else if is_float((*value).ptr as *mut ::core::ffi::c_char) != 0 {
                *pint = if atof((*value).ptr as *mut ::core::ffi::c_char) >= 0.0f64 {
                    (atof((*value).ptr as *mut ::core::ffi::c_char) + 0.5f64) as ::core::ffi::c_int
                } else if atof((*value).ptr as *mut ::core::ffi::c_char)
                    - atof((*value).ptr as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                        as ::core::ffi::c_double
                    <= -0.5f64
                {
                    atof((*value).ptr as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                } else {
                    (atof((*value).ptr as *mut ::core::ffi::c_char) - 0.5f64) as ::core::ffi::c_int
                };
            } else {
                return FALSE;
            }
        }
        BINN_BOOL => {
            *pint = (*value).c2rust_unnamed.vbool as ::core::ffi::c_int;
        }
        _ => return FALSE,
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_int64(mut value: *mut binn, mut pint: *mut int64) -> BOOL {
    if value.is_null() || pint.is_null() {
        return FALSE;
    }
    if type_family((*value).type_0) == BINN_FAMILY_INT {
        return copy_int_value(
            (*value).ptr,
            pint as *mut ::core::ffi::c_void,
            (*value).type_0,
            BINN_INT64,
        );
    }
    match (*value).type_0 {
        BINN_FLOAT => {
            if (*value).c2rust_unnamed.vfloat < INT64_MIN as ::core::ffi::c_float
                || (*value).c2rust_unnamed.vfloat > INT64_MAX as ::core::ffi::c_float
            {
                return FALSE;
            }
            *pint = (if (*value).c2rust_unnamed.vfloat as ::core::ffi::c_double >= 0.0f64 {
                ((*value).c2rust_unnamed.vfloat as ::core::ffi::c_double + 0.5f64)
                    as ::core::ffi::c_int
            } else if (*value).c2rust_unnamed.vfloat as ::core::ffi::c_double
                - (*value).c2rust_unnamed.vfloat as ::core::ffi::c_int as ::core::ffi::c_double
                <= -0.5f64
            {
                (*value).c2rust_unnamed.vfloat as ::core::ffi::c_int
            } else {
                ((*value).c2rust_unnamed.vfloat as ::core::ffi::c_double - 0.5f64)
                    as ::core::ffi::c_int
            }) as int64;
        }
        BINN_DOUBLE => {
            if (*value).c2rust_unnamed.vdouble < INT64_MIN as ::core::ffi::c_double
                || (*value).c2rust_unnamed.vdouble > INT64_MAX as ::core::ffi::c_double
            {
                return FALSE;
            }
            *pint = (if (*value).c2rust_unnamed.vdouble >= 0.0f64 {
                ((*value).c2rust_unnamed.vdouble + 0.5f64) as ::core::ffi::c_int
            } else if (*value).c2rust_unnamed.vdouble
                - (*value).c2rust_unnamed.vdouble as ::core::ffi::c_int as ::core::ffi::c_double
                <= -0.5f64
            {
                (*value).c2rust_unnamed.vdouble as ::core::ffi::c_int
            } else {
                ((*value).c2rust_unnamed.vdouble - 0.5f64) as ::core::ffi::c_int
            }) as int64;
        }
        BINN_STRING => {
            if is_integer((*value).ptr as *mut ::core::ffi::c_char) != 0 {
                *pint = atoi64((*value).ptr as *mut ::core::ffi::c_char);
            } else if is_float((*value).ptr as *mut ::core::ffi::c_char) != 0 {
                *pint = (if atof((*value).ptr as *mut ::core::ffi::c_char) >= 0.0f64 {
                    (atof((*value).ptr as *mut ::core::ffi::c_char) + 0.5f64) as ::core::ffi::c_int
                } else if atof((*value).ptr as *mut ::core::ffi::c_char)
                    - atof((*value).ptr as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                        as ::core::ffi::c_double
                    <= -0.5f64
                {
                    atof((*value).ptr as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                } else {
                    (atof((*value).ptr as *mut ::core::ffi::c_char) - 0.5f64) as ::core::ffi::c_int
                }) as int64;
            } else {
                return FALSE;
            }
        }
        BINN_BOOL => {
            *pint = (*value).c2rust_unnamed.vbool as int64;
        }
        _ => return FALSE,
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_double(
    mut value: *mut binn,
    mut pfloat: *mut ::core::ffi::c_double,
) -> BOOL {
    let mut vint: int64 = 0;
    if value.is_null() || pfloat.is_null() {
        return FALSE;
    }
    if type_family((*value).type_0) == BINN_FAMILY_INT {
        if copy_int_value(
            (*value).ptr,
            &raw mut vint as *mut ::core::ffi::c_void,
            (*value).type_0,
            BINN_INT64,
        ) == FALSE
        {
            return FALSE;
        }
        *pfloat = vint as ::core::ffi::c_double;
        return TRUE;
    }
    match (*value).type_0 {
        BINN_FLOAT => {
            *pfloat = (*value).c2rust_unnamed.vfloat as ::core::ffi::c_double;
        }
        BINN_DOUBLE => {
            *pfloat = (*value).c2rust_unnamed.vdouble;
        }
        BINN_STRING => {
            if is_integer((*value).ptr as *mut ::core::ffi::c_char) != 0 {
                *pfloat = atoi64((*value).ptr as *mut ::core::ffi::c_char) as ::core::ffi::c_double;
            } else if is_float((*value).ptr as *mut ::core::ffi::c_char) != 0 {
                *pfloat = atof((*value).ptr as *mut ::core::ffi::c_char);
            } else {
                return FALSE;
            }
        }
        BINN_BOOL => {
            *pfloat = (*value).c2rust_unnamed.vbool as ::core::ffi::c_double;
        }
        _ => return FALSE,
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_bool(mut value: *mut binn, mut pbool: *mut BOOL) -> BOOL {
    let mut vint: int64 = 0;
    if value.is_null() || pbool.is_null() {
        return FALSE;
    }
    if type_family((*value).type_0) == BINN_FAMILY_INT {
        if copy_int_value(
            (*value).ptr,
            &raw mut vint as *mut ::core::ffi::c_void,
            (*value).type_0,
            BINN_INT64,
        ) == FALSE
        {
            return FALSE;
        }
        *pbool = (if vint != 0 as ::core::ffi::c_longlong {
            TRUE
        } else {
            FALSE
        }) as BOOL;
        return TRUE;
    }
    match (*value).type_0 {
        BINN_BOOL => {
            *pbool = (*value).c2rust_unnamed.vbool;
        }
        BINN_FLOAT => {
            *pbool = (if (*value).c2rust_unnamed.vfloat
                != 0 as ::core::ffi::c_int as ::core::ffi::c_float
            {
                TRUE
            } else {
                FALSE
            }) as BOOL;
        }
        BINN_DOUBLE => {
            *pbool = (if (*value).c2rust_unnamed.vdouble
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                TRUE
            } else {
                FALSE
            }) as BOOL;
        }
        BINN_STRING => {
            return is_bool_str((*value).ptr as *mut ::core::ffi::c_char, pbool);
        }
        _ => return FALSE,
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_str(mut value: *mut binn) -> *mut ::core::ffi::c_char {
    let mut current_block: u64;
    let mut vint: int64 = 0;
    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
    if value.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if type_family((*value).type_0) == BINN_FAMILY_INT {
        if copy_int_value(
            (*value).ptr,
            &raw mut vint as *mut ::core::ffi::c_void,
            (*value).type_0,
            BINN_INT64,
        ) == FALSE
        {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            b"%lli\0" as *const u8 as *const ::core::ffi::c_char,
            vint,
        );
    } else {
        match (*value).type_0 {
            BINN_FLOAT => {
                (*value).c2rust_unnamed.vdouble =
                    (*value).c2rust_unnamed.vfloat as ::core::ffi::c_double;
                current_block = 10880284618433631640;
            }
            BINN_DOUBLE => {
                current_block = 10880284618433631640;
            }
            BINN_STRING => return (*value).ptr as *mut ::core::ffi::c_char,
            BINN_BOOL => {
                if (*value).c2rust_unnamed.vbool != 0 {
                    strcpy(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"true\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    strcpy(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"false\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                current_block = 13639494924840279963;
            }
            _ => return ::core::ptr::null_mut::<::core::ffi::c_char>(),
        }
        match current_block {
            13639494924840279963 => {}
            _ => {
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                    b"%g\0" as *const u8 as *const ::core::ffi::c_char,
                    (*value).c2rust_unnamed.vdouble,
                );
            }
        }
    }
    (*value).ptr = strdup(&raw mut buf as *mut ::core::ffi::c_char) as *mut ::core::ffi::c_void;
    if (*value).ptr.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*value).freefn =
        Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()) as binn_mem_free;
    (*value).type_0 = BINN_STRING;
    return (*value).ptr as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_container(mut item: *mut binn) -> BOOL {
    if item.is_null() {
        return FALSE;
    }
    match (*item).type_0 {
        BINN_LIST | BINN_MAP | BINN_OBJECT => return TRUE,
        _ => return FALSE,
    };
}
pub const BINN_VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"3.0.0\0") };
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_BINN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BINN_STORAGE_NOBYTES: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BINN_STORAGE_BYTE: ::core::ffi::c_int = 32;
pub const BINN_STORAGE_WORD: ::core::ffi::c_int = 64;
pub const BINN_STORAGE_DWORD: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const BINN_STORAGE_QWORD: ::core::ffi::c_int = 128;
pub const BINN_STORAGE_STRING: ::core::ffi::c_int = 0xa0 as ::core::ffi::c_int;
pub const BINN_STORAGE_BLOB: ::core::ffi::c_int = 192;
pub const BINN_STORAGE_CONTAINER: ::core::ffi::c_int = 0xe0 as ::core::ffi::c_int;
pub const BINN_STORAGE_VIRTUAL: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const BINN_STORAGE_MIN: ::core::ffi::c_int = BINN_STORAGE_NOBYTES;
pub const BINN_STORAGE_MAX: ::core::ffi::c_int = BINN_STORAGE_CONTAINER;
pub const BINN_STORAGE_MASK: ::core::ffi::c_int = 0xe0 as ::core::ffi::c_int;
pub const BINN_STORAGE_MASK16: ::core::ffi::c_int = 0xe000 as ::core::ffi::c_int;
pub const BINN_STORAGE_HAS_MORE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BINN_TYPE_MASK: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
pub const BINN_TYPE_MASK16: ::core::ffi::c_int = 0xfff as ::core::ffi::c_int;
pub const BINN_LIST: ::core::ffi::c_int = 224;
pub const BINN_MAP: ::core::ffi::c_int = 225;
pub const BINN_OBJECT: ::core::ffi::c_int = 226;
pub const BINN_NULL: ::core::ffi::c_int = 0;
pub const BINN_TRUE: ::core::ffi::c_int = 1;
pub const BINN_FALSE: ::core::ffi::c_int = 2;
pub const BINN_UINT8: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BINN_INT8: ::core::ffi::c_int = 0x21 as ::core::ffi::c_int;
pub const BINN_UINT16: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const BINN_INT16: ::core::ffi::c_int = 0x41 as ::core::ffi::c_int;
pub const BINN_UINT32: ::core::ffi::c_int = 0x60 as ::core::ffi::c_int;
pub const BINN_INT32: ::core::ffi::c_int = 0x61 as ::core::ffi::c_int;
pub const BINN_UINT64: ::core::ffi::c_int = 128;
pub const BINN_INT64: ::core::ffi::c_int = 129;
pub const BINN_STRING: ::core::ffi::c_int = 160;
pub const BINN_DATETIME: ::core::ffi::c_int = 161;
pub const BINN_DATE: ::core::ffi::c_int = 162;
pub const BINN_TIME: ::core::ffi::c_int = 163;
pub const BINN_DECIMAL: ::core::ffi::c_int = 164;
pub const BINN_CURRENCYSTR: ::core::ffi::c_int = 165;
pub const BINN_SINGLE_STR: ::core::ffi::c_int = 166;
pub const BINN_DOUBLE_STR: ::core::ffi::c_int = 167;
pub const BINN_FLOAT32: ::core::ffi::c_int = 98;
pub const BINN_FLOAT64: ::core::ffi::c_int = 130;
pub const BINN_FLOAT: ::core::ffi::c_int = BINN_FLOAT32;
pub const BINN_DOUBLE: ::core::ffi::c_int = BINN_FLOAT64;
pub const BINN_CURRENCY: ::core::ffi::c_int = 131;
pub const BINN_BLOB: ::core::ffi::c_int = 192;
pub const BINN_BOOL: ::core::ffi::c_int = 524385;
pub const BINN_HTML: ::core::ffi::c_int = 45057;
pub const BINN_XML: ::core::ffi::c_int = 45058;
pub const BINN_JSON: ::core::ffi::c_int = 45059;
pub const BINN_JAVASCRIPT: ::core::ffi::c_int = 45060;
pub const BINN_CSS: ::core::ffi::c_int = 45061;
pub const BINN_JPEG: ::core::ffi::c_int = 53249;
pub const BINN_GIF: ::core::ffi::c_int = 53250;
pub const BINN_PNG: ::core::ffi::c_int = 53251;
pub const BINN_BMP: ::core::ffi::c_int = 53252;
pub const BINN_FAMILY_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BINN_FAMILY_NULL: ::core::ffi::c_int = 0xf1 as ::core::ffi::c_int;
pub const BINN_FAMILY_INT: ::core::ffi::c_int = 0xf2 as ::core::ffi::c_int;
pub const BINN_FAMILY_FLOAT: ::core::ffi::c_int = 0xf3 as ::core::ffi::c_int;
pub const BINN_FAMILY_STRING: ::core::ffi::c_int = 0xf4 as ::core::ffi::c_int;
pub const BINN_FAMILY_BLOB: ::core::ffi::c_int = 0xf5 as ::core::ffi::c_int;
pub const BINN_FAMILY_BOOL: ::core::ffi::c_int = 0xf6 as ::core::ffi::c_int;
pub const BINN_FAMILY_BINN: ::core::ffi::c_int = 0xf7 as ::core::ffi::c_int;
pub const BINN_SIGNED_INT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const BINN_UNSIGNED_INT: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn binn_list_add_value(mut list: *mut binn, mut value: *mut binn) -> BOOL {
    return binn_list_add(
        list,
        (*value).type_0,
        binn_ptr(value as *const ::core::ffi::c_void),
        binn_size(value as *const ::core::ffi::c_void),
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_set_value(
    mut map: *mut binn,
    mut id: ::core::ffi::c_int,
    mut value: *mut binn,
) -> BOOL {
    return binn_map_set(
        map,
        id,
        (*value).type_0,
        binn_ptr(value as *const ::core::ffi::c_void),
        binn_size(value as *const ::core::ffi::c_void),
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_set_value(
    mut obj: *mut binn,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut binn,
) -> BOOL {
    return binn_object_set(
        obj,
        key,
        (*value).type_0,
        binn_ptr(value as *const ::core::ffi::c_void),
        binn_size(value as *const ::core::ffi::c_void),
    );
}
