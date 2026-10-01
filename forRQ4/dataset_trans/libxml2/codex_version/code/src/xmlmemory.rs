extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlMallocAtomic: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    static mut xmlMemStrdup: xmlStrdupFunc;
    fn xmlInitParser();
    fn xmlInitMutex(mutex: xmlMutexPtr);
    fn xmlCleanupMutex(mutex: xmlMutexPtr);
    fn xmlMutexLock(tok: xmlMutexPtr);
    fn xmlMutexUnlock(tok: xmlMutexPtr);
    fn __xmlGenericErrorContext() -> *mut *mut ::core::ffi::c_void;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
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
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type xmlStrdupFunc =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char>;
pub type MEMHDR = memnod;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memnod {
    pub mh_tag: ::core::ffi::c_uint,
    pub mh_type: ::core::ffi::c_uint,
    pub mh_number: ::core::ffi::c_ulong,
    pub mh_size: size_t,
    pub mh_file: *const ::core::ffi::c_char,
    pub mh_line: ::core::ffi::c_uint,
}
pub type xmlMutex = _xmlMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlMutex {
    pub lock: pthread_mutex_t,
}
pub type xmlMutexPtr = *mut xmlMutex;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut debugMemSize: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
static mut debugMemBlocks: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
static mut debugMaxMemSize: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
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
pub const MEMTAG: ::core::ffi::c_uint = 0x5aa5 as ::core::ffi::c_uint;
pub const MALLOC_TYPE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const REALLOC_TYPE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STRDUP_TYPE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MALLOC_ATOMIC_TYPE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ALIGN_SIZE: usize = ::core::mem::size_of::<::core::ffi::c_double>();
pub const HDR_SIZE: usize = ::core::mem::size_of::<MEMHDR>();
pub const RESERVE_SIZE: usize = HDR_SIZE
    .wrapping_add(ALIGN_SIZE.wrapping_sub(1 as usize))
    .wrapping_div(ALIGN_SIZE)
    .wrapping_mul(ALIGN_SIZE);
