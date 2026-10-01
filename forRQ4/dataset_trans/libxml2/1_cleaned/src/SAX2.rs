use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type _xmlRegexp;
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlAttrHashBucket;
    pub type _xmlParserNsData;
    pub type _xmlHashTable;
    pub type _xmlStartTag;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    fn xmlAddNotationDecl(
        ctxt: xmlValidCtxtPtr,
        dtd: xmlDtdPtr,
        name: *const xmlChar,
        PublicID: *const xmlChar,
        SystemID: *const xmlChar,
    ) -> xmlNotationPtr;
    fn xmlAddElementDecl(
        ctxt: xmlValidCtxtPtr,
        dtd: xmlDtdPtr,
        name: *const xmlChar,
        type_0: xmlElementTypeVal,
        content: xmlElementContentPtr,
    ) -> xmlElementPtr;
    fn xmlAddAttributeDecl(
        ctxt: xmlValidCtxtPtr,
        dtd: xmlDtdPtr,
        elem: *const xmlChar,
        name: *const xmlChar,
        ns: *const xmlChar,
        type_0: xmlAttributeType,
        def: xmlAttributeDefault,
        defaultValue: *const xmlChar,
        tree: xmlEnumerationPtr,
    ) -> xmlAttributePtr;
    fn xmlAddID(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        value: *const xmlChar,
        attr: xmlAttrPtr,
    ) -> xmlIDPtr;
    fn xmlIsID(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr) -> c_int;
    fn xmlAddRef(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        value: *const xmlChar,
        attr: xmlAttrPtr,
    ) -> xmlRefPtr;
    fn xmlIsRef(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr) -> c_int;
    fn xmlValidateRoot(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn xmlValidateElementDecl(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlElementPtr,
    ) -> c_int;
    fn xmlValidNormalizeAttributeValue(
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlValidCtxtNormalizeAttributeValue(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlValidateAttributeDecl(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        attr: xmlAttributePtr,
    ) -> c_int;
    fn xmlValidateNotationDecl(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        nota: xmlNotationPtr,
    ) -> c_int;
    fn xmlValidateDtdFinal(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn xmlValidateOneElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> c_int;
    fn xmlValidateOneAttribute(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        attr: xmlAttrPtr,
        value: *const xmlChar,
    ) -> c_int;
    fn xmlValidateOneNamespace(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        prefix: *const xmlChar,
        ns: xmlNsPtr,
        value: *const xmlChar,
    ) -> c_int;
    fn xmlValidateDocumentFinal(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn xmlGetDtdQAttrDesc(
        dtd: xmlDtdPtr,
        elem: *const xmlChar,
        name: *const xmlChar,
        prefix: *const xmlChar,
    ) -> xmlAttributePtr;
    fn xmlGetDtdQElementDesc(
        dtd: xmlDtdPtr,
        name: *const xmlChar,
        prefix: *const xmlChar,
    ) -> xmlElementPtr;
    fn xmlDictReference(dict: xmlDictPtr) -> c_int;
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: c_int,
    ) -> *const xmlChar;
    fn xmlDictQLookup(
        dict: xmlDictPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
    ) -> *const xmlChar;
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> c_int;
    fn xmlLoadExternalEntity(
        URL: *const c_char,
        ID: *const c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
    fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc;
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
    fn xmlGetIntSubset(doc: *const xmlDoc) -> xmlDtdPtr;
    fn xmlFreeDtd(cur: xmlDtdPtr);
    fn xmlNewNs(node: xmlNodePtr, href: *const xmlChar, prefix: *const xmlChar) -> xmlNsPtr;
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    fn xmlNewNsPropEatName(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *mut xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewDocNodeEatName(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *mut xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewDocPI(doc: xmlDocPtr, name: *const xmlChar, content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewDocComment(doc: xmlDocPtr, content: *const xmlChar) -> xmlNodePtr;
    fn xmlNewCDataBlock(
        doc: xmlDocPtr,
        content: *const xmlChar,
        len: c_int,
    ) -> xmlNodePtr;
    fn xmlNewReference(doc: *const xmlDoc, name: *const xmlChar) -> xmlNodePtr;
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlTextConcat(
        node: xmlNodePtr,
        content: *const xmlChar,
        len: c_int,
    ) -> c_int;
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr, nameSpace: *const xmlChar) -> xmlNsPtr;
    fn xmlSetNs(node: xmlNodePtr, ns: xmlNsPtr);
    fn xmlStringGetNodeList(doc: *const xmlDoc, value: *const xmlChar) -> xmlNodePtr;
    fn xmlStringLenGetNodeList(
        doc: *const xmlDoc,
        value: *const xmlChar,
        len: c_int,
    ) -> xmlNodePtr;
    fn xmlAddDocEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlAddDtdEntity(
        doc: xmlDocPtr,
        name: *const xmlChar,
        type_0: c_int,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlGetPredefinedEntity(name: *const xmlChar) -> xmlEntityPtr;
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlGetParameterEntity(doc: xmlDocPtr, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlPushInput(ctxt: xmlParserCtxtPtr, input: xmlParserInputPtr) -> c_int;
    fn xmlPopInput(ctxt: xmlParserCtxtPtr) -> xmlChar;
    fn xmlFreeInputStream(input: xmlParserInputPtr);
    fn xmlSplitQName(
        ctxt: xmlParserCtxtPtr,
        name: *const xmlChar,
        prefix: *mut *mut xmlChar,
    ) -> *mut xmlChar;
    fn xmlParseExternalSubset(
        ctxt: xmlParserCtxtPtr,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlStringDecodeEntities(
        ctxt: xmlParserCtxtPtr,
        str: *const xmlChar,
        what: c_int,
        end: xmlChar,
        end2: xmlChar,
        end3: xmlChar,
    ) -> *mut xmlChar;
    fn xmlStringLenDecodeEntities(
        ctxt: xmlParserCtxtPtr,
        str: *const xmlChar,
        len: c_int,
        what: c_int,
        end: xmlChar,
        end2: xmlChar,
        end3: xmlChar,
    ) -> *mut xmlChar;
    fn nodePush(ctxt: xmlParserCtxtPtr, value: xmlNodePtr) -> c_int;
    fn nodePop(ctxt: xmlParserCtxtPtr) -> xmlNodePtr;
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn htmlNewDocNoDtD(URI: *const xmlChar, ExternalID: *const xmlChar) -> htmlDocPtr;
    fn xmlErrMemory(ctxt: xmlParserCtxtPtr, extra: *const c_char);
    fn xmlParserNsUpdateSax(
        ctxt: xmlParserCtxtPtr,
        prefix: *const xmlChar,
        saxData: *mut c_void,
    ) -> c_int;
    fn xmlParserNsLookupSax(
        ctxt: xmlParserCtxtPtr,
        prefix: *const xmlChar,
    ) -> *mut c_void;
}

pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
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

pub const XML_ATTRIBUTE_FIXED: xmlAttributeDefault = 4;
pub const XML_ATTRIBUTE_IMPLIED: xmlAttributeDefault = 3;
pub const XML_ATTRIBUTE_REQUIRED: xmlAttributeDefault = 2;
pub const XML_ATTRIBUTE_NONE: xmlAttributeDefault = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttribute {
    pub _private: *mut c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub nexth: *mut _xmlAttribute,
    pub atype: xmlAttributeType,
    pub def: xmlAttributeDefault,
    pub defaultValue: *const xmlChar,
    pub tree: xmlEnumerationPtr,
    pub prefix: *const xmlChar,
    pub elem: *const xmlChar,
}
pub type xmlAttribute = _xmlAttribute;
pub type xmlAttributePtr = *mut xmlAttribute;

pub const XML_ELEMENT_TYPE_ELEMENT: xmlElementTypeVal = 4;
pub const XML_ELEMENT_TYPE_MIXED: xmlElementTypeVal = 3;
pub const XML_ELEMENT_TYPE_ANY: xmlElementTypeVal = 2;
pub const XML_ELEMENT_TYPE_EMPTY: xmlElementTypeVal = 1;
pub const XML_ELEMENT_TYPE_UNDEFINED: xmlElementTypeVal = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElement {
    pub _private: *mut c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub etype: xmlElementTypeVal,
    pub content: xmlElementContentPtr,
    pub attributes: xmlAttributePtr,
    pub prefix: *const xmlChar,
    pub contModel: xmlRegexpPtr,
}
pub type xmlElement = _xmlElement;
pub type xmlElementPtr = *mut xmlElement;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlID {
    pub next: *mut _xmlID,
    pub value: *const xmlChar,
    pub attr: xmlAttrPtr,
    pub name: *const xmlChar,
    pub lineno: c_int,
    pub doc: *mut _xmlDoc,
}
pub type xmlID = _xmlID;
pub type xmlIDPtr = *mut xmlID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRef {
    pub next: *mut _xmlRef,
    pub value: *const xmlChar,
    pub attr: xmlAttrPtr,
    pub name: *const xmlChar,
    pub lineno: c_int,
}
pub type xmlRef = _xmlRef;
pub type xmlRefPtr = *mut xmlRef;
pub type C2RustUnnamed = c_uint;
pub const XML_DOC_HTML: C2RustUnnamed = 128;
pub const XML_DOC_INTERNAL: C2RustUnnamed = 64;
pub const XML_DOC_USERBUILT: C2RustUnnamed = 32;
pub const XML_DOC_XINCLUDE: C2RustUnnamed = 16;
pub const XML_DOC_DTDVALID: C2RustUnnamed = 8;
pub const XML_DOC_OLD10: C2RustUnnamed = 4;
pub const XML_DOC_NSVALID: C2RustUnnamed = 2;
pub const XML_DOC_WELLFORMED: C2RustUnnamed = 1;
pub type xmlRegisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type C2RustUnnamed_0 = c_uint;
pub const XML_FROM_URI: C2RustUnnamed_0 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_0 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_0 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_0 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_0 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_0 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_0 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_0 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_0 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_0 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_0 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_0 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_0 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_0 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_0 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_0 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_0 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_0 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_0 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_0 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_0 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_0 = 9;
pub const XML_FROM_IO: C2RustUnnamed_0 = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed_0 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_0 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_0 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_0 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_0 = 3;
pub const XML_FROM_TREE: C2RustUnnamed_0 = 2;
pub const XML_FROM_PARSER: C2RustUnnamed_0 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_0 = 0;

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

pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type C2RustUnnamed_1 = c_uint;
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
pub type htmlDocPtr = xmlDocPtr;

pub const USHRT_MAX: c_int =
    __SHRT_MAX__ * 2 as c_int + 1 as c_int;
pub const ULONG_MAX: c_ulong = (__LONG_MAX__ as c_ulong)
    .wrapping_mul(2 as c_ulong)
    .wrapping_add(1 as c_ulong);

unsafe extern "C" fn xmlSAX2ErrMemory(
    mut ctxt: xmlParserCtxtPtr,
    mut msg: *const c_char,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut str1: *const c_char =
        b"out of memory\n\0" as *const u8 as *const c_char;
    if !ctxt.is_null() {
        (*ctxt).errNo = XML_ERR_NO_MEMORY as c_int;
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).initialized == XML_SAX2_MAGIC {
            schannel = (*(*ctxt).sax).serror;
        }
        __xmlRaiseError(
            schannel,
            (*ctxt).vctxt.error as xmlGenericErrorFunc,
            (*ctxt).vctxt.userData,
            ctxt as *mut c_void,
            NULL,
            XML_FROM_PARSER as c_int,
            XML_ERR_NO_MEMORY as c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            str1,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            msg,
            str1,
            NULL,
        );
        (*ctxt).errNo = XML_ERR_NO_MEMORY as c_int;
        (*ctxt).instate = XML_PARSER_EOF;
        (*ctxt).disableSAX = 1 as c_int;
    } else {
        __xmlRaiseError(
            schannel,
            None,
            NULL,
            ctxt as *mut c_void,
            NULL,
            XML_FROM_PARSER as c_int,
            XML_ERR_NO_MEMORY as c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            str1,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            msg,
            str1,
            NULL,
        );
    };
}
unsafe extern "C" fn xmlErrValid(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut str1: *const c_char,
    mut str2: *const c_char,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as c_int
        && (*ctxt).instate as c_int == XML_PARSER_EOF as c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as c_int;
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).initialized == XML_SAX2_MAGIC {
            schannel = (*(*ctxt).sax).serror;
        }
        __xmlRaiseError(
            schannel,
            (*ctxt).vctxt.error as xmlGenericErrorFunc,
            (*ctxt).vctxt.userData,
            ctxt as *mut c_void,
            NULL,
            XML_FROM_DTD as c_int,
            error as c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            str1,
            str2,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            msg,
            str1,
            str2,
        );
        (*ctxt).valid = 0 as c_int;
    } else {
        __xmlRaiseError(
            schannel,
            None,
            NULL,
            ctxt as *mut c_void,
            NULL,
            XML_FROM_DTD as c_int,
            error as c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            str1,
            str2,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            msg,
            str1,
            str2,
        );
    };
}
unsafe extern "C" fn xmlFatalErrMsg(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as c_int
        && (*ctxt).instate as c_int == XML_PARSER_EOF as c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut c_void,
        NULL,
        XML_FROM_PARSER as c_int,
        error as c_int,
        XML_ERR_FATAL,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        str1 as *const c_char,
        str2 as *const c_char,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        msg,
        str1,
        str2,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as c_int;
        (*ctxt).valid = 0 as c_int;
        if (*ctxt).recovery == 0 as c_int {
            (*ctxt).disableSAX = 1 as c_int;
        }
    }
}
unsafe extern "C" fn xmlWarnMsg(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut str1: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as c_int
        && (*ctxt).instate as c_int == XML_PARSER_EOF as c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut c_void,
        NULL,
        XML_FROM_PARSER as c_int,
        error as c_int,
        XML_ERR_WARNING,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        str1 as *const c_char,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        msg,
        str1,
    );
}
unsafe extern "C" fn xmlNsWarnMsg(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as c_int
        && (*ctxt).instate as c_int == XML_PARSER_EOF as c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut c_void,
        NULL,
        XML_FROM_NAMESPACE as c_int,
        error as c_int,
        XML_ERR_WARNING,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        str1 as *const c_char,
        str2 as *const c_char,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        msg,
        str1,
        str2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetPublicId(mut ctx: *mut c_void) -> *const xmlChar {
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetSystemId(mut ctx: *mut c_void) -> *const xmlChar {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || (*ctxt).input.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return (*(*ctxt).input).filename as *const xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetLineNumber(
    mut ctx: *mut c_void,
) -> c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || (*ctxt).input.is_null() {
        return 0 as c_int;
    }
    return (*(*ctxt).input).line;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetColumnNumber(
    mut ctx: *mut c_void,
) -> c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || (*ctxt).input.is_null() {
        return 0 as c_int;
    }
    return (*(*ctxt).input).col;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2IsStandalone(
    mut ctx: *mut c_void,
) -> c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || (*ctxt).myDoc.is_null() {
        return 0 as c_int;
    }
    return ((*(*ctxt).myDoc).standalone == 1 as c_int) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2HasInternalSubset(
    mut ctx: *mut c_void,
) -> c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctxt.is_null() || (*ctxt).myDoc.is_null() {
        return 0 as c_int;
    }
    return ((*(*ctxt).myDoc).intSubset != NULL as *mut _xmlDtd) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2HasExternalSubset(
    mut ctx: *mut c_void,
) -> c_int {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctxt.is_null() || (*ctxt).myDoc.is_null() {
        return 0 as c_int;
    }
    return ((*(*ctxt).myDoc).extSubset != NULL as *mut _xmlDtd) as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InternalSubset(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut dtd: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    if ctx.is_null() {
        return;
    }
    if (*ctxt).myDoc.is_null() {
        return;
    }
    dtd = xmlGetIntSubset((*ctxt).myDoc as *const xmlDoc);
    if !dtd.is_null() {
        if (*ctxt).html != 0 {
            return;
        }
        xmlUnlinkNode(dtd as xmlNodePtr);
        xmlFreeDtd(dtd);
        (*(*ctxt).myDoc).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
    }
    (*(*ctxt).myDoc).intSubset =
        xmlCreateIntSubset((*ctxt).myDoc, name, ExternalID, SystemID) as *mut _xmlDtd;
    if (*(*ctxt).myDoc).intSubset.is_null() {
        xmlSAX2ErrMemory(
            ctxt,
            b"xmlSAX2InternalSubset\0" as *const u8 as *const c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ExternalSubset(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    if (!ExternalID.is_null() || !SystemID.is_null())
        && (((*ctxt).validate != 0 || (*ctxt).loadsubset != 0 as c_int)
            && ((*ctxt).wellFormed != 0 && !(*ctxt).myDoc.is_null()))
    {
        let mut oldinput: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
        let mut oldinputNr: c_int = 0;
        let mut oldinputMax: c_int = 0;
        let mut oldinputTab: *mut xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInputPtr>();
        let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
        let mut oldencoding: *const xmlChar = ::core::ptr::null::<xmlChar>();
        let mut oldprogressive: c_int = 0;
        let mut consumed: c_ulong = 0;
        let mut buffered: size_t = 0;
        if !(*ctxt).sax.is_null() && (*(*ctxt).sax).resolveEntity.is_some() {
            input = (*(*ctxt).sax)
                .resolveEntity
                .expect("non-null function pointer")(
                (*ctxt).userData, ExternalID, SystemID
            );
        }
        if input.is_null() {
            return;
        }
        xmlNewDtd((*ctxt).myDoc, name, ExternalID, SystemID);
        oldinput = (*ctxt).input;
        oldinputNr = (*ctxt).inputNr;
        oldinputMax = (*ctxt).inputMax;
        oldinputTab = (*ctxt).inputTab;
        oldencoding = (*ctxt).encoding;
        oldprogressive = (*ctxt).progressive;
        (*ctxt).encoding = ::core::ptr::null::<xmlChar>();
        (*ctxt).progressive = 0 as c_int;
        (*ctxt).inputTab = xmlMalloc.expect("non-null function pointer")(
            (5 as size_t).wrapping_mul(::core::mem::size_of::<xmlParserInputPtr>() as size_t),
        ) as *mut xmlParserInputPtr;
        if (*ctxt).inputTab.is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2ExternalSubset\0" as *const u8 as *const c_char,
            );
            xmlFreeInputStream(input);
            (*ctxt).input = oldinput;
            (*ctxt).inputNr = oldinputNr;
            (*ctxt).inputMax = oldinputMax;
            (*ctxt).inputTab = oldinputTab;
            (*ctxt).encoding = oldencoding;
            (*ctxt).progressive = oldprogressive;
            return;
        }
        (*ctxt).inputNr = 0 as c_int;
        (*ctxt).inputMax = 5 as c_int;
        (*ctxt).input = ::core::ptr::null_mut::<xmlParserInput>();
        xmlPushInput(ctxt, input);
        if (*input).filename.is_null() {
            (*input).filename = xmlCanonicPath(SystemID) as *mut c_char;
        }
        (*input).line = 1 as c_int;
        (*input).col = 1 as c_int;
        (*input).base = (*(*ctxt).input).cur;
        (*input).cur = (*(*ctxt).input).cur;
        (*input).free = None;
        xmlParseExternalSubset(ctxt, ExternalID, SystemID);
        while (*ctxt).inputNr > 1 as c_int {
            xmlPopInput(ctxt);
        }
        consumed = (*(*ctxt).input).consumed;
        buffered = (*(*ctxt).input).cur.offset_from((*(*ctxt).input).base) as c_long
            as size_t;
        if buffered > (ULONG_MAX as size_t).wrapping_sub(consumed as size_t) {
            consumed = ULONG_MAX;
        } else {
            consumed = consumed.wrapping_add(buffered as c_ulong);
        }
        if consumed > ULONG_MAX.wrapping_sub((*ctxt).sizeentities) {
            (*ctxt).sizeentities = ULONG_MAX;
        } else {
            (*ctxt).sizeentities = (*ctxt).sizeentities.wrapping_add(consumed);
        }
        xmlFreeInputStream((*ctxt).input);
        xmlFree.expect("non-null function pointer")((*ctxt).inputTab as *mut c_void);
        (*ctxt).input = oldinput;
        (*ctxt).inputNr = oldinputNr;
        (*ctxt).inputMax = oldinputMax;
        (*ctxt).inputTab = oldinputTab;
        if !(*ctxt).encoding.is_null()
            && ((*ctxt).dict.is_null() || xmlDictOwns((*ctxt).dict, (*ctxt).encoding) == 0)
        {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).encoding as *mut xmlChar as *mut c_void,
            );
        }
        (*ctxt).encoding = oldencoding;
        (*ctxt).progressive = oldprogressive;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ResolveEntity(
    mut ctx: *mut c_void,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) -> xmlParserInputPtr {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut URI: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut base: *const c_char = ::core::ptr::null::<c_char>();
    if ctx.is_null() {
        return ::core::ptr::null_mut::<xmlParserInput>();
    }
    if !(*ctxt).input.is_null() {
        base = (*(*ctxt).input).filename;
    }
    if base.is_null() {
        base = (*ctxt).directory;
    }
    URI = xmlBuildURI(systemId, base as *const xmlChar);
    ret = xmlLoadExternalEntity(
        URI as *const c_char,
        publicId as *const c_char,
        ctxt,
    );
    if !URI.is_null() {
        xmlFree.expect("non-null function pointer")(URI as *mut c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetEntity(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    if ctx.is_null() {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    if (*ctxt).inSubset == 0 as c_int {
        ret = xmlGetPredefinedEntity(name);
        if !ret.is_null() {
            return ret;
        }
    }
    if !(*ctxt).myDoc.is_null() && (*(*ctxt).myDoc).standalone == 1 as c_int {
        if (*ctxt).inSubset == 2 as c_int {
            (*(*ctxt).myDoc).standalone = 0 as c_int;
            ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
            (*(*ctxt).myDoc).standalone = 1 as c_int;
        } else {
            ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
            if ret.is_null() {
                (*(*ctxt).myDoc).standalone = 0 as c_int;
                ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
                if !ret.is_null() {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_NOT_STANDALONE,
                        b"Entity(%s) document marked standalone but requires external subset\n\0"
                            as *const u8 as *const c_char,
                        name,
                        ::core::ptr::null::<xmlChar>(),
                    );
                }
                (*(*ctxt).myDoc).standalone = 1 as c_int;
            }
        }
    } else {
        ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetParameterEntity(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    if ctx.is_null() {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    ret = xmlGetParameterEntity((*ctxt).myDoc, name);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EntityDecl(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut type_0: c_int,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut content: *mut xmlChar,
) {
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    if (*ctxt).inSubset == 1 as c_int {
        ent = xmlAddDocEntity((*ctxt).myDoc, name, type_0, publicId, systemId, content);
        if ent.is_null() && (*ctxt).pedantic != 0 {
            xmlWarnMsg(
                ctxt,
                XML_WAR_ENTITY_REDEFINED,
                b"Entity(%s) already defined in the internal subset\n\0" as *const u8
                    as *const c_char,
                name,
            );
        }
        if !ent.is_null() && (*ent).URI.is_null() && !systemId.is_null() {
            let mut URI: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut base: *const c_char = ::core::ptr::null::<c_char>();
            if !(*ctxt).input.is_null() {
                base = (*(*ctxt).input).filename;
            }
            if base.is_null() {
                base = (*ctxt).directory;
            }
            URI = xmlBuildURI(systemId, base as *const xmlChar);
            (*ent).URI = URI;
        }
    } else if (*ctxt).inSubset == 2 as c_int {
        ent = xmlAddDtdEntity((*ctxt).myDoc, name, type_0, publicId, systemId, content);
        if ent.is_null()
            && (*ctxt).pedantic != 0
            && !(*ctxt).sax.is_null()
            && (*(*ctxt).sax).warning.is_some()
        {
            (*(*ctxt).sax).warning.expect("non-null function pointer")(
                (*ctxt).userData,
                b"Entity(%s) already defined in the external subset\n\0" as *const u8
                    as *const c_char,
                name,
            );
        }
        if !ent.is_null() && (*ent).URI.is_null() && !systemId.is_null() {
            let mut URI_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut base_0: *const c_char = ::core::ptr::null::<c_char>();
            if !(*ctxt).input.is_null() {
                base_0 = (*(*ctxt).input).filename;
            }
            if base_0.is_null() {
                base_0 = (*ctxt).directory;
            }
            URI_0 = xmlBuildURI(systemId, base_0 as *const xmlChar);
            (*ent).URI = URI_0;
        }
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_ENTITY_PROCESSING,
            b"SAX.xmlSAX2EntityDecl(%s) called while not in subset\n\0" as *const u8
                as *const c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2AttributeDecl(
    mut ctx: *mut c_void,
    mut elem: *const xmlChar,
    mut fullname: *const xmlChar,
    mut type_0: c_int,
    mut def: c_int,
    mut defaultValue: *const xmlChar,
    mut tree: xmlEnumerationPtr,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut attr: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if ctxt.is_null() || (*ctxt).myDoc.is_null() {
        return;
    }
    if xmlStrEqual(
        fullname,
        b"xml:id\0" as *const u8 as *const c_char as *mut xmlChar,
    ) != 0
        && type_0 != XML_ATTRIBUTE_ID as c_int
    {
        let mut tmp: c_int = (*ctxt).valid;
        xmlErrValid(
            ctxt,
            XML_DTD_XMLID_TYPE,
            b"xml:id : attribute type should be ID\n\0" as *const u8 as *const c_char,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
        );
        (*ctxt).valid = tmp;
    }
    name = xmlSplitQName(ctxt, fullname, &raw mut prefix);
    (*ctxt).vctxt.valid = 1 as c_int;
    if (*ctxt).inSubset == 1 as c_int {
        attr = xmlAddAttributeDecl(
            &raw mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).intSubset as xmlDtdPtr,
            elem,
            name,
            prefix,
            type_0 as xmlAttributeType,
            def as xmlAttributeDefault,
            defaultValue,
            tree,
        );
    } else if (*ctxt).inSubset == 2 as c_int {
        attr = xmlAddAttributeDecl(
            &raw mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).extSubset as xmlDtdPtr,
            elem,
            name,
            prefix,
            type_0 as xmlAttributeType,
            def as xmlAttributeDefault,
            defaultValue,
            tree,
        );
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"SAX.xmlSAX2AttributeDecl(%s) called while not in subset\n\0" as *const u8
                as *const c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
        xmlFree.expect("non-null function pointer")(name as *mut c_void);
        xmlFreeEnumeration(tree);
        return;
    }
    if (*ctxt).vctxt.valid == 0 as c_int {
        (*ctxt).valid = 0 as c_int;
    }
    if !attr.is_null()
        && (*ctxt).validate != 0
        && (*ctxt).wellFormed != 0
        && !(*(*ctxt).myDoc).intSubset.is_null()
    {
        (*ctxt).valid &= xmlValidateAttributeDecl(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, attr);
    }
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
    }
    if !name.is_null() {
        xmlFree.expect("non-null function pointer")(name as *mut c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ElementDecl(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut type_0: c_int,
    mut content: xmlElementContentPtr,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut elem: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    if ctxt.is_null() || (*ctxt).myDoc.is_null() {
        return;
    }
    if (*ctxt).inSubset == 1 as c_int {
        elem = xmlAddElementDecl(
            &raw mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).intSubset as xmlDtdPtr,
            name,
            type_0 as xmlElementTypeVal,
            content,
        );
    } else if (*ctxt).inSubset == 2 as c_int {
        elem = xmlAddElementDecl(
            &raw mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).extSubset as xmlDtdPtr,
            name,
            type_0 as xmlElementTypeVal,
            content,
        );
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"SAX.xmlSAX2ElementDecl(%s) called while not in subset\n\0" as *const u8
                as *const c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
        return;
    }
    if elem.is_null() {
        (*ctxt).valid = 0 as c_int;
    }
    if (*ctxt).validate != 0
        && (*ctxt).wellFormed != 0
        && !(*ctxt).myDoc.is_null()
        && !(*(*ctxt).myDoc).intSubset.is_null()
    {
        (*ctxt).valid &= xmlValidateElementDecl(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, elem);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2NotationDecl(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut nota: xmlNotationPtr = ::core::ptr::null_mut::<xmlNotation>();
    if ctxt.is_null() || (*ctxt).myDoc.is_null() {
        return;
    }
    if publicId.is_null() && systemId.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NOTATION_PROCESSING,
            b"SAX.xmlSAX2NotationDecl(%s) externalID or PublicID missing\n\0" as *const u8
                as *const c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
        return;
    } else if (*ctxt).inSubset == 1 as c_int {
        nota = xmlAddNotationDecl(
            &raw mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).intSubset as xmlDtdPtr,
            name,
            publicId,
            systemId,
        );
    } else if (*ctxt).inSubset == 2 as c_int {
        nota = xmlAddNotationDecl(
            &raw mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).extSubset as xmlDtdPtr,
            name,
            publicId,
            systemId,
        );
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NOTATION_PROCESSING,
            b"SAX.xmlSAX2NotationDecl(%s) called while not in subset\n\0" as *const u8
                as *const c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
        return;
    }
    if nota.is_null() {
        (*ctxt).valid = 0 as c_int;
    }
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 && !(*(*ctxt).myDoc).intSubset.is_null() {
        (*ctxt).valid &= xmlValidateNotationDecl(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, nota);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2UnparsedEntityDecl(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut notationName: *const xmlChar,
) {
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    if (*ctxt).inSubset == 1 as c_int {
        ent = xmlAddDocEntity(
            (*ctxt).myDoc,
            name,
            XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as c_int,
            publicId,
            systemId,
            notationName,
        );
        if ent.is_null()
            && (*ctxt).pedantic != 0
            && !(*ctxt).sax.is_null()
            && (*(*ctxt).sax).warning.is_some()
        {
            (*(*ctxt).sax).warning.expect("non-null function pointer")(
                (*ctxt).userData,
                b"Entity(%s) already defined in the internal subset\n\0" as *const u8
                    as *const c_char,
                name,
            );
        }
        if !ent.is_null() && (*ent).URI.is_null() && !systemId.is_null() {
            let mut URI: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut base: *const c_char = ::core::ptr::null::<c_char>();
            if !(*ctxt).input.is_null() {
                base = (*(*ctxt).input).filename;
            }
            if base.is_null() {
                base = (*ctxt).directory;
            }
            URI = xmlBuildURI(systemId, base as *const xmlChar);
            (*ent).URI = URI;
        }
    } else if (*ctxt).inSubset == 2 as c_int {
        ent = xmlAddDtdEntity(
            (*ctxt).myDoc,
            name,
            XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as c_int,
            publicId,
            systemId,
            notationName,
        );
        if ent.is_null()
            && (*ctxt).pedantic != 0
            && !(*ctxt).sax.is_null()
            && (*(*ctxt).sax).warning.is_some()
        {
            (*(*ctxt).sax).warning.expect("non-null function pointer")(
                (*ctxt).userData,
                b"Entity(%s) already defined in the external subset\n\0" as *const u8
                    as *const c_char,
                name,
            );
        }
        if !ent.is_null() && (*ent).URI.is_null() && !systemId.is_null() {
            let mut URI_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut base_0: *const c_char = ::core::ptr::null::<c_char>();
            if !(*ctxt).input.is_null() {
                base_0 = (*(*ctxt).input).filename;
            }
            if base_0.is_null() {
                base_0 = (*ctxt).directory;
            }
            URI_0 = xmlBuildURI(systemId, base_0 as *const xmlChar);
            (*ent).URI = URI_0;
        }
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"SAX.xmlSAX2UnparsedEntityDecl(%s) called while not in subset\n\0" as *const u8
                as *const c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2SetDocumentLocator(
    mut ctx: *mut c_void,
    mut loc: xmlSAXLocatorPtr,
) {
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartDocument(mut ctx: *mut c_void) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if ctx.is_null() {
        return;
    }
    if (*ctxt).html != 0 {
        if (*ctxt).myDoc.is_null() {
            (*ctxt).myDoc = htmlNewDocNoDtD(
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            ) as xmlDocPtr;
        }
        if (*ctxt).myDoc.is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2StartDocument\0" as *const u8 as *const c_char,
            );
            return;
        }
        (*(*ctxt).myDoc).properties = XML_DOC_HTML as c_int;
        (*(*ctxt).myDoc).parseFlags = (*ctxt).options;
    } else {
        (*ctxt).myDoc = xmlNewDoc((*ctxt).version);
        doc = (*ctxt).myDoc;
        if !doc.is_null() {
            (*doc).properties = 0 as c_int;
            if (*ctxt).options & XML_PARSE_OLD10 as c_int != 0 {
                (*doc).properties |= XML_DOC_OLD10 as c_int;
            }
            (*doc).parseFlags = (*ctxt).options;
            (*doc).standalone = (*ctxt).standalone;
        } else {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2StartDocument\0" as *const u8 as *const c_char,
            );
            return;
        }
        if (*ctxt).dictNames != 0 && !doc.is_null() {
            (*doc).dict = (*ctxt).dict as *mut _xmlDict;
            xmlDictReference((*doc).dict as xmlDictPtr);
        }
    }
    if !(*ctxt).myDoc.is_null()
        && (*(*ctxt).myDoc).URL.is_null()
        && !(*ctxt).input.is_null()
        && !(*(*ctxt).input).filename.is_null()
    {
        (*(*ctxt).myDoc).URL = xmlPathToURI((*(*ctxt).input).filename as *const xmlChar);
        if (*(*ctxt).myDoc).URL.is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2StartDocument\0" as *const u8 as *const c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndDocument(mut ctx: *mut c_void) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if ctx.is_null() {
        return;
    }
    if (*ctxt).validate != 0
        && (*ctxt).wellFormed != 0
        && !(*ctxt).myDoc.is_null()
        && !(*(*ctxt).myDoc).intSubset.is_null()
    {
        (*ctxt).valid &= xmlValidateDocumentFinal(&raw mut (*ctxt).vctxt, (*ctxt).myDoc);
    }
    doc = (*ctxt).myDoc;
    if !doc.is_null() && (*doc).encoding.is_null() {
        let mut encoding: *const xmlChar = ::core::ptr::null::<xmlChar>();
        if (*(*ctxt).input).flags as c_uint & XML_INPUT_USES_ENC_DECL != 0
            || (*(*ctxt).input).flags as c_uint & XML_INPUT_AUTO_ENCODING != 0
        {
            encoding = (*ctxt).encoding;
        } else if !(*(*ctxt).input).buf.is_null() && !(*(*(*ctxt).input).buf).encoder.is_null() {
            encoding = (*(*(*(*ctxt).input).buf).encoder).name as *mut xmlChar;
        } else if (*(*ctxt).input).flags as c_uint & XML_INPUT_HAS_ENCODING != 0 {
            encoding = b"UTF-8\0" as *const u8 as *const c_char as *mut xmlChar;
        }
        if !encoding.is_null() {
            (*doc).encoding = xmlStrdup(encoding);
            if (*doc).encoding.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2EndDocument\0" as *const u8 as *const c_char,
                );
            }
        }
    }
}
unsafe extern "C" fn xmlNsErrMsg(
    mut ctxt: xmlParserCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    if !ctxt.is_null()
        && (*ctxt).disableSAX != 0 as c_int
        && (*ctxt).instate as c_int == XML_PARSER_EOF as c_int
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as c_int;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut c_void,
        NULL,
        XML_FROM_NAMESPACE as c_int,
        error as c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        str1 as *const c_char,
        str2 as *const c_char,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        msg,
        str1,
        str2,
    );
}
unsafe extern "C" fn xmlSAX2AttributeInternal(
    mut ctx: *mut c_void,
    mut fullname: *const xmlChar,
    mut value: *const xmlChar,
    mut prefix: *const xmlChar,
) {
    let mut current_block: u64;
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ns: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut nval: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut namespace: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if (*ctxt).html != 0 {
        name = xmlStrdup(fullname);
        ns = ::core::ptr::null_mut::<xmlChar>();
        namespace = ::core::ptr::null_mut::<xmlNs>();
    } else {
        name = xmlSplitQName(ctxt, fullname, &raw mut ns);
        if !name.is_null()
            && *name.offset(0 as c_int as isize) as c_int
                == 0 as c_int
        {
            if xmlStrEqual(
                ns,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                xmlNsErrMsg(
                    ctxt,
                    XML_ERR_NS_DECL_ERROR,
                    b"invalid namespace declaration '%s'\n\0" as *const u8
                        as *const c_char,
                    fullname,
                    ::core::ptr::null::<xmlChar>(),
                );
            } else {
                xmlNsWarnMsg(
                    ctxt,
                    XML_WAR_NS_COLUMN,
                    b"Avoid attribute ending with ':' like '%s'\n\0" as *const u8
                        as *const c_char,
                    fullname,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as *mut c_void);
            }
            ns = ::core::ptr::null_mut::<xmlChar>();
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
            name = xmlStrdup(fullname);
        }
    }
    if name.is_null() {
        xmlSAX2ErrMemory(
            ctxt,
            b"xmlSAX2StartElement\0" as *const u8 as *const c_char,
        );
        if !ns.is_null() {
            xmlFree.expect("non-null function pointer")(ns as *mut c_void);
        }
        return;
    }
    if (*ctxt).html != 0 && value.is_null() && htmlIsBooleanAttr(fullname) != 0 {
        nval = xmlStrdup(fullname);
        value = nval as *const xmlChar;
    } else {
        (*ctxt).vctxt.valid = 1 as c_int;
        nval = xmlValidCtxtNormalizeAttributeValue(
            &raw mut (*ctxt).vctxt,
            (*ctxt).myDoc,
            (*ctxt).node,
            fullname,
            value,
        );
        if (*ctxt).vctxt.valid != 1 as c_int {
            (*ctxt).valid = 0 as c_int;
        }
        if !nval.is_null() {
            value = nval;
        }
    }
    if (*ctxt).html == 0
        && ns.is_null()
        && *name.offset(0 as c_int as isize) as c_int == 'x' as i32
        && *name.offset(1 as c_int as isize) as c_int == 'm' as i32
        && *name.offset(2 as c_int as isize) as c_int == 'l' as i32
        && *name.offset(3 as c_int as isize) as c_int == 'n' as i32
        && *name.offset(4 as c_int as isize) as c_int == 's' as i32
        && *name.offset(5 as c_int as isize) as c_int
            == 0 as c_int
    {
        let mut nsret: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
        let mut val: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        if (*ctxt).replaceEntities == 0 {
            (*ctxt).depth += 1;
            val = xmlStringDecodeEntities(
                ctxt,
                value,
                XML_SUBSTITUTE_REF,
                0 as xmlChar,
                0 as xmlChar,
                0 as xmlChar,
            );
            (*ctxt).depth -= 1;
            if val.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2StartElement\0" as *const u8 as *const c_char,
                );
                if !name.is_null() {
                    xmlFree.expect("non-null function pointer")(name as *mut c_void);
                }
                if !nval.is_null() {
                    xmlFree.expect("non-null function pointer")(nval as *mut c_void);
                }
                return;
            }
        } else {
            val = value as *mut xmlChar;
        }
        if *val.offset(0 as c_int as isize) as c_int
            != 0 as c_int
        {
            let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
            uri = xmlParseURI(val as *const c_char);
            if uri.is_null() {
                if !(*ctxt).sax.is_null() && (*(*ctxt).sax).warning.is_some() {
                    (*(*ctxt).sax).warning.expect("non-null function pointer")(
                        (*ctxt).userData,
                        b"xmlns: %s not a valid URI\n\0" as *const u8 as *const c_char,
                        val,
                    );
                }
            } else {
                if (*uri).scheme.is_null() {
                    if !(*ctxt).sax.is_null() && (*(*ctxt).sax).warning.is_some() {
                        (*(*ctxt).sax).warning.expect("non-null function pointer")(
                            (*ctxt).userData,
                            b"xmlns: URI %s is not absolute\n\0" as *const u8
                                as *const c_char,
                            val,
                        );
                    }
                }
                xmlFreeURI(uri);
            }
        }
        nsret = xmlNewNs((*ctxt).node, val, ::core::ptr::null::<xmlChar>());
        if !nsret.is_null()
            && (*ctxt).validate != 0
            && (*ctxt).wellFormed != 0
            && !(*ctxt).myDoc.is_null()
            && !(*(*ctxt).myDoc).intSubset.is_null()
        {
            (*ctxt).valid &= xmlValidateOneNamespace(
                &raw mut (*ctxt).vctxt,
                (*ctxt).myDoc,
                (*ctxt).node,
                prefix,
                nsret,
                val,
            );
        }
        if !name.is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
        }
        if !nval.is_null() {
            xmlFree.expect("non-null function pointer")(nval as *mut c_void);
        }
        if val != value as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(val as *mut c_void);
        }
        return;
    }
    if (*ctxt).html == 0
        && !ns.is_null()
        && *ns.offset(0 as c_int as isize) as c_int == 'x' as i32
        && *ns.offset(1 as c_int as isize) as c_int == 'm' as i32
        && *ns.offset(2 as c_int as isize) as c_int == 'l' as i32
        && *ns.offset(3 as c_int as isize) as c_int == 'n' as i32
        && *ns.offset(4 as c_int as isize) as c_int == 's' as i32
        && *ns.offset(5 as c_int as isize) as c_int
            == 0 as c_int
    {
        let mut nsret_0: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
        let mut val_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        if (*ctxt).replaceEntities == 0 {
            (*ctxt).depth += 1;
            val_0 = xmlStringDecodeEntities(
                ctxt,
                value,
                XML_SUBSTITUTE_REF,
                0 as xmlChar,
                0 as xmlChar,
                0 as xmlChar,
            );
            (*ctxt).depth -= 1;
            if val_0.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2StartElement\0" as *const u8 as *const c_char,
                );
                xmlFree.expect("non-null function pointer")(ns as *mut c_void);
                if !name.is_null() {
                    xmlFree.expect("non-null function pointer")(name as *mut c_void);
                }
                if !nval.is_null() {
                    xmlFree.expect("non-null function pointer")(nval as *mut c_void);
                }
                return;
            }
        } else {
            val_0 = value as *mut xmlChar;
        }
        if *val_0.offset(0 as c_int as isize) as c_int
            == 0 as c_int
        {
            xmlNsErrMsg(
                ctxt,
                XML_NS_ERR_EMPTY,
                b"Empty namespace name for prefix %s\n\0" as *const u8
                    as *const c_char,
                name,
                ::core::ptr::null::<xmlChar>(),
            );
        }
        if (*ctxt).pedantic != 0 as c_int
            && *val_0.offset(0 as c_int as isize) as c_int
                != 0 as c_int
        {
            let mut uri_0: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
            uri_0 = xmlParseURI(val_0 as *const c_char);
            if uri_0.is_null() {
                xmlNsWarnMsg(
                    ctxt,
                    XML_WAR_NS_URI,
                    b"xmlns:%s: %s not a valid URI\n\0" as *const u8 as *const c_char,
                    name,
                    value,
                );
            } else {
                if (*uri_0).scheme.is_null() {
                    xmlNsWarnMsg(
                        ctxt,
                        XML_WAR_NS_URI_RELATIVE,
                        b"xmlns:%s: URI %s is not absolute\n\0" as *const u8
                            as *const c_char,
                        name,
                        value,
                    );
                }
                xmlFreeURI(uri_0);
            }
        }
        nsret_0 = xmlNewNs((*ctxt).node, val_0, name);
        xmlFree.expect("non-null function pointer")(ns as *mut c_void);
        if !nsret_0.is_null()
            && (*ctxt).validate != 0
            && (*ctxt).wellFormed != 0
            && !(*ctxt).myDoc.is_null()
            && !(*(*ctxt).myDoc).intSubset.is_null()
        {
            (*ctxt).valid &= xmlValidateOneNamespace(
                &raw mut (*ctxt).vctxt,
                (*ctxt).myDoc,
                (*ctxt).node,
                prefix,
                nsret_0,
                value,
            );
        }
        if !name.is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
        }
        if !nval.is_null() {
            xmlFree.expect("non-null function pointer")(nval as *mut c_void);
        }
        if val_0 != value as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(val_0 as *mut c_void);
        }
        return;
    }
    if !ns.is_null() {
        namespace = xmlSearchNs((*ctxt).myDoc, (*ctxt).node, ns);
        if namespace.is_null() {
            xmlNsErrMsg(
                ctxt,
                XML_NS_ERR_UNDEFINED_NAMESPACE,
                b"Namespace prefix %s of attribute %s is not defined\n\0" as *const u8
                    as *const c_char,
                ns,
                name,
            );
            current_block = 12079920068676227593;
        } else {
            let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
            prop = (*(*ctxt).node).properties as xmlAttrPtr;
            loop {
                if prop.is_null() {
                    current_block = 12079920068676227593;
                    break;
                }
                if !(*prop).ns.is_null() {
                    if xmlStrEqual(name, (*prop).name) != 0
                        && (namespace == (*prop).ns
                            || xmlStrEqual((*namespace).href, (*(*prop).ns).href) != 0)
                    {
                        xmlNsErrMsg(
                            ctxt,
                            XML_ERR_ATTRIBUTE_REDEFINED,
                            b"Attribute %s in %s redefined\n\0" as *const u8
                                as *const c_char,
                            name,
                            (*namespace).href,
                        );
                        (*ctxt).wellFormed = 0 as c_int;
                        if (*ctxt).recovery == 0 as c_int {
                            (*ctxt).disableSAX = 1 as c_int;
                        }
                        if !name.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                name as *mut c_void,
                            );
                        }
                        current_block = 4935335238440984547;
                        break;
                    }
                }
                prop = (*prop).next as xmlAttrPtr;
            }
        }
    } else {
        namespace = ::core::ptr::null_mut::<xmlNs>();
        current_block = 12079920068676227593;
    }
    match current_block {
        12079920068676227593 => {
            ret = xmlNewNsPropEatName(
                (*ctxt).node,
                namespace,
                name,
                ::core::ptr::null::<xmlChar>(),
            );
            if !ret.is_null() {
                if (*ctxt).replaceEntities == 0 as c_int && (*ctxt).html == 0 {
                    let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                    (*ret).children = xmlStringGetNodeList((*ctxt).myDoc as *const xmlDoc, value)
                        as *mut _xmlNode;
                    tmp = (*ret).children as xmlNodePtr;
                    while !tmp.is_null() {
                        (*tmp).parent = ret as xmlNodePtr as *mut _xmlNode;
                        if (*tmp).next.is_null() {
                            (*ret).last = tmp as *mut _xmlNode;
                        }
                        tmp = (*tmp).next as xmlNodePtr;
                    }
                } else if !value.is_null() {
                    (*ret).children =
                        xmlNewDocText((*ctxt).myDoc as *const xmlDoc, value) as *mut _xmlNode;
                    (*ret).last = (*ret).children;
                    if !(*ret).children.is_null() {
                        (*(*ret).children).parent = ret as xmlNodePtr as *mut _xmlNode;
                    }
                }
                if (*ctxt).html == 0
                    && (*ctxt).validate != 0
                    && (*ctxt).wellFormed != 0
                    && !(*ctxt).myDoc.is_null()
                    && !(*(*ctxt).myDoc).intSubset.is_null()
                {
                    if (*ctxt).replaceEntities == 0 {
                        let mut val_1: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        (*ctxt).depth += 1;
                        val_1 = xmlStringDecodeEntities(
                            ctxt,
                            value,
                            XML_SUBSTITUTE_REF,
                            0 as xmlChar,
                            0 as xmlChar,
                            0 as xmlChar,
                        );
                        (*ctxt).depth -= 1;
                        if val_1.is_null() {
                            (*ctxt).valid &= xmlValidateOneAttribute(
                                &raw mut (*ctxt).vctxt,
                                (*ctxt).myDoc,
                                (*ctxt).node,
                                ret,
                                value,
                            );
                        } else {
                            let mut nvalnorm: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                            nvalnorm = xmlValidNormalizeAttributeValue(
                                (*ctxt).myDoc,
                                (*ctxt).node,
                                fullname,
                                val_1,
                            );
                            if !nvalnorm.is_null() {
                                xmlFree.expect("non-null function pointer")(
                                    val_1 as *mut c_void,
                                );
                                val_1 = nvalnorm;
                            }
                            (*ctxt).valid &= xmlValidateOneAttribute(
                                &raw mut (*ctxt).vctxt,
                                (*ctxt).myDoc,
                                (*ctxt).node,
                                ret,
                                val_1,
                            );
                            xmlFree.expect("non-null function pointer")(
                                val_1 as *mut c_void,
                            );
                        }
                    } else {
                        (*ctxt).valid &= xmlValidateOneAttribute(
                            &raw mut (*ctxt).vctxt,
                            (*ctxt).myDoc,
                            (*ctxt).node,
                            ret,
                            value,
                        );
                    }
                } else if (*ctxt).loadsubset & XML_SKIP_IDS == 0 as c_int
                    && ((*ctxt).replaceEntities == 0 as c_int
                        && (*ctxt).external != 2 as c_int
                        || (*ctxt).replaceEntities != 0 as c_int
                            && (*ctxt).inSubset == 0 as c_int)
                    && !(*ret).children.is_null()
                    && (*(*ret).children).type_0 as c_uint
                        == XML_TEXT_NODE as c_int as c_uint
                    && (*(*ret).children).next.is_null()
                {
                    let mut content: *mut xmlChar = (*(*ret).children).content;
                    if xmlStrEqual(
                        fullname,
                        b"xml:id\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) != 0
                    {
                        if xmlValidateNCName(content, 1 as c_int)
                            != 0 as c_int
                        {
                            xmlErrValid(
                                ctxt,
                                XML_DTD_XMLID_VALUE,
                                b"xml:id : attribute value %s is not an NCName\n\0" as *const u8
                                    as *const c_char,
                                content as *const c_char,
                                ::core::ptr::null::<c_char>(),
                            );
                        }
                        xmlAddID(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
                    } else if xmlIsID((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
                        xmlAddID(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
                    } else if xmlIsRef((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
                        xmlAddRef(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
                    }
                }
            }
        }
        _ => {}
    }
    if !nval.is_null() {
        xmlFree.expect("non-null function pointer")(nval as *mut c_void);
    }
    if !ns.is_null() {
        xmlFree.expect("non-null function pointer")(ns as *mut c_void);
    }
}
unsafe extern "C" fn xmlCheckDefaultedAttributes(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const xmlChar,
    mut prefix: *const xmlChar,
    mut atts: *mut *const xmlChar,
) {
    let mut elemDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    let mut att: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut internal: c_int = 1 as c_int;
    let mut i: c_int = 0;
    elemDecl = xmlGetDtdQElementDesc((*(*ctxt).myDoc).intSubset as xmlDtdPtr, name, prefix);
    if elemDecl.is_null() {
        elemDecl = xmlGetDtdQElementDesc((*(*ctxt).myDoc).extSubset as xmlDtdPtr, name, prefix);
        internal = 0 as c_int;
    }
    while !elemDecl.is_null() {
        let mut attr: xmlAttributePtr = (*elemDecl).attributes;
        if (*(*ctxt).myDoc).standalone == 1 as c_int
            && !(*(*ctxt).myDoc).extSubset.is_null()
            && (*ctxt).validate != 0
        {
            while !attr.is_null() {
                if !(*attr).defaultValue.is_null()
                    && xmlGetDtdQAttrDesc(
                        (*(*ctxt).myDoc).extSubset as xmlDtdPtr,
                        (*attr).elem,
                        (*attr).name,
                        (*attr).prefix,
                    ) == attr
                    && xmlGetDtdQAttrDesc(
                        (*(*ctxt).myDoc).intSubset as xmlDtdPtr,
                        (*attr).elem,
                        (*attr).name,
                        (*attr).prefix,
                    )
                    .is_null()
                {
                    let mut fulln: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    if !(*attr).prefix.is_null() {
                        fulln = xmlStrdup((*attr).prefix);
                        fulln = xmlStrcat(
                            fulln,
                            b":\0" as *const u8 as *const c_char as *mut xmlChar,
                        );
                        fulln = xmlStrcat(fulln, (*attr).name);
                    } else {
                        fulln = xmlStrdup((*attr).name);
                    }
                    if fulln.is_null() {
                        xmlSAX2ErrMemory(
                            ctxt,
                            b"xmlSAX2StartElement\0" as *const u8 as *const c_char,
                        );
                        break;
                    } else {
                        att = ::core::ptr::null::<xmlChar>();
                        if !atts.is_null() {
                            i = 0 as c_int;
                            att = *atts.offset(i as isize);
                            while !att.is_null() {
                                if xmlStrEqual(att, fulln) != 0 {
                                    break;
                                }
                                i += 2 as c_int;
                                att = *atts.offset(i as isize);
                            }
                        }
                        if att.is_null() {
                            xmlErrValid(
                                ctxt,
                                XML_DTD_STANDALONE_DEFAULTED,
                                b"standalone: attribute %s on %s defaulted from external subset\n\0"
                                    as *const u8
                                    as *const c_char,
                                fulln as *const c_char,
                                (*attr).elem as *const c_char,
                            );
                        }
                        xmlFree.expect("non-null function pointer")(
                            fulln as *mut c_void,
                        );
                    }
                }
                attr = (*attr).nexth as xmlAttributePtr;
            }
        }
        attr = (*elemDecl).attributes;
        while !attr.is_null() {
            if !(*attr).defaultValue.is_null() {
                if !(*attr).prefix.is_null()
                    && xmlStrEqual(
                        (*attr).prefix,
                        b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) != 0
                    || (*attr).prefix.is_null()
                        && xmlStrEqual(
                            (*attr).name,
                            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                        ) != 0
                    || (*ctxt).loadsubset & XML_COMPLETE_ATTRS != 0
                {
                    let mut tst: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
                    tst = xmlGetDtdQAttrDesc(
                        (*(*ctxt).myDoc).intSubset as xmlDtdPtr,
                        (*attr).elem,
                        (*attr).name,
                        (*attr).prefix,
                    );
                    if tst == attr || tst.is_null() {
                        let mut fn_0: [xmlChar; 50] = [0; 50];
                        let mut fulln_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        fulln_0 = xmlBuildQName(
                            (*attr).name,
                            (*attr).prefix,
                            &raw mut fn_0 as *mut xmlChar,
                            50 as c_int,
                        );
                        if fulln_0.is_null() {
                            xmlSAX2ErrMemory(
                                ctxt,
                                b"xmlSAX2StartElement\0" as *const u8 as *const c_char,
                            );
                            return;
                        }
                        att = ::core::ptr::null::<xmlChar>();
                        if !atts.is_null() {
                            i = 0 as c_int;
                            att = *atts.offset(i as isize);
                            while !att.is_null() {
                                if xmlStrEqual(att, fulln_0) != 0 {
                                    break;
                                }
                                i += 2 as c_int;
                                att = *atts.offset(i as isize);
                            }
                        }
                        if att.is_null() {
                            xmlSAX2AttributeInternal(
                                ctxt as *mut c_void,
                                fulln_0,
                                (*attr).defaultValue,
                                prefix,
                            );
                        }
                        if fulln_0 != &raw mut fn_0 as *mut xmlChar
                            && fulln_0 != (*attr).name as *mut xmlChar
                        {
                            xmlFree.expect("non-null function pointer")(
                                fulln_0 as *mut c_void,
                            );
                        }
                    }
                }
            }
            attr = (*attr).nexth as xmlAttributePtr;
        }
        if !(internal == 1 as c_int) {
            break;
        }
        elemDecl = xmlGetDtdQElementDesc((*(*ctxt).myDoc).extSubset as xmlDtdPtr, name, prefix);
        internal = 0 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartElement(
    mut ctx: *mut c_void,
    mut fullname: *const xmlChar,
    mut atts: *mut *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut att: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut value: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut i: c_int = 0;
    if ctx.is_null() || fullname.is_null() || (*ctxt).myDoc.is_null() {
        return;
    }
    if (*ctxt).validate != 0
        && (*(*ctxt).myDoc).extSubset.is_null()
        && ((*(*ctxt).myDoc).intSubset.is_null()
            || (*(*(*ctxt).myDoc).intSubset).notations.is_null()
                && (*(*(*ctxt).myDoc).intSubset).elements.is_null()
                && (*(*(*ctxt).myDoc).intSubset).attributes.is_null()
                && (*(*(*ctxt).myDoc).intSubset).entities.is_null())
    {
        xmlErrValid(
            ctxt,
            XML_ERR_NO_DTD,
            b"Validation failed: no DTD found !\0" as *const u8 as *const c_char,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
        );
        (*ctxt).validate = 0 as c_int;
    }
    if (*ctxt).html != 0 {
        prefix = ::core::ptr::null_mut::<xmlChar>();
        name = xmlStrdup(fullname);
    } else {
        name = xmlSplitQName(ctxt, fullname, &raw mut prefix);
    }
    ret = xmlNewDocNodeEatName(
        (*ctxt).myDoc,
        ::core::ptr::null_mut::<xmlNs>(),
        name,
        ::core::ptr::null::<xmlChar>(),
    );
    if ret.is_null() {
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        }
        xmlSAX2ErrMemory(
            ctxt,
            b"xmlSAX2StartElement\0" as *const u8 as *const c_char,
        );
        return;
    }
    (*ctxt).nodemem = -(1 as c_int);
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if ((*(*ctxt).input).line as c_uint) < USHRT_MAX as c_uint {
                (*ret).line = (*(*ctxt).input).line as c_ushort;
            } else {
                (*ret).line = USHRT_MAX as c_ushort;
            }
        }
    }
    parent = (*ctxt).node;
    if parent.is_null() {
        parent = (*ctxt).myDoc as xmlNodePtr;
    }
    if nodePush(ctxt, ret) < 0 as c_int {
        xmlUnlinkNode(ret);
        xmlFreeNode(ret);
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        }
        return;
    }
    xmlAddChild(parent, ret);
    if (*ctxt).html == 0 {
        if !(*(*ctxt).myDoc).intSubset.is_null() || !(*(*ctxt).myDoc).extSubset.is_null() {
            xmlCheckDefaultedAttributes(ctxt, name, prefix, atts);
        }
        if !atts.is_null() {
            i = 0 as c_int;
            let fresh0 = i;
            i = i + 1;
            att = *atts.offset(fresh0 as isize);
            let fresh1 = i;
            i = i + 1;
            value = *atts.offset(fresh1 as isize);
            while !att.is_null() && !value.is_null() {
                if *att.offset(0 as c_int as isize) as c_int == 'x' as i32
                    && *att.offset(1 as c_int as isize) as c_int
                        == 'm' as i32
                    && *att.offset(2 as c_int as isize) as c_int
                        == 'l' as i32
                    && *att.offset(3 as c_int as isize) as c_int
                        == 'n' as i32
                    && *att.offset(4 as c_int as isize) as c_int
                        == 's' as i32
                {
                    xmlSAX2AttributeInternal(ctxt as *mut c_void, att, value, prefix);
                }
                let fresh2 = i;
                i = i + 1;
                att = *atts.offset(fresh2 as isize);
                let fresh3 = i;
                i = i + 1;
                value = *atts.offset(fresh3 as isize);
            }
        }
        ns = xmlSearchNs((*ctxt).myDoc, ret, prefix);
        if ns.is_null() && !parent.is_null() {
            ns = xmlSearchNs((*ctxt).myDoc, parent, prefix);
        }
        if !prefix.is_null() && ns.is_null() {
            ns = xmlNewNs(ret, ::core::ptr::null::<xmlChar>(), prefix);
            xmlNsWarnMsg(
                ctxt,
                XML_NS_ERR_UNDEFINED_NAMESPACE,
                b"Namespace prefix %s is not defined\n\0" as *const u8
                    as *const c_char,
                prefix,
                ::core::ptr::null::<xmlChar>(),
            );
        }
        if !ns.is_null()
            && !(*ns).href.is_null()
            && (*(*ns).href.offset(0 as c_int as isize) as c_int
                != 0 as c_int
                || !(*ns).prefix.is_null())
        {
            xmlSetNs(ret, ns);
        }
    }
    if !atts.is_null() {
        i = 0 as c_int;
        let fresh4 = i;
        i = i + 1;
        att = *atts.offset(fresh4 as isize);
        let fresh5 = i;
        i = i + 1;
        value = *atts.offset(fresh5 as isize);
        if (*ctxt).html != 0 {
            while !att.is_null() {
                xmlSAX2AttributeInternal(
                    ctxt as *mut c_void,
                    att,
                    value,
                    ::core::ptr::null::<xmlChar>(),
                );
                let fresh6 = i;
                i = i + 1;
                att = *atts.offset(fresh6 as isize);
                let fresh7 = i;
                i = i + 1;
                value = *atts.offset(fresh7 as isize);
            }
        } else {
            while !att.is_null() && !value.is_null() {
                if *att.offset(0 as c_int as isize) as c_int != 'x' as i32
                    || *att.offset(1 as c_int as isize) as c_int
                        != 'm' as i32
                    || *att.offset(2 as c_int as isize) as c_int
                        != 'l' as i32
                    || *att.offset(3 as c_int as isize) as c_int
                        != 'n' as i32
                    || *att.offset(4 as c_int as isize) as c_int
                        != 's' as i32
                {
                    xmlSAX2AttributeInternal(
                        ctxt as *mut c_void,
                        att,
                        value,
                        ::core::ptr::null::<xmlChar>(),
                    );
                }
                let fresh8 = i;
                i = i + 1;
                att = *atts.offset(fresh8 as isize);
                let fresh9 = i;
                i = i + 1;
                value = *atts.offset(fresh9 as isize);
            }
        }
    }
    if (*ctxt).validate != 0
        && (*ctxt).vctxt.flags & XML_VCTXT_DTD_VALIDATED == 0 as c_uint
    {
        let mut chk: c_int = 0;
        chk = xmlValidateDtdFinal(&raw mut (*ctxt).vctxt, (*ctxt).myDoc);
        if chk <= 0 as c_int {
            (*ctxt).valid = 0 as c_int;
        }
        if chk < 0 as c_int {
            (*ctxt).wellFormed = 0 as c_int;
        }
        (*ctxt).valid &= xmlValidateRoot(&raw mut (*ctxt).vctxt, (*ctxt).myDoc);
        (*ctxt).vctxt.flags |= XML_VCTXT_DTD_VALIDATED;
    }
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndElement(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    (*ctxt).nodemem = -(1 as c_int);
    if (*ctxt).validate != 0
        && (*ctxt).wellFormed != 0
        && !(*ctxt).myDoc.is_null()
        && !(*(*ctxt).myDoc).intSubset.is_null()
    {
        (*ctxt).valid &= xmlValidateOneElement(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, (*ctxt).node);
    }
    nodePop(ctxt);
}
unsafe extern "C" fn xmlSAX2TextNode(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut len: c_int,
) -> xmlNodePtr {
    let mut current_block: u64;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut intern: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if !(*ctxt).freeElems.is_null() {
        ret = (*ctxt).freeElems;
        (*ctxt).freeElems = (*ret).next as xmlNodePtr;
        (*ctxt).freeElemsNr -= 1;
    } else {
        ret = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlNode>() as size_t
        ) as xmlNodePtr;
    }
    if ret.is_null() {
        xmlErrMemory(
            ctxt,
            b"xmlSAX2Characters\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    if (*ctxt).dictNames != 0 {
        let mut cur: xmlChar = *str.offset(len as isize);
        if len
            < (2 as usize).wrapping_mul(::core::mem::size_of::<*mut c_void>() as usize)
                as c_int
            && (*ctxt).options & XML_PARSE_COMPACT as c_int != 0
        {
            let mut tmp: *mut xmlChar = &raw mut (*ret).properties as *mut xmlChar;
            memcpy(
                tmp as *mut c_void,
                str as *const c_void,
                len as size_t,
            );
            *tmp.offset(len as isize) = 0 as xmlChar;
            intern = tmp;
        } else if len <= 3 as c_int
            && (cur as c_int == '"' as i32
                || cur as c_int == '\'' as i32
                || cur as c_int == '<' as i32
                    && *str.offset((len + 1 as c_int) as isize) as c_int
                        != '!' as i32)
        {
            intern = xmlDictLookup((*ctxt).dict, str, len);
        } else if (*str as c_int == 0x20 as c_int
            || 0x9 as c_int <= *str as c_int
                && *str as c_int <= 0xa as c_int
            || *str as c_int == 0xd as c_int)
            && len < 60 as c_int
            && cur as c_int == '<' as i32
            && *str.offset((len + 1 as c_int) as isize) as c_int
                != '!' as i32
        {
            let mut i: c_int = 0;
            i = 1 as c_int;
            loop {
                if !(i < len) {
                    current_block = 4808432441040389987;
                    break;
                }
                if !(*str.offset(i as isize) as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *str.offset(i as isize) as c_int
                        && *str.offset(i as isize) as c_int
                            <= 0xa as c_int
                    || *str.offset(i as isize) as c_int == 0xd as c_int)
                {
                    current_block = 13903165572170679785;
                    break;
                }
                i += 1;
            }
            match current_block {
                13903165572170679785 => {}
                _ => {
                    intern = xmlDictLookup((*ctxt).dict, str, len);
                }
            }
        }
    }
    (*ret).type_0 = XML_TEXT_NODE;
    (*ret).name = &raw const xmlStringText as *const xmlChar;
    if intern.is_null() {
        (*ret).content = xmlStrndup(str, len);
        if (*ret).content.is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2TextNode\0" as *const u8 as *const c_char,
            );
            xmlFree.expect("non-null function pointer")(ret as *mut c_void);
            return ::core::ptr::null_mut::<xmlNode>();
        }
    } else {
        (*ret).content = intern as *mut xmlChar;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret);
    }
    return ret;
}
unsafe extern "C" fn xmlSAX2DecodeAttrEntities(
    mut ctxt: xmlParserCtxtPtr,
    mut str: *const xmlChar,
    mut end: *const xmlChar,
) -> *mut xmlChar {
    let mut current_block: u64;
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    in_0 = str;
    loop {
        if !(in_0 < end) {
            current_block = 12675440807659640239;
            break;
        }
        let fresh12 = in_0;
        in_0 = in_0.offset(1);
        if *fresh12 as c_int == '&' as i32 {
            current_block = 17997983287443350040;
            break;
        }
    }
    match current_block {
        12675440807659640239 => return ::core::ptr::null_mut::<xmlChar>(),
        _ => {
            (*ctxt).depth += 1;
            ret = xmlStringLenDecodeEntities(
                ctxt,
                str,
                end.offset_from(str) as c_long as c_int,
                XML_SUBSTITUTE_REF,
                0 as xmlChar,
                0 as xmlChar,
                0 as xmlChar,
            );
            (*ctxt).depth -= 1;
            return ret;
        }
    };
}
unsafe extern "C" fn xmlSAX2AttributeNs(
    mut ctxt: xmlParserCtxtPtr,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut value: *const xmlChar,
    mut valueend: *const xmlChar,
) -> xmlAttrPtr {
    let mut ret: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut namespace: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut dup: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if !prefix.is_null() {
        namespace = xmlParserNsLookupSax(ctxt, prefix) as xmlNsPtr;
        if namespace.is_null()
            && xmlStrEqual(
                prefix,
                b"xml\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
        {
            namespace = xmlSearchNs((*ctxt).myDoc, (*ctxt).node, prefix);
        }
    }
    if !(*ctxt).freeAttrs.is_null() {
        ret = (*ctxt).freeAttrs;
        (*ctxt).freeAttrs = (*ret).next as xmlAttrPtr;
        (*ctxt).freeAttrsNr -= 1;
    } else {
        ret = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlAttr>() as size_t
        ) as xmlAttrPtr;
        if ret.is_null() {
            xmlSAX2ErrMemory(ctxt, ::core::ptr::null::<c_char>());
            return ::core::ptr::null_mut::<xmlAttr>();
        }
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlAttr>() as size_t,
    );
    (*ret).type_0 = XML_ATTRIBUTE_NODE;
    (*ret).parent = (*ctxt).node as *mut _xmlNode;
    (*ret).doc = (*(*ctxt).node).doc;
    (*ret).ns = namespace as *mut xmlNs;
    if (*ctxt).dictNames != 0 {
        (*ret).name = localname;
    } else {
        (*ret).name = xmlStrdup(localname);
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret as xmlNodePtr);
    }
    if (*ctxt).replaceEntities == 0 as c_int && (*ctxt).html == 0 {
        let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        if *valueend as c_int != 0 as c_int {
            tmp = xmlSAX2TextNode(
                ctxt,
                value,
                valueend.offset_from(value) as c_long as c_int,
            );
            (*ret).children = tmp as *mut _xmlNode;
            (*ret).last = tmp as *mut _xmlNode;
            if !tmp.is_null() {
                (*tmp).doc = (*ret).doc;
                (*tmp).parent = ret as xmlNodePtr as *mut _xmlNode;
            }
        } else {
            (*ret).children = xmlStringLenGetNodeList(
                (*ctxt).myDoc as *const xmlDoc,
                value,
                valueend.offset_from(value) as c_long as c_int,
            ) as *mut _xmlNode;
            tmp = (*ret).children as xmlNodePtr;
            while !tmp.is_null() {
                (*tmp).doc = (*ret).doc;
                (*tmp).parent = ret as xmlNodePtr as *mut _xmlNode;
                if (*tmp).next.is_null() {
                    (*ret).last = tmp as *mut _xmlNode;
                }
                tmp = (*tmp).next as xmlNodePtr;
            }
        }
    } else if !value.is_null() {
        let mut tmp_0: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        tmp_0 = xmlSAX2TextNode(
            ctxt,
            value,
            valueend.offset_from(value) as c_long as c_int,
        );
        (*ret).children = tmp_0 as *mut _xmlNode;
        (*ret).last = tmp_0 as *mut _xmlNode;
        if !tmp_0.is_null() {
            (*tmp_0).doc = (*ret).doc;
            (*tmp_0).parent = ret as xmlNodePtr as *mut _xmlNode;
        }
    }
    if (*ctxt).html == 0
        && (*ctxt).validate != 0
        && (*ctxt).wellFormed != 0
        && !(*ctxt).myDoc.is_null()
        && !(*(*ctxt).myDoc).intSubset.is_null()
    {
        if (*ctxt).replaceEntities == 0 {
            dup = xmlSAX2DecodeAttrEntities(ctxt, value, valueend);
            if dup.is_null() {
                if *valueend as c_int == 0 as c_int {
                    (*ctxt).valid &= xmlValidateOneAttribute(
                        &raw mut (*ctxt).vctxt,
                        (*ctxt).myDoc,
                        (*ctxt).node,
                        ret,
                        value,
                    );
                } else {
                    dup = xmlStrndup(
                        value,
                        valueend.offset_from(value) as c_long as c_int,
                    );
                    (*ctxt).valid &= xmlValidateOneAttribute(
                        &raw mut (*ctxt).vctxt,
                        (*ctxt).myDoc,
                        (*ctxt).node,
                        ret,
                        dup,
                    );
                }
            } else {
                if !(*ctxt).attsSpecial.is_null() {
                    let mut nvalnorm: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    let mut fn_0: [xmlChar; 50] = [0; 50];
                    let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    fullname = xmlBuildQName(
                        localname,
                        prefix,
                        &raw mut fn_0 as *mut xmlChar,
                        50 as c_int,
                    );
                    if !fullname.is_null() {
                        (*ctxt).vctxt.valid = 1 as c_int;
                        nvalnorm = xmlValidCtxtNormalizeAttributeValue(
                            &raw mut (*ctxt).vctxt,
                            (*ctxt).myDoc,
                            (*ctxt).node,
                            fullname,
                            dup,
                        );
                        if (*ctxt).vctxt.valid != 1 as c_int {
                            (*ctxt).valid = 0 as c_int;
                        }
                        if fullname != &raw mut fn_0 as *mut xmlChar
                            && fullname != localname as *mut xmlChar
                        {
                            xmlFree.expect("non-null function pointer")(
                                fullname as *mut c_void,
                            );
                        }
                        if !nvalnorm.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                dup as *mut c_void,
                            );
                            dup = nvalnorm;
                        }
                    }
                }
                (*ctxt).valid &= xmlValidateOneAttribute(
                    &raw mut (*ctxt).vctxt,
                    (*ctxt).myDoc,
                    (*ctxt).node,
                    ret,
                    dup,
                );
            }
        } else {
            dup = xmlStrndup(
                value,
                valueend.offset_from(value) as c_long as c_int,
            );
            (*ctxt).valid &= xmlValidateOneAttribute(
                &raw mut (*ctxt).vctxt,
                (*ctxt).myDoc,
                (*ctxt).node,
                ret,
                dup,
            );
        }
    } else if (*ctxt).loadsubset & XML_SKIP_IDS == 0 as c_int
        && ((*ctxt).replaceEntities == 0 as c_int
            && (*ctxt).external != 2 as c_int
            || (*ctxt).replaceEntities != 0 as c_int
                && (*ctxt).inSubset == 0 as c_int)
        && !(*ret).children.is_null()
        && (*(*ret).children).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
        && (*(*ret).children).next.is_null()
    {
        let mut content: *mut xmlChar = (*(*ret).children).content;
        if prefix == (*ctxt).str_xml
            && *localname.offset(0 as c_int as isize) as c_int
                == 'i' as i32
            && *localname.offset(1 as c_int as isize) as c_int
                == 'd' as i32
            && *localname.offset(2 as c_int as isize) as c_int
                == 0 as c_int
        {
            if xmlValidateNCName(content, 1 as c_int) != 0 as c_int {
                xmlErrValid(
                    ctxt,
                    XML_DTD_XMLID_VALUE,
                    b"xml:id : attribute value %s is not an NCName\n\0" as *const u8
                        as *const c_char,
                    content as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
            }
            xmlAddID(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
        } else if xmlIsID((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
            xmlAddID(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
        } else if xmlIsRef((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
            xmlAddRef(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
        }
    }
    if !dup.is_null() {
        xmlFree.expect("non-null function pointer")(dup as *mut c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartElementNs(
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
    let mut current_block: u64;
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut last: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut uri: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut pref: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut lname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    if ctx.is_null() {
        return;
    }
    if (*ctxt).validate != 0
        && (*(*ctxt).myDoc).extSubset.is_null()
        && ((*(*ctxt).myDoc).intSubset.is_null()
            || (*(*(*ctxt).myDoc).intSubset).notations.is_null()
                && (*(*(*ctxt).myDoc).intSubset).elements.is_null()
                && (*(*(*ctxt).myDoc).intSubset).attributes.is_null()
                && (*(*(*ctxt).myDoc).intSubset).entities.is_null())
    {
        xmlErrValid(
            ctxt,
            XML_DTD_NO_DTD,
            b"Validation failed: no DTD found !\0" as *const u8 as *const c_char,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
        );
        (*ctxt).validate = 0 as c_int;
    }
    if !prefix.is_null() && URI.is_null() {
        if (*ctxt).dictNames != 0 {
            let mut fullname: *const xmlChar = ::core::ptr::null::<xmlChar>();
            fullname = xmlDictQLookup((*ctxt).dict, prefix, localname);
            if !fullname.is_null() {
                localname = fullname;
            }
        } else {
            lname = xmlBuildQName(
                localname,
                prefix,
                ::core::ptr::null_mut::<xmlChar>(),
                0 as c_int,
            );
        }
    }
    if !(*ctxt).freeElems.is_null() {
        ret = (*ctxt).freeElems;
        (*ctxt).freeElems = (*ret).next as xmlNodePtr;
        (*ctxt).freeElemsNr -= 1;
        memset(
            ret as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<xmlNode>() as size_t,
        );
        (*ret).doc = (*ctxt).myDoc as *mut _xmlDoc;
        (*ret).type_0 = XML_ELEMENT_NODE;
        if (*ctxt).dictNames != 0 {
            (*ret).name = localname;
        } else {
            if lname.is_null() {
                (*ret).name = xmlStrdup(localname);
            } else {
                (*ret).name = lname;
            }
            if (*ret).name.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2StartElementNs\0" as *const u8 as *const c_char,
                );
                xmlFree.expect("non-null function pointer")(ret as *mut c_void);
                return;
            }
        }
        if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
            (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret);
        }
    } else {
        if (*ctxt).dictNames != 0 {
            ret = xmlNewDocNodeEatName(
                (*ctxt).myDoc,
                ::core::ptr::null_mut::<xmlNs>(),
                localname as *mut xmlChar,
                ::core::ptr::null::<xmlChar>(),
            );
        } else if lname.is_null() {
            ret = xmlNewDocNode(
                (*ctxt).myDoc,
                ::core::ptr::null_mut::<xmlNs>(),
                localname,
                ::core::ptr::null::<xmlChar>(),
            );
        } else {
            ret = xmlNewDocNodeEatName(
                (*ctxt).myDoc,
                ::core::ptr::null_mut::<xmlNs>(),
                lname,
                ::core::ptr::null::<xmlChar>(),
            );
        }
        if ret.is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2StartElementNs\0" as *const u8 as *const c_char,
            );
            return;
        }
    }
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if ((*(*ctxt).input).line as c_uint) < USHRT_MAX as c_uint {
                (*ret).line = (*(*ctxt).input).line as c_ushort;
            } else {
                (*ret).line = USHRT_MAX as c_ushort;
            }
        }
    }
    i = 0 as c_int;
    j = 0 as c_int;
    while j < nb_namespaces {
        let fresh10 = i;
        i = i + 1;
        pref = *namespaces.offset(fresh10 as isize);
        let fresh11 = i;
        i = i + 1;
        uri = *namespaces.offset(fresh11 as isize);
        ns = xmlNewNs(::core::ptr::null_mut::<xmlNode>(), uri, pref);
        if !ns.is_null() {
            if last.is_null() {
                last = ns;
                (*ret).nsDef = last as *mut xmlNs;
            } else {
                (*last).next = ns as *mut _xmlNs;
                last = ns;
            }
            if !URI.is_null() && prefix == pref {
                (*ret).ns = ns as *mut xmlNs;
            }
            xmlParserNsUpdateSax(ctxt, pref, ns as *mut c_void);
            if (*ctxt).html == 0
                && (*ctxt).validate != 0
                && (*ctxt).wellFormed != 0
                && !(*ctxt).myDoc.is_null()
                && !(*(*ctxt).myDoc).intSubset.is_null()
            {
                (*ctxt).valid &= xmlValidateOneNamespace(
                    &raw mut (*ctxt).vctxt,
                    (*ctxt).myDoc,
                    ret,
                    prefix,
                    ns,
                    uri,
                );
            }
        }
        j += 1;
    }
    (*ctxt).nodemem = -(1 as c_int);
    parent = (*ctxt).node;
    if parent.is_null() {
        parent = (*ctxt).myDoc as xmlNodePtr;
    }
    if nodePush(ctxt, ret) < 0 as c_int {
        xmlUnlinkNode(ret);
        xmlFreeNode(ret);
        return;
    }
    xmlAddChild(parent, ret);
    if nb_defaulted != 0 as c_int
        && (*ctxt).loadsubset & XML_COMPLETE_ATTRS == 0 as c_int
    {
        nb_attributes -= nb_defaulted;
    }
    if !URI.is_null() && (*ret).ns.is_null() {
        (*ret).ns = xmlParserNsLookupSax(ctxt, prefix) as *mut xmlNs;
        if (*ret).ns.is_null()
            && xmlStrEqual(
                prefix,
                b"xml\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
        {
            (*ret).ns = xmlSearchNs((*ctxt).myDoc, ret, prefix) as *mut xmlNs;
        }
        if (*ret).ns.is_null() {
            ns = xmlNewNs(ret, ::core::ptr::null::<xmlChar>(), prefix);
            if ns.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2StartElementNs\0" as *const u8 as *const c_char,
                );
                return;
            }
            if !prefix.is_null() {
                xmlNsWarnMsg(
                    ctxt,
                    XML_NS_ERR_UNDEFINED_NAMESPACE,
                    b"Namespace prefix %s was not found\n\0" as *const u8
                        as *const c_char,
                    prefix,
                    ::core::ptr::null::<xmlChar>(),
                );
            } else {
                xmlNsWarnMsg(
                    ctxt,
                    XML_NS_ERR_UNDEFINED_NAMESPACE,
                    b"Namespace default prefix was not found\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
            }
        }
    }
    if nb_attributes > 0 as c_int {
        let mut prev: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
        j = 0 as c_int;
        i = 0 as c_int;
        while i < nb_attributes {
            let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
            if !(*attributes.offset((j + 1 as c_int) as isize)).is_null()
                && (*attributes.offset((j + 2 as c_int) as isize)).is_null()
            {
                if (*ctxt).dictNames != 0 {
                    let mut fullname_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
                    fullname_0 = xmlDictQLookup(
                        (*ctxt).dict,
                        *attributes.offset((j + 1 as c_int) as isize),
                        *attributes.offset(j as isize),
                    );
                    if !fullname_0.is_null() {
                        attr = xmlSAX2AttributeNs(
                            ctxt,
                            fullname_0,
                            ::core::ptr::null::<xmlChar>(),
                            *attributes.offset((j + 3 as c_int) as isize),
                            *attributes.offset((j + 4 as c_int) as isize),
                        );
                        current_block = 6693394777901144942;
                    } else {
                        current_block = 8723848109087415604;
                    }
                } else {
                    lname = xmlBuildQName(
                        *attributes.offset(j as isize),
                        *attributes.offset((j + 1 as c_int) as isize),
                        ::core::ptr::null_mut::<xmlChar>(),
                        0 as c_int,
                    );
                    if !lname.is_null() {
                        attr = xmlSAX2AttributeNs(
                            ctxt,
                            lname,
                            ::core::ptr::null::<xmlChar>(),
                            *attributes.offset((j + 3 as c_int) as isize),
                            *attributes.offset((j + 4 as c_int) as isize),
                        );
                        xmlFree.expect("non-null function pointer")(
                            lname as *mut c_void,
                        );
                        current_block = 6693394777901144942;
                    } else {
                        current_block = 8723848109087415604;
                    }
                }
            } else {
                current_block = 8723848109087415604;
            }
            match current_block {
                8723848109087415604 => {
                    attr = xmlSAX2AttributeNs(
                        ctxt,
                        *attributes.offset(j as isize),
                        *attributes.offset((j + 1 as c_int) as isize),
                        *attributes.offset((j + 3 as c_int) as isize),
                        *attributes.offset((j + 4 as c_int) as isize),
                    );
                }
                _ => {}
            }
            if !attr.is_null() {
                if prev.is_null() {
                    (*(*ctxt).node).properties = attr as *mut _xmlAttr;
                } else {
                    (*prev).next = attr as *mut _xmlAttr;
                    (*attr).prev = prev as *mut _xmlAttr;
                }
                prev = attr;
            }
            i += 1;
            j += 5 as c_int;
        }
    }
    if (*ctxt).validate != 0
        && (*ctxt).vctxt.flags & XML_VCTXT_DTD_VALIDATED == 0 as c_uint
    {
        let mut chk: c_int = 0;
        chk = xmlValidateDtdFinal(&raw mut (*ctxt).vctxt, (*ctxt).myDoc);
        if chk <= 0 as c_int {
            (*ctxt).valid = 0 as c_int;
        }
        if chk < 0 as c_int {
            (*ctxt).wellFormed = 0 as c_int;
        }
        (*ctxt).valid &= xmlValidateRoot(&raw mut (*ctxt).vctxt, (*ctxt).myDoc);
        (*ctxt).vctxt.flags |= XML_VCTXT_DTD_VALIDATED;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndElementNs(
    mut ctx: *mut c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    (*ctxt).nodemem = -(1 as c_int);
    if (*ctxt).validate != 0
        && (*ctxt).wellFormed != 0
        && !(*ctxt).myDoc.is_null()
        && !(*(*ctxt).myDoc).intSubset.is_null()
    {
        (*ctxt).valid &= xmlValidateOneElement(&raw mut (*ctxt).vctxt, (*ctxt).myDoc, (*ctxt).node);
    }
    nodePop(ctxt);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Reference(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ctx.is_null() {
        return;
    }
    ret = xmlNewReference((*ctxt).myDoc as *const xmlDoc, name);
    if xmlAddChild((*ctxt).node, ret).is_null() {
        xmlFreeNode(ret);
    }
}
unsafe extern "C" fn xmlSAX2Text(
    mut ctxt: xmlParserCtxtPtr,
    mut ch: *const xmlChar,
    mut len: c_int,
    mut type_0: xmlElementType,
) {
    let mut lastChild: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ctxt.is_null() {
        return;
    }
    if (*ctxt).node.is_null() {
        return;
    }
    lastChild = (*(*ctxt).node).last as xmlNodePtr;
    if lastChild.is_null() {
        if type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
        {
            lastChild = xmlSAX2TextNode(ctxt, ch, len);
        } else {
            lastChild = xmlNewCDataBlock((*ctxt).myDoc, ch, len);
        }
        if !lastChild.is_null() {
            (*(*ctxt).node).children = lastChild as *mut _xmlNode;
            (*(*ctxt).node).last = lastChild as *mut _xmlNode;
            (*lastChild).parent = (*ctxt).node as *mut _xmlNode;
            (*lastChild).doc = (*(*ctxt).node).doc;
            (*ctxt).nodelen = len;
            (*ctxt).nodemem = len + 1 as c_int;
        } else {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2Characters\0" as *const u8 as *const c_char,
            );
            return;
        }
    } else {
        let mut coalesceText: c_int = (!lastChild.is_null()
            && (*lastChild).type_0 as c_uint == type_0 as c_uint
            && (type_0 as c_uint
                != XML_TEXT_NODE as c_int as c_uint
                || (*lastChild).name == &raw const xmlStringText as *const xmlChar))
            as c_int;
        if coalesceText != 0 && (*ctxt).nodemem != 0 as c_int {
            if (*lastChild).content == &raw mut (*lastChild).properties as *mut xmlChar {
                (*lastChild).content = xmlStrdup((*lastChild).content);
                (*lastChild).properties = ::core::ptr::null_mut::<_xmlAttr>();
            } else if (*ctxt).nodemem == (*ctxt).nodelen + 1 as c_int
                && xmlDictOwns((*ctxt).dict, (*lastChild).content) != 0
            {
                (*lastChild).content = xmlStrdup((*lastChild).content);
            }
            if (*lastChild).content.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2Characters: xmlStrdup returned NULL\0" as *const u8
                        as *const c_char,
                );
                return;
            }
            if (*ctxt).nodelen > INT_MAX - len {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2Characters overflow prevented\0" as *const u8
                        as *const c_char,
                );
                return;
            }
            if (*ctxt).nodelen + len > XML_MAX_TEXT_LENGTH
                && (*ctxt).options & XML_PARSE_HUGE as c_int == 0 as c_int
            {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2Characters: huge text node\0" as *const u8
                        as *const c_char,
                );
                return;
            }
            if (*ctxt).nodelen + len >= (*ctxt).nodemem {
                let mut newbuf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                let mut size: c_int = 0;
                size = if (*ctxt).nodemem > INT_MAX - len {
                    INT_MAX
                } else {
                    (*ctxt).nodemem + len
                };
                size = if size > INT_MAX / 2 as c_int {
                    INT_MAX
                } else {
                    size * 2 as c_int
                };
                newbuf = xmlRealloc.expect("non-null function pointer")(
                    (*lastChild).content as *mut c_void,
                    size as size_t,
                ) as *mut xmlChar;
                if newbuf.is_null() {
                    xmlSAX2ErrMemory(
                        ctxt,
                        b"xmlSAX2Characters\0" as *const u8 as *const c_char,
                    );
                    return;
                }
                (*ctxt).nodemem = size;
                (*lastChild).content = newbuf;
            }
            memcpy(
                (*lastChild).content.offset((*ctxt).nodelen as isize) as *mut xmlChar
                    as *mut c_void,
                ch as *const c_void,
                len as size_t,
            );
            (*ctxt).nodelen += len;
            *(*lastChild).content.offset((*ctxt).nodelen as isize) = 0 as xmlChar;
        } else if coalesceText != 0 {
            if xmlTextConcat(lastChild, ch, len) != 0 {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2Characters\0" as *const u8 as *const c_char,
                );
            }
            if !(*(*ctxt).node).children.is_null() {
                (*ctxt).nodelen = xmlStrlen((*lastChild).content);
                (*ctxt).nodemem = (*ctxt).nodelen + 1 as c_int;
            }
        } else {
            if type_0 as c_uint
                == XML_TEXT_NODE as c_int as c_uint
            {
                lastChild = xmlSAX2TextNode(ctxt, ch, len);
                if !lastChild.is_null() {
                    (*lastChild).doc = (*ctxt).myDoc as *mut _xmlDoc;
                }
            } else {
                lastChild = xmlNewCDataBlock((*ctxt).myDoc, ch, len);
            }
            if !lastChild.is_null() {
                xmlAddChild((*ctxt).node, lastChild);
                if !(*(*ctxt).node).children.is_null() {
                    (*ctxt).nodelen = len;
                    (*ctxt).nodemem = len + 1 as c_int;
                }
            }
        }
    }
    if !lastChild.is_null()
        && type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
        && (*ctxt).linenumbers != 0
        && !(*ctxt).input.is_null()
    {
        if ((*(*ctxt).input).line as c_uint) < USHRT_MAX as c_uint {
            (*lastChild).line = (*(*ctxt).input).line as c_ushort;
        } else {
            (*lastChild).line = USHRT_MAX as c_ushort;
            if (*ctxt).options & XML_PARSE_BIG_LINES as c_int != 0 {
                (*lastChild).psvi = (*(*ctxt).input).line as ptrdiff_t as *mut c_void;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Characters(
    mut ctx: *mut c_void,
    mut ch: *const xmlChar,
    mut len: c_int,
) {
    xmlSAX2Text(ctx as xmlParserCtxtPtr, ch, len, XML_TEXT_NODE);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2IgnorableWhitespace(
    mut ctx: *mut c_void,
    mut ch: *const xmlChar,
    mut len: c_int,
) {
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ProcessingInstruction(
    mut ctx: *mut c_void,
    mut target: *const xmlChar,
    mut data: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ctx.is_null() {
        return;
    }
    parent = (*ctxt).node;
    ret = xmlNewDocPI((*ctxt).myDoc, target, data);
    if ret.is_null() {
        return;
    }
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if ((*(*ctxt).input).line as c_uint) < USHRT_MAX as c_uint {
                (*ret).line = (*(*ctxt).input).line as c_ushort;
            } else {
                (*ret).line = USHRT_MAX as c_ushort;
            }
        }
    }
    if (*ctxt).inSubset == 1 as c_int {
        xmlAddChild((*(*ctxt).myDoc).intSubset as xmlNodePtr, ret);
        return;
    } else if (*ctxt).inSubset == 2 as c_int {
        xmlAddChild((*(*ctxt).myDoc).extSubset as xmlNodePtr, ret);
        return;
    }
    if parent.is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
        return;
    }
    if (*parent).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
    {
        xmlAddChild(parent, ret);
    } else {
        xmlAddSibling(parent, ret);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Comment(
    mut ctx: *mut c_void,
    mut value: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ctx.is_null() {
        return;
    }
    parent = (*ctxt).node;
    ret = xmlNewDocComment((*ctxt).myDoc, value);
    if ret.is_null() {
        return;
    }
    if (*ctxt).linenumbers != 0 {
        if !(*ctxt).input.is_null() {
            if ((*(*ctxt).input).line as c_uint) < USHRT_MAX as c_uint {
                (*ret).line = (*(*ctxt).input).line as c_ushort;
            } else {
                (*ret).line = USHRT_MAX as c_ushort;
            }
        }
    }
    if (*ctxt).inSubset == 1 as c_int {
        xmlAddChild((*(*ctxt).myDoc).intSubset as xmlNodePtr, ret);
        return;
    } else if (*ctxt).inSubset == 2 as c_int {
        xmlAddChild((*(*ctxt).myDoc).extSubset as xmlNodePtr, ret);
        return;
    }
    if parent.is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
        return;
    }
    if (*parent).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
    {
        xmlAddChild(parent, ret);
    } else {
        xmlAddSibling(parent, ret);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2CDataBlock(
    mut ctx: *mut c_void,
    mut value: *const xmlChar,
    mut len: c_int,
) {
    xmlSAX2Text(ctx as xmlParserCtxtPtr, value, len, XML_CDATA_SECTION_NODE);
}
static mut xmlSAX2DefaultVersionValue: c_int = 2 as c_int;
#[no_mangle]
pub unsafe extern "C" fn xmlSAXDefaultVersion(
    mut version: c_int,
) -> c_int {
    let mut ret: c_int = xmlSAX2DefaultVersionValue;
    if version != 1 as c_int && version != 2 as c_int {
        return -(1 as c_int);
    }
    xmlSAX2DefaultVersionValue = version;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXVersion(
    mut hdlr: *mut xmlSAXHandler,
    mut version: c_int,
) -> c_int {
    if hdlr.is_null() {
        return -(1 as c_int);
    }
    if version == 2 as c_int {
        (*hdlr).startElementNs = Some(
            xmlSAX2StartElementNs
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
        (*hdlr).endElementNs = Some(
            xmlSAX2EndElementNs
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ) as endElementNsSAX2Func;
        (*hdlr).serror = None;
        (*hdlr).initialized = XML_SAX2_MAGIC;
    } else if version == 1 as c_int {
        (*hdlr).initialized = 1 as c_uint;
    } else {
        return -(1 as c_int);
    }
    (*hdlr).startElement = Some(
        xmlSAX2StartElement
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*hdlr).endElement = Some(
        xmlSAX2EndElement as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    (*hdlr).internalSubset = Some(
        xmlSAX2InternalSubset
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as internalSubsetSAXFunc;
    (*hdlr).externalSubset = Some(
        xmlSAX2ExternalSubset
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as externalSubsetSAXFunc;
    (*hdlr).isStandalone = Some(
        xmlSAX2IsStandalone as unsafe extern "C" fn(*mut c_void) -> c_int,
    ) as isStandaloneSAXFunc;
    (*hdlr).hasInternalSubset = Some(
        xmlSAX2HasInternalSubset
            as unsafe extern "C" fn(*mut c_void) -> c_int,
    ) as hasInternalSubsetSAXFunc;
    (*hdlr).hasExternalSubset = Some(
        xmlSAX2HasExternalSubset
            as unsafe extern "C" fn(*mut c_void) -> c_int,
    ) as hasExternalSubsetSAXFunc;
    (*hdlr).resolveEntity = Some(
        xmlSAX2ResolveEntity
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *const xmlChar,
            ) -> xmlParserInputPtr,
    ) as resolveEntitySAXFunc;
    (*hdlr).getEntity = Some(
        xmlSAX2GetEntity
            as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
    ) as getEntitySAXFunc;
    (*hdlr).getParameterEntity = Some(
        xmlSAX2GetParameterEntity
            as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
    ) as getParameterEntitySAXFunc;
    (*hdlr).entityDecl = Some(
        xmlSAX2EntityDecl
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
                *const xmlChar,
                *const xmlChar,
                *mut xmlChar,
            ) -> (),
    ) as entityDeclSAXFunc;
    (*hdlr).attributeDecl = Some(
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
    ) as attributeDeclSAXFunc;
    (*hdlr).elementDecl = Some(
        xmlSAX2ElementDecl
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
                xmlElementContentPtr,
            ) -> (),
    ) as elementDeclSAXFunc;
    (*hdlr).notationDecl = Some(
        xmlSAX2NotationDecl
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as notationDeclSAXFunc;
    (*hdlr).unparsedEntityDecl = Some(
        xmlSAX2UnparsedEntityDecl
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as unparsedEntityDeclSAXFunc;
    (*hdlr).setDocumentLocator = Some(
        xmlSAX2SetDocumentLocator
            as unsafe extern "C" fn(*mut c_void, xmlSAXLocatorPtr) -> (),
    ) as setDocumentLocatorSAXFunc;
    (*hdlr).startDocument =
        Some(xmlSAX2StartDocument as unsafe extern "C" fn(*mut c_void) -> ())
            as startDocumentSAXFunc;
    (*hdlr).endDocument =
        Some(xmlSAX2EndDocument as unsafe extern "C" fn(*mut c_void) -> ())
            as endDocumentSAXFunc;
    (*hdlr).reference = Some(
        xmlSAX2Reference as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
    ) as referenceSAXFunc;
    (*hdlr).characters = Some(
        xmlSAX2Characters
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*hdlr).cdataBlock = Some(
        xmlSAX2CDataBlock
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as cdataBlockSAXFunc;
    (*hdlr).ignorableWhitespace = Some(
        xmlSAX2Characters
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*hdlr).processingInstruction = Some(
        xmlSAX2ProcessingInstruction
            as unsafe extern "C" fn(*mut c_void, *const xmlChar, *const xmlChar) -> (),
    ) as processingInstructionSAXFunc;
    (*hdlr).comment = Some(
        xmlSAX2Comment as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
    ) as commentSAXFunc;
    (*hdlr).warning = Some(
        xmlParserWarning
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    ) as warningSAXFunc;
    (*hdlr).error = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    ) as errorSAXFunc;
    (*hdlr).fatalError = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    ) as fatalErrorSAXFunc;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InitDefaultSAXHandler(
    mut hdlr: *mut xmlSAXHandler,
    mut warning: c_int,
) {
    if hdlr.is_null() || (*hdlr).initialized != 0 as c_uint {
        return;
    }
    xmlSAXVersion(hdlr, xmlSAX2DefaultVersionValue);
    if warning == 0 as c_int {
        (*hdlr).warning = None;
    } else {
        (*hdlr).warning = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlDefaultSAXHandlerInit() {}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InitHtmlDefaultSAXHandler(mut hdlr: *mut xmlSAXHandler) {
    if hdlr.is_null() || (*hdlr).initialized != 0 as c_uint {
        return;
    }
    (*hdlr).internalSubset = Some(
        xmlSAX2InternalSubset
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *const xmlChar,
                *const xmlChar,
            ) -> (),
    ) as internalSubsetSAXFunc;
    (*hdlr).externalSubset = None;
    (*hdlr).isStandalone = None;
    (*hdlr).hasInternalSubset = None;
    (*hdlr).hasExternalSubset = None;
    (*hdlr).resolveEntity = None;
    (*hdlr).getEntity = Some(
        xmlSAX2GetEntity
            as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
    ) as getEntitySAXFunc;
    (*hdlr).getParameterEntity = None;
    (*hdlr).entityDecl = None;
    (*hdlr).attributeDecl = None;
    (*hdlr).elementDecl = None;
    (*hdlr).notationDecl = None;
    (*hdlr).unparsedEntityDecl = None;
    (*hdlr).setDocumentLocator = Some(
        xmlSAX2SetDocumentLocator
            as unsafe extern "C" fn(*mut c_void, xmlSAXLocatorPtr) -> (),
    ) as setDocumentLocatorSAXFunc;
    (*hdlr).startDocument =
        Some(xmlSAX2StartDocument as unsafe extern "C" fn(*mut c_void) -> ())
            as startDocumentSAXFunc;
    (*hdlr).endDocument =
        Some(xmlSAX2EndDocument as unsafe extern "C" fn(*mut c_void) -> ())
            as endDocumentSAXFunc;
    (*hdlr).startElement = Some(
        xmlSAX2StartElement
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*hdlr).endElement = Some(
        xmlSAX2EndElement as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    (*hdlr).reference = None;
    (*hdlr).characters = Some(
        xmlSAX2Characters
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*hdlr).cdataBlock = Some(
        xmlSAX2CDataBlock
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as cdataBlockSAXFunc;
    (*hdlr).ignorableWhitespace = Some(
        xmlSAX2IgnorableWhitespace
            as unsafe extern "C" fn(
                *mut c_void,
                *const xmlChar,
                c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*hdlr).processingInstruction = Some(
        xmlSAX2ProcessingInstruction
            as unsafe extern "C" fn(*mut c_void, *const xmlChar, *const xmlChar) -> (),
    ) as processingInstructionSAXFunc;
    (*hdlr).comment = Some(
        xmlSAX2Comment as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
    ) as commentSAXFunc;
    (*hdlr).warning = Some(
        xmlParserWarning
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    ) as warningSAXFunc;
    (*hdlr).error = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    ) as errorSAXFunc;
    (*hdlr).fatalError = Some(
        xmlParserError
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    ) as fatalErrorSAXFunc;
    (*hdlr).initialized = 1 as c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn htmlDefaultSAXHandlerInit() {}

pub const XML_VCTXT_DTD_VALIDATED: c_uint =
    (1 as c_uint) << 0 as c_int;

pub const __SHRT_MAX__: c_int = 32767 as c_int;

