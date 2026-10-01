use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
use ::c2rust_bitfields;
use crate::src::globals::__xmlGenericError;
use crate::src::globals::__xmlGenericErrorContext;
use crate::src::uri::xmlFreeURI;
use crate::src::hash::xmlHashAddEntry2;
use crate::src::hash::xmlHashCreate;
use crate::src::hash::xmlHashFree;
use crate::src::hash::xmlHashLookup2;
use crate::src::uri::xmlParseURI;
use crate::src::xmlregexp::xmlRegexpExec;
use crate::src::tree::xmlSplitQName2;
use crate::src::xmlstring::xmlStrEqual;
use crate::src::xmlstring::xmlStrcat;
use crate::src::xmlstring::xmlStrcmp;
use crate::src::xmlstring::xmlStrdup;
use crate::src::xmlstring::xmlStrndup;
use crate::src::xmlstring::xmlUTF8Strlen;
use crate::src::tree::xmlValidateNCName;
use crate::src::tree::xmlValidateNMToken;
use crate::src::tree::xmlValidateName;
use crate::src::tree::xmlValidateQName;
use crate::src::xpath::xmlXPathIsNaN;
pub use crate::src::valid::_xmlValidState;
pub use crate::src::dict::_xmlDict;
pub use crate::src::hash::_xmlHashTable;
pub use crate::src::xmlregexp::_xmlRegexp;
pub use crate::src::xmlregexp::_xmlAutomataState;
pub use crate::src::xmlregexp::_xmlAutomata;
extern "C" {
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr, nameSpace: *const xmlChar) -> xmlNsPtr;
    fn xmlAddID(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        value: *const xmlChar,
        attr: xmlAttrPtr,
    ) -> xmlIDPtr;
    fn xmlAddRef(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        value: *const xmlChar,
        attr: xmlAttrPtr,
    ) -> xmlRefPtr;
    fn xmlValidateNotationUse(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        notationName: *const xmlChar,
    ) -> c_int;
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar) -> xmlEntityPtr;
    static mut xmlXPathNAN: c_double;
    static mut xmlXPathPINF: c_double;
    static mut xmlXPathNINF: c_double;
    fn xmlSchemaFreeType(type_0: xmlSchemaTypePtr);
    fn xmlSchemaFreeWildcard(wildcard: xmlSchemaWildcardPtr);
    fn xmlSchemaNewFacet() -> xmlSchemaFacetPtr;
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

pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
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

pub type xmlNsPtr = *mut xmlNs;
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

pub type xmlValidCtxtPtr = *mut xmlValidCtxt;

pub type xmlSchemaAnnotPtr = *mut xmlSchemaAnnot;
pub type xmlSchemaAnnot = _xmlSchemaAnnot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAnnot {
    pub next: *mut _xmlSchemaAnnot,
    pub content: xmlNodePtr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaVal {
    pub type_0: xmlSchemaValType,
    pub next: *mut _xmlSchemaVal,
    pub value: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub decimal: xmlSchemaValDecimal,
    pub date: xmlSchemaValDate,
    pub dur: xmlSchemaValDuration,
    pub qname: xmlSchemaValQName,
    pub hex: xmlSchemaValHex,
    pub base64: xmlSchemaValBase64,
    pub f: c_float,
    pub d: c_double,
    pub b: c_int,
    pub str_0: *mut xmlChar,
}
pub type xmlSchemaValBase64 = _xmlSchemaValBase64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValBase64 {
    pub str_0: *mut xmlChar,
    pub total: c_uint,
}
pub type xmlSchemaValHex = _xmlSchemaValHex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValHex {
    pub str_0: *mut xmlChar,
    pub total: c_uint,
}
pub type xmlSchemaValQName = _xmlSchemaValQName;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValQName {
    pub name: *mut xmlChar,
    pub uri: *mut xmlChar,
}
pub type xmlSchemaValDuration = _xmlSchemaValDuration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaValDuration {
    pub mon: c_long,
    pub day: c_long,
    pub sec: c_double,
}
pub type xmlSchemaValDate = _xmlSchemaValDate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _xmlSchemaValDate {
    pub year: c_long,
    #[bitfield(name = "mon", ty = "c_uint", bits = "0..=3")]
    #[bitfield(name = "day", ty = "c_uint", bits = "4..=8")]
    #[bitfield(name = "hour", ty = "c_uint", bits = "9..=13")]
    #[bitfield(name = "min", ty = "c_uint", bits = "14..=19")]
    pub mon_day_hour_min: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub sec: c_double,
    #[bitfield(name = "tz_flag", ty = "c_uint", bits = "0..=0")]
    #[bitfield(name = "tzo", ty = "c_int", bits = "1..=12")]
    pub tz_flag_tzo: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 6],
}
pub type xmlSchemaValDecimal = _xmlSchemaValDecimal;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _xmlSchemaValDecimal {
    pub lo: c_ulong,
    pub mi: c_ulong,
    pub hi: c_ulong,
    pub extra: c_uint,
    #[bitfield(name = "sign", ty = "c_uint", bits = "0..=0")]
    #[bitfield(name = "frac", ty = "c_uint", bits = "1..=7")]
    #[bitfield(name = "total", ty = "c_uint", bits = "8..=15")]
    pub sign_frac_total: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type xmlSchemaVal = _xmlSchemaVal;
pub type xmlSchemaValPtr = *mut xmlSchemaVal;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaType {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaType,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_0: *const xmlChar,
    pub refNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub subtypes: xmlSchemaTypePtr,
    pub attributes: xmlSchemaAttributePtr,
    pub node: xmlNodePtr,
    pub minOccurs: c_int,
    pub maxOccurs: c_int,
    pub flags: c_int,
    pub contentType: xmlSchemaContentType,
    pub base: *const xmlChar,
    pub baseNs: *const xmlChar,
    pub baseType: xmlSchemaTypePtr,
    pub facets: xmlSchemaFacetPtr,
    pub redef: *mut _xmlSchemaType,
    pub recurse: c_int,
    pub attributeUses: *mut xmlSchemaAttributeLinkPtr,
    pub attributeWildcard: xmlSchemaWildcardPtr,
    pub builtInType: c_int,
    pub memberTypes: xmlSchemaTypeLinkPtr,
    pub facetSet: xmlSchemaFacetLinkPtr,
    pub refPrefix: *const xmlChar,
    pub contentTypeDef: xmlSchemaTypePtr,
    pub contModel: xmlRegexpPtr,
    pub targetNamespace: *const xmlChar,
    pub attrUses: *mut c_void,
}
pub type xmlSchemaTypePtr = *mut xmlSchemaType;
pub type xmlSchemaType = _xmlSchemaType;
pub type xmlSchemaFacetLinkPtr = *mut xmlSchemaFacetLink;
pub type xmlSchemaFacetLink = _xmlSchemaFacetLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaFacetLink {
    pub next: *mut _xmlSchemaFacetLink,
    pub facet: xmlSchemaFacetPtr,
}
pub type xmlSchemaFacetPtr = *mut xmlSchemaFacet;
pub type xmlSchemaFacet = _xmlSchemaFacet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaFacet {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaFacet,
    pub value: *const xmlChar,
    pub id: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub node: xmlNodePtr,
    pub fixed: c_int,
    pub whitespace: c_int,
    pub val: xmlSchemaValPtr,
    pub regexp: xmlRegexpPtr,
}
pub type xmlSchemaTypeLinkPtr = *mut xmlSchemaTypeLink;
pub type xmlSchemaTypeLink = _xmlSchemaTypeLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaTypeLink {
    pub next: *mut _xmlSchemaTypeLink,
    pub type_0: xmlSchemaTypePtr,
}
pub type xmlSchemaWildcardPtr = *mut xmlSchemaWildcard;
pub type xmlSchemaWildcard = _xmlSchemaWildcard;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaWildcard {
    pub type_0: xmlSchemaTypeType,
    pub id: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub node: xmlNodePtr,
    pub minOccurs: c_int,
    pub maxOccurs: c_int,
    pub processContents: c_int,
    pub any: c_int,
    pub nsSet: xmlSchemaWildcardNsPtr,
    pub negNsSet: xmlSchemaWildcardNsPtr,
    pub flags: c_int,
}

pub type xmlSchemaAttributeLinkPtr = *mut xmlSchemaAttributeLink;
pub type xmlSchemaAttributeLink = _xmlSchemaAttributeLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAttributeLink {
    pub next: *mut _xmlSchemaAttributeLink,
    pub attr: *mut _xmlSchemaAttribute,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAttribute {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaAttribute,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_0: *const xmlChar,
    pub refNs: *const xmlChar,
    pub typeName: *const xmlChar,
    pub typeNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub base: xmlSchemaTypePtr,
    pub occurs: c_int,
    pub defValue: *const xmlChar,
    pub subtypes: xmlSchemaTypePtr,
    pub node: xmlNodePtr,
    pub targetNamespace: *const xmlChar,
    pub flags: c_int,
    pub refPrefix: *const xmlChar,
    pub defVal: xmlSchemaValPtr,
    pub refDecl: xmlSchemaAttributePtr,
}
pub type xmlSchemaAttributePtr = *mut xmlSchemaAttribute;
pub type xmlSchemaAttribute = _xmlSchemaAttribute;

pub type xmlSchemaParticlePtr = *mut xmlSchemaParticle;
pub type xmlSchemaParticle = _xmlSchemaParticle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaParticle {
    pub type_0: xmlSchemaTypeType,
    pub annot: xmlSchemaAnnotPtr,
    pub next: xmlSchemaTreeItemPtr,
    pub children: xmlSchemaTreeItemPtr,
    pub minOccurs: c_int,
    pub maxOccurs: c_int,
    pub node: xmlNodePtr,
}
pub type xmlSchemaTreeItemPtr = *mut xmlSchemaTreeItem;
pub type xmlSchemaTreeItem = _xmlSchemaTreeItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaTreeItem {
    pub type_0: xmlSchemaTypeType,
    pub annot: xmlSchemaAnnotPtr,
    pub next: xmlSchemaTreeItemPtr,
    pub children: xmlSchemaTreeItemPtr,
}
pub type xmlSchemaModelGroupPtr = *mut xmlSchemaModelGroup;
pub type xmlSchemaModelGroup = _xmlSchemaModelGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaModelGroup {
    pub type_0: xmlSchemaTypeType,
    pub annot: xmlSchemaAnnotPtr,
    pub next: xmlSchemaTreeItemPtr,
    pub children: xmlSchemaTreeItemPtr,
    pub node: xmlNodePtr,
}
pub type xmlSchemaValDatePtr = *mut xmlSchemaValDate;
pub type xmlSchemaValDurationPtr = *mut xmlSchemaValDuration;

pub const XML_SCHEMAS_ANY_LAX: c_int = 2 as c_int;

pub const XML_SCHEMAS_NAMESPACE_NAME: *const xmlChar =
    b"http://www.w3.org/2001/XMLSchema\0" as *const u8 as *const c_char
        as *const xmlChar;
static mut xmlSchemaTypesInitialized: c_int = 0 as c_int;
static mut xmlSchemaTypesBank: xmlHashTablePtr =
    ::core::ptr::null::<xmlHashTable>() as *mut xmlHashTable;
static mut xmlSchemaTypeStringDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeAnyTypeDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeAnySimpleTypeDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeDecimalDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeDatetimeDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeDateDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeTimeDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeGYearDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeGYearMonthDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeGDayDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeGMonthDayDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeGMonthDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeDurationDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeFloatDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeBooleanDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeDoubleDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeHexBinaryDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeBase64BinaryDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeAnyURIDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypePositiveIntegerDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNonPositiveIntegerDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNegativeIntegerDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNonNegativeIntegerDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeIntegerDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeLongDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeIntDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeShortDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeByteDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeUnsignedLongDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeUnsignedIntDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeUnsignedShortDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeUnsignedByteDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNormStringDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeTokenDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeLanguageDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNameDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeQNameDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNCNameDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeIdDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeIdrefDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeIdrefsDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeEntityDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeEntitiesDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNotationDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNmtokenDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
static mut xmlSchemaTypeNmtokensDef: xmlSchemaTypePtr =
    ::core::ptr::null::<xmlSchemaType>() as *mut xmlSchemaType;
unsafe fn xmlSchemaTypeErrMemory(
    mut node: xmlNodePtr,
    mut extra: *const c_char,
) {
    __xmlSimpleError(
        XML_FROM_DATATYPE as c_int,
        XML_ERR_NO_MEMORY as c_int,
        node as *mut _xmlNode,
        ::core::ptr::null::<c_char>(),
        extra,
    );
}
fn xmlSchemaNewValue(mut type_0: xmlSchemaValType) -> xmlSchemaValPtr { unsafe {
    let mut value: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    value = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlSchemaVal>() as size_t
    ) as xmlSchemaValPtr;
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    memset(
        value as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlSchemaVal>() as size_t,
    );
    (*value).type_0 = type_0;
    return value;
} }
fn xmlSchemaNewMinLengthFacet(
    mut value: c_int,
) -> xmlSchemaFacetPtr { unsafe {
    let mut ret: xmlSchemaFacetPtr = ::core::ptr::null_mut::<xmlSchemaFacet>();
    ret = xmlSchemaNewFacet();
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaFacet>();
    }
    (*ret).type_0 = XML_SCHEMA_FACET_MINLENGTH;
    (*ret).val = xmlSchemaNewValue(XML_SCHEMAS_NNINTEGER);
    if (*ret).val.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
        return ::core::ptr::null_mut::<xmlSchemaFacet>();
    }
    (*(*ret).val).value.decimal.lo = value as c_ulong;
    return ret;
} }
unsafe fn xmlSchemaInitBasicType(
    mut name: *const c_char,
    mut type_0: xmlSchemaValType,
    mut baseType: xmlSchemaTypePtr,
) -> xmlSchemaTypePtr {
    let mut ret: xmlSchemaTypePtr = ::core::ptr::null_mut::<xmlSchemaType>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlSchemaType>() as size_t
    ) as xmlSchemaTypePtr;
    if ret.is_null() {
        xmlSchemaTypeErrMemory(
            ::core::ptr::null_mut::<xmlNode>(),
            b"could not initialize basic types\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlSchemaType>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlSchemaType>() as size_t,
    );
    (*ret).name = name as *const xmlChar;
    (*ret).targetNamespace = XML_SCHEMAS_NAMESPACE_NAME;
    (*ret).type_0 = XML_SCHEMA_TYPE_BASIC;
    (*ret).baseType = baseType;
    (*ret).contentType = XML_SCHEMA_CONTENT_BASIC;
    match type_0 as c_uint {
        1 | 3 | 10 | 11 | 4 | 8 | 9 | 6 | 7 | 5 | 12 | 13 | 14 | 15 | 29 | 43 | 44 | 21 | 28 => {
            (*ret).flags |= XML_SCHEMAS_TYPE_BUILTIN_PRIMITIVE;
        }
        _ => {}
    }
    match type_0 as c_uint {
        45 | 46 => {}
        25 | 19 | 27 => {
            (*ret).flags |= XML_SCHEMAS_TYPE_VARIETY_LIST;
            (*ret).facets = xmlSchemaNewMinLengthFacet(1 as c_int);
            (*ret).flags |= XML_SCHEMAS_TYPE_HAS_FACETS;
        }
        _ => {
            (*ret).flags |= XML_SCHEMAS_TYPE_VARIETY_ATOMIC;
        }
    }
    xmlHashAddEntry2(
        xmlSchemaTypesBank,
        (*ret).name,
        XML_SCHEMAS_NAMESPACE_NAME,
        ret as *mut c_void,
    );
    (*ret).builtInType = type_0 as c_int;
    return ret;
}

