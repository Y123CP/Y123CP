extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn xmlCleanupInputCallbacks();
    fn xmlRegisterDefaultInputCallbacks();
    fn xmlCleanupOutputCallbacks();
    fn xmlRegisterDefaultOutputCallbacks();
    fn xmlCleanupCharEncodingHandlers();
    fn xmlCatalogCleanup();
    fn xmlSchemaCleanupTypes();
    fn xmlRelaxNGCleanupTypes();
    fn xmlInitDictInternal();
    fn xmlCleanupDictInternal();
    fn xmlInitRandom();
    fn xmlCleanupRandom();
    fn xmlInitEncodingInternal();
    fn xmlInitGlobalsInternal();
    fn xmlCleanupGlobalsInternal();
    fn xmlInitMemoryInternal();
    fn xmlCleanupMemoryInternal();
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    fn xmlInitXPathInternal();
}
pub type size_t = usize;
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
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [::core::ffi::c_uint; 2],
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: ::core::ffi::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: ::core::ffi::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
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
    pub held: ::core::ffi::c_uint,
    pub waiters: ::core::ffi::c_uint,
    pub tid: pthread_t,
    pub cv: pthread_cond_t,
}
pub type xmlRMutex = _xmlRMutex;
pub type xmlRMutexPtr = *mut xmlRMutex;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut libxml_is_threaded: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut xmlLibraryLock: xmlRMutexPtr = ::core::ptr::null::<xmlRMutex>() as *mut xmlRMutex;
#[no_mangle]
pub unsafe extern "C" fn xmlInitMutex(mut mutex: xmlMutexPtr) {
    if (libxml_is_threaded == 0) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        pthread_mutex_init(
            &raw mut (*mutex).lock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewMutex() -> xmlMutexPtr {
    let mut tok: xmlMutexPtr = ::core::ptr::null_mut::<xmlMutex>();
    tok = malloc(::core::mem::size_of::<xmlMutex>() as size_t) as xmlMutexPtr;
    if tok.is_null() {
        return ::core::ptr::null_mut::<xmlMutex>();
    }
    xmlInitMutex(tok);
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupMutex(mut mutex: xmlMutexPtr) {
    if (libxml_is_threaded == 0) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        pthread_mutex_destroy(&raw mut (*mutex).lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeMutex(mut tok: xmlMutexPtr) {
    if tok.is_null() {
        return;
    }
    xmlCleanupMutex(tok);
    free(tok as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlMutexLock(mut tok: xmlMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded != 0 as ::core::ffi::c_int {
        pthread_mutex_lock(&raw mut (*tok).lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlMutexUnlock(mut tok: xmlMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded != 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut (*tok).lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewRMutex() -> xmlRMutexPtr {
    let mut tok: xmlRMutexPtr = ::core::ptr::null_mut::<xmlRMutex>();
    tok = malloc(::core::mem::size_of::<xmlRMutex>() as size_t) as xmlRMutexPtr;
    if tok.is_null() {
        return ::core::ptr::null_mut::<xmlRMutex>();
    }
    if (libxml_is_threaded == 0) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        pthread_mutex_init(
            &raw mut (*tok).lock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
        (*tok).held = 0 as ::core::ffi::c_uint;
        (*tok).waiters = 0 as ::core::ffi::c_uint;
        pthread_cond_init(
            &raw mut (*tok).cv,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
    }
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeRMutex(mut tok: xmlRMutexPtr) {
    if tok.is_null() {
        return;
    }
    if (libxml_is_threaded == 0) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        pthread_mutex_destroy(&raw mut (*tok).lock);
        pthread_cond_destroy(&raw mut (*tok).cv);
    }
    free(tok as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRMutexLock(mut tok: xmlRMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded == 0 as ::core::ffi::c_int {
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
    (*tok).held = 1 as ::core::ffi::c_uint;
    pthread_mutex_unlock(&raw mut (*tok).lock);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRMutexUnlock(mut tok: xmlRMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded == 0 as ::core::ffi::c_int {
        return;
    }
    pthread_mutex_lock(&raw mut (*tok).lock);
    (*tok).held = (*tok).held.wrapping_sub(1);
    if (*tok).held == 0 as ::core::ffi::c_uint {
        if (*tok).waiters != 0 {
            pthread_cond_signal(&raw mut (*tok).cv);
        }
        memset(
            &raw mut (*tok).tid as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<pthread_t>() as size_t,
        );
    }
    pthread_mutex_unlock(&raw mut (*tok).lock);
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetThreadId() -> ::core::ffi::c_int {
    let mut id: pthread_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if libxml_is_threaded == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    id = pthread_self();
    memcpy(
        &raw mut ret as *mut ::core::ffi::c_void,
        &raw mut id as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlLockLibrary() {
    xmlRMutexLock(xmlLibraryLock);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUnlockLibrary() {
    xmlRMutexUnlock(xmlLibraryLock);
}
#[no_mangle]
pub unsafe extern "C" fn xmlInitThreads() {
    xmlInitParser();
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupThreads() {}
static mut xmlParserInitialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlParserInnerInitialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut global_init_lock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __elision: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
            __next: ::core::ptr::null::<__pthread_internal_list>() as *mut __pthread_internal_list,
        },
    },
};
unsafe extern "C" fn xmlGlobalInitMutexLock() {
    if libxml_is_threaded == -(1 as ::core::ffi::c_int) {
        libxml_is_threaded = (Some(
            pthread_mutex_init
                as unsafe extern "C" fn(
                    *mut pthread_mutex_t,
                    *const pthread_mutexattr_t,
                ) -> ::core::ffi::c_int,
        )
        .is_some()
            && Some(
                pthread_mutex_destroy
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(
                pthread_mutex_lock
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(
                pthread_mutex_unlock
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(
                pthread_cond_init
                    as unsafe extern "C" fn(
                        *mut pthread_cond_t,
                        *const pthread_condattr_t,
                    ) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(
                pthread_cond_destroy
                    as unsafe extern "C" fn(*mut pthread_cond_t) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(
                pthread_cond_wait
                    as unsafe extern "C" fn(
                        *mut pthread_cond_t,
                        *mut pthread_mutex_t,
                    ) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(pthread_self as unsafe extern "C" fn() -> pthread_t).is_some()
            && Some(
                pthread_cond_signal
                    as unsafe extern "C" fn(*mut pthread_cond_t) -> ::core::ffi::c_int,
            )
            .is_some()) as ::core::ffi::c_int;
    }
    if libxml_is_threaded != 0 as ::core::ffi::c_int {
        pthread_mutex_lock(&raw mut global_init_lock);
    }
}
unsafe extern "C" fn xmlGlobalInitMutexUnlock() {
    if libxml_is_threaded != 0 as ::core::ffi::c_int {
        pthread_mutex_unlock(&raw mut global_init_lock);
    }
}
unsafe extern "C" fn xmlGlobalInitMutexDestroy() {}
#[no_mangle]
pub unsafe extern "C" fn xmlInitParser() {
    if xmlParserInitialized != 0 as ::core::ffi::c_int {
        return;
    }
    xmlGlobalInitMutexLock();
    if xmlParserInnerInitialized == 0 as ::core::ffi::c_int {
        xmlInitMemoryInternal();
        xmlInitGlobalsInternal();
        xmlInitRandom();
        xmlInitDictInternal();
        xmlInitEncodingInternal();
        xmlInitXPathInternal();
        xmlRegisterDefaultInputCallbacks();
        xmlRegisterDefaultOutputCallbacks();
        xmlParserInnerInitialized = 1 as ::core::ffi::c_int;
    }
    xmlGlobalInitMutexUnlock();
    xmlParserInitialized = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupParser() {
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
    xmlParserInitialized = 0 as ::core::ffi::c_int;
    xmlParserInnerInitialized = 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> ::core::ffi::c_int {
    return (__thread1 == __thread2) as ::core::ffi::c_int;
}
