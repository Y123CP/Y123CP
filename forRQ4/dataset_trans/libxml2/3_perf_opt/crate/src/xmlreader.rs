use core::ffi::*;
use crate::src::globals::__xmlGenericError;
use crate::src::globals::__xmlGenericErrorContext;
use crate::src::buf::xmlBufContent;
use crate::src::buf::xmlBufCreateSize;
use crate::src::buf::xmlBufEmpty;
use crate::src::buf::xmlBufFree;
use crate::src::buf::xmlBufSetAllocationScheme;
use crate::src::buf::xmlBufShrink;
use crate::src::buf::xmlBufUse;
use crate::src::tree::xmlBufferCat;
use crate::src::tree::xmlBufferCreate;
use crate::src::tree::xmlBufferFree;
use crate::src::tree::xmlBufferSetAllocationScheme;
use crate::src::uri::xmlCanonicPath;
use crate::src::dict::xmlDictCreate;
use crate::src::dict::xmlDictFree;
use crate::src::dict::xmlDictLookup;
use crate::src::dict::xmlDictOwns;
use crate::src::dict::xmlDictQLookup;
use crate::src::encoding::xmlFindCharEncodingHandler;
use crate::src::valid::xmlFreeIDTable;
use crate::src::pattern::xmlFreePattern;
use crate::src::valid::xmlFreeRefTable;
use crate::src::xmlIO::xmlParserGetDirectory;
use crate::src::pattern::xmlPatterncompile;
use crate::src::relaxng::xmlRelaxNGFree;
use crate::src::relaxng::xmlRelaxNGFreeParserCtxt;
use crate::src::relaxng::xmlRelaxNGFreeValidCtxt;
use crate::src::relaxng::xmlRelaxNGNewParserCtxt;
use crate::src::relaxng::xmlRelaxNGNewValidCtxt;
use crate::src::relaxng::xmlRelaxNGParse;
use crate::src::relaxng::xmlRelaxNGSetParserErrors;
use crate::src::relaxng::xmlRelaxNGSetValidErrors;
use crate::src::relaxng::xmlRelaxNGSetValidStructuredErrors;
use crate::src::relaxng::xmlRelaxNGValidatePushCData;
use crate::src::xmlschemas::xmlSchemaFree;
use crate::src::xmlschemas::xmlSchemaFreeParserCtxt;
use crate::src::xmlschemas::xmlSchemaFreeValidCtxt;
use crate::src::xmlschemas::xmlSchemaIsValid;
use crate::src::xmlschemas::xmlSchemaNewParserCtxt;
use crate::src::xmlschemas::xmlSchemaNewValidCtxt;
use crate::src::xmlschemas::xmlSchemaParse;
use crate::src::xmlschemas::xmlSchemaSAXUnplug;
use crate::src::xmlschemas::xmlSchemaSetParserErrors;
use crate::src::xmlschemas::xmlSchemaSetValidErrors;
use crate::src::xmlschemas::xmlSchemaSetValidStructuredErrors;
use crate::src::xmlschemas::xmlSchemaValidateSetLocator;
use crate::src::tree::xmlSplitQName2;
use crate::src::xmlstring::xmlStrEqual;
use crate::src::xmlstring::xmlStrcat;
use crate::src::xmlstring::xmlStrdup;
use crate::src::xinclude::xmlXIncludeFreeContext;
use crate::src::xinclude::xmlXIncludeSetFlags;
use crate::src::xinclude::xmlXIncludeSetStreamingMode;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::xmlschemas::_xmlSchemaSAXPlug;
pub use crate::src::xinclude::_xmlXIncludeCtxt;
pub use crate::src::relaxng::_xmlRelaxNG;
pub use crate::src::relaxng::_xmlRelaxNGParserCtxt;
pub use crate::src::relaxng::_xmlRelaxNGValidCtxt;
pub use crate::src::xmlschemas::_xmlSchema;
pub use crate::src::xmlschemas::_xmlSchemaParserCtxt;
pub use crate::src::xmlschemas::_xmlSchemaValidCtxt;
pub use crate::src::pattern::_xmlPattern;
pub use crate::src::valid::_xmlValidState;
pub use crate::src::dict::_xmlDict;
pub use crate::src::hash::_xmlHashTable;
pub use crate::src::buf::_xmlBuf;
pub use crate::src::parser::_xmlAttrHashBucket;
pub use crate::src::parser::_xmlParserNsData;
pub use crate::src::parser::_xmlStartTag;
pub use crate::src::xmlregexp::_xmlAutomataState;
pub use crate::src::xmlregexp::_xmlAutomata;
extern "C" {
    fn vsnprintf(
        __s: *mut c_char,
        __maxlen: size_t,
        __format: *const c_char,
        __arg: ::core::ffi::VaList,
    ) -> c_int;
    fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc;
    fn xmlFreeDtd(cur: xmlDtdPtr);
    fn xmlFreeNs(cur: xmlNsPtr);
    fn xmlFreeNsList(cur: xmlNsPtr);
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlCopyDtd(dtd: xmlDtdPtr) -> xmlDtdPtr;
    fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar) -> xmlNodePtr;
    fn xmlDocCopyNode(
        node: xmlNodePtr,
        doc: xmlDocPtr,
        recursive: c_int,
    ) -> xmlNodePtr;
    fn xmlGetLineNo(node: *const xmlNode) -> c_long;
    fn xmlIsBlankNode(node: *const xmlNode) -> c_int;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr, nameSpace: *const xmlChar) -> xmlNsPtr;
    fn xmlGetNoNsProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlGetNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlNodeListGetString(
        doc: xmlDocPtr,
        list: *const xmlNode,
        inLine: c_int,
    ) -> *mut xmlChar;
    fn xmlBufGetNodeContent(buf: xmlBufPtr, cur: *const xmlNode) -> c_int;
    fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeGetSpacePreserve(cur: *const xmlNode) -> c_int;
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeDump(
        buf: xmlBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        level: c_int,
        format: c_int,
    ) -> c_int;
    fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFilename(
        URI: *const c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFd(
        fd: c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateMem(
        mem: *const c_char,
        size: c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferRead(
        in_0: xmlParserInputBufferPtr,
        len: c_int,
    ) -> c_int;
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    fn xmlStopParser(ctxt: xmlParserCtxtPtr);
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlCreatePushParserCtxt(
        sax: xmlSAXHandlerPtr,
        user_data: *mut c_void,
        chunk: *const c_char,
        size: c_int,
        filename: *const c_char,
    ) -> xmlParserCtxtPtr;
    fn xmlParseChunk(
        ctxt: xmlParserCtxtPtr,
        chunk: *const c_char,
        size: c_int,
        terminate: c_int,
    ) -> c_int;
    fn xmlByteConsumed(ctxt: xmlParserCtxtPtr) -> c_long;
    fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options: c_int)
        -> c_int;
    fn xmlCtxtSetMaxAmplification(ctxt: xmlParserCtxtPtr, maxAmpl: c_uint);
    fn xmlSAXVersion(hdlr: *mut xmlSAXHandler, version: c_int) -> c_int;
    fn xmlValidatePushElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        qname: *const xmlChar,
    ) -> c_int;
    fn xmlValidatePushCData(
        ctxt: xmlValidCtxtPtr,
        data: *const xmlChar,
        len: c_int,
    ) -> c_int;
    fn xmlValidatePopElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        qname: *const xmlChar,
    ) -> c_int;
    fn xmlRelaxNGValidatePushElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int;
    fn xmlRelaxNGValidatePopElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int;
    fn xmlRelaxNGValidateFullElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int;
    fn xmlSchemaSAXPlug(
        ctxt: xmlSchemaValidCtxtPtr,
        sax: *mut xmlSAXHandlerPtr,
        user_data: *mut *mut c_void,
    ) -> xmlSchemaSAXPlugPtr;
    fn xmlXIncludeNewContext(doc: xmlDocPtr) -> xmlXIncludeCtxtPtr;
    fn xmlXIncludeProcessNode(ctxt: xmlXIncludeCtxtPtr, tree: xmlNodePtr) -> c_int;
    fn xmlSwitchToEncoding(
        ctxt: xmlParserCtxtPtr,
        handler: xmlCharEncodingHandlerPtr,
    ) -> c_int;
    fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr) -> c_int;
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr) -> c_int;
    fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> c_int;
}

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: c_int,
    pub replaceEntities: c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: c_int,
    pub html: c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: c_int,
    pub inputMax: c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: c_int,
    pub nodeMax: c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: c_int,
    pub hasExternalSubset: c_int,
    pub hasPErefs: c_int,
    pub external: c_int,
    pub valid: c_int,
    pub validate: c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: c_int,
    pub directory: *mut c_char,
    pub name: *const xmlChar,
    pub nameNr: c_int,
    pub nameMax: c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: c_long,
    pub checkIndex: c_long,
    pub keepBlanks: c_int,
    pub disableSAX: c_int,
    pub inSubset: c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut c_int,
    pub spaceNr: c_int,
    pub spaceMax: c_int,
    pub spaceTab: *mut c_int,
    pub depth: c_int,
    pub entity: xmlParserInputPtr,
    pub charset: c_int,
    pub nodelen: c_int,
    pub nodemem: c_int,
    pub pedantic: c_int,
    pub _private: *mut c_void,
    pub loadsubset: c_int,
    pub linenumbers: c_int,
    pub catalogs: *mut c_void,
    pub recovery: c_int,
    pub progressive: c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: c_int,
    pub docdict: c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: c_int,
    pub nsNr: c_int,
    pub nsMax: c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut c_uint,
    pub pushTab: *mut xmlStartTag,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: c_int,
    pub options: c_int,
    pub dictNames: c_int,
    pub freeElemsNr: c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: c_ulong,
    pub sizeentities: c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: c_int,
    pub nodeInfoMax: c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: c_int,
    pub sizeentcopy: c_ulong,
    pub endCheckState: c_int,
    pub nbErrors: c_ushort,
    pub nbWarnings: c_ushort,
    pub maxAmpl: c_uint,
    pub nsdb: *mut xmlParserNsData,
    pub attrHashMax: c_uint,
    pub attrHash: *mut xmlAttrHashBucket,
}
pub type xmlAttrHashBucket = _xmlAttrHashBucket;
pub type xmlParserNsData = _xmlParserNsData;
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: c_ulong,
    pub begin_line: c_ulong,
    pub end_pos: c_ulong,
    pub end_line: c_ulong,
}

pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlStartTag = _xmlStartTag;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;

pub type xmlValidCtxt = _xmlValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: c_int,
    pub nodeMax: c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub flags: c_uint,
    pub doc: xmlDocPtr,
    pub valid: c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: c_int,
    pub vstateMax: c_int,
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

pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: c_ulong,
    pub length: c_ulong,
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
    pub initialized: c_uint,
    pub _private: *mut c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}

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

pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;

pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type xmlDeregisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;

pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlIDTable = _xmlHashTable;
pub type xmlIDTablePtr = *mut xmlIDTable;
pub type xmlRefTable = _xmlHashTable;
pub type xmlRefTablePtr = *mut xmlRefTable;
pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_htdd24ee73 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_htdd24ee73 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_htdd24ee73 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_htdd24ee73 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_htdd24ee73 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_htdd24ee73 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_htdd24ee73 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_htdd24ee73 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_htdd24ee73 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_htdd24ee73 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_htdd24ee73 = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed_htdd24ee73 = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_htdd24ee73 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_htdd24ee73 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_htdd24ee73 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_htdd24ee73 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_htdd24ee73 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_htdd24ee73 = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_htdd24ee73 = 1;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;

pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;

pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;

pub type xmlSchemaSAXPlugStruct = _xmlSchemaSAXPlug;
pub type xmlSchemaSAXPlugPtr = *mut xmlSchemaSAXPlugStruct;
pub type xmlParserSeverities = c_uint;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
pub const XML_TEXTREADER_MODE_READING: C2RustUnnamed_htdd24ee73 = 5;
pub const XML_TEXTREADER_MODE_CLOSED: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_TEXTREADER_MODE_EOF: C2RustUnnamed_htdd24ee73 = 3;
pub const XML_TEXTREADER_MODE_ERROR: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_TEXTREADER_MODE_INTERACTIVE: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_TEXTREADER_MODE_INITIAL: C2RustUnnamed_htdd24ee73 = 0;
pub type xmlParserProperties = c_uint;
pub const XML_PARSER_SUBST_ENTITIES: xmlParserProperties = 4;
pub const XML_PARSER_VALIDATE: xmlParserProperties = 3;
pub const XML_PARSER_DEFAULTATTRS: xmlParserProperties = 2;
pub const XML_PARSER_LOADDTD: xmlParserProperties = 1;
pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed_htdd24ee73 = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed_htdd24ee73 = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed_htdd24ee73 = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed_htdd24ee73 = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed_htdd24ee73 = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed_htdd24ee73 = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed_htdd24ee73 = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed_htdd24ee73 = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed_htdd24ee73 = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed_htdd24ee73 = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed_htdd24ee73 = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed_htdd24ee73 = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed_htdd24ee73 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextReader {
    pub mode: c_int,
    pub doc: xmlDocPtr,
    pub validate: xmlTextReaderValidate,
    pub allocs: c_int,
    pub state: xmlTextReaderState,
    pub ctxt: xmlParserCtxtPtr,
    pub sax: xmlSAXHandlerPtr,
    pub input: xmlParserInputBufferPtr,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub characters: charactersSAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub base: c_uint,
    pub cur: c_uint,
    pub node: xmlNodePtr,
    pub curnode: xmlNodePtr,
    pub depth: c_int,
    pub faketext: xmlNodePtr,
    pub preserve: c_int,
    pub buffer: xmlBufPtr,
    pub dict: xmlDictPtr,
    pub ent: xmlNodePtr,
    pub entNr: c_int,
    pub entMax: c_int,
    pub entTab: *mut xmlNodePtr,
    pub errorFunc: xmlTextReaderErrorFunc,
    pub errorFuncArg: *mut c_void,
    pub rngSchemas: xmlRelaxNGPtr,
    pub rngValidCtxt: xmlRelaxNGValidCtxtPtr,
    pub rngPreserveCtxt: c_int,
    pub rngValidErrors: c_int,
    pub rngFullNode: xmlNodePtr,
    pub xsdSchemas: xmlSchemaPtr,
    pub xsdValidCtxt: xmlSchemaValidCtxtPtr,
    pub xsdPreserveCtxt: c_int,
    pub xsdValidErrors: c_int,
    pub xsdPlug: xmlSchemaSAXPlugPtr,
    pub xinclude: c_int,
    pub xinclude_name: *const xmlChar,
    pub xincctxt: xmlXIncludeCtxtPtr,
    pub in_xinclude: c_int,
    pub patternNr: c_int,
    pub patternMax: c_int,
    pub patternTab: *mut xmlPatternPtr,
    pub preserves: c_int,
    pub parserFlags: c_int,
    pub sErrorFunc: xmlStructuredErrorFunc,
}
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlPattern = _xmlPattern;
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
pub type xmlTextReaderErrorFunc = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        xmlParserSeverities,
        xmlTextReaderLocatorPtr,
    ) -> (),
>;
pub type xmlTextReaderLocatorPtr = *mut c_void;
pub type xmlTextReaderState = c_int;
pub const XML_TEXTREADER_ERROR: xmlTextReaderState = 6;
pub const XML_TEXTREADER_DONE: xmlTextReaderState = 5;
pub const XML_TEXTREADER_BACKTRACK: xmlTextReaderState = 4;
pub const XML_TEXTREADER_EMPTY: xmlTextReaderState = 3;
pub const XML_TEXTREADER_END: xmlTextReaderState = 2;
pub const XML_TEXTREADER_ELEMENT: xmlTextReaderState = 1;
pub const XML_TEXTREADER_START: xmlTextReaderState = 0;
pub const XML_TEXTREADER_NONE: xmlTextReaderState = -1;
pub type xmlTextReaderValidate = c_uint;
pub const XML_TEXTREADER_VALIDATE_XSD: xmlTextReaderValidate = 4;
pub const XML_TEXTREADER_VALIDATE_RNG: xmlTextReaderValidate = 2;
pub const XML_TEXTREADER_VALIDATE_DTD: xmlTextReaderValidate = 1;
pub const XML_TEXTREADER_NOT_VALIDATE: xmlTextReaderValidate = 0;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;

