extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlAttrHashBucket;
    pub type _xmlParserNsData;
    pub type _xmlHashTable;
    pub type _xmlStartTag;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
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
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn __xstat(
        __ver: ::core::ffi::c_int,
        __filename: *const ::core::ffi::c_char,
        __stat_buf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn xmlGetCharEncodingHandler(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr;
    fn xmlFindCharEncodingHandler(name: *const ::core::ffi::c_char) -> xmlCharEncodingHandlerPtr;
    fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler) -> ::core::ffi::c_int;
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    fn xmlBufEnd(buf: xmlBufPtr) -> *mut xmlChar;
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    fn __xmlDefaultBufferSize() -> *mut ::core::ffi::c_int;
    fn xmlBufferAdd(
        buf: xmlBufferPtr,
        str: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
    static mut xmlMemStrdup: xmlStrdupFunc;
    fn __xmlOutputBufferCreateFilenameValue() -> *mut xmlOutputBufferCreateFilenameFunc;
    fn __xmlParserInputBufferCreateFilenameValue() -> *mut xmlParserInputBufferCreateFilenameFunc;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrndup(cur: *const xmlChar, len: ::core::ffi::c_int) -> *mut xmlChar;
    fn xmlStrstr(str: *const xmlChar, val: *const xmlChar) -> *const xmlChar;
    fn xmlStrncasecmp(
        str1: *const xmlChar,
        str2: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrPrintf(
        buf: *mut xmlChar,
        len: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn xmlSwitchInputEncoding(
        ctxt: xmlParserCtxtPtr,
        input: xmlParserInputPtr,
        handler: xmlCharEncodingHandlerPtr,
    ) -> ::core::ffi::c_int;
    fn xmlFreeInputStream(input: xmlParserInputPtr);
    fn xmlNewInputFromFile(
        ctxt: xmlParserCtxtPtr,
        filename: *const ::core::ffi::c_char,
    ) -> xmlParserInputPtr;
    fn xmlParseURI(str: *const ::core::ffi::c_char) -> xmlURIPtr;
    fn xmlURIUnescapeString(
        str: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        target: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn xmlFreeURI(uri: xmlURIPtr);
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    fn xmlNanoHTTPMethod(
        URL: *const ::core::ffi::c_char,
        method: *const ::core::ffi::c_char,
        input: *const ::core::ffi::c_char,
        contentType: *mut *mut ::core::ffi::c_char,
        headers: *const ::core::ffi::c_char,
        ilen: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn xmlNanoHTTPOpen(
        URL: *const ::core::ffi::c_char,
        contentType: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn xmlNanoHTTPReturnCode(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlNanoHTTPRedir(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn xmlNanoHTTPEncoding(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn xmlNanoHTTPMimeType(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn xmlNanoHTTPRead(
        ctx: *mut ::core::ffi::c_void,
        dest: *mut ::core::ffi::c_void,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlNanoHTTPClose(ctx: *mut ::core::ffi::c_void);
    fn xmlCatalogResolve(pubID: *const xmlChar, sysID: *const xmlChar) -> *mut xmlChar;
    fn xmlCatalogResolveURI(URI: *const xmlChar) -> *mut xmlChar;
    fn xmlCatalogLocalResolve(
        catalogs: *mut ::core::ffi::c_void,
        pubID: *const xmlChar,
        sysID: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlCatalogLocalResolveURI(
        catalogs: *mut ::core::ffi::c_void,
        URI: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlCatalogGetDefaults() -> xmlCatalogAllow;
    fn xmlBufCreate() -> xmlBufPtr;
    fn xmlBufCreateSize(size: size_t) -> xmlBufPtr;
    fn xmlBufSetAllocationScheme(
        buf: xmlBufPtr,
        scheme: xmlBufferAllocationScheme,
    ) -> ::core::ffi::c_int;
    fn xmlBufFree(buf: xmlBufPtr);
    fn xmlBufGrow(buf: xmlBufPtr, len: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xmlBufAdd(
        buf: xmlBufPtr,
        str: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlBufAvail(buf: xmlBufPtr) -> size_t;
    fn xmlBufAddLen(buf: xmlBufPtr, len: size_t) -> ::core::ffi::c_int;
    fn xmlCharEncInput(input: xmlParserInputBufferPtr) -> ::core::ffi::c_int;
    fn xmlCharEncOutput(output: xmlOutputBufferPtr, init: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn __xmlRaiseError(
        schannel: xmlStructuredErrorFunc,
        channel: xmlGenericErrorFunc,
        data: *mut ::core::ffi::c_void,
        ctx: *mut ::core::ffi::c_void,
        nod: *mut ::core::ffi::c_void,
        domain: ::core::ffi::c_int,
        code: ::core::ffi::c_int,
        level: xmlErrorLevel,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
        str3: *const ::core::ffi::c_char,
        int1: ::core::ffi::c_int,
        col: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn __xmlSimpleError(
        domain: ::core::ffi::c_int,
        code: ::core::ffi::c_int,
        node: *mut _xmlNode,
        msg: *const ::core::ffi::c_char,
        extra: *const ::core::ffi::c_char,
    );
    fn xmlErrMemory(ctxt: xmlParserCtxtPtr, extra: *const ::core::ffi::c_char);
    fn __xmlErrEncoding(
        ctxt: xmlParserCtxtPtr,
        xmlerr: xmlParserErrors,
        msg: *const ::core::ffi::c_char,
        str1: *const xmlChar,
        str2: *const xmlChar,
    );
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub type xmlCharEncoding = ::core::ffi::c_int;
pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
pub type xmlCharEncodingInputFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlCharEncodingOutputFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut ::core::ffi::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
}
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_0: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
pub type xmlChar = ::core::ffi::c_uchar;
pub type xmlBufferAllocationScheme = ::core::ffi::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlStrdupFunc =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut ::core::ffi::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub rawconsumed: ::core::ffi::c_ulong,
}
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
pub type xmlInputCloseCallback =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type xmlInputReadCallback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_char,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputBuffer {
    pub context: *mut ::core::ffi::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
}
pub type xmlOutputCloseCallback =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type xmlOutputWriteCallback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const ::core::ffi::c_char,
    pub directory: *const ::core::ffi::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: ::core::ffi::c_int,
    pub line: ::core::ffi::c_int,
    pub col: ::core::ffi::c_int,
    pub consumed: ::core::ffi::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub flags: ::core::ffi::c_int,
    pub id: ::core::ffi::c_int,
    pub parentConsumed: ::core::ffi::c_ulong,
    pub entity: xmlEntityPtr,
}
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEntity {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub orig: *mut xmlChar,
    pub content: *mut xmlChar,
    pub length: ::core::ffi::c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub expandedSize: ::core::ffi::c_ulong,
}
pub type xmlEntityType = ::core::ffi::c_uint;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *mut ::core::ffi::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: ::core::ffi::c_int,
    pub standalone: ::core::ffi::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut ::core::ffi::c_void,
    pub refs: *mut ::core::ffi::c_void,
    pub URL: *const xmlChar,
    pub charset: ::core::ffi::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut ::core::ffi::c_void,
    pub parseFlags: ::core::ffi::c_int,
    pub properties: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut ::core::ffi::c_void,
    pub context: *mut _xmlDoc,
}
pub type xmlElementType = xmlNsType;
pub type xmlNsType = ::core::ffi::c_uint;
pub const XML_XINCLUDE_END: xmlNsType = 20;
pub const XML_XINCLUDE_START: xmlNsType = 19;
pub const XML_NAMESPACE_DECL: xmlNsType = 18;
pub const XML_ENTITY_DECL: xmlNsType = 17;
pub const XML_ATTRIBUTE_DECL: xmlNsType = 16;
pub const XML_ELEMENT_DECL: xmlNsType = 15;
pub const XML_DTD_NODE: xmlNsType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlNsType = 13;
pub const XML_NOTATION_NODE: xmlNsType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlNsType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlNsType = 10;
pub const XML_DOCUMENT_NODE: xmlNsType = 9;
pub const XML_COMMENT_NODE: xmlNsType = 8;
pub const XML_PI_NODE: xmlNsType = 7;
pub const XML_ENTITY_NODE: xmlNsType = 6;
pub const XML_ENTITY_REF_NODE: xmlNsType = 5;
pub const XML_CDATA_SECTION_NODE: xmlNsType = 4;
pub const XML_TEXT_NODE: xmlNsType = 3;
pub const XML_ATTRIBUTE_NODE: xmlNsType = 2;
pub const XML_ELEMENT_NODE: xmlNsType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut ::core::ffi::c_void,
    pub elements: *mut ::core::ffi::c_void,
    pub attributes: *mut ::core::ffi::c_void,
    pub entities: *mut ::core::ffi::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut ::core::ffi::c_void,
    pub line: ::core::ffi::c_ushort,
    pub extra: ::core::ffi::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut ::core::ffi::c_void,
}
pub type xmlAttributeType = ::core::ffi::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlParserInputDeallocate = Option<unsafe extern "C" fn(*mut xmlChar) -> ()>;
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut ::core::ffi::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: ::core::ffi::c_int,
    pub replaceEntities: ::core::ffi::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: ::core::ffi::c_int,
    pub html: ::core::ffi::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: ::core::ffi::c_int,
    pub inputMax: ::core::ffi::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: ::core::ffi::c_int,
    pub nodeMax: ::core::ffi::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: ::core::ffi::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: ::core::ffi::c_int,
    pub hasExternalSubset: ::core::ffi::c_int,
    pub hasPErefs: ::core::ffi::c_int,
    pub external: ::core::ffi::c_int,
    pub valid: ::core::ffi::c_int,
    pub validate: ::core::ffi::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: ::core::ffi::c_int,
    pub directory: *mut ::core::ffi::c_char,
    pub name: *const xmlChar,
    pub nameNr: ::core::ffi::c_int,
    pub nameMax: ::core::ffi::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: ::core::ffi::c_long,
    pub checkIndex: ::core::ffi::c_long,
    pub keepBlanks: ::core::ffi::c_int,
    pub disableSAX: ::core::ffi::c_int,
    pub inSubset: ::core::ffi::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut ::core::ffi::c_int,
    pub spaceNr: ::core::ffi::c_int,
    pub spaceMax: ::core::ffi::c_int,
    pub spaceTab: *mut ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: ::core::ffi::c_int,
    pub nodelen: ::core::ffi::c_int,
    pub nodemem: ::core::ffi::c_int,
    pub pedantic: ::core::ffi::c_int,
    pub _private: *mut ::core::ffi::c_void,
    pub loadsubset: ::core::ffi::c_int,
    pub linenumbers: ::core::ffi::c_int,
    pub catalogs: *mut ::core::ffi::c_void,
    pub recovery: ::core::ffi::c_int,
    pub progressive: ::core::ffi::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: ::core::ffi::c_int,
    pub docdict: ::core::ffi::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: ::core::ffi::c_int,
    pub nsNr: ::core::ffi::c_int,
    pub nsMax: ::core::ffi::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut ::core::ffi::c_uint,
    pub pushTab: *mut xmlStartTag,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: ::core::ffi::c_int,
    pub options: ::core::ffi::c_int,
    pub dictNames: ::core::ffi::c_int,
    pub freeElemsNr: ::core::ffi::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: ::core::ffi::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: ::core::ffi::c_ulong,
    pub sizeentities: ::core::ffi::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: ::core::ffi::c_int,
    pub nodeInfoMax: ::core::ffi::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: ::core::ffi::c_int,
    pub sizeentcopy: ::core::ffi::c_ulong,
    pub endCheckState: ::core::ffi::c_int,
    pub nbErrors: ::core::ffi::c_ushort,
    pub nbWarnings: ::core::ffi::c_ushort,
    pub maxAmpl: ::core::ffi::c_uint,
    pub nsdb: *mut xmlParserNsData,
    pub attrHashMax: ::core::ffi::c_uint,
    pub attrHash: *mut xmlAttrHashBucket,
}
pub type xmlAttrHashBucket = _xmlAttrHashBucket;
pub type xmlParserNsData = _xmlParserNsData;
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: ::core::ffi::c_ulong,
    pub begin_line: ::core::ffi::c_ulong,
    pub end_pos: ::core::ffi::c_ulong,
    pub end_line: ::core::ffi::c_ulong,
}
pub type xmlParserMode = ::core::ffi::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub message: *mut ::core::ffi::c_char,
    pub level: xmlErrorLevel,
    pub file: *mut ::core::ffi::c_char,
    pub line: ::core::ffi::c_int,
    pub str1: *mut ::core::ffi::c_char,
    pub str2: *mut ::core::ffi::c_char,
    pub str3: *mut ::core::ffi::c_char,
    pub int1: ::core::ffi::c_int,
    pub int2: ::core::ffi::c_int,
    pub ctxt: *mut ::core::ffi::c_void,
    pub node: *mut ::core::ffi::c_void,
}
pub type xmlErrorLevel = ::core::ffi::c_uint;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlStartTag = _xmlStartTag;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlParserInputState = ::core::ffi::c_int;
pub const XML_PARSER_XML_DECL: xmlParserInputState = 17;
pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const XML_PARSER_DTD: xmlParserInputState = 3;
pub const XML_PARSER_PI: xmlParserInputState = 2;
pub const XML_PARSER_MISC: xmlParserInputState = 1;
pub const XML_PARSER_START: xmlParserInputState = 0;
pub const XML_PARSER_EOF: xmlParserInputState = -1;
pub type xmlValidCtxt = _xmlValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut ::core::ffi::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: ::core::ffi::c_int,
    pub nodeMax: ::core::ffi::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub flags: ::core::ffi::c_uint,
    pub doc: xmlDocPtr,
    pub valid: ::core::ffi::c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: ::core::ffi::c_int,
    pub vstateMax: ::core::ffi::c_int,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
pub type xmlValidState = _xmlValidState;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlValidityWarningFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlValidityErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: ::core::ffi::c_ulong,
    pub length: ::core::ffi::c_ulong,
    pub buffer: *mut xmlParserNodeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandler {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: ::core::ffi::c_uint,
    pub _private: *mut ::core::ffi::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
pub type xmlStructuredErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> ()>;
pub type endElementNsSAX2Func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type startElementNsSAX2Func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        ::core::ffi::c_int,
        *mut *const xmlChar,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut *const xmlChar,
    ) -> (),
>;
pub type externalSubsetSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type cdataBlockSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type getParameterEntitySAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
pub type fatalErrorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type errorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type warningSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type commentSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type processingInstructionSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar) -> ()>;
pub type ignorableWhitespaceSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type charactersSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type referenceSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type endElementSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type startElementSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *mut *const xmlChar) -> (),
>;
pub type endDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type startDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> ()>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
    pub getSystemId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
    pub getLineNumber: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub getColumnNumber:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}
pub type unparsedEntityDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type elementDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
        xmlElementContentPtr,
    ) -> (),
>;
pub type xmlElementContentPtr = *mut xmlElementContent;
pub type xmlElementContent = _xmlElementContent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElementContent {
    pub type_0: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
pub type xmlElementContentOccur = ::core::ffi::c_uint;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = ::core::ffi::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type attributeDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *const xmlChar,
        xmlEnumerationPtr,
    ) -> (),
>;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type notationDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type entityDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
        *const xmlChar,
        *const xmlChar,
        *mut xmlChar,
    ) -> (),
>;
pub type getEntitySAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
pub type resolveEntitySAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlParserInputPtr,
>;
pub type hasExternalSubsetSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type hasInternalSubsetSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type isStandaloneSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type internalSubsetSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type xmlInputMatchCallback =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>;
pub type xmlInputOpenCallback =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void>;
pub type xmlOutputMatchCallback =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>;
pub type xmlOutputOpenCallback =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void>;
pub type xmlParserInputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, xmlCharEncoding) -> xmlParserInputBufferPtr,
>;
pub type xmlOutputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        xmlCharEncodingHandlerPtr,
        ::core::ffi::c_int,
    ) -> xmlOutputBufferPtr,
>;
pub type xmlInputCallback = _xmlInputCallback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlInputCallback {
    pub matchcallback: xmlInputMatchCallback,
    pub opencallback: xmlInputOpenCallback,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
}
pub const XML_FROM_IO: C2RustUnnamed = 8;
pub const XML_IO_UNKNOWN: xmlParserErrors = 1500;
pub const XML_IO_EAFNOSUPPORT: xmlParserErrors = 1556;
pub const XML_IO_EALREADY: xmlParserErrors = 1555;
pub const XML_IO_EINPROGRESS: xmlParserErrors = 1513;
pub const XML_IO_EADDRINUSE: xmlParserErrors = 1554;
pub const XML_IO_ENETUNREACH: xmlParserErrors = 1553;
pub const XML_IO_ETIMEDOUT: xmlParserErrors = 1541;
pub const XML_IO_ECONNREFUSED: xmlParserErrors = 1552;
pub const XML_IO_EISCONN: xmlParserErrors = 1551;
pub const XML_IO_ENOTSOCK: xmlParserErrors = 1550;
pub const XML_IO_EXDEV: xmlParserErrors = 1542;
pub const XML_IO_ESRCH: xmlParserErrors = 1540;
pub const XML_IO_ESPIPE: xmlParserErrors = 1539;
pub const XML_IO_EROFS: xmlParserErrors = 1538;
pub const XML_IO_ERANGE: xmlParserErrors = 1537;
pub const XML_IO_EPIPE: xmlParserErrors = 1536;
pub const XML_IO_EPERM: xmlParserErrors = 1535;
pub const XML_IO_ENXIO: xmlParserErrors = 1534;
pub const XML_IO_ENOTTY: xmlParserErrors = 1533;
pub const XML_IO_ENOTSUP: xmlParserErrors = 1532;
pub const XML_IO_ENOTEMPTY: xmlParserErrors = 1531;
pub const XML_IO_ENOTDIR: xmlParserErrors = 1530;
pub const XML_IO_ENOSYS: xmlParserErrors = 1529;
pub const XML_IO_ENOSPC: xmlParserErrors = 1528;
pub const XML_IO_ENOMEM: xmlParserErrors = 1527;
pub const XML_IO_ENOLCK: xmlParserErrors = 1526;
pub const XML_IO_ENOEXEC: xmlParserErrors = 1525;
pub const XML_IO_ENOENT: xmlParserErrors = 1524;
pub const XML_IO_ENODEV: xmlParserErrors = 1523;
pub const XML_IO_ENFILE: xmlParserErrors = 1522;
pub const XML_IO_ENAMETOOLONG: xmlParserErrors = 1521;
pub const XML_IO_EMSGSIZE: xmlParserErrors = 1520;
pub const XML_IO_EMLINK: xmlParserErrors = 1519;
pub const XML_IO_EMFILE: xmlParserErrors = 1518;
pub const XML_IO_EISDIR: xmlParserErrors = 1517;
pub const XML_IO_EIO: xmlParserErrors = 1516;
pub const XML_IO_EINVAL: xmlParserErrors = 1515;
pub const XML_IO_EINTR: xmlParserErrors = 1514;
pub const XML_IO_EFBIG: xmlParserErrors = 1512;
pub const XML_IO_EFAULT: xmlParserErrors = 1511;
pub const XML_IO_EEXIST: xmlParserErrors = 1510;
pub const XML_IO_EDOM: xmlParserErrors = 1509;
pub const XML_IO_EDEADLK: xmlParserErrors = 1508;
pub const XML_IO_ECHILD: xmlParserErrors = 1507;
pub const XML_IO_ECANCELED: xmlParserErrors = 1506;
pub const XML_IO_EBUSY: xmlParserErrors = 1505;
pub const XML_IO_EBADMSG: xmlParserErrors = 1504;
pub const XML_IO_EBADF: xmlParserErrors = 1503;
pub const XML_IO_EAGAIN: xmlParserErrors = 1502;
pub const XML_IO_EACCES: xmlParserErrors = 1501;
pub type ptrdiff_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlMemIOCtxt {
    pub mem: *mut ::core::ffi::c_char,
    pub cur: *const ::core::ffi::c_char,
    pub size: size_t,
}
pub const XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub type xmlOutputCallback = _xmlOutputCallback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputCallback {
    pub matchcallback: xmlOutputMatchCallback,
    pub opencallback: xmlOutputOpenCallback,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
}
pub type xmlIOHTTPWriteCtxtPtr = *mut xmlIOHTTPWriteCtxt_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlIOHTTPWriteCtxt_ {
    pub compression: ::core::ffi::c_int,
    pub uri: *mut ::core::ffi::c_char,
    pub doc_buff: *mut ::core::ffi::c_void,
}
pub const XML_IO_FLUSH: xmlParserErrors = 1545;
pub const XML_IO_ENCODER: xmlParserErrors = 1544;
pub const XML_IO_WRITE: xmlParserErrors = 1546;
pub type xmlIOHTTPWriteCtxt = xmlIOHTTPWriteCtxt_;
pub type xmlURIPtr = *mut xmlURI;
pub type xmlURI = _xmlURI;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: *mut ::core::ffi::c_char,
    pub opaque: *mut ::core::ffi::c_char,
    pub authority: *mut ::core::ffi::c_char,
    pub server: *mut ::core::ffi::c_char,
    pub user: *mut ::core::ffi::c_char,
    pub port: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub query: *mut ::core::ffi::c_char,
    pub fragment: *mut ::core::ffi::c_char,
    pub cleanup: ::core::ffi::c_int,
    pub query_raw: *mut ::core::ffi::c_char,
}
pub type xmlParserErrors = ::core::ffi::c_uint;
pub const XML_BUF_OVERFLOW: xmlParserErrors = 7000;
pub const XML_I18N_NO_OUTPUT: xmlParserErrors = 6004;
pub const XML_I18N_CONV_FAILED: xmlParserErrors = 6003;
pub const XML_I18N_EXCESS_HANDLER: xmlParserErrors = 6002;
pub const XML_I18N_NO_HANDLER: xmlParserErrors = 6001;
pub const XML_I18N_NO_NAME: xmlParserErrors = 6000;
pub const XML_CHECK_NAME_NOT_NULL: xmlParserErrors = 5037;
pub const XML_CHECK_WRONG_NAME: xmlParserErrors = 5036;
pub const XML_CHECK_OUTSIDE_DICT: xmlParserErrors = 5035;
pub const XML_CHECK_NOT_NCNAME: xmlParserErrors = 5034;
pub const XML_CHECK_NO_DICT: xmlParserErrors = 5033;
pub const XML_CHECK_NOT_UTF8: xmlParserErrors = 5032;
pub const XML_CHECK_NS_ANCESTOR: xmlParserErrors = 5031;
pub const XML_CHECK_NS_SCOPE: xmlParserErrors = 5030;
pub const XML_CHECK_WRONG_PARENT: xmlParserErrors = 5029;
pub const XML_CHECK_NO_HREF: xmlParserErrors = 5028;
pub const XML_CHECK_NOT_NS_DECL: xmlParserErrors = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: xmlParserErrors = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: xmlParserErrors = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: xmlParserErrors = 5024;
pub const XML_CHECK_NOT_ATTR: xmlParserErrors = 5023;
pub const XML_CHECK_NOT_DTD: xmlParserErrors = 5022;
pub const XML_CHECK_WRONG_NEXT: xmlParserErrors = 5021;
pub const XML_CHECK_NO_NEXT: xmlParserErrors = 5020;
pub const XML_CHECK_WRONG_PREV: xmlParserErrors = 5019;
pub const XML_CHECK_NO_PREV: xmlParserErrors = 5018;
pub const XML_CHECK_WRONG_DOC: xmlParserErrors = 5017;
pub const XML_CHECK_NO_ELEM: xmlParserErrors = 5016;
pub const XML_CHECK_NO_NAME: xmlParserErrors = 5015;
pub const XML_CHECK_NO_DOC: xmlParserErrors = 5014;
pub const XML_CHECK_NO_PARENT: xmlParserErrors = 5013;
pub const XML_CHECK_ENTITY_TYPE: xmlParserErrors = 5012;
pub const XML_CHECK_UNKNOWN_NODE: xmlParserErrors = 5011;
pub const XML_CHECK_FOUND_NOTATION: xmlParserErrors = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: xmlParserErrors = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: xmlParserErrors = 5008;
pub const XML_CHECK_FOUND_COMMENT: xmlParserErrors = 5007;
pub const XML_CHECK_FOUND_PI: xmlParserErrors = 5006;
pub const XML_CHECK_FOUND_ENTITY: xmlParserErrors = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: xmlParserErrors = 5004;
pub const XML_CHECK_FOUND_CDATA: xmlParserErrors = 5003;
pub const XML_CHECK_FOUND_TEXT: xmlParserErrors = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: xmlParserErrors = 5001;
pub const XML_CHECK_FOUND_ELEMENT: xmlParserErrors = 5000;
pub const XML_MODULE_CLOSE: xmlParserErrors = 4901;
pub const XML_MODULE_OPEN: xmlParserErrors = 4900;
pub const XML_SCHEMATRONV_REPORT: xmlParserErrors = 4001;
pub const XML_SCHEMATRONV_ASSERT: xmlParserErrors = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: xmlParserErrors = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: xmlParserErrors = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: xmlParserErrors = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: xmlParserErrors = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: xmlParserErrors = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: xmlParserErrors = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: xmlParserErrors = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: xmlParserErrors = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: xmlParserErrors = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: xmlParserErrors = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: xmlParserErrors = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: xmlParserErrors = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: xmlParserErrors = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: xmlParserErrors = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: xmlParserErrors = 3077;
pub const XML_SCHEMAP_SRC_CT_1: xmlParserErrors = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: xmlParserErrors = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: xmlParserErrors = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: xmlParserErrors = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: xmlParserErrors = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: xmlParserErrors = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: xmlParserErrors = 3070;
pub const XML_SCHEMAP_INTERNAL: xmlParserErrors = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: xmlParserErrors = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: xmlParserErrors = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: xmlParserErrors = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: xmlParserErrors = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: xmlParserErrors = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: xmlParserErrors = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: xmlParserErrors = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: xmlParserErrors = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: xmlParserErrors = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: xmlParserErrors = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: xmlParserErrors = 3058;
pub const XML_SCHEMAP_NO_XSI: xmlParserErrors = 3057;
pub const XML_SCHEMAP_NO_XMLNS: xmlParserErrors = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: xmlParserErrors = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: xmlParserErrors = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: xmlParserErrors = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: xmlParserErrors = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: xmlParserErrors = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: xmlParserErrors = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: xmlParserErrors = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: xmlParserErrors = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: xmlParserErrors = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: xmlParserErrors = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: xmlParserErrors = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: xmlParserErrors = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: xmlParserErrors = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: xmlParserErrors = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: xmlParserErrors = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: xmlParserErrors = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: xmlParserErrors = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: xmlParserErrors = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: xmlParserErrors = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: xmlParserErrors = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: xmlParserErrors = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: xmlParserErrors = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: xmlParserErrors = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: xmlParserErrors = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: xmlParserErrors = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: xmlParserErrors = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: xmlParserErrors = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: xmlParserErrors = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: xmlParserErrors = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: xmlParserErrors = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: xmlParserErrors = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: xmlParserErrors = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: xmlParserErrors = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: xmlParserErrors = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: xmlParserErrors = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: xmlParserErrors = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: xmlParserErrors = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: xmlParserErrors = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: xmlParserErrors = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: xmlParserErrors = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: xmlParserErrors = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: xmlParserErrors = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: xmlParserErrors = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: xmlParserErrors = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: xmlParserErrors = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: xmlParserErrors = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: xmlParserErrors = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: xmlParserErrors = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: xmlParserErrors = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: xmlParserErrors = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: xmlParserErrors = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: xmlParserErrors = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: xmlParserErrors = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: xmlParserErrors = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: xmlParserErrors = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: xmlParserErrors = 3000;
pub const XML_HTTP_UNKNOWN_HOST: xmlParserErrors = 2022;
pub const XML_HTTP_USE_IP: xmlParserErrors = 2021;
pub const XML_HTTP_URL_SYNTAX: xmlParserErrors = 2020;
pub const XML_FTP_URL_SYNTAX: xmlParserErrors = 2003;
pub const XML_FTP_ACCNT: xmlParserErrors = 2002;
pub const XML_FTP_EPSV_ANSWER: xmlParserErrors = 2001;
pub const XML_FTP_PASV_ANSWER: xmlParserErrors = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: xmlParserErrors = 1955;
pub const XML_C14N_UNKNOW_NODE: xmlParserErrors = 1954;
pub const XML_C14N_INVALID_NODE: xmlParserErrors = 1953;
pub const XML_C14N_CREATE_STACK: xmlParserErrors = 1952;
pub const XML_C14N_REQUIRES_UTF8: xmlParserErrors = 1951;
pub const XML_C14N_CREATE_CTXT: xmlParserErrors = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: xmlParserErrors = 1903;
pub const XML_XPTR_EVAL_FAILED: xmlParserErrors = 1902;
pub const XML_XPTR_CHILDSEQ_START: xmlParserErrors = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: xmlParserErrors = 1900;
pub const XML_SCHEMAV_MISC: xmlParserErrors = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: xmlParserErrors = 1878;
pub const XML_SCHEMAV_CVC_IDC: xmlParserErrors = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: xmlParserErrors = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: xmlParserErrors = 1875;
pub const XML_SCHEMAV_CVC_AU: xmlParserErrors = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: xmlParserErrors = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: xmlParserErrors = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: xmlParserErrors = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: xmlParserErrors = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: xmlParserErrors = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: xmlParserErrors = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: xmlParserErrors = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: xmlParserErrors = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: xmlParserErrors = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: xmlParserErrors = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: xmlParserErrors = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: xmlParserErrors = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: xmlParserErrors = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: xmlParserErrors = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: xmlParserErrors = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: xmlParserErrors = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: xmlParserErrors = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: xmlParserErrors = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: xmlParserErrors = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: xmlParserErrors = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: xmlParserErrors = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: xmlParserErrors = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: xmlParserErrors = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: xmlParserErrors = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: xmlParserErrors = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: xmlParserErrors = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: xmlParserErrors = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: xmlParserErrors = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: xmlParserErrors = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: xmlParserErrors = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: xmlParserErrors = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: xmlParserErrors = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: xmlParserErrors = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: xmlParserErrors = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: xmlParserErrors = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: xmlParserErrors = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: xmlParserErrors = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: xmlParserErrors = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: xmlParserErrors = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: xmlParserErrors = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: xmlParserErrors = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: xmlParserErrors = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: xmlParserErrors = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: xmlParserErrors = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: xmlParserErrors = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: xmlParserErrors = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: xmlParserErrors = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: xmlParserErrors = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: xmlParserErrors = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: xmlParserErrors = 1824;
pub const XML_SCHEMAV_FACET: xmlParserErrors = 1823;
pub const XML_SCHEMAV_VALUE: xmlParserErrors = 1822;
pub const XML_SCHEMAV_ATTRINVALID: xmlParserErrors = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: xmlParserErrors = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: xmlParserErrors = 1819;
pub const XML_SCHEMAV_INTERNAL: xmlParserErrors = 1818;
pub const XML_SCHEMAV_CONSTRUCT: xmlParserErrors = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: xmlParserErrors = 1816;
pub const XML_SCHEMAV_INVALIDELEM: xmlParserErrors = 1815;
pub const XML_SCHEMAV_INVALIDATTR: xmlParserErrors = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: xmlParserErrors = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: xmlParserErrors = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: xmlParserErrors = 1811;
pub const XML_SCHEMAV_ELEMCONT: xmlParserErrors = 1810;
pub const XML_SCHEMAV_NOTEMPTY: xmlParserErrors = 1809;
pub const XML_SCHEMAV_ISABSTRACT: xmlParserErrors = 1808;
pub const XML_SCHEMAV_NOROLLBACK: xmlParserErrors = 1807;
pub const XML_SCHEMAV_NOTYPE: xmlParserErrors = 1806;
pub const XML_SCHEMAV_WRONGELEM: xmlParserErrors = 1805;
pub const XML_SCHEMAV_MISSING: xmlParserErrors = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: xmlParserErrors = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: xmlParserErrors = 1802;
pub const XML_SCHEMAV_NOROOT: xmlParserErrors = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: xmlParserErrors = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: xmlParserErrors = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: xmlParserErrors = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: xmlParserErrors = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: xmlParserErrors = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: xmlParserErrors = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: xmlParserErrors = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: xmlParserErrors = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: xmlParserErrors = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: xmlParserErrors = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: xmlParserErrors = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: xmlParserErrors = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: xmlParserErrors = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: xmlParserErrors = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: xmlParserErrors = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: xmlParserErrors = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: xmlParserErrors = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: xmlParserErrors = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: xmlParserErrors = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: xmlParserErrors = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: xmlParserErrors = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: xmlParserErrors = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: xmlParserErrors = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: xmlParserErrors = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: xmlParserErrors = 1776;
pub const XML_SCHEMAP_RECURSIVE: xmlParserErrors = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: xmlParserErrors = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: xmlParserErrors = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: xmlParserErrors = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: xmlParserErrors = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: xmlParserErrors = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: xmlParserErrors = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: xmlParserErrors = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: xmlParserErrors = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: xmlParserErrors = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: xmlParserErrors = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: xmlParserErrors = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: xmlParserErrors = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: xmlParserErrors = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: xmlParserErrors = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: xmlParserErrors = 1760;
pub const XML_SCHEMAP_NOROOT: xmlParserErrors = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: xmlParserErrors = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: xmlParserErrors = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: xmlParserErrors = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: xmlParserErrors = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: xmlParserErrors = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: xmlParserErrors = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: xmlParserErrors = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: xmlParserErrors = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: xmlParserErrors = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: xmlParserErrors = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: xmlParserErrors = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: xmlParserErrors = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: xmlParserErrors = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: xmlParserErrors = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: xmlParserErrors = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: xmlParserErrors = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: xmlParserErrors = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: xmlParserErrors = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: xmlParserErrors = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: xmlParserErrors = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: xmlParserErrors = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: xmlParserErrors = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: xmlParserErrors = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: xmlParserErrors = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: xmlParserErrors = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: xmlParserErrors = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: xmlParserErrors = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: xmlParserErrors = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: xmlParserErrors = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: xmlParserErrors = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: xmlParserErrors = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: xmlParserErrors = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: xmlParserErrors = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: xmlParserErrors = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: xmlParserErrors = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: xmlParserErrors = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: xmlParserErrors = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: xmlParserErrors = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: xmlParserErrors = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: xmlParserErrors = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: xmlParserErrors = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: xmlParserErrors = 1717;
pub const XML_SCHEMAP_INVALID_FACET: xmlParserErrors = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: xmlParserErrors = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: xmlParserErrors = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: xmlParserErrors = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: xmlParserErrors = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: xmlParserErrors = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: xmlParserErrors = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: xmlParserErrors = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: xmlParserErrors = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: xmlParserErrors = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: xmlParserErrors = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: xmlParserErrors = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: xmlParserErrors = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: xmlParserErrors = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: xmlParserErrors = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: xmlParserErrors = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: xmlParserErrors = 1700;
pub const XML_CATALOG_RECURSION: xmlParserErrors = 1654;
pub const XML_CATALOG_NOT_CATALOG: xmlParserErrors = 1653;
pub const XML_CATALOG_PREFER_VALUE: xmlParserErrors = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: xmlParserErrors = 1651;
pub const XML_CATALOG_MISSING_ATTR: xmlParserErrors = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: xmlParserErrors = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: xmlParserErrors = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: xmlParserErrors = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: xmlParserErrors = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: xmlParserErrors = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: xmlParserErrors = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: xmlParserErrors = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: xmlParserErrors = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: xmlParserErrors = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: xmlParserErrors = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: xmlParserErrors = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: xmlParserErrors = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: xmlParserErrors = 1606;
pub const XML_XINCLUDE_HREF_URI: xmlParserErrors = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: xmlParserErrors = 1604;
pub const XML_XINCLUDE_NO_HREF: xmlParserErrors = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: xmlParserErrors = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: xmlParserErrors = 1601;
pub const XML_XINCLUDE_RECURSION: xmlParserErrors = 1600;
pub const XML_IO_LOAD_ERROR: xmlParserErrors = 1549;
pub const XML_IO_BUFFER_FULL: xmlParserErrors = 1548;
pub const XML_IO_NO_INPUT: xmlParserErrors = 1547;
pub const XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1543;
pub const XML_REGEXP_COMPILE_ERROR: xmlParserErrors = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: xmlParserErrors = 1403;
pub const XML_SAVE_NO_DOCTYPE: xmlParserErrors = 1402;
pub const XML_SAVE_CHAR_INVALID: xmlParserErrors = 1401;
pub const XML_SAVE_NOT_UTF8: xmlParserErrors = 1400;
pub const XML_TREE_NOT_UTF8: xmlParserErrors = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: xmlParserErrors = 1302;
pub const XML_TREE_INVALID_DEC: xmlParserErrors = 1301;
pub const XML_TREE_INVALID_HEX: xmlParserErrors = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: xmlParserErrors = 1221;
pub const XML_XPATH_ENCODING_ERROR: xmlParserErrors = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: xmlParserErrors = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: xmlParserErrors = 1218;
pub const XML_XPTR_RESOURCE_ERROR: xmlParserErrors = 1217;
pub const XML_XPTR_SYNTAX_ERROR: xmlParserErrors = 1216;
pub const XML_XPATH_MEMORY_ERROR: xmlParserErrors = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: xmlParserErrors = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: xmlParserErrors = 1213;
pub const XML_XPATH_INVALID_ARITY: xmlParserErrors = 1212;
pub const XML_XPATH_INVALID_TYPE: xmlParserErrors = 1211;
pub const XML_XPATH_INVALID_OPERAND: xmlParserErrors = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: xmlParserErrors = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: xmlParserErrors = 1208;
pub const XML_XPATH_EXPR_ERROR: xmlParserErrors = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: xmlParserErrors = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: xmlParserErrors = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: xmlParserErrors = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: xmlParserErrors = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: xmlParserErrors = 1202;
pub const XML_XPATH_NUMBER_ERROR: xmlParserErrors = 1201;
pub const XML_XPATH_EXPRESSION_OK: xmlParserErrors = 1200;
pub const XML_RNGP_XML_NS: xmlParserErrors = 1122;
pub const XML_RNGP_XMLNS_NAME: xmlParserErrors = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: xmlParserErrors = 1120;
pub const XML_RNGP_VALUE_EMPTY: xmlParserErrors = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: xmlParserErrors = 1118;
pub const XML_RNGP_URI_FRAGMENT: xmlParserErrors = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: xmlParserErrors = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: xmlParserErrors = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: xmlParserErrors = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: xmlParserErrors = 1113;
pub const XML_RNGP_TYPE_VALUE: xmlParserErrors = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: xmlParserErrors = 1111;
pub const XML_RNGP_TYPE_MISSING: xmlParserErrors = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: xmlParserErrors = 1109;
pub const XML_RNGP_TEXT_EXPECTED: xmlParserErrors = 1108;
pub const XML_RNGP_START_MISSING: xmlParserErrors = 1107;
pub const XML_RNGP_START_EMPTY: xmlParserErrors = 1106;
pub const XML_RNGP_START_CONTENT: xmlParserErrors = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: xmlParserErrors = 1103;
pub const XML_RNGP_REF_NO_NAME: xmlParserErrors = 1102;
pub const XML_RNGP_REF_NO_DEF: xmlParserErrors = 1101;
pub const XML_RNGP_REF_NAME_INVALID: xmlParserErrors = 1100;
pub const XML_RNGP_REF_CYCLE: xmlParserErrors = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: xmlParserErrors = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: xmlParserErrors = 1097;
pub const XML_RNGP_PAT_START_VALUE: xmlParserErrors = 1096;
pub const XML_RNGP_PAT_START_TEXT: xmlParserErrors = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: xmlParserErrors = 1094;
pub const XML_RNGP_PAT_START_LIST: xmlParserErrors = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: xmlParserErrors = 1092;
pub const XML_RNGP_PAT_START_GROUP: xmlParserErrors = 1091;
pub const XML_RNGP_PAT_START_EMPTY: xmlParserErrors = 1090;
pub const XML_RNGP_PAT_START_DATA: xmlParserErrors = 1089;
pub const XML_RNGP_PAT_START_ATTR: xmlParserErrors = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: xmlParserErrors = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: xmlParserErrors = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: xmlParserErrors = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: xmlParserErrors = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: xmlParserErrors = 1083;
pub const XML_RNGP_PAT_LIST_REF: xmlParserErrors = 1082;
pub const XML_RNGP_PAT_LIST_LIST: xmlParserErrors = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: xmlParserErrors = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: xmlParserErrors = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: xmlParserErrors = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: xmlParserErrors = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: xmlParserErrors = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: xmlParserErrors = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: xmlParserErrors = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: xmlParserErrors = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: xmlParserErrors = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: xmlParserErrors = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: xmlParserErrors = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: xmlParserErrors = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: xmlParserErrors = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: xmlParserErrors = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: xmlParserErrors = 1066;
pub const XML_RNGP_PARSE_ERROR: xmlParserErrors = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: xmlParserErrors = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: xmlParserErrors = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: xmlParserErrors = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: xmlParserErrors = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: xmlParserErrors = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: xmlParserErrors = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: xmlParserErrors = 1058;
pub const XML_RNGP_NSNAME_NO_NS: xmlParserErrors = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: xmlParserErrors = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: xmlParserErrors = 1055;
pub const XML_RNGP_NEED_COMBINE: xmlParserErrors = 1054;
pub const XML_RNGP_NAME_MISSING: xmlParserErrors = 1053;
pub const XML_RNGP_MISSING_HREF: xmlParserErrors = 1052;
pub const XML_RNGP_INVALID_VALUE: xmlParserErrors = 1051;
pub const XML_RNGP_INVALID_URI: xmlParserErrors = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: xmlParserErrors = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: xmlParserErrors = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: xmlParserErrors = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: xmlParserErrors = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: xmlParserErrors = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: xmlParserErrors = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: xmlParserErrors = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: xmlParserErrors = 1042;
pub const XML_RNGP_HREF_ERROR: xmlParserErrors = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: xmlParserErrors = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: xmlParserErrors = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: xmlParserErrors = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: xmlParserErrors = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: xmlParserErrors = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: xmlParserErrors = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: xmlParserErrors = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: xmlParserErrors = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: xmlParserErrors = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: xmlParserErrors = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: xmlParserErrors = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: xmlParserErrors = 1029;
pub const XML_RNGP_EXCEPT_MISSING: xmlParserErrors = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: xmlParserErrors = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: xmlParserErrors = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: xmlParserErrors = 1025;
pub const XML_RNGP_EMPTY_CONTENT: xmlParserErrors = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: xmlParserErrors = 1023;
pub const XML_RNGP_EMPTY: xmlParserErrors = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: xmlParserErrors = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: xmlParserErrors = 1020;
pub const XML_RNGP_ELEMENT_NAME: xmlParserErrors = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: xmlParserErrors = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: xmlParserErrors = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: xmlParserErrors = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: xmlParserErrors = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: xmlParserErrors = 1014;
pub const XML_RNGP_DEFINE_MISSING: xmlParserErrors = 1013;
pub const XML_RNGP_DEFINE_EMPTY: xmlParserErrors = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: xmlParserErrors = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1010;
pub const XML_RNGP_DATA_CONTENT: xmlParserErrors = 1009;
pub const XML_RNGP_CREATE_FAILURE: xmlParserErrors = 1008;
pub const XML_RNGP_CHOICE_EMPTY: xmlParserErrors = 1007;
pub const XML_RNGP_CHOICE_CONTENT: xmlParserErrors = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: xmlParserErrors = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: xmlParserErrors = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: xmlParserErrors = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: xmlParserErrors = 1002;
pub const XML_RNGP_ATTR_CONFLICT: xmlParserErrors = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: xmlParserErrors = 1000;
pub const XML_HTML_INCORRECTLY_OPENED_COMMENT: xmlParserErrors = 802;
pub const XML_HTML_UNKNOWN_TAG: xmlParserErrors = 801;
pub const XML_HTML_STRUCURE_ERROR: xmlParserErrors = 800;
pub const XML_DTD_DUP_TOKEN: xmlParserErrors = 541;
pub const XML_DTD_XMLID_TYPE: xmlParserErrors = 540;
pub const XML_DTD_XMLID_VALUE: xmlParserErrors = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: xmlParserErrors = 538;
pub const XML_DTD_UNKNOWN_NOTATION: xmlParserErrors = 537;
pub const XML_DTD_UNKNOWN_ID: xmlParserErrors = 536;
pub const XML_DTD_UNKNOWN_ENTITY: xmlParserErrors = 535;
pub const XML_DTD_UNKNOWN_ELEM: xmlParserErrors = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: xmlParserErrors = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: xmlParserErrors = 532;
pub const XML_DTD_ROOT_NAME: xmlParserErrors = 531;
pub const XML_DTD_NOT_STANDALONE: xmlParserErrors = 530;
pub const XML_DTD_NOT_PCDATA: xmlParserErrors = 529;
pub const XML_DTD_NOT_EMPTY: xmlParserErrors = 528;
pub const XML_DTD_NOTATION_VALUE: xmlParserErrors = 527;
pub const XML_DTD_NOTATION_REDEFINED: xmlParserErrors = 526;
pub const XML_DTD_NO_ROOT: xmlParserErrors = 525;
pub const XML_DTD_NO_PREFIX: xmlParserErrors = 524;
pub const XML_DTD_NO_ELEM_NAME: xmlParserErrors = 523;
pub const XML_DTD_NO_DTD: xmlParserErrors = 522;
pub const XML_DTD_NO_DOC: xmlParserErrors = 521;
pub const XML_DTD_MULTIPLE_ID: xmlParserErrors = 520;
pub const XML_DTD_MIXED_CORRUPT: xmlParserErrors = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: xmlParserErrors = 518;
pub const XML_DTD_LOAD_ERROR: xmlParserErrors = 517;
pub const XML_DTD_INVALID_DEFAULT: xmlParserErrors = 516;
pub const XML_DTD_INVALID_CHILD: xmlParserErrors = 515;
pub const XML_DTD_ID_SUBSET: xmlParserErrors = 514;
pub const XML_DTD_ID_REDEFINED: xmlParserErrors = 513;
pub const XML_DTD_ID_FIXED: xmlParserErrors = 512;
pub const XML_DTD_ENTITY_TYPE: xmlParserErrors = 511;
pub const XML_DTD_EMPTY_NOTATION: xmlParserErrors = 510;
pub const XML_DTD_ELEM_REDEFINED: xmlParserErrors = 509;
pub const XML_DTD_ELEM_NAMESPACE: xmlParserErrors = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: xmlParserErrors = 507;
pub const XML_DTD_DIFFERENT_PREFIX: xmlParserErrors = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: xmlParserErrors = 505;
pub const XML_DTD_CONTENT_MODEL: xmlParserErrors = 504;
pub const XML_DTD_CONTENT_ERROR: xmlParserErrors = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: xmlParserErrors = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: xmlParserErrors = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: xmlParserErrors = 500;
pub const XML_NS_ERR_COLON: xmlParserErrors = 205;
pub const XML_NS_ERR_EMPTY: xmlParserErrors = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 203;
pub const XML_NS_ERR_QNAME: xmlParserErrors = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: xmlParserErrors = 201;
pub const XML_NS_ERR_XML_NAMESPACE: xmlParserErrors = 200;
pub const XML_WAR_ENCODING_MISMATCH: xmlParserErrors = 113;
pub const XML_ERR_COMMENT_ABRUPTLY_ENDED: xmlParserErrors = 112;
pub const XML_ERR_USER_STOP: xmlParserErrors = 111;
pub const XML_ERR_NAME_TOO_LONG: xmlParserErrors = 110;
pub const XML_ERR_VERSION_MISMATCH: xmlParserErrors = 109;
pub const XML_ERR_UNKNOWN_VERSION: xmlParserErrors = 108;
pub const XML_WAR_ENTITY_REDEFINED: xmlParserErrors = 107;
pub const XML_WAR_NS_COLUMN: xmlParserErrors = 106;
pub const XML_ERR_NOTATION_PROCESSING: xmlParserErrors = 105;
pub const XML_ERR_ENTITY_PROCESSING: xmlParserErrors = 104;
pub const XML_ERR_NOT_STANDALONE: xmlParserErrors = 103;
pub const XML_WAR_SPACE_VALUE: xmlParserErrors = 102;
pub const XML_ERR_MISSING_ENCODING: xmlParserErrors = 101;
pub const XML_WAR_NS_URI_RELATIVE: xmlParserErrors = 100;
pub const XML_WAR_NS_URI: xmlParserErrors = 99;
pub const XML_WAR_LANG_VALUE: xmlParserErrors = 98;
pub const XML_WAR_UNKNOWN_VERSION: xmlParserErrors = 97;
pub const XML_ERR_VERSION_MISSING: xmlParserErrors = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: xmlParserErrors = 95;
pub const XML_ERR_NO_DTD: xmlParserErrors = 94;
pub const XML_WAR_CATALOG_PI: xmlParserErrors = 93;
pub const XML_ERR_URI_FRAGMENT: xmlParserErrors = 92;
pub const XML_ERR_INVALID_URI: xmlParserErrors = 91;
pub const XML_ERR_ENTITY_BOUNDARY: xmlParserErrors = 90;
pub const XML_ERR_ENTITY_LOOP: xmlParserErrors = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: xmlParserErrors = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: xmlParserErrors = 87;
pub const XML_ERR_EXTRA_CONTENT: xmlParserErrors = 86;
pub const XML_ERR_NOT_WELL_BALANCED: xmlParserErrors = 85;
pub const XML_ERR_VALUE_REQUIRED: xmlParserErrors = 84;
pub const XML_ERR_CONDSEC_INVALID: xmlParserErrors = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: xmlParserErrors = 82;
pub const XML_ERR_INVALID_ENCODING: xmlParserErrors = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: xmlParserErrors = 80;
pub const XML_ERR_ENCODING_NAME: xmlParserErrors = 79;
pub const XML_ERR_STANDALONE_VALUE: xmlParserErrors = 78;
pub const XML_ERR_TAG_NOT_FINISHED: xmlParserErrors = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: xmlParserErrors = 76;
pub const XML_ERR_EQUAL_REQUIRED: xmlParserErrors = 75;
pub const XML_ERR_LTSLASH_REQUIRED: xmlParserErrors = 74;
pub const XML_ERR_GT_REQUIRED: xmlParserErrors = 73;
pub const XML_ERR_LT_REQUIRED: xmlParserErrors = 72;
pub const XML_ERR_PUBID_REQUIRED: xmlParserErrors = 71;
pub const XML_ERR_URI_REQUIRED: xmlParserErrors = 70;
pub const XML_ERR_PCDATA_REQUIRED: xmlParserErrors = 69;
pub const XML_ERR_NAME_REQUIRED: xmlParserErrors = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: xmlParserErrors = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: xmlParserErrors = 66;
pub const XML_ERR_SPACE_REQUIRED: xmlParserErrors = 65;
pub const XML_ERR_RESERVED_XML_NAME: xmlParserErrors = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: xmlParserErrors = 63;
pub const XML_ERR_MISPLACED_CDATA_END: xmlParserErrors = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: xmlParserErrors = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: xmlParserErrors = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: xmlParserErrors = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: xmlParserErrors = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: xmlParserErrors = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: xmlParserErrors = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: xmlParserErrors = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: xmlParserErrors = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: xmlParserErrors = 53;
pub const XML_ERR_MIXED_NOT_STARTED: xmlParserErrors = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: xmlParserErrors = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: xmlParserErrors = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: xmlParserErrors = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: xmlParserErrors = 48;
pub const XML_ERR_PI_NOT_FINISHED: xmlParserErrors = 47;
pub const XML_ERR_PI_NOT_STARTED: xmlParserErrors = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: xmlParserErrors = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: xmlParserErrors = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: xmlParserErrors = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: xmlParserErrors = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: xmlParserErrors = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: xmlParserErrors = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: xmlParserErrors = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: xmlParserErrors = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: xmlParserErrors = 36;
pub const XML_ERR_NS_DECL_ERROR: xmlParserErrors = 35;
pub const XML_ERR_STRING_NOT_CLOSED: xmlParserErrors = 34;
pub const XML_ERR_STRING_NOT_STARTED: xmlParserErrors = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: xmlParserErrors = 32;
pub const XML_ERR_UNKNOWN_ENCODING: xmlParserErrors = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: xmlParserErrors = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: xmlParserErrors = 29;
pub const XML_ERR_UNPARSED_ENTITY: xmlParserErrors = 28;
pub const XML_WAR_UNDECLARED_ENTITY: xmlParserErrors = 27;
pub const XML_ERR_UNDECLARED_ENTITY: xmlParserErrors = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: xmlParserErrors = 25;
pub const XML_ERR_PEREF_NO_NAME: xmlParserErrors = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: xmlParserErrors = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: xmlParserErrors = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: xmlParserErrors = 21;
pub const XML_ERR_PEREF_IN_EPILOG: xmlParserErrors = 20;
pub const XML_ERR_PEREF_IN_PROLOG: xmlParserErrors = 19;
pub const XML_ERR_PEREF_AT_EOF: xmlParserErrors = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: xmlParserErrors = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: xmlParserErrors = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: xmlParserErrors = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: xmlParserErrors = 14;
pub const XML_ERR_CHARREF_IN_DTD: xmlParserErrors = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: xmlParserErrors = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: xmlParserErrors = 11;
pub const XML_ERR_CHARREF_AT_EOF: xmlParserErrors = 10;
pub const XML_ERR_INVALID_CHAR: xmlParserErrors = 9;
pub const XML_ERR_INVALID_CHARREF: xmlParserErrors = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: xmlParserErrors = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: xmlParserErrors = 6;
pub const XML_ERR_DOCUMENT_END: xmlParserErrors = 5;
pub const XML_ERR_DOCUMENT_EMPTY: xmlParserErrors = 4;
pub const XML_ERR_DOCUMENT_START: xmlParserErrors = 3;
pub const XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const XML_ERR_OK: xmlParserErrors = 0;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub const XML_CATA_ALLOW_GLOBAL: xmlCatalogAllow = 1;
pub type xmlCatalogAllow = ::core::ffi::c_uint;
pub const XML_CATA_ALLOW_ALL: xmlCatalogAllow = 3;
pub const XML_CATA_ALLOW_DOCUMENT: xmlCatalogAllow = 2;
pub const XML_CATA_ALLOW_NONE: xmlCatalogAllow = 0;
pub const XML_PARSE_NONET: C2RustUnnamed_0 = 2048;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const XML_FROM_URI: C2RustUnnamed = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed = 28;
pub const XML_FROM_I18N: C2RustUnnamed = 27;
pub const XML_FROM_MODULE: C2RustUnnamed = 26;
pub const XML_FROM_WRITER: C2RustUnnamed = 25;
pub const XML_FROM_CHECK: C2RustUnnamed = 24;
pub const XML_FROM_VALID: C2RustUnnamed = 23;
pub const XML_FROM_XSLT: C2RustUnnamed = 22;
pub const XML_FROM_C14N: C2RustUnnamed = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed = 13;
pub const XML_FROM_XPATH: C2RustUnnamed = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed = 11;
pub const XML_FROM_HTTP: C2RustUnnamed = 10;
pub const XML_FROM_FTP: C2RustUnnamed = 9;
pub const XML_FROM_OUTPUT: C2RustUnnamed = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed = 6;
pub const XML_FROM_HTML: C2RustUnnamed = 5;
pub const XML_FROM_DTD: C2RustUnnamed = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed = 3;
pub const XML_FROM_TREE: C2RustUnnamed = 2;
pub const XML_FROM_PARSER: C2RustUnnamed = 1;
pub const XML_FROM_NONE: C2RustUnnamed = 0;
pub type xmlExternalEntityLoader = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        xmlParserCtxtPtr,
    ) -> xmlParserInputPtr,
>;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_0 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_0 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_0 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_0 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_0 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_0 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_0 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_0 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_0 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_0 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_0 = 4096;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_0 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_0 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_0 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_0 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_0 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_0 = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_0 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_0 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_0 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_0 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlStringIOCtxt {
    pub str_0: *const xmlChar,
}
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EDEADLK: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const ENAMETOOLONG: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const ENOLCK: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const ENOTEMPTY: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const EBADMSG: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const ENOTSOCK: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
pub const EMSGSIZE: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const EAFNOSUPPORT: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const EADDRINUSE: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const ENETUNREACH: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const EISCONN: ::core::ffi::c_int = 106 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const ECONNREFUSED: ::core::ffi::c_int = 111 as ::core::ffi::c_int;
pub const EALREADY: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
pub const ECANCELED: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ENXIO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ENOEXEC: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const ECHILD: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EFAULT: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const EBUSY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const EXDEV: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const ENODEV: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ENFILE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const EMFILE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const ENOTTY: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const EFBIG: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const ENOSPC: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const ESPIPE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const EROFS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const EMLINK: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const EPIPE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EDOM: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const _STAT_VER_LINUX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _STAT_VER: ::core::ffi::c_int = _STAT_VER_LINUX;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const ::core::ffi::c_char,
    mut __statbuf: *mut stat,
) -> ::core::ffi::c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const XML_SAX2_MAGIC: ::core::ffi::c_uint = 0xdeedbeaf as ::core::ffi::c_uint;
pub const MINLEN: ::core::ffi::c_int = 4000 as ::core::ffi::c_int;
pub const MAX_INPUT_CALLBACK: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
static mut xmlInputCallbackTable: [xmlInputCallback; 15] = [xmlInputCallback {
    matchcallback: None,
    opencallback: None,
    readcallback: None,
    closecallback: None,
}; 15];
static mut xmlInputCallbackNr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlInputCallbackInitialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAX_OUTPUT_CALLBACK: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
static mut xmlOutputCallbackTable: [xmlOutputCallback; 15] = [xmlOutputCallback {
    matchcallback: None,
    opencallback: None,
    writecallback: None,
    closecallback: None,
}; 15];
static mut xmlOutputCallbackNr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlOutputCallbackInitialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut IOerr: [*const ::core::ffi::c_char; 57] = [
    b"Unknown IO error\0" as *const u8 as *const ::core::ffi::c_char,
    b"Permission denied\0" as *const u8 as *const ::core::ffi::c_char,
    b"Resource temporarily unavailable\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bad file descriptor\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bad message\0" as *const u8 as *const ::core::ffi::c_char,
    b"Resource busy\0" as *const u8 as *const ::core::ffi::c_char,
    b"Operation canceled\0" as *const u8 as *const ::core::ffi::c_char,
    b"No child processes\0" as *const u8 as *const ::core::ffi::c_char,
    b"Resource deadlock avoided\0" as *const u8 as *const ::core::ffi::c_char,
    b"Domain error\0" as *const u8 as *const ::core::ffi::c_char,
    b"File exists\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bad address\0" as *const u8 as *const ::core::ffi::c_char,
    b"File too large\0" as *const u8 as *const ::core::ffi::c_char,
    b"Operation in progress\0" as *const u8 as *const ::core::ffi::c_char,
    b"Interrupted function call\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
    b"Input/output error\0" as *const u8 as *const ::core::ffi::c_char,
    b"Is a directory\0" as *const u8 as *const ::core::ffi::c_char,
    b"Too many open files\0" as *const u8 as *const ::core::ffi::c_char,
    b"Too many links\0" as *const u8 as *const ::core::ffi::c_char,
    b"Inappropriate message buffer length\0" as *const u8 as *const ::core::ffi::c_char,
    b"Filename too long\0" as *const u8 as *const ::core::ffi::c_char,
    b"Too many open files in system\0" as *const u8 as *const ::core::ffi::c_char,
    b"No such device\0" as *const u8 as *const ::core::ffi::c_char,
    b"No such file or directory\0" as *const u8 as *const ::core::ffi::c_char,
    b"Exec format error\0" as *const u8 as *const ::core::ffi::c_char,
    b"No locks available\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not enough space\0" as *const u8 as *const ::core::ffi::c_char,
    b"No space left on device\0" as *const u8 as *const ::core::ffi::c_char,
    b"Function not implemented\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not a directory\0" as *const u8 as *const ::core::ffi::c_char,
    b"Directory not empty\0" as *const u8 as *const ::core::ffi::c_char,
    b"Not supported\0" as *const u8 as *const ::core::ffi::c_char,
    b"Inappropriate I/O control operation\0" as *const u8 as *const ::core::ffi::c_char,
    b"No such device or address\0" as *const u8 as *const ::core::ffi::c_char,
    b"Operation not permitted\0" as *const u8 as *const ::core::ffi::c_char,
    b"Broken pipe\0" as *const u8 as *const ::core::ffi::c_char,
    b"Result too large\0" as *const u8 as *const ::core::ffi::c_char,
    b"Read-only file system\0" as *const u8 as *const ::core::ffi::c_char,
    b"Invalid seek\0" as *const u8 as *const ::core::ffi::c_char,
    b"No such process\0" as *const u8 as *const ::core::ffi::c_char,
    b"Operation timed out\0" as *const u8 as *const ::core::ffi::c_char,
    b"Improper link\0" as *const u8 as *const ::core::ffi::c_char,
    b"Attempt to load network entity %s\0" as *const u8 as *const ::core::ffi::c_char,
    b"encoder error\0" as *const u8 as *const ::core::ffi::c_char,
    b"flush error\0" as *const u8 as *const ::core::ffi::c_char,
    b"write error\0" as *const u8 as *const ::core::ffi::c_char,
    b"no input\0" as *const u8 as *const ::core::ffi::c_char,
    b"buffer full\0" as *const u8 as *const ::core::ffi::c_char,
    b"loading error\0" as *const u8 as *const ::core::ffi::c_char,
    b"not a socket\0" as *const u8 as *const ::core::ffi::c_char,
    b"already connected\0" as *const u8 as *const ::core::ffi::c_char,
    b"connection refused\0" as *const u8 as *const ::core::ffi::c_char,
    b"unreachable network\0" as *const u8 as *const ::core::ffi::c_char,
    b"address in use\0" as *const u8 as *const ::core::ffi::c_char,
    b"already in use\0" as *const u8 as *const ::core::ffi::c_char,
    b"unknown address family\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn xmlIOErrMemory(mut extra: *const ::core::ffi::c_char) {
    __xmlSimpleError(
        XML_FROM_IO as ::core::ffi::c_int,
        XML_ERR_NO_MEMORY as ::core::ffi::c_int,
        ::core::ptr::null_mut::<_xmlNode>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        extra,
    );
}
#[no_mangle]
pub unsafe extern "C" fn __xmlIOErr(
    mut domain: ::core::ffi::c_int,
    mut code: ::core::ffi::c_int,
    mut extra: *const ::core::ffi::c_char,
) {
    let mut idx: ::core::ffi::c_uint = 0;
    if code == 0 as ::core::ffi::c_int {
        if *__errno_location() == 0 as ::core::ffi::c_int {
            code = 0 as ::core::ffi::c_int;
        } else if *__errno_location() == EACCES {
            code = XML_IO_EACCES as ::core::ffi::c_int;
        } else if *__errno_location() == EAGAIN {
            code = XML_IO_EAGAIN as ::core::ffi::c_int;
        } else if *__errno_location() == EBADF {
            code = XML_IO_EBADF as ::core::ffi::c_int;
        } else if *__errno_location() == EBADMSG {
            code = XML_IO_EBADMSG as ::core::ffi::c_int;
        } else if *__errno_location() == EBUSY {
            code = XML_IO_EBUSY as ::core::ffi::c_int;
        } else if *__errno_location() == ECANCELED {
            code = XML_IO_ECANCELED as ::core::ffi::c_int;
        } else if *__errno_location() == ECHILD {
            code = XML_IO_ECHILD as ::core::ffi::c_int;
        } else if *__errno_location() == EDEADLK {
            code = XML_IO_EDEADLK as ::core::ffi::c_int;
        } else if *__errno_location() == EDOM {
            code = XML_IO_EDOM as ::core::ffi::c_int;
        } else if *__errno_location() == EEXIST {
            code = XML_IO_EEXIST as ::core::ffi::c_int;
        } else if *__errno_location() == EFAULT {
            code = XML_IO_EFAULT as ::core::ffi::c_int;
        } else if *__errno_location() == EFBIG {
            code = XML_IO_EFBIG as ::core::ffi::c_int;
        } else if *__errno_location() == EINPROGRESS {
            code = XML_IO_EINPROGRESS as ::core::ffi::c_int;
        } else if *__errno_location() == EINTR {
            code = XML_IO_EINTR as ::core::ffi::c_int;
        } else if *__errno_location() == EINVAL {
            code = XML_IO_EINVAL as ::core::ffi::c_int;
        } else if *__errno_location() == EIO {
            code = XML_IO_EIO as ::core::ffi::c_int;
        } else if *__errno_location() == EISDIR {
            code = XML_IO_EISDIR as ::core::ffi::c_int;
        } else if *__errno_location() == EMFILE {
            code = XML_IO_EMFILE as ::core::ffi::c_int;
        } else if *__errno_location() == EMLINK {
            code = XML_IO_EMLINK as ::core::ffi::c_int;
        } else if *__errno_location() == EMSGSIZE {
            code = XML_IO_EMSGSIZE as ::core::ffi::c_int;
        } else if *__errno_location() == ENAMETOOLONG {
            code = XML_IO_ENAMETOOLONG as ::core::ffi::c_int;
        } else if *__errno_location() == ENFILE {
            code = XML_IO_ENFILE as ::core::ffi::c_int;
        } else if *__errno_location() == ENODEV {
            code = XML_IO_ENODEV as ::core::ffi::c_int;
        } else if *__errno_location() == ENOENT {
            code = XML_IO_ENOENT as ::core::ffi::c_int;
        } else if *__errno_location() == ENOEXEC {
            code = XML_IO_ENOEXEC as ::core::ffi::c_int;
        } else if *__errno_location() == ENOLCK {
            code = XML_IO_ENOLCK as ::core::ffi::c_int;
        } else if *__errno_location() == ENOMEM {
            code = XML_IO_ENOMEM as ::core::ffi::c_int;
        } else if *__errno_location() == ENOSPC {
            code = XML_IO_ENOSPC as ::core::ffi::c_int;
        } else if *__errno_location() == ENOSYS {
            code = XML_IO_ENOSYS as ::core::ffi::c_int;
        } else if *__errno_location() == ENOTDIR {
            code = XML_IO_ENOTDIR as ::core::ffi::c_int;
        } else if *__errno_location() == ENOTEMPTY {
            code = XML_IO_ENOTEMPTY as ::core::ffi::c_int;
        } else if *__errno_location() == ENOTSUP {
            code = XML_IO_ENOTSUP as ::core::ffi::c_int;
        } else if *__errno_location() == ENOTTY {
            code = XML_IO_ENOTTY as ::core::ffi::c_int;
        } else if *__errno_location() == ENXIO {
            code = XML_IO_ENXIO as ::core::ffi::c_int;
        } else if *__errno_location() == EPERM {
            code = XML_IO_EPERM as ::core::ffi::c_int;
        } else if *__errno_location() == EPIPE {
            code = XML_IO_EPIPE as ::core::ffi::c_int;
        } else if *__errno_location() == ERANGE {
            code = XML_IO_ERANGE as ::core::ffi::c_int;
        } else if *__errno_location() == EROFS {
            code = XML_IO_EROFS as ::core::ffi::c_int;
        } else if *__errno_location() == ESPIPE {
            code = XML_IO_ESPIPE as ::core::ffi::c_int;
        } else if *__errno_location() == ESRCH {
            code = XML_IO_ESRCH as ::core::ffi::c_int;
        } else if *__errno_location() == ETIMEDOUT {
            code = XML_IO_ETIMEDOUT as ::core::ffi::c_int;
        } else if *__errno_location() == EXDEV {
            code = XML_IO_EXDEV as ::core::ffi::c_int;
        } else if *__errno_location() == ENOTSOCK {
            code = XML_IO_ENOTSOCK as ::core::ffi::c_int;
        } else if *__errno_location() == EISCONN {
            code = XML_IO_EISCONN as ::core::ffi::c_int;
        } else if *__errno_location() == ECONNREFUSED {
            code = XML_IO_ECONNREFUSED as ::core::ffi::c_int;
        } else if *__errno_location() == ETIMEDOUT {
            code = XML_IO_ETIMEDOUT as ::core::ffi::c_int;
        } else if *__errno_location() == ENETUNREACH {
            code = XML_IO_ENETUNREACH as ::core::ffi::c_int;
        } else if *__errno_location() == EADDRINUSE {
            code = XML_IO_EADDRINUSE as ::core::ffi::c_int;
        } else if *__errno_location() == EINPROGRESS {
            code = XML_IO_EINPROGRESS as ::core::ffi::c_int;
        } else if *__errno_location() == EALREADY {
            code = XML_IO_EALREADY as ::core::ffi::c_int;
        } else if *__errno_location() == EAFNOSUPPORT {
            code = XML_IO_EAFNOSUPPORT as ::core::ffi::c_int;
        } else {
            code = XML_IO_UNKNOWN as ::core::ffi::c_int;
        }
    }
    idx = 0 as ::core::ffi::c_uint;
    if code >= XML_IO_UNKNOWN as ::core::ffi::c_int {
        idx = (code - XML_IO_UNKNOWN as ::core::ffi::c_int) as ::core::ffi::c_uint;
    }
    if idx as usize
        >= (::core::mem::size_of::<[*const ::core::ffi::c_char; 57]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        idx = 0 as ::core::ffi::c_uint;
    }
    __xmlSimpleError(
        domain,
        code,
        ::core::ptr::null_mut::<_xmlNode>(),
        IOerr[idx as usize],
        extra,
    );
}
unsafe extern "C" fn xmlIOErr(mut code: ::core::ffi::c_int, mut extra: *const ::core::ffi::c_char) {
    __xmlIOErr(XML_FROM_IO as ::core::ffi::c_int, code, extra);
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLoaderErr(
    mut ctx: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut ::core::ffi::c_void = NULL;
    let mut level: xmlErrorLevel = XML_ERR_ERROR;
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() && !(*ctxt).sax.is_null() {
        if (*ctxt).validate != 0 {
            channel = (*(*ctxt).sax).error as xmlGenericErrorFunc;
            level = XML_ERR_ERROR;
        } else {
            channel = (*(*ctxt).sax).warning as xmlGenericErrorFunc;
            level = XML_ERR_WARNING;
        }
        if (*(*ctxt).sax).initialized == XML_SAX2_MAGIC {
            schannel = (*(*ctxt).sax).serror;
        }
        data = (*ctxt).userData;
    }
    __xmlRaiseError(
        schannel,
        channel,
        data,
        ctxt as *mut ::core::ffi::c_void,
        NULL,
        XML_FROM_IO as ::core::ffi::c_int,
        XML_IO_LOAD_ERROR as ::core::ffi::c_int,
        level,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        filename,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        msg,
        filename,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlNormalizeWindowsPath(mut path: *const xmlChar) -> *mut xmlChar {
    return xmlCanonicPath(path);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupInputCallbacks() {
    let mut i: ::core::ffi::c_int = 0;
    if xmlInputCallbackInitialized == 0 {
        return;
    }
    i = xmlInputCallbackNr - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        xmlInputCallbackTable[i as usize].matchcallback = None;
        xmlInputCallbackTable[i as usize].opencallback = None;
        xmlInputCallbackTable[i as usize].readcallback = None;
        xmlInputCallbackTable[i as usize].closecallback = None;
        i -= 1;
    }
    xmlInputCallbackNr = 0 as ::core::ffi::c_int;
    xmlInputCallbackInitialized = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPopInputCallbacks() -> ::core::ffi::c_int {
    if xmlInputCallbackInitialized == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if xmlInputCallbackNr <= 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    xmlInputCallbackNr -= 1;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].matchcallback = None;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].opencallback = None;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].readcallback = None;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].closecallback = None;
    return xmlInputCallbackNr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupOutputCallbacks() {
    let mut i: ::core::ffi::c_int = 0;
    if xmlOutputCallbackInitialized == 0 {
        return;
    }
    i = xmlOutputCallbackNr - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        xmlOutputCallbackTable[i as usize].matchcallback = None;
        xmlOutputCallbackTable[i as usize].opencallback = None;
        xmlOutputCallbackTable[i as usize].writecallback = None;
        xmlOutputCallbackTable[i as usize].closecallback = None;
        i -= 1;
    }
    xmlOutputCallbackNr = 0 as ::core::ffi::c_int;
    xmlOutputCallbackInitialized = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPopOutputCallbacks() -> ::core::ffi::c_int {
    if xmlOutputCallbackInitialized == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if xmlOutputCallbackNr <= 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    xmlOutputCallbackNr -= 1;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].matchcallback = None;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].opencallback = None;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].writecallback = None;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].closecallback = None;
    return xmlOutputCallbackNr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCheckFilename(
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut stat_buffer: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if path.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if stat(path, &raw mut stat_buffer) == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if stat_buffer.st_mode as ::core::ffi::c_uint & __S_IFMT as ::core::ffi::c_uint
        == 0o40000 as ::core::ffi::c_uint
    {
        return 2 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlFdRead(
    mut context: *mut ::core::ffi::c_void,
    mut buffer: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = read(
        context as ptrdiff_t as ::core::ffi::c_int,
        buffer.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        len as size_t,
    ) as ::core::ffi::c_int;
    if ret < 0 as ::core::ffi::c_int {
        xmlIOErr(
            0 as ::core::ffi::c_int,
            b"read()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
unsafe extern "C" fn xmlFdWrite(
    mut context: *mut ::core::ffi::c_void,
    mut buffer: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if len > 0 as ::core::ffi::c_int {
        ret = write(
            context as ptrdiff_t as ::core::ffi::c_int,
            buffer.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            len as size_t,
        ) as ::core::ffi::c_int;
        if ret < 0 as ::core::ffi::c_int {
            xmlIOErr(
                0 as ::core::ffi::c_int,
                b"write()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    return ret;
}
unsafe extern "C" fn xmlFdClose(mut context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = close(context as ptrdiff_t as ::core::ffi::c_int);
    if ret < 0 as ::core::ffi::c_int {
        xmlIOErr(
            0 as ::core::ffi::c_int,
            b"close()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFileMatch(
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlFileOpen_real(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut path: *const ::core::ffi::c_char = filename;
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    if filename.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if strcmp(filename, b"-\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        fd = stdin;
        return fd as *mut ::core::ffi::c_void;
    }
    if xmlStrncasecmp(
        filename as *mut xmlChar,
        b"file://localhost/\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        17 as ::core::ffi::c_int,
    ) == 0
    {
        path = filename.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
    } else if xmlStrncasecmp(
        filename as *mut xmlChar,
        b"file:///\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        8 as ::core::ffi::c_int,
    ) == 0
    {
        path = filename.offset(7 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
    } else if xmlStrncasecmp(
        filename as *mut xmlChar,
        b"file:/\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        6 as ::core::ffi::c_int,
    ) == 0
    {
        path = filename.offset(5 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
    }
    if xmlCheckFilename(path) == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    fd = fopen(path, b"rb\0" as *const u8 as *const ::core::ffi::c_char);
    if fd.is_null() {
        xmlIOErr(0 as ::core::ffi::c_int, path);
    }
    return fd as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFileOpen(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut unescaped: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut retval: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    retval = xmlFileOpen_real(filename);
    if retval.is_null() {
        unescaped = xmlURIUnescapeString(
            filename,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
        if !unescaped.is_null() {
            retval = xmlFileOpen_real(unescaped);
            xmlFree.expect("non-null function pointer")(unescaped as *mut ::core::ffi::c_void);
        }
    }
    return retval;
}
unsafe extern "C" fn xmlFileOpenW(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    if strcmp(filename, b"-\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        fd = stdout;
        return fd as *mut ::core::ffi::c_void;
    }
    if xmlStrncasecmp(
        filename as *mut xmlChar,
        b"file://localhost/\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        17 as ::core::ffi::c_int,
    ) == 0
    {
        path = filename.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
    } else if xmlStrncasecmp(
        filename as *mut xmlChar,
        b"file:///\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        8 as ::core::ffi::c_int,
    ) == 0
    {
        path = filename.offset(7 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
    } else {
        path = filename;
    }
    if path.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    fd = fopen(path, b"wb\0" as *const u8 as *const ::core::ffi::c_char);
    if fd.is_null() {
        xmlIOErr(0 as ::core::ffi::c_int, path);
    }
    return fd as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFileRead(
    mut context: *mut ::core::ffi::c_void,
    mut buffer: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if context.is_null() || buffer.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ret = fread(
        buffer.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        1 as size_t,
        len as size_t,
        context as *mut FILE,
    ) as ::core::ffi::c_int;
    if ret < 0 as ::core::ffi::c_int {
        xmlIOErr(
            0 as ::core::ffi::c_int,
            b"fread()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
unsafe extern "C" fn xmlFileWrite(
    mut context: *mut ::core::ffi::c_void,
    mut buffer: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut items: ::core::ffi::c_int = 0;
    if context.is_null() || buffer.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    items = fwrite(
        buffer.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        len as size_t,
        1 as size_t,
        context as *mut FILE,
    ) as ::core::ffi::c_int;
    if items == 0 as ::core::ffi::c_int && ferror(context as *mut FILE) != 0 {
        xmlIOErr(
            0 as ::core::ffi::c_int,
            b"fwrite()\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return items * len;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFileClose(mut context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut fil: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut ret: ::core::ffi::c_int = 0;
    if context.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    fil = context as *mut FILE;
    if fil == stdout || fil == stderr {
        ret = fflush(fil);
        if ret < 0 as ::core::ffi::c_int {
            xmlIOErr(
                0 as ::core::ffi::c_int,
                b"fflush()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    if fil == stdin {
        return 0 as ::core::ffi::c_int;
    }
    ret = if fclose(context as *mut FILE) == EOF {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
    if ret < 0 as ::core::ffi::c_int {
        xmlIOErr(
            0 as ::core::ffi::c_int,
            b"fclose()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
unsafe extern "C" fn xmlFileFlush(mut context: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if context.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ret = if fflush(context as *mut FILE) == EOF {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
    if ret < 0 as ::core::ffi::c_int {
        xmlIOErr(
            0 as ::core::ffi::c_int,
            b"fflush()\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
unsafe extern "C" fn xmlBufferWrite(
    mut context: *mut ::core::ffi::c_void,
    mut buffer: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = xmlBufferAdd(context as xmlBufferPtr, buffer as *const xmlChar, len);
    if ret != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return len;
}
unsafe extern "C" fn xmlFreeHTTPWriteCtxt(mut ctxt: xmlIOHTTPWriteCtxtPtr) {
    if !(*ctxt).uri.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).uri as *mut ::core::ffi::c_void);
    }
    if !(*ctxt).doc_buff.is_null() {
        xmlOutputBufferClose((*ctxt).doc_buff as xmlOutputBufferPtr);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPMatch(
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if xmlStrncasecmp(
        filename as *mut xmlChar,
        b"http://\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        7 as ::core::ffi::c_int,
    ) == 0
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPOpen(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    return xmlNanoHTTPOpen(
        filename,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPOpenW(
    mut post_uri: *const ::core::ffi::c_char,
    mut compression: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut ctxt: xmlIOHTTPWriteCtxtPtr = ::core::ptr::null_mut::<xmlIOHTTPWriteCtxt_>();
    if post_uri.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    ctxt = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlIOHTTPWriteCtxt>() as size_t,
    ) as xmlIOHTTPWriteCtxtPtr;
    if ctxt.is_null() {
        xmlIOErrMemory(
            b"creating HTTP output context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    memset(
        ctxt as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlIOHTTPWriteCtxt>() as size_t,
    );
    (*ctxt).uri = xmlStrdup(post_uri as *const xmlChar) as *mut ::core::ffi::c_char;
    if (*ctxt).uri.is_null() {
        xmlIOErrMemory(b"copying URI\0" as *const u8 as *const ::core::ffi::c_char);
        xmlFreeHTTPWriteCtxt(ctxt);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*ctxt).doc_buff =
        xmlAllocOutputBufferInternal(::core::ptr::null_mut::<xmlCharEncodingHandler>())
            as *mut ::core::ffi::c_void;
    if (*ctxt).doc_buff.is_null() {
        xmlFreeHTTPWriteCtxt(ctxt);
        ctxt = ::core::ptr::null_mut::<xmlIOHTTPWriteCtxt_>();
    }
    return ctxt as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn xmlIOHTTPDfltOpenW(
    mut post_uri: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    return xmlIOHTTPOpenW(post_uri, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPRead(
    mut context: *mut ::core::ffi::c_void,
    mut buffer: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if buffer.is_null() || len < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return xmlNanoHTTPRead(
        context,
        buffer.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        len,
    );
}
unsafe extern "C" fn xmlIOHTTPWrite(
    mut context: *mut ::core::ffi::c_void,
    mut buffer: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlIOHTTPWriteCtxtPtr = context as xmlIOHTTPWriteCtxtPtr;
    if ctxt.is_null() || (*ctxt).doc_buff.is_null() || buffer.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if len > 0 as ::core::ffi::c_int {
        len = xmlOutputBufferWrite((*ctxt).doc_buff as xmlOutputBufferPtr, len, buffer);
        if len < 0 as ::core::ffi::c_int {
            let mut msg: [xmlChar; 500] = [0; 500];
            xmlStrPrintf(
                &raw mut msg as *mut xmlChar,
                500 as ::core::ffi::c_int,
                b"xmlIOHTTPWrite:  %s\n%s '%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"Error appending to internal buffer.\0" as *const u8 as *const ::core::ffi::c_char,
                b"Error sending document to URI\0" as *const u8 as *const ::core::ffi::c_char,
                (*ctxt).uri,
            );
            xmlIOErr(
                XML_IO_WRITE as ::core::ffi::c_int,
                &raw mut msg as *mut xmlChar as *const ::core::ffi::c_char,
            );
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn xmlIOHTTPClose(
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    xmlNanoHTTPClose(context);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlIOHTTPCloseWrite(
    mut context: *mut ::core::ffi::c_void,
    mut http_mthd: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut close_rc: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut http_rtn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut content_lgth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctxt: xmlIOHTTPWriteCtxtPtr = context as xmlIOHTTPWriteCtxtPtr;
    let mut http_content: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut content_encoding: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut content_type: *mut ::core::ffi::c_char =
        b"text/xml\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut http_ctxt: *mut ::core::ffi::c_void = NULL;
    if ctxt.is_null() || http_mthd.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut dctxt: xmlOutputBufferPtr = (*ctxt).doc_buff as xmlOutputBufferPtr;
    http_content = xmlBufContent((*dctxt).buffer as *const xmlBuf) as *mut ::core::ffi::c_char;
    content_lgth = xmlBufUse((*dctxt).buffer) as ::core::ffi::c_int;
    if http_content.is_null() {
        let mut msg: [xmlChar; 500] = [0; 500];
        xmlStrPrintf(
            &raw mut msg as *mut xmlChar,
            500 as ::core::ffi::c_int,
            b"xmlIOHTTPCloseWrite:  %s '%s' %s '%s'.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"Error retrieving content.\nUnable to\0" as *const u8 as *const ::core::ffi::c_char,
            http_mthd,
            b"data to URI\0" as *const u8 as *const ::core::ffi::c_char,
            (*ctxt).uri,
        );
        xmlIOErr(
            XML_IO_WRITE as ::core::ffi::c_int,
            &raw mut msg as *mut xmlChar as *const ::core::ffi::c_char,
        );
    } else {
        http_ctxt = xmlNanoHTTPMethod(
            (*ctxt).uri,
            http_mthd,
            http_content,
            &raw mut content_type,
            content_encoding,
            content_lgth,
        );
        if !http_ctxt.is_null() {
            http_rtn = xmlNanoHTTPReturnCode(http_ctxt);
            if http_rtn >= 200 as ::core::ffi::c_int && http_rtn < 300 as ::core::ffi::c_int {
                close_rc = 0 as ::core::ffi::c_int;
            } else {
                let mut msg_0: [xmlChar; 500] = [0; 500];
                xmlStrPrintf(
                    &raw mut msg_0 as *mut xmlChar,
                    500 as ::core::ffi::c_int,
                    b"xmlIOHTTPCloseWrite: HTTP '%s' of %d %s\n'%s' %s %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    http_mthd,
                    content_lgth,
                    b"bytes to URI\0" as *const u8 as *const ::core::ffi::c_char,
                    (*ctxt).uri,
                    b"failed.  HTTP return code:\0" as *const u8 as *const ::core::ffi::c_char,
                    http_rtn,
                );
                xmlIOErr(
                    XML_IO_WRITE as ::core::ffi::c_int,
                    &raw mut msg_0 as *mut xmlChar as *const ::core::ffi::c_char,
                );
            }
            xmlNanoHTTPClose(http_ctxt);
            xmlFree.expect("non-null function pointer")(content_type as *mut ::core::ffi::c_void);
        }
    }
    xmlFreeHTTPWriteCtxt(ctxt);
    return close_rc;
}
unsafe extern "C" fn xmlIOHTTPClosePut(mut ctxt: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    return xmlIOHTTPCloseWrite(ctxt, b"PUT\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn xmlIOHTTPClosePost(mut ctxt: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    return xmlIOHTTPCloseWrite(ctxt, b"POST\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterInputCallbacks(
    mut matchFunc: xmlInputMatchCallback,
    mut openFunc: xmlInputOpenCallback,
    mut readFunc: xmlInputReadCallback,
    mut closeFunc: xmlInputCloseCallback,
) -> ::core::ffi::c_int {
    if xmlInputCallbackNr >= MAX_INPUT_CALLBACK {
        return -(1 as ::core::ffi::c_int);
    }
    xmlInputCallbackTable[xmlInputCallbackNr as usize].matchcallback = matchFunc;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].opencallback = openFunc;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].readcallback = readFunc;
    xmlInputCallbackTable[xmlInputCallbackNr as usize].closecallback = closeFunc;
    xmlInputCallbackInitialized = 1 as ::core::ffi::c_int;
    let fresh0 = xmlInputCallbackNr;
    xmlInputCallbackNr = xmlInputCallbackNr + 1;
    return fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterOutputCallbacks(
    mut matchFunc: xmlOutputMatchCallback,
    mut openFunc: xmlOutputOpenCallback,
    mut writeFunc: xmlOutputWriteCallback,
    mut closeFunc: xmlOutputCloseCallback,
) -> ::core::ffi::c_int {
    if xmlOutputCallbackNr >= MAX_OUTPUT_CALLBACK {
        return -(1 as ::core::ffi::c_int);
    }
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].matchcallback = matchFunc;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].opencallback = openFunc;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].writecallback = writeFunc;
    xmlOutputCallbackTable[xmlOutputCallbackNr as usize].closecallback = closeFunc;
    xmlOutputCallbackInitialized = 1 as ::core::ffi::c_int;
    let fresh1 = xmlOutputCallbackNr;
    xmlOutputCallbackNr = xmlOutputCallbackNr + 1;
    return fresh1;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterDefaultInputCallbacks() {
    if xmlInputCallbackInitialized != 0 {
        return;
    }
    xmlRegisterInputCallbacks(
        Some(
            xmlFileMatch as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        Some(
            xmlFileOpen
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
        ),
        Some(
            xmlFileRead
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(xmlFileClose as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int),
    );
    xmlRegisterInputCallbacks(
        Some(
            xmlIOHTTPMatch
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        Some(
            xmlIOHTTPOpen
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
        ),
        Some(
            xmlIOHTTPRead
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            xmlIOHTTPClose as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    );
    xmlInputCallbackInitialized = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterDefaultOutputCallbacks() {
    if xmlOutputCallbackInitialized != 0 {
        return;
    }
    xmlRegisterOutputCallbacks(
        Some(
            xmlFileMatch as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        Some(
            xmlFileOpenW
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
        ),
        Some(
            xmlFileWrite
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(xmlFileClose as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int),
    );
    xmlRegisterOutputCallbacks(
        Some(
            xmlIOHTTPMatch
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        Some(
            xmlIOHTTPDfltOpenW
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
        ),
        Some(
            xmlIOHTTPWrite
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            xmlIOHTTPClosePut
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    );
    xmlOutputCallbackInitialized = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterHTTPPostCallbacks() {
    if xmlOutputCallbackInitialized == 0 as ::core::ffi::c_int {
        xmlRegisterDefaultOutputCallbacks();
    }
    xmlRegisterOutputCallbacks(
        Some(
            xmlIOHTTPMatch
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        Some(
            xmlIOHTTPDfltOpenW
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
        ),
        Some(
            xmlIOHTTPWrite
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        Some(
            xmlIOHTTPClosePost
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlAllocParserInputBuffer(
    mut enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlParserInputBuffer>() as size_t,
    ) as xmlParserInputBufferPtr;
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlParserInputBuffer>() as size_t,
    );
    (*ret).buffer =
        xmlBufCreateSize((2 as ::core::ffi::c_int * *__xmlDefaultBufferSize()) as size_t);
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    (*ret).encoder = xmlGetCharEncodingHandler(enc);
    if !(*ret).encoder.is_null() {
        (*ret).raw =
            xmlBufCreateSize((2 as ::core::ffi::c_int * *__xmlDefaultBufferSize()) as size_t);
    } else {
        (*ret).raw = ::core::ptr::null_mut::<xmlBuf>();
    }
    (*ret).readcallback = None;
    (*ret).closecallback = None;
    (*ret).context = NULL;
    (*ret).compressed = -(1 as ::core::ffi::c_int);
    (*ret).rawconsumed = 0 as ::core::ffi::c_ulong;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAllocOutputBuffer(
    mut encoder: xmlCharEncodingHandlerPtr,
) -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlOutputBuffer>() as size_t
    ) as xmlOutputBufferPtr;
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlOutputBuffer>() as size_t,
    );
    (*ret).buffer = xmlBufCreate();
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    (*ret).encoder = encoder;
    if !encoder.is_null() {
        (*ret).conv = xmlBufCreateSize(4000 as size_t);
        if (*ret).conv.is_null() {
            xmlBufFree((*ret).buffer);
            xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xmlOutputBuffer>();
        }
        xmlCharEncOutput(ret, 1 as ::core::ffi::c_int);
    } else {
        (*ret).conv = ::core::ptr::null_mut::<xmlBuf>();
    }
    (*ret).writecallback = None;
    (*ret).closecallback = None;
    (*ret).context = NULL;
    (*ret).written = 0 as ::core::ffi::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAllocOutputBufferInternal(
    mut encoder: xmlCharEncodingHandlerPtr,
) -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlOutputBuffer>() as size_t
    ) as xmlOutputBufferPtr;
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlOutputBuffer>() as size_t,
    );
    (*ret).buffer = xmlBufCreate();
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_IO);
    (*ret).encoder = encoder;
    if !encoder.is_null() {
        (*ret).conv = xmlBufCreateSize(4000 as size_t);
        if (*ret).conv.is_null() {
            xmlBufFree((*ret).buffer);
            xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xmlOutputBuffer>();
        }
        xmlCharEncOutput(ret, 1 as ::core::ffi::c_int);
    } else {
        (*ret).conv = ::core::ptr::null_mut::<xmlBuf>();
    }
    (*ret).writecallback = None;
    (*ret).closecallback = None;
    (*ret).context = NULL;
    (*ret).written = 0 as ::core::ffi::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeParserInputBuffer(mut in_0: xmlParserInputBufferPtr) {
    if in_0.is_null() {
        return;
    }
    if !(*in_0).raw.is_null() {
        xmlBufFree((*in_0).raw);
        (*in_0).raw = ::core::ptr::null_mut::<xmlBuf>();
    }
    if !(*in_0).encoder.is_null() {
        xmlCharEncCloseFunc((*in_0).encoder as *mut xmlCharEncodingHandler);
    }
    if (*in_0).closecallback.is_some() {
        (*in_0).closecallback.expect("non-null function pointer")((*in_0).context);
    }
    if !(*in_0).buffer.is_null() {
        xmlBufFree((*in_0).buffer);
        (*in_0).buffer = ::core::ptr::null_mut::<xmlBuf>();
    }
    xmlFree.expect("non-null function pointer")(in_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferClose(mut out: xmlOutputBufferPtr) -> ::core::ffi::c_int {
    let mut written: ::core::ffi::c_int = 0;
    let mut err_rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if out.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*out).writecallback.is_some() {
        xmlOutputBufferFlush(out);
    }
    if (*out).closecallback.is_some() {
        err_rc = (*out).closecallback.expect("non-null function pointer")((*out).context);
    }
    written = (*out).written;
    if !(*out).conv.is_null() {
        xmlBufFree((*out).conv);
        (*out).conv = ::core::ptr::null_mut::<xmlBuf>();
    }
    if !(*out).encoder.is_null() {
        xmlCharEncCloseFunc((*out).encoder as *mut xmlCharEncodingHandler);
    }
    if !(*out).buffer.is_null() {
        xmlBufFree((*out).buffer);
        (*out).buffer = ::core::ptr::null_mut::<xmlBuf>();
    }
    if (*out).error != 0 {
        err_rc = -(1 as ::core::ffi::c_int);
    }
    xmlFree.expect("non-null function pointer")(out as *mut ::core::ffi::c_void);
    return if err_rc == 0 as ::core::ffi::c_int {
        written
    } else {
        err_rc
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserInputBufferCreateFilename(
    mut URI: *const ::core::ffi::c_char,
    mut enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut context: *mut ::core::ffi::c_void = NULL;
    if xmlInputCallbackInitialized == 0 as ::core::ffi::c_int {
        xmlRegisterDefaultInputCallbacks();
    }
    if URI.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    if context.is_null() {
        i = xmlInputCallbackNr - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            if xmlInputCallbackTable[i as usize].matchcallback.is_some()
                && xmlInputCallbackTable[i as usize]
                    .matchcallback
                    .expect("non-null function pointer")(URI)
                    != 0 as ::core::ffi::c_int
            {
                context = xmlInputCallbackTable[i as usize]
                    .opencallback
                    .expect("non-null function pointer")(URI);
                if !context.is_null() {
                    break;
                }
            }
            i -= 1;
        }
    }
    if context.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = context;
        (*ret).readcallback = xmlInputCallbackTable[i as usize].readcallback;
        (*ret).closecallback = xmlInputCallbackTable[i as usize].closecallback;
    } else {
        xmlInputCallbackTable[i as usize]
            .closecallback
            .expect("non-null function pointer")(context);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFilename(
    mut URI: *const ::core::ffi::c_char,
    mut enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    if (*__xmlParserInputBufferCreateFilenameValue()).is_some() {
        return (*__xmlParserInputBufferCreateFilenameValue()).expect("non-null function pointer")(
            URI, enc,
        );
    }
    return __xmlParserInputBufferCreateFilename(URI, enc);
}
#[no_mangle]
pub unsafe extern "C" fn __xmlOutputBufferCreateFilename(
    mut URI: *const ::core::ffi::c_char,
    mut encoder: xmlCharEncodingHandlerPtr,
    mut compression: ::core::ffi::c_int,
) -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    let mut puri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut context: *mut ::core::ffi::c_void = NULL;
    let mut unescaped: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if xmlOutputCallbackInitialized == 0 as ::core::ffi::c_int {
        xmlRegisterDefaultOutputCallbacks();
    }
    if URI.is_null() {
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    puri = xmlParseURI(URI);
    if !puri.is_null() {
        if (*puri).scheme.is_null()
            || xmlStrEqual(
                (*puri).scheme as *mut xmlChar,
                b"file\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
        {
            unescaped = xmlURIUnescapeString(
                URI,
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        xmlFreeURI(puri);
    }
    if !unescaped.is_null() {
        i = xmlOutputCallbackNr - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            if xmlOutputCallbackTable[i as usize].matchcallback.is_some()
                && xmlOutputCallbackTable[i as usize]
                    .matchcallback
                    .expect("non-null function pointer")(unescaped)
                    != 0 as ::core::ffi::c_int
            {
                context = xmlOutputCallbackTable[i as usize]
                    .opencallback
                    .expect("non-null function pointer")(unescaped);
                if !context.is_null() {
                    break;
                }
            }
            i -= 1;
        }
        xmlFree.expect("non-null function pointer")(unescaped as *mut ::core::ffi::c_void);
    }
    if context.is_null() {
        i = xmlOutputCallbackNr - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            if xmlOutputCallbackTable[i as usize].matchcallback.is_some()
                && xmlOutputCallbackTable[i as usize]
                    .matchcallback
                    .expect("non-null function pointer")(URI)
                    != 0 as ::core::ffi::c_int
            {
                context = xmlOutputCallbackTable[i as usize]
                    .opencallback
                    .expect("non-null function pointer")(URI);
                if !context.is_null() {
                    break;
                }
            }
            i -= 1;
        }
    }
    if context.is_null() {
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    ret = xmlAllocOutputBufferInternal(encoder);
    if !ret.is_null() {
        (*ret).context = context;
        (*ret).writecallback = xmlOutputCallbackTable[i as usize].writecallback;
        (*ret).closecallback = xmlOutputCallbackTable[i as usize].closecallback;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFilename(
    mut URI: *const ::core::ffi::c_char,
    mut encoder: xmlCharEncodingHandlerPtr,
    mut compression: ::core::ffi::c_int,
) -> xmlOutputBufferPtr {
    if (*__xmlOutputBufferCreateFilenameValue()).is_some() {
        return (*__xmlOutputBufferCreateFilenameValue()).expect("non-null function pointer")(
            URI,
            encoder,
            compression,
        );
    }
    return __xmlOutputBufferCreateFilename(URI, encoder, compression);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFile(
    mut file: *mut FILE,
    mut enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if xmlInputCallbackInitialized == 0 as ::core::ffi::c_int {
        xmlRegisterDefaultInputCallbacks();
    }
    if file.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = file as *mut ::core::ffi::c_void;
        (*ret).readcallback = Some(
            xmlFileRead
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ) as xmlInputReadCallback;
        (*ret).closecallback = Some(
            xmlFileFlush as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ) as xmlInputCloseCallback;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFile(
    mut file: *mut FILE,
    mut encoder: xmlCharEncodingHandlerPtr,
) -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    if xmlOutputCallbackInitialized == 0 as ::core::ffi::c_int {
        xmlRegisterDefaultOutputCallbacks();
    }
    if file.is_null() {
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    ret = xmlAllocOutputBufferInternal(encoder);
    if !ret.is_null() {
        (*ret).context = file as *mut ::core::ffi::c_void;
        (*ret).writecallback = Some(
            xmlFileWrite
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ) as xmlOutputWriteCallback;
        (*ret).closecallback = Some(
            xmlFileFlush as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ) as xmlOutputCloseCallback;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateBuffer(
    mut buffer: xmlBufferPtr,
    mut encoder: xmlCharEncodingHandlerPtr,
) -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    if buffer.is_null() {
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    ret = xmlOutputBufferCreateIO(
        Some(
            xmlBufferWrite
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        None,
        buffer as *mut ::core::ffi::c_void,
        encoder,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferGetContent(mut out: xmlOutputBufferPtr) -> *const xmlChar {
    if out.is_null() || (*out).buffer.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return xmlBufContent((*out).buffer as *const xmlBuf);
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferGetSize(mut out: xmlOutputBufferPtr) -> size_t {
    if out.is_null() || (*out).buffer.is_null() {
        return 0 as size_t;
    }
    return xmlBufUse((*out).buffer);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFd(
    mut fd: ::core::ffi::c_int,
    mut enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = fd as ptrdiff_t as *mut ::core::ffi::c_void;
        (*ret).readcallback = Some(
            xmlFdRead
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ) as xmlInputReadCallback;
        (*ret).closecallback = Some(
            xmlFdClose as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ) as xmlInputCloseCallback;
    }
    return ret;
}
unsafe extern "C" fn xmlMemRead(
    mut vctxt: *mut ::core::ffi::c_void,
    mut buf: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxt: *mut xmlMemIOCtxt = vctxt as *mut xmlMemIOCtxt;
    if size as size_t > (*ctxt).size {
        size = (*ctxt).size as ::core::ffi::c_int;
    }
    memcpy(
        buf as *mut ::core::ffi::c_void,
        (*ctxt).cur as *const ::core::ffi::c_void,
        size as size_t,
    );
    (*ctxt).cur = (*ctxt).cur.offset(size as isize);
    (*ctxt).size = ((*ctxt).size as ::core::ffi::c_ulong).wrapping_sub(size as ::core::ffi::c_ulong)
        as size_t as size_t;
    return size;
}
unsafe extern "C" fn xmlMemClose(mut vctxt: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctxt: *mut xmlMemIOCtxt = vctxt as *mut xmlMemIOCtxt;
    if !(*ctxt).mem.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).mem as *mut ::core::ffi::c_void);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateMem(
    mut mem: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut ctxt: *mut xmlMemIOCtxt = ::core::ptr::null_mut::<xmlMemIOCtxt>();
    let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if size < 0 as ::core::ffi::c_int || mem.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    copy = xmlStrndup(mem as *const xmlChar, size) as *mut ::core::ffi::c_char;
    if copy.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    buf = xmlParserInputBufferCreateStatic(copy, size, enc);
    if buf.is_null() {
        xmlFree.expect("non-null function pointer")(copy as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ctxt = (*buf).context as *mut xmlMemIOCtxt;
    (*ctxt).mem = copy;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateStatic(
    mut mem: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut ctxt: *mut xmlMemIOCtxt = ::core::ptr::null_mut::<xmlMemIOCtxt>();
    if size < 0 as ::core::ffi::c_int || mem.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ret = xmlAllocParserInputBuffer(enc);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ctxt = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlMemIOCtxt>() as size_t
    ) as *mut xmlMemIOCtxt;
    if ctxt.is_null() {
        xmlFreeParserInputBuffer(ret);
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    (*ctxt).mem = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*ctxt).cur = mem;
    (*ctxt).size = size as size_t;
    (*ret).context = ctxt as *mut ::core::ffi::c_void;
    (*ret).readcallback = Some(
        xmlMemRead
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as xmlInputReadCallback;
    (*ret).closecallback =
        Some(xmlMemClose as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int)
            as xmlInputCloseCallback;
    return ret;
}
unsafe extern "C" fn xmlStringRead(
    mut vctxt: *mut ::core::ffi::c_void,
    mut buf: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxt: *mut xmlStringIOCtxt = vctxt as *mut xmlStringIOCtxt;
    let mut zero: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut len: size_t = 0;
    zero = memchr(
        (*ctxt).str_0 as *const ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        size as size_t,
    ) as *const xmlChar;
    len = (if !zero.is_null() {
        zero.offset_from((*ctxt).str_0) as ::core::ffi::c_long
    } else {
        size as ::core::ffi::c_long
    }) as size_t;
    memcpy(
        buf as *mut ::core::ffi::c_void,
        (*ctxt).str_0 as *const ::core::ffi::c_void,
        len,
    );
    (*ctxt).str_0 = (*ctxt).str_0.offset(len as isize);
    return len as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlStringClose(mut vctxt: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    xmlFree.expect("non-null function pointer")(vctxt);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateString(
    mut str: *const xmlChar,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut ctxt: *mut xmlStringIOCtxt = ::core::ptr::null_mut::<xmlStringIOCtxt>();
    if str.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ret = xmlAllocParserInputBuffer(XML_CHAR_ENCODING_NONE);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ctxt = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlStringIOCtxt>() as size_t
    ) as *mut xmlStringIOCtxt;
    if ctxt.is_null() {
        xmlFreeParserInputBuffer(ret);
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    (*ctxt).str_0 = str;
    (*ret).context = ctxt as *mut ::core::ffi::c_void;
    (*ret).readcallback = Some(
        xmlStringRead
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    ) as xmlInputReadCallback;
    (*ret).closecallback = Some(
        xmlStringClose as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    ) as xmlInputCloseCallback;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFd(
    mut fd: ::core::ffi::c_int,
    mut encoder: xmlCharEncodingHandlerPtr,
) -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    ret = xmlAllocOutputBufferInternal(encoder);
    if !ret.is_null() {
        (*ret).context = fd as ptrdiff_t as *mut ::core::ffi::c_void;
        (*ret).writecallback = Some(
            xmlFdWrite
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ) as xmlOutputWriteCallback;
        (*ret).closecallback = None;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateIO(
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut ::core::ffi::c_void,
    mut enc: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if ioread.is_none() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    ret = xmlAllocParserInputBuffer(enc);
    if !ret.is_null() {
        (*ret).context = ioctx;
        (*ret).readcallback = ioread;
        (*ret).closecallback = ioclose;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateIO(
    mut iowrite: xmlOutputWriteCallback,
    mut ioclose: xmlOutputCloseCallback,
    mut ioctx: *mut ::core::ffi::c_void,
    mut encoder: xmlCharEncodingHandlerPtr,
) -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    if iowrite.is_none() {
        return ::core::ptr::null_mut::<xmlOutputBuffer>();
    }
    ret = xmlAllocOutputBufferInternal(encoder);
    if !ret.is_null() {
        (*ret).context = ioctx;
        (*ret).writecallback = iowrite;
        (*ret).closecallback = ioclose;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferCreateFilenameDefault(
    mut func: xmlParserInputBufferCreateFilenameFunc,
) -> xmlParserInputBufferCreateFilenameFunc {
    let mut old: xmlParserInputBufferCreateFilenameFunc =
        *__xmlParserInputBufferCreateFilenameValue();
    if old.is_none() {
        old = Some(
            __xmlParserInputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    xmlCharEncoding,
                ) -> xmlParserInputBufferPtr,
        ) as xmlParserInputBufferCreateFilenameFunc;
    }
    let ref mut fresh21 = *__xmlParserInputBufferCreateFilenameValue();
    *fresh21 = func;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferCreateFilenameDefault(
    mut func: xmlOutputBufferCreateFilenameFunc,
) -> xmlOutputBufferCreateFilenameFunc {
    let mut old: xmlOutputBufferCreateFilenameFunc = *__xmlOutputBufferCreateFilenameValue();
    if old.is_none() {
        old = Some(
            __xmlOutputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    xmlCharEncodingHandlerPtr,
                    ::core::ffi::c_int,
                ) -> xmlOutputBufferPtr,
        ) as xmlOutputBufferCreateFilenameFunc;
    }
    let ref mut fresh22 = *__xmlOutputBufferCreateFilenameValue();
    *fresh22 = func;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferPush(
    mut in_0: xmlParserInputBufferPtr,
    mut len: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut nbchars: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    if len < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if in_0.is_null() || (*in_0).error != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*in_0).encoder.is_null() {
        if (*in_0).raw.is_null() {
            (*in_0).raw = xmlBufCreate();
            if (*in_0).raw.is_null() {
                (*in_0).error = XML_ERR_NO_MEMORY as ::core::ffi::c_int;
                return -(1 as ::core::ffi::c_int);
            }
        }
        ret = xmlBufAdd((*in_0).raw, buf as *const xmlChar, len);
        if ret != 0 as ::core::ffi::c_int {
            (*in_0).error = XML_ERR_NO_MEMORY as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        nbchars = xmlCharEncInput(in_0);
        if nbchars < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        nbchars = len;
        ret = xmlBufAdd((*in_0).buffer, buf as *mut xmlChar, nbchars);
        if ret != 0 as ::core::ffi::c_int {
            (*in_0).error = XML_ERR_NO_MEMORY as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
    }
    return nbchars;
}
unsafe extern "C" fn endOfInput(
    mut context: *mut ::core::ffi::c_void,
    mut buffer: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferGrow(
    mut in_0: xmlParserInputBufferPtr,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if in_0.is_null() || (*in_0).error != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if len <= MINLEN && len != 4 as ::core::ffi::c_int {
        len = MINLEN;
    }
    if (*in_0).encoder.is_null() {
        if (*in_0).readcallback.is_none() {
            return 0 as ::core::ffi::c_int;
        }
        buf = (*in_0).buffer;
    } else {
        if (*in_0).raw.is_null() {
            (*in_0).raw = xmlBufCreate();
        }
        buf = (*in_0).raw;
    }
    if (*in_0).readcallback.is_some() {
        if xmlBufGrow(buf, len + 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*in_0).error = XML_ERR_NO_MEMORY as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        res = (*in_0).readcallback.expect("non-null function pointer")(
            (*in_0).context,
            xmlBufEnd(buf) as *mut ::core::ffi::c_char,
            len,
        );
        if res <= 0 as ::core::ffi::c_int {
            (*in_0).readcallback = Some(
                endOfInput
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ) as xmlInputReadCallback;
        }
        if res < 0 as ::core::ffi::c_int {
            (*in_0).error = XML_IO_UNKNOWN as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        if xmlBufAddLen(buf, res as size_t) < 0 as ::core::ffi::c_int {
            (*in_0).error = XML_ERR_NO_MEMORY as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
    }
    (*in_0).compressed == -(1 as ::core::ffi::c_int);
    if !(*in_0).encoder.is_null() {
        res = xmlCharEncInput(in_0);
        if res < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserInputBufferRead(
    mut in_0: xmlParserInputBufferPtr,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return xmlParserInputBufferGrow(in_0, len);
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferWrite(
    mut out: xmlOutputBufferPtr,
    mut len: ::core::ffi::c_int,
    mut buf: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut nbchars: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    let mut written: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut chunk: ::core::ffi::c_int = 0;
    if out.is_null() || (*out).error != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if len < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*out).error != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        chunk = len;
        if chunk > 4 as ::core::ffi::c_int * MINLEN {
            chunk = 4 as ::core::ffi::c_int * MINLEN;
        }
        if !(*out).encoder.is_null() {
            if (*out).conv.is_null() {
                (*out).conv = xmlBufCreate();
            }
            ret = xmlBufAdd((*out).buffer, buf as *const xmlChar, chunk);
            if ret != 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            if xmlBufUse((*out).buffer) < MINLEN as size_t && chunk == len {
                break;
            }
            ret = xmlCharEncOutput(out, 0 as ::core::ffi::c_int);
            if ret < 0 as ::core::ffi::c_int && ret != -(3 as ::core::ffi::c_int) {
                xmlIOErr(
                    XML_IO_ENCODER as ::core::ffi::c_int,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                (*out).error = XML_IO_ENCODER as ::core::ffi::c_int;
                return -(1 as ::core::ffi::c_int);
            }
            if (*out).writecallback.is_some() {
                nbchars = xmlBufUse((*out).conv) as ::core::ffi::c_int;
            } else {
                nbchars = if ret >= 0 as ::core::ffi::c_int {
                    ret
                } else {
                    0 as ::core::ffi::c_int
                };
            }
        } else {
            ret = xmlBufAdd((*out).buffer, buf as *const xmlChar, chunk);
            if ret != 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            if (*out).writecallback.is_some() {
                nbchars = xmlBufUse((*out).buffer) as ::core::ffi::c_int;
            } else {
                nbchars = chunk;
            }
        }
        buf = buf.offset(chunk as isize);
        len -= chunk;
        if (*out).writecallback.is_some() {
            if nbchars < MINLEN && len <= 0 as ::core::ffi::c_int {
                break;
            }
            if !(*out).encoder.is_null() {
                ret = (*out).writecallback.expect("non-null function pointer")(
                    (*out).context,
                    xmlBufContent((*out).conv as *const xmlBuf) as *const ::core::ffi::c_char,
                    nbchars,
                );
                if ret >= 0 as ::core::ffi::c_int {
                    xmlBufShrink((*out).conv, ret as size_t);
                }
            } else {
                ret = (*out).writecallback.expect("non-null function pointer")(
                    (*out).context,
                    xmlBufContent((*out).buffer as *const xmlBuf) as *const ::core::ffi::c_char,
                    nbchars,
                );
                if ret >= 0 as ::core::ffi::c_int {
                    xmlBufShrink((*out).buffer, ret as size_t);
                }
            }
            if ret < 0 as ::core::ffi::c_int {
                xmlIOErr(
                    XML_IO_WRITE as ::core::ffi::c_int,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                (*out).error = XML_IO_WRITE as ::core::ffi::c_int;
                return ret;
            }
            if (*out).written > INT_MAX - ret {
                (*out).written = INT_MAX;
            } else {
                (*out).written += ret;
            }
        }
        written += nbchars;
        if !(len > 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return written;
}
unsafe extern "C" fn xmlEscapeContent(
    mut out: *mut ::core::ffi::c_uchar,
    mut outlen: *mut ::core::ffi::c_int,
    mut in_0: *const xmlChar,
    mut inlen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut outstart: *mut ::core::ffi::c_uchar = out;
    let mut base: *const ::core::ffi::c_uchar = in_0 as *const ::core::ffi::c_uchar;
    let mut outend: *mut ::core::ffi::c_uchar = out.offset(*outlen as isize);
    let mut inend: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    inend = in_0.offset(*inlen as isize) as *const ::core::ffi::c_uchar;
    while in_0 < inend && out < outend {
        if *in_0 as ::core::ffi::c_int == '<' as i32 {
            if (outend.offset_from(out) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                break;
            }
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = '&' as i32 as ::core::ffi::c_uchar;
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 = 'l' as i32 as ::core::ffi::c_uchar;
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = 't' as i32 as ::core::ffi::c_uchar;
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = ';' as i32 as ::core::ffi::c_uchar;
        } else if *in_0 as ::core::ffi::c_int == '>' as i32 {
            if (outend.offset_from(out) as ::core::ffi::c_long) < 4 as ::core::ffi::c_long {
                break;
            }
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 = '&' as i32 as ::core::ffi::c_uchar;
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = 'g' as i32 as ::core::ffi::c_uchar;
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = 't' as i32 as ::core::ffi::c_uchar;
            let fresh9 = out;
            out = out.offset(1);
            *fresh9 = ';' as i32 as ::core::ffi::c_uchar;
        } else if *in_0 as ::core::ffi::c_int == '&' as i32 {
            if (outend.offset_from(out) as ::core::ffi::c_long) < 5 as ::core::ffi::c_long {
                break;
            }
            let fresh10 = out;
            out = out.offset(1);
            *fresh10 = '&' as i32 as ::core::ffi::c_uchar;
            let fresh11 = out;
            out = out.offset(1);
            *fresh11 = 'a' as i32 as ::core::ffi::c_uchar;
            let fresh12 = out;
            out = out.offset(1);
            *fresh12 = 'm' as i32 as ::core::ffi::c_uchar;
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = 'p' as i32 as ::core::ffi::c_uchar;
            let fresh14 = out;
            out = out.offset(1);
            *fresh14 = ';' as i32 as ::core::ffi::c_uchar;
        } else if *in_0 as ::core::ffi::c_int == '\r' as i32 {
            if (outend.offset_from(out) as ::core::ffi::c_long) < 5 as ::core::ffi::c_long {
                break;
            }
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = '&' as i32 as ::core::ffi::c_uchar;
            let fresh16 = out;
            out = out.offset(1);
            *fresh16 = '#' as i32 as ::core::ffi::c_uchar;
            let fresh17 = out;
            out = out.offset(1);
            *fresh17 = '1' as i32 as ::core::ffi::c_uchar;
            let fresh18 = out;
            out = out.offset(1);
            *fresh18 = '3' as i32 as ::core::ffi::c_uchar;
            let fresh19 = out;
            out = out.offset(1);
            *fresh19 = ';' as i32 as ::core::ffi::c_uchar;
        } else {
            let fresh20 = out;
            out = out.offset(1);
            *fresh20 = *in_0 as ::core::ffi::c_uchar;
        }
        in_0 = in_0.offset(1);
    }
    *outlen = out.offset_from(outstart) as ::core::ffi::c_long as ::core::ffi::c_int;
    *inlen = in_0.offset_from(base) as ::core::ffi::c_long as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferWriteEscape(
    mut out: xmlOutputBufferPtr,
    mut str: *const xmlChar,
    mut escaping: xmlCharEncodingOutputFunc,
) -> ::core::ffi::c_int {
    let mut nbchars: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    let mut written: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut oldwritten: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut chunk: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut cons: ::core::ffi::c_int = 0;
    if out.is_null() || (*out).error != 0 || str.is_null() || (*out).buffer.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    len = strlen(str as *const ::core::ffi::c_char) as ::core::ffi::c_int;
    if len < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*out).error != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if escaping.is_none() {
        escaping = Some(
            xmlEscapeContent
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_uchar,
                    *mut ::core::ffi::c_int,
                    *const xmlChar,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ) as xmlCharEncodingOutputFunc;
    }
    loop {
        oldwritten = written;
        cons = len;
        chunk = xmlBufAvail((*out).buffer) as ::core::ffi::c_int;
        if chunk < 40 as ::core::ffi::c_int {
            if xmlBufGrow((*out).buffer, 100 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            oldwritten = -(1 as ::core::ffi::c_int);
        } else {
            if !(*out).encoder.is_null() {
                if (*out).conv.is_null() {
                    (*out).conv = xmlBufCreate();
                }
                ret = escaping.expect("non-null function pointer")(
                    xmlBufEnd((*out).buffer) as *mut ::core::ffi::c_uchar,
                    &raw mut chunk,
                    str as *const ::core::ffi::c_uchar,
                    &raw mut cons,
                );
                if ret < 0 as ::core::ffi::c_int || chunk == 0 as ::core::ffi::c_int {
                    return -(1 as ::core::ffi::c_int);
                }
                xmlBufAddLen((*out).buffer, chunk as size_t);
                if xmlBufUse((*out).buffer) < MINLEN as size_t && cons == len {
                    break;
                }
                ret = xmlCharEncOutput(out, 0 as ::core::ffi::c_int);
                if ret < 0 as ::core::ffi::c_int && ret != -(3 as ::core::ffi::c_int) {
                    xmlIOErr(
                        XML_IO_ENCODER as ::core::ffi::c_int,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                    (*out).error = XML_IO_ENCODER as ::core::ffi::c_int;
                    return -(1 as ::core::ffi::c_int);
                }
                if (*out).writecallback.is_some() {
                    nbchars = xmlBufUse((*out).conv) as ::core::ffi::c_int;
                } else {
                    nbchars = if ret >= 0 as ::core::ffi::c_int {
                        ret
                    } else {
                        0 as ::core::ffi::c_int
                    };
                }
            } else {
                ret = escaping.expect("non-null function pointer")(
                    xmlBufEnd((*out).buffer) as *mut ::core::ffi::c_uchar,
                    &raw mut chunk,
                    str as *const ::core::ffi::c_uchar,
                    &raw mut cons,
                );
                if ret < 0 as ::core::ffi::c_int || chunk == 0 as ::core::ffi::c_int {
                    return -(1 as ::core::ffi::c_int);
                }
                xmlBufAddLen((*out).buffer, chunk as size_t);
                if (*out).writecallback.is_some() {
                    nbchars = xmlBufUse((*out).buffer) as ::core::ffi::c_int;
                } else {
                    nbchars = chunk;
                }
            }
            str = str.offset(cons as isize);
            len -= cons;
            if (*out).writecallback.is_some() {
                if nbchars < MINLEN && len <= 0 as ::core::ffi::c_int {
                    break;
                }
                if !(*out).encoder.is_null() {
                    ret = (*out).writecallback.expect("non-null function pointer")(
                        (*out).context,
                        xmlBufContent((*out).conv as *const xmlBuf) as *const ::core::ffi::c_char,
                        nbchars,
                    );
                    if ret >= 0 as ::core::ffi::c_int {
                        xmlBufShrink((*out).conv, ret as size_t);
                    }
                } else {
                    ret = (*out).writecallback.expect("non-null function pointer")(
                        (*out).context,
                        xmlBufContent((*out).buffer as *const xmlBuf) as *const ::core::ffi::c_char,
                        nbchars,
                    );
                    if ret >= 0 as ::core::ffi::c_int {
                        xmlBufShrink((*out).buffer, ret as size_t);
                    }
                }
                if ret < 0 as ::core::ffi::c_int {
                    xmlIOErr(
                        XML_IO_WRITE as ::core::ffi::c_int,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                    (*out).error = XML_IO_WRITE as ::core::ffi::c_int;
                    return ret;
                }
                if (*out).written > INT_MAX - ret {
                    (*out).written = INT_MAX;
                } else {
                    (*out).written += ret;
                }
            } else if xmlBufAvail((*out).buffer) < MINLEN as size_t {
                xmlBufGrow((*out).buffer, MINLEN);
            }
            written += nbchars;
        }
        if !(len > 0 as ::core::ffi::c_int && oldwritten != written) {
            break;
        }
    }
    return written;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferWriteString(
    mut out: xmlOutputBufferPtr,
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    if out.is_null() || (*out).error != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if str.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    len = strlen(str) as ::core::ffi::c_int;
    if len > 0 as ::core::ffi::c_int {
        return xmlOutputBufferWrite(out, len, str);
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn xmlOutputBufferFlush(mut out: xmlOutputBufferPtr) -> ::core::ffi::c_int {
    let mut nbchars: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if out.is_null() || (*out).error != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*out).conv.is_null() && !(*out).encoder.is_null() {
        loop {
            nbchars = xmlCharEncOutput(out, 0 as ::core::ffi::c_int);
            if nbchars < 0 as ::core::ffi::c_int {
                xmlIOErr(
                    XML_IO_ENCODER as ::core::ffi::c_int,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                (*out).error = XML_IO_ENCODER as ::core::ffi::c_int;
                return -(1 as ::core::ffi::c_int);
            }
            if !(nbchars != 0) {
                break;
            }
        }
    }
    if !(*out).conv.is_null() && !(*out).encoder.is_null() && (*out).writecallback.is_some() {
        ret = (*out).writecallback.expect("non-null function pointer")(
            (*out).context,
            xmlBufContent((*out).conv as *const xmlBuf) as *const ::core::ffi::c_char,
            xmlBufUse((*out).conv) as ::core::ffi::c_int,
        );
        if ret >= 0 as ::core::ffi::c_int {
            xmlBufShrink((*out).conv, ret as size_t);
        }
    } else if (*out).writecallback.is_some() {
        ret = (*out).writecallback.expect("non-null function pointer")(
            (*out).context,
            xmlBufContent((*out).buffer as *const xmlBuf) as *const ::core::ffi::c_char,
            xmlBufUse((*out).buffer) as ::core::ffi::c_int,
        );
        if ret >= 0 as ::core::ffi::c_int {
            xmlBufShrink((*out).buffer, ret as size_t);
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        xmlIOErr(
            XML_IO_FLUSH as ::core::ffi::c_int,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        (*out).error = XML_IO_FLUSH as ::core::ffi::c_int;
        return ret;
    }
    if (*out).written > INT_MAX - ret {
        (*out).written = INT_MAX;
    } else {
        (*out).written += ret;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserGetDirectory(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dir: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut cur: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if xmlInputCallbackInitialized == 0 as ::core::ffi::c_int {
        xmlRegisterDefaultInputCallbacks();
    }
    if filename.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    strncpy(
        &raw mut dir as *mut ::core::ffi::c_char,
        filename,
        1023 as size_t,
    );
    dir[1023 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    cur = (&raw mut dir as *mut ::core::ffi::c_char).offset((strlen
        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> size_t)(
        &raw mut dir as *mut ::core::ffi::c_char,
    ) as isize) as *mut ::core::ffi::c_char;
    while cur > &raw mut dir as *mut ::core::ffi::c_char {
        if *cur as ::core::ffi::c_int == '/' as i32 {
            break;
        }
        cur = cur.offset(-1);
    }
    if *cur as ::core::ffi::c_int == '/' as i32 {
        if cur == &raw mut dir as *mut ::core::ffi::c_char {
            dir[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        } else {
            *cur = 0 as ::core::ffi::c_char;
        }
        ret = xmlMemStrdup.expect("non-null function pointer")(
            &raw mut dir as *mut ::core::ffi::c_char,
        );
    } else if !getcwd(&raw mut dir as *mut ::core::ffi::c_char, 1024 as size_t).is_null() {
        dir[1023 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        ret = xmlMemStrdup.expect("non-null function pointer")(
            &raw mut dir as *mut ::core::ffi::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCheckHTTPInput(
    mut ctxt: xmlParserCtxtPtr,
    mut ret: xmlParserInputPtr,
) -> xmlParserInputPtr {
    if !ret.is_null()
        && !(*ret).buf.is_null()
        && (*(*ret).buf).readcallback
            == Some(
                xmlIOHTTPRead
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        && !(*(*ret).buf).context.is_null()
    {
        let mut encoding: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut redir: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut mime: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut code: ::core::ffi::c_int = 0;
        code = xmlNanoHTTPReturnCode((*(*ret).buf).context);
        if code >= 400 as ::core::ffi::c_int {
            if !(*ret).filename.is_null() {
                __xmlLoaderErr(
                    ctxt as *mut ::core::ffi::c_void,
                    b"failed to load HTTP resource \"%s\"\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*ret).filename,
                );
            } else {
                __xmlLoaderErr(
                    ctxt as *mut ::core::ffi::c_void,
                    b"failed to load HTTP resource\n\0" as *const u8 as *const ::core::ffi::c_char,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
            xmlFreeInputStream(ret);
            ret = ::core::ptr::null_mut::<xmlParserInput>();
        } else {
            mime = xmlNanoHTTPMimeType((*(*ret).buf).context);
            if !xmlStrstr(
                mime as *mut xmlChar,
                b"/xml\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            )
            .is_null()
                || !xmlStrstr(
                    mime as *mut xmlChar,
                    b"+xml\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                )
                .is_null()
            {
                encoding = xmlNanoHTTPEncoding((*(*ret).buf).context);
                if !encoding.is_null() {
                    let mut handler: xmlCharEncodingHandlerPtr =
                        ::core::ptr::null_mut::<xmlCharEncodingHandler>();
                    handler = xmlFindCharEncodingHandler(encoding);
                    if !handler.is_null() {
                        xmlSwitchInputEncoding(ctxt, ret, handler);
                    } else {
                        __xmlErrEncoding(
                            ctxt,
                            XML_ERR_UNKNOWN_ENCODING,
                            b"Unknown encoding %s\0" as *const u8 as *const ::core::ffi::c_char,
                            encoding as *mut xmlChar,
                            ::core::ptr::null::<xmlChar>(),
                        );
                    }
                }
            }
            redir = xmlNanoHTTPRedir((*(*ret).buf).context);
            if !redir.is_null() {
                if !(*ret).filename.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*ret).filename as *mut xmlChar as *mut ::core::ffi::c_void,
                    );
                }
                if !(*ret).directory.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*ret).directory as *mut xmlChar as *mut ::core::ffi::c_void,
                    );
                    (*ret).directory = ::core::ptr::null::<::core::ffi::c_char>();
                }
                (*ret).filename = xmlStrdup(redir as *const xmlChar) as *mut ::core::ffi::c_char;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn xmlNoNetExists(mut URL: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if URL.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if xmlStrncasecmp(
        URL as *mut xmlChar,
        b"file://localhost/\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        17 as ::core::ffi::c_int,
    ) == 0
    {
        path = URL.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
    } else if xmlStrncasecmp(
        URL as *mut xmlChar,
        b"file:///\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        8 as ::core::ffi::c_int,
    ) == 0
    {
        path = URL.offset(7 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
    } else {
        path = URL;
    }
    return xmlCheckFilename(path);
}
unsafe extern "C" fn xmlResolveResourceFromCatalog(
    mut URL: *const ::core::ffi::c_char,
    mut ID: *const ::core::ffi::c_char,
    mut ctxt: xmlParserCtxtPtr,
) -> *mut xmlChar {
    let mut resource: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut pref: xmlCatalogAllow = XML_CATA_ALLOW_NONE;
    pref = xmlCatalogGetDefaults();
    if pref as ::core::ffi::c_uint
        != XML_CATA_ALLOW_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        && xmlNoNetExists(URL) == 0
    {
        if !ctxt.is_null()
            && !(*ctxt).catalogs.is_null()
            && (pref as ::core::ffi::c_uint
                == XML_CATA_ALLOW_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
                || pref as ::core::ffi::c_uint
                    == XML_CATA_ALLOW_DOCUMENT as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            resource = xmlCatalogLocalResolve(
                (*ctxt).catalogs,
                ID as *const xmlChar,
                URL as *const xmlChar,
            );
        }
        if resource.is_null()
            && (pref as ::core::ffi::c_uint
                == XML_CATA_ALLOW_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
                || pref as ::core::ffi::c_uint
                    == XML_CATA_ALLOW_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            resource = xmlCatalogResolve(ID as *const xmlChar, URL as *const xmlChar);
        }
        if resource.is_null() && !URL.is_null() {
            resource = xmlStrdup(URL as *const xmlChar);
        }
        if !resource.is_null() && xmlNoNetExists(resource as *const ::core::ffi::c_char) == 0 {
            let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            if !ctxt.is_null()
                && !(*ctxt).catalogs.is_null()
                && (pref as ::core::ffi::c_uint
                    == XML_CATA_ALLOW_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
                    || pref as ::core::ffi::c_uint
                        == XML_CATA_ALLOW_DOCUMENT as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                tmp = xmlCatalogLocalResolveURI((*ctxt).catalogs, resource);
            }
            if tmp.is_null()
                && (pref as ::core::ffi::c_uint
                    == XML_CATA_ALLOW_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
                    || pref as ::core::ffi::c_uint
                        == XML_CATA_ALLOW_GLOBAL as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                tmp = xmlCatalogResolveURI(resource);
            }
            if !tmp.is_null() {
                xmlFree.expect("non-null function pointer")(resource as *mut ::core::ffi::c_void);
                resource = tmp;
            }
        }
    }
    return resource;
}
unsafe extern "C" fn xmlDefaultExternalEntityLoader(
    mut URL: *const ::core::ffi::c_char,
    mut ID: *const ::core::ffi::c_char,
    mut ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    let mut ret: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut resource: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if !ctxt.is_null() && (*ctxt).options & XML_PARSE_NONET as ::core::ffi::c_int != 0 {
        let mut options: ::core::ffi::c_int = (*ctxt).options;
        (*ctxt).options -= XML_PARSE_NONET as ::core::ffi::c_int;
        ret = xmlNoNetExternalEntityLoader(URL, ID, ctxt);
        (*ctxt).options = options;
        return ret;
    }
    resource = xmlResolveResourceFromCatalog(URL, ID, ctxt);
    if resource.is_null() {
        resource = URL as *mut xmlChar;
    }
    if resource.is_null() {
        if ID.is_null() {
            ID = b"NULL\0" as *const u8 as *const ::core::ffi::c_char;
        }
        __xmlLoaderErr(
            ctxt as *mut ::core::ffi::c_void,
            b"failed to load external entity \"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
            ID,
        );
        return ::core::ptr::null_mut::<xmlParserInput>();
    }
    ret = xmlNewInputFromFile(ctxt, resource as *const ::core::ffi::c_char);
    if !resource.is_null() && resource != URL as *mut xmlChar {
        xmlFree.expect("non-null function pointer")(resource as *mut ::core::ffi::c_void);
    }
    return ret;
}
static mut xmlCurrentExternalEntityLoader: xmlExternalEntityLoader = unsafe {
    Some(
        xmlDefaultExternalEntityLoader
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                xmlParserCtxtPtr,
            ) -> xmlParserInputPtr,
    )
};
#[no_mangle]
pub unsafe extern "C" fn xmlSetExternalEntityLoader(mut f: xmlExternalEntityLoader) {
    xmlCurrentExternalEntityLoader = f;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetExternalEntityLoader() -> xmlExternalEntityLoader {
    return xmlCurrentExternalEntityLoader;
}
#[no_mangle]
pub unsafe extern "C" fn xmlLoadExternalEntity(
    mut URL: *const ::core::ffi::c_char,
    mut ID: *const ::core::ffi::c_char,
    mut ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    if !URL.is_null() && xmlNoNetExists(URL) == 0 as ::core::ffi::c_int {
        let mut canonicFilename: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ret: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
        canonicFilename = xmlCanonicPath(URL as *const xmlChar) as *mut ::core::ffi::c_char;
        if canonicFilename.is_null() {
            xmlErrMemory(
                ctxt,
                b"building canonical path\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<xmlParserInput>();
        }
        ret = xmlCurrentExternalEntityLoader.expect("non-null function pointer")(
            canonicFilename,
            ID,
            ctxt,
        );
        xmlFree.expect("non-null function pointer")(canonicFilename as *mut ::core::ffi::c_void);
        return ret;
    }
    return xmlCurrentExternalEntityLoader.expect("non-null function pointer")(URL, ID, ctxt);
}
#[no_mangle]
pub unsafe extern "C" fn xmlNoNetExternalEntityLoader(
    mut URL: *const ::core::ffi::c_char,
    mut ID: *const ::core::ffi::c_char,
    mut ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut resource: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    resource = xmlResolveResourceFromCatalog(URL, ID, ctxt);
    if resource.is_null() {
        resource = URL as *mut xmlChar;
    }
    if !resource.is_null() {
        if xmlStrncasecmp(
            resource,
            b"ftp://\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            6 as ::core::ffi::c_int,
        ) == 0
            || xmlStrncasecmp(
                resource,
                b"http://\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                7 as ::core::ffi::c_int,
            ) == 0
        {
            xmlIOErr(
                XML_IO_NETWORK_ATTEMPT as ::core::ffi::c_int,
                resource as *const ::core::ffi::c_char,
            );
            if resource != URL as *mut xmlChar {
                xmlFree.expect("non-null function pointer")(resource as *mut ::core::ffi::c_void);
            }
            return ::core::ptr::null_mut::<xmlParserInput>();
        }
    }
    input = xmlDefaultExternalEntityLoader(resource as *const ::core::ffi::c_char, ID, ctxt);
    if resource != URL as *mut xmlChar {
        xmlFree.expect("non-null function pointer")(resource as *mut ::core::ffi::c_void);
    }
    return input;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