pub const MAX_SIZE_T: size_t = -(1 as ::core::ffi::c_int) as size_t;
static mut block: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
static mut xmlMemStopAtBlock: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
static mut xmlMemTraceBlockAt: *mut ::core::ffi::c_void = NULL;
#[no_mangle]
pub unsafe extern "C" fn xmlMallocBreakpoint() {
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"xmlMallocBreakpoint reached on block %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        xmlMemStopAtBlock,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlMallocLoc(
    mut size: size_t,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    xmlInitParser();
    if size > MAX_SIZE_T.wrapping_sub(RESERVE_SIZE) {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMallocLoc : Unsigned overflow\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    p = malloc(RESERVE_SIZE.wrapping_add(size)) as *mut MEMHDR;
    if p.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMallocLoc : Out of free space\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*p).mh_tag = MEMTAG;
    (*p).mh_size = size;
    (*p).mh_type = MALLOC_TYPE as ::core::ffi::c_uint;
    (*p).mh_file = file;
    (*p).mh_line = line as ::core::ffi::c_uint;
    xmlMutexLock(&raw mut xmlMemMutex);
    block = block.wrapping_add(1);
    (*p).mh_number = block as ::core::ffi::c_ulong;
    debugMemSize = debugMemSize.wrapping_add(size as ::core::ffi::c_ulong);
    debugMemBlocks = debugMemBlocks.wrapping_add(1);
    if debugMemSize > debugMaxMemSize {
        debugMaxMemSize = debugMemSize;
    }
    xmlMutexUnlock(&raw mut xmlMemMutex);
    if xmlMemStopAtBlock as ::core::ffi::c_ulong == (*p).mh_number {
        xmlMallocBreakpoint();
    }
    ret = (p as *mut ::core::ffi::c_char).offset(RESERVE_SIZE as isize) as *mut ::core::ffi::c_void;
    if xmlMemTraceBlockAt == ret {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"%p : Malloc(%lu) Ok\n\0" as *const u8 as *const ::core::ffi::c_char,
            xmlMemTraceBlockAt,
            size as ::core::ffi::c_ulong,
        );
        xmlMallocBreakpoint();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlMallocAtomicLoc(
    mut size: size_t,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    xmlInitParser();
    if size > MAX_SIZE_T.wrapping_sub(RESERVE_SIZE) {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMallocAtomicLoc : Unsigned overflow\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    p = malloc(RESERVE_SIZE.wrapping_add(size)) as *mut MEMHDR;
    if p.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMallocAtomicLoc : Out of free space\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*p).mh_tag = MEMTAG;
    (*p).mh_size = size;
    (*p).mh_type = MALLOC_ATOMIC_TYPE as ::core::ffi::c_uint;
    (*p).mh_file = file;
    (*p).mh_line = line as ::core::ffi::c_uint;
    xmlMutexLock(&raw mut xmlMemMutex);
    block = block.wrapping_add(1);
    (*p).mh_number = block as ::core::ffi::c_ulong;
    debugMemSize = debugMemSize.wrapping_add(size as ::core::ffi::c_ulong);
    debugMemBlocks = debugMemBlocks.wrapping_add(1);
    if debugMemSize > debugMaxMemSize {
        debugMaxMemSize = debugMemSize;
    }
    xmlMutexUnlock(&raw mut xmlMemMutex);
    if xmlMemStopAtBlock as ::core::ffi::c_ulong == (*p).mh_number {
        xmlMallocBreakpoint();
    }
    ret = (p as *mut ::core::ffi::c_char).offset(RESERVE_SIZE as isize) as *mut ::core::ffi::c_void;
    if xmlMemTraceBlockAt == ret {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"%p : Malloc(%lu) Ok\n\0" as *const u8 as *const ::core::ffi::c_char,
            xmlMemTraceBlockAt,
            size as ::core::ffi::c_ulong,
        );
        xmlMallocBreakpoint();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemMalloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    return xmlMallocLoc(
        size,
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlReallocLoc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut tmp: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut number: ::core::ffi::c_ulong = 0;
    if ptr.is_null() {
        return xmlMallocLoc(size, file, line);
    }
    xmlInitParser();
    p = (ptr as *mut ::core::ffi::c_char).offset(-(RESERVE_SIZE as isize))
        as *mut ::core::ffi::c_void as *mut MEMHDR;
    number = (*p).mh_number;
    if xmlMemStopAtBlock as ::core::ffi::c_ulong == number {
        xmlMallocBreakpoint();
    }
    if (*p).mh_tag != MEMTAG {
        debugmem_tag_error(p as *mut ::core::ffi::c_void);
    } else {
        (*p).mh_tag = !MEMTAG;
        xmlMutexLock(&raw mut xmlMemMutex);
        debugMemSize = debugMemSize.wrapping_sub((*p).mh_size as ::core::ffi::c_ulong);
        debugMemBlocks = debugMemBlocks.wrapping_sub(1);
        xmlMutexUnlock(&raw mut xmlMemMutex);
        if size > MAX_SIZE_T.wrapping_sub(RESERVE_SIZE) {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlReallocLoc : Unsigned overflow\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        tmp = realloc(
            p as *mut ::core::ffi::c_void,
            RESERVE_SIZE.wrapping_add(size),
        ) as *mut MEMHDR;
        if tmp.is_null() {
            free(p as *mut ::core::ffi::c_void);
        } else {
            p = tmp;
            if xmlMemTraceBlockAt == ptr {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"%p : Realloced(%lu -> %lu) Ok\n\0" as *const u8 as *const ::core::ffi::c_char,
                    xmlMemTraceBlockAt,
                    (*p).mh_size as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulong,
                );
                xmlMallocBreakpoint();
            }
            (*p).mh_tag = MEMTAG;
            (*p).mh_number = number;
            (*p).mh_type = REALLOC_TYPE as ::core::ffi::c_uint;
            (*p).mh_size = size;
            (*p).mh_file = file;
            (*p).mh_line = line as ::core::ffi::c_uint;
            xmlMutexLock(&raw mut xmlMemMutex);
            debugMemSize = debugMemSize.wrapping_add(size as ::core::ffi::c_ulong);
            debugMemBlocks = debugMemBlocks.wrapping_add(1);
            if debugMemSize > debugMaxMemSize {
                debugMaxMemSize = debugMemSize;
            }
            xmlMutexUnlock(&raw mut xmlMemMutex);
            return (p as *mut ::core::ffi::c_char).offset(RESERVE_SIZE as isize)
                as *mut ::core::ffi::c_void;
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemRealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    return xmlReallocLoc(
        ptr,
        size,
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemFree(mut ptr: *mut ::core::ffi::c_void) {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    let mut target: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ptr.is_null() {
        return;
    }
    if ptr == -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"trying to free pointer from freed area\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        if xmlMemTraceBlockAt == ptr {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"%p : Freed()\n\0" as *const u8 as *const ::core::ffi::c_char,
                xmlMemTraceBlockAt,
            );
            xmlMallocBreakpoint();
        }
        target = ptr as *mut ::core::ffi::c_char;
        p = (ptr as *mut ::core::ffi::c_char).offset(-(RESERVE_SIZE as isize))
            as *mut ::core::ffi::c_void as *mut MEMHDR;
        if (*p).mh_tag != MEMTAG {
            debugmem_tag_error(p as *mut ::core::ffi::c_void);
        } else {
            if xmlMemStopAtBlock as ::core::ffi::c_ulong == (*p).mh_number {
                xmlMallocBreakpoint();
            }
            (*p).mh_tag = !MEMTAG;
            memset(
                target as *mut ::core::ffi::c_void,
                -(1 as ::core::ffi::c_int),
                (*p).mh_size,
            );
            xmlMutexLock(&raw mut xmlMemMutex);
            debugMemSize = debugMemSize.wrapping_sub((*p).mh_size as ::core::ffi::c_ulong);
            debugMemBlocks = debugMemBlocks.wrapping_sub(1);
            xmlMutexUnlock(&raw mut xmlMemMutex);
            free(p as *mut ::core::ffi::c_void);
            return;
        }
    }
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"xmlMemFree(%p) error\n\0" as *const u8 as *const ::core::ffi::c_char,
        ptr,
    );
    xmlMallocBreakpoint();
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemStrdupLoc(
    mut str: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut size: size_t = strlen(str).wrapping_add(1 as size_t);
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    xmlInitParser();
    if size > MAX_SIZE_T.wrapping_sub(RESERVE_SIZE) {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlMemStrdupLoc : Unsigned overflow\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = malloc(RESERVE_SIZE.wrapping_add(size)) as *mut MEMHDR;
    if p.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        (*p).mh_tag = MEMTAG;
        (*p).mh_size = size;
        (*p).mh_type = STRDUP_TYPE as ::core::ffi::c_uint;
        (*p).mh_file = file;
        (*p).mh_line = line as ::core::ffi::c_uint;
        xmlMutexLock(&raw mut xmlMemMutex);
        block = block.wrapping_add(1);
        (*p).mh_number = block as ::core::ffi::c_ulong;
        debugMemSize = debugMemSize.wrapping_add(size as ::core::ffi::c_ulong);
        debugMemBlocks = debugMemBlocks.wrapping_add(1);
        if debugMemSize > debugMaxMemSize {
            debugMaxMemSize = debugMemSize;
        }
        xmlMutexUnlock(&raw mut xmlMemMutex);
        s = (p as *mut ::core::ffi::c_char).offset(RESERVE_SIZE as isize)
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
        if xmlMemStopAtBlock as ::core::ffi::c_ulong == (*p).mh_number {
            xmlMallocBreakpoint();
        }
        strcpy(s, str);
        if xmlMemTraceBlockAt == s as *mut ::core::ffi::c_void {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"%p : Strdup() Ok\n\0" as *const u8 as *const ::core::ffi::c_char,
                xmlMemTraceBlockAt,
            );
            xmlMallocBreakpoint();
        }
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemoryStrdup(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return xmlMemStrdupLoc(
        str,
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemSize(mut ptr: *mut ::core::ffi::c_void) -> size_t {
    let mut p: *mut MEMHDR = ::core::ptr::null_mut::<MEMHDR>();
    if ptr.is_null() {
        return 0 as size_t;
    }
    p = (ptr as *mut ::core::ffi::c_char).offset(-(RESERVE_SIZE as isize))
        as *mut ::core::ffi::c_void as *mut MEMHDR;
    if (*p).mh_tag != MEMTAG {
        return 0 as size_t;
    }
    return (*p).mh_size;
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemUsed() -> ::core::ffi::c_int {
    return debugMemSize as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemBlocks() -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlMemMutex);
    res = debugMemBlocks as ::core::ffi::c_int;
    xmlMutexUnlock(&raw mut xmlMemMutex);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemDisplayLast(mut fp: *mut FILE, mut nbBytes: ::core::ffi::c_long) {
    let mut old_fp: *mut FILE = fp;
    if nbBytes <= 0 as ::core::ffi::c_long {
        return;
    }
    if fp.is_null() {
        fp = fopen(
            b".memorylist\0" as *const u8 as *const ::core::ffi::c_char,
            b"w\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if fp.is_null() {
            return;
        }
    }
    fprintf(
        fp,
        b"Memory list not compiled (MEM_LIST not defined !)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if old_fp.is_null() {
        fclose(fp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemDisplay(mut fp: *mut FILE) {
    let mut old_fp: *mut FILE = fp;
    if fp.is_null() {
        fp = fopen(
            b".memorylist\0" as *const u8 as *const ::core::ffi::c_char,
            b"w\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if fp.is_null() {
            return;
        }
    }
    fprintf(
        fp,
        b"Memory list not compiled (MEM_LIST not defined !)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if old_fp.is_null() {
        fclose(fp);
    }
}
unsafe extern "C" fn debugmem_tag_error(mut p: *mut ::core::ffi::c_void) {
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"Memory tag error occurs :%p \n\t bye\n\0" as *const u8 as *const ::core::ffi::c_char,
        p,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemShow(mut fp: *mut FILE, mut nr: ::core::ffi::c_int) {
    if !fp.is_null() {
        fprintf(
            fp,
            b"      MEMORY ALLOCATED : %lu, MAX was %lu\n\0" as *const u8
                as *const ::core::ffi::c_char,
            debugMemSize,
            debugMaxMemSize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemoryDump() {}
#[no_mangle]
pub unsafe extern "C" fn xmlInitMemory() -> ::core::ffi::c_int {
    xmlInitParser();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlInitMemoryInternal() {
    let mut breakpoint: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    xmlInitMutex(&raw mut xmlMemMutex);
    breakpoint = getenv(b"XML_MEM_BREAKPOINT\0" as *const u8 as *const ::core::ffi::c_char);
    if !breakpoint.is_null() {
        sscanf(
            breakpoint,
            b"%ud\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut xmlMemStopAtBlock,
        );
    }
    breakpoint = getenv(b"XML_MEM_TRACE\0" as *const u8 as *const ::core::ffi::c_char);
    if !breakpoint.is_null() {
        sscanf(
            breakpoint,
            b"%p\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut xmlMemTraceBlockAt,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupMemory() {}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupMemoryInternal() {
    xmlCleanupMutex(&raw mut xmlMemMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemSetup(
    mut freeFunc: xmlFreeFunc,
    mut mallocFunc: xmlMallocFunc,
    mut reallocFunc: xmlReallocFunc,
    mut strdupFunc: xmlStrdupFunc,
) -> ::core::ffi::c_int {
    if freeFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if mallocFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if reallocFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if strdupFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    xmlFree = freeFunc;
    xmlMalloc = mallocFunc;
    xmlMallocAtomic = mallocFunc;
    xmlRealloc = reallocFunc;
    xmlMemStrdup = strdupFunc;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlMemGet(
    mut freeFunc: *mut xmlFreeFunc,
    mut mallocFunc: *mut xmlMallocFunc,
    mut reallocFunc: *mut xmlReallocFunc,
    mut strdupFunc: *mut xmlStrdupFunc,
) -> ::core::ffi::c_int {
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
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGcMemSetup(
    mut freeFunc: xmlFreeFunc,
    mut mallocFunc: xmlMallocFunc,
    mut mallocAtomicFunc: xmlMallocFunc,
    mut reallocFunc: xmlReallocFunc,
    mut strdupFunc: xmlStrdupFunc,
) -> ::core::ffi::c_int {
    if freeFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if mallocFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if mallocAtomicFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if reallocFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if strdupFunc.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    xmlFree = freeFunc;
    xmlMalloc = mallocFunc;
    xmlMallocAtomic = mallocAtomicFunc;
    xmlRealloc = reallocFunc;
    xmlMemStrdup = strdupFunc;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGcMemGet(
    mut freeFunc: *mut xmlFreeFunc,
    mut mallocFunc: *mut xmlMallocFunc,
    mut mallocAtomicFunc: *mut xmlMallocFunc,
    mut reallocFunc: *mut xmlReallocFunc,
    mut strdupFunc: *mut xmlStrdupFunc,
) -> ::core::ffi::c_int {
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
    return 0 as ::core::ffi::c_int;
}
