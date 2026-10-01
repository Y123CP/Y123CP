use core::ffi::*;
use crate::src::globals::__xmlGenericError;
use crate::src::globals::__xmlGenericErrorContext;
use crate::src::threads::xmlInitParser;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
extern "C" {
    fn xmlInitMutex(mutex: xmlMutexPtr);
    fn xmlCleanupMutex(mutex: xmlMutexPtr);
    fn xmlMutexLock(tok: xmlMutexPtr);
    fn xmlMutexUnlock(tok: xmlMutexPtr);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40],
    pub __align: c_long,
}

pub type MEMHDR = memnod;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memnod {
    pub mh_tag: c_uint,
    pub mh_type: c_uint,
    pub mh_number: c_ulong,
    pub mh_size: size_t,
    pub mh_file: *const c_char,
    pub mh_line: c_uint,
}
pub type xmlMutex = _xmlMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlMutex {
    pub lock: pthread_mutex_t,
}
pub type xmlMutexPtr = *mut xmlMutex;

static mut debugMemSize: c_ulong = 0 as c_ulong;
static mut debugMemBlocks: c_ulong = 0 as c_ulong;
static mut debugMaxMemSize: c_ulong = 0 as c_ulong;
static mut xmlMemMutex: xmlMutex = xmlMutex {
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
pub const MEMTAG: c_uint = 0x5aa5 as c_uint;
pub const MALLOC_TYPE: c_int = 1 as c_int;
pub const REALLOC_TYPE: c_int = 2 as c_int;
pub const STRDUP_TYPE: c_int = 3 as c_int;
pub const MALLOC_ATOMIC_TYPE: c_int = 4 as c_int;
pub const ALIGN_SIZE: usize = ::core::mem::size_of::<c_double>();
pub const HDR_SIZE: usize = ::core::mem::size_of::<MEMHDR>();
pub const RESERVE_SIZE: usize = HDR_SIZE
    .wrapping_add(ALIGN_SIZE.wrapping_sub(1 as usize))
    .wrapping_div(ALIGN_SIZE)
    .wrapping_mul(ALIGN_SIZE);
pub const MAX_SIZE_T: size_t = -(1 as c_int) as size_t;
static mut block: c_uint = 0 as c_uint;
static mut xmlMemStopAtBlock: c_uint = 0 as c_uint;
static mut xmlMemTraceBlockAt: *mut c_void = NULL;
#[inline]
pub fn xmlMallocBreakpoint() { unsafe {
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"xmlMallocBreakpoint reached on block %d\n\0" as *const u8 as *const c_char,
        xmlMemStopAtBlock,
    );
} }
#[inline]
pub unsafe fn xmlMallocLoc(
    mut size: size_t,
    mut file: *const c_char,
    mut line: c_int,
) -> *mut c_void {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut ret: *mut c_void = ::core::ptr::null_mut::<c_void>();
    xmlInitParser();
    if size > MAX_SIZE_T.wrapping_sub(RESERVE_SIZE) {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMallocLoc : Unsigned overflow\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<c_void>();
    }
    p = malloc(RESERVE_SIZE.wrapping_add(size)) as *mut MEMHDR;
    if p.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMallocLoc : Out of free space\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<c_void>();
    }
    (*p).mh_tag = MEMTAG;
    (*p).mh_size = size;
    (*p).mh_type = MALLOC_TYPE as c_uint;
    (*p).mh_file = file;
    (*p).mh_line = line as c_uint;
    xmlMutexLock(&raw mut xmlMemMutex);
    block = block.wrapping_add(1);
    (*p).mh_number = block as c_ulong;
    debugMemSize = debugMemSize.wrapping_add(size as c_ulong);
    debugMemBlocks = debugMemBlocks.wrapping_add(1);
    if debugMemSize > debugMaxMemSize {
        debugMaxMemSize = debugMemSize;
    }
    xmlMutexUnlock(&raw mut xmlMemMutex);
    if xmlMemStopAtBlock as c_ulong == (*p).mh_number {
        xmlMallocBreakpoint();
    }
    ret = (p as *mut c_char).offset(RESERVE_SIZE as isize) as *mut c_void;
    if xmlMemTraceBlockAt == ret {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"%p : Malloc(%lu) Ok\n\0" as *const u8 as *const c_char,
            xmlMemTraceBlockAt,
            size as c_ulong,
        );
        xmlMallocBreakpoint();
    }
    return ret;
}
#[inline]
pub unsafe fn xmlMallocAtomicLoc(
    mut size: size_t,
    mut file: *const c_char,
    mut line: c_int,
) -> *mut c_void {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut ret: *mut c_void = ::core::ptr::null_mut::<c_void>();
    xmlInitParser();
    if size > MAX_SIZE_T.wrapping_sub(RESERVE_SIZE) {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMallocAtomicLoc : Unsigned overflow\n\0" as *const u8
                as *const c_char,
        );
        return ::core::ptr::null_mut::<c_void>();
    }
    p = malloc(RESERVE_SIZE.wrapping_add(size)) as *mut MEMHDR;
    if p.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMallocAtomicLoc : Out of free space\n\0" as *const u8
                as *const c_char,
        );
        return ::core::ptr::null_mut::<c_void>();
    }
    (*p).mh_tag = MEMTAG;
    (*p).mh_size = size;
    (*p).mh_type = MALLOC_ATOMIC_TYPE as c_uint;
    (*p).mh_file = file;
    (*p).mh_line = line as c_uint;
    xmlMutexLock(&raw mut xmlMemMutex);
    block = block.wrapping_add(1);
    (*p).mh_number = block as c_ulong;
    debugMemSize = debugMemSize.wrapping_add(size as c_ulong);
    debugMemBlocks = debugMemBlocks.wrapping_add(1);
    if debugMemSize > debugMaxMemSize {
        debugMaxMemSize = debugMemSize;
    }
    xmlMutexUnlock(&raw mut xmlMemMutex);
    if xmlMemStopAtBlock as c_ulong == (*p).mh_number {
        xmlMallocBreakpoint();
    }
    ret = (p as *mut c_char).offset(RESERVE_SIZE as isize) as *mut c_void;
    if xmlMemTraceBlockAt == ret {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"%p : Malloc(%lu) Ok\n\0" as *const u8 as *const c_char,
            xmlMemTraceBlockAt,
            size as c_ulong,
        );
        xmlMallocBreakpoint();
    }
    return ret;
}
#[inline]
pub fn xmlMemMalloc(mut size: size_t) -> *mut c_void { unsafe {
    return xmlMallocLoc(
        size,
        b"none\0" as *const u8 as *const c_char,
        0 as c_int,
    );
} }
#[inline]
pub unsafe fn xmlReallocLoc(
    mut ptr: *mut c_void,
    mut size: size_t,
    mut file: *const c_char,
    mut line: c_int,
) -> *mut c_void {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut tmp: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut number: c_ulong = 0;
    if ptr.is_null() {
        return xmlMallocLoc(size, file, line);
    }
    xmlInitParser();
    p = (ptr as *mut c_char).offset(-(RESERVE_SIZE as isize))
        as *mut c_void as *mut MEMHDR;
    number = (*p).mh_number;
    if xmlMemStopAtBlock as c_ulong == number {
        xmlMallocBreakpoint();
    }
    if (*p).mh_tag != MEMTAG {
        debugmem_tag_error(p as *mut c_void);
    } else {
        (*p).mh_tag = !MEMTAG;
        xmlMutexLock(&raw mut xmlMemMutex);
        debugMemSize = debugMemSize.wrapping_sub((*p).mh_size as c_ulong);
        debugMemBlocks = debugMemBlocks.wrapping_sub(1);
        xmlMutexUnlock(&raw mut xmlMemMutex);
        if size > MAX_SIZE_T.wrapping_sub(RESERVE_SIZE) {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlReallocLoc : Unsigned overflow\n\0" as *const u8 as *const c_char,
            );
            return ::core::ptr::null_mut::<c_void>();
        }
        tmp = realloc(
            p as *mut c_void,
            RESERVE_SIZE.wrapping_add(size),
        ) as *mut MEMHDR;
        if tmp.is_null() {
            free(p as *mut c_void);
        } else {
            p = tmp;
            if xmlMemTraceBlockAt == ptr {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"%p : Realloced(%lu -> %lu) Ok\n\0" as *const u8 as *const c_char,
                    xmlMemTraceBlockAt,
                    (*p).mh_size as c_ulong,
                    size as c_ulong,
                );
                xmlMallocBreakpoint();
            }
            (*p).mh_tag = MEMTAG;
            (*p).mh_number = number;
            (*p).mh_type = REALLOC_TYPE as c_uint;
            (*p).mh_size = size;
            (*p).mh_file = file;
            (*p).mh_line = line as c_uint;
            xmlMutexLock(&raw mut xmlMemMutex);
            debugMemSize = debugMemSize.wrapping_add(size as c_ulong);
            debugMemBlocks = debugMemBlocks.wrapping_add(1);
            if debugMemSize > debugMaxMemSize {
                debugMaxMemSize = debugMemSize;
            }
            xmlMutexUnlock(&raw mut xmlMemMutex);
            return (p as *mut c_char).offset(RESERVE_SIZE as isize)
                as *mut c_void;
        }
    }
    return ::core::ptr::null_mut::<c_void>();
}
#[inline]
pub unsafe fn xmlMemRealloc(
    mut ptr: *mut c_void,
    mut size: size_t,
) -> *mut c_void {
    return xmlReallocLoc(
        ptr,
        size,
        b"none\0" as *const u8 as *const c_char,
        0 as c_int,
    );
}
#[inline]
pub unsafe fn xmlMemFree(mut ptr: *mut c_void) {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut target: *mut c_char = ::core::ptr::null_mut::<c_char>();
    if ptr.is_null() {
        return;
    }
    if ptr == -(1 as c_int) as *mut c_void {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"trying to free pointer from freed area\n\0" as *const u8
                as *const c_char,
        );
    } else {
        if xmlMemTraceBlockAt == ptr {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"%p : Freed()\n\0" as *const u8 as *const c_char,
                xmlMemTraceBlockAt,
            );
            xmlMallocBreakpoint();
        }
        target = ptr as *mut c_char;
        p = (ptr as *mut c_char).offset(-(RESERVE_SIZE as isize))
            as *mut c_void as *mut MEMHDR;
        if (*p).mh_tag != MEMTAG {
            debugmem_tag_error(p as *mut c_void);
        } else {
            if xmlMemStopAtBlock as c_ulong == (*p).mh_number {
                xmlMallocBreakpoint();
            }
            (*p).mh_tag = !MEMTAG;
            memset(
                target as *mut c_void,
                -(1 as c_int),
                (*p).mh_size,
            );
            xmlMutexLock(&raw mut xmlMemMutex);
            debugMemSize = debugMemSize.wrapping_sub((*p).mh_size as c_ulong);
            debugMemBlocks = debugMemBlocks.wrapping_sub(1);
            xmlMutexUnlock(&raw mut xmlMemMutex);
            free(p as *mut c_void);
            return;
        }
    }
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"xmlMemFree(%p) error\n\0" as *const u8 as *const c_char,
        ptr,
    );
    xmlMallocBreakpoint();
}
#[inline]
pub unsafe fn xmlMemStrdupLoc(
    mut str: *const c_char,
    mut file: *const c_char,
    mut line: c_int,
) -> *mut c_char {
    let mut s: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut size: size_t = strlen(str).wrapping_add(1 as size_t);
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    xmlInitParser();
    if size > MAX_SIZE_T.wrapping_sub(RESERVE_SIZE) {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMemStrdupLoc : Unsigned overflow\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<c_char>();
    }
    p = malloc(RESERVE_SIZE.wrapping_add(size)) as *mut MEMHDR;
    if p.is_null() {
        return ::core::ptr::null_mut::<c_char>();
    } else {
        (*p).mh_tag = MEMTAG;
        (*p).mh_size = size;
        (*p).mh_type = STRDUP_TYPE as c_uint;
        (*p).mh_file = file;
        (*p).mh_line = line as c_uint;
        xmlMutexLock(&raw mut xmlMemMutex);
        block = block.wrapping_add(1);
        (*p).mh_number = block as c_ulong;
        debugMemSize = debugMemSize.wrapping_add(size as c_ulong);
        debugMemBlocks = debugMemBlocks.wrapping_add(1);
        if debugMemSize > debugMaxMemSize {
            debugMaxMemSize = debugMemSize;
        }
        xmlMutexUnlock(&raw mut xmlMemMutex);
        s = (p as *mut c_char).offset(RESERVE_SIZE as isize)
            as *mut c_void as *mut c_char;
        if xmlMemStopAtBlock as c_ulong == (*p).mh_number {
            xmlMallocBreakpoint();
        }
        strcpy(s, str);
        if xmlMemTraceBlockAt == s as *mut c_void {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"%p : Strdup() Ok\n\0" as *const u8 as *const c_char,
                xmlMemTraceBlockAt,
            );
            xmlMallocBreakpoint();
        }
        return s;
    };
}
#[inline]
pub unsafe fn xmlMemoryStrdup(
    mut str: *const c_char,
) -> *mut c_char {
    return xmlMemStrdupLoc(
        str,
        b"none\0" as *const u8 as *const c_char,
        0 as c_int,
    );
}
#[inline]
pub unsafe fn xmlMemSize(mut ptr: *mut c_void) -> size_t {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    if ptr.is_null() {
        return 0 as size_t;
    }
    p = (ptr as *mut c_char).offset(-(RESERVE_SIZE as isize))
        as *mut c_void as *mut MEMHDR;
    if (*p).mh_tag != MEMTAG {
        return 0 as size_t;
    }
    return (*p).mh_size;
}
#[inline]
pub fn xmlMemUsed() -> c_int { unsafe {
    return debugMemSize as c_int;
} }
#[inline]
pub fn xmlMemBlocks() -> c_int { unsafe {
    let mut res: c_int = 0;
    xmlMutexLock(&raw mut xmlMemMutex);
    res = debugMemBlocks as c_int;
    xmlMutexUnlock(&raw mut xmlMemMutex);
    return res;
} }
#[inline]
pub unsafe fn xmlMemDisplayLast(mut fp: *mut FILE, mut nbBytes: c_long) {
    let mut old_fp: *mut FILE = fp;
    if nbBytes <= 0 as c_long {
        return;
    }
    if fp.is_null() {
        fp = fopen(
            b".memorylist\0" as *const u8 as *const c_char,
            b"w\0" as *const u8 as *const c_char,
        );
        if fp.is_null() {
            return;
        }
    }
    fprintf(
        fp,
        b"Memory list not compiled (MEM_LIST not defined !)\n\0" as *const u8
            as *const c_char,
    );
    if old_fp.is_null() {
        fclose(fp);
    }
}
#[inline]
pub unsafe fn xmlMemDisplay(mut fp: *mut FILE) {
    let mut old_fp: *mut FILE = fp;
    if fp.is_null() {
        fp = fopen(
            b".memorylist\0" as *const u8 as *const c_char,
            b"w\0" as *const u8 as *const c_char,
        );
        if fp.is_null() {
            return;
        }
    }
    fprintf(
        fp,
        b"Memory list not compiled (MEM_LIST not defined !)\n\0" as *const u8
            as *const c_char,
    );
    if old_fp.is_null() {
        fclose(fp);
    }
}
unsafe fn debugmem_tag_error(mut p: *mut c_void) {
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"Memory tag error occurs :%p \n\t bye\n\0" as *const u8 as *const c_char,
        p,
    );
}
#[inline]
pub unsafe fn xmlMemShow(mut fp: *mut FILE, mut nr: c_int) {
    if !fp.is_null() {
        fprintf(
            fp,
            b"      MEMORY ALLOCATED : %lu, MAX was %lu\n\0" as *const u8
                as *const c_char,
            debugMemSize,
            debugMaxMemSize,
        );
    }
}
#[inline]
pub fn xmlMemoryDump() { {} }
#[inline]
pub fn xmlInitMemory() -> c_int { {
    xmlInitParser();
    return 0 as c_int;
} }
#[inline]
pub fn xmlInitMemoryInternal() { unsafe {
    let mut breakpoint: *mut c_char = ::core::ptr::null_mut::<c_char>();
    xmlInitMutex(&raw mut xmlMemMutex);
    breakpoint = getenv(b"XML_MEM_BREAKPOINT\0" as *const u8 as *const c_char);
    if !breakpoint.is_null() {
        sscanf(
            breakpoint,
            b"%ud\0" as *const u8 as *const c_char,
            &raw mut xmlMemStopAtBlock,
        );
    }
    breakpoint = getenv(b"XML_MEM_TRACE\0" as *const u8 as *const c_char);
    if !breakpoint.is_null() {
        sscanf(
            breakpoint,
            b"%p\0" as *const u8 as *const c_char,
            &raw mut xmlMemTraceBlockAt,
        );
    }
} }
#[inline]
pub fn xmlCleanupMemory() { {} }
#[inline]
pub fn xmlCleanupMemoryInternal() { unsafe {
    xmlCleanupMutex(&raw mut xmlMemMutex);
} }
#[inline]
pub fn xmlMemSetup(
    mut freeFunc: xmlFreeFunc,
    mut mallocFunc: xmlMallocFunc,
    mut reallocFunc: xmlReallocFunc,
    mut strdupFunc: xmlStrdupFunc,
) -> c_int { unsafe {
    if freeFunc.is_none() {
        return -(1 as c_int);
    }
    if mallocFunc.is_none() {
        return -(1 as c_int);
    }
    if reallocFunc.is_none() {
        return -(1 as c_int);
    }
    if strdupFunc.is_none() {
        return -(1 as c_int);
    }
    xmlFree = freeFunc;
    xmlMalloc = mallocFunc;
    xmlMallocAtomic = mallocFunc;
    xmlRealloc = reallocFunc;
    xmlMemStrdup = strdupFunc;
    return 0 as c_int;
} }
#[inline]
pub unsafe fn xmlMemGet(
    mut freeFunc: *mut xmlFreeFunc,
    mut mallocFunc: *mut xmlMallocFunc,
    mut reallocFunc: *mut xmlReallocFunc,
    mut strdupFunc: *mut xmlStrdupFunc,
) -> c_int {
    if !freeFunc.is_null() {
        *freeFunc = xmlFree;
    }
    if !mallocFunc.is_null() {
        *mallocFunc = xmlMalloc;
    }
    if !reallocFunc.is_null() {
        *reallocFunc = xmlRealloc;
    }
    if !strdupFunc.is_null() {
        *strdupFunc = xmlMemStrdup;
    }
    return 0 as c_int;
}
#[inline]
pub fn xmlGcMemSetup(
    mut freeFunc: xmlFreeFunc,
    mut mallocFunc: xmlMallocFunc,
    mut mallocAtomicFunc: xmlMallocFunc,
    mut reallocFunc: xmlReallocFunc,
    mut strdupFunc: xmlStrdupFunc,
) -> c_int { unsafe {
    if freeFunc.is_none() {
        return -(1 as c_int);
    }
    if mallocFunc.is_none() {
        return -(1 as c_int);
    }
    if mallocAtomicFunc.is_none() {
        return -(1 as c_int);
    }
    if reallocFunc.is_none() {
        return -(1 as c_int);
    }
    if strdupFunc.is_none() {
        return -(1 as c_int);
    }
    xmlFree = freeFunc;
    xmlMalloc = mallocFunc;
    xmlMallocAtomic = mallocAtomicFunc;
    xmlRealloc = reallocFunc;
    xmlMemStrdup = strdupFunc;
    return 0 as c_int;
} }
#[inline]
pub unsafe fn xmlGcMemGet(
    mut freeFunc: *mut xmlFreeFunc,
    mut mallocFunc: *mut xmlMallocFunc,
    mut mallocAtomicFunc: *mut xmlMallocFunc,
    mut reallocFunc: *mut xmlReallocFunc,
    mut strdupFunc: *mut xmlStrdupFunc,
) -> c_int {
    if !freeFunc.is_null() {
        *freeFunc = xmlFree;
    }
    if !mallocFunc.is_null() {
        *mallocFunc = xmlMalloc;
    }
    if !mallocAtomicFunc.is_null() {
        *mallocAtomicFunc = xmlMallocAtomic;
    }
    if !reallocFunc.is_null() {
        *reallocFunc = xmlRealloc;
    }
    if !strdupFunc.is_null() {
        *strdupFunc = xmlMemStrdup;
    }
    return 0 as c_int;
}
