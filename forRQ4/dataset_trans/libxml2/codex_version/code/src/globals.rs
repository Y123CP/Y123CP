extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlBuf;
    pub type _xmlDict;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn xmlCharStrdup(cur: *const ::core::ffi::c_char) -> *mut xmlChar;
    fn __xmlParserInputBufferCreateFilename(
        URI: *const ::core::ffi::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn __xmlOutputBufferCreateFilename(
        URI: *const ::core::ffi::c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: ::core::ffi::c_int,
    ) -> xmlOutputBufferPtr;
    fn xmlSAX2GetPublicId(ctx: *mut ::core::ffi::c_void) -> *const xmlChar;
    fn xmlSAX2GetSystemId(ctx: *mut ::core::ffi::c_void) -> *const xmlChar;
    fn xmlSAX2SetDocumentLocator(ctx: *mut ::core::ffi::c_void, loc: xmlSAXLocatorPtr);
    fn xmlSAX2GetLineNumber(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2GetColumnNumber(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2IsStandalone(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2HasInternalSubset(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2HasExternalSubset(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn xmlSAX2InternalSubset(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlSAX2ExternalSubset(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlSAX2GetEntity(ctx: *mut ::core::ffi::c_void, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlSAX2GetParameterEntity(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlSAX2ResolveEntity(
        ctx: *mut ::core::ffi::c_void,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    ) -> xmlParserInputPtr;
    fn xmlSAX2EntityDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        type_0: ::core::ffi::c_int,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        content: *mut xmlChar,
    );
    fn xmlSAX2AttributeDecl(
        ctx: *mut ::core::ffi::c_void,
        elem: *const xmlChar,
        fullname: *const xmlChar,
        type_0: ::core::ffi::c_int,
        def: ::core::ffi::c_int,
        defaultValue: *const xmlChar,
        tree: xmlEnumerationPtr,
    );
    fn xmlSAX2ElementDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        type_0: ::core::ffi::c_int,
        content: xmlElementContentPtr,
    );
    fn xmlSAX2NotationDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    );
    fn xmlSAX2UnparsedEntityDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        notationName: *const xmlChar,
    );
    fn xmlSAX2StartDocument(ctx: *mut ::core::ffi::c_void);
    fn xmlSAX2EndDocument(ctx: *mut ::core::ffi::c_void);
    fn xmlSAX2StartElement(
        ctx: *mut ::core::ffi::c_void,
        fullname: *const xmlChar,
        atts: *mut *const xmlChar,
    );
    fn xmlSAX2EndElement(ctx: *mut ::core::ffi::c_void, name: *const xmlChar);
    fn xmlSAX2Reference(ctx: *mut ::core::ffi::c_void, name: *const xmlChar);
    fn xmlSAX2Characters(
        ctx: *mut ::core::ffi::c_void,
        ch: *const xmlChar,
        len: ::core::ffi::c_int,
    );
    fn xmlSAX2IgnorableWhitespace(
        ctx: *mut ::core::ffi::c_void,
        ch: *const xmlChar,
        len: ::core::ffi::c_int,
    );
    fn xmlSAX2ProcessingInstruction(
        ctx: *mut ::core::ffi::c_void,
        target: *const xmlChar,
        data: *const xmlChar,
    );
    fn xmlSAX2Comment(ctx: *mut ::core::ffi::c_void, value: *const xmlChar);
    fn xmlSAX2CDataBlock(
        ctx: *mut ::core::ffi::c_void,
        value: *const xmlChar,
        len: ::core::ffi::c_int,
    );
    fn xmlInitMutex(mutex: xmlMutexPtr);
    fn xmlCleanupMutex(mutex: xmlMutexPtr);
    fn xmlParserError(ctx: *mut ::core::ffi::c_void, msg: *const ::core::ffi::c_char, ...);
    fn xmlParserWarning(ctx: *mut ::core::ffi::c_void, msg: *const ::core::ffi::c_char, ...);
    fn xmlResetError(err: xmlErrorPtr);
    static xmlParserVersion: *const ::core::ffi::c_char;
    fn xmlInitParser();
    fn xmlMutexLock(tok: xmlMutexPtr);
    fn xmlMutexUnlock(tok: xmlMutexPtr);
    fn xmlGenericErrorDefaultFunc(
        ctx: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn pthread_self() -> pthread_t;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn pthread_key_delete(__key: pthread_key_t) -> ::core::ffi::c_int;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut ::core::ffi::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    static mut __xmlRegisterCallbacks: ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type pthread_key_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type xmlChar = ::core::ffi::c_uchar;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
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
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlStructuredErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> ()>;
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
pub type xmlBufferAllocationScheme = ::core::ffi::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
pub type xmlRegisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlDeregisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlGlobalStatePtr = *mut xmlGlobalState;
pub type xmlGlobalState = _xmlGlobalState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlGlobalState {
    pub initialized: ::core::ffi::c_int,
    pub gs_xmlLastError: xmlError,
    pub gs_xmlGenericError: xmlGenericErrorFunc,
    pub gs_xmlGenericErrorContext: *mut ::core::ffi::c_void,
    pub gs_xmlStructuredError: xmlStructuredErrorFunc,
    pub gs_xmlStructuredErrorContext: *mut ::core::ffi::c_void,
    pub gs_htmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub gs_xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc,
    pub gs_xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc,
    pub gs_oldXMLWDcompatibility: ::core::ffi::c_int,
    pub gs_xmlDefaultSAXLocator: xmlSAXLocator,
    pub gs_xmlDoValidityCheckingDefaultValue: ::core::ffi::c_int,
    pub gs_xmlGetWarningsDefaultValue: ::core::ffi::c_int,
    pub gs_xmlKeepBlanksDefaultValue: ::core::ffi::c_int,
    pub gs_xmlLineNumbersDefaultValue: ::core::ffi::c_int,
    pub gs_xmlLoadExtDtdDefaultValue: ::core::ffi::c_int,
    pub gs_xmlParserDebugEntities: ::core::ffi::c_int,
    pub gs_xmlPedanticParserDefaultValue: ::core::ffi::c_int,
    pub gs_xmlSubstituteEntitiesDefaultValue: ::core::ffi::c_int,
    pub gs_xmlIndentTreeOutput: ::core::ffi::c_int,
    pub gs_xmlTreeIndentString: *const ::core::ffi::c_char,
    pub gs_xmlSaveNoEmptyTags: ::core::ffi::c_int,
    pub gs_xmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub gs_xmlBufferAllocScheme: xmlBufferAllocationScheme,
    pub gs_xmlDefaultBufferSize: ::core::ffi::c_int,
    pub gs_xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc,
    pub gs_xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc,
}
pub type xmlSAXHandlerV1 = _xmlSAXHandlerV1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandlerV1 {
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
}
pub type xmlOutputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        xmlCharEncodingHandlerPtr,
        ::core::ffi::c_int,
    ) -> xmlOutputBufferPtr,
>;
pub type xmlParserInputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, xmlCharEncoding) -> xmlParserInputBufferPtr,
>;
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
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlMutex = _xmlMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlMutex {
    pub lock: pthread_mutex_t,
}
pub type xmlMutexPtr = *mut xmlMutex;
pub type xmlErrorPtr = *mut xmlError;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut parserInitialized: ::core::ffi::c_int = 0;
static mut xmlThrDefMutex: xmlMutex = xmlMutex {
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
static mut libxml_is_threaded: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut globalkey: pthread_key_t = 0;
static mut mainthread: pthread_t = 0;
#[no_mangle]
pub static mut xmlFree: xmlFreeFunc =
    unsafe { Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()) };
#[no_mangle]
pub static mut xmlMalloc: xmlMallocFunc =
    unsafe { Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void) };
#[no_mangle]
pub static mut xmlMallocAtomic: xmlMallocFunc =
    unsafe { Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void) };
#[no_mangle]
pub static mut xmlRealloc: xmlReallocFunc = unsafe {
    Some(
        realloc
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void,
    )
};
unsafe extern "C" fn xmlPosixStrdup(
    mut cur: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return xmlCharStrdup(cur) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub static mut xmlMemStrdup: xmlStrdupFunc = unsafe {
    Some(
        xmlPosixStrdup
            as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char,
    )
};
#[no_mangle]
pub static mut xmlBufferAllocScheme: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_EXACT;
static mut xmlBufferAllocSchemeThrDef: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_EXACT;
#[no_mangle]
pub static mut xmlDefaultBufferSize: ::core::ffi::c_int = BASE_BUFFER_SIZE;
static mut xmlDefaultBufferSizeThrDef: ::core::ffi::c_int = BASE_BUFFER_SIZE;
#[no_mangle]
pub static mut oldXMLWDcompatibility: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlParserDebugEntities: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlParserDebugEntitiesThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlDoValidityCheckingDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlDoValidityCheckingDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlGetWarningsDefaultValue: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut xmlGetWarningsDefaultValueThrDef: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlLoadExtDtdDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlLoadExtDtdDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlPedanticParserDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlPedanticParserDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlLineNumbersDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlLineNumbersDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlKeepBlanksDefaultValue: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut xmlKeepBlanksDefaultValueThrDef: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlSubstituteEntitiesDefaultValue: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlSubstituteEntitiesDefaultValueThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc = None;
static mut xmlRegisterNodeDefaultValueThrDef: xmlRegisterNodeFunc = None;
#[no_mangle]
pub static mut xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc = None;
static mut xmlDeregisterNodeDefaultValueThrDef: xmlDeregisterNodeFunc = None;
#[no_mangle]
pub static mut xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc =
    None;
static mut xmlParserInputBufferCreateFilenameValueThrDef: xmlParserInputBufferCreateFilenameFunc =
    None;
#[no_mangle]
pub static mut xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc = None;
static mut xmlOutputBufferCreateFilenameValueThrDef: xmlOutputBufferCreateFilenameFunc = None;
#[no_mangle]
pub static mut xmlGenericError: xmlGenericErrorFunc = unsafe {
    Some(
        xmlGenericErrorDefaultFunc
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    )
};
static mut xmlGenericErrorThrDef: xmlGenericErrorFunc = unsafe {
    Some(
        xmlGenericErrorDefaultFunc
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    )
};
#[no_mangle]
pub static mut xmlStructuredError: xmlStructuredErrorFunc = None;
static mut xmlStructuredErrorThrDef: xmlStructuredErrorFunc = None;
#[no_mangle]
pub static mut xmlGenericErrorContext: *mut ::core::ffi::c_void = NULL;
static mut xmlGenericErrorContextThrDef: *mut ::core::ffi::c_void = NULL;
#[no_mangle]
pub static mut xmlStructuredErrorContext: *mut ::core::ffi::c_void = NULL;
static mut xmlStructuredErrorContextThrDef: *mut ::core::ffi::c_void = NULL;
#[no_mangle]
pub static mut xmlLastError: xmlError = xmlError {
    domain: 0,
    code: 0,
    message: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    level: XML_ERR_NONE,
    file: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    line: 0,
    str1: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    str2: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    str3: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    int1: 0,
    int2: 0,
    ctxt: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    node: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
#[no_mangle]
pub static mut xmlIndentTreeOutput: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut xmlIndentTreeOutputThrDef: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlTreeIndentString: *const ::core::ffi::c_char =
    b"  \0" as *const u8 as *const ::core::ffi::c_char;
static mut xmlTreeIndentStringThrDef: *const ::core::ffi::c_char =
    b"  \0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub static mut xmlSaveNoEmptyTags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xmlSaveNoEmptyTagsThrDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlDefaultSAXHandler: xmlSAXHandlerV1 = unsafe {
    _xmlSAXHandlerV1 {
        internalSubset: Some(
            xmlSAX2InternalSubset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        isStandalone: Some(
            xmlSAX2IsStandalone
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        hasInternalSubset: Some(
            xmlSAX2HasInternalSubset
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        hasExternalSubset: Some(
            xmlSAX2HasExternalSubset
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        resolveEntity: Some(
            xmlSAX2ResolveEntity
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> xmlParserInputPtr,
        ),
        getEntity: Some(
            xmlSAX2GetEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: Some(
            xmlSAX2EntityDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    *const xmlChar,
                    *const xmlChar,
                    *mut xmlChar,
                ) -> (),
        ),
        notationDecl: Some(
            xmlSAX2NotationDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        attributeDecl: Some(
            xmlSAX2AttributeDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *const xmlChar,
                    xmlEnumerationPtr,
                ) -> (),
        ),
        elementDecl: Some(
            xmlSAX2ElementDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    xmlElementContentPtr,
                ) -> (),
        ),
        unparsedEntityDecl: Some(
            xmlSAX2UnparsedEntityDecl
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        setDocumentLocator: Some(
            xmlSAX2SetDocumentLocator
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            xmlSAX2StartDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        endDocument: Some(
            xmlSAX2EndDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        startElement: Some(
            xmlSAX2StartElement
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *mut *const xmlChar,
                ) -> (),
        ),
        endElement: Some(
            xmlSAX2EndElement
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        reference: Some(
            xmlSAX2Reference
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        characters: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        processingInstruction: Some(
            xmlSAX2ProcessingInstruction
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        comment: Some(
            xmlSAX2Comment as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: Some(
            xmlSAX2GetParameterEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        cdataBlock: Some(
            xmlSAX2CDataBlock
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        externalSubset: Some(
            xmlSAX2ExternalSubset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        initialized: 1 as ::core::ffi::c_uint,
    }
};
#[no_mangle]
pub static mut xmlDefaultSAXLocator: xmlSAXLocator = unsafe {
    _xmlSAXLocator {
        getPublicId: Some(
            xmlSAX2GetPublicId as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
        ),
        getSystemId: Some(
            xmlSAX2GetSystemId as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
        ),
        getLineNumber: Some(
            xmlSAX2GetLineNumber
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        getColumnNumber: Some(
            xmlSAX2GetColumnNumber
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut htmlDefaultSAXHandler: xmlSAXHandlerV1 = unsafe {
    _xmlSAXHandlerV1 {
        internalSubset: Some(
            xmlSAX2InternalSubset
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        isStandalone: None,
        hasInternalSubset: None,
        hasExternalSubset: None,
        resolveEntity: None,
        getEntity: Some(
            xmlSAX2GetEntity
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: None,
        notationDecl: None,
        attributeDecl: None,
        elementDecl: None,
        unparsedEntityDecl: None,
        setDocumentLocator: Some(
            xmlSAX2SetDocumentLocator
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            xmlSAX2StartDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        endDocument: Some(
            xmlSAX2EndDocument as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        ),
        startElement: Some(
            xmlSAX2StartElement
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *mut *const xmlChar,
                ) -> (),
        ),
        endElement: Some(
            xmlSAX2EndElement
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        reference: None,
        characters: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            xmlSAX2IgnorableWhitespace
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        processingInstruction: Some(
            xmlSAX2ProcessingInstruction
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        comment: Some(
            xmlSAX2Comment as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: None,
        cdataBlock: Some(
            xmlSAX2CDataBlock
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        externalSubset: None,
        initialized: 1 as ::core::ffi::c_uint,
    }
};
#[no_mangle]
pub unsafe extern "C" fn xmlInitGlobals() {
    xmlInitParser();
}
#[no_mangle]
pub unsafe extern "C" fn xmlInitGlobalsInternal() {
    xmlInitMutex(&raw mut xmlThrDefMutex);
    if libxml_is_threaded == -(1 as ::core::ffi::c_int) {
        libxml_is_threaded = (Some(
            pthread_getspecific as unsafe extern "C" fn(pthread_key_t) -> *mut ::core::ffi::c_void,
        )
        .is_some()
            && Some(
                pthread_setspecific
                    as unsafe extern "C" fn(
                        pthread_key_t,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(
                pthread_key_create
                    as unsafe extern "C" fn(
                        *mut pthread_key_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    ) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(
                pthread_key_delete as unsafe extern "C" fn(pthread_key_t) -> ::core::ffi::c_int,
            )
            .is_some()
            && Some(pthread_self as unsafe extern "C" fn() -> pthread_t).is_some())
            as ::core::ffi::c_int;
    }
    if libxml_is_threaded == 0 as ::core::ffi::c_int {
        return;
    }
    pthread_key_create(
        &raw mut globalkey,
        Some(xmlFreeGlobalState as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    mainthread = pthread_self();
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupGlobals() {}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupGlobalsInternal() {
    xmlResetError(&raw mut xmlLastError);
    xmlCleanupMutex(&raw mut xmlThrDefMutex);
    if libxml_is_threaded == 0 as ::core::ffi::c_int {
        return;
    }
    pthread_key_delete(globalkey);
    parserInitialized = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeGlobalState(mut gs: xmlGlobalStatePtr) {}
#[no_mangle]
pub unsafe extern "C" fn xmlGetGlobalState() -> xmlGlobalStatePtr {
    return ::core::ptr::null_mut::<xmlGlobalState>();
}
unsafe extern "C" fn xmlIsMainThreadInternal() -> ::core::ffi::c_int {
    if parserInitialized == 0 as ::core::ffi::c_int {
        xmlInitParser();
        parserInitialized = 1 as ::core::ffi::c_int;
    }
    if libxml_is_threaded == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    return pthread_equal(mainthread, pthread_self());
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsMainThread() -> ::core::ffi::c_int {
    return xmlIsMainThreadInternal();
}
unsafe extern "C" fn xmlFreeGlobalState(mut state: *mut ::core::ffi::c_void) {
    let mut gs: *mut xmlGlobalState = state as *mut xmlGlobalState;
    xmlResetError(&raw mut (*gs).gs_xmlLastError);
    free(state);
}
unsafe extern "C" fn xmlInitGlobalState(mut gs: xmlGlobalStatePtr) {
    xmlMutexLock(&raw mut xmlThrDefMutex);
    (*gs).gs_oldXMLWDcompatibility = 0 as ::core::ffi::c_int;
    (*gs).gs_xmlBufferAllocScheme = xmlBufferAllocSchemeThrDef;
    (*gs).gs_xmlDefaultBufferSize = xmlDefaultBufferSizeThrDef;
    (*gs).gs_xmlDefaultSAXLocator.getPublicId = Some(
        xmlSAX2GetPublicId as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>;
    (*gs).gs_xmlDefaultSAXLocator.getSystemId = Some(
        xmlSAX2GetSystemId as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>;
    (*gs).gs_xmlDefaultSAXLocator.getLineNumber = Some(
        xmlSAX2GetLineNumber
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    (*gs).gs_xmlDefaultSAXLocator.getColumnNumber = Some(
        xmlSAX2GetColumnNumber
            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    (*gs).gs_xmlDoValidityCheckingDefaultValue = xmlDoValidityCheckingDefaultValueThrDef;
    (*gs).gs_xmlGetWarningsDefaultValue = xmlGetWarningsDefaultValueThrDef;
    (*gs).gs_xmlIndentTreeOutput = xmlIndentTreeOutputThrDef;
    (*gs).gs_xmlTreeIndentString = xmlTreeIndentStringThrDef;
    (*gs).gs_xmlSaveNoEmptyTags = xmlSaveNoEmptyTagsThrDef;
    (*gs).gs_xmlKeepBlanksDefaultValue = xmlKeepBlanksDefaultValueThrDef;
    (*gs).gs_xmlLineNumbersDefaultValue = xmlLineNumbersDefaultValueThrDef;
    (*gs).gs_xmlLoadExtDtdDefaultValue = xmlLoadExtDtdDefaultValueThrDef;
    (*gs).gs_xmlParserDebugEntities = xmlParserDebugEntitiesThrDef;
    (*gs).gs_xmlPedanticParserDefaultValue = xmlPedanticParserDefaultValueThrDef;
    (*gs).gs_xmlSubstituteEntitiesDefaultValue = xmlSubstituteEntitiesDefaultValueThrDef;
    (*gs).gs_xmlGenericError = xmlGenericErrorThrDef;
    (*gs).gs_xmlStructuredError = xmlStructuredErrorThrDef;
    (*gs).gs_xmlGenericErrorContext = xmlGenericErrorContextThrDef;
    (*gs).gs_xmlStructuredErrorContext = xmlStructuredErrorContextThrDef;
    (*gs).gs_xmlRegisterNodeDefaultValue = xmlRegisterNodeDefaultValueThrDef;
    (*gs).gs_xmlDeregisterNodeDefaultValue = xmlDeregisterNodeDefaultValueThrDef;
    (*gs).gs_xmlParserInputBufferCreateFilenameValue =
        xmlParserInputBufferCreateFilenameValueThrDef;
    (*gs).gs_xmlOutputBufferCreateFilenameValue = xmlOutputBufferCreateFilenameValueThrDef;
    memset(
        &raw mut (*gs).gs_xmlLastError as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlError>() as size_t,
    );
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    pthread_setspecific(globalkey, gs as *const ::core::ffi::c_void);
    (*gs).initialized = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlNewGlobalState(mut allowFailure: ::core::ffi::c_int) -> xmlGlobalStatePtr {
    let mut gs: *mut xmlGlobalState = ::core::ptr::null_mut::<xmlGlobalState>();
    gs = malloc(::core::mem::size_of::<xmlGlobalState>() as size_t) as *mut xmlGlobalState;
    if gs.is_null() {
        if allowFailure != 0 {
            return ::core::ptr::null_mut::<xmlGlobalState>();
        }
        fprintf(
            stderr,
            b"libxml2: Failed to allocate globals for thread\nlibxml2: See xmlCheckThreadLocalStorage\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        abort();
    }
    memset(
        gs as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlGlobalState>() as size_t,
    );
    xmlInitGlobalState(gs as xmlGlobalStatePtr);
    return gs as xmlGlobalStatePtr;
}
unsafe extern "C" fn xmlGetThreadLocalStorage(
    mut allowFailure: ::core::ffi::c_int,
) -> xmlGlobalStatePtr {
    let mut gs: *mut xmlGlobalState = ::core::ptr::null_mut::<xmlGlobalState>();
    gs = pthread_getspecific(globalkey) as *mut xmlGlobalState;
    if gs.is_null() {
        gs = xmlNewGlobalState(allowFailure) as *mut xmlGlobalState;
    }
    return gs as xmlGlobalStatePtr;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredErrorContext() -> *mut *mut ::core::ffi::c_void {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlStructuredErrorContext;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlStructuredErrorContext;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredError() -> *mut xmlStructuredErrorFunc {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlStructuredError;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlStructuredError;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLastError() -> *mut xmlError {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlLastError;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlLastError;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericError() -> *mut xmlGenericErrorFunc {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlGenericError;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlGenericError;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericErrorContext() -> *mut *mut ::core::ffi::c_void {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlGenericErrorContext;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlGenericErrorContext;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut htmlDefaultSAXHandler;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_htmlDefaultSAXHandler;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserInputBufferCreateFilenameValue(
) -> *mut xmlParserInputBufferCreateFilenameFunc {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlParserInputBufferCreateFilenameValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlParserInputBufferCreateFilenameValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlOutputBufferCreateFilenameValue(
) -> *mut xmlOutputBufferCreateFilenameFunc {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlOutputBufferCreateFilenameValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlOutputBufferCreateFilenameValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLineNumbersDefaultValue() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlLineNumbersDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlLineNumbersDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlPedanticParserDefaultValue() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlPedanticParserDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlPedanticParserDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLoadExtDtdDefaultValue() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlLoadExtDtdDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlLoadExtDtdDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserDebugEntities() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlParserDebugEntities;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlParserDebugEntities;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlIndentTreeOutput() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlIndentTreeOutput;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlIndentTreeOutput;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlTreeIndentString() -> *mut *const ::core::ffi::c_char {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlTreeIndentString;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlTreeIndentString;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlSubstituteEntitiesDefaultValue() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlSubstituteEntitiesDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlSubstituteEntitiesDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlSaveNoEmptyTags() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlSaveNoEmptyTags;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlSaveNoEmptyTags;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDefaultSAXHandler;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlDefaultSAXHandler;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDefaultSAXLocator;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlDefaultSAXLocator;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __oldXMLWDcompatibility() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut oldXMLWDcompatibility;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_oldXMLWDcompatibility;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDoValidityCheckingDefaultValue() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDoValidityCheckingDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlDoValidityCheckingDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGetWarningsDefaultValue() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlGetWarningsDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlGetWarningsDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlKeepBlanksDefaultValue() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlKeepBlanksDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlKeepBlanksDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlBufferAllocScheme() -> *mut xmlBufferAllocationScheme {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlBufferAllocScheme;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlBufferAllocScheme;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlRegisterNodeDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlRegisterNodeDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDeregisterNodeDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlDeregisterNodeDefaultValue;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultBufferSize() -> *mut ::core::ffi::c_int {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDefaultBufferSize;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(::core::ffi::c_int) -> xmlGlobalStatePtr)(
            0 as ::core::ffi::c_int,
        ))
        .gs_xmlDefaultBufferSize;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserVersion() -> *const *const ::core::ffi::c_char {
    return &raw const xmlParserVersion;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCheckThreadLocalStorage() -> ::core::ffi::c_int {
    if xmlIsMainThreadInternal() == 0 && xmlGetThreadLocalStorage(1 as ::core::ffi::c_int).is_null()
    {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetGenericErrorFunc(
    mut ctx: *mut ::core::ffi::c_void,
    mut handler: xmlGenericErrorFunc,
) {
    xmlMutexLock(&raw mut xmlThrDefMutex);
    xmlGenericErrorContextThrDef = ctx;
    if handler.is_some() {
        xmlGenericErrorThrDef = handler;
    } else {
        xmlGenericErrorThrDef = Some(
            xmlGenericErrorDefaultFunc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as xmlGenericErrorFunc;
    }
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetStructuredErrorFunc(
    mut ctx: *mut ::core::ffi::c_void,
    mut handler: xmlStructuredErrorFunc,
) {
    xmlMutexLock(&raw mut xmlThrDefMutex);
    xmlStructuredErrorContextThrDef = ctx;
    xmlStructuredErrorThrDef = handler;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefBufferAllocScheme(
    mut v: xmlBufferAllocationScheme,
) -> xmlBufferAllocationScheme {
    let mut ret: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_DOUBLEIT;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlBufferAllocSchemeThrDef;
    xmlBufferAllocSchemeThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDefaultBufferSize(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlDefaultBufferSizeThrDef;
    xmlDefaultBufferSizeThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDoValidityCheckingDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlDoValidityCheckingDefaultValueThrDef;
    xmlDoValidityCheckingDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefGetWarningsDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlGetWarningsDefaultValueThrDef;
    xmlGetWarningsDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefIndentTreeOutput(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlIndentTreeOutputThrDef;
    xmlIndentTreeOutputThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefTreeIndentString(
    mut v: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut ret: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlTreeIndentStringThrDef;
    xmlTreeIndentStringThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSaveNoEmptyTags(mut v: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlSaveNoEmptyTagsThrDef;
    xmlSaveNoEmptyTagsThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefKeepBlanksDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlKeepBlanksDefaultValueThrDef;
    xmlKeepBlanksDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLineNumbersDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlLineNumbersDefaultValueThrDef;
    xmlLineNumbersDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLoadExtDtdDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlLoadExtDtdDefaultValueThrDef;
    xmlLoadExtDtdDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserDebugEntities(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlParserDebugEntitiesThrDef;
    xmlParserDebugEntitiesThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefPedanticParserDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlPedanticParserDefaultValueThrDef;
    xmlPedanticParserDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSubstituteEntitiesDefaultValue(
    mut v: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlSubstituteEntitiesDefaultValueThrDef;
    xmlSubstituteEntitiesDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc {
    let mut old: xmlRegisterNodeFunc = None;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    old = xmlRegisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
    xmlRegisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc {
    let mut old: xmlDeregisterNodeFunc = None;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    old = xmlDeregisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as ::core::ffi::c_int;
    xmlDeregisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserInputBufferCreateFilenameDefault(
    mut func: xmlParserInputBufferCreateFilenameFunc,
) -> xmlParserInputBufferCreateFilenameFunc {
    let mut old: xmlParserInputBufferCreateFilenameFunc = None;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    old = xmlParserInputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old = Some(
            __xmlParserInputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    xmlCharEncoding,
                ) -> xmlParserInputBufferPtr,
        ) as xmlParserInputBufferCreateFilenameFunc;
    }
    xmlParserInputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefOutputBufferCreateFilenameDefault(
    mut func: xmlOutputBufferCreateFilenameFunc,
) -> xmlOutputBufferCreateFilenameFunc {
    let mut old: xmlOutputBufferCreateFilenameFunc = None;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    old = xmlOutputBufferCreateFilenameValueThrDef;
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
    xmlOutputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return old;
}
pub const BASE_BUFFER_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> ::core::ffi::c_int {
    return (__thread1 == __thread2) as ::core::ffi::c_int;
}