extern "C" fn xmlSchemaAddParticle() -> xmlSchemaParticlePtr { unsafe {
    let mut ret: xmlSchemaParticlePtr = ::core::ptr::null_mut::<xmlSchemaParticle>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlSchemaParticle>() as size_t
    ) as xmlSchemaParticlePtr;
    if ret.is_null() {
        xmlSchemaTypeErrMemory(
            ::core::ptr::null_mut::<xmlNode>(),
            b"allocating particle component\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlSchemaParticle>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlSchemaParticle>() as size_t,
    );
    (*ret).type_0 = XML_SCHEMA_TYPE_PARTICLE;
    (*ret).minOccurs = 1 as c_int;
    (*ret).maxOccurs = 1 as c_int;
    return ret;
} }
unsafe extern "C" fn xmlSchemaFreeTypeEntry(
    mut type_0: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlSchemaFreeType(type_0 as xmlSchemaTypePtr);
}
fn xmlSchemaCleanupTypesInternal() { unsafe {
    let mut particle: xmlSchemaParticlePtr = ::core::ptr::null_mut::<xmlSchemaParticle>();
    if !xmlSchemaTypeAnyTypeDef.is_null() {
        xmlSchemaFreeWildcard((*xmlSchemaTypeAnyTypeDef).attributeWildcard);
        particle = (*xmlSchemaTypeAnyTypeDef).subtypes as xmlSchemaParticlePtr;
        xmlSchemaFreeWildcard((*(*(*particle).children).children).children as xmlSchemaWildcardPtr);
        xmlFree.expect("non-null function pointer")(
            (*(*particle).children).children as xmlSchemaParticlePtr as *mut c_void,
        );
        xmlFree.expect("non-null function pointer")(
            (*particle).children as xmlSchemaModelGroupPtr as *mut c_void,
        );
        xmlFree.expect("non-null function pointer")(particle as *mut c_void);
        (*xmlSchemaTypeAnyTypeDef).subtypes = ::core::ptr::null_mut::<xmlSchemaType>();
        xmlSchemaTypeAnyTypeDef = ::core::ptr::null_mut::<xmlSchemaType>();
    }
    xmlHashFree(
        xmlSchemaTypesBank,
        Some(
            xmlSchemaFreeTypeEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
    xmlSchemaTypesBank = ::core::ptr::null_mut::<xmlHashTable>();
} }
#[inline]
pub fn xmlSchemaInitTypes() -> c_int { unsafe {
    if xmlSchemaTypesInitialized != 0 as c_int {
        return 0 as c_int;
    }
    xmlSchemaTypesBank = xmlHashCreate(40 as c_int);
    if xmlSchemaTypesBank.is_null() {
        xmlSchemaTypeErrMemory(
            ::core::ptr::null_mut::<xmlNode>(),
            ::core::ptr::null::<c_char>(),
        );
    } else {
        xmlSchemaTypeAnyTypeDef = xmlSchemaInitBasicType(
            b"anyType\0" as *const u8 as *const c_char,
            XML_SCHEMAS_ANYTYPE,
            ::core::ptr::null_mut::<xmlSchemaType>(),
        );
        if !xmlSchemaTypeAnyTypeDef.is_null() {
            (*xmlSchemaTypeAnyTypeDef).baseType = xmlSchemaTypeAnyTypeDef;
            (*xmlSchemaTypeAnyTypeDef).contentType = XML_SCHEMA_CONTENT_MIXED;
            (*xmlSchemaTypeAnyTypeDef).contentType = XML_SCHEMA_CONTENT_MIXED;
            let mut particle: xmlSchemaParticlePtr = ::core::ptr::null_mut::<xmlSchemaParticle>();
            let mut sequence: xmlSchemaModelGroupPtr =
                ::core::ptr::null_mut::<xmlSchemaModelGroup>();
            let mut wild: xmlSchemaWildcardPtr = ::core::ptr::null_mut::<xmlSchemaWildcard>();
            particle = xmlSchemaAddParticle();
            if !particle.is_null() {
                (*xmlSchemaTypeAnyTypeDef).subtypes = particle as xmlSchemaTypePtr;
                sequence = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<
                    xmlSchemaModelGroup,
                >()
                    as size_t) as xmlSchemaModelGroupPtr;
                if sequence.is_null() {
                    xmlSchemaTypeErrMemory(
                        ::core::ptr::null_mut::<xmlNode>(),
                        b"allocating model group component\0" as *const u8
                            as *const c_char,
                    );
                } else {
                    memset(
                        sequence as *mut c_void,
                        0 as c_int,
                        ::core::mem::size_of::<xmlSchemaModelGroup>() as size_t,
                    );
                    (*sequence).type_0 = XML_SCHEMA_TYPE_SEQUENCE;
                    (*particle).children = sequence as xmlSchemaTreeItemPtr;
                    particle = xmlSchemaAddParticle();
                    if !particle.is_null() {
                        (*particle).minOccurs = 0 as c_int;
                        (*particle).maxOccurs = UNBOUNDED;
                        (*sequence).children = particle as xmlSchemaTreeItemPtr;
                        wild =
                            xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<
                                xmlSchemaWildcard,
                            >(
                            )
                                as size_t) as xmlSchemaWildcardPtr;
                        if wild.is_null() {
                            xmlSchemaTypeErrMemory(
                                ::core::ptr::null_mut::<xmlNode>(),
                                b"allocating wildcard component\0" as *const u8
                                    as *const c_char,
                            );
                        } else {
                            memset(
                                wild as *mut c_void,
                                0 as c_int,
                                ::core::mem::size_of::<xmlSchemaWildcard>() as size_t,
                            );
                            (*wild).type_0 = XML_SCHEMA_TYPE_ANY;
                            (*wild).any = 1 as c_int;
                            (*wild).processContents = XML_SCHEMAS_ANY_LAX;
                            (*particle).children = wild as xmlSchemaTreeItemPtr;
                            wild = xmlMalloc.expect("non-null function pointer")(
                                ::core::mem::size_of::<xmlSchemaWildcard>() as size_t,
                            ) as xmlSchemaWildcardPtr;
                            if wild.is_null() {
                                xmlSchemaTypeErrMemory(
                                    ::core::ptr::null_mut::<xmlNode>(),
                                    b"could not create an attribute wildcard on anyType\0"
                                        as *const u8
                                        as *const c_char,
                                );
                            } else {
                                memset(
                                    wild as *mut c_void,
                                    0 as c_int,
                                    ::core::mem::size_of::<xmlSchemaWildcard>() as size_t,
                                );
                                (*wild).any = 1 as c_int;
                                (*wild).processContents = XML_SCHEMAS_ANY_LAX;
                                (*xmlSchemaTypeAnyTypeDef).attributeWildcard = wild;
                                xmlSchemaTypeAnySimpleTypeDef = xmlSchemaInitBasicType(
                                    b"anySimpleType\0" as *const u8 as *const c_char,
                                    XML_SCHEMAS_ANYSIMPLETYPE,
                                    xmlSchemaTypeAnyTypeDef,
                                );
                                if !xmlSchemaTypeAnySimpleTypeDef.is_null() {
                                    xmlSchemaTypeStringDef = xmlSchemaInitBasicType(
                                        b"string\0" as *const u8 as *const c_char,
                                        XML_SCHEMAS_STRING,
                                        xmlSchemaTypeAnySimpleTypeDef,
                                    );
                                    if !xmlSchemaTypeStringDef.is_null() {
                                        xmlSchemaTypeDecimalDef = xmlSchemaInitBasicType(
                                            b"decimal\0" as *const u8 as *const c_char,
                                            XML_SCHEMAS_DECIMAL,
                                            xmlSchemaTypeAnySimpleTypeDef,
                                        );
                                        if !xmlSchemaTypeDecimalDef.is_null() {
                                            xmlSchemaTypeDateDef = xmlSchemaInitBasicType(
                                                b"date\0" as *const u8
                                                    as *const c_char,
                                                XML_SCHEMAS_DATE,
                                                xmlSchemaTypeAnySimpleTypeDef,
                                            );
                                            if !xmlSchemaTypeDateDef.is_null() {
                                                xmlSchemaTypeDatetimeDef = xmlSchemaInitBasicType(
                                                    b"dateTime\0" as *const u8
                                                        as *const c_char,
                                                    XML_SCHEMAS_DATETIME,
                                                    xmlSchemaTypeAnySimpleTypeDef,
                                                );
                                                if !xmlSchemaTypeDatetimeDef.is_null() {
                                                    xmlSchemaTypeTimeDef = xmlSchemaInitBasicType(
                                                        b"time\0" as *const u8
                                                            as *const c_char,
                                                        XML_SCHEMAS_TIME,
                                                        xmlSchemaTypeAnySimpleTypeDef,
                                                    );
                                                    if !xmlSchemaTypeTimeDef.is_null() {
                                                        xmlSchemaTypeGYearDef =
                                                            xmlSchemaInitBasicType(
                                                                b"gYear\0" as *const u8
                                                                    as *const c_char,
                                                                XML_SCHEMAS_GYEAR,
                                                                xmlSchemaTypeAnySimpleTypeDef,
                                                            );
                                                        if !xmlSchemaTypeGYearDef.is_null() {
                                                            xmlSchemaTypeGYearMonthDef = xmlSchemaInitBasicType(
                                                                b"gYearMonth\0" as *const u8 as *const c_char,
                                                                XML_SCHEMAS_GYEARMONTH,
                                                                xmlSchemaTypeAnySimpleTypeDef,
                                                            );
                                                            if !xmlSchemaTypeGYearMonthDef.is_null()
                                                            {
                                                                xmlSchemaTypeGMonthDef = xmlSchemaInitBasicType(
                                                                    b"gMonth\0" as *const u8 as *const c_char,
                                                                    XML_SCHEMAS_GMONTH,
                                                                    xmlSchemaTypeAnySimpleTypeDef,
                                                                );
                                                                if !xmlSchemaTypeGMonthDef.is_null()
                                                                {
                                                                    xmlSchemaTypeGMonthDayDef = xmlSchemaInitBasicType(
                                                                        b"gMonthDay\0" as *const u8 as *const c_char,
                                                                        XML_SCHEMAS_GMONTHDAY,
                                                                        xmlSchemaTypeAnySimpleTypeDef,
                                                                    );
                                                                    if !xmlSchemaTypeGMonthDayDef
                                                                        .is_null()
                                                                    {
                                                                        xmlSchemaTypeGDayDef = xmlSchemaInitBasicType(
                                                                            b"gDay\0" as *const u8 as *const c_char,
                                                                            XML_SCHEMAS_GDAY,
                                                                            xmlSchemaTypeAnySimpleTypeDef,
                                                                        );
                                                                        if !xmlSchemaTypeGDayDef
                                                                            .is_null()
                                                                        {
                                                                            xmlSchemaTypeDurationDef = xmlSchemaInitBasicType(
                                                                                b"duration\0" as *const u8 as *const c_char,
                                                                                XML_SCHEMAS_DURATION,
                                                                                xmlSchemaTypeAnySimpleTypeDef,
                                                                            );
                                                                            if !xmlSchemaTypeDurationDef.is_null() {
                                                                                xmlSchemaTypeFloatDef = xmlSchemaInitBasicType(
                                                                                    b"float\0" as *const u8 as *const c_char,
                                                                                    XML_SCHEMAS_FLOAT,
                                                                                    xmlSchemaTypeAnySimpleTypeDef,
                                                                                );
                                                                                if !xmlSchemaTypeFloatDef.is_null() {
                                                                                    xmlSchemaTypeDoubleDef = xmlSchemaInitBasicType(
                                                                                        b"double\0" as *const u8 as *const c_char,
                                                                                        XML_SCHEMAS_DOUBLE,
                                                                                        xmlSchemaTypeAnySimpleTypeDef,
                                                                                    );
                                                                                    if !xmlSchemaTypeDoubleDef.is_null() {
                                                                                        xmlSchemaTypeBooleanDef = xmlSchemaInitBasicType(
                                                                                            b"boolean\0" as *const u8 as *const c_char,
                                                                                            XML_SCHEMAS_BOOLEAN,
                                                                                            xmlSchemaTypeAnySimpleTypeDef,
                                                                                        );
                                                                                        if !xmlSchemaTypeBooleanDef.is_null() {
                                                                                            xmlSchemaTypeAnyURIDef = xmlSchemaInitBasicType(
                                                                                                b"anyURI\0" as *const u8 as *const c_char,
                                                                                                XML_SCHEMAS_ANYURI,
                                                                                                xmlSchemaTypeAnySimpleTypeDef,
                                                                                            );
                                                                                            if !xmlSchemaTypeAnyURIDef.is_null() {
                                                                                                xmlSchemaTypeHexBinaryDef = xmlSchemaInitBasicType(
                                                                                                    b"hexBinary\0" as *const u8 as *const c_char,
                                                                                                    XML_SCHEMAS_HEXBINARY,
                                                                                                    xmlSchemaTypeAnySimpleTypeDef,
                                                                                                );
                                                                                                if !xmlSchemaTypeHexBinaryDef.is_null() {
                                                                                                    xmlSchemaTypeBase64BinaryDef = xmlSchemaInitBasicType(
                                                                                                        b"base64Binary\0" as *const u8
                                                                                                            as *const c_char,
                                                                                                        XML_SCHEMAS_BASE64BINARY,
                                                                                                        xmlSchemaTypeAnySimpleTypeDef,
                                                                                                    );
                                                                                                    if !xmlSchemaTypeBase64BinaryDef.is_null() {
                                                                                                        xmlSchemaTypeNotationDef = xmlSchemaInitBasicType(
                                                                                                            b"NOTATION\0" as *const u8 as *const c_char,
                                                                                                            XML_SCHEMAS_NOTATION,
                                                                                                            xmlSchemaTypeAnySimpleTypeDef,
                                                                                                        );
                                                                                                        if !xmlSchemaTypeNotationDef.is_null() {
                                                                                                            xmlSchemaTypeQNameDef = xmlSchemaInitBasicType(
                                                                                                                b"QName\0" as *const u8 as *const c_char,
                                                                                                                XML_SCHEMAS_QNAME,
                                                                                                                xmlSchemaTypeAnySimpleTypeDef,
                                                                                                            );
                                                                                                            if !xmlSchemaTypeQNameDef.is_null() {
                                                                                                                xmlSchemaTypeIntegerDef = xmlSchemaInitBasicType(
                                                                                                                    b"integer\0" as *const u8 as *const c_char,
                                                                                                                    XML_SCHEMAS_INTEGER,
                                                                                                                    xmlSchemaTypeDecimalDef,
                                                                                                                );
                                                                                                                if !xmlSchemaTypeIntegerDef.is_null() {
                                                                                                                    xmlSchemaTypeNonPositiveIntegerDef = xmlSchemaInitBasicType(
                                                                                                                        b"nonPositiveInteger\0" as *const u8
                                                                                                                            as *const c_char,
                                                                                                                        XML_SCHEMAS_NPINTEGER,
                                                                                                                        xmlSchemaTypeIntegerDef,
                                                                                                                    );
                                                                                                                    if !xmlSchemaTypeNonPositiveIntegerDef.is_null() {
                                                                                                                        xmlSchemaTypeNegativeIntegerDef = xmlSchemaInitBasicType(
                                                                                                                            b"negativeInteger\0" as *const u8
                                                                                                                                as *const c_char,
                                                                                                                            XML_SCHEMAS_NINTEGER,
                                                                                                                            xmlSchemaTypeNonPositiveIntegerDef,
                                                                                                                        );
                                                                                                                        if !xmlSchemaTypeNegativeIntegerDef.is_null() {
                                                                                                                            xmlSchemaTypeLongDef = xmlSchemaInitBasicType(
                                                                                                                                b"long\0" as *const u8 as *const c_char,
                                                                                                                                XML_SCHEMAS_LONG,
                                                                                                                                xmlSchemaTypeIntegerDef,
                                                                                                                            );
                                                                                                                            if !xmlSchemaTypeLongDef.is_null() {
                                                                                                                                xmlSchemaTypeIntDef = xmlSchemaInitBasicType(
                                                                                                                                    b"int\0" as *const u8 as *const c_char,
                                                                                                                                    XML_SCHEMAS_INT,
                                                                                                                                    xmlSchemaTypeLongDef,
                                                                                                                                );
                                                                                                                                if !xmlSchemaTypeIntDef.is_null() {
                                                                                                                                    xmlSchemaTypeShortDef = xmlSchemaInitBasicType(
                                                                                                                                        b"short\0" as *const u8 as *const c_char,
                                                                                                                                        XML_SCHEMAS_SHORT,
                                                                                                                                        xmlSchemaTypeIntDef,
                                                                                                                                    );
                                                                                                                                    if !xmlSchemaTypeShortDef.is_null() {
                                                                                                                                        xmlSchemaTypeByteDef = xmlSchemaInitBasicType(
                                                                                                                                            b"byte\0" as *const u8 as *const c_char,
                                                                                                                                            XML_SCHEMAS_BYTE,
                                                                                                                                            xmlSchemaTypeShortDef,
                                                                                                                                        );
                                                                                                                                        if !xmlSchemaTypeByteDef.is_null() {
                                                                                                                                            xmlSchemaTypeNonNegativeIntegerDef = xmlSchemaInitBasicType(
                                                                                                                                                b"nonNegativeInteger\0" as *const u8
                                                                                                                                                    as *const c_char,
                                                                                                                                                XML_SCHEMAS_NNINTEGER,
                                                                                                                                                xmlSchemaTypeIntegerDef,
                                                                                                                                            );
                                                                                                                                            if !xmlSchemaTypeNonNegativeIntegerDef.is_null() {
                                                                                                                                                xmlSchemaTypeUnsignedLongDef = xmlSchemaInitBasicType(
                                                                                                                                                    b"unsignedLong\0" as *const u8
                                                                                                                                                        as *const c_char,
                                                                                                                                                    XML_SCHEMAS_ULONG,
                                                                                                                                                    xmlSchemaTypeNonNegativeIntegerDef,
                                                                                                                                                );
                                                                                                                                                if !xmlSchemaTypeUnsignedLongDef.is_null() {
                                                                                                                                                    xmlSchemaTypeUnsignedIntDef = xmlSchemaInitBasicType(
                                                                                                                                                        b"unsignedInt\0" as *const u8 as *const c_char,
                                                                                                                                                        XML_SCHEMAS_UINT,
                                                                                                                                                        xmlSchemaTypeUnsignedLongDef,
                                                                                                                                                    );
                                                                                                                                                    if !xmlSchemaTypeUnsignedIntDef.is_null() {
                                                                                                                                                        xmlSchemaTypeUnsignedShortDef = xmlSchemaInitBasicType(
                                                                                                                                                            b"unsignedShort\0" as *const u8
                                                                                                                                                                as *const c_char,
                                                                                                                                                            XML_SCHEMAS_USHORT,
                                                                                                                                                            xmlSchemaTypeUnsignedIntDef,
                                                                                                                                                        );
                                                                                                                                                        if !xmlSchemaTypeUnsignedShortDef.is_null() {
                                                                                                                                                            xmlSchemaTypeUnsignedByteDef = xmlSchemaInitBasicType(
                                                                                                                                                                b"unsignedByte\0" as *const u8
                                                                                                                                                                    as *const c_char,
                                                                                                                                                                XML_SCHEMAS_UBYTE,
                                                                                                                                                                xmlSchemaTypeUnsignedShortDef,
                                                                                                                                                            );
                                                                                                                                                            if !xmlSchemaTypeUnsignedByteDef.is_null() {
                                                                                                                                                                xmlSchemaTypePositiveIntegerDef = xmlSchemaInitBasicType(
                                                                                                                                                                    b"positiveInteger\0" as *const u8
                                                                                                                                                                        as *const c_char,
                                                                                                                                                                    XML_SCHEMAS_PINTEGER,
                                                                                                                                                                    xmlSchemaTypeNonNegativeIntegerDef,
                                                                                                                                                                );
                                                                                                                                                                if !xmlSchemaTypePositiveIntegerDef.is_null() {
                                                                                                                                                                    xmlSchemaTypeNormStringDef = xmlSchemaInitBasicType(
                                                                                                                                                                        b"normalizedString\0" as *const u8
                                                                                                                                                                            as *const c_char,
                                                                                                                                                                        XML_SCHEMAS_NORMSTRING,
                                                                                                                                                                        xmlSchemaTypeStringDef,
                                                                                                                                                                    );
                                                                                                                                                                    if !xmlSchemaTypeNormStringDef.is_null() {
                                                                                                                                                                        xmlSchemaTypeTokenDef = xmlSchemaInitBasicType(
                                                                                                                                                                            b"token\0" as *const u8 as *const c_char,
                                                                                                                                                                            XML_SCHEMAS_TOKEN,
                                                                                                                                                                            xmlSchemaTypeNormStringDef,
                                                                                                                                                                        );
                                                                                                                                                                        if !xmlSchemaTypeTokenDef.is_null() {
                                                                                                                                                                            xmlSchemaTypeLanguageDef = xmlSchemaInitBasicType(
                                                                                                                                                                                b"language\0" as *const u8 as *const c_char,
                                                                                                                                                                                XML_SCHEMAS_LANGUAGE,
                                                                                                                                                                                xmlSchemaTypeTokenDef,
                                                                                                                                                                            );
                                                                                                                                                                            if !xmlSchemaTypeLanguageDef.is_null() {
                                                                                                                                                                                xmlSchemaTypeNameDef = xmlSchemaInitBasicType(
                                                                                                                                                                                    b"Name\0" as *const u8 as *const c_char,
                                                                                                                                                                                    XML_SCHEMAS_NAME,
                                                                                                                                                                                    xmlSchemaTypeTokenDef,
                                                                                                                                                                                );
                                                                                                                                                                                if !xmlSchemaTypeNameDef.is_null() {
                                                                                                                                                                                    xmlSchemaTypeNmtokenDef = xmlSchemaInitBasicType(
                                                                                                                                                                                        b"NMTOKEN\0" as *const u8 as *const c_char,
                                                                                                                                                                                        XML_SCHEMAS_NMTOKEN,
                                                                                                                                                                                        xmlSchemaTypeTokenDef,
                                                                                                                                                                                    );
                                                                                                                                                                                    if !xmlSchemaTypeNmtokenDef.is_null() {
                                                                                                                                                                                        xmlSchemaTypeNCNameDef = xmlSchemaInitBasicType(
                                                                                                                                                                                            b"NCName\0" as *const u8 as *const c_char,
                                                                                                                                                                                            XML_SCHEMAS_NCNAME,
                                                                                                                                                                                            xmlSchemaTypeNameDef,
                                                                                                                                                                                        );
                                                                                                                                                                                        if !xmlSchemaTypeNCNameDef.is_null() {
                                                                                                                                                                                            xmlSchemaTypeIdDef = xmlSchemaInitBasicType(
                                                                                                                                                                                                b"ID\0" as *const u8 as *const c_char,
                                                                                                                                                                                                XML_SCHEMAS_ID,
                                                                                                                                                                                                xmlSchemaTypeNCNameDef,
                                                                                                                                                                                            );
                                                                                                                                                                                            if !xmlSchemaTypeIdDef.is_null() {
                                                                                                                                                                                                xmlSchemaTypeIdrefDef = xmlSchemaInitBasicType(
                                                                                                                                                                                                    b"IDREF\0" as *const u8 as *const c_char,
                                                                                                                                                                                                    XML_SCHEMAS_IDREF,
                                                                                                                                                                                                    xmlSchemaTypeNCNameDef,
                                                                                                                                                                                                );
                                                                                                                                                                                                if !xmlSchemaTypeIdrefDef.is_null() {
                                                                                                                                                                                                    xmlSchemaTypeEntityDef = xmlSchemaInitBasicType(
                                                                                                                                                                                                        b"ENTITY\0" as *const u8 as *const c_char,
                                                                                                                                                                                                        XML_SCHEMAS_ENTITY,
                                                                                                                                                                                                        xmlSchemaTypeNCNameDef,
                                                                                                                                                                                                    );
                                                                                                                                                                                                    if !xmlSchemaTypeEntityDef.is_null() {
                                                                                                                                                                                                        xmlSchemaTypeEntitiesDef = xmlSchemaInitBasicType(
                                                                                                                                                                                                            b"ENTITIES\0" as *const u8 as *const c_char,
                                                                                                                                                                                                            XML_SCHEMAS_ENTITIES,
                                                                                                                                                                                                            xmlSchemaTypeAnySimpleTypeDef,
                                                                                                                                                                                                        );
                                                                                                                                                                                                        if !xmlSchemaTypeEntitiesDef.is_null() {
                                                                                                                                                                                                            (*xmlSchemaTypeEntitiesDef).subtypes = xmlSchemaTypeEntityDef;
                                                                                                                                                                                                            xmlSchemaTypeIdrefsDef = xmlSchemaInitBasicType(
                                                                                                                                                                                                                b"IDREFS\0" as *const u8 as *const c_char,
                                                                                                                                                                                                                XML_SCHEMAS_IDREFS,
                                                                                                                                                                                                                xmlSchemaTypeAnySimpleTypeDef,
                                                                                                                                                                                                            );
                                                                                                                                                                                                            if !xmlSchemaTypeIdrefsDef.is_null() {
                                                                                                                                                                                                                (*xmlSchemaTypeIdrefsDef).subtypes = xmlSchemaTypeIdrefDef;
                                                                                                                                                                                                                xmlSchemaTypeNmtokensDef = xmlSchemaInitBasicType(
                                                                                                                                                                                                                    b"NMTOKENS\0" as *const u8 as *const c_char,
                                                                                                                                                                                                                    XML_SCHEMAS_NMTOKENS,
                                                                                                                                                                                                                    xmlSchemaTypeAnySimpleTypeDef,
                                                                                                                                                                                                                );
                                                                                                                                                                                                                if !xmlSchemaTypeNmtokensDef.is_null() {
                                                                                                                                                                                                                    (*xmlSchemaTypeNmtokensDef).subtypes = xmlSchemaTypeNmtokenDef;
                                                                                                                                                                                                                    xmlSchemaTypesInitialized = 1 as c_int;
                                                                                                                                                                                                                    return 0 as c_int;
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
        }
    }
    xmlSchemaCleanupTypesInternal();
    return -(1 as c_int);
} }
#[inline]
pub fn xmlSchemaCleanupTypes() { unsafe {
    if xmlSchemaTypesInitialized != 0 as c_int {
        xmlSchemaCleanupTypesInternal();
        xmlSchemaTypesInitialized = 0 as c_int;
    }
} }
#[no_mangle]
pub extern "C" fn xmlSchemaIsBuiltInTypeFacet(
    mut type_0: xmlSchemaTypePtr,
    mut facetType: c_int,
) -> c_int { unsafe {
    if type_0.is_null() {
        return -(1 as c_int);
    }
    if (*type_0).type_0 as c_uint
        != XML_SCHEMA_TYPE_BASIC as c_int as c_uint
    {
        return -(1 as c_int);
    }
    match (*type_0).builtInType {
        15 => {
            if facetType == XML_SCHEMA_FACET_PATTERN as c_int
                || facetType == XML_SCHEMA_FACET_WHITESPACE as c_int
            {
                return 1 as c_int;
            } else {
                return 0 as c_int;
            }
        }
        1 | 28 | 21 | 29 | 44 | 43 => {
            if facetType == XML_SCHEMA_FACET_LENGTH as c_int
                || facetType == XML_SCHEMA_FACET_MINLENGTH as c_int
                || facetType == XML_SCHEMA_FACET_MAXLENGTH as c_int
                || facetType == XML_SCHEMA_FACET_PATTERN as c_int
                || facetType == XML_SCHEMA_FACET_ENUMERATION as c_int
                || facetType == XML_SCHEMA_FACET_WHITESPACE as c_int
            {
                return 1 as c_int;
            } else {
                return 0 as c_int;
            }
        }
        3 => {
            if facetType == XML_SCHEMA_FACET_TOTALDIGITS as c_int
                || facetType == XML_SCHEMA_FACET_FRACTIONDIGITS as c_int
                || facetType == XML_SCHEMA_FACET_PATTERN as c_int
                || facetType == XML_SCHEMA_FACET_WHITESPACE as c_int
                || facetType == XML_SCHEMA_FACET_ENUMERATION as c_int
                || facetType == XML_SCHEMA_FACET_MAXINCLUSIVE as c_int
                || facetType == XML_SCHEMA_FACET_MAXEXCLUSIVE as c_int
                || facetType == XML_SCHEMA_FACET_MININCLUSIVE as c_int
                || facetType == XML_SCHEMA_FACET_MINEXCLUSIVE as c_int
            {
                return 1 as c_int;
            } else {
                return 0 as c_int;
            }
        }
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 => {
            if facetType == XML_SCHEMA_FACET_PATTERN as c_int
                || facetType == XML_SCHEMA_FACET_ENUMERATION as c_int
                || facetType == XML_SCHEMA_FACET_WHITESPACE as c_int
                || facetType == XML_SCHEMA_FACET_MAXINCLUSIVE as c_int
                || facetType == XML_SCHEMA_FACET_MAXEXCLUSIVE as c_int
                || facetType == XML_SCHEMA_FACET_MININCLUSIVE as c_int
                || facetType == XML_SCHEMA_FACET_MINEXCLUSIVE as c_int
            {
                return 1 as c_int;
            } else {
                return 0 as c_int;
            }
        }
        _ => {}
    }
    return 0 as c_int;
} }
#[no_mangle]
pub extern "C" fn xmlSchemaGetBuiltInType(mut type_0: xmlSchemaValType) -> xmlSchemaTypePtr { unsafe {
    if xmlSchemaTypesInitialized == 0 as c_int
        && xmlSchemaInitTypes() < 0 as c_int
    {
        return ::core::ptr::null_mut::<xmlSchemaType>();
    }
    match type_0 as c_uint {
        46 => return xmlSchemaTypeAnySimpleTypeDef,
        1 => return xmlSchemaTypeStringDef,
        2 => return xmlSchemaTypeNormStringDef,
        3 => return xmlSchemaTypeDecimalDef,
        4 => return xmlSchemaTypeTimeDef,
        5 => return xmlSchemaTypeGDayDef,
        6 => return xmlSchemaTypeGMonthDef,
        7 => return xmlSchemaTypeGMonthDayDef,
        8 => return xmlSchemaTypeGYearDef,
        9 => return xmlSchemaTypeGYearMonthDef,
        10 => return xmlSchemaTypeDateDef,
        11 => return xmlSchemaTypeDatetimeDef,
        12 => return xmlSchemaTypeDurationDef,
        13 => return xmlSchemaTypeFloatDef,
        14 => return xmlSchemaTypeDoubleDef,
        15 => return xmlSchemaTypeBooleanDef,
        16 => return xmlSchemaTypeTokenDef,
        17 => return xmlSchemaTypeLanguageDef,
        18 => return xmlSchemaTypeNmtokenDef,
        19 => return xmlSchemaTypeNmtokensDef,
        20 => return xmlSchemaTypeNameDef,
        21 => return xmlSchemaTypeQNameDef,
        22 => return xmlSchemaTypeNCNameDef,
        23 => return xmlSchemaTypeIdDef,
        24 => return xmlSchemaTypeIdrefDef,
        25 => return xmlSchemaTypeIdrefsDef,
        26 => return xmlSchemaTypeEntityDef,
        27 => return xmlSchemaTypeEntitiesDef,
        28 => return xmlSchemaTypeNotationDef,
        29 => return xmlSchemaTypeAnyURIDef,
        30 => return xmlSchemaTypeIntegerDef,
        31 => return xmlSchemaTypeNonPositiveIntegerDef,
        32 => return xmlSchemaTypeNegativeIntegerDef,
        33 => return xmlSchemaTypeNonNegativeIntegerDef,
        34 => return xmlSchemaTypePositiveIntegerDef,
        35 => return xmlSchemaTypeIntDef,
        36 => return xmlSchemaTypeUnsignedIntDef,
        37 => return xmlSchemaTypeLongDef,
        38 => return xmlSchemaTypeUnsignedLongDef,
        39 => return xmlSchemaTypeShortDef,
        40 => return xmlSchemaTypeUnsignedShortDef,
        41 => return xmlSchemaTypeByteDef,
        42 => return xmlSchemaTypeUnsignedByteDef,
        43 => return xmlSchemaTypeHexBinaryDef,
        44 => return xmlSchemaTypeBase64BinaryDef,
        45 => return xmlSchemaTypeAnyTypeDef,
        _ => return ::core::ptr::null_mut::<xmlSchemaType>(),
    };
} }
#[inline]
pub fn xmlSchemaValueAppend(
    mut prev: xmlSchemaValPtr,
    mut cur: xmlSchemaValPtr,
) -> c_int { unsafe {
    if prev.is_null() || cur.is_null() {
        return -(1 as c_int);
    }
    (*prev).next = cur as *mut _xmlSchemaVal;
    return 0 as c_int;
} }
#[inline]
pub fn xmlSchemaValueGetNext(mut cur: xmlSchemaValPtr) -> xmlSchemaValPtr { unsafe {
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    return (*cur).next as xmlSchemaValPtr;
} }
#[inline]
pub fn xmlSchemaValueGetAsString(mut val: xmlSchemaValPtr) -> *const xmlChar { unsafe {
    if val.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    match (*val).type_0 as c_uint {
        1 | 2 | 46 | 16 | 17 | 18 | 20 | 22 | 23 | 24 | 26 | 29 => {
            return (*val).value.str_0;
        }
        _ => {}
    }
    return ::core::ptr::null::<xmlChar>();
} }
#[inline]
pub fn xmlSchemaValueGetAsBoolean(
    mut val: xmlSchemaValPtr,
) -> c_int { unsafe {
    if val.is_null()
        || (*val).type_0 as c_uint
            != XML_SCHEMAS_BOOLEAN as c_int as c_uint
    {
        return 0 as c_int;
    }
    return (*val).value.b;
} }
#[inline]
pub unsafe fn xmlSchemaNewStringValue(
    mut type_0: xmlSchemaValType,
    mut value: *const xmlChar,
) -> xmlSchemaValPtr {
    let mut val: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    if type_0 as c_uint
        != XML_SCHEMAS_STRING as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    val = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlSchemaVal>() as size_t
    ) as xmlSchemaValPtr;
    if val.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    memset(
        val as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlSchemaVal>() as size_t,
    );
    (*val).type_0 = type_0;
    (*val).value.str_0 = value as *mut xmlChar;
    return val;
}
#[inline]
pub unsafe fn xmlSchemaNewNOTATIONValue(
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
) -> xmlSchemaValPtr {
    let mut val: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    val = xmlSchemaNewValue(XML_SCHEMAS_NOTATION);
    if val.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    (*val).value.qname.name = name as *mut xmlChar;
    if !ns.is_null() {
        (*val).value.qname.uri = ns as *mut xmlChar;
    }
    return val;
}
#[inline]
pub unsafe fn xmlSchemaNewQNameValue(
    mut namespaceName: *const xmlChar,
    mut localName: *const xmlChar,
) -> xmlSchemaValPtr {
    let mut val: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    val = xmlSchemaNewValue(XML_SCHEMAS_QNAME);
    if val.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    (*val).value.qname.name = localName as *mut xmlChar;
    (*val).value.qname.uri = namespaceName as *mut xmlChar;
    return val;
}
#[inline]
pub fn xmlSchemaFreeValue(mut value: xmlSchemaValPtr) { unsafe {
    let mut prev: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    while !value.is_null() {
        match (*value).type_0 as c_uint {
            1 | 2 | 16 | 17 | 18 | 19 | 20 | 22 | 23 | 24 | 25 | 26 | 27 | 29 | 46 => {
                if !(*value).value.str_0.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*value).value.str_0 as *mut c_void,
                    );
                }
            }
            28 | 21 => {
                if !(*value).value.qname.uri.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*value).value.qname.uri as *mut c_void,
                    );
                }
                if !(*value).value.qname.name.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*value).value.qname.name as *mut c_void,
                    );
                }
            }
            43 => {
                if !(*value).value.hex.str_0.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*value).value.hex.str_0 as *mut c_void,
                    );
                }
            }
            44 => {
                if !(*value).value.base64.str_0.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*value).value.base64.str_0 as *mut c_void,
                    );
                }
            }
            _ => {}
        }
        prev = value;
        value = (*value).next as xmlSchemaValPtr;
        xmlFree.expect("non-null function pointer")(prev as *mut c_void);
    }
} }
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaGetPredefinedType(
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
) -> xmlSchemaTypePtr {
    if xmlSchemaTypesInitialized == 0 as c_int
        && xmlSchemaInitTypes() < 0 as c_int
    {
        return ::core::ptr::null_mut::<xmlSchemaType>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaType>();
    }
    return xmlHashLookup2(xmlSchemaTypesBank, name, ns) as xmlSchemaTypePtr;
}
#[inline]
pub fn xmlSchemaGetBuiltInListSimpleTypeItemType(
    mut type_0: xmlSchemaTypePtr,
) -> xmlSchemaTypePtr { unsafe {
    if type_0.is_null()
        || (*type_0).type_0 as c_uint
            != XML_SCHEMA_TYPE_BASIC as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlSchemaType>();
    }
    match (*type_0).builtInType {
        19 => return xmlSchemaTypeNmtokenDef,
        25 => return xmlSchemaTypeIdrefDef,
        27 => return xmlSchemaTypeEntityDef,
        _ => return ::core::ptr::null_mut::<xmlSchemaType>(),
    };
} }
static mut daysInMonth: [c_uint; 12] = [
    31 as c_int as c_uint,
    28 as c_int as c_uint,
    31 as c_int as c_uint,
    30 as c_int as c_uint,
    31 as c_int as c_uint,
    30 as c_int as c_uint,
    31 as c_int as c_uint,
    31 as c_int as c_uint,
    30 as c_int as c_uint,
    31 as c_int as c_uint,
    30 as c_int as c_uint,
    31 as c_int as c_uint,
];
static mut daysInMonthLeap: [c_uint; 12] = [
    31 as c_int as c_uint,
    29 as c_int as c_uint,
    31 as c_int as c_uint,
    30 as c_int as c_uint,
    31 as c_int as c_uint,
    30 as c_int as c_uint,
    31 as c_int as c_uint,
    31 as c_int as c_uint,
    30 as c_int as c_uint,
    31 as c_int as c_uint,
    30 as c_int as c_uint,
    31 as c_int as c_uint,
];
pub const SECS_PER_MIN: c_int = 60 as c_int;
pub const MINS_PER_HOUR: c_int = 60 as c_int;
pub const HOURS_PER_DAY: c_int = 24 as c_int;
pub const SECS_PER_HOUR: c_int = MINS_PER_HOUR * SECS_PER_MIN;
pub const SECS_PER_DAY: c_int = HOURS_PER_DAY * SECS_PER_HOUR;
pub const MINS_PER_DAY: c_int = HOURS_PER_DAY * MINS_PER_HOUR;
static mut dayInYearByMonth: [c_long; 12] = [
    0 as c_int as c_long,
    31 as c_int as c_long,
    59 as c_int as c_long,
    90 as c_int as c_long,
    120 as c_int as c_long,
    151 as c_int as c_long,
    181 as c_int as c_long,
    212 as c_int as c_long,
    243 as c_int as c_long,
    273 as c_int as c_long,
    304 as c_int as c_long,
    334 as c_int as c_long,
];
static mut dayInLeapYearByMonth: [c_long; 12] = [
    0 as c_int as c_long,
    31 as c_int as c_long,
    60 as c_int as c_long,
    91 as c_int as c_long,
    121 as c_int as c_long,
    152 as c_int as c_long,
    182 as c_int as c_long,
    213 as c_int as c_long,
    244 as c_int as c_long,
    274 as c_int as c_long,
    305 as c_int as c_long,
    335 as c_int as c_long,
];
unsafe fn _xmlSchemaParseGYear(
    mut dt: xmlSchemaValDatePtr,
    mut str: *mut *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = *str;
    let mut firstChar: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut isneg: c_int = 0 as c_int;
    let mut digcnt: c_int = 0 as c_int;
    if ((*cur as c_int) < '0' as i32 || *cur as c_int > '9' as i32)
        && *cur as c_int != '-' as i32
        && *cur as c_int != '+' as i32
    {
        return -(1 as c_int);
    }
    if *cur as c_int == '-' as i32 {
        isneg = 1 as c_int;
        cur = cur.offset(1);
    }
    firstChar = cur;
    while *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
        let mut digit: c_int = *cur as c_int - '0' as i32;
        if (*dt).year > LONG_MAX / 10 as c_long {
            return 2 as c_int;
        }
        (*dt).year *= 10 as c_long;
        if (*dt).year > LONG_MAX - digit as c_long {
            return 2 as c_int;
        }
        (*dt).year += digit as c_long;
        cur = cur.offset(1);
        digcnt += 1;
    }
    if digcnt < 4 as c_int
        || digcnt > 4 as c_int && *firstChar as c_int == '0' as i32
    {
        return 1 as c_int;
    }
    if isneg != 0 {
        (*dt).year = -(*dt).year;
    }
    if !((*dt).year != 0 as c_long) {
        return 2 as c_int;
    }
    *str = cur;
    return 0 as c_int;
}
unsafe fn _xmlSchemaParseGMonth(
    mut dt: xmlSchemaValDatePtr,
    mut str: *mut *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = *str;
    let mut ret: c_int = 0 as c_int;
    let mut value: c_uint = 0 as c_uint;
    if (*cur.offset(0 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(0 as c_int as isize) as c_int > '9' as i32
        || (*cur.offset(1 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(1 as c_int as isize) as c_int > '9' as i32
    {
        ret = 1 as c_int;
    } else {
        value = ((*cur.offset(0 as c_int as isize) as c_int - '0' as i32)
            * 10 as c_int
            + (*cur.offset(1 as c_int as isize) as c_int - '0' as i32))
            as c_uint;
    }
    cur = cur.offset(2 as c_int as isize);
    if ret != 0 as c_int {
        return ret;
    }
    if !(value >= 1 as c_uint && value <= 12 as c_uint) {
        return 2 as c_int;
    }
    (*dt).set_mon(value as c_uint);
    *str = cur;
    return 0 as c_int;
}
unsafe fn _xmlSchemaParseGDay(
    mut dt: xmlSchemaValDatePtr,
    mut str: *mut *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = *str;
    let mut ret: c_int = 0 as c_int;
    let mut value: c_uint = 0 as c_uint;
    if (*cur.offset(0 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(0 as c_int as isize) as c_int > '9' as i32
        || (*cur.offset(1 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(1 as c_int as isize) as c_int > '9' as i32
    {
        ret = 1 as c_int;
    } else {
        value = ((*cur.offset(0 as c_int as isize) as c_int - '0' as i32)
            * 10 as c_int
            + (*cur.offset(1 as c_int as isize) as c_int - '0' as i32))
            as c_uint;
    }
    cur = cur.offset(2 as c_int as isize);
    if ret != 0 as c_int {
        return ret;
    }
    if !(value >= 1 as c_uint && value <= 31 as c_uint) {
        return 2 as c_int;
    }
    (*dt).set_day(value as c_uint);
    *str = cur;
    return 0 as c_int;
}
unsafe fn _xmlSchemaParseTime(
    mut dt: xmlSchemaValDatePtr,
    mut str: *mut *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = *str;
    let mut ret: c_int = 0 as c_int;
    let mut value: c_int = 0 as c_int;
    if (*cur.offset(0 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(0 as c_int as isize) as c_int > '9' as i32
        || (*cur.offset(1 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(1 as c_int as isize) as c_int > '9' as i32
    {
        ret = 1 as c_int;
    } else {
        value = (*cur.offset(0 as c_int as isize) as c_int - '0' as i32)
            * 10 as c_int
            + (*cur.offset(1 as c_int as isize) as c_int - '0' as i32);
    }
    cur = cur.offset(2 as c_int as isize);
    if ret != 0 as c_int {
        return ret;
    }
    if *cur as c_int != ':' as i32 {
        return 1 as c_int;
    }
    if !(value >= 0 as c_int && value <= 23 as c_int)
        && value != 24 as c_int
    {
        return 2 as c_int;
    }
    cur = cur.offset(1);
    (*dt).set_hour(value as c_uint as c_uint);
    if (*cur.offset(0 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(0 as c_int as isize) as c_int > '9' as i32
        || (*cur.offset(1 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(1 as c_int as isize) as c_int > '9' as i32
    {
        ret = 1 as c_int;
    } else {
        value = (*cur.offset(0 as c_int as isize) as c_int - '0' as i32)
            * 10 as c_int
            + (*cur.offset(1 as c_int as isize) as c_int - '0' as i32);
    }
    cur = cur.offset(2 as c_int as isize);
    if ret != 0 as c_int {
        return ret;
    }
    if !(value >= 0 as c_int && value <= 59 as c_int) {
        return 2 as c_int;
    }
    (*dt).set_min(value as c_uint as c_uint);
    if *cur as c_int != ':' as i32 {
        return 1 as c_int;
    }
    cur = cur.offset(1);
    if (*cur.offset(0 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(0 as c_int as isize) as c_int > '9' as i32
        || (*cur.offset(1 as c_int as isize) as c_int) < '0' as i32
        || *cur.offset(1 as c_int as isize) as c_int > '9' as i32
    {
        ret = 1 as c_int;
    } else {
        (*dt).sec = ((*cur.offset(0 as c_int as isize) as c_int
            - '0' as i32)
            * 10 as c_int
            + (*cur.offset(1 as c_int as isize) as c_int - '0' as i32))
            as c_double;
    }
    cur = cur.offset(2 as c_int as isize);
    if ret == 0 && *cur as c_int == '.' as i32 {
        let mut mult: c_double = 1 as c_int as c_double;
        cur = cur.offset(1);
        if (*cur as c_int) < '0' as i32 || *cur as c_int > '9' as i32 {
            ret = 1 as c_int;
        }
        while *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
            mult /= 10 as c_int as c_double;
            (*dt).sec += (*cur as c_int - '0' as i32) as c_double * mult;
            cur = cur.offset(1);
        }
    }
    if ret != 0 as c_int {
        return ret;
    }
    if !(((*dt).hour() as c_int >= 0 as c_int
        && (*dt).hour() as c_int <= 23 as c_int
        && ((*dt).min() as c_int >= 0 as c_int
            && (*dt).min() as c_int <= 59 as c_int)
        && ((*dt).sec >= 0 as c_int as c_double
            && (*dt).sec < 60 as c_int as c_double)
        || (*dt).hour() as c_int == 24 as c_int
            && (*dt).min() as c_int == 0 as c_int
            && (*dt).sec == 0 as c_int as c_double)
        && ((*dt).tzo() >= -(840 as c_int)
            && (*dt).tzo() <= 840 as c_int))
    {
        return 2 as c_int;
    }
    *str = cur;
    return 0 as c_int;
}
unsafe fn _xmlSchemaParseTimeZone(
    mut dt: xmlSchemaValDatePtr,
    mut str: *mut *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: c_int = 0 as c_int;
    if str.is_null() {
        return -(1 as c_int);
    }
    cur = *str;
    match *cur as c_int {
        0 => {
            (*dt).set_tz_flag(0 as c_uint as c_uint);
            (*dt).set_tzo(0 as c_int as c_int);
        }
        90 => {
            (*dt).set_tz_flag(1 as c_uint as c_uint);
            (*dt).set_tzo(0 as c_int as c_int);
            cur = cur.offset(1);
        }
        43 | 45 => {
            let mut isneg: c_int = 0 as c_int;
            let mut tmp: c_int = 0 as c_int;
            isneg = (*cur as c_int == '-' as i32) as c_int;
            cur = cur.offset(1);
            if (*cur.offset(0 as c_int as isize) as c_int) < '0' as i32
                || *cur.offset(0 as c_int as isize) as c_int > '9' as i32
                || (*cur.offset(1 as c_int as isize) as c_int)
                    < '0' as i32
                || *cur.offset(1 as c_int as isize) as c_int > '9' as i32
            {
                ret = 1 as c_int;
            } else {
                tmp = (*cur.offset(0 as c_int as isize) as c_int
                    - '0' as i32)
                    * 10 as c_int
                    + (*cur.offset(1 as c_int as isize) as c_int
                        - '0' as i32);
            }
            cur = cur.offset(2 as c_int as isize);
            if ret != 0 as c_int {
                return ret;
            }
            if !(tmp >= 0 as c_int && tmp <= 23 as c_int) {
                return 2 as c_int;
            }
            if *cur as c_int != ':' as i32 {
                return 1 as c_int;
            }
            cur = cur.offset(1);
            (*dt).set_tzo((tmp * 60 as c_int) as c_int);
            if (*cur.offset(0 as c_int as isize) as c_int) < '0' as i32
                || *cur.offset(0 as c_int as isize) as c_int > '9' as i32
                || (*cur.offset(1 as c_int as isize) as c_int)
                    < '0' as i32
                || *cur.offset(1 as c_int as isize) as c_int > '9' as i32
            {
                ret = 1 as c_int;
            } else {
                tmp = (*cur.offset(0 as c_int as isize) as c_int
                    - '0' as i32)
                    * 10 as c_int
                    + (*cur.offset(1 as c_int as isize) as c_int
                        - '0' as i32);
            }
            cur = cur.offset(2 as c_int as isize);
            if ret != 0 as c_int {
                return ret;
            }
            if !(tmp >= 0 as c_int && tmp <= 59 as c_int) {
                return 2 as c_int;
            }
            (*dt).set_tzo((*dt).tzo() + tmp as c_int);
            if isneg != 0 {
                (*dt).set_tzo(-(*dt).tzo() as c_int);
            }
            if !((*dt).tzo() >= -(840 as c_int)
                && (*dt).tzo() <= 840 as c_int)
            {
                return 2 as c_int;
            }
            (*dt).set_tz_flag(1 as c_uint as c_uint);
        }
        _ => return 1 as c_int,
    }
    *str = cur;
    return 0 as c_int;
}
fn _xmlSchemaBase64Decode(ch: xmlChar) -> c_int { {
    if 'A' as i32 <= ch as c_int && ch as c_int <= 'Z' as i32 {
        return ch as c_int - 'A' as i32;
    }
    if 'a' as i32 <= ch as c_int && ch as c_int <= 'z' as i32 {
        return ch as c_int - 'a' as i32 + 26 as c_int;
    }
    if '0' as i32 <= ch as c_int && ch as c_int <= '9' as i32 {
        return ch as c_int - '0' as i32 + 52 as c_int;
    }
    if '+' as i32 == ch as c_int {
        return 62 as c_int;
    }
    if '/' as i32 == ch as c_int {
        return 63 as c_int;
    }
    if '=' as i32 == ch as c_int {
        return 64 as c_int;
    }
    return -(1 as c_int);
} }
unsafe fn xmlSchemaValidateDates(
    mut type_0: xmlSchemaValType,
    mut dateTime: *const xmlChar,
    mut val: *mut xmlSchemaValPtr,
    mut collapse: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut dt: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut ret: c_int = 0;
    let mut cur: *const xmlChar = dateTime;
    if dateTime.is_null() {
        return -(1 as c_int);
    }
    if collapse != 0 {
        while *cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int
        {
            cur = cur.offset(1);
        }
    }
    if *cur as c_int != '-' as i32
        && (*cur as c_int) < '0' as i32
        && *cur as c_int > '9' as i32
    {
        return 1 as c_int;
    }
    dt = xmlSchemaNewValue(XML_SCHEMAS_UNKNOWN);
    if dt.is_null() {
        return -(1 as c_int);
    }
    if *cur.offset(0 as c_int as isize) as c_int == '-' as i32
        && *cur.offset(1 as c_int as isize) as c_int == '-' as i32
    {
        cur = cur.offset(2 as c_int as isize);
        if *cur as c_int == '-' as i32 {
            if type_0 as c_uint
                == XML_SCHEMAS_GMONTH as c_int as c_uint
            {
                current_block = 2135172792998989146;
            } else {
                cur = cur.offset(1);
                ret = _xmlSchemaParseGDay(&raw mut (*dt).value.date, &raw mut cur);
                if ret != 0 as c_int {
                    current_block = 2135172792998989146;
                } else if *cur as c_int == 0 as c_int
                    || *cur as c_int == 'Z' as i32
                    || *cur as c_int == '+' as i32
                    || *cur as c_int == '-' as i32
                {
                    ret = _xmlSchemaParseTimeZone(&raw mut (*dt).value.date, &raw mut cur);
                    if ret == 0 as c_int {
                        if *cur as c_int != 0 as c_int {
                            current_block = 2135172792998989146;
                        } else {
                            (*dt).type_0 = XML_SCHEMAS_GDAY;
                            current_block = 17869761855184201886;
                        }
                    } else {
                        current_block = 2135172792998989146;
                    }
                } else {
                    current_block = 2135172792998989146;
                }
            }
        } else {
            ret = _xmlSchemaParseGMonth(&raw mut (*dt).value.date, &raw mut cur);
            if ret != 0 as c_int {
                current_block = 2135172792998989146;
            } else {
                if *cur as c_int == '-' as i32 {
                    let mut rewnd: *const xmlChar = cur;
                    cur = cur.offset(1);
                    ret = _xmlSchemaParseGDay(&raw mut (*dt).value.date, &raw mut cur);
                    if ret == 0 as c_int
                        && (*cur as c_int == 0 as c_int
                            || *cur as c_int != ':' as i32)
                    {
                        if if (*dt).value.date.year % 4 as c_long
                            == 0 as c_long
                            && (*dt).value.date.year % 100 as c_long
                                != 0 as c_long
                            || (*dt).value.date.year % 400 as c_long
                                == 0 as c_long
                        {
                            ((*dt).value.date.day()
                                <= daysInMonthLeap[((*dt).value.date.mon() as c_int
                                    - 1 as c_int)
                                    as usize]) as c_int
                        } else {
                            ((*dt).value.date.day()
                                <= daysInMonth[((*dt).value.date.mon() as c_int
                                    - 1 as c_int)
                                    as usize]) as c_int
                        } != 0
                        {
                            if *cur as c_int == 0 as c_int
                                || *cur as c_int == 'Z' as i32
                                || *cur as c_int == '+' as i32
                                || *cur as c_int == '-' as i32
                            {
                                ret = _xmlSchemaParseTimeZone(
                                    &raw mut (*dt).value.date,
                                    &raw mut cur,
                                );
                                if ret == 0 as c_int {
                                    if *cur as c_int != 0 as c_int {
                                        current_block = 2135172792998989146;
                                    } else {
                                        (*dt).type_0 = XML_SCHEMAS_GMONTHDAY;
                                        current_block = 17869761855184201886;
                                    }
                                } else {
                                    current_block = 2135172792998989146;
                                }
                            } else {
                                current_block = 2135172792998989146;
                            }
                        } else {
                            current_block = 652864300344834934;
                        }
                    } else {
                        current_block = 652864300344834934;
                    }
                    match current_block {
                        17869761855184201886 => {}
                        2135172792998989146 => {}
                        _ => {
                            cur = rewnd;
                            current_block = 14136749492126903395;
                        }
                    }
                } else {
                    current_block = 14136749492126903395;
                }
                match current_block {
                    2135172792998989146 => {}
                    17869761855184201886 => {}
                    _ => {
                        if *cur as c_int == 0 as c_int
                            || *cur as c_int == 'Z' as i32
                            || *cur as c_int == '+' as i32
                            || *cur as c_int == '-' as i32
                        {
                            ret = _xmlSchemaParseTimeZone(&raw mut (*dt).value.date, &raw mut cur);
                            if ret == 0 as c_int {
                                if *cur as c_int != 0 as c_int {
                                    current_block = 2135172792998989146;
                                } else {
                                    (*dt).type_0 = XML_SCHEMAS_GMONTH;
                                    current_block = 17869761855184201886;
                                }
                            } else {
                                current_block = 2135172792998989146;
                            }
                        } else {
                            current_block = 2135172792998989146;
                        }
                    }
                }
            }
        }
    } else {
        if *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
            ret = _xmlSchemaParseTime(&raw mut (*dt).value.date, &raw mut cur);
            if ret == 0 as c_int {
                if *cur as c_int == 0 as c_int
                    || *cur as c_int == 'Z' as i32
                    || *cur as c_int == '+' as i32
                    || *cur as c_int == '-' as i32
                {
                    ret = _xmlSchemaParseTimeZone(&raw mut (*dt).value.date, &raw mut cur);
                    if ret == 0 as c_int {
                        if *cur as c_int != 0 as c_int {
                            current_block = 2135172792998989146;
                        } else {
                            (*dt).type_0 = XML_SCHEMAS_TIME;
                            current_block = 17869761855184201886;
                        }
                    } else {
                        current_block = 16738040538446813684;
                    }
                } else {
                    current_block = 16738040538446813684;
                }
            } else {
                current_block = 16738040538446813684;
            }
        } else {
            current_block = 16738040538446813684;
        }
        match current_block {
            2135172792998989146 => {}
            17869761855184201886 => {}
            _ => {
                cur = dateTime;
                ret = _xmlSchemaParseGYear(&raw mut (*dt).value.date, &raw mut cur);
                if ret != 0 as c_int {
                    current_block = 2135172792998989146;
                } else {
                    if *cur as c_int == 0 as c_int
                        || *cur as c_int == 'Z' as i32
                        || *cur as c_int == '+' as i32
                        || *cur as c_int == '-' as i32
                    {
                        ret = _xmlSchemaParseTimeZone(&raw mut (*dt).value.date, &raw mut cur);
                        if ret == 0 as c_int {
                            if *cur as c_int != 0 as c_int {
                                current_block = 2135172792998989146;
                            } else {
                                (*dt).type_0 = XML_SCHEMAS_GYEAR;
                                current_block = 17869761855184201886;
                            }
                        } else {
                            current_block = 16415152177862271243;
                        }
                    } else {
                        current_block = 16415152177862271243;
                    }
                    match current_block {
                        2135172792998989146 => {}
                        17869761855184201886 => {}
                        _ => {
                            if *cur as c_int != '-' as i32 {
                                current_block = 2135172792998989146;
                            } else {
                                cur = cur.offset(1);
                                ret =
                                    _xmlSchemaParseGMonth(&raw mut (*dt).value.date, &raw mut cur);
                                if ret != 0 as c_int {
                                    current_block = 2135172792998989146;
                                } else {
                                    if *cur as c_int == 0 as c_int
                                        || *cur as c_int == 'Z' as i32
                                        || *cur as c_int == '+' as i32
                                        || *cur as c_int == '-' as i32
                                    {
                                        ret = _xmlSchemaParseTimeZone(
                                            &raw mut (*dt).value.date,
                                            &raw mut cur,
                                        );
                                        if ret == 0 as c_int {
                                            if *cur as c_int != 0 as c_int
                                            {
                                                current_block = 2135172792998989146;
                                            } else {
                                                (*dt).type_0 = XML_SCHEMAS_GYEARMONTH;
                                                current_block = 17869761855184201886;
                                            }
                                        } else {
                                            current_block = 981995395831942902;
                                        }
                                    } else {
                                        current_block = 981995395831942902;
                                    }
                                    match current_block {
                                        2135172792998989146 => {}
                                        17869761855184201886 => {}
                                        _ => {
                                            if *cur as c_int != '-' as i32 {
                                                current_block = 2135172792998989146;
                                            } else {
                                                cur = cur.offset(1);
                                                ret = _xmlSchemaParseGDay(
                                                    &raw mut (*dt).value.date,
                                                    &raw mut cur,
                                                );
                                                if ret != 0 as c_int
                                                    || !((*dt).value.date.year
                                                        != 0 as c_long
                                                        && ((*dt).value.date.mon()
                                                            as c_int
                                                            >= 1 as c_int
                                                            && (*dt).value.date.mon()
                                                                as c_int
                                                                <= 12 as c_int)
                                                        && (if (*dt).value.date.year
                                                            % 4 as c_long
                                                            == 0 as c_long
                                                            && (*dt).value.date.year
                                                                % 100 as c_long
                                                                != 0 as c_long
                                                            || (*dt).value.date.year
                                                                % 400 as c_long
                                                                == 0 as c_long
                                                        {
                                                            ((*dt).value.date.day()
                                                                <= daysInMonthLeap[((*dt)
                                                                    .value
                                                                    .date
                                                                    .mon()
                                                                    as c_int
                                                                    - 1 as c_int)
                                                                    as usize])
                                                                as c_int
                                                        } else {
                                                            ((*dt).value.date.day()
                                                                <= daysInMonth[((*dt)
                                                                    .value
                                                                    .date
                                                                    .mon()
                                                                    as c_int
                                                                    - 1 as c_int)
                                                                    as usize])
                                                                as c_int
                                                        }) != 0)
                                                {
                                                    current_block = 2135172792998989146;
                                                } else {
                                                    if *cur as c_int
                                                        == 0 as c_int
                                                        || *cur as c_int == 'Z' as i32
                                                        || *cur as c_int == '+' as i32
                                                        || *cur as c_int == '-' as i32
                                                    {
                                                        ret = _xmlSchemaParseTimeZone(
                                                            &raw mut (*dt).value.date,
                                                            &raw mut cur,
                                                        );
                                                        if ret == 0 as c_int {
                                                            if *cur as c_int
                                                                != 0 as c_int
                                                            {
                                                                current_block = 2135172792998989146;
                                                            } else {
                                                                (*dt).type_0 = XML_SCHEMAS_DATE;
                                                                current_block =
                                                                    17869761855184201886;
                                                            }
                                                        } else {
                                                            current_block = 17239133558811367971;
                                                        }
                                                    } else {
                                                        current_block = 17239133558811367971;
                                                    }
                                                    match current_block {
                                                        17869761855184201886 => {}
                                                        2135172792998989146 => {}
                                                        _ => {
                                                            if *cur as c_int
                                                                != 'T' as i32
                                                            {
                                                                current_block = 2135172792998989146;
                                                            } else {
                                                                cur = cur.offset(1);
                                                                ret = _xmlSchemaParseTime(
                                                                    &raw mut (*dt).value.date,
                                                                    &raw mut cur,
                                                                );
                                                                if ret != 0 as c_int {
                                                                    current_block =
                                                                        2135172792998989146;
                                                                } else {
                                                                    ret = _xmlSchemaParseTimeZone(
                                                                        &raw mut (*dt).value.date,
                                                                        &raw mut cur,
                                                                    );
                                                                    if collapse != 0 {
                                                                        while *cur as c_int
                                                                            == 0x20 as c_int
                                                                            || 0x9 as c_int <= *cur as c_int
                                                                                && *cur as c_int <= 0xa as c_int
                                                                            || *cur as c_int == 0xd as c_int
                                                                        {
                                                                            cur = cur.offset(1);
                                                                        }
                                                                    }
                                                                    if ret != 0 as c_int
                                                                        || *cur as c_int != 0 as c_int
                                                                        || !((*dt).value.date.year != 0 as c_long
                                                                            && ((*dt).value.date.mon() as c_int
                                                                                >= 1 as c_int
                                                                                && (*dt).value.date.mon() as c_int
                                                                                    <= 12 as c_int)
                                                                            && (if (*dt).value.date.year % 4 as c_long
                                                                                == 0 as c_long
                                                                                && (*dt).value.date.year % 100 as c_long
                                                                                    != 0 as c_long
                                                                                || (*dt).value.date.year % 400 as c_long
                                                                                    == 0 as c_long
                                                                            {
                                                                                ((*dt).value.date.day()
                                                                                    <= daysInMonthLeap[((*dt).value.date.mon()
                                                                                        as c_int - 1 as c_int) as usize])
                                                                                    as c_int
                                                                            } else {
                                                                                ((*dt).value.date.day()
                                                                                    <= daysInMonth[((*dt).value.date.mon() as c_int
                                                                                        - 1 as c_int) as usize]) as c_int
                                                                            }) != 0
                                                                            && (((*dt).value.date.hour() as c_int
                                                                                >= 0 as c_int
                                                                                && (*dt).value.date.hour() as c_int
                                                                                    <= 23 as c_int
                                                                                && ((*dt).value.date.min() as c_int
                                                                                    >= 0 as c_int
                                                                                    && (*dt).value.date.min() as c_int
                                                                                        <= 59 as c_int)
                                                                                && ((*dt).value.date.sec
                                                                                    >= 0 as c_int as c_double
                                                                                    && (*dt).value.date.sec
                                                                                        < 60 as c_int as c_double)
                                                                                || (*dt).value.date.hour() as c_int
                                                                                    == 24 as c_int
                                                                                    && (*dt).value.date.min() as c_int
                                                                                        == 0 as c_int
                                                                                    && (*dt).value.date.sec
                                                                                        == 0 as c_int as c_double)
                                                                                && ((*dt).value.date.tzo() >= -(840 as c_int)
                                                                                    && (*dt).value.date.tzo() <= 840 as c_int)))
                                                                    {
                                                                        current_block = 2135172792998989146;
                                                                    } else {
                                                                        (*dt).type_0 = XML_SCHEMAS_DATETIME;
                                                                        current_block = 17869761855184201886;
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
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        17869761855184201886 => {
            if !(type_0 as c_uint
                != XML_SCHEMAS_UNKNOWN as c_int as c_uint
                && type_0 as c_uint != (*dt).type_0 as c_uint)
            {
                if !val.is_null() {
                    *val = dt;
                } else {
                    xmlSchemaFreeValue(dt);
                }
                return 0 as c_int;
            }
        }
        _ => {}
    }
    if !dt.is_null() {
        xmlSchemaFreeValue(dt);
    }
    return 1 as c_int;
}
unsafe fn xmlSchemaValidateDuration(
    mut type_0: xmlSchemaTypePtr,
    mut duration: *const xmlChar,
    mut val: *mut xmlSchemaValPtr,
    mut collapse: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut cur: *const xmlChar = duration;
    let mut dur: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut isneg: c_int = 0 as c_int;
    let mut seq: c_uint = 0 as c_uint;
    let mut days: c_long = 0;
    let mut secs: c_long = 0 as c_long;
    let mut sec_frac: c_double = 0.0f64;
    if duration.is_null() {
        return -(1 as c_int);
    }
    if collapse != 0 {
        while *cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int
        {
            cur = cur.offset(1);
        }
    }
    if *cur as c_int == '-' as i32 {
        isneg = 1 as c_int;
        cur = cur.offset(1);
    }
    let fresh20 = cur;
    cur = cur.offset(1);
    if *fresh20 as c_int != 'P' as i32 {
        return 1 as c_int;
    }
    if *cur as c_int == 0 as c_int {
        return 1 as c_int;
    }
    dur = xmlSchemaNewValue(XML_SCHEMAS_DURATION);
    if dur.is_null() {
        return -(1 as c_int);
    }
    's_73: loop {
        if !(*cur as c_int != 0 as c_int) {
            current_block = 8835654301469918283;
            break;
        }
        let mut num: c_long = 0 as c_long;
        let mut has_digits: size_t = 0 as size_t;
        let mut has_frac: c_int = 0 as c_int;
        let desig: [xmlChar; 6] = [
            'Y' as i32 as xmlChar,
            'M' as i32 as xmlChar,
            'D' as i32 as xmlChar,
            'H' as i32 as xmlChar,
            'M' as i32 as xmlChar,
            'S' as i32 as xmlChar,
        ];
        if seq as usize >= ::core::mem::size_of::<[xmlChar; 6]>() as usize {
            current_block = 4607817804165056648;
            break;
        }
        if *cur as c_int == 'T' as i32 {
            if seq > 3 as c_uint {
                current_block = 4607817804165056648;
                break;
            }
            cur = cur.offset(1);
            seq = 3 as c_uint;
        } else if seq == 3 as c_uint {
            current_block = 4607817804165056648;
            break;
        }
        while *cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32 {
            let mut digit: c_long =
                (*cur as c_int - '0' as i32) as c_long;
            if num > LONG_MAX / 10 as c_long {
                current_block = 4607817804165056648;
                break 's_73;
            }
            num *= 10 as c_long;
            if num > LONG_MAX - digit {
                current_block = 4607817804165056648;
                break 's_73;
            }
            num += digit;
            has_digits = 1 as size_t;
            cur = cur.offset(1);
        }
        if *cur as c_int == '.' as i32 {
            let mut mult: c_double = 1.0f64;
            cur = cur.offset(1);
            has_frac = 1 as c_int;
            while *cur as c_int >= '0' as i32
                && *cur as c_int <= '9' as i32
            {
                mult /= 10.0f64;
                sec_frac +=
                    (*cur as c_int - '0' as i32) as c_double * mult;
                has_digits = 1 as size_t;
                cur = cur.offset(1);
            }
        }
        while *cur as c_int != desig[seq as usize] as c_int {
            seq = seq.wrapping_add(1);
            if seq == 3 as c_uint
                || seq as usize == ::core::mem::size_of::<[xmlChar; 6]>() as usize
            {
                current_block = 4607817804165056648;
                break 's_73;
            }
        }
        cur = cur.offset(1);
        if has_digits == 0 || has_frac != 0 && seq != 5 as c_uint {
            current_block = 4607817804165056648;
            break;
        }
        match seq {
            0 => {
                if num > LONG_MAX / 12 as c_long {
                    current_block = 4607817804165056648;
                    break;
                }
                (*dur).value.dur.mon = num * 12 as c_long;
            }
            1 => {
                if (*dur).value.dur.mon > LONG_MAX - num {
                    current_block = 4607817804165056648;
                    break;
                }
                (*dur).value.dur.mon += num;
            }
            2 => {
                (*dur).value.dur.day = num;
            }
            3 => {
                days = num / HOURS_PER_DAY as c_long;
                if (*dur).value.dur.day > LONG_MAX - days {
                    current_block = 4607817804165056648;
                    break;
                }
                (*dur).value.dur.day += days;
                secs = num % HOURS_PER_DAY as c_long
                    * SECS_PER_HOUR as c_long;
            }
            4 => {
                days = num / MINS_PER_DAY as c_long;
                if (*dur).value.dur.day > LONG_MAX - days {
                    current_block = 4607817804165056648;
                    break;
                }
                (*dur).value.dur.day += days;
                secs +=
                    num % MINS_PER_DAY as c_long * SECS_PER_MIN as c_long;
            }
            5 => {
                days = num / SECS_PER_DAY as c_long;
                if (*dur).value.dur.day > LONG_MAX - days {
                    current_block = 4607817804165056648;
                    break;
                }
                (*dur).value.dur.day += days;
                secs += num % SECS_PER_DAY as c_long;
            }
            _ => {}
        }
        seq = seq.wrapping_add(1);
    }
    match current_block {
        8835654301469918283 => {
            days = secs / SECS_PER_DAY as c_long;
            if !((*dur).value.dur.day > LONG_MAX - days) {
                (*dur).value.dur.day += days;
                (*dur).value.dur.sec = (secs % SECS_PER_DAY as c_long)
                    as c_double
                    + sec_frac;
                if isneg != 0 {
                    (*dur).value.dur.mon = -(*dur).value.dur.mon;
                    (*dur).value.dur.day = -(*dur).value.dur.day;
                    (*dur).value.dur.sec = -(*dur).value.dur.sec;
                }
                if !val.is_null() {
                    *val = dur;
                } else {
                    xmlSchemaFreeValue(dur);
                }
                return 0 as c_int;
            }
        }
        _ => {}
    }
    if !dur.is_null() {
        xmlSchemaFreeValue(dur);
    }
    return 1 as c_int;
}
unsafe fn xmlSchemaStrip(mut value: *const xmlChar) -> *mut xmlChar {
    let mut start: *const xmlChar = value;
    let mut end: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut f: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while *start as c_int != 0 as c_int
        && (*start as c_int == 0x20 as c_int
            || 0x9 as c_int <= *start as c_int
                && *start as c_int <= 0xa as c_int
            || *start as c_int == 0xd as c_int)
    {
        start = start.offset(1);
    }
    end = start;
    while *end as c_int != 0 as c_int {
        end = end.offset(1);
    }
    f = end;
    end = end.offset(-1);
    while end > start
        && (*end as c_int == 0x20 as c_int
            || 0x9 as c_int <= *end as c_int
                && *end as c_int <= 0xa as c_int
            || *end as c_int == 0xd as c_int)
    {
        end = end.offset(-1);
    }
    end = end.offset(1);
    if start == value && f == end {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlStrndup(
        start,
        end.offset_from(start) as c_long as c_int,
    );
}
#[inline]
pub unsafe fn xmlSchemaWhiteSpaceReplace(mut value: *const xmlChar) -> *mut xmlChar {
    let mut cur: *const xmlChar = value;
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut mcur: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while *cur as c_int != 0 as c_int
        && (*cur as c_int != 0xd as c_int
            && *cur as c_int != 0x9 as c_int
            && *cur as c_int != 0xa as c_int)
    {
        cur = cur.offset(1);
    }
    if *cur as c_int == 0 as c_int {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlStrdup(value);
    mcur = ret.offset(cur.offset_from(value) as c_long as isize);
    loop {
        if *mcur as c_int == 0xd as c_int
            || *mcur as c_int == 0x9 as c_int
            || *mcur as c_int == 0xa as c_int
        {
            *mcur = ' ' as i32 as xmlChar;
        }
        mcur = mcur.offset(1);
        if !(*mcur as c_int != 0 as c_int) {
            break;
        }
    }
    return ret;
}
#[inline]
pub unsafe fn xmlSchemaCollapseString(mut value: *const xmlChar) -> *mut xmlChar {
    let mut start: *const xmlChar = value;
    let mut end: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut f: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut g: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut col: c_int = 0 as c_int;
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while *start as c_int != 0 as c_int
        && (*start as c_int == 0x20 as c_int
            || 0x9 as c_int <= *start as c_int
                && *start as c_int <= 0xa as c_int
            || *start as c_int == 0xd as c_int)
    {
        start = start.offset(1);
    }
    end = start;
    while *end as c_int != 0 as c_int {
        if *end as c_int == ' ' as i32
            && (*end.offset(1 as c_int as isize) as c_int
                == 0x20 as c_int
                || 0x9 as c_int
                    <= *end.offset(1 as c_int as isize) as c_int
                    && *end.offset(1 as c_int as isize) as c_int
                        <= 0xa as c_int
                || *end.offset(1 as c_int as isize) as c_int
                    == 0xd as c_int)
        {
            col = end.offset_from(start) as c_long as c_int;
            break;
        } else if *end as c_int == 0xa as c_int
            || *end as c_int == 0x9 as c_int
            || *end as c_int == 0xd as c_int
        {
            col = end.offset_from(start) as c_long as c_int;
            break;
        } else {
            end = end.offset(1);
        }
    }
    if col == 0 as c_int {
        f = end;
        end = end.offset(-1);
        while end > start
            && (*end as c_int == 0x20 as c_int
                || 0x9 as c_int <= *end as c_int
                    && *end as c_int <= 0xa as c_int
                || *end as c_int == 0xd as c_int)
        {
            end = end.offset(-1);
        }
        end = end.offset(1);
        if start == value && f == end {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        return xmlStrndup(
            start,
            end.offset_from(start) as c_long as c_int,
        );
    }
    start = xmlStrdup(start);
    if start.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    g = start.offset(col as isize) as *mut xmlChar;
    end = g;
    while *end as c_int != 0 as c_int {
        if *end as c_int == 0x20 as c_int
            || 0x9 as c_int <= *end as c_int
                && *end as c_int <= 0xa as c_int
            || *end as c_int == 0xd as c_int
        {
            end = end.offset(1);
            while *end as c_int == 0x20 as c_int
                || 0x9 as c_int <= *end as c_int
                    && *end as c_int <= 0xa as c_int
                || *end as c_int == 0xd as c_int
            {
                end = end.offset(1);
            }
            if *end as c_int != 0 as c_int {
                let fresh15 = g;
                g = g.offset(1);
                *fresh15 = ' ' as i32 as xmlChar;
            }
        } else {
            let fresh16 = end;
            end = end.offset(1);
            let fresh17 = g;
            g = g.offset(1);
            *fresh17 = *fresh16;
        }
    }
    *g = 0 as xmlChar;
    return start as *mut xmlChar;
}
unsafe fn xmlSchemaValAtomicListNode(
    mut type_0: xmlSchemaTypePtr,
    mut value: *const xmlChar,
    mut ret: *mut xmlSchemaValPtr,
    mut node: xmlNodePtr,
) -> c_int {
    let mut val: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut endval: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut nb_values: c_int = 0 as c_int;
    let mut tmp: c_int = 0 as c_int;
    if value.is_null() {
        return -(1 as c_int);
    }
    val = xmlStrdup(value);
    if val.is_null() {
        return -(1 as c_int);
    }
    if !ret.is_null() {
        *ret = ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    cur = val;
    while *cur as c_int == 0x20 as c_int
        || 0x9 as c_int <= *cur as c_int
            && *cur as c_int <= 0xa as c_int
        || *cur as c_int == 0xd as c_int
    {
        let fresh18 = cur;
        cur = cur.offset(1);
        *fresh18 = 0 as xmlChar;
    }
    while *cur as c_int != 0 as c_int {
        if *cur as c_int == 0x20 as c_int
            || 0x9 as c_int <= *cur as c_int
                && *cur as c_int <= 0xa as c_int
            || *cur as c_int == 0xd as c_int
        {
            *cur = 0 as xmlChar;
            cur = cur.offset(1);
            while *cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *cur as c_int
                    && *cur as c_int <= 0xa as c_int
                || *cur as c_int == 0xd as c_int
            {
                let fresh19 = cur;
                cur = cur.offset(1);
                *fresh19 = 0 as xmlChar;
            }
        } else {
            nb_values += 1;
            cur = cur.offset(1);
            while *cur as c_int != 0 as c_int
                && !(*cur as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *cur as c_int
                        && *cur as c_int <= 0xa as c_int
                    || *cur as c_int == 0xd as c_int)
            {
                cur = cur.offset(1);
            }
        }
    }
    if nb_values == 0 as c_int {
        xmlFree.expect("non-null function pointer")(val as *mut c_void);
        return nb_values;
    }
    endval = cur;
    cur = val;
    while *cur as c_int == 0 as c_int && cur != endval {
        cur = cur.offset(1);
    }
    while cur != endval {
        tmp = xmlSchemaValPredefTypeNode(
            type_0,
            cur,
            ::core::ptr::null_mut::<xmlSchemaValPtr>(),
            node,
        );
        if tmp != 0 as c_int {
            break;
        }
        while *cur as c_int != 0 as c_int {
            cur = cur.offset(1);
        }
        while *cur as c_int == 0 as c_int && cur != endval {
            cur = cur.offset(1);
        }
    }
    xmlFree.expect("non-null function pointer")(val as *mut c_void);
    if tmp == 0 as c_int {
        return nb_values;
    }
    return -(1 as c_int);
}
unsafe fn xmlSchemaParseUInt(
    mut str: *mut *const xmlChar,
    mut llo: *mut c_ulong,
    mut lmi: *mut c_ulong,
    mut lhi: *mut c_ulong,
) -> c_int {
    let llo_view: &mut c_ulong = unsafe { &mut *llo };
    let mut lo: c_ulong = 0 as c_ulong;
    let mut mi: c_ulong = 0 as c_ulong;
    let mut hi: c_ulong = 0 as c_ulong;
    let mut tmp: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut cur: *const xmlChar = *str;
    let mut ret: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    if !(*cur as c_int >= '0' as i32 && *cur as c_int <= '9' as i32) {
        return -(2 as c_int);
    }
    while *cur as c_int == '0' as i32 {
        cur = cur.offset(1);
    }
    tmp = cur;
    while *tmp as c_int != 0 as c_int
        && *tmp as c_int >= '0' as i32
        && *tmp as c_int <= '9' as i32
    {
        i += 1;
        tmp = tmp.offset(1);
        ret += 1;
    }
    if i > 24 as c_int {
        *str = tmp;
        return -(1 as c_int);
    }
    while i > 16 as c_int {
        let fresh12 = cur;
        cur = cur.offset(1);
        hi = hi
            .wrapping_mul(10 as c_ulong)
            .wrapping_add((*fresh12 as c_int - '0' as i32) as c_ulong);
        i -= 1;
    }
    while i > 8 as c_int {
        let fresh13 = cur;
        cur = cur.offset(1);
        mi = mi
            .wrapping_mul(10 as c_ulong)
            .wrapping_add((*fresh13 as c_int - '0' as i32) as c_ulong);
        i -= 1;
    }
    while i > 0 as c_int {
        let fresh14 = cur;
        cur = cur.offset(1);
        lo = lo
            .wrapping_mul(10 as c_ulong)
            .wrapping_add((*fresh14 as c_int - '0' as i32) as c_ulong);
        i -= 1;
    }
    *str = cur;
    *llo_view = lo;
    *lmi = mi;
    *lhi = hi;
    return ret;
}
unsafe fn xmlSchemaCheckLanguageType(mut value: *const xmlChar) -> c_int {
    let mut first: c_int = 1 as c_int;
    let mut len: c_int = 0 as c_int;
    let mut cur: *const xmlChar = value;
    if value.is_null() {
        return 0 as c_int;
    }
    while *cur.offset(0 as c_int as isize) as c_int
        != 0 as c_int
    {
        if !(*cur.offset(0 as c_int as isize) as c_int >= 'a' as i32
            && *cur.offset(0 as c_int as isize) as c_int <= 'z' as i32
            || *cur.offset(0 as c_int as isize) as c_int >= 'A' as i32
                && *cur.offset(0 as c_int as isize) as c_int
                    <= 'Z' as i32
            || *cur.offset(0 as c_int as isize) as c_int == '-' as i32
            || first == 0 as c_int
                && (0x30 as c_int
                    <= *cur.offset(0 as c_int as isize) as c_int
                    && *cur.offset(0 as c_int as isize) as c_int
                        <= 0x39 as c_int))
        {
            return 0 as c_int;
        }
        if *cur.offset(0 as c_int as isize) as c_int == '-' as i32 {
            if len < 1 as c_int || len > 8 as c_int {
                return 0 as c_int;
            }
            len = 0 as c_int;
            first = 0 as c_int;
        } else {
            len += 1;
        }
        cur = cur.offset(1);
    }
    if len < 1 as c_int || len > 8 as c_int {
        return 0 as c_int;
    }
    return 1 as c_int;
}
unsafe fn xmlSchemaValAtomicType(
    mut type_0: xmlSchemaTypePtr,
    mut value: *const xmlChar,
    mut val: *mut xmlSchemaValPtr,
    mut node: xmlNodePtr,
    mut flags: c_int,
    mut ws: xmlSchemaWhitespaceValueType,
    mut normOnTheFly: c_int,
    mut applyNorm: c_int,
    mut createStringValue: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut v: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut norm: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ret: c_int = 0 as c_int;
    if xmlSchemaTypesInitialized == 0 as c_int
        && xmlSchemaInitTypes() < 0 as c_int
    {
        return -(1 as c_int);
    }
    if type_0.is_null() {
        return -(1 as c_int);
    }
    if value.is_null() {
        value = b"\0" as *const u8 as *const c_char as *mut xmlChar;
    }
    if !val.is_null() {
        *val = ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    if flags == 0 as c_int && !value.is_null() {
        if (*type_0).builtInType != XML_SCHEMAS_STRING as c_int
            && (*type_0).builtInType != XML_SCHEMAS_ANYTYPE as c_int
            && (*type_0).builtInType != XML_SCHEMAS_ANYSIMPLETYPE as c_int
        {
            if (*type_0).builtInType == XML_SCHEMAS_NORMSTRING as c_int {
                norm = xmlSchemaWhiteSpaceReplace(value);
            } else {
                norm = xmlSchemaCollapseString(value);
            }
            if !norm.is_null() {
                value = norm;
            }
        }
    }
    match (*type_0).builtInType {
        0 => {
            current_block = 16183496351178143973;
        }
        45 | 46 => {
            if createStringValue != 0 && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_ANYSIMPLETYPE);
                if !v.is_null() {
                    (*v).value.str_0 = xmlStrdup(value);
                    *val = v;
                    current_block = 7651148207230631754;
                } else {
                    current_block = 16183496351178143973;
                }
            } else {
                current_block = 7651148207230631754;
            }
        }
        1 => {
            if normOnTheFly == 0 {
                let mut cur: *const xmlChar = value;
                if ws as c_uint
                    == XML_SCHEMA_WHITESPACE_REPLACE as c_int as c_uint
                {
                    loop {
                        if !(*cur as c_int != 0 as c_int) {
                            current_block = 6450597802325118133;
                            break;
                        }
                        if *cur as c_int == 0xd as c_int
                            || *cur as c_int == 0xa as c_int
                            || *cur as c_int == 0x9 as c_int
                        {
                            current_block = 3267341202843997150;
                            break;
                        }
                        cur = cur.offset(1);
                    }
                } else if ws as c_uint
                    == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int as c_uint
                {
                    loop {
                        if !(*cur as c_int != 0 as c_int) {
                            current_block = 6450597802325118133;
                            break;
                        }
                        if *cur as c_int == 0xd as c_int
                            || *cur as c_int == 0xa as c_int
                            || *cur as c_int == 0x9 as c_int
                        {
                            current_block = 3267341202843997150;
                            break;
                        }
                        if *cur as c_int == 0x20 as c_int {
                            cur = cur.offset(1);
                            if *cur as c_int == 0x20 as c_int {
                                current_block = 3267341202843997150;
                                break;
                            }
                        } else {
                            cur = cur.offset(1);
                        }
                    }
                } else {
                    current_block = 6450597802325118133;
                }
            } else {
                current_block = 6450597802325118133;
            }
            match current_block {
                3267341202843997150 => {}
                _ => {
                    if createStringValue != 0 && !val.is_null() {
                        if applyNorm != 0 {
                            if ws as c_uint
                                == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int
                                    as c_uint
                            {
                                norm = xmlSchemaCollapseString(value);
                            } else if ws as c_uint
                                == XML_SCHEMA_WHITESPACE_REPLACE as c_int
                                    as c_uint
                            {
                                norm = xmlSchemaWhiteSpaceReplace(value);
                            }
                            if !norm.is_null() {
                                value = norm;
                            }
                        }
                        v = xmlSchemaNewValue(XML_SCHEMAS_STRING);
                        if !v.is_null() {
                            (*v).value.str_0 = xmlStrdup(value);
                            *val = v;
                            current_block = 7651148207230631754;
                        } else {
                            current_block = 16183496351178143973;
                        }
                    } else {
                        current_block = 7651148207230631754;
                    }
                }
            }
        }
        2 => {
            if normOnTheFly != 0 {
                if applyNorm != 0 {
                    if ws as c_uint
                        == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int
                            as c_uint
                    {
                        norm = xmlSchemaCollapseString(value);
                    } else {
                        norm = xmlSchemaWhiteSpaceReplace(value);
                    }
                    if !norm.is_null() {
                        value = norm;
                    }
                }
                current_block = 8869332144787829186;
            } else {
                let mut cur_0: *const xmlChar = value;
                loop {
                    if !(*cur_0 as c_int != 0 as c_int) {
                        current_block = 8869332144787829186;
                        break;
                    }
                    if *cur_0 as c_int == 0xd as c_int
                        || *cur_0 as c_int == 0xa as c_int
                        || *cur_0 as c_int == 0x9 as c_int
                    {
                        current_block = 3267341202843997150;
                        break;
                    }
                    cur_0 = cur_0.offset(1);
                }
            }
            match current_block {
                3267341202843997150 => {}
                _ => {
                    if !val.is_null() {
                        v = xmlSchemaNewValue(XML_SCHEMAS_NORMSTRING);
                        if !v.is_null() {
                            (*v).value.str_0 = xmlStrdup(value);
                            *val = v;
                            current_block = 7651148207230631754;
                        } else {
                            current_block = 16183496351178143973;
                        }
                    } else {
                        current_block = 7651148207230631754;
                    }
                }
            }
        }
        3 => {
            let mut cur_1: *const xmlChar = value;
            let mut len: c_uint = 0;
            let mut neg: c_uint = 0;
            let mut integ: c_uint = 0;
            let mut hasLeadingZeroes: c_uint = 0;
            let mut cval: [xmlChar; 25] = [0; 25];
            let mut cptr: *mut xmlChar = &raw mut cval as *mut xmlChar;
            if cur_1.is_null() || *cur_1 as c_int == 0 as c_int {
                current_block = 3267341202843997150;
            } else {
                if normOnTheFly != 0 {
                    while *cur_1 as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *cur_1 as c_int
                            && *cur_1 as c_int <= 0xa as c_int
                        || *cur_1 as c_int == 0xd as c_int
                    {
                        cur_1 = cur_1.offset(1);
                    }
                }
                neg = 0 as c_uint;
                if *cur_1 as c_int == '-' as i32 {
                    neg = 1 as c_uint;
                    cur_1 = cur_1.offset(1);
                } else if *cur_1 as c_int == '+' as i32 {
                    cur_1 = cur_1.offset(1);
                }
                if *cur_1 as c_int == 0 as c_int {
                    current_block = 3267341202843997150;
                } else {
                    len = 0 as c_uint;
                    integ = !(0 as c_uint);
                    hasLeadingZeroes = 0 as c_uint;
                    while *cur_1 as c_int == '0' as i32 {
                        cur_1 = cur_1.offset(1);
                        hasLeadingZeroes = 1 as c_uint;
                    }
                    if *cur_1 as c_int != 0 as c_int {
                        loop {
                            if *cur_1 as c_int >= '0' as i32
                                && *cur_1 as c_int <= '9' as i32
                            {
                                let fresh0 = cur_1;
                                cur_1 = cur_1.offset(1);
                                let fresh1 = cptr;
                                cptr = cptr.offset(1);
                                *fresh1 = *fresh0;
                                len = len.wrapping_add(1);
                                if !(len < 24 as c_uint) {
                                    current_block = 5265702136860997526;
                                    break;
                                }
                            } else {
                                if !(*cur_1 as c_int == '.' as i32) {
                                    current_block = 5265702136860997526;
                                    break;
                                }
                                cur_1 = cur_1.offset(1);
                                integ = len;
                                while *cur_1 as c_int >= '0' as i32
                                    && *cur_1 as c_int <= '9' as i32
                                {
                                    let fresh2 = cur_1;
                                    cur_1 = cur_1.offset(1);
                                    let fresh3 = cptr;
                                    cptr = cptr.offset(1);
                                    *fresh3 = *fresh2;
                                    len = len.wrapping_add(1);
                                    if !(len < 24 as c_uint) {
                                        break;
                                    }
                                }
                                if len == 0 as c_uint && hasLeadingZeroes == 0 {
                                    current_block = 3267341202843997150;
                                    break;
                                } else {
                                    current_block = 5265702136860997526;
                                    break;
                                }
                            }
                        }
                    } else {
                        current_block = 5265702136860997526;
                    }
                    match current_block {
                        3267341202843997150 => {}
                        _ => {
                            if normOnTheFly != 0 {
                                while *cur_1 as c_int == 0x20 as c_int
                                    || 0x9 as c_int <= *cur_1 as c_int
                                        && *cur_1 as c_int <= 0xa as c_int
                                    || *cur_1 as c_int == 0xd as c_int
                                {
                                    cur_1 = cur_1.offset(1);
                                }
                            }
                            if *cur_1 as c_int != 0 as c_int {
                                current_block = 3267341202843997150;
                            } else {
                                if !val.is_null() {
                                    v = xmlSchemaNewValue(XML_SCHEMAS_DECIMAL);
                                    if !v.is_null() {
                                        if len != 0 as c_uint {
                                            if integ != !(0 as c_uint) {
                                                while len != integ
                                                    && *cptr
                                                        .offset(-(1 as c_int as isize))
                                                        as c_int
                                                        == '0' as i32
                                                {
                                                    cptr = cptr.offset(-1);
                                                    len = len.wrapping_sub(1);
                                                }
                                            }
                                            if len != 0 as c_uint {
                                                *cptr = 0 as xmlChar;
                                                cptr = &raw mut cval as *mut xmlChar;
                                                xmlSchemaParseUInt(
                                                    &raw mut cptr as *mut *const xmlChar,
                                                    &raw mut (*v).value.decimal.lo,
                                                    &raw mut (*v).value.decimal.mi,
                                                    &raw mut (*v).value.decimal.hi,
                                                );
                                            }
                                        }
                                        (*v).value.decimal.set_sign(neg as c_uint);
                                        if len == 0 as c_uint {
                                            (*v).value.decimal.set_total(
                                                1 as c_uint as c_uint,
                                            );
                                        } else {
                                            (*v).value
                                                .decimal
                                                .set_total(len as c_uint);
                                            if integ == !(0 as c_uint) {
                                                (*v).value.decimal.set_frac(
                                                    0 as c_uint as c_uint,
                                                );
                                            } else {
                                                (*v).value
                                                    .decimal
                                                    .set_frac(len.wrapping_sub(integ)
                                                        as c_uint);
                                            }
                                        }
                                        *val = v;
                                    }
                                }
                                current_block = 7651148207230631754;
                            }
                        }
                    }
                }
            }
        }
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
            ret = xmlSchemaValidateDates(
                (*type_0).builtInType as xmlSchemaValType,
                value,
                val,
                normOnTheFly,
            );
            current_block = 9421093467022454731;
        }
        12 => {
            ret = xmlSchemaValidateDuration(type_0, value, val, normOnTheFly);
            current_block = 9421093467022454731;
        }
        13 | 14 => {
            let mut cur_2: *const xmlChar = value;
            let mut neg_0: c_int = 0 as c_int;
            let mut digits_before: c_int = 0 as c_int;
            let mut digits_after: c_int = 0 as c_int;
            if normOnTheFly != 0 {
                while *cur_2 as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *cur_2 as c_int
                        && *cur_2 as c_int <= 0xa as c_int
                    || *cur_2 as c_int == 0xd as c_int
                {
                    cur_2 = cur_2.offset(1);
                }
            }
            if *cur_2.offset(0 as c_int as isize) as c_int == 'N' as i32
                && *cur_2.offset(1 as c_int as isize) as c_int
                    == 'a' as i32
                && *cur_2.offset(2 as c_int as isize) as c_int
                    == 'N' as i32
            {
                cur_2 = cur_2.offset(3 as c_int as isize);
                if *cur_2 as c_int != 0 as c_int {
                    current_block = 3267341202843997150;
                } else if !val.is_null() {
                    if type_0 == xmlSchemaTypeFloatDef {
                        v = xmlSchemaNewValue(XML_SCHEMAS_FLOAT);
                        if !v.is_null() {
                            (*v).value.f = xmlXPathNAN as c_float;
                            current_block = 10784681114964964746;
                        } else {
                            xmlSchemaFreeValue(v);
                            current_block = 16183496351178143973;
                        }
                    } else {
                        v = xmlSchemaNewValue(XML_SCHEMAS_DOUBLE);
                        if !v.is_null() {
                            (*v).value.d = xmlXPathNAN;
                            current_block = 10784681114964964746;
                        } else {
                            xmlSchemaFreeValue(v);
                            current_block = 16183496351178143973;
                        }
                    }
                    match current_block {
                        16183496351178143973 => {}
                        _ => {
                            *val = v;
                            current_block = 7651148207230631754;
                        }
                    }
                } else {
                    current_block = 7651148207230631754;
                }
            } else {
                if *cur_2 as c_int == '-' as i32 {
                    neg_0 = 1 as c_int;
                    cur_2 = cur_2.offset(1);
                }
                if *cur_2.offset(0 as c_int as isize) as c_int
                    == 'I' as i32
                    && *cur_2.offset(1 as c_int as isize) as c_int
                        == 'N' as i32
                    && *cur_2.offset(2 as c_int as isize) as c_int
                        == 'F' as i32
                {
                    cur_2 = cur_2.offset(3 as c_int as isize);
                    if *cur_2 as c_int != 0 as c_int {
                        current_block = 3267341202843997150;
                    } else if !val.is_null() {
                        if type_0 == xmlSchemaTypeFloatDef {
                            v = xmlSchemaNewValue(XML_SCHEMAS_FLOAT);
                            if !v.is_null() {
                                if neg_0 != 0 {
                                    (*v).value.f = xmlXPathNINF as c_float;
                                } else {
                                    (*v).value.f = xmlXPathPINF as c_float;
                                }
                                current_block = 15908231092227701503;
                            } else {
                                xmlSchemaFreeValue(v);
                                current_block = 16183496351178143973;
                            }
                        } else {
                            v = xmlSchemaNewValue(XML_SCHEMAS_DOUBLE);
                            if !v.is_null() {
                                if neg_0 != 0 {
                                    (*v).value.d = xmlXPathNINF;
                                } else {
                                    (*v).value.d = xmlXPathPINF;
                                }
                                current_block = 15908231092227701503;
                            } else {
                                xmlSchemaFreeValue(v);
                                current_block = 16183496351178143973;
                            }
                        }
                        match current_block {
                            16183496351178143973 => {}
                            _ => {
                                *val = v;
                                current_block = 7651148207230631754;
                            }
                        }
                    } else {
                        current_block = 7651148207230631754;
                    }
                } else {
                    if neg_0 == 0 as c_int
                        && *cur_2 as c_int == '+' as i32
                    {
                        cur_2 = cur_2.offset(1);
                    }
                    if *cur_2.offset(0 as c_int as isize) as c_int
                        == 0 as c_int
                        || *cur_2.offset(0 as c_int as isize) as c_int
                            == '+' as i32
                        || *cur_2.offset(0 as c_int as isize) as c_int
                            == '-' as i32
                    {
                        current_block = 3267341202843997150;
                    } else {
                        while *cur_2 as c_int >= '0' as i32
                            && *cur_2 as c_int <= '9' as i32
                        {
                            cur_2 = cur_2.offset(1);
                            digits_before += 1;
                        }
                        if *cur_2 as c_int == '.' as i32 {
                            cur_2 = cur_2.offset(1);
                            while *cur_2 as c_int >= '0' as i32
                                && *cur_2 as c_int <= '9' as i32
                            {
                                cur_2 = cur_2.offset(1);
                                digits_after += 1;
                            }
                        }
                        if digits_before == 0 as c_int
                            && digits_after == 0 as c_int
                        {
                            current_block = 3267341202843997150;
                        } else {
                            if *cur_2 as c_int == 'e' as i32
                                || *cur_2 as c_int == 'E' as i32
                            {
                                cur_2 = cur_2.offset(1);
                                if *cur_2 as c_int == '-' as i32
                                    || *cur_2 as c_int == '+' as i32
                                {
                                    cur_2 = cur_2.offset(1);
                                }
                                while *cur_2 as c_int >= '0' as i32
                                    && *cur_2 as c_int <= '9' as i32
                                {
                                    cur_2 = cur_2.offset(1);
                                }
                            }
                            if normOnTheFly != 0 {
                                while *cur_2 as c_int == 0x20 as c_int
                                    || 0x9 as c_int <= *cur_2 as c_int
                                        && *cur_2 as c_int <= 0xa as c_int
                                    || *cur_2 as c_int == 0xd as c_int
                                {
                                    cur_2 = cur_2.offset(1);
                                }
                            }
                            if *cur_2 as c_int != 0 as c_int {
                                current_block = 3267341202843997150;
                            } else if !val.is_null() {
                                if type_0 == xmlSchemaTypeFloatDef {
                                    v = xmlSchemaNewValue(XML_SCHEMAS_FLOAT);
                                    if !v.is_null() {
                                        if sscanf(
                                            value as *const c_char,
                                            b"%f\0" as *const u8 as *const c_char,
                                            &raw mut (*v).value.f,
                                        ) == 1 as c_int
                                        {
                                            *val = v;
                                            current_block = 7651148207230631754;
                                        } else {
                                            xmlSchemaFreeValue(v);
                                            current_block = 3267341202843997150;
                                        }
                                    } else {
                                        current_block = 16183496351178143973;
                                    }
                                } else {
                                    v = xmlSchemaNewValue(XML_SCHEMAS_DOUBLE);
                                    if !v.is_null() {
                                        if sscanf(
                                            value as *const c_char,
                                            b"%lf\0" as *const u8 as *const c_char,
                                            &raw mut (*v).value.d,
                                        ) == 1 as c_int
                                        {
                                            *val = v;
                                            current_block = 7651148207230631754;
                                        } else {
                                            xmlSchemaFreeValue(v);
                                            current_block = 3267341202843997150;
                                        }
                                    } else {
                                        current_block = 16183496351178143973;
                                    }
                                }
                            } else {
                                current_block = 7651148207230631754;
                            }
                        }
                    }
                }
            }
        }
        15 => {
            let mut cur_3: *const xmlChar = value;
            if normOnTheFly != 0 {
                while *cur_3 as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *cur_3 as c_int
                        && *cur_3 as c_int <= 0xa as c_int
                    || *cur_3 as c_int == 0xd as c_int
                {
                    cur_3 = cur_3.offset(1);
                }
                if *cur_3 as c_int == '0' as i32 {
                    ret = 0 as c_int;
                    cur_3 = cur_3.offset(1);
                    current_block = 13114814261106982490;
                } else if *cur_3 as c_int == '1' as i32 {
                    ret = 1 as c_int;
                    cur_3 = cur_3.offset(1);
                    current_block = 13114814261106982490;
                } else if *cur_3 as c_int == 't' as i32 {
                    cur_3 = cur_3.offset(1);
                    let fresh4 = cur_3;
                    cur_3 = cur_3.offset(1);
                    if *fresh4 as c_int == 'r' as i32
                        && {
                            let fresh5 = cur_3;
                            cur_3 = cur_3.offset(1);
                            *fresh5 as c_int == 'u' as i32
                        }
                        && {
                            let fresh6 = cur_3;
                            cur_3 = cur_3.offset(1);
                            *fresh6 as c_int == 'e' as i32
                        }
                    {
                        ret = 1 as c_int;
                        current_block = 13114814261106982490;
                    } else {
                        current_block = 3267341202843997150;
                    }
                } else if *cur_3 as c_int == 'f' as i32 {
                    cur_3 = cur_3.offset(1);
                    let fresh7 = cur_3;
                    cur_3 = cur_3.offset(1);
                    if *fresh7 as c_int == 'a' as i32
                        && {
                            let fresh8 = cur_3;
                            cur_3 = cur_3.offset(1);
                            *fresh8 as c_int == 'l' as i32
                        }
                        && {
                            let fresh9 = cur_3;
                            cur_3 = cur_3.offset(1);
                            *fresh9 as c_int == 's' as i32
                        }
                        && {
                            let fresh10 = cur_3;
                            cur_3 = cur_3.offset(1);
                            *fresh10 as c_int == 'e' as i32
                        }
                    {
                        ret = 0 as c_int;
                        current_block = 13114814261106982490;
                    } else {
                        current_block = 3267341202843997150;
                    }
                } else {
                    current_block = 3267341202843997150;
                }
                match current_block {
                    3267341202843997150 => {}
                    _ => {
                        if *cur_3 as c_int != 0 as c_int {
                            while *cur_3 as c_int == 0x20 as c_int
                                || 0x9 as c_int <= *cur_3 as c_int
                                    && *cur_3 as c_int <= 0xa as c_int
                                || *cur_3 as c_int == 0xd as c_int
                            {
                                cur_3 = cur_3.offset(1);
                            }
                            if *cur_3 as c_int != 0 as c_int {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 15455430299222214173;
                            }
                        } else {
                            current_block = 15455430299222214173;
                        }
                    }
                }
            } else if *cur_3.offset(0 as c_int as isize) as c_int
                == '0' as i32
                && *cur_3.offset(1 as c_int as isize) as c_int
                    == 0 as c_int
            {
                ret = 0 as c_int;
                current_block = 15455430299222214173;
            } else if *cur_3.offset(0 as c_int as isize) as c_int
                == '1' as i32
                && *cur_3.offset(1 as c_int as isize) as c_int
                    == 0 as c_int
            {
                ret = 1 as c_int;
                current_block = 15455430299222214173;
            } else if *cur_3.offset(0 as c_int as isize) as c_int
                == 't' as i32
                && *cur_3.offset(1 as c_int as isize) as c_int
                    == 'r' as i32
                && *cur_3.offset(2 as c_int as isize) as c_int
                    == 'u' as i32
                && *cur_3.offset(3 as c_int as isize) as c_int
                    == 'e' as i32
                && *cur_3.offset(4 as c_int as isize) as c_int
                    == 0 as c_int
            {
                ret = 1 as c_int;
                current_block = 15455430299222214173;
            } else if *cur_3.offset(0 as c_int as isize) as c_int
                == 'f' as i32
                && *cur_3.offset(1 as c_int as isize) as c_int
                    == 'a' as i32
                && *cur_3.offset(2 as c_int as isize) as c_int
                    == 'l' as i32
                && *cur_3.offset(3 as c_int as isize) as c_int
                    == 's' as i32
                && *cur_3.offset(4 as c_int as isize) as c_int
                    == 'e' as i32
                && *cur_3.offset(5 as c_int as isize) as c_int
                    == 0 as c_int
            {
                ret = 0 as c_int;
                current_block = 15455430299222214173;
            } else {
                current_block = 3267341202843997150;
            }
            match current_block {
                3267341202843997150 => {}
                _ => {
                    if !val.is_null() {
                        v = xmlSchemaNewValue(XML_SCHEMAS_BOOLEAN);
                        if !v.is_null() {
                            (*v).value.b = ret;
                            *val = v;
                            current_block = 7651148207230631754;
                        } else {
                            current_block = 16183496351178143973;
                        }
                    } else {
                        current_block = 7651148207230631754;
                    }
                }
            }
        }
        16 => {
            let mut cur_4: *const xmlChar = value;
            if normOnTheFly == 0 {
                loop {
                    if !(*cur_4 as c_int != 0 as c_int) {
                        current_block = 5015379258158208590;
                        break;
                    }
                    if *cur_4 as c_int == 0xd as c_int
                        || *cur_4 as c_int == 0xa as c_int
                        || *cur_4 as c_int == 0x9 as c_int
                    {
                        current_block = 3267341202843997150;
                        break;
                    }
                    if *cur_4 as c_int == ' ' as i32 {
                        cur_4 = cur_4.offset(1);
                        if *cur_4 as c_int == 0 as c_int {
                            current_block = 3267341202843997150;
                            break;
                        }
                        if *cur_4 as c_int == ' ' as i32 {
                            current_block = 3267341202843997150;
                            break;
                        }
                    } else {
                        cur_4 = cur_4.offset(1);
                    }
                }
            } else {
                current_block = 5015379258158208590;
            }
            match current_block {
                3267341202843997150 => {}
                _ => {
                    if !val.is_null() {
                        v = xmlSchemaNewValue(XML_SCHEMAS_TOKEN);
                        if !v.is_null() {
                            (*v).value.str_0 = xmlStrdup(value);
                            *val = v;
                            current_block = 7651148207230631754;
                        } else {
                            current_block = 16183496351178143973;
                        }
                    } else {
                        current_block = 7651148207230631754;
                    }
                }
            }
        }
        17 => {
            if norm.is_null() && normOnTheFly != 0 {
                norm = xmlSchemaCollapseString(value);
                if !norm.is_null() {
                    value = norm;
                }
            }
            if xmlSchemaCheckLanguageType(value) == 1 as c_int {
                if !val.is_null() {
                    v = xmlSchemaNewValue(XML_SCHEMAS_LANGUAGE);
                    if !v.is_null() {
                        (*v).value.str_0 = xmlStrdup(value);
                        *val = v;
                        current_block = 7651148207230631754;
                    } else {
                        current_block = 16183496351178143973;
                    }
                } else {
                    current_block = 7651148207230631754;
                }
            } else {
                current_block = 3267341202843997150;
            }
        }
        18 => {
            if xmlValidateNMToken(value, 1 as c_int) == 0 as c_int {
                if !val.is_null() {
                    v = xmlSchemaNewValue(XML_SCHEMAS_NMTOKEN);
                    if !v.is_null() {
                        (*v).value.str_0 = xmlStrdup(value);
                        *val = v;
                        current_block = 7651148207230631754;
                    } else {
                        current_block = 16183496351178143973;
                    }
                } else {
                    current_block = 7651148207230631754;
                }
            } else {
                current_block = 3267341202843997150;
            }
        }
        19 => {
            ret = xmlSchemaValAtomicListNode(xmlSchemaTypeNmtokenDef, value, val, node);
            if ret > 0 as c_int {
                ret = 0 as c_int;
            } else {
                ret = 1 as c_int;
            }
            current_block = 9421093467022454731;
        }
        20 => {
            ret = xmlValidateName(value, 1 as c_int);
            if ret == 0 as c_int && !val.is_null() && !value.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_NAME);
                if !v.is_null() {
                    let mut start: *const xmlChar = value;
                    let mut end: *const xmlChar = ::core::ptr::null::<xmlChar>();
                    while *start as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *start as c_int
                            && *start as c_int <= 0xa as c_int
                        || *start as c_int == 0xd as c_int
                    {
                        start = start.offset(1);
                    }
                    end = start;
                    while *end as c_int != 0 as c_int
                        && !(*end as c_int == 0x20 as c_int
                            || 0x9 as c_int <= *end as c_int
                                && *end as c_int <= 0xa as c_int
                            || *end as c_int == 0xd as c_int)
                    {
                        end = end.offset(1);
                    }
                    (*v).value.str_0 = xmlStrndup(
                        start,
                        end.offset_from(start) as c_long as c_int,
                    );
                    *val = v;
                    current_block = 9421093467022454731;
                } else {
                    current_block = 16183496351178143973;
                }
            } else {
                current_block = 9421093467022454731;
            }
        }
        21 => {
            let mut uri: *const xmlChar = ::core::ptr::null::<xmlChar>();
            let mut local: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            ret = xmlValidateQName(value, 1 as c_int);
            if ret != 0 as c_int {
                current_block = 9421093467022454731;
            } else {
                if !node.is_null() {
                    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                    local = xmlSplitQName2(value, &raw mut prefix);
                    ns = xmlSearchNs((*node).doc as xmlDocPtr, node, prefix);
                    if ns.is_null() && !prefix.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            prefix as *mut c_void,
                        );
                        if !local.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                local as *mut c_void,
                            );
                        }
                        current_block = 3267341202843997150;
                    } else {
                        if !ns.is_null() {
                            uri = (*ns).href;
                        }
                        if !prefix.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                prefix as *mut c_void,
                            );
                        }
                        current_block = 3893156993890508670;
                    }
                } else {
                    current_block = 3893156993890508670;
                }
                match current_block {
                    3267341202843997150 => {}
                    _ => {
                        if !val.is_null() {
                            v = xmlSchemaNewValue(XML_SCHEMAS_QNAME);
                            if v.is_null() {
                                if !local.is_null() {
                                    xmlFree.expect("non-null function pointer")(
                                        local as *mut c_void,
                                    );
                                }
                                current_block = 16183496351178143973;
                            } else {
                                if !local.is_null() {
                                    (*v).value.qname.name = local;
                                } else {
                                    (*v).value.qname.name = xmlStrdup(value);
                                }
                                if !uri.is_null() {
                                    (*v).value.qname.uri = xmlStrdup(uri);
                                }
                                *val = v;
                                current_block = 9421093467022454731;
                            }
                        } else {
                            if !local.is_null() {
                                xmlFree.expect("non-null function pointer")(
                                    local as *mut c_void,
                                );
                            }
                            current_block = 9421093467022454731;
                        }
                    }
                }
            }
        }
        22 => {
            ret = xmlValidateNCName(value, 1 as c_int);
            if ret == 0 as c_int && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_NCNAME);
                if !v.is_null() {
                    (*v).value.str_0 = xmlStrdup(value);
                    *val = v;
                    current_block = 9421093467022454731;
                } else {
                    current_block = 16183496351178143973;
                }
            } else {
                current_block = 9421093467022454731;
            }
        }
        23 => {
            ret = xmlValidateNCName(value, 1 as c_int);
            if ret == 0 as c_int && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_ID);
                if !v.is_null() {
                    (*v).value.str_0 = xmlStrdup(value);
                    *val = v;
                    current_block = 5102396516157810314;
                } else {
                    current_block = 16183496351178143973;
                }
            } else {
                current_block = 5102396516157810314;
            }
            match current_block {
                16183496351178143973 => {}
                _ => {
                    if ret == 0 as c_int
                        && !node.is_null()
                        && (*node).type_0 as c_uint
                            == XML_ATTRIBUTE_NODE as c_int as c_uint
                    {
                        let mut attr: xmlAttrPtr = node as xmlAttrPtr;
                        if (*attr).atype as c_uint
                            != XML_ATTRIBUTE_ID as c_int as c_uint
                        {
                            let mut res: xmlIDPtr = ::core::ptr::null_mut::<xmlID>();
                            let mut strip: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                            strip = xmlSchemaStrip(value);
                            if !strip.is_null() {
                                res = xmlAddID(
                                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                                    (*node).doc as xmlDocPtr,
                                    strip,
                                    attr,
                                );
                                xmlFree.expect("non-null function pointer")(
                                    strip as *mut c_void,
                                );
                            } else {
                                res = xmlAddID(
                                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                                    (*node).doc as xmlDocPtr,
                                    value,
                                    attr,
                                );
                            }
                            if res.is_null() {
                                ret = 2 as c_int;
                            } else {
                                (*attr).atype = XML_ATTRIBUTE_ID;
                            }
                        }
                    }
                    current_block = 9421093467022454731;
                }
            }
        }
        24 => {
            ret = xmlValidateNCName(value, 1 as c_int);
            if ret == 0 as c_int && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_IDREF);
                if v.is_null() {
                    current_block = 16183496351178143973;
                } else {
                    (*v).value.str_0 = xmlStrdup(value);
                    *val = v;
                    current_block = 10447688102407413348;
                }
            } else {
                current_block = 10447688102407413348;
            }
            match current_block {
                16183496351178143973 => {}
                _ => {
                    if ret == 0 as c_int
                        && !node.is_null()
                        && (*node).type_0 as c_uint
                            == XML_ATTRIBUTE_NODE as c_int as c_uint
                    {
                        let mut attr_0: xmlAttrPtr = node as xmlAttrPtr;
                        let mut strip_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        strip_0 = xmlSchemaStrip(value);
                        if !strip_0.is_null() {
                            xmlAddRef(
                                ::core::ptr::null_mut::<xmlValidCtxt>(),
                                (*node).doc as xmlDocPtr,
                                strip_0,
                                attr_0,
                            );
                            xmlFree.expect("non-null function pointer")(
                                strip_0 as *mut c_void,
                            );
                        } else {
                            xmlAddRef(
                                ::core::ptr::null_mut::<xmlValidCtxt>(),
                                (*node).doc as xmlDocPtr,
                                value,
                                attr_0,
                            );
                        }
                        (*attr_0).atype = XML_ATTRIBUTE_IDREF;
                    }
                    current_block = 9421093467022454731;
                }
            }
        }
        25 => {
            ret = xmlSchemaValAtomicListNode(xmlSchemaTypeIdrefDef, value, val, node);
            if ret < 0 as c_int {
                ret = 2 as c_int;
            } else {
                ret = 0 as c_int;
            }
            if ret == 0 as c_int
                && !node.is_null()
                && (*node).type_0 as c_uint
                    == XML_ATTRIBUTE_NODE as c_int as c_uint
            {
                let mut attr_1: xmlAttrPtr = node as xmlAttrPtr;
                (*attr_1).atype = XML_ATTRIBUTE_IDREFS;
            }
            current_block = 9421093467022454731;
        }
        26 => {
            let mut strip_1: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            ret = xmlValidateNCName(value, 1 as c_int);
            if node.is_null() || (*node).doc.is_null() {
                ret = 3 as c_int;
            }
            if ret == 0 as c_int {
                let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
                strip_1 = xmlSchemaStrip(value);
                if !strip_1.is_null() {
                    ent = xmlGetDocEntity((*node).doc, strip_1);
                    xmlFree.expect("non-null function pointer")(
                        strip_1 as *mut c_void,
                    );
                } else {
                    ent = xmlGetDocEntity((*node).doc, value);
                }
                if ent.is_null()
                    || (*ent).etype as c_uint
                        != XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as c_int
                            as c_uint
                {
                    ret = 4 as c_int;
                }
            }
            if ret == 0 as c_int && !val.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlschemastypes.c\0"
                        as *const u8 as *const c_char,
                    3049 as c_int,
                );
            }
            if ret == 0 as c_int
                && !node.is_null()
                && (*node).type_0 as c_uint
                    == XML_ATTRIBUTE_NODE as c_int as c_uint
            {
                let mut attr_2: xmlAttrPtr = node as xmlAttrPtr;
                (*attr_2).atype = XML_ATTRIBUTE_ENTITY;
            }
            current_block = 9421093467022454731;
        }
        27 => {
            if node.is_null() || (*node).doc.is_null() {
                if !norm.is_null() {
                    xmlFree.expect("non-null function pointer")(norm as *mut c_void);
                }
                return 3 as c_int;
            } else {
                ret = xmlSchemaValAtomicListNode(xmlSchemaTypeEntityDef, value, val, node);
                if ret <= 0 as c_int {
                    ret = 1 as c_int;
                } else {
                    ret = 0 as c_int;
                }
                if ret == 0 as c_int
                    && !node.is_null()
                    && (*node).type_0 as c_uint
                        == XML_ATTRIBUTE_NODE as c_int as c_uint
                {
                    let mut attr_3: xmlAttrPtr = node as xmlAttrPtr;
                    (*attr_3).atype = XML_ATTRIBUTE_ENTITIES;
                }
            }
            current_block = 9421093467022454731;
        }
        28 => {
            let mut uri_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut local_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            ret = xmlValidateQName(value, 1 as c_int);
            if ret == 0 as c_int && !node.is_null() {
                let mut prefix_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                local_0 = xmlSplitQName2(value, &raw mut prefix_0);
                if !prefix_0.is_null() {
                    let mut ns_0: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                    ns_0 = xmlSearchNs((*node).doc as xmlDocPtr, node, prefix_0);
                    if ns_0.is_null() {
                        ret = 1 as c_int;
                    } else if !val.is_null() {
                        uri_0 = xmlStrdup((*ns_0).href);
                    }
                }
                if !local_0.is_null() && (val.is_null() || ret != 0 as c_int) {
                    xmlFree.expect("non-null function pointer")(
                        local_0 as *mut c_void,
                    );
                }
                if !prefix_0.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        prefix_0 as *mut c_void,
                    );
                }
            }
            if node.is_null() || (*node).doc.is_null() {
                ret = 3 as c_int;
            }
            if ret == 0 as c_int {
                ret = xmlValidateNotationUse(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    (*node).doc as xmlDocPtr,
                    value,
                );
                if ret == 1 as c_int {
                    ret = 0 as c_int;
                } else {
                    ret = 1 as c_int;
                }
            }
            if ret == 0 as c_int && !val.is_null() {
                v = xmlSchemaNewValue(XML_SCHEMAS_NOTATION);
                if !v.is_null() {
                    if !local_0.is_null() {
                        (*v).value.qname.name = local_0;
                    } else {
                        (*v).value.qname.name = xmlStrdup(value);
                    }
                    if !uri_0.is_null() {
                        (*v).value.qname.uri = uri_0;
                    }
                    *val = v;
                    current_block = 9421093467022454731;
                } else {
                    if !local_0.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            local_0 as *mut c_void,
                        );
                    }
                    if !uri_0.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            uri_0 as *mut c_void,
                        );
                    }
                    current_block = 16183496351178143973;
                }
            } else {
                current_block = 9421093467022454731;
            }
        }
        29 => {
            if *value as c_int != 0 as c_int {
                let mut uri_1: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
                let mut tmpval: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                let mut cur_5: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                if norm.is_null() && normOnTheFly != 0 {
                    norm = xmlSchemaCollapseString(value);
                    if !norm.is_null() {
                        value = norm;
                    }
                }
                tmpval = xmlStrdup(value);
                if tmpval.is_null() {
                    current_block = 16183496351178143973;
                } else {
                    cur_5 = tmpval;
                    while *cur_5 != 0 {
                        if (*cur_5 as c_int) < 32 as c_int
                            || *cur_5 as c_int >= 127 as c_int
                            || *cur_5 as c_int == ' ' as i32
                            || *cur_5 as c_int == '<' as i32
                            || *cur_5 as c_int == '>' as i32
                            || *cur_5 as c_int == '"' as i32
                            || *cur_5 as c_int == '{' as i32
                            || *cur_5 as c_int == '}' as i32
                            || *cur_5 as c_int == '|' as i32
                            || *cur_5 as c_int == '\\' as i32
                            || *cur_5 as c_int == '^' as i32
                            || *cur_5 as c_int == '`' as i32
                            || *cur_5 as c_int == '\'' as i32
                        {
                            *cur_5 = '_' as i32 as xmlChar;
                        }
                        cur_5 = cur_5.offset(1);
                    }
                    uri_1 = xmlParseURI(tmpval as *const c_char);
                    xmlFree.expect("non-null function pointer")(tmpval as *mut c_void);
                    if uri_1.is_null() {
                        current_block = 3267341202843997150;
                    } else {
                        xmlFreeURI(uri_1);
                        current_block = 6031918995612662703;
                    }
                }
            } else {
                current_block = 6031918995612662703;
            }
            match current_block {
                16183496351178143973 => {}
                3267341202843997150 => {}
                _ => {
                    if !val.is_null() {
                        v = xmlSchemaNewValue(XML_SCHEMAS_ANYURI);
                        if v.is_null() {
                            current_block = 16183496351178143973;
                        } else {
                            (*v).value.str_0 = xmlStrdup(value);
                            *val = v;
                            current_block = 7651148207230631754;
                        }
                    } else {
                        current_block = 7651148207230631754;
                    }
                }
            }
        }
        43 => {
            let mut cur_6: *const xmlChar = value;
            let mut start_0: *const xmlChar = ::core::ptr::null::<xmlChar>();
            let mut base: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut total: c_int = 0;
            let mut i: c_int = 0 as c_int;
            if cur_6.is_null() {
                current_block = 3267341202843997150;
            } else {
                if normOnTheFly != 0 {
                    while *cur_6 as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *cur_6 as c_int
                            && *cur_6 as c_int <= 0xa as c_int
                        || *cur_6 as c_int == 0xd as c_int
                    {
                        cur_6 = cur_6.offset(1);
                    }
                }
                start_0 = cur_6;
                while *cur_6 as c_int >= '0' as i32
                    && *cur_6 as c_int <= '9' as i32
                    || *cur_6 as c_int >= 'A' as i32
                        && *cur_6 as c_int <= 'F' as i32
                    || *cur_6 as c_int >= 'a' as i32
                        && *cur_6 as c_int <= 'f' as i32
                {
                    i += 1;
                    cur_6 = cur_6.offset(1);
                }
                if normOnTheFly != 0 {
                    while *cur_6 as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *cur_6 as c_int
                            && *cur_6 as c_int <= 0xa as c_int
                        || *cur_6 as c_int == 0xd as c_int
                    {
                        cur_6 = cur_6.offset(1);
                    }
                }
                if *cur_6 as c_int != 0 as c_int {
                    current_block = 3267341202843997150;
                } else if i % 2 as c_int != 0 as c_int {
                    current_block = 3267341202843997150;
                } else if !val.is_null() {
                    v = xmlSchemaNewValue(XML_SCHEMAS_HEXBINARY);
                    if v.is_null() {
                        current_block = 16183496351178143973;
                    } else {
                        cur_6 = xmlStrndup(start_0, i);
                        if cur_6.is_null() {
                            xmlSchemaTypeErrMemory(
                                node,
                                b"allocating hexbin data\0" as *const u8
                                    as *const c_char,
                            );
                            xmlFree.expect("non-null function pointer")(
                                v as *mut c_void,
                            );
                            current_block = 3267341202843997150;
                        } else {
                            total = i / 2 as c_int;
                            base = cur_6 as *mut xmlChar;
                            loop {
                                let fresh11 = i;
                                i = i - 1;
                                if !(fresh11 > 0 as c_int) {
                                    break;
                                }
                                if *base as c_int >= 'a' as i32 {
                                    *base = (*base as c_int
                                        - ('a' as i32 - 'A' as i32))
                                        as xmlChar;
                                }
                                base = base.offset(1);
                            }
                            (*v).value.hex.str_0 = cur_6 as *mut xmlChar;
                            (*v).value.hex.total = total as c_uint;
                            *val = v;
                            current_block = 7651148207230631754;
                        }
                    }
                } else {
                    current_block = 7651148207230631754;
                }
            }
        }
        44 => {
            let mut cur_7: *const xmlChar = value;
            let mut base_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut total_0: c_int = 0;
            let mut i_0: c_int = 0 as c_int;
            let mut pad: c_int = 0 as c_int;
            if cur_7.is_null() {
                current_block = 3267341202843997150;
            } else {
                while *cur_7 != 0 {
                    let mut decc: c_int = 0;
                    decc = _xmlSchemaBase64Decode(*cur_7);
                    if !(decc < 0 as c_int) {
                        if !(decc < 64 as c_int) {
                            break;
                        }
                        i_0 += 1;
                    }
                    cur_7 = cur_7.offset(1);
                }
                loop {
                    if !(*cur_7 != 0) {
                        current_block = 17336923773064764666;
                        break;
                    }
                    let mut decc_0: c_int = 0;
                    decc_0 = _xmlSchemaBase64Decode(*cur_7);
                    if !(decc_0 < 0 as c_int) {
                        if decc_0 < 64 as c_int {
                            current_block = 3267341202843997150;
                            break;
                        }
                    }
                    if decc_0 == 64 as c_int {
                        pad += 1;
                    }
                    cur_7 = cur_7.offset(1);
                }
                match current_block {
                    3267341202843997150 => {}
                    _ => {
                        total_0 = 3 as c_int * (i_0 / 4 as c_int);
                        if pad == 0 as c_int {
                            if i_0 % 4 as c_int != 0 as c_int {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 816709800765564752;
                            }
                        } else if pad == 1 as c_int {
                            let mut decc_1: c_int = 0;
                            if i_0 % 4 as c_int != 3 as c_int {
                                current_block = 3267341202843997150;
                            } else {
                                decc_1 = _xmlSchemaBase64Decode(*cur_7);
                                while decc_1 < 0 as c_int
                                    || decc_1 > 63 as c_int
                                {
                                    cur_7 = cur_7.offset(-1);
                                    decc_1 = _xmlSchemaBase64Decode(*cur_7);
                                }
                                if decc_1 & !(0x3c as c_int) != 0 {
                                    current_block = 3267341202843997150;
                                } else {
                                    total_0 += 2 as c_int;
                                    current_block = 816709800765564752;
                                }
                            }
                        } else if pad == 2 as c_int {
                            let mut decc_2: c_int = 0;
                            if i_0 % 4 as c_int != 2 as c_int {
                                current_block = 3267341202843997150;
                            } else {
                                decc_2 = _xmlSchemaBase64Decode(*cur_7);
                                while decc_2 < 0 as c_int
                                    || decc_2 > 63 as c_int
                                {
                                    cur_7 = cur_7.offset(-1);
                                    decc_2 = _xmlSchemaBase64Decode(*cur_7);
                                }
                                if decc_2 & !(0x30 as c_int) != 0 {
                                    current_block = 3267341202843997150;
                                } else {
                                    total_0 += 1 as c_int;
                                    current_block = 816709800765564752;
                                }
                            }
                        } else {
                            current_block = 3267341202843997150;
                        }
                        match current_block {
                            3267341202843997150 => {}
                            _ => {
                                if !val.is_null() {
                                    v = xmlSchemaNewValue(XML_SCHEMAS_BASE64BINARY);
                                    if v.is_null() {
                                        current_block = 16183496351178143973;
                                    } else {
                                        base_0 = xmlMallocAtomic.expect("non-null function pointer")(
                                            (i_0 + pad + 1 as c_int) as size_t,
                                        )
                                            as *mut xmlChar;
                                        if base_0.is_null() {
                                            xmlSchemaTypeErrMemory(
                                                node,
                                                b"allocating base64 data\0" as *const u8
                                                    as *const c_char,
                                            );
                                            xmlFree.expect("non-null function pointer")(
                                                v as *mut c_void,
                                            );
                                            current_block = 3267341202843997150;
                                        } else {
                                            (*v).value.base64.str_0 = base_0;
                                            cur_7 = value;
                                            while *cur_7 != 0 {
                                                if _xmlSchemaBase64Decode(*cur_7)
                                                    >= 0 as c_int
                                                {
                                                    *base_0 = *cur_7;
                                                    base_0 = base_0.offset(1);
                                                }
                                                cur_7 = cur_7.offset(1);
                                            }
                                            *base_0 = 0 as xmlChar;
                                            (*v).value.base64.total =
                                                total_0 as c_uint;
                                            *val = v;
                                            current_block = 7651148207230631754;
                                        }
                                    }
                                } else {
                                    current_block = 7651148207230631754;
                                }
                            }
                        }
                    }
                }
            }
        }
        30 | 34 | 31 | 32 | 33 => {
            let mut cur_8: *const xmlChar = value;
            let mut lo: c_ulong = 0;
            let mut mi: c_ulong = 0;
            let mut hi: c_ulong = 0;
            let mut sign: c_int = 0 as c_int;
            if cur_8.is_null() {
                current_block = 3267341202843997150;
            } else {
                if normOnTheFly != 0 {
                    while *cur_8 as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *cur_8 as c_int
                            && *cur_8 as c_int <= 0xa as c_int
                        || *cur_8 as c_int == 0xd as c_int
                    {
                        cur_8 = cur_8.offset(1);
                    }
                }
                if *cur_8 as c_int == '-' as i32 {
                    sign = 1 as c_int;
                    cur_8 = cur_8.offset(1);
                } else if *cur_8 as c_int == '+' as i32 {
                    cur_8 = cur_8.offset(1);
                }
                ret = xmlSchemaParseUInt(&raw mut cur_8, &raw mut lo, &raw mut mi, &raw mut hi);
                if ret < 0 as c_int {
                    current_block = 3267341202843997150;
                } else {
                    if normOnTheFly != 0 {
                        while *cur_8 as c_int == 0x20 as c_int
                            || 0x9 as c_int <= *cur_8 as c_int
                                && *cur_8 as c_int <= 0xa as c_int
                            || *cur_8 as c_int == 0xd as c_int
                        {
                            cur_8 = cur_8.offset(1);
                        }
                    }
                    if *cur_8 as c_int != 0 as c_int {
                        current_block = 3267341202843997150;
                    } else {
                        if (*type_0).builtInType == XML_SCHEMAS_NPINTEGER as c_int {
                            if sign == 0 as c_int
                                && (hi != 0 as c_ulong
                                    || mi != 0 as c_ulong
                                    || lo != 0 as c_ulong)
                            {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 14498797008854086737;
                            }
                        } else if (*type_0).builtInType
                            == XML_SCHEMAS_PINTEGER as c_int
                        {
                            if sign == 1 as c_int {
                                current_block = 3267341202843997150;
                            } else if hi == 0 as c_ulong
                                && mi == 0 as c_ulong
                                && lo == 0 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 14498797008854086737;
                            }
                        } else if (*type_0).builtInType
                            == XML_SCHEMAS_NINTEGER as c_int
                        {
                            if sign == 0 as c_int {
                                current_block = 3267341202843997150;
                            } else if hi == 0 as c_ulong
                                && mi == 0 as c_ulong
                                && lo == 0 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 14498797008854086737;
                            }
                        } else if (*type_0).builtInType
                            == XML_SCHEMAS_NNINTEGER as c_int
                        {
                            if sign == 1 as c_int
                                && (hi != 0 as c_ulong
                                    || mi != 0 as c_ulong
                                    || lo != 0 as c_ulong)
                            {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 14498797008854086737;
                            }
                        } else {
                            current_block = 14498797008854086737;
                        }
                        match current_block {
                            3267341202843997150 => {}
                            _ => {
                                if !val.is_null() {
                                    v = xmlSchemaNewValue(
                                        (*type_0).builtInType as xmlSchemaValType,
                                    );
                                    if !v.is_null() {
                                        if ret == 0 as c_int {
                                            ret += 1;
                                        }
                                        (*v).value.decimal.lo = lo;
                                        (*v).value.decimal.mi = mi;
                                        (*v).value.decimal.hi = hi;
                                        (*v).value.decimal.set_sign(
                                            sign as c_uint as c_uint,
                                        );
                                        (*v).value.decimal.set_frac(
                                            0 as c_uint as c_uint,
                                        );
                                        (*v).value.decimal.set_total(
                                            ret as c_uint as c_uint,
                                        );
                                        *val = v;
                                    }
                                }
                                current_block = 7651148207230631754;
                            }
                        }
                    }
                }
            }
        }
        37 | 41 | 39 | 35 => {
            let mut cur_9: *const xmlChar = value;
            let mut lo_0: c_ulong = 0;
            let mut mi_0: c_ulong = 0;
            let mut hi_0: c_ulong = 0;
            let mut sign_0: c_int = 0 as c_int;
            if cur_9.is_null() {
                current_block = 3267341202843997150;
            } else {
                if normOnTheFly != 0 {
                    while *cur_9 as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *cur_9 as c_int
                            && *cur_9 as c_int <= 0xa as c_int
                        || *cur_9 as c_int == 0xd as c_int
                    {
                        cur_9 = cur_9.offset(1);
                    }
                }
                if *cur_9 as c_int == '-' as i32 {
                    sign_0 = 1 as c_int;
                    cur_9 = cur_9.offset(1);
                } else if *cur_9 as c_int == '+' as i32 {
                    cur_9 = cur_9.offset(1);
                }
                ret =
                    xmlSchemaParseUInt(&raw mut cur_9, &raw mut lo_0, &raw mut mi_0, &raw mut hi_0);
                if ret < 0 as c_int {
                    current_block = 3267341202843997150;
                } else {
                    if normOnTheFly != 0 {
                        while *cur_9 as c_int == 0x20 as c_int
                            || 0x9 as c_int <= *cur_9 as c_int
                                && *cur_9 as c_int <= 0xa as c_int
                            || *cur_9 as c_int == 0xd as c_int
                        {
                            cur_9 = cur_9.offset(1);
                        }
                    }
                    if *cur_9 as c_int != 0 as c_int {
                        current_block = 3267341202843997150;
                    } else {
                        if (*type_0).builtInType == XML_SCHEMAS_LONG as c_int {
                            if hi_0 >= 922 as c_ulong {
                                if hi_0 > 922 as c_ulong {
                                    current_block = 3267341202843997150;
                                } else if mi_0 >= 33720368 as c_ulong {
                                    if mi_0 > 33720368 as c_ulong {
                                        current_block = 3267341202843997150;
                                    } else if sign_0 == 0 as c_int
                                        && lo_0 > 54775807 as c_ulong
                                    {
                                        current_block = 3267341202843997150;
                                    } else if sign_0 == 1 as c_int
                                        && lo_0 > 54775808 as c_ulong
                                    {
                                        current_block = 3267341202843997150;
                                    } else {
                                        current_block = 17421566479416273753;
                                    }
                                } else {
                                    current_block = 17421566479416273753;
                                }
                            } else {
                                current_block = 17421566479416273753;
                            }
                        } else if (*type_0).builtInType == XML_SCHEMAS_INT as c_int {
                            if hi_0 != 0 as c_ulong {
                                current_block = 3267341202843997150;
                            } else if mi_0 >= 21 as c_ulong {
                                if mi_0 > 21 as c_ulong {
                                    current_block = 3267341202843997150;
                                } else if sign_0 == 0 as c_int
                                    && lo_0 > 47483647 as c_ulong
                                {
                                    current_block = 3267341202843997150;
                                } else if sign_0 == 1 as c_int
                                    && lo_0 > 47483648 as c_ulong
                                {
                                    current_block = 3267341202843997150;
                                } else {
                                    current_block = 17421566479416273753;
                                }
                            } else {
                                current_block = 17421566479416273753;
                            }
                        } else if (*type_0).builtInType == XML_SCHEMAS_SHORT as c_int {
                            if mi_0 != 0 as c_ulong
                                || hi_0 != 0 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else if sign_0 == 1 as c_int
                                && lo_0 > 32768 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else if sign_0 == 0 as c_int
                                && lo_0 > 32767 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 17421566479416273753;
                            }
                        } else if (*type_0).builtInType == XML_SCHEMAS_BYTE as c_int {
                            if mi_0 != 0 as c_ulong
                                || hi_0 != 0 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else if sign_0 == 1 as c_int
                                && lo_0 > 128 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else if sign_0 == 0 as c_int
                                && lo_0 > 127 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 17421566479416273753;
                            }
                        } else {
                            current_block = 17421566479416273753;
                        }
                        match current_block {
                            3267341202843997150 => {}
                            _ => {
                                if !val.is_null() {
                                    v = xmlSchemaNewValue(
                                        (*type_0).builtInType as xmlSchemaValType,
                                    );
                                    if !v.is_null() {
                                        (*v).value.decimal.lo = lo_0;
                                        (*v).value.decimal.mi = mi_0;
                                        (*v).value.decimal.hi = hi_0;
                                        (*v).value.decimal.set_sign(
                                            sign_0 as c_uint as c_uint,
                                        );
                                        (*v).value.decimal.set_frac(
                                            0 as c_uint as c_uint,
                                        );
                                        (*v).value.decimal.set_total(
                                            ret as c_uint as c_uint,
                                        );
                                        *val = v;
                                    }
                                }
                                current_block = 7651148207230631754;
                            }
                        }
                    }
                }
            }
        }
        36 | 38 | 40 | 42 => {
            let mut cur_10: *const xmlChar = value;
            let mut lo_1: c_ulong = 0;
            let mut mi_1: c_ulong = 0;
            let mut hi_1: c_ulong = 0;
            if cur_10.is_null() {
                current_block = 3267341202843997150;
            } else {
                if normOnTheFly != 0 {
                    while *cur_10 as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *cur_10 as c_int
                            && *cur_10 as c_int <= 0xa as c_int
                        || *cur_10 as c_int == 0xd as c_int
                    {
                        cur_10 = cur_10.offset(1);
                    }
                }
                ret = xmlSchemaParseUInt(
                    &raw mut cur_10,
                    &raw mut lo_1,
                    &raw mut mi_1,
                    &raw mut hi_1,
                );
                if ret < 0 as c_int {
                    current_block = 3267341202843997150;
                } else {
                    if normOnTheFly != 0 {
                        while *cur_10 as c_int == 0x20 as c_int
                            || 0x9 as c_int <= *cur_10 as c_int
                                && *cur_10 as c_int <= 0xa as c_int
                            || *cur_10 as c_int == 0xd as c_int
                        {
                            cur_10 = cur_10.offset(1);
                        }
                    }
                    if *cur_10 as c_int != 0 as c_int {
                        current_block = 3267341202843997150;
                    } else {
                        if (*type_0).builtInType == XML_SCHEMAS_ULONG as c_int {
                            if hi_1 >= 1844 as c_ulong {
                                if hi_1 > 1844 as c_ulong {
                                    current_block = 3267341202843997150;
                                } else if mi_1 >= 67440737 as c_ulong {
                                    if mi_1 > 67440737 as c_ulong {
                                        current_block = 3267341202843997150;
                                    } else if lo_1 > 9551615 as c_ulong {
                                        current_block = 3267341202843997150;
                                    } else {
                                        current_block = 2483213355175154851;
                                    }
                                } else {
                                    current_block = 2483213355175154851;
                                }
                            } else {
                                current_block = 2483213355175154851;
                            }
                        } else if (*type_0).builtInType == XML_SCHEMAS_UINT as c_int {
                            if hi_1 != 0 as c_ulong {
                                current_block = 3267341202843997150;
                            } else if mi_1 >= 42 as c_ulong {
                                if mi_1 > 42 as c_ulong {
                                    current_block = 3267341202843997150;
                                } else if lo_1 > 94967295 as c_ulong {
                                    current_block = 3267341202843997150;
                                } else {
                                    current_block = 2483213355175154851;
                                }
                            } else {
                                current_block = 2483213355175154851;
                            }
                        } else if (*type_0).builtInType == XML_SCHEMAS_USHORT as c_int
                        {
                            if mi_1 != 0 as c_ulong
                                || hi_1 != 0 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else if lo_1 > 65535 as c_ulong {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 2483213355175154851;
                            }
                        } else if (*type_0).builtInType == XML_SCHEMAS_UBYTE as c_int {
                            if mi_1 != 0 as c_ulong
                                || hi_1 != 0 as c_ulong
                            {
                                current_block = 3267341202843997150;
                            } else if lo_1 > 255 as c_ulong {
                                current_block = 3267341202843997150;
                            } else {
                                current_block = 2483213355175154851;
                            }
                        } else {
                            current_block = 2483213355175154851;
                        }
                        match current_block {
                            3267341202843997150 => {}
                            _ => {
                                if !val.is_null() {
                                    v = xmlSchemaNewValue(
                                        (*type_0).builtInType as xmlSchemaValType,
                                    );
                                    if !v.is_null() {
                                        (*v).value.decimal.lo = lo_1;
                                        (*v).value.decimal.mi = mi_1;
                                        (*v).value.decimal.hi = hi_1;
                                        (*v).value.decimal.set_sign(
                                            0 as c_uint as c_uint,
                                        );
                                        (*v).value.decimal.set_frac(
                                            0 as c_uint as c_uint,
                                        );
                                        (*v).value.decimal.set_total(
                                            ret as c_uint as c_uint,
                                        );
                                        *val = v;
                                    }
                                }
                                current_block = 7651148207230631754;
                            }
                        }
                    }
                }
            }
        }
        _ => {
            current_block = 9421093467022454731;
        }
    }
    match current_block {
        16183496351178143973 => {
            if !norm.is_null() {
                xmlFree.expect("non-null function pointer")(norm as *mut c_void);
            }
            return -(1 as c_int);
        }
        3267341202843997150 => {
            if !norm.is_null() {
                xmlFree.expect("non-null function pointer")(norm as *mut c_void);
            }
            return 1 as c_int;
        }
        7651148207230631754 => {
            if !norm.is_null() {
                xmlFree.expect("non-null function pointer")(norm as *mut c_void);
            }
            return 0 as c_int;
        }
        _ => {
            if !norm.is_null() {
                xmlFree.expect("non-null function pointer")(norm as *mut c_void);
            }
            return ret;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValPredefTypeNode(
    mut type_0: xmlSchemaTypePtr,
    mut value: *const xmlChar,
    mut val: *mut xmlSchemaValPtr,
    mut node: xmlNodePtr,
) -> c_int {
    return xmlSchemaValAtomicType(
        type_0,
        value,
        val,
        node,
        0 as c_int,
        XML_SCHEMA_WHITESPACE_UNKNOWN,
        1 as c_int,
        1 as c_int,
        0 as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValPredefTypeNodeNoNorm(
    mut type_0: xmlSchemaTypePtr,
    mut value: *const xmlChar,
    mut val: *mut xmlSchemaValPtr,
    mut node: xmlNodePtr,
) -> c_int {
    return xmlSchemaValAtomicType(
        type_0,
        value,
        val,
        node,
        1 as c_int,
        XML_SCHEMA_WHITESPACE_UNKNOWN,
        1 as c_int,
        0 as c_int,
        1 as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidatePredefinedType(
    mut type_0: xmlSchemaTypePtr,
    mut value: *const xmlChar,
    mut val: *mut xmlSchemaValPtr,
) -> c_int {
    return xmlSchemaValPredefTypeNode(type_0, value, val, ::core::ptr::null_mut::<xmlNode>());
}
fn xmlSchemaCompareDecimals(
    mut x: xmlSchemaValPtr,
    mut y: xmlSchemaValPtr,
) -> c_int { unsafe {
    let mut swp: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut order: c_int = 1 as c_int;
    let mut integx: c_int = 0;
    let mut integy: c_int = 0;
    let mut dlen: c_int = 0;
    let mut hi: c_ulong = 0;
    let mut mi: c_ulong = 0;
    let mut lo: c_ulong = 0;
    if (*x).value.decimal.sign() as c_int != 0
        && ((*x).value.decimal.lo != 0 as c_ulong
            || (*x).value.decimal.mi != 0 as c_ulong
            || (*x).value.decimal.hi != 0 as c_ulong)
    {
        if (*y).value.decimal.sign() as c_int != 0
            && ((*y).value.decimal.lo != 0 as c_ulong
                || (*y).value.decimal.mi != 0 as c_ulong
                || (*y).value.decimal.hi != 0 as c_ulong)
        {
            order = -(1 as c_int);
        } else {
            return -(1 as c_int);
        }
    } else if (*y).value.decimal.sign() as c_int != 0
        && ((*y).value.decimal.lo != 0 as c_ulong
            || (*y).value.decimal.mi != 0 as c_ulong
            || (*y).value.decimal.hi != 0 as c_ulong)
    {
        return 1 as c_int;
    }
    integx = (*x).value.decimal.total() as c_int
        - (*x).value.decimal.frac() as c_int;
    integy = (*y).value.decimal.total() as c_int
        - (*y).value.decimal.frac() as c_int;
    if integx == 1 as c_int {
        if (*x).value.decimal.lo == 0 as c_ulong {
            if integy != 1 as c_int {
                return -order;
            } else if (*y).value.decimal.lo != 0 as c_ulong {
                return -order;
            } else {
                return 0 as c_int;
            }
        }
    }
    if integy == 1 as c_int {
        if (*y).value.decimal.lo == 0 as c_ulong {
            if integx != 1 as c_int {
                return order;
            } else if (*x).value.decimal.lo != 0 as c_ulong {
                return order;
            } else {
                return 0 as c_int;
            }
        }
    }
    if integx > integy {
        return order;
    } else if integy > integx {
        return -order;
    }
    dlen = (*x).value.decimal.total() as c_int
        - (*y).value.decimal.total() as c_int;
    if dlen < 0 as c_int {
        swp = x;
        hi = (*y).value.decimal.hi;
        mi = (*y).value.decimal.mi;
        lo = (*y).value.decimal.lo;
        dlen = -dlen;
        order = -order;
    } else {
        swp = y;
        hi = (*x).value.decimal.hi;
        mi = (*x).value.decimal.mi;
        lo = (*x).value.decimal.lo;
    }
    while dlen > 8 as c_int {
        lo = mi;
        mi = hi;
        hi = 0 as c_ulong;
        dlen -= 8 as c_int;
    }
    while dlen > 0 as c_int {
        let mut rem1: c_ulong = 0;
        let mut rem2: c_ulong = 0;
        rem1 = hi
            .wrapping_rem(10 as c_ulong)
            .wrapping_mul(100000000 as c_ulong);
        hi = hi.wrapping_div(10 as c_ulong);
        rem2 = mi
            .wrapping_rem(10 as c_ulong)
            .wrapping_mul(100000000 as c_ulong);
        mi = mi
            .wrapping_add(rem1)
            .wrapping_div(10 as c_ulong);
        lo = lo
            .wrapping_add(rem2)
            .wrapping_div(10 as c_ulong);
        dlen -= 1;
    }
    if hi > (*swp).value.decimal.hi {
        return order;
    } else if hi == (*swp).value.decimal.hi {
        if mi > (*swp).value.decimal.mi {
            return order;
        } else if mi == (*swp).value.decimal.mi {
            if lo > (*swp).value.decimal.lo {
                return order;
            } else if lo == (*swp).value.decimal.lo {
                if (*x).value.decimal.total() as c_int
                    == (*y).value.decimal.total() as c_int
                {
                    return 0 as c_int;
                } else {
                    return order;
                }
            }
        }
    }
    return -order;
} }
fn xmlSchemaCompareDurations(
    mut x: xmlSchemaValPtr,
    mut y: xmlSchemaValPtr,
) -> c_int { unsafe {
    let mut carry: c_long = 0;
    let mut mon: c_long = 0;
    let mut day: c_long = 0;
    let mut sec: c_double = 0.;
    let mut invert: c_int = 1 as c_int;
    let mut xmon: c_long = 0;
    let mut xday: c_long = 0;
    let mut myear: c_long = 0;
    let mut minday: c_long = 0;
    let mut maxday: c_long = 0;
    static mut dayRange: [[c_long; 12]; 2] = [
        [
            0 as c_int as c_long,
            28 as c_int as c_long,
            59 as c_int as c_long,
            89 as c_int as c_long,
            120 as c_int as c_long,
            150 as c_int as c_long,
            181 as c_int as c_long,
            212 as c_int as c_long,
            242 as c_int as c_long,
            273 as c_int as c_long,
            303 as c_int as c_long,
            334 as c_int as c_long,
        ],
        [
            0 as c_int as c_long,
            31 as c_int as c_long,
            62 as c_int as c_long,
            92 as c_int as c_long,
            123 as c_int as c_long,
            153 as c_int as c_long,
            184 as c_int as c_long,
            215 as c_int as c_long,
            245 as c_int as c_long,
            276 as c_int as c_long,
            306 as c_int as c_long,
            337 as c_int as c_long,
        ],
    ];
    if x.is_null() || y.is_null() {
        return -(2 as c_int);
    }
    mon = (*x).value.dur.mon - (*y).value.dur.mon;
    sec = (*x).value.dur.sec - (*y).value.dur.sec;
    carry = (sec / SECS_PER_DAY as c_double) as c_long;
    sec -= carry as c_double * SECS_PER_DAY as c_double;
    day = (*x).value.dur.day - (*y).value.dur.day + carry;
    if mon == 0 as c_long {
        if day == 0 as c_long {
            if sec == 0.0f64 {
                return 0 as c_int;
            } else if sec < 0.0f64 {
                return -(1 as c_int);
            } else {
                return 1 as c_int;
            }
        } else if day < 0 as c_long {
            return -(1 as c_int);
        } else {
            return 1 as c_int;
        }
    }
    if mon > 0 as c_long {
        if day >= 0 as c_long && sec >= 0.0f64 {
            return 1 as c_int;
        } else {
            xmon = mon;
            xday = -day;
        }
    } else if day <= 0 as c_long && sec <= 0.0f64 {
        return -(1 as c_int);
    } else {
        invert = -(1 as c_int);
        xmon = -mon;
        xday = day;
    }
    myear = xmon / 12 as c_long;
    if myear == 0 as c_long {
        minday = 0 as c_long;
        maxday = 0 as c_long;
    } else {
        if myear > LONG_MAX / 366 as c_long {
            return -(2 as c_int);
        }
        maxday = 365 as c_long * myear
            + (myear + 3 as c_long) / 4 as c_long;
        minday = maxday - 1 as c_long;
    }
    xmon = xmon % 12 as c_long;
    minday += dayRange[0 as c_int as usize][xmon as usize];
    maxday += dayRange[1 as c_int as usize][xmon as usize];
    if maxday == minday && maxday == xday {
        return 0 as c_int;
    }
    if maxday < xday {
        return -invert;
    }
    if minday > xday {
        return invert;
    }
    return 2 as c_int;
} }
fn xmlSchemaDupVal(mut v: xmlSchemaValPtr) -> xmlSchemaValPtr { unsafe {
    let mut ret: xmlSchemaValPtr = xmlSchemaNewValue((*v).type_0);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    memcpy(
        ret as *mut c_void,
        v as *const c_void,
        ::core::mem::size_of::<xmlSchemaVal>() as size_t,
    );
    (*ret).next = ::core::ptr::null_mut::<_xmlSchemaVal>();
    return ret;
} }
#[inline]
pub fn xmlSchemaCopyValue(mut val: xmlSchemaValPtr) -> xmlSchemaValPtr { unsafe {
    let mut ret: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut prev: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut cur: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    while !val.is_null() {
        match (*val).type_0 as c_uint {
            45 | 25 | 27 | 19 => {
                xmlSchemaFreeValue(ret);
                return ::core::ptr::null_mut::<xmlSchemaVal>();
            }
            46 | 1 | 2 | 16 | 17 | 20 | 22 | 23 | 24 | 26 | 18 | 29 => {
                cur = xmlSchemaDupVal(val);
                if !(*val).value.str_0.is_null() {
                    (*cur).value.str_0 = xmlStrdup((*val).value.str_0);
                }
            }
            21 | 28 => {
                cur = xmlSchemaDupVal(val);
                if !(*val).value.qname.name.is_null() {
                    (*cur).value.qname.name = xmlStrdup((*val).value.qname.name);
                }
                if !(*val).value.qname.uri.is_null() {
                    (*cur).value.qname.uri = xmlStrdup((*val).value.qname.uri);
                }
            }
            43 => {
                cur = xmlSchemaDupVal(val);
                if !(*val).value.hex.str_0.is_null() {
                    (*cur).value.hex.str_0 = xmlStrdup((*val).value.hex.str_0);
                }
            }
            44 => {
                cur = xmlSchemaDupVal(val);
                if !(*val).value.base64.str_0.is_null() {
                    (*cur).value.base64.str_0 = xmlStrdup((*val).value.base64.str_0);
                }
            }
            _ => {
                cur = xmlSchemaDupVal(val);
            }
        }
        if ret.is_null() {
            ret = cur;
        } else {
            (*prev).next = cur as *mut _xmlSchemaVal;
        }
        prev = cur;
        val = (*val).next as xmlSchemaValPtr;
    }
    return ret;
} }
fn _xmlSchemaDateAdd(
    mut dt: xmlSchemaValPtr,
    mut dur: xmlSchemaValPtr,
) -> xmlSchemaValPtr { unsafe {
    let mut ret: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut tmp: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut carry: c_long = 0;
    let mut tempdays: c_long = 0;
    let mut temp: c_long = 0;
    let mut r: xmlSchemaValDatePtr = ::core::ptr::null_mut::<xmlSchemaValDate>();
    let mut d: xmlSchemaValDatePtr = ::core::ptr::null_mut::<xmlSchemaValDate>();
    let mut u: xmlSchemaValDurationPtr = ::core::ptr::null_mut::<xmlSchemaValDuration>();
    if dt.is_null() || dur.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    ret = xmlSchemaNewValue((*dt).type_0);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    tmp = xmlSchemaDupVal(dt);
    if tmp.is_null() {
        xmlSchemaFreeValue(ret);
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    r = &raw mut (*ret).value.date as xmlSchemaValDatePtr;
    d = &raw mut (*tmp).value.date as xmlSchemaValDatePtr;
    u = &raw mut (*dur).value.dur as xmlSchemaValDurationPtr;
    if (*d).mon() as c_int == 0 as c_int {
        (*d).set_mon(1 as c_uint as c_uint);
    }
    (*u).sec -= ((*d).tzo() * 60 as c_int) as c_double;
    (*d).set_tzo(0 as c_int as c_int);
    if (*d).day() as c_int == 0 as c_int {
        (*d).set_day(1 as c_uint as c_uint);
    }
    carry = (*d).mon() as c_long + (*u).mon;
    (*r).set_mon(
        ((carry - 1 as c_long) as c_double
            - floor(
                (carry - 1 as c_long) as c_double
                    / (13 as c_int - 1 as c_int) as c_double,
            ) * (13 as c_int - 1 as c_int) as c_double
            + 1 as c_int as c_double) as c_uint
            as c_uint,
    );
    carry = floor(
        (carry - 1 as c_long) as c_double
            / (13 as c_int - 1 as c_int) as c_double,
    ) as c_long;
    (*r).year = (*d).year + carry;
    if (*r).year == 0 as c_long {
        if (*d).year > 0 as c_long {
            (*r).year -= 1;
        } else {
            (*r).year += 1;
        }
    }
    (*r).set_tzo((*d).tzo() as c_int);
    (*r).set_tz_flag((*d).tz_flag() as c_uint);
    (*r).sec = (*d).sec + (*u).sec;
    carry = floor(
        (*r).sec as c_long as c_double
            / 60 as c_int as c_double,
    ) as c_long;
    if (*r).sec != 0.0f64 {
        (*r).sec = (*r).sec - floor((*r).sec / 60.0f64) * 60.0f64;
    }
    carry += (*d).min() as c_long;
    (*r).set_min(
        (carry as c_double
            - floor(
                carry as c_double / 60 as c_int as c_double,
            ) * 60 as c_int as c_double) as c_uint
            as c_uint,
    );
    carry =
        floor(carry as c_double / 60 as c_int as c_double)
            as c_long;
    carry += (*d).hour() as c_long;
    (*r).set_hour(
        (carry as c_double
            - floor(
                carry as c_double / 24 as c_int as c_double,
            ) * 24 as c_int as c_double) as c_uint
            as c_uint,
    );
    carry =
        floor(carry as c_double / 24 as c_int as c_double)
            as c_long;
    if (*r).year != 0 as c_long
        && ((*r).mon() as c_int >= 1 as c_int
            && (*r).mon() as c_int <= 12 as c_int)
        && (*d).day()
            > (if (*r).year % 4 as c_long == 0 as c_long
                && (*r).year % 100 as c_long != 0 as c_long
                || (*r).year % 400 as c_long == 0 as c_long
            {
                daysInMonthLeap
                    [((*r).mon() as c_int - 1 as c_int) as usize]
            } else {
                daysInMonth[((*r).mon() as c_int - 1 as c_int) as usize]
            })
    {
        tempdays = (if (*r).year % 4 as c_long == 0 as c_long
            && (*r).year % 100 as c_long != 0 as c_long
            || (*r).year % 400 as c_long == 0 as c_long
        {
            daysInMonthLeap[((*r).mon() as c_int - 1 as c_int) as usize]
        } else {
            daysInMonth[((*r).mon() as c_int - 1 as c_int) as usize]
        }) as c_long;
    } else if ((*d).day() as c_int) < 1 as c_int {
        tempdays = 1 as c_long;
    } else {
        tempdays = (*d).day() as c_long;
    }
    tempdays += (*u).day + carry;
    loop {
        if tempdays < 1 as c_long {
            let mut tmon: c_long = (((*r).mon() as c_int
                - 1 as c_int
                - 1 as c_int)
                as c_double
                - floor(
                    ((*r).mon() as c_int
                        - 1 as c_int
                        - 1 as c_int) as c_double
                        / (13 as c_int - 1 as c_int)
                            as c_double,
                ) * (13 as c_int - 1 as c_int) as c_double
                + 1 as c_int as c_double)
                as c_long;
            let mut tyr: c_long = (*r).year
                + floor(
                    ((*r).mon() as c_int
                        - 1 as c_int
                        - 1 as c_int) as c_double
                        / (13 as c_int - 1 as c_int)
                            as c_double,
                ) as c_long;
            if tyr == 0 as c_long {
                tyr -= 1;
            }
            if tmon < 1 as c_long {
                tmon = 1 as c_long;
            }
            if tmon > 12 as c_long {
                tmon = 12 as c_long;
            }
            tempdays += (if tyr % 4 as c_long == 0 as c_long
                && tyr % 100 as c_long != 0 as c_long
                || tyr % 400 as c_long == 0 as c_long
            {
                daysInMonthLeap[(tmon - 1 as c_long) as usize]
            } else {
                daysInMonth[(tmon - 1 as c_long) as usize]
            }) as c_long;
            carry = -(1 as c_int) as c_long;
        } else {
            if !((*r).year != 0 as c_long
                && ((*r).mon() as c_int >= 1 as c_int
                    && (*r).mon() as c_int <= 12 as c_int)
                && tempdays
                    > (if (*r).year % 4 as c_long == 0 as c_long
                        && (*r).year % 100 as c_long != 0 as c_long
                        || (*r).year % 400 as c_long == 0 as c_long
                    {
                        daysInMonthLeap
                            [((*r).mon() as c_int - 1 as c_int) as usize]
                    } else {
                        daysInMonth
                            [((*r).mon() as c_int - 1 as c_int) as usize]
                    }) as c_long)
            {
                break;
            }
            tempdays = tempdays
                - (if (*r).year % 4 as c_long == 0 as c_long
                    && (*r).year % 100 as c_long != 0 as c_long
                    || (*r).year % 400 as c_long == 0 as c_long
                {
                    daysInMonthLeap
                        [((*r).mon() as c_int - 1 as c_int) as usize]
                } else {
                    daysInMonth
                        [((*r).mon() as c_int - 1 as c_int) as usize]
                }) as c_long;
            carry = 1 as c_long;
        }
        temp = (*r).mon() as c_long + carry;
        (*r).set_mon(
            ((temp - 1 as c_long) as c_double
                - floor(
                    (temp - 1 as c_long) as c_double
                        / (13 as c_int - 1 as c_int)
                            as c_double,
                ) * (13 as c_int - 1 as c_int) as c_double
                + 1 as c_int as c_double)
                as c_uint as c_uint,
        );
        (*r).year = (*r).year
            + floor(
                (temp - 1 as c_long) as c_double
                    / (13 as c_int - 1 as c_int) as c_double,
            ) as c_long;
        if (*r).year == 0 as c_long {
            if temp < 1 as c_long {
                (*r).year -= 1;
            } else {
                (*r).year += 1;
            }
        }
    }
    (*r).set_day(tempdays as c_uint as c_uint);
    if (*ret).type_0 as c_uint
        != XML_SCHEMAS_DATETIME as c_int as c_uint
    {
        if (*r).hour() as c_int != 0
            || (*r).min() as c_int != 0
            || (*r).sec != 0.
        {
            (*ret).type_0 = XML_SCHEMAS_DATETIME;
        } else if (*ret).type_0 as c_uint
            != XML_SCHEMAS_DATE as c_int as c_uint
        {
            if (*r).mon() as c_int != 1 as c_int
                && (*r).day() as c_int != 1 as c_int
            {
                (*ret).type_0 = XML_SCHEMAS_DATE;
            } else if (*ret).type_0 as c_uint
                != XML_SCHEMAS_GYEARMONTH as c_int as c_uint
                && (*r).mon() as c_int != 1 as c_int
            {
                (*ret).type_0 = XML_SCHEMAS_GYEARMONTH;
            }
        }
    }
    xmlSchemaFreeValue(tmp);
    return ret;
} }
fn xmlSchemaDateNormalize(
    mut dt: xmlSchemaValPtr,
    mut offset: c_double,
) -> xmlSchemaValPtr { unsafe {
    let mut dur: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut ret: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    if dt.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    if (*dt).type_0 as c_uint
        != XML_SCHEMAS_TIME as c_int as c_uint
        && (*dt).type_0 as c_uint
            != XML_SCHEMAS_DATETIME as c_int as c_uint
        && (*dt).type_0 as c_uint
            != XML_SCHEMAS_DATE as c_int as c_uint
        || (*dt).value.date.tzo() == 0 as c_int
    {
        return xmlSchemaDupVal(dt);
    }
    dur = xmlSchemaNewValue(XML_SCHEMAS_DURATION);
    if dur.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    (*dur).value.date.sec -= offset;
    ret = _xmlSchemaDateAdd(dt, dur);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlSchemaVal>();
    }
    xmlSchemaFreeValue(dur);
    return ret;
} }
fn _xmlSchemaDateCastYMToDays(dt: xmlSchemaValPtr) -> c_long { unsafe {
    let mut ret: c_long = 0;
    let mut mon: c_int = 0;
    mon = (*dt).value.date.mon() as c_int;
    if mon <= 0 as c_int {
        mon = 1 as c_int;
    }
    if (*dt).value.date.year <= 0 as c_long {
        ret = (*dt).value.date.year * 365 as c_long
            + (((*dt).value.date.year + 1 as c_long) / 4 as c_long
                - ((*dt).value.date.year + 1 as c_long) / 100 as c_long
                + ((*dt).value.date.year + 1 as c_long) / 400 as c_long)
            + ((if (*dt).value.date.year % 4 as c_long == 0 as c_long
                && (*dt).value.date.year % 100 as c_long != 0 as c_long
                || (*dt).value.date.year % 400 as c_long == 0 as c_long
            {
                dayInLeapYearByMonth[(mon - 1 as c_int) as usize]
            } else {
                dayInYearByMonth[(mon - 1 as c_int) as usize]
            }) + 0 as c_long);
    } else {
        ret = ((*dt).value.date.year - 1 as c_long) * 365 as c_long
            + (((*dt).value.date.year - 1 as c_long) / 4 as c_long
                - ((*dt).value.date.year - 1 as c_long) / 100 as c_long
                + ((*dt).value.date.year - 1 as c_long) / 400 as c_long)
            + ((if (*dt).value.date.year % 4 as c_long == 0 as c_long
                && (*dt).value.date.year % 100 as c_long != 0 as c_long
                || (*dt).value.date.year % 400 as c_long == 0 as c_long
            {
                dayInLeapYearByMonth[(mon - 1 as c_int) as usize]
            } else {
                dayInYearByMonth[(mon - 1 as c_int) as usize]
            }) + 0 as c_long);
    }
    return ret;
} }
fn xmlSchemaCompareDates(
    mut x: xmlSchemaValPtr,
    mut y: xmlSchemaValPtr,
) -> c_int { unsafe {
    let mut xmask: c_uchar = 0;
    let mut ymask: c_uchar = 0;
    let mut xor_mask: c_uchar = 0;
    let mut and_mask: c_uchar = 0;
    let mut p1: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut p2: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut q1: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut q2: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
    let mut p1d: c_long = 0;
    let mut p2d: c_long = 0;
    let mut q1d: c_long = 0;
    let mut q2d: c_long = 0;
    if x.is_null() || y.is_null() {
        return -(2 as c_int);
    }
    if (*x).value.date.year > LONG_MAX / 366 as c_long
        || (*x).value.date.year < LONG_MIN / 366 as c_long
        || (*y).value.date.year > LONG_MAX / 366 as c_long
        || (*y).value.date.year < LONG_MIN / 366 as c_long
    {
        return -(2 as c_int);
    }
    if (*x).value.date.tz_flag() != 0 {
        if (*y).value.date.tz_flag() == 0 {
            p1 = xmlSchemaDateNormalize(x, 0 as c_int as c_double);
            if p1.is_null() {
                return -(2 as c_int);
            }
            p1d = _xmlSchemaDateCastYMToDays(p1) + (*p1).value.date.day() as c_long;
            q1 = xmlSchemaDateNormalize(
                y,
                (14 as c_int * SECS_PER_HOUR) as c_double,
            );
            if q1.is_null() {
                xmlSchemaFreeValue(p1);
                return -(2 as c_int);
            }
            q1d = _xmlSchemaDateCastYMToDays(q1) + (*q1).value.date.day() as c_long;
            if p1d < q1d {
                xmlSchemaFreeValue(p1);
                xmlSchemaFreeValue(q1);
                return -(1 as c_int);
            } else if p1d == q1d {
                let mut sec: c_double = 0.;
                sec = ((*p1).value.date.hour() as c_int * SECS_PER_HOUR
                    + (*p1).value.date.min() as c_int * SECS_PER_MIN
                    + (*p1).value.date.tzo() * SECS_PER_MIN)
                    as c_double
                    + (*p1).value.date.sec
                    - (((*q1).value.date.hour() as c_int * SECS_PER_HOUR
                        + (*q1).value.date.min() as c_int * SECS_PER_MIN
                        + (*q1).value.date.tzo() * SECS_PER_MIN)
                        as c_double
                        + (*q1).value.date.sec);
                if sec < 0.0f64 {
                    xmlSchemaFreeValue(p1);
                    xmlSchemaFreeValue(q1);
                    return -(1 as c_int);
                } else {
                    let mut ret: c_int = 0 as c_int;
                    q2 = xmlSchemaDateNormalize(
                        y,
                        -(14 as c_int * SECS_PER_HOUR) as c_double,
                    );
                    if q2.is_null() {
                        xmlSchemaFreeValue(p1);
                        xmlSchemaFreeValue(q1);
                        return -(2 as c_int);
                    }
                    q2d = _xmlSchemaDateCastYMToDays(q2)
                        + (*q2).value.date.day() as c_long;
                    if p1d > q2d {
                        ret = 1 as c_int;
                    } else if p1d == q2d {
                        sec = ((*p1).value.date.hour() as c_int * SECS_PER_HOUR
                            + (*p1).value.date.min() as c_int * SECS_PER_MIN
                            + (*p1).value.date.tzo() * SECS_PER_MIN)
                            as c_double
                            + (*p1).value.date.sec
                            - (((*q2).value.date.hour() as c_int * SECS_PER_HOUR
                                + (*q2).value.date.min() as c_int * SECS_PER_MIN
                                + (*q2).value.date.tzo() * SECS_PER_MIN)
                                as c_double
                                + (*q2).value.date.sec);
                        if sec > 0.0f64 {
                            ret = 1 as c_int;
                        } else {
                            ret = 2 as c_int;
                        }
                    }
                    xmlSchemaFreeValue(p1);
                    xmlSchemaFreeValue(q1);
                    xmlSchemaFreeValue(q2);
                    if ret != 0 as c_int {
                        return ret;
                    }
                }
            } else {
                xmlSchemaFreeValue(p1);
                xmlSchemaFreeValue(q1);
            }
        }
    } else if (*y).value.date.tz_flag() != 0 {
        q1 = xmlSchemaDateNormalize(y, 0 as c_int as c_double);
        if q1.is_null() {
            return -(2 as c_int);
        }
        q1d = _xmlSchemaDateCastYMToDays(q1) + (*q1).value.date.day() as c_long;
        p1 = xmlSchemaDateNormalize(
            x,
            -(14 as c_int * SECS_PER_HOUR) as c_double,
        );
        if p1.is_null() {
            xmlSchemaFreeValue(q1);
            return -(2 as c_int);
        }
        p1d = _xmlSchemaDateCastYMToDays(p1) + (*p1).value.date.day() as c_long;
        if p1d < q1d {
            xmlSchemaFreeValue(p1);
            xmlSchemaFreeValue(q1);
            return -(1 as c_int);
        } else if p1d == q1d {
            let mut sec_0: c_double = 0.;
            sec_0 = ((*p1).value.date.hour() as c_int * SECS_PER_HOUR
                + (*p1).value.date.min() as c_int * SECS_PER_MIN
                + (*p1).value.date.tzo() * SECS_PER_MIN)
                as c_double
                + (*p1).value.date.sec
                - (((*q1).value.date.hour() as c_int * SECS_PER_HOUR
                    + (*q1).value.date.min() as c_int * SECS_PER_MIN
                    + (*q1).value.date.tzo() * SECS_PER_MIN)
                    as c_double
                    + (*q1).value.date.sec);
            if sec_0 < 0.0f64 {
                xmlSchemaFreeValue(p1);
                xmlSchemaFreeValue(q1);
                return -(1 as c_int);
            } else {
                let mut ret_0: c_int = 0 as c_int;
                p2 = xmlSchemaDateNormalize(
                    x,
                    (14 as c_int * SECS_PER_HOUR) as c_double,
                );
                if p2.is_null() {
                    xmlSchemaFreeValue(p1);
                    xmlSchemaFreeValue(q1);
                    return -(2 as c_int);
                }
                p2d =
                    _xmlSchemaDateCastYMToDays(p2) + (*p2).value.date.day() as c_long;
                if p2d > q1d {
                    ret_0 = 1 as c_int;
                } else if p2d == q1d {
                    sec_0 = ((*p2).value.date.hour() as c_int * SECS_PER_HOUR
                        + (*p2).value.date.min() as c_int * SECS_PER_MIN
                        + (*p2).value.date.tzo() * SECS_PER_MIN)
                        as c_double
                        + (*p2).value.date.sec
                        - (((*q1).value.date.hour() as c_int * SECS_PER_HOUR
                            + (*q1).value.date.min() as c_int * SECS_PER_MIN
                            + (*q1).value.date.tzo() * SECS_PER_MIN)
                            as c_double
                            + (*q1).value.date.sec);
                    if sec_0 > 0.0f64 {
                        ret_0 = 1 as c_int;
                    } else {
                        ret_0 = 2 as c_int;
                    }
                }
                xmlSchemaFreeValue(p1);
                xmlSchemaFreeValue(q1);
                xmlSchemaFreeValue(p2);
                if ret_0 != 0 as c_int {
                    return ret_0;
                }
            }
        } else {
            xmlSchemaFreeValue(p1);
            xmlSchemaFreeValue(q1);
        }
    }
    if (*x).type_0 as c_uint == (*y).type_0 as c_uint {
        let mut ret_1: c_int = 0 as c_int;
        q1 = xmlSchemaDateNormalize(y, 0 as c_int as c_double);
        if q1.is_null() {
            return -(2 as c_int);
        }
        q1d = _xmlSchemaDateCastYMToDays(q1) + (*q1).value.date.day() as c_long;
        p1 = xmlSchemaDateNormalize(x, 0 as c_int as c_double);
        if p1.is_null() {
            xmlSchemaFreeValue(q1);
            return -(2 as c_int);
        }
        p1d = _xmlSchemaDateCastYMToDays(p1) + (*p1).value.date.day() as c_long;
        if p1d < q1d {
            ret_1 = -(1 as c_int);
        } else if p1d > q1d {
            ret_1 = 1 as c_int;
        } else {
            let mut sec_1: c_double = 0.;
            sec_1 = ((*p1).value.date.hour() as c_int * SECS_PER_HOUR
                + (*p1).value.date.min() as c_int * SECS_PER_MIN
                + (*p1).value.date.tzo() * SECS_PER_MIN)
                as c_double
                + (*p1).value.date.sec
                - (((*q1).value.date.hour() as c_int * SECS_PER_HOUR
                    + (*q1).value.date.min() as c_int * SECS_PER_MIN
                    + (*q1).value.date.tzo() * SECS_PER_MIN)
                    as c_double
                    + (*q1).value.date.sec);
            if sec_1 < 0.0f64 {
                ret_1 = -(1 as c_int);
            } else if sec_1 > 0.0f64 {
                ret_1 = 1 as c_int;
            }
        }
        xmlSchemaFreeValue(p1);
        xmlSchemaFreeValue(q1);
        return ret_1;
    }
    match (*x).type_0 as c_uint {
        11 => {
            xmask = 0xf as c_uchar;
        }
        10 => {
            xmask = 0x7 as c_uchar;
        }
        8 => {
            xmask = 0x1 as c_uchar;
        }
        6 => {
            xmask = 0x2 as c_uchar;
        }
        5 => {
            xmask = 0x3 as c_uchar;
        }
        9 => {
            xmask = 0x3 as c_uchar;
        }
        7 => {
            xmask = 0x6 as c_uchar;
        }
        4 => {
            xmask = 0x8 as c_uchar;
        }
        _ => {
            xmask = 0 as c_uchar;
        }
    }
    match (*y).type_0 as c_uint {
        11 => {
            ymask = 0xf as c_uchar;
        }
        10 => {
            ymask = 0x7 as c_uchar;
        }
        8 => {
            ymask = 0x1 as c_uchar;
        }
        6 => {
            ymask = 0x2 as c_uchar;
        }
        5 => {
            ymask = 0x3 as c_uchar;
        }
        9 => {
            ymask = 0x3 as c_uchar;
        }
        7 => {
            ymask = 0x6 as c_uchar;
        }
        4 => {
            ymask = 0x8 as c_uchar;
        }
        _ => {
            ymask = 0 as c_uchar;
        }
    }
    xor_mask = (xmask as c_int ^ ymask as c_int) as c_uchar;
    and_mask = (xmask as c_int & ymask as c_int) as c_uchar;
    if xor_mask as c_int & 1 as c_int != 0 {
        return 2 as c_int;
    } else if and_mask as c_int & 1 as c_int != 0 {
        if (*x).value.date.year < (*y).value.date.year {
            return -(1 as c_int);
        } else if (*x).value.date.year > (*y).value.date.year {
            return 1 as c_int;
        }
    }
    if xor_mask as c_int & 2 as c_int != 0 {
        return 2 as c_int;
    } else if and_mask as c_int & 2 as c_int != 0 {
        if ((*x).value.date.mon() as c_int)
            < (*y).value.date.mon() as c_int
        {
            return -(1 as c_int);
        } else if (*x).value.date.mon() as c_int
            > (*y).value.date.mon() as c_int
        {
            return 1 as c_int;
        }
    }
    if xor_mask as c_int & 4 as c_int != 0 {
        return 2 as c_int;
    } else if and_mask as c_int & 4 as c_int != 0 {
        if ((*x).value.date.day() as c_int)
            < (*y).value.date.day() as c_int
        {
            return -(1 as c_int);
        } else if (*x).value.date.day() as c_int
            > (*y).value.date.day() as c_int
        {
            return 1 as c_int;
        }
    }
    if xor_mask as c_int & 8 as c_int != 0 {
        return 2 as c_int;
    } else if and_mask as c_int & 8 as c_int != 0 {
        if ((*x).value.date.hour() as c_int)
            < (*y).value.date.hour() as c_int
        {
            return -(1 as c_int);
        } else if (*x).value.date.hour() as c_int
            > (*y).value.date.hour() as c_int
        {
            return 1 as c_int;
        } else if ((*x).value.date.min() as c_int)
            < (*y).value.date.min() as c_int
        {
            return -(1 as c_int);
        } else if (*x).value.date.min() as c_int
            > (*y).value.date.min() as c_int
        {
            return 1 as c_int;
        } else if (*x).value.date.sec < (*y).value.date.sec {
            return -(1 as c_int);
        } else if (*x).value.date.sec > (*y).value.date.sec {
            return 1 as c_int;
        }
    }
    return 0 as c_int;
} }
unsafe fn xmlSchemaComparePreserveReplaceStrings(
    mut x: *const xmlChar,
    mut y: *const xmlChar,
    mut invert: c_int,
) -> c_int {
    let mut tmp: c_int = 0;
    while *x as c_int != 0 as c_int
        && *y as c_int != 0 as c_int
    {
        if *y as c_int == 0x9 as c_int
            || *y as c_int == 0xa as c_int
            || *y as c_int == 0xd as c_int
        {
            if !(*x as c_int == 0x20 as c_int) {
                if (*x as c_int - 0x20 as c_int) < 0 as c_int
                {
                    if invert != 0 {
                        return 1 as c_int;
                    } else {
                        return -(1 as c_int);
                    }
                } else if invert != 0 {
                    return -(1 as c_int);
                } else {
                    return 1 as c_int;
                }
            }
        } else {
            tmp = *x as c_int - *y as c_int;
            if tmp < 0 as c_int {
                if invert != 0 {
                    return 1 as c_int;
                } else {
                    return -(1 as c_int);
                }
            }
            if tmp > 0 as c_int {
                if invert != 0 {
                    return -(1 as c_int);
                } else {
                    return 1 as c_int;
                }
            }
        }
        x = x.offset(1);
        y = y.offset(1);
    }
    if *x as c_int != 0 as c_int {
        if invert != 0 {
            return -(1 as c_int);
        } else {
            return 1 as c_int;
        }
    }
    if *y as c_int != 0 as c_int {
        if invert != 0 {
            return 1 as c_int;
        } else {
            return -(1 as c_int);
        }
    }
    return 0 as c_int;
}
unsafe fn xmlSchemaComparePreserveCollapseStrings(
    mut x: *const xmlChar,
    mut y: *const xmlChar,
    mut invert: c_int,
) -> c_int {
    let mut tmp: c_int = 0;
    while *y as c_int == 0x20 as c_int
        || 0x9 as c_int <= *y as c_int
            && *y as c_int <= 0xa as c_int
        || *y as c_int == 0xd as c_int
    {
        y = y.offset(1);
    }
    while *x as c_int != 0 as c_int
        && *y as c_int != 0 as c_int
    {
        if *y as c_int == 0x20 as c_int
            || 0x9 as c_int <= *y as c_int
                && *y as c_int <= 0xa as c_int
            || *y as c_int == 0xd as c_int
        {
            if !(*x as c_int == 0x20 as c_int) {
                if (*x as c_int - 0x20 as c_int) < 0 as c_int
                {
                    if invert != 0 {
                        return 1 as c_int;
                    } else {
                        return -(1 as c_int);
                    }
                } else if invert != 0 {
                    return -(1 as c_int);
                } else {
                    return 1 as c_int;
                }
            }
            x = x.offset(1);
            y = y.offset(1);
            while *y as c_int == 0x20 as c_int
                || 0x9 as c_int <= *y as c_int
                    && *y as c_int <= 0xa as c_int
                || *y as c_int == 0xd as c_int
            {
                y = y.offset(1);
            }
        } else {
            let fresh25 = x;
            x = x.offset(1);
            let fresh26 = y;
            y = y.offset(1);
            tmp = *fresh25 as c_int - *fresh26 as c_int;
            if tmp < 0 as c_int {
                if invert != 0 {
                    return 1 as c_int;
                } else {
                    return -(1 as c_int);
                }
            }
            if tmp > 0 as c_int {
                if invert != 0 {
                    return -(1 as c_int);
                } else {
                    return 1 as c_int;
                }
            }
        }
    }
    if *x as c_int != 0 as c_int {
        if invert != 0 {
            return -(1 as c_int);
        } else {
            return 1 as c_int;
        }
    }
    if *y as c_int != 0 as c_int {
        while *y as c_int == 0x20 as c_int
            || 0x9 as c_int <= *y as c_int
                && *y as c_int <= 0xa as c_int
            || *y as c_int == 0xd as c_int
        {
            y = y.offset(1);
        }
        if *y as c_int != 0 as c_int {
            if invert != 0 {
                return 1 as c_int;
            } else {
                return -(1 as c_int);
            }
        }
    }
    return 0 as c_int;
}
unsafe fn xmlSchemaCompareReplaceCollapseStrings(
    mut x: *const xmlChar,
    mut y: *const xmlChar,
    mut invert: c_int,
) -> c_int {
    let mut tmp: c_int = 0;
    while *y as c_int == 0x20 as c_int
        || 0x9 as c_int <= *y as c_int
            && *y as c_int <= 0xa as c_int
        || *y as c_int == 0xd as c_int
    {
        y = y.offset(1);
    }
    while *x as c_int != 0 as c_int
        && *y as c_int != 0 as c_int
    {
        if *y as c_int == 0x20 as c_int
            || 0x9 as c_int <= *y as c_int
                && *y as c_int <= 0xa as c_int
            || *y as c_int == 0xd as c_int
        {
            if !(*x as c_int == 0x20 as c_int
                || 0x9 as c_int <= *x as c_int
                    && *x as c_int <= 0xa as c_int
                || *x as c_int == 0xd as c_int)
            {
                if (*x as c_int - 0x20 as c_int) < 0 as c_int
                {
                    if invert != 0 {
                        return 1 as c_int;
                    } else {
                        return -(1 as c_int);
                    }
                } else if invert != 0 {
                    return -(1 as c_int);
                } else {
                    return 1 as c_int;
                }
            }
            x = x.offset(1);
            y = y.offset(1);
            while *y as c_int == 0x20 as c_int
                || 0x9 as c_int <= *y as c_int
                    && *y as c_int <= 0xa as c_int
                || *y as c_int == 0xd as c_int
            {
                y = y.offset(1);
            }
        } else {
            if *x as c_int == 0x20 as c_int
                || 0x9 as c_int <= *x as c_int
                    && *x as c_int <= 0xa as c_int
                || *x as c_int == 0xd as c_int
            {
                if (0x20 as c_int - *y as c_int) < 0 as c_int
                {
                    if invert != 0 {
                        return 1 as c_int;
                    } else {
                        return -(1 as c_int);
                    }
                } else if invert != 0 {
                    return -(1 as c_int);
                } else {
                    return 1 as c_int;
                }
            }
            let fresh23 = x;
            x = x.offset(1);
            let fresh24 = y;
            y = y.offset(1);
            tmp = *fresh23 as c_int - *fresh24 as c_int;
            if tmp < 0 as c_int {
                return -(1 as c_int);
            }
            if tmp > 0 as c_int {
                return 1 as c_int;
            }
        }
    }
    if *x as c_int != 0 as c_int {
        if invert != 0 {
            return -(1 as c_int);
        } else {
            return 1 as c_int;
        }
    }
    if *y as c_int != 0 as c_int {
        while *y as c_int == 0x20 as c_int
            || 0x9 as c_int <= *y as c_int
                && *y as c_int <= 0xa as c_int
            || *y as c_int == 0xd as c_int
        {
            y = y.offset(1);
        }
        if *y as c_int != 0 as c_int {
            if invert != 0 {
                return 1 as c_int;
            } else {
                return -(1 as c_int);
            }
        }
    }
    return 0 as c_int;
}
unsafe fn xmlSchemaCompareReplacedStrings(
    mut x: *const xmlChar,
    mut y: *const xmlChar,
) -> c_int {
    let mut tmp: c_int = 0;
    while *x as c_int != 0 as c_int
        && *y as c_int != 0 as c_int
    {
        if *y as c_int == 0x20 as c_int
            || 0x9 as c_int <= *y as c_int
                && *y as c_int <= 0xa as c_int
            || *y as c_int == 0xd as c_int
        {
            if !(*x as c_int == 0x20 as c_int
                || 0x9 as c_int <= *x as c_int
                    && *x as c_int <= 0xa as c_int
                || *x as c_int == 0xd as c_int)
            {
                if (*x as c_int - 0x20 as c_int) < 0 as c_int
                {
                    return -(1 as c_int);
                } else {
                    return 1 as c_int;
                }
            }
        } else {
            if *x as c_int == 0x20 as c_int
                || 0x9 as c_int <= *x as c_int
                    && *x as c_int <= 0xa as c_int
                || *x as c_int == 0xd as c_int
            {
                if (0x20 as c_int - *y as c_int) < 0 as c_int
                {
                    return -(1 as c_int);
                } else {
                    return 1 as c_int;
                }
            }
            tmp = *x as c_int - *y as c_int;
            if tmp < 0 as c_int {
                return -(1 as c_int);
            }
            if tmp > 0 as c_int {
                return 1 as c_int;
            }
        }
        x = x.offset(1);
        y = y.offset(1);
    }
    if *x as c_int != 0 as c_int {
        return 1 as c_int;
    }
    if *y as c_int != 0 as c_int {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
unsafe fn xmlSchemaCompareNormStrings(
    mut x: *const xmlChar,
    mut y: *const xmlChar,
) -> c_int {
    let mut tmp: c_int = 0;
    while *x as c_int == 0x20 as c_int
        || 0x9 as c_int <= *x as c_int
            && *x as c_int <= 0xa as c_int
        || *x as c_int == 0xd as c_int
    {
        x = x.offset(1);
    }
    while *y as c_int == 0x20 as c_int
        || 0x9 as c_int <= *y as c_int
            && *y as c_int <= 0xa as c_int
        || *y as c_int == 0xd as c_int
    {
        y = y.offset(1);
    }
    while *x as c_int != 0 as c_int
        && *y as c_int != 0 as c_int
    {
        if *x as c_int == 0x20 as c_int
            || 0x9 as c_int <= *x as c_int
                && *x as c_int <= 0xa as c_int
            || *x as c_int == 0xd as c_int
        {
            if !(*y as c_int == 0x20 as c_int
                || 0x9 as c_int <= *y as c_int
                    && *y as c_int <= 0xa as c_int
                || *y as c_int == 0xd as c_int)
            {
                tmp = *x as c_int - *y as c_int;
                return tmp;
            }
            while *x as c_int == 0x20 as c_int
                || 0x9 as c_int <= *x as c_int
                    && *x as c_int <= 0xa as c_int
                || *x as c_int == 0xd as c_int
            {
                x = x.offset(1);
            }
            while *y as c_int == 0x20 as c_int
                || 0x9 as c_int <= *y as c_int
                    && *y as c_int <= 0xa as c_int
                || *y as c_int == 0xd as c_int
            {
                y = y.offset(1);
            }
        } else {
            let fresh21 = x;
            x = x.offset(1);
            let fresh22 = y;
            y = y.offset(1);
            tmp = *fresh21 as c_int - *fresh22 as c_int;
            if tmp < 0 as c_int {
                return -(1 as c_int);
            }
            if tmp > 0 as c_int {
                return 1 as c_int;
            }
        }
    }
    if *x as c_int != 0 as c_int {
        while *x as c_int == 0x20 as c_int
            || 0x9 as c_int <= *x as c_int
                && *x as c_int <= 0xa as c_int
            || *x as c_int == 0xd as c_int
        {
            x = x.offset(1);
        }
        if *x as c_int != 0 as c_int {
            return 1 as c_int;
        }
    }
    if *y as c_int != 0 as c_int {
        while *y as c_int == 0x20 as c_int
            || 0x9 as c_int <= *y as c_int
                && *y as c_int <= 0xa as c_int
            || *y as c_int == 0xd as c_int
        {
            y = y.offset(1);
        }
        if *y as c_int != 0 as c_int {
            return -(1 as c_int);
        }
    }
    return 0 as c_int;
}
fn xmlSchemaCompareFloats(
    mut x: xmlSchemaValPtr,
    mut y: xmlSchemaValPtr,
) -> c_int { unsafe {
    let mut d1: c_double = 0.;
    let mut d2: c_double = 0.;
    if x.is_null() || y.is_null() {
        return -(2 as c_int);
    }
    if (*x).type_0 as c_uint
        == XML_SCHEMAS_DOUBLE as c_int as c_uint
    {
        d1 = (*x).value.d;
    } else if (*x).type_0 as c_uint
        == XML_SCHEMAS_FLOAT as c_int as c_uint
    {
        d1 = (*x).value.f as c_double;
    } else {
        return -(2 as c_int);
    }
    if (*y).type_0 as c_uint
        == XML_SCHEMAS_DOUBLE as c_int as c_uint
    {
        d2 = (*y).value.d;
    } else if (*y).type_0 as c_uint
        == XML_SCHEMAS_FLOAT as c_int as c_uint
    {
        d2 = (*y).value.f as c_double;
    } else {
        return -(2 as c_int);
    }
    if xmlXPathIsNaN(d1) != 0 {
        if xmlXPathIsNaN(d2) != 0 {
            return 0 as c_int;
        }
        return 1 as c_int;
    }
    if xmlXPathIsNaN(d2) != 0 {
        return -(1 as c_int);
    }
    if d1 == xmlXPathPINF {
        if d2 == xmlXPathPINF {
            return 0 as c_int;
        }
        return 1 as c_int;
    }
    if d2 == xmlXPathPINF {
        return -(1 as c_int);
    }
    if d1 == xmlXPathNINF {
        if d2 == xmlXPathNINF {
            return 0 as c_int;
        }
        return -(1 as c_int);
    }
    if d2 == xmlXPathNINF {
        return 1 as c_int;
    }
    if d1 < d2 {
        return -(1 as c_int);
    }
    if d1 > d2 {
        return 1 as c_int;
    }
    if d1 == d2 {
        return 0 as c_int;
    }
    return 2 as c_int;
} }
unsafe fn xmlSchemaCompareValuesInternal(
    mut xtype: xmlSchemaValType,
    mut x: xmlSchemaValPtr,
    mut xvalue: *const xmlChar,
    mut xws: xmlSchemaWhitespaceValueType,
    mut ytype: xmlSchemaValType,
    mut y: xmlSchemaValPtr,
    mut yvalue: *const xmlChar,
    mut yws: xmlSchemaWhitespaceValueType,
) -> c_int {
    match xtype as c_uint {
        0 | 45 => return -(2 as c_int),
        30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 3 => {
            if x.is_null() || y.is_null() {
                return -(2 as c_int);
            }
            if ytype as c_uint == xtype as c_uint {
                return xmlSchemaCompareDecimals(x, y);
            }
            if ytype as c_uint
                == XML_SCHEMAS_DECIMAL as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_INTEGER as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_NPINTEGER as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_NINTEGER as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_NNINTEGER as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_PINTEGER as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_INT as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_UINT as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_LONG as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_ULONG as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_SHORT as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_USHORT as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_BYTE as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_UBYTE as c_int as c_uint
            {
                return xmlSchemaCompareDecimals(x, y);
            }
            return -(2 as c_int);
        }
        12 => {
            if x.is_null() || y.is_null() {
                return -(2 as c_int);
            }
            if ytype as c_uint
                == XML_SCHEMAS_DURATION as c_int as c_uint
            {
                return xmlSchemaCompareDurations(x, y);
            }
            return -(2 as c_int);
        }
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
            if x.is_null() || y.is_null() {
                return -(2 as c_int);
            }
            if ytype as c_uint
                == XML_SCHEMAS_DATETIME as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_TIME as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_GDAY as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_GMONTH as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_GMONTHDAY as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_GYEAR as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_DATE as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_GYEARMONTH as c_int as c_uint
            {
                return xmlSchemaCompareDates(x, y);
            }
            return -(2 as c_int);
        }
        46 | 1 | 2 | 16 | 17 | 18 | 20 | 22 | 23 | 24 | 26 | 29 => {
            let mut xv: *const xmlChar = ::core::ptr::null::<xmlChar>();
            let mut yv: *const xmlChar = ::core::ptr::null::<xmlChar>();
            if x.is_null() {
                xv = xvalue;
            } else {
                xv = (*x).value.str_0;
            }
            if y.is_null() {
                yv = yvalue;
            } else {
                yv = (*y).value.str_0;
            }
            if ytype as c_uint
                == XML_SCHEMAS_QNAME as c_int as c_uint
            {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlschemastypes.c\0"
                        as *const u8 as *const c_char,
                    5048 as c_int,
                );
                if y.is_null() {
                    return -(2 as c_int);
                }
                return -(2 as c_int);
            }
            if ytype as c_uint
                == XML_SCHEMAS_ANYSIMPLETYPE as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_STRING as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_NORMSTRING as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_TOKEN as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_LANGUAGE as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_NMTOKEN as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_NAME as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_NCNAME as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_ID as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_IDREF as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_ENTITY as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_ANYURI as c_int as c_uint
            {
                if xws as c_uint
                    == XML_SCHEMA_WHITESPACE_PRESERVE as c_int as c_uint
                {
                    if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_PRESERVE as c_int
                            as c_uint
                    {
                        if xmlStrEqual(xv, yv) != 0 {
                            return 0 as c_int;
                        } else {
                            return 2 as c_int;
                        }
                    } else if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_REPLACE as c_int
                            as c_uint
                    {
                        return xmlSchemaComparePreserveReplaceStrings(
                            xv,
                            yv,
                            0 as c_int,
                        );
                    } else if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int
                            as c_uint
                    {
                        return xmlSchemaComparePreserveCollapseStrings(
                            xv,
                            yv,
                            0 as c_int,
                        );
                    }
                } else if xws as c_uint
                    == XML_SCHEMA_WHITESPACE_REPLACE as c_int as c_uint
                {
                    if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_PRESERVE as c_int
                            as c_uint
                    {
                        return xmlSchemaComparePreserveReplaceStrings(
                            yv,
                            xv,
                            1 as c_int,
                        );
                    }
                    if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_REPLACE as c_int
                            as c_uint
                    {
                        return xmlSchemaCompareReplacedStrings(xv, yv);
                    }
                    if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int
                            as c_uint
                    {
                        return xmlSchemaCompareReplaceCollapseStrings(
                            xv,
                            yv,
                            0 as c_int,
                        );
                    }
                } else if xws as c_uint
                    == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int as c_uint
                {
                    if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_PRESERVE as c_int
                            as c_uint
                    {
                        return xmlSchemaComparePreserveCollapseStrings(
                            yv,
                            xv,
                            1 as c_int,
                        );
                    }
                    if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_REPLACE as c_int
                            as c_uint
                    {
                        return xmlSchemaCompareReplaceCollapseStrings(
                            yv,
                            xv,
                            1 as c_int,
                        );
                    }
                    if yws as c_uint
                        == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int
                            as c_uint
                    {
                        return xmlSchemaCompareNormStrings(xv, yv);
                    }
                } else {
                    return -(2 as c_int);
                }
            }
            return -(2 as c_int);
        }
        21 | 28 => {
            if x.is_null() || y.is_null() {
                return -(2 as c_int);
            }
            if ytype as c_uint
                == XML_SCHEMAS_QNAME as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_NOTATION as c_int as c_uint
            {
                if xmlStrEqual((*x).value.qname.name, (*y).value.qname.name) != 0
                    && xmlStrEqual((*x).value.qname.uri, (*y).value.qname.uri) != 0
                {
                    return 0 as c_int;
                }
                return 2 as c_int;
            }
            return -(2 as c_int);
        }
        13 | 14 => {
            if x.is_null() || y.is_null() {
                return -(2 as c_int);
            }
            if ytype as c_uint
                == XML_SCHEMAS_FLOAT as c_int as c_uint
                || ytype as c_uint
                    == XML_SCHEMAS_DOUBLE as c_int as c_uint
            {
                return xmlSchemaCompareFloats(x, y);
            }
            return -(2 as c_int);
        }
        15 => {
            if x.is_null() || y.is_null() {
                return -(2 as c_int);
            }
            if ytype as c_uint
                == XML_SCHEMAS_BOOLEAN as c_int as c_uint
            {
                if (*x).value.b == (*y).value.b {
                    return 0 as c_int;
                }
                if (*x).value.b == 0 as c_int {
                    return -(1 as c_int);
                }
                return 1 as c_int;
            }
            return -(2 as c_int);
        }
        43 => {
            if x.is_null() || y.is_null() {
                return -(2 as c_int);
            }
            if ytype as c_uint
                == XML_SCHEMAS_HEXBINARY as c_int as c_uint
            {
                if (*x).value.hex.total == (*y).value.hex.total {
                    let mut ret: c_int =
                        xmlStrcmp((*x).value.hex.str_0, (*y).value.hex.str_0);
                    if ret > 0 as c_int {
                        return 1 as c_int;
                    } else if ret == 0 as c_int {
                        return 0 as c_int;
                    }
                } else if (*x).value.hex.total > (*y).value.hex.total {
                    return 1 as c_int;
                }
                return -(1 as c_int);
            }
            return -(2 as c_int);
        }
        44 => {
            if x.is_null() || y.is_null() {
                return -(2 as c_int);
            }
            if ytype as c_uint
                == XML_SCHEMAS_BASE64BINARY as c_int as c_uint
            {
                if (*x).value.base64.total == (*y).value.base64.total {
                    let mut ret_0: c_int =
                        xmlStrcmp((*x).value.base64.str_0, (*y).value.base64.str_0);
                    if ret_0 > 0 as c_int {
                        return 1 as c_int;
                    } else if ret_0 == 0 as c_int {
                        return 0 as c_int;
                    } else {
                        return -(1 as c_int);
                    }
                } else if (*x).value.base64.total > (*y).value.base64.total {
                    return 1 as c_int;
                } else {
                    return -(1 as c_int);
                }
            }
            return -(2 as c_int);
        }
        25 | 27 | 19 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlschemastypes.c\0"
                    as *const u8 as *const c_char,
                5173 as c_int,
            );
        }
        _ => {}
    }
    return -(2 as c_int);
}
#[inline]
pub fn xmlSchemaCompareValues(
    mut x: xmlSchemaValPtr,
    mut y: xmlSchemaValPtr,
) -> c_int { unsafe {
    let mut xws: xmlSchemaWhitespaceValueType = XML_SCHEMA_WHITESPACE_UNKNOWN;
    let mut yws: xmlSchemaWhitespaceValueType = XML_SCHEMA_WHITESPACE_UNKNOWN;
    if x.is_null() || y.is_null() {
        return -(2 as c_int);
    }
    if (*x).type_0 as c_uint
        == XML_SCHEMAS_STRING as c_int as c_uint
    {
        xws = XML_SCHEMA_WHITESPACE_PRESERVE;
    } else if (*x).type_0 as c_uint
        == XML_SCHEMAS_NORMSTRING as c_int as c_uint
    {
        xws = XML_SCHEMA_WHITESPACE_REPLACE;
    } else {
        xws = XML_SCHEMA_WHITESPACE_COLLAPSE;
    }
    if (*y).type_0 as c_uint
        == XML_SCHEMAS_STRING as c_int as c_uint
    {
        yws = XML_SCHEMA_WHITESPACE_PRESERVE;
    } else if (*y).type_0 as c_uint
        == XML_SCHEMAS_NORMSTRING as c_int as c_uint
    {
        yws = XML_SCHEMA_WHITESPACE_REPLACE;
    } else {
        yws = XML_SCHEMA_WHITESPACE_COLLAPSE;
    }
    return xmlSchemaCompareValuesInternal(
        (*x).type_0,
        x,
        ::core::ptr::null::<xmlChar>(),
        xws,
        (*y).type_0,
        y,
        ::core::ptr::null::<xmlChar>(),
        yws,
    );
} }
#[inline]
pub fn xmlSchemaCompareValuesWhtsp(
    mut x: xmlSchemaValPtr,
    mut xws: xmlSchemaWhitespaceValueType,
    mut y: xmlSchemaValPtr,
    mut yws: xmlSchemaWhitespaceValueType,
) -> c_int { unsafe {
    if x.is_null() || y.is_null() {
        return -(2 as c_int);
    }
    return xmlSchemaCompareValuesInternal(
        (*x).type_0,
        x,
        ::core::ptr::null::<xmlChar>(),
        xws,
        (*y).type_0,
        y,
        ::core::ptr::null::<xmlChar>(),
        yws,
    );
} }
unsafe fn xmlSchemaCompareValuesWhtspExt(
    mut xtype: xmlSchemaValType,
    mut x: xmlSchemaValPtr,
    mut xvalue: *const xmlChar,
    mut xws: xmlSchemaWhitespaceValueType,
    mut ytype: xmlSchemaValType,
    mut y: xmlSchemaValPtr,
    mut yvalue: *const xmlChar,
    mut yws: xmlSchemaWhitespaceValueType,
) -> c_int {
    return xmlSchemaCompareValuesInternal(xtype, x, xvalue, xws, ytype, y, yvalue, yws);
}
unsafe fn xmlSchemaNormLen(mut value: *const xmlChar) -> c_int {
    let mut utf: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: c_int = 0 as c_int;
    if value.is_null() {
        return -(1 as c_int);
    }
    utf = value;
    while *utf as c_int == 0x20 as c_int
        || 0x9 as c_int <= *utf as c_int
            && *utf as c_int <= 0xa as c_int
        || *utf as c_int == 0xd as c_int
    {
        utf = utf.offset(1);
    }
    while *utf as c_int != 0 as c_int {
        if *utf.offset(0 as c_int as isize) as c_int
            & 0x80 as c_int
            != 0
        {
            if *utf.offset(1 as c_int as isize) as c_int
                & 0xc0 as c_int
                != 0x80 as c_int
            {
                return -(1 as c_int);
            }
            if *utf.offset(0 as c_int as isize) as c_int
                & 0xe0 as c_int
                == 0xe0 as c_int
            {
                if *utf.offset(2 as c_int as isize) as c_int
                    & 0xc0 as c_int
                    != 0x80 as c_int
                {
                    return -(1 as c_int);
                }
                if *utf.offset(0 as c_int as isize) as c_int
                    & 0xf0 as c_int
                    == 0xf0 as c_int
                {
                    if *utf.offset(0 as c_int as isize) as c_int
                        & 0xf8 as c_int
                        != 0xf0 as c_int
                        || *utf.offset(3 as c_int as isize) as c_int
                            & 0xc0 as c_int
                            != 0x80 as c_int
                    {
                        return -(1 as c_int);
                    }
                    utf = utf.offset(4 as c_int as isize);
                } else {
                    utf = utf.offset(3 as c_int as isize);
                }
            } else {
                utf = utf.offset(2 as c_int as isize);
            }
        } else if *utf as c_int == 0x20 as c_int
            || 0x9 as c_int <= *utf as c_int
                && *utf as c_int <= 0xa as c_int
            || *utf as c_int == 0xd as c_int
        {
            while *utf as c_int == 0x20 as c_int
                || 0x9 as c_int <= *utf as c_int
                    && *utf as c_int <= 0xa as c_int
                || *utf as c_int == 0xd as c_int
            {
                utf = utf.offset(1);
            }
            if *utf as c_int == 0 as c_int {
                break;
            }
        } else {
            utf = utf.offset(1);
        }
        ret += 1;
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn xmlSchemaGetFacetValueAsULong(
    mut facet: xmlSchemaFacetPtr,
) -> c_ulong { unsafe {
    if facet.is_null() || (*facet).val.is_null() {
        return 0 as c_ulong;
    }
    return (*(*facet).val).value.decimal.lo;
} }
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateListSimpleTypeFacet(
    mut facet: xmlSchemaFacetPtr,
    mut value: *const xmlChar,
    mut actualLen: c_ulong,
    mut expectedLen: *mut c_ulong,
) -> c_int {
    if facet.is_null() {
        return -(1 as c_int);
    }
    if (*facet).type_0 as c_uint
        == XML_SCHEMA_FACET_LENGTH as c_int as c_uint
    {
        if actualLen != (*(*facet).val).value.decimal.lo {
            if !expectedLen.is_null() {
                *expectedLen = (*(*facet).val).value.decimal.lo;
            }
            return XML_SCHEMAV_CVC_LENGTH_VALID as c_int;
        }
    } else if (*facet).type_0 as c_uint
        == XML_SCHEMA_FACET_MINLENGTH as c_int as c_uint
    {
        if actualLen < (*(*facet).val).value.decimal.lo {
            if !expectedLen.is_null() {
                *expectedLen = (*(*facet).val).value.decimal.lo;
            }
            return XML_SCHEMAV_CVC_MINLENGTH_VALID as c_int;
        }
    } else if (*facet).type_0 as c_uint
        == XML_SCHEMA_FACET_MAXLENGTH as c_int as c_uint
    {
        if actualLen > (*(*facet).val).value.decimal.lo {
            if !expectedLen.is_null() {
                *expectedLen = (*(*facet).val).value.decimal.lo;
            }
            return XML_SCHEMAV_CVC_MAXLENGTH_VALID as c_int;
        }
    } else {
        return xmlSchemaValidateFacet(
            ::core::ptr::null_mut::<xmlSchemaType>(),
            facet,
            value,
            ::core::ptr::null_mut::<xmlSchemaVal>(),
        );
    }
    return 0 as c_int;
}
unsafe fn xmlSchemaValidateLengthFacetInternal(
    mut facet: xmlSchemaFacetPtr,
    mut valType: xmlSchemaValType,
    mut value: *const xmlChar,
    mut val: xmlSchemaValPtr,
    mut length: *mut c_ulong,
    mut ws: xmlSchemaWhitespaceValueType,
) -> c_int {
    let mut len: c_uint = 0 as c_uint;
    if length.is_null() || facet.is_null() {
        return -(1 as c_int);
    }
    *length = 0 as c_ulong;
    if (*facet).type_0 as c_uint
        != XML_SCHEMA_FACET_LENGTH as c_int as c_uint
        && (*facet).type_0 as c_uint
            != XML_SCHEMA_FACET_MAXLENGTH as c_int as c_uint
        && (*facet).type_0 as c_uint
            != XML_SCHEMA_FACET_MINLENGTH as c_int as c_uint
    {
        return -(1 as c_int);
    }
    if (*facet).val.is_null()
        || (*(*facet).val).type_0 as c_uint
            != XML_SCHEMAS_DECIMAL as c_int as c_uint
            && (*(*facet).val).type_0 as c_uint
                != XML_SCHEMAS_NNINTEGER as c_int as c_uint
        || (*(*facet).val).value.decimal.frac() as c_int != 0 as c_int
    {
        return -(1 as c_int);
    }
    if !val.is_null()
        && (*val).type_0 as c_uint
            == XML_SCHEMAS_HEXBINARY as c_int as c_uint
    {
        len = (*val).value.hex.total;
    } else if !val.is_null()
        && (*val).type_0 as c_uint
            == XML_SCHEMAS_BASE64BINARY as c_int as c_uint
    {
        len = (*val).value.base64.total;
    } else {
        match valType as c_uint {
            1 | 2 => {
                if ws as c_uint
                    == XML_SCHEMA_WHITESPACE_UNKNOWN as c_int as c_uint
                {
                    if valType as c_uint
                        == XML_SCHEMAS_STRING as c_int as c_uint
                    {
                        len = xmlUTF8Strlen(value) as c_uint;
                    } else {
                        len = xmlSchemaNormLen(value) as c_uint;
                    }
                } else if !value.is_null() {
                    if ws as c_uint
                        == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int
                            as c_uint
                    {
                        len = xmlSchemaNormLen(value) as c_uint;
                    } else {
                        len = xmlUTF8Strlen(value) as c_uint;
                    }
                }
            }
            24 | 16 | 17 | 18 | 20 | 22 | 23 | 29 => {
                if !value.is_null() {
                    len = xmlSchemaNormLen(value) as c_uint;
                }
            }
            21 | 28 => return 0 as c_int,
            _ => {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                    b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlschemastypes.c\0"
                        as *const u8 as *const c_char,
                    5474 as c_int,
                );
            }
        }
    }
    *length = len as c_ulong;
    if (*facet).type_0 as c_uint
        == XML_SCHEMA_FACET_LENGTH as c_int as c_uint
    {
        if len as c_ulong != (*(*facet).val).value.decimal.lo {
            return XML_SCHEMAV_CVC_LENGTH_VALID as c_int;
        }
    } else if (*facet).type_0 as c_uint
        == XML_SCHEMA_FACET_MINLENGTH as c_int as c_uint
    {
        if (len as c_ulong) < (*(*facet).val).value.decimal.lo {
            return XML_SCHEMAV_CVC_MINLENGTH_VALID as c_int;
        }
    } else if len as c_ulong > (*(*facet).val).value.decimal.lo {
        return XML_SCHEMAV_CVC_MAXLENGTH_VALID as c_int;
    }
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlSchemaValidateLengthFacet(
    mut type_0: xmlSchemaTypePtr,
    mut facet: xmlSchemaFacetPtr,
    mut value: *const xmlChar,
    mut val: xmlSchemaValPtr,
    mut length: *mut c_ulong,
) -> c_int {
    if type_0.is_null() {
        return -(1 as c_int);
    }
    return xmlSchemaValidateLengthFacetInternal(
        facet,
        (*type_0).builtInType as xmlSchemaValType,
        value,
        val,
        length,
        XML_SCHEMA_WHITESPACE_UNKNOWN,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateLengthFacetWhtsp(
    mut facet: xmlSchemaFacetPtr,
    mut valType: xmlSchemaValType,
    mut value: *const xmlChar,
    mut val: xmlSchemaValPtr,
    mut length: *mut c_ulong,
    mut ws: xmlSchemaWhitespaceValueType,
) -> c_int {
    return xmlSchemaValidateLengthFacetInternal(facet, valType, value, val, length, ws);
}
unsafe fn xmlSchemaValidateFacetInternal(
    mut facet: xmlSchemaFacetPtr,
    mut fws: xmlSchemaWhitespaceValueType,
    mut valType: xmlSchemaValType,
    mut value: *const xmlChar,
    mut val: xmlSchemaValPtr,
    mut ws: xmlSchemaWhitespaceValueType,
) -> c_int {
    let mut ret: c_int = 0;
    if facet.is_null() {
        return -(1 as c_int);
    }
    let mut current_block_100: u64;
    match (*facet).type_0 as c_uint {
        1006 => {
            if value.is_null() {
                return -(1 as c_int);
            }
            if !val.is_null()
                && !(*val).value.str_0.is_null()
                && ((*val).type_0 as c_uint
                    >= XML_SCHEMAS_STRING as c_int as c_uint
                    && (*val).type_0 as c_uint
                        <= XML_SCHEMAS_NORMSTRING as c_int as c_uint
                    || (*val).type_0 as c_uint
                        >= XML_SCHEMAS_TOKEN as c_int as c_uint
                        && (*val).type_0 as c_uint
                            <= XML_SCHEMAS_ENTITIES as c_int as c_uint
                        && (*val).type_0 as c_uint
                            != XML_SCHEMAS_QNAME as c_int as c_uint)
            {
                value = (*val).value.str_0;
            }
            ret = xmlRegexpExec((*facet).regexp, value);
            if ret == 1 as c_int {
                return 0 as c_int;
            }
            if ret == 0 as c_int {
                return XML_SCHEMAV_CVC_PATTERN_VALID as c_int;
            }
            return ret;
        }
        1003 => {
            ret = xmlSchemaCompareValues(val, (*facet).val);
            if ret == -(2 as c_int) {
                return -(1 as c_int);
            }
            if ret == -(1 as c_int) {
                return 0 as c_int;
            }
            return XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID as c_int;
        }
        1002 => {
            ret = xmlSchemaCompareValues(val, (*facet).val);
            if ret == -(2 as c_int) {
                return -(1 as c_int);
            }
            if ret == -(1 as c_int) || ret == 0 as c_int {
                return 0 as c_int;
            }
            return XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID as c_int;
        }
        1001 => {
            ret = xmlSchemaCompareValues(val, (*facet).val);
            if ret == -(2 as c_int) {
                return -(1 as c_int);
            }
            if ret == 1 as c_int {
                return 0 as c_int;
            }
            return XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID as c_int;
        }
        1000 => {
            ret = xmlSchemaCompareValues(val, (*facet).val);
            if ret == -(2 as c_int) {
                return -(1 as c_int);
            }
            if ret == 1 as c_int || ret == 0 as c_int {
                return 0 as c_int;
            }
            return XML_SCHEMAV_CVC_MININCLUSIVE_VALID as c_int;
        }
        1008 => return 0 as c_int,
        1007 => {
            if ws as c_uint
                == XML_SCHEMA_WHITESPACE_UNKNOWN as c_int as c_uint
            {
                if !(*facet).value.is_null() && xmlStrEqual((*facet).value, value) != 0 {
                    return 0 as c_int;
                }
            } else {
                ret = xmlSchemaCompareValuesWhtspExt(
                    (*(*facet).val).type_0,
                    (*facet).val,
                    (*facet).value,
                    fws,
                    valType,
                    val,
                    value,
                    ws,
                );
                if ret == -(2 as c_int) {
                    return -(1 as c_int);
                }
                if ret == 0 as c_int {
                    return 0 as c_int;
                }
            }
            return XML_SCHEMAV_CVC_ENUMERATION_VALID as c_int;
        }
        1009 => {
            if valType as c_uint
                == XML_SCHEMAS_QNAME as c_int as c_uint
                || valType as c_uint
                    == XML_SCHEMAS_NOTATION as c_int as c_uint
            {
                return 0 as c_int;
            }
            current_block_100 = 980989089337379490;
        }
        1010 | 1011 => {
            current_block_100 = 980989089337379490;
        }
        1004 | 1005 => {
            if (*facet).val.is_null()
                || (*(*facet).val).type_0 as c_uint
                    != XML_SCHEMAS_PINTEGER as c_int as c_uint
                    && (*(*facet).val).type_0 as c_uint
                        != XML_SCHEMAS_NNINTEGER as c_int as c_uint
                || (*(*facet).val).value.decimal.frac() as c_int
                    != 0 as c_int
            {
                return -(1 as c_int);
            }
            if val.is_null()
                || (*val).type_0 as c_uint
                    != XML_SCHEMAS_DECIMAL as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_INTEGER as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_NPINTEGER as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_NINTEGER as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_NNINTEGER as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_PINTEGER as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_INT as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_UINT as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_LONG as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_ULONG as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_SHORT as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_USHORT as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_BYTE as c_int as c_uint
                    && (*val).type_0 as c_uint
                        != XML_SCHEMAS_UBYTE as c_int as c_uint
            {
                return -(1 as c_int);
            }
            if (*facet).type_0 as c_uint
                == XML_SCHEMA_FACET_TOTALDIGITS as c_int as c_uint
            {
                if (*val).value.decimal.total() as c_ulong
                    > (*(*facet).val).value.decimal.lo
                {
                    return XML_SCHEMAV_CVC_TOTALDIGITS_VALID as c_int;
                }
            } else if (*facet).type_0 as c_uint
                == XML_SCHEMA_FACET_FRACTIONDIGITS as c_int as c_uint
            {
                if (*val).value.decimal.frac() as c_ulong
                    > (*(*facet).val).value.decimal.lo
                {
                    return XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID as c_int;
                }
            }
            current_block_100 = 12705158477165241210;
        }
        _ => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlschemastypes.c\0"
                    as *const u8 as *const c_char,
                5779 as c_int,
            );
            current_block_100 = 12705158477165241210;
        }
    }
    match current_block_100 {
        980989089337379490 => {
            let mut len: c_uint = 0 as c_uint;
            if valType as c_uint
                == XML_SCHEMAS_QNAME as c_int as c_uint
                || valType as c_uint
                    == XML_SCHEMAS_NOTATION as c_int as c_uint
            {
                return 0 as c_int;
            }
            if (*facet).val.is_null()
                || (*(*facet).val).type_0 as c_uint
                    != XML_SCHEMAS_DECIMAL as c_int as c_uint
                    && (*(*facet).val).type_0 as c_uint
                        != XML_SCHEMAS_NNINTEGER as c_int as c_uint
                || (*(*facet).val).value.decimal.frac() as c_int
                    != 0 as c_int
            {
                return -(1 as c_int);
            }
            if !val.is_null()
                && (*val).type_0 as c_uint
                    == XML_SCHEMAS_HEXBINARY as c_int as c_uint
            {
                len = (*val).value.hex.total;
            } else if !val.is_null()
                && (*val).type_0 as c_uint
                    == XML_SCHEMAS_BASE64BINARY as c_int as c_uint
            {
                len = (*val).value.base64.total;
            } else {
                match valType as c_uint {
                    1 | 2 => {
                        if ws as c_uint
                            == XML_SCHEMA_WHITESPACE_UNKNOWN as c_int
                                as c_uint
                        {
                            if valType as c_uint
                                == XML_SCHEMAS_STRING as c_int as c_uint
                            {
                                len = xmlUTF8Strlen(value) as c_uint;
                            } else {
                                len = xmlSchemaNormLen(value) as c_uint;
                            }
                        } else if !value.is_null() {
                            if ws as c_uint
                                == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int
                                    as c_uint
                            {
                                len = xmlSchemaNormLen(value) as c_uint;
                            } else {
                                len = xmlUTF8Strlen(value) as c_uint;
                            }
                        }
                    }
                    24 | 16 | 17 | 18 | 20 | 22 | 23 | 29 => {
                        if !value.is_null() {
                            len = xmlSchemaNormLen(value) as c_uint;
                        }
                    }
                    _ => {
                        (*__xmlGenericError())
                            .expect(
                                "non-null function pointer",
                            )(
                            *__xmlGenericErrorContext(),
                            b"Unimplemented block at %s:%d\n\0" as *const u8
                                as *const c_char,
                            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlschemastypes.c\0"
                                as *const u8 as *const c_char,
                            5728 as c_int,
                        );
                    }
                }
            }
            if (*facet).type_0 as c_uint
                == XML_SCHEMA_FACET_LENGTH as c_int as c_uint
            {
                if len as c_ulong != (*(*facet).val).value.decimal.lo {
                    return XML_SCHEMAV_CVC_LENGTH_VALID as c_int;
                }
            } else if (*facet).type_0 as c_uint
                == XML_SCHEMA_FACET_MINLENGTH as c_int as c_uint
            {
                if (len as c_ulong) < (*(*facet).val).value.decimal.lo {
                    return XML_SCHEMAV_CVC_MINLENGTH_VALID as c_int;
                }
            } else if len as c_ulong > (*(*facet).val).value.decimal.lo {
                return XML_SCHEMAV_CVC_MAXLENGTH_VALID as c_int;
            }
        }
        _ => {}
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateFacet(
    mut base: xmlSchemaTypePtr,
    mut facet: xmlSchemaFacetPtr,
    mut value: *const xmlChar,
    mut val: xmlSchemaValPtr,
) -> c_int {
    if !val.is_null() {
        return xmlSchemaValidateFacetInternal(
            facet,
            XML_SCHEMA_WHITESPACE_UNKNOWN,
            (*val).type_0,
            value,
            val,
            XML_SCHEMA_WHITESPACE_UNKNOWN,
        );
    } else if !base.is_null() {
        return xmlSchemaValidateFacetInternal(
            facet,
            XML_SCHEMA_WHITESPACE_UNKNOWN,
            (*base).builtInType as xmlSchemaValType,
            value,
            val,
            XML_SCHEMA_WHITESPACE_UNKNOWN,
        );
    }
    return -(1 as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSchemaValidateFacetWhtsp(
    mut facet: xmlSchemaFacetPtr,
    mut fws: xmlSchemaWhitespaceValueType,
    mut valType: xmlSchemaValType,
    mut value: *const xmlChar,
    mut val: xmlSchemaValPtr,
    mut ws: xmlSchemaWhitespaceValueType,
) -> c_int {
    return xmlSchemaValidateFacetInternal(facet, fws, valType, value, val, ws);
}
pub unsafe fn xmlSchemaGetCanonValue(
    mut val: xmlSchemaValPtr,
    mut retValue: *mut *const xmlChar,
) -> c_int {
    if retValue.is_null() || val.is_null() {
        return -(1 as c_int);
    }
    *retValue = ::core::ptr::null::<xmlChar>();
    match (*val).type_0 as c_uint {
        1 => {
            if (*val).value.str_0.is_null() {
                *retValue =
                    xmlStrdup(b"\0" as *const u8 as *const c_char as *mut xmlChar);
            } else {
                *retValue = xmlStrdup((*val).value.str_0 as *const xmlChar);
            }
        }
        2 => {
            if (*val).value.str_0.is_null() {
                *retValue =
                    xmlStrdup(b"\0" as *const u8 as *const c_char as *mut xmlChar);
            } else {
                *retValue = xmlSchemaWhiteSpaceReplace((*val).value.str_0 as *const xmlChar);
                if (*retValue).is_null() {
                    *retValue = xmlStrdup((*val).value.str_0 as *const xmlChar);
                }
            }
        }
        16 | 17 | 18 | 20 | 22 | 23 | 24 | 26 | 28 | 29 => {
            if (*val).value.str_0.is_null() {
                return -(1 as c_int);
            }
            *retValue = xmlSchemaCollapseString((*val).value.str_0);
            if (*retValue).is_null() {
                *retValue = xmlStrdup((*val).value.str_0 as *const xmlChar);
            }
        }
        21 => {
            if (*val).value.qname.uri.is_null() {
                *retValue = xmlStrdup((*val).value.qname.name);
                return 0 as c_int;
            } else {
                *retValue =
                    xmlStrdup(b"{\0" as *const u8 as *const c_char as *mut xmlChar);
                *retValue = xmlStrcat(*retValue as *mut xmlChar, (*val).value.qname.uri);
                *retValue = xmlStrcat(
                    *retValue as *mut xmlChar,
                    b"}\0" as *const u8 as *const c_char as *mut xmlChar,
                );
                *retValue = xmlStrcat(*retValue as *mut xmlChar, (*val).value.qname.uri);
            }
        }
        3 => {
            if (*val).value.decimal.total() as c_int == 1 as c_int
                && (*val).value.decimal.lo == 0 as c_ulong
            {
                *retValue =
                    xmlStrdup(b"0.0\0" as *const u8 as *const c_char as *mut xmlChar);
            } else {
                let mut dec: xmlSchemaValDecimal = (*val).value.decimal;
                let mut bufsize: c_int = 0;
                let mut buf: *mut c_char =
                    ::core::ptr::null_mut::<c_char>();
                let mut offs: *mut c_char =
                    ::core::ptr::null_mut::<c_char>();
                bufsize = dec.total() as c_int + 2 as c_int;
                if dec.sign() != 0 {
                    bufsize += 1;
                }
                if dec.frac() as c_int == 0 as c_int
                    || dec.frac() as c_int == dec.total() as c_int
                {
                    bufsize += 1;
                }
                buf = xmlMalloc.expect("non-null function pointer")(bufsize as size_t)
                    as *mut c_char;
                if buf.is_null() {
                    return -(1 as c_int);
                }
                offs = buf;
                if dec.sign() != 0 {
                    let fresh27 = offs;
                    offs = offs.offset(1);
                    *fresh27 = '-' as i32 as c_char;
                }
                if dec.frac() as c_int == dec.total() as c_int {
                    let fresh28 = offs;
                    offs = offs.offset(1);
                    *fresh28 = '0' as i32 as c_char;
                    let fresh29 = offs;
                    offs = offs.offset(1);
                    *fresh29 = '.' as i32 as c_char;
                }
                if dec.hi != 0 as c_ulong {
                    snprintf(
                        offs,
                        (bufsize as c_long
                            - offs.offset_from(buf) as c_long)
                            as size_t,
                        b"%lu%lu%lu\0" as *const u8 as *const c_char,
                        dec.hi,
                        dec.mi,
                        dec.lo,
                    );
                } else if dec.mi != 0 as c_ulong {
                    snprintf(
                        offs,
                        (bufsize as c_long
                            - offs.offset_from(buf) as c_long)
                            as size_t,
                        b"%lu%lu\0" as *const u8 as *const c_char,
                        dec.mi,
                        dec.lo,
                    );
                } else {
                    snprintf(
                        offs,
                        (bufsize as c_long
                            - offs.offset_from(buf) as c_long)
                            as size_t,
                        b"%lu\0" as *const u8 as *const c_char,
                        dec.lo,
                    );
                }
                if dec.frac() as c_int != 0 as c_int {
                    if dec.frac() as c_int != dec.total() as c_int {
                        let mut diff: c_int =
                            dec.total() as c_int - dec.frac() as c_int;
                        memmove(
                            offs.offset(diff as isize)
                                .offset(1 as c_int as isize)
                                as *mut c_void,
                            offs.offset(diff as isize) as *const c_void,
                            (dec.frac() as c_int + 1 as c_int) as size_t,
                        );
                        *offs.offset(diff as isize) = '.' as i32 as c_char;
                    } else {
                        let mut i: c_uint = 0 as c_uint;
                        while *offs.offset(i as isize) as c_int
                            != 0 as c_int
                        {
                            i = i.wrapping_add(1);
                        }
                        if i < dec.total() {
                            memmove(
                                offs.offset(dec.total().wrapping_sub(i) as isize)
                                    as *mut c_void,
                                offs as *const c_void,
                                i.wrapping_add(1 as c_uint) as size_t,
                            );
                            memset(
                                offs as *mut c_void,
                                '0' as i32,
                                dec.total().wrapping_sub(i) as size_t,
                            );
                        }
                    }
                } else {
                    offs = buf
                        .offset(bufsize as isize)
                        .offset(-(1 as c_int as isize));
                    let fresh30 = offs;
                    offs = offs.offset(-1);
                    *fresh30 = 0 as c_char;
                    let fresh31 = offs;
                    offs = offs.offset(-1);
                    *fresh31 = '0' as i32 as c_char;
                    let fresh32 = offs;
                    offs = offs.offset(-1);
                    *fresh32 = '.' as i32 as c_char;
                }
                *retValue = buf as *mut xmlChar;
            }
        }
        30 | 34 | 31 | 32 | 33 | 37 | 41 | 39 | 35 | 36 | 38 | 40 | 42 => {
            if (*val).value.decimal.total() as c_int == 1 as c_int
                && (*val).value.decimal.lo == 0 as c_ulong
            {
                *retValue =
                    xmlStrdup(b"0\0" as *const u8 as *const c_char as *mut xmlChar);
            } else {
                let mut dec_0: xmlSchemaValDecimal = (*val).value.decimal;
                let mut bufsize_0: c_int =
                    dec_0.total() as c_int + 1 as c_int;
                if dec_0.sign() != 0 {
                    bufsize_0 += 1;
                }
                *retValue = xmlMalloc.expect("non-null function pointer")(bufsize_0 as size_t)
                    as *const xmlChar;
                if (*retValue).is_null() {
                    return -(1 as c_int);
                }
                if dec_0.hi != 0 as c_ulong {
                    if dec_0.sign() != 0 {
                        snprintf(
                            *retValue as *mut c_char,
                            bufsize_0 as size_t,
                            b"-%lu%lu%lu\0" as *const u8 as *const c_char,
                            dec_0.hi,
                            dec_0.mi,
                            dec_0.lo,
                        );
                    } else {
                        snprintf(
                            *retValue as *mut c_char,
                            bufsize_0 as size_t,
                            b"%lu%lu%lu\0" as *const u8 as *const c_char,
                            dec_0.hi,
                            dec_0.mi,
                            dec_0.lo,
                        );
                    }
                } else if dec_0.mi != 0 as c_ulong {
                    if dec_0.sign() != 0 {
                        snprintf(
                            *retValue as *mut c_char,
                            bufsize_0 as size_t,
                            b"-%lu%lu\0" as *const u8 as *const c_char,
                            dec_0.mi,
                            dec_0.lo,
                        );
                    } else {
                        snprintf(
                            *retValue as *mut c_char,
                            bufsize_0 as size_t,
                            b"%lu%lu\0" as *const u8 as *const c_char,
                            dec_0.mi,
                            dec_0.lo,
                        );
                    }
                } else if dec_0.sign() != 0 {
                    snprintf(
                        *retValue as *mut c_char,
                        bufsize_0 as size_t,
                        b"-%lu\0" as *const u8 as *const c_char,
                        dec_0.lo,
                    );
                } else {
                    snprintf(
                        *retValue as *mut c_char,
                        bufsize_0 as size_t,
                        b"%lu\0" as *const u8 as *const c_char,
                        dec_0.lo,
                    );
                }
            }
        }
        15 => {
            if (*val).value.b != 0 {
                *retValue =
                    xmlStrdup(b"true\0" as *const u8 as *const c_char as *mut xmlChar);
            } else {
                *retValue = xmlStrdup(
                    b"false\0" as *const u8 as *const c_char as *mut xmlChar,
                );
            }
        }
        12 => {
            let mut buf_0: [c_char; 100] = [0; 100];
            let mut year: c_ulong = 0;
            let mut mon: c_ulong = 0;
            let mut day: c_ulong = 0;
            let mut hour: c_ulong = 0 as c_ulong;
            let mut min: c_ulong = 0 as c_ulong;
            let mut sec: c_double = 0 as c_int as c_double;
            let mut left: c_double = 0.;
            year = floor(
                labs((*val).value.dur.mon) as c_double
                    / 12 as c_int as c_double,
            ) as c_ulong;
            mon = (labs((*val).value.dur.mon) as c_ulong)
                .wrapping_sub((12 as c_ulong).wrapping_mul(year));
            day = floor(
                fabs((*val).value.dur.sec) / 86400 as c_int as c_double,
            ) as c_ulong;
            left = fabs((*val).value.dur.sec)
                - day.wrapping_mul(86400 as c_ulong) as c_double;
            if left > 0 as c_int as c_double {
                hour = floor(left / 3600 as c_int as c_double)
                    as c_ulong;
                left =
                    left - hour.wrapping_mul(3600 as c_ulong) as c_double;
                if left > 0 as c_int as c_double {
                    min = floor(left / 60 as c_int as c_double)
                        as c_ulong;
                    sec = left
                        - min.wrapping_mul(60 as c_ulong) as c_double;
                }
            }
            if (*val).value.dur.mon < 0 as c_long
                || (*val).value.dur.sec < 0 as c_int as c_double
            {
                snprintf(
                    &raw mut buf_0 as *mut c_char,
                    100 as size_t,
                    b"P%luY%luM%luDT%luH%luM%.14gS\0" as *const u8 as *const c_char,
                    year,
                    mon,
                    day,
                    hour,
                    min,
                    sec,
                );
            } else {
                snprintf(
                    &raw mut buf_0 as *mut c_char,
                    100 as size_t,
                    b"-P%luY%luM%luDT%luH%luM%.14gS\0" as *const u8 as *const c_char,
                    year,
                    mon,
                    day,
                    hour,
                    min,
                    sec,
                );
            }
            *retValue = xmlStrdup(&raw mut buf_0 as *mut c_char as *mut xmlChar);
        }
        8 => {
            let mut buf_1: [c_char; 30] = [0; 30];
            snprintf(
                &raw mut buf_1 as *mut c_char,
                30 as size_t,
                b"%04ld\0" as *const u8 as *const c_char,
                (*val).value.date.year,
            );
            *retValue = xmlStrdup(&raw mut buf_1 as *mut c_char as *mut xmlChar);
        }
        6 => {
            *retValue =
                xmlMalloc.expect("non-null function pointer")(6 as size_t) as *const xmlChar;
            if (*retValue).is_null() {
                return -(1 as c_int);
            }
            snprintf(
                *retValue as *mut c_char,
                6 as size_t,
                b"--%02u\0" as *const u8 as *const c_char,
                (*val).value.date.mon() as c_int,
            );
        }
        5 => {
            *retValue =
                xmlMalloc.expect("non-null function pointer")(6 as size_t) as *const xmlChar;
            if (*retValue).is_null() {
                return -(1 as c_int);
            }
            snprintf(
                *retValue as *mut c_char,
                6 as size_t,
                b"---%02u\0" as *const u8 as *const c_char,
                (*val).value.date.day() as c_int,
            );
        }
        7 => {
            *retValue =
                xmlMalloc.expect("non-null function pointer")(8 as size_t) as *const xmlChar;
            if (*retValue).is_null() {
                return -(1 as c_int);
            }
            snprintf(
                *retValue as *mut c_char,
                8 as size_t,
                b"--%02u-%02u\0" as *const u8 as *const c_char,
                (*val).value.date.mon() as c_int,
                (*val).value.date.day() as c_int,
            );
        }
        9 => {
            let mut buf_2: [c_char; 35] = [0; 35];
            if (*val).value.date.year < 0 as c_long {
                snprintf(
                    &raw mut buf_2 as *mut c_char,
                    35 as size_t,
                    b"-%04ld-%02u\0" as *const u8 as *const c_char,
                    labs((*val).value.date.year),
                    (*val).value.date.mon() as c_int,
                );
            } else {
                snprintf(
                    &raw mut buf_2 as *mut c_char,
                    35 as size_t,
                    b"%04ld-%02u\0" as *const u8 as *const c_char,
                    (*val).value.date.year,
                    (*val).value.date.mon() as c_int,
                );
            }
            *retValue = xmlStrdup(&raw mut buf_2 as *mut c_char as *mut xmlChar);
        }
        4 => {
            let mut buf_3: [c_char; 30] = [0; 30];
            if (*val).value.date.tz_flag() != 0 {
                let mut norm: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
                norm =
                    xmlSchemaDateNormalize(val, 0 as c_int as c_double);
                if norm.is_null() {
                    return -(1 as c_int);
                }
                snprintf(
                    &raw mut buf_3 as *mut c_char,
                    30 as size_t,
                    b"%02u:%02u:%02.14gZ\0" as *const u8 as *const c_char,
                    (*norm).value.date.hour() as c_int,
                    (*norm).value.date.min() as c_int,
                    (*norm).value.date.sec,
                );
                xmlSchemaFreeValue(norm);
            } else {
                snprintf(
                    &raw mut buf_3 as *mut c_char,
                    30 as size_t,
                    b"%02u:%02u:%02.14g\0" as *const u8 as *const c_char,
                    (*val).value.date.hour() as c_int,
                    (*val).value.date.min() as c_int,
                    (*val).value.date.sec,
                );
            }
            *retValue = xmlStrdup(&raw mut buf_3 as *mut c_char as *mut xmlChar);
        }
        10 => {
            let mut buf_4: [c_char; 30] = [0; 30];
            if (*val).value.date.tz_flag() != 0 {
                let mut norm_0: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
                norm_0 =
                    xmlSchemaDateNormalize(val, 0 as c_int as c_double);
                if norm_0.is_null() {
                    return -(1 as c_int);
                }
                snprintf(
                    &raw mut buf_4 as *mut c_char,
                    30 as size_t,
                    b"%04ld-%02u-%02uZ\0" as *const u8 as *const c_char,
                    (*norm_0).value.date.year,
                    (*norm_0).value.date.mon() as c_int,
                    (*norm_0).value.date.day() as c_int,
                );
                xmlSchemaFreeValue(norm_0);
            } else {
                snprintf(
                    &raw mut buf_4 as *mut c_char,
                    30 as size_t,
                    b"%04ld-%02u-%02u\0" as *const u8 as *const c_char,
                    (*val).value.date.year,
                    (*val).value.date.mon() as c_int,
                    (*val).value.date.day() as c_int,
                );
            }
            *retValue = xmlStrdup(&raw mut buf_4 as *mut c_char as *mut xmlChar);
        }
        11 => {
            let mut buf_5: [c_char; 50] = [0; 50];
            if (*val).value.date.tz_flag() != 0 {
                let mut norm_1: xmlSchemaValPtr = ::core::ptr::null_mut::<xmlSchemaVal>();
                norm_1 =
                    xmlSchemaDateNormalize(val, 0 as c_int as c_double);
                if norm_1.is_null() {
                    return -(1 as c_int);
                }
                snprintf(
                    &raw mut buf_5 as *mut c_char,
                    50 as size_t,
                    b"%04ld-%02u-%02uT%02u:%02u:%02.14gZ\0" as *const u8
                        as *const c_char,
                    (*norm_1).value.date.year,
                    (*norm_1).value.date.mon() as c_int,
                    (*norm_1).value.date.day() as c_int,
                    (*norm_1).value.date.hour() as c_int,
                    (*norm_1).value.date.min() as c_int,
                    (*norm_1).value.date.sec,
                );
                xmlSchemaFreeValue(norm_1);
            } else {
                snprintf(
                    &raw mut buf_5 as *mut c_char,
                    50 as size_t,
                    b"%04ld-%02u-%02uT%02u:%02u:%02.14g\0" as *const u8
                        as *const c_char,
                    (*val).value.date.year,
                    (*val).value.date.mon() as c_int,
                    (*val).value.date.day() as c_int,
                    (*val).value.date.hour() as c_int,
                    (*val).value.date.min() as c_int,
                    (*val).value.date.sec,
                );
            }
            *retValue = xmlStrdup(&raw mut buf_5 as *mut c_char as *mut xmlChar);
        }
        43 => {
            *retValue = xmlStrdup((*val).value.hex.str_0);
        }
        44 => {
            *retValue = xmlStrdup((*val).value.base64.str_0);
        }
        13 => {
            let mut buf_6: [c_char; 30] = [0; 30];
            snprintf(
                &raw mut buf_6 as *mut c_char,
                30 as size_t,
                b"%01.14e\0" as *const u8 as *const c_char,
                (*val).value.f as c_double,
            );
            *retValue = xmlStrdup(&raw mut buf_6 as *mut c_char as *mut xmlChar);
        }
        14 => {
            let mut buf_7: [c_char; 40] = [0; 40];
            snprintf(
                &raw mut buf_7 as *mut c_char,
                40 as size_t,
                b"%01.14e\0" as *const u8 as *const c_char,
                (*val).value.d,
            );
            *retValue = xmlStrdup(&raw mut buf_7 as *mut c_char as *mut xmlChar);
        }
        _ => {
            *retValue =
                xmlStrdup(b"???\0" as *const u8 as *const c_char as *mut xmlChar);
            return 1 as c_int;
        }
    }
    if (*retValue).is_null() {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[inline]
pub unsafe fn xmlSchemaGetCanonValueWhtsp(
    mut val: xmlSchemaValPtr,
    mut retValue: *mut *const xmlChar,
    mut ws: xmlSchemaWhitespaceValueType,
) -> c_int {
    if retValue.is_null() || val.is_null() {
        return -(1 as c_int);
    }
    if ws as c_uint
        == XML_SCHEMA_WHITESPACE_UNKNOWN as c_int as c_uint
        || ws as c_uint
            > XML_SCHEMA_WHITESPACE_COLLAPSE as c_int as c_uint
    {
        return -(1 as c_int);
    }
    *retValue = ::core::ptr::null::<xmlChar>();
    match (*val).type_0 as c_uint {
        1 => {
            if (*val).value.str_0.is_null() {
                *retValue =
                    xmlStrdup(b"\0" as *const u8 as *const c_char as *mut xmlChar);
            } else if ws as c_uint
                == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int as c_uint
            {
                *retValue = xmlSchemaCollapseString((*val).value.str_0);
            } else if ws as c_uint
                == XML_SCHEMA_WHITESPACE_REPLACE as c_int as c_uint
            {
                *retValue = xmlSchemaWhiteSpaceReplace((*val).value.str_0);
            }
            if (*retValue).is_null() {
                *retValue = xmlStrdup((*val).value.str_0);
            }
        }
        2 => {
            if (*val).value.str_0.is_null() {
                *retValue =
                    xmlStrdup(b"\0" as *const u8 as *const c_char as *mut xmlChar);
            } else {
                if ws as c_uint
                    == XML_SCHEMA_WHITESPACE_COLLAPSE as c_int as c_uint
                {
                    *retValue = xmlSchemaCollapseString((*val).value.str_0);
                } else {
                    *retValue = xmlSchemaWhiteSpaceReplace((*val).value.str_0);
                }
                if (*retValue).is_null() {
                    *retValue = xmlStrdup((*val).value.str_0);
                }
            }
        }
        _ => return xmlSchemaGetCanonValue(val, retValue),
    }
    return 0 as c_int;
}
#[inline]
pub fn xmlSchemaGetValType(mut val: xmlSchemaValPtr) -> xmlSchemaValType { unsafe {
    if val.is_null() {
        return XML_SCHEMAS_UNKNOWN;
    }
    return (*val).type_0;
} }

pub const LONG_MIN: c_long = -__LONG_MAX__ - 1 as c_long;

