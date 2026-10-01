extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlBuf;
    pub type _xmlAttrHashBucket;
    pub type _xmlParserNsData;
    pub type _xmlStartTag;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    pub type _xmlRMutex;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __xstat(
        __ver: ::core::ffi::c_int,
        __filename: *const ::core::ffi::c_char,
        __stat_buf: *mut stat,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlMallocAtomic: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrndup(cur: *const xmlChar, len: ::core::ffi::c_int) -> *mut xmlChar;
    fn xmlStrncmp(
        str1: *const xmlChar,
        str2: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrlen(str: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    fn xmlHashCreate(size: ::core::ffi::c_int) -> xmlHashTablePtr;
    fn xmlHashFree(hash: xmlHashTablePtr, dealloc: xmlHashDeallocator);
    fn xmlHashAddEntry(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn xmlHashRemoveEntry(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        dealloc: xmlHashDeallocator,
    ) -> ::core::ffi::c_int;
    fn xmlHashLookup(hash: xmlHashTablePtr, name: *const xmlChar) -> *mut ::core::ffi::c_void;
    fn xmlHashSize(hash: xmlHashTablePtr) -> ::core::ffi::c_int;
    fn xmlHashScan(hash: xmlHashTablePtr, scan: xmlHashScanner, data: *mut ::core::ffi::c_void);
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    fn xmlParserInputBufferCreateFilename(
        URI: *const ::core::ffi::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    fn xmlParserGetDirectory(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn xmlOutputBufferCreateFile(
        file: *mut FILE,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut ::core::ffi::c_void;
    fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int;
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlNewDtd(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    fn xmlNewNs(node: xmlNodePtr, href: *const xmlChar, prefix: *const xmlChar) -> xmlNsPtr;
    fn xmlFreeNs(cur: xmlNsPtr);
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlSearchNsByHref(doc: xmlDocPtr, node: xmlNodePtr, href: *const xmlChar) -> xmlNsPtr;
    fn xmlSetProp(node: xmlNodePtr, name: *const xmlChar, value: *const xmlChar) -> xmlAttrPtr;
    fn xmlSetNsProp(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlGetNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
    fn xmlSaveFormatFileTo(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const ::core::ffi::c_char,
        format: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlNewRMutex() -> xmlRMutexPtr;
    fn xmlRMutexLock(tok: xmlRMutexPtr);
    fn xmlRMutexUnlock(tok: xmlRMutexPtr);
    fn xmlFreeRMutex(tok: xmlRMutexPtr);
    fn xmlGetThreadId() -> ::core::ffi::c_int;
    fn xmlCharInRange(
        val: ::core::ffi::c_uint,
        group: *const xmlChRangeGroup,
    ) -> ::core::ffi::c_int;
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    static xmlIsDigitGroup: xmlChRangeGroup;
    static xmlIsPubidChar_tab: [::core::ffi::c_uchar; 256];
    fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr) -> ::core::ffi::c_int;
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
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr) -> ::core::ffi::c_int;
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
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type xmlChar = ::core::ffi::c_uchar;
pub type xmlDict = _xmlDict;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlHashTable = _xmlHashTable;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashDeallocator =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type xmlHashScanner = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *const xmlChar) -> (),
>;
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
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut ::core::ffi::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
}
pub type xmlCharEncodingOutputFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlCharEncodingInputFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
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
pub type xmlStartTag = _xmlStartTag;
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
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
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
pub const XML_FROM_IO: C2RustUnnamed = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed = 6;
pub const XML_FROM_HTML: C2RustUnnamed = 5;
pub const XML_FROM_DTD: C2RustUnnamed = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed = 3;
pub const XML_FROM_TREE: C2RustUnnamed = 2;
pub const XML_FROM_PARSER: C2RustUnnamed = 1;
pub const XML_FROM_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_0 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_0 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_0 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_0 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_0 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_0 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_0 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_0 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_0 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_0 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_0 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_0 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_0 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_0 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_0 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_0 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_0 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_0 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_0 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_0 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_0 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_0 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_0 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_0 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_0 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_0 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_0 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_0 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_0 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_0 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_0 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_0 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_0 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_0 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_0 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_0 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_0 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_0 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_0 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_0 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_0 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_0 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_0 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_0 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_0 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_0 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_0 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_0 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_0 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_0 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_0 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_0 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_0 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_0 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_0 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_0 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_0 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_0 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_0 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_0 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_0 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_0 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_0 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_0 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_0 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_0 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_0 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_0 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_0 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_0 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_0 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_0 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_0 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_0 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_0 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_0 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_0 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_0 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_0 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_0 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_0 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_0 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_0 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_0 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_0 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_0 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_0 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_0 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_0 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_0 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_0 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_0 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_0 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_0 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_0 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_0 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_0 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_0 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_0 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_0 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_0 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_0 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_0 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_0 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_0 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_0 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_0 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_0 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_0 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_0 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_0 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_0 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_0 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_0 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_0 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_0 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_0 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_0 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_0 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_0 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_0 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_0 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_0 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_0 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_0 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_0 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_0 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_0 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_0 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_0 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_0 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_0 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_0 = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_0 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_0 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_0 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_0 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_0 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_0 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_0 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_0 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_0 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_0 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_0 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_0 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_0 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_0 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_0 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_0 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_0 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_0 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_0 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_0 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_0 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_0 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_0 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_0 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_0 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_0 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_0 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_0 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_0 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_0 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_0 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_0 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_0 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_0 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_0 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_0 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_0 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_0 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_0 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_0 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_0 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_0 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_0 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_0 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_0 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_0 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_0 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_0 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_0 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_0 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_0 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_0 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_0 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_0 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_0 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_0 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_0 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_0 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_0 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_0 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_0 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_0 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_0 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_0 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_0 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_0 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_0 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_0 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_0 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_0 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_0 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_0 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_0 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_0 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_0 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_0 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_0 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_0 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_0 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_0 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_0 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_0 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_0 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_0 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_0 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_0 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_0 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_0 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_0 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_0 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_0 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_0 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_0 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_0 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_0 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_0 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_0 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_0 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_0 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_0 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_0 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_0 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_0 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_0 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_0 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_0 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_0 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_0 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_0 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_0 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_0 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_0 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_0 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_0 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_0 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_0 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_0 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_0 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_0 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_0 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_0 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_0 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_0 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_0 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_0 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_0 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_0 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_0 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_0 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_0 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_0 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_0 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_0 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_0 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_0 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_0 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_0 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_0 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_0 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_0 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_0 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_0 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_0 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_0 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_0 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_0 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_0 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_0 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_0 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_0 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_0 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_0 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_0 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_0 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_0 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_0 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_0 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_0 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_0 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_0 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_0 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_0 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_0 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_0 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_0 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_0 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_0 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_0 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_0 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_0 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_0 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_0 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_0 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_0 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_0 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_0 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_0 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_0 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_0 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_0 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_0 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_0 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_0 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_0 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_0 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_0 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_0 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_0 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_0 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_0 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_0 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_0 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_0 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_0 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_0 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_0 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_0 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_0 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_0 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_0 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_0 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_0 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_0 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_0 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_0 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_0 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_0 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_0 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_0 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_0 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_0 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_0 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_0 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_0 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_0 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_0 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_0 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_0 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_0 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_0 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_0 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_0 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_0 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_0 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_0 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_0 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_0 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_0 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_0 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_0 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_0 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_0 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_0 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_0 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_0 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_0 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_0 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_0 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_0 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_0 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_0 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_0 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_0 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_0 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_0 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_0 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_0 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_0 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_0 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_0 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_0 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_0 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_0 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_0 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_0 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_0 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_0 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_0 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_0 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_0 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_0 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_0 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_0 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_0 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_0 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_0 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_0 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_0 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_0 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_0 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_0 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_0 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_0 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_0 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_0 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_0 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_0 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_0 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_0 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_0 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_0 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_0 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_0 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_0 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_0 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_0 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_0 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_0 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_0 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_0 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_0 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_0 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_0 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_0 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_0 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_0 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_0 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_0 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_0 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_0 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_0 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_0 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_0 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_0 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_0 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_0 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_0 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_0 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_0 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_0 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_0 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_0 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_0 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_0 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_0 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_0 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_0 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_0 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_0 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_0 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_0 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_0 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_0 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_0 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_0 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_0 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_0 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_0 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_0 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_0 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_0 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_0 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_0 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_0 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_0 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_0 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_0 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_0 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_0 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_0 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_0 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_0 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_0 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_0 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_0 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_0 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_0 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_0 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_0 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_0 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_0 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_0 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_0 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_0 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_0 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_0 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_0 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_0 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_0 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_0 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_0 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_0 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_0 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_0 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_0 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_0 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_0 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_0 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_0 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_0 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_0 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_0 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_0 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_0 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_0 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_0 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_0 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_0 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_0 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_0 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_0 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_0 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_0 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_0 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_0 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_0 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_0 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_0 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_0 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_0 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_0 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_0 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_0 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_0 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_0 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_0 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_0 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_0 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_0 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_0 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_0 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_0 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_0 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_0 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_0 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_0 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_0 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_0 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_0 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_0 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_0 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_0 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_0 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_0 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_0 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_0 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1000;
pub const XML_HTML_INCORRECTLY_OPENED_COMMENT: C2RustUnnamed_0 = 802;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_0 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_0 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_0 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_0 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_0 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_0 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_0 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_0 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_0 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_0 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_0 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_0 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_0 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_0 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_0 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_0 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_0 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_0 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_0 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_0 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_0 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_0 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_0 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_0 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_0 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_0 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_0 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_0 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_0 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_0 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_0 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_0 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_0 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_0 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_0 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_0 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_0 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_0 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_0 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_0 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_0 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_0 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_0 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_0 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_0 = 200;
pub const XML_WAR_ENCODING_MISMATCH: C2RustUnnamed_0 = 113;
pub const XML_ERR_COMMENT_ABRUPTLY_ENDED: C2RustUnnamed_0 = 112;
pub const XML_ERR_USER_STOP: C2RustUnnamed_0 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_0 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_0 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_0 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_0 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_0 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_0 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_0 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_0 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_0 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_0 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_0 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_0 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_0 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_0 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_0 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_0 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_0 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_0 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_0 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_0 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_0 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_0 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_0 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_0 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_0 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_0 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_0 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_0 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_0 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_0 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_0 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_0 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_0 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_0 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_0 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_0 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_0 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_0 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_0 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_0 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_0 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_0 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_0 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_0 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_0 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_0 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_0 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_0 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_0 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_0 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_0 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_0 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_0 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_0 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_0 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_0 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_0 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_0 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_0 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_0 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_0 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_0 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_0 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_0 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_0 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_0 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_0 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_0 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_0 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_0 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_0 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_0 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_0 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_0 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_0 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_0 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_0 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_0 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_0 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_0 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_0 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_0 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_0 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_0 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_0 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_0 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_0 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_0 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_0 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_0 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_0 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_0 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_0 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_0 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_0 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_0 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_0 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_0 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_0 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_0 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_0 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_0 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_0 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_0 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_0 = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_0 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_0 = 1;
pub const XML_ERR_OK: C2RustUnnamed_0 = 0;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
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
pub type xmlRMutex = _xmlRMutex;
pub type xmlRMutexPtr = *mut xmlRMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: ::core::ffi::c_ushort,
    pub high: ::core::ffi::c_ushort,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: ::core::ffi::c_uint,
    pub high: ::core::ffi::c_uint,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: ::core::ffi::c_int,
    pub nbLongRange: ::core::ffi::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChRangeGroup = _xmlChRangeGroup;
pub type xmlCatalogPrefer = ::core::ffi::c_uint;
pub const XML_CATA_PREFER_SYSTEM: xmlCatalogPrefer = 2;
pub const XML_CATA_PREFER_PUBLIC: xmlCatalogPrefer = 1;
pub const XML_CATA_PREFER_NONE: xmlCatalogPrefer = 0;
pub type xmlCatalogAllow = ::core::ffi::c_uint;
pub const XML_CATA_ALLOW_ALL: xmlCatalogAllow = 3;
pub const XML_CATA_ALLOW_DOCUMENT: xmlCatalogAllow = 2;
pub const XML_CATA_ALLOW_GLOBAL: xmlCatalogAllow = 1;
pub const XML_CATA_ALLOW_NONE: xmlCatalogAllow = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCatalog {
    pub type_0: xmlCatalogType,
    pub catalTab: [*mut ::core::ffi::c_char; 10],
    pub catalNr: ::core::ffi::c_int,
    pub catalMax: ::core::ffi::c_int,
    pub sgml: xmlHashTablePtr,
    pub prefer: xmlCatalogPrefer,
    pub xml: xmlCatalogEntryPtr,
}
pub type xmlCatalogEntryPtr = *mut xmlCatalogEntry;
pub type xmlCatalogEntry = _xmlCatalogEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCatalogEntry {
    pub next: *mut _xmlCatalogEntry,
    pub parent: *mut _xmlCatalogEntry,
    pub children: *mut _xmlCatalogEntry,
    pub type_0: xmlCatalogEntryType,
    pub name: *mut xmlChar,
    pub value: *mut xmlChar,
    pub URL: *mut xmlChar,
    pub prefer: xmlCatalogPrefer,
    pub dealloc: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub group: *mut _xmlCatalogEntry,
}
pub type xmlCatalogEntryType = ::core::ffi::c_int;
pub const SGML_CATA_SGMLDECL: xmlCatalogEntryType = 24;
pub const SGML_CATA_DOCUMENT: xmlCatalogEntryType = 23;
pub const SGML_CATA_CATALOG: xmlCatalogEntryType = 22;
pub const SGML_CATA_BASE: xmlCatalogEntryType = 21;
pub const SGML_CATA_DELEGATE: xmlCatalogEntryType = 20;
pub const SGML_CATA_NOTATION: xmlCatalogEntryType = 19;
pub const SGML_CATA_LINKTYPE: xmlCatalogEntryType = 18;
pub const SGML_CATA_DOCTYPE: xmlCatalogEntryType = 17;
pub const SGML_CATA_PENTITY: xmlCatalogEntryType = 16;
pub const SGML_CATA_ENTITY: xmlCatalogEntryType = 15;
pub const SGML_CATA_PUBLIC: xmlCatalogEntryType = 14;
pub const SGML_CATA_SYSTEM: xmlCatalogEntryType = 13;
pub const XML_CATA_DELEGATE_URI: xmlCatalogEntryType = 12;
pub const XML_CATA_REWRITE_URI: xmlCatalogEntryType = 11;
pub const XML_CATA_URI: xmlCatalogEntryType = 10;
pub const XML_CATA_DELEGATE_SYSTEM: xmlCatalogEntryType = 9;
pub const XML_CATA_DELEGATE_PUBLIC: xmlCatalogEntryType = 8;
pub const XML_CATA_REWRITE_SYSTEM: xmlCatalogEntryType = 7;
pub const XML_CATA_SYSTEM: xmlCatalogEntryType = 6;
pub const XML_CATA_PUBLIC: xmlCatalogEntryType = 5;
pub const XML_CATA_GROUP: xmlCatalogEntryType = 4;
pub const XML_CATA_NEXT_CATALOG: xmlCatalogEntryType = 3;
pub const XML_CATA_BROKEN_CATALOG: xmlCatalogEntryType = 2;
pub const XML_CATA_CATALOG: xmlCatalogEntryType = 1;
pub const XML_CATA_NONE: xmlCatalogEntryType = 0;
pub const XML_CATA_REMOVED: xmlCatalogEntryType = -1;
pub type xmlCatalogType = ::core::ffi::c_uint;
pub const XML_SGML_CATALOG_TYPE: xmlCatalogType = 2;
pub const XML_XML_CATALOG_TYPE: xmlCatalogType = 1;
pub type xmlCatalog = _xmlCatalog;
pub type xmlCatalogPtr = *mut xmlCatalog;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _STAT_VER_LINUX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _STAT_VER: ::core::ffi::c_int = _STAT_VER_LINUX;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const ::core::ffi::c_char,
    mut __statbuf: *mut stat,
) -> ::core::ffi::c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
pub const XML_XML_NAMESPACE: *const xmlChar = b"http://www.w3.org/XML/1998/namespace\0" as *const u8
    as *const ::core::ffi::c_char as *const xmlChar;
pub const XML_MAX_NAMELEN: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const XML_CATALOGS_NAMESPACE: *const xmlChar = b"urn:oasis:names:tc:entity:xmlns:xml:catalog\0"
    as *const u8 as *const ::core::ffi::c_char
    as *const xmlChar;
pub const MAX_DELEGATE: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const MAX_CATAL_DEPTH: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const PATH_SEPARATOR: ::core::ffi::c_int = ':' as i32;
pub const XML_URN_PUBID: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"urn:publicid:\0") };
pub const XML_CATAL_BREAK: *mut xmlChar = -(1 as ::core::ffi::c_int) as *mut xmlChar;
pub const XML_XML_DEFAULT_CATALOG: [::core::ffi::c_char; 34] = unsafe {
    ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
        *b"file:///usr/local/etc/xml/catalog\0",
    )
};
pub const XML_MAX_SGML_CATA_DEPTH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut xmlDebugCatalogs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlCatalogDefaultAllow: xmlCatalogAllow = XML_CATA_ALLOW_ALL;
static mut xmlCatalogDefaultPrefer: xmlCatalogPrefer = XML_CATA_PREFER_PUBLIC;
static mut xmlCatalogXMLFiles: xmlHashTablePtr =
    ::core::ptr::null::<xmlHashTable>() as *mut xmlHashTable;
static mut xmlDefaultCatalog: xmlCatalogPtr = ::core::ptr::null::<xmlCatalog>() as *mut xmlCatalog;
static mut xmlCatalogMutex: xmlRMutexPtr = ::core::ptr::null::<xmlRMutex>() as *mut xmlRMutex;
static mut xmlCatalogInitialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn xmlCatalogErrMemory(mut extra: *const ::core::ffi::c_char) {
    __xmlRaiseError(
        None,
        None,
        NULL,
        NULL,
        NULL,
        XML_FROM_CATALOG as ::core::ffi::c_int,
        XML_ERR_NO_MEMORY as ::core::ffi::c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        extra,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        b"Memory allocation failed : %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        extra,
    );
}
unsafe extern "C" fn xmlCatalogErr(
    mut catal: xmlCatalogEntryPtr,
    mut node: xmlNodePtr,
    mut error: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut str3: *const xmlChar,
) {
    __xmlRaiseError(
        None,
        None,
        NULL,
        catal as *mut ::core::ffi::c_void,
        node as *mut ::core::ffi::c_void,
        XML_FROM_CATALOG as ::core::ffi::c_int,
        error,
        XML_ERR_ERROR,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        str1 as *const ::core::ffi::c_char,
        str2 as *const ::core::ffi::c_char,
        str3 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        msg,
        str1,
        str2,
        str3,
    );
}
unsafe extern "C" fn xmlNewCatalogEntry(
    mut type_0: xmlCatalogEntryType,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
    mut URL: *const xmlChar,
    mut prefer: xmlCatalogPrefer,
    mut group: xmlCatalogEntryPtr,
) -> xmlCatalogEntryPtr {
    let mut ret: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut normid: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlCatalogEntry>() as size_t
    ) as xmlCatalogEntryPtr;
    if ret.is_null() {
        xmlCatalogErrMemory(
            b"allocating catalog entry\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlCatalogEntry>();
    }
    (*ret).next = ::core::ptr::null_mut::<_xmlCatalogEntry>();
    (*ret).parent = ::core::ptr::null_mut::<_xmlCatalogEntry>();
    (*ret).children = ::core::ptr::null_mut::<_xmlCatalogEntry>();
    (*ret).type_0 = type_0;
    if type_0 as ::core::ffi::c_int == XML_CATA_PUBLIC as ::core::ffi::c_int
        || type_0 as ::core::ffi::c_int == XML_CATA_DELEGATE_PUBLIC as ::core::ffi::c_int
    {
        normid = xmlCatalogNormalizePublic(name);
        if !normid.is_null() {
            name = if *normid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                normid
            } else {
                ::core::ptr::null_mut::<xmlChar>()
            };
        }
    }
    if !name.is_null() {
        (*ret).name = xmlStrdup(name);
    } else {
        (*ret).name = ::core::ptr::null_mut::<xmlChar>();
    }
    if !normid.is_null() {
        xmlFree.expect("non-null function pointer")(normid as *mut ::core::ffi::c_void);
    }
    if !value.is_null() {
        (*ret).value = xmlStrdup(value);
    } else {
        (*ret).value = ::core::ptr::null_mut::<xmlChar>();
    }
    if URL.is_null() {
        URL = value;
    }
    if !URL.is_null() {
        (*ret).URL = xmlStrdup(URL);
    } else {
        (*ret).URL = ::core::ptr::null_mut::<xmlChar>();
    }
    (*ret).prefer = prefer;
    (*ret).dealloc = 0 as ::core::ffi::c_int;
    (*ret).depth = 0 as ::core::ffi::c_int;
    (*ret).group = group as *mut _xmlCatalogEntry;
    return ret;
}
unsafe extern "C" fn xmlFreeCatalogEntry(
    mut payload: *mut ::core::ffi::c_void,
    mut name: *const xmlChar,
) {
    let mut ret: xmlCatalogEntryPtr = payload as xmlCatalogEntryPtr;
    if ret.is_null() {
        return;
    }
    if (*ret).dealloc == 1 as ::core::ffi::c_int {
        return;
    }
    if xmlDebugCatalogs != 0 {
        if !(*ret).name.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Free catalog entry %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*ret).name,
            );
        } else if !(*ret).value.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Free catalog entry %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*ret).value,
            );
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Free catalog entry\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if !(*ret).name.is_null() {
        xmlFree.expect("non-null function pointer")((*ret).name as *mut ::core::ffi::c_void);
    }
    if !(*ret).value.is_null() {
        xmlFree.expect("non-null function pointer")((*ret).value as *mut ::core::ffi::c_void);
    }
    if !(*ret).URL.is_null() {
        xmlFree.expect("non-null function pointer")((*ret).URL as *mut ::core::ffi::c_void);
    }
    xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlFreeCatalogEntryList(mut ret: xmlCatalogEntryPtr) {
    let mut next: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    while !ret.is_null() {
        next = (*ret).next as xmlCatalogEntryPtr;
        xmlFreeCatalogEntry(
            ret as *mut ::core::ffi::c_void,
            ::core::ptr::null::<xmlChar>(),
        );
        ret = next;
    }
}
unsafe extern "C" fn xmlFreeCatalogHashEntryList(
    mut payload: *mut ::core::ffi::c_void,
    mut name: *const xmlChar,
) {
    let mut catal: xmlCatalogEntryPtr = payload as xmlCatalogEntryPtr;
    let mut children: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut next: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    if catal.is_null() {
        return;
    }
    children = (*catal).children as xmlCatalogEntryPtr;
    while !children.is_null() {
        next = (*children).next as xmlCatalogEntryPtr;
        (*children).dealloc = 0 as ::core::ffi::c_int;
        (*children).children = ::core::ptr::null_mut::<_xmlCatalogEntry>();
        xmlFreeCatalogEntry(
            children as *mut ::core::ffi::c_void,
            ::core::ptr::null::<xmlChar>(),
        );
        children = next;
    }
    (*catal).dealloc = 0 as ::core::ffi::c_int;
    xmlFreeCatalogEntry(
        catal as *mut ::core::ffi::c_void,
        ::core::ptr::null::<xmlChar>(),
    );
}
unsafe extern "C" fn xmlCreateNewCatalog(
    mut type_0: xmlCatalogType,
    mut prefer: xmlCatalogPrefer,
) -> xmlCatalogPtr {
    let mut ret: xmlCatalogPtr = ::core::ptr::null_mut::<xmlCatalog>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlCatalog>() as size_t
    ) as xmlCatalogPtr;
    if ret.is_null() {
        xmlCatalogErrMemory(b"allocating catalog\0" as *const u8 as *const ::core::ffi::c_char);
        return ::core::ptr::null_mut::<xmlCatalog>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlCatalog>() as size_t,
    );
    (*ret).type_0 = type_0;
    (*ret).catalNr = 0 as ::core::ffi::c_int;
    (*ret).catalMax = XML_MAX_SGML_CATA_DEPTH;
    (*ret).prefer = prefer;
    if (*ret).type_0 as ::core::ffi::c_uint
        == XML_SGML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*ret).sgml = xmlHashCreate(10 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeCatalog(mut catal: xmlCatalogPtr) {
    if catal.is_null() {
        return;
    }
    if !(*catal).xml.is_null() {
        xmlFreeCatalogEntryList((*catal).xml);
    }
    if !(*catal).sgml.is_null() {
        xmlHashFree(
            (*catal).sgml,
            Some(
                xmlFreeCatalogEntry
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
            ),
        );
    }
    xmlFree.expect("non-null function pointer")(catal as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlCatalogDumpEntry(
    mut payload: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
    mut name: *const xmlChar,
) {
    let mut entry: xmlCatalogEntryPtr = payload as xmlCatalogEntryPtr;
    let mut out: *mut FILE = data as *mut FILE;
    if entry.is_null() || out.is_null() {
        return;
    }
    match (*entry).type_0 as ::core::ffi::c_int {
        15 => {
            fprintf(out, b"ENTITY \0" as *const u8 as *const ::core::ffi::c_char);
        }
        16 => {
            fprintf(
                out,
                b"ENTITY %%\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        17 => {
            fprintf(
                out,
                b"DOCTYPE \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        18 => {
            fprintf(
                out,
                b"LINKTYPE \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        19 => {
            fprintf(
                out,
                b"NOTATION \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        14 => {
            fprintf(out, b"PUBLIC \0" as *const u8 as *const ::core::ffi::c_char);
        }
        13 => {
            fprintf(out, b"SYSTEM \0" as *const u8 as *const ::core::ffi::c_char);
        }
        20 => {
            fprintf(
                out,
                b"DELEGATE \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        21 => {
            fprintf(out, b"BASE \0" as *const u8 as *const ::core::ffi::c_char);
        }
        22 => {
            fprintf(
                out,
                b"CATALOG \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        23 => {
            fprintf(
                out,
                b"DOCUMENT \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        24 => {
            fprintf(
                out,
                b"SGMLDECL \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        _ => return,
    }
    match (*entry).type_0 as ::core::ffi::c_int {
        15 | 16 | 17 | 18 | 19 => {
            fprintf(
                out,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*entry).name as *const ::core::ffi::c_char,
            );
        }
        14 | 13 | 24 | 23 | 22 | 21 | 20 => {
            fprintf(
                out,
                b"\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                (*entry).name,
            );
        }
        _ => {}
    }
    match (*entry).type_0 as ::core::ffi::c_int {
        15 | 16 | 17 | 18 | 19 | 14 | 13 | 20 => {
            fprintf(
                out,
                b" \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                (*entry).value,
            );
        }
        _ => {}
    }
    fprintf(out, b"\n\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn xmlDumpXMLCatalogNode(
    mut catal: xmlCatalogEntryPtr,
    mut catalog: xmlNodePtr,
    mut doc: xmlDocPtr,
    mut ns: xmlNsPtr,
    mut cgroup: xmlCatalogEntryPtr,
) {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut cur: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    cur = catal;
    let mut current_block_49: u64;
    while !cur.is_null() {
        if (*cur).group == cgroup {
            match (*cur).type_0 as ::core::ffi::c_int {
                2 | 1 => {
                    current_block_49 = 210332235020735180;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                3 => {
                    current_block_49 = 14220523932482161753;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                4 => {
                    current_block_49 = 1822579226956249074;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                5 => {
                    current_block_49 = 17553960669368917637;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                6 => {
                    current_block_49 = 17836174079017535235;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                7 => {
                    current_block_49 = 7204805488150849096;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                8 => {
                    current_block_49 = 35032888785256685;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                9 => {
                    current_block_49 = 15193445391710277469;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                10 => {
                    current_block_49 = 27432119168115066;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                11 => {
                    current_block_49 = 16684841253143231238;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                12 => {
                    current_block_49 = 15735151238258917980;
                    match current_block_49 {
                        210332235020735180 => {
                            if cur == catal {
                                cur = (*cur).children as xmlCatalogEntryPtr;
                                continue;
                            }
                        }
                        16684841253143231238 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        27432119168115066 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"name\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        15193445391710277469 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        35032888785256685 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        7204805488150849096 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17836174079017535235 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"system\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"systemId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        17553960669368917637 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"public\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"publicId\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        1822579226956249074 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"group\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                                (*cur).name,
                            );
                            if !(*cur).value.is_null() {
                                let mut xns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                                xns = xmlSearchNsByHref(doc, node, XML_XML_NAMESPACE);
                                if !xns.is_null() {
                                    xmlSetNsProp(
                                        node,
                                        xns,
                                        b"base\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        (*cur).value,
                                    );
                                }
                            }
                            match (*cur).prefer as ::core::ffi::c_uint {
                                1 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"public\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                2 => {
                                    xmlSetProp(
                                        node,
                                        b"prefer\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                        b"system\0" as *const u8 as *const ::core::ffi::c_char
                                            as *mut xmlChar,
                                    );
                                }
                                0 | _ => {}
                            }
                            xmlDumpXMLCatalogNode(
                                (*cur).next as xmlCatalogEntryPtr,
                                node,
                                doc,
                                ns,
                                cur,
                            );
                            xmlAddChild(catalog, node);
                        }
                        14220523932482161753 => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                        _ => {
                            node = xmlNewDocNode(
                                doc,
                                ns,
                                b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            xmlSetProp(
                                node,
                                b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).name,
                            );
                            xmlSetProp(
                                node,
                                b"catalog\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                (*cur).value,
                            );
                            xmlAddChild(catalog, node);
                        }
                    }
                }
                -1 | 0 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | _ => {}
            }
        }
        cur = (*cur).next as xmlCatalogEntryPtr;
    }
}
unsafe extern "C" fn xmlDumpXMLCatalog(
    mut out: *mut FILE,
    mut catal: xmlCatalogEntryPtr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut dtd: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut catalog: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    doc = xmlNewDoc(::core::ptr::null::<xmlChar>());
    if doc.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    dtd = xmlNewDtd(
        doc,
        b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        b"-//OASIS//DTD Entity Resolution XML Catalog V1.0//EN\0" as *const u8
            as *const ::core::ffi::c_char as *mut xmlChar,
        b"http://www.oasis-open.org/committees/entity/release/1.0/catalog.dtd\0" as *const u8
            as *const ::core::ffi::c_char as *mut xmlChar,
    );
    xmlAddChild(doc as xmlNodePtr, dtd as xmlNodePtr);
    ns = xmlNewNs(
        ::core::ptr::null_mut::<xmlNode>(),
        XML_CATALOGS_NAMESPACE,
        ::core::ptr::null::<xmlChar>(),
    );
    if ns.is_null() {
        xmlFreeDoc(doc);
        return -(1 as ::core::ffi::c_int);
    }
    catalog = xmlNewDocNode(
        doc,
        ns,
        b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ::core::ptr::null::<xmlChar>(),
    );
    if catalog.is_null() {
        xmlFreeNs(ns);
        xmlFreeDoc(doc);
        return -(1 as ::core::ffi::c_int);
    }
    (*catalog).nsDef = ns as *mut xmlNs;
    xmlAddChild(doc as xmlNodePtr, catalog);
    xmlDumpXMLCatalogNode(
        catal,
        catalog,
        doc,
        ns,
        ::core::ptr::null_mut::<xmlCatalogEntry>(),
    );
    buf = xmlOutputBufferCreateFile(out, ::core::ptr::null_mut::<xmlCharEncodingHandler>());
    if buf.is_null() {
        xmlFreeDoc(doc);
        return -(1 as ::core::ffi::c_int);
    }
    ret = xmlSaveFormatFileTo(
        buf,
        doc,
        ::core::ptr::null::<::core::ffi::c_char>(),
        1 as ::core::ffi::c_int,
    );
    xmlFreeDoc(doc);
    return ret;
}
unsafe extern "C" fn xmlCatalogConvertEntry(
    mut payload: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
    mut name: *const xmlChar,
) {
    let mut entry: xmlCatalogEntryPtr = payload as xmlCatalogEntryPtr;
    let mut catal: xmlCatalogPtr = data as xmlCatalogPtr;
    if entry.is_null() || catal.is_null() || (*catal).sgml.is_null() || (*catal).xml.is_null() {
        return;
    }
    match (*entry).type_0 as ::core::ffi::c_int {
        15 => {
            (*entry).type_0 = XML_CATA_PUBLIC;
        }
        16 => {
            (*entry).type_0 = XML_CATA_PUBLIC;
        }
        17 => {
            (*entry).type_0 = XML_CATA_PUBLIC;
        }
        18 => {
            (*entry).type_0 = XML_CATA_PUBLIC;
        }
        19 => {
            (*entry).type_0 = XML_CATA_PUBLIC;
        }
        14 => {
            (*entry).type_0 = XML_CATA_PUBLIC;
        }
        13 => {
            (*entry).type_0 = XML_CATA_SYSTEM;
        }
        20 => {
            (*entry).type_0 = XML_CATA_DELEGATE_PUBLIC;
        }
        22 => {
            (*entry).type_0 = XML_CATA_CATALOG;
        }
        _ => {
            xmlHashRemoveEntry(
                (*catal).sgml,
                (*entry).name,
                Some(
                    xmlFreeCatalogEntry
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
                ),
            );
            return;
        }
    }
    xmlHashRemoveEntry((*catal).sgml, (*entry).name, None);
    (*entry).parent = (*catal).xml as *mut _xmlCatalogEntry;
    (*entry).next = ::core::ptr::null_mut::<_xmlCatalogEntry>();
    if (*(*catal).xml).children.is_null() {
        (*(*catal).xml).children = entry as *mut _xmlCatalogEntry;
    } else {
        let mut prev: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
        prev = (*(*catal).xml).children as xmlCatalogEntryPtr;
        while !(*prev).next.is_null() {
            prev = (*prev).next as xmlCatalogEntryPtr;
        }
        (*prev).next = entry as *mut _xmlCatalogEntry;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlConvertSGMLCatalog(mut catal: xmlCatalogPtr) -> ::core::ffi::c_int {
    if catal.is_null()
        || (*catal).type_0 as ::core::ffi::c_uint
            != XML_SGML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Converting SGML catalog to XML\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    xmlHashScan(
        (*catal).sgml,
        Some(
            xmlCatalogConvertEntry
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                ) -> (),
        ),
        &raw mut catal as *mut ::core::ffi::c_void,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlCatalogUnWrapURN(mut urn: *const xmlChar) -> *mut xmlChar {
    let mut result: [xmlChar; 2000] = [0; 2000];
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if xmlStrncmp(
        urn,
        XML_URN_PUBID.as_ptr() as *mut xmlChar,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    ) != 0
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    urn = urn.offset(
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as usize).wrapping_sub(1 as usize)
            as isize,
    );
    while *urn as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if i as usize
            > (::core::mem::size_of::<[xmlChar; 2000]>() as usize).wrapping_sub(4 as usize)
        {
            break;
        }
        if *urn as ::core::ffi::c_int == '+' as i32 {
            let fresh6 = i;
            i = i.wrapping_add(1);
            result[fresh6 as usize] = ' ' as i32 as xmlChar;
            urn = urn.offset(1);
        } else if *urn as ::core::ffi::c_int == ':' as i32 {
            let fresh7 = i;
            i = i.wrapping_add(1);
            result[fresh7 as usize] = '/' as i32 as xmlChar;
            let fresh8 = i;
            i = i.wrapping_add(1);
            result[fresh8 as usize] = '/' as i32 as xmlChar;
            urn = urn.offset(1);
        } else if *urn as ::core::ffi::c_int == ';' as i32 {
            let fresh9 = i;
            i = i.wrapping_add(1);
            result[fresh9 as usize] = ':' as i32 as xmlChar;
            let fresh10 = i;
            i = i.wrapping_add(1);
            result[fresh10 as usize] = ':' as i32 as xmlChar;
            urn = urn.offset(1);
        } else if *urn as ::core::ffi::c_int == '%' as i32 {
            if *urn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '2' as i32
                && *urn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'B' as i32
            {
                let fresh11 = i;
                i = i.wrapping_add(1);
                result[fresh11 as usize] = '+' as i32 as xmlChar;
            } else if *urn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '3' as i32
                && *urn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'A' as i32
            {
                let fresh12 = i;
                i = i.wrapping_add(1);
                result[fresh12 as usize] = ':' as i32 as xmlChar;
            } else if *urn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '2' as i32
                && *urn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'F' as i32
            {
                let fresh13 = i;
                i = i.wrapping_add(1);
                result[fresh13 as usize] = '/' as i32 as xmlChar;
            } else if *urn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '3' as i32
                && *urn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'B' as i32
            {
                let fresh14 = i;
                i = i.wrapping_add(1);
                result[fresh14 as usize] = ';' as i32 as xmlChar;
            } else if *urn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '2' as i32
                && *urn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '7' as i32
            {
                let fresh15 = i;
                i = i.wrapping_add(1);
                result[fresh15 as usize] = '\'' as i32 as xmlChar;
            } else if *urn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '3' as i32
                && *urn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'F' as i32
            {
                let fresh16 = i;
                i = i.wrapping_add(1);
                result[fresh16 as usize] = '?' as i32 as xmlChar;
            } else if *urn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '2' as i32
                && *urn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '3' as i32
            {
                let fresh17 = i;
                i = i.wrapping_add(1);
                result[fresh17 as usize] = '#' as i32 as xmlChar;
            } else if *urn.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '2' as i32
                && *urn.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '5' as i32
            {
                let fresh18 = i;
                i = i.wrapping_add(1);
                result[fresh18 as usize] = '%' as i32 as xmlChar;
            } else {
                let fresh19 = i;
                i = i.wrapping_add(1);
                result[fresh19 as usize] = *urn;
                urn = urn.offset(1);
                continue;
            }
            urn = urn.offset(3 as ::core::ffi::c_int as isize);
        } else {
            let fresh20 = i;
            i = i.wrapping_add(1);
            result[fresh20 as usize] = *urn;
            urn = urn.offset(1);
        }
    }
    result[i as usize] = 0 as xmlChar;
    return xmlStrdup(&raw mut result as *mut xmlChar);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseCatalogFile(
    mut filename: *const ::core::ffi::c_char,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut directory: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        xmlCatalogErrMemory(
            b"allocating parser context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    buf = xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputStream = xmlNewInputStream(ctxt);
    if inputStream.is_null() {
        xmlFreeParserInputBuffer(buf);
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    (*inputStream).filename =
        xmlCanonicPath(filename as *const xmlChar) as *mut ::core::ffi::c_char;
    (*inputStream).buf = buf;
    xmlBufResetInput((*buf).buffer, inputStream);
    inputPush(ctxt, inputStream);
    if (*ctxt).directory.is_null() {
        directory = xmlParserGetDirectory(filename);
    }
    if (*ctxt).directory.is_null() && !directory.is_null() {
        (*ctxt).directory = directory;
    }
    (*ctxt).valid = 0 as ::core::ffi::c_int;
    (*ctxt).validate = 0 as ::core::ffi::c_int;
    (*ctxt).loadsubset = 0 as ::core::ffi::c_int;
    (*ctxt).pedantic = 0 as ::core::ffi::c_int;
    (*ctxt).dictNames = 1 as ::core::ffi::c_int;
    xmlParseDocument(ctxt);
    if (*ctxt).wellFormed != 0 {
        ret = (*ctxt).myDoc;
    } else {
        ret = ::core::ptr::null_mut::<xmlDoc>();
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
unsafe extern "C" fn xmlLoadFileContent(mut filename: *const ::core::ffi::c_char) -> *mut xmlChar {
    let mut fd: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_long = 0;
    let mut info: stat = stat {
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
    let mut content: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if filename.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if stat(filename, &raw mut info) < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    fd = open(filename, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    size = info.st_size as ::core::ffi::c_long;
    content = xmlMallocAtomic.expect("non-null function pointer")(
        (size + 10 as ::core::ffi::c_long) as size_t,
    ) as *mut xmlChar;
    if content.is_null() {
        xmlCatalogErrMemory(
            b"allocating catalog data\0" as *const u8 as *const ::core::ffi::c_char,
        );
        close(fd);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    len = read(fd, content as *mut ::core::ffi::c_void, size as size_t) as ::core::ffi::c_int;
    close(fd);
    if len < 0 as ::core::ffi::c_int {
        xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    *content.offset(len as isize) = 0 as xmlChar;
    return content;
}
unsafe extern "C" fn xmlCatalogNormalizePublic(mut pubID: *const xmlChar) -> *mut xmlChar {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut white: ::core::ffi::c_int = 0;
    let mut p: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut q: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if pubID.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    white = 1 as ::core::ffi::c_int;
    p = pubID;
    while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int && ok != 0 {
        if !(*p as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *p as ::core::ffi::c_int
                && *p as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
        {
            white = 0 as ::core::ffi::c_int;
        } else if *p as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int && white == 0 {
            white = 1 as ::core::ffi::c_int;
        } else {
            ok = 0 as ::core::ffi::c_int;
        }
        p = p.offset(1);
    }
    if ok != 0 && white == 0 {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlStrdup(pubID);
    q = ret;
    white = 0 as ::core::ffi::c_int;
    p = pubID;
    while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *p as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *p as ::core::ffi::c_int
                && *p as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
        {
            if q != ret {
                white = 1 as ::core::ffi::c_int;
            }
        } else {
            if white != 0 {
                let fresh0 = q;
                q = q.offset(1);
                *fresh0 = 0x20 as xmlChar;
                white = 0 as ::core::ffi::c_int;
            }
            let fresh1 = q;
            q = q.offset(1);
            *fresh1 = *p;
        }
        p = p.offset(1);
    }
    *q = 0 as xmlChar;
    return ret;
}
unsafe extern "C" fn xmlGetXMLCatalogEntryType(mut name: *const xmlChar) -> xmlCatalogEntryType {
    let mut type_0: xmlCatalogEntryType = XML_CATA_NONE;
    if xmlStrEqual(
        name,
        b"system\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_SYSTEM;
    } else if xmlStrEqual(
        name,
        b"public\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_PUBLIC;
    } else if xmlStrEqual(
        name,
        b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_REWRITE_SYSTEM;
    } else if xmlStrEqual(
        name,
        b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_DELEGATE_PUBLIC;
    } else if xmlStrEqual(
        name,
        b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_DELEGATE_SYSTEM;
    } else if xmlStrEqual(
        name,
        b"uri\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_URI;
    } else if xmlStrEqual(
        name,
        b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_REWRITE_URI;
    } else if xmlStrEqual(
        name,
        b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_DELEGATE_URI;
    } else if xmlStrEqual(
        name,
        b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_NEXT_CATALOG;
    } else if xmlStrEqual(
        name,
        b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = XML_CATA_CATALOG;
    }
    return type_0;
}
unsafe extern "C" fn xmlParseXMLCatalogOneNode(
    mut cur: xmlNodePtr,
    mut type_0: xmlCatalogEntryType,
    mut name: *const xmlChar,
    mut attrName: *const xmlChar,
    mut uriAttrName: *const xmlChar,
    mut prefer: xmlCatalogPrefer,
    mut cgroup: xmlCatalogEntryPtr,
) -> xmlCatalogEntryPtr {
    let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut uriValue: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut nameValue: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut base: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut URL: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    if !attrName.is_null() {
        nameValue = xmlGetProp(cur as *const xmlNode, attrName);
        if nameValue.is_null() {
            xmlCatalogErr(
                ret,
                cur,
                XML_CATALOG_MISSING_ATTR as ::core::ffi::c_int,
                b"%s entry lacks '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
                attrName,
                ::core::ptr::null::<xmlChar>(),
            );
            ok = 0 as ::core::ffi::c_int;
        }
    }
    uriValue = xmlGetProp(cur as *const xmlNode, uriAttrName);
    if uriValue.is_null() {
        xmlCatalogErr(
            ret,
            cur,
            XML_CATALOG_MISSING_ATTR as ::core::ffi::c_int,
            b"%s entry lacks '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
            uriAttrName,
            ::core::ptr::null::<xmlChar>(),
        );
        ok = 0 as ::core::ffi::c_int;
    }
    if ok == 0 {
        if !nameValue.is_null() {
            xmlFree.expect("non-null function pointer")(nameValue as *mut ::core::ffi::c_void);
        }
        if !uriValue.is_null() {
            xmlFree.expect("non-null function pointer")(uriValue as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<xmlCatalogEntry>();
    }
    base = xmlNodeGetBase((*cur).doc, cur as *const xmlNode);
    URL = xmlBuildURI(uriValue, base);
    if !URL.is_null() {
        if xmlDebugCatalogs > 1 as ::core::ffi::c_int {
            if !nameValue.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Found %s: '%s' '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                    name,
                    nameValue,
                    URL,
                );
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Found %s: '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                    name,
                    URL,
                );
            }
        }
        ret = xmlNewCatalogEntry(type_0, nameValue, uriValue, URL, prefer, cgroup);
    } else {
        xmlCatalogErr(
            ret,
            cur,
            XML_CATALOG_ENTRY_BROKEN as ::core::ffi::c_int,
            b"%s entry '%s' broken ?: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
            uriAttrName,
            uriValue,
        );
    }
    if !nameValue.is_null() {
        xmlFree.expect("non-null function pointer")(nameValue as *mut ::core::ffi::c_void);
    }
    if !uriValue.is_null() {
        xmlFree.expect("non-null function pointer")(uriValue as *mut ::core::ffi::c_void);
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as *mut ::core::ffi::c_void);
    }
    if !URL.is_null() {
        xmlFree.expect("non-null function pointer")(URL as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn xmlParseXMLCatalogNode(
    mut cur: xmlNodePtr,
    mut prefer: xmlCatalogPrefer,
    mut parent: xmlCatalogEntryPtr,
    mut cgroup: xmlCatalogEntryPtr,
) {
    let mut base: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut entry: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    if cur.is_null() {
        return;
    }
    if xmlStrEqual(
        (*cur).name,
        b"group\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        let mut prop: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut pref: xmlCatalogPrefer = XML_CATA_PREFER_NONE;
        prop = xmlGetProp(
            cur as *const xmlNode,
            b"prefer\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        );
        if !prop.is_null() {
            if xmlStrEqual(
                prop,
                b"system\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
            {
                prefer = XML_CATA_PREFER_SYSTEM;
            } else if xmlStrEqual(
                prop,
                b"public\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
            {
                prefer = XML_CATA_PREFER_PUBLIC;
            } else {
                xmlCatalogErr(
                    parent,
                    cur,
                    XML_CATALOG_PREFER_VALUE as ::core::ffi::c_int,
                    b"Invalid value for prefer: '%s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    prop,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            xmlFree.expect("non-null function pointer")(prop as *mut ::core::ffi::c_void);
            pref = prefer;
        }
        prop = xmlGetProp(
            cur as *const xmlNode,
            b"id\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        );
        base = xmlGetNsProp(
            cur as *const xmlNode,
            b"base\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            XML_XML_NAMESPACE,
        );
        entry = xmlNewCatalogEntry(
            XML_CATA_GROUP,
            prop,
            base,
            ::core::ptr::null::<xmlChar>(),
            pref,
            cgroup,
        );
        xmlFree.expect("non-null function pointer")(prop as *mut ::core::ffi::c_void);
    } else if xmlStrEqual(
        (*cur).name,
        b"public\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_PUBLIC,
            b"public\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"publicId\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    } else if xmlStrEqual(
        (*cur).name,
        b"system\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_SYSTEM,
            b"system\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"systemId\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    } else if xmlStrEqual(
        (*cur).name,
        b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_REWRITE_SYSTEM,
            b"rewriteSystem\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    } else if xmlStrEqual(
        (*cur).name,
        b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_DELEGATE_PUBLIC,
            b"delegatePublic\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"publicIdStartString\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    } else if xmlStrEqual(
        (*cur).name,
        b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_DELEGATE_SYSTEM,
            b"delegateSystem\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"systemIdStartString\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    } else if xmlStrEqual(
        (*cur).name,
        b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_URI,
            b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"name\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"uri\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    } else if xmlStrEqual(
        (*cur).name,
        b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_REWRITE_URI,
            b"rewriteURI\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"rewritePrefix\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    } else if xmlStrEqual(
        (*cur).name,
        b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_DELEGATE_URI,
            b"delegateURI\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"uriStartString\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    } else if xmlStrEqual(
        (*cur).name,
        b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        entry = xmlParseXMLCatalogOneNode(
            cur,
            XML_CATA_NEXT_CATALOG,
            b"nextCatalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ::core::ptr::null::<xmlChar>(),
            b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            prefer,
            cgroup,
        );
    }
    if !entry.is_null() {
        if !parent.is_null() {
            (*entry).parent = parent as *mut _xmlCatalogEntry;
            if (*parent).children.is_null() {
                (*parent).children = entry as *mut _xmlCatalogEntry;
            } else {
                let mut prev: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
                prev = (*parent).children as xmlCatalogEntryPtr;
                while !(*prev).next.is_null() {
                    prev = (*prev).next as xmlCatalogEntryPtr;
                }
                (*prev).next = entry as *mut _xmlCatalogEntry;
            }
        }
        if (*entry).type_0 as ::core::ffi::c_int == XML_CATA_GROUP as ::core::ffi::c_int {
            xmlParseXMLCatalogNodeList((*cur).children as xmlNodePtr, prefer, parent, entry);
        }
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn xmlParseXMLCatalogNodeList(
    mut cur: xmlNodePtr,
    mut prefer: xmlCatalogPrefer,
    mut parent: xmlCatalogEntryPtr,
    mut cgroup: xmlCatalogEntryPtr,
) {
    while !cur.is_null() {
        if !(*cur).ns.is_null()
            && !(*(*cur).ns).href.is_null()
            && xmlStrEqual((*(*cur).ns).href, XML_CATALOGS_NAMESPACE) != 0
        {
            xmlParseXMLCatalogNode(cur, prefer, parent, cgroup);
        }
        cur = (*cur).next as xmlNodePtr;
    }
}
unsafe extern "C" fn xmlParseXMLCatalogFile(
    mut prefer: xmlCatalogPrefer,
    mut filename: *const xmlChar,
) -> xmlCatalogEntryPtr {
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut prop: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut parent: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    if filename.is_null() {
        return ::core::ptr::null_mut::<xmlCatalogEntry>();
    }
    doc = xmlParseCatalogFile(filename as *const ::core::ffi::c_char);
    if doc.is_null() {
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Failed to parse catalog %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
        }
        return ::core::ptr::null_mut::<xmlCatalogEntry>();
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"%d Parsing catalog %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            xmlGetThreadId(),
            filename,
        );
    }
    cur = xmlDocGetRootElement(doc as *const xmlDoc);
    if !cur.is_null()
        && xmlStrEqual(
            (*cur).name,
            b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ) != 0
        && !(*cur).ns.is_null()
        && !(*(*cur).ns).href.is_null()
        && xmlStrEqual((*(*cur).ns).href, XML_CATALOGS_NAMESPACE) != 0
    {
        parent = xmlNewCatalogEntry(
            XML_CATA_CATALOG,
            ::core::ptr::null::<xmlChar>(),
            filename,
            ::core::ptr::null::<xmlChar>(),
            prefer,
            ::core::ptr::null_mut::<xmlCatalogEntry>(),
        );
        if parent.is_null() {
            xmlFreeDoc(doc);
            return ::core::ptr::null_mut::<xmlCatalogEntry>();
        }
        prop = xmlGetProp(
            cur as *const xmlNode,
            b"prefer\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        );
        if !prop.is_null() {
            if xmlStrEqual(
                prop,
                b"system\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
            {
                prefer = XML_CATA_PREFER_SYSTEM;
            } else if xmlStrEqual(
                prop,
                b"public\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
            {
                prefer = XML_CATA_PREFER_PUBLIC;
            } else {
                xmlCatalogErr(
                    ::core::ptr::null_mut::<xmlCatalogEntry>(),
                    cur,
                    XML_CATALOG_PREFER_VALUE as ::core::ffi::c_int,
                    b"Invalid value for prefer: '%s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    prop,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            xmlFree.expect("non-null function pointer")(prop as *mut ::core::ffi::c_void);
        }
        cur = (*cur).children as xmlNodePtr;
        xmlParseXMLCatalogNodeList(
            cur,
            prefer,
            parent,
            ::core::ptr::null_mut::<xmlCatalogEntry>(),
        );
    } else {
        xmlCatalogErr(
            ::core::ptr::null_mut::<xmlCatalogEntry>(),
            doc as xmlNodePtr,
            XML_CATALOG_NOT_CATALOG as ::core::ffi::c_int,
            b"File %s is not an XML Catalog\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        xmlFreeDoc(doc);
        return ::core::ptr::null_mut::<xmlCatalogEntry>();
    }
    xmlFreeDoc(doc);
    return parent;
}
unsafe extern "C" fn xmlFetchXMLCatalogFile(mut catal: xmlCatalogEntryPtr) -> ::core::ffi::c_int {
    let mut doc: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    if catal.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*catal).URL.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    xmlRMutexLock(xmlCatalogMutex);
    if !(*catal).children.is_null() {
        xmlRMutexUnlock(xmlCatalogMutex);
        return 0 as ::core::ffi::c_int;
    }
    if !xmlCatalogXMLFiles.is_null() {
        doc = xmlHashLookup(xmlCatalogXMLFiles, (*catal).URL) as xmlCatalogEntryPtr;
        if !doc.is_null() {
            if xmlDebugCatalogs != 0 {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Found %s in file hash\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*catal).URL,
                );
            }
            if (*catal).type_0 as ::core::ffi::c_int == XML_CATA_CATALOG as ::core::ffi::c_int {
                (*catal).children = (*doc).children;
            } else {
                (*catal).children = doc as *mut _xmlCatalogEntry;
            }
            (*catal).dealloc = 0 as ::core::ffi::c_int;
            xmlRMutexUnlock(xmlCatalogMutex);
            return 0 as ::core::ffi::c_int;
        }
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"%s not found in file hash\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*catal).URL,
            );
        }
    }
    doc = xmlParseXMLCatalogFile((*catal).prefer, (*catal).URL);
    if doc.is_null() {
        (*catal).type_0 = XML_CATA_BROKEN_CATALOG;
        xmlRMutexUnlock(xmlCatalogMutex);
        return -(1 as ::core::ffi::c_int);
    }
    if (*catal).type_0 as ::core::ffi::c_int == XML_CATA_CATALOG as ::core::ffi::c_int {
        (*catal).children = (*doc).children;
    } else {
        (*catal).children = doc as *mut _xmlCatalogEntry;
    }
    (*doc).dealloc = 1 as ::core::ffi::c_int;
    if xmlCatalogXMLFiles.is_null() {
        xmlCatalogXMLFiles = xmlHashCreate(10 as ::core::ffi::c_int);
    }
    if !xmlCatalogXMLFiles.is_null() {
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"%s added to file hash\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*catal).URL,
            );
        }
        xmlHashAddEntry(
            xmlCatalogXMLFiles,
            (*catal).URL,
            doc as *mut ::core::ffi::c_void,
        );
    }
    xmlRMutexUnlock(xmlCatalogMutex);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlAddXMLCatalog(
    mut catal: xmlCatalogEntryPtr,
    mut type_0: *const xmlChar,
    mut orig: *const xmlChar,
    mut replace: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut cur: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut typ: xmlCatalogEntryType = XML_CATA_NONE;
    let mut doregister: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if catal.is_null()
        || (*catal).type_0 as ::core::ffi::c_int != XML_CATA_CATALOG as ::core::ffi::c_int
            && (*catal).type_0 as ::core::ffi::c_int
                != XML_CATA_BROKEN_CATALOG as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*catal).children.is_null() {
        xmlFetchXMLCatalogFile(catal);
    }
    if (*catal).children.is_null() {
        doregister = 1 as ::core::ffi::c_int;
    }
    typ = xmlGetXMLCatalogEntryType(type_0);
    if typ as ::core::ffi::c_int == XML_CATA_NONE as ::core::ffi::c_int {
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Failed to add unknown element %s to catalog\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                type_0,
            );
        }
        return -(1 as ::core::ffi::c_int);
    }
    cur = (*catal).children as xmlCatalogEntryPtr;
    if !cur.is_null() {
        while !cur.is_null() {
            if !orig.is_null()
                && (*cur).type_0 as ::core::ffi::c_int == typ as ::core::ffi::c_int
                && xmlStrEqual(orig, (*cur).name) != 0
            {
                if xmlDebugCatalogs != 0 {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Updating element %s to catalog\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        type_0,
                    );
                }
                if !(*cur).value.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*cur).value as *mut ::core::ffi::c_void,
                    );
                }
                if !(*cur).URL.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*cur).URL as *mut ::core::ffi::c_void,
                    );
                }
                (*cur).value = xmlStrdup(replace);
                (*cur).URL = xmlStrdup(replace);
                return 0 as ::core::ffi::c_int;
            }
            if (*cur).next.is_null() {
                break;
            }
            cur = (*cur).next as xmlCatalogEntryPtr;
        }
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Adding element %s to catalog\n\0" as *const u8 as *const ::core::ffi::c_char,
            type_0,
        );
    }
    if cur.is_null() {
        (*catal).children = xmlNewCatalogEntry(
            typ,
            orig,
            replace,
            ::core::ptr::null::<xmlChar>(),
            (*catal).prefer,
            ::core::ptr::null_mut::<xmlCatalogEntry>(),
        ) as *mut _xmlCatalogEntry;
    } else {
        (*cur).next = xmlNewCatalogEntry(
            typ,
            orig,
            replace,
            ::core::ptr::null::<xmlChar>(),
            (*catal).prefer,
            ::core::ptr::null_mut::<xmlCatalogEntry>(),
        ) as *mut _xmlCatalogEntry;
    }
    if doregister != 0 {
        (*catal).type_0 = XML_CATA_CATALOG;
        cur = xmlHashLookup(xmlCatalogXMLFiles, (*catal).URL) as xmlCatalogEntryPtr;
        if !cur.is_null() {
            (*cur).children = (*catal).children;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlDelXMLCatalog(
    mut catal: xmlCatalogEntryPtr,
    mut value: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut cur: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if catal.is_null()
        || (*catal).type_0 as ::core::ffi::c_int != XML_CATA_CATALOG as ::core::ffi::c_int
            && (*catal).type_0 as ::core::ffi::c_int
                != XML_CATA_BROKEN_CATALOG as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if value.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*catal).children.is_null() {
        xmlFetchXMLCatalogFile(catal);
    }
    cur = (*catal).children as xmlCatalogEntryPtr;
    while !cur.is_null() {
        if !(*cur).name.is_null() && xmlStrEqual(value, (*cur).name) != 0
            || xmlStrEqual(value, (*cur).value) != 0
        {
            if xmlDebugCatalogs != 0 {
                if !(*cur).name.is_null() {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Removing element %s from catalog\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*cur).name,
                    );
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Removing element %s from catalog\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*cur).value,
                    );
                }
            }
            (*cur).type_0 = XML_CATA_REMOVED;
        }
        cur = (*cur).next as xmlCatalogEntryPtr;
    }
    return ret;
}
unsafe extern "C" fn xmlCatalogXMLResolve(
    mut catal: xmlCatalogEntryPtr,
    mut pubID: *const xmlChar,
    mut sysID: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut haveDelegate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut haveNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*catal).depth > MAX_CATAL_DEPTH {
        xmlCatalogErr(
            catal,
            ::core::ptr::null_mut::<xmlNode>(),
            XML_CATALOG_RECURSION as ::core::ffi::c_int,
            b"Detected recursion in catalog %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*catal).name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    (*catal).depth += 1;
    if !sysID.is_null() {
        let mut rewrite: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
        let mut lenrewrite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut len: ::core::ffi::c_int = 0;
        cur = catal;
        haveDelegate = 0 as ::core::ffi::c_int;
        while !cur.is_null() {
            match (*cur).type_0 as ::core::ffi::c_int {
                6 => {
                    if xmlStrEqual(sysID, (*cur).name) != 0 {
                        if xmlDebugCatalogs != 0 {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"Found system match %s, using %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*cur).name,
                                (*cur).URL,
                            );
                        }
                        (*catal).depth -= 1;
                        return xmlStrdup((*cur).URL);
                    }
                }
                7 => {
                    len = xmlStrlen((*cur).name);
                    if len > lenrewrite && xmlStrncmp(sysID, (*cur).name, len) == 0 {
                        lenrewrite = len;
                        rewrite = cur;
                    }
                }
                9 => {
                    if xmlStrncmp(sysID, (*cur).name, xmlStrlen((*cur).name)) == 0 {
                        haveDelegate += 1;
                    }
                }
                3 => {
                    haveNext += 1;
                }
                _ => {}
            }
            cur = (*cur).next as xmlCatalogEntryPtr;
        }
        if !rewrite.is_null() {
            if xmlDebugCatalogs != 0 {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Using rewriting rule %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*rewrite).name,
                );
            }
            ret = xmlStrdup((*rewrite).URL);
            if !ret.is_null() {
                ret = xmlStrcat(ret, sysID.offset(lenrewrite as isize) as *const xmlChar);
            }
            (*catal).depth -= 1;
            return ret;
        }
        if haveDelegate != 0 {
            let mut delegates: [*const xmlChar; 50] = [::core::ptr::null::<xmlChar>(); 50];
            let mut nbList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0;
            cur = catal;
            while !cur.is_null() {
                if (*cur).type_0 as ::core::ffi::c_int
                    == XML_CATA_DELEGATE_SYSTEM as ::core::ffi::c_int
                    && xmlStrncmp(sysID, (*cur).name, xmlStrlen((*cur).name)) == 0
                {
                    i = 0 as ::core::ffi::c_int;
                    while i < nbList {
                        if xmlStrEqual((*cur).URL, delegates[i as usize]) != 0 {
                            break;
                        }
                        i += 1;
                    }
                    if i < nbList {
                        cur = (*cur).next as xmlCatalogEntryPtr;
                        continue;
                    } else {
                        if nbList < MAX_DELEGATE {
                            let fresh4 = nbList;
                            nbList = nbList + 1;
                            delegates[fresh4 as usize] = (*cur).URL;
                        }
                        if (*cur).children.is_null() {
                            xmlFetchXMLCatalogFile(cur);
                        }
                        if !(*cur).children.is_null() {
                            if xmlDebugCatalogs != 0 {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"Trying system delegate %s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    (*cur).URL,
                                );
                            }
                            ret = xmlCatalogListXMLResolve(
                                (*cur).children as xmlCatalogEntryPtr,
                                ::core::ptr::null::<xmlChar>(),
                                sysID,
                            );
                            if !ret.is_null() {
                                (*catal).depth -= 1;
                                return ret;
                            }
                        }
                    }
                }
                cur = (*cur).next as xmlCatalogEntryPtr;
            }
            (*catal).depth -= 1;
            return -(1 as ::core::ffi::c_int) as *mut xmlChar;
        }
    }
    if !pubID.is_null() {
        cur = catal;
        haveDelegate = 0 as ::core::ffi::c_int;
        while !cur.is_null() {
            match (*cur).type_0 as ::core::ffi::c_int {
                5 => {
                    if xmlStrEqual(pubID, (*cur).name) != 0 {
                        if xmlDebugCatalogs != 0 {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"Found public match %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*cur).name,
                            );
                        }
                        (*catal).depth -= 1;
                        return xmlStrdup((*cur).URL);
                    }
                }
                8 => {
                    if xmlStrncmp(pubID, (*cur).name, xmlStrlen((*cur).name)) == 0
                        && (*cur).prefer as ::core::ffi::c_uint
                            == XML_CATA_PREFER_PUBLIC as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        haveDelegate += 1;
                    }
                }
                3 => {
                    if sysID.is_null() {
                        haveNext += 1;
                    }
                }
                _ => {}
            }
            cur = (*cur).next as xmlCatalogEntryPtr;
        }
        if haveDelegate != 0 {
            let mut delegates_0: [*const xmlChar; 50] = [::core::ptr::null::<xmlChar>(); 50];
            let mut nbList_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut i_0: ::core::ffi::c_int = 0;
            cur = catal;
            while !cur.is_null() {
                if (*cur).type_0 as ::core::ffi::c_int
                    == XML_CATA_DELEGATE_PUBLIC as ::core::ffi::c_int
                    && (*cur).prefer as ::core::ffi::c_uint
                        == XML_CATA_PREFER_PUBLIC as ::core::ffi::c_int as ::core::ffi::c_uint
                    && xmlStrncmp(pubID, (*cur).name, xmlStrlen((*cur).name)) == 0
                {
                    i_0 = 0 as ::core::ffi::c_int;
                    while i_0 < nbList_0 {
                        if xmlStrEqual((*cur).URL, delegates_0[i_0 as usize]) != 0 {
                            break;
                        }
                        i_0 += 1;
                    }
                    if i_0 < nbList_0 {
                        cur = (*cur).next as xmlCatalogEntryPtr;
                        continue;
                    } else {
                        if nbList_0 < MAX_DELEGATE {
                            let fresh5 = nbList_0;
                            nbList_0 = nbList_0 + 1;
                            delegates_0[fresh5 as usize] = (*cur).URL;
                        }
                        if (*cur).children.is_null() {
                            xmlFetchXMLCatalogFile(cur);
                        }
                        if !(*cur).children.is_null() {
                            if xmlDebugCatalogs != 0 {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"Trying public delegate %s\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    (*cur).URL,
                                );
                            }
                            ret = xmlCatalogListXMLResolve(
                                (*cur).children as xmlCatalogEntryPtr,
                                pubID,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            if !ret.is_null() {
                                (*catal).depth -= 1;
                                return ret;
                            }
                        }
                    }
                }
                cur = (*cur).next as xmlCatalogEntryPtr;
            }
            (*catal).depth -= 1;
            return -(1 as ::core::ffi::c_int) as *mut xmlChar;
        }
    }
    if haveNext != 0 {
        cur = catal;
        while !cur.is_null() {
            if (*cur).type_0 as ::core::ffi::c_int == XML_CATA_NEXT_CATALOG as ::core::ffi::c_int {
                if (*cur).children.is_null() {
                    xmlFetchXMLCatalogFile(cur);
                }
                if !(*cur).children.is_null() {
                    ret = xmlCatalogListXMLResolve(
                        (*cur).children as xmlCatalogEntryPtr,
                        pubID,
                        sysID,
                    );
                    if !ret.is_null() {
                        (*catal).depth -= 1;
                        return ret;
                    } else if (*catal).depth > MAX_CATAL_DEPTH {
                        return ::core::ptr::null_mut::<xmlChar>();
                    }
                }
            }
            cur = (*cur).next as xmlCatalogEntryPtr;
        }
    }
    (*catal).depth -= 1;
    return ::core::ptr::null_mut::<xmlChar>();
}
unsafe extern "C" fn xmlCatalogXMLResolveURI(
    mut catal: xmlCatalogEntryPtr,
    mut URI: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut haveDelegate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut haveNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rewrite: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut lenrewrite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = 0;
    if catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if URI.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*catal).depth > MAX_CATAL_DEPTH {
        xmlCatalogErr(
            catal,
            ::core::ptr::null_mut::<xmlNode>(),
            XML_CATALOG_RECURSION as ::core::ffi::c_int,
            b"Detected recursion in catalog %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*catal).name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    cur = catal;
    haveDelegate = 0 as ::core::ffi::c_int;
    while !cur.is_null() {
        match (*cur).type_0 as ::core::ffi::c_int {
            10 => {
                if xmlStrEqual(URI, (*cur).name) != 0 {
                    if xmlDebugCatalogs != 0 {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Found URI match %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                            (*cur).name,
                        );
                    }
                    return xmlStrdup((*cur).URL);
                }
            }
            11 => {
                len = xmlStrlen((*cur).name);
                if len > lenrewrite && xmlStrncmp(URI, (*cur).name, len) == 0 {
                    lenrewrite = len;
                    rewrite = cur;
                }
            }
            12 => {
                if xmlStrncmp(URI, (*cur).name, xmlStrlen((*cur).name)) == 0 {
                    haveDelegate += 1;
                }
            }
            3 => {
                haveNext += 1;
            }
            _ => {}
        }
        cur = (*cur).next as xmlCatalogEntryPtr;
    }
    if !rewrite.is_null() {
        if xmlDebugCatalogs != 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Using rewriting rule %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*rewrite).name,
            );
        }
        ret = xmlStrdup((*rewrite).URL);
        if !ret.is_null() {
            ret = xmlStrcat(ret, URI.offset(lenrewrite as isize) as *const xmlChar);
        }
        return ret;
    }
    if haveDelegate != 0 {
        let mut delegates: [*const xmlChar; 50] = [::core::ptr::null::<xmlChar>(); 50];
        let mut nbList: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        cur = catal;
        while !cur.is_null() {
            if ((*cur).type_0 as ::core::ffi::c_int
                == XML_CATA_DELEGATE_SYSTEM as ::core::ffi::c_int
                || (*cur).type_0 as ::core::ffi::c_int
                    == XML_CATA_DELEGATE_URI as ::core::ffi::c_int)
                && xmlStrncmp(URI, (*cur).name, xmlStrlen((*cur).name)) == 0
            {
                i = 0 as ::core::ffi::c_int;
                while i < nbList {
                    if xmlStrEqual((*cur).URL, delegates[i as usize]) != 0 {
                        break;
                    }
                    i += 1;
                }
                if i < nbList {
                    cur = (*cur).next as xmlCatalogEntryPtr;
                    continue;
                } else {
                    if nbList < MAX_DELEGATE {
                        let fresh21 = nbList;
                        nbList = nbList + 1;
                        delegates[fresh21 as usize] = (*cur).URL;
                    }
                    if (*cur).children.is_null() {
                        xmlFetchXMLCatalogFile(cur);
                    }
                    if !(*cur).children.is_null() {
                        if xmlDebugCatalogs != 0 {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"Trying URI delegate %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*cur).URL,
                            );
                        }
                        ret =
                            xmlCatalogListXMLResolveURI((*cur).children as xmlCatalogEntryPtr, URI);
                        if !ret.is_null() {
                            return ret;
                        }
                    }
                }
            }
            cur = (*cur).next as xmlCatalogEntryPtr;
        }
        return -(1 as ::core::ffi::c_int) as *mut xmlChar;
    }
    if haveNext != 0 {
        cur = catal;
        while !cur.is_null() {
            if (*cur).type_0 as ::core::ffi::c_int == XML_CATA_NEXT_CATALOG as ::core::ffi::c_int {
                if (*cur).children.is_null() {
                    xmlFetchXMLCatalogFile(cur);
                }
                if !(*cur).children.is_null() {
                    ret = xmlCatalogListXMLResolveURI((*cur).children as xmlCatalogEntryPtr, URI);
                    if !ret.is_null() {
                        return ret;
                    }
                }
            }
            cur = (*cur).next as xmlCatalogEntryPtr;
        }
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
unsafe extern "C" fn xmlCatalogListXMLResolve(
    mut catal: xmlCatalogEntryPtr,
    mut pubID: *const xmlChar,
    mut sysID: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut urnID: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut normid: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if pubID.is_null() && sysID.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    normid = xmlCatalogNormalizePublic(pubID);
    if !normid.is_null() {
        pubID = if *normid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            normid
        } else {
            ::core::ptr::null_mut::<xmlChar>()
        };
    }
    if xmlStrncmp(
        pubID,
        XML_URN_PUBID.as_ptr() as *mut xmlChar,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    ) == 0
    {
        urnID = xmlCatalogUnWrapURN(pubID);
        if xmlDebugCatalogs != 0 {
            if urnID.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Public URN ID %s expanded to NULL\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    pubID,
                );
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Public URN ID expanded to %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    urnID,
                );
            }
        }
        ret = xmlCatalogListXMLResolve(catal, urnID, sysID);
        if !urnID.is_null() {
            xmlFree.expect("non-null function pointer")(urnID as *mut ::core::ffi::c_void);
        }
        if !normid.is_null() {
            xmlFree.expect("non-null function pointer")(normid as *mut ::core::ffi::c_void);
        }
        return ret;
    }
    if xmlStrncmp(
        sysID,
        XML_URN_PUBID.as_ptr() as *mut xmlChar,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    ) == 0
    {
        urnID = xmlCatalogUnWrapURN(sysID);
        if xmlDebugCatalogs != 0 {
            if urnID.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"System URN ID %s expanded to NULL\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    sysID,
                );
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"System URN ID expanded to %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    urnID,
                );
            }
        }
        if pubID.is_null() {
            ret = xmlCatalogListXMLResolve(catal, urnID, ::core::ptr::null::<xmlChar>());
        } else if xmlStrEqual(pubID, urnID) != 0 {
            ret = xmlCatalogListXMLResolve(catal, pubID, ::core::ptr::null::<xmlChar>());
        } else {
            ret = xmlCatalogListXMLResolve(catal, pubID, urnID);
        }
        if !urnID.is_null() {
            xmlFree.expect("non-null function pointer")(urnID as *mut ::core::ffi::c_void);
        }
        if !normid.is_null() {
            xmlFree.expect("non-null function pointer")(normid as *mut ::core::ffi::c_void);
        }
        return ret;
    }
    while !catal.is_null() {
        if (*catal).type_0 as ::core::ffi::c_int == XML_CATA_CATALOG as ::core::ffi::c_int {
            if (*catal).children.is_null() {
                xmlFetchXMLCatalogFile(catal);
            }
            if !(*catal).children.is_null() {
                ret = xmlCatalogXMLResolve((*catal).children as xmlCatalogEntryPtr, pubID, sysID);
                if !ret.is_null() {
                    break;
                }
                if (*(*catal).children).depth > MAX_CATAL_DEPTH {
                    ret = ::core::ptr::null_mut::<xmlChar>();
                    break;
                }
            }
        }
        catal = (*catal).next as xmlCatalogEntryPtr;
    }
    if !normid.is_null() {
        xmlFree.expect("non-null function pointer")(normid as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn xmlCatalogListXMLResolveURI(
    mut catal: xmlCatalogEntryPtr,
    mut URI: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut urnID: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if URI.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlStrncmp(
        URI,
        XML_URN_PUBID.as_ptr() as *mut xmlChar,
        (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as usize).wrapping_sub(1 as usize)
            as ::core::ffi::c_int,
    ) == 0
    {
        urnID = xmlCatalogUnWrapURN(URI);
        if xmlDebugCatalogs != 0 {
            if urnID.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"URN ID %s expanded to NULL\n\0" as *const u8 as *const ::core::ffi::c_char,
                    URI,
                );
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"URN ID expanded to %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    urnID,
                );
            }
        }
        ret = xmlCatalogListXMLResolve(catal, urnID, ::core::ptr::null::<xmlChar>());
        if !urnID.is_null() {
            xmlFree.expect("non-null function pointer")(urnID as *mut ::core::ffi::c_void);
        }
        return ret;
    }
    while !catal.is_null() {
        if (*catal).type_0 as ::core::ffi::c_int == XML_CATA_CATALOG as ::core::ffi::c_int {
            if (*catal).children.is_null() {
                xmlFetchXMLCatalogFile(catal);
            }
            if !(*catal).children.is_null() {
                ret = xmlCatalogXMLResolveURI((*catal).children as xmlCatalogEntryPtr, URI);
                if !ret.is_null() {
                    return ret;
                }
            }
        }
        catal = (*catal).next as xmlCatalogEntryPtr;
    }
    return ret;
}
unsafe extern "C" fn xmlParseSGMLCatalogComment(mut cur: *const xmlChar) -> *const xmlChar {
    if *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32
        || *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32
    {
        return cur;
    }
    cur = cur.offset(2 as ::core::ffi::c_int as isize);
    while *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        && (*cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32
            || *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32)
    {
        cur = cur.offset(1);
    }
    if *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null::<xmlChar>();
    }
    return cur.offset(2 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn xmlParseSGMLCatalogPubid(
    mut cur: *const xmlChar,
    mut id: *mut *mut xmlChar,
) -> *const xmlChar {
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut stop: xmlChar = 0;
    *id = ::core::ptr::null_mut::<xmlChar>();
    if *cur as ::core::ffi::c_int == '"' as i32 {
        cur = cur.offset(1);
        stop = '"' as i32 as xmlChar;
    } else if *cur as ::core::ffi::c_int == '\'' as i32 {
        cur = cur.offset(1);
        stop = '\'' as i32 as xmlChar;
    } else {
        stop = ' ' as i32 as xmlChar;
    }
    buf = xmlMallocAtomic.expect("non-null function pointer")(size as size_t) as *mut xmlChar;
    if buf.is_null() {
        xmlCatalogErrMemory(b"allocating public ID\0" as *const u8 as *const ::core::ffi::c_char);
        return ::core::ptr::null::<xmlChar>();
    }
    while xmlIsPubidChar_tab[*cur as usize] as ::core::ffi::c_int != 0
        || *cur as ::core::ffi::c_int == '?' as i32
    {
        if *cur as ::core::ffi::c_int == stop as ::core::ffi::c_int
            && stop as ::core::ffi::c_int != ' ' as i32
        {
            break;
        }
        if stop as ::core::ffi::c_int == ' ' as i32
            && (*cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                    && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
        {
            break;
        }
        if len + 1 as ::core::ffi::c_int >= size {
            size *= 2 as ::core::ffi::c_int;
            tmp = xmlRealloc.expect("non-null function pointer")(
                buf as *mut ::core::ffi::c_void,
                size as size_t,
            ) as *mut xmlChar;
            if tmp.is_null() {
                xmlCatalogErrMemory(
                    b"allocating public ID\0" as *const u8 as *const ::core::ffi::c_char,
                );
                xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                return ::core::ptr::null::<xmlChar>();
            }
            buf = tmp;
        }
        let fresh2 = len;
        len = len + 1;
        *buf.offset(fresh2 as isize) = *cur;
        cur = cur.offset(1);
    }
    *buf.offset(len as isize) = 0 as xmlChar;
    if stop as ::core::ffi::c_int == ' ' as i32 {
        if !(*cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
        {
            xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
            return ::core::ptr::null::<xmlChar>();
        }
    } else {
        if *cur as ::core::ffi::c_int != stop as ::core::ffi::c_int {
            xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
            return ::core::ptr::null::<xmlChar>();
        }
        cur = cur.offset(1);
    }
    *id = buf;
    return cur;
}
unsafe extern "C" fn xmlParseSGMLCatalogName(
    mut cur: *const xmlChar,
    mut name: *mut *mut xmlChar,
) -> *const xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    *name = ::core::ptr::null_mut::<xmlChar>();
    c = *cur as ::core::ffi::c_int;
    if !((if c < 0x100 as ::core::ffi::c_int {
        (0x41 as ::core::ffi::c_int <= c && c <= 0x5a as ::core::ffi::c_int
            || 0x61 as ::core::ffi::c_int <= c && c <= 0x7a as ::core::ffi::c_int
            || 0xc0 as ::core::ffi::c_int <= c && c <= 0xd6 as ::core::ffi::c_int
            || 0xd8 as ::core::ffi::c_int <= c && c <= 0xf6 as ::core::ffi::c_int
            || 0xf8 as ::core::ffi::c_int <= c) as ::core::ffi::c_int
    } else {
        xmlCharInRange(c as ::core::ffi::c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            (0x4e00 as ::core::ffi::c_int <= c && c <= 0x9fa5 as ::core::ffi::c_int
                || c == 0x3007 as ::core::ffi::c_int
                || 0x3021 as ::core::ffi::c_int <= c && c <= 0x3029 as ::core::ffi::c_int)
                as ::core::ffi::c_int
        }) != 0)
        && c != '_' as i32
        && c != ':' as i32
    {
        return ::core::ptr::null::<xmlChar>();
    }
    while (if c < 0x100 as ::core::ffi::c_int {
        (0x41 as ::core::ffi::c_int <= c && c <= 0x5a as ::core::ffi::c_int
            || 0x61 as ::core::ffi::c_int <= c && c <= 0x7a as ::core::ffi::c_int
            || 0xc0 as ::core::ffi::c_int <= c && c <= 0xd6 as ::core::ffi::c_int
            || 0xd8 as ::core::ffi::c_int <= c && c <= 0xf6 as ::core::ffi::c_int
            || 0xf8 as ::core::ffi::c_int <= c) as ::core::ffi::c_int
    } else {
        xmlCharInRange(c as ::core::ffi::c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            (0x4e00 as ::core::ffi::c_int <= c && c <= 0x9fa5 as ::core::ffi::c_int
                || c == 0x3007 as ::core::ffi::c_int
                || 0x3021 as ::core::ffi::c_int <= c && c <= 0x3029 as ::core::ffi::c_int)
                as ::core::ffi::c_int
        }) != 0
        || (if c < 0x100 as ::core::ffi::c_int {
            (0x30 as ::core::ffi::c_int <= c && c <= 0x39 as ::core::ffi::c_int)
                as ::core::ffi::c_int
        } else {
            xmlCharInRange(c as ::core::ffi::c_uint, &raw const xmlIsDigitGroup)
        }) != 0
        || c == '.' as i32
        || c == '-' as i32
        || c == '_' as i32
        || c == ':' as i32
    {
        let fresh3 = len;
        len = len + 1;
        buf[fresh3 as usize] = c as xmlChar;
        cur = cur.offset(1);
        c = *cur as ::core::ffi::c_int;
        if len >= XML_MAX_NAMELEN {
            return ::core::ptr::null::<xmlChar>();
        }
    }
    *name = xmlStrndup(&raw mut buf as *mut xmlChar, len);
    return cur;
}
unsafe extern "C" fn xmlGetSGMLCatalogEntryType(mut name: *const xmlChar) -> xmlCatalogEntryType {
    let mut type_0: xmlCatalogEntryType = XML_CATA_NONE;
    if xmlStrEqual(
        name,
        b"SYSTEM\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_SYSTEM;
    } else if xmlStrEqual(
        name,
        b"PUBLIC\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_PUBLIC;
    } else if xmlStrEqual(
        name,
        b"DELEGATE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_DELEGATE;
    } else if xmlStrEqual(
        name,
        b"ENTITY\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_ENTITY;
    } else if xmlStrEqual(
        name,
        b"DOCTYPE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_DOCTYPE;
    } else if xmlStrEqual(
        name,
        b"LINKTYPE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_LINKTYPE;
    } else if xmlStrEqual(
        name,
        b"NOTATION\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_NOTATION;
    } else if xmlStrEqual(
        name,
        b"SGMLDECL\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_SGMLDECL;
    } else if xmlStrEqual(
        name,
        b"DOCUMENT\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_DOCUMENT;
    } else if xmlStrEqual(
        name,
        b"CATALOG\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_CATALOG;
    } else if xmlStrEqual(
        name,
        b"BASE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
    ) != 0
    {
        type_0 = SGML_CATA_BASE;
    }
    return type_0;
}
unsafe extern "C" fn xmlParseSGMLCatalog(
    mut catal: xmlCatalogPtr,
    mut value: *const xmlChar,
    mut file: *const ::core::ffi::c_char,
    mut super_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cur: *const xmlChar = value;
    let mut base: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut res: ::core::ffi::c_int = 0;
    if cur.is_null() || file.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    base = xmlStrdup(file as *const xmlChar);
    while !cur.is_null()
        && *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        while *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
        {
            cur = cur.offset(1);
        }
        if *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        if *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
            && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        {
            cur = xmlParseSGMLCatalogComment(cur);
            if cur.is_null() {
                break;
            }
        } else {
            let mut sysid: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut type_0: xmlCatalogEntryType = XML_CATA_NONE;
            cur = xmlParseSGMLCatalogName(cur, &raw mut name);
            if cur.is_null() || name.is_null() {
                break;
            }
            if !(*cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                    && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
            {
                xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
                break;
            } else {
                while *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                    || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                        && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                    || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
                {
                    cur = cur.offset(1);
                }
                if xmlStrEqual(
                    name,
                    b"SYSTEM\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_SYSTEM;
                } else if xmlStrEqual(
                    name,
                    b"PUBLIC\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_PUBLIC;
                } else if xmlStrEqual(
                    name,
                    b"DELEGATE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_DELEGATE;
                } else if xmlStrEqual(
                    name,
                    b"ENTITY\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_ENTITY;
                } else if xmlStrEqual(
                    name,
                    b"DOCTYPE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_DOCTYPE;
                } else if xmlStrEqual(
                    name,
                    b"LINKTYPE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_LINKTYPE;
                } else if xmlStrEqual(
                    name,
                    b"NOTATION\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_NOTATION;
                } else if xmlStrEqual(
                    name,
                    b"SGMLDECL\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_SGMLDECL;
                } else if xmlStrEqual(
                    name,
                    b"DOCUMENT\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_DOCUMENT;
                } else if xmlStrEqual(
                    name,
                    b"CATALOG\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_CATALOG;
                } else if xmlStrEqual(
                    name,
                    b"BASE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    type_0 = SGML_CATA_BASE;
                } else if xmlStrEqual(
                    name,
                    b"OVERRIDE\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
                {
                    xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
                    cur = xmlParseSGMLCatalogName(cur, &raw mut name);
                    if name.is_null() {
                        break;
                    }
                    xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
                    continue;
                }
                xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
                name = ::core::ptr::null_mut::<xmlChar>();
                let mut current_block_56: u64;
                match type_0 as ::core::ffi::c_int {
                    15 => {
                        if *cur as ::core::ffi::c_int == '%' as i32 {
                            type_0 = SGML_CATA_PENTITY;
                        }
                        current_block_56 = 5210836057962117608;
                    }
                    16 | 17 | 18 | 19 => {
                        current_block_56 = 5210836057962117608;
                    }
                    14 | 13 | 20 => {
                        cur = xmlParseSGMLCatalogPubid(cur, &raw mut name);
                        if cur.is_null() {
                            current_block_56 = 993425571616822999;
                        } else {
                            if type_0 as ::core::ffi::c_int
                                != SGML_CATA_SYSTEM as ::core::ffi::c_int
                            {
                                let mut normid: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                normid = xmlCatalogNormalizePublic(name);
                                if !normid.is_null() {
                                    if !name.is_null() {
                                        xmlFree.expect("non-null function pointer")(
                                            name as *mut ::core::ffi::c_void,
                                        );
                                    }
                                    if *normid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                        name = normid;
                                    } else {
                                        xmlFree.expect("non-null function pointer")(
                                            normid as *mut ::core::ffi::c_void,
                                        );
                                        name = ::core::ptr::null_mut::<xmlChar>();
                                    }
                                }
                            }
                            if !(*cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                                || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                                    && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                                || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
                            {
                                current_block_56 = 993425571616822999;
                            } else {
                                while *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                                    || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                                        && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                                    || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
                                {
                                    cur = cur.offset(1);
                                }
                                cur = xmlParseSGMLCatalogPubid(cur, &raw mut sysid);
                                cur.is_null();
                                current_block_56 = 993425571616822999;
                            }
                        }
                    }
                    21 | 22 | 23 | 24 => {
                        cur = xmlParseSGMLCatalogPubid(cur, &raw mut sysid);
                        cur.is_null();
                        current_block_56 = 993425571616822999;
                    }
                    _ => {
                        current_block_56 = 993425571616822999;
                    }
                }
                match current_block_56 {
                    5210836057962117608 => {
                        cur = xmlParseSGMLCatalogName(cur, &raw mut name);
                        if !cur.is_null() {
                            if *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                                || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                                    && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                                || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
                            {
                                while *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                                    || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                                        && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                                    || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
                                {
                                    cur = cur.offset(1);
                                }
                                cur = xmlParseSGMLCatalogPubid(cur, &raw mut sysid);
                                cur.is_null();
                            }
                        }
                    }
                    _ => {}
                }
                if cur.is_null() {
                    if !name.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            name as *mut ::core::ffi::c_void,
                        );
                    }
                    if !sysid.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            sysid as *mut ::core::ffi::c_void,
                        );
                    }
                    break;
                } else {
                    if type_0 as ::core::ffi::c_int == SGML_CATA_BASE as ::core::ffi::c_int {
                        if !base.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                base as *mut ::core::ffi::c_void,
                            );
                        }
                        base = xmlStrdup(sysid);
                    } else if type_0 as ::core::ffi::c_int == SGML_CATA_PUBLIC as ::core::ffi::c_int
                        || type_0 as ::core::ffi::c_int == SGML_CATA_SYSTEM as ::core::ffi::c_int
                    {
                        let mut filename: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        filename = xmlBuildURI(sysid, base);
                        if !filename.is_null() {
                            let mut entry: xmlCatalogEntryPtr =
                                ::core::ptr::null_mut::<xmlCatalogEntry>();
                            entry = xmlNewCatalogEntry(
                                type_0,
                                name,
                                filename,
                                ::core::ptr::null::<xmlChar>(),
                                XML_CATA_PREFER_NONE,
                                ::core::ptr::null_mut::<xmlCatalogEntry>(),
                            );
                            res = xmlHashAddEntry(
                                (*catal).sgml,
                                name,
                                entry as *mut ::core::ffi::c_void,
                            );
                            if res < 0 as ::core::ffi::c_int {
                                xmlFreeCatalogEntry(
                                    entry as *mut ::core::ffi::c_void,
                                    ::core::ptr::null::<xmlChar>(),
                                );
                            }
                            xmlFree.expect("non-null function pointer")(
                                filename as *mut ::core::ffi::c_void,
                            );
                        }
                    } else if type_0 as ::core::ffi::c_int
                        == SGML_CATA_CATALOG as ::core::ffi::c_int
                    {
                        if super_0 != 0 {
                            let mut entry_0: xmlCatalogEntryPtr =
                                ::core::ptr::null_mut::<xmlCatalogEntry>();
                            entry_0 = xmlNewCatalogEntry(
                                type_0,
                                sysid,
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                                XML_CATA_PREFER_NONE,
                                ::core::ptr::null_mut::<xmlCatalogEntry>(),
                            );
                            res = xmlHashAddEntry(
                                (*catal).sgml,
                                sysid,
                                entry_0 as *mut ::core::ffi::c_void,
                            );
                            if res < 0 as ::core::ffi::c_int {
                                xmlFreeCatalogEntry(
                                    entry_0 as *mut ::core::ffi::c_void,
                                    ::core::ptr::null::<xmlChar>(),
                                );
                            }
                        } else {
                            let mut filename_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                            filename_0 = xmlBuildURI(sysid, base);
                            if !filename_0.is_null() {
                                xmlExpandCatalog(catal, filename_0 as *const ::core::ffi::c_char);
                                xmlFree.expect("non-null function pointer")(
                                    filename_0 as *mut ::core::ffi::c_void,
                                );
                            }
                        }
                    }
                    if !name.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            name as *mut ::core::ffi::c_void,
                        );
                    }
                    if !sysid.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            sysid as *mut ::core::ffi::c_void,
                        );
                    }
                }
            }
        }
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as *mut ::core::ffi::c_void);
    }
    if cur.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlCatalogGetSGMLPublic(
    mut catal: xmlHashTablePtr,
    mut pubID: *const xmlChar,
) -> *const xmlChar {
    let mut entry: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut normid: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if catal.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    normid = xmlCatalogNormalizePublic(pubID);
    if !normid.is_null() {
        pubID = if *normid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            normid
        } else {
            ::core::ptr::null_mut::<xmlChar>()
        };
    }
    entry = xmlHashLookup(catal, pubID) as xmlCatalogEntryPtr;
    if entry.is_null() {
        if !normid.is_null() {
            xmlFree.expect("non-null function pointer")(normid as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null::<xmlChar>();
    }
    if (*entry).type_0 as ::core::ffi::c_int == SGML_CATA_PUBLIC as ::core::ffi::c_int {
        if !normid.is_null() {
            xmlFree.expect("non-null function pointer")(normid as *mut ::core::ffi::c_void);
        }
        return (*entry).URL;
    }
    if !normid.is_null() {
        xmlFree.expect("non-null function pointer")(normid as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null::<xmlChar>();
}
unsafe extern "C" fn xmlCatalogGetSGMLSystem(
    mut catal: xmlHashTablePtr,
    mut sysID: *const xmlChar,
) -> *const xmlChar {
    let mut entry: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    if catal.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    entry = xmlHashLookup(catal, sysID) as xmlCatalogEntryPtr;
    if entry.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if (*entry).type_0 as ::core::ffi::c_int == SGML_CATA_SYSTEM as ::core::ffi::c_int {
        return (*entry).URL;
    }
    return ::core::ptr::null::<xmlChar>();
}
unsafe extern "C" fn xmlCatalogSGMLResolve(
    mut catal: xmlCatalogPtr,
    mut pubID: *const xmlChar,
    mut sysID: *const xmlChar,
) -> *const xmlChar {
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if (*catal).sgml.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !pubID.is_null() {
        ret = xmlCatalogGetSGMLPublic((*catal).sgml, pubID);
    }
    if !ret.is_null() {
        return ret;
    }
    if !sysID.is_null() {
        ret = xmlCatalogGetSGMLSystem((*catal).sgml, sysID);
    }
    if !ret.is_null() {
        return ret;
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlLoadSGMLSuperCatalog(
    mut filename: *const ::core::ffi::c_char,
) -> xmlCatalogPtr {
    let mut content: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut catal: xmlCatalogPtr = ::core::ptr::null_mut::<xmlCatalog>();
    let mut ret: ::core::ffi::c_int = 0;
    content = xmlLoadFileContent(filename);
    if content.is_null() {
        return ::core::ptr::null_mut::<xmlCatalog>();
    }
    catal = xmlCreateNewCatalog(XML_SGML_CATALOG_TYPE, xmlCatalogDefaultPrefer);
    if catal.is_null() {
        xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlCatalog>();
    }
    ret = xmlParseSGMLCatalog(catal, content, filename, 1 as ::core::ffi::c_int);
    xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
    if ret < 0 as ::core::ffi::c_int {
        xmlFreeCatalog(catal);
        return ::core::ptr::null_mut::<xmlCatalog>();
    }
    return catal;
}
#[no_mangle]
pub unsafe extern "C" fn xmlLoadACatalog(
    mut filename: *const ::core::ffi::c_char,
) -> xmlCatalogPtr {
    let mut content: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut first: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut catal: xmlCatalogPtr = ::core::ptr::null_mut::<xmlCatalog>();
    let mut ret: ::core::ffi::c_int = 0;
    content = xmlLoadFileContent(filename);
    if content.is_null() {
        return ::core::ptr::null_mut::<xmlCatalog>();
    }
    first = content;
    while *first as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && *first as ::core::ffi::c_int != '-' as i32
        && *first as ::core::ffi::c_int != '<' as i32
        && !(*first as ::core::ffi::c_int >= 'A' as i32
            && *first as ::core::ffi::c_int <= 'Z' as i32
            || *first as ::core::ffi::c_int >= 'a' as i32
                && *first as ::core::ffi::c_int <= 'z' as i32)
    {
        first = first.offset(1);
    }
    if *first as ::core::ffi::c_int != '<' as i32 {
        catal = xmlCreateNewCatalog(XML_SGML_CATALOG_TYPE, xmlCatalogDefaultPrefer);
        if catal.is_null() {
            xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xmlCatalog>();
        }
        ret = xmlParseSGMLCatalog(catal, content, filename, 0 as ::core::ffi::c_int);
        if ret < 0 as ::core::ffi::c_int {
            xmlFreeCatalog(catal);
            xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xmlCatalog>();
        }
    } else {
        catal = xmlCreateNewCatalog(XML_XML_CATALOG_TYPE, xmlCatalogDefaultPrefer);
        if catal.is_null() {
            xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xmlCatalog>();
        }
        (*catal).xml = xmlNewCatalogEntry(
            XML_CATA_CATALOG,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
            filename as *mut xmlChar,
            xmlCatalogDefaultPrefer,
            ::core::ptr::null_mut::<xmlCatalogEntry>(),
        );
    }
    xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
    return catal;
}
unsafe extern "C" fn xmlExpandCatalog(
    mut catal: xmlCatalogPtr,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if catal.is_null() || filename.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_SGML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut content: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        content = xmlLoadFileContent(filename);
        if content.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        ret = xmlParseSGMLCatalog(catal, content, filename, 0 as ::core::ffi::c_int);
        if ret < 0 as ::core::ffi::c_int {
            xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
    } else {
        let mut tmp: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
        let mut cur: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
        tmp = xmlNewCatalogEntry(
            XML_CATA_CATALOG,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
            filename as *mut xmlChar,
            xmlCatalogDefaultPrefer,
            ::core::ptr::null_mut::<xmlCatalogEntry>(),
        );
        cur = (*catal).xml;
        if cur.is_null() {
            (*catal).xml = tmp;
        } else {
            while !(*cur).next.is_null() {
                cur = (*cur).next as xmlCatalogEntryPtr;
            }
            (*cur).next = tmp as *mut _xmlCatalogEntry;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogResolveSystem(
    mut catal: xmlCatalogPtr,
    mut sysID: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if sysID.is_null() || catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Resolve sysID %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            sysID,
        );
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_XML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ret = xmlCatalogListXMLResolve((*catal).xml, ::core::ptr::null::<xmlChar>(), sysID);
        if ret == XML_CATAL_BREAK {
            ret = ::core::ptr::null_mut::<xmlChar>();
        }
    } else {
        let mut sgml: *const xmlChar = ::core::ptr::null::<xmlChar>();
        sgml = xmlCatalogGetSGMLSystem((*catal).sgml, sysID);
        if !sgml.is_null() {
            ret = xmlStrdup(sgml);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogResolvePublic(
    mut catal: xmlCatalogPtr,
    mut pubID: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if pubID.is_null() || catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Resolve pubID %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            pubID,
        );
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_XML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ret = xmlCatalogListXMLResolve((*catal).xml, pubID, ::core::ptr::null::<xmlChar>());
        if ret == XML_CATAL_BREAK {
            ret = ::core::ptr::null_mut::<xmlChar>();
        }
    } else {
        let mut sgml: *const xmlChar = ::core::ptr::null::<xmlChar>();
        sgml = xmlCatalogGetSGMLPublic((*catal).sgml, pubID);
        if !sgml.is_null() {
            ret = xmlStrdup(sgml);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogResolve(
    mut catal: xmlCatalogPtr,
    mut pubID: *const xmlChar,
    mut sysID: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if pubID.is_null() && sysID.is_null() || catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlDebugCatalogs != 0 {
        if !pubID.is_null() && !sysID.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Resolve: pubID %s sysID %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                pubID,
                sysID,
            );
        } else if !pubID.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Resolve: pubID %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                pubID,
            );
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Resolve: sysID %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                sysID,
            );
        }
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_XML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ret = xmlCatalogListXMLResolve((*catal).xml, pubID, sysID);
        if ret == XML_CATAL_BREAK {
            ret = ::core::ptr::null_mut::<xmlChar>();
        }
    } else {
        let mut sgml: *const xmlChar = ::core::ptr::null::<xmlChar>();
        sgml = xmlCatalogSGMLResolve(catal, pubID, sysID);
        if !sgml.is_null() {
            ret = xmlStrdup(sgml);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogResolveURI(
    mut catal: xmlCatalogPtr,
    mut URI: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if URI.is_null() || catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Resolve URI %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            URI,
        );
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_XML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ret = xmlCatalogListXMLResolveURI((*catal).xml, URI);
        if ret == XML_CATAL_BREAK {
            ret = ::core::ptr::null_mut::<xmlChar>();
        }
    } else {
        let mut sgml: *const xmlChar = ::core::ptr::null::<xmlChar>();
        sgml = xmlCatalogSGMLResolve(catal, ::core::ptr::null::<xmlChar>(), URI);
        if !sgml.is_null() {
            ret = xmlStrdup(sgml);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogDump(mut catal: xmlCatalogPtr, mut out: *mut FILE) {
    if out.is_null() || catal.is_null() {
        return;
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_XML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlDumpXMLCatalog(out, (*catal).xml);
    } else {
        xmlHashScan(
            (*catal).sgml,
            Some(
                xmlCatalogDumpEntry
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            out as *mut ::core::ffi::c_void,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogAdd(
    mut catal: xmlCatalogPtr,
    mut type_0: *const xmlChar,
    mut orig: *const xmlChar,
    mut replace: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if catal.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_XML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        res = xmlAddXMLCatalog((*catal).xml, type_0, orig, replace);
    } else {
        let mut cattype: xmlCatalogEntryType = XML_CATA_NONE;
        cattype = xmlGetSGMLCatalogEntryType(type_0);
        if cattype as ::core::ffi::c_int != XML_CATA_NONE as ::core::ffi::c_int {
            let mut entry: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
            entry = xmlNewCatalogEntry(
                cattype,
                orig,
                replace,
                ::core::ptr::null::<xmlChar>(),
                XML_CATA_PREFER_NONE,
                ::core::ptr::null_mut::<xmlCatalogEntry>(),
            );
            if (*catal).sgml.is_null() {
                (*catal).sgml = xmlHashCreate(10 as ::core::ffi::c_int);
            }
            res = xmlHashAddEntry((*catal).sgml, orig, entry as *mut ::core::ffi::c_void);
            if res < 0 as ::core::ffi::c_int {
                xmlFreeCatalogEntry(
                    entry as *mut ::core::ffi::c_void,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlACatalogRemove(
    mut catal: xmlCatalogPtr,
    mut value: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if catal.is_null() || value.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_XML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        res = xmlDelXMLCatalog((*catal).xml, value);
    } else {
        res = xmlHashRemoveEntry(
            (*catal).sgml,
            value,
            Some(
                xmlFreeCatalogEntry
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
            ),
        );
        if res == 0 as ::core::ffi::c_int {
            res = 1 as ::core::ffi::c_int;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewCatalog(mut sgml: ::core::ffi::c_int) -> xmlCatalogPtr {
    let mut catal: xmlCatalogPtr = ::core::ptr::null_mut::<xmlCatalog>();
    if sgml != 0 {
        catal = xmlCreateNewCatalog(XML_SGML_CATALOG_TYPE, xmlCatalogDefaultPrefer);
        if !catal.is_null() && (*catal).sgml.is_null() {
            (*catal).sgml = xmlHashCreate(10 as ::core::ffi::c_int);
        }
    } else {
        catal = xmlCreateNewCatalog(XML_XML_CATALOG_TYPE, xmlCatalogDefaultPrefer);
    }
    return catal;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogIsEmpty(mut catal: xmlCatalogPtr) -> ::core::ffi::c_int {
    if catal.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*catal).type_0 as ::core::ffi::c_uint
        == XML_XML_CATALOG_TYPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*catal).xml.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        if (*(*catal).xml).type_0 as ::core::ffi::c_int != XML_CATA_CATALOG as ::core::ffi::c_int
            && (*(*catal).xml).type_0 as ::core::ffi::c_int
                != XML_CATA_BROKEN_CATALOG as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        if (*(*catal).xml).children.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    } else {
        let mut res: ::core::ffi::c_int = 0;
        if (*catal).sgml.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        res = xmlHashSize((*catal).sgml);
        if res == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if res < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlInitializeCatalogData() {
    if xmlCatalogInitialized != 0 as ::core::ffi::c_int {
        return;
    }
    if !getenv(b"XML_DEBUG_CATALOG\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        xmlDebugCatalogs = 1 as ::core::ffi::c_int;
    }
    xmlCatalogMutex = xmlNewRMutex();
    xmlCatalogInitialized = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeCatalog() {
    if xmlCatalogInitialized != 0 as ::core::ffi::c_int {
        return;
    }
    xmlInitializeCatalogData();
    xmlRMutexLock(xmlCatalogMutex);
    if !getenv(b"XML_DEBUG_CATALOG\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        xmlDebugCatalogs = 1 as ::core::ffi::c_int;
    }
    if xmlDefaultCatalog.is_null() {
        let mut catalogs: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut cur: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut paths: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut catal: xmlCatalogPtr = ::core::ptr::null_mut::<xmlCatalog>();
        let mut nextent: *mut xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntryPtr>();
        catalogs = getenv(b"XML_CATALOG_FILES\0" as *const u8 as *const ::core::ffi::c_char)
            as *const ::core::ffi::c_char;
        if catalogs.is_null() {
            catalogs = XML_XML_DEFAULT_CATALOG.as_ptr();
        }
        catal = xmlCreateNewCatalog(XML_XML_CATALOG_TYPE, xmlCatalogDefaultPrefer);
        if !catal.is_null() {
            cur = catalogs;
            nextent = &raw mut (*catal).xml;
            while *cur as ::core::ffi::c_int != '\0' as i32 {
                while *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                    || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                        && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                    || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
                {
                    cur = cur.offset(1);
                }
                if *cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    paths = cur;
                    while *cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                        && !(*cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                            || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                                && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                            || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
                    {
                        cur = cur.offset(1);
                    }
                    path = xmlStrndup(
                        paths as *const xmlChar,
                        cur.offset_from(paths) as ::core::ffi::c_long as ::core::ffi::c_int,
                    ) as *mut ::core::ffi::c_char;
                    if !path.is_null() {
                        *nextent = xmlNewCatalogEntry(
                            XML_CATA_CATALOG,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                            path as *mut xmlChar,
                            xmlCatalogDefaultPrefer,
                            ::core::ptr::null_mut::<xmlCatalogEntry>(),
                        );
                        if !(*nextent).is_null() {
                            nextent = &raw mut (**nextent).next as *mut xmlCatalogEntryPtr;
                        }
                        xmlFree.expect("non-null function pointer")(
                            path as *mut ::core::ffi::c_void,
                        );
                    }
                }
            }
            xmlDefaultCatalog = catal;
        }
    }
    xmlRMutexUnlock(xmlCatalogMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlLoadCatalog(
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut catal: xmlCatalogPtr = ::core::ptr::null_mut::<xmlCatalog>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalogData();
    }
    xmlRMutexLock(xmlCatalogMutex);
    if xmlDefaultCatalog.is_null() {
        catal = xmlLoadACatalog(filename);
        if catal.is_null() {
            xmlRMutexUnlock(xmlCatalogMutex);
            return -(1 as ::core::ffi::c_int);
        }
        xmlDefaultCatalog = catal;
        xmlRMutexUnlock(xmlCatalogMutex);
        return 0 as ::core::ffi::c_int;
    }
    ret = xmlExpandCatalog(xmlDefaultCatalog, filename);
    xmlRMutexUnlock(xmlCatalogMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlLoadCatalogs(mut pathss: *const ::core::ffi::c_char) {
    let mut cur: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut paths: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut path: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if pathss.is_null() {
        return;
    }
    cur = pathss;
    while *cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        while *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
        {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            paths = cur;
            while *cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && *cur as ::core::ffi::c_int != PATH_SEPARATOR
                && !(*cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                    || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                        && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                    || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
            {
                cur = cur.offset(1);
            }
            path = xmlStrndup(
                paths as *const xmlChar,
                cur.offset_from(paths) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
            if !path.is_null() {
                xmlLoadCatalog(path as *const ::core::ffi::c_char);
                xmlFree.expect("non-null function pointer")(path as *mut ::core::ffi::c_void);
            }
        }
        while *cur as ::core::ffi::c_int == PATH_SEPARATOR {
            cur = cur.offset(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogCleanup() {
    if xmlCatalogInitialized == 0 as ::core::ffi::c_int {
        return;
    }
    xmlRMutexLock(xmlCatalogMutex);
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Catalogs cleanup\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !xmlCatalogXMLFiles.is_null() {
        xmlHashFree(
            xmlCatalogXMLFiles,
            Some(
                xmlFreeCatalogHashEntryList
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
            ),
        );
    }
    xmlCatalogXMLFiles = ::core::ptr::null_mut::<xmlHashTable>();
    if !xmlDefaultCatalog.is_null() {
        xmlFreeCatalog(xmlDefaultCatalog);
    }
    xmlDefaultCatalog = ::core::ptr::null_mut::<xmlCatalog>();
    xmlDebugCatalogs = 0 as ::core::ffi::c_int;
    xmlCatalogInitialized = 0 as ::core::ffi::c_int;
    xmlRMutexUnlock(xmlCatalogMutex);
    xmlFreeRMutex(xmlCatalogMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogResolveSystem(mut sysID: *const xmlChar) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    ret = xmlACatalogResolveSystem(xmlDefaultCatalog, sysID);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogResolvePublic(mut pubID: *const xmlChar) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    ret = xmlACatalogResolvePublic(xmlDefaultCatalog, pubID);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogResolve(
    mut pubID: *const xmlChar,
    mut sysID: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    ret = xmlACatalogResolve(xmlDefaultCatalog, pubID, sysID);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogResolveURI(mut URI: *const xmlChar) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    ret = xmlACatalogResolveURI(xmlDefaultCatalog, URI);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogDump(mut out: *mut FILE) {
    if out.is_null() {
        return;
    }
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    xmlACatalogDump(xmlDefaultCatalog, out);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogAdd(
    mut type_0: *const xmlChar,
    mut orig: *const xmlChar,
    mut replace: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalogData();
    }
    xmlRMutexLock(xmlCatalogMutex);
    if xmlDefaultCatalog.is_null()
        && xmlStrEqual(
            type_0,
            b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ) != 0
    {
        xmlDefaultCatalog = xmlCreateNewCatalog(XML_XML_CATALOG_TYPE, xmlCatalogDefaultPrefer);
        if !xmlDefaultCatalog.is_null() {
            (*xmlDefaultCatalog).xml = xmlNewCatalogEntry(
                XML_CATA_CATALOG,
                ::core::ptr::null::<xmlChar>(),
                orig,
                ::core::ptr::null::<xmlChar>(),
                xmlCatalogDefaultPrefer,
                ::core::ptr::null_mut::<xmlCatalogEntry>(),
            );
        }
        xmlRMutexUnlock(xmlCatalogMutex);
        return 0 as ::core::ffi::c_int;
    }
    res = xmlACatalogAdd(xmlDefaultCatalog, type_0, orig, replace);
    xmlRMutexUnlock(xmlCatalogMutex);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogRemove(mut value: *const xmlChar) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    xmlRMutexLock(xmlCatalogMutex);
    res = xmlACatalogRemove(xmlDefaultCatalog, value);
    xmlRMutexUnlock(xmlCatalogMutex);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogConvert() -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    xmlRMutexLock(xmlCatalogMutex);
    res = xmlConvertSGMLCatalog(xmlDefaultCatalog);
    xmlRMutexUnlock(xmlCatalogMutex);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogGetDefaults() -> xmlCatalogAllow {
    return xmlCatalogDefaultAllow;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogSetDefaults(mut allow: xmlCatalogAllow) {
    if xmlDebugCatalogs != 0 {
        match allow as ::core::ffi::c_uint {
            0 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Disabling catalog usage\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            1 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Allowing only global catalogs\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            2 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Allowing only catalogs from the document\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            3 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Allowing all catalogs\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        }
    }
    xmlCatalogDefaultAllow = allow;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogSetDefaultPrefer(
    mut prefer: xmlCatalogPrefer,
) -> xmlCatalogPrefer {
    let mut ret: xmlCatalogPrefer = xmlCatalogDefaultPrefer;
    if prefer as ::core::ffi::c_uint
        == XML_CATA_PREFER_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ret;
    }
    if xmlDebugCatalogs != 0 {
        match prefer as ::core::ffi::c_uint {
            1 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Setting catalog preference to PUBLIC\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            2 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Setting catalog preference to SYSTEM\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            _ => return ret,
        }
    }
    xmlCatalogDefaultPrefer = prefer;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogSetDebug(mut level: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = xmlDebugCatalogs;
    if level <= 0 as ::core::ffi::c_int {
        xmlDebugCatalogs = 0 as ::core::ffi::c_int;
    } else {
        xmlDebugCatalogs = level;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogFreeLocal(mut catalogs: *mut ::core::ffi::c_void) {
    let mut catal: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    catal = catalogs as xmlCatalogEntryPtr;
    if !catal.is_null() {
        xmlFreeCatalogEntryList(catal);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogAddLocal(
    mut catalogs: *mut ::core::ffi::c_void,
    mut URL: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    let mut catal: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut add: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    if URL.is_null() {
        return catalogs;
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Adding document catalog %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            URL,
        );
    }
    add = xmlNewCatalogEntry(
        XML_CATA_CATALOG,
        ::core::ptr::null::<xmlChar>(),
        URL,
        ::core::ptr::null::<xmlChar>(),
        xmlCatalogDefaultPrefer,
        ::core::ptr::null_mut::<xmlCatalogEntry>(),
    );
    if add.is_null() {
        return catalogs;
    }
    catal = catalogs as xmlCatalogEntryPtr;
    if catal.is_null() {
        return add as *mut ::core::ffi::c_void;
    }
    while !(*catal).next.is_null() {
        catal = (*catal).next as xmlCatalogEntryPtr;
    }
    (*catal).next = add as *mut _xmlCatalogEntry;
    return catalogs;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogLocalResolve(
    mut catalogs: *mut ::core::ffi::c_void,
    mut pubID: *const xmlChar,
    mut sysID: *const xmlChar,
) -> *mut xmlChar {
    let mut catal: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    if pubID.is_null() && sysID.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlDebugCatalogs != 0 {
        if !pubID.is_null() && !sysID.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Local Resolve: pubID %s sysID %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                pubID,
                sysID,
            );
        } else if !pubID.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Local Resolve: pubID %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                pubID,
            );
        } else {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Local Resolve: sysID %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                sysID,
            );
        }
    }
    catal = catalogs as xmlCatalogEntryPtr;
    if catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlCatalogListXMLResolve(catal, pubID, sysID);
    if !ret.is_null() && ret != XML_CATAL_BREAK {
        return ret;
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogLocalResolveURI(
    mut catalogs: *mut ::core::ffi::c_void,
    mut URI: *const xmlChar,
) -> *mut xmlChar {
    let mut catal: xmlCatalogEntryPtr = ::core::ptr::null_mut::<xmlCatalogEntry>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    if URI.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlDebugCatalogs != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Resolve URI %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            URI,
        );
    }
    catal = catalogs as xmlCatalogEntryPtr;
    if catal.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlCatalogListXMLResolveURI(catal, URI);
    if !ret.is_null() && ret != XML_CATAL_BREAK {
        return ret;
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogGetSystem(mut sysID: *const xmlChar) -> *const xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    static mut result: [xmlChar; 1000] = [0; 1000];
    static mut msg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    if msg == 0 as ::core::ffi::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Use of deprecated xmlCatalogGetSystem() call\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        msg += 1;
    }
    if sysID.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !xmlDefaultCatalog.is_null() {
        ret = xmlCatalogListXMLResolve(
            (*xmlDefaultCatalog).xml,
            ::core::ptr::null::<xmlChar>(),
            sysID,
        );
        if !ret.is_null() && ret != XML_CATAL_BREAK {
            snprintf(
                &raw mut result as *mut xmlChar as *mut ::core::ffi::c_char,
                (::core::mem::size_of::<[xmlChar; 1000]>() as size_t).wrapping_sub(1 as size_t),
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                ret as *mut ::core::ffi::c_char,
            );
            result[(::core::mem::size_of::<[xmlChar; 1000]>() as usize).wrapping_sub(1 as usize)
                as usize] = 0 as xmlChar;
            return &raw mut result as *mut xmlChar;
        }
    }
    if !xmlDefaultCatalog.is_null() {
        return xmlCatalogGetSGMLSystem((*xmlDefaultCatalog).sgml, sysID);
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlCatalogGetPublic(mut pubID: *const xmlChar) -> *const xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    static mut result: [xmlChar; 1000] = [0; 1000];
    static mut msg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if xmlCatalogInitialized == 0 {
        xmlInitializeCatalog();
    }
    if msg == 0 as ::core::ffi::c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Use of deprecated xmlCatalogGetPublic() call\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        msg += 1;
    }
    if pubID.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !xmlDefaultCatalog.is_null() {
        ret = xmlCatalogListXMLResolve(
            (*xmlDefaultCatalog).xml,
            pubID,
            ::core::ptr::null::<xmlChar>(),
        );
        if !ret.is_null() && ret != XML_CATAL_BREAK {
            snprintf(
                &raw mut result as *mut xmlChar as *mut ::core::ffi::c_char,
                (::core::mem::size_of::<[xmlChar; 1000]>() as size_t).wrapping_sub(1 as size_t),
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                ret as *mut ::core::ffi::c_char,
            );
            result[(::core::mem::size_of::<[xmlChar; 1000]>() as usize).wrapping_sub(1 as usize)
                as usize] = 0 as xmlChar;
            return &raw mut result as *mut xmlChar;
        }
    }
    if !xmlDefaultCatalog.is_null() {
        return xmlCatalogGetSGMLPublic((*xmlDefaultCatalog).sgml, pubID);
    }
    return ::core::ptr::null::<xmlChar>();
}
