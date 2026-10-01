use core::ffi::*;
use crate::src::c_inlined_fns::pthread_equal;
use crate::src::catalog::xmlCatalogCleanup;
use crate::src::encoding::xmlCleanupCharEncodingHandlers;
use crate::src::dict::xmlCleanupDictInternal;
use crate::src::globals::xmlCleanupGlobalsInternal;
use crate::src::xmlIO::xmlCleanupInputCallbacks;
use crate::src::xmlmemory::xmlCleanupMemoryInternal;
use crate::src::xmlIO::xmlCleanupOutputCallbacks;
use crate::src::dict::xmlCleanupRandom;
use crate::src::dict::xmlInitDictInternal;
use crate::src::encoding::xmlInitEncodingInternal;
use crate::src::globals::xmlInitGlobalsInternal;
use crate::src::xmlmemory::xmlInitMemoryInternal;
use crate::src::xpath::xmlInitXPathInternal;
use crate::src::xmlIO::xmlRegisterDefaultInputCallbacks;
use crate::src::xmlIO::xmlRegisterDefaultOutputCallbacks;
use crate::src::relaxng::xmlRelaxNGCleanupTypes;
use crate::src::xmlschemastypes::xmlSchemaCleanupTypes;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    fn xmlInitRandom();
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> c_int;
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
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [c_uint; 2],
    pub __g_size: [c_uint; 2],
    pub __g1_orig_size: c_uint,
    pub __wrefs: c_uint,
    pub __g_signals: [c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: c_ulonglong,
    pub __g1_start32: C2RustUnnamed_hsf43fd4d3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_hsf43fd4d3 {
    pub __low: c_uint,
    pub __high: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: c_ulonglong,
    pub __wseq32: C2RustUnnamed_hsf43fd4d3,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [c_char; 4],
    pub __align: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [c_char; 4],
    pub __align: c_int,
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [c_char; 48],
    pub __align: c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlMutex {
    pub lock: pthread_mutex_t,
}
pub type xmlMutex = _xmlMutex;
pub type xmlMutexPtr = *mut xmlMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRMutex {
    pub lock: pthread_mutex_t,
    pub held: c_uint,
    pub waiters: c_uint,
    pub tid: pthread_t,
    pub cv: pthread_cond_t,
}
pub type xmlRMutex = _xmlRMutex;
pub type xmlRMutexPtr = *mut xmlRMutex;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_htdd24ee73 = 0;

pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_htdd24ee73 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_htdd24ee73 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_htdd24ee73 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_htdd24ee73 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_htdd24ee73 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_htdd24ee73 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_htdd24ee73 = 1;

static mut libxml_is_threaded: c_int = -(1 as c_int);
static mut xmlLibraryLock: xmlRMutexPtr = ::core::ptr::null::<xmlRMutex>() as *mut xmlRMutex;
#[no_mangle]
pub extern "C" fn xmlInitMutex(mut mutex: xmlMutexPtr) { unsafe {
    if (libxml_is_threaded == 0) as c_int == 0 as c_int {
        pthread_mutex_init(
            &raw mut (*mutex).lock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
    }
} }
#[inline]
pub fn xmlNewMutex() -> xmlMutexPtr { unsafe {
    let mut tok: xmlMutexPtr = ::core::ptr::null_mut::<xmlMutex>();
    tok = malloc(::core::mem::size_of::<xmlMutex>() as size_t) as xmlMutexPtr;
    if tok.is_null() {
        return ::core::ptr::null_mut::<xmlMutex>();
    }
    xmlInitMutex(tok);
    return tok;
} }
#[no_mangle]
pub extern "C" fn xmlCleanupMutex(mut mutex: xmlMutexPtr) { unsafe {
    if (libxml_is_threaded == 0) as c_int == 0 as c_int {
        pthread_mutex_destroy(&raw mut (*mutex).lock);
    }
} }
#[inline]
pub fn xmlFreeMutex(mut tok: xmlMutexPtr) { unsafe {
    if tok.is_null() {
        return;
    }
    xmlCleanupMutex(tok);
    free(tok as *mut c_void);
} }
#[no_mangle]
pub extern "C" fn xmlMutexLock(mut tok: xmlMutexPtr) { unsafe {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded != 0 as c_int {
        pthread_mutex_lock(&raw mut (*tok).lock);
    }
} }
#[no_mangle]
pub extern "C" fn xmlMutexUnlock(mut tok: xmlMutexPtr) { unsafe {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded != 0 as c_int {
        pthread_mutex_unlock(&raw mut (*tok).lock);
    }
} }
#[inline]
pub fn xmlNewRMutex() -> xmlRMutexPtr { unsafe {
    let mut tok: xmlRMutexPtr = ::core::ptr::null_mut::<xmlRMutex>();
    tok = malloc(::core::mem::size_of::<xmlRMutex>() as size_t) as xmlRMutexPtr;
    if tok.is_null() {
        return ::core::ptr::null_mut::<xmlRMutex>();
    }
    if (libxml_is_threaded == 0) as c_int == 0 as c_int {
        pthread_mutex_init(
            &raw mut (*tok).lock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
        (*tok).held = 0 as c_uint;
        (*tok).waiters = 0 as c_uint;
        pthread_cond_init(
            &raw mut (*tok).cv,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
    }
    return tok;
} }
#[inline]
pub fn xmlFreeRMutex(mut tok: xmlRMutexPtr) { unsafe {
    if tok.is_null() {
        return;
    }
    if (libxml_is_threaded == 0) as c_int == 0 as c_int {
        pthread_mutex_destroy(&raw mut (*tok).lock);
        pthread_cond_destroy(&raw mut (*tok).cv);
    }
    free(tok as *mut c_void);
} }
#[inline]
pub fn xmlRMutexLock(mut tok: xmlRMutexPtr) { unsafe {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded == 0 as c_int {
        return;
    }
    pthread_mutex_lock(&raw mut (*tok).lock);
    if (*tok).held != 0 {
        if pthread_equal((*tok).tid, pthread_self()) != 0 {
            (*tok).held = (*tok).held.wrapping_add(1);
            pthread_mutex_unlock(&raw mut (*tok).lock);
            return;
        } else {
            (*tok).waiters = (*tok).waiters.wrapping_add(1);
            while (*tok).held != 0 {
                pthread_cond_wait(&raw mut (*tok).cv, &raw mut (*tok).lock);
            }
            (*tok).waiters = (*tok).waiters.wrapping_sub(1);
        }
    }
    (*tok).tid = pthread_self();
    (*tok).held = 1 as c_uint;
    pthread_mutex_unlock(&raw mut (*tok).lock);
} }
#[inline]
pub fn xmlRMutexUnlock(mut tok: xmlRMutexPtr) { unsafe {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded == 0 as c_int {
        return;
    }
    pthread_mutex_lock(&raw mut (*tok).lock);
    (*tok).held = (*tok).held.wrapping_sub(1);
    if (*tok).held == 0 as c_uint {
        if (*tok).waiters != 0 {
            pthread_cond_signal(&raw mut (*tok).cv);
        }
        memset(
            &raw mut (*tok).tid as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<pthread_t>() as size_t,
        );
    }
    pthread_mutex_unlock(&raw mut (*tok).lock);
} }
#[inline]
pub fn xmlGetThreadId() -> c_int { unsafe {
    let mut id: pthread_t = 0;
    let mut ret: c_int = 0;
    if libxml_is_threaded == 0 as c_int {
        return 0 as c_int;
    }
    id = pthread_self();
    memcpy(
        &raw mut ret as *mut c_void,
        &raw mut id as *const c_void,
        ::core::mem::size_of::<c_int>() as size_t,
    );
    return ret;
} }
#[inline]
pub fn xmlLockLibrary() { unsafe {
    xmlRMutexLock(xmlLibraryLock);
} }
#[inline]
pub fn xmlUnlockLibrary() { unsafe {
    xmlRMutexUnlock(xmlLibraryLock);
} }
#[inline]
pub fn xmlInitThreads() { {
    xmlInitParser();
} }
#[inline]
pub fn xmlCleanupThreads() { {} }
static mut xmlParserInitialized: c_int = 0 as c_int;
static mut xmlParserInnerInitialized: c_int = 0 as c_int;
static mut global_init_lock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as c_int,
        __count: 0 as c_uint,
        __owner: 0 as c_int,
        __nusers: 0 as c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as c_int,
        __spins: 0 as c_short,
        __elision: 0 as c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
            __next: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
        },
    },
};
fn xmlGlobalInitMutexLock() { unsafe {
    if libxml_is_threaded == -(1 as c_int) {
        libxml_is_threaded = (Some(
            pthread_mutex_init
                as unsafe extern "C" fn(
                    *mut pthread_mutex_t,
                    *const pthread_mutexattr_t,
                ) -> c_int,
        )
        .is_some()
            && Some(
                pthread_mutex_destroy
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> c_int,
            )
            .is_some()
            && Some(
                pthread_mutex_lock
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> c_int,
            )
            .is_some()
            && Some(
                pthread_mutex_unlock
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> c_int,
            )
            .is_some()
            && Some(
                pthread_cond_init
                    as unsafe extern "C" fn(
                        *mut pthread_cond_t,
                        *const pthread_condattr_t,
                    ) -> c_int,
            )
            .is_some()
            && Some(
                pthread_cond_destroy
                    as unsafe extern "C" fn(*mut pthread_cond_t) -> c_int,
            )
            .is_some()
            && Some(
                pthread_cond_wait
                    as unsafe extern "C" fn(
                        *mut pthread_cond_t,
                        *mut pthread_mutex_t,
                    ) -> c_int,
            )
            .is_some()
            && Some(pthread_self as unsafe extern "C" fn() -> pthread_t).is_some()
            && Some(
                pthread_cond_signal
                    as unsafe extern "C" fn(*mut pthread_cond_t) -> c_int,
            )
            .is_some()) as c_int;
    }
    if libxml_is_threaded != 0 as c_int {
        pthread_mutex_lock(&raw mut global_init_lock);
    }
} }
fn xmlGlobalInitMutexUnlock() { unsafe {
    if libxml_is_threaded != 0 as c_int {
        pthread_mutex_unlock(&raw mut global_init_lock);
    }
} }
fn xmlGlobalInitMutexDestroy() { {} }
#[inline]
pub fn xmlInitParser() { unsafe {
    if xmlParserInitialized != 0 as c_int {
        return;
    }
    xmlGlobalInitMutexLock();
    if xmlParserInnerInitialized == 0 as c_int {
        xmlInitMemoryInternal();
        xmlInitGlobalsInternal();
        xmlInitRandom();
        xmlInitDictInternal();
        xmlInitEncodingInternal();
        xmlInitXPathInternal();
        xmlRegisterDefaultInputCallbacks();
        xmlRegisterDefaultOutputCallbacks();
        xmlParserInnerInitialized = 1 as c_int;
    }
    xmlGlobalInitMutexUnlock();
    xmlParserInitialized = 1 as c_int;
} }
#[inline]
pub fn xmlCleanupParser() { unsafe {
    if xmlParserInitialized == 0 {
        return;
    }
    xmlCleanupCharEncodingHandlers();
    xmlCatalogCleanup();
    xmlSchemaCleanupTypes();
    xmlRelaxNGCleanupTypes();
    xmlCleanupInputCallbacks();
    xmlCleanupOutputCallbacks();
    xmlCleanupDictInternal();
    xmlCleanupRandom();
    xmlCleanupGlobalsInternal();
    xmlCleanupMemoryInternal();
    xmlGlobalInitMutexDestroy();
    xmlParserInitialized = 0 as c_int;
    xmlParserInnerInitialized = 0 as c_int;
} }

