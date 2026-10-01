use core::ffi::*;
use crate::src::c_inlined_fns::toupper;
use crate::src::buf::xmlBufAddLen;
use crate::src::buf::xmlBufAvail;
use crate::src::buf::xmlBufContent;
use crate::src::buf::xmlBufEnd;
use crate::src::buf::xmlBufGrow;
use crate::src::buf::xmlBufShrink;
use crate::src::buf::xmlBufUse;
use crate::src::tree::xmlBufferGrow;
use crate::src::tree::xmlBufferShrink;
use crate::src::xmlstring::xmlGetUTF8Char;
use crate::src::threads::xmlInitParser;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
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
    fn UTF8ToHtml(
        out: *mut c_uchar,
        outlen: *mut c_int,
        in_0: *const c_uchar,
        inlen: *mut c_int,
    ) -> c_int;
}

pub type C2RustUnnamed_htaaa91a68 = c_int;
pub const XML_ENC_ERR_MEMORY: C2RustUnnamed_htaaa91a68 = -5;
pub const XML_ENC_ERR_INTERNAL: C2RustUnnamed_htaaa91a68 = -4;
pub const XML_ENC_ERR_PARTIAL: C2RustUnnamed_htaaa91a68 = -3;
pub const XML_ENC_ERR_INPUT: C2RustUnnamed_htaaa91a68 = -2;
pub const XML_ENC_ERR_SPACE: C2RustUnnamed_htaaa91a68 = -1;
pub const XML_ENC_ERR_SUCCESS: C2RustUnnamed_htaaa91a68 = 0;

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

