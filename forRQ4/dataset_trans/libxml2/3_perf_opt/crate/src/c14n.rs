use core::ffi::*;
use crate::src::buf::xmlBufContent;
use crate::src::buf::xmlBufUse;
use crate::src::buf::xmlBufWriteQuotedString;
use crate::src::uri::xmlBuildURI;
use crate::src::uri::xmlFreeURI;
use crate::src::list::xmlListCreate;
use crate::src::list::xmlListDelete;
use crate::src::list::xmlListInsert;
use crate::src::list::xmlListSearch;
use crate::src::list::xmlListWalk;
use crate::src::uri::xmlParseURI;
use crate::src::xmlstring::xmlStrEqual;
use crate::src::xmlstring::xmlStrcat;
use crate::src::xmlstring::xmlStrcmp;
use crate::src::xmlstring::xmlStrndup;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::dict::_xmlDict;
pub use crate::src::buf::_xmlBuf;
pub use crate::src::list::_xmlLink;
pub use crate::src::list::_xmlList;
extern "C" {
    fn xmlNewNsProp(
        node: xmlNodePtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        value: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlFreePropList(cur: xmlAttrPtr);
    fn xmlSearchNs(doc: xmlDocPtr, node: xmlNodePtr, nameSpace: *const xmlChar) -> xmlNsPtr;
    fn xmlHasNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> xmlAttrPtr;
    fn xmlNodeListGetString(
        doc: xmlDocPtr,
        list: *const xmlNode,
        inLine: c_int,
    ) -> *mut xmlChar;
    fn xmlAllocOutputBuffer(encoder: xmlCharEncodingHandlerPtr) -> xmlOutputBufferPtr;
    fn xmlOutputBufferCreateFilename(
        URI: *const c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: c_int,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferWriteString(
        out: xmlOutputBufferPtr,
        str: *const c_char,
    ) -> c_int;
    fn xmlOutputBufferFlush(out: xmlOutputBufferPtr) -> c_int;
    fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> c_int;
    fn xmlXPathNodeSetContains(cur: xmlNodeSetPtr, val: xmlNodePtr) -> c_int;
}

pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;

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

pub type xmlLink = _xmlLink;
pub type xmlLinkPtr = *mut xmlLink;
pub type xmlList = _xmlList;
pub type xmlListPtr = *mut xmlList;
pub type xmlListDeallocator = Option<unsafe extern "C" fn(xmlLinkPtr) -> ()>;

pub type xmlNodeSetPtr = *mut xmlNodeSet;
pub type xmlNodeSet = _xmlNodeSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeSet {
    pub nodeNr: c_int,
    pub nodeMax: c_int,
    pub nodeTab: *mut xmlNodePtr,
}
pub type xmlC14NMode = c_uint;
pub const XML_C14N_1_1: xmlC14NMode = 2;
pub const XML_C14N_EXCLUSIVE_1_0: xmlC14NMode = 1;
pub const XML_C14N_1_0: xmlC14NMode = 0;
pub type xmlC14NIsVisibleCallback = Option<
    unsafe extern "C" fn(*mut c_void, xmlNodePtr, xmlNodePtr) -> c_int,
>;
pub type xmlC14NCtxPtr = *mut _xmlC14NCtx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlC14NCtx {
    pub doc: xmlDocPtr,
    pub is_visible_callback: xmlC14NIsVisibleCallback,
    pub user_data: *mut c_void,
    pub with_comments: c_int,
    pub buf: xmlOutputBufferPtr,
    pub pos: xmlC14NPosition,
    pub parent_is_doc: c_int,
    pub ns_rendered: xmlC14NVisibleNsStackPtr,
    pub mode: xmlC14NMode,
    pub inclusive_ns_prefixes: *mut *mut xmlChar,
    pub error: c_int,
}
pub type xmlC14NVisibleNsStackPtr = *mut _xmlC14NVisibleNsStack;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlC14NVisibleNsStack {
    pub nsCurEnd: c_int,
    pub nsPrevStart: c_int,
    pub nsPrevEnd: c_int,
    pub nsMax: c_int,
    pub nsTab: *mut xmlNsPtr,
    pub nodeTab: *mut xmlNodePtr,
}
pub type xmlC14NPosition = c_uint;
pub const XMLC14N_AFTER_DOCUMENT_ELEMENT: xmlC14NPosition = 2;
pub const XMLC14N_INSIDE_DOCUMENT_ELEMENT: xmlC14NPosition = 1;
pub const XMLC14N_BEFORE_DOCUMENT_ELEMENT: xmlC14NPosition = 0;
pub type xmlC14NVisibleNsStack = _xmlC14NVisibleNsStack;
pub type xmlC14NNormalizationMode = c_uint;
pub const XMLC14N_NORMALIZE_TEXT: xmlC14NNormalizationMode = 3;
pub const XMLC14N_NORMALIZE_PI: xmlC14NNormalizationMode = 2;
pub const XMLC14N_NORMALIZE_COMMENT: xmlC14NNormalizationMode = 1;
pub const XMLC14N_NORMALIZE_ATTR: xmlC14NNormalizationMode = 0;
pub type xmlC14NCtx = _xmlC14NCtx;

pub const XML_XML_NAMESPACE: *const xmlChar = b"http://www.w3.org/XML/1998/namespace\0" as *const u8
    as *const c_char as *const xmlChar;
unsafe fn xmlC14NErrMemory(mut extra: *const c_char) {
    __xmlRaiseError(
        None,
        None,
        NULL,
        NULL,
        NULL,
        XML_FROM_C14N as c_int,
        XML_ERR_NO_MEMORY as c_int,
        XML_ERR_ERROR,
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
}
unsafe fn xmlC14NErrParam(mut extra: *const c_char) {
    __xmlRaiseError(
        None,
        None,
        NULL,
        NULL,
        NULL,
        XML_FROM_C14N as c_int,
        XML_ERR_INTERNAL_ERROR as c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        extra,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        b"Invalid parameter : %s\n\0" as *const u8 as *const c_char,
        extra,
    );
}
unsafe fn xmlC14NErrInternal(mut extra: *const c_char) {
    __xmlRaiseError(
        None,
        None,
        NULL,
        NULL,
        NULL,
        XML_FROM_C14N as c_int,
        XML_ERR_INTERNAL_ERROR as c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        extra,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        b"Internal error : %s\n\0" as *const u8 as *const c_char,
        extra,
    );
}
unsafe fn xmlC14NErrInvalidNode(
    mut node_type: *const c_char,
    mut extra: *const c_char,
) {
    __xmlRaiseError(
        None,
        None,
        NULL,
        NULL,
        NULL,
        XML_FROM_C14N as c_int,
        XML_C14N_INVALID_NODE as c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        extra,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        b"Node %s is invalid here : %s\n\0" as *const u8 as *const c_char,
        node_type,
        extra,
    );
}
unsafe fn xmlC14NErrUnknownNode(
    mut node_type: c_int,
    mut extra: *const c_char,
) {
    __xmlRaiseError(
        None,
        None,
        NULL,
        NULL,
        NULL,
        XML_FROM_C14N as c_int,
        XML_C14N_UNKNOW_NODE as c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        extra,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        b"Unknown node type %d found : %s\n\0" as *const u8 as *const c_char,
        node_type,
        extra,
    );
}
unsafe fn xmlC14NErrRelativeNamespace(mut ns_uri: *const c_char) {
    __xmlRaiseError(
        None,
        None,
        NULL,
        NULL,
        NULL,
        XML_FROM_C14N as c_int,
        XML_C14N_RELATIVE_NAMESPACE as c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        b"Relative namespace UR is invalid here : %s\n\0" as *const u8
            as *const c_char,
        ns_uri,
    );
}
unsafe fn xmlC14NErr(
    mut ctxt: xmlC14NCtxPtr,
    mut node: xmlNodePtr,
    mut error: c_int,
    mut msg: *const c_char,
) {
    if !ctxt.is_null() {
        (*ctxt).error = error;
    }
    __xmlRaiseError(
        None,
        None,
        NULL,
        ctxt as *mut c_void,
        node as *mut c_void,
        XML_FROM_C14N as c_int,
        error,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        0 as c_int,
        b"%s\0" as *const u8 as *const c_char,
        msg,
    );
}
pub const XML_NAMESPACES_DEFAULT: c_int = 16 as c_int;
unsafe extern "C" fn xmlC14NIsNodeInNodeset(
    mut user_data: *mut c_void,
    mut node: xmlNodePtr,
    mut parent: xmlNodePtr,
) -> c_int {
    let mut nodes: xmlNodeSetPtr = user_data as xmlNodeSetPtr;
    if !nodes.is_null() && !node.is_null() {
        if (*node).type_0 as c_uint
            != XML_NAMESPACE_DECL as c_int as c_uint
        {
            return xmlXPathNodeSetContains(nodes, node);
        } else {
            let mut ns: xmlNs = xmlNs {
                next: ::core::ptr::null_mut::<_xmlNs>(),
                type_0: 0 as xmlNsType,
                href: ::core::ptr::null::<xmlChar>(),
                prefix: ::core::ptr::null::<xmlChar>(),
                _private: ::core::ptr::null_mut::<c_void>(),
                context: ::core::ptr::null_mut::<_xmlDoc>(),
            };
            memcpy(
                &raw mut ns as *mut c_void,
                node as *const c_void,
                ::core::mem::size_of::<xmlNs>() as size_t,
            );
            if !parent.is_null()
                && (*parent).type_0 as c_uint
                    == XML_ATTRIBUTE_NODE as c_int as c_uint
            {
                ns.next = (*parent).parent as xmlNsPtr as *mut _xmlNs;
            } else {
                ns.next = parent as xmlNsPtr as *mut _xmlNs;
            }
            return xmlXPathNodeSetContains(nodes, &raw mut ns as xmlNodePtr);
        }
    }
    return 1 as c_int;
}
fn xmlC14NVisibleNsStackCreate() -> xmlC14NVisibleNsStackPtr { unsafe {
    let mut ret: xmlC14NVisibleNsStackPtr = ::core::ptr::null_mut::<_xmlC14NVisibleNsStack>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlC14NVisibleNsStack>() as size_t,
    ) as xmlC14NVisibleNsStackPtr;
    if ret.is_null() {
        xmlC14NErrMemory(b"creating namespaces stack\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<_xmlC14NVisibleNsStack>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlC14NVisibleNsStack>() as size_t,
    );
    return ret;
} }
fn xmlC14NVisibleNsStackDestroy(mut cur: xmlC14NVisibleNsStackPtr) { unsafe {
    if cur.is_null() {
        xmlC14NErrParam(
            b"destroying namespaces stack\0" as *const u8 as *const c_char,
        );
        return;
    }
    if !(*cur).nsTab.is_null() {
        memset(
            (*cur).nsTab as *mut c_void,
            0 as c_int,
            ((*cur).nsMax as size_t).wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
        );
        xmlFree.expect("non-null function pointer")((*cur).nsTab as *mut c_void);
    }
    if !(*cur).nodeTab.is_null() {
        memset(
            (*cur).nodeTab as *mut c_void,
            0 as c_int,
            ((*cur).nsMax as size_t).wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        xmlFree.expect("non-null function pointer")((*cur).nodeTab as *mut c_void);
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlC14NVisibleNsStack>() as size_t,
    );
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
} }
fn xmlC14NVisibleNsStackAdd(
    mut cur: xmlC14NVisibleNsStackPtr,
    mut ns: xmlNsPtr,
    mut node: xmlNodePtr,
) { unsafe {
    if cur.is_null()
        || (*cur).nsTab.is_null() && !(*cur).nodeTab.is_null()
        || !(*cur).nsTab.is_null() && (*cur).nodeTab.is_null()
    {
        xmlC14NErrParam(b"adding namespace to stack\0" as *const u8 as *const c_char);
        return;
    }
    if (*cur).nsTab.is_null() && (*cur).nodeTab.is_null() {
        (*cur).nsTab = xmlMalloc.expect("non-null function pointer")(
            (XML_NAMESPACES_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
        ) as *mut xmlNsPtr;
        (*cur).nodeTab = xmlMalloc.expect("non-null function pointer")(
            (XML_NAMESPACES_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if (*cur).nsTab.is_null() || (*cur).nodeTab.is_null() {
            xmlC14NErrMemory(b"adding node to stack\0" as *const u8 as *const c_char);
            return;
        }
        memset(
            (*cur).nsTab as *mut c_void,
            0 as c_int,
            (XML_NAMESPACES_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
        );
        memset(
            (*cur).nodeTab as *mut c_void,
            0 as c_int,
            (XML_NAMESPACES_DEFAULT as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        (*cur).nsMax = XML_NAMESPACES_DEFAULT;
    } else if (*cur).nsMax == (*cur).nsCurEnd {
        let mut tmp: *mut c_void = ::core::ptr::null_mut::<c_void>();
        let mut tmpSize: c_int = 0;
        tmpSize = 2 as c_int * (*cur).nsMax;
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*cur).nsTab as *mut c_void,
            (tmpSize as size_t).wrapping_mul(::core::mem::size_of::<xmlNsPtr>() as size_t),
        );
        if tmp.is_null() {
            xmlC14NErrMemory(b"adding node to stack\0" as *const u8 as *const c_char);
            return;
        }
        (*cur).nsTab = tmp as *mut xmlNsPtr;
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*cur).nodeTab as *mut c_void,
            (tmpSize as size_t).wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        );
        if tmp.is_null() {
            xmlC14NErrMemory(b"adding node to stack\0" as *const u8 as *const c_char);
            return;
        }
        (*cur).nodeTab = tmp as *mut xmlNodePtr;
        (*cur).nsMax = tmpSize;
    }
    let ref mut fresh37 = *(*cur).nsTab.offset((*cur).nsCurEnd as isize);
    *fresh37 = ns;
    let ref mut fresh38 = *(*cur).nodeTab.offset((*cur).nsCurEnd as isize);
    *fresh38 = node;
    (*cur).nsCurEnd += 1;
} }
fn xmlC14NVisibleNsStackSave(
    mut cur: xmlC14NVisibleNsStackPtr,
    mut state: xmlC14NVisibleNsStackPtr,
) { unsafe {
    if cur.is_null() || state.is_null() {
        xmlC14NErrParam(b"saving namespaces stack\0" as *const u8 as *const c_char);
        return;
    }
    (*state).nsCurEnd = (*cur).nsCurEnd;
    (*state).nsPrevStart = (*cur).nsPrevStart;
    (*state).nsPrevEnd = (*cur).nsPrevEnd;
} }
fn xmlC14NVisibleNsStackRestore(
    mut cur: xmlC14NVisibleNsStackPtr,
    mut state: xmlC14NVisibleNsStackPtr,
) { unsafe {
    if cur.is_null() || state.is_null() {
        xmlC14NErrParam(b"restoring namespaces stack\0" as *const u8 as *const c_char);
        return;
    }
    (*cur).nsCurEnd = (*state).nsCurEnd;
    (*cur).nsPrevStart = (*state).nsPrevStart;
    (*cur).nsPrevEnd = (*state).nsPrevEnd;
} }
fn xmlC14NVisibleNsStackShift(mut cur: xmlC14NVisibleNsStackPtr) { unsafe {
    if cur.is_null() {
        xmlC14NErrParam(b"shifting namespaces stack\0" as *const u8 as *const c_char);
        return;
    }
    (*cur).nsPrevStart = (*cur).nsPrevEnd;
    (*cur).nsPrevEnd = (*cur).nsCurEnd;
} }
unsafe fn xmlC14NStrEqual(
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) -> c_int {
    if str1 == str2 {
        return 1 as c_int;
    }
    if str1.is_null() {
        return (*str2 as c_int == '\0' as i32) as c_int;
    }
    if str2.is_null() {
        return (*str1 as c_int == '\0' as i32) as c_int;
    }
    loop {
        let fresh35 = str1;
        str1 = str1.offset(1);
        if *fresh35 as c_int != *str2 as c_int {
            return 0 as c_int;
        }
        let fresh36 = str2;
        str2 = str2.offset(1);
        if !(*fresh36 != 0) {
            break;
        }
    }
    return 1 as c_int;
}
fn xmlC14NVisibleNsStackFind(
    mut cur: xmlC14NVisibleNsStackPtr,
    mut ns: xmlNsPtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut prefix: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut href: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut has_empty_ns: c_int = 0;
    if cur.is_null() {
        xmlC14NErrParam(
            b"searching namespaces stack (c14n)\0" as *const u8 as *const c_char,
        );
        return 0 as c_int;
    }
    prefix = if ns.is_null() || (*ns).prefix.is_null() {
        b"\0" as *const u8 as *const c_char as *mut xmlChar as *const xmlChar
    } else {
        (*ns).prefix
    };
    href = if ns.is_null() || (*ns).href.is_null() {
        b"\0" as *const u8 as *const c_char as *mut xmlChar as *const xmlChar
    } else {
        (*ns).href
    };
    has_empty_ns = (xmlC14NStrEqual(prefix, ::core::ptr::null::<xmlChar>()) != 0
        && xmlC14NStrEqual(href, ::core::ptr::null::<xmlChar>()) != 0)
        as c_int;
    if !(*cur).nsTab.is_null() {
        let mut start: c_int = if has_empty_ns != 0 {
            0 as c_int
        } else {
            (*cur).nsPrevStart
        };
        i = (*cur).nsCurEnd - 1 as c_int;
        while i >= start {
            let mut ns1: xmlNsPtr = *(*cur).nsTab.offset(i as isize);
            if xmlC14NStrEqual(
                prefix,
                if !ns1.is_null() {
                    (*ns1).prefix
                } else {
                    ::core::ptr::null::<xmlChar>()
                },
            ) != 0
            {
                return xmlC14NStrEqual(
                    href,
                    if !ns1.is_null() {
                        (*ns1).href
                    } else {
                        ::core::ptr::null::<xmlChar>()
                    },
                );
            }
            i -= 1;
        }
    }
    return has_empty_ns;
} }
fn xmlExcC14NVisibleNsStackFind(
    mut cur: xmlC14NVisibleNsStackPtr,
    mut ns: xmlNsPtr,
    mut ctx: xmlC14NCtxPtr,
) -> c_int { unsafe {
    let mut i: c_int = 0;
    let mut prefix: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut href: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut has_empty_ns: c_int = 0;
    if cur.is_null() {
        xmlC14NErrParam(
            b"searching namespaces stack (exc c14n)\0" as *const u8 as *const c_char,
        );
        return 0 as c_int;
    }
    prefix = if ns.is_null() || (*ns).prefix.is_null() {
        b"\0" as *const u8 as *const c_char as *mut xmlChar as *const xmlChar
    } else {
        (*ns).prefix
    };
    href = if ns.is_null() || (*ns).href.is_null() {
        b"\0" as *const u8 as *const c_char as *mut xmlChar as *const xmlChar
    } else {
        (*ns).href
    };
    has_empty_ns = (xmlC14NStrEqual(prefix, ::core::ptr::null::<xmlChar>()) != 0
        && xmlC14NStrEqual(href, ::core::ptr::null::<xmlChar>()) != 0)
        as c_int;
    if !(*cur).nsTab.is_null() {
        let mut start: c_int = 0 as c_int;
        i = (*cur).nsCurEnd - 1 as c_int;
        while i >= start {
            let mut ns1: xmlNsPtr = *(*cur).nsTab.offset(i as isize);
            if xmlC14NStrEqual(
                prefix,
                if !ns1.is_null() {
                    (*ns1).prefix
                } else {
                    ::core::ptr::null::<xmlChar>()
                },
            ) != 0
            {
                if xmlC14NStrEqual(
                    href,
                    if !ns1.is_null() {
                        (*ns1).href
                    } else {
                        ::core::ptr::null::<xmlChar>()
                    },
                ) != 0
                {
                    return if (*ctx).is_visible_callback.is_some() {
                        (*ctx)
                            .is_visible_callback
                            .expect("non-null function pointer")(
                            (*ctx).user_data,
                            ns1 as xmlNodePtr,
                            *(*cur).nodeTab.offset(i as isize),
                        )
                    } else {
                        1 as c_int
                    };
                } else {
                    return 0 as c_int;
                }
            }
            i -= 1;
        }
    }
    return has_empty_ns;
} }
fn xmlC14NIsXmlNs(mut ns: xmlNsPtr) -> c_int { unsafe {
    return (!ns.is_null()
        && xmlStrEqual(
            (*ns).prefix,
            b"xml\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
        && xmlStrEqual((*ns).href, XML_XML_NAMESPACE) != 0) as c_int;
} }
unsafe extern "C" fn xmlC14NNsCompare(
    mut data1: *const c_void,
    mut data2: *const c_void,
) -> c_int {
    let ns1: xmlNsPtr = data1 as xmlNsPtr;
    let ns2: xmlNsPtr = data2 as xmlNsPtr;
    if ns1 == ns2 {
        return 0 as c_int;
    }
    if ns1.is_null() {
        return -(1 as c_int);
    }
    if ns2.is_null() {
        return 1 as c_int;
    }
    return xmlStrcmp((*ns1).prefix, (*ns2).prefix);
}
fn xmlC14NPrintNamespaces(
    ns: xmlNsPtr,
    mut ctx: xmlC14NCtxPtr,
) -> c_int { unsafe {
    if ns.is_null() || ctx.is_null() {
        xmlC14NErrParam(b"writing namespaces\0" as *const u8 as *const c_char);
        return 0 as c_int;
    }
    if !(*ns).prefix.is_null() {
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b" xmlns:\0" as *const u8 as *const c_char,
        );
        xmlOutputBufferWriteString((*ctx).buf, (*ns).prefix as *const c_char);
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b"=\0" as *const u8 as *const c_char,
        );
    } else {
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b" xmlns=\0" as *const u8 as *const c_char,
        );
    }
    if !(*ns).href.is_null() {
        xmlBufWriteQuotedString((*(*ctx).buf).buffer, (*ns).href);
    } else {
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b"\"\"\0" as *const u8 as *const c_char,
        );
    }
    return 1 as c_int;
} }
unsafe extern "C" fn xmlC14NPrintNamespacesWalker(
    mut ns: *const c_void,
    mut ctx: *mut c_void,
) -> c_int {
    return xmlC14NPrintNamespaces(ns as xmlNsPtr, ctx as xmlC14NCtxPtr);
}
fn xmlC14NProcessNamespacesAxis(
    mut ctx: xmlC14NCtxPtr,
    mut cur: xmlNodePtr,
    mut visible: c_int,
) -> c_int { unsafe {
    let mut n: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut tmp: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut list: xmlListPtr = ::core::ptr::null_mut::<xmlList>();
    let mut already_rendered: c_int = 0;
    let mut has_empty_ns: c_int = 0 as c_int;
    if ctx.is_null()
        || cur.is_null()
        || (*cur).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        xmlC14NErrParam(
            b"processing namespaces axis (c14n)\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    list = xmlListCreate(
        None,
        Some(
            xmlC14NNsCompare
                as unsafe extern "C" fn(
                    *const c_void,
                    *const c_void,
                ) -> c_int,
        ),
    );
    if list.is_null() {
        xmlC14NErrInternal(
            b"creating namespaces list (c14n)\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    n = cur;
    while !n.is_null() {
        ns = (*n).nsDef as xmlNsPtr;
        while !ns.is_null() {
            tmp = xmlSearchNs((*cur).doc as xmlDocPtr, cur, (*ns).prefix);
            if tmp == ns
                && xmlC14NIsXmlNs(ns) == 0
                && (if (*ctx).is_visible_callback.is_some() {
                    (*ctx)
                        .is_visible_callback
                        .expect("non-null function pointer")(
                        (*ctx).user_data,
                        ns as xmlNodePtr,
                        cur,
                    )
                } else {
                    1 as c_int
                }) != 0
            {
                already_rendered = xmlC14NVisibleNsStackFind((*ctx).ns_rendered, ns);
                if visible != 0 {
                    xmlC14NVisibleNsStackAdd((*ctx).ns_rendered, ns, cur);
                }
                if already_rendered == 0 {
                    xmlListInsert(list, ns as *mut c_void);
                }
                if xmlStrlen((*ns).prefix) == 0 as c_int {
                    has_empty_ns = 1 as c_int;
                }
            }
            ns = (*ns).next as xmlNsPtr;
        }
        n = (*n).parent as xmlNodePtr;
    }
    if visible != 0 && has_empty_ns == 0 {
        static mut ns_default: xmlNs = xmlNs {
            next: ::core::ptr::null_mut::<_xmlNs>(),
            type_0: 0 as xmlNsType,
            href: ::core::ptr::null::<xmlChar>(),
            prefix: ::core::ptr::null::<xmlChar>(),
            _private: ::core::ptr::null_mut::<c_void>(),
            context: ::core::ptr::null_mut::<_xmlDoc>(),
        };
        memset(
            &raw mut ns_default as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<xmlNs>() as size_t,
        );
        if xmlC14NVisibleNsStackFind((*ctx).ns_rendered, &raw mut ns_default) == 0 {
            xmlC14NPrintNamespaces(&raw mut ns_default, ctx);
        }
    }
    xmlListWalk(
        list,
        Some(
            xmlC14NPrintNamespacesWalker
                as unsafe extern "C" fn(
                    *const c_void,
                    *mut c_void,
                ) -> c_int,
        ),
        ctx as *mut c_void,
    );
    xmlListDelete(list);
    return 0 as c_int;
} }
fn xmlExcC14NProcessNamespacesAxis(
    mut ctx: xmlC14NCtxPtr,
    mut cur: xmlNodePtr,
    mut visible: c_int,
) -> c_int { unsafe {
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut list: xmlListPtr = ::core::ptr::null_mut::<xmlList>();
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut already_rendered: c_int = 0;
    let mut has_empty_ns: c_int = 0 as c_int;
    let mut has_visibly_utilized_empty_ns: c_int = 0 as c_int;
    let mut has_empty_ns_in_inclusive_list: c_int = 0 as c_int;
    if ctx.is_null()
        || cur.is_null()
        || (*cur).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        xmlC14NErrParam(
            b"processing namespaces axis (exc c14n)\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if !((*ctx).mode as c_uint
        == XML_C14N_EXCLUSIVE_1_0 as c_int as c_uint)
    {
        xmlC14NErrParam(
            b"processing namespaces axis (exc c14n)\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    list = xmlListCreate(
        None,
        Some(
            xmlC14NNsCompare
                as unsafe extern "C" fn(
                    *const c_void,
                    *const c_void,
                ) -> c_int,
        ),
    );
    if list.is_null() {
        xmlC14NErrInternal(
            b"creating namespaces list (exc c14n)\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if !(*ctx).inclusive_ns_prefixes.is_null() {
        let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut i: c_int = 0;
        i = 0 as c_int;
        while !(*(*ctx).inclusive_ns_prefixes.offset(i as isize)).is_null() {
            prefix = *(*ctx).inclusive_ns_prefixes.offset(i as isize);
            if xmlStrEqual(
                prefix,
                b"#default\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
                || xmlStrEqual(
                    prefix,
                    b"\0" as *const u8 as *const c_char as *mut xmlChar,
                ) != 0
            {
                prefix = ::core::ptr::null_mut::<xmlChar>();
                has_empty_ns_in_inclusive_list = 1 as c_int;
            }
            ns = xmlSearchNs((*cur).doc as xmlDocPtr, cur, prefix);
            if !ns.is_null()
                && xmlC14NIsXmlNs(ns) == 0
                && (if (*ctx).is_visible_callback.is_some() {
                    (*ctx)
                        .is_visible_callback
                        .expect("non-null function pointer")(
                        (*ctx).user_data,
                        ns as xmlNodePtr,
                        cur,
                    )
                } else {
                    1 as c_int
                }) != 0
            {
                already_rendered = xmlC14NVisibleNsStackFind((*ctx).ns_rendered, ns);
                if visible != 0 {
                    xmlC14NVisibleNsStackAdd((*ctx).ns_rendered, ns, cur);
                }
                if already_rendered == 0 {
                    xmlListInsert(list, ns as *mut c_void);
                }
                if xmlStrlen((*ns).prefix) == 0 as c_int {
                    has_empty_ns = 1 as c_int;
                }
            }
            i += 1;
        }
    }
    if !(*cur).ns.is_null() {
        ns = (*cur).ns as xmlNsPtr;
    } else {
        ns = xmlSearchNs((*cur).doc as xmlDocPtr, cur, ::core::ptr::null::<xmlChar>());
        has_visibly_utilized_empty_ns = 1 as c_int;
    }
    if !ns.is_null() && xmlC14NIsXmlNs(ns) == 0 {
        if visible != 0
            && (if (*ctx).is_visible_callback.is_some() {
                (*ctx)
                    .is_visible_callback
                    .expect("non-null function pointer")(
                    (*ctx).user_data, ns as xmlNodePtr, cur
                )
            } else {
                1 as c_int
            }) != 0
        {
            if xmlExcC14NVisibleNsStackFind((*ctx).ns_rendered, ns, ctx) == 0 {
                xmlListInsert(list, ns as *mut c_void);
            }
        }
        if visible != 0 {
            xmlC14NVisibleNsStackAdd((*ctx).ns_rendered, ns, cur);
        }
        if xmlStrlen((*ns).prefix) == 0 as c_int {
            has_empty_ns = 1 as c_int;
        }
    }
    attr = (*cur).properties as xmlAttrPtr;
    while !attr.is_null() {
        if !(*attr).ns.is_null()
            && xmlC14NIsXmlNs((*attr).ns as xmlNsPtr) == 0
            && (if (*ctx).is_visible_callback.is_some() {
                (*ctx)
                    .is_visible_callback
                    .expect("non-null function pointer")(
                    (*ctx).user_data, attr as xmlNodePtr, cur
                )
            } else {
                1 as c_int
            }) != 0
        {
            already_rendered =
                xmlExcC14NVisibleNsStackFind((*ctx).ns_rendered, (*attr).ns as xmlNsPtr, ctx);
            xmlC14NVisibleNsStackAdd((*ctx).ns_rendered, (*attr).ns as xmlNsPtr, cur);
            if already_rendered == 0 && visible != 0 {
                xmlListInsert(list, (*attr).ns as *mut c_void);
            }
            if xmlStrlen((*(*attr).ns).prefix) == 0 as c_int {
                has_empty_ns = 1 as c_int;
            }
        } else if !(*attr).ns.is_null()
            && xmlStrlen((*(*attr).ns).prefix) == 0 as c_int
            && xmlStrlen((*(*attr).ns).href) == 0 as c_int
        {
            has_visibly_utilized_empty_ns = 1 as c_int;
        }
        attr = (*attr).next as xmlAttrPtr;
    }
    if visible != 0
        && has_visibly_utilized_empty_ns != 0
        && has_empty_ns == 0
        && has_empty_ns_in_inclusive_list == 0
    {
        static mut ns_default: xmlNs = xmlNs {
            next: ::core::ptr::null_mut::<_xmlNs>(),
            type_0: 0 as xmlNsType,
            href: ::core::ptr::null::<xmlChar>(),
            prefix: ::core::ptr::null::<xmlChar>(),
            _private: ::core::ptr::null_mut::<c_void>(),
            context: ::core::ptr::null_mut::<_xmlDoc>(),
        };
        memset(
            &raw mut ns_default as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<xmlNs>() as size_t,
        );
        already_rendered =
            xmlExcC14NVisibleNsStackFind((*ctx).ns_rendered, &raw mut ns_default, ctx);
        if already_rendered == 0 {
            xmlC14NPrintNamespaces(&raw mut ns_default, ctx);
        }
    } else if visible != 0 && has_empty_ns == 0 && has_empty_ns_in_inclusive_list != 0 {
        static mut ns_default_0: xmlNs = xmlNs {
            next: ::core::ptr::null_mut::<_xmlNs>(),
            type_0: 0 as xmlNsType,
            href: ::core::ptr::null::<xmlChar>(),
            prefix: ::core::ptr::null::<xmlChar>(),
            _private: ::core::ptr::null_mut::<c_void>(),
            context: ::core::ptr::null_mut::<_xmlDoc>(),
        };
        memset(
            &raw mut ns_default_0 as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<xmlNs>() as size_t,
        );
        if xmlC14NVisibleNsStackFind((*ctx).ns_rendered, &raw mut ns_default_0) == 0 {
            xmlC14NPrintNamespaces(&raw mut ns_default_0, ctx);
        }
    }
    xmlListWalk(
        list,
        Some(
            xmlC14NPrintNamespacesWalker
                as unsafe extern "C" fn(
                    *const c_void,
                    *mut c_void,
                ) -> c_int,
        ),
        ctx as *mut c_void,
    );
    xmlListDelete(list);
    return 0 as c_int;
} }
fn xmlC14NIsXmlAttr(mut attr: xmlAttrPtr) -> c_int { unsafe {
    return (!(*attr).ns.is_null()
        && xmlC14NIsXmlNs((*attr).ns as xmlNsPtr) != 0 as c_int)
        as c_int;
} }
unsafe extern "C" fn xmlC14NAttrsCompare(
    mut data1: *const c_void,
    mut data2: *const c_void,
) -> c_int {
    let attr1: xmlAttrPtr = data1 as xmlAttrPtr;
    let attr2: xmlAttrPtr = data2 as xmlAttrPtr;
    let mut ret: c_int = 0 as c_int;
    if attr1 == attr2 {
        return 0 as c_int;
    }
    if attr1.is_null() {
        return -(1 as c_int);
    }
    if attr2.is_null() {
        return 1 as c_int;
    }
    if (*attr1).ns == (*attr2).ns {
        return xmlStrcmp((*attr1).name, (*attr2).name);
    }
    if (*attr1).ns.is_null() {
        return -(1 as c_int);
    }
    if (*attr2).ns.is_null() {
        return 1 as c_int;
    }
    if (*(*attr1).ns).prefix.is_null() {
        return -(1 as c_int);
    }
    if (*(*attr2).ns).prefix.is_null() {
        return 1 as c_int;
    }
    ret = xmlStrcmp((*(*attr1).ns).href, (*(*attr2).ns).href);
    if ret == 0 as c_int {
        ret = xmlStrcmp((*attr1).name, (*attr2).name);
    }
    return ret;
}
unsafe extern "C" fn xmlC14NPrintAttrs(
    mut data: *const c_void,
    mut user: *mut c_void,
) -> c_int {
    let attr: xmlAttrPtr = data as xmlAttrPtr;
    let mut ctx: xmlC14NCtxPtr = user as xmlC14NCtxPtr;
    let mut value: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if attr.is_null() || ctx.is_null() {
        xmlC14NErrParam(b"writing attributes\0" as *const u8 as *const c_char);
        return 0 as c_int;
    }
    xmlOutputBufferWriteString(
        (*ctx).buf,
        b" \0" as *const u8 as *const c_char,
    );
    if !(*attr).ns.is_null() && xmlStrlen((*(*attr).ns).prefix) > 0 as c_int {
        xmlOutputBufferWriteString(
            (*ctx).buf,
            (*(*attr).ns).prefix as *const c_char,
        );
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b":\0" as *const u8 as *const c_char,
        );
    }
    xmlOutputBufferWriteString((*ctx).buf, (*attr).name as *const c_char);
    xmlOutputBufferWriteString(
        (*ctx).buf,
        b"=\"\0" as *const u8 as *const c_char,
    );
    value = xmlNodeListGetString((*ctx).doc, (*attr).children, 1 as c_int);
    if !value.is_null() {
        buffer = xmlC11NNormalizeString(value, XMLC14N_NORMALIZE_ATTR);
        xmlFree.expect("non-null function pointer")(value as *mut c_void);
        if !buffer.is_null() {
            xmlOutputBufferWriteString((*ctx).buf, buffer as *const c_char);
            xmlFree.expect("non-null function pointer")(buffer as *mut c_void);
        } else {
            xmlC14NErrInternal(
                b"normalizing attributes axis\0" as *const u8 as *const c_char,
            );
            return 0 as c_int;
        }
    }
    xmlOutputBufferWriteString(
        (*ctx).buf,
        b"\"\0" as *const u8 as *const c_char,
    );
    return 1 as c_int;
}
unsafe fn xmlC14NFindHiddenParentAttr(
    mut ctx: xmlC14NCtxPtr,
    mut cur: xmlNodePtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
) -> xmlAttrPtr {
    let mut res: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    while !cur.is_null()
        && (if (*ctx).is_visible_callback.is_some() {
            (*ctx)
                .is_visible_callback
                .expect("non-null function pointer")(
                (*ctx).user_data,
                cur,
                (*cur).parent as xmlNodePtr,
            )
        } else {
            1 as c_int
        }) == 0
    {
        res = xmlHasNsProp(cur as *const xmlNode, name, ns);
        if !res.is_null() {
            return res;
        }
        cur = (*cur).parent as xmlNodePtr;
    }
    return ::core::ptr::null_mut::<xmlAttr>();
}
fn xmlC14NFixupBaseAttr(
    mut ctx: xmlC14NCtxPtr,
    mut xml_base_attr: xmlAttrPtr,
) -> xmlAttrPtr { unsafe {
    let mut res: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut tmp_str: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut tmp_str2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut tmp_str_len: c_int = 0;
    if ctx.is_null() || xml_base_attr.is_null() || (*xml_base_attr).parent.is_null() {
        xmlC14NErrParam(
            b"processing xml:base attribute\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    res = xmlNodeListGetString(
        (*ctx).doc,
        (*xml_base_attr).children,
        1 as c_int,
    );
    if res.is_null() {
        xmlC14NErrInternal(
            b"processing xml:base attribute - can't get attr value\0" as *const u8
                as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    cur = (*(*xml_base_attr).parent).parent as xmlNodePtr;
    while !cur.is_null()
        && (if (*ctx).is_visible_callback.is_some() {
            (*ctx)
                .is_visible_callback
                .expect("non-null function pointer")(
                (*ctx).user_data,
                cur,
                (*cur).parent as xmlNodePtr,
            )
        } else {
            1 as c_int
        }) == 0
    {
        attr = xmlHasNsProp(
            cur as *const xmlNode,
            b"base\0" as *const u8 as *const c_char as *mut xmlChar,
            XML_XML_NAMESPACE,
        );
        if !attr.is_null() {
            tmp_str = xmlNodeListGetString((*ctx).doc, (*attr).children, 1 as c_int);
            if tmp_str.is_null() {
                xmlFree.expect("non-null function pointer")(res as *mut c_void);
                xmlC14NErrInternal(
                    b"processing xml:base attribute - can't get attr value\0" as *const u8
                        as *const c_char,
                );
                return ::core::ptr::null_mut::<xmlAttr>();
            }
            tmp_str_len = xmlStrlen(tmp_str);
            if tmp_str_len > 1 as c_int
                && *tmp_str.offset((tmp_str_len - 2 as c_int) as isize)
                    as c_int
                    == '.' as i32
            {
                tmp_str2 = xmlStrcat(
                    tmp_str,
                    b"/\0" as *const u8 as *const c_char as *mut xmlChar,
                );
                if tmp_str2.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        tmp_str as *mut c_void,
                    );
                    xmlFree.expect("non-null function pointer")(res as *mut c_void);
                    xmlC14NErrInternal(
                        b"processing xml:base attribute - can't modify uri\0" as *const u8
                            as *const c_char,
                    );
                    return ::core::ptr::null_mut::<xmlAttr>();
                }
                tmp_str = tmp_str2;
            }
            tmp_str2 = xmlBuildURI(res, tmp_str);
            if tmp_str2.is_null() {
                xmlFree.expect("non-null function pointer")(tmp_str as *mut c_void);
                xmlFree.expect("non-null function pointer")(res as *mut c_void);
                xmlC14NErrInternal(
                    b"processing xml:base attribute - can't construct uri\0" as *const u8
                        as *const c_char,
                );
                return ::core::ptr::null_mut::<xmlAttr>();
            }
            xmlFree.expect("non-null function pointer")(tmp_str as *mut c_void);
            xmlFree.expect("non-null function pointer")(res as *mut c_void);
            res = tmp_str2;
        }
        cur = (*cur).parent as xmlNodePtr;
    }
    if res.is_null()
        || xmlStrEqual(
            res,
            b"\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
    {
        xmlFree.expect("non-null function pointer")(res as *mut c_void);
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    attr = xmlNewNsProp(
        ::core::ptr::null_mut::<xmlNode>(),
        (*xml_base_attr).ns as xmlNsPtr,
        b"base\0" as *const u8 as *const c_char as *mut xmlChar,
        res,
    );
    if attr.is_null() {
        xmlFree.expect("non-null function pointer")(res as *mut c_void);
        xmlC14NErrInternal(
            b"processing xml:base attribute - can't construct attribute\0" as *const u8
                as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    xmlFree.expect("non-null function pointer")(res as *mut c_void);
    return attr;
} }
fn xmlC14NProcessAttrsAxis(
    mut ctx: xmlC14NCtxPtr,
    mut cur: xmlNodePtr,
    mut parent_visible: c_int,
) -> c_int { unsafe {
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut list: xmlListPtr = ::core::ptr::null_mut::<xmlList>();
    let mut attrs_to_delete: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut xml_base_attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut xml_lang_attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut xml_space_attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if ctx.is_null()
        || cur.is_null()
        || (*cur).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        xmlC14NErrParam(b"processing attributes axis\0" as *const u8 as *const c_char);
        return -(1 as c_int);
    }
    list = xmlListCreate(
        None,
        Some(
            xmlC14NAttrsCompare
                as unsafe extern "C" fn(
                    *const c_void,
                    *const c_void,
                ) -> c_int,
        ),
    );
    if list.is_null() {
        xmlC14NErrInternal(
            b"creating attributes list\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    match (*ctx).mode as c_uint {
        0 => {
            attr = (*cur).properties as xmlAttrPtr;
            while !attr.is_null() {
                if if (*ctx).is_visible_callback.is_some() {
                    (*ctx)
                        .is_visible_callback
                        .expect("non-null function pointer")(
                        (*ctx).user_data,
                        attr as xmlNodePtr,
                        cur,
                    )
                } else {
                    1 as c_int
                } != 0
                {
                    xmlListInsert(list, attr as *mut c_void);
                }
                attr = (*attr).next as xmlAttrPtr;
            }
            if parent_visible != 0
                && !(*cur).parent.is_null()
                && (if (*ctx).is_visible_callback.is_some() {
                    (*ctx)
                        .is_visible_callback
                        .expect("non-null function pointer")(
                        (*ctx).user_data,
                        (*cur).parent as xmlNodePtr,
                        (*(*cur).parent).parent as xmlNodePtr,
                    )
                } else {
                    1 as c_int
                }) == 0
            {
                let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                tmp = (*cur).parent as xmlNodePtr;
                while !tmp.is_null() {
                    attr = (*tmp).properties as xmlAttrPtr;
                    while !attr.is_null() {
                        if xmlC14NIsXmlAttr(attr) != 0 as c_int {
                            if xmlListSearch(list, attr as *mut c_void).is_null() {
                                xmlListInsert(list, attr as *mut c_void);
                            }
                        }
                        attr = (*attr).next as xmlAttrPtr;
                    }
                    tmp = (*tmp).parent as xmlNodePtr;
                }
            }
        }
        1 => {
            attr = (*cur).properties as xmlAttrPtr;
            while !attr.is_null() {
                if if (*ctx).is_visible_callback.is_some() {
                    (*ctx)
                        .is_visible_callback
                        .expect("non-null function pointer")(
                        (*ctx).user_data,
                        attr as xmlNodePtr,
                        cur,
                    )
                } else {
                    1 as c_int
                } != 0
                {
                    xmlListInsert(list, attr as *mut c_void);
                }
                attr = (*attr).next as xmlAttrPtr;
            }
        }
        2 => {
            attr = (*cur).properties as xmlAttrPtr;
            while !attr.is_null() {
                if parent_visible == 0 || xmlC14NIsXmlAttr(attr) == 0 as c_int {
                    if if (*ctx).is_visible_callback.is_some() {
                        (*ctx)
                            .is_visible_callback
                            .expect("non-null function pointer")(
                            (*ctx).user_data,
                            attr as xmlNodePtr,
                            cur,
                        )
                    } else {
                        1 as c_int
                    } != 0
                    {
                        xmlListInsert(list, attr as *mut c_void);
                    }
                } else {
                    let mut matched: c_int = 0 as c_int;
                    if matched == 0
                        && xml_lang_attr.is_null()
                        && xmlStrEqual(
                            (*attr).name,
                            b"lang\0" as *const u8 as *const c_char as *mut xmlChar,
                        ) != 0
                    {
                        xml_lang_attr = attr;
                        matched = 1 as c_int;
                    }
                    if matched == 0
                        && xml_space_attr.is_null()
                        && xmlStrEqual(
                            (*attr).name,
                            b"space\0" as *const u8 as *const c_char as *mut xmlChar,
                        ) != 0
                    {
                        xml_space_attr = attr;
                        matched = 1 as c_int;
                    }
                    if matched == 0
                        && xml_base_attr.is_null()
                        && xmlStrEqual(
                            (*attr).name,
                            b"base\0" as *const u8 as *const c_char as *mut xmlChar,
                        ) != 0
                    {
                        xml_base_attr = attr;
                        matched = 1 as c_int;
                    }
                    if matched == 0
                        && (if (*ctx).is_visible_callback.is_some() {
                            (*ctx)
                                .is_visible_callback
                                .expect("non-null function pointer")(
                                (*ctx).user_data,
                                attr as xmlNodePtr,
                                cur,
                            )
                        } else {
                            1 as c_int
                        }) != 0
                    {
                        xmlListInsert(list, attr as *mut c_void);
                    }
                }
                attr = (*attr).next as xmlAttrPtr;
            }
            if parent_visible != 0 {
                if xml_lang_attr.is_null() {
                    xml_lang_attr = xmlC14NFindHiddenParentAttr(
                        ctx,
                        (*cur).parent as xmlNodePtr,
                        b"lang\0" as *const u8 as *const c_char as *mut xmlChar,
                        XML_XML_NAMESPACE,
                    );
                }
                if !xml_lang_attr.is_null() {
                    xmlListInsert(list, xml_lang_attr as *mut c_void);
                }
                if xml_space_attr.is_null() {
                    xml_space_attr = xmlC14NFindHiddenParentAttr(
                        ctx,
                        (*cur).parent as xmlNodePtr,
                        b"space\0" as *const u8 as *const c_char as *mut xmlChar,
                        XML_XML_NAMESPACE,
                    );
                }
                if !xml_space_attr.is_null() {
                    xmlListInsert(list, xml_space_attr as *mut c_void);
                }
                if xml_base_attr.is_null() {
                    xml_base_attr = xmlC14NFindHiddenParentAttr(
                        ctx,
                        (*cur).parent as xmlNodePtr,
                        b"base\0" as *const u8 as *const c_char as *mut xmlChar,
                        XML_XML_NAMESPACE,
                    );
                }
                if !xml_base_attr.is_null() {
                    xml_base_attr = xmlC14NFixupBaseAttr(ctx, xml_base_attr);
                    if !xml_base_attr.is_null() {
                        xmlListInsert(list, xml_base_attr as *mut c_void);
                        (*xml_base_attr).next = attrs_to_delete as *mut _xmlAttr;
                        attrs_to_delete = xml_base_attr;
                    }
                }
            }
        }
        _ => {}
    }
    xmlListWalk(
        list,
        Some(
            xmlC14NPrintAttrs
                as unsafe extern "C" fn(
                    *const c_void,
                    *mut c_void,
                ) -> c_int,
        ),
        ctx as *mut c_void,
    );
    xmlFreePropList(attrs_to_delete);
    xmlListDelete(list);
    return 0 as c_int;
} }
fn xmlC14NCheckForRelativeNamespaces(
    mut ctx: xmlC14NCtxPtr,
    mut cur: xmlNodePtr,
) -> c_int { unsafe {
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if ctx.is_null()
        || cur.is_null()
        || (*cur).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        xmlC14NErrParam(
            b"checking for relative namespaces\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    ns = (*cur).nsDef as xmlNsPtr;
    while !ns.is_null() {
        if xmlStrlen((*ns).href) > 0 as c_int {
            let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
            uri = xmlParseURI((*ns).href as *const c_char);
            if uri.is_null() {
                xmlC14NErrInternal(
                    b"parsing namespace uri\0" as *const u8 as *const c_char,
                );
                return -(1 as c_int);
            }
            if xmlStrlen((*uri).scheme as *const xmlChar) == 0 as c_int {
                xmlC14NErrRelativeNamespace((*uri).scheme);
                xmlFreeURI(uri);
                return -(1 as c_int);
            }
            xmlFreeURI(uri);
        }
        ns = (*ns).next as xmlNsPtr;
    }
    return 0 as c_int;
} }
fn xmlC14NProcessElementNode(
    mut ctx: xmlC14NCtxPtr,
    mut cur: xmlNodePtr,
    mut visible: c_int,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    let mut state: xmlC14NVisibleNsStack = xmlC14NVisibleNsStack {
        nsCurEnd: 0,
        nsPrevStart: 0,
        nsPrevEnd: 0,
        nsMax: 0,
        nsTab: ::core::ptr::null_mut::<xmlNsPtr>(),
        nodeTab: ::core::ptr::null_mut::<xmlNodePtr>(),
    };
    let mut parent_is_doc: c_int = 0 as c_int;
    if ctx.is_null()
        || cur.is_null()
        || (*cur).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        xmlC14NErrParam(b"processing element node\0" as *const u8 as *const c_char);
        return -(1 as c_int);
    }
    if xmlC14NCheckForRelativeNamespaces(ctx, cur) < 0 as c_int {
        xmlC14NErrInternal(
            b"checking for relative namespaces\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    memset(
        &raw mut state as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlC14NVisibleNsStack>() as size_t,
    );
    xmlC14NVisibleNsStackSave((*ctx).ns_rendered, &raw mut state);
    if visible != 0 {
        if (*ctx).parent_is_doc != 0 {
            parent_is_doc = (*ctx).parent_is_doc;
            (*ctx).parent_is_doc = 0 as c_int;
            (*ctx).pos = XMLC14N_INSIDE_DOCUMENT_ELEMENT;
        }
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b"<\0" as *const u8 as *const c_char,
        );
        if !(*cur).ns.is_null() && xmlStrlen((*(*cur).ns).prefix) > 0 as c_int {
            xmlOutputBufferWriteString(
                (*ctx).buf,
                (*(*cur).ns).prefix as *const c_char,
            );
            xmlOutputBufferWriteString(
                (*ctx).buf,
                b":\0" as *const u8 as *const c_char,
            );
        }
        xmlOutputBufferWriteString((*ctx).buf, (*cur).name as *const c_char);
    }
    if !((*ctx).mode as c_uint
        == XML_C14N_EXCLUSIVE_1_0 as c_int as c_uint)
    {
        ret = xmlC14NProcessNamespacesAxis(ctx, cur, visible);
    } else {
        ret = xmlExcC14NProcessNamespacesAxis(ctx, cur, visible);
    }
    if ret < 0 as c_int {
        xmlC14NErrInternal(
            b"processing namespaces axis\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if visible != 0 {
        xmlC14NVisibleNsStackShift((*ctx).ns_rendered);
    }
    ret = xmlC14NProcessAttrsAxis(ctx, cur, visible);
    if ret < 0 as c_int {
        xmlC14NErrInternal(
            b"processing attributes axis\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    if visible != 0 {
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b">\0" as *const u8 as *const c_char,
        );
    }
    if !(*cur).children.is_null() {
        ret = xmlC14NProcessNodeList(ctx, (*cur).children as xmlNodePtr);
        if ret < 0 as c_int {
            xmlC14NErrInternal(
                b"processing childrens list\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
    }
    if visible != 0 {
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b"</\0" as *const u8 as *const c_char,
        );
        if !(*cur).ns.is_null() && xmlStrlen((*(*cur).ns).prefix) > 0 as c_int {
            xmlOutputBufferWriteString(
                (*ctx).buf,
                (*(*cur).ns).prefix as *const c_char,
            );
            xmlOutputBufferWriteString(
                (*ctx).buf,
                b":\0" as *const u8 as *const c_char,
            );
        }
        xmlOutputBufferWriteString((*ctx).buf, (*cur).name as *const c_char);
        xmlOutputBufferWriteString(
            (*ctx).buf,
            b">\0" as *const u8 as *const c_char,
        );
        if parent_is_doc != 0 {
            (*ctx).parent_is_doc = parent_is_doc;
            (*ctx).pos = XMLC14N_AFTER_DOCUMENT_ELEMENT;
        }
    }
    xmlC14NVisibleNsStackRestore((*ctx).ns_rendered, &raw mut state);
    return 0 as c_int;
} }
fn xmlC14NProcessNode(
    mut ctx: xmlC14NCtxPtr,
    mut cur: xmlNodePtr,
) -> c_int { unsafe {
    let mut ret: c_int = 0 as c_int;
    let mut visible: c_int = 0;
    if ctx.is_null() || cur.is_null() {
        xmlC14NErrParam(b"processing node\0" as *const u8 as *const c_char);
        return -(1 as c_int);
    }
    visible = if (*ctx).is_visible_callback.is_some() {
        (*ctx)
            .is_visible_callback
            .expect("non-null function pointer")(
            (*ctx).user_data, cur, (*cur).parent as xmlNodePtr
        )
    } else {
        1 as c_int
    };
    match (*cur).type_0 as c_uint {
        1 => {
            ret = xmlC14NProcessElementNode(ctx, cur, visible);
        }
        4 | 3 => {
            if visible != 0 && !(*cur).content.is_null() {
                let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                buffer = xmlC11NNormalizeString((*cur).content, XMLC14N_NORMALIZE_TEXT);
                if !buffer.is_null() {
                    xmlOutputBufferWriteString((*ctx).buf, buffer as *const c_char);
                    xmlFree.expect("non-null function pointer")(buffer as *mut c_void);
                } else {
                    xmlC14NErrInternal(
                        b"normalizing text node\0" as *const u8 as *const c_char,
                    );
                    return -(1 as c_int);
                }
            }
        }
        7 => {
            if visible != 0 {
                if (*ctx).pos as c_uint
                    == XMLC14N_AFTER_DOCUMENT_ELEMENT as c_int as c_uint
                {
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b"\n<?\0" as *const u8 as *const c_char,
                    );
                } else {
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b"<?\0" as *const u8 as *const c_char,
                    );
                }
                xmlOutputBufferWriteString((*ctx).buf, (*cur).name as *const c_char);
                if !(*cur).content.is_null() && *(*cur).content as c_int != '\0' as i32
                {
                    let mut buffer_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b" \0" as *const u8 as *const c_char,
                    );
                    buffer_0 = xmlC11NNormalizeString((*cur).content, XMLC14N_NORMALIZE_PI);
                    if !buffer_0.is_null() {
                        xmlOutputBufferWriteString(
                            (*ctx).buf,
                            buffer_0 as *const c_char,
                        );
                        xmlFree.expect("non-null function pointer")(
                            buffer_0 as *mut c_void,
                        );
                    } else {
                        xmlC14NErrInternal(
                            b"normalizing pi node\0" as *const u8 as *const c_char,
                        );
                        return -(1 as c_int);
                    }
                }
                if (*ctx).pos as c_uint
                    == XMLC14N_BEFORE_DOCUMENT_ELEMENT as c_int as c_uint
                {
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b"?>\n\0" as *const u8 as *const c_char,
                    );
                } else {
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b"?>\0" as *const u8 as *const c_char,
                    );
                }
            }
        }
        8 => {
            if visible != 0 && (*ctx).with_comments != 0 {
                if (*ctx).pos as c_uint
                    == XMLC14N_AFTER_DOCUMENT_ELEMENT as c_int as c_uint
                {
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b"\n<!--\0" as *const u8 as *const c_char,
                    );
                } else {
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b"<!--\0" as *const u8 as *const c_char,
                    );
                }
                if !(*cur).content.is_null() {
                    let mut buffer_1: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                    buffer_1 = xmlC11NNormalizeString((*cur).content, XMLC14N_NORMALIZE_COMMENT);
                    if !buffer_1.is_null() {
                        xmlOutputBufferWriteString(
                            (*ctx).buf,
                            buffer_1 as *const c_char,
                        );
                        xmlFree.expect("non-null function pointer")(
                            buffer_1 as *mut c_void,
                        );
                    } else {
                        xmlC14NErrInternal(
                            b"normalizing comment node\0" as *const u8
                                as *const c_char,
                        );
                        return -(1 as c_int);
                    }
                }
                if (*ctx).pos as c_uint
                    == XMLC14N_BEFORE_DOCUMENT_ELEMENT as c_int as c_uint
                {
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b"-->\n\0" as *const u8 as *const c_char,
                    );
                } else {
                    xmlOutputBufferWriteString(
                        (*ctx).buf,
                        b"-->\0" as *const u8 as *const c_char,
                    );
                }
            }
        }
        9 | 11 | 13 => {
            if !(*cur).children.is_null() {
                (*ctx).pos = XMLC14N_BEFORE_DOCUMENT_ELEMENT;
                (*ctx).parent_is_doc = 1 as c_int;
                ret = xmlC14NProcessNodeList(ctx, (*cur).children as xmlNodePtr);
            }
        }
        2 => {
            xmlC14NErrInvalidNode(
                b"XML_ATTRIBUTE_NODE\0" as *const u8 as *const c_char,
                b"processing node\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        18 => {
            xmlC14NErrInvalidNode(
                b"XML_NAMESPACE_DECL\0" as *const u8 as *const c_char,
                b"processing node\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        5 => {
            xmlC14NErrInvalidNode(
                b"XML_ENTITY_REF_NODE\0" as *const u8 as *const c_char,
                b"processing node\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        6 => {
            xmlC14NErrInvalidNode(
                b"XML_ENTITY_NODE\0" as *const u8 as *const c_char,
                b"processing node\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        10 | 12 | 14 | 15 | 16 | 17 | 19 | 20 => {}
        _ => {
            xmlC14NErrUnknownNode(
                (*cur).type_0 as c_int,
                b"processing node\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
    }
    return ret;
} }
fn xmlC14NProcessNodeList(
    mut ctx: xmlC14NCtxPtr,
    mut cur: xmlNodePtr,
) -> c_int { unsafe {
    let mut ret: c_int = 0;
    if ctx.is_null() {
        xmlC14NErrParam(b"processing node list\0" as *const u8 as *const c_char);
        return -(1 as c_int);
    }
    ret = 0 as c_int;
    while !cur.is_null() && ret >= 0 as c_int {
        ret = xmlC14NProcessNode(ctx, cur);
        cur = (*cur).next as xmlNodePtr;
    }
    return ret;
} }
fn xmlC14NFreeCtx(mut ctx: xmlC14NCtxPtr) { unsafe {
    if ctx.is_null() {
        xmlC14NErrParam(b"freeing context\0" as *const u8 as *const c_char);
        return;
    }
    if !(*ctx).ns_rendered.is_null() {
        xmlC14NVisibleNsStackDestroy((*ctx).ns_rendered);
    }
    xmlFree.expect("non-null function pointer")(ctx as *mut c_void);
} }
unsafe fn xmlC14NNewCtx(
    mut doc: xmlDocPtr,
    mut is_visible_callback: xmlC14NIsVisibleCallback,
    mut user_data: *mut c_void,
    mut mode: xmlC14NMode,
    mut inclusive_ns_prefixes: *mut *mut xmlChar,
    mut with_comments: c_int,
    mut buf: xmlOutputBufferPtr,
) -> xmlC14NCtxPtr {
    let mut ctx: xmlC14NCtxPtr = ::core::ptr::null_mut::<_xmlC14NCtx>();
    if doc.is_null() || buf.is_null() {
        xmlC14NErrParam(b"creating new context\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<_xmlC14NCtx>();
    }
    if !(*buf).encoder.is_null() {
        xmlC14NErr(
            ctx,
            doc as xmlNodePtr,
            XML_C14N_REQUIRES_UTF8 as c_int,
            b"xmlC14NNewCtx: output buffer encoder != NULL but C14N requires UTF8 output\n\0"
                as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<_xmlC14NCtx>();
    }
    ctx = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlC14NCtx>() as size_t
    ) as xmlC14NCtxPtr;
    if ctx.is_null() {
        xmlC14NErrMemory(b"creating context\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<_xmlC14NCtx>();
    }
    memset(
        ctx as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlC14NCtx>() as size_t,
    );
    (*ctx).doc = doc;
    (*ctx).with_comments = with_comments;
    (*ctx).is_visible_callback = is_visible_callback;
    (*ctx).user_data = user_data;
    (*ctx).buf = buf;
    (*ctx).parent_is_doc = 1 as c_int;
    (*ctx).pos = XMLC14N_BEFORE_DOCUMENT_ELEMENT;
    (*ctx).ns_rendered = xmlC14NVisibleNsStackCreate();
    if (*ctx).ns_rendered.is_null() {
        xmlC14NErr(
            ctx,
            doc as xmlNodePtr,
            XML_C14N_CREATE_STACK as c_int,
            b"xmlC14NNewCtx: xmlC14NVisibleNsStackCreate failed\n\0" as *const u8
                as *const c_char,
        );
        xmlC14NFreeCtx(ctx);
        return ::core::ptr::null_mut::<_xmlC14NCtx>();
    }
    (*ctx).mode = mode;
    if (*ctx).mode as c_uint
        == XML_C14N_EXCLUSIVE_1_0 as c_int as c_uint
    {
        (*ctx).inclusive_ns_prefixes = inclusive_ns_prefixes;
    }
    return ctx;
}
#[inline]
pub unsafe fn xmlC14NExecute(
    mut doc: xmlDocPtr,
    mut is_visible_callback: xmlC14NIsVisibleCallback,
    mut user_data: *mut c_void,
    mut mode: c_int,
    mut inclusive_ns_prefixes: *mut *mut xmlChar,
    mut with_comments: c_int,
    mut buf: xmlOutputBufferPtr,
) -> c_int {
    let mut ctx: xmlC14NCtxPtr = ::core::ptr::null_mut::<_xmlC14NCtx>();
    let mut c14n_mode: xmlC14NMode = XML_C14N_1_0;
    let mut ret: c_int = 0;
    if buf.is_null() || doc.is_null() {
        xmlC14NErrParam(b"executing c14n\0" as *const u8 as *const c_char);
        return -(1 as c_int);
    }
    match mode {
        0 | 1 | 2 => {
            c14n_mode = mode as xmlC14NMode;
        }
        _ => {
            xmlC14NErrParam(
                b"invalid mode for executing c14n\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
    }
    if !(*buf).encoder.is_null() {
        xmlC14NErr(
            ::core::ptr::null_mut::<_xmlC14NCtx>(),
            doc as xmlNodePtr,
            XML_C14N_REQUIRES_UTF8 as c_int,
            b"xmlC14NExecute: output buffer encoder != NULL but C14N requires UTF8 output\n\0"
                as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    ctx = xmlC14NNewCtx(
        doc,
        is_visible_callback,
        user_data,
        c14n_mode,
        inclusive_ns_prefixes,
        with_comments,
        buf,
    );
    if ctx.is_null() {
        xmlC14NErr(
            ::core::ptr::null_mut::<_xmlC14NCtx>(),
            doc as xmlNodePtr,
            XML_C14N_CREATE_CTXT as c_int,
            b"xmlC14NExecute: unable to create C14N context\n\0" as *const u8
                as *const c_char,
        );
        return -(1 as c_int);
    }
    if !(*doc).children.is_null() {
        ret = xmlC14NProcessNodeList(ctx, (*doc).children as xmlNodePtr);
        if ret < 0 as c_int {
            xmlC14NErrInternal(
                b"processing docs children list\0" as *const u8 as *const c_char,
            );
            xmlC14NFreeCtx(ctx);
            return -(1 as c_int);
        }
    }
    ret = xmlOutputBufferFlush(buf);
    if ret < 0 as c_int {
        xmlC14NErrInternal(b"flushing output buffer\0" as *const u8 as *const c_char);
        xmlC14NFreeCtx(ctx);
        return -(1 as c_int);
    }
    xmlC14NFreeCtx(ctx);
    return ret;
}
#[inline]
pub unsafe fn xmlC14NDocSaveTo(
    mut doc: xmlDocPtr,
    mut nodes: xmlNodeSetPtr,
    mut mode: c_int,
    mut inclusive_ns_prefixes: *mut *mut xmlChar,
    mut with_comments: c_int,
    mut buf: xmlOutputBufferPtr,
) -> c_int {
    return xmlC14NExecute(
        doc,
        Some(
            xmlC14NIsNodeInNodeset
                as unsafe extern "C" fn(
                    *mut c_void,
                    xmlNodePtr,
                    xmlNodePtr,
                ) -> c_int,
        ),
        nodes as *mut c_void,
        mode,
        inclusive_ns_prefixes,
        with_comments,
        buf,
    );
}
#[inline]
pub unsafe fn xmlC14NDocDumpMemory(
    mut doc: xmlDocPtr,
    mut nodes: xmlNodeSetPtr,
    mut mode: c_int,
    mut inclusive_ns_prefixes: *mut *mut xmlChar,
    mut with_comments: c_int,
    mut doc_txt_ptr: *mut *mut xmlChar,
) -> c_int {
    let mut ret: c_int = 0;
    let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    if doc_txt_ptr.is_null() {
        xmlC14NErrParam(b"dumping doc to memory\0" as *const u8 as *const c_char);
        return -(1 as c_int);
    }
    *doc_txt_ptr = ::core::ptr::null_mut::<xmlChar>();
    buf = xmlAllocOutputBuffer(::core::ptr::null_mut::<xmlCharEncodingHandler>());
    if buf.is_null() {
        xmlC14NErrMemory(b"creating output buffer\0" as *const u8 as *const c_char);
        return -(1 as c_int);
    }
    ret = xmlC14NDocSaveTo(doc, nodes, mode, inclusive_ns_prefixes, with_comments, buf);
    if ret < 0 as c_int {
        xmlC14NErrInternal(
            b"saving doc to output buffer\0" as *const u8 as *const c_char,
        );
        xmlOutputBufferClose(buf);
        return -(1 as c_int);
    }
    ret = xmlBufUse((*buf).buffer) as c_int;
    if ret >= 0 as c_int {
        *doc_txt_ptr = xmlStrndup(xmlBufContent((*buf).buffer as *const xmlBuf), ret);
    }
    xmlOutputBufferClose(buf);
    if (*doc_txt_ptr).is_null() && ret >= 0 as c_int {
        xmlC14NErrMemory(
            b"copying canonicalized document\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    return ret;
}
#[inline]
pub unsafe fn xmlC14NDocSave(
    mut doc: xmlDocPtr,
    mut nodes: xmlNodeSetPtr,
    mut mode: c_int,
    mut inclusive_ns_prefixes: *mut *mut xmlChar,
    mut with_comments: c_int,
    mut filename: *const c_char,
    mut compression: c_int,
) -> c_int {
    let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    let mut ret: c_int = 0;
    if filename.is_null() {
        xmlC14NErrParam(b"saving doc\0" as *const u8 as *const c_char);
        return -(1 as c_int);
    }
    buf = xmlOutputBufferCreateFilename(
        filename,
        ::core::ptr::null_mut::<xmlCharEncodingHandler>(),
        compression,
    );
    if buf.is_null() {
        xmlC14NErrInternal(
            b"creating temporary filename\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    ret = xmlC14NDocSaveTo(doc, nodes, mode, inclusive_ns_prefixes, with_comments, buf);
    if ret < 0 as c_int {
        xmlC14NErrInternal(
            b"canonize document to buffer\0" as *const u8 as *const c_char,
        );
        xmlOutputBufferClose(buf);
        return -(1 as c_int);
    }
    ret = xmlOutputBufferClose(buf);
    return ret;
}
unsafe fn xmlC11NNormalizeString(
    mut input: *const xmlChar,
    mut mode: xmlC14NNormalizationMode,
) -> *mut xmlChar {
    let mut cur: *const xmlChar = input;
    let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut out: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut buffer_size: c_int = 0 as c_int;
    if input.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buffer_size = 1000 as c_int;
    buffer =
        xmlMallocAtomic.expect("non-null function pointer")(buffer_size as size_t) as *mut xmlChar;
    if buffer.is_null() {
        xmlC14NErrMemory(b"allocating buffer\0" as *const u8 as *const c_char);
        return ::core::ptr::null_mut::<xmlChar>();
    }
    out = buffer;
    while *cur as c_int != '\0' as i32 {
        if out.offset_from(buffer) as c_long
            > (buffer_size - 10 as c_int) as c_long
        {
            let mut indx: c_int =
                out.offset_from(buffer) as c_long as c_int;
            buffer_size *= 2 as c_int;
            buffer = xmlRealloc.expect("non-null function pointer")(
                buffer as *mut c_void,
                buffer_size as size_t,
            ) as *mut xmlChar;
            if buffer.is_null() {
                xmlC14NErrMemory(b"growing buffer\0" as *const u8 as *const c_char);
                return ::core::ptr::null_mut::<xmlChar>();
            }
            out = buffer.offset(indx as isize) as *mut xmlChar;
        }
        if *cur as c_int == '<' as i32
            && (mode as c_uint
                == XMLC14N_NORMALIZE_ATTR as c_int as c_uint
                || mode as c_uint
                    == XMLC14N_NORMALIZE_TEXT as c_int as c_uint)
        {
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = '&' as i32 as xmlChar;
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = 'l' as i32 as xmlChar;
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = 't' as i32 as xmlChar;
            let fresh3 = out;
            out = out.offset(1);
            *fresh3 = ';' as i32 as xmlChar;
        } else if *cur as c_int == '>' as i32
            && mode as c_uint
                == XMLC14N_NORMALIZE_TEXT as c_int as c_uint
        {
            let fresh4 = out;
            out = out.offset(1);
            *fresh4 = '&' as i32 as xmlChar;
            let fresh5 = out;
            out = out.offset(1);
            *fresh5 = 'g' as i32 as xmlChar;
            let fresh6 = out;
            out = out.offset(1);
            *fresh6 = 't' as i32 as xmlChar;
            let fresh7 = out;
            out = out.offset(1);
            *fresh7 = ';' as i32 as xmlChar;
        } else if *cur as c_int == '&' as i32
            && (mode as c_uint
                == XMLC14N_NORMALIZE_ATTR as c_int as c_uint
                || mode as c_uint
                    == XMLC14N_NORMALIZE_TEXT as c_int as c_uint)
        {
            let fresh8 = out;
            out = out.offset(1);
            *fresh8 = '&' as i32 as xmlChar;
            let fresh9 = out;
            out = out.offset(1);
            *fresh9 = 'a' as i32 as xmlChar;
            let fresh10 = out;
            out = out.offset(1);
            *fresh10 = 'm' as i32 as xmlChar;
            let fresh11 = out;
            out = out.offset(1);
            *fresh11 = 'p' as i32 as xmlChar;
            let fresh12 = out;
            out = out.offset(1);
            *fresh12 = ';' as i32 as xmlChar;
        } else if *cur as c_int == '"' as i32
            && mode as c_uint
                == XMLC14N_NORMALIZE_ATTR as c_int as c_uint
        {
            let fresh13 = out;
            out = out.offset(1);
            *fresh13 = '&' as i32 as xmlChar;
            let fresh14 = out;
            out = out.offset(1);
            *fresh14 = 'q' as i32 as xmlChar;
            let fresh15 = out;
            out = out.offset(1);
            *fresh15 = 'u' as i32 as xmlChar;
            let fresh16 = out;
            out = out.offset(1);
            *fresh16 = 'o' as i32 as xmlChar;
            let fresh17 = out;
            out = out.offset(1);
            *fresh17 = 't' as i32 as xmlChar;
            let fresh18 = out;
            out = out.offset(1);
            *fresh18 = ';' as i32 as xmlChar;
        } else if *cur as c_int == '\t' as i32
            && mode as c_uint
                == XMLC14N_NORMALIZE_ATTR as c_int as c_uint
        {
            let fresh19 = out;
            out = out.offset(1);
            *fresh19 = '&' as i32 as xmlChar;
            let fresh20 = out;
            out = out.offset(1);
            *fresh20 = '#' as i32 as xmlChar;
            let fresh21 = out;
            out = out.offset(1);
            *fresh21 = 'x' as i32 as xmlChar;
            let fresh22 = out;
            out = out.offset(1);
            *fresh22 = '9' as i32 as xmlChar;
            let fresh23 = out;
            out = out.offset(1);
            *fresh23 = ';' as i32 as xmlChar;
        } else if *cur as c_int == '\n' as i32
            && mode as c_uint
                == XMLC14N_NORMALIZE_ATTR as c_int as c_uint
        {
            let fresh24 = out;
            out = out.offset(1);
            *fresh24 = '&' as i32 as xmlChar;
            let fresh25 = out;
            out = out.offset(1);
            *fresh25 = '#' as i32 as xmlChar;
            let fresh26 = out;
            out = out.offset(1);
            *fresh26 = 'x' as i32 as xmlChar;
            let fresh27 = out;
            out = out.offset(1);
            *fresh27 = 'A' as i32 as xmlChar;
            let fresh28 = out;
            out = out.offset(1);
            *fresh28 = ';' as i32 as xmlChar;
        } else if *cur as c_int == '\r' as i32
            && (mode as c_uint
                == XMLC14N_NORMALIZE_ATTR as c_int as c_uint
                || mode as c_uint
                    == XMLC14N_NORMALIZE_TEXT as c_int as c_uint
                || mode as c_uint
                    == XMLC14N_NORMALIZE_COMMENT as c_int as c_uint
                || mode as c_uint
                    == XMLC14N_NORMALIZE_PI as c_int as c_uint)
        {
            let fresh29 = out;
            out = out.offset(1);
            *fresh29 = '&' as i32 as xmlChar;
            let fresh30 = out;
            out = out.offset(1);
            *fresh30 = '#' as i32 as xmlChar;
            let fresh31 = out;
            out = out.offset(1);
            *fresh31 = 'x' as i32 as xmlChar;
            let fresh32 = out;
            out = out.offset(1);
            *fresh32 = 'D' as i32 as xmlChar;
            let fresh33 = out;
            out = out.offset(1);
            *fresh33 = ';' as i32 as xmlChar;
        } else {
            let fresh34 = out;
            out = out.offset(1);
            *fresh34 = *cur;
        }
        cur = cur.offset(1);
    }
    *out = 0 as xmlChar;
    return buffer;
}
