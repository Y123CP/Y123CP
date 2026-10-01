extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrndup(cur: *const xmlChar, len: ::core::ffi::c_int) -> *mut xmlChar;
    fn xmlCharStrdup(cur: *const ::core::ffi::c_char) -> *mut xmlChar;
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrncmp(
        str1: *const xmlChar,
        str2: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrlen(str: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlGetUTF8Char(
        utf: *const ::core::ffi::c_uchar,
        len: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlMallocAtomic: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFd(
        fd: ::core::ffi::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateMem(
        mem: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateStatic(
        mem: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::core::ffi::c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferGrow(
        in_0: xmlParserInputBufferPtr,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlParserInputBufferPush(
        in_0: xmlParserInputBufferPtr,
        len: ::core::ffi::c_int,
        buf: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    fn xmlParserGetDirectory(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn xmlFindCharEncodingHandler(name: *const ::core::ffi::c_char) -> xmlCharEncodingHandlerPtr;
    fn xmlNewDocElementContent(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: xmlElementContentType,
    ) -> xmlElementContentPtr;
    fn xmlFreeDocElementContent(doc: xmlDocPtr, cur: xmlElementContentPtr);
    fn xmlCreateEnumeration(name: *const xmlChar) -> xmlEnumerationPtr;
    fn xmlFreeEnumeration(cur: xmlEnumerationPtr);
    fn xmlValidateRoot(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> ::core::ffi::c_int;
    fn xmlValidateElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::core::ffi::c_int;
    fn xmlIsMixedElement(doc: xmlDocPtr, name: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlSAX2GetEntity(ctx: *mut ::core::ffi::c_void, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlSAX2EntityDecl(
        ctx: *mut ::core::ffi::c_void,
        name: *const xmlChar,
        type_0: ::core::ffi::c_int,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        content: *mut xmlChar,
    );
    fn xmlSAX2IgnorableWhitespace(
        ctx: *mut ::core::ffi::c_void,
        ch: *const xmlChar,
        len: ::core::ffi::c_int,
    );
    fn xmlDictSetLimit(dict: xmlDictPtr, limit: size_t) -> size_t;
    fn xmlDictReference(dict: xmlDictPtr) -> ::core::ffi::c_int;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> *const xmlChar;
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> ::core::ffi::c_int;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut ::core::ffi::c_void;
    fn xmlResetError(err: xmlErrorPtr);
    fn xmlCopyError(from: *const xmlError, to: xmlErrorPtr) -> ::core::ffi::c_int;
    fn __xmlParserDebugEntities() -> *mut ::core::ffi::c_int;
    fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1;
    fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator;
    fn xmlInitParser();
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    fn xmlNewSAXParserCtxt(
        sax: *const xmlSAXHandler,
        userData: *mut ::core::ffi::c_void,
    ) -> xmlParserCtxtPtr;
    fn xmlClearParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlNewIOInputStream(
        ctxt: xmlParserCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
    ) -> xmlParserInputPtr;
    fn xmlParserFindNodeInfo(ctxt: xmlParserCtxtPtr, node: xmlNodePtr) -> *const xmlParserNodeInfo;
    fn xmlInitNodeInfoSeq(seq: xmlParserNodeInfoSeqPtr);
    fn xmlParserAddNodeInfo(ctxt: xmlParserCtxtPtr, info: xmlParserNodeInfoPtr);
    fn xmlLoadExternalEntity(
        URL: *const ::core::ffi::c_char,
        ID: *const ::core::ffi::c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    fn xmlSplitQName3(name: *const xmlChar, len: *mut ::core::ffi::c_int) -> *const xmlChar;
    fn xmlCreateIntSubset(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    fn xmlNewDtd(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewDocComment(doc: xmlDocPtr, content: *const xmlChar) -> xmlNodePtr;
    fn xmlDocCopyNode(
        node: xmlNodePtr,
        doc: xmlDocPtr,
        recursive: ::core::ffi::c_int,
    ) -> xmlNodePtr;
    fn xmlGetLastChild(parent: *const xmlNode) -> xmlNodePtr;
    fn xmlNodeIsText(node: *const xmlNode) -> ::core::ffi::c_int;
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddChildList(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlFreeNodeList(cur: xmlNodePtr);
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlSetTreeDoc(tree: xmlNodePtr, doc: xmlDocPtr);
    fn xmlSearchNsByHref(doc: xmlDocPtr, node: xmlNodePtr, href: *const xmlChar) -> xmlNsPtr;
    fn htmlCreateMemoryParserCtxt(
        buffer: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> htmlParserCtxtPtr;
    fn xmlGetPredefinedEntity(name: *const xmlChar) -> xmlEntityPtr;
    fn xmlCharInRange(
        val: ::core::ffi::c_uint,
        group: *const xmlChRangeGroup,
    ) -> ::core::ffi::c_int;
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    static xmlIsCombiningGroup: xmlChRangeGroup;
    static xmlIsDigitGroup: xmlChRangeGroup;
    static xmlIsExtenderGroup: xmlChRangeGroup;
    static xmlIsPubidChar_tab: [::core::ffi::c_uchar; 256];
    fn xmlSwitchEncoding(ctxt: xmlParserCtxtPtr, enc: xmlCharEncoding) -> ::core::ffi::c_int;
    fn xmlSwitchToEncoding(
        ctxt: xmlParserCtxtPtr,
        handler: xmlCharEncodingHandlerPtr,
    ) -> ::core::ffi::c_int;
    fn xmlNewEntityInputStream(ctxt: xmlParserCtxtPtr, entity: xmlEntityPtr) -> xmlParserInputPtr;
    fn xmlFreeInputStream(input: xmlParserInputPtr);
    fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    fn xmlStringCurrentChar(
        ctxt: xmlParserCtxtPtr,
        cur: *const xmlChar,
        len: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlCurrentChar(ctxt: xmlParserCtxtPtr, len: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xmlCopyCharMultiByte(out: *mut xmlChar, val: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xmlCopyChar(
        len: ::core::ffi::c_int,
        out: *mut xmlChar,
        val: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlNextChar(ctxt: xmlParserCtxtPtr);
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlParseURI(str: *const ::core::ffi::c_char) -> xmlURIPtr;
    fn xmlFreeURI(uri: xmlURIPtr);
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    fn xmlCatalogFreeLocal(catalogs: *mut ::core::ffi::c_void);
    fn xmlCatalogAddLocal(
        catalogs: *mut ::core::ffi::c_void,
        URL: *const xmlChar,
    ) -> *mut ::core::ffi::c_void;
    fn xmlCatalogGetDefaults() -> xmlCatalogAllow;
    fn xmlBufIsEmpty(buf: xmlBufPtr) -> ::core::ffi::c_int;
    fn xmlBufDetach(buf: xmlBufPtr) -> *mut xmlChar;
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr) -> ::core::ffi::c_int;
    fn xmlBufUpdateInput(
        buf: xmlBufPtr,
        input: xmlParserInputPtr,
        pos: size_t,
    ) -> ::core::ffi::c_int;
    fn xmlDictComputeHash(dict: *const xmlDict, string: *const xmlChar) -> ::core::ffi::c_uint;
    fn xmlDictCombineHash(v1: ::core::ffi::c_uint, v2: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn xmlDictLookupHashed(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> xmlHashedString;
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
    fn __htmlParseContent(ctx: *mut ::core::ffi::c_void);
    fn xmlParserInputBufferCreateString(str: *const xmlChar) -> xmlParserInputBufferPtr;
    fn xmlErrMemory(ctxt: xmlParserCtxtPtr, extra: *const ::core::ffi::c_char);
    fn xmlFatalErr(
        ctxt: xmlParserCtxtPtr,
        error: xmlParserErrors,
        info: *const ::core::ffi::c_char,
    );
    fn __xmlErrEncoding(
        ctxt: xmlParserCtxtPtr,
        xmlerr: xmlParserErrors,
        msg: *const ::core::ffi::c_char,
        str1: *const xmlChar,
        str2: *const xmlChar,
    );
    fn xmlHaltParser(ctxt: xmlParserCtxtPtr);
    fn xmlParserGrow(ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int;
    fn xmlParserShrink(ctxt: xmlParserCtxtPtr);
    fn xmlDetectEncoding(ctxt: xmlParserCtxtPtr);
    fn xmlSetDeclaredEncoding(ctxt: xmlParserCtxtPtr, encoding: *mut xmlChar);
    fn xmlHashCreateDict(size: ::core::ffi::c_int, dict: xmlDictPtr) -> xmlHashTablePtr;
    fn xmlHashFree(hash: xmlHashTablePtr, dealloc: xmlHashDeallocator);
    fn xmlHashDefaultDeallocator(entry: *mut ::core::ffi::c_void, name: *const xmlChar);
    fn xmlHashAddEntry2(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn xmlHashUpdateEntry2(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut ::core::ffi::c_void,
        dealloc: xmlHashDeallocator,
    ) -> ::core::ffi::c_int;
    fn xmlHashRemoveEntry2(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        dealloc: xmlHashDeallocator,
    ) -> ::core::ffi::c_int;
    fn xmlHashLookup2(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
    ) -> *mut ::core::ffi::c_void;
    fn xmlHashQLookup2(
        hash: xmlHashTablePtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
        prefix2: *const xmlChar,
        name2: *const xmlChar,
    ) -> *mut ::core::ffi::c_void;
    fn xmlHashSize(hash: xmlHashTablePtr) -> ::core::ffi::c_int;
    fn xmlHashScanFull(
        hash: xmlHashTablePtr,
        scan: xmlHashScannerFull,
        data: *mut ::core::ffi::c_void,
    );
}
pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type xmlChar = ::core::ffi::c_uchar;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttrHashBucket {
    pub index: ::core::ffi::c_int,
}
pub type xmlParserNsData = _xmlParserNsData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNsData {
    pub extra: *mut xmlParserNsExtra,
    pub hashSize: ::core::ffi::c_uint,
    pub hashElems: ::core::ffi::c_uint,
    pub hash: *mut xmlParserNsBucket,
    pub elementId: ::core::ffi::c_uint,
    pub defaultNsIndex: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlParserNsBucket {
    pub hashValue: ::core::ffi::c_uint,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlParserNsExtra {
    pub saxData: *mut ::core::ffi::c_void,
    pub prefixHashValue: ::core::ffi::c_uint,
    pub uriHashValue: ::core::ffi::c_uint,
    pub elementId: ::core::ffi::c_uint,
    pub oldIndex: ::core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStartTag {
    pub prefix: *const xmlChar,
    pub URI: *const xmlChar,
    pub line: ::core::ffi::c_int,
    pub nsNr: ::core::ffi::c_int,
}
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
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const XML_ATTRIBUTE_FIXED: C2RustUnnamed = 4;
pub const XML_ATTRIBUTE_IMPLIED: C2RustUnnamed = 3;
pub const XML_ATTRIBUTE_REQUIRED: C2RustUnnamed = 2;
pub const XML_ATTRIBUTE_NONE: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const XML_ELEMENT_TYPE_ELEMENT: C2RustUnnamed_0 = 4;
pub const XML_ELEMENT_TYPE_MIXED: C2RustUnnamed_0 = 3;
pub const XML_ELEMENT_TYPE_ANY: C2RustUnnamed_0 = 2;
pub const XML_ELEMENT_TYPE_EMPTY: C2RustUnnamed_0 = 1;
pub const XML_ELEMENT_TYPE_UNDEFINED: C2RustUnnamed_0 = 0;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const XML_DOC_HTML: C2RustUnnamed_1 = 128;
pub const XML_DOC_INTERNAL: C2RustUnnamed_1 = 64;
pub const XML_DOC_USERBUILT: C2RustUnnamed_1 = 32;
pub const XML_DOC_XINCLUDE: C2RustUnnamed_1 = 16;
pub const XML_DOC_DTDVALID: C2RustUnnamed_1 = 8;
pub const XML_DOC_OLD10: C2RustUnnamed_1 = 4;
pub const XML_DOC_NSVALID: C2RustUnnamed_1 = 2;
pub const XML_DOC_WELLFORMED: C2RustUnnamed_1 = 1;
pub type xmlHashDeallocator =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type xmlHashScannerFull = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const XML_FROM_URI: C2RustUnnamed_2 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_2 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_2 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_2 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_2 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_2 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_2 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_2 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_2 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_2 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_2 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_2 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_2 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_2 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_2 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_2 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_2 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_2 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_2 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_2 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_2 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_2 = 9;
pub const XML_FROM_IO: C2RustUnnamed_2 = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed_2 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_2 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_2 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_2 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_2 = 3;
pub const XML_FROM_TREE: C2RustUnnamed_2 = 2;
pub const XML_FROM_PARSER: C2RustUnnamed_2 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_2 = 0;
pub type xmlErrorPtr = *mut xmlError;
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
pub const XML_IO_EAFNOSUPPORT: xmlParserErrors = 1556;
pub const XML_IO_EALREADY: xmlParserErrors = 1555;
pub const XML_IO_EADDRINUSE: xmlParserErrors = 1554;
pub const XML_IO_ENETUNREACH: xmlParserErrors = 1553;
pub const XML_IO_ECONNREFUSED: xmlParserErrors = 1552;
pub const XML_IO_EISCONN: xmlParserErrors = 1551;
pub const XML_IO_ENOTSOCK: xmlParserErrors = 1550;
pub const XML_IO_LOAD_ERROR: xmlParserErrors = 1549;
pub const XML_IO_BUFFER_FULL: xmlParserErrors = 1548;
pub const XML_IO_NO_INPUT: xmlParserErrors = 1547;
pub const XML_IO_WRITE: xmlParserErrors = 1546;
pub const XML_IO_FLUSH: xmlParserErrors = 1545;
pub const XML_IO_ENCODER: xmlParserErrors = 1544;
pub const XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1543;
pub const XML_IO_EXDEV: xmlParserErrors = 1542;
pub const XML_IO_ETIMEDOUT: xmlParserErrors = 1541;
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
pub const XML_IO_EINPROGRESS: xmlParserErrors = 1513;
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
pub const XML_IO_UNKNOWN: xmlParserErrors = 1500;
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
pub const XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub const XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const XML_ERR_OK: xmlParserErrors = 0;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
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
pub type xmlParserNodeInfoPtr = *mut xmlParserNodeInfo;
pub type xmlParserNodeInfoSeqPtr = *mut xmlParserNodeInfoSeq;
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
pub type xmlSAXHandlerV1 = _xmlSAXHandlerV1;
pub const XML_PARSE_OLD10: C2RustUnnamed_3 = 131072;
pub const XML_PARSE_HUGE: C2RustUnnamed_3 = 524288;
pub const XML_CATA_ALLOW_ALL: xmlCatalogAllow = 3;
pub type xmlCatalogAllow = ::core::ffi::c_uint;
pub const XML_CATA_ALLOW_DOCUMENT: xmlCatalogAllow = 2;
pub const XML_CATA_ALLOW_GLOBAL: xmlCatalogAllow = 1;
pub const XML_CATA_ALLOW_NONE: xmlCatalogAllow = 0;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_3 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_3 = 4;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_3 = 16;
pub const XML_PARSE_NOENT: C2RustUnnamed_3 = 2;
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: ::core::ffi::c_int,
    pub nbLongRange: ::core::ffi::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: ::core::ffi::c_uint,
    pub high: ::core::ffi::c_uint,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: ::core::ffi::c_ushort,
    pub high: ::core::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlHashedString {
    pub hashValue: ::core::ffi::c_uint,
    pub name: *const xmlChar,
}
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_3 = 8192;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_3 = 1048576;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlDefAttr {
    pub prefix: xmlHashedString,
    pub name: xmlHashedString,
    pub value: xmlHashedString,
    pub valueEnd: *const xmlChar,
    pub external: ::core::ffi::c_int,
    pub expandedSize: ::core::ffi::c_int,
}
pub type xmlDefAttrsPtr = *mut xmlDefAttrs;
pub type xmlDefAttrs = _xmlDefAttrs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDefAttrs {
    pub nbAttrs: ::core::ffi::c_int,
    pub maxAttrs: ::core::ffi::c_int,
    pub attrs: [xmlDefAttr; 0],
}
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
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_3 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_3 = 2097152;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_3 = 262144;
pub const XML_PARSE_COMPACT: C2RustUnnamed_3 = 65536;
pub const XML_PARSE_NONET: C2RustUnnamed_3 = 2048;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_3 = 16384;
pub const XML_PARSE_NODICT: C2RustUnnamed_3 = 4096;
pub const XML_PARSE_SAX1: C2RustUnnamed_3 = 512;
pub const XML_PARSE_NOERROR: C2RustUnnamed_3 = 32;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_3 = 64;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_3 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_3 = 128;
pub const XML_PARSE_RECOVER: C2RustUnnamed_3 = 1;
pub const HTML_PARSE_NOIMPLIED: C2RustUnnamed_4 = 8192;
pub type htmlParserCtxtPtr = xmlParserCtxtPtr;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_3 = 32768;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_3 = 1024;
pub type xmlFeature = ::core::ffi::c_uint;
pub const XML_WITH_NONE: xmlFeature = 99999;
pub const XML_WITH_LZMA: xmlFeature = 33;
pub const XML_WITH_ICU: xmlFeature = 32;
pub const XML_WITH_ZLIB: xmlFeature = 31;
pub const XML_WITH_DEBUG_RUN: xmlFeature = 30;
pub const XML_WITH_DEBUG_MEM: xmlFeature = 29;
pub const XML_WITH_DEBUG: xmlFeature = 28;
pub const XML_WITH_MODULES: xmlFeature = 27;
pub const XML_WITH_SCHEMATRON: xmlFeature = 26;
pub const XML_WITH_SCHEMAS: xmlFeature = 25;
pub const XML_WITH_EXPR: xmlFeature = 24;
pub const XML_WITH_AUTOMATA: xmlFeature = 23;
pub const XML_WITH_REGEXP: xmlFeature = 22;
pub const XML_WITH_UNICODE: xmlFeature = 21;
pub const XML_WITH_ISO8859X: xmlFeature = 20;
pub const XML_WITH_ICONV: xmlFeature = 19;
pub const XML_WITH_XINCLUDE: xmlFeature = 18;
pub const XML_WITH_XPTR: xmlFeature = 17;
pub const XML_WITH_XPATH: xmlFeature = 16;
pub const XML_WITH_CATALOG: xmlFeature = 15;
pub const XML_WITH_C14N: xmlFeature = 14;
pub const XML_WITH_LEGACY: xmlFeature = 13;
pub const XML_WITH_HTML: xmlFeature = 12;
pub const XML_WITH_VALID: xmlFeature = 11;
pub const XML_WITH_HTTP: xmlFeature = 10;
pub const XML_WITH_FTP: xmlFeature = 9;
pub const XML_WITH_SAX1: xmlFeature = 8;
pub const XML_WITH_WRITER: xmlFeature = 7;
pub const XML_WITH_PATTERN: xmlFeature = 6;
pub const XML_WITH_READER: xmlFeature = 5;
pub const XML_WITH_PUSH: xmlFeature = 4;
pub const XML_WITH_OUTPUT: xmlFeature = 3;
pub const XML_WITH_TREE: xmlFeature = 2;
pub const XML_WITH_THREAD: xmlFeature = 1;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const HTML_PARSE_IGNORE_ENC: C2RustUnnamed_4 = 2097152;
pub const HTML_PARSE_COMPACT: C2RustUnnamed_4 = 65536;
pub const HTML_PARSE_NONET: C2RustUnnamed_4 = 2048;
pub const HTML_PARSE_NOBLANKS: C2RustUnnamed_4 = 256;
pub const HTML_PARSE_PEDANTIC: C2RustUnnamed_4 = 128;
pub const HTML_PARSE_NOWARNING: C2RustUnnamed_4 = 64;
pub const HTML_PARSE_NOERROR: C2RustUnnamed_4 = 32;
pub const HTML_PARSE_NODEFDTD: C2RustUnnamed_4 = 4;
pub const HTML_PARSE_RECOVER: C2RustUnnamed_4 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NS_INDEX_EMPTY: ::core::ffi::c_int = INT_MAX;
pub const NS_INDEX_XML: ::core::ffi::c_int = INT_MAX - 1 as ::core::ffi::c_int;
pub const URI_HASH_EMPTY: ::core::ffi::c_uint = 0xd943a04e as ::core::ffi::c_uint;
pub const URI_HASH_XML: ::core::ffi::c_uint = 0xf0451f02 as ::core::ffi::c_uint;
pub const XML_PARSER_ALLOWED_EXPANSION: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
pub const XML_ENT_FIXED_COST: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlParserMaxDepth: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
pub const XML_PARSER_BIG_BUFFER_SIZE: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
pub const XML_PARSER_BUFFER_SIZE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
#[no_mangle]
pub static mut xmlParserVersion: *const ::core::ffi::c_char =
    b"21205\0" as *const u8 as *const ::core::ffi::c_char;
static mut xmlW3CPIs: [*const ::core::ffi::c_char; 3] = [
    b"xml-stylesheet\0" as *const u8 as *const ::core::ffi::c_char,
    b"xml-model\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn xmlErrAttributeDup(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *const xmlChar,
    mut localname: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = XML_ERR_ATTRIBUTE_REDEFINED as ::core::ffi::c_int;
    }
    if prefix.is_null() {
        __xmlRaiseError(
            None,
            None,
            NULL,
            ctxt as *mut ::core::ffi::c_void,
            NULL,
            XML_FROM_PARSER as ::core::ffi::c_int,
            XML_ERR_ATTRIBUTE_REDEFINED as ::core::ffi::c_int,
            XML_ERR_FATAL,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            localname as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"Attribute %s redefined\n\0" as *const u8 as *const ::core::ffi::c_char,
            localname,
        );
    } else {
        __xmlRaiseError(
            None,
            None,
            NULL,
            ctxt as *mut ::core::ffi::c_void,
            NULL,
            XML_FROM_PARSER as ::core::ffi::c_int,
            XML_ERR_ATTRIBUTE_REDEFINED as ::core::ffi::c_int,
            XML_ERR_FATAL,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            prefix as *const ::core::ffi::c_char,
            localname as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"Attribute %s:%s redefined\n\0" as *const u8 as *const ::core::ffi::c_char,
            prefix,
            localname,
        );
    }
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as ::core::ffi::c_int;
        if (*ctxt).recovery == 0 as ::core::ffi::c_int {
            (*ctxt).disableSAX = 1 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn xmlFatalErrMsg(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as ::core::ffi::c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        NULL,
        XML_FROM_PARSER as ::core::ffi::c_int,
        error as ::core::ffi::c_int,
        XML_ERR_FATAL,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        msg,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as ::core::ffi::c_int;
        if (*ctxt).recovery == 0 as ::core::ffi::c_int {
            (*ctxt).disableSAX = 1 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlWarningMsg(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() && !(*ctxt).sax.is_null() && (*(*ctxt).sax).initialized == XML_SAX2_MAGIC {
        schannel = (*(*ctxt).sax).serror;
    }
    if !ctxt.is_null() {
        __xmlRaiseError(
            schannel,
            if !(*ctxt).sax.is_null() {
                (*(*ctxt).sax).warning as xmlGenericErrorFunc
            } else {
                None
            },
            (*ctxt).userData,
            ctxt as *mut ::core::ffi::c_void,
            NULL,
            XML_FROM_PARSER as ::core::ffi::c_int,
            error as ::core::ffi::c_int,
            XML_ERR_WARNING,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            str1 as *const ::core::ffi::c_char,
            str2 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            msg,
            str1 as *const ::core::ffi::c_char,
            str2 as *const ::core::ffi::c_char,
        );
    } else {
        __xmlRaiseError(
            schannel,
            None,
            NULL,
            ctxt as *mut ::core::ffi::c_void,
            NULL,
            XML_FROM_PARSER as ::core::ffi::c_int,
            error as ::core::ffi::c_int,
            XML_ERR_WARNING,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            str1 as *const ::core::ffi::c_char,
            str2 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            msg,
            str1 as *const ::core::ffi::c_char,
            str2 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn xmlValidityError(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as ::core::ffi::c_int;
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).initialized == XML_SAX2_MAGIC {
            schannel = (*(*ctxt).sax).serror;
        }
    }
    if !ctxt.is_null() {
        __xmlRaiseError(
            schannel,
            (*ctxt).vctxt.error as xmlGenericErrorFunc,
            (*ctxt).vctxt.userData,
            ctxt as *mut ::core::ffi::c_void,
            NULL,
            XML_FROM_DTD as ::core::ffi::c_int,
            error as ::core::ffi::c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            str1 as *const ::core::ffi::c_char,
            str2 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            msg,
            str1 as *const ::core::ffi::c_char,
            str2 as *const ::core::ffi::c_char,
        );
        (*ctxt).valid = 0 as ::core::ffi::c_int;
    } else {
        __xmlRaiseError(
            schannel,
            None,
            NULL,
            ctxt as *mut ::core::ffi::c_void,
            NULL,
            XML_FROM_DTD as ::core::ffi::c_int,
            error as ::core::ffi::c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            str1 as *const ::core::ffi::c_char,
            str2 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            msg,
            str1 as *const ::core::ffi::c_char,
            str2 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn xmlFatalErrMsgInt(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
    mut val: ::core::ffi::c_int,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as ::core::ffi::c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        NULL,
        XML_FROM_PARSER as ::core::ffi::c_int,
        error as ::core::ffi::c_int,
        XML_ERR_FATAL,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        val,
        0 as ::core::ffi::c_int,
        msg,
        val,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as ::core::ffi::c_int;
        if (*ctxt).recovery == 0 as ::core::ffi::c_int {
            (*ctxt).disableSAX = 1 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn xmlFatalErrMsgStrIntStr(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
    mut str1: *const xmlChar,
    mut val: ::core::ffi::c_int,
    mut str2: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as ::core::ffi::c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        NULL,
        XML_FROM_PARSER as ::core::ffi::c_int,
        error as ::core::ffi::c_int,
        XML_ERR_FATAL,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        str1 as *const ::core::ffi::c_char,
        str2 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        val,
        0 as ::core::ffi::c_int,
        msg,
        str1,
        val,
        str2,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as ::core::ffi::c_int;
        if (*ctxt).recovery == 0 as ::core::ffi::c_int {
            (*ctxt).disableSAX = 1 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn xmlFatalErrMsgStr(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
    mut val: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as ::core::ffi::c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        NULL,
        XML_FROM_PARSER as ::core::ffi::c_int,
        error as ::core::ffi::c_int,
        XML_ERR_FATAL,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        val as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        msg,
        val,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as ::core::ffi::c_int;
        if (*ctxt).recovery == 0 as ::core::ffi::c_int {
            (*ctxt).disableSAX = 1 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn xmlErrMsgStr(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
    mut val: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as ::core::ffi::c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        NULL,
        XML_FROM_PARSER as ::core::ffi::c_int,
        error as ::core::ffi::c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        val as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        msg,
        val,
    );
}
unsafe extern "C" fn xmlNsErr(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
    mut info1: *const xmlChar,
    mut info2: *const xmlChar,
    mut info3: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as ::core::ffi::c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        NULL,
        XML_FROM_NAMESPACE as ::core::ffi::c_int,
        error as ::core::ffi::c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        info1 as *const ::core::ffi::c_char,
        info2 as *const ::core::ffi::c_char,
        info3 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        msg,
        info1,
        info2,
        info3,
    );
    if !ctxt.is_null() {
        (*ctxt).nsWellFormed = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn xmlNsWarn(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const ::core::ffi::c_char,
    mut info1: *const xmlChar,
    mut info2: *const xmlChar,
    mut info3: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        NULL,
        XML_FROM_NAMESPACE as ::core::ffi::c_int,
        error as ::core::ffi::c_int,
        XML_ERR_WARNING,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        info1 as *const ::core::ffi::c_char,
        info2 as *const ::core::ffi::c_char,
        info3 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        msg,
        info1,
        info2,
        info3,
    );
}
unsafe extern "C" fn xmlSaturatedAdd(
    mut dst: *mut ::core::ffi::c_ulong,
    mut val: ::core::ffi::c_ulong,
) {
    if val > ULONG_MAX.wrapping_sub(*dst) {
        *dst = ULONG_MAX;
    } else {
        *dst = (*dst).wrapping_add(val);
    };
}
unsafe extern "C" fn xmlSaturatedAddSizeT(
    mut dst: *mut ::core::ffi::c_ulong,
    mut val: ::core::ffi::c_ulong,
) {
    if val > ULONG_MAX.wrapping_sub(*dst) {
        *dst = ULONG_MAX;
    } else {
        *dst = (*dst).wrapping_add(val);
    };
}
unsafe extern "C" fn xmlParserEntityCheck(
    mut ctxt: xmlParserCtxtPtr,
    mut extra: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let mut consumed: ::core::ffi::c_ulong = 0;
    let mut input: xmlParserInputPtr = (*ctxt).input;
    let mut entity: xmlEntityPtr = (*input).entity;
    consumed = (*input).parentConsumed;
    if entity.is_null()
        || (*entity).etype as ::core::ffi::c_uint
            == XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*entity).flags & XML_ENT_PARSED == 0 as ::core::ffi::c_int
    {
        xmlSaturatedAdd(&raw mut consumed, (*input).consumed);
        xmlSaturatedAddSizeT(
            &raw mut consumed,
            (*input).cur.offset_from((*input).base) as ::core::ffi::c_long as ::core::ffi::c_ulong,
        );
    }
    xmlSaturatedAdd(&raw mut consumed, (*ctxt).sizeentities);
    xmlSaturatedAdd(&raw mut (*ctxt).sizeentcopy, extra);
    xmlSaturatedAdd(
        &raw mut (*ctxt).sizeentcopy,
        XML_ENT_FIXED_COST as ::core::ffi::c_ulong,
    );
    if (*ctxt).sizeentcopy > XML_PARSER_ALLOWED_EXPANSION as ::core::ffi::c_ulong
        && ((*ctxt).sizeentcopy >= ULONG_MAX
            || (*ctxt)
                .sizeentcopy
                .wrapping_div((*ctxt).maxAmpl as ::core::ffi::c_ulong)
                > consumed)
    {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_ENTITY_LOOP,
            b"Maximum entity amplification factor exceeded, see xmlCtxtSetMaxAmplification.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        xmlHaltParser(ctxt);
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHasFeature(mut feature: xmlFeature) -> ::core::ffi::c_int {
    match feature as ::core::ffi::c_uint {
        1 => return 1 as ::core::ffi::c_int,
        2 => return 1 as ::core::ffi::c_int,
        3 => return 1 as ::core::ffi::c_int,
        4 => return 1 as ::core::ffi::c_int,
        5 => return 1 as ::core::ffi::c_int,
        6 => return 1 as ::core::ffi::c_int,
        7 => return 1 as ::core::ffi::c_int,
        8 => return 1 as ::core::ffi::c_int,
        9 => return 0 as ::core::ffi::c_int,
        10 => return 1 as ::core::ffi::c_int,
        11 => return 1 as ::core::ffi::c_int,
        12 => return 1 as ::core::ffi::c_int,
        13 => return 0 as ::core::ffi::c_int,
        14 => return 1 as ::core::ffi::c_int,
        15 => return 1 as ::core::ffi::c_int,
        16 => return 1 as ::core::ffi::c_int,
        17 => return 1 as ::core::ffi::c_int,
        18 => return 1 as ::core::ffi::c_int,
        19 => return 0 as ::core::ffi::c_int,
        20 => return 1 as ::core::ffi::c_int,
        21 => return 1 as ::core::ffi::c_int,
        22 => return 1 as ::core::ffi::c_int,
        23 => return 1 as ::core::ffi::c_int,
        24 => return 0 as ::core::ffi::c_int,
        25 => return 1 as ::core::ffi::c_int,
        26 => return 1 as ::core::ffi::c_int,
        27 => return 1 as ::core::ffi::c_int,
        28 => return 1 as ::core::ffi::c_int,
        29 => return 0 as ::core::ffi::c_int,
        30 => return 0 as ::core::ffi::c_int,
        31 => return 0 as ::core::ffi::c_int,
        33 => return 0 as ::core::ffi::c_int,
        32 => return 0 as ::core::ffi::c_int,
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlDetectSAX2(mut ctxt: xmlParserCtxtPtr) {
    let mut sax: xmlSAXHandlerPtr = ::core::ptr::null_mut::<xmlSAXHandler>();
    if ctxt.is_null() {
        return;
    }
    sax = (*ctxt).sax as xmlSAXHandlerPtr;
    if !sax.is_null()
        && (*sax).initialized == XML_SAX2_MAGIC
        && ((*sax).startElementNs.is_some()
            || (*sax).endElementNs.is_some()
            || (*sax).startElement.is_none() && (*sax).endElement.is_none())
    {
        (*ctxt).sax2 = 1 as ::core::ffi::c_int;
    }
    (*ctxt).str_xml = xmlDictLookup(
        (*ctxt).dict,
        b"xml\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        3 as ::core::ffi::c_int,
    );
    (*ctxt).str_xmlns = xmlDictLookup(
        (*ctxt).dict,
        b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        5 as ::core::ffi::c_int,
    );
    (*ctxt).str_xml_ns = xmlDictLookup((*ctxt).dict, XML_XML_NAMESPACE, 36 as ::core::ffi::c_int);
    if (*ctxt).str_xml.is_null() || (*ctxt).str_xmlns.is_null() || (*ctxt).str_xml_ns.is_null() {
        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
    }
}
unsafe extern "C" fn xmlAttrNormalizeSpace(
    mut src: *const xmlChar,
    mut dst: *mut xmlChar,
) -> *mut xmlChar {
    if src.is_null() || dst.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while *src as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int {
        src = src.offset(1);
    }
    while *src as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *src as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int {
            while *src as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int {
                src = src.offset(1);
            }
            if *src as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                let fresh99 = dst;
                dst = dst.offset(1);
                *fresh99 = 0x20 as xmlChar;
            }
        } else {
            let fresh100 = src;
            src = src.offset(1);
            let fresh101 = dst;
            dst = dst.offset(1);
            *fresh101 = *fresh100;
        }
    }
    *dst = 0 as xmlChar;
    if dst == src as *mut xmlChar {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return dst;
}
unsafe extern "C" fn xmlAttrNormalizeSpace2(
    mut ctxt: xmlParserCtxtPtr,
    mut src: *mut xmlChar,
    mut len: *mut ::core::ffi::c_int,
) -> *const xmlChar {
    let mut i: ::core::ffi::c_int = 0;
    let mut remove_head: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut need_realloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if ctxt.is_null() || src.is_null() || len.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    i = *len;
    if i <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    cur = src;
    while *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int {
        cur = cur.offset(1);
        remove_head += 1;
    }
    while *cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int {
            cur = cur.offset(1);
            if !(*cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            {
                continue;
            }
            need_realloc = 1 as ::core::ffi::c_int;
            break;
        } else {
            cur = cur.offset(1);
        }
    }
    if need_realloc != 0 {
        let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        ret = xmlStrndup(
            src.offset(remove_head as isize),
            i - remove_head + 1 as ::core::ffi::c_int,
        );
        if ret.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return ::core::ptr::null::<xmlChar>();
        }
        xmlAttrNormalizeSpace(ret, ret);
        *len = strlen(ret as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        return ret;
    } else if remove_head != 0 {
        *len -= remove_head;
        memmove(
            src as *mut ::core::ffi::c_void,
            src.offset(remove_head as isize) as *const ::core::ffi::c_void,
            (1 as ::core::ffi::c_int + *len) as size_t,
        );
        return src;
    }
    return ::core::ptr::null::<xmlChar>();
}
unsafe extern "C" fn xmlAddDefAttrs(
    mut ctxt: xmlParserCtxtPtr,
    mut fullname: *const xmlChar,
    mut fullattr: *const xmlChar,
    mut value: *const xmlChar,
) {
    let mut current_block: u64;
    let mut defaults: xmlDefAttrsPtr = ::core::ptr::null_mut::<xmlDefAttrs>();
    let mut attr: *mut xmlDefAttr = ::core::ptr::null_mut::<xmlDefAttr>();
    let mut len: ::core::ffi::c_int = 0;
    let mut expandedSize: ::core::ffi::c_int = 0;
    let mut name: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut prefix: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut hvalue: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut localname: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if !(*ctxt).attsSpecial.is_null() {
        if !xmlHashLookup2((*ctxt).attsSpecial, fullname, fullattr).is_null() {
            return;
        }
    }
    if (*ctxt).attsDefault.is_null() {
        (*ctxt).attsDefault = xmlHashCreateDict(10 as ::core::ffi::c_int, (*ctxt).dict);
        if (*ctxt).attsDefault.is_null() {
            current_block = 619953177835619926;
        } else {
            current_block = 13513818773234778473;
        }
    } else {
        current_block = 13513818773234778473;
    }
    match current_block {
        13513818773234778473 => {
            localname = xmlSplitQName3(fullname, &raw mut len);
            if localname.is_null() {
                name = xmlDictLookupHashed((*ctxt).dict, fullname, -(1 as ::core::ffi::c_int));
                prefix.name = ::core::ptr::null::<xmlChar>();
                current_block = 7976072742316086414;
            } else {
                name = xmlDictLookupHashed((*ctxt).dict, localname, -(1 as ::core::ffi::c_int));
                prefix = xmlDictLookupHashed((*ctxt).dict, fullname, len);
                if prefix.name.is_null() {
                    current_block = 619953177835619926;
                } else {
                    current_block = 7976072742316086414;
                }
            }
            match current_block {
                619953177835619926 => {}
                _ => {
                    if !name.name.is_null() {
                        defaults = xmlHashLookup2((*ctxt).attsDefault, name.name, prefix.name)
                            as xmlDefAttrsPtr;
                        if defaults.is_null() || (*defaults).nbAttrs >= (*defaults).maxAttrs {
                            let mut temp: xmlDefAttrsPtr = ::core::ptr::null_mut::<xmlDefAttrs>();
                            let mut newSize: ::core::ffi::c_int = 0;
                            newSize = if !defaults.is_null() {
                                2 as ::core::ffi::c_int * (*defaults).maxAttrs
                            } else {
                                4 as ::core::ffi::c_int
                            };
                            temp = xmlRealloc.expect("non-null function pointer")(
                                defaults as *mut ::core::ffi::c_void,
                                (::core::mem::size_of::<xmlDefAttrs>() as size_t).wrapping_add(
                                    (newSize as size_t).wrapping_mul(::core::mem::size_of::<
                                        xmlDefAttr,
                                    >(
                                    )
                                        as size_t),
                                ),
                            ) as xmlDefAttrsPtr;
                            if temp.is_null() {
                                current_block = 619953177835619926;
                            } else {
                                if defaults.is_null() {
                                    (*temp).nbAttrs = 0 as ::core::ffi::c_int;
                                }
                                (*temp).maxAttrs = newSize;
                                defaults = temp;
                                if xmlHashUpdateEntry2(
                                    (*ctxt).attsDefault,
                                    name.name,
                                    prefix.name,
                                    defaults as *mut ::core::ffi::c_void,
                                    None,
                                ) < 0 as ::core::ffi::c_int
                                {
                                    xmlFree.expect("non-null function pointer")(
                                        defaults as *mut ::core::ffi::c_void,
                                    );
                                    current_block = 619953177835619926;
                                } else {
                                    current_block = 6669252993407410313;
                                }
                            }
                        } else {
                            current_block = 6669252993407410313;
                        }
                        match current_block {
                            619953177835619926 => {}
                            _ => {
                                localname = xmlSplitQName3(fullattr, &raw mut len);
                                if localname.is_null() {
                                    name = xmlDictLookupHashed(
                                        (*ctxt).dict,
                                        fullattr,
                                        -(1 as ::core::ffi::c_int),
                                    );
                                    prefix.name = ::core::ptr::null::<xmlChar>();
                                    current_block = 3275366147856559585;
                                } else {
                                    name = xmlDictLookupHashed(
                                        (*ctxt).dict,
                                        localname,
                                        -(1 as ::core::ffi::c_int),
                                    );
                                    prefix = xmlDictLookupHashed((*ctxt).dict, fullattr, len);
                                    if prefix.name.is_null() {
                                        current_block = 619953177835619926;
                                    } else {
                                        current_block = 3275366147856559585;
                                    }
                                }
                                match current_block {
                                    619953177835619926 => {}
                                    _ => {
                                        if !name.name.is_null() {
                                            len = strlen(value as *const ::core::ffi::c_char)
                                                as ::core::ffi::c_int;
                                            hvalue = xmlDictLookupHashed((*ctxt).dict, value, len);
                                            if !hvalue.name.is_null() {
                                                expandedSize =
                                                    strlen(name.name as *const ::core::ffi::c_char)
                                                        as ::core::ffi::c_int;
                                                if !prefix.name.is_null() {
                                                    expandedSize = (expandedSize
                                                        as ::core::ffi::c_ulong)
                                                        .wrapping_add(strlen(
                                                            prefix.name
                                                                as *const ::core::ffi::c_char,
                                                        )
                                                            as ::core::ffi::c_ulong)
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_int;
                                                }
                                                expandedSize += len;
                                                let fresh106 = (*defaults).nbAttrs;
                                                (*defaults).nbAttrs = (*defaults).nbAttrs + 1;
                                                attr = (&raw mut (*defaults).attrs
                                                    as *mut xmlDefAttr)
                                                    .offset(fresh106 as isize)
                                                    as *mut xmlDefAttr;
                                                (*attr).name = name;
                                                (*attr).prefix = prefix;
                                                (*attr).value = hvalue;
                                                (*attr).valueEnd = hvalue.name.offset(len as isize);
                                                (*attr).external = (*ctxt).external;
                                                (*attr).expandedSize = expandedSize;
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
}
unsafe extern "C" fn xmlAddSpecialAttr(
    mut ctxt: xmlParserCtxtPtr,
    mut fullname: *const xmlChar,
    mut fullattr: *const xmlChar,
    mut type_0: ::core::ffi::c_int,
) {
    if (*ctxt).attsSpecial.is_null() {
        (*ctxt).attsSpecial = xmlHashCreateDict(10 as ::core::ffi::c_int, (*ctxt).dict);
        if (*ctxt).attsSpecial.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return;
        }
    }
    if !xmlHashLookup2((*ctxt).attsSpecial, fullname, fullattr).is_null() {
        return;
    }
    xmlHashAddEntry2(
        (*ctxt).attsSpecial,
        fullname,
        fullattr,
        type_0 as ptrdiff_t as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn xmlCleanSpecialAttrCallback(
    mut payload: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
    mut fullname: *const xmlChar,
    mut fullattr: *const xmlChar,
    mut unused: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = data as xmlParserCtxtPtr;
    if payload as ptrdiff_t == XML_ATTRIBUTE_CDATA as ::core::ffi::c_int as ptrdiff_t {
        xmlHashRemoveEntry2((*ctxt).attsSpecial, fullname, fullattr, None);
    }
}
unsafe extern "C" fn xmlCleanSpecialAttr(mut ctxt: xmlParserCtxtPtr) {
    if (*ctxt).attsSpecial.is_null() {
        return;
    }
    xmlHashScanFull(
        (*ctxt).attsSpecial,
        Some(
            xmlCleanSpecialAttrCallback
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        ctxt as *mut ::core::ffi::c_void,
    );
    if xmlHashSize((*ctxt).attsSpecial) == 0 as ::core::ffi::c_int {
        xmlHashFree((*ctxt).attsSpecial, None);
        (*ctxt).attsSpecial = ::core::ptr::null_mut::<xmlHashTable>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlCheckLanguageID(mut lang: *const xmlChar) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut cur: *const xmlChar = lang;
    let mut nxt: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if cur.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'i' as i32
        && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        || *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'I' as i32
            && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        || *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'x' as i32
            && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        || *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'X' as i32
            && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
    {
        cur = cur.offset(2 as ::core::ffi::c_int as isize);
        while *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'A' as i32
            && *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'Z' as i32
            || *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'a' as i32
                && *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'z' as i32
        {
            cur = cur.offset(1);
        }
        return (*cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    nxt = cur;
    while *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'A' as i32
        && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'Z' as i32
        || *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'a' as i32
            && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'z' as i32
    {
        nxt = nxt.offset(1);
    }
    if nxt.offset_from(cur) as ::core::ffi::c_long >= 4 as ::core::ffi::c_long {
        if nxt.offset_from(cur) as ::core::ffi::c_long > 8 as ::core::ffi::c_long
            || *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
    if (nxt.offset_from(cur) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
        return 0 as ::core::ffi::c_int;
    }
    if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    nxt = nxt.offset(1);
    cur = nxt;
    if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
        && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
    {
        current_block = 2864011465692865651;
    } else {
        while *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'A' as i32
            && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'Z' as i32
            || *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'a' as i32
                && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'z' as i32
        {
            nxt = nxt.offset(1);
        }
        if nxt.offset_from(cur) as ::core::ffi::c_long == 4 as ::core::ffi::c_long {
            current_block = 13652844147963915793;
        } else if nxt.offset_from(cur) as ::core::ffi::c_long == 2 as ::core::ffi::c_long {
            current_block = 15194834278218444012;
        } else if nxt.offset_from(cur) as ::core::ffi::c_long >= 5 as ::core::ffi::c_long
            && nxt.offset_from(cur) as ::core::ffi::c_long <= 8 as ::core::ffi::c_long
        {
            current_block = 3733359323116310951;
        } else {
            if nxt.offset_from(cur) as ::core::ffi::c_long != 3 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32 {
                return 0 as ::core::ffi::c_int;
            }
            nxt = nxt.offset(1);
            cur = nxt;
            if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
                && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
            {
                current_block = 2864011465692865651;
            } else {
                while *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >= 'A' as i32
                    && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= 'Z' as i32
                    || *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        >= 'a' as i32
                        && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            <= 'z' as i32
                {
                    nxt = nxt.offset(1);
                }
                if nxt.offset_from(cur) as ::core::ffi::c_long == 2 as ::core::ffi::c_long {
                    current_block = 15194834278218444012;
                } else if nxt.offset_from(cur) as ::core::ffi::c_long >= 5 as ::core::ffi::c_long
                    && nxt.offset_from(cur) as ::core::ffi::c_long <= 8 as ::core::ffi::c_long
                {
                    current_block = 3733359323116310951;
                } else {
                    if nxt.offset_from(cur) as ::core::ffi::c_long != 4 as ::core::ffi::c_long {
                        return 0 as ::core::ffi::c_int;
                    }
                    current_block = 13652844147963915793;
                }
            }
        }
        match current_block {
            15194834278218444012 => {}
            3733359323116310951 => {}
            2864011465692865651 => {}
            _ => {
                if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    return 1 as ::core::ffi::c_int;
                }
                if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32
                {
                    return 0 as ::core::ffi::c_int;
                }
                nxt = nxt.offset(1);
                cur = nxt;
                if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
                    && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= '9' as i32
                {
                    current_block = 2864011465692865651;
                } else {
                    while *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        >= 'A' as i32
                        && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            <= 'Z' as i32
                        || *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            >= 'a' as i32
                            && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                <= 'z' as i32
                    {
                        nxt = nxt.offset(1);
                    }
                    if nxt.offset_from(cur) as ::core::ffi::c_long >= 5 as ::core::ffi::c_long
                        && nxt.offset_from(cur) as ::core::ffi::c_long <= 8 as ::core::ffi::c_long
                    {
                        current_block = 3733359323116310951;
                    } else {
                        if nxt.offset_from(cur) as ::core::ffi::c_long != 2 as ::core::ffi::c_long {
                            return 0 as ::core::ffi::c_int;
                        }
                        current_block = 15194834278218444012;
                    }
                }
            }
        }
    }
    match current_block {
        2864011465692865651 => {
            if *nxt.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
                && *nxt.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
                && (*nxt.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >= '0' as i32
                    && *nxt.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= '9' as i32)
            {
                nxt = nxt.offset(3 as ::core::ffi::c_int as isize);
            } else {
                return 0 as ::core::ffi::c_int;
            }
            current_block = 15194834278218444012;
        }
        _ => {}
    }
    match current_block {
        15194834278218444012 => {
            if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32 {
                return 0 as ::core::ffi::c_int;
            }
            nxt = nxt.offset(1);
            cur = nxt;
            while *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'A' as i32
                && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'Z' as i32
                || *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'a' as i32
                    && *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= 'z' as i32
            {
                nxt = nxt.offset(1);
            }
            if (nxt.offset_from(cur) as ::core::ffi::c_long) < 5 as ::core::ffi::c_long
                || nxt.offset_from(cur) as ::core::ffi::c_long > 8 as ::core::ffi::c_long
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if *nxt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserNsCreate() -> *mut xmlParserNsData {
    let mut nsdb: *mut xmlParserNsData = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlParserNsData>() as size_t,
    ) as *mut xmlParserNsData;
    if nsdb.is_null() {
        return ::core::ptr::null_mut::<xmlParserNsData>();
    }
    memset(
        nsdb as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlParserNsData>() as size_t,
    );
    (*nsdb).defaultNsIndex = INT_MAX;
    return nsdb;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserNsFree(mut nsdb: *mut xmlParserNsData) {
    if nsdb.is_null() {
        return;
    }
    xmlFree.expect("non-null function pointer")((*nsdb).extra as *mut ::core::ffi::c_void);
    xmlFree.expect("non-null function pointer")((*nsdb).hash as *mut ::core::ffi::c_void);
    xmlFree.expect("non-null function pointer")(nsdb as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlParserNsReset(mut nsdb: *mut xmlParserNsData) {
    if nsdb.is_null() {
        return;
    }
    (*nsdb).hashElems = 0 as ::core::ffi::c_uint;
    (*nsdb).elementId = 0 as ::core::ffi::c_uint;
    (*nsdb).defaultNsIndex = INT_MAX;
    if !(*nsdb).hash.is_null() {
        memset(
            (*nsdb).hash as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*nsdb).hashSize as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlParserNsBucket>() as size_t),
        );
    }
}
unsafe extern "C" fn xmlParserNsStartElement(mut nsdb: *mut xmlParserNsData) -> ::core::ffi::c_int {
    if (*nsdb).elementId == UINT_MAX {
        return -(1 as ::core::ffi::c_int);
    }
    (*nsdb).elementId = (*nsdb).elementId.wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParserNsLookup(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *const xmlHashedString,
    mut bucketPtr: *mut *mut xmlParserNsBucket,
) -> ::core::ffi::c_int {
    let mut bucket: *mut xmlParserNsBucket = ::core::ptr::null_mut::<xmlParserNsBucket>();
    let mut index: ::core::ffi::c_uint = 0;
    let mut hashValue: ::core::ffi::c_uint = 0;
    if (*prefix).name.is_null() {
        return (*(*ctxt).nsdb).defaultNsIndex;
    }
    if (*(*ctxt).nsdb).hashSize == 0 as ::core::ffi::c_uint {
        return 2147483647 as ::core::ffi::c_int;
    }
    hashValue = (*prefix).hashValue;
    index = hashValue
        & (*(*ctxt).nsdb)
            .hashSize
            .wrapping_sub(1 as ::core::ffi::c_uint);
    bucket = (*(*ctxt).nsdb).hash.offset(index as isize) as *mut xmlParserNsBucket;
    while (*bucket).hashValue != 0 {
        if (*bucket).hashValue == hashValue && (*bucket).index != INT_MAX {
            if *(*ctxt)
                .nsTab
                .offset(((*bucket).index * 2 as ::core::ffi::c_int) as isize)
                == (*prefix).name
            {
                if !bucketPtr.is_null() {
                    *bucketPtr = bucket;
                }
                return (*bucket).index;
            }
        }
        index = index.wrapping_add(1);
        bucket = bucket.offset(1);
        if index == (*(*ctxt).nsdb).hashSize {
            index = 0 as ::core::ffi::c_uint;
            bucket = (*(*ctxt).nsdb).hash;
        }
    }
    if !bucketPtr.is_null() {
        *bucketPtr = bucket;
    }
    return 2147483647 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParserNsLookupUri(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *const xmlHashedString,
) -> *const xmlChar {
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut nsIndex: ::core::ffi::c_int = 0;
    if (*prefix).name == (*ctxt).str_xml {
        return (*ctxt).str_xml_ns;
    }
    nsIndex = xmlParserNsLookup(
        ctxt,
        prefix,
        ::core::ptr::null_mut::<*mut xmlParserNsBucket>(),
    );
    if nsIndex == INT_MAX {
        return ::core::ptr::null::<xmlChar>();
    }
    ret = *(*ctxt)
        .nsTab
        .offset((nsIndex * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
    if *ret.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        ret = ::core::ptr::null::<xmlChar>();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserNsLookupSax(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    let mut hprefix: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut nsIndex: ::core::ffi::c_int = 0;
    if prefix == (*ctxt).str_xml {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    hprefix.name = prefix;
    if !prefix.is_null() {
        hprefix.hashValue = xmlDictComputeHash((*ctxt).dict as *const xmlDict, prefix);
    } else {
        hprefix.hashValue = 0 as ::core::ffi::c_uint;
    }
    nsIndex = xmlParserNsLookup(
        ctxt,
        &raw mut hprefix,
        ::core::ptr::null_mut::<*mut xmlParserNsBucket>(),
    );
    if nsIndex == INT_MAX {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*(*(*ctxt).nsdb).extra.offset(nsIndex as isize)).saxData;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserNsUpdateSax(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *const xmlChar,
    mut saxData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut hprefix: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut nsIndex: ::core::ffi::c_int = 0;
    if prefix == (*ctxt).str_xml {
        return -(1 as ::core::ffi::c_int);
    }
    hprefix.name = prefix;
    if !prefix.is_null() {
        hprefix.hashValue = xmlDictComputeHash((*ctxt).dict as *const xmlDict, prefix);
    } else {
        hprefix.hashValue = 0 as ::core::ffi::c_uint;
    }
    nsIndex = xmlParserNsLookup(
        ctxt,
        &raw mut hprefix,
        ::core::ptr::null_mut::<*mut xmlParserNsBucket>(),
    );
    if nsIndex == INT_MAX {
        return -(1 as ::core::ffi::c_int);
    }
    let ref mut fresh120 = (*(*(*ctxt).nsdb).extra.offset(nsIndex as isize)).saxData;
    *fresh120 = saxData;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParserNsGrow(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut table: *mut *const xmlChar = ::core::ptr::null_mut::<*const xmlChar>();
    let mut extra: *mut xmlParserNsExtra = ::core::ptr::null_mut::<xmlParserNsExtra>();
    let mut newSize: ::core::ffi::c_int = 0;
    if !((*ctxt).nsMax > INT_MAX / 2 as ::core::ffi::c_int) {
        newSize = if (*ctxt).nsMax != 0 {
            (*ctxt).nsMax * 2 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        };
        table = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).nsTab as *mut ::core::ffi::c_void,
            ((2 as ::core::ffi::c_int * newSize) as size_t)
                .wrapping_mul(::core::mem::size_of::<*const xmlChar>() as size_t),
        ) as *mut *const xmlChar;
        if !table.is_null() {
            (*ctxt).nsTab = table;
            extra = xmlRealloc.expect("non-null function pointer")(
                (*(*ctxt).nsdb).extra as *mut ::core::ffi::c_void,
                (newSize as size_t)
                    .wrapping_mul(::core::mem::size_of::<xmlParserNsExtra>() as size_t),
            ) as *mut xmlParserNsExtra;
            if !extra.is_null() {
                (*(*ctxt).nsdb).extra = extra;
                (*ctxt).nsMax = newSize;
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn xmlParserNsPush(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *const xmlHashedString,
    mut uri: *const xmlHashedString,
    mut saxData: *mut ::core::ffi::c_void,
    mut defAttr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bucket: *mut xmlParserNsBucket = ::core::ptr::null_mut::<xmlParserNsBucket>();
    let mut extra: *mut xmlParserNsExtra = ::core::ptr::null_mut::<xmlParserNsExtra>();
    let mut ns: *mut *const xmlChar = ::core::ptr::null_mut::<*const xmlChar>();
    let mut hashValue: ::core::ffi::c_uint = 0;
    let mut nsIndex: ::core::ffi::c_uint = 0;
    let mut oldIndex: ::core::ffi::c_uint = 0;
    if !prefix.is_null() && (*prefix).name == (*ctxt).str_xml {
        return 0 as ::core::ffi::c_int;
    }
    if (*ctxt).nsNr >= (*ctxt).nsMax && xmlParserNsGrow(ctxt) < 0 as ::core::ffi::c_int {
        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        return -(1 as ::core::ffi::c_int);
    }
    if prefix.is_null() || (*prefix).name.is_null() {
        oldIndex = (*(*ctxt).nsdb).defaultNsIndex as ::core::ffi::c_uint;
        if oldIndex != INT_MAX as ::core::ffi::c_uint {
            extra = (*(*ctxt).nsdb).extra.offset(oldIndex as isize) as *mut xmlParserNsExtra;
            if (*extra).elementId == (*(*ctxt).nsdb).elementId {
                if defAttr == 0 as ::core::ffi::c_int {
                    xmlErrAttributeDup(
                        ctxt,
                        ::core::ptr::null::<xmlChar>(),
                        b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                    );
                }
                return 0 as ::core::ffi::c_int;
            }
            if (*ctxt).options & XML_PARSE_NSCLEAN as ::core::ffi::c_int != 0
                && (*uri).name
                    == *(*ctxt).nsTab.offset(
                        oldIndex
                            .wrapping_mul(2 as ::core::ffi::c_uint)
                            .wrapping_add(1 as ::core::ffi::c_uint)
                            as isize,
                    )
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        (*(*ctxt).nsdb).defaultNsIndex = (*ctxt).nsNr;
    } else {
        oldIndex = xmlParserNsLookup(ctxt, prefix, &raw mut bucket) as ::core::ffi::c_uint;
        if oldIndex != INT_MAX as ::core::ffi::c_uint {
            extra = (*(*ctxt).nsdb).extra.offset(oldIndex as isize) as *mut xmlParserNsExtra;
            if (*extra).elementId == (*(*ctxt).nsdb).elementId {
                if defAttr == 0 as ::core::ffi::c_int {
                    xmlErrAttributeDup(
                        ctxt,
                        b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                        (*prefix).name,
                    );
                }
                return 0 as ::core::ffi::c_int;
            }
            if (*ctxt).options & XML_PARSE_NSCLEAN as ::core::ffi::c_int != 0
                && (*uri).name
                    == *(*ctxt).nsTab.offset(
                        ((*bucket).index * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as isize,
                    )
            {
                return 0 as ::core::ffi::c_int;
            }
            (*bucket).index = (*ctxt).nsNr;
        } else {
            hashValue = (*prefix).hashValue;
            if (*(*ctxt).nsdb)
                .hashElems
                .wrapping_add(1 as ::core::ffi::c_uint)
                > (*(*ctxt).nsdb)
                    .hashSize
                    .wrapping_div(2 as ::core::ffi::c_uint)
            {
                let mut newHash: *mut xmlParserNsBucket =
                    ::core::ptr::null_mut::<xmlParserNsBucket>();
                let mut newSize: ::core::ffi::c_uint = 0;
                let mut i: ::core::ffi::c_uint = 0;
                let mut index: ::core::ffi::c_uint = 0;
                if (*(*ctxt).nsdb).hashSize > UINT_MAX.wrapping_div(2 as ::core::ffi::c_uint) {
                    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                    return -(1 as ::core::ffi::c_int);
                }
                newSize = if (*(*ctxt).nsdb).hashSize != 0 {
                    (*(*ctxt).nsdb)
                        .hashSize
                        .wrapping_mul(2 as ::core::ffi::c_uint)
                } else {
                    16 as ::core::ffi::c_uint
                };
                newHash = xmlMalloc.expect("non-null function pointer")(
                    (newSize as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlParserNsBucket>() as size_t),
                ) as *mut xmlParserNsBucket;
                if newHash.is_null() {
                    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                    return -(1 as ::core::ffi::c_int);
                }
                memset(
                    newHash as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (newSize as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlParserNsBucket>() as size_t),
                );
                i = 0 as ::core::ffi::c_uint;
                while i < (*(*ctxt).nsdb).hashSize {
                    let mut hv: ::core::ffi::c_uint =
                        (*(*(*ctxt).nsdb).hash.offset(i as isize)).hashValue;
                    let mut newIndex: ::core::ffi::c_uint = 0;
                    if !(hv == 0 as ::core::ffi::c_uint) {
                        newIndex = hv & newSize.wrapping_sub(1 as ::core::ffi::c_uint);
                        while (*newHash.offset(newIndex as isize)).hashValue
                            != 0 as ::core::ffi::c_uint
                        {
                            newIndex = newIndex.wrapping_add(1);
                            if newIndex == newSize {
                                newIndex = 0 as ::core::ffi::c_uint;
                            }
                        }
                        *newHash.offset(newIndex as isize) =
                            *(*(*ctxt).nsdb).hash.offset(i as isize);
                    }
                    i = i.wrapping_add(1);
                }
                xmlFree.expect("non-null function pointer")(
                    (*(*ctxt).nsdb).hash as *mut ::core::ffi::c_void,
                );
                (*(*ctxt).nsdb).hash = newHash;
                (*(*ctxt).nsdb).hashSize = newSize;
                index = hashValue & newSize.wrapping_sub(1 as ::core::ffi::c_uint);
                while (*newHash.offset(index as isize)).hashValue != 0 as ::core::ffi::c_uint {
                    index = index.wrapping_add(1);
                    if index == newSize {
                        index = 0 as ::core::ffi::c_uint;
                    }
                }
                bucket = newHash.offset(index as isize) as *mut xmlParserNsBucket;
            }
            (*bucket).hashValue = hashValue;
            (*bucket).index = (*ctxt).nsNr;
            (*(*ctxt).nsdb).hashElems = (*(*ctxt).nsdb).hashElems.wrapping_add(1);
            oldIndex = INT_MAX as ::core::ffi::c_uint;
        }
    }
    nsIndex = (*ctxt).nsNr as ::core::ffi::c_uint;
    ns = (*ctxt)
        .nsTab
        .offset(nsIndex.wrapping_mul(2 as ::core::ffi::c_uint) as isize)
        as *mut *const xmlChar;
    let ref mut fresh17 = *ns.offset(0 as ::core::ffi::c_int as isize);
    *fresh17 = if !prefix.is_null() {
        (*prefix).name
    } else {
        ::core::ptr::null::<xmlChar>()
    };
    let ref mut fresh18 = *ns.offset(1 as ::core::ffi::c_int as isize);
    *fresh18 = (*uri).name;
    extra = (*(*ctxt).nsdb).extra.offset(nsIndex as isize) as *mut xmlParserNsExtra;
    (*extra).saxData = saxData;
    (*extra).prefixHashValue = if !prefix.is_null() {
        (*prefix).hashValue
    } else {
        0 as ::core::ffi::c_uint
    };
    (*extra).uriHashValue = (*uri).hashValue;
    (*extra).elementId = (*(*ctxt).nsdb).elementId;
    (*extra).oldIndex = oldIndex as ::core::ffi::c_int;
    (*ctxt).nsNr += 1;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParserNsPop(
    mut ctxt: xmlParserCtxtPtr,
    mut nr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = (*ctxt).nsNr - 1 as ::core::ffi::c_int;
    while i >= (*ctxt).nsNr - nr {
        let mut prefix: *const xmlChar =
            *(*ctxt).nsTab.offset((i * 2 as ::core::ffi::c_int) as isize);
        let mut extra: *mut xmlParserNsExtra =
            (*(*ctxt).nsdb).extra.offset(i as isize) as *mut xmlParserNsExtra;
        if prefix.is_null() {
            (*(*ctxt).nsdb).defaultNsIndex = (*extra).oldIndex;
        } else {
            let mut hprefix: xmlHashedString = xmlHashedString {
                hashValue: 0,
                name: ::core::ptr::null::<xmlChar>(),
            };
            let mut bucket: *mut xmlParserNsBucket = ::core::ptr::null_mut::<xmlParserNsBucket>();
            hprefix.name = prefix;
            hprefix.hashValue = (*extra).prefixHashValue;
            xmlParserNsLookup(ctxt, &raw mut hprefix, &raw mut bucket);
            (*bucket).index = (*extra).oldIndex;
        }
        i -= 1;
    }
    (*ctxt).nsNr -= nr;
    return nr;
}
unsafe extern "C" fn xmlCtxtGrowAttrs(
    mut ctxt: xmlParserCtxtPtr,
    mut nr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut atts: *mut *const xmlChar = ::core::ptr::null_mut::<*const xmlChar>();
    let mut attallocs: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
    let mut maxatts: ::core::ffi::c_int = 0;
    if nr + 5 as ::core::ffi::c_int > (*ctxt).maxatts {
        maxatts = if (*ctxt).maxatts == 0 as ::core::ffi::c_int {
            55 as ::core::ffi::c_int
        } else {
            (nr + 5 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int
        };
        atts = xmlMalloc.expect("non-null function pointer")(
            (maxatts as size_t).wrapping_mul(::core::mem::size_of::<*const xmlChar>() as size_t),
        ) as *mut *const xmlChar;
        if atts.is_null() {
            current_block = 213213189033268961;
        } else {
            attallocs = xmlRealloc.expect("non-null function pointer")(
                (*ctxt).attallocs as *mut ::core::ffi::c_void,
                ((maxatts / 5 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uint>() as size_t),
            ) as *mut ::core::ffi::c_uint;
            if attallocs.is_null() {
                xmlFree.expect("non-null function pointer")(atts as *mut ::core::ffi::c_void);
                current_block = 213213189033268961;
            } else {
                if (*ctxt).maxatts > 0 as ::core::ffi::c_int {
                    memcpy(
                        atts as *mut ::core::ffi::c_void,
                        (*ctxt).atts as *const ::core::ffi::c_void,
                        ((*ctxt).maxatts as size_t)
                            .wrapping_mul(::core::mem::size_of::<*const xmlChar>() as size_t),
                    );
                }
                xmlFree.expect("non-null function pointer")(
                    (*ctxt).atts as *mut ::core::ffi::c_void,
                );
                (*ctxt).atts = atts;
                (*ctxt).attallocs = attallocs;
                (*ctxt).maxatts = maxatts;
                current_block = 17965632435239708295;
            }
        }
        match current_block {
            17965632435239708295 => {}
            _ => {
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                return -(1 as ::core::ffi::c_int);
            }
        }
    }
    return (*ctxt).maxatts;
}
#[no_mangle]
pub unsafe extern "C" fn inputPush(
    mut ctxt: xmlParserCtxtPtr,
    mut value: xmlParserInputPtr,
) -> ::core::ffi::c_int {
    if ctxt.is_null() || value.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).inputNr >= (*ctxt).inputMax {
        let mut newSize: size_t = ((*ctxt).inputMax * 2 as ::core::ffi::c_int) as size_t;
        let mut tmp: *mut xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInputPtr>();
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).inputTab as *mut ::core::ffi::c_void,
            newSize.wrapping_mul(::core::mem::size_of::<xmlParserInputPtr>() as size_t),
        ) as *mut xmlParserInputPtr;
        if tmp.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return -(1 as ::core::ffi::c_int);
        }
        (*ctxt).inputTab = tmp;
        (*ctxt).inputMax = newSize as ::core::ffi::c_int;
    }
    let ref mut fresh8 = *(*ctxt).inputTab.offset((*ctxt).inputNr as isize);
    *fresh8 = value;
    (*ctxt).input = value;
    let fresh9 = (*ctxt).inputNr;
    (*ctxt).inputNr = (*ctxt).inputNr + 1;
    return fresh9;
}
#[no_mangle]
pub unsafe extern "C" fn inputPop(mut ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr {
    let mut ret: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlParserInput>();
    }
    if (*ctxt).inputNr <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlParserInput>();
    }
    (*ctxt).inputNr -= 1;
    if (*ctxt).inputNr > 0 as ::core::ffi::c_int {
        (*ctxt).input = *(*ctxt)
            .inputTab
            .offset(((*ctxt).inputNr - 1 as ::core::ffi::c_int) as isize);
    } else {
        (*ctxt).input = ::core::ptr::null_mut::<xmlParserInput>();
    }
    ret = *(*ctxt).inputTab.offset((*ctxt).inputNr as isize);
    let ref mut fresh2 = *(*ctxt).inputTab.offset((*ctxt).inputNr as isize);
    *fresh2 = ::core::ptr::null_mut::<xmlParserInput>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn nodePush(
    mut ctxt: xmlParserCtxtPtr,
    mut value: xmlNodePtr,
) -> ::core::ffi::c_int {
    if ctxt.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*ctxt).nodeNr >= (*ctxt).nodeMax {
        let mut tmp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).nodeTab as *mut ::core::ffi::c_void,
            (((*ctxt).nodeMax * 2 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if tmp.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return -(1 as ::core::ffi::c_int);
        }
        (*ctxt).nodeTab = tmp;
        (*ctxt).nodeMax *= 2 as ::core::ffi::c_int;
    }
    if (*ctxt).nodeNr as ::core::ffi::c_uint > xmlParserMaxDepth
        && (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        xmlFatalErrMsgInt(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"Excessive depth in document: %d use XML_PARSE_HUGE option\n\0" as *const u8
                as *const ::core::ffi::c_char,
            xmlParserMaxDepth as ::core::ffi::c_int,
        );
        xmlHaltParser(ctxt);
        return -(1 as ::core::ffi::c_int);
    }
    let ref mut fresh15 = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    *fresh15 = value;
    (*ctxt).node = value;
    let fresh16 = (*ctxt).nodeNr;
    (*ctxt).nodeNr = (*ctxt).nodeNr + 1;
    return fresh16;
}
#[no_mangle]
pub unsafe extern "C" fn nodePop(mut ctxt: xmlParserCtxtPtr) -> xmlNodePtr {
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*ctxt).nodeNr <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    (*ctxt).nodeNr -= 1;
    if (*ctxt).nodeNr > 0 as ::core::ffi::c_int {
        (*ctxt).node = *(*ctxt)
            .nodeTab
            .offset(((*ctxt).nodeNr - 1 as ::core::ffi::c_int) as isize);
    } else {
        (*ctxt).node = ::core::ptr::null_mut::<xmlNode>();
    }
    ret = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    let ref mut fresh21 = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    *fresh21 = ::core::ptr::null_mut::<xmlNode>();
    return ret;
}
unsafe extern "C" fn nameNsPush(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
    mut line: ::core::ffi::c_int,
    mut nsNr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut tag: *mut xmlStartTag = ::core::ptr::null_mut::<xmlStartTag>();
    if (*ctxt).nameNr >= (*ctxt).nameMax {
        let mut tmp: *mut *const xmlChar = ::core::ptr::null_mut::<*const xmlChar>();
        let mut tmp2: *mut xmlStartTag = ::core::ptr::null_mut::<xmlStartTag>();
        (*ctxt).nameMax *= 2 as ::core::ffi::c_int;
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).nameTab as *mut *mut xmlChar as *mut ::core::ffi::c_void,
            ((*ctxt).nameMax as size_t)
                .wrapping_mul(::core::mem::size_of::<*const xmlChar>() as size_t),
        ) as *mut *const xmlChar;
        if tmp.is_null() {
            (*ctxt).nameMax /= 2 as ::core::ffi::c_int;
            current_block = 15192010698983220831;
        } else {
            (*ctxt).nameTab = tmp;
            tmp2 = xmlRealloc.expect("non-null function pointer")(
                (*ctxt).pushTab as *mut *mut ::core::ffi::c_void as *mut ::core::ffi::c_void,
                ((*ctxt).nameMax as size_t)
                    .wrapping_mul(::core::mem::size_of::<xmlStartTag>() as size_t),
            ) as *mut xmlStartTag;
            if tmp2.is_null() {
                (*ctxt).nameMax /= 2 as ::core::ffi::c_int;
                current_block = 15192010698983220831;
            } else {
                (*ctxt).pushTab = tmp2;
                current_block = 1856101646708284338;
            }
        }
    } else if (*ctxt).pushTab.is_null() {
        (*ctxt).pushTab = xmlMalloc.expect("non-null function pointer")(
            ((*ctxt).nameMax as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlStartTag>() as size_t),
        ) as *mut xmlStartTag;
        if (*ctxt).pushTab.is_null() {
            current_block = 15192010698983220831;
        } else {
            current_block = 1856101646708284338;
        }
    } else {
        current_block = 1856101646708284338;
    }
    match current_block {
        15192010698983220831 => {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return -(1 as ::core::ffi::c_int);
        }
        _ => {
            let ref mut fresh22 = *(*ctxt).nameTab.offset((*ctxt).nameNr as isize);
            *fresh22 = value;
            (*ctxt).name = value;
            tag = (*ctxt).pushTab.offset((*ctxt).nameNr as isize) as *mut xmlStartTag;
            (*tag).prefix = prefix;
            (*tag).URI = URI;
            (*tag).line = line;
            (*tag).nsNr = nsNr;
            let fresh23 = (*ctxt).nameNr;
            (*ctxt).nameNr = (*ctxt).nameNr + 1;
            return fresh23;
        }
    };
}
unsafe extern "C" fn nameNsPop(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if (*ctxt).nameNr <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    (*ctxt).nameNr -= 1;
    if (*ctxt).nameNr > 0 as ::core::ffi::c_int {
        (*ctxt).name = *(*ctxt)
            .nameTab
            .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize);
    } else {
        (*ctxt).name = ::core::ptr::null::<xmlChar>();
    }
    ret = *(*ctxt).nameTab.offset((*ctxt).nameNr as isize);
    let ref mut fresh108 = *(*ctxt).nameTab.offset((*ctxt).nameNr as isize);
    *fresh108 = ::core::ptr::null::<xmlChar>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn namePush(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *const xmlChar,
) -> ::core::ffi::c_int {
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).nameNr >= (*ctxt).nameMax {
        let mut tmp: *mut *const xmlChar = ::core::ptr::null_mut::<*const xmlChar>();
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).nameTab as *mut *mut xmlChar as *mut ::core::ffi::c_void,
            (((*ctxt).nameMax * 2 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<*const xmlChar>() as size_t),
        ) as *mut *const xmlChar;
        if tmp.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return -(1 as ::core::ffi::c_int);
        } else {
            (*ctxt).nameTab = tmp;
            (*ctxt).nameMax *= 2 as ::core::ffi::c_int;
        }
    }
    let ref mut fresh118 = *(*ctxt).nameTab.offset((*ctxt).nameNr as isize);
    *fresh118 = value;
    (*ctxt).name = value;
    let fresh119 = (*ctxt).nameNr;
    (*ctxt).nameNr = (*ctxt).nameNr + 1;
    return fresh119;
}
#[no_mangle]
pub unsafe extern "C" fn namePop(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if ctxt.is_null() || (*ctxt).nameNr <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    (*ctxt).nameNr -= 1;
    if (*ctxt).nameNr > 0 as ::core::ffi::c_int {
        (*ctxt).name = *(*ctxt)
            .nameTab
            .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize);
    } else {
        (*ctxt).name = ::core::ptr::null::<xmlChar>();
    }
    ret = *(*ctxt).nameTab.offset((*ctxt).nameNr as isize);
    let ref mut fresh10 = *(*ctxt).nameTab.offset((*ctxt).nameNr as isize);
    *fresh10 = ::core::ptr::null::<xmlChar>();
    return ret;
}
unsafe extern "C" fn spacePush(
    mut ctxt: xmlParserCtxtPtr,
    mut val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*ctxt).spaceNr >= (*ctxt).spaceMax {
        let mut tmp: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        (*ctxt).spaceMax *= 2 as ::core::ffi::c_int;
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).spaceTab as *mut ::core::ffi::c_void,
            ((*ctxt).spaceMax as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        ) as *mut ::core::ffi::c_int;
        if tmp.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            (*ctxt).spaceMax /= 2 as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        (*ctxt).spaceTab = tmp;
    }
    *(*ctxt).spaceTab.offset((*ctxt).spaceNr as isize) = val;
    (*ctxt).space = (*ctxt).spaceTab.offset((*ctxt).spaceNr as isize) as *mut ::core::ffi::c_int;
    let fresh102 = (*ctxt).spaceNr;
    (*ctxt).spaceNr = (*ctxt).spaceNr + 1;
    return fresh102;
}
unsafe extern "C" fn spacePop(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if (*ctxt).spaceNr <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    (*ctxt).spaceNr -= 1;
    if (*ctxt).spaceNr > 0 as ::core::ffi::c_int {
        (*ctxt).space = (*ctxt)
            .spaceTab
            .offset(((*ctxt).spaceNr - 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_int;
    } else {
        (*ctxt).space =
            (*ctxt).spaceTab.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    }
    ret = *(*ctxt).spaceTab.offset((*ctxt).spaceNr as isize);
    *(*ctxt).spaceTab.offset((*ctxt).spaceNr as isize) = -(1 as ::core::ffi::c_int);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSkipBlankChars(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*ctxt).inputNr == 1 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_DTD as ::core::ffi::c_int
        || (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_START as ::core::ffi::c_int
    {
        let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
        cur = (*(*ctxt).input).cur;
        while *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
        {
            if *cur as ::core::ffi::c_int == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
            } else {
                (*(*ctxt).input).col += 1;
            }
            cur = cur.offset(1);
            if res < INT_MAX {
                res += 1;
            }
            if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*(*ctxt).input).cur = cur;
                xmlParserGrow(ctxt);
                cur = (*(*ctxt).input).cur;
            }
        }
        (*(*ctxt).input).cur = cur;
    } else {
        let mut expandPE: ::core::ffi::c_int = ((*ctxt).external != 0 as ::core::ffi::c_int
            || (*ctxt).inputNr != 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        while (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int {
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
                    && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
            {
                xmlNextChar(ctxt);
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '%' as i32 {
                if expandPE == 0 as ::core::ffi::c_int
                    || (*(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0x20 as ::core::ffi::c_int
                        || 0x9 as ::core::ffi::c_int
                            <= *(*(*ctxt).input)
                                .cur
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                            && *(*(*ctxt).input)
                                .cur
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                <= 0xa as ::core::ffi::c_int
                        || *(*(*ctxt).input)
                            .cur
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0xd as ::core::ffi::c_int)
                    || *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    break;
                }
                xmlParsePEReference(ctxt);
            } else {
                if !(*(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                    break;
                }
                let mut consumed: ::core::ffi::c_ulong = 0;
                let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
                if (*ctxt).inputNr <= 1 as ::core::ffi::c_int {
                    break;
                }
                consumed = (*(*ctxt).input).consumed;
                xmlSaturatedAddSizeT(
                    &raw mut consumed,
                    (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                        as ::core::ffi::c_ulong,
                );
                ent = (*(*ctxt).input).entity;
                if (*ent).etype as ::core::ffi::c_uint
                    == XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*ent).flags & XML_ENT_PARSED == 0 as ::core::ffi::c_int
                {
                    (*ent).flags |= XML_ENT_PARSED;
                    xmlSaturatedAdd(&raw mut (*ctxt).sizeentities, consumed);
                }
                xmlParserEntityCheck(ctxt, consumed);
                xmlPopInput(ctxt);
            }
            if res < INT_MAX {
                res += 1;
            }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPopInput(mut ctxt: xmlParserCtxtPtr) -> xmlChar {
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if ctxt.is_null() || (*ctxt).inputNr <= 1 as ::core::ffi::c_int {
        return 0 as xmlChar;
    }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Popping input %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*ctxt).inputNr,
        );
    }
    if (*ctxt).inputNr > 1 as ::core::ffi::c_int
        && (*ctxt).inSubset == 0 as ::core::ffi::c_int
        && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"Unfinished entity outside the DTD\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    input = inputPop(ctxt);
    if !(*input).entity.is_null() {
        (*(*input).entity).flags &= !XML_ENT_EXPANDING;
    }
    xmlFreeInputStream(input);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    return *(*(*ctxt).input).cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPushInput(
    mut ctxt: xmlParserCtxtPtr,
    mut input: xmlParserInputPtr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if input.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if *__xmlParserDebugEntities() != 0 {
        if !(*ctxt).input.is_null() && !(*(*ctxt).input).filename.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"%s(%d): \0" as *const u8 as *const ::core::ffi::c_char,
                (*(*ctxt).input).filename,
                (*(*ctxt).input).line,
            );
        }
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Pushing input %d : %.30s\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*ctxt).inputNr + 1 as ::core::ffi::c_int,
            (*input).cur,
        );
    }
    if (*ctxt).inputNr > 40 as ::core::ffi::c_int
        && (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*ctxt).inputNr > 100 as ::core::ffi::c_int
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_ENTITY_LOOP,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        while (*ctxt).inputNr > 1 as ::core::ffi::c_int {
            xmlFreeInputStream(inputPop(ctxt));
        }
        return -(1 as ::core::ffi::c_int);
    }
    ret = inputPush(ctxt, input);
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseCharRef(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut val: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '&' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '#' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'x' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(3 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        while *(*(*ctxt).input).cur as ::core::ffi::c_int != ';' as i32 {
            let fresh19 = count;
            count = count + 1;
            if fresh19 > 20 as ::core::ffi::c_int {
                count = 0 as ::core::ffi::c_int;
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int >= '0' as i32
                && *(*(*ctxt).input).cur as ::core::ffi::c_int <= '9' as i32
            {
                val = val * 16 as ::core::ffi::c_int
                    + (*(*(*ctxt).input).cur as ::core::ffi::c_int - '0' as i32);
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int >= 'a' as i32
                && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 'f' as i32
                && count < 20 as ::core::ffi::c_int
            {
                val = val * 16 as ::core::ffi::c_int
                    + (*(*(*ctxt).input).cur as ::core::ffi::c_int - 'a' as i32)
                    + 10 as ::core::ffi::c_int;
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int >= 'A' as i32
                && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 'F' as i32
                && count < 20 as ::core::ffi::c_int
            {
                val = val * 16 as ::core::ffi::c_int
                    + (*(*(*ctxt).input).cur as ::core::ffi::c_int - 'A' as i32)
                    + 10 as ::core::ffi::c_int;
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_INVALID_HEX_CHARREF,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                val = 0 as ::core::ffi::c_int;
                break;
            }
            if val > 0x110000 as ::core::ffi::c_int {
                val = 0x110000 as ::core::ffi::c_int;
            }
            xmlNextChar(ctxt);
            count += 1;
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == ';' as i32 {
            (*(*ctxt).input).col += 1;
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
        }
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '&' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '#' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(2 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        while *(*(*ctxt).input).cur as ::core::ffi::c_int != ';' as i32 {
            let fresh20 = count;
            count = count + 1;
            if fresh20 > 20 as ::core::ffi::c_int {
                count = 0 as ::core::ffi::c_int;
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int >= '0' as i32
                && *(*(*ctxt).input).cur as ::core::ffi::c_int <= '9' as i32
            {
                val = val * 10 as ::core::ffi::c_int
                    + (*(*(*ctxt).input).cur as ::core::ffi::c_int - '0' as i32);
                if val > 0x110000 as ::core::ffi::c_int {
                    val = 0x110000 as ::core::ffi::c_int;
                }
                xmlNextChar(ctxt);
                count += 1;
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_INVALID_DEC_CHARREF,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                val = 0 as ::core::ffi::c_int;
                break;
            }
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == ';' as i32 {
            (*(*ctxt).input).col += 1;
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
        }
    } else {
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '&' as i32 {
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 1 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
        }
        xmlFatalErr(
            ctxt,
            XML_ERR_INVALID_CHARREF,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if val >= 0x110000 as ::core::ffi::c_int {
        xmlFatalErrMsgInt(
            ctxt,
            XML_ERR_INVALID_CHAR,
            b"xmlParseCharRef: character reference out of bounds\n\0" as *const u8
                as *const ::core::ffi::c_char,
            val,
        );
    } else if if val < 0x100 as ::core::ffi::c_int {
        (0x9 as ::core::ffi::c_int <= val && val <= 0xa as ::core::ffi::c_int
            || val == 0xd as ::core::ffi::c_int
            || 0x20 as ::core::ffi::c_int <= val) as ::core::ffi::c_int
    } else {
        (0x100 as ::core::ffi::c_int <= val && val <= 0xd7ff as ::core::ffi::c_int
            || 0xe000 as ::core::ffi::c_int <= val && val <= 0xfffd as ::core::ffi::c_int
            || 0x10000 as ::core::ffi::c_int <= val && val <= 0x10ffff as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } != 0
    {
        return val;
    } else {
        xmlFatalErrMsgInt(
            ctxt,
            XML_ERR_INVALID_CHAR,
            b"xmlParseCharRef: invalid xmlChar value %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            val,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParseStringCharRef(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> ::core::ffi::c_int {
    let mut ptr: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut cur: xmlChar = 0;
    let mut val: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if str.is_null() || (*str).is_null() {
        return 0 as ::core::ffi::c_int;
    }
    ptr = *str;
    cur = *ptr;
    if cur as ::core::ffi::c_int == '&' as i32
        && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
        && *ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'x' as i32
    {
        ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
        cur = *ptr;
        while cur as ::core::ffi::c_int != ';' as i32 {
            if cur as ::core::ffi::c_int >= '0' as i32 && cur as ::core::ffi::c_int <= '9' as i32 {
                val = val * 16 as ::core::ffi::c_int + (cur as ::core::ffi::c_int - '0' as i32);
            } else if cur as ::core::ffi::c_int >= 'a' as i32
                && cur as ::core::ffi::c_int <= 'f' as i32
            {
                val = val * 16 as ::core::ffi::c_int
                    + (cur as ::core::ffi::c_int - 'a' as i32)
                    + 10 as ::core::ffi::c_int;
            } else if cur as ::core::ffi::c_int >= 'A' as i32
                && cur as ::core::ffi::c_int <= 'F' as i32
            {
                val = val * 16 as ::core::ffi::c_int
                    + (cur as ::core::ffi::c_int - 'A' as i32)
                    + 10 as ::core::ffi::c_int;
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_INVALID_HEX_CHARREF,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                val = 0 as ::core::ffi::c_int;
                break;
            }
            if val > 0x110000 as ::core::ffi::c_int {
                val = 0x110000 as ::core::ffi::c_int;
            }
            ptr = ptr.offset(1);
            cur = *ptr;
        }
        if cur as ::core::ffi::c_int == ';' as i32 {
            ptr = ptr.offset(1);
        }
    } else if cur as ::core::ffi::c_int == '&' as i32
        && *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
    {
        ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
        cur = *ptr;
        while cur as ::core::ffi::c_int != ';' as i32 {
            if cur as ::core::ffi::c_int >= '0' as i32 && cur as ::core::ffi::c_int <= '9' as i32 {
                val = val * 10 as ::core::ffi::c_int + (cur as ::core::ffi::c_int - '0' as i32);
                if val > 0x110000 as ::core::ffi::c_int {
                    val = 0x110000 as ::core::ffi::c_int;
                }
                ptr = ptr.offset(1);
                cur = *ptr;
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_INVALID_DEC_CHARREF,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                val = 0 as ::core::ffi::c_int;
                break;
            }
        }
        if cur as ::core::ffi::c_int == ';' as i32 {
            ptr = ptr.offset(1);
        }
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_INVALID_CHARREF,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return 0 as ::core::ffi::c_int;
    }
    *str = ptr;
    if val >= 0x110000 as ::core::ffi::c_int {
        xmlFatalErrMsgInt(
            ctxt,
            XML_ERR_INVALID_CHAR,
            b"xmlParseStringCharRef: character reference out of bounds\n\0" as *const u8
                as *const ::core::ffi::c_char,
            val,
        );
    } else if if val < 0x100 as ::core::ffi::c_int {
        (0x9 as ::core::ffi::c_int <= val && val <= 0xa as ::core::ffi::c_int
            || val == 0xd as ::core::ffi::c_int
            || 0x20 as ::core::ffi::c_int <= val) as ::core::ffi::c_int
    } else {
        (0x100 as ::core::ffi::c_int <= val && val <= 0xd7ff as ::core::ffi::c_int
            || 0xe000 as ::core::ffi::c_int <= val && val <= 0xfffd as ::core::ffi::c_int
            || 0x10000 as ::core::ffi::c_int <= val && val <= 0x10ffff as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } != 0
    {
        return val;
    } else {
        xmlFatalErrMsgInt(
            ctxt,
            XML_ERR_INVALID_CHAR,
            b"xmlParseStringCharRef: invalid xmlChar value %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            val,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserHandlePEReference(mut ctxt: xmlParserCtxtPtr) {
    match (*ctxt).instate as ::core::ffi::c_int {
        8 => return,
        5 => return,
        6 => return,
        9 => return,
        -1 => {
            xmlFatalErr(
                ctxt,
                XML_ERR_PEREF_AT_EOF,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
        4 | 0 | 17 | 1 => {
            xmlFatalErr(
                ctxt,
                XML_ERR_PEREF_IN_PROLOG,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
        10 | 7 | 12 | 2 | 13 | 16 => return,
        14 => {
            xmlFatalErr(
                ctxt,
                XML_ERR_PEREF_IN_EPILOG,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
        11 => return,
        3 => {
            if (*ctxt).external == 0 as ::core::ffi::c_int
                && (*ctxt).inputNr == 1 as ::core::ffi::c_int
            {
                return;
            }
            if *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int
                    <= *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                    && *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= 0xa as ::core::ffi::c_int
                || *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 0xd as ::core::ffi::c_int
                || *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                return;
            }
        }
        15 => return,
        _ => {}
    }
    xmlParsePEReference(ctxt);
}
unsafe extern "C" fn xmlStringDecodeEntitiesInt(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut len: ::core::ffi::c_int,
    mut what: ::core::ffi::c_int,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
    mut check: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut buffer_size: size_t = 0 as size_t;
    let mut nbchars: size_t = 0 as size_t;
    let mut current: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut rep: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut last: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut c: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    if str.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    last = str.offset(len as isize);
    if (*ctxt).depth > 40 as ::core::ffi::c_int
        && (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*ctxt).depth > 100 as ::core::ffi::c_int
    {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_ENTITY_LOOP,
            b"Maximum entity nesting depth exceeded\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buffer_size = XML_PARSER_BIG_BUFFER_SIZE as size_t;
    buffer = xmlMallocAtomic.expect("non-null function pointer")(buffer_size) as *mut xmlChar;
    if buffer.is_null() {
        current_block = 15533639510234485578;
    } else {
        if str < last {
            c = xmlStringCurrentChar(ctxt, str, &raw mut l);
        } else {
            c = 0 as ::core::ffi::c_int;
        }
        's_63: loop {
            if !(c != 0 as ::core::ffi::c_int
                && c != end as ::core::ffi::c_int
                && c != end2 as ::core::ffi::c_int
                && c != end3 as ::core::ffi::c_int
                && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int)
            {
                current_block = 5916212523694105379;
                break;
            }
            if c == 0 as ::core::ffi::c_int {
                current_block = 5916212523694105379;
                break;
            }
            if c == '&' as i32
                && *str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
            {
                let mut val: ::core::ffi::c_int = xmlParseStringCharRef(ctxt, &raw mut str);
                if val == 0 as ::core::ffi::c_int {
                    current_block = 1139312351219438743;
                    break;
                }
                if val < 0x80 as ::core::ffi::c_int {
                    let fresh54 = nbchars;
                    nbchars = nbchars.wrapping_add(1);
                    *buffer.offset(fresh54 as isize) = val as xmlChar;
                } else {
                    nbchars = (nbchars as ::core::ffi::c_ulong).wrapping_add(xmlCopyCharMultiByte(
                        buffer.offset(nbchars as isize) as *mut xmlChar,
                        val,
                    )
                        as ::core::ffi::c_ulong) as size_t as size_t;
                }
                if nbchars.wrapping_add(XML_PARSER_BUFFER_SIZE as size_t) > buffer_size {
                    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    let mut new_size: size_t = buffer_size
                        .wrapping_mul(2 as size_t)
                        .wrapping_add(100 as size_t);
                    if new_size < buffer_size {
                        current_block = 15533639510234485578;
                        break;
                    }
                    tmp = xmlRealloc.expect("non-null function pointer")(
                        buffer as *mut ::core::ffi::c_void,
                        new_size,
                    ) as *mut xmlChar;
                    if tmp.is_null() {
                        current_block = 15533639510234485578;
                        break;
                    }
                    buffer = tmp;
                    buffer_size = new_size;
                }
            } else if c == '&' as i32 && what & XML_SUBSTITUTE_REF != 0 {
                if *__xmlParserDebugEntities() != 0 {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"String decoding Entity Reference: %.30s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        str,
                    );
                }
                ent = xmlParseStringEntityRef(ctxt, &raw mut str);
                if !ent.is_null()
                    && (*ent).etype as ::core::ffi::c_uint
                        == XML_INTERNAL_PREDEFINED_ENTITY as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                {
                    if !(*ent).content.is_null() {
                        if (*(*ent).content.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int)
                            < 0x80 as ::core::ffi::c_int
                        {
                            let fresh55 = nbchars;
                            nbchars = nbchars.wrapping_add(1);
                            *buffer.offset(fresh55 as isize) =
                                *(*ent).content.offset(0 as ::core::ffi::c_int as isize);
                        } else {
                            nbchars = (nbchars as ::core::ffi::c_ulong).wrapping_add(
                                xmlCopyCharMultiByte(
                                    buffer.offset(nbchars as isize) as *mut xmlChar,
                                    *(*ent).content.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int,
                                ) as ::core::ffi::c_ulong,
                            ) as size_t as size_t;
                        }
                        if nbchars.wrapping_add(XML_PARSER_BUFFER_SIZE as size_t) > buffer_size {
                            let mut tmp_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                            let mut new_size_0: size_t = buffer_size
                                .wrapping_mul(2 as size_t)
                                .wrapping_add(100 as size_t);
                            if new_size_0 < buffer_size {
                                current_block = 15533639510234485578;
                                break;
                            }
                            tmp_0 = xmlRealloc.expect("non-null function pointer")(
                                buffer as *mut ::core::ffi::c_void,
                                new_size_0,
                            ) as *mut xmlChar;
                            if tmp_0.is_null() {
                                current_block = 15533639510234485578;
                                break;
                            }
                            buffer = tmp_0;
                            buffer_size = new_size_0;
                        }
                    } else {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_INTERNAL_ERROR,
                            b"predefined entity has no content\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 1139312351219438743;
                        break;
                    }
                } else if !ent.is_null() && !(*ent).content.is_null() {
                    if check != 0
                        && xmlParserEntityCheck(ctxt, (*ent).length as ::core::ffi::c_ulong) != 0
                    {
                        current_block = 1139312351219438743;
                        break;
                    }
                    if (*ent).flags & XML_ENT_EXPANDING != 0 {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_ENTITY_LOOP,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                        xmlHaltParser(ctxt);
                        *(*ent).content.offset(0 as ::core::ffi::c_int as isize) = 0 as xmlChar;
                        current_block = 1139312351219438743;
                        break;
                    } else {
                        (*ent).flags |= XML_ENT_EXPANDING;
                        (*ctxt).depth += 1;
                        rep = xmlStringDecodeEntitiesInt(
                            ctxt,
                            (*ent).content,
                            (*ent).length,
                            what,
                            0 as xmlChar,
                            0 as xmlChar,
                            0 as xmlChar,
                            check,
                        );
                        (*ctxt).depth -= 1;
                        (*ent).flags &= !XML_ENT_EXPANDING;
                        if rep.is_null() {
                            *(*ent).content.offset(0 as ::core::ffi::c_int as isize) = 0 as xmlChar;
                            current_block = 1139312351219438743;
                            break;
                        } else {
                            current = rep;
                            while *current as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                let fresh56 = current;
                                current = current.offset(1);
                                let fresh57 = nbchars;
                                nbchars = nbchars.wrapping_add(1);
                                *buffer.offset(fresh57 as isize) = *fresh56;
                                if !(nbchars.wrapping_add(XML_PARSER_BUFFER_SIZE as size_t)
                                    > buffer_size)
                                {
                                    continue;
                                }
                                let mut tmp_1: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                let mut new_size_1: size_t = buffer_size
                                    .wrapping_mul(2 as size_t)
                                    .wrapping_add(100 as size_t);
                                if new_size_1 < buffer_size {
                                    current_block = 15533639510234485578;
                                    break 's_63;
                                }
                                tmp_1 = xmlRealloc.expect("non-null function pointer")(
                                    buffer as *mut ::core::ffi::c_void,
                                    new_size_1,
                                ) as *mut xmlChar;
                                if tmp_1.is_null() {
                                    current_block = 15533639510234485578;
                                    break 's_63;
                                }
                                buffer = tmp_1;
                                buffer_size = new_size_1;
                            }
                            xmlFree.expect("non-null function pointer")(
                                rep as *mut ::core::ffi::c_void,
                            );
                            rep = ::core::ptr::null_mut::<xmlChar>();
                        }
                    }
                } else if !ent.is_null() {
                    let mut i: ::core::ffi::c_int = xmlStrlen((*ent).name);
                    let mut cur: *const xmlChar = (*ent).name;
                    let fresh58 = nbchars;
                    nbchars = nbchars.wrapping_add(1);
                    *buffer.offset(fresh58 as isize) = '&' as i32 as xmlChar;
                    if nbchars
                        .wrapping_add(i as size_t)
                        .wrapping_add(XML_PARSER_BUFFER_SIZE as size_t)
                        > buffer_size
                    {
                        let mut tmp_2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        let mut new_size_2: size_t = buffer_size
                            .wrapping_mul(2 as size_t)
                            .wrapping_add(i as size_t)
                            .wrapping_add(100 as size_t);
                        if new_size_2 < buffer_size {
                            current_block = 15533639510234485578;
                            break;
                        }
                        tmp_2 = xmlRealloc.expect("non-null function pointer")(
                            buffer as *mut ::core::ffi::c_void,
                            new_size_2,
                        ) as *mut xmlChar;
                        if tmp_2.is_null() {
                            current_block = 15533639510234485578;
                            break;
                        }
                        buffer = tmp_2;
                        buffer_size = new_size_2;
                    }
                    while i > 0 as ::core::ffi::c_int {
                        let fresh59 = cur;
                        cur = cur.offset(1);
                        let fresh60 = nbchars;
                        nbchars = nbchars.wrapping_add(1);
                        *buffer.offset(fresh60 as isize) = *fresh59;
                        i -= 1;
                    }
                    let fresh61 = nbchars;
                    nbchars = nbchars.wrapping_add(1);
                    *buffer.offset(fresh61 as isize) = ';' as i32 as xmlChar;
                }
            } else if c == '%' as i32 && what & XML_SUBSTITUTE_PEREF != 0 {
                if *__xmlParserDebugEntities() != 0 {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"String decoding PE Reference: %.30s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        str,
                    );
                }
                ent = xmlParseStringPEReference(ctxt, &raw mut str);
                if !ent.is_null() {
                    if (*ent).content.is_null() {
                        if (*ctxt).options & XML_PARSE_NOENT as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                            || (*ctxt).options & XML_PARSE_DTDVALID as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int
                            || (*ctxt).validate != 0 as ::core::ffi::c_int
                        {
                            xmlLoadEntityContent(ctxt, ent);
                        } else {
                            xmlWarningMsg(
                                ctxt,
                                XML_ERR_ENTITY_PROCESSING,
                                b"not validating will not read content for PE entity %s\n\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                (*ent).name,
                                ::core::ptr::null::<xmlChar>(),
                            );
                        }
                    }
                    if check != 0
                        && xmlParserEntityCheck(ctxt, (*ent).length as ::core::ffi::c_ulong) != 0
                    {
                        current_block = 1139312351219438743;
                        break;
                    }
                    if (*ent).flags & XML_ENT_EXPANDING != 0 {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_ENTITY_LOOP,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                        xmlHaltParser(ctxt);
                        if !(*ent).content.is_null() {
                            *(*ent).content.offset(0 as ::core::ffi::c_int as isize) = 0 as xmlChar;
                        }
                        current_block = 1139312351219438743;
                        break;
                    } else {
                        (*ent).flags |= XML_ENT_EXPANDING;
                        (*ctxt).depth += 1;
                        rep = xmlStringDecodeEntitiesInt(
                            ctxt,
                            (*ent).content,
                            (*ent).length,
                            what,
                            0 as xmlChar,
                            0 as xmlChar,
                            0 as xmlChar,
                            check,
                        );
                        (*ctxt).depth -= 1;
                        (*ent).flags &= !XML_ENT_EXPANDING;
                        if rep.is_null() {
                            if !(*ent).content.is_null() {
                                *(*ent).content.offset(0 as ::core::ffi::c_int as isize) =
                                    0 as xmlChar;
                            }
                            current_block = 1139312351219438743;
                            break;
                        } else {
                            current = rep;
                            while *current as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                let fresh62 = current;
                                current = current.offset(1);
                                let fresh63 = nbchars;
                                nbchars = nbchars.wrapping_add(1);
                                *buffer.offset(fresh63 as isize) = *fresh62;
                                if !(nbchars.wrapping_add(XML_PARSER_BUFFER_SIZE as size_t)
                                    > buffer_size)
                                {
                                    continue;
                                }
                                let mut tmp_3: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                let mut new_size_3: size_t = buffer_size
                                    .wrapping_mul(2 as size_t)
                                    .wrapping_add(100 as size_t);
                                if new_size_3 < buffer_size {
                                    current_block = 15533639510234485578;
                                    break 's_63;
                                }
                                tmp_3 = xmlRealloc.expect("non-null function pointer")(
                                    buffer as *mut ::core::ffi::c_void,
                                    new_size_3,
                                ) as *mut xmlChar;
                                if tmp_3.is_null() {
                                    current_block = 15533639510234485578;
                                    break 's_63;
                                }
                                buffer = tmp_3;
                                buffer_size = new_size_3;
                            }
                            xmlFree.expect("non-null function pointer")(
                                rep as *mut ::core::ffi::c_void,
                            );
                            rep = ::core::ptr::null_mut::<xmlChar>();
                        }
                    }
                }
            } else {
                if c < 0x80 as ::core::ffi::c_int {
                    let fresh64 = nbchars;
                    nbchars = nbchars.wrapping_add(1);
                    *buffer.offset(fresh64 as isize) = c as xmlChar;
                } else {
                    nbchars = (nbchars as ::core::ffi::c_ulong).wrapping_add(xmlCopyCharMultiByte(
                        buffer.offset(nbchars as isize) as *mut xmlChar,
                        c,
                    )
                        as ::core::ffi::c_ulong) as size_t as size_t;
                }
                str = str.offset(l as isize);
                if nbchars.wrapping_add(XML_PARSER_BUFFER_SIZE as size_t) > buffer_size {
                    let mut tmp_4: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    let mut new_size_4: size_t = buffer_size
                        .wrapping_mul(2 as size_t)
                        .wrapping_add(100 as size_t);
                    if new_size_4 < buffer_size {
                        current_block = 15533639510234485578;
                        break;
                    }
                    tmp_4 = xmlRealloc.expect("non-null function pointer")(
                        buffer as *mut ::core::ffi::c_void,
                        new_size_4,
                    ) as *mut xmlChar;
                    if tmp_4.is_null() {
                        current_block = 15533639510234485578;
                        break;
                    }
                    buffer = tmp_4;
                    buffer_size = new_size_4;
                }
            }
            if str < last {
                c = xmlStringCurrentChar(ctxt, str, &raw mut l);
            } else {
                c = 0 as ::core::ffi::c_int;
            }
        }
        match current_block {
            15533639510234485578 => {}
            1139312351219438743 => {}
            _ => {
                *buffer.offset(nbchars as isize) = 0 as xmlChar;
                return buffer;
            }
        }
    }
    match current_block {
        15533639510234485578 => {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        }
        _ => {}
    }
    if !rep.is_null() {
        xmlFree.expect("non-null function pointer")(rep as *mut ::core::ffi::c_void);
    }
    if !buffer.is_null() {
        xmlFree.expect("non-null function pointer")(buffer as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlStringLenDecodeEntities(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut len: ::core::ffi::c_int,
    mut what: ::core::ffi::c_int,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
) -> *mut xmlChar {
    if ctxt.is_null() || str.is_null() || len < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStringDecodeEntitiesInt(
        ctxt,
        str,
        len,
        what,
        end,
        end2,
        end3,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlStringDecodeEntities(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut what: ::core::ffi::c_int,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
) -> *mut xmlChar {
    if ctxt.is_null() || str.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStringDecodeEntitiesInt(
        ctxt,
        str,
        xmlStrlen(str),
        what,
        end,
        end2,
        end3,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn areBlanks(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut len: ::core::ffi::c_int,
    mut blank_chars: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut lastChild: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if (*(*ctxt).sax).ignorableWhitespace == (*(*ctxt).sax).characters {
        return 0 as ::core::ffi::c_int;
    }
    if (*ctxt).space.is_null()
        || *(*ctxt).space == 1 as ::core::ffi::c_int
        || *(*ctxt).space == -(2 as ::core::ffi::c_int)
    {
        return 0 as ::core::ffi::c_int;
    }
    if blank_chars == 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < len {
            if !(*str.offset(i as isize) as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int <= *str.offset(i as isize) as ::core::ffi::c_int
                    && *str.offset(i as isize) as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                || *str.offset(i as isize) as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
            {
                return 0 as ::core::ffi::c_int;
            }
            i += 1;
        }
    }
    if (*ctxt).node.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*ctxt).myDoc.is_null() {
        ret = xmlIsMixedElement((*ctxt).myDoc, (*(*ctxt).node).name);
        if ret == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if ret == 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        && *(*(*ctxt).input).cur as ::core::ffi::c_int != 0xd as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*(*ctxt).node).children.is_null()
        && *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        return 0 as ::core::ffi::c_int;
    }
    lastChild = xmlGetLastChild((*ctxt).node as *const xmlNode);
    if lastChild.is_null() {
        if (*(*ctxt).node).type_0 as ::core::ffi::c_uint
            != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            && !(*(*ctxt).node).content.is_null()
        {
            return 0 as ::core::ffi::c_int;
        }
    } else if xmlNodeIsText(lastChild as *const xmlNode) != 0 {
        return 0 as ::core::ffi::c_int;
    } else if !(*(*ctxt).node).children.is_null() && xmlNodeIsText((*(*ctxt).node).children) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSplitQName(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut prefix: *mut *mut xmlChar,
) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max: ::core::ffi::c_int = XML_MAX_NAMELEN;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: *const xmlChar = name;
    let mut c: ::core::ffi::c_int = 0;
    if prefix.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    *prefix = ::core::ptr::null_mut::<xmlChar>();
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if *cur.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32 {
        return xmlStrdup(name);
    }
    let fresh109 = cur;
    cur = cur.offset(1);
    c = *fresh109 as ::core::ffi::c_int;
    while c != 0 as ::core::ffi::c_int && c != ':' as i32 && len < max {
        let fresh110 = len;
        len = len + 1;
        buf[fresh110 as usize] = c as xmlChar;
        let fresh111 = cur;
        cur = cur.offset(1);
        c = *fresh111 as ::core::ffi::c_int;
    }
    if len >= max {
        max = len * 2 as ::core::ffi::c_int;
        buffer = xmlMallocAtomic.expect("non-null function pointer")(max as size_t) as *mut xmlChar;
        if buffer.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return ::core::ptr::null_mut::<xmlChar>();
        }
        memcpy(
            buffer as *mut ::core::ffi::c_void,
            &raw mut buf as *mut xmlChar as *const ::core::ffi::c_void,
            len as size_t,
        );
        while c != 0 as ::core::ffi::c_int && c != ':' as i32 {
            if len + 10 as ::core::ffi::c_int > max {
                let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                max *= 2 as ::core::ffi::c_int;
                tmp = xmlRealloc.expect("non-null function pointer")(
                    buffer as *mut ::core::ffi::c_void,
                    max as size_t,
                ) as *mut xmlChar;
                if tmp.is_null() {
                    xmlFree.expect("non-null function pointer")(buffer as *mut ::core::ffi::c_void);
                    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                    return ::core::ptr::null_mut::<xmlChar>();
                }
                buffer = tmp;
            }
            let fresh112 = len;
            len = len + 1;
            *buffer.offset(fresh112 as isize) = c as xmlChar;
            let fresh113 = cur;
            cur = cur.offset(1);
            c = *fresh113 as ::core::ffi::c_int;
        }
        *buffer.offset(len as isize) = 0 as xmlChar;
    }
    if c == ':' as i32 && *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if !buffer.is_null() {
            xmlFree.expect("non-null function pointer")(buffer as *mut ::core::ffi::c_void);
        }
        *prefix = ::core::ptr::null_mut::<xmlChar>();
        return xmlStrdup(name);
    }
    if buffer.is_null() {
        ret = xmlStrndup(&raw mut buf as *mut xmlChar, len);
    } else {
        ret = buffer;
        buffer = ::core::ptr::null_mut::<xmlChar>();
        max = XML_MAX_NAMELEN;
    }
    if c == ':' as i32 {
        c = *cur as ::core::ffi::c_int;
        *prefix = ret;
        if c == 0 as ::core::ffi::c_int {
            return xmlStrndup(
                b"\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                0 as ::core::ffi::c_int,
            );
        }
        len = 0 as ::core::ffi::c_int;
        if !(c >= 0x61 as ::core::ffi::c_int && c <= 0x7a as ::core::ffi::c_int
            || c >= 0x41 as ::core::ffi::c_int && c <= 0x5a as ::core::ffi::c_int
            || c == '_' as i32
            || c == ':' as i32)
        {
            let mut l: ::core::ffi::c_int = 0;
            let mut first: ::core::ffi::c_int = xmlStringCurrentChar(ctxt, cur, &raw mut l);
            if !((if first < 0x100 as ::core::ffi::c_int {
                (0x41 as ::core::ffi::c_int <= first && first <= 0x5a as ::core::ffi::c_int
                    || 0x61 as ::core::ffi::c_int <= first && first <= 0x7a as ::core::ffi::c_int
                    || 0xc0 as ::core::ffi::c_int <= first && first <= 0xd6 as ::core::ffi::c_int
                    || 0xd8 as ::core::ffi::c_int <= first && first <= 0xf6 as ::core::ffi::c_int
                    || 0xf8 as ::core::ffi::c_int <= first) as ::core::ffi::c_int
            } else {
                xmlCharInRange(first as ::core::ffi::c_uint, &raw const xmlIsBaseCharGroup)
            }) != 0
                || (if first < 0x100 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int
                } else {
                    (0x4e00 as ::core::ffi::c_int <= first && first <= 0x9fa5 as ::core::ffi::c_int
                        || first == 0x3007 as ::core::ffi::c_int
                        || 0x3021 as ::core::ffi::c_int <= first
                            && first <= 0x3029 as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                }) != 0)
                && first != '_' as i32
            {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_NS_ERR_QNAME,
                    b"Name %s is not XML Namespace compliant\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    name,
                );
            }
        }
        cur = cur.offset(1);
        while c != 0 as ::core::ffi::c_int && len < max {
            let fresh114 = len;
            len = len + 1;
            buf[fresh114 as usize] = c as xmlChar;
            let fresh115 = cur;
            cur = cur.offset(1);
            c = *fresh115 as ::core::ffi::c_int;
        }
        if len >= max {
            max = len * 2 as ::core::ffi::c_int;
            buffer =
                xmlMallocAtomic.expect("non-null function pointer")(max as size_t) as *mut xmlChar;
            if buffer.is_null() {
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                return ::core::ptr::null_mut::<xmlChar>();
            }
            memcpy(
                buffer as *mut ::core::ffi::c_void,
                &raw mut buf as *mut xmlChar as *const ::core::ffi::c_void,
                len as size_t,
            );
            while c != 0 as ::core::ffi::c_int {
                if len + 10 as ::core::ffi::c_int > max {
                    let mut tmp_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    max *= 2 as ::core::ffi::c_int;
                    tmp_0 = xmlRealloc.expect("non-null function pointer")(
                        buffer as *mut ::core::ffi::c_void,
                        max as size_t,
                    ) as *mut xmlChar;
                    if tmp_0.is_null() {
                        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                        xmlFree.expect("non-null function pointer")(
                            buffer as *mut ::core::ffi::c_void,
                        );
                        return ::core::ptr::null_mut::<xmlChar>();
                    }
                    buffer = tmp_0;
                }
                let fresh116 = len;
                len = len + 1;
                *buffer.offset(fresh116 as isize) = c as xmlChar;
                let fresh117 = cur;
                cur = cur.offset(1);
                c = *fresh117 as ::core::ffi::c_int;
            }
            *buffer.offset(len as isize) = 0 as xmlChar;
        }
        if buffer.is_null() {
            ret = xmlStrndup(&raw mut buf as *mut xmlChar, len);
        } else {
            ret = buffer;
        }
    }
    return ret;
}
unsafe extern "C" fn xmlIsNameStartChar(
    mut ctxt: xmlParserCtxtPtr,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*ctxt).options & XML_PARSE_OLD10 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if c != ' ' as i32
            && c != '>' as i32
            && c != '/' as i32
            && (c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32
                || c == '_' as i32
                || c == ':' as i32
                || c >= 0xc0 as ::core::ffi::c_int && c <= 0xd6 as ::core::ffi::c_int
                || c >= 0xd8 as ::core::ffi::c_int && c <= 0xf6 as ::core::ffi::c_int
                || c >= 0xf8 as ::core::ffi::c_int && c <= 0x2ff as ::core::ffi::c_int
                || c >= 0x370 as ::core::ffi::c_int && c <= 0x37d as ::core::ffi::c_int
                || c >= 0x37f as ::core::ffi::c_int && c <= 0x1fff as ::core::ffi::c_int
                || c >= 0x200c as ::core::ffi::c_int && c <= 0x200d as ::core::ffi::c_int
                || c >= 0x2070 as ::core::ffi::c_int && c <= 0x218f as ::core::ffi::c_int
                || c >= 0x2c00 as ::core::ffi::c_int && c <= 0x2fef as ::core::ffi::c_int
                || c >= 0x3001 as ::core::ffi::c_int && c <= 0xd7ff as ::core::ffi::c_int
                || c >= 0xf900 as ::core::ffi::c_int && c <= 0xfdcf as ::core::ffi::c_int
                || c >= 0xfdf0 as ::core::ffi::c_int && c <= 0xfffd as ::core::ffi::c_int
                || c >= 0x10000 as ::core::ffi::c_int && c <= 0xeffff as ::core::ffi::c_int)
        {
            return 1 as ::core::ffi::c_int;
        }
    } else if (if c < 0x100 as ::core::ffi::c_int {
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
        || c == '_' as i32
        || c == ':' as i32
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlIsNameChar(
    mut ctxt: xmlParserCtxtPtr,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*ctxt).options & XML_PARSE_OLD10 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if c != ' ' as i32
            && c != '>' as i32
            && c != '/' as i32
            && (c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32
                || c >= '0' as i32 && c <= '9' as i32
                || c == '_' as i32
                || c == ':' as i32
                || c == '-' as i32
                || c == '.' as i32
                || c == 0xb7 as ::core::ffi::c_int
                || c >= 0xc0 as ::core::ffi::c_int && c <= 0xd6 as ::core::ffi::c_int
                || c >= 0xd8 as ::core::ffi::c_int && c <= 0xf6 as ::core::ffi::c_int
                || c >= 0xf8 as ::core::ffi::c_int && c <= 0x2ff as ::core::ffi::c_int
                || c >= 0x300 as ::core::ffi::c_int && c <= 0x36f as ::core::ffi::c_int
                || c >= 0x370 as ::core::ffi::c_int && c <= 0x37d as ::core::ffi::c_int
                || c >= 0x37f as ::core::ffi::c_int && c <= 0x1fff as ::core::ffi::c_int
                || c >= 0x200c as ::core::ffi::c_int && c <= 0x200d as ::core::ffi::c_int
                || c >= 0x203f as ::core::ffi::c_int && c <= 0x2040 as ::core::ffi::c_int
                || c >= 0x2070 as ::core::ffi::c_int && c <= 0x218f as ::core::ffi::c_int
                || c >= 0x2c00 as ::core::ffi::c_int && c <= 0x2fef as ::core::ffi::c_int
                || c >= 0x3001 as ::core::ffi::c_int && c <= 0xd7ff as ::core::ffi::c_int
                || c >= 0xf900 as ::core::ffi::c_int && c <= 0xfdcf as ::core::ffi::c_int
                || c >= 0xfdf0 as ::core::ffi::c_int && c <= 0xfffd as ::core::ffi::c_int
                || c >= 0x10000 as ::core::ffi::c_int && c <= 0xeffff as ::core::ffi::c_int)
        {
            return 1 as ::core::ffi::c_int;
        }
    } else if (if c < 0x100 as ::core::ffi::c_int {
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
        || (if c < 0x100 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            xmlCharInRange(c as ::core::ffi::c_uint, &raw const xmlIsCombiningGroup)
        }) != 0
        || (if c < 0x100 as ::core::ffi::c_int {
            (c == 0xb7 as ::core::ffi::c_int) as ::core::ffi::c_int
        } else {
            xmlCharInRange(c as ::core::ffi::c_uint, &raw const xmlIsExtenderGroup)
        }) != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParseNameComplex(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut l: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_TEXT_LENGTH
        } else {
            XML_MAX_NAME_LENGTH
        };
    c = xmlCurrentChar(ctxt, &raw mut l);
    if (*ctxt).options & XML_PARSE_OLD10 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if c == ' ' as i32
            || c == '>' as i32
            || c == '/' as i32
            || !(c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32
                || c == '_' as i32
                || c == ':' as i32
                || c >= 0xc0 as ::core::ffi::c_int && c <= 0xd6 as ::core::ffi::c_int
                || c >= 0xd8 as ::core::ffi::c_int && c <= 0xf6 as ::core::ffi::c_int
                || c >= 0xf8 as ::core::ffi::c_int && c <= 0x2ff as ::core::ffi::c_int
                || c >= 0x370 as ::core::ffi::c_int && c <= 0x37d as ::core::ffi::c_int
                || c >= 0x37f as ::core::ffi::c_int && c <= 0x1fff as ::core::ffi::c_int
                || c >= 0x200c as ::core::ffi::c_int && c <= 0x200d as ::core::ffi::c_int
                || c >= 0x2070 as ::core::ffi::c_int && c <= 0x218f as ::core::ffi::c_int
                || c >= 0x2c00 as ::core::ffi::c_int && c <= 0x2fef as ::core::ffi::c_int
                || c >= 0x3001 as ::core::ffi::c_int && c <= 0xd7ff as ::core::ffi::c_int
                || c >= 0xf900 as ::core::ffi::c_int && c <= 0xfdcf as ::core::ffi::c_int
                || c >= 0xfdf0 as ::core::ffi::c_int && c <= 0xfffd as ::core::ffi::c_int
                || c >= 0x10000 as ::core::ffi::c_int && c <= 0xeffff as ::core::ffi::c_int)
        {
            return ::core::ptr::null::<xmlChar>();
        }
        len += l;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
        c = xmlCurrentChar(ctxt, &raw mut l);
        while c != ' ' as i32
            && c != '>' as i32
            && c != '/' as i32
            && (c >= 'a' as i32 && c <= 'z' as i32
                || c >= 'A' as i32 && c <= 'Z' as i32
                || c >= '0' as i32 && c <= '9' as i32
                || c == '_' as i32
                || c == ':' as i32
                || c == '-' as i32
                || c == '.' as i32
                || c == 0xb7 as ::core::ffi::c_int
                || c >= 0xc0 as ::core::ffi::c_int && c <= 0xd6 as ::core::ffi::c_int
                || c >= 0xd8 as ::core::ffi::c_int && c <= 0xf6 as ::core::ffi::c_int
                || c >= 0xf8 as ::core::ffi::c_int && c <= 0x2ff as ::core::ffi::c_int
                || c >= 0x300 as ::core::ffi::c_int && c <= 0x36f as ::core::ffi::c_int
                || c >= 0x370 as ::core::ffi::c_int && c <= 0x37d as ::core::ffi::c_int
                || c >= 0x37f as ::core::ffi::c_int && c <= 0x1fff as ::core::ffi::c_int
                || c >= 0x200c as ::core::ffi::c_int && c <= 0x200d as ::core::ffi::c_int
                || c >= 0x203f as ::core::ffi::c_int && c <= 0x2040 as ::core::ffi::c_int
                || c >= 0x2070 as ::core::ffi::c_int && c <= 0x218f as ::core::ffi::c_int
                || c >= 0x2c00 as ::core::ffi::c_int && c <= 0x2fef as ::core::ffi::c_int
                || c >= 0x3001 as ::core::ffi::c_int && c <= 0xd7ff as ::core::ffi::c_int
                || c >= 0xf900 as ::core::ffi::c_int && c <= 0xfdcf as ::core::ffi::c_int
                || c >= 0xfdf0 as ::core::ffi::c_int && c <= 0xfffd as ::core::ffi::c_int
                || c >= 0x10000 as ::core::ffi::c_int && c <= 0xeffff as ::core::ffi::c_int)
        {
            if len <= INT_MAX - l {
                len += l;
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
            } else {
                (*(*ctxt).input).col += 1;
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            c = xmlCurrentChar(ctxt, &raw mut l);
        }
    } else {
        if c == ' ' as i32
            || c == '>' as i32
            || c == '/' as i32
            || !((if c < 0x100 as ::core::ffi::c_int {
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
        len += l;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
        c = xmlCurrentChar(ctxt, &raw mut l);
        while c != ' ' as i32
            && c != '>' as i32
            && c != '/' as i32
            && ((if c < 0x100 as ::core::ffi::c_int {
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
                || (if c < 0x100 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int
                } else {
                    xmlCharInRange(c as ::core::ffi::c_uint, &raw const xmlIsCombiningGroup)
                }) != 0
                || (if c < 0x100 as ::core::ffi::c_int {
                    (c == 0xb7 as ::core::ffi::c_int) as ::core::ffi::c_int
                } else {
                    xmlCharInRange(c as ::core::ffi::c_uint, &raw const xmlIsExtenderGroup)
                }) != 0)
        {
            if len <= INT_MAX - l {
                len += l;
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
            } else {
                (*(*ctxt).input).col += 1;
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            c = xmlCurrentChar(ctxt, &raw mut l);
        }
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    if len > maxLength {
        xmlFatalErr(
            ctxt,
            XML_ERR_NAME_TOO_LONG,
            b"Name\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<xmlChar>();
    }
    if ((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long)
        < len as ::core::ffi::c_long
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"unexpected change of input buffer\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<xmlChar>();
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '\r' as i32
    {
        return xmlDictLookup(
            (*ctxt).dict,
            (*(*ctxt).input)
                .cur
                .offset(-((len + 1 as ::core::ffi::c_int) as isize)),
            len,
        );
    }
    return xmlDictLookup(
        (*ctxt).dict,
        (*(*ctxt).input).cur.offset(-(len as isize)),
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseName(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut count: size_t = 0 as size_t;
    let mut maxLength: size_t = (if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
        XML_MAX_TEXT_LENGTH
    } else {
        XML_MAX_NAME_LENGTH
    }) as size_t;
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    in_0 = (*(*ctxt).input).cur;
    if *in_0 as ::core::ffi::c_int >= 0x61 as ::core::ffi::c_int
        && *in_0 as ::core::ffi::c_int <= 0x7a as ::core::ffi::c_int
        || *in_0 as ::core::ffi::c_int >= 0x41 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int <= 0x5a as ::core::ffi::c_int
        || *in_0 as ::core::ffi::c_int == '_' as i32
        || *in_0 as ::core::ffi::c_int == ':' as i32
    {
        in_0 = in_0.offset(1);
        while *in_0 as ::core::ffi::c_int >= 0x61 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int <= 0x7a as ::core::ffi::c_int
            || *in_0 as ::core::ffi::c_int >= 0x41 as ::core::ffi::c_int
                && *in_0 as ::core::ffi::c_int <= 0x5a as ::core::ffi::c_int
            || *in_0 as ::core::ffi::c_int >= 0x30 as ::core::ffi::c_int
                && *in_0 as ::core::ffi::c_int <= 0x39 as ::core::ffi::c_int
            || *in_0 as ::core::ffi::c_int == '_' as i32
            || *in_0 as ::core::ffi::c_int == '-' as i32
            || *in_0 as ::core::ffi::c_int == ':' as i32
            || *in_0 as ::core::ffi::c_int == '.' as i32
        {
            in_0 = in_0.offset(1);
        }
        if *in_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (*in_0 as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
        {
            count = in_0.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
            if count > maxLength {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_NAME_TOO_LONG,
                    b"Name\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return ::core::ptr::null::<xmlChar>();
            }
            ret = xmlDictLookup(
                (*ctxt).dict,
                (*(*ctxt).input).cur,
                count as ::core::ffi::c_int,
            );
            (*(*ctxt).input).cur = in_0;
            (*(*ctxt).input).col = ((*(*ctxt).input).col as ::core::ffi::c_ulong)
                .wrapping_add(count as ::core::ffi::c_ulong)
                as ::core::ffi::c_int as ::core::ffi::c_int;
            if ret.is_null() {
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            }
            return ret;
        }
    }
    return xmlParseNameComplex(ctxt);
}
unsafe extern "C" fn xmlParseNCNameComplex(mut ctxt: xmlParserCtxtPtr) -> xmlHashedString {
    let mut ret: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut l: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_TEXT_LENGTH
        } else {
            XML_MAX_NAME_LENGTH
        };
    let mut startPosition: size_t = 0 as size_t;
    ret.name = ::core::ptr::null::<xmlChar>();
    ret.hashValue = 0 as ::core::ffi::c_uint;
    startPosition =
        (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long as size_t;
    c = xmlCurrentChar(ctxt, &raw mut l);
    if c == ' ' as i32
        || c == '>' as i32
        || c == '/' as i32
        || (xmlIsNameStartChar(ctxt, c) == 0 || c == ':' as i32)
    {
        return ret;
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && (xmlIsNameChar(ctxt, c) != 0 && c != ':' as i32)
    {
        if len <= INT_MAX - l {
            len += l;
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
        c = xmlCurrentChar(ctxt, &raw mut l);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return ret;
    }
    if len > maxLength {
        xmlFatalErr(
            ctxt,
            XML_ERR_NAME_TOO_LONG,
            b"NCName\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ret;
    }
    ret = xmlDictLookupHashed(
        (*ctxt).dict,
        (*(*ctxt).input).base.offset(startPosition as isize),
        len,
    );
    return ret;
}
unsafe extern "C" fn xmlParseNCName(mut ctxt: xmlParserCtxtPtr) -> xmlHashedString {
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut e: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut count: size_t = 0 as size_t;
    let mut maxLength: size_t = (if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
        XML_MAX_TEXT_LENGTH
    } else {
        XML_MAX_NAME_LENGTH
    }) as size_t;
    ret.name = ::core::ptr::null::<xmlChar>();
    in_0 = (*(*ctxt).input).cur;
    e = (*(*ctxt).input).end;
    if (*in_0 as ::core::ffi::c_int >= 0x61 as ::core::ffi::c_int
        && *in_0 as ::core::ffi::c_int <= 0x7a as ::core::ffi::c_int
        || *in_0 as ::core::ffi::c_int >= 0x41 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int <= 0x5a as ::core::ffi::c_int
        || *in_0 as ::core::ffi::c_int == '_' as i32)
        && in_0 < e
    {
        in_0 = in_0.offset(1);
        while (*in_0 as ::core::ffi::c_int >= 0x61 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int <= 0x7a as ::core::ffi::c_int
            || *in_0 as ::core::ffi::c_int >= 0x41 as ::core::ffi::c_int
                && *in_0 as ::core::ffi::c_int <= 0x5a as ::core::ffi::c_int
            || *in_0 as ::core::ffi::c_int >= 0x30 as ::core::ffi::c_int
                && *in_0 as ::core::ffi::c_int <= 0x39 as ::core::ffi::c_int
            || *in_0 as ::core::ffi::c_int == '_' as i32
            || *in_0 as ::core::ffi::c_int == '-' as i32
            || *in_0 as ::core::ffi::c_int == '.' as i32)
            && in_0 < e
        {
            in_0 = in_0.offset(1);
        }
        if !(in_0 >= e) {
            if *in_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && (*in_0 as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int
            {
                count = in_0.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
                if count > maxLength {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NAME_TOO_LONG,
                        b"NCName\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return ret;
                }
                ret = xmlDictLookupHashed(
                    (*ctxt).dict,
                    (*(*ctxt).input).cur,
                    count as ::core::ffi::c_int,
                );
                (*(*ctxt).input).cur = in_0;
                (*(*ctxt).input).col = ((*(*ctxt).input).col as ::core::ffi::c_ulong)
                    .wrapping_add(count as ::core::ffi::c_ulong)
                    as ::core::ffi::c_int
                    as ::core::ffi::c_int;
                if ret.name.is_null() {
                    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                }
                return ret;
            }
        }
    }
    return xmlParseNCNameComplex(ctxt);
}
unsafe extern "C" fn xmlParseNameAndCompare(
    mut ctxt: xmlParserCtxtPtr,
    mut other: *const xmlChar,
) -> *const xmlChar {
    let mut cmp: *const xmlChar = other;
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    in_0 = (*(*ctxt).input).cur;
    while *in_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && *in_0 as ::core::ffi::c_int == *cmp as ::core::ffi::c_int
    {
        in_0 = in_0.offset(1);
        cmp = cmp.offset(1);
    }
    if *cmp as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*in_0 as ::core::ffi::c_int == '>' as i32
            || (*in_0 as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int <= *in_0 as ::core::ffi::c_int
                    && *in_0 as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0xd as ::core::ffi::c_int))
    {
        (*(*ctxt).input).col = ((*(*ctxt).input).col as ::core::ffi::c_long
            + in_0.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            as ::core::ffi::c_int;
        (*(*ctxt).input).cur = in_0;
        return 1 as ::core::ffi::c_int as *const xmlChar;
    }
    ret = xmlParseName(ctxt);
    if ret == other {
        return 1 as ::core::ffi::c_int as *const xmlChar;
    }
    return ret;
}
unsafe extern "C" fn xmlParseStringName(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut cur: *const xmlChar = *str;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut l: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_TEXT_LENGTH
        } else {
            XML_MAX_NAME_LENGTH
        };
    c = xmlStringCurrentChar(ctxt, cur, &raw mut l);
    if xmlIsNameStartChar(ctxt, c) == 0 {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if c < 0x80 as ::core::ffi::c_int {
        let fresh65 = len;
        len = len + 1;
        buf[fresh65 as usize] = c as xmlChar;
    } else {
        len += xmlCopyCharMultiByte(
            (&raw mut buf as *mut xmlChar).offset(len as isize) as *mut xmlChar,
            c,
        );
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(ctxt, cur, &raw mut l);
    while xmlIsNameChar(ctxt, c) != 0 {
        if c < 0x80 as ::core::ffi::c_int {
            let fresh66 = len;
            len = len + 1;
            buf[fresh66 as usize] = c as xmlChar;
        } else {
            len += xmlCopyCharMultiByte(
                (&raw mut buf as *mut xmlChar).offset(len as isize) as *mut xmlChar,
                c,
            );
        }
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(ctxt, cur, &raw mut l);
        if len >= XML_MAX_NAMELEN {
            let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut max: ::core::ffi::c_int = len * 2 as ::core::ffi::c_int;
            buffer =
                xmlMallocAtomic.expect("non-null function pointer")(max as size_t) as *mut xmlChar;
            if buffer.is_null() {
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                return ::core::ptr::null_mut::<xmlChar>();
            }
            memcpy(
                buffer as *mut ::core::ffi::c_void,
                &raw mut buf as *mut xmlChar as *const ::core::ffi::c_void,
                len as size_t,
            );
            while xmlIsNameChar(ctxt, c) != 0 {
                if len + 10 as ::core::ffi::c_int > max {
                    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    max *= 2 as ::core::ffi::c_int;
                    tmp = xmlRealloc.expect("non-null function pointer")(
                        buffer as *mut ::core::ffi::c_void,
                        max as size_t,
                    ) as *mut xmlChar;
                    if tmp.is_null() {
                        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                        xmlFree.expect("non-null function pointer")(
                            buffer as *mut ::core::ffi::c_void,
                        );
                        return ::core::ptr::null_mut::<xmlChar>();
                    }
                    buffer = tmp;
                }
                if c < 0x80 as ::core::ffi::c_int {
                    let fresh67 = len;
                    len = len + 1;
                    *buffer.offset(fresh67 as isize) = c as xmlChar;
                } else {
                    len += xmlCopyCharMultiByte(buffer.offset(len as isize) as *mut xmlChar, c);
                }
                cur = cur.offset(l as isize);
                c = xmlStringCurrentChar(ctxt, cur, &raw mut l);
                if len > maxLength {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NAME_TOO_LONG,
                        b"NCName\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    xmlFree.expect("non-null function pointer")(buffer as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<xmlChar>();
                }
            }
            *buffer.offset(len as isize) = 0 as xmlChar;
            *str = cur;
            return buffer;
        }
    }
    if len > maxLength {
        xmlFatalErr(
            ctxt,
            XML_ERR_NAME_TOO_LONG,
            b"NCName\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    *str = cur;
    return xmlStrndup(&raw mut buf as *mut xmlChar, len);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseNmtoken(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut l: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_TEXT_LENGTH
        } else {
            XML_MAX_NAME_LENGTH
        };
    c = xmlCurrentChar(ctxt, &raw mut l);
    while xmlIsNameChar(ctxt, c) != 0 {
        if c < 0x80 as ::core::ffi::c_int {
            let fresh11 = len;
            len = len + 1;
            buf[fresh11 as usize] = c as xmlChar;
        } else {
            len += xmlCopyCharMultiByte(
                (&raw mut buf as *mut xmlChar).offset(len as isize) as *mut xmlChar,
                c,
            );
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
        c = xmlCurrentChar(ctxt, &raw mut l);
        if len >= XML_MAX_NAMELEN {
            let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut max: ::core::ffi::c_int = len * 2 as ::core::ffi::c_int;
            buffer =
                xmlMallocAtomic.expect("non-null function pointer")(max as size_t) as *mut xmlChar;
            if buffer.is_null() {
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                return ::core::ptr::null_mut::<xmlChar>();
            }
            memcpy(
                buffer as *mut ::core::ffi::c_void,
                &raw mut buf as *mut xmlChar as *const ::core::ffi::c_void,
                len as size_t,
            );
            while xmlIsNameChar(ctxt, c) != 0 {
                if len + 10 as ::core::ffi::c_int > max {
                    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    max *= 2 as ::core::ffi::c_int;
                    tmp = xmlRealloc.expect("non-null function pointer")(
                        buffer as *mut ::core::ffi::c_void,
                        max as size_t,
                    ) as *mut xmlChar;
                    if tmp.is_null() {
                        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                        xmlFree.expect("non-null function pointer")(
                            buffer as *mut ::core::ffi::c_void,
                        );
                        return ::core::ptr::null_mut::<xmlChar>();
                    }
                    buffer = tmp;
                }
                if c < 0x80 as ::core::ffi::c_int {
                    let fresh12 = len;
                    len = len + 1;
                    *buffer.offset(fresh12 as isize) = c as xmlChar;
                } else {
                    len += xmlCopyCharMultiByte(buffer.offset(len as isize) as *mut xmlChar, c);
                }
                if len > maxLength {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NAME_TOO_LONG,
                        b"NmToken\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    xmlFree.expect("non-null function pointer")(buffer as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<xmlChar>();
                }
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                } else {
                    (*(*ctxt).input).col += 1;
                }
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                c = xmlCurrentChar(ctxt, &raw mut l);
            }
            *buffer.offset(len as isize) = 0 as xmlChar;
            if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                xmlFree.expect("non-null function pointer")(buffer as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            return buffer;
        }
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if len == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if len > maxLength {
        xmlFatalErr(
            ctxt,
            XML_ERR_NAME_TOO_LONG,
            b"NmToken\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStrndup(&raw mut buf as *mut xmlChar, len);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEntityValue(
    mut ctxt: xmlParserCtxtPtr,
    mut orig: *mut *mut xmlChar,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = XML_PARSER_BUFFER_SIZE;
    let mut c: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_HUGE_LENGTH
        } else {
            XML_MAX_TEXT_LENGTH
        };
    let mut stop: xmlChar = 0;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '"' as i32 {
        stop = '"' as i32 as xmlChar;
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\'' as i32 {
        stop = '\'' as i32 as xmlChar;
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_ENTITY_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buf = xmlMallocAtomic.expect("non-null function pointer")(size as size_t) as *mut xmlChar;
    if buf.is_null() {
        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        return ::core::ptr::null_mut::<xmlChar>();
    }
    (*ctxt).instate = XML_PARSER_ENTITY_VALUE;
    input = (*ctxt).input;
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if !((*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int) {
        xmlNextChar(ctxt);
        c = xmlCurrentChar(ctxt, &raw mut l);
        loop {
            if !((if c < 0x100 as ::core::ffi::c_int {
                (0x9 as ::core::ffi::c_int <= c && c <= 0xa as ::core::ffi::c_int
                    || c == 0xd as ::core::ffi::c_int
                    || 0x20 as ::core::ffi::c_int <= c) as ::core::ffi::c_int
            } else {
                (0x100 as ::core::ffi::c_int <= c && c <= 0xd7ff as ::core::ffi::c_int
                    || 0xe000 as ::core::ffi::c_int <= c && c <= 0xfffd as ::core::ffi::c_int
                    || 0x10000 as ::core::ffi::c_int <= c && c <= 0x10ffff as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            }) != 0
                && (c != stop as ::core::ffi::c_int || (*ctxt).input != input)
                && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int)
            {
                current_block = 10758786907990354186;
                break;
            }
            if len + 5 as ::core::ffi::c_int >= size {
                let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                size *= 2 as ::core::ffi::c_int;
                tmp = xmlRealloc.expect("non-null function pointer")(
                    buf as *mut ::core::ffi::c_void,
                    size as size_t,
                ) as *mut xmlChar;
                if tmp.is_null() {
                    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                    current_block = 10900495365614327488;
                    break;
                } else {
                    buf = tmp;
                }
            }
            if c < 0x80 as ::core::ffi::c_int {
                let fresh107 = len;
                len = len + 1;
                *buf.offset(fresh107 as isize) = c as xmlChar;
            } else {
                len += xmlCopyCharMultiByte(buf.offset(len as isize) as *mut xmlChar, c);
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
            } else {
                (*(*ctxt).input).col += 1;
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
            c = xmlCurrentChar(ctxt, &raw mut l);
            if c == 0 as ::core::ffi::c_int {
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                c = xmlCurrentChar(ctxt, &raw mut l);
            }
            if !(len > maxLength) {
                continue;
            }
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_ENTITY_NOT_FINISHED,
                b"entity value too long\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 10900495365614327488;
            break;
        }
        match current_block {
            10900495365614327488 => {}
            _ => {
                *buf.offset(len as isize) = 0 as xmlChar;
                if !((*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int)
                {
                    if c != stop as ::core::ffi::c_int {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_ENTITY_NOT_FINISHED,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    } else {
                        xmlNextChar(ctxt);
                        cur = buf;
                        loop {
                            if !(*cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                                current_block = 2706659501864706830;
                                break;
                            }
                            if *cur as ::core::ffi::c_int == '%' as i32
                                || *cur as ::core::ffi::c_int == '&' as i32
                                    && *cur.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        != '#' as i32
                            {
                                let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                let mut tmp_0: xmlChar = *cur;
                                let mut nameOk: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                cur = cur.offset(1);
                                name = xmlParseStringName(ctxt, &raw mut cur);
                                if !name.is_null() {
                                    nameOk = 1 as ::core::ffi::c_int;
                                    xmlFree.expect("non-null function pointer")(
                                        name as *mut ::core::ffi::c_void,
                                    );
                                }
                                if nameOk == 0 as ::core::ffi::c_int
                                    || *cur as ::core::ffi::c_int != ';' as i32
                                {
                                    xmlFatalErrMsgInt(
                                        ctxt,
                                        XML_ERR_ENTITY_CHAR_ERROR,
                                        b"EntityValue: '%c' forbidden except for entities references\n\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        tmp_0 as ::core::ffi::c_int,
                                    );
                                    current_block = 10900495365614327488;
                                    break;
                                } else if tmp_0 as ::core::ffi::c_int == '%' as i32
                                    && (*ctxt).inSubset == 1 as ::core::ffi::c_int
                                    && (*ctxt).inputNr == 1 as ::core::ffi::c_int
                                {
                                    xmlFatalErr(
                                        ctxt,
                                        XML_ERR_ENTITY_PE_INTERNAL,
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                    );
                                    current_block = 10900495365614327488;
                                    break;
                                } else if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                    current_block = 2706659501864706830;
                                    break;
                                }
                            }
                            cur = cur.offset(1);
                        }
                        match current_block {
                            10900495365614327488 => {}
                            _ => {
                                (*ctxt).depth += 1;
                                ret = xmlStringDecodeEntitiesInt(
                                    ctxt,
                                    buf,
                                    len,
                                    XML_SUBSTITUTE_PEREF,
                                    0 as xmlChar,
                                    0 as xmlChar,
                                    0 as xmlChar,
                                    1 as ::core::ffi::c_int,
                                );
                                (*ctxt).depth -= 1;
                                if !orig.is_null() {
                                    *orig = buf;
                                    buf = ::core::ptr::null_mut::<xmlChar>();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !buf.is_null() {
        xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn xmlParseAttValueComplex(
    mut ctxt: xmlParserCtxtPtr,
    mut attlen: *mut ::core::ffi::c_int,
    mut normalize: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut limit: xmlChar = 0 as xmlChar;
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut rep: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: size_t = 0 as size_t;
    let mut buf_size: size_t = 0 as size_t;
    let mut maxLength: size_t = (if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
        XML_MAX_HUGE_LENGTH
    } else {
        XML_MAX_TEXT_LENGTH
    }) as size_t;
    let mut c: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut in_space: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    if *(*(*ctxt).input)
        .cur
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '"' as i32
    {
        (*ctxt).instate = XML_PARSER_ATTRIBUTE_VALUE;
        limit = '"' as i32 as xmlChar;
        xmlNextChar(ctxt);
    } else if *(*(*ctxt).input)
        .cur
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '\'' as i32
    {
        limit = '\'' as i32 as xmlChar;
        (*ctxt).instate = XML_PARSER_ATTRIBUTE_VALUE;
        xmlNextChar(ctxt);
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_ATTRIBUTE_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buf_size = XML_PARSER_BUFFER_SIZE as size_t;
    buf = xmlMallocAtomic.expect("non-null function pointer")(buf_size) as *mut xmlChar;
    if buf.is_null() {
        current_block = 9010557660937214638;
    } else {
        c = xmlCurrentChar(ctxt, &raw mut l);
        's_79: loop {
            if !(*(*(*ctxt).input)
                .cur
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != limit as ::core::ffi::c_int
                && (if c < 0x100 as ::core::ffi::c_int {
                    (0x9 as ::core::ffi::c_int <= c && c <= 0xa as ::core::ffi::c_int
                        || c == 0xd as ::core::ffi::c_int
                        || 0x20 as ::core::ffi::c_int <= c)
                        as ::core::ffi::c_int
                } else {
                    (0x100 as ::core::ffi::c_int <= c && c <= 0xd7ff as ::core::ffi::c_int
                        || 0xe000 as ::core::ffi::c_int <= c && c <= 0xfffd as ::core::ffi::c_int
                        || 0x10000 as ::core::ffi::c_int <= c
                            && c <= 0x10ffff as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                }) != 0
                && c != '<' as i32
                && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int)
            {
                current_block = 9216188846964669005;
                break;
            }
            if c == '&' as i32 {
                in_space = 0 as ::core::ffi::c_int;
                if *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '#' as i32
                {
                    let mut val: ::core::ffi::c_int = xmlParseCharRef(ctxt);
                    if val == '&' as i32 {
                        if (*ctxt).replaceEntities != 0 {
                            if len.wrapping_add(10 as size_t) > buf_size {
                                let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                let mut new_size: size_t = buf_size
                                    .wrapping_mul(2 as size_t)
                                    .wrapping_add(10 as size_t);
                                if new_size < buf_size {
                                    current_block = 9010557660937214638;
                                    break;
                                }
                                tmp = xmlRealloc.expect("non-null function pointer")(
                                    buf as *mut ::core::ffi::c_void,
                                    new_size,
                                ) as *mut xmlChar;
                                if tmp.is_null() {
                                    current_block = 9010557660937214638;
                                    break;
                                }
                                buf = tmp;
                                buf_size = new_size;
                            }
                            let fresh32 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh32 as isize) = '&' as i32 as xmlChar;
                        } else {
                            if len.wrapping_add(10 as size_t) > buf_size {
                                let mut tmp_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                let mut new_size_0: size_t = buf_size
                                    .wrapping_mul(2 as size_t)
                                    .wrapping_add(10 as size_t);
                                if new_size_0 < buf_size {
                                    current_block = 9010557660937214638;
                                    break;
                                }
                                tmp_0 = xmlRealloc.expect("non-null function pointer")(
                                    buf as *mut ::core::ffi::c_void,
                                    new_size_0,
                                ) as *mut xmlChar;
                                if tmp_0.is_null() {
                                    current_block = 9010557660937214638;
                                    break;
                                }
                                buf = tmp_0;
                                buf_size = new_size_0;
                            }
                            let fresh33 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh33 as isize) = '&' as i32 as xmlChar;
                            let fresh34 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh34 as isize) = '#' as i32 as xmlChar;
                            let fresh35 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh35 as isize) = '3' as i32 as xmlChar;
                            let fresh36 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh36 as isize) = '8' as i32 as xmlChar;
                            let fresh37 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh37 as isize) = ';' as i32 as xmlChar;
                        }
                    } else if val != 0 as ::core::ffi::c_int {
                        if len.wrapping_add(10 as size_t) > buf_size {
                            let mut tmp_1: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                            let mut new_size_1: size_t = buf_size
                                .wrapping_mul(2 as size_t)
                                .wrapping_add(10 as size_t);
                            if new_size_1 < buf_size {
                                current_block = 9010557660937214638;
                                break;
                            }
                            tmp_1 = xmlRealloc.expect("non-null function pointer")(
                                buf as *mut ::core::ffi::c_void,
                                new_size_1,
                            ) as *mut xmlChar;
                            if tmp_1.is_null() {
                                current_block = 9010557660937214638;
                                break;
                            }
                            buf = tmp_1;
                            buf_size = new_size_1;
                        }
                        len = (len as ::core::ffi::c_ulong).wrapping_add(xmlCopyChar(
                            0 as ::core::ffi::c_int,
                            buf.offset(len as isize) as *mut xmlChar,
                            val,
                        )
                            as ::core::ffi::c_ulong) as size_t
                            as size_t;
                    }
                } else {
                    ent = xmlParseEntityRef(ctxt);
                    if !ent.is_null()
                        && (*ent).etype as ::core::ffi::c_uint
                            == XML_INTERNAL_PREDEFINED_ENTITY as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                    {
                        if len.wrapping_add(10 as size_t) > buf_size {
                            let mut tmp_2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                            let mut new_size_2: size_t = buf_size
                                .wrapping_mul(2 as size_t)
                                .wrapping_add(10 as size_t);
                            if new_size_2 < buf_size {
                                current_block = 9010557660937214638;
                                break;
                            }
                            tmp_2 = xmlRealloc.expect("non-null function pointer")(
                                buf as *mut ::core::ffi::c_void,
                                new_size_2,
                            ) as *mut xmlChar;
                            if tmp_2.is_null() {
                                current_block = 9010557660937214638;
                                break;
                            }
                            buf = tmp_2;
                            buf_size = new_size_2;
                        }
                        if (*ctxt).replaceEntities == 0 as ::core::ffi::c_int
                            && *(*ent).content.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '&' as i32
                        {
                            let fresh38 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh38 as isize) = '&' as i32 as xmlChar;
                            let fresh39 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh39 as isize) = '#' as i32 as xmlChar;
                            let fresh40 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh40 as isize) = '3' as i32 as xmlChar;
                            let fresh41 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh41 as isize) = '8' as i32 as xmlChar;
                            let fresh42 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh42 as isize) = ';' as i32 as xmlChar;
                        } else {
                            let fresh43 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh43 as isize) =
                                *(*ent).content.offset(0 as ::core::ffi::c_int as isize);
                        }
                    } else if !ent.is_null() && (*ctxt).replaceEntities != 0 as ::core::ffi::c_int {
                        if (*ent).etype as ::core::ffi::c_uint
                            != XML_INTERNAL_PREDEFINED_ENTITY as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                        {
                            if xmlParserEntityCheck(ctxt, (*ent).length as ::core::ffi::c_ulong)
                                != 0
                            {
                                current_block = 11152243663167581758;
                                break;
                            }
                            (*ctxt).depth += 1;
                            rep = xmlStringDecodeEntitiesInt(
                                ctxt,
                                (*ent).content,
                                (*ent).length,
                                XML_SUBSTITUTE_REF,
                                0 as xmlChar,
                                0 as xmlChar,
                                0 as xmlChar,
                                1 as ::core::ffi::c_int,
                            );
                            (*ctxt).depth -= 1;
                            if !rep.is_null() {
                                current = rep;
                                while *current as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                    if *current as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
                                        || *current as ::core::ffi::c_int
                                            == 0xa as ::core::ffi::c_int
                                        || *current as ::core::ffi::c_int
                                            == 0x9 as ::core::ffi::c_int
                                    {
                                        let fresh44 = len;
                                        len = len.wrapping_add(1);
                                        *buf.offset(fresh44 as isize) = 0x20 as xmlChar;
                                        current = current.offset(1);
                                    } else {
                                        let fresh45 = current;
                                        current = current.offset(1);
                                        let fresh46 = len;
                                        len = len.wrapping_add(1);
                                        *buf.offset(fresh46 as isize) = *fresh45;
                                    }
                                    if !(len.wrapping_add(10 as size_t) > buf_size) {
                                        continue;
                                    }
                                    let mut tmp_3: *mut xmlChar =
                                        ::core::ptr::null_mut::<xmlChar>();
                                    let mut new_size_3: size_t = buf_size
                                        .wrapping_mul(2 as size_t)
                                        .wrapping_add(10 as size_t);
                                    if new_size_3 < buf_size {
                                        current_block = 9010557660937214638;
                                        break 's_79;
                                    }
                                    tmp_3 = xmlRealloc.expect("non-null function pointer")(
                                        buf as *mut ::core::ffi::c_void,
                                        new_size_3,
                                    ) as *mut xmlChar;
                                    if tmp_3.is_null() {
                                        current_block = 9010557660937214638;
                                        break 's_79;
                                    }
                                    buf = tmp_3;
                                    buf_size = new_size_3;
                                }
                                xmlFree.expect("non-null function pointer")(
                                    rep as *mut ::core::ffi::c_void,
                                );
                                rep = ::core::ptr::null_mut::<xmlChar>();
                            }
                        } else {
                            if len.wrapping_add(10 as size_t) > buf_size {
                                let mut tmp_4: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                let mut new_size_4: size_t = buf_size
                                    .wrapping_mul(2 as size_t)
                                    .wrapping_add(10 as size_t);
                                if new_size_4 < buf_size {
                                    current_block = 9010557660937214638;
                                    break;
                                }
                                tmp_4 = xmlRealloc.expect("non-null function pointer")(
                                    buf as *mut ::core::ffi::c_void,
                                    new_size_4,
                                ) as *mut xmlChar;
                                if tmp_4.is_null() {
                                    current_block = 9010557660937214638;
                                    break;
                                }
                                buf = tmp_4;
                                buf_size = new_size_4;
                            }
                            if !(*ent).content.is_null() {
                                let fresh47 = len;
                                len = len.wrapping_add(1);
                                *buf.offset(fresh47 as isize) =
                                    *(*ent).content.offset(0 as ::core::ffi::c_int as isize);
                            }
                        }
                    } else if !ent.is_null() {
                        let mut i: ::core::ffi::c_int = xmlStrlen((*ent).name);
                        let mut cur: *const xmlChar = (*ent).name;
                        if (*ent).etype as ::core::ffi::c_uint
                            != XML_INTERNAL_PREDEFINED_ENTITY as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            && !(*ent).content.is_null()
                        {
                            if (*ent).flags & XML_ENT_CHECKED == 0 as ::core::ffi::c_int {
                                let mut oldCopy: ::core::ffi::c_ulong = (*ctxt).sizeentcopy;
                                (*ctxt).sizeentcopy = (*ent).length as ::core::ffi::c_ulong;
                                (*ctxt).depth += 1;
                                rep = xmlStringDecodeEntitiesInt(
                                    ctxt,
                                    (*ent).content,
                                    (*ent).length,
                                    XML_SUBSTITUTE_REF,
                                    0 as xmlChar,
                                    0 as xmlChar,
                                    0 as xmlChar,
                                    1 as ::core::ffi::c_int,
                                );
                                (*ctxt).depth -= 1;
                                if (*ctxt).inSubset == 0 as ::core::ffi::c_int {
                                    (*ent).flags |= XML_ENT_CHECKED;
                                    (*ent).expandedSize = (*ctxt).sizeentcopy;
                                }
                                if !rep.is_null() {
                                    xmlFree.expect("non-null function pointer")(
                                        rep as *mut ::core::ffi::c_void,
                                    );
                                    rep = ::core::ptr::null_mut::<xmlChar>();
                                } else {
                                    *(*ent).content.offset(0 as ::core::ffi::c_int as isize) =
                                        0 as xmlChar;
                                }
                                if xmlParserEntityCheck(ctxt, oldCopy) != 0 {
                                    current_block = 11152243663167581758;
                                    break;
                                }
                            } else if xmlParserEntityCheck(ctxt, (*ent).expandedSize) != 0 {
                                current_block = 11152243663167581758;
                                break;
                            }
                        }
                        let fresh48 = len;
                        len = len.wrapping_add(1);
                        *buf.offset(fresh48 as isize) = '&' as i32 as xmlChar;
                        while len.wrapping_add(i as size_t).wrapping_add(10 as size_t) > buf_size {
                            let mut tmp_5: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                            let mut new_size_5: size_t = buf_size
                                .wrapping_mul(2 as size_t)
                                .wrapping_add(i as size_t)
                                .wrapping_add(10 as size_t);
                            if new_size_5 < buf_size {
                                current_block = 9010557660937214638;
                                break 's_79;
                            }
                            tmp_5 = xmlRealloc.expect("non-null function pointer")(
                                buf as *mut ::core::ffi::c_void,
                                new_size_5,
                            ) as *mut xmlChar;
                            if tmp_5.is_null() {
                                current_block = 9010557660937214638;
                                break 's_79;
                            }
                            buf = tmp_5;
                            buf_size = new_size_5;
                        }
                        while i > 0 as ::core::ffi::c_int {
                            let fresh49 = cur;
                            cur = cur.offset(1);
                            let fresh50 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh50 as isize) = *fresh49;
                            i -= 1;
                        }
                        let fresh51 = len;
                        len = len.wrapping_add(1);
                        *buf.offset(fresh51 as isize) = ';' as i32 as xmlChar;
                    }
                }
            } else {
                if c == 0x20 as ::core::ffi::c_int
                    || c == 0xd as ::core::ffi::c_int
                    || c == 0xa as ::core::ffi::c_int
                    || c == 0x9 as ::core::ffi::c_int
                {
                    if len != 0 as size_t || normalize == 0 {
                        if normalize == 0 || in_space == 0 {
                            if (0x20 as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
                                let fresh52 = len;
                                len = len.wrapping_add(1);
                                *buf.offset(fresh52 as isize) = 0x20 as xmlChar;
                            } else {
                                len = (len as ::core::ffi::c_ulong).wrapping_add(
                                    xmlCopyCharMultiByte(
                                        buf.offset(len as isize) as *mut xmlChar,
                                        0x20 as ::core::ffi::c_int,
                                    ) as ::core::ffi::c_ulong,
                                ) as size_t as size_t;
                            }
                            while len.wrapping_add(10 as size_t) > buf_size {
                                let mut tmp_6: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                let mut new_size_6: size_t = buf_size
                                    .wrapping_mul(2 as size_t)
                                    .wrapping_add(10 as size_t);
                                if new_size_6 < buf_size {
                                    current_block = 9010557660937214638;
                                    break 's_79;
                                }
                                tmp_6 = xmlRealloc.expect("non-null function pointer")(
                                    buf as *mut ::core::ffi::c_void,
                                    new_size_6,
                                ) as *mut xmlChar;
                                if tmp_6.is_null() {
                                    current_block = 9010557660937214638;
                                    break 's_79;
                                }
                                buf = tmp_6;
                                buf_size = new_size_6;
                            }
                        }
                        in_space = 1 as ::core::ffi::c_int;
                    }
                } else {
                    in_space = 0 as ::core::ffi::c_int;
                    if c < 0x80 as ::core::ffi::c_int {
                        let fresh53 = len;
                        len = len.wrapping_add(1);
                        *buf.offset(fresh53 as isize) = c as xmlChar;
                    } else {
                        len = (len as ::core::ffi::c_ulong).wrapping_add(xmlCopyCharMultiByte(
                            buf.offset(len as isize) as *mut xmlChar,
                            c,
                        )
                            as ::core::ffi::c_ulong) as size_t
                            as size_t;
                    }
                    if len.wrapping_add(10 as size_t) > buf_size {
                        let mut tmp_7: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        let mut new_size_7: size_t = buf_size
                            .wrapping_mul(2 as size_t)
                            .wrapping_add(10 as size_t);
                        if new_size_7 < buf_size {
                            current_block = 9010557660937214638;
                            break;
                        }
                        tmp_7 = xmlRealloc.expect("non-null function pointer")(
                            buf as *mut ::core::ffi::c_void,
                            new_size_7,
                        ) as *mut xmlChar;
                        if tmp_7.is_null() {
                            current_block = 9010557660937214638;
                            break;
                        }
                        buf = tmp_7;
                        buf_size = new_size_7;
                    }
                }
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                } else {
                    (*(*ctxt).input).col += 1;
                }
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
            }
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
            c = xmlCurrentChar(ctxt, &raw mut l);
            if !(len > maxLength) {
                continue;
            }
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_ATTRIBUTE_NOT_FINISHED,
                b"AttValue length too long\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 9010557660937214638;
            break;
        }
        match current_block {
            11152243663167581758 => {}
            9010557660937214638 => {}
            _ => {
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    current_block = 11152243663167581758;
                } else {
                    if in_space != 0 && normalize != 0 {
                        while len > 0 as size_t
                            && *buf.offset(len.wrapping_sub(1 as size_t) as isize)
                                as ::core::ffi::c_int
                                == 0x20 as ::core::ffi::c_int
                        {
                            len = len.wrapping_sub(1);
                        }
                    }
                    *buf.offset(len as isize) = 0 as xmlChar;
                    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32 {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_LT_IN_ATTRIBUTE,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int
                        != limit as ::core::ffi::c_int
                    {
                        if c != 0 as ::core::ffi::c_int
                            && (if c < 0x100 as ::core::ffi::c_int {
                                (0x9 as ::core::ffi::c_int <= c && c <= 0xa as ::core::ffi::c_int
                                    || c == 0xd as ::core::ffi::c_int
                                    || 0x20 as ::core::ffi::c_int <= c)
                                    as ::core::ffi::c_int
                            } else {
                                (0x100 as ::core::ffi::c_int <= c
                                    && c <= 0xd7ff as ::core::ffi::c_int
                                    || 0xe000 as ::core::ffi::c_int <= c
                                        && c <= 0xfffd as ::core::ffi::c_int
                                    || 0x10000 as ::core::ffi::c_int <= c
                                        && c <= 0x10ffff as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                            }) == 0
                        {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_INVALID_CHAR,
                                b"invalid character in attribute value\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_ATTRIBUTE_NOT_FINISHED,
                                b"AttValue: ' expected\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    } else {
                        xmlNextChar(ctxt);
                    }
                    if !attlen.is_null() {
                        *attlen = len as ::core::ffi::c_int;
                    }
                    return buf;
                }
            }
        }
    }
    match current_block {
        9010557660937214638 => {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        }
        _ => {}
    }
    if !buf.is_null() {
        xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
    }
    if !rep.is_null() {
        xmlFree.expect("non-null function pointer")(rep as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseAttValue(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    if ctxt.is_null() || (*ctxt).input.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlParseAttValueInternal(
        ctxt,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseSystemLiteral(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = XML_PARSER_BUFFER_SIZE;
    let mut cur: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_TEXT_LENGTH
        } else {
            XML_MAX_NAME_LENGTH
        };
    let mut stop: xmlChar = 0;
    let mut state: ::core::ffi::c_int = (*ctxt).instate as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '"' as i32 {
        xmlNextChar(ctxt);
        stop = '"' as i32 as xmlChar;
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\'' as i32 {
        xmlNextChar(ctxt);
        stop = '\'' as i32 as xmlChar;
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_LITERAL_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buf = xmlMallocAtomic.expect("non-null function pointer")(size as size_t) as *mut xmlChar;
    if buf.is_null() {
        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        return ::core::ptr::null_mut::<xmlChar>();
    }
    (*ctxt).instate = XML_PARSER_SYSTEM_LITERAL;
    cur = xmlCurrentChar(ctxt, &raw mut l);
    while (if cur < 0x100 as ::core::ffi::c_int {
        (0x9 as ::core::ffi::c_int <= cur && cur <= 0xa as ::core::ffi::c_int
            || cur == 0xd as ::core::ffi::c_int
            || 0x20 as ::core::ffi::c_int <= cur) as ::core::ffi::c_int
    } else {
        (0x100 as ::core::ffi::c_int <= cur && cur <= 0xd7ff as ::core::ffi::c_int
            || 0xe000 as ::core::ffi::c_int <= cur && cur <= 0xfffd as ::core::ffi::c_int
            || 0x10000 as ::core::ffi::c_int <= cur && cur <= 0x10ffff as ::core::ffi::c_int)
            as ::core::ffi::c_int
    }) != 0
        && cur != stop as ::core::ffi::c_int
    {
        if len + 5 as ::core::ffi::c_int >= size {
            let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            size *= 2 as ::core::ffi::c_int;
            tmp = xmlRealloc.expect("non-null function pointer")(
                buf as *mut ::core::ffi::c_void,
                size as size_t,
            ) as *mut xmlChar;
            if tmp.is_null() {
                xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                (*ctxt).instate = state as xmlParserInputState;
                return ::core::ptr::null_mut::<xmlChar>();
            }
            buf = tmp;
        }
        if cur < 0x80 as ::core::ffi::c_int {
            let fresh104 = len;
            len = len + 1;
            *buf.offset(fresh104 as isize) = cur as xmlChar;
        } else {
            len += xmlCopyCharMultiByte(buf.offset(len as isize) as *mut xmlChar, cur);
        }
        if len > maxLength {
            xmlFatalErr(
                ctxt,
                XML_ERR_NAME_TOO_LONG,
                b"SystemLiteral\0" as *const u8 as *const ::core::ffi::c_char,
            );
            xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
            (*ctxt).instate = state as xmlParserInputState;
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
        cur = xmlCurrentChar(ctxt, &raw mut l);
    }
    *buf.offset(len as isize) = 0 as xmlChar;
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    (*ctxt).instate = state as xmlParserInputState;
    if if cur < 0x100 as ::core::ffi::c_int {
        (0x9 as ::core::ffi::c_int <= cur && cur <= 0xa as ::core::ffi::c_int
            || cur == 0xd as ::core::ffi::c_int
            || 0x20 as ::core::ffi::c_int <= cur) as ::core::ffi::c_int
    } else {
        (0x100 as ::core::ffi::c_int <= cur && cur <= 0xd7ff as ::core::ffi::c_int
            || 0xe000 as ::core::ffi::c_int <= cur && cur <= 0xfffd as ::core::ffi::c_int
            || 0x10000 as ::core::ffi::c_int <= cur && cur <= 0x10ffff as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } == 0
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_LITERAL_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else {
        xmlNextChar(ctxt);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParsePubidLiteral(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = XML_PARSER_BUFFER_SIZE;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_TEXT_LENGTH
        } else {
            XML_MAX_NAME_LENGTH
        };
    let mut cur: xmlChar = 0;
    let mut stop: xmlChar = 0;
    let mut oldstate: xmlParserInputState = (*ctxt).instate;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '"' as i32 {
        xmlNextChar(ctxt);
        stop = '"' as i32 as xmlChar;
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\'' as i32 {
        xmlNextChar(ctxt);
        stop = '\'' as i32 as xmlChar;
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_LITERAL_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buf = xmlMallocAtomic.expect("non-null function pointer")(size as size_t) as *mut xmlChar;
    if buf.is_null() {
        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        return ::core::ptr::null_mut::<xmlChar>();
    }
    (*ctxt).instate = XML_PARSER_PUBLIC_LITERAL;
    cur = *(*(*ctxt).input).cur;
    while xmlIsPubidChar_tab[cur as usize] as ::core::ffi::c_int != 0
        && cur as ::core::ffi::c_int != stop as ::core::ffi::c_int
    {
        if len + 1 as ::core::ffi::c_int >= size {
            let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            size *= 2 as ::core::ffi::c_int;
            tmp = xmlRealloc.expect("non-null function pointer")(
                buf as *mut ::core::ffi::c_void,
                size as size_t,
            ) as *mut xmlChar;
            if tmp.is_null() {
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            buf = tmp;
        }
        let fresh105 = len;
        len = len + 1;
        *buf.offset(fresh105 as isize) = cur;
        if len > maxLength {
            xmlFatalErr(
                ctxt,
                XML_ERR_NAME_TOO_LONG,
                b"Public ID\0" as *const u8 as *const ::core::ffi::c_char,
            );
            xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xmlChar>();
        }
        xmlNextChar(ctxt);
        cur = *(*(*ctxt).input).cur;
    }
    *buf.offset(len as isize) = 0 as xmlChar;
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if cur as ::core::ffi::c_int != stop as ::core::ffi::c_int {
        xmlFatalErr(
            ctxt,
            XML_ERR_LITERAL_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else {
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize);
    }
    (*ctxt).instate = oldstate;
    return buf;
}
static mut test_char_data: [::core::ffi::c_uchar; 256] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x29 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2d as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x30 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x31 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x32 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x33 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x34 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x35 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x36 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x37 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x38 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x39 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3d as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x40 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x42 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x43 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x44 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x45 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x46 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x47 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x48 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x49 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4d as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x50 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x51 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x52 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x54 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x55 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x57 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x58 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x59 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x60 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x61 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x62 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x63 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x65 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x66 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x67 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x68 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x69 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6d as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x6f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x70 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x71 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x72 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x73 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x74 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x76 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x77 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x78 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x79 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7b as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7c as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7d as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
unsafe extern "C" fn xmlParseCharDataInternal(
    mut ctxt: xmlParserCtxtPtr,
    mut partial: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut nbchar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut line: ::core::ffi::c_int = (*(*ctxt).input).line;
    let mut col: ::core::ffi::c_int = (*(*ctxt).input).col;
    let mut ccol: ::core::ffi::c_int = 0;
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    in_0 = (*(*ctxt).input).cur;
    loop {
        while *in_0 as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int {
            in_0 = in_0.offset(1);
            (*(*ctxt).input).col += 1;
        }
        if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
            loop {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                in_0 = in_0.offset(1);
                if !(*in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int) {
                    break;
                }
            }
        } else {
            if *in_0 as ::core::ffi::c_int == '<' as i32 {
                nbchar = in_0.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                if nbchar > 0 as ::core::ffi::c_int {
                    let mut tmp: *const xmlChar = (*(*ctxt).input).cur;
                    (*(*ctxt).input).cur = in_0;
                    if !(*ctxt).sax.is_null()
                        && (*ctxt).disableSAX == 0 as ::core::ffi::c_int
                        && (*(*ctxt).sax).ignorableWhitespace != (*(*ctxt).sax).characters
                    {
                        if areBlanks(ctxt, tmp, nbchar, 1 as ::core::ffi::c_int) != 0 {
                            if (*(*ctxt).sax).ignorableWhitespace.is_some() {
                                (*(*ctxt).sax)
                                    .ignorableWhitespace
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    tmp,
                                    nbchar,
                                );
                            }
                        } else {
                            if (*(*ctxt).sax).characters.is_some() {
                                (*(*ctxt).sax)
                                    .characters
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    tmp,
                                    nbchar,
                                );
                            }
                            if *(*ctxt).space == -(1 as ::core::ffi::c_int) {
                                *(*ctxt).space = -(2 as ::core::ffi::c_int);
                            }
                        }
                    } else if !(*ctxt).sax.is_null()
                        && (*ctxt).disableSAX == 0 as ::core::ffi::c_int
                        && (*(*ctxt).sax).characters.is_some()
                    {
                        (*(*ctxt).sax)
                            .characters
                            .expect("non-null function pointer")(
                            (*ctxt).userData, tmp, nbchar
                        );
                    }
                }
                return;
            }
            loop {
                ccol = (*(*ctxt).input).col;
                while test_char_data[*in_0 as usize] != 0 {
                    in_0 = in_0.offset(1);
                    ccol += 1;
                }
                (*(*ctxt).input).col = ccol;
                if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                    loop {
                        (*(*ctxt).input).line += 1;
                        (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                        in_0 = in_0.offset(1);
                        if !(*in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int) {
                            break;
                        }
                    }
                } else {
                    if !(*in_0 as ::core::ffi::c_int == ']' as i32) {
                        break;
                    }
                    if *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ']' as i32
                        && *in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '>' as i32
                    {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_MISPLACED_CDATA_END,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                        if (*ctxt).instate as ::core::ffi::c_int
                            != XML_PARSER_EOF as ::core::ffi::c_int
                        {
                            (*(*ctxt).input).cur = in_0.offset(1 as ::core::ffi::c_int as isize);
                        }
                        return;
                    }
                    in_0 = in_0.offset(1);
                    (*(*ctxt).input).col += 1;
                }
            }
            nbchar =
                in_0.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as ::core::ffi::c_int;
            if nbchar > 0 as ::core::ffi::c_int {
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0 as ::core::ffi::c_int
                    && (*(*ctxt).sax).ignorableWhitespace != (*(*ctxt).sax).characters
                    && (*(*(*ctxt).input).cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                        || 0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
                            && *(*(*ctxt).input).cur as ::core::ffi::c_int
                                <= 0xa as ::core::ffi::c_int
                        || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
                {
                    let mut tmp_0: *const xmlChar = (*(*ctxt).input).cur;
                    (*(*ctxt).input).cur = in_0;
                    if areBlanks(ctxt, tmp_0, nbchar, 0 as ::core::ffi::c_int) != 0 {
                        if (*(*ctxt).sax).ignorableWhitespace.is_some() {
                            (*(*ctxt).sax)
                                .ignorableWhitespace
                                .expect("non-null function pointer")(
                                (*ctxt).userData,
                                tmp_0,
                                nbchar,
                            );
                        }
                    } else {
                        if (*(*ctxt).sax).characters.is_some() {
                            (*(*ctxt).sax)
                                .characters
                                .expect("non-null function pointer")(
                                (*ctxt).userData,
                                tmp_0,
                                nbchar,
                            );
                        }
                        if *(*ctxt).space == -(1 as ::core::ffi::c_int) {
                            *(*ctxt).space = -(2 as ::core::ffi::c_int);
                        }
                    }
                    line = (*(*ctxt).input).line;
                    col = (*(*ctxt).input).col;
                } else if !(*ctxt).sax.is_null() && (*ctxt).disableSAX == 0 as ::core::ffi::c_int {
                    if (*(*ctxt).sax).characters.is_some() {
                        (*(*ctxt).sax)
                            .characters
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            (*(*ctxt).input).cur,
                            nbchar,
                        );
                    }
                    line = (*(*ctxt).input).line;
                    col = (*(*ctxt).input).col;
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    return;
                }
            }
            (*(*ctxt).input).cur = in_0;
            if *in_0 as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
                in_0 = in_0.offset(1);
                if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                    (*(*ctxt).input).cur = in_0;
                    in_0 = in_0.offset(1);
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                    current_block = 820271813250567934;
                } else {
                    in_0 = in_0.offset(-1);
                    current_block = 17020603795727957434;
                }
            } else {
                current_block = 17020603795727957434;
            }
            match current_block {
                17020603795727957434 => {
                    if *in_0 as ::core::ffi::c_int == '<' as i32 {
                        return;
                    }
                    if *in_0 as ::core::ffi::c_int == '&' as i32 {
                        return;
                    }
                    if ((*ctxt).progressive == 0 as ::core::ffi::c_int
                        || (*ctxt).inputNr > 1 as ::core::ffi::c_int)
                        && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                            as ::core::ffi::c_long
                            > (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
                        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                            as ::core::ffi::c_long)
                            < (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
                    {
                        xmlParserShrink(ctxt);
                    }
                    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                        as ::core::ffi::c_long)
                        < INPUT_CHUNK as ::core::ffi::c_long
                    {
                        xmlParserGrow(ctxt);
                    }
                    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
                    {
                        return;
                    }
                    in_0 = (*(*ctxt).input).cur;
                }
                _ => {}
            }
            if !(*in_0 as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                && *in_0 as ::core::ffi::c_int <= 0x7f as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0x9 as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int)
            {
                break;
            }
        }
    }
    (*(*ctxt).input).line = line;
    (*(*ctxt).input).col = col;
    xmlParseCharDataComplex(ctxt, partial);
}
unsafe extern "C" fn xmlParseCharDataComplex(
    mut ctxt: xmlParserCtxtPtr,
    mut partial: ::core::ffi::c_int,
) {
    let mut buf: [xmlChar; 305] = [0; 305];
    let mut nbchar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cur: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    cur = xmlCurrentChar(ctxt, &raw mut l);
    while cur != '<' as i32
        && cur != '&' as i32
        && (if cur < 0x100 as ::core::ffi::c_int {
            (0x9 as ::core::ffi::c_int <= cur && cur <= 0xa as ::core::ffi::c_int
                || cur == 0xd as ::core::ffi::c_int
                || 0x20 as ::core::ffi::c_int <= cur) as ::core::ffi::c_int
        } else {
            (0x100 as ::core::ffi::c_int <= cur && cur <= 0xd7ff as ::core::ffi::c_int
                || 0xe000 as ::core::ffi::c_int <= cur && cur <= 0xfffd as ::core::ffi::c_int
                || 0x10000 as ::core::ffi::c_int <= cur && cur <= 0x10ffff as ::core::ffi::c_int)
                as ::core::ffi::c_int
        }) != 0
    {
        if cur == ']' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ']' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '>' as i32
        {
            xmlFatalErr(
                ctxt,
                XML_ERR_MISPLACED_CDATA_END,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if cur < 0x80 as ::core::ffi::c_int {
            let fresh13 = nbchar;
            nbchar = nbchar + 1;
            buf[fresh13 as usize] = cur as xmlChar;
        } else {
            nbchar += xmlCopyCharMultiByte(
                (&raw mut buf as *mut xmlChar).offset(nbchar as isize) as *mut xmlChar,
                cur,
            );
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
        if nbchar >= XML_PARSER_BIG_BUFFER_SIZE {
            buf[nbchar as usize] = 0 as xmlChar;
            if !(*ctxt).sax.is_null() && (*ctxt).disableSAX == 0 {
                if areBlanks(
                    ctxt,
                    &raw mut buf as *mut xmlChar,
                    nbchar,
                    0 as ::core::ffi::c_int,
                ) != 0
                {
                    if (*(*ctxt).sax).ignorableWhitespace.is_some() {
                        (*(*ctxt).sax)
                            .ignorableWhitespace
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            &raw mut buf as *mut xmlChar,
                            nbchar,
                        );
                    }
                } else {
                    if (*(*ctxt).sax).characters.is_some() {
                        (*(*ctxt).sax)
                            .characters
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            &raw mut buf as *mut xmlChar,
                            nbchar,
                        );
                    }
                    if (*(*ctxt).sax).characters != (*(*ctxt).sax).ignorableWhitespace
                        && *(*ctxt).space == -(1 as ::core::ffi::c_int)
                    {
                        *(*ctxt).space = -(2 as ::core::ffi::c_int);
                    }
                }
            }
            nbchar = 0 as ::core::ffi::c_int;
            if (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_CONTENT as ::core::ffi::c_int {
                return;
            }
            if ((*ctxt).progressive == 0 as ::core::ffi::c_int
                || (*ctxt).inputNr > 1 as ::core::ffi::c_int)
                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                    > (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
            {
                xmlParserShrink(ctxt);
            }
        }
        cur = xmlCurrentChar(ctxt, &raw mut l);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return;
    }
    if nbchar != 0 as ::core::ffi::c_int {
        buf[nbchar as usize] = 0 as xmlChar;
        if !(*ctxt).sax.is_null() && (*ctxt).disableSAX == 0 {
            if areBlanks(
                ctxt,
                &raw mut buf as *mut xmlChar,
                nbchar,
                0 as ::core::ffi::c_int,
            ) != 0
            {
                if (*(*ctxt).sax).ignorableWhitespace.is_some() {
                    (*(*ctxt).sax)
                        .ignorableWhitespace
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        &raw mut buf as *mut xmlChar,
                        nbchar,
                    );
                }
            } else {
                if (*(*ctxt).sax).characters.is_some() {
                    (*(*ctxt).sax)
                        .characters
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        &raw mut buf as *mut xmlChar,
                        nbchar,
                    );
                }
                if (*(*ctxt).sax).characters != (*(*ctxt).sax).ignorableWhitespace
                    && *(*ctxt).space == -(1 as ::core::ffi::c_int)
                {
                    *(*ctxt).space = -(2 as ::core::ffi::c_int);
                }
            }
        }
    }
    if (*(*ctxt).input).cur < (*(*ctxt).input).end {
        if cur == 0 as ::core::ffi::c_int
            && *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            if partial == 0 as ::core::ffi::c_int {
                xmlFatalErrMsgInt(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Incomplete UTF-8 sequence starting with %02X\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *(*(*ctxt).input).cur as ::core::ffi::c_int,
                );
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                } else {
                    (*(*ctxt).input).col += 1;
                }
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize);
            }
        } else if cur != '<' as i32 && cur != '&' as i32 {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"PCDATA invalid Char value %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                cur,
            );
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
            } else {
                (*(*ctxt).input).col += 1;
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseCharData(
    mut ctxt: xmlParserCtxtPtr,
    mut cdata: ::core::ffi::c_int,
) {
    xmlParseCharDataInternal(ctxt, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseExternalID(
    mut ctxt: xmlParserCtxtPtr,
    mut publicID: *mut *mut xmlChar,
    mut strict: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut URI: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    *publicID = ::core::ptr::null_mut::<xmlChar>();
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'S' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'Y' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'S' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'M' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(6 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 6 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after 'SYSTEM'\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        URI = xmlParseSystemLiteral(ctxt);
        if URI.is_null() {
            xmlFatalErr(
                ctxt,
                XML_ERR_URI_REQUIRED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'P' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'U' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'B' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'L' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'C' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(6 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 6 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after 'PUBLIC'\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        *publicID = xmlParsePubidLiteral(ctxt);
        if (*publicID).is_null() {
            xmlFatalErr(
                ctxt,
                XML_ERR_PUBID_REQUIRED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if strict != 0 {
            if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after the Public Identifier\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        } else {
            if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                return ::core::ptr::null_mut::<xmlChar>();
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int != '\'' as i32
                && *(*(*ctxt).input).cur as ::core::ffi::c_int != '"' as i32
            {
                return ::core::ptr::null_mut::<xmlChar>();
            }
        }
        URI = xmlParseSystemLiteral(ctxt);
        if URI.is_null() {
            xmlFatalErr(
                ctxt,
                XML_ERR_URI_REQUIRED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return URI;
}
unsafe extern "C" fn xmlParseCommentComplex(
    mut ctxt: xmlParserCtxtPtr,
    mut buf: *mut xmlChar,
    mut len: size_t,
    mut size: size_t,
) {
    let mut q: ::core::ffi::c_int = 0;
    let mut ql: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut rl: ::core::ffi::c_int = 0;
    let mut cur: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut maxLength: size_t = (if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
        XML_MAX_HUGE_LENGTH
    } else {
        XML_MAX_TEXT_LENGTH
    }) as size_t;
    let mut inputid: ::core::ffi::c_int = 0;
    inputid = (*(*ctxt).input).id;
    if buf.is_null() {
        len = 0 as size_t;
        size = XML_PARSER_BUFFER_SIZE as size_t;
        buf = xmlMallocAtomic.expect("non-null function pointer")(size) as *mut xmlChar;
        if buf.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return;
        }
    }
    q = xmlCurrentChar(ctxt, &raw mut ql);
    if !(q == 0 as ::core::ffi::c_int) {
        if if q < 0x100 as ::core::ffi::c_int {
            (0x9 as ::core::ffi::c_int <= q && q <= 0xa as ::core::ffi::c_int
                || q == 0xd as ::core::ffi::c_int
                || 0x20 as ::core::ffi::c_int <= q) as ::core::ffi::c_int
        } else {
            (0x100 as ::core::ffi::c_int <= q && q <= 0xd7ff as ::core::ffi::c_int
                || 0xe000 as ::core::ffi::c_int <= q && q <= 0xfffd as ::core::ffi::c_int
                || 0x10000 as ::core::ffi::c_int <= q && q <= 0x10ffff as ::core::ffi::c_int)
                as ::core::ffi::c_int
        } == 0
        {
            xmlFatalErrMsgInt(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"xmlParseComment: invalid xmlChar value %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                q,
            );
            xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
            return;
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(ql as isize);
        r = xmlCurrentChar(ctxt, &raw mut rl);
        if !(r == 0 as ::core::ffi::c_int) {
            if if r < 0x100 as ::core::ffi::c_int {
                (0x9 as ::core::ffi::c_int <= r && r <= 0xa as ::core::ffi::c_int
                    || r == 0xd as ::core::ffi::c_int
                    || 0x20 as ::core::ffi::c_int <= r) as ::core::ffi::c_int
            } else {
                (0x100 as ::core::ffi::c_int <= r && r <= 0xd7ff as ::core::ffi::c_int
                    || 0xe000 as ::core::ffi::c_int <= r && r <= 0xfffd as ::core::ffi::c_int
                    || 0x10000 as ::core::ffi::c_int <= r && r <= 0x10ffff as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } == 0
            {
                xmlFatalErrMsgInt(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"xmlParseComment: invalid xmlChar value %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    r,
                );
                xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                return;
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
            } else {
                (*(*ctxt).input).col += 1;
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(rl as isize);
            cur = xmlCurrentChar(ctxt, &raw mut l);
            if !(cur == 0 as ::core::ffi::c_int) {
                while (if cur < 0x100 as ::core::ffi::c_int {
                    (0x9 as ::core::ffi::c_int <= cur && cur <= 0xa as ::core::ffi::c_int
                        || cur == 0xd as ::core::ffi::c_int
                        || 0x20 as ::core::ffi::c_int <= cur)
                        as ::core::ffi::c_int
                } else {
                    (0x100 as ::core::ffi::c_int <= cur && cur <= 0xd7ff as ::core::ffi::c_int
                        || 0xe000 as ::core::ffi::c_int <= cur
                            && cur <= 0xfffd as ::core::ffi::c_int
                        || 0x10000 as ::core::ffi::c_int <= cur
                            && cur <= 0x10ffff as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                }) != 0
                    && (cur != '>' as i32 || r != '-' as i32 || q != '-' as i32)
                {
                    if r == '-' as i32 && q == '-' as i32 {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_HYPHEN_IN_COMMENT,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                    if len.wrapping_add(5 as size_t) >= size {
                        let mut new_buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        let mut new_size: size_t = 0;
                        new_size = size.wrapping_mul(2 as size_t);
                        new_buf = xmlRealloc.expect("non-null function pointer")(
                            buf as *mut ::core::ffi::c_void,
                            new_size,
                        ) as *mut xmlChar;
                        if new_buf.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                buf as *mut ::core::ffi::c_void,
                            );
                            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                            return;
                        }
                        buf = new_buf;
                        size = new_size;
                    }
                    if q < 0x80 as ::core::ffi::c_int {
                        let fresh0 = len;
                        len = len.wrapping_add(1);
                        *buf.offset(fresh0 as isize) = q as xmlChar;
                    } else {
                        len = (len as ::core::ffi::c_ulong).wrapping_add(xmlCopyCharMultiByte(
                            buf.offset(len as isize) as *mut xmlChar,
                            q,
                        )
                            as ::core::ffi::c_ulong) as size_t
                            as size_t;
                    }
                    if len > maxLength {
                        xmlFatalErrMsgStr(
                            ctxt,
                            XML_ERR_COMMENT_NOT_FINISHED,
                            b"Comment too big found\0" as *const u8 as *const ::core::ffi::c_char,
                            ::core::ptr::null::<xmlChar>(),
                        );
                        xmlFree.expect("non-null function pointer")(
                            buf as *mut ::core::ffi::c_void,
                        );
                        return;
                    }
                    q = r;
                    ql = rl;
                    r = cur;
                    rl = l;
                    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                        (*(*ctxt).input).line += 1;
                        (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                    } else {
                        (*(*ctxt).input).col += 1;
                    }
                    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                    cur = xmlCurrentChar(ctxt, &raw mut l);
                }
                *buf.offset(len as isize) = 0 as xmlChar;
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                    return;
                }
                if cur == 0 as ::core::ffi::c_int {
                    xmlFatalErrMsgStr(
                        ctxt,
                        XML_ERR_COMMENT_NOT_FINISHED,
                        b"Comment not terminated \n<!--%.50s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        buf,
                    );
                } else if if cur < 0x100 as ::core::ffi::c_int {
                    (0x9 as ::core::ffi::c_int <= cur && cur <= 0xa as ::core::ffi::c_int
                        || cur == 0xd as ::core::ffi::c_int
                        || 0x20 as ::core::ffi::c_int <= cur)
                        as ::core::ffi::c_int
                } else {
                    (0x100 as ::core::ffi::c_int <= cur && cur <= 0xd7ff as ::core::ffi::c_int
                        || 0xe000 as ::core::ffi::c_int <= cur
                            && cur <= 0xfffd as ::core::ffi::c_int
                        || 0x10000 as ::core::ffi::c_int <= cur
                            && cur <= 0x10ffff as ::core::ffi::c_int)
                        as ::core::ffi::c_int
                } == 0
                {
                    xmlFatalErrMsgInt(
                        ctxt,
                        XML_ERR_INVALID_CHAR,
                        b"xmlParseComment: invalid xmlChar value %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        cur,
                    );
                } else {
                    if inputid != (*(*ctxt).input).id {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ENTITY_BOUNDARY,
                            b"Comment doesn't start and stop in the same entity\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    xmlNextChar(ctxt);
                    if !(*ctxt).sax.is_null()
                        && (*(*ctxt).sax).comment.is_some()
                        && (*ctxt).disableSAX == 0
                    {
                        (*(*ctxt).sax).comment.expect("non-null function pointer")(
                            (*ctxt).userData,
                            buf,
                        );
                    }
                }
                xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                return;
            }
        }
    }
    xmlFatalErrMsgStr(
        ctxt,
        XML_ERR_COMMENT_NOT_FINISHED,
        b"Comment not terminated\n\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<xmlChar>(),
    );
    xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseComment(mut ctxt: xmlParserCtxtPtr) {
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut size: size_t = XML_PARSER_BUFFER_SIZE as size_t;
    let mut len: size_t = 0 as size_t;
    let mut maxLength: size_t = (if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
        XML_MAX_HUGE_LENGTH
    } else {
        XML_MAX_TEXT_LENGTH
    }) as size_t;
    let mut state: xmlParserInputState = XML_PARSER_START;
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut nbchar: size_t = 0 as size_t;
    let mut ccol: ::core::ffi::c_int = 0;
    let mut inputid: ::core::ffi::c_int = 0;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '!' as i32
    {
        return;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(2 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '-' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '-' as i32
    {
        return;
    }
    state = (*ctxt).instate;
    (*ctxt).instate = XML_PARSER_COMMENT;
    inputid = (*(*ctxt).input).id;
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(2 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    in_0 = (*(*ctxt).input).cur;
    loop {
        if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
            loop {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                in_0 = in_0.offset(1);
                if !(*in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int) {
                    break;
                }
            }
        }
        loop {
            ccol = (*(*ctxt).input).col;
            while *in_0 as ::core::ffi::c_int > '-' as i32
                && *in_0 as ::core::ffi::c_int <= 0x7f as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && (*in_0 as ::core::ffi::c_int) < '-' as i32
                || *in_0 as ::core::ffi::c_int == 0x9 as ::core::ffi::c_int
            {
                in_0 = in_0.offset(1);
                ccol += 1;
            }
            (*(*ctxt).input).col = ccol;
            if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                loop {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                    in_0 = in_0.offset(1);
                    if !(*in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int) {
                        break;
                    }
                }
            } else {
                nbchar = in_0.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
                if nbchar > 0 as size_t {
                    if buf.is_null() {
                        if *in_0 as ::core::ffi::c_int == '-' as i32
                            && *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == '-' as i32
                        {
                            size = nbchar.wrapping_add(1 as size_t);
                        } else {
                            size = (XML_PARSER_BUFFER_SIZE as size_t).wrapping_add(nbchar);
                        }
                        buf = xmlMallocAtomic.expect("non-null function pointer")(size)
                            as *mut xmlChar;
                        if buf.is_null() {
                            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                            (*ctxt).instate = state;
                            return;
                        }
                        len = 0 as size_t;
                    } else if len.wrapping_add(nbchar).wrapping_add(1 as size_t) >= size {
                        let mut new_buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        size = (size as ::core::ffi::c_ulong).wrapping_add(
                            len.wrapping_add(nbchar)
                                .wrapping_add(XML_PARSER_BUFFER_SIZE as size_t)
                                as ::core::ffi::c_ulong,
                        ) as size_t as size_t;
                        new_buf = xmlRealloc.expect("non-null function pointer")(
                            buf as *mut ::core::ffi::c_void,
                            size,
                        ) as *mut xmlChar;
                        if new_buf.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                buf as *mut ::core::ffi::c_void,
                            );
                            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                            (*ctxt).instate = state;
                            return;
                        }
                        buf = new_buf;
                    }
                    memcpy(
                        buf.offset(len as isize) as *mut xmlChar as *mut ::core::ffi::c_void,
                        (*(*ctxt).input).cur as *const ::core::ffi::c_void,
                        nbchar,
                    );
                    len = (len as ::core::ffi::c_ulong).wrapping_add(nbchar as ::core::ffi::c_ulong)
                        as size_t as size_t;
                    *buf.offset(len as isize) = 0 as xmlChar;
                }
                if len > maxLength {
                    xmlFatalErrMsgStr(
                        ctxt,
                        XML_ERR_COMMENT_NOT_FINISHED,
                        b"Comment too big found\0" as *const u8 as *const ::core::ffi::c_char,
                        ::core::ptr::null::<xmlChar>(),
                    );
                    xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                    return;
                }
                (*(*ctxt).input).cur = in_0;
                if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                    in_0 = in_0.offset(1);
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                }
                if *in_0 as ::core::ffi::c_int == 0xd as ::core::ffi::c_int {
                    in_0 = in_0.offset(1);
                    if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                        (*(*ctxt).input).cur = in_0;
                        in_0 = in_0.offset(1);
                        (*(*ctxt).input).line += 1;
                        (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                        continue;
                    } else {
                        in_0 = in_0.offset(-1);
                    }
                }
                if ((*ctxt).progressive == 0 as ::core::ffi::c_int
                    || (*ctxt).inputNr > 1 as ::core::ffi::c_int)
                    && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                        as ::core::ffi::c_long
                        > (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
                    && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                        as ::core::ffi::c_long)
                        < (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
                {
                    xmlParserShrink(ctxt);
                }
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                    return;
                }
                in_0 = (*(*ctxt).input).cur;
                if !(*in_0 as ::core::ffi::c_int == '-' as i32) {
                    break;
                }
                if *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '-' as i32
                {
                    if *in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '>' as i32
                    {
                        if (*(*ctxt).input).id != inputid {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_ENTITY_BOUNDARY,
                                b"comment doesn't start and stop in the same entity\n\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        (*(*ctxt).input).cur = (*(*ctxt).input)
                            .cur
                            .offset(3 as ::core::ffi::c_int as isize);
                        (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
                        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            xmlParserGrow(ctxt);
                        }
                        if !(*ctxt).sax.is_null()
                            && (*(*ctxt).sax).comment.is_some()
                            && (*ctxt).disableSAX == 0
                        {
                            if !buf.is_null() {
                                (*(*ctxt).sax).comment.expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    buf,
                                );
                            } else {
                                (*(*ctxt).sax).comment.expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    b"\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut xmlChar,
                                );
                            }
                        }
                        if !buf.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                buf as *mut ::core::ffi::c_void,
                            );
                        }
                        if (*ctxt).instate as ::core::ffi::c_int
                            != XML_PARSER_EOF as ::core::ffi::c_int
                        {
                            (*ctxt).instate = state;
                        }
                        return;
                    }
                    if !buf.is_null() {
                        xmlFatalErrMsgStr(
                            ctxt,
                            XML_ERR_HYPHEN_IN_COMMENT,
                            b"Double hyphen within comment: <!--%.50s\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            buf,
                        );
                    } else {
                        xmlFatalErrMsgStr(
                            ctxt,
                            XML_ERR_HYPHEN_IN_COMMENT,
                            b"Double hyphen within comment\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null::<xmlChar>(),
                        );
                    }
                    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
                    {
                        xmlFree.expect("non-null function pointer")(
                            buf as *mut ::core::ffi::c_void,
                        );
                        return;
                    }
                    in_0 = in_0.offset(1);
                    (*(*ctxt).input).col += 1;
                }
                in_0 = in_0.offset(1);
                (*(*ctxt).input).col += 1;
            }
        }
        if !(*in_0 as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int <= 0x7f as ::core::ffi::c_int
            || *in_0 as ::core::ffi::c_int == 0x9 as ::core::ffi::c_int
            || *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int)
        {
            break;
        }
    }
    xmlParseCommentComplex(ctxt, buf, len, size);
    (*ctxt).instate = state;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParsePITarget(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    name = xmlParseName(ctxt);
    if !name.is_null()
        && (*name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'x' as i32
            || *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'X' as i32)
        && (*name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'm' as i32
            || *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'M' as i32)
        && (*name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'l' as i32
            || *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'L' as i32)
    {
        let mut i: ::core::ffi::c_int = 0;
        if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'x' as i32
            && *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'm' as i32
            && *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'l' as i32
            && *name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_RESERVED_XML_NAME,
                b"XML declaration allowed only at the start of the document\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return name;
        } else if *name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            xmlFatalErr(
                ctxt,
                XML_ERR_RESERVED_XML_NAME,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return name;
        }
        i = 0 as ::core::ffi::c_int;
        while !xmlW3CPIs[i as usize].is_null() {
            if xmlStrEqual(name, xmlW3CPIs[i as usize] as *const xmlChar) != 0 {
                return name;
            }
            i += 1;
        }
        xmlWarningMsg(
            ctxt,
            XML_ERR_RESERVED_XML_NAME,
            b"xmlParsePITarget: invalid name prefix 'xml'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
    }
    if !name.is_null() && !xmlStrchr(name, ':' as i32 as xmlChar).is_null() {
        xmlNsErr(
            ctxt,
            XML_NS_ERR_COLON,
            b"colons are forbidden from PI names '%s'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
    }
    return name;
}
unsafe extern "C" fn xmlParseCatalogPI(mut ctxt: xmlParserCtxtPtr, mut catalog: *const xmlChar) {
    let mut URL: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut tmp: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut base: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut marker: xmlChar = 0;
    tmp = catalog;
    while *tmp as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
        || 0x9 as ::core::ffi::c_int <= *tmp as ::core::ffi::c_int
            && *tmp as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
        || *tmp as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
    {
        tmp = tmp.offset(1);
    }
    if !(xmlStrncmp(
        tmp,
        b"catalog\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        7 as ::core::ffi::c_int,
    ) != 0)
    {
        tmp = tmp.offset(7 as ::core::ffi::c_int as isize);
        while *tmp as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *tmp as ::core::ffi::c_int
                && *tmp as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *tmp as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
        {
            tmp = tmp.offset(1);
        }
        if *tmp as ::core::ffi::c_int != '=' as i32 {
            return;
        }
        tmp = tmp.offset(1);
        while *tmp as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *tmp as ::core::ffi::c_int
                && *tmp as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *tmp as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
        {
            tmp = tmp.offset(1);
        }
        marker = *tmp;
        if !(marker as ::core::ffi::c_int != '\'' as i32
            && marker as ::core::ffi::c_int != '"' as i32)
        {
            tmp = tmp.offset(1);
            base = tmp;
            while *tmp as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && *tmp as ::core::ffi::c_int != marker as ::core::ffi::c_int
            {
                tmp = tmp.offset(1);
            }
            if !(*tmp as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                URL = xmlStrndup(
                    base,
                    tmp.offset_from(base) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
                tmp = tmp.offset(1);
                while *tmp as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                    || 0x9 as ::core::ffi::c_int <= *tmp as ::core::ffi::c_int
                        && *tmp as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                    || *tmp as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
                {
                    tmp = tmp.offset(1);
                }
                if !(*tmp as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
                    if !URL.is_null() {
                        (*ctxt).catalogs = xmlCatalogAddLocal((*ctxt).catalogs, URL);
                        xmlFree.expect("non-null function pointer")(
                            URL as *mut ::core::ffi::c_void,
                        );
                    }
                    return;
                }
            }
        }
    }
    xmlWarningMsg(
        ctxt,
        XML_WAR_CATALOG_PI,
        b"Catalog PI syntax error: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        catalog,
        ::core::ptr::null::<xmlChar>(),
    );
    if !URL.is_null() {
        xmlFree.expect("non-null function pointer")(URL as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParsePI(mut ctxt: xmlParserCtxtPtr) {
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: size_t = 0 as size_t;
    let mut size: size_t = XML_PARSER_BUFFER_SIZE as size_t;
    let mut maxLength: size_t = (if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
        XML_MAX_HUGE_LENGTH
    } else {
        XML_MAX_TEXT_LENGTH
    }) as size_t;
    let mut cur: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut target: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut state: xmlParserInputState = XML_PARSER_START;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '?' as i32
    {
        let mut inputid: ::core::ffi::c_int = (*(*ctxt).input).id;
        state = (*ctxt).instate;
        (*ctxt).instate = XML_PARSER_PI;
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(2 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        target = xmlParsePITarget(ctxt);
        if !target.is_null() {
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '?' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '>' as i32
            {
                if inputid != (*(*ctxt).input).id {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ENTITY_BOUNDARY,
                        b"PI declaration doesn't start and stop in the same entity\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(2 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0
                    && (*(*ctxt).sax).processingInstruction.is_some()
                {
                    (*(*ctxt).sax)
                        .processingInstruction
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        target,
                        ::core::ptr::null::<xmlChar>(),
                    );
                }
                if (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int {
                    (*ctxt).instate = state;
                }
                return;
            }
            buf = xmlMallocAtomic.expect("non-null function pointer")(size) as *mut xmlChar;
            if buf.is_null() {
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                (*ctxt).instate = state;
                return;
            }
            if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"ParsePI: PI %s space expected\n\0" as *const u8 as *const ::core::ffi::c_char,
                    target,
                );
            }
            cur = xmlCurrentChar(ctxt, &raw mut l);
            while (if cur < 0x100 as ::core::ffi::c_int {
                (0x9 as ::core::ffi::c_int <= cur && cur <= 0xa as ::core::ffi::c_int
                    || cur == 0xd as ::core::ffi::c_int
                    || 0x20 as ::core::ffi::c_int <= cur) as ::core::ffi::c_int
            } else {
                (0x100 as ::core::ffi::c_int <= cur && cur <= 0xd7ff as ::core::ffi::c_int
                    || 0xe000 as ::core::ffi::c_int <= cur && cur <= 0xfffd as ::core::ffi::c_int
                    || 0x10000 as ::core::ffi::c_int <= cur
                        && cur <= 0x10ffff as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            }) != 0
                && (cur != '?' as i32
                    || *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        != '>' as i32)
            {
                if len.wrapping_add(5 as size_t) >= size {
                    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    let mut new_size: size_t = size.wrapping_mul(2 as size_t);
                    tmp = xmlRealloc.expect("non-null function pointer")(
                        buf as *mut ::core::ffi::c_void,
                        new_size,
                    ) as *mut xmlChar;
                    if tmp.is_null() {
                        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                        xmlFree.expect("non-null function pointer")(
                            buf as *mut ::core::ffi::c_void,
                        );
                        (*ctxt).instate = state;
                        return;
                    }
                    buf = tmp;
                    size = new_size;
                }
                if cur < 0x80 as ::core::ffi::c_int {
                    let fresh1 = len;
                    len = len.wrapping_add(1);
                    *buf.offset(fresh1 as isize) = cur as xmlChar;
                } else {
                    len = (len as ::core::ffi::c_ulong).wrapping_add(xmlCopyCharMultiByte(
                        buf.offset(len as isize) as *mut xmlChar,
                        cur,
                    )
                        as ::core::ffi::c_ulong) as size_t as size_t;
                }
                if len > maxLength {
                    xmlFatalErrMsgStr(
                        ctxt,
                        XML_ERR_PI_NOT_FINISHED,
                        b"PI %s too big found\0" as *const u8 as *const ::core::ffi::c_char,
                        target,
                    );
                    xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                    (*ctxt).instate = state;
                    return;
                }
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                    (*(*ctxt).input).line += 1;
                    (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                } else {
                    (*(*ctxt).input).col += 1;
                }
                (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                cur = xmlCurrentChar(ctxt, &raw mut l);
            }
            *buf.offset(len as isize) = 0 as xmlChar;
            if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                return;
            }
            if cur != '?' as i32 {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_PI_NOT_FINISHED,
                    b"ParsePI: PI %s never end ...\n\0" as *const u8 as *const ::core::ffi::c_char,
                    target,
                );
            } else {
                if inputid != (*(*ctxt).input).id {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ENTITY_BOUNDARY,
                        b"PI declaration doesn't start and stop in the same entity\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(2 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
                if (state as ::core::ffi::c_int == XML_PARSER_MISC as ::core::ffi::c_int
                    || state as ::core::ffi::c_int == XML_PARSER_START as ::core::ffi::c_int)
                    && xmlStrEqual(target, XML_CATALOG_PI) != 0
                {
                    let mut allow: xmlCatalogAllow = xmlCatalogGetDefaults();
                    if allow as ::core::ffi::c_uint
                        == XML_CATA_ALLOW_DOCUMENT as ::core::ffi::c_int as ::core::ffi::c_uint
                        || allow as ::core::ffi::c_uint
                            == XML_CATA_ALLOW_ALL as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        xmlParseCatalogPI(ctxt, buf);
                    }
                }
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0
                    && (*(*ctxt).sax).processingInstruction.is_some()
                {
                    (*(*ctxt).sax)
                        .processingInstruction
                        .expect("non-null function pointer")(
                        (*ctxt).userData, target, buf
                    );
                }
            }
            xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
        } else {
            xmlFatalErr(
                ctxt,
                XML_ERR_PI_NOT_STARTED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int {
            (*ctxt).instate = state;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseNotationDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut Pubid: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut Systemid: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '!' as i32
    {
        return;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(2 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'O' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'O' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
    {
        let mut inputid: ::core::ffi::c_int = (*(*ctxt).input).id;
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(8 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 8 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after '<!NOTATION'\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        name = xmlParseName(ctxt);
        if name.is_null() {
            xmlFatalErr(
                ctxt,
                XML_ERR_NOTATION_NOT_STARTED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return;
        }
        if !xmlStrchr(name, ':' as i32 as xmlChar).is_null() {
            xmlNsErr(
                ctxt,
                XML_NS_ERR_COLON,
                b"colons are forbidden from notation names '%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after the NOTATION name'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        Systemid = xmlParseExternalID(ctxt, &raw mut Pubid, 0 as ::core::ffi::c_int);
        xmlSkipBlankChars(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '>' as i32 {
            if inputid != (*(*ctxt).input).id {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ENTITY_BOUNDARY,
                    b"Notation declaration doesn't start and stop in the same entity\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            xmlNextChar(ctxt);
            if !(*ctxt).sax.is_null()
                && (*ctxt).disableSAX == 0
                && (*(*ctxt).sax).notationDecl.is_some()
            {
                (*(*ctxt).sax)
                    .notationDecl
                    .expect("non-null function pointer")(
                    (*ctxt).userData, name, Pubid, Systemid
                );
            }
        } else {
            xmlFatalErr(
                ctxt,
                XML_ERR_NOTATION_NOT_FINISHED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if !Systemid.is_null() {
            xmlFree.expect("non-null function pointer")(Systemid as *mut ::core::ffi::c_void);
        }
        if !Pubid.is_null() {
            xmlFree.expect("non-null function pointer")(Pubid as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEntityDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut current_block: u64;
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut value: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut URI: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut literal: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ndata: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut isParameter: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut orig: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '!' as i32
    {
        return;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(2 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'Y' as i32
    {
        let mut inputid: ::core::ffi::c_int = (*(*ctxt).input).id;
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(6 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 6 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after '<!ENTITY'\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '%' as i32 {
            xmlNextChar(ctxt);
            if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required after '%%'\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            isParameter = 1 as ::core::ffi::c_int;
        }
        name = xmlParseName(ctxt);
        if name.is_null() {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"xmlParseEntityDecl: no name\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if !xmlStrchr(name, ':' as i32 as xmlChar).is_null() {
            xmlNsErr(
                ctxt,
                XML_NS_ERR_COLON,
                b"colons are forbidden from entities names '%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after the entity name\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*ctxt).instate = XML_PARSER_ENTITY_DECL;
        if isParameter != 0 {
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '"' as i32
                || *(*(*ctxt).input).cur as ::core::ffi::c_int == '\'' as i32
            {
                value = xmlParseEntityValue(ctxt, &raw mut orig);
                if !value.is_null() {
                    if !(*ctxt).sax.is_null()
                        && (*ctxt).disableSAX == 0
                        && (*(*ctxt).sax).entityDecl.is_some()
                    {
                        (*(*ctxt).sax)
                            .entityDecl
                            .expect("non-null function pointer")(
                            (*ctxt).userData,
                            name,
                            XML_INTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                            value,
                        );
                    }
                }
            } else {
                URI = xmlParseExternalID(ctxt, &raw mut literal, 1 as ::core::ffi::c_int);
                if URI.is_null() && literal.is_null() {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_VALUE_REQUIRED,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
                if !URI.is_null() {
                    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
                    uri = xmlParseURI(URI as *const ::core::ffi::c_char);
                    if uri.is_null() {
                        xmlErrMsgStr(
                            ctxt,
                            XML_ERR_INVALID_URI,
                            b"Invalid URI: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                            URI,
                        );
                    } else {
                        if !(*uri).fragment.is_null() {
                            xmlFatalErr(
                                ctxt,
                                XML_ERR_URI_FRAGMENT,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                            );
                        } else if !(*ctxt).sax.is_null()
                            && (*ctxt).disableSAX == 0
                            && (*(*ctxt).sax).entityDecl.is_some()
                        {
                            (*(*ctxt).sax)
                                .entityDecl
                                .expect("non-null function pointer")(
                                (*ctxt).userData,
                                name,
                                XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int,
                                literal,
                                URI,
                                ::core::ptr::null_mut::<xmlChar>(),
                            );
                        }
                        xmlFreeURI(uri);
                    }
                }
            }
            current_block = 13003737910779602957;
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '"' as i32
            || *(*(*ctxt).input).cur as ::core::ffi::c_int == '\'' as i32
        {
            value = xmlParseEntityValue(ctxt, &raw mut orig);
            if !(*ctxt).sax.is_null()
                && (*ctxt).disableSAX == 0
                && (*(*ctxt).sax).entityDecl.is_some()
            {
                (*(*ctxt).sax)
                    .entityDecl
                    .expect("non-null function pointer")(
                    (*ctxt).userData,
                    name,
                    XML_INTERNAL_GENERAL_ENTITY as ::core::ffi::c_int,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                    value,
                );
            }
            if (*ctxt).myDoc.is_null()
                || xmlStrEqual(
                    (*(*ctxt).myDoc).version,
                    b"SAX compatibility mode document\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut xmlChar,
                ) != 0
            {
                if (*ctxt).myDoc.is_null() {
                    (*ctxt).myDoc = xmlNewDoc(
                        b"SAX compatibility mode document\0" as *const u8
                            as *const ::core::ffi::c_char as *mut xmlChar,
                    );
                    if (*ctxt).myDoc.is_null() {
                        xmlErrMemory(
                            ctxt,
                            b"New Doc failed\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        current_block = 3901875730753812121;
                    } else {
                        (*(*ctxt).myDoc).properties = XML_DOC_INTERNAL as ::core::ffi::c_int;
                        current_block = 8835654301469918283;
                    }
                } else {
                    current_block = 8835654301469918283;
                }
                match current_block {
                    3901875730753812121 => {}
                    _ => {
                        if (*(*ctxt).myDoc).intSubset.is_null() {
                            (*(*ctxt).myDoc).intSubset = xmlNewDtd(
                                (*ctxt).myDoc,
                                b"fake\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                            )
                                as *mut _xmlDtd;
                        }
                        xmlSAX2EntityDecl(
                            ctxt as *mut ::core::ffi::c_void,
                            name,
                            XML_INTERNAL_GENERAL_ENTITY as ::core::ffi::c_int,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                            value,
                        );
                        current_block = 13003737910779602957;
                    }
                }
            } else {
                current_block = 13003737910779602957;
            }
        } else {
            URI = xmlParseExternalID(ctxt, &raw mut literal, 1 as ::core::ffi::c_int);
            if URI.is_null() && literal.is_null() {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_VALUE_REQUIRED,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
            if !URI.is_null() {
                let mut uri_0: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
                uri_0 = xmlParseURI(URI as *const ::core::ffi::c_char);
                if uri_0.is_null() {
                    xmlErrMsgStr(
                        ctxt,
                        XML_ERR_INVALID_URI,
                        b"Invalid URI: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                        URI,
                    );
                } else {
                    if !(*uri_0).fragment.is_null() {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_URI_FRAGMENT,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                    xmlFreeURI(uri_0);
                }
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32
                && xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int
            {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"Space required before 'NDATA'\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'N' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'D' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'A' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'T' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'A' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 5 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
                if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_SPACE_REQUIRED,
                        b"Space required after 'NDATA'\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                ndata = xmlParseName(ctxt);
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0
                    && (*(*ctxt).sax).unparsedEntityDecl.is_some()
                {
                    (*(*ctxt).sax)
                        .unparsedEntityDecl
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        name,
                        literal,
                        URI,
                        ndata,
                    );
                }
                current_block = 13003737910779602957;
            } else {
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0
                    && (*(*ctxt).sax).entityDecl.is_some()
                {
                    (*(*ctxt).sax)
                        .entityDecl
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        name,
                        XML_EXTERNAL_GENERAL_PARSED_ENTITY as ::core::ffi::c_int,
                        literal,
                        URI,
                        ::core::ptr::null_mut::<xmlChar>(),
                    );
                }
                if (*ctxt).replaceEntities != 0 as ::core::ffi::c_int
                    && ((*ctxt).myDoc.is_null()
                        || xmlStrEqual(
                            (*(*ctxt).myDoc).version,
                            b"SAX compatibility mode document\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut xmlChar,
                        ) != 0)
                {
                    if (*ctxt).myDoc.is_null() {
                        (*ctxt).myDoc = xmlNewDoc(
                            b"SAX compatibility mode document\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut xmlChar,
                        );
                        if (*ctxt).myDoc.is_null() {
                            xmlErrMemory(
                                ctxt,
                                b"New Doc failed\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            current_block = 3901875730753812121;
                        } else {
                            (*(*ctxt).myDoc).properties = XML_DOC_INTERNAL as ::core::ffi::c_int;
                            current_block = 7416055328783156979;
                        }
                    } else {
                        current_block = 7416055328783156979;
                    }
                    match current_block {
                        3901875730753812121 => {}
                        _ => {
                            if (*(*ctxt).myDoc).intSubset.is_null() {
                                (*(*ctxt).myDoc).intSubset = xmlNewDtd(
                                    (*ctxt).myDoc,
                                    b"fake\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut xmlChar,
                                    ::core::ptr::null::<xmlChar>(),
                                    ::core::ptr::null::<xmlChar>(),
                                )
                                    as *mut _xmlDtd;
                            }
                            xmlSAX2EntityDecl(
                                ctxt as *mut ::core::ffi::c_void,
                                name,
                                XML_EXTERNAL_GENERAL_PARSED_ENTITY as ::core::ffi::c_int,
                                literal,
                                URI,
                                ::core::ptr::null_mut::<xmlChar>(),
                            );
                            current_block = 13003737910779602957;
                        }
                    }
                } else {
                    current_block = 13003737910779602957;
                }
            }
        }
        match current_block {
            13003737910779602957 => {
                if !((*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int)
                {
                    xmlSkipBlankChars(ctxt);
                    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32 {
                        xmlFatalErrMsgStr(
                            ctxt,
                            XML_ERR_ENTITY_NOT_FINISHED,
                            b"xmlParseEntityDecl: entity %s not terminated\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            name,
                        );
                        xmlHaltParser(ctxt);
                    } else {
                        if inputid != (*(*ctxt).input).id {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_ENTITY_BOUNDARY,
                                b"Entity declaration doesn't start and stop in the same entity\n\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        xmlNextChar(ctxt);
                    }
                    if !orig.is_null() {
                        let mut cur: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
                        if isParameter != 0 {
                            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).getParameterEntity.is_some()
                            {
                                cur = (*(*ctxt).sax)
                                    .getParameterEntity
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData, name
                                );
                            }
                        } else {
                            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).getEntity.is_some() {
                                cur = (*(*ctxt).sax).getEntity.expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    name,
                                );
                            }
                            if cur.is_null() && (*ctxt).userData == ctxt as *mut ::core::ffi::c_void
                            {
                                cur = xmlSAX2GetEntity(ctxt as *mut ::core::ffi::c_void, name);
                            }
                        }
                        if !cur.is_null() && (*cur).orig.is_null() {
                            (*cur).orig = orig;
                            orig = ::core::ptr::null_mut::<xmlChar>();
                        }
                    }
                }
            }
            _ => {}
        }
        if !value.is_null() {
            xmlFree.expect("non-null function pointer")(value as *mut ::core::ffi::c_void);
        }
        if !URI.is_null() {
            xmlFree.expect("non-null function pointer")(URI as *mut ::core::ffi::c_void);
        }
        if !literal.is_null() {
            xmlFree.expect("non-null function pointer")(literal as *mut ::core::ffi::c_void);
        }
        if !orig.is_null() {
            xmlFree.expect("non-null function pointer")(orig as *mut ::core::ffi::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseDefaultDecl(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *mut *mut xmlChar,
) -> ::core::ffi::c_int {
    let mut val: ::core::ffi::c_int = 0;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    *value = ::core::ptr::null_mut::<xmlChar>();
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '#' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'R' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'Q' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'U' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'R' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(9 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 9 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_REQUIRED as ::core::ffi::c_int;
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '#' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'M' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'P' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'L' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(8 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 8 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_IMPLIED as ::core::ffi::c_int;
    }
    val = XML_ATTRIBUTE_NONE as ::core::ffi::c_int;
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '#' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'F' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'X' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(6 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 6 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        val = XML_ATTRIBUTE_FIXED as ::core::ffi::c_int;
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after '#FIXED'\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    ret = xmlParseAttValue(ctxt);
    (*ctxt).instate = XML_PARSER_DTD;
    if ret.is_null() {
        xmlFatalErrMsg(
            ctxt,
            (*ctxt).errNo as xmlParserErrors,
            b"Attribute default value declaration error\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
        *value = ret;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseNotationType(mut ctxt: xmlParserCtxtPtr) -> xmlEnumerationPtr {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    let mut last: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    let mut cur: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    let mut tmp: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '(' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOTATION_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<xmlEnumeration>();
    }
    loop {
        xmlNextChar(ctxt);
        xmlSkipBlankChars(ctxt);
        name = xmlParseName(ctxt);
        if name.is_null() {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"Name expected in NOTATION declaration\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            xmlFreeEnumeration(ret);
            return ::core::ptr::null_mut::<xmlEnumeration>();
        }
        tmp = ret;
        while !tmp.is_null() {
            if xmlStrEqual(name, (*tmp).name) != 0 {
                xmlValidityError(
                    ctxt,
                    XML_DTD_DUP_TOKEN,
                    b"standalone: attribute notation value token %s duplicated\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    name,
                    ::core::ptr::null::<xmlChar>(),
                );
                if xmlDictOwns((*ctxt).dict, name) == 0 {
                    xmlFree.expect("non-null function pointer")(
                        name as *mut xmlChar as *mut ::core::ffi::c_void,
                    );
                }
                break;
            } else {
                tmp = (*tmp).next as xmlEnumerationPtr;
            }
        }
        if tmp.is_null() {
            cur = xmlCreateEnumeration(name);
            if cur.is_null() {
                xmlFreeEnumeration(ret);
                return ::core::ptr::null_mut::<xmlEnumeration>();
            }
            if last.is_null() {
                last = cur;
                ret = last;
            } else {
                (*last).next = cur as *mut _xmlEnumeration;
                last = cur;
            }
        }
        xmlSkipBlankChars(ctxt);
        if !(*(*(*ctxt).input).cur as ::core::ffi::c_int == '|' as i32) {
            break;
        }
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != ')' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOTATION_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        xmlFreeEnumeration(ret);
        return ::core::ptr::null_mut::<xmlEnumeration>();
    }
    xmlNextChar(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEnumerationType(mut ctxt: xmlParserCtxtPtr) -> xmlEnumerationPtr {
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    let mut last: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    let mut cur: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    let mut tmp: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '(' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_ATTLIST_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<xmlEnumeration>();
    }
    loop {
        xmlNextChar(ctxt);
        xmlSkipBlankChars(ctxt);
        name = xmlParseNmtoken(ctxt);
        if name.is_null() {
            xmlFatalErr(
                ctxt,
                XML_ERR_NMTOKEN_REQUIRED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return ret;
        }
        tmp = ret;
        while !tmp.is_null() {
            if xmlStrEqual(name, (*tmp).name) != 0 {
                xmlValidityError(
                    ctxt,
                    XML_DTD_DUP_TOKEN,
                    b"standalone: attribute enumeration value token %s duplicated\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    name,
                    ::core::ptr::null::<xmlChar>(),
                );
                if xmlDictOwns((*ctxt).dict, name) == 0 {
                    xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
                }
                break;
            } else {
                tmp = (*tmp).next as xmlEnumerationPtr;
            }
        }
        if tmp.is_null() {
            cur = xmlCreateEnumeration(name);
            if xmlDictOwns((*ctxt).dict, name) == 0 {
                xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
            }
            if cur.is_null() {
                xmlFreeEnumeration(ret);
                return ::core::ptr::null_mut::<xmlEnumeration>();
            }
            if last.is_null() {
                last = cur;
                ret = last;
            } else {
                (*last).next = cur as *mut _xmlEnumeration;
                last = cur;
            }
        }
        xmlSkipBlankChars(ctxt);
        if !(*(*(*ctxt).input).cur as ::core::ffi::c_int == '|' as i32) {
            break;
        }
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != ')' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_ATTLIST_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ret;
    }
    xmlNextChar(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEnumeratedType(
    mut ctxt: xmlParserCtxtPtr,
    mut tree: *mut xmlEnumerationPtr,
) -> ::core::ffi::c_int {
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'O' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'O' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(8 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 8 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after 'NOTATION'\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        *tree = xmlParseNotationType(ctxt);
        if (*tree).is_null() {
            return 0 as ::core::ffi::c_int;
        }
        return XML_ATTRIBUTE_NOTATION as ::core::ffi::c_int;
    }
    *tree = xmlParseEnumerationType(ctxt);
    if (*tree).is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return XML_ATTRIBUTE_ENUMERATION as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseAttributeType(
    mut ctxt: xmlParserCtxtPtr,
    mut tree: *mut xmlEnumerationPtr,
) -> ::core::ffi::c_int {
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'C' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(5 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 5 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_CDATA as ::core::ffi::c_int;
    } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'R' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'F' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'S' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(6 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 6 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_IDREFS as ::core::ffi::c_int;
    } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'R' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'F' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(5 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 5 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_IDREF as ::core::ffi::c_int;
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == 'I' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(2 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_ID as ::core::ffi::c_int;
    } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'Y' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(6 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 6 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_ENTITY as ::core::ffi::c_int;
    } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'S' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(8 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 8 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_ENTITIES as ::core::ffi::c_int;
    } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'M' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'O' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'K' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'S' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(8 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 8 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_NMTOKENS as ::core::ffi::c_int;
    } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'M' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'O' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'K' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(7 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 7 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        return XML_ATTRIBUTE_NMTOKEN as ::core::ffi::c_int;
    }
    return xmlParseEnumeratedType(ctxt, tree);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseAttributeListDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut elemName: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut attrName: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut tree: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '!' as i32
    {
        return;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(2 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'A' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'L' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'I' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'S' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
    {
        let mut inputid: ::core::ffi::c_int = (*(*ctxt).input).id;
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(7 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 7 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after '<!ATTLIST'\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        elemName = xmlParseName(ctxt);
        if elemName.is_null() {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"ATTLIST: no name for Element\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        xmlSkipBlankChars(ctxt);
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        while *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32
            && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
        {
            let mut type_0: ::core::ffi::c_int = 0;
            let mut def: ::core::ffi::c_int = 0;
            let mut defaultValue: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
            tree = ::core::ptr::null_mut::<xmlEnumeration>();
            attrName = xmlParseName(ctxt);
            if attrName.is_null() {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_NAME_REQUIRED,
                    b"ATTLIST: no name for Attribute\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                break;
            } else {
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_SPACE_REQUIRED,
                        b"Space required after the attribute name\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    break;
                } else {
                    type_0 = xmlParseAttributeType(ctxt, &raw mut tree);
                    if type_0 <= 0 as ::core::ffi::c_int {
                        break;
                    }
                    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                        as ::core::ffi::c_long)
                        < INPUT_CHUNK as ::core::ffi::c_long
                    {
                        xmlParserGrow(ctxt);
                    }
                    if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_SPACE_REQUIRED,
                            b"Space required after the attribute type\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        if !tree.is_null() {
                            xmlFreeEnumeration(tree);
                        }
                        break;
                    } else {
                        def = xmlParseDefaultDecl(ctxt, &raw mut defaultValue);
                        if def <= 0 as ::core::ffi::c_int {
                            if !defaultValue.is_null() {
                                xmlFree.expect("non-null function pointer")(
                                    defaultValue as *mut ::core::ffi::c_void,
                                );
                            }
                            if !tree.is_null() {
                                xmlFreeEnumeration(tree);
                            }
                            break;
                        } else {
                            if type_0 != XML_ATTRIBUTE_CDATA as ::core::ffi::c_int
                                && !defaultValue.is_null()
                            {
                                xmlAttrNormalizeSpace(defaultValue, defaultValue);
                            }
                            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as ::core::ffi::c_long)
                                < INPUT_CHUNK as ::core::ffi::c_long
                            {
                                xmlParserGrow(ctxt);
                            }
                            if *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32 {
                                if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                                    xmlFatalErrMsg(
                                        ctxt,
                                        XML_ERR_SPACE_REQUIRED,
                                        b"Space required after the attribute default value\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    if !defaultValue.is_null() {
                                        xmlFree.expect("non-null function pointer")(
                                            defaultValue as *mut ::core::ffi::c_void,
                                        );
                                    }
                                    if !tree.is_null() {
                                        xmlFreeEnumeration(tree);
                                    }
                                    break;
                                }
                            }
                            if !(*ctxt).sax.is_null()
                                && (*ctxt).disableSAX == 0
                                && (*(*ctxt).sax).attributeDecl.is_some()
                            {
                                (*(*ctxt).sax)
                                    .attributeDecl
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    elemName,
                                    attrName,
                                    type_0,
                                    def,
                                    defaultValue,
                                    tree,
                                );
                            } else if !tree.is_null() {
                                xmlFreeEnumeration(tree);
                            }
                            if (*ctxt).sax2 != 0
                                && !defaultValue.is_null()
                                && def != XML_ATTRIBUTE_IMPLIED as ::core::ffi::c_int
                                && def != XML_ATTRIBUTE_REQUIRED as ::core::ffi::c_int
                            {
                                xmlAddDefAttrs(ctxt, elemName, attrName, defaultValue);
                            }
                            if (*ctxt).sax2 != 0 {
                                xmlAddSpecialAttr(ctxt, elemName, attrName, type_0);
                            }
                            if !defaultValue.is_null() {
                                xmlFree.expect("non-null function pointer")(
                                    defaultValue as *mut ::core::ffi::c_void,
                                );
                            }
                            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                                as ::core::ffi::c_long)
                                < INPUT_CHUNK as ::core::ffi::c_long
                            {
                                xmlParserGrow(ctxt);
                            }
                        }
                    }
                }
            }
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '>' as i32 {
            if inputid != (*(*ctxt).input).id {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ENTITY_BOUNDARY,
                    b"Attribute list declaration doesn't start and stop in the same entity\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            xmlNextChar(ctxt);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseElementMixedContentDecl(
    mut ctxt: xmlParserCtxtPtr,
    mut inputchk: ::core::ffi::c_int,
) -> xmlElementContentPtr {
    let mut ret: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut cur: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut n: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut elem: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '#' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'P' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'C' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(7 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 7 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        xmlSkipBlankChars(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == ')' as i32 {
            if (*(*ctxt).input).id != inputchk {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ENTITY_BOUNDARY,
                    b"Element content declaration doesn't start and stop in the same entity\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            xmlNextChar(ctxt);
            ret = xmlNewDocElementContent(
                (*ctxt).myDoc,
                ::core::ptr::null::<xmlChar>(),
                XML_ELEMENT_CONTENT_PCDATA,
            );
            if ret.is_null() {
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '*' as i32 {
                (*ret).ocur = XML_ELEMENT_CONTENT_MULT;
                xmlNextChar(ctxt);
            }
            return ret;
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '(' as i32
            || *(*(*ctxt).input).cur as ::core::ffi::c_int == '|' as i32
        {
            cur = xmlNewDocElementContent(
                (*ctxt).myDoc,
                ::core::ptr::null::<xmlChar>(),
                XML_ELEMENT_CONTENT_PCDATA,
            );
            ret = cur;
            if ret.is_null() {
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
        }
        while *(*(*ctxt).input).cur as ::core::ffi::c_int == '|' as i32
            && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
        {
            xmlNextChar(ctxt);
            if elem.is_null() {
                ret = xmlNewDocElementContent(
                    (*ctxt).myDoc,
                    ::core::ptr::null::<xmlChar>(),
                    XML_ELEMENT_CONTENT_OR,
                );
                if ret.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, cur);
                    return ::core::ptr::null_mut::<xmlElementContent>();
                }
                (*ret).c1 = cur as *mut _xmlElementContent;
                if !cur.is_null() {
                    (*cur).parent = ret as *mut _xmlElementContent;
                }
                cur = ret;
            } else {
                n = xmlNewDocElementContent(
                    (*ctxt).myDoc,
                    ::core::ptr::null::<xmlChar>(),
                    XML_ELEMENT_CONTENT_OR,
                );
                if n.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, ret);
                    return ::core::ptr::null_mut::<xmlElementContent>();
                }
                (*n).c1 = xmlNewDocElementContent((*ctxt).myDoc, elem, XML_ELEMENT_CONTENT_ELEMENT)
                    as *mut _xmlElementContent;
                if !(*n).c1.is_null() {
                    (*(*n).c1).parent = n as *mut _xmlElementContent;
                }
                (*cur).c2 = n as *mut _xmlElementContent;
                if !n.is_null() {
                    (*n).parent = cur as *mut _xmlElementContent;
                }
                cur = n;
            }
            xmlSkipBlankChars(ctxt);
            elem = xmlParseName(ctxt);
            if elem.is_null() {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_NAME_REQUIRED,
                    b"xmlParseElementMixedContentDecl : Name expected\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                xmlFreeDocElementContent((*ctxt).myDoc, ret);
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            xmlSkipBlankChars(ctxt);
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == ')' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '*' as i32
        {
            if !elem.is_null() {
                (*cur).c2 =
                    xmlNewDocElementContent((*ctxt).myDoc, elem, XML_ELEMENT_CONTENT_ELEMENT)
                        as *mut _xmlElementContent;
                if !(*cur).c2.is_null() {
                    (*(*cur).c2).parent = cur as *mut _xmlElementContent;
                }
            }
            if !ret.is_null() {
                (*ret).ocur = XML_ELEMENT_CONTENT_MULT;
            }
            if (*(*ctxt).input).id != inputchk {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ENTITY_BOUNDARY,
                    b"Element content declaration doesn't start and stop in the same entity\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
        } else {
            xmlFreeDocElementContent((*ctxt).myDoc, ret);
            xmlFatalErr(
                ctxt,
                XML_ERR_MIXED_NOT_STARTED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return ::core::ptr::null_mut::<xmlElementContent>();
        }
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_PCDATA_REQUIRED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    return ret;
}
unsafe extern "C" fn xmlParseElementChildrenContentDeclPriv(
    mut ctxt: xmlParserCtxtPtr,
    mut inputchk: ::core::ffi::c_int,
    mut depth: ::core::ffi::c_int,
) -> xmlElementContentPtr {
    let mut ret: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut cur: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut last: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut op: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut elem: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut type_0: xmlChar = 0 as xmlChar;
    if depth > 128 as ::core::ffi::c_int
        && (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || depth > 2048 as ::core::ffi::c_int
    {
        xmlFatalErrMsgInt(
            ctxt,
            XML_ERR_ELEMCONTENT_NOT_FINISHED,
            b"xmlParseElementChildrenContentDecl : depth %d too deep, use XML_PARSE_HUGE\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            depth,
        );
        return ::core::ptr::null_mut::<xmlElementContent>();
    }
    xmlSkipBlankChars(ctxt);
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '(' as i32 {
        let mut inputid: ::core::ffi::c_int = (*(*ctxt).input).id;
        xmlNextChar(ctxt);
        xmlSkipBlankChars(ctxt);
        ret =
            xmlParseElementChildrenContentDeclPriv(ctxt, inputid, depth + 1 as ::core::ffi::c_int);
        cur = ret;
        if cur.is_null() {
            return ::core::ptr::null_mut::<xmlElementContent>();
        }
        xmlSkipBlankChars(ctxt);
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
    } else {
        elem = xmlParseName(ctxt);
        if elem.is_null() {
            xmlFatalErr(
                ctxt,
                XML_ERR_ELEMCONTENT_NOT_STARTED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return ::core::ptr::null_mut::<xmlElementContent>();
        }
        ret = xmlNewDocElementContent((*ctxt).myDoc, elem, XML_ELEMENT_CONTENT_ELEMENT);
        cur = ret;
        if cur.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return ::core::ptr::null_mut::<xmlElementContent>();
        }
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '?' as i32 {
            (*cur).ocur = XML_ELEMENT_CONTENT_OPT;
            xmlNextChar(ctxt);
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '*' as i32 {
            (*cur).ocur = XML_ELEMENT_CONTENT_MULT;
            xmlNextChar(ctxt);
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '+' as i32 {
            (*cur).ocur = XML_ELEMENT_CONTENT_PLUS;
            xmlNextChar(ctxt);
        } else {
            (*cur).ocur = XML_ELEMENT_CONTENT_ONCE;
        }
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
    }
    xmlSkipBlankChars(ctxt);
    while *(*(*ctxt).input).cur as ::core::ffi::c_int != ')' as i32
        && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
    {
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == ',' as i32 {
            if type_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                type_0 = *(*(*ctxt).input).cur;
            } else if type_0 as ::core::ffi::c_int != *(*(*ctxt).input).cur as ::core::ffi::c_int {
                xmlFatalErrMsgInt(
                    ctxt,
                    XML_ERR_SEPARATOR_REQUIRED,
                    b"xmlParseElementChildrenContentDecl : '%c' expected\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    type_0 as ::core::ffi::c_int,
                );
                if !last.is_null() && last != ret {
                    xmlFreeDocElementContent((*ctxt).myDoc, last);
                }
                if !ret.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, ret);
                }
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            xmlNextChar(ctxt);
            op = xmlNewDocElementContent(
                (*ctxt).myDoc,
                ::core::ptr::null::<xmlChar>(),
                XML_ELEMENT_CONTENT_SEQ,
            );
            if op.is_null() {
                if !last.is_null() && last != ret {
                    xmlFreeDocElementContent((*ctxt).myDoc, last);
                }
                xmlFreeDocElementContent((*ctxt).myDoc, ret);
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            if last.is_null() {
                (*op).c1 = ret as *mut _xmlElementContent;
                if !ret.is_null() {
                    (*ret).parent = op as *mut _xmlElementContent;
                }
                cur = op;
                ret = cur;
            } else {
                (*cur).c2 = op as *mut _xmlElementContent;
                if !op.is_null() {
                    (*op).parent = cur as *mut _xmlElementContent;
                }
                (*op).c1 = last as *mut _xmlElementContent;
                if !last.is_null() {
                    (*last).parent = op as *mut _xmlElementContent;
                }
                cur = op;
                last = ::core::ptr::null_mut::<xmlElementContent>();
            }
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '|' as i32 {
            if type_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                type_0 = *(*(*ctxt).input).cur;
            } else if type_0 as ::core::ffi::c_int != *(*(*ctxt).input).cur as ::core::ffi::c_int {
                xmlFatalErrMsgInt(
                    ctxt,
                    XML_ERR_SEPARATOR_REQUIRED,
                    b"xmlParseElementChildrenContentDecl : '%c' expected\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    type_0 as ::core::ffi::c_int,
                );
                if !last.is_null() && last != ret {
                    xmlFreeDocElementContent((*ctxt).myDoc, last);
                }
                if !ret.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, ret);
                }
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            xmlNextChar(ctxt);
            op = xmlNewDocElementContent(
                (*ctxt).myDoc,
                ::core::ptr::null::<xmlChar>(),
                XML_ELEMENT_CONTENT_OR,
            );
            if op.is_null() {
                if !last.is_null() && last != ret {
                    xmlFreeDocElementContent((*ctxt).myDoc, last);
                }
                if !ret.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, ret);
                }
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            if last.is_null() {
                (*op).c1 = ret as *mut _xmlElementContent;
                if !ret.is_null() {
                    (*ret).parent = op as *mut _xmlElementContent;
                }
                cur = op;
                ret = cur;
            } else {
                (*cur).c2 = op as *mut _xmlElementContent;
                if !op.is_null() {
                    (*op).parent = cur as *mut _xmlElementContent;
                }
                (*op).c1 = last as *mut _xmlElementContent;
                if !last.is_null() {
                    (*last).parent = op as *mut _xmlElementContent;
                }
                cur = op;
                last = ::core::ptr::null_mut::<xmlElementContent>();
            }
        } else {
            xmlFatalErr(
                ctxt,
                XML_ERR_ELEMCONTENT_NOT_FINISHED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if !last.is_null() && last != ret {
                xmlFreeDocElementContent((*ctxt).myDoc, last);
            }
            if !ret.is_null() {
                xmlFreeDocElementContent((*ctxt).myDoc, ret);
            }
            return ::core::ptr::null_mut::<xmlElementContent>();
        }
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        xmlSkipBlankChars(ctxt);
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '(' as i32 {
            let mut inputid_0: ::core::ffi::c_int = (*(*ctxt).input).id;
            xmlNextChar(ctxt);
            xmlSkipBlankChars(ctxt);
            last = xmlParseElementChildrenContentDeclPriv(
                ctxt,
                inputid_0,
                depth + 1 as ::core::ffi::c_int,
            );
            if last.is_null() {
                if !ret.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, ret);
                }
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            xmlSkipBlankChars(ctxt);
        } else {
            elem = xmlParseName(ctxt);
            if elem.is_null() {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_ELEMCONTENT_NOT_STARTED,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                if !ret.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, ret);
                }
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            last = xmlNewDocElementContent((*ctxt).myDoc, elem, XML_ELEMENT_CONTENT_ELEMENT);
            if last.is_null() {
                if !ret.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, ret);
                }
                return ::core::ptr::null_mut::<xmlElementContent>();
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '?' as i32 {
                (*last).ocur = XML_ELEMENT_CONTENT_OPT;
                xmlNextChar(ctxt);
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '*' as i32 {
                (*last).ocur = XML_ELEMENT_CONTENT_MULT;
                xmlNextChar(ctxt);
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '+' as i32 {
                (*last).ocur = XML_ELEMENT_CONTENT_PLUS;
                xmlNextChar(ctxt);
            } else {
                (*last).ocur = XML_ELEMENT_CONTENT_ONCE;
            }
        }
        xmlSkipBlankChars(ctxt);
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
    }
    if !cur.is_null() && !last.is_null() {
        (*cur).c2 = last as *mut _xmlElementContent;
        if !last.is_null() {
            (*last).parent = cur as *mut _xmlElementContent;
        }
    }
    if (*(*ctxt).input).id != inputchk {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_ENTITY_BOUNDARY,
            b"Element content declaration doesn't start and stop in the same entity\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    xmlNextChar(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '?' as i32 {
        if !ret.is_null() {
            if (*ret).ocur as ::core::ffi::c_uint
                == XML_ELEMENT_CONTENT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*ret).ocur as ::core::ffi::c_uint
                    == XML_ELEMENT_CONTENT_MULT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*ret).ocur = XML_ELEMENT_CONTENT_MULT;
            } else {
                (*ret).ocur = XML_ELEMENT_CONTENT_OPT;
            }
        }
        xmlNextChar(ctxt);
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '*' as i32 {
        if !ret.is_null() {
            (*ret).ocur = XML_ELEMENT_CONTENT_MULT;
            cur = ret;
            while !cur.is_null()
                && (*cur).type_0 as ::core::ffi::c_uint
                    == XML_ELEMENT_CONTENT_OR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if !(*cur).c1.is_null()
                    && ((*(*cur).c1).ocur as ::core::ffi::c_uint
                        == XML_ELEMENT_CONTENT_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*(*cur).c1).ocur as ::core::ffi::c_uint
                            == XML_ELEMENT_CONTENT_MULT as ::core::ffi::c_int
                                as ::core::ffi::c_uint)
                {
                    (*(*cur).c1).ocur = XML_ELEMENT_CONTENT_ONCE;
                }
                if !(*cur).c2.is_null()
                    && ((*(*cur).c2).ocur as ::core::ffi::c_uint
                        == XML_ELEMENT_CONTENT_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*(*cur).c2).ocur as ::core::ffi::c_uint
                            == XML_ELEMENT_CONTENT_MULT as ::core::ffi::c_int
                                as ::core::ffi::c_uint)
                {
                    (*(*cur).c2).ocur = XML_ELEMENT_CONTENT_ONCE;
                }
                cur = (*cur).c2 as xmlElementContentPtr;
            }
        }
        xmlNextChar(ctxt);
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '+' as i32 {
        if !ret.is_null() {
            let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if (*ret).ocur as ::core::ffi::c_uint
                == XML_ELEMENT_CONTENT_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*ret).ocur as ::core::ffi::c_uint
                    == XML_ELEMENT_CONTENT_MULT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*ret).ocur = XML_ELEMENT_CONTENT_MULT;
            } else {
                (*ret).ocur = XML_ELEMENT_CONTENT_PLUS;
            }
            while !cur.is_null()
                && (*cur).type_0 as ::core::ffi::c_uint
                    == XML_ELEMENT_CONTENT_OR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if !(*cur).c1.is_null()
                    && ((*(*cur).c1).ocur as ::core::ffi::c_uint
                        == XML_ELEMENT_CONTENT_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*(*cur).c1).ocur as ::core::ffi::c_uint
                            == XML_ELEMENT_CONTENT_MULT as ::core::ffi::c_int
                                as ::core::ffi::c_uint)
                {
                    (*(*cur).c1).ocur = XML_ELEMENT_CONTENT_ONCE;
                    found = 1 as ::core::ffi::c_int;
                }
                if !(*cur).c2.is_null()
                    && ((*(*cur).c2).ocur as ::core::ffi::c_uint
                        == XML_ELEMENT_CONTENT_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*(*cur).c2).ocur as ::core::ffi::c_uint
                            == XML_ELEMENT_CONTENT_MULT as ::core::ffi::c_int
                                as ::core::ffi::c_uint)
                {
                    (*(*cur).c2).ocur = XML_ELEMENT_CONTENT_ONCE;
                    found = 1 as ::core::ffi::c_int;
                }
                cur = (*cur).c2 as xmlElementContentPtr;
            }
            if found != 0 {
                (*ret).ocur = XML_ELEMENT_CONTENT_MULT;
            }
        }
        xmlNextChar(ctxt);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseElementChildrenContentDecl(
    mut ctxt: xmlParserCtxtPtr,
    mut inputchk: ::core::ffi::c_int,
) -> xmlElementContentPtr {
    return xmlParseElementChildrenContentDeclPriv(ctxt, inputchk, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseElementContentDecl(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut result: *mut xmlElementContentPtr,
) -> ::core::ffi::c_int {
    let mut tree: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut inputid: ::core::ffi::c_int = (*(*ctxt).input).id;
    let mut res: ::core::ffi::c_int = 0;
    *result = ::core::ptr::null_mut::<xmlElementContent>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '(' as i32 {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_ELEMCONTENT_NOT_STARTED,
            b"xmlParseElementContentDecl : %s '(' expected\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
        return -(1 as ::core::ffi::c_int);
    }
    xmlNextChar(ctxt);
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    xmlSkipBlankChars(ctxt);
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '#' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'P' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'C' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
    {
        tree = xmlParseElementMixedContentDecl(ctxt, inputid);
        res = XML_ELEMENT_TYPE_MIXED as ::core::ffi::c_int;
    } else {
        tree = xmlParseElementChildrenContentDeclPriv(ctxt, inputid, 1 as ::core::ffi::c_int);
        res = XML_ELEMENT_TYPE_ELEMENT as ::core::ffi::c_int;
    }
    xmlSkipBlankChars(ctxt);
    *result = tree;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseElementDecl(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut content: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '!' as i32
    {
        return ret;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(2 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'L' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'M' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
    {
        let mut inputid: ::core::ffi::c_int = (*(*ctxt).input).id;
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(7 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 7 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after 'ELEMENT'\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        name = xmlParseName(ctxt);
        if name.is_null() {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NAME_REQUIRED,
                b"xmlParseElementDecl: no name for Element\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_SPACE_REQUIRED,
                b"Space required after the element name\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'M' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'P' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'Y' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(5 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 5 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
            ret = XML_ELEMENT_TYPE_EMPTY as ::core::ffi::c_int;
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == 'A' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'N' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'Y' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(3 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
            ret = XML_ELEMENT_TYPE_ANY as ::core::ffi::c_int;
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '(' as i32 {
            ret = xmlParseElementContentDecl(ctxt, name, &raw mut content);
        } else {
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '%' as i32
                && (*ctxt).external == 0 as ::core::ffi::c_int
                && (*ctxt).inputNr == 1 as ::core::ffi::c_int
            {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_PEREF_IN_INT_SUBSET,
                    b"PEReference: forbidden within markup decl in internal subset\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ELEMCONTENT_NOT_STARTED,
                    b"xmlParseElementDecl: 'EMPTY', 'ANY' or '(' expected\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            return -(1 as ::core::ffi::c_int);
        }
        xmlSkipBlankChars(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_GT_REQUIRED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            if !content.is_null() {
                xmlFreeDocElementContent((*ctxt).myDoc, content);
            }
        } else {
            if inputid != (*(*ctxt).input).id {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ENTITY_BOUNDARY,
                    b"Element declaration doesn't start and stop in the same entity\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            xmlNextChar(ctxt);
            if !(*ctxt).sax.is_null()
                && (*ctxt).disableSAX == 0
                && (*(*ctxt).sax).elementDecl.is_some()
            {
                if !content.is_null() {
                    (*content).parent = ::core::ptr::null_mut::<_xmlElementContent>();
                }
                (*(*ctxt).sax)
                    .elementDecl
                    .expect("non-null function pointer")(
                    (*ctxt).userData, name, ret, content
                );
                if !content.is_null() && (*content).parent.is_null() {
                    xmlFreeDocElementContent((*ctxt).myDoc, content);
                }
            } else if !content.is_null() {
                xmlFreeDocElementContent((*ctxt).myDoc, content);
            }
        }
    }
    return ret;
}
unsafe extern "C" fn xmlParseConditionalSections(mut ctxt: xmlParserCtxtPtr) {
    let mut inputIds: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut inputIdsSize: size_t = 0 as size_t;
    let mut depth: size_t = 0 as size_t;
    's_7: while (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int {
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '!' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '[' as i32
        {
            let mut id: ::core::ffi::c_int = (*(*ctxt).input).id;
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(3 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
            xmlSkipBlankChars(ctxt);
            if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'I' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'N' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'C' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'L' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'U' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'D' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(6 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'E' as i32
            {
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(7 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 7 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
                xmlSkipBlankChars(ctxt);
                if *(*(*ctxt).input).cur as ::core::ffi::c_int != '[' as i32 {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_CONDSEC_INVALID,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                    xmlHaltParser(ctxt);
                    break;
                } else {
                    if (*(*ctxt).input).id != id {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ENTITY_BOUNDARY,
                            b"All markup of the conditional section is not in the same entity\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    xmlNextChar(ctxt);
                    if inputIdsSize <= depth {
                        let mut tmp: *mut ::core::ffi::c_int =
                            ::core::ptr::null_mut::<::core::ffi::c_int>();
                        inputIdsSize = if inputIdsSize == 0 as size_t {
                            4 as size_t
                        } else {
                            inputIdsSize.wrapping_mul(2 as size_t)
                        };
                        tmp = xmlRealloc.expect("non-null function pointer")(
                            inputIds as *mut ::core::ffi::c_void,
                            inputIdsSize.wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_int>() as size_t
                            ),
                        ) as *mut ::core::ffi::c_int;
                        if tmp.is_null() {
                            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                            break;
                        } else {
                            inputIds = tmp;
                        }
                    }
                    *inputIds.offset(depth as isize) = id;
                    depth = depth.wrapping_add(1);
                }
            } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                == 'I' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'G' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'N' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'O' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'R' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'E' as i32
            {
                let mut ignoreDepth: size_t = 0 as size_t;
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(6 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 6 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
                xmlSkipBlankChars(ctxt);
                if *(*(*ctxt).input).cur as ::core::ffi::c_int != '[' as i32 {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_CONDSEC_INVALID,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                    xmlHaltParser(ctxt);
                    break;
                } else {
                    if (*(*ctxt).input).id != id {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_ENTITY_BOUNDARY,
                            b"All markup of the conditional section is not in the same entity\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    xmlNextChar(ctxt);
                    while *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
                            && *(*(*ctxt).input)
                                .cur
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '!' as i32
                            && *(*(*ctxt).input)
                                .cur
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '[' as i32
                        {
                            (*(*ctxt).input).cur = (*(*ctxt).input)
                                .cur
                                .offset(3 as ::core::ffi::c_int as isize);
                            (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
                            if *(*(*ctxt).input).cur as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                xmlParserGrow(ctxt);
                            }
                            ignoreDepth = ignoreDepth.wrapping_add(1);
                            if !(ignoreDepth == 0 as size_t) {
                                continue;
                            }
                            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                            break 's_7;
                        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == ']' as i32
                            && *(*(*ctxt).input)
                                .cur
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ']' as i32
                            && *(*(*ctxt).input)
                                .cur
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '>' as i32
                        {
                            if ignoreDepth == 0 as size_t {
                                break;
                            }
                            (*(*ctxt).input).cur = (*(*ctxt).input)
                                .cur
                                .offset(3 as ::core::ffi::c_int as isize);
                            (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
                            if *(*(*ctxt).input).cur as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                xmlParserGrow(ctxt);
                            }
                            ignoreDepth = ignoreDepth.wrapping_sub(1);
                        } else {
                            xmlNextChar(ctxt);
                        }
                    }
                    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_CONDSEC_NOT_FINISHED,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                        break;
                    } else {
                        if (*(*ctxt).input).id != id {
                            xmlFatalErrMsg(
                                ctxt,
                                XML_ERR_ENTITY_BOUNDARY,
                                b"All markup of the conditional section is not in the same entity\n\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        (*(*ctxt).input).cur = (*(*ctxt).input)
                            .cur
                            .offset(3 as ::core::ffi::c_int as isize);
                        (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
                        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            xmlParserGrow(ctxt);
                        }
                    }
                }
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_CONDSEC_INVALID_KEYWORD,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                xmlHaltParser(ctxt);
                break;
            }
        } else if depth > 0 as size_t
            && *(*(*ctxt).input).cur as ::core::ffi::c_int == ']' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ']' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '>' as i32
        {
            depth = depth.wrapping_sub(1);
            if (*(*ctxt).input).id != *inputIds.offset(depth as isize) {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_ENTITY_BOUNDARY,
                    b"All markup of the conditional section is not in the same entity\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(3 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
            && (*(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '!' as i32
                || *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '?' as i32)
        {
            xmlParseMarkupDecl(ctxt);
        } else {
            xmlFatalErr(
                ctxt,
                XML_ERR_EXT_SUBSET_NOT_FINISHED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlHaltParser(ctxt);
            break;
        }
        if depth == 0 as size_t {
            break;
        }
        xmlSkipBlankChars(ctxt);
        if ((*ctxt).progressive == 0 as ::core::ffi::c_int
            || (*ctxt).inputNr > 1 as ::core::ffi::c_int)
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                > (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
        {
            xmlParserShrink(ctxt);
        }
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
    }
    xmlFree.expect("non-null function pointer")(inputIds as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseMarkupDecl(mut ctxt: xmlParserCtxtPtr) {
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32 {
        if *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '!' as i32
        {
            match *(*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            {
                69 => {
                    if *(*(*ctxt).input)
                        .cur
                        .offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 'L' as i32
                    {
                        xmlParseElementDecl(ctxt);
                    } else if *(*(*ctxt).input)
                        .cur
                        .offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 'N' as i32
                    {
                        xmlParseEntityDecl(ctxt);
                    } else {
                        (*(*ctxt).input).cur = (*(*ctxt).input)
                            .cur
                            .offset(2 as ::core::ffi::c_int as isize);
                        (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
                        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            xmlParserGrow(ctxt);
                        }
                    }
                }
                65 => {
                    xmlParseAttributeListDecl(ctxt);
                }
                78 => {
                    xmlParseNotationDecl(ctxt);
                }
                45 => {
                    xmlParseComment(ctxt);
                }
                _ => {
                    (*(*ctxt).input).cur = (*(*ctxt).input)
                        .cur
                        .offset(2 as ::core::ffi::c_int as isize);
                    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
                    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        xmlParserGrow(ctxt);
                    }
                }
            }
        } else if *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '?' as i32
        {
            xmlParsePI(ctxt);
        }
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return;
    }
    (*ctxt).instate = XML_PARSER_DTD;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseTextDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut version: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut oldstate: ::core::ffi::c_int = 0;
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '<' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '?' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'x' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'm' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'l' as i32
        && (*(*(*ctxt).input)
            .cur
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int
                <= *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                && *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= 0xa as ::core::ffi::c_int
            || *(*(*ctxt).input)
                .cur
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xd as ::core::ffi::c_int)
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(5 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 5 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_XMLDECL_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    oldstate = (*ctxt).instate as ::core::ffi::c_int;
    (*ctxt).instate = XML_PARSER_START;
    if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_SPACE_REQUIRED,
            b"Space needed after '<?xml'\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    version = xmlParseVersionInfo(ctxt);
    if version.is_null() {
        version = xmlCharStrdup(XML_DEFAULT_VERSION.as_ptr());
    } else if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_SPACE_REQUIRED,
            b"Space needed here\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(*ctxt).input).version = version;
    xmlParseEncodingDecl(ctxt);
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return;
    }
    if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as ::core::ffi::c_int {
        (*ctxt).instate = oldstate as xmlParserInputState;
        return;
    }
    xmlSkipBlankChars(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '?' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '>' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(2 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '>' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_XMLDECL_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        xmlNextChar(ctxt);
    } else {
        let mut c: ::core::ffi::c_int = 0;
        xmlFatalErr(
            ctxt,
            XML_ERR_XMLDECL_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        loop {
            c = *(*(*ctxt).input).cur as ::core::ffi::c_int;
            if !(c != 0 as ::core::ffi::c_int) {
                break;
            }
            xmlNextChar(ctxt);
            if c == '>' as i32 {
                break;
            }
        }
    }
    if (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int {
        (*ctxt).instate = oldstate as xmlParserInputState;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseExternalSubset(
    mut ctxt: xmlParserCtxtPtr,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    xmlDetectSAX2(ctxt);
    xmlDetectEncoding(ctxt);
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '<' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '?' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'x' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'm' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'l' as i32
    {
        xmlParseTextDecl(ctxt);
        if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as ::core::ffi::c_int {
            xmlHaltParser(ctxt);
            return;
        }
    }
    if (*ctxt).myDoc.is_null() {
        (*ctxt).myDoc =
            xmlNewDoc(b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
        if (*ctxt).myDoc.is_null() {
            xmlErrMemory(
                ctxt,
                b"New Doc failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        (*(*ctxt).myDoc).properties = XML_DOC_INTERNAL as ::core::ffi::c_int;
    }
    if !(*ctxt).myDoc.is_null() && (*(*ctxt).myDoc).intSubset.is_null() {
        xmlCreateIntSubset(
            (*ctxt).myDoc,
            ::core::ptr::null::<xmlChar>(),
            ExternalID,
            SystemID,
        );
    }
    (*ctxt).instate = XML_PARSER_DTD;
    (*ctxt).external = 1 as ::core::ffi::c_int;
    xmlSkipBlankChars(ctxt);
    while (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
        && *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '!' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '[' as i32
        {
            xmlParseConditionalSections(ctxt);
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
            && (*(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '!' as i32
                || *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '?' as i32)
        {
            xmlParseMarkupDecl(ctxt);
        } else {
            xmlFatalErr(
                ctxt,
                XML_ERR_EXT_SUBSET_NOT_FINISHED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlHaltParser(ctxt);
            return;
        }
        xmlSkipBlankChars(ctxt);
        if ((*ctxt).progressive == 0 as ::core::ffi::c_int
            || (*ctxt).inputNr > 1 as ::core::ffi::c_int)
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                > (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
        {
            xmlParserShrink(ctxt);
        }
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        xmlFatalErr(
            ctxt,
            XML_ERR_EXT_SUBSET_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseReference(mut ctxt: xmlParserCtxtPtr) {
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut val: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut was_checked: ::core::ffi::c_int = 0;
    let mut list: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: xmlParserErrors = XML_ERR_OK;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '&' as i32 {
        return;
    }
    if *(*(*ctxt).input)
        .cur
        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '#' as i32
    {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut out: [xmlChar; 16] = [0; 16];
        let mut value: ::core::ffi::c_int = xmlParseCharRef(ctxt);
        if value == 0 as ::core::ffi::c_int {
            return;
        }
        if value < 0x80 as ::core::ffi::c_int {
            let fresh14 = i;
            i = i + 1;
            out[fresh14 as usize] = value as xmlChar;
        } else {
            i += xmlCopyCharMultiByte(
                (&raw mut out as *mut xmlChar).offset(i as isize) as *mut xmlChar,
                value,
            );
        }
        out[i as usize] = 0 as xmlChar;
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).characters.is_some() && (*ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .characters
                .expect("non-null function pointer")(
                (*ctxt).userData,
                &raw mut out as *mut xmlChar,
                i,
            );
        }
        return;
    }
    ent = xmlParseEntityRef(ctxt);
    if ent.is_null() {
        return;
    }
    if (*ctxt).wellFormed == 0 {
        return;
    }
    was_checked = (*ent).flags & XML_ENT_PARSED;
    if (*ent).name.is_null()
        || (*ent).etype as ::core::ffi::c_uint
            == XML_INTERNAL_PREDEFINED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        val = (*ent).content;
        if val.is_null() {
            return;
        }
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).characters.is_some() && (*ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .characters
                .expect("non-null function pointer")(
                (*ctxt).userData, val, xmlStrlen(val)
            );
        }
        return;
    }
    if (*ent).flags & XML_ENT_PARSED == 0 as ::core::ffi::c_int
        && ((*ent).etype as ::core::ffi::c_uint
            != XML_EXTERNAL_GENERAL_PARSED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*ctxt).options
                & (XML_PARSE_NOENT as ::core::ffi::c_int
                    | XML_PARSE_DTDVALID as ::core::ffi::c_int)
                != 0)
    {
        let mut oldsizeentcopy: ::core::ffi::c_ulong = (*ctxt).sizeentcopy;
        let mut user_data: *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*ctxt).userData == ctxt as *mut ::core::ffi::c_void {
            user_data = NULL;
        } else {
            user_data = (*ctxt).userData;
        }
        (*ctxt).sizeentcopy = 0 as ::core::ffi::c_ulong;
        if (*ent).flags & XML_ENT_EXPANDING != 0 {
            xmlFatalErr(
                ctxt,
                XML_ERR_ENTITY_LOOP,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlHaltParser(ctxt);
            return;
        }
        (*ent).flags |= XML_ENT_EXPANDING;
        if (*ent).etype as ::core::ffi::c_uint
            == XML_INTERNAL_GENERAL_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*ctxt).depth += 1;
            ret =
                xmlParseBalancedChunkMemoryInternal(ctxt, (*ent).content, user_data, &raw mut list);
            (*ctxt).depth -= 1;
        } else if (*ent).etype as ::core::ffi::c_uint
            == XML_EXTERNAL_GENERAL_PARSED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*ctxt).depth += 1;
            ret = xmlParseExternalEntityPrivate(
                (*ctxt).myDoc,
                ctxt,
                (*ctxt).sax as xmlSAXHandlerPtr,
                user_data,
                (*ctxt).depth,
                (*ent).URI,
                (*ent).ExternalID,
                &raw mut list,
            );
            (*ctxt).depth -= 1;
        } else {
            ret = XML_ERR_ENTITY_PE_INTERNAL;
            xmlErrMsgStr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"invalid entity type found\n\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<xmlChar>(),
            );
        }
        (*ent).flags &= !XML_ENT_EXPANDING;
        (*ent).flags |= XML_ENT_PARSED | XML_ENT_CHECKED;
        (*ent).expandedSize = (*ctxt).sizeentcopy;
        if ret as ::core::ffi::c_uint
            == XML_ERR_ENTITY_LOOP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xmlHaltParser(ctxt);
            xmlFreeNodeList(list);
            return;
        }
        if xmlParserEntityCheck(ctxt, oldsizeentcopy) != 0 {
            xmlFreeNodeList(list);
            return;
        }
        if ret as ::core::ffi::c_uint == XML_ERR_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            && !list.is_null()
        {
            (*ent).children = list as *mut _xmlNode;
            if (*ctxt).replaceEntities == 0 as ::core::ffi::c_int
                || (*ctxt).parseMode as ::core::ffi::c_uint
                    == XML_PARSE_READER as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*list).type_0 as ::core::ffi::c_uint
                    == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*list).next.is_null()
            {
                (*ent).owner = 1 as ::core::ffi::c_int;
                while !list.is_null() {
                    (*list).parent = ent as xmlNodePtr as *mut _xmlNode;
                    if (*list).doc != (*ent).doc {
                        xmlSetTreeDoc(list, (*ent).doc as xmlDocPtr);
                    }
                    if (*list).next.is_null() {
                        (*ent).last = list as *mut _xmlNode;
                    }
                    list = (*list).next as xmlNodePtr;
                }
                list = ::core::ptr::null_mut::<xmlNode>();
            } else {
                (*ent).owner = 0 as ::core::ffi::c_int;
                while !list.is_null() {
                    (*list).parent = (*ctxt).node as *mut _xmlNode;
                    (*list).doc = (*ctxt).myDoc as *mut _xmlDoc;
                    if (*list).next.is_null() {
                        (*ent).last = list as *mut _xmlNode;
                    }
                    list = (*list).next as xmlNodePtr;
                }
                list = (*ent).children as xmlNodePtr;
            }
        } else if ret as ::core::ffi::c_uint
            != XML_ERR_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            && ret as ::core::ffi::c_uint
                != XML_WAR_UNDECLARED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_UNDECLARED_ENTITY,
                b"Entity '%s' failed to parse\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*ent).name,
            );
            if !(*ent).content.is_null() {
                *(*ent).content.offset(0 as ::core::ffi::c_int as isize) = 0 as xmlChar;
            }
        } else if !list.is_null() {
            xmlFreeNodeList(list);
            list = ::core::ptr::null_mut::<xmlNode>();
        }
        was_checked = 0 as ::core::ffi::c_int;
    }
    if (*ent).children.is_null() {
        if was_checked != 0 as ::core::ffi::c_int {
            let mut user_data_0: *mut ::core::ffi::c_void =
                ::core::ptr::null_mut::<::core::ffi::c_void>();
            if (*ctxt).userData == ctxt as *mut ::core::ffi::c_void {
                user_data_0 = NULL;
            } else {
                user_data_0 = (*ctxt).userData;
            }
            if (*ent).etype as ::core::ffi::c_uint
                == XML_INTERNAL_GENERAL_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*ctxt).depth += 1;
                ret = xmlParseBalancedChunkMemoryInternal(
                    ctxt,
                    (*ent).content,
                    user_data_0,
                    ::core::ptr::null_mut::<xmlNodePtr>(),
                );
                (*ctxt).depth -= 1;
            } else if (*ent).etype as ::core::ffi::c_uint
                == XML_EXTERNAL_GENERAL_PARSED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut oldsizeentities: ::core::ffi::c_ulong = (*ctxt).sizeentities;
                (*ctxt).depth += 1;
                ret = xmlParseExternalEntityPrivate(
                    (*ctxt).myDoc,
                    ctxt,
                    (*ctxt).sax as xmlSAXHandlerPtr,
                    user_data_0,
                    (*ctxt).depth,
                    (*ent).URI,
                    (*ent).ExternalID,
                    ::core::ptr::null_mut::<xmlNodePtr>(),
                );
                (*ctxt).depth -= 1;
                (*ctxt).sizeentities = oldsizeentities;
            } else {
                ret = XML_ERR_ENTITY_PE_INTERNAL;
                xmlErrMsgStr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"invalid entity type found\n\0" as *const u8 as *const ::core::ffi::c_char,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            if ret as ::core::ffi::c_uint
                == XML_ERR_ENTITY_LOOP as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_ENTITY_LOOP,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                return;
            }
            if xmlParserEntityCheck(ctxt, 0 as ::core::ffi::c_ulong) != 0 {
                return;
            }
        }
        if !(*ctxt).sax.is_null()
            && (*(*ctxt).sax).reference.is_some()
            && (*ctxt).replaceEntities == 0 as ::core::ffi::c_int
            && (*ctxt).disableSAX == 0
        {
            (*(*ctxt).sax).reference.expect("non-null function pointer")(
                (*ctxt).userData,
                (*ent).name,
            );
        }
        return;
    }
    if was_checked != 0 as ::core::ffi::c_int
        && xmlParserEntityCheck(ctxt, (*ent).expandedSize) != 0
    {
        return;
    }
    if !(*ctxt).sax.is_null()
        && (*(*ctxt).sax).reference.is_some()
        && (*ctxt).replaceEntities == 0 as ::core::ffi::c_int
        && (*ctxt).disableSAX == 0
    {
        (*(*ctxt).sax).reference.expect("non-null function pointer")((*ctxt).userData, (*ent).name);
        return;
    }
    if (*ctxt).replaceEntities != 0 {
        if !(*ctxt).node.is_null() {
            if list.is_null() && (*ent).owner == 0 as ::core::ffi::c_int
                || (*ctxt).parseMode as ::core::ffi::c_uint
                    == XML_PARSE_READER as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut nw: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                let mut firstChild: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                cur = (*ent).children as xmlNodePtr;
                while !cur.is_null() {
                    nw = xmlDocCopyNode(cur, (*ctxt).myDoc, 1 as ::core::ffi::c_int);
                    if !nw.is_null() {
                        if (*nw)._private.is_null() {
                            (*nw)._private = (*cur)._private;
                        }
                        if firstChild.is_null() {
                            firstChild = nw;
                        }
                        nw = xmlAddChild((*ctxt).node, nw);
                    }
                    if cur == (*ent).last {
                        if (*ctxt).parseMode as ::core::ffi::c_uint
                            == XML_PARSE_READER as ::core::ffi::c_int as ::core::ffi::c_uint
                            && !nw.is_null()
                            && (*nw).type_0 as ::core::ffi::c_uint
                                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*nw).children.is_null()
                        {
                            (*nw).extra = 1 as ::core::ffi::c_ushort;
                        }
                        break;
                    } else {
                        cur = (*cur).next as xmlNodePtr;
                    }
                }
            } else if list.is_null() || (*ctxt).inputNr > 0 as ::core::ffi::c_int {
                let mut nw_0: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                let mut cur_0: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                let mut next: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                let mut last: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                let mut firstChild_0: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                cur_0 = (*ent).children as xmlNodePtr;
                (*ent).children = ::core::ptr::null_mut::<_xmlNode>();
                last = (*ent).last as xmlNodePtr;
                (*ent).last = ::core::ptr::null_mut::<_xmlNode>();
                while !cur_0.is_null() {
                    next = (*cur_0).next as xmlNodePtr;
                    (*cur_0).next = ::core::ptr::null_mut::<_xmlNode>();
                    (*cur_0).parent = ::core::ptr::null_mut::<_xmlNode>();
                    nw_0 = xmlDocCopyNode(cur_0, (*ctxt).myDoc, 1 as ::core::ffi::c_int);
                    if !nw_0.is_null() {
                        if (*nw_0)._private.is_null() {
                            (*nw_0)._private = (*cur_0)._private;
                        }
                        if firstChild_0.is_null() {
                            firstChild_0 = cur_0;
                        }
                        xmlAddChild(ent as xmlNodePtr, nw_0);
                    }
                    xmlAddChild((*ctxt).node, cur_0);
                    if cur_0 == last {
                        break;
                    }
                    cur_0 = next;
                }
                if (*ent).owner == 0 as ::core::ffi::c_int {
                    (*ent).owner = 1 as ::core::ffi::c_int;
                }
            } else {
                let mut nbktext: *const xmlChar = ::core::ptr::null::<xmlChar>();
                nbktext = xmlDictLookup(
                    (*ctxt).dict,
                    b"nbktext\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                    -(1 as ::core::ffi::c_int),
                );
                if (*(*ent).children).type_0 as ::core::ffi::c_uint
                    == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*(*ent).children).name = nbktext;
                }
                if (*ent).last != (*ent).children
                    && (*(*ent).last).type_0 as ::core::ffi::c_uint
                        == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*(*ent).last).name = nbktext;
                }
                xmlAddChildList((*ctxt).node, (*ent).children as xmlNodePtr);
            }
            (*ctxt).nodemem = 0 as ::core::ffi::c_int;
            (*ctxt).nodelen = 0 as ::core::ffi::c_int;
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEntityRef(mut ctxt: xmlParserCtxtPtr) -> xmlEntityPtr {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '&' as i32 {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    xmlNextChar(ctxt);
    name = xmlParseName(ctxt);
    if name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"xmlParseEntityRef: no name\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != ';' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_ENTITYREF_SEMICOL_MISSING,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    xmlNextChar(ctxt);
    if (*ctxt).options & XML_PARSE_OLDSAX as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        ent = xmlGetPredefinedEntity(name);
        if !ent.is_null() {
            return ent;
        }
    }
    if !(*ctxt).sax.is_null() {
        if (*(*ctxt).sax).getEntity.is_some() {
            ent = (*(*ctxt).sax).getEntity.expect("non-null function pointer")(
                (*ctxt).userData,
                name,
            );
        }
        if (*ctxt).wellFormed == 1 as ::core::ffi::c_int
            && ent.is_null()
            && (*ctxt).options & XML_PARSE_OLDSAX as ::core::ffi::c_int != 0
        {
            ent = xmlGetPredefinedEntity(name);
        }
        if (*ctxt).wellFormed == 1 as ::core::ffi::c_int
            && ent.is_null()
            && (*ctxt).userData == ctxt as *mut ::core::ffi::c_void
        {
            ent = xmlSAX2GetEntity(ctxt as *mut ::core::ffi::c_void, name);
        }
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    if ent.is_null() {
        if (*ctxt).standalone == 1 as ::core::ffi::c_int
            || (*ctxt).hasExternalSubset == 0 as ::core::ffi::c_int
                && (*ctxt).hasPErefs == 0 as ::core::ffi::c_int
        {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_UNDECLARED_ENTITY,
                b"Entity '%s' not defined\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
        } else {
            xmlErrMsgStr(
                ctxt,
                XML_WAR_UNDECLARED_ENTITY,
                b"Entity '%s' not defined\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
            if (*ctxt).inSubset == 0 as ::core::ffi::c_int
                && !(*ctxt).sax.is_null()
                && (*ctxt).disableSAX == 0 as ::core::ffi::c_int
                && (*(*ctxt).sax).reference.is_some()
            {
                (*(*ctxt).sax).reference.expect("non-null function pointer")(
                    (*ctxt).userData,
                    name,
                );
            }
        }
        (*ctxt).valid = 0 as ::core::ffi::c_int;
    } else if (*ent).etype as ::core::ffi::c_uint
        == XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_UNPARSED_ENTITY,
            b"Entity reference to unparsed entity %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
    } else if (*ctxt).instate as ::core::ffi::c_int
        == XML_PARSER_ATTRIBUTE_VALUE as ::core::ffi::c_int
        && (*ent).etype as ::core::ffi::c_uint
            == XML_EXTERNAL_GENERAL_PARSED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_ENTITY_IS_EXTERNAL,
            b"Attribute references external entity '%s'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
    } else if (*ctxt).instate as ::core::ffi::c_int
        == XML_PARSER_ATTRIBUTE_VALUE as ::core::ffi::c_int
        && (*ent).etype as ::core::ffi::c_uint
            != XML_INTERNAL_PREDEFINED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*ent).flags & XML_ENT_CHECKED_LT == 0 as ::core::ffi::c_int {
            if !(*ent).content.is_null()
                && !xmlStrchr((*ent).content, '<' as i32 as xmlChar).is_null()
            {
                (*ent).flags |= XML_ENT_CONTAINS_LT;
            }
            (*ent).flags |= XML_ENT_CHECKED_LT;
        }
        if (*ent).flags & XML_ENT_CONTAINS_LT != 0 {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_LT_IN_ATTRIBUTE,
                b"'<' in entity '%s' is not allowed in attributes values\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
            );
        }
    } else {
        match (*ent).etype as ::core::ffi::c_uint {
            4 | 5 => {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_ENTITY_IS_PARAMETER,
                    b"Attempt to reference the parameter entity '%s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    name,
                );
            }
            _ => {}
        }
    }
    return ent;
}
unsafe extern "C" fn xmlParseStringEntityRef(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> xmlEntityPtr {
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ptr: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut cur: xmlChar = 0;
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    if str.is_null() || (*str).is_null() {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    ptr = *str;
    cur = *ptr;
    if cur as ::core::ffi::c_int != '&' as i32 {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    ptr = ptr.offset(1);
    name = xmlParseStringName(ctxt, &raw mut ptr);
    if name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"xmlParseStringEntityRef: no name\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        *str = ptr;
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    if *ptr as ::core::ffi::c_int != ';' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_ENTITYREF_SEMICOL_MISSING,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
        *str = ptr;
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    ptr = ptr.offset(1);
    if (*ctxt).options & XML_PARSE_OLDSAX as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        ent = xmlGetPredefinedEntity(name);
        if !ent.is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
            *str = ptr;
            return ent;
        }
    }
    if !(*ctxt).sax.is_null() {
        if (*(*ctxt).sax).getEntity.is_some() {
            ent = (*(*ctxt).sax).getEntity.expect("non-null function pointer")(
                (*ctxt).userData,
                name,
            );
        }
        if ent.is_null() && (*ctxt).options & XML_PARSE_OLDSAX as ::core::ffi::c_int != 0 {
            ent = xmlGetPredefinedEntity(name);
        }
        if ent.is_null() && (*ctxt).userData == ctxt as *mut ::core::ffi::c_void {
            ent = xmlSAX2GetEntity(ctxt as *mut ::core::ffi::c_void, name);
        }
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    if ent.is_null() {
        if (*ctxt).standalone == 1 as ::core::ffi::c_int
            || (*ctxt).hasExternalSubset == 0 as ::core::ffi::c_int
                && (*ctxt).hasPErefs == 0 as ::core::ffi::c_int
        {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_UNDECLARED_ENTITY,
                b"Entity '%s' not defined\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
        } else {
            xmlErrMsgStr(
                ctxt,
                XML_WAR_UNDECLARED_ENTITY,
                b"Entity '%s' not defined\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
        }
    } else if (*ent).etype as ::core::ffi::c_uint
        == XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_UNPARSED_ENTITY,
            b"Entity reference to unparsed entity %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
    } else if (*ctxt).instate as ::core::ffi::c_int
        == XML_PARSER_ATTRIBUTE_VALUE as ::core::ffi::c_int
        && (*ent).etype as ::core::ffi::c_uint
            == XML_EXTERNAL_GENERAL_PARSED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_ENTITY_IS_EXTERNAL,
            b"Attribute references external entity '%s'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
    } else if (*ctxt).instate as ::core::ffi::c_int
        == XML_PARSER_ATTRIBUTE_VALUE as ::core::ffi::c_int
        && (*ent).etype as ::core::ffi::c_uint
            != XML_INTERNAL_PREDEFINED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*ent).flags & XML_ENT_CHECKED_LT == 0 as ::core::ffi::c_int {
            if !(*ent).content.is_null()
                && !xmlStrchr((*ent).content, '<' as i32 as xmlChar).is_null()
            {
                (*ent).flags |= XML_ENT_CONTAINS_LT;
            }
            (*ent).flags |= XML_ENT_CHECKED_LT;
        }
        if (*ent).flags & XML_ENT_CONTAINS_LT != 0 {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_LT_IN_ATTRIBUTE,
                b"'<' in entity '%s' is not allowed in attributes values\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
            );
        }
    } else {
        match (*ent).etype as ::core::ffi::c_uint {
            4 | 5 => {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_ENTITY_IS_PARAMETER,
                    b"Attempt to reference the parameter entity '%s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    name,
                );
            }
            _ => {}
        }
    }
    xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
    *str = ptr;
    return ent;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParsePEReference(mut ctxt: xmlParserCtxtPtr) {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut entity: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '%' as i32 {
        return;
    }
    xmlNextChar(ctxt);
    name = xmlParseName(ctxt);
    if name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_PEREF_NO_NAME,
            b"PEReference: no name\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"PEReference: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != ';' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_PEREF_SEMICOL_MISSING,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    xmlNextChar(ctxt);
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).getParameterEntity.is_some() {
        entity = (*(*ctxt).sax)
            .getParameterEntity
            .expect("non-null function pointer")((*ctxt).userData, name);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return;
    }
    if entity.is_null() {
        if (*ctxt).standalone == 1 as ::core::ffi::c_int
            || (*ctxt).hasExternalSubset == 0 as ::core::ffi::c_int
                && (*ctxt).hasPErefs == 0 as ::core::ffi::c_int
        {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_UNDECLARED_ENTITY,
                b"PEReference: %%%s; not found\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
        } else {
            if (*ctxt).validate != 0 && (*ctxt).vctxt.error.is_some() {
                xmlValidityError(
                    ctxt,
                    XML_WAR_UNDECLARED_ENTITY,
                    b"PEReference: %%%s; not found\n\0" as *const u8 as *const ::core::ffi::c_char,
                    name,
                    ::core::ptr::null::<xmlChar>(),
                );
            } else {
                xmlWarningMsg(
                    ctxt,
                    XML_WAR_UNDECLARED_ENTITY,
                    b"PEReference: %%%s; not found\n\0" as *const u8 as *const ::core::ffi::c_char,
                    name,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            (*ctxt).valid = 0 as ::core::ffi::c_int;
        }
    } else if (*entity).etype as ::core::ffi::c_uint
        != XML_INTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*entity).etype as ::core::ffi::c_uint
            != XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlWarningMsg(
            ctxt,
            XML_WAR_UNDECLARED_ENTITY,
            b"Internal: %%%s; is not a parameter entity\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
    } else {
        let mut parentConsumed: ::core::ffi::c_ulong = 0;
        let mut oldEnt: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
        if (*entity).etype as ::core::ffi::c_uint
            == XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*ctxt).options & XML_PARSE_NOENT as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*ctxt).options & XML_PARSE_DTDVALID as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*ctxt).options & XML_PARSE_DTDLOAD as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*ctxt).options & XML_PARSE_DTDATTR as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*ctxt).replaceEntities == 0 as ::core::ffi::c_int
            && (*ctxt).validate == 0 as ::core::ffi::c_int
        {
            return;
        }
        if (*entity).flags & XML_ENT_EXPANDING != 0 {
            xmlFatalErr(
                ctxt,
                XML_ERR_ENTITY_LOOP,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlHaltParser(ctxt);
            return;
        }
        parentConsumed = (*(*ctxt).input).parentConsumed;
        oldEnt = (*(*ctxt).input).entity;
        if oldEnt.is_null()
            || (*oldEnt).etype as ::core::ffi::c_uint
                == XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*oldEnt).flags & XML_ENT_PARSED == 0 as ::core::ffi::c_int
        {
            xmlSaturatedAdd(&raw mut parentConsumed, (*(*ctxt).input).consumed);
            xmlSaturatedAddSizeT(
                &raw mut parentConsumed,
                (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                    as ::core::ffi::c_ulong,
            );
        }
        input = xmlNewEntityInputStream(ctxt, entity);
        if xmlPushInput(ctxt, input) < 0 as ::core::ffi::c_int {
            xmlFreeInputStream(input);
            return;
        }
        (*entity).flags |= XML_ENT_EXPANDING;
        (*input).parentConsumed = parentConsumed;
        if (*entity).etype as ::core::ffi::c_uint
            == XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xmlDetectEncoding(ctxt);
            if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '<' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '?' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'x' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'm' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'l' as i32
                && (*(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 0x20 as ::core::ffi::c_int
                    || 0x9 as ::core::ffi::c_int
                        <= *(*(*ctxt).input)
                            .cur
                            .offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                        && *(*(*ctxt).input)
                            .cur
                            .offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            <= 0xa as ::core::ffi::c_int
                    || *(*(*ctxt).input)
                        .cur
                        .offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0xd as ::core::ffi::c_int)
            {
                xmlParseTextDecl(ctxt);
            }
        }
    }
    (*ctxt).hasPErefs = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlLoadEntityContent(
    mut ctxt: xmlParserCtxtPtr,
    mut entity: xmlEntityPtr,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut oldinput: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut oldinputTab: *mut xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInputPtr>();
    let mut oldencoding: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut content: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut length: size_t = 0;
    let mut i: size_t = 0;
    let mut oldinputNr: ::core::ffi::c_int = 0;
    let mut oldinputMax: ::core::ffi::c_int = 0;
    let mut oldprogressive: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut res: ::core::ffi::c_int = 0;
    if ctxt.is_null()
        || entity.is_null()
        || (*entity).etype as ::core::ffi::c_uint
            != XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*entity).etype as ::core::ffi::c_uint
                != XML_EXTERNAL_GENERAL_PARSED_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
        || !(*entity).content.is_null()
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"xmlLoadEntityContent parameter error\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if *__xmlParserDebugEntities() != 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Reading %s entity content input\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*entity).name,
        );
    }
    input = xmlLoadExternalEntity(
        (*entity).URI as *mut ::core::ffi::c_char,
        (*entity).ExternalID as *mut ::core::ffi::c_char,
        ctxt,
    );
    if input.is_null() {
        xmlFatalErr(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"xmlLoadEntityContent input error\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    oldinput = (*ctxt).input;
    oldinputNr = (*ctxt).inputNr;
    oldinputMax = (*ctxt).inputMax;
    oldinputTab = (*ctxt).inputTab;
    oldencoding = (*ctxt).encoding;
    oldprogressive = (*ctxt).progressive;
    (*ctxt).input = ::core::ptr::null_mut::<xmlParserInput>();
    (*ctxt).inputNr = 0 as ::core::ffi::c_int;
    (*ctxt).inputMax = 1 as ::core::ffi::c_int;
    (*ctxt).encoding = ::core::ptr::null::<xmlChar>();
    (*ctxt).progressive = 0 as ::core::ffi::c_int;
    (*ctxt).inputTab = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<
        xmlParserInputPtr,
    >() as size_t) as *mut xmlParserInputPtr;
    if (*ctxt).inputTab.is_null() {
        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        xmlFreeInputStream(input);
    } else {
        xmlBufResetInput((*(*input).buf).buffer, input);
        inputPush(ctxt, input);
        xmlDetectEncoding(ctxt);
        if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'x' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'm' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'l' as i32
            && (*(*(*ctxt).input)
                .cur
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int
                    <= *(*(*ctxt).input)
                        .cur
                        .offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                    && *(*(*ctxt).input)
                        .cur
                        .offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= 0xa as ::core::ffi::c_int
                || *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 0xd as ::core::ffi::c_int)
        {
            xmlParseTextDecl(ctxt);
            if xmlStrEqual(
                (*ctxt).version,
                b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
                && xmlStrEqual(
                    (*(*ctxt).input).version,
                    b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                ) == 0
            {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_VERSION_MISMATCH,
                    b"Version mismatch between document and entity\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if !((*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int) {
            length = (*input).cur.offset_from((*input).base) as ::core::ffi::c_long as size_t;
            xmlBufShrink((*(*input).buf).buffer, length);
            xmlSaturatedAdd(
                &raw mut (*ctxt).sizeentities,
                length as ::core::ffi::c_ulong,
            );
            loop {
                res = xmlParserInputBufferGrow((*input).buf, 4096 as ::core::ffi::c_int);
                if !(res > 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            xmlBufResetInput((*(*input).buf).buffer, input);
            if res < 0 as ::core::ffi::c_int {
                xmlFatalErr(
                    ctxt,
                    (*(*input).buf).error as xmlParserErrors,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            } else {
                length = xmlBufUse((*(*input).buf).buffer);
                content = xmlBufDetach((*(*input).buf).buffer);
                if length > INT_MAX as size_t {
                    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                } else {
                    i = 0 as size_t;
                    loop {
                        if !(i < length) {
                            current_block = 2543120759711851213;
                            break;
                        }
                        let mut clen: ::core::ffi::c_int =
                            length.wrapping_sub(i) as ::core::ffi::c_int;
                        let mut c: ::core::ffi::c_int =
                            xmlGetUTF8Char(content.offset(i as isize), &raw mut clen);
                        if c < 0 as ::core::ffi::c_int
                            || (if c < 0x100 as ::core::ffi::c_int {
                                (0x9 as ::core::ffi::c_int <= c && c <= 0xa as ::core::ffi::c_int
                                    || c == 0xd as ::core::ffi::c_int
                                    || 0x20 as ::core::ffi::c_int <= c)
                                    as ::core::ffi::c_int
                            } else {
                                (0x100 as ::core::ffi::c_int <= c
                                    && c <= 0xd7ff as ::core::ffi::c_int
                                    || 0xe000 as ::core::ffi::c_int <= c
                                        && c <= 0xfffd as ::core::ffi::c_int
                                    || 0x10000 as ::core::ffi::c_int <= c
                                        && c <= 0x10ffff as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                            }) == 0
                        {
                            xmlFatalErrMsgInt(
                                ctxt,
                                XML_ERR_INVALID_CHAR,
                                b"xmlLoadEntityContent: invalid char value %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                *content.offset(i as isize) as ::core::ffi::c_int,
                            );
                            current_block = 4582019160478557585;
                            break;
                        } else {
                            i = (i as ::core::ffi::c_ulong)
                                .wrapping_add(clen as ::core::ffi::c_ulong)
                                as size_t as size_t;
                        }
                    }
                    match current_block {
                        4582019160478557585 => {}
                        _ => {
                            xmlSaturatedAdd(
                                &raw mut (*ctxt).sizeentities,
                                length as ::core::ffi::c_ulong,
                            );
                            (*entity).content = content;
                            (*entity).length = length as ::core::ffi::c_int;
                            content = ::core::ptr::null_mut::<xmlChar>();
                            ret = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    while (*ctxt).inputNr > 0 as ::core::ffi::c_int {
        xmlFreeInputStream(inputPop(ctxt));
    }
    xmlFree.expect("non-null function pointer")((*ctxt).inputTab as *mut ::core::ffi::c_void);
    xmlFree.expect("non-null function pointer")(
        (*ctxt).encoding as *mut xmlChar as *mut ::core::ffi::c_void,
    );
    (*ctxt).input = oldinput;
    (*ctxt).inputNr = oldinputNr;
    (*ctxt).inputMax = oldinputMax;
    (*ctxt).inputTab = oldinputTab;
    (*ctxt).encoding = oldencoding;
    (*ctxt).progressive = oldprogressive;
    xmlFree.expect("non-null function pointer")(content as *mut ::core::ffi::c_void);
    return ret;
}
unsafe extern "C" fn xmlParseStringPEReference(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *mut *const xmlChar,
) -> xmlEntityPtr {
    let mut ptr: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut cur: xmlChar = 0;
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut entity: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    if str.is_null() || (*str).is_null() {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    ptr = *str;
    cur = *ptr;
    if cur as ::core::ffi::c_int != '%' as i32 {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    ptr = ptr.offset(1);
    name = xmlParseStringName(ctxt, &raw mut ptr);
    if name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"xmlParseStringPEReference: no name\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        *str = ptr;
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    cur = *ptr;
    if cur as ::core::ffi::c_int != ';' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_ENTITYREF_SEMICOL_MISSING,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
        *str = ptr;
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    ptr = ptr.offset(1);
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).getParameterEntity.is_some() {
        entity = (*(*ctxt).sax)
            .getParameterEntity
            .expect("non-null function pointer")((*ctxt).userData, name);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
        *str = ptr;
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    if entity.is_null() {
        if (*ctxt).standalone == 1 as ::core::ffi::c_int
            || (*ctxt).hasExternalSubset == 0 as ::core::ffi::c_int
                && (*ctxt).hasPErefs == 0 as ::core::ffi::c_int
        {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_UNDECLARED_ENTITY,
                b"PEReference: %%%s; not found\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
        } else {
            xmlWarningMsg(
                ctxt,
                XML_WAR_UNDECLARED_ENTITY,
                b"PEReference: %%%s; not found\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
                ::core::ptr::null::<xmlChar>(),
            );
            (*ctxt).valid = 0 as ::core::ffi::c_int;
        }
    } else if (*entity).etype as ::core::ffi::c_uint
        != XML_INTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*entity).etype as ::core::ffi::c_uint
            != XML_EXTERNAL_PARAMETER_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlWarningMsg(
            ctxt,
            XML_WAR_UNDECLARED_ENTITY,
            b"%%%s; is not a parameter entity\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
    }
    (*ctxt).hasPErefs = 1 as ::core::ffi::c_int;
    xmlFree.expect("non-null function pointer")(name as *mut ::core::ffi::c_void);
    *str = ptr;
    return entity;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseDocTypeDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ExternalID: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut URI: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(9 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 9 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    xmlSkipBlankChars(ctxt);
    name = xmlParseName(ctxt);
    if name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"xmlParseDocTypeDecl : no DOCTYPE name !\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*ctxt).intSubName = name;
    xmlSkipBlankChars(ctxt);
    URI = xmlParseExternalID(ctxt, &raw mut ExternalID, 1 as ::core::ffi::c_int);
    if !URI.is_null() || !ExternalID.is_null() {
        (*ctxt).hasExternalSubset = 1 as ::core::ffi::c_int;
    }
    (*ctxt).extSubURI = URI;
    (*ctxt).extSubSystem = ExternalID;
    xmlSkipBlankChars(ctxt);
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).internalSubset.is_some() && (*ctxt).disableSAX == 0
    {
        (*(*ctxt).sax)
            .internalSubset
            .expect("non-null function pointer")((*ctxt).userData, name, ExternalID, URI);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return;
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '[' as i32 {
        return;
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_DOCTYPE_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    xmlNextChar(ctxt);
}
unsafe extern "C" fn xmlParseInternalSubset(mut ctxt: xmlParserCtxtPtr) {
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '[' as i32 {
        let mut baseInputNr: ::core::ffi::c_int = (*ctxt).inputNr;
        (*ctxt).instate = XML_PARSER_DTD;
        xmlNextChar(ctxt);
        xmlSkipBlankChars(ctxt);
        while (*(*(*ctxt).input).cur as ::core::ffi::c_int != ']' as i32
            || (*ctxt).inputNr > baseInputNr)
            && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
        {
            if (*ctxt).inputNr > 1 as ::core::ffi::c_int
                && !(*(*ctxt).input).filename.is_null()
                && *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '!' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '[' as i32
            {
                xmlParseConditionalSections(ctxt);
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
                && (*(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '!' as i32
                    || *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '?' as i32)
            {
                xmlParseMarkupDecl(ctxt);
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '%' as i32 {
                xmlParsePEReference(ctxt);
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlParseInternalSubset: error detected in Markup declaration\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                xmlHaltParser(ctxt);
                return;
            }
            xmlSkipBlankChars(ctxt);
            if ((*ctxt).progressive == 0 as ::core::ffi::c_int
                || (*ctxt).inputNr > 1 as ::core::ffi::c_int)
                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                    > (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
            {
                xmlParserShrink(ctxt);
            }
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == ']' as i32 {
            xmlNextChar(ctxt);
            xmlSkipBlankChars(ctxt);
        }
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_DOCTYPE_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    xmlNextChar(ctxt);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseAttribute(
    mut ctxt: xmlParserCtxtPtr,
    mut value: *mut *mut xmlChar,
) -> *const xmlChar {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut val: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    *value = ::core::ptr::null_mut::<xmlChar>();
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    name = xmlParseName(ctxt);
    if name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"error parsing attribute name\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<xmlChar>();
    }
    xmlSkipBlankChars(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '=' as i32 {
        xmlNextChar(ctxt);
        xmlSkipBlankChars(ctxt);
        val = xmlParseAttValue(ctxt);
        (*ctxt).instate = XML_PARSER_CONTENT;
    } else {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_ATTRIBUTE_WITHOUT_VALUE,
            b"Specification mandates value for attribute %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
        return name;
    }
    if (*ctxt).pedantic != 0
        && xmlStrEqual(
            name,
            b"xml:lang\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ) != 0
    {
        if xmlCheckLanguageID(val) == 0 {
            xmlWarningMsg(
                ctxt,
                XML_WAR_LANG_VALUE,
                b"Malformed value for xml:lang : %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                val,
                ::core::ptr::null::<xmlChar>(),
            );
        }
    }
    if xmlStrEqual(
        name,
        b"xml:space\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) != 0
    {
        if xmlStrEqual(
            val,
            b"default\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ) != 0
        {
            *(*ctxt).space = 0 as ::core::ffi::c_int;
        } else if xmlStrEqual(
            val,
            b"preserve\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ) != 0
        {
            *(*ctxt).space = 1 as ::core::ffi::c_int;
        } else {
            xmlWarningMsg(
                ctxt,
                XML_WAR_SPACE_VALUE,
                b"Invalid value \"%s\" for xml:space : \"default\" or \"preserve\" expected\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                val,
                ::core::ptr::null::<xmlChar>(),
            );
        }
    }
    *value = val;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseStartTag(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut current_block: u64;
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut attname: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut attvalue: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut atts: *mut *const xmlChar = (*ctxt).atts;
    let mut nbatts: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut maxatts: ::core::ffi::c_int = (*ctxt).maxatts;
    let mut i: ::core::ffi::c_int = 0;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32 {
        return ::core::ptr::null::<xmlChar>();
    }
    (*(*ctxt).input).col += 1;
    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    name = xmlParseName(ctxt);
    if name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"xmlParseStartTag: invalid element name\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<xmlChar>();
    }
    xmlSkipBlankChars(ctxt);
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    while *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32
        && (*(*(*ctxt).input).cur as ::core::ffi::c_int != '/' as i32
            || *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '>' as i32)
        && (0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
            && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
            || 0x20 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int)
        && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
    {
        attname = xmlParseAttribute(ctxt, &raw mut attvalue);
        if attname.is_null() {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"xmlParseStartTag: problem parsing attributes\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            break;
        } else {
            if !attvalue.is_null() {
                i = 0 as ::core::ffi::c_int;
                loop {
                    if !(i < nbatts) {
                        current_block = 11298138898191919651;
                        break;
                    }
                    if xmlStrEqual(*atts.offset(i as isize), attname) != 0 {
                        xmlErrAttributeDup(ctxt, ::core::ptr::null::<xmlChar>(), attname);
                        xmlFree.expect("non-null function pointer")(
                            attvalue as *mut ::core::ffi::c_void,
                        );
                        current_block = 841351632615576294;
                        break;
                    } else {
                        i += 2 as ::core::ffi::c_int;
                    }
                }
                match current_block {
                    841351632615576294 => {}
                    _ => {
                        if atts.is_null() {
                            maxatts = 22 as ::core::ffi::c_int;
                            atts = xmlMalloc.expect("non-null function pointer")(
                                (maxatts as size_t)
                                    .wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as size_t),
                            ) as *mut *const xmlChar;
                            if atts.is_null() {
                                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                                if !attvalue.is_null() {
                                    xmlFree.expect("non-null function pointer")(
                                        attvalue as *mut ::core::ffi::c_void,
                                    );
                                }
                                current_block = 841351632615576294;
                            } else {
                                (*ctxt).atts = atts;
                                (*ctxt).maxatts = maxatts;
                                current_block = 9007357115414505193;
                            }
                        } else if nbatts + 4 as ::core::ffi::c_int > maxatts {
                            let mut n: *mut *const xmlChar =
                                ::core::ptr::null_mut::<*const xmlChar>();
                            maxatts *= 2 as ::core::ffi::c_int;
                            n = xmlRealloc.expect("non-null function pointer")(
                                atts as *mut ::core::ffi::c_void,
                                (maxatts as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<*const xmlChar>() as size_t
                                    ),
                            ) as *mut *const xmlChar;
                            if n.is_null() {
                                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                                if !attvalue.is_null() {
                                    xmlFree.expect("non-null function pointer")(
                                        attvalue as *mut ::core::ffi::c_void,
                                    );
                                }
                                current_block = 841351632615576294;
                            } else {
                                atts = n;
                                (*ctxt).atts = atts;
                                (*ctxt).maxatts = maxatts;
                                current_block = 9007357115414505193;
                            }
                        } else {
                            current_block = 9007357115414505193;
                        }
                        match current_block {
                            841351632615576294 => {}
                            _ => {
                                let fresh24 = nbatts;
                                nbatts = nbatts + 1;
                                let ref mut fresh25 = *atts.offset(fresh24 as isize);
                                *fresh25 = attname;
                                let fresh26 = nbatts;
                                nbatts = nbatts + 1;
                                let ref mut fresh27 = *atts.offset(fresh26 as isize);
                                *fresh27 = attvalue;
                                let ref mut fresh28 = *atts.offset(nbatts as isize);
                                *fresh28 = ::core::ptr::null::<xmlChar>();
                                let ref mut fresh29 =
                                    *atts.offset((nbatts + 1 as ::core::ffi::c_int) as isize);
                                *fresh29 = ::core::ptr::null::<xmlChar>();
                            }
                        }
                    }
                }
            } else if !attvalue.is_null() {
                xmlFree.expect("non-null function pointer")(attvalue as *mut ::core::ffi::c_void);
            }
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '>' as i32
                || *(*(*ctxt).input).cur as ::core::ffi::c_int == '/' as i32
                    && *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '>' as i32
            {
                break;
            }
            if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"attributes construct error\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if ((*ctxt).progressive == 0 as ::core::ffi::c_int
                || (*ctxt).inputNr > 1 as ::core::ffi::c_int)
                && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                    > (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
                && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
            {
                xmlParserShrink(ctxt);
            }
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
        }
    }
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).startElement.is_some() && (*ctxt).disableSAX == 0 {
        if nbatts > 0 as ::core::ffi::c_int {
            (*(*ctxt).sax)
                .startElement
                .expect("non-null function pointer")((*ctxt).userData, name, atts);
        } else {
            (*(*ctxt).sax)
                .startElement
                .expect("non-null function pointer")(
                (*ctxt).userData,
                name,
                ::core::ptr::null_mut::<*const xmlChar>(),
            );
        }
    }
    if !atts.is_null() {
        i = 1 as ::core::ffi::c_int;
        while i < nbatts {
            if !(*atts.offset(i as isize)).is_null() {
                xmlFree.expect("non-null function pointer")(
                    *atts.offset(i as isize) as *mut xmlChar as *mut ::core::ffi::c_void,
                );
            }
            i += 2 as ::core::ffi::c_int;
        }
    }
    return name;
}
unsafe extern "C" fn xmlParseEndTag1(mut ctxt: xmlParserCtxtPtr, mut line: ::core::ffi::c_int) {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '/' as i32
    {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_LTSLASH_REQUIRED,
            b"xmlParseEndTag: '</' not found\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(2 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    name = xmlParseNameAndCompare(ctxt, (*ctxt).name);
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    xmlSkipBlankChars(ctxt);
    if !(0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
        && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
        || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
        || 0x20 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int)
        || *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_GT_REQUIRED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else {
        (*(*ctxt).input).col += 1;
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
    }
    if name != 1 as ::core::ffi::c_int as *mut xmlChar as *const xmlChar {
        if name.is_null() {
            name = b"unparsable\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar;
        }
        xmlFatalErrMsgStrIntStr(
            ctxt,
            XML_ERR_TAG_NAME_MISMATCH,
            b"Opening and ending tag mismatch: %s line %d and %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*ctxt).name,
            line,
            name,
        );
    }
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endElement.is_some() && (*ctxt).disableSAX == 0 {
        (*(*ctxt).sax)
            .endElement
            .expect("non-null function pointer")((*ctxt).userData, (*ctxt).name);
    }
    namePop(ctxt);
    spacePop(ctxt);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEndTag(mut ctxt: xmlParserCtxtPtr) {
    xmlParseEndTag1(ctxt, 0 as ::core::ffi::c_int);
}
unsafe extern "C" fn xmlParseQNameHashed(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *mut xmlHashedString,
) -> xmlHashedString {
    let mut l: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut p: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut start: ::core::ffi::c_int = 0;
    let mut isNCName: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    l.name = ::core::ptr::null::<xmlChar>();
    p.name = ::core::ptr::null::<xmlChar>();
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return l;
    }
    start = (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
        as ::core::ffi::c_int;
    l = xmlParseNCName(ctxt);
    if !l.name.is_null() {
        isNCName = 1 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == ':' as i32 {
            xmlNextChar(ctxt);
            p = l;
            l = xmlParseNCName(ctxt);
        }
    }
    if l.name.is_null() || *(*(*ctxt).input).cur as ::core::ffi::c_int == ':' as i32 {
        let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        l.name = ::core::ptr::null::<xmlChar>();
        p.name = ::core::ptr::null::<xmlChar>();
        if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
            return l;
        }
        if isNCName == 0 as ::core::ffi::c_int
            && *(*(*ctxt).input).cur as ::core::ffi::c_int != ':' as i32
        {
            return l;
        }
        tmp = xmlParseNmtoken(ctxt);
        if !tmp.is_null() {
            xmlFree.expect("non-null function pointer")(tmp as *mut ::core::ffi::c_void);
        }
        if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
            return l;
        }
        l = xmlDictLookupHashed(
            (*ctxt).dict,
            (*(*ctxt).input).base.offset(start as isize),
            (*(*ctxt).input)
                .cur
                .offset_from((*(*ctxt).input).base.offset(start as isize))
                as ::core::ffi::c_long as ::core::ffi::c_int,
        );
        xmlNsErr(
            ctxt,
            XML_NS_ERR_QNAME,
            b"Failed to parse QName '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            l.name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
    }
    *prefix = p;
    return l;
}
unsafe extern "C" fn xmlParseQName(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *mut *const xmlChar,
) -> *const xmlChar {
    let mut n: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut p: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    n = xmlParseQNameHashed(ctxt, &raw mut p);
    if n.name.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    *prefix = p.name;
    return n.name;
}
unsafe extern "C" fn xmlParseQNameAndCompare(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut prefix: *const xmlChar,
) -> *const xmlChar {
    let mut cmp: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut prefix2: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if prefix.is_null() {
        return xmlParseNameAndCompare(ctxt, name);
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    in_0 = (*(*ctxt).input).cur;
    cmp = prefix;
    while *in_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && *in_0 as ::core::ffi::c_int == *cmp as ::core::ffi::c_int
    {
        in_0 = in_0.offset(1);
        cmp = cmp.offset(1);
    }
    if *cmp as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && *in_0 as ::core::ffi::c_int == ':' as i32
    {
        in_0 = in_0.offset(1);
        cmp = name;
        while *in_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int == *cmp as ::core::ffi::c_int
        {
            in_0 = in_0.offset(1);
            cmp = cmp.offset(1);
        }
        if *cmp as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*in_0 as ::core::ffi::c_int == '>' as i32
                || (*in_0 as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                    || 0x9 as ::core::ffi::c_int <= *in_0 as ::core::ffi::c_int
                        && *in_0 as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                    || *in_0 as ::core::ffi::c_int == 0xd as ::core::ffi::c_int))
        {
            (*(*ctxt).input).col = ((*(*ctxt).input).col as ::core::ffi::c_long
                + in_0.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                as ::core::ffi::c_int;
            (*(*ctxt).input).cur = in_0;
            return 1 as ::core::ffi::c_int as *const xmlChar;
        }
    }
    ret = xmlParseQName(ctxt, &raw mut prefix2);
    if ret.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if ret == name && prefix == prefix2 {
        return 1 as ::core::ffi::c_int as *const xmlChar;
    }
    return ret;
}
unsafe extern "C" fn xmlParseAttValueInternal(
    mut ctxt: xmlParserCtxtPtr,
    mut len: *mut ::core::ffi::c_int,
    mut alloc: *mut ::core::ffi::c_int,
    mut normalize: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut limit: xmlChar = 0 as xmlChar;
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut start: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut end: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut last: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut line: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_HUGE_LENGTH
        } else {
            XML_MAX_TEXT_LENGTH
        };
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    in_0 = (*(*ctxt).input).cur as *mut xmlChar;
    line = (*(*ctxt).input).line;
    col = (*(*ctxt).input).col;
    if *in_0 as ::core::ffi::c_int != '"' as i32 && *in_0 as ::core::ffi::c_int != '\'' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_ATTRIBUTE_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    (*ctxt).instate = XML_PARSER_ATTRIBUTE_VALUE;
    let fresh30 = in_0;
    in_0 = in_0.offset(1);
    limit = *fresh30;
    col += 1;
    end = (*(*ctxt).input).end;
    start = in_0;
    if in_0 >= end {
        let mut oldbase: *const xmlChar = (*(*ctxt).input).base;
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if oldbase != (*(*ctxt).input).base {
            let mut delta: ptrdiff_t = (*(*ctxt).input).base.offset_from(oldbase) as ptrdiff_t;
            start = start.offset(delta as isize);
            in_0 = in_0.offset(delta as isize);
        }
        end = (*(*ctxt).input).end;
    }
    if normalize != 0 {
        while in_0 < end
            && *in_0 as ::core::ffi::c_int != limit as ::core::ffi::c_int
            && (*in_0 as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0x9 as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
        {
            if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                line += 1;
                col = 1 as ::core::ffi::c_int;
            } else {
                col += 1;
            }
            in_0 = in_0.offset(1);
            start = in_0;
            if in_0 >= end {
                let mut oldbase_0: *const xmlChar = (*(*ctxt).input).base;
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    return ::core::ptr::null_mut::<xmlChar>();
                }
                if oldbase_0 != (*(*ctxt).input).base {
                    let mut delta_0: ptrdiff_t =
                        (*(*ctxt).input).base.offset_from(oldbase_0) as ptrdiff_t;
                    start = start.offset(delta_0 as isize);
                    in_0 = in_0.offset(delta_0 as isize);
                }
                end = (*(*ctxt).input).end;
                if in_0.offset_from(start) as ::core::ffi::c_long > maxLength as ::core::ffi::c_long
                {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ATTRIBUTE_NOT_FINISHED,
                        b"AttValue length too long\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return ::core::ptr::null_mut::<xmlChar>();
                }
            }
        }
        while in_0 < end
            && *in_0 as ::core::ffi::c_int != limit as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int <= 0x7f as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int != '&' as i32
            && *in_0 as ::core::ffi::c_int != '<' as i32
        {
            col += 1;
            let fresh31 = in_0;
            in_0 = in_0.offset(1);
            if *fresh31 as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                && *in_0 as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            {
                break;
            }
            if in_0 >= end {
                let mut oldbase_1: *const xmlChar = (*(*ctxt).input).base;
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    return ::core::ptr::null_mut::<xmlChar>();
                }
                if oldbase_1 != (*(*ctxt).input).base {
                    let mut delta_1: ptrdiff_t =
                        (*(*ctxt).input).base.offset_from(oldbase_1) as ptrdiff_t;
                    start = start.offset(delta_1 as isize);
                    in_0 = in_0.offset(delta_1 as isize);
                }
                end = (*(*ctxt).input).end;
                if in_0.offset_from(start) as ::core::ffi::c_long > maxLength as ::core::ffi::c_long
                {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ATTRIBUTE_NOT_FINISHED,
                        b"AttValue length too long\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return ::core::ptr::null_mut::<xmlChar>();
                }
            }
        }
        last = in_0;
        while *last.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == 0x20 as ::core::ffi::c_int
            && last > start
        {
            last = last.offset(-1);
        }
        while in_0 < end
            && *in_0 as ::core::ffi::c_int != limit as ::core::ffi::c_int
            && (*in_0 as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0x9 as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int
                || *in_0 as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
        {
            if *in_0 as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                line += 1;
                col = 1 as ::core::ffi::c_int;
            } else {
                col += 1;
            }
            in_0 = in_0.offset(1);
            if in_0 >= end {
                let mut oldbase_2: *const xmlChar = (*(*ctxt).input).base;
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    return ::core::ptr::null_mut::<xmlChar>();
                }
                if oldbase_2 != (*(*ctxt).input).base {
                    let mut delta_2: ptrdiff_t =
                        (*(*ctxt).input).base.offset_from(oldbase_2) as ptrdiff_t;
                    start = start.offset(delta_2 as isize);
                    in_0 = in_0.offset(delta_2 as isize);
                    last = last.offset(delta_2 as isize);
                }
                end = (*(*ctxt).input).end;
                if in_0.offset_from(start) as ::core::ffi::c_long > maxLength as ::core::ffi::c_long
                {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ATTRIBUTE_NOT_FINISHED,
                        b"AttValue length too long\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return ::core::ptr::null_mut::<xmlChar>();
                }
            }
        }
        if in_0.offset_from(start) as ::core::ffi::c_long > maxLength as ::core::ffi::c_long {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_ATTRIBUTE_NOT_FINISHED,
                b"AttValue length too long\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if *in_0 as ::core::ffi::c_int != limit as ::core::ffi::c_int {
            current_block = 1341405339194517839;
        } else {
            current_block = 5265702136860997526;
        }
    } else {
        while in_0 < end
            && *in_0 as ::core::ffi::c_int != limit as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int <= 0x7f as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int != '&' as i32
            && *in_0 as ::core::ffi::c_int != '<' as i32
        {
            in_0 = in_0.offset(1);
            col += 1;
            if in_0 >= end {
                let mut oldbase_3: *const xmlChar = (*(*ctxt).input).base;
                if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                    < INPUT_CHUNK as ::core::ffi::c_long
                {
                    xmlParserGrow(ctxt);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    return ::core::ptr::null_mut::<xmlChar>();
                }
                if oldbase_3 != (*(*ctxt).input).base {
                    let mut delta_3: ptrdiff_t =
                        (*(*ctxt).input).base.offset_from(oldbase_3) as ptrdiff_t;
                    start = start.offset(delta_3 as isize);
                    in_0 = in_0.offset(delta_3 as isize);
                }
                end = (*(*ctxt).input).end;
                if in_0.offset_from(start) as ::core::ffi::c_long > maxLength as ::core::ffi::c_long
                {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_ATTRIBUTE_NOT_FINISHED,
                        b"AttValue length too long\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return ::core::ptr::null_mut::<xmlChar>();
                }
            }
        }
        last = in_0;
        if in_0.offset_from(start) as ::core::ffi::c_long > maxLength as ::core::ffi::c_long {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_ATTRIBUTE_NOT_FINISHED,
                b"AttValue length too long\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if *in_0 as ::core::ffi::c_int != limit as ::core::ffi::c_int {
            current_block = 1341405339194517839;
        } else {
            current_block = 5265702136860997526;
        }
    }
    match current_block {
        1341405339194517839 => {
            if !alloc.is_null() {
                *alloc = 1 as ::core::ffi::c_int;
            }
            return xmlParseAttValueComplex(ctxt, len, normalize);
        }
        _ => {
            in_0 = in_0.offset(1);
            col += 1;
            if !len.is_null() {
                if !alloc.is_null() {
                    *alloc = 0 as ::core::ffi::c_int;
                }
                *len = last.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_int;
                ret = start as *mut xmlChar;
            } else {
                if !alloc.is_null() {
                    *alloc = 1 as ::core::ffi::c_int;
                }
                ret = xmlStrndup(
                    start,
                    last.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_int,
                );
            }
            (*(*ctxt).input).cur = in_0;
            (*(*ctxt).input).line = line;
            (*(*ctxt).input).col = col;
            return ret;
        }
    };
}
unsafe extern "C" fn xmlParseAttribute2(
    mut ctxt: xmlParserCtxtPtr,
    mut pref: *const xmlChar,
    mut elem: *const xmlChar,
    mut hprefix: *mut xmlHashedString,
    mut value: *mut *mut xmlChar,
    mut len: *mut ::core::ffi::c_int,
    mut alloc: *mut ::core::ffi::c_int,
) -> xmlHashedString {
    let mut hname: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut prefix: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut val: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut internal_val: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut normalize: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *value = ::core::ptr::null_mut::<xmlChar>();
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    hname = xmlParseQNameHashed(ctxt, hprefix);
    if hname.name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"error parsing attribute name\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return hname;
    }
    name = hname.name;
    if !(*hprefix).name.is_null() {
        prefix = (*hprefix).name;
    } else {
        prefix = ::core::ptr::null::<xmlChar>();
    }
    if !(*ctxt).attsSpecial.is_null() {
        let mut type_0: ::core::ffi::c_int = 0;
        type_0 = xmlHashQLookup2((*ctxt).attsSpecial, pref, elem, prefix, name) as ptrdiff_t
            as ::core::ffi::c_int;
        if type_0 != 0 as ::core::ffi::c_int {
            normalize = 1 as ::core::ffi::c_int;
        }
    }
    xmlSkipBlankChars(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '=' as i32 {
        xmlNextChar(ctxt);
        xmlSkipBlankChars(ctxt);
        val = xmlParseAttValueInternal(ctxt, len, alloc, normalize);
        if val.is_null() {
            hname.name = ::core::ptr::null::<xmlChar>();
            return hname;
        }
        if normalize != 0 {
            if *alloc != 0 {
                let mut val2: *const xmlChar = ::core::ptr::null::<xmlChar>();
                val2 = xmlAttrNormalizeSpace2(ctxt, val, len);
                if !val2.is_null() && val2 != val as *const xmlChar {
                    xmlFree.expect("non-null function pointer")(val as *mut ::core::ffi::c_void);
                    val = val2 as *mut xmlChar;
                }
            }
        }
        (*ctxt).instate = XML_PARSER_CONTENT;
    } else {
        xmlFatalErrMsgStr(
            ctxt,
            XML_ERR_ATTRIBUTE_WITHOUT_VALUE,
            b"Specification mandates value for attribute %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
        return hname;
    }
    if prefix == (*ctxt).str_xml {
        if (*ctxt).pedantic != 0
            && xmlStrEqual(
                name,
                b"lang\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
        {
            internal_val = xmlStrndup(val, *len);
            if xmlCheckLanguageID(internal_val) == 0 {
                xmlWarningMsg(
                    ctxt,
                    XML_WAR_LANG_VALUE,
                    b"Malformed value for xml:lang : %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    internal_val,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
        }
        if xmlStrEqual(
            name,
            b"space\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ) != 0
        {
            internal_val = xmlStrndup(val, *len);
            if xmlStrEqual(
                internal_val,
                b"default\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
            {
                *(*ctxt).space = 0 as ::core::ffi::c_int;
            } else if xmlStrEqual(
                internal_val,
                b"preserve\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) != 0
            {
                *(*ctxt).space = 1 as ::core::ffi::c_int;
            } else {
                xmlWarningMsg(
                    ctxt,
                    XML_WAR_SPACE_VALUE,
                    b"Invalid value \"%s\" for xml:space : \"default\" or \"preserve\" expected\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    internal_val,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
        }
        if !internal_val.is_null() {
            xmlFree.expect("non-null function pointer")(internal_val as *mut ::core::ffi::c_void);
        }
    }
    *value = val;
    return hname;
}
unsafe extern "C" fn xmlAttrHashInsert(
    mut ctxt: xmlParserCtxtPtr,
    mut size: ::core::ffi::c_uint,
    mut name: *const xmlChar,
    mut uri: *const xmlChar,
    mut hashValue: ::core::ffi::c_uint,
    mut aindex: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut table: *mut xmlAttrHashBucket = (*ctxt).attrHash;
    let mut bucket: *mut xmlAttrHashBucket = ::core::ptr::null_mut::<xmlAttrHashBucket>();
    let mut hindex: ::core::ffi::c_uint = 0;
    hindex = hashValue & size.wrapping_sub(1 as ::core::ffi::c_uint);
    bucket = table.offset(hindex as isize) as *mut xmlAttrHashBucket;
    while (*bucket).index >= 0 as ::core::ffi::c_int {
        let mut atts: *mut *const xmlChar =
            (*ctxt).atts.offset((*bucket).index as isize) as *mut *const xmlChar;
        if name == *atts.offset(0 as ::core::ffi::c_int as isize) {
            let mut nsIndex: ::core::ffi::c_int =
                *atts.offset(2 as ::core::ffi::c_int as isize) as ptrdiff_t as ::core::ffi::c_int;
            if if nsIndex == NS_INDEX_EMPTY {
                (uri == NULL as *const xmlChar) as ::core::ffi::c_int
            } else if nsIndex == NS_INDEX_XML {
                (uri == (*ctxt).str_xml) as ::core::ffi::c_int
            } else {
                (uri == *(*ctxt)
                    .nsTab
                    .offset((nsIndex * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    as ::core::ffi::c_int
            } != 0
            {
                return (*bucket).index;
            }
        }
        hindex = hindex.wrapping_add(1);
        bucket = bucket.offset(1);
        if hindex >= size {
            hindex = 0 as ::core::ffi::c_uint;
            bucket = table;
        }
    }
    (*bucket).index = aindex;
    return 2147483647 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParseStartTag2(
    mut ctxt: xmlParserCtxtPtr,
    mut pref: *mut *const xmlChar,
    mut URI: *mut *const xmlChar,
    mut nbNsPtr: *mut ::core::ffi::c_int,
) -> *const xmlChar {
    let mut current_block: u64;
    let mut hlocalname: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut hprefix: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut hattname: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut haprefix: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut localname: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut prefix: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut attname: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut aprefix: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut uri: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut attvalue: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut atts: *mut *const xmlChar = (*ctxt).atts;
    let mut attrHashSize: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut maxatts: ::core::ffi::c_int = (*ctxt).maxatts;
    let mut nratts: ::core::ffi::c_int = 0;
    let mut nbatts: ::core::ffi::c_int = 0;
    let mut nbdef: ::core::ffi::c_int = 0;
    let mut inputid: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut nbNs: ::core::ffi::c_int = 0;
    let mut nbTotalDef: ::core::ffi::c_int = 0;
    let mut attval: ::core::ffi::c_int = 0;
    let mut nsIndex: ::core::ffi::c_int = 0;
    let mut maxAtts: ::core::ffi::c_int = 0;
    let mut alloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32 {
        return ::core::ptr::null::<xmlChar>();
    }
    (*(*ctxt).input).col += 1;
    (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    inputid = (*(*ctxt).input).id;
    nbatts = 0 as ::core::ffi::c_int;
    nratts = 0 as ::core::ffi::c_int;
    nbdef = 0 as ::core::ffi::c_int;
    nbNs = 0 as ::core::ffi::c_int;
    nbTotalDef = 0 as ::core::ffi::c_int;
    attval = 0 as ::core::ffi::c_int;
    if xmlParserNsStartElement((*ctxt).nsdb) < 0 as ::core::ffi::c_int {
        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        return ::core::ptr::null::<xmlChar>();
    }
    hlocalname = xmlParseQNameHashed(ctxt, &raw mut hprefix);
    if hlocalname.name.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NAME_REQUIRED,
            b"StartTag: invalid element name\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<xmlChar>();
    }
    localname = hlocalname.name;
    prefix = hprefix.name;
    xmlSkipBlankChars(ctxt);
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    while *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32
        && (*(*(*ctxt).input).cur as ::core::ffi::c_int != '/' as i32
            || *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '>' as i32)
        && (0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
            && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
            || 0x20 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int)
        && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
    {
        let mut len: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        hattname = xmlParseAttribute2(
            ctxt,
            prefix,
            localname,
            &raw mut haprefix,
            &raw mut attvalue,
            &raw mut len,
            &raw mut alloc,
        );
        if hattname.name.is_null() {
            xmlFatalErr(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"xmlParseStartTag: problem parsing attributes\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            break;
        } else {
            if !attvalue.is_null() {
                attname = hattname.name;
                aprefix = haprefix.name;
                if len < 0 as ::core::ffi::c_int {
                    len = xmlStrlen(attvalue);
                }
                if attname == (*ctxt).str_xmlns && aprefix.is_null() {
                    let mut huri: xmlHashedString = xmlHashedString {
                        hashValue: 0,
                        name: ::core::ptr::null::<xmlChar>(),
                    };
                    let mut parsedUri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
                    huri = xmlDictLookupHashed((*ctxt).dict, attvalue, len);
                    uri = huri.name;
                    if uri.is_null() {
                        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                    } else {
                        if *uri as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            parsedUri = xmlParseURI(uri as *const ::core::ffi::c_char);
                            if parsedUri.is_null() {
                                xmlNsErr(
                                    ctxt,
                                    XML_WAR_NS_URI,
                                    b"xmlns: '%s' is not a valid URI\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    uri,
                                    ::core::ptr::null::<xmlChar>(),
                                    ::core::ptr::null::<xmlChar>(),
                                );
                            } else {
                                if (*parsedUri).scheme.is_null() {
                                    xmlNsWarn(
                                        ctxt,
                                        XML_WAR_NS_URI_RELATIVE,
                                        b"xmlns: URI %s is not absolute\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        uri,
                                        ::core::ptr::null::<xmlChar>(),
                                        ::core::ptr::null::<xmlChar>(),
                                    );
                                }
                                xmlFreeURI(parsedUri);
                            }
                            if uri == (*ctxt).str_xml_ns {
                                if attname != (*ctxt).str_xml {
                                    xmlNsErr(
                                        ctxt,
                                        XML_NS_ERR_XML_NAMESPACE,
                                        b"xml namespace URI cannot be the default namespace\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        ::core::ptr::null::<xmlChar>(),
                                        ::core::ptr::null::<xmlChar>(),
                                        ::core::ptr::null::<xmlChar>(),
                                    );
                                }
                                current_block = 17374685173522264211;
                            } else if len == 29 as ::core::ffi::c_int
                                && xmlStrEqual(
                                    uri,
                                    b"http://www.w3.org/2000/xmlns/\0" as *const u8
                                        as *const ::core::ffi::c_char
                                        as *mut xmlChar,
                                ) != 0
                            {
                                xmlNsErr(
                                    ctxt,
                                    XML_NS_ERR_XML_NAMESPACE,
                                    b"reuse of the xmlns namespace name is forbidden\n\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    ::core::ptr::null::<xmlChar>(),
                                    ::core::ptr::null::<xmlChar>(),
                                    ::core::ptr::null::<xmlChar>(),
                                );
                                current_block = 17374685173522264211;
                            } else {
                                current_block = 2516253395664191498;
                            }
                        } else {
                            current_block = 2516253395664191498;
                        }
                        match current_block {
                            17374685173522264211 => {}
                            _ => {
                                if xmlParserNsPush(
                                    ctxt,
                                    ::core::ptr::null::<xmlHashedString>(),
                                    &raw mut huri,
                                    NULL,
                                    0 as ::core::ffi::c_int,
                                ) > 0 as ::core::ffi::c_int
                                {
                                    nbNs += 1;
                                }
                            }
                        }
                    }
                } else if aprefix == (*ctxt).str_xmlns {
                    let mut huri_0: xmlHashedString = xmlHashedString {
                        hashValue: 0,
                        name: ::core::ptr::null::<xmlChar>(),
                    };
                    let mut parsedUri_0: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
                    huri_0 = xmlDictLookupHashed((*ctxt).dict, attvalue, len);
                    uri = huri_0.name;
                    if uri.is_null() {
                        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                    } else if attname == (*ctxt).str_xml {
                        if uri != (*ctxt).str_xml_ns {
                            xmlNsErr(
                                ctxt,
                                XML_NS_ERR_XML_NAMESPACE,
                                b"xml namespace prefix mapped to wrong URI\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                            );
                        }
                    } else if uri == (*ctxt).str_xml_ns {
                        if attname != (*ctxt).str_xml {
                            xmlNsErr(
                                ctxt,
                                XML_NS_ERR_XML_NAMESPACE,
                                b"xml namespace URI mapped to wrong prefix\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                            );
                        }
                    } else if attname == (*ctxt).str_xmlns {
                        xmlNsErr(
                            ctxt,
                            XML_NS_ERR_XML_NAMESPACE,
                            b"redefinition of the xmlns prefix is forbidden\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                        );
                    } else if len == 29 as ::core::ffi::c_int
                        && xmlStrEqual(
                            uri,
                            b"http://www.w3.org/2000/xmlns/\0" as *const u8
                                as *const ::core::ffi::c_char
                                as *mut xmlChar,
                        ) != 0
                    {
                        xmlNsErr(
                            ctxt,
                            XML_NS_ERR_XML_NAMESPACE,
                            b"reuse of the xmlns namespace name is forbidden\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                        );
                    } else if uri.is_null()
                        || *uri.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        xmlNsErr(
                            ctxt,
                            XML_NS_ERR_XML_NAMESPACE,
                            b"xmlns:%s: Empty XML namespace is not allowed\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            attname,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                        );
                    } else {
                        parsedUri_0 = xmlParseURI(uri as *const ::core::ffi::c_char);
                        if parsedUri_0.is_null() {
                            xmlNsErr(
                                ctxt,
                                XML_WAR_NS_URI,
                                b"xmlns:%s: '%s' is not a valid URI\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                attname,
                                uri,
                                ::core::ptr::null::<xmlChar>(),
                            );
                        } else {
                            if (*ctxt).pedantic != 0 && (*parsedUri_0).scheme.is_null() {
                                xmlNsWarn(
                                    ctxt,
                                    XML_WAR_NS_URI_RELATIVE,
                                    b"xmlns:%s: URI %s is not absolute\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    attname,
                                    uri,
                                    ::core::ptr::null::<xmlChar>(),
                                );
                            }
                            xmlFreeURI(parsedUri_0);
                        }
                        if xmlParserNsPush(
                            ctxt,
                            &raw mut hattname,
                            &raw mut huri_0,
                            NULL,
                            0 as ::core::ffi::c_int,
                        ) > 0 as ::core::ffi::c_int
                        {
                            nbNs += 1;
                        }
                    }
                } else {
                    if atts.is_null() || nbatts + 5 as ::core::ffi::c_int > maxatts {
                        if xmlCtxtGrowAttrs(ctxt, nbatts + 5 as ::core::ffi::c_int)
                            < 0 as ::core::ffi::c_int
                        {
                            current_block = 17374685173522264211;
                        } else {
                            maxatts = (*ctxt).maxatts;
                            atts = (*ctxt).atts;
                            current_block = 10393716428851982524;
                        }
                    } else {
                        current_block = 10393716428851982524;
                    }
                    match current_block {
                        17374685173522264211 => {}
                        _ => {
                            let fresh68 = nratts;
                            nratts = nratts + 1;
                            *(*ctxt).attallocs.offset(fresh68 as isize) = hattname.hashValue
                                & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint
                                | (alloc as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int;
                            let fresh69 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh70 = *atts.offset(fresh69 as isize);
                            *fresh70 = attname;
                            let fresh71 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh72 = *atts.offset(fresh71 as isize);
                            *fresh72 = aprefix;
                            let fresh73 = nbatts;
                            nbatts = nbatts + 1;
                            let ref mut fresh74 = *atts.offset(fresh73 as isize);
                            *fresh74 = haprefix.hashValue as size_t as *const xmlChar;
                            if alloc != 0 {
                                let fresh75 = nbatts;
                                nbatts = nbatts + 1;
                                let ref mut fresh76 = *atts.offset(fresh75 as isize);
                                *fresh76 = attvalue;
                                attvalue = attvalue.offset(len as isize);
                                let fresh77 = nbatts;
                                nbatts = nbatts + 1;
                                let ref mut fresh78 = *atts.offset(fresh77 as isize);
                                *fresh78 = attvalue;
                            } else {
                                let fresh79 = nbatts;
                                nbatts = nbatts + 1;
                                let ref mut fresh80 = *atts.offset(fresh79 as isize);
                                *fresh80 = attvalue.offset_from((*(*ctxt).input).base)
                                    as ::core::ffi::c_long
                                    as *mut ::core::ffi::c_void
                                    as *const xmlChar;
                                attvalue = attvalue.offset(len as isize);
                                let fresh81 = nbatts;
                                nbatts = nbatts + 1;
                                let ref mut fresh82 = *atts.offset(fresh81 as isize);
                                *fresh82 = attvalue.offset_from((*(*ctxt).input).base)
                                    as ::core::ffi::c_long
                                    as *mut ::core::ffi::c_void
                                    as *const xmlChar;
                            }
                            if alloc != 0 as ::core::ffi::c_int {
                                attval = 1 as ::core::ffi::c_int;
                            }
                            attvalue = ::core::ptr::null_mut::<xmlChar>();
                        }
                    }
                }
            }
            if !attvalue.is_null() && alloc != 0 as ::core::ffi::c_int {
                xmlFree.expect("non-null function pointer")(attvalue as *mut ::core::ffi::c_void);
                attvalue = ::core::ptr::null_mut::<xmlChar>();
            }
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
            if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                break;
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '>' as i32
                || *(*(*ctxt).input).cur as ::core::ffi::c_int == '/' as i32
                    && *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '>' as i32
            {
                break;
            }
            if xmlSkipBlankChars(ctxt) == 0 as ::core::ffi::c_int {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_SPACE_REQUIRED,
                    b"attributes construct error\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
                break;
            } else if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                as ::core::ffi::c_long)
                < INPUT_CHUNK as ::core::ffi::c_long
            {
                xmlParserGrow(ctxt);
            }
        }
    }
    if (*(*ctxt).input).id != inputid {
        xmlFatalErr(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"Unexpected change of input\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        localname = ::core::ptr::null::<xmlChar>();
    } else {
        if !(*ctxt).attsDefault.is_null() {
            let mut defaults: xmlDefAttrsPtr = ::core::ptr::null_mut::<xmlDefAttrs>();
            defaults = xmlHashLookup2((*ctxt).attsDefault, localname, prefix) as xmlDefAttrsPtr;
            if !defaults.is_null() {
                i = 0 as ::core::ffi::c_int;
                while i < (*defaults).nbAttrs {
                    let mut attr: *mut xmlDefAttr = (&raw mut (*defaults).attrs as *mut xmlDefAttr)
                        .offset(i as isize)
                        as *mut xmlDefAttr;
                    attname = (*attr).name.name;
                    aprefix = (*attr).prefix.name;
                    if attname == (*ctxt).str_xmlns && aprefix.is_null() {
                        xmlParserEntityCheck(ctxt, (*attr).expandedSize as ::core::ffi::c_ulong);
                        if xmlParserNsPush(
                            ctxt,
                            ::core::ptr::null::<xmlHashedString>(),
                            &raw mut (*attr).value,
                            NULL,
                            1 as ::core::ffi::c_int,
                        ) > 0 as ::core::ffi::c_int
                        {
                            nbNs += 1;
                        }
                    } else if aprefix == (*ctxt).str_xmlns {
                        xmlParserEntityCheck(ctxt, (*attr).expandedSize as ::core::ffi::c_ulong);
                        if xmlParserNsPush(
                            ctxt,
                            &raw mut (*attr).name,
                            &raw mut (*attr).value,
                            NULL,
                            1 as ::core::ffi::c_int,
                        ) > 0 as ::core::ffi::c_int
                        {
                            nbNs += 1;
                        }
                    } else {
                        nbTotalDef += 1 as ::core::ffi::c_int;
                    }
                    i += 1;
                }
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < nbatts {
            attname = *atts.offset(i as isize);
            aprefix = *atts.offset((i + 1 as ::core::ffi::c_int) as isize);
            if aprefix.is_null() {
                nsIndex = NS_INDEX_EMPTY;
            } else if aprefix == (*ctxt).str_xml {
                nsIndex = NS_INDEX_XML;
            } else {
                haprefix.name = aprefix;
                haprefix.hashValue = *atts.offset((i + 2 as ::core::ffi::c_int) as isize) as size_t
                    as ::core::ffi::c_uint;
                nsIndex = xmlParserNsLookup(
                    ctxt,
                    &raw mut haprefix,
                    ::core::ptr::null_mut::<*mut xmlParserNsBucket>(),
                );
                if nsIndex == INT_MAX {
                    xmlNsErr(
                        ctxt,
                        XML_NS_ERR_UNDEFINED_NAMESPACE,
                        b"Namespace prefix %s for %s on %s is not defined\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        aprefix,
                        attname,
                        localname,
                    );
                    nsIndex = NS_INDEX_EMPTY;
                }
            }
            let ref mut fresh83 = *atts.offset((i + 2 as ::core::ffi::c_int) as isize);
            *fresh83 = nsIndex as ptrdiff_t as *const xmlChar;
            i += 5 as ::core::ffi::c_int;
        }
        maxAtts = nratts + nbTotalDef;
        if maxAtts > 1 as ::core::ffi::c_int {
            attrHashSize = 4 as ::core::ffi::c_uint;
            while attrHashSize.wrapping_div(2 as ::core::ffi::c_uint)
                < maxAtts as ::core::ffi::c_uint
            {
                attrHashSize = attrHashSize.wrapping_mul(2 as ::core::ffi::c_uint);
            }
            if attrHashSize > (*ctxt).attrHashMax {
                let mut tmp: *mut xmlAttrHashBucket = ::core::ptr::null_mut::<xmlAttrHashBucket>();
                tmp = xmlRealloc.expect("non-null function pointer")(
                    (*ctxt).attrHash as *mut ::core::ffi::c_void,
                    (attrHashSize as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlAttrHashBucket>() as size_t),
                ) as *mut xmlAttrHashBucket;
                if tmp.is_null() {
                    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                    current_block = 9184737459233949222;
                } else {
                    (*ctxt).attrHash = tmp;
                    (*ctxt).attrHashMax = attrHashSize;
                    current_block = 15514437232607373049;
                }
            } else {
                current_block = 15514437232607373049;
            }
            match current_block {
                9184737459233949222 => {}
                _ => {
                    memset(
                        (*ctxt).attrHash as *mut ::core::ffi::c_void,
                        -(1 as ::core::ffi::c_int),
                        (attrHashSize as size_t)
                            .wrapping_mul(::core::mem::size_of::<xmlAttrHashBucket>() as size_t),
                    );
                    i = 0 as ::core::ffi::c_int;
                    j = 0 as ::core::ffi::c_int;
                    while j < nratts {
                        let mut nsuri: *const xmlChar = ::core::ptr::null::<xmlChar>();
                        let mut hashValue: ::core::ffi::c_uint = 0;
                        let mut nameHashValue: ::core::ffi::c_uint = 0;
                        let mut uriHashValue: ::core::ffi::c_uint = 0;
                        let mut res: ::core::ffi::c_int = 0;
                        attname = *atts.offset(i as isize);
                        aprefix = *atts.offset((i + 1 as ::core::ffi::c_int) as isize);
                        nsIndex = *atts.offset((i + 2 as ::core::ffi::c_int) as isize) as ptrdiff_t
                            as ::core::ffi::c_int;
                        nameHashValue = *(*ctxt).attallocs.offset(j as isize)
                            | 0x80000000 as ::core::ffi::c_uint;
                        if nsIndex == NS_INDEX_EMPTY {
                            nsuri = ::core::ptr::null::<xmlChar>();
                            uriHashValue = URI_HASH_EMPTY;
                        } else if nsIndex == NS_INDEX_XML {
                            nsuri = (*ctxt).str_xml_ns;
                            uriHashValue = URI_HASH_XML;
                        } else {
                            nsuri = *(*ctxt).nsTab.offset(
                                (nsIndex * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as isize,
                            );
                            uriHashValue =
                                (*(*(*ctxt).nsdb).extra.offset(nsIndex as isize)).uriHashValue;
                        }
                        hashValue = xmlDictCombineHash(nameHashValue, uriHashValue);
                        res = xmlAttrHashInsert(ctxt, attrHashSize, attname, nsuri, hashValue, i);
                        if !(res < 0 as ::core::ffi::c_int) {
                            if res < INT_MAX {
                                if aprefix == *atts.offset((res + 1 as ::core::ffi::c_int) as isize)
                                {
                                    xmlErrAttributeDup(ctxt, aprefix, attname);
                                } else {
                                    xmlNsErr(
                                        ctxt,
                                        XML_NS_ERR_ATTRIBUTE_REDEFINED,
                                        b"Namespaced Attribute %s in '%s' redefined\n\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        attname,
                                        nsuri,
                                        ::core::ptr::null::<xmlChar>(),
                                    );
                                }
                            }
                        }
                        i += 5 as ::core::ffi::c_int;
                        j += 1;
                    }
                    current_block = 12045739402850935335;
                }
            }
        } else {
            current_block = 12045739402850935335;
        }
        match current_block {
            9184737459233949222 => {}
            _ => {
                if !(*ctxt).attsDefault.is_null() {
                    let mut defaults_0: xmlDefAttrsPtr = ::core::ptr::null_mut::<xmlDefAttrs>();
                    defaults_0 =
                        xmlHashLookup2((*ctxt).attsDefault, localname, prefix) as xmlDefAttrsPtr;
                    if !defaults_0.is_null() {
                        i = 0 as ::core::ffi::c_int;
                        loop {
                            if !(i < (*defaults_0).nbAttrs) {
                                current_block = 8454008399274236288;
                                break;
                            }
                            let mut attr_0: *mut xmlDefAttr = (&raw mut (*defaults_0).attrs
                                as *mut xmlDefAttr)
                                .offset(i as isize)
                                as *mut xmlDefAttr;
                            let mut nsuri_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
                            let mut hashValue_0: ::core::ffi::c_uint = 0;
                            let mut uriHashValue_0: ::core::ffi::c_uint = 0;
                            let mut res_0: ::core::ffi::c_int = 0;
                            attname = (*attr_0).name.name;
                            aprefix = (*attr_0).prefix.name;
                            if !(attname == (*ctxt).str_xmlns && aprefix.is_null()) {
                                if !(aprefix == (*ctxt).str_xmlns) {
                                    if aprefix.is_null() {
                                        nsIndex = NS_INDEX_EMPTY;
                                        nsuri_0 = ::core::ptr::null::<xmlChar>();
                                        uriHashValue_0 = URI_HASH_EMPTY;
                                    }
                                    if aprefix == (*ctxt).str_xml {
                                        nsIndex = NS_INDEX_XML;
                                        nsuri_0 = (*ctxt).str_xml_ns;
                                        uriHashValue_0 = URI_HASH_XML;
                                    } else if !aprefix.is_null() {
                                        nsIndex = xmlParserNsLookup(
                                            ctxt,
                                            &raw mut (*attr_0).prefix,
                                            ::core::ptr::null_mut::<*mut xmlParserNsBucket>(),
                                        );
                                        if nsIndex == INT_MAX {
                                            xmlNsErr(
                                                ctxt,
                                                XML_NS_ERR_UNDEFINED_NAMESPACE,
                                                b"Namespace prefix %s for %s on %s is not defined\n\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                aprefix,
                                                attname,
                                                localname,
                                            );
                                            nsIndex = NS_INDEX_EMPTY;
                                            nsuri_0 = ::core::ptr::null::<xmlChar>();
                                            uriHashValue_0 = URI_HASH_EMPTY;
                                        } else {
                                            nsuri_0 = *(*ctxt).nsTab.offset(
                                                (nsIndex * 2 as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            );
                                            uriHashValue_0 =
                                                (*(*(*ctxt).nsdb).extra.offset(nsIndex as isize))
                                                    .uriHashValue;
                                        }
                                    }
                                    if maxAtts > 1 as ::core::ffi::c_int {
                                        hashValue_0 = xmlDictCombineHash(
                                            (*attr_0).name.hashValue,
                                            uriHashValue_0,
                                        );
                                        res_0 = xmlAttrHashInsert(
                                            ctxt,
                                            attrHashSize,
                                            attname,
                                            nsuri_0,
                                            hashValue_0,
                                            nbatts,
                                        );
                                        if res_0 < 0 as ::core::ffi::c_int {
                                            current_block = 11322929247169729670;
                                        } else if res_0 < INT_MAX {
                                            if aprefix
                                                == *atts.offset(
                                                    (res_0 + 1 as ::core::ffi::c_int) as isize,
                                                )
                                            {
                                                current_block = 11322929247169729670;
                                            } else {
                                                xmlNsErr(
                                                    ctxt,
                                                    XML_NS_ERR_ATTRIBUTE_REDEFINED,
                                                    b"Namespaced Attribute %s in '%s' redefined\n\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    attname,
                                                    nsuri_0,
                                                    ::core::ptr::null::<xmlChar>(),
                                                );
                                                current_block = 13598064676775803384;
                                            }
                                        } else {
                                            current_block = 13598064676775803384;
                                        }
                                    } else {
                                        current_block = 13598064676775803384;
                                    }
                                    match current_block {
                                        11322929247169729670 => {}
                                        _ => {
                                            xmlParserEntityCheck(
                                                ctxt,
                                                (*attr_0).expandedSize as ::core::ffi::c_ulong,
                                            );
                                            if atts.is_null()
                                                || nbatts + 5 as ::core::ffi::c_int > maxatts
                                            {
                                                if xmlCtxtGrowAttrs(
                                                    ctxt,
                                                    nbatts + 5 as ::core::ffi::c_int,
                                                ) < 0 as ::core::ffi::c_int
                                                {
                                                    localname = ::core::ptr::null::<xmlChar>();
                                                    current_block = 9184737459233949222;
                                                    break;
                                                } else {
                                                    maxatts = (*ctxt).maxatts;
                                                    atts = (*ctxt).atts;
                                                }
                                            }
                                            let fresh84 = nbatts;
                                            nbatts = nbatts + 1;
                                            let ref mut fresh85 = *atts.offset(fresh84 as isize);
                                            *fresh85 = attname;
                                            let fresh86 = nbatts;
                                            nbatts = nbatts + 1;
                                            let ref mut fresh87 = *atts.offset(fresh86 as isize);
                                            *fresh87 = aprefix;
                                            let fresh88 = nbatts;
                                            nbatts = nbatts + 1;
                                            let ref mut fresh89 = *atts.offset(fresh88 as isize);
                                            *fresh89 = nsIndex as ptrdiff_t as *const xmlChar;
                                            let fresh90 = nbatts;
                                            nbatts = nbatts + 1;
                                            let ref mut fresh91 = *atts.offset(fresh90 as isize);
                                            *fresh91 = (*attr_0).value.name;
                                            let fresh92 = nbatts;
                                            nbatts = nbatts + 1;
                                            let ref mut fresh93 = *atts.offset(fresh92 as isize);
                                            *fresh93 = (*attr_0).valueEnd;
                                            if (*ctxt).standalone == 1 as ::core::ffi::c_int
                                                && (*attr_0).external != 0 as ::core::ffi::c_int
                                            {
                                                xmlValidityError(
                                                    ctxt,
                                                    XML_DTD_STANDALONE_DEFAULTED,
                                                    b"standalone: attribute %s on %s defaulted from external subset\n\0"
                                                        as *const u8 as *const ::core::ffi::c_char,
                                                    attname,
                                                    localname,
                                                );
                                            }
                                            nbdef += 1;
                                        }
                                    }
                                }
                            }
                            i += 1;
                        }
                    } else {
                        current_block = 8454008399274236288;
                    }
                } else {
                    current_block = 8454008399274236288;
                }
                match current_block {
                    9184737459233949222 => {}
                    _ => {
                        i = 0 as ::core::ffi::c_int;
                        j = 0 as ::core::ffi::c_int;
                        while i < nbatts {
                            nsIndex = *atts.offset((i + 2 as ::core::ffi::c_int) as isize)
                                as ptrdiff_t
                                as ::core::ffi::c_int;
                            if nsIndex == INT_MAX {
                                let ref mut fresh94 =
                                    *atts.offset((i + 2 as ::core::ffi::c_int) as isize);
                                *fresh94 = ::core::ptr::null::<xmlChar>();
                            } else if nsIndex == INT_MAX - 1 as ::core::ffi::c_int {
                                let ref mut fresh95 =
                                    *atts.offset((i + 2 as ::core::ffi::c_int) as isize);
                                *fresh95 = (*ctxt).str_xml_ns;
                            } else {
                                let ref mut fresh96 =
                                    *atts.offset((i + 2 as ::core::ffi::c_int) as isize);
                                *fresh96 = *(*ctxt).nsTab.offset(
                                    (nsIndex * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        as isize,
                                );
                            }
                            if j < nratts
                                && *(*ctxt).attallocs.offset(j as isize)
                                    & 0x80000000 as ::core::ffi::c_uint
                                    == 0 as ::core::ffi::c_uint
                            {
                                let ref mut fresh97 =
                                    *atts.offset((i + 3 as ::core::ffi::c_int) as isize);
                                *fresh97 = (*(*ctxt).input)
                                    .base
                                    .offset(*atts.offset((i + 3 as ::core::ffi::c_int) as isize)
                                        as ptrdiff_t
                                        as isize);
                                let ref mut fresh98 =
                                    *atts.offset((i + 4 as ::core::ffi::c_int) as isize);
                                *fresh98 = (*(*ctxt).input)
                                    .base
                                    .offset(*atts.offset((i + 4 as ::core::ffi::c_int) as isize)
                                        as ptrdiff_t
                                        as isize);
                            }
                            i += 5 as ::core::ffi::c_int;
                            j += 1;
                        }
                        uri = xmlParserNsLookupUri(ctxt, &raw mut hprefix);
                        if !prefix.is_null() && uri.is_null() {
                            xmlNsErr(
                                ctxt,
                                XML_NS_ERR_UNDEFINED_NAMESPACE,
                                b"Namespace prefix %s on %s is not defined\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                prefix,
                                localname,
                                ::core::ptr::null::<xmlChar>(),
                            );
                        }
                        *pref = prefix;
                        *URI = uri;
                        if !(*ctxt).sax.is_null()
                            && (*(*ctxt).sax).startElementNs.is_some()
                            && (*ctxt).disableSAX == 0
                        {
                            if nbNs > 0 as ::core::ffi::c_int {
                                (*(*ctxt).sax)
                                    .startElementNs
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    localname,
                                    prefix,
                                    uri,
                                    nbNs,
                                    (*ctxt).nsTab.offset(
                                        (2 as ::core::ffi::c_int * ((*ctxt).nsNr - nbNs)) as isize,
                                    ),
                                    nbatts / 5 as ::core::ffi::c_int,
                                    nbdef,
                                    atts,
                                );
                            } else {
                                (*(*ctxt).sax)
                                    .startElementNs
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    localname,
                                    prefix,
                                    uri,
                                    0 as ::core::ffi::c_int,
                                    ::core::ptr::null_mut::<*const xmlChar>(),
                                    nbatts / 5 as ::core::ffi::c_int,
                                    nbdef,
                                    atts,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    if attval != 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while j < nratts {
            if *(*ctxt).attallocs.offset(j as isize) & 0x80000000 as ::core::ffi::c_uint != 0 {
                xmlFree.expect("non-null function pointer")(
                    *atts.offset((i + 3 as ::core::ffi::c_int) as isize) as *mut xmlChar
                        as *mut ::core::ffi::c_void,
                );
            }
            i += 5 as ::core::ffi::c_int;
            j += 1;
        }
    }
    *nbNsPtr = nbNs;
    return localname;
}
unsafe extern "C" fn xmlParseEndTag2(mut ctxt: xmlParserCtxtPtr, mut tag: *const xmlStartTag) {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '/' as i32
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_LTSLASH_REQUIRED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(2 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if (*tag).prefix.is_null() {
        name = xmlParseNameAndCompare(ctxt, (*ctxt).name);
    } else {
        name = xmlParseQNameAndCompare(ctxt, (*ctxt).name, (*tag).prefix);
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return;
    }
    xmlSkipBlankChars(ctxt);
    if !(0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
        && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
        || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
        || 0x20 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int)
        || *(*(*ctxt).input).cur as ::core::ffi::c_int != '>' as i32
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_GT_REQUIRED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else {
        (*(*ctxt).input).col += 1;
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
    }
    if name != 1 as ::core::ffi::c_int as *mut xmlChar as *const xmlChar {
        if name.is_null() {
            name = b"unparsable\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar;
        }
        xmlFatalErrMsgStrIntStr(
            ctxt,
            XML_ERR_TAG_NAME_MISMATCH,
            b"Opening and ending tag mismatch: %s line %d and %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*ctxt).name,
            (*tag).line,
            name,
        );
    }
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endElementNs.is_some() && (*ctxt).disableSAX == 0 {
        (*(*ctxt).sax)
            .endElementNs
            .expect("non-null function pointer")(
            (*ctxt).userData,
            (*ctxt).name,
            (*tag).prefix,
            (*tag).URI,
        );
    }
    spacePop(ctxt);
    if (*tag).nsNr != 0 as ::core::ffi::c_int {
        xmlParserNsPop(ctxt, (*tag).nsNr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseCDSect(mut ctxt: xmlParserCtxtPtr) {
    let mut current_block: u64;
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = XML_PARSER_BUFFER_SIZE;
    let mut r: ::core::ffi::c_int = 0;
    let mut rl: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut sl: ::core::ffi::c_int = 0;
    let mut cur: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_HUGE_LENGTH
        } else {
            XML_MAX_TEXT_LENGTH
        };
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '!' as i32
        || *(*(*ctxt).input)
            .cur
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '[' as i32
    {
        return;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(3 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if !(*((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'C' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'A' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '[' as i32)
    {
        return;
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(6 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 6 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    (*ctxt).instate = XML_PARSER_CDATA_SECTION;
    r = xmlCurrentChar(ctxt, &raw mut rl);
    if if r < 0x100 as ::core::ffi::c_int {
        (0x9 as ::core::ffi::c_int <= r && r <= 0xa as ::core::ffi::c_int
            || r == 0xd as ::core::ffi::c_int
            || 0x20 as ::core::ffi::c_int <= r) as ::core::ffi::c_int
    } else {
        (0x100 as ::core::ffi::c_int <= r && r <= 0xd7ff as ::core::ffi::c_int
            || 0xe000 as ::core::ffi::c_int <= r && r <= 0xfffd as ::core::ffi::c_int
            || 0x10000 as ::core::ffi::c_int <= r && r <= 0x10ffff as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } == 0
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_CDATA_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else {
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
            (*(*ctxt).input).line += 1;
            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
        } else {
            (*(*ctxt).input).col += 1;
        }
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(rl as isize);
        s = xmlCurrentChar(ctxt, &raw mut sl);
        if if s < 0x100 as ::core::ffi::c_int {
            (0x9 as ::core::ffi::c_int <= s && s <= 0xa as ::core::ffi::c_int
                || s == 0xd as ::core::ffi::c_int
                || 0x20 as ::core::ffi::c_int <= s) as ::core::ffi::c_int
        } else {
            (0x100 as ::core::ffi::c_int <= s && s <= 0xd7ff as ::core::ffi::c_int
                || 0xe000 as ::core::ffi::c_int <= s && s <= 0xfffd as ::core::ffi::c_int
                || 0x10000 as ::core::ffi::c_int <= s && s <= 0x10ffff as ::core::ffi::c_int)
                as ::core::ffi::c_int
        } == 0
        {
            xmlFatalErr(
                ctxt,
                XML_ERR_CDATA_NOT_FINISHED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        } else {
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                (*(*ctxt).input).line += 1;
                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
            } else {
                (*(*ctxt).input).col += 1;
            }
            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(sl as isize);
            cur = xmlCurrentChar(ctxt, &raw mut l);
            buf =
                xmlMallocAtomic.expect("non-null function pointer")(size as size_t) as *mut xmlChar;
            if buf.is_null() {
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            } else {
                loop {
                    if !((if cur < 0x100 as ::core::ffi::c_int {
                        (0x9 as ::core::ffi::c_int <= cur && cur <= 0xa as ::core::ffi::c_int
                            || cur == 0xd as ::core::ffi::c_int
                            || 0x20 as ::core::ffi::c_int <= cur)
                            as ::core::ffi::c_int
                    } else {
                        (0x100 as ::core::ffi::c_int <= cur && cur <= 0xd7ff as ::core::ffi::c_int
                            || 0xe000 as ::core::ffi::c_int <= cur
                                && cur <= 0xfffd as ::core::ffi::c_int
                            || 0x10000 as ::core::ffi::c_int <= cur
                                && cur <= 0x10ffff as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                    }) != 0
                        && (r != ']' as i32 || s != ']' as i32 || cur != '>' as i32))
                    {
                        current_block = 8545136480011357681;
                        break;
                    }
                    if len + 5 as ::core::ffi::c_int >= size {
                        let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        tmp = xmlRealloc.expect("non-null function pointer")(
                            buf as *mut ::core::ffi::c_void,
                            (size * 2 as ::core::ffi::c_int) as size_t,
                        ) as *mut xmlChar;
                        if tmp.is_null() {
                            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                            current_block = 13151969230923514215;
                            break;
                        } else {
                            buf = tmp;
                            size *= 2 as ::core::ffi::c_int;
                        }
                    }
                    if r < 0x80 as ::core::ffi::c_int {
                        let fresh103 = len;
                        len = len + 1;
                        *buf.offset(fresh103 as isize) = r as xmlChar;
                    } else {
                        len += xmlCopyCharMultiByte(buf.offset(len as isize) as *mut xmlChar, r);
                    }
                    if len > maxLength {
                        xmlFatalErrMsg(
                            ctxt,
                            XML_ERR_CDATA_NOT_FINISHED,
                            b"CData section too big found\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 13151969230923514215;
                        break;
                    } else {
                        r = s;
                        rl = sl;
                        s = cur;
                        sl = l;
                        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                            (*(*ctxt).input).line += 1;
                            (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                        } else {
                            (*(*ctxt).input).col += 1;
                        }
                        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                        cur = xmlCurrentChar(ctxt, &raw mut l);
                    }
                }
                match current_block {
                    13151969230923514215 => {}
                    _ => {
                        *buf.offset(len as isize) = 0 as xmlChar;
                        if (*ctxt).instate as ::core::ffi::c_int
                            == XML_PARSER_EOF as ::core::ffi::c_int
                        {
                            xmlFree.expect("non-null function pointer")(
                                buf as *mut ::core::ffi::c_void,
                            );
                            return;
                        }
                        if cur != '>' as i32 {
                            xmlFatalErrMsgStr(
                                ctxt,
                                XML_ERR_CDATA_NOT_FINISHED,
                                b"CData section not finished\n%.50s\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                buf,
                            );
                        } else {
                            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                                (*(*ctxt).input).line += 1;
                                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                            } else {
                                (*(*ctxt).input).col += 1;
                            }
                            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(l as isize);
                            if !(*ctxt).sax.is_null() && (*ctxt).disableSAX == 0 {
                                if (*(*ctxt).sax).cdataBlock.is_some() {
                                    (*(*ctxt).sax)
                                        .cdataBlock
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        buf,
                                        len,
                                    );
                                } else if (*(*ctxt).sax).characters.is_some() {
                                    (*(*ctxt).sax)
                                        .characters
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        buf,
                                        len,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int {
        (*ctxt).instate = XML_PARSER_CONTENT;
    }
    xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlParseContentInternal(mut ctxt: xmlParserCtxtPtr) {
    let mut nameNr: ::core::ffi::c_int = (*ctxt).nameNr;
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    while (*(*ctxt).input).cur < (*(*ctxt).input).end
        && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
    {
        let mut cur: *const xmlChar = (*(*ctxt).input).cur;
        if *cur as ::core::ffi::c_int == '<' as i32
            && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '?' as i32
        {
            xmlParsePI(ctxt);
        } else if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '<' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '!' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '[' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'C' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'D' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'T' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'A' as i32
            && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '[' as i32
        {
            xmlParseCDSect(ctxt);
        } else if *cur as ::core::ffi::c_int == '<' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '!' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32
        {
            xmlParseComment(ctxt);
            (*ctxt).instate = XML_PARSER_CONTENT;
        } else if *cur as ::core::ffi::c_int == '<' as i32 {
            if *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
            {
                if (*ctxt).nameNr <= nameNr {
                    break;
                }
                xmlParseElementEnd(ctxt);
            } else {
                xmlParseElementStart(ctxt);
            }
        } else if *cur as ::core::ffi::c_int == '&' as i32 {
            xmlParseReference(ctxt);
        } else {
            xmlParseCharDataInternal(ctxt, 0 as ::core::ffi::c_int);
        }
        if ((*ctxt).progressive == 0 as ::core::ffi::c_int
            || (*ctxt).inputNr > 1 as ::core::ffi::c_int)
            && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                > (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
            && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < (2 as ::core::ffi::c_int * INPUT_CHUNK) as ::core::ffi::c_long
        {
            xmlParserShrink(ctxt);
        }
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseContent(mut ctxt: xmlParserCtxtPtr) {
    let mut nameNr: ::core::ffi::c_int = (*ctxt).nameNr;
    xmlParseContentInternal(ctxt);
    if (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
        && (*ctxt).errNo == XML_ERR_OK as ::core::ffi::c_int
        && (*ctxt).nameNr > nameNr
    {
        let mut name: *const xmlChar = *(*ctxt)
            .nameTab
            .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize);
        let mut line: ::core::ffi::c_int = (*(*ctxt)
            .pushTab
            .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize))
        .line;
        xmlFatalErrMsgStrIntStr(
            ctxt,
            XML_ERR_TAG_NOT_FINISHED,
            b"Premature end of data in tag %s line %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
            line,
            ::core::ptr::null::<xmlChar>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseElement(mut ctxt: xmlParserCtxtPtr) {
    if xmlParseElementStart(ctxt) != 0 as ::core::ffi::c_int {
        return;
    }
    xmlParseContentInternal(ctxt);
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return;
    }
    if (*(*ctxt).input).cur >= (*(*ctxt).input).end {
        if (*ctxt).errNo == XML_ERR_OK as ::core::ffi::c_int {
            let mut name: *const xmlChar = *(*ctxt)
                .nameTab
                .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize);
            let mut line: ::core::ffi::c_int = (*(*ctxt)
                .pushTab
                .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize))
            .line;
            xmlFatalErrMsgStrIntStr(
                ctxt,
                XML_ERR_TAG_NOT_FINISHED,
                b"Premature end of data in tag %s line %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
                line,
                ::core::ptr::null::<xmlChar>(),
            );
        }
        return;
    }
    xmlParseElementEnd(ctxt);
}
unsafe extern "C" fn xmlParseElementStart(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut prefix: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut URI: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut node_info: xmlParserNodeInfo = xmlParserNodeInfo {
        node: ::core::ptr::null::<_xmlNode>(),
        begin_pos: 0,
        begin_line: 0,
        end_pos: 0,
        end_line: 0,
    };
    let mut line: ::core::ffi::c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut nbNs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*ctxt).nameNr as ::core::ffi::c_uint > xmlParserMaxDepth
        && (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        xmlFatalErrMsgInt(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"Excessive depth in document: %d use XML_PARSE_HUGE option\n\0" as *const u8
                as *const ::core::ffi::c_char,
            xmlParserMaxDepth as ::core::ffi::c_int,
        );
        xmlHaltParser(ctxt);
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).record_info != 0 {
        node_info.begin_pos = (*(*ctxt).input)
            .consumed
            .wrapping_add((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                as ::core::ffi::c_long as ::core::ffi::c_ulong);
        node_info.begin_line = (*(*ctxt).input).line as ::core::ffi::c_ulong;
    }
    if (*ctxt).spaceNr == 0 as ::core::ffi::c_int {
        spacePush(ctxt, -(1 as ::core::ffi::c_int));
    } else if *(*ctxt).space == -(2 as ::core::ffi::c_int) {
        spacePush(ctxt, -(1 as ::core::ffi::c_int));
    } else {
        spacePush(ctxt, *(*ctxt).space);
    }
    line = (*(*ctxt).input).line;
    if (*ctxt).sax2 != 0 {
        name = xmlParseStartTag2(ctxt, &raw mut prefix, &raw mut URI, &raw mut nbNs);
    } else {
        name = xmlParseStartTag(ctxt);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if name.is_null() {
        spacePop(ctxt);
        return -(1 as ::core::ffi::c_int);
    }
    nameNsPush(ctxt, name, prefix, URI, line, nbNs);
    cur = (*ctxt).node;
    if (*ctxt).validate != 0
        && (*ctxt).wellFormed != 0
        && !(*ctxt).myDoc.is_null()
        && !(*ctxt).node.is_null()
        && (*ctxt).node == (*(*ctxt).myDoc).children
    {
        (*ctxt).valid &= xmlValidateRoot(&raw mut (*ctxt).vctxt, (*ctxt).myDoc);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '/' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '>' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(2 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if (*ctxt).sax2 != 0 {
            if !(*ctxt).sax.is_null()
                && (*(*ctxt).sax).endElementNs.is_some()
                && (*ctxt).disableSAX == 0
            {
                (*(*ctxt).sax)
                    .endElementNs
                    .expect("non-null function pointer")(
                    (*ctxt).userData, name, prefix, URI
                );
            }
        } else if !(*ctxt).sax.is_null()
            && (*(*ctxt).sax).endElement.is_some()
            && (*ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .endElement
                .expect("non-null function pointer")((*ctxt).userData, name);
        }
        namePop(ctxt);
        spacePop(ctxt);
        if nbNs > 0 as ::core::ffi::c_int {
            xmlParserNsPop(ctxt, nbNs);
        }
        if !cur.is_null() && (*ctxt).record_info != 0 {
            node_info.node = cur as *const _xmlNode;
            node_info.end_pos = (*(*ctxt).input)
                .consumed
                .wrapping_add((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                    as ::core::ffi::c_long as ::core::ffi::c_ulong);
            node_info.end_line = (*(*ctxt).input).line as ::core::ffi::c_ulong;
            xmlParserAddNodeInfo(ctxt, &raw mut node_info);
        }
        return 1 as ::core::ffi::c_int;
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '>' as i32 {
        (*(*ctxt).input).col += 1;
        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        if !cur.is_null() && (*ctxt).record_info != 0 {
            node_info.node = cur as *const _xmlNode;
            node_info.end_pos = 0 as ::core::ffi::c_ulong;
            node_info.end_line = 0 as ::core::ffi::c_ulong;
            xmlParserAddNodeInfo(ctxt, &raw mut node_info);
        }
    } else {
        xmlFatalErrMsgStrIntStr(
            ctxt,
            XML_ERR_GT_REQUIRED,
            b"Couldn't find end of Start Tag %s line %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
            line,
            ::core::ptr::null::<xmlChar>(),
        );
        nodePop(ctxt);
        namePop(ctxt);
        spacePop(ctxt);
        if nbNs > 0 as ::core::ffi::c_int {
            xmlParserNsPop(ctxt, nbNs);
        }
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParseElementEnd(mut ctxt: xmlParserCtxtPtr) {
    let mut cur: xmlNodePtr = (*ctxt).node;
    if (*ctxt).nameNr <= 0 as ::core::ffi::c_int {
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
        }
        return;
    }
    if (*ctxt).sax2 != 0 {
        xmlParseEndTag2(
            ctxt,
            (*ctxt)
                .pushTab
                .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize)
                as *mut xmlStartTag,
        );
        namePop(ctxt);
    } else {
        xmlParseEndTag1(ctxt, 0 as ::core::ffi::c_int);
    }
    if !cur.is_null() && (*ctxt).record_info != 0 {
        let mut node_info: xmlParserNodeInfoPtr = ::core::ptr::null_mut::<xmlParserNodeInfo>();
        node_info = xmlParserFindNodeInfo(ctxt, cur) as xmlParserNodeInfoPtr;
        if !node_info.is_null() {
            (*node_info).end_pos = (*(*ctxt).input)
                .consumed
                .wrapping_add((*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                    as ::core::ffi::c_long as ::core::ffi::c_ulong);
            (*node_info).end_line = (*(*ctxt).input).line as ::core::ffi::c_ulong;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseVersionNum(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut cur: xmlChar = 0;
    buf = xmlMallocAtomic.expect("non-null function pointer")(size as size_t) as *mut xmlChar;
    if buf.is_null() {
        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
        return ::core::ptr::null_mut::<xmlChar>();
    }
    cur = *(*(*ctxt).input).cur;
    if !(cur as ::core::ffi::c_int >= '0' as i32 && cur as ::core::ffi::c_int <= '9' as i32) {
        xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    let fresh5 = len;
    len = len + 1;
    *buf.offset(fresh5 as isize) = cur;
    xmlNextChar(ctxt);
    cur = *(*(*ctxt).input).cur;
    if cur as ::core::ffi::c_int != '.' as i32 {
        xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    let fresh6 = len;
    len = len + 1;
    *buf.offset(fresh6 as isize) = cur;
    xmlNextChar(ctxt);
    cur = *(*(*ctxt).input).cur;
    while cur as ::core::ffi::c_int >= '0' as i32 && cur as ::core::ffi::c_int <= '9' as i32 {
        if len + 1 as ::core::ffi::c_int >= size {
            let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            size *= 2 as ::core::ffi::c_int;
            tmp = xmlRealloc.expect("non-null function pointer")(
                buf as *mut ::core::ffi::c_void,
                size as size_t,
            ) as *mut xmlChar;
            if tmp.is_null() {
                xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                return ::core::ptr::null_mut::<xmlChar>();
            }
            buf = tmp;
        }
        let fresh7 = len;
        len = len + 1;
        *buf.offset(fresh7 as isize) = cur;
        xmlNextChar(ctxt);
        cur = *(*(*ctxt).input).cur;
    }
    *buf.offset(len as isize) = 0 as xmlChar;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseVersionInfo(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut version: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 'v' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'e' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'r' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 's' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'i' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'o' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'n' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(7 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 7 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        xmlSkipBlankChars(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int != '=' as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_EQUAL_REQUIRED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return ::core::ptr::null_mut::<xmlChar>();
        }
        xmlNextChar(ctxt);
        xmlSkipBlankChars(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '"' as i32 {
            xmlNextChar(ctxt);
            version = xmlParseVersionNum(ctxt);
            if *(*(*ctxt).input).cur as ::core::ffi::c_int != '"' as i32 {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_STRING_NOT_CLOSED,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            } else {
                xmlNextChar(ctxt);
            }
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\'' as i32 {
            xmlNextChar(ctxt);
            version = xmlParseVersionNum(ctxt);
            if *(*(*ctxt).input).cur as ::core::ffi::c_int != '\'' as i32 {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_STRING_NOT_CLOSED,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            } else {
                xmlNextChar(ctxt);
            }
        } else {
            xmlFatalErr(
                ctxt,
                XML_ERR_STRING_NOT_STARTED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return version;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEncName(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut maxLength: ::core::ffi::c_int =
        if (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
            XML_MAX_TEXT_LENGTH
        } else {
            XML_MAX_NAME_LENGTH
        };
    let mut cur: xmlChar = 0;
    cur = *(*(*ctxt).input).cur;
    if cur as ::core::ffi::c_int >= 'a' as i32 && cur as ::core::ffi::c_int <= 'z' as i32
        || cur as ::core::ffi::c_int >= 'A' as i32 && cur as ::core::ffi::c_int <= 'Z' as i32
    {
        buf = xmlMallocAtomic.expect("non-null function pointer")(size as size_t) as *mut xmlChar;
        if buf.is_null() {
            xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
            return ::core::ptr::null_mut::<xmlChar>();
        }
        let fresh3 = len;
        len = len + 1;
        *buf.offset(fresh3 as isize) = cur;
        xmlNextChar(ctxt);
        cur = *(*(*ctxt).input).cur;
        while cur as ::core::ffi::c_int >= 'a' as i32 && cur as ::core::ffi::c_int <= 'z' as i32
            || cur as ::core::ffi::c_int >= 'A' as i32 && cur as ::core::ffi::c_int <= 'Z' as i32
            || cur as ::core::ffi::c_int >= '0' as i32 && cur as ::core::ffi::c_int <= '9' as i32
            || cur as ::core::ffi::c_int == '.' as i32
            || cur as ::core::ffi::c_int == '_' as i32
            || cur as ::core::ffi::c_int == '-' as i32
        {
            if len + 1 as ::core::ffi::c_int >= size {
                let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                size *= 2 as ::core::ffi::c_int;
                tmp = xmlRealloc.expect("non-null function pointer")(
                    buf as *mut ::core::ffi::c_void,
                    size as size_t,
                ) as *mut xmlChar;
                if tmp.is_null() {
                    xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                    xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<xmlChar>();
                }
                buf = tmp;
            }
            let fresh4 = len;
            len = len + 1;
            *buf.offset(fresh4 as isize) = cur;
            if len > maxLength {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_NAME_TOO_LONG,
                    b"EncName\0" as *const u8 as *const ::core::ffi::c_char,
                );
                xmlFree.expect("non-null function pointer")(buf as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            xmlNextChar(ctxt);
            cur = *(*(*ctxt).input).cur;
        }
        *buf.offset(len as isize) = 0 as xmlChar;
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_ENCODING_NAME,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEncodingDecl(mut ctxt: xmlParserCtxtPtr) -> *const xmlChar {
    let mut encoding: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    xmlSkipBlankChars(ctxt);
    if (*((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 'e' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'n' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'c' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'o' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'd' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'i' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'n' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'g' as i32) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null::<xmlChar>();
    }
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(8 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 8 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    xmlSkipBlankChars(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '=' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_EQUAL_REQUIRED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return ::core::ptr::null::<xmlChar>();
    }
    xmlNextChar(ctxt);
    xmlSkipBlankChars(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '"' as i32 {
        xmlNextChar(ctxt);
        encoding = xmlParseEncName(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int != '"' as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_STRING_NOT_CLOSED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlFree.expect("non-null function pointer")(encoding as *mut ::core::ffi::c_void);
            return ::core::ptr::null::<xmlChar>();
        } else {
            xmlNextChar(ctxt);
        }
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\'' as i32 {
        xmlNextChar(ctxt);
        encoding = xmlParseEncName(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int != '\'' as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_STRING_NOT_CLOSED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlFree.expect("non-null function pointer")(encoding as *mut ::core::ffi::c_void);
            return ::core::ptr::null::<xmlChar>();
        } else {
            xmlNextChar(ctxt);
        }
    } else {
        xmlFatalErr(
            ctxt,
            XML_ERR_STRING_NOT_STARTED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if encoding.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    xmlSetDeclaredEncoding(ctxt, encoding);
    return (*ctxt).encoding;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseSDDecl(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut standalone: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    xmlSkipBlankChars(ctxt);
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == 's' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 't' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'a' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'n' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'd' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'a' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'l' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'o' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'n' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'e' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(10 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 10 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
        xmlSkipBlankChars(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int != '=' as i32 {
            xmlFatalErr(
                ctxt,
                XML_ERR_EQUAL_REQUIRED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            return standalone;
        }
        xmlNextChar(ctxt);
        xmlSkipBlankChars(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\'' as i32 {
            xmlNextChar(ctxt);
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 'n' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'o' as i32
            {
                standalone = 0 as ::core::ffi::c_int;
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(2 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == 'y' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'e' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 's' as i32
            {
                standalone = 1 as ::core::ffi::c_int;
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(3 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_STANDALONE_VALUE,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int != '\'' as i32 {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_STRING_NOT_CLOSED,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            } else {
                xmlNextChar(ctxt);
            }
        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '"' as i32 {
            xmlNextChar(ctxt);
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 'n' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'o' as i32
            {
                standalone = 0 as ::core::ffi::c_int;
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(2 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
            } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == 'y' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 'e' as i32
                && *(*(*ctxt).input)
                    .cur
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 's' as i32
            {
                standalone = 1 as ::core::ffi::c_int;
                (*(*ctxt).input).cur = (*(*ctxt).input)
                    .cur
                    .offset(3 as ::core::ffi::c_int as isize);
                (*(*ctxt).input).col += 3 as ::core::ffi::c_int;
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    xmlParserGrow(ctxt);
                }
            } else {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_STANDALONE_VALUE,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
            if *(*(*ctxt).input).cur as ::core::ffi::c_int != '"' as i32 {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_STRING_NOT_CLOSED,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            } else {
                xmlNextChar(ctxt);
            }
        } else {
            xmlFatalErr(
                ctxt,
                XML_ERR_STRING_NOT_STARTED,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return standalone;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseXMLDecl(mut ctxt: xmlParserCtxtPtr) {
    let mut version: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    (*ctxt).standalone = -(2 as ::core::ffi::c_int);
    (*(*ctxt).input).cur = (*(*ctxt).input)
        .cur
        .offset(5 as ::core::ffi::c_int as isize);
    (*(*ctxt).input).col += 5 as ::core::ffi::c_int;
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlParserGrow(ctxt);
    }
    if !(*(*(*ctxt).input).cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
        || 0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
            && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
        || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
    {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_SPACE_REQUIRED,
            b"Blank needed after '<?xml'\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    xmlSkipBlankChars(ctxt);
    version = xmlParseVersionInfo(ctxt);
    if version.is_null() {
        xmlFatalErr(
            ctxt,
            XML_ERR_VERSION_MISSING,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else {
        if xmlStrEqual(version, XML_DEFAULT_VERSION.as_ptr() as *const xmlChar) == 0 {
            if (*ctxt).options & XML_PARSE_OLD10 as ::core::ffi::c_int != 0 {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNKNOWN_VERSION,
                    b"Unsupported version '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                    version,
                );
            } else if *version.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '1' as i32
                && *version.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '.' as i32
            {
                xmlWarningMsg(
                    ctxt,
                    XML_WAR_UNKNOWN_VERSION,
                    b"Unsupported version '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                    version,
                    ::core::ptr::null::<xmlChar>(),
                );
            } else {
                xmlFatalErrMsgStr(
                    ctxt,
                    XML_ERR_UNKNOWN_VERSION,
                    b"Unsupported version '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                    version,
                );
            }
        }
        if !(*ctxt).version.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).version as *mut ::core::ffi::c_void,
            );
        }
        (*ctxt).version = version;
    }
    if !(*(*(*ctxt).input).cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
        || 0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
            && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
        || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
    {
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '?' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '>' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
            return;
        }
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_SPACE_REQUIRED,
            b"Blank needed here\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    xmlParseEncodingDecl(ctxt);
    if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as ::core::ffi::c_int
        || (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
    {
        return;
    }
    if !(*ctxt).encoding.is_null()
        && !(*(*(*ctxt).input).cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int <= *(*(*ctxt).input).cur as ::core::ffi::c_int
                && *(*(*ctxt).input).cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
            || *(*(*ctxt).input).cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
    {
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '?' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '>' as i32
        {
            (*(*ctxt).input).cur = (*(*ctxt).input)
                .cur
                .offset(2 as ::core::ffi::c_int as isize);
            (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
            if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xmlParserGrow(ctxt);
            }
            return;
        }
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_SPACE_REQUIRED,
            b"Blank needed here\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    xmlSkipBlankChars(ctxt);
    (*ctxt).standalone = xmlParseSDDecl(ctxt);
    xmlSkipBlankChars(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '?' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '>' as i32
    {
        (*(*ctxt).input).cur = (*(*ctxt).input)
            .cur
            .offset(2 as ::core::ffi::c_int as isize);
        (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            xmlParserGrow(ctxt);
        }
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '>' as i32 {
        xmlFatalErr(
            ctxt,
            XML_ERR_XMLDECL_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        xmlNextChar(ctxt);
    } else {
        let mut c: ::core::ffi::c_int = 0;
        xmlFatalErr(
            ctxt,
            XML_ERR_XMLDECL_NOT_FINISHED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        loop {
            c = *(*(*ctxt).input).cur as ::core::ffi::c_int;
            if !(c != 0 as ::core::ffi::c_int) {
                break;
            }
            xmlNextChar(ctxt);
            if c == '>' as i32 {
                break;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseMisc(mut ctxt: xmlParserCtxtPtr) {
    while (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int {
        xmlSkipBlankChars(ctxt);
        if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
            < INPUT_CHUNK as ::core::ffi::c_long
        {
            xmlParserGrow(ctxt);
        }
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32
        {
            xmlParsePI(ctxt);
        } else {
            if !(*((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '<' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '!' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '-' as i32
                && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '-' as i32)
            {
                break;
            }
            xmlParseComment(ctxt);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseDocument(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    xmlInitParser();
    if ctxt.is_null() || (*ctxt).input.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    xmlDetectSAX2(ctxt);
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some() {
        (*(*ctxt).sax)
            .setDocumentLocator
            .expect("non-null function pointer")((*ctxt).userData, __xmlDefaultSAXLocator());
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    xmlDetectEncoding(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlFatalErr(
            ctxt,
            XML_ERR_DOCUMENT_EMPTY,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '<' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '?' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'x' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'm' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'l' as i32
        && (*(*(*ctxt).input)
            .cur
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int
                <= *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                && *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= 0xa as ::core::ffi::c_int
            || *(*(*ctxt).input)
                .cur
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xd as ::core::ffi::c_int)
    {
        xmlParseXMLDecl(ctxt);
        if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as ::core::ffi::c_int
            || (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        xmlSkipBlankChars(ctxt);
    } else {
        (*ctxt).version = xmlCharStrdup(XML_DEFAULT_VERSION.as_ptr());
    }
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).startDocument.is_some() && (*ctxt).disableSAX == 0 {
        (*(*ctxt).sax)
            .startDocument
            .expect("non-null function pointer")((*ctxt).userData);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*ctxt).myDoc.is_null()
        && !(*ctxt).input.is_null()
        && !(*(*ctxt).input).buf.is_null()
        && (*(*(*ctxt).input).buf).compressed >= 0 as ::core::ffi::c_int
    {
        (*(*ctxt).myDoc).compression = (*(*(*ctxt).input).buf).compressed;
    }
    xmlParseMisc(ctxt);
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '<' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '!' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'D' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'O' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'C' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'T' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'Y' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'P' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'E' as i32
    {
        (*ctxt).inSubset = 1 as ::core::ffi::c_int;
        xmlParseDocTypeDecl(ctxt);
        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '[' as i32 {
            (*ctxt).instate = XML_PARSER_DTD;
            xmlParseInternalSubset(ctxt);
            if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
        }
        (*ctxt).inSubset = 2 as ::core::ffi::c_int;
        if !(*ctxt).sax.is_null()
            && (*(*ctxt).sax).externalSubset.is_some()
            && (*ctxt).disableSAX == 0
        {
            (*(*ctxt).sax)
                .externalSubset
                .expect("non-null function pointer")(
                (*ctxt).userData,
                (*ctxt).intSubName,
                (*ctxt).extSubSystem,
                (*ctxt).extSubURI,
            );
        }
        if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        (*ctxt).inSubset = 0 as ::core::ffi::c_int;
        xmlCleanSpecialAttr(ctxt);
        (*ctxt).instate = XML_PARSER_PROLOG;
        xmlParseMisc(ctxt);
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int != '<' as i32 {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_DOCUMENT_EMPTY,
            b"Start tag expected, '<' not found\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        (*ctxt).instate = XML_PARSER_CONTENT;
        xmlParseElement(ctxt);
        (*ctxt).instate = XML_PARSER_EPILOG;
        xmlParseMisc(ctxt);
        if (*(*ctxt).input).cur < (*(*ctxt).input).end {
            if (*ctxt).errNo == XML_ERR_OK as ::core::ffi::c_int {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_DOCUMENT_END,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        } else if !(*(*ctxt).input).buf.is_null()
            && !(*(*(*ctxt).input).buf).encoder.is_null()
            && xmlBufIsEmpty((*(*(*ctxt).input).buf).raw) == 0
        {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"Truncated multi-byte sequence at EOF\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*ctxt).instate = XML_PARSER_EOF;
    }
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
        (*(*ctxt).sax)
            .endDocument
            .expect("non-null function pointer")((*ctxt).userData);
    }
    if !(*ctxt).myDoc.is_null()
        && xmlStrEqual(
            (*(*ctxt).myDoc).version,
            b"SAX compatibility mode document\0" as *const u8 as *const ::core::ffi::c_char
                as *mut xmlChar,
        ) != 0
    {
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    if (*ctxt).wellFormed != 0 && !(*ctxt).myDoc.is_null() {
        (*(*ctxt).myDoc).properties |= XML_DOC_WELLFORMED as ::core::ffi::c_int;
        if (*ctxt).valid != 0 {
            (*(*ctxt).myDoc).properties |= XML_DOC_DTDVALID as ::core::ffi::c_int;
        }
        if (*ctxt).nsWellFormed != 0 {
            (*(*ctxt).myDoc).properties |= XML_DOC_NSVALID as ::core::ffi::c_int;
        }
        if (*ctxt).options & XML_PARSE_OLD10 as ::core::ffi::c_int != 0 {
            (*(*ctxt).myDoc).properties |= XML_DOC_OLD10 as ::core::ffi::c_int;
        }
    }
    if (*ctxt).wellFormed == 0 {
        (*ctxt).valid = 0 as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseExtParsedEnt(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    if ctxt.is_null() || (*ctxt).input.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    xmlDetectSAX2(ctxt);
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some() {
        (*(*ctxt).sax)
            .setDocumentLocator
            .expect("non-null function pointer")((*ctxt).userData, __xmlDefaultSAXLocator());
    }
    xmlDetectEncoding(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        xmlFatalErr(
            ctxt,
            XML_ERR_DOCUMENT_EMPTY,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
        < INPUT_CHUNK as ::core::ffi::c_long
    {
        xmlParserGrow(ctxt);
    }
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '<' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '?' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'x' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'm' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'l' as i32
        && (*(*(*ctxt).input)
            .cur
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int
                <= *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                && *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= 0xa as ::core::ffi::c_int
            || *(*(*ctxt).input)
                .cur
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xd as ::core::ffi::c_int)
    {
        xmlParseXMLDecl(ctxt);
        if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        xmlSkipBlankChars(ctxt);
    } else {
        (*ctxt).version = xmlCharStrdup(XML_DEFAULT_VERSION.as_ptr());
    }
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).startDocument.is_some() && (*ctxt).disableSAX == 0 {
        (*(*ctxt).sax)
            .startDocument
            .expect("non-null function pointer")((*ctxt).userData);
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    (*ctxt).instate = XML_PARSER_CONTENT;
    (*ctxt).validate = 0 as ::core::ffi::c_int;
    (*ctxt).loadsubset = 0 as ::core::ffi::c_int;
    (*ctxt).depth = 0 as ::core::ffi::c_int;
    xmlParseContent(ctxt);
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOT_WELL_BALANCED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        xmlFatalErr(
            ctxt,
            XML_ERR_EXTRA_CONTENT,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
        (*(*ctxt).sax)
            .endDocument
            .expect("non-null function pointer")((*ctxt).userData);
    }
    if (*ctxt).wellFormed == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParseLookupChar(
    mut ctxt: xmlParserCtxtPtr,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if (*ctxt).checkIndex == 0 as ::core::ffi::c_long {
        cur = (*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize);
    } else {
        cur = (*(*ctxt).input).cur.offset((*ctxt).checkIndex as isize);
    }
    if memchr(
        cur as *const ::core::ffi::c_void,
        c,
        (*(*ctxt).input).end.offset_from(cur) as ::core::ffi::c_long as size_t,
    )
    .is_null()
    {
        let mut index: size_t =
            (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
        if index > LONG_MAX as size_t {
            (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
            return 1 as ::core::ffi::c_int;
        }
        (*ctxt).checkIndex = index as ::core::ffi::c_long;
        return 0 as ::core::ffi::c_int;
    } else {
        (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn xmlParseLookupString(
    mut ctxt: xmlParserCtxtPtr,
    mut startDelta: size_t,
    mut str: *const ::core::ffi::c_char,
    mut strLen: size_t,
) -> *const xmlChar {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut term: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if (*ctxt).checkIndex == 0 as ::core::ffi::c_long {
        cur = (*(*ctxt).input).cur.offset(startDelta as isize);
    } else {
        cur = (*(*ctxt).input).cur.offset((*ctxt).checkIndex as isize);
    }
    term = strstr(cur as *const ::core::ffi::c_char, str) as *mut xmlChar;
    if term.is_null() {
        let mut end: *const xmlChar = (*(*ctxt).input).end;
        let mut index: size_t = 0;
        if (end.offset_from(cur) as ::core::ffi::c_long as size_t) < strLen {
            end = cur;
        } else {
            end = end.offset(-(strLen.wrapping_sub(1 as size_t) as isize));
        }
        index = end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
        if index > LONG_MAX as size_t {
            (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
            return (*(*ctxt).input).end.offset(-(strLen as isize));
        }
        (*ctxt).checkIndex = index as ::core::ffi::c_long;
    } else {
        (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
    }
    return term;
}
unsafe extern "C" fn xmlParseLookupCharData(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut cur: *const xmlChar = (*(*ctxt).input).cur.offset((*ctxt).checkIndex as isize);
    let mut end: *const xmlChar = (*(*ctxt).input).end;
    let mut index: size_t = 0;
    while cur < end {
        if *cur as ::core::ffi::c_int == '<' as i32 || *cur as ::core::ffi::c_int == '&' as i32 {
            (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
            return 1 as ::core::ffi::c_int;
        }
        cur = cur.offset(1);
    }
    index = cur.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
    if index > LONG_MAX as size_t {
        (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
        return 1 as ::core::ffi::c_int;
    }
    (*ctxt).checkIndex = index as ::core::ffi::c_long;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParseLookupGt(mut ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut end: *const xmlChar = (*(*ctxt).input).end;
    let mut state: ::core::ffi::c_int = (*ctxt).endCheckState;
    let mut index: size_t = 0;
    if (*ctxt).checkIndex == 0 as ::core::ffi::c_long {
        cur = (*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize);
    } else {
        cur = (*(*ctxt).input).cur.offset((*ctxt).checkIndex as isize);
    }
    while cur < end {
        if state != 0 {
            if *cur as ::core::ffi::c_int == state {
                state = 0 as ::core::ffi::c_int;
            }
        } else if *cur as ::core::ffi::c_int == '\'' as i32
            || *cur as ::core::ffi::c_int == '"' as i32
        {
            state = *cur as ::core::ffi::c_int;
        } else if *cur as ::core::ffi::c_int == '>' as i32 {
            (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
            (*ctxt).endCheckState = 0 as ::core::ffi::c_int;
            return 1 as ::core::ffi::c_int;
        }
        cur = cur.offset(1);
    }
    index = cur.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
    if index > LONG_MAX as size_t {
        (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
        (*ctxt).endCheckState = 0 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    (*ctxt).checkIndex = index as ::core::ffi::c_long;
    (*ctxt).endCheckState = state;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParseLookupInternalSubset(
    mut ctxt: xmlParserCtxtPtr,
) -> ::core::ffi::c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut start: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut end: *const xmlChar = (*(*ctxt).input).end;
    let mut state: ::core::ffi::c_int = (*ctxt).endCheckState;
    let mut index: size_t = 0;
    if (*ctxt).checkIndex == 0 as ::core::ffi::c_long {
        cur = (*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize);
    } else {
        cur = (*(*ctxt).input).cur.offset((*ctxt).checkIndex as isize);
    }
    start = cur;
    while cur < end {
        if state == '-' as i32 {
            if *cur as ::core::ffi::c_int == '-' as i32
                && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
                && *cur.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32
            {
                state = 0 as ::core::ffi::c_int;
                cur = cur.offset(3 as ::core::ffi::c_int as isize);
                start = cur;
                continue;
            }
        } else if state == ']' as i32 {
            if *cur as ::core::ffi::c_int == '>' as i32 {
                (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
                (*ctxt).endCheckState = 0 as ::core::ffi::c_int;
                return 1 as ::core::ffi::c_int;
            }
            if *cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                    && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
            {
                state = ' ' as i32;
            } else if *cur as ::core::ffi::c_int != ']' as i32 {
                state = 0 as ::core::ffi::c_int;
                start = cur;
                continue;
            }
        } else if state == ' ' as i32 {
            if *cur as ::core::ffi::c_int == '>' as i32 {
                (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
                (*ctxt).endCheckState = 0 as ::core::ffi::c_int;
                return 1 as ::core::ffi::c_int;
            }
            if !(*cur as ::core::ffi::c_int == 0x20 as ::core::ffi::c_int
                || 0x9 as ::core::ffi::c_int <= *cur as ::core::ffi::c_int
                    && *cur as ::core::ffi::c_int <= 0xa as ::core::ffi::c_int
                || *cur as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
            {
                state = 0 as ::core::ffi::c_int;
                start = cur;
                continue;
            }
        } else if state != 0 as ::core::ffi::c_int {
            if *cur as ::core::ffi::c_int == state {
                state = 0 as ::core::ffi::c_int;
                start = cur.offset(1 as ::core::ffi::c_int as isize);
            }
        } else if *cur as ::core::ffi::c_int == '<' as i32 {
            if *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '!' as i32
                && *cur.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
                && *cur.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
            {
                state = '-' as i32;
                cur = cur.offset(4 as ::core::ffi::c_int as isize);
                start = cur;
                continue;
            }
        } else if *cur as ::core::ffi::c_int == '"' as i32
            || *cur as ::core::ffi::c_int == '\'' as i32
            || *cur as ::core::ffi::c_int == ']' as i32
        {
            state = *cur as ::core::ffi::c_int;
        }
        cur = cur.offset(1);
    }
    if state == 0 as ::core::ffi::c_int || state == '-' as i32 {
        if (cur.offset_from(start) as ::core::ffi::c_long) < 3 as ::core::ffi::c_long {
            cur = start;
        } else {
            cur = cur.offset(-(3 as ::core::ffi::c_int as isize));
        }
    }
    index = cur.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
    if index > LONG_MAX as size_t {
        (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
        (*ctxt).endCheckState = 0 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    (*ctxt).checkIndex = index as ::core::ffi::c_long;
    (*ctxt).endCheckState = state;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlCheckCdataPush(
    mut utf: *const xmlChar,
    mut len: ::core::ffi::c_int,
    mut complete: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ix: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_uchar = 0;
    let mut codepoint: ::core::ffi::c_int = 0;
    if utf.is_null() || len <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    ix = 0 as ::core::ffi::c_int;
    while ix < len {
        c = *utf.offset(ix as isize) as ::core::ffi::c_uchar;
        if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if c as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int {
                ix += 1;
            } else if c as ::core::ffi::c_int == 0xa as ::core::ffi::c_int
                || c as ::core::ffi::c_int == 0xd as ::core::ffi::c_int
                || c as ::core::ffi::c_int == 0x9 as ::core::ffi::c_int
            {
                ix += 1;
            } else {
                return -ix;
            }
        } else if c as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int
        {
            if ix + 2 as ::core::ffi::c_int > len {
                return if complete != 0 { -ix } else { ix };
            }
            if *utf.offset((ix + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
            {
                return -ix;
            }
            codepoint = (*utf.offset(ix as isize) as ::core::ffi::c_int
                & 0x1f as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int;
            codepoint |= *utf.offset((ix + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int;
            if if codepoint < 0x100 as ::core::ffi::c_int {
                (0x9 as ::core::ffi::c_int <= codepoint && codepoint <= 0xa as ::core::ffi::c_int
                    || codepoint == 0xd as ::core::ffi::c_int
                    || 0x20 as ::core::ffi::c_int <= codepoint)
                    as ::core::ffi::c_int
            } else {
                (0x100 as ::core::ffi::c_int <= codepoint
                    && codepoint <= 0xd7ff as ::core::ffi::c_int
                    || 0xe000 as ::core::ffi::c_int <= codepoint
                        && codepoint <= 0xfffd as ::core::ffi::c_int
                    || 0x10000 as ::core::ffi::c_int <= codepoint
                        && codepoint <= 0x10ffff as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } == 0
            {
                return -ix;
            }
            ix += 2 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int
        {
            if ix + 3 as ::core::ffi::c_int > len {
                return if complete != 0 { -ix } else { ix };
            }
            if *utf.offset((ix + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
                || *utf.offset((ix + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                return -ix;
            }
            codepoint = (*utf.offset(ix as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int)
                << 12 as ::core::ffi::c_int;
            codepoint |= (*utf.offset((ix + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int;
            codepoint |= *utf.offset((ix + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int;
            if if codepoint < 0x100 as ::core::ffi::c_int {
                (0x9 as ::core::ffi::c_int <= codepoint && codepoint <= 0xa as ::core::ffi::c_int
                    || codepoint == 0xd as ::core::ffi::c_int
                    || 0x20 as ::core::ffi::c_int <= codepoint)
                    as ::core::ffi::c_int
            } else {
                (0x100 as ::core::ffi::c_int <= codepoint
                    && codepoint <= 0xd7ff as ::core::ffi::c_int
                    || 0xe000 as ::core::ffi::c_int <= codepoint
                        && codepoint <= 0xfffd as ::core::ffi::c_int
                    || 0x10000 as ::core::ffi::c_int <= codepoint
                        && codepoint <= 0x10ffff as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } == 0
            {
                return -ix;
            }
            ix += 3 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int
        {
            if ix + 4 as ::core::ffi::c_int > len {
                return if complete != 0 { -ix } else { ix };
            }
            if *utf.offset((ix + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int
                != 0x80 as ::core::ffi::c_int
                || *utf.offset((ix + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                || *utf.offset((ix + 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
            {
                return -ix;
            }
            codepoint = (*utf.offset(ix as isize) as ::core::ffi::c_int
                & 0x7 as ::core::ffi::c_int)
                << 18 as ::core::ffi::c_int;
            codepoint |= (*utf.offset((ix + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int)
                << 12 as ::core::ffi::c_int;
            codepoint |= (*utf.offset((ix + 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int;
            codepoint |= *utf.offset((ix + 3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int;
            if if codepoint < 0x100 as ::core::ffi::c_int {
                (0x9 as ::core::ffi::c_int <= codepoint && codepoint <= 0xa as ::core::ffi::c_int
                    || codepoint == 0xd as ::core::ffi::c_int
                    || 0x20 as ::core::ffi::c_int <= codepoint)
                    as ::core::ffi::c_int
            } else {
                (0x100 as ::core::ffi::c_int <= codepoint
                    && codepoint <= 0xd7ff as ::core::ffi::c_int
                    || 0xe000 as ::core::ffi::c_int <= codepoint
                        && codepoint <= 0xfffd as ::core::ffi::c_int
                    || 0x10000 as ::core::ffi::c_int <= codepoint
                        && codepoint <= 0x10ffff as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } == 0
            {
                return -ix;
            }
            ix += 4 as ::core::ffi::c_int;
        } else {
            return -ix;
        }
    }
    return ix;
}
unsafe extern "C" fn xmlParseTryOrFinish(
    mut ctxt: xmlParserCtxtPtr,
    mut terminate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut avail: size_t = 0;
    let mut cur: xmlChar = 0;
    let mut next: xmlChar = 0;
    if (*ctxt).input.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*ctxt).input.is_null()
        && (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
            > 4096 as ::core::ffi::c_long
    {
        xmlParserShrink(ctxt);
    }
    loop {
        if !((*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int) {
            current_block = 4092119135571340431;
            break;
        }
        if (*ctxt).errNo != XML_ERR_OK as ::core::ffi::c_int
            && (*ctxt).disableSAX == 1 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        avail =
            (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long as size_t;
        if avail < 1 as size_t {
            current_block = 4092119135571340431;
            break;
        }
        match (*ctxt).instate as ::core::ffi::c_int {
            -1 => {
                current_block = 4092119135571340431;
                break;
            }
            0 => {
                if terminate == 0 && avail < 4 as size_t {
                    current_block = 4092119135571340431;
                    break;
                }
                if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == 0x4c as ::core::ffi::c_int
                    && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0x6f as ::core::ffi::c_int
                    && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0xa7 as ::core::ffi::c_int
                    && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
                        .offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 0x94 as ::core::ffi::c_int
                    && terminate == 0
                    && avail < 200 as size_t
                {
                    current_block = 4092119135571340431;
                    break;
                }
                xmlDetectEncoding(ctxt);
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    current_block = 4092119135571340431;
                    break;
                }
                (*ctxt).instate = XML_PARSER_XML_DECL;
            }
            17 => {
                if terminate == 0 && avail < 2 as size_t {
                    current_block = 4092119135571340431;
                    break;
                }
                cur = *(*(*ctxt).input)
                    .cur
                    .offset(0 as ::core::ffi::c_int as isize);
                next = *(*(*ctxt).input)
                    .cur
                    .offset(1 as ::core::ffi::c_int as isize);
                if cur as ::core::ffi::c_int == '<' as i32
                    && next as ::core::ffi::c_int == '?' as i32
                {
                    if terminate == 0
                        && xmlParseLookupString(
                            ctxt,
                            2 as size_t,
                            b"?>\0" as *const u8 as *const ::core::ffi::c_char,
                            2 as size_t,
                        )
                        .is_null()
                    {
                        current_block = 4092119135571340431;
                        break;
                    }
                    if *(*(*ctxt).input)
                        .cur
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == 'x' as i32
                        && *(*(*ctxt).input)
                            .cur
                            .offset(3 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 'm' as i32
                        && *(*(*ctxt).input)
                            .cur
                            .offset(4 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 'l' as i32
                        && (*(*(*ctxt).input)
                            .cur
                            .offset(5 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == 0x20 as ::core::ffi::c_int
                            || 0x9 as ::core::ffi::c_int
                                <= *(*(*ctxt).input)
                                    .cur
                                    .offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                && *(*(*ctxt).input)
                                    .cur
                                    .offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    <= 0xa as ::core::ffi::c_int
                            || *(*(*ctxt).input)
                                .cur
                                .offset(5 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 0xd as ::core::ffi::c_int)
                    {
                        ret += 5 as ::core::ffi::c_int;
                        xmlParseXMLDecl(ctxt);
                        if (*ctxt).errNo == XML_ERR_UNSUPPORTED_ENCODING as ::core::ffi::c_int {
                            xmlHaltParser(ctxt);
                            return 0 as ::core::ffi::c_int;
                        }
                    } else {
                        (*ctxt).version = xmlCharStrdup(XML_DEFAULT_VERSION.as_ptr());
                    }
                } else {
                    (*ctxt).version = xmlCharStrdup(XML_DEFAULT_VERSION.as_ptr());
                    if (*ctxt).version.is_null() {
                        xmlErrMemory(ctxt, ::core::ptr::null::<::core::ffi::c_char>());
                        continue;
                    }
                }
                if !(*ctxt).sax.is_null() && (*(*ctxt).sax).setDocumentLocator.is_some() {
                    (*(*ctxt).sax)
                        .setDocumentLocator
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        __xmlDefaultSAXLocator(),
                    );
                }
                if !(*ctxt).sax.is_null()
                    && (*(*ctxt).sax).startDocument.is_some()
                    && (*ctxt).disableSAX == 0
                {
                    (*(*ctxt).sax)
                        .startDocument
                        .expect("non-null function pointer")((*ctxt).userData);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    current_block = 4092119135571340431;
                    break;
                }
                (*ctxt).instate = XML_PARSER_MISC;
            }
            6 => {
                let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
                let mut prefix: *const xmlChar = ::core::ptr::null::<xmlChar>();
                let mut URI: *const xmlChar = ::core::ptr::null::<xmlChar>();
                let mut line: ::core::ffi::c_int = (*(*ctxt).input).line;
                let mut nbNs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if terminate == 0 && avail < 2 as size_t {
                    current_block = 4092119135571340431;
                    break;
                }
                cur = *(*(*ctxt).input)
                    .cur
                    .offset(0 as ::core::ffi::c_int as isize);
                if cur as ::core::ffi::c_int != '<' as i32 {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_DOCUMENT_EMPTY,
                        b"Start tag expected, '<' not found\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    xmlHaltParser(ctxt);
                    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
                        (*(*ctxt).sax)
                            .endDocument
                            .expect("non-null function pointer")(
                            (*ctxt).userData
                        );
                    }
                    current_block = 4092119135571340431;
                    break;
                } else {
                    if terminate == 0 && xmlParseLookupGt(ctxt) == 0 {
                        current_block = 4092119135571340431;
                        break;
                    }
                    if (*ctxt).spaceNr == 0 as ::core::ffi::c_int {
                        spacePush(ctxt, -(1 as ::core::ffi::c_int));
                    } else if *(*ctxt).space == -(2 as ::core::ffi::c_int) {
                        spacePush(ctxt, -(1 as ::core::ffi::c_int));
                    } else {
                        spacePush(ctxt, *(*ctxt).space);
                    }
                    if (*ctxt).sax2 != 0 {
                        name =
                            xmlParseStartTag2(ctxt, &raw mut prefix, &raw mut URI, &raw mut nbNs);
                    } else {
                        name = xmlParseStartTag(ctxt);
                    }
                    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int
                    {
                        current_block = 4092119135571340431;
                        break;
                    }
                    if name.is_null() {
                        spacePop(ctxt);
                        xmlHaltParser(ctxt);
                        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
                            (*(*ctxt).sax)
                                .endDocument
                                .expect("non-null function pointer")(
                                (*ctxt).userData
                            );
                        }
                        current_block = 4092119135571340431;
                        break;
                    } else {
                        if (*ctxt).validate != 0
                            && (*ctxt).wellFormed != 0
                            && !(*ctxt).myDoc.is_null()
                            && !(*ctxt).node.is_null()
                            && (*ctxt).node == (*(*ctxt).myDoc).children
                        {
                            (*ctxt).valid &= xmlValidateRoot(&raw mut (*ctxt).vctxt, (*ctxt).myDoc);
                        }
                        if *(*(*ctxt).input).cur as ::core::ffi::c_int == '/' as i32
                            && *(*(*ctxt).input)
                                .cur
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '>' as i32
                        {
                            (*(*ctxt).input).cur = (*(*ctxt).input)
                                .cur
                                .offset(2 as ::core::ffi::c_int as isize);
                            (*(*ctxt).input).col += 2 as ::core::ffi::c_int;
                            if *(*(*ctxt).input).cur as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                xmlParserGrow(ctxt);
                            }
                            if (*ctxt).sax2 != 0 {
                                if !(*ctxt).sax.is_null()
                                    && (*(*ctxt).sax).endElementNs.is_some()
                                    && (*ctxt).disableSAX == 0
                                {
                                    (*(*ctxt).sax)
                                        .endElementNs
                                        .expect("non-null function pointer")(
                                        (*ctxt).userData,
                                        name,
                                        prefix,
                                        URI,
                                    );
                                }
                                if nbNs > 0 as ::core::ffi::c_int {
                                    xmlParserNsPop(ctxt, nbNs);
                                }
                            } else if !(*ctxt).sax.is_null()
                                && (*(*ctxt).sax).endElement.is_some()
                                && (*ctxt).disableSAX == 0
                            {
                                (*(*ctxt).sax)
                                    .endElement
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData, name
                                );
                            }
                            spacePop(ctxt);
                        } else if *(*(*ctxt).input).cur as ::core::ffi::c_int == '>' as i32 {
                            xmlNextChar(ctxt);
                            nameNsPush(ctxt, name, prefix, URI, line, nbNs);
                        } else {
                            xmlFatalErrMsgStr(
                                ctxt,
                                XML_ERR_GT_REQUIRED,
                                b"Couldn't find end of Start Tag %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                name,
                            );
                            nodePop(ctxt);
                            spacePop(ctxt);
                            if nbNs > 0 as ::core::ffi::c_int {
                                xmlParserNsPop(ctxt, nbNs);
                            }
                        }
                        if (*ctxt).instate as ::core::ffi::c_int
                            == XML_PARSER_EOF as ::core::ffi::c_int
                        {
                            current_block = 4092119135571340431;
                            break;
                        }
                        if (*ctxt).nameNr == 0 as ::core::ffi::c_int {
                            (*ctxt).instate = XML_PARSER_EPILOG;
                        } else {
                            (*ctxt).instate = XML_PARSER_CONTENT;
                        }
                    }
                }
            }
            7 => {
                cur = *(*(*ctxt).input)
                    .cur
                    .offset(0 as ::core::ffi::c_int as isize);
                if cur as ::core::ffi::c_int == '<' as i32 {
                    if terminate == 0 && avail < 2 as size_t {
                        current_block = 4092119135571340431;
                        break;
                    }
                    next = *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize);
                    if next as ::core::ffi::c_int == '/' as i32 {
                        (*ctxt).instate = XML_PARSER_END_TAG;
                    } else if next as ::core::ffi::c_int == '?' as i32 {
                        if terminate == 0
                            && xmlParseLookupString(
                                ctxt,
                                2 as size_t,
                                b"?>\0" as *const u8 as *const ::core::ffi::c_char,
                                2 as size_t,
                            )
                            .is_null()
                        {
                            current_block = 4092119135571340431;
                            break;
                        }
                        xmlParsePI(ctxt);
                        if (*ctxt).instate as ::core::ffi::c_int
                            == XML_PARSER_EOF as ::core::ffi::c_int
                        {
                            current_block = 4092119135571340431;
                            break;
                        }
                        (*ctxt).instate = XML_PARSER_CONTENT;
                    } else {
                        if next as ::core::ffi::c_int == '!' as i32 {
                            if terminate == 0 && avail < 3 as size_t {
                                current_block = 4092119135571340431;
                                break;
                            }
                            next = *(*(*ctxt).input)
                                .cur
                                .offset(2 as ::core::ffi::c_int as isize);
                            if next as ::core::ffi::c_int == '-' as i32 {
                                if terminate == 0 && avail < 4 as size_t {
                                    current_block = 4092119135571340431;
                                    break;
                                }
                                if *(*(*ctxt).input)
                                    .cur
                                    .offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '-' as i32
                                {
                                    if terminate == 0
                                        && xmlParseLookupString(
                                            ctxt,
                                            4 as size_t,
                                            b"-->\0" as *const u8 as *const ::core::ffi::c_char,
                                            3 as size_t,
                                        )
                                        .is_null()
                                    {
                                        current_block = 4092119135571340431;
                                        break;
                                    }
                                    xmlParseComment(ctxt);
                                    if (*ctxt).instate as ::core::ffi::c_int
                                        == XML_PARSER_EOF as ::core::ffi::c_int
                                    {
                                        current_block = 4092119135571340431;
                                        break;
                                    }
                                    (*ctxt).instate = XML_PARSER_CONTENT;
                                    continue;
                                }
                            } else if next as ::core::ffi::c_int == '[' as i32 {
                                if terminate == 0 && avail < 9 as size_t {
                                    current_block = 4092119135571340431;
                                    break;
                                }
                                if *(*(*ctxt).input)
                                    .cur
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '[' as i32
                                    && *(*(*ctxt).input)
                                        .cur
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 'C' as i32
                                    && *(*(*ctxt).input)
                                        .cur
                                        .offset(4 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 'D' as i32
                                    && *(*(*ctxt).input)
                                        .cur
                                        .offset(5 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 'A' as i32
                                    && *(*(*ctxt).input)
                                        .cur
                                        .offset(6 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 'T' as i32
                                    && *(*(*ctxt).input)
                                        .cur
                                        .offset(7 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == 'A' as i32
                                    && *(*(*ctxt).input)
                                        .cur
                                        .offset(8 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '[' as i32
                                {
                                    (*(*ctxt).input).cur = (*(*ctxt).input)
                                        .cur
                                        .offset(9 as ::core::ffi::c_int as isize);
                                    (*(*ctxt).input).col += 9 as ::core::ffi::c_int;
                                    if *(*(*ctxt).input).cur as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                    {
                                        xmlParserGrow(ctxt);
                                    }
                                    (*ctxt).instate = XML_PARSER_CDATA_SECTION;
                                    continue;
                                }
                            }
                        }
                        (*ctxt).instate = XML_PARSER_START_TAG;
                    }
                } else if cur as ::core::ffi::c_int == '&' as i32 {
                    if terminate == 0 && xmlParseLookupChar(ctxt, ';' as i32) == 0 {
                        current_block = 4092119135571340431;
                        break;
                    }
                    xmlParseReference(ctxt);
                } else {
                    if avail < XML_PARSER_BIG_BUFFER_SIZE as size_t {
                        if terminate == 0 && xmlParseLookupCharData(ctxt) == 0 {
                            current_block = 4092119135571340431;
                            break;
                        }
                    }
                    (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
                    xmlParseCharDataInternal(ctxt, (terminate == 0) as ::core::ffi::c_int);
                }
            }
            9 => {
                if terminate == 0 && xmlParseLookupChar(ctxt, '>' as i32) == 0 {
                    current_block = 4092119135571340431;
                    break;
                }
                if (*ctxt).sax2 != 0 {
                    xmlParseEndTag2(
                        ctxt,
                        (*ctxt)
                            .pushTab
                            .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize)
                            as *mut xmlStartTag,
                    );
                    nameNsPop(ctxt);
                } else {
                    xmlParseEndTag1(ctxt, 0 as ::core::ffi::c_int);
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    current_block = 4092119135571340431;
                    break;
                }
                if (*ctxt).nameNr == 0 as ::core::ffi::c_int {
                    (*ctxt).instate = XML_PARSER_EPILOG;
                } else {
                    (*ctxt).instate = XML_PARSER_CONTENT;
                }
            }
            8 => {
                let mut term: *const xmlChar = ::core::ptr::null::<xmlChar>();
                if terminate != 0 {
                    term = strstr(
                        (*(*ctxt).input).cur as *const ::core::ffi::c_char,
                        b"]]>\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as *mut xmlChar;
                } else {
                    term = xmlParseLookupString(
                        ctxt,
                        0 as size_t,
                        b"]]>\0" as *const u8 as *const ::core::ffi::c_char,
                        3 as size_t,
                    );
                }
                if term.is_null() {
                    let mut tmp: ::core::ffi::c_int = 0;
                    let mut size: ::core::ffi::c_int = 0;
                    if terminate != 0 {
                        size = (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                            as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                    } else {
                        if avail < (XML_PARSER_BIG_BUFFER_SIZE + 2 as ::core::ffi::c_int) as size_t
                        {
                            current_block = 4092119135571340431;
                            break;
                        }
                        (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
                        size = XML_PARSER_BIG_BUFFER_SIZE;
                    }
                    tmp = xmlCheckCdataPush((*(*ctxt).input).cur, size, 0 as ::core::ffi::c_int);
                    if tmp <= 0 as ::core::ffi::c_int {
                        tmp = -tmp;
                        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(tmp as isize);
                        current_block = 14577264187145228707;
                        break;
                    } else {
                        if !(*ctxt).sax.is_null() && (*ctxt).disableSAX == 0 {
                            if (*(*ctxt).sax).cdataBlock.is_some() {
                                (*(*ctxt).sax)
                                    .cdataBlock
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    (*(*ctxt).input).cur,
                                    tmp,
                                );
                            } else if (*(*ctxt).sax).characters.is_some() {
                                (*(*ctxt).sax)
                                    .characters
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    (*(*ctxt).input).cur,
                                    tmp,
                                );
                            }
                        }
                        if (*ctxt).instate as ::core::ffi::c_int
                            == XML_PARSER_EOF as ::core::ffi::c_int
                        {
                            current_block = 4092119135571340431;
                            break;
                        }
                        let mut skipl: ::core::ffi::c_int = 0;
                        skipl = 0 as ::core::ffi::c_int;
                        while skipl < tmp {
                            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                                (*(*ctxt).input).line += 1;
                                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                            } else {
                                (*(*ctxt).input).col += 1;
                            }
                            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
                            skipl += 1;
                        }
                        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            xmlParserGrow(ctxt);
                        }
                    }
                } else {
                    let mut base: ::core::ffi::c_int = term.offset_from((*(*ctxt).input).cur)
                        as ::core::ffi::c_long
                        as ::core::ffi::c_int;
                    let mut tmp_0: ::core::ffi::c_int = 0;
                    tmp_0 = xmlCheckCdataPush((*(*ctxt).input).cur, base, 1 as ::core::ffi::c_int);
                    if tmp_0 < 0 as ::core::ffi::c_int || tmp_0 != base {
                        tmp_0 = -tmp_0;
                        (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(tmp_0 as isize);
                        current_block = 14577264187145228707;
                        break;
                    } else {
                        if !(*ctxt).sax.is_null()
                            && base == 0 as ::core::ffi::c_int
                            && (*(*ctxt).sax).cdataBlock.is_some()
                            && (*ctxt).disableSAX == 0
                        {
                            if (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                                as ::core::ffi::c_long
                                >= 9 as ::core::ffi::c_long
                                && strncmp(
                                    (*(*ctxt).input)
                                        .cur
                                        .offset(-(9 as ::core::ffi::c_int) as isize)
                                        as *const xmlChar
                                        as *const ::core::ffi::c_char,
                                    b"<![CDATA[\0" as *const u8 as *const ::core::ffi::c_char,
                                    9 as size_t,
                                ) == 0
                            {
                                (*(*ctxt).sax)
                                    .cdataBlock
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    b"\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut xmlChar,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                        } else if !(*ctxt).sax.is_null()
                            && base > 0 as ::core::ffi::c_int
                            && (*ctxt).disableSAX == 0
                        {
                            if (*(*ctxt).sax).cdataBlock.is_some() {
                                (*(*ctxt).sax)
                                    .cdataBlock
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    (*(*ctxt).input).cur,
                                    base,
                                );
                            } else if (*(*ctxt).sax).characters.is_some() {
                                (*(*ctxt).sax)
                                    .characters
                                    .expect("non-null function pointer")(
                                    (*ctxt).userData,
                                    (*(*ctxt).input).cur,
                                    base,
                                );
                            }
                        }
                        if (*ctxt).instate as ::core::ffi::c_int
                            == XML_PARSER_EOF as ::core::ffi::c_int
                        {
                            current_block = 4092119135571340431;
                            break;
                        }
                        let mut skipl_0: ::core::ffi::c_int = 0;
                        skipl_0 = 0 as ::core::ffi::c_int;
                        while skipl_0 < base + 3 as ::core::ffi::c_int {
                            if *(*(*ctxt).input).cur as ::core::ffi::c_int == '\n' as i32 {
                                (*(*ctxt).input).line += 1;
                                (*(*ctxt).input).col = 1 as ::core::ffi::c_int;
                            } else {
                                (*(*ctxt).input).col += 1;
                            }
                            (*(*ctxt).input).cur = (*(*ctxt).input).cur.offset(1);
                            skipl_0 += 1;
                        }
                        if *(*(*ctxt).input).cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            xmlParserGrow(ctxt);
                        }
                        (*ctxt).instate = XML_PARSER_CONTENT;
                    }
                }
            }
            1 | 4 | 14 => {
                xmlSkipBlankChars(ctxt);
                avail = (*(*ctxt).input).end.offset_from((*(*ctxt).input).cur)
                    as ::core::ffi::c_long as size_t;
                if avail < 1 as size_t {
                    current_block = 4092119135571340431;
                    break;
                }
                if *(*(*ctxt).input)
                    .cur
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '<' as i32
                {
                    if terminate == 0 && avail < 2 as size_t {
                        current_block = 4092119135571340431;
                        break;
                    }
                    next = *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize);
                    if next as ::core::ffi::c_int == '?' as i32 {
                        if terminate == 0
                            && xmlParseLookupString(
                                ctxt,
                                2 as size_t,
                                b"?>\0" as *const u8 as *const ::core::ffi::c_char,
                                2 as size_t,
                            )
                            .is_null()
                        {
                            current_block = 4092119135571340431;
                            break;
                        }
                        xmlParsePI(ctxt);
                        if (*ctxt).instate as ::core::ffi::c_int
                            == XML_PARSER_EOF as ::core::ffi::c_int
                        {
                            current_block = 4092119135571340431;
                            break;
                        } else {
                            continue;
                        }
                    } else if next as ::core::ffi::c_int == '!' as i32 {
                        if terminate == 0 && avail < 3 as size_t {
                            current_block = 4092119135571340431;
                            break;
                        }
                        if *(*(*ctxt).input)
                            .cur
                            .offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '-' as i32
                        {
                            if terminate == 0 && avail < 4 as size_t {
                                current_block = 4092119135571340431;
                                break;
                            }
                            if *(*(*ctxt).input)
                                .cur
                                .offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '-' as i32
                            {
                                if terminate == 0
                                    && xmlParseLookupString(
                                        ctxt,
                                        4 as size_t,
                                        b"-->\0" as *const u8 as *const ::core::ffi::c_char,
                                        3 as size_t,
                                    )
                                    .is_null()
                                {
                                    current_block = 4092119135571340431;
                                    break;
                                }
                                xmlParseComment(ctxt);
                                if (*ctxt).instate as ::core::ffi::c_int
                                    == XML_PARSER_EOF as ::core::ffi::c_int
                                {
                                    current_block = 4092119135571340431;
                                    break;
                                } else {
                                    continue;
                                }
                            }
                        } else if (*ctxt).instate as ::core::ffi::c_int
                            == XML_PARSER_MISC as ::core::ffi::c_int
                        {
                            if terminate == 0 && avail < 9 as size_t {
                                current_block = 4092119135571340431;
                                break;
                            }
                            if *(*(*ctxt).input)
                                .cur
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == 'D' as i32
                                && *(*(*ctxt).input)
                                    .cur
                                    .offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'O' as i32
                                && *(*(*ctxt).input)
                                    .cur
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'C' as i32
                                && *(*(*ctxt).input)
                                    .cur
                                    .offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'T' as i32
                                && *(*(*ctxt).input)
                                    .cur
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'Y' as i32
                                && *(*(*ctxt).input)
                                    .cur
                                    .offset(7 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'P' as i32
                                && *(*(*ctxt).input)
                                    .cur
                                    .offset(8 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 'E' as i32
                            {
                                if terminate == 0 && xmlParseLookupGt(ctxt) == 0 {
                                    current_block = 4092119135571340431;
                                    break;
                                }
                                (*ctxt).inSubset = 1 as ::core::ffi::c_int;
                                xmlParseDocTypeDecl(ctxt);
                                if (*ctxt).instate as ::core::ffi::c_int
                                    == XML_PARSER_EOF as ::core::ffi::c_int
                                {
                                    current_block = 4092119135571340431;
                                    break;
                                }
                                if *(*(*ctxt).input).cur as ::core::ffi::c_int == '[' as i32 {
                                    (*ctxt).instate = XML_PARSER_DTD;
                                    continue;
                                } else {
                                    (*ctxt).inSubset = 2 as ::core::ffi::c_int;
                                    if !(*ctxt).sax.is_null()
                                        && (*ctxt).disableSAX == 0
                                        && (*(*ctxt).sax).externalSubset.is_some()
                                    {
                                        (*(*ctxt).sax)
                                            .externalSubset
                                            .expect("non-null function pointer")(
                                            (*ctxt).userData,
                                            (*ctxt).intSubName,
                                            (*ctxt).extSubSystem,
                                            (*ctxt).extSubURI,
                                        );
                                    }
                                    (*ctxt).inSubset = 0 as ::core::ffi::c_int;
                                    xmlCleanSpecialAttr(ctxt);
                                    if (*ctxt).instate as ::core::ffi::c_int
                                        == XML_PARSER_EOF as ::core::ffi::c_int
                                    {
                                        current_block = 4092119135571340431;
                                        break;
                                    }
                                    (*ctxt).instate = XML_PARSER_PROLOG;
                                    continue;
                                }
                            }
                        }
                    }
                }
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EPILOG as ::core::ffi::c_int
                {
                    if (*ctxt).errNo == XML_ERR_OK as ::core::ffi::c_int {
                        xmlFatalErr(
                            ctxt,
                            XML_ERR_DOCUMENT_END,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                    (*ctxt).instate = XML_PARSER_EOF;
                    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
                        (*(*ctxt).sax)
                            .endDocument
                            .expect("non-null function pointer")(
                            (*ctxt).userData
                        );
                    }
                } else {
                    (*ctxt).instate = XML_PARSER_START_TAG;
                }
            }
            3 => {
                if terminate == 0 && xmlParseLookupInternalSubset(ctxt) == 0 {
                    current_block = 4092119135571340431;
                    break;
                }
                xmlParseInternalSubset(ctxt);
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    current_block = 4092119135571340431;
                    break;
                }
                (*ctxt).inSubset = 2 as ::core::ffi::c_int;
                if !(*ctxt).sax.is_null()
                    && (*ctxt).disableSAX == 0
                    && (*(*ctxt).sax).externalSubset.is_some()
                {
                    (*(*ctxt).sax)
                        .externalSubset
                        .expect("non-null function pointer")(
                        (*ctxt).userData,
                        (*ctxt).intSubName,
                        (*ctxt).extSubSystem,
                        (*ctxt).extSubURI,
                    );
                }
                (*ctxt).inSubset = 0 as ::core::ffi::c_int;
                xmlCleanSpecialAttr(ctxt);
                if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
                    current_block = 4092119135571340431;
                    break;
                }
                (*ctxt).instate = XML_PARSER_PROLOG;
            }
            _ => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"PP: internal error\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*ctxt).instate = XML_PARSER_EOF;
            }
        }
    }
    match current_block {
        4092119135571340431 => return ret,
        _ => {
            if ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long)
                < 4 as ::core::ffi::c_long
            {
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Input is not proper UTF-8, indicate encoding !\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
            } else {
                let mut buffer: [::core::ffi::c_char; 150] = [0; 150];
                snprintf(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    149 as size_t,
                    b"Bytes: 0x%02X 0x%02X 0x%02X 0x%02X\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *(*(*ctxt).input)
                        .cur
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int,
                    *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int,
                    *(*(*ctxt).input)
                        .cur
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int,
                    *(*(*ctxt).input)
                        .cur
                        .offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int,
                );
                __xmlErrEncoding(
                    ctxt,
                    XML_ERR_INVALID_CHAR,
                    b"Input is not proper UTF-8, indicate encoding !\n%s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &raw mut buffer as *mut ::core::ffi::c_char as *mut xmlChar,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            return 0 as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseChunk(
    mut ctxt: xmlParserCtxtPtr,
    mut chunk: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut terminate: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut end_in_lf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ctxt.is_null() {
        return XML_ERR_INTERNAL_ERROR as ::core::ffi::c_int;
    }
    if (*ctxt).errNo != XML_ERR_OK as ::core::ffi::c_int
        && (*ctxt).disableSAX == 1 as ::core::ffi::c_int
    {
        return (*ctxt).errNo;
    }
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).input.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*ctxt).progressive = 1 as ::core::ffi::c_int;
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_START as ::core::ffi::c_int {
        xmlDetectSAX2(ctxt);
    }
    if size > 0 as ::core::ffi::c_int
        && !chunk.is_null()
        && terminate == 0
        && *chunk.offset((size - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '\r' as i32
    {
        end_in_lf = 1 as ::core::ffi::c_int;
        size -= 1;
    }
    if size > 0 as ::core::ffi::c_int
        && !chunk.is_null()
        && !(*ctxt).input.is_null()
        && !(*(*ctxt).input).buf.is_null()
        && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
    {
        let mut pos: size_t = (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
            as ::core::ffi::c_long as size_t;
        let mut res: ::core::ffi::c_int = 0;
        res = xmlParserInputBufferPush((*(*ctxt).input).buf, size, chunk);
        xmlBufUpdateInput((*(*(*ctxt).input).buf).buffer, (*ctxt).input, pos);
        if res < 0 as ::core::ffi::c_int {
            xmlFatalErr(
                ctxt,
                (*(*(*ctxt).input).buf).error as xmlParserErrors,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlHaltParser(ctxt);
            return (*ctxt).errNo;
        }
    }
    xmlParseTryOrFinish(ctxt, terminate);
    if (*ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
        return (*ctxt).errNo;
    }
    if !(*ctxt).input.is_null()
        && ((*(*ctxt).input).end.offset_from((*(*ctxt).input).cur) as ::core::ffi::c_long
            > XML_MAX_LOOKUP_LIMIT as ::core::ffi::c_long
            || (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                > XML_MAX_LOOKUP_LIMIT as ::core::ffi::c_long)
        && (*ctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"Huge input lookup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        xmlHaltParser(ctxt);
    }
    if (*ctxt).errNo != XML_ERR_OK as ::core::ffi::c_int
        && (*ctxt).disableSAX == 1 as ::core::ffi::c_int
    {
        return (*ctxt).errNo;
    }
    if end_in_lf == 1 as ::core::ffi::c_int
        && !(*ctxt).input.is_null()
        && !(*(*ctxt).input).buf.is_null()
    {
        let mut pos_0: size_t = (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
            as ::core::ffi::c_long as size_t;
        let mut res_0: ::core::ffi::c_int = 0;
        res_0 = xmlParserInputBufferPush(
            (*(*ctxt).input).buf,
            1 as ::core::ffi::c_int,
            b"\r\0" as *const u8 as *const ::core::ffi::c_char,
        );
        xmlBufUpdateInput((*(*(*ctxt).input).buf).buffer, (*ctxt).input, pos_0);
        if res_0 < 0 as ::core::ffi::c_int {
            xmlFatalErr(
                ctxt,
                (*(*(*ctxt).input).buf).error as xmlParserErrors,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlHaltParser(ctxt);
            return (*ctxt).errNo;
        }
    }
    if terminate != 0 {
        if (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int
            && (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EPILOG as ::core::ffi::c_int
        {
            if (*ctxt).nameNr > 0 as ::core::ffi::c_int {
                let mut name: *const xmlChar = *(*ctxt)
                    .nameTab
                    .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize);
                let mut line: ::core::ffi::c_int = (*(*ctxt)
                    .pushTab
                    .offset(((*ctxt).nameNr - 1 as ::core::ffi::c_int) as isize))
                .line;
                xmlFatalErrMsgStrIntStr(
                    ctxt,
                    XML_ERR_TAG_NOT_FINISHED,
                    b"Premature end of data in tag %s line %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    name,
                    line,
                    ::core::ptr::null::<xmlChar>(),
                );
            } else if (*ctxt).instate as ::core::ffi::c_int
                == XML_PARSER_START as ::core::ffi::c_int
            {
                xmlFatalErr(
                    ctxt,
                    XML_ERR_DOCUMENT_EMPTY,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            } else {
                xmlFatalErrMsg(
                    ctxt,
                    XML_ERR_DOCUMENT_EMPTY,
                    b"Start tag expected, '<' not found\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        } else if !(*(*ctxt).input).buf.is_null()
            && !(*(*(*ctxt).input).buf).encoder.is_null()
            && xmlBufIsEmpty((*(*(*ctxt).input).buf).raw) == 0
        {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_INVALID_CHAR,
                b"Truncated multi-byte sequence at EOF\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*ctxt).instate as ::core::ffi::c_int != XML_PARSER_EOF as ::core::ffi::c_int {
            if !(*ctxt).sax.is_null() && (*(*ctxt).sax).endDocument.is_some() {
                (*(*ctxt).sax)
                    .endDocument
                    .expect("non-null function pointer")((*ctxt).userData);
            }
        }
        (*ctxt).instate = XML_PARSER_EOF;
    }
    if (*ctxt).wellFormed == 0 as ::core::ffi::c_int {
        return (*ctxt).errNo as xmlParserErrors as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreatePushParserCtxt(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut ::core::ffi::c_void,
    mut chunk: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut filename: *const ::core::ffi::c_char,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    buf = xmlAllocParserInputBuffer(XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    ctxt = xmlNewSAXParserCtxt(sax as *const xmlSAXHandler, user_data);
    if ctxt.is_null() {
        xmlErrMemory(
            ::core::ptr::null_mut::<xmlParserCtxt>(),
            b"creating parser: out of memory\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        xmlFreeParserInputBuffer(buf);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    (*ctxt).dictNames = 1 as ::core::ffi::c_int;
    if filename.is_null() {
        (*ctxt).directory = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        (*ctxt).directory = xmlParserGetDirectory(filename);
    }
    inputStream = xmlNewInputStream(ctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt(ctxt);
        xmlFreeParserInputBuffer(buf);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    if filename.is_null() {
        (*inputStream).filename = ::core::ptr::null::<::core::ffi::c_char>();
    } else {
        (*inputStream).filename =
            xmlCanonicPath(filename as *const xmlChar) as *mut ::core::ffi::c_char;
        if (*inputStream).filename.is_null() {
            xmlFreeInputStream(inputStream);
            xmlFreeParserCtxt(ctxt);
            xmlFreeParserInputBuffer(buf);
            return ::core::ptr::null_mut::<xmlParserCtxt>();
        }
    }
    (*inputStream).buf = buf;
    xmlBufResetInput((*(*inputStream).buf).buffer, inputStream);
    inputPush(ctxt, inputStream);
    if size != 0 as ::core::ffi::c_int
        && !chunk.is_null()
        && !(*ctxt).input.is_null()
        && !(*(*ctxt).input).buf.is_null()
    {
        let mut pos: size_t = (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
            as ::core::ffi::c_long as size_t;
        let mut res: ::core::ffi::c_int = 0;
        res = xmlParserInputBufferPush((*(*ctxt).input).buf, size, chunk);
        xmlBufUpdateInput((*(*(*ctxt).input).buf).buffer, (*ctxt).input, pos);
        if res < 0 as ::core::ffi::c_int {
            xmlFatalErr(
                ctxt,
                (*(*(*ctxt).input).buf).error as xmlParserErrors,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlHaltParser(ctxt);
        }
    }
    return ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStopParser(mut ctxt: xmlParserCtxtPtr) {
    if ctxt.is_null() {
        return;
    }
    xmlHaltParser(ctxt);
    (*ctxt).errNo = XML_ERR_USER_STOP as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateIOParserCtxt(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut ::core::ffi::c_void,
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut ::core::ffi::c_void,
    mut enc: xmlCharEncoding,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if ioread.is_none() {
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    buf = xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, enc);
    if buf.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    ctxt = xmlNewSAXParserCtxt(sax as *const xmlSAXHandler, user_data);
    if ctxt.is_null() {
        xmlFreeParserInputBuffer(buf);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    inputStream = xmlNewIOInputStream(ctxt, buf, enc);
    if inputStream.is_null() {
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    inputPush(ctxt, inputStream);
    return ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn xmlIOParseDTD(
    mut sax: xmlSAXHandlerPtr,
    mut input: xmlParserInputBufferPtr,
    mut enc: xmlCharEncoding,
) -> xmlDtdPtr {
    let mut ret: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut pinput: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    ctxt = xmlNewSAXParserCtxt(sax as *const xmlSAXHandler, NULL);
    if ctxt.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    (*ctxt).options |= XML_PARSE_DTDLOAD as ::core::ffi::c_int;
    xmlDetectSAX2(ctxt);
    pinput = xmlNewIOInputStream(ctxt, input, XML_CHAR_ENCODING_NONE);
    if pinput.is_null() {
        xmlFreeParserInputBuffer(input);
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    if xmlPushInput(ctxt, pinput) < 0 as ::core::ffi::c_int {
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    if enc as ::core::ffi::c_int != XML_CHAR_ENCODING_NONE as ::core::ffi::c_int {
        xmlSwitchEncoding(ctxt, enc);
    }
    (*ctxt).inSubset = 2 as ::core::ffi::c_int;
    (*ctxt).myDoc = xmlNewDoc(b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
    if (*ctxt).myDoc.is_null() {
        xmlErrMemory(
            ctxt,
            b"New Doc failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    (*(*ctxt).myDoc).properties = XML_DOC_INTERNAL as ::core::ffi::c_int;
    (*(*ctxt).myDoc).extSubset = xmlNewDtd(
        (*ctxt).myDoc,
        b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) as *mut _xmlDtd;
    xmlDetectEncoding(ctxt);
    xmlParseExternalSubset(
        ctxt,
        b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    );
    if !(*ctxt).myDoc.is_null() {
        if (*ctxt).wellFormed != 0 {
            ret = (*(*ctxt).myDoc).extSubset as xmlDtdPtr;
            (*(*ctxt).myDoc).extSubset = ::core::ptr::null_mut::<_xmlDtd>();
            if !ret.is_null() {
                let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                (*ret).doc = ::core::ptr::null_mut::<_xmlDoc>();
                tmp = (*ret).children as xmlNodePtr;
                while !tmp.is_null() {
                    (*tmp).doc = ::core::ptr::null_mut::<_xmlDoc>();
                    tmp = (*tmp).next as xmlNodePtr;
                }
            }
        } else {
            ret = ::core::ptr::null_mut::<xmlDtd>();
        }
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseDTD(
    mut sax: xmlSAXHandlerPtr,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlDtdPtr {
    let mut ret: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut systemIdCanonic: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if ExternalID.is_null() && SystemID.is_null() {
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    ctxt = xmlNewSAXParserCtxt(sax as *const xmlSAXHandler, NULL);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    (*ctxt).options |= XML_PARSE_DTDLOAD as ::core::ffi::c_int;
    systemIdCanonic = xmlCanonicPath(SystemID);
    if !SystemID.is_null() && systemIdCanonic.is_null() {
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).resolveEntity.is_some() {
        input = (*(*ctxt).sax)
            .resolveEntity
            .expect("non-null function pointer")(
            (*ctxt).userData, ExternalID, systemIdCanonic
        );
    }
    if input.is_null() {
        xmlFreeParserCtxt(ctxt);
        if !systemIdCanonic.is_null() {
            xmlFree.expect("non-null function pointer")(
                systemIdCanonic as *mut ::core::ffi::c_void,
            );
        }
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    if xmlPushInput(ctxt, input) < 0 as ::core::ffi::c_int {
        xmlFreeParserCtxt(ctxt);
        if !systemIdCanonic.is_null() {
            xmlFree.expect("non-null function pointer")(
                systemIdCanonic as *mut ::core::ffi::c_void,
            );
        }
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    xmlDetectEncoding(ctxt);
    if (*input).filename.is_null() {
        (*input).filename = systemIdCanonic as *mut ::core::ffi::c_char;
    } else {
        xmlFree.expect("non-null function pointer")(systemIdCanonic as *mut ::core::ffi::c_void);
    }
    (*ctxt).inSubset = 2 as ::core::ffi::c_int;
    (*ctxt).myDoc = xmlNewDoc(b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
    if (*ctxt).myDoc.is_null() {
        xmlErrMemory(
            ctxt,
            b"New Doc failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    (*(*ctxt).myDoc).properties = XML_DOC_INTERNAL as ::core::ffi::c_int;
    (*(*ctxt).myDoc).extSubset = xmlNewDtd(
        (*ctxt).myDoc,
        b"none\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ExternalID,
        SystemID,
    ) as *mut _xmlDtd;
    xmlParseExternalSubset(ctxt, ExternalID, SystemID);
    if !(*ctxt).myDoc.is_null() {
        if (*ctxt).wellFormed != 0 {
            ret = (*(*ctxt).myDoc).extSubset as xmlDtdPtr;
            (*(*ctxt).myDoc).extSubset = ::core::ptr::null_mut::<_xmlDtd>();
            if !ret.is_null() {
                let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                (*ret).doc = ::core::ptr::null_mut::<_xmlDoc>();
                tmp = (*ret).children as xmlNodePtr;
                while !tmp.is_null() {
                    (*tmp).doc = ::core::ptr::null_mut::<_xmlDoc>();
                    tmp = (*tmp).next as xmlNodePtr;
                }
            }
        } else {
            ret = ::core::ptr::null_mut::<xmlDtd>();
        }
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseDTD(
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlDtdPtr {
    return xmlSAXParseDTD(
        ::core::ptr::null_mut::<xmlSAXHandler>(),
        ExternalID,
        SystemID,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseCtxtExternalEntity(
    mut ctx: xmlParserCtxtPtr,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> ::core::ffi::c_int {
    let mut userData: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctx).userData == ctx as *mut ::core::ffi::c_void {
        userData = NULL;
    } else {
        userData = (*ctx).userData;
    }
    return xmlParseExternalEntityPrivate(
        (*ctx).myDoc,
        ctx,
        (*ctx).sax as xmlSAXHandlerPtr,
        userData,
        (*ctx).depth + 1 as ::core::ffi::c_int,
        URL,
        ID,
        lst,
    ) as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlParseExternalEntityPrivate(
    mut doc: xmlDocPtr,
    mut oldctxt: xmlParserCtxtPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut ::core::ffi::c_void,
    mut depth: ::core::ffi::c_int,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut list: *mut xmlNodePtr,
) -> xmlParserErrors {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut newDoc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut newRoot: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: xmlParserErrors = XML_ERR_OK;
    if depth > 40 as ::core::ffi::c_int
        && (oldctxt.is_null()
            || (*oldctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        || depth > 100 as ::core::ffi::c_int
    {
        xmlFatalErrMsg(
            oldctxt,
            XML_ERR_ENTITY_LOOP,
            b"Maximum entity nesting depth exceeded\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return XML_ERR_ENTITY_LOOP;
    }
    if !list.is_null() {
        *list = ::core::ptr::null_mut::<xmlNode>();
    }
    if URL.is_null() && ID.is_null() {
        return XML_ERR_INTERNAL_ERROR;
    }
    if doc.is_null() {
        return XML_ERR_INTERNAL_ERROR;
    }
    ctxt = xmlCreateEntityParserCtxtInternal(
        sax,
        user_data,
        URL,
        ID,
        ::core::ptr::null::<xmlChar>(),
        oldctxt,
    );
    if ctxt.is_null() {
        return XML_WAR_UNDECLARED_ENTITY;
    }
    if !oldctxt.is_null() {
        (*ctxt).nbErrors = (*oldctxt).nbErrors;
        (*ctxt).nbWarnings = (*oldctxt).nbWarnings;
    }
    xmlDetectSAX2(ctxt);
    newDoc = xmlNewDoc(b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
    if newDoc.is_null() {
        xmlFreeParserCtxt(ctxt);
        return XML_ERR_INTERNAL_ERROR;
    }
    (*newDoc).properties = XML_DOC_INTERNAL as ::core::ffi::c_int;
    if !doc.is_null() {
        (*newDoc).intSubset = (*doc).intSubset;
        (*newDoc).extSubset = (*doc).extSubset;
        if !(*doc).dict.is_null() {
            (*newDoc).dict = (*doc).dict;
            xmlDictReference((*newDoc).dict as xmlDictPtr);
        }
        if !(*doc).URL.is_null() {
            (*newDoc).URL = xmlStrdup((*doc).URL);
        }
    }
    newRoot = xmlNewDocNode(
        newDoc,
        ::core::ptr::null_mut::<xmlNs>(),
        b"pseudoroot\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ::core::ptr::null::<xmlChar>(),
    );
    if newRoot.is_null() {
        if !sax.is_null() {
            xmlFreeParserCtxt(ctxt);
        }
        (*newDoc).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
        (*newDoc).extSubset = ::core::ptr::null_mut::<_xmlDtd>();
        xmlFreeDoc(newDoc);
        return XML_ERR_INTERNAL_ERROR;
    }
    xmlAddChild(newDoc as xmlNodePtr, newRoot);
    nodePush(ctxt, (*newDoc).children as xmlNodePtr);
    if doc.is_null() {
        (*ctxt).myDoc = newDoc;
    } else {
        (*ctxt).myDoc = doc;
        (*newRoot).doc = doc as *mut _xmlDoc;
    }
    xmlDetectEncoding(ctxt);
    if *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        == '<' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '?' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'x' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'm' as i32
        && *((*(*ctxt).input).cur as *mut ::core::ffi::c_uchar)
            .offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'l' as i32
        && (*(*(*ctxt).input)
            .cur
            .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0x20 as ::core::ffi::c_int
            || 0x9 as ::core::ffi::c_int
                <= *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                && *(*(*ctxt).input)
                    .cur
                    .offset(5 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= 0xa as ::core::ffi::c_int
            || *(*(*ctxt).input)
                .cur
                .offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xd as ::core::ffi::c_int)
    {
        xmlParseTextDecl(ctxt);
        if xmlStrEqual(
            (*oldctxt).version,
            b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ) != 0
            && xmlStrEqual(
                (*(*ctxt).input).version,
                b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            ) == 0
        {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_VERSION_MISMATCH,
                b"Version mismatch between document and entity\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    (*ctxt).instate = XML_PARSER_CONTENT;
    (*ctxt).depth = depth;
    if !oldctxt.is_null() {
        (*ctxt)._private = (*oldctxt)._private;
        (*ctxt).loadsubset = (*oldctxt).loadsubset;
        (*ctxt).validate = (*oldctxt).validate;
        (*ctxt).valid = (*oldctxt).valid;
        (*ctxt).replaceEntities = (*oldctxt).replaceEntities;
        if (*oldctxt).validate != 0 {
            (*ctxt).vctxt.error = (*oldctxt).vctxt.error;
            (*ctxt).vctxt.warning = (*oldctxt).vctxt.warning;
            (*ctxt).vctxt.userData = (*oldctxt).vctxt.userData;
            (*ctxt).vctxt.flags = (*oldctxt).vctxt.flags;
        }
        (*ctxt).external = (*oldctxt).external;
        if !(*ctxt).dict.is_null() {
            xmlDictFree((*ctxt).dict);
        }
        (*ctxt).dict = (*oldctxt).dict;
        (*ctxt).str_xml = xmlDictLookup(
            (*ctxt).dict,
            b"xml\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            3 as ::core::ffi::c_int,
        );
        (*ctxt).str_xmlns = xmlDictLookup(
            (*ctxt).dict,
            b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            5 as ::core::ffi::c_int,
        );
        (*ctxt).str_xml_ns =
            xmlDictLookup((*ctxt).dict, XML_XML_NAMESPACE, 36 as ::core::ffi::c_int);
        (*ctxt).dictNames = (*oldctxt).dictNames;
        (*ctxt).attsDefault = (*oldctxt).attsDefault;
        (*ctxt).attsSpecial = (*oldctxt).attsSpecial;
        (*ctxt).linenumbers = (*oldctxt).linenumbers;
        (*ctxt).record_info = (*oldctxt).record_info;
        (*ctxt).node_seq.maximum = (*oldctxt).node_seq.maximum;
        (*ctxt).node_seq.length = (*oldctxt).node_seq.length;
        (*ctxt).node_seq.buffer = (*oldctxt).node_seq.buffer;
    } else {
        (*ctxt)._private = NULL;
        (*ctxt).validate = 0 as ::core::ffi::c_int;
        (*ctxt).external = 2 as ::core::ffi::c_int;
        (*ctxt).loadsubset = 0 as ::core::ffi::c_int;
    }
    xmlParseContent(ctxt);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOT_WELL_BALANCED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        xmlFatalErr(
            ctxt,
            XML_ERR_EXTRA_CONTENT,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if (*ctxt).node != (*newDoc).children {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOT_WELL_BALANCED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if (*ctxt).wellFormed == 0 {
        ret = (*ctxt).errNo as xmlParserErrors;
        if !oldctxt.is_null() {
            (*oldctxt).errNo = (*ctxt).errNo;
            (*oldctxt).wellFormed = 0 as ::core::ffi::c_int;
            xmlCopyError(&raw mut (*ctxt).lastError, &raw mut (*oldctxt).lastError);
        }
    } else {
        if !list.is_null() {
            let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            cur = (*(*newDoc).children).children as xmlNodePtr;
            *list = cur;
            while !cur.is_null() {
                (*cur).parent = ::core::ptr::null_mut::<_xmlNode>();
                cur = (*cur).next as xmlNodePtr;
            }
            (*(*newDoc).children).children = ::core::ptr::null_mut::<_xmlNode>();
        }
        ret = XML_ERR_OK;
    }
    if !(*ctxt).input.is_null() && !oldctxt.is_null() {
        let mut consumed: ::core::ffi::c_ulong = (*(*ctxt).input).consumed;
        xmlSaturatedAddSizeT(
            &raw mut consumed,
            (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as ::core::ffi::c_long
                as ::core::ffi::c_ulong,
        );
        xmlSaturatedAdd(&raw mut (*oldctxt).sizeentities, consumed);
        xmlSaturatedAdd(&raw mut (*oldctxt).sizeentities, (*ctxt).sizeentities);
        xmlSaturatedAdd(&raw mut (*oldctxt).sizeentcopy, consumed);
        xmlSaturatedAdd(&raw mut (*oldctxt).sizeentcopy, (*ctxt).sizeentcopy);
    }
    if !oldctxt.is_null() {
        (*ctxt).dict = ::core::ptr::null_mut::<xmlDict>();
        (*ctxt).attsDefault = ::core::ptr::null_mut::<xmlHashTable>();
        (*ctxt).attsSpecial = ::core::ptr::null_mut::<xmlHashTable>();
        (*oldctxt).nbErrors = (*ctxt).nbErrors;
        (*oldctxt).nbWarnings = (*ctxt).nbWarnings;
        (*oldctxt).validate = (*ctxt).validate;
        (*oldctxt).valid = (*ctxt).valid;
        (*oldctxt).node_seq.maximum = (*ctxt).node_seq.maximum;
        (*oldctxt).node_seq.length = (*ctxt).node_seq.length;
        (*oldctxt).node_seq.buffer = (*ctxt).node_seq.buffer;
    }
    (*ctxt).node_seq.maximum = 0 as ::core::ffi::c_ulong;
    (*ctxt).node_seq.length = 0 as ::core::ffi::c_ulong;
    (*ctxt).node_seq.buffer = ::core::ptr::null_mut::<xmlParserNodeInfo>();
    xmlFreeParserCtxt(ctxt);
    (*newDoc).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
    (*newDoc).extSubset = ::core::ptr::null_mut::<_xmlDtd>();
    xmlFreeDoc(newDoc);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseExternalEntity(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut ::core::ffi::c_void,
    mut depth: ::core::ffi::c_int,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> ::core::ffi::c_int {
    return xmlParseExternalEntityPrivate(
        doc,
        ::core::ptr::null_mut::<xmlParserCtxt>(),
        sax,
        user_data,
        depth,
        URL,
        ID,
        lst,
    ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseBalancedChunkMemory(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut ::core::ffi::c_void,
    mut depth: ::core::ffi::c_int,
    mut string: *const xmlChar,
    mut lst: *mut xmlNodePtr,
) -> ::core::ffi::c_int {
    return xmlParseBalancedChunkMemoryRecover(
        doc,
        sax,
        user_data,
        depth,
        string,
        lst,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn xmlParseBalancedChunkMemoryInternal(
    mut oldctxt: xmlParserCtxtPtr,
    mut string: *const xmlChar,
    mut user_data: *mut ::core::ffi::c_void,
    mut lst: *mut xmlNodePtr,
) -> xmlParserErrors {
    let mut current_block: u64;
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut newDoc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut newRoot: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut oldsax: xmlSAXHandlerPtr = ::core::ptr::null_mut::<xmlSAXHandler>();
    let mut content: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut last: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: xmlParserErrors = XML_ERR_OK;
    let mut hprefix: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut huri: xmlHashedString = xmlHashedString {
        hashValue: 0,
        name: ::core::ptr::null::<xmlChar>(),
    };
    let mut i: ::core::ffi::c_uint = 0;
    if (*oldctxt).depth > 40 as ::core::ffi::c_int
        && (*oldctxt).options & XML_PARSE_HUGE as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*oldctxt).depth > 100 as ::core::ffi::c_int
    {
        xmlFatalErrMsg(
            oldctxt,
            XML_ERR_ENTITY_LOOP,
            b"Maximum entity nesting depth exceeded\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return XML_ERR_ENTITY_LOOP;
    }
    if !lst.is_null() {
        *lst = ::core::ptr::null_mut::<xmlNode>();
    }
    if string.is_null() {
        return XML_ERR_INTERNAL_ERROR;
    }
    ctxt = xmlCreateDocParserCtxt(string);
    if ctxt.is_null() {
        return XML_WAR_UNDECLARED_ENTITY;
    }
    (*ctxt).nbErrors = (*oldctxt).nbErrors;
    (*ctxt).nbWarnings = (*oldctxt).nbWarnings;
    if !user_data.is_null() {
        (*ctxt).userData = user_data;
    } else {
        (*ctxt).userData = ctxt as *mut ::core::ffi::c_void;
    }
    if !(*ctxt).dict.is_null() {
        xmlDictFree((*ctxt).dict);
    }
    (*ctxt).dict = (*oldctxt).dict;
    (*ctxt).input_id = (*oldctxt).input_id;
    (*ctxt).str_xml = xmlDictLookup(
        (*ctxt).dict,
        b"xml\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        3 as ::core::ffi::c_int,
    );
    (*ctxt).str_xmlns = xmlDictLookup(
        (*ctxt).dict,
        b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        5 as ::core::ffi::c_int,
    );
    (*ctxt).str_xml_ns = xmlDictLookup((*ctxt).dict, XML_XML_NAMESPACE, 36 as ::core::ffi::c_int);
    hprefix.name = ::core::ptr::null::<xmlChar>();
    hprefix.hashValue = 0 as ::core::ffi::c_uint;
    huri.name = xmlParserNsLookupUri(oldctxt, &raw mut hprefix);
    huri.hashValue = 0 as ::core::ffi::c_uint;
    if !huri.name.is_null() {
        xmlParserNsPush(
            ctxt,
            ::core::ptr::null::<xmlHashedString>(),
            &raw mut huri,
            NULL,
            0 as ::core::ffi::c_int,
        );
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*(*oldctxt).nsdb).hashSize {
        let mut bucket: *mut xmlParserNsBucket =
            (*(*oldctxt).nsdb).hash.offset(i as isize) as *mut xmlParserNsBucket;
        let mut ns: *mut *const xmlChar = ::core::ptr::null_mut::<*const xmlChar>();
        let mut extra: *mut xmlParserNsExtra = ::core::ptr::null_mut::<xmlParserNsExtra>();
        let mut nsIndex: ::core::ffi::c_uint = 0;
        if (*bucket).hashValue != 0 as ::core::ffi::c_uint && (*bucket).index != INT_MAX {
            nsIndex = (*bucket).index as ::core::ffi::c_uint;
            ns = (*oldctxt)
                .nsTab
                .offset(nsIndex.wrapping_mul(2 as ::core::ffi::c_uint) as isize)
                as *mut *const xmlChar;
            extra = (*(*oldctxt).nsdb).extra.offset(nsIndex as isize) as *mut xmlParserNsExtra;
            hprefix.name = *ns.offset(0 as ::core::ffi::c_int as isize);
            hprefix.hashValue = (*bucket).hashValue;
            huri.name = *ns.offset(1 as ::core::ffi::c_int as isize);
            huri.hashValue = (*extra).uriHashValue;
            xmlParserNsPush(
                ctxt,
                &raw mut hprefix,
                &raw mut huri,
                NULL,
                0 as ::core::ffi::c_int,
            );
        }
        i = i.wrapping_add(1);
    }
    oldsax = (*ctxt).sax as xmlSAXHandlerPtr;
    (*ctxt).sax = (*oldctxt).sax;
    xmlDetectSAX2(ctxt);
    (*ctxt).replaceEntities = (*oldctxt).replaceEntities;
    (*ctxt).options = (*oldctxt).options;
    (*ctxt)._private = (*oldctxt)._private;
    if (*oldctxt).myDoc.is_null() {
        newDoc = xmlNewDoc(b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
        if newDoc.is_null() {
            ret = XML_ERR_INTERNAL_ERROR;
            current_block = 5746759955443182853;
        } else {
            (*newDoc).properties = XML_DOC_INTERNAL as ::core::ffi::c_int;
            (*newDoc).dict = (*ctxt).dict as *mut _xmlDict;
            xmlDictReference((*newDoc).dict as xmlDictPtr);
            (*ctxt).myDoc = newDoc;
            current_block = 10399321362245223758;
        }
    } else {
        (*ctxt).myDoc = (*oldctxt).myDoc;
        content = (*(*ctxt).myDoc).children as xmlNodePtr;
        last = (*(*ctxt).myDoc).last as xmlNodePtr;
        current_block = 10399321362245223758;
    }
    match current_block {
        10399321362245223758 => {
            newRoot = xmlNewDocNode(
                (*ctxt).myDoc,
                ::core::ptr::null_mut::<xmlNs>(),
                b"pseudoroot\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                ::core::ptr::null::<xmlChar>(),
            );
            if newRoot.is_null() {
                ret = XML_ERR_INTERNAL_ERROR;
            } else {
                (*(*ctxt).myDoc).children = ::core::ptr::null_mut::<_xmlNode>();
                (*(*ctxt).myDoc).last = ::core::ptr::null_mut::<_xmlNode>();
                xmlAddChild((*ctxt).myDoc as xmlNodePtr, newRoot);
                nodePush(ctxt, (*(*ctxt).myDoc).children as xmlNodePtr);
                (*ctxt).instate = XML_PARSER_CONTENT;
                (*ctxt).depth = (*oldctxt).depth;
                (*ctxt).validate = 0 as ::core::ffi::c_int;
                (*ctxt).loadsubset = (*oldctxt).loadsubset;
                if (*oldctxt).validate != 0 || (*oldctxt).replaceEntities != 0 as ::core::ffi::c_int
                {
                    (*ctxt).loadsubset |= XML_SKIP_IDS;
                }
                (*ctxt).dictNames = (*oldctxt).dictNames;
                (*ctxt).attsDefault = (*oldctxt).attsDefault;
                (*ctxt).attsSpecial = (*oldctxt).attsSpecial;
                xmlParseContent(ctxt);
                if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
                    && *(*(*ctxt).input)
                        .cur
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '/' as i32
                {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NOT_WELL_BALANCED,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                } else if *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_EXTRA_CONTENT,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
                if (*ctxt).node != (*(*ctxt).myDoc).children {
                    xmlFatalErr(
                        ctxt,
                        XML_ERR_NOT_WELL_BALANCED,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
                if (*ctxt).wellFormed == 0 {
                    ret = (*ctxt).errNo as xmlParserErrors;
                    (*oldctxt).errNo = (*ctxt).errNo;
                    (*oldctxt).wellFormed = 0 as ::core::ffi::c_int;
                    xmlCopyError(&raw mut (*ctxt).lastError, &raw mut (*oldctxt).lastError);
                } else {
                    ret = XML_ERR_OK;
                }
                if !lst.is_null()
                    && ret as ::core::ffi::c_uint
                        == XML_ERR_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                    cur = (*(*(*ctxt).myDoc).children).children as xmlNodePtr;
                    *lst = cur;
                    while !cur.is_null() {
                        if (*oldctxt).validate != 0
                            && (*oldctxt).wellFormed != 0
                            && !(*oldctxt).myDoc.is_null()
                            && !(*(*oldctxt).myDoc).intSubset.is_null()
                            && (*cur).type_0 as ::core::ffi::c_uint
                                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            (*oldctxt).valid &= xmlValidateElement(
                                &raw mut (*oldctxt).vctxt,
                                (*oldctxt).myDoc,
                                cur,
                            );
                        }
                        (*cur).parent = ::core::ptr::null_mut::<_xmlNode>();
                        cur = (*cur).next as xmlNodePtr;
                    }
                    (*(*(*ctxt).myDoc).children).children = ::core::ptr::null_mut::<_xmlNode>();
                }
                if !(*ctxt).myDoc.is_null() {
                    xmlFreeNode((*(*ctxt).myDoc).children as xmlNodePtr);
                    (*(*ctxt).myDoc).children = content as *mut _xmlNode;
                    (*(*ctxt).myDoc).last = last as *mut _xmlNode;
                }
                if !(*ctxt).input.is_null() && !oldctxt.is_null() {
                    let mut consumed: ::core::ffi::c_ulong = (*(*ctxt).input).consumed;
                    xmlSaturatedAddSizeT(
                        &raw mut consumed,
                        (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
                            as ::core::ffi::c_long as ::core::ffi::c_ulong,
                    );
                    xmlSaturatedAdd(&raw mut (*oldctxt).sizeentcopy, consumed);
                    xmlSaturatedAdd(&raw mut (*oldctxt).sizeentcopy, (*ctxt).sizeentcopy);
                }
                (*oldctxt).nbErrors = (*ctxt).nbErrors;
                (*oldctxt).nbWarnings = (*ctxt).nbWarnings;
            }
        }
        _ => {}
    }
    (*ctxt).sax = oldsax as *mut _xmlSAXHandler;
    (*ctxt).dict = ::core::ptr::null_mut::<xmlDict>();
    (*ctxt).attsDefault = ::core::ptr::null_mut::<xmlHashTable>();
    (*ctxt).attsSpecial = ::core::ptr::null_mut::<xmlHashTable>();
    xmlFreeParserCtxt(ctxt);
    if !newDoc.is_null() {
        xmlFreeDoc(newDoc);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseInNodeContext(
    mut node: xmlNodePtr,
    mut data: *const ::core::ffi::c_char,
    mut datalen: ::core::ffi::c_int,
    mut options: ::core::ffi::c_int,
    mut lst: *mut xmlNodePtr,
) -> xmlParserErrors {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut fake: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut nsnr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: xmlParserErrors = XML_ERR_OK;
    if lst.is_null() || node.is_null() || data.is_null() || datalen < 0 as ::core::ffi::c_int {
        return XML_ERR_INTERNAL_ERROR;
    }
    match (*node).type_0 as ::core::ffi::c_uint {
        1 | 2 | 3 | 4 | 5 | 7 | 8 | 9 | 13 => {}
        _ => return XML_ERR_INTERNAL_ERROR,
    }
    while !node.is_null()
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_HTML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        node = (*node).parent as xmlNodePtr;
    }
    if node.is_null() {
        return XML_ERR_INTERNAL_ERROR;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        doc = (*node).doc as xmlDocPtr;
    } else {
        doc = node as xmlDocPtr;
    }
    if doc.is_null() {
        return XML_ERR_INTERNAL_ERROR;
    }
    if (*doc).type_0 as ::core::ffi::c_uint
        == XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ctxt = xmlCreateMemoryParserCtxt(data as *mut ::core::ffi::c_char, datalen);
    } else if (*doc).type_0 as ::core::ffi::c_uint
        == XML_HTML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ctxt = htmlCreateMemoryParserCtxt(data as *mut ::core::ffi::c_char, datalen)
            as xmlParserCtxtPtr;
        options |= HTML_PARSE_NOIMPLIED as ::core::ffi::c_int;
    } else {
        return XML_ERR_INTERNAL_ERROR;
    }
    if ctxt.is_null() {
        return XML_ERR_NO_MEMORY;
    }
    if !(*doc).dict.is_null() {
        if !(*ctxt).dict.is_null() {
            xmlDictFree((*ctxt).dict);
        }
        (*ctxt).dict = (*doc).dict as xmlDictPtr;
    } else {
        options |= XML_PARSE_NODICT as ::core::ffi::c_int;
    }
    if !(*doc).encoding.is_null() {
        let mut hdlr: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
        hdlr = xmlFindCharEncodingHandler((*doc).encoding as *const ::core::ffi::c_char);
        if !hdlr.is_null() {
            xmlSwitchToEncoding(ctxt, hdlr);
        } else {
            return XML_ERR_UNSUPPORTED_ENCODING;
        }
    }
    xmlCtxtUseOptionsInternal(ctxt, options);
    xmlDetectSAX2(ctxt);
    (*ctxt).myDoc = doc;
    (*ctxt).input_id = 2 as ::core::ffi::c_int;
    (*ctxt).instate = XML_PARSER_CONTENT;
    fake = xmlNewDocComment((*node).doc as xmlDocPtr, ::core::ptr::null::<xmlChar>());
    if fake.is_null() {
        xmlFreeParserCtxt(ctxt);
        return XML_ERR_NO_MEMORY;
    }
    xmlAddChild(node, fake);
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        nodePush(ctxt, node);
    }
    if (*ctxt).html == 0 as ::core::ffi::c_int
        && (*node).type_0 as ::core::ffi::c_uint
            == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        cur = node;
        while !cur.is_null()
            && (*cur).type_0 as ::core::ffi::c_uint
                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut ns: xmlNsPtr = (*cur).nsDef as xmlNsPtr;
            let mut hprefix: xmlHashedString = xmlHashedString {
                hashValue: 0,
                name: ::core::ptr::null::<xmlChar>(),
            };
            let mut huri: xmlHashedString = xmlHashedString {
                hashValue: 0,
                name: ::core::ptr::null::<xmlChar>(),
            };
            while !ns.is_null() {
                hprefix =
                    xmlDictLookupHashed((*ctxt).dict, (*ns).prefix, -(1 as ::core::ffi::c_int));
                huri = xmlDictLookupHashed((*ctxt).dict, (*ns).href, -(1 as ::core::ffi::c_int));
                if xmlParserNsPush(
                    ctxt,
                    &raw mut hprefix,
                    &raw mut huri,
                    ns as *mut ::core::ffi::c_void,
                    1 as ::core::ffi::c_int,
                ) > 0 as ::core::ffi::c_int
                {
                    nsnr += 1;
                }
                ns = (*ns).next as xmlNsPtr;
            }
            cur = (*cur).parent as xmlNodePtr;
        }
    }
    if (*ctxt).validate != 0 || (*ctxt).replaceEntities != 0 as ::core::ffi::c_int {
        (*ctxt).loadsubset |= XML_SKIP_IDS;
    }
    if (*doc).type_0 as ::core::ffi::c_uint
        == XML_HTML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        __htmlParseContent(ctxt as *mut ::core::ffi::c_void);
    } else {
        xmlParseContent(ctxt);
    }
    xmlParserNsPop(ctxt, nsnr);
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOT_WELL_BALANCED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        xmlFatalErr(
            ctxt,
            XML_ERR_EXTRA_CONTENT,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if !(*ctxt).node.is_null() && (*ctxt).node != node {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOT_WELL_BALANCED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        (*ctxt).wellFormed = 0 as ::core::ffi::c_int;
    }
    if (*ctxt).wellFormed == 0 {
        if (*ctxt).errNo == 0 as ::core::ffi::c_int {
            ret = XML_ERR_INTERNAL_ERROR;
        } else {
            ret = (*ctxt).errNo as xmlParserErrors;
        }
    } else {
        ret = XML_ERR_OK;
    }
    cur = (*fake).next as xmlNodePtr;
    (*fake).next = ::core::ptr::null_mut::<_xmlNode>();
    (*node).last = fake as *mut _xmlNode;
    if !cur.is_null() {
        (*cur).prev = ::core::ptr::null_mut::<_xmlNode>();
    }
    *lst = cur;
    while !cur.is_null() {
        (*cur).parent = ::core::ptr::null_mut::<_xmlNode>();
        cur = (*cur).next as xmlNodePtr;
    }
    xmlUnlinkNode(fake);
    xmlFreeNode(fake);
    if ret as ::core::ffi::c_uint != XML_ERR_OK as ::core::ffi::c_int as ::core::ffi::c_uint {
        xmlFreeNodeList(*lst);
        *lst = ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*doc).dict.is_null() {
        (*ctxt).dict = ::core::ptr::null_mut::<xmlDict>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseBalancedChunkMemoryRecover(
    mut doc: xmlDocPtr,
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut ::core::ffi::c_void,
    mut depth: ::core::ffi::c_int,
    mut string: *const xmlChar,
    mut lst: *mut xmlNodePtr,
    mut recover: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut newDoc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut oldsax: xmlSAXHandlerPtr = ::core::ptr::null_mut::<xmlSAXHandler>();
    let mut content: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut newRoot: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if depth > 40 as ::core::ffi::c_int {
        return XML_ERR_ENTITY_LOOP as ::core::ffi::c_int;
    }
    if !lst.is_null() {
        *lst = ::core::ptr::null_mut::<xmlNode>();
    }
    if string.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ctxt = xmlCreateDocParserCtxt(string);
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*ctxt).userData = ctxt as *mut ::core::ffi::c_void;
    if !sax.is_null() {
        oldsax = (*ctxt).sax as xmlSAXHandlerPtr;
        (*ctxt).sax = sax as *mut _xmlSAXHandler;
        if !user_data.is_null() {
            (*ctxt).userData = user_data;
        }
    }
    newDoc = xmlNewDoc(b"1.0\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
    if newDoc.is_null() {
        xmlFreeParserCtxt(ctxt);
        return -(1 as ::core::ffi::c_int);
    }
    (*newDoc).properties = XML_DOC_INTERNAL as ::core::ffi::c_int;
    if !doc.is_null() && !(*doc).dict.is_null() {
        xmlDictFree((*ctxt).dict);
        (*ctxt).dict = (*doc).dict as xmlDictPtr;
        xmlDictReference((*ctxt).dict);
        (*ctxt).str_xml = xmlDictLookup(
            (*ctxt).dict,
            b"xml\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            3 as ::core::ffi::c_int,
        );
        (*ctxt).str_xmlns = xmlDictLookup(
            (*ctxt).dict,
            b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            5 as ::core::ffi::c_int,
        );
        (*ctxt).str_xml_ns =
            xmlDictLookup((*ctxt).dict, XML_XML_NAMESPACE, 36 as ::core::ffi::c_int);
        (*ctxt).dictNames = 1 as ::core::ffi::c_int;
        (*newDoc).dict = (*ctxt).dict as *mut _xmlDict;
        xmlDictReference((*newDoc).dict as xmlDictPtr);
    } else {
        xmlCtxtUseOptionsInternal(ctxt, XML_PARSE_NODICT as ::core::ffi::c_int);
    }
    if !doc.is_null() {
        (*newDoc).intSubset = (*doc).intSubset;
        (*newDoc).extSubset = (*doc).extSubset;
    }
    newRoot = xmlNewDocNode(
        newDoc,
        ::core::ptr::null_mut::<xmlNs>(),
        b"pseudoroot\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ::core::ptr::null::<xmlChar>(),
    );
    if newRoot.is_null() {
        if !sax.is_null() {
            (*ctxt).sax = oldsax as *mut _xmlSAXHandler;
        }
        xmlFreeParserCtxt(ctxt);
        (*newDoc).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
        (*newDoc).extSubset = ::core::ptr::null_mut::<_xmlDtd>();
        xmlFreeDoc(newDoc);
        return -(1 as ::core::ffi::c_int);
    }
    xmlAddChild(newDoc as xmlNodePtr, newRoot);
    nodePush(ctxt, newRoot);
    if doc.is_null() {
        (*ctxt).myDoc = newDoc;
    } else {
        (*ctxt).myDoc = newDoc;
        xmlSearchNsByHref(doc, doc as xmlNodePtr, XML_XML_NAMESPACE);
        (*newDoc).oldNs = (*doc).oldNs;
    }
    (*ctxt).instate = XML_PARSER_CONTENT;
    (*ctxt).input_id = 2 as ::core::ffi::c_int;
    (*ctxt).depth = depth;
    (*ctxt).validate = 0 as ::core::ffi::c_int;
    (*ctxt).loadsubset = 0 as ::core::ffi::c_int;
    xmlDetectSAX2(ctxt);
    if !doc.is_null() {
        content = (*doc).children as xmlNodePtr;
        (*doc).children = ::core::ptr::null_mut::<_xmlNode>();
        xmlParseContent(ctxt);
        (*doc).children = content as *mut _xmlNode;
    } else {
        xmlParseContent(ctxt);
    }
    if *(*(*ctxt).input).cur as ::core::ffi::c_int == '<' as i32
        && *(*(*ctxt).input)
            .cur
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOT_WELL_BALANCED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    } else if *(*(*ctxt).input).cur as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        xmlFatalErr(
            ctxt,
            XML_ERR_EXTRA_CONTENT,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if (*ctxt).node != (*newDoc).children {
        xmlFatalErr(
            ctxt,
            XML_ERR_NOT_WELL_BALANCED,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if (*ctxt).wellFormed == 0 {
        if (*ctxt).errNo == 0 as ::core::ffi::c_int {
            ret = 1 as ::core::ffi::c_int;
        } else {
            ret = (*ctxt).errNo;
        }
    } else {
        ret = 0 as ::core::ffi::c_int;
    }
    if !lst.is_null() && (ret == 0 as ::core::ffi::c_int || recover == 1 as ::core::ffi::c_int) {
        let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        cur = (*(*newDoc).children).children as xmlNodePtr;
        *lst = cur;
        while !cur.is_null() {
            xmlSetTreeDoc(cur, doc);
            (*cur).parent = ::core::ptr::null_mut::<_xmlNode>();
            cur = (*cur).next as xmlNodePtr;
        }
        (*(*newDoc).children).children = ::core::ptr::null_mut::<_xmlNode>();
    }
    if !sax.is_null() {
        (*ctxt).sax = oldsax as *mut _xmlSAXHandler;
    }
    xmlFreeParserCtxt(ctxt);
    (*newDoc).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
    (*newDoc).extSubset = ::core::ptr::null_mut::<_xmlDtd>();
    (*newDoc).oldNs = ::core::ptr::null_mut::<_xmlNs>();
    xmlFreeDoc(newDoc);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseEntity(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const ::core::ffi::c_char,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    ctxt = xmlCreateFileParserCtxt(filename);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if !sax.is_null() {
        if !(*ctxt).sax.is_null() {
            xmlFree.expect("non-null function pointer")((*ctxt).sax as *mut ::core::ffi::c_void);
        }
        (*ctxt).sax = sax as *mut _xmlSAXHandler;
        (*ctxt).userData = NULL;
    }
    xmlParseExtParsedEnt(ctxt);
    if (*ctxt).wellFormed != 0 {
        ret = (*ctxt).myDoc;
    } else {
        ret = ::core::ptr::null_mut::<xmlDoc>();
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    if !sax.is_null() {
        (*ctxt).sax = ::core::ptr::null_mut::<_xmlSAXHandler>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseEntity(mut filename: *const ::core::ffi::c_char) -> xmlDocPtr {
    return xmlSAXParseEntity(::core::ptr::null_mut::<xmlSAXHandler>(), filename);
}
unsafe extern "C" fn xmlCreateEntityParserCtxtInternal(
    mut sax: xmlSAXHandlerPtr,
    mut userData: *mut ::core::ffi::c_void,
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut base: *const xmlChar,
    mut pctx: xmlParserCtxtPtr,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut directory: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uri: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    ctxt = xmlNewSAXParserCtxt(sax as *const xmlSAXHandler, userData);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    if !pctx.is_null() {
        (*ctxt).options = (*pctx).options;
        (*ctxt)._private = (*pctx)._private;
        (*ctxt).input_id = (*pctx).input_id;
    }
    if xmlStrcmp(
        URL,
        b"-\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) == 0 as ::core::ffi::c_int
    {
        URL = b"./-\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar;
    }
    uri = xmlBuildURI(URL, base);
    if uri.is_null() {
        inputStream = xmlLoadExternalEntity(
            URL as *mut ::core::ffi::c_char,
            ID as *mut ::core::ffi::c_char,
            ctxt,
        );
        if inputStream.is_null() {
            xmlFreeParserCtxt(ctxt);
            return ::core::ptr::null_mut::<xmlParserCtxt>();
        }
        inputPush(ctxt, inputStream);
        if (*ctxt).directory.is_null() && directory.is_null() {
            directory = xmlParserGetDirectory(URL as *mut ::core::ffi::c_char);
        }
        if (*ctxt).directory.is_null() && !directory.is_null() {
            (*ctxt).directory = directory;
        }
    } else {
        inputStream = xmlLoadExternalEntity(
            uri as *mut ::core::ffi::c_char,
            ID as *mut ::core::ffi::c_char,
            ctxt,
        );
        if inputStream.is_null() {
            xmlFree.expect("non-null function pointer")(uri as *mut ::core::ffi::c_void);
            xmlFreeParserCtxt(ctxt);
            return ::core::ptr::null_mut::<xmlParserCtxt>();
        }
        inputPush(ctxt, inputStream);
        if (*ctxt).directory.is_null() && directory.is_null() {
            directory = xmlParserGetDirectory(uri as *mut ::core::ffi::c_char);
        }
        if (*ctxt).directory.is_null() && !directory.is_null() {
            (*ctxt).directory = directory;
        }
        xmlFree.expect("non-null function pointer")(uri as *mut ::core::ffi::c_void);
    }
    return ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateEntityParserCtxt(
    mut URL: *const xmlChar,
    mut ID: *const xmlChar,
    mut base: *const xmlChar,
) -> xmlParserCtxtPtr {
    return xmlCreateEntityParserCtxtInternal(
        ::core::ptr::null_mut::<xmlSAXHandler>(),
        NULL,
        URL,
        ID,
        base,
        ::core::ptr::null_mut::<xmlParserCtxt>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateURLParserCtxt(
    mut filename: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut directory: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        xmlErrMemory(
            ::core::ptr::null_mut::<xmlParserCtxt>(),
            b"cannot allocate parser context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    if options != 0 {
        xmlCtxtUseOptionsInternal(ctxt, options);
    }
    (*ctxt).linenumbers = 1 as ::core::ffi::c_int;
    inputStream = xmlLoadExternalEntity(filename, ::core::ptr::null::<::core::ffi::c_char>(), ctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    inputPush(ctxt, inputStream);
    if (*ctxt).directory.is_null() && directory.is_null() {
        directory = xmlParserGetDirectory(filename);
    }
    if (*ctxt).directory.is_null() && !directory.is_null() {
        (*ctxt).directory = directory;
    }
    return ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateFileParserCtxt(
    mut filename: *const ::core::ffi::c_char,
) -> xmlParserCtxtPtr {
    return xmlCreateURLParserCtxt(filename, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseFileWithData(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const ::core::ffi::c_char,
    mut recovery: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    xmlInitParser();
    ctxt = xmlCreateFileParserCtxt(filename);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if !sax.is_null() {
        if !(*ctxt).sax.is_null() {
            xmlFree.expect("non-null function pointer")((*ctxt).sax as *mut ::core::ffi::c_void);
        }
        (*ctxt).sax = sax as *mut _xmlSAXHandler;
    }
    xmlDetectSAX2(ctxt);
    if !data.is_null() {
        (*ctxt)._private = data;
    }
    if (*ctxt).directory.is_null() {
        (*ctxt).directory = xmlParserGetDirectory(filename);
    }
    (*ctxt).recovery = recovery;
    xmlParseDocument(ctxt);
    if (*ctxt).wellFormed != 0 || recovery != 0 {
        ret = (*ctxt).myDoc;
        if !ret.is_null() && !(*(*ctxt).input).buf.is_null() {
            if (*(*(*ctxt).input).buf).compressed > 0 as ::core::ffi::c_int {
                (*ret).compression = 9 as ::core::ffi::c_int;
            } else {
                (*ret).compression = (*(*(*ctxt).input).buf).compressed;
            }
        }
    } else {
        ret = ::core::ptr::null_mut::<xmlDoc>();
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    if !sax.is_null() {
        (*ctxt).sax = ::core::ptr::null_mut::<_xmlSAXHandler>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseFile(
    mut sax: xmlSAXHandlerPtr,
    mut filename: *const ::core::ffi::c_char,
    mut recovery: ::core::ffi::c_int,
) -> xmlDocPtr {
    return xmlSAXParseFileWithData(sax, filename, recovery, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRecoverDoc(mut cur: *const xmlChar) -> xmlDocPtr {
    return xmlSAXParseDoc(
        ::core::ptr::null_mut::<xmlSAXHandler>(),
        cur,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseFile(mut filename: *const ::core::ffi::c_char) -> xmlDocPtr {
    return xmlSAXParseFile(
        ::core::ptr::null_mut::<xmlSAXHandler>(),
        filename,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlRecoverFile(mut filename: *const ::core::ffi::c_char) -> xmlDocPtr {
    return xmlSAXParseFile(
        ::core::ptr::null_mut::<xmlSAXHandler>(),
        filename,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlSetupParserForBuffer(
    mut ctxt: xmlParserCtxtPtr,
    mut buffer: *const xmlChar,
    mut filename: *const ::core::ffi::c_char,
) {
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if ctxt.is_null() || buffer.is_null() {
        return;
    }
    input = xmlNewInputStream(ctxt);
    if input.is_null() {
        xmlErrMemory(
            ::core::ptr::null_mut::<xmlParserCtxt>(),
            b"parsing new buffer: out of memory\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        xmlClearParserCtxt(ctxt);
        return;
    }
    xmlClearParserCtxt(ctxt);
    if !filename.is_null() {
        (*input).filename = xmlCanonicPath(filename as *const xmlChar) as *mut ::core::ffi::c_char;
    }
    (*input).base = buffer;
    (*input).cur = buffer;
    (*input).end = buffer.offset((xmlStrlen
        as unsafe extern "C" fn(*const xmlChar) -> ::core::ffi::c_int)(
        buffer
    ) as isize) as *const xmlChar;
    inputPush(ctxt, input);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXUserParseFile(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut ::core::ffi::c_void,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    ctxt = xmlCreateFileParserCtxt(filename);
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).sax != __xmlDefaultSAXHandler() as xmlSAXHandlerPtr {
        xmlFree.expect("non-null function pointer")((*ctxt).sax as *mut ::core::ffi::c_void);
    }
    (*ctxt).sax = sax as *mut _xmlSAXHandler;
    xmlDetectSAX2(ctxt);
    if !user_data.is_null() {
        (*ctxt).userData = user_data;
    }
    xmlParseDocument(ctxt);
    if (*ctxt).wellFormed != 0 {
        ret = 0 as ::core::ffi::c_int;
    } else if (*ctxt).errNo != 0 as ::core::ffi::c_int {
        ret = (*ctxt).errNo;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    if !sax.is_null() {
        (*ctxt).sax = ::core::ptr::null_mut::<_xmlSAXHandler>();
    }
    if !(*ctxt).myDoc.is_null() {
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateMemoryParserCtxt(
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if buffer.is_null() {
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    if size <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    buf = xmlParserInputBufferCreateMem(buffer, size, XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    input = xmlNewInputStream(ctxt);
    if input.is_null() {
        xmlFreeParserInputBuffer(buf);
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    (*input).filename = ::core::ptr::null::<::core::ffi::c_char>();
    (*input).buf = buf;
    xmlBufResetInput((*(*input).buf).buffer, input);
    inputPush(ctxt, input);
    return ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseMemoryWithData(
    mut sax: xmlSAXHandlerPtr,
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut recovery: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    xmlInitParser();
    ctxt = xmlCreateMemoryParserCtxt(buffer, size);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if !sax.is_null() {
        if !(*ctxt).sax.is_null() {
            xmlFree.expect("non-null function pointer")((*ctxt).sax as *mut ::core::ffi::c_void);
        }
        (*ctxt).sax = sax as *mut _xmlSAXHandler;
    }
    xmlDetectSAX2(ctxt);
    if !data.is_null() {
        (*ctxt)._private = data;
    }
    (*ctxt).recovery = recovery;
    xmlParseDocument(ctxt);
    if (*ctxt).wellFormed != 0 || recovery != 0 {
        ret = (*ctxt).myDoc;
    } else {
        ret = ::core::ptr::null_mut::<xmlDoc>();
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    if !sax.is_null() {
        (*ctxt).sax = ::core::ptr::null_mut::<_xmlSAXHandler>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseMemory(
    mut sax: xmlSAXHandlerPtr,
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut recovery: ::core::ffi::c_int,
) -> xmlDocPtr {
    return xmlSAXParseMemoryWithData(sax, buffer, size, recovery, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseMemory(
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> xmlDocPtr {
    return xmlSAXParseMemory(
        ::core::ptr::null_mut::<xmlSAXHandler>(),
        buffer,
        size,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlRecoverMemory(
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> xmlDocPtr {
    return xmlSAXParseMemory(
        ::core::ptr::null_mut::<xmlSAXHandler>(),
        buffer,
        size,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXUserParseMemory(
    mut sax: xmlSAXHandlerPtr,
    mut user_data: *mut ::core::ffi::c_void,
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    xmlInitParser();
    ctxt = xmlCreateMemoryParserCtxt(buffer, size);
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).sax != __xmlDefaultSAXHandler() as xmlSAXHandlerPtr {
        xmlFree.expect("non-null function pointer")((*ctxt).sax as *mut ::core::ffi::c_void);
    }
    (*ctxt).sax = sax as *mut _xmlSAXHandler;
    xmlDetectSAX2(ctxt);
    if !user_data.is_null() {
        (*ctxt).userData = user_data;
    }
    xmlParseDocument(ctxt);
    if (*ctxt).wellFormed != 0 {
        ret = 0 as ::core::ffi::c_int;
    } else if (*ctxt).errNo != 0 as ::core::ffi::c_int {
        ret = (*ctxt).errNo;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    if !sax.is_null() {
        (*ctxt).sax = ::core::ptr::null_mut::<_xmlSAXHandler>();
    }
    if !(*ctxt).myDoc.is_null() {
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateDocParserCtxt(mut str: *const xmlChar) -> xmlParserCtxtPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if str.is_null() {
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    buf = xmlParserInputBufferCreateString(str);
    if buf.is_null() {
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    input = xmlNewInputStream(ctxt);
    if input.is_null() {
        xmlFreeParserInputBuffer(buf);
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlParserCtxt>();
    }
    (*input).filename = ::core::ptr::null::<::core::ffi::c_char>();
    (*input).buf = buf;
    xmlBufResetInput((*(*input).buf).buffer, input);
    inputPush(ctxt, input);
    return ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXParseDoc(
    mut sax: xmlSAXHandlerPtr,
    mut cur: *const xmlChar,
    mut recovery: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut oldsax: xmlSAXHandlerPtr = ::core::ptr::null_mut::<xmlSAXHandler>();
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    ctxt = xmlCreateDocParserCtxt(cur);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if !sax.is_null() {
        oldsax = (*ctxt).sax as xmlSAXHandlerPtr;
        (*ctxt).sax = sax as *mut _xmlSAXHandler;
        (*ctxt).userData = NULL;
    }
    xmlDetectSAX2(ctxt);
    xmlParseDocument(ctxt);
    if (*ctxt).wellFormed != 0 || recovery != 0 {
        ret = (*ctxt).myDoc;
    } else {
        ret = ::core::ptr::null_mut::<xmlDoc>();
        xmlFreeDoc((*ctxt).myDoc);
        (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    if !sax.is_null() {
        (*ctxt).sax = oldsax as *mut _xmlSAXHandler;
    }
    xmlFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseDoc(mut cur: *const xmlChar) -> xmlDocPtr {
    return xmlSAXParseDoc(
        ::core::ptr::null_mut::<xmlSAXHandler>(),
        cur,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReset(mut ctxt: xmlParserCtxtPtr) {
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if ctxt.is_null() {
        return;
    }
    dict = (*ctxt).dict;
    loop {
        input = inputPop(ctxt);
        if input.is_null() {
            break;
        }
        xmlFreeInputStream(input);
    }
    (*ctxt).inputNr = 0 as ::core::ffi::c_int;
    (*ctxt).input = ::core::ptr::null_mut::<xmlParserInput>();
    (*ctxt).spaceNr = 0 as ::core::ffi::c_int;
    if !(*ctxt).spaceTab.is_null() {
        *(*ctxt).spaceTab.offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
        (*ctxt).space =
            (*ctxt).spaceTab.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    } else {
        (*ctxt).space = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    (*ctxt).nodeNr = 0 as ::core::ffi::c_int;
    (*ctxt).node = ::core::ptr::null_mut::<xmlNode>();
    (*ctxt).nameNr = 0 as ::core::ffi::c_int;
    (*ctxt).name = ::core::ptr::null::<xmlChar>();
    (*ctxt).nsNr = 0 as ::core::ffi::c_int;
    xmlParserNsReset((*ctxt).nsdb);
    if !(*ctxt).version.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*ctxt).version) == 0 as ::core::ffi::c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).version as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
    }
    (*ctxt).version = ::core::ptr::null::<xmlChar>();
    if !(*ctxt).encoding.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*ctxt).encoding) == 0 as ::core::ffi::c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).encoding as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
    }
    (*ctxt).encoding = ::core::ptr::null::<xmlChar>();
    if !(*ctxt).directory.is_null()
        && (dict.is_null()
            || xmlDictOwns(dict, (*ctxt).directory as *const xmlChar) == 0 as ::core::ffi::c_int)
    {
        xmlFree.expect("non-null function pointer")((*ctxt).directory as *mut ::core::ffi::c_void);
    }
    (*ctxt).directory = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*ctxt).extSubURI.is_null()
        && (dict.is_null()
            || xmlDictOwns(dict, (*ctxt).extSubURI as *const xmlChar) == 0 as ::core::ffi::c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).extSubURI as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
    }
    (*ctxt).extSubURI = ::core::ptr::null_mut::<xmlChar>();
    if !(*ctxt).extSubSystem.is_null()
        && (dict.is_null()
            || xmlDictOwns(dict, (*ctxt).extSubSystem as *const xmlChar) == 0 as ::core::ffi::c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*ctxt).extSubSystem as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
    }
    (*ctxt).extSubSystem = ::core::ptr::null_mut::<xmlChar>();
    if !(*ctxt).myDoc.is_null() {
        xmlFreeDoc((*ctxt).myDoc);
    }
    (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    (*ctxt).standalone = -(1 as ::core::ffi::c_int);
    (*ctxt).hasExternalSubset = 0 as ::core::ffi::c_int;
    (*ctxt).hasPErefs = 0 as ::core::ffi::c_int;
    (*ctxt).html = 0 as ::core::ffi::c_int;
    (*ctxt).external = 0 as ::core::ffi::c_int;
    (*ctxt).instate = XML_PARSER_START;
    (*ctxt).token = 0 as ::core::ffi::c_int;
    (*ctxt).wellFormed = 1 as ::core::ffi::c_int;
    (*ctxt).nsWellFormed = 1 as ::core::ffi::c_int;
    (*ctxt).disableSAX = 0 as ::core::ffi::c_int;
    (*ctxt).valid = 1 as ::core::ffi::c_int;
    (*ctxt).record_info = 0 as ::core::ffi::c_int;
    (*ctxt).checkIndex = 0 as ::core::ffi::c_long;
    (*ctxt).endCheckState = 0 as ::core::ffi::c_int;
    (*ctxt).inSubset = 0 as ::core::ffi::c_int;
    (*ctxt).errNo = XML_ERR_OK as ::core::ffi::c_int;
    (*ctxt).depth = 0 as ::core::ffi::c_int;
    (*ctxt).catalogs = NULL;
    (*ctxt).sizeentities = 0 as ::core::ffi::c_ulong;
    (*ctxt).sizeentcopy = 0 as ::core::ffi::c_ulong;
    xmlInitNodeInfoSeq(&raw mut (*ctxt).node_seq);
    if !(*ctxt).attsDefault.is_null() {
        xmlHashFree(
            (*ctxt).attsDefault,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
            ),
        );
        (*ctxt).attsDefault = ::core::ptr::null_mut::<xmlHashTable>();
    }
    if !(*ctxt).attsSpecial.is_null() {
        xmlHashFree((*ctxt).attsSpecial, None);
        (*ctxt).attsSpecial = ::core::ptr::null_mut::<xmlHashTable>();
    }
    if !(*ctxt).catalogs.is_null() {
        xmlCatalogFreeLocal((*ctxt).catalogs);
    }
    (*ctxt).nbErrors = 0 as ::core::ffi::c_ushort;
    (*ctxt).nbWarnings = 0 as ::core::ffi::c_ushort;
    if (*ctxt).lastError.code != XML_ERR_OK as ::core::ffi::c_int {
        xmlResetError(&raw mut (*ctxt).lastError);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtResetPush(
    mut ctxt: xmlParserCtxtPtr,
    mut chunk: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut filename: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if ctxt.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    buf = xmlAllocParserInputBuffer(XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if ctxt.is_null() {
        xmlFreeParserInputBuffer(buf);
        return 1 as ::core::ffi::c_int;
    }
    xmlCtxtReset(ctxt);
    if filename.is_null() {
        (*ctxt).directory = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        (*ctxt).directory = xmlParserGetDirectory(filename);
    }
    inputStream = xmlNewInputStream(ctxt);
    if inputStream.is_null() {
        xmlFreeParserInputBuffer(buf);
        return 1 as ::core::ffi::c_int;
    }
    if filename.is_null() {
        (*inputStream).filename = ::core::ptr::null::<::core::ffi::c_char>();
    } else {
        (*inputStream).filename =
            xmlCanonicPath(filename as *const xmlChar) as *mut ::core::ffi::c_char;
    }
    (*inputStream).buf = buf;
    xmlBufResetInput((*buf).buffer, inputStream);
    inputPush(ctxt, inputStream);
    if size > 0 as ::core::ffi::c_int
        && !chunk.is_null()
        && !(*ctxt).input.is_null()
        && !(*(*ctxt).input).buf.is_null()
    {
        let mut pos: size_t = (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base)
            as ::core::ffi::c_long as size_t;
        let mut res: ::core::ffi::c_int = 0;
        res = xmlParserInputBufferPush((*(*ctxt).input).buf, size, chunk);
        xmlBufUpdateInput((*(*(*ctxt).input).buf).buffer, (*ctxt).input, pos);
        if res < 0 as ::core::ffi::c_int {
            xmlFatalErr(
                ctxt,
                (*(*(*ctxt).input).buf).error as xmlParserErrors,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
            xmlHaltParser(ctxt);
            return 1 as ::core::ffi::c_int;
        }
    }
    if !encoding.is_null() {
        let mut hdlr: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
        hdlr = xmlFindCharEncodingHandler(encoding);
        if !hdlr.is_null() {
            xmlSwitchToEncoding(ctxt, hdlr);
        } else {
            xmlFatalErrMsgStr(
                ctxt,
                XML_ERR_UNSUPPORTED_ENCODING,
                b"Unsupported encoding %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                encoding as *mut xmlChar,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlCtxtUseOptionsInternal(
    mut ctxt: xmlParserCtxtPtr,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if options & XML_PARSE_RECOVER as ::core::ffi::c_int != 0 {
        (*ctxt).recovery = 1 as ::core::ffi::c_int;
        options -= XML_PARSE_RECOVER as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_RECOVER as ::core::ffi::c_int;
    } else {
        (*ctxt).recovery = 0 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_DTDLOAD as ::core::ffi::c_int != 0 {
        (*ctxt).loadsubset = XML_DETECT_IDS;
        options -= XML_PARSE_DTDLOAD as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_DTDLOAD as ::core::ffi::c_int;
    } else {
        (*ctxt).loadsubset = 0 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_DTDATTR as ::core::ffi::c_int != 0 {
        (*ctxt).loadsubset |= XML_COMPLETE_ATTRS;
        options -= XML_PARSE_DTDATTR as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_DTDATTR as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NOENT as ::core::ffi::c_int != 0 {
        (*ctxt).replaceEntities = 1 as ::core::ffi::c_int;
        options -= XML_PARSE_NOENT as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_NOENT as ::core::ffi::c_int;
    } else {
        (*ctxt).replaceEntities = 0 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_PEDANTIC as ::core::ffi::c_int != 0 {
        (*ctxt).pedantic = 1 as ::core::ffi::c_int;
        options -= XML_PARSE_PEDANTIC as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_PEDANTIC as ::core::ffi::c_int;
    } else {
        (*ctxt).pedantic = 0 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NOBLANKS as ::core::ffi::c_int != 0 {
        (*ctxt).keepBlanks = 0 as ::core::ffi::c_int;
        (*(*ctxt).sax).ignorableWhitespace = Some(
            xmlSAX2IgnorableWhitespace
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    ::core::ffi::c_int,
                ) -> (),
        ) as ignorableWhitespaceSAXFunc;
        options -= XML_PARSE_NOBLANKS as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_NOBLANKS as ::core::ffi::c_int;
    } else {
        (*ctxt).keepBlanks = 1 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_DTDVALID as ::core::ffi::c_int != 0 {
        (*ctxt).validate = 1 as ::core::ffi::c_int;
        if options & XML_PARSE_NOWARNING as ::core::ffi::c_int != 0 {
            (*ctxt).vctxt.warning = None;
        }
        if options & XML_PARSE_NOERROR as ::core::ffi::c_int != 0 {
            (*ctxt).vctxt.error = None;
        }
        options -= XML_PARSE_DTDVALID as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_DTDVALID as ::core::ffi::c_int;
    } else {
        (*ctxt).validate = 0 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NOWARNING as ::core::ffi::c_int != 0 {
        (*(*ctxt).sax).warning = None;
        options -= XML_PARSE_NOWARNING as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NOERROR as ::core::ffi::c_int != 0 {
        (*(*ctxt).sax).error = None;
        (*(*ctxt).sax).fatalError = None;
        options -= XML_PARSE_NOERROR as ::core::ffi::c_int;
    }
    if options & XML_PARSE_SAX1 as ::core::ffi::c_int != 0 {
        (*(*ctxt).sax).startElementNs = None;
        (*(*ctxt).sax).endElementNs = None;
        (*(*ctxt).sax).initialized = 1 as ::core::ffi::c_uint;
        options -= XML_PARSE_SAX1 as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_SAX1 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NODICT as ::core::ffi::c_int != 0 {
        (*ctxt).dictNames = 0 as ::core::ffi::c_int;
        options -= XML_PARSE_NODICT as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_NODICT as ::core::ffi::c_int;
    } else {
        (*ctxt).dictNames = 1 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NOCDATA as ::core::ffi::c_int != 0 {
        (*(*ctxt).sax).cdataBlock = None;
        options -= XML_PARSE_NOCDATA as ::core::ffi::c_int;
        (*ctxt).options |= XML_PARSE_NOCDATA as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NSCLEAN as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_NSCLEAN as ::core::ffi::c_int;
        options -= XML_PARSE_NSCLEAN as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NONET as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_NONET as ::core::ffi::c_int;
        options -= XML_PARSE_NONET as ::core::ffi::c_int;
    }
    if options & XML_PARSE_COMPACT as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_COMPACT as ::core::ffi::c_int;
        options -= XML_PARSE_COMPACT as ::core::ffi::c_int;
    }
    if options & XML_PARSE_OLD10 as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_OLD10 as ::core::ffi::c_int;
        options -= XML_PARSE_OLD10 as ::core::ffi::c_int;
    }
    if options & XML_PARSE_NOBASEFIX as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_NOBASEFIX as ::core::ffi::c_int;
        options -= XML_PARSE_NOBASEFIX as ::core::ffi::c_int;
    }
    if options & XML_PARSE_HUGE as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_HUGE as ::core::ffi::c_int;
        options -= XML_PARSE_HUGE as ::core::ffi::c_int;
        if !(*ctxt).dict.is_null() {
            xmlDictSetLimit((*ctxt).dict, 0 as size_t);
        }
    }
    if options & XML_PARSE_OLDSAX as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_OLDSAX as ::core::ffi::c_int;
        options -= XML_PARSE_OLDSAX as ::core::ffi::c_int;
    }
    if options & XML_PARSE_IGNORE_ENC as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_IGNORE_ENC as ::core::ffi::c_int;
        options -= XML_PARSE_IGNORE_ENC as ::core::ffi::c_int;
    }
    if options & XML_PARSE_BIG_LINES as ::core::ffi::c_int != 0 {
        (*ctxt).options |= XML_PARSE_BIG_LINES as ::core::ffi::c_int;
        options -= XML_PARSE_BIG_LINES as ::core::ffi::c_int;
    }
    (*ctxt).linenumbers = 1 as ::core::ffi::c_int;
    return options;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtUseOptions(
    mut ctxt: xmlParserCtxtPtr,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return xmlCtxtUseOptionsInternal(ctxt, options);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtSetMaxAmplification(
    mut ctxt: xmlParserCtxtPtr,
    mut maxAmpl: ::core::ffi::c_uint,
) {
    (*ctxt).maxAmpl = maxAmpl;
}
unsafe extern "C" fn xmlDoRead(
    mut ctxt: xmlParserCtxtPtr,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
    mut reuse: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    xmlCtxtUseOptionsInternal(ctxt, options);
    if !encoding.is_null() {
        let mut hdlr: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
        hdlr = xmlFindCharEncodingHandler(encoding);
        if !hdlr.is_null() {
            xmlSwitchToEncoding(ctxt, hdlr);
        }
    }
    if !URL.is_null() && !(*ctxt).input.is_null() && (*(*ctxt).input).filename.is_null() {
        (*(*ctxt).input).filename = xmlStrdup(URL as *const xmlChar) as *mut ::core::ffi::c_char;
    }
    xmlParseDocument(ctxt);
    if (*ctxt).wellFormed != 0 || (*ctxt).recovery != 0 {
        ret = (*ctxt).myDoc;
    } else {
        ret = ::core::ptr::null_mut::<xmlDoc>();
        if !(*ctxt).myDoc.is_null() {
            xmlFreeDoc((*ctxt).myDoc);
        }
    }
    (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    if reuse == 0 {
        xmlFreeParserCtxt(ctxt);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReadDoc(
    mut cur: *const xmlChar,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlInitParser();
    ctxt = xmlCreateDocParserCtxt(cur);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    return xmlDoRead(ctxt, URL, encoding, options, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReadFile(
    mut filename: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    xmlInitParser();
    ctxt = xmlCreateURLParserCtxt(filename, options);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    return xmlDoRead(
        ctxt,
        ::core::ptr::null::<::core::ffi::c_char>(),
        encoding,
        options,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlReadMemory(
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    xmlInitParser();
    ctxt = xmlCreateMemoryParserCtxt(buffer, size);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    return xmlDoRead(ctxt, URL, encoding, options, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReadFd(
    mut fd: ::core::ffi::c_int,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut stream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlInitParser();
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    (*input).closecallback = None;
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    stream = xmlNewIOInputStream(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer(input);
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputPush(ctxt, stream);
    return xmlDoRead(ctxt, URL, encoding, options, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReadIO(
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut ::core::ffi::c_void,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut stream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if ioread.is_none() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlInitParser();
    input = xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    ctxt = xmlNewParserCtxt();
    if ctxt.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    stream = xmlNewIOInputStream(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer(input);
        xmlFreeParserCtxt(ctxt);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputPush(ctxt, stream);
    return xmlDoRead(ctxt, URL, encoding, options, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadDoc(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut stream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if str.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlInitParser();
    xmlCtxtReset(ctxt);
    input = xmlParserInputBufferCreateString(str);
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    stream = xmlNewIOInputStream(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputPush(ctxt, stream);
    return xmlDoRead(ctxt, URL, encoding, options, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadFile(
    mut ctxt: xmlParserCtxtPtr,
    mut filename: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut stream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if filename.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlInitParser();
    xmlCtxtReset(ctxt);
    stream = xmlLoadExternalEntity(filename, ::core::ptr::null::<::core::ffi::c_char>(), ctxt);
    if stream.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputPush(ctxt, stream);
    return xmlDoRead(
        ctxt,
        ::core::ptr::null::<::core::ffi::c_char>(),
        encoding,
        options,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadMemory(
    mut ctxt: xmlParserCtxtPtr,
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut stream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if buffer.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlInitParser();
    xmlCtxtReset(ctxt);
    input = xmlParserInputBufferCreateStatic(buffer, size, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    stream = xmlNewIOInputStream(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputPush(ctxt, stream);
    return xmlDoRead(ctxt, URL, encoding, options, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadFd(
    mut ctxt: xmlParserCtxtPtr,
    mut fd: ::core::ffi::c_int,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut stream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlInitParser();
    xmlCtxtReset(ctxt);
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    (*input).closecallback = None;
    stream = xmlNewIOInputStream(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputPush(ctxt, stream);
    return xmlDoRead(ctxt, URL, encoding, options, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCtxtReadIO(
    mut ctxt: xmlParserCtxtPtr,
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut ::core::ffi::c_void,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlDocPtr {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut stream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if ioread.is_none() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlInitParser();
    xmlCtxtReset(ctxt);
    input = xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    stream = xmlNewIOInputStream(ctxt, input, XML_CHAR_ENCODING_NONE);
    if stream.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputPush(ctxt, stream);
    return xmlDoRead(ctxt, URL, encoding, options, 1 as ::core::ffi::c_int);
}
pub const XML_DEFAULT_VERSION: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"1.0\0") };
pub const XML_DETECT_IDS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XML_COMPLETE_ATTRS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const XML_SKIP_IDS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const XML_SAX2_MAGIC: ::core::ffi::c_uint = 0xdeedbeaf as ::core::ffi::c_uint;
pub const XML_XML_NAMESPACE: *const xmlChar = b"http://www.w3.org/XML/1998/namespace\0" as *const u8
    as *const ::core::ffi::c_char as *const xmlChar;
pub const XML_MAX_TEXT_LENGTH: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const XML_MAX_HUGE_LENGTH: ::core::ffi::c_int = 1000000000 as ::core::ffi::c_int;
pub const XML_MAX_NAME_LENGTH: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;
pub const XML_MAX_LOOKUP_LIMIT: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const XML_MAX_NAMELEN: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const INPUT_CHUNK: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const XML_SUBSTITUTE_REF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XML_SUBSTITUTE_PEREF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XML_CATALOG_PI: *const xmlChar =
    b"oasis-xml-catalog\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
pub const XML_ENT_PARSED: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const XML_ENT_CHECKED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const XML_ENT_EXPANDING: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const XML_ENT_CHECKED_LT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const XML_ENT_CONTAINS_LT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
