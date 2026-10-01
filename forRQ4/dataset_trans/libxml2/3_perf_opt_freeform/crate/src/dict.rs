use core::ffi::*;
use crate::src::threads::xmlInitParser;
use crate::src::xmlstring::xmlStrQEqual;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn xmlInitMutex(mutex: xmlMutexPtr);
    fn xmlCleanupMutex(mutex: xmlMutexPtr);
    fn xmlMutexLock(tok: xmlMutexPtr);
    fn xmlMutexUnlock(tok: xmlMutexPtr);
}

pub type time_t = __time_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDict {
    pub ref_counter: c_int,
    pub table: *mut xmlDictEntry,
    pub size: size_t,
    pub nbElems: c_uint,
    pub strings: xmlDictStringsPtr,
    pub subdict: *mut _xmlDict,
    pub seed: c_uint,
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
    pub hashValue: c_uint,
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
    pub __size: [c_char; 40],
    pub __align: c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: c_int,
    pub __count: c_uint,
    pub __owner: c_int,
    pub __nusers: c_uint,
    pub __kind: c_int,
    pub __spins: c_short,
    pub __elision: c_short,
    pub __list: __pthread_list_t,
}

pub type xmlMutexPtr = *mut xmlMutex;

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
#[inline]
pub fn xmlInitializeDict() -> c_int { {
    xmlInitParser();
    return 0 as c_int;
} }
#[inline]
pub fn xmlInitDictInternal() { unsafe {
    xmlInitMutex(&raw mut xmlDictMutex);
} }
#[inline]
pub fn xmlDictCleanup() { {} }
#[inline]
pub fn xmlCleanupDictInternal() { unsafe {
    xmlCleanupMutex(&raw mut xmlDictMutex);
} }
unsafe fn xmlDictAddString(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut namelen: c_uint,
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
        if (*pool).end.offset_from((*pool).free) as c_long as size_t
            > namelen as size_t
        {
            current_block = 17930004333942323661;
            break;
        }
        if (*pool).size > size {
            size = (*pool).size;
        }
        limit = (limit as c_ulong).wrapping_add((*pool).size as c_ulong)
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
                    size = (size as c_ulong).wrapping_mul(4 as c_ulong)
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
                    .offset(0 as c_int as isize)
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
        (*pool).free as *mut c_void,
        name as *const c_void,
        namelen as size_t,
    );
    (*pool).free = (*pool).free.offset(namelen as isize);
    let fresh2 = (*pool).free;
    (*pool).free = (*pool).free.offset(1);
    *fresh2 = 0 as xmlChar;
    (*pool).nbStrings = (*pool).nbStrings.wrapping_add(1);
    return ret;
}
unsafe fn xmlDictAddQString(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut plen: c_uint,
    mut name: *const xmlChar,
    mut namelen: c_uint,
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
        if (*pool).end.offset_from((*pool).free) as c_long as size_t
            > namelen
                .wrapping_add(plen)
                .wrapping_add(1 as c_uint) as size_t
        {
            current_block = 11279562675727845827;
            break;
        }
        if (*pool).size > size {
            size = (*pool).size;
        }
        limit = (limit as c_ulong).wrapping_add((*pool).size as c_ulong)
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
                    size = (size as c_ulong).wrapping_mul(4 as c_ulong)
                        as size_t as size_t;
                }
                if size
                    < (4 as c_uint).wrapping_mul(
                        namelen
                            .wrapping_add(plen)
                            .wrapping_add(1 as c_uint),
                    ) as size_t
                {
                    size = (4 as c_uint).wrapping_mul(
                        namelen
                            .wrapping_add(plen)
                            .wrapping_add(1 as c_uint),
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
                    .offset(0 as c_int as isize)
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
        (*pool).free as *mut c_void,
        prefix as *const c_void,
        plen as size_t,
    );
    (*pool).free = (*pool).free.offset(plen as isize);
    let fresh0 = (*pool).free;
    (*pool).free = (*pool).free.offset(1);
    *fresh0 = ':' as i32 as xmlChar;
    memcpy(
        (*pool).free as *mut c_void,
        name as *const c_void,
        namelen as size_t,
    );
    (*pool).free = (*pool).free.offset(namelen as isize);
    let fresh1 = (*pool).free;
    (*pool).free = (*pool).free.offset(1);
    *fresh1 = 0 as xmlChar;
    (*pool).nbStrings = (*pool).nbStrings.wrapping_add(1);
    return ret;
}
#[inline]
pub fn xmlDictCreate() -> xmlDictPtr { unsafe {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    xmlInitParser();
    dict = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlDict>() as size_t)
        as xmlDictPtr;
    if dict.is_null() {
        return ::core::ptr::null_mut::<xmlDict>();
    }
    (*dict).ref_counter = 1 as c_int;
    (*dict).limit = 0 as size_t;
    (*dict).size = 0 as size_t;
    (*dict).nbElems = 0 as c_uint;
    (*dict).table = ::core::ptr::null_mut::<xmlDictEntry>();
    (*dict).strings = ::core::ptr::null_mut::<xmlDictStrings>();
    (*dict).subdict = ::core::ptr::null_mut::<_xmlDict>();
    (*dict).seed = xmlRandom();
    return dict;
} }
#[inline]
pub fn xmlDictCreateSub(mut sub: xmlDictPtr) -> xmlDictPtr { unsafe {
    let mut dict: xmlDictPtr = xmlDictCreate();
    if !dict.is_null() && !sub.is_null() {
        (*dict).seed = (*sub).seed;
        (*dict).subdict = sub as *mut _xmlDict;
        xmlDictReference((*dict).subdict as xmlDictPtr);
    }
    return dict;
} }
#[inline]
pub fn xmlDictReference(mut dict: xmlDictPtr) -> c_int { unsafe {
    if dict.is_null() {
        return -(1 as c_int);
    }
    xmlMutexLock(&raw mut xmlDictMutex);
    (*dict).ref_counter += 1;
    xmlMutexUnlock(&raw mut xmlDictMutex);
    return 0 as c_int;
} }
#[inline]
pub fn xmlDictFree(mut dict: xmlDictPtr) { unsafe {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut nextp: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    if dict.is_null() {
        return;
    }
    xmlMutexLock(&raw mut xmlDictMutex);
    (*dict).ref_counter -= 1;
    if (*dict).ref_counter > 0 as c_int {
        xmlMutexUnlock(&raw mut xmlDictMutex);
        return;
    }
    xmlMutexUnlock(&raw mut xmlDictMutex);
    if !(*dict).subdict.is_null() {
        xmlDictFree((*dict).subdict as xmlDictPtr);
    }
    if !(*dict).table.is_null() {
        xmlFree.expect("non-null function pointer")((*dict).table as *mut c_void);
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        nextp = (*pool).next;
        xmlFree.expect("non-null function pointer")(pool as *mut c_void);
        pool = nextp;
    }
    xmlFree.expect("non-null function pointer")(dict as *mut c_void);
} }
#[inline]
pub unsafe fn xmlDictOwns(
    mut dict: xmlDictPtr,
    mut str: *const xmlChar,
) -> c_int {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    if dict.is_null() || str.is_null() {
        return -(1 as c_int);
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        if str
            >= (&raw mut (*pool).array as *mut xmlChar).offset(0 as c_int as isize)
                as *mut xmlChar as *const xmlChar
            && str <= (*pool).free as *const xmlChar
        {
            return 1 as c_int;
        }
        pool = (*pool).next;
    }
    if !(*dict).subdict.is_null() {
        return xmlDictOwns((*dict).subdict as xmlDictPtr, str);
    }
    return 0 as c_int;
}
#[inline]
pub fn xmlDictSize(mut dict: xmlDictPtr) -> c_int { unsafe {
    if dict.is_null() {
        return -(1 as c_int);
    }
    if !(*dict).subdict.is_null() {
        return (*dict).nbElems.wrapping_add((*(*dict).subdict).nbElems) as c_int;
    }
    return (*dict).nbElems as c_int;
} }
#[inline]
pub fn xmlDictSetLimit(mut dict: xmlDictPtr, mut limit: size_t) -> size_t { unsafe {
    let mut ret: size_t = 0;
    if dict.is_null() {
        return 0 as size_t;
    }
    ret = (*dict).limit;
    (*dict).limit = limit;
    return ret;
} }
#[inline]
pub fn xmlDictGetUsage(mut dict: xmlDictPtr) -> size_t { unsafe {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut limit: size_t = 0 as size_t;
    if dict.is_null() {
        return 0 as size_t;
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        limit = (limit as c_ulong).wrapping_add((*pool).size as c_ulong)
            as size_t as size_t;
        pool = (*pool).next;
    }
    return limit;
} }
unsafe fn xmlDictHashName(
    mut seed: c_uint,
    mut data: *const xmlChar,
    mut maxLen: size_t,
    mut plen: *mut size_t,
) -> c_uint {
    let mut h1: c_uint = 0;
    let mut h2: c_uint = 0;
    let mut i: size_t = 0;
    h1 = seed ^ 0x3b00 as c_uint;
    h2 = seed << 15 as c_int
        | (seed & 0xffffffff as c_uint)
            >> 32 as c_int - 15 as c_int;
    i = 0 as size_t;
    while i < maxLen && *data.offset(i as isize) as c_int != 0 {
        h1 = h1.wrapping_add(*data.offset(i as isize) as c_uint);
        h1 = h1.wrapping_add(h1 << 3 as c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as c_int
            | (h2 & 0xffffffff as c_uint)
                >> 32 as c_int - 7 as c_int;
        h2 = h2.wrapping_add(h2 << 2 as c_int);
        i = i.wrapping_add(1);
    }
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 14 as c_int
            | (h2 & 0xffffffff as c_uint)
                >> 32 as c_int - 14 as c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as c_uint) >> 6 as c_int
            | h1 << 32 as c_int - 6 as c_int,
    );
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 5 as c_int
            | (h2 & 0xffffffff as c_uint)
                >> 32 as c_int - 5 as c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as c_uint) >> 8 as c_int
            | h1 << 32 as c_int - 8 as c_int,
    );
    h2 &= 0xffffffff as c_uint;
    *plen = i;
    return h2 | MAX_HASH_SIZE;
}
unsafe fn xmlDictHashQName(
    mut seed: c_uint,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut pplen: *mut size_t,
    mut plen: *mut size_t,
) -> c_uint {
    let mut h1: c_uint = 0;
    let mut h2: c_uint = 0;
    let mut i: size_t = 0;
    h1 = seed ^ 0x3b00 as c_uint;
    h2 = seed << 15 as c_int
        | (seed & 0xffffffff as c_uint)
            >> 32 as c_int - 15 as c_int;
    i = 0 as size_t;
    while *prefix.offset(i as isize) as c_int != 0 as c_int {
        h1 = h1.wrapping_add(*prefix.offset(i as isize) as c_uint);
        h1 = h1.wrapping_add(h1 << 3 as c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as c_int
            | (h2 & 0xffffffff as c_uint)
                >> 32 as c_int - 7 as c_int;
        h2 = h2.wrapping_add(h2 << 2 as c_int);
        i = i.wrapping_add(1);
    }
    *pplen = i;
    h1 = h1.wrapping_add(':' as i32 as c_uint);
    h1 = h1.wrapping_add(h1 << 3 as c_int);
    h2 = h2.wrapping_add(h1);
    h2 = h2 << 7 as c_int
        | (h2 & 0xffffffff as c_uint)
            >> 32 as c_int - 7 as c_int;
    h2 = h2.wrapping_add(h2 << 2 as c_int);
    i = 0 as size_t;
    while *name.offset(i as isize) as c_int != 0 as c_int {
        h1 = h1.wrapping_add(*name.offset(i as isize) as c_uint);
        h1 = h1.wrapping_add(h1 << 3 as c_int);
        h2 = h2.wrapping_add(h1);
        h2 = h2 << 7 as c_int
            | (h2 & 0xffffffff as c_uint)
                >> 32 as c_int - 7 as c_int;
        h2 = h2.wrapping_add(h2 << 2 as c_int);
        i = i.wrapping_add(1);
    }
    *plen = i;
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 14 as c_int
            | (h2 & 0xffffffff as c_uint)
                >> 32 as c_int - 14 as c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as c_uint) >> 6 as c_int
            | h1 << 32 as c_int - 6 as c_int,
    );
    h1 ^= h2;
    h1 = h1.wrapping_add(
        h2 << 5 as c_int
            | (h2 & 0xffffffff as c_uint)
                >> 32 as c_int - 5 as c_int,
    );
    h2 ^= h1;
    h2 = h2.wrapping_add(
        (h1 & 0xffffffff as c_uint) >> 8 as c_int
            | h1 << 32 as c_int - 8 as c_int,
    );
    h2 &= 0xffffffff as c_uint;
    return h2 | MAX_HASH_SIZE;
}
#[inline]
pub unsafe fn xmlDictComputeHash(
    mut dict: *const xmlDict,
    mut string: *const xmlChar,
) -> c_uint {
    let mut len: size_t = 0;
    return xmlDictHashName((*dict).seed, string, SIZE_MAX, &raw mut len);
}
#[inline]
pub fn xmlDictCombineHash(
    mut v1: c_uint,
    mut v2: c_uint,
) -> c_uint { {
    v1 ^= v2;
    v1 = v1.wrapping_add(
        v2 << 5 as c_int
            | (v2 & 0x7fffffff as c_int as c_uint)
                >> 31 as c_int - 5 as c_int,
    );
    return v1 & 0xffffffff as c_uint | 0x80000000 as c_uint;
} }
unsafe fn xmlDictFindEntry(
    mut dict: *const xmlDict,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut len: c_int,
    mut hashValue: c_uint,
    mut pfound: *mut c_int,
) -> *mut xmlDictEntry {
    let mut entry: *mut xmlDictEntry = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut mask: c_uint = 0;
    let mut pos: c_uint = 0;
    let mut displ: c_uint = 0;
    let mut found: c_int = 0 as c_int;
    mask = (*dict).size.wrapping_sub(1 as size_t) as c_uint;
    pos = hashValue & mask;
    entry = (*dict).table.offset(pos as isize) as *mut xmlDictEntry;
    if (*entry).hashValue != 0 as c_uint {
        displ = 0 as c_uint;
        loop {
            if (*entry).hashValue == hashValue {
                if prefix.is_null() {
                    if strncmp(
                        (*entry).name as *const c_char,
                        name as *const c_char,
                        len as size_t,
                    ) == 0 as c_int
                        && *(*entry).name.offset(len as isize) as c_int
                            == 0 as c_int
                    {
                        found = 1 as c_int;
                        break;
                    }
                } else if xmlStrQEqual(prefix, name, (*entry).name) != 0 {
                    found = 1 as c_int;
                    break;
                }
            }
            displ = displ.wrapping_add(1);
            pos = pos.wrapping_add(1);
            entry = entry.offset(1);
            if pos & mask == 0 as c_uint {
                entry = (*dict).table;
            }
            if !((*entry).hashValue != 0 as c_uint
                && pos.wrapping_sub((*entry).hashValue) & mask >= displ)
            {
                break;
            }
        }
    }
    *pfound = found;
    return entry;
}
fn xmlDictGrow(
    mut dict: xmlDictPtr,
    mut size: c_uint,
) -> c_int { unsafe {
    let mut oldentry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    let mut oldend: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    let mut end: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    let mut table: *mut xmlDictEntry = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut oldsize: c_uint = 0;
    let mut i: c_uint = 0;
    if (size as size_t).wrapping_add(0 as size_t)
        > SIZE_MAX.wrapping_div(::core::mem::size_of::<xmlDictEntry>() as size_t)
    {
        return -(1 as c_int);
    }
    table = xmlMalloc.expect("non-null function pointer")(
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
    ) as *mut xmlDictEntry;
    if table.is_null() {
        return -(1 as c_int);
    }
    memset(
        table as *mut c_void,
        0 as c_int,
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
    );
    oldsize = (*dict).size as c_uint;
    if !(oldsize == 0 as c_uint) {
        oldend = (*dict).table.offset(oldsize as isize) as *mut xmlDictEntry;
        end = table.offset(size as isize) as *mut xmlDictEntry;
        oldentry = (*dict).table;
        while (*oldentry).hashValue != 0 as c_uint {
            oldentry = oldentry.offset(1);
            if oldentry >= oldend {
                oldentry = (*dict).table;
            }
        }
        i = 0 as c_uint;
        while i < oldsize {
            if (*oldentry).hashValue != 0 as c_uint {
                let mut entry: *mut xmlDictEntry = table.offset(
                    ((*oldentry).hashValue & size.wrapping_sub(1 as c_uint)) as isize,
                ) as *mut xmlDictEntry;
                while (*entry).hashValue != 0 as c_uint {
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
        xmlFree.expect("non-null function pointer")((*dict).table as *mut c_void);
    }
    (*dict).table = table;
    (*dict).size = size as size_t;
    return 0 as c_int;
} }
unsafe fn xmlDictLookupInternal(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut maybeLen: c_int,
    mut update: c_int,
) -> *const xmlDictEntry {
    let mut entry: *mut xmlDictEntry = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut hashValue: c_uint = 0;
    let mut maxLen: size_t = 0;
    let mut len: size_t = 0;
    let mut plen: size_t = 0;
    let mut klen: size_t = 0;
    let mut found: c_int = 0 as c_int;
    if dict.is_null() || name.is_null() {
        return ::core::ptr::null::<xmlDictEntry>();
    }
    maxLen = if maybeLen < 0 as c_int {
        SIZE_MAX
    } else {
        maybeLen as size_t
    };
    if prefix.is_null() {
        hashValue = xmlDictHashName((*dict).seed, name, maxLen, &raw mut len);
        if len > (INT_MAX / 2 as c_int) as size_t {
            return ::core::ptr::null::<xmlDictEntry>();
        }
        klen = len;
    } else {
        hashValue = xmlDictHashQName((*dict).seed, prefix, name, &raw mut plen, &raw mut len);
        if len > (INT_MAX / 2 as c_int) as size_t
            || plen >= ((INT_MAX / 2 as c_int) as size_t).wrapping_sub(len)
        {
            return ::core::ptr::null::<xmlDictEntry>();
        }
        klen = plen.wrapping_add(1 as size_t).wrapping_add(len);
    }
    if (*dict).limit > 0 as size_t && klen >= (*dict).limit {
        return ::core::ptr::null::<xmlDictEntry>();
    }
    if (*dict).size > 0 as size_t {
        entry = xmlDictFindEntry(
            dict as *const xmlDict,
            prefix,
            name,
            klen as c_int,
            hashValue,
            &raw mut found,
        );
    }
    if found != 0 {
        return entry;
    }
    if !(*dict).subdict.is_null() && (*(*dict).subdict).size > 0 as size_t {
        let mut subEntry: *mut xmlDictEntry = ::core::ptr::null_mut::<xmlDictEntry>();
        let mut subHashValue: c_uint = 0;
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
            klen as c_int,
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
    if (*dict).nbElems.wrapping_add(1 as c_uint) as size_t
        > (*dict)
            .size
            .wrapping_div(MAX_FILL_DENOM as size_t)
            .wrapping_mul(MAX_FILL_NUM as size_t)
    {
        let mut newSize: c_uint = 0;
        let mut mask: c_uint = 0;
        let mut displ: c_uint = 0;
        let mut pos: c_uint = 0;
        if (*dict).size == 0 as size_t {
            newSize = MIN_HASH_SIZE as c_uint;
        } else {
            if (*dict).size >= MAX_HASH_SIZE as size_t {
                return ::core::ptr::null::<xmlDictEntry>();
            }
            newSize = (*dict).size.wrapping_mul(2 as size_t) as c_uint;
        }
        if xmlDictGrow(dict, newSize) != 0 as c_int {
            return ::core::ptr::null::<xmlDictEntry>();
        }
        mask = (*dict).size.wrapping_sub(1 as size_t) as c_uint;
        displ = 0 as c_uint;
        pos = hashValue & mask;
        entry = (*dict).table.offset(pos as isize) as *mut xmlDictEntry;
        while (*entry).hashValue != 0 as c_uint
            && pos.wrapping_sub((*entry).hashValue) & mask >= displ
        {
            displ = displ.wrapping_add(1);
            pos = pos.wrapping_add(1);
            entry = entry.offset(1);
            if pos & mask == 0 as c_uint {
                entry = (*dict).table;
            }
        }
    }
    if prefix.is_null() {
        ret = xmlDictAddString(dict, name, len as c_uint);
    } else {
        ret = xmlDictAddQString(
            dict,
            prefix,
            plen as c_uint,
            name,
            len as c_uint,
        );
    }
    if ret.is_null() {
        return ::core::ptr::null::<xmlDictEntry>();
    }
    if (*entry).hashValue != 0 as c_uint {
        let mut end: *const xmlDictEntry =
            (*dict).table.offset((*dict).size as isize) as *mut xmlDictEntry;
        let mut cur: *const xmlDictEntry = entry;
        loop {
            cur = cur.offset(1);
            if cur >= end {
                cur = (*dict).table;
            }
            if !((*cur).hashValue != 0 as c_uint) {
                break;
            }
        }
        if cur < entry as *const xmlDictEntry {
            memmove(
                (*dict).table.offset(1 as c_int as isize) as *mut xmlDictEntry
                    as *mut c_void,
                (*dict).table as *const c_void,
                (cur as *mut c_char)
                    .offset_from((*dict).table as *mut c_char)
                    as c_long as size_t,
            );
            cur = end.offset(-(1 as c_int as isize));
            *(*dict).table.offset(0 as c_int as isize) = *cur;
        }
        memmove(
            entry.offset(1 as c_int as isize) as *mut xmlDictEntry
                as *mut c_void,
            entry as *const c_void,
            (cur as *mut c_char).offset_from(entry as *mut c_char)
                as c_long as size_t,
        );
    }
    (*entry).hashValue = hashValue;
    (*entry).name = ret;
    (*dict).nbElems = (*dict).nbElems.wrapping_add(1);
    return entry;
}
#[inline]
pub unsafe fn xmlDictLookup(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: c_int,
) -> *const xmlChar {
    let mut entry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    entry = xmlDictLookupInternal(
        dict,
        ::core::ptr::null::<xmlChar>(),
        name,
        len,
        1 as c_int,
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
    mut len: c_int,
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
        1 as c_int,
    );
    if entry.is_null() {
        ret.name = ::core::ptr::null::<xmlChar>();
        ret.hashValue = 0 as c_uint;
    } else {
        ret = *entry as xmlHashedString;
    }
    return ret;
}
#[inline]
pub unsafe fn xmlDictExists(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: c_int,
) -> *const xmlChar {
    let mut entry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    entry = xmlDictLookupInternal(
        dict,
        ::core::ptr::null::<xmlChar>(),
        name,
        len,
        0 as c_int,
    );
    if entry.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return (*entry).name;
}
#[inline]
pub unsafe fn xmlDictQLookup(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
) -> *const xmlChar {
    let mut entry: *const xmlDictEntry = ::core::ptr::null::<xmlDictEntry>();
    entry = xmlDictLookupInternal(
        dict,
        prefix,
        name,
        -(1 as c_int),
        1 as c_int,
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
static mut globalRngState: [c_uint; 2] = [0; 2];
#[no_mangle]
pub extern "C" fn xmlInitRandom() { unsafe {
    let mut var: c_int = 0;
    xmlInitMutex(&raw mut xmlRngMutex);
    globalRngState[0 as c_int as usize] = time(::core::ptr::null_mut::<time_t>())
        as c_uint
        ^ ((::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, size_t>(Some(
            xmlInitRandom as unsafe extern "C" fn() -> (),
        )) as c_uint)
            << 8 as c_int
            | (::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, size_t>(Some(
                xmlInitRandom as unsafe extern "C" fn() -> (),
            )) as c_uint
                & 0xffffffff as c_uint)
                >> 32 as c_int - 8 as c_int);
    globalRngState[1 as c_int as usize] = ((&raw mut xmlRngMutex as size_t
        as c_uint)
        << 16 as c_int
        | (&raw mut xmlRngMutex as size_t as c_uint
            & 0xffffffff as c_uint)
            >> 32 as c_int - 16 as c_int)
        ^ ((&raw mut var as size_t as c_uint) << 24 as c_int
            | (&raw mut var as size_t as c_uint & 0xffffffff as c_uint)
                >> 32 as c_int - 24 as c_int);
} }
#[inline]
pub fn xmlCleanupRandom() { unsafe {
    xmlCleanupMutex(&raw mut xmlRngMutex);
} }
unsafe fn xoroshiro64ss(mut s: *mut c_uint) -> c_uint {
    let mut s0: c_uint = *s.offset(0 as c_int as isize);
    let mut s1: c_uint = *s.offset(1 as c_int as isize);
    let mut result: c_uint = (s0.wrapping_mul(0x9e3779bb as c_uint)
        << 5 as c_int
        | (s0.wrapping_mul(0x9e3779bb as c_uint) & 0xffffffff as c_uint)
            >> 32 as c_int - 5 as c_int)
        .wrapping_mul(5 as c_uint);
    s1 ^= s0;
    *s.offset(0 as c_int as isize) = (s0 << 26 as c_int
        | (s0 & 0xffffffff as c_uint)
            >> 32 as c_int - 26 as c_int)
        ^ s1
        ^ s1 << 9 as c_int;
    *s.offset(1 as c_int as isize) = s1 << 13 as c_int
        | (s1 & 0xffffffff as c_uint)
            >> 32 as c_int - 13 as c_int;
    return result & 0xffffffff as c_uint;
}
#[inline]
pub fn xmlRandom() -> c_uint { unsafe {
    let mut ret: c_uint = 0;
    xmlMutexLock(&raw mut xmlRngMutex);
    ret = xoroshiro64ss(&raw mut globalRngState as *mut c_uint);
    xmlMutexUnlock(&raw mut xmlRngMutex);
    return ret;
} }

