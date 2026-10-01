extern "C" {
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn xmlStrQEqual(
        pref: *const xmlChar,
        name: *const xmlChar,
        str: *const xmlChar,
    ) -> ::core::ffi::c_int;
    fn xmlInitMutex(mutex: xmlMutexPtr);
    fn xmlCleanupMutex(mutex: xmlMutexPtr);
    fn xmlMutexLock(tok: xmlMutexPtr);
    fn xmlMutexUnlock(tok: xmlMutexPtr);
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlInitParser();
}
pub type size_t = usize;
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
pub type xmlChar = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDict {
    pub ref_counter: ::core::ffi::c_int,
    pub table: *mut xmlDictEntry,
    pub size: size_t,
    pub nbElems: ::core::ffi::c_uint,
    pub strings: xmlDictStringsPtr,
    pub subdict: *mut _xmlDict,
    pub seed: ::core::ffi::c_uint,
    pub limit: size_t,
}
pub type xmlDictStringsPtr = *mut xmlDictStrings;
pub type xmlDictStrings = _xmlDictStrings;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDictStrings {
    pub next: xmlDictStringsPtr,
    pub free: *mut xmlChar,
    pub end: *mut xmlChar,
    pub size: size_t,
    pub nbStrings: size_t,
    pub array: [xmlChar; 1],
}
pub type xmlHashedString = xmlDictEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlDictEntry {
    pub hashValue: ::core::ffi::c_uint,
    pub name: *const xmlChar,
}
pub type xmlDict = _xmlDict;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlMutex = _xmlMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlMutex {
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type xmlMutexPtr = *mut xmlMutex;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIZE_MAX: size_t = -(1 as ::core::ffi::c_int) as size_t;
pub const MAX_FILL_NUM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const MAX_FILL_DENOM: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MIN_HASH_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAX_HASH_SIZE: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int;
static mut xmlDictMutex: xmlMutex = xmlMutex {
    lock: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null::<__pthread_internal_list>()
                    as *mut __pthread_internal_list,
                __next: ::core::ptr::null::<__pthread_internal_list>()
                    as *mut __pthread_internal_list,
            },
        },
    },
};
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeDict() -> ::core::ffi::c_int {
    xmlInitParser();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlInitDictInternal() {
    xmlInitMutex(&raw mut xmlDictMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCleanup() {}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupDictInternal() {
    xmlCleanupMutex(&raw mut xmlDictMutex);
}
unsafe extern "C" fn xmlDictAddString(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut namelen: ::core::ffi::c_uint,
) -> *const xmlChar {
    let mut current_block: u64;
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut size: size_t = 0 as size_t;
    let mut limit: size_t = 0 as size_t;
    pool = (*dict).strings;
    loop {
        if pool.is_null() {
            current_block = 7351195479953500246;
            break;
        }
        if (*pool).end.offset_from((*pool).free) as ::core::ffi::c_long as size_t
            > namelen as size_t
        {
            current_block = 17930004333942323661;
            break;
        }
        if (*pool).size > size {
            size = (*pool).size;
        }
        limit = (limit as ::core::ffi::c_ulong).wrapping_add((*pool).size as ::core::ffi::c_ulong)
            as size_t as size_t;
        pool = (*pool).next;
    }
    match current_block {
        7351195479953500246 => {
            if pool.is_null() {
                if (*dict).limit > 0 as size_t && limit > (*dict).limit {
                    return ::core::ptr::null::<xmlChar>();
                }
                if size == 0 as size_t {
                    size = 1000 as size_t;
                } else if size
                    < SIZE_MAX
                        .wrapping_sub(::core::mem::size_of::<xmlDictStrings>() as size_t)
                        .wrapping_div(4 as size_t)
                {
                    size = (size as ::core::ffi::c_ulong).wrapping_mul(4 as ::core::ffi::c_ulong)
                        as size_t as size_t;
                } else {
                    size =
                        SIZE_MAX.wrapping_sub(::core::mem::size_of::<xmlDictStrings>() as size_t);
                }
                if size.wrapping_div(4 as size_t) < namelen as size_t {
                    if (namelen as size_t).wrapping_add(0 as size_t)
                        < SIZE_MAX
                            .wrapping_sub(::core::mem::size_of::<xmlDictStrings>() as size_t)
                            .wrapping_div(4 as size_t)
                    {
                        size = (4 as size_t).wrapping_mul(namelen as size_t);
                    } else {
                        return ::core::ptr::null::<xmlChar>();
                    }
                }
                pool = xmlMalloc.expect("non-null function pointer")(
                    (::core::mem::size_of::<xmlDictStrings>() as size_t).wrapping_add(size),
                ) as xmlDictStringsPtr;
                if pool.is_null() {
                    return ::core::ptr::null::<xmlChar>();
                }
                (*pool).size = size;
                (*pool).nbStrings = 0 as size_t;
                (*pool).free = (&raw mut (*pool).array as *mut xmlChar)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut xmlChar;
                (*pool).end =
                    (&raw mut (*pool).array as *mut xmlChar).offset(size as isize) as *mut xmlChar;
                (*pool).next = (*dict).strings;
                (*dict).strings = pool;
            }
        }
        _ => {}
    }
    ret = (*pool).free;
    memcpy(
        (*pool).free as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        namelen as size_t,
    );
    (*pool).free = (*pool).free.offset(namelen as isize);
    let fresh2 = (*pool).free;
    (*pool).free = (*pool).free.offset(1);
    *fresh2 = 0 as xmlChar;
    (*pool).nbStrings = (*pool).nbStrings.wrapping_add(1);
    return ret;
}
unsafe extern "C" fn xmlDictAddQString(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut plen: ::core::ffi::c_uint,
    mut name: *const xmlChar,
    mut namelen: ::core::ffi::c_uint,
) -> *const xmlChar {
    let mut current_block: u64;
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut size: size_t = 0 as size_t;
    let mut limit: size_t = 0 as size_t;
    pool = (*dict).strings;
    loop {
        if pool.is_null() {
            current_block = 7351195479953500246;
            break;
        }
        if (*pool).end.offset_from((*pool).free) as ::core::ffi::c_long as size_t
            > namelen
                .wrapping_add(plen)
                .wrapping_add(1 as ::core::ffi::c_uint) as size_t
        {
            current_block = 11279562675727845827;
            break;
        }
        if (*pool).size > size {
            size = (*pool).size;
        }
        limit = (limit as ::core::ffi::c_ulong).wrapping_add((*pool).size as ::core::ffi::c_ulong)
            as size_t as size_t;
        pool = (*pool).next;
    }
    match current_block {
        7351195479953500246 => {
            if pool.is_null() {
                if (*dict).limit > 0 as size_t && limit > (*dict).limit {
                    return ::core::ptr::null::<xmlChar>();
                }
                if size == 0 as size_t {
                    size = 1000 as size_t;
                } else {
                    size = (size as ::core::ffi::c_ulong).wrapping_mul(4 as ::core::ffi::c_ulong)
                        as size_t as size_t;
                }
                if size
                    < (4 as ::core::ffi::c_uint).wrapping_mul(
                        namelen
                            .wrapping_add(plen)
                            .wrapping_add(1 as ::core::ffi::c_uint),
                    ) as size_t
                {
                    size = (4 as ::core::ffi::c_uint).wrapping_mul(
                        namelen
                            .wrapping_add(plen)
                            .wrapping_add(1 as ::core::ffi::c_uint),
                    ) as size_t;
                }
                pool = xmlMalloc.expect("non-null function pointer")(
                    (::core::mem::size_of::<xmlDictStrings>() as size_t).wrapping_add(size),
                ) as xmlDictStringsPtr;
                if pool.is_null() {
                    return ::core::ptr::null::<xmlChar>();
                }
                (*pool).size = size;
                (*pool).nbStrings = 0 as size_t;
                (*pool).free = (&raw mut (*pool).array as *mut xmlChar)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut xmlChar;
                (*pool).end =
                    (&raw mut (*pool).array as *mut xmlChar).offset(size as isize) as *mut xmlChar;
                (*pool).next = (*dict).strings;
                (*dict).strings = pool;
            }
        }
        _ => {}
    }
    ret = (*pool).free;
    memcpy(
        (*pool).free as *mut ::core::ffi::c_void,
        prefix as *const ::core::ffi::c_void,
        plen as size_t,
    );
    (*pool).free = (*pool).free.offset(plen as isize);
    let fresh0 = (*pool).free;
    (*pool).free = (*pool).free.offset(1);
    *fresh0 = ':' as i32 as xmlChar;
    memcpy(
        (*pool).free as *mut ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        namelen as size_t,
    );
    (*pool).free = (*pool).free.offset(namelen as isize);
    let fresh1 = (*pool).free;
    (*pool).free = (*pool).free.offset(1);
    *fresh1 = 0 as xmlChar;
    (*pool).nbStrings = (*pool).nbStrings.wrapping_add(1);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCreate() -> xmlDictPtr {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    xmlInitParser();
    dict = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlDict>() as size_t)
        as xmlDictPtr;
    if dict.is_null() {
        return ::core::ptr::null_mut::<xmlDict>();
    }
    (*dict).ref_counter = 1 as ::core::ffi::c_int;
    (*dict).limit = 0 as size_t;
    (*dict).size = 0 as size_t;
    (*dict).nbElems = 0 as ::core::ffi::c_uint;
    (*dict).table = ::core::ptr::null_mut::<xmlDictEntry>();
    (*dict).strings = ::core::ptr::null_mut::<xmlDictStrings>();
    (*dict).subdict = ::core::ptr::null_mut::<_xmlDict>();
    (*dict).seed = xmlRandom();
    return dict;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCreateSub(mut sub: xmlDictPtr) -> xmlDictPtr {
    let mut dict: xmlDictPtr = xmlDictCreate();
    if !dict.is_null() && !sub.is_null() {
        (*dict).seed = (*sub).seed;
        (*dict).subdict = sub as *mut _xmlDict;
        xmlDictReference((*dict).subdict as xmlDictPtr);
    }
    return dict;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictReference(mut dict: xmlDictPtr) -> ::core::ffi::c_int {
    if dict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    xmlMutexLock(&raw mut xmlDictMutex);
    (*dict).ref_counter += 1;
    xmlMutexUnlock(&raw mut xmlDictMutex);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictFree(mut dict: xmlDictPtr) {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut nextp: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    if dict.is_null() {
        return;
    }
    xmlMutexLock(&raw mut xmlDictMutex);
    (*dict).ref_counter -= 1;
    if (*dict).ref_counter > 0 as ::core::ffi::c_int {
        xmlMutexUnlock(&raw mut xmlDictMutex);
        return;
    }
    xmlMutexUnlock(&raw mut xmlDictMutex);
    if !(*dict).subdict.is_null() {
        xmlDictFree((*dict).subdict as xmlDictPtr);
    }
    if !(*dict).table.is_null() {
        xmlFree.expect("non-null function pointer")((*dict).table as *mut ::core::ffi::c_void);
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        nextp = (*pool).next;
        xmlFree.expect("non-null function pointer")(pool as *mut ::core::ffi::c_void);
        pool = nextp;
    }
    xmlFree.expect("non-null function pointer")(dict as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictOwns(
    mut dict: xmlDictPtr,
    mut str: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    if dict.is_null() || str.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        if str
            >= (&raw mut (*pool).array as *mut xmlChar).offset(0 as ::core::ffi::c_int as isize)
                as *mut xmlChar as *const xmlChar
            && str <= (*pool).free as *const xmlChar
        {
            return 1 as ::core::ffi::c_int;
        }
        pool = (*pool).next;
    }
    if !(*dict).subdict.is_null() {
        return xmlDictOwns((*dict).subdict as xmlDictPtr, str);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictSize(mut dict: xmlDictPtr) -> ::core::ffi::c_int {
    if dict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*dict).subdict.is_null() {
        return (*dict).nbElems.wrapping_add((*(*dict).subdict).nbElems) as ::core::ffi::c_int;
    }
    return (*dict).nbElems as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictSetLimit(mut dict: xmlDictPtr, mut limit: size_t) -> size_t {
    let mut ret: size_t = 0;
    if dict.is_null() {
        return 0 as size_t;
    }
    ret = (*dict).limit;
    (*dict).limit = limit;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictGetUsage(mut dict: xmlDictPtr) -> size_t {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut limit: size_t = 0 as size_t;
    if dict.is_null() {
        return 0 as size_t;
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        limit = (limit as ::core::ffi::c_ulong).wrapping_add((*pool).size as ::core::ffi::c_ulong)
            as size_t as size_t;
        pool = (*pool).next;
    }
    return limit;
}
unsafe extern "C" fn xmlDictHashName(
    mut seed: ::core::ffi::c_uint,
    mut data: *const xmlChar,
    mut maxLen: size_t,
    mut plen: *mut size_t,
) -> ::core::ffi::c_uint {
    let mut h1: ::core::ffi::c_uint = 0;
    let mut h2: ::core::ffi::c_uint = 0;
    let mut i: size_t = 0;
    h1 = seed ^ 0x3b00 as ::core::ffi::c_uint;
    h2 = seed << 15 as ::core::ffi::c_int
        | (seed & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < maxLen && *data.offset(i as isize) as ::core::ffi::c_int != 0 {
        h1 = h1.wrapping_add(*data.offset(i as isize) as ::core::ffi::c_uint);
        h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 14 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as ::core::ffi::c_uint) >> 6 as ::core::ffi::c_int
            | h1 << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int,
    );
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 5 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
            | h1 << 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int,
    );
    h2 &= 0xffffffff as ::core::ffi::c_uint;
    *plen = i;
    return h2 | MAX_HASH_SIZE;
}
unsafe extern "C" fn xmlDictHashQName(
    mut seed: ::core::ffi::c_uint,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut pplen: *mut size_t,
    mut plen: *mut size_t,
) -> ::core::ffi::c_uint {
    let mut h1: ::core::ffi::c_uint = 0;
    let mut h2: ::core::ffi::c_uint = 0;
    let mut i: size_t = 0;
    h1 = seed ^ 0x3b00 as ::core::ffi::c_uint;
    h2 = seed << 15 as ::core::ffi::c_int
        | (seed & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    i = 0 as size_t;
    while *prefix.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        h1 = h1.wrapping_add(*prefix.offset(i as isize) as ::core::ffi::c_uint);
        h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    *pplen = i;
    h1 = h1.wrapping_add(':' as i32 as ::core::ffi::c_uint);
    h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
    h2 = h2.wrapping_add(h1);
    h2 = h2 << 7 as ::core::ffi::c_int
        | (h2 & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
    i = 0 as size_t;
    while *name.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        h1 = h1.wrapping_add(*name.offset(i as isize) as ::core::ffi::c_uint);
        h1 = h1.wrapping_add(h1 << 3 as ::core::ffi::c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        h2 = h2.wrapping_add(h2 << 2 as ::core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    *plen = i;
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 14 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as ::core::ffi::c_uint) >> 6 as ::core::ffi::c_int
            | h1 << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int,
    );
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 5 as ::core::ffi::c_int
            | (h2 & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
            | h1 << 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int,
    );
    h2 &= 0xffffffff as ::core::ffi::c_uint;
    return h2 | MAX_HASH_SIZE;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictComputeHash(
    mut dict: *const xmlDict,
    mut string: *const xmlChar,
) -> ::core::ffi::c_uint {
    let mut len: size_t = 0;
    return xmlDictHashName((*dict).seed, string, SIZE_MAX, &raw mut len);
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCombineHash(
    mut v1: ::core::ffi::c_uint,
    mut v2: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    v1 ^= v2;
    v1 = v1.wrapping_add(
        v2 << 5 as ::core::ffi::c_int
            | (v2 & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint)
                >> 31 as ::core::ffi::c_int - 5 as ::core::ffi::c_int,
    );
    return v1 & 0xffffffff as ::core::ffi::c_uint | 0x80000000 as ::core::ffi::c_uint;
}
unsafe extern "C" fn xmlDictFindEntry(
    mut dict: *const xmlDict,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut len: size_t,
    mut plen: size_t,
    mut hashValue: ::core::ffi::c_uint,
    mut pfound: *mut ::core::ffi::c_int,
) -> *mut xmlDictEntry {
    let mut entry: *mut xmlDictEntry = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut mask: ::core::ffi::c_uint = 0;
    let mut pos: ::core::ffi::c_uint = 0;
    let mut displ: ::core::ffi::c_uint = 0;
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    mask = (*dict).size.wrapping_sub(1 as size_t) as ::core::ffi::c_uint;
    pos = hashValue & mask;
    entry = (*dict).table.offset(pos as isize) as *mut xmlDictEntry;
    if (*entry).hashValue != 0 as ::core::ffi::c_uint {
        displ = 0 as ::core::ffi::c_uint;
        loop {
            if (*entry).hashValue == hashValue {
                if prefix.is_null() {
                    if strncmp(
                        (*entry).name as *const ::core::ffi::c_char,
                        name as *const ::core::ffi::c_char,
                        len,
                    ) == 0 as ::core::ffi::c_int
                        && *(*entry).name.offset(len as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        found = 1 as ::core::ffi::c_int;
                        break;
                    }
                } else if strncmp(
                    (*entry).name as *const ::core::ffi::c_char,
                    prefix as *const ::core::ffi::c_char,
                    plen,
                ) == 0 as ::core::ffi::c_int
                    && *(*entry).name.offset(plen as isize) as ::core::ffi::c_int == ':' as i32
                    && strncmp(
                        (*entry).name.offset(plen.wrapping_add(1) as isize)
                            as *const ::core::ffi::c_char,
                        name as *const ::core::ffi::c_char,
                        len,
                    ) == 0 as ::core::ffi::c_int
                    && *(*entry)
                        .name
                        .offset(plen.wrapping_add(1).wrapping_add(len) as isize)
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    found = 1 as ::core::ffi::c_int;
                    break;
                }
            }
            displ = displ.wrapping_add(1);
            pos = pos.wrapping_add(1);
            entry = entry.offset(1);
            if pos & mask == 0 as ::core::ffi::c_uint {
                entry = (*dict).table;
            }
            if !((*entry).hashValue != 0 as ::core::ffi::c_uint
                && pos.wrapping_sub((*entry).hashValue) & mask >= displ)
            {
                break;
            }
        }
    }
    *pfound = found;
    return entry;
}
unsafe extern "C" fn xmlDictGrow(
    mut dict: xmlDictPtr,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut oldentry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    let mut oldend: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    let mut end: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    let mut table: *mut xmlDictEntry = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut oldsize: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    if (size as size_t).wrapping_add(0 as size_t)
        > SIZE_MAX.wrapping_div(::core::mem::size_of::<xmlDictEntry>() as size_t)
    {
        return -(1 as ::core::ffi::c_int);
    }
    table = xmlMalloc.expect("non-null function pointer")(
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
    ) as *mut xmlDictEntry;
    if table.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        table as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
    );
    oldsize = (*dict).size as ::core::ffi::c_uint;
    if !(oldsize == 0 as ::core::ffi::c_uint) {
        oldend = (*dict).table.offset(oldsize as isize) as *mut xmlDictEntry;
        end = table.offset(size as isize) as *mut xmlDictEntry;
        oldentry = (*dict).table;
        while (*oldentry).hashValue != 0 as ::core::ffi::c_uint {
            oldentry = oldentry.offset(1);
            if oldentry >= oldend {
                oldentry = (*dict).table;
            }
        }
        i = 0 as ::core::ffi::c_uint;
        while i < oldsize {
            if (*oldentry).hashValue != 0 as ::core::ffi::c_uint {
                let mut entry: *mut xmlDictEntry = table.offset(
                    ((*oldentry).hashValue & size.wrapping_sub(1 as ::core::ffi::c_uint)) as isize,
                ) as *mut xmlDictEntry;
                while (*entry).hashValue != 0 as ::core::ffi::c_uint {
                    entry = entry.offset(1);
                    if entry >= end as *mut xmlDictEntry {
                        entry = table;
                    }
                }
                *entry = *oldentry;
            }
            oldentry = oldentry.offset(1);
            if oldentry >= oldend {
                oldentry = (*dict).table;
            }
            i = i.wrapping_add(1);
        }
        xmlFree.expect("non-null function pointer")((*dict).table as *mut ::core::ffi::c_void);
    }
    (*dict).table = table;
    (*dict).size = size as size_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlDictLookupInternal(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut maybeLen: ::core::ffi::c_int,
    mut update: ::core::ffi::c_int,
) -> *const xmlDictEntry {
    let mut entry: *mut xmlDictEntry = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut hashValue: ::core::ffi::c_uint = 0;
    let mut maxLen: size_t = 0;
    let mut len: size_t = 0;
    let mut plen: size_t = 0;
    let mut klen: size_t = 0;
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if dict.is_null() || name.is_null() {
        return ::core::ptr::null::<xmlDictEntry>();
    }
    maxLen = if maybeLen < 0 as ::core::ffi::c_int {
        SIZE_MAX
    } else {
        maybeLen as size_t
    };
    if prefix.is_null() {
        hashValue = xmlDictHashName((*dict).seed, name, maxLen, &raw mut len);
        if len > (INT_MAX / 2 as ::core::ffi::c_int) as size_t {
            return ::core::ptr::null::<xmlDictEntry>();
        }
        klen = len;
    } else {
        hashValue = xmlDictHashQName((*dict).seed, prefix, name, &raw mut plen, &raw mut len);
        if len > (INT_MAX / 2 as ::core::ffi::c_int) as size_t
            || plen >= ((INT_MAX / 2 as ::core::ffi::c_int) as size_t).wrapping_sub(len)
        {
            return ::core::ptr::null::<xmlDictEntry>();
        }
        klen = plen.wrapping_add(1 as size_t).wrapping_add(len);
    }
    if (*dict).limit > 0 as size_t && klen >= (*dict).limit {
        return ::core::ptr::null::<xmlDictEntry>();
    }
    if (*dict).size > 0 as size_t {
        entry = xmlDictFindEntry(dict as *const xmlDict, prefix, name, len, plen, hashValue, &raw mut found);
    }
    if found != 0 {
        return entry;
    }
    if !(*dict).subdict.is_null() && (*(*dict).subdict).size > 0 as size_t {
        let mut subEntry: *mut xmlDictEntry = ::core::ptr::null_mut::<xmlDictEntry>();
        let mut subHashValue: ::core::ffi::c_uint = 0;
        if prefix.is_null() {
            subHashValue = xmlDictHashName((*(*dict).subdict).seed, name, len, &raw mut len);
        } else {
            subHashValue = xmlDictHashQName(
                (*(*dict).subdict).seed,
                prefix,
                name,
                &raw mut plen,
                &raw mut len,
            );
        }
        subEntry = xmlDictFindEntry(
            (*dict).subdict,
            prefix,
            name,
            len,
            plen,
            subHashValue,
            &raw mut found,
        );
        if found != 0 {
            return subEntry;
        }
    }
    if update == 0 {
        return ::core::ptr::null::<xmlDictEntry>();
    }
    if (*dict).nbElems.wrapping_add(1 as ::core::ffi::c_uint) as size_t
        > (*dict)
            .size
            .wrapping_div(MAX_FILL_DENOM as size_t)
            .wrapping_mul(MAX_FILL_NUM as size_t)
    {
        let mut newSize: ::core::ffi::c_uint = 0;
        let mut mask: ::core::ffi::c_uint = 0;
        let mut displ: ::core::ffi::c_uint = 0;
        let mut pos: ::core::ffi::c_uint = 0;
        if (*dict).size == 0 as size_t {
            newSize = MIN_HASH_SIZE as ::core::ffi::c_uint;
        } else {
            if (*dict).size >= MAX_HASH_SIZE as size_t {
                return ::core::ptr::null::<xmlDictEntry>();
            }
            newSize = (*dict).size.wrapping_mul(2 as size_t) as ::core::ffi::c_uint;
        }
        if xmlDictGrow(dict, newSize) != 0 as ::core::ffi::c_int {
            return ::core::ptr::null::<xmlDictEntry>();
        }
        mask = (*dict).size.wrapping_sub(1 as size_t) as ::core::ffi::c_uint;
        displ = 0 as ::core::ffi::c_uint;
        pos = hashValue & mask;
        entry = (*dict).table.offset(pos as isize) as *mut xmlDictEntry;
        while (*entry).hashValue != 0 as ::core::ffi::c_uint
            && pos.wrapping_sub((*entry).hashValue) & mask >= displ
        {
            displ = displ.wrapping_add(1);
            pos = pos.wrapping_add(1);
            entry = entry.offset(1);
            if pos & mask == 0 as ::core::ffi::c_uint {
                entry = (*dict).table;
            }
        }
    }
    if prefix.is_null() {
        ret = xmlDictAddString(dict, name, len as ::core::ffi::c_uint);
    } else {
        ret = xmlDictAddQString(
            dict,
            prefix,
            plen as ::core::ffi::c_uint,
            name,
            len as ::core::ffi::c_uint,
        );
    }
    if ret.is_null() {
        return ::core::ptr::null::<xmlDictEntry>();
    }
    if (*entry).hashValue != 0 as ::core::ffi::c_uint {
        let mut end: *const xmlDictEntry =
            (*dict).table.offset((*dict).size as isize) as *mut xmlDictEntry;
        let mut cur: *const xmlDictEntry = entry;
        loop {
            cur = cur.offset(1);
            if cur >= end {
                cur = (*dict).table;
            }
            if !((*cur).hashValue != 0 as ::core::ffi::c_uint) {
                break;
            }
        }
        if cur < entry as *const xmlDictEntry {
            memmove(
                (*dict).table.offset(1 as ::core::ffi::c_int as isize) as *mut xmlDictEntry
                    as *mut ::core::ffi::c_void,
                (*dict).table as *const ::core::ffi::c_void,
                (cur as *mut ::core::ffi::c_char)
                    .offset_from((*dict).table as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as size_t,
            );
            cur = end.offset(-(1 as ::core::ffi::c_int as isize));
            *(*dict).table.offset(0 as ::core::ffi::c_int as isize) = *cur;
        }
        memmove(
            entry.offset(1 as ::core::ffi::c_int as isize) as *mut xmlDictEntry
                as *mut ::core::ffi::c_void,
            entry as *const ::core::ffi::c_void,
            (cur as *mut ::core::ffi::c_char).offset_from(entry as *mut ::core::ffi::c_char)
                as ::core::ffi::c_long as size_t,
        );
    }
    (*entry).hashValue = hashValue;
    (*entry).name = ret;
    (*dict).nbElems = (*dict).nbElems.wrapping_add(1);
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictLookup(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *const xmlChar {
    let mut entry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    entry = xmlDictLookupInternal(
        dict,
        ::core::ptr::null::<xmlChar>(),
        name,
        len,
        1 as ::core::ffi::c_int,
    );
    if entry.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return (*entry).name;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictLookupHashed(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> xmlHashedString {
    let mut entry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    let mut ret: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    entry = xmlDictLookupInternal(
        dict,
        ::core::ptr::null::<xmlChar>(),
        name,
        len,
        1 as ::core::ffi::c_int,
    );
    if entry.is_null() {
        ret.name = ::core::ptr::null::<xmlChar>();
        ret.hashValue = 0 as ::core::ffi::c_uint;
    } else {
        ret = *entry as xmlHashedString;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictExists(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *const xmlChar {
    let mut entry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    entry = xmlDictLookupInternal(
        dict,
        ::core::ptr::null::<xmlChar>(),
        name,
        len,
        0 as ::core::ffi::c_int,
    );
    if entry.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return (*entry).name;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictQLookup(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
) -> *const xmlChar {
    let mut entry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    entry = xmlDictLookupInternal(
        dict,
        prefix,
        name,
        -(1 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int,
    );
    if entry.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return (*entry).name;
}
static mut xmlRngMutex: xmlMutex = xmlMutex {
    lock: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_list_t {
                __prev: ::core::ptr::null::<__pthread_internal_list>()
                    as *mut __pthread_internal_list,
                __next: ::core::ptr::null::<__pthread_internal_list>()
                    as *mut __pthread_internal_list,
            },
        },
    },
};
static mut globalRngState: [::core::ffi::c_uint; 2] = [0; 2];
#[no_mangle]
pub unsafe extern "C" fn xmlInitRandom() {
    let mut var: ::core::ffi::c_int = 0;
    xmlInitMutex(&raw mut xmlRngMutex);
    globalRngState[0 as ::core::ffi::c_int as usize] = time(::core::ptr::null_mut::<time_t>())
        as ::core::ffi::c_uint
        ^ ((::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, size_t>(Some(
            xmlInitRandom as unsafe extern "C" fn() -> (),
        )) as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_int
            | (::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, size_t>(Some(
                xmlInitRandom as unsafe extern "C" fn() -> (),
            )) as ::core::ffi::c_uint
                & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int);
    globalRngState[1 as ::core::ffi::c_int as usize] = ((&raw mut xmlRngMutex as size_t
        as ::core::ffi::c_uint)
        << 16 as ::core::ffi::c_int
        | (&raw mut xmlRngMutex as size_t as ::core::ffi::c_uint
            & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int)
        ^ ((&raw mut var as size_t as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
            | (&raw mut var as size_t as ::core::ffi::c_uint & 0xffffffff as ::core::ffi::c_uint)
                >> 32 as ::core::ffi::c_int - 24 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupRandom() {
    xmlCleanupMutex(&raw mut xmlRngMutex);
}
unsafe extern "C" fn xoroshiro64ss(mut s: *mut ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut s0: ::core::ffi::c_uint = *s.offset(0 as ::core::ffi::c_int as isize);
    let mut s1: ::core::ffi::c_uint = *s.offset(1 as ::core::ffi::c_int as isize);
    let mut result: ::core::ffi::c_uint = (s0.wrapping_mul(0x9e3779bb as ::core::ffi::c_uint)
        << 5 as ::core::ffi::c_int
        | (s0.wrapping_mul(0x9e3779bb as ::core::ffi::c_uint) & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
        .wrapping_mul(5 as ::core::ffi::c_uint);
    s1 ^= s0;
    *s.offset(0 as ::core::ffi::c_int as isize) = (s0 << 26 as ::core::ffi::c_int
        | (s0 & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 26 as ::core::ffi::c_int)
        ^ s1
        ^ s1 << 9 as ::core::ffi::c_int;
    *s.offset(1 as ::core::ffi::c_int as isize) = s1 << 13 as ::core::ffi::c_int
        | (s1 & 0xffffffff as ::core::ffi::c_uint)
            >> 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int;
    return result & 0xffffffff as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRandom() -> ::core::ffi::c_uint {
    let mut ret: ::core::ffi::c_uint = 0;
    xmlMutexLock(&raw mut xmlRngMutex);
    ret = xoroshiro64ss(&raw mut globalRngState as *mut ::core::ffi::c_uint);
    xmlMutexUnlock(&raw mut xmlRngMutex);
    return ret;
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
