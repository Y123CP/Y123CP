extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn perror(__s: *const ::core::ffi::c_char);
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn has_match(
        needle: *const ::core::ffi::c_char,
        haystack: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "match"]
    fn match_0(needle: *const ::core::ffi::c_char, haystack: *const ::core::ffi::c_char)
        -> score_t;
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_t {
    pub benchmark: ::core::ffi::c_int,
    pub filter: *const ::core::ffi::c_char,
    pub init_search: *const ::core::ffi::c_char,
    pub tty_filename: *const ::core::ffi::c_char,
    pub show_scores: ::core::ffi::c_int,
    pub num_lines: ::core::ffi::c_uint,
    pub scrolloff: ::core::ffi::c_uint,
    pub prompt: *const ::core::ffi::c_char,
    pub workers: ::core::ffi::c_uint,
    pub input_delimiter: ::core::ffi::c_char,
    pub show_info: ::core::ffi::c_int,
}
pub type score_t = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scored_result {
    pub score: score_t,
    pub str_0: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct choices_t {
    pub buffer: *mut ::core::ffi::c_char,
    pub buffer_size: size_t,
    pub capacity: size_t,
    pub size: size_t,
    pub strings: *mut *const ::core::ffi::c_char,
    pub results: *mut scored_result,
    pub available: size_t,
    pub selection: size_t,
    pub worker_count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_job {
    pub lock: pthread_mutex_t,
    pub choices: *mut choices_t,
    pub search: *const ::core::ffi::c_char,
    pub processed: size_t,
    pub worker_count: ::core::ffi::c_uint,
    pub workers: *mut worker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker {
    pub thread_id: pthread_t,
    pub job: *mut search_job,
    pub worker_num: ::core::ffi::c_uint,
    pub result: result_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct result_list {
    pub list: *mut scored_result,
    pub size: size_t,
    pub capacity: size_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INITIAL_BUFFER_CAPACITY: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const INITIAL_CHOICE_CAPACITY: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
unsafe extern "C" fn cmpchoice(
    mut _idx1: *const ::core::ffi::c_void,
    mut _idx2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const scored_result = _idx1 as *const scored_result;
    let mut b: *const scored_result = _idx2 as *const scored_result;
    if (*a).score == (*b).score {
        if (*a).str_0 < (*b).str_0 {
            return -(1 as ::core::ffi::c_int);
        } else {
            return 1 as ::core::ffi::c_int;
        }
    } else if (*a).score < (*b).score {
        return 1 as ::core::ffi::c_int;
    } else {
        return -(1 as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn safe_realloc(
    mut buffer: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    buffer = realloc(buffer, size);
    if buffer.is_null() {
        fprintf(
            stderr,
            b"Error: Can't allocate memory (%zu bytes)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            size,
        );
        abort();
    }
    return buffer;
}
#[inline]
unsafe extern "C" fn result_precedes(
    mut a: *const scored_result,
    mut b: *const scored_result,
) -> bool {
    return (*a).score > (*b).score || (*a).score == (*b).score && (*a).str_0 < (*b).str_0;
}
unsafe extern "C" fn result_list_reserve(mut result: *mut result_list, mut capacity: size_t) {
    if capacity <= (*result).capacity {
        return;
    }
    (*result).list = safe_realloc(
        (*result).list as *mut ::core::ffi::c_void,
        capacity.wrapping_mul(::core::mem::size_of::<scored_result>() as size_t),
    ) as *mut scored_result;
    (*result).capacity = capacity;
}
unsafe extern "C" fn result_list_push(
    mut result: *mut result_list,
    mut score: score_t,
    mut str_0: *const ::core::ffi::c_char,
) {
    if (*result).size == (*result).capacity {
        let mut next_capacity: size_t = if (*result).capacity != 0 {
            (*result).capacity.wrapping_mul(2 as size_t)
        } else {
            BATCH_SIZE as size_t
        };
        if next_capacity <= (*result).size {
            next_capacity = (*result).size.wrapping_add(1 as size_t);
        }
        result_list_reserve(result, next_capacity);
    }
    let mut entry: *mut scored_result = (*result).list.offset((*result).size as isize);
    (*entry).score = score;
    (*entry).str_0 = str_0;
    (*result).size = (*result).size.wrapping_add(1 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn choices_fread(
    mut c: *mut choices_t,
    mut file: *mut FILE,
    mut input_delimiter: ::core::ffi::c_char,
) {
    let mut buffer_start: size_t = (*c).buffer_size;
    let mut capacity: size_t = INITIAL_BUFFER_CAPACITY as size_t;
    while capacity <= (*c).buffer_size {
        capacity = capacity.wrapping_mul(2 as size_t);
    }
    (*c).buffer =
        safe_realloc((*c).buffer as *mut ::core::ffi::c_void, capacity) as *mut ::core::ffi::c_char;
    loop {
        (*c).buffer_size = ((*c).buffer_size as ::core::ffi::c_ulong).wrapping_add(fread(
            (*c).buffer.offset((*c).buffer_size as isize) as *mut ::core::ffi::c_void,
            1 as size_t,
            capacity.wrapping_sub((*c).buffer_size),
            file,
        )) as size_t as size_t;
        if !((*c).buffer_size == capacity) {
            break;
        }
        capacity = capacity.wrapping_mul(2 as size_t);
        (*c).buffer = safe_realloc((*c).buffer as *mut ::core::ffi::c_void, capacity)
            as *mut ::core::ffi::c_char;
    }
    (*c).buffer = safe_realloc(
        (*c).buffer as *mut ::core::ffi::c_void,
        (*c).buffer_size.wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    let fresh0 = (*c).buffer_size;
    (*c).buffer_size = (*c).buffer_size.wrapping_add(1);
    *(*c).buffer.offset(fresh0 as isize) = '\0' as i32 as ::core::ffi::c_char;
    let mut line_end: *const ::core::ffi::c_char = (*c).buffer.offset((*c).buffer_size as isize);
    let mut line: *mut ::core::ffi::c_char = (*c).buffer.offset(buffer_start as isize);
    loop {
        let mut nl: *mut ::core::ffi::c_char = strchr(line, input_delimiter as ::core::ffi::c_int);
        if !nl.is_null() {
            let fresh1 = nl;
            nl = nl.offset(1);
            *fresh1 = '\0' as i32 as ::core::ffi::c_char;
        }
        if *line != 0 {
            choices_add(c, line);
        }
        line = nl;
        if !(!line.is_null() && line < line_end as *mut ::core::ffi::c_char) {
            break;
        }
    }
}
unsafe extern "C" fn choices_resize(mut c: *mut choices_t, mut new_capacity: size_t) {
    (*c).strings = safe_realloc(
        (*c).strings as *mut ::core::ffi::c_void,
        new_capacity.wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    ) as *mut *const ::core::ffi::c_char;
    (*c).capacity = new_capacity;
}
unsafe extern "C" fn choices_reset_search(mut c: *mut choices_t) {
    free((*c).results as *mut ::core::ffi::c_void);
    (*c).available = 0 as size_t;
    (*c).selection = (*c).available;
    (*c).results = ::core::ptr::null_mut::<scored_result>();
}
#[no_mangle]
pub unsafe extern "C" fn choices_init(mut c: *mut choices_t, mut options: *mut options_t) {
    (*c).strings = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    (*c).results = ::core::ptr::null_mut::<scored_result>();
    (*c).buffer_size = 0 as size_t;
    (*c).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*c).size = 0 as size_t;
    (*c).capacity = (*c).size;
    choices_resize(c, INITIAL_CHOICE_CAPACITY as size_t);
    if (*options).workers != 0 {
        (*c).worker_count = (*options).workers;
    } else {
        (*c).worker_count = sysconf(_SC_NPROCESSORS_ONLN as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    choices_reset_search(c);
}
#[no_mangle]
pub unsafe extern "C" fn choices_destroy(mut c: *mut choices_t) {
    free((*c).buffer as *mut ::core::ffi::c_void);
    (*c).buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*c).buffer_size = 0 as size_t;
    free((*c).strings as *mut ::core::ffi::c_void);
    (*c).strings = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    (*c).size = 0 as size_t;
    (*c).capacity = (*c).size;
    free((*c).results as *mut ::core::ffi::c_void);
    (*c).results = ::core::ptr::null_mut::<scored_result>();
    (*c).selection = 0 as size_t;
    (*c).available = (*c).selection;
}
#[no_mangle]
pub unsafe extern "C" fn choices_add(
    mut c: *mut choices_t,
    mut choice: *const ::core::ffi::c_char,
) {
    choices_reset_search(c);
    if (*c).size == (*c).capacity {
        choices_resize(c, (*c).capacity.wrapping_mul(2 as size_t));
    }
    let fresh2 = (*c).size;
    (*c).size = (*c).size.wrapping_add(1);
    let ref mut fresh3 = *(*c).strings.offset(fresh2 as isize);
    *fresh3 = choice;
}
#[no_mangle]
pub unsafe extern "C" fn choices_available(mut c: *mut choices_t) -> size_t {
    return (*c).available;
}
pub const BATCH_SIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
unsafe extern "C" fn worker_get_next_batch(
    mut job: *mut search_job,
    mut start: *mut size_t,
    mut end: *mut size_t,
) {
    pthread_mutex_lock(&raw mut (*job).lock);
    *start = (*job).processed;
    (*job).processed = (*job).processed.wrapping_add(BATCH_SIZE as size_t);
    if (*job).processed > (*(*job).choices).size {
        (*job).processed = (*(*job).choices).size;
    }
    *end = (*job).processed;
    pthread_mutex_unlock(&raw mut (*job).lock);
}
unsafe extern "C" fn merge2(mut list1: result_list, mut list2: result_list) -> result_list {
    let mut result: result_list = list1;
    let mut index1: size_t = result.size;
    let mut index2: size_t = list2.size;
    result.size = result.size.wrapping_add(list2.size);
    result_list_reserve(&raw mut result, result.size);
    let mut result_index: size_t = result.size;
    while index1 != 0 && index2 != 0 {
        let mut a: *const scored_result = result.list.offset(index1.wrapping_sub(1 as size_t) as isize);
        let mut b: *const scored_result = list2.list.offset(index2.wrapping_sub(1 as size_t) as isize);
        result_index = result_index.wrapping_sub(1 as size_t);
        if result_precedes(a, b) {
            index2 = index2.wrapping_sub(1 as size_t);
            *result.list.offset(result_index as isize) = *b;
        } else {
            index1 = index1.wrapping_sub(1 as size_t);
            *result.list.offset(result_index as isize) = *a;
        }
    }
    while index2 != 0 {
        index2 = index2.wrapping_sub(1 as size_t);
        result_index = result_index.wrapping_sub(1 as size_t);
        *result.list.offset(result_index as isize) = *list2.list.offset(index2 as isize);
    }
    free(list2.list as *mut ::core::ffi::c_void);
    return result;
}
unsafe extern "C" fn choices_search_worker(
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut w: *mut worker = data as *mut worker;
    let mut job: *mut search_job = (*w).job;
    let mut c: *const choices_t = (*job).choices;
    let mut result: *mut result_list = &raw mut (*w).result;
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    loop {
        worker_get_next_batch(job, &raw mut start, &raw mut end);
        if start == end {
            break;
        }
        let mut i: size_t = start;
        while i < end {
            if has_match((*job).search, *(*c).strings.offset(i as isize)) != 0 {
                result_list_push(
                    result,
                    match_0((*job).search, *(*c).strings.offset(i as isize)),
                    *(*c).strings.offset(i as isize),
                );
            }
            i = i.wrapping_add(1);
        }
    }
    qsort(
        (*result).list as *mut ::core::ffi::c_void,
        (*result).size,
        ::core::mem::size_of::<scored_result>() as size_t,
        Some(
            cmpchoice
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let mut step: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while !((*w)
        .worker_num
        .wrapping_rem(((2 as ::core::ffi::c_int) << step) as ::core::ffi::c_uint)
        != 0)
    {
        let mut next_worker: ::core::ffi::c_uint =
            (*w).worker_num | ((1 as ::core::ffi::c_int) << step) as ::core::ffi::c_uint;
        if next_worker >= (*job).worker_count {
            break;
        }
        let ref mut fresh8 = *__errno_location();
        *fresh8 = pthread_join(
            (*(*job).workers.offset(next_worker as isize)).thread_id,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        if *fresh8 != 0 {
            perror(b"pthread_join\0" as *const u8 as *const ::core::ffi::c_char);
            exit(EXIT_FAILURE);
        }
        (*w).result = merge2(
            (*w).result,
            (*(*job).workers.offset(next_worker as isize)).result,
        );
        step = step.wrapping_add(1);
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn choices_search(
    mut c: *mut choices_t,
    mut search: *const ::core::ffi::c_char,
) {
    choices_reset_search(c);
    if (*c).size == 0 {
        return;
    }
    if *search == 0 {
        (*c).results = malloc(
            (*c).size.wrapping_mul(::core::mem::size_of::<scored_result>() as size_t),
        ) as *mut scored_result;
        if (*c).size != 0 && (*c).results.is_null() {
            fprintf(
                stderr,
                b"Error: Can't allocate memory\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            abort();
        }
        let mut i: size_t = 0 as size_t;
        while i < (*c).size {
            (*(*c).results.offset(i as isize)).score = -::core::f32::INFINITY as score_t;
            (*(*c).results.offset(i as isize)).str_0 = *(*c).strings.offset(i as isize);
            i = i.wrapping_add(1 as size_t);
        }
        (*c).available = (*c).size;
        return;
    }
    let mut worker_count: ::core::ffi::c_uint = (*c).worker_count;
    if worker_count == 0 {
        worker_count = 1 as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    if (*c).size != 0 && worker_count as size_t > (*c).size {
        worker_count = (*c).size as ::core::ffi::c_uint;
    }
    let mut job: *mut search_job =
        calloc(1 as size_t, ::core::mem::size_of::<search_job>() as size_t) as *mut search_job;
    if job.is_null() {
        fprintf(
            stderr,
            b"Error: Can't allocate memory\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        abort();
    }
    (*job).search = search;
    (*job).choices = c;
    (*job).worker_count = worker_count;
    if pthread_mutex_init(
        &raw mut (*job).lock,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    ) != 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"Error: pthread_mutex_init failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        abort();
    }
    (*job).workers = calloc(
        worker_count as size_t,
        ::core::mem::size_of::<worker>() as size_t,
    ) as *mut worker;
    if (*job).workers.is_null() {
        fprintf(
            stderr,
            b"Error: Can't allocate memory\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        abort();
    }
    let mut workers: *mut worker = (*job).workers as *mut worker;
    let mut expected_matches_per_worker: size_t = ((*c)
        .size
        .wrapping_add(worker_count as size_t)
        .wrapping_sub(1 as size_t))
        .wrapping_div(worker_count as size_t)
        .wrapping_add(BATCH_SIZE as size_t);
    if expected_matches_per_worker > (*c).size {
        expected_matches_per_worker = (*c).size;
    }
    let mut i: ::core::ffi::c_int =
        worker_count.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let ref mut fresh4 = (*workers.offset(i as isize)).job;
        *fresh4 = job;
        (*workers.offset(i as isize)).worker_num = i as ::core::ffi::c_uint;
        (*workers.offset(i as isize)).result.size = 0 as size_t;
        (*workers.offset(i as isize)).result.capacity = 0 as size_t;
        (*workers.offset(i as isize)).result.list = ::core::ptr::null_mut::<scored_result>();
        if expected_matches_per_worker != 0 {
            result_list_reserve(
                &raw mut (*workers.offset(i as isize)).result,
                expected_matches_per_worker,
            );
        }
        let ref mut fresh6 = *__errno_location();
        *fresh6 = pthread_create(
            &raw mut (*workers.offset(i as isize)).thread_id,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                choices_search_worker
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            workers.offset(i as isize) as *mut worker as *mut ::core::ffi::c_void,
        );
        if *fresh6 != 0 {
            perror(b"pthread_create\0" as *const u8 as *const ::core::ffi::c_char);
            exit(EXIT_FAILURE);
        }
        i -= 1;
    }
    if pthread_join(
        (*workers.offset(0 as ::core::ffi::c_int as isize)).thread_id,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    ) != 0
    {
        perror(b"pthread_join\0" as *const u8 as *const ::core::ffi::c_char);
        exit(EXIT_FAILURE);
    }
    (*c).results = (*workers.offset(0 as ::core::ffi::c_int as isize))
        .result
        .list;
    (*c).available = (*workers.offset(0 as ::core::ffi::c_int as isize))
        .result
        .size;
    free(workers as *mut ::core::ffi::c_void);
    pthread_mutex_destroy(&raw mut (*job).lock);
    free(job as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn choices_get(
    mut c: *mut choices_t,
    mut n: size_t,
) -> *const ::core::ffi::c_char {
    if n < (*c).available {
        return (*(*c).results.offset(n as isize)).str_0;
    } else {
        return ::core::ptr::null::<::core::ffi::c_char>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn choices_getscore(mut c: *mut choices_t, mut n: size_t) -> score_t {
    return (*(*c).results.offset(n as isize)).score;
}
#[no_mangle]
pub unsafe extern "C" fn choices_prev(mut c: *mut choices_t) {
    if (*c).available != 0 {
        (*c).selection = (*c)
            .selection
            .wrapping_add((*c).available)
            .wrapping_sub(1 as size_t)
            .wrapping_rem((*c).available);
    }
}
#[no_mangle]
pub unsafe extern "C" fn choices_next(mut c: *mut choices_t) {
    if (*c).available != 0 {
        (*c).selection = (*c)
            .selection
            .wrapping_add(1 as size_t)
            .wrapping_rem((*c).available);
    }
}
