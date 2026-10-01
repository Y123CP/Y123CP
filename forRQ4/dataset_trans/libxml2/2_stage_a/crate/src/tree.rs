use core::ffi::*;
use crate::src::globals::__xmlBufferAllocScheme;
use crate::src::globals::__xmlDefaultBufferSize;
use crate::src::buf::xmlBufAdd;
use crate::src::buf::xmlBufBackToBuffer;
use crate::src::buf::xmlBufCat;
use crate::src::buf::xmlBufCreate;
use crate::src::buf::xmlBufCreateSize;
use crate::src::buf::xmlBufDetach;
use crate::src::buf::xmlBufFree;
use crate::src::buf::xmlBufFromBuffer;
use crate::src::buf::xmlBufIsEmpty;
use crate::src::buf::xmlBufSetAllocationScheme;
use crate::src::uri::xmlBuildURI;
use crate::src::valid::xmlCopyAttributeTable;
use crate::src::parserInternals::xmlCopyCharMultiByte;
use crate::src::valid::xmlCopyElementTable;
use crate::src::entities::xmlCopyEntitiesTable;
use crate::src::valid::xmlCopyNotationTable;
use crate::src::dict::xmlDictFree;
use crate::src::dict::xmlDictLookup;
use crate::src::dict::xmlDictOwns;
use crate::src::valid::xmlFreeAttributeTable;
use crate::src::valid::xmlFreeElementTable;
use crate::src::entities::xmlFreeEntitiesTable;
use crate::src::valid::xmlFreeIDTable;
use crate::src::valid::xmlFreeNotationTable;
use crate::src::valid::xmlFreeRefTable;
use crate::src::hash::xmlHashLookup;
use crate::src::hash::xmlHashRemoveEntry;
use crate::src::uri::xmlPathToURI;
use crate::src::xmlstring::xmlStrEqual;
use crate::src::xmlstring::xmlStrcasecmp;
use crate::src::xmlstring::xmlStrcat;
use crate::src::xmlstring::xmlStrchr;
use crate::src::xmlstring::xmlStrdup;
use crate::src::xmlstring::xmlStrncat;
use crate::src::xmlstring::xmlStrncatNew;
use crate::src::xmlstring::xmlStrncmp;
use crate::src::xmlstring::xmlStrndup;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
pub use crate::src::valid::_xmlValidState;
pub use crate::src::dict::_xmlDict;
pub use crate::src::hash::_xmlHashTable;
pub use crate::src::buf::_xmlBuf;
pub use crate::src::xmlregexp::_xmlRegexp;
pub use crate::src::parser::_xmlAttrHashBucket;
pub use crate::src::parser::_xmlParserNsData;
pub use crate::src::parser::_xmlStartTag;
pub use crate::src::xmlregexp::_xmlAutomataState;
pub use crate::src::xmlregexp::_xmlAutomata;
extern "C" {
    fn xmlAddID(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        value: *const xmlChar,
        attr: xmlAttrPtr,
    ) -> xmlIDPtr;
    fn xmlIsID(doc: xmlDocPtr, elem: xmlNodePtr, attr: xmlAttrPtr) -> c_int;
    fn xmlRemoveID(doc: xmlDocPtr, attr: xmlAttrPtr) -> c_int;
    fn xmlGetDtdAttrDesc(
        dtd: xmlDtdPtr,
        elem: *const xmlChar,
        name: *const xmlChar,
    ) -> xmlAttributePtr;
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
    fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc;
    fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc;
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlEncodeEntitiesReentrant(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
    fn xmlEncodeSpecialChars(doc: *const xmlDoc, input: *const xmlChar) -> *mut xmlChar;
    fn xmlCharInRange(
        val: c_uint,
        group: *const xmlChRangeGroup,
    ) -> c_int;
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    static xmlIsCombiningGroup: xmlChRangeGroup;
    static xmlIsDigitGroup: xmlChRangeGroup;
    static xmlIsExtenderGroup: xmlChRangeGroup;
    fn xmlStringCurrentChar(
        ctxt: xmlParserCtxtPtr,
        cur: *const xmlChar,
        len: *mut c_int,
    ) -> c_int;
    fn xmlEncodeAttributeEntities(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
    fn __xmlSimpleError(
        domain: c_int,
        code: c_int,
        node: *mut _xmlNode,
        msg: *const c_char,
        extra: *const c_char,
    );
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
pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const XML_DOC_HTML: C2RustUnnamed_htdd24ee73 = 128;
pub const XML_DOC_INTERNAL: C2RustUnnamed_htdd24ee73 = 64;
pub const XML_DOC_USERBUILT: C2RustUnnamed_htdd24ee73 = 32;
pub const XML_DOC_XINCLUDE: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_DOC_DTDVALID: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_DOC_OLD10: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_DOC_NSVALID: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_DOC_WELLFORMED: C2RustUnnamed_htdd24ee73 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDOMWrapCtxt {
    pub _private: *mut c_void,
    pub type_0: c_int,
    pub namespaceMap: *mut c_void,
    pub getNsForNodeFunc: xmlDOMWrapAcquireNsFunction,
}
pub type xmlDOMWrapAcquireNsFunction = Option<
    unsafe extern "C" fn(xmlDOMWrapCtxtPtr, xmlNodePtr, *const xmlChar, *const xmlChar) -> xmlNsPtr,
>;
pub type xmlDOMWrapCtxtPtr = *mut xmlDOMWrapCtxt;
pub type xmlDOMWrapCtxt = _xmlDOMWrapCtxt;
pub type xmlRegisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlDeregisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: c_int,
    pub nbLongRange: c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}

pub const XML_ERR_NO_MEMORY: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_FROM_TREE: C2RustUnnamed_htdd24ee73 = 2;
pub type xmlEntitiesTablePtr = *mut xmlEntitiesTable;
pub type xmlEntitiesTable = _xmlHashTable;
pub type xmlAttributeTablePtr = *mut xmlAttributeTable;
pub type xmlAttributeTable = _xmlHashTable;
pub type xmlElementTablePtr = *mut xmlElementTable;
pub type xmlElementTable = _xmlHashTable;
pub type xmlNotationTablePtr = *mut xmlNotationTable;
pub type xmlNotationTable = _xmlHashTable;

pub type xmlRefTablePtr = *mut xmlRefTable;
pub type xmlRefTable = _xmlHashTable;
pub type xmlIDTablePtr = *mut xmlIDTable;
pub type xmlIDTable = _xmlHashTable;
pub const XML_CHAR_ENCODING_UTF8: C2RustUnnamed_htaaa91a68 = 1;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_htdd24ee73 = 1302;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_htdd24ee73 = 1303;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_htdd24ee73 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_htdd24ee73 = 1300;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlNsMapPtr = *mut xmlNsMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlNsMap {
    pub first: xmlNsMapItemPtr,
    pub last: xmlNsMapItemPtr,
    pub pool: xmlNsMapItemPtr,
}
pub type xmlNsMapItemPtr = *mut xmlNsMapItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlNsMapItem {
    pub next: xmlNsMapItemPtr,
    pub prev: xmlNsMapItemPtr,
    pub oldNs: xmlNsPtr,
    pub newNs: xmlNsPtr,
    pub shadowDepth: c_int,
    pub depth: c_int,
}
pub const XML_DOM_RECONNS_REMOVEREDUND: xmlDOMReconcileNSOptions = 1;
pub type xmlDOMReconcileNSOptions = c_uint;
pub const XML_FROM_URI: C2RustUnnamed_htdd24ee73 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_htdd24ee73 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_htdd24ee73 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_htdd24ee73 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_htdd24ee73 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_htdd24ee73 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_htdd24ee73 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_htdd24ee73 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_htdd24ee73 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_htdd24ee73 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_htdd24ee73 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_htdd24ee73 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_htdd24ee73 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_htdd24ee73 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_htdd24ee73 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_htdd24ee73 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_htdd24ee73 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_htdd24ee73 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_htdd24ee73 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_htdd24ee73 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_htdd24ee73 = 9;
pub const XML_FROM_IO: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed_htdd24ee73 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_htdd24ee73 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_htdd24ee73 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_htdd24ee73 = 3;
pub const XML_FROM_PARSER: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_htdd24ee73 = 0;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_htdd24ee73 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_htdd24ee73 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_htdd24ee73 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_htdd24ee73 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_htdd24ee73 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_htdd24ee73 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_htdd24ee73 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_htdd24ee73 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_htdd24ee73 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_htdd24ee73 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_htdd24ee73 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_htdd24ee73 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_htdd24ee73 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_htdd24ee73 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_htdd24ee73 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_htdd24ee73 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_htdd24ee73 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_htdd24ee73 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_htdd24ee73 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_htdd24ee73 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_htdd24ee73 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_htdd24ee73 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_htdd24ee73 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_htdd24ee73 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_htdd24ee73 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_htdd24ee73 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_htdd24ee73 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_htdd24ee73 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_htdd24ee73 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_htdd24ee73 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_htdd24ee73 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_htdd24ee73 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_htdd24ee73 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_htdd24ee73 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_htdd24ee73 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_htdd24ee73 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_htdd24ee73 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_htdd24ee73 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_htdd24ee73 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_htdd24ee73 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_htdd24ee73 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_htdd24ee73 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_htdd24ee73 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_htdd24ee73 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_htdd24ee73 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_htdd24ee73 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_htdd24ee73 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_htdd24ee73 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_htdd24ee73 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_htdd24ee73 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_htdd24ee73 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_htdd24ee73 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_htdd24ee73 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_htdd24ee73 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_htdd24ee73 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_htdd24ee73 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_htdd24ee73 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_htdd24ee73 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_htdd24ee73 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_htdd24ee73 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_htdd24ee73 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_htdd24ee73 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_htdd24ee73 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_htdd24ee73 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_htdd24ee73 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_htdd24ee73 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_htdd24ee73 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_htdd24ee73 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_htdd24ee73 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_htdd24ee73 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_htdd24ee73 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_htdd24ee73 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_htdd24ee73 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_htdd24ee73 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_htdd24ee73 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_htdd24ee73 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_htdd24ee73 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_htdd24ee73 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_htdd24ee73 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_htdd24ee73 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_htdd24ee73 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_htdd24ee73 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_htdd24ee73 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_htdd24ee73 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_htdd24ee73 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_htdd24ee73 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_htdd24ee73 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_htdd24ee73 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_htdd24ee73 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_htdd24ee73 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_htdd24ee73 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_htdd24ee73 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_htdd24ee73 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_htdd24ee73 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_htdd24ee73 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_htdd24ee73 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_htdd24ee73 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_htdd24ee73 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_htdd24ee73 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_htdd24ee73 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_htdd24ee73 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_htdd24ee73 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_htdd24ee73 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_htdd24ee73 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_htdd24ee73 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_htdd24ee73 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_htdd24ee73 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_htdd24ee73 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_htdd24ee73 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_htdd24ee73 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_htdd24ee73 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_htdd24ee73 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_htdd24ee73 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_htdd24ee73 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_htdd24ee73 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_htdd24ee73 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_htdd24ee73 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_htdd24ee73 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_htdd24ee73 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_htdd24ee73 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_htdd24ee73 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_htdd24ee73 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_htdd24ee73 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_htdd24ee73 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_htdd24ee73 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_htdd24ee73 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_htdd24ee73 = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_htdd24ee73 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_htdd24ee73 = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_htdd24ee73 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_htdd24ee73 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_htdd24ee73 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_htdd24ee73 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_htdd24ee73 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_htdd24ee73 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_htdd24ee73 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_htdd24ee73 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_htdd24ee73 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_htdd24ee73 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_htdd24ee73 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_htdd24ee73 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_htdd24ee73 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_htdd24ee73 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_htdd24ee73 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_htdd24ee73 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_htdd24ee73 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_htdd24ee73 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_htdd24ee73 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_htdd24ee73 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_htdd24ee73 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_htdd24ee73 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_htdd24ee73 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_htdd24ee73 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_htdd24ee73 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_htdd24ee73 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_htdd24ee73 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_htdd24ee73 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_htdd24ee73 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_htdd24ee73 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_htdd24ee73 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_htdd24ee73 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_htdd24ee73 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_htdd24ee73 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_htdd24ee73 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_htdd24ee73 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_htdd24ee73 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_htdd24ee73 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_htdd24ee73 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_htdd24ee73 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_htdd24ee73 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_htdd24ee73 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_htdd24ee73 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_htdd24ee73 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_htdd24ee73 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_htdd24ee73 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_htdd24ee73 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_htdd24ee73 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_htdd24ee73 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_htdd24ee73 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_htdd24ee73 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_htdd24ee73 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_htdd24ee73 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_htdd24ee73 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_htdd24ee73 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_htdd24ee73 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_htdd24ee73 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_htdd24ee73 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_htdd24ee73 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_htdd24ee73 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_htdd24ee73 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_htdd24ee73 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_htdd24ee73 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_htdd24ee73 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_htdd24ee73 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_htdd24ee73 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_htdd24ee73 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_htdd24ee73 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_htdd24ee73 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_htdd24ee73 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_htdd24ee73 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_htdd24ee73 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_htdd24ee73 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_htdd24ee73 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_htdd24ee73 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_htdd24ee73 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_htdd24ee73 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_htdd24ee73 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_htdd24ee73 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_htdd24ee73 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_htdd24ee73 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_htdd24ee73 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_htdd24ee73 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_htdd24ee73 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_htdd24ee73 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_htdd24ee73 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_htdd24ee73 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_htdd24ee73 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_htdd24ee73 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_htdd24ee73 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_htdd24ee73 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_htdd24ee73 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_htdd24ee73 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_htdd24ee73 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_htdd24ee73 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_htdd24ee73 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_htdd24ee73 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_htdd24ee73 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_htdd24ee73 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_htdd24ee73 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_htdd24ee73 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_htdd24ee73 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_htdd24ee73 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_htdd24ee73 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_htdd24ee73 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_htdd24ee73 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_htdd24ee73 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_htdd24ee73 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_htdd24ee73 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_htdd24ee73 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_htdd24ee73 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_htdd24ee73 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_htdd24ee73 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_htdd24ee73 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_htdd24ee73 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_htdd24ee73 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_htdd24ee73 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_htdd24ee73 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_htdd24ee73 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_htdd24ee73 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_htdd24ee73 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_htdd24ee73 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_htdd24ee73 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_htdd24ee73 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_htdd24ee73 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_htdd24ee73 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_htdd24ee73 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_htdd24ee73 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_htdd24ee73 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_htdd24ee73 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_htdd24ee73 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_htdd24ee73 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_htdd24ee73 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_htdd24ee73 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_htdd24ee73 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_htdd24ee73 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_htdd24ee73 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_htdd24ee73 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_htdd24ee73 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_htdd24ee73 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_htdd24ee73 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_htdd24ee73 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_htdd24ee73 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_htdd24ee73 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_htdd24ee73 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_htdd24ee73 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_htdd24ee73 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_htdd24ee73 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_htdd24ee73 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_htdd24ee73 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_htdd24ee73 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_htdd24ee73 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_htdd24ee73 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_htdd24ee73 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_htdd24ee73 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_htdd24ee73 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_htdd24ee73 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_htdd24ee73 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_htdd24ee73 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_htdd24ee73 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_htdd24ee73 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_htdd24ee73 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_htdd24ee73 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_htdd24ee73 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_htdd24ee73 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_htdd24ee73 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_htdd24ee73 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_htdd24ee73 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_htdd24ee73 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_htdd24ee73 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_htdd24ee73 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_htdd24ee73 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_htdd24ee73 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_htdd24ee73 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_htdd24ee73 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_htdd24ee73 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_htdd24ee73 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_htdd24ee73 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_htdd24ee73 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_htdd24ee73 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_htdd24ee73 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_htdd24ee73 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_htdd24ee73 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_htdd24ee73 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_htdd24ee73 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_htdd24ee73 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_htdd24ee73 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_htdd24ee73 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_htdd24ee73 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_htdd24ee73 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_htdd24ee73 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_htdd24ee73 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_htdd24ee73 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_htdd24ee73 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_htdd24ee73 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_htdd24ee73 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_htdd24ee73 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_htdd24ee73 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_htdd24ee73 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_htdd24ee73 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_htdd24ee73 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_htdd24ee73 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_htdd24ee73 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_htdd24ee73 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_htdd24ee73 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_htdd24ee73 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_htdd24ee73 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_htdd24ee73 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_htdd24ee73 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_htdd24ee73 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_htdd24ee73 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_htdd24ee73 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_htdd24ee73 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_htdd24ee73 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_htdd24ee73 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_htdd24ee73 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_htdd24ee73 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_htdd24ee73 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_htdd24ee73 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_htdd24ee73 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_htdd24ee73 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_htdd24ee73 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_htdd24ee73 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_htdd24ee73 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_htdd24ee73 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_htdd24ee73 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_htdd24ee73 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_htdd24ee73 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_htdd24ee73 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_htdd24ee73 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_htdd24ee73 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_htdd24ee73 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_htdd24ee73 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_htdd24ee73 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_htdd24ee73 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_htdd24ee73 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_htdd24ee73 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_htdd24ee73 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_htdd24ee73 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_htdd24ee73 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_htdd24ee73 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_htdd24ee73 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_htdd24ee73 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_htdd24ee73 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_htdd24ee73 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_htdd24ee73 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_htdd24ee73 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_htdd24ee73 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_htdd24ee73 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_htdd24ee73 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_htdd24ee73 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_htdd24ee73 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_htdd24ee73 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_htdd24ee73 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_htdd24ee73 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_htdd24ee73 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_htdd24ee73 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_htdd24ee73 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_htdd24ee73 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_htdd24ee73 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_htdd24ee73 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_htdd24ee73 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_htdd24ee73 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_htdd24ee73 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_htdd24ee73 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_htdd24ee73 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_htdd24ee73 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_htdd24ee73 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_htdd24ee73 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_htdd24ee73 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_htdd24ee73 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_htdd24ee73 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_htdd24ee73 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_htdd24ee73 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_htdd24ee73 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_htdd24ee73 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_htdd24ee73 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_htdd24ee73 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_htdd24ee73 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_htdd24ee73 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_htdd24ee73 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_htdd24ee73 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_htdd24ee73 = 1400;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_htdd24ee73 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_htdd24ee73 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_htdd24ee73 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_htdd24ee73 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_htdd24ee73 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_htdd24ee73 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_htdd24ee73 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_htdd24ee73 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_htdd24ee73 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_htdd24ee73 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_htdd24ee73 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_htdd24ee73 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_htdd24ee73 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_htdd24ee73 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_htdd24ee73 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_htdd24ee73 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_htdd24ee73 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_htdd24ee73 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_htdd24ee73 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_htdd24ee73 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_htdd24ee73 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_htdd24ee73 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_htdd24ee73 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_htdd24ee73 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_htdd24ee73 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_htdd24ee73 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_htdd24ee73 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_htdd24ee73 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_htdd24ee73 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_htdd24ee73 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_htdd24ee73 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_htdd24ee73 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_htdd24ee73 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_htdd24ee73 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_htdd24ee73 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_htdd24ee73 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_htdd24ee73 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_htdd24ee73 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_htdd24ee73 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_htdd24ee73 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_htdd24ee73 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_htdd24ee73 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_htdd24ee73 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_htdd24ee73 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_htdd24ee73 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_htdd24ee73 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_htdd24ee73 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_htdd24ee73 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_htdd24ee73 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_htdd24ee73 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_htdd24ee73 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_htdd24ee73 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_htdd24ee73 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_htdd24ee73 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_htdd24ee73 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_htdd24ee73 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_htdd24ee73 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_htdd24ee73 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_htdd24ee73 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_htdd24ee73 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_htdd24ee73 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_htdd24ee73 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_htdd24ee73 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_htdd24ee73 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_htdd24ee73 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_htdd24ee73 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_htdd24ee73 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_htdd24ee73 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_htdd24ee73 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_htdd24ee73 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_htdd24ee73 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_htdd24ee73 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_htdd24ee73 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_htdd24ee73 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_htdd24ee73 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_htdd24ee73 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_htdd24ee73 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_htdd24ee73 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_htdd24ee73 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_htdd24ee73 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_htdd24ee73 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_htdd24ee73 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_htdd24ee73 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_htdd24ee73 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_htdd24ee73 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_htdd24ee73 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_htdd24ee73 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_htdd24ee73 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_htdd24ee73 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_htdd24ee73 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_htdd24ee73 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_htdd24ee73 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_htdd24ee73 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_htdd24ee73 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_htdd24ee73 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_htdd24ee73 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_htdd24ee73 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_htdd24ee73 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_htdd24ee73 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_htdd24ee73 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_htdd24ee73 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_htdd24ee73 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_htdd24ee73 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_htdd24ee73 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_htdd24ee73 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_htdd24ee73 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_htdd24ee73 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_htdd24ee73 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_htdd24ee73 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_htdd24ee73 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_htdd24ee73 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_htdd24ee73 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_htdd24ee73 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_htdd24ee73 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_htdd24ee73 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_htdd24ee73 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_htdd24ee73 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_htdd24ee73 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_htdd24ee73 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_htdd24ee73 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_htdd24ee73 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_htdd24ee73 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_htdd24ee73 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_htdd24ee73 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_htdd24ee73 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_htdd24ee73 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_htdd24ee73 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_htdd24ee73 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_htdd24ee73 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_htdd24ee73 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_htdd24ee73 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_htdd24ee73 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_htdd24ee73 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_htdd24ee73 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_htdd24ee73 = 1000;
pub const XML_HTML_INCORRECTLY_OPENED_COMMENT: C2RustUnnamed_htdd24ee73 = 802;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_htdd24ee73 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_htdd24ee73 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_htdd24ee73 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_htdd24ee73 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_htdd24ee73 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_htdd24ee73 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_htdd24ee73 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_htdd24ee73 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_htdd24ee73 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_htdd24ee73 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_htdd24ee73 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_htdd24ee73 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_htdd24ee73 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_htdd24ee73 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_htdd24ee73 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_htdd24ee73 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_htdd24ee73 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_htdd24ee73 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_htdd24ee73 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_htdd24ee73 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_htdd24ee73 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_htdd24ee73 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_htdd24ee73 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_htdd24ee73 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_htdd24ee73 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_htdd24ee73 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_htdd24ee73 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_htdd24ee73 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_htdd24ee73 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_htdd24ee73 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_htdd24ee73 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_htdd24ee73 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_htdd24ee73 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_htdd24ee73 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_htdd24ee73 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_htdd24ee73 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_htdd24ee73 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_htdd24ee73 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_htdd24ee73 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_htdd24ee73 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_htdd24ee73 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_htdd24ee73 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_htdd24ee73 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_htdd24ee73 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_htdd24ee73 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_htdd24ee73 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_htdd24ee73 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_htdd24ee73 = 200;
pub const XML_WAR_ENCODING_MISMATCH: C2RustUnnamed_htdd24ee73 = 113;
pub const XML_ERR_COMMENT_ABRUPTLY_ENDED: C2RustUnnamed_htdd24ee73 = 112;
pub const XML_ERR_USER_STOP: C2RustUnnamed_htdd24ee73 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_htdd24ee73 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_htdd24ee73 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_htdd24ee73 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_htdd24ee73 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_htdd24ee73 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_htdd24ee73 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_htdd24ee73 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_htdd24ee73 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_htdd24ee73 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_htdd24ee73 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_htdd24ee73 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_htdd24ee73 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_htdd24ee73 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_htdd24ee73 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_htdd24ee73 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_htdd24ee73 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_htdd24ee73 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_htdd24ee73 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_htdd24ee73 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_htdd24ee73 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_htdd24ee73 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_htdd24ee73 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_htdd24ee73 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_htdd24ee73 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_htdd24ee73 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_htdd24ee73 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_htdd24ee73 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_htdd24ee73 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_htdd24ee73 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_htdd24ee73 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_htdd24ee73 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_htdd24ee73 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_htdd24ee73 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_htdd24ee73 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_htdd24ee73 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_htdd24ee73 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_htdd24ee73 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_htdd24ee73 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_htdd24ee73 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_htdd24ee73 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_htdd24ee73 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_htdd24ee73 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_htdd24ee73 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_htdd24ee73 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_htdd24ee73 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_htdd24ee73 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_htdd24ee73 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_htdd24ee73 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_htdd24ee73 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_htdd24ee73 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_htdd24ee73 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_htdd24ee73 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_htdd24ee73 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_htdd24ee73 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_htdd24ee73 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_htdd24ee73 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_htdd24ee73 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_htdd24ee73 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_htdd24ee73 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_htdd24ee73 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_htdd24ee73 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_htdd24ee73 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_htdd24ee73 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_htdd24ee73 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_htdd24ee73 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_htdd24ee73 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_htdd24ee73 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_htdd24ee73 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_htdd24ee73 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_htdd24ee73 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_htdd24ee73 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_htdd24ee73 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_htdd24ee73 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_htdd24ee73 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_htdd24ee73 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_htdd24ee73 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_htdd24ee73 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_htdd24ee73 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_htdd24ee73 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_htdd24ee73 = 3;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_ERR_OK: C2RustUnnamed_htdd24ee73 = 0;
pub type C2RustUnnamed_htaaa91a68 = c_int;
pub const XML_CHAR_ENCODING_ASCII: C2RustUnnamed_htaaa91a68 = 22;
pub const XML_CHAR_ENCODING_EUC_JP: C2RustUnnamed_htaaa91a68 = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: C2RustUnnamed_htaaa91a68 = 20;
pub const XML_CHAR_ENCODING_2022_JP: C2RustUnnamed_htaaa91a68 = 19;
pub const XML_CHAR_ENCODING_8859_9: C2RustUnnamed_htaaa91a68 = 18;
pub const XML_CHAR_ENCODING_8859_8: C2RustUnnamed_htaaa91a68 = 17;
pub const XML_CHAR_ENCODING_8859_7: C2RustUnnamed_htaaa91a68 = 16;
pub const XML_CHAR_ENCODING_8859_6: C2RustUnnamed_htaaa91a68 = 15;
pub const XML_CHAR_ENCODING_8859_5: C2RustUnnamed_htaaa91a68 = 14;
pub const XML_CHAR_ENCODING_8859_4: C2RustUnnamed_htaaa91a68 = 13;
pub const XML_CHAR_ENCODING_8859_3: C2RustUnnamed_htaaa91a68 = 12;
pub const XML_CHAR_ENCODING_8859_2: C2RustUnnamed_htaaa91a68 = 11;
pub const XML_CHAR_ENCODING_8859_1: C2RustUnnamed_htaaa91a68 = 10;
pub const XML_CHAR_ENCODING_UCS2: C2RustUnnamed_htaaa91a68 = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: C2RustUnnamed_htaaa91a68 = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: C2RustUnnamed_htaaa91a68 = 7;
pub const XML_CHAR_ENCODING_EBCDIC: C2RustUnnamed_htaaa91a68 = 6;
pub const XML_CHAR_ENCODING_UCS4BE: C2RustUnnamed_htaaa91a68 = 5;
pub const XML_CHAR_ENCODING_UCS4LE: C2RustUnnamed_htaaa91a68 = 4;
pub const XML_CHAR_ENCODING_UTF16BE: C2RustUnnamed_htaaa91a68 = 3;
pub const XML_CHAR_ENCODING_UTF16LE: C2RustUnnamed_htaaa91a68 = 2;
pub const XML_CHAR_ENCODING_NONE: C2RustUnnamed_htaaa91a68 = 0;
pub const XML_CHAR_ENCODING_ERROR: C2RustUnnamed_htaaa91a68 = -1;

#[no_mangle]
pub static mut __xmlRegisterCallbacks: c_int = 0 as c_int;
unsafe fn xmlTreeErrMemory(mut extra: *const c_char) {
    __xmlSimpleError(
        XML_FROM_TREE as c_int,
        XML_ERR_NO_MEMORY as c_int,
        ::core::ptr::null_mut::<_xmlNode>(),
        ::core::ptr::null::<c_char>(),
        extra,
    );
}
unsafe fn xmlTreeErr(
    mut code: c_int,
    mut node: xmlNodePtr,
    mut extra: *const c_char,
) {
    let mut msg: *const c_char = ::core::ptr::null::<c_char>();
    match code {
        1300 => {
            msg = b"invalid hexadecimal character value\n\0" as *const u8
                as *const c_char;
        }
        1301 => {
            msg = b"invalid decimal character value\n\0" as *const u8 as *const c_char;
        }
        1302 => {
            msg = b"unterminated entity reference %15s\n\0" as *const u8
                as *const c_char;
        }
        1303 => {
            msg = b"string is not in UTF-8\n\0" as *const u8 as *const c_char;
        }
        _ => {
            msg = b"unexpected error number\n\0" as *const u8 as *const c_char;
        }
    }
    __xmlSimpleError(
        XML_FROM_TREE as c_int,
        code,
        node as *mut _xmlNode,
        msg,
        extra,
    );
}
#[no_mangle]
pub static mut xmlStringText: [xmlChar; 5] = [
    't' as i32 as xmlChar,
    'e' as i32 as xmlChar,
    'x' as i32 as xmlChar,
    't' as i32 as xmlChar,
    0 as c_int as xmlChar,
];
#[no_mangle]
pub static mut xmlStringTextNoenc: [xmlChar; 10] = [
    't' as i32 as xmlChar,
    'e' as i32 as xmlChar,
    'x' as i32 as xmlChar,
    't' as i32 as xmlChar,
    'n' as i32 as xmlChar,
    'o' as i32 as xmlChar,
    'e' as i32 as xmlChar,
    'n' as i32 as xmlChar,
    'c' as i32 as xmlChar,
    0 as c_int as xmlChar,
];
#[no_mangle]
pub static mut xmlStringComment: [xmlChar; 8] = [
    'c' as i32 as xmlChar,
    'o' as i32 as xmlChar,
    'm' as i32 as xmlChar,
    'm' as i32 as xmlChar,
    'e' as i32 as xmlChar,
    'n' as i32 as xmlChar,
    't' as i32 as xmlChar,
    0 as c_int as xmlChar,
];
static mut xmlCompressMode: c_int = 0 as c_int;
static mut xmlCheckDTD: c_int = 1 as c_int;
unsafe fn xmlGetEntityFromDtd(
    mut dtd: *const xmlDtd,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    let mut table: xmlEntitiesTablePtr = ::core::ptr::null_mut::<xmlEntitiesTable>();
    if !dtd.is_null() && !(*dtd).entities.is_null() {
        table = (*dtd).entities as xmlEntitiesTablePtr;
        return xmlHashLookup(table as xmlHashTablePtr, name) as xmlEntityPtr;
    }
    return ::core::ptr::null_mut::<xmlEntity>();
}
unsafe fn xmlGetParameterEntityFromDtd(
    mut dtd: *const xmlDtd,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    let mut table: xmlEntitiesTablePtr = ::core::ptr::null_mut::<xmlEntitiesTable>();
    if !dtd.is_null() && !(*dtd).pentities.is_null() {
        table = (*dtd).pentities as xmlEntitiesTablePtr;
        return xmlHashLookup(table as xmlHashTablePtr, name) as xmlEntityPtr;
    }
    return ::core::ptr::null_mut::<xmlEntity>();
}
#[inline]
pub unsafe fn xmlBuildQName(
    mut ncname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut memory: *mut xmlChar,
    mut len: c_int,
) -> *mut xmlChar {
    let mut lenn: c_int = 0;
    let mut lenp: c_int = 0;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if ncname.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if prefix.is_null() {
        return ncname as *mut xmlChar;
    }
    lenn = strlen(ncname as *mut c_char) as c_int;
    lenp = strlen(prefix as *mut c_char) as c_int;
    if memory.is_null() || len < lenn + lenp + 2 as c_int {
        ret = xmlMallocAtomic.expect("non-null function pointer")(
            (lenn + lenp + 2 as c_int) as size_t,
        ) as *mut xmlChar;
        if ret.is_null() {
            xmlTreeErrMemory(b"building QName\0" as *const u8 as *const c_char);
            return ::core::ptr::null_mut::<xmlChar>();
        }
    } else {
        ret = memory;
    }
    memcpy(
        ret.offset(0 as c_int as isize) as *mut xmlChar as *mut c_void,
        prefix as *const c_void,
        lenp as size_t,
    );
    *ret.offset(lenp as isize) = ':' as i32 as xmlChar;
    memcpy(
        ret.offset((lenp + 1 as c_int) as isize) as *mut xmlChar
            as *mut c_void,
        ncname as *const c_void,
        lenn as size_t,
    );
    *ret.offset((lenn + lenp + 1 as c_int) as isize) = 0 as xmlChar;
    return ret;
}
#[inline]
pub unsafe fn xmlSplitQName2(
    mut name: *const xmlChar,
    mut prefix: *mut *mut xmlChar,
) -> *mut xmlChar {
    let mut len: c_int = 0 as c_int;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if prefix.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    *prefix = ::core::ptr::null_mut::<xmlChar>();
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if *name.offset(0 as c_int as isize) as c_int == ':' as i32 {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while *name.offset(len as isize) as c_int != 0 as c_int
        && *name.offset(len as isize) as c_int != ':' as i32
    {
        len += 1;
    }
    if *name.offset(len as isize) as c_int == 0 as c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    *prefix = xmlStrndup(name, len);
    if (*prefix).is_null() {
        xmlTreeErrMemory(b"QName split\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlStrdup(name.offset((len + 1 as c_int) as isize) as *const xmlChar);
    if ret.is_null() {
        xmlTreeErrMemory(b"QName split\0" as *const u8 as *const c_char);
        if !(*prefix).is_null() {
            xmlFree.expect("non-null function pointer")(*prefix as *mut c_void);
            *prefix = ::core::ptr::null_mut::<xmlChar>();
        }
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return ret;
}
#[inline]
pub unsafe fn xmlSplitQName3(
    mut name: *const xmlChar,
    mut len: *mut c_int,
) -> *const xmlChar {
    let mut l: c_int = 0 as c_int;
    if name.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if len.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if *name.offset(0 as c_int as isize) as c_int == ':' as i32 {
        return ::core::ptr::null::<xmlChar>();
    }
    while *name.offset(l as isize) as c_int != 0 as c_int
        && *name.offset(l as isize) as c_int != ':' as i32
    {
        l += 1;
    }
    if *name.offset(l as isize) as c_int == 0 as c_int {
        return ::core::ptr::null::<xmlChar>();
    }
    *len = l;
    return name.offset((l + 1 as c_int) as isize) as *const xmlChar;
}
#[inline]
pub unsafe fn xmlValidateNCName(
    mut value: *const xmlChar,
    mut space: c_int,
) -> c_int {
    let mut cur: *const xmlChar = value;
    let mut c: c_int = 0;
    let mut l: c_int = 0;
    if value.is_null() {
        return -(1 as c_int);
    }
    if space != 0 {
        while *cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int
        {
            cur = cur.offset(1);
        }
    }
    if *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
        || *cur as c_int >= 'A' as i32 && *cur as c_int <= 'Z' as i32
        || *cur as c_int == '_' as i32
    {
        cur = cur.offset(1);
        while *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
            || *cur as c_int >= 'A' as i32 && *cur as c_int <= 'Z' as i32
            || *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32
            || *cur as c_int == '_' as i32
            || *cur as c_int == '-' as i32
            || *cur as c_int == '.' as i32
        {
            cur = cur.offset(1);
        }
        if space != 0 {
            while *cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *cur as c_int
                    && *cur as c_int <= 0xa as c_int
                || *cur as c_int == 0xd as c_int
            {
                cur = cur.offset(1);
            }
        }
        if *cur as c_int == 0 as c_int {
            return 0 as c_int;
        }
    }
    cur = value;
    c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    if space != 0 {
        while if c < 0x100 as c_int {
            (c == 0x20 as c_int
                || 0x9 as c_int <= c && c <= 0xa as c_int
                || c == 0xd as c_int) as c_int
        } else {
            0 as c_int
        } != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if !((if c < 0x100 as c_int {
        (0x41 as c_int <= c && c <= 0x5a as c_int
            || 0x61 as c_int <= c && c <= 0x7a as c_int
            || 0xc0 as c_int <= c && c <= 0xd6 as c_int
            || 0xd8 as c_int <= c && c <= 0xf6 as c_int
            || 0xf8 as c_int <= c) as c_int
    } else {
        xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                || c == 0x3007 as c_int
                || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                as c_int
        }) != 0)
        && c != '_' as i32
    {
        return 1 as c_int;
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    while (if c < 0x100 as c_int {
        (0x41 as c_int <= c && c <= 0x5a as c_int
            || 0x61 as c_int <= c && c <= 0x7a as c_int
            || 0xc0 as c_int <= c && c <= 0xd6 as c_int
            || 0xd8 as c_int <= c && c <= 0xf6 as c_int
            || 0xf8 as c_int <= c) as c_int
    } else {
        xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                || c == 0x3007 as c_int
                || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                as c_int
        }) != 0
        || (if c < 0x100 as c_int {
            (0x30 as c_int <= c && c <= 0x39 as c_int)
                as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsDigitGroup)
        }) != 0
        || c == '.' as i32
        || c == '-' as i32
        || c == '_' as i32
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsCombiningGroup)
        }) != 0
        || (if c < 0x100 as c_int {
            (c == 0xb7 as c_int) as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsExtenderGroup)
        }) != 0
    {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    }
    if space != 0 {
        while if c < 0x100 as c_int {
            (c == 0x20 as c_int
                || 0x9 as c_int <= c && c <= 0xa as c_int
                || c == 0xd as c_int) as c_int
        } else {
            0 as c_int
        } != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if c != 0 as c_int {
        return 1 as c_int;
    }
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlValidateQName(
    mut value: *const xmlChar,
    mut space: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut cur: *const xmlChar = value;
    let mut c: c_int = 0;
    let mut l: c_int = 0;
    if value.is_null() {
        return -(1 as c_int);
    }
    if space != 0 {
        while *cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int
        {
            cur = cur.offset(1);
        }
    }
    if *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
        || *cur as c_int >= 'A' as i32 && *cur as c_int <= 'Z' as i32
        || *cur as c_int == '_' as i32
    {
        cur = cur.offset(1);
        while *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
            || *cur as c_int >= 'A' as i32 && *cur as c_int <= 'Z' as i32
            || *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32
            || *cur as c_int == '_' as i32
            || *cur as c_int == '-' as i32
            || *cur as c_int == '.' as i32
        {
            cur = cur.offset(1);
        }
        if *cur as c_int == ':' as i32 {
            cur = cur.offset(1);
            if *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
                || *cur as c_int >= 'A' as i32
                    && *cur as c_int <= 'Z' as i32
                || *cur as c_int == '_' as i32
            {
                cur = cur.offset(1);
                while *cur as c_int >= 'a' as i32
                    && *cur as c_int <= 'z' as i32
                    || *cur as c_int >= 'A' as i32
                        && *cur as c_int <= 'Z' as i32
                    || *cur as c_int >= '0' as i32
                        && *cur as c_int <= '9' as i32
                    || *cur as c_int == '_' as i32
                    || *cur as c_int == '-' as i32
                    || *cur as c_int == '.' as i32
                {
                    cur = cur.offset(1);
                }
                current_block = 3512920355445576850;
            } else {
                current_block = 1534697753323942756;
            }
        } else {
            current_block = 3512920355445576850;
        }
        match current_block {
            1534697753323942756 => {}
            _ => {
                if space != 0 {
                    while *cur as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *cur as c_int
                            && *cur as c_int <= 0xa as c_int
                        || *cur as c_int == 0xd as c_int
                    {
                        cur = cur.offset(1);
                    }
                }
                if *cur as c_int == 0 as c_int {
                    return 0 as c_int;
                }
            }
        }
    }
    cur = value;
    c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    if space != 0 {
        while if c < 0x100 as c_int {
            (c == 0x20 as c_int
                || 0x9 as c_int <= c && c <= 0xa as c_int
                || c == 0xd as c_int) as c_int
        } else {
            0 as c_int
        } != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if !((if c < 0x100 as c_int {
        (0x41 as c_int <= c && c <= 0x5a as c_int
            || 0x61 as c_int <= c && c <= 0x7a as c_int
            || 0xc0 as c_int <= c && c <= 0xd6 as c_int
            || 0xd8 as c_int <= c && c <= 0xf6 as c_int
            || 0xf8 as c_int <= c) as c_int
    } else {
        xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                || c == 0x3007 as c_int
                || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                as c_int
        }) != 0)
        && c != '_' as i32
    {
        return 1 as c_int;
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    while (if c < 0x100 as c_int {
        (0x41 as c_int <= c && c <= 0x5a as c_int
            || 0x61 as c_int <= c && c <= 0x7a as c_int
            || 0xc0 as c_int <= c && c <= 0xd6 as c_int
            || 0xd8 as c_int <= c && c <= 0xf6 as c_int
            || 0xf8 as c_int <= c) as c_int
    } else {
        xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                || c == 0x3007 as c_int
                || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                as c_int
        }) != 0
        || (if c < 0x100 as c_int {
            (0x30 as c_int <= c && c <= 0x39 as c_int)
                as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsDigitGroup)
        }) != 0
        || c == '.' as i32
        || c == '-' as i32
        || c == '_' as i32
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsCombiningGroup)
        }) != 0
        || (if c < 0x100 as c_int {
            (c == 0xb7 as c_int) as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsExtenderGroup)
        }) != 0
    {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    }
    if c == ':' as i32 {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        if !((if c < 0x100 as c_int {
            (0x41 as c_int <= c && c <= 0x5a as c_int
                || 0x61 as c_int <= c && c <= 0x7a as c_int
                || 0xc0 as c_int <= c && c <= 0xd6 as c_int
                || 0xd8 as c_int <= c && c <= 0xf6 as c_int
                || 0xf8 as c_int <= c) as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
        }) != 0
            || (if c < 0x100 as c_int {
                0 as c_int
            } else {
                (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                    || c == 0x3007 as c_int
                    || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                    as c_int
            }) != 0)
            && c != '_' as i32
        {
            return 1 as c_int;
        }
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        while (if c < 0x100 as c_int {
            (0x41 as c_int <= c && c <= 0x5a as c_int
                || 0x61 as c_int <= c && c <= 0x7a as c_int
                || 0xc0 as c_int <= c && c <= 0xd6 as c_int
                || 0xd8 as c_int <= c && c <= 0xf6 as c_int
                || 0xf8 as c_int <= c) as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
        }) != 0
            || (if c < 0x100 as c_int {
                0 as c_int
            } else {
                (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                    || c == 0x3007 as c_int
                    || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                    as c_int
            }) != 0
            || (if c < 0x100 as c_int {
                (0x30 as c_int <= c && c <= 0x39 as c_int)
                    as c_int
            } else {
                xmlCharInRange(c as c_uint, &raw const xmlIsDigitGroup)
            }) != 0
            || c == '.' as i32
            || c == '-' as i32
            || c == '_' as i32
            || (if c < 0x100 as c_int {
                0 as c_int
            } else {
                xmlCharInRange(c as c_uint, &raw const xmlIsCombiningGroup)
            }) != 0
            || (if c < 0x100 as c_int {
                (c == 0xb7 as c_int) as c_int
            } else {
                xmlCharInRange(c as c_uint, &raw const xmlIsExtenderGroup)
            }) != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if space != 0 {
        while if c < 0x100 as c_int {
            (c == 0x20 as c_int
                || 0x9 as c_int <= c && c <= 0xa as c_int
                || c == 0xd as c_int) as c_int
        } else {
            0 as c_int
        } != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if c != 0 as c_int {
        return 1 as c_int;
    }
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlValidateName(
    mut value: *const xmlChar,
    mut space: c_int,
) -> c_int {
    let mut cur: *const xmlChar = value;
    let mut c: c_int = 0;
    let mut l: c_int = 0;
    if value.is_null() {
        return -(1 as c_int);
    }
    if space != 0 {
        while *cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int
        {
            cur = cur.offset(1);
        }
    }
    if *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
        || *cur as c_int >= 'A' as i32 && *cur as c_int <= 'Z' as i32
        || *cur as c_int == '_' as i32
        || *cur as c_int == ':' as i32
    {
        cur = cur.offset(1);
        while *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
            || *cur as c_int >= 'A' as i32 && *cur as c_int <= 'Z' as i32
            || *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32
            || *cur as c_int == '_' as i32
            || *cur as c_int == '-' as i32
            || *cur as c_int == '.' as i32
            || *cur as c_int == ':' as i32
        {
            cur = cur.offset(1);
        }
        if space != 0 {
            while *cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *cur as c_int
                    && *cur as c_int <= 0xa as c_int
                || *cur as c_int == 0xd as c_int
            {
                cur = cur.offset(1);
            }
        }
        if *cur as c_int == 0 as c_int {
            return 0 as c_int;
        }
    }
    cur = value;
    c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    if space != 0 {
        while if c < 0x100 as c_int {
            (c == 0x20 as c_int
                || 0x9 as c_int <= c && c <= 0xa as c_int
                || c == 0xd as c_int) as c_int
        } else {
            0 as c_int
        } != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if !((if c < 0x100 as c_int {
        (0x41 as c_int <= c && c <= 0x5a as c_int
            || 0x61 as c_int <= c && c <= 0x7a as c_int
            || 0xc0 as c_int <= c && c <= 0xd6 as c_int
            || 0xd8 as c_int <= c && c <= 0xf6 as c_int
            || 0xf8 as c_int <= c) as c_int
    } else {
        xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                || c == 0x3007 as c_int
                || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                as c_int
        }) != 0)
        && c != '_' as i32
        && c != ':' as i32
    {
        return 1 as c_int;
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    while (if c < 0x100 as c_int {
        (0x41 as c_int <= c && c <= 0x5a as c_int
            || 0x61 as c_int <= c && c <= 0x7a as c_int
            || 0xc0 as c_int <= c && c <= 0xd6 as c_int
            || 0xd8 as c_int <= c && c <= 0xf6 as c_int
            || 0xf8 as c_int <= c) as c_int
    } else {
        xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                || c == 0x3007 as c_int
                || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                as c_int
        }) != 0
        || (if c < 0x100 as c_int {
            (0x30 as c_int <= c && c <= 0x39 as c_int)
                as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsDigitGroup)
        }) != 0
        || c == '.' as i32
        || c == ':' as i32
        || c == '-' as i32
        || c == '_' as i32
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsCombiningGroup)
        }) != 0
        || (if c < 0x100 as c_int {
            (c == 0xb7 as c_int) as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsExtenderGroup)
        }) != 0
    {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    }
    if space != 0 {
        while if c < 0x100 as c_int {
            (c == 0x20 as c_int
                || 0x9 as c_int <= c && c <= 0xa as c_int
                || c == 0xd as c_int) as c_int
        } else {
            0 as c_int
        } != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if c != 0 as c_int {
        return 1 as c_int;
    }
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlValidateNMToken(
    mut value: *const xmlChar,
    mut space: c_int,
) -> c_int {
    let mut cur: *const xmlChar = value;
    let mut c: c_int = 0;
    let mut l: c_int = 0;
    if value.is_null() {
        return -(1 as c_int);
    }
    if space != 0 {
        while *cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int
        {
            cur = cur.offset(1);
        }
    }
    if *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
        || *cur as c_int >= 'A' as i32 && *cur as c_int <= 'Z' as i32
        || *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32
        || *cur as c_int == '_' as i32
        || *cur as c_int == '-' as i32
        || *cur as c_int == '.' as i32
        || *cur as c_int == ':' as i32
    {
        cur = cur.offset(1);
        while *cur as c_int >= 'a' as i32 && *cur as c_int <= 'z' as i32
            || *cur as c_int >= 'A' as i32 && *cur as c_int <= 'Z' as i32
            || *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32
            || *cur as c_int == '_' as i32
            || *cur as c_int == '-' as i32
            || *cur as c_int == '.' as i32
            || *cur as c_int == ':' as i32
        {
            cur = cur.offset(1);
        }
        if space != 0 {
            while *cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *cur as c_int
                    && *cur as c_int <= 0xa as c_int
                || *cur as c_int == 0xd as c_int
            {
                cur = cur.offset(1);
            }
        }
        if *cur as c_int == 0 as c_int {
            return 0 as c_int;
        }
    }
    cur = value;
    c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    if space != 0 {
        while if c < 0x100 as c_int {
            (c == 0x20 as c_int
                || 0x9 as c_int <= c && c <= 0xa as c_int
                || c == 0xd as c_int) as c_int
        } else {
            0 as c_int
        } != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if !((if c < 0x100 as c_int {
        (0x41 as c_int <= c && c <= 0x5a as c_int
            || 0x61 as c_int <= c && c <= 0x7a as c_int
            || 0xc0 as c_int <= c && c <= 0xd6 as c_int
            || 0xd8 as c_int <= c && c <= 0xf6 as c_int
            || 0xf8 as c_int <= c) as c_int
    } else {
        xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                || c == 0x3007 as c_int
                || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                as c_int
        }) != 0
        || (if c < 0x100 as c_int {
            (0x30 as c_int <= c && c <= 0x39 as c_int)
                as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsDigitGroup)
        }) != 0
        || c == '.' as i32
        || c == ':' as i32
        || c == '-' as i32
        || c == '_' as i32
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsCombiningGroup)
        }) != 0
        || (if c < 0x100 as c_int {
            (c == 0xb7 as c_int) as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsExtenderGroup)
        }) != 0)
    {
        return 1 as c_int;
    }
    cur = cur.offset(l as isize);
    c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    while (if c < 0x100 as c_int {
        (0x41 as c_int <= c && c <= 0x5a as c_int
            || 0x61 as c_int <= c && c <= 0x7a as c_int
            || 0xc0 as c_int <= c && c <= 0xd6 as c_int
            || 0xd8 as c_int <= c && c <= 0xf6 as c_int
            || 0xf8 as c_int <= c) as c_int
    } else {
        xmlCharInRange(c as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= c && c <= 0x9fa5 as c_int
                || c == 0x3007 as c_int
                || 0x3021 as c_int <= c && c <= 0x3029 as c_int)
                as c_int
        }) != 0
        || (if c < 0x100 as c_int {
            (0x30 as c_int <= c && c <= 0x39 as c_int)
                as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsDigitGroup)
        }) != 0
        || c == '.' as i32
        || c == ':' as i32
        || c == '-' as i32
        || c == '_' as i32
        || (if c < 0x100 as c_int {
            0 as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsCombiningGroup)
        }) != 0
        || (if c < 0x100 as c_int {
            (c == 0xb7 as c_int) as c_int
        } else {
            xmlCharInRange(c as c_uint, &raw const xmlIsExtenderGroup)
        }) != 0
    {
        cur = cur.offset(l as isize);
        c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
    }
    if space != 0 {
        while if c < 0x100 as c_int {
            (c == 0x20 as c_int
                || 0x9 as c_int <= c && c <= 0xa as c_int
                || c == 0xd as c_int) as c_int
        } else {
            0 as c_int
        } != 0
        {
            cur = cur.offset(l as isize);
            c = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut l);
        }
    }
    if c != 0 as c_int {
        return 1 as c_int;
    }
    return 0 as c_int;
}
#[inline]
pub fn xmlSetBufferAllocationScheme(mut scheme: xmlBufferAllocationScheme) { unsafe {
    if scheme as c_uint
        == XML_BUFFER_ALLOC_EXACT as c_int as c_uint
        || scheme as c_uint
            == XML_BUFFER_ALLOC_DOUBLEIT as c_int as c_uint
        || scheme as c_uint
            == XML_BUFFER_ALLOC_HYBRID as c_int as c_uint
    {
        *__xmlBufferAllocScheme() = scheme;
    }
} }
#[inline]
pub fn xmlGetBufferAllocationScheme() -> xmlBufferAllocationScheme { unsafe {
    return *__xmlBufferAllocScheme();
} }
#[no_mangle]
pub unsafe extern "C" fn xmlNewNs(
    mut node: xmlNodePtr,
    mut href: *const xmlChar,
    mut prefix: *const xmlChar,
) -> xmlNsPtr {
    let mut cur: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if !node.is_null()
        && (*node).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    if !prefix.is_null()
        && xmlStrEqual(
            prefix,
            b"xml\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
    {
        if xmlStrEqual(href, XML_XML_NAMESPACE) != 0 {
            return ::core::ptr::null_mut::<xmlNs>();
        }
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNs>() as size_t)
        as xmlNsPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building namespace\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNs>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNs>() as size_t,
    );
    (*cur).type_0 = XML_NAMESPACE_DECL;
    if !href.is_null() {
        (*cur).href = xmlStrdup(href);
    }
    if !prefix.is_null() {
        (*cur).prefix = xmlStrdup(prefix);
    }
    if !node.is_null() {
        if (*node).nsDef.is_null() {
            (*node).nsDef = cur as *mut xmlNs;
        } else {
            let mut prev: xmlNsPtr = (*node).nsDef as xmlNsPtr;
            if (*prev).prefix.is_null() && (*cur).prefix.is_null()
                || xmlStrEqual((*prev).prefix, (*cur).prefix) != 0
            {
                xmlFreeNs(cur);
                return ::core::ptr::null_mut::<xmlNs>();
            }
            while !(*prev).next.is_null() {
                prev = (*prev).next as xmlNsPtr;
                if (*prev).prefix.is_null() && (*cur).prefix.is_null()
                    || xmlStrEqual((*prev).prefix, (*cur).prefix) != 0
                {
                    xmlFreeNs(cur);
                    return ::core::ptr::null_mut::<xmlNs>();
                }
            }
            (*prev).next = cur as *mut _xmlNs;
        }
    }
    return cur;
}
#[no_mangle]
pub extern "C" fn xmlSetNs(mut node: xmlNodePtr, mut ns: xmlNsPtr) { unsafe {
    if node.is_null() {
        return;
    }
    if (*node).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        || (*node).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        (*node).ns = ns as *mut xmlNs;
    }
} }
#[no_mangle]
pub extern "C" fn xmlFreeNs(mut cur: xmlNsPtr) { unsafe {
    if cur.is_null() {
        return;
    }
    if !(*cur).href.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*cur).href as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).prefix.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*cur).prefix as *mut c_char as *mut c_void,
        );
    }
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
} }
#[no_mangle]
pub extern "C" fn xmlFreeNsList(mut cur: xmlNsPtr) { unsafe {
    let mut next: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if cur.is_null() {
        return;
    }
    while !cur.is_null() {
        next = (*cur).next as xmlNsPtr;
        xmlFreeNs(cur);
        cur = next;
    }
} }
#[no_mangle]
pub unsafe extern "C" fn xmlNewDtd(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlDtdPtr {
    let mut cur: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    if !doc.is_null() && !(*doc).extSubset.is_null() {
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlDtd>() as size_t)
        as xmlDtdPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building DTD\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlDtd>() as size_t,
    );
    (*cur).type_0 = XML_DTD_NODE;
    if !name.is_null() {
        (*cur).name = xmlStrdup(name);
    }
    if !ExternalID.is_null() {
        (*cur).ExternalID = xmlStrdup(ExternalID);
    }
    if !SystemID.is_null() {
        (*cur).SystemID = xmlStrdup(SystemID);
    }
    if !doc.is_null() {
        (*doc).extSubset = cur as *mut _xmlDtd;
    }
    (*cur).doc = doc as *mut _xmlDoc;
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetIntSubset(mut doc: *const xmlDoc) -> xmlDtdPtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    cur = (*doc).children as xmlNodePtr;
    while !cur.is_null() {
        if (*cur).type_0 as c_uint
            == XML_DTD_NODE as c_int as c_uint
        {
            return cur as xmlDtdPtr;
        }
        cur = (*cur).next as xmlNodePtr;
    }
    return (*doc).intSubset as xmlDtdPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateIntSubset(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlDtdPtr {
    let mut cur: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    if !doc.is_null() && !xmlGetIntSubset(doc as *const xmlDoc).is_null() {
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlDtd>() as size_t)
        as xmlDtdPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building internal subset\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlDtd>() as size_t,
    );
    (*cur).type_0 = XML_DTD_NODE;
    if !name.is_null() {
        (*cur).name = xmlStrdup(name);
        if (*cur).name.is_null() {
            xmlTreeErrMemory(
                b"building internal subset\0" as *const u8 as *const c_char,
            );
            xmlFree.expect("non-null function pointer")(cur as *mut c_void);
            return ::core::ptr::null_mut::<xmlDtd>();
        }
    }
    if !ExternalID.is_null() {
        (*cur).ExternalID = xmlStrdup(ExternalID);
        if (*cur).ExternalID.is_null() {
            xmlTreeErrMemory(
                b"building internal subset\0" as *const u8 as *const c_char,
            );
            if !(*cur).name.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*cur).name as *mut c_char as *mut c_void,
                );
            }
            xmlFree.expect("non-null function pointer")(cur as *mut c_void);
            return ::core::ptr::null_mut::<xmlDtd>();
        }
    }
    if !SystemID.is_null() {
        (*cur).SystemID = xmlStrdup(SystemID);
        if (*cur).SystemID.is_null() {
            xmlTreeErrMemory(
                b"building internal subset\0" as *const u8 as *const c_char,
            );
            if !(*cur).name.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*cur).name as *mut c_char as *mut c_void,
                );
            }
            if !(*cur).ExternalID.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*cur).ExternalID as *mut c_char as *mut c_void,
                );
            }
            xmlFree.expect("non-null function pointer")(cur as *mut c_void);
            return ::core::ptr::null_mut::<xmlDtd>();
        }
    }
    if !doc.is_null() {
        (*doc).intSubset = cur as *mut _xmlDtd;
        (*cur).parent = doc as *mut _xmlDoc;
        (*cur).doc = doc as *mut _xmlDoc;
        if (*doc).children.is_null() {
            (*doc).children = cur as xmlNodePtr as *mut _xmlNode;
            (*doc).last = cur as xmlNodePtr as *mut _xmlNode;
        } else if (*doc).type_0 as c_uint
            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
        {
            let mut prev: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            prev = (*doc).children as xmlNodePtr;
            (*prev).prev = cur as xmlNodePtr as *mut _xmlNode;
            (*cur).next = prev as *mut _xmlNode;
            (*doc).children = cur as xmlNodePtr as *mut _xmlNode;
        } else {
            let mut next: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            next = (*doc).children as xmlNodePtr;
            while !next.is_null()
                && (*next).type_0 as c_uint
                    != XML_ELEMENT_NODE as c_int as c_uint
            {
                next = (*next).next as xmlNodePtr;
            }
            if next.is_null() {
                (*cur).prev = (*doc).last;
                (*(*cur).prev).next = cur as xmlNodePtr as *mut _xmlNode;
                (*cur).next = ::core::ptr::null_mut::<_xmlNode>();
                (*doc).last = cur as xmlNodePtr as *mut _xmlNode;
            } else {
                (*cur).next = next as *mut _xmlNode;
                (*cur).prev = (*next).prev;
                if (*cur).prev.is_null() {
                    (*doc).children = cur as xmlNodePtr as *mut _xmlNode;
                } else {
                    (*(*cur).prev).next = cur as xmlNodePtr as *mut _xmlNode;
                }
                (*next).prev = cur as xmlNodePtr as *mut _xmlNode;
            }
        }
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    return cur;
}
#[no_mangle]
pub extern "C" fn xmlFreeDtd(mut cur: xmlDtdPtr) { unsafe {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if cur.is_null() {
        return;
    }
    if !(*cur).doc.is_null() {
        dict = (*(*cur).doc).dict as xmlDictPtr;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    if !(*cur).children.is_null() {
        let mut next: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        let mut c: xmlNodePtr = (*cur).children as xmlNodePtr;
        while !c.is_null() {
            next = (*c).next as xmlNodePtr;
            if (*c).type_0 as c_uint
                != XML_NOTATION_NODE as c_int as c_uint
                && (*c).type_0 as c_uint
                    != XML_ELEMENT_DECL as c_int as c_uint
                && (*c).type_0 as c_uint
                    != XML_ATTRIBUTE_DECL as c_int as c_uint
                && (*c).type_0 as c_uint
                    != XML_ENTITY_DECL as c_int as c_uint
            {
                xmlUnlinkNode(c);
                xmlFreeNode(c);
            }
            c = next;
        }
    }
    if !(*cur).name.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).name as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).SystemID.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).SystemID) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).SystemID as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).ExternalID.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).ExternalID) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).ExternalID as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).notations.is_null() {
        xmlFreeNotationTable((*cur).notations as xmlNotationTablePtr);
    }
    if !(*cur).elements.is_null() {
        xmlFreeElementTable((*cur).elements as xmlElementTablePtr);
    }
    if !(*cur).attributes.is_null() {
        xmlFreeAttributeTable((*cur).attributes as xmlAttributeTablePtr);
    }
    if !(*cur).entities.is_null() {
        xmlFreeEntitiesTable((*cur).entities as xmlEntitiesTablePtr);
    }
    if !(*cur).pentities.is_null() {
        xmlFreeEntitiesTable((*cur).pentities as xmlEntitiesTablePtr);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
} }
#[no_mangle]
pub unsafe extern "C" fn xmlNewDoc(mut version: *const xmlChar) -> xmlDocPtr {
    let mut cur: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if version.is_null() {
        version = b"1.0\0" as *const u8 as *const c_char as *const xmlChar;
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlDoc>() as size_t)
        as xmlDocPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building doc\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlDoc>() as size_t,
    );
    (*cur).type_0 = XML_DOCUMENT_NODE;
    (*cur).version = xmlStrdup(version);
    if (*cur).version.is_null() {
        xmlTreeErrMemory(b"building doc\0" as *const u8 as *const c_char);
        xmlFree.expect("non-null function pointer")(cur as *mut c_void);
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    (*cur).standalone = -(1 as c_int);
    (*cur).compression = -(1 as c_int);
    (*cur).doc = cur as *mut _xmlDoc;
    (*cur).parseFlags = 0 as c_int;
    (*cur).properties = XML_DOC_USERBUILT as c_int;
    (*cur).charset = XML_CHAR_ENCODING_UTF8 as c_int;
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    return cur;
}
#[no_mangle]
pub extern "C" fn xmlFreeDoc(mut cur: xmlDocPtr) { unsafe {
    let mut extSubset: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut intSubset: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if cur.is_null() {
        return;
    }
    if !cur.is_null() {
        dict = (*cur).dict as xmlDictPtr;
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
        xmlFreeNodeList((*cur).children as xmlNodePtr);
    }
    if !(*cur).oldNs.is_null() {
        xmlFreeNsList((*cur).oldNs as xmlNsPtr);
    }
    if !(*cur).version.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).version) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).version as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).name.is_null()
        && (dict.is_null()
            || xmlDictOwns(dict, (*cur).name as *const xmlChar) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")((*cur).name as *mut c_void);
    }
    if !(*cur).encoding.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).encoding) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).encoding as *mut c_char as *mut c_void,
        );
    }
    if !(*cur).URL.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).URL) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).URL as *mut c_char as *mut c_void,
        );
    }
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
    if !dict.is_null() {
        xmlDictFree(dict);
    }
} }
#[no_mangle]
pub unsafe extern "C" fn xmlStringLenGetNodeList(
    mut doc: *const xmlDoc,
    mut value: *const xmlChar,
    mut len: c_int,
) -> xmlNodePtr {
    let mut current_block: u64;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut last: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut val: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut end: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut q: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut buf: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    cur = value;
    end = cur.offset(len as isize);
    buf = xmlBufCreateSize(0 as size_t);
    if buf.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    xmlBufSetAllocationScheme(buf, XML_BUFFER_ALLOC_DOUBLEIT);
    q = cur;
    loop {
        if !(cur < end && *cur as c_int != 0 as c_int) {
            current_block = 6014157347423944569;
            break;
        }
        if *cur.offset(0 as c_int as isize) as c_int == '&' as i32 {
            let mut charval: c_int = 0 as c_int;
            let mut tmp: xmlChar = 0;
            if cur != q {
                if xmlBufAdd(
                    buf,
                    q,
                    cur.offset_from(q) as c_long as c_int,
                ) != 0
                {
                    current_block = 718167063954554577;
                    break;
                }
            }
            q = cur;
            if cur.offset(2 as c_int as isize) < end
                && *cur.offset(1 as c_int as isize) as c_int == '#' as i32
                && *cur.offset(2 as c_int as isize) as c_int == 'x' as i32
            {
                cur = cur.offset(3 as c_int as isize);
                if cur < end {
                    tmp = *cur;
                } else {
                    tmp = 0 as xmlChar;
                }
                while tmp as c_int != ';' as i32 {
                    if tmp as c_int >= '0' as i32
                        && tmp as c_int <= '9' as i32
                    {
                        charval = charval * 16 as c_int
                            + (tmp as c_int - '0' as i32);
                    } else if tmp as c_int >= 'a' as i32
                        && tmp as c_int <= 'f' as i32
                    {
                        charval = charval * 16 as c_int
                            + (tmp as c_int - 'a' as i32)
                            + 10 as c_int;
                    } else if tmp as c_int >= 'A' as i32
                        && tmp as c_int <= 'F' as i32
                    {
                        charval = charval * 16 as c_int
                            + (tmp as c_int - 'A' as i32)
                            + 10 as c_int;
                    } else {
                        xmlTreeErr(
                            XML_TREE_INVALID_HEX as c_int,
                            doc as xmlNodePtr,
                            ::core::ptr::null::<c_char>(),
                        );
                        charval = 0 as c_int;
                        break;
                    }
                    cur = cur.offset(1);
                    if cur < end {
                        tmp = *cur;
                    } else {
                        tmp = 0 as xmlChar;
                    }
                }
                if tmp as c_int == ';' as i32 {
                    cur = cur.offset(1);
                }
                q = cur;
            } else if cur.offset(1 as c_int as isize) < end
                && *cur.offset(1 as c_int as isize) as c_int == '#' as i32
            {
                cur = cur.offset(2 as c_int as isize);
                if cur < end {
                    tmp = *cur;
                } else {
                    tmp = 0 as xmlChar;
                }
                while tmp as c_int != ';' as i32 {
                    if tmp as c_int >= '0' as i32
                        && tmp as c_int <= '9' as i32
                    {
                        charval = charval * 10 as c_int
                            + (tmp as c_int - '0' as i32);
                        cur = cur.offset(1);
                        if cur < end {
                            tmp = *cur;
                        } else {
                            tmp = 0 as xmlChar;
                        }
                    } else {
                        xmlTreeErr(
                            XML_TREE_INVALID_DEC as c_int,
                            doc as xmlNodePtr,
                            ::core::ptr::null::<c_char>(),
                        );
                        charval = 0 as c_int;
                        break;
                    }
                }
                if tmp as c_int == ';' as i32 {
                    cur = cur.offset(1);
                }
                q = cur;
            } else {
                cur = cur.offset(1);
                q = cur;
                while cur < end
                    && *cur as c_int != 0 as c_int
                    && *cur as c_int != ';' as i32
                {
                    cur = cur.offset(1);
                }
                if cur >= end || *cur as c_int == 0 as c_int {
                    xmlTreeErr(
                        XML_TREE_UNTERMINATED_ENTITY as c_int,
                        doc as xmlNodePtr,
                        q as *const c_char,
                    );
                    current_block = 718167063954554577;
                    break;
                } else {
                    if cur != q {
                        val = xmlStrndup(
                            q,
                            cur.offset_from(q) as c_long as c_int,
                        );
                        ent = xmlGetDocEntity(doc, val);
                        if !ent.is_null()
                            && (*ent).etype as c_uint
                                == XML_INTERNAL_PREDEFINED_ENTITY as c_int
                                    as c_uint
                        {
                            if xmlBufCat(buf, (*ent).content) != 0 {
                                current_block = 718167063954554577;
                                break;
                            }
                        } else {
                            if xmlBufIsEmpty(buf) == 0 {
                                node = xmlNewDocText(doc, ::core::ptr::null::<xmlChar>());
                                if node.is_null() {
                                    if !val.is_null() {
                                        xmlFree.expect("non-null function pointer")(
                                            val as *mut c_void,
                                        );
                                    }
                                    current_block = 718167063954554577;
                                    break;
                                } else {
                                    (*node).content = xmlBufDetach(buf);
                                    if last.is_null() {
                                        ret = node;
                                        last = ret;
                                    } else {
                                        last = xmlAddNextSibling(last, node);
                                    }
                                }
                            }
                            node = xmlNewReference(doc, val);
                            if node.is_null() {
                                if !val.is_null() {
                                    xmlFree.expect("non-null function pointer")(
                                        val as *mut c_void,
                                    );
                                }
                                current_block = 718167063954554577;
                                break;
                            } else {
                                if !ent.is_null()
                                    && (*ent).flags & XML_ENT_PARSED == 0 as c_int
                                    && (*ent).flags & XML_ENT_EXPANDING == 0 as c_int
                                {
                                    let mut temp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                                    (*ent).flags |= XML_ENT_EXPANDING;
                                    (*ent).children = xmlStringGetNodeList(
                                        doc,
                                        (*node).content as *const xmlChar,
                                    )
                                        as *mut _xmlNode;
                                    (*ent).owner = 1 as c_int;
                                    (*ent).flags &= !XML_ENT_EXPANDING;
                                    (*ent).flags |= XML_ENT_PARSED;
                                    temp = (*ent).children as xmlNodePtr;
                                    while !temp.is_null() {
                                        (*temp).parent = ent as xmlNodePtr as *mut _xmlNode;
                                        (*ent).last = temp as *mut _xmlNode;
                                        temp = (*temp).next as xmlNodePtr;
                                    }
                                }
                                if last.is_null() {
                                    ret = node;
                                    last = ret;
                                } else {
                                    last = xmlAddNextSibling(last, node);
                                }
                            }
                        }
                        xmlFree.expect("non-null function pointer")(
                            val as *mut c_void,
                        );
                    }
                    cur = cur.offset(1);
                    q = cur;
                }
            }
            if !(charval != 0 as c_int) {
                continue;
            }
            let mut buffer: [xmlChar; 10] = [0; 10];
            let mut l: c_int = 0;
            l = xmlCopyCharMultiByte(&raw mut buffer as *mut xmlChar, charval);
            buffer[l as usize] = 0 as xmlChar;
            if xmlBufCat(buf, &raw mut buffer as *mut xmlChar) != 0 {
                current_block = 718167063954554577;
                break;
            }
            charval = 0 as c_int;
        } else {
            cur = cur.offset(1);
        }
    }
    match current_block {
        6014157347423944569 => {
            if cur != q {
                if xmlBufAdd(
                    buf,
                    q,
                    cur.offset_from(q) as c_long as c_int,
                ) != 0
                {
                    current_block = 718167063954554577;
                } else {
                    current_block = 15734707049249739970;
                }
            } else {
                current_block = 15734707049249739970;
            }
            match current_block {
                718167063954554577 => {}
                _ => {
                    if xmlBufIsEmpty(buf) == 0 {
                        node = xmlNewDocText(doc, ::core::ptr::null::<xmlChar>());
                        if !node.is_null() {
                            (*node).content = xmlBufDetach(buf);
                            if last.is_null() {
                                ret = node;
                            } else {
                                xmlAddNextSibling(last, node);
                            }
                        }
                    } else if ret.is_null() {
                        ret = xmlNewDocText(
                            doc,
                            b"\0" as *const u8 as *const c_char as *mut xmlChar,
                        );
                    }
                }
            }
        }
        _ => {}
    }
    xmlBufFree(buf);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStringGetNodeList(
    mut doc: *const xmlDoc,
    mut value: *const xmlChar,
) -> xmlNodePtr {
    let mut current_block: u64;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut head: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut last: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut val: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: *const xmlChar = value;
    let mut q: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut buf: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    buf = xmlBufCreateSize(0 as size_t);
    if buf.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    xmlBufSetAllocationScheme(buf, XML_BUFFER_ALLOC_DOUBLEIT);
    q = cur;
    loop {
        if !(*cur as c_int != 0 as c_int) {
            current_block = 18383263831861166299;
            break;
        }
        if *cur.offset(0 as c_int as isize) as c_int == '&' as i32 {
            let mut charval: c_int = 0 as c_int;
            let mut tmp: xmlChar = 0;
            if cur != q {
                if xmlBufAdd(
                    buf,
                    q,
                    cur.offset_from(q) as c_long as c_int,
                ) != 0
                {
                    current_block = 6084808002588646011;
                    break;
                }
            }
            q = cur;
            if *cur.offset(1 as c_int as isize) as c_int == '#' as i32
                && *cur.offset(2 as c_int as isize) as c_int == 'x' as i32
            {
                cur = cur.offset(3 as c_int as isize);
                tmp = *cur;
                while tmp as c_int != ';' as i32 {
                    if tmp as c_int >= '0' as i32
                        && tmp as c_int <= '9' as i32
                    {
                        charval = charval * 16 as c_int
                            + (tmp as c_int - '0' as i32);
                    } else if tmp as c_int >= 'a' as i32
                        && tmp as c_int <= 'f' as i32
                    {
                        charval = charval * 16 as c_int
                            + (tmp as c_int - 'a' as i32)
                            + 10 as c_int;
                    } else if tmp as c_int >= 'A' as i32
                        && tmp as c_int <= 'F' as i32
                    {
                        charval = charval * 16 as c_int
                            + (tmp as c_int - 'A' as i32)
                            + 10 as c_int;
                    } else {
                        xmlTreeErr(
                            XML_TREE_INVALID_HEX as c_int,
                            doc as xmlNodePtr,
                            ::core::ptr::null::<c_char>(),
                        );
                        charval = 0 as c_int;
                        break;
                    }
                    cur = cur.offset(1);
                    tmp = *cur;
                }
                if tmp as c_int == ';' as i32 {
                    cur = cur.offset(1);
                }
                q = cur;
            } else if *cur.offset(1 as c_int as isize) as c_int
                == '#' as i32
            {
                cur = cur.offset(2 as c_int as isize);
                tmp = *cur;
                while tmp as c_int != ';' as i32 {
                    if tmp as c_int >= '0' as i32
                        && tmp as c_int <= '9' as i32
                    {
                        charval = charval * 10 as c_int
                            + (tmp as c_int - '0' as i32);
                        cur = cur.offset(1);
                        tmp = *cur;
                    } else {
                        xmlTreeErr(
                            XML_TREE_INVALID_DEC as c_int,
                            doc as xmlNodePtr,
                            ::core::ptr::null::<c_char>(),
                        );
                        charval = 0 as c_int;
                        break;
                    }
                }
                if tmp as c_int == ';' as i32 {
                    cur = cur.offset(1);
                }
                q = cur;
            } else {
                cur = cur.offset(1);
                q = cur;
                while *cur as c_int != 0 as c_int
                    && *cur as c_int != ';' as i32
                {
                    cur = cur.offset(1);
                }
                if *cur as c_int == 0 as c_int {
                    xmlTreeErr(
                        XML_TREE_UNTERMINATED_ENTITY as c_int,
                        doc as xmlNodePtr,
                        q as *const c_char,
                    );
                    current_block = 6084808002588646011;
                    break;
                } else {
                    if cur != q {
                        val = xmlStrndup(
                            q,
                            cur.offset_from(q) as c_long as c_int,
                        );
                        ent = xmlGetDocEntity(doc, val);
                        if !ent.is_null()
                            && (*ent).etype as c_uint
                                == XML_INTERNAL_PREDEFINED_ENTITY as c_int
                                    as c_uint
                        {
                            if xmlBufCat(buf, (*ent).content) != 0 {
                                current_block = 6084808002588646011;
                                break;
                            }
                        } else {
                            if xmlBufIsEmpty(buf) == 0 {
                                node = xmlNewDocText(doc, ::core::ptr::null::<xmlChar>());
                                if node.is_null() {
                                    current_block = 6084808002588646011;
                                    break;
                                }
                                (*node).content = xmlBufDetach(buf);
                                if last.is_null() {
                                    head = node;
                                    last = head;
                                } else {
                                    last = xmlAddNextSibling(last, node);
                                }
                            }
                            node = xmlNewReference(doc, val);
                            if node.is_null() {
                                current_block = 6084808002588646011;
                                break;
                            }
                            if !ent.is_null()
                                && (*ent).flags & XML_ENT_PARSED == 0 as c_int
                                && (*ent).flags & XML_ENT_EXPANDING == 0 as c_int
                            {
                                let mut temp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                                (*ent).flags |= XML_ENT_EXPANDING;
                                (*ent).children =
                                    xmlStringGetNodeList(doc, (*node).content as *const xmlChar)
                                        as *mut _xmlNode;
                                (*ent).owner = 1 as c_int;
                                (*ent).flags &= !XML_ENT_EXPANDING;
                                (*ent).flags |= XML_ENT_PARSED;
                                temp = (*ent).children as xmlNodePtr;
                                while !temp.is_null() {
                                    (*temp).parent = ent as xmlNodePtr as *mut _xmlNode;
                                    (*ent).last = temp as *mut _xmlNode;
                                    temp = (*temp).next as xmlNodePtr;
                                }
                            }
                            if last.is_null() {
                                head = node;
                                last = head;
                            } else {
                                last = xmlAddNextSibling(last, node);
                            }
                        }
                        xmlFree.expect("non-null function pointer")(
                            val as *mut c_void,
                        );
                        val = ::core::ptr::null_mut::<xmlChar>();
                    }
                    cur = cur.offset(1);
                    q = cur;
                }
            }
            if !(charval != 0 as c_int) {
                continue;
            }
            let mut buffer: [xmlChar; 10] = [0; 10];
            let mut len: c_int = 0;
            len = xmlCopyCharMultiByte(&raw mut buffer as *mut xmlChar, charval);
            buffer[len as usize] = 0 as xmlChar;
            if xmlBufCat(buf, &raw mut buffer as *mut xmlChar) != 0 {
                current_block = 6084808002588646011;
                break;
            }
            charval = 0 as c_int;
        } else {
            cur = cur.offset(1);
        }
    }
    match current_block {
        18383263831861166299 => {
            if cur != q || head.is_null() {
                xmlBufAdd(
                    buf,
                    q,
                    cur.offset_from(q) as c_long as c_int,
                );
            }
            if xmlBufIsEmpty(buf) == 0 {
                node = xmlNewDocText(doc, ::core::ptr::null::<xmlChar>());
                if node.is_null() {
                    current_block = 6084808002588646011;
                } else {
                    (*node).content = xmlBufDetach(buf);
                    if last.is_null() {
                        head = node;
                    } else {
                        xmlAddNextSibling(last, node);
                    }
                    current_block = 17769492591016358583;
                }
            } else {
                current_block = 17769492591016358583;
            }
            match current_block {
                6084808002588646011 => {}
                _ => {
                    ret = head;
                    head = ::core::ptr::null_mut::<xmlNode>();
                }
            }
        }
        _ => {}
    }
    xmlBufFree(buf);
    if !val.is_null() {
        xmlFree.expect("non-null function pointer")(val as *mut c_void);
    }
    if !head.is_null() {
        xmlFreeNodeList(head);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNodeListGetString(
    mut doc: xmlDocPtr,
    mut list: *const xmlNode,
    mut inLine: c_int,
) -> *mut xmlChar {
    let mut node: *const xmlNode = list;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    let mut attr: c_int = 0;
    if list.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*list).parent.is_null()
        && (*(*list).parent).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        attr = 1 as c_int;
    } else {
        attr = 0 as c_int;
    }
    while !node.is_null() {
        if (*node).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_CDATA_SECTION_NODE as c_int as c_uint
        {
            if inLine != 0 {
                ret = xmlStrcat(ret, (*node).content);
            } else {
                let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                if attr != 0 {
                    buffer = xmlEncodeAttributeEntities(doc, (*node).content);
                } else {
                    buffer = xmlEncodeEntitiesReentrant(doc, (*node).content);
                }
                if !buffer.is_null() {
                    ret = xmlStrcat(ret, buffer);
                    xmlFree.expect("non-null function pointer")(buffer as *mut c_void);
                }
            }
        } else if (*node).type_0 as c_uint
            == XML_ENTITY_REF_NODE as c_int as c_uint
        {
            if inLine != 0 {
                ent = xmlGetDocEntity(doc as *const xmlDoc, (*node).name);
                if !ent.is_null() {
                    let mut buffer_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    buffer_0 = xmlNodeListGetString(doc, (*ent).children, 1 as c_int);
                    if !buffer_0.is_null() {
                        ret = xmlStrcat(ret, buffer_0);
                        xmlFree.expect("non-null function pointer")(
                            buffer_0 as *mut c_void,
                        );
                    }
                } else {
                    ret = xmlStrcat(ret, (*node).content);
                }
            } else {
                let mut buf: [xmlChar; 2] = [0; 2];
                buf[0 as c_int as usize] = '&' as i32 as xmlChar;
                buf[1 as c_int as usize] = 0 as xmlChar;
                ret = xmlStrncat(ret, &raw mut buf as *mut xmlChar, 1 as c_int);
                ret = xmlStrcat(ret, (*node).name);
                buf[0 as c_int as usize] = ';' as i32 as xmlChar;
                buf[1 as c_int as usize] = 0 as xmlChar;
                ret = xmlStrncat(ret, &raw mut buf as *mut xmlChar, 1 as c_int);
            }
        }
        node = (*node).next;
    }
    return ret;
}
#[inline]
pub unsafe fn xmlNodeListGetRawString(
    mut doc: *const xmlDoc,
    mut list: *const xmlNode,
    mut inLine: c_int,
) -> *mut xmlChar {
    let mut node: *const xmlNode = list;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    if list.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while !node.is_null() {
        if (*node).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_CDATA_SECTION_NODE as c_int as c_uint
        {
            if inLine != 0 {
                ret = xmlStrcat(ret, (*node).content);
            } else {
                let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                buffer = xmlEncodeSpecialChars(doc, (*node).content);
                if !buffer.is_null() {
                    ret = xmlStrcat(ret, buffer);
                    xmlFree.expect("non-null function pointer")(buffer as *mut c_void);
                }
            }
        } else if (*node).type_0 as c_uint
            == XML_ENTITY_REF_NODE as c_int as c_uint
        {
            if inLine != 0 {
                ent = xmlGetDocEntity(doc, (*node).name);
                if !ent.is_null() {
                    let mut buffer_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    buffer_0 =
                        xmlNodeListGetRawString(doc, (*ent).children, 1 as c_int);
                    if !buffer_0.is_null() {
                        ret = xmlStrcat(ret, buffer_0);
                        xmlFree.expect("non-null function pointer")(
                            buffer_0 as *mut c_void,
                        );
                    }
                } else {
                    ret = xmlStrcat(ret, (*node).content);
                }
            } else {
                let mut buf: [xmlChar; 2] = [0; 2];
                buf[0 as c_int as usize] = '&' as i32 as xmlChar;
                buf[1 as c_int as usize] = 0 as xmlChar;
                ret = xmlStrncat(ret, &raw mut buf as *mut xmlChar, 1 as c_int);
                ret = xmlStrcat(ret, (*node).name);
                buf[0 as c_int as usize] = ';' as i32 as xmlChar;
                buf[1 as c_int as usize] = 0 as xmlChar;
                ret = xmlStrncat(ret, &raw mut buf as *mut xmlChar, 1 as c_int);
            }
        }
        node = (*node).next;
    }
    return ret;
}
unsafe fn xmlNewPropInternal(
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
    mut eatname: c_int,
) -> xmlAttrPtr {
    let mut cur: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if !node.is_null()
        && (*node).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        if eatname == 1 as c_int
            && ((*node).doc.is_null()
                || (*(*node).doc).dict.is_null()
                || xmlDictOwns((*(*node).doc).dict as xmlDictPtr, name) == 0)
        {
            xmlFree.expect("non-null function pointer")(
                name as *mut xmlChar as *mut c_void,
            );
        }
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlAttr>() as size_t)
        as xmlAttrPtr;
    if cur.is_null() {
        if eatname == 1 as c_int
            && (node.is_null()
                || (*node).doc.is_null()
                || (*(*node).doc).dict.is_null()
                || xmlDictOwns((*(*node).doc).dict as xmlDictPtr, name) == 0)
        {
            xmlFree.expect("non-null function pointer")(
                name as *mut xmlChar as *mut c_void,
            );
        }
        xmlTreeErrMemory(b"building attribute\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlAttr>() as size_t,
    );
    (*cur).type_0 = XML_ATTRIBUTE_NODE;
    (*cur).parent = node as *mut _xmlNode;
    if !node.is_null() {
        doc = (*node).doc as xmlDocPtr;
        (*cur).doc = doc as *mut _xmlDoc;
    }
    (*cur).ns = ns as *mut xmlNs;
    if eatname == 0 as c_int {
        if !doc.is_null() && !(*doc).dict.is_null() {
            (*cur).name = xmlDictLookup((*doc).dict as xmlDictPtr, name, -(1 as c_int))
                as *mut xmlChar;
        } else {
            (*cur).name = xmlStrdup(name);
        }
    } else {
        (*cur).name = name;
    }
    if !value.is_null() {
        let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        (*cur).children = xmlNewDocText(doc as *const xmlDoc, value) as *mut _xmlNode;
        (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
        tmp = (*cur).children as xmlNodePtr;
        while !tmp.is_null() {
            (*tmp).parent = cur as xmlNodePtr as *mut _xmlNode;
            if (*tmp).next.is_null() {
                (*cur).last = tmp as *mut _xmlNode;
            }
            tmp = (*tmp).next as xmlNodePtr;
        }
    }
    if !node.is_null() {
        if (*node).properties.is_null() {
            (*node).properties = cur as *mut _xmlAttr;
        } else {
            let mut prev: xmlAttrPtr = (*node).properties as xmlAttrPtr;
            while !(*prev).next.is_null() {
                prev = (*prev).next as xmlAttrPtr;
            }
            (*prev).next = cur as *mut _xmlAttr;
            (*cur).prev = prev as *mut _xmlAttr;
        }
    }
    if !value.is_null()
        && !node.is_null()
        && xmlIsID((*node).doc as xmlDocPtr, node, cur) == 1 as c_int
    {
        xmlAddID(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            (*node).doc as xmlDocPtr,
            value,
            cur,
        );
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewProp(
    mut node: xmlNodePtr,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
) -> xmlAttrPtr {
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    return xmlNewPropInternal(
        node,
        ::core::ptr::null_mut::<xmlNs>(),
        name,
        value,
        0 as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewNsProp(
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
) -> xmlAttrPtr {
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    return xmlNewPropInternal(node, ns, name, value, 0 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewNsPropEatName(
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
    mut name: *mut xmlChar,
    mut value: *const xmlChar,
) -> xmlAttrPtr {
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    return xmlNewPropInternal(node, ns, name, value, 1 as c_int);
}
#[inline]
pub unsafe fn xmlNewDocProp(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
) -> xmlAttrPtr {
    let mut cur: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlAttr>() as size_t)
        as xmlAttrPtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building attribute\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlAttr>() as size_t,
    );
    (*cur).type_0 = XML_ATTRIBUTE_NODE;
    if !doc.is_null() && !(*doc).dict.is_null() {
        (*cur).name = xmlDictLookup((*doc).dict as xmlDictPtr, name, -(1 as c_int));
    } else {
        (*cur).name = xmlStrdup(name);
    }
    (*cur).doc = doc as *mut _xmlDoc;
    if !value.is_null() {
        let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        (*cur).children = xmlStringGetNodeList(doc as *const xmlDoc, value) as *mut _xmlNode;
        (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
        tmp = (*cur).children as xmlNodePtr;
        while !tmp.is_null() {
            (*tmp).parent = cur as xmlNodePtr as *mut _xmlNode;
            if (*tmp).next.is_null() {
                (*cur).last = tmp as *mut _xmlNode;
            }
            tmp = (*tmp).next as xmlNodePtr;
        }
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    return cur;
}
#[no_mangle]
pub extern "C" fn xmlFreePropList(mut cur: xmlAttrPtr) { unsafe {
    let mut next: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if cur.is_null() {
        return;
    }
    while !cur.is_null() {
        next = (*cur).next as xmlAttrPtr;
        xmlFreeProp(cur);
        cur = next;
    }
} }
#[inline]
pub fn xmlFreeProp(mut cur: xmlAttrPtr) { unsafe {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if cur.is_null() {
        return;
    }
    if !(*cur).doc.is_null() {
        dict = (*(*cur).doc).dict as xmlDictPtr;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur as xmlNodePtr);
    }
    if !(*cur).doc.is_null()
        && (*cur).atype as c_uint
            == XML_ATTRIBUTE_ID as c_int as c_uint
    {
        xmlRemoveID((*cur).doc as xmlDocPtr, cur);
    }
    if !(*cur).children.is_null() {
        xmlFreeNodeList((*cur).children as xmlNodePtr);
    }
    if !(*cur).name.is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).name as *mut c_char as *mut c_void,
        );
    }
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
} }
#[inline]
pub fn xmlRemoveProp(mut cur: xmlAttrPtr) -> c_int { unsafe {
    let mut tmp: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if cur.is_null() {
        return -(1 as c_int);
    }
    if (*cur).parent.is_null() {
        return -(1 as c_int);
    }
    tmp = (*(*cur).parent).properties as xmlAttrPtr;
    if tmp == cur {
        (*(*cur).parent).properties = (*cur).next;
        if !(*cur).next.is_null() {
            (*(*cur).next).prev = ::core::ptr::null_mut::<_xmlAttr>();
        }
        xmlFreeProp(cur);
        return 0 as c_int;
    }
    while !tmp.is_null() {
        if (*tmp).next == cur {
            (*tmp).next = (*cur).next;
            if !(*tmp).next.is_null() {
                (*(*tmp).next).prev = tmp as *mut _xmlAttr;
            }
            xmlFreeProp(cur);
            return 0 as c_int;
        }
        tmp = (*tmp).next as xmlAttrPtr;
    }
    return -(1 as c_int);
} }
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocPI(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building PI\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_PI_NODE;
    if !doc.is_null() && !(*doc).dict.is_null() {
        (*cur).name = xmlDictLookup((*doc).dict as xmlDictPtr, name, -(1 as c_int));
    } else {
        (*cur).name = xmlStrdup(name);
    }
    if !content.is_null() {
        (*cur).content = xmlStrdup(content);
    }
    (*cur).doc = doc as *mut _xmlDoc;
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[inline]
pub unsafe fn xmlNewPI(
    mut name: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    return xmlNewDocPI(::core::ptr::null_mut::<xmlDoc>(), name, content);
}
#[inline]
pub unsafe fn xmlNewNode(mut ns: xmlNsPtr, mut name: *const xmlChar) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building node\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_ELEMENT_NODE;
    (*cur).name = xmlStrdup(name);
    (*cur).ns = ns as *mut xmlNs;
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[inline]
pub unsafe fn xmlNewNodeEatName(mut ns: xmlNsPtr, mut name: *mut xmlChar) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building node\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_ELEMENT_NODE;
    (*cur).name = name;
    (*cur).ns = ns as *mut xmlNs;
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocNode(
    mut doc: xmlDocPtr,
    mut ns: xmlNsPtr,
    mut name: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if !doc.is_null() && !(*doc).dict.is_null() {
        cur = xmlNewNodeEatName(
            ns,
            xmlDictLookup((*doc).dict as xmlDictPtr, name, -(1 as c_int))
                as *mut xmlChar,
        );
    } else {
        cur = xmlNewNode(ns, name);
    }
    if !cur.is_null() {
        (*cur).doc = doc as *mut _xmlDoc;
        if !content.is_null() {
            (*cur).children = xmlStringGetNodeList(doc as *const xmlDoc, content) as *mut _xmlNode;
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children as xmlNodePtr;
                if ulccur.is_null() {
                    (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur as *mut _xmlNode;
                        ulccur = (*ulccur).next as xmlNodePtr;
                    }
                    (*ulccur).parent = cur as *mut _xmlNode;
                    (*cur).last = ulccur as *mut _xmlNode;
                }
            }
        }
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocNodeEatName(
    mut doc: xmlDocPtr,
    mut ns: xmlNsPtr,
    mut name: *mut xmlChar,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlNewNodeEatName(ns, name);
    if !cur.is_null() {
        (*cur).doc = doc as *mut _xmlDoc;
        if !content.is_null() {
            (*cur).children = xmlStringGetNodeList(doc as *const xmlDoc, content) as *mut _xmlNode;
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children as xmlNodePtr;
                if ulccur.is_null() {
                    (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur as *mut _xmlNode;
                        ulccur = (*ulccur).next as xmlNodePtr;
                    }
                    (*ulccur).parent = cur as *mut _xmlNode;
                    (*cur).last = ulccur as *mut _xmlNode;
                }
            }
        }
    } else if !name.is_null()
        && (doc.is_null()
            || (*doc).dict.is_null()
            || xmlDictOwns((*doc).dict as xmlDictPtr, name) == 0)
    {
        xmlFree.expect("non-null function pointer")(name as *mut c_void);
    }
    return cur;
}
#[inline]
pub unsafe fn xmlNewDocRawNode(
    mut doc: xmlDocPtr,
    mut ns: xmlNsPtr,
    mut name: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlNewDocNode(doc, ns, name, ::core::ptr::null::<xmlChar>());
    if !cur.is_null() {
        (*cur).doc = doc as *mut _xmlDoc;
        if !content.is_null() {
            (*cur).children = xmlNewDocText(doc as *const xmlDoc, content) as *mut _xmlNode;
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children as xmlNodePtr;
                if ulccur.is_null() {
                    (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur as *mut _xmlNode;
                        ulccur = (*ulccur).next as xmlNodePtr;
                    }
                    (*ulccur).parent = cur as *mut _xmlNode;
                    (*cur).last = ulccur as *mut _xmlNode;
                }
            }
        }
    }
    return cur;
}
#[inline]
pub fn xmlNewDocFragment(mut doc: xmlDocPtr) -> xmlNodePtr { unsafe {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building fragment\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_DOCUMENT_FRAG_NODE;
    (*cur).doc = doc as *mut _xmlDoc;
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
} }
#[inline]
pub unsafe fn xmlNewText(mut content: *const xmlChar) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building text\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_TEXT_NODE;
    (*cur).name = &raw const xmlStringText as *const xmlChar;
    if !content.is_null() {
        (*cur).content = xmlStrdup(content);
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[inline]
pub unsafe fn xmlNewTextChild(
    mut parent: xmlNodePtr,
    mut ns: xmlNsPtr,
    mut name: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut prev: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if parent.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*parent).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
    {
        if ns.is_null() {
            cur = xmlNewDocRawNode(
                (*parent).doc as xmlDocPtr,
                (*parent).ns as xmlNsPtr,
                name,
                content,
            );
        } else {
            cur = xmlNewDocRawNode((*parent).doc as xmlDocPtr, ns, name, content);
        }
    } else if (*parent).type_0 as c_uint
        == XML_DOCUMENT_NODE as c_int as c_uint
        || (*parent).type_0 as c_uint
            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        if ns.is_null() {
            cur = xmlNewDocRawNode(
                parent as xmlDocPtr,
                ::core::ptr::null_mut::<xmlNs>(),
                name,
                content,
            );
        } else {
            cur = xmlNewDocRawNode(parent as xmlDocPtr, ns, name, content);
        }
    } else if (*parent).type_0 as c_uint
        == XML_DOCUMENT_FRAG_NODE as c_int as c_uint
    {
        cur = xmlNewDocRawNode((*parent).doc as xmlDocPtr, ns, name, content);
    } else {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    (*cur).type_0 = XML_ELEMENT_NODE;
    (*cur).parent = parent as *mut _xmlNode;
    (*cur).doc = (*parent).doc;
    if (*parent).children.is_null() {
        (*parent).children = cur as *mut _xmlNode;
        (*parent).last = cur as *mut _xmlNode;
    } else {
        prev = (*parent).last as xmlNodePtr;
        (*prev).next = cur as *mut _xmlNode;
        (*cur).prev = prev as *mut _xmlNode;
        (*parent).last = cur as *mut _xmlNode;
    }
    return cur;
}
#[inline]
pub unsafe fn xmlNewCharRef(mut doc: xmlDocPtr, mut name: *const xmlChar) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(
            b"building character reference\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_ENTITY_REF_NODE;
    (*cur).doc = doc as *mut _xmlDoc;
    if *name.offset(0 as c_int as isize) as c_int == '&' as i32 {
        let mut len: c_int = 0;
        name = name.offset(1);
        len = xmlStrlen(name);
        if *name.offset((len - 1 as c_int) as isize) as c_int
            == ';' as i32
        {
            (*cur).name = xmlStrndup(name, len - 1 as c_int);
        } else {
            (*cur).name = xmlStrndup(name, len);
        }
    } else {
        (*cur).name = xmlStrdup(name);
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewReference(
    mut doc: *const xmlDoc,
    mut name: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building reference\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_ENTITY_REF_NODE;
    (*cur).doc = doc as *mut xmlDoc as *mut _xmlDoc;
    if *name.offset(0 as c_int as isize) as c_int == '&' as i32 {
        let mut len: c_int = 0;
        name = name.offset(1);
        len = xmlStrlen(name);
        if *name.offset((len - 1 as c_int) as isize) as c_int
            == ';' as i32
        {
            (*cur).name = xmlStrndup(name, len - 1 as c_int);
        } else {
            (*cur).name = xmlStrndup(name, len);
        }
    } else {
        (*cur).name = xmlStrdup(name);
    }
    ent = xmlGetDocEntity(doc, (*cur).name);
    if !ent.is_null() {
        (*cur).content = (*ent).content;
        (*cur).children = ent as xmlNodePtr as *mut _xmlNode;
        (*cur).last = ent as xmlNodePtr as *mut _xmlNode;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocText(
    mut doc: *const xmlDoc,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlNewText(content);
    if !cur.is_null() {
        (*cur).doc = doc as *mut xmlDoc as *mut _xmlDoc;
    }
    return cur;
}
#[inline]
pub unsafe fn xmlNewTextLen(
    mut content: *const xmlChar,
    mut len: c_int,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building text\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_TEXT_NODE;
    (*cur).name = &raw const xmlStringText as *const xmlChar;
    if !content.is_null() {
        (*cur).content = xmlStrndup(content, len);
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[inline]
pub unsafe fn xmlNewDocTextLen(
    mut doc: xmlDocPtr,
    mut content: *const xmlChar,
    mut len: c_int,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlNewTextLen(content, len);
    if !cur.is_null() {
        (*cur).doc = doc as *mut _xmlDoc;
    }
    return cur;
}
#[inline]
pub unsafe fn xmlNewComment(mut content: *const xmlChar) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building comment\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_COMMENT_NODE;
    (*cur).name = &raw const xmlStringComment as *const xmlChar;
    if !content.is_null() {
        (*cur).content = xmlStrdup(content);
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewCDataBlock(
    mut doc: xmlDocPtr,
    mut content: *const xmlChar,
    mut len: c_int,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if cur.is_null() {
        xmlTreeErrMemory(b"building CDATA\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*cur).type_0 = XML_CDATA_SECTION_NODE;
    (*cur).doc = doc as *mut _xmlDoc;
    if !content.is_null() {
        (*cur).content = xmlStrndup(content, len);
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocComment(
    mut doc: xmlDocPtr,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    cur = xmlNewComment(content);
    if !cur.is_null() {
        (*cur).doc = doc as *mut _xmlDoc;
    }
    return cur;
}
unsafe fn _copyStringForNewDictIfNeeded(
    mut oldDict: xmlDictPtr,
    mut newDict: xmlDictPtr,
    mut oldValue: *const xmlChar,
) -> *const xmlChar {
    let mut newValue: *const xmlChar = oldValue;
    if !oldValue.is_null() {
        let mut oldDictOwnsOldValue: c_int = (!oldDict.is_null()
            && xmlDictOwns(oldDict, oldValue) == 1 as c_int)
            as c_int;
        if oldDictOwnsOldValue != 0 {
            if !newDict.is_null() {
                newValue = xmlDictLookup(newDict, oldValue, -(1 as c_int));
            } else {
                newValue = xmlStrdup(oldValue);
            }
        }
    }
    return newValue;
}
#[no_mangle]
pub extern "C" fn xmlSetTreeDoc(mut tree: xmlNodePtr, mut doc: xmlDocPtr) { unsafe {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if tree.is_null()
        || (*tree).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return;
    }
    if (*tree).doc != doc {
        let mut oldTreeDict: xmlDictPtr = if !(*tree).doc.is_null() {
            (*(*tree).doc).dict as xmlDictPtr
        } else {
            ::core::ptr::null_mut::<xmlDict>()
        };
        let mut newDict: xmlDictPtr = if !doc.is_null() {
            (*doc).dict as xmlDictPtr
        } else {
            ::core::ptr::null_mut::<xmlDict>()
        };
        if (*tree).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            prop = (*tree).properties as xmlAttrPtr;
            while !prop.is_null() {
                if (*prop).atype as c_uint
                    == XML_ATTRIBUTE_ID as c_int as c_uint
                {
                    xmlRemoveID((*tree).doc as xmlDocPtr, prop);
                }
                if (*prop).doc != doc {
                    let mut oldPropDict: xmlDictPtr = if !(*prop).doc.is_null() {
                        (*(*prop).doc).dict as xmlDictPtr
                    } else {
                        ::core::ptr::null_mut::<xmlDict>()
                    };
                    (*prop).name =
                        _copyStringForNewDictIfNeeded(oldPropDict, newDict, (*prop).name);
                    (*prop).doc = doc as *mut _xmlDoc;
                }
                xmlSetListDoc((*prop).children as xmlNodePtr, doc);
                prop = (*prop).next as xmlAttrPtr;
            }
        }
        if (*tree).type_0 as c_uint
            == XML_ENTITY_REF_NODE as c_int as c_uint
        {
            (*tree).children = ::core::ptr::null_mut::<_xmlNode>();
        } else if !(*tree).children.is_null() {
            xmlSetListDoc((*tree).children as xmlNodePtr, doc);
        }
        (*tree).name = _copyStringForNewDictIfNeeded(oldTreeDict, newDict, (*tree).name);
        (*tree).content = _copyStringForNewDictIfNeeded(
            oldTreeDict,
            ::core::ptr::null_mut::<xmlDict>(),
            (*tree).content,
        ) as *mut xmlChar;
        (*tree).doc = doc as *mut _xmlDoc;
    }
} }
#[inline]
pub fn xmlSetListDoc(mut list: xmlNodePtr, mut doc: xmlDocPtr) { unsafe {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if list.is_null()
        || (*list).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return;
    }
    cur = list;
    while !cur.is_null() {
        if (*cur).doc != doc {
            xmlSetTreeDoc(cur, doc);
        }
        cur = (*cur).next as xmlNodePtr;
    }
} }
#[no_mangle]
pub unsafe extern "C" fn xmlNewChild(
    mut parent: xmlNodePtr,
    mut ns: xmlNsPtr,
    mut name: *const xmlChar,
    mut content: *const xmlChar,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut prev: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if parent.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*parent).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
    {
        if ns.is_null() {
            cur = xmlNewDocNode(
                (*parent).doc as xmlDocPtr,
                (*parent).ns as xmlNsPtr,
                name,
                content,
            );
        } else {
            cur = xmlNewDocNode((*parent).doc as xmlDocPtr, ns, name, content);
        }
    } else if (*parent).type_0 as c_uint
        == XML_DOCUMENT_NODE as c_int as c_uint
        || (*parent).type_0 as c_uint
            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        if ns.is_null() {
            cur = xmlNewDocNode(
                parent as xmlDocPtr,
                ::core::ptr::null_mut::<xmlNs>(),
                name,
                content,
            );
        } else {
            cur = xmlNewDocNode(parent as xmlDocPtr, ns, name, content);
        }
    } else if (*parent).type_0 as c_uint
        == XML_DOCUMENT_FRAG_NODE as c_int as c_uint
    {
        cur = xmlNewDocNode((*parent).doc as xmlDocPtr, ns, name, content);
    } else {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    (*cur).type_0 = XML_ELEMENT_NODE;
    (*cur).parent = parent as *mut _xmlNode;
    (*cur).doc = (*parent).doc;
    if (*parent).children.is_null() {
        (*parent).children = cur as *mut _xmlNode;
        (*parent).last = cur as *mut _xmlNode;
    } else {
        prev = (*parent).last as xmlNodePtr;
        (*prev).next = cur as *mut _xmlNode;
        (*cur).prev = prev as *mut _xmlNode;
        (*parent).last = cur as *mut _xmlNode;
    }
    return cur;
}
fn xmlAddPropSibling(
    mut prev: xmlNodePtr,
    mut cur: xmlNodePtr,
    mut prop: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if cur.is_null()
        || (*cur).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
        || prop.is_null()
        || (*prop).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
        || !prev.is_null()
            && (*prev).type_0 as c_uint
                != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*prop).ns.is_null() {
        attr = xmlHasNsProp((*cur).parent, (*prop).name, ::core::ptr::null::<xmlChar>());
    } else {
        attr = xmlHasNsProp((*cur).parent, (*prop).name, (*(*prop).ns).href);
    }
    if (*prop).doc != (*cur).doc {
        xmlSetTreeDoc(prop, (*cur).doc as xmlDocPtr);
    }
    (*prop).parent = (*cur).parent;
    (*prop).prev = prev as *mut _xmlNode;
    if !prev.is_null() {
        (*prop).next = (*prev).next;
        (*prev).next = prop as *mut _xmlNode;
        if !(*prop).next.is_null() {
            (*(*prop).next).prev = prop as *mut _xmlNode;
        }
    } else {
        (*prop).next = cur as *mut _xmlNode;
        (*cur).prev = prop as *mut _xmlNode;
    }
    if (*prop).prev.is_null() && !(*prop).parent.is_null() {
        (*(*prop).parent).properties = prop as xmlAttrPtr as *mut _xmlAttr;
    }
    if !attr.is_null()
        && (*attr).type_0 as c_uint
            != XML_ATTRIBUTE_DECL as c_int as c_uint
    {
        xmlRemoveProp(attr);
    }
    return prop;
} }
#[no_mangle]
pub extern "C" fn xmlAddNextSibling(
    mut cur: xmlNodePtr,
    mut elem: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if cur.is_null()
        || (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if elem.is_null()
        || (*elem).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur == elem {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    xmlUnlinkNode(elem);
    if (*elem).type_0 as c_uint
        == XML_TEXT_NODE as c_int as c_uint
    {
        if (*cur).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
        {
            xmlNodeAddContent(cur, (*elem).content);
            xmlFreeNode(elem);
            return cur;
        }
        if !(*cur).next.is_null()
            && (*(*cur).next).type_0 as c_uint
                == XML_TEXT_NODE as c_int as c_uint
            && (*cur).name == (*(*cur).next).name
        {
            let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            tmp = xmlStrdup((*elem).content);
            tmp = xmlStrcat(tmp, (*(*cur).next).content);
            xmlNodeSetContent((*cur).next as xmlNodePtr, tmp);
            xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
            xmlFreeNode(elem);
            return (*cur).next as xmlNodePtr;
        }
    } else if (*elem).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return xmlAddPropSibling(cur, cur, elem);
    }
    if (*elem).doc != (*cur).doc {
        xmlSetTreeDoc(elem, (*cur).doc as xmlDocPtr);
    }
    (*elem).parent = (*cur).parent;
    (*elem).prev = cur as *mut _xmlNode;
    (*elem).next = (*cur).next;
    (*cur).next = elem as *mut _xmlNode;
    if !(*elem).next.is_null() {
        (*(*elem).next).prev = elem as *mut _xmlNode;
    }
    if !(*elem).parent.is_null() && (*(*elem).parent).last == cur {
        (*(*elem).parent).last = elem as *mut _xmlNode;
    }
    return elem;
} }
#[no_mangle]
pub extern "C" fn xmlAddPrevSibling(
    mut cur: xmlNodePtr,
    mut elem: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if cur.is_null()
        || (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if elem.is_null()
        || (*elem).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur == elem {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    xmlUnlinkNode(elem);
    if (*elem).type_0 as c_uint
        == XML_TEXT_NODE as c_int as c_uint
    {
        if (*cur).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
        {
            let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            tmp = xmlStrdup((*elem).content);
            tmp = xmlStrcat(tmp, (*cur).content);
            xmlNodeSetContent(cur, tmp);
            xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
            xmlFreeNode(elem);
            return cur;
        }
        if !(*cur).prev.is_null()
            && (*(*cur).prev).type_0 as c_uint
                == XML_TEXT_NODE as c_int as c_uint
            && (*cur).name == (*(*cur).prev).name
        {
            xmlNodeAddContent((*cur).prev as xmlNodePtr, (*elem).content);
            xmlFreeNode(elem);
            return (*cur).prev as xmlNodePtr;
        }
    } else if (*elem).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return xmlAddPropSibling((*cur).prev as xmlNodePtr, cur, elem);
    }
    if (*elem).doc != (*cur).doc {
        xmlSetTreeDoc(elem, (*cur).doc as xmlDocPtr);
    }
    (*elem).parent = (*cur).parent;
    (*elem).next = cur as *mut _xmlNode;
    (*elem).prev = (*cur).prev;
    (*cur).prev = elem as *mut _xmlNode;
    if !(*elem).prev.is_null() {
        (*(*elem).prev).next = elem as *mut _xmlNode;
    }
    if !(*elem).parent.is_null() && (*(*elem).parent).children == cur {
        (*(*elem).parent).children = elem as *mut _xmlNode;
    }
    return elem;
} }
#[no_mangle]
pub extern "C" fn xmlAddSibling(mut cur: xmlNodePtr, mut elem: xmlNodePtr) -> xmlNodePtr { unsafe {
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if cur.is_null()
        || (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if elem.is_null()
        || (*elem).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur == elem {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*cur).type_0 as c_uint
        != XML_ATTRIBUTE_NODE as c_int as c_uint
        && !(*cur).parent.is_null()
        && !(*(*cur).parent).children.is_null()
        && !(*(*cur).parent).last.is_null()
        && (*(*(*cur).parent).last).next.is_null()
    {
        cur = (*(*cur).parent).last as xmlNodePtr;
    } else {
        while !(*cur).next.is_null() {
            cur = (*cur).next as xmlNodePtr;
        }
    }
    xmlUnlinkNode(elem);
    if (*cur).type_0 as c_uint
        == XML_TEXT_NODE as c_int as c_uint
        && (*elem).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
        && (*cur).name == (*elem).name
    {
        xmlNodeAddContent(cur, (*elem).content);
        xmlFreeNode(elem);
        return cur;
    } else if (*elem).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return xmlAddPropSibling(cur, cur, elem);
    }
    if (*elem).doc != (*cur).doc {
        xmlSetTreeDoc(elem, (*cur).doc as xmlDocPtr);
    }
    parent = (*cur).parent as xmlNodePtr;
    (*elem).prev = cur as *mut _xmlNode;
    (*elem).next = ::core::ptr::null_mut::<_xmlNode>();
    (*elem).parent = parent as *mut _xmlNode;
    (*cur).next = elem as *mut _xmlNode;
    if !parent.is_null() {
        (*parent).last = elem as *mut _xmlNode;
    }
    return elem;
} }
#[no_mangle]
pub extern "C" fn xmlAddChildList(
    mut parent: xmlNodePtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    let mut prev: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if parent.is_null()
        || (*parent).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null()
        || (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    !(*cur).doc.is_null() && !(*parent).doc.is_null() && (*cur).doc != (*parent).doc;
    if (*parent).children.is_null() {
        (*parent).children = cur as *mut _xmlNode;
    } else {
        if (*cur).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
            && (*(*parent).last).type_0 as c_uint
                == XML_TEXT_NODE as c_int as c_uint
            && (*cur).name == (*(*parent).last).name
        {
            xmlNodeAddContent((*parent).last as xmlNodePtr, (*cur).content);
            if (*cur).next.is_null() {
                xmlFreeNode(cur);
                return (*parent).last as xmlNodePtr;
            }
            prev = cur;
            cur = (*cur).next as xmlNodePtr;
            xmlFreeNode(prev);
        }
        prev = (*parent).last as xmlNodePtr;
        (*prev).next = cur as *mut _xmlNode;
        (*cur).prev = prev as *mut _xmlNode;
    }
    while !(*cur).next.is_null() {
        (*cur).parent = parent as *mut _xmlNode;
        if (*cur).doc != (*parent).doc {
            xmlSetTreeDoc(cur, (*parent).doc as xmlDocPtr);
        }
        cur = (*cur).next as xmlNodePtr;
    }
    (*cur).parent = parent as *mut _xmlNode;
    if (*cur).doc != (*parent).doc {
        xmlSetTreeDoc(cur, (*parent).doc as xmlDocPtr);
    }
    (*parent).last = cur as *mut _xmlNode;
    return cur;
} }
#[no_mangle]
pub extern "C" fn xmlAddChild(mut parent: xmlNodePtr, mut cur: xmlNodePtr) -> xmlNodePtr { unsafe {
    let mut prev: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if parent.is_null()
        || (*parent).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null()
        || (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if parent == cur {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*cur).type_0 as c_uint
        == XML_TEXT_NODE as c_int as c_uint
    {
        if (*parent).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
            && !(*parent).content.is_null()
            && (*parent).name == (*cur).name
        {
            xmlNodeAddContent(parent, (*cur).content);
            xmlFreeNode(cur);
            return parent;
        }
        if !(*parent).last.is_null()
            && (*(*parent).last).type_0 as c_uint
                == XML_TEXT_NODE as c_int as c_uint
            && (*(*parent).last).name == (*cur).name
            && (*parent).last != cur
        {
            xmlNodeAddContent((*parent).last as xmlNodePtr, (*cur).content);
            xmlFreeNode(cur);
            return (*parent).last as xmlNodePtr;
        }
    }
    prev = (*cur).parent as xmlNodePtr;
    (*cur).parent = parent as *mut _xmlNode;
    if (*cur).doc != (*parent).doc {
        xmlSetTreeDoc(cur, (*parent).doc as xmlDocPtr);
    }
    if prev == parent {
        return cur;
    }
    if (*parent).type_0 as c_uint
        == XML_TEXT_NODE as c_int as c_uint
        && !(*parent).content.is_null()
        && parent != cur
    {
        xmlNodeAddContent(parent, (*cur).content);
        xmlFreeNode(cur);
        return parent;
    }
    if (*cur).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        if (*parent).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
        {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if !(*parent).properties.is_null() {
            let mut lastattr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
            if (*cur).ns.is_null() {
                lastattr = xmlHasNsProp(
                    parent as *const xmlNode,
                    (*cur).name,
                    ::core::ptr::null::<xmlChar>(),
                );
            } else {
                lastattr = xmlHasNsProp(parent as *const xmlNode, (*cur).name, (*(*cur).ns).href);
            }
            if !lastattr.is_null()
                && lastattr != cur as xmlAttrPtr
                && (*lastattr).type_0 as c_uint
                    != XML_ATTRIBUTE_DECL as c_int as c_uint
            {
                xmlUnlinkNode(lastattr as xmlNodePtr);
                xmlFreeProp(lastattr);
            }
            if lastattr == cur as xmlAttrPtr {
                return cur;
            }
        }
        if (*parent).properties.is_null() {
            (*parent).properties = cur as xmlAttrPtr as *mut _xmlAttr;
        } else {
            let mut lastattr_0: xmlAttrPtr = (*parent).properties as xmlAttrPtr;
            while !(*lastattr_0).next.is_null() {
                lastattr_0 = (*lastattr_0).next as xmlAttrPtr;
            }
            (*lastattr_0).next = cur as xmlAttrPtr as *mut _xmlAttr;
            let ref mut fresh3 = (*(cur as xmlAttrPtr)).prev;
            *fresh3 = lastattr_0 as *mut _xmlAttr;
        }
    } else if (*parent).children.is_null() {
        (*parent).children = cur as *mut _xmlNode;
        (*parent).last = cur as *mut _xmlNode;
    } else {
        prev = (*parent).last as xmlNodePtr;
        (*prev).next = cur as *mut _xmlNode;
        (*cur).prev = prev as *mut _xmlNode;
        (*parent).last = cur as *mut _xmlNode;
    }
    return cur;
} }
#[no_mangle]
pub unsafe extern "C" fn xmlGetLastChild(mut parent: *const xmlNode) -> xmlNodePtr {
    if parent.is_null()
        || (*parent).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    return (*parent).last as xmlNodePtr;
}
#[inline]
pub fn xmlChildElementCount(mut parent: xmlNodePtr) -> c_ulong { unsafe {
    let mut ret: c_ulong = 0 as c_ulong;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if parent.is_null() {
        return 0 as c_ulong;
    }
    match (*parent).type_0 as c_uint {
        1 | 6 | 9 | 11 | 13 => {
            cur = (*parent).children as xmlNodePtr;
        }
        _ => return 0 as c_ulong,
    }
    while !cur.is_null() {
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            ret = ret.wrapping_add(1);
        }
        cur = (*cur).next as xmlNodePtr;
    }
    return ret;
} }
#[inline]
pub fn xmlFirstElementChild(mut parent: xmlNodePtr) -> xmlNodePtr { unsafe {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if parent.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    match (*parent).type_0 as c_uint {
        1 | 6 | 9 | 11 | 13 => {
            cur = (*parent).children as xmlNodePtr;
        }
        _ => return ::core::ptr::null_mut::<xmlNode>(),
    }
    while !cur.is_null() {
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            return cur;
        }
        cur = (*cur).next as xmlNodePtr;
    }
    return ::core::ptr::null_mut::<xmlNode>();
} }
#[inline]
pub fn xmlLastElementChild(mut parent: xmlNodePtr) -> xmlNodePtr { unsafe {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if parent.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    match (*parent).type_0 as c_uint {
        1 | 6 | 9 | 11 | 13 => {
            cur = (*parent).last as xmlNodePtr;
        }
        _ => return ::core::ptr::null_mut::<xmlNode>(),
    }
    while !cur.is_null() {
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            return cur;
        }
        cur = (*cur).prev as xmlNodePtr;
    }
    return ::core::ptr::null_mut::<xmlNode>();
} }
#[inline]
pub fn xmlPreviousElementSibling(mut node: xmlNodePtr) -> xmlNodePtr { unsafe {
    if node.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    match (*node).type_0 as c_uint {
        1 | 3 | 4 | 5 | 6 | 7 | 8 | 19 | 20 => {
            node = (*node).prev as xmlNodePtr;
        }
        _ => return ::core::ptr::null_mut::<xmlNode>(),
    }
    while !node.is_null() {
        if (*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            return node;
        }
        node = (*node).prev as xmlNodePtr;
    }
    return ::core::ptr::null_mut::<xmlNode>();
} }
#[inline]
pub fn xmlNextElementSibling(mut node: xmlNodePtr) -> xmlNodePtr { unsafe {
    if node.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    match (*node).type_0 as c_uint {
        1 | 3 | 4 | 5 | 6 | 7 | 8 | 14 | 19 | 20 => {
            node = (*node).next as xmlNodePtr;
        }
        _ => return ::core::ptr::null_mut::<xmlNode>(),
    }
    while !node.is_null() {
        if (*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            return node;
        }
        node = (*node).next as xmlNodePtr;
    }
    return ::core::ptr::null_mut::<xmlNode>();
} }
#[no_mangle]
pub extern "C" fn xmlFreeNodeList(mut cur: xmlNodePtr) { unsafe {
    let mut next: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    let mut depth: size_t = 0 as size_t;
    if cur.is_null() {
        return;
    }
    if (*cur).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        xmlFreeNsList(cur as xmlNsPtr);
        return;
    }
    if !(*cur).doc.is_null() {
        dict = (*(*cur).doc).dict as xmlDictPtr;
    }
    loop {
        while !(*cur).children.is_null()
            && (*cur).type_0 as c_uint
                != XML_DOCUMENT_NODE as c_int as c_uint
            && (*cur).type_0 as c_uint
                != XML_HTML_DOCUMENT_NODE as c_int as c_uint
            && (*cur).type_0 as c_uint
                != XML_DTD_NODE as c_int as c_uint
            && (*cur).type_0 as c_uint
                != XML_ENTITY_REF_NODE as c_int as c_uint
        {
            cur = (*cur).children as xmlNodePtr;
            depth = (depth as c_ulong).wrapping_add(1 as c_ulong)
                as size_t as size_t;
        }
        next = (*cur).next as xmlNodePtr;
        parent = (*cur).parent as xmlNodePtr;
        if (*cur).type_0 as c_uint
            == XML_DOCUMENT_NODE as c_int as c_uint
            || (*cur).type_0 as c_uint
                == XML_HTML_DOCUMENT_NODE as c_int as c_uint
        {
            xmlFreeDoc(cur as xmlDocPtr);
        } else if (*cur).type_0 as c_uint
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
                xmlFreePropList((*cur).properties as xmlAttrPtr);
            }
            if (*cur).type_0 as c_uint
                != XML_ELEMENT_NODE as c_int as c_uint
                && (*cur).type_0 as c_uint
                    != XML_XINCLUDE_START as c_int as c_uint
                && (*cur).type_0 as c_uint
                    != XML_XINCLUDE_END as c_int as c_uint
                && (*cur).type_0 as c_uint
                    != XML_ENTITY_REF_NODE as c_int as c_uint
                && (*cur).content != &raw mut (*cur).properties as *mut xmlChar
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
            if !(*cur).name.is_null()
                && (*cur).type_0 as c_uint
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
            xmlFree.expect("non-null function pointer")(cur as *mut c_void);
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
#[no_mangle]
pub extern "C" fn xmlFreeNode(mut cur: xmlNodePtr) { unsafe {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if cur.is_null() {
        return;
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
        xmlFreeProp(cur as xmlAttrPtr);
        return;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    if !(*cur).doc.is_null() {
        dict = (*(*cur).doc).dict as xmlDictPtr;
    }
    if (*cur).type_0 as c_uint
        == XML_ENTITY_DECL as c_int as c_uint
    {
        let mut ent: xmlEntityPtr = cur as xmlEntityPtr;
        if !(*ent).SystemID.is_null()
            && (dict.is_null() || xmlDictOwns(dict, (*ent).SystemID) == 0 as c_int)
        {
            xmlFree.expect("non-null function pointer")(
                (*ent).SystemID as *mut c_char as *mut c_void,
            );
        }
        if !(*ent).ExternalID.is_null()
            && (dict.is_null() || xmlDictOwns(dict, (*ent).ExternalID) == 0 as c_int)
        {
            xmlFree.expect("non-null function pointer")(
                (*ent).ExternalID as *mut c_char as *mut c_void,
            );
        }
    }
    if !(*cur).children.is_null()
        && (*cur).type_0 as c_uint
            != XML_ENTITY_REF_NODE as c_int as c_uint
    {
        xmlFreeNodeList((*cur).children as xmlNodePtr);
    }
    if (*cur).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_XINCLUDE_START as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_XINCLUDE_END as c_int as c_uint
    {
        if !(*cur).properties.is_null() {
            xmlFreePropList((*cur).properties as xmlAttrPtr);
        }
        if !(*cur).nsDef.is_null() {
            xmlFreeNsList((*cur).nsDef as xmlNsPtr);
        }
    } else if !(*cur).content.is_null()
        && (*cur).type_0 as c_uint
            != XML_ENTITY_REF_NODE as c_int as c_uint
        && (*cur).content != &raw mut (*cur).properties as *mut xmlChar
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
    if !(*cur).name.is_null()
        && (*cur).type_0 as c_uint
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
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
} }
#[no_mangle]
pub extern "C" fn xmlUnlinkNode(mut cur: xmlNodePtr) { unsafe {
    if cur.is_null() {
        return;
    }
    if (*cur).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return;
    }
    if (*cur).type_0 as c_uint
        == XML_DTD_NODE as c_int as c_uint
    {
        let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
        doc = (*cur).doc as xmlDocPtr;
        if !doc.is_null() {
            if (*doc).intSubset == cur as xmlDtdPtr {
                (*doc).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
            }
            if (*doc).extSubset == cur as xmlDtdPtr {
                (*doc).extSubset = ::core::ptr::null_mut::<_xmlDtd>();
            }
        }
    }
    if (*cur).type_0 as c_uint
        == XML_ENTITY_DECL as c_int as c_uint
    {
        let mut doc_0: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
        doc_0 = (*cur).doc as xmlDocPtr;
        if !doc_0.is_null() {
            if !(*doc_0).intSubset.is_null() {
                if xmlHashLookup(
                    (*(*doc_0).intSubset).entities as xmlHashTablePtr,
                    (*cur).name,
                ) == cur as *mut c_void
                {
                    xmlHashRemoveEntry(
                        (*(*doc_0).intSubset).entities as xmlHashTablePtr,
                        (*cur).name,
                        None,
                    );
                }
                if xmlHashLookup(
                    (*(*doc_0).intSubset).pentities as xmlHashTablePtr,
                    (*cur).name,
                ) == cur as *mut c_void
                {
                    xmlHashRemoveEntry(
                        (*(*doc_0).intSubset).pentities as xmlHashTablePtr,
                        (*cur).name,
                        None,
                    );
                }
            }
            if !(*doc_0).extSubset.is_null() {
                if xmlHashLookup(
                    (*(*doc_0).extSubset).entities as xmlHashTablePtr,
                    (*cur).name,
                ) == cur as *mut c_void
                {
                    xmlHashRemoveEntry(
                        (*(*doc_0).extSubset).entities as xmlHashTablePtr,
                        (*cur).name,
                        None,
                    );
                }
                if xmlHashLookup(
                    (*(*doc_0).extSubset).pentities as xmlHashTablePtr,
                    (*cur).name,
                ) == cur as *mut c_void
                {
                    xmlHashRemoveEntry(
                        (*(*doc_0).extSubset).pentities as xmlHashTablePtr,
                        (*cur).name,
                        None,
                    );
                }
            }
        }
    }
    if !(*cur).parent.is_null() {
        let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        parent = (*cur).parent as xmlNodePtr;
        if (*cur).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
        {
            if (*parent).properties == cur as xmlAttrPtr {
                (*parent).properties = (*(cur as xmlAttrPtr)).next;
            }
        } else {
            if (*parent).children == cur {
                (*parent).children = (*cur).next;
            }
            if (*parent).last == cur {
                (*parent).last = (*cur).prev;
            }
        }
        (*cur).parent = ::core::ptr::null_mut::<_xmlNode>();
    }
    if !(*cur).next.is_null() {
        (*(*cur).next).prev = (*cur).prev;
    }
    if !(*cur).prev.is_null() {
        (*(*cur).prev).next = (*cur).next;
    }
    (*cur).prev = ::core::ptr::null_mut::<_xmlNode>();
    (*cur).next = (*cur).prev;
} }
#[inline]
pub fn xmlReplaceNode(mut old: xmlNodePtr, mut cur: xmlNodePtr) -> xmlNodePtr { unsafe {
    if old == cur {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if old.is_null()
        || (*old).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
        || (*old).parent.is_null()
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null()
        || (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        xmlUnlinkNode(old);
        return old;
    }
    if cur == old {
        return old;
    }
    if (*old).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
        && (*cur).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return old;
    }
    if (*cur).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
        && (*old).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return old;
    }
    xmlUnlinkNode(cur);
    xmlSetTreeDoc(cur, (*old).doc as xmlDocPtr);
    (*cur).parent = (*old).parent;
    (*cur).next = (*old).next;
    if !(*cur).next.is_null() {
        (*(*cur).next).prev = cur as *mut _xmlNode;
    }
    (*cur).prev = (*old).prev;
    if !(*cur).prev.is_null() {
        (*(*cur).prev).next = cur as *mut _xmlNode;
    }
    if !(*cur).parent.is_null() {
        if (*cur).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
        {
            if (*(*cur).parent).properties == old as xmlAttrPtr {
                (*(*cur).parent).properties = cur as xmlAttrPtr as *mut _xmlAttr;
            }
        } else {
            if (*(*cur).parent).children == old {
                (*(*cur).parent).children = cur as *mut _xmlNode;
            }
            if (*(*cur).parent).last == old {
                (*(*cur).parent).last = cur as *mut _xmlNode;
            }
        }
    }
    (*old).prev = ::core::ptr::null_mut::<_xmlNode>();
    (*old).next = (*old).prev;
    (*old).parent = ::core::ptr::null_mut::<_xmlNode>();
    return old;
} }
#[inline]
pub fn xmlCopyNamespace(mut cur: xmlNsPtr) -> xmlNsPtr { unsafe {
    let mut ret: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    match (*cur).type_0 as c_uint {
        18 => {
            ret = xmlNewNs(
                ::core::ptr::null_mut::<xmlNode>(),
                (*cur).href,
                (*cur).prefix,
            );
        }
        _ => return ::core::ptr::null_mut::<xmlNs>(),
    }
    return ret;
} }
#[inline]
pub fn xmlCopyNamespaceList(mut cur: xmlNsPtr) -> xmlNsPtr { unsafe {
    let mut ret: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut p: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut q: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    while !cur.is_null() {
        q = xmlCopyNamespace(cur);
        if q.is_null() {
            xmlFreeNsList(ret);
            return ::core::ptr::null_mut::<xmlNs>();
        }
        if p.is_null() {
            p = q;
            ret = p;
        } else {
            (*p).next = q as *mut _xmlNs;
            p = q;
        }
        cur = (*cur).next as xmlNsPtr;
    }
    return ret;
} }
fn xmlCopyPropInternal(
    mut doc: xmlDocPtr,
    mut target: xmlNodePtr,
    mut cur: xmlAttrPtr,
) -> xmlAttrPtr { unsafe {
    let mut ret: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    if !target.is_null()
        && (*target).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    if !target.is_null() {
        ret = xmlNewDocProp(
            (*target).doc as xmlDocPtr,
            (*cur).name,
            ::core::ptr::null::<xmlChar>(),
        );
    } else if !doc.is_null() {
        ret = xmlNewDocProp(doc, (*cur).name, ::core::ptr::null::<xmlChar>());
    } else if !(*cur).parent.is_null() {
        ret = xmlNewDocProp(
            (*(*cur).parent).doc as xmlDocPtr,
            (*cur).name,
            ::core::ptr::null::<xmlChar>(),
        );
    } else if !(*cur).children.is_null() {
        ret = xmlNewDocProp(
            (*(*cur).children).doc as xmlDocPtr,
            (*cur).name,
            ::core::ptr::null::<xmlChar>(),
        );
    } else {
        ret = xmlNewDocProp(
            ::core::ptr::null_mut::<xmlDoc>(),
            (*cur).name,
            ::core::ptr::null::<xmlChar>(),
        );
    }
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    (*ret).parent = target as *mut _xmlNode;
    if !(*cur).ns.is_null() && !target.is_null() {
        let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
        ns = xmlSearchNs((*target).doc as xmlDocPtr, target, (*(*cur).ns).prefix);
        if ns.is_null() {
            ns = xmlSearchNs(
                (*cur).doc as xmlDocPtr,
                (*cur).parent as xmlNodePtr,
                (*(*cur).ns).prefix,
            );
            if !ns.is_null() {
                let mut root: xmlNodePtr = target;
                let mut pred: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                while !(*root).parent.is_null() {
                    pred = root;
                    root = (*root).parent as xmlNodePtr;
                }
                if root == (*target).doc as xmlNodePtr {
                    root = pred;
                }
                (*ret).ns = xmlNewNs(root, (*ns).href, (*ns).prefix) as *mut xmlNs;
            }
        } else if xmlStrEqual((*ns).href, (*(*cur).ns).href) != 0 {
            (*ret).ns = ns as *mut xmlNs;
        } else {
            (*ret).ns =
                xmlNewReconciledNs((*target).doc as xmlDocPtr, target, (*cur).ns as xmlNsPtr)
                    as *mut xmlNs;
        }
    } else {
        (*ret).ns = ::core::ptr::null_mut::<xmlNs>();
    }
    if !(*cur).children.is_null() {
        let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        (*ret).children = xmlStaticCopyNodeList(
            (*cur).children as xmlNodePtr,
            (*ret).doc as xmlDocPtr,
            ret as xmlNodePtr,
        ) as *mut _xmlNode;
        (*ret).last = ::core::ptr::null_mut::<_xmlNode>();
        tmp = (*ret).children as xmlNodePtr;
        while !tmp.is_null() {
            if (*tmp).next.is_null() {
                (*ret).last = tmp as *mut _xmlNode;
            }
            tmp = (*tmp).next as xmlNodePtr;
        }
    }
    if !target.is_null()
        && !cur.is_null()
        && !(*target).doc.is_null()
        && !(*cur).doc.is_null()
        && !(*(*cur).doc).ids.is_null()
        && !(*cur).parent.is_null()
    {
        if xmlIsID((*cur).doc as xmlDocPtr, (*cur).parent as xmlNodePtr, cur) != 0 {
            let mut id: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            id = xmlNodeListGetString(
                (*cur).doc as xmlDocPtr,
                (*cur).children,
                1 as c_int,
            );
            if !id.is_null() {
                xmlAddID(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    (*target).doc as xmlDocPtr,
                    id,
                    ret,
                );
                xmlFree.expect("non-null function pointer")(id as *mut c_void);
            }
        }
    }
    return ret;
} }
#[inline]
pub fn xmlCopyProp(mut target: xmlNodePtr, mut cur: xmlAttrPtr) -> xmlAttrPtr { {
    return xmlCopyPropInternal(::core::ptr::null_mut::<xmlDoc>(), target, cur);
} }
#[inline]
pub fn xmlCopyPropList(
    mut target: xmlNodePtr,
    mut cur: xmlAttrPtr,
) -> xmlAttrPtr { unsafe {
    let mut ret: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut p: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut q: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if !target.is_null()
        && (*target).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    while !cur.is_null() {
        q = xmlCopyProp(target, cur);
        if q.is_null() {
            xmlFreePropList(ret);
            return ::core::ptr::null_mut::<xmlAttr>();
        }
        if p.is_null() {
            p = q;
            ret = p;
        } else {
            (*p).next = q as *mut _xmlAttr;
            (*q).prev = p as *mut _xmlAttr;
            p = q;
        }
        cur = (*cur).next as xmlAttrPtr;
    }
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlStaticCopyNode(
    mut node: xmlNodePtr,
    mut doc: xmlDocPtr,
    mut parent: xmlNodePtr,
    mut extended: c_int,
) -> xmlNodePtr { unsafe {
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if node.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    match (*node).type_0 as c_uint {
        2 => return xmlCopyPropInternal(doc, parent, node as xmlAttrPtr) as xmlNodePtr,
        18 => return xmlCopyNamespaceList(node as xmlNsPtr) as xmlNodePtr,
        9 | 13 => return xmlCopyDoc(node as xmlDocPtr, extended) as xmlNodePtr,
        10 | 12 | 14 | 15 | 16 | 17 => return ::core::ptr::null_mut::<xmlNode>(),
        3 | 4 | 1 | 11 | 5 | 6 | 7 | 8 | 19 | 20 | _ => {}
    }
    ret = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNode>() as size_t)
        as xmlNodePtr;
    if ret.is_null() {
        xmlTreeErrMemory(b"copying node\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNode>() as size_t,
    );
    (*ret).type_0 = (*node).type_0;
    (*ret).doc = doc as *mut _xmlDoc;
    (*ret).parent = parent as *mut _xmlNode;
    if (*node).name == &raw const xmlStringText as *const xmlChar {
        (*ret).name = &raw const xmlStringText as *const xmlChar;
    } else if (*node).name == &raw const xmlStringTextNoenc as *const xmlChar {
        (*ret).name = &raw const xmlStringTextNoenc as *const xmlChar;
    } else if (*node).name == &raw const xmlStringComment as *const xmlChar {
        (*ret).name = &raw const xmlStringComment as *const xmlChar;
    } else if !(*node).name.is_null() {
        if !doc.is_null() && !(*doc).dict.is_null() {
            (*ret).name = xmlDictLookup(
                (*doc).dict as xmlDictPtr,
                (*node).name,
                -(1 as c_int),
            );
        } else {
            (*ret).name = xmlStrdup((*node).name);
        }
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
        && !(*node).content.is_null()
        && (*node).type_0 as c_uint
            != XML_ENTITY_REF_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_XINCLUDE_END as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_XINCLUDE_START as c_int as c_uint
    {
        (*ret).content = xmlStrdup((*node).content);
    } else if (*node).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
    {
        (*ret).line = (*node).line;
    }
    if !parent.is_null() {
        let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        if __xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some() {
            (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret);
        }
        tmp = xmlAddChild(parent, ret);
        if tmp != ret {
            return tmp;
        }
    }
    if !(extended == 0) {
        if ((*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_XINCLUDE_START as c_int as c_uint)
            && !(*node).nsDef.is_null()
        {
            (*ret).nsDef = xmlCopyNamespaceList((*node).nsDef as xmlNsPtr) as *mut xmlNs;
        }
        if !(*node).ns.is_null() {
            let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
            ns = xmlSearchNs(doc, ret, (*(*node).ns).prefix);
            if ns.is_null() {
                ns = xmlSearchNs((*node).doc as xmlDocPtr, node, (*(*node).ns).prefix);
                if !ns.is_null() {
                    let mut root: xmlNodePtr = ret;
                    while !(*root).parent.is_null() {
                        root = (*root).parent as xmlNodePtr;
                    }
                    (*ret).ns = xmlNewNs(root, (*ns).href, (*ns).prefix) as *mut xmlNs;
                } else {
                    (*ret).ns = xmlNewReconciledNs(doc, ret, (*node).ns as xmlNsPtr) as *mut xmlNs;
                }
            } else {
                (*ret).ns = ns as *mut xmlNs;
            }
        }
        if ((*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_XINCLUDE_START as c_int as c_uint)
            && !(*node).properties.is_null()
        {
            (*ret).properties =
                xmlCopyPropList(ret, (*node).properties as xmlAttrPtr) as *mut _xmlAttr;
        }
        if (*node).type_0 as c_uint
            == XML_ENTITY_REF_NODE as c_int as c_uint
        {
            if doc.is_null() || (*node).doc != doc {
                (*ret).children = xmlGetDocEntity(doc as *const xmlDoc, (*ret).name) as xmlNodePtr
                    as *mut _xmlNode;
            } else {
                (*ret).children = (*node).children;
            }
            (*ret).last = (*ret).children;
        } else if !(*node).children.is_null() && extended != 2 as c_int {
            let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            let mut insert: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            cur = (*node).children as xmlNodePtr;
            insert = ret;
            while !cur.is_null() {
                let mut copy: xmlNodePtr =
                    xmlStaticCopyNode(cur, doc, insert, 2 as c_int);
                if copy.is_null() {
                    xmlFreeNode(ret);
                    return ::core::ptr::null_mut::<xmlNode>();
                }
                if (*insert).last != copy {
                    if (*insert).last.is_null() {
                        (*insert).children = copy as *mut _xmlNode;
                    } else {
                        (*copy).prev = (*insert).last;
                        (*(*insert).last).next = copy as *mut _xmlNode;
                    }
                    (*insert).last = copy as *mut _xmlNode;
                }
                if (*cur).type_0 as c_uint
                    != XML_ENTITY_REF_NODE as c_int as c_uint
                    && !(*cur).children.is_null()
                {
                    cur = (*cur).children as xmlNodePtr;
                    insert = copy;
                } else {
                    loop {
                        if !(*cur).next.is_null() {
                            cur = (*cur).next as xmlNodePtr;
                            break;
                        } else {
                            cur = (*cur).parent as xmlNodePtr;
                            insert = (*insert).parent as xmlNodePtr;
                            if !(cur == node) {
                                continue;
                            }
                            cur = ::core::ptr::null_mut::<xmlNode>();
                            break;
                        }
                    }
                }
            }
        }
    }
    if parent.is_null()
        && (__xmlRegisterCallbacks != 0 && (*__xmlRegisterNodeDefaultValue()).is_some())
    {
        (*__xmlRegisterNodeDefaultValue()).expect("non-null function pointer")(ret);
    }
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlStaticCopyNodeList(
    mut node: xmlNodePtr,
    mut doc: xmlDocPtr,
    mut parent: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    let mut current_block: u64;
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut p: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut q: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut newSubset: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut linkedSubset: c_int = 0 as c_int;
    loop {
        if node.is_null() {
            current_block = 10652014663920648156;
            break;
        }
        if (*node).type_0 as c_uint
            == XML_DTD_NODE as c_int as c_uint
        {
            if doc.is_null() {
                node = (*node).next as xmlNodePtr;
                continue;
            } else if (*doc).intSubset.is_null() && newSubset.is_null() {
                q = xmlCopyDtd(node as xmlDtdPtr) as xmlNodePtr;
                if q.is_null() {
                    current_block = 8076212527060520942;
                    break;
                }
                (*q).doc = doc as *mut _xmlDoc;
                (*q).parent = parent as *mut _xmlNode;
                newSubset = q as xmlDtdPtr;
                xmlAddChild(parent, q);
            } else {
                linkedSubset = 1 as c_int;
                q = (*doc).intSubset as xmlNodePtr;
                xmlAddChild(parent, q);
            }
        } else {
            q = xmlStaticCopyNode(node, doc, parent, 1 as c_int);
        }
        if q.is_null() {
            current_block = 8076212527060520942;
            break;
        }
        if ret.is_null() {
            (*q).prev = ::core::ptr::null_mut::<_xmlNode>();
            p = q;
            ret = p;
        } else if p != q {
            (*p).next = q as *mut _xmlNode;
            (*q).prev = p as *mut _xmlNode;
            p = q;
        }
        node = (*node).next as xmlNodePtr;
    }
    match current_block {
        8076212527060520942 => {
            if linkedSubset != 0 as c_int {
                xmlUnlinkNode((*doc).intSubset as xmlNodePtr);
            }
            xmlFreeNodeList(ret);
            return ::core::ptr::null_mut::<xmlNode>();
        }
        _ => {
            if !doc.is_null() && !newSubset.is_null() {
                (*doc).intSubset = newSubset as *mut _xmlDtd;
            }
            return ret;
        }
    };
} }
#[inline]
pub fn xmlCopyNode(
    mut node: xmlNodePtr,
    mut extended: c_int,
) -> xmlNodePtr { {
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    ret = xmlStaticCopyNode(
        node,
        ::core::ptr::null_mut::<xmlDoc>(),
        ::core::ptr::null_mut::<xmlNode>(),
        extended,
    );
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlDocCopyNode(
    mut node: xmlNodePtr,
    mut doc: xmlDocPtr,
    mut extended: c_int,
) -> xmlNodePtr { {
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    ret = xmlStaticCopyNode(node, doc, ::core::ptr::null_mut::<xmlNode>(), extended);
    return ret;
} }
#[inline]
pub fn xmlDocCopyNodeList(
    mut doc: xmlDocPtr,
    mut node: xmlNodePtr,
) -> xmlNodePtr { {
    let mut ret: xmlNodePtr = xmlStaticCopyNodeList(node, doc, ::core::ptr::null_mut::<xmlNode>());
    return ret;
} }
#[inline]
pub fn xmlCopyNodeList(mut node: xmlNodePtr) -> xmlNodePtr { {
    let mut ret: xmlNodePtr = xmlStaticCopyNodeList(
        node,
        ::core::ptr::null_mut::<xmlDoc>(),
        ::core::ptr::null_mut::<xmlNode>(),
    );
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlCopyDtd(mut dtd: xmlDtdPtr) -> xmlDtdPtr { unsafe {
    let mut ret: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut p: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut q: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if dtd.is_null() {
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    ret = xmlNewDtd(
        ::core::ptr::null_mut::<xmlDoc>(),
        (*dtd).name,
        (*dtd).ExternalID,
        (*dtd).SystemID,
    );
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlDtd>();
    }
    if !(*dtd).entities.is_null() {
        (*ret).entities = xmlCopyEntitiesTable((*dtd).entities as xmlEntitiesTablePtr)
            as *mut c_void;
    }
    if !(*dtd).notations.is_null() {
        (*ret).notations = xmlCopyNotationTable((*dtd).notations as xmlNotationTablePtr)
            as *mut c_void;
    }
    if !(*dtd).elements.is_null() {
        (*ret).elements =
            xmlCopyElementTable((*dtd).elements as xmlElementTablePtr) as *mut c_void;
    }
    if !(*dtd).attributes.is_null() {
        (*ret).attributes = xmlCopyAttributeTable((*dtd).attributes as xmlAttributeTablePtr)
            as *mut c_void;
    }
    if !(*dtd).pentities.is_null() {
        (*ret).pentities = xmlCopyEntitiesTable((*dtd).pentities as xmlEntitiesTablePtr)
            as *mut c_void;
    }
    cur = (*dtd).children as xmlNodePtr;
    while !cur.is_null() {
        q = ::core::ptr::null_mut::<xmlNode>();
        if (*cur).type_0 as c_uint
            == XML_ENTITY_DECL as c_int as c_uint
        {
            let mut tmp: xmlEntityPtr = cur as xmlEntityPtr;
            match (*tmp).etype as c_uint {
                1 | 2 | 3 => {
                    q = xmlGetEntityFromDtd(ret as *const xmlDtd, (*tmp).name) as xmlNodePtr;
                }
                4 | 5 => {
                    q = xmlGetParameterEntityFromDtd(ret as *const xmlDtd, (*tmp).name)
                        as xmlNodePtr;
                }
                6 | _ => {}
            }
        } else if (*cur).type_0 as c_uint
            == XML_ELEMENT_DECL as c_int as c_uint
        {
            let mut tmp_0: xmlElementPtr = cur as xmlElementPtr;
            q = xmlGetDtdQElementDesc(ret, (*tmp_0).name, (*tmp_0).prefix) as xmlNodePtr;
        } else if (*cur).type_0 as c_uint
            == XML_ATTRIBUTE_DECL as c_int as c_uint
        {
            let mut tmp_1: xmlAttributePtr = cur as xmlAttributePtr;
            q = xmlGetDtdQAttrDesc(ret, (*tmp_1).elem, (*tmp_1).name, (*tmp_1).prefix)
                as xmlNodePtr;
        } else if (*cur).type_0 as c_uint
            == XML_COMMENT_NODE as c_int as c_uint
        {
            q = xmlCopyNode(cur, 0 as c_int);
        }
        if q.is_null() {
            cur = (*cur).next as xmlNodePtr;
        } else {
            if p.is_null() {
                (*ret).children = q as *mut _xmlNode;
            } else {
                (*p).next = q as *mut _xmlNode;
            }
            (*q).prev = p as *mut _xmlNode;
            (*q).parent = ret as xmlNodePtr as *mut _xmlNode;
            (*q).next = ::core::ptr::null_mut::<_xmlNode>();
            (*ret).last = q as *mut _xmlNode;
            p = q;
            cur = (*cur).next as xmlNodePtr;
        }
    }
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlCopyDoc(
    mut doc: xmlDocPtr,
    mut recursive: c_int,
) -> xmlDocPtr { unsafe {
    let mut ret: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    ret = xmlNewDoc((*doc).version);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    (*ret).type_0 = (*doc).type_0;
    if !(*doc).name.is_null() {
        (*ret).name = xmlMemStrdup.expect("non-null function pointer")((*doc).name);
    }
    if !(*doc).encoding.is_null() {
        (*ret).encoding = xmlStrdup((*doc).encoding);
    }
    if !(*doc).URL.is_null() {
        (*ret).URL = xmlStrdup((*doc).URL);
    }
    (*ret).charset = (*doc).charset;
    (*ret).compression = (*doc).compression;
    (*ret).standalone = (*doc).standalone;
    if recursive == 0 {
        return ret;
    }
    (*ret).last = ::core::ptr::null_mut::<_xmlNode>();
    (*ret).children = ::core::ptr::null_mut::<_xmlNode>();
    if !(*doc).intSubset.is_null() {
        (*ret).intSubset = xmlCopyDtd((*doc).intSubset as xmlDtdPtr) as *mut _xmlDtd;
        if (*ret).intSubset.is_null() {
            xmlFreeDoc(ret);
            return ::core::ptr::null_mut::<xmlDoc>();
        }
        xmlSetTreeDoc((*ret).intSubset as xmlNodePtr, ret);
        (*(*ret).intSubset).parent = ret as *mut _xmlDoc;
    }
    if !(*doc).oldNs.is_null() {
        (*ret).oldNs = xmlCopyNamespaceList((*doc).oldNs as xmlNsPtr) as *mut _xmlNs;
    }
    if !(*doc).children.is_null() {
        let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        (*ret).children =
            xmlStaticCopyNodeList((*doc).children as xmlNodePtr, ret, ret as xmlNodePtr)
                as *mut _xmlNode;
        (*ret).last = ::core::ptr::null_mut::<_xmlNode>();
        tmp = (*ret).children as xmlNodePtr;
        while !tmp.is_null() {
            if (*tmp).next.is_null() {
                (*ret).last = tmp as *mut _xmlNode;
            }
            tmp = (*tmp).next as xmlNodePtr;
        }
    }
    return ret;
} }
unsafe fn xmlGetLineNoInternal(
    mut node: *const xmlNode,
    mut depth: c_int,
) -> c_long {
    let mut result: c_long = -(1 as c_int) as c_long;
    if depth >= 5 as c_int {
        return -(1 as c_int) as c_long;
    }
    if node.is_null() {
        return result;
    }
    if (*node).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        || (*node).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
        || (*node).type_0 as c_uint
            == XML_COMMENT_NODE as c_int as c_uint
        || (*node).type_0 as c_uint
            == XML_PI_NODE as c_int as c_uint
    {
        if (*node).line as c_int == 65535 as c_int {
            if (*node).type_0 as c_uint
                == XML_TEXT_NODE as c_int as c_uint
                && !(*node).psvi.is_null()
            {
                result = (*node).psvi as ptrdiff_t as c_long;
            } else if (*node).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
                && !(*node).children.is_null()
            {
                result = xmlGetLineNoInternal((*node).children, depth + 1 as c_int);
            } else if !(*node).next.is_null() {
                result = xmlGetLineNoInternal((*node).next, depth + 1 as c_int);
            } else if !(*node).prev.is_null() {
                result = xmlGetLineNoInternal((*node).prev, depth + 1 as c_int);
            }
        }
        if result == -(1 as c_int) as c_long
            || result == 65535 as c_long
        {
            result = (*node).line as c_long;
        }
    } else if !(*node).prev.is_null()
        && ((*(*node).prev).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
            || (*(*node).prev).type_0 as c_uint
                == XML_TEXT_NODE as c_int as c_uint
            || (*(*node).prev).type_0 as c_uint
                == XML_COMMENT_NODE as c_int as c_uint
            || (*(*node).prev).type_0 as c_uint
                == XML_PI_NODE as c_int as c_uint)
    {
        result = xmlGetLineNoInternal((*node).prev, depth + 1 as c_int);
    } else if !(*node).parent.is_null()
        && (*(*node).parent).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
    {
        result = xmlGetLineNoInternal((*node).parent, depth + 1 as c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetLineNo(mut node: *const xmlNode) -> c_long {
    return xmlGetLineNoInternal(node, 0 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetNodePath(mut node: *const xmlNode) -> *mut xmlChar {
    let mut cur: *const xmlNode = ::core::ptr::null::<xmlNode>();
    let mut tmp: *const xmlNode = ::core::ptr::null::<xmlNode>();
    let mut next: *const xmlNode = ::core::ptr::null::<xmlNode>();
    let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut temp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut buf_len: size_t = 0;
    let mut buf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut sep: *const c_char = ::core::ptr::null::<c_char>();
    let mut name: *const c_char = ::core::ptr::null::<c_char>();
    let mut nametemp: [c_char; 100] = [0; 100];
    let mut occur: c_int = 0 as c_int;
    let mut generic: c_int = 0;
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buf_len = 500 as size_t;
    buffer = xmlMallocAtomic.expect("non-null function pointer")(buf_len) as *mut xmlChar;
    if buffer.is_null() {
        xmlTreeErrMemory(b"getting node path\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buf = xmlMallocAtomic.expect("non-null function pointer")(buf_len) as *mut xmlChar;
    if buf.is_null() {
        xmlTreeErrMemory(b"getting node path\0" as *const u8 as *const c_char);
        xmlFree.expect("non-null function pointer")(buffer as *mut c_void);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    *buffer.offset(0 as c_int as isize) = 0 as xmlChar;
    cur = node;
    loop {
        name = b"\0" as *const u8 as *const c_char;
        sep = b"?\0" as *const u8 as *const c_char;
        occur = 0 as c_int;
        if (*cur).type_0 as c_uint
            == XML_DOCUMENT_NODE as c_int as c_uint
            || (*cur).type_0 as c_uint
                == XML_HTML_DOCUMENT_NODE as c_int as c_uint
        {
            if *buffer.offset(0 as c_int as isize) as c_int == '/' as i32
            {
                break;
            }
            sep = b"/\0" as *const u8 as *const c_char;
            next = ::core::ptr::null::<xmlNode>();
        } else if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            generic = 0 as c_int;
            sep = b"/\0" as *const u8 as *const c_char;
            name = (*cur).name as *const c_char;
            if !(*cur).ns.is_null() {
                if !(*(*cur).ns).prefix.is_null() {
                    snprintf(
                        &raw mut nametemp as *mut c_char,
                        (::core::mem::size_of::<[c_char; 100]>() as size_t)
                            .wrapping_sub(1 as size_t),
                        b"%s:%s\0" as *const u8 as *const c_char,
                        (*(*cur).ns).prefix as *mut c_char,
                        (*cur).name as *mut c_char,
                    );
                    nametemp[(::core::mem::size_of::<[c_char; 100]>() as usize)
                        .wrapping_sub(1 as usize) as usize] = 0 as c_char;
                    name = &raw mut nametemp as *mut c_char;
                } else {
                    generic = 1 as c_int;
                    name = b"*\0" as *const u8 as *const c_char;
                }
            }
            next = (*cur).parent;
            tmp = (*cur).prev;
            while !tmp.is_null() {
                if (*tmp).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    && (generic != 0
                        || xmlStrEqual((*cur).name, (*tmp).name) != 0
                            && ((*tmp).ns == (*cur).ns
                                || !(*tmp).ns.is_null()
                                    && !(*cur).ns.is_null()
                                    && xmlStrEqual((*(*cur).ns).prefix, (*(*tmp).ns).prefix) != 0))
                {
                    occur += 1;
                }
                tmp = (*tmp).prev;
            }
            if occur == 0 as c_int {
                tmp = (*cur).next;
                while !tmp.is_null() && occur == 0 as c_int {
                    if (*tmp).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                        && (generic != 0
                            || xmlStrEqual((*cur).name, (*tmp).name) != 0
                                && ((*tmp).ns == (*cur).ns
                                    || !(*tmp).ns.is_null()
                                        && !(*cur).ns.is_null()
                                        && xmlStrEqual((*(*cur).ns).prefix, (*(*tmp).ns).prefix)
                                            != 0))
                    {
                        occur += 1;
                    }
                    tmp = (*tmp).next;
                }
                if occur != 0 as c_int {
                    occur = 1 as c_int;
                }
            } else {
                occur += 1;
            }
        } else if (*cur).type_0 as c_uint
            == XML_COMMENT_NODE as c_int as c_uint
        {
            sep = b"/\0" as *const u8 as *const c_char;
            name = b"comment()\0" as *const u8 as *const c_char;
            next = (*cur).parent;
            tmp = (*cur).prev;
            while !tmp.is_null() {
                if (*tmp).type_0 as c_uint
                    == XML_COMMENT_NODE as c_int as c_uint
                {
                    occur += 1;
                }
                tmp = (*tmp).prev;
            }
            if occur == 0 as c_int {
                tmp = (*cur).next;
                while !tmp.is_null() && occur == 0 as c_int {
                    if (*tmp).type_0 as c_uint
                        == XML_COMMENT_NODE as c_int as c_uint
                    {
                        occur += 1;
                    }
                    tmp = (*tmp).next;
                }
                if occur != 0 as c_int {
                    occur = 1 as c_int;
                }
            } else {
                occur += 1;
            }
        } else if (*cur).type_0 as c_uint
            == XML_TEXT_NODE as c_int as c_uint
            || (*cur).type_0 as c_uint
                == XML_CDATA_SECTION_NODE as c_int as c_uint
        {
            sep = b"/\0" as *const u8 as *const c_char;
            name = b"text()\0" as *const u8 as *const c_char;
            next = (*cur).parent;
            tmp = (*cur).prev;
            while !tmp.is_null() {
                if (*tmp).type_0 as c_uint
                    == XML_TEXT_NODE as c_int as c_uint
                    || (*tmp).type_0 as c_uint
                        == XML_CDATA_SECTION_NODE as c_int as c_uint
                {
                    occur += 1;
                }
                tmp = (*tmp).prev;
            }
            if occur == 0 as c_int {
                tmp = (*cur).next;
                while !tmp.is_null() {
                    if (*tmp).type_0 as c_uint
                        == XML_TEXT_NODE as c_int as c_uint
                        || (*tmp).type_0 as c_uint
                            == XML_CDATA_SECTION_NODE as c_int as c_uint
                    {
                        occur = 1 as c_int;
                        break;
                    } else {
                        tmp = (*tmp).next;
                    }
                }
            } else {
                occur += 1;
            }
        } else if (*cur).type_0 as c_uint
            == XML_PI_NODE as c_int as c_uint
        {
            sep = b"/\0" as *const u8 as *const c_char;
            snprintf(
                &raw mut nametemp as *mut c_char,
                (::core::mem::size_of::<[c_char; 100]>() as size_t)
                    .wrapping_sub(1 as size_t),
                b"processing-instruction('%s')\0" as *const u8 as *const c_char,
                (*cur).name as *mut c_char,
            );
            nametemp[(::core::mem::size_of::<[c_char; 100]>() as usize)
                .wrapping_sub(1 as usize) as usize] = 0 as c_char;
            name = &raw mut nametemp as *mut c_char;
            next = (*cur).parent;
            tmp = (*cur).prev;
            while !tmp.is_null() {
                if (*tmp).type_0 as c_uint
                    == XML_PI_NODE as c_int as c_uint
                    && xmlStrEqual((*cur).name, (*tmp).name) != 0
                {
                    occur += 1;
                }
                tmp = (*tmp).prev;
            }
            if occur == 0 as c_int {
                tmp = (*cur).next;
                while !tmp.is_null() && occur == 0 as c_int {
                    if (*tmp).type_0 as c_uint
                        == XML_PI_NODE as c_int as c_uint
                        && xmlStrEqual((*cur).name, (*tmp).name) != 0
                    {
                        occur += 1;
                    }
                    tmp = (*tmp).next;
                }
                if occur != 0 as c_int {
                    occur = 1 as c_int;
                }
            } else {
                occur += 1;
            }
        } else if (*cur).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
        {
            sep = b"/@\0" as *const u8 as *const c_char;
            name = (*(cur as xmlAttrPtr)).name as *const c_char;
            if !(*cur).ns.is_null() {
                if !(*(*cur).ns).prefix.is_null() {
                    snprintf(
                        &raw mut nametemp as *mut c_char,
                        (::core::mem::size_of::<[c_char; 100]>() as size_t)
                            .wrapping_sub(1 as size_t),
                        b"%s:%s\0" as *const u8 as *const c_char,
                        (*(*cur).ns).prefix as *mut c_char,
                        (*cur).name as *mut c_char,
                    );
                } else {
                    snprintf(
                        &raw mut nametemp as *mut c_char,
                        (::core::mem::size_of::<[c_char; 100]>() as size_t)
                            .wrapping_sub(1 as size_t),
                        b"%s\0" as *const u8 as *const c_char,
                        (*cur).name as *mut c_char,
                    );
                }
                nametemp[(::core::mem::size_of::<[c_char; 100]>() as usize)
                    .wrapping_sub(1 as usize) as usize] = 0 as c_char;
                name = &raw mut nametemp as *mut c_char;
            }
            next = (*(cur as xmlAttrPtr)).parent;
        } else {
            xmlFree.expect("non-null function pointer")(buf as *mut c_void);
            xmlFree.expect("non-null function pointer")(buffer as *mut c_void);
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if (xmlStrlen(buffer) as usize)
            .wrapping_add(::core::mem::size_of::<[c_char; 100]>() as usize)
            .wrapping_add(20 as usize)
            > buf_len
        {
            buf_len = (2 as size_t)
                .wrapping_mul(buf_len)
                .wrapping_add(xmlStrlen(buffer) as size_t)
                .wrapping_add(::core::mem::size_of::<[c_char; 100]>() as size_t)
                .wrapping_add(20 as size_t);
            temp = xmlRealloc.expect("non-null function pointer")(
                buffer as *mut c_void,
                buf_len,
            ) as *mut xmlChar;
            if temp.is_null() {
                xmlTreeErrMemory(b"getting node path\0" as *const u8 as *const c_char);
                xmlFree.expect("non-null function pointer")(buf as *mut c_void);
                xmlFree.expect("non-null function pointer")(buffer as *mut c_void);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            buffer = temp;
            temp = xmlRealloc.expect("non-null function pointer")(
                buf as *mut c_void,
                buf_len,
            ) as *mut xmlChar;
            if temp.is_null() {
                xmlTreeErrMemory(b"getting node path\0" as *const u8 as *const c_char);
                xmlFree.expect("non-null function pointer")(buf as *mut c_void);
                xmlFree.expect("non-null function pointer")(buffer as *mut c_void);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            buf = temp;
        }
        if occur == 0 as c_int {
            snprintf(
                buf as *mut c_char,
                buf_len,
                b"%s%s%s\0" as *const u8 as *const c_char,
                sep,
                name,
                buffer as *mut c_char,
            );
        } else {
            snprintf(
                buf as *mut c_char,
                buf_len,
                b"%s%s[%d]%s\0" as *const u8 as *const c_char,
                sep,
                name,
                occur,
                buffer as *mut c_char,
            );
        }
        snprintf(
            buffer as *mut c_char,
            buf_len,
            b"%s\0" as *const u8 as *const c_char,
            buf as *mut c_char,
        );
        cur = next;
        if cur.is_null() {
            break;
        }
    }
    xmlFree.expect("non-null function pointer")(buf as *mut c_void);
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDocGetRootElement(mut doc: *const xmlDoc) -> xmlNodePtr {
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    ret = (*doc).children as xmlNodePtr;
    while !ret.is_null() {
        if (*ret).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            return ret;
        }
        ret = (*ret).next as xmlNodePtr;
    }
    return ret;
}
#[inline]
pub fn xmlDocSetRootElement(
    mut doc: xmlDocPtr,
    mut root: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    let mut old: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if root.is_null()
        || (*root).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    xmlUnlinkNode(root);
    xmlSetTreeDoc(root, doc);
    (*root).parent = doc as xmlNodePtr as *mut _xmlNode;
    old = (*doc).children as xmlNodePtr;
    while !old.is_null() {
        if (*old).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            break;
        }
        old = (*old).next as xmlNodePtr;
    }
    if old.is_null() {
        if (*doc).children.is_null() {
            (*doc).children = root as *mut _xmlNode;
            (*doc).last = root as *mut _xmlNode;
        } else {
            xmlAddSibling((*doc).children as xmlNodePtr, root);
        }
    } else {
        xmlReplaceNode(old, root);
    }
    return old;
} }
#[inline]
pub unsafe fn xmlNodeSetLang(mut cur: xmlNodePtr, mut lang: *const xmlChar) {
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if cur.is_null() {
        return;
    }
    match (*cur).type_0 as c_uint {
        3 | 4 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 7 | 5 | 6 | 18 | 19 | 20 => return,
        1 | 2 | _ => {}
    }
    ns = xmlSearchNsByHref((*cur).doc as xmlDocPtr, cur, XML_XML_NAMESPACE);
    if ns.is_null() {
        return;
    }
    xmlSetNsProp(
        cur,
        ns,
        b"lang\0" as *const u8 as *const c_char as *mut xmlChar,
        lang,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlNodeGetLang(mut cur: *const xmlNode) -> *mut xmlChar {
    let mut lang: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if cur.is_null()
        || (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while !cur.is_null() {
        lang = xmlGetNsProp(
            cur,
            b"lang\0" as *const u8 as *const c_char as *mut xmlChar,
            XML_XML_NAMESPACE,
        );
        if !lang.is_null() {
            return lang;
        }
        cur = (*cur).parent;
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[inline]
pub fn xmlNodeSetSpacePreserve(mut cur: xmlNodePtr, mut val: c_int) { unsafe {
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if cur.is_null() {
        return;
    }
    match (*cur).type_0 as c_uint {
        3 | 4 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 7 | 5 | 6 | 18 | 19 | 20 => return,
        1 | 2 | _ => {}
    }
    ns = xmlSearchNsByHref((*cur).doc as xmlDocPtr, cur, XML_XML_NAMESPACE);
    if ns.is_null() {
        return;
    }
    match val {
        0 => {
            xmlSetNsProp(
                cur,
                ns,
                b"space\0" as *const u8 as *const c_char as *mut xmlChar,
                b"default\0" as *const u8 as *const c_char as *mut xmlChar,
            );
        }
        1 => {
            xmlSetNsProp(
                cur,
                ns,
                b"space\0" as *const u8 as *const c_char as *mut xmlChar,
                b"preserve\0" as *const u8 as *const c_char as *mut xmlChar,
            );
        }
        _ => {}
    };
} }
#[no_mangle]
pub unsafe extern "C" fn xmlNodeGetSpacePreserve(mut cur: *const xmlNode) -> c_int {
    let mut space: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if cur.is_null()
        || (*cur).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return -(1 as c_int);
    }
    while !cur.is_null() {
        space = xmlGetNsProp(
            cur,
            b"space\0" as *const u8 as *const c_char as *mut xmlChar,
            XML_XML_NAMESPACE,
        );
        if !space.is_null() {
            if xmlStrEqual(
                space,
                b"preserve\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                xmlFree.expect("non-null function pointer")(space as *mut c_void);
                return 1 as c_int;
            }
            if xmlStrEqual(
                space,
                b"default\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                xmlFree.expect("non-null function pointer")(space as *mut c_void);
                return 0 as c_int;
            }
            xmlFree.expect("non-null function pointer")(space as *mut c_void);
        }
        cur = (*cur).parent;
    }
    return -(1 as c_int);
}
#[inline]
pub unsafe fn xmlNodeSetName(mut cur: xmlNodePtr, mut name: *const xmlChar) {
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    let mut freeme: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if cur.is_null() {
        return;
    }
    if name.is_null() {
        return;
    }
    match (*cur).type_0 as c_uint {
        3 | 4 | 8 | 10 | 11 | 12 | 13 | 18 | 19 | 20 => return,
        1 | 2 | 7 | 5 | 6 | 14 | 9 | 15 | 16 | 17 | _ => {}
    }
    doc = (*cur).doc as xmlDocPtr;
    if !doc.is_null() {
        dict = (*doc).dict as xmlDictPtr;
    } else {
        dict = ::core::ptr::null_mut::<xmlDict>();
    }
    if !dict.is_null() {
        if !(*cur).name.is_null() && xmlDictOwns(dict, (*cur).name) == 0 {
            freeme = (*cur).name;
        }
        (*cur).name = xmlDictLookup(dict, name, -(1 as c_int));
    } else {
        if !(*cur).name.is_null() {
            freeme = (*cur).name;
        }
        (*cur).name = xmlStrdup(name);
    }
    if !freeme.is_null() {
        xmlFree.expect("non-null function pointer")(
            freeme as *mut xmlChar as *mut c_void,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlNodeSetBase(mut cur: xmlNodePtr, mut uri: *const xmlChar) {
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut fixed: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if cur.is_null() {
        return;
    }
    match (*cur).type_0 as c_uint {
        3 | 4 | 8 | 10 | 11 | 12 | 14 | 15 | 16 | 17 | 7 | 5 | 6 | 18 | 19 | 20 => return,
        9 | 13 => {
            let mut doc: xmlDocPtr = cur as xmlDocPtr;
            if !(*doc).URL.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*doc).URL as *mut xmlChar as *mut c_void,
                );
            }
            if uri.is_null() {
                (*doc).URL = ::core::ptr::null::<xmlChar>();
            } else {
                (*doc).URL = xmlPathToURI(uri);
            }
            return;
        }
        1 | 2 | _ => {}
    }
    ns = xmlSearchNsByHref((*cur).doc as xmlDocPtr, cur, XML_XML_NAMESPACE);
    if ns.is_null() {
        return;
    }
    fixed = xmlPathToURI(uri);
    if !fixed.is_null() {
        xmlSetNsProp(
            cur,
            ns,
            b"base\0" as *const u8 as *const c_char as *mut xmlChar,
            fixed,
        );
        xmlFree.expect("non-null function pointer")(fixed as *mut c_void);
    } else {
        xmlSetNsProp(
            cur,
            ns,
            b"base\0" as *const u8 as *const c_char as *mut xmlChar,
            uri,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlNodeGetBase(
    mut doc: *const xmlDoc,
    mut cur: *const xmlNode,
) -> *mut xmlChar {
    let mut oldbase: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut base: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut newbase: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if cur.is_null() && doc.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !cur.is_null()
        && (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if doc.is_null() {
        doc = (*cur).doc;
    }
    if !doc.is_null()
        && (*doc).type_0 as c_uint
            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        cur = (*doc).children;
        while !cur.is_null() && !(*cur).name.is_null() {
            if (*cur).type_0 as c_uint
                != XML_ELEMENT_NODE as c_int as c_uint
            {
                cur = (*cur).next;
            } else if xmlStrcasecmp(
                (*cur).name,
                b"html\0" as *const u8 as *const c_char as *mut xmlChar,
            ) == 0
            {
                cur = (*cur).children;
            } else if xmlStrcasecmp(
                (*cur).name,
                b"head\0" as *const u8 as *const c_char as *mut xmlChar,
            ) == 0
            {
                cur = (*cur).children;
            } else {
                if xmlStrcasecmp(
                    (*cur).name,
                    b"base\0" as *const u8 as *const c_char as *mut xmlChar,
                ) == 0
                {
                    return xmlGetProp(
                        cur,
                        b"href\0" as *const u8 as *const c_char as *mut xmlChar,
                    );
                }
                cur = (*cur).next;
            }
        }
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while !cur.is_null() {
        if (*cur).type_0 as c_uint
            == XML_ENTITY_DECL as c_int as c_uint
        {
            let mut ent: xmlEntityPtr = cur as xmlEntityPtr;
            return xmlStrdup((*ent).URI);
        }
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            base = xmlGetNsProp(
                cur,
                b"base\0" as *const u8 as *const c_char as *mut xmlChar,
                XML_XML_NAMESPACE,
            );
            if !base.is_null() {
                if !oldbase.is_null() {
                    newbase = xmlBuildURI(oldbase, base);
                    if !newbase.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            oldbase as *mut c_void,
                        );
                        xmlFree.expect("non-null function pointer")(
                            base as *mut c_void,
                        );
                        oldbase = newbase;
                    } else {
                        xmlFree.expect("non-null function pointer")(
                            oldbase as *mut c_void,
                        );
                        xmlFree.expect("non-null function pointer")(
                            base as *mut c_void,
                        );
                        return ::core::ptr::null_mut::<xmlChar>();
                    }
                } else {
                    oldbase = base;
                }
                if xmlStrncmp(
                    oldbase,
                    b"http://\0" as *const u8 as *const c_char as *mut xmlChar,
                    7 as c_int,
                ) == 0
                    || xmlStrncmp(
                        oldbase,
                        b"ftp://\0" as *const u8 as *const c_char as *mut xmlChar,
                        6 as c_int,
                    ) == 0
                    || xmlStrncmp(
                        oldbase,
                        b"urn:\0" as *const u8 as *const c_char as *mut xmlChar,
                        4 as c_int,
                    ) == 0
                {
                    return oldbase;
                }
            }
        }
        cur = (*cur).parent;
    }
    if !doc.is_null() && !(*doc).URL.is_null() {
        if oldbase.is_null() {
            return xmlStrdup((*doc).URL);
        }
        newbase = xmlBuildURI(oldbase, (*doc).URL);
        xmlFree.expect("non-null function pointer")(oldbase as *mut c_void);
        return newbase;
    }
    return oldbase;
}
#[inline]
pub unsafe fn xmlNodeBufGetContent(
    mut buffer: xmlBufferPtr,
    mut cur: *const xmlNode,
) -> c_int {
    let mut buf: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut ret: c_int = 0;
    if cur.is_null() || buffer.is_null() {
        return -(1 as c_int);
    }
    buf = xmlBufFromBuffer(buffer);
    ret = xmlBufGetNodeContent(buf, cur);
    buffer = xmlBufBackToBuffer(buf);
    if ret < 0 as c_int || buffer.is_null() {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlBufGetNodeContent(
    mut buf: xmlBufPtr,
    mut cur: *const xmlNode,
) -> c_int {
    if cur.is_null() || buf.is_null() {
        return -(1 as c_int);
    }
    match (*cur).type_0 as c_uint {
        4 | 3 => {
            xmlBufCat(buf, (*cur).content);
        }
        11 | 1 => {
            let mut tmp: *const xmlNode = cur;
            while !tmp.is_null() {
                match (*tmp).type_0 as c_uint {
                    4 | 3 => {
                        if !(*tmp).content.is_null() {
                            xmlBufCat(buf, (*tmp).content);
                        }
                    }
                    5 => {
                        xmlBufGetNodeContent(buf, tmp);
                    }
                    _ => {}
                }
                if !(*tmp).children.is_null() {
                    if (*(*tmp).children).type_0 as c_uint
                        != XML_ENTITY_DECL as c_int as c_uint
                    {
                        tmp = (*tmp).children;
                        continue;
                    }
                }
                if tmp == cur {
                    break;
                }
                if !(*tmp).next.is_null() {
                    tmp = (*tmp).next;
                } else {
                    loop {
                        tmp = (*tmp).parent;
                        if tmp.is_null() {
                            break;
                        }
                        if tmp == cur {
                            tmp = ::core::ptr::null::<xmlNode>();
                            break;
                        } else if !(*tmp).next.is_null() {
                            tmp = (*tmp).next;
                            break;
                        } else if tmp.is_null() {
                            break;
                        }
                    }
                }
            }
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            let mut tmp_0: xmlNodePtr = (*attr).children as xmlNodePtr;
            while !tmp_0.is_null() {
                if (*tmp_0).type_0 as c_uint
                    == XML_TEXT_NODE as c_int as c_uint
                {
                    xmlBufCat(buf, (*tmp_0).content);
                } else {
                    xmlBufGetNodeContent(buf, tmp_0 as *const xmlNode);
                }
                tmp_0 = (*tmp_0).next as xmlNodePtr;
            }
        }
        8 | 7 => {
            xmlBufCat(buf, (*cur).content);
        }
        5 => {
            let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
            let mut tmp_1: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            ent = xmlGetDocEntity((*cur).doc, (*cur).name);
            if ent.is_null() {
                return -(1 as c_int);
            }
            tmp_1 = (*ent).children as xmlNodePtr;
            while !tmp_1.is_null() {
                xmlBufGetNodeContent(buf, tmp_1 as *const xmlNode);
                tmp_1 = (*tmp_1).next as xmlNodePtr;
            }
        }
        9 | 13 => {
            cur = (*cur).children;
            while !cur.is_null() {
                if (*cur).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    || (*cur).type_0 as c_uint
                        == XML_TEXT_NODE as c_int as c_uint
                    || (*cur).type_0 as c_uint
                        == XML_CDATA_SECTION_NODE as c_int as c_uint
                {
                    xmlBufGetNodeContent(buf, cur);
                }
                cur = (*cur).next;
            }
        }
        18 => {
            xmlBufCat(buf, (*(cur as xmlNsPtr)).href);
        }
        6 | 10 | 12 | 14 | 19 | 20 | 15 | 16 | 17 | _ => {}
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNodeGetContent(mut cur: *const xmlNode) -> *mut xmlChar {
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    match (*cur).type_0 as c_uint {
        11 | 1 => {
            let mut buf: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
            let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            buf = xmlBufCreateSize(64 as size_t);
            if buf.is_null() {
                return ::core::ptr::null_mut::<xmlChar>();
            }
            xmlBufSetAllocationScheme(buf, XML_BUFFER_ALLOC_DOUBLEIT);
            xmlBufGetNodeContent(buf, cur);
            ret = xmlBufDetach(buf);
            xmlBufFree(buf);
            return ret;
        }
        2 => return xmlGetPropNodeValueInternal(cur as xmlAttrPtr as *const xmlAttr),
        8 | 7 => {
            if !(*cur).content.is_null() {
                return xmlStrdup((*cur).content);
            }
            return ::core::ptr::null_mut::<xmlChar>();
        }
        5 => {
            let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
            let mut buf_0: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
            let mut ret_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            ent = xmlGetDocEntity((*cur).doc, (*cur).name);
            if ent.is_null() {
                return ::core::ptr::null_mut::<xmlChar>();
            }
            buf_0 = xmlBufCreate();
            if buf_0.is_null() {
                return ::core::ptr::null_mut::<xmlChar>();
            }
            xmlBufSetAllocationScheme(buf_0, XML_BUFFER_ALLOC_DOUBLEIT);
            xmlBufGetNodeContent(buf_0, cur);
            ret_0 = xmlBufDetach(buf_0);
            xmlBufFree(buf_0);
            return ret_0;
        }
        6 | 10 | 12 | 14 | 19 | 20 => return ::core::ptr::null_mut::<xmlChar>(),
        9 | 13 => {
            let mut buf_1: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
            let mut ret_1: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            buf_1 = xmlBufCreate();
            if buf_1.is_null() {
                return ::core::ptr::null_mut::<xmlChar>();
            }
            xmlBufSetAllocationScheme(buf_1, XML_BUFFER_ALLOC_DOUBLEIT);
            xmlBufGetNodeContent(buf_1, cur as xmlNodePtr as *const xmlNode);
            ret_1 = xmlBufDetach(buf_1);
            xmlBufFree(buf_1);
            return ret_1;
        }
        18 => {
            let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            tmp = xmlStrdup((*(cur as xmlNsPtr)).href);
            return tmp;
        }
        15 => return ::core::ptr::null_mut::<xmlChar>(),
        16 => return ::core::ptr::null_mut::<xmlChar>(),
        17 => return ::core::ptr::null_mut::<xmlChar>(),
        4 | 3 => {
            if !(*cur).content.is_null() {
                return xmlStrdup((*cur).content);
            }
            return ::core::ptr::null_mut::<xmlChar>();
        }
        _ => {}
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlNodeSetContent(mut cur: xmlNodePtr, mut content: *const xmlChar) {
    if cur.is_null() {
        return;
    }
    match (*cur).type_0 as c_uint {
        11 | 1 | 2 => {
            if !(*cur).children.is_null() {
                xmlFreeNodeList((*cur).children as xmlNodePtr);
            }
            (*cur).children = xmlStringGetNodeList((*cur).doc, content) as *mut _xmlNode;
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children as xmlNodePtr;
                if ulccur.is_null() {
                    (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur as *mut _xmlNode;
                        ulccur = (*ulccur).next as xmlNodePtr;
                    }
                    (*ulccur).parent = cur as *mut _xmlNode;
                    (*cur).last = ulccur as *mut _xmlNode;
                }
            }
        }
        3 | 4 | 5 | 6 | 7 | 8 => {
            if !(*cur).content.is_null()
                && (*cur).content != &raw mut (*cur).properties as *mut xmlChar
            {
                if !(!(*cur).doc.is_null()
                    && !(*(*cur).doc).dict.is_null()
                    && xmlDictOwns((*(*cur).doc).dict as xmlDictPtr, (*cur).content) != 0)
                {
                    xmlFree.expect("non-null function pointer")(
                        (*cur).content as *mut c_void,
                    );
                }
            }
            if !(*cur).children.is_null() {
                xmlFreeNodeList((*cur).children as xmlNodePtr);
            }
            (*cur).children = ::core::ptr::null_mut::<_xmlNode>();
            (*cur).last = (*cur).children;
            if !content.is_null() {
                (*cur).content = xmlStrdup(content);
            } else {
                (*cur).content = ::core::ptr::null_mut::<xmlChar>();
            }
            (*cur).properties = ::core::ptr::null_mut::<_xmlAttr>();
        }
        9 | 13 | 10 | 19 | 20 | 12 | 14 | 18 | 15 | 16 | 17 | _ => {}
    };
}
#[inline]
pub unsafe fn xmlNodeSetContentLen(
    mut cur: xmlNodePtr,
    mut content: *const xmlChar,
    mut len: c_int,
) {
    if cur.is_null() {
        return;
    }
    match (*cur).type_0 as c_uint {
        11 | 1 | 2 => {
            if !(*cur).children.is_null() {
                xmlFreeNodeList((*cur).children as xmlNodePtr);
            }
            (*cur).children = xmlStringLenGetNodeList((*cur).doc, content, len) as *mut _xmlNode;
            if !cur.is_null() {
                let mut ulccur: xmlNodePtr = (*cur).children as xmlNodePtr;
                if ulccur.is_null() {
                    (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
                } else {
                    while !(*ulccur).next.is_null() {
                        (*ulccur).parent = cur as *mut _xmlNode;
                        ulccur = (*ulccur).next as xmlNodePtr;
                    }
                    (*ulccur).parent = cur as *mut _xmlNode;
                    (*cur).last = ulccur as *mut _xmlNode;
                }
            }
        }
        3 | 4 | 5 | 6 | 7 | 8 | 12 => {
            if !(*cur).content.is_null()
                && (*cur).content != &raw mut (*cur).properties as *mut xmlChar
            {
                if !(!(*cur).doc.is_null()
                    && !(*(*cur).doc).dict.is_null()
                    && xmlDictOwns((*(*cur).doc).dict as xmlDictPtr, (*cur).content) != 0)
                {
                    xmlFree.expect("non-null function pointer")(
                        (*cur).content as *mut c_void,
                    );
                }
            }
            if !(*cur).children.is_null() {
                xmlFreeNodeList((*cur).children as xmlNodePtr);
            }
            (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
            (*cur).children = (*cur).last;
            if !content.is_null() {
                (*cur).content = xmlStrndup(content, len);
            } else {
                (*cur).content = ::core::ptr::null_mut::<xmlChar>();
            }
            (*cur).properties = ::core::ptr::null_mut::<_xmlAttr>();
        }
        9 | 14 | 13 | 10 | 18 | 19 | 20 | 15 | 16 | 17 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlNodeAddContentLen(
    mut cur: xmlNodePtr,
    mut content: *const xmlChar,
    mut len: c_int,
) {
    if cur.is_null() {
        return;
    }
    if len <= 0 as c_int {
        return;
    }
    match (*cur).type_0 as c_uint {
        11 | 1 => {
            let mut last: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            let mut newNode: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            last = (*cur).last as xmlNodePtr;
            newNode = xmlNewDocTextLen((*cur).doc as xmlDocPtr, content, len);
            if !newNode.is_null() {
                tmp = xmlAddChild(cur, newNode);
                if tmp != newNode {
                    return;
                }
                if !last.is_null() && (*last).next == newNode {
                    xmlTextMerge(last, newNode);
                }
            }
        }
        3 | 4 | 5 | 6 | 7 | 8 | 12 => {
            if !content.is_null() {
                if (*cur).content == &raw mut (*cur).properties as *mut xmlChar
                    || !(*cur).doc.is_null()
                        && !(*(*cur).doc).dict.is_null()
                        && xmlDictOwns((*(*cur).doc).dict as xmlDictPtr, (*cur).content) != 0
                {
                    (*cur).content = xmlStrncatNew((*cur).content, content, len);
                    (*cur).properties = ::core::ptr::null_mut::<_xmlAttr>();
                } else {
                    (*cur).content = xmlStrncat((*cur).content, content, len);
                }
            }
        }
        2 | 9 | 14 | 13 | 10 | 18 | 19 | 20 | 15 | 16 | 17 | _ => {}
    };
}
#[inline]
pub unsafe fn xmlNodeAddContent(mut cur: xmlNodePtr, mut content: *const xmlChar) {
    let mut len: c_int = 0;
    if cur.is_null() {
        return;
    }
    if content.is_null() {
        return;
    }
    len = xmlStrlen(content);
    xmlNodeAddContentLen(cur, content, len);
}
#[inline]
pub fn xmlTextMerge(mut first: xmlNodePtr, mut second: xmlNodePtr) -> xmlNodePtr { unsafe {
    if first.is_null() {
        return second;
    }
    if second.is_null() {
        return first;
    }
    if (*first).type_0 as c_uint
        != XML_TEXT_NODE as c_int as c_uint
    {
        return first;
    }
    if (*second).type_0 as c_uint
        != XML_TEXT_NODE as c_int as c_uint
    {
        return first;
    }
    if (*second).name != (*first).name {
        return first;
    }
    xmlNodeAddContent(first, (*second).content);
    xmlUnlinkNode(second);
    xmlFreeNode(second);
    return first;
} }
#[no_mangle]
pub unsafe extern "C" fn xmlGetNsList(
    mut doc: *const xmlDoc,
    mut node: *const xmlNode,
) -> *mut xmlNsPtr {
    let mut cur: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut ret: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
    let mut nbns: c_int = 0 as c_int;
    let mut maxns: c_int = 0 as c_int;
    let mut i: c_int = 0;
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNsPtr>();
    }
    while !node.is_null() {
        if (*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            cur = (*node).nsDef as xmlNsPtr;
            while !cur.is_null() {
                i = 0 as c_int;
                while i < nbns {
                    if (*cur).prefix == (**ret.offset(i as isize)).prefix
                        || xmlStrEqual((*cur).prefix, (**ret.offset(i as isize)).prefix) != 0
                    {
                        break;
                    }
                    i += 1;
                }
                if i >= nbns {
                    if nbns >= maxns {
                        let mut tmp: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
                        maxns = if maxns != 0 {
                            maxns * 2 as c_int
                        } else {
                            10 as c_int
                        };
                        tmp = xmlRealloc.expect("non-null function pointer")(
                            ret as *mut c_void,
                            ((maxns + 1 as c_int) as size_t)
                                .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
                        ) as *mut xmlNsPtr;
                        if tmp.is_null() {
                            xmlTreeErrMemory(
                                b"getting namespace list\0" as *const u8
                                    as *const c_char,
                            );
                            xmlFree.expect("non-null function pointer")(
                                ret as *mut c_void,
                            );
                            return ::core::ptr::null_mut::<xmlNsPtr>();
                        }
                        ret = tmp;
                    }
                    let fresh0 = nbns;
                    nbns = nbns + 1;
                    let ref mut fresh1 = *ret.offset(fresh0 as isize);
                    *fresh1 = cur;
                    let ref mut fresh2 = *ret.offset(nbns as isize);
                    *fresh2 = ::core::ptr::null_mut::<xmlNs>();
                }
                cur = (*cur).next as xmlNsPtr;
            }
        }
        node = (*node).parent;
    }
    return ret;
}
fn xmlTreeEnsureXMLDecl(mut doc: xmlDocPtr) -> xmlNsPtr { unsafe {
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    if !(*doc).oldNs.is_null() {
        return (*doc).oldNs as xmlNsPtr;
    }
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    ns = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNs>() as size_t)
        as xmlNsPtr;
    if ns.is_null() {
        xmlTreeErrMemory(
            b"allocating the XML namespace\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlNs>();
    }
    memset(
        ns as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNs>() as size_t,
    );
    (*ns).type_0 = XML_NAMESPACE_DECL;
    (*ns).href = xmlStrdup(XML_XML_NAMESPACE);
    (*ns).prefix = xmlStrdup(b"xml\0" as *const u8 as *const c_char as *const xmlChar);
    (*doc).oldNs = ns as *mut _xmlNs;
    return ns;
} }
#[no_mangle]
pub unsafe extern "C" fn xmlSearchNs(
    mut doc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut nameSpace: *const xmlChar,
) -> xmlNsPtr {
    let mut cur: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut orig: *const xmlNode = node as *const xmlNode;
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    if !nameSpace.is_null()
        && xmlStrEqual(
            nameSpace,
            b"xml\0" as *const u8 as *const c_char as *const xmlChar,
        ) != 0
    {
        if doc.is_null()
            && (*node).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
        {
            cur = xmlMalloc.expect("non-null function pointer")(
                ::core::mem::size_of::<xmlNs>() as size_t
            ) as xmlNsPtr;
            if cur.is_null() {
                xmlTreeErrMemory(
                    b"searching namespace\0" as *const u8 as *const c_char,
                );
                return ::core::ptr::null_mut::<xmlNs>();
            }
            memset(
                cur as *mut c_void,
                0 as c_int,
                ::core::mem::size_of::<xmlNs>() as size_t,
            );
            (*cur).type_0 = XML_NAMESPACE_DECL;
            (*cur).href = xmlStrdup(XML_XML_NAMESPACE);
            (*cur).prefix =
                xmlStrdup(b"xml\0" as *const u8 as *const c_char as *const xmlChar);
            (*cur).next = (*node).nsDef as *mut _xmlNs;
            (*node).nsDef = cur as *mut xmlNs;
            return cur;
        }
        if doc.is_null() {
            doc = (*node).doc as xmlDocPtr;
            if doc.is_null() {
                return ::core::ptr::null_mut::<xmlNs>();
            }
        }
        if (*doc).oldNs.is_null() {
            return xmlTreeEnsureXMLDecl(doc);
        } else {
            return (*doc).oldNs as xmlNsPtr;
        }
    }
    while !node.is_null() {
        if (*node).type_0 as c_uint
            == XML_ENTITY_REF_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_ENTITY_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_ENTITY_DECL as c_int as c_uint
        {
            return ::core::ptr::null_mut::<xmlNs>();
        }
        if (*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            cur = (*node).nsDef as xmlNsPtr;
            while !cur.is_null() {
                if (*cur).prefix.is_null() && nameSpace.is_null() && !(*cur).href.is_null() {
                    return cur;
                }
                if !(*cur).prefix.is_null()
                    && !nameSpace.is_null()
                    && !(*cur).href.is_null()
                    && xmlStrEqual((*cur).prefix, nameSpace) != 0
                {
                    return cur;
                }
                cur = (*cur).next as xmlNsPtr;
            }
            if orig != node as *const xmlNode {
                cur = (*node).ns as xmlNsPtr;
                if !cur.is_null() {
                    if (*cur).prefix.is_null() && nameSpace.is_null() && !(*cur).href.is_null() {
                        return cur;
                    }
                    if !(*cur).prefix.is_null()
                        && !nameSpace.is_null()
                        && !(*cur).href.is_null()
                        && xmlStrEqual((*cur).prefix, nameSpace) != 0
                    {
                        return cur;
                    }
                }
            }
        }
        node = (*node).parent as xmlNodePtr;
    }
    return ::core::ptr::null_mut::<xmlNs>();
}
unsafe fn xmlNsInScope(
    mut doc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut ancestor: xmlNodePtr,
    mut prefix: *const xmlChar,
) -> c_int {
    let mut tst: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    while !node.is_null() && node != ancestor {
        if (*node).type_0 as c_uint
            == XML_ENTITY_REF_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_ENTITY_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_ENTITY_DECL as c_int as c_uint
        {
            return -(1 as c_int);
        }
        if (*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            tst = (*node).nsDef as xmlNsPtr;
            while !tst.is_null() {
                if (*tst).prefix.is_null() && prefix.is_null() {
                    return 0 as c_int;
                }
                if !(*tst).prefix.is_null()
                    && !prefix.is_null()
                    && xmlStrEqual((*tst).prefix, prefix) != 0
                {
                    return 0 as c_int;
                }
                tst = (*tst).next as xmlNsPtr;
            }
        }
        node = (*node).parent as xmlNodePtr;
    }
    if node != ancestor {
        return -(1 as c_int);
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSearchNsByHref(
    mut doc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut href: *const xmlChar,
) -> xmlNsPtr {
    let mut cur: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut orig: xmlNodePtr = node;
    let mut is_attr: c_int = 0;
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
        || href.is_null()
    {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    if xmlStrEqual(href, XML_XML_NAMESPACE) != 0 {
        if doc.is_null()
            && (*node).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
        {
            cur = xmlMalloc.expect("non-null function pointer")(
                ::core::mem::size_of::<xmlNs>() as size_t
            ) as xmlNsPtr;
            if cur.is_null() {
                xmlTreeErrMemory(
                    b"searching namespace\0" as *const u8 as *const c_char,
                );
                return ::core::ptr::null_mut::<xmlNs>();
            }
            memset(
                cur as *mut c_void,
                0 as c_int,
                ::core::mem::size_of::<xmlNs>() as size_t,
            );
            (*cur).type_0 = XML_NAMESPACE_DECL;
            (*cur).href = xmlStrdup(XML_XML_NAMESPACE);
            (*cur).prefix =
                xmlStrdup(b"xml\0" as *const u8 as *const c_char as *const xmlChar);
            (*cur).next = (*node).nsDef as *mut _xmlNs;
            (*node).nsDef = cur as *mut xmlNs;
            return cur;
        }
        if doc.is_null() {
            doc = (*node).doc as xmlDocPtr;
            if doc.is_null() {
                return ::core::ptr::null_mut::<xmlNs>();
            }
        }
        if (*doc).oldNs.is_null() {
            return xmlTreeEnsureXMLDecl(doc);
        } else {
            return (*doc).oldNs as xmlNsPtr;
        }
    }
    is_attr = ((*node).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint)
        as c_int;
    while !node.is_null() {
        if (*node).type_0 as c_uint
            == XML_ENTITY_REF_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_ENTITY_NODE as c_int as c_uint
            || (*node).type_0 as c_uint
                == XML_ENTITY_DECL as c_int as c_uint
        {
            return ::core::ptr::null_mut::<xmlNs>();
        }
        if (*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            cur = (*node).nsDef as xmlNsPtr;
            while !cur.is_null() {
                if !(*cur).href.is_null() && !href.is_null() && xmlStrEqual((*cur).href, href) != 0
                {
                    if (is_attr == 0 || !(*cur).prefix.is_null())
                        && xmlNsInScope(doc, orig, node, (*cur).prefix) == 1 as c_int
                    {
                        return cur;
                    }
                }
                cur = (*cur).next as xmlNsPtr;
            }
            if orig != node {
                cur = (*node).ns as xmlNsPtr;
                if !cur.is_null() {
                    if !(*cur).href.is_null()
                        && !href.is_null()
                        && xmlStrEqual((*cur).href, href) != 0
                    {
                        if (is_attr == 0 || !(*cur).prefix.is_null())
                            && xmlNsInScope(doc, orig, node, (*cur).prefix)
                                == 1 as c_int
                        {
                            return cur;
                        }
                    }
                }
            }
        }
        node = (*node).parent as xmlNodePtr;
    }
    return ::core::ptr::null_mut::<xmlNs>();
}
fn xmlNewReconciledNs(
    mut doc: xmlDocPtr,
    mut tree: xmlNodePtr,
    mut ns: xmlNsPtr,
) -> xmlNsPtr { unsafe {
    let mut def: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut prefix: [xmlChar; 50] = [0; 50];
    let mut counter: c_int = 1 as c_int;
    if tree.is_null()
        || (*tree).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    if ns.is_null()
        || (*ns).type_0 as c_uint
            != XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    def = xmlSearchNsByHref(doc, tree, (*ns).href);
    if !def.is_null() {
        return def;
    }
    if (*ns).prefix.is_null() {
        snprintf(
            &raw mut prefix as *mut xmlChar as *mut c_char,
            ::core::mem::size_of::<[xmlChar; 50]>() as size_t,
            b"default\0" as *const u8 as *const c_char,
        );
    } else {
        snprintf(
            &raw mut prefix as *mut xmlChar as *mut c_char,
            ::core::mem::size_of::<[xmlChar; 50]>() as size_t,
            b"%.20s\0" as *const u8 as *const c_char,
            (*ns).prefix as *mut c_char,
        );
    }
    def = xmlSearchNs(doc, tree, &raw mut prefix as *mut xmlChar);
    while !def.is_null() {
        if counter > 1000 as c_int {
            return ::core::ptr::null_mut::<xmlNs>();
        }
        if (*ns).prefix.is_null() {
            let fresh4 = counter;
            counter = counter + 1;
            snprintf(
                &raw mut prefix as *mut xmlChar as *mut c_char,
                ::core::mem::size_of::<[xmlChar; 50]>() as size_t,
                b"default%d\0" as *const u8 as *const c_char,
                fresh4,
            );
        } else {
            let fresh5 = counter;
            counter = counter + 1;
            snprintf(
                &raw mut prefix as *mut xmlChar as *mut c_char,
                ::core::mem::size_of::<[xmlChar; 50]>() as size_t,
                b"%.20s%d\0" as *const u8 as *const c_char,
                (*ns).prefix as *mut c_char,
                fresh5,
            );
        }
        def = xmlSearchNs(doc, tree, &raw mut prefix as *mut xmlChar);
    }
    def = xmlNewNs(tree, (*ns).href, &raw mut prefix as *mut xmlChar);
    return def;
} }
#[inline]
pub fn xmlReconciliateNs(
    mut doc: xmlDocPtr,
    mut tree: xmlNodePtr,
) -> c_int { unsafe {
    let mut oldNs: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
    let mut newNs: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
    let mut sizeCache: c_int = 0 as c_int;
    let mut nbCache: c_int = 0 as c_int;
    let mut n: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut node: xmlNodePtr = tree;
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut ret: c_int = 0 as c_int;
    let mut i: c_int = 0;
    if node.is_null()
        || (*node).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return -(1 as c_int);
    }
    if doc.is_null()
        || (*doc).type_0 as c_uint
            != XML_DOCUMENT_NODE as c_int as c_uint
    {
        return -(1 as c_int);
    }
    if (*node).doc != doc {
        return -(1 as c_int);
    }
    while !node.is_null() {
        if !(*node).ns.is_null() {
            if sizeCache == 0 as c_int {
                sizeCache = 10 as c_int;
                oldNs = xmlMalloc.expect("non-null function pointer")(
                    (sizeCache as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
                ) as *mut xmlNsPtr;
                if oldNs.is_null() {
                    xmlTreeErrMemory(
                        b"fixing namespaces\0" as *const u8 as *const c_char,
                    );
                    return -(1 as c_int);
                }
                newNs = xmlMalloc.expect("non-null function pointer")(
                    (sizeCache as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
                ) as *mut xmlNsPtr;
                if newNs.is_null() {
                    xmlTreeErrMemory(
                        b"fixing namespaces\0" as *const u8 as *const c_char,
                    );
                    xmlFree.expect("non-null function pointer")(oldNs as *mut c_void);
                    return -(1 as c_int);
                }
            }
            i = 0 as c_int;
            while i < nbCache {
                if *oldNs.offset(i as isize) == (*node).ns {
                    (*node).ns = *newNs.offset(i as isize) as *mut xmlNs;
                    break;
                } else {
                    i += 1;
                }
            }
            if i == nbCache {
                n = xmlNewReconciledNs(doc, tree, (*node).ns as xmlNsPtr);
                if !n.is_null() {
                    if sizeCache <= nbCache {
                        sizeCache *= 2 as c_int;
                        oldNs = xmlRealloc.expect("non-null function pointer")(
                            oldNs as *mut c_void,
                            (sizeCache as size_t)
                                .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
                        ) as *mut xmlNsPtr;
                        if oldNs.is_null() {
                            xmlTreeErrMemory(
                                b"fixing namespaces\0" as *const u8 as *const c_char,
                            );
                            xmlFree.expect("non-null function pointer")(
                                newNs as *mut c_void,
                            );
                            return -(1 as c_int);
                        }
                        newNs = xmlRealloc.expect("non-null function pointer")(
                            newNs as *mut c_void,
                            (sizeCache as size_t)
                                .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
                        ) as *mut xmlNsPtr;
                        if newNs.is_null() {
                            xmlTreeErrMemory(
                                b"fixing namespaces\0" as *const u8 as *const c_char,
                            );
                            xmlFree.expect("non-null function pointer")(
                                oldNs as *mut c_void,
                            );
                            return -(1 as c_int);
                        }
                    }
                    let ref mut fresh6 = *newNs.offset(nbCache as isize);
                    *fresh6 = n;
                    let fresh7 = nbCache;
                    nbCache = nbCache + 1;
                    let ref mut fresh8 = *oldNs.offset(fresh7 as isize);
                    *fresh8 = (*node).ns as xmlNsPtr;
                    (*node).ns = n as *mut xmlNs;
                }
            }
        }
        if (*node).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            attr = (*node).properties as xmlAttrPtr;
            while !attr.is_null() {
                if !(*attr).ns.is_null() {
                    if sizeCache == 0 as c_int {
                        sizeCache = 10 as c_int;
                        oldNs = xmlMalloc.expect("non-null function pointer")(
                            (sizeCache as size_t)
                                .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
                        ) as *mut xmlNsPtr;
                        if oldNs.is_null() {
                            xmlTreeErrMemory(
                                b"fixing namespaces\0" as *const u8 as *const c_char,
                            );
                            return -(1 as c_int);
                        }
                        newNs = xmlMalloc.expect("non-null function pointer")(
                            (sizeCache as size_t)
                                .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
                        ) as *mut xmlNsPtr;
                        if newNs.is_null() {
                            xmlTreeErrMemory(
                                b"fixing namespaces\0" as *const u8 as *const c_char,
                            );
                            xmlFree.expect("non-null function pointer")(
                                oldNs as *mut c_void,
                            );
                            return -(1 as c_int);
                        }
                    }
                    i = 0 as c_int;
                    while i < nbCache {
                        if *oldNs.offset(i as isize) == (*attr).ns {
                            (*attr).ns = *newNs.offset(i as isize) as *mut xmlNs;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    if i == nbCache {
                        n = xmlNewReconciledNs(doc, tree, (*attr).ns as xmlNsPtr);
                        if !n.is_null() {
                            if sizeCache <= nbCache {
                                sizeCache *= 2 as c_int;
                                oldNs = xmlRealloc.expect("non-null function pointer")(
                                    oldNs as *mut c_void,
                                    (sizeCache as size_t).wrapping_mul(::core::mem::size_of::<
                                        xmlNsPtr,
                                    >(
                                    )
                                        as size_t),
                                ) as *mut xmlNsPtr;
                                if oldNs.is_null() {
                                    xmlTreeErrMemory(
                                        b"fixing namespaces\0" as *const u8
                                            as *const c_char,
                                    );
                                    xmlFree.expect("non-null function pointer")(
                                        newNs as *mut c_void,
                                    );
                                    return -(1 as c_int);
                                }
                                newNs = xmlRealloc.expect("non-null function pointer")(
                                    newNs as *mut c_void,
                                    (sizeCache as size_t).wrapping_mul(::core::mem::size_of::<
                                        xmlNsPtr,
                                    >(
                                    )
                                        as size_t),
                                ) as *mut xmlNsPtr;
                                if newNs.is_null() {
                                    xmlTreeErrMemory(
                                        b"fixing namespaces\0" as *const u8
                                            as *const c_char,
                                    );
                                    xmlFree.expect("non-null function pointer")(
                                        oldNs as *mut c_void,
                                    );
                                    return -(1 as c_int);
                                }
                            }
                            let ref mut fresh9 = *newNs.offset(nbCache as isize);
                            *fresh9 = n;
                            let fresh10 = nbCache;
                            nbCache = nbCache + 1;
                            let ref mut fresh11 = *oldNs.offset(fresh10 as isize);
                            *fresh11 = (*attr).ns as xmlNsPtr;
                            (*attr).ns = n as *mut xmlNs;
                        }
                    }
                }
                attr = (*attr).next as xmlAttrPtr;
            }
        }
        if !(*node).children.is_null()
            && (*node).type_0 as c_uint
                != XML_ENTITY_REF_NODE as c_int as c_uint
        {
            node = (*node).children as xmlNodePtr;
        } else if node != tree && !(*node).next.is_null() {
            node = (*node).next as xmlNodePtr;
        } else {
            if !(node != tree) {
                break;
            }
            while node != tree {
                if !(*node).parent.is_null() {
                    node = (*node).parent as xmlNodePtr;
                }
                if node != tree && !(*node).next.is_null() {
                    node = (*node).next as xmlNodePtr;
                    break;
                } else {
                    if !(*node).parent.is_null() {
                        continue;
                    }
                    node = ::core::ptr::null_mut::<xmlNode>();
                    break;
                }
            }
            if node == tree {
                node = ::core::ptr::null_mut::<xmlNode>();
            }
        }
    }
    if !oldNs.is_null() {
        xmlFree.expect("non-null function pointer")(oldNs as *mut c_void);
    }
    if !newNs.is_null() {
        xmlFree.expect("non-null function pointer")(newNs as *mut c_void);
    }
    return ret;
} }
unsafe fn xmlGetPropNodeInternal(
    mut node: *const xmlNode,
    mut name: *const xmlChar,
    mut nsName: *const xmlChar,
    mut useDTD: c_int,
) -> xmlAttrPtr {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if node.is_null()
        || (*node).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
        || name.is_null()
    {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    if !(*node).properties.is_null() {
        prop = (*node).properties as xmlAttrPtr;
        if nsName.is_null() {
            loop {
                if (*prop).ns.is_null() && xmlStrEqual((*prop).name, name) != 0 {
                    return prop;
                }
                prop = (*prop).next as xmlAttrPtr;
                if prop.is_null() {
                    break;
                }
            }
        } else {
            loop {
                if !(*prop).ns.is_null()
                    && xmlStrEqual((*prop).name, name) != 0
                    && ((*(*prop).ns).href == nsName
                        || xmlStrEqual((*(*prop).ns).href, nsName) != 0)
                {
                    return prop;
                }
                prop = (*prop).next as xmlAttrPtr;
                if prop.is_null() {
                    break;
                }
            }
        }
    }
    if useDTD == 0 {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    if !(*node).doc.is_null() && !(*(*node).doc).intSubset.is_null() {
        let mut doc: xmlDocPtr = (*node).doc as xmlDocPtr;
        let mut attrDecl: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
        let mut elemQName: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut tmpstr: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
            tmpstr = xmlStrdup((*(*node).ns).prefix);
            tmpstr = xmlStrcat(
                tmpstr,
                b":\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            tmpstr = xmlStrcat(tmpstr, (*node).name);
            if tmpstr.is_null() {
                return ::core::ptr::null_mut::<xmlAttr>();
            }
            elemQName = tmpstr;
        } else {
            elemQName = (*node).name as *mut xmlChar;
        }
        if nsName.is_null() {
            attrDecl = xmlGetDtdQAttrDesc(
                (*doc).intSubset as xmlDtdPtr,
                elemQName,
                name,
                ::core::ptr::null::<xmlChar>(),
            );
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdQAttrDesc(
                    (*doc).extSubset as xmlDtdPtr,
                    elemQName,
                    name,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
        } else if xmlStrEqual(nsName, XML_XML_NAMESPACE) != 0 {
            attrDecl = xmlGetDtdQAttrDesc(
                (*doc).intSubset as xmlDtdPtr,
                elemQName,
                name,
                b"xml\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdQAttrDesc(
                    (*doc).extSubset as xmlDtdPtr,
                    elemQName,
                    name,
                    b"xml\0" as *const u8 as *const c_char as *mut xmlChar,
                );
            }
        } else {
            let mut nsList: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
            let mut cur: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
            nsList = xmlGetNsList((*node).doc, node);
            if nsList.is_null() {
                if !tmpstr.is_null() {
                    xmlFree.expect("non-null function pointer")(tmpstr as *mut c_void);
                }
                return ::core::ptr::null_mut::<xmlAttr>();
            }
            cur = nsList;
            while !(*cur).is_null() {
                if xmlStrEqual((**cur).href, nsName) != 0 {
                    attrDecl = xmlGetDtdQAttrDesc(
                        (*doc).intSubset as xmlDtdPtr,
                        elemQName,
                        name,
                        (**cur).prefix,
                    );
                    if !attrDecl.is_null() {
                        break;
                    }
                    if !(*doc).extSubset.is_null() {
                        attrDecl = xmlGetDtdQAttrDesc(
                            (*doc).extSubset as xmlDtdPtr,
                            elemQName,
                            name,
                            (**cur).prefix,
                        );
                        if !attrDecl.is_null() {
                            break;
                        }
                    }
                }
                cur = cur.offset(1);
            }
            xmlFree.expect("non-null function pointer")(nsList as *mut c_void);
        }
        if !tmpstr.is_null() {
            xmlFree.expect("non-null function pointer")(tmpstr as *mut c_void);
        }
        if !attrDecl.is_null() && !(*attrDecl).defaultValue.is_null() {
            return attrDecl as xmlAttrPtr;
        }
    }
    return ::core::ptr::null_mut::<xmlAttr>();
}
unsafe fn xmlGetPropNodeValueInternal(mut prop: *const xmlAttr) -> *mut xmlChar {
    if prop.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*prop).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        if !(*prop).children.is_null() {
            if (*(*prop).children).next.is_null()
                && ((*(*prop).children).type_0 as c_uint
                    == XML_TEXT_NODE as c_int as c_uint
                    || (*(*prop).children).type_0 as c_uint
                        == XML_CDATA_SECTION_NODE as c_int as c_uint)
            {
                return xmlStrdup((*(*prop).children).content);
            } else {
                let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                ret = xmlNodeListGetString(
                    (*prop).doc as xmlDocPtr,
                    (*prop).children,
                    1 as c_int,
                );
                if !ret.is_null() {
                    return ret;
                }
            }
        }
        return xmlStrdup(b"\0" as *const u8 as *const c_char as *mut xmlChar);
    } else if (*prop).type_0 as c_uint
        == XML_ATTRIBUTE_DECL as c_int as c_uint
    {
        return xmlStrdup((*(prop as xmlAttributePtr)).defaultValue);
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlHasProp(
    mut node: *const xmlNode,
    mut name: *const xmlChar,
) -> xmlAttrPtr {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if node.is_null()
        || (*node).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
        || name.is_null()
    {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    prop = (*node).properties as xmlAttrPtr;
    while !prop.is_null() {
        if xmlStrEqual((*prop).name, name) != 0 {
            return prop;
        }
        prop = (*prop).next as xmlAttrPtr;
    }
    if xmlCheckDTD == 0 {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    doc = (*node).doc as xmlDocPtr;
    if !doc.is_null() {
        let mut attrDecl: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
        if !(*doc).intSubset.is_null() {
            attrDecl = xmlGetDtdAttrDesc((*doc).intSubset as xmlDtdPtr, (*node).name, name);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdAttrDesc((*doc).extSubset as xmlDtdPtr, (*node).name, name);
            }
            if !attrDecl.is_null() && !(*attrDecl).defaultValue.is_null() {
                return attrDecl as xmlAttrPtr;
            }
        }
    }
    return ::core::ptr::null_mut::<xmlAttr>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlHasNsProp(
    mut node: *const xmlNode,
    mut name: *const xmlChar,
    mut nameSpace: *const xmlChar,
) -> xmlAttrPtr {
    return xmlGetPropNodeInternal(node, name, nameSpace, xmlCheckDTD);
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetProp(
    mut node: *const xmlNode,
    mut name: *const xmlChar,
) -> *mut xmlChar {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    prop = xmlHasProp(node, name);
    if prop.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlGetPropNodeValueInternal(prop as *const xmlAttr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetNoNsProp(
    mut node: *const xmlNode,
    mut name: *const xmlChar,
) -> *mut xmlChar {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    prop = xmlGetPropNodeInternal(node, name, ::core::ptr::null::<xmlChar>(), xmlCheckDTD);
    if prop.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlGetPropNodeValueInternal(prop as *const xmlAttr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetNsProp(
    mut node: *const xmlNode,
    mut name: *const xmlChar,
    mut nameSpace: *const xmlChar,
) -> *mut xmlChar {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    prop = xmlGetPropNodeInternal(node, name, nameSpace, xmlCheckDTD);
    if prop.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlGetPropNodeValueInternal(prop as *const xmlAttr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUnsetProp(
    mut node: xmlNodePtr,
    mut name: *const xmlChar,
) -> c_int {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    prop = xmlGetPropNodeInternal(
        node as *const xmlNode,
        name,
        ::core::ptr::null::<xmlChar>(),
        0 as c_int,
    );
    if prop.is_null() {
        return -(1 as c_int);
    }
    xmlUnlinkNode(prop as xmlNodePtr);
    xmlFreeProp(prop);
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlUnsetNsProp(
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
    mut name: *const xmlChar,
) -> c_int {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    prop = xmlGetPropNodeInternal(
        node as *const xmlNode,
        name,
        if !ns.is_null() {
            (*ns).href
        } else {
            ::core::ptr::null::<xmlChar>()
        },
        0 as c_int,
    );
    if prop.is_null() {
        return -(1 as c_int);
    }
    xmlUnlinkNode(prop as xmlNodePtr);
    xmlFreeProp(prop);
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSetProp(
    mut node: xmlNodePtr,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
) -> xmlAttrPtr {
    let mut len: c_int = 0;
    let mut nqname: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if node.is_null()
        || name.is_null()
        || (*node).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    nqname = xmlSplitQName3(name, &raw mut len);
    if !nqname.is_null() {
        let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
        let mut prefix: *mut xmlChar = xmlStrndup(name, len);
        ns = xmlSearchNs((*node).doc as xmlDocPtr, node, prefix);
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        }
        if !ns.is_null() {
            return xmlSetNsProp(node, ns, nqname, value);
        }
    }
    return xmlSetNsProp(node, ::core::ptr::null_mut::<xmlNs>(), name, value);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSetNsProp(
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
) -> xmlAttrPtr {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if !ns.is_null() && (*ns).href.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    prop = xmlGetPropNodeInternal(
        node as *const xmlNode,
        name,
        if !ns.is_null() {
            (*ns).href
        } else {
            ::core::ptr::null::<xmlChar>()
        },
        0 as c_int,
    );
    if !prop.is_null() {
        if (*prop).atype as c_uint
            == XML_ATTRIBUTE_ID as c_int as c_uint
        {
            xmlRemoveID((*node).doc as xmlDocPtr, prop);
            (*prop).atype = XML_ATTRIBUTE_ID;
        }
        if !(*prop).children.is_null() {
            xmlFreeNodeList((*prop).children as xmlNodePtr);
        }
        (*prop).children = ::core::ptr::null_mut::<_xmlNode>();
        (*prop).last = ::core::ptr::null_mut::<_xmlNode>();
        (*prop).ns = ns as *mut xmlNs;
        if !value.is_null() {
            let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            (*prop).children = xmlNewDocText((*node).doc, value) as *mut _xmlNode;
            (*prop).last = ::core::ptr::null_mut::<_xmlNode>();
            tmp = (*prop).children as xmlNodePtr;
            while !tmp.is_null() {
                (*tmp).parent = prop as xmlNodePtr as *mut _xmlNode;
                if (*tmp).next.is_null() {
                    (*prop).last = tmp as *mut _xmlNode;
                }
                tmp = (*tmp).next as xmlNodePtr;
            }
        }
        if (*prop).atype as c_uint
            == XML_ATTRIBUTE_ID as c_int as c_uint
        {
            xmlAddID(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                (*node).doc as xmlDocPtr,
                value,
                prop,
            );
        }
        return prop;
    }
    return xmlNewPropInternal(node, ns, name, value, 0 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlNodeIsText(mut node: *const xmlNode) -> c_int {
    if node.is_null() {
        return 0 as c_int;
    }
    if (*node).type_0 as c_uint
        == XML_TEXT_NODE as c_int as c_uint
    {
        return 1 as c_int;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsBlankNode(mut node: *const xmlNode) -> c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if node.is_null() {
        return 0 as c_int;
    }
    if (*node).type_0 as c_uint
        != XML_TEXT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_CDATA_SECTION_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    if (*node).content.is_null() {
        return 1 as c_int;
    }
    cur = (*node).content;
    while *cur as c_int != 0 as c_int {
        if !(*cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int)
        {
            return 0 as c_int;
        }
        cur = cur.offset(1);
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextConcat(
    mut node: xmlNodePtr,
    mut content: *const xmlChar,
    mut len: c_int,
) -> c_int {
    if node.is_null() {
        return -(1 as c_int);
    }
    if (*node).type_0 as c_uint
        != XML_TEXT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_CDATA_SECTION_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_COMMENT_NODE as c_int as c_uint
        && (*node).type_0 as c_uint
            != XML_PI_NODE as c_int as c_uint
    {
        return -(1 as c_int);
    }
    if (*node).content == &raw mut (*node).properties as *mut xmlChar
        || !(*node).doc.is_null()
            && !(*(*node).doc).dict.is_null()
            && xmlDictOwns((*(*node).doc).dict as xmlDictPtr, (*node).content) != 0
    {
        (*node).content = xmlStrncatNew((*node).content, content, len);
    } else {
        (*node).content = xmlStrncat((*node).content, content, len);
    }
    (*node).properties = ::core::ptr::null_mut::<_xmlAttr>();
    if (*node).content.is_null() {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[inline]
pub fn xmlBufferCreate() -> xmlBufferPtr { unsafe {
    let mut ret: xmlBufferPtr = ::core::ptr::null_mut::<xmlBuffer>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlBuffer>() as size_t
    ) as xmlBufferPtr;
    if ret.is_null() {
        xmlTreeErrMemory(b"creating buffer\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlBuffer>();
    }
    (*ret).use_0 = 0 as c_uint;
    (*ret).size = *__xmlDefaultBufferSize() as c_uint;
    (*ret).alloc = *__xmlBufferAllocScheme();
    (*ret).content =
        xmlMallocAtomic.expect("non-null function pointer")((*ret).size as size_t) as *mut xmlChar;
    if (*ret).content.is_null() {
        xmlTreeErrMemory(b"creating buffer\0" as *const u8 as *const c_char);
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
        return ::core::ptr::null_mut::<xmlBuffer>();
    }
    *(*ret).content.offset(0 as c_int as isize) = 0 as xmlChar;
    (*ret).contentIO = ::core::ptr::null_mut::<xmlChar>();
    return ret;
} }
#[inline]
pub fn xmlBufferCreateSize(mut size: size_t) -> xmlBufferPtr { unsafe {
    let mut ret: xmlBufferPtr = ::core::ptr::null_mut::<xmlBuffer>();
    if size >= UINT_MAX as size_t {
        return ::core::ptr::null_mut::<xmlBuffer>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlBuffer>() as size_t
    ) as xmlBufferPtr;
    if ret.is_null() {
        xmlTreeErrMemory(b"creating buffer\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlBuffer>();
    }
    (*ret).use_0 = 0 as c_uint;
    (*ret).alloc = *__xmlBufferAllocScheme();
    (*ret).size = (if size != 0 {
        size.wrapping_add(1 as size_t)
    } else {
        0 as size_t
    }) as c_uint;
    if (*ret).size != 0 {
        (*ret).content = xmlMallocAtomic.expect("non-null function pointer")((*ret).size as size_t)
            as *mut xmlChar;
        if (*ret).content.is_null() {
            xmlTreeErrMemory(b"creating buffer\0" as *const u8 as *const c_char);
            xmlFree.expect("non-null function pointer")(ret as *mut c_void);
            return ::core::ptr::null_mut::<xmlBuffer>();
        }
        *(*ret).content.offset(0 as c_int as isize) = 0 as xmlChar;
    } else {
        (*ret).content = ::core::ptr::null_mut::<xmlChar>();
    }
    (*ret).contentIO = ::core::ptr::null_mut::<xmlChar>();
    return ret;
} }
#[inline]
pub fn xmlBufferDetach(mut buf: xmlBufferPtr) -> *mut xmlChar { unsafe {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if buf.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = (*buf).content;
    (*buf).content = ::core::ptr::null_mut::<xmlChar>();
    (*buf).size = 0 as c_uint;
    (*buf).use_0 = 0 as c_uint;
    return ret;
} }
#[inline]
pub unsafe fn xmlBufferCreateStatic(
    mut mem: *mut c_void,
    mut size: size_t,
) -> xmlBufferPtr {
    let mut buf: xmlBufferPtr = xmlBufferCreateSize(size);
    xmlBufferAdd(buf, mem as *const xmlChar, size as c_int);
    return buf;
}
#[inline]
pub fn xmlBufferSetAllocationScheme(
    mut buf: xmlBufferPtr,
    mut scheme: xmlBufferAllocationScheme,
) { unsafe {
    if buf.is_null() {
        return;
    }
    if (*buf).alloc as c_uint
        == XML_BUFFER_ALLOC_IO as c_int as c_uint
    {
        return;
    }
    if scheme as c_uint
        == XML_BUFFER_ALLOC_DOUBLEIT as c_int as c_uint
        || scheme as c_uint
            == XML_BUFFER_ALLOC_EXACT as c_int as c_uint
        || scheme as c_uint
            == XML_BUFFER_ALLOC_HYBRID as c_int as c_uint
    {
        (*buf).alloc = scheme;
    }
} }
#[inline]
pub fn xmlBufferFree(mut buf: xmlBufferPtr) { unsafe {
    if buf.is_null() {
        return;
    }
    if (*buf).alloc as c_uint
        == XML_BUFFER_ALLOC_IO as c_int as c_uint
        && !(*buf).contentIO.is_null()
    {
        xmlFree.expect("non-null function pointer")((*buf).contentIO as *mut c_void);
    } else if !(*buf).content.is_null() {
        xmlFree.expect("non-null function pointer")((*buf).content as *mut c_void);
    }
    xmlFree.expect("non-null function pointer")(buf as *mut c_void);
} }
#[inline]
pub fn xmlBufferEmpty(mut buf: xmlBufferPtr) { unsafe {
    if buf.is_null() {
        return;
    }
    if (*buf).content.is_null() {
        return;
    }
    (*buf).use_0 = 0 as c_uint;
    if (*buf).alloc as c_uint
        == XML_BUFFER_ALLOC_IO as c_int as c_uint
        && !(*buf).contentIO.is_null()
    {
        let mut start_buf: size_t =
            (*buf).content.offset_from((*buf).contentIO) as c_long as size_t;
        (*buf).size = ((*buf).size as c_ulong)
            .wrapping_add(start_buf as c_ulong)
            as c_uint as c_uint;
        (*buf).content = (*buf).contentIO;
        *(*buf).content.offset(0 as c_int as isize) = 0 as xmlChar;
    } else {
        *(*buf).content.offset(0 as c_int as isize) = 0 as xmlChar;
    };
} }
#[inline]
pub fn xmlBufferShrink(
    mut buf: xmlBufferPtr,
    mut len: c_uint,
) -> c_int { unsafe {
    if buf.is_null() {
        return -(1 as c_int);
    }
    if len == 0 as c_uint {
        return 0 as c_int;
    }
    if len > (*buf).use_0 {
        return -(1 as c_int);
    }
    (*buf).use_0 = (*buf).use_0.wrapping_sub(len);
    if (*buf).alloc as c_uint
        == XML_BUFFER_ALLOC_IO as c_int as c_uint
        && !(*buf).contentIO.is_null()
    {
        (*buf).content = (*buf).content.offset(len as isize);
        (*buf).size = (*buf).size.wrapping_sub(len);
        if (*buf).alloc as c_uint
            == XML_BUFFER_ALLOC_IO as c_int as c_uint
            && !(*buf).contentIO.is_null()
        {
            let mut start_buf: size_t =
                (*buf).content.offset_from((*buf).contentIO) as c_long as size_t;
            if start_buf >= (*buf).size as size_t {
                memmove(
                    (*buf).contentIO as *mut c_void,
                    (*buf).content.offset(0 as c_int as isize) as *mut xmlChar
                        as *const c_void,
                    (*buf).use_0 as size_t,
                );
                (*buf).content = (*buf).contentIO;
                *(*buf).content.offset((*buf).use_0 as isize) = 0 as xmlChar;
                (*buf).size = ((*buf).size as c_ulong)
                    .wrapping_add(start_buf as c_ulong)
                    as c_uint as c_uint;
            }
        }
    } else {
        memmove(
            (*buf).content as *mut c_void,
            (*buf).content.offset(len as isize) as *mut xmlChar as *const c_void,
            (*buf).use_0 as size_t,
        );
        *(*buf).content.offset((*buf).use_0 as isize) = 0 as xmlChar;
    }
    return len as c_int;
} }
#[inline]
pub fn xmlBufferGrow(
    mut buf: xmlBufferPtr,
    mut len: c_uint,
) -> c_int { unsafe {
    let mut size: c_uint = 0;
    let mut newbuf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if buf.is_null() {
        return -(1 as c_int);
    }
    if len < (*buf).size.wrapping_sub((*buf).use_0) {
        return 0 as c_int;
    }
    if len >= UINT_MAX.wrapping_sub((*buf).use_0) {
        xmlTreeErrMemory(
            b"growing buffer past UINT_MAX\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if (*buf).size as size_t > len as size_t {
        size = if (*buf).size > UINT_MAX.wrapping_div(2 as c_uint) {
            UINT_MAX
        } else {
            (*buf).size.wrapping_mul(2 as c_uint)
        };
    } else {
        size = (*buf).use_0.wrapping_add(len);
        size = if size > UINT_MAX.wrapping_sub(100 as c_uint) {
            UINT_MAX
        } else {
            size.wrapping_add(100 as c_uint)
        };
    }
    if (*buf).alloc as c_uint
        == XML_BUFFER_ALLOC_IO as c_int as c_uint
        && !(*buf).contentIO.is_null()
    {
        let mut start_buf: size_t =
            (*buf).content.offset_from((*buf).contentIO) as c_long as size_t;
        newbuf = xmlRealloc.expect("non-null function pointer")(
            (*buf).contentIO as *mut c_void,
            start_buf.wrapping_add(size as size_t),
        ) as *mut xmlChar;
        if newbuf.is_null() {
            xmlTreeErrMemory(b"growing buffer\0" as *const u8 as *const c_char);
            return -(1 as c_int);
        }
        (*buf).contentIO = newbuf;
        (*buf).content = newbuf.offset(start_buf as isize);
    } else {
        newbuf = xmlRealloc.expect("non-null function pointer")(
            (*buf).content as *mut c_void,
            size as size_t,
        ) as *mut xmlChar;
        if newbuf.is_null() {
            xmlTreeErrMemory(b"growing buffer\0" as *const u8 as *const c_char);
            return -(1 as c_int);
        }
        (*buf).content = newbuf;
    }
    (*buf).size = size;
    return (*buf)
        .size
        .wrapping_sub((*buf).use_0)
        .wrapping_sub(1 as c_uint) as c_int;
} }
#[inline]
pub unsafe fn xmlBufferDump(
    mut file: *mut FILE,
    mut buf: xmlBufferPtr,
) -> c_int {
    let mut ret: size_t = 0;
    if buf.is_null() {
        return 0 as c_int;
    }
    if (*buf).content.is_null() {
        return 0 as c_int;
    }
    if file.is_null() {
        file = stdout;
    }
    ret = fwrite(
        (*buf).content as *const c_void,
        1 as size_t,
        (*buf).use_0 as size_t,
        file,
    ) as size_t;
    return (if ret > INT_MAX as size_t {
        INT_MAX as size_t
    } else {
        ret
    }) as c_int;
}
#[inline]
pub unsafe fn xmlBufferContent(mut buf: *const xmlBuffer) -> *const xmlChar {
    if buf.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return (*buf).content;
}
#[inline]
pub unsafe fn xmlBufferLength(mut buf: *const xmlBuffer) -> c_int {
    if buf.is_null() {
        return 0 as c_int;
    }
    return (*buf).use_0 as c_int;
}
#[inline]
pub fn xmlBufferResize(
    mut buf: xmlBufferPtr,
    mut size: c_uint,
) -> c_int { unsafe {
    let mut newSize: c_uint = 0;
    let mut rebuf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut start_buf: size_t = 0;
    if buf.is_null() {
        return 0 as c_int;
    }
    if size < (*buf).size {
        return 1 as c_int;
    }
    if size > UINT_MAX.wrapping_sub(10 as c_uint) {
        xmlTreeErrMemory(
            b"growing buffer past UINT_MAX\0" as *const u8 as *const c_char,
        );
        return 0 as c_int;
    }
    match (*buf).alloc as c_uint {
        3 | 0 => {
            if (*buf).size == 0 as c_uint {
                newSize = if size > UINT_MAX.wrapping_sub(10 as c_uint) {
                    UINT_MAX
                } else {
                    size.wrapping_add(10 as c_uint)
                };
            } else {
                newSize = (*buf).size;
            }
            while size > newSize {
                if newSize > UINT_MAX.wrapping_div(2 as c_uint) {
                    xmlTreeErrMemory(
                        b"growing buffer\0" as *const u8 as *const c_char,
                    );
                    return 0 as c_int;
                }
                newSize = newSize.wrapping_mul(2 as c_uint);
            }
        }
        1 => {
            newSize = if size > UINT_MAX.wrapping_sub(10 as c_uint) {
                UINT_MAX
            } else {
                size.wrapping_add(10 as c_uint)
            };
        }
        4 => {
            if (*buf).use_0 < BASE_BUFFER_SIZE as c_uint {
                newSize = size;
            } else {
                newSize = (*buf).size;
                while size > newSize {
                    if newSize > UINT_MAX.wrapping_div(2 as c_uint) {
                        xmlTreeErrMemory(
                            b"growing buffer\0" as *const u8 as *const c_char,
                        );
                        return 0 as c_int;
                    }
                    newSize = newSize.wrapping_mul(2 as c_uint);
                }
            }
        }
        _ => {
            newSize = if size > UINT_MAX.wrapping_sub(10 as c_uint) {
                UINT_MAX
            } else {
                size.wrapping_add(10 as c_uint)
            };
        }
    }
    if (*buf).alloc as c_uint
        == XML_BUFFER_ALLOC_IO as c_int as c_uint
        && !(*buf).contentIO.is_null()
    {
        start_buf = (*buf).content.offset_from((*buf).contentIO) as c_long as size_t;
        if start_buf > newSize as size_t {
            memmove(
                (*buf).contentIO as *mut c_void,
                (*buf).content as *const c_void,
                (*buf).use_0 as size_t,
            );
            (*buf).content = (*buf).contentIO;
            *(*buf).content.offset((*buf).use_0 as isize) = 0 as xmlChar;
            (*buf).size = ((*buf).size as c_ulong)
                .wrapping_add(start_buf as c_ulong)
                as c_uint as c_uint;
        } else {
            rebuf = xmlRealloc.expect("non-null function pointer")(
                (*buf).contentIO as *mut c_void,
                start_buf.wrapping_add(newSize as size_t),
            ) as *mut xmlChar;
            if rebuf.is_null() {
                xmlTreeErrMemory(b"growing buffer\0" as *const u8 as *const c_char);
                return 0 as c_int;
            }
            (*buf).contentIO = rebuf;
            (*buf).content = rebuf.offset(start_buf as isize);
        }
    } else {
        if (*buf).content.is_null() {
            rebuf = xmlMallocAtomic.expect("non-null function pointer")(newSize as size_t)
                as *mut xmlChar;
            (*buf).use_0 = 0 as c_uint;
            *rebuf.offset((*buf).use_0 as isize) = 0 as xmlChar;
        } else if (*buf).size.wrapping_sub((*buf).use_0) < 100 as c_uint {
            rebuf = xmlRealloc.expect("non-null function pointer")(
                (*buf).content as *mut c_void,
                newSize as size_t,
            ) as *mut xmlChar;
        } else {
            rebuf = xmlMallocAtomic.expect("non-null function pointer")(newSize as size_t)
                as *mut xmlChar;
            if !rebuf.is_null() {
                memcpy(
                    rebuf as *mut c_void,
                    (*buf).content as *const c_void,
                    (*buf).use_0 as size_t,
                );
                xmlFree.expect("non-null function pointer")(
                    (*buf).content as *mut c_void,
                );
                *rebuf.offset((*buf).use_0 as isize) = 0 as xmlChar;
            }
        }
        if rebuf.is_null() {
            xmlTreeErrMemory(b"growing buffer\0" as *const u8 as *const c_char);
            return 0 as c_int;
        }
        (*buf).content = rebuf;
    }
    (*buf).size = newSize;
    return 1 as c_int;
} }
#[inline]
pub unsafe fn xmlBufferAdd(
    mut buf: xmlBufferPtr,
    mut str: *const xmlChar,
    mut len: c_int,
) -> c_int {
    let mut needSize: c_uint = 0;
    if str.is_null() || buf.is_null() {
        return -(1 as c_int);
    }
    if len < -(1 as c_int) {
        return -(1 as c_int);
    }
    if len == 0 as c_int {
        return 0 as c_int;
    }
    if len < 0 as c_int {
        len = xmlStrlen(str);
    }
    if len < 0 as c_int {
        return -(1 as c_int);
    }
    if len == 0 as c_int {
        return 0 as c_int;
    }
    if len as c_uint >= (*buf).size.wrapping_sub((*buf).use_0) {
        if len as c_uint >= UINT_MAX.wrapping_sub((*buf).use_0) {
            xmlTreeErrMemory(
                b"growing buffer past UINT_MAX\0" as *const u8 as *const c_char,
            );
            return XML_ERR_NO_MEMORY as c_int;
        }
        needSize = (*buf)
            .use_0
            .wrapping_add(len as c_uint)
            .wrapping_add(1 as c_uint);
        if xmlBufferResize(buf, needSize) == 0 {
            xmlTreeErrMemory(b"growing buffer\0" as *const u8 as *const c_char);
            return XML_ERR_NO_MEMORY as c_int;
        }
    }
    memmove(
        (*buf).content.offset((*buf).use_0 as isize) as *mut xmlChar as *mut c_void,
        str as *const c_void,
        len as size_t,
    );
    (*buf).use_0 = (*buf).use_0.wrapping_add(len as c_uint);
    *(*buf).content.offset((*buf).use_0 as isize) = 0 as xmlChar;
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlBufferAddHead(
    mut buf: xmlBufferPtr,
    mut str: *const xmlChar,
    mut len: c_int,
) -> c_int {
    let mut needSize: c_uint = 0;
    if buf.is_null() {
        return -(1 as c_int);
    }
    if str.is_null() {
        return -(1 as c_int);
    }
    if len < -(1 as c_int) {
        return -(1 as c_int);
    }
    if len == 0 as c_int {
        return 0 as c_int;
    }
    if len < 0 as c_int {
        len = xmlStrlen(str);
    }
    if len <= 0 as c_int {
        return -(1 as c_int);
    }
    if (*buf).alloc as c_uint
        == XML_BUFFER_ALLOC_IO as c_int as c_uint
        && !(*buf).contentIO.is_null()
    {
        let mut start_buf: size_t =
            (*buf).content.offset_from((*buf).contentIO) as c_long as size_t;
        if start_buf > len as c_uint as size_t {
            (*buf).content = (*buf).content.offset(-(len as isize));
            memmove(
                (*buf).content.offset(0 as c_int as isize) as *mut xmlChar
                    as *mut c_void,
                str as *const c_void,
                len as size_t,
            );
            (*buf).use_0 = (*buf).use_0.wrapping_add(len as c_uint);
            (*buf).size = (*buf).size.wrapping_add(len as c_uint);
            *(*buf).content.offset((*buf).use_0 as isize) = 0 as xmlChar;
            return 0 as c_int;
        }
    }
    if len as c_uint >= (*buf).size.wrapping_sub((*buf).use_0) {
        if len as c_uint >= UINT_MAX.wrapping_sub((*buf).use_0) {
            xmlTreeErrMemory(
                b"growing buffer past UINT_MAX\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        needSize = (*buf)
            .use_0
            .wrapping_add(len as c_uint)
            .wrapping_add(1 as c_uint);
        if xmlBufferResize(buf, needSize) == 0 {
            xmlTreeErrMemory(b"growing buffer\0" as *const u8 as *const c_char);
            return XML_ERR_NO_MEMORY as c_int;
        }
    }
    memmove(
        (*buf).content.offset(len as isize) as *mut xmlChar as *mut c_void,
        (*buf).content.offset(0 as c_int as isize) as *mut xmlChar
            as *const c_void,
        (*buf).use_0 as size_t,
    );
    memmove(
        (*buf).content.offset(0 as c_int as isize) as *mut xmlChar
            as *mut c_void,
        str as *const c_void,
        len as size_t,
    );
    (*buf).use_0 = (*buf).use_0.wrapping_add(len as c_uint);
    *(*buf).content.offset((*buf).use_0 as isize) = 0 as xmlChar;
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlBufferCat(
    mut buf: xmlBufferPtr,
    mut str: *const xmlChar,
) -> c_int {
    if buf.is_null() {
        return -(1 as c_int);
    }
    if str.is_null() {
        return -(1 as c_int);
    }
    return xmlBufferAdd(buf, str, -(1 as c_int));
}
#[inline]
pub unsafe fn xmlBufferCCat(
    mut buf: xmlBufferPtr,
    mut str: *const c_char,
) -> c_int {
    return xmlBufferCat(buf, str as *const xmlChar);
}
#[inline]
pub unsafe fn xmlBufferWriteCHAR(mut buf: xmlBufferPtr, mut string: *const xmlChar) {
    if buf.is_null() {
        return;
    }
    xmlBufferCat(buf, string);
}
#[inline]
pub unsafe fn xmlBufferWriteChar(
    mut buf: xmlBufferPtr,
    mut string: *const c_char,
) {
    if buf.is_null() {
        return;
    }
    xmlBufferCCat(buf, string);
}
#[inline]
pub unsafe fn xmlBufferWriteQuotedString(
    mut buf: xmlBufferPtr,
    mut string: *const xmlChar,
) {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut base: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if buf.is_null() {
        return;
    }
    if !xmlStrchr(string, '"' as i32 as xmlChar).is_null() {
        if !xmlStrchr(string, '\'' as i32 as xmlChar).is_null() {
            xmlBufferCCat(buf, b"\"\0" as *const u8 as *const c_char);
            cur = string;
            base = cur;
            while *cur as c_int != 0 as c_int {
                if *cur as c_int == '"' as i32 {
                    if base != cur {
                        xmlBufferAdd(
                            buf,
                            base,
                            cur.offset_from(base) as c_long as c_int,
                        );
                    }
                    xmlBufferAdd(
                        buf,
                        b"&quot;\0" as *const u8 as *const c_char as *mut xmlChar,
                        6 as c_int,
                    );
                    cur = cur.offset(1);
                    base = cur;
                } else {
                    cur = cur.offset(1);
                }
            }
            if base != cur {
                xmlBufferAdd(
                    buf,
                    base,
                    cur.offset_from(base) as c_long as c_int,
                );
            }
            xmlBufferCCat(buf, b"\"\0" as *const u8 as *const c_char);
        } else {
            xmlBufferCCat(buf, b"'\0" as *const u8 as *const c_char);
            xmlBufferCat(buf, string);
            xmlBufferCCat(buf, b"'\0" as *const u8 as *const c_char);
        }
    } else {
        xmlBufferCCat(buf, b"\"\0" as *const u8 as *const c_char);
        xmlBufferCat(buf, string);
        xmlBufferCCat(buf, b"\"\0" as *const u8 as *const c_char);
    };
}
#[inline]
pub unsafe fn xmlGetDocCompressMode(mut doc: *const xmlDoc) -> c_int {
    if doc.is_null() {
        return -(1 as c_int);
    }
    return (*doc).compression;
}
#[no_mangle]
pub extern "C" fn xmlSetDocCompressMode(mut doc: xmlDocPtr, mut mode: c_int) { unsafe {
    if doc.is_null() {
        return;
    }
    if mode < 0 as c_int {
        (*doc).compression = 0 as c_int;
    } else if mode > 9 as c_int {
        (*doc).compression = 9 as c_int;
    } else {
        (*doc).compression = mode;
    };
} }
#[inline]
pub fn xmlGetCompressMode() -> c_int { unsafe {
    return xmlCompressMode;
} }
#[inline]
pub fn xmlSetCompressMode(mut mode: c_int) { unsafe {
    if mode < 0 as c_int {
        xmlCompressMode = 0 as c_int;
    } else if mode > 9 as c_int {
        xmlCompressMode = 9 as c_int;
    } else {
        xmlCompressMode = mode;
    };
} }
pub const XML_TREE_NSMAP_PARENT: c_int = -(1 as c_int);
pub const XML_TREE_NSMAP_DOC: c_int = -(3 as c_int);
pub const XML_TREE_NSMAP_CUSTOM: c_int = -(4 as c_int);
fn xmlDOMWrapNsMapFree(mut nsmap: xmlNsMapPtr) { unsafe {
    let mut cur: xmlNsMapItemPtr = ::core::ptr::null_mut::<xmlNsMapItem>();
    let mut tmp: xmlNsMapItemPtr = ::core::ptr::null_mut::<xmlNsMapItem>();
    if nsmap.is_null() {
        return;
    }
    cur = (*nsmap).pool;
    while !cur.is_null() {
        tmp = cur;
        cur = (*cur).next;
        xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
    }
    cur = (*nsmap).first;
    while !cur.is_null() {
        tmp = cur;
        cur = (*cur).next;
        xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
    }
    xmlFree.expect("non-null function pointer")(nsmap as *mut c_void);
} }
unsafe fn xmlDOMWrapNsMapAddItem(
    mut nsmap: *mut xmlNsMapPtr,
    mut position: c_int,
    mut oldNs: xmlNsPtr,
    mut newNs: xmlNsPtr,
    mut depth: c_int,
) -> xmlNsMapItemPtr {
    let mut ret: xmlNsMapItemPtr = ::core::ptr::null_mut::<xmlNsMapItem>();
    let mut map: xmlNsMapPtr = ::core::ptr::null_mut::<xmlNsMap>();
    if nsmap.is_null() {
        return ::core::ptr::null_mut::<xmlNsMapItem>();
    }
    if position != -(1 as c_int) && position != 0 as c_int {
        return ::core::ptr::null_mut::<xmlNsMapItem>();
    }
    map = *nsmap;
    if map.is_null() {
        map = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlNsMap>() as size_t
        ) as xmlNsMapPtr;
        if map.is_null() {
            xmlTreeErrMemory(
                b"allocating namespace map\0" as *const u8 as *const c_char,
            );
            return ::core::ptr::null_mut::<xmlNsMapItem>();
        }
        memset(
            map as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<xmlNsMap>() as size_t,
        );
        *nsmap = map;
    }
    if !(*map).pool.is_null() {
        ret = (*map).pool;
        (*map).pool = (*ret).next;
        memset(
            ret as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<xmlNsMapItem>() as size_t,
        );
    } else {
        ret = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlNsMapItem>() as size_t
        ) as xmlNsMapItemPtr;
        if ret.is_null() {
            xmlTreeErrMemory(
                b"allocating namespace map item\0" as *const u8 as *const c_char,
            );
            return ::core::ptr::null_mut::<xmlNsMapItem>();
        }
        memset(
            ret as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<xmlNsMapItem>() as size_t,
        );
    }
    if (*map).first.is_null() {
        (*map).first = ret;
        (*map).last = ret;
    } else if position == -(1 as c_int) {
        (*ret).prev = (*map).last;
        (*(*map).last).next = ret;
        (*map).last = ret;
    } else if position == 0 as c_int {
        (*(*map).first).prev = ret;
        (*ret).next = (*map).first;
        (*map).first = ret;
    }
    (*ret).oldNs = oldNs;
    (*ret).newNs = newNs;
    (*ret).shadowDepth = -(1 as c_int);
    (*ret).depth = depth;
    return ret;
}
unsafe fn xmlDOMWrapStoreNs(
    mut doc: xmlDocPtr,
    mut nsName: *const xmlChar,
    mut prefix: *const xmlChar,
) -> xmlNsPtr {
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    ns = xmlTreeEnsureXMLDecl(doc);
    if ns.is_null() {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    if !(*ns).next.is_null() {
        ns = (*ns).next as xmlNsPtr;
        while !ns.is_null() {
            if ((*ns).prefix == prefix || xmlStrEqual((*ns).prefix, prefix) != 0)
                && xmlStrEqual((*ns).href, nsName) != 0
            {
                return ns;
            }
            if (*ns).next.is_null() {
                break;
            }
            ns = (*ns).next as xmlNsPtr;
        }
    }
    if !ns.is_null() {
        (*ns).next = xmlNewNs(::core::ptr::null_mut::<xmlNode>(), nsName, prefix) as *mut _xmlNs;
        return (*ns).next as xmlNsPtr;
    }
    return ::core::ptr::null_mut::<xmlNs>();
}
#[inline]
pub fn xmlDOMWrapNewCtxt() -> xmlDOMWrapCtxtPtr { unsafe {
    let mut ret: xmlDOMWrapCtxtPtr = ::core::ptr::null_mut::<xmlDOMWrapCtxt>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlDOMWrapCtxt>() as size_t
    ) as xmlDOMWrapCtxtPtr;
    if ret.is_null() {
        xmlTreeErrMemory(
            b"allocating DOM-wrapper context\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlDOMWrapCtxt>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlDOMWrapCtxt>() as size_t,
    );
    return ret;
} }
#[inline]
pub fn xmlDOMWrapFreeCtxt(mut ctxt: xmlDOMWrapCtxtPtr) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if !(*ctxt).namespaceMap.is_null() {
        xmlDOMWrapNsMapFree((*ctxt).namespaceMap as xmlNsMapPtr);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut c_void);
} }
unsafe fn xmlTreeNSListLookupByPrefix(
    mut nsList: xmlNsPtr,
    mut prefix: *const xmlChar,
) -> xmlNsPtr {
    if nsList.is_null() {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    ns = nsList;
    loop {
        if prefix == (*ns).prefix || xmlStrEqual(prefix, (*ns).prefix) != 0 {
            return ns;
        }
        ns = (*ns).next as xmlNsPtr;
        if ns.is_null() {
            break;
        }
    }
    return ::core::ptr::null_mut::<xmlNs>();
}
unsafe fn xmlDOMWrapNSNormGatherInScopeNs(
    mut map: *mut xmlNsMapPtr,
    mut node: xmlNodePtr,
) -> c_int {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut mi: xmlNsMapItemPtr = ::core::ptr::null_mut::<xmlNsMapItem>();
    let mut shadowed: c_int = 0;
    if map.is_null() || !(*map).is_null() {
        return -(1 as c_int);
    }
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return -(1 as c_int);
    }
    cur = node;
    while !cur.is_null() && cur != (*cur).doc as xmlNodePtr {
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            if !(*cur).nsDef.is_null() {
                ns = (*cur).nsDef as xmlNsPtr;
                loop {
                    shadowed = 0 as c_int;
                    if !(*map).is_null() && !(**map).first.is_null() {
                        mi = (**map).first;
                        while !mi.is_null() {
                            if (*ns).prefix == (*(*mi).newNs).prefix
                                || xmlStrEqual((*ns).prefix, (*(*mi).newNs).prefix) != 0
                            {
                                shadowed = 1 as c_int;
                                break;
                            } else {
                                mi = (*mi).next;
                            }
                        }
                    }
                    mi = xmlDOMWrapNsMapAddItem(
                        map,
                        0 as c_int,
                        ::core::ptr::null_mut::<xmlNs>(),
                        ns,
                        XML_TREE_NSMAP_PARENT,
                    );
                    if mi.is_null() {
                        return -(1 as c_int);
                    }
                    if shadowed != 0 {
                        (*mi).shadowDepth = 0 as c_int;
                    }
                    ns = (*ns).next as xmlNsPtr;
                    if ns.is_null() {
                        break;
                    }
                }
            }
        }
        cur = (*cur).parent as xmlNodePtr;
    }
    return 0 as c_int;
}
unsafe fn xmlDOMWrapNSNormAddNsMapItem2(
    mut list: *mut *mut xmlNsPtr,
    mut size: *mut c_int,
    mut number: *mut c_int,
    mut oldNs: xmlNsPtr,
    mut newNs: xmlNsPtr,
) -> c_int {
    if (*list).is_null() {
        *list = xmlMalloc.expect("non-null function pointer")(
            (6 as size_t).wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
        ) as *mut xmlNsPtr;
        if (*list).is_null() {
            xmlTreeErrMemory(b"alloc ns map item\0" as *const u8 as *const c_char);
            return -(1 as c_int);
        }
        *size = 3 as c_int;
        *number = 0 as c_int;
    } else if *number >= *size {
        *size *= 2 as c_int;
        *list = xmlRealloc.expect("non-null function pointer")(
            *list as *mut c_void,
            ((*size * 2 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
        ) as *mut xmlNsPtr;
        if (*list).is_null() {
            xmlTreeErrMemory(b"realloc ns map item\0" as *const u8 as *const c_char);
            return -(1 as c_int);
        }
    }
    let ref mut fresh12 = *(*list).offset((2 as c_int * *number) as isize);
    *fresh12 = oldNs;
    let ref mut fresh13 =
        *(*list).offset((2 as c_int * *number + 1 as c_int) as isize);
    *fresh13 = newNs;
    *number += 1;
    return 0 as c_int;
}
#[inline]
pub fn xmlDOMWrapRemoveNode(
    mut ctxt: xmlDOMWrapCtxtPtr,
    mut doc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut options: c_int,
) -> c_int { unsafe {
    let mut current_block: u64;
    let mut list: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
    let mut sizeList: c_int = 0;
    let mut nbList: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if node.is_null() || doc.is_null() || (*node).doc != doc {
        return -(1 as c_int);
    }
    if (*node).parent.is_null() {
        return 0 as c_int;
    }
    match (*node).type_0 as c_uint {
        3 | 4 | 5 | 7 | 8 => {
            xmlUnlinkNode(node);
            return 0 as c_int;
        }
        1 | 2 => {}
        _ => return 1 as c_int,
    }
    xmlUnlinkNode(node);
    's_57: loop {
        match (*node).type_0 as c_uint {
            1 => {
                if ctxt.is_null() && !(*node).nsDef.is_null() {
                    ns = (*node).nsDef as xmlNsPtr;
                    loop {
                        if xmlDOMWrapNSNormAddNsMapItem2(
                            &raw mut list,
                            &raw mut sizeList,
                            &raw mut nbList,
                            ns,
                            ns,
                        ) == -(1 as c_int)
                        {
                            current_block = 10936373222305415628;
                            break 's_57;
                        }
                        ns = (*ns).next as xmlNsPtr;
                        if ns.is_null() {
                            break;
                        }
                    }
                    current_block = 7492746457666006258;
                } else {
                    current_block = 7492746457666006258;
                }
            }
            2 => {
                current_block = 7492746457666006258;
            }
            _ => {
                current_block = 15302305935079075729;
            }
        }
        match current_block {
            7492746457666006258 => {
                if !(*node).ns.is_null() {
                    if !list.is_null() {
                        i = 0 as c_int;
                        j = 0 as c_int;
                        loop {
                            if !(i < nbList) {
                                current_block = 11194104282611034094;
                                break;
                            }
                            if (*node).ns == *list.offset(j as isize) {
                                j += 1;
                                (*node).ns = *list.offset(j as isize) as *mut xmlNs;
                                current_block = 7220920915309061855;
                                break;
                            } else {
                                i += 1;
                                j += 2 as c_int;
                            }
                        }
                    } else {
                        current_block = 11194104282611034094;
                    }
                    match current_block {
                        7220920915309061855 => {}
                        _ => {
                            ns = ::core::ptr::null_mut::<xmlNs>();
                            if ctxt.is_null() {
                                ns = xmlDOMWrapStoreNs(
                                    doc,
                                    (*(*node).ns).href,
                                    (*(*node).ns).prefix,
                                );
                                if ns.is_null() {
                                    current_block = 10936373222305415628;
                                    break;
                                }
                            }
                            if !ns.is_null() {
                                if xmlDOMWrapNSNormAddNsMapItem2(
                                    &raw mut list,
                                    &raw mut sizeList,
                                    &raw mut nbList,
                                    (*node).ns as xmlNsPtr,
                                    ns,
                                ) == -(1 as c_int)
                                {
                                    current_block = 10936373222305415628;
                                    break;
                                }
                            }
                            (*node).ns = ns as *mut xmlNs;
                            current_block = 1538046216550696469;
                        }
                    }
                } else {
                    current_block = 1538046216550696469;
                }
                match current_block {
                    1538046216550696469 => {
                        if (*node).type_0 as c_uint
                            == XML_ELEMENT_NODE as c_int as c_uint
                            && !(*node).properties.is_null()
                        {
                            node = (*node).properties as xmlNodePtr;
                            current_block = 11650488183268122163;
                        } else {
                            current_block = 7220920915309061855;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    11650488183268122163 => {}
                    _ => {
                        if (*node).type_0 as c_uint
                            == XML_ELEMENT_NODE as c_int as c_uint
                            && !(*node).children.is_null()
                        {
                            node = (*node).children as xmlNodePtr;
                            current_block = 11650488183268122163;
                        } else {
                            current_block = 15302305935079075729;
                        }
                    }
                }
            }
            _ => {}
        }
        loop {
            match current_block {
                11650488183268122163 => {
                    if !node.is_null() {
                        break;
                    } else {
                        current_block = 790185930182612747;
                        break 's_57;
                    }
                }
                _ => {
                    if node.is_null() {
                        current_block = 790185930182612747;
                        break 's_57;
                    }
                    if !(*node).next.is_null() {
                        node = (*node).next as xmlNodePtr;
                        current_block = 11650488183268122163;
                    } else {
                        node = (*node).parent as xmlNodePtr;
                        current_block = 15302305935079075729;
                    }
                }
            }
        }
    }
    match current_block {
        10936373222305415628 => {
            if !list.is_null() {
                xmlFree.expect("non-null function pointer")(list as *mut c_void);
            }
            return -(1 as c_int);
        }
        _ => {
            if !list.is_null() {
                xmlFree.expect("non-null function pointer")(list as *mut c_void);
            }
            return 0 as c_int;
        }
    };
} }
unsafe fn xmlSearchNsByNamespaceStrict(
    mut doc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut nsName: *const xmlChar,
    mut retNs: *mut xmlNsPtr,
    mut prefixed: c_int,
) -> c_int {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut prev: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut out: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut prevns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if doc.is_null() || nsName.is_null() || retNs.is_null() {
        return -(1 as c_int);
    }
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return -(1 as c_int);
    }
    *retNs = ::core::ptr::null_mut::<xmlNs>();
    if xmlStrEqual(nsName, XML_XML_NAMESPACE) != 0 {
        *retNs = xmlTreeEnsureXMLDecl(doc);
        if (*retNs).is_null() {
            return -(1 as c_int);
        }
        return 1 as c_int;
    }
    cur = node;
    loop {
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            if !(*cur).nsDef.is_null() {
                let mut current_block_20: u64;
                ns = (*cur).nsDef as xmlNsPtr;
                while !ns.is_null() {
                    if !(prefixed != 0 && (*ns).prefix.is_null()) {
                        if !prev.is_null() {
                            prevns = (*prev).nsDef as xmlNsPtr;
                            while !((*prevns).prefix == (*ns).prefix
                                || !(*prevns).prefix.is_null()
                                    && !(*ns).prefix.is_null()
                                    && xmlStrEqual((*prevns).prefix, (*ns).prefix) != 0)
                            {
                                prevns = (*prevns).next as xmlNsPtr;
                                if prevns.is_null() {
                                    break;
                                }
                            }
                            if !prevns.is_null() {
                                current_block_20 = 2968425633554183086;
                            } else {
                                current_block_20 = 26972500619410423;
                            }
                        } else {
                            current_block_20 = 26972500619410423;
                        }
                        match current_block_20 {
                            2968425633554183086 => {}
                            _ => {
                                if nsName == (*ns).href || xmlStrEqual(nsName, (*ns).href) != 0 {
                                    if !out.is_null() {
                                        let mut ret: c_int = 0;
                                        ret = xmlNsInScope(doc, node, prev, (*ns).prefix);
                                        if ret < 0 as c_int {
                                            return -(1 as c_int);
                                        }
                                        if ret == 0 {
                                            current_block_20 = 2968425633554183086;
                                        } else {
                                            current_block_20 = 18386322304582297246;
                                        }
                                    } else {
                                        current_block_20 = 18386322304582297246;
                                    }
                                    match current_block_20 {
                                        2968425633554183086 => {}
                                        _ => {
                                            *retNs = ns;
                                            return 1 as c_int;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ns = (*ns).next as xmlNsPtr;
                }
                out = prev;
                prev = cur;
            }
        } else if (*cur).type_0 as c_uint
            == XML_ENTITY_NODE as c_int as c_uint
            || (*cur).type_0 as c_uint
                == XML_ENTITY_DECL as c_int as c_uint
        {
            return 0 as c_int;
        }
        cur = (*cur).parent as xmlNodePtr;
        if !(!cur.is_null() && (*cur).doc != cur as xmlDocPtr) {
            break;
        }
    }
    return 0 as c_int;
}
unsafe fn xmlSearchNsByPrefixStrict(
    mut doc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut prefix: *const xmlChar,
    mut retNs: *mut xmlNsPtr,
) -> c_int {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if doc.is_null()
        || node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return -(1 as c_int);
    }
    if !retNs.is_null() {
        *retNs = ::core::ptr::null_mut::<xmlNs>();
    }
    if !prefix.is_null()
        && *prefix.offset(0 as c_int as isize) as c_int == 'x' as i32
        && *prefix.offset(1 as c_int as isize) as c_int == 'm' as i32
        && *prefix.offset(2 as c_int as isize) as c_int == 'l' as i32
        && *prefix.offset(3 as c_int as isize) as c_int
            == 0 as c_int
    {
        if !retNs.is_null() {
            *retNs = xmlTreeEnsureXMLDecl(doc);
            if (*retNs).is_null() {
                return -(1 as c_int);
            }
        }
        return 1 as c_int;
    }
    cur = node;
    loop {
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            if !(*cur).nsDef.is_null() {
                ns = (*cur).nsDef as xmlNsPtr;
                loop {
                    if prefix == (*ns).prefix || xmlStrEqual(prefix, (*ns).prefix) != 0 {
                        if (*ns).href.is_null() {
                            return 0 as c_int;
                        }
                        if !retNs.is_null() {
                            *retNs = ns;
                        }
                        return 1 as c_int;
                    }
                    ns = (*ns).next as xmlNsPtr;
                    if ns.is_null() {
                        break;
                    }
                }
            }
        } else if (*cur).type_0 as c_uint
            == XML_ENTITY_NODE as c_int as c_uint
            || (*cur).type_0 as c_uint
                == XML_ENTITY_DECL as c_int as c_uint
        {
            return 0 as c_int;
        }
        cur = (*cur).parent as xmlNodePtr;
        if !(!cur.is_null() && (*cur).doc != cur as xmlDocPtr) {
            break;
        }
    }
    return 0 as c_int;
}
unsafe fn xmlDOMWrapNSNormDeclareNsForced(
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut nsName: *const xmlChar,
    mut prefix: *const xmlChar,
    mut checkShadow: c_int,
) -> xmlNsPtr {
    let mut current_block: u64;
    let mut ret: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut buf: [c_char; 50] = [0; 50];
    let mut pref: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut counter: c_int = 0 as c_int;
    if doc.is_null()
        || elem.is_null()
        || (*elem).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNs>();
    }
    pref = prefix;
    loop {
        if !(!(*elem).nsDef.is_null()
            && !xmlTreeNSListLookupByPrefix((*elem).nsDef as xmlNsPtr, pref).is_null())
        {
            if checkShadow != 0
                && !(*elem).parent.is_null()
                && (*(*elem).parent).doc as xmlNodePtr != (*elem).parent
            {
                if xmlSearchNsByPrefixStrict(
                    doc,
                    (*elem).parent as xmlNodePtr,
                    pref,
                    ::core::ptr::null_mut::<xmlNsPtr>(),
                ) == 1 as c_int
                {
                    current_block = 18207144653218018988;
                } else {
                    current_block = 7815301370352969686;
                }
            } else {
                current_block = 7815301370352969686;
            }
            match current_block {
                18207144653218018988 => {}
                _ => {
                    ret = xmlNewNs(::core::ptr::null_mut::<xmlNode>(), nsName, pref);
                    if ret.is_null() {
                        return ::core::ptr::null_mut::<xmlNs>();
                    }
                    if (*elem).nsDef.is_null() {
                        (*elem).nsDef = ret as *mut xmlNs;
                    } else {
                        let mut ns2: xmlNsPtr = (*elem).nsDef as xmlNsPtr;
                        while !(*ns2).next.is_null() {
                            ns2 = (*ns2).next as xmlNsPtr;
                        }
                        (*ns2).next = ret as *mut _xmlNs;
                    }
                    return ret;
                }
            }
        }
        counter += 1;
        if counter > 1000 as c_int {
            return ::core::ptr::null_mut::<xmlNs>();
        }
        if prefix.is_null() {
            snprintf(
                &raw mut buf as *mut c_char,
                ::core::mem::size_of::<[c_char; 50]>() as size_t,
                b"ns_%d\0" as *const u8 as *const c_char,
                counter,
            );
        } else {
            snprintf(
                &raw mut buf as *mut c_char,
                ::core::mem::size_of::<[c_char; 50]>() as size_t,
                b"%.30s_%d\0" as *const u8 as *const c_char,
                prefix as *mut c_char,
                counter,
            );
        }
        pref = &raw mut buf as *mut c_char as *mut xmlChar;
    }
}
unsafe fn xmlDOMWrapNSNormAcquireNormalizedNs(
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut ns: xmlNsPtr,
    mut retNs: *mut xmlNsPtr,
    mut nsMap: *mut xmlNsMapPtr,
    mut depth: c_int,
    mut ancestorsOnly: c_int,
    mut prefixed: c_int,
) -> c_int {
    let mut mi: xmlNsMapItemPtr = ::core::ptr::null_mut::<xmlNsMapItem>();
    if doc.is_null() || ns.is_null() || retNs.is_null() || nsMap.is_null() {
        return -(1 as c_int);
    }
    *retNs = ::core::ptr::null_mut::<xmlNs>();
    if !(*ns).prefix.is_null()
        && *(*ns).prefix.offset(0 as c_int as isize) as c_int
            == 'x' as i32
        && *(*ns).prefix.offset(1 as c_int as isize) as c_int
            == 'm' as i32
        && *(*ns).prefix.offset(2 as c_int as isize) as c_int
            == 'l' as i32
        && *(*ns).prefix.offset(3 as c_int as isize) as c_int
            == 0 as c_int
    {
        *retNs = xmlTreeEnsureXMLDecl(doc);
        if (*retNs).is_null() {
            return -(1 as c_int);
        }
        return 0 as c_int;
    }
    if !(*nsMap).is_null() && !(**nsMap).first.is_null() && !(ancestorsOnly != 0 && elem.is_null())
    {
        mi = (**nsMap).first;
        while !mi.is_null() {
            if (*mi).depth >= XML_TREE_NSMAP_PARENT
                && (ancestorsOnly == 0 || (*mi).depth == XML_TREE_NSMAP_PARENT)
                && (*mi).shadowDepth == -(1 as c_int)
                && (!(*(*mi).newNs).href.is_null()
                    && *(*(*mi).newNs).href.offset(0 as c_int as isize)
                        as c_int
                        != 0 as c_int)
                && (prefixed == 0 || !(*(*mi).newNs).prefix.is_null())
                && ((*(*mi).newNs).href == (*ns).href
                    || xmlStrEqual((*(*mi).newNs).href, (*ns).href) != 0)
            {
                (*mi).oldNs = ns;
                *retNs = (*mi).newNs;
                return 0 as c_int;
            }
            mi = (*mi).next;
        }
    }
    if elem.is_null() {
        let mut tmpns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
        tmpns = xmlDOMWrapStoreNs(doc, (*ns).href, (*ns).prefix);
        if tmpns.is_null() {
            return -(1 as c_int);
        }
        if xmlDOMWrapNsMapAddItem(
            nsMap,
            -(1 as c_int),
            ns,
            tmpns,
            XML_TREE_NSMAP_DOC,
        )
        .is_null()
        {
            xmlFreeNs(tmpns);
            return -(1 as c_int);
        }
        *retNs = tmpns;
    } else {
        let mut tmpns_0: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
        tmpns_0 = xmlDOMWrapNSNormDeclareNsForced(
            doc,
            elem,
            (*ns).href,
            (*ns).prefix,
            0 as c_int,
        );
        if tmpns_0.is_null() {
            return -(1 as c_int);
        }
        if !(*nsMap).is_null() {
            mi = (**nsMap).first;
            while !mi.is_null() {
                if (*mi).depth < depth
                    && (*mi).shadowDepth == -(1 as c_int)
                    && ((*ns).prefix == (*(*mi).newNs).prefix
                        || xmlStrEqual((*ns).prefix, (*(*mi).newNs).prefix) != 0)
                {
                    (*mi).shadowDepth = depth;
                    break;
                } else {
                    mi = (*mi).next;
                }
            }
        }
        if xmlDOMWrapNsMapAddItem(nsMap, -(1 as c_int), ns, tmpns_0, depth).is_null() {
            xmlFreeNs(tmpns_0);
            return -(1 as c_int);
        }
        *retNs = tmpns_0;
    }
    return 0 as c_int;
}
#[inline]
pub fn xmlDOMWrapReconcileNamespaces(
    mut ctxt: xmlDOMWrapCtxtPtr,
    mut elem: xmlNodePtr,
    mut options: c_int,
) -> c_int { unsafe {
    let mut current_block: u64;
    let mut depth: c_int = -(1 as c_int);
    let mut adoptns: c_int = 0 as c_int;
    let mut parnsdone: c_int = 0 as c_int;
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut prevns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut curElem: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut nsMap: xmlNsMapPtr = ::core::ptr::null_mut::<xmlNsMap>();
    let mut mi: xmlNsMapItemPtr = ::core::ptr::null_mut::<xmlNsMapItem>();
    let mut ancestorsOnly: c_int = 0 as c_int;
    let mut optRemoveRedundantNS: c_int = if options as xmlDOMReconcileNSOptions
        as c_uint
        & XML_DOM_RECONNS_REMOVEREDUND as c_int as c_uint
        != 0
    {
        1 as c_int
    } else {
        0 as c_int
    };
    let mut listRedund: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
    let mut sizeRedund: c_int = 0 as c_int;
    let mut nbRedund: c_int = 0 as c_int;
    let mut ret: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    if elem.is_null()
        || (*elem).doc.is_null()
        || (*elem).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return -(1 as c_int);
    }
    doc = (*elem).doc as xmlDocPtr;
    cur = elem;
    's_37: loop {
        match (*cur).type_0 as c_uint {
            1 => {
                adoptns = 1 as c_int;
                curElem = cur;
                depth += 1;
                if !(*cur).nsDef.is_null() {
                    prevns = ::core::ptr::null_mut::<xmlNs>();
                    ns = (*cur).nsDef as xmlNsPtr;
                    while !ns.is_null() {
                        if parnsdone == 0 {
                            if !(*elem).parent.is_null()
                                && (*(*elem).parent).doc as xmlNodePtr != (*elem).parent
                            {
                                if xmlDOMWrapNSNormGatherInScopeNs(
                                    &raw mut nsMap,
                                    (*elem).parent as xmlNodePtr,
                                ) == -(1 as c_int)
                                {
                                    current_block = 2374863394780976037;
                                    break 's_37;
                                }
                            }
                            parnsdone = 1 as c_int;
                        }
                        if optRemoveRedundantNS != 0
                            && (!nsMap.is_null() && !(*nsMap).first.is_null())
                        {
                            mi = (*nsMap).first;
                            loop {
                                if mi.is_null() {
                                    current_block = 18386322304582297246;
                                    break;
                                }
                                if (*mi).depth >= XML_TREE_NSMAP_PARENT
                                    && (*mi).shadowDepth == -(1 as c_int)
                                    && ((*ns).prefix == (*(*mi).newNs).prefix
                                        || xmlStrEqual((*ns).prefix, (*(*mi).newNs).prefix) != 0)
                                    && ((*ns).href == (*(*mi).newNs).href
                                        || xmlStrEqual((*ns).href, (*(*mi).newNs).href) != 0)
                                {
                                    if xmlDOMWrapNSNormAddNsMapItem2(
                                        &raw mut listRedund,
                                        &raw mut sizeRedund,
                                        &raw mut nbRedund,
                                        ns,
                                        (*mi).newNs,
                                    ) == -(1 as c_int)
                                    {
                                        current_block = 2374863394780976037;
                                        break 's_37;
                                    }
                                    if !prevns.is_null() {
                                        (*prevns).next = (*ns).next;
                                    } else {
                                        (*cur).nsDef = (*ns).next as *mut xmlNs;
                                    }
                                    current_block = 4642361641053693309;
                                    break;
                                } else {
                                    mi = (*mi).next;
                                }
                            }
                        } else {
                            current_block = 18386322304582297246;
                        }
                        match current_block {
                            18386322304582297246 => {
                                if !(*cur).ns.is_null() && adoptns != 0 && (*cur).ns == ns {
                                    adoptns = 0 as c_int;
                                }
                                if !nsMap.is_null() && !(*nsMap).first.is_null() {
                                    mi = (*nsMap).first;
                                    while !mi.is_null() {
                                        if (*mi).depth >= XML_TREE_NSMAP_PARENT
                                            && (*mi).shadowDepth == -(1 as c_int)
                                            && ((*ns).prefix == (*(*mi).newNs).prefix
                                                || xmlStrEqual((*ns).prefix, (*(*mi).newNs).prefix)
                                                    != 0)
                                        {
                                            (*mi).shadowDepth = depth;
                                        }
                                        mi = (*mi).next;
                                    }
                                }
                                if xmlDOMWrapNsMapAddItem(
                                    &raw mut nsMap,
                                    -(1 as c_int),
                                    ns,
                                    ns,
                                    depth,
                                )
                                .is_null()
                                {
                                    current_block = 2374863394780976037;
                                    break 's_37;
                                }
                                prevns = ns;
                            }
                            _ => {}
                        }
                        ns = (*ns).next as xmlNsPtr;
                    }
                }
                if adoptns == 0 {
                    current_block = 12565033917272123027;
                } else {
                    current_block = 12787836519440156046;
                }
            }
            2 => {
                current_block = 12787836519440156046;
            }
            _ => {
                current_block = 4621179585644257154;
            }
        }
        match current_block {
            12787836519440156046 => {
                if (*cur).ns.is_null() {
                    current_block = 12565033917272123027;
                } else {
                    if parnsdone == 0 {
                        if !(*elem).parent.is_null()
                            && (*(*elem).parent).doc as xmlNodePtr != (*elem).parent
                        {
                            if xmlDOMWrapNSNormGatherInScopeNs(
                                &raw mut nsMap,
                                (*elem).parent as xmlNodePtr,
                            ) == -(1 as c_int)
                            {
                                current_block = 2374863394780976037;
                                break;
                            }
                        }
                        parnsdone = 1 as c_int;
                    }
                    if !listRedund.is_null() {
                        i = 0 as c_int;
                        j = 0 as c_int;
                        while i < nbRedund {
                            if (*cur).ns == *listRedund.offset(j as isize) {
                                j += 1;
                                (*cur).ns = *listRedund.offset(j as isize) as *mut xmlNs;
                                break;
                            } else {
                                i += 1;
                                j += 2 as c_int;
                            }
                        }
                    }
                    if !nsMap.is_null() && !(*nsMap).first.is_null() {
                        mi = (*nsMap).first;
                        loop {
                            if mi.is_null() {
                                current_block = 9241535491006583629;
                                break;
                            }
                            if (*mi).shadowDepth == -(1 as c_int)
                                && (*cur).ns == (*mi).oldNs
                            {
                                (*cur).ns = (*mi).newNs as *mut xmlNs;
                                current_block = 12565033917272123027;
                                break;
                            } else {
                                mi = (*mi).next;
                            }
                        }
                    } else {
                        current_block = 9241535491006583629;
                    }
                    match current_block {
                        12565033917272123027 => {}
                        _ => {
                            if xmlDOMWrapNSNormAcquireNormalizedNs(
                                doc,
                                curElem,
                                (*cur).ns as xmlNsPtr,
                                &raw mut ns,
                                &raw mut nsMap,
                                depth,
                                ancestorsOnly,
                                (if (*cur).type_0 as c_uint
                                    == XML_ATTRIBUTE_NODE as c_int
                                        as c_uint
                                {
                                    1 as c_int
                                } else {
                                    0 as c_int
                                }),
                            ) == -(1 as c_int)
                            {
                                current_block = 2374863394780976037;
                                break;
                            }
                            (*cur).ns = ns as *mut xmlNs;
                            current_block = 12565033917272123027;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            12565033917272123027 => {
                if (*cur).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    && !(*cur).properties.is_null()
                {
                    cur = (*cur).properties as xmlNodePtr;
                    current_block = 4906268039856690917;
                } else {
                    current_block = 6702857154449209910;
                }
            }
            _ => {}
        }
        loop {
            match current_block {
                4906268039856690917 => {
                    if !cur.is_null() {
                        break;
                    } else {
                        current_block = 12705158477165241210;
                        break 's_37;
                    }
                }
                4621179585644257154 => {
                    if cur == elem {
                        current_block = 12705158477165241210;
                        break 's_37;
                    }
                    if (*cur).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                    {
                        if !nsMap.is_null() && !(*nsMap).first.is_null() {
                            while !(*nsMap).last.is_null() && (*(*nsMap).last).depth >= depth {
                                mi = (*nsMap).last;
                                (*nsMap).last = (*mi).prev;
                                if (*nsMap).last.is_null() {
                                    (*nsMap).first = ::core::ptr::null_mut::<xmlNsMapItem>();
                                } else {
                                    (*(*nsMap).last).next = ::core::ptr::null_mut::<xmlNsMapItem>();
                                }
                                (*mi).next = (*nsMap).pool;
                                (*nsMap).pool = mi;
                            }
                            mi = (*nsMap).first;
                            while !mi.is_null() {
                                if (*mi).shadowDepth >= depth {
                                    (*mi).shadowDepth = -(1 as c_int);
                                }
                                mi = (*mi).next;
                            }
                        }
                        depth -= 1;
                    }
                    if !(*cur).next.is_null() {
                        cur = (*cur).next as xmlNodePtr;
                        current_block = 4906268039856690917;
                    } else if (*cur).type_0 as c_uint
                        == XML_ATTRIBUTE_NODE as c_int as c_uint
                    {
                        cur = (*cur).parent as xmlNodePtr;
                        current_block = 6702857154449209910;
                    } else {
                        cur = (*cur).parent as xmlNodePtr;
                        current_block = 4621179585644257154;
                    }
                }
                _ => {
                    if !((*cur).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                        && !(*cur).children.is_null())
                    {
                        current_block = 4621179585644257154;
                        continue;
                    }
                    cur = (*cur).children as xmlNodePtr;
                    current_block = 4906268039856690917;
                }
            }
        }
    }
    match current_block {
        2374863394780976037 => {
            ret = -(1 as c_int);
        }
        _ => {
            ret = 0 as c_int;
        }
    }
    if !listRedund.is_null() {
        i = 0 as c_int;
        j = 0 as c_int;
        while i < nbRedund {
            xmlFreeNs(*listRedund.offset(j as isize));
            i += 1;
            j += 2 as c_int;
        }
        xmlFree.expect("non-null function pointer")(listRedund as *mut c_void);
    }
    if !nsMap.is_null() {
        xmlDOMWrapNsMapFree(nsMap);
    }
    return ret;
} }
fn xmlDOMWrapAdoptBranch(
    mut ctxt: xmlDOMWrapCtxtPtr,
    mut sourceDoc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut destDoc: xmlDocPtr,
    mut destParent: xmlNodePtr,
    mut options: c_int,
) -> c_int { unsafe {
    let mut current_block: u64;
    let mut ret: c_int = 0 as c_int;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut curElem: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut nsMap: xmlNsMapPtr = ::core::ptr::null_mut::<xmlNsMap>();
    let mut mi: xmlNsMapItemPtr = ::core::ptr::null_mut::<xmlNsMapItem>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut depth: c_int = -(1 as c_int);
    let mut adoptStr: c_int = 1 as c_int;
    let mut parnsdone: c_int = 0;
    let mut ancestorsOnly: c_int = 0 as c_int;
    if !sourceDoc.is_null() && (*sourceDoc).dict == (*destDoc).dict {
        adoptStr = 0 as c_int;
    } else {
        adoptStr = 1 as c_int;
    }
    if !ctxt.is_null() {
        nsMap = (*ctxt).namespaceMap as xmlNsMapPtr;
    }
    if destParent.is_null() || !ctxt.is_null() && (*ctxt).getNsForNodeFunc.is_some() {
        parnsdone = 1 as c_int;
    } else {
        parnsdone = 0 as c_int;
    }
    cur = node;
    if !cur.is_null()
        && (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        current_block = 5113145255723220787;
    } else {
        current_block = 7651349459974463963;
    }
    '_internal_error: loop {
        match current_block {
            5113145255723220787 => {
                ret = -(1 as c_int);
                break;
            }
            _ => {
                if cur.is_null() {
                    break;
                }
                if (*cur).doc != sourceDoc {
                    if (*cur).next.is_null() {
                        current_block = 17858191363619347262;
                    } else {
                        loop {
                            cur = (*cur).next as xmlNodePtr;
                            if (*cur).type_0 as c_uint
                                == XML_XINCLUDE_END as c_int as c_uint
                                || (*cur).doc == (*node).doc
                            {
                                break;
                            }
                            if (*cur).next.is_null() {
                                break;
                            }
                        }
                        if (*cur).doc != (*node).doc {
                            current_block = 17858191363619347262;
                        } else {
                            current_block = 7175849428784450219;
                        }
                    }
                } else {
                    current_block = 7175849428784450219;
                }
                match current_block {
                    7175849428784450219 => {
                        (*cur).doc = destDoc as *mut _xmlDoc;
                        match (*cur).type_0 as c_uint {
                            19 | 20 => return -(1 as c_int),
                            1 => {
                                curElem = cur;
                                depth += 1;
                                if !(*cur).nsDef.is_null()
                                    && (ctxt.is_null() || (*ctxt).getNsForNodeFunc.is_none())
                                {
                                    if parnsdone == 0 {
                                        if xmlDOMWrapNSNormGatherInScopeNs(
                                            &raw mut nsMap,
                                            destParent,
                                        ) == -(1 as c_int)
                                        {
                                            current_block = 5113145255723220787;
                                            continue;
                                        }
                                        parnsdone = 1 as c_int;
                                    }
                                    ns = (*cur).nsDef as xmlNsPtr;
                                    while !ns.is_null() {
                                        if !nsMap.is_null() && !(*nsMap).first.is_null() {
                                            mi = (*nsMap).first;
                                            while !mi.is_null() {
                                                if (*mi).depth >= XML_TREE_NSMAP_PARENT
                                                    && (*mi).shadowDepth
                                                        == -(1 as c_int)
                                                    && ((*ns).prefix == (*(*mi).newNs).prefix
                                                        || xmlStrEqual(
                                                            (*ns).prefix,
                                                            (*(*mi).newNs).prefix,
                                                        ) != 0)
                                                {
                                                    (*mi).shadowDepth = depth;
                                                }
                                                mi = (*mi).next;
                                            }
                                        }
                                        if xmlDOMWrapNsMapAddItem(
                                            &raw mut nsMap,
                                            -(1 as c_int),
                                            ns,
                                            ns,
                                            depth,
                                        )
                                        .is_null()
                                        {
                                            current_block = 5113145255723220787;
                                            continue '_internal_error;
                                        }
                                        ns = (*ns).next as xmlNsPtr;
                                    }
                                    current_block = 12964802341717277787;
                                } else {
                                    current_block = 12964802341717277787;
                                }
                            }
                            2 => {
                                current_block = 12964802341717277787;
                            }
                            3 | 4 => {
                                if adoptStr != 0
                                    && !(*cur).content.is_null()
                                    && !sourceDoc.is_null()
                                    && !(*sourceDoc).dict.is_null()
                                    && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*cur).content)
                                        != 0
                                {
                                    if !(*destDoc).dict.is_null() {
                                        (*cur).content = xmlDictLookup(
                                            (*destDoc).dict as xmlDictPtr,
                                            (*cur).content,
                                            -(1 as c_int),
                                        )
                                            as *mut xmlChar;
                                    } else {
                                        (*cur).content = xmlStrdup((*cur).content);
                                    }
                                }
                                current_block = 17858191363619347262;
                            }
                            5 => {
                                (*cur).content = ::core::ptr::null_mut::<xmlChar>();
                                (*cur).children = ::core::ptr::null_mut::<_xmlNode>();
                                (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
                                if !(*destDoc).intSubset.is_null()
                                    || !(*destDoc).extSubset.is_null()
                                {
                                    let mut ent: xmlEntityPtr =
                                        ::core::ptr::null_mut::<xmlEntity>();
                                    ent = xmlGetDocEntity(destDoc as *const xmlDoc, (*cur).name);
                                    if !ent.is_null() {
                                        (*cur).content = (*ent).content;
                                        (*cur).children = ent as xmlNodePtr as *mut _xmlNode;
                                        (*cur).last = ent as xmlNodePtr as *mut _xmlNode;
                                    }
                                }
                                current_block = 17858191363619347262;
                            }
                            7 => {
                                if adoptStr != 0 && !(*cur).name.is_null() {
                                    if !(*destDoc).dict.is_null() {
                                        let mut old_0: *const xmlChar = (*cur).name;
                                        (*cur).name = xmlDictLookup(
                                            (*destDoc).dict as xmlDictPtr,
                                            (*cur).name,
                                            -(1 as c_int),
                                        );
                                        if sourceDoc.is_null()
                                            || (*sourceDoc).dict.is_null()
                                            || xmlDictOwns((*sourceDoc).dict as xmlDictPtr, old_0)
                                                == 0
                                        {
                                            xmlFree.expect("non-null function pointer")(
                                                old_0 as *mut c_char
                                                    as *mut c_void,
                                            );
                                        }
                                    } else if !sourceDoc.is_null()
                                        && !(*sourceDoc).dict.is_null()
                                        && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*cur).name)
                                            != 0
                                    {
                                        (*cur).name = xmlStrdup((*cur).name);
                                    }
                                }
                                if adoptStr != 0
                                    && !(*cur).content.is_null()
                                    && !sourceDoc.is_null()
                                    && !(*sourceDoc).dict.is_null()
                                    && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*cur).content)
                                        != 0
                                {
                                    if !(*destDoc).dict.is_null() {
                                        (*cur).content = xmlDictLookup(
                                            (*destDoc).dict as xmlDictPtr,
                                            (*cur).content,
                                            -(1 as c_int),
                                        )
                                            as *mut xmlChar;
                                    } else {
                                        (*cur).content = xmlStrdup((*cur).content);
                                    }
                                }
                                current_block = 2793352396589381719;
                            }
                            8 => {
                                current_block = 2793352396589381719;
                            }
                            _ => {
                                current_block = 5113145255723220787;
                                continue;
                            }
                        }
                        match current_block {
                            17858191363619347262 => {}
                            _ => {
                                match current_block {
                                    12964802341717277787 => {
                                        if !(*cur).ns.is_null() {
                                            if parnsdone == 0 {
                                                if xmlDOMWrapNSNormGatherInScopeNs(
                                                    &raw mut nsMap,
                                                    destParent,
                                                ) == -(1 as c_int)
                                                {
                                                    current_block = 5113145255723220787;
                                                    continue;
                                                }
                                                parnsdone = 1 as c_int;
                                            }
                                            if !nsMap.is_null() && !(*nsMap).first.is_null() {
                                                mi = (*nsMap).first;
                                                loop {
                                                    if mi.is_null() {
                                                        current_block = 6476622998065200121;
                                                        break;
                                                    }
                                                    if (*mi).shadowDepth
                                                        == -(1 as c_int)
                                                        && (*cur).ns == (*mi).oldNs
                                                    {
                                                        (*cur).ns = (*mi).newNs as *mut xmlNs;
                                                        current_block = 1553292573541944674;
                                                        break;
                                                    } else {
                                                        mi = (*mi).next;
                                                    }
                                                }
                                            } else {
                                                current_block = 6476622998065200121;
                                            }
                                            match current_block {
                                                1553292573541944674 => {}
                                                _ => {
                                                    if !ctxt.is_null()
                                                        && (*ctxt).getNsForNodeFunc.is_some()
                                                    {
                                                        ns = (*ctxt)
                                                            .getNsForNodeFunc
                                                            .expect("non-null function pointer")(
                                                            ctxt,
                                                            cur,
                                                            (*(*cur).ns).href,
                                                            (*(*cur).ns).prefix,
                                                        );
                                                        if xmlDOMWrapNsMapAddItem(
                                                            &raw mut nsMap,
                                                            -(1 as c_int),
                                                            (*cur).ns as xmlNsPtr,
                                                            ns,
                                                            XML_TREE_NSMAP_CUSTOM,
                                                        )
                                                        .is_null()
                                                        {
                                                            current_block = 5113145255723220787;
                                                            continue;
                                                        }
                                                        (*cur).ns = ns as *mut xmlNs;
                                                    } else {
                                                        if xmlDOMWrapNSNormAcquireNormalizedNs(
                                                            destDoc,
                                                            (if !destParent.is_null() {
                                                                curElem
                                                            } else {
                                                                ::core::ptr::null_mut::<xmlNode>()
                                                            }),
                                                            (*cur).ns as xmlNsPtr,
                                                            &raw mut ns,
                                                            &raw mut nsMap,
                                                            depth,
                                                            ancestorsOnly,
                                                            (if (*cur).type_0 as c_uint
                                                                == XML_ATTRIBUTE_NODE
                                                                    as c_int
                                                                    as c_uint
                                                            {
                                                                1 as c_int
                                                            } else {
                                                                0 as c_int
                                                            }),
                                                        ) == -(1 as c_int)
                                                        {
                                                            current_block = 5113145255723220787;
                                                            continue;
                                                        }
                                                        (*cur).ns = ns as *mut xmlNs;
                                                    }
                                                }
                                            }
                                        }
                                        if adoptStr != 0 && !(*cur).name.is_null() {
                                            if !(*destDoc).dict.is_null() {
                                                let mut old: *const xmlChar = (*cur).name;
                                                (*cur).name = xmlDictLookup(
                                                    (*destDoc).dict as xmlDictPtr,
                                                    (*cur).name,
                                                    -(1 as c_int),
                                                );
                                                if sourceDoc.is_null()
                                                    || (*sourceDoc).dict.is_null()
                                                    || xmlDictOwns(
                                                        (*sourceDoc).dict as xmlDictPtr,
                                                        old,
                                                    ) == 0
                                                {
                                                    xmlFree.expect("non-null function pointer")(
                                                        old as *mut c_char
                                                            as *mut c_void,
                                                    );
                                                }
                                            } else if !sourceDoc.is_null()
                                                && !(*sourceDoc).dict.is_null()
                                                && xmlDictOwns(
                                                    (*sourceDoc).dict as xmlDictPtr,
                                                    (*cur).name,
                                                ) != 0
                                            {
                                                (*cur).name = xmlStrdup((*cur).name);
                                            }
                                        }
                                        if (*cur).type_0 as c_uint
                                            == XML_ELEMENT_NODE as c_int
                                                as c_uint
                                        {
                                            (*cur).psvi = NULL;
                                            (*cur).line = 0 as c_ushort;
                                            (*cur).extra = 0 as c_ushort;
                                            if !(*cur).properties.is_null() {
                                                cur = (*cur).properties as xmlNodePtr;
                                                current_block = 7651349459974463963;
                                                continue;
                                            }
                                        } else {
                                            if !sourceDoc.is_null()
                                                && (*(cur as xmlAttrPtr)).atype
                                                    as c_uint
                                                    == XML_ATTRIBUTE_ID as c_int
                                                        as c_uint
                                            {
                                                xmlRemoveID(sourceDoc, cur as xmlAttrPtr);
                                            }
                                            (*(cur as xmlAttrPtr)).atype = 0 as xmlAttributeType;
                                            let ref mut fresh14 = (*(cur as xmlAttrPtr)).psvi;
                                            *fresh14 = NULL;
                                        }
                                    }
                                    _ => {}
                                }
                                if !(*cur).children.is_null() {
                                    cur = (*cur).children as xmlNodePtr;
                                    current_block = 7651349459974463963;
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {}
                }
                loop {
                    if cur == node {
                        break '_internal_error;
                    }
                    if (*cur).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                        || (*cur).type_0 as c_uint
                            == XML_XINCLUDE_START as c_int as c_uint
                        || (*cur).type_0 as c_uint
                            == XML_XINCLUDE_END as c_int as c_uint
                    {
                        if !nsMap.is_null() && !(*nsMap).first.is_null() {
                            while !(*nsMap).last.is_null() && (*(*nsMap).last).depth >= depth {
                                mi = (*nsMap).last;
                                (*nsMap).last = (*mi).prev;
                                if (*nsMap).last.is_null() {
                                    (*nsMap).first = ::core::ptr::null_mut::<xmlNsMapItem>();
                                } else {
                                    (*(*nsMap).last).next = ::core::ptr::null_mut::<xmlNsMapItem>();
                                }
                                (*mi).next = (*nsMap).pool;
                                (*nsMap).pool = mi;
                            }
                            mi = (*nsMap).first;
                            while !mi.is_null() {
                                if (*mi).shadowDepth >= depth {
                                    (*mi).shadowDepth = -(1 as c_int);
                                }
                                mi = (*mi).next;
                            }
                        }
                        depth -= 1;
                    }
                    if !(*cur).next.is_null() {
                        cur = (*cur).next as xmlNodePtr;
                        current_block = 7651349459974463963;
                        break;
                    } else if (*cur).type_0 as c_uint
                        == XML_ATTRIBUTE_NODE as c_int as c_uint
                        && !(*(*cur).parent).children.is_null()
                    {
                        cur = (*(*cur).parent).children as xmlNodePtr;
                        current_block = 7651349459974463963;
                        break;
                    } else {
                        cur = (*cur).parent as xmlNodePtr;
                    }
                }
            }
        }
    }
    if !nsMap.is_null() {
        if !ctxt.is_null() && (*ctxt).namespaceMap == nsMap as *mut c_void {
            if !(*nsMap).first.is_null() {
                if !(*nsMap).pool.is_null() {
                    (*(*nsMap).last).next = (*nsMap).pool;
                }
                (*nsMap).pool = (*nsMap).first;
                (*nsMap).first = ::core::ptr::null_mut::<xmlNsMapItem>();
            }
        } else {
            xmlDOMWrapNsMapFree(nsMap);
        }
    }
    return ret;
} }
pub unsafe fn xmlDOMWrapCloneNode(
    mut ctxt: xmlDOMWrapCtxtPtr,
    mut sourceDoc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut resNode: *mut xmlNodePtr,
    mut destDoc: xmlDocPtr,
    mut destParent: xmlNodePtr,
    mut deep: c_int,
    mut options: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut ret: c_int = 0 as c_int;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut curElem: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut nsMap: xmlNsMapPtr = ::core::ptr::null_mut::<xmlNsMap>();
    let mut mi: xmlNsMapItemPtr = ::core::ptr::null_mut::<xmlNsMapItem>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut depth: c_int = -(1 as c_int);
    let mut parnsdone: c_int = 0 as c_int;
    let mut ancestorsOnly: c_int = 0 as c_int;
    let mut resultClone: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut clone: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parentClone: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut prevClone: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut cloneNs: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut cloneNsDefSlot: *mut xmlNsPtr = ::core::ptr::null_mut::<xmlNsPtr>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if node.is_null() || resNode.is_null() || destDoc.is_null() {
        return -(1 as c_int);
    }
    if (*node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 1 as c_int;
    }
    if !(*node).doc.is_null() && !sourceDoc.is_null() && (*node).doc != sourceDoc {
        return -(1 as c_int);
    }
    if sourceDoc.is_null() {
        sourceDoc = (*node).doc as xmlDocPtr;
    }
    if sourceDoc.is_null() {
        return -(1 as c_int);
    }
    dict = (*destDoc).dict as xmlDictPtr;
    if !ctxt.is_null() {
        nsMap = (*ctxt).namespaceMap as xmlNsMapPtr;
    }
    *resNode = ::core::ptr::null_mut::<xmlNode>();
    cur = node;
    if !cur.is_null()
        && (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return -(1 as c_int);
    }
    's_86: loop {
        if cur.is_null() {
            current_block = 14799897360614227022;
            break;
        }
        if (*cur).doc != sourceDoc {
            current_block = 14946474315211274328;
            break;
        }
        match (*cur).type_0 as c_uint {
            1 | 3 | 4 | 8 | 7 | 11 | 5 | 6 => {
                clone = xmlMalloc.expect("non-null function pointer")(
                    ::core::mem::size_of::<xmlNode>() as size_t,
                ) as xmlNodePtr;
                if clone.is_null() {
                    xmlTreeErrMemory(
                        b"xmlDOMWrapCloneNode(): allocating a node\0" as *const u8
                            as *const c_char,
                    );
                    current_block = 14946474315211274328;
                    break;
                } else {
                    memset(
                        clone as *mut c_void,
                        0 as c_int,
                        ::core::mem::size_of::<xmlNode>() as size_t,
                    );
                    if !resultClone.is_null() {
                        (*clone).parent = parentClone as *mut _xmlNode;
                        if !prevClone.is_null() {
                            (*prevClone).next = clone as *mut _xmlNode;
                            (*clone).prev = prevClone as *mut _xmlNode;
                        } else {
                            (*parentClone).children = clone as *mut _xmlNode;
                        }
                    } else {
                        resultClone = clone;
                    }
                }
            }
            2 => {
                clone = xmlRealloc.expect("non-null function pointer")(
                    NULL,
                    ::core::mem::size_of::<xmlAttr>() as size_t,
                ) as xmlNodePtr;
                if clone.is_null() {
                    xmlTreeErrMemory(
                        b"xmlDOMWrapCloneNode(): allocating an attr-node\0" as *const u8
                            as *const c_char,
                    );
                    current_block = 14946474315211274328;
                    break;
                } else {
                    memset(
                        clone as *mut c_void,
                        0 as c_int,
                        ::core::mem::size_of::<xmlAttr>() as size_t,
                    );
                    if !resultClone.is_null() {
                        (*clone).parent = parentClone as *mut _xmlNode;
                        if !prevClone.is_null() {
                            (*prevClone).next = clone as *mut _xmlNode;
                            (*clone).prev = prevClone as *mut _xmlNode;
                        } else {
                            (*parentClone).properties = clone as xmlAttrPtr as *mut _xmlAttr;
                        }
                    } else {
                        resultClone = clone;
                    }
                }
            }
            19 | 20 | _ => {
                current_block = 14946474315211274328;
                break;
            }
        }
        (*clone).type_0 = (*cur).type_0;
        (*clone).doc = destDoc as *mut _xmlDoc;
        if (*cur).name == &raw const xmlStringText as *const xmlChar {
            (*clone).name = &raw const xmlStringText as *const xmlChar;
        } else if (*cur).name == &raw const xmlStringTextNoenc as *const xmlChar {
            (*clone).name = &raw const xmlStringTextNoenc as *const xmlChar;
        } else if (*cur).name == &raw const xmlStringComment as *const xmlChar {
            (*clone).name = &raw const xmlStringComment as *const xmlChar;
        } else if !(*cur).name.is_null() {
            if !(*cur).name.is_null() {
                if !dict.is_null() {
                    if xmlDictOwns(dict, (*cur).name) != 0 {
                        (*clone).name = (*cur).name;
                    } else {
                        (*clone).name =
                            xmlDictLookup(dict, (*cur).name, -(1 as c_int));
                    }
                } else {
                    (*clone).name = xmlStrdup((*cur).name) as *const xmlChar;
                }
            }
        }
        match (*cur).type_0 as c_uint {
            19 | 20 => return -(1 as c_int),
            1 => {
                curElem = cur;
                depth += 1;
                if !(*cur).nsDef.is_null() {
                    if parnsdone == 0 {
                        if !destParent.is_null() && ctxt.is_null() {
                            if xmlDOMWrapNSNormGatherInScopeNs(&raw mut nsMap, destParent)
                                == -(1 as c_int)
                            {
                                current_block = 14946474315211274328;
                                break;
                            }
                        }
                        parnsdone = 1 as c_int;
                    }
                    cloneNsDefSlot = &raw mut (*clone).nsDef as *mut xmlNsPtr;
                    ns = (*cur).nsDef as xmlNsPtr;
                    while !ns.is_null() {
                        cloneNs =
                            xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<
                                xmlNs,
                            >(
                            )
                                as size_t) as xmlNsPtr;
                        if cloneNs.is_null() {
                            xmlTreeErrMemory(
                                b"xmlDOMWrapCloneNode(): allocating namespace\0" as *const u8
                                    as *const c_char,
                            );
                            return -(1 as c_int);
                        }
                        memset(
                            cloneNs as *mut c_void,
                            0 as c_int,
                            ::core::mem::size_of::<xmlNs>() as size_t,
                        );
                        (*cloneNs).type_0 = XML_NAMESPACE_DECL;
                        if !(*ns).href.is_null() {
                            (*cloneNs).href = xmlStrdup((*ns).href);
                        }
                        if !(*ns).prefix.is_null() {
                            (*cloneNs).prefix = xmlStrdup((*ns).prefix);
                        }
                        *cloneNsDefSlot = cloneNs;
                        cloneNsDefSlot = &raw mut (*cloneNs).next as *mut xmlNsPtr;
                        if ctxt.is_null() || (*ctxt).getNsForNodeFunc.is_none() {
                            if !nsMap.is_null() && !(*nsMap).first.is_null() {
                                mi = (*nsMap).first;
                                while !mi.is_null() {
                                    if (*mi).depth >= XML_TREE_NSMAP_PARENT
                                        && (*mi).shadowDepth == -(1 as c_int)
                                        && ((*ns).prefix == (*(*mi).newNs).prefix
                                            || xmlStrEqual((*ns).prefix, (*(*mi).newNs).prefix)
                                                != 0)
                                    {
                                        (*mi).shadowDepth = depth;
                                    }
                                    mi = (*mi).next;
                                }
                            }
                            if xmlDOMWrapNsMapAddItem(
                                &raw mut nsMap,
                                -(1 as c_int),
                                ns,
                                cloneNs,
                                depth,
                            )
                            .is_null()
                            {
                                current_block = 14946474315211274328;
                                break 's_86;
                            }
                        }
                        ns = (*ns).next as xmlNsPtr;
                    }
                    current_block = 10945915984064580713;
                } else {
                    current_block = 10945915984064580713;
                }
            }
            2 => {
                current_block = 10945915984064580713;
            }
            3 | 4 => {
                if !(*cur).content.is_null() {
                    if !dict.is_null() {
                        if xmlDictOwns(dict, (*cur).content as *const xmlChar) != 0 {
                            (*clone).content = (*cur).content;
                        } else {
                            (*clone).content = xmlDictLookup(
                                dict,
                                (*cur).content as *const xmlChar,
                                -(1 as c_int),
                            ) as *mut xmlChar;
                        }
                    } else {
                        (*clone).content = xmlStrdup((*cur).content as *const xmlChar);
                    }
                }
                current_block = 4707564613567208506;
            }
            6 => {
                current_block = 4707564613567208506;
            }
            5 => {
                if sourceDoc != destDoc {
                    if !(*destDoc).intSubset.is_null() || !(*destDoc).extSubset.is_null() {
                        let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
                        ent = xmlGetDocEntity(destDoc as *const xmlDoc, (*cur).name);
                        if !ent.is_null() {
                            (*clone).content = (*ent).content;
                            (*clone).children = ent as xmlNodePtr as *mut _xmlNode;
                            (*clone).last = ent as xmlNodePtr as *mut _xmlNode;
                        }
                    }
                } else {
                    (*clone).content = (*cur).content;
                    (*clone).children = (*cur).children;
                    (*clone).last = (*cur).last;
                }
                current_block = 4707564613567208506;
            }
            7 => {
                if !(*cur).content.is_null() {
                    if !dict.is_null() {
                        if xmlDictOwns(dict, (*cur).content as *const xmlChar) != 0 {
                            (*clone).content = (*cur).content;
                        } else {
                            (*clone).content = xmlDictLookup(
                                dict,
                                (*cur).content as *const xmlChar,
                                -(1 as c_int),
                            ) as *mut xmlChar;
                        }
                    } else {
                        (*clone).content = xmlStrdup((*cur).content as *const xmlChar);
                    }
                }
                current_block = 4707564613567208506;
            }
            8 => {
                if !(*cur).content.is_null() {
                    if !dict.is_null() {
                        if xmlDictOwns(dict, (*cur).content as *const xmlChar) != 0 {
                            (*clone).content = (*cur).content;
                        } else {
                            (*clone).content = xmlDictLookup(
                                dict,
                                (*cur).content as *const xmlChar,
                                -(1 as c_int),
                            ) as *mut xmlChar;
                        }
                    } else {
                        (*clone).content = xmlStrdup((*cur).content as *const xmlChar);
                    }
                }
                current_block = 4707564613567208506;
            }
            _ => {
                current_block = 14946474315211274328;
                break;
            }
        }
        match current_block {
            10945915984064580713 => {
                if !(*cur).ns.is_null() {
                    if parnsdone == 0 {
                        if !destParent.is_null() && ctxt.is_null() {
                            if xmlDOMWrapNSNormGatherInScopeNs(&raw mut nsMap, destParent)
                                == -(1 as c_int)
                            {
                                current_block = 14946474315211274328;
                                break;
                            }
                        }
                        parnsdone = 1 as c_int;
                    }
                    if !nsMap.is_null() && !(*nsMap).first.is_null() {
                        mi = (*nsMap).first;
                        loop {
                            if mi.is_null() {
                                current_block = 11227437541145425351;
                                break;
                            }
                            if (*mi).shadowDepth == -(1 as c_int)
                                && (*cur).ns == (*mi).oldNs
                            {
                                (*clone).ns = (*mi).newNs as *mut xmlNs;
                                current_block = 1095045285004442024;
                                break;
                            } else {
                                mi = (*mi).next;
                            }
                        }
                    } else {
                        current_block = 11227437541145425351;
                    }
                    match current_block {
                        1095045285004442024 => {}
                        _ => {
                            if !ctxt.is_null() && (*ctxt).getNsForNodeFunc.is_some() {
                                ns = (*ctxt).getNsForNodeFunc.expect("non-null function pointer")(
                                    ctxt,
                                    cur,
                                    (*(*cur).ns).href,
                                    (*(*cur).ns).prefix,
                                );
                                if xmlDOMWrapNsMapAddItem(
                                    &raw mut nsMap,
                                    -(1 as c_int),
                                    (*cur).ns as xmlNsPtr,
                                    ns,
                                    XML_TREE_NSMAP_CUSTOM,
                                )
                                .is_null()
                                {
                                    current_block = 14946474315211274328;
                                    break;
                                }
                                (*clone).ns = ns as *mut xmlNs;
                            } else {
                                if xmlDOMWrapNSNormAcquireNormalizedNs(
                                    destDoc,
                                    (if !destParent.is_null() {
                                        curElem
                                    } else {
                                        ::core::ptr::null_mut::<xmlNode>()
                                    }),
                                    (*cur).ns as xmlNsPtr,
                                    &raw mut ns,
                                    &raw mut nsMap,
                                    depth,
                                    ancestorsOnly,
                                    (if (*cur).type_0 as c_uint
                                        == XML_ATTRIBUTE_NODE as c_int
                                            as c_uint
                                    {
                                        1 as c_int
                                    } else {
                                        0 as c_int
                                    }),
                                ) == -(1 as c_int)
                                {
                                    current_block = 14946474315211274328;
                                    break;
                                }
                                (*clone).ns = ns as *mut xmlNs;
                            }
                        }
                    }
                }
                if (*clone).type_0 as c_uint
                    == XML_ATTRIBUTE_NODE as c_int as c_uint
                    && !(*clone).parent.is_null()
                {
                    if xmlIsID(destDoc, (*clone).parent as xmlNodePtr, clone as xmlAttrPtr) != 0 {
                        let mut idVal: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        idVal = xmlNodeListGetString(
                            (*cur).doc as xmlDocPtr,
                            (*cur).children,
                            1 as c_int,
                        );
                        if !idVal.is_null() {
                            if xmlAddID(
                                ::core::ptr::null_mut::<xmlValidCtxt>(),
                                destDoc,
                                idVal,
                                cur as xmlAttrPtr,
                            )
                            .is_null()
                            {
                                xmlFree.expect("non-null function pointer")(
                                    idVal as *mut c_void,
                                );
                                current_block = 14946474315211274328;
                                break;
                            } else {
                                xmlFree.expect("non-null function pointer")(
                                    idVal as *mut c_void,
                                );
                            }
                        }
                    }
                }
                if (*cur).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    && !(*cur).properties.is_null()
                {
                    prevClone = ::core::ptr::null_mut::<xmlNode>();
                    parentClone = clone;
                    cur = (*cur).properties as xmlNodePtr;
                    continue;
                } else {
                    current_block = 16915751198307277387;
                }
            }
            _ => {}
        }
        loop {
            match current_block {
                4707564613567208506 => {
                    if cur == node {
                        current_block = 14799897360614227022;
                        break 's_86;
                    }
                    if (*cur).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                        || (*cur).type_0 as c_uint
                            == XML_XINCLUDE_START as c_int as c_uint
                        || (*cur).type_0 as c_uint
                            == XML_XINCLUDE_END as c_int as c_uint
                    {
                        if !nsMap.is_null() && !(*nsMap).first.is_null() {
                            while !(*nsMap).last.is_null() && (*(*nsMap).last).depth >= depth {
                                mi = (*nsMap).last;
                                (*nsMap).last = (*mi).prev;
                                if (*nsMap).last.is_null() {
                                    (*nsMap).first = ::core::ptr::null_mut::<xmlNsMapItem>();
                                } else {
                                    (*(*nsMap).last).next = ::core::ptr::null_mut::<xmlNsMapItem>();
                                }
                                (*mi).next = (*nsMap).pool;
                                (*nsMap).pool = mi;
                            }
                            mi = (*nsMap).first;
                            while !mi.is_null() {
                                if (*mi).shadowDepth >= depth {
                                    (*mi).shadowDepth = -(1 as c_int);
                                }
                                mi = (*mi).next;
                            }
                        }
                        depth -= 1;
                    }
                    if !(*cur).next.is_null() {
                        prevClone = clone;
                        cur = (*cur).next as xmlNodePtr;
                        break;
                    } else if (*cur).type_0 as c_uint
                        != XML_ATTRIBUTE_NODE as c_int as c_uint
                    {
                        if !(*clone).parent.is_null() {
                            (*(*clone).parent).last = clone as *mut _xmlNode;
                        }
                        clone = (*clone).parent as xmlNodePtr;
                        if !clone.is_null() {
                            parentClone = (*clone).parent as xmlNodePtr;
                        }
                        cur = (*cur).parent as xmlNodePtr;
                        current_block = 4707564613567208506;
                    } else {
                        clone = (*clone).parent as xmlNodePtr;
                        parentClone = (*clone).parent as xmlNodePtr;
                        cur = (*cur).parent as xmlNodePtr;
                        current_block = 16915751198307277387;
                    }
                }
                _ => {
                    if (*cur).children.is_null() {
                        current_block = 4707564613567208506;
                        continue;
                    }
                    if !(deep != 0
                        || (*cur).type_0 as c_uint
                            == XML_ATTRIBUTE_NODE as c_int as c_uint)
                    {
                        current_block = 4707564613567208506;
                        continue;
                    }
                    prevClone = ::core::ptr::null_mut::<xmlNode>();
                    parentClone = clone;
                    cur = (*cur).children as xmlNodePtr;
                    break;
                }
            }
        }
    }
    match current_block {
        14946474315211274328 => {
            ret = -(1 as c_int);
        }
        _ => {}
    }
    if !nsMap.is_null() {
        if !ctxt.is_null() && (*ctxt).namespaceMap == nsMap as *mut c_void {
            if !(*nsMap).first.is_null() {
                if !(*nsMap).pool.is_null() {
                    (*(*nsMap).last).next = (*nsMap).pool;
                }
                (*nsMap).pool = (*nsMap).first;
                (*nsMap).first = ::core::ptr::null_mut::<xmlNsMapItem>();
            }
        } else {
            xmlDOMWrapNsMapFree(nsMap);
        }
    }
    *resNode = resultClone;
    return ret;
}
fn xmlDOMWrapAdoptAttr(
    mut ctxt: xmlDOMWrapCtxtPtr,
    mut sourceDoc: xmlDocPtr,
    mut attr: xmlAttrPtr,
    mut destDoc: xmlDocPtr,
    mut destParent: xmlNodePtr,
    mut options: c_int,
) -> c_int { unsafe {
    let mut current_block: u64;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut adoptStr: c_int = 1 as c_int;
    if attr.is_null() || destDoc.is_null() {
        return -(1 as c_int);
    }
    (*attr).doc = destDoc as *mut _xmlDoc;
    if !(*attr).ns.is_null() {
        let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
        !ctxt.is_null();
        if !(*(*attr).ns).prefix.is_null()
            && *(*(*attr).ns)
                .prefix
                .offset(0 as c_int as isize) as c_int
                == 'x' as i32
            && *(*(*attr).ns)
                .prefix
                .offset(1 as c_int as isize) as c_int
                == 'm' as i32
            && *(*(*attr).ns)
                .prefix
                .offset(2 as c_int as isize) as c_int
                == 'l' as i32
            && *(*(*attr).ns)
                .prefix
                .offset(3 as c_int as isize) as c_int
                == 0 as c_int
        {
            ns = xmlTreeEnsureXMLDecl(destDoc);
            current_block = 12599329904712511516;
        } else if destParent.is_null() {
            ns = xmlDOMWrapStoreNs(destDoc, (*(*attr).ns).href, (*(*attr).ns).prefix);
            current_block = 12599329904712511516;
        } else if xmlSearchNsByNamespaceStrict(
            destDoc,
            destParent,
            (*(*attr).ns).href,
            &raw mut ns,
            1 as c_int,
        ) == -(1 as c_int)
        {
            current_block = 11014592810393162460;
        } else {
            if ns.is_null() {
                ns = xmlDOMWrapNSNormDeclareNsForced(
                    destDoc,
                    destParent,
                    (*(*attr).ns).href,
                    (*(*attr).ns).prefix,
                    1 as c_int,
                );
            }
            current_block = 12599329904712511516;
        }
        match current_block {
            11014592810393162460 => {}
            _ => {
                if ns.is_null() {
                    current_block = 11014592810393162460;
                } else {
                    (*attr).ns = ns as *mut xmlNs;
                    current_block = 7149356873433890176;
                }
            }
        }
    } else {
        current_block = 7149356873433890176;
    }
    match current_block {
        7149356873433890176 => {
            if adoptStr != 0 && !(*attr).name.is_null() {
                if !(*destDoc).dict.is_null() {
                    let mut old: *const xmlChar = (*attr).name;
                    (*attr).name = xmlDictLookup(
                        (*destDoc).dict as xmlDictPtr,
                        (*attr).name,
                        -(1 as c_int),
                    );
                    if sourceDoc.is_null()
                        || (*sourceDoc).dict.is_null()
                        || xmlDictOwns((*sourceDoc).dict as xmlDictPtr, old) == 0
                    {
                        xmlFree.expect("non-null function pointer")(
                            old as *mut c_char as *mut c_void,
                        );
                    }
                } else if !sourceDoc.is_null()
                    && !(*sourceDoc).dict.is_null()
                    && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*attr).name) != 0
                {
                    (*attr).name = xmlStrdup((*attr).name);
                }
            }
            (*attr).atype = 0 as xmlAttributeType;
            (*attr).psvi = NULL;
            if (*attr).children.is_null() {
                return 0 as c_int;
            }
            cur = (*attr).children as xmlNodePtr;
            if !(!cur.is_null()
                && (*cur).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint)
            {
                's_153: while !cur.is_null() {
                    (*cur).doc = destDoc as *mut _xmlDoc;
                    match (*cur).type_0 as c_uint {
                        3 | 4 => {
                            if adoptStr != 0
                                && !(*cur).content.is_null()
                                && !sourceDoc.is_null()
                                && !(*sourceDoc).dict.is_null()
                                && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*cur).content) != 0
                            {
                                if !(*destDoc).dict.is_null() {
                                    (*cur).content = xmlDictLookup(
                                        (*destDoc).dict as xmlDictPtr,
                                        (*cur).content,
                                        -(1 as c_int),
                                    )
                                        as *mut xmlChar;
                                } else {
                                    (*cur).content = xmlStrdup((*cur).content);
                                }
                            }
                        }
                        5 => {
                            (*cur).content = ::core::ptr::null_mut::<xmlChar>();
                            (*cur).children = ::core::ptr::null_mut::<_xmlNode>();
                            (*cur).last = ::core::ptr::null_mut::<_xmlNode>();
                            if !(*destDoc).intSubset.is_null() || !(*destDoc).extSubset.is_null() {
                                let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
                                ent = xmlGetDocEntity(destDoc as *const xmlDoc, (*cur).name);
                                if !ent.is_null() {
                                    (*cur).content = (*ent).content;
                                    (*cur).children = ent as xmlNodePtr as *mut _xmlNode;
                                    (*cur).last = ent as xmlNodePtr as *mut _xmlNode;
                                }
                            }
                        }
                        _ => {}
                    }
                    if !(*cur).children.is_null() {
                        cur = (*cur).children as xmlNodePtr;
                    } else {
                        loop {
                            if cur == attr as xmlNodePtr {
                                break 's_153;
                            }
                            if !(*cur).next.is_null() {
                                break;
                            }
                            cur = (*cur).parent as xmlNodePtr;
                        }
                        cur = (*cur).next as xmlNodePtr;
                    }
                }
                return 0 as c_int;
            }
        }
        _ => {}
    }
    return -(1 as c_int);
} }
#[inline]
pub fn xmlDOMWrapAdoptNode(
    mut ctxt: xmlDOMWrapCtxtPtr,
    mut sourceDoc: xmlDocPtr,
    mut node: xmlNodePtr,
    mut destDoc: xmlDocPtr,
    mut destParent: xmlNodePtr,
    mut options: c_int,
) -> c_int { unsafe {
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
        || destDoc.is_null()
        || !destParent.is_null() && (*destParent).doc != destDoc
    {
        return -(1 as c_int);
    }
    if !(*node).doc.is_null() && !sourceDoc.is_null() && (*node).doc != sourceDoc {
        return -(1 as c_int);
    }
    if sourceDoc.is_null() {
        sourceDoc = (*node).doc as xmlDocPtr;
    }
    if sourceDoc == destDoc {
        return -(1 as c_int);
    }
    match (*node).type_0 as c_uint {
        1 | 2 | 3 | 4 | 5 | 7 | 8 => {}
        11 => return 2 as c_int,
        _ => return 1 as c_int,
    }
    if !(*node).parent.is_null() && destParent != (*node).parent {
        xmlUnlinkNode(node);
    }
    if (*node).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
    {
        return xmlDOMWrapAdoptBranch(ctxt, sourceDoc, node, destDoc, destParent, options);
    } else if (*node).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        return xmlDOMWrapAdoptAttr(
            ctxt,
            sourceDoc,
            node as xmlAttrPtr,
            destDoc,
            destParent,
            options,
        );
    } else {
        let mut cur: xmlNodePtr = node;
        let mut adoptStr: c_int = 1 as c_int;
        (*cur).doc = destDoc as *mut _xmlDoc;
        if !sourceDoc.is_null() && (*sourceDoc).dict == (*destDoc).dict {
            adoptStr = 0 as c_int;
        }
        match (*node).type_0 as c_uint {
            3 | 4 => {
                if adoptStr != 0
                    && !(*node).content.is_null()
                    && !sourceDoc.is_null()
                    && !(*sourceDoc).dict.is_null()
                    && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*cur).content) != 0
                {
                    if !(*destDoc).dict.is_null() {
                        (*cur).content = xmlDictLookup(
                            (*destDoc).dict as xmlDictPtr,
                            (*cur).content,
                            -(1 as c_int),
                        ) as *mut xmlChar;
                    } else {
                        (*cur).content = xmlStrdup((*cur).content);
                    }
                }
            }
            5 => {
                (*node).content = ::core::ptr::null_mut::<xmlChar>();
                (*node).children = ::core::ptr::null_mut::<_xmlNode>();
                (*node).last = ::core::ptr::null_mut::<_xmlNode>();
                if !(*destDoc).intSubset.is_null() || !(*destDoc).extSubset.is_null() {
                    let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
                    ent = xmlGetDocEntity(destDoc as *const xmlDoc, (*node).name);
                    if !ent.is_null() {
                        (*node).content = (*ent).content;
                        (*node).children = ent as xmlNodePtr as *mut _xmlNode;
                        (*node).last = ent as xmlNodePtr as *mut _xmlNode;
                    }
                }
                if adoptStr != 0 && !(*node).name.is_null() {
                    if !(*destDoc).dict.is_null() {
                        let mut old: *const xmlChar = (*node).name;
                        (*node).name = xmlDictLookup(
                            (*destDoc).dict as xmlDictPtr,
                            (*node).name,
                            -(1 as c_int),
                        );
                        if sourceDoc.is_null()
                            || (*sourceDoc).dict.is_null()
                            || xmlDictOwns((*sourceDoc).dict as xmlDictPtr, old) == 0
                        {
                            xmlFree.expect("non-null function pointer")(
                                old as *mut c_char as *mut c_void,
                            );
                        }
                    } else if !sourceDoc.is_null()
                        && !(*sourceDoc).dict.is_null()
                        && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*node).name) != 0
                    {
                        (*node).name = xmlStrdup((*node).name);
                    }
                }
            }
            7 => {
                if adoptStr != 0 && !(*node).name.is_null() {
                    if !(*destDoc).dict.is_null() {
                        let mut old_0: *const xmlChar = (*node).name;
                        (*node).name = xmlDictLookup(
                            (*destDoc).dict as xmlDictPtr,
                            (*node).name,
                            -(1 as c_int),
                        );
                        if sourceDoc.is_null()
                            || (*sourceDoc).dict.is_null()
                            || xmlDictOwns((*sourceDoc).dict as xmlDictPtr, old_0) == 0
                        {
                            xmlFree.expect("non-null function pointer")(
                                old_0 as *mut c_char as *mut c_void,
                            );
                        }
                    } else if !sourceDoc.is_null()
                        && !(*sourceDoc).dict.is_null()
                        && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*node).name) != 0
                    {
                        (*node).name = xmlStrdup((*node).name);
                    }
                }
                if adoptStr != 0
                    && !(*node).content.is_null()
                    && !sourceDoc.is_null()
                    && !(*sourceDoc).dict.is_null()
                    && xmlDictOwns((*sourceDoc).dict as xmlDictPtr, (*cur).content) != 0
                {
                    if !(*destDoc).dict.is_null() {
                        (*cur).content = xmlDictLookup(
                            (*destDoc).dict as xmlDictPtr,
                            (*cur).content,
                            -(1 as c_int),
                        ) as *mut xmlChar;
                    } else {
                        (*cur).content = xmlStrdup((*cur).content);
                    }
                }
            }
            _ => {}
        }
    }
    return 0 as c_int;
} }
#[inline]
pub unsafe fn xmlIsXHTML(
    mut systemID: *const xmlChar,
    mut publicID: *const xmlChar,
) -> c_int {
    if systemID.is_null() && publicID.is_null() {
        return -(1 as c_int);
    }
    if !publicID.is_null() {
        if xmlStrEqual(
            publicID,
            b"-//W3C//DTD XHTML 1.0 Strict//EN\0" as *const u8 as *const c_char
                as *mut xmlChar,
        ) != 0
        {
            return 1 as c_int;
        }
        if xmlStrEqual(
            publicID,
            b"-//W3C//DTD XHTML 1.0 Frameset//EN\0" as *const u8 as *const c_char
                as *mut xmlChar,
        ) != 0
        {
            return 1 as c_int;
        }
        if xmlStrEqual(
            publicID,
            b"-//W3C//DTD XHTML 1.0 Transitional//EN\0" as *const u8 as *const c_char
                as *mut xmlChar,
        ) != 0
        {
            return 1 as c_int;
        }
    }
    if !systemID.is_null() {
        if xmlStrEqual(
            systemID,
            b"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\0" as *const u8
                as *const c_char as *mut xmlChar,
        ) != 0
        {
            return 1 as c_int;
        }
        if xmlStrEqual(
            systemID,
            b"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\0" as *const u8
                as *const c_char as *mut xmlChar,
        ) != 0
        {
            return 1 as c_int;
        }
        if xmlStrEqual(
            systemID,
            b"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\0" as *const u8
                as *const c_char as *mut xmlChar,
        ) != 0
        {
            return 1 as c_int;
        }
    }
    return 0 as c_int;
}
#[inline]
pub fn xmlRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc { unsafe {
    let mut old: xmlRegisterNodeFunc = *__xmlRegisterNodeDefaultValue();
    __xmlRegisterCallbacks = 1 as c_int;
    let ref mut fresh15 = *__xmlRegisterNodeDefaultValue();
    *fresh15 = func;
    return old;
} }
#[inline]
pub fn xmlDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc { unsafe {
    let mut old: xmlDeregisterNodeFunc = *__xmlDeregisterNodeDefaultValue();
    __xmlRegisterCallbacks = 1 as c_int;
    let ref mut fresh16 = *__xmlDeregisterNodeDefaultValue();
    *fresh16 = func;
    return old;
} }

pub const XML_XML_NAMESPACE: *const xmlChar = b"http://www.w3.org/XML/1998/namespace\0" as *const u8
    as *const c_char as *const xmlChar;

pub const UINT_MAX: c_uint = (__INT_MAX__ as c_uint)
    .wrapping_mul(2 as c_uint)
    .wrapping_add(1 as c_uint);