pub const MAX_ERR_MSG_SIZE: c_int = 64000 as c_int;
pub const MAX_FREE_NODES: c_int = 100 as c_int;
pub const CHUNK_SIZE: c_int = 512 as c_int;
pub const XML_TEXTREADER_INPUT: c_int = 1 as c_int;
pub const XML_TEXTREADER_CTXT: c_int = 2 as c_int;
pub const NODE_IS_EMPTY: c_int = 0x1 as c_int;
pub const NODE_IS_PRESERVED: c_int = 0x2 as c_int;
pub const NODE_IS_SPRESERVED: c_int = 0x4 as c_int;
fn xmlTextReaderFreeProp(mut reader: xmlTextReaderPtr, mut cur: xmlAttrPtr) { unsafe {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if !reader.is_null() && !(*reader).ctxt.is_null() {
        dict = (*(*reader).ctxt).dict;
    } else {
        dict = ::core::ptr::null_mut::<xmlDict>();
    }
    if cur.is_null() {
        return;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    if !(*cur).children.is_null() {
        xmlTextReaderFreeNodeList(reader, (*cur).children as xmlNodePtr);
    }
    if !(*cur).name.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).name as *mut c_char as *mut c_void,
        );
    }
    if !reader.is_null()
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).freeAttrsNr < MAX_FREE_NODES
    {
        (*cur).next = (*(*reader).ctxt).freeAttrs as *mut _xmlAttr;
        (*(*reader).ctxt).freeAttrs = cur;
        (*(*reader).ctxt).freeAttrsNr += 1;
    } else {
        xmlFree.expect("non-null function pointer")(cur as *mut c_void);
    };
} }
fn xmlTextReaderFreePropList(mut reader: xmlTextReaderPtr, mut cur: xmlAttrPtr) { unsafe {
    let mut next: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    while !cur.is_null() {
        next = (*cur).next as xmlAttrPtr;
        xmlTextReaderFreeProp(reader, cur);
        cur = next;
    }
} }
fn xmlTextReaderFreeNodeList(mut reader: xmlTextReaderPtr, mut cur: xmlNodePtr) { unsafe {
    let mut next: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    let mut depth: size_t = 0 as size_t;
    if !reader.is_null() && !(*reader).ctxt.is_null() {
        dict = (*(*reader).ctxt).dict;
    } else {
        dict = ::core::ptr::null_mut::<xmlDict>();
    }
    if cur.is_null() {
        return;
    }
    if (*cur).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        xmlFreeNsList(cur as xmlNsPtr);
        return;
    }
    if (*cur).type_0 as c_uint
        == XML_DOCUMENT_NODE as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        xmlFreeDoc(cur as xmlDocPtr);
        return;
    }
    loop {
        while (*cur).type_0 as c_uint
            != XML_DTD_NODE as c_int as c_uint
            && (*cur).type_0 as c_uint
                != XML_ENTITY_REF_NODE as c_int as c_uint
            && !(*cur).children.is_null()
            && (*(*cur).children).parent == cur
        {
            cur = (*cur).children as xmlNodePtr;
            depth = (depth as c_ulong).wrapping_add(1 as c_ulong)
                as size_t as size_t;
        }
        next = (*cur).next as xmlNodePtr;
        parent = (*cur).parent as xmlNodePtr;
        if (*cur).type_0 as c_uint
            != XML_DTD_NODE as c_int as c_uint
        {
            if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
                (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
            }
            if ((*cur).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
                || (*cur).type_0 as c_uint
                    == XML_XINCLUDE_START as c_int as c_uint
                || (*cur).type_0 as c_uint
                    == XML_XINCLUDE_END as c_int as c_uint)
                && !(*cur).properties.is_null()
            {
                xmlTextReaderFreePropList(reader, (*cur).properties as xmlAttrPtr);
            }
            if (*cur).content != &raw mut (*cur).properties as *mut xmlChar
                && (*cur).type_0 as c_uint
                    != XML_ELEMENT_NODE as c_int as c_uint
                && (*cur).type_0 as c_uint
                    != XML_XINCLUDE_START as c_int as c_uint
                && (*cur).type_0 as c_uint
                    != XML_XINCLUDE_END as c_int as c_uint
                && (*cur).type_0 as c_uint
                    != XML_ENTITY_REF_NODE as c_int as c_uint
            {
                if !(*cur).content.is_null()
                    && (dict.is_null()
                        || xmlDictOwns(dict, (*cur).content as *const xmlChar)
                            == 0 as c_int)
                {
                    xmlFree.expect("non-null function pointer")(
                        (*cur).content as *mut c_char as *mut c_void,
                    );
                }
            }
            if ((*cur).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
                || (*cur).type_0 as c_uint
                    == XML_XINCLUDE_START as c_int as c_uint
                || (*cur).type_0 as c_uint
                    == XML_XINCLUDE_END as c_int as c_uint)
                && !(*cur).nsDef.is_null()
            {
                xmlFreeNsList((*cur).nsDef as xmlNsPtr);
            }
            if (*cur).type_0 as c_uint
                != XML_TEXT_NODE as c_int as c_uint
                && (*cur).type_0 as c_uint
                    != XML_COMMENT_NODE as c_int as c_uint
            {
                if !(*cur).name.is_null()
                    && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as c_int)
                {
                    xmlFree.expect("non-null function pointer")(
                        (*cur).name as *mut c_char as *mut c_void,
                    );
                }
            }
            if ((*cur).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
                || (*cur).type_0 as c_uint
                    == XML_TEXT_NODE as c_int as c_uint)
                && !reader.is_null()
                && !(*reader).ctxt.is_null()
                && (*(*reader).ctxt).freeElemsNr < MAX_FREE_NODES
            {
                (*cur).next = (*(*reader).ctxt).freeElems as *mut _xmlNode;
                (*(*reader).ctxt).freeElems = cur;
                (*(*reader).ctxt).freeElemsNr += 1;
            } else {
                xmlFree.expect("non-null function pointer")(cur as *mut c_void);
            }
        }
        if !next.is_null() {
            cur = next;
        } else {
            if depth == 0 as size_t || parent.is_null() {
                break;
            }
            depth = (depth as c_ulong).wrapping_sub(1 as c_ulong)
                as size_t as size_t;
            cur = parent;
            (*cur).children = ::core::ptr::null_mut::<_xmlNode>();
        }
    }
} }
fn xmlTextReaderFreeNode(mut reader: xmlTextReaderPtr, mut cur: xmlNodePtr) { unsafe {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if !reader.is_null() && !(*reader).ctxt.is_null() {
        dict = (*(*reader).ctxt).dict;
    } else {
        dict = ::core::ptr::null_mut::<xmlDict>();
    }
    if (*cur).type_0 as c_uint
        == XML_DTD_NODE as c_int as c_uint
    {
        xmlFreeDtd(cur as xmlDtdPtr);
        return;
    }
    if (*cur).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        xmlFreeNs(cur as xmlNsPtr);
        return;
    }
    if (*cur).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        xmlTextReaderFreeProp(reader, cur as xmlAttrPtr);
        return;
    }
    if !(*cur).children.is_null()
        && (*cur).type_0 as c_uint
            != XML_ENTITY_REF_NODE as c_int as c_uint
    {
        if (*(*cur).children).parent == cur {
            xmlTextReaderFreeNodeList(reader, (*cur).children as xmlNodePtr);
        }
        (*cur).children = ::core::ptr::null_mut::<_xmlNode>();
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    if ((*cur).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_XINCLUDE_START as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_XINCLUDE_END as c_int as c_uint)
        && !(*cur).properties.is_null()
    {
        xmlTextReaderFreePropList(reader, (*cur).properties as xmlAttrPtr);
    }
    if (*cur).content != &raw mut (*cur).properties as *mut xmlChar
        && (*cur).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
        && (*cur).type_0 as c_uint
            != XML_XINCLUDE_START as c_int as c_uint
        && (*cur).type_0 as c_uint
            != XML_XINCLUDE_END as c_int as c_uint
        && (*cur).type_0 as c_uint
            != XML_ENTITY_REF_NODE as c_int as c_uint
    {
        if !(*cur).content.is_null()
            && (dict.is_null()
                || xmlDictOwns(dict, (*cur).content as *const xmlChar) == 0 as c_int)
        {
            xmlFree.expect("non-null function pointer")(
                (*cur).content as *mut c_char as *mut c_void,
            );
        }
    }
    if ((*cur).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_XINCLUDE_START as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_XINCLUDE_END as c_int as c_uint)
        && !(*cur).nsDef.is_null()
    {
        xmlFreeNsList((*cur).nsDef as xmlNsPtr);
    }
    if (*cur).type_0 as c_uint
        != XML_TEXT_NODE as c_int as c_uint
        && (*cur).type_0 as c_uint
            != XML_COMMENT_NODE as c_int as c_uint
    {
        if !(*cur).name.is_null()
            && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as c_int)
        {
            xmlFree.expect("non-null function pointer")(
                (*cur).name as *mut c_char as *mut c_void,
            );
        }
    }
    if ((*cur).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint)
        && !reader.is_null()
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).freeElemsNr < MAX_FREE_NODES
    {
        (*cur).next = (*(*reader).ctxt).freeElems as *mut _xmlNode;
        (*(*reader).ctxt).freeElems = cur;
        (*(*reader).ctxt).freeElemsNr += 1;
    } else {
        xmlFree.expect("non-null function pointer")(cur as *mut c_void);
    };
} }
fn xmlTextReaderFreeDoc(mut reader: xmlTextReaderPtr, mut cur: xmlDocPtr) { unsafe {
    let mut extSubset: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut intSubset: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    if cur.is_null() {
        return;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    if !(*cur).ids.is_null() {
        xmlFreeIDTable((*cur).ids as xmlIDTablePtr);
    }
    (*cur).ids = NULL;
    if !(*cur).refs.is_null() {
        xmlFreeRefTable((*cur).refs as xmlRefTablePtr);
    }
    (*cur).refs = NULL;
    extSubset = (*cur).extSubset as xmlDtdPtr;
    intSubset = (*cur).intSubset as xmlDtdPtr;
    if intSubset == extSubset {
        extSubset = ::core::ptr::null_mut::<xmlDtd>();
    }
    if !extSubset.is_null() {
        xmlUnlinkNode((*cur).extSubset as xmlNodePtr);
        (*cur).extSubset = ::core::ptr::null_mut::<_xmlDtd>();
        xmlFreeDtd(extSubset);
    }
    if !intSubset.is_null() {
        xmlUnlinkNode((*cur).intSubset as xmlNodePtr);
        (*cur).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
        xmlFreeDtd(intSubset);
    }
    if !(*cur).children.is_null() {
        xmlTextReaderFreeNodeList(reader, (*cur).children as xmlNodePtr);
    }
    if !(*cur).version.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*cur).version as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).name.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).name as *mut c_void);
    }
    if !(*cur).encoding.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*cur).encoding as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).oldNs.is_null() {
        xmlFreeNsList((*cur).oldNs as xmlNsPtr);
    }
    if !(*cur).URL.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*cur).URL as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).dict.is_null() {
        xmlDictFree((*cur).dict as xmlDictPtr);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
} }
fn xmlTextReaderEntPush(
    mut reader: xmlTextReaderPtr,
    mut value: xmlNodePtr,
) -> c_int { unsafe {
    if (*reader).entNr >= (*reader).entMax {
        let mut newSize: size_t = (if (*reader).entMax == 0 as c_int {
            10 as c_int
        } else {
            (*reader).entMax * 2 as c_int
        }) as size_t;
        let mut tmp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*reader).entTab as *mut c_void,
            newSize.wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if tmp.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*reader).entTab = tmp;
        (*reader).entMax = newSize as c_int;
    }
    let ref mut fresh2 = *(*reader).entTab.offset((*reader).entNr as isize);
    *fresh2 = value;
    (*reader).ent = value;
    let fresh3 = (*reader).entNr;
    (*reader).entNr = (*reader).entNr + 1;
    return fresh3;
} }
fn xmlTextReaderEntPop(mut reader: xmlTextReaderPtr) -> xmlNodePtr { unsafe {
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if (*reader).entNr <= 0 as c_int {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    (*reader).entNr -= 1;
    if (*reader).entNr > 0 as c_int {
        (*reader).ent = *(*reader)
            .entTab
            .offset(((*reader).entNr - 1 as c_int) as isize);
    } else {
        (*reader).ent = ::core::ptr::null_mut::<xmlNode>();
    }
    ret = *(*reader).entTab.offset((*reader).entNr as isize);
    let ref mut fresh1 = *(*reader).entTab.offset((*reader).entNr as isize);
    *fresh1 = ::core::ptr::null_mut::<xmlNode>();
    return ret;
} }
unsafe extern "C" fn xmlTextReaderStartElement(
    mut ctx: *mut c_void,
    mut fullname: *const xmlChar,
    mut atts: *mut *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).startElement.is_some() {
        (*reader).startElement.expect("non-null function pointer")(ctx, fullname, atts);
        if !(*ctxt).node.is_null()
            && !(*ctxt).input.is_null()
            && !(*(*ctxt).input).cur.is_null()
            && *(*(*ctxt).input)
                .cur
                .offset(0 as c_int as isize) as c_int
                == '/' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as c_int as isize) as c_int
                == '>' as i32
        {
            (*(*ctxt).node).extra = NODE_IS_EMPTY as c_ushort;
        }
    }
    if !reader.is_null() {
        (*reader).state = XML_TEXTREADER_ELEMENT;
    }
}
unsafe extern "C" fn xmlTextReaderEndElement(
    mut ctx: *mut c_void,
    mut fullname: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).endElement.is_some() {
        (*reader).endElement.expect("non-null function pointer")(ctx, fullname);
    }
}
unsafe extern "C" fn xmlTextReaderStartElementNs(
    mut ctx: *mut c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
    mut nb_namespaces: c_int,
    mut namespaces: *mut *const xmlChar,
    mut nb_attributes: c_int,
    mut nb_defaulted: c_int,
    mut attributes: *mut *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).startElementNs.is_some() {
        (*reader).startElementNs.expect("non-null function pointer")(
            ctx,
            localname,
            prefix,
            URI,
            nb_namespaces,
            namespaces,
            nb_attributes,
            nb_defaulted,
            attributes,
        );
        if !(*ctxt).node.is_null()
            && !(*ctxt).input.is_null()
            && !(*(*ctxt).input).cur.is_null()
            && *(*(*ctxt).input)
                .cur
                .offset(0 as c_int as isize) as c_int
                == '/' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as c_int as isize) as c_int
                == '>' as i32
        {
            (*(*ctxt).node).extra = NODE_IS_EMPTY as c_ushort;
        }
    }
    if !reader.is_null() {
        (*reader).state = XML_TEXTREADER_ELEMENT;
    }
}
unsafe extern "C" fn xmlTextReaderEndElementNs(
    mut ctx: *mut c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).endElementNs.is_some() {
        (*reader).endElementNs.expect("non-null function pointer")(ctx, localname, prefix, URI);
    }
}
unsafe extern "C" fn xmlTextReaderCharacters(
    mut ctx: *mut c_void,
    mut ch: *const xmlChar,
    mut len: c_int,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).characters.is_some() {
        (*reader).characters.expect("non-null function pointer")(ctx, ch, len);
    }
}
unsafe extern "C" fn xmlTextReaderCDataBlock(
    mut ctx: *mut c_void,
    mut ch: *const xmlChar,
    mut len: c_int,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).cdataBlock.is_some() {
        (*reader).cdataBlock.expect("non-null function pointer")(ctx, ch, len);
    }
}
fn xmlTextReaderPushData(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    let mut inbuf: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut val: c_int = 0;
    let mut s: c_int = 0;
    let mut oldstate: xmlTextReaderState = XML_TEXTREADER_START;
    if (*reader).input.is_null() || (*(*reader).input).buffer.is_null() {
        return -(1 as c_int);
    }
    oldstate = (*reader).state;
    (*reader).state = XML_TEXTREADER_NONE;
    inbuf = (*(*reader).input).buffer;
    while (*reader).state as c_int == XML_TEXTREADER_NONE as c_int {
        if xmlBufUse(inbuf)
            < (*reader)
                .cur
                .wrapping_add(CHUNK_SIZE as c_uint) as size_t
        {
            if !((*reader).mode != XML_TEXTREADER_MODE_EOF as c_int) {
                break;
            }
            val = xmlParserInputBufferRead((*reader).input, 4096 as c_int);
            if val == 0 as c_int {
                if xmlBufUse(inbuf) == (*reader).cur as size_t {
                    (*reader).mode = XML_TEXTREADER_MODE_EOF as c_int;
                    break;
                }
            } else if val < 0 as c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlParserInputBufferRead failed\n\0" as *const u8
                        as *const c_char,
                );
                (*reader).mode = XML_TEXTREADER_MODE_EOF as c_int;
                (*reader).state = oldstate;
                return val;
            }
        }
        if xmlBufUse(inbuf)
            >= (*reader)
                .cur
                .wrapping_add(CHUNK_SIZE as c_uint) as size_t
        {
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const c_char)
                    .offset((*reader).cur as isize),
                CHUNK_SIZE,
                0 as c_int,
            );
            (*reader).cur = (*reader)
                .cur
                .wrapping_add(CHUNK_SIZE as c_uint);
            if val != 0 as c_int {
                (*(*reader).ctxt).wellFormed = 0 as c_int;
            }
            if (*(*reader).ctxt).wellFormed == 0 as c_int {
                break;
            }
        } else {
            s = xmlBufUse(inbuf).wrapping_sub((*reader).cur as size_t) as c_int;
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const c_char)
                    .offset((*reader).cur as isize),
                s,
                0 as c_int,
            );
            (*reader).cur = (*reader).cur.wrapping_add(s as c_uint);
            if val != 0 as c_int {
                (*(*reader).ctxt).wellFormed = 0 as c_int;
            }
            break;
        }
    }
    (*reader).state = oldstate;
    if (*reader).mode == XML_TEXTREADER_MODE_INTERACTIVE as c_int {
        if (*(*reader).input).readcallback.is_some() {
            if (*reader).cur >= 4096 as c_uint
                && xmlBufUse(inbuf).wrapping_sub((*reader).cur as size_t) <= CHUNK_SIZE as size_t
            {
                val = xmlBufShrink(inbuf, (*reader).cur as size_t) as c_int;
                if val >= 0 as c_int {
                    (*reader).cur = (*reader).cur.wrapping_sub(val as c_uint);
                }
            }
        }
    } else if (*reader).mode == XML_TEXTREADER_MODE_EOF as c_int {
        if (*reader).state as c_int != XML_TEXTREADER_DONE as c_int {
            s = xmlBufUse(inbuf).wrapping_sub((*reader).cur as size_t) as c_int;
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const c_char)
                    .offset((*reader).cur as isize),
                s,
                1 as c_int,
            );
            (*reader).cur = xmlBufUse(inbuf) as c_uint;
            (*reader).state = XML_TEXTREADER_DONE;
            if val != 0 as c_int {
                if (*(*reader).ctxt).wellFormed != 0 {
                    (*(*reader).ctxt).wellFormed = 0 as c_int;
                } else {
                    return -(1 as c_int);
                }
            }
        }
    }
    if (*(*reader).ctxt).wellFormed == 0 as c_int {
        (*reader).mode = XML_TEXTREADER_MODE_EOF as c_int;
        return -(1 as c_int);
    }
    return 0 as c_int;
} }
fn xmlTextReaderValidatePush(mut reader: xmlTextReaderPtr) { unsafe {
    let mut node: xmlNodePtr = (*reader).node;
    if (*reader).validate as c_uint
        == XML_TEXTREADER_VALIDATE_DTD as c_int as c_uint
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).validate == 1 as c_int
    {
        if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
            (*(*reader).ctxt).valid &= xmlValidatePushElement(
                &raw mut (*(*reader).ctxt).vctxt,
                (*(*reader).ctxt).myDoc,
                node,
                (*node).name,
            );
        } else {
            let mut qname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            qname = xmlStrdup((*(*node).ns).prefix);
            qname = xmlStrcat(
                qname,
                b":\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            qname = xmlStrcat(qname, (*node).name);
            (*(*reader).ctxt).valid &= xmlValidatePushElement(
                &raw mut (*(*reader).ctxt).vctxt,
                (*(*reader).ctxt).myDoc,
                node,
                qname,
            );
            if !qname.is_null() {
                xmlFree.expect("non-null function pointer")(qname as *mut c_void);
            }
        }
    }
    if (*reader).validate as c_uint
        == XML_TEXTREADER_VALIDATE_RNG as c_int as c_uint
        && !(*reader).rngValidCtxt.is_null()
    {
        let mut ret: c_int = 0;
        if !(*reader).rngFullNode.is_null() {
            return;
        }
        ret = xmlRelaxNGValidatePushElement((*reader).rngValidCtxt, (*(*reader).ctxt).myDoc, node);
        if ret == 0 as c_int {
            node = xmlTextReaderExpand(reader);
            if node.is_null() {
                ret = -(1 as c_int);
            } else {
                ret = xmlRelaxNGValidateFullElement(
                    (*reader).rngValidCtxt,
                    (*(*reader).ctxt).myDoc,
                    node,
                );
                (*reader).rngFullNode = node;
            }
        }
        if ret != 1 as c_int {
            (*reader).rngValidErrors += 1;
        }
    }
} }
unsafe fn xmlTextReaderValidateCData(
    mut reader: xmlTextReaderPtr,
    mut data: *const xmlChar,
    mut len: c_int,
) {
    if (*reader).validate as c_uint
        == XML_TEXTREADER_VALIDATE_DTD as c_int as c_uint
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).validate == 1 as c_int
    {
        (*(*reader).ctxt).valid &=
            xmlValidatePushCData(&raw mut (*(*reader).ctxt).vctxt, data, len);
    }
    if (*reader).validate as c_uint
        == XML_TEXTREADER_VALIDATE_RNG as c_int as c_uint
        && !(*reader).rngValidCtxt.is_null()
    {
        let mut ret: c_int = 0;
        if !(*reader).rngFullNode.is_null() {
            return;
        }
        ret = xmlRelaxNGValidatePushCData((*reader).rngValidCtxt, data, len);
        if ret != 1 as c_int {
            (*reader).rngValidErrors += 1;
        }
    }
}
fn xmlTextReaderValidatePop(mut reader: xmlTextReaderPtr) { unsafe {
    let mut node: xmlNodePtr = (*reader).node;
    if (*reader).validate as c_uint
        == XML_TEXTREADER_VALIDATE_DTD as c_int as c_uint
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).validate == 1 as c_int
    {
        if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
            (*(*reader).ctxt).valid &= xmlValidatePopElement(
                &raw mut (*(*reader).ctxt).vctxt,
                (*(*reader).ctxt).myDoc,
                node,
                (*node).name,
            );
        } else {
            let mut qname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            qname = xmlStrdup((*(*node).ns).prefix);
            qname = xmlStrcat(
                qname,
                b":\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            qname = xmlStrcat(qname, (*node).name);
            (*(*reader).ctxt).valid &= xmlValidatePopElement(
                &raw mut (*(*reader).ctxt).vctxt,
                (*(*reader).ctxt).myDoc,
                node,
                qname,
            );
            if !qname.is_null() {
                xmlFree.expect("non-null function pointer")(qname as *mut c_void);
            }
        }
    }
    if (*reader).validate as c_uint
        == XML_TEXTREADER_VALIDATE_RNG as c_int as c_uint
        && !(*reader).rngValidCtxt.is_null()
    {
        let mut ret: c_int = 0;
        if !(*reader).rngFullNode.is_null() {
            if node == (*reader).rngFullNode {
                (*reader).rngFullNode = ::core::ptr::null_mut::<xmlNode>();
            }
            return;
        }
        ret = xmlRelaxNGValidatePopElement((*reader).rngValidCtxt, (*(*reader).ctxt).myDoc, node);
        if ret != 1 as c_int {
            (*reader).rngValidErrors += 1;
        }
    }
} }
fn xmlTextReaderValidateEntity(mut reader: xmlTextReaderPtr) { unsafe {
    let mut current_block: u64;
    let mut oldnode: xmlNodePtr = (*reader).node;
    let mut node: xmlNodePtr = (*reader).node;
    loop {
        if (*node).type_0 as c_uint
            == XML_ENTITY_REF_NODE as c_int as c_uint
        {
            if !(*node).children.is_null()
                && (*(*node).children).type_0 as c_uint
                    == XML_ENTITY_DECL as c_int as c_uint
                && !(*(*node).children).children.is_null()
            {
                if xmlTextReaderEntPush(reader, node) < 0 as c_int {
                    if node == oldnode {
                        break;
                    }
                    current_block = 13920728574496106596;
                } else {
                    node = (*(*node).children).children as xmlNodePtr;
                    current_block = 11174649648027449784;
                }
            } else {
                if node == oldnode {
                    break;
                }
                current_block = 13920728574496106596;
            }
        } else {
            if (*node).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
            {
                (*reader).node = node;
                xmlTextReaderValidatePush(reader);
            } else if (*node).type_0 as c_uint
                == XML_TEXT_NODE as c_int as c_uint
                || (*node).type_0 as c_uint
                    == XML_CDATA_SECTION_NODE as c_int as c_uint
            {
                xmlTextReaderValidateCData(reader, (*node).content, xmlStrlen((*node).content));
            }
            if !(*node).children.is_null() {
                node = (*node).children as xmlNodePtr;
                current_block = 11174649648027449784;
            } else {
                if (*node).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                {
                    xmlTextReaderValidatePop(reader);
                }
                current_block = 13920728574496106596;
            }
        }
        match current_block {
            13920728574496106596 => {
                if !(*node).next.is_null() {
                    node = (*node).next as xmlNodePtr;
                } else {
                    loop {
                        node = (*node).parent as xmlNodePtr;
                        if (*node).type_0 as c_uint
                            == XML_ELEMENT_NODE as c_int as c_uint
                        {
                            let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                            if (*reader).entNr == 0 as c_int {
                                loop {
                                    tmp = (*node).last as xmlNodePtr;
                                    if tmp.is_null() {
                                        break;
                                    }
                                    if !((*tmp).extra as c_int & NODE_IS_PRESERVED
                                        == 0 as c_int)
                                    {
                                        break;
                                    }
                                    xmlUnlinkNode(tmp);
                                    xmlTextReaderFreeNode(reader, tmp);
                                }
                            }
                            (*reader).node = node;
                            xmlTextReaderValidatePop(reader);
                        }
                        if (*node).type_0 as c_uint
                            == XML_ENTITY_DECL as c_int as c_uint
                            && !(*reader).ent.is_null()
                            && (*(*reader).ent).children == node
                        {
                            node = xmlTextReaderEntPop(reader);
                        }
                        if node == oldnode {
                            break;
                        }
                        if !(*node).next.is_null() {
                            node = (*node).next as xmlNodePtr;
                            break;
                        } else if !(!node.is_null() && node != oldnode) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        if !(!node.is_null() && node != oldnode) {
            break;
        }
    }
    (*reader).node = oldnode;
} }
fn xmlTextReaderGetSuccessor(mut cur: xmlNodePtr) -> xmlNodePtr { unsafe {
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*cur).next.is_null() {
        return (*cur).next as xmlNodePtr;
    }
    loop {
        cur = (*cur).parent as xmlNodePtr;
        if cur.is_null() {
            break;
        }
        if !(*cur).next.is_null() {
            return (*cur).next as xmlNodePtr;
        }
        if cur.is_null() {
            break;
        }
    }
    return cur;
} }
fn xmlTextReaderDoExpand(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    let mut val: c_int = 0;
    if reader.is_null() || (*reader).node.is_null() || (*reader).ctxt.is_null() {
        return -(1 as c_int);
    }
    loop {
        if (*(*reader).ctxt).instate as c_int == XML_PARSER_EOF as c_int {
            return 1 as c_int;
        }
        if !xmlTextReaderGetSuccessor((*reader).node).is_null() {
            return 1 as c_int;
        }
        if (*(*reader).ctxt).nodeNr < (*reader).depth {
            return 1 as c_int;
        }
        if (*reader).mode == XML_TEXTREADER_MODE_EOF as c_int {
            return 1 as c_int;
        }
        val = xmlTextReaderPushData(reader);
        if val < 0 as c_int {
            (*reader).mode = XML_TEXTREADER_MODE_ERROR as c_int;
            return -(1 as c_int);
        }
        if !((*reader).mode != XML_TEXTREADER_MODE_EOF as c_int) {
            break;
        }
    }
    return 1 as c_int;
} }
fn xmlTextReaderCollectSiblings(mut node: xmlNodePtr) -> *mut xmlChar { unsafe {
    let mut buffer: xmlBufferPtr = ::core::ptr::null_mut::<xmlBuffer>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buffer = xmlBufferCreate();
    if buffer.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    xmlBufferSetAllocationScheme(buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    while !node.is_null() {
        match (*node).type_0 as c_uint {
            3 | 4 => {
                xmlBufferCat(buffer, (*node).content);
            }
            1 => {
                let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                tmp = xmlTextReaderCollectSiblings((*node).children as xmlNodePtr);
                xmlBufferCat(buffer, tmp);
                xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
            }
            _ => {}
        }
        node = (*node).next as xmlNodePtr;
    }
    ret = (*buffer).content;
    (*buffer).content = ::core::ptr::null_mut::<xmlChar>();
    xmlBufferFree(buffer);
    return ret;
} }
#[inline]
pub fn xmlTextReaderRead(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    let mut current_block: u64;
    let mut val: c_int = 0;
    let mut olddepth: c_int = 0 as c_int;
    let mut oldstate: xmlTextReaderState = XML_TEXTREADER_START;
    let mut oldnode: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    if !(*reader).doc.is_null() {
        return xmlTextReaderReadTree(reader);
    }
    if (*reader).ctxt.is_null() {
        return -(1 as c_int);
    }
    if (*reader).mode == XML_TEXTREADER_MODE_INITIAL as c_int {
        (*reader).mode = XML_TEXTREADER_MODE_INTERACTIVE as c_int;
        loop {
            val = xmlTextReaderPushData(reader);
            if val < 0 as c_int {
                (*reader).mode = XML_TEXTREADER_MODE_ERROR as c_int;
                (*reader).state = XML_TEXTREADER_ERROR;
                return -(1 as c_int);
            }
            if !((*(*reader).ctxt).node.is_null()
                && ((*reader).mode != XML_TEXTREADER_MODE_EOF as c_int
                    && (*reader).state as c_int
                        != XML_TEXTREADER_DONE as c_int))
            {
                break;
            }
        }
        if (*(*reader).ctxt).node.is_null() {
            if !(*(*reader).ctxt).myDoc.is_null() {
                (*reader).node = (*(*(*reader).ctxt).myDoc).children as xmlNodePtr;
            }
            if (*reader).node.is_null() {
                (*reader).mode = XML_TEXTREADER_MODE_ERROR as c_int;
                (*reader).state = XML_TEXTREADER_ERROR;
                return -(1 as c_int);
            }
            (*reader).state = XML_TEXTREADER_ELEMENT;
        } else {
            if !(*(*reader).ctxt).myDoc.is_null() {
                (*reader).node = (*(*(*reader).ctxt).myDoc).children as xmlNodePtr;
            }
            if (*reader).node.is_null() {
                (*reader).node = *(*(*reader).ctxt)
                    .nodeTab
                    .offset(0 as c_int as isize);
            }
            (*reader).state = XML_TEXTREADER_ELEMENT;
        }
        (*reader).depth = 0 as c_int;
        (*(*reader).ctxt).parseMode = XML_PARSE_READER;
        current_block = 1022895542554905014;
    } else {
        oldstate = (*reader).state;
        olddepth = (*(*reader).ctxt).nodeNr;
        oldnode = (*reader).node;
        current_block = 14230955220427894555;
    }
    '_get_next_node: loop {
        match current_block {
            14230955220427894555 => {
                if (*reader).node.is_null() {
                    if (*reader).mode == XML_TEXTREADER_MODE_EOF as c_int {
                        return 0 as c_int;
                    } else {
                        return -(1 as c_int);
                    }
                }
                while !(*reader).node.is_null()
                    && (*(*reader).node).next.is_null()
                    && (*(*reader).ctxt).nodeNr == olddepth
                    && (oldstate as c_int
                        == XML_TEXTREADER_BACKTRACK as c_int
                        || (*(*reader).node).children.is_null()
                        || (*(*reader).node).type_0 as c_uint
                            == XML_ENTITY_REF_NODE as c_int as c_uint
                        || !(*(*reader).node).children.is_null()
                            && (*(*(*reader).node).children).type_0 as c_uint
                                == XML_TEXT_NODE as c_int as c_uint
                            && (*(*(*reader).node).children).next.is_null()
                        || (*(*reader).node).type_0 as c_uint
                            == XML_DTD_NODE as c_int as c_uint
                        || (*(*reader).node).type_0 as c_uint
                            == XML_DOCUMENT_NODE as c_int as c_uint
                        || (*(*reader).node).type_0 as c_uint
                            == XML_HTML_DOCUMENT_NODE as c_int as c_uint)
                    && ((*(*reader).ctxt).node.is_null()
                        || (*(*reader).ctxt).node == (*reader).node
                        || (*(*reader).ctxt).node == (*(*reader).node).parent)
                    && (*(*reader).ctxt).instate as c_int
                        != XML_PARSER_EOF as c_int
                {
                    val = xmlTextReaderPushData(reader);
                    if val < 0 as c_int {
                        (*reader).mode = XML_TEXTREADER_MODE_ERROR as c_int;
                        (*reader).state = XML_TEXTREADER_ERROR;
                        return -(1 as c_int);
                    }
                    if (*reader).node.is_null() {
                        break '_get_next_node;
                    }
                }
                if oldstate as c_int != XML_TEXTREADER_BACKTRACK as c_int
                {
                    if !(*(*reader).node).children.is_null()
                        && (*(*reader).node).type_0 as c_uint
                            != XML_ENTITY_REF_NODE as c_int as c_uint
                        && (*(*reader).node).type_0 as c_uint
                            != XML_XINCLUDE_START as c_int as c_uint
                        && (*(*reader).node).type_0 as c_uint
                            != XML_DTD_NODE as c_int as c_uint
                    {
                        (*reader).node = (*(*reader).node).children as xmlNodePtr;
                        (*reader).depth += 1;
                        (*reader).state = XML_TEXTREADER_ELEMENT;
                        current_block = 1022895542554905014;
                        continue;
                    }
                }
                if !(*(*reader).node).next.is_null() {
                    if oldstate as c_int
                        == XML_TEXTREADER_ELEMENT as c_int
                        && (*(*reader).node).type_0 as c_uint
                            == XML_ELEMENT_NODE as c_int as c_uint
                        && (*(*reader).node).children.is_null()
                        && (*(*reader).node).extra as c_int & NODE_IS_EMPTY
                            == 0 as c_int
                        && (*reader).in_xinclude <= 0 as c_int
                    {
                        (*reader).state = XML_TEXTREADER_END;
                        current_block = 1022895542554905014;
                    } else {
                        if (*reader).validate as c_uint != 0
                            && (*(*reader).node).type_0 as c_uint
                                == XML_ELEMENT_NODE as c_int as c_uint
                        {
                            xmlTextReaderValidatePop(reader);
                        }
                        if (*reader).preserves > 0 as c_int
                            && (*(*reader).node).extra as c_int & NODE_IS_SPRESERVED
                                != 0
                        {
                            (*reader).preserves -= 1;
                        }
                        (*reader).node = (*(*reader).node).next as xmlNodePtr;
                        (*reader).state = XML_TEXTREADER_ELEMENT;
                        if (*reader).preserves == 0 as c_int
                            && (*reader).in_xinclude == 0 as c_int
                            && (*reader).entNr == 0 as c_int
                            && !(*(*reader).node).prev.is_null()
                            && (*(*(*reader).node).prev).type_0 as c_uint
                                != XML_DTD_NODE as c_int as c_uint
                        {
                            let mut tmp: xmlNodePtr = (*(*reader).node).prev as xmlNodePtr;
                            if (*tmp).extra as c_int & NODE_IS_PRESERVED
                                == 0 as c_int
                            {
                                if oldnode == tmp {
                                    oldnode = ::core::ptr::null_mut::<xmlNode>();
                                }
                                xmlUnlinkNode(tmp);
                                xmlTextReaderFreeNode(reader, tmp);
                            }
                        }
                        current_block = 1022895542554905014;
                    }
                } else if oldstate as c_int
                    == XML_TEXTREADER_ELEMENT as c_int
                    && (*(*reader).node).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                    && (*(*reader).node).children.is_null()
                    && (*(*reader).node).extra as c_int & NODE_IS_EMPTY
                        == 0 as c_int
                {
                    (*reader).state = XML_TEXTREADER_END;
                    current_block = 1022895542554905014;
                } else {
                    if (*reader).validate as c_uint
                        != XML_TEXTREADER_NOT_VALIDATE as c_int as c_uint
                        && (*(*reader).node).type_0 as c_uint
                            == XML_ELEMENT_NODE as c_int as c_uint
                    {
                        xmlTextReaderValidatePop(reader);
                    }
                    if (*reader).preserves > 0 as c_int
                        && (*(*reader).node).extra as c_int & NODE_IS_SPRESERVED != 0
                    {
                        (*reader).preserves -= 1;
                    }
                    (*reader).node = (*(*reader).node).parent as xmlNodePtr;
                    if (*reader).node.is_null()
                        || (*(*reader).node).type_0 as c_uint
                            == XML_DOCUMENT_NODE as c_int as c_uint
                        || (*(*reader).node).type_0 as c_uint
                            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
                    {
                        if (*reader).mode != XML_TEXTREADER_MODE_EOF as c_int {
                            val = xmlParseChunk(
                                (*reader).ctxt,
                                b"\0" as *const u8 as *const c_char,
                                0 as c_int,
                                1 as c_int,
                            );
                            (*reader).state = XML_TEXTREADER_DONE;
                            if val != 0 as c_int {
                                return -(1 as c_int);
                            }
                        }
                        (*reader).node = ::core::ptr::null_mut::<xmlNode>();
                        (*reader).depth = -(1 as c_int);
                        if !oldnode.is_null()
                            && (*reader).preserves == 0 as c_int
                            && (*reader).in_xinclude == 0 as c_int
                            && (*reader).entNr == 0 as c_int
                            && (*oldnode).type_0 as c_uint
                                != XML_DTD_NODE as c_int as c_uint
                            && (*oldnode).extra as c_int & NODE_IS_PRESERVED
                                == 0 as c_int
                        {
                            xmlUnlinkNode(oldnode);
                            xmlTextReaderFreeNode(reader, oldnode);
                        }
                        break;
                    } else {
                        if (*reader).preserves == 0 as c_int
                            && (*reader).in_xinclude == 0 as c_int
                            && (*reader).entNr == 0 as c_int
                            && !(*(*reader).node).last.is_null()
                            && (*(*(*reader).node).last).extra as c_int
                                & NODE_IS_PRESERVED
                                == 0 as c_int
                        {
                            let mut tmp_0: xmlNodePtr = (*(*reader).node).last as xmlNodePtr;
                            xmlUnlinkNode(tmp_0);
                            xmlTextReaderFreeNode(reader, tmp_0);
                        }
                        (*reader).depth -= 1;
                        (*reader).state = XML_TEXTREADER_BACKTRACK;
                        current_block = 1022895542554905014;
                    }
                }
            }
            _ => {
                if !(*reader).node.is_null()
                    && (*(*reader).node).next.is_null()
                    && ((*(*reader).node).type_0 as c_uint
                        == XML_TEXT_NODE as c_int as c_uint
                        || (*(*reader).node).type_0 as c_uint
                            == XML_CDATA_SECTION_NODE as c_int as c_uint)
                {
                    if xmlTextReaderExpand(reader).is_null() {
                        return -(1 as c_int);
                    }
                }
                if (*reader).xinclude != 0
                    && (*reader).in_xinclude == 0 as c_int
                    && (*reader).state as c_int
                        != XML_TEXTREADER_BACKTRACK as c_int
                    && !(*reader).node.is_null()
                    && (*(*reader).node).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                    && !(*(*reader).node).ns.is_null()
                    && (xmlStrEqual((*(*(*reader).node).ns).href, XINCLUDE_NS) != 0
                        || xmlStrEqual((*(*(*reader).node).ns).href, XINCLUDE_OLD_NS) != 0)
                {
                    if (*reader).xincctxt.is_null() {
                        (*reader).xincctxt = xmlXIncludeNewContext((*(*reader).ctxt).myDoc);
                        xmlXIncludeSetFlags(
                            (*reader).xincctxt,
                            (*reader).parserFlags & !(XML_PARSE_NOXINCNODE as c_int),
                        );
                        xmlXIncludeSetStreamingMode((*reader).xincctxt, 1 as c_int);
                    }
                    if xmlTextReaderExpand(reader).is_null() {
                        return -(1 as c_int);
                    }
                    xmlXIncludeProcessNode((*reader).xincctxt, (*reader).node);
                }
                if !(*reader).node.is_null()
                    && (*(*reader).node).type_0 as c_uint
                        == XML_XINCLUDE_START as c_int as c_uint
                {
                    (*reader).in_xinclude += 1;
                    current_block = 14230955220427894555;
                } else if !(*reader).node.is_null()
                    && (*(*reader).node).type_0 as c_uint
                        == XML_XINCLUDE_END as c_int as c_uint
                {
                    (*reader).in_xinclude -= 1;
                    current_block = 14230955220427894555;
                } else {
                    if !(*reader).node.is_null()
                        && (*(*reader).node).type_0 as c_uint
                            == XML_ENTITY_REF_NODE as c_int as c_uint
                        && !(*reader).ctxt.is_null()
                        && (*(*reader).ctxt).replaceEntities == 1 as c_int
                    {
                        if !(*(*reader).node).children.is_null()
                            && (*(*(*reader).node).children).type_0 as c_uint
                                == XML_ENTITY_DECL as c_int as c_uint
                            && !(*(*(*reader).node).children).children.is_null()
                        {
                            if xmlTextReaderEntPush(reader, (*reader).node)
                                < 0 as c_int
                            {
                                current_block = 14230955220427894555;
                                continue;
                            }
                            (*reader).node = (*(*(*reader).node).children).children as xmlNodePtr;
                        }
                    } else if !(*reader).node.is_null()
                        && (*(*reader).node).type_0 as c_uint
                            == XML_ENTITY_REF_NODE as c_int as c_uint
                        && !(*reader).ctxt.is_null()
                        && (*reader).validate as c_uint != 0
                    {
                        xmlTextReaderValidateEntity(reader);
                    }
                    if !(*reader).node.is_null()
                        && (*(*reader).node).type_0 as c_uint
                            == XML_ENTITY_DECL as c_int as c_uint
                        && !(*reader).ent.is_null()
                        && (*(*reader).ent).children == (*reader).node
                    {
                        (*reader).node = xmlTextReaderEntPop(reader);
                        (*reader).depth += 1;
                        current_block = 14230955220427894555;
                    } else {
                        if (*reader).validate as c_uint
                            != XML_TEXTREADER_NOT_VALIDATE as c_int
                                as c_uint
                            && !(*reader).node.is_null()
                        {
                            let mut node: xmlNodePtr = (*reader).node;
                            if (*node).type_0 as c_uint
                                == XML_ELEMENT_NODE as c_int as c_uint
                                && ((*reader).state as c_int
                                    != XML_TEXTREADER_END as c_int
                                    && (*reader).state as c_int
                                        != XML_TEXTREADER_BACKTRACK as c_int)
                            {
                                xmlTextReaderValidatePush(reader);
                            } else if (*node).type_0 as c_uint
                                == XML_TEXT_NODE as c_int as c_uint
                                || (*node).type_0 as c_uint
                                    == XML_CDATA_SECTION_NODE as c_int
                                        as c_uint
                            {
                                xmlTextReaderValidateCData(
                                    reader,
                                    (*node).content,
                                    xmlStrlen((*node).content),
                                );
                            }
                        }
                        if (*reader).patternNr > 0 as c_int
                            && (*reader).state as c_int
                                != XML_TEXTREADER_END as c_int
                            && (*reader).state as c_int
                                != XML_TEXTREADER_BACKTRACK as c_int
                        {
                            let mut i: c_int = 0;
                            i = 0 as c_int;
                            while i < (*reader).patternNr {
                                if xmlPatternMatch(
                                    *(*reader).patternTab.offset(i as isize),
                                    (*reader).node,
                                ) == 1 as c_int
                                {
                                    xmlTextReaderPreserve(reader);
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                        }
                        if (*reader).validate as c_uint
                            == XML_TEXTREADER_VALIDATE_XSD as c_int
                                as c_uint
                            && (*reader).xsdValidErrors == 0 as c_int
                            && !(*reader).xsdValidCtxt.is_null()
                        {
                            (*reader).xsdValidErrors = (xmlSchemaIsValid((*reader).xsdValidCtxt)
                                == 0)
                                as c_int;
                        }
                        return 1 as c_int;
                    }
                }
            }
        }
    }
    (*reader).state = XML_TEXTREADER_DONE;
    return 0 as c_int;
} }
#[inline]
pub fn xmlTextReaderReadState(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    return (*reader).mode;
} }
#[inline]
pub fn xmlTextReaderExpand(mut reader: xmlTextReaderPtr) -> xmlNodePtr { unsafe {
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*reader).doc.is_null() {
        return (*reader).node;
    }
    if (*reader).ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if xmlTextReaderDoExpand(reader) < 0 as c_int {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    return (*reader).node;
} }
#[inline]
pub fn xmlTextReaderNext(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    let mut ret: c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if !(*reader).doc.is_null() {
        return xmlTextReaderNextTree(reader);
    }
    cur = (*reader).node;
    if cur.is_null()
        || (*cur).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return xmlTextReaderRead(reader);
    }
    if (*reader).state as c_int == XML_TEXTREADER_END as c_int
        || (*reader).state as c_int == XML_TEXTREADER_BACKTRACK as c_int
    {
        return xmlTextReaderRead(reader);
    }
    if (*cur).extra as c_int & NODE_IS_EMPTY != 0 {
        return xmlTextReaderRead(reader);
    }
    loop {
        ret = xmlTextReaderRead(reader);
        if ret != 1 as c_int {
            return ret;
        }
        if !((*reader).node != cur) {
            break;
        }
    }
    return xmlTextReaderRead(reader);
} }
#[inline]
pub fn xmlTextReaderReadInnerXml(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    let mut resbuf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut cur_node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut buff: xmlBufferPtr = ::core::ptr::null_mut::<xmlBuffer>();
    let mut buff2: xmlBufferPtr = ::core::ptr::null_mut::<xmlBuffer>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if xmlTextReaderExpand(reader).is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    doc = (*(*reader).node).doc as xmlDocPtr;
    buff = xmlBufferCreate();
    if buff.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    xmlBufferSetAllocationScheme(buff, XML_BUFFER_ALLOC_DOUBLEIT);
    cur_node = (*(*reader).node).children as xmlNodePtr;
    while !cur_node.is_null() {
        node = xmlDocCopyNode(cur_node, doc, 1 as c_int);
        buff2 = xmlBufferCreate();
        xmlBufferSetAllocationScheme(buff2, XML_BUFFER_ALLOC_DOUBLEIT);
        if xmlNodeDump(
            buff2,
            doc,
            node,
            0 as c_int,
            0 as c_int,
        ) == -(1 as c_int)
        {
            xmlFreeNode(node);
            xmlBufferFree(buff2);
            xmlBufferFree(buff);
            return ::core::ptr::null_mut::<xmlChar>();
        }
        xmlBufferCat(buff, (*buff2).content);
        xmlFreeNode(node);
        xmlBufferFree(buff2);
        cur_node = (*cur_node).next as xmlNodePtr;
    }
    resbuf = (*buff).content;
    (*buff).content = ::core::ptr::null_mut::<xmlChar>();
    xmlBufferFree(buff);
    return resbuf;
} }
#[inline]
pub fn xmlTextReaderReadOuterXml(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    let mut resbuf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut buff: xmlBufferPtr = ::core::ptr::null_mut::<xmlBuffer>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if xmlTextReaderExpand(reader).is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    node = (*reader).node;
    doc = (*node).doc as xmlDocPtr;
    if (*node).type_0 as c_uint
        == XML_DTD_NODE as c_int as c_uint
    {
        node = xmlCopyDtd(node as xmlDtdPtr) as xmlNodePtr;
    } else {
        node = xmlDocCopyNode(node, doc, 1 as c_int);
    }
    buff = xmlBufferCreate();
    xmlBufferSetAllocationScheme(buff, XML_BUFFER_ALLOC_DOUBLEIT);
    if xmlNodeDump(
        buff,
        doc,
        node,
        0 as c_int,
        0 as c_int,
    ) == -(1 as c_int)
    {
        xmlFreeNode(node);
        xmlBufferFree(buff);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    resbuf = (*buff).content;
    (*buff).content = ::core::ptr::null_mut::<xmlChar>();
    xmlFreeNode(node);
    xmlBufferFree(buff);
    return resbuf;
} }
#[inline]
pub fn xmlTextReaderReadString(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    node = if !(*reader).curnode.is_null() {
        (*reader).curnode
    } else {
        (*reader).node
    };
    match (*node).type_0 as c_uint {
        3 => {
            if !(*node).content.is_null() {
                return xmlStrdup((*node).content);
            }
        }
        1 => {
            if xmlTextReaderDoExpand(reader) != -(1 as c_int) {
                return xmlTextReaderCollectSiblings((*node).children as xmlNodePtr);
            }
        }
        2 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlreader.c\0"
                    as *const u8 as *const c_char,
                1677 as c_int,
            );
        }
        _ => {}
    }
    return ::core::ptr::null_mut::<xmlChar>();
} }
fn xmlTextReaderNextTree(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).state as c_int == XML_TEXTREADER_END as c_int {
        return 0 as c_int;
    }
    if (*reader).node.is_null() {
        if (*(*reader).doc).children.is_null() {
            (*reader).state = XML_TEXTREADER_END;
            return 0 as c_int;
        }
        (*reader).node = (*(*reader).doc).children as xmlNodePtr;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as c_int;
    }
    if (*reader).state as c_int != XML_TEXTREADER_BACKTRACK as c_int {
        if !(*(*reader).node).next.is_null() {
            (*reader).node = (*(*reader).node).next as xmlNodePtr;
            (*reader).state = XML_TEXTREADER_START;
            return 1 as c_int;
        }
        (*reader).state = XML_TEXTREADER_BACKTRACK;
        xmlTextReaderRead(reader);
    }
    if !(*(*reader).node).next.is_null() {
        (*reader).node = (*(*reader).node).next as xmlNodePtr;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as c_int;
    }
    if !(*(*reader).node).parent.is_null() {
        if (*(*(*reader).node).parent).type_0 as c_uint
            == XML_DOCUMENT_NODE as c_int as c_uint
        {
            (*reader).state = XML_TEXTREADER_END;
            return 0 as c_int;
        }
        (*reader).node = (*(*reader).node).parent as xmlNodePtr;
        (*reader).depth -= 1;
        (*reader).state = XML_TEXTREADER_BACKTRACK;
        xmlTextReaderNextTree(reader);
    }
    (*reader).state = XML_TEXTREADER_END;
    return 1 as c_int;
} }
fn xmlTextReaderReadTree(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    let mut current_block: u64;
    if (*reader).state as c_int == XML_TEXTREADER_END as c_int {
        return 0 as c_int;
    }
    loop {
        if (*reader).node.is_null() {
            if (*(*reader).doc).children.is_null() {
                (*reader).state = XML_TEXTREADER_END;
                return 0 as c_int;
            }
            (*reader).node = (*(*reader).doc).children as xmlNodePtr;
            (*reader).state = XML_TEXTREADER_START;
        } else {
            if (*reader).state as c_int
                != XML_TEXTREADER_BACKTRACK as c_int
                && (*(*reader).node).type_0 as c_uint
                    != XML_DTD_NODE as c_int as c_uint
                && (*(*reader).node).type_0 as c_uint
                    != XML_XINCLUDE_START as c_int as c_uint
                && (*(*reader).node).type_0 as c_uint
                    != XML_ENTITY_REF_NODE as c_int as c_uint
            {
                if !(*(*reader).node).children.is_null() {
                    (*reader).node = (*(*reader).node).children as xmlNodePtr;
                    (*reader).depth += 1;
                    (*reader).state = XML_TEXTREADER_START;
                    current_block = 66159954028994333;
                } else if (*(*reader).node).type_0 as c_uint
                    == XML_ATTRIBUTE_NODE as c_int as c_uint
                {
                    (*reader).state = XML_TEXTREADER_BACKTRACK;
                    current_block = 66159954028994333;
                } else {
                    current_block = 12800627514080957624;
                }
            } else {
                current_block = 12800627514080957624;
            }
            match current_block {
                66159954028994333 => {}
                _ => {
                    if !(*(*reader).node).next.is_null() {
                        (*reader).node = (*(*reader).node).next as xmlNodePtr;
                        (*reader).state = XML_TEXTREADER_START;
                    } else if !(*(*reader).node).parent.is_null() {
                        if (*(*(*reader).node).parent).type_0 as c_uint
                            == XML_DOCUMENT_NODE as c_int as c_uint
                            || (*(*(*reader).node).parent).type_0 as c_uint
                                == XML_HTML_DOCUMENT_NODE as c_int
                                    as c_uint
                        {
                            (*reader).state = XML_TEXTREADER_END;
                            return 0 as c_int;
                        }
                        (*reader).node = (*(*reader).node).parent as xmlNodePtr;
                        (*reader).depth -= 1;
                        (*reader).state = XML_TEXTREADER_BACKTRACK;
                    } else {
                        (*reader).state = XML_TEXTREADER_END;
                    }
                }
            }
        }
        if !((*(*reader).node).type_0 as c_uint
            == XML_XINCLUDE_START as c_int as c_uint
            || (*(*reader).node).type_0 as c_uint
                == XML_XINCLUDE_END as c_int as c_uint)
        {
            break;
        }
    }
    return 1 as c_int;
} }
#[inline]
pub fn xmlTextReaderNextSibling(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).doc.is_null() {
        return -(1 as c_int);
    }
    if (*reader).state as c_int == XML_TEXTREADER_END as c_int {
        return 0 as c_int;
    }
    if (*reader).node.is_null() {
        return xmlTextReaderNextTree(reader);
    }
    if !(*(*reader).node).next.is_null() {
        (*reader).node = (*(*reader).node).next as xmlNodePtr;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as c_int;
    }
    return 0 as c_int;
} }
#[inline]
pub unsafe fn xmlNewTextReader(
    mut input: xmlParserInputBufferPtr,
    mut URI: *const c_char,
) -> xmlTextReaderPtr {
    let mut ret: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlTextReader>() as size_t
    ) as xmlTextReaderPtr;
    if ret.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlTextReader>() as size_t,
    );
    (*ret).doc = ::core::ptr::null_mut::<xmlDoc>();
    (*ret).entTab = ::core::ptr::null_mut::<xmlNodePtr>();
    (*ret).entMax = 0 as c_int;
    (*ret).entNr = 0 as c_int;
    (*ret).input = input;
    (*ret).buffer = xmlBufCreateSize(100 as size_t);
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    (*ret).sax = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlSAXHandler>() as size_t,
    ) as *mut xmlSAXHandler as xmlSAXHandlerPtr;
    if (*ret).sax.is_null() {
        xmlBufFree((*ret).buffer);
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    xmlSAXVersion((*ret).sax as *mut xmlSAXHandler, 2 as c_int);
    (*ret).startElement = (*(*ret).sax).startElement;
    (*(*ret).sax).startElement = Some(
        xmlTextReaderStartElement
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*ret).endElement = (*(*ret).sax).endElement;
    (*(*ret).sax).endElement = Some(
        xmlTextReaderEndElement
            as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    if (*(*ret).sax).initialized == XML_SAX2_MAGIC {
        (*ret).startElementNs = (*(*ret).sax).startElementNs;
        (*(*ret).sax).startElementNs = Some(
            xmlTextReaderStartElementNs
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    c_int,
                    *mut *const xmlChar,
                    c_int,
                    c_int,
                    *mut *const xmlChar,
                ) -> (),
        ) as startElementNsSAX2Func;
        (*ret).endElementNs = (*(*ret).sax).endElementNs;
        (*(*ret).sax).endElementNs = Some(
            xmlTextReaderEndElementNs
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ) as endElementNsSAX2Func;
    } else {
        (*ret).startElementNs = None;
        (*ret).endElementNs = None;
    }
    (*ret).characters = (*(*ret).sax).characters;
    (*(*ret).sax).characters = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*(*ret).sax).ignorableWhitespace = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*ret).cdataBlock = (*(*ret).sax).cdataBlock;
    (*(*ret).sax).cdataBlock = Some(
        xmlTextReaderCDataBlock
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as cdataBlockSAXFunc;
    (*ret).mode = XML_TEXTREADER_MODE_INITIAL as c_int;
    (*ret).node = ::core::ptr::null_mut::<xmlNode>();
    (*ret).curnode = ::core::ptr::null_mut::<xmlNode>();
    if xmlBufUse((*(*ret).input).buffer) < 4 as size_t {
        xmlParserInputBufferRead(input, 4 as c_int);
    }
    if xmlBufUse((*(*ret).input).buffer) >= 4 as size_t {
        (*ret).ctxt = xmlCreatePushParserCtxt(
            (*ret).sax,
            NULL,
            xmlBufContent((*(*ret).input).buffer as *const xmlBuf) as *const c_char,
            4 as c_int,
            URI,
        );
        (*ret).base = 0 as c_uint;
        (*ret).cur = 4 as c_uint;
    } else {
        (*ret).ctxt = xmlCreatePushParserCtxt(
            (*ret).sax,
            NULL,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            URI,
        );
        (*ret).base = 0 as c_uint;
        (*ret).cur = 0 as c_uint;
    }
    if (*ret).ctxt.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const c_char,
        );
        xmlBufFree((*ret).buffer);
        xmlFree.expect("non-null function pointer")((*ret).sax as *mut c_void);
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    (*(*ret).ctxt).parseMode = XML_PARSE_READER;
    (*(*ret).ctxt)._private = ret as *mut c_void;
    (*(*ret).ctxt).linenumbers = 1 as c_int;
    (*(*ret).ctxt).dictNames = 1 as c_int;
    (*ret).allocs = XML_TEXTREADER_CTXT;
    (*(*ret).ctxt).docdict = 1 as c_int;
    (*ret).dict = (*(*ret).ctxt).dict;
    (*ret).xinclude = 0 as c_int;
    (*ret).patternMax = 0 as c_int;
    (*ret).patternTab = ::core::ptr::null_mut::<xmlPatternPtr>();
    return ret;
}
#[inline]
pub unsafe fn xmlNewTextReaderFilename(
    mut URI: *const c_char,
) -> xmlTextReaderPtr {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut ret: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    let mut directory: *mut c_char = ::core::ptr::null_mut::<c_char>();
    input = xmlParserInputBufferCreateFilename(URI, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    ret = xmlNewTextReader(input, URI);
    if ret.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    (*ret).allocs |= XML_TEXTREADER_INPUT;
    if (*(*ret).ctxt).directory.is_null() {
        directory = xmlParserGetDirectory(URI);
    }
    if (*(*ret).ctxt).directory.is_null() && !directory.is_null() {
        (*(*ret).ctxt).directory = xmlStrdup(directory as *mut xmlChar) as *mut c_char;
    }
    if !directory.is_null() {
        xmlFree.expect("non-null function pointer")(directory as *mut c_void);
    }
    return ret;
}
#[inline]
pub fn xmlFreeTextReader(mut reader: xmlTextReaderPtr) { unsafe {
    if reader.is_null() {
        return;
    }
    if !(*reader).rngSchemas.is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        (*reader).rngSchemas = ::core::ptr::null_mut::<xmlRelaxNG>();
    }
    if !(*reader).rngValidCtxt.is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        (*reader).rngValidCtxt = ::core::ptr::null_mut::<xmlRelaxNGValidCtxt>();
    }
    if !(*reader).xsdPlug.is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        (*reader).xsdPlug = ::core::ptr::null_mut::<xmlSchemaSAXPlugStruct>();
    }
    if !(*reader).xsdValidCtxt.is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        (*reader).xsdValidCtxt = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
    }
    if !(*reader).xsdSchemas.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
    }
    if !(*reader).xincctxt.is_null() {
        xmlXIncludeFreeContext((*reader).xincctxt);
    }
    if !(*reader).patternTab.is_null() {
        let mut i: c_int = 0;
        i = 0 as c_int;
        while i < (*reader).patternNr {
            if !(*(*reader).patternTab.offset(i as isize)).is_null() {
                xmlFreePattern(*(*reader).patternTab.offset(i as isize));
            }
            i += 1;
        }
        xmlFree.expect("non-null function pointer")(
            (*reader).patternTab as *mut c_void,
        );
    }
    if (*reader).mode != XML_TEXTREADER_MODE_CLOSED as c_int {
        xmlTextReaderClose(reader);
    }
    if !(*reader).ctxt.is_null() {
        if (*reader).dict == (*(*reader).ctxt).dict {
            (*reader).dict = ::core::ptr::null_mut::<xmlDict>();
        }
        if (*reader).allocs & XML_TEXTREADER_CTXT != 0 {
            xmlFreeParserCtxt((*reader).ctxt);
        }
    }
    if !(*reader).sax.is_null() {
        xmlFree.expect("non-null function pointer")((*reader).sax as *mut c_void);
    }
    if !(*reader).buffer.is_null() {
        xmlBufFree((*reader).buffer);
    }
    if !(*reader).entTab.is_null() {
        xmlFree.expect("non-null function pointer")((*reader).entTab as *mut c_void);
    }
    if !(*reader).dict.is_null() {
        xmlDictFree((*reader).dict);
    }
    xmlFree.expect("non-null function pointer")(reader as *mut c_void);
} }
#[inline]
pub fn xmlTextReaderClose(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    (*reader).node = ::core::ptr::null_mut::<xmlNode>();
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).mode = XML_TEXTREADER_MODE_CLOSED as c_int;
    if !(*reader).faketext.is_null() {
        xmlFreeNode((*reader).faketext);
        (*reader).faketext = ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*reader).ctxt.is_null() {
        if !(*(*reader).ctxt).vctxt.vstateTab.is_null()
            && (*(*reader).ctxt).vctxt.vstateMax > 0 as c_int
        {
            while (*(*reader).ctxt).vctxt.vstateNr > 0 as c_int {
                xmlValidatePopElement(
                    &raw mut (*(*reader).ctxt).vctxt,
                    ::core::ptr::null_mut::<xmlDoc>(),
                    ::core::ptr::null_mut::<xmlNode>(),
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            xmlFree.expect("non-null function pointer")(
                (*(*reader).ctxt).vctxt.vstateTab as *mut c_void,
            );
            (*(*reader).ctxt).vctxt.vstateTab = ::core::ptr::null_mut::<xmlValidState>();
            (*(*reader).ctxt).vctxt.vstateMax = 0 as c_int;
        }
        xmlStopParser((*reader).ctxt);
        if !(*(*reader).ctxt).myDoc.is_null() {
            if (*reader).preserve == 0 as c_int {
                xmlTextReaderFreeDoc(reader, (*(*reader).ctxt).myDoc);
            }
            (*(*reader).ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
        }
    }
    if !(*reader).input.is_null() && (*reader).allocs & XML_TEXTREADER_INPUT != 0 {
        xmlFreeParserInputBuffer((*reader).input);
        (*reader).allocs -= XML_TEXTREADER_INPUT;
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlTextReaderGetAttributeNo(
    mut reader: xmlTextReaderPtr,
    mut no: c_int,
) -> *mut xmlChar { unsafe {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut i: c_int = 0;
    let mut cur: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ns = (*(*reader).node).nsDef as xmlNsPtr;
    i = 0 as c_int;
    while i < no && !ns.is_null() {
        ns = (*ns).next as xmlNsPtr;
        i += 1;
    }
    if !ns.is_null() {
        return xmlStrdup((*ns).href);
    }
    cur = (*(*reader).node).properties as xmlAttrPtr;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while i < no {
        cur = (*cur).next as xmlAttrPtr;
        if cur.is_null() {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        i += 1;
    }
    ret = xmlNodeListGetString(
        (*(*reader).node).doc as xmlDocPtr,
        (*cur).children,
        1 as c_int,
    );
    if ret.is_null() {
        return xmlStrdup(b"\0" as *const u8 as *const c_char as *mut xmlChar);
    }
    return ret;
} }
#[inline]
pub unsafe fn xmlTextReaderGetAttribute(
    mut reader: xmlTextReaderPtr,
    mut name: *const xmlChar,
) -> *mut xmlChar {
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut localname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if reader.is_null() || name.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    localname = xmlSplitQName2(name, &raw mut prefix);
    if localname.is_null() {
        if xmlStrEqual(
            name,
            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        {
            ns = (*(*reader).node).nsDef as xmlNsPtr;
            while !ns.is_null() {
                if (*ns).prefix.is_null() {
                    return xmlStrdup((*ns).href);
                }
                ns = (*ns).next as xmlNsPtr;
            }
            return ::core::ptr::null_mut::<xmlChar>();
        }
        return xmlGetNoNsProp((*reader).node as *const xmlNode, name);
    }
    if xmlStrEqual(
        prefix,
        b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
    ) != 0
    {
        ns = (*(*reader).node).nsDef as xmlNsPtr;
        while !ns.is_null() {
            if !(*ns).prefix.is_null() && xmlStrEqual((*ns).prefix, localname) != 0 {
                ret = xmlStrdup((*ns).href);
                break;
            } else {
                ns = (*ns).next as xmlNsPtr;
            }
        }
    } else {
        ns = xmlSearchNs((*(*reader).node).doc as xmlDocPtr, (*reader).node, prefix);
        if !ns.is_null() {
            ret = xmlGetNsProp((*reader).node as *const xmlNode, localname, (*ns).href);
        }
    }
    xmlFree.expect("non-null function pointer")(localname as *mut c_void);
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
    }
    return ret;
}
#[inline]
pub unsafe fn xmlTextReaderGetAttributeNs(
    mut reader: xmlTextReaderPtr,
    mut localName: *const xmlChar,
    mut namespaceURI: *const xmlChar,
) -> *mut xmlChar {
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if reader.is_null() || localName.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlStrEqual(
        namespaceURI,
        b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const c_char
            as *mut xmlChar,
    ) != 0
    {
        if xmlStrEqual(
            localName,
            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
        ) == 0
        {
            prefix = localName as *mut xmlChar;
        }
        ns = (*(*reader).node).nsDef as xmlNsPtr;
        while !ns.is_null() {
            if prefix.is_null() && (*ns).prefix.is_null()
                || !(*ns).prefix.is_null() && xmlStrEqual((*ns).prefix, localName) != 0
            {
                return xmlStrdup((*ns).href);
            }
            ns = (*ns).next as xmlNsPtr;
        }
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlGetNsProp((*reader).node as *const xmlNode, localName, namespaceURI);
}
#[inline]
pub fn xmlTextReaderGetRemainder(
    mut reader: xmlTextReaderPtr,
) -> xmlParserInputBufferPtr { unsafe {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    (*reader).node = ::core::ptr::null_mut::<xmlNode>();
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).mode = XML_TEXTREADER_MODE_EOF as c_int;
    if !(*reader).ctxt.is_null() {
        xmlStopParser((*reader).ctxt);
        if !(*(*reader).ctxt).myDoc.is_null() {
            if (*reader).preserve == 0 as c_int {
                xmlTextReaderFreeDoc(reader, (*(*reader).ctxt).myDoc);
            }
            (*(*reader).ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
        }
    }
    if (*reader).allocs & XML_TEXTREADER_INPUT != 0 {
        ret = (*reader).input;
        (*reader).input = ::core::ptr::null_mut::<xmlParserInputBuffer>();
        (*reader).allocs -= XML_TEXTREADER_INPUT;
    } else {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlreader.c\0" as *const u8
                as *const c_char,
            2405 as c_int,
        );
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    return ret;
} }
#[inline]
pub unsafe fn xmlTextReaderLookupNamespace(
    mut reader: xmlTextReaderPtr,
    mut prefix: *const xmlChar,
) -> *mut xmlChar {
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ns = xmlSearchNs((*(*reader).node).doc as xmlDocPtr, (*reader).node, prefix);
    if ns.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStrdup((*ns).href);
}
#[inline]
pub fn xmlTextReaderMoveToAttributeNo(
    mut reader: xmlTextReaderPtr,
    mut no: c_int,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut cur: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return -(1 as c_int);
    }
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    ns = (*(*reader).node).nsDef as xmlNsPtr;
    i = 0 as c_int;
    while i < no && !ns.is_null() {
        ns = (*ns).next as xmlNsPtr;
        i += 1;
    }
    if !ns.is_null() {
        (*reader).curnode = ns as xmlNodePtr;
        return 1 as c_int;
    }
    cur = (*(*reader).node).properties as xmlAttrPtr;
    if cur.is_null() {
        return 0 as c_int;
    }
    while i < no {
        cur = (*cur).next as xmlAttrPtr;
        if cur.is_null() {
            return 0 as c_int;
        }
        i += 1;
    }
    (*reader).curnode = cur as xmlNodePtr;
    return 1 as c_int;
} }
#[inline]
pub unsafe fn xmlTextReaderMoveToAttribute(
    mut reader: xmlTextReaderPtr,
    mut name: *const xmlChar,
) -> c_int {
    let mut current_block: u64;
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut localname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if reader.is_null() || name.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    localname = xmlSplitQName2(name, &raw mut prefix);
    if localname.is_null() {
        if xmlStrEqual(
            name,
            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        {
            ns = (*(*reader).node).nsDef as xmlNsPtr;
            while !ns.is_null() {
                if (*ns).prefix.is_null() {
                    (*reader).curnode = ns as xmlNodePtr;
                    return 1 as c_int;
                }
                ns = (*ns).next as xmlNsPtr;
            }
            return 0 as c_int;
        }
        prop = (*(*reader).node).properties as xmlAttrPtr;
        while !prop.is_null() {
            if xmlStrEqual((*prop).name, name) != 0
                && ((*prop).ns.is_null() || (*(*prop).ns).prefix.is_null())
            {
                (*reader).curnode = prop as xmlNodePtr;
                return 1 as c_int;
            }
            prop = (*prop).next as xmlAttrPtr;
        }
        return 0 as c_int;
    }
    if xmlStrEqual(
        prefix,
        b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
    ) != 0
    {
        ns = (*(*reader).node).nsDef as xmlNsPtr;
        loop {
            if ns.is_null() {
                current_block = 12264598442885531551;
                break;
            }
            if !(*ns).prefix.is_null() && xmlStrEqual((*ns).prefix, localname) != 0 {
                (*reader).curnode = ns as xmlNodePtr;
                current_block = 675685531659200550;
                break;
            } else {
                ns = (*ns).next as xmlNsPtr;
            }
        }
    } else {
        prop = (*(*reader).node).properties as xmlAttrPtr;
        loop {
            if prop.is_null() {
                current_block = 12264598442885531551;
                break;
            }
            if xmlStrEqual((*prop).name, localname) != 0
                && !(*prop).ns.is_null()
                && xmlStrEqual((*(*prop).ns).prefix, prefix) != 0
            {
                (*reader).curnode = prop as xmlNodePtr;
                current_block = 675685531659200550;
                break;
            } else {
                prop = (*prop).next as xmlAttrPtr;
            }
        }
    }
    match current_block {
        675685531659200550 => {
            if !localname.is_null() {
                xmlFree.expect("non-null function pointer")(localname as *mut c_void);
            }
            if !prefix.is_null() {
                xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
            }
            return 1 as c_int;
        }
        _ => {
            if !localname.is_null() {
                xmlFree.expect("non-null function pointer")(localname as *mut c_void);
            }
            if !prefix.is_null() {
                xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
            }
            return 0 as c_int;
        }
    };
}
#[inline]
pub unsafe fn xmlTextReaderMoveToAttributeNs(
    mut reader: xmlTextReaderPtr,
    mut localName: *const xmlChar,
    mut namespaceURI: *const xmlChar,
) -> c_int {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if reader.is_null() || localName.is_null() || namespaceURI.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    node = (*reader).node;
    if xmlStrEqual(
        namespaceURI,
        b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const c_char
            as *mut xmlChar,
    ) != 0
    {
        if xmlStrEqual(
            localName,
            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
        ) == 0
        {
            prefix = localName as *mut xmlChar;
        }
        ns = (*(*reader).node).nsDef as xmlNsPtr;
        while !ns.is_null() {
            if prefix.is_null() && (*ns).prefix.is_null()
                || !(*ns).prefix.is_null() && xmlStrEqual((*ns).prefix, localName) != 0
            {
                (*reader).curnode = ns as xmlNodePtr;
                return 1 as c_int;
            }
            ns = (*ns).next as xmlNsPtr;
        }
        return 0 as c_int;
    }
    prop = (*node).properties as xmlAttrPtr;
    while !prop.is_null() {
        if xmlStrEqual((*prop).name, localName) != 0
            && (!(*prop).ns.is_null() && xmlStrEqual((*(*prop).ns).href, namespaceURI) != 0)
        {
            (*reader).curnode = prop as xmlNodePtr;
            return 1 as c_int;
        }
        prop = (*prop).next as xmlAttrPtr;
    }
    return 0 as c_int;
}
#[inline]
pub fn xmlTextReaderMoveToFirstAttribute(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    if !(*(*reader).node).nsDef.is_null() {
        (*reader).curnode = (*(*reader).node).nsDef as xmlNodePtr;
        return 1 as c_int;
    }
    if !(*(*reader).node).properties.is_null() {
        (*reader).curnode = (*(*reader).node).properties as xmlNodePtr;
        return 1 as c_int;
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlTextReaderMoveToNextAttribute(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    if (*reader).curnode.is_null() {
        return xmlTextReaderMoveToFirstAttribute(reader);
    }
    if (*(*reader).curnode).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        let mut ns: xmlNsPtr = (*reader).curnode as xmlNsPtr;
        if !(*ns).next.is_null() {
            (*reader).curnode = (*ns).next as xmlNodePtr;
            return 1 as c_int;
        }
        if !(*(*reader).node).properties.is_null() {
            (*reader).curnode = (*(*reader).node).properties as xmlNodePtr;
            return 1 as c_int;
        }
        return 0 as c_int;
    } else if (*(*reader).curnode).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
        && !(*(*reader).curnode).next.is_null()
    {
        (*reader).curnode = (*(*reader).curnode).next as xmlNodePtr;
        return 1 as c_int;
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlTextReaderMoveToElement(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    if !(*reader).curnode.is_null() {
        (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
        return 1 as c_int;
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlTextReaderReadAttributeValue(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if (*reader).curnode.is_null() {
        return 0 as c_int;
    }
    if (*(*reader).curnode).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        if (*(*reader).curnode).children.is_null() {
            return 0 as c_int;
        }
        (*reader).curnode = (*(*reader).curnode).children as xmlNodePtr;
    } else if (*(*reader).curnode).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        let mut ns: xmlNsPtr = (*reader).curnode as xmlNsPtr;
        if (*reader).faketext.is_null() {
            (*reader).faketext = xmlNewDocText((*(*reader).node).doc, (*ns).href);
        } else {
            if !(*(*reader).faketext).content.is_null()
                && (*(*reader).faketext).content
                    != &raw mut (*(*reader).faketext).properties as *mut xmlChar
            {
                xmlFree.expect("non-null function pointer")(
                    (*(*reader).faketext).content as *mut c_void,
                );
            }
            (*(*reader).faketext).content = xmlStrdup((*ns).href);
        }
        (*reader).curnode = (*reader).faketext;
    } else {
        if (*(*reader).curnode).next.is_null() {
            return 0 as c_int;
        }
        (*reader).curnode = (*(*reader).curnode).next as xmlNodePtr;
    }
    return 1 as c_int;
} }
#[inline]
pub fn xmlTextReaderConstEncoding(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar { unsafe {
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if reader.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).doc.is_null() {
        doc = (*reader).doc;
    } else if !(*reader).ctxt.is_null() {
        doc = (*(*reader).ctxt).myDoc;
    }
    if doc.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if (*doc).encoding.is_null() {
        return ::core::ptr::null::<xmlChar>();
    } else {
        return xmlDictLookup((*reader).dict, (*doc).encoding, -(1 as c_int));
    };
} }
#[inline]
pub fn xmlTextReaderAttributeCount(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return 0 as c_int;
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    if (*reader).state as c_int == XML_TEXTREADER_END as c_int
        || (*reader).state as c_int == XML_TEXTREADER_BACKTRACK as c_int
    {
        return 0 as c_int;
    }
    ret = 0 as c_int;
    attr = (*node).properties as xmlAttrPtr;
    while !attr.is_null() {
        ret += 1;
        attr = (*attr).next as xmlAttrPtr;
    }
    ns = (*node).nsDef as xmlNsPtr;
    while !ns.is_null() {
        ret += 1;
        ns = (*ns).next as xmlNsPtr;
    }
    return ret;
} }
#[inline]
pub fn xmlTextReaderNodeType(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return XML_READER_TYPE_NONE as c_int;
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as c_uint {
        1 => {
            if (*reader).state as c_int == XML_TEXTREADER_END as c_int
                || (*reader).state as c_int
                    == XML_TEXTREADER_BACKTRACK as c_int
            {
                return XML_READER_TYPE_END_ELEMENT as c_int;
            }
            return XML_READER_TYPE_ELEMENT as c_int;
        }
        18 | 2 => return XML_READER_TYPE_ATTRIBUTE as c_int,
        3 => {
            if xmlIsBlankNode((*reader).node as *const xmlNode) != 0 {
                if xmlNodeGetSpacePreserve((*reader).node as *const xmlNode) != 0 {
                    return XML_READER_TYPE_SIGNIFICANT_WHITESPACE as c_int;
                } else {
                    return XML_READER_TYPE_WHITESPACE as c_int;
                }
            } else {
                return XML_READER_TYPE_TEXT as c_int;
            }
        }
        4 => return XML_READER_TYPE_CDATA as c_int,
        5 => return XML_READER_TYPE_ENTITY_REFERENCE as c_int,
        6 => return XML_READER_TYPE_ENTITY as c_int,
        7 => return XML_READER_TYPE_PROCESSING_INSTRUCTION as c_int,
        8 => return XML_READER_TYPE_COMMENT as c_int,
        9 | 13 => return XML_READER_TYPE_DOCUMENT as c_int,
        11 => return XML_READER_TYPE_DOCUMENT_FRAGMENT as c_int,
        12 => return XML_READER_TYPE_NOTATION as c_int,
        10 | 14 => return XML_READER_TYPE_DOCUMENT_TYPE as c_int,
        15 | 16 | 17 | 19 | 20 => return XML_READER_TYPE_NONE as c_int,
        _ => {}
    }
    return -(1 as c_int);
} }
#[inline]
pub fn xmlTextReaderIsEmptyElement(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() || (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if (*(*reader).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    if !(*reader).curnode.is_null() {
        return 0 as c_int;
    }
    if !(*(*reader).node).children.is_null() {
        return 0 as c_int;
    }
    if (*reader).state as c_int == XML_TEXTREADER_END as c_int {
        return 0 as c_int;
    }
    if !(*reader).doc.is_null() {
        return 1 as c_int;
    }
    if (*reader).in_xinclude > 0 as c_int {
        return 1 as c_int;
    }
    return ((*(*reader).node).extra as c_int & NODE_IS_EMPTY
        != 0 as c_int) as c_int;
} }
#[inline]
pub fn xmlTextReaderLocalName(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return xmlStrdup(
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            );
        } else {
            return xmlStrdup((*ns).prefix);
        }
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return xmlTextReaderName(reader);
    }
    return xmlStrdup((*node).name);
} }
#[inline]
pub fn xmlTextReaderConstLocalName(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return xmlDictLookup(
                (*reader).dict,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                -(1 as c_int),
            );
        } else {
            return (*ns).prefix;
        }
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return xmlTextReaderConstName(reader);
    }
    return (*node).name;
} }
#[inline]
pub fn xmlTextReaderName(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as c_uint {
        1 | 2 => {
            if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
                return xmlStrdup((*node).name);
            }
            ret = xmlStrdup((*(*node).ns).prefix);
            ret = xmlStrcat(
                ret,
                b":\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            ret = xmlStrcat(ret, (*node).name);
            return ret;
        }
        3 => {
            return xmlStrdup(
                b"#text\0" as *const u8 as *const c_char as *mut xmlChar,
            );
        }
        4 => {
            return xmlStrdup(
                b"#cdata-section\0" as *const u8 as *const c_char as *mut xmlChar,
            );
        }
        6 | 5 => return xmlStrdup((*node).name),
        7 => return xmlStrdup((*node).name),
        8 => {
            return xmlStrdup(
                b"#comment\0" as *const u8 as *const c_char as *mut xmlChar,
            );
        }
        9 | 13 => {
            return xmlStrdup(
                b"#document\0" as *const u8 as *const c_char as *mut xmlChar,
            );
        }
        11 => {
            return xmlStrdup(
                b"#document-fragment\0" as *const u8 as *const c_char as *mut xmlChar,
            );
        }
        12 => return xmlStrdup((*node).name),
        10 | 14 => return xmlStrdup((*node).name),
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            ret = xmlStrdup(b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar);
            if (*ns).prefix.is_null() {
                return ret;
            }
            ret = xmlStrcat(
                ret,
                b":\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            ret = xmlStrcat(ret, (*ns).prefix);
            return ret;
        }
        15 | 16 | 17 | 19 | 20 => return ::core::ptr::null_mut::<xmlChar>(),
        _ => {}
    }
    return ::core::ptr::null_mut::<xmlChar>();
} }
#[inline]
pub fn xmlTextReaderConstName(mut reader: xmlTextReaderPtr) -> *const xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as c_uint {
        1 | 2 => {
            if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
                return (*node).name;
            }
            return xmlDictQLookup((*reader).dict, (*(*node).ns).prefix, (*node).name);
        }
        3 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#text\0" as *const u8 as *const c_char as *mut xmlChar,
                -(1 as c_int),
            );
        }
        4 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#cdata-section\0" as *const u8 as *const c_char as *mut xmlChar,
                -(1 as c_int),
            );
        }
        6 | 5 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as c_int));
        }
        7 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as c_int));
        }
        8 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#comment\0" as *const u8 as *const c_char as *mut xmlChar,
                -(1 as c_int),
            );
        }
        9 | 13 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#document\0" as *const u8 as *const c_char as *mut xmlChar,
                -(1 as c_int),
            );
        }
        11 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#document-fragment\0" as *const u8 as *const c_char as *mut xmlChar,
                -(1 as c_int),
            );
        }
        12 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as c_int));
        }
        10 | 14 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as c_int));
        }
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            if (*ns).prefix.is_null() {
                return xmlDictLookup(
                    (*reader).dict,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                    -(1 as c_int),
                );
            }
            return xmlDictQLookup(
                (*reader).dict,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                (*ns).prefix,
            );
        }
        15 | 16 | 17 | 19 | 20 => return ::core::ptr::null::<xmlChar>(),
        _ => {}
    }
    return ::core::ptr::null::<xmlChar>();
} }
#[inline]
pub fn xmlTextReaderPrefix(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        return xmlStrdup(b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar);
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
        return xmlStrdup((*(*node).ns).prefix);
    }
    return ::core::ptr::null_mut::<xmlChar>();
} }
#[inline]
pub fn xmlTextReaderConstPrefix(mut reader: xmlTextReaderPtr) -> *const xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return ::core::ptr::null::<xmlChar>();
        }
        return xmlDictLookup(
            (*reader).dict,
            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            -(1 as c_int),
        );
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
        return xmlDictLookup(
            (*reader).dict,
            (*(*node).ns).prefix,
            -(1 as c_int),
        );
    }
    return ::core::ptr::null::<xmlChar>();
} }
#[inline]
pub fn xmlTextReaderNamespaceUri(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return xmlStrdup(
            b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const c_char
                as *mut xmlChar,
        );
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*node).ns.is_null() {
        return xmlStrdup((*(*node).ns).href);
    }
    return ::core::ptr::null_mut::<xmlChar>();
} }
#[inline]
pub fn xmlTextReaderConstNamespaceUri(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return xmlDictLookup(
            (*reader).dict,
            b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const c_char
                as *mut xmlChar,
            -(1 as c_int),
        );
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*node).ns.is_null() {
        return xmlDictLookup(
            (*reader).dict,
            (*(*node).ns).href,
            -(1 as c_int),
        );
    }
    return ::core::ptr::null::<xmlChar>();
} }
#[inline]
pub fn xmlTextReaderBaseUri(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlNodeGetBase(
        ::core::ptr::null::<xmlDoc>(),
        (*reader).node as *const xmlNode,
    );
} }
#[inline]
pub fn xmlTextReaderConstBaseUri(mut reader: xmlTextReaderPtr) -> *const xmlChar { unsafe {
    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    tmp = xmlNodeGetBase(
        ::core::ptr::null::<xmlDoc>(),
        (*reader).node as *const xmlNode,
    );
    if tmp.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    ret = xmlDictLookup((*reader).dict, tmp, -(1 as c_int));
    xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
    return ret;
} }
#[inline]
pub fn xmlTextReaderDepth(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return 0 as c_int;
    }
    if !(*reader).curnode.is_null() {
        if (*(*reader).curnode).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
            || (*(*reader).curnode).type_0 as c_uint
                == XML_NAMESPACE_DECL as c_int as c_uint
        {
            return (*reader).depth + 1 as c_int;
        }
        return (*reader).depth + 2 as c_int;
    }
    return (*reader).depth;
} }
#[inline]
pub fn xmlTextReaderHasAttributes(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return 0 as c_int;
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        && (!(*node).properties.is_null() || !(*node).nsDef.is_null())
    {
        return 1 as c_int;
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlTextReaderHasValue(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return 0 as c_int;
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as c_uint {
        2 | 3 | 4 | 7 | 8 | 18 => return 1 as c_int,
        _ => {}
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlTextReaderValue(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as c_uint {
        18 => return xmlStrdup((*(node as xmlNsPtr)).href),
        2 => {
            let mut attr: xmlAttrPtr = node as xmlAttrPtr;
            if !(*attr).parent.is_null() {
                return xmlNodeListGetString(
                    (*(*attr).parent).doc as xmlDocPtr,
                    (*attr).children,
                    1 as c_int,
                );
            } else {
                return xmlNodeListGetString(
                    ::core::ptr::null_mut::<xmlDoc>(),
                    (*attr).children,
                    1 as c_int,
                );
            }
        }
        3 | 4 | 7 | 8 => {
            if !(*node).content.is_null() {
                return xmlStrdup((*node).content);
            }
        }
        _ => {}
    }
    return ::core::ptr::null_mut::<xmlChar>();
} }
#[inline]
pub fn xmlTextReaderConstValue(mut reader: xmlTextReaderPtr) -> *const xmlChar { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as c_uint {
        18 => return (*(node as xmlNsPtr)).href,
        2 => {
            let mut attr: xmlAttrPtr = node as xmlAttrPtr;
            let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
            if !(*attr).children.is_null()
                && (*(*attr).children).type_0 as c_uint
                    == XML_TEXT_NODE as c_int as c_uint
                && (*(*attr).children).next.is_null()
            {
                return (*(*attr).children).content;
            } else {
                if (*reader).buffer.is_null() {
                    (*reader).buffer = xmlBufCreateSize(100 as size_t);
                    if (*reader).buffer.is_null() {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"xmlTextReaderSetup : malloc failed\n\0" as *const u8
                                as *const c_char,
                        );
                        return ::core::ptr::null::<xmlChar>();
                    }
                    xmlBufSetAllocationScheme((*reader).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
                } else {
                    xmlBufEmpty((*reader).buffer);
                }
                xmlBufGetNodeContent((*reader).buffer, node as *const xmlNode);
                ret = xmlBufContent((*reader).buffer as *const xmlBuf);
                if ret.is_null() {
                    xmlBufFree((*reader).buffer);
                    (*reader).buffer = xmlBufCreateSize(100 as size_t);
                    xmlBufSetAllocationScheme((*reader).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
                    ret = b"\0" as *const u8 as *const c_char as *mut xmlChar;
                }
                return ret;
            }
        }
        3 | 4 | 7 | 8 => return (*node).content,
        _ => {}
    }
    return ::core::ptr::null::<xmlChar>();
} }
#[inline]
pub fn xmlTextReaderIsDefault(
    mut reader: xmlTextReaderPtr,
) -> c_int { {
    if reader.is_null() {
        return -(1 as c_int);
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlTextReaderQuoteChar(
    mut reader: xmlTextReaderPtr,
) -> c_int { {
    if reader.is_null() {
        return -(1 as c_int);
    }
    return '"' as i32;
} }
#[inline]
pub fn xmlTextReaderXmlLang(mut reader: xmlTextReaderPtr) -> *mut xmlChar { unsafe {
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlNodeGetLang((*reader).node as *const xmlNode);
} }
#[inline]
pub fn xmlTextReaderConstXmlLang(mut reader: xmlTextReaderPtr) -> *const xmlChar { unsafe {
    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if reader.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    tmp = xmlNodeGetLang((*reader).node as *const xmlNode);
    if tmp.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    ret = xmlDictLookup((*reader).dict, tmp, -(1 as c_int));
    xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
    return ret;
} }
#[inline]
pub unsafe fn xmlTextReaderConstString(
    mut reader: xmlTextReaderPtr,
    mut str: *const xmlChar,
) -> *const xmlChar {
    if reader.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return xmlDictLookup((*reader).dict, str, -(1 as c_int));
}
#[inline]
pub fn xmlTextReaderNormalization(
    mut reader: xmlTextReaderPtr,
) -> c_int { {
    if reader.is_null() {
        return -(1 as c_int);
    }
    return 1 as c_int;
} }
#[inline]
pub fn xmlTextReaderSetParserProp(
    mut reader: xmlTextReaderPtr,
    mut prop: c_int,
    mut value: c_int,
) -> c_int { unsafe {
    let mut p: xmlParserProperties = prop as xmlParserProperties;
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as c_int);
    }
    ctxt = (*reader).ctxt;
    match p as c_uint {
        1 => {
            if value != 0 as c_int {
                if (*ctxt).loadsubset == 0 as c_int {
                    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as c_int {
                        return -(1 as c_int);
                    }
                    (*ctxt).loadsubset = XML_DETECT_IDS;
                }
            } else {
                (*ctxt).loadsubset = 0 as c_int;
            }
            return 0 as c_int;
        }
        2 => {
            if value != 0 as c_int {
                (*ctxt).loadsubset |= XML_COMPLETE_ATTRS;
            } else if (*ctxt).loadsubset & XML_COMPLETE_ATTRS != 0 {
                (*ctxt).loadsubset -= XML_COMPLETE_ATTRS;
            }
            return 0 as c_int;
        }
        3 => {
            if value != 0 as c_int {
                (*ctxt).options |= XML_PARSE_DTDVALID as c_int;
                (*ctxt).validate = 1 as c_int;
                (*reader).validate = XML_TEXTREADER_VALIDATE_DTD;
            } else {
                (*ctxt).options &= !(XML_PARSE_DTDVALID as c_int);
                (*ctxt).validate = 0 as c_int;
            }
            return 0 as c_int;
        }
        4 => {
            if value != 0 as c_int {
                (*ctxt).options |= XML_PARSE_NOENT as c_int;
                (*ctxt).replaceEntities = 1 as c_int;
            } else {
                (*ctxt).options &= !(XML_PARSE_NOENT as c_int);
                (*ctxt).replaceEntities = 0 as c_int;
            }
            return 0 as c_int;
        }
        _ => {}
    }
    return -(1 as c_int);
} }
#[inline]
pub fn xmlTextReaderGetParserProp(
    mut reader: xmlTextReaderPtr,
    mut prop: c_int,
) -> c_int { unsafe {
    let mut p: xmlParserProperties = prop as xmlParserProperties;
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as c_int);
    }
    ctxt = (*reader).ctxt;
    match p as c_uint {
        1 => {
            if (*ctxt).loadsubset != 0 as c_int
                || (*ctxt).validate != 0 as c_int
            {
                return 1 as c_int;
            }
            return 0 as c_int;
        }
        2 => {
            if (*ctxt).loadsubset & XML_COMPLETE_ATTRS != 0 {
                return 1 as c_int;
            }
            return 0 as c_int;
        }
        3 => return (*reader).validate as c_int,
        4 => return (*ctxt).replaceEntities,
        _ => {}
    }
    return -(1 as c_int);
} }
#[inline]
pub fn xmlTextReaderGetParserLineNumber(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() || (*reader).ctxt.is_null() || (*(*reader).ctxt).input.is_null() {
        return 0 as c_int;
    }
    return (*(*(*reader).ctxt).input).line;
} }
#[inline]
pub fn xmlTextReaderGetParserColumnNumber(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    if reader.is_null() || (*reader).ctxt.is_null() || (*(*reader).ctxt).input.is_null() {
        return 0 as c_int;
    }
    return (*(*(*reader).ctxt).input).col;
} }
#[inline]
pub fn xmlTextReaderCurrentNode(mut reader: xmlTextReaderPtr) -> xmlNodePtr { unsafe {
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*reader).curnode.is_null() {
        return (*reader).curnode;
    }
    return (*reader).node;
} }
#[inline]
pub fn xmlTextReaderPreserve(mut reader: xmlTextReaderPtr) -> xmlNodePtr { unsafe {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*reader).curnode.is_null() {
        cur = (*reader).curnode;
    } else {
        cur = (*reader).node;
    }
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*cur).type_0 as c_uint
        != XML_DOCUMENT_NODE as c_int as c_uint
        && (*cur).type_0 as c_uint
            != XML_DTD_NODE as c_int as c_uint
    {
        (*cur).extra =
            ((*cur).extra as c_int | NODE_IS_PRESERVED) as c_ushort;
        (*cur).extra =
            ((*cur).extra as c_int | NODE_IS_SPRESERVED) as c_ushort;
    }
    (*reader).preserves += 1;
    parent = (*cur).parent as xmlNodePtr;
    while !parent.is_null() {
        if (*parent).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            (*parent).extra = ((*parent).extra as c_int | NODE_IS_PRESERVED)
                as c_ushort;
        }
        parent = (*parent).parent as xmlNodePtr;
    }
    return cur;
} }
#[inline]
pub unsafe fn xmlTextReaderPreservePattern(
    mut reader: xmlTextReaderPtr,
    mut pattern: *const xmlChar,
    mut namespaces: *mut *const xmlChar,
) -> c_int {
    let mut comp: xmlPatternPtr = ::core::ptr::null_mut::<xmlPattern>();
    if reader.is_null() || pattern.is_null() {
        return -(1 as c_int);
    }
    comp = xmlPatterncompile(
        pattern,
        (*reader).dict as *mut xmlDict,
        0 as c_int,
        namespaces,
    );
    if comp.is_null() {
        return -(1 as c_int);
    }
    if (*reader).patternMax <= 0 as c_int {
        (*reader).patternMax = 4 as c_int;
        (*reader).patternTab = xmlMalloc.expect("non-null function pointer")(
            ((*reader).patternMax as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlPatternPtr>() as size_t),
        ) as *mut xmlPatternPtr;
        if (*reader).patternTab.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlMalloc failed !\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
    }
    if (*reader).patternNr >= (*reader).patternMax {
        let mut tmp: *mut xmlPatternPtr = ::core::ptr::null_mut::<xmlPatternPtr>();
        (*reader).patternMax *= 2 as c_int;
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*reader).patternTab as *mut c_void,
            ((*reader).patternMax as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlPatternPtr>() as size_t),
        ) as *mut xmlPatternPtr;
        if tmp.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const c_char,
            );
            (*reader).patternMax /= 2 as c_int;
            return -(1 as c_int);
        }
        (*reader).patternTab = tmp;
    }
    let ref mut fresh4 = *(*reader).patternTab.offset((*reader).patternNr as isize);
    *fresh4 = comp;
    let fresh5 = (*reader).patternNr;
    (*reader).patternNr = (*reader).patternNr + 1;
    return fresh5;
}
#[inline]
pub fn xmlTextReaderCurrentDoc(mut reader: xmlTextReaderPtr) -> xmlDocPtr { unsafe {
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if !(*reader).doc.is_null() {
        return (*reader).doc;
    }
    if (*reader).ctxt.is_null() || (*(*reader).ctxt).myDoc.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    (*reader).preserve = 1 as c_int;
    return (*(*reader).ctxt).myDoc;
} }
unsafe extern "C" fn xmlTextReaderValidityErrorRelay(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut reader: xmlTextReaderPtr = ctx as xmlTextReaderPtr;
    let mut str: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    str = xmlTextReaderBuildMessage(msg, ap.as_va_list());
    if (*reader).errorFunc.is_none() {
        xmlTextReaderValidityError(ctx, b"%s\0" as *const u8 as *const c_char, str);
    } else {
        (*reader).errorFunc.expect("non-null function pointer")(
            (*reader).errorFuncArg,
            str,
            XML_PARSER_SEVERITY_VALIDITY_ERROR,
            NULL,
        );
    }
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut c_void);
    }
}
unsafe extern "C" fn xmlTextReaderValidityWarningRelay(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut reader: xmlTextReaderPtr = ctx as xmlTextReaderPtr;
    let mut str: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    str = xmlTextReaderBuildMessage(msg, ap.as_va_list());
    if (*reader).errorFunc.is_none() {
        xmlTextReaderValidityWarning(ctx, b"%s\0" as *const u8 as *const c_char, str);
    } else {
        (*reader).errorFunc.expect("non-null function pointer")(
            (*reader).errorFuncArg,
            str,
            XML_PARSER_SEVERITY_VALIDITY_WARNING,
            NULL,
        );
    }
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut c_void);
    }
}
unsafe extern "C" fn xmlTextReaderValidityStructuredRelay(
    mut userData: *mut c_void,
    mut error: *const xmlError,
) {
    let mut reader: xmlTextReaderPtr = userData as xmlTextReaderPtr;
    if (*reader).sErrorFunc.is_some() {
        (*reader).sErrorFunc.expect("non-null function pointer")((*reader).errorFuncArg, error);
    } else {
        xmlTextReaderStructuredError(reader as *mut c_void, error);
    };
}
#[inline]
pub fn xmlTextReaderRelaxNGSetSchema(
    mut reader: xmlTextReaderPtr,
    mut schema: xmlRelaxNGPtr,
) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if schema.is_null() {
        if !(*reader).rngSchemas.is_null() {
            xmlRelaxNGFree((*reader).rngSchemas);
            (*reader).rngSchemas = ::core::ptr::null_mut::<xmlRelaxNG>();
        }
        if !(*reader).rngValidCtxt.is_null() {
            if (*reader).rngPreserveCtxt == 0 {
                xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
            }
            (*reader).rngValidCtxt = ::core::ptr::null_mut::<xmlRelaxNGValidCtxt>();
        }
        (*reader).rngPreserveCtxt = 0 as c_int;
        return 0 as c_int;
    }
    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as c_int {
        return -(1 as c_int);
    }
    if !(*reader).rngSchemas.is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        (*reader).rngSchemas = ::core::ptr::null_mut::<xmlRelaxNG>();
    }
    if !(*reader).rngValidCtxt.is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        (*reader).rngValidCtxt = ::core::ptr::null_mut::<xmlRelaxNGValidCtxt>();
    }
    (*reader).rngPreserveCtxt = 0 as c_int;
    (*reader).rngValidCtxt = xmlRelaxNGNewValidCtxt(schema);
    if (*reader).rngValidCtxt.is_null() {
        return -(1 as c_int);
    }
    if (*reader).errorFunc.is_some() {
        xmlRelaxNGSetValidErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        ...
                    ) -> (),
            ),
            reader as *mut c_void,
        );
    }
    if (*reader).sErrorFunc.is_some() {
        xmlRelaxNGSetValidStructuredErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut c_void, *const xmlError) -> (),
            ),
            reader as *mut c_void,
        );
    }
    (*reader).rngValidErrors = 0 as c_int;
    (*reader).rngFullNode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).validate = XML_TEXTREADER_VALIDATE_RNG;
    return 0 as c_int;
} }
unsafe extern "C" fn xmlTextReaderLocator(
    mut ctx: *mut c_void,
    mut file: *mut *const c_char,
    mut line: *mut c_ulong,
) -> c_int {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    if ctx.is_null() || file.is_null() && line.is_null() {
        return -(1 as c_int);
    }
    if !file.is_null() {
        *file = ::core::ptr::null::<c_char>();
    }
    if !line.is_null() {
        *line = 0 as c_ulong;
    }
    reader = ctx as xmlTextReaderPtr;
    if !(*reader).ctxt.is_null() && !(*(*reader).ctxt).input.is_null() {
        if !file.is_null() {
            *file = (*(*(*reader).ctxt).input).filename;
        }
        if !line.is_null() {
            *line = (*(*(*reader).ctxt).input).line as c_ulong;
        }
        return 0 as c_int;
    }
    if !(*reader).node.is_null() {
        let mut res: c_long = 0;
        let mut ret: c_int = 0 as c_int;
        if !line.is_null() {
            res = xmlGetLineNo((*reader).node as *const xmlNode);
            if res > 0 as c_long {
                *line = res as c_ulong;
            } else {
                ret = -(1 as c_int);
            }
        }
        if !file.is_null() {
            let mut doc: xmlDocPtr = (*(*reader).node).doc as xmlDocPtr;
            if !doc.is_null() && !(*doc).URL.is_null() {
                *file = (*doc).URL as *const c_char;
            } else {
                ret = -(1 as c_int);
            }
        }
        return ret;
    }
    return -(1 as c_int);
}
#[inline]
pub fn xmlTextReaderSetSchema(
    mut reader: xmlTextReaderPtr,
    mut schema: xmlSchemaPtr,
) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if schema.is_null() {
        if !(*reader).xsdPlug.is_null() {
            xmlSchemaSAXUnplug((*reader).xsdPlug);
            (*reader).xsdPlug = ::core::ptr::null_mut::<xmlSchemaSAXPlugStruct>();
        }
        if !(*reader).xsdValidCtxt.is_null() {
            if (*reader).xsdPreserveCtxt == 0 {
                xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
            }
            (*reader).xsdValidCtxt = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
        }
        (*reader).xsdPreserveCtxt = 0 as c_int;
        if !(*reader).xsdSchemas.is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
        }
        return 0 as c_int;
    }
    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as c_int {
        return -(1 as c_int);
    }
    if !(*reader).xsdPlug.is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        (*reader).xsdPlug = ::core::ptr::null_mut::<xmlSchemaSAXPlugStruct>();
    }
    if !(*reader).xsdValidCtxt.is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        (*reader).xsdValidCtxt = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
    }
    (*reader).xsdPreserveCtxt = 0 as c_int;
    if !(*reader).xsdSchemas.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
    }
    (*reader).xsdValidCtxt = xmlSchemaNewValidCtxt(schema);
    if (*reader).xsdValidCtxt.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
        return -(1 as c_int);
    }
    (*reader).xsdPlug = xmlSchemaSAXPlug(
        (*reader).xsdValidCtxt,
        &raw mut (*(*reader).ctxt).sax,
        &raw mut (*(*reader).ctxt).userData,
    );
    if (*reader).xsdPlug.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
        xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        (*reader).xsdValidCtxt = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
        return -(1 as c_int);
    }
    xmlSchemaValidateSetLocator(
        (*reader).xsdValidCtxt,
        Some(
            xmlTextReaderLocator
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut *const c_char,
                    *mut c_ulong,
                ) -> c_int,
        ),
        reader as *mut c_void,
    );
    if (*reader).errorFunc.is_some() {
        xmlSchemaSetValidErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        ...
                    ) -> (),
            ),
            reader as *mut c_void,
        );
    }
    if (*reader).sErrorFunc.is_some() {
        xmlSchemaSetValidStructuredErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut c_void, *const xmlError) -> (),
            ),
            reader as *mut c_void,
        );
    }
    (*reader).xsdValidErrors = 0 as c_int;
    (*reader).validate = XML_TEXTREADER_VALIDATE_XSD;
    return 0 as c_int;
} }
unsafe fn xmlTextReaderRelaxNGValidateInternal(
    mut reader: xmlTextReaderPtr,
    mut rng: *const c_char,
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut options: c_int,
) -> c_int {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if !rng.is_null() && !ctxt.is_null() {
        return -(1 as c_int);
    }
    if (!rng.is_null() || !ctxt.is_null())
        && ((*reader).mode != XML_TEXTREADER_MODE_INITIAL as c_int
            || (*reader).ctxt.is_null())
    {
        return -(1 as c_int);
    }
    if !(*reader).rngValidCtxt.is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        (*reader).rngValidCtxt = ::core::ptr::null_mut::<xmlRelaxNGValidCtxt>();
    }
    (*reader).rngPreserveCtxt = 0 as c_int;
    if !(*reader).rngSchemas.is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        (*reader).rngSchemas = ::core::ptr::null_mut::<xmlRelaxNG>();
    }
    if rng.is_null() && ctxt.is_null() {
        return 0 as c_int;
    }
    if !rng.is_null() {
        let mut pctxt: xmlRelaxNGParserCtxtPtr = ::core::ptr::null_mut::<xmlRelaxNGParserCtxt>();
        pctxt = xmlRelaxNGNewParserCtxt(rng);
        if (*reader).errorFunc.is_some() {
            xmlRelaxNGSetParserErrors(
                pctxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            ...
                        ) -> (),
                ),
                reader as *mut c_void,
            );
        }
        if (*reader).sErrorFunc.is_some() {
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut c_void, *const xmlError) -> (),
                ),
                reader as *mut c_void,
            );
        }
        (*reader).rngSchemas = xmlRelaxNGParse(pctxt);
        xmlRelaxNGFreeParserCtxt(pctxt);
        if (*reader).rngSchemas.is_null() {
            return -(1 as c_int);
        }
        (*reader).rngValidCtxt = xmlRelaxNGNewValidCtxt((*reader).rngSchemas);
        if (*reader).rngValidCtxt.is_null() {
            xmlRelaxNGFree((*reader).rngSchemas);
            (*reader).rngSchemas = ::core::ptr::null_mut::<xmlRelaxNG>();
            return -(1 as c_int);
        }
    } else {
        (*reader).rngValidCtxt = ctxt;
        (*reader).rngPreserveCtxt = 1 as c_int;
    }
    if (*reader).errorFunc.is_some() {
        xmlRelaxNGSetValidErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        ...
                    ) -> (),
            ),
            reader as *mut c_void,
        );
    }
    if (*reader).sErrorFunc.is_some() {
        xmlRelaxNGSetValidStructuredErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut c_void, *const xmlError) -> (),
            ),
            reader as *mut c_void,
        );
    }
    (*reader).rngValidErrors = 0 as c_int;
    (*reader).rngFullNode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).validate = XML_TEXTREADER_VALIDATE_RNG;
    return 0 as c_int;
}
unsafe fn xmlTextReaderSchemaValidateInternal(
    mut reader: xmlTextReaderPtr,
    mut xsd: *const c_char,
    mut ctxt: xmlSchemaValidCtxtPtr,
    mut options: c_int,
) -> c_int {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if !xsd.is_null() && !ctxt.is_null() {
        return -(1 as c_int);
    }
    if (!xsd.is_null() || !ctxt.is_null())
        && ((*reader).mode != XML_TEXTREADER_MODE_INITIAL as c_int
            || (*reader).ctxt.is_null())
    {
        return -(1 as c_int);
    }
    if !(*reader).xsdPlug.is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        (*reader).xsdPlug = ::core::ptr::null_mut::<xmlSchemaSAXPlugStruct>();
    }
    if !(*reader).xsdValidCtxt.is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        (*reader).xsdValidCtxt = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
    }
    (*reader).xsdPreserveCtxt = 0 as c_int;
    if !(*reader).xsdSchemas.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
    }
    if xsd.is_null() && ctxt.is_null() {
        return 0 as c_int;
    }
    if !xsd.is_null() {
        let mut pctxt: xmlSchemaParserCtxtPtr = ::core::ptr::null_mut::<xmlSchemaParserCtxt>();
        pctxt = xmlSchemaNewParserCtxt(xsd);
        if (*reader).errorFunc.is_some() {
            xmlSchemaSetParserErrors(
                pctxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            ...
                        ) -> (),
                ),
                reader as *mut c_void,
            );
        }
        (*reader).xsdSchemas = xmlSchemaParse(pctxt);
        xmlSchemaFreeParserCtxt(pctxt);
        if (*reader).xsdSchemas.is_null() {
            return -(1 as c_int);
        }
        (*reader).xsdValidCtxt = xmlSchemaNewValidCtxt((*reader).xsdSchemas);
        if (*reader).xsdValidCtxt.is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
            return -(1 as c_int);
        }
        (*reader).xsdPlug = xmlSchemaSAXPlug(
            (*reader).xsdValidCtxt,
            &raw mut (*(*reader).ctxt).sax,
            &raw mut (*(*reader).ctxt).userData,
        );
        if (*reader).xsdPlug.is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
            (*reader).xsdValidCtxt = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
            return -(1 as c_int);
        }
    } else {
        (*reader).xsdValidCtxt = ctxt;
        (*reader).xsdPreserveCtxt = 1 as c_int;
        (*reader).xsdPlug = xmlSchemaSAXPlug(
            (*reader).xsdValidCtxt,
            &raw mut (*(*reader).ctxt).sax,
            &raw mut (*(*reader).ctxt).userData,
        );
        if (*reader).xsdPlug.is_null() {
            (*reader).xsdValidCtxt = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
            (*reader).xsdPreserveCtxt = 0 as c_int;
            return -(1 as c_int);
        }
    }
    xmlSchemaValidateSetLocator(
        (*reader).xsdValidCtxt,
        Some(
            xmlTextReaderLocator
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut *const c_char,
                    *mut c_ulong,
                ) -> c_int,
        ),
        reader as *mut c_void,
    );
    if (*reader).errorFunc.is_some() {
        xmlSchemaSetValidErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *const c_char,
                        ...
                    ) -> (),
            ),
            reader as *mut c_void,
        );
    }
    if (*reader).sErrorFunc.is_some() {
        xmlSchemaSetValidStructuredErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut c_void, *const xmlError) -> (),
            ),
            reader as *mut c_void,
        );
    }
    (*reader).xsdValidErrors = 0 as c_int;
    (*reader).validate = XML_TEXTREADER_VALIDATE_XSD;
    return 0 as c_int;
}
#[inline]
pub fn xmlTextReaderSchemaValidateCtxt(
    mut reader: xmlTextReaderPtr,
    mut ctxt: xmlSchemaValidCtxtPtr,
    mut options: c_int,
) -> c_int { unsafe {
    return xmlTextReaderSchemaValidateInternal(
        reader,
        ::core::ptr::null::<c_char>(),
        ctxt,
        options,
    );
} }
#[inline]
pub unsafe fn xmlTextReaderSchemaValidate(
    mut reader: xmlTextReaderPtr,
    mut xsd: *const c_char,
) -> c_int {
    return xmlTextReaderSchemaValidateInternal(
        reader,
        xsd,
        ::core::ptr::null_mut::<xmlSchemaValidCtxt>(),
        0 as c_int,
    );
}
#[inline]
pub fn xmlTextReaderRelaxNGValidateCtxt(
    mut reader: xmlTextReaderPtr,
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut options: c_int,
) -> c_int { unsafe {
    return xmlTextReaderRelaxNGValidateInternal(
        reader,
        ::core::ptr::null::<c_char>(),
        ctxt,
        options,
    );
} }
#[inline]
pub unsafe fn xmlTextReaderRelaxNGValidate(
    mut reader: xmlTextReaderPtr,
    mut rng: *const c_char,
) -> c_int {
    return xmlTextReaderRelaxNGValidateInternal(
        reader,
        rng,
        ::core::ptr::null_mut::<xmlRelaxNGValidCtxt>(),
        0 as c_int,
    );
}
#[inline]
pub fn xmlTextReaderIsNamespaceDecl(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as c_int);
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if XML_NAMESPACE_DECL as c_int as c_uint
        == (*node).type_0 as c_uint
    {
        return 1 as c_int;
    } else {
        return 0 as c_int;
    };
} }
#[inline]
pub fn xmlTextReaderConstXmlVersion(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar { unsafe {
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if reader.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).doc.is_null() {
        doc = (*reader).doc;
    } else if !(*reader).ctxt.is_null() {
        doc = (*(*reader).ctxt).myDoc;
    }
    if doc.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if (*doc).version.is_null() {
        return ::core::ptr::null::<xmlChar>();
    } else {
        return xmlDictLookup((*reader).dict, (*doc).version, -(1 as c_int));
    };
} }
#[inline]
pub fn xmlTextReaderStandalone(
    mut reader: xmlTextReaderPtr,
) -> c_int { unsafe {
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if !(*reader).doc.is_null() {
        doc = (*reader).doc;
    } else if !(*reader).ctxt.is_null() {
        doc = (*(*reader).ctxt).myDoc;
    }
    if doc.is_null() {
        return -(1 as c_int);
    }
    return (*doc).standalone;
} }
unsafe fn xmlTextReaderBuildMessage(
    mut msg: *const c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut c_char {
    let mut size: c_int = 0 as c_int;
    let mut chars: c_int = 0;
    let mut larger: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut str: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut aq: ::core::ffi::VaListImpl;
    loop {
        aq = ap.clone();
        chars = vsnprintf(str, size as size_t, msg, aq.as_va_list());
        if chars < 0 as c_int {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"vsnprintf failed !\n\0" as *const u8 as *const c_char,
            );
            if !str.is_null() {
                xmlFree.expect("non-null function pointer")(str as *mut c_void);
            }
            return ::core::ptr::null_mut::<c_char>();
        }
        if chars < size || size == MAX_ERR_MSG_SIZE {
            break;
        }
        if chars < MAX_ERR_MSG_SIZE {
            size = chars + 1 as c_int;
        } else {
            size = MAX_ERR_MSG_SIZE;
        }
        larger = xmlRealloc.expect("non-null function pointer")(
            str as *mut c_void,
            size as size_t,
        ) as *mut c_char;
        if larger.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const c_char,
            );
            if !str.is_null() {
                xmlFree.expect("non-null function pointer")(str as *mut c_void);
            }
            return ::core::ptr::null_mut::<c_char>();
        }
        str = larger;
    }
    return str;
}
#[inline]
pub fn xmlTextReaderLocatorLineNumber(
    mut locator: xmlTextReaderLocatorPtr,
) -> c_int { unsafe {
    let mut ctx: xmlParserCtxtPtr = locator as xmlParserCtxtPtr;
    let mut ret: c_int = -(1 as c_int);
    if locator.is_null() {
        return -(1 as c_int);
    }
    if !(*ctx).node.is_null() {
        ret = xmlGetLineNo((*ctx).node as *const xmlNode) as c_int;
    } else {
        let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
        input = (*ctx).input;
        if (*input).filename.is_null() && (*ctx).inputNr > 1 as c_int {
            input = *(*ctx)
                .inputTab
                .offset(((*ctx).inputNr - 2 as c_int) as isize);
        }
        if !input.is_null() {
            ret = (*input).line;
        } else {
            ret = -(1 as c_int);
        }
    }
    return ret;
} }
#[inline]
pub fn xmlTextReaderLocatorBaseURI(
    mut locator: xmlTextReaderLocatorPtr,
) -> *mut xmlChar { unsafe {
    let mut ctx: xmlParserCtxtPtr = locator as xmlParserCtxtPtr;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if locator.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*ctx).node.is_null() {
        ret = xmlNodeGetBase(::core::ptr::null::<xmlDoc>(), (*ctx).node as *const xmlNode);
    } else {
        let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
        input = (*ctx).input;
        if (*input).filename.is_null() && (*ctx).inputNr > 1 as c_int {
            input = *(*ctx)
                .inputTab
                .offset(((*ctx).inputNr - 2 as c_int) as isize);
        }
        if !input.is_null() {
            ret = xmlStrdup((*input).filename as *mut xmlChar);
        } else {
            ret = ::core::ptr::null_mut::<xmlChar>();
        }
    }
    return ret;
} }
unsafe fn xmlTextReaderGenericError(
    mut ctxt: *mut c_void,
    mut severity: xmlParserSeverities,
    mut str: *mut c_char,
) {
    let mut ctx: xmlParserCtxtPtr = ctxt as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctx)._private as xmlTextReaderPtr;
    if !str.is_null() {
        if (*reader).errorFunc.is_some() {
            (*reader).errorFunc.expect("non-null function pointer")(
                (*reader).errorFuncArg,
                str,
                severity,
                ctx as xmlTextReaderLocatorPtr,
            );
        }
        xmlFree.expect("non-null function pointer")(str as *mut c_void);
    }
}
unsafe extern "C" fn xmlTextReaderStructuredError(
    mut ctxt: *mut c_void,
    mut error: *const xmlError,
) {
    let mut ctx: xmlParserCtxtPtr = ctxt as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctx)._private as xmlTextReaderPtr;
    if !error.is_null() && (*reader).sErrorFunc.is_some() {
        (*reader).sErrorFunc.expect("non-null function pointer")(
            (*reader).errorFuncArg,
            error as xmlErrorPtr as *const xmlError,
        );
    }
}
unsafe extern "C" fn xmlTextReaderError(
    mut ctxt: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    xmlTextReaderGenericError(
        ctxt,
        XML_PARSER_SEVERITY_ERROR,
        xmlTextReaderBuildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn xmlTextReaderWarning(
    mut ctxt: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    xmlTextReaderGenericError(
        ctxt,
        XML_PARSER_SEVERITY_WARNING,
        xmlTextReaderBuildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn xmlTextReaderValidityError(
    mut ctxt: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut len: c_int = xmlStrlen(msg as *const xmlChar);
    if len > 1 as c_int
        && *msg.offset((len - 2 as c_int) as isize) as c_int != ':' as i32
    {
        ap = args.clone();
        xmlTextReaderGenericError(
            ctxt,
            XML_PARSER_SEVERITY_VALIDITY_ERROR,
            xmlTextReaderBuildMessage(msg, ap.as_va_list()),
        );
    }
}
unsafe extern "C" fn xmlTextReaderValidityWarning(
    mut ctxt: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut len: c_int = xmlStrlen(msg as *const xmlChar);
    if len != 0 as c_int
        && *msg.offset((len - 1 as c_int) as isize) as c_int != ':' as i32
    {
        ap = args.clone();
        xmlTextReaderGenericError(
            ctxt,
            XML_PARSER_SEVERITY_VALIDITY_WARNING,
            xmlTextReaderBuildMessage(msg, ap.as_va_list()),
        );
    }
}
#[inline]
pub unsafe fn xmlTextReaderSetErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: xmlTextReaderErrorFunc,
    mut arg: *mut c_void,
) {
    if f.is_some() {
        (*(*(*reader).ctxt).sax).error = Some(
            xmlTextReaderError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as errorSAXFunc;
        (*(*(*reader).ctxt).sax).serror = None;
        (*(*reader).ctxt).vctxt.error = Some(
            xmlTextReaderValidityError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*(*(*reader).ctxt).sax).warning = Some(
            xmlTextReaderWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*(*reader).ctxt).vctxt.warning = Some(
            xmlTextReaderValidityWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityWarningFunc;
        (*reader).errorFunc = f;
        (*reader).sErrorFunc = None;
        (*reader).errorFuncArg = arg;
        if !(*reader).rngValidCtxt.is_null() {
            xmlRelaxNGSetValidErrors(
                (*reader).rngValidCtxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            ...
                        ) -> (),
                ),
                reader as *mut c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut c_void,
            );
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_char,
                            ...
                        ) -> (),
                ),
                reader as *mut c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut c_void,
            );
        }
    } else {
        (*(*(*reader).ctxt).sax).error = Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as errorSAXFunc;
        (*(*reader).ctxt).vctxt.error = Some(
            xmlParserValidityError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*(*(*reader).ctxt).sax).warning = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*(*reader).ctxt).vctxt.warning = Some(
            xmlParserValidityWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityWarningFunc;
        (*reader).errorFunc = None;
        (*reader).sErrorFunc = None;
        (*reader).errorFuncArg = NULL;
        if !(*reader).rngValidCtxt.is_null() {
            xmlRelaxNGSetValidErrors(
                (*reader).rngValidCtxt,
                None,
                None,
                reader as *mut c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut c_void,
            );
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut c_void,
            );
        }
    };
}
#[inline]
pub unsafe fn xmlTextReaderSetStructuredErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: xmlStructuredErrorFunc,
    mut arg: *mut c_void,
) {
    if f.is_some() {
        (*(*(*reader).ctxt).sax).error = None;
        (*(*(*reader).ctxt).sax).serror = Some(
            xmlTextReaderStructuredError
                as unsafe extern "C" fn(*mut c_void, *const xmlError) -> (),
        ) as xmlStructuredErrorFunc;
        (*(*reader).ctxt).vctxt.error = Some(
            xmlTextReaderValidityError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*(*(*reader).ctxt).sax).warning = Some(
            xmlTextReaderWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*(*reader).ctxt).vctxt.warning = Some(
            xmlTextReaderValidityWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityWarningFunc;
        (*reader).sErrorFunc = f;
        (*reader).errorFunc = None;
        (*reader).errorFuncArg = arg;
        if !(*reader).rngValidCtxt.is_null() {
            xmlRelaxNGSetValidErrors(
                (*reader).rngValidCtxt,
                None,
                None,
                reader as *mut c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut c_void, *const xmlError) -> (),
                ),
                reader as *mut c_void,
            );
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut c_void, *const xmlError) -> (),
                ),
                reader as *mut c_void,
            );
        }
    } else {
        (*(*(*reader).ctxt).sax).error = Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as errorSAXFunc;
        (*(*(*reader).ctxt).sax).serror = None;
        (*(*reader).ctxt).vctxt.error = Some(
            xmlParserValidityError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*(*(*reader).ctxt).sax).warning = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*(*reader).ctxt).vctxt.warning = Some(
            xmlParserValidityWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityWarningFunc;
        (*reader).errorFunc = None;
        (*reader).sErrorFunc = None;
        (*reader).errorFuncArg = NULL;
        if !(*reader).rngValidCtxt.is_null() {
            xmlRelaxNGSetValidErrors(
                (*reader).rngValidCtxt,
                None,
                None,
                reader as *mut c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut c_void,
            );
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut c_void,
            );
        }
    };
}
#[inline]
pub fn xmlTextReaderIsValid(mut reader: xmlTextReaderPtr) -> c_int { unsafe {
    if reader.is_null() {
        return -(1 as c_int);
    }
    if (*reader).validate as c_uint
        == XML_TEXTREADER_VALIDATE_RNG as c_int as c_uint
    {
        return ((*reader).rngValidErrors == 0 as c_int) as c_int;
    }
    if (*reader).validate as c_uint
        == XML_TEXTREADER_VALIDATE_XSD as c_int as c_uint
    {
        return ((*reader).xsdValidErrors == 0 as c_int) as c_int;
    }
    if !(*reader).ctxt.is_null() && (*(*reader).ctxt).validate == 1 as c_int {
        return (*(*reader).ctxt).valid;
    }
    return 0 as c_int;
} }
#[inline]
pub unsafe fn xmlTextReaderGetErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: *mut xmlTextReaderErrorFunc,
    mut arg: *mut *mut c_void,
) {
    if !f.is_null() {
        *f = (*reader).errorFunc;
    }
    if !arg.is_null() {
        *arg = (*reader).errorFuncArg;
    }
}
#[inline]
pub unsafe fn xmlTextReaderSetup(
    mut reader: xmlTextReaderPtr,
    mut input: xmlParserInputBufferPtr,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> c_int {
    if reader.is_null() {
        if !input.is_null() {
            xmlFreeParserInputBuffer(input);
        }
        return -(1 as c_int);
    }
    options |= XML_PARSE_COMPACT as c_int;
    (*reader).doc = ::core::ptr::null_mut::<xmlDoc>();
    (*reader).entNr = 0 as c_int;
    (*reader).parserFlags = options;
    (*reader).validate = XML_TEXTREADER_NOT_VALIDATE;
    if !input.is_null()
        && !(*reader).input.is_null()
        && (*reader).allocs & XML_TEXTREADER_INPUT != 0
    {
        xmlFreeParserInputBuffer((*reader).input);
        (*reader).input = ::core::ptr::null_mut::<xmlParserInputBuffer>();
        (*reader).allocs -= XML_TEXTREADER_INPUT;
    }
    if !input.is_null() {
        (*reader).input = input;
        (*reader).allocs |= XML_TEXTREADER_INPUT;
    }
    if (*reader).buffer.is_null() {
        (*reader).buffer = xmlBufCreateSize(100 as size_t);
    }
    if (*reader).buffer.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlTextReaderSetup : malloc failed\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    xmlBufSetAllocationScheme((*reader).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    if (*reader).sax.is_null() {
        (*reader).sax = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<
            xmlSAXHandler,
        >() as size_t) as *mut xmlSAXHandler as xmlSAXHandlerPtr;
    }
    if (*reader).sax.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlTextReaderSetup : malloc failed\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    xmlSAXVersion((*reader).sax as *mut xmlSAXHandler, 2 as c_int);
    (*reader).startElement = (*(*reader).sax).startElement;
    (*(*reader).sax).startElement = Some(
        xmlTextReaderStartElement
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*reader).endElement = (*(*reader).sax).endElement;
    (*(*reader).sax).endElement = Some(
        xmlTextReaderEndElement
            as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    if (*(*reader).sax).initialized == XML_SAX2_MAGIC {
        (*reader).startElementNs = (*(*reader).sax).startElementNs;
        (*(*reader).sax).startElementNs = Some(
            xmlTextReaderStartElementNs
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    c_int,
                    *mut *const xmlChar,
                    c_int,
                    c_int,
                    *mut *const xmlChar,
                ) -> (),
        ) as startElementNsSAX2Func;
        (*reader).endElementNs = (*(*reader).sax).endElementNs;
        (*(*reader).sax).endElementNs = Some(
            xmlTextReaderEndElementNs
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ) as endElementNsSAX2Func;
    } else {
        (*reader).startElementNs = None;
        (*reader).endElementNs = None;
    }
    (*reader).characters = (*(*reader).sax).characters;
    (*(*reader).sax).characters = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*(*reader).sax).ignorableWhitespace = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*reader).cdataBlock = (*(*reader).sax).cdataBlock;
    (*(*reader).sax).cdataBlock = Some(
        xmlTextReaderCDataBlock
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as cdataBlockSAXFunc;
    (*reader).mode = XML_TEXTREADER_MODE_INITIAL as c_int;
    (*reader).node = ::core::ptr::null_mut::<xmlNode>();
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    if !input.is_null() {
        if xmlBufUse((*(*reader).input).buffer) < 4 as size_t {
            xmlParserInputBufferRead(input, 4 as c_int);
        }
        if (*reader).ctxt.is_null() {
            if xmlBufUse((*(*reader).input).buffer) >= 4 as size_t {
                (*reader).ctxt = xmlCreatePushParserCtxt(
                    (*reader).sax,
                    NULL,
                    xmlBufContent((*(*reader).input).buffer as *const xmlBuf)
                        as *const c_char,
                    4 as c_int,
                    URL,
                );
                (*reader).base = 0 as c_uint;
                (*reader).cur = 4 as c_uint;
            } else {
                (*reader).ctxt = xmlCreatePushParserCtxt(
                    (*reader).sax,
                    NULL,
                    ::core::ptr::null::<c_char>(),
                    0 as c_int,
                    URL,
                );
                (*reader).base = 0 as c_uint;
                (*reader).cur = 0 as c_uint;
            }
        } else {
            let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
            let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
            let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
            xmlCtxtReset((*reader).ctxt);
            buf = xmlAllocParserInputBuffer(enc);
            if buf.is_null() {
                return -(1 as c_int);
            }
            inputStream = xmlNewInputStream((*reader).ctxt);
            if inputStream.is_null() {
                xmlFreeParserInputBuffer(buf);
                return -(1 as c_int);
            }
            if URL.is_null() {
                (*inputStream).filename = ::core::ptr::null::<c_char>();
            } else {
                (*inputStream).filename =
                    xmlCanonicPath(URL as *const xmlChar) as *mut c_char;
            }
            (*inputStream).buf = buf;
            xmlBufResetInput((*buf).buffer, inputStream);
            inputPush((*reader).ctxt, inputStream);
            (*reader).cur = 0 as c_uint;
        }
        if (*reader).ctxt.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlTextReaderSetup : malloc failed\n\0" as *const u8
                    as *const c_char,
            );
            return -(1 as c_int);
        }
    }
    if !(*reader).dict.is_null() {
        if !(*(*reader).ctxt).dict.is_null() {
            if (*reader).dict != (*(*reader).ctxt).dict {
                xmlDictFree((*reader).dict);
                (*reader).dict = (*(*reader).ctxt).dict;
            }
        } else {
            (*(*reader).ctxt).dict = (*reader).dict;
        }
    } else {
        if (*(*reader).ctxt).dict.is_null() {
            (*(*reader).ctxt).dict = xmlDictCreate();
        }
        (*reader).dict = (*(*reader).ctxt).dict;
    }
    (*(*reader).ctxt)._private = reader as *mut c_void;
    (*(*reader).ctxt).linenumbers = 1 as c_int;
    (*(*reader).ctxt).dictNames = 1 as c_int;
    (*(*reader).ctxt).docdict = 1 as c_int;
    (*(*reader).ctxt).parseMode = XML_PARSE_READER;
    if !(*reader).xincctxt.is_null() {
        xmlXIncludeFreeContext((*reader).xincctxt);
        (*reader).xincctxt = ::core::ptr::null_mut::<xmlXIncludeCtxt>();
    }
    if options & XML_PARSE_XINCLUDE as c_int != 0 {
        (*reader).xinclude = 1 as c_int;
        (*reader).xinclude_name =
            xmlDictLookup((*reader).dict, XINCLUDE_NODE, -(1 as c_int));
        options -= XML_PARSE_XINCLUDE as c_int;
    } else {
        (*reader).xinclude = 0 as c_int;
    }
    (*reader).in_xinclude = 0 as c_int;
    if (*reader).patternTab.is_null() {
        (*reader).patternNr = 0 as c_int;
        (*reader).patternMax = 0 as c_int;
    }
    while (*reader).patternNr > 0 as c_int {
        (*reader).patternNr -= 1;
        if !(*(*reader).patternTab.offset((*reader).patternNr as isize)).is_null() {
            xmlFreePattern(*(*reader).patternTab.offset((*reader).patternNr as isize));
            let ref mut fresh0 = *(*reader).patternTab.offset((*reader).patternNr as isize);
            *fresh0 = ::core::ptr::null_mut::<xmlPattern>();
        }
    }
    if options & XML_PARSE_DTDVALID as c_int != 0 {
        (*reader).validate = XML_TEXTREADER_VALIDATE_DTD;
    }
    xmlCtxtUseOptions((*reader).ctxt, options);
    if !encoding.is_null() {
        let mut hdlr: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
        hdlr = xmlFindCharEncodingHandler(encoding);
        if !hdlr.is_null() {
            xmlSwitchToEncoding((*reader).ctxt, hdlr);
        }
    }
    if !URL.is_null()
        && !(*(*reader).ctxt).input.is_null()
        && (*(*(*reader).ctxt).input).filename.is_null()
    {
        (*(*(*reader).ctxt).input).filename =
            xmlStrdup(URL as *const xmlChar) as *mut c_char;
    }
    (*reader).doc = ::core::ptr::null_mut::<xmlDoc>();
    return 0 as c_int;
}
#[inline]
pub fn xmlTextReaderSetMaxAmplification(
    mut reader: xmlTextReaderPtr,
    mut maxAmpl: c_uint,
) { unsafe {
    xmlCtxtSetMaxAmplification((*reader).ctxt, maxAmpl);
} }
#[inline]
pub fn xmlTextReaderByteConsumed(
    mut reader: xmlTextReaderPtr,
) -> c_long { unsafe {
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as c_int) as c_long;
    }
    return xmlByteConsumed((*reader).ctxt);
} }
#[inline]
pub fn xmlReaderWalker(mut doc: xmlDocPtr) -> xmlTextReaderPtr { unsafe {
    let mut ret: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlTextReader>() as size_t
    ) as xmlTextReaderPtr;
    if ret.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlTextReader>() as size_t,
    );
    (*ret).entNr = 0 as c_int;
    (*ret).input = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    (*ret).mode = XML_TEXTREADER_MODE_INITIAL as c_int;
    (*ret).node = ::core::ptr::null_mut::<xmlNode>();
    (*ret).curnode = ::core::ptr::null_mut::<xmlNode>();
    (*ret).base = 0 as c_uint;
    (*ret).cur = 0 as c_uint;
    (*ret).allocs = XML_TEXTREADER_CTXT;
    (*ret).doc = doc;
    (*ret).state = XML_TEXTREADER_START;
    (*ret).dict = xmlDictCreate();
    return ret;
} }
#[inline]
pub unsafe fn xmlReaderForDoc(
    mut cur: *const xmlChar,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> xmlTextReaderPtr {
    let mut len: c_int = 0;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    len = xmlStrlen(cur);
    return xmlReaderForMemory(
        cur as *const c_char,
        len,
        URL,
        encoding,
        options,
    );
}
#[inline]
pub unsafe fn xmlReaderForFile(
    mut filename: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    reader = xmlNewTextReaderFilename(filename);
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    xmlTextReaderSetup(
        reader,
        ::core::ptr::null_mut::<xmlParserInputBuffer>(),
        ::core::ptr::null::<c_char>(),
        encoding,
        options,
    );
    return reader;
}
#[inline]
pub unsafe fn xmlReaderForMemory(
    mut buffer: *const c_char,
    mut size: c_int,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    buf = xmlParserInputBufferCreateMem(buffer, size, XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    reader = xmlNewTextReader(buf, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(buf);
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    (*reader).allocs |= XML_TEXTREADER_INPUT;
    xmlTextReaderSetup(
        reader,
        ::core::ptr::null_mut::<xmlParserInputBuffer>(),
        URL,
        encoding,
        options,
    );
    return reader;
}
#[inline]
pub unsafe fn xmlReaderForFd(
    mut fd: c_int,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if fd < 0 as c_int {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    (*input).closecallback = None;
    reader = xmlNewTextReader(input, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    (*reader).allocs |= XML_TEXTREADER_INPUT;
    xmlTextReaderSetup(
        reader,
        ::core::ptr::null_mut::<xmlParserInputBuffer>(),
        URL,
        encoding,
        options,
    );
    return reader;
}
#[inline]
pub unsafe fn xmlReaderForIO(
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut c_void,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if ioread.is_none() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    input = xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    reader = xmlNewTextReader(input, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(input);
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    (*reader).allocs |= XML_TEXTREADER_INPUT;
    xmlTextReaderSetup(
        reader,
        ::core::ptr::null_mut::<xmlParserInputBuffer>(),
        URL,
        encoding,
        options,
    );
    return reader;
}
#[inline]
pub fn xmlReaderNewWalker(
    mut reader: xmlTextReaderPtr,
    mut doc: xmlDocPtr,
) -> c_int { unsafe {
    if doc.is_null() {
        return -(1 as c_int);
    }
    if reader.is_null() {
        return -(1 as c_int);
    }
    if !(*reader).input.is_null() {
        xmlFreeParserInputBuffer((*reader).input);
    }
    if !(*reader).ctxt.is_null() {
        xmlCtxtReset((*reader).ctxt);
    }
    (*reader).entNr = 0 as c_int;
    (*reader).input = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    (*reader).mode = XML_TEXTREADER_MODE_INITIAL as c_int;
    (*reader).node = ::core::ptr::null_mut::<xmlNode>();
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).base = 0 as c_uint;
    (*reader).cur = 0 as c_uint;
    (*reader).allocs = XML_TEXTREADER_CTXT;
    (*reader).doc = doc;
    (*reader).state = XML_TEXTREADER_START;
    if (*reader).dict.is_null() {
        if !(*reader).ctxt.is_null() && !(*(*reader).ctxt).dict.is_null() {
            (*reader).dict = (*(*reader).ctxt).dict;
        } else {
            (*reader).dict = xmlDictCreate();
        }
    }
    return 0 as c_int;
} }
#[inline]
pub unsafe fn xmlReaderNewDoc(
    mut reader: xmlTextReaderPtr,
    mut cur: *const xmlChar,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> c_int {
    let mut len: c_int = 0;
    if cur.is_null() {
        return -(1 as c_int);
    }
    if reader.is_null() {
        return -(1 as c_int);
    }
    len = xmlStrlen(cur);
    return xmlReaderNewMemory(
        reader,
        cur as *const c_char,
        len,
        URL,
        encoding,
        options,
    );
}
#[inline]
pub unsafe fn xmlReaderNewFile(
    mut reader: xmlTextReaderPtr,
    mut filename: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> c_int {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if filename.is_null() {
        return -(1 as c_int);
    }
    if reader.is_null() {
        return -(1 as c_int);
    }
    input = xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as c_int);
    }
    return xmlTextReaderSetup(reader, input, filename, encoding, options);
}
#[inline]
pub unsafe fn xmlReaderNewMemory(
    mut reader: xmlTextReaderPtr,
    mut buffer: *const c_char,
    mut size: c_int,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> c_int {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if reader.is_null() {
        return -(1 as c_int);
    }
    if buffer.is_null() {
        return -(1 as c_int);
    }
    input = xmlParserInputBufferCreateMem(buffer, size, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as c_int);
    }
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
#[inline]
pub unsafe fn xmlReaderNewFd(
    mut reader: xmlTextReaderPtr,
    mut fd: c_int,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> c_int {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if fd < 0 as c_int {
        return -(1 as c_int);
    }
    if reader.is_null() {
        return -(1 as c_int);
    }
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as c_int);
    }
    (*input).closecallback = None;
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
#[inline]
pub unsafe fn xmlReaderNewIO(
    mut reader: xmlTextReaderPtr,
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut c_void,
    mut URL: *const c_char,
    mut encoding: *const c_char,
    mut options: c_int,
) -> c_int {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if ioread.is_none() {
        return -(1 as c_int);
    }
    if reader.is_null() {
        return -(1 as c_int);
    }
    input = xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return -(1 as c_int);
    }
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
pub const XINCLUDE_NS: *const xmlChar = b"http://www.w3.org/2003/XInclude\0" as *const u8
    as *const c_char as *const xmlChar;
pub const XINCLUDE_OLD_NS: *const xmlChar = b"http://www.w3.org/2001/XInclude\0" as *const u8
    as *const c_char as *const xmlChar;
pub const XINCLUDE_NODE: *const xmlChar =
    b"include\0" as *const u8 as *const c_char as *const xmlChar;
