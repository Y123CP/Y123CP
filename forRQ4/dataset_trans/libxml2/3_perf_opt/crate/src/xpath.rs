use core::ffi::*;
use crate::src::globals::__xmlGenericError;
use crate::src::globals::__xmlGenericErrorContext;
use crate::src::c_inlined_fns::toupper;
use crate::src::buf::xmlBufAdd;
use crate::src::buf::xmlBufContent;
use crate::src::buf::xmlBufCreate;
use crate::src::buf::xmlBufFree;
use crate::src::tree::xmlBuildQName;
use crate::src::parserInternals::xmlCopyChar;
use crate::src::debugXML::xmlDebugDumpString;
use crate::src::dict::xmlDictFree;
use crate::src::dict::xmlDictLookup;
use crate::src::dict::xmlDictReference;
use crate::src::pattern::xmlFreePattern;
use crate::src::pattern::xmlFreePatternList;
use crate::src::pattern::xmlFreeStreamCtxt;
use crate::src::hash::xmlHashAddEntry;
use crate::src::hash::xmlHashAddEntry2;
use crate::src::hash::xmlHashCreate;
use crate::src::hash::xmlHashFree;
use crate::src::hash::xmlHashLookup;
use crate::src::hash::xmlHashLookup2;
use crate::src::hash::xmlHashRemoveEntry;
use crate::src::hash::xmlHashRemoveEntry2;
use crate::src::hash::xmlHashUpdateEntry;
use crate::src::hash::xmlHashUpdateEntry2;
use crate::src::threads::xmlInitParser;
use crate::src::pattern::xmlPatternFromRoot;
use crate::src::pattern::xmlPatternGetStreamCtxt;
use crate::src::pattern::xmlPatternMaxDepth;
use crate::src::pattern::xmlPatternMinDepth;
use crate::src::pattern::xmlPatternStreamable;
use crate::src::pattern::xmlPatterncompile;
use crate::src::error::xmlResetError;
use crate::src::xmlstring::xmlStrEqual;
use crate::src::xmlstring::xmlStrcat;
use crate::src::xmlstring::xmlStrchr;
use crate::src::xmlstring::xmlStrdup;
use crate::src::pattern::xmlStreamPop;
use crate::src::pattern::xmlStreamPush;
use crate::src::pattern::xmlStreamPushNode;
use crate::src::pattern::xmlStreamWantsAnyNode;
use crate::src::xmlstring::xmlStrncmp;
use crate::src::xmlstring::xmlStrndup;
use crate::src::xmlstring::xmlStrstr;
use crate::src::xmlstring::xmlUTF8Strlen;
use crate::src::xmlstring::xmlUTF8Strloc;
use crate::src::xmlstring::xmlUTF8Strpos;
use crate::src::xmlstring::xmlUTF8Strsize;
use crate::src::xmlstring::xmlUTF8Strsub;
pub use crate::src::c_inlined_fns::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
pub use crate::src::pattern::_xmlPattern;
pub use crate::src::pattern::_xmlStreamCtxt;
pub use crate::src::dict::_xmlDict;
pub use crate::src::hash::_xmlHashTable;
pub use crate::src::buf::_xmlBuf;
extern "C" {
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlFreeNodeList(cur: xmlNodePtr);
    fn xmlGetNsList(doc: *const xmlDoc, node: *const xmlNode) -> *mut xmlNsPtr;
    fn xmlNodeGetContent(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlGetID(doc: xmlDocPtr, ID: *const xmlChar) -> xmlAttrPtr;
    fn xmlCharInRange(
        val: c_uint,
        group: *const xmlChRangeGroup,
    ) -> c_int;
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    static xmlIsCombiningGroup: xmlChRangeGroup;
    static xmlIsDigitGroup: xmlChRangeGroup;
    static xmlIsExtenderGroup: xmlChRangeGroup;
    fn xmlDebugDumpAttr(output: *mut FILE, attr: xmlAttrPtr, depth: c_int);
    fn xmlDebugDumpOneNode(output: *mut FILE, node: xmlNodePtr, depth: c_int);
}

pub type __uint64_t = u64;

pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
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

pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;

pub type xmlNsPtr = *mut xmlNs;

pub type C2RustUnnamed_htdd24ee73 = c_uint;
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
pub const XML_FROM_TREE: C2RustUnnamed_htdd24ee73 = 2;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathContext {
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub nb_variables_unused: c_int,
    pub max_variables_unused: c_int,
    pub varHash: xmlHashTablePtr,
    pub nb_types: c_int,
    pub max_types: c_int,
    pub types: xmlXPathTypePtr,
    pub nb_funcs_unused: c_int,
    pub max_funcs_unused: c_int,
    pub funcHash: xmlHashTablePtr,
    pub nb_axis: c_int,
    pub max_axis: c_int,
    pub axis: xmlXPathAxisPtr,
    pub namespaces: *mut xmlNsPtr,
    pub nsNr: c_int,
    pub user: *mut c_void,
    pub contextSize: c_int,
    pub proximityPosition: c_int,
    pub xptr: c_int,
    pub here: xmlNodePtr,
    pub origin: xmlNodePtr,
    pub nsHash: xmlHashTablePtr,
    pub varLookupFunc: xmlXPathVariableLookupFunc,
    pub varLookupData: *mut c_void,
    pub extra: *mut c_void,
    pub function: *const xmlChar,
    pub functionURI: *const xmlChar,
    pub funcLookupFunc: xmlXPathFuncLookupFunc,
    pub funcLookupData: *mut c_void,
    pub tmpNsList: *mut xmlNsPtr,
    pub tmpNsNr: c_int,
    pub userData: *mut c_void,
    pub error: xmlStructuredErrorFunc,
    pub lastError: xmlError,
    pub debugNode: xmlNodePtr,
    pub dict: xmlDictPtr,
    pub flags: c_int,
    pub cache: *mut c_void,
    pub opLimit: c_ulong,
    pub opCount: c_ulong,
    pub depth: c_int,
}
pub type xmlXPathFuncLookupFunc = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlXPathFunction,
>;
pub type xmlXPathFunction =
    Option<unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> ()>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: c_int,
    pub context: xmlXPathContextPtr,
    pub value: xmlXPathObjectPtr,
    pub valueNr: c_int,
    pub valueMax: c_int,
    pub valueTab: *mut xmlXPathObjectPtr,
    pub comp: xmlXPathCompExprPtr,
    pub xptr: c_int,
    pub ancestor: xmlNodePtr,
    pub valueFrame: c_int,
}
pub type xmlXPathCompExprPtr = *mut xmlXPathCompExpr;
pub type xmlXPathCompExpr = _xmlXPathCompExpr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathCompExpr {
    pub nbStep: c_int,
    pub maxStep: c_int,
    pub steps: *mut xmlXPathStepOp,
    pub last: c_int,
    pub expr: *mut xmlChar,
    pub dict: xmlDictPtr,
    pub stream: xmlPatternPtr,
}
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlPattern = _xmlPattern;
pub type xmlXPathStepOp = _xmlXPathStepOp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathStepOp {
    pub op: xmlXPathOp,
    pub ch1: c_int,
    pub ch2: c_int,
    pub value: c_int,
    pub value2: c_int,
    pub value3: c_int,
    pub value4: *mut c_void,
    pub value5: *mut c_void,
    pub cache: xmlXPathFunction,
    pub cacheURI: *mut c_void,
}
pub type xmlXPathOp = c_uint;
pub const XPATH_OP_SORT: xmlXPathOp = 17;
pub const XPATH_OP_FILTER: xmlXPathOp = 16;
pub const XPATH_OP_PREDICATE: xmlXPathOp = 15;
pub const XPATH_OP_ARG: xmlXPathOp = 14;
pub const XPATH_OP_FUNCTION: xmlXPathOp = 13;
pub const XPATH_OP_VARIABLE: xmlXPathOp = 12;
pub const XPATH_OP_VALUE: xmlXPathOp = 11;
pub const XPATH_OP_COLLECT: xmlXPathOp = 10;
pub const XPATH_OP_NODE: xmlXPathOp = 9;
pub const XPATH_OP_ROOT: xmlXPathOp = 8;
pub const XPATH_OP_UNION: xmlXPathOp = 7;
pub const XPATH_OP_MULT: xmlXPathOp = 6;
pub const XPATH_OP_PLUS: xmlXPathOp = 5;
pub const XPATH_OP_CMP: xmlXPathOp = 4;
pub const XPATH_OP_EQUAL: xmlXPathOp = 3;
pub const XPATH_OP_OR: xmlXPathOp = 2;
pub const XPATH_OP_AND: xmlXPathOp = 1;
pub const XPATH_OP_END: xmlXPathOp = 0;
pub type xmlXPathObjectPtr = *mut xmlXPathObject;
pub type xmlXPathObject = _xmlXPathObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathObject {
    pub type_0: xmlXPathObjectType,
    pub nodesetval: xmlNodeSetPtr,
    pub boolval: c_int,
    pub floatval: c_double,
    pub stringval: *mut xmlChar,
    pub user: *mut c_void,
    pub index: c_int,
    pub user2: *mut c_void,
    pub index2: c_int,
}
pub type xmlNodeSetPtr = *mut xmlNodeSet;
pub type xmlNodeSet = _xmlNodeSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeSet {
    pub nodeNr: c_int,
    pub nodeMax: c_int,
    pub nodeTab: *mut xmlNodePtr,
}

pub type xmlXPathContextPtr = *mut xmlXPathContext;
pub type xmlXPathContext = _xmlXPathContext;
pub type xmlXPathVariableLookupFunc = Option<
    unsafe extern "C" fn(
        *mut c_void,
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
    Option<unsafe extern "C" fn(xmlXPathObjectPtr, c_int) -> c_int>;
pub const XPATH_RECURSION_LIMIT_EXCEEDED: C2RustUnnamed_htdd24ee73 = 26;
pub const XPATH_OP_LIMIT_EXCEEDED: C2RustUnnamed_htdd24ee73 = 25;
pub const XPATH_FORBID_VARIABLE_ERROR: C2RustUnnamed_htdd24ee73 = 24;
pub const XPATH_STACK_ERROR: C2RustUnnamed_htdd24ee73 = 23;
pub const XPATH_INVALID_CTXT: C2RustUnnamed_htdd24ee73 = 22;
pub const XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_htdd24ee73 = 21;
pub const XPATH_ENCODING_ERROR: C2RustUnnamed_htdd24ee73 = 20;
pub const XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_htdd24ee73 = 19;
pub const XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_htdd24ee73 = 18;
pub const XPTR_RESOURCE_ERROR: C2RustUnnamed_htdd24ee73 = 17;
pub const XPTR_SYNTAX_ERROR: C2RustUnnamed_htdd24ee73 = 16;
pub const XPATH_MEMORY_ERROR: C2RustUnnamed_htdd24ee73 = 15;
pub const XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_htdd24ee73 = 14;
pub const XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_htdd24ee73 = 13;
pub const XPATH_INVALID_ARITY: C2RustUnnamed_htdd24ee73 = 12;
pub const XPATH_INVALID_TYPE: C2RustUnnamed_htdd24ee73 = 11;
pub const XPATH_INVALID_OPERAND: C2RustUnnamed_htdd24ee73 = 10;
pub const XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_htdd24ee73 = 9;
pub const XPATH_UNCLOSED_ERROR: C2RustUnnamed_htdd24ee73 = 8;
pub const XPATH_EXPR_ERROR: C2RustUnnamed_htdd24ee73 = 7;
pub const XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_htdd24ee73 = 6;
pub const XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_htdd24ee73 = 5;
pub const XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_htdd24ee73 = 4;
pub const XPATH_START_LITERAL_ERROR: C2RustUnnamed_htdd24ee73 = 3;
pub const XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_htdd24ee73 = 2;
pub const XPATH_NUMBER_ERROR: C2RustUnnamed_htdd24ee73 = 1;
pub const XPATH_EXPRESSION_OK: C2RustUnnamed_htdd24ee73 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TIM_SORT_RUN_T {
    pub start: size_t,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TEMP_STORAGE_T {
    pub alloc: size_t,
    pub storage: *mut xmlNodePtr,
}
pub type uint64_t = __uint64_t;
pub type xmlPointerListPtr = *mut xmlPointerList;
pub type xmlPointerList = _xmlPointerList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlPointerList {
    pub items: *mut *mut c_void,
    pub number: c_int,
    pub size: c_int,
}
pub type xmlXPathContextCachePtr = *mut xmlXPathContextCache;
pub type xmlXPathContextCache = _xmlXPathContextCache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathContextCache {
    pub nodesetObjs: xmlPointerListPtr,
    pub stringObjs: xmlPointerListPtr,
    pub booleanObjs: xmlPointerListPtr,
    pub numberObjs: xmlPointerListPtr,
    pub miscObjs: xmlPointerListPtr,
    pub maxNodeset: c_int,
    pub maxString: c_int,
    pub maxBoolean: c_int,
    pub maxNumber: c_int,
    pub maxMisc: c_int,
}
pub type xmlXPathStepOpPtr = *mut xmlXPathStepOp;
pub type xmlXPathNodeSetMergeFunction =
    Option<unsafe extern "C" fn(xmlNodeSetPtr, xmlNodeSetPtr) -> xmlNodeSetPtr>;
pub const AXIS_NAMESPACE: xmlXPathAxisVal = 9;
pub type xmlXPathAxisVal = c_uint;
pub const AXIS_SELF: xmlXPathAxisVal = 13;
pub const AXIS_PRECEDING_SIBLING: xmlXPathAxisVal = 12;
pub const AXIS_PRECEDING: xmlXPathAxisVal = 11;
pub const AXIS_PARENT: xmlXPathAxisVal = 10;
pub const AXIS_FOLLOWING_SIBLING: xmlXPathAxisVal = 8;
pub const AXIS_FOLLOWING: xmlXPathAxisVal = 7;
pub const AXIS_DESCENDANT_OR_SELF: xmlXPathAxisVal = 6;
pub const AXIS_DESCENDANT: xmlXPathAxisVal = 5;
pub const AXIS_CHILD: xmlXPathAxisVal = 4;
pub const AXIS_ATTRIBUTE: xmlXPathAxisVal = 3;
pub const AXIS_ANCESTOR_OR_SELF: xmlXPathAxisVal = 2;
pub const AXIS_ANCESTOR: xmlXPathAxisVal = 1;
pub const NODE_TEST_NAME: xmlXPathTestVal = 5;
pub const NODE_TEST_NS: xmlXPathTestVal = 4;
pub const NODE_TEST_ALL: xmlXPathTestVal = 3;
pub const NODE_TEST_PI: xmlXPathTestVal = 2;
pub const NODE_TYPE_TEXT: xmlXPathTypeVal = 3;
pub type xmlXPathTypeVal = c_uint;
pub const NODE_TYPE_PI: xmlXPathTypeVal = 7;
pub const NODE_TYPE_COMMENT: xmlXPathTypeVal = 8;
pub const NODE_TYPE_NODE: xmlXPathTypeVal = 0;
pub const NODE_TEST_TYPE: xmlXPathTestVal = 1;
pub const NODE_TEST_NONE: xmlXPathTestVal = 0;
pub type xmlXPathTestVal = c_uint;
pub type xmlXPathTraversalFunction =
    Option<unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr>;
pub type xmlStreamCtxtPtr = *mut xmlStreamCtxt;
pub type xmlStreamCtxt = _xmlStreamCtxt;
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: c_int,
    pub nbLongRange: c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}

pub const XML_PATTERN_XPATH: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_PATTERN_XSFIELD: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_PATTERN_XSSEL: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_PATTERN_DEFAULT: C2RustUnnamed_htdd24ee73 = 0;

pub const XML_XML_NAMESPACE: *const xmlChar = b"http://www.w3.org/XML/1998/namespace\0" as *const u8
    as *const c_char as *const xmlChar;

pub const XML_XPATH_NOVAR: c_int =
    (1 as c_int) << 1 as c_int;
pub const XPATH_MAX_STEPS: c_int = 1000000 as c_int;
pub const XPATH_MAX_STACK_DEPTH: c_int = 1000000 as c_int;
pub const XPATH_MAX_NODESET_LENGTH: c_int = 10000000 as c_int;
pub const XPATH_MAX_RECURSION_DEPTH: c_int = 5000 as c_int;
#[no_mangle]
pub static mut xmlXPathNAN: c_double = 0.0f64;
#[no_mangle]
pub static mut xmlXPathPINF: c_double = 0.0f64;
#[no_mangle]
pub static mut xmlXPathNINF: c_double = 0.0f64;
#[inline]
pub fn xmlXPathInit() { {
    xmlInitParser();
} }
#[inline]
pub fn xmlInitXPathInternal() { unsafe {
    xmlXPathNAN = ::core::f32::NAN as c_double;
    xmlXPathPINF = ::core::f32::INFINITY as c_double;
    xmlXPathNINF = -::core::f32::INFINITY as c_double;
} }
#[inline]
pub fn xmlXPathIsNaN(mut val: c_double) -> c_int { {
    return val.is_nan() as i32;
} }
#[inline]
pub fn xmlXPathIsInf(mut val: c_double) -> c_int { {
    return if if val.is_infinite() {
        if val.is_sign_positive() {
            1
        } else {
            -1
        }
    } else {
        0
    } != 0
    {
        if val > 0 as c_int as c_double {
            1 as c_int
        } else {
            -(1 as c_int)
        }
    } else {
        0 as c_int
    };
} }
static mut xmlXPathXMLNamespaceStruct: xmlNs = _xmlNs {
    next: ::core::ptr::null::<_xmlNs>() as *mut _xmlNs,
    type_0: XML_NAMESPACE_DECL,
    href: XML_XML_NAMESPACE,
    prefix: b"xml\0" as *const u8 as *const c_char as *mut xmlChar,
    _private: NULL_0,
    context: ::core::ptr::null::<_xmlDoc>() as *mut _xmlDoc,
};
static mut xmlXPathXMLNamespace: xmlNsPtr =
    unsafe { &raw const xmlXPathXMLNamespaceStruct as xmlNsPtr };
fn xmlXPathCmpNodesExt(
    mut node1: xmlNodePtr,
    mut node2: xmlNodePtr,
) -> c_int { unsafe {
    let mut current_block: u64;
    let mut depth1: c_int = 0;
    let mut depth2: c_int = 0;
    let mut misc: c_int = 0 as c_int;
    let mut precedence1: c_int = 0 as c_int;
    let mut precedence2: c_int = 0 as c_int;
    let mut miscNode1: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut miscNode2: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut root: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut l1: ptrdiff_t = 0;
    let mut l2: ptrdiff_t = 0;
    if node1.is_null() || node2.is_null() {
        return -(2 as c_int);
    }
    if node1 == node2 {
        return 0 as c_int;
    }
    match (*node1).type_0 as c_uint {
        1 => {
            if (*node2).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
            {
                if 0 as ptrdiff_t > (*node1).content as ptrdiff_t
                    && 0 as ptrdiff_t > (*node2).content as ptrdiff_t
                    && (*node1).doc == (*node2).doc
                {
                    l1 = -((*node1).content as ptrdiff_t);
                    l2 = -((*node2).content as ptrdiff_t);
                    if l1 < l2 {
                        return 1 as c_int;
                    }
                    if l1 > l2 {
                        return -(1 as c_int);
                    }
                    current_block = 6450636197030046351;
                } else {
                    current_block = 2143635778540179221;
                }
            } else {
                current_block = 6450636197030046351;
            }
        }
        2 => {
            precedence1 = 1 as c_int;
            miscNode1 = node1;
            node1 = (*node1).parent as xmlNodePtr;
            misc = 1 as c_int;
            current_block = 6450636197030046351;
        }
        3 | 4 | 8 | 7 => {
            miscNode1 = node1;
            if !(*node1).prev.is_null() {
                loop {
                    node1 = (*node1).prev as xmlNodePtr;
                    if (*node1).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                    {
                        precedence1 = 3 as c_int;
                        break;
                    } else {
                        if !(*node1).prev.is_null() {
                            continue;
                        }
                        precedence1 = 2 as c_int;
                        node1 = (*node1).parent as xmlNodePtr;
                        break;
                    }
                }
            } else {
                precedence1 = 2 as c_int;
                node1 = (*node1).parent as xmlNodePtr;
            }
            if node1.is_null()
                || (*node1).type_0 as c_uint
                    != XML_ELEMENT_NODE as c_int as c_uint
                || 0 as ptrdiff_t <= (*node1).content as ptrdiff_t
            {
                node1 = miscNode1;
                precedence1 = 0 as c_int;
            } else {
                misc = 1 as c_int;
            }
            current_block = 6450636197030046351;
        }
        18 => return 1 as c_int,
        _ => {
            current_block = 6450636197030046351;
        }
    }
    match current_block {
        6450636197030046351 => {
            match (*node2).type_0 as c_uint {
                2 => {
                    precedence2 = 1 as c_int;
                    miscNode2 = node2;
                    node2 = (*node2).parent as xmlNodePtr;
                    misc = 1 as c_int;
                }
                3 | 4 | 8 | 7 => {
                    miscNode2 = node2;
                    if !(*node2).prev.is_null() {
                        loop {
                            node2 = (*node2).prev as xmlNodePtr;
                            if (*node2).type_0 as c_uint
                                == XML_ELEMENT_NODE as c_int as c_uint
                            {
                                precedence2 = 3 as c_int;
                                break;
                            } else {
                                if !(*node2).prev.is_null() {
                                    continue;
                                }
                                precedence2 = 2 as c_int;
                                node2 = (*node2).parent as xmlNodePtr;
                                break;
                            }
                        }
                    } else {
                        precedence2 = 2 as c_int;
                        node2 = (*node2).parent as xmlNodePtr;
                    }
                    if node2.is_null()
                        || (*node2).type_0 as c_uint
                            != XML_ELEMENT_NODE as c_int as c_uint
                        || 0 as ptrdiff_t <= (*node2).content as ptrdiff_t
                    {
                        node2 = miscNode2;
                        precedence2 = 0 as c_int;
                    } else {
                        misc = 1 as c_int;
                    }
                }
                18 => return 1 as c_int,
                1 | _ => {}
            }
            if misc != 0 {
                if node1 == node2 {
                    if precedence1 == precedence2 {
                        cur = (*miscNode2).prev as xmlNodePtr;
                        while !cur.is_null() {
                            if cur == miscNode1 {
                                return 1 as c_int;
                            }
                            if (*cur).type_0 as c_uint
                                == XML_ELEMENT_NODE as c_int as c_uint
                            {
                                return -(1 as c_int);
                            }
                            cur = (*cur).prev as xmlNodePtr;
                        }
                        return -(1 as c_int);
                    } else if precedence1 < precedence2 {
                        return 1 as c_int;
                    } else {
                        return -(1 as c_int);
                    }
                }
                if precedence2 == 3 as c_int && precedence1 > 1 as c_int {
                    cur = (*node1).parent as xmlNodePtr;
                    while !cur.is_null() {
                        if cur == node2 {
                            return 1 as c_int;
                        }
                        cur = (*cur).parent as xmlNodePtr;
                    }
                }
                if precedence1 == 3 as c_int && precedence2 > 1 as c_int {
                    cur = (*node2).parent as xmlNodePtr;
                    while !cur.is_null() {
                        if cur == node1 {
                            return -(1 as c_int);
                        }
                        cur = (*cur).parent as xmlNodePtr;
                    }
                }
            }
            if (*node1).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
                && (*node2).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                && 0 as ptrdiff_t > (*node1).content as ptrdiff_t
                && 0 as ptrdiff_t > (*node2).content as ptrdiff_t
                && (*node1).doc == (*node2).doc
            {
                l1 = -((*node1).content as ptrdiff_t);
                l2 = -((*node2).content as ptrdiff_t);
                if l1 < l2 {
                    return 1 as c_int;
                }
                if l1 > l2 {
                    return -(1 as c_int);
                }
            }
        }
        _ => {}
    }
    if node1 == (*node2).prev {
        return 1 as c_int;
    }
    if node1 == (*node2).next {
        return -(1 as c_int);
    }
    depth2 = 0 as c_int;
    cur = node2;
    while !(*cur).parent.is_null() {
        if (*cur).parent == node1 {
            return 1 as c_int;
        }
        depth2 += 1;
        cur = (*cur).parent as xmlNodePtr;
    }
    root = cur;
    depth1 = 0 as c_int;
    cur = node1;
    while !(*cur).parent.is_null() {
        if (*cur).parent == node2 {
            return -(1 as c_int);
        }
        depth1 += 1;
        cur = (*cur).parent as xmlNodePtr;
    }
    if root != cur {
        return -(2 as c_int);
    }
    while depth1 > depth2 {
        depth1 -= 1;
        node1 = (*node1).parent as xmlNodePtr;
    }
    while depth2 > depth1 {
        depth2 -= 1;
        node2 = (*node2).parent as xmlNodePtr;
    }
    while (*node1).parent != (*node2).parent {
        node1 = (*node1).parent as xmlNodePtr;
        node2 = (*node2).parent as xmlNodePtr;
        if node1.is_null() || node2.is_null() {
            return -(2 as c_int);
        }
    }
    if node1 == (*node2).prev {
        return 1 as c_int;
    }
    if node1 == (*node2).next {
        return -(1 as c_int);
    }
    if (*node1).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        && (*node2).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        && 0 as ptrdiff_t > (*node1).content as ptrdiff_t
        && 0 as ptrdiff_t > (*node2).content as ptrdiff_t
        && (*node1).doc == (*node2).doc
    {
        l1 = -((*node1).content as ptrdiff_t);
        l2 = -((*node2).content as ptrdiff_t);
        if l1 < l2 {
            return 1 as c_int;
        }
        if l1 > l2 {
            return -(1 as c_int);
        }
    }
    cur = (*node1).next as xmlNodePtr;
    while !cur.is_null() {
        if cur == node2 {
            return 1 as c_int;
        }
        cur = (*cur).next as xmlNodePtr;
    }
    return -(1 as c_int);
} }
fn wrap_cmp(mut x: xmlNodePtr, mut y: xmlNodePtr) -> c_int { {
    let mut res: c_int = xmlXPathCmpNodesExt(x, y);
    return if res == -(2 as c_int) {
        res
    } else {
        -res
    };
} }
static mut xmlXPathErrorMessages: [*const c_char; 28] = [
    b"Ok\n\0" as *const u8 as *const c_char,
    b"Number encoding\n\0" as *const u8 as *const c_char,
    b"Unfinished literal\n\0" as *const u8 as *const c_char,
    b"Start of literal\n\0" as *const u8 as *const c_char,
    b"Expected $ for variable reference\n\0" as *const u8 as *const c_char,
    b"Undefined variable\n\0" as *const u8 as *const c_char,
    b"Invalid predicate\n\0" as *const u8 as *const c_char,
    b"Invalid expression\n\0" as *const u8 as *const c_char,
    b"Missing closing curly brace\n\0" as *const u8 as *const c_char,
    b"Unregistered function\n\0" as *const u8 as *const c_char,
    b"Invalid operand\n\0" as *const u8 as *const c_char,
    b"Invalid type\n\0" as *const u8 as *const c_char,
    b"Invalid number of arguments\n\0" as *const u8 as *const c_char,
    b"Invalid context size\n\0" as *const u8 as *const c_char,
    b"Invalid context position\n\0" as *const u8 as *const c_char,
    b"Memory allocation error\n\0" as *const u8 as *const c_char,
    b"Syntax error\n\0" as *const u8 as *const c_char,
    b"Resource error\n\0" as *const u8 as *const c_char,
    b"Sub resource error\n\0" as *const u8 as *const c_char,
    b"Undefined namespace prefix\n\0" as *const u8 as *const c_char,
    b"Encoding error\n\0" as *const u8 as *const c_char,
    b"Char out of XML range\n\0" as *const u8 as *const c_char,
    b"Invalid or incomplete context\n\0" as *const u8 as *const c_char,
    b"Stack usage error\n\0" as *const u8 as *const c_char,
    b"Forbidden variable\n\0" as *const u8 as *const c_char,
    b"Operation limit exceeded\n\0" as *const u8 as *const c_char,
    b"Recursion limit exceeded\n\0" as *const u8 as *const c_char,
    b"?? Unknown error ??\n\0" as *const u8 as *const c_char,
];
pub const MAXERRNO: c_int = (::core::mem::size_of::<[*const c_char; 28]>()
    as usize)
    .wrapping_div(::core::mem::size_of::<*const c_char>() as usize)
    as c_int
    - 1 as c_int;
unsafe fn xmlXPathErrMemory(
    mut ctxt: xmlXPathContextPtr,
    mut extra: *const c_char,
) {
    if !ctxt.is_null() {
        xmlResetError(&raw mut (*ctxt).lastError);
        if !extra.is_null() {
            let mut buf: [xmlChar; 200] = [0; 200];
            xmlStrPrintf(
                &raw mut buf as *mut xmlChar,
                200 as c_int,
                b"Memory allocation failed : %s\n\0" as *const u8 as *const c_char,
                extra,
            );
            (*ctxt).lastError.message =
                xmlStrdup(&raw mut buf as *mut xmlChar) as *mut c_char;
        } else {
            (*ctxt).lastError.message = xmlStrdup(
                b"Memory allocation failed\n\0" as *const u8 as *const c_char
                    as *mut xmlChar,
            ) as *mut c_char;
        }
        (*ctxt).lastError.domain = XML_FROM_XPATH as c_int;
        (*ctxt).lastError.code = XML_ERR_NO_MEMORY as c_int;
        if (*ctxt).error.is_some() {
            (*ctxt).error.expect("non-null function pointer")(
                (*ctxt).userData,
                &raw mut (*ctxt).lastError,
            );
        }
    } else if !extra.is_null() {
        __xmlRaiseError(
            None,
            None,
            NULL,
            NULL,
            NULL,
            XML_FROM_XPATH as c_int,
            XML_ERR_NO_MEMORY as c_int,
            XML_ERR_FATAL,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            extra,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            b"Memory allocation failed : %s\n\0" as *const u8 as *const c_char,
            extra,
        );
    } else {
        __xmlRaiseError(
            None,
            None,
            NULL,
            NULL,
            NULL,
            XML_FROM_XPATH as c_int,
            XML_ERR_NO_MEMORY as c_int,
            XML_ERR_FATAL,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            b"Memory allocation failed\n\0" as *const u8 as *const c_char,
        );
    };
}
unsafe fn xmlXPathPErrMemory(
    mut ctxt: xmlXPathParserContextPtr,
    mut extra: *const c_char,
) {
    if ctxt.is_null() {
        xmlXPathErrMemory(::core::ptr::null_mut::<xmlXPathContext>(), extra);
    } else {
        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
        xmlXPathErrMemory((*ctxt).context, extra);
    };
}
#[no_mangle]
pub extern "C" fn xmlXPathErr(
    mut ctxt: xmlXPathParserContextPtr,
    mut error: c_int,
) { unsafe {
    if error < 0 as c_int || error > MAXERRNO {
        error = MAXERRNO;
    }
    if ctxt.is_null() {
        __xmlRaiseError(
            None,
            None,
            NULL,
            NULL,
            NULL,
            XML_FROM_XPATH as c_int,
            error + XML_XPATH_EXPRESSION_OK as c_int
                - XPATH_EXPRESSION_OK as c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            b"%s\0" as *const u8 as *const c_char,
            xmlXPathErrorMessages[error as usize],
        );
        return;
    }
    if (*ctxt).error != 0 as c_int {
        return;
    }
    (*ctxt).error = error;
    if (*ctxt).context.is_null() {
        __xmlRaiseError(
            None,
            None,
            NULL,
            NULL,
            NULL,
            XML_FROM_XPATH as c_int,
            error + XML_XPATH_EXPRESSION_OK as c_int
                - XPATH_EXPRESSION_OK as c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            (*ctxt).base as *const c_char,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            (*ctxt).cur.offset_from((*ctxt).base) as c_long as c_int,
            0 as c_int,
            b"%s\0" as *const u8 as *const c_char,
            xmlXPathErrorMessages[error as usize],
        );
        return;
    }
    xmlResetError(&raw mut (*(*ctxt).context).lastError);
    (*(*ctxt).context).lastError.domain = XML_FROM_XPATH as c_int;
    (*(*ctxt).context).lastError.code = error + XML_XPATH_EXPRESSION_OK as c_int
        - XPATH_EXPRESSION_OK as c_int;
    (*(*ctxt).context).lastError.level = XML_ERR_ERROR;
    (*(*ctxt).context).lastError.str1 = xmlStrdup((*ctxt).base) as *mut c_char;
    (*(*ctxt).context).lastError.int1 =
        (*ctxt).cur.offset_from((*ctxt).base) as c_long as c_int;
    (*(*ctxt).context).lastError.node = (*(*ctxt).context).debugNode as *mut c_void;
    if (*(*ctxt).context).error.is_some() {
        (*(*ctxt).context).error.expect("non-null function pointer")(
            (*(*ctxt).context).userData,
            &raw mut (*(*ctxt).context).lastError,
        );
    } else {
        __xmlRaiseError(
            None,
            None,
            NULL,
            NULL,
            (*(*ctxt).context).debugNode as *mut c_void,
            XML_FROM_XPATH as c_int,
            error + XML_XPATH_EXPRESSION_OK as c_int
                - XPATH_EXPRESSION_OK as c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            (*ctxt).base as *const c_char,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            (*ctxt).cur.offset_from((*ctxt).base) as c_long as c_int,
            0 as c_int,
            b"%s\0" as *const u8 as *const c_char,
            xmlXPathErrorMessages[error as usize],
        );
    };
} }
#[inline]
pub unsafe fn xmlXPatherror(
    mut ctxt: xmlXPathParserContextPtr,
    mut file: *const c_char,
    mut line: c_int,
    mut no: c_int,
) {
    xmlXPathErr(ctxt, no);
}
fn xmlXPathCheckOpLimit(
    mut ctxt: xmlXPathParserContextPtr,
    mut opCount: c_ulong,
) -> c_int { unsafe {
    let mut xpctxt: xmlXPathContextPtr = (*ctxt).context;
    if opCount > (*xpctxt).opLimit || (*xpctxt).opCount > (*xpctxt).opLimit.wrapping_sub(opCount) {
        (*xpctxt).opCount = (*xpctxt).opLimit;
        xmlXPathErr(ctxt, XPATH_OP_LIMIT_EXCEEDED as c_int);
        return -(1 as c_int);
    }
    (*xpctxt).opCount = (*xpctxt).opCount.wrapping_add(opCount);
    return 0 as c_int;
} }
unsafe fn xmlPointerListAddSize(
    mut list: xmlPointerListPtr,
    mut item: *mut c_void,
    mut initialSize: c_int,
) -> c_int {
    if (*list).size <= (*list).number {
        let mut tmp: *mut *mut c_void =
            ::core::ptr::null_mut::<*mut c_void>();
        let mut newSize: size_t = 0;
        if (*list).size == 0 as c_int {
            if initialSize <= 0 as c_int {
                initialSize = 1 as c_int;
            }
            newSize = initialSize as size_t;
        } else {
            if (*list).size > 50000000 as c_int {
                xmlXPathErrMemory(
                    ::core::ptr::null_mut::<xmlXPathContext>(),
                    b"xmlPointerListAddSize: re-allocating item\n\0" as *const u8
                        as *const c_char,
                );
                return -(1 as c_int);
            }
            newSize = ((*list).size * 2 as c_int) as size_t;
        }
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*list).items as *mut c_void,
            newSize.wrapping_mul(::core::mem::size_of::<*mut c_void>() as size_t),
        ) as *mut *mut c_void;
        if tmp.is_null() {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"xmlPointerListAddSize: re-allocating item\n\0" as *const u8
                    as *const c_char,
            );
            return -(1 as c_int);
        }
        (*list).items = tmp;
        (*list).size = newSize as c_int;
    }
    let fresh34 = (*list).number;
    (*list).number = (*list).number + 1;
    let ref mut fresh35 = *(*list).items.offset(fresh34 as isize);
    *fresh35 = item;
    return 0 as c_int;
}
fn xmlPointerListCreate(
    mut initialSize: c_int,
) -> xmlPointerListPtr { unsafe {
    let mut ret: xmlPointerListPtr = ::core::ptr::null_mut::<xmlPointerList>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlPointerList>() as size_t
    ) as xmlPointerListPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"xmlPointerListCreate: allocating item\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlPointerList>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlPointerList>() as size_t,
    );
    if initialSize > 0 as c_int {
        xmlPointerListAddSize(ret, NULL, initialSize);
        (*ret).number = 0 as c_int;
    }
    return ret;
} }
fn xmlPointerListFree(mut list: xmlPointerListPtr) { unsafe {
    if list.is_null() {
        return;
    }
    if !(*list).items.is_null() {
        xmlFree.expect("non-null function pointer")((*list).items as *mut c_void);
    }
    xmlFree.expect("non-null function pointer")(list as *mut c_void);
} }
fn xmlXPathNewCompExpr() -> xmlXPathCompExprPtr { unsafe {
    let mut cur: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    cur = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathCompExpr>() as size_t
    ) as xmlXPathCompExprPtr;
    if cur.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"allocating component\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathCompExpr>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathCompExpr>() as size_t,
    );
    (*cur).maxStep = 10 as c_int;
    (*cur).nbStep = 0 as c_int;
    (*cur).steps = xmlMalloc.expect("non-null function pointer")(
        ((*cur).maxStep as size_t).wrapping_mul(::core::mem::size_of::<xmlXPathStepOp>() as size_t),
    ) as *mut xmlXPathStepOp;
    if (*cur).steps.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"allocating steps\n\0" as *const u8 as *const c_char,
        );
        xmlFree.expect("non-null function pointer")(cur as *mut c_void);
        return ::core::ptr::null_mut::<xmlXPathCompExpr>();
    }
    memset(
        (*cur).steps as *mut c_void,
        0 as c_int,
        ((*cur).maxStep as size_t).wrapping_mul(::core::mem::size_of::<xmlXPathStepOp>() as size_t),
    );
    (*cur).last = -(1 as c_int);
    return cur;
} }
#[inline]
pub fn xmlXPathFreeCompExpr(mut comp: xmlXPathCompExprPtr) { unsafe {
    let mut op: xmlXPathStepOpPtr = ::core::ptr::null_mut::<xmlXPathStepOp>();
    let mut i: c_int = 0;
    if comp.is_null() {
        return;
    }
    if (*comp).dict.is_null() {
        i = 0 as c_int;
        while i < (*comp).nbStep {
            op = (*comp).steps.offset(i as isize) as *mut xmlXPathStepOp as xmlXPathStepOpPtr;
            if !(*op).value4.is_null() {
                if (*op).op as c_uint
                    == XPATH_OP_VALUE as c_int as c_uint
                {
                    xmlXPathFreeObject((*op).value4 as xmlXPathObjectPtr);
                } else {
                    xmlFree.expect("non-null function pointer")((*op).value4);
                }
            }
            if !(*op).value5.is_null() {
                xmlFree.expect("non-null function pointer")((*op).value5);
            }
            i += 1;
        }
    } else {
        i = 0 as c_int;
        while i < (*comp).nbStep {
            op = (*comp).steps.offset(i as isize) as *mut xmlXPathStepOp as xmlXPathStepOpPtr;
            if !(*op).value4.is_null() {
                if (*op).op as c_uint
                    == XPATH_OP_VALUE as c_int as c_uint
                {
                    xmlXPathFreeObject((*op).value4 as xmlXPathObjectPtr);
                }
            }
            i += 1;
        }
        xmlDictFree((*comp).dict);
    }
    if !(*comp).steps.is_null() {
        xmlFree.expect("non-null function pointer")((*comp).steps as *mut c_void);
    }
    if !(*comp).stream.is_null() {
        xmlFreePatternList((*comp).stream);
    }
    if !(*comp).expr.is_null() {
        xmlFree.expect("non-null function pointer")((*comp).expr as *mut c_void);
    }
    xmlFree.expect("non-null function pointer")(comp as *mut c_void);
} }
unsafe fn xmlXPathCompExprAdd(
    mut ctxt: xmlXPathParserContextPtr,
    mut ch1: c_int,
    mut ch2: c_int,
    mut op: xmlXPathOp,
    mut value: c_int,
    mut value2: c_int,
    mut value3: c_int,
    mut value4: *mut c_void,
    mut value5: *mut c_void,
) -> c_int {
    let mut comp: xmlXPathCompExprPtr = (*ctxt).comp;
    if (*comp).nbStep >= (*comp).maxStep {
        let mut real: *mut xmlXPathStepOp = ::core::ptr::null_mut::<xmlXPathStepOp>();
        if (*comp).maxStep >= XPATH_MAX_STEPS {
            xmlXPathPErrMemory(
                ctxt,
                b"adding step\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*comp).maxStep *= 2 as c_int;
        real = xmlRealloc.expect("non-null function pointer")(
            (*comp).steps as *mut c_void,
            ((*comp).maxStep as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlXPathStepOp>() as size_t),
        ) as *mut xmlXPathStepOp;
        if real.is_null() {
            (*comp).maxStep /= 2 as c_int;
            xmlXPathPErrMemory(
                ctxt,
                b"adding step\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*comp).steps = real;
    }
    (*comp).last = (*comp).nbStep;
    (*(*comp).steps.offset((*comp).nbStep as isize)).ch1 = ch1;
    (*(*comp).steps.offset((*comp).nbStep as isize)).ch2 = ch2;
    (*(*comp).steps.offset((*comp).nbStep as isize)).op = op;
    (*(*comp).steps.offset((*comp).nbStep as isize)).value = value;
    (*(*comp).steps.offset((*comp).nbStep as isize)).value2 = value2;
    (*(*comp).steps.offset((*comp).nbStep as isize)).value3 = value3;
    if !(*comp).dict.is_null()
        && (op as c_uint
            == XPATH_OP_FUNCTION as c_int as c_uint
            || op as c_uint
                == XPATH_OP_VARIABLE as c_int as c_uint
            || op as c_uint
                == XPATH_OP_COLLECT as c_int as c_uint)
    {
        if !value4.is_null() {
            let ref mut fresh66 = (*(*comp).steps.offset((*comp).nbStep as isize)).value4;
            *fresh66 = xmlDictLookup(
                (*comp).dict,
                value4 as *const xmlChar,
                -(1 as c_int),
            ) as *mut c_void as *mut xmlChar
                as *mut c_void;
            xmlFree.expect("non-null function pointer")(value4);
        } else {
            let ref mut fresh67 = (*(*comp).steps.offset((*comp).nbStep as isize)).value4;
            *fresh67 = NULL;
        }
        if !value5.is_null() {
            let ref mut fresh68 = (*(*comp).steps.offset((*comp).nbStep as isize)).value5;
            *fresh68 = xmlDictLookup(
                (*comp).dict,
                value5 as *const xmlChar,
                -(1 as c_int),
            ) as *mut c_void as *mut xmlChar
                as *mut c_void;
            xmlFree.expect("non-null function pointer")(value5);
        } else {
            let ref mut fresh69 = (*(*comp).steps.offset((*comp).nbStep as isize)).value5;
            *fresh69 = NULL;
        }
    } else {
        let ref mut fresh70 = (*(*comp).steps.offset((*comp).nbStep as isize)).value4;
        *fresh70 = value4;
        let ref mut fresh71 = (*(*comp).steps.offset((*comp).nbStep as isize)).value5;
        *fresh71 = value5;
    }
    let ref mut fresh72 = (*(*comp).steps.offset((*comp).nbStep as isize)).cache;
    *fresh72 = None;
    let fresh73 = (*comp).nbStep;
    (*comp).nbStep = (*comp).nbStep + 1;
    return fresh73;
}
fn xmlXPathCompSwap(mut op: xmlXPathStepOpPtr) { unsafe {
    let mut tmp: c_int = 0;
    tmp = (*op).ch1;
    (*op).ch1 = (*op).ch2;
    (*op).ch2 = tmp;
} }
unsafe fn xmlXPathDebugDumpNode(
    mut output: *mut FILE,
    mut cur: xmlNodePtr,
    mut depth: c_int,
) {
    let mut i: c_int = 0;
    let mut shift: [c_char; 100] = [0; 100];
    i = 0 as c_int;
    while i < depth && i < 25 as c_int {
        shift[(2 as c_int * i + 1 as c_int) as usize] =
            ' ' as i32 as c_char;
        shift[(2 as c_int * i) as usize] =
            shift[(2 as c_int * i + 1 as c_int) as usize];
        i += 1;
    }
    shift[(2 as c_int * i + 1 as c_int) as usize] =
        0 as c_char;
    shift[(2 as c_int * i) as usize] =
        shift[(2 as c_int * i + 1 as c_int) as usize];
    if cur.is_null() {
        fprintf(
            output,
            b"%s\0" as *const u8 as *const c_char,
            &raw mut shift as *mut c_char,
        );
        fprintf(
            output,
            b"Node is NULL !\n\0" as *const u8 as *const c_char,
        );
        return;
    }
    if (*cur).type_0 as c_uint
        == XML_DOCUMENT_NODE as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        fprintf(
            output,
            b"%s\0" as *const u8 as *const c_char,
            &raw mut shift as *mut c_char,
        );
        fprintf(output, b" /\n\0" as *const u8 as *const c_char);
    } else if (*cur).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        xmlDebugDumpAttr(output, cur as xmlAttrPtr, depth);
    } else {
        xmlDebugDumpOneNode(output, cur, depth);
    };
}
unsafe fn xmlXPathDebugDumpNodeList(
    mut output: *mut FILE,
    mut cur: xmlNodePtr,
    mut depth: c_int,
) {
    let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut i: c_int = 0;
    let mut shift: [c_char; 100] = [0; 100];
    i = 0 as c_int;
    while i < depth && i < 25 as c_int {
        shift[(2 as c_int * i + 1 as c_int) as usize] =
            ' ' as i32 as c_char;
        shift[(2 as c_int * i) as usize] =
            shift[(2 as c_int * i + 1 as c_int) as usize];
        i += 1;
    }
    shift[(2 as c_int * i + 1 as c_int) as usize] =
        0 as c_char;
    shift[(2 as c_int * i) as usize] =
        shift[(2 as c_int * i + 1 as c_int) as usize];
    if cur.is_null() {
        fprintf(
            output,
            b"%s\0" as *const u8 as *const c_char,
            &raw mut shift as *mut c_char,
        );
        fprintf(
            output,
            b"Node is NULL !\n\0" as *const u8 as *const c_char,
        );
        return;
    }
    while !cur.is_null() {
        tmp = cur;
        cur = (*cur).next as xmlNodePtr;
        xmlDebugDumpOneNode(output, tmp, depth);
    }
}
unsafe fn xmlXPathDebugDumpNodeSet(
    mut output: *mut FILE,
    mut cur: xmlNodeSetPtr,
    mut depth: c_int,
) {
    let mut i: c_int = 0;
    let mut shift: [c_char; 100] = [0; 100];
    i = 0 as c_int;
    while i < depth && i < 25 as c_int {
        shift[(2 as c_int * i + 1 as c_int) as usize] =
            ' ' as i32 as c_char;
        shift[(2 as c_int * i) as usize] =
            shift[(2 as c_int * i + 1 as c_int) as usize];
        i += 1;
    }
    shift[(2 as c_int * i + 1 as c_int) as usize] =
        0 as c_char;
    shift[(2 as c_int * i) as usize] =
        shift[(2 as c_int * i + 1 as c_int) as usize];
    if cur.is_null() {
        fprintf(
            output,
            b"%s\0" as *const u8 as *const c_char,
            &raw mut shift as *mut c_char,
        );
        fprintf(
            output,
            b"NodeSet is NULL !\n\0" as *const u8 as *const c_char,
        );
        return;
    }
    if !cur.is_null() {
        fprintf(
            output,
            b"Set contains %d nodes:\n\0" as *const u8 as *const c_char,
            (*cur).nodeNr,
        );
        i = 0 as c_int;
        while i < (*cur).nodeNr {
            fprintf(
                output,
                b"%s\0" as *const u8 as *const c_char,
                &raw mut shift as *mut c_char,
            );
            fprintf(
                output,
                b"%d\0" as *const u8 as *const c_char,
                i + 1 as c_int,
            );
            xmlXPathDebugDumpNode(
                output,
                *(*cur).nodeTab.offset(i as isize),
                depth + 1 as c_int,
            );
            i += 1;
        }
    }
}
unsafe fn xmlXPathDebugDumpValueTree(
    mut output: *mut FILE,
    mut cur: xmlNodeSetPtr,
    mut depth: c_int,
) {
    let mut i: c_int = 0;
    let mut shift: [c_char; 100] = [0; 100];
    i = 0 as c_int;
    while i < depth && i < 25 as c_int {
        shift[(2 as c_int * i + 1 as c_int) as usize] =
            ' ' as i32 as c_char;
        shift[(2 as c_int * i) as usize] =
            shift[(2 as c_int * i + 1 as c_int) as usize];
        i += 1;
    }
    shift[(2 as c_int * i + 1 as c_int) as usize] =
        0 as c_char;
    shift[(2 as c_int * i) as usize] =
        shift[(2 as c_int * i + 1 as c_int) as usize];
    if cur.is_null()
        || (*cur).nodeNr == 0 as c_int
        || (*(*cur).nodeTab.offset(0 as c_int as isize)).is_null()
    {
        fprintf(
            output,
            b"%s\0" as *const u8 as *const c_char,
            &raw mut shift as *mut c_char,
        );
        fprintf(
            output,
            b"Value Tree is NULL !\n\0" as *const u8 as *const c_char,
        );
        return;
    }
    fprintf(
        output,
        b"%s\0" as *const u8 as *const c_char,
        &raw mut shift as *mut c_char,
    );
    fprintf(
        output,
        b"%d\0" as *const u8 as *const c_char,
        i + 1 as c_int,
    );
    xmlXPathDebugDumpNodeList(
        output,
        (**(*cur).nodeTab.offset(0 as c_int as isize)).children as xmlNodePtr,
        depth + 1 as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathDebugDumpObject(
    mut output: *mut FILE,
    mut cur: xmlXPathObjectPtr,
    mut depth: c_int,
) {
    let mut i: c_int = 0;
    let mut shift: [c_char; 100] = [0; 100];
    if output.is_null() {
        return;
    }
    i = 0 as c_int;
    while i < depth && i < 25 as c_int {
        shift[(2 as c_int * i + 1 as c_int) as usize] =
            ' ' as i32 as c_char;
        shift[(2 as c_int * i) as usize] =
            shift[(2 as c_int * i + 1 as c_int) as usize];
        i += 1;
    }
    shift[(2 as c_int * i + 1 as c_int) as usize] =
        0 as c_char;
    shift[(2 as c_int * i) as usize] =
        shift[(2 as c_int * i + 1 as c_int) as usize];
    fprintf(
        output,
        b"%s\0" as *const u8 as *const c_char,
        &raw mut shift as *mut c_char,
    );
    if cur.is_null() {
        fprintf(
            output,
            b"Object is empty (NULL)\n\0" as *const u8 as *const c_char,
        );
        return;
    }
    match (*cur).type_0 as c_uint {
        0 => {
            fprintf(
                output,
                b"Object is uninitialized\n\0" as *const u8 as *const c_char,
            );
        }
        1 => {
            fprintf(
                output,
                b"Object is a Node Set :\n\0" as *const u8 as *const c_char,
            );
            xmlXPathDebugDumpNodeSet(output, (*cur).nodesetval, depth);
        }
        9 => {
            fprintf(
                output,
                b"Object is an XSLT value tree :\n\0" as *const u8 as *const c_char,
            );
            xmlXPathDebugDumpValueTree(output, (*cur).nodesetval, depth);
        }
        2 => {
            fprintf(
                output,
                b"Object is a Boolean : \0" as *const u8 as *const c_char,
            );
            if (*cur).boolval != 0 {
                fprintf(
                    output,
                    b"true\n\0" as *const u8 as *const c_char,
                );
            } else {
                fprintf(
                    output,
                    b"false\n\0" as *const u8 as *const c_char,
                );
            }
        }
        3 => match xmlXPathIsInf((*cur).floatval) {
            1 => {
                fprintf(
                    output,
                    b"Object is a number : Infinity\n\0" as *const u8 as *const c_char,
                );
            }
            -1 => {
                fprintf(
                    output,
                    b"Object is a number : -Infinity\n\0" as *const u8
                        as *const c_char,
                );
            }
            _ => {
                if xmlXPathIsNaN((*cur).floatval) != 0 {
                    fprintf(
                        output,
                        b"Object is a number : NaN\n\0" as *const u8 as *const c_char,
                    );
                } else if (*cur).floatval == 0 as c_int as c_double {
                    fprintf(
                        output,
                        b"Object is a number : 0\n\0" as *const u8 as *const c_char,
                    );
                } else {
                    fprintf(
                        output,
                        b"Object is a number : %0g\n\0" as *const u8 as *const c_char,
                        (*cur).floatval,
                    );
                }
            }
        },
        4 => {
            fprintf(
                output,
                b"Object is a string : \0" as *const u8 as *const c_char,
            );
            xmlDebugDumpString(output, (*cur).stringval);
            fprintf(output, b"\n\0" as *const u8 as *const c_char);
        }
        8 => {
            fprintf(
                output,
                b"Object is user defined\n\0" as *const u8 as *const c_char,
            );
        }
        _ => {}
    };
}
unsafe fn xmlXPathDebugDumpStepOp(
    mut output: *mut FILE,
    mut comp: xmlXPathCompExprPtr,
    mut op: xmlXPathStepOpPtr,
    mut depth: c_int,
) {
    let mut current_block: u64;
    let mut i: c_int = 0;
    let mut shift: [c_char; 100] = [0; 100];
    i = 0 as c_int;
    while i < depth && i < 25 as c_int {
        shift[(2 as c_int * i + 1 as c_int) as usize] =
            ' ' as i32 as c_char;
        shift[(2 as c_int * i) as usize] =
            shift[(2 as c_int * i + 1 as c_int) as usize];
        i += 1;
    }
    shift[(2 as c_int * i + 1 as c_int) as usize] =
        0 as c_char;
    shift[(2 as c_int * i) as usize] =
        shift[(2 as c_int * i + 1 as c_int) as usize];
    fprintf(
        output,
        b"%s\0" as *const u8 as *const c_char,
        &raw mut shift as *mut c_char,
    );
    if op.is_null() {
        fprintf(
            output,
            b"Step is NULL\n\0" as *const u8 as *const c_char,
        );
        return;
    }
    match (*op).op as c_uint {
        0 => {
            fprintf(output, b"END\0" as *const u8 as *const c_char);
            current_block = 6040267449472925966;
        }
        1 => {
            fprintf(output, b"AND\0" as *const u8 as *const c_char);
            current_block = 6040267449472925966;
        }
        2 => {
            fprintf(output, b"OR\0" as *const u8 as *const c_char);
            current_block = 6040267449472925966;
        }
        3 => {
            if (*op).value != 0 {
                fprintf(
                    output,
                    b"EQUAL =\0" as *const u8 as *const c_char,
                );
            } else {
                fprintf(
                    output,
                    b"EQUAL !=\0" as *const u8 as *const c_char,
                );
            }
            current_block = 6040267449472925966;
        }
        4 => {
            if (*op).value != 0 {
                fprintf(
                    output,
                    b"CMP <\0" as *const u8 as *const c_char,
                );
            } else {
                fprintf(
                    output,
                    b"CMP >\0" as *const u8 as *const c_char,
                );
            }
            if (*op).value2 == 0 {
                fprintf(output, b"=\0" as *const u8 as *const c_char);
            }
            current_block = 6040267449472925966;
        }
        5 => {
            if (*op).value == 0 as c_int {
                fprintf(
                    output,
                    b"PLUS -\0" as *const u8 as *const c_char,
                );
            } else if (*op).value == 1 as c_int {
                fprintf(
                    output,
                    b"PLUS +\0" as *const u8 as *const c_char,
                );
            } else if (*op).value == 2 as c_int {
                fprintf(
                    output,
                    b"PLUS unary -\0" as *const u8 as *const c_char,
                );
            } else if (*op).value == 3 as c_int {
                fprintf(
                    output,
                    b"PLUS unary - -\0" as *const u8 as *const c_char,
                );
            }
            current_block = 6040267449472925966;
        }
        6 => {
            if (*op).value == 0 as c_int {
                fprintf(
                    output,
                    b"MULT *\0" as *const u8 as *const c_char,
                );
            } else if (*op).value == 1 as c_int {
                fprintf(
                    output,
                    b"MULT div\0" as *const u8 as *const c_char,
                );
            } else {
                fprintf(
                    output,
                    b"MULT mod\0" as *const u8 as *const c_char,
                );
            }
            current_block = 6040267449472925966;
        }
        7 => {
            fprintf(
                output,
                b"UNION\0" as *const u8 as *const c_char,
            );
            current_block = 6040267449472925966;
        }
        8 => {
            fprintf(output, b"ROOT\0" as *const u8 as *const c_char);
            current_block = 6040267449472925966;
        }
        9 => {
            fprintf(output, b"NODE\0" as *const u8 as *const c_char);
            current_block = 6040267449472925966;
        }
        17 => {
            fprintf(output, b"SORT\0" as *const u8 as *const c_char);
            current_block = 6040267449472925966;
        }
        10 => {
            let mut axis: xmlXPathAxisVal = (*op).value as xmlXPathAxisVal;
            let mut test: xmlXPathTestVal = (*op).value2 as xmlXPathTestVal;
            let mut type_0: xmlXPathTypeVal = (*op).value3 as xmlXPathTypeVal;
            let mut prefix: *const xmlChar = (*op).value4 as *const xmlChar;
            let mut name: *const xmlChar = (*op).value5 as *const xmlChar;
            fprintf(
                output,
                b"COLLECT \0" as *const u8 as *const c_char,
            );
            match axis as c_uint {
                1 => {
                    fprintf(
                        output,
                        b" 'ancestors' \0" as *const u8 as *const c_char,
                    );
                }
                2 => {
                    fprintf(
                        output,
                        b" 'ancestors-or-self' \0" as *const u8 as *const c_char,
                    );
                }
                3 => {
                    fprintf(
                        output,
                        b" 'attributes' \0" as *const u8 as *const c_char,
                    );
                }
                4 => {
                    fprintf(
                        output,
                        b" 'child' \0" as *const u8 as *const c_char,
                    );
                }
                5 => {
                    fprintf(
                        output,
                        b" 'descendant' \0" as *const u8 as *const c_char,
                    );
                }
                6 => {
                    fprintf(
                        output,
                        b" 'descendant-or-self' \0" as *const u8 as *const c_char,
                    );
                }
                7 => {
                    fprintf(
                        output,
                        b" 'following' \0" as *const u8 as *const c_char,
                    );
                }
                8 => {
                    fprintf(
                        output,
                        b" 'following-siblings' \0" as *const u8 as *const c_char,
                    );
                }
                9 => {
                    fprintf(
                        output,
                        b" 'namespace' \0" as *const u8 as *const c_char,
                    );
                }
                10 => {
                    fprintf(
                        output,
                        b" 'parent' \0" as *const u8 as *const c_char,
                    );
                }
                11 => {
                    fprintf(
                        output,
                        b" 'preceding' \0" as *const u8 as *const c_char,
                    );
                }
                12 => {
                    fprintf(
                        output,
                        b" 'preceding-sibling' \0" as *const u8 as *const c_char,
                    );
                }
                13 => {
                    fprintf(
                        output,
                        b" 'self' \0" as *const u8 as *const c_char,
                    );
                }
                _ => {}
            }
            match test as c_uint {
                0 => {
                    fprintf(
                        output,
                        b"'none' \0" as *const u8 as *const c_char,
                    );
                }
                1 => {
                    fprintf(
                        output,
                        b"'type' \0" as *const u8 as *const c_char,
                    );
                }
                2 => {
                    fprintf(
                        output,
                        b"'PI' \0" as *const u8 as *const c_char,
                    );
                }
                3 => {
                    fprintf(
                        output,
                        b"'all' \0" as *const u8 as *const c_char,
                    );
                }
                4 => {
                    fprintf(
                        output,
                        b"'namespace' \0" as *const u8 as *const c_char,
                    );
                }
                5 => {
                    fprintf(
                        output,
                        b"'name' \0" as *const u8 as *const c_char,
                    );
                }
                _ => {}
            }
            match type_0 as c_uint {
                0 => {
                    fprintf(
                        output,
                        b"'node' \0" as *const u8 as *const c_char,
                    );
                }
                8 => {
                    fprintf(
                        output,
                        b"'comment' \0" as *const u8 as *const c_char,
                    );
                }
                3 => {
                    fprintf(
                        output,
                        b"'text' \0" as *const u8 as *const c_char,
                    );
                }
                7 => {
                    fprintf(
                        output,
                        b"'PI' \0" as *const u8 as *const c_char,
                    );
                }
                _ => {}
            }
            if !prefix.is_null() {
                fprintf(
                    output,
                    b"%s:\0" as *const u8 as *const c_char,
                    prefix,
                );
            }
            if !name.is_null() {
                fprintf(
                    output,
                    b"%s\0" as *const u8 as *const c_char,
                    name as *const c_char,
                );
            }
            current_block = 6040267449472925966;
        }
        11 => {
            let mut object: xmlXPathObjectPtr = (*op).value4 as xmlXPathObjectPtr;
            fprintf(
                output,
                b"ELEM \0" as *const u8 as *const c_char,
            );
            xmlXPathDebugDumpObject(output, object, 0 as c_int);
            current_block = 5221243979320478134;
        }
        12 => {
            let mut prefix_0: *const xmlChar = (*op).value5 as *const xmlChar;
            let mut name_0: *const xmlChar = (*op).value4 as *const xmlChar;
            if !prefix_0.is_null() {
                fprintf(
                    output,
                    b"VARIABLE %s:%s\0" as *const u8 as *const c_char,
                    prefix_0,
                    name_0,
                );
            } else {
                fprintf(
                    output,
                    b"VARIABLE %s\0" as *const u8 as *const c_char,
                    name_0,
                );
            }
            current_block = 6040267449472925966;
        }
        13 => {
            let mut nbargs: c_int = (*op).value;
            let mut prefix_1: *const xmlChar = (*op).value5 as *const xmlChar;
            let mut name_1: *const xmlChar = (*op).value4 as *const xmlChar;
            if !prefix_1.is_null() {
                fprintf(
                    output,
                    b"FUNCTION %s:%s(%d args)\0" as *const u8 as *const c_char,
                    prefix_1,
                    name_1,
                    nbargs,
                );
            } else {
                fprintf(
                    output,
                    b"FUNCTION %s(%d args)\0" as *const u8 as *const c_char,
                    name_1,
                    nbargs,
                );
            }
            current_block = 6040267449472925966;
        }
        14 => {
            fprintf(output, b"ARG\0" as *const u8 as *const c_char);
            current_block = 6040267449472925966;
        }
        15 => {
            fprintf(
                output,
                b"PREDICATE\0" as *const u8 as *const c_char,
            );
            current_block = 6040267449472925966;
        }
        16 => {
            fprintf(
                output,
                b"FILTER\0" as *const u8 as *const c_char,
            );
            current_block = 6040267449472925966;
        }
        _ => {
            fprintf(
                output,
                b"UNKNOWN %d\n\0" as *const u8 as *const c_char,
                (*op).op as c_uint,
            );
            return;
        }
    }
    match current_block {
        6040267449472925966 => {
            fprintf(output, b"\n\0" as *const u8 as *const c_char);
        }
        _ => {}
    }
    if (*op).ch1 >= 0 as c_int {
        xmlXPathDebugDumpStepOp(
            output,
            comp,
            (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            depth + 1 as c_int,
        );
    }
    if (*op).ch2 >= 0 as c_int {
        xmlXPathDebugDumpStepOp(
            output,
            comp,
            (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
            depth + 1 as c_int,
        );
    }
}
#[inline]
pub unsafe fn xmlXPathDebugDumpCompExpr(
    mut output: *mut FILE,
    mut comp: xmlXPathCompExprPtr,
    mut depth: c_int,
) {
    let mut i: c_int = 0;
    let mut shift: [c_char; 100] = [0; 100];
    if output.is_null() || comp.is_null() {
        return;
    }
    i = 0 as c_int;
    while i < depth && i < 25 as c_int {
        shift[(2 as c_int * i + 1 as c_int) as usize] =
            ' ' as i32 as c_char;
        shift[(2 as c_int * i) as usize] =
            shift[(2 as c_int * i + 1 as c_int) as usize];
        i += 1;
    }
    shift[(2 as c_int * i + 1 as c_int) as usize] =
        0 as c_char;
    shift[(2 as c_int * i) as usize] =
        shift[(2 as c_int * i + 1 as c_int) as usize];
    fprintf(
        output,
        b"%s\0" as *const u8 as *const c_char,
        &raw mut shift as *mut c_char,
    );
    if !(*comp).stream.is_null() {
        fprintf(
            output,
            b"Streaming Expression\n\0" as *const u8 as *const c_char,
        );
    } else {
        fprintf(
            output,
            b"Compiled Expression : %d elements\n\0" as *const u8 as *const c_char,
            (*comp).nbStep,
        );
        i = (*comp).last;
        xmlXPathDebugDumpStepOp(
            output,
            comp,
            (*comp).steps.offset(i as isize) as xmlXPathStepOpPtr,
            depth + 1 as c_int,
        );
    };
}
fn xmlXPathNewCache() -> xmlXPathContextCachePtr { unsafe {
    let mut ret: xmlXPathContextCachePtr = ::core::ptr::null_mut::<xmlXPathContextCache>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathContextCache>() as size_t,
    ) as xmlXPathContextCachePtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating object cache\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathContextCache>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathContextCache>() as size_t,
    );
    (*ret).maxNodeset = 100 as c_int;
    (*ret).maxString = 100 as c_int;
    (*ret).maxBoolean = 100 as c_int;
    (*ret).maxNumber = 100 as c_int;
    (*ret).maxMisc = 100 as c_int;
    return ret;
} }
fn xmlXPathCacheFreeObjectList(mut list: xmlPointerListPtr) { unsafe {
    let mut i: c_int = 0;
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if list.is_null() {
        return;
    }
    i = 0 as c_int;
    while i < (*list).number {
        obj = *(*list).items.offset(i as isize) as xmlXPathObjectPtr;
        if !(*obj).nodesetval.is_null() {
            if !(*(*obj).nodesetval).nodeTab.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*(*obj).nodesetval).nodeTab as *mut c_void,
                );
            }
            xmlFree.expect("non-null function pointer")(
                (*obj).nodesetval as *mut c_void,
            );
        }
        xmlFree.expect("non-null function pointer")(obj as *mut c_void);
        i += 1;
    }
    xmlPointerListFree(list);
} }
fn xmlXPathFreeCache(mut cache: xmlXPathContextCachePtr) { unsafe {
    if cache.is_null() {
        return;
    }
    if !(*cache).nodesetObjs.is_null() {
        xmlXPathCacheFreeObjectList((*cache).nodesetObjs);
    }
    if !(*cache).stringObjs.is_null() {
        xmlXPathCacheFreeObjectList((*cache).stringObjs);
    }
    if !(*cache).booleanObjs.is_null() {
        xmlXPathCacheFreeObjectList((*cache).booleanObjs);
    }
    if !(*cache).numberObjs.is_null() {
        xmlXPathCacheFreeObjectList((*cache).numberObjs);
    }
    if !(*cache).miscObjs.is_null() {
        xmlXPathCacheFreeObjectList((*cache).miscObjs);
    }
    xmlFree.expect("non-null function pointer")(cache as *mut c_void);
} }
#[inline]
pub fn xmlXPathContextSetCache(
    mut ctxt: xmlXPathContextPtr,
    mut active: c_int,
    mut value: c_int,
    mut options: c_int,
) -> c_int { unsafe {
    if ctxt.is_null() {
        return -(1 as c_int);
    }
    if active != 0 {
        let mut cache: xmlXPathContextCachePtr = ::core::ptr::null_mut::<xmlXPathContextCache>();
        if (*ctxt).cache.is_null() {
            (*ctxt).cache = xmlXPathNewCache() as *mut c_void;
            if (*ctxt).cache.is_null() {
                return -(1 as c_int);
            }
        }
        cache = (*ctxt).cache as xmlXPathContextCachePtr;
        if options == 0 as c_int {
            if value < 0 as c_int {
                value = 100 as c_int;
            }
            (*cache).maxNodeset = value;
            (*cache).maxString = value;
            (*cache).maxNumber = value;
            (*cache).maxBoolean = value;
            (*cache).maxMisc = value;
        }
    } else if !(*ctxt).cache.is_null() {
        xmlXPathFreeCache((*ctxt).cache as xmlXPathContextCachePtr);
        (*ctxt).cache = NULL;
    }
    return 0 as c_int;
} }
fn xmlXPathCacheWrapNodeSet(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlNodeSetPtr,
) -> xmlXPathObjectPtr { unsafe {
    if !ctxt.is_null() && !(*ctxt).cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache as xmlXPathContextCachePtr;
        if !(*cache).miscObjs.is_null() && (*(*cache).miscObjs).number != 0 as c_int {
            let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            (*(*cache).miscObjs).number -= 1;
            ret = *(*(*cache).miscObjs)
                .items
                .offset((*(*cache).miscObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_NODESET;
            (*ret).nodesetval = val;
            return ret;
        }
    }
    return xmlXPathWrapNodeSet(val);
} }
unsafe fn xmlXPathCacheWrapString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *mut xmlChar,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !(*ctxt).cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache as xmlXPathContextCachePtr;
        if !(*cache).stringObjs.is_null()
            && (*(*cache).stringObjs).number != 0 as c_int
        {
            let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            (*(*cache).stringObjs).number -= 1;
            ret = *(*(*cache).stringObjs)
                .items
                .offset((*(*cache).stringObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_STRING;
            (*ret).stringval = val;
            return ret;
        } else if !(*cache).miscObjs.is_null()
            && (*(*cache).miscObjs).number != 0 as c_int
        {
            let mut ret_0: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *(*(*cache).miscObjs)
                .items
                .offset((*(*cache).miscObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_STRING;
            (*ret_0).stringval = val;
            return ret_0;
        }
    }
    return xmlXPathWrapString(val);
}
fn xmlXPathCacheNewNodeSet(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlNodePtr,
) -> xmlXPathObjectPtr { unsafe {
    if !ctxt.is_null() && !(*ctxt).cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache as xmlXPathContextCachePtr;
        if !(*cache).nodesetObjs.is_null()
            && (*(*cache).nodesetObjs).number != 0 as c_int
        {
            let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            (*(*cache).nodesetObjs).number -= 1;
            ret = *(*(*cache).nodesetObjs)
                .items
                .offset((*(*cache).nodesetObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_NODESET;
            (*ret).boolval = 0 as c_int;
            if !val.is_null() {
                if (*(*ret).nodesetval).nodeMax == 0 as c_int
                    || (*val).type_0 as c_uint
                        == XML_NAMESPACE_DECL as c_int as c_uint
                {
                    xmlXPathNodeSetAddUnique((*ret).nodesetval, val);
                } else {
                    let ref mut fresh42 = *(*(*ret).nodesetval)
                        .nodeTab
                        .offset(0 as c_int as isize);
                    *fresh42 = val;
                    (*(*ret).nodesetval).nodeNr = 1 as c_int;
                }
            }
            return ret;
        } else if !(*cache).miscObjs.is_null()
            && (*(*cache).miscObjs).number != 0 as c_int
        {
            let mut ret_0: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            let mut set: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
            set = xmlXPathNodeSetCreate(val);
            if set.is_null() {
                (*ctxt).lastError.domain = XML_FROM_XPATH as c_int;
                (*ctxt).lastError.code = XML_ERR_NO_MEMORY as c_int;
                return ::core::ptr::null_mut::<xmlXPathObject>();
            }
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *(*(*cache).miscObjs)
                .items
                .offset((*(*cache).miscObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_NODESET;
            (*ret_0).boolval = 0 as c_int;
            (*ret_0).nodesetval = set;
            return ret_0;
        }
    }
    return xmlXPathNewNodeSet(val);
} }
unsafe fn xmlXPathCacheNewString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *const xmlChar,
) -> xmlXPathObjectPtr {
    if !ctxt.is_null() && !(*ctxt).cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache as xmlXPathContextCachePtr;
        if !(*cache).stringObjs.is_null()
            && (*(*cache).stringObjs).number != 0 as c_int
        {
            let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            let mut copy: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            if val.is_null() {
                val = b"\0" as *const u8 as *const c_char as *mut xmlChar;
            }
            copy = xmlStrdup(val);
            if copy.is_null() {
                xmlXPathErrMemory(ctxt, ::core::ptr::null::<c_char>());
                return ::core::ptr::null_mut::<xmlXPathObject>();
            }
            (*(*cache).stringObjs).number -= 1;
            ret = *(*(*cache).stringObjs)
                .items
                .offset((*(*cache).stringObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_STRING;
            (*ret).stringval = copy;
            return ret;
        } else if !(*cache).miscObjs.is_null()
            && (*(*cache).miscObjs).number != 0 as c_int
        {
            let mut ret_0: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            let mut copy_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            if val.is_null() {
                val = b"\0" as *const u8 as *const c_char as *mut xmlChar;
            }
            copy_0 = xmlStrdup(val);
            if copy_0.is_null() {
                xmlXPathErrMemory(ctxt, ::core::ptr::null::<c_char>());
                return ::core::ptr::null_mut::<xmlXPathObject>();
            }
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *(*(*cache).miscObjs)
                .items
                .offset((*(*cache).miscObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_STRING;
            (*ret_0).stringval = copy_0;
            return ret_0;
        }
    }
    return xmlXPathNewString(val);
}
unsafe fn xmlXPathCacheNewCString(
    mut ctxt: xmlXPathContextPtr,
    mut val: *const c_char,
) -> xmlXPathObjectPtr {
    return xmlXPathCacheNewString(ctxt, val as *mut xmlChar);
}
fn xmlXPathCacheNewBoolean(
    mut ctxt: xmlXPathContextPtr,
    mut val: c_int,
) -> xmlXPathObjectPtr { unsafe {
    if !ctxt.is_null() && !(*ctxt).cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache as xmlXPathContextCachePtr;
        if !(*cache).booleanObjs.is_null()
            && (*(*cache).booleanObjs).number != 0 as c_int
        {
            let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            (*(*cache).booleanObjs).number -= 1;
            ret = *(*(*cache).booleanObjs)
                .items
                .offset((*(*cache).booleanObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_BOOLEAN;
            (*ret).boolval = (val != 0 as c_int) as c_int;
            return ret;
        } else if !(*cache).miscObjs.is_null()
            && (*(*cache).miscObjs).number != 0 as c_int
        {
            let mut ret_0: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *(*(*cache).miscObjs)
                .items
                .offset((*(*cache).miscObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_BOOLEAN;
            (*ret_0).boolval = (val != 0 as c_int) as c_int;
            return ret_0;
        }
    }
    return xmlXPathNewBoolean(val);
} }
fn xmlXPathCacheNewFloat(
    mut ctxt: xmlXPathContextPtr,
    mut val: c_double,
) -> xmlXPathObjectPtr { unsafe {
    if !ctxt.is_null() && !(*ctxt).cache.is_null() {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache as xmlXPathContextCachePtr;
        if !(*cache).numberObjs.is_null()
            && (*(*cache).numberObjs).number != 0 as c_int
        {
            let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            (*(*cache).numberObjs).number -= 1;
            ret = *(*(*cache).numberObjs)
                .items
                .offset((*(*cache).numberObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret).type_0 = XPATH_NUMBER;
            (*ret).floatval = val;
            return ret;
        } else if !(*cache).miscObjs.is_null()
            && (*(*cache).miscObjs).number != 0 as c_int
        {
            let mut ret_0: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            (*(*cache).miscObjs).number -= 1;
            ret_0 = *(*(*cache).miscObjs)
                .items
                .offset((*(*cache).miscObjs).number as isize)
                as xmlXPathObjectPtr;
            (*ret_0).type_0 = XPATH_NUMBER;
            (*ret_0).floatval = val;
            return ret_0;
        }
    }
    return xmlXPathNewFloat(val);
} }
fn xmlXPathCacheConvertString(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr { unsafe {
    let mut res: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if val.is_null() {
        return xmlXPathCacheNewCString(ctxt, b"\0" as *const u8 as *const c_char);
    }
    match (*val).type_0 as c_uint {
        1 | 9 => {
            res = xmlXPathCastNodeSetToString((*val).nodesetval);
        }
        4 => return val,
        2 => {
            res = xmlXPathCastBooleanToString((*val).boolval);
        }
        3 => {
            res = xmlXPathCastNumberToString((*val).floatval);
        }
        8 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                    as *const c_char,
                2158 as c_int,
            );
        }
        0 | _ => {}
    }
    xmlXPathReleaseObject(ctxt, val);
    if res.is_null() {
        return xmlXPathCacheNewCString(ctxt, b"\0" as *const u8 as *const c_char);
    }
    return xmlXPathCacheWrapString(ctxt, res);
} }
fn xmlXPathCacheObjectCopy(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr { unsafe {
    if val.is_null() {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    if !ctxt.is_null() && !(*ctxt).cache.is_null() {
        match (*val).type_0 as c_uint {
            1 => {
                return xmlXPathCacheWrapNodeSet(
                    ctxt,
                    xmlXPathNodeSetMerge(::core::ptr::null_mut::<xmlNodeSet>(), (*val).nodesetval),
                );
            }
            4 => return xmlXPathCacheNewString(ctxt, (*val).stringval),
            2 => return xmlXPathCacheNewBoolean(ctxt, (*val).boolval),
            3 => return xmlXPathCacheNewFloat(ctxt, (*val).floatval),
            _ => {}
        }
    }
    return xmlXPathObjectCopy(val);
} }
fn xmlXPathCacheConvertBoolean(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if val.is_null() {
        return xmlXPathCacheNewBoolean(ctxt, 0 as c_int);
    }
    if (*val).type_0 as c_uint
        == XPATH_BOOLEAN as c_int as c_uint
    {
        return val;
    }
    ret = xmlXPathCacheNewBoolean(ctxt, xmlXPathCastToBoolean(val));
    xmlXPathReleaseObject(ctxt, val);
    return ret;
} }
fn xmlXPathCacheConvertNumber(
    mut ctxt: xmlXPathContextPtr,
    mut val: xmlXPathObjectPtr,
) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if val.is_null() {
        return xmlXPathCacheNewFloat(ctxt, 0.0f64);
    }
    if (*val).type_0 as c_uint
        == XPATH_NUMBER as c_int as c_uint
    {
        return val;
    }
    ret = xmlXPathCacheNewFloat(ctxt, xmlXPathCastToNumber(val));
    xmlXPathReleaseObject(ctxt, val);
    return ret;
} }
#[no_mangle]
pub extern "C" fn valuePop(mut ctxt: xmlXPathParserContextPtr) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() || (*ctxt).valueNr <= 0 as c_int {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    (*ctxt).valueNr -= 1;
    if (*ctxt).valueNr > 0 as c_int {
        (*ctxt).value = *(*ctxt)
            .valueTab
            .offset(((*ctxt).valueNr - 1 as c_int) as isize);
    } else {
        (*ctxt).value = ::core::ptr::null_mut::<xmlXPathObject>();
    }
    ret = *(*ctxt).valueTab.offset((*ctxt).valueNr as isize);
    let ref mut fresh38 = *(*ctxt).valueTab.offset((*ctxt).valueNr as isize);
    *fresh38 = ::core::ptr::null_mut::<xmlXPathObject>();
    return ret;
} }
#[no_mangle]
pub extern "C" fn valuePush(
    mut ctxt: xmlXPathParserContextPtr,
    mut value: xmlXPathObjectPtr,
) -> c_int { unsafe {
    if ctxt.is_null() {
        return -(1 as c_int);
    }
    if value.is_null() {
        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
        return -(1 as c_int);
    }
    if (*ctxt).valueNr >= (*ctxt).valueMax {
        let mut tmp: *mut xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObjectPtr>();
        if (*ctxt).valueMax >= XPATH_MAX_STACK_DEPTH {
            xmlXPathPErrMemory(
                ctxt,
                b"XPath stack depth limit reached\n\0" as *const u8 as *const c_char,
            );
            xmlXPathFreeObject(value);
            return -(1 as c_int);
        }
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).valueTab as *mut c_void,
            ((2 as c_int * (*ctxt).valueMax) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlXPathObjectPtr>() as size_t),
        ) as *mut xmlXPathObjectPtr;
        if tmp.is_null() {
            xmlXPathPErrMemory(
                ctxt,
                b"pushing value\n\0" as *const u8 as *const c_char,
            );
            xmlXPathFreeObject(value);
            return -(1 as c_int);
        }
        (*ctxt).valueMax *= 2 as c_int;
        (*ctxt).valueTab = tmp;
    }
    let ref mut fresh36 = *(*ctxt).valueTab.offset((*ctxt).valueNr as isize);
    *fresh36 = value;
    (*ctxt).value = value;
    let fresh37 = (*ctxt).valueNr;
    (*ctxt).valueNr = (*ctxt).valueNr + 1;
    return fresh37;
} }
#[inline]
pub fn xmlXPathPopBoolean(
    mut ctxt: xmlXPathParserContextPtr,
) -> c_int { unsafe {
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut ret: c_int = 0;
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            2344 as c_int,
            XPATH_INVALID_OPERAND as c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as c_int;
        }
        return 0 as c_int;
    }
    if (*obj).type_0 as c_uint
        != XPATH_BOOLEAN as c_int as c_uint
    {
        ret = xmlXPathCastToBoolean(obj);
    } else {
        ret = (*obj).boolval;
    }
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
} }
#[inline]
pub fn xmlXPathPopNumber(
    mut ctxt: xmlXPathParserContextPtr,
) -> c_double { unsafe {
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut ret: c_double = 0.;
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            2371 as c_int,
            XPATH_INVALID_OPERAND as c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as c_int;
        }
        return 0 as c_int as c_double;
    }
    if (*obj).type_0 as c_uint
        != XPATH_NUMBER as c_int as c_uint
    {
        ret = xmlXPathCastToNumber(obj);
    } else {
        ret = (*obj).floatval;
    }
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
} }
#[inline]
pub fn xmlXPathPopString(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar { unsafe {
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPatherror(
            ctxt,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            2398 as c_int,
            XPATH_INVALID_OPERAND as c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as c_int;
        }
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlXPathCastToString(obj);
    if (*obj).stringval == ret {
        (*obj).stringval = ::core::ptr::null_mut::<xmlChar>();
    }
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
} }
#[inline]
pub fn xmlXPathPopNodeSet(mut ctxt: xmlXPathParserContextPtr) -> xmlNodeSetPtr { unsafe {
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut ret: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlNodeSet>();
    }
    if (*ctxt).value.is_null() {
        xmlXPatherror(
            ctxt,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            2425 as c_int,
            XPATH_INVALID_OPERAND as c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as c_int;
        }
        return ::core::ptr::null_mut::<xmlNodeSet>();
    }
    if !(!(*ctxt).value.is_null()
        && ((*(*ctxt).value).type_0 as c_uint
            == XPATH_NODESET as c_int as c_uint
            || (*(*ctxt).value).type_0 as c_uint
                == XPATH_XSLT_TREE as c_int as c_uint))
    {
        xmlXPatherror(
            ctxt,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            2429 as c_int,
            XPATH_INVALID_TYPE as c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_TYPE as c_int;
        }
        return ::core::ptr::null_mut::<xmlNodeSet>();
    }
    obj = valuePop(ctxt);
    ret = (*obj).nodesetval;
    (*obj).nodesetval = ::core::ptr::null_mut::<xmlNodeSet>();
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
} }
#[inline]
pub fn xmlXPathPopExternal(
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut c_void { unsafe {
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut ret: *mut c_void = ::core::ptr::null_mut::<c_void>();
    if ctxt.is_null() || (*ctxt).value.is_null() {
        xmlXPatherror(
            ctxt,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            2459 as c_int,
            XPATH_INVALID_OPERAND as c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_OPERAND as c_int;
        }
        return ::core::ptr::null_mut::<c_void>();
    }
    if (*(*ctxt).value).type_0 as c_uint
        != XPATH_USERS as c_int as c_uint
    {
        xmlXPatherror(
            ctxt,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            2463 as c_int,
            XPATH_INVALID_TYPE as c_int,
        );
        if !ctxt.is_null() {
            (*ctxt).error = XPATH_INVALID_TYPE as c_int;
        }
        return ::core::ptr::null_mut::<c_void>();
    }
    obj = valuePop(ctxt);
    ret = (*obj).user;
    (*obj).user = NULL;
    xmlXPathReleaseObject((*ctxt).context, obj);
    return ret;
} }
pub const UPPER_DOUBLE: c_double = 1E9f64;
pub const LOWER_DOUBLE: c_double = 1E-5f64;
pub const EXPONENT_DIGITS: c_int = 3 as c_int + 2 as c_int;
unsafe fn xmlXPathFormatNumber(
    mut number: c_double,
    mut buffer: *mut c_char,
    mut buffersize: c_int,
) {
    match xmlXPathIsInf(number) {
        1 => {
            if buffersize > ::core::mem::size_of::<[c_char; 9]>() as c_int
            {
                snprintf(
                    buffer as *mut c_char,
                    buffersize as size_t,
                    b"Infinity\0" as *const u8 as *const c_char,
                );
            }
        }
        -1 => {
            if buffersize
                > ::core::mem::size_of::<[c_char; 10]>() as c_int
            {
                snprintf(
                    buffer as *mut c_char,
                    buffersize as size_t,
                    b"-Infinity\0" as *const u8 as *const c_char,
                );
            }
        }
        _ => {
            if xmlXPathIsNaN(number) != 0 {
                if buffersize
                    > ::core::mem::size_of::<[c_char; 4]>() as c_int
                {
                    snprintf(
                        buffer as *mut c_char,
                        buffersize as size_t,
                        b"NaN\0" as *const u8 as *const c_char,
                    );
                }
            } else if number == 0 as c_int as c_double {
                snprintf(
                    buffer as *mut c_char,
                    buffersize as size_t,
                    b"0\0" as *const u8 as *const c_char,
                );
            } else if number > INT_MIN as c_double
                && number < INT_MAX as c_double
                && number == number as c_int as c_double
            {
                let mut work: [c_char; 30] = [0; 30];
                let mut ptr: *mut c_char =
                    ::core::ptr::null_mut::<c_char>();
                let mut cur: *mut c_char =
                    ::core::ptr::null_mut::<c_char>();
                let mut value: c_int = number as c_int;
                ptr = buffer.offset(0 as c_int as isize) as *mut c_char;
                if value == 0 as c_int {
                    let fresh28 = ptr;
                    ptr = ptr.offset(1);
                    *fresh28 = '0' as i32 as c_char;
                } else {
                    snprintf(
                        &raw mut work as *mut c_char,
                        29 as size_t,
                        b"%d\0" as *const u8 as *const c_char,
                        value,
                    );
                    cur = (&raw mut work as *mut c_char)
                        .offset(0 as c_int as isize)
                        as *mut c_char;
                    while *cur as c_int != 0
                        && (ptr.offset_from(buffer) as c_long)
                            < buffersize as c_long
                    {
                        let fresh29 = cur;
                        cur = cur.offset(1);
                        let fresh30 = ptr;
                        ptr = ptr.offset(1);
                        *fresh30 = *fresh29;
                    }
                }
                if (ptr.offset_from(buffer) as c_long)
                    < buffersize as c_long
                {
                    *ptr = 0 as c_char;
                } else if buffersize > 0 as c_int {
                    ptr = ptr.offset(-1);
                    *ptr = 0 as c_char;
                }
            } else {
                let mut work_0: [c_char; 28] = [0; 28];
                let mut integer_place: c_int = 0;
                let mut fraction_place: c_int = 0;
                let mut ptr_0: *mut c_char =
                    ::core::ptr::null_mut::<c_char>();
                let mut after_fraction: *mut c_char =
                    ::core::ptr::null_mut::<c_char>();
                let mut absolute_value: c_double = 0.;
                let mut size: c_int = 0;
                absolute_value = fabs(number);
                if (absolute_value > UPPER_DOUBLE || absolute_value < LOWER_DOUBLE)
                    && absolute_value != 0.0f64
                {
                    integer_place = DBL_DIG + EXPONENT_DIGITS + 1 as c_int;
                    fraction_place = DBL_DIG - 1 as c_int;
                    size = snprintf(
                        &raw mut work_0 as *mut c_char,
                        ::core::mem::size_of::<[c_char; 28]>() as size_t,
                        b"%*.*e\0" as *const u8 as *const c_char,
                        integer_place,
                        fraction_place,
                        number,
                    );
                    while size > 0 as c_int
                        && work_0[size as usize] as c_int != 'e' as i32
                    {
                        size -= 1;
                    }
                } else {
                    if absolute_value > 0.0f64 {
                        integer_place = log10(absolute_value) as c_int;
                        if integer_place > 0 as c_int {
                            fraction_place = DBL_DIG - integer_place - 1 as c_int;
                        } else {
                            fraction_place = DBL_DIG - integer_place;
                        }
                    } else {
                        fraction_place = 1 as c_int;
                    }
                    size = snprintf(
                        &raw mut work_0 as *mut c_char,
                        ::core::mem::size_of::<[c_char; 28]>() as size_t,
                        b"%0.*f\0" as *const u8 as *const c_char,
                        fraction_place,
                        number,
                    );
                }
                while work_0[0 as c_int as usize] as c_int == ' ' as i32 {
                    ptr_0 = (&raw mut work_0 as *mut c_char)
                        .offset(0 as c_int as isize)
                        as *mut c_char;
                    loop {
                        let ref mut fresh31 = *ptr_0.offset(0 as c_int as isize);
                        *fresh31 = *ptr_0.offset(1 as c_int as isize);
                        if !(*fresh31 != 0) {
                            break;
                        }
                        ptr_0 = ptr_0.offset(1);
                    }
                    size -= 1;
                }
                after_fraction =
                    (&raw mut work_0 as *mut c_char).offset(size as isize);
                ptr_0 = after_fraction;
                loop {
                    ptr_0 = ptr_0.offset(-1);
                    if !(*ptr_0 as c_int == '0' as i32) {
                        break;
                    }
                }
                if *ptr_0 as c_int != '.' as i32 {
                    ptr_0 = ptr_0.offset(1);
                }
                loop {
                    let fresh32 = after_fraction;
                    after_fraction = after_fraction.offset(1);
                    let fresh33 = ptr_0;
                    ptr_0 = ptr_0.offset(1);
                    *fresh33 = *fresh32;
                    if !(*fresh33 as c_int != 0 as c_int) {
                        break;
                    }
                }
                size = strlen(&raw mut work_0 as *mut c_char).wrapping_add(1 as size_t)
                    as c_int;
                if size > buffersize {
                    work_0[(buffersize - 1 as c_int) as usize] =
                        0 as c_char;
                    size = buffersize;
                }
                memmove(
                    buffer as *mut c_void,
                    &raw mut work_0 as *mut c_char as *const c_void,
                    size as size_t,
                );
            }
        }
    };
}
#[inline]
pub fn xmlXPathOrderDocElems(mut doc: xmlDocPtr) -> c_long { unsafe {
    let mut count: ptrdiff_t = 0 as ptrdiff_t;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if doc.is_null() {
        return -(1 as c_int) as c_long;
    }
    cur = (*doc).children as xmlNodePtr;
    while !cur.is_null() {
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            count += 1;
            (*cur).content = -count as *mut c_void as *mut xmlChar;
            if !(*cur).children.is_null() {
                cur = (*cur).children as xmlNodePtr;
                continue;
            }
        }
        if !(*cur).next.is_null() {
            cur = (*cur).next as xmlNodePtr;
        } else {
            loop {
                cur = (*cur).parent as xmlNodePtr;
                if cur.is_null() {
                    break;
                }
                if cur == doc as xmlNodePtr {
                    cur = ::core::ptr::null_mut::<xmlNode>();
                    break;
                } else if !(*cur).next.is_null() {
                    cur = (*cur).next as xmlNodePtr;
                    break;
                } else if cur.is_null() {
                    break;
                }
            }
        }
    }
    return count as c_long;
} }
#[inline]
pub fn xmlXPathCmpNodes(
    mut node1: xmlNodePtr,
    mut node2: xmlNodePtr,
) -> c_int { unsafe {
    let mut depth1: c_int = 0;
    let mut depth2: c_int = 0;
    let mut attr1: c_int = 0 as c_int;
    let mut attr2: c_int = 0 as c_int;
    let mut attrNode1: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut attrNode2: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut root: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if node1.is_null() || node2.is_null() {
        return -(2 as c_int);
    }
    if node1 == node2 {
        return 0 as c_int;
    }
    if (*node1).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        attr1 = 1 as c_int;
        attrNode1 = node1;
        node1 = (*node1).parent as xmlNodePtr;
    }
    if (*node2).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
    {
        attr2 = 1 as c_int;
        attrNode2 = node2;
        node2 = (*node2).parent as xmlNodePtr;
    }
    if node1 == node2 {
        if attr1 == attr2 {
            if attr1 != 0 as c_int {
                cur = (*attrNode2).prev as xmlNodePtr;
                while !cur.is_null() {
                    if cur == attrNode1 {
                        return 1 as c_int;
                    }
                    cur = (*cur).prev as xmlNodePtr;
                }
                return -(1 as c_int);
            }
            return 0 as c_int;
        }
        if attr2 == 1 as c_int {
            return 1 as c_int;
        }
        return -(1 as c_int);
    }
    if (*node1).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
        || (*node2).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return 1 as c_int;
    }
    if node1 == (*node2).prev {
        return 1 as c_int;
    }
    if node1 == (*node2).next {
        return -(1 as c_int);
    }
    if (*node1).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        && (*node2).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        && 0 as ptrdiff_t > (*node1).content as ptrdiff_t
        && 0 as ptrdiff_t > (*node2).content as ptrdiff_t
        && (*node1).doc == (*node2).doc
    {
        let mut l1: ptrdiff_t = 0;
        let mut l2: ptrdiff_t = 0;
        l1 = -((*node1).content as ptrdiff_t);
        l2 = -((*node2).content as ptrdiff_t);
        if l1 < l2 {
            return 1 as c_int;
        }
        if l1 > l2 {
            return -(1 as c_int);
        }
    }
    depth2 = 0 as c_int;
    cur = node2;
    while !(*cur).parent.is_null() {
        if (*cur).parent == node1 {
            return 1 as c_int;
        }
        depth2 += 1;
        cur = (*cur).parent as xmlNodePtr;
    }
    root = cur;
    depth1 = 0 as c_int;
    cur = node1;
    while !(*cur).parent.is_null() {
        if (*cur).parent == node2 {
            return -(1 as c_int);
        }
        depth1 += 1;
        cur = (*cur).parent as xmlNodePtr;
    }
    if root != cur {
        return -(2 as c_int);
    }
    while depth1 > depth2 {
        depth1 -= 1;
        node1 = (*node1).parent as xmlNodePtr;
    }
    while depth2 > depth1 {
        depth2 -= 1;
        node2 = (*node2).parent as xmlNodePtr;
    }
    while (*node1).parent != (*node2).parent {
        node1 = (*node1).parent as xmlNodePtr;
        node2 = (*node2).parent as xmlNodePtr;
        if node1.is_null() || node2.is_null() {
            return -(2 as c_int);
        }
    }
    if node1 == (*node2).prev {
        return 1 as c_int;
    }
    if node1 == (*node2).next {
        return -(1 as c_int);
    }
    if (*node1).type_0 as c_uint
        == XML_ELEMENT_NODE as c_int as c_uint
        && (*node2).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        && 0 as ptrdiff_t > (*node1).content as ptrdiff_t
        && 0 as ptrdiff_t > (*node2).content as ptrdiff_t
        && (*node1).doc == (*node2).doc
    {
        let mut l1_0: ptrdiff_t = 0;
        let mut l2_0: ptrdiff_t = 0;
        l1_0 = -((*node1).content as ptrdiff_t);
        l2_0 = -((*node2).content as ptrdiff_t);
        if l1_0 < l2_0 {
            return 1 as c_int;
        }
        if l1_0 > l2_0 {
            return -(1 as c_int);
        }
    }
    cur = (*node1).next as xmlNodePtr;
    while !cur.is_null() {
        if cur == node2 {
            return 1 as c_int;
        }
        cur = (*cur).next as xmlNodePtr;
    }
    return -(1 as c_int);
} }
#[inline]
pub fn xmlXPathNodeSetSort(mut set: xmlNodeSetPtr) { unsafe {
    if set.is_null() {
        return;
    }
    libxml_domnode_tim_sort((*set).nodeTab, (*set).nodeNr as size_t);
} }
pub const XML_NODESET_DEFAULT: c_int = 10 as c_int;
fn xmlXPathNodeSetDupNs(mut node: xmlNodePtr, mut ns: xmlNsPtr) -> xmlNodePtr { unsafe {
    let mut cur: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if ns.is_null()
        || (*ns).type_0 as c_uint
            != XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if node.is_null()
        || (*node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ns as xmlNodePtr;
    }
    cur = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlNs>() as size_t)
        as xmlNsPtr;
    if cur.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"duplicating namespace\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlNode>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNs>() as size_t,
    );
    (*cur).type_0 = XML_NAMESPACE_DECL;
    if !(*ns).href.is_null() {
        (*cur).href = xmlStrdup((*ns).href);
    }
    if !(*ns).prefix.is_null() {
        (*cur).prefix = xmlStrdup((*ns).prefix);
    }
    (*cur).next = node as xmlNsPtr as *mut _xmlNs;
    return cur as xmlNodePtr;
} }
#[inline]
pub fn xmlXPathNodeSetFreeNs(mut ns: xmlNsPtr) { unsafe {
    if ns.is_null()
        || (*ns).type_0 as c_uint
            != XML_NAMESPACE_DECL as c_int as c_uint
    {
        return;
    }
    if !(*ns).next.is_null()
        && (*(*ns).next).type_0 as c_uint
            != XML_NAMESPACE_DECL as c_int as c_uint
    {
        if !(*ns).href.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ns).href as *mut xmlChar as *mut c_void,
            );
        }
        if !(*ns).prefix.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ns).prefix as *mut xmlChar as *mut c_void,
            );
        }
        xmlFree.expect("non-null function pointer")(ns as *mut c_void);
    }
} }
#[inline]
pub fn xmlXPathNodeSetCreate(mut val: xmlNodePtr) -> xmlNodeSetPtr { unsafe {
    let mut ret: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlNodeSet>() as size_t
    ) as xmlNodeSetPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating nodeset\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlNodeSet>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNodeSet>() as size_t,
    );
    if !val.is_null() {
        (*ret).nodeTab = xmlMalloc.expect("non-null function pointer")(
            (XML_NODESET_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if (*ret).nodeTab.is_null() {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"creating nodeset\n\0" as *const u8 as *const c_char,
            );
            xmlFree.expect("non-null function pointer")(ret as *mut c_void);
            return ::core::ptr::null_mut::<xmlNodeSet>();
        }
        memset(
            (*ret).nodeTab as *mut c_void,
            0 as c_int,
            (XML_NODESET_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        (*ret).nodeMax = XML_NODESET_DEFAULT;
        if (*val).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
        {
            let mut ns: xmlNsPtr = val as xmlNsPtr;
            let mut nsNode: xmlNodePtr = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns);
            if nsNode.is_null() {
                xmlXPathFreeNodeSet(ret);
                return ::core::ptr::null_mut::<xmlNodeSet>();
            }
            let fresh0 = (*ret).nodeNr;
            (*ret).nodeNr = (*ret).nodeNr + 1;
            let ref mut fresh1 = *(*ret).nodeTab.offset(fresh0 as isize);
            *fresh1 = nsNode;
        } else {
            let fresh2 = (*ret).nodeNr;
            (*ret).nodeNr = (*ret).nodeNr + 1;
            let ref mut fresh3 = *(*ret).nodeTab.offset(fresh2 as isize);
            *fresh3 = val;
        }
    }
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlXPathNodeSetContains(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    if cur.is_null() || val.is_null() {
        return 0 as c_int;
    }
    if (*val).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        i = 0 as c_int;
        while i < (*cur).nodeNr {
            if (**(*cur).nodeTab.offset(i as isize)).type_0 as c_uint
                == XML_NAMESPACE_DECL as c_int as c_uint
            {
                let mut ns1: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                let mut ns2: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                ns1 = val as xmlNsPtr;
                ns2 = *(*cur).nodeTab.offset(i as isize) as xmlNsPtr;
                if ns1 == ns2 {
                    return 1 as c_int;
                }
                if !(*ns1).next.is_null()
                    && (*ns2).next == (*ns1).next
                    && xmlStrEqual((*ns1).prefix, (*ns2).prefix) != 0
                {
                    return 1 as c_int;
                }
            }
            i += 1;
        }
    } else {
        i = 0 as c_int;
        while i < (*cur).nodeNr {
            if *(*cur).nodeTab.offset(i as isize) == val {
                return 1 as c_int;
            }
            i += 1;
        }
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlXPathNodeSetAddNs(
    mut cur: xmlNodeSetPtr,
    mut node: xmlNodePtr,
    mut ns: xmlNsPtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut nsNode: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if cur.is_null()
        || ns.is_null()
        || node.is_null()
        || (*ns).type_0 as c_uint
            != XML_NAMESPACE_DECL as c_int as c_uint
        || (*node).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < (*cur).nodeNr {
        if !(*(*cur).nodeTab.offset(i as isize)).is_null()
            && (**(*cur).nodeTab.offset(i as isize)).type_0 as c_uint
                == XML_NAMESPACE_DECL as c_int as c_uint
            && (*(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr)).next == node as xmlNsPtr
            && xmlStrEqual(
                (*ns).prefix,
                (*(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr)).prefix,
            ) != 0
        {
            return 0 as c_int;
        }
        i += 1;
    }
    if (*cur).nodeMax == 0 as c_int {
        (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
            (XML_NODESET_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if (*cur).nodeTab.is_null() {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        memset(
            (*cur).nodeTab as *mut c_void,
            0 as c_int,
            (XML_NODESET_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        (*cur).nodeMax = XML_NODESET_DEFAULT;
    } else if (*cur).nodeNr == (*cur).nodeMax {
        let mut temp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
        if (*cur).nodeMax >= XPATH_MAX_NODESET_LENGTH {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset hit limit\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        temp = xmlRealloc.expect("non-null function pointer")(
            (*cur).nodeTab as *mut c_void,
            (((*cur).nodeMax * 2 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if temp.is_null() {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*cur).nodeMax *= 2 as c_int;
        (*cur).nodeTab = temp;
    }
    nsNode = xmlXPathNodeSetDupNs(node, ns);
    if nsNode.is_null() {
        return -(1 as c_int);
    }
    let fresh55 = (*cur).nodeNr;
    (*cur).nodeNr = (*cur).nodeNr + 1;
    let ref mut fresh56 = *(*cur).nodeTab.offset(fresh55 as isize);
    *fresh56 = nsNode;
    return 0 as c_int;
} }
#[inline]
pub fn xmlXPathNodeSetAdd(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    if cur.is_null() || val.is_null() {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < (*cur).nodeNr {
        if *(*cur).nodeTab.offset(i as isize) == val {
            return 0 as c_int;
        }
        i += 1;
    }
    if (*cur).nodeMax == 0 as c_int {
        (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
            (XML_NODESET_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if (*cur).nodeTab.is_null() {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        memset(
            (*cur).nodeTab as *mut c_void,
            0 as c_int,
            (XML_NODESET_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        (*cur).nodeMax = XML_NODESET_DEFAULT;
    } else if (*cur).nodeNr == (*cur).nodeMax {
        let mut temp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
        if (*cur).nodeMax >= XPATH_MAX_NODESET_LENGTH {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset hit limit\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        temp = xmlRealloc.expect("non-null function pointer")(
            (*cur).nodeTab as *mut c_void,
            (((*cur).nodeMax * 2 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if temp.is_null() {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*cur).nodeMax *= 2 as c_int;
        (*cur).nodeTab = temp;
    }
    if (*val).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        let mut ns: xmlNsPtr = val as xmlNsPtr;
        let mut nsNode: xmlNodePtr = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns);
        if nsNode.is_null() {
            return -(1 as c_int);
        }
        let fresh47 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh48 = *(*cur).nodeTab.offset(fresh47 as isize);
        *fresh48 = nsNode;
    } else {
        let fresh49 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh50 = *(*cur).nodeTab.offset(fresh49 as isize);
        *fresh50 = val;
    }
    return 0 as c_int;
} }
#[no_mangle]
pub extern "C" fn xmlXPathNodeSetAddUnique(
    mut cur: xmlNodeSetPtr,
    mut val: xmlNodePtr,
) -> c_int { unsafe {
    if cur.is_null() || val.is_null() {
        return -(1 as c_int);
    }
    if (*cur).nodeMax == 0 as c_int {
        (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
            (XML_NODESET_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if (*cur).nodeTab.is_null() {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        memset(
            (*cur).nodeTab as *mut c_void,
            0 as c_int,
            (XML_NODESET_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        (*cur).nodeMax = XML_NODESET_DEFAULT;
    } else if (*cur).nodeNr == (*cur).nodeMax {
        let mut temp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
        if (*cur).nodeMax >= XPATH_MAX_NODESET_LENGTH {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset hit limit\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        temp = xmlRealloc.expect("non-null function pointer")(
            (*cur).nodeTab as *mut c_void,
            (((*cur).nodeMax * 2 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if temp.is_null() {
            xmlXPathErrMemory(
                ::core::ptr::null_mut::<xmlXPathContext>(),
                b"growing nodeset\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*cur).nodeTab = temp;
        (*cur).nodeMax *= 2 as c_int;
    }
    if (*val).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        let mut ns: xmlNsPtr = val as xmlNsPtr;
        let mut nsNode: xmlNodePtr = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns);
        if nsNode.is_null() {
            return -(1 as c_int);
        }
        let fresh43 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh44 = *(*cur).nodeTab.offset(fresh43 as isize);
        *fresh44 = nsNode;
    } else {
        let fresh45 = (*cur).nodeNr;
        (*cur).nodeNr = (*cur).nodeNr + 1;
        let ref mut fresh46 = *(*cur).nodeTab.offset(fresh45 as isize);
        *fresh46 = val;
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlXPathNodeSetMerge(
    mut val1: xmlNodeSetPtr,
    mut val2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    let mut current_block: u64;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut initNr: c_int = 0;
    let mut skip: c_int = 0;
    let mut n1: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut n2: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if val2.is_null() {
        return val1;
    }
    if val1.is_null() {
        val1 = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
        if val1.is_null() {
            return ::core::ptr::null_mut::<xmlNodeSet>();
        }
    }
    initNr = (*val1).nodeNr;
    i = 0 as c_int;
    loop {
        if !(i < (*val2).nodeNr) {
            current_block = 14832935472441733737;
            break;
        }
        n2 = *(*val2).nodeTab.offset(i as isize);
        skip = 0 as c_int;
        j = 0 as c_int;
        while j < initNr {
            n1 = *(*val1).nodeTab.offset(j as isize);
            if n1 == n2 {
                skip = 1 as c_int;
                break;
            } else {
                if (*n1).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
                    && (*n2).type_0 as c_uint
                        == XML_NAMESPACE_DECL as c_int as c_uint
                {
                    if (*(n1 as xmlNsPtr)).next == (*(n2 as xmlNsPtr)).next
                        && xmlStrEqual((*(n1 as xmlNsPtr)).prefix, (*(n2 as xmlNsPtr)).prefix) != 0
                    {
                        skip = 1 as c_int;
                        break;
                    }
                }
                j += 1;
            }
        }
        if !(skip != 0) {
            if (*val1).nodeMax == 0 as c_int {
                (*val1).nodeTab = xmlMalloc.expect("non-null function pointer")(
                    (XML_NODESET_DEFAULT as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
                ) as *mut xmlNodePtr;
                if (*val1).nodeTab.is_null() {
                    xmlXPathErrMemory(
                        ::core::ptr::null_mut::<xmlXPathContext>(),
                        b"merging nodeset\n\0" as *const u8 as *const c_char,
                    );
                    current_block = 9560816355290546355;
                    break;
                } else {
                    memset(
                        (*val1).nodeTab as *mut c_void,
                        0 as c_int,
                        (XML_NODESET_DEFAULT as size_t)
                            .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
                    );
                    (*val1).nodeMax = XML_NODESET_DEFAULT;
                }
            } else if (*val1).nodeNr == (*val1).nodeMax {
                let mut temp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
                if (*val1).nodeMax >= XPATH_MAX_NODESET_LENGTH {
                    xmlXPathErrMemory(
                        ::core::ptr::null_mut::<xmlXPathContext>(),
                        b"merging nodeset hit limit\n\0" as *const u8 as *const c_char,
                    );
                    current_block = 9560816355290546355;
                    break;
                } else {
                    temp = xmlRealloc.expect("non-null function pointer")(
                        (*val1).nodeTab as *mut c_void,
                        (((*val1).nodeMax * 2 as c_int) as size_t)
                            .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
                    ) as *mut xmlNodePtr;
                    if temp.is_null() {
                        xmlXPathErrMemory(
                            ::core::ptr::null_mut::<xmlXPathContext>(),
                            b"merging nodeset\n\0" as *const u8 as *const c_char,
                        );
                        current_block = 9560816355290546355;
                        break;
                    } else {
                        (*val1).nodeTab = temp;
                        (*val1).nodeMax *= 2 as c_int;
                    }
                }
            }
            if (*n2).type_0 as c_uint
                == XML_NAMESPACE_DECL as c_int as c_uint
            {
                let mut ns: xmlNsPtr = n2 as xmlNsPtr;
                let mut nsNode: xmlNodePtr = xmlXPathNodeSetDupNs((*ns).next as xmlNodePtr, ns);
                if nsNode.is_null() {
                    current_block = 9560816355290546355;
                    break;
                }
                let fresh4 = (*val1).nodeNr;
                (*val1).nodeNr = (*val1).nodeNr + 1;
                let ref mut fresh5 = *(*val1).nodeTab.offset(fresh4 as isize);
                *fresh5 = nsNode;
            } else {
                let fresh6 = (*val1).nodeNr;
                (*val1).nodeNr = (*val1).nodeNr + 1;
                let ref mut fresh7 = *(*val1).nodeTab.offset(fresh6 as isize);
                *fresh7 = n2;
            }
        }
        i += 1;
    }
    match current_block {
        14832935472441733737 => return val1,
        _ => {
            xmlXPathFreeNodeSet(val1);
            return ::core::ptr::null_mut::<xmlNodeSet>();
        }
    };
} }
extern "C" fn xmlXPathNodeSetMergeAndClear(
    mut set1: xmlNodeSetPtr,
    mut set2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    let mut current_block: u64;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut initNbSet1: c_int = 0;
    let mut n1: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut n2: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    initNbSet1 = (*set1).nodeNr;
    i = 0 as c_int;
    loop {
        if !(i < (*set2).nodeNr) {
            current_block = 13550086250199790493;
            break;
        }
        n2 = *(*set2).nodeTab.offset(i as isize);
        j = 0 as c_int;
        loop {
            if !(j < initNbSet1) {
                current_block = 7651349459974463963;
                break;
            }
            n1 = *(*set1).nodeTab.offset(j as isize);
            if n1 == n2 {
                current_block = 8262508561327738881;
                break;
            }
            if (*n1).type_0 as c_uint
                == XML_NAMESPACE_DECL as c_int as c_uint
                && (*n2).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
            {
                if (*(n1 as xmlNsPtr)).next == (*(n2 as xmlNsPtr)).next
                    && xmlStrEqual((*(n1 as xmlNsPtr)).prefix, (*(n2 as xmlNsPtr)).prefix) != 0
                {
                    xmlXPathNodeSetFreeNs(n2 as xmlNsPtr);
                    current_block = 8262508561327738881;
                    break;
                }
            }
            j += 1;
        }
        match current_block {
            7651349459974463963 => {
                if (*set1).nodeMax == 0 as c_int {
                    (*set1).nodeTab = xmlMalloc.expect("non-null function pointer")(
                        (XML_NODESET_DEFAULT as size_t)
                            .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
                    ) as *mut xmlNodePtr;
                    if (*set1).nodeTab.is_null() {
                        xmlXPathErrMemory(
                            ::core::ptr::null_mut::<xmlXPathContext>(),
                            b"merging nodeset\n\0" as *const u8 as *const c_char,
                        );
                        current_block = 12010185295218340136;
                        break;
                    } else {
                        memset(
                            (*set1).nodeTab as *mut c_void,
                            0 as c_int,
                            (XML_NODESET_DEFAULT as size_t).wrapping_mul(::core::mem::size_of::<
                                xmlNodePtr,
                            >(
                            )
                                as size_t),
                        );
                        (*set1).nodeMax = XML_NODESET_DEFAULT;
                    }
                } else if (*set1).nodeNr >= (*set1).nodeMax {
                    let mut temp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
                    if (*set1).nodeMax >= XPATH_MAX_NODESET_LENGTH {
                        xmlXPathErrMemory(
                            ::core::ptr::null_mut::<xmlXPathContext>(),
                            b"merging nodeset hit limit\n\0" as *const u8
                                as *const c_char,
                        );
                        current_block = 12010185295218340136;
                        break;
                    } else {
                        temp = xmlRealloc.expect("non-null function pointer")(
                            (*set1).nodeTab as *mut c_void,
                            (((*set1).nodeMax * 2 as c_int) as size_t)
                                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
                        ) as *mut xmlNodePtr;
                        if temp.is_null() {
                            xmlXPathErrMemory(
                                ::core::ptr::null_mut::<xmlXPathContext>(),
                                b"merging nodeset\n\0" as *const u8 as *const c_char,
                            );
                            current_block = 12010185295218340136;
                            break;
                        } else {
                            (*set1).nodeTab = temp;
                            (*set1).nodeMax *= 2 as c_int;
                        }
                    }
                }
                let fresh60 = (*set1).nodeNr;
                (*set1).nodeNr = (*set1).nodeNr + 1;
                let ref mut fresh61 = *(*set1).nodeTab.offset(fresh60 as isize);
                *fresh61 = n2;
            }
            _ => {}
        }
        let ref mut fresh62 = *(*set2).nodeTab.offset(i as isize);
        *fresh62 = ::core::ptr::null_mut::<xmlNode>();
        i += 1;
    }
    match current_block {
        12010185295218340136 => {
            xmlXPathFreeNodeSet(set1);
            xmlXPathNodeSetClear(set2, 1 as c_int);
            return ::core::ptr::null_mut::<xmlNodeSet>();
        }
        _ => {
            (*set2).nodeNr = 0 as c_int;
            return set1;
        }
    };
} }
extern "C" fn xmlXPathNodeSetMergeAndClearNoDupls(
    mut set1: xmlNodeSetPtr,
    mut set2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    let mut current_block: u64;
    let mut i: c_int = 0;
    let mut n2: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    i = 0 as c_int;
    loop {
        if !(i < (*set2).nodeNr) {
            current_block = 15904375183555213903;
            break;
        }
        n2 = *(*set2).nodeTab.offset(i as isize);
        if (*set1).nodeMax == 0 as c_int {
            (*set1).nodeTab = xmlMalloc.expect("non-null function pointer")(
                (XML_NODESET_DEFAULT as size_t)
                    .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
            ) as *mut xmlNodePtr;
            if (*set1).nodeTab.is_null() {
                xmlXPathErrMemory(
                    ::core::ptr::null_mut::<xmlXPathContext>(),
                    b"merging nodeset\n\0" as *const u8 as *const c_char,
                );
                current_block = 6154001708813816476;
                break;
            } else {
                memset(
                    (*set1).nodeTab as *mut c_void,
                    0 as c_int,
                    (XML_NODESET_DEFAULT as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
                );
                (*set1).nodeMax = XML_NODESET_DEFAULT;
            }
        } else if (*set1).nodeNr >= (*set1).nodeMax {
            let mut temp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
            if (*set1).nodeMax >= XPATH_MAX_NODESET_LENGTH {
                xmlXPathErrMemory(
                    ::core::ptr::null_mut::<xmlXPathContext>(),
                    b"merging nodeset hit limit\n\0" as *const u8 as *const c_char,
                );
                current_block = 6154001708813816476;
                break;
            } else {
                temp = xmlRealloc.expect("non-null function pointer")(
                    (*set1).nodeTab as *mut c_void,
                    (((*set1).nodeMax * 2 as c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
                ) as *mut xmlNodePtr;
                if temp.is_null() {
                    xmlXPathErrMemory(
                        ::core::ptr::null_mut::<xmlXPathContext>(),
                        b"merging nodeset\n\0" as *const u8 as *const c_char,
                    );
                    current_block = 6154001708813816476;
                    break;
                } else {
                    (*set1).nodeTab = temp;
                    (*set1).nodeMax *= 2 as c_int;
                }
            }
        }
        let fresh57 = (*set1).nodeNr;
        (*set1).nodeNr = (*set1).nodeNr + 1;
        let ref mut fresh58 = *(*set1).nodeTab.offset(fresh57 as isize);
        *fresh58 = n2;
        let ref mut fresh59 = *(*set2).nodeTab.offset(i as isize);
        *fresh59 = ::core::ptr::null_mut::<xmlNode>();
        i += 1;
    }
    match current_block {
        6154001708813816476 => {
            xmlXPathFreeNodeSet(set1);
            xmlXPathNodeSetClear(set2, 1 as c_int);
            return ::core::ptr::null_mut::<xmlNodeSet>();
        }
        _ => {
            (*set2).nodeNr = 0 as c_int;
            return set1;
        }
    };
} }
#[inline]
pub fn xmlXPathNodeSetDel(mut cur: xmlNodeSetPtr, mut val: xmlNodePtr) { unsafe {
    let mut i: c_int = 0;
    if cur.is_null() {
        return;
    }
    if val.is_null() {
        return;
    }
    i = 0 as c_int;
    while i < (*cur).nodeNr {
        if *(*cur).nodeTab.offset(i as isize) == val {
            break;
        }
        i += 1;
    }
    if i >= (*cur).nodeNr {
        return;
    }
    if !(*(*cur).nodeTab.offset(i as isize)).is_null()
        && (**(*cur).nodeTab.offset(i as isize)).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        xmlXPathNodeSetFreeNs(*(*cur).nodeTab.offset(i as isize) as xmlNsPtr);
    }
    (*cur).nodeNr -= 1;
    while i < (*cur).nodeNr {
        let ref mut fresh83 = *(*cur).nodeTab.offset(i as isize);
        *fresh83 = *(*cur)
            .nodeTab
            .offset((i + 1 as c_int) as isize);
        i += 1;
    }
    let ref mut fresh84 = *(*cur).nodeTab.offset((*cur).nodeNr as isize);
    *fresh84 = ::core::ptr::null_mut::<xmlNode>();
} }
#[inline]
pub fn xmlXPathNodeSetRemove(
    mut cur: xmlNodeSetPtr,
    mut val: c_int,
) { unsafe {
    if cur.is_null() {
        return;
    }
    if val >= (*cur).nodeNr {
        return;
    }
    if !(*(*cur).nodeTab.offset(val as isize)).is_null()
        && (**(*cur).nodeTab.offset(val as isize)).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        xmlXPathNodeSetFreeNs(*(*cur).nodeTab.offset(val as isize) as xmlNsPtr);
    }
    (*cur).nodeNr -= 1;
    while val < (*cur).nodeNr {
        let ref mut fresh85 = *(*cur).nodeTab.offset(val as isize);
        *fresh85 = *(*cur)
            .nodeTab
            .offset((val + 1 as c_int) as isize);
        val += 1;
    }
    let ref mut fresh86 = *(*cur).nodeTab.offset((*cur).nodeNr as isize);
    *fresh86 = ::core::ptr::null_mut::<xmlNode>();
} }
#[inline]
pub fn xmlXPathFreeNodeSet(mut obj: xmlNodeSetPtr) { unsafe {
    if obj.is_null() {
        return;
    }
    if !(*obj).nodeTab.is_null() {
        let mut i: c_int = 0;
        i = 0 as c_int;
        while i < (*obj).nodeNr {
            if !(*(*obj).nodeTab.offset(i as isize)).is_null()
                && (**(*obj).nodeTab.offset(i as isize)).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
            {
                xmlXPathNodeSetFreeNs(*(*obj).nodeTab.offset(i as isize) as xmlNsPtr);
            }
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*obj).nodeTab as *mut c_void);
    }
    xmlFree.expect("non-null function pointer")(obj as *mut c_void);
} }
fn xmlXPathNodeSetClearFromPos(
    mut set: xmlNodeSetPtr,
    mut pos: c_int,
    mut hasNsNodes: c_int,
) { unsafe {
    if set.is_null() || pos >= (*set).nodeNr {
        return;
    } else if hasNsNodes != 0 {
        let mut i: c_int = 0;
        let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        i = pos;
        while i < (*set).nodeNr {
            node = *(*set).nodeTab.offset(i as isize);
            if !node.is_null()
                && (*node).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
            {
                xmlXPathNodeSetFreeNs(node as xmlNsPtr);
            }
            i += 1;
        }
    }
    (*set).nodeNr = pos;
} }
fn xmlXPathNodeSetClear(
    mut set: xmlNodeSetPtr,
    mut hasNsNodes: c_int,
) { {
    xmlXPathNodeSetClearFromPos(set, 0 as c_int, hasNsNodes);
} }
fn xmlXPathNodeSetKeepLast(mut set: xmlNodeSetPtr) { unsafe {
    let mut i: c_int = 0;
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if set.is_null() || (*set).nodeNr <= 1 as c_int {
        return;
    }
    i = 0 as c_int;
    while i < (*set).nodeNr - 1 as c_int {
        node = *(*set).nodeTab.offset(i as isize);
        if !node.is_null()
            && (*node).type_0 as c_uint
                == XML_NAMESPACE_DECL as c_int as c_uint
        {
            xmlXPathNodeSetFreeNs(node as xmlNsPtr);
        }
        i += 1;
    }
    let ref mut fresh63 = *(*set).nodeTab.offset(0 as c_int as isize);
    *fresh63 = *(*set)
        .nodeTab
        .offset(((*set).nodeNr - 1 as c_int) as isize);
    (*set).nodeNr = 1 as c_int;
} }
fn xmlXPathFreeValueTree(mut obj: xmlNodeSetPtr) { unsafe {
    let mut i: c_int = 0;
    if obj.is_null() {
        return;
    }
    if !(*obj).nodeTab.is_null() {
        i = 0 as c_int;
        while i < (*obj).nodeNr {
            if !(*(*obj).nodeTab.offset(i as isize)).is_null() {
                if (**(*obj).nodeTab.offset(i as isize)).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
                {
                    xmlXPathNodeSetFreeNs(*(*obj).nodeTab.offset(i as isize) as xmlNsPtr);
                } else {
                    xmlFreeNodeList(*(*obj).nodeTab.offset(i as isize));
                }
            }
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*obj).nodeTab as *mut c_void);
    }
    xmlFree.expect("non-null function pointer")(obj as *mut c_void);
} }
#[no_mangle]
pub extern "C" fn xmlXPathNewNodeSet(mut val: xmlNodePtr) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating nodeset\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    (*ret).type_0 = XPATH_NODESET;
    (*ret).boolval = 0 as c_int;
    (*ret).nodesetval = xmlXPathNodeSetCreate(val);
    return ret;
} }
#[inline]
pub fn xmlXPathNewValueTree(mut val: xmlNodePtr) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating result value tree\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    (*ret).type_0 = XPATH_XSLT_TREE;
    (*ret).boolval = 1 as c_int;
    (*ret).user = val as *mut c_void;
    (*ret).nodesetval = xmlXPathNodeSetCreate(val);
    return ret;
} }
#[inline]
pub fn xmlXPathNewNodeSetList(mut val: xmlNodeSetPtr) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut i: c_int = 0;
    if val.is_null() {
        ret = ::core::ptr::null_mut::<xmlXPathObject>();
    } else if (*val).nodeTab.is_null() {
        ret = xmlXPathNewNodeSet(::core::ptr::null_mut::<xmlNode>());
    } else {
        ret = xmlXPathNewNodeSet(*(*val).nodeTab.offset(0 as c_int as isize));
        if !ret.is_null() {
            i = 1 as c_int;
            while i < (*val).nodeNr {
                if xmlXPathNodeSetAddUnique((*ret).nodesetval, *(*val).nodeTab.offset(i as isize))
                    < 0 as c_int
                {
                    break;
                }
                i += 1;
            }
        }
    }
    return ret;
} }
#[inline]
pub fn xmlXPathWrapNodeSet(mut val: xmlNodeSetPtr) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating node set object\n\0" as *const u8 as *const c_char,
        );
        xmlXPathFreeNodeSet(val);
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    (*ret).type_0 = XPATH_NODESET;
    (*ret).nodesetval = val;
    return ret;
} }
#[inline]
pub fn xmlXPathFreeNodeSetList(mut obj: xmlXPathObjectPtr) { unsafe {
    if obj.is_null() {
        return;
    }
    xmlFree.expect("non-null function pointer")(obj as *mut c_void);
} }
#[inline]
pub fn xmlXPathDifference(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    let mut ret: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut i: c_int = 0;
    let mut l1: c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if nodes2.is_null()
        || (*nodes2).nodeNr == 0 as c_int
        || (*nodes2).nodeTab.is_null()
    {
        return nodes1;
    }
    ret = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
    if nodes1.is_null()
        || (*nodes1).nodeNr == 0 as c_int
        || (*nodes1).nodeTab.is_null()
    {
        return ret;
    }
    l1 = if !nodes1.is_null() {
        (*nodes1).nodeNr
    } else {
        0 as c_int
    };
    i = 0 as c_int;
    while i < l1 {
        cur = if !nodes1.is_null() && i >= 0 as c_int && i < (*nodes1).nodeNr {
            *(*nodes1).nodeTab.offset(i as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        };
        if xmlXPathNodeSetContains(nodes2, cur) == 0 {
            if xmlXPathNodeSetAddUnique(ret, cur) < 0 as c_int {
                break;
            }
        }
        i += 1;
    }
    return ret;
} }
#[inline]
pub fn xmlXPathIntersection(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    let mut ret: xmlNodeSetPtr = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
    let mut i: c_int = 0;
    let mut l1: c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ret.is_null() {
        return ret;
    }
    if nodes1.is_null()
        || (*nodes1).nodeNr == 0 as c_int
        || (*nodes1).nodeTab.is_null()
    {
        return ret;
    }
    if nodes2.is_null()
        || (*nodes2).nodeNr == 0 as c_int
        || (*nodes2).nodeTab.is_null()
    {
        return ret;
    }
    l1 = if !nodes1.is_null() {
        (*nodes1).nodeNr
    } else {
        0 as c_int
    };
    i = 0 as c_int;
    while i < l1 {
        cur = if !nodes1.is_null() && i >= 0 as c_int && i < (*nodes1).nodeNr {
            *(*nodes1).nodeTab.offset(i as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        };
        if xmlXPathNodeSetContains(nodes2, cur) != 0 {
            if xmlXPathNodeSetAddUnique(ret, cur) < 0 as c_int {
                break;
            }
        }
        i += 1;
    }
    return ret;
} }
#[inline]
pub fn xmlXPathDistinctSorted(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr { unsafe {
    let mut current_block: u64;
    let mut ret: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut hash: xmlHashTablePtr = ::core::ptr::null_mut::<xmlHashTable>();
    let mut i: c_int = 0;
    let mut l: c_int = 0;
    let mut strval: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if nodes.is_null() || (*nodes).nodeNr == 0 as c_int || (*nodes).nodeTab.is_null() {
        return nodes;
    }
    ret = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
    if ret.is_null() {
        return ret;
    }
    l = if !nodes.is_null() {
        (*nodes).nodeNr
    } else {
        0 as c_int
    };
    hash = xmlHashCreate(l);
    i = 0 as c_int;
    loop {
        if !(i < l) {
            current_block = 8831408221741692167;
            break;
        }
        cur = if !nodes.is_null() && i >= 0 as c_int && i < (*nodes).nodeNr {
            *(*nodes).nodeTab.offset(i as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        };
        strval = xmlXPathCastNodeToString(cur);
        if xmlHashLookup(hash, strval).is_null() {
            if xmlHashAddEntry(hash, strval, strval as *mut c_void)
                < 0 as c_int
            {
                xmlFree.expect("non-null function pointer")(strval as *mut c_void);
                current_block = 14942300688322350028;
                break;
            } else if xmlXPathNodeSetAddUnique(ret, cur) < 0 as c_int {
                current_block = 14942300688322350028;
                break;
            }
        } else {
            xmlFree.expect("non-null function pointer")(strval as *mut c_void);
        }
        i += 1;
    }
    match current_block {
        14942300688322350028 => {
            xmlHashFree(
                hash,
                Some(
                    xmlHashDefaultDeallocator
                        as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
                ),
            );
            xmlXPathFreeNodeSet(ret);
            return ::core::ptr::null_mut::<xmlNodeSet>();
        }
        _ => {
            xmlHashFree(
                hash,
                Some(
                    xmlHashDefaultDeallocator
                        as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
                ),
            );
            return ret;
        }
    };
} }
#[inline]
pub fn xmlXPathDistinct(mut nodes: xmlNodeSetPtr) -> xmlNodeSetPtr { unsafe {
    if nodes.is_null() || (*nodes).nodeNr == 0 as c_int || (*nodes).nodeTab.is_null() {
        return nodes;
    }
    xmlXPathNodeSetSort(nodes);
    return xmlXPathDistinctSorted(nodes);
} }
#[inline]
pub fn xmlXPathHasSameNodes(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut l: c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if nodes1.is_null()
        || (*nodes1).nodeNr == 0 as c_int
        || (*nodes1).nodeTab.is_null()
        || (nodes2.is_null()
            || (*nodes2).nodeNr == 0 as c_int
            || (*nodes2).nodeTab.is_null())
    {
        return 0 as c_int;
    }
    l = if !nodes1.is_null() {
        (*nodes1).nodeNr
    } else {
        0 as c_int
    };
    i = 0 as c_int;
    while i < l {
        cur = if !nodes1.is_null() && i >= 0 as c_int && i < (*nodes1).nodeNr {
            *(*nodes1).nodeTab.offset(i as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        };
        if xmlXPathNodeSetContains(nodes2, cur) != 0 {
            return 1 as c_int;
        }
        i += 1;
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlXPathNodeLeadingSorted(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr { unsafe {
    let mut i: c_int = 0;
    let mut l: c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    if node.is_null() {
        return nodes;
    }
    ret = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
    if ret.is_null() {
        return ret;
    }
    if nodes.is_null()
        || (*nodes).nodeNr == 0 as c_int
        || (*nodes).nodeTab.is_null()
        || xmlXPathNodeSetContains(nodes, node) == 0
    {
        return ret;
    }
    l = if !nodes.is_null() {
        (*nodes).nodeNr
    } else {
        0 as c_int
    };
    i = 0 as c_int;
    while i < l {
        cur = if !nodes.is_null() && i >= 0 as c_int && i < (*nodes).nodeNr {
            *(*nodes).nodeTab.offset(i as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        };
        if cur == node {
            break;
        }
        if xmlXPathNodeSetAddUnique(ret, cur) < 0 as c_int {
            break;
        }
        i += 1;
    }
    return ret;
} }
#[inline]
pub fn xmlXPathNodeLeading(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr { {
    xmlXPathNodeSetSort(nodes);
    return xmlXPathNodeLeadingSorted(nodes, node);
} }
#[inline]
pub fn xmlXPathLeadingSorted(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    if nodes2.is_null()
        || (*nodes2).nodeNr == 0 as c_int
        || (*nodes2).nodeTab.is_null()
    {
        return nodes1;
    }
    return xmlXPathNodeLeadingSorted(
        nodes1,
        if !nodes2.is_null()
            && 1 as c_int >= 0 as c_int
            && (1 as c_int) < (*nodes2).nodeNr
        {
            *(*nodes2).nodeTab.offset(1 as c_int as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        },
    );
} }
#[inline]
pub fn xmlXPathLeading(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    if nodes2.is_null()
        || (*nodes2).nodeNr == 0 as c_int
        || (*nodes2).nodeTab.is_null()
    {
        return nodes1;
    }
    if nodes1.is_null()
        || (*nodes1).nodeNr == 0 as c_int
        || (*nodes1).nodeTab.is_null()
    {
        return xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
    }
    xmlXPathNodeSetSort(nodes1);
    xmlXPathNodeSetSort(nodes2);
    return xmlXPathNodeLeadingSorted(
        nodes1,
        if !nodes2.is_null()
            && 1 as c_int >= 0 as c_int
            && (1 as c_int) < (*nodes2).nodeNr
        {
            *(*nodes2).nodeTab.offset(1 as c_int as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        },
    );
} }
#[inline]
pub fn xmlXPathNodeTrailingSorted(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr { unsafe {
    let mut i: c_int = 0;
    let mut l: c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    if node.is_null() {
        return nodes;
    }
    ret = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
    if ret.is_null() {
        return ret;
    }
    if nodes.is_null()
        || (*nodes).nodeNr == 0 as c_int
        || (*nodes).nodeTab.is_null()
        || xmlXPathNodeSetContains(nodes, node) == 0
    {
        return ret;
    }
    l = if !nodes.is_null() {
        (*nodes).nodeNr
    } else {
        0 as c_int
    };
    i = l - 1 as c_int;
    while i >= 0 as c_int {
        cur = if !nodes.is_null() && i >= 0 as c_int && i < (*nodes).nodeNr {
            *(*nodes).nodeTab.offset(i as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        };
        if cur == node {
            break;
        }
        if xmlXPathNodeSetAddUnique(ret, cur) < 0 as c_int {
            break;
        }
        i -= 1;
    }
    xmlXPathNodeSetSort(ret);
    return ret;
} }
#[inline]
pub fn xmlXPathNodeTrailing(
    mut nodes: xmlNodeSetPtr,
    mut node: xmlNodePtr,
) -> xmlNodeSetPtr { {
    xmlXPathNodeSetSort(nodes);
    return xmlXPathNodeTrailingSorted(nodes, node);
} }
#[inline]
pub fn xmlXPathTrailingSorted(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    if nodes2.is_null()
        || (*nodes2).nodeNr == 0 as c_int
        || (*nodes2).nodeTab.is_null()
    {
        return nodes1;
    }
    return xmlXPathNodeTrailingSorted(
        nodes1,
        if !nodes2.is_null()
            && 0 as c_int >= 0 as c_int
            && (0 as c_int) < (*nodes2).nodeNr
        {
            *(*nodes2).nodeTab.offset(0 as c_int as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        },
    );
} }
#[inline]
pub fn xmlXPathTrailing(
    mut nodes1: xmlNodeSetPtr,
    mut nodes2: xmlNodeSetPtr,
) -> xmlNodeSetPtr { unsafe {
    if nodes2.is_null()
        || (*nodes2).nodeNr == 0 as c_int
        || (*nodes2).nodeTab.is_null()
    {
        return nodes1;
    }
    if nodes1.is_null()
        || (*nodes1).nodeNr == 0 as c_int
        || (*nodes1).nodeTab.is_null()
    {
        return xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
    }
    xmlXPathNodeSetSort(nodes1);
    xmlXPathNodeSetSort(nodes2);
    return xmlXPathNodeTrailingSorted(
        nodes1,
        if !nodes2.is_null()
            && 0 as c_int >= 0 as c_int
            && (0 as c_int) < (*nodes2).nodeNr
        {
            *(*nodes2).nodeTab.offset(0 as c_int as isize)
        } else {
            ::core::ptr::null_mut::<xmlNode>()
        },
    );
} }
#[inline]
pub unsafe fn xmlXPathRegisterFunc(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut f: xmlXPathFunction,
) -> c_int {
    return xmlXPathRegisterFuncNS(ctxt, name, ::core::ptr::null::<xmlChar>(), f);
}
#[inline]
pub unsafe fn xmlXPathRegisterFuncNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut f: xmlXPathFunction,
) -> c_int {
    if ctxt.is_null() {
        return -(1 as c_int);
    }
    if name.is_null() {
        return -(1 as c_int);
    }
    if (*ctxt).funcHash.is_null() {
        (*ctxt).funcHash = xmlHashCreate(0 as c_int);
    }
    if (*ctxt).funcHash.is_null() {
        return -(1 as c_int);
    }
    if f.is_none() {
        return xmlHashRemoveEntry2((*ctxt).funcHash, name, ns_uri, None);
    }
    return xmlHashAddEntry2(
        (*ctxt).funcHash,
        name,
        ns_uri,
        ::core::mem::transmute::<xmlXPathFunction, *mut c_void>(f),
    );
}
#[inline]
pub unsafe fn xmlXPathRegisterFuncLookup(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathFuncLookupFunc,
    mut funcCtxt: *mut c_void,
) {
    if ctxt.is_null() {
        return;
    }
    (*ctxt).funcLookupFunc = f;
    (*ctxt).funcLookupData = funcCtxt;
}
#[inline]
pub unsafe fn xmlXPathFunctionLookup(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathFunction {
    if ctxt.is_null() {
        return None;
    }
    if (*ctxt).funcLookupFunc.is_some() {
        let mut ret: xmlXPathFunction = None;
        let mut f: xmlXPathFuncLookupFunc = None;
        f = (*ctxt).funcLookupFunc;
        ret = f.expect("non-null function pointer")(
            (*ctxt).funcLookupData,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
        if ret.is_some() {
            return ret;
        }
    }
    return xmlXPathFunctionLookupNS(ctxt, name, ::core::ptr::null::<xmlChar>());
}
#[inline]
pub unsafe fn xmlXPathFunctionLookupNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathFunction {
    let mut ret: xmlXPathFunction = None;
    if ctxt.is_null() {
        return None;
    }
    if name.is_null() {
        return None;
    }
    if (*ctxt).funcLookupFunc.is_some() {
        let mut f: xmlXPathFuncLookupFunc = None;
        f = (*ctxt).funcLookupFunc;
        ret = f.expect("non-null function pointer")((*ctxt).funcLookupData, name, ns_uri);
        if ret.is_some() {
            return ret;
        }
    }
    if (*ctxt).funcHash.is_null() {
        return None;
    }
    ret = ::core::mem::transmute::<*mut c_void, xmlXPathFunction>(xmlHashLookup2(
        (*ctxt).funcHash,
        name,
        ns_uri,
    ));
    return ret;
}
#[inline]
pub fn xmlXPathRegisteredFuncsCleanup(mut ctxt: xmlXPathContextPtr) { unsafe {
    if ctxt.is_null() {
        return;
    }
    xmlHashFree((*ctxt).funcHash, None);
    (*ctxt).funcHash = ::core::ptr::null_mut::<xmlHashTable>();
} }
#[inline]
pub unsafe fn xmlXPathRegisterVariable(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> c_int {
    return xmlXPathRegisterVariableNS(ctxt, name, ::core::ptr::null::<xmlChar>(), value);
}
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterVariableNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
    mut value: xmlXPathObjectPtr,
) -> c_int {
    if ctxt.is_null() {
        return -(1 as c_int);
    }
    if name.is_null() {
        return -(1 as c_int);
    }
    if (*ctxt).varHash.is_null() {
        (*ctxt).varHash = xmlHashCreate(0 as c_int);
    }
    if (*ctxt).varHash.is_null() {
        return -(1 as c_int);
    }
    if value.is_null() {
        return xmlHashRemoveEntry2(
            (*ctxt).varHash,
            name,
            ns_uri,
            Some(
                xmlXPathFreeObjectEntry
                    as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
            ),
        );
    }
    return xmlHashUpdateEntry2(
        (*ctxt).varHash,
        name,
        ns_uri,
        value as *mut c_void,
        Some(
            xmlXPathFreeObjectEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
}
#[inline]
pub unsafe fn xmlXPathRegisterVariableLookup(
    mut ctxt: xmlXPathContextPtr,
    mut f: xmlXPathVariableLookupFunc,
    mut data: *mut c_void,
) {
    if ctxt.is_null() {
        return;
    }
    (*ctxt).varLookupFunc = f;
    (*ctxt).varLookupData = data;
}
#[inline]
pub unsafe fn xmlXPathVariableLookup(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
) -> xmlXPathObjectPtr {
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    if (*ctxt).varLookupFunc.is_some() {
        let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
        ret = (*ctxt).varLookupFunc.expect("non-null function pointer")(
            (*ctxt).varLookupData,
            name,
            ::core::ptr::null::<xmlChar>(),
        );
        return ret;
    }
    return xmlXPathVariableLookupNS(ctxt, name, ::core::ptr::null::<xmlChar>());
}
#[inline]
pub unsafe fn xmlXPathVariableLookupNS(
    mut ctxt: xmlXPathContextPtr,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathObjectPtr {
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    if (*ctxt).varLookupFunc.is_some() {
        let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
        ret = (*ctxt).varLookupFunc.expect("non-null function pointer")(
            (*ctxt).varLookupData,
            name,
            ns_uri,
        );
        if !ret.is_null() {
            return ret;
        }
    }
    if (*ctxt).varHash.is_null() {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    return xmlXPathCacheObjectCopy(
        ctxt,
        xmlHashLookup2((*ctxt).varHash, name, ns_uri) as xmlXPathObjectPtr,
    );
}
#[inline]
pub fn xmlXPathRegisteredVariablesCleanup(mut ctxt: xmlXPathContextPtr) { unsafe {
    if ctxt.is_null() {
        return;
    }
    xmlHashFree(
        (*ctxt).varHash,
        Some(
            xmlXPathFreeObjectEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
    (*ctxt).varHash = ::core::ptr::null_mut::<xmlHashTable>();
} }
#[no_mangle]
pub unsafe extern "C" fn xmlXPathRegisterNs(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> c_int {
    let mut copy: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if ctxt.is_null() {
        return -(1 as c_int);
    }
    if prefix.is_null() {
        return -(1 as c_int);
    }
    if *prefix.offset(0 as c_int as isize) as c_int
        == 0 as c_int
    {
        return -(1 as c_int);
    }
    if (*ctxt).nsHash.is_null() {
        (*ctxt).nsHash = xmlHashCreate(10 as c_int);
    }
    if (*ctxt).nsHash.is_null() {
        return -(1 as c_int);
    }
    if ns_uri.is_null() {
        return xmlHashRemoveEntry(
            (*ctxt).nsHash,
            prefix,
            Some(
                xmlHashDefaultDeallocator
                    as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
            ),
        );
    }
    copy = xmlStrdup(ns_uri);
    if copy.is_null() {
        return -(1 as c_int);
    }
    if xmlHashUpdateEntry(
        (*ctxt).nsHash,
        prefix,
        copy as *mut c_void,
        Some(
            xmlHashDefaultDeallocator
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    ) < 0 as c_int
    {
        xmlFree.expect("non-null function pointer")(copy as *mut c_void);
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlXPathNsLookup(
    mut ctxt: xmlXPathContextPtr,
    mut prefix: *const xmlChar,
) -> *const xmlChar {
    if ctxt.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if prefix.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if xmlStrEqual(
        prefix,
        b"xml\0" as *const u8 as *const c_char as *const xmlChar,
    ) != 0
    {
        return b"http://www.w3.org/XML/1998/namespace\0" as *const u8 as *const c_char
            as *const xmlChar;
    }
    if !(*ctxt).namespaces.is_null() {
        let mut i: c_int = 0;
        i = 0 as c_int;
        while i < (*ctxt).nsNr {
            if !(*(*ctxt).namespaces.offset(i as isize)).is_null()
                && xmlStrEqual((**(*ctxt).namespaces.offset(i as isize)).prefix, prefix) != 0
            {
                return (**(*ctxt).namespaces.offset(i as isize)).href;
            }
            i += 1;
        }
    }
    return xmlHashLookup((*ctxt).nsHash, prefix) as *const xmlChar;
}
#[inline]
pub fn xmlXPathRegisteredNsCleanup(mut ctxt: xmlXPathContextPtr) { unsafe {
    if ctxt.is_null() {
        return;
    }
    xmlHashFree(
        (*ctxt).nsHash,
        Some(
            xmlHashDefaultDeallocator
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
    (*ctxt).nsHash = ::core::ptr::null_mut::<xmlHashTable>();
} }
#[inline]
pub fn xmlXPathNewFloat(mut val: c_double) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating float object\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    (*ret).type_0 = XPATH_NUMBER;
    (*ret).floatval = val;
    return ret;
} }
#[inline]
pub fn xmlXPathNewBoolean(mut val: c_int) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating boolean object\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    (*ret).type_0 = XPATH_BOOLEAN;
    (*ret).boolval = (val != 0 as c_int) as c_int;
    return ret;
} }
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewString(mut val: *const xmlChar) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating string object\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    (*ret).type_0 = XPATH_STRING;
    if val.is_null() {
        val = b"\0" as *const u8 as *const c_char as *mut xmlChar;
    }
    (*ret).stringval = xmlStrdup(val);
    if (*ret).stringval.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    return ret;
}
#[inline]
pub unsafe fn xmlXPathWrapString(mut val: *mut xmlChar) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating string object\n\0" as *const u8 as *const c_char,
        );
        xmlFree.expect("non-null function pointer")(val as *mut c_void);
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    (*ret).type_0 = XPATH_STRING;
    (*ret).stringval = val;
    return ret;
}
#[inline]
pub unsafe fn xmlXPathNewCString(
    mut val: *const c_char,
) -> xmlXPathObjectPtr {
    return xmlXPathNewString(val as *mut xmlChar);
}
#[inline]
pub unsafe fn xmlXPathWrapCString(
    mut val: *mut c_char,
) -> xmlXPathObjectPtr {
    return xmlXPathWrapString(val as *mut xmlChar);
}
#[inline]
pub unsafe fn xmlXPathWrapExternal(
    mut val: *mut c_void,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating user object\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    (*ret).type_0 = XPATH_USERS;
    (*ret).user = val;
    return ret;
}
#[inline]
pub fn xmlXPathObjectCopy(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if val.is_null() {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathObject>() as size_t
    ) as xmlXPathObjectPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"copying object\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    memcpy(
        ret as *mut c_void,
        val as *const c_void,
        ::core::mem::size_of::<xmlXPathObject>() as size_t,
    );
    match (*val).type_0 as c_uint {
        4 => {
            (*ret).stringval = xmlStrdup((*val).stringval);
            if (*ret).stringval.is_null() {
                xmlFree.expect("non-null function pointer")(ret as *mut c_void);
                return ::core::ptr::null_mut::<xmlXPathObject>();
            }
        }
        9 | 1 => {
            (*ret).nodesetval =
                xmlXPathNodeSetMerge(::core::ptr::null_mut::<xmlNodeSet>(), (*val).nodesetval);
            (*ret).boolval = 0 as c_int;
        }
        8 => {
            (*ret).user = (*val).user;
        }
        0 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlXPathObjectCopy: unsupported type %d\n\0" as *const u8
                    as *const c_char,
                (*val).type_0 as c_uint,
            );
        }
        2 | 3 | _ => {}
    }
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlXPathFreeObject(mut obj: xmlXPathObjectPtr) { unsafe {
    if obj.is_null() {
        return;
    }
    if (*obj).type_0 as c_uint
        == XPATH_NODESET as c_int as c_uint
        || (*obj).type_0 as c_uint
            == XPATH_XSLT_TREE as c_int as c_uint
    {
        if (*obj).boolval != 0 {
            (*obj).type_0 = XPATH_XSLT_TREE;
            if !(*obj).nodesetval.is_null() {
                xmlXPathFreeValueTree((*obj).nodesetval);
            }
        } else if !(*obj).nodesetval.is_null() {
            xmlXPathFreeNodeSet((*obj).nodesetval);
        }
    } else if (*obj).type_0 as c_uint
        == XPATH_STRING as c_int as c_uint
    {
        if !(*obj).stringval.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*obj).stringval as *mut c_void,
            );
        }
    }
    xmlFree.expect("non-null function pointer")(obj as *mut c_void);
} }
unsafe extern "C" fn xmlXPathFreeObjectEntry(
    mut obj: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlXPathFreeObject(obj as xmlXPathObjectPtr);
}
fn xmlXPathReleaseObject(
    mut ctxt: xmlXPathContextPtr,
    mut obj: xmlXPathObjectPtr,
) { unsafe {
    let mut current_block: u64;
    if obj.is_null() {
        return;
    }
    if ctxt.is_null() || (*ctxt).cache.is_null() {
        xmlXPathFreeObject(obj);
    } else {
        let mut cache: xmlXPathContextCachePtr = (*ctxt).cache as xmlXPathContextCachePtr;
        match (*obj).type_0 as c_uint {
            1 | 9 => {
                if !(*obj).nodesetval.is_null() {
                    if (*obj).boolval != 0 {
                        (*obj).type_0 = XPATH_XSLT_TREE;
                        xmlXPathFreeValueTree((*obj).nodesetval);
                        (*obj).nodesetval = ::core::ptr::null_mut::<xmlNodeSet>();
                        current_block = 12199444798915819164;
                    } else if (*(*obj).nodesetval).nodeMax <= 40 as c_int
                        && ((*cache).nodesetObjs.is_null()
                            || (*(*cache).nodesetObjs).number < (*cache).maxNodeset)
                    {
                        if (*cache).nodesetObjs.is_null() {
                            (*cache).nodesetObjs = xmlPointerListCreate(10 as c_int);
                            if (*cache).nodesetObjs.is_null() {
                                current_block = 7689215457434231438;
                            } else {
                                current_block = 7651349459974463963;
                            }
                        } else {
                            current_block = 7651349459974463963;
                        }
                        match current_block {
                            7689215457434231438 => {}
                            _ => {
                                if xmlPointerListAddSize(
                                    (*cache).nodesetObjs,
                                    obj as *mut c_void,
                                    0 as c_int,
                                ) == -(1 as c_int)
                                {
                                    current_block = 7689215457434231438;
                                } else {
                                    current_block = 17307684059818097895;
                                }
                            }
                        }
                    } else {
                        xmlXPathFreeNodeSet((*obj).nodesetval);
                        (*obj).nodesetval = ::core::ptr::null_mut::<xmlNodeSet>();
                        current_block = 12199444798915819164;
                    }
                } else {
                    current_block = 12199444798915819164;
                }
            }
            4 => {
                if !(*obj).stringval.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*obj).stringval as *mut c_void,
                    );
                }
                if (*cache).stringObjs.is_null()
                    || (*(*cache).stringObjs).number < (*cache).maxString
                {
                    if (*cache).stringObjs.is_null() {
                        (*cache).stringObjs = xmlPointerListCreate(10 as c_int);
                        if (*cache).stringObjs.is_null() {
                            current_block = 7689215457434231438;
                        } else {
                            current_block = 13472856163611868459;
                        }
                    } else {
                        current_block = 13472856163611868459;
                    }
                    match current_block {
                        7689215457434231438 => {}
                        _ => {
                            if xmlPointerListAddSize(
                                (*cache).stringObjs,
                                obj as *mut c_void,
                                0 as c_int,
                            ) == -(1 as c_int)
                            {
                                current_block = 7689215457434231438;
                            } else {
                                current_block = 17307684059818097895;
                            }
                        }
                    }
                } else {
                    current_block = 12199444798915819164;
                }
            }
            2 => {
                if (*cache).booleanObjs.is_null()
                    || (*(*cache).booleanObjs).number < (*cache).maxBoolean
                {
                    if (*cache).booleanObjs.is_null() {
                        (*cache).booleanObjs = xmlPointerListCreate(10 as c_int);
                        if (*cache).booleanObjs.is_null() {
                            current_block = 7689215457434231438;
                        } else {
                            current_block = 11459959175219260272;
                        }
                    } else {
                        current_block = 11459959175219260272;
                    }
                    match current_block {
                        7689215457434231438 => {}
                        _ => {
                            if xmlPointerListAddSize(
                                (*cache).booleanObjs,
                                obj as *mut c_void,
                                0 as c_int,
                            ) == -(1 as c_int)
                            {
                                current_block = 7689215457434231438;
                            } else {
                                current_block = 17307684059818097895;
                            }
                        }
                    }
                } else {
                    current_block = 12199444798915819164;
                }
            }
            3 => {
                if (*cache).numberObjs.is_null()
                    || (*(*cache).numberObjs).number < (*cache).maxNumber
                {
                    if (*cache).numberObjs.is_null() {
                        (*cache).numberObjs = xmlPointerListCreate(10 as c_int);
                        if (*cache).numberObjs.is_null() {
                            current_block = 7689215457434231438;
                        } else {
                            current_block = 15597372965620363352;
                        }
                    } else {
                        current_block = 15597372965620363352;
                    }
                    match current_block {
                        7689215457434231438 => {}
                        _ => {
                            if xmlPointerListAddSize(
                                (*cache).numberObjs,
                                obj as *mut c_void,
                                0 as c_int,
                            ) == -(1 as c_int)
                            {
                                current_block = 7689215457434231438;
                            } else {
                                current_block = 17307684059818097895;
                            }
                        }
                    }
                } else {
                    current_block = 12199444798915819164;
                }
            }
            _ => {
                current_block = 7689215457434231438;
            }
        }
        match current_block {
            12199444798915819164 => {
                if (*cache).miscObjs.is_null() || (*(*cache).miscObjs).number < (*cache).maxMisc {
                    if (*cache).miscObjs.is_null() {
                        (*cache).miscObjs = xmlPointerListCreate(10 as c_int);
                        if (*cache).miscObjs.is_null() {
                            current_block = 7689215457434231438;
                        } else {
                            current_block = 6717214610478484138;
                        }
                    } else {
                        current_block = 6717214610478484138;
                    }
                    match current_block {
                        7689215457434231438 => {}
                        _ => {
                            if xmlPointerListAddSize(
                                (*cache).miscObjs,
                                obj as *mut c_void,
                                0 as c_int,
                            ) == -(1 as c_int)
                            {
                                current_block = 7689215457434231438;
                            } else {
                                current_block = 17307684059818097895;
                            }
                        }
                    }
                } else {
                    current_block = 7689215457434231438;
                }
            }
            _ => {}
        }
        match current_block {
            7689215457434231438 => {
                if !(*obj).nodesetval.is_null() {
                    xmlXPathFreeNodeSet((*obj).nodesetval);
                }
                xmlFree.expect("non-null function pointer")(obj as *mut c_void);
            }
            _ => {
                if !(*obj).nodesetval.is_null() {
                    let mut tmpset: xmlNodeSetPtr = (*obj).nodesetval;
                    if (*tmpset).nodeNr > 1 as c_int {
                        let mut i: c_int = 0;
                        let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                        i = 0 as c_int;
                        while i < (*tmpset).nodeNr {
                            node = *(*tmpset).nodeTab.offset(i as isize);
                            if !node.is_null()
                                && (*node).type_0 as c_uint
                                    == XML_NAMESPACE_DECL as c_int
                                        as c_uint
                            {
                                xmlXPathNodeSetFreeNs(node as xmlNsPtr);
                            }
                            i += 1;
                        }
                    } else if (*tmpset).nodeNr == 1 as c_int {
                        if !(*(*tmpset).nodeTab.offset(0 as c_int as isize)).is_null()
                            && (**(*tmpset).nodeTab.offset(0 as c_int as isize)).type_0
                                as c_uint
                                == XML_NAMESPACE_DECL as c_int as c_uint
                        {
                            xmlXPathNodeSetFreeNs(
                                *(*tmpset).nodeTab.offset(0 as c_int as isize)
                                    as xmlNsPtr,
                            );
                        }
                    }
                    (*tmpset).nodeNr = 0 as c_int;
                    memset(
                        obj as *mut c_void,
                        0 as c_int,
                        ::core::mem::size_of::<xmlXPathObject>() as size_t,
                    );
                    (*obj).nodesetval = tmpset;
                } else {
                    memset(
                        obj as *mut c_void,
                        0 as c_int,
                        ::core::mem::size_of::<xmlXPathObject>() as size_t,
                    );
                }
                return;
            }
        }
    };
} }
#[inline]
pub fn xmlXPathCastBooleanToString(mut val: c_int) -> *mut xmlChar { unsafe {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if val != 0 {
        ret = xmlStrdup(b"true\0" as *const u8 as *const c_char as *const xmlChar);
    } else {
        ret = xmlStrdup(b"false\0" as *const u8 as *const c_char as *const xmlChar);
    }
    return ret;
} }
#[inline]
pub fn xmlXPathCastNumberToString(
    mut val: c_double,
) -> *mut xmlChar { unsafe {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    match xmlXPathIsInf(val) {
        1 => {
            ret = xmlStrdup(
                b"Infinity\0" as *const u8 as *const c_char as *const xmlChar,
            );
        }
        -1 => {
            ret = xmlStrdup(
                b"-Infinity\0" as *const u8 as *const c_char as *const xmlChar,
            );
        }
        _ => {
            if xmlXPathIsNaN(val) != 0 {
                ret = xmlStrdup(
                    b"NaN\0" as *const u8 as *const c_char as *const xmlChar,
                );
            } else if val == 0 as c_int as c_double {
                ret =
                    xmlStrdup(b"0\0" as *const u8 as *const c_char as *const xmlChar);
            } else {
                let mut buf: [c_char; 100] = [0; 100];
                xmlXPathFormatNumber(
                    val,
                    &raw mut buf as *mut c_char,
                    99 as c_int,
                );
                buf[99 as c_int as usize] = 0 as c_char;
                ret = xmlStrdup(&raw mut buf as *mut c_char as *const xmlChar);
            }
        }
    }
    return ret;
} }
#[inline]
pub fn xmlXPathCastNodeToString(mut node: xmlNodePtr) -> *mut xmlChar { unsafe {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    ret = xmlNodeGetContent(node as *const xmlNode);
    if ret.is_null() {
        ret = xmlStrdup(b"\0" as *const u8 as *const c_char as *const xmlChar);
    }
    return ret;
} }
#[inline]
pub fn xmlXPathCastNodeSetToString(mut ns: xmlNodeSetPtr) -> *mut xmlChar { unsafe {
    if ns.is_null() || (*ns).nodeNr == 0 as c_int || (*ns).nodeTab.is_null() {
        return xmlStrdup(b"\0" as *const u8 as *const c_char as *const xmlChar);
    }
    if (*ns).nodeNr > 1 as c_int {
        xmlXPathNodeSetSort(ns);
    }
    return xmlXPathCastNodeToString(*(*ns).nodeTab.offset(0 as c_int as isize));
} }
#[inline]
pub fn xmlXPathCastToString(mut val: xmlXPathObjectPtr) -> *mut xmlChar { unsafe {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if val.is_null() {
        return xmlStrdup(b"\0" as *const u8 as *const c_char as *const xmlChar);
    }
    match (*val).type_0 as c_uint {
        0 => {
            ret = xmlStrdup(b"\0" as *const u8 as *const c_char as *const xmlChar);
        }
        1 | 9 => {
            ret = xmlXPathCastNodeSetToString((*val).nodesetval);
        }
        4 => return xmlStrdup((*val).stringval),
        2 => {
            ret = xmlXPathCastBooleanToString((*val).boolval);
        }
        3 => {
            ret = xmlXPathCastNumberToString((*val).floatval);
        }
        8 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                    as *const c_char,
                5117 as c_int,
            );
            ret = xmlStrdup(b"\0" as *const u8 as *const c_char as *const xmlChar);
        }
        _ => {}
    }
    return ret;
} }
#[inline]
pub fn xmlXPathConvertString(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr { unsafe {
    let mut res: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if val.is_null() {
        return xmlXPathNewCString(b"\0" as *const u8 as *const c_char);
    }
    match (*val).type_0 as c_uint {
        1 | 9 => {
            res = xmlXPathCastNodeSetToString((*val).nodesetval);
        }
        4 => return val,
        2 => {
            res = xmlXPathCastBooleanToString((*val).boolval);
        }
        3 => {
            res = xmlXPathCastNumberToString((*val).floatval);
        }
        8 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                    as *const c_char,
                5161 as c_int,
            );
        }
        0 | _ => {}
    }
    xmlXPathFreeObject(val);
    if res.is_null() {
        return xmlXPathNewCString(b"\0" as *const u8 as *const c_char);
    }
    return xmlXPathWrapString(res);
} }
#[inline]
pub fn xmlXPathCastBooleanToNumber(
    mut val: c_int,
) -> c_double { {
    if val != 0 {
        return 1.0f64;
    }
    return 0.0f64;
} }
#[inline]
pub unsafe fn xmlXPathCastStringToNumber(
    mut val: *const xmlChar,
) -> c_double {
    return xmlXPathStringEvalNumber(val);
}
#[inline]
pub fn xmlXPathCastNodeToNumber(mut node: xmlNodePtr) -> c_double { unsafe {
    let mut strval: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: c_double = 0.;
    if node.is_null() {
        return xmlXPathNAN;
    }
    strval = xmlXPathCastNodeToString(node);
    if strval.is_null() {
        return xmlXPathNAN;
    }
    ret = xmlXPathCastStringToNumber(strval);
    xmlFree.expect("non-null function pointer")(strval as *mut c_void);
    return ret;
} }
#[inline]
pub fn xmlXPathCastNodeSetToNumber(
    mut ns: xmlNodeSetPtr,
) -> c_double { unsafe {
    let mut str: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: c_double = 0.;
    if ns.is_null() {
        return xmlXPathNAN;
    }
    str = xmlXPathCastNodeSetToString(ns);
    ret = xmlXPathCastStringToNumber(str);
    xmlFree.expect("non-null function pointer")(str as *mut c_void);
    return ret;
} }
#[inline]
pub fn xmlXPathCastToNumber(mut val: xmlXPathObjectPtr) -> c_double { unsafe {
    let mut ret: c_double = 0.0f64;
    if val.is_null() {
        return xmlXPathNAN;
    }
    match (*val).type_0 as c_uint {
        0 => {
            ret = xmlXPathNAN;
        }
        1 | 9 => {
            ret = xmlXPathCastNodeSetToNumber((*val).nodesetval);
        }
        4 => {
            ret = xmlXPathCastStringToNumber((*val).stringval);
        }
        3 => {
            ret = (*val).floatval;
        }
        2 => {
            ret = xmlXPathCastBooleanToNumber((*val).boolval);
        }
        8 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                    as *const c_char,
                5280 as c_int,
            );
            ret = xmlXPathNAN;
        }
        _ => {}
    }
    return ret;
} }
#[inline]
pub fn xmlXPathConvertNumber(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if val.is_null() {
        return xmlXPathNewFloat(0.0f64);
    }
    if (*val).type_0 as c_uint
        == XPATH_NUMBER as c_int as c_uint
    {
        return val;
    }
    ret = xmlXPathNewFloat(xmlXPathCastToNumber(val));
    xmlXPathFreeObject(val);
    return ret;
} }
#[inline]
pub fn xmlXPathCastNumberToBoolean(
    mut val: c_double,
) -> c_int { {
    if xmlXPathIsNaN(val) != 0 || val == 0.0f64 {
        return 0 as c_int;
    }
    return 1 as c_int;
} }
#[inline]
pub unsafe fn xmlXPathCastStringToBoolean(
    mut val: *const xmlChar,
) -> c_int {
    if val.is_null() || xmlStrlen(val) == 0 as c_int {
        return 0 as c_int;
    }
    return 1 as c_int;
}
#[inline]
pub fn xmlXPathCastNodeSetToBoolean(mut ns: xmlNodeSetPtr) -> c_int { unsafe {
    if ns.is_null() || (*ns).nodeNr == 0 as c_int {
        return 0 as c_int;
    }
    return 1 as c_int;
} }
#[inline]
pub fn xmlXPathCastToBoolean(mut val: xmlXPathObjectPtr) -> c_int { unsafe {
    let mut ret: c_int = 0 as c_int;
    if val.is_null() {
        return 0 as c_int;
    }
    match (*val).type_0 as c_uint {
        0 => {
            ret = 0 as c_int;
        }
        1 | 9 => {
            ret = xmlXPathCastNodeSetToBoolean((*val).nodesetval);
        }
        4 => {
            ret = xmlXPathCastStringToBoolean((*val).stringval);
        }
        3 => {
            ret = xmlXPathCastNumberToBoolean((*val).floatval);
        }
        2 => {
            ret = (*val).boolval;
        }
        8 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                    as *const c_char,
                5391 as c_int,
            );
            ret = 0 as c_int;
        }
        _ => {}
    }
    return ret;
} }
#[inline]
pub fn xmlXPathConvertBoolean(mut val: xmlXPathObjectPtr) -> xmlXPathObjectPtr { unsafe {
    let mut ret: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if val.is_null() {
        return xmlXPathNewBoolean(0 as c_int);
    }
    if (*val).type_0 as c_uint
        == XPATH_BOOLEAN as c_int as c_uint
    {
        return val;
    }
    ret = xmlXPathNewBoolean(xmlXPathCastToBoolean(val));
    xmlXPathFreeObject(val);
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlXPathNewContext(mut doc: xmlDocPtr) -> xmlXPathContextPtr { unsafe {
    let mut ret: xmlXPathContextPtr = ::core::ptr::null_mut::<xmlXPathContext>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathContext>() as size_t
    ) as xmlXPathContextPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"creating context\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathContext>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathContext>() as size_t,
    );
    (*ret).doc = doc;
    (*ret).node = ::core::ptr::null_mut::<xmlNode>();
    (*ret).varHash = ::core::ptr::null_mut::<xmlHashTable>();
    (*ret).nb_types = 0 as c_int;
    (*ret).max_types = 0 as c_int;
    (*ret).types = ::core::ptr::null_mut::<xmlXPathType>();
    (*ret).funcHash = xmlHashCreate(0 as c_int);
    (*ret).nb_axis = 0 as c_int;
    (*ret).max_axis = 0 as c_int;
    (*ret).axis = ::core::ptr::null_mut::<xmlXPathAxis>();
    (*ret).nsHash = ::core::ptr::null_mut::<xmlHashTable>();
    (*ret).user = NULL;
    (*ret).contextSize = -(1 as c_int);
    (*ret).proximityPosition = -(1 as c_int);
    xmlXPathRegisterAllFunctions(ret);
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlXPathFreeContext(mut ctxt: xmlXPathContextPtr) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if !(*ctxt).cache.is_null() {
        xmlXPathFreeCache((*ctxt).cache as xmlXPathContextCachePtr);
    }
    xmlXPathRegisteredNsCleanup(ctxt);
    xmlXPathRegisteredFuncsCleanup(ctxt);
    xmlXPathRegisteredVariablesCleanup(ctxt);
    xmlResetError(&raw mut (*ctxt).lastError);
    xmlFree.expect("non-null function pointer")(ctxt as *mut c_void);
} }
#[no_mangle]
pub unsafe extern "C" fn xmlXPathNewParserContext(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr {
    let mut ret: xmlXPathParserContextPtr = ::core::ptr::null_mut::<xmlXPathParserContext>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathParserContext>() as size_t,
    ) as xmlXPathParserContextPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ctxt,
            b"creating parser context\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathParserContext>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathParserContext>() as size_t,
    );
    (*ret).base = str;
    (*ret).cur = (*ret).base;
    (*ret).context = ctxt;
    (*ret).comp = xmlXPathNewCompExpr();
    if (*ret).comp.is_null() {
        xmlFree.expect("non-null function pointer")((*ret).valueTab as *mut c_void);
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
        return ::core::ptr::null_mut::<xmlXPathParserContext>();
    }
    if !ctxt.is_null() && !(*ctxt).dict.is_null() {
        (*(*ret).comp).dict = (*ctxt).dict;
        xmlDictReference((*(*ret).comp).dict);
    }
    return ret;
}
fn xmlXPathCompParserContext(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathParserContextPtr { unsafe {
    let mut ret: xmlXPathParserContextPtr = ::core::ptr::null_mut::<xmlXPathParserContext>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlXPathParserContext>() as size_t,
    ) as xmlXPathParserContextPtr;
    if ret.is_null() {
        xmlXPathErrMemory(
            ctxt,
            b"creating evaluation context\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathParserContext>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlXPathParserContext>() as size_t,
    );
    (*ret).valueTab = xmlMalloc.expect("non-null function pointer")(
        (10 as size_t).wrapping_mul(::core::mem::size_of::<xmlXPathObjectPtr>() as size_t),
    ) as *mut xmlXPathObjectPtr;
    if (*ret).valueTab.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
        xmlXPathErrMemory(
            ctxt,
            b"creating evaluation context\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathParserContext>();
    }
    (*ret).valueNr = 0 as c_int;
    (*ret).valueMax = 10 as c_int;
    (*ret).value = ::core::ptr::null_mut::<xmlXPathObject>();
    (*ret).context = ctxt;
    (*ret).comp = comp;
    return ret;
} }
#[no_mangle]
pub extern "C" fn xmlXPathFreeParserContext(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut i: c_int = 0;
    if !(*ctxt).valueTab.is_null() {
        i = 0 as c_int;
        while i < (*ctxt).valueNr {
            if !(*ctxt).context.is_null() {
                xmlXPathReleaseObject((*ctxt).context, *(*ctxt).valueTab.offset(i as isize));
            } else {
                xmlXPathFreeObject(*(*ctxt).valueTab.offset(i as isize));
            }
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*ctxt).valueTab as *mut c_void);
    }
    if !(*ctxt).comp.is_null() {
        if !(*(*ctxt).comp).stream.is_null() {
            xmlFreePatternList((*(*ctxt).comp).stream);
            (*(*ctxt).comp).stream = ::core::ptr::null_mut::<xmlPattern>();
        }
        xmlXPathFreeCompExpr((*ctxt).comp);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut c_void);
} }
fn xmlXPathNodeValHash(mut node: xmlNodePtr) -> c_uint { unsafe {
    let mut len: c_int = 2 as c_int;
    let mut string: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: c_uint = 0 as c_uint;
    if node.is_null() {
        return 0 as c_uint;
    }
    if (*node).type_0 as c_uint
        == XML_DOCUMENT_NODE as c_int as c_uint
    {
        tmp = xmlDocGetRootElement(node as xmlDocPtr as *const xmlDoc);
        if tmp.is_null() {
            node = (*node).children as xmlNodePtr;
        } else {
            node = tmp;
        }
        if node.is_null() {
            return 0 as c_uint;
        }
    }
    match (*node).type_0 as c_uint {
        8 | 7 | 4 | 3 => {
            string = (*node).content;
            if string.is_null() {
                return 0 as c_uint;
            }
            if *string.offset(0 as c_int as isize) as c_int
                == 0 as c_int
            {
                return 0 as c_uint;
            }
            return (*string.offset(0 as c_int as isize) as c_int
                + ((*string.offset(1 as c_int as isize) as c_int)
                    << 8 as c_int)) as c_uint;
        }
        18 => {
            string = (*(node as xmlNsPtr)).href;
            if string.is_null() {
                return 0 as c_uint;
            }
            if *string.offset(0 as c_int as isize) as c_int
                == 0 as c_int
            {
                return 0 as c_uint;
            }
            return (*string.offset(0 as c_int as isize) as c_int
                + ((*string.offset(1 as c_int as isize) as c_int)
                    << 8 as c_int)) as c_uint;
        }
        2 => {
            tmp = (*(node as xmlAttrPtr)).children as xmlNodePtr;
        }
        1 => {
            tmp = (*node).children as xmlNodePtr;
        }
        _ => return 0 as c_uint,
    }
    while !tmp.is_null() {
        match (*tmp).type_0 as c_uint {
            4 | 3 => {
                string = (*tmp).content;
            }
            _ => {
                string = ::core::ptr::null::<xmlChar>();
            }
        }
        if !string.is_null()
            && *string.offset(0 as c_int as isize) as c_int
                != 0 as c_int
        {
            if len == 1 as c_int {
                return ret.wrapping_add(
                    ((*string.offset(0 as c_int as isize) as c_int)
                        << 8 as c_int) as c_uint,
                );
            }
            if *string.offset(1 as c_int as isize) as c_int
                == 0 as c_int
            {
                len = 1 as c_int;
                ret = *string.offset(0 as c_int as isize) as c_uint;
            } else {
                return (*string.offset(0 as c_int as isize) as c_int
                    + ((*string.offset(1 as c_int as isize) as c_int)
                        << 8 as c_int)) as c_uint;
            }
        }
        if !(*tmp).children.is_null()
            && (*tmp).type_0 as c_uint
                != XML_DTD_NODE as c_int as c_uint
            && (*tmp).type_0 as c_uint
                != XML_ENTITY_REF_NODE as c_int as c_uint
            && (*(*tmp).children).type_0 as c_uint
                != XML_ENTITY_DECL as c_int as c_uint
        {
            tmp = (*tmp).children as xmlNodePtr;
        } else {
            if tmp == node {
                break;
            }
            if !(*tmp).next.is_null() {
                tmp = (*tmp).next as xmlNodePtr;
            } else {
                loop {
                    tmp = (*tmp).parent as xmlNodePtr;
                    if tmp.is_null() {
                        break;
                    }
                    if tmp == node {
                        tmp = ::core::ptr::null_mut::<xmlNode>();
                        break;
                    } else if !(*tmp).next.is_null() {
                        tmp = (*tmp).next as xmlNodePtr;
                        break;
                    } else if tmp.is_null() {
                        break;
                    }
                }
            }
        }
    }
    return ret;
} }
unsafe fn xmlXPathStringHash(mut string: *const xmlChar) -> c_uint {
    if string.is_null() {
        return 0 as c_uint;
    }
    if *string.offset(0 as c_int as isize) as c_int
        == 0 as c_int
    {
        return 0 as c_uint;
    }
    return (*string.offset(0 as c_int as isize) as c_int
        + ((*string.offset(1 as c_int as isize) as c_int)
            << 8 as c_int)) as c_uint;
}
fn xmlXPathCompareNodeSetFloat(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: c_int,
    mut strict: c_int,
    mut arg: xmlXPathObjectPtr,
    mut f: xmlXPathObjectPtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut ret: c_int = 0 as c_int;
    let mut ns: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut str2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if f.is_null()
        || arg.is_null()
        || (*arg).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, arg);
        xmlXPathReleaseObject((*ctxt).context, f);
        return 0 as c_int;
    }
    ns = (*arg).nodesetval;
    if !ns.is_null() {
        i = 0 as c_int;
        while i < (*ns).nodeNr {
            str2 = xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize));
            if !str2.is_null() {
                valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                xmlFree.expect("non-null function pointer")(str2 as *mut c_void);
                xmlXPathNumberFunction(ctxt, 1 as c_int);
                valuePush(ctxt, xmlXPathCacheObjectCopy((*ctxt).context, f));
                ret = xmlXPathCompareValues(ctxt, inf, strict);
                if ret != 0 {
                    break;
                }
            }
            i += 1;
        }
    }
    xmlXPathReleaseObject((*ctxt).context, arg);
    xmlXPathReleaseObject((*ctxt).context, f);
    return ret;
} }
fn xmlXPathCompareNodeSetString(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: c_int,
    mut strict: c_int,
    mut arg: xmlXPathObjectPtr,
    mut s: xmlXPathObjectPtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut ret: c_int = 0 as c_int;
    let mut ns: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut str2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if s.is_null()
        || arg.is_null()
        || (*arg).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, arg);
        xmlXPathReleaseObject((*ctxt).context, s);
        return 0 as c_int;
    }
    ns = (*arg).nodesetval;
    if !ns.is_null() {
        i = 0 as c_int;
        while i < (*ns).nodeNr {
            str2 = xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize));
            if !str2.is_null() {
                valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                xmlFree.expect("non-null function pointer")(str2 as *mut c_void);
                valuePush(ctxt, xmlXPathCacheObjectCopy((*ctxt).context, s));
                ret = xmlXPathCompareValues(ctxt, inf, strict);
                if ret != 0 {
                    break;
                }
            }
            i += 1;
        }
    }
    xmlXPathReleaseObject((*ctxt).context, arg);
    xmlXPathReleaseObject((*ctxt).context, s);
    return ret;
} }
fn xmlXPathCompareNodeSets(
    mut inf: c_int,
    mut strict: c_int,
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut init: c_int = 0 as c_int;
    let mut val1: c_double = 0.;
    let mut values2: *mut c_double = ::core::ptr::null_mut::<c_double>();
    let mut ret: c_int = 0 as c_int;
    let mut ns1: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut ns2: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    if arg1.is_null()
        || (*arg1).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg1).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathFreeObject(arg2);
        return 0 as c_int;
    }
    if arg2.is_null()
        || (*arg2).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg2).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        return 0 as c_int;
    }
    ns1 = (*arg1).nodesetval;
    ns2 = (*arg2).nodesetval;
    if ns1.is_null() || (*ns1).nodeNr <= 0 as c_int {
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        return 0 as c_int;
    }
    if ns2.is_null() || (*ns2).nodeNr <= 0 as c_int {
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        return 0 as c_int;
    }
    values2 = xmlMalloc.expect("non-null function pointer")(
        ((*ns2).nodeNr as size_t)
            .wrapping_mul(::core::mem::size_of::<c_double>() as size_t),
    ) as *mut c_double;
    if values2.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"comparing nodesets\n\0" as *const u8 as *const c_char,
        );
        xmlXPathFreeObject(arg1);
        xmlXPathFreeObject(arg2);
        return 0 as c_int;
    }
    i = 0 as c_int;
    while i < (*ns1).nodeNr {
        val1 = xmlXPathCastNodeToNumber(*(*ns1).nodeTab.offset(i as isize));
        if !(xmlXPathIsNaN(val1) != 0) {
            j = 0 as c_int;
            while j < (*ns2).nodeNr {
                if init == 0 as c_int {
                    *values2.offset(j as isize) =
                        xmlXPathCastNodeToNumber(*(*ns2).nodeTab.offset(j as isize));
                }
                if !(xmlXPathIsNaN(*values2.offset(j as isize)) != 0) {
                    if inf != 0 && strict != 0 {
                        ret = (val1 < *values2.offset(j as isize)) as c_int;
                    } else if inf != 0 && strict == 0 {
                        ret = (val1 <= *values2.offset(j as isize)) as c_int;
                    } else if inf == 0 && strict != 0 {
                        ret = (val1 > *values2.offset(j as isize)) as c_int;
                    } else if inf == 0 && strict == 0 {
                        ret = (val1 >= *values2.offset(j as isize)) as c_int;
                    }
                    if ret != 0 {
                        break;
                    }
                }
                j += 1;
            }
            if ret != 0 {
                break;
            }
            init = 1 as c_int;
        }
        i += 1;
    }
    xmlFree.expect("non-null function pointer")(values2 as *mut c_void);
    xmlXPathFreeObject(arg1);
    xmlXPathFreeObject(arg2);
    return ret;
} }
fn xmlXPathCompareNodeSetValue(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: c_int,
    mut strict: c_int,
    mut arg: xmlXPathObjectPtr,
    mut val: xmlXPathObjectPtr,
) -> c_int { unsafe {
    if val.is_null()
        || arg.is_null()
        || (*arg).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        return 0 as c_int;
    }
    match (*val).type_0 as c_uint {
        3 => return xmlXPathCompareNodeSetFloat(ctxt, inf, strict, arg, val),
        1 | 9 => return xmlXPathCompareNodeSets(inf, strict, arg, val),
        4 => return xmlXPathCompareNodeSetString(ctxt, inf, strict, arg, val),
        2 => {
            valuePush(ctxt, arg);
            xmlXPathBooleanFunction(ctxt, 1 as c_int);
            valuePush(ctxt, val);
            return xmlXPathCompareValues(ctxt, inf, strict);
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompareNodeSetValue: Can't compare node set and object of type %d\n\0"
                    as *const u8 as *const c_char,
                (*val).type_0 as c_uint,
            );
            xmlXPathReleaseObject((*ctxt).context, arg);
            xmlXPathReleaseObject((*ctxt).context, val);
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
            return 0 as c_int;
        }
    };
} }
unsafe fn xmlXPathEqualNodeSetString(
    mut arg: xmlXPathObjectPtr,
    mut str: *const xmlChar,
    mut neq: c_int,
) -> c_int {
    let mut i: c_int = 0;
    let mut ns: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut str2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut hash: c_uint = 0;
    if str.is_null()
        || arg.is_null()
        || (*arg).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        return 0 as c_int;
    }
    ns = (*arg).nodesetval;
    if ns.is_null() || (*ns).nodeNr <= 0 as c_int {
        return 0 as c_int;
    }
    hash = xmlXPathStringHash(str);
    i = 0 as c_int;
    while i < (*ns).nodeNr {
        if xmlXPathNodeValHash(*(*ns).nodeTab.offset(i as isize)) == hash {
            str2 = xmlNodeGetContent(*(*ns).nodeTab.offset(i as isize) as *const xmlNode);
            if !str2.is_null() && xmlStrEqual(str, str2) != 0 {
                xmlFree.expect("non-null function pointer")(str2 as *mut c_void);
                if !(neq != 0) {
                    return 1 as c_int;
                }
            } else if str2.is_null()
                && xmlStrEqual(
                    str,
                    b"\0" as *const u8 as *const c_char as *mut xmlChar,
                ) != 0
            {
                if !(neq != 0) {
                    return 1 as c_int;
                }
            } else {
                if neq != 0 {
                    if !str2.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            str2 as *mut c_void,
                        );
                    }
                    return 1 as c_int;
                }
                if !str2.is_null() {
                    xmlFree.expect("non-null function pointer")(str2 as *mut c_void);
                }
            }
        } else if neq != 0 {
            return 1 as c_int;
        }
        i += 1;
    }
    return 0 as c_int;
}
fn xmlXPathEqualNodeSetFloat(
    mut ctxt: xmlXPathParserContextPtr,
    mut arg: xmlXPathObjectPtr,
    mut f: c_double,
    mut neq: c_int,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut ret: c_int = 0 as c_int;
    let mut ns: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut str2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut val: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut v: c_double = 0.;
    if arg.is_null()
        || (*arg).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        return 0 as c_int;
    }
    ns = (*arg).nodesetval;
    if !ns.is_null() {
        i = 0 as c_int;
        while i < (*ns).nodeNr {
            str2 = xmlXPathCastNodeToString(*(*ns).nodeTab.offset(i as isize));
            if !str2.is_null() {
                valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, str2));
                xmlFree.expect("non-null function pointer")(str2 as *mut c_void);
                xmlXPathNumberFunction(ctxt, 1 as c_int);
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return 0 as c_int;
                }
                val = valuePop(ctxt);
                v = (*val).floatval;
                xmlXPathReleaseObject((*ctxt).context, val);
                if xmlXPathIsNaN(v) == 0 {
                    if neq == 0 && v == f {
                        ret = 1 as c_int;
                        break;
                    } else if neq != 0 && v != f {
                        ret = 1 as c_int;
                        break;
                    }
                } else if neq != 0 {
                    ret = 1 as c_int;
                }
            }
            i += 1;
        }
    }
    return ret;
} }
fn xmlXPathEqualNodeSets(
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
    mut neq: c_int,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut hashs1: *mut c_uint = ::core::ptr::null_mut::<c_uint>();
    let mut hashs2: *mut c_uint = ::core::ptr::null_mut::<c_uint>();
    let mut values1: *mut *mut xmlChar = ::core::ptr::null_mut::<*mut xmlChar>();
    let mut values2: *mut *mut xmlChar = ::core::ptr::null_mut::<*mut xmlChar>();
    let mut ret: c_int = 0 as c_int;
    let mut ns1: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut ns2: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    if arg1.is_null()
        || (*arg1).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg1).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        return 0 as c_int;
    }
    if arg2.is_null()
        || (*arg2).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg2).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        return 0 as c_int;
    }
    ns1 = (*arg1).nodesetval;
    ns2 = (*arg2).nodesetval;
    if ns1.is_null() || (*ns1).nodeNr <= 0 as c_int {
        return 0 as c_int;
    }
    if ns2.is_null() || (*ns2).nodeNr <= 0 as c_int {
        return 0 as c_int;
    }
    if neq == 0 as c_int {
        i = 0 as c_int;
        while i < (*ns1).nodeNr {
            j = 0 as c_int;
            while j < (*ns2).nodeNr {
                if *(*ns1).nodeTab.offset(i as isize) == *(*ns2).nodeTab.offset(j as isize) {
                    return 1 as c_int;
                }
                j += 1;
            }
            i += 1;
        }
    }
    values1 = xmlMalloc.expect("non-null function pointer")(
        ((*ns1).nodeNr as size_t).wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as size_t),
    ) as *mut *mut xmlChar;
    if values1.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"comparing nodesets\n\0" as *const u8 as *const c_char,
        );
        return 0 as c_int;
    }
    hashs1 = xmlMalloc.expect("non-null function pointer")(
        ((*ns1).nodeNr as size_t)
            .wrapping_mul(::core::mem::size_of::<c_uint>() as size_t),
    ) as *mut c_uint;
    if hashs1.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"comparing nodesets\n\0" as *const u8 as *const c_char,
        );
        xmlFree.expect("non-null function pointer")(values1 as *mut c_void);
        return 0 as c_int;
    }
    memset(
        values1 as *mut c_void,
        0 as c_int,
        ((*ns1).nodeNr as size_t).wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as size_t),
    );
    values2 = xmlMalloc.expect("non-null function pointer")(
        ((*ns2).nodeNr as size_t).wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as size_t),
    ) as *mut *mut xmlChar;
    if values2.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"comparing nodesets\n\0" as *const u8 as *const c_char,
        );
        xmlFree.expect("non-null function pointer")(hashs1 as *mut c_void);
        xmlFree.expect("non-null function pointer")(values1 as *mut c_void);
        return 0 as c_int;
    }
    hashs2 = xmlMalloc.expect("non-null function pointer")(
        ((*ns2).nodeNr as size_t)
            .wrapping_mul(::core::mem::size_of::<c_uint>() as size_t),
    ) as *mut c_uint;
    if hashs2.is_null() {
        xmlXPathErrMemory(
            ::core::ptr::null_mut::<xmlXPathContext>(),
            b"comparing nodesets\n\0" as *const u8 as *const c_char,
        );
        xmlFree.expect("non-null function pointer")(hashs1 as *mut c_void);
        xmlFree.expect("non-null function pointer")(values1 as *mut c_void);
        xmlFree.expect("non-null function pointer")(values2 as *mut c_void);
        return 0 as c_int;
    }
    memset(
        values2 as *mut c_void,
        0 as c_int,
        ((*ns2).nodeNr as size_t).wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as size_t),
    );
    i = 0 as c_int;
    while i < (*ns1).nodeNr {
        *hashs1.offset(i as isize) = xmlXPathNodeValHash(*(*ns1).nodeTab.offset(i as isize));
        j = 0 as c_int;
        while j < (*ns2).nodeNr {
            if i == 0 as c_int {
                *hashs2.offset(j as isize) =
                    xmlXPathNodeValHash(*(*ns2).nodeTab.offset(j as isize));
            }
            if *hashs1.offset(i as isize) != *hashs2.offset(j as isize) {
                if neq != 0 {
                    ret = 1 as c_int;
                    break;
                }
            } else {
                if (*values1.offset(i as isize)).is_null() {
                    let ref mut fresh64 = *values1.offset(i as isize);
                    *fresh64 =
                        xmlNodeGetContent(*(*ns1).nodeTab.offset(i as isize) as *const xmlNode);
                }
                if (*values2.offset(j as isize)).is_null() {
                    let ref mut fresh65 = *values2.offset(j as isize);
                    *fresh65 =
                        xmlNodeGetContent(*(*ns2).nodeTab.offset(j as isize) as *const xmlNode);
                }
                ret = xmlStrEqual(*values1.offset(i as isize), *values2.offset(j as isize)) ^ neq;
                if ret != 0 {
                    break;
                }
            }
            j += 1;
        }
        if ret != 0 {
            break;
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < (*ns1).nodeNr {
        if !(*values1.offset(i as isize)).is_null() {
            xmlFree.expect("non-null function pointer")(
                *values1.offset(i as isize) as *mut c_void
            );
        }
        i += 1;
    }
    j = 0 as c_int;
    while j < (*ns2).nodeNr {
        if !(*values2.offset(j as isize)).is_null() {
            xmlFree.expect("non-null function pointer")(
                *values2.offset(j as isize) as *mut c_void
            );
        }
        j += 1;
    }
    xmlFree.expect("non-null function pointer")(values1 as *mut c_void);
    xmlFree.expect("non-null function pointer")(values2 as *mut c_void);
    xmlFree.expect("non-null function pointer")(hashs1 as *mut c_void);
    xmlFree.expect("non-null function pointer")(hashs2 as *mut c_void);
    return ret;
} }
fn xmlXPathEqualValuesCommon(
    mut ctxt: xmlXPathParserContextPtr,
    mut arg1: xmlXPathObjectPtr,
    mut arg2: xmlXPathObjectPtr,
) -> c_int { unsafe {
    let mut ret: c_int = 0 as c_int;
    match (*arg1).type_0 as c_uint {
        2 => match (*arg2).type_0 as c_uint {
            2 => {
                ret = ((*arg1).boolval == (*arg2).boolval) as c_int;
            }
            3 => {
                ret = ((*arg1).boolval == xmlXPathCastNumberToBoolean((*arg2).floatval))
                    as c_int;
            }
            4 => {
                if (*arg2).stringval.is_null()
                    || *(*arg2).stringval.offset(0 as c_int as isize)
                        as c_int
                        == 0 as c_int
                {
                    ret = 0 as c_int;
                } else {
                    ret = 1 as c_int;
                }
                ret = ((*arg1).boolval == ret) as c_int;
            }
            8 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0"
                        as *const u8 as *const c_char,
                    6316 as c_int,
                );
            }
            0 | 1 | 9 | _ => {}
        },
        3 => {
            let mut current_block_37: u64;
            match (*arg2).type_0 as c_uint {
                2 => {
                    ret = ((*arg2).boolval == xmlXPathCastNumberToBoolean((*arg1).floatval))
                        as c_int;
                    current_block_37 = 12997042908615822766;
                }
                4 => {
                    valuePush(ctxt, arg2);
                    xmlXPathNumberFunction(ctxt, 1 as c_int);
                    arg2 = valuePop(ctxt);
                    if (*ctxt).error != 0 {
                        current_block_37 = 12997042908615822766;
                    } else {
                        current_block_37 = 6683055545966474422;
                    }
                }
                3 => {
                    current_block_37 = 6683055545966474422;
                }
                8 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\0" as *const u8
                            as *const c_char,
                        b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0"
                            as *const u8 as *const c_char,
                        6373 as c_int,
                    );
                    current_block_37 = 12997042908615822766;
                }
                0 | 1 | 9 | _ => {
                    current_block_37 = 12997042908615822766;
                }
            }
            match current_block_37 {
                6683055545966474422 => {
                    if xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0
                    {
                        ret = 0 as c_int;
                    } else if xmlXPathIsInf((*arg1).floatval) == 1 as c_int {
                        if xmlXPathIsInf((*arg2).floatval) == 1 as c_int {
                            ret = 1 as c_int;
                        } else {
                            ret = 0 as c_int;
                        }
                    } else if xmlXPathIsInf((*arg1).floatval) == -(1 as c_int) {
                        if xmlXPathIsInf((*arg2).floatval) == -(1 as c_int) {
                            ret = 1 as c_int;
                        } else {
                            ret = 0 as c_int;
                        }
                    } else if xmlXPathIsInf((*arg2).floatval) == 1 as c_int {
                        if xmlXPathIsInf((*arg1).floatval) == 1 as c_int {
                            ret = 1 as c_int;
                        } else {
                            ret = 0 as c_int;
                        }
                    } else if xmlXPathIsInf((*arg2).floatval) == -(1 as c_int) {
                        if xmlXPathIsInf((*arg1).floatval) == -(1 as c_int) {
                            ret = 1 as c_int;
                        } else {
                            ret = 0 as c_int;
                        }
                    } else {
                        ret = ((*arg1).floatval == (*arg2).floatval) as c_int;
                    }
                }
                _ => {}
            }
        }
        4 => match (*arg2).type_0 as c_uint {
            2 => {
                if (*arg1).stringval.is_null()
                    || *(*arg1).stringval.offset(0 as c_int as isize)
                        as c_int
                        == 0 as c_int
                {
                    ret = 0 as c_int;
                } else {
                    ret = 1 as c_int;
                }
                ret = ((*arg2).boolval == ret) as c_int;
            }
            4 => {
                ret = xmlStrEqual((*arg1).stringval, (*arg2).stringval);
            }
            3 => {
                valuePush(ctxt, arg1);
                xmlXPathNumberFunction(ctxt, 1 as c_int);
                arg1 = valuePop(ctxt);
                if !((*ctxt).error != 0) {
                    if xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0
                    {
                        ret = 0 as c_int;
                    } else if xmlXPathIsInf((*arg1).floatval) == 1 as c_int {
                        if xmlXPathIsInf((*arg2).floatval) == 1 as c_int {
                            ret = 1 as c_int;
                        } else {
                            ret = 0 as c_int;
                        }
                    } else if xmlXPathIsInf((*arg1).floatval) == -(1 as c_int) {
                        if xmlXPathIsInf((*arg2).floatval) == -(1 as c_int) {
                            ret = 1 as c_int;
                        } else {
                            ret = 0 as c_int;
                        }
                    } else if xmlXPathIsInf((*arg2).floatval) == 1 as c_int {
                        if xmlXPathIsInf((*arg1).floatval) == 1 as c_int {
                            ret = 1 as c_int;
                        } else {
                            ret = 0 as c_int;
                        }
                    } else if xmlXPathIsInf((*arg2).floatval) == -(1 as c_int) {
                        if xmlXPathIsInf((*arg1).floatval) == -(1 as c_int) {
                            ret = 1 as c_int;
                        } else {
                            ret = 0 as c_int;
                        }
                    } else {
                        ret = ((*arg1).floatval == (*arg2).floatval) as c_int;
                    }
                }
            }
            8 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0"
                        as *const u8 as *const c_char,
                    6434 as c_int,
                );
            }
            0 | 1 | 9 | _ => {}
        },
        8 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                    as *const c_char,
                6447 as c_int,
            );
        }
        0 | 1 | 9 | _ => {}
    }
    xmlXPathReleaseObject((*ctxt).context, arg1);
    xmlXPathReleaseObject((*ctxt).context, arg2);
    return ret;
} }
#[inline]
pub fn xmlXPathEqualValues(
    mut ctxt: xmlXPathParserContextPtr,
) -> c_int { unsafe {
    let mut arg1: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut arg2: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut argtmp: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut ret: c_int = 0 as c_int;
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return 0 as c_int;
    }
    arg2 = valuePop(ctxt);
    arg1 = valuePop(ctxt);
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            xmlXPathReleaseObject((*ctxt).context, arg1);
        } else {
            xmlXPathReleaseObject((*ctxt).context, arg2);
        }
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return 0 as c_int;
    }
    if arg1 == arg2 {
        xmlXPathFreeObject(arg1);
        return 1 as c_int;
    }
    if (*arg2).type_0 as c_uint
        == XPATH_NODESET as c_int as c_uint
        || (*arg2).type_0 as c_uint
            == XPATH_XSLT_TREE as c_int as c_uint
        || (*arg1).type_0 as c_uint
            == XPATH_NODESET as c_int as c_uint
        || (*arg1).type_0 as c_uint
            == XPATH_XSLT_TREE as c_int as c_uint
    {
        if (*arg1).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg1).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
        {
            argtmp = arg2;
            arg2 = arg1;
            arg1 = argtmp;
        }
        match (*arg2).type_0 as c_uint {
            1 | 9 => {
                ret = xmlXPathEqualNodeSets(arg1, arg2, 0 as c_int);
            }
            2 => {
                if (*arg1).nodesetval.is_null()
                    || (*(*arg1).nodesetval).nodeNr == 0 as c_int
                {
                    ret = 0 as c_int;
                } else {
                    ret = 1 as c_int;
                }
                ret = (ret == (*arg2).boolval) as c_int;
            }
            3 => {
                ret = xmlXPathEqualNodeSetFloat(
                    ctxt,
                    arg1,
                    (*arg2).floatval,
                    0 as c_int,
                );
            }
            4 => {
                ret = xmlXPathEqualNodeSetString(arg1, (*arg2).stringval, 0 as c_int);
            }
            8 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0"
                        as *const u8 as *const c_char,
                    6526 as c_int,
                );
            }
            0 | _ => {}
        }
        xmlXPathReleaseObject((*ctxt).context, arg1);
        xmlXPathReleaseObject((*ctxt).context, arg2);
        return ret;
    }
    return xmlXPathEqualValuesCommon(ctxt, arg1, arg2);
} }
#[inline]
pub fn xmlXPathNotEqualValues(
    mut ctxt: xmlXPathParserContextPtr,
) -> c_int { unsafe {
    let mut arg1: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut arg2: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut argtmp: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut ret: c_int = 0 as c_int;
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return 0 as c_int;
    }
    arg2 = valuePop(ctxt);
    arg1 = valuePop(ctxt);
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            xmlXPathReleaseObject((*ctxt).context, arg1);
        } else {
            xmlXPathReleaseObject((*ctxt).context, arg2);
        }
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return 0 as c_int;
    }
    if arg1 == arg2 {
        xmlXPathReleaseObject((*ctxt).context, arg1);
        return 0 as c_int;
    }
    if (*arg2).type_0 as c_uint
        == XPATH_NODESET as c_int as c_uint
        || (*arg2).type_0 as c_uint
            == XPATH_XSLT_TREE as c_int as c_uint
        || (*arg1).type_0 as c_uint
            == XPATH_NODESET as c_int as c_uint
        || (*arg1).type_0 as c_uint
            == XPATH_XSLT_TREE as c_int as c_uint
    {
        if (*arg1).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*arg1).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
        {
            argtmp = arg2;
            arg2 = arg1;
            arg1 = argtmp;
        }
        match (*arg2).type_0 as c_uint {
            1 | 9 => {
                ret = xmlXPathEqualNodeSets(arg1, arg2, 1 as c_int);
            }
            2 => {
                if (*arg1).nodesetval.is_null()
                    || (*(*arg1).nodesetval).nodeNr == 0 as c_int
                {
                    ret = 0 as c_int;
                } else {
                    ret = 1 as c_int;
                }
                ret = (ret != (*arg2).boolval) as c_int;
            }
            3 => {
                ret = xmlXPathEqualNodeSetFloat(
                    ctxt,
                    arg1,
                    (*arg2).floatval,
                    1 as c_int,
                );
            }
            4 => {
                ret = xmlXPathEqualNodeSetString(arg1, (*arg2).stringval, 1 as c_int);
            }
            8 => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0"
                        as *const u8 as *const c_char,
                    6605 as c_int,
                );
            }
            0 | _ => {}
        }
        xmlXPathReleaseObject((*ctxt).context, arg1);
        xmlXPathReleaseObject((*ctxt).context, arg2);
        return ret;
    }
    return (xmlXPathEqualValuesCommon(ctxt, arg1, arg2) == 0) as c_int;
} }
#[inline]
pub fn xmlXPathCompareValues(
    mut ctxt: xmlXPathParserContextPtr,
    mut inf: c_int,
    mut strict: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0 as c_int;
    let mut arg1i: c_int = 0 as c_int;
    let mut arg2i: c_int = 0 as c_int;
    let mut arg1: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut arg2: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return 0 as c_int;
    }
    arg2 = valuePop(ctxt);
    arg1 = valuePop(ctxt);
    if arg1.is_null() || arg2.is_null() {
        if !arg1.is_null() {
            xmlXPathReleaseObject((*ctxt).context, arg1);
        } else {
            xmlXPathReleaseObject((*ctxt).context, arg2);
        }
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return 0 as c_int;
    }
    if (*arg2).type_0 as c_uint
        == XPATH_NODESET as c_int as c_uint
        || (*arg2).type_0 as c_uint
            == XPATH_XSLT_TREE as c_int as c_uint
        || (*arg1).type_0 as c_uint
            == XPATH_NODESET as c_int as c_uint
        || (*arg1).type_0 as c_uint
            == XPATH_XSLT_TREE as c_int as c_uint
    {
        if ((*arg2).type_0 as c_uint
            == XPATH_NODESET as c_int as c_uint
            || (*arg2).type_0 as c_uint
                == XPATH_XSLT_TREE as c_int as c_uint)
            && ((*arg1).type_0 as c_uint
                == XPATH_NODESET as c_int as c_uint
                || (*arg1).type_0 as c_uint
                    == XPATH_XSLT_TREE as c_int as c_uint)
        {
            ret = xmlXPathCompareNodeSets(inf, strict, arg1, arg2);
        } else if (*arg1).type_0 as c_uint
            == XPATH_NODESET as c_int as c_uint
            || (*arg1).type_0 as c_uint
                == XPATH_XSLT_TREE as c_int as c_uint
        {
            ret = xmlXPathCompareNodeSetValue(ctxt, inf, strict, arg1, arg2);
        } else {
            ret = xmlXPathCompareNodeSetValue(
                ctxt,
                (inf == 0) as c_int,
                strict,
                arg2,
                arg1,
            );
        }
        return ret;
    }
    if (*arg1).type_0 as c_uint
        != XPATH_NUMBER as c_int as c_uint
    {
        valuePush(ctxt, arg1);
        xmlXPathNumberFunction(ctxt, 1 as c_int);
        arg1 = valuePop(ctxt);
    }
    if (*arg2).type_0 as c_uint
        != XPATH_NUMBER as c_int as c_uint
    {
        valuePush(ctxt, arg2);
        xmlXPathNumberFunction(ctxt, 1 as c_int);
        arg2 = valuePop(ctxt);
    }
    if !((*ctxt).error != 0) {
        if xmlXPathIsNaN((*arg1).floatval) != 0 || xmlXPathIsNaN((*arg2).floatval) != 0 {
            ret = 0 as c_int;
        } else {
            arg1i = xmlXPathIsInf((*arg1).floatval);
            arg2i = xmlXPathIsInf((*arg2).floatval);
            if inf != 0 && strict != 0 {
                if arg1i == -(1 as c_int) && arg2i != -(1 as c_int)
                    || arg2i == 1 as c_int && arg1i != 1 as c_int
                {
                    ret = 1 as c_int;
                } else if arg1i == 0 as c_int && arg2i == 0 as c_int {
                    ret = ((*arg1).floatval < (*arg2).floatval) as c_int;
                } else {
                    ret = 0 as c_int;
                }
            } else if inf != 0 && strict == 0 {
                if arg1i == -(1 as c_int) || arg2i == 1 as c_int {
                    ret = 1 as c_int;
                } else if arg1i == 0 as c_int && arg2i == 0 as c_int {
                    ret = ((*arg1).floatval <= (*arg2).floatval) as c_int;
                } else {
                    ret = 0 as c_int;
                }
            } else if inf == 0 && strict != 0 {
                if arg1i == 1 as c_int && arg2i != 1 as c_int
                    || arg2i == -(1 as c_int) && arg1i != -(1 as c_int)
                {
                    ret = 1 as c_int;
                } else if arg1i == 0 as c_int && arg2i == 0 as c_int {
                    ret = ((*arg1).floatval > (*arg2).floatval) as c_int;
                } else {
                    ret = 0 as c_int;
                }
            } else if inf == 0 && strict == 0 {
                if arg1i == 1 as c_int || arg2i == -(1 as c_int) {
                    ret = 1 as c_int;
                } else if arg1i == 0 as c_int && arg2i == 0 as c_int {
                    ret = ((*arg1).floatval >= (*arg2).floatval) as c_int;
                } else {
                    ret = 0 as c_int;
                }
            }
        }
    }
    xmlXPathReleaseObject((*ctxt).context, arg1);
    xmlXPathReleaseObject((*ctxt).context, arg2);
    return ret;
} }
#[inline]
pub fn xmlXPathValueFlipSign(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    (*(*ctxt).value).floatval = -(*(*ctxt).value).floatval;
} }
#[inline]
pub fn xmlXPathAddValues(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut arg: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut val: c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return;
    }
    val = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    (*(*ctxt).value).floatval += val;
} }
#[inline]
pub fn xmlXPathSubValues(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut arg: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut val: c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return;
    }
    val = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    (*(*ctxt).value).floatval -= val;
} }
#[inline]
pub fn xmlXPathMultValues(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut arg: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut val: c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return;
    }
    val = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    (*(*ctxt).value).floatval *= val;
} }
#[inline]
pub fn xmlXPathDivValues(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut arg: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut val: c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return;
    }
    val = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    (*(*ctxt).value).floatval /= val;
} }
#[inline]
pub fn xmlXPathModValues(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut arg: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut arg1: c_double = 0.;
    let mut arg2: c_double = 0.;
    arg = valuePop(ctxt);
    if arg.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return;
    }
    arg2 = xmlXPathCastToNumber(arg);
    xmlXPathReleaseObject((*ctxt).context, arg);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    arg1 = (*(*ctxt).value).floatval;
    if arg2 == 0 as c_int as c_double {
        (*(*ctxt).value).floatval = xmlXPathNAN;
    } else {
        (*(*ctxt).value).floatval = fmod(arg1, arg2);
    };
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        return (*(*ctxt).context).node;
    }
    return ::core::ptr::null_mut::<xmlNode>();
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextChild(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        if (*(*ctxt).context).node.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        match (*(*(*ctxt).context).node).type_0 as c_uint {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 => {
                return (*(*(*ctxt).context).node).children as xmlNodePtr;
            }
            9 | 10 | 11 | 13 => {
                return (*((*(*ctxt).context).node as xmlDocPtr)).children as xmlNodePtr;
            }
            15 | 16 | 17 | 2 | 18 | 19 | 20 => return ::core::ptr::null_mut::<xmlNode>(),
            _ => {}
        }
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*cur).type_0 as c_uint
        == XML_DOCUMENT_NODE as c_int as c_uint
        || (*cur).type_0 as c_uint
            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    return (*cur).next as xmlNodePtr;
} }
extern "C" fn xmlXPathNextChildElement(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        cur = (*(*ctxt).context).node;
        if cur.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        match (*cur).type_0 as c_uint {
            1 | 11 | 5 | 6 => {
                cur = (*cur).children as xmlNodePtr;
                if !cur.is_null() {
                    if (*cur).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                    {
                        return cur;
                    }
                    loop {
                        cur = (*cur).next as xmlNodePtr;
                        if !(!cur.is_null()
                            && (*cur).type_0 as c_uint
                                != XML_ELEMENT_NODE as c_int as c_uint)
                        {
                            break;
                        }
                    }
                    return cur;
                }
                return ::core::ptr::null_mut::<xmlNode>();
            }
            9 | 13 => return xmlDocGetRootElement(cur as xmlDocPtr as *const xmlDoc),
            _ => return ::core::ptr::null_mut::<xmlNode>(),
        }
    }
    match (*cur).type_0 as c_uint {
        1 | 3 | 5 | 6 | 4 | 7 | 8 | 20 => {}
        _ => return ::core::ptr::null_mut::<xmlNode>(),
    }
    if !(*cur).next.is_null() {
        if (*(*cur).next).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            return (*cur).next as xmlNodePtr;
        }
        cur = (*cur).next as xmlNodePtr;
        loop {
            cur = (*cur).next as xmlNodePtr;
            if !(!cur.is_null()
                && (*cur).type_0 as c_uint
                    != XML_ELEMENT_NODE as c_int as c_uint)
            {
                break;
            }
        }
        return cur;
    }
    return ::core::ptr::null_mut::<xmlNode>();
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextDescendant(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        if (*(*ctxt).context).node.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if (*(*(*ctxt).context).node).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
            || (*(*(*ctxt).context).node).type_0 as c_uint
                == XML_NAMESPACE_DECL as c_int as c_uint
        {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if (*(*ctxt).context).node == (*(*ctxt).context).doc as xmlNodePtr {
            return (*(*(*ctxt).context).doc).children as xmlNodePtr;
        }
        return (*(*(*ctxt).context).node).children as xmlNodePtr;
    }
    if (*cur).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*cur).children.is_null() {
        if (*(*cur).children).type_0 as c_uint
            != XML_ENTITY_DECL as c_int as c_uint
        {
            cur = (*cur).children as xmlNodePtr;
            if (*cur).type_0 as c_uint
                != XML_DTD_NODE as c_int as c_uint
            {
                return cur;
            }
        }
    }
    if cur == (*(*ctxt).context).node {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    while !(*cur).next.is_null() {
        cur = (*cur).next as xmlNodePtr;
        if (*cur).type_0 as c_uint
            != XML_ENTITY_DECL as c_int as c_uint
            && (*cur).type_0 as c_uint
                != XML_DTD_NODE as c_int as c_uint
        {
            return cur;
        }
    }
    loop {
        cur = (*cur).parent as xmlNodePtr;
        if cur.is_null() {
            break;
        }
        if cur == (*(*ctxt).context).node {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if !(*cur).next.is_null() {
            cur = (*cur).next as xmlNodePtr;
            return cur;
        }
        if cur.is_null() {
            break;
        }
    }
    return cur;
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextDescendantOrSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        return (*(*ctxt).context).node;
    }
    if (*(*ctxt).context).node.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*(*(*ctxt).context).node).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
        || (*(*(*ctxt).context).node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    return xmlXPathNextDescendant(ctxt, cur);
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextParent(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        if (*(*ctxt).context).node.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        match (*(*(*ctxt).context).node).type_0 as c_uint {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 15 | 16 | 19 | 20 | 17 => {
                if (*(*(*ctxt).context).node).parent.is_null() {
                    return (*(*ctxt).context).doc as xmlNodePtr;
                }
                if (*(*(*(*ctxt).context).node).parent).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    && (*(*(*(*(*ctxt).context).node).parent)
                        .name
                        .offset(0 as c_int as isize)
                        as c_int
                        == ' ' as i32
                        || xmlStrEqual(
                            (*(*(*(*ctxt).context).node).parent).name,
                            b"fake node libxslt\0" as *const u8 as *const c_char
                                as *mut xmlChar,
                        ) != 0)
                {
                    return ::core::ptr::null_mut::<xmlNode>();
                }
                return (*(*(*ctxt).context).node).parent as xmlNodePtr;
            }
            2 => {
                let mut att: xmlAttrPtr = (*(*ctxt).context).node as xmlAttrPtr;
                return (*att).parent as xmlNodePtr;
            }
            9 | 10 | 11 | 13 => return ::core::ptr::null_mut::<xmlNode>(),
            18 => {
                let mut ns: xmlNsPtr = (*(*ctxt).context).node as xmlNsPtr;
                if !(*ns).next.is_null()
                    && (*(*ns).next).type_0 as c_uint
                        != XML_NAMESPACE_DECL as c_int as c_uint
                {
                    return (*ns).next as xmlNodePtr;
                }
                return ::core::ptr::null_mut::<xmlNode>();
            }
            _ => {}
        }
    }
    return ::core::ptr::null_mut::<xmlNode>();
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextAncestor(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        if (*(*ctxt).context).node.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        match (*(*(*ctxt).context).node).type_0 as c_uint {
            1 | 3 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 16 | 17 | 12 | 19 | 20 => {
                if (*(*(*ctxt).context).node).parent.is_null() {
                    return (*(*ctxt).context).doc as xmlNodePtr;
                }
                if (*(*(*(*ctxt).context).node).parent).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    && (*(*(*(*(*ctxt).context).node).parent)
                        .name
                        .offset(0 as c_int as isize)
                        as c_int
                        == ' ' as i32
                        || xmlStrEqual(
                            (*(*(*(*ctxt).context).node).parent).name,
                            b"fake node libxslt\0" as *const u8 as *const c_char
                                as *mut xmlChar,
                        ) != 0)
                {
                    return ::core::ptr::null_mut::<xmlNode>();
                }
                return (*(*(*ctxt).context).node).parent as xmlNodePtr;
            }
            2 => {
                let mut tmp: xmlAttrPtr = (*(*ctxt).context).node as xmlAttrPtr;
                return (*tmp).parent as xmlNodePtr;
            }
            9 | 10 | 11 | 13 => return ::core::ptr::null_mut::<xmlNode>(),
            18 => {
                let mut ns: xmlNsPtr = (*(*ctxt).context).node as xmlNsPtr;
                if !(*ns).next.is_null()
                    && (*(*ns).next).type_0 as c_uint
                        != XML_NAMESPACE_DECL as c_int as c_uint
                {
                    return (*ns).next as xmlNodePtr;
                }
                return ::core::ptr::null_mut::<xmlNode>();
            }
            _ => {}
        }
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur == (*(*(*ctxt).context).doc).children {
        return (*(*ctxt).context).doc as xmlNodePtr;
    }
    if cur == (*(*ctxt).context).doc as xmlNodePtr {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    match (*cur).type_0 as c_uint {
        1 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 15 | 16 | 17 | 19 | 20 => {
            if (*cur).parent.is_null() {
                return ::core::ptr::null_mut::<xmlNode>();
            }
            if (*(*cur).parent).type_0 as c_uint
                == XML_ELEMENT_NODE as c_int as c_uint
                && (*(*(*cur).parent)
                    .name
                    .offset(0 as c_int as isize)
                    as c_int
                    == ' ' as i32
                    || xmlStrEqual(
                        (*(*cur).parent).name,
                        b"fake node libxslt\0" as *const u8 as *const c_char
                            as *mut xmlChar,
                    ) != 0)
            {
                return ::core::ptr::null_mut::<xmlNode>();
            }
            return (*cur).parent as xmlNodePtr;
        }
        2 => {
            let mut att: xmlAttrPtr = cur as xmlAttrPtr;
            return (*att).parent as xmlNodePtr;
        }
        18 => {
            let mut ns_0: xmlNsPtr = cur as xmlNsPtr;
            if !(*ns_0).next.is_null()
                && (*(*ns_0).next).type_0 as c_uint
                    != XML_NAMESPACE_DECL as c_int as c_uint
            {
                return (*ns_0).next as xmlNodePtr;
            }
            return ::core::ptr::null_mut::<xmlNode>();
        }
        9 | 10 | 11 | 13 => return ::core::ptr::null_mut::<xmlNode>(),
        _ => {}
    }
    return ::core::ptr::null_mut::<xmlNode>();
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextAncestorOrSelf(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        return (*(*ctxt).context).node;
    }
    return xmlXPathNextAncestor(ctxt, cur);
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextFollowingSibling(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*(*(*ctxt).context).node).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
        || (*(*(*ctxt).context).node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur == (*(*ctxt).context).doc as xmlNodePtr {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        return (*(*(*ctxt).context).node).next as xmlNodePtr;
    }
    return (*cur).next as xmlNodePtr;
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextPrecedingSibling(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*(*(*ctxt).context).node).type_0 as c_uint
        == XML_ATTRIBUTE_NODE as c_int as c_uint
        || (*(*(*ctxt).context).node).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur == (*(*ctxt).context).doc as xmlNodePtr {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        return (*(*(*ctxt).context).node).prev as xmlNodePtr;
    }
    if !(*cur).prev.is_null()
        && (*(*cur).prev).type_0 as c_uint
            == XML_DTD_NODE as c_int as c_uint
    {
        cur = (*cur).prev as xmlNodePtr;
        if cur.is_null() {
            return (*(*(*ctxt).context).node).prev as xmlNodePtr;
        }
    }
    return (*cur).prev as xmlNodePtr;
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextFollowing(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !cur.is_null()
        && (*cur).type_0 as c_uint
            != XML_ATTRIBUTE_NODE as c_int as c_uint
        && (*cur).type_0 as c_uint
            != XML_NAMESPACE_DECL as c_int as c_uint
        && !(*cur).children.is_null()
    {
        return (*cur).children as xmlNodePtr;
    }
    if cur.is_null() {
        cur = (*(*ctxt).context).node;
        if (*cur).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
        {
            cur = (*cur).parent as xmlNodePtr;
        } else if (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if (*ns).next.is_null()
                || (*(*ns).next).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
            {
                return ::core::ptr::null_mut::<xmlNode>();
            }
            cur = (*ns).next as xmlNodePtr;
        }
    }
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
        if cur == (*(*ctxt).context).doc as xmlNodePtr {
            return ::core::ptr::null_mut::<xmlNode>();
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
fn xmlXPathIsAncestor(
    mut ancestor: xmlNodePtr,
    mut node: xmlNodePtr,
) -> c_int { unsafe {
    if ancestor.is_null() || node.is_null() {
        return 0 as c_int;
    }
    if (*node).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return 0 as c_int;
    }
    if (*ancestor).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return 0 as c_int;
    }
    if (*ancestor).doc != (*node).doc {
        return 0 as c_int;
    }
    if ancestor == (*node).doc as xmlNodePtr {
        return 1 as c_int;
    }
    if node == (*ancestor).doc as xmlNodePtr {
        return 0 as c_int;
    }
    while !(*node).parent.is_null() {
        if (*node).parent == ancestor {
            return 1 as c_int;
        }
        node = (*node).parent as xmlNodePtr;
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlXPathNextPreceding(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        cur = (*(*ctxt).context).node;
        if (*cur).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
        {
            cur = (*cur).parent as xmlNodePtr;
        } else if (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if (*ns).next.is_null()
                || (*(*ns).next).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
            {
                return ::core::ptr::null_mut::<xmlNode>();
            }
            cur = (*ns).next as xmlNodePtr;
        }
    }
    if cur.is_null()
        || (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*cur).prev.is_null()
        && (*(*cur).prev).type_0 as c_uint
            == XML_DTD_NODE as c_int as c_uint
    {
        cur = (*cur).prev as xmlNodePtr;
    }
    loop {
        if !(*cur).prev.is_null() {
            cur = (*cur).prev as xmlNodePtr;
            while !(*cur).last.is_null() {
                cur = (*cur).last as xmlNodePtr;
            }
            return cur;
        }
        cur = (*cur).parent as xmlNodePtr;
        if cur.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if cur == (*(*(*ctxt).context).doc).children {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if !(xmlXPathIsAncestor(cur, (*(*ctxt).context).node) != 0) {
            break;
        }
    }
    return cur;
} }
extern "C" fn xmlXPathNextPrecedingInternal(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        cur = (*(*ctxt).context).node;
        if cur.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if (*cur).type_0 as c_uint
            == XML_ATTRIBUTE_NODE as c_int as c_uint
        {
            cur = (*cur).parent as xmlNodePtr;
        } else if (*cur).type_0 as c_uint
            == XML_NAMESPACE_DECL as c_int as c_uint
        {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            if (*ns).next.is_null()
                || (*(*ns).next).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
            {
                return ::core::ptr::null_mut::<xmlNode>();
            }
            cur = (*ns).next as xmlNodePtr;
        }
        (*ctxt).ancestor = (*cur).parent as xmlNodePtr;
    }
    if (*cur).type_0 as c_uint
        == XML_NAMESPACE_DECL as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*cur).prev.is_null()
        && (*(*cur).prev).type_0 as c_uint
            == XML_DTD_NODE as c_int as c_uint
    {
        cur = (*cur).prev as xmlNodePtr;
    }
    while (*cur).prev.is_null() {
        cur = (*cur).parent as xmlNodePtr;
        if cur.is_null() {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if cur == (*(*(*ctxt).context).doc).children {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        if cur != (*ctxt).ancestor {
            return cur;
        }
        (*ctxt).ancestor = (*cur).parent as xmlNodePtr;
    }
    cur = (*cur).prev as xmlNodePtr;
    while !(*cur).last.is_null() {
        cur = (*cur).last as xmlNodePtr;
    }
    return cur;
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextNamespace(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*(*(*ctxt).context).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        if !(*(*ctxt).context).tmpNsList.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*(*ctxt).context).tmpNsList as *mut c_void,
            );
        }
        (*(*ctxt).context).tmpNsList = xmlGetNsList(
            (*(*ctxt).context).doc as *const xmlDoc,
            (*(*ctxt).context).node as *const xmlNode,
        );
        (*(*ctxt).context).tmpNsNr = 0 as c_int;
        if !(*(*ctxt).context).tmpNsList.is_null() {
            while !(*(*(*ctxt).context)
                .tmpNsList
                .offset((*(*ctxt).context).tmpNsNr as isize))
            .is_null()
            {
                (*(*ctxt).context).tmpNsNr += 1;
            }
        }
        return xmlXPathXMLNamespace as xmlNodePtr;
    }
    if (*(*ctxt).context).tmpNsNr > 0 as c_int {
        (*(*ctxt).context).tmpNsNr -= 1;
        return *(*(*ctxt).context)
            .tmpNsList
            .offset((*(*ctxt).context).tmpNsNr as isize) as xmlNodePtr;
    } else {
        if !(*(*ctxt).context).tmpNsList.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*(*ctxt).context).tmpNsList as *mut c_void,
            );
        }
        (*(*ctxt).context).tmpNsList = ::core::ptr::null_mut::<xmlNsPtr>();
        return ::core::ptr::null_mut::<xmlNode>();
    };
} }
#[no_mangle]
pub extern "C" fn xmlXPathNextAttribute(
    mut ctxt: xmlXPathParserContextPtr,
    mut cur: xmlNodePtr,
) -> xmlNodePtr { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*(*ctxt).context).node.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if (*(*(*ctxt).context).node).type_0 as c_uint
        != XML_ELEMENT_NODE as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if cur.is_null() {
        if (*(*ctxt).context).node == (*(*ctxt).context).doc as xmlNodePtr {
            return ::core::ptr::null_mut::<xmlNode>();
        }
        return (*(*(*ctxt).context).node).properties as xmlNodePtr;
    }
    return (*cur).next as xmlNodePtr;
} }
#[no_mangle]
pub extern "C" fn xmlXPathRoot(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    if ctxt.is_null() || (*ctxt).context.is_null() {
        return;
    }
    valuePush(
        ctxt,
        xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).doc as xmlNodePtr),
    );
} }
#[no_mangle]
pub extern "C" fn xmlXPathLastFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 0 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if (*(*ctxt).context).contextSize >= 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat(
                (*ctxt).context,
                (*(*ctxt).context).contextSize as c_double,
            ),
        );
    } else {
        xmlXPathErr(ctxt, XPATH_INVALID_CTXT_SIZE as c_int);
        return;
    };
} }
#[no_mangle]
pub extern "C" fn xmlXPathPositionFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 0 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if (*(*ctxt).context).proximityPosition >= 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat(
                (*ctxt).context,
                (*(*ctxt).context).proximityPosition as c_double,
            ),
        );
    } else {
        xmlXPathErr(ctxt, XPATH_INVALID_CTXT_POSITION as c_int);
        return;
    };
} }
#[no_mangle]
pub extern "C" fn xmlXPathCountFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*(*ctxt).value).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    cur = valuePop(ctxt);
    if cur.is_null() || (*cur).nodesetval.is_null() {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat(
                (*ctxt).context,
                0 as c_int as c_double,
            ),
        );
    } else {
        valuePush(
            ctxt,
            xmlXPathCacheNewFloat(
                (*ctxt).context,
                (*(*cur).nodesetval).nodeNr as c_double,
            ),
        );
    }
    xmlXPathReleaseObject((*ctxt).context, cur);
} }
unsafe fn xmlXPathGetElementsByIds(
    mut doc: xmlDocPtr,
    mut ids: *const xmlChar,
) -> xmlNodeSetPtr {
    let mut ret: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut cur: *const xmlChar = ids;
    let mut ID: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut elem: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ids.is_null() {
        return ::core::ptr::null_mut::<xmlNodeSet>();
    }
    ret = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
    if ret.is_null() {
        return ret;
    }
    while *cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *cur as c_int
            && *cur as c_int <= 0xa as c_int
        || *cur as c_int == 0xd as c_int
    {
        cur = cur.offset(1);
    }
    while *cur as c_int != 0 as c_int {
        while !(*cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int)
            && *cur as c_int != 0 as c_int
        {
            cur = cur.offset(1);
        }
        ID = xmlStrndup(
            ids,
            cur.offset_from(ids) as c_long as c_int,
        );
        if !ID.is_null() {
            attr = xmlGetID(doc, ID);
            if !attr.is_null() {
                if (*attr).type_0 as c_uint
                    == XML_ATTRIBUTE_NODE as c_int as c_uint
                {
                    elem = (*attr).parent as xmlNodePtr;
                } else if (*attr).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                {
                    elem = attr as xmlNodePtr;
                } else {
                    elem = ::core::ptr::null_mut::<xmlNode>();
                }
                if !elem.is_null() {
                    xmlXPathNodeSetAdd(ret, elem);
                }
            }
            xmlFree.expect("non-null function pointer")(ID as *mut c_void);
        }
        while *cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int
        {
            cur = cur.offset(1);
        }
        ids = cur;
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn xmlXPathIdFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut tokens: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    obj = valuePop(ctxt);
    if obj.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return;
    }
    if (*obj).type_0 as c_uint
        == XPATH_NODESET as c_int as c_uint
        || (*obj).type_0 as c_uint
            == XPATH_XSLT_TREE as c_int as c_uint
    {
        let mut ns: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
        let mut i: c_int = 0;
        ret = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
        if !(*obj).nodesetval.is_null() {
            i = 0 as c_int;
            while i < (*(*obj).nodesetval).nodeNr {
                tokens = xmlXPathCastNodeToString(*(*(*obj).nodesetval).nodeTab.offset(i as isize));
                ns = xmlXPathGetElementsByIds((*(*ctxt).context).doc, tokens);
                ret = xmlXPathNodeSetMerge(ret, ns);
                xmlXPathFreeNodeSet(ns);
                if !tokens.is_null() {
                    xmlFree.expect("non-null function pointer")(tokens as *mut c_void);
                }
                i += 1;
            }
        }
        xmlXPathReleaseObject((*ctxt).context, obj);
        valuePush(ctxt, xmlXPathCacheWrapNodeSet((*ctxt).context, ret));
        return;
    }
    obj = xmlXPathCacheConvertString((*ctxt).context, obj);
    if obj.is_null() {
        return;
    }
    ret = xmlXPathGetElementsByIds((*(*ctxt).context).doc, (*obj).stringval);
    valuePush(ctxt, xmlXPathCacheWrapNodeSet((*ctxt).context, ret));
    xmlXPathReleaseObject((*ctxt).context, obj);
} }
#[no_mangle]
pub extern "C" fn xmlXPathLocalNameFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
        );
        nargs = 1 as c_int;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*(*ctxt).value).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    cur = valuePop(ctxt);
    if (*cur).nodesetval.is_null() || (*(*cur).nodesetval).nodeNr == 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewCString(
                (*ctxt).context,
                b"\0" as *const u8 as *const c_char,
            ),
        );
    } else {
        let mut i: c_int = 0 as c_int;
        match (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).type_0 as c_uint {
            1 | 2 | 7 => {
                if *(**(*(*cur).nodesetval).nodeTab.offset(i as isize))
                    .name
                    .offset(0 as c_int as isize)
                    as c_int
                    == ' ' as i32
                {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewCString(
                            (*ctxt).context,
                            b"\0" as *const u8 as *const c_char,
                        ),
                    );
                } else {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewString(
                            (*ctxt).context,
                            (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).name,
                        ),
                    );
                }
            }
            18 => {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewString(
                        (*ctxt).context,
                        (*(*(*(*cur).nodesetval).nodeTab.offset(i as isize) as xmlNsPtr)).prefix,
                    ),
                );
            }
            _ => {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewCString(
                        (*ctxt).context,
                        b"\0" as *const u8 as *const c_char,
                    ),
                );
            }
        }
    }
    xmlXPathReleaseObject((*ctxt).context, cur);
} }
#[no_mangle]
pub extern "C" fn xmlXPathNamespaceURIFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
        );
        nargs = 1 as c_int;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*(*ctxt).value).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    cur = valuePop(ctxt);
    if (*cur).nodesetval.is_null() || (*(*cur).nodesetval).nodeNr == 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewCString(
                (*ctxt).context,
                b"\0" as *const u8 as *const c_char,
            ),
        );
    } else {
        let mut i: c_int = 0 as c_int;
        match (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).type_0 as c_uint {
            1 | 2 => {
                if (**(*(*cur).nodesetval).nodeTab.offset(i as isize))
                    .ns
                    .is_null()
                {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewCString(
                            (*ctxt).context,
                            b"\0" as *const u8 as *const c_char,
                        ),
                    );
                } else {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewString(
                            (*ctxt).context,
                            (*(**(*(*cur).nodesetval).nodeTab.offset(i as isize)).ns).href,
                        ),
                    );
                }
            }
            _ => {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewCString(
                        (*ctxt).context,
                        b"\0" as *const u8 as *const c_char,
                    ),
                );
            }
        }
    }
    xmlXPathReleaseObject((*ctxt).context, cur);
} }
extern "C" fn xmlXPathNameFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if nargs == 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
        );
        nargs = 1 as c_int;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*(*ctxt).value).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    cur = valuePop(ctxt);
    if (*cur).nodesetval.is_null() || (*(*cur).nodesetval).nodeNr == 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheNewCString(
                (*ctxt).context,
                b"\0" as *const u8 as *const c_char,
            ),
        );
    } else {
        let mut i: c_int = 0 as c_int;
        match (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).type_0 as c_uint {
            1 | 2 => {
                if *(**(*(*cur).nodesetval).nodeTab.offset(i as isize))
                    .name
                    .offset(0 as c_int as isize)
                    as c_int
                    == ' ' as i32
                {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewCString(
                            (*ctxt).context,
                            b"\0" as *const u8 as *const c_char,
                        ),
                    );
                } else if (**(*(*cur).nodesetval).nodeTab.offset(i as isize))
                    .ns
                    .is_null()
                    || (*(**(*(*cur).nodesetval).nodeTab.offset(i as isize)).ns)
                        .prefix
                        .is_null()
                {
                    valuePush(
                        ctxt,
                        xmlXPathCacheNewString(
                            (*ctxt).context,
                            (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).name,
                        ),
                    );
                } else {
                    let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    fullname = xmlBuildQName(
                        (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).name,
                        (*(**(*(*cur).nodesetval).nodeTab.offset(i as isize)).ns).prefix,
                        ::core::ptr::null_mut::<xmlChar>(),
                        0 as c_int,
                    );
                    if fullname
                        == (**(*(*cur).nodesetval).nodeTab.offset(i as isize)).name as *mut xmlChar
                    {
                        fullname =
                            xmlStrdup((**(*(*cur).nodesetval).nodeTab.offset(i as isize)).name);
                    }
                    if fullname.is_null() {
                        xmlXPathPErrMemory(ctxt, ::core::ptr::null::<c_char>());
                    }
                    valuePush(ctxt, xmlXPathCacheWrapString((*ctxt).context, fullname));
                }
            }
            _ => {
                valuePush(
                    ctxt,
                    xmlXPathCacheNewNodeSet(
                        (*ctxt).context,
                        *(*(*cur).nodesetval).nodeTab.offset(i as isize),
                    ),
                );
                xmlXPathLocalNameFunction(ctxt, 1 as c_int);
            }
        }
    }
    xmlXPathReleaseObject((*ctxt).context, cur);
} }
#[no_mangle]
pub extern "C" fn xmlXPathStringFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheWrapString(
                (*ctxt).context,
                xmlXPathCastNodeToString((*(*ctxt).context).node),
            ),
        );
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    cur = valuePop(ctxt);
    if cur.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return;
    }
    valuePush(ctxt, xmlXPathCacheConvertString((*ctxt).context, cur));
} }
#[no_mangle]
pub extern "C" fn xmlXPathStringLengthFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if nargs == 0 as c_int {
        if ctxt.is_null() || (*ctxt).context.is_null() {
            return;
        }
        if (*(*ctxt).context).node.is_null() {
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(
                    (*ctxt).context,
                    0 as c_int as c_double,
                ),
            );
        } else {
            let mut content: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            content = xmlXPathCastNodeToString((*(*ctxt).context).node);
            valuePush(
                ctxt,
                xmlXPathCacheNewFloat(
                    (*ctxt).context,
                    xmlUTF8Strlen(content) as c_double,
                ),
            );
            xmlFree.expect("non-null function pointer")(content as *mut c_void);
        }
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    cur = valuePop(ctxt);
    valuePush(
        ctxt,
        xmlXPathCacheNewFloat(
            (*ctxt).context,
            xmlUTF8Strlen((*cur).stringval) as c_double,
        ),
    );
    xmlXPathReleaseObject((*ctxt).context, cur);
} }
#[no_mangle]
pub extern "C" fn xmlXPathConcatFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut newobj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if ctxt.is_null() {
        return;
    }
    if nargs < 2 as c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 2 as c_int {
            xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
            return;
        }
        if (*ctxt).valueNr < 2 as c_int {
            xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
            return;
        }
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    cur = valuePop(ctxt);
    if cur.is_null()
        || (*cur).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, cur);
        return;
    }
    nargs -= 1;
    while nargs > 0 as c_int {
        if !(*ctxt).value.is_null()
            && (*(*ctxt).value).type_0 as c_uint
                != XPATH_STRING as c_int as c_uint
        {
            xmlXPathStringFunction(ctxt, 1 as c_int);
        }
        newobj = valuePop(ctxt);
        if newobj.is_null()
            || (*newobj).type_0 as c_uint
                != XPATH_STRING as c_int as c_uint
        {
            xmlXPathReleaseObject((*ctxt).context, newobj);
            xmlXPathReleaseObject((*ctxt).context, cur);
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
            return;
        }
        tmp = xmlStrcat((*newobj).stringval, (*cur).stringval);
        (*newobj).stringval = (*cur).stringval;
        (*cur).stringval = tmp;
        xmlXPathReleaseObject((*ctxt).context, newobj);
        nargs -= 1;
    }
    valuePush(ctxt, cur);
} }
#[no_mangle]
pub extern "C" fn xmlXPathContainsFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut hay: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut needle: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 2 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    needle = valuePop(ctxt);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    hay = valuePop(ctxt);
    if hay.is_null()
        || (*hay).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, hay);
        xmlXPathReleaseObject((*ctxt).context, needle);
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    if !xmlStrstr((*hay).stringval, (*needle).stringval).is_null() {
        valuePush(
            ctxt,
            xmlXPathCacheNewBoolean((*ctxt).context, 1 as c_int),
        );
    } else {
        valuePush(
            ctxt,
            xmlXPathCacheNewBoolean((*ctxt).context, 0 as c_int),
        );
    }
    xmlXPathReleaseObject((*ctxt).context, hay);
    xmlXPathReleaseObject((*ctxt).context, needle);
} }
#[no_mangle]
pub extern "C" fn xmlXPathStartsWithFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut hay: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut needle: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut n: c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 2 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    needle = valuePop(ctxt);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    hay = valuePop(ctxt);
    if hay.is_null()
        || (*hay).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathReleaseObject((*ctxt).context, hay);
        xmlXPathReleaseObject((*ctxt).context, needle);
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    n = xmlStrlen((*needle).stringval);
    if xmlStrncmp((*hay).stringval, (*needle).stringval, n) != 0 {
        valuePush(
            ctxt,
            xmlXPathCacheNewBoolean((*ctxt).context, 0 as c_int),
        );
    } else {
        valuePush(
            ctxt,
            xmlXPathCacheNewBoolean((*ctxt).context, 1 as c_int),
        );
    }
    xmlXPathReleaseObject((*ctxt).context, hay);
    xmlXPathReleaseObject((*ctxt).context, needle);
} }
#[no_mangle]
pub extern "C" fn xmlXPathSubstringFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut str: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut start: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut len: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut le: c_double = 0 as c_int as c_double;
    let mut in_0: c_double = 0.;
    let mut i: c_int = 1 as c_int;
    let mut j: c_int = INT_MAX;
    if nargs < 2 as c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 2 as c_int {
            xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
            return;
        }
        if (*ctxt).valueNr < 2 as c_int {
            xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
            return;
        }
    }
    if nargs > 3 as c_int {
        if ctxt.is_null() {
            return;
        }
        if nargs != 3 as c_int {
            xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
            return;
        }
        if (*ctxt).valueNr < 3 as c_int {
            xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
            return;
        }
    }
    if nargs == 3 as c_int {
        if !(*ctxt).value.is_null()
            && (*(*ctxt).value).type_0 as c_uint
                != XPATH_NUMBER as c_int as c_uint
        {
            xmlXPathNumberFunction(ctxt, 1 as c_int);
        }
        if (*ctxt).value.is_null()
            || (*(*ctxt).value).type_0 as c_uint
                != XPATH_NUMBER as c_int as c_uint
        {
            xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
            return;
        }
        len = valuePop(ctxt);
        le = (*len).floatval;
        xmlXPathReleaseObject((*ctxt).context, len);
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    start = valuePop(ctxt);
    in_0 = (*start).floatval;
    xmlXPathReleaseObject((*ctxt).context, start);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    str = valuePop(ctxt);
    if !(in_0 < INT_MAX as c_double) {
        i = INT_MAX;
    } else if in_0 >= 1.0f64 {
        i = in_0 as c_int;
        if in_0 - floor(in_0) >= 0.5f64 {
            i += 1 as c_int;
        }
    }
    if nargs == 3 as c_int {
        let mut rin: c_double = 0.;
        let mut rle: c_double = 0.;
        let mut end: c_double = 0.;
        rin = floor(in_0);
        if in_0 - rin >= 0.5f64 {
            rin += 1.0f64;
        }
        rle = floor(le);
        if le - rle >= 0.5f64 {
            rle += 1.0f64;
        }
        end = rin + rle;
        if !(end >= 1.0f64) {
            j = 1 as c_int;
        } else if end < INT_MAX as c_double {
            j = end as c_int;
        }
    }
    if i < j {
        let mut ret: *mut xmlChar =
            xmlUTF8Strsub((*str).stringval, i - 1 as c_int, j - i);
        valuePush(ctxt, xmlXPathCacheNewString((*ctxt).context, ret));
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
    } else {
        valuePush(
            ctxt,
            xmlXPathCacheNewCString(
                (*ctxt).context,
                b"\0" as *const u8 as *const c_char,
            ),
        );
    }
    xmlXPathReleaseObject((*ctxt).context, str);
} }
#[no_mangle]
pub extern "C" fn xmlXPathSubstringBeforeFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut str: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut find: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut target: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut point: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut offset: c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 2 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    find = valuePop(ctxt);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    str = valuePop(ctxt);
    target = xmlBufCreate();
    if !target.is_null() {
        point = xmlStrstr((*str).stringval, (*find).stringval);
        if !point.is_null() {
            offset =
                point.offset_from((*str).stringval) as c_long as c_int;
            xmlBufAdd(target, (*str).stringval, offset);
        }
        valuePush(
            ctxt,
            xmlXPathCacheNewString((*ctxt).context, xmlBufContent(target as *const xmlBuf)),
        );
        xmlBufFree(target);
    }
    xmlXPathReleaseObject((*ctxt).context, str);
    xmlXPathReleaseObject((*ctxt).context, find);
} }
#[no_mangle]
pub extern "C" fn xmlXPathSubstringAfterFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut str: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut find: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut target: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut point: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut offset: c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 2 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    find = valuePop(ctxt);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    str = valuePop(ctxt);
    target = xmlBufCreate();
    if !target.is_null() {
        point = xmlStrstr((*str).stringval, (*find).stringval);
        if !point.is_null() {
            offset = (point.offset_from((*str).stringval) as c_long
                + xmlStrlen((*find).stringval) as c_long)
                as c_int;
            xmlBufAdd(
                target,
                (*str).stringval.offset(offset as isize) as *mut xmlChar,
                xmlStrlen((*str).stringval) - offset,
            );
        }
        valuePush(
            ctxt,
            xmlXPathCacheNewString((*ctxt).context, xmlBufContent(target as *const xmlBuf)),
        );
        xmlBufFree(target);
    }
    xmlXPathReleaseObject((*ctxt).context, str);
    xmlXPathReleaseObject((*ctxt).context, find);
} }
#[no_mangle]
pub extern "C" fn xmlXPathNormalizeFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut source: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut target: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut blank: c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as c_int {
        valuePush(
            ctxt,
            xmlXPathCacheWrapString(
                (*ctxt).context,
                xmlXPathCastNodeToString((*(*ctxt).context).node),
            ),
        );
        nargs = 1 as c_int;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    source = (*(*ctxt).value).stringval;
    if source.is_null() {
        return;
    }
    target = source;
    while *source as c_int == 0x20 as c_int
        || 0x9 as c_int <= *source as c_int
            && *source as c_int <= 0xa as c_int
        || *source as c_int == 0xd as c_int
    {
        source = source.offset(1);
    }
    blank = 0 as c_int;
    while *source != 0 {
        if *source as c_int == 0x20 as c_int
            || 0x9 as c_int <= *source as c_int
                && *source as c_int <= 0xa as c_int
            || *source as c_int == 0xd as c_int
        {
            blank = 1 as c_int;
        } else {
            if blank != 0 {
                let fresh40 = target;
                target = target.offset(1);
                *fresh40 = 0x20 as xmlChar;
                blank = 0 as c_int;
            }
            let fresh41 = target;
            target = target.offset(1);
            *fresh41 = *source;
        }
        source = source.offset(1);
    }
    *target = 0 as xmlChar;
} }
#[no_mangle]
pub extern "C" fn xmlXPathTranslateFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut str: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut from: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut to: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut target: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut offset: c_int = 0;
    let mut max: c_int = 0;
    let mut ch: c_int = 0;
    let mut point: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut cptr: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if ctxt.is_null() {
        return;
    }
    if nargs != 3 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 3 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    to = valuePop(ctxt);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    from = valuePop(ctxt);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    str = valuePop(ctxt);
    target = xmlBufCreate();
    if !target.is_null() {
        max = xmlUTF8Strlen((*to).stringval);
        cptr = (*str).stringval;
        loop {
            ch = *cptr as c_int;
            if !(ch != 0) {
                break;
            }
            offset = xmlUTF8Strloc((*from).stringval, cptr);
            if offset >= 0 as c_int {
                if offset < max {
                    point = xmlUTF8Strpos((*to).stringval, offset);
                    if !point.is_null() {
                        xmlBufAdd(
                            target,
                            point,
                            xmlUTF8Strsize(point, 1 as c_int),
                        );
                    }
                }
            } else {
                xmlBufAdd(target, cptr, xmlUTF8Strsize(cptr, 1 as c_int));
            }
            cptr = cptr.offset(1);
            if !(ch & 0x80 as c_int != 0) {
                continue;
            }
            if ch & 0xc0 as c_int != 0xc0 as c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathTranslateFunction: Invalid UTF8 string\n\0" as *const u8
                        as *const c_char,
                );
                break;
            } else {
                loop {
                    ch <<= 1 as c_int;
                    if !(ch & 0x80 as c_int != 0) {
                        break;
                    }
                    let fresh39 = cptr;
                    cptr = cptr.offset(1);
                    if !(*fresh39 as c_int & 0xc0 as c_int
                        != 0x80 as c_int)
                    {
                        continue;
                    }
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathTranslateFunction: Invalid UTF8 string\n\0" as *const u8
                            as *const c_char,
                    );
                    break;
                }
                if ch & 0x80 as c_int != 0 {
                    break;
                }
            }
        }
    }
    valuePush(
        ctxt,
        xmlXPathCacheNewString((*ctxt).context, xmlBufContent(target as *const xmlBuf)),
    );
    xmlBufFree(target);
    xmlXPathReleaseObject((*ctxt).context, str);
    xmlXPathReleaseObject((*ctxt).context, from);
    xmlXPathReleaseObject((*ctxt).context, to);
} }
#[no_mangle]
pub extern "C" fn xmlXPathBooleanFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    cur = valuePop(ctxt);
    if cur.is_null() {
        xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
        return;
    }
    cur = xmlXPathCacheConvertBoolean((*ctxt).context, cur);
    valuePush(ctxt, cur);
} }
#[no_mangle]
pub extern "C" fn xmlXPathNotFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_BOOLEAN as c_int as c_uint
    {
        xmlXPathBooleanFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_BOOLEAN as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    (*(*ctxt).value).boolval = ((*(*ctxt).value).boolval == 0) as c_int;
} }
#[no_mangle]
pub extern "C" fn xmlXPathTrueFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 0 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    valuePush(
        ctxt,
        xmlXPathCacheNewBoolean((*ctxt).context, 1 as c_int),
    );
} }
#[no_mangle]
pub extern "C" fn xmlXPathFalseFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if nargs != 0 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 0 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    valuePush(
        ctxt,
        xmlXPathCacheNewBoolean((*ctxt).context, 0 as c_int),
    );
} }
#[no_mangle]
pub extern "C" fn xmlXPathLangFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut current_block: u64;
    let mut val: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut theLang: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut lang: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: c_int = 0 as c_int;
    let mut i: c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    val = valuePop(ctxt);
    lang = (*val).stringval;
    theLang = xmlNodeGetLang((*(*ctxt).context).node as *const xmlNode);
    if !theLang.is_null() && !lang.is_null() {
        i = 0 as c_int;
        loop {
            if !(*lang.offset(i as isize) as c_int != 0 as c_int) {
                current_block = 224731115979188411;
                break;
            }
            if ({
                let mut __res: c_int = 0;
                if ::core::mem::size_of::<xmlChar>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: c_int =
                            *lang.offset(i as isize) as c_int;
                        __res = (if __c < -(128 as c_int)
                            || __c > 255 as c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as c_int;
                    } else {
                        __res = toupper(*lang.offset(i as isize) as c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*lang.offset(i as isize) as c_int as isize)
                        as c_int;
                }
                __res
            }) != ({
                let mut __res: c_int = 0;
                if ::core::mem::size_of::<xmlChar>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: c_int =
                            *theLang.offset(i as isize) as c_int;
                        __res = (if __c < -(128 as c_int)
                            || __c > 255 as c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as c_int;
                    } else {
                        __res = toupper(*theLang.offset(i as isize) as c_int);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(*theLang.offset(i as isize) as c_int as isize)
                        as c_int;
                }
                __res
            }) {
                current_block = 11586683935931013361;
                break;
            }
            i += 1;
        }
        match current_block {
            11586683935931013361 => {}
            _ => {
                if *theLang.offset(i as isize) as c_int == 0 as c_int
                    || *theLang.offset(i as isize) as c_int == '-' as i32
                {
                    ret = 1 as c_int;
                }
            }
        }
    }
    if !theLang.is_null() {
        xmlFree.expect("non-null function pointer")(theLang as *mut c_void);
    }
    xmlXPathReleaseObject((*ctxt).context, val);
    valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, ret));
} }
#[no_mangle]
pub extern "C" fn xmlXPathNumberFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut res: c_double = 0.;
    if ctxt.is_null() {
        return;
    }
    if nargs == 0 as c_int {
        if (*(*ctxt).context).node.is_null() {
            valuePush(ctxt, xmlXPathCacheNewFloat((*ctxt).context, 0.0f64));
        } else {
            let mut content: *mut xmlChar =
                xmlNodeGetContent((*(*ctxt).context).node as *const xmlNode);
            res = xmlXPathStringEvalNumber(content);
            valuePush(ctxt, xmlXPathCacheNewFloat((*ctxt).context, res));
            xmlFree.expect("non-null function pointer")(content as *mut c_void);
        }
        return;
    }
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    cur = valuePop(ctxt);
    valuePush(ctxt, xmlXPathCacheConvertNumber((*ctxt).context, cur));
} }
#[no_mangle]
pub extern "C" fn xmlXPathSumFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut cur: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut i: c_int = 0;
    let mut res: c_double = 0.0f64;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
            && (*(*ctxt).value).type_0 as c_uint
                != XPATH_XSLT_TREE as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    cur = valuePop(ctxt);
    if !(*cur).nodesetval.is_null() && (*(*cur).nodesetval).nodeNr != 0 as c_int {
        i = 0 as c_int;
        while i < (*(*cur).nodesetval).nodeNr {
            res += xmlXPathCastNodeToNumber(*(*(*cur).nodesetval).nodeTab.offset(i as isize));
            i += 1;
        }
    }
    valuePush(ctxt, xmlXPathCacheNewFloat((*ctxt).context, res));
    xmlXPathReleaseObject((*ctxt).context, cur);
} }
#[no_mangle]
pub extern "C" fn xmlXPathFloorFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    (*(*ctxt).value).floatval = floor((*(*ctxt).value).floatval);
} }
#[no_mangle]
pub extern "C" fn xmlXPathCeilingFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    (*(*ctxt).value).floatval = ceil((*(*ctxt).value).floatval);
} }
#[no_mangle]
pub extern "C" fn xmlXPathRoundFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut f: c_double = 0.;
    if ctxt.is_null() {
        return;
    }
    if nargs != 1 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 1 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathNumberFunction(ctxt, 1 as c_int);
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NUMBER as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return;
    }
    f = (*(*ctxt).value).floatval;
    if f >= -0.5f64 && f < 0.5f64 {
        (*(*ctxt).value).floatval *= 0.0f64;
    } else {
        let mut rounded: c_double = floor(f);
        if f - rounded >= 0.5f64 {
            rounded += 1.0f64;
        }
        (*(*ctxt).value).floatval = rounded;
    };
} }
unsafe fn xmlXPathCurrentChar(
    mut ctxt: xmlXPathParserContextPtr,
    mut len: *mut c_int,
) -> c_int {
    let mut current_block: u64;
    let mut c: c_uchar = 0;
    let mut val: c_uint = 0;
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if ctxt.is_null() {
        return 0 as c_int;
    }
    cur = (*ctxt).cur;
    c = *cur as c_uchar;
    if c as c_int & 0x80 as c_int != 0 {
        if !(*cur.offset(1 as c_int as isize) as c_int
            & 0xc0 as c_int
            != 0x80 as c_int)
        {
            if c as c_int & 0xe0 as c_int == 0xe0 as c_int {
                if *cur.offset(2 as c_int as isize) as c_int
                    & 0xc0 as c_int
                    != 0x80 as c_int
                {
                    current_block = 4443643870453801414;
                } else if c as c_int & 0xf0 as c_int
                    == 0xf0 as c_int
                {
                    if c as c_int & 0xf8 as c_int
                        != 0xf0 as c_int
                        || *cur.offset(3 as c_int as isize) as c_int
                            & 0xc0 as c_int
                            != 0x80 as c_int
                    {
                        current_block = 4443643870453801414;
                    } else {
                        *len = 4 as c_int;
                        val = ((*cur.offset(0 as c_int as isize)
                            as c_int
                            & 0x7 as c_int)
                            << 18 as c_int)
                            as c_uint;
                        val |= ((*cur.offset(1 as c_int as isize)
                            as c_int
                            & 0x3f as c_int)
                            << 12 as c_int)
                            as c_uint;
                        val |= ((*cur.offset(2 as c_int as isize)
                            as c_int
                            & 0x3f as c_int)
                            << 6 as c_int)
                            as c_uint;
                        val |= (*cur.offset(3 as c_int as isize) as c_int
                            & 0x3f as c_int)
                            as c_uint;
                        current_block = 7175849428784450219;
                    }
                } else {
                    *len = 3 as c_int;
                    val = ((*cur.offset(0 as c_int as isize) as c_int
                        & 0xf as c_int)
                        << 12 as c_int)
                        as c_uint;
                    val |= ((*cur.offset(1 as c_int as isize) as c_int
                        & 0x3f as c_int)
                        << 6 as c_int)
                        as c_uint;
                    val |= (*cur.offset(2 as c_int as isize) as c_int
                        & 0x3f as c_int)
                        as c_uint;
                    current_block = 7175849428784450219;
                }
            } else {
                *len = 2 as c_int;
                val = ((*cur.offset(0 as c_int as isize) as c_int
                    & 0x1f as c_int)
                    << 6 as c_int) as c_uint;
                val |= (*cur.offset(1 as c_int as isize) as c_int
                    & 0x3f as c_int) as c_uint;
                current_block = 7175849428784450219;
            }
            match current_block {
                4443643870453801414 => {}
                _ => {
                    if if val < 0x100 as c_uint {
                        (0x9 as c_uint <= val && val <= 0xa as c_uint
                            || val == 0xd as c_uint
                            || 0x20 as c_uint <= val)
                            as c_int
                    } else {
                        (0x100 as c_uint <= val
                            && val <= 0xd7ff as c_uint
                            || 0xe000 as c_uint <= val
                                && val <= 0xfffd as c_uint
                            || 0x10000 as c_int as c_uint <= val
                                && val <= 0x10ffff as c_int as c_uint)
                            as c_int
                    } == 0
                    {
                        xmlXPathErr(ctxt, XPATH_INVALID_CHAR_ERROR as c_int);
                        return 0 as c_int;
                    }
                    return val as c_int;
                }
            }
        }
        *len = 0 as c_int;
        xmlXPathErr(ctxt, XPATH_ENCODING_ERROR as c_int);
        return 0 as c_int;
    } else {
        *len = 1 as c_int;
        return *cur as c_int;
    };
}
#[no_mangle]
pub extern "C" fn xmlXPathParseNCName(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar { unsafe {
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut count: c_int = 0 as c_int;
    if ctxt.is_null() || (*ctxt).cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    in_0 = (*ctxt).cur;
    if *in_0 as c_int >= 0x61 as c_int
        && *in_0 as c_int <= 0x7a as c_int
        || *in_0 as c_int >= 0x41 as c_int
            && *in_0 as c_int <= 0x5a as c_int
        || *in_0 as c_int == '_' as i32
    {
        in_0 = in_0.offset(1);
        while *in_0 as c_int >= 0x61 as c_int
            && *in_0 as c_int <= 0x7a as c_int
            || *in_0 as c_int >= 0x41 as c_int
                && *in_0 as c_int <= 0x5a as c_int
            || *in_0 as c_int >= 0x30 as c_int
                && *in_0 as c_int <= 0x39 as c_int
            || *in_0 as c_int == '_' as i32
            || *in_0 as c_int == '.' as i32
            || *in_0 as c_int == '-' as i32
        {
            in_0 = in_0.offset(1);
        }
        if *in_0 as c_int == ' ' as i32
            || *in_0 as c_int == '>' as i32
            || *in_0 as c_int == '/' as i32
            || *in_0 as c_int == '[' as i32
            || *in_0 as c_int == ']' as i32
            || *in_0 as c_int == ':' as i32
            || *in_0 as c_int == '@' as i32
            || *in_0 as c_int == '*' as i32
        {
            count = in_0.offset_from((*ctxt).cur) as c_long as c_int;
            if count == 0 as c_int {
                return ::core::ptr::null_mut::<xmlChar>();
            }
            ret = xmlStrndup((*ctxt).cur, count);
            (*ctxt).cur = in_0;
            return ret;
        }
    }
    return xmlXPathParseNameComplex(ctxt, 0 as c_int);
} }
unsafe fn xmlXPathParseQName(
    mut ctxt: xmlXPathParserContextPtr,
    mut prefix: *mut *mut xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    *prefix = ::core::ptr::null_mut::<xmlChar>();
    ret = xmlXPathParseNCName(ctxt);
    if !ret.is_null() && *(*ctxt).cur as c_int == ':' as i32 {
        *prefix = ret;
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        ret = xmlXPathParseNCName(ctxt);
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn xmlXPathParseName(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar { unsafe {
    let mut in_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut count: size_t = 0 as size_t;
    if ctxt.is_null() || (*ctxt).cur.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    in_0 = (*ctxt).cur;
    if *in_0 as c_int >= 0x61 as c_int
        && *in_0 as c_int <= 0x7a as c_int
        || *in_0 as c_int >= 0x41 as c_int
            && *in_0 as c_int <= 0x5a as c_int
        || *in_0 as c_int == '_' as i32
        || *in_0 as c_int == ':' as i32
    {
        in_0 = in_0.offset(1);
        while *in_0 as c_int >= 0x61 as c_int
            && *in_0 as c_int <= 0x7a as c_int
            || *in_0 as c_int >= 0x41 as c_int
                && *in_0 as c_int <= 0x5a as c_int
            || *in_0 as c_int >= 0x30 as c_int
                && *in_0 as c_int <= 0x39 as c_int
            || *in_0 as c_int == '_' as i32
            || *in_0 as c_int == '-' as i32
            || *in_0 as c_int == ':' as i32
            || *in_0 as c_int == '.' as i32
        {
            in_0 = in_0.offset(1);
        }
        if *in_0 as c_int > 0 as c_int
            && (*in_0 as c_int) < 0x80 as c_int
        {
            count = in_0.offset_from((*ctxt).cur) as c_long as size_t;
            if count > XML_MAX_NAME_LENGTH as size_t {
                (*ctxt).cur = in_0;
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            ret = xmlStrndup((*ctxt).cur, count as c_int);
            (*ctxt).cur = in_0;
            return ret;
        }
    }
    return xmlXPathParseNameComplex(ctxt, 1 as c_int);
} }
fn xmlXPathParseNameComplex(
    mut ctxt: xmlXPathParserContextPtr,
    mut qualified: c_int,
) -> *mut xmlChar { unsafe {
    let mut buf: [xmlChar; 105] = [0; 105];
    let mut len: c_int = 0 as c_int;
    let mut l: c_int = 0;
    let mut c: c_int = 0;
    c = xmlXPathCurrentChar(ctxt, &raw mut l);
    if c == ' ' as i32
        || c == '>' as i32
        || c == '/' as i32
        || c == '[' as i32
        || c == ']' as i32
        || c == '@' as i32
        || c == '*' as i32
        || !((if c < 0x100 as c_int {
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
            && (qualified == 0 || c != ':' as i32)
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && ((if c < 0x100 as c_int {
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
            || qualified != 0 && c == ':' as i32
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
        if l == 1 as c_int {
            let fresh74 = len;
            len = len + 1;
            buf[fresh74 as usize] = c as xmlChar;
        } else {
            len += xmlCopyChar(
                l,
                (&raw mut buf as *mut xmlChar).offset(len as isize) as *mut xmlChar,
                c,
            );
        }
        (*ctxt).cur = (*ctxt).cur.offset(l as isize);
        c = xmlXPathCurrentChar(ctxt, &raw mut l);
        if len >= XML_MAX_NAMELEN {
            let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut max: c_int = len * 2 as c_int;
            if len > XML_MAX_NAME_LENGTH {
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            buffer =
                xmlMallocAtomic.expect("non-null function pointer")(max as size_t) as *mut xmlChar;
            if buffer.is_null() {
                xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as c_int);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            memcpy(
                buffer as *mut c_void,
                &raw mut buf as *mut xmlChar as *const c_void,
                len as size_t,
            );
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
                || qualified != 0 && c == ':' as i32
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
                if len + 10 as c_int > max {
                    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    if max > XML_MAX_NAME_LENGTH {
                        xmlFree.expect("non-null function pointer")(
                            buffer as *mut c_void,
                        );
                        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
                        return ::core::ptr::null_mut::<xmlChar>();
                    }
                    max *= 2 as c_int;
                    tmp = xmlRealloc.expect("non-null function pointer")(
                        buffer as *mut c_void,
                        max as size_t,
                    ) as *mut xmlChar;
                    if tmp.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            buffer as *mut c_void,
                        );
                        xmlXPathErr(ctxt, XPATH_MEMORY_ERROR as c_int);
                        return ::core::ptr::null_mut::<xmlChar>();
                    }
                    buffer = tmp;
                }
                if l == 1 as c_int {
                    let fresh75 = len;
                    len = len + 1;
                    *buffer.offset(fresh75 as isize) = c as xmlChar;
                } else {
                    len += xmlCopyChar(l, buffer.offset(len as isize) as *mut xmlChar, c);
                }
                (*ctxt).cur = (*ctxt).cur.offset(l as isize);
                c = xmlXPathCurrentChar(ctxt, &raw mut l);
            }
            *buffer.offset(len as isize) = 0 as xmlChar;
            return buffer;
        }
    }
    if len == 0 as c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStrndup(&raw mut buf as *mut xmlChar, len);
} }
pub const MAX_FRAC: c_int = 20 as c_int;
#[inline]
pub unsafe fn xmlXPathStringEvalNumber(
    mut str: *const xmlChar,
) -> c_double {
    let mut cur: *const xmlChar = str;
    let mut ret: c_double = 0.;
    let mut ok: c_int = 0 as c_int;
    let mut isneg: c_int = 0 as c_int;
    let mut exponent: c_int = 0 as c_int;
    let mut is_exponent_negative: c_int = 0 as c_int;
    let mut tmp: c_ulong = 0 as c_ulong;
    let mut temp: c_double = 0.;
    if cur.is_null() {
        return 0 as c_int as c_double;
    }
    while *cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *cur as c_int
            && *cur as c_int <= 0xa as c_int
        || *cur as c_int == 0xd as c_int
    {
        cur = cur.offset(1);
    }
    if *cur as c_int == '-' as i32 {
        isneg = 1 as c_int;
        cur = cur.offset(1);
    }
    if *cur as c_int != '.' as i32
        && ((*cur as c_int) < '0' as i32 || *cur as c_int > '9' as i32)
    {
        return xmlXPathNAN;
    }
    ret = 0 as c_int as c_double;
    while *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
        ret = ret * 10 as c_int as c_double;
        tmp = (*cur as c_int - '0' as i32) as c_ulong;
        ok = 1 as c_int;
        cur = cur.offset(1);
        temp = tmp as c_double;
        ret = ret + temp;
    }
    if *cur as c_int == '.' as i32 {
        let mut v: c_int = 0;
        let mut frac: c_int = 0 as c_int;
        let mut max: c_int = 0;
        let mut fraction: c_double = 0 as c_int as c_double;
        cur = cur.offset(1);
        if ((*cur as c_int) < '0' as i32 || *cur as c_int > '9' as i32)
            && ok == 0
        {
            return xmlXPathNAN;
        }
        while *cur as c_int == '0' as i32 {
            frac = frac + 1 as c_int;
            cur = cur.offset(1);
        }
        max = frac + MAX_FRAC;
        while *cur as c_int >= '0' as i32
            && *cur as c_int <= '9' as i32
            && frac < max
        {
            v = *cur as c_int - '0' as i32;
            fraction = fraction * 10 as c_int as c_double
                + v as c_double;
            frac = frac + 1 as c_int;
            cur = cur.offset(1);
        }
        fraction /= pow(10.0f64, frac as c_double);
        ret = ret + fraction;
        while *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
            cur = cur.offset(1);
        }
    }
    if *cur as c_int == 'e' as i32 || *cur as c_int == 'E' as i32 {
        cur = cur.offset(1);
        if *cur as c_int == '-' as i32 {
            is_exponent_negative = 1 as c_int;
            cur = cur.offset(1);
        } else if *cur as c_int == '+' as i32 {
            cur = cur.offset(1);
        }
        while *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
            if exponent < 1000000 as c_int {
                exponent =
                    exponent * 10 as c_int + (*cur as c_int - '0' as i32);
            }
            cur = cur.offset(1);
        }
    }
    while *cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *cur as c_int
            && *cur as c_int <= 0xa as c_int
        || *cur as c_int == 0xd as c_int
    {
        cur = cur.offset(1);
    }
    if *cur as c_int != 0 as c_int {
        return xmlXPathNAN;
    }
    if isneg != 0 {
        ret = -ret;
    }
    if is_exponent_negative != 0 {
        exponent = -exponent;
    }
    ret *= pow(10.0f64, exponent as c_double);
    return ret;
}
fn xmlXPathCompNumber(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut ret: c_double = 0.0f64;
    let mut ok: c_int = 0 as c_int;
    let mut exponent: c_int = 0 as c_int;
    let mut is_exponent_negative: c_int = 0 as c_int;
    let mut num: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut tmp: c_ulong = 0 as c_ulong;
    let mut temp: c_double = 0.;
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    if *(*ctxt).cur as c_int != '.' as i32
        && ((*(*ctxt).cur as c_int) < '0' as i32
            || *(*ctxt).cur as c_int > '9' as i32)
    {
        xmlXPathErr(ctxt, XPATH_NUMBER_ERROR as c_int);
        return;
    }
    ret = 0 as c_int as c_double;
    while *(*ctxt).cur as c_int >= '0' as i32
        && *(*ctxt).cur as c_int <= '9' as i32
    {
        ret = ret * 10 as c_int as c_double;
        tmp = (*(*ctxt).cur as c_int - '0' as i32) as c_ulong;
        ok = 1 as c_int;
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        temp = tmp as c_double;
        ret = ret + temp;
    }
    if *(*ctxt).cur as c_int == '.' as i32 {
        let mut v: c_int = 0;
        let mut frac: c_int = 0 as c_int;
        let mut max: c_int = 0;
        let mut fraction: c_double = 0 as c_int as c_double;
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        if ((*(*ctxt).cur as c_int) < '0' as i32
            || *(*ctxt).cur as c_int > '9' as i32)
            && ok == 0
        {
            xmlXPathErr(ctxt, XPATH_NUMBER_ERROR as c_int);
            return;
        }
        while *(*ctxt).cur as c_int == '0' as i32 {
            frac = frac + 1 as c_int;
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        max = frac + MAX_FRAC;
        while *(*ctxt).cur as c_int >= '0' as i32
            && *(*ctxt).cur as c_int <= '9' as i32
            && frac < max
        {
            v = *(*ctxt).cur as c_int - '0' as i32;
            fraction = fraction * 10 as c_int as c_double
                + v as c_double;
            frac = frac + 1 as c_int;
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        fraction /= pow(10.0f64, frac as c_double);
        ret = ret + fraction;
        while *(*ctxt).cur as c_int >= '0' as i32
            && *(*ctxt).cur as c_int <= '9' as i32
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
    if *(*ctxt).cur as c_int == 'e' as i32
        || *(*ctxt).cur as c_int == 'E' as i32
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        if *(*ctxt).cur as c_int == '-' as i32 {
            is_exponent_negative = 1 as c_int;
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        } else if *(*ctxt).cur as c_int == '+' as i32 {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        while *(*ctxt).cur as c_int >= '0' as i32
            && *(*ctxt).cur as c_int <= '9' as i32
        {
            if exponent < 1000000 as c_int {
                exponent = exponent * 10 as c_int
                    + (*(*ctxt).cur as c_int - '0' as i32);
            }
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        if is_exponent_negative != 0 {
            exponent = -exponent;
        }
        ret *= pow(10.0f64, exponent as c_double);
    }
    num = xmlXPathCacheNewFloat((*ctxt).context, ret);
    if num.is_null() {
        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
    } else if xmlXPathCompExprAdd(
        ctxt,
        (*(*ctxt).comp).last,
        -(1 as c_int),
        XPATH_OP_VALUE,
        XPATH_NUMBER as c_int,
        0 as c_int,
        0 as c_int,
        num as *mut c_void,
        ::core::ptr::null_mut::<c_void>(),
    ) == -(1 as c_int)
    {
        xmlXPathReleaseObject((*ctxt).context, num);
    }
} }
fn xmlXPathParseLiteral(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar { unsafe {
    let mut q: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if *(*ctxt).cur as c_int == '"' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        q = (*ctxt).cur;
        while (0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
            || 0x20 as c_int <= *(*ctxt).cur as c_int)
            && *(*ctxt).cur as c_int != '"' as i32
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        if !(0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
            || 0x20 as c_int <= *(*ctxt).cur as c_int)
        {
            xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as c_int);
            return ::core::ptr::null_mut::<xmlChar>();
        } else {
            ret = xmlStrndup(
                q,
                (*ctxt).cur.offset_from(q) as c_long as c_int,
            );
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    } else if *(*ctxt).cur as c_int == '\'' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        q = (*ctxt).cur;
        while (0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
            || 0x20 as c_int <= *(*ctxt).cur as c_int)
            && *(*ctxt).cur as c_int != '\'' as i32
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        if !(0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
            || 0x20 as c_int <= *(*ctxt).cur as c_int)
        {
            xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as c_int);
            return ::core::ptr::null_mut::<xmlChar>();
        } else {
            ret = xmlStrndup(
                q,
                (*ctxt).cur.offset_from(q) as c_long as c_int,
            );
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    } else {
        xmlXPathErr(ctxt, XPATH_START_LITERAL_ERROR as c_int);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return ret;
} }
fn xmlXPathCompLiteral(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut q: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut lit: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if *(*ctxt).cur as c_int == '"' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        q = (*ctxt).cur;
        while (0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
            || 0x20 as c_int <= *(*ctxt).cur as c_int)
            && *(*ctxt).cur as c_int != '"' as i32
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        if !(0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
            || 0x20 as c_int <= *(*ctxt).cur as c_int)
        {
            xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as c_int);
            return;
        } else {
            ret = xmlStrndup(
                q,
                (*ctxt).cur.offset_from(q) as c_long as c_int,
            );
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    } else if *(*ctxt).cur as c_int == '\'' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        q = (*ctxt).cur;
        while (0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
            || 0x20 as c_int <= *(*ctxt).cur as c_int)
            && *(*ctxt).cur as c_int != '\'' as i32
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        if !(0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
            || 0x20 as c_int <= *(*ctxt).cur as c_int)
        {
            xmlXPathErr(ctxt, XPATH_UNFINISHED_LITERAL_ERROR as c_int);
            return;
        } else {
            ret = xmlStrndup(
                q,
                (*ctxt).cur.offset_from(q) as c_long as c_int,
            );
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    } else {
        xmlXPathErr(ctxt, XPATH_START_LITERAL_ERROR as c_int);
        return;
    }
    if ret.is_null() {
        xmlXPathPErrMemory(ctxt, ::core::ptr::null::<c_char>());
        return;
    }
    lit = xmlXPathCacheNewString((*ctxt).context, ret);
    if lit.is_null() {
        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
    } else if xmlXPathCompExprAdd(
        ctxt,
        (*(*ctxt).comp).last,
        -(1 as c_int),
        XPATH_OP_VALUE,
        XPATH_STRING as c_int,
        0 as c_int,
        0 as c_int,
        lit as *mut c_void,
        ::core::ptr::null_mut::<c_void>(),
    ) == -(1 as c_int)
    {
        xmlXPathReleaseObject((*ctxt).context, lit);
    }
    xmlFree.expect("non-null function pointer")(ret as *mut c_void);
} }
fn xmlXPathCompVariableReference(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int != '$' as i32 {
        xmlXPathErr(ctxt, XPATH_VARIABLE_REF_ERROR as c_int);
        return;
    }
    if *(*ctxt).cur as c_int != 0 {
        (*ctxt).cur = (*ctxt).cur.offset(1);
    } else {
    };
    name = xmlXPathParseQName(ctxt, &raw mut prefix);
    if name.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        xmlXPathErr(ctxt, XPATH_VARIABLE_REF_ERROR as c_int);
        return;
    }
    (*(*ctxt).comp).last = -(1 as c_int);
    if xmlXPathCompExprAdd(
        ctxt,
        (*(*ctxt).comp).last,
        -(1 as c_int),
        XPATH_OP_VARIABLE,
        0 as c_int,
        0 as c_int,
        0 as c_int,
        name as *mut c_void,
        prefix as *mut c_void,
    ) == -(1 as c_int)
    {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        xmlFree.expect("non-null function pointer")(name as *mut c_void);
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if !(*ctxt).context.is_null() && (*(*ctxt).context).flags & XML_XPATH_NOVAR != 0 {
        xmlXPathErr(ctxt, XPATH_FORBID_VARIABLE_ERROR as c_int);
        return;
    }
} }
#[inline]
pub unsafe fn xmlXPathIsNodeType(mut name: *const xmlChar) -> c_int {
    if name.is_null() {
        return 0 as c_int;
    }
    if xmlStrEqual(
        name,
        b"node\0" as *const u8 as *const c_char as *mut xmlChar,
    ) != 0
    {
        return 1 as c_int;
    }
    if xmlStrEqual(
        name,
        b"text\0" as *const u8 as *const c_char as *mut xmlChar,
    ) != 0
    {
        return 1 as c_int;
    }
    if xmlStrEqual(
        name,
        b"comment\0" as *const u8 as *const c_char as *mut xmlChar,
    ) != 0
    {
        return 1 as c_int;
    }
    if xmlStrEqual(
        name,
        b"processing-instruction\0" as *const u8 as *const c_char as *mut xmlChar,
    ) != 0
    {
        return 1 as c_int;
    }
    return 0 as c_int;
}
fn xmlXPathCompFunctionCall(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut nbargs: c_int = 0 as c_int;
    let mut sort: c_int = 1 as c_int;
    name = xmlXPathParseQName(ctxt, &raw mut prefix);
    if name.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int != '(' as i32 {
        xmlFree.expect("non-null function pointer")(name as *mut c_void);
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
        return;
    }
    if *(*ctxt).cur as c_int != 0 {
        (*ctxt).cur = (*ctxt).cur.offset(1);
    } else {
    };
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if prefix.is_null()
        && *name.offset(0 as c_int as isize) as c_int == 'c' as i32
        && xmlStrEqual(
            name,
            b"count\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
    {
        sort = 0 as c_int;
    }
    (*(*ctxt).comp).last = -(1 as c_int);
    if *(*ctxt).cur as c_int != ')' as i32 {
        while *(*ctxt).cur as c_int != 0 as c_int {
            let mut op1: c_int = (*(*ctxt).comp).last;
            (*(*ctxt).comp).last = -(1 as c_int);
            xmlXPathCompileExpr(ctxt, sort);
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                xmlFree.expect("non-null function pointer")(name as *mut c_void);
                xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
                return;
            }
            xmlXPathCompExprAdd(
                ctxt,
                op1,
                (*(*ctxt).comp).last,
                XPATH_OP_ARG,
                0 as c_int,
                0 as c_int,
                0 as c_int,
                NULL,
                NULL,
            );
            nbargs += 1;
            if *(*ctxt).cur as c_int == ')' as i32 {
                break;
            }
            if *(*ctxt).cur as c_int != ',' as i32 {
                xmlFree.expect("non-null function pointer")(name as *mut c_void);
                xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
                xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
                return;
            }
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
            while *(*ctxt).cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *(*ctxt).cur as c_int
                    && *(*ctxt).cur as c_int <= 0xa as c_int
                || *(*ctxt).cur as c_int == 0xd as c_int
            {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
            }
        }
    }
    if xmlXPathCompExprAdd(
        ctxt,
        (*(*ctxt).comp).last,
        -(1 as c_int),
        XPATH_OP_FUNCTION,
        nbargs,
        0 as c_int,
        0 as c_int,
        name as *mut c_void,
        prefix as *mut c_void,
    ) == -(1 as c_int)
    {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        xmlFree.expect("non-null function pointer")(name as *mut c_void);
    }
    if *(*ctxt).cur as c_int != 0 {
        (*ctxt).cur = (*ctxt).cur.offset(1);
    } else {
    };
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
} }
fn xmlXPathCompPrimaryExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int == '$' as i32 {
        xmlXPathCompVariableReference(ctxt);
    } else if *(*ctxt).cur as c_int == '(' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompileExpr(ctxt, 1 as c_int);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        if *(*ctxt).cur as c_int != ')' as i32 {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
            return;
        }
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    } else if 0x30 as c_int <= *(*ctxt).cur as c_int
        && *(*ctxt).cur as c_int <= 0x39 as c_int
        || *(*ctxt).cur as c_int == '.' as i32
            && (0x30 as c_int
                <= *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                && *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                    <= 0x39 as c_int)
    {
        xmlXPathCompNumber(ctxt);
    } else if *(*ctxt).cur as c_int == '\'' as i32
        || *(*ctxt).cur as c_int == '"' as i32
    {
        xmlXPathCompLiteral(ctxt);
    } else {
        xmlXPathCompFunctionCall(ctxt);
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
} }
fn xmlXPathCompFilterExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    xmlXPathCompPrimaryExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == '[' as i32 {
        xmlXPathCompPredicate(ctxt, 1 as c_int);
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
} }
fn xmlXPathScanName(mut ctxt: xmlXPathParserContextPtr) -> *mut xmlChar { unsafe {
    let mut l: c_int = 0;
    let mut c: c_int = 0;
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    cur = (*ctxt).cur;
    c = xmlXPathCurrentChar(ctxt, &raw mut l);
    if c == ' ' as i32
        || c == '>' as i32
        || c == '/' as i32
        || !((if c < 0x100 as c_int {
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
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while c != ' ' as i32
        && c != '>' as i32
        && c != '/' as i32
        && ((if c < 0x100 as c_int {
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
            || c == ':' as i32
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
        (*ctxt).cur = (*ctxt).cur.offset(l as isize);
        c = xmlXPathCurrentChar(ctxt, &raw mut l);
    }
    ret = xmlStrndup(
        cur,
        (*ctxt).cur.offset_from(cur) as c_long as c_int,
    );
    (*ctxt).cur = cur;
    return ret;
} }
fn xmlXPathCompPathExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut lc: c_int = 1 as c_int;
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int == '$' as i32
        || *(*ctxt).cur as c_int == '(' as i32
        || 0x30 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0x39 as c_int
        || *(*ctxt).cur as c_int == '\'' as i32
        || *(*ctxt).cur as c_int == '"' as i32
        || *(*ctxt).cur as c_int == '.' as i32
            && (0x30 as c_int
                <= *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                && *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                    <= 0x39 as c_int)
    {
        lc = 0 as c_int;
    } else if *(*ctxt).cur as c_int == '*' as i32 {
        lc = 1 as c_int;
    } else if *(*ctxt).cur as c_int == '/' as i32 {
        lc = 1 as c_int;
    } else if *(*ctxt).cur as c_int == '@' as i32 {
        lc = 1 as c_int;
    } else if *(*ctxt).cur as c_int == '.' as i32 {
        lc = 1 as c_int;
    } else {
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        name = xmlXPathScanName(ctxt);
        if !name.is_null()
            && !xmlStrstr(
                name,
                b"::\0" as *const u8 as *const c_char as *mut xmlChar,
            )
            .is_null()
        {
            lc = 1 as c_int;
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
        } else if !name.is_null() {
            let mut len: c_int = xmlStrlen(name);
            while *(*ctxt).cur.offset(len as isize) as c_int != 0 as c_int
            {
                if *(*ctxt).cur.offset(len as isize) as c_int == '/' as i32 {
                    lc = 1 as c_int;
                    break;
                } else if *(*ctxt).cur.offset(len as isize) as c_int
                    == 0x20 as c_int
                    || 0x9 as c_int
                        <= *(*ctxt).cur.offset(len as isize) as c_int
                        && *(*ctxt).cur.offset(len as isize) as c_int
                            <= 0xa as c_int
                    || *(*ctxt).cur.offset(len as isize) as c_int
                        == 0xd as c_int
                {
                    len += 1;
                } else if *(*ctxt).cur.offset(len as isize) as c_int == ':' as i32 {
                    lc = 1 as c_int;
                    break;
                } else if *(*ctxt).cur.offset(len as isize) as c_int == '(' as i32 {
                    if xmlXPathIsNodeType(name) != 0 {
                        lc = 1 as c_int;
                    } else {
                        lc = 0 as c_int;
                    }
                    break;
                } else if *(*ctxt).cur.offset(len as isize) as c_int == '[' as i32 {
                    lc = 1 as c_int;
                    break;
                } else if *(*ctxt).cur.offset(len as isize) as c_int == '<' as i32
                    || *(*ctxt).cur.offset(len as isize) as c_int == '>' as i32
                    || *(*ctxt).cur.offset(len as isize) as c_int == '=' as i32
                {
                    lc = 1 as c_int;
                    break;
                } else {
                    lc = 1 as c_int;
                    break;
                }
            }
            if *(*ctxt).cur.offset(len as isize) as c_int == 0 as c_int {
                lc = 1 as c_int;
            }
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
        } else {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
            return;
        }
    }
    if lc != 0 {
        if *(*ctxt).cur as c_int == '/' as i32 {
            xmlXPathCompExprAdd(
                ctxt,
                -(1 as c_int),
                -(1 as c_int),
                XPATH_OP_ROOT,
                0 as c_int,
                0 as c_int,
                0 as c_int,
                NULL,
                NULL,
            );
        } else {
            xmlXPathCompExprAdd(
                ctxt,
                -(1 as c_int),
                -(1 as c_int),
                XPATH_OP_NODE,
                0 as c_int,
                0 as c_int,
                0 as c_int,
                NULL,
                NULL,
            );
        }
        xmlXPathCompLocationPath(ctxt);
    } else {
        xmlXPathCompFilterExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        if *(*ctxt).cur as c_int == '/' as i32
            && *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                == '/' as i32
        {
            (*ctxt).cur = (*ctxt).cur.offset(2 as c_int as isize);
            while *(*ctxt).cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *(*ctxt).cur as c_int
                    && *(*ctxt).cur as c_int <= 0xa as c_int
                || *(*ctxt).cur as c_int == 0xd as c_int
            {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
            }
            xmlXPathCompExprAdd(
                ctxt,
                (*(*ctxt).comp).last,
                -(1 as c_int),
                XPATH_OP_COLLECT,
                AXIS_DESCENDANT_OR_SELF as c_int,
                NODE_TEST_TYPE as c_int,
                NODE_TYPE_NODE as c_int,
                ::core::ptr::null_mut::<c_void>(),
                ::core::ptr::null_mut::<c_void>(),
            );
            xmlXPathCompRelativeLocationPath(ctxt);
        } else if *(*ctxt).cur as c_int == '/' as i32 {
            xmlXPathCompRelativeLocationPath(ctxt);
        }
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
} }
fn xmlXPathCompUnionExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    xmlXPathCompPathExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == '|' as i32 {
        let mut op1: c_int = (*(*ctxt).comp).last;
        xmlXPathCompExprAdd(
            ctxt,
            -(1 as c_int),
            -(1 as c_int),
            XPATH_OP_NODE,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompPathExpr(ctxt);
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_UNION,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
} }
fn xmlXPathCompUnaryExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut minus: c_int = 0 as c_int;
    let mut found: c_int = 0 as c_int;
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == '-' as i32 {
        minus = 1 as c_int - minus;
        found = 1 as c_int;
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
    xmlXPathCompUnionExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    if found != 0 {
        if minus != 0 {
            xmlXPathCompExprAdd(
                ctxt,
                (*(*ctxt).comp).last,
                -(1 as c_int),
                XPATH_OP_PLUS,
                2 as c_int,
                0 as c_int,
                0 as c_int,
                NULL,
                NULL,
            );
        } else {
            xmlXPathCompExprAdd(
                ctxt,
                (*(*ctxt).comp).last,
                -(1 as c_int),
                XPATH_OP_PLUS,
                3 as c_int,
                0 as c_int,
                0 as c_int,
                NULL,
                NULL,
            );
        }
    }
} }
fn xmlXPathCompMultiplicativeExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    xmlXPathCompUnaryExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == '*' as i32
        || *(*ctxt).cur as c_int == 'd' as i32
            && *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                == 'i' as i32
            && *(*ctxt).cur.offset(2 as c_int as isize) as c_int
                == 'v' as i32
        || *(*ctxt).cur as c_int == 'm' as i32
            && *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                == 'o' as i32
            && *(*ctxt).cur.offset(2 as c_int as isize) as c_int
                == 'd' as i32
    {
        let mut op: c_int = -(1 as c_int);
        let mut op1: c_int = (*(*ctxt).comp).last;
        if *(*ctxt).cur as c_int == '*' as i32 {
            op = 0 as c_int;
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        } else if *(*ctxt).cur as c_int == 'd' as i32 {
            op = 1 as c_int;
            (*ctxt).cur = (*ctxt).cur.offset(3 as c_int as isize);
        } else if *(*ctxt).cur as c_int == 'm' as i32 {
            op = 2 as c_int;
            (*ctxt).cur = (*ctxt).cur.offset(3 as c_int as isize);
        }
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompUnaryExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_MULT,
            op,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
} }
fn xmlXPathCompAdditiveExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    xmlXPathCompMultiplicativeExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == '+' as i32
        || *(*ctxt).cur as c_int == '-' as i32
    {
        let mut plus: c_int = 0;
        let mut op1: c_int = (*(*ctxt).comp).last;
        if *(*ctxt).cur as c_int == '+' as i32 {
            plus = 1 as c_int;
        } else {
            plus = 0 as c_int;
        }
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompMultiplicativeExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_PLUS,
            plus,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
} }
fn xmlXPathCompRelationalExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    xmlXPathCompAdditiveExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == '<' as i32
        || *(*ctxt).cur as c_int == '>' as i32
    {
        let mut inf: c_int = 0;
        let mut strict: c_int = 0;
        let mut op1: c_int = (*(*ctxt).comp).last;
        if *(*ctxt).cur as c_int == '<' as i32 {
            inf = 1 as c_int;
        } else {
            inf = 0 as c_int;
        }
        if *(*ctxt).cur.offset(1 as c_int as isize) as c_int == '=' as i32
        {
            strict = 0 as c_int;
        } else {
            strict = 1 as c_int;
        }
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        if strict == 0 {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompAdditiveExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_CMP,
            inf,
            strict,
            0 as c_int,
            NULL,
            NULL,
        );
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
} }
fn xmlXPathCompEqualityExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    xmlXPathCompRelationalExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == '=' as i32
        || *(*ctxt).cur as c_int == '!' as i32
            && *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                == '=' as i32
    {
        let mut eq: c_int = 0;
        let mut op1: c_int = (*(*ctxt).comp).last;
        if *(*ctxt).cur as c_int == '=' as i32 {
            eq = 1 as c_int;
        } else {
            eq = 0 as c_int;
        }
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        if eq == 0 {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompRelationalExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_EQUAL,
            eq,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
} }
fn xmlXPathCompAndExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    xmlXPathCompEqualityExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == 'a' as i32
        && *(*ctxt).cur.offset(1 as c_int as isize) as c_int == 'n' as i32
        && *(*ctxt).cur.offset(2 as c_int as isize) as c_int == 'd' as i32
    {
        let mut op1: c_int = (*(*ctxt).comp).last;
        (*ctxt).cur = (*ctxt).cur.offset(3 as c_int as isize);
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompEqualityExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_AND,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
} }
fn xmlXPathCompileExpr(
    mut ctxt: xmlXPathParserContextPtr,
    mut sort: c_int,
) { unsafe {
    let mut xpctxt: xmlXPathContextPtr = (*ctxt).context;
    if !xpctxt.is_null() {
        if (*xpctxt).depth >= XPATH_MAX_RECURSION_DEPTH {
            xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as c_int);
            return;
        }
        (*xpctxt).depth += 10 as c_int;
    }
    xmlXPathCompAndExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == 'o' as i32
        && *(*ctxt).cur.offset(1 as c_int as isize) as c_int == 'r' as i32
    {
        let mut op1: c_int = (*(*ctxt).comp).last;
        (*ctxt).cur = (*ctxt).cur.offset(2 as c_int as isize);
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompAndExpr(ctxt);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_OR,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
    if sort != 0
        && (*(*(*ctxt).comp).steps.offset((*(*ctxt).comp).last as isize)).op as c_uint
            != XPATH_OP_VALUE as c_int as c_uint
    {
        xmlXPathCompExprAdd(
            ctxt,
            (*(*ctxt).comp).last,
            -(1 as c_int),
            XPATH_OP_SORT,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
    }
    if !xpctxt.is_null() {
        (*xpctxt).depth -= 10 as c_int;
    }
} }
fn xmlXPathCompPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut filter: c_int,
) { unsafe {
    let mut op1: c_int = (*(*ctxt).comp).last;
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int != '[' as i32 {
        xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as c_int);
        return;
    }
    if *(*ctxt).cur as c_int != 0 {
        (*ctxt).cur = (*ctxt).cur.offset(1);
    } else {
    };
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    (*(*ctxt).comp).last = -(1 as c_int);
    if filter == 0 {
        xmlXPathCompileExpr(ctxt, 0 as c_int);
    } else {
        xmlXPathCompileExpr(ctxt, 1 as c_int);
    }
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    if *(*ctxt).cur as c_int != ']' as i32 {
        xmlXPathErr(ctxt, XPATH_INVALID_PREDICATE_ERROR as c_int);
        return;
    }
    if filter != 0 {
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_FILTER,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
    } else {
        xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_PREDICATE,
            0 as c_int,
            0 as c_int,
            0 as c_int,
            NULL,
            NULL,
        );
    }
    if *(*ctxt).cur as c_int != 0 {
        (*ctxt).cur = (*ctxt).cur.offset(1);
    } else {
    };
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
} }
unsafe fn xmlXPathCompNodeTest(
    mut ctxt: xmlXPathParserContextPtr,
    mut test: *mut xmlXPathTestVal,
    mut type_0: *mut xmlXPathTypeVal,
    mut prefix: *mut *mut xmlChar,
    mut name: *mut xmlChar,
) -> *mut xmlChar {
    let mut blanks: c_int = 0;
    if test.is_null() || type_0.is_null() || prefix.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Internal error at %s:%d\n\0" as *const u8 as *const c_char,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            10303 as c_int,
        );
        return ::core::ptr::null_mut::<xmlChar>();
    }
    *type_0 = NODE_TYPE_NODE;
    *test = NODE_TEST_NONE;
    *prefix = ::core::ptr::null_mut::<xmlChar>();
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if name.is_null() && *(*ctxt).cur as c_int == '*' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        *test = NODE_TEST_ALL;
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if name.is_null() {
        name = xmlXPathParseNCName(ctxt);
    }
    if name.is_null() {
        xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    blanks = (*(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int)
        as c_int;
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int == '(' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        if xmlStrEqual(
            name,
            b"comment\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        {
            *type_0 = NODE_TYPE_COMMENT;
        } else if xmlStrEqual(
            name,
            b"node\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        {
            *type_0 = NODE_TYPE_NODE;
        } else if xmlStrEqual(
            name,
            b"processing-instruction\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        {
            *type_0 = NODE_TYPE_PI;
        } else if xmlStrEqual(
            name,
            b"text\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        {
            *type_0 = NODE_TYPE_TEXT;
        } else {
            if !name.is_null() {
                xmlFree.expect("non-null function pointer")(name as *mut c_void);
            }
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
            return ::core::ptr::null_mut::<xmlChar>();
        }
        *test = NODE_TEST_TYPE;
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        if *type_0 as c_uint
            == NODE_TYPE_PI as c_int as c_uint
        {
            if !name.is_null() {
                xmlFree.expect("non-null function pointer")(name as *mut c_void);
            }
            name = ::core::ptr::null_mut::<xmlChar>();
            if *(*ctxt).cur as c_int != ')' as i32 {
                name = xmlXPathParseLiteral(ctxt);
                if name.is_null() {
                    xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
                    return ::core::ptr::null_mut::<xmlChar>();
                }
                *test = NODE_TEST_PI;
                while *(*ctxt).cur as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *(*ctxt).cur as c_int
                        && *(*ctxt).cur as c_int <= 0xa as c_int
                    || *(*ctxt).cur as c_int == 0xd as c_int
                {
                    if *(*ctxt).cur as c_int != 0 {
                        (*ctxt).cur = (*ctxt).cur.offset(1);
                    } else {
                    };
                }
            }
        }
        if *(*ctxt).cur as c_int != ')' as i32 {
            if !name.is_null() {
                xmlFree.expect("non-null function pointer")(name as *mut c_void);
            }
            xmlXPathErr(ctxt, XPATH_UNCLOSED_ERROR as c_int);
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        return name;
    }
    *test = NODE_TEST_NAME;
    if blanks == 0 && *(*ctxt).cur as c_int == ':' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        *prefix = name;
        if *(*ctxt).cur as c_int == '*' as i32 {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
            *test = NODE_TEST_ALL;
            return ::core::ptr::null_mut::<xmlChar>();
        }
        name = xmlXPathParseNCName(ctxt);
        if name.is_null() {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
            return ::core::ptr::null_mut::<xmlChar>();
        }
    }
    return name;
}
unsafe fn xmlXPathIsAxisName(mut name: *const xmlChar) -> xmlXPathAxisVal {
    let mut ret: xmlXPathAxisVal = 0 as xmlXPathAxisVal;
    match *name.offset(0 as c_int as isize) as c_int {
        97 => {
            if xmlStrEqual(
                name,
                b"ancestor\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_ANCESTOR;
            }
            if xmlStrEqual(
                name,
                b"ancestor-or-self\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_ANCESTOR_OR_SELF;
            }
            if xmlStrEqual(
                name,
                b"attribute\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_ATTRIBUTE;
            }
        }
        99 => {
            if xmlStrEqual(
                name,
                b"child\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_CHILD;
            }
        }
        100 => {
            if xmlStrEqual(
                name,
                b"descendant\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_DESCENDANT;
            }
            if xmlStrEqual(
                name,
                b"descendant-or-self\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_DESCENDANT_OR_SELF;
            }
        }
        102 => {
            if xmlStrEqual(
                name,
                b"following\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_FOLLOWING;
            }
            if xmlStrEqual(
                name,
                b"following-sibling\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_FOLLOWING_SIBLING;
            }
        }
        110 => {
            if xmlStrEqual(
                name,
                b"namespace\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_NAMESPACE;
            }
        }
        112 => {
            if xmlStrEqual(
                name,
                b"parent\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_PARENT;
            }
            if xmlStrEqual(
                name,
                b"preceding\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_PRECEDING;
            }
            if xmlStrEqual(
                name,
                b"preceding-sibling\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_PRECEDING_SIBLING;
            }
        }
        115 => {
            if xmlStrEqual(
                name,
                b"self\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                ret = AXIS_SELF;
            }
        }
        _ => {}
    }
    return ret;
}
fn xmlXPathCompStep(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int == '.' as i32
        && *(*ctxt).cur.offset(1 as c_int as isize) as c_int == '.' as i32
    {
        (*ctxt).cur = (*ctxt).cur.offset(2 as c_int as isize);
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompExprAdd(
            ctxt,
            (*(*ctxt).comp).last,
            -(1 as c_int),
            XPATH_OP_COLLECT,
            AXIS_PARENT as c_int,
            NODE_TEST_TYPE as c_int,
            NODE_TYPE_NODE as c_int,
            ::core::ptr::null_mut::<c_void>(),
            ::core::ptr::null_mut::<c_void>(),
        );
    } else if *(*ctxt).cur as c_int == '.' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    } else {
        let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut test: xmlXPathTestVal = NODE_TEST_NONE;
        let mut axis: xmlXPathAxisVal = 0 as xmlXPathAxisVal;
        let mut type_0: xmlXPathTypeVal = NODE_TYPE_NODE;
        let mut op1: c_int = 0;
        if *(*ctxt).cur as c_int == '*' as i32 {
            axis = AXIS_CHILD;
        } else {
            if name.is_null() {
                name = xmlXPathParseNCName(ctxt);
            }
            if !name.is_null() {
                axis = xmlXPathIsAxisName(name);
                if axis as c_uint != 0 as c_uint {
                    while *(*ctxt).cur as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *(*ctxt).cur as c_int
                            && *(*ctxt).cur as c_int <= 0xa as c_int
                        || *(*ctxt).cur as c_int == 0xd as c_int
                    {
                        if *(*ctxt).cur as c_int != 0 {
                            (*ctxt).cur = (*ctxt).cur.offset(1);
                        } else {
                        };
                    }
                    if *(*ctxt).cur as c_int == ':' as i32
                        && *(*ctxt).cur.offset(1 as c_int as isize)
                            as c_int
                            == ':' as i32
                    {
                        (*ctxt).cur = (*ctxt).cur.offset(2 as c_int as isize);
                        xmlFree.expect("non-null function pointer")(
                            name as *mut c_void,
                        );
                        name = ::core::ptr::null_mut::<xmlChar>();
                    } else {
                        axis = AXIS_CHILD;
                    }
                } else {
                    axis = AXIS_CHILD;
                }
            } else if *(*ctxt).cur as c_int == '@' as i32 {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
                axis = AXIS_ATTRIBUTE;
            } else {
                axis = AXIS_CHILD;
            }
        }
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
            return;
        }
        name = xmlXPathCompNodeTest(ctxt, &raw mut test, &raw mut type_0, &raw mut prefix, name);
        if test as c_uint == 0 as c_uint {
            return;
        }
        if !prefix.is_null()
            && !(*ctxt).context.is_null()
            && (*(*ctxt).context).flags & XML_XPATH_CHECKNS != 0
        {
            if xmlXPathNsLookup((*ctxt).context, prefix).is_null() {
                xmlXPathErr(ctxt, XPATH_UNDEF_PREFIX_ERROR as c_int);
            }
        }
        op1 = (*(*ctxt).comp).last;
        (*(*ctxt).comp).last = -(1 as c_int);
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        while *(*ctxt).cur as c_int == '[' as i32 {
            xmlXPathCompPredicate(ctxt, 0 as c_int);
        }
        if xmlXPathCompExprAdd(
            ctxt,
            op1,
            (*(*ctxt).comp).last,
            XPATH_OP_COLLECT,
            axis as c_int,
            test as c_int,
            type_0 as c_int,
            prefix as *mut c_void,
            name as *mut c_void,
        ) == -(1 as c_int)
        {
            xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
        }
    };
} }
fn xmlXPathCompRelativeLocationPath(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int == '/' as i32
        && *(*ctxt).cur.offset(1 as c_int as isize) as c_int == '/' as i32
    {
        (*ctxt).cur = (*ctxt).cur.offset(2 as c_int as isize);
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
        xmlXPathCompExprAdd(
            ctxt,
            (*(*ctxt).comp).last,
            -(1 as c_int),
            XPATH_OP_COLLECT,
            AXIS_DESCENDANT_OR_SELF as c_int,
            NODE_TEST_TYPE as c_int,
            NODE_TYPE_NODE as c_int,
            ::core::ptr::null_mut::<c_void>(),
            ::core::ptr::null_mut::<c_void>(),
        );
    } else if *(*ctxt).cur as c_int == '/' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
    xmlXPathCompStep(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return;
    }
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    while *(*ctxt).cur as c_int == '/' as i32 {
        if *(*ctxt).cur as c_int == '/' as i32
            && *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                == '/' as i32
        {
            (*ctxt).cur = (*ctxt).cur.offset(2 as c_int as isize);
            while *(*ctxt).cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *(*ctxt).cur as c_int
                    && *(*ctxt).cur as c_int <= 0xa as c_int
                || *(*ctxt).cur as c_int == 0xd as c_int
            {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
            }
            xmlXPathCompExprAdd(
                ctxt,
                (*(*ctxt).comp).last,
                -(1 as c_int),
                XPATH_OP_COLLECT,
                AXIS_DESCENDANT_OR_SELF as c_int,
                NODE_TEST_TYPE as c_int,
                NODE_TYPE_NODE as c_int,
                ::core::ptr::null_mut::<c_void>(),
                ::core::ptr::null_mut::<c_void>(),
            );
            xmlXPathCompStep(ctxt);
        } else if *(*ctxt).cur as c_int == '/' as i32 {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
            while *(*ctxt).cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *(*ctxt).cur as c_int
                    && *(*ctxt).cur as c_int <= 0xa as c_int
                || *(*ctxt).cur as c_int == 0xd as c_int
            {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
            }
            xmlXPathCompStep(ctxt);
        }
        while *(*ctxt).cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *(*ctxt).cur as c_int
                && *(*ctxt).cur as c_int <= 0xa as c_int
            || *(*ctxt).cur as c_int == 0xd as c_int
        {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
        }
    }
} }
fn xmlXPathCompLocationPath(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    while *(*ctxt).cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *(*ctxt).cur as c_int
            && *(*ctxt).cur as c_int <= 0xa as c_int
        || *(*ctxt).cur as c_int == 0xd as c_int
    {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
    }
    if *(*ctxt).cur as c_int != '/' as i32 {
        xmlXPathCompRelativeLocationPath(ctxt);
    } else {
        while *(*ctxt).cur as c_int == '/' as i32 {
            if *(*ctxt).cur as c_int == '/' as i32
                && *(*ctxt).cur.offset(1 as c_int as isize) as c_int
                    == '/' as i32
            {
                (*ctxt).cur = (*ctxt).cur.offset(2 as c_int as isize);
                while *(*ctxt).cur as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *(*ctxt).cur as c_int
                        && *(*ctxt).cur as c_int <= 0xa as c_int
                    || *(*ctxt).cur as c_int == 0xd as c_int
                {
                    if *(*ctxt).cur as c_int != 0 {
                        (*ctxt).cur = (*ctxt).cur.offset(1);
                    } else {
                    };
                }
                xmlXPathCompExprAdd(
                    ctxt,
                    (*(*ctxt).comp).last,
                    -(1 as c_int),
                    XPATH_OP_COLLECT,
                    AXIS_DESCENDANT_OR_SELF as c_int,
                    NODE_TEST_TYPE as c_int,
                    NODE_TYPE_NODE as c_int,
                    ::core::ptr::null_mut::<c_void>(),
                    ::core::ptr::null_mut::<c_void>(),
                );
                xmlXPathCompRelativeLocationPath(ctxt);
            } else if *(*ctxt).cur as c_int == '/' as i32 {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
                while *(*ctxt).cur as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *(*ctxt).cur as c_int
                        && *(*ctxt).cur as c_int <= 0xa as c_int
                    || *(*ctxt).cur as c_int == 0xd as c_int
                {
                    if *(*ctxt).cur as c_int != 0 {
                        (*ctxt).cur = (*ctxt).cur.offset(1);
                    } else {
                    };
                }
                if *(*ctxt).cur as c_int != 0 as c_int
                    && (0x41 as c_int <= *(*ctxt).cur as c_int
                        && *(*ctxt).cur as c_int <= 0x5a as c_int
                        || 0x61 as c_int <= *(*ctxt).cur as c_int
                            && *(*ctxt).cur as c_int <= 0x7a as c_int
                        || *(*ctxt).cur as c_int == '_' as i32
                        || *(*ctxt).cur as c_int == '.' as i32
                        || *(*ctxt).cur as c_int == '@' as i32
                        || *(*ctxt).cur as c_int == '*' as i32)
                {
                    xmlXPathCompRelativeLocationPath(ctxt);
                }
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return;
            }
        }
    };
} }
fn xmlXPathNodeSetFilter(
    mut ctxt: xmlXPathParserContextPtr,
    mut set: xmlNodeSetPtr,
    mut filterOpIndex: c_int,
    mut minPos: c_int,
    mut maxPos: c_int,
    mut hasNsNodes: c_int,
) { unsafe {
    let mut xpctxt: xmlXPathContextPtr = ::core::ptr::null_mut::<xmlXPathContext>();
    let mut oldnode: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut olddoc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut filterOp: xmlXPathStepOpPtr = ::core::ptr::null_mut::<xmlXPathStepOp>();
    let mut oldcs: c_int = 0;
    let mut oldpp: c_int = 0;
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut pos: c_int = 0;
    if set.is_null() || (*set).nodeNr == 0 as c_int {
        return;
    }
    if (*set).nodeNr < minPos {
        xmlXPathNodeSetClear(set, hasNsNodes);
        return;
    }
    xpctxt = (*ctxt).context;
    oldnode = (*xpctxt).node;
    olddoc = (*xpctxt).doc;
    oldcs = (*xpctxt).contextSize;
    oldpp = (*xpctxt).proximityPosition;
    filterOp = (*(*ctxt).comp).steps.offset(filterOpIndex as isize) as *mut xmlXPathStepOp
        as xmlXPathStepOpPtr;
    (*xpctxt).contextSize = (*set).nodeNr;
    i = 0 as c_int;
    j = 0 as c_int;
    pos = 1 as c_int;
    while i < (*set).nodeNr {
        let mut node: xmlNodePtr = *(*set).nodeTab.offset(i as isize);
        let mut res: c_int = 0;
        (*xpctxt).node = node;
        (*xpctxt).proximityPosition = i + 1 as c_int;
        if (*node).type_0 as c_uint
            != XML_NAMESPACE_DECL as c_int as c_uint
            && !(*node).doc.is_null()
        {
            (*xpctxt).doc = (*node).doc as xmlDocPtr;
        }
        res = xmlXPathCompOpEvalToBoolean(ctxt, filterOp, 1 as c_int);
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            break;
        }
        if res < 0 as c_int {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
            break;
        } else {
            if res != 0 as c_int && (pos >= minPos && pos <= maxPos) {
                if i != j {
                    let ref mut fresh51 = *(*set).nodeTab.offset(j as isize);
                    *fresh51 = node;
                    let ref mut fresh52 = *(*set).nodeTab.offset(i as isize);
                    *fresh52 = ::core::ptr::null_mut::<xmlNode>();
                }
                j += 1 as c_int;
            } else {
                let ref mut fresh53 = *(*set).nodeTab.offset(i as isize);
                *fresh53 = ::core::ptr::null_mut::<xmlNode>();
                if (*node).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
                {
                    xmlXPathNodeSetFreeNs(node as xmlNsPtr);
                }
            }
            if res != 0 as c_int {
                if pos == maxPos {
                    i += 1 as c_int;
                    break;
                } else {
                    pos += 1 as c_int;
                }
            }
            i += 1;
        }
    }
    if hasNsNodes != 0 {
        while i < (*set).nodeNr {
            let mut node_0: xmlNodePtr = *(*set).nodeTab.offset(i as isize);
            if !node_0.is_null()
                && (*node_0).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
            {
                xmlXPathNodeSetFreeNs(node_0 as xmlNsPtr);
            }
            i += 1;
        }
    }
    (*set).nodeNr = j;
    if (*set).nodeMax > XML_NODESET_DEFAULT
        && (*set).nodeNr < (*set).nodeMax / 2 as c_int
    {
        let mut tmp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
        let mut nodeMax: c_int = (*set).nodeNr;
        if nodeMax < XML_NODESET_DEFAULT {
            nodeMax = XML_NODESET_DEFAULT;
        }
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*set).nodeTab as *mut c_void,
            (nodeMax as size_t).wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if tmp.is_null() {
            xmlXPathPErrMemory(
                ctxt,
                b"shrinking nodeset\n\0" as *const u8 as *const c_char,
            );
        } else {
            (*set).nodeTab = tmp;
            (*set).nodeMax = nodeMax;
        }
    }
    (*xpctxt).node = oldnode;
    (*xpctxt).doc = olddoc;
    (*xpctxt).contextSize = oldcs;
    (*xpctxt).proximityPosition = oldpp;
} }
fn xmlXPathCompOpEvalPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut set: xmlNodeSetPtr,
    mut minPos: c_int,
    mut maxPos: c_int,
    mut hasNsNodes: c_int,
) { unsafe {
    if (*op).ch1 != -(1 as c_int) {
        let mut comp: xmlXPathCompExprPtr = (*ctxt).comp;
        if (*(*comp).steps.offset((*op).ch1 as isize)).op as c_uint
            != XPATH_OP_PREDICATE as c_int as c_uint
        {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompOpEvalPredicate: Expected a predicate\n\0" as *const u8
                    as *const c_char,
            );
            xmlXPathErr(ctxt, XPATH_INVALID_OPERAND as c_int);
            return;
        }
        if (*(*ctxt).context).depth >= XPATH_MAX_RECURSION_DEPTH {
            xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as c_int);
            return;
        }
        (*(*ctxt).context).depth += 1 as c_int;
        xmlXPathCompOpEvalPredicate(
            ctxt,
            (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            set,
            1 as c_int,
            (*set).nodeNr,
            hasNsNodes,
        );
        (*(*ctxt).context).depth -= 1 as c_int;
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
    }
    if (*op).ch2 != -(1 as c_int) {
        xmlXPathNodeSetFilter(ctxt, set, (*op).ch2, minPos, maxPos, hasNsNodes);
    }
} }
unsafe fn xmlXPathIsPositionalPredicate(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut maxPos: *mut c_int,
) -> c_int {
    let mut exprOp: xmlXPathStepOpPtr = ::core::ptr::null_mut::<xmlXPathStepOp>();
    if (*op).op as c_uint
        != XPATH_OP_PREDICATE as c_int as c_uint
        && (*op).op as c_uint
            != XPATH_OP_FILTER as c_int as c_uint
    {
        return 0 as c_int;
    }
    if (*op).ch2 != -(1 as c_int) {
        exprOp = (*(*ctxt).comp).steps.offset((*op).ch2 as isize) as *mut xmlXPathStepOp
            as xmlXPathStepOpPtr;
    } else {
        return 0 as c_int;
    }
    if !exprOp.is_null()
        && (*exprOp).op as c_uint
            == XPATH_OP_VALUE as c_int as c_uint
        && !(*exprOp).value4.is_null()
        && (*((*exprOp).value4 as xmlXPathObjectPtr)).type_0 as c_uint
            == XPATH_NUMBER as c_int as c_uint
    {
        let mut floatval: c_double =
            (*((*exprOp).value4 as xmlXPathObjectPtr)).floatval;
        if floatval > INT_MIN as c_double
            && floatval < INT_MAX as c_double
        {
            *maxPos = floatval as c_int;
            if floatval == *maxPos as c_double {
                return 1 as c_int;
            }
        }
    }
    return 0 as c_int;
}
unsafe fn xmlXPathNodeCollectAndTest(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
    mut last: *mut xmlNodePtr,
    mut toBool: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut axis: xmlXPathAxisVal = (*op).value as xmlXPathAxisVal;
    let mut test: xmlXPathTestVal = (*op).value2 as xmlXPathTestVal;
    let mut type_0: xmlXPathTypeVal = (*op).value3 as xmlXPathTypeVal;
    let mut prefix: *const xmlChar = (*op).value4 as *const xmlChar;
    let mut name: *const xmlChar = (*op).value5 as *const xmlChar;
    let mut URI: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut total: c_int = 0 as c_int;
    let mut hasNsNodes: c_int = 0 as c_int;
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut contextSeq: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut contextIdx: c_int = 0;
    let mut contextNode: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut outSeq: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut seq: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut predOp: xmlXPathStepOpPtr = ::core::ptr::null_mut::<xmlXPathStepOp>();
    let mut maxPos: c_int = 0;
    let mut hasPredicateRange: c_int = 0;
    let mut hasAxisRange: c_int = 0;
    let mut pos: c_int = 0;
    let mut breakOnFirstHit: c_int = 0;
    let mut next: xmlXPathTraversalFunction = None;
    let mut addNode: Option<unsafe extern "C" fn(xmlNodeSetPtr, xmlNodePtr) -> c_int> =
        None;
    let mut mergeAndClear: xmlXPathNodeSetMergeFunction = None;
    let mut oldContextNode: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut xpctxt: xmlXPathContextPtr = (*ctxt).context;
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return 0 as c_int;
    }
    obj = valuePop(ctxt);
    if !prefix.is_null() {
        URI = xmlXPathNsLookup(xpctxt, prefix);
        if URI.is_null() {
            xmlXPathReleaseObject(xpctxt, obj);
            xmlXPathErr(ctxt, XPATH_UNDEF_PREFIX_ERROR as c_int);
            return 0 as c_int;
        }
    }
    mergeAndClear = Some(
        xmlXPathNodeSetMergeAndClear
            as unsafe extern "C" fn(xmlNodeSetPtr, xmlNodeSetPtr) -> xmlNodeSetPtr,
    ) as xmlXPathNodeSetMergeFunction;
    match axis as c_uint {
        1 => {
            first = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextAncestor
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        2 => {
            first = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextAncestorOrSelf
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        3 => {
            first = ::core::ptr::null_mut::<xmlNodePtr>();
            last = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextAttribute
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
            mergeAndClear = Some(
                xmlXPathNodeSetMergeAndClearNoDupls
                    as unsafe extern "C" fn(xmlNodeSetPtr, xmlNodeSetPtr) -> xmlNodeSetPtr,
            ) as xmlXPathNodeSetMergeFunction;
        }
        4 => {
            last = ::core::ptr::null_mut::<xmlNodePtr>();
            if (test as c_uint
                == NODE_TEST_NAME as c_int as c_uint
                || test as c_uint
                    == NODE_TEST_ALL as c_int as c_uint)
                && type_0 as c_uint
                    == NODE_TYPE_NODE as c_int as c_uint
            {
                next = Some(
                    xmlXPathNextChildElement
                        as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
                ) as xmlXPathTraversalFunction;
            } else {
                next = Some(
                    xmlXPathNextChild
                        as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
                ) as xmlXPathTraversalFunction;
            }
            mergeAndClear = Some(
                xmlXPathNodeSetMergeAndClearNoDupls
                    as unsafe extern "C" fn(xmlNodeSetPtr, xmlNodeSetPtr) -> xmlNodeSetPtr,
            ) as xmlXPathNodeSetMergeFunction;
        }
        5 => {
            last = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextDescendant
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        6 => {
            last = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextDescendantOrSelf
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        7 => {
            last = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextFollowing
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        8 => {
            last = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextFollowingSibling
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        9 => {
            first = ::core::ptr::null_mut::<xmlNodePtr>();
            last = ::core::ptr::null_mut::<xmlNodePtr>();
            next = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr>,
                xmlXPathTraversalFunction,
            >(Some(
                xmlXPathNextNamespace
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ));
            mergeAndClear = Some(
                xmlXPathNodeSetMergeAndClearNoDupls
                    as unsafe extern "C" fn(xmlNodeSetPtr, xmlNodeSetPtr) -> xmlNodeSetPtr,
            ) as xmlXPathNodeSetMergeFunction;
        }
        10 => {
            first = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextParent
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        11 => {
            first = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextPrecedingInternal
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        12 => {
            first = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextPrecedingSibling
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
        }
        13 => {
            first = ::core::ptr::null_mut::<xmlNodePtr>();
            last = ::core::ptr::null_mut::<xmlNodePtr>();
            next = Some(
                xmlXPathNextSelf
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, xmlNodePtr) -> xmlNodePtr,
            ) as xmlXPathTraversalFunction;
            mergeAndClear = Some(
                xmlXPathNodeSetMergeAndClearNoDupls
                    as unsafe extern "C" fn(xmlNodeSetPtr, xmlNodeSetPtr) -> xmlNodeSetPtr,
            ) as xmlXPathNodeSetMergeFunction;
        }
        _ => {}
    }
    if next.is_none() {
        xmlXPathReleaseObject(xpctxt, obj);
        return 0 as c_int;
    }
    contextSeq = (*obj).nodesetval;
    if contextSeq.is_null() || (*contextSeq).nodeNr <= 0 as c_int {
        xmlXPathReleaseObject(xpctxt, obj);
        valuePush(
            ctxt,
            xmlXPathCacheWrapNodeSet(xpctxt, ::core::ptr::null_mut::<xmlNodeSet>()),
        );
        return 0 as c_int;
    }
    maxPos = 0 as c_int;
    predOp = ::core::ptr::null_mut::<xmlXPathStepOp>();
    hasPredicateRange = 0 as c_int;
    hasAxisRange = 0 as c_int;
    if (*op).ch2 != -(1 as c_int) {
        predOp = (*(*ctxt).comp).steps.offset((*op).ch2 as isize) as *mut xmlXPathStepOp
            as xmlXPathStepOpPtr;
        if xmlXPathIsPositionalPredicate(ctxt, predOp, &raw mut maxPos) != 0 {
            if (*predOp).ch1 != -(1 as c_int) {
                predOp = (*(*ctxt).comp).steps.offset((*predOp).ch1 as isize) as *mut xmlXPathStepOp
                    as xmlXPathStepOpPtr;
                hasPredicateRange = 1 as c_int;
            } else {
                predOp = ::core::ptr::null_mut::<xmlXPathStepOp>();
                hasAxisRange = 1 as c_int;
            }
        }
    }
    breakOnFirstHit = if toBool != 0 && predOp.is_null() {
        1 as c_int
    } else {
        0 as c_int
    };
    oldContextNode = (*xpctxt).node;
    addNode = Some(
        xmlXPathNodeSetAddUnique
            as unsafe extern "C" fn(xmlNodeSetPtr, xmlNodePtr) -> c_int,
    )
        as Option<unsafe extern "C" fn(xmlNodeSetPtr, xmlNodePtr) -> c_int>;
    outSeq = ::core::ptr::null_mut::<xmlNodeSet>();
    seq = ::core::ptr::null_mut::<xmlNodeSet>();
    contextNode = ::core::ptr::null_mut::<xmlNode>();
    contextIdx = 0 as c_int;
    's_386: while (contextIdx < (*contextSeq).nodeNr || !contextNode.is_null())
        && (*ctxt).error == XPATH_EXPRESSION_OK as c_int
    {
        let fresh54 = contextIdx;
        contextIdx = contextIdx + 1;
        (*xpctxt).node = *(*contextSeq).nodeTab.offset(fresh54 as isize);
        if seq.is_null() {
            seq = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
            if seq.is_null() {
                total = 0 as c_int;
                break;
            }
        }
        pos = 0 as c_int;
        cur = ::core::ptr::null_mut::<xmlNode>();
        hasNsNodes = 0 as c_int;
        loop {
            if (*(*ctxt).context).opLimit != 0 as c_ulong
                && xmlXPathCheckOpLimit(ctxt, 1 as c_ulong) < 0 as c_int
            {
                break 's_386;
            }
            cur = next.expect("non-null function pointer")(ctxt, cur);
            if cur.is_null() {
                current_block = 3087853033418004326;
                break;
            }
            if !first.is_null() && !(*first).is_null() {
                if *first == cur {
                    current_block = 3087853033418004326;
                    break;
                }
                if total % 256 as c_int == 0 as c_int
                    && xmlXPathCmpNodesExt(*first, cur) >= 0 as c_int
                {
                    current_block = 3087853033418004326;
                    break;
                }
            }
            if !last.is_null() && !(*last).is_null() {
                if *last == cur {
                    current_block = 3087853033418004326;
                    break;
                }
                if total % 256 as c_int == 0 as c_int
                    && xmlXPathCmpNodesExt(cur, *last) >= 0 as c_int
                {
                    current_block = 3087853033418004326;
                    break;
                }
            }
            total += 1;
            match test as c_uint {
                0 => {
                    total = 0 as c_int;
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Internal error at %s:%d\n\0" as *const u8 as *const c_char,
                        b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0"
                            as *const u8 as *const c_char,
                        11384 as c_int,
                    );
                    break 's_386;
                }
                1 => {
                    if type_0 as c_uint
                        == NODE_TYPE_NODE as c_int as c_uint
                    {
                        match (*cur).type_0 as c_uint {
                            9 | 13 | 1 | 2 | 7 | 8 | 4 | 3 => {
                                current_block = 7614493777402006307;
                                match current_block {
                                    7614493777402006307 => {
                                        if hasAxisRange != 0 as c_int {
                                            pos += 1;
                                            if pos == maxPos {
                                                if addNode.expect("non-null function pointer")(
                                                    seq, cur,
                                                ) < 0 as c_int
                                                {
                                                    (*ctxt).error =
                                                        XPATH_MEMORY_ERROR as c_int;
                                                }
                                                current_block = 15908361485829668090;
                                                break;
                                            }
                                        } else {
                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                < 0 as c_int
                                            {
                                                (*ctxt).error =
                                                    XPATH_MEMORY_ERROR as c_int;
                                            }
                                            if breakOnFirstHit != 0 {
                                                current_block = 13599015263181939531;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        if axis as c_uint
                                            == AXIS_NAMESPACE as c_int
                                                as c_uint
                                        {
                                            if hasAxisRange != 0 as c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    hasNsNodes = 1 as c_int;
                                                    if xmlXPathNodeSetAddNs(
                                                        seq,
                                                        (*xpctxt).node,
                                                        cur as xmlNsPtr,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    current_block = 15908361485829668090;
                                                    break;
                                                }
                                            } else {
                                                hasNsNodes = 1 as c_int;
                                                if xmlXPathNodeSetAddNs(
                                                    seq,
                                                    (*xpctxt).node,
                                                    cur as xmlNsPtr,
                                                ) < 0 as c_int
                                                {
                                                    (*ctxt).error =
                                                        XPATH_MEMORY_ERROR as c_int;
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 13599015263181939531;
                                                    break;
                                                }
                                            }
                                        } else {
                                            hasNsNodes = 1 as c_int;
                                            if hasAxisRange != 0 as c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    if addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    current_block = 15908361485829668090;
                                                    break;
                                                }
                                            } else {
                                                if addNode.expect("non-null function pointer")(
                                                    seq, cur,
                                                ) < 0 as c_int
                                                {
                                                    (*ctxt).error =
                                                        XPATH_MEMORY_ERROR as c_int;
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 13599015263181939531;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            18 => {
                                current_block = 11674240781755647963;
                                match current_block {
                                    7614493777402006307 => {
                                        if hasAxisRange != 0 as c_int {
                                            pos += 1;
                                            if pos == maxPos {
                                                if addNode.expect("non-null function pointer")(
                                                    seq, cur,
                                                ) < 0 as c_int
                                                {
                                                    (*ctxt).error =
                                                        XPATH_MEMORY_ERROR as c_int;
                                                }
                                                current_block = 15908361485829668090;
                                                break;
                                            }
                                        } else {
                                            if addNode.expect("non-null function pointer")(seq, cur)
                                                < 0 as c_int
                                            {
                                                (*ctxt).error =
                                                    XPATH_MEMORY_ERROR as c_int;
                                            }
                                            if breakOnFirstHit != 0 {
                                                current_block = 13599015263181939531;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        if axis as c_uint
                                            == AXIS_NAMESPACE as c_int
                                                as c_uint
                                        {
                                            if hasAxisRange != 0 as c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    hasNsNodes = 1 as c_int;
                                                    if xmlXPathNodeSetAddNs(
                                                        seq,
                                                        (*xpctxt).node,
                                                        cur as xmlNsPtr,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    current_block = 15908361485829668090;
                                                    break;
                                                }
                                            } else {
                                                hasNsNodes = 1 as c_int;
                                                if xmlXPathNodeSetAddNs(
                                                    seq,
                                                    (*xpctxt).node,
                                                    cur as xmlNsPtr,
                                                ) < 0 as c_int
                                                {
                                                    (*ctxt).error =
                                                        XPATH_MEMORY_ERROR as c_int;
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 13599015263181939531;
                                                    break;
                                                }
                                            }
                                        } else {
                                            hasNsNodes = 1 as c_int;
                                            if hasAxisRange != 0 as c_int {
                                                pos += 1;
                                                if pos == maxPos {
                                                    if addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    current_block = 15908361485829668090;
                                                    break;
                                                }
                                            } else {
                                                if addNode.expect("non-null function pointer")(
                                                    seq, cur,
                                                ) < 0 as c_int
                                                {
                                                    (*ctxt).error =
                                                        XPATH_MEMORY_ERROR as c_int;
                                                }
                                                if breakOnFirstHit != 0 {
                                                    current_block = 13599015263181939531;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else if (*cur).type_0 as c_uint
                        == type_0 as xmlElementType as c_uint
                    {
                        if (*cur).type_0 as c_uint
                            == XML_NAMESPACE_DECL as c_int as c_uint
                        {
                            if hasAxisRange != 0 as c_int {
                                pos += 1;
                                if pos == maxPos {
                                    hasNsNodes = 1 as c_int;
                                    if xmlXPathNodeSetAddNs(seq, (*xpctxt).node, cur as xmlNsPtr)
                                        < 0 as c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                    }
                                    current_block = 15908361485829668090;
                                    break;
                                }
                            } else {
                                hasNsNodes = 1 as c_int;
                                if xmlXPathNodeSetAddNs(seq, (*xpctxt).node, cur as xmlNsPtr)
                                    < 0 as c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 13599015263181939531;
                                    break;
                                }
                            }
                        } else if hasAxisRange != 0 as c_int {
                            pos += 1;
                            if pos == maxPos {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                }
                                current_block = 15908361485829668090;
                                break;
                            }
                        } else {
                            if addNode.expect("non-null function pointer")(seq, cur)
                                < 0 as c_int
                            {
                                (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 13599015263181939531;
                                break;
                            }
                        }
                    } else if type_0 as c_uint
                        == NODE_TYPE_TEXT as c_int as c_uint
                        && (*cur).type_0 as c_uint
                            == XML_CDATA_SECTION_NODE as c_int as c_uint
                    {
                        if hasAxisRange != 0 as c_int {
                            pos += 1;
                            if pos == maxPos {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                }
                                current_block = 15908361485829668090;
                                break;
                            }
                        } else {
                            if addNode.expect("non-null function pointer")(seq, cur)
                                < 0 as c_int
                            {
                                (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 13599015263181939531;
                                break;
                            }
                        }
                    }
                }
                2 => {
                    if (*cur).type_0 as c_uint
                        == XML_PI_NODE as c_int as c_uint
                        && (name.is_null() || xmlStrEqual(name, (*cur).name) != 0)
                    {
                        if hasAxisRange != 0 as c_int {
                            pos += 1;
                            if pos == maxPos {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                }
                                current_block = 15908361485829668090;
                                break;
                            }
                        } else {
                            if addNode.expect("non-null function pointer")(seq, cur)
                                < 0 as c_int
                            {
                                (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                            }
                            if breakOnFirstHit != 0 {
                                current_block = 13599015263181939531;
                                break;
                            }
                        }
                    }
                }
                3 => {
                    if axis as c_uint
                        == AXIS_ATTRIBUTE as c_int as c_uint
                    {
                        if (*cur).type_0 as c_uint
                            == XML_ATTRIBUTE_NODE as c_int as c_uint
                        {
                            if prefix.is_null() {
                                if hasAxisRange != 0 as c_int {
                                    pos += 1;
                                    if pos == maxPos {
                                        if addNode.expect("non-null function pointer")(seq, cur)
                                            < 0 as c_int
                                        {
                                            (*ctxt).error =
                                                XPATH_MEMORY_ERROR as c_int;
                                        }
                                        current_block = 15908361485829668090;
                                        break;
                                    }
                                } else {
                                    if addNode.expect("non-null function pointer")(seq, cur)
                                        < 0 as c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                    }
                                    if breakOnFirstHit != 0 {
                                        current_block = 13599015263181939531;
                                        break;
                                    }
                                }
                            } else if !(*cur).ns.is_null()
                                && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                            {
                                if hasAxisRange != 0 as c_int {
                                    pos += 1;
                                    if pos == maxPos {
                                        if addNode.expect("non-null function pointer")(seq, cur)
                                            < 0 as c_int
                                        {
                                            (*ctxt).error =
                                                XPATH_MEMORY_ERROR as c_int;
                                        }
                                        current_block = 15908361485829668090;
                                        break;
                                    }
                                } else {
                                    if addNode.expect("non-null function pointer")(seq, cur)
                                        < 0 as c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                    }
                                    if breakOnFirstHit != 0 {
                                        current_block = 13599015263181939531;
                                        break;
                                    }
                                }
                            }
                        }
                    } else if axis as c_uint
                        == AXIS_NAMESPACE as c_int as c_uint
                    {
                        if (*cur).type_0 as c_uint
                            == XML_NAMESPACE_DECL as c_int as c_uint
                        {
                            if hasAxisRange != 0 as c_int {
                                pos += 1;
                                if pos == maxPos {
                                    hasNsNodes = 1 as c_int;
                                    if xmlXPathNodeSetAddNs(seq, (*xpctxt).node, cur as xmlNsPtr)
                                        < 0 as c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                    }
                                    current_block = 15908361485829668090;
                                    break;
                                }
                            } else {
                                hasNsNodes = 1 as c_int;
                                if xmlXPathNodeSetAddNs(seq, (*xpctxt).node, cur as xmlNsPtr)
                                    < 0 as c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 13599015263181939531;
                                    break;
                                }
                            }
                        }
                    } else if (*cur).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                    {
                        if prefix.is_null() {
                            if hasAxisRange != 0 as c_int {
                                pos += 1;
                                if pos == maxPos {
                                    if addNode.expect("non-null function pointer")(seq, cur)
                                        < 0 as c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                    }
                                    current_block = 15908361485829668090;
                                    break;
                                }
                            } else {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 13599015263181939531;
                                    break;
                                }
                            }
                        } else if !(*cur).ns.is_null() && xmlStrEqual(URI, (*(*cur).ns).href) != 0 {
                            if hasAxisRange != 0 as c_int {
                                pos += 1;
                                if pos == maxPos {
                                    if addNode.expect("non-null function pointer")(seq, cur)
                                        < 0 as c_int
                                    {
                                        (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                    }
                                    current_block = 15908361485829668090;
                                    break;
                                }
                            } else {
                                if addNode.expect("non-null function pointer")(seq, cur)
                                    < 0 as c_int
                                {
                                    (*ctxt).error = XPATH_MEMORY_ERROR as c_int;
                                }
                                if breakOnFirstHit != 0 {
                                    current_block = 13599015263181939531;
                                    break;
                                }
                            }
                        }
                    }
                }
                4 => {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Unimplemented block at %s:%d\n\0" as *const u8
                            as *const c_char,
                        b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0"
                            as *const u8 as *const c_char,
                        11462 as c_int,
                    );
                }
                5 => {
                    if axis as c_uint
                        == AXIS_ATTRIBUTE as c_int as c_uint
                    {
                        if (*cur).type_0 as c_uint
                            != XML_ATTRIBUTE_NODE as c_int as c_uint
                        {
                            current_block = 6838274324784804404;
                        } else {
                            current_block = 17034918949615525785;
                        }
                    } else if axis as c_uint
                        == AXIS_NAMESPACE as c_int as c_uint
                    {
                        if (*cur).type_0 as c_uint
                            != XML_NAMESPACE_DECL as c_int as c_uint
                        {
                            current_block = 6838274324784804404;
                        } else {
                            current_block = 17034918949615525785;
                        }
                    } else if (*cur).type_0 as c_uint
                        != XML_ELEMENT_NODE as c_int as c_uint
                    {
                        current_block = 6838274324784804404;
                    } else {
                        current_block = 17034918949615525785;
                    }
                    match current_block {
                        6838274324784804404 => {}
                        _ => match (*cur).type_0 as c_uint {
                            1 => {
                                current_block = 8822390976028539604;
                                match current_block {
                                    8822390976028539604 => {
                                        if xmlStrEqual(name, (*cur).name) != 0 {
                                            if prefix.is_null() {
                                                if (*cur).ns.is_null() {
                                                    if hasAxisRange != 0 as c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            ) < 0 as c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR
                                                                    as c_int;
                                                            }
                                                            current_block = 15908361485829668090;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 13599015263181939531;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !(*cur).ns.is_null()
                                                && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    if addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    10337218291927598571 => {
                                        let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                        if xmlStrEqual(name, (*attr).name) != 0 {
                                            if prefix.is_null() {
                                                if (*attr).ns.is_null()
                                                    || (*(*attr).ns).prefix.is_null()
                                                {
                                                    if hasAxisRange != 0 as c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            ) < 0 as c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR
                                                                    as c_int;
                                                            }
                                                            current_block = 15908361485829668090;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 13599015263181939531;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !(*attr).ns.is_null()
                                                && xmlStrEqual(URI, (*(*attr).ns).href) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    if addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        if (*cur).type_0 as c_uint
                                            == XML_NAMESPACE_DECL as c_int
                                                as c_uint
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !(*ns).prefix.is_null()
                                                && !name.is_null()
                                                && xmlStrEqual((*ns).prefix, name) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1 as c_int;
                                                        if xmlXPathNodeSetAddNs(
                                                            seq,
                                                            (*xpctxt).node,
                                                            cur as xmlNsPtr,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1 as c_int;
                                                    if xmlXPathNodeSetAddNs(
                                                        seq,
                                                        (*xpctxt).node,
                                                        cur as xmlNsPtr,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            2 => {
                                current_block = 10337218291927598571;
                                match current_block {
                                    8822390976028539604 => {
                                        if xmlStrEqual(name, (*cur).name) != 0 {
                                            if prefix.is_null() {
                                                if (*cur).ns.is_null() {
                                                    if hasAxisRange != 0 as c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            ) < 0 as c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR
                                                                    as c_int;
                                                            }
                                                            current_block = 15908361485829668090;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 13599015263181939531;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !(*cur).ns.is_null()
                                                && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    if addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    10337218291927598571 => {
                                        let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                        if xmlStrEqual(name, (*attr).name) != 0 {
                                            if prefix.is_null() {
                                                if (*attr).ns.is_null()
                                                    || (*(*attr).ns).prefix.is_null()
                                                {
                                                    if hasAxisRange != 0 as c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            ) < 0 as c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR
                                                                    as c_int;
                                                            }
                                                            current_block = 15908361485829668090;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 13599015263181939531;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !(*attr).ns.is_null()
                                                && xmlStrEqual(URI, (*(*attr).ns).href) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    if addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        if (*cur).type_0 as c_uint
                                            == XML_NAMESPACE_DECL as c_int
                                                as c_uint
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !(*ns).prefix.is_null()
                                                && !name.is_null()
                                                && xmlStrEqual((*ns).prefix, name) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1 as c_int;
                                                        if xmlXPathNodeSetAddNs(
                                                            seq,
                                                            (*xpctxt).node,
                                                            cur as xmlNsPtr,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1 as c_int;
                                                    if xmlXPathNodeSetAddNs(
                                                        seq,
                                                        (*xpctxt).node,
                                                        cur as xmlNsPtr,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            18 => {
                                current_block = 14862504257937502724;
                                match current_block {
                                    8822390976028539604 => {
                                        if xmlStrEqual(name, (*cur).name) != 0 {
                                            if prefix.is_null() {
                                                if (*cur).ns.is_null() {
                                                    if hasAxisRange != 0 as c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            ) < 0 as c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR
                                                                    as c_int;
                                                            }
                                                            current_block = 15908361485829668090;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 13599015263181939531;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !(*cur).ns.is_null()
                                                && xmlStrEqual(URI, (*(*cur).ns).href) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    if addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    10337218291927598571 => {
                                        let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
                                        if xmlStrEqual(name, (*attr).name) != 0 {
                                            if prefix.is_null() {
                                                if (*attr).ns.is_null()
                                                    || (*(*attr).ns).prefix.is_null()
                                                {
                                                    if hasAxisRange != 0 as c_int {
                                                        pos += 1;
                                                        if pos == maxPos {
                                                            if addNode
                                                                .expect("non-null function pointer")(
                                                                seq, cur,
                                                            ) < 0 as c_int
                                                            {
                                                                (*ctxt).error = XPATH_MEMORY_ERROR
                                                                    as c_int;
                                                            }
                                                            current_block = 15908361485829668090;
                                                            break;
                                                        }
                                                    } else {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        if breakOnFirstHit != 0 {
                                                            current_block = 13599015263181939531;
                                                            break;
                                                        }
                                                    }
                                                }
                                            } else if !(*attr).ns.is_null()
                                                && xmlStrEqual(URI, (*(*attr).ns).href) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        if addNode
                                                            .expect("non-null function pointer")(
                                                            seq, cur,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    if addNode.expect("non-null function pointer")(
                                                        seq, cur,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        if (*cur).type_0 as c_uint
                                            == XML_NAMESPACE_DECL as c_int
                                                as c_uint
                                        {
                                            let mut ns: xmlNsPtr = cur as xmlNsPtr;
                                            if !(*ns).prefix.is_null()
                                                && !name.is_null()
                                                && xmlStrEqual((*ns).prefix, name) != 0
                                            {
                                                if hasAxisRange != 0 as c_int {
                                                    pos += 1;
                                                    if pos == maxPos {
                                                        hasNsNodes = 1 as c_int;
                                                        if xmlXPathNodeSetAddNs(
                                                            seq,
                                                            (*xpctxt).node,
                                                            cur as xmlNsPtr,
                                                        ) < 0 as c_int
                                                        {
                                                            (*ctxt).error = XPATH_MEMORY_ERROR
                                                                as c_int;
                                                        }
                                                        current_block = 15908361485829668090;
                                                        break;
                                                    }
                                                } else {
                                                    hasNsNodes = 1 as c_int;
                                                    if xmlXPathNodeSetAddNs(
                                                        seq,
                                                        (*xpctxt).node,
                                                        cur as xmlNsPtr,
                                                    ) < 0 as c_int
                                                    {
                                                        (*ctxt).error = XPATH_MEMORY_ERROR
                                                            as c_int;
                                                    }
                                                    if breakOnFirstHit != 0 {
                                                        current_block = 13599015263181939531;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                    }
                }
                _ => {}
            }
            if !(!cur.is_null() && (*ctxt).error == XPATH_EXPRESSION_OK as c_int) {
                current_block = 3087853033418004326;
                break;
            }
        }
        match current_block {
            13599015263181939531 => {
                if outSeq.is_null() {
                    outSeq = seq;
                    seq = ::core::ptr::null_mut::<xmlNodeSet>();
                } else {
                    outSeq = mergeAndClear.expect("non-null function pointer")(outSeq, seq);
                }
                break;
            }
            3087853033418004326 => {
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    break;
                }
                if !predOp.is_null() && (*seq).nodeNr > 0 as c_int {
                    if hasPredicateRange != 0 as c_int {
                        xmlXPathCompOpEvalPredicate(ctxt, predOp, seq, maxPos, maxPos, hasNsNodes);
                    } else {
                        xmlXPathCompOpEvalPredicate(
                            ctxt,
                            predOp,
                            seq,
                            1 as c_int,
                            (*seq).nodeNr,
                            hasNsNodes,
                        );
                    }
                    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                        total = 0 as c_int;
                        break;
                    }
                }
                if !((*seq).nodeNr > 0 as c_int) {
                    continue;
                }
                if outSeq.is_null() {
                    outSeq = seq;
                    seq = ::core::ptr::null_mut::<xmlNodeSet>();
                } else {
                    outSeq = mergeAndClear.expect("non-null function pointer")(outSeq, seq);
                }
                if toBool != 0 {
                    break;
                }
            }
            _ => {
                if outSeq.is_null() {
                    outSeq = seq;
                    seq = ::core::ptr::null_mut::<xmlNodeSet>();
                } else {
                    outSeq = mergeAndClear.expect("non-null function pointer")(outSeq, seq);
                }
                if toBool != 0 {
                    break;
                }
            }
        }
    }
    if (*obj).boolval != 0 && !(*obj).user.is_null() {
        (*(*ctxt).value).boolval = 1 as c_int;
        (*(*ctxt).value).user = (*obj).user;
        (*obj).user = NULL;
        (*obj).boolval = 0 as c_int;
    }
    xmlXPathReleaseObject(xpctxt, obj);
    if outSeq.is_null() {
        if !seq.is_null() && (*seq).nodeNr == 0 as c_int {
            outSeq = seq;
        } else {
            outSeq = xmlXPathNodeSetCreate(::core::ptr::null_mut::<xmlNode>());
        }
    }
    if !seq.is_null() && seq != outSeq {
        xmlXPathFreeNodeSet(seq);
    }
    valuePush(ctxt, xmlXPathCacheWrapNodeSet(xpctxt, outSeq));
    (*xpctxt).node = oldContextNode;
    if !(*xpctxt).tmpNsList.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*xpctxt).tmpNsList as *mut c_void,
        );
        (*xpctxt).tmpNsList = ::core::ptr::null_mut::<xmlNsPtr>();
    }
    return total;
}
unsafe fn xmlXPathCompOpEvalFirst(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
) -> c_int {
    let mut total: c_int = 0 as c_int;
    let mut cur: c_int = 0;
    let mut comp: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    let mut arg1: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut arg2: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return 0 as c_int;
    }
    if (*(*ctxt).context).opLimit != 0 as c_ulong
        && xmlXPathCheckOpLimit(ctxt, 1 as c_ulong) < 0 as c_int
    {
        return 0 as c_int;
    }
    if (*(*ctxt).context).depth >= XPATH_MAX_RECURSION_DEPTH {
        xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as c_int);
        return 0 as c_int;
    }
    (*(*ctxt).context).depth += 1 as c_int;
    comp = (*ctxt).comp;
    match (*op).op as c_uint {
        0 => {}
        7 => {
            total = xmlXPathCompOpEvalFirst(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                first,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if !(*ctxt).value.is_null()
                && (*(*ctxt).value).type_0 as c_uint
                    == XPATH_NODESET as c_int as c_uint
                && !(*(*ctxt).value).nodesetval.is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr >= 1 as c_int
            {
                if (*(*(*ctxt).value).nodesetval).nodeNr > 1 as c_int {
                    xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
                }
                *first = *(*(*(*ctxt).value).nodesetval)
                    .nodeTab
                    .offset(0 as c_int as isize);
            }
            cur = xmlXPathCompOpEvalFirst(
                ctxt,
                (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                first,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            arg2 = valuePop(ctxt);
            arg1 = valuePop(ctxt);
            if arg1.is_null()
                || (*arg1).type_0 as c_uint
                    != XPATH_NODESET as c_int as c_uint
                || arg2.is_null()
                || (*arg2).type_0 as c_uint
                    != XPATH_NODESET as c_int as c_uint
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
                return 0 as c_int;
            }
            if (*(*ctxt).context).opLimit != 0 as c_ulong
                && (!(*arg1).nodesetval.is_null()
                    && xmlXPathCheckOpLimit(
                        ctxt,
                        (*(*arg1).nodesetval).nodeNr as c_ulong,
                    ) < 0 as c_int
                    || !(*arg2).nodesetval.is_null()
                        && xmlXPathCheckOpLimit(
                            ctxt,
                            (*(*arg2).nodesetval).nodeNr as c_ulong,
                        ) < 0 as c_int)
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
            } else {
                (*arg1).nodesetval = xmlXPathNodeSetMerge((*arg1).nodesetval, (*arg2).nodesetval);
                valuePush(ctxt, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
                if total > cur {
                    xmlXPathCompSwap(op);
                }
                total += cur;
            }
        }
        8 => {
            xmlXPathRoot(ctxt);
        }
        9 => {
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if (*op).ch2 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
            );
        }
        10 => {
            if !((*op).ch1 == -(1 as c_int)) {
                total = xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return 0 as c_int;
                }
                total += xmlXPathNodeCollectAndTest(
                    ctxt,
                    op,
                    first,
                    ::core::ptr::null_mut::<xmlNodePtr>(),
                    0 as c_int,
                );
            }
        }
        11 => {
            valuePush(
                ctxt,
                xmlXPathCacheObjectCopy((*ctxt).context, (*op).value4 as xmlXPathObjectPtr),
            );
        }
        17 => {
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEvalFirst(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                    first,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if !(*ctxt).value.is_null()
                && (*(*ctxt).value).type_0 as c_uint
                    == XPATH_NODESET as c_int as c_uint
                && !(*(*ctxt).value).nodesetval.is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as c_int
            {
                xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
            }
        }
        16 => {
            total += xmlXPathCompOpEvalFilterFirst(ctxt, op, first);
        }
        _ => {
            total += xmlXPathCompOpEval(ctxt, op);
        }
    }
    (*(*ctxt).context).depth -= 1 as c_int;
    return total;
}
unsafe fn xmlXPathCompOpEvalLast(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut last: *mut xmlNodePtr,
) -> c_int {
    let mut total: c_int = 0 as c_int;
    let mut cur: c_int = 0;
    let mut comp: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    let mut arg1: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut arg2: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return 0 as c_int;
    }
    if (*(*ctxt).context).opLimit != 0 as c_ulong
        && xmlXPathCheckOpLimit(ctxt, 1 as c_ulong) < 0 as c_int
    {
        return 0 as c_int;
    }
    if (*(*ctxt).context).depth >= XPATH_MAX_RECURSION_DEPTH {
        xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as c_int);
        return 0 as c_int;
    }
    (*(*ctxt).context).depth += 1 as c_int;
    comp = (*ctxt).comp;
    match (*op).op as c_uint {
        0 => {}
        7 => {
            total = xmlXPathCompOpEvalLast(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                last,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if !(*ctxt).value.is_null()
                && (*(*ctxt).value).type_0 as c_uint
                    == XPATH_NODESET as c_int as c_uint
                && !(*(*ctxt).value).nodesetval.is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr >= 1 as c_int
            {
                if (*(*(*ctxt).value).nodesetval).nodeNr > 1 as c_int {
                    xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
                }
                *last = *(*(*(*ctxt).value).nodesetval).nodeTab.offset(
                    ((*(*(*ctxt).value).nodesetval).nodeNr - 1 as c_int) as isize,
                );
            }
            cur = xmlXPathCompOpEvalLast(
                ctxt,
                (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                last,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            !(*ctxt).value.is_null()
                && (*(*ctxt).value).type_0 as c_uint
                    == XPATH_NODESET as c_int as c_uint
                && !(*(*ctxt).value).nodesetval.is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr >= 1 as c_int;
            arg2 = valuePop(ctxt);
            arg1 = valuePop(ctxt);
            if arg1.is_null()
                || (*arg1).type_0 as c_uint
                    != XPATH_NODESET as c_int as c_uint
                || arg2.is_null()
                || (*arg2).type_0 as c_uint
                    != XPATH_NODESET as c_int as c_uint
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
                return 0 as c_int;
            }
            if (*(*ctxt).context).opLimit != 0 as c_ulong
                && (!(*arg1).nodesetval.is_null()
                    && xmlXPathCheckOpLimit(
                        ctxt,
                        (*(*arg1).nodesetval).nodeNr as c_ulong,
                    ) < 0 as c_int
                    || !(*arg2).nodesetval.is_null()
                        && xmlXPathCheckOpLimit(
                            ctxt,
                            (*(*arg2).nodesetval).nodeNr as c_ulong,
                        ) < 0 as c_int)
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
            } else {
                (*arg1).nodesetval = xmlXPathNodeSetMerge((*arg1).nodesetval, (*arg2).nodesetval);
                valuePush(ctxt, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
                if total > cur {
                    xmlXPathCompSwap(op);
                }
                total += cur;
            }
        }
        8 => {
            xmlXPathRoot(ctxt);
        }
        9 => {
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if (*op).ch2 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
            );
        }
        10 => {
            if !((*op).ch1 == -(1 as c_int)) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return 0 as c_int;
                }
                total += xmlXPathNodeCollectAndTest(
                    ctxt,
                    op,
                    ::core::ptr::null_mut::<xmlNodePtr>(),
                    last,
                    0 as c_int,
                );
            }
        }
        11 => {
            valuePush(
                ctxt,
                xmlXPathCacheObjectCopy((*ctxt).context, (*op).value4 as xmlXPathObjectPtr),
            );
        }
        17 => {
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEvalLast(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                    last,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if !(*ctxt).value.is_null()
                && (*(*ctxt).value).type_0 as c_uint
                    == XPATH_NODESET as c_int as c_uint
                && !(*(*ctxt).value).nodesetval.is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as c_int
            {
                xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
            }
        }
        _ => {
            total += xmlXPathCompOpEval(ctxt, op);
        }
    }
    (*(*ctxt).context).depth -= 1 as c_int;
    return total;
}
unsafe fn xmlXPathCompOpEvalFilterFirst(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut first: *mut xmlNodePtr,
) -> c_int {
    let mut total: c_int = 0 as c_int;
    let mut comp: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut set: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return 0 as c_int;
    }
    comp = (*ctxt).comp;
    if (*op).ch1 != -(1 as c_int)
        && (*op).ch2 != -(1 as c_int)
        && (*(*comp).steps.offset((*op).ch1 as isize)).op as c_uint
            == XPATH_OP_SORT as c_int as c_uint
        && (*(*comp).steps.offset((*op).ch2 as isize)).op as c_uint
            == XPATH_OP_SORT as c_int as c_uint
    {
        let mut f: c_int = (*(*comp).steps.offset((*op).ch2 as isize)).ch1;
        if f != -(1 as c_int)
            && (*(*comp).steps.offset(f as isize)).op as c_uint
                == XPATH_OP_FUNCTION as c_int as c_uint
            && (*(*comp).steps.offset(f as isize)).value5.is_null()
            && (*(*comp).steps.offset(f as isize)).value == 0 as c_int
            && !(*(*comp).steps.offset(f as isize)).value4.is_null()
            && xmlStrEqual(
                (*(*comp).steps.offset(f as isize)).value4 as *const xmlChar,
                b"last\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
        {
            let mut last: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            total += xmlXPathCompOpEvalLast(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                &raw mut last,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if !(*ctxt).value.is_null()
                && (*(*ctxt).value).type_0 as c_uint
                    == XPATH_NODESET as c_int as c_uint
                && !(*(*ctxt).value).nodesetval.is_null()
                && !(*(*(*ctxt).value).nodesetval).nodeTab.is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as c_int
            {
                xmlXPathNodeSetKeepLast((*(*ctxt).value).nodesetval);
                *first = *(*(*(*ctxt).value).nodesetval).nodeTab;
            }
            return total;
        }
    }
    if (*op).ch1 != -(1 as c_int) {
        total += xmlXPathCompOpEval(
            ctxt,
            (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
        );
    }
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return 0 as c_int;
    }
    if (*op).ch2 == -(1 as c_int) {
        return total;
    }
    if (*ctxt).value.is_null() {
        return total;
    }
    if (*ctxt).value.is_null()
        || (*(*ctxt).value).type_0 as c_uint
            != XPATH_NODESET as c_int as c_uint
    {
        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
        return 0 as c_int;
    }
    obj = valuePop(ctxt);
    set = (*obj).nodesetval;
    if !set.is_null() {
        xmlXPathNodeSetFilter(
            ctxt,
            set,
            (*op).ch2,
            1 as c_int,
            1 as c_int,
            1 as c_int,
        );
        if (*set).nodeNr > 0 as c_int {
            *first = *(*set).nodeTab.offset(0 as c_int as isize);
        }
    }
    valuePush(ctxt, obj);
    return total;
}
fn xmlXPathCompOpEval(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
) -> c_int { unsafe {
    let mut total: c_int = 0 as c_int;
    let mut equal: c_int = 0;
    let mut ret: c_int = 0;
    let mut comp: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    let mut arg1: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut arg2: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        return 0 as c_int;
    }
    if (*(*ctxt).context).opLimit != 0 as c_ulong
        && xmlXPathCheckOpLimit(ctxt, 1 as c_ulong) < 0 as c_int
    {
        return 0 as c_int;
    }
    if (*(*ctxt).context).depth >= XPATH_MAX_RECURSION_DEPTH {
        xmlXPathErr(ctxt, XPATH_RECURSION_LIMIT_EXCEEDED as c_int);
        return 0 as c_int;
    }
    (*(*ctxt).context).depth += 1 as c_int;
    comp = (*ctxt).comp;
    let mut current_block_223: u64;
    match (*op).op as c_uint {
        0 => {}
        1 => {
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            xmlXPathBooleanFunction(ctxt, 1 as c_int);
            if !((*ctxt).value.is_null() || (*(*ctxt).value).boolval == 0 as c_int) {
                arg2 = valuePop(ctxt);
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != 0 {
                    xmlXPathFreeObject(arg2);
                } else {
                    xmlXPathBooleanFunction(ctxt, 1 as c_int);
                    if !(*ctxt).value.is_null() {
                        (*(*ctxt).value).boolval &= (*arg2).boolval;
                    }
                    xmlXPathReleaseObject((*ctxt).context, arg2);
                }
            }
        }
        2 => {
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            xmlXPathBooleanFunction(ctxt, 1 as c_int);
            if !((*ctxt).value.is_null() || (*(*ctxt).value).boolval == 1 as c_int) {
                arg2 = valuePop(ctxt);
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != 0 {
                    xmlXPathFreeObject(arg2);
                } else {
                    xmlXPathBooleanFunction(ctxt, 1 as c_int);
                    if !(*ctxt).value.is_null() {
                        (*(*ctxt).value).boolval |= (*arg2).boolval;
                    }
                    xmlXPathReleaseObject((*ctxt).context, arg2);
                }
            }
        }
        3 => {
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if (*op).value != 0 {
                equal = xmlXPathEqualValues(ctxt);
            } else {
                equal = xmlXPathNotEqualValues(ctxt);
            }
            valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, equal));
        }
        4 => {
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            ret = xmlXPathCompareValues(ctxt, (*op).value, (*op).value2);
            valuePush(ctxt, xmlXPathCacheNewBoolean((*ctxt).context, ret));
        }
        5 => {
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if (*op).ch2 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if (*op).value == 0 as c_int {
                xmlXPathSubValues(ctxt);
            } else if (*op).value == 1 as c_int {
                xmlXPathAddValues(ctxt);
            } else if (*op).value == 2 as c_int {
                xmlXPathValueFlipSign(ctxt);
            } else if (*op).value == 3 as c_int {
                if !(*ctxt).value.is_null()
                    && (*(*ctxt).value).type_0 as c_uint
                        != XPATH_NUMBER as c_int as c_uint
                {
                    xmlXPathNumberFunction(ctxt, 1 as c_int);
                }
                if (*ctxt).value.is_null()
                    || (*(*ctxt).value).type_0 as c_uint
                        != XPATH_NUMBER as c_int as c_uint
                {
                    xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
                    return 0 as c_int;
                }
            }
        }
        6 => {
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if (*op).value == 0 as c_int {
                xmlXPathMultValues(ctxt);
            } else if (*op).value == 1 as c_int {
                xmlXPathDivValues(ctxt);
            } else if (*op).value == 2 as c_int {
                xmlXPathModValues(ctxt);
            }
        }
        7 => {
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            total += xmlXPathCompOpEval(
                ctxt,
                (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
            );
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            arg2 = valuePop(ctxt);
            arg1 = valuePop(ctxt);
            if arg1.is_null()
                || (*arg1).type_0 as c_uint
                    != XPATH_NODESET as c_int as c_uint
                || arg2.is_null()
                || (*arg2).type_0 as c_uint
                    != XPATH_NODESET as c_int as c_uint
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
                xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
                return 0 as c_int;
            }
            if (*(*ctxt).context).opLimit != 0 as c_ulong
                && (!(*arg1).nodesetval.is_null()
                    && xmlXPathCheckOpLimit(
                        ctxt,
                        (*(*arg1).nodesetval).nodeNr as c_ulong,
                    ) < 0 as c_int
                    || !(*arg2).nodesetval.is_null()
                        && xmlXPathCheckOpLimit(
                            ctxt,
                            (*(*arg2).nodesetval).nodeNr as c_ulong,
                        ) < 0 as c_int)
            {
                xmlXPathReleaseObject((*ctxt).context, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
            } else {
                if (*arg1).nodesetval.is_null()
                    || !(*arg2).nodesetval.is_null()
                        && (*(*arg2).nodesetval).nodeNr != 0 as c_int
                {
                    (*arg1).nodesetval =
                        xmlXPathNodeSetMerge((*arg1).nodesetval, (*arg2).nodesetval);
                }
                valuePush(ctxt, arg1);
                xmlXPathReleaseObject((*ctxt).context, arg2);
            }
        }
        8 => {
            xmlXPathRoot(ctxt);
        }
        9 => {
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if (*op).ch2 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            valuePush(
                ctxt,
                xmlXPathCacheNewNodeSet((*ctxt).context, (*(*ctxt).context).node),
            );
        }
        10 => {
            if !((*op).ch1 == -(1 as c_int)) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return 0 as c_int;
                }
                total += xmlXPathNodeCollectAndTest(
                    ctxt,
                    op,
                    ::core::ptr::null_mut::<xmlNodePtr>(),
                    ::core::ptr::null_mut::<xmlNodePtr>(),
                    0 as c_int,
                );
            }
        }
        11 => {
            valuePush(
                ctxt,
                xmlXPathCacheObjectCopy((*ctxt).context, (*op).value4 as xmlXPathObjectPtr),
            );
        }
        12 => {
            let mut val: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*op).value5.is_null() {
                val = xmlXPathVariableLookup((*ctxt).context, (*op).value4 as *const xmlChar);
                if val.is_null() {
                    xmlXPathErr(ctxt, XPATH_UNDEF_VARIABLE_ERROR as c_int);
                    return 0 as c_int;
                }
                valuePush(ctxt, val);
            } else {
                let mut URI: *const xmlChar = ::core::ptr::null::<xmlChar>();
                URI = xmlXPathNsLookup((*ctxt).context, (*op).value5 as *const xmlChar);
                if URI.is_null() {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"xmlXPathCompOpEval: variable %s bound to undefined prefix %s\n\0"
                            as *const u8 as *const c_char,
                        (*op).value4 as *mut c_char,
                        (*op).value5 as *mut c_char,
                    );
                    (*ctxt).error = XPATH_UNDEF_PREFIX_ERROR as c_int;
                } else {
                    val = xmlXPathVariableLookupNS(
                        (*ctxt).context,
                        (*op).value4 as *const xmlChar,
                        URI,
                    );
                    if val.is_null() {
                        xmlXPathErr(ctxt, XPATH_UNDEF_VARIABLE_ERROR as c_int);
                        return 0 as c_int;
                    }
                    valuePush(ctxt, val);
                }
            }
        }
        13 => {
            let mut func: xmlXPathFunction = None;
            let mut oldFunc: *const xmlChar = ::core::ptr::null::<xmlChar>();
            let mut oldFuncURI: *const xmlChar = ::core::ptr::null::<xmlChar>();
            let mut i: c_int = 0;
            let mut frame: c_int = 0;
            frame = (*ctxt).valueNr;
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    current_block_223 = 6950536787749910113;
                } else {
                    current_block_223 = 13256895345714485905;
                }
            } else {
                current_block_223 = 13256895345714485905;
            }
            match current_block_223 {
                6950536787749910113 => {}
                _ => {
                    if (*ctxt).valueNr < frame + (*op).value {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"xmlXPathCompOpEval: parameter error\n\0" as *const u8
                                as *const c_char,
                        );
                        (*ctxt).error = XPATH_INVALID_OPERAND as c_int;
                    } else {
                        i = 0 as c_int;
                        while i < (*op).value {
                            if (*(*ctxt)
                                .valueTab
                                .offset(((*ctxt).valueNr - 1 as c_int - i) as isize))
                            .is_null()
                            {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"xmlXPathCompOpEval: parameter error\n\0" as *const u8
                                        as *const c_char,
                                );
                                (*ctxt).error = XPATH_INVALID_OPERAND as c_int;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        if (*op).cache.is_some() {
                            func = (*op).cache;
                            current_block_223 = 1421636490742020198;
                        } else {
                            let mut URI_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
                            if (*op).value5.is_null() {
                                func = xmlXPathFunctionLookup(
                                    (*ctxt).context,
                                    (*op).value4 as *const xmlChar,
                                );
                                current_block_223 = 7999014830792590863;
                            } else {
                                URI_0 = xmlXPathNsLookup(
                                    (*ctxt).context,
                                    (*op).value5 as *const xmlChar,
                                );
                                if URI_0.is_null() {
                                    (*__xmlGenericError())
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        *__xmlGenericErrorContext(),
                                        b"xmlXPathCompOpEval: function %s bound to undefined prefix %s\n\0"
                                            as *const u8 as *const c_char,
                                        (*op).value4 as *mut c_char,
                                        (*op).value5 as *mut c_char,
                                    );
                                    (*ctxt).error = XPATH_UNDEF_PREFIX_ERROR as c_int;
                                    current_block_223 = 6950536787749910113;
                                } else {
                                    func = xmlXPathFunctionLookupNS(
                                        (*ctxt).context,
                                        (*op).value4 as *const xmlChar,
                                        URI_0,
                                    );
                                    current_block_223 = 7999014830792590863;
                                }
                            }
                            match current_block_223 {
                                6950536787749910113 => {}
                                _ => {
                                    if func.is_none() {
                                        (*__xmlGenericError()).expect("non-null function pointer")(
                                            *__xmlGenericErrorContext(),
                                            b"xmlXPathCompOpEval: function %s not found\n\0"
                                                as *const u8
                                                as *const c_char,
                                            (*op).value4 as *mut c_char,
                                        );
                                        xmlXPathErr(
                                            ctxt,
                                            XPATH_UNKNOWN_FUNC_ERROR as c_int,
                                        );
                                        return 0 as c_int;
                                    }
                                    (*op).cache = func;
                                    (*op).cacheURI = URI_0 as *mut c_void;
                                    current_block_223 = 1421636490742020198;
                                }
                            }
                        }
                        match current_block_223 {
                            6950536787749910113 => {}
                            _ => {
                                oldFunc = (*(*ctxt).context).function;
                                oldFuncURI = (*(*ctxt).context).functionURI;
                                (*(*ctxt).context).function = (*op).value4 as *const xmlChar;
                                (*(*ctxt).context).functionURI = (*op).cacheURI as *const xmlChar;
                                func.expect("non-null function pointer")(ctxt, (*op).value);
                                (*(*ctxt).context).function = oldFunc;
                                (*(*ctxt).context).functionURI = oldFuncURI;
                                if (*ctxt).error == XPATH_EXPRESSION_OK as c_int
                                    && (*ctxt).valueNr != frame + 1 as c_int
                                {
                                    xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
                                    return 0 as c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        14 => {
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return 0 as c_int;
                }
            }
            if (*op).ch2 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return 0 as c_int;
                }
            }
        }
        15 | 16 => {
            let mut obj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            let mut set: xmlNodeSetPtr = ::core::ptr::null_mut::<xmlNodeSet>();
            if (*op).ch1 != -(1 as c_int)
                && (*op).ch2 != -(1 as c_int)
                && ((*(*comp).steps.offset((*op).ch1 as isize)).op as c_uint
                    == XPATH_OP_SORT as c_int as c_uint
                    || (*(*comp).steps.offset((*op).ch1 as isize)).op as c_uint
                        == XPATH_OP_FILTER as c_int as c_uint)
                && (*(*comp).steps.offset((*op).ch2 as isize)).op as c_uint
                    == XPATH_OP_VALUE as c_int as c_uint
            {
                let mut val_0: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
                val_0 = (*(*comp).steps.offset((*op).ch2 as isize)).value4 as xmlXPathObjectPtr;
                if !val_0.is_null()
                    && (*val_0).type_0 as c_uint
                        == XPATH_NUMBER as c_int as c_uint
                    && (*val_0).floatval == 1.0f64
                {
                    let mut first: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                    total += xmlXPathCompOpEvalFirst(
                        ctxt,
                        (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                        &raw mut first,
                    );
                    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                        return 0 as c_int;
                    }
                    if !(*ctxt).value.is_null()
                        && (*(*ctxt).value).type_0 as c_uint
                            == XPATH_NODESET as c_int as c_uint
                        && !(*(*ctxt).value).nodesetval.is_null()
                        && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as c_int
                    {
                        xmlXPathNodeSetClearFromPos(
                            (*(*ctxt).value).nodesetval,
                            1 as c_int,
                            1 as c_int,
                        );
                    }
                    current_block_223 = 6950536787749910113;
                } else {
                    current_block_223 = 10996290961880923853;
                }
            } else {
                current_block_223 = 10996290961880923853;
            }
            match current_block_223 {
                6950536787749910113 => {}
                _ => {
                    if (*op).ch1 != -(1 as c_int)
                        && (*op).ch2 != -(1 as c_int)
                        && (*(*comp).steps.offset((*op).ch1 as isize)).op as c_uint
                            == XPATH_OP_SORT as c_int as c_uint
                        && (*(*comp).steps.offset((*op).ch2 as isize)).op as c_uint
                            == XPATH_OP_SORT as c_int as c_uint
                    {
                        let mut f: c_int =
                            (*(*comp).steps.offset((*op).ch2 as isize)).ch1;
                        if f != -(1 as c_int)
                            && (*(*comp).steps.offset(f as isize)).op as c_uint
                                == XPATH_OP_FUNCTION as c_int as c_uint
                            && (*(*comp).steps.offset(f as isize)).value5.is_null()
                            && (*(*comp).steps.offset(f as isize)).value == 0 as c_int
                            && !(*(*comp).steps.offset(f as isize)).value4.is_null()
                            && xmlStrEqual(
                                (*(*comp).steps.offset(f as isize)).value4 as *const xmlChar,
                                b"last\0" as *const u8 as *const c_char
                                    as *mut xmlChar,
                            ) != 0
                        {
                            let mut last: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                            total += xmlXPathCompOpEvalLast(
                                ctxt,
                                (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                                &raw mut last,
                            );
                            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                                return 0 as c_int;
                            }
                            if !(*ctxt).value.is_null()
                                && (*(*ctxt).value).type_0 as c_uint
                                    == XPATH_NODESET as c_int as c_uint
                                && !(*(*ctxt).value).nodesetval.is_null()
                                && !(*(*(*ctxt).value).nodesetval).nodeTab.is_null()
                                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as c_int
                            {
                                xmlXPathNodeSetKeepLast((*(*ctxt).value).nodesetval);
                            }
                            current_block_223 = 6950536787749910113;
                        } else {
                            current_block_223 = 5153559628248805366;
                        }
                    } else {
                        current_block_223 = 5153559628248805366;
                    }
                    match current_block_223 {
                        6950536787749910113 => {}
                        _ => {
                            if (*op).ch1 != -(1 as c_int) {
                                total += xmlXPathCompOpEval(
                                    ctxt,
                                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                                );
                            }
                            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                                return 0 as c_int;
                            }
                            if !((*op).ch2 == -(1 as c_int)) {
                                if !(*ctxt).value.is_null() {
                                    if (*ctxt).value.is_null()
                                        || (*(*ctxt).value).type_0 as c_uint
                                            != XPATH_NODESET as c_int
                                                as c_uint
                                    {
                                        xmlXPathErr(ctxt, XPATH_INVALID_TYPE as c_int);
                                        return 0 as c_int;
                                    }
                                    obj = valuePop(ctxt);
                                    set = (*obj).nodesetval;
                                    if !set.is_null() {
                                        xmlXPathNodeSetFilter(
                                            ctxt,
                                            set,
                                            (*op).ch2,
                                            1 as c_int,
                                            (*set).nodeNr,
                                            1 as c_int,
                                        );
                                    }
                                    valuePush(ctxt, obj);
                                }
                            }
                        }
                    }
                }
            }
        }
        17 => {
            if (*op).ch1 != -(1 as c_int) {
                total += xmlXPathCompOpEval(
                    ctxt,
                    (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
            }
            if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                return 0 as c_int;
            }
            if !(*ctxt).value.is_null()
                && (*(*ctxt).value).type_0 as c_uint
                    == XPATH_NODESET as c_int as c_uint
                && !(*(*ctxt).value).nodesetval.is_null()
                && (*(*(*ctxt).value).nodesetval).nodeNr > 1 as c_int
            {
                xmlXPathNodeSetSort((*(*ctxt).value).nodesetval);
            }
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"XPath: unknown precompiled operation %d\n\0" as *const u8
                    as *const c_char,
                (*op).op as c_uint,
            );
            (*ctxt).error = XPATH_INVALID_OPERAND as c_int;
        }
    }
    (*(*ctxt).context).depth -= 1 as c_int;
    return total;
} }
fn xmlXPathCompOpEvalToBoolean(
    mut ctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
    mut isPredicate: c_int,
) -> c_int { unsafe {
    let mut resObj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    loop {
        if (*(*ctxt).context).opLimit != 0 as c_ulong
            && xmlXPathCheckOpLimit(ctxt, 1 as c_ulong) < 0 as c_int
        {
            return 0 as c_int;
        }
        match (*op).op as c_uint {
            0 => return 0 as c_int,
            11 => {
                resObj = (*op).value4 as xmlXPathObjectPtr;
                if isPredicate != 0 {
                    return xmlXPathEvaluatePredicateResult(ctxt, resObj);
                }
                return xmlXPathCastToBoolean(resObj);
            }
            17 => {
                if (*op).ch1 != -(1 as c_int) {
                    op = (*(*ctxt).comp).steps.offset((*op).ch1 as isize) as *mut xmlXPathStepOp
                        as xmlXPathStepOpPtr;
                } else {
                    return 0 as c_int;
                }
            }
            10 => {
                if (*op).ch1 == -(1 as c_int) {
                    return 0 as c_int;
                }
                xmlXPathCompOpEval(
                    ctxt,
                    (*(*ctxt).comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return -(1 as c_int);
                }
                xmlXPathNodeCollectAndTest(
                    ctxt,
                    op,
                    ::core::ptr::null_mut::<xmlNodePtr>(),
                    ::core::ptr::null_mut::<xmlNodePtr>(),
                    1 as c_int,
                );
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return -(1 as c_int);
                }
                resObj = valuePop(ctxt);
                if resObj.is_null() {
                    return -(1 as c_int);
                }
                break;
            }
            _ => {
                xmlXPathCompOpEval(ctxt, op);
                if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
                    return -(1 as c_int);
                }
                resObj = valuePop(ctxt);
                if resObj.is_null() {
                    return -(1 as c_int);
                }
                break;
            }
        }
    }
    if !resObj.is_null() {
        let mut res: c_int = 0;
        if (*resObj).type_0 as c_uint
            == XPATH_BOOLEAN as c_int as c_uint
        {
            res = (*resObj).boolval;
        } else if isPredicate != 0 {
            res = xmlXPathEvaluatePredicateResult(ctxt, resObj);
        } else {
            res = xmlXPathCastToBoolean(resObj);
        }
        xmlXPathReleaseObject((*ctxt).context, resObj);
        return res;
    }
    return 0 as c_int;
} }
unsafe fn xmlXPathRunStreamEval(
    mut ctxt: xmlXPathContextPtr,
    mut comp: xmlPatternPtr,
    mut resultSeq: *mut xmlXPathObjectPtr,
    mut toBool: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut max_depth: c_int = 0;
    let mut min_depth: c_int = 0;
    let mut from_root: c_int = 0;
    let mut ret: c_int = 0;
    let mut depth: c_int = 0;
    let mut eval_all_nodes: c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut limit: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut patstream: xmlStreamCtxtPtr = ::core::ptr::null_mut::<xmlStreamCtxt>();
    if ctxt.is_null() || comp.is_null() {
        return -(1 as c_int);
    }
    max_depth = xmlPatternMaxDepth(comp);
    if max_depth == -(1 as c_int) {
        return -(1 as c_int);
    }
    if max_depth == -(2 as c_int) {
        max_depth = 10000 as c_int;
    }
    min_depth = xmlPatternMinDepth(comp);
    if min_depth == -(1 as c_int) {
        return -(1 as c_int);
    }
    from_root = xmlPatternFromRoot(comp);
    if from_root < 0 as c_int {
        return -(1 as c_int);
    }
    if toBool == 0 {
        if resultSeq.is_null() {
            return -(1 as c_int);
        }
        *resultSeq = xmlXPathCacheNewNodeSet(ctxt, ::core::ptr::null_mut::<xmlNode>());
        if (*resultSeq).is_null() {
            return -(1 as c_int);
        }
    }
    if min_depth == 0 as c_int {
        if from_root != 0 {
            if toBool != 0 {
                return 1 as c_int;
            }
            xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, (*ctxt).doc as xmlNodePtr);
        } else {
            if toBool != 0 {
                return 1 as c_int;
            }
            xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, (*ctxt).node);
        }
    }
    if max_depth == 0 as c_int {
        return 0 as c_int;
    }
    if from_root != 0 {
        cur = (*ctxt).doc as xmlNodePtr;
    } else if !(*ctxt).node.is_null() {
        match (*(*ctxt).node).type_0 as c_uint {
            1 | 9 | 11 | 13 => {
                cur = (*ctxt).node;
            }
            2 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 14 | 10 | 15 | 16 | 17 | 18 | 19 | 20 | _ => {}
        }
        limit = cur;
    }
    if cur.is_null() {
        return 0 as c_int;
    }
    patstream = xmlPatternGetStreamCtxt(comp);
    if patstream.is_null() {
        return 0 as c_int;
    }
    eval_all_nodes = xmlStreamWantsAnyNode(patstream);
    if from_root != 0 {
        ret = xmlStreamPush(
            patstream,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        if ret < 0 as c_int {
            current_block = 7990025728955927862;
        } else if ret == 1 as c_int {
            if toBool != 0 {
                current_block = 17345290900215380028;
            } else {
                xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, cur);
                current_block = 7990025728955927862;
            }
        } else {
            current_block = 7990025728955927862;
        }
    } else {
        current_block = 7990025728955927862;
    }
    match current_block {
        7990025728955927862 => {
            depth = 0 as c_int;
            '_scan_children: loop {
                if (*cur).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
                {
                    current_block = 1010597913873238679;
                    break;
                }
                if !(*cur).children.is_null() && depth < max_depth {
                    if (*(*cur).children).type_0 as c_uint
                        != XML_ENTITY_DECL as c_int as c_uint
                    {
                        cur = (*cur).children as xmlNodePtr;
                        depth += 1;
                        if (*cur).type_0 as c_uint
                            != XML_DTD_NODE as c_int as c_uint
                        {
                            current_block = 13321564401369230990;
                        } else {
                            current_block = 5722677567366458307;
                        }
                    } else {
                        current_block = 5722677567366458307;
                    }
                } else {
                    current_block = 5722677567366458307;
                }
                match current_block {
                    5722677567366458307 => {
                        if cur == limit {
                            current_block = 1010597913873238679;
                            break;
                        }
                        loop {
                            if (*cur).next.is_null() {
                                current_block = 14541395414537699361;
                                break;
                            }
                            cur = (*cur).next as xmlNodePtr;
                            if (*cur).type_0 as c_uint
                                != XML_ENTITY_DECL as c_int as c_uint
                                && (*cur).type_0 as c_uint
                                    != XML_DTD_NODE as c_int as c_uint
                            {
                                current_block = 17747245473264231573;
                                break;
                            }
                        }
                        match current_block {
                            17747245473264231573 => {}
                            _ => {
                                loop {
                                    cur = (*cur).parent as xmlNodePtr;
                                    depth -= 1;
                                    if cur.is_null()
                                        || cur == limit
                                        || (*cur).type_0 as c_uint
                                            == XML_DOCUMENT_NODE as c_int
                                                as c_uint
                                    {
                                        current_block = 1010597913873238679;
                                        break '_scan_children;
                                    }
                                    if (*cur).type_0 as c_uint
                                        == XML_ELEMENT_NODE as c_int
                                            as c_uint
                                    {
                                        ret = xmlStreamPop(patstream);
                                    } else if eval_all_nodes != 0
                                        && ((*cur).type_0 as c_uint
                                            == XML_TEXT_NODE as c_int
                                                as c_uint
                                            || (*cur).type_0 as c_uint
                                                == XML_CDATA_SECTION_NODE as c_int
                                                    as c_uint
                                            || (*cur).type_0 as c_uint
                                                == XML_COMMENT_NODE as c_int
                                                    as c_uint
                                            || (*cur).type_0 as c_uint
                                                == XML_PI_NODE as c_int
                                                    as c_uint)
                                    {
                                        ret = xmlStreamPop(patstream);
                                    }
                                    if !(*cur).next.is_null() {
                                        cur = (*cur).next as xmlNodePtr;
                                        break;
                                    } else if cur.is_null() {
                                        break;
                                    }
                                }
                                current_block = 13321564401369230990;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    13321564401369230990 => {
                        if !(!cur.is_null() && depth >= 0 as c_int) {
                            current_block = 1010597913873238679;
                            break;
                        }
                    }
                    _ => {}
                }
                's_267: loop {
                    if (*ctxt).opLimit != 0 as c_ulong {
                        if (*ctxt).opCount >= (*ctxt).opLimit {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"XPath operation limit exceeded\n\0" as *const u8
                                    as *const c_char,
                            );
                            xmlFreeStreamCtxt(patstream);
                            return -(1 as c_int);
                        }
                        (*ctxt).opCount = (*ctxt).opCount.wrapping_add(1);
                    }
                    match (*cur).type_0 as c_uint {
                        1 | 3 | 4 | 8 | 7 => {}
                        _ => {
                            break;
                        }
                    }
                    if (*cur).type_0 as c_uint
                        == XML_ELEMENT_NODE as c_int as c_uint
                    {
                        ret = xmlStreamPush(
                            patstream,
                            (*cur).name,
                            if !(*cur).ns.is_null() {
                                (*(*cur).ns).href
                            } else {
                                ::core::ptr::null::<xmlChar>()
                            },
                        );
                    } else {
                        if !(eval_all_nodes != 0) {
                            break;
                        }
                        ret = xmlStreamPushNode(
                            patstream,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                            (*cur).type_0 as c_int,
                        );
                    }
                    if !(ret < 0 as c_int) {
                        if ret == 1 as c_int {
                            if toBool != 0 {
                                current_block = 17345290900215380028;
                                break '_scan_children;
                            }
                            if xmlXPathNodeSetAddUnique((**resultSeq).nodesetval, cur)
                                < 0 as c_int
                            {
                                (*ctxt).lastError.domain = XML_FROM_XPATH as c_int;
                                (*ctxt).lastError.code = XML_ERR_NO_MEMORY as c_int;
                            }
                        }
                    }
                    if !((*cur).children.is_null() || depth >= max_depth) {
                        break;
                    }
                    ret = xmlStreamPop(patstream);
                    loop {
                        if (*cur).next.is_null() {
                            break 's_267;
                        }
                        cur = (*cur).next as xmlNodePtr;
                        if (*cur).type_0 as c_uint
                            != XML_ENTITY_DECL as c_int as c_uint
                            && (*cur).type_0 as c_uint
                                != XML_DTD_NODE as c_int as c_uint
                        {
                            break;
                        }
                    }
                }
            }
            match current_block {
                17345290900215380028 => {}
                _ => {
                    if !patstream.is_null() {
                        xmlFreeStreamCtxt(patstream);
                    }
                    return 0 as c_int;
                }
            }
        }
        _ => {}
    }
    if !patstream.is_null() {
        xmlFreeStreamCtxt(patstream);
    }
    return 1 as c_int;
}
fn xmlXPathRunEval(
    mut ctxt: xmlXPathParserContextPtr,
    mut toBool: c_int,
) -> c_int { unsafe {
    let mut comp: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    let mut oldDepth: c_int = 0;
    if ctxt.is_null() || (*ctxt).comp.is_null() {
        return -(1 as c_int);
    }
    if (*ctxt).valueTab.is_null() {
        (*ctxt).valueTab = xmlMalloc.expect("non-null function pointer")(
            (10 as size_t).wrapping_mul(::core::mem::size_of::<xmlXPathObjectPtr>() as size_t),
        ) as *mut xmlXPathObjectPtr;
        if (*ctxt).valueTab.is_null() {
            xmlXPathPErrMemory(
                ctxt,
                b"creating evaluation context\n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*ctxt).valueNr = 0 as c_int;
        (*ctxt).valueMax = 10 as c_int;
        (*ctxt).value = ::core::ptr::null_mut::<xmlXPathObject>();
    }
    if !(*(*ctxt).comp).stream.is_null() {
        let mut res: c_int = 0;
        if toBool != 0 {
            res = xmlXPathRunStreamEval(
                (*ctxt).context,
                (*(*ctxt).comp).stream,
                ::core::ptr::null_mut::<xmlXPathObjectPtr>(),
                1 as c_int,
            );
            if res != -(1 as c_int) {
                return res;
            }
        } else {
            let mut resObj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
            res = xmlXPathRunStreamEval(
                (*ctxt).context,
                (*(*ctxt).comp).stream,
                &raw mut resObj,
                0 as c_int,
            );
            if res != -(1 as c_int) && !resObj.is_null() {
                valuePush(ctxt, resObj);
                return 0 as c_int;
            }
            if !resObj.is_null() {
                xmlXPathReleaseObject((*ctxt).context, resObj);
            }
        }
    }
    comp = (*ctxt).comp;
    if (*comp).last < 0 as c_int {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlXPathRunEval: last is less than zero\n\0" as *const u8
                as *const c_char,
        );
        return -(1 as c_int);
    }
    oldDepth = (*(*ctxt).context).depth;
    if toBool != 0 {
        return xmlXPathCompOpEvalToBoolean(
            ctxt,
            (*comp).steps.offset((*comp).last as isize) as xmlXPathStepOpPtr,
            0 as c_int,
        );
    } else {
        xmlXPathCompOpEval(
            ctxt,
            (*comp).steps.offset((*comp).last as isize) as xmlXPathStepOpPtr,
        );
    }
    (*(*ctxt).context).depth = oldDepth;
    return 0 as c_int;
} }
#[inline]
pub fn xmlXPathEvalPredicate(
    mut ctxt: xmlXPathContextPtr,
    mut res: xmlXPathObjectPtr,
) -> c_int { unsafe {
    if ctxt.is_null() || res.is_null() {
        return 0 as c_int;
    }
    match (*res).type_0 as c_uint {
        2 => return (*res).boolval,
        3 => {
            return ((*res).floatval == (*ctxt).proximityPosition as c_double)
                as c_int;
        }
        1 | 9 => {
            if (*res).nodesetval.is_null() {
                return 0 as c_int;
            }
            return ((*(*res).nodesetval).nodeNr != 0 as c_int) as c_int;
        }
        4 => {
            return (!(*res).stringval.is_null()
                && xmlStrlen((*res).stringval) != 0 as c_int)
                as c_int;
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Internal error at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                    as *const c_char,
                13098 as c_int,
            );
        }
    }
    return 0 as c_int;
} }
#[inline]
pub fn xmlXPathEvaluatePredicateResult(
    mut ctxt: xmlXPathParserContextPtr,
    mut res: xmlXPathObjectPtr,
) -> c_int { unsafe {
    if ctxt.is_null() || res.is_null() {
        return 0 as c_int;
    }
    match (*res).type_0 as c_uint {
        2 => return (*res).boolval,
        3 => {
            return ((*res).floatval
                == (*(*ctxt).context).proximityPosition as c_double)
                as c_int;
        }
        1 | 9 => {
            if (*res).nodesetval.is_null() {
                return 0 as c_int;
            }
            return ((*(*res).nodesetval).nodeNr != 0 as c_int) as c_int;
        }
        4 => {
            return (!(*res).stringval.is_null()
                && *(*res).stringval.offset(0 as c_int as isize) as c_int
                    != 0 as c_int) as c_int;
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Internal error at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                    as *const c_char,
                13149 as c_int,
            );
        }
    }
    return 0 as c_int;
} }
unsafe fn xmlXPathTryStreamCompile(
    mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    let mut stream: xmlPatternPtr = ::core::ptr::null_mut::<xmlPattern>();
    let mut comp: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    let mut namespaces: *mut *const xmlChar = ::core::ptr::null_mut::<*const xmlChar>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    if xmlStrchr(str, '[' as i32 as xmlChar).is_null()
        && xmlStrchr(str, '(' as i32 as xmlChar).is_null()
        && xmlStrchr(str, '@' as i32 as xmlChar).is_null()
    {
        let mut tmp: *const xmlChar = ::core::ptr::null::<xmlChar>();
        tmp = xmlStrchr(str, ':' as i32 as xmlChar);
        if !tmp.is_null()
            && (ctxt.is_null()
                || (*ctxt).nsNr == 0 as c_int
                || *tmp.offset(1 as c_int as isize) as c_int
                    == ':' as i32)
        {
            return ::core::ptr::null_mut::<xmlXPathCompExpr>();
        }
        if !ctxt.is_null() {
            dict = (*ctxt).dict;
            if (*ctxt).nsNr > 0 as c_int {
                namespaces = xmlMalloc.expect("non-null function pointer")(
                    ((2 as c_int * ((*ctxt).nsNr + 1 as c_int))
                        as size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut xmlChar>() as size_t),
                ) as *mut *const xmlChar;
                if namespaces.is_null() {
                    xmlXPathErrMemory(
                        ctxt,
                        b"allocating namespaces array\n\0" as *const u8
                            as *const c_char,
                    );
                    return ::core::ptr::null_mut::<xmlXPathCompExpr>();
                }
                i = 0 as c_int;
                j = 0 as c_int;
                while j < (*ctxt).nsNr {
                    ns = *(*ctxt).namespaces.offset(j as isize);
                    let fresh76 = i;
                    i = i + 1;
                    let ref mut fresh77 = *namespaces.offset(fresh76 as isize);
                    *fresh77 = (*ns).href;
                    let fresh78 = i;
                    i = i + 1;
                    let ref mut fresh79 = *namespaces.offset(fresh78 as isize);
                    *fresh79 = (*ns).prefix;
                    j += 1;
                }
                let fresh80 = i;
                i = i + 1;
                let ref mut fresh81 = *namespaces.offset(fresh80 as isize);
                *fresh81 = ::core::ptr::null::<xmlChar>();
                let ref mut fresh82 = *namespaces.offset(i as isize);
                *fresh82 = ::core::ptr::null::<xmlChar>();
            }
        }
        stream = xmlPatterncompile(
            str,
            dict as *mut xmlDict,
            XML_PATTERN_XPATH as c_int,
            namespaces,
        );
        if !namespaces.is_null() {
            xmlFree.expect("non-null function pointer")(
                namespaces as *mut *mut xmlChar as *mut c_void,
            );
        }
        if !stream.is_null() && xmlPatternStreamable(stream) == 1 as c_int {
            comp = xmlXPathNewCompExpr();
            if comp.is_null() {
                xmlXPathErrMemory(
                    ctxt,
                    b"allocating streamable expression\n\0" as *const u8
                        as *const c_char,
                );
                xmlFreePattern(stream);
                return ::core::ptr::null_mut::<xmlXPathCompExpr>();
            }
            (*comp).stream = stream;
            (*comp).dict = dict;
            if !(*comp).dict.is_null() {
                xmlDictReference((*comp).dict);
            }
            return comp;
        }
        xmlFreePattern(stream);
    }
    return ::core::ptr::null_mut::<xmlXPathCompExpr>();
}
fn xmlXPathOptimizeExpression(
    mut pctxt: xmlXPathParserContextPtr,
    mut op: xmlXPathStepOpPtr,
) { unsafe {
    let mut comp: xmlXPathCompExprPtr = (*pctxt).comp;
    let mut ctxt: xmlXPathContextPtr = ::core::ptr::null_mut::<xmlXPathContext>();
    if (*op).op as c_uint
        == XPATH_OP_COLLECT as c_int as c_uint
        && (*op).ch1 != -(1 as c_int)
        && (*op).ch2 == -(1 as c_int)
    {
        let mut prevop: xmlXPathStepOpPtr =
            (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr;
        if (*prevop).op as c_uint
            == XPATH_OP_COLLECT as c_int as c_uint
            && (*prevop).value as xmlXPathAxisVal as c_uint
                == AXIS_DESCENDANT_OR_SELF as c_int as c_uint
            && (*prevop).ch2 == -(1 as c_int)
            && (*prevop).value2 as xmlXPathTestVal as c_uint
                == NODE_TEST_TYPE as c_int as c_uint
            && (*prevop).value3 as xmlXPathTypeVal as c_uint
                == NODE_TYPE_NODE as c_int as c_uint
        {
            match (*op).value as xmlXPathAxisVal as c_uint {
                4 | 5 => {
                    (*op).ch1 = (*prevop).ch1;
                    (*op).value = AXIS_DESCENDANT as c_int;
                }
                13 | 6 => {
                    (*op).ch1 = (*prevop).ch1;
                    (*op).value = AXIS_DESCENDANT_OR_SELF as c_int;
                }
                _ => {}
            }
        }
    }
    if (*op).op as c_uint
        == XPATH_OP_VALUE as c_int as c_uint
    {
        return;
    }
    ctxt = (*pctxt).context;
    if !ctxt.is_null() {
        if (*ctxt).depth >= XPATH_MAX_RECURSION_DEPTH {
            return;
        }
        (*ctxt).depth += 1 as c_int;
    }
    if (*op).ch1 != -(1 as c_int) {
        xmlXPathOptimizeExpression(
            pctxt,
            (*comp).steps.offset((*op).ch1 as isize) as xmlXPathStepOpPtr,
        );
    }
    if (*op).ch2 != -(1 as c_int) {
        xmlXPathOptimizeExpression(
            pctxt,
            (*comp).steps.offset((*op).ch2 as isize) as xmlXPathStepOpPtr,
        );
    }
    if !ctxt.is_null() {
        (*ctxt).depth -= 1 as c_int;
    }
} }
#[no_mangle]
pub unsafe extern "C" fn xmlXPathCtxtCompile(
    mut ctxt: xmlXPathContextPtr,
    mut str: *const xmlChar,
) -> xmlXPathCompExprPtr {
    let mut pctxt: xmlXPathParserContextPtr = ::core::ptr::null_mut::<xmlXPathParserContext>();
    let mut comp: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    let mut oldDepth: c_int = 0 as c_int;
    comp = xmlXPathTryStreamCompile(ctxt, str);
    if !comp.is_null() {
        return comp;
    }
    xmlInitParser();
    pctxt = xmlXPathNewParserContext(str, ctxt);
    if pctxt.is_null() {
        return ::core::ptr::null_mut::<xmlXPathCompExpr>();
    }
    if !ctxt.is_null() {
        oldDepth = (*ctxt).depth;
    }
    xmlXPathCompileExpr(pctxt, 1 as c_int);
    if !ctxt.is_null() {
        (*ctxt).depth = oldDepth;
    }
    if (*pctxt).error != XPATH_EXPRESSION_OK as c_int {
        xmlXPathFreeParserContext(pctxt);
        return ::core::ptr::null_mut::<xmlXPathCompExpr>();
    }
    if *(*pctxt).cur as c_int != 0 as c_int {
        xmlXPatherror(
            pctxt,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            13358 as c_int,
            XPATH_EXPR_ERROR as c_int,
        );
        comp = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    } else {
        comp = (*pctxt).comp;
        if (*comp).nbStep > 1 as c_int && (*comp).last >= 0 as c_int {
            if !ctxt.is_null() {
                oldDepth = (*ctxt).depth;
            }
            xmlXPathOptimizeExpression(
                pctxt,
                (*comp).steps.offset((*comp).last as isize) as xmlXPathStepOpPtr,
            );
            if !ctxt.is_null() {
                (*ctxt).depth = oldDepth;
            }
        }
        (*pctxt).comp = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    }
    xmlXPathFreeParserContext(pctxt);
    if !comp.is_null() {
        (*comp).expr = xmlStrdup(str);
    }
    return comp;
}
#[inline]
pub unsafe fn xmlXPathCompile(mut str: *const xmlChar) -> xmlXPathCompExprPtr {
    return xmlXPathCtxtCompile(::core::ptr::null_mut::<xmlXPathContext>(), str);
}
unsafe fn xmlXPathCompiledEvalInternal(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
    mut resObjPtr: *mut xmlXPathObjectPtr,
    mut toBool: c_int,
) -> c_int {
    let mut pctxt: xmlXPathParserContextPtr = ::core::ptr::null_mut::<xmlXPathParserContext>();
    let mut resObj: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut res: c_int = 0;
    if ctxt.is_null() {
        __xmlRaiseError(
            None,
            None,
            NULL,
            NULL,
            NULL,
            XML_FROM_XPATH as c_int,
            XML_ERR_INTERNAL_ERROR as c_int,
            XML_ERR_FATAL,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            13419 as c_int,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            b"NULL context pointer\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if comp.is_null() {
        return -(1 as c_int);
    }
    xmlInitParser();
    pctxt = xmlXPathCompParserContext(comp, ctxt);
    if pctxt.is_null() {
        return -(1 as c_int);
    }
    res = xmlXPathRunEval(pctxt, toBool);
    if (*pctxt).error != XPATH_EXPRESSION_OK as c_int {
        resObj = ::core::ptr::null_mut::<xmlXPathObject>();
    } else {
        resObj = valuePop(pctxt);
        if resObj.is_null() {
            if toBool == 0 {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlXPathCompiledEval: No result on the stack.\n\0" as *const u8
                        as *const c_char,
                );
            }
        } else if (*pctxt).valueNr > 0 as c_int {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompiledEval: %d object(s) left on the stack.\n\0" as *const u8
                    as *const c_char,
                (*pctxt).valueNr,
            );
        }
    }
    if !resObjPtr.is_null() {
        *resObjPtr = resObj;
    } else {
        xmlXPathReleaseObject(ctxt, resObj);
    }
    (*pctxt).comp = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    xmlXPathFreeParserContext(pctxt);
    return res;
}
#[no_mangle]
pub extern "C" fn xmlXPathCompiledEval(
    mut comp: xmlXPathCompExprPtr,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr { unsafe {
    let mut res: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    xmlXPathCompiledEvalInternal(comp, ctx, &raw mut res, 0 as c_int);
    return res;
} }
#[inline]
pub fn xmlXPathCompiledEvalToBoolean(
    mut comp: xmlXPathCompExprPtr,
    mut ctxt: xmlXPathContextPtr,
) -> c_int { unsafe {
    return xmlXPathCompiledEvalInternal(
        comp,
        ctxt,
        ::core::ptr::null_mut::<xmlXPathObjectPtr>(),
        1 as c_int,
    );
} }
#[no_mangle]
pub extern "C" fn xmlXPathEvalExpr(mut ctxt: xmlXPathParserContextPtr) { unsafe {
    let mut comp: xmlXPathCompExprPtr = ::core::ptr::null_mut::<xmlXPathCompExpr>();
    let mut oldDepth: c_int = 0 as c_int;
    if ctxt.is_null() {
        return;
    }
    comp = xmlXPathTryStreamCompile((*ctxt).context, (*ctxt).base);
    if !comp.is_null() {
        if !(*ctxt).comp.is_null() {
            xmlXPathFreeCompExpr((*ctxt).comp);
        }
        (*ctxt).comp = comp;
    } else {
        if !(*ctxt).context.is_null() {
            oldDepth = (*(*ctxt).context).depth;
        }
        xmlXPathCompileExpr(ctxt, 1 as c_int);
        if !(*ctxt).context.is_null() {
            (*(*ctxt).context).depth = oldDepth;
        }
        if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
            return;
        }
        if *(*ctxt).cur as c_int != 0 as c_int {
            xmlXPathErr(ctxt, XPATH_EXPR_ERROR as c_int);
            return;
        }
        if (*(*ctxt).comp).nbStep > 1 as c_int
            && (*(*ctxt).comp).last >= 0 as c_int
        {
            if !(*ctxt).context.is_null() {
                oldDepth = (*(*ctxt).context).depth;
            }
            xmlXPathOptimizeExpression(
                ctxt,
                (*(*ctxt).comp).steps.offset((*(*ctxt).comp).last as isize) as xmlXPathStepOpPtr,
            );
            if !(*ctxt).context.is_null() {
                (*(*ctxt).context).depth = oldDepth;
            }
        }
    }
    xmlXPathRunEval(ctxt, 0 as c_int);
} }
#[no_mangle]
pub unsafe extern "C" fn xmlXPathEval(
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    let mut ctxt: xmlXPathParserContextPtr = ::core::ptr::null_mut::<xmlXPathParserContext>();
    let mut res: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    if ctx.is_null() {
        __xmlRaiseError(
            None,
            None,
            NULL,
            NULL,
            NULL,
            XML_FROM_XPATH as c_int,
            XML_ERR_INTERNAL_ERROR as c_int,
            XML_ERR_FATAL,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xpath.c\0" as *const u8
                as *const c_char,
            13566 as c_int,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            b"NULL context pointer\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    xmlInitParser();
    ctxt = xmlXPathNewParserContext(str, ctx);
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    xmlXPathEvalExpr(ctxt);
    if (*ctxt).error != XPATH_EXPRESSION_OK as c_int {
        res = ::core::ptr::null_mut::<xmlXPathObject>();
    } else {
        res = valuePop(ctxt);
        if res.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompiledEval: No result on the stack.\n\0" as *const u8
                    as *const c_char,
            );
        } else if (*ctxt).valueNr > 0 as c_int {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlXPathCompiledEval: %d object(s) left on the stack.\n\0" as *const u8
                    as *const c_char,
                (*ctxt).valueNr,
            );
        }
    }
    xmlXPathFreeParserContext(ctxt);
    return res;
}
#[inline]
pub fn xmlXPathSetContextNode(
    mut node: xmlNodePtr,
    mut ctx: xmlXPathContextPtr,
) -> c_int { unsafe {
    if node.is_null() || ctx.is_null() {
        return -(1 as c_int);
    }
    if (*node).doc == (*ctx).doc {
        (*ctx).node = node;
        return 0 as c_int;
    }
    return -(1 as c_int);
} }
#[inline]
pub unsafe fn xmlXPathNodeEval(
    mut node: xmlNodePtr,
    mut str: *const xmlChar,
    mut ctx: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    if str.is_null() {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    if xmlXPathSetContextNode(node, ctx) < 0 as c_int {
        return ::core::ptr::null_mut::<xmlXPathObject>();
    }
    return xmlXPathEval(str, ctx);
}
#[inline]
pub unsafe fn xmlXPathEvalExpression(
    mut str: *const xmlChar,
    mut ctxt: xmlXPathContextPtr,
) -> xmlXPathObjectPtr {
    return xmlXPathEval(str, ctxt);
}
extern "C" fn xmlXPathEscapeUriFunction(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: c_int,
) { unsafe {
    let mut str: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    let mut escape_reserved: c_int = 0;
    let mut target: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut cptr: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut escape: [xmlChar; 4] = [0; 4];
    if ctxt.is_null() {
        return;
    }
    if nargs != 2 as c_int {
        xmlXPathErr(ctxt, XPATH_INVALID_ARITY as c_int);
        return;
    }
    if (*ctxt).valueNr < 2 as c_int {
        xmlXPathErr(ctxt, XPATH_STACK_ERROR as c_int);
        return;
    }
    escape_reserved = xmlXPathPopBoolean(ctxt);
    if !(*ctxt).value.is_null()
        && (*(*ctxt).value).type_0 as c_uint
            != XPATH_STRING as c_int as c_uint
    {
        xmlXPathStringFunction(ctxt, 1 as c_int);
    }
    str = valuePop(ctxt);
    target = xmlBufCreate();
    escape[0 as c_int as usize] = '%' as i32 as xmlChar;
    escape[3 as c_int as usize] = 0 as xmlChar;
    if !target.is_null() {
        cptr = (*str).stringval;
        while *cptr != 0 {
            if *cptr as c_int >= 'A' as i32
                && *cptr as c_int <= 'Z' as i32
                || *cptr as c_int >= 'a' as i32
                    && *cptr as c_int <= 'z' as i32
                || *cptr as c_int >= '0' as i32
                    && *cptr as c_int <= '9' as i32
                || *cptr as c_int == '-' as i32
                || *cptr as c_int == '_' as i32
                || *cptr as c_int == '.' as i32
                || *cptr as c_int == '!' as i32
                || *cptr as c_int == '~' as i32
                || *cptr as c_int == '*' as i32
                || *cptr as c_int == '\'' as i32
                || *cptr as c_int == '(' as i32
                || *cptr as c_int == ')' as i32
                || *cptr as c_int == '%' as i32
                    && (*cptr.offset(1 as c_int as isize) as c_int
                        >= 'A' as i32
                        && *cptr.offset(1 as c_int as isize) as c_int
                            <= 'F' as i32
                        || *cptr.offset(1 as c_int as isize) as c_int
                            >= 'a' as i32
                            && *cptr.offset(1 as c_int as isize) as c_int
                                <= 'f' as i32
                        || *cptr.offset(1 as c_int as isize) as c_int
                            >= '0' as i32
                            && *cptr.offset(1 as c_int as isize) as c_int
                                <= '9' as i32)
                    && (*cptr.offset(2 as c_int as isize) as c_int
                        >= 'A' as i32
                        && *cptr.offset(2 as c_int as isize) as c_int
                            <= 'F' as i32
                        || *cptr.offset(2 as c_int as isize) as c_int
                            >= 'a' as i32
                            && *cptr.offset(2 as c_int as isize) as c_int
                                <= 'f' as i32
                        || *cptr.offset(2 as c_int as isize) as c_int
                            >= '0' as i32
                            && *cptr.offset(2 as c_int as isize) as c_int
                                <= '9' as i32)
                || escape_reserved == 0
                    && (*cptr as c_int == ';' as i32
                        || *cptr as c_int == '/' as i32
                        || *cptr as c_int == '?' as i32
                        || *cptr as c_int == ':' as i32
                        || *cptr as c_int == '@' as i32
                        || *cptr as c_int == '&' as i32
                        || *cptr as c_int == '=' as i32
                        || *cptr as c_int == '+' as i32
                        || *cptr as c_int == '$' as i32
                        || *cptr as c_int == ',' as i32)
            {
                xmlBufAdd(target, cptr, 1 as c_int);
            } else {
                if (*cptr as c_int >> 4 as c_int)
                    < 10 as c_int
                {
                    escape[1 as c_int as usize] = ('0' as i32
                        + (*cptr as c_int >> 4 as c_int))
                        as xmlChar;
                } else {
                    escape[1 as c_int as usize] = ('A' as i32
                        - 10 as c_int
                        + (*cptr as c_int >> 4 as c_int))
                        as xmlChar;
                }
                if (*cptr as c_int & 0xf as c_int)
                    < 10 as c_int
                {
                    escape[2 as c_int as usize] = ('0' as i32
                        + (*cptr as c_int & 0xf as c_int))
                        as xmlChar;
                } else {
                    escape[2 as c_int as usize] = ('A' as i32
                        - 10 as c_int
                        + (*cptr as c_int & 0xf as c_int))
                        as xmlChar;
                }
                xmlBufAdd(
                    target,
                    (&raw mut escape as *mut xmlChar).offset(0 as c_int as isize)
                        as *mut xmlChar,
                    3 as c_int,
                );
            }
            cptr = cptr.offset(1);
        }
    }
    valuePush(
        ctxt,
        xmlXPathCacheNewString((*ctxt).context, xmlBufContent(target as *const xmlBuf)),
    );
    xmlBufFree(target);
    xmlXPathReleaseObject((*ctxt).context, str);
} }
#[inline]
pub fn xmlXPathRegisterAllFunctions(mut ctxt: xmlXPathContextPtr) { unsafe {
    xmlXPathRegisterFunc(
        ctxt,
        b"boolean\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathBooleanFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"ceiling\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathCeilingFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"count\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathCountFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"concat\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathConcatFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"contains\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathContainsFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"id\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathIdFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"false\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathFalseFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"floor\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathFloorFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"last\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathLastFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"lang\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathLangFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"local-name\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathLocalNameFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"not\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathNotFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"name\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathNameFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"namespace-uri\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathNamespaceURIFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"normalize-space\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathNormalizeFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"number\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathNumberFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"position\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathPositionFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"round\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathRoundFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"string\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathStringFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"string-length\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathStringLengthFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"starts-with\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathStartsWithFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"substring\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathSubstringFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"substring-before\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathSubstringBeforeFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"substring-after\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathSubstringAfterFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"sum\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathSumFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"true\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathTrueFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFunc(
        ctxt,
        b"translate\0" as *const u8 as *const c_char as *const xmlChar,
        Some(
            xmlXPathTranslateFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
    xmlXPathRegisterFuncNS(
        ctxt,
        b"escape-uri\0" as *const u8 as *const c_char as *const xmlChar,
        b"http://www.w3.org/2002/08/xquery-functions\0" as *const u8 as *const c_char
            as *const xmlChar,
        Some(
            xmlXPathEscapeUriFunction
                as unsafe extern "C" fn(xmlXPathParserContextPtr, c_int) -> (),
        ),
    );
} }
#[inline]
fn compute_minrun(size: uint64_t) -> c_int { {
    let top_bit: c_int =
        64 as c_int - (size as c_ulonglong).leading_zeros() as i32;
    let shift: c_int = (if top_bit > 6 as c_int {
        top_bit
    } else {
        6 as c_int
    }) - 6 as c_int;
    let minrun: c_int = (size >> shift) as c_int;
    let mask: uint64_t = ((1 as c_ulonglong) << shift)
        .wrapping_sub(1 as c_ulonglong) as uint64_t;
    if mask & size != 0 {
        return minrun + 1 as c_int;
    }
    return minrun;
} }
#[inline]
unsafe fn libxml_domnode_binary_insertion_find(
    mut dst: *mut xmlNodePtr,
    x: xmlNodePtr,
    size: size_t,
) -> size_t {
    let mut l: size_t = 0;
    let mut c: size_t = 0;
    let mut r: size_t = 0;
    let mut cx: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    l = 0 as size_t;
    r = size.wrapping_sub(1 as size_t);
    c = r >> 1 as c_int;
    if wrap_cmp(x, *dst.offset(0 as c_int as isize)) < 0 as c_int {
        return 0 as size_t;
    } else if wrap_cmp(x, *dst.offset(r as isize)) > 0 as c_int {
        return r;
    }
    cx = *dst.offset(c as isize);
    loop {
        let val: c_int = wrap_cmp(x, cx) as c_int;
        if val < 0 as c_int {
            if c.wrapping_sub(l) <= 1 as size_t {
                return c;
            }
            r = c;
        } else {
            if r.wrapping_sub(c) <= 1 as size_t {
                return c.wrapping_add(1 as size_t);
            }
            l = c;
        }
        c = l.wrapping_add(r.wrapping_sub(l) >> 1 as c_int);
        cx = *dst.offset(c as isize);
    }
}
unsafe fn libxml_domnode_binary_insertion_sort_start(
    mut dst: *mut xmlNodePtr,
    start: size_t,
    size: size_t,
) {
    let mut i: size_t = 0;
    i = start;
    while i < size {
        let mut j: size_t = 0;
        let mut x: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        let mut location: size_t = 0;
        if !(wrap_cmp(
            *dst.offset(i.wrapping_sub(1 as size_t) as isize),
            *dst.offset(i as isize),
        ) <= 0 as c_int)
        {
            x = *dst.offset(i as isize);
            location = libxml_domnode_binary_insertion_find(dst, x, i);
            j = i.wrapping_sub(1 as size_t);
            while j >= location {
                let ref mut fresh22 = *dst.offset(j.wrapping_add(1 as size_t) as isize);
                *fresh22 = *dst.offset(j as isize);
                if j == 0 as size_t {
                    break;
                }
                j = j.wrapping_sub(1);
            }
            let ref mut fresh23 = *dst.offset(location as isize);
            *fresh23 = x;
        }
        i = i.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn libxml_domnode_binary_insertion_sort(
    mut dst: *mut xmlNodePtr,
    size: size_t,
) {
    if size <= 1 as size_t {
        return;
    }
    libxml_domnode_binary_insertion_sort_start(dst, 1 as size_t, size);
}
#[inline]
unsafe fn libxml_domnode_reverse_elements(
    mut dst: *mut xmlNodePtr,
    mut start: size_t,
    mut end: size_t,
) {
    loop {
        if start >= end {
            return;
        }
        let mut __SORT_SWAP_t: xmlNodePtr = *dst.offset(start as isize);
        let ref mut fresh20 = *dst.offset(start as isize);
        *fresh20 = *dst.offset(end as isize);
        let ref mut fresh21 = *dst.offset(end as isize);
        *fresh21 = __SORT_SWAP_t;
        start = start.wrapping_add(1);
        end = end.wrapping_sub(1);
    }
}
unsafe fn libxml_domnode_count_run(
    mut dst: *mut xmlNodePtr,
    start: size_t,
    size: size_t,
) -> size_t {
    let mut curr: size_t = 0;
    if size.wrapping_sub(start) == 1 as size_t {
        return 1 as size_t;
    }
    if start >= size.wrapping_sub(2 as size_t) {
        if wrap_cmp(
            *dst.offset(size.wrapping_sub(2 as size_t) as isize),
            *dst.offset(size.wrapping_sub(1 as size_t) as isize),
        ) > 0 as c_int
        {
            let mut __SORT_SWAP_t: xmlNodePtr =
                *dst.offset(size.wrapping_sub(2 as size_t) as isize);
            let ref mut fresh18 = *dst.offset(size.wrapping_sub(2 as size_t) as isize);
            *fresh18 = *dst.offset(size.wrapping_sub(1 as size_t) as isize);
            let ref mut fresh19 = *dst.offset(size.wrapping_sub(1 as size_t) as isize);
            *fresh19 = __SORT_SWAP_t;
        }
        return 2 as size_t;
    }
    curr = start.wrapping_add(2 as size_t);
    if wrap_cmp(
        *dst.offset(start as isize),
        *dst.offset(start.wrapping_add(1 as size_t) as isize),
    ) <= 0 as c_int
    {
        while !(curr == size.wrapping_sub(1 as size_t)) {
            if wrap_cmp(
                *dst.offset(curr.wrapping_sub(1 as size_t) as isize),
                *dst.offset(curr as isize),
            ) > 0 as c_int
            {
                break;
            }
            curr = curr.wrapping_add(1);
        }
        return curr.wrapping_sub(start);
    } else {
        while !(curr == size.wrapping_sub(1 as size_t)) {
            if wrap_cmp(
                *dst.offset(curr.wrapping_sub(1 as size_t) as isize),
                *dst.offset(curr as isize),
            ) <= 0 as c_int
            {
                break;
            }
            curr = curr.wrapping_add(1);
        }
        libxml_domnode_reverse_elements(dst, start, curr.wrapping_sub(1 as size_t));
        return curr.wrapping_sub(start);
    };
}
unsafe fn libxml_domnode_check_invariant(
    mut stack: *mut TIM_SORT_RUN_T,
    stack_curr: c_int,
) -> c_int {
    let mut A: size_t = 0;
    let mut B: size_t = 0;
    let mut C: size_t = 0;
    if stack_curr < 2 as c_int {
        return 1 as c_int;
    }
    if stack_curr == 2 as c_int {
        let A1: size_t = (*stack.offset((stack_curr - 2 as c_int) as isize)).length;
        let B1: size_t = (*stack.offset((stack_curr - 1 as c_int) as isize)).length;
        if A1 <= B1 {
            return 0 as c_int;
        }
        return 1 as c_int;
    }
    A = (*stack.offset((stack_curr - 3 as c_int) as isize)).length;
    B = (*stack.offset((stack_curr - 2 as c_int) as isize)).length;
    C = (*stack.offset((stack_curr - 1 as c_int) as isize)).length;
    if A <= B.wrapping_add(C) || B <= C {
        return 0 as c_int;
    }
    return 1 as c_int;
}
unsafe fn libxml_domnode_tim_sort_resize(
    mut store: *mut TEMP_STORAGE_T,
    new_size: size_t,
) {
    if (*store).alloc < new_size {
        let mut tempstore: *mut xmlNodePtr = realloc(
            (*store).storage as *mut c_void,
            new_size.wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if tempstore.is_null() {
            fprintf(
                stderr,
                b"Error allocating temporary storage for tim sort: need %lu bytes\0" as *const u8
                    as *const c_char,
                (::core::mem::size_of::<xmlNodePtr>() as usize).wrapping_mul(new_size as usize)
                    as c_ulong,
            );
            exit(1 as c_int);
        }
        (*store).storage = tempstore;
        (*store).alloc = new_size;
    }
}
unsafe fn libxml_domnode_tim_sort_merge(
    mut dst: *mut xmlNodePtr,
    mut stack: *const TIM_SORT_RUN_T,
    stack_curr: c_int,
    mut store: *mut TEMP_STORAGE_T,
) {
    let A: size_t = (*stack.offset((stack_curr - 2 as c_int) as isize)).length;
    let B: size_t = (*stack.offset((stack_curr - 1 as c_int) as isize)).length;
    let curr: size_t = (*stack.offset((stack_curr - 2 as c_int) as isize)).start;
    let mut storage: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    libxml_domnode_tim_sort_resize(store, if A < B { A } else { B });
    storage = (*store).storage;
    if A < B {
        memcpy(
            storage as *mut c_void,
            dst.offset(curr as isize) as *mut xmlNodePtr as *const c_void,
            A.wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        i = 0 as size_t;
        j = curr.wrapping_add(A);
        k = curr;
        while k < curr.wrapping_add(A).wrapping_add(B) {
            if i < A && j < curr.wrapping_add(A).wrapping_add(B) {
                if wrap_cmp(*storage.offset(i as isize), *dst.offset(j as isize))
                    <= 0 as c_int
                {
                    let fresh9 = i;
                    i = i.wrapping_add(1);
                    let ref mut fresh10 = *dst.offset(k as isize);
                    *fresh10 = *storage.offset(fresh9 as isize);
                } else {
                    let fresh11 = j;
                    j = j.wrapping_add(1);
                    let ref mut fresh12 = *dst.offset(k as isize);
                    *fresh12 = *dst.offset(fresh11 as isize);
                }
            } else {
                if !(i < A) {
                    break;
                }
                let fresh13 = i;
                i = i.wrapping_add(1);
                let ref mut fresh14 = *dst.offset(k as isize);
                *fresh14 = *storage.offset(fresh13 as isize);
            }
            k = k.wrapping_add(1);
        }
    } else {
        memcpy(
            storage as *mut c_void,
            dst.offset(curr.wrapping_add(A) as isize) as *mut xmlNodePtr
                as *const c_void,
            B.wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        i = B;
        j = curr.wrapping_add(A);
        k = curr.wrapping_add(A).wrapping_add(B);
        while k > curr {
            k = k.wrapping_sub(1);
            if i > 0 as size_t && j > curr {
                if wrap_cmp(
                    *dst.offset(j.wrapping_sub(1 as size_t) as isize),
                    *storage.offset(i.wrapping_sub(1 as size_t) as isize),
                ) > 0 as c_int
                {
                    j = j.wrapping_sub(1);
                    let ref mut fresh15 = *dst.offset(k as isize);
                    *fresh15 = *dst.offset(j as isize);
                } else {
                    i = i.wrapping_sub(1);
                    let ref mut fresh16 = *dst.offset(k as isize);
                    *fresh16 = *storage.offset(i as isize);
                }
            } else {
                if !(i > 0 as size_t) {
                    break;
                }
                i = i.wrapping_sub(1);
                let ref mut fresh17 = *dst.offset(k as isize);
                *fresh17 = *storage.offset(i as isize);
            }
        }
    };
}
unsafe fn libxml_domnode_tim_sort_collapse(
    mut dst: *mut xmlNodePtr,
    mut stack: *mut TIM_SORT_RUN_T,
    mut stack_curr: c_int,
    mut store: *mut TEMP_STORAGE_T,
    size: size_t,
) -> c_int {
    loop {
        let mut A: size_t = 0;
        let mut B: size_t = 0;
        let mut C: size_t = 0;
        let mut D: size_t = 0;
        let mut ABC: c_int = 0;
        let mut BCD: c_int = 0;
        let mut CD: c_int = 0;
        if stack_curr <= 1 as c_int {
            break;
        }
        if stack_curr == 2 as c_int
            && (*stack.offset(0 as c_int as isize))
                .length
                .wrapping_add((*stack.offset(1 as c_int as isize)).length)
                == size
        {
            libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store);
            let ref mut fresh24 = (*stack.offset(0 as c_int as isize)).length;
            *fresh24 = (*fresh24 as c_ulong).wrapping_add(
                (*stack.offset(1 as c_int as isize)).length as c_ulong,
            ) as size_t as size_t;
            stack_curr -= 1;
            break;
        } else if stack_curr == 2 as c_int
            && (*stack.offset(0 as c_int as isize)).length
                <= (*stack.offset(1 as c_int as isize)).length
        {
            libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store);
            let ref mut fresh25 = (*stack.offset(0 as c_int as isize)).length;
            *fresh25 = (*fresh25 as c_ulong).wrapping_add(
                (*stack.offset(1 as c_int as isize)).length as c_ulong,
            ) as size_t as size_t;
            stack_curr -= 1;
            break;
        } else {
            if stack_curr == 2 as c_int {
                break;
            }
            B = (*stack.offset((stack_curr - 3 as c_int) as isize)).length;
            C = (*stack.offset((stack_curr - 2 as c_int) as isize)).length;
            D = (*stack.offset((stack_curr - 1 as c_int) as isize)).length;
            if stack_curr >= 4 as c_int {
                A = (*stack.offset((stack_curr - 4 as c_int) as isize)).length;
                ABC = (A <= B.wrapping_add(C)) as c_int;
            } else {
                ABC = 0 as c_int;
            }
            BCD = (B <= C.wrapping_add(D) || ABC != 0) as c_int;
            CD = (C <= D) as c_int;
            if BCD == 0 && CD == 0 {
                break;
            }
            if BCD != 0 && CD == 0 {
                libxml_domnode_tim_sort_merge(
                    dst,
                    stack,
                    stack_curr - 1 as c_int,
                    store,
                );
                let ref mut fresh26 =
                    (*stack.offset((stack_curr - 3 as c_int) as isize)).length;
                *fresh26 = (*fresh26 as c_ulong).wrapping_add(
                    (*stack.offset((stack_curr - 2 as c_int) as isize)).length
                        as c_ulong,
                ) as size_t as size_t;
                *stack.offset((stack_curr - 2 as c_int) as isize) =
                    *stack.offset((stack_curr - 1 as c_int) as isize);
                stack_curr -= 1;
            } else {
                libxml_domnode_tim_sort_merge(dst, stack, stack_curr, store);
                let ref mut fresh27 =
                    (*stack.offset((stack_curr - 2 as c_int) as isize)).length;
                *fresh27 = (*fresh27 as c_ulong).wrapping_add(
                    (*stack.offset((stack_curr - 1 as c_int) as isize)).length
                        as c_ulong,
                ) as size_t as size_t;
                stack_curr -= 1;
            }
        }
    }
    return stack_curr;
}
#[inline]
unsafe fn PUSH_NEXT(
    mut dst: *mut xmlNodePtr,
    size: size_t,
    mut store: *mut TEMP_STORAGE_T,
    minrun: size_t,
    mut run_stack: *mut TIM_SORT_RUN_T,
    mut stack_curr: *mut size_t,
    mut curr: *mut size_t,
) -> c_int {
    let mut len: size_t = libxml_domnode_count_run(dst, *curr, size);
    let mut run: size_t = minrun;
    if run > size.wrapping_sub(*curr) {
        run = size.wrapping_sub(*curr);
    }
    if run > len {
        libxml_domnode_binary_insertion_sort_start(
            dst.offset(*curr as isize) as *mut xmlNodePtr,
            len,
            run,
        );
        len = run;
    }
    (*run_stack.offset(*stack_curr as isize)).start = *curr;
    (*run_stack.offset(*stack_curr as isize)).length = len;
    *stack_curr = (*stack_curr).wrapping_add(1);
    *curr = (*curr as c_ulong).wrapping_add(len as c_ulong) as size_t
        as size_t;
    if *curr == size {
        while *stack_curr > 1 as size_t {
            libxml_domnode_tim_sort_merge(dst, run_stack, *stack_curr as c_int, store);
            let ref mut fresh8 =
                (*run_stack.offset((*stack_curr).wrapping_sub(2 as size_t) as isize)).length;
            *fresh8 = (*fresh8 as c_ulong).wrapping_add(
                (*run_stack.offset((*stack_curr).wrapping_sub(1 as size_t) as isize)).length
                    as c_ulong,
            ) as size_t as size_t;
            *stack_curr = (*stack_curr).wrapping_sub(1);
        }
        if !(*store).storage.is_null() {
            free((*store).storage as *mut c_void);
            (*store).storage = ::core::ptr::null_mut::<xmlNodePtr>();
        }
        return 0 as c_int;
    }
    return 1 as c_int;
}
#[inline]
pub unsafe fn libxml_domnode_tim_sort(mut dst: *mut xmlNodePtr, size: size_t) {
    let mut minrun: size_t = 0;
    let mut _store: TEMP_STORAGE_T = TEMP_STORAGE_T {
        alloc: 0,
        storage: ::core::ptr::null_mut::<xmlNodePtr>(),
    };
    let mut store: *mut TEMP_STORAGE_T = ::core::ptr::null_mut::<TEMP_STORAGE_T>();
    let mut run_stack: [TIM_SORT_RUN_T; 128] = [TIM_SORT_RUN_T {
        start: 0,
        length: 0,
    }; 128];
    let mut stack_curr: size_t = 0 as size_t;
    let mut curr: size_t = 0 as size_t;
    if size <= 1 as size_t {
        return;
    }
    if size < 64 as size_t {
        libxml_domnode_binary_insertion_sort(dst, size);
        return;
    }
    minrun = compute_minrun(size as uint64_t) as size_t;
    store = &raw mut _store;
    (*store).alloc = 0 as size_t;
    (*store).storage = ::core::ptr::null_mut::<xmlNodePtr>();
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        &raw mut run_stack as *mut TIM_SORT_RUN_T,
        &raw mut stack_curr,
        &raw mut curr,
    ) == 0
    {
        return;
    }
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        &raw mut run_stack as *mut TIM_SORT_RUN_T,
        &raw mut stack_curr,
        &raw mut curr,
    ) == 0
    {
        return;
    }
    if PUSH_NEXT(
        dst,
        size,
        store,
        minrun,
        &raw mut run_stack as *mut TIM_SORT_RUN_T,
        &raw mut stack_curr,
        &raw mut curr,
    ) == 0
    {
        return;
    }
    loop {
        if libxml_domnode_check_invariant(
            &raw mut run_stack as *mut TIM_SORT_RUN_T,
            stack_curr as c_int,
        ) == 0
        {
            stack_curr = libxml_domnode_tim_sort_collapse(
                dst,
                &raw mut run_stack as *mut TIM_SORT_RUN_T,
                stack_curr as c_int,
                store,
                size,
            ) as size_t;
        } else if PUSH_NEXT(
            dst,
            size,
            store,
            minrun,
            &raw mut run_stack as *mut TIM_SORT_RUN_T,
            &raw mut stack_curr,
            &raw mut curr,
        ) == 0
        {
            return;
        }
    }
}

pub const INT_MIN: c_int = -__INT_MAX__ - 1 as c_int;
pub const DBL_DIG: c_int = __DBL_DIG__;

pub const __DBL_DIG__: c_int = 15 as c_int;
