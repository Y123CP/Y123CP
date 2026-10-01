extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlAttrHashBucket;
    pub type _xmlParserNsData;
    pub type _xmlHashTable;
    pub type _xmlStartTag;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    pub type _xmlXPathCompExpr;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlGetUTF8Char(
        utf: *const ::core::ffi::c_uchar,
        len: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    fn xmlCreateIntSubset(
        doc: xmlDocPtr,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlDtdPtr;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar) -> xmlNodePtr;
    fn xmlDocCopyNode(
        node: xmlNodePtr,
        doc: xmlDocPtr,
        recursive: ::core::ffi::c_int,
    ) -> xmlNodePtr;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlAddPrevSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddNextSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlFreeNodeList(cur: xmlNodePtr);
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlGetNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlNodeAddContentLen(cur: xmlNodePtr, content: *const xmlChar, len: ::core::ffi::c_int);
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeSetBase(cur: xmlNodePtr, uri: *const xmlChar);
    fn xmlUnsetProp(node: xmlNodePtr, name: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlDictReference(dict: xmlDictPtr) -> ::core::ffi::c_int;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlHashScan(hash: xmlHashTablePtr, scan: xmlHashScanner, data: *mut ::core::ffi::c_void);
    fn xmlAddDocEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: ::core::ffi::c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlGetCharEncodingHandler(enc: xmlCharEncoding) -> xmlCharEncodingHandlerPtr;
    fn xmlParseCharEncoding(name: *const ::core::ffi::c_char) -> xmlCharEncoding;
    fn xmlCharEncCloseFunc(handler: *mut xmlCharEncodingHandler) -> ::core::ffi::c_int;
    fn xmlParserInputBufferRead(
        in_0: xmlParserInputBufferPtr,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlParserGetDirectory(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlBuildRelativeURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlParseURI(str: *const ::core::ffi::c_char) -> xmlURIPtr;
    fn xmlSaveUri(uri: xmlURIPtr) -> *mut xmlChar;
    fn xmlURIEscape(str: *const xmlChar) -> *mut xmlChar;
    fn xmlFreeURI(uri: xmlURIPtr);
    fn xmlInitParser();
    fn xmlParseDocument(ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_int;
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlLoadExternalEntity(
        URL: *const ::core::ffi::c_char,
        ID: *const ::core::ffi::c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
    fn xmlXPtrNewContext(
        doc: xmlDocPtr,
        here: xmlNodePtr,
        origin: xmlNodePtr,
    ) -> xmlXPathContextPtr;
    fn xmlXPtrEval(str: *const xmlChar, ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
    fn xmlFreeInputStream(input: xmlParserInputPtr);
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr) -> ::core::ffi::c_int;
    fn xmlBufLength(buf: xmlBufPtr) -> size_t;
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
    fn xmlStaticCopyNode(
        node: xmlNodePtr,
        doc: xmlDocPtr,
        parent: xmlNodePtr,
        extended: ::core::ffi::c_int,
    ) -> xmlNodePtr;
    fn xmlStaticCopyNodeList(node: xmlNodePtr, doc: xmlDocPtr, parent: xmlNodePtr) -> xmlNodePtr;
}
pub type size_t = usize;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type xmlChar = ::core::ffi::c_uchar;
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
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type xmlHashScanner = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *const xmlChar) -> (),
>;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_1 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_1 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_1 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_1 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_1 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_1 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_1 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_1 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_1 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_1 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_1 = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed_1 = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_1 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_1 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_1 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_1 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_1 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_1 = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_1 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_1 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_1 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_1 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_1 = 1;
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
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathContext {
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub nb_variables_unused: ::core::ffi::c_int,
    pub max_variables_unused: ::core::ffi::c_int,
    pub varHash: xmlHashTablePtr,
    pub nb_types: ::core::ffi::c_int,
    pub max_types: ::core::ffi::c_int,
    pub types: xmlXPathTypePtr,
    pub nb_funcs_unused: ::core::ffi::c_int,
    pub max_funcs_unused: ::core::ffi::c_int,
    pub funcHash: xmlHashTablePtr,
    pub nb_axis: ::core::ffi::c_int,
    pub max_axis: ::core::ffi::c_int,
    pub axis: xmlXPathAxisPtr,
    pub namespaces: *mut xmlNsPtr,
    pub nsNr: ::core::ffi::c_int,
    pub user: *mut ::core::ffi::c_void,
    pub contextSize: ::core::ffi::c_int,
    pub proximityPosition: ::core::ffi::c_int,
    pub xptr: ::core::ffi::c_int,
    pub here: xmlNodePtr,
    pub origin: xmlNodePtr,
    pub nsHash: xmlHashTablePtr,
    pub varLookupFunc: xmlXPathVariableLookupFunc,
    pub varLookupData: *mut ::core::ffi::c_void,
    pub extra: *mut ::core::ffi::c_void,
    pub function: *const xmlChar,
    pub functionURI: *const xmlChar,
    pub funcLookupFunc: xmlXPathFuncLookupFunc,
    pub funcLookupData: *mut ::core::ffi::c_void,
    pub tmpNsList: *mut xmlNsPtr,
    pub tmpNsNr: ::core::ffi::c_int,
    pub userData: *mut ::core::ffi::c_void,
    pub error: xmlStructuredErrorFunc,
    pub lastError: xmlError,
    pub debugNode: xmlNodePtr,
    pub dict: xmlDictPtr,
    pub flags: ::core::ffi::c_int,
    pub cache: *mut ::core::ffi::c_void,
    pub opLimit: ::core::ffi::c_ulong,
    pub opCount: ::core::ffi::c_ulong,
    pub depth: ::core::ffi::c_int,
}
pub type xmlXPathFuncLookupFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlXPathFunction,
>;
pub type xmlXPathFunction =
    Option<unsafe extern "C" fn(xmlXPathParserContextPtr, ::core::ffi::c_int) -> ()>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: ::core::ffi::c_int,
    pub context: xmlXPathContextPtr,
    pub value: xmlXPathObjectPtr,
    pub valueNr: ::core::ffi::c_int,
    pub valueMax: ::core::ffi::c_int,
    pub valueTab: *mut xmlXPathObjectPtr,
    pub comp: xmlXPathCompExprPtr,
    pub xptr: ::core::ffi::c_int,
    pub ancestor: xmlNodePtr,
    pub valueFrame: ::core::ffi::c_int,
}
pub type xmlXPathCompExprPtr = *mut xmlXPathCompExpr;
pub type xmlXPathCompExpr = _xmlXPathCompExpr;
pub type xmlXPathObjectPtr = *mut xmlXPathObject;
pub type xmlXPathObject = _xmlXPathObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathObject {
    pub type_0: xmlXPathObjectType,
    pub nodesetval: xmlNodeSetPtr,
    pub boolval: ::core::ffi::c_int,
    pub floatval: ::core::ffi::c_double,
    pub stringval: *mut xmlChar,
    pub user: *mut ::core::ffi::c_void,
    pub index: ::core::ffi::c_int,
    pub user2: *mut ::core::ffi::c_void,
    pub index2: ::core::ffi::c_int,
}
pub type xmlNodeSetPtr = *mut xmlNodeSet;
pub type xmlNodeSet = _xmlNodeSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeSet {
    pub nodeNr: ::core::ffi::c_int,
    pub nodeMax: ::core::ffi::c_int,
    pub nodeTab: *mut xmlNodePtr,
}
pub type xmlXPathObjectType = ::core::ffi::c_uint;
pub const XPATH_XSLT_TREE: xmlXPathObjectType = 9;
pub const XPATH_USERS: xmlXPathObjectType = 8;
pub const XPATH_STRING: xmlXPathObjectType = 4;
pub const XPATH_NUMBER: xmlXPathObjectType = 3;
pub const XPATH_BOOLEAN: xmlXPathObjectType = 2;
pub const XPATH_NODESET: xmlXPathObjectType = 1;
pub const XPATH_UNDEFINED: xmlXPathObjectType = 0;
pub type xmlXPathContextPtr = *mut xmlXPathContext;
pub type xmlXPathContext = _xmlXPathContext;
pub type xmlXPathVariableLookupFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlXPathObjectPtr,
>;
pub type xmlXPathAxisPtr = *mut xmlXPathAxis;
pub type xmlXPathAxis = _xmlXPathAxis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathAxis {
    pub name: *const xmlChar,
    pub func: xmlXPathAxisFunc,
}
pub type xmlXPathAxisFunc =
    Option<unsafe extern "C" fn(xmlXPathParserContextPtr, xmlXPathObjectPtr) -> xmlXPathObjectPtr>;
pub type xmlXPathTypePtr = *mut xmlXPathType;
pub type xmlXPathType = _xmlXPathType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathType {
    pub name: *const xmlChar,
    pub func: xmlXPathConvertFunc,
}
pub type xmlXPathConvertFunc =
    Option<unsafe extern "C" fn(xmlXPathObjectPtr, ::core::ffi::c_int) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeCtxt {
    pub doc: xmlDocPtr,
    pub incNr: ::core::ffi::c_int,
    pub incMax: ::core::ffi::c_int,
    pub incTab: *mut xmlXIncludeRefPtr,
    pub txtNr: ::core::ffi::c_int,
    pub txtMax: ::core::ffi::c_int,
    pub txtTab: *mut xmlXIncludeTxt,
    pub urlNr: ::core::ffi::c_int,
    pub urlMax: ::core::ffi::c_int,
    pub urlTab: *mut xmlXIncludeDoc,
    pub nbErrors: ::core::ffi::c_int,
    pub fatalErr: ::core::ffi::c_int,
    pub legacy: ::core::ffi::c_int,
    pub parseFlags: ::core::ffi::c_int,
    pub base: *mut xmlChar,
    pub _private: *mut ::core::ffi::c_void,
    pub depth: ::core::ffi::c_int,
    pub isStream: ::core::ffi::c_int,
}
pub type xmlXIncludeDoc = _xmlXIncludeDoc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeDoc {
    pub doc: xmlDocPtr,
    pub url: *mut xmlChar,
    pub expanding: ::core::ffi::c_int,
}
pub type xmlXIncludeTxt = _xmlXIncludeTxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeTxt {
    pub text: *mut xmlChar,
    pub url: *mut xmlChar,
}
pub type xmlXIncludeRefPtr = *mut xmlXIncludeRef;
pub type xmlXIncludeRef = _xmlXIncludeRef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeRef {
    pub URI: *mut xmlChar,
    pub fragment: *mut xmlChar,
    pub elem: xmlNodePtr,
    pub inc: xmlNodePtr,
    pub xml: ::core::ffi::c_int,
    pub fallback: ::core::ffi::c_int,
    pub emptyFb: ::core::ffi::c_int,
    pub expanding: ::core::ffi::c_int,
    pub replace: ::core::ffi::c_int,
}
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
pub type xmlXIncludeDocPtr = *mut xmlXIncludeDoc;
pub type xmlURL = *mut xmlChar;
pub type xmlXIncludeMergeData = _xmlXIncludeMergeData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXIncludeMergeData {
    pub doc: xmlDocPtr,
    pub ctxt: xmlXIncludeCtxtPtr,
}
pub type xmlXIncludeMergeDataPtr = *mut xmlXIncludeMergeData;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XML_XML_NAMESPACE: *const xmlChar = b"http://www.w3.org/XML/1998/namespace\0" as *const u8
    as *const ::core::ffi::c_char as *const xmlChar;
pub const XML_DETECT_IDS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XINCLUDE_MAX_DEPTH: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
unsafe extern "C" fn xmlXIncludeErrMemory(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
    mut extra: *const ::core::ffi::c_char,
) {
    if !ctxt.is_null() {
        (*ctxt).nbErrors += 1;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        node as *mut ::core::ffi::c_void,
        XML_FROM_XINCLUDE as ::core::ffi::c_int,
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
unsafe extern "C" fn xmlXIncludeErr(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
    mut error: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
    mut extra: *const xmlChar,
) {
    if !ctxt.is_null() {
        (*ctxt).nbErrors += 1;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut ::core::ffi::c_void,
        node as *mut ::core::ffi::c_void,
        XML_FROM_XINCLUDE as ::core::ffi::c_int,
        error,
        XML_ERR_ERROR,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        extra as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        msg,
        extra as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn xmlXIncludeGetProp(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut cur: xmlNodePtr,
    mut name: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    ret = xmlGetNsProp(cur as *const xmlNode, XINCLUDE_NS, name);
    if !ret.is_null() {
        return ret;
    }
    if (*ctxt).legacy != 0 as ::core::ffi::c_int {
        ret = xmlGetNsProp(cur as *const xmlNode, XINCLUDE_OLD_NS, name);
        if !ret.is_null() {
            return ret;
        }
    }
    ret = xmlGetProp(cur as *const xmlNode, name);
    return ret;
}
unsafe extern "C" fn xmlXIncludeFreeRef(mut ref_0: xmlXIncludeRefPtr) {
    if ref_0.is_null() {
        return;
    }
    if !(*ref_0).URI.is_null() {
        xmlFree.expect("non-null function pointer")((*ref_0).URI as *mut ::core::ffi::c_void);
    }
    if !(*ref_0).fragment.is_null() {
        xmlFree.expect("non-null function pointer")((*ref_0).fragment as *mut ::core::ffi::c_void);
    }
    xmlFree.expect("non-null function pointer")(ref_0 as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlXIncludeNewRef(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut URI: *const xmlChar,
    mut elem: xmlNodePtr,
) -> xmlXIncludeRefPtr {
    let mut ret: xmlXIncludeRefPtr = ::core::ptr::null_mut::<xmlXIncludeRef>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXIncludeRef>() as size_t
    ) as xmlXIncludeRefPtr;
    if ret.is_null() {
        xmlXIncludeErrMemory(
            ctxt,
            elem,
            b"growing XInclude context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlXIncludeRef>() as size_t,
    );
    if URI.is_null() {
        (*ret).URI = ::core::ptr::null_mut::<xmlChar>();
    } else {
        (*ret).URI = xmlStrdup(URI);
    }
    (*ret).fragment = ::core::ptr::null_mut::<xmlChar>();
    (*ret).elem = elem;
    (*ret).xml = 0 as ::core::ffi::c_int;
    (*ret).inc = ::core::ptr::null_mut::<xmlNode>();
    if (*ctxt).incNr >= (*ctxt).incMax {
        let mut tmp: *mut xmlXIncludeRefPtr = ::core::ptr::null_mut::<xmlXIncludeRefPtr>();
        let mut newSize: size_t = (if (*ctxt).incMax != 0 {
            (*ctxt).incMax * 2 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        }) as size_t;
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).incTab as *mut ::core::ffi::c_void,
            newSize.wrapping_mul(::core::mem::size_of::<xmlXIncludeRefPtr>() as size_t),
        ) as *mut xmlXIncludeRefPtr;
        if tmp.is_null() {
            xmlXIncludeErrMemory(
                ctxt,
                elem,
                b"growing XInclude context\0" as *const u8 as *const ::core::ffi::c_char,
            );
            xmlXIncludeFreeRef(ret);
            return ::core::ptr::null_mut::<xmlXIncludeRef>();
        }
        (*ctxt).incTab = tmp;
        (*ctxt).incMax = newSize as ::core::ffi::c_int;
    }
    let fresh8 = (*ctxt).incNr;
    (*ctxt).incNr = (*ctxt).incNr + 1;
    let ref mut fresh9 = *(*ctxt).incTab.offset(fresh8 as isize);
    *fresh9 = ret;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeNewContext(mut doc: xmlDocPtr) -> xmlXIncludeCtxtPtr {
    let mut ret: xmlXIncludeCtxtPtr = ::core::ptr::null_mut::<xmlXIncludeCtxt>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlXIncludeCtxt>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXIncludeCtxt>() as size_t
    ) as xmlXIncludeCtxtPtr;
    if ret.is_null() {
        xmlXIncludeErrMemory(
            ::core::ptr::null_mut::<xmlXIncludeCtxt>(),
            doc as xmlNodePtr,
            b"creating XInclude context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlXIncludeCtxt>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlXIncludeCtxt>() as size_t,
    );
    (*ret).doc = doc;
    (*ret).incNr = 0 as ::core::ffi::c_int;
    (*ret).incMax = 0 as ::core::ffi::c_int;
    (*ret).incTab = ::core::ptr::null_mut::<xmlXIncludeRefPtr>();
    (*ret).nbErrors = 0 as ::core::ffi::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeFreeContext(mut ctxt: xmlXIncludeCtxtPtr) {
    let mut i: ::core::ffi::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if !(*ctxt).urlTab.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*ctxt).urlNr {
            xmlFreeDoc((*(*ctxt).urlTab.offset(i as isize)).doc);
            xmlFree.expect("non-null function pointer")(
                (*(*ctxt).urlTab.offset(i as isize)).url as *mut ::core::ffi::c_void,
            );
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*ctxt).urlTab as *mut ::core::ffi::c_void);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*ctxt).incNr {
        if !(*(*ctxt).incTab.offset(i as isize)).is_null() {
            xmlXIncludeFreeRef(*(*ctxt).incTab.offset(i as isize));
        }
        i += 1;
    }
    if !(*ctxt).incTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).incTab as *mut ::core::ffi::c_void);
    }
    if !(*ctxt).txtTab.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*ctxt).txtNr {
            xmlFree.expect("non-null function pointer")(
                (*(*ctxt).txtTab.offset(i as isize)).text as *mut ::core::ffi::c_void,
            );
            xmlFree.expect("non-null function pointer")(
                (*(*ctxt).txtTab.offset(i as isize)).url as *mut ::core::ffi::c_void,
            );
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*ctxt).txtTab as *mut ::core::ffi::c_void);
    }
    if !(*ctxt).base.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).base as *mut ::core::ffi::c_void);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlXIncludeParseFile(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut URL: *const ::core::ffi::c_char,
) -> xmlDocPtr {
    let mut ret: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut pctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    xmlInitParser();
    pctxt = xmlNewParserCtxt();
    if pctxt.is_null() {
        xmlXIncludeErrMemory(
            ctxt,
            ::core::ptr::null_mut::<xmlNode>(),
            b"cannot allocate parser context\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    (*pctxt)._private = (*ctxt)._private;
    if !(*ctxt).doc.is_null() && !(*(*ctxt).doc).dict.is_null() {
        if !(*pctxt).dict.is_null() {
            xmlDictFree((*pctxt).dict);
        }
        (*pctxt).dict = (*(*ctxt).doc).dict as xmlDictPtr;
        xmlDictReference((*pctxt).dict);
    }
    xmlCtxtUseOptions(
        pctxt,
        (*ctxt).parseFlags | XML_PARSE_DTDLOAD as ::core::ffi::c_int,
    );
    if !URL.is_null()
        && strcmp(URL, b"-\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        URL = b"./-\0" as *const u8 as *const ::core::ffi::c_char;
    }
    inputStream = xmlLoadExternalEntity(URL, ::core::ptr::null::<::core::ffi::c_char>(), pctxt);
    if inputStream.is_null() {
        xmlFreeParserCtxt(pctxt);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    inputPush(pctxt, inputStream);
    if (*pctxt).directory.is_null() {
        (*pctxt).directory = xmlParserGetDirectory(URL);
    }
    (*pctxt).loadsubset |= XML_DETECT_IDS;
    xmlParseDocument(pctxt);
    if (*pctxt).wellFormed != 0 {
        ret = (*pctxt).myDoc;
    } else {
        ret = ::core::ptr::null_mut::<xmlDoc>();
        if !(*pctxt).myDoc.is_null() {
            xmlFreeDoc((*pctxt).myDoc);
        }
        (*pctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
    }
    xmlFreeParserCtxt(pctxt);
    return ret;
}
unsafe extern "C" fn xmlXIncludeAddNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut cur: xmlNodePtr,
) -> xmlXIncludeRefPtr {
    let mut ref_0: xmlXIncludeRefPtr = ::core::ptr::null_mut::<xmlXIncludeRef>();
    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    let mut URL: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut fragment: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut href: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut parse: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut base: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut URI: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut xml: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut local: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    href = xmlXIncludeGetProp(ctxt, cur, XINCLUDE_HREF);
    if href.is_null() {
        href = xmlStrdup(b"\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
        if href.is_null() {
            return ::core::ptr::null_mut::<xmlXIncludeRef>();
        }
    }
    parse = xmlXIncludeGetProp(ctxt, cur, XINCLUDE_PARSE);
    if !parse.is_null() {
        if xmlStrEqual(parse, XINCLUDE_PARSE_XML) != 0 {
            xml = 1 as ::core::ffi::c_int;
        } else if xmlStrEqual(parse, XINCLUDE_PARSE_TEXT) != 0 {
            xml = 0 as ::core::ffi::c_int;
        } else {
            xmlXIncludeErr(
                ctxt,
                cur,
                XML_XINCLUDE_PARSE_VALUE as ::core::ffi::c_int,
                b"invalid value %s for 'parse'\n\0" as *const u8 as *const ::core::ffi::c_char,
                parse,
            );
            if !href.is_null() {
                xmlFree.expect("non-null function pointer")(href as *mut ::core::ffi::c_void);
            }
            if !parse.is_null() {
                xmlFree.expect("non-null function pointer")(parse as *mut ::core::ffi::c_void);
            }
            return ::core::ptr::null_mut::<xmlXIncludeRef>();
        }
    }
    base = xmlNodeGetBase((*ctxt).doc as *const xmlDoc, cur as *const xmlNode);
    if base.is_null() {
        URI = xmlBuildURI(href, (*(*ctxt).doc).URL);
    } else {
        URI = xmlBuildURI(href, base);
    }
    if URI.is_null() {
        let mut escbase: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut eschref: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        escbase = xmlURIEscape(base);
        eschref = xmlURIEscape(href);
        URI = xmlBuildURI(eschref, escbase);
        if !escbase.is_null() {
            xmlFree.expect("non-null function pointer")(escbase as *mut ::core::ffi::c_void);
        }
        if !eschref.is_null() {
            xmlFree.expect("non-null function pointer")(eschref as *mut ::core::ffi::c_void);
        }
    }
    if !parse.is_null() {
        xmlFree.expect("non-null function pointer")(parse as *mut ::core::ffi::c_void);
    }
    if !href.is_null() {
        xmlFree.expect("non-null function pointer")(href as *mut ::core::ffi::c_void);
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as *mut ::core::ffi::c_void);
    }
    if URI.is_null() {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
            b"failed build URL\n\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<xmlChar>(),
        );
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    fragment = xmlXIncludeGetProp(ctxt, cur, XINCLUDE_PARSE_XPOINTER);
    uri = xmlParseURI(URI as *const ::core::ffi::c_char);
    if uri.is_null() {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            URI,
        );
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as *mut ::core::ffi::c_void);
        }
        xmlFree.expect("non-null function pointer")(URI as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    if !(*uri).fragment.is_null() {
        if (*ctxt).legacy != 0 as ::core::ffi::c_int {
            if fragment.is_null() {
                fragment = (*uri).fragment as *mut xmlChar;
            } else {
                xmlFree.expect("non-null function pointer")(
                    (*uri).fragment as *mut ::core::ffi::c_void,
                );
            }
        } else {
            xmlXIncludeErr(
                ctxt,
                cur,
                XML_XINCLUDE_FRAGMENT_ID as ::core::ffi::c_int,
                b"Invalid fragment identifier in URI %s use the xpointer attribute\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                URI,
            );
            if !fragment.is_null() {
                xmlFree.expect("non-null function pointer")(fragment as *mut ::core::ffi::c_void);
            }
            xmlFreeURI(uri);
            xmlFree.expect("non-null function pointer")(URI as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<xmlXIncludeRef>();
        }
        (*uri).fragment = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    URL = xmlSaveUri(uri);
    xmlFreeURI(uri);
    if URL.is_null() {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            URI,
        );
        if !fragment.is_null() {
            xmlFree.expect("non-null function pointer")(fragment as *mut ::core::ffi::c_void);
        }
        xmlFree.expect("non-null function pointer")(URI as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    xmlFree.expect("non-null function pointer")(URI as *mut ::core::ffi::c_void);
    if xmlStrEqual(URL, (*(*ctxt).doc).URL) != 0 {
        local = 1 as ::core::ffi::c_int;
    }
    if local == 1 as ::core::ffi::c_int
        && xml == 1 as ::core::ffi::c_int
        && (fragment.is_null()
            || *fragment.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int)
    {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_RECURSION as ::core::ffi::c_int,
            b"detected a local recursion with no xpointer in %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            URL,
        );
        xmlFree.expect("non-null function pointer")(URL as *mut ::core::ffi::c_void);
        xmlFree.expect("non-null function pointer")(fragment as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    ref_0 = xmlXIncludeNewRef(ctxt, URL, cur);
    xmlFree.expect("non-null function pointer")(URL as *mut ::core::ffi::c_void);
    if ref_0.is_null() {
        xmlFree.expect("non-null function pointer")(fragment as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    (*ref_0).fragment = fragment;
    (*ref_0).xml = xml;
    return ref_0;
}
unsafe extern "C" fn xmlXIncludeRecurseDoc(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut doc: xmlDocPtr,
    url: xmlURL,
) {
    let mut oldDoc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut oldIncTab: *mut xmlXIncludeRefPtr = ::core::ptr::null_mut::<xmlXIncludeRefPtr>();
    let mut oldIncMax: ::core::ffi::c_int = 0;
    let mut oldIncNr: ::core::ffi::c_int = 0;
    let mut oldIsStream: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    oldDoc = (*ctxt).doc;
    oldIncMax = (*ctxt).incMax;
    oldIncNr = (*ctxt).incNr;
    oldIncTab = (*ctxt).incTab;
    oldIsStream = (*ctxt).isStream;
    (*ctxt).doc = doc;
    (*ctxt).incMax = 0 as ::core::ffi::c_int;
    (*ctxt).incNr = 0 as ::core::ffi::c_int;
    (*ctxt).incTab = ::core::ptr::null_mut::<xmlXIncludeRefPtr>();
    (*ctxt).isStream = 0 as ::core::ffi::c_int;
    xmlXIncludeDoProcess(ctxt, xmlDocGetRootElement(doc as *const xmlDoc));
    if !(*ctxt).incTab.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*ctxt).incNr {
            xmlXIncludeFreeRef(*(*ctxt).incTab.offset(i as isize));
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*ctxt).incTab as *mut ::core::ffi::c_void);
    }
    (*ctxt).doc = oldDoc;
    (*ctxt).incMax = oldIncMax;
    (*ctxt).incNr = oldIncNr;
    (*ctxt).incTab = oldIncTab;
    (*ctxt).isStream = oldIsStream;
}
unsafe extern "C" fn xmlXIncludeCopyNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut elem: xmlNodePtr,
    mut copyChildren: ::core::ffi::c_int,
) -> xmlNodePtr {
    let mut result: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut insertParent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut insertLast: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if copyChildren != 0 {
        cur = (*elem).children as xmlNodePtr;
        if cur.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
    } else {
        cur = elem;
    }
    loop {
        let mut copy: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        let mut recurse: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !((*cur).type_0 as ::core::ffi::c_uint
            == XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cur).type_0 as ::core::ffi::c_uint
                == XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            if (*cur).type_0 as ::core::ffi::c_uint
                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                && !(*cur).ns.is_null()
                && xmlStrEqual((*cur).name, XINCLUDE_NODE) != 0
                && (xmlStrEqual((*(*cur).ns).href, XINCLUDE_NS) != 0
                    || xmlStrEqual((*(*cur).ns).href, XINCLUDE_OLD_NS) != 0)
            {
                let mut ref_0: xmlXIncludeRefPtr = xmlXIncludeExpandNode(ctxt, cur);
                if ref_0.is_null() {
                    break;
                }
                if !(*ref_0).inc.is_null() {
                    copy = xmlStaticCopyNodeList((*ref_0).inc, (*ctxt).doc, insertParent);
                    if copy.is_null() {
                        break;
                    }
                }
            } else {
                copy = xmlStaticCopyNode(cur, (*ctxt).doc, insertParent, 2 as ::core::ffi::c_int);
                if copy.is_null() {
                    break;
                }
                recurse = ((*cur).type_0 as ::core::ffi::c_uint
                    != XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !(*cur).children.is_null()) as ::core::ffi::c_int;
            }
        }
        if !copy.is_null() {
            if result.is_null() {
                result = copy;
            }
            if !insertLast.is_null() {
                (*insertLast).next = copy as *mut _xmlNode;
                (*copy).prev = insertLast as *mut _xmlNode;
            } else if !insertParent.is_null() {
                (*insertParent).children = copy as *mut _xmlNode;
            }
            insertLast = copy;
            while !(*insertLast).next.is_null() {
                insertLast = (*insertLast).next as xmlNodePtr;
            }
        }
        if recurse != 0 {
            cur = (*cur).children as xmlNodePtr;
            insertParent = insertLast;
            insertLast = ::core::ptr::null_mut::<xmlNode>();
        } else {
            if cur == elem {
                return result;
            }
            while (*cur).next.is_null() {
                if !insertParent.is_null() {
                    (*insertParent).last = insertLast as *mut _xmlNode;
                }
                cur = (*cur).parent as xmlNodePtr;
                if cur == elem {
                    return result;
                }
                insertLast = insertParent;
                insertParent = (*insertParent).parent as xmlNodePtr;
            }
            cur = (*cur).next as xmlNodePtr;
        }
    }
    xmlFreeNodeList(result);
    return ::core::ptr::null_mut::<xmlNode>();
}
unsafe extern "C" fn xmlXIncludeCopyXPointer(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut obj: xmlXPathObjectPtr,
) -> xmlNodePtr {
    let mut list: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut last: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut copy: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut i: ::core::ffi::c_int = 0;
    if ctxt.is_null() || obj.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    match (*obj).type_0 as ::core::ffi::c_uint {
        1 => {
            let mut set: xmlNodeSetPtr = (*obj).nodesetval;
            if set.is_null() {
                return ::core::ptr::null_mut::<xmlNode>();
            }
            let mut current_block_22: u64;
            i = 0 as ::core::ffi::c_int;
            while i < (*set).nodeNr {
                let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                if !(*(*set).nodeTab.offset(i as isize)).is_null() {
                    match (**(*set).nodeTab.offset(i as isize)).type_0 as ::core::ffi::c_uint {
                        9 | 13 => {
                            node = xmlDocGetRootElement(*(*set).nodeTab.offset(i as isize)
                                as xmlDocPtr
                                as *const xmlDoc);
                            if node.is_null() {
                                xmlXIncludeErr(
                                    ctxt,
                                    *(*set).nodeTab.offset(i as isize),
                                    XML_ERR_INTERNAL_ERROR as ::core::ffi::c_int,
                                    b"document without root\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    ::core::ptr::null::<xmlChar>(),
                                );
                                current_block_22 = 15240798224410183470;
                            } else {
                                current_block_22 = 15976848397966268834;
                            }
                        }
                        3 | 4 | 1 | 7 | 8 => {
                            node = *(*set).nodeTab.offset(i as isize);
                            current_block_22 = 15976848397966268834;
                        }
                        _ => {
                            xmlXIncludeErr(
                                ctxt,
                                *(*set).nodeTab.offset(i as isize),
                                XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                b"invalid node type in XPtr result\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            current_block_22 = 15240798224410183470;
                        }
                    }
                    match current_block_22 {
                        15240798224410183470 => {}
                        _ => {
                            copy = xmlXIncludeCopyNode(ctxt, node, 0 as ::core::ffi::c_int);
                            if copy.is_null() {
                                xmlFreeNodeList(list);
                                return ::core::ptr::null_mut::<xmlNode>();
                            }
                            if last.is_null() {
                                list = copy;
                            } else {
                                while !(*last).next.is_null() {
                                    last = (*last).next as xmlNodePtr;
                                }
                                (*copy).prev = last as *mut _xmlNode;
                                (*last).next = copy as *mut _xmlNode;
                            }
                            last = copy;
                        }
                    }
                }
                i += 1;
            }
        }
        _ => {}
    }
    return list;
}
unsafe extern "C" fn xmlXIncludeMergeEntity(
    mut payload: *mut ::core::ffi::c_void,
    mut vdata: *mut ::core::ffi::c_void,
    mut name: *const xmlChar,
) {
    let mut current_block: u64;
    let mut ent: xmlEntityPtr = payload as xmlEntityPtr;
    let mut data: xmlXIncludeMergeDataPtr = vdata as xmlXIncludeMergeDataPtr;
    let mut ret: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut prev: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut ctxt: xmlXIncludeCtxtPtr = ::core::ptr::null_mut::<xmlXIncludeCtxt>();
    if ent.is_null() || data.is_null() {
        return;
    }
    ctxt = (*data).ctxt;
    doc = (*data).doc;
    if ctxt.is_null() || doc.is_null() {
        return;
    }
    match (*ent).etype as ::core::ffi::c_uint {
        4 | 5 | 6 => return,
        1 | 2 | 3 | _ => {}
    }
    ret = xmlAddDocEntity(
        doc,
        (*ent).name,
        (*ent).etype as ::core::ffi::c_int,
        (*ent).ExternalID,
        (*ent).SystemID,
        (*ent).content,
    );
    if !ret.is_null() {
        if !(*ent).URI.is_null() {
            (*ret).URI = xmlStrdup((*ent).URI);
        }
    } else {
        prev = xmlGetDocEntity(doc as *const xmlDoc, (*ent).name);
        if !prev.is_null() {
            if (*ent).etype as ::core::ffi::c_uint != (*prev).etype as ::core::ffi::c_uint {
                current_block = 13397657639218570250;
            } else if !(*ent).SystemID.is_null() && !(*prev).SystemID.is_null() {
                if xmlStrEqual((*ent).SystemID, (*prev).SystemID) == 0 {
                    current_block = 13397657639218570250;
                } else {
                    current_block = 9828876828309294594;
                }
            } else if !(*ent).ExternalID.is_null() && !(*prev).ExternalID.is_null() {
                if xmlStrEqual((*ent).ExternalID, (*prev).ExternalID) == 0 {
                    current_block = 13397657639218570250;
                } else {
                    current_block = 9828876828309294594;
                }
            } else if !(*ent).content.is_null() && !(*prev).content.is_null() {
                if xmlStrEqual((*ent).content, (*prev).content) == 0 {
                    current_block = 13397657639218570250;
                } else {
                    current_block = 9828876828309294594;
                }
            } else {
                current_block = 13397657639218570250;
            }
            match current_block {
                9828876828309294594 => {}
                _ => {
                    match (*ent).etype as ::core::ffi::c_uint {
                        4 | 5 | 6 | 1 | 2 => return,
                        3 | _ => {}
                    }
                    xmlXIncludeErr(
                        ctxt,
                        ent as xmlNodePtr,
                        XML_XINCLUDE_ENTITY_DEF_MISMATCH as ::core::ffi::c_int,
                        b"mismatch in redefinition of entity %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*ent).name,
                    );
                    return;
                }
            }
        }
    };
}
unsafe extern "C" fn xmlXIncludeMergeEntities(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut doc: xmlDocPtr,
    mut from: xmlDocPtr,
) -> ::core::ffi::c_int {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut target: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut source: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if from.is_null() || (*from).intSubset.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    target = (*doc).intSubset as xmlDtdPtr;
    if target.is_null() {
        cur = xmlDocGetRootElement(doc as *const xmlDoc);
        if cur.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        target = xmlCreateIntSubset(
            doc,
            (*cur).name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        if target.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    }
    source = (*from).intSubset as xmlDtdPtr;
    if !source.is_null() && !(*source).entities.is_null() {
        let mut data: xmlXIncludeMergeData = xmlXIncludeMergeData {
            doc: ::core::ptr::null_mut::<xmlDoc>(),
            ctxt: ::core::ptr::null_mut::<xmlXIncludeCtxt>(),
        };
        data.ctxt = ctxt;
        data.doc = doc;
        xmlHashScan(
            (*source).entities as xmlHashTablePtr,
            Some(
                xmlXIncludeMergeEntity
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            &raw mut data as *mut ::core::ffi::c_void,
        );
    }
    source = (*from).extSubset as xmlDtdPtr;
    if !source.is_null() && !(*source).entities.is_null() {
        let mut data_0: xmlXIncludeMergeData = xmlXIncludeMergeData {
            doc: ::core::ptr::null_mut::<xmlDoc>(),
            ctxt: ::core::ptr::null_mut::<xmlXIncludeCtxt>(),
        };
        data_0.ctxt = ctxt;
        data_0.doc = doc;
        if xmlStrEqual((*target).ExternalID, (*source).ExternalID) == 0
            && xmlStrEqual((*target).SystemID, (*source).SystemID) == 0
        {
            xmlHashScan(
                (*source).entities as xmlHashTablePtr,
                Some(
                    xmlXIncludeMergeEntity
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_void,
                            *const xmlChar,
                        ) -> (),
                ),
                &raw mut data_0 as *mut ::core::ffi::c_void,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlXIncludeLoadDoc(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut url: *const xmlChar,
    mut ref_0: xmlXIncludeRefPtr,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut cache: xmlXIncludeDocPtr = ::core::ptr::null_mut::<xmlXIncludeDoc>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    let mut URL: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut fragment: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cacheNr: ::core::ffi::c_int = 0;
    let mut saveFlags: ::core::ffi::c_int = 0;
    uri = xmlParseURI(url as *const ::core::ffi::c_char);
    if uri.is_null() {
        xmlXIncludeErr(
            ctxt,
            (*ref_0).elem,
            XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            url,
        );
    } else {
        if !(*uri).fragment.is_null() {
            fragment = (*uri).fragment as *mut xmlChar;
            (*uri).fragment = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(*ref_0).fragment.is_null() {
            if !fragment.is_null() {
                xmlFree.expect("non-null function pointer")(fragment as *mut ::core::ffi::c_void);
            }
            fragment = xmlStrdup((*ref_0).fragment);
        }
        URL = xmlSaveUri(uri);
        xmlFreeURI(uri);
        if URL.is_null() {
            xmlXIncludeErr(
                ctxt,
                (*ref_0).elem,
                XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
                b"invalid value URI %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                url,
            );
        } else {
            if *URL.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                || *URL.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
                || !(*ctxt).doc.is_null() && xmlStrEqual(URL, (*(*ctxt).doc).URL) != 0
            {
                doc = (*ctxt).doc;
                current_block = 14220223478914065737;
            } else {
                i = 0 as ::core::ffi::c_int;
                loop {
                    if !(i < (*ctxt).urlNr) {
                        current_block = 18386322304582297246;
                        break;
                    }
                    if xmlStrEqual(URL, (*(*ctxt).urlTab.offset(i as isize)).url) != 0 {
                        if (*(*ctxt).urlTab.offset(i as isize)).expanding != 0 {
                            xmlXIncludeErr(
                                ctxt,
                                (*ref_0).elem,
                                XML_XINCLUDE_RECURSION as ::core::ffi::c_int,
                                b"inclusion loop detected\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            current_block = 17083819973035331397;
                            break;
                        } else {
                            doc = (*(*ctxt).urlTab.offset(i as isize)).doc;
                            if doc.is_null() {
                                current_block = 17083819973035331397;
                                break;
                            } else {
                                current_block = 14220223478914065737;
                                break;
                            }
                        }
                    } else {
                        i += 1;
                    }
                }
                match current_block {
                    17083819973035331397 => {}
                    14220223478914065737 => {}
                    _ => {
                        saveFlags = (*ctxt).parseFlags;
                        if !fragment.is_null() {
                            (*ctxt).parseFlags |= XML_PARSE_NOENT as ::core::ffi::c_int;
                        }
                        doc = xmlXIncludeParseFile(ctxt, URL as *const ::core::ffi::c_char);
                        (*ctxt).parseFlags = saveFlags;
                        if (*ctxt).urlNr >= (*ctxt).urlMax {
                            let mut tmp: *mut xmlXIncludeDoc =
                                ::core::ptr::null_mut::<xmlXIncludeDoc>();
                            let mut newSize: size_t = (if (*ctxt).urlMax != 0 {
                                (*ctxt).urlMax * 2 as ::core::ffi::c_int
                            } else {
                                8 as ::core::ffi::c_int
                            }) as size_t;
                            tmp = xmlRealloc.expect("non-null function pointer")(
                                (*ctxt).urlTab as *mut ::core::ffi::c_void,
                                (::core::mem::size_of::<xmlXIncludeDoc>() as size_t)
                                    .wrapping_mul(newSize),
                            ) as *mut xmlXIncludeDoc;
                            if tmp.is_null() {
                                xmlXIncludeErrMemory(
                                    ctxt,
                                    (*ref_0).elem,
                                    b"growing XInclude URL table\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                xmlFreeDoc(doc);
                                current_block = 17083819973035331397;
                            } else {
                                (*ctxt).urlMax = newSize as ::core::ffi::c_int;
                                (*ctxt).urlTab = tmp;
                                current_block = 6450636197030046351;
                            }
                        } else {
                            current_block = 6450636197030046351;
                        }
                        match current_block {
                            17083819973035331397 => {}
                            _ => {
                                let fresh3 = (*ctxt).urlNr;
                                (*ctxt).urlNr = (*ctxt).urlNr + 1;
                                cacheNr = fresh3;
                                cache = (*ctxt).urlTab.offset(cacheNr as isize)
                                    as *mut xmlXIncludeDoc
                                    as xmlXIncludeDocPtr;
                                (*cache).doc = doc;
                                (*cache).url = xmlStrdup(URL);
                                (*cache).expanding = 0 as ::core::ffi::c_int;
                                if doc.is_null() {
                                    current_block = 17083819973035331397;
                                } else {
                                    if xmlStrEqual(URL, (*doc).URL) == 0 {
                                        xmlFree.expect("non-null function pointer")(
                                            URL as *mut ::core::ffi::c_void,
                                        );
                                        URL = xmlStrdup((*doc).URL);
                                    }
                                    xmlXIncludeMergeEntities(ctxt, (*ctxt).doc, doc);
                                    (*cache).expanding = 1 as ::core::ffi::c_int;
                                    xmlXIncludeRecurseDoc(ctxt, doc, URL as xmlURL);
                                    cache = (*ctxt).urlTab.offset(cacheNr as isize)
                                        as *mut xmlXIncludeDoc
                                        as xmlXIncludeDocPtr;
                                    (*cache).expanding = 0 as ::core::ffi::c_int;
                                    current_block = 14220223478914065737;
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                17083819973035331397 => {}
                _ => {
                    if fragment.is_null() {
                        (*ref_0).inc = xmlDocCopyNode(
                            xmlDocGetRootElement(doc as *const xmlDoc),
                            (*ctxt).doc,
                            1 as ::core::ffi::c_int,
                        );
                        current_block = 16791665189521845338;
                    } else {
                        let mut xptr: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
                        let mut xptrctxt: xmlXPathContextPtr =
                            ::core::ptr::null_mut::<xmlXPathContext>();
                        let mut set: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
                        if (*ctxt).isStream != 0 && doc == (*ctxt).doc {
                            xmlXIncludeErr(
                                ctxt,
                                (*ref_0).elem,
                                XML_XINCLUDE_XPTR_FAILED as ::core::ffi::c_int,
                                b"XPointer expressions not allowed in streaming mode\n\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            current_block = 17083819973035331397;
                        } else {
                            xptrctxt = xmlXPtrNewContext(
                                doc,
                                ::core::ptr::null_mut::<xmlNode>(),
                                ::core::ptr::null_mut::<xmlNode>(),
                            );
                            if xptrctxt.is_null() {
                                xmlXIncludeErr(
                                    ctxt,
                                    (*ref_0).elem,
                                    XML_XINCLUDE_XPTR_FAILED as ::core::ffi::c_int,
                                    b"could not create XPointer context\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    ::core::ptr::null::<xmlChar>(),
                                );
                                current_block = 17083819973035331397;
                            } else {
                                xptr = xmlXPtrEval(fragment, xptrctxt);
                                if xptr.is_null() {
                                    xmlXIncludeErr(
                                        ctxt,
                                        (*ref_0).elem,
                                        XML_XINCLUDE_XPTR_FAILED as ::core::ffi::c_int,
                                        b"XPointer evaluation failed: #%s\n\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        fragment,
                                    );
                                    xmlXPathFreeContext(xptrctxt);
                                    current_block = 17083819973035331397;
                                } else {
                                    match (*xptr).type_0 as ::core::ffi::c_uint {
                                        0 | 2 | 3 | 4 | 8 | 9 => {
                                            xmlXIncludeErr(
                                                ctxt,
                                                (*ref_0).elem,
                                                XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                b"XPointer is not a range: #%s\n\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                fragment,
                                            );
                                            xmlXPathFreeObject(xptr);
                                            xmlXPathFreeContext(xptrctxt);
                                            current_block = 17083819973035331397;
                                        }
                                        1 => {
                                            if (*xptr).nodesetval.is_null()
                                                || (*(*xptr).nodesetval).nodeNr
                                                    <= 0 as ::core::ffi::c_int
                                            {
                                                xmlXPathFreeObject(xptr);
                                                xmlXPathFreeContext(xptrctxt);
                                                current_block = 17083819973035331397;
                                            } else {
                                                current_block = 15970011996474399071;
                                            }
                                        }
                                        _ => {
                                            current_block = 15970011996474399071;
                                        }
                                    }
                                    match current_block {
                                        17083819973035331397 => {}
                                        _ => {
                                            set = (*xptr).nodesetval;
                                            if !set.is_null() {
                                                let mut current_block_64: u64;
                                                i = 0 as ::core::ffi::c_int;
                                                while i < (*set).nodeNr {
                                                    if !(*(*set).nodeTab.offset(i as isize))
                                                        .is_null()
                                                    {
                                                        match (**(*set).nodeTab.offset(i as isize))
                                                            .type_0
                                                            as ::core::ffi::c_uint
                                                        {
                                                            2 => {
                                                                current_block_64 =
                                                                    4513888698325685980;
                                                                match current_block_64 {
                                                                    4513888698325685980 => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects an attribute: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh4 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh4 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                    7360144896257856683 => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects a namespace: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh5 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh5 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                    _ => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects unexpected nodes: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh6 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh6 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                        let ref mut fresh7 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh7 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                }
                                                            }
                                                            18 => {
                                                                current_block_64 =
                                                                    7360144896257856683;
                                                                match current_block_64 {
                                                                    4513888698325685980 => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects an attribute: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh4 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh4 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                    7360144896257856683 => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects a namespace: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh5 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh5 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                    _ => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects unexpected nodes: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh6 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh6 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                        let ref mut fresh7 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh7 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                }
                                                            }
                                                            10 | 11 | 12 | 14 | 15 | 16 | 17
                                                            | 19 | 20 => {
                                                                current_block_64 =
                                                                    15147678989343080559;
                                                                match current_block_64 {
                                                                    4513888698325685980 => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects an attribute: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh4 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh4 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                    7360144896257856683 => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects a namespace: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh5 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh5 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                    _ => {
                                                                        xmlXIncludeErr(
                                                                            ctxt,
                                                                            (*ref_0).elem,
                                                                            XML_XINCLUDE_XPTR_RESULT as ::core::ffi::c_int,
                                                                            b"XPointer selects unexpected nodes: #%s\n\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                            fragment,
                                                                        );
                                                                        let ref mut fresh6 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh6 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                        let ref mut fresh7 =
                                                                            *(*set)
                                                                                .nodeTab
                                                                                .offset(i as isize);
                                                                        *fresh7 =
                                                                            ::core::ptr::null_mut::<
                                                                                xmlNode,
                                                                            >(
                                                                            );
                                                                    }
                                                                }
                                                            }
                                                            1 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 13
                                                            | _ => {}
                                                        }
                                                    }
                                                    i += 1;
                                                }
                                            }
                                            (*ref_0).inc = xmlXIncludeCopyXPointer(ctxt, xptr);
                                            xmlXPathFreeObject(xptr);
                                            xmlXPathFreeContext(xptrctxt);
                                            current_block = 16791665189521845338;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        17083819973035331397 => {}
                        _ => {
                            if !doc.is_null()
                                && !URL.is_null()
                                && (*ctxt).parseFlags & XML_PARSE_NOBASEFIX as ::core::ffi::c_int
                                    == 0
                                && (*doc).parseFlags & XML_PARSE_NOBASEFIX as ::core::ffi::c_int
                                    == 0
                            {
                                let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                                let mut base: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                let mut curBase: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                base = xmlGetNsProp(
                                    (*ref_0).elem as *const xmlNode,
                                    b"base\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut xmlChar,
                                    XML_XML_NAMESPACE,
                                );
                                if base.is_null() {
                                    curBase = xmlBuildRelativeURI(URL, (*ctxt).base);
                                    if curBase.is_null() {
                                        xmlXIncludeErr(
                                            ctxt,
                                            (*ref_0).elem,
                                            XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
                                            b"trying to build relative URI from %s\n\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            URL,
                                        );
                                    } else if xmlStrchr(curBase, '/' as i32 as xmlChar).is_null() {
                                        xmlFree.expect("non-null function pointer")(
                                            curBase as *mut ::core::ffi::c_void,
                                        );
                                    } else {
                                        base = curBase;
                                    }
                                }
                                if !base.is_null() {
                                    node = (*ref_0).inc;
                                    while !node.is_null() {
                                        if (*node).type_0 as ::core::ffi::c_uint
                                            == XML_ELEMENT_NODE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                        {
                                            curBase =
                                                xmlNodeGetBase((*node).doc, node as *const xmlNode);
                                            if curBase.is_null() {
                                                xmlNodeSetBase(node, base);
                                            } else {
                                                if xmlStrEqual(curBase, (*(*node).doc).URL) != 0 {
                                                    xmlNodeSetBase(node, base);
                                                } else {
                                                    let mut xmlBase: *mut xmlChar =
                                                        ::core::ptr::null_mut::<xmlChar>();
                                                    xmlBase = xmlGetNsProp(
                                                        node as *const xmlNode,
                                                        b"base\0" as *const u8
                                                            as *const ::core::ffi::c_char
                                                            as *mut xmlChar,
                                                        XML_XML_NAMESPACE,
                                                    );
                                                    if !xmlBase.is_null() {
                                                        let mut relBase: *mut xmlChar =
                                                            ::core::ptr::null_mut::<xmlChar>();
                                                        relBase = xmlBuildURI(xmlBase, base);
                                                        if relBase.is_null() {
                                                            xmlXIncludeErr(
                                                                ctxt,
                                                                (*ref_0).elem,
                                                                XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
                                                                b"trying to rebuild base from %s\n\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                xmlBase,
                                                            );
                                                        } else {
                                                            xmlNodeSetBase(node, relBase);
                                                            xmlFree.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                relBase as *mut ::core::ffi::c_void
                                                            );
                                                        }
                                                        xmlFree.expect("non-null function pointer")(
                                                            xmlBase as *mut ::core::ffi::c_void,
                                                        );
                                                    }
                                                }
                                                xmlFree.expect("non-null function pointer")(
                                                    curBase as *mut ::core::ffi::c_void,
                                                );
                                            }
                                        }
                                        node = (*node).next as xmlNodePtr;
                                    }
                                    xmlFree.expect("non-null function pointer")(
                                        base as *mut ::core::ffi::c_void,
                                    );
                                }
                            }
                            ret = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    xmlFree.expect("non-null function pointer")(URL as *mut ::core::ffi::c_void);
    xmlFree.expect("non-null function pointer")(fragment as *mut ::core::ffi::c_void);
    return ret;
}
unsafe extern "C" fn xmlXIncludeLoadTxt(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut url: *const xmlChar,
    mut ref_0: xmlXIncludeRefPtr,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    let mut URL: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut encoding: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut pctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut len: ::core::ffi::c_int = 0;
    let mut content: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if xmlStrcmp(
        url,
        b"-\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
    ) == 0 as ::core::ffi::c_int
    {
        url = b"./-\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar;
    }
    uri = xmlParseURI(url as *const ::core::ffi::c_char);
    if uri.is_null() {
        xmlXIncludeErr(
            ctxt,
            (*ref_0).elem,
            XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
            b"invalid value URI %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            url,
        );
    } else if !(*uri).fragment.is_null() {
        xmlXIncludeErr(
            ctxt,
            (*ref_0).elem,
            XML_XINCLUDE_TEXT_FRAGMENT as ::core::ffi::c_int,
            b"fragment identifier forbidden for text: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*uri).fragment as *const xmlChar,
        );
    } else {
        URL = xmlSaveUri(uri);
        if URL.is_null() {
            xmlXIncludeErr(
                ctxt,
                (*ref_0).elem,
                XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
                b"invalid value URI %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                url,
            );
        } else if *URL.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            xmlXIncludeErr(
                ctxt,
                (*ref_0).elem,
                XML_XINCLUDE_TEXT_DOCUMENT as ::core::ffi::c_int,
                b"text serialization of document not available\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                ::core::ptr::null::<xmlChar>(),
            );
        } else {
            i = 0 as ::core::ffi::c_int;
            loop {
                if !(i < (*ctxt).txtNr) {
                    current_block = 13797916685926291137;
                    break;
                }
                if xmlStrEqual(URL, (*(*ctxt).txtTab.offset(i as isize)).url) != 0 {
                    node = xmlNewDocText(
                        (*ctxt).doc as *const xmlDoc,
                        (*(*ctxt).txtTab.offset(i as isize)).text,
                    );
                    current_block = 451306527461536364;
                    break;
                } else {
                    i += 1;
                }
            }
            match current_block {
                13797916685926291137 => {
                    if !(*ref_0).elem.is_null() {
                        encoding =
                            xmlGetProp((*ref_0).elem as *const xmlNode, XINCLUDE_PARSE_ENCODING);
                    }
                    if !encoding.is_null() {
                        enc = xmlParseCharEncoding(encoding as *const ::core::ffi::c_char);
                        if enc as ::core::ffi::c_int
                            == XML_CHAR_ENCODING_ERROR as ::core::ffi::c_int
                        {
                            xmlXIncludeErr(
                                ctxt,
                                (*ref_0).elem,
                                XML_XINCLUDE_UNKNOWN_ENCODING as ::core::ffi::c_int,
                                b"encoding %s not supported\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                encoding,
                            );
                            current_block = 4246642513628410975;
                        } else {
                            current_block = 4068382217303356765;
                        }
                    } else {
                        current_block = 4068382217303356765;
                    }
                    match current_block {
                        4246642513628410975 => {}
                        _ => {
                            pctxt = xmlNewParserCtxt();
                            inputStream = xmlLoadExternalEntity(
                                URL as *const ::core::ffi::c_char,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                pctxt,
                            );
                            if inputStream.is_null() {
                                current_block = 4246642513628410975;
                            } else {
                                buf = (*inputStream).buf;
                                if buf.is_null() {
                                    current_block = 4246642513628410975;
                                } else {
                                    if !(*buf).encoder.is_null() {
                                        xmlCharEncCloseFunc(
                                            (*buf).encoder as *mut xmlCharEncodingHandler,
                                        );
                                    }
                                    (*buf).encoder = xmlGetCharEncodingHandler(enc);
                                    node = xmlNewDocText(
                                        (*ctxt).doc as *const xmlDoc,
                                        ::core::ptr::null::<xmlChar>(),
                                    );
                                    if node.is_null() {
                                        xmlXIncludeErrMemory(
                                            ctxt,
                                            (*ref_0).elem,
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                        );
                                        current_block = 4246642513628410975;
                                    } else {
                                        while xmlParserInputBufferRead(
                                            buf,
                                            4096 as ::core::ffi::c_int,
                                        ) > 0 as ::core::ffi::c_int
                                        {
                                        }
                                        content = xmlBufContent((*buf).buffer as *const xmlBuf);
                                        len = xmlBufLength((*buf).buffer) as ::core::ffi::c_int;
                                        i = 0 as ::core::ffi::c_int;
                                        loop {
                                            if !(i < len) {
                                                current_block = 14447253356787937536;
                                                break;
                                            }
                                            let mut cur: ::core::ffi::c_int = 0;
                                            let mut l: ::core::ffi::c_int = 0;
                                            l = len - i;
                                            cur = xmlGetUTF8Char(
                                                content.offset(i as isize)
                                                    as *const ::core::ffi::c_uchar,
                                                &raw mut l,
                                            );
                                            if cur < 0 as ::core::ffi::c_int
                                                || (if cur < 0x100 as ::core::ffi::c_int {
                                                    (0x9 as ::core::ffi::c_int <= cur
                                                        && cur <= 0xa as ::core::ffi::c_int
                                                        || cur == 0xd as ::core::ffi::c_int
                                                        || 0x20 as ::core::ffi::c_int <= cur)
                                                        as ::core::ffi::c_int
                                                } else {
                                                    (0x100 as ::core::ffi::c_int <= cur
                                                        && cur <= 0xd7ff as ::core::ffi::c_int
                                                        || 0xe000 as ::core::ffi::c_int <= cur
                                                            && cur <= 0xfffd as ::core::ffi::c_int
                                                        || 0x10000 as ::core::ffi::c_int <= cur
                                                            && cur
                                                                <= 0x10ffff as ::core::ffi::c_int)
                                                        as ::core::ffi::c_int
                                                }) == 0
                                            {
                                                xmlXIncludeErr(
                                                    ctxt,
                                                    (*ref_0).elem,
                                                    XML_XINCLUDE_INVALID_CHAR as ::core::ffi::c_int,
                                                    b"%s contains invalid char\n\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    URL,
                                                );
                                                current_block = 4246642513628410975;
                                                break;
                                            } else {
                                                i += l;
                                            }
                                        }
                                        match current_block {
                                            4246642513628410975 => {}
                                            _ => {
                                                xmlNodeAddContentLen(node, content, len);
                                                if (*ctxt).txtNr >= (*ctxt).txtMax {
                                                    let mut tmp: *mut xmlXIncludeTxt =
                                                        ::core::ptr::null_mut::<xmlXIncludeTxt>();
                                                    let mut newSize: size_t =
                                                        (if (*ctxt).txtMax != 0 {
                                                            (*ctxt).txtMax * 2 as ::core::ffi::c_int
                                                        } else {
                                                            8 as ::core::ffi::c_int
                                                        })
                                                            as size_t;
                                                    tmp = xmlRealloc
                                                        .expect("non-null function pointer")(
                                                        (*ctxt).txtTab as *mut ::core::ffi::c_void,
                                                        (::core::mem::size_of::<xmlXIncludeTxt>()
                                                            as size_t)
                                                            .wrapping_mul(newSize),
                                                    )
                                                        as *mut xmlXIncludeTxt;
                                                    if tmp.is_null() {
                                                        xmlXIncludeErrMemory(
                                                            ctxt,
                                                            (*ref_0).elem,
                                                            b"growing XInclude text table\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        current_block = 4246642513628410975;
                                                    } else {
                                                        (*ctxt).txtMax =
                                                            newSize as ::core::ffi::c_int;
                                                        (*ctxt).txtTab = tmp;
                                                        current_block = 11777552016271000781;
                                                    }
                                                } else {
                                                    current_block = 11777552016271000781;
                                                }
                                                match current_block {
                                                    4246642513628410975 => {}
                                                    _ => {
                                                        let ref mut fresh1 = (*(*ctxt)
                                                            .txtTab
                                                            .offset((*ctxt).txtNr as isize))
                                                        .text;
                                                        *fresh1 = xmlStrdup((*node).content);
                                                        let ref mut fresh2 = (*(*ctxt)
                                                            .txtTab
                                                            .offset((*ctxt).txtNr as isize))
                                                        .url;
                                                        *fresh2 = xmlStrdup(URL);
                                                        (*ctxt).txtNr += 1;
                                                        current_block = 451306527461536364;
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
            match current_block {
                4246642513628410975 => {}
                _ => {
                    (*ref_0).inc = node;
                    node = ::core::ptr::null_mut::<xmlNode>();
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    xmlFreeNode(node);
    xmlFreeInputStream(inputStream);
    xmlFreeParserCtxt(pctxt);
    xmlFree.expect("non-null function pointer")(encoding as *mut ::core::ffi::c_void);
    xmlFreeURI(uri);
    xmlFree.expect("non-null function pointer")(URL as *mut ::core::ffi::c_void);
    return ret;
}
unsafe extern "C" fn xmlXIncludeLoadFallback(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut fallback: xmlNodePtr,
    mut ref_0: xmlXIncludeRefPtr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut oldNbErrors: ::core::ffi::c_int = 0;
    if fallback.is_null()
        || (*fallback).type_0 as ::core::ffi::c_uint
            == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
        || ctxt.is_null()
    {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*fallback).children.is_null() {
        oldNbErrors = (*ctxt).nbErrors;
        (*ref_0).inc = xmlXIncludeCopyNode(ctxt, fallback, 1 as ::core::ffi::c_int);
        if (*ctxt).nbErrors > oldNbErrors {
            ret = -(1 as ::core::ffi::c_int);
        } else if (*ref_0).inc.is_null() {
            (*ref_0).emptyFb = 1 as ::core::ffi::c_int;
        }
    } else {
        (*ref_0).inc = ::core::ptr::null_mut::<xmlNode>();
        (*ref_0).emptyFb = 1 as ::core::ffi::c_int;
    }
    (*ref_0).fallback = 1 as ::core::ffi::c_int;
    return ret;
}
unsafe extern "C" fn xmlXIncludeExpandNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlXIncludeRefPtr {
    let mut ref_0: xmlXIncludeRefPtr = ::core::ptr::null_mut::<xmlXIncludeRef>();
    let mut i: ::core::ffi::c_int = 0;
    if (*ctxt).fatalErr != 0 {
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    if (*ctxt).depth >= XINCLUDE_MAX_DEPTH {
        xmlXIncludeErr(
            ctxt,
            node,
            XML_XINCLUDE_RECURSION as ::core::ffi::c_int,
            b"maximum recursion depth exceeded\n\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<xmlChar>(),
        );
        (*ctxt).fatalErr = 1 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*ctxt).incNr {
        if (**(*ctxt).incTab.offset(i as isize)).elem == node {
            if (**(*ctxt).incTab.offset(i as isize)).expanding != 0 {
                xmlXIncludeErr(
                    ctxt,
                    node,
                    XML_XINCLUDE_RECURSION as ::core::ffi::c_int,
                    b"inclusion loop detected\n\0" as *const u8 as *const ::core::ffi::c_char,
                    ::core::ptr::null::<xmlChar>(),
                );
                return ::core::ptr::null_mut::<xmlXIncludeRef>();
            }
            return *(*ctxt).incTab.offset(i as isize);
        }
        i += 1;
    }
    ref_0 = xmlXIncludeAddNode(ctxt, node);
    if ref_0.is_null() {
        return ::core::ptr::null_mut::<xmlXIncludeRef>();
    }
    (*ref_0).expanding = 1 as ::core::ffi::c_int;
    (*ctxt).depth += 1;
    xmlXIncludeLoadNode(ctxt, ref_0);
    (*ctxt).depth -= 1;
    (*ref_0).expanding = 0 as ::core::ffi::c_int;
    return ref_0;
}
unsafe extern "C" fn xmlXIncludeLoadNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut ref_0: xmlXIncludeRefPtr,
) -> ::core::ffi::c_int {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut href: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut parse: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut base: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut oldBase: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut URI: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut xml: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    if ctxt.is_null() || ref_0.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    cur = (*ref_0).elem;
    if cur.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    href = xmlXIncludeGetProp(ctxt, cur, XINCLUDE_HREF);
    if href.is_null() {
        href = xmlStrdup(b"\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
        if href.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    }
    parse = xmlXIncludeGetProp(ctxt, cur, XINCLUDE_PARSE);
    if !parse.is_null() {
        if xmlStrEqual(parse, XINCLUDE_PARSE_XML) != 0 {
            xml = 1 as ::core::ffi::c_int;
        } else if xmlStrEqual(parse, XINCLUDE_PARSE_TEXT) != 0 {
            xml = 0 as ::core::ffi::c_int;
        } else {
            xmlXIncludeErr(
                ctxt,
                cur,
                XML_XINCLUDE_PARSE_VALUE as ::core::ffi::c_int,
                b"invalid value %s for 'parse'\n\0" as *const u8 as *const ::core::ffi::c_char,
                parse,
            );
            if !href.is_null() {
                xmlFree.expect("non-null function pointer")(href as *mut ::core::ffi::c_void);
            }
            if !parse.is_null() {
                xmlFree.expect("non-null function pointer")(parse as *mut ::core::ffi::c_void);
            }
            return -(1 as ::core::ffi::c_int);
        }
    }
    base = xmlNodeGetBase((*ctxt).doc as *const xmlDoc, cur as *const xmlNode);
    if base.is_null() {
        URI = xmlBuildURI(href, (*(*ctxt).doc).URL);
    } else {
        URI = xmlBuildURI(href, base);
    }
    if URI.is_null() {
        let mut escbase: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut eschref: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        escbase = xmlURIEscape(base);
        eschref = xmlURIEscape(href);
        URI = xmlBuildURI(eschref, escbase);
        if !escbase.is_null() {
            xmlFree.expect("non-null function pointer")(escbase as *mut ::core::ffi::c_void);
        }
        if !eschref.is_null() {
            xmlFree.expect("non-null function pointer")(eschref as *mut ::core::ffi::c_void);
        }
    }
    if URI.is_null() {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_HREF_URI as ::core::ffi::c_int,
            b"failed build URL\n\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<xmlChar>(),
        );
        if !parse.is_null() {
            xmlFree.expect("non-null function pointer")(parse as *mut ::core::ffi::c_void);
        }
        if !href.is_null() {
            xmlFree.expect("non-null function pointer")(href as *mut ::core::ffi::c_void);
        }
        if !base.is_null() {
            xmlFree.expect("non-null function pointer")(base as *mut ::core::ffi::c_void);
        }
        return -(1 as ::core::ffi::c_int);
    }
    oldBase = (*ctxt).base;
    (*ctxt).base = base;
    if xml != 0 {
        ret = xmlXIncludeLoadDoc(ctxt, URI, ref_0);
    } else {
        ret = xmlXIncludeLoadTxt(ctxt, URI, ref_0);
    }
    (*ctxt).base = oldBase;
    if ret < 0 as ::core::ffi::c_int {
        let mut children: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        children = (*cur).children as xmlNodePtr;
        while !children.is_null() {
            if (*children).type_0 as ::core::ffi::c_uint
                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                && !(*children).ns.is_null()
                && xmlStrEqual((*children).name, XINCLUDE_FALLBACK) != 0
                && (xmlStrEqual((*(*children).ns).href, XINCLUDE_NS) != 0
                    || xmlStrEqual((*(*children).ns).href, XINCLUDE_OLD_NS) != 0)
            {
                ret = xmlXIncludeLoadFallback(ctxt, children, ref_0);
                break;
            } else {
                children = (*children).next as xmlNodePtr;
            }
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        xmlXIncludeErr(
            ctxt,
            cur,
            XML_XINCLUDE_NO_FALLBACK as ::core::ffi::c_int,
            b"could not load %s, and no fallback was found\n\0" as *const u8
                as *const ::core::ffi::c_char,
            URI,
        );
    }
    if !URI.is_null() {
        xmlFree.expect("non-null function pointer")(URI as *mut ::core::ffi::c_void);
    }
    if !parse.is_null() {
        xmlFree.expect("non-null function pointer")(parse as *mut ::core::ffi::c_void);
    }
    if !href.is_null() {
        xmlFree.expect("non-null function pointer")(href as *mut ::core::ffi::c_void);
    }
    if !base.is_null() {
        xmlFree.expect("non-null function pointer")(base as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlXIncludeIncludeNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut ref_0: xmlXIncludeRefPtr,
) -> ::core::ffi::c_int {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut end: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut list: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ctxt.is_null() || ref_0.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    cur = (*ref_0).elem;
    if cur.is_null()
        || (*cur).type_0 as ::core::ffi::c_uint
            == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    list = (*ref_0).inc;
    (*ref_0).inc = ::core::ptr::null_mut::<xmlNode>();
    (*ref_0).emptyFb = 0 as ::core::ffi::c_int;
    if !(*cur).parent.is_null()
        && (*(*cur).parent).type_0 as ::core::ffi::c_uint
            != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut nb_elem: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        tmp = list;
        while !tmp.is_null() {
            if (*tmp).type_0 as ::core::ffi::c_uint
                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                nb_elem += 1;
            }
            tmp = (*tmp).next as xmlNodePtr;
        }
        if nb_elem > 1 as ::core::ffi::c_int {
            xmlXIncludeErr(
                ctxt,
                (*ref_0).elem,
                XML_XINCLUDE_MULTIPLE_ROOT as ::core::ffi::c_int,
                b"XInclude error: would result in multiple root nodes\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                ::core::ptr::null::<xmlChar>(),
            );
            xmlFreeNodeList(list);
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*ctxt).parseFlags & XML_PARSE_NOXINCNODE as ::core::ffi::c_int != 0 {
        while !list.is_null() {
            end = list;
            list = (*list).next as xmlNodePtr;
            xmlAddPrevSibling(cur, end);
        }
        xmlUnlinkNode(cur);
        xmlFreeNode(cur);
    } else {
        let mut child: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        let mut next: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        if (*ref_0).fallback != 0 {
            xmlUnsetProp(
                cur,
                b"href\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
        }
        (*cur).type_0 = XML_XINCLUDE_START;
        child = (*cur).children as xmlNodePtr;
        while !child.is_null() {
            next = (*child).next as xmlNodePtr;
            xmlUnlinkNode(child);
            xmlFreeNode(child);
            child = next;
        }
        end = xmlNewDocNode(
            (*cur).doc as xmlDocPtr,
            (*cur).ns as xmlNsPtr,
            (*cur).name,
            ::core::ptr::null::<xmlChar>(),
        );
        if end.is_null() {
            xmlXIncludeErr(
                ctxt,
                (*ref_0).elem,
                XML_XINCLUDE_BUILD_FAILED as ::core::ffi::c_int,
                b"failed to build node\n\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<xmlChar>(),
            );
            xmlFreeNodeList(list);
            return -(1 as ::core::ffi::c_int);
        }
        (*end).type_0 = XML_XINCLUDE_END;
        xmlAddNextSibling(cur, end);
        while !list.is_null() {
            cur = list;
            list = (*list).next as xmlNodePtr;
            xmlAddPrevSibling(end, cur);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlXIncludeTestNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
) -> ::core::ffi::c_int {
    if node.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*node).ns.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if xmlStrEqual((*(*node).ns).href, XINCLUDE_NS) != 0
        || xmlStrEqual((*(*node).ns).href, XINCLUDE_OLD_NS) != 0
    {
        if xmlStrEqual((*(*node).ns).href, XINCLUDE_OLD_NS) != 0 {
            if (*ctxt).legacy == 0 as ::core::ffi::c_int {
                (*ctxt).legacy = 1 as ::core::ffi::c_int;
            }
        }
        if xmlStrEqual((*node).name, XINCLUDE_NODE) != 0 {
            let mut child: xmlNodePtr = (*node).children as xmlNodePtr;
            let mut nb_fallback: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while !child.is_null() {
                if (*child).type_0 as ::core::ffi::c_uint
                    == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !(*child).ns.is_null()
                    && (xmlStrEqual((*(*child).ns).href, XINCLUDE_NS) != 0
                        || xmlStrEqual((*(*child).ns).href, XINCLUDE_OLD_NS) != 0)
                {
                    if xmlStrEqual((*child).name, XINCLUDE_NODE) != 0 {
                        xmlXIncludeErr(
                            ctxt,
                            node,
                            XML_XINCLUDE_INCLUDE_IN_INCLUDE as ::core::ffi::c_int,
                            b"%s has an 'include' child\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            XINCLUDE_NODE,
                        );
                        return 0 as ::core::ffi::c_int;
                    }
                    if xmlStrEqual((*child).name, XINCLUDE_FALLBACK) != 0 {
                        nb_fallback += 1;
                    }
                }
                child = (*child).next as xmlNodePtr;
            }
            if nb_fallback > 1 as ::core::ffi::c_int {
                xmlXIncludeErr(
                    ctxt,
                    node,
                    XML_XINCLUDE_FALLBACKS_IN_INCLUDE as ::core::ffi::c_int,
                    b"%s has multiple fallback children\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    XINCLUDE_NODE,
                );
                return 0 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
        if xmlStrEqual((*node).name, XINCLUDE_FALLBACK) != 0 {
            if (*node).parent.is_null()
                || (*(*node).parent).type_0 as ::core::ffi::c_uint
                    != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(*node).parent).ns.is_null()
                || xmlStrEqual((*(*(*node).parent).ns).href, XINCLUDE_NS) == 0
                    && xmlStrEqual((*(*(*node).parent).ns).href, XINCLUDE_OLD_NS) == 0
                || xmlStrEqual((*(*node).parent).name, XINCLUDE_NODE) == 0
            {
                xmlXIncludeErr(
                    ctxt,
                    node,
                    XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE as ::core::ffi::c_int,
                    b"%s is not the child of an 'include'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    XINCLUDE_FALLBACK,
                );
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlXIncludeDoProcess(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut tree: xmlNodePtr,
) -> ::core::ffi::c_int {
    let mut ref_0: xmlXIncludeRefPtr = ::core::ptr::null_mut::<xmlXIncludeRef>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut start: ::core::ffi::c_int = 0;
    if tree.is_null()
        || (*tree).type_0 as ::core::ffi::c_uint
            == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    start = (*ctxt).incNr;
    cur = tree;
    let mut current_block_13: u64;
    loop {
        if xmlXIncludeTestNode(ctxt, cur) == 1 as ::core::ffi::c_int {
            ref_0 = xmlXIncludeExpandNode(ctxt, cur);
            if !ref_0.is_null() {
                (*ref_0).replace = 1 as ::core::ffi::c_int;
            }
            current_block_13 = 5399440093318478209;
        } else if !(*cur).children.is_null()
            && ((*cur).type_0 as ::core::ffi::c_uint
                == XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cur).type_0 as ::core::ffi::c_uint
                    == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            cur = (*cur).children as xmlNodePtr;
            current_block_13 = 6873731126896040597;
        } else {
            current_block_13 = 5399440093318478209;
        }
        match current_block_13 {
            5399440093318478209 => {
                while !(cur == tree) {
                    if !(*cur).next.is_null() {
                        cur = (*cur).next as xmlNodePtr;
                        break;
                    } else {
                        cur = (*cur).parent as xmlNodePtr;
                        if cur.is_null() {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        if !(!cur.is_null() && cur != tree) {
            break;
        }
    }
    i = start;
    while i < (*ctxt).incNr {
        if (**(*ctxt).incTab.offset(i as isize)).replace != 0 as ::core::ffi::c_int {
            if !(**(*ctxt).incTab.offset(i as isize)).inc.is_null()
                || (**(*ctxt).incTab.offset(i as isize)).emptyFb != 0 as ::core::ffi::c_int
            {
                xmlXIncludeIncludeNode(ctxt, *(*ctxt).incTab.offset(i as isize));
            }
            (**(*ctxt).incTab.offset(i as isize)).replace = 0 as ::core::ffi::c_int;
        } else if !(**(*ctxt).incTab.offset(i as isize)).inc.is_null() {
            xmlFreeNodeList((**(*ctxt).incTab.offset(i as isize)).inc);
            let ref mut fresh0 = (**(*ctxt).incTab.offset(i as isize)).inc;
            *fresh0 = ::core::ptr::null_mut::<xmlNode>();
        }
        ret += 1;
        i += 1;
    }
    if (*ctxt).isStream != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < (*ctxt).incNr {
            xmlXIncludeFreeRef(*(*ctxt).incTab.offset(i as isize));
            i += 1;
        }
        (*ctxt).incNr = 0 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeSetFlags(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*ctxt).parseFlags = flags;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeSetStreamingMode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*ctxt).isStream = (mode != 0) as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTreeFlagsData(
    mut tree: xmlNodePtr,
    mut flags: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlXIncludeCtxtPtr = ::core::ptr::null_mut::<xmlXIncludeCtxt>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if tree.is_null()
        || (*tree).type_0 as ::core::ffi::c_uint
            == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*tree).doc.is_null()
    {
        return -(1 as ::core::ffi::c_int);
    }
    ctxt = xmlXIncludeNewContext((*tree).doc as xmlDocPtr);
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*ctxt)._private = data;
    (*ctxt).base = xmlStrdup((*(*tree).doc).URL as *mut xmlChar);
    xmlXIncludeSetFlags(ctxt, flags);
    ret = xmlXIncludeDoProcess(ctxt, tree);
    if ret >= 0 as ::core::ffi::c_int && (*ctxt).nbErrors > 0 as ::core::ffi::c_int {
        ret = -(1 as ::core::ffi::c_int);
    }
    xmlXIncludeFreeContext(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessFlagsData(
    mut doc: xmlDocPtr,
    mut flags: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut tree: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if doc.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    tree = xmlDocGetRootElement(doc as *const xmlDoc);
    if tree.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return xmlXIncludeProcessTreeFlagsData(tree, flags, data);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessFlags(
    mut doc: xmlDocPtr,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return xmlXIncludeProcessFlagsData(doc, flags, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcess(mut doc: xmlDocPtr) -> ::core::ffi::c_int {
    return xmlXIncludeProcessFlags(doc, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTreeFlags(
    mut tree: xmlNodePtr,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlXIncludeCtxtPtr = ::core::ptr::null_mut::<xmlXIncludeCtxt>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if tree.is_null()
        || (*tree).type_0 as ::core::ffi::c_uint
            == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*tree).doc.is_null()
    {
        return -(1 as ::core::ffi::c_int);
    }
    ctxt = xmlXIncludeNewContext((*tree).doc as xmlDocPtr);
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*ctxt).base = xmlNodeGetBase((*tree).doc, tree as *const xmlNode);
    xmlXIncludeSetFlags(ctxt, flags);
    ret = xmlXIncludeDoProcess(ctxt, tree);
    if ret >= 0 as ::core::ffi::c_int && (*ctxt).nbErrors > 0 as ::core::ffi::c_int {
        ret = -(1 as ::core::ffi::c_int);
    }
    xmlXIncludeFreeContext(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessTree(mut tree: xmlNodePtr) -> ::core::ffi::c_int {
    return xmlXIncludeProcessTreeFlags(tree, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXIncludeProcessNode(
    mut ctxt: xmlXIncludeCtxtPtr,
    mut node: xmlNodePtr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if node.is_null()
        || (*node).type_0 as ::core::ffi::c_uint
            == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*node).doc.is_null()
        || ctxt.is_null()
    {
        return -(1 as ::core::ffi::c_int);
    }
    ret = xmlXIncludeDoProcess(ctxt, node);
    if ret >= 0 as ::core::ffi::c_int && (*ctxt).nbErrors > 0 as ::core::ffi::c_int {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
pub const XINCLUDE_NS: *const xmlChar = b"http://www.w3.org/2003/XInclude\0" as *const u8
    as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_OLD_NS: *const xmlChar = b"http://www.w3.org/2001/XInclude\0" as *const u8
    as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_NODE: *const xmlChar =
    b"include\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_FALLBACK: *const xmlChar =
    b"fallback\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_HREF: *const xmlChar =
    b"href\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_PARSE: *const xmlChar =
    b"parse\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_PARSE_XML: *const xmlChar =
    b"xml\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_PARSE_TEXT: *const xmlChar =
    b"text\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_PARSE_ENCODING: *const xmlChar =
    b"encoding\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_PARSE_XPOINTER: *const xmlChar =
    b"xpointer\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
