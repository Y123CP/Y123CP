use core::ffi::*;
use crate::src::c_inlined_fns::pthread_equal;
use crate::src::xmlstring::xmlCharStrdup;
use crate::src::threads::xmlInitParser;
use crate::src::error::xmlResetError;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
pub use crate::src::dict::_xmlDict;
pub use crate::src::buf::_xmlBuf;
extern "C" {
    fn __xmlParserInputBufferCreateFilename(
        URI: *const c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn __xmlOutputBufferCreateFilename(
        URI: *const c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: c_int,
    ) -> xmlOutputBufferPtr;
    fn xmlSAX2GetPublicId(ctx: *mut c_void) -> *const xmlChar;
    fn xmlSAX2GetSystemId(ctx: *mut c_void) -> *const xmlChar;
    fn xmlSAX2SetDocumentLocator(ctx: *mut c_void, loc: xmlSAXLocatorPtr);
    fn xmlSAX2GetColumnNumber(ctx: *mut c_void) -> c_int;
    fn xmlSAX2IsStandalone(ctx: *mut c_void) -> c_int;
    fn xmlSAX2HasInternalSubset(ctx: *mut c_void) -> c_int;
    fn xmlSAX2HasExternalSubset(ctx: *mut c_void) -> c_int;
    fn xmlSAX2InternalSubset(
        ctx: *mut c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlSAX2ExternalSubset(
        ctx: *mut c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlSAX2GetEntity(ctx: *mut c_void, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlSAX2GetParameterEntity(
        ctx: *mut c_void,
        name: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlSAX2ResolveEntity(
        ctx: *mut c_void,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    ) -> xmlParserInputPtr;
    fn xmlSAX2AttributeDecl(
        ctx: *mut c_void,
        elem: *const xmlChar,
        fullname: *const xmlChar,
        type_0: c_int,
        def: c_int,
        defaultValue: *const xmlChar,
        tree: xmlEnumerationPtr,
    );
    fn xmlSAX2ElementDecl(
        ctx: *mut c_void,
        name: *const xmlChar,
        type_0: c_int,
        content: xmlElementContentPtr,
    );
    fn xmlSAX2NotationDecl(
        ctx: *mut c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    );
    fn xmlSAX2UnparsedEntityDecl(
        ctx: *mut c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        notationName: *const xmlChar,
    );
    fn xmlSAX2StartDocument(ctx: *mut c_void);
    fn xmlSAX2EndDocument(ctx: *mut c_void);
    fn xmlSAX2Reference(ctx: *mut c_void, name: *const xmlChar);
    fn xmlSAX2Characters(
        ctx: *mut c_void,
        ch: *const xmlChar,
        len: c_int,
    );
    fn xmlSAX2ProcessingInstruction(
        ctx: *mut c_void,
        target: *const xmlChar,
        data: *const xmlChar,
    );
    fn xmlSAX2Comment(ctx: *mut c_void, value: *const xmlChar);
    fn xmlSAX2CDataBlock(
        ctx: *mut c_void,
        value: *const xmlChar,
        len: c_int,
    );
    fn xmlInitMutex(mutex: xmlMutexPtr);
    fn xmlCleanupMutex(mutex: xmlMutexPtr);
    fn xmlMutexLock(tok: xmlMutexPtr);
    fn xmlMutexUnlock(tok: xmlMutexPtr);
    fn xmlGenericErrorDefaultFunc(
        ctx: *mut c_void,
        msg: *const c_char,
        ...
    );
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    ) -> c_int;
    fn pthread_key_delete(__key: pthread_key_t) -> c_int;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const c_void,
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

pub type pthread_key_t = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40],
    pub __align: c_long,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: c_int,
    pub error: c_int,
    pub rawconsumed: c_ulong,
}
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;

pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputBuffer {
    pub context: *mut c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: c_int,
    pub error: c_int,
}

pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const c_char,
    pub directory: *const c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: c_int,
    pub line: c_int,
    pub col: c_int,
    pub consumed: c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub flags: c_int,
    pub id: c_int,
    pub parentConsumed: c_ulong,
    pub entity: xmlEntityPtr,
}
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEntity {
    pub _private: *mut c_void,
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
    pub length: c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: c_int,
    pub flags: c_int,
    pub expandedSize: c_ulong,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut c_void,
    pub type_0: xmlElementType,
    pub name: *mut c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: c_int,
    pub standalone: c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut c_void,
    pub refs: *mut c_void,
    pub URL: *const xmlChar,
    pub charset: c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut c_void,
    pub parseFlags: c_int,
    pub properties: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut c_void,
    pub context: *mut _xmlDoc,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut c_void,
    pub elements: *mut c_void,
    pub attributes: *mut c_void,
    pub entities: *mut c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut c_void,
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
    pub psvi: *mut c_void,
    pub line: c_ushort,
    pub extra: c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut c_void,
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
    pub psvi: *mut c_void,
}

pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;

pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;

pub type getParameterEntitySAXFunc =
    Option<unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr>;

pub type getEntitySAXFunc =
    Option<unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr>;
pub type resolveEntitySAXFunc = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlParserInputPtr,
>;

pub type xmlRegisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlDeregisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlGlobalStatePtr = *mut xmlGlobalState;
pub type xmlGlobalState = _xmlGlobalState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlGlobalState {
    pub initialized: c_int,
    pub gs_xmlLastError: xmlError,
    pub gs_xmlGenericError: xmlGenericErrorFunc,
    pub gs_xmlGenericErrorContext: *mut c_void,
    pub gs_xmlStructuredError: xmlStructuredErrorFunc,
    pub gs_xmlStructuredErrorContext: *mut c_void,
    pub gs_htmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub gs_xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc,
    pub gs_xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc,
    pub gs_oldXMLWDcompatibility: c_int,
    pub gs_xmlDefaultSAXLocator: xmlSAXLocator,
    pub gs_xmlDoValidityCheckingDefaultValue: c_int,
    pub gs_xmlGetWarningsDefaultValue: c_int,
    pub gs_xmlKeepBlanksDefaultValue: c_int,
    pub gs_xmlLineNumbersDefaultValue: c_int,
    pub gs_xmlLoadExtDtdDefaultValue: c_int,
    pub gs_xmlParserDebugEntities: c_int,
    pub gs_xmlPedanticParserDefaultValue: c_int,
    pub gs_xmlSubstituteEntitiesDefaultValue: c_int,
    pub gs_xmlIndentTreeOutput: c_int,
    pub gs_xmlTreeIndentString: *const c_char,
    pub gs_xmlSaveNoEmptyTags: c_int,
    pub gs_xmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub gs_xmlBufferAllocScheme: xmlBufferAllocationScheme,
    pub gs_xmlDefaultBufferSize: c_int,
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
    pub initialized: c_uint,
}
pub type xmlOutputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(
        *const c_char,
        xmlCharEncodingHandlerPtr,
        c_int,
    ) -> xmlOutputBufferPtr,
>;
pub type xmlParserInputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(*const c_char, xmlCharEncoding) -> xmlParserInputBufferPtr,
>;

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

pub type xmlMutex = _xmlMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlMutex {
    pub lock: pthread_mutex_t,
}
pub type xmlMutexPtr = *mut xmlMutex;

static mut parserInitialized: c_int = 0;
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
static mut libxml_is_threaded: c_int = -(1 as c_int);
static mut globalkey: pthread_key_t = 0;
static mut mainthread: pthread_t = 0;
#[no_mangle]
pub static mut xmlFree: xmlFreeFunc =
    Some(free as unsafe extern "C" fn(*mut c_void) -> ());
#[no_mangle]
pub static mut xmlMalloc: xmlMallocFunc =
    Some(malloc as unsafe extern "C" fn(size_t) -> *mut c_void);
#[no_mangle]
pub static mut xmlMallocAtomic: xmlMallocFunc =
    Some(malloc as unsafe extern "C" fn(size_t) -> *mut c_void);
#[no_mangle]
pub static mut xmlRealloc: xmlReallocFunc = Some(
        realloc
            as unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void,
    );
unsafe extern "C" fn xmlPosixStrdup(
    mut cur: *const c_char,
) -> *mut c_char {
    return xmlCharStrdup(cur) as *mut c_char;
}
#[no_mangle]
pub static mut xmlMemStrdup: xmlStrdupFunc = Some(
        xmlPosixStrdup
            as unsafe extern "C" fn(*const c_char) -> *mut c_char,
    );
#[no_mangle]
pub static mut xmlBufferAllocScheme: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_EXACT;
static mut xmlBufferAllocSchemeThrDef: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_EXACT;
#[no_mangle]
pub static mut xmlDefaultBufferSize: c_int = BASE_BUFFER_SIZE;
static mut xmlDefaultBufferSizeThrDef: c_int = BASE_BUFFER_SIZE;
#[no_mangle]
pub static mut oldXMLWDcompatibility: c_int = 0 as c_int;
#[no_mangle]
pub static mut xmlParserDebugEntities: c_int = 0 as c_int;
static mut xmlParserDebugEntitiesThrDef: c_int = 0 as c_int;
#[no_mangle]
pub static mut xmlDoValidityCheckingDefaultValue: c_int = 0 as c_int;
static mut xmlDoValidityCheckingDefaultValueThrDef: c_int = 0 as c_int;
#[no_mangle]
pub static mut xmlGetWarningsDefaultValue: c_int = 1 as c_int;
static mut xmlGetWarningsDefaultValueThrDef: c_int = 1 as c_int;
#[no_mangle]
pub static mut xmlLoadExtDtdDefaultValue: c_int = 0 as c_int;
static mut xmlLoadExtDtdDefaultValueThrDef: c_int = 0 as c_int;
#[no_mangle]
pub static mut xmlPedanticParserDefaultValue: c_int = 0 as c_int;
static mut xmlPedanticParserDefaultValueThrDef: c_int = 0 as c_int;
#[no_mangle]
pub static mut xmlLineNumbersDefaultValue: c_int = 0 as c_int;
static mut xmlLineNumbersDefaultValueThrDef: c_int = 0 as c_int;
#[no_mangle]
pub static mut xmlKeepBlanksDefaultValue: c_int = 1 as c_int;
static mut xmlKeepBlanksDefaultValueThrDef: c_int = 1 as c_int;
#[no_mangle]
pub static mut xmlSubstituteEntitiesDefaultValue: c_int = 0 as c_int;
static mut xmlSubstituteEntitiesDefaultValueThrDef: c_int = 0 as c_int;
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
pub static mut xmlGenericError: xmlGenericErrorFunc = Some(
        xmlGenericErrorDefaultFunc
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    );
static mut xmlGenericErrorThrDef: xmlGenericErrorFunc = Some(
        xmlGenericErrorDefaultFunc
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    );
#[no_mangle]
pub static mut xmlStructuredError: xmlStructuredErrorFunc = None;
static mut xmlStructuredErrorThrDef: xmlStructuredErrorFunc = None;
#[no_mangle]
pub static mut xmlGenericErrorContext: *mut c_void = NULL;
static mut xmlGenericErrorContextThrDef: *mut c_void = NULL;
#[no_mangle]
pub static mut xmlStructuredErrorContext: *mut c_void = NULL;
static mut xmlStructuredErrorContextThrDef: *mut c_void = NULL;
#[no_mangle]
pub static mut xmlLastError: xmlError = xmlError {
    domain: 0,
    code: 0,
    message: ::core::ptr::null::<c_char>() as *mut c_char,
    level: XML_ERR_NONE,
    file: ::core::ptr::null::<c_char>() as *mut c_char,
    line: 0,
    str1: ::core::ptr::null::<c_char>() as *mut c_char,
    str2: ::core::ptr::null::<c_char>() as *mut c_char,
    str3: ::core::ptr::null::<c_char>() as *mut c_char,
    int1: 0,
    int2: 0,
    ctxt: ::core::ptr::null::<c_void>() as *mut c_void,
    node: ::core::ptr::null::<c_void>() as *mut c_void,
};
#[no_mangle]
pub static mut xmlIndentTreeOutput: c_int = 1 as c_int;
static mut xmlIndentTreeOutputThrDef: c_int = 1 as c_int;
#[no_mangle]
pub static mut xmlTreeIndentString: *const c_char =
    b"  \0" as *const u8 as *const c_char;
static mut xmlTreeIndentStringThrDef: *const c_char =
    b"  \0" as *const u8 as *const c_char;
#[no_mangle]
pub static mut xmlSaveNoEmptyTags: c_int = 0 as c_int;
static mut xmlSaveNoEmptyTagsThrDef: c_int = 0 as c_int;
#[no_mangle]
pub static mut xmlDefaultSAXHandler: xmlSAXHandlerV1 = _xmlSAXHandlerV1 {
        internalSubset: Some(
            xmlSAX2InternalSubset
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        isStandalone: Some(
            xmlSAX2IsStandalone
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        hasInternalSubset: Some(
            xmlSAX2HasInternalSubset
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        hasExternalSubset: Some(
            xmlSAX2HasExternalSubset
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        resolveEntity: Some(
            xmlSAX2ResolveEntity
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> xmlParserInputPtr,
        ),
        getEntity: Some(
            xmlSAX2GetEntity
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: Some(
            xmlSAX2EntityDecl
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                    *const xmlChar,
                    *const xmlChar,
                    *mut xmlChar,
                ) -> (),
        ),
        notationDecl: Some(
            xmlSAX2NotationDecl
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        attributeDecl: Some(
            xmlSAX2AttributeDecl
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    c_int,
                    c_int,
                    *const xmlChar,
                    xmlEnumerationPtr,
                ) -> (),
        ),
        elementDecl: Some(
            xmlSAX2ElementDecl
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                    xmlElementContentPtr,
                ) -> (),
        ),
        unparsedEntityDecl: Some(
            xmlSAX2UnparsedEntityDecl
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        setDocumentLocator: Some(
            xmlSAX2SetDocumentLocator
                as unsafe extern "C" fn(*mut c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            xmlSAX2StartDocument as unsafe extern "C" fn(*mut c_void) -> (),
        ),
        endDocument: Some(
            xmlSAX2EndDocument as unsafe extern "C" fn(*mut c_void) -> (),
        ),
        startElement: Some(
            xmlSAX2StartElement
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *mut *const xmlChar,
                ) -> (),
        ),
        endElement: Some(
            xmlSAX2EndElement
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        reference: Some(
            xmlSAX2Reference
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        characters: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        processingInstruction: Some(
            xmlSAX2ProcessingInstruction
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        comment: Some(
            xmlSAX2Comment as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: Some(
            xmlSAX2GetParameterEntity
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        cdataBlock: Some(
            xmlSAX2CDataBlock
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        externalSubset: Some(
            xmlSAX2ExternalSubset
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        initialized: 1 as c_uint,
    };
#[no_mangle]
pub static mut xmlDefaultSAXLocator: xmlSAXLocator = _xmlSAXLocator {
        getPublicId: Some(
            xmlSAX2GetPublicId as unsafe extern "C" fn(*mut c_void) -> *const xmlChar,
        ),
        getSystemId: Some(
            xmlSAX2GetSystemId as unsafe extern "C" fn(*mut c_void) -> *const xmlChar,
        ),
        getLineNumber: Some(
            xmlSAX2GetLineNumber
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        getColumnNumber: Some(
            xmlSAX2GetColumnNumber
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
    };
#[no_mangle]
pub static mut htmlDefaultSAXHandler: xmlSAXHandlerV1 = _xmlSAXHandlerV1 {
        internalSubset: Some(
            xmlSAX2InternalSubset
                as unsafe extern "C" fn(
                    *mut c_void,
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
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: None,
        notationDecl: None,
        attributeDecl: None,
        elementDecl: None,
        unparsedEntityDecl: None,
        setDocumentLocator: Some(
            xmlSAX2SetDocumentLocator
                as unsafe extern "C" fn(*mut c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            xmlSAX2StartDocument as unsafe extern "C" fn(*mut c_void) -> (),
        ),
        endDocument: Some(
            xmlSAX2EndDocument as unsafe extern "C" fn(*mut c_void) -> (),
        ),
        startElement: Some(
            xmlSAX2StartElement
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *mut *const xmlChar,
                ) -> (),
        ),
        endElement: Some(
            xmlSAX2EndElement
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        reference: None,
        characters: Some(
            xmlSAX2Characters
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            xmlSAX2IgnorableWhitespace
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        processingInstruction: Some(
            xmlSAX2ProcessingInstruction
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        comment: Some(
            xmlSAX2Comment as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: None,
        cdataBlock: Some(
            xmlSAX2CDataBlock
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        externalSubset: None,
        initialized: 1 as c_uint,
    };
#[inline]
pub fn xmlInitGlobals() { {
    xmlInitParser();
} }
#[inline]
pub fn xmlInitGlobalsInternal() { unsafe {
    xmlInitMutex(&raw mut xmlThrDefMutex);
    if libxml_is_threaded == -(1 as c_int) {
        libxml_is_threaded = (Some(
            pthread_getspecific as unsafe extern "C" fn(pthread_key_t) -> *mut c_void,
        )
        .is_some()
            && Some(
                pthread_setspecific
                    as unsafe extern "C" fn(
                        pthread_key_t,
                        *const c_void,
                    ) -> c_int,
            )
            .is_some()
            && Some(
                pthread_key_create
                    as unsafe extern "C" fn(
                        *mut pthread_key_t,
                        Option<unsafe extern "C" fn(*mut c_void) -> ()>,
                    ) -> c_int,
            )
            .is_some()
            && Some(
                pthread_key_delete as unsafe extern "C" fn(pthread_key_t) -> c_int,
            )
            .is_some()
            && Some(pthread_self as unsafe extern "C" fn() -> pthread_t).is_some())
            as c_int;
    }
    if libxml_is_threaded == 0 as c_int {
        return;
    }
    pthread_key_create(
        &raw mut globalkey,
        Some(xmlFreeGlobalState as unsafe extern "C" fn(*mut c_void) -> ()),
    );
    mainthread = pthread_self();
} }
#[inline]
pub fn xmlCleanupGlobals() { {} }
#[inline]
pub fn xmlCleanupGlobalsInternal() { unsafe {
    xmlResetError(&raw mut xmlLastError);
    xmlCleanupMutex(&raw mut xmlThrDefMutex);
    if libxml_is_threaded == 0 as c_int {
        return;
    }
    pthread_key_delete(globalkey);
    parserInitialized = 0 as c_int;
} }
#[inline]
pub fn xmlInitializeGlobalState(mut gs: xmlGlobalStatePtr) { {} }
#[inline]
pub fn xmlGetGlobalState() -> xmlGlobalStatePtr { {
    return ::core::ptr::null_mut::<xmlGlobalState>();
} }
fn xmlIsMainThreadInternal() -> c_int { unsafe {
    if parserInitialized == 0 as c_int {
        xmlInitParser();
        parserInitialized = 1 as c_int;
    }
    if libxml_is_threaded == 0 as c_int {
        return 1 as c_int;
    }
    return pthread_equal(mainthread, pthread_self());
} }
#[inline]
pub fn xmlIsMainThread() -> c_int { {
    return xmlIsMainThreadInternal();
} }
unsafe extern "C" fn xmlFreeGlobalState(mut state: *mut c_void) {
    let mut gs: *mut xmlGlobalState = state as *mut xmlGlobalState;
    xmlResetError(&raw mut (*gs).gs_xmlLastError);
    free(state);
}
fn xmlInitGlobalState(mut gs: xmlGlobalStatePtr) { unsafe {
    xmlMutexLock(&raw mut xmlThrDefMutex);
    (*gs).gs_oldXMLWDcompatibility = 0 as c_int;
    (*gs).gs_xmlBufferAllocScheme = xmlBufferAllocSchemeThrDef;
    (*gs).gs_xmlDefaultBufferSize = xmlDefaultBufferSizeThrDef;
    (*gs).gs_xmlDefaultSAXLocator.getPublicId = Some(
        xmlSAX2GetPublicId as unsafe extern "C" fn(*mut c_void) -> *const xmlChar,
    )
        as Option<unsafe extern "C" fn(*mut c_void) -> *const xmlChar>;
    (*gs).gs_xmlDefaultSAXLocator.getSystemId = Some(
        xmlSAX2GetSystemId as unsafe extern "C" fn(*mut c_void) -> *const xmlChar,
    )
        as Option<unsafe extern "C" fn(*mut c_void) -> *const xmlChar>;
    (*gs).gs_xmlDefaultSAXLocator.getLineNumber = Some(
        xmlSAX2GetLineNumber
            as unsafe extern "C" fn(*mut c_void) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
    (*gs).gs_xmlDefaultSAXLocator.getColumnNumber = Some(
        xmlSAX2GetColumnNumber
            as unsafe extern "C" fn(*mut c_void) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
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
        &raw mut (*gs).gs_xmlLastError as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlError>() as size_t,
    );
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    pthread_setspecific(globalkey, gs as *const c_void);
    (*gs).initialized = 1 as c_int;
} }
fn xmlNewGlobalState(mut allowFailure: c_int) -> xmlGlobalStatePtr { unsafe {
    let mut gs: *mut xmlGlobalState = ::core::ptr::null_mut::<xmlGlobalState>();
    gs = malloc(::core::mem::size_of::<xmlGlobalState>() as size_t) as *mut xmlGlobalState;
    if gs.is_null() {
        if allowFailure != 0 {
            return ::core::ptr::null_mut::<xmlGlobalState>();
        }
        fprintf(
            stderr,
            b"libxml2: Failed to allocate globals for thread\nlibxml2: See xmlCheckThreadLocalStorage\n\0"
                as *const u8 as *const c_char,
        );
        abort();
    }
    memset(
        gs as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlGlobalState>() as size_t,
    );
    xmlInitGlobalState(gs as xmlGlobalStatePtr);
    return gs as xmlGlobalStatePtr;
} }
extern "C" fn xmlGetThreadLocalStorage(
    mut allowFailure: c_int,
) -> xmlGlobalStatePtr { unsafe {
    let mut gs: *mut xmlGlobalState = ::core::ptr::null_mut::<xmlGlobalState>();
    gs = pthread_getspecific(globalkey) as *mut xmlGlobalState;
    if gs.is_null() {
        gs = xmlNewGlobalState(allowFailure) as *mut xmlGlobalState;
    }
    return gs as xmlGlobalStatePtr;
} }
#[inline]
pub fn __xmlStructuredErrorContext() -> *mut *mut c_void { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlStructuredErrorContext;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlStructuredErrorContext;
    };
} }
#[inline]
pub fn __xmlStructuredError() -> *mut xmlStructuredErrorFunc { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlStructuredError;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlStructuredError;
    };
} }
#[inline]
pub fn __xmlLastError() -> *mut xmlError { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlLastError;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlLastError;
    };
} }
#[inline]
pub fn __xmlGenericError() -> *mut xmlGenericErrorFunc { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlGenericError;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlGenericError;
    };
} }
#[inline]
pub fn __xmlGenericErrorContext() -> *mut *mut c_void { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlGenericErrorContext;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlGenericErrorContext;
    };
} }
#[inline]
pub fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut htmlDefaultSAXHandler;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_htmlDefaultSAXHandler;
    };
} }
#[no_mangle]
pub extern "C" fn __xmlParserInputBufferCreateFilenameValue(
) -> *mut xmlParserInputBufferCreateFilenameFunc { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlParserInputBufferCreateFilenameValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlParserInputBufferCreateFilenameValue;
    };
} }
#[no_mangle]
pub extern "C" fn __xmlOutputBufferCreateFilenameValue(
) -> *mut xmlOutputBufferCreateFilenameFunc { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlOutputBufferCreateFilenameValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlOutputBufferCreateFilenameValue;
    };
} }
#[inline]
pub fn __xmlLineNumbersDefaultValue() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlLineNumbersDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlLineNumbersDefaultValue;
    };
} }
#[inline]
pub fn __xmlPedanticParserDefaultValue() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlPedanticParserDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlPedanticParserDefaultValue;
    };
} }
#[inline]
pub fn __xmlLoadExtDtdDefaultValue() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlLoadExtDtdDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlLoadExtDtdDefaultValue;
    };
} }
#[inline]
pub fn __xmlParserDebugEntities() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlParserDebugEntities;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlParserDebugEntities;
    };
} }
#[inline]
pub fn __xmlIndentTreeOutput() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlIndentTreeOutput;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlIndentTreeOutput;
    };
} }
#[inline]
pub fn __xmlTreeIndentString() -> *mut *const c_char { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlTreeIndentString;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlTreeIndentString;
    };
} }
#[inline]
pub fn __xmlSubstituteEntitiesDefaultValue() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlSubstituteEntitiesDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlSubstituteEntitiesDefaultValue;
    };
} }
#[inline]
pub fn __xmlSaveNoEmptyTags() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlSaveNoEmptyTags;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlSaveNoEmptyTags;
    };
} }
#[inline]
pub fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDefaultSAXHandler;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlDefaultSAXHandler;
    };
} }
#[inline]
pub fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDefaultSAXLocator;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlDefaultSAXLocator;
    };
} }
#[inline]
pub fn __oldXMLWDcompatibility() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut oldXMLWDcompatibility;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_oldXMLWDcompatibility;
    };
} }
#[inline]
pub fn __xmlDoValidityCheckingDefaultValue() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDoValidityCheckingDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlDoValidityCheckingDefaultValue;
    };
} }
#[inline]
pub fn __xmlGetWarningsDefaultValue() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlGetWarningsDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlGetWarningsDefaultValue;
    };
} }
#[inline]
pub fn __xmlKeepBlanksDefaultValue() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlKeepBlanksDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlKeepBlanksDefaultValue;
    };
} }
#[inline]
pub fn __xmlBufferAllocScheme() -> *mut xmlBufferAllocationScheme { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlBufferAllocScheme;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlBufferAllocScheme;
    };
} }
#[no_mangle]
pub extern "C" fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlRegisterNodeDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlRegisterNodeDefaultValue;
    };
} }
#[no_mangle]
pub extern "C" fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDeregisterNodeDefaultValue;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlDeregisterNodeDefaultValue;
    };
} }
#[inline]
pub fn __xmlDefaultBufferSize() -> *mut c_int { unsafe {
    if xmlIsMainThreadInternal() != 0 {
        return &raw mut xmlDefaultBufferSize;
    } else {
        return &raw mut (*(xmlGetThreadLocalStorage
            as unsafe extern "C" fn(c_int) -> xmlGlobalStatePtr)(
            0 as c_int,
        ))
        .gs_xmlDefaultBufferSize;
    };
} }
#[inline]
pub fn __xmlParserVersion() -> *const *const c_char { unsafe {
    return &raw const xmlParserVersion;
} }
#[inline]
pub fn xmlCheckThreadLocalStorage() -> c_int { {
    if xmlIsMainThreadInternal() == 0 && xmlGetThreadLocalStorage(1 as c_int).is_null()
    {
        return -(1 as c_int);
    }
    return 0 as c_int;
} }
#[inline]
pub unsafe fn xmlThrDefSetGenericErrorFunc(
    mut ctx: *mut c_void,
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
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlGenericErrorFunc;
    }
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
}
#[inline]
pub unsafe fn xmlThrDefSetStructuredErrorFunc(
    mut ctx: *mut c_void,
    mut handler: xmlStructuredErrorFunc,
) {
    xmlMutexLock(&raw mut xmlThrDefMutex);
    xmlStructuredErrorContextThrDef = ctx;
    xmlStructuredErrorThrDef = handler;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
}
#[inline]
pub fn xmlThrDefBufferAllocScheme(
    mut v: xmlBufferAllocationScheme,
) -> xmlBufferAllocationScheme { unsafe {
    let mut ret: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_DOUBLEIT;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlBufferAllocSchemeThrDef;
    xmlBufferAllocSchemeThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefDefaultBufferSize(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlDefaultBufferSizeThrDef;
    xmlDefaultBufferSizeThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefDoValidityCheckingDefaultValue(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlDoValidityCheckingDefaultValueThrDef;
    xmlDoValidityCheckingDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefGetWarningsDefaultValue(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlGetWarningsDefaultValueThrDef;
    xmlGetWarningsDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefIndentTreeOutput(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlIndentTreeOutputThrDef;
    xmlIndentTreeOutputThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub unsafe fn xmlThrDefTreeIndentString(
    mut v: *const c_char,
) -> *const c_char {
    let mut ret: *const c_char = ::core::ptr::null::<c_char>();
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlTreeIndentStringThrDef;
    xmlTreeIndentStringThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
}
#[inline]
pub fn xmlThrDefSaveNoEmptyTags(mut v: c_int) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlSaveNoEmptyTagsThrDef;
    xmlSaveNoEmptyTagsThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefKeepBlanksDefaultValue(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlKeepBlanksDefaultValueThrDef;
    xmlKeepBlanksDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefLineNumbersDefaultValue(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlLineNumbersDefaultValueThrDef;
    xmlLineNumbersDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefLoadExtDtdDefaultValue(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlLoadExtDtdDefaultValueThrDef;
    xmlLoadExtDtdDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefParserDebugEntities(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlParserDebugEntitiesThrDef;
    xmlParserDebugEntitiesThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefPedanticParserDefaultValue(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlPedanticParserDefaultValueThrDef;
    xmlPedanticParserDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefSubstituteEntitiesDefaultValue(
    mut v: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    ret = xmlSubstituteEntitiesDefaultValueThrDef;
    xmlSubstituteEntitiesDefaultValueThrDef = v;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return ret;
} }
#[inline]
pub fn xmlThrDefRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc { unsafe {
    let mut old: xmlRegisterNodeFunc = None;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    old = xmlRegisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as c_int;
    xmlRegisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return old;
} }
#[inline]
pub fn xmlThrDefDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc { unsafe {
    let mut old: xmlDeregisterNodeFunc = None;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    old = xmlDeregisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as c_int;
    xmlDeregisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return old;
} }
#[inline]
pub fn xmlThrDefParserInputBufferCreateFilenameDefault(
    mut func: xmlParserInputBufferCreateFilenameFunc,
) -> xmlParserInputBufferCreateFilenameFunc { unsafe {
    let mut old: xmlParserInputBufferCreateFilenameFunc = None;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    old = xmlParserInputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old = Some(
            __xmlParserInputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const c_char,
                    xmlCharEncoding,
                ) -> xmlParserInputBufferPtr,
        ) as xmlParserInputBufferCreateFilenameFunc;
    }
    xmlParserInputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return old;
} }
#[inline]
pub fn xmlThrDefOutputBufferCreateFilenameDefault(
    mut func: xmlOutputBufferCreateFilenameFunc,
) -> xmlOutputBufferCreateFilenameFunc { unsafe {
    let mut old: xmlOutputBufferCreateFilenameFunc = None;
    xmlMutexLock(&raw mut xmlThrDefMutex);
    old = xmlOutputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old = Some(
            __xmlOutputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const c_char,
                    xmlCharEncodingHandlerPtr,
                    c_int,
                ) -> xmlOutputBufferPtr,
        ) as xmlOutputBufferCreateFilenameFunc;
    }
    xmlOutputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(&raw mut xmlThrDefMutex);
    return old;
} }