pub type xmlCharEncodingAliasPtr = *mut xmlCharEncodingAlias;
pub type xmlCharEncodingAlias = _xmlCharEncodingAlias;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingAlias {
    pub name: *const c_char,
    pub alias: *const c_char,
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
pub type C2RustUnnamed_htdd24ee73 = c_uint;
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
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_htdd24ee73 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_htdd24ee73 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_htdd24ee73 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_htdd24ee73 = 1300;
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
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_ERR_OK: C2RustUnnamed_htdd24ee73 = 0;

static mut xmlCharEncodingAliases: xmlCharEncodingAliasPtr =
    ::core::ptr::null::<xmlCharEncodingAlias>() as *mut xmlCharEncodingAlias;
static mut xmlCharEncodingAliasesNb: c_int = 0 as c_int;
static mut xmlCharEncodingAliasesMax: c_int = 0 as c_int;
static mut xmlLittleEndian: c_int = 1 as c_int;
unsafe extern "C" fn asciiToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    let mut outstart: *mut c_uchar = out;
    let mut base: *const c_uchar = in_0;
    let mut processed: *const c_uchar = in_0;
    let mut outend: *mut c_uchar = out.offset(*outlen as isize);
    let mut inend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut c: c_uint = 0;
    inend = in_0.offset(*inlen as isize);
    while in_0 < inend
        && (out.offset_from(outstart) as c_long + 5 as c_long)
            < *outlen as c_long
    {
        let fresh23 = in_0;
        in_0 = in_0.offset(1);
        c = *fresh23 as c_uint;
        if out >= outend {
            break;
        }
        if c < 0x80 as c_uint {
            let fresh24 = out;
            out = out.offset(1);
            *fresh24 = c as c_uchar;
        } else {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(base) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        }
        processed = in_0;
    }
    *outlen = out.offset_from(outstart) as c_long as c_int;
    *inlen = processed.offset_from(base) as c_long as c_int;
    return *outlen;
}
unsafe extern "C" fn UTF8Toascii(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    let mut processed: *const c_uchar = in_0;
    let mut outend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut outstart: *const c_uchar = out;
    let mut instart: *const c_uchar = in_0;
    let mut inend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut c: c_uint = 0;
    let mut d: c_uint = 0;
    let mut trailing: c_int = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if in_0.is_null() {
        *outlen = 0 as c_int;
        *inlen = 0 as c_int;
        return 0 as c_int;
    }
    inend = in_0.offset(*inlen as isize);
    outend = out.offset(*outlen as isize);
    while in_0 < inend {
        let fresh20 = in_0;
        in_0 = in_0.offset(1);
        d = *fresh20 as c_uint;
        if d < 0x80 as c_uint {
            c = d;
            trailing = 0 as c_int;
        } else if d < 0xc0 as c_uint {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        } else if d < 0xe0 as c_uint {
            c = d & 0x1f as c_uint;
            trailing = 1 as c_int;
        } else if d < 0xf0 as c_uint {
            c = d & 0xf as c_uint;
            trailing = 2 as c_int;
        } else if d < 0xf8 as c_uint {
            c = d & 0x7 as c_uint;
            trailing = 3 as c_int;
        } else {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        }
        if (inend.offset_from(in_0) as c_long) < trailing as c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                let fresh21 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh21 as c_uint;
                d & 0xc0 as c_uint != 0x80 as c_uint
            } {
                break;
            }
            c <<= 6 as c_int;
            c |= d & 0x3f as c_uint;
            trailing -= 1;
        }
        if c < 0x80 as c_uint {
            if out >= outend as *mut c_uchar {
                break;
            }
            let fresh22 = out;
            out = out.offset(1);
            *fresh22 = c as c_uchar;
            processed = in_0;
        } else {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        }
    }
    *outlen = out.offset_from(outstart) as c_long as c_int;
    *inlen = processed.offset_from(instart) as c_long as c_int;
    return *outlen;
}
#[no_mangle]
pub unsafe extern "C" fn isolat1ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    let mut outstart: *mut c_uchar = out;
    let mut base: *const c_uchar = in_0;
    let mut outend: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut inend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut instop: *const c_uchar = ::core::ptr::null::<c_uchar>();
    if out.is_null() || in_0.is_null() || outlen.is_null() || inlen.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    outend = out.offset(*outlen as isize);
    inend = in_0.offset(*inlen as isize);
    instop = inend;
    while in_0 < inend && out < outend.offset(-(1 as c_int as isize)) {
        if *in_0 as c_int >= 0x80 as c_int {
            let fresh28 = out;
            out = out.offset(1);
            *fresh28 = (*in_0 as c_int >> 6 as c_int
                & 0x1f as c_int
                | 0xc0 as c_int) as c_uchar;
            let fresh29 = out;
            out = out.offset(1);
            *fresh29 = (*in_0 as c_int & 0x3f as c_int
                | 0x80 as c_int) as c_uchar;
            in_0 = in_0.offset(1);
        }
        if instop.offset_from(in_0) as c_long
            > outend.offset_from(out) as c_long
        {
            instop = in_0.offset(outend.offset_from(out) as c_long as isize);
        }
        while in_0 < instop && (*in_0 as c_int) < 0x80 as c_int {
            let fresh30 = in_0;
            in_0 = in_0.offset(1);
            let fresh31 = out;
            out = out.offset(1);
            *fresh31 = *fresh30;
        }
    }
    if in_0 < inend && out < outend && (*in_0 as c_int) < 0x80 as c_int {
        let fresh32 = in_0;
        in_0 = in_0.offset(1);
        let fresh33 = out;
        out = out.offset(1);
        *fresh33 = *fresh32;
    }
    *outlen = out.offset_from(outstart) as c_long as c_int;
    *inlen = in_0.offset_from(base) as c_long as c_int;
    return *outlen;
}
unsafe extern "C" fn UTF8ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut inb: *const c_uchar,
    mut inlenb: *mut c_int,
) -> c_int {
    let mut len: c_int = 0;
    if out.is_null() || outlen.is_null() || inlenb.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if inb.is_null() {
        *outlen = 0 as c_int;
        *inlenb = 0 as c_int;
        return 0 as c_int;
    }
    if *outlen > *inlenb {
        len = *inlenb;
    } else {
        len = *outlen;
    }
    if len < 0 as c_int {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    memcpy(
        out as *mut c_void,
        inb as *const c_void,
        len as size_t,
    );
    *outlen = len;
    *inlenb = len;
    return *outlen;
}
#[no_mangle]
pub unsafe extern "C" fn UTF8Toisolat1(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    let mut processed: *const c_uchar = in_0;
    let mut outend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut outstart: *const c_uchar = out;
    let mut instart: *const c_uchar = in_0;
    let mut inend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut c: c_uint = 0;
    let mut d: c_uint = 0;
    let mut trailing: c_int = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if in_0.is_null() {
        *outlen = 0 as c_int;
        *inlen = 0 as c_int;
        return 0 as c_int;
    }
    inend = in_0.offset(*inlen as isize);
    outend = out.offset(*outlen as isize);
    while in_0 < inend {
        let fresh25 = in_0;
        in_0 = in_0.offset(1);
        d = *fresh25 as c_uint;
        if d < 0x80 as c_uint {
            c = d;
            trailing = 0 as c_int;
        } else if d < 0xc0 as c_uint {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        } else if d < 0xe0 as c_uint {
            c = d & 0x1f as c_uint;
            trailing = 1 as c_int;
        } else if d < 0xf0 as c_uint {
            c = d & 0xf as c_uint;
            trailing = 2 as c_int;
        } else if d < 0xf8 as c_uint {
            c = d & 0x7 as c_uint;
            trailing = 3 as c_int;
        } else {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        }
        if (inend.offset_from(in_0) as c_long) < trailing as c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend {
                break;
            }
            let fresh26 = in_0;
            in_0 = in_0.offset(1);
            d = *fresh26 as c_uint;
            if d & 0xc0 as c_uint != 0x80 as c_uint {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen =
                    processed.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
            c <<= 6 as c_int;
            c |= d & 0x3f as c_uint;
            trailing -= 1;
        }
        if c <= 0xff as c_uint {
            if out >= outend as *mut c_uchar {
                break;
            }
            let fresh27 = out;
            out = out.offset(1);
            *fresh27 = c as c_uchar;
            processed = in_0;
        } else {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        }
    }
    *outlen = out.offset_from(outstart) as c_long as c_int;
    *inlen = processed.offset_from(instart) as c_long as c_int;
    return *outlen;
}
unsafe extern "C" fn UTF16LEToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut inb: *const c_uchar,
    mut inlenb: *mut c_int,
) -> c_int {
    let mut outstart: *mut c_uchar = out;
    let mut processed: *const c_uchar = inb;
    let mut outend: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut in_0: *mut c_ushort =
        inb as *mut c_void as *mut c_ushort;
    let mut inend: *mut c_ushort = ::core::ptr::null_mut::<c_ushort>();
    let mut c: c_uint = 0;
    let mut d: c_uint = 0;
    let mut inlen: c_uint = 0;
    let mut tmp: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut bits: c_int = 0;
    if *outlen == 0 as c_int {
        *inlenb = 0 as c_int;
        return 0 as c_int;
    }
    outend = out.offset(*outlen as isize);
    if *inlenb % 2 as c_int == 1 as c_int {
        *inlenb -= 1;
    }
    inlen = (*inlenb / 2 as c_int) as c_uint;
    inend = in_0.offset(inlen as isize);
    while in_0 < inend
        && (out.offset_from(outstart) as c_long + 5 as c_long)
            < *outlen as c_long
    {
        if xmlLittleEndian != 0 {
            let fresh39 = in_0;
            in_0 = in_0.offset(1);
            c = *fresh39 as c_uint;
        } else {
            tmp = in_0 as *mut c_uchar;
            let fresh40 = tmp;
            tmp = tmp.offset(1);
            c = *fresh40 as c_uint;
            c = c
                | ((*tmp as c_int) << 8 as c_int) as c_uint;
            in_0 = in_0.offset(1);
        }
        if c & 0xfc00 as c_uint == 0xd800 as c_uint {
            if in_0 >= inend {
                break;
            }
            if xmlLittleEndian != 0 {
                let fresh41 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh41 as c_uint;
            } else {
                tmp = in_0 as *mut c_uchar;
                let fresh42 = tmp;
                tmp = tmp.offset(1);
                d = *fresh42 as c_uint;
                d = d
                    | ((*tmp as c_int) << 8 as c_int)
                        as c_uint;
                in_0 = in_0.offset(1);
            }
            if d & 0xfc00 as c_uint == 0xdc00 as c_uint {
                c &= 0x3ff as c_uint;
                c <<= 10 as c_int;
                c |= d & 0x3ff as c_uint;
                c = c.wrapping_add(0x10000 as c_int as c_uint);
            } else {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlenb = processed.offset_from(inb) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
        }
        if out >= outend {
            break;
        }
        if c < 0x80 as c_uint {
            let fresh43 = out;
            out = out.offset(1);
            *fresh43 = c as c_uchar;
            bits = -(6 as c_int);
        } else if c < 0x800 as c_uint {
            let fresh44 = out;
            out = out.offset(1);
            *fresh44 = (c >> 6 as c_int & 0x1f as c_uint
                | 0xc0 as c_uint) as c_uchar;
            bits = 0 as c_int;
        } else if c < 0x10000 as c_int as c_uint {
            let fresh45 = out;
            out = out.offset(1);
            *fresh45 = (c >> 12 as c_int & 0xf as c_uint
                | 0xe0 as c_uint) as c_uchar;
            bits = 6 as c_int;
        } else {
            let fresh46 = out;
            out = out.offset(1);
            *fresh46 = (c >> 18 as c_int & 0x7 as c_uint
                | 0xf0 as c_uint) as c_uchar;
            bits = 12 as c_int;
        }
        while bits >= 0 as c_int {
            if out >= outend {
                break;
            }
            let fresh47 = out;
            out = out.offset(1);
            *fresh47 = (c >> bits & 0x3f as c_uint | 0x80 as c_uint)
                as c_uchar;
            bits -= 6 as c_int;
        }
        processed = in_0 as *const c_uchar;
    }
    *outlen = out.offset_from(outstart) as c_long as c_int;
    *inlenb = processed.offset_from(inb) as c_long as c_int;
    return *outlen;
}
unsafe extern "C" fn UTF8ToUTF16LE(
    mut outb: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    let mut out: *mut c_ushort =
        outb as *mut c_void as *mut c_ushort;
    let mut processed: *const c_uchar = in_0;
    let instart: *const c_uchar = in_0;
    let mut outstart: *mut c_ushort = out;
    let mut outend: *mut c_ushort = ::core::ptr::null_mut::<c_ushort>();
    let mut inend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut c: c_uint = 0;
    let mut d: c_uint = 0;
    let mut trailing: c_int = 0;
    let mut tmp: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut tmp1: c_ushort = 0;
    let mut tmp2: c_ushort = 0;
    if out.is_null() || outlen.is_null() || inlen.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if in_0.is_null() {
        *outlen = 0 as c_int;
        *inlen = 0 as c_int;
        return 0 as c_int;
    }
    inend = in_0.offset(*inlen as isize);
    outend = out.offset((*outlen / 2 as c_int) as isize);
    while in_0 < inend {
        let fresh34 = in_0;
        in_0 = in_0.offset(1);
        d = *fresh34 as c_uint;
        if d < 0x80 as c_uint {
            c = d;
            trailing = 0 as c_int;
        } else if d < 0xc0 as c_uint {
            *outlen = (out.offset_from(outstart) as c_long * 2 as c_long)
                as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        } else if d < 0xe0 as c_uint {
            c = d & 0x1f as c_uint;
            trailing = 1 as c_int;
        } else if d < 0xf0 as c_uint {
            c = d & 0xf as c_uint;
            trailing = 2 as c_int;
        } else if d < 0xf8 as c_uint {
            c = d & 0x7 as c_uint;
            trailing = 3 as c_int;
        } else {
            *outlen = (out.offset_from(outstart) as c_long * 2 as c_long)
                as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        }
        if (inend.offset_from(in_0) as c_long) < trailing as c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                let fresh35 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh35 as c_uint;
                d & 0xc0 as c_uint != 0x80 as c_uint
            } {
                break;
            }
            c <<= 6 as c_int;
            c |= d & 0x3f as c_uint;
            trailing -= 1;
        }
        if c < 0x10000 as c_int as c_uint {
            if out >= outend {
                break;
            }
            if xmlLittleEndian != 0 {
                let fresh36 = out;
                out = out.offset(1);
                *fresh36 = c as c_ushort;
            } else {
                tmp = out as *mut c_uchar;
                *tmp = c as c_uchar;
                *tmp.offset(1 as c_int as isize) =
                    (c >> 8 as c_int) as c_uchar;
                out = out.offset(1);
            }
        } else {
            if !(c < 0x110000 as c_int as c_uint) {
                break;
            }
            if out.offset(1 as c_int as isize) >= outend {
                break;
            }
            c = c.wrapping_sub(0x10000 as c_int as c_uint);
            if xmlLittleEndian != 0 {
                let fresh37 = out;
                out = out.offset(1);
                *fresh37 = (0xd800 as c_uint | c >> 10 as c_int)
                    as c_ushort;
                let fresh38 = out;
                out = out.offset(1);
                *fresh38 = (0xdc00 as c_uint | c & 0x3ff as c_uint)
                    as c_ushort;
            } else {
                tmp1 = (0xd800 as c_uint | c >> 10 as c_int)
                    as c_ushort;
                tmp = out as *mut c_uchar;
                *tmp = tmp1 as c_uchar;
                *tmp.offset(1 as c_int as isize) =
                    (tmp1 as c_int >> 8 as c_int) as c_uchar;
                out = out.offset(1);
                tmp2 = (0xdc00 as c_uint | c & 0x3ff as c_uint)
                    as c_ushort;
                tmp = out as *mut c_uchar;
                *tmp = tmp2 as c_uchar;
                *tmp.offset(1 as c_int as isize) =
                    (tmp2 as c_int >> 8 as c_int) as c_uchar;
                out = out.offset(1);
            }
        }
        processed = in_0;
    }
    *outlen = (out.offset_from(outstart) as c_long * 2 as c_long)
        as c_int;
    *inlen = processed.offset_from(instart) as c_long as c_int;
    return *outlen;
}
unsafe extern "C" fn UTF8ToUTF16(
    mut outb: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    if in_0.is_null() {
        if *outlen >= 2 as c_int {
            *outb.offset(0 as c_int as isize) = 0xff as c_uchar;
            *outb.offset(1 as c_int as isize) = 0xfe as c_uchar;
            *outlen = 2 as c_int;
            *inlen = 0 as c_int;
            return 2 as c_int;
        }
        *outlen = 0 as c_int;
        *inlen = 0 as c_int;
        return 0 as c_int;
    }
    return UTF8ToUTF16LE(outb, outlen, in_0, inlen);
}
unsafe extern "C" fn UTF16BEToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut inb: *const c_uchar,
    mut inlenb: *mut c_int,
) -> c_int {
    let mut outstart: *mut c_uchar = out;
    let mut processed: *const c_uchar = inb;
    let mut outend: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut in_0: *mut c_ushort =
        inb as *mut c_void as *mut c_ushort;
    let mut inend: *mut c_ushort = ::core::ptr::null_mut::<c_ushort>();
    let mut c: c_uint = 0;
    let mut d: c_uint = 0;
    let mut inlen: c_uint = 0;
    let mut tmp: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut bits: c_int = 0;
    if *outlen == 0 as c_int {
        *inlenb = 0 as c_int;
        return 0 as c_int;
    }
    outend = out.offset(*outlen as isize);
    if *inlenb % 2 as c_int == 1 as c_int {
        *inlenb -= 1;
    }
    inlen = (*inlenb / 2 as c_int) as c_uint;
    inend = in_0.offset(inlen as isize);
    while in_0 < inend
        && (out.offset_from(outstart) as c_long + 5 as c_long)
            < *outlen as c_long
    {
        if xmlLittleEndian != 0 {
            tmp = in_0 as *mut c_uchar;
            let fresh53 = tmp;
            tmp = tmp.offset(1);
            c = *fresh53 as c_uint;
            c = c << 8 as c_int | *tmp as c_uint;
            in_0 = in_0.offset(1);
        } else {
            let fresh54 = in_0;
            in_0 = in_0.offset(1);
            c = *fresh54 as c_uint;
        }
        if c & 0xfc00 as c_uint == 0xd800 as c_uint {
            if in_0 >= inend {
                break;
            }
            if xmlLittleEndian != 0 {
                tmp = in_0 as *mut c_uchar;
                let fresh55 = tmp;
                tmp = tmp.offset(1);
                d = *fresh55 as c_uint;
                d = d << 8 as c_int | *tmp as c_uint;
                in_0 = in_0.offset(1);
            } else {
                let fresh56 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh56 as c_uint;
            }
            if d & 0xfc00 as c_uint == 0xdc00 as c_uint {
                c &= 0x3ff as c_uint;
                c <<= 10 as c_int;
                c |= d & 0x3ff as c_uint;
                c = c.wrapping_add(0x10000 as c_int as c_uint);
            } else {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlenb = processed.offset_from(inb) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
        }
        if out >= outend {
            break;
        }
        if c < 0x80 as c_uint {
            let fresh57 = out;
            out = out.offset(1);
            *fresh57 = c as c_uchar;
            bits = -(6 as c_int);
        } else if c < 0x800 as c_uint {
            let fresh58 = out;
            out = out.offset(1);
            *fresh58 = (c >> 6 as c_int & 0x1f as c_uint
                | 0xc0 as c_uint) as c_uchar;
            bits = 0 as c_int;
        } else if c < 0x10000 as c_int as c_uint {
            let fresh59 = out;
            out = out.offset(1);
            *fresh59 = (c >> 12 as c_int & 0xf as c_uint
                | 0xe0 as c_uint) as c_uchar;
            bits = 6 as c_int;
        } else {
            let fresh60 = out;
            out = out.offset(1);
            *fresh60 = (c >> 18 as c_int & 0x7 as c_uint
                | 0xf0 as c_uint) as c_uchar;
            bits = 12 as c_int;
        }
        while bits >= 0 as c_int {
            if out >= outend {
                break;
            }
            let fresh61 = out;
            out = out.offset(1);
            *fresh61 = (c >> bits & 0x3f as c_uint | 0x80 as c_uint)
                as c_uchar;
            bits -= 6 as c_int;
        }
        processed = in_0 as *const c_uchar;
    }
    *outlen = out.offset_from(outstart) as c_long as c_int;
    *inlenb = processed.offset_from(inb) as c_long as c_int;
    return *outlen;
}
unsafe extern "C" fn UTF8ToUTF16BE(
    mut outb: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    let mut out: *mut c_ushort =
        outb as *mut c_void as *mut c_ushort;
    let mut processed: *const c_uchar = in_0;
    let instart: *const c_uchar = in_0;
    let mut outstart: *mut c_ushort = out;
    let mut outend: *mut c_ushort = ::core::ptr::null_mut::<c_ushort>();
    let mut inend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut c: c_uint = 0;
    let mut d: c_uint = 0;
    let mut trailing: c_int = 0;
    let mut tmp: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut tmp1: c_ushort = 0;
    let mut tmp2: c_ushort = 0;
    if outb.is_null() || outlen.is_null() || inlen.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if in_0.is_null() {
        *outlen = 0 as c_int;
        *inlen = 0 as c_int;
        return 0 as c_int;
    }
    inend = in_0.offset(*inlen as isize);
    outend = out.offset((*outlen / 2 as c_int) as isize);
    while in_0 < inend {
        let fresh48 = in_0;
        in_0 = in_0.offset(1);
        d = *fresh48 as c_uint;
        if d < 0x80 as c_uint {
            c = d;
            trailing = 0 as c_int;
        } else if d < 0xc0 as c_uint {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        } else if d < 0xe0 as c_uint {
            c = d & 0x1f as c_uint;
            trailing = 1 as c_int;
        } else if d < 0xf0 as c_uint {
            c = d & 0xf as c_uint;
            trailing = 2 as c_int;
        } else if d < 0xf8 as c_uint {
            c = d & 0x7 as c_uint;
            trailing = 3 as c_int;
        } else {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        }
        if (inend.offset_from(in_0) as c_long) < trailing as c_long {
            break;
        }
        while trailing != 0 {
            if in_0 >= inend || {
                let fresh49 = in_0;
                in_0 = in_0.offset(1);
                d = *fresh49 as c_uint;
                d & 0xc0 as c_uint != 0x80 as c_uint
            } {
                break;
            }
            c <<= 6 as c_int;
            c |= d & 0x3f as c_uint;
            trailing -= 1;
        }
        if c < 0x10000 as c_int as c_uint {
            if out >= outend {
                break;
            }
            if xmlLittleEndian != 0 {
                tmp = out as *mut c_uchar;
                *tmp = (c >> 8 as c_int) as c_uchar;
                *tmp.offset(1 as c_int as isize) = c as c_uchar;
                out = out.offset(1);
            } else {
                let fresh50 = out;
                out = out.offset(1);
                *fresh50 = c as c_ushort;
            }
        } else {
            if !(c < 0x110000 as c_int as c_uint) {
                break;
            }
            if out.offset(1 as c_int as isize) >= outend {
                break;
            }
            c = c.wrapping_sub(0x10000 as c_int as c_uint);
            if xmlLittleEndian != 0 {
                tmp1 = (0xd800 as c_uint | c >> 10 as c_int)
                    as c_ushort;
                tmp = out as *mut c_uchar;
                *tmp =
                    (tmp1 as c_int >> 8 as c_int) as c_uchar;
                *tmp.offset(1 as c_int as isize) = tmp1 as c_uchar;
                out = out.offset(1);
                tmp2 = (0xdc00 as c_uint | c & 0x3ff as c_uint)
                    as c_ushort;
                tmp = out as *mut c_uchar;
                *tmp =
                    (tmp2 as c_int >> 8 as c_int) as c_uchar;
                *tmp.offset(1 as c_int as isize) = tmp2 as c_uchar;
                out = out.offset(1);
            } else {
                let fresh51 = out;
                out = out.offset(1);
                *fresh51 = (0xd800 as c_uint | c >> 10 as c_int)
                    as c_ushort;
                let fresh52 = out;
                out = out.offset(1);
                *fresh52 = (0xdc00 as c_uint | c & 0x3ff as c_uint)
                    as c_ushort;
            }
        }
        processed = in_0;
    }
    *outlen = (out.offset_from(outstart) as c_long * 2 as c_long)
        as c_int;
    *inlen = processed.offset_from(instart) as c_long as c_int;
    return *outlen;
}
#[inline]
pub unsafe fn xmlDetectCharEncoding(
    mut in_0: *const c_uchar,
    mut len: c_int,
) -> xmlCharEncoding {
    if in_0.is_null() {
        return XML_CHAR_ENCODING_NONE;
    }
    if len >= 4 as c_int {
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0 as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0 as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0 as c_int
            && *in_0.offset(3 as c_int as isize) as c_int
                == 0x3c as c_int
        {
            return XML_CHAR_ENCODING_UCS4BE;
        }
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0x3c as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0 as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0 as c_int
            && *in_0.offset(3 as c_int as isize) as c_int
                == 0 as c_int
        {
            return XML_CHAR_ENCODING_UCS4LE;
        }
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0 as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0 as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0x3c as c_int
            && *in_0.offset(3 as c_int as isize) as c_int
                == 0 as c_int
        {
            return XML_CHAR_ENCODING_UCS4_2143;
        }
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0 as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0x3c as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0 as c_int
            && *in_0.offset(3 as c_int as isize) as c_int
                == 0 as c_int
        {
            return XML_CHAR_ENCODING_UCS4_3412;
        }
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0x4c as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0x6f as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0xa7 as c_int
            && *in_0.offset(3 as c_int as isize) as c_int
                == 0x94 as c_int
        {
            return XML_CHAR_ENCODING_EBCDIC;
        }
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0x3c as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0x3f as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0x78 as c_int
            && *in_0.offset(3 as c_int as isize) as c_int
                == 0x6d as c_int
        {
            return XML_CHAR_ENCODING_UTF8;
        }
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0x3c as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0 as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0x3f as c_int
            && *in_0.offset(3 as c_int as isize) as c_int
                == 0 as c_int
        {
            return XML_CHAR_ENCODING_UTF16LE;
        }
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0 as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0x3c as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0 as c_int
            && *in_0.offset(3 as c_int as isize) as c_int
                == 0x3f as c_int
        {
            return XML_CHAR_ENCODING_UTF16BE;
        }
    }
    if len >= 3 as c_int {
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0xef as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0xbb as c_int
            && *in_0.offset(2 as c_int as isize) as c_int
                == 0xbf as c_int
        {
            return XML_CHAR_ENCODING_UTF8;
        }
    }
    if len >= 2 as c_int {
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0xfe as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0xff as c_int
        {
            return XML_CHAR_ENCODING_UTF16BE;
        }
        if *in_0.offset(0 as c_int as isize) as c_int
            == 0xff as c_int
            && *in_0.offset(1 as c_int as isize) as c_int
                == 0xfe as c_int
        {
            return XML_CHAR_ENCODING_UTF16LE;
        }
    }
    return XML_CHAR_ENCODING_NONE;
}
#[inline]
pub fn xmlCleanupEncodingAliases() { unsafe {
    let mut i: c_int = 0;
    if xmlCharEncodingAliases.is_null() {
        return;
    }
    i = 0 as c_int;
    while i < xmlCharEncodingAliasesNb {
        if !(*xmlCharEncodingAliases.offset(i as isize)).name.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*xmlCharEncodingAliases.offset(i as isize)).name as *mut c_char
                    as *mut c_void,
            );
        }
        if !(*xmlCharEncodingAliases.offset(i as isize)).alias.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*xmlCharEncodingAliases.offset(i as isize)).alias as *mut c_char
                    as *mut c_void,
            );
        }
        i += 1;
    }
    xmlCharEncodingAliasesNb = 0 as c_int;
    xmlCharEncodingAliasesMax = 0 as c_int;
    xmlFree.expect("non-null function pointer")(xmlCharEncodingAliases as *mut c_void);
    xmlCharEncodingAliases = ::core::ptr::null_mut::<xmlCharEncodingAlias>();
} }
#[inline]
pub unsafe fn xmlGetEncodingAlias(
    mut alias: *const c_char,
) -> *const c_char {
    let mut i: c_int = 0;
    let mut upper: [c_char; 100] = [0; 100];
    if alias.is_null() {
        return ::core::ptr::null::<c_char>();
    }
    if xmlCharEncodingAliases.is_null() {
        return ::core::ptr::null::<c_char>();
    }
    i = 0 as c_int;
    while i < 99 as c_int {
        upper[i as usize] = ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_uchar>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *alias.offset(i as isize) as c_uchar as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = toupper(
                        *alias.offset(i as isize) as c_uchar as c_int
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*alias.offset(i as isize)
                    as c_uchar
                    as c_int
                    as isize) as c_int;
            }
            __res
        }) as c_char;
        if upper[i as usize] as c_int == 0 as c_int {
            break;
        }
        i += 1;
    }
    upper[i as usize] = 0 as c_char;
    i = 0 as c_int;
    while i < xmlCharEncodingAliasesNb {
        if strcmp(
            (*xmlCharEncodingAliases.offset(i as isize)).alias,
            &raw mut upper as *mut c_char,
        ) == 0
        {
            return (*xmlCharEncodingAliases.offset(i as isize)).name;
        }
        i += 1;
    }
    return ::core::ptr::null::<c_char>();
}
#[inline]
pub unsafe fn xmlAddEncodingAlias(
    mut name: *const c_char,
    mut alias: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    let mut upper: [c_char; 100] = [0; 100];
    let mut nameCopy: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut aliasCopy: *mut c_char = ::core::ptr::null_mut::<c_char>();
    if name.is_null() || alias.is_null() {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < 99 as c_int {
        upper[i as usize] = ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_uchar>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *alias.offset(i as isize) as c_uchar as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = toupper(
                        *alias.offset(i as isize) as c_uchar as c_int
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*alias.offset(i as isize)
                    as c_uchar
                    as c_int
                    as isize) as c_int;
            }
            __res
        }) as c_char;
        if upper[i as usize] as c_int == 0 as c_int {
            break;
        }
        i += 1;
    }
    upper[i as usize] = 0 as c_char;
    if xmlCharEncodingAliasesNb >= xmlCharEncodingAliasesMax {
        let mut tmp: xmlCharEncodingAliasPtr = ::core::ptr::null_mut::<xmlCharEncodingAlias>();
        let mut newSize: size_t = (if xmlCharEncodingAliasesMax != 0 {
            xmlCharEncodingAliasesMax * 2 as c_int
        } else {
            20 as c_int
        }) as size_t;
        tmp = xmlRealloc.expect("non-null function pointer")(
            xmlCharEncodingAliases as *mut c_void,
            newSize.wrapping_mul(::core::mem::size_of::<xmlCharEncodingAlias>() as size_t),
        ) as xmlCharEncodingAliasPtr;
        if tmp.is_null() {
            return -(1 as c_int);
        }
        xmlCharEncodingAliases = tmp;
        xmlCharEncodingAliasesMax = newSize as c_int;
    }
    i = 0 as c_int;
    while i < xmlCharEncodingAliasesNb {
        if strcmp(
            (*xmlCharEncodingAliases.offset(i as isize)).alias,
            &raw mut upper as *mut c_char,
        ) == 0
        {
            nameCopy = xmlMemStrdup.expect("non-null function pointer")(name);
            if nameCopy.is_null() {
                return -(1 as c_int);
            }
            xmlFree.expect("non-null function pointer")(
                (*xmlCharEncodingAliases.offset(i as isize)).name as *mut c_char
                    as *mut c_void,
            );
            let ref mut fresh62 = (*xmlCharEncodingAliases.offset(i as isize)).name;
            *fresh62 = nameCopy;
            return 0 as c_int;
        }
        i += 1;
    }
    nameCopy = xmlMemStrdup.expect("non-null function pointer")(name);
    if nameCopy.is_null() {
        return -(1 as c_int);
    }
    aliasCopy = xmlMemStrdup.expect("non-null function pointer")(
        &raw mut upper as *mut c_char,
    );
    if aliasCopy.is_null() {
        xmlFree.expect("non-null function pointer")(nameCopy as *mut c_void);
        return -(1 as c_int);
    }
    let ref mut fresh63 = (*xmlCharEncodingAliases.offset(xmlCharEncodingAliasesNb as isize)).name;
    *fresh63 = nameCopy;
    let ref mut fresh64 = (*xmlCharEncodingAliases.offset(xmlCharEncodingAliasesNb as isize)).alias;
    *fresh64 = aliasCopy;
    xmlCharEncodingAliasesNb += 1;
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlDelEncodingAlias(
    mut alias: *const c_char,
) -> c_int {
    let mut i: c_int = 0;
    if alias.is_null() {
        return -(1 as c_int);
    }
    if xmlCharEncodingAliases.is_null() {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < xmlCharEncodingAliasesNb {
        if strcmp((*xmlCharEncodingAliases.offset(i as isize)).alias, alias) == 0 {
            xmlFree.expect("non-null function pointer")(
                (*xmlCharEncodingAliases.offset(i as isize)).name as *mut c_char
                    as *mut c_void,
            );
            xmlFree.expect("non-null function pointer")(
                (*xmlCharEncodingAliases.offset(i as isize)).alias as *mut c_char
                    as *mut c_void,
            );
            xmlCharEncodingAliasesNb -= 1;
            memmove(
                xmlCharEncodingAliases.offset(i as isize) as *mut xmlCharEncodingAlias
                    as *mut c_void,
                xmlCharEncodingAliases.offset((i + 1 as c_int) as isize)
                    as *mut xmlCharEncodingAlias as *const c_void,
                (::core::mem::size_of::<xmlCharEncodingAlias>() as size_t)
                    .wrapping_mul((xmlCharEncodingAliasesNb - i) as size_t),
            );
            return 0 as c_int;
        }
        i += 1;
    }
    return -(1 as c_int);
}
#[inline]
pub unsafe fn xmlParseCharEncoding(
    mut name: *const c_char,
) -> xmlCharEncoding {
    let mut alias: *const c_char = ::core::ptr::null::<c_char>();
    let mut upper: [c_char; 500] = [0; 500];
    let mut i: c_int = 0;
    if name.is_null() {
        return XML_CHAR_ENCODING_NONE;
    }
    alias = xmlGetEncodingAlias(name);
    if !alias.is_null() {
        name = alias;
    }
    i = 0 as c_int;
    while i < 499 as c_int {
        upper[i as usize] = ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_uchar>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *name.offset(i as isize) as c_uchar as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = toupper(
                        *name.offset(i as isize) as c_uchar as c_int
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*name.offset(i as isize)
                    as c_uchar
                    as c_int
                    as isize) as c_int;
            }
            __res
        }) as c_char;
        if upper[i as usize] as c_int == 0 as c_int {
            break;
        }
        i += 1;
    }
    upper[i as usize] = 0 as c_char;
    if strcmp(
        &raw mut upper as *mut c_char,
        b"\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_NONE;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"UTF-8\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UTF8;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"UTF8\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UTF8;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"UTF-16\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UTF16LE;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"UTF16\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UTF16LE;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-10646-UCS-2\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UCS2;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"UCS-2\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UCS2;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"UCS2\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UCS2;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-10646-UCS-4\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UCS4LE;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"UCS-4\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UCS4LE;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"UCS4\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_UCS4LE;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-1\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_1;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-LATIN-1\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_1;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO LATIN 1\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_1;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-2\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_2;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-LATIN-2\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_2;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO LATIN 2\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_2;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-3\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_3;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-4\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_4;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-5\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_5;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-6\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_6;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-7\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_7;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-8\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_8;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-8859-9\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_8859_9;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"ISO-2022-JP\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_2022_JP;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"SHIFT_JIS\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_SHIFT_JIS;
    }
    if strcmp(
        &raw mut upper as *mut c_char,
        b"EUC-JP\0" as *const u8 as *const c_char,
    ) == 0
    {
        return XML_CHAR_ENCODING_EUC_JP;
    }
    return XML_CHAR_ENCODING_ERROR;
}
#[inline]
pub fn xmlGetCharEncodingName(
    mut enc: xmlCharEncoding,
) -> *const c_char { {
    match enc as c_int {
        -1 => return ::core::ptr::null::<c_char>(),
        0 => return ::core::ptr::null::<c_char>(),
        1 => return b"UTF-8\0" as *const u8 as *const c_char,
        2 => return b"UTF-16\0" as *const u8 as *const c_char,
        3 => return b"UTF-16\0" as *const u8 as *const c_char,
        6 => return b"EBCDIC\0" as *const u8 as *const c_char,
        4 => return b"ISO-10646-UCS-4\0" as *const u8 as *const c_char,
        5 => return b"ISO-10646-UCS-4\0" as *const u8 as *const c_char,
        7 => return b"ISO-10646-UCS-4\0" as *const u8 as *const c_char,
        8 => return b"ISO-10646-UCS-4\0" as *const u8 as *const c_char,
        9 => return b"ISO-10646-UCS-2\0" as *const u8 as *const c_char,
        10 => return b"ISO-8859-1\0" as *const u8 as *const c_char,
        11 => return b"ISO-8859-2\0" as *const u8 as *const c_char,
        12 => return b"ISO-8859-3\0" as *const u8 as *const c_char,
        13 => return b"ISO-8859-4\0" as *const u8 as *const c_char,
        14 => return b"ISO-8859-5\0" as *const u8 as *const c_char,
        15 => return b"ISO-8859-6\0" as *const u8 as *const c_char,
        16 => return b"ISO-8859-7\0" as *const u8 as *const c_char,
        17 => return b"ISO-8859-8\0" as *const u8 as *const c_char,
        18 => return b"ISO-8859-9\0" as *const u8 as *const c_char,
        19 => return b"ISO-2022-JP\0" as *const u8 as *const c_char,
        20 => return b"Shift-JIS\0" as *const u8 as *const c_char,
        21 => return b"EUC-JP\0" as *const u8 as *const c_char,
        22 => return ::core::ptr::null::<c_char>(),
        _ => {}
    }
    return ::core::ptr::null::<c_char>();
} }
static mut defaultHandlers: [xmlCharEncodingHandler; 22] = [
        _xmlCharEncodingHandler {
            name: b"UTF-8\0" as *const u8 as *const c_char as *mut c_char,
            input: Some(
                UTF8ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"UTF-16LE\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                UTF16LEToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToUTF16LE
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"UTF-16BE\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                UTF16BEToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToUTF16BE
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"UTF-16\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                UTF16LEToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToUTF16
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-1\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                isolat1ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8Toisolat1
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ASCII\0" as *const u8 as *const c_char as *mut c_char,
            input: Some(
                asciiToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8Toascii
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"US-ASCII\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                asciiToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8Toascii
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"HTML\0" as *const u8 as *const c_char as *mut c_char,
            input: None,
            output: Some(
                UTF8ToHtml
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-2\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_2ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_2
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-3\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_3ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_3
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-4\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_4ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_4
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-5\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_5ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_5
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-6\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_6ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_6
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-7\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_7ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_7
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-8\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_8ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-9\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_9ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_9
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-10\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_10ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_10
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-11\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_11ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_11
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-13\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_13ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_13
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-14\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_14ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_14
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-15\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_15ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_15
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
        _xmlCharEncodingHandler {
            name: b"ISO-8859-16\0" as *const u8 as *const c_char
                as *mut c_char,
            input: Some(
                ISO8859_16ToUTF8
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
            output: Some(
                UTF8ToISO8859_16
                    as unsafe extern "C" fn(
                        *mut c_uchar,
                        *mut c_int,
                        *const c_uchar,
                        *mut c_int,
                    ) -> c_int,
            ),
        },
    ];
pub const NUM_DEFAULT_HANDLERS: usize = (::core::mem::size_of::<[xmlCharEncodingHandler; 22]>()
    as usize)
    .wrapping_div(::core::mem::size_of::<xmlCharEncodingHandler>() as usize);
static mut xmlUTF16LEHandler: *const xmlCharEncodingHandler =
    ::core::ptr::null::<xmlCharEncodingHandler>();
static mut xmlUTF16BEHandler: *const xmlCharEncodingHandler =
    ::core::ptr::null::<xmlCharEncodingHandler>();
pub const MAX_ENCODING_HANDLERS: c_int = 50 as c_int;
static mut handlers: *mut xmlCharEncodingHandlerPtr =
    ::core::ptr::null::<xmlCharEncodingHandlerPtr>() as *mut xmlCharEncodingHandlerPtr;
static mut nbCharEncodingHandler: c_int = 0 as c_int;
#[inline]
pub unsafe fn xmlNewCharEncodingHandler(
    mut name: *const c_char,
    mut input: xmlCharEncodingInputFunc,
    mut output: xmlCharEncodingOutputFunc,
) -> xmlCharEncodingHandlerPtr {
    let mut handler: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    let mut alias: *const c_char = ::core::ptr::null::<c_char>();
    let mut upper: [c_char; 500] = [0; 500];
    let mut i: c_int = 0;
    let mut up: *mut c_char = ::core::ptr::null_mut::<c_char>();
    alias = xmlGetEncodingAlias(name);
    if !alias.is_null() {
        name = alias;
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    }
    i = 0 as c_int;
    while i < 499 as c_int {
        upper[i as usize] = ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_uchar>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *name.offset(i as isize) as c_uchar as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = toupper(
                        *name.offset(i as isize) as c_uchar as c_int
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*name.offset(i as isize)
                    as c_uchar
                    as c_int
                    as isize) as c_int;
            }
            __res
        }) as c_char;
        if upper[i as usize] as c_int == 0 as c_int {
            break;
        }
        i += 1;
    }
    upper[i as usize] = 0 as c_char;
    up = xmlMemStrdup.expect("non-null function pointer")(
        &raw mut upper as *mut c_char,
    );
    if up.is_null() {
        return ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    }
    handler = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<
        xmlCharEncodingHandler,
    >() as size_t) as xmlCharEncodingHandlerPtr;
    if handler.is_null() {
        xmlFree.expect("non-null function pointer")(up as *mut c_void);
        return ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    }
    memset(
        handler as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlCharEncodingHandler>() as size_t,
    );
    (*handler).input = input;
    (*handler).output = output;
    (*handler).name = up;
    xmlRegisterCharEncodingHandler(handler);
    return handler;
}
#[inline]
pub fn xmlInitCharEncodingHandlers() { {
    xmlInitParser();
} }
#[inline]
pub fn xmlInitEncodingInternal() { unsafe {
    let mut tst: c_ushort = 0x1234 as c_ushort;
    let mut ptr: *mut c_uchar = &raw mut tst as *mut c_uchar;
    if *ptr as c_int == 0x12 as c_int {
        xmlLittleEndian = 0 as c_int;
    } else {
        xmlLittleEndian = 1 as c_int;
    };
} }
#[inline]
pub fn xmlCleanupCharEncodingHandlers() { unsafe {
    xmlCleanupEncodingAliases();
    if handlers.is_null() {
        return;
    }
    while nbCharEncodingHandler > 0 as c_int {
        nbCharEncodingHandler -= 1;
        if !(*handlers.offset(nbCharEncodingHandler as isize)).is_null() {
            if !(**handlers.offset(nbCharEncodingHandler as isize))
                .name
                .is_null()
            {
                xmlFree.expect("non-null function pointer")(
                    (**handlers.offset(nbCharEncodingHandler as isize)).name
                        as *mut c_void,
                );
            }
            xmlFree.expect("non-null function pointer")(
                *handlers.offset(nbCharEncodingHandler as isize) as *mut c_void,
            );
        }
    }
    xmlFree.expect("non-null function pointer")(handlers as *mut c_void);
    handlers = ::core::ptr::null_mut::<xmlCharEncodingHandlerPtr>();
    nbCharEncodingHandler = 0 as c_int;
} }
#[inline]
pub fn xmlRegisterCharEncodingHandler(mut handler: xmlCharEncodingHandlerPtr) { unsafe {
    let mut current_block: u64;
    if handler.is_null() {
        return;
    }
    if handlers.is_null() {
        handlers = xmlMalloc.expect("non-null function pointer")(
            (MAX_ENCODING_HANDLERS as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlCharEncodingHandlerPtr>() as size_t),
        ) as *mut xmlCharEncodingHandlerPtr;
        if handlers.is_null() {
            current_block = 9452249753465017513;
        } else {
            current_block = 17179679302217393232;
        }
    } else {
        current_block = 17179679302217393232;
    }
    match current_block {
        17179679302217393232 => {
            if !(nbCharEncodingHandler >= MAX_ENCODING_HANDLERS) {
                let fresh0 = nbCharEncodingHandler;
                nbCharEncodingHandler = nbCharEncodingHandler + 1;
                let ref mut fresh1 = *handlers.offset(fresh0 as isize);
                *fresh1 = handler;
                return;
            }
        }
        _ => {}
    }
    if !handler.is_null() {
        if !(*handler).name.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*handler).name as *mut c_void,
            );
        }
        xmlFree.expect("non-null function pointer")(handler as *mut c_void);
    }
} }
#[inline]
pub fn xmlGetCharEncodingHandler(
    mut enc: xmlCharEncoding,
) -> xmlCharEncodingHandlerPtr { unsafe {
    let mut handler: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    match enc as c_int {
        -1 => return ::core::ptr::null_mut::<xmlCharEncodingHandler>(),
        0 => return ::core::ptr::null_mut::<xmlCharEncodingHandler>(),
        1 => return ::core::ptr::null_mut::<xmlCharEncodingHandler>(),
        2 => return xmlUTF16LEHandler as xmlCharEncodingHandlerPtr,
        3 => return xmlUTF16BEHandler as xmlCharEncodingHandlerPtr,
        6 => {
            handler =
                xmlFindCharEncodingHandler(b"EBCDIC\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
            handler =
                xmlFindCharEncodingHandler(b"ebcdic\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
            handler = xmlFindCharEncodingHandler(
                b"EBCDIC-US\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
            handler =
                xmlFindCharEncodingHandler(b"IBM-037\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
        }
        5 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-10646-UCS-4\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
            handler =
                xmlFindCharEncodingHandler(b"UCS-4\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
            handler =
                xmlFindCharEncodingHandler(b"UCS4\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
        }
        4 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-10646-UCS-4\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
            handler =
                xmlFindCharEncodingHandler(b"UCS-4\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
            handler =
                xmlFindCharEncodingHandler(b"UCS4\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
        }
        9 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-10646-UCS-2\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
            handler =
                xmlFindCharEncodingHandler(b"UCS-2\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
            handler =
                xmlFindCharEncodingHandler(b"UCS2\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
        }
        10 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-1\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        11 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-2\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        12 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-3\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        13 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-4\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        14 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-5\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        15 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-6\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        16 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-7\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        17 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-8\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        18 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-8859-9\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        19 => {
            handler = xmlFindCharEncodingHandler(
                b"ISO-2022-JP\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        20 => {
            handler = xmlFindCharEncodingHandler(
                b"SHIFT-JIS\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
            handler = xmlFindCharEncodingHandler(
                b"SHIFT_JIS\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
            handler = xmlFindCharEncodingHandler(
                b"Shift_JIS\0" as *const u8 as *const c_char,
            );
            if !handler.is_null() {
                return handler;
            }
        }
        21 => {
            handler =
                xmlFindCharEncodingHandler(b"EUC-JP\0" as *const u8 as *const c_char);
            if !handler.is_null() {
                return handler;
            }
        }
        7 | 8 | _ => {}
    }
    return ::core::ptr::null_mut::<xmlCharEncodingHandler>();
} }
#[inline]
pub unsafe fn xmlFindCharEncodingHandler(
    mut name: *const c_char,
) -> xmlCharEncodingHandlerPtr {
    let mut nalias: *const c_char = ::core::ptr::null::<c_char>();
    let mut norig: *const c_char = ::core::ptr::null::<c_char>();
    let mut alias: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut upper: [c_char; 100] = [0; 100];
    let mut i: c_int = 0;
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    }
    if *name.offset(0 as c_int as isize) as c_int
        == 0 as c_int
    {
        return ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    }
    norig = name;
    nalias = xmlGetEncodingAlias(name);
    if !nalias.is_null() {
        name = nalias;
    }
    i = 0 as c_int;
    while i < 99 as c_int {
        upper[i as usize] = ({
            let mut __res: c_int = 0;
            if ::core::mem::size_of::<c_uchar>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: c_int =
                        *name.offset(i as isize) as c_uchar as c_int;
                    __res =
                        (if __c < -(128 as c_int) || __c > 255 as c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as c_int;
                } else {
                    __res = toupper(
                        *name.offset(i as isize) as c_uchar as c_int
                    );
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*name.offset(i as isize)
                    as c_uchar
                    as c_int
                    as isize) as c_int;
            }
            __res
        }) as c_char;
        if upper[i as usize] as c_int == 0 as c_int {
            break;
        }
        i += 1;
    }
    upper[i as usize] = 0 as c_char;
    i = 0 as c_int;
    while i < NUM_DEFAULT_HANDLERS as c_int {
        if strcmp(
            &raw mut upper as *mut c_char,
            defaultHandlers[i as usize].name,
        ) == 0 as c_int
        {
            return (&raw const defaultHandlers as *const xmlCharEncodingHandler).offset(i as isize)
                as *const xmlCharEncodingHandler as xmlCharEncodingHandlerPtr;
        }
        i += 1;
    }
    if !handlers.is_null() {
        i = 0 as c_int;
        while i < nbCharEncodingHandler {
            if strcmp(
                &raw mut upper as *mut c_char,
                (**handlers.offset(i as isize)).name,
            ) == 0
            {
                return *handlers.offset(i as isize);
            }
            i += 1;
        }
    }
    alias = xmlParseCharEncoding(norig);
    if alias as c_int != XML_CHAR_ENCODING_ERROR as c_int {
        let mut canon: *const c_char = ::core::ptr::null::<c_char>();
        canon = xmlGetCharEncodingName(alias);
        if !canon.is_null() && strcmp(name, canon) != 0 {
            return xmlFindCharEncodingHandler(canon);
        }
    }
    return ::core::ptr::null_mut::<xmlCharEncodingHandler>();
}
fn xmlEncConvertError(mut code: c_int) -> c_int { {
    let mut ret: c_int = 0;
    match code {
        0 => {
            ret = XML_ERR_OK as c_int;
        }
        -2 => {
            ret = XML_ERR_INVALID_ENCODING as c_int;
        }
        -5 => {
            ret = XML_ERR_NO_MEMORY as c_int;
        }
        _ => {
            ret = XML_ERR_INTERNAL_ERROR as c_int;
        }
    }
    return ret;
} }
#[inline]
pub unsafe fn xmlEncInputChunk(
    mut handler: *mut xmlCharEncodingHandler,
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    let handler_view: &xmlCharEncodingHandler = unsafe { &*handler };
    let mut ret: c_int = 0;
    if handler_view.input.is_some() {
        let mut oldinlen: c_int = *inlen;
        ret = handler_view.input.expect("non-null function pointer")(out, outlen, in_0, inlen);
        if ret >= 0 as c_int {
            if *inlen < oldinlen {
                if *outlen > 0 as c_int {
                    ret = XML_ENC_ERR_SPACE as c_int;
                } else {
                    ret = XML_ENC_ERR_PARTIAL as c_int;
                }
            } else {
                ret = XML_ENC_ERR_SUCCESS as c_int;
            }
        }
    } else {
        *outlen = 0 as c_int;
        *inlen = 0 as c_int;
        ret = XML_ENC_ERR_INTERNAL as c_int;
    }
    if ret == XML_ENC_ERR_PARTIAL as c_int {
        ret = XML_ENC_ERR_SUCCESS as c_int;
    }
    return ret;
}
unsafe fn xmlEncOutputChunk(
    mut handler: *mut xmlCharEncodingHandler,
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    let handler_view: &xmlCharEncodingHandler = unsafe { &*handler };
    let mut ret: c_int = 0;
    if handler_view.output.is_some() {
        let mut oldinlen: c_int = *inlen;
        ret = handler_view.output.expect("non-null function pointer")(out, outlen, in_0, inlen);
        if ret >= 0 as c_int {
            if *inlen < oldinlen {
                if *outlen > 0 as c_int {
                    ret = XML_ENC_ERR_SPACE as c_int;
                } else {
                    ret = XML_ENC_ERR_PARTIAL as c_int;
                }
            } else {
                ret = XML_ENC_ERR_SUCCESS as c_int;
            }
        }
    } else {
        *outlen = 0 as c_int;
        *inlen = 0 as c_int;
        ret = XML_ENC_ERR_INTERNAL as c_int;
    }
    if ret == XML_ENC_ERR_PARTIAL as c_int {
        ret = XML_ENC_ERR_INTERNAL as c_int;
    }
    return ret;
}
#[inline]
pub unsafe fn xmlCharEncFirstLine(
    mut handler: *mut xmlCharEncodingHandler,
    mut out: xmlBufferPtr,
    mut in_0: xmlBufferPtr,
) -> c_int {
    return xmlCharEncInFunc(handler, out, in_0);
}
#[no_mangle]
pub extern "C" fn xmlCharEncInput(mut input: xmlParserInputBufferPtr) -> c_int { unsafe {
    let mut ret: c_int = 0;
    let mut avail: size_t = 0;
    let mut toconv: size_t = 0;
    let mut c_in: c_int = 0;
    let mut c_out: c_int = 0;
    let mut in_0: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut out: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut inData: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut inTotal: size_t = 0 as size_t;
    if input.is_null()
        || (*input).encoder.is_null()
        || (*input).buffer.is_null()
        || (*input).raw.is_null()
    {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    out = (*input).buffer;
    in_0 = (*input).raw;
    toconv = xmlBufUse(in_0);
    if toconv == 0 as size_t {
        return 0 as c_int;
    }
    inData = xmlBufContent(in_0 as *const xmlBuf);
    inTotal = 0 as size_t;
    loop {
        c_in = (if toconv > (INT_MAX / 2 as c_int) as size_t {
            (INT_MAX / 2 as c_int) as size_t
        } else {
            toconv
        }) as c_int;
        avail = xmlBufAvail(out);
        if avail > INT_MAX as size_t {
            avail = INT_MAX as size_t;
        }
        if avail < 4096 as size_t {
            if xmlBufGrow(out, 4096 as c_int) < 0 as c_int {
                (*input).error = XML_ERR_NO_MEMORY as c_int;
                return XML_ENC_ERR_MEMORY as c_int;
            }
            avail = xmlBufAvail(out);
        }
        c_in = toconv as c_int;
        c_out = avail as c_int;
        ret = xmlEncInputChunk(
            (*input).encoder as *mut xmlCharEncodingHandler,
            xmlBufEnd(out) as *mut c_uchar,
            &raw mut c_out,
            inData as *const c_uchar,
            &raw mut c_in,
        );
        inTotal = (inTotal as c_ulong).wrapping_add(c_in as c_ulong)
            as size_t as size_t;
        inData = inData.offset(c_in as isize);
        toconv = (toconv as c_ulong).wrapping_sub(c_in as c_ulong)
            as size_t as size_t;
        xmlBufAddLen(out, c_out as size_t);
        if !(ret == XML_ENC_ERR_SPACE as c_int) {
            break;
        }
    }
    xmlBufShrink(in_0, inTotal);
    if (*input).rawconsumed > ULONG_MAX.wrapping_sub(c_in as c_ulong) {
        (*input).rawconsumed = ULONG_MAX;
    } else {
        (*input).rawconsumed = (*input)
            .rawconsumed
            .wrapping_add(c_in as c_ulong);
    }
    if c_out == 0 as c_int && ret != 0 as c_int {
        if (*input).error == 0 as c_int {
            (*input).error = xmlEncConvertError(ret);
        }
        return ret;
    }
    return c_out;
} }
#[inline]
pub unsafe fn xmlCharEncInFunc(
    mut handler: *mut xmlCharEncodingHandler,
    mut out: xmlBufferPtr,
    mut in_0: xmlBufferPtr,
) -> c_int {
    let mut ret: c_int = 0;
    let mut written: c_int = 0;
    let mut toconv: c_int = 0;
    if handler.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if out.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if in_0.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    toconv = (*in_0).use_0 as c_int;
    if toconv == 0 as c_int {
        return 0 as c_int;
    }
    written = (*out)
        .size
        .wrapping_sub((*out).use_0)
        .wrapping_sub(1 as c_uint) as c_int;
    if toconv * 2 as c_int >= written {
        xmlBufferGrow(
            out,
            (*out)
                .size
                .wrapping_add((toconv * 2 as c_int) as c_uint),
        );
        written = (*out)
            .size
            .wrapping_sub((*out).use_0)
            .wrapping_sub(1 as c_uint) as c_int;
    }
    ret = xmlEncInputChunk(
        handler,
        (*out).content.offset((*out).use_0 as isize) as *mut c_uchar,
        &raw mut written,
        (*in_0).content,
        &raw mut toconv,
    );
    xmlBufferShrink(in_0, toconv as c_uint);
    (*out).use_0 = (*out).use_0.wrapping_add(written as c_uint);
    *(*out).content.offset((*out).use_0 as isize) = 0 as xmlChar;
    return if written != 0 { written } else { ret };
}
#[no_mangle]
pub extern "C" fn xmlCharEncOutput(
    mut output: xmlOutputBufferPtr,
    mut init: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    let mut written: size_t = 0;
    let mut writtentot: c_int = 0 as c_int;
    let mut toconv: size_t = 0;
    let mut c_in: c_int = 0;
    let mut c_out: c_int = 0;
    let mut in_0: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut out: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    if output.is_null()
        || (*output).encoder.is_null()
        || (*output).buffer.is_null()
        || (*output).conv.is_null()
    {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    out = (*output).conv;
    in_0 = (*output).buffer;
    loop {
        written = xmlBufAvail(out);
        if init != 0 {
            c_in = 0 as c_int;
            c_out = written as c_int;
            xmlEncOutputChunk(
                (*output).encoder as *mut xmlCharEncodingHandler,
                xmlBufEnd(out) as *mut c_uchar,
                &raw mut c_out,
                ::core::ptr::null::<c_uchar>(),
                &raw mut c_in,
            );
            xmlBufAddLen(out, c_out as size_t);
            return c_out;
        }
        toconv = xmlBufUse(in_0);
        if toconv > (64 as c_int * 1024 as c_int) as size_t {
            toconv = (64 as c_int * 1024 as c_int) as size_t;
        }
        if toconv.wrapping_mul(4 as size_t) >= written {
            xmlBufGrow(out, toconv.wrapping_mul(4 as size_t) as c_int);
            written = xmlBufAvail(out);
        }
        if written > (256 as c_int * 1024 as c_int) as size_t {
            written = (256 as c_int * 1024 as c_int) as size_t;
        }
        c_in = toconv as c_int;
        c_out = written as c_int;
        ret = xmlEncOutputChunk(
            (*output).encoder as *mut xmlCharEncodingHandler,
            xmlBufEnd(out) as *mut c_uchar,
            &raw mut c_out,
            xmlBufContent(in_0 as *const xmlBuf),
            &raw mut c_in,
        );
        xmlBufShrink(in_0, c_in as size_t);
        xmlBufAddLen(out, c_out as size_t);
        writtentot += c_out;
        if ret == XML_ENC_ERR_SPACE as c_int {
            continue;
        }
        if !(ret == XML_ENC_ERR_INPUT as c_int) {
            break;
        }
        let mut charref: [xmlChar; 20] = [0; 20];
        let mut len: c_int = xmlBufUse(in_0) as c_int;
        let mut content: *mut xmlChar = xmlBufContent(in_0 as *const xmlBuf);
        let mut cur: c_int = 0;
        let mut charrefLen: c_int = 0;
        cur = xmlGetUTF8Char(content, &raw mut len);
        if cur <= 0 as c_int {
            break;
        }
        charrefLen = snprintf(
            (&raw mut charref as *mut xmlChar).offset(0 as c_int as isize)
                as *mut xmlChar as *mut c_char,
            ::core::mem::size_of::<[xmlChar; 20]>() as size_t,
            b"&#%d;\0" as *const u8 as *const c_char,
            cur,
        );
        xmlBufShrink(in_0, len as size_t);
        xmlBufGrow(out, charrefLen * 4 as c_int);
        c_out = xmlBufAvail(out) as c_int;
        c_in = charrefLen;
        ret = xmlEncOutputChunk(
            (*output).encoder as *mut xmlCharEncodingHandler,
            xmlBufEnd(out) as *mut c_uchar,
            &raw mut c_out,
            &raw mut charref as *mut xmlChar,
            &raw mut c_in,
        );
        if ret < 0 as c_int || c_in != charrefLen {
            ret = XML_ENC_ERR_INTERNAL as c_int;
            break;
        } else {
            xmlBufAddLen(out, c_out as size_t);
            writtentot += c_out;
        }
    }
    if writtentot <= 0 as c_int && ret != 0 as c_int {
        if (*output).error == 0 as c_int {
            (*output).error = xmlEncConvertError(ret);
        }
        return ret;
    }
    return writtentot;
} }
#[inline]
pub unsafe fn xmlCharEncOutFunc(
    mut handler: *mut xmlCharEncodingHandler,
    mut out: xmlBufferPtr,
    mut in_0: xmlBufferPtr,
) -> c_int {
    let mut ret: c_int = 0;
    let mut written: c_int = 0;
    let mut writtentot: c_int = 0 as c_int;
    let mut toconv: c_int = 0;
    if handler.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if out.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    loop {
        written = (*out).size.wrapping_sub((*out).use_0) as c_int;
        if written > 0 as c_int {
            written -= 1;
        }
        if in_0.is_null() {
            toconv = 0 as c_int;
            xmlEncOutputChunk(
                handler,
                (*out).content.offset((*out).use_0 as isize) as *mut c_uchar,
                &raw mut written,
                ::core::ptr::null::<c_uchar>(),
                &raw mut toconv,
            );
            (*out).use_0 = (*out).use_0.wrapping_add(written as c_uint);
            *(*out).content.offset((*out).use_0 as isize) = 0 as xmlChar;
            return 0 as c_int;
        }
        toconv = (*in_0).use_0 as c_int;
        if toconv * 4 as c_int >= written {
            xmlBufferGrow(
                out,
                (toconv * 4 as c_int) as c_uint,
            );
            written = (*out)
                .size
                .wrapping_sub((*out).use_0)
                .wrapping_sub(1 as c_uint) as c_int;
        }
        ret = xmlEncOutputChunk(
            handler,
            (*out).content.offset((*out).use_0 as isize) as *mut c_uchar,
            &raw mut written,
            (*in_0).content,
            &raw mut toconv,
        );
        xmlBufferShrink(in_0, toconv as c_uint);
        (*out).use_0 = (*out).use_0.wrapping_add(written as c_uint);
        writtentot += written;
        *(*out).content.offset((*out).use_0 as isize) = 0 as xmlChar;
        if ret == XML_ENC_ERR_SPACE as c_int {
            continue;
        }
        if !(ret == XML_ENC_ERR_INPUT as c_int) {
            break;
        }
        let mut charref: [xmlChar; 20] = [0; 20];
        let mut len: c_int = (*in_0).use_0 as c_int;
        let mut utf: *const xmlChar = (*in_0).content as *const xmlChar;
        let mut cur: c_int = 0;
        let mut charrefLen: c_int = 0;
        cur = xmlGetUTF8Char(utf as *const c_uchar, &raw mut len);
        if cur <= 0 as c_int {
            return ret;
        }
        charrefLen = snprintf(
            (&raw mut charref as *mut xmlChar).offset(0 as c_int as isize)
                as *mut xmlChar as *mut c_char,
            ::core::mem::size_of::<[xmlChar; 20]>() as size_t,
            b"&#%d;\0" as *const u8 as *const c_char,
            cur,
        );
        xmlBufferShrink(in_0, len as c_uint);
        xmlBufferGrow(
            out,
            (charrefLen * 4 as c_int) as c_uint,
        );
        written = (*out)
            .size
            .wrapping_sub((*out).use_0)
            .wrapping_sub(1 as c_uint) as c_int;
        toconv = charrefLen;
        ret = xmlEncOutputChunk(
            handler,
            (*out).content.offset((*out).use_0 as isize) as *mut c_uchar,
            &raw mut written,
            &raw mut charref as *mut xmlChar,
            &raw mut toconv,
        );
        if ret < 0 as c_int || toconv != charrefLen {
            return XML_ENC_ERR_INTERNAL as c_int;
        }
        (*out).use_0 = (*out).use_0.wrapping_add(written as c_uint);
        writtentot += written;
        *(*out).content.offset((*out).use_0 as isize) = 0 as xmlChar;
    }
    return if writtentot != 0 { writtentot } else { ret };
}
#[inline]
pub unsafe fn xmlCharEncCloseFunc(
    mut handler: *mut xmlCharEncodingHandler,
) -> c_int {
    let mut ret: c_int = 0 as c_int;
    let mut tofree: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    if handler.is_null() {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < NUM_DEFAULT_HANDLERS as c_int {
        if handler
            == (&raw const defaultHandlers as *const xmlCharEncodingHandler).offset(i as isize)
                as *const xmlCharEncodingHandler as *mut xmlCharEncodingHandler
        {
            return 0 as c_int;
        }
        i += 1;
    }
    if !handlers.is_null() {
        i = 0 as c_int;
        while i < nbCharEncodingHandler {
            if handler == *handlers.offset(i as isize) {
                return 0 as c_int;
            }
            i += 1;
        }
    }
    if tofree != 0 {
        if !(*handler).name.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*handler).name as *mut c_void,
            );
        }
        (*handler).name = ::core::ptr::null_mut::<c_char>();
        xmlFree.expect("non-null function pointer")(handler as *mut c_void);
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn xmlByteConsumed(mut ctxt: xmlParserCtxtPtr) -> c_long { unsafe {
    let mut in_0: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    if ctxt.is_null() {
        return -(1 as c_int) as c_long;
    }
    in_0 = (*ctxt).input;
    if in_0.is_null() {
        return -(1 as c_int) as c_long;
    }
    if !(*in_0).buf.is_null() && !(*(*in_0).buf).encoder.is_null() {
        let mut unused: c_uint = 0 as c_uint;
        let mut handler: *mut xmlCharEncodingHandler =
            (*(*in_0).buf).encoder as *mut xmlCharEncodingHandler;
        if (*in_0).end.offset_from((*in_0).cur) as c_long > 0 as c_long {
            let mut convbuf: [c_uchar; 32000] = [0; 32000];
            let mut cur: *const c_uchar = (*in_0).cur as *const c_uchar;
            let mut toconv: c_int =
                (*in_0).end.offset_from((*in_0).cur) as c_long as c_int;
            let mut written: c_int = 32000 as c_int;
            let mut ret: c_int = 0;
            loop {
                toconv = (*in_0).end.offset_from(cur) as c_long as c_int;
                written = 32000 as c_int;
                ret = xmlEncOutputChunk(
                    handler,
                    (&raw mut convbuf as *mut c_uchar)
                        .offset(0 as c_int as isize)
                        as *mut c_uchar,
                    &raw mut written,
                    cur,
                    &raw mut toconv,
                );
                if ret != XML_ENC_ERR_SUCCESS as c_int
                    && ret != XML_ENC_ERR_SPACE as c_int
                {
                    return -(1 as c_int) as c_long;
                }
                unused = unused.wrapping_add(written as c_uint);
                cur = cur.offset(toconv as isize);
                if !(ret == XML_ENC_ERR_SPACE as c_int) {
                    break;
                }
            }
        }
        if (*(*in_0).buf).rawconsumed < unused as c_ulong {
            return -(1 as c_int) as c_long;
        }
        return (*(*in_0).buf)
            .rawconsumed
            .wrapping_sub(unused as c_ulong) as c_long;
    }
    return (*in_0).consumed.wrapping_add(
        (*in_0).cur.offset_from((*in_0).base) as c_long as c_ulong
    ) as c_long;
} }
unsafe fn UTF8ToISO8859x(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
    xlattable: *const c_uchar,
) -> c_int {
    let mut outstart: *const c_uchar = out;
    let mut inend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut instart: *const c_uchar = in_0;
    let mut processed: *const c_uchar = in_0;
    if out.is_null() || outlen.is_null() || inlen.is_null() || xlattable.is_null() {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    if in_0.is_null() {
        *outlen = 0 as c_int;
        *inlen = 0 as c_int;
        return 0 as c_int;
    }
    inend = in_0.offset(*inlen as isize);
    while in_0 < inend {
        let fresh2 = in_0;
        in_0 = in_0.offset(1);
        let mut d: c_uchar = *fresh2;
        if (d as c_int) < 0x80 as c_int {
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 = d;
        } else if (d as c_int) < 0xc0 as c_int {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        } else if (d as c_int) < 0xe0 as c_int {
            let mut c: c_uchar = 0;
            if !(in_0 < inend) {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen =
                    processed.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_PARTIAL as c_int;
            }
            let fresh4 = in_0;
            in_0 = in_0.offset(1);
            c = *fresh4;
            if c as c_int & 0xc0 as c_int != 0x80 as c_int {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen =
                    processed.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
            c = (c as c_int & 0x3f as c_int) as c_uchar;
            d = (d as c_int & 0x1f as c_int) as c_uchar;
            d = *xlattable.offset(
                (48 as c_int
                    + c as c_int
                    + *xlattable.offset(d as isize) as c_int
                        * 64 as c_int) as isize,
            );
            if d as c_int == 0 as c_int {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen =
                    processed.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = d;
        } else if (d as c_int) < 0xf0 as c_int {
            let mut c1: c_uchar = 0;
            let mut c2: c_uchar = 0;
            if !(in_0 < inend.offset(-(1 as c_int as isize))) {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen =
                    processed.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_PARTIAL as c_int;
            }
            let fresh6 = in_0;
            in_0 = in_0.offset(1);
            c1 = *fresh6;
            if c1 as c_int & 0xc0 as c_int != 0x80 as c_int {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen =
                    processed.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
            let fresh7 = in_0;
            in_0 = in_0.offset(1);
            c2 = *fresh7;
            if c2 as c_int & 0xc0 as c_int != 0x80 as c_int {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen =
                    processed.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
            c1 = (c1 as c_int & 0x3f as c_int) as c_uchar;
            c2 = (c2 as c_int & 0x3f as c_int) as c_uchar;
            d = (d as c_int & 0xf as c_int) as c_uchar;
            d = *xlattable.offset(
                (48 as c_int
                    + c2 as c_int
                    + *xlattable.offset(
                        (48 as c_int
                            + c1 as c_int
                            + *xlattable.offset(
                                (32 as c_int + d as c_int) as isize,
                            ) as c_int
                                * 64 as c_int) as isize,
                    ) as c_int
                        * 64 as c_int) as isize,
            );
            if d as c_int == 0 as c_int {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen =
                    processed.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = d;
        } else {
            *outlen = out.offset_from(outstart) as c_long as c_int;
            *inlen = processed.offset_from(instart) as c_long as c_int;
            return XML_ENC_ERR_INPUT as c_int;
        }
        processed = in_0;
    }
    *outlen = out.offset_from(outstart) as c_long as c_int;
    *inlen = processed.offset_from(instart) as c_long as c_int;
    return *outlen;
}
unsafe fn ISO8859xToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
    mut unicodetable: *const c_ushort,
) -> c_int {
    let mut outstart: *mut c_uchar = out;
    let mut outend: *mut c_uchar = ::core::ptr::null_mut::<c_uchar>();
    let mut instart: *const c_uchar = in_0;
    let mut inend: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut instop: *const c_uchar = ::core::ptr::null::<c_uchar>();
    let mut c: c_uint = 0;
    if out.is_null()
        || outlen.is_null()
        || inlen.is_null()
        || in_0.is_null()
        || unicodetable.is_null()
    {
        return XML_ENC_ERR_INTERNAL as c_int;
    }
    outend = out.offset(*outlen as isize);
    inend = in_0.offset(*inlen as isize);
    instop = inend;
    while in_0 < inend && out < outend.offset(-(2 as c_int as isize)) {
        if *in_0 as c_int >= 0x80 as c_int {
            c = *unicodetable
                .offset((*in_0 as c_int - 0x80 as c_int) as isize)
                as c_uint;
            if c == 0 as c_uint {
                *outlen = out.offset_from(outstart) as c_long as c_int;
                *inlen = in_0.offset_from(instart) as c_long as c_int;
                return XML_ENC_ERR_INPUT as c_int;
            }
            if c < 0x800 as c_uint {
                let fresh9 = out;
                out = out.offset(1);
                *fresh9 = (c >> 6 as c_int & 0x1f as c_uint
                    | 0xc0 as c_uint)
                    as c_uchar;
                let fresh10 = out;
                out = out.offset(1);
                *fresh10 = (c & 0x3f as c_uint | 0x80 as c_uint)
                    as c_uchar;
            } else {
                let fresh11 = out;
                out = out.offset(1);
                *fresh11 = (c >> 12 as c_int & 0xf as c_uint
                    | 0xe0 as c_uint)
                    as c_uchar;
                let fresh12 = out;
                out = out.offset(1);
                *fresh12 = (c >> 6 as c_int & 0x3f as c_uint
                    | 0x80 as c_uint)
                    as c_uchar;
                let fresh13 = out;
                out = out.offset(1);
                *fresh13 = (c & 0x3f as c_uint | 0x80 as c_uint)
                    as c_uchar;
            }
            in_0 = in_0.offset(1);
        }
        if instop.offset_from(in_0) as c_long
            > outend.offset_from(out) as c_long
        {
            instop = in_0.offset(outend.offset_from(out) as c_long as isize);
        }
        while (*in_0 as c_int) < 0x80 as c_int && in_0 < instop {
            let fresh14 = in_0;
            in_0 = in_0.offset(1);
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = *fresh14;
        }
    }
    if in_0 < inend && out < outend && (*in_0 as c_int) < 0x80 as c_int {
        let fresh16 = in_0;
        in_0 = in_0.offset(1);
        let fresh17 = out;
        out = out.offset(1);
        *fresh17 = *fresh16;
    }
    if in_0 < inend && out < outend && (*in_0 as c_int) < 0x80 as c_int {
        let fresh18 = in_0;
        in_0 = in_0.offset(1);
        let fresh19 = out;
        out = out.offset(1);
        *fresh19 = *fresh18;
    }
    *outlen = out.offset_from(outstart) as c_long as c_int;
    *inlen = in_0.offset_from(instart) as c_long as c_int;
    return *outlen;
}
static mut xmlunicodetable_ISO8859_2: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x104 as c_int as c_ushort,
    0x2d8 as c_int as c_ushort,
    0x141 as c_int as c_ushort,
    0xa4 as c_int as c_ushort,
    0x13d as c_int as c_ushort,
    0x15a as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0xa8 as c_int as c_ushort,
    0x160 as c_int as c_ushort,
    0x15e as c_int as c_ushort,
    0x164 as c_int as c_ushort,
    0x179 as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0x17d as c_int as c_ushort,
    0x17b as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0x105 as c_int as c_ushort,
    0x2db as c_int as c_ushort,
    0x142 as c_int as c_ushort,
    0xb4 as c_int as c_ushort,
    0x13e as c_int as c_ushort,
    0x15b as c_int as c_ushort,
    0x2c7 as c_int as c_ushort,
    0xb8 as c_int as c_ushort,
    0x161 as c_int as c_ushort,
    0x15f as c_int as c_ushort,
    0x165 as c_int as c_ushort,
    0x17a as c_int as c_ushort,
    0x2dd as c_int as c_ushort,
    0x17e as c_int as c_ushort,
    0x17c as c_int as c_ushort,
    0x154 as c_int as c_ushort,
    0xc1 as c_int as c_ushort,
    0xc2 as c_int as c_ushort,
    0x102 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0x139 as c_int as c_ushort,
    0x106 as c_int as c_ushort,
    0xc7 as c_int as c_ushort,
    0x10c as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0x118 as c_int as c_ushort,
    0xcb as c_int as c_ushort,
    0x11a as c_int as c_ushort,
    0xcd as c_int as c_ushort,
    0xce as c_int as c_ushort,
    0x10e as c_int as c_ushort,
    0x110 as c_int as c_ushort,
    0x143 as c_int as c_ushort,
    0x147 as c_int as c_ushort,
    0xd3 as c_int as c_ushort,
    0xd4 as c_int as c_ushort,
    0x150 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0xd7 as c_int as c_ushort,
    0x158 as c_int as c_ushort,
    0x16e as c_int as c_ushort,
    0xda as c_int as c_ushort,
    0x170 as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0xdd as c_int as c_ushort,
    0x162 as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0x155 as c_int as c_ushort,
    0xe1 as c_int as c_ushort,
    0xe2 as c_int as c_ushort,
    0x103 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0x13a as c_int as c_ushort,
    0x107 as c_int as c_ushort,
    0xe7 as c_int as c_ushort,
    0x10d as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0x119 as c_int as c_ushort,
    0xeb as c_int as c_ushort,
    0x11b as c_int as c_ushort,
    0xed as c_int as c_ushort,
    0xee as c_int as c_ushort,
    0x10f as c_int as c_ushort,
    0x111 as c_int as c_ushort,
    0x144 as c_int as c_ushort,
    0x148 as c_int as c_ushort,
    0xf3 as c_int as c_ushort,
    0xf4 as c_int as c_ushort,
    0x151 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0xf7 as c_int as c_ushort,
    0x159 as c_int as c_ushort,
    0x16f as c_int as c_ushort,
    0xfa as c_int as c_ushort,
    0x171 as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0xfd as c_int as c_ushort,
    0x163 as c_int as c_ushort,
    0x2d9 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_2: [c_uchar; 432] = unsafe { ::core::mem::transmute::<[u8; 432], [c_uchar; 432]>(*b"\0\0\x01\x05\x02\x04\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\0\xA4\0\0\xA7\xA8\0\0\0\0\xAD\0\0\xB0\0\0\0\xB4\0\0\0\xB8\0\0\0\0\0\0\0\0\0\xC3\xE3\xA1\xB1\xC6\xE6\0\0\0\0\xC8\xE8\xCF\xEF\xD0\xF0\0\0\0\0\0\0\xCA\xEA\xCC\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC5\xE5\0\0\xA5\xB5\0\0\0\0\0\0\0\0\xB7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA2\xFF\0\xB2\0\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA3\xB3\xD1\xF1\0\0\xD2\xF2\0\0\0\0\0\0\0\xD5\xF5\0\0\xC0\xE0\0\0\xD8\xF8\xA6\xB6\0\0\xAA\xBA\xA9\xB9\xDE\xFE\xAB\xBB\0\0\0\0\0\0\0\0\xD9\xF9\xDB\xFB\0\0\0\0\0\0\0\xAC\xBC\xAF\xBF\xAE\xBE\0\0\xC1\xC2\0\xC4\0\0\xC7\0\xC9\0\xCB\0\xCD\xCE\0\0\0\0\xD3\xD4\0\xD6\xD7\0\0\xDA\0\xDC\xDD\0\xDF\0\xE1\xE2\0\xE4\0\0\xE7\0\xE9\0\xEB\0\xED\xEE\0\0\0\0\xF3\xF4\0\xF6\xF7\0\0\xFA\0\xFC\xFD\0\0") };
static mut xmlunicodetable_ISO8859_3: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x126 as c_int as c_ushort,
    0x2d8 as c_int as c_ushort,
    0xa3 as c_int as c_ushort,
    0xa4 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x124 as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0xa8 as c_int as c_ushort,
    0x130 as c_int as c_ushort,
    0x15e as c_int as c_ushort,
    0x11e as c_int as c_ushort,
    0x134 as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x17b as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0x127 as c_int as c_ushort,
    0xb2 as c_int as c_ushort,
    0xb3 as c_int as c_ushort,
    0xb4 as c_int as c_ushort,
    0xb5 as c_int as c_ushort,
    0x125 as c_int as c_ushort,
    0xb7 as c_int as c_ushort,
    0xb8 as c_int as c_ushort,
    0x131 as c_int as c_ushort,
    0x15f as c_int as c_ushort,
    0x11f as c_int as c_ushort,
    0x135 as c_int as c_ushort,
    0xbd as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x17c as c_int as c_ushort,
    0xc0 as c_int as c_ushort,
    0xc1 as c_int as c_ushort,
    0xc2 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0x10a as c_int as c_ushort,
    0x108 as c_int as c_ushort,
    0xc7 as c_int as c_ushort,
    0xc8 as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0xca as c_int as c_ushort,
    0xcb as c_int as c_ushort,
    0xcc as c_int as c_ushort,
    0xcd as c_int as c_ushort,
    0xce as c_int as c_ushort,
    0xcf as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xd1 as c_int as c_ushort,
    0xd2 as c_int as c_ushort,
    0xd3 as c_int as c_ushort,
    0xd4 as c_int as c_ushort,
    0x120 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0xd7 as c_int as c_ushort,
    0x11c as c_int as c_ushort,
    0xd9 as c_int as c_ushort,
    0xda as c_int as c_ushort,
    0xdb as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0x16c as c_int as c_ushort,
    0x15c as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0xe0 as c_int as c_ushort,
    0xe1 as c_int as c_ushort,
    0xe2 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0x10b as c_int as c_ushort,
    0x109 as c_int as c_ushort,
    0xe7 as c_int as c_ushort,
    0xe8 as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0xea as c_int as c_ushort,
    0xeb as c_int as c_ushort,
    0xec as c_int as c_ushort,
    0xed as c_int as c_ushort,
    0xee as c_int as c_ushort,
    0xef as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xf1 as c_int as c_ushort,
    0xf2 as c_int as c_ushort,
    0xf3 as c_int as c_ushort,
    0xf4 as c_int as c_ushort,
    0x121 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0xf7 as c_int as c_ushort,
    0x11d as c_int as c_ushort,
    0xf9 as c_int as c_ushort,
    0xfa as c_int as c_ushort,
    0xfb as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0x16d as c_int as c_ushort,
    0x15d as c_int as c_ushort,
    0x2d9 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_3: [c_uchar; 496] = unsafe { ::core::mem::transmute::<[u8; 496], [c_uchar; 496]>(*b"\x04\0\x01\x06\x02\x05\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\xA3\xA4\0\0\xA7\xA8\0\0\0\0\xAD\0\0\xB0\0\xB2\xB3\xB4\xB5\0\xB7\xB8\0\0\0\0\xBD\0\0\0\0\0\0\0\0\0\0\xC6\xE6\xC5\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD8\xF8\xAB\xBB\xD5\xF5\0\0\xA6\xB6\xA1\xB1\0\0\0\0\0\0\0\0\xA9\xB9\0\0\xAC\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA2\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDE\xFE\xAA\xBA\0\0\0\0\0\0\0\0\0\0\0\0\xDD\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\xAF\xBF\0\0\0\xC0\xC1\xC2\0\xC4\0\0\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\0\xD1\xD2\xD3\xD4\0\xD6\xD7\0\xD9\xDA\xDB\xDC\0\0\xDF\xE0\xE1\xE2\0\xE4\0\0\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\0\xF1\xF2\xF3\xF4\0\xF6\xF7\0\xF9\xFA\xFB\xFC\0\0\0") };
static mut xmlunicodetable_ISO8859_4: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x104 as c_int as c_ushort,
    0x138 as c_int as c_ushort,
    0x156 as c_int as c_ushort,
    0xa4 as c_int as c_ushort,
    0x128 as c_int as c_ushort,
    0x13b as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0xa8 as c_int as c_ushort,
    0x160 as c_int as c_ushort,
    0x112 as c_int as c_ushort,
    0x122 as c_int as c_ushort,
    0x166 as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0x17d as c_int as c_ushort,
    0xaf as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0x105 as c_int as c_ushort,
    0x2db as c_int as c_ushort,
    0x157 as c_int as c_ushort,
    0xb4 as c_int as c_ushort,
    0x129 as c_int as c_ushort,
    0x13c as c_int as c_ushort,
    0x2c7 as c_int as c_ushort,
    0xb8 as c_int as c_ushort,
    0x161 as c_int as c_ushort,
    0x113 as c_int as c_ushort,
    0x123 as c_int as c_ushort,
    0x167 as c_int as c_ushort,
    0x14a as c_int as c_ushort,
    0x17e as c_int as c_ushort,
    0x14b as c_int as c_ushort,
    0x100 as c_int as c_ushort,
    0xc1 as c_int as c_ushort,
    0xc2 as c_int as c_ushort,
    0xc3 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0xc5 as c_int as c_ushort,
    0xc6 as c_int as c_ushort,
    0x12e as c_int as c_ushort,
    0x10c as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0x118 as c_int as c_ushort,
    0xcb as c_int as c_ushort,
    0x116 as c_int as c_ushort,
    0xcd as c_int as c_ushort,
    0xce as c_int as c_ushort,
    0x12a as c_int as c_ushort,
    0x110 as c_int as c_ushort,
    0x145 as c_int as c_ushort,
    0x14c as c_int as c_ushort,
    0x136 as c_int as c_ushort,
    0xd4 as c_int as c_ushort,
    0xd5 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0xd7 as c_int as c_ushort,
    0xd8 as c_int as c_ushort,
    0x172 as c_int as c_ushort,
    0xda as c_int as c_ushort,
    0xdb as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0x168 as c_int as c_ushort,
    0x16a as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0x101 as c_int as c_ushort,
    0xe1 as c_int as c_ushort,
    0xe2 as c_int as c_ushort,
    0xe3 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0xe5 as c_int as c_ushort,
    0xe6 as c_int as c_ushort,
    0x12f as c_int as c_ushort,
    0x10d as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0x119 as c_int as c_ushort,
    0xeb as c_int as c_ushort,
    0x117 as c_int as c_ushort,
    0xed as c_int as c_ushort,
    0xee as c_int as c_ushort,
    0x12b as c_int as c_ushort,
    0x111 as c_int as c_ushort,
    0x146 as c_int as c_ushort,
    0x14d as c_int as c_ushort,
    0x137 as c_int as c_ushort,
    0xf4 as c_int as c_ushort,
    0xf5 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0xf7 as c_int as c_ushort,
    0xf8 as c_int as c_ushort,
    0x173 as c_int as c_ushort,
    0xfa as c_int as c_ushort,
    0xfb as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0x169 as c_int as c_ushort,
    0x16b as c_int as c_ushort,
    0x2d9 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_4: [c_uchar; 432] = unsafe { ::core::mem::transmute::<[u8; 432], [c_uchar; 432]>(*b"\0\0\x01\x05\x02\x03\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\0\xA4\0\0\xA7\xA8\0\0\0\0\xAD\0\xAF\xB0\0\0\0\xB4\0\0\0\xB8\0\0\0\0\0\0\0\xC0\xE0\0\0\xA1\xB1\0\0\0\0\0\0\xC8\xE8\0\0\xD0\xF0\xAA\xBA\0\0\xCC\xEC\xCA\xEA\0\0\0\0\0\0\0\0\xAB\xBB\0\0\0\0\xA5\xB5\xCF\xEF\0\0\xC7\xE7\0\0\0\0\0\0\xD3\xF3\xA2\0\0\xA6\xB6\0\0\0\0\0\0\0\0\xD1\xF1\0\0\0\xBD\xBF\xD2\xF2\0\0\0\0\0\0\0\0\xA3\xB3\0\0\0\0\0\0\0\0\xA9\xB9\0\0\0\0\xAC\xBC\xDD\xFD\xDE\xFE\0\0\0\0\0\0\xD9\xF9\0\0\0\0\0\0\0\0\0\xAE\xBE\0\0\0\0\0\0\0\0\xB7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\0\xB2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC1\xC2\xC3\xC4\xC5\xC6\0\0\xC9\0\xCB\0\xCD\xCE\0\0\0\0\0\xD4\xD5\xD6\xD7\xD8\0\xDA\xDB\xDC\0\0\xDF\0\xE1\xE2\xE3\xE4\xE5\xE6\0\0\xE9\0\xEB\0\xED\xEE\0\0\0\0\0\xF4\xF5\xF6\xF7\xF8\0\xFA\xFB\xFC\0\0\0") };
static mut xmlunicodetable_ISO8859_5: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x401 as c_int as c_ushort,
    0x402 as c_int as c_ushort,
    0x403 as c_int as c_ushort,
    0x404 as c_int as c_ushort,
    0x405 as c_int as c_ushort,
    0x406 as c_int as c_ushort,
    0x407 as c_int as c_ushort,
    0x408 as c_int as c_ushort,
    0x409 as c_int as c_ushort,
    0x40a as c_int as c_ushort,
    0x40b as c_int as c_ushort,
    0x40c as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0x40e as c_int as c_ushort,
    0x40f as c_int as c_ushort,
    0x410 as c_int as c_ushort,
    0x411 as c_int as c_ushort,
    0x412 as c_int as c_ushort,
    0x413 as c_int as c_ushort,
    0x414 as c_int as c_ushort,
    0x415 as c_int as c_ushort,
    0x416 as c_int as c_ushort,
    0x417 as c_int as c_ushort,
    0x418 as c_int as c_ushort,
    0x419 as c_int as c_ushort,
    0x41a as c_int as c_ushort,
    0x41b as c_int as c_ushort,
    0x41c as c_int as c_ushort,
    0x41d as c_int as c_ushort,
    0x41e as c_int as c_ushort,
    0x41f as c_int as c_ushort,
    0x420 as c_int as c_ushort,
    0x421 as c_int as c_ushort,
    0x422 as c_int as c_ushort,
    0x423 as c_int as c_ushort,
    0x424 as c_int as c_ushort,
    0x425 as c_int as c_ushort,
    0x426 as c_int as c_ushort,
    0x427 as c_int as c_ushort,
    0x428 as c_int as c_ushort,
    0x429 as c_int as c_ushort,
    0x42a as c_int as c_ushort,
    0x42b as c_int as c_ushort,
    0x42c as c_int as c_ushort,
    0x42d as c_int as c_ushort,
    0x42e as c_int as c_ushort,
    0x42f as c_int as c_ushort,
    0x430 as c_int as c_ushort,
    0x431 as c_int as c_ushort,
    0x432 as c_int as c_ushort,
    0x433 as c_int as c_ushort,
    0x434 as c_int as c_ushort,
    0x435 as c_int as c_ushort,
    0x436 as c_int as c_ushort,
    0x437 as c_int as c_ushort,
    0x438 as c_int as c_ushort,
    0x439 as c_int as c_ushort,
    0x43a as c_int as c_ushort,
    0x43b as c_int as c_ushort,
    0x43c as c_int as c_ushort,
    0x43d as c_int as c_ushort,
    0x43e as c_int as c_ushort,
    0x43f as c_int as c_ushort,
    0x440 as c_int as c_ushort,
    0x441 as c_int as c_ushort,
    0x442 as c_int as c_ushort,
    0x443 as c_int as c_ushort,
    0x444 as c_int as c_ushort,
    0x445 as c_int as c_ushort,
    0x446 as c_int as c_ushort,
    0x447 as c_int as c_ushort,
    0x448 as c_int as c_ushort,
    0x449 as c_int as c_ushort,
    0x44a as c_int as c_ushort,
    0x44b as c_int as c_ushort,
    0x44c as c_int as c_ushort,
    0x44d as c_int as c_ushort,
    0x44e as c_int as c_ushort,
    0x44f as c_int as c_ushort,
    0x2116 as c_int as c_ushort,
    0x451 as c_int as c_ushort,
    0x452 as c_int as c_ushort,
    0x453 as c_int as c_ushort,
    0x454 as c_int as c_ushort,
    0x455 as c_int as c_ushort,
    0x456 as c_int as c_ushort,
    0x457 as c_int as c_ushort,
    0x458 as c_int as c_ushort,
    0x459 as c_int as c_ushort,
    0x45a as c_int as c_ushort,
    0x45b as c_int as c_ushort,
    0x45c as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0x45e as c_int as c_ushort,
    0x45f as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_5: [c_uchar; 432] = unsafe { ::core::mem::transmute::<[u8; 432], [c_uchar; 432]>(*b"\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\0\0\0\0\xFD\0\0\0\0\0\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\0\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\xD0\xD1\xD2\xD3\xD4\xD5\xD6\xD7\xD8\xD9\xDA\xDB\xDC\xDD\xDE\xDF\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\0\xF1\xF2\xF3\xF4\xF5\xF6\xF7\xF8\xF9\xFA\xFB\xFC\0\xFE\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") };
static mut xmlunicodetable_ISO8859_6: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xa4 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x60c as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x61b as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x61f as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x621 as c_int as c_ushort,
    0x622 as c_int as c_ushort,
    0x623 as c_int as c_ushort,
    0x624 as c_int as c_ushort,
    0x625 as c_int as c_ushort,
    0x626 as c_int as c_ushort,
    0x627 as c_int as c_ushort,
    0x628 as c_int as c_ushort,
    0x629 as c_int as c_ushort,
    0x62a as c_int as c_ushort,
    0x62b as c_int as c_ushort,
    0x62c as c_int as c_ushort,
    0x62d as c_int as c_ushort,
    0x62e as c_int as c_ushort,
    0x62f as c_int as c_ushort,
    0x630 as c_int as c_ushort,
    0x631 as c_int as c_ushort,
    0x632 as c_int as c_ushort,
    0x633 as c_int as c_ushort,
    0x634 as c_int as c_ushort,
    0x635 as c_int as c_ushort,
    0x636 as c_int as c_ushort,
    0x637 as c_int as c_ushort,
    0x638 as c_int as c_ushort,
    0x639 as c_int as c_ushort,
    0x63a as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x640 as c_int as c_ushort,
    0x641 as c_int as c_ushort,
    0x642 as c_int as c_ushort,
    0x643 as c_int as c_ushort,
    0x644 as c_int as c_ushort,
    0x645 as c_int as c_ushort,
    0x646 as c_int as c_ushort,
    0x647 as c_int as c_ushort,
    0x648 as c_int as c_ushort,
    0x649 as c_int as c_ushort,
    0x64a as c_int as c_ushort,
    0x64b as c_int as c_ushort,
    0x64c as c_int as c_ushort,
    0x64d as c_int as c_ushort,
    0x64e as c_int as c_ushort,
    0x64f as c_int as c_ushort,
    0x650 as c_int as c_ushort,
    0x651 as c_int as c_ushort,
    0x652 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_6: [c_uchar; 368] = unsafe { ::core::mem::transmute::<[u8; 368], [c_uchar; 368]>(*b"\x02\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\0\xA4\0\0\0\0\0\0\0\0\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xBB\0\0\0\xBF\0\xC1\xC2\xC3\xC4\xC5\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\xD0\xD1\xD2\xD3\xD4\xD5\xD6\xD7\xD8\xD9\xDA\0\0\0\0\0\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\xF0\xF1\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") };
static mut xmlunicodetable_ISO8859_7: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x2018 as c_int as c_ushort,
    0x2019 as c_int as c_ushort,
    0xa3 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xa6 as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0xa8 as c_int as c_ushort,
    0xa9 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xab as c_int as c_ushort,
    0xac as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x2015 as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0xb1 as c_int as c_ushort,
    0xb2 as c_int as c_ushort,
    0xb3 as c_int as c_ushort,
    0x384 as c_int as c_ushort,
    0x385 as c_int as c_ushort,
    0x386 as c_int as c_ushort,
    0xb7 as c_int as c_ushort,
    0x388 as c_int as c_ushort,
    0x389 as c_int as c_ushort,
    0x38a as c_int as c_ushort,
    0xbb as c_int as c_ushort,
    0x38c as c_int as c_ushort,
    0xbd as c_int as c_ushort,
    0x38e as c_int as c_ushort,
    0x38f as c_int as c_ushort,
    0x390 as c_int as c_ushort,
    0x391 as c_int as c_ushort,
    0x392 as c_int as c_ushort,
    0x393 as c_int as c_ushort,
    0x394 as c_int as c_ushort,
    0x395 as c_int as c_ushort,
    0x396 as c_int as c_ushort,
    0x397 as c_int as c_ushort,
    0x398 as c_int as c_ushort,
    0x399 as c_int as c_ushort,
    0x39a as c_int as c_ushort,
    0x39b as c_int as c_ushort,
    0x39c as c_int as c_ushort,
    0x39d as c_int as c_ushort,
    0x39e as c_int as c_ushort,
    0x39f as c_int as c_ushort,
    0x3a0 as c_int as c_ushort,
    0x3a1 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x3a3 as c_int as c_ushort,
    0x3a4 as c_int as c_ushort,
    0x3a5 as c_int as c_ushort,
    0x3a6 as c_int as c_ushort,
    0x3a7 as c_int as c_ushort,
    0x3a8 as c_int as c_ushort,
    0x3a9 as c_int as c_ushort,
    0x3aa as c_int as c_ushort,
    0x3ab as c_int as c_ushort,
    0x3ac as c_int as c_ushort,
    0x3ad as c_int as c_ushort,
    0x3ae as c_int as c_ushort,
    0x3af as c_int as c_ushort,
    0x3b0 as c_int as c_ushort,
    0x3b1 as c_int as c_ushort,
    0x3b2 as c_int as c_ushort,
    0x3b3 as c_int as c_ushort,
    0x3b4 as c_int as c_ushort,
    0x3b5 as c_int as c_ushort,
    0x3b6 as c_int as c_ushort,
    0x3b7 as c_int as c_ushort,
    0x3b8 as c_int as c_ushort,
    0x3b9 as c_int as c_ushort,
    0x3ba as c_int as c_ushort,
    0x3bb as c_int as c_ushort,
    0x3bc as c_int as c_ushort,
    0x3bd as c_int as c_ushort,
    0x3be as c_int as c_ushort,
    0x3bf as c_int as c_ushort,
    0x3c0 as c_int as c_ushort,
    0x3c1 as c_int as c_ushort,
    0x3c2 as c_int as c_ushort,
    0x3c3 as c_int as c_ushort,
    0x3c4 as c_int as c_ushort,
    0x3c5 as c_int as c_ushort,
    0x3c6 as c_int as c_ushort,
    0x3c7 as c_int as c_ushort,
    0x3c8 as c_int as c_ushort,
    0x3c9 as c_int as c_ushort,
    0x3ca as c_int as c_ushort,
    0x3cb as c_int as c_ushort,
    0x3cc as c_int as c_ushort,
    0x3cd as c_int as c_ushort,
    0x3ce as c_int as c_ushort,
    0 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_7: [c_uchar; 496] = unsafe { ::core::mem::transmute::<[u8; 496], [c_uchar; 496]>(*b"\x04\0\x01\0\0\0\0\0\0\0\0\0\0\0\x05\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\xA3\0\0\xA6\xA7\xA8\xA9\0\xAB\xAC\xAD\0\0\xB0\xB1\xB2\xB3\0\0\0\xB7\0\0\0\xBB\0\xBD\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xAF\0\0\xA1\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB4\xB5\xB6\0\xB8\xB9\xBA\0\xBC\0\xBE\xBF\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\xD0\xD1\0\xD3\xD4\xD5\xD6\xD7\xD8\xD9\xDA\xDB\xDC\xDD\xDE\xDF\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\xF0\xF1\xF2\xF3\xF4\xF5\xF6\xF7\xF8\xF9\xFA\xFB\xFC\xFD\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") };
static mut xmlunicodetable_ISO8859_8: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xa2 as c_int as c_ushort,
    0xa3 as c_int as c_ushort,
    0xa4 as c_int as c_ushort,
    0xa5 as c_int as c_ushort,
    0xa6 as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0xa8 as c_int as c_ushort,
    0xa9 as c_int as c_ushort,
    0xd7 as c_int as c_ushort,
    0xab as c_int as c_ushort,
    0xac as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0xae as c_int as c_ushort,
    0xaf as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0xb1 as c_int as c_ushort,
    0xb2 as c_int as c_ushort,
    0xb3 as c_int as c_ushort,
    0xb4 as c_int as c_ushort,
    0xb5 as c_int as c_ushort,
    0xb6 as c_int as c_ushort,
    0xb7 as c_int as c_ushort,
    0xb8 as c_int as c_ushort,
    0xb9 as c_int as c_ushort,
    0xf7 as c_int as c_ushort,
    0xbb as c_int as c_ushort,
    0xbc as c_int as c_ushort,
    0xbd as c_int as c_ushort,
    0xbe as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x2017 as c_int as c_ushort,
    0x5d0 as c_int as c_ushort,
    0x5d1 as c_int as c_ushort,
    0x5d2 as c_int as c_ushort,
    0x5d3 as c_int as c_ushort,
    0x5d4 as c_int as c_ushort,
    0x5d5 as c_int as c_ushort,
    0x5d6 as c_int as c_ushort,
    0x5d7 as c_int as c_ushort,
    0x5d8 as c_int as c_ushort,
    0x5d9 as c_int as c_ushort,
    0x5da as c_int as c_ushort,
    0x5db as c_int as c_ushort,
    0x5dc as c_int as c_ushort,
    0x5dd as c_int as c_ushort,
    0x5de as c_int as c_ushort,
    0x5df as c_int as c_ushort,
    0x5e0 as c_int as c_ushort,
    0x5e1 as c_int as c_ushort,
    0x5e2 as c_int as c_ushort,
    0x5e3 as c_int as c_ushort,
    0x5e4 as c_int as c_ushort,
    0x5e5 as c_int as c_ushort,
    0x5e6 as c_int as c_ushort,
    0x5e7 as c_int as c_ushort,
    0x5e8 as c_int as c_ushort,
    0x5e9 as c_int as c_ushort,
    0x5ea as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0x200e as c_int as c_ushort,
    0x200f as c_int as c_ushort,
    0 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_8: [c_uchar; 496] = unsafe { ::core::mem::transmute::<[u8; 496], [c_uchar; 496]>(*b"\x02\0\x01\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06\0\0\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\0\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\0\xBB\xBC\xBD\xBE\0\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xBA\0\0\0\0\0\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFD\xFE\0\0\0\0\0\0\0\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\xF0\xF1\xF2\xF3\xF4\xF5\xF6\xF7\xF8\xF9\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") };
static mut xmlunicodetable_ISO8859_9: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0xa1 as c_int as c_ushort,
    0xa2 as c_int as c_ushort,
    0xa3 as c_int as c_ushort,
    0xa4 as c_int as c_ushort,
    0xa5 as c_int as c_ushort,
    0xa6 as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0xa8 as c_int as c_ushort,
    0xa9 as c_int as c_ushort,
    0xaa as c_int as c_ushort,
    0xab as c_int as c_ushort,
    0xac as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0xae as c_int as c_ushort,
    0xaf as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0xb1 as c_int as c_ushort,
    0xb2 as c_int as c_ushort,
    0xb3 as c_int as c_ushort,
    0xb4 as c_int as c_ushort,
    0xb5 as c_int as c_ushort,
    0xb6 as c_int as c_ushort,
    0xb7 as c_int as c_ushort,
    0xb8 as c_int as c_ushort,
    0xb9 as c_int as c_ushort,
    0xba as c_int as c_ushort,
    0xbb as c_int as c_ushort,
    0xbc as c_int as c_ushort,
    0xbd as c_int as c_ushort,
    0xbe as c_int as c_ushort,
    0xbf as c_int as c_ushort,
    0xc0 as c_int as c_ushort,
    0xc1 as c_int as c_ushort,
    0xc2 as c_int as c_ushort,
    0xc3 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0xc5 as c_int as c_ushort,
    0xc6 as c_int as c_ushort,
    0xc7 as c_int as c_ushort,
    0xc8 as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0xca as c_int as c_ushort,
    0xcb as c_int as c_ushort,
    0xcc as c_int as c_ushort,
    0xcd as c_int as c_ushort,
    0xce as c_int as c_ushort,
    0xcf as c_int as c_ushort,
    0x11e as c_int as c_ushort,
    0xd1 as c_int as c_ushort,
    0xd2 as c_int as c_ushort,
    0xd3 as c_int as c_ushort,
    0xd4 as c_int as c_ushort,
    0xd5 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0xd7 as c_int as c_ushort,
    0xd8 as c_int as c_ushort,
    0xd9 as c_int as c_ushort,
    0xda as c_int as c_ushort,
    0xdb as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0x130 as c_int as c_ushort,
    0x15e as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0xe0 as c_int as c_ushort,
    0xe1 as c_int as c_ushort,
    0xe2 as c_int as c_ushort,
    0xe3 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0xe5 as c_int as c_ushort,
    0xe6 as c_int as c_ushort,
    0xe7 as c_int as c_ushort,
    0xe8 as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0xea as c_int as c_ushort,
    0xeb as c_int as c_ushort,
    0xec as c_int as c_ushort,
    0xed as c_int as c_ushort,
    0xee as c_int as c_ushort,
    0xef as c_int as c_ushort,
    0x11f as c_int as c_ushort,
    0xf1 as c_int as c_ushort,
    0xf2 as c_int as c_ushort,
    0xf3 as c_int as c_ushort,
    0xf4 as c_int as c_ushort,
    0xf5 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0xf7 as c_int as c_ushort,
    0xf8 as c_int as c_ushort,
    0xf9 as c_int as c_ushort,
    0xfa as c_int as c_ushort,
    0xfb as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0x131 as c_int as c_ushort,
    0x15f as c_int as c_ushort,
    0xff as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_9: [c_uchar; 368] = unsafe { ::core::mem::transmute::<[u8; 368], [c_uchar; 368]>(*b"\0\0\x01\x02\x03\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\0\xD1\xD2\xD3\xD4\xD5\xD6\xD7\xD8\xD9\xDA\xDB\xDC\0\0\xDF\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\0\xF1\xF2\xF3\xF4\xF5\xF6\xF7\xF8\xF9\xFA\xFB\xFC\0\0\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD0\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDD\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xDE\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") };
static mut xmlunicodetable_ISO8859_10: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x104 as c_int as c_ushort,
    0x112 as c_int as c_ushort,
    0x122 as c_int as c_ushort,
    0x12a as c_int as c_ushort,
    0x128 as c_int as c_ushort,
    0x136 as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0x13b as c_int as c_ushort,
    0x110 as c_int as c_ushort,
    0x160 as c_int as c_ushort,
    0x166 as c_int as c_ushort,
    0x17d as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0x16a as c_int as c_ushort,
    0x14a as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0x105 as c_int as c_ushort,
    0x113 as c_int as c_ushort,
    0x123 as c_int as c_ushort,
    0x12b as c_int as c_ushort,
    0x129 as c_int as c_ushort,
    0x137 as c_int as c_ushort,
    0xb7 as c_int as c_ushort,
    0x13c as c_int as c_ushort,
    0x111 as c_int as c_ushort,
    0x161 as c_int as c_ushort,
    0x167 as c_int as c_ushort,
    0x17e as c_int as c_ushort,
    0x2015 as c_int as c_ushort,
    0x16b as c_int as c_ushort,
    0x14b as c_int as c_ushort,
    0x100 as c_int as c_ushort,
    0xc1 as c_int as c_ushort,
    0xc2 as c_int as c_ushort,
    0xc3 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0xc5 as c_int as c_ushort,
    0xc6 as c_int as c_ushort,
    0x12e as c_int as c_ushort,
    0x10c as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0x118 as c_int as c_ushort,
    0xcb as c_int as c_ushort,
    0x116 as c_int as c_ushort,
    0xcd as c_int as c_ushort,
    0xce as c_int as c_ushort,
    0xcf as c_int as c_ushort,
    0xd0 as c_int as c_ushort,
    0x145 as c_int as c_ushort,
    0x14c as c_int as c_ushort,
    0xd3 as c_int as c_ushort,
    0xd4 as c_int as c_ushort,
    0xd5 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0x168 as c_int as c_ushort,
    0xd8 as c_int as c_ushort,
    0x172 as c_int as c_ushort,
    0xda as c_int as c_ushort,
    0xdb as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0xdd as c_int as c_ushort,
    0xde as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0x101 as c_int as c_ushort,
    0xe1 as c_int as c_ushort,
    0xe2 as c_int as c_ushort,
    0xe3 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0xe5 as c_int as c_ushort,
    0xe6 as c_int as c_ushort,
    0x12f as c_int as c_ushort,
    0x10d as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0x119 as c_int as c_ushort,
    0xeb as c_int as c_ushort,
    0x117 as c_int as c_ushort,
    0xed as c_int as c_ushort,
    0xee as c_int as c_ushort,
    0xef as c_int as c_ushort,
    0xf0 as c_int as c_ushort,
    0x146 as c_int as c_ushort,
    0x14d as c_int as c_ushort,
    0xf3 as c_int as c_ushort,
    0xf4 as c_int as c_ushort,
    0xf5 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0x169 as c_int as c_ushort,
    0xf8 as c_int as c_ushort,
    0x173 as c_int as c_ushort,
    0xfa as c_int as c_ushort,
    0xfb as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0xfd as c_int as c_ushort,
    0xfe as c_int as c_ushort,
    0x138 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_10: [c_uchar; 496] = unsafe { ::core::mem::transmute::<[u8; 496], [c_uchar; 496]>(*b"\0\0\x01\x06\x02\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\0\0\0\0\xA7\0\0\0\0\0\xAD\0\0\xB0\0\0\0\0\0\0\xB7\0\0\0\0\0\0\0\0\xC0\xE0\0\0\xA1\xB1\0\0\0\0\0\0\xC8\xE8\0\0\xA9\xB9\xA2\xB2\0\0\xCC\xEC\xCA\xEA\0\0\0\0\0\0\0\0\xA3\xB3\0\0\0\0\xA5\xB5\xA4\xB4\0\0\xC7\xE7\0\0\0\0\0\0\xA6\xB6\xFF\0\0\xA8\xB8\0\0\0\0\0\0\0\0\xD1\xF1\0\0\0\xAF\xBF\xD2\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xAA\xBA\0\0\0\0\xAB\xBB\xD7\xF7\xAE\xBE\0\0\0\0\0\0\xD9\xF9\0\0\0\0\0\0\0\0\0\xAC\xBC\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC1\xC2\xC3\xC4\xC5\xC6\0\0\xC9\0\xCB\0\xCD\xCE\xCF\xD0\0\0\xD3\xD4\xD5\xD6\0\xD8\0\xDA\xDB\xDC\xDD\xDE\xDF\0\xE1\xE2\xE3\xE4\xE5\xE6\0\0\xE9\0\xEB\0\xED\xEE\xEF\xF0\0\0\xF3\xF4\xF5\xF6\0\xF8\0\xFA\xFB\xFC\xFD\xFE\0") };
static mut xmlunicodetable_ISO8859_11: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0xe01 as c_int as c_ushort,
    0xe02 as c_int as c_ushort,
    0xe03 as c_int as c_ushort,
    0xe04 as c_int as c_ushort,
    0xe05 as c_int as c_ushort,
    0xe06 as c_int as c_ushort,
    0xe07 as c_int as c_ushort,
    0xe08 as c_int as c_ushort,
    0xe09 as c_int as c_ushort,
    0xe0a as c_int as c_ushort,
    0xe0b as c_int as c_ushort,
    0xe0c as c_int as c_ushort,
    0xe0d as c_int as c_ushort,
    0xe0e as c_int as c_ushort,
    0xe0f as c_int as c_ushort,
    0xe10 as c_int as c_ushort,
    0xe11 as c_int as c_ushort,
    0xe12 as c_int as c_ushort,
    0xe13 as c_int as c_ushort,
    0xe14 as c_int as c_ushort,
    0xe15 as c_int as c_ushort,
    0xe16 as c_int as c_ushort,
    0xe17 as c_int as c_ushort,
    0xe18 as c_int as c_ushort,
    0xe19 as c_int as c_ushort,
    0xe1a as c_int as c_ushort,
    0xe1b as c_int as c_ushort,
    0xe1c as c_int as c_ushort,
    0xe1d as c_int as c_ushort,
    0xe1e as c_int as c_ushort,
    0xe1f as c_int as c_ushort,
    0xe20 as c_int as c_ushort,
    0xe21 as c_int as c_ushort,
    0xe22 as c_int as c_ushort,
    0xe23 as c_int as c_ushort,
    0xe24 as c_int as c_ushort,
    0xe25 as c_int as c_ushort,
    0xe26 as c_int as c_ushort,
    0xe27 as c_int as c_ushort,
    0xe28 as c_int as c_ushort,
    0xe29 as c_int as c_ushort,
    0xe2a as c_int as c_ushort,
    0xe2b as c_int as c_ushort,
    0xe2c as c_int as c_ushort,
    0xe2d as c_int as c_ushort,
    0xe2e as c_int as c_ushort,
    0xe2f as c_int as c_ushort,
    0xe30 as c_int as c_ushort,
    0xe31 as c_int as c_ushort,
    0xe32 as c_int as c_ushort,
    0xe33 as c_int as c_ushort,
    0xe34 as c_int as c_ushort,
    0xe35 as c_int as c_ushort,
    0xe36 as c_int as c_ushort,
    0xe37 as c_int as c_ushort,
    0xe38 as c_int as c_ushort,
    0xe39 as c_int as c_ushort,
    0xe3a as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0xe3f as c_int as c_ushort,
    0xe40 as c_int as c_ushort,
    0xe41 as c_int as c_ushort,
    0xe42 as c_int as c_ushort,
    0xe43 as c_int as c_ushort,
    0xe44 as c_int as c_ushort,
    0xe45 as c_int as c_ushort,
    0xe46 as c_int as c_ushort,
    0xe47 as c_int as c_ushort,
    0xe48 as c_int as c_ushort,
    0xe49 as c_int as c_ushort,
    0xe4a as c_int as c_ushort,
    0xe4b as c_int as c_ushort,
    0xe4c as c_int as c_ushort,
    0xe4d as c_int as c_ushort,
    0xe4e as c_int as c_ushort,
    0xe4f as c_int as c_ushort,
    0xe50 as c_int as c_ushort,
    0xe51 as c_int as c_ushort,
    0xe52 as c_int as c_ushort,
    0xe53 as c_int as c_ushort,
    0xe54 as c_int as c_ushort,
    0xe55 as c_int as c_ushort,
    0xe56 as c_int as c_ushort,
    0xe57 as c_int as c_ushort,
    0xe58 as c_int as c_ushort,
    0xe59 as c_int as c_ushort,
    0xe5a as c_int as c_ushort,
    0xe5b as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
    0 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_11: [c_uchar; 432] = unsafe { ::core::mem::transmute::<[u8; 432], [c_uchar; 432]>(*b"\x04\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x05\0\0\0\0\0\0\0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\xD0\xD1\xD2\xD3\xD4\xD5\xD6\xD7\xD8\xD9\xDA\0\0\0\0\xDF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\xF0\xF1\xF2\xF3\xF4\xF5\xF6\xF7\xF8\xF9\xFA\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0") };
static mut xmlunicodetable_ISO8859_13: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x201d as c_int as c_ushort,
    0xa2 as c_int as c_ushort,
    0xa3 as c_int as c_ushort,
    0xa4 as c_int as c_ushort,
    0x201e as c_int as c_ushort,
    0xa6 as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0xd8 as c_int as c_ushort,
    0xa9 as c_int as c_ushort,
    0x156 as c_int as c_ushort,
    0xab as c_int as c_ushort,
    0xac as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0xae as c_int as c_ushort,
    0xc6 as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0xb1 as c_int as c_ushort,
    0xb2 as c_int as c_ushort,
    0xb3 as c_int as c_ushort,
    0x201c as c_int as c_ushort,
    0xb5 as c_int as c_ushort,
    0xb6 as c_int as c_ushort,
    0xb7 as c_int as c_ushort,
    0xf8 as c_int as c_ushort,
    0xb9 as c_int as c_ushort,
    0x157 as c_int as c_ushort,
    0xbb as c_int as c_ushort,
    0xbc as c_int as c_ushort,
    0xbd as c_int as c_ushort,
    0xbe as c_int as c_ushort,
    0xe6 as c_int as c_ushort,
    0x104 as c_int as c_ushort,
    0x12e as c_int as c_ushort,
    0x100 as c_int as c_ushort,
    0x106 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0xc5 as c_int as c_ushort,
    0x118 as c_int as c_ushort,
    0x112 as c_int as c_ushort,
    0x10c as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0x179 as c_int as c_ushort,
    0x116 as c_int as c_ushort,
    0x122 as c_int as c_ushort,
    0x136 as c_int as c_ushort,
    0x12a as c_int as c_ushort,
    0x13b as c_int as c_ushort,
    0x160 as c_int as c_ushort,
    0x143 as c_int as c_ushort,
    0x145 as c_int as c_ushort,
    0xd3 as c_int as c_ushort,
    0x14c as c_int as c_ushort,
    0xd5 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0xd7 as c_int as c_ushort,
    0x172 as c_int as c_ushort,
    0x141 as c_int as c_ushort,
    0x15a as c_int as c_ushort,
    0x16a as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0x17b as c_int as c_ushort,
    0x17d as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0x105 as c_int as c_ushort,
    0x12f as c_int as c_ushort,
    0x101 as c_int as c_ushort,
    0x107 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0xe5 as c_int as c_ushort,
    0x119 as c_int as c_ushort,
    0x113 as c_int as c_ushort,
    0x10d as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0x17a as c_int as c_ushort,
    0x117 as c_int as c_ushort,
    0x123 as c_int as c_ushort,
    0x137 as c_int as c_ushort,
    0x12b as c_int as c_ushort,
    0x13c as c_int as c_ushort,
    0x161 as c_int as c_ushort,
    0x144 as c_int as c_ushort,
    0x146 as c_int as c_ushort,
    0xf3 as c_int as c_ushort,
    0x14d as c_int as c_ushort,
    0xf5 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0xf7 as c_int as c_ushort,
    0x173 as c_int as c_ushort,
    0x142 as c_int as c_ushort,
    0x15b as c_int as c_ushort,
    0x16b as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0x17c as c_int as c_ushort,
    0x17e as c_int as c_ushort,
    0x2019 as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_13: [c_uchar; 496] = unsafe { ::core::mem::transmute::<[u8; 496], [c_uchar; 496]>(*b"\0\0\x01\x04\x06\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\xA2\xA3\xA4\0\xA6\xA7\0\xA9\0\xAB\xAC\xAD\xAE\0\xB0\xB1\xB2\xB3\0\xB5\xB6\xB7\0\xB9\0\xBB\xBC\xBD\xBE\0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xFF\0\0\xB4\xA1\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC4\xC5\xAF\0\0\xC9\0\0\0\0\0\0\0\0\0\xD3\0\xD5\xD6\xD7\xA8\0\0\0\xDC\0\0\xDF\0\0\0\0\xE4\xE5\xBF\0\0\xE9\0\0\0\0\0\0\0\0\0\xF3\0\xF5\xF6\xF7\xB8\0\0\0\xFC\0\0\0\0\xD9\xF9\xD1\xF1\xD2\xF2\0\0\0\0\0\xD4\xF4\0\0\0\0\0\0\0\0\xAA\xBA\0\0\xDA\xFA\0\0\0\0\xD0\xF0\0\0\0\0\0\0\0\0\xDB\xFB\0\0\0\0\0\0\xD8\xF8\0\0\0\0\0\xCA\xEA\xDD\xFD\xDE\xFE\0\xC2\xE2\0\0\xC0\xE0\xC3\xE3\0\0\0\0\xC8\xE8\0\0\0\0\xC7\xE7\0\0\xCB\xEB\xC6\xE6\0\0\0\0\0\0\0\0\xCC\xEC\0\0\0\0\0\0\xCE\xEE\0\0\xC1\xE1\0\0\0\0\0\0\xCD\xED\0\0\0\xCF\xEF\0\0\0") };
static mut xmlunicodetable_ISO8859_14: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x1e02 as c_int as c_ushort,
    0x1e03 as c_int as c_ushort,
    0xa3 as c_int as c_ushort,
    0x10a as c_int as c_ushort,
    0x10b as c_int as c_ushort,
    0x1e0a as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0x1e80 as c_int as c_ushort,
    0xa9 as c_int as c_ushort,
    0x1e82 as c_int as c_ushort,
    0x1e0b as c_int as c_ushort,
    0x1ef2 as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0xae as c_int as c_ushort,
    0x178 as c_int as c_ushort,
    0x1e1e as c_int as c_ushort,
    0x1e1f as c_int as c_ushort,
    0x120 as c_int as c_ushort,
    0x121 as c_int as c_ushort,
    0x1e40 as c_int as c_ushort,
    0x1e41 as c_int as c_ushort,
    0xb6 as c_int as c_ushort,
    0x1e56 as c_int as c_ushort,
    0x1e81 as c_int as c_ushort,
    0x1e57 as c_int as c_ushort,
    0x1e83 as c_int as c_ushort,
    0x1e60 as c_int as c_ushort,
    0x1ef3 as c_int as c_ushort,
    0x1e84 as c_int as c_ushort,
    0x1e85 as c_int as c_ushort,
    0x1e61 as c_int as c_ushort,
    0xc0 as c_int as c_ushort,
    0xc1 as c_int as c_ushort,
    0xc2 as c_int as c_ushort,
    0xc3 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0xc5 as c_int as c_ushort,
    0xc6 as c_int as c_ushort,
    0xc7 as c_int as c_ushort,
    0xc8 as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0xca as c_int as c_ushort,
    0xcb as c_int as c_ushort,
    0xcc as c_int as c_ushort,
    0xcd as c_int as c_ushort,
    0xce as c_int as c_ushort,
    0xcf as c_int as c_ushort,
    0x174 as c_int as c_ushort,
    0xd1 as c_int as c_ushort,
    0xd2 as c_int as c_ushort,
    0xd3 as c_int as c_ushort,
    0xd4 as c_int as c_ushort,
    0xd5 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0x1e6a as c_int as c_ushort,
    0xd8 as c_int as c_ushort,
    0xd9 as c_int as c_ushort,
    0xda as c_int as c_ushort,
    0xdb as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0xdd as c_int as c_ushort,
    0x176 as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0xe0 as c_int as c_ushort,
    0xe1 as c_int as c_ushort,
    0xe2 as c_int as c_ushort,
    0xe3 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0xe5 as c_int as c_ushort,
    0xe6 as c_int as c_ushort,
    0xe7 as c_int as c_ushort,
    0xe8 as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0xea as c_int as c_ushort,
    0xeb as c_int as c_ushort,
    0xec as c_int as c_ushort,
    0xed as c_int as c_ushort,
    0xee as c_int as c_ushort,
    0xef as c_int as c_ushort,
    0x175 as c_int as c_ushort,
    0xf1 as c_int as c_ushort,
    0xf2 as c_int as c_ushort,
    0xf3 as c_int as c_ushort,
    0xf4 as c_int as c_ushort,
    0xf5 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0x1e6b as c_int as c_ushort,
    0xf8 as c_int as c_ushort,
    0xf9 as c_int as c_ushort,
    0xfa as c_int as c_ushort,
    0xfb as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0xfd as c_int as c_ushort,
    0x177 as c_int as c_ushort,
    0xff as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_14: [c_uchar; 688] = unsafe { ::core::mem::transmute::<[u8; 688], [c_uchar; 688]>(*b"\0\0\x01\t\x04\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\xA3\0\0\0\xA7\0\xA9\0\0\0\xAD\xAE\0\0\0\0\0\0\0\xB6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x08\x05\x06\0\0\0\0\0\0\xA1\xA2\0\0\0\0\0\0\xA6\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB0\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA4\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB2\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA8\xB8\xAA\xBA\xBD\xBE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xAC\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD0\xF0\xDE\xFE\xAF\0\0\0\0\0\0\0\xB4\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB7\xB9\0\0\0\0\0\0\0\0\xBB\xBF\0\0\0\0\0\0\0\0\xD7\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\0\xD1\xD2\xD3\xD4\xD5\xD6\0\xD8\xD9\xDA\xDB\xDC\xDD\0\xDF\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\0\xF1\xF2\xF3\xF4\xF5\xF6\0\xF8\xF9\xFA\xFB\xFC\xFD\0\xFF") };
static mut xmlunicodetable_ISO8859_15: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0xa1 as c_int as c_ushort,
    0xa2 as c_int as c_ushort,
    0xa3 as c_int as c_ushort,
    0x20ac as c_int as c_ushort,
    0xa5 as c_int as c_ushort,
    0x160 as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0x161 as c_int as c_ushort,
    0xa9 as c_int as c_ushort,
    0xaa as c_int as c_ushort,
    0xab as c_int as c_ushort,
    0xac as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0xae as c_int as c_ushort,
    0xaf as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0xb1 as c_int as c_ushort,
    0xb2 as c_int as c_ushort,
    0xb3 as c_int as c_ushort,
    0x17d as c_int as c_ushort,
    0xb5 as c_int as c_ushort,
    0xb6 as c_int as c_ushort,
    0xb7 as c_int as c_ushort,
    0x17e as c_int as c_ushort,
    0xb9 as c_int as c_ushort,
    0xba as c_int as c_ushort,
    0xbb as c_int as c_ushort,
    0x152 as c_int as c_ushort,
    0x153 as c_int as c_ushort,
    0x178 as c_int as c_ushort,
    0xbf as c_int as c_ushort,
    0xc0 as c_int as c_ushort,
    0xc1 as c_int as c_ushort,
    0xc2 as c_int as c_ushort,
    0xc3 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0xc5 as c_int as c_ushort,
    0xc6 as c_int as c_ushort,
    0xc7 as c_int as c_ushort,
    0xc8 as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0xca as c_int as c_ushort,
    0xcb as c_int as c_ushort,
    0xcc as c_int as c_ushort,
    0xcd as c_int as c_ushort,
    0xce as c_int as c_ushort,
    0xcf as c_int as c_ushort,
    0xd0 as c_int as c_ushort,
    0xd1 as c_int as c_ushort,
    0xd2 as c_int as c_ushort,
    0xd3 as c_int as c_ushort,
    0xd4 as c_int as c_ushort,
    0xd5 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0xd7 as c_int as c_ushort,
    0xd8 as c_int as c_ushort,
    0xd9 as c_int as c_ushort,
    0xda as c_int as c_ushort,
    0xdb as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0xdd as c_int as c_ushort,
    0xde as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0xe0 as c_int as c_ushort,
    0xe1 as c_int as c_ushort,
    0xe2 as c_int as c_ushort,
    0xe3 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0xe5 as c_int as c_ushort,
    0xe6 as c_int as c_ushort,
    0xe7 as c_int as c_ushort,
    0xe8 as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0xea as c_int as c_ushort,
    0xeb as c_int as c_ushort,
    0xec as c_int as c_ushort,
    0xed as c_int as c_ushort,
    0xee as c_int as c_ushort,
    0xef as c_int as c_ushort,
    0xf0 as c_int as c_ushort,
    0xf1 as c_int as c_ushort,
    0xf2 as c_int as c_ushort,
    0xf3 as c_int as c_ushort,
    0xf4 as c_int as c_ushort,
    0xf5 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0xf7 as c_int as c_ushort,
    0xf8 as c_int as c_ushort,
    0xf9 as c_int as c_ushort,
    0xfa as c_int as c_ushort,
    0xfb as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0xfd as c_int as c_ushort,
    0xfe as c_int as c_ushort,
    0xff as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_15: [c_uchar; 432] = unsafe { ::core::mem::transmute::<[u8; 432], [c_uchar; 432]>(*b"\0\0\x01\x05\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\xA1\xA2\xA3\0\xA5\0\xA7\0\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\0\xB5\xB6\xB7\0\xB9\xBA\xBB\0\0\0\xBF\0\0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xBC\xBD\0\0\0\0\0\0\0\0\0\0\0\0\xA6\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xBE\0\0\0\0\xB4\xB8\0\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\xD0\xD1\xD2\xD3\xD4\xD5\xD6\xD7\xD8\xD9\xDA\xDB\xDC\xDD\xDE\xDF\xE0\xE1\xE2\xE3\xE4\xE5\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\xF0\xF1\xF2\xF3\xF4\xF5\xF6\xF7\xF8\xF9\xFA\xFB\xFC\xFD\xFE\xFF") };
static mut xmlunicodetable_ISO8859_16: [c_ushort; 128] = [
    0x80 as c_int as c_ushort,
    0x81 as c_int as c_ushort,
    0x82 as c_int as c_ushort,
    0x83 as c_int as c_ushort,
    0x84 as c_int as c_ushort,
    0x85 as c_int as c_ushort,
    0x86 as c_int as c_ushort,
    0x87 as c_int as c_ushort,
    0x88 as c_int as c_ushort,
    0x89 as c_int as c_ushort,
    0x8a as c_int as c_ushort,
    0x8b as c_int as c_ushort,
    0x8c as c_int as c_ushort,
    0x8d as c_int as c_ushort,
    0x8e as c_int as c_ushort,
    0x8f as c_int as c_ushort,
    0x90 as c_int as c_ushort,
    0x91 as c_int as c_ushort,
    0x92 as c_int as c_ushort,
    0x93 as c_int as c_ushort,
    0x94 as c_int as c_ushort,
    0x95 as c_int as c_ushort,
    0x96 as c_int as c_ushort,
    0x97 as c_int as c_ushort,
    0x98 as c_int as c_ushort,
    0x99 as c_int as c_ushort,
    0x9a as c_int as c_ushort,
    0x9b as c_int as c_ushort,
    0x9c as c_int as c_ushort,
    0x9d as c_int as c_ushort,
    0x9e as c_int as c_ushort,
    0x9f as c_int as c_ushort,
    0xa0 as c_int as c_ushort,
    0x104 as c_int as c_ushort,
    0x105 as c_int as c_ushort,
    0x141 as c_int as c_ushort,
    0x20ac as c_int as c_ushort,
    0x201e as c_int as c_ushort,
    0x160 as c_int as c_ushort,
    0xa7 as c_int as c_ushort,
    0x161 as c_int as c_ushort,
    0xa9 as c_int as c_ushort,
    0x218 as c_int as c_ushort,
    0xab as c_int as c_ushort,
    0x179 as c_int as c_ushort,
    0xad as c_int as c_ushort,
    0x17a as c_int as c_ushort,
    0x17b as c_int as c_ushort,
    0xb0 as c_int as c_ushort,
    0xb1 as c_int as c_ushort,
    0x10c as c_int as c_ushort,
    0x142 as c_int as c_ushort,
    0x17d as c_int as c_ushort,
    0x201d as c_int as c_ushort,
    0xb6 as c_int as c_ushort,
    0xb7 as c_int as c_ushort,
    0x17e as c_int as c_ushort,
    0x10d as c_int as c_ushort,
    0x219 as c_int as c_ushort,
    0xbb as c_int as c_ushort,
    0x152 as c_int as c_ushort,
    0x153 as c_int as c_ushort,
    0x178 as c_int as c_ushort,
    0x17c as c_int as c_ushort,
    0xc0 as c_int as c_ushort,
    0xc1 as c_int as c_ushort,
    0xc2 as c_int as c_ushort,
    0x102 as c_int as c_ushort,
    0xc4 as c_int as c_ushort,
    0x106 as c_int as c_ushort,
    0xc6 as c_int as c_ushort,
    0xc7 as c_int as c_ushort,
    0xc8 as c_int as c_ushort,
    0xc9 as c_int as c_ushort,
    0xca as c_int as c_ushort,
    0xcb as c_int as c_ushort,
    0xcc as c_int as c_ushort,
    0xcd as c_int as c_ushort,
    0xce as c_int as c_ushort,
    0xcf as c_int as c_ushort,
    0x110 as c_int as c_ushort,
    0x143 as c_int as c_ushort,
    0xd2 as c_int as c_ushort,
    0xd3 as c_int as c_ushort,
    0xd4 as c_int as c_ushort,
    0x150 as c_int as c_ushort,
    0xd6 as c_int as c_ushort,
    0x15a as c_int as c_ushort,
    0x170 as c_int as c_ushort,
    0xd9 as c_int as c_ushort,
    0xda as c_int as c_ushort,
    0xdb as c_int as c_ushort,
    0xdc as c_int as c_ushort,
    0x118 as c_int as c_ushort,
    0x21a as c_int as c_ushort,
    0xdf as c_int as c_ushort,
    0xe0 as c_int as c_ushort,
    0xe1 as c_int as c_ushort,
    0xe2 as c_int as c_ushort,
    0x103 as c_int as c_ushort,
    0xe4 as c_int as c_ushort,
    0x107 as c_int as c_ushort,
    0xe6 as c_int as c_ushort,
    0xe7 as c_int as c_ushort,
    0xe8 as c_int as c_ushort,
    0xe9 as c_int as c_ushort,
    0xea as c_int as c_ushort,
    0xeb as c_int as c_ushort,
    0xec as c_int as c_ushort,
    0xed as c_int as c_ushort,
    0xee as c_int as c_ushort,
    0xef as c_int as c_ushort,
    0x111 as c_int as c_ushort,
    0x144 as c_int as c_ushort,
    0xf2 as c_int as c_ushort,
    0xf3 as c_int as c_ushort,
    0xf4 as c_int as c_ushort,
    0x151 as c_int as c_ushort,
    0xf6 as c_int as c_ushort,
    0x15b as c_int as c_ushort,
    0x171 as c_int as c_ushort,
    0xf9 as c_int as c_ushort,
    0xfa as c_int as c_ushort,
    0xfb as c_int as c_ushort,
    0xfc as c_int as c_ushort,
    0x119 as c_int as c_ushort,
    0x21b as c_int as c_ushort,
    0xff as c_int as c_ushort,
];
static mut xmltranscodetable_ISO8859_16: [c_uchar; 624] = unsafe { ::core::mem::transmute::<[u8; 624], [c_uchar; 624]>(*b"\0\0\x01\x08\x02\x03\0\0\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\xA0\0\0\0\0\0\0\xA7\0\xA9\0\xAB\0\xAD\0\0\xB0\xB1\0\0\0\0\xB6\xB7\0\0\0\xBB\0\0\0\0\0\0\xC3\xE3\xA1\xA2\xC5\xE5\0\0\0\0\xB2\xB9\0\0\xD0\xF0\0\0\0\0\0\0\xDD\xFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA3\xB3\xD1\xF1\0\0\0\0\0\0\0\0\0\0\0\xD5\xF5\xBC\xBD\0\0\0\0\0\0\xD7\xF7\0\0\0\0\xA6\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xD8\xF8\0\0\0\0\0\0\xBE\xAC\xAE\xAF\xBF\xB4\xB8\0\x06\0\x05\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xB5\xA5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xAA\xBA\xDE\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\xC0\xC1\xC2\0\xC4\0\xC6\xC7\xC8\xC9\xCA\xCB\xCC\xCD\xCE\xCF\0\0\xD2\xD3\xD4\0\xD6\0\0\xD9\xDA\xDB\xDC\0\0\xDF\xE0\xE1\xE2\0\xE4\0\xE6\xE7\xE8\xE9\xEA\xEB\xEC\xED\xEE\xEF\0\0\xF2\xF3\xF4\0\xF6\0\0\xF9\xFA\xFB\xFC\0\0\xFF") };
unsafe extern "C" fn ISO8859_2ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_2 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_2(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_2 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_3ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_3 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_3(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_3 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_4ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_4 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_4(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_4 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_5ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_5 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_5(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_5 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_6ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_6 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_6(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_6 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_7ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_7 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_7(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_7 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_8ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_8 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_8 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_9ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_9 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_9(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_9 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_10ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_10 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_10(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_10 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_11ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_11 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_11(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_11 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_13ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_13 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_13(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_13 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_14ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_14 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_14(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_14 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_15ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_15 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_15(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_15 as *const c_uchar,
    );
}
unsafe extern "C" fn ISO8859_16ToUTF8(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return ISO8859xToUTF8(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmlunicodetable_ISO8859_16 as *const c_ushort,
    );
}
unsafe extern "C" fn UTF8ToISO8859_16(
    mut out: *mut c_uchar,
    mut outlen: *mut c_int,
    mut in_0: *const c_uchar,
    mut inlen: *mut c_int,
) -> c_int {
    return UTF8ToISO8859x(
        out,
        outlen,
        in_0,
        inlen,
        &raw const xmltranscodetable_ISO8859_16 as *const c_uchar,
    );
}

pub const ULONG_MAX: c_ulong = (__LONG_MAX__ as c_ulong)
    .wrapping_mul(2 as c_ulong)
    .wrapping_add(1 as c_ulong);

extern "C" fn run_static_initializers() { unsafe {
    xmlUTF16BEHandler = (&raw const defaultHandlers as *const xmlCharEncodingHandler)
        .offset(2 as c_int as isize)
        as *const xmlCharEncodingHandler;
    xmlUTF16LEHandler = (&raw const defaultHandlers as *const xmlCharEncodingHandler)
        .offset(1 as c_int as isize)
        as *const xmlCharEncodingHandler;
} }
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
