use core::ffi::*;
use crate::src::HTMLparser::htmlTagLookup;
use crate::src::buf::xmlBufBackToBuffer;
use crate::src::buf::xmlBufContent;
use crate::src::buf::xmlBufFromBuffer;
use crate::src::buf::xmlBufUse;
use crate::src::buf::xmlBufWriteQuotedString;
use crate::src::encoding::xmlFindCharEncodingHandler;
use crate::src::threads::xmlInitParser;
use crate::src::encoding::xmlParseCharEncoding;
use crate::src::xmlstring::xmlStrEqual;
use crate::src::xmlstring::xmlStrcasecmp;
use crate::src::xmlstring::xmlStrcasestr;
use crate::src::xmlstring::xmlStrcmp;
use crate::src::xmlstring::xmlStrndup;
use crate::src::xmlstring::xmlStrstr;
use crate::src::uri::xmlURIEscapeStr;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
pub use crate::src::c_extern_types::*;
pub use crate::src::dict::_xmlDict;
pub use crate::src::buf::_xmlBuf;
extern "C" {
    fn xmlOutputBufferCreateFilename(
        URI: *const c_char,
        encoder: xmlCharEncodingHandlerPtr,
        compression: c_int,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferCreateFile(
        file: *mut FILE,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferWriteString(
        out: xmlOutputBufferPtr,
        str: *const c_char,
    ) -> c_int;
    fn xmlOutputBufferFlush(out: xmlOutputBufferPtr) -> c_int;
    fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> c_int;
    fn xmlNewProp(node: xmlNodePtr, name: *const xmlChar, value: *const xmlChar) -> xmlAttrPtr;
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlAddChild(parent: xmlNodePtr, cur: xmlNodePtr) -> xmlNodePtr;
    fn xmlAddPrevSibling(cur: xmlNodePtr, elem: xmlNodePtr) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlSetProp(node: xmlNodePtr, name: *const xmlChar, value: *const xmlChar) -> xmlAttrPtr;
    fn xmlNodeListGetString(
        doc: xmlDocPtr,
        list: *const xmlNode,
        inLine: c_int,
    ) -> *mut xmlChar;
    fn xmlEncodeEntitiesReentrant(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
    fn __xmlSimpleError(
        domain: c_int,
        code: c_int,
        node: *mut _xmlNode,
        msg: *const c_char,
        extra: *const c_char,
    );
    fn xmlAllocOutputBufferInternal(encoder: xmlCharEncodingHandlerPtr) -> xmlOutputBufferPtr;
    fn xmlNsListDumpOutput(buf: xmlOutputBufferPtr, cur: xmlNsPtr);
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
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
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
pub type htmlDocPtr = xmlDocPtr;
pub type htmlNodePtr = xmlNodePtr;

pub const HTML_COMMENT_NODE: c_uint = 8 as c_uint;
pub const HTML_PRESERVE_NODE: c_uint = 4 as c_uint;
pub const HTML_PI_NODE: c_uint = 7 as c_uint;
#[no_mangle]
pub extern "C" fn htmlGetMetaEncoding(mut doc: htmlDocPtr) -> *const xmlChar { unsafe {
    let mut current_block: u64;
    let mut cur: htmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut content: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut encoding: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if doc.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    cur = (*doc).children as htmlNodePtr;
    loop {
        if cur.is_null() {
            current_block = 1394248824506584008;
            break;
        }
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
            && !(*cur).name.is_null()
        {
            if xmlStrEqual(
                (*cur).name,
                b"html\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                current_block = 1394248824506584008;
                break;
            }
            if xmlStrEqual(
                (*cur).name,
                b"head\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                current_block = 12454271086832539970;
                break;
            }
            if xmlStrEqual(
                (*cur).name,
                b"meta\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                current_block = 2838571290723028321;
                break;
            }
        }
        cur = (*cur).next as htmlNodePtr;
    }
    match current_block {
        1394248824506584008 => {
            if cur.is_null() {
                return ::core::ptr::null::<xmlChar>();
            }
            cur = (*cur).children as htmlNodePtr;
            loop {
                if cur.is_null() {
                    current_block = 4956146061682418353;
                    break;
                }
                if (*cur).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    && !(*cur).name.is_null()
                {
                    if xmlStrEqual(
                        (*cur).name,
                        b"head\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 4956146061682418353;
                        break;
                    }
                    if xmlStrEqual(
                        (*cur).name,
                        b"meta\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 2838571290723028321;
                        break;
                    }
                }
                cur = (*cur).next as htmlNodePtr;
            }
            match current_block {
                2838571290723028321 => {}
                _ => {
                    if cur.is_null() {
                        return ::core::ptr::null::<xmlChar>();
                    }
                    current_block = 12454271086832539970;
                }
            }
        }
        _ => {}
    }
    match current_block {
        12454271086832539970 => {
            cur = (*cur).children as htmlNodePtr;
        }
        _ => {}
    }
    's_100: loop {
        if cur.is_null() {
            current_block = 2604890879466389055;
            break;
        }
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
            && !(*cur).name.is_null()
        {
            if xmlStrEqual(
                (*cur).name,
                b"meta\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                let mut attr: xmlAttrPtr = (*cur).properties as xmlAttrPtr;
                let mut http: c_int = 0;
                let mut value: *const xmlChar = ::core::ptr::null::<xmlChar>();
                content = ::core::ptr::null::<xmlChar>();
                http = 0 as c_int;
                while !attr.is_null() {
                    if !(*attr).children.is_null()
                        && (*(*attr).children).type_0 as c_uint
                            == XML_TEXT_NODE as c_int as c_uint
                        && (*(*attr).children).next.is_null()
                    {
                        value = (*(*attr).children).content;
                        if xmlStrcasecmp(
                            (*attr).name,
                            b"http-equiv\0" as *const u8 as *const c_char
                                as *mut xmlChar,
                        ) == 0
                            && xmlStrcasecmp(
                                value,
                                b"Content-Type\0" as *const u8 as *const c_char
                                    as *mut xmlChar,
                            ) == 0
                        {
                            http = 1 as c_int;
                        } else if !value.is_null()
                            && xmlStrcasecmp(
                                (*attr).name,
                                b"content\0" as *const u8 as *const c_char
                                    as *mut xmlChar,
                            ) == 0
                        {
                            content = value;
                        }
                        if http != 0 as c_int && !content.is_null() {
                            current_block = 11500260272211011089;
                            break 's_100;
                        }
                    }
                    attr = (*attr).next as xmlAttrPtr;
                }
            }
        }
        cur = (*cur).next as htmlNodePtr;
    }
    match current_block {
        2604890879466389055 => return ::core::ptr::null::<xmlChar>(),
        _ => {
            encoding = xmlStrstr(
                content,
                b"charset=\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            if encoding.is_null() {
                encoding = xmlStrstr(
                    content,
                    b"Charset=\0" as *const u8 as *const c_char as *mut xmlChar,
                );
            }
            if encoding.is_null() {
                encoding = xmlStrstr(
                    content,
                    b"CHARSET=\0" as *const u8 as *const c_char as *mut xmlChar,
                );
            }
            if !encoding.is_null() {
                encoding = encoding.offset(8 as c_int as isize);
            } else {
                encoding = xmlStrstr(
                    content,
                    b"charset =\0" as *const u8 as *const c_char as *mut xmlChar,
                );
                if encoding.is_null() {
                    encoding = xmlStrstr(
                        content,
                        b"Charset =\0" as *const u8 as *const c_char as *mut xmlChar,
                    );
                }
                if encoding.is_null() {
                    encoding = xmlStrstr(
                        content,
                        b"CHARSET =\0" as *const u8 as *const c_char as *mut xmlChar,
                    );
                }
                if !encoding.is_null() {
                    encoding = encoding.offset(9 as c_int as isize);
                }
            }
            if !encoding.is_null() {
                while *encoding as c_int == ' ' as i32
                    || *encoding as c_int == '\t' as i32
                {
                    encoding = encoding.offset(1);
                }
            }
            return encoding;
        }
    };
} }
#[no_mangle]
pub unsafe extern "C" fn htmlSetMetaEncoding(
    mut doc: htmlDocPtr,
    mut encoding: *const xmlChar,
) -> c_int {
    let mut current_block: u64;
    let mut cur: htmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut meta: htmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut head: htmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut content: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut newcontent: [c_char; 100] = [0; 100];
    newcontent[0 as c_int as usize] = 0 as c_char;
    if doc.is_null() {
        return -(1 as c_int);
    }
    if xmlStrcasecmp(
        encoding,
        b"html\0" as *const u8 as *const c_char as *mut xmlChar,
    ) == 0
    {
        return -(1 as c_int);
    }
    if !encoding.is_null() {
        snprintf(
            &raw mut newcontent as *mut c_char,
            ::core::mem::size_of::<[c_char; 100]>() as size_t,
            b"text/html; charset=%s\0" as *const u8 as *const c_char,
            encoding as *mut c_char,
        );
        newcontent[(::core::mem::size_of::<[c_char; 100]>() as usize)
            .wrapping_sub(1 as usize) as usize] = 0 as c_char;
    }
    cur = (*doc).children as htmlNodePtr;
    loop {
        if cur.is_null() {
            current_block = 10048703153582371463;
            break;
        }
        if (*cur).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
            && !(*cur).name.is_null()
        {
            if xmlStrcasecmp(
                (*cur).name,
                b"html\0" as *const u8 as *const c_char as *mut xmlChar,
            ) == 0 as c_int
            {
                current_block = 10048703153582371463;
                break;
            }
            if xmlStrcasecmp(
                (*cur).name,
                b"head\0" as *const u8 as *const c_char as *mut xmlChar,
            ) == 0 as c_int
            {
                current_block = 8361634153022159813;
                break;
            }
            if xmlStrcasecmp(
                (*cur).name,
                b"meta\0" as *const u8 as *const c_char as *mut xmlChar,
            ) == 0 as c_int
            {
                current_block = 17106228780814512145;
                break;
            }
        }
        cur = (*cur).next as htmlNodePtr;
    }
    match current_block {
        10048703153582371463 => {
            if cur.is_null() {
                return -(1 as c_int);
            }
            cur = (*cur).children as htmlNodePtr;
            loop {
                if cur.is_null() {
                    current_block = 5634871135123216486;
                    break;
                }
                if (*cur).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    && !(*cur).name.is_null()
                {
                    if xmlStrcasecmp(
                        (*cur).name,
                        b"head\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) == 0 as c_int
                    {
                        current_block = 5634871135123216486;
                        break;
                    }
                    if xmlStrcasecmp(
                        (*cur).name,
                        b"meta\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) == 0 as c_int
                    {
                        head = (*cur).parent as htmlNodePtr;
                        current_block = 17106228780814512145;
                        break;
                    }
                }
                cur = (*cur).next as htmlNodePtr;
            }
            match current_block {
                17106228780814512145 => {}
                _ => {
                    if cur.is_null() {
                        return -(1 as c_int);
                    }
                    current_block = 8361634153022159813;
                }
            }
        }
        _ => {}
    }
    match current_block {
        8361634153022159813 => {
            head = cur;
            if (*cur).children.is_null() {
                current_block = 1048676024829310743;
            } else {
                cur = (*cur).children as htmlNodePtr;
                current_block = 17106228780814512145;
            }
        }
        _ => {}
    }
    match current_block {
        17106228780814512145 => {
            while !cur.is_null() {
                if (*cur).type_0 as c_uint
                    == XML_ELEMENT_NODE as c_int as c_uint
                    && !(*cur).name.is_null()
                {
                    if xmlStrcasecmp(
                        (*cur).name,
                        b"meta\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) == 0 as c_int
                    {
                        let mut attr: xmlAttrPtr = (*cur).properties as xmlAttrPtr;
                        let mut http: c_int = 0;
                        let mut value: *const xmlChar = ::core::ptr::null::<xmlChar>();
                        content = ::core::ptr::null::<xmlChar>();
                        http = 0 as c_int;
                        while !attr.is_null() {
                            if !(*attr).children.is_null()
                                && (*(*attr).children).type_0 as c_uint
                                    == XML_TEXT_NODE as c_int as c_uint
                                && (*(*attr).children).next.is_null()
                            {
                                value = (*(*attr).children).content;
                                if xmlStrcasecmp(
                                    (*attr).name,
                                    b"http-equiv\0" as *const u8 as *const c_char
                                        as *mut xmlChar,
                                ) == 0
                                    && xmlStrcasecmp(
                                        value,
                                        b"Content-Type\0" as *const u8 as *const c_char
                                            as *mut xmlChar,
                                    ) == 0
                                {
                                    http = 1 as c_int;
                                } else if !value.is_null()
                                    && xmlStrcasecmp(
                                        (*attr).name,
                                        b"content\0" as *const u8 as *const c_char
                                            as *mut xmlChar,
                                    ) == 0
                                {
                                    content = value;
                                }
                                if http != 0 as c_int && !content.is_null() {
                                    break;
                                }
                            }
                            attr = (*attr).next as xmlAttrPtr;
                        }
                        if http != 0 as c_int && !content.is_null() {
                            meta = cur;
                            break;
                        }
                    }
                }
                cur = (*cur).next as htmlNodePtr;
            }
        }
        _ => {}
    }
    if meta.is_null() {
        if !encoding.is_null() && !head.is_null() {
            meta = xmlNewDocNode(
                doc as xmlDocPtr,
                ::core::ptr::null_mut::<xmlNs>(),
                b"meta\0" as *const u8 as *const c_char as *mut xmlChar,
                ::core::ptr::null::<xmlChar>(),
            ) as htmlNodePtr;
            if (*head).children.is_null() {
                xmlAddChild(head as xmlNodePtr, meta as xmlNodePtr);
            } else {
                xmlAddPrevSibling((*head).children as xmlNodePtr, meta as xmlNodePtr);
            }
            xmlNewProp(
                meta as xmlNodePtr,
                b"http-equiv\0" as *const u8 as *const c_char as *mut xmlChar,
                b"Content-Type\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            xmlNewProp(
                meta as xmlNodePtr,
                b"content\0" as *const u8 as *const c_char as *mut xmlChar,
                &raw mut newcontent as *mut c_char as *mut xmlChar,
            );
        }
    } else if encoding.is_null() {
        xmlUnlinkNode(meta as xmlNodePtr);
        xmlFreeNode(meta as xmlNodePtr);
    } else if xmlStrcasestr(content, encoding).is_null() {
        xmlSetProp(
            meta as xmlNodePtr,
            b"content\0" as *const u8 as *const c_char as *mut xmlChar,
            &raw mut newcontent as *mut c_char as *mut xmlChar,
        );
    }
    return 0 as c_int;
}
static mut htmlBooleanAttrs: [*const c_char; 14] = [
    b"checked\0" as *const u8 as *const c_char,
    b"compact\0" as *const u8 as *const c_char,
    b"declare\0" as *const u8 as *const c_char,
    b"defer\0" as *const u8 as *const c_char,
    b"disabled\0" as *const u8 as *const c_char,
    b"ismap\0" as *const u8 as *const c_char,
    b"multiple\0" as *const u8 as *const c_char,
    b"nohref\0" as *const u8 as *const c_char,
    b"noresize\0" as *const u8 as *const c_char,
    b"noshade\0" as *const u8 as *const c_char,
    b"nowrap\0" as *const u8 as *const c_char,
    b"readonly\0" as *const u8 as *const c_char,
    b"selected\0" as *const u8 as *const c_char,
    ::core::ptr::null::<c_char>(),
];
#[inline]
pub unsafe fn htmlIsBooleanAttr(mut name: *const xmlChar) -> c_int {
    let mut i: c_int = 0 as c_int;
    while !htmlBooleanAttrs[i as usize].is_null() {
        if xmlStrcasecmp(htmlBooleanAttrs[i as usize] as *const xmlChar, name)
            == 0 as c_int
        {
            return 1 as c_int;
        }
        i += 1;
    }
    return 0 as c_int;
}
unsafe fn htmlSaveErrMemory(mut extra: *const c_char) {
    __xmlSimpleError(
        XML_FROM_OUTPUT as c_int,
        XML_ERR_NO_MEMORY as c_int,
        ::core::ptr::null_mut::<_xmlNode>(),
        ::core::ptr::null::<c_char>(),
        extra,
    );
}
unsafe fn htmlSaveErr(
    mut code: c_int,
    mut node: xmlNodePtr,
    mut extra: *const c_char,
) {
    let mut msg: *const c_char = ::core::ptr::null::<c_char>();
    match code {
        1400 => {
            msg = b"string is not in UTF-8\n\0" as *const u8 as *const c_char;
        }
        1401 => {
            msg = b"invalid character value\n\0" as *const u8 as *const c_char;
        }
        1403 => {
            msg = b"unknown encoding %s\n\0" as *const u8 as *const c_char;
        }
        1402 => {
            msg = b"HTML has no DOCTYPE\n\0" as *const u8 as *const c_char;
        }
        _ => {
            msg = b"unexpected error number\n\0" as *const u8 as *const c_char;
        }
    }
    __xmlSimpleError(
        XML_FROM_OUTPUT as c_int,
        code,
        node as *mut _xmlNode,
        msg,
        extra,
    );
}
fn htmlBufNodeDumpFormat(
    mut buf: xmlBufPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut format: c_int,
) -> size_t { unsafe {
    let mut use_0: size_t = 0;
    let mut ret: c_int = 0;
    let mut outbuf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    if cur.is_null() {
        return -(1 as c_int) as size_t;
    }
    if buf.is_null() {
        return -(1 as c_int) as size_t;
    }
    outbuf = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlOutputBuffer>() as size_t,
    ) as xmlOutputBufferPtr;
    if outbuf.is_null() {
        htmlSaveErrMemory(
            b"allocating HTML output buffer\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int) as size_t;
    }
    memset(
        outbuf as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlOutputBuffer>() as size_t,
    );
    (*outbuf).buffer = buf;
    (*outbuf).encoder = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    (*outbuf).writecallback = None;
    (*outbuf).closecallback = None;
    (*outbuf).context = NULL;
    (*outbuf).written = 0 as c_int;
    use_0 = xmlBufUse(buf);
    htmlNodeDumpFormatOutput(
        outbuf,
        doc,
        cur,
        ::core::ptr::null::<c_char>(),
        format,
    );
    xmlFree.expect("non-null function pointer")(outbuf as *mut c_void);
    ret = xmlBufUse(buf).wrapping_sub(use_0) as c_int;
    return ret as size_t;
} }
#[inline]
pub fn htmlNodeDump(
    mut buf: xmlBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
) -> c_int { {
    let mut buffer: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut ret: size_t = 0;
    if buf.is_null() || cur.is_null() {
        return -(1 as c_int);
    }
    xmlInitParser();
    buffer = xmlBufFromBuffer(buf);
    if buffer.is_null() {
        return -(1 as c_int);
    }
    ret = htmlBufNodeDumpFormat(buffer, doc, cur, 1 as c_int);
    xmlBufBackToBuffer(buffer);
    if ret > INT_MAX as size_t {
        return -(1 as c_int);
    }
    return ret as c_int;
} }
#[inline]
pub unsafe fn htmlNodeDumpFileFormat(
    mut out: *mut FILE,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut encoding: *const c_char,
    mut format: c_int,
) -> c_int {
    let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    let mut handler: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    let mut ret: c_int = 0;
    xmlInitParser();
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as c_int != XML_CHAR_ENCODING_UTF8 as c_int {
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as c_int,
                    ::core::ptr::null_mut::<xmlNode>(),
                    encoding,
                );
            }
        }
    } else {
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"HTML\0" as *const u8 as *const c_char);
        }
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"ascii\0" as *const u8 as *const c_char);
        }
    }
    buf = xmlOutputBufferCreateFile(out, handler);
    if buf.is_null() {
        return 0 as c_int;
    }
    htmlNodeDumpFormatOutput(
        buf,
        doc,
        cur,
        ::core::ptr::null::<c_char>(),
        format,
    );
    ret = xmlOutputBufferClose(buf);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn htmlNodeDumpFile(
    mut out: *mut FILE,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
) {
    htmlNodeDumpFileFormat(
        out,
        doc,
        cur,
        ::core::ptr::null::<c_char>(),
        1 as c_int,
    );
}
#[inline]
pub unsafe fn htmlDocDumpMemoryFormat(
    mut cur: xmlDocPtr,
    mut mem: *mut *mut xmlChar,
    mut size: *mut c_int,
    mut format: c_int,
) {
    let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    let mut handler: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    let mut encoding: *const c_char = ::core::ptr::null::<c_char>();
    xmlInitParser();
    if mem.is_null() || size.is_null() {
        return;
    }
    if cur.is_null() {
        *mem = ::core::ptr::null_mut::<xmlChar>();
        *size = 0 as c_int;
        return;
    }
    encoding = htmlGetMetaEncoding(cur as htmlDocPtr) as *const c_char;
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as c_int != XML_CHAR_ENCODING_UTF8 as c_int {
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as c_int,
                    ::core::ptr::null_mut::<xmlNode>(),
                    encoding,
                );
            }
        }
    } else {
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"HTML\0" as *const u8 as *const c_char);
        }
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"ascii\0" as *const u8 as *const c_char);
        }
    }
    buf = xmlAllocOutputBufferInternal(handler);
    if buf.is_null() {
        *mem = ::core::ptr::null_mut::<xmlChar>();
        *size = 0 as c_int;
        return;
    }
    htmlDocContentDumpFormatOutput(buf, cur, ::core::ptr::null::<c_char>(), format);
    xmlOutputBufferFlush(buf);
    if !(*buf).conv.is_null() {
        *size = xmlBufUse((*buf).conv) as c_int;
        *mem = xmlStrndup(xmlBufContent((*buf).conv as *const xmlBuf), *size);
    } else {
        *size = xmlBufUse((*buf).buffer) as c_int;
        *mem = xmlStrndup(xmlBufContent((*buf).buffer as *const xmlBuf), *size);
    }
    xmlOutputBufferClose(buf);
}
#[inline]
pub unsafe fn htmlDocDumpMemory(
    mut cur: xmlDocPtr,
    mut mem: *mut *mut xmlChar,
    mut size: *mut c_int,
) {
    htmlDocDumpMemoryFormat(cur, mem, size, 1 as c_int);
}
unsafe fn htmlDtdDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut encoding: *const c_char,
) {
    let mut cur: xmlDtdPtr = (*doc).intSubset as xmlDtdPtr;
    if cur.is_null() {
        htmlSaveErr(
            XML_SAVE_NO_DOCTYPE as c_int,
            doc as xmlNodePtr,
            ::core::ptr::null::<c_char>(),
        );
        return;
    }
    xmlOutputBufferWriteString(
        buf,
        b"<!DOCTYPE \0" as *const u8 as *const c_char,
    );
    xmlOutputBufferWriteString(buf, (*cur).name as *const c_char);
    if !(*cur).ExternalID.is_null() {
        xmlOutputBufferWriteString(
            buf,
            b" PUBLIC \0" as *const u8 as *const c_char,
        );
        xmlBufWriteQuotedString((*buf).buffer, (*cur).ExternalID);
        if !(*cur).SystemID.is_null() {
            xmlOutputBufferWriteString(buf, b" \0" as *const u8 as *const c_char);
            xmlBufWriteQuotedString((*buf).buffer, (*cur).SystemID);
        }
    } else if !(*cur).SystemID.is_null()
        && xmlStrcmp(
            (*cur).SystemID,
            b"about:legacy-compat\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
    {
        xmlOutputBufferWriteString(
            buf,
            b" SYSTEM \0" as *const u8 as *const c_char,
        );
        xmlBufWriteQuotedString((*buf).buffer, (*cur).SystemID);
    }
    xmlOutputBufferWriteString(buf, b">\n\0" as *const u8 as *const c_char);
}
fn htmlAttrDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlAttrPtr,
) { unsafe {
    let mut value: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if cur.is_null() {
        return;
    }
    xmlOutputBufferWriteString(buf, b" \0" as *const u8 as *const c_char);
    if !(*cur).ns.is_null() && !(*(*cur).ns).prefix.is_null() {
        xmlOutputBufferWriteString(buf, (*(*cur).ns).prefix as *const c_char);
        xmlOutputBufferWriteString(buf, b":\0" as *const u8 as *const c_char);
    }
    xmlOutputBufferWriteString(buf, (*cur).name as *const c_char);
    if !(*cur).children.is_null() && htmlIsBooleanAttr((*cur).name) == 0 {
        value = xmlNodeListGetString(doc, (*cur).children, 0 as c_int);
        if !value.is_null() {
            xmlOutputBufferWriteString(buf, b"=\0" as *const u8 as *const c_char);
            if (*cur).ns.is_null()
                && !(*cur).parent.is_null()
                && (*(*cur).parent).ns.is_null()
                && (xmlStrcasecmp(
                    (*cur).name,
                    b"href\0" as *const u8 as *const c_char as *mut xmlChar,
                ) == 0
                    || xmlStrcasecmp(
                        (*cur).name,
                        b"action\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) == 0
                    || xmlStrcasecmp(
                        (*cur).name,
                        b"src\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) == 0
                    || xmlStrcasecmp(
                        (*cur).name,
                        b"name\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) == 0
                        && xmlStrcasecmp(
                            (*(*cur).parent).name,
                            b"a\0" as *const u8 as *const c_char as *mut xmlChar,
                        ) == 0)
            {
                let mut escaped: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                let mut tmp: *mut xmlChar = value;
                while *tmp as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *tmp as c_int
                        && *tmp as c_int <= 0xa as c_int
                    || *tmp as c_int == 0xd as c_int
                {
                    tmp = tmp.offset(1);
                }
                escaped = xmlURIEscapeStr(
                    tmp,
                    b"\"#$%&+,/:;<=>?@[\\]^`{|}\0" as *const u8 as *const c_char
                        as *mut xmlChar,
                );
                if !escaped.is_null() {
                    xmlBufWriteQuotedString((*buf).buffer, escaped);
                    xmlFree.expect("non-null function pointer")(
                        escaped as *mut c_void,
                    );
                } else {
                    xmlBufWriteQuotedString((*buf).buffer, value);
                }
            } else {
                xmlBufWriteQuotedString((*buf).buffer, value);
            }
            xmlFree.expect("non-null function pointer")(value as *mut c_void);
        } else {
            xmlOutputBufferWriteString(buf, b"=\"\"\0" as *const u8 as *const c_char);
        }
    }
} }
#[no_mangle]
pub unsafe extern "C" fn htmlNodeDumpFormatOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut encoding: *const c_char,
    mut format: c_int,
) {
    let mut root: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut info: *const htmlElemDesc = ::core::ptr::null::<htmlElemDesc>();
    xmlInitParser();
    if cur.is_null() || buf.is_null() {
        return;
    }
    root = cur;
    parent = (*cur).parent as xmlNodePtr;
    loop {
        match (*cur).type_0 as c_uint {
            13 | 9 => {
                if !(*(cur as xmlDocPtr)).intSubset.is_null() {
                    htmlDtdDumpOutput(
                        buf,
                        cur as xmlDocPtr,
                        ::core::ptr::null::<c_char>(),
                    );
                }
                if !(*cur).children.is_null() {
                    if (*cur).parent == parent {
                        parent = cur;
                        cur = (*cur).children as xmlNodePtr;
                        continue;
                    }
                } else {
                    xmlOutputBufferWriteString(
                        buf,
                        b"\n\0" as *const u8 as *const c_char,
                    );
                }
            }
            1 => {
                if (*cur).parent != parent && !(*cur).children.is_null() {
                    htmlNodeDumpFormatOutput(buf, doc, cur, encoding, format);
                } else {
                    if (*cur).ns.is_null() {
                        info = htmlTagLookup((*cur).name);
                    } else {
                        info = ::core::ptr::null::<htmlElemDesc>();
                    }
                    xmlOutputBufferWriteString(
                        buf,
                        b"<\0" as *const u8 as *const c_char,
                    );
                    if !(*cur).ns.is_null() && !(*(*cur).ns).prefix.is_null() {
                        xmlOutputBufferWriteString(
                            buf,
                            (*(*cur).ns).prefix as *const c_char,
                        );
                        xmlOutputBufferWriteString(
                            buf,
                            b":\0" as *const u8 as *const c_char,
                        );
                    }
                    xmlOutputBufferWriteString(buf, (*cur).name as *const c_char);
                    if !(*cur).nsDef.is_null() {
                        xmlNsListDumpOutput(buf, (*cur).nsDef as xmlNsPtr);
                    }
                    attr = (*cur).properties as xmlAttrPtr;
                    while !attr.is_null() {
                        htmlAttrDumpOutput(buf, doc, attr);
                        attr = (*attr).next as xmlAttrPtr;
                    }
                    if !info.is_null() && (*info).empty as c_int != 0 {
                        xmlOutputBufferWriteString(
                            buf,
                            b">\0" as *const u8 as *const c_char,
                        );
                    } else if (*cur).children.is_null() {
                        if !info.is_null()
                            && (*info).saveEndTag as c_int != 0 as c_int
                            && xmlStrcmp(
                                (*info).name as *mut xmlChar,
                                b"html\0" as *const u8 as *const c_char
                                    as *mut xmlChar,
                            ) != 0
                            && xmlStrcmp(
                                (*info).name as *mut xmlChar,
                                b"body\0" as *const u8 as *const c_char
                                    as *mut xmlChar,
                            ) != 0
                        {
                            xmlOutputBufferWriteString(
                                buf,
                                b">\0" as *const u8 as *const c_char,
                            );
                        } else {
                            xmlOutputBufferWriteString(
                                buf,
                                b"></\0" as *const u8 as *const c_char,
                            );
                            if !(*cur).ns.is_null() && !(*(*cur).ns).prefix.is_null() {
                                xmlOutputBufferWriteString(
                                    buf,
                                    (*(*cur).ns).prefix as *const c_char,
                                );
                                xmlOutputBufferWriteString(
                                    buf,
                                    b":\0" as *const u8 as *const c_char,
                                );
                            }
                            xmlOutputBufferWriteString(
                                buf,
                                (*cur).name as *const c_char,
                            );
                            xmlOutputBufferWriteString(
                                buf,
                                b">\0" as *const u8 as *const c_char,
                            );
                        }
                    } else {
                        xmlOutputBufferWriteString(
                            buf,
                            b">\0" as *const u8 as *const c_char,
                        );
                        if format != 0
                            && !info.is_null()
                            && (*info).isinline == 0
                            && (*(*cur).children).type_0 as c_uint
                                != XML_TEXT_NODE as c_int as c_uint
                            && (*(*cur).children).type_0 as c_uint
                                != XML_ENTITY_REF_NODE as c_int as c_uint
                            && (*cur).children != (*cur).last
                            && !(*cur).name.is_null()
                            && *(*cur).name.offset(0 as c_int as isize)
                                as c_int
                                != 'p' as i32
                        {
                            xmlOutputBufferWriteString(
                                buf,
                                b"\n\0" as *const u8 as *const c_char,
                            );
                        }
                        parent = cur;
                        cur = (*cur).children as xmlNodePtr;
                        continue;
                    }
                    if format != 0
                        && !(*cur).next.is_null()
                        && !info.is_null()
                        && (*info).isinline == 0
                    {
                        if (*(*cur).next).type_0 as c_uint
                            != XML_TEXT_NODE as c_int as c_uint
                            && (*(*cur).next).type_0 as c_uint
                                != XML_ENTITY_REF_NODE as c_int as c_uint
                            && !parent.is_null()
                            && !(*parent).name.is_null()
                            && *(*parent).name.offset(0 as c_int as isize)
                                as c_int
                                != 'p' as i32
                        {
                            xmlOutputBufferWriteString(
                                buf,
                                b"\n\0" as *const u8 as *const c_char,
                            );
                        }
                    }
                }
            }
            2 => {
                htmlAttrDumpOutput(buf, doc, cur as xmlAttrPtr);
            }
            3 => {
                if !(*cur).content.is_null() {
                    if ((*cur).name == &raw const xmlStringText as *const xmlChar
                        || (*cur).name != &raw const xmlStringTextNoenc as *const xmlChar)
                        && (parent.is_null()
                            || xmlStrcasecmp(
                                (*parent).name,
                                b"script\0" as *const u8 as *const c_char
                                    as *mut xmlChar,
                            ) != 0
                                && xmlStrcasecmp(
                                    (*parent).name,
                                    b"style\0" as *const u8 as *const c_char
                                        as *mut xmlChar,
                                ) != 0)
                    {
                        let mut buffer: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                        buffer = xmlEncodeEntitiesReentrant(doc, (*cur).content);
                        if !buffer.is_null() {
                            xmlOutputBufferWriteString(buf, buffer as *const c_char);
                            xmlFree.expect("non-null function pointer")(
                                buffer as *mut c_void,
                            );
                        }
                    } else {
                        xmlOutputBufferWriteString(
                            buf,
                            (*cur).content as *const c_char,
                        );
                    }
                }
            }
            8 => {
                if !(*cur).content.is_null() {
                    xmlOutputBufferWriteString(
                        buf,
                        b"<!--\0" as *const u8 as *const c_char,
                    );
                    xmlOutputBufferWriteString(buf, (*cur).content as *const c_char);
                    xmlOutputBufferWriteString(
                        buf,
                        b"-->\0" as *const u8 as *const c_char,
                    );
                }
            }
            7 => {
                if !(*cur).name.is_null() {
                    xmlOutputBufferWriteString(
                        buf,
                        b"<?\0" as *const u8 as *const c_char,
                    );
                    xmlOutputBufferWriteString(buf, (*cur).name as *const c_char);
                    if !(*cur).content.is_null() {
                        xmlOutputBufferWriteString(
                            buf,
                            b" \0" as *const u8 as *const c_char,
                        );
                        xmlOutputBufferWriteString(
                            buf,
                            (*cur).content as *const c_char,
                        );
                    }
                    xmlOutputBufferWriteString(
                        buf,
                        b">\0" as *const u8 as *const c_char,
                    );
                }
            }
            5 => {
                xmlOutputBufferWriteString(buf, b"&\0" as *const u8 as *const c_char);
                xmlOutputBufferWriteString(buf, (*cur).name as *const c_char);
                xmlOutputBufferWriteString(buf, b";\0" as *const u8 as *const c_char);
            }
            4 => {
                if !(*cur).content.is_null() {
                    xmlOutputBufferWriteString(buf, (*cur).content as *const c_char);
                }
            }
            _ => {}
        }
        loop {
            if cur == root {
                return;
            }
            if !(*cur).next.is_null() {
                cur = (*cur).next as xmlNodePtr;
                break;
            } else {
                cur = parent;
                parent = (*cur).parent as xmlNodePtr;
                if (*cur).type_0 as c_uint
                    == XML_HTML_DOCUMENT_NODE as c_int as c_uint
                    || (*cur).type_0 as c_uint
                        == XML_DOCUMENT_NODE as c_int as c_uint
                {
                    xmlOutputBufferWriteString(
                        buf,
                        b"\n\0" as *const u8 as *const c_char,
                    );
                } else {
                    if format != 0 && (*cur).ns.is_null() {
                        info = htmlTagLookup((*cur).name);
                    } else {
                        info = ::core::ptr::null::<htmlElemDesc>();
                    }
                    if format != 0
                        && !info.is_null()
                        && (*info).isinline == 0
                        && (*(*cur).last).type_0 as c_uint
                            != XML_TEXT_NODE as c_int as c_uint
                        && (*(*cur).last).type_0 as c_uint
                            != XML_ENTITY_REF_NODE as c_int as c_uint
                        && (*cur).children != (*cur).last
                        && !(*cur).name.is_null()
                        && *(*cur).name.offset(0 as c_int as isize)
                            as c_int
                            != 'p' as i32
                    {
                        xmlOutputBufferWriteString(
                            buf,
                            b"\n\0" as *const u8 as *const c_char,
                        );
                    }
                    xmlOutputBufferWriteString(
                        buf,
                        b"</\0" as *const u8 as *const c_char,
                    );
                    if !(*cur).ns.is_null() && !(*(*cur).ns).prefix.is_null() {
                        xmlOutputBufferWriteString(
                            buf,
                            (*(*cur).ns).prefix as *const c_char,
                        );
                        xmlOutputBufferWriteString(
                            buf,
                            b":\0" as *const u8 as *const c_char,
                        );
                    }
                    xmlOutputBufferWriteString(buf, (*cur).name as *const c_char);
                    xmlOutputBufferWriteString(
                        buf,
                        b">\0" as *const u8 as *const c_char,
                    );
                    if format != 0
                        && !info.is_null()
                        && (*info).isinline == 0
                        && !(*cur).next.is_null()
                    {
                        if (*(*cur).next).type_0 as c_uint
                            != XML_TEXT_NODE as c_int as c_uint
                            && (*(*cur).next).type_0 as c_uint
                                != XML_ENTITY_REF_NODE as c_int as c_uint
                            && !parent.is_null()
                            && !(*parent).name.is_null()
                            && *(*parent).name.offset(0 as c_int as isize)
                                as c_int
                                != 'p' as i32
                        {
                            xmlOutputBufferWriteString(
                                buf,
                                b"\n\0" as *const u8 as *const c_char,
                            );
                        }
                    }
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn htmlNodeDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut doc: xmlDocPtr,
    mut cur: xmlNodePtr,
    mut encoding: *const c_char,
) {
    htmlNodeDumpFormatOutput(
        buf,
        doc,
        cur,
        ::core::ptr::null::<c_char>(),
        1 as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn htmlDocContentDumpFormatOutput(
    mut buf: xmlOutputBufferPtr,
    mut cur: xmlDocPtr,
    mut encoding: *const c_char,
    mut format: c_int,
) {
    let mut type_0: c_int = 0 as c_int;
    if !cur.is_null() {
        type_0 = (*cur).type_0 as c_int;
        (*cur).type_0 = XML_HTML_DOCUMENT_NODE;
    }
    htmlNodeDumpFormatOutput(
        buf,
        cur,
        cur as xmlNodePtr,
        ::core::ptr::null::<c_char>(),
        format,
    );
    if !cur.is_null() {
        (*cur).type_0 = type_0 as xmlElementType;
    }
}
#[inline]
pub unsafe fn htmlDocContentDumpOutput(
    mut buf: xmlOutputBufferPtr,
    mut cur: xmlDocPtr,
    mut encoding: *const c_char,
) {
    htmlNodeDumpFormatOutput(
        buf,
        cur,
        cur as xmlNodePtr,
        ::core::ptr::null::<c_char>(),
        1 as c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn htmlDocDump(mut f: *mut FILE, mut cur: xmlDocPtr) -> c_int {
    let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    let mut handler: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    let mut encoding: *const c_char = ::core::ptr::null::<c_char>();
    let mut ret: c_int = 0;
    xmlInitParser();
    if cur.is_null() || f.is_null() {
        return -(1 as c_int);
    }
    encoding = htmlGetMetaEncoding(cur as htmlDocPtr) as *const c_char;
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as c_int != XML_CHAR_ENCODING_UTF8 as c_int {
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as c_int,
                    ::core::ptr::null_mut::<xmlNode>(),
                    encoding,
                );
            }
        }
    } else {
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"HTML\0" as *const u8 as *const c_char);
        }
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"ascii\0" as *const u8 as *const c_char);
        }
    }
    buf = xmlOutputBufferCreateFile(f, handler);
    if buf.is_null() {
        return -(1 as c_int);
    }
    htmlDocContentDumpOutput(buf, cur, ::core::ptr::null::<c_char>());
    ret = xmlOutputBufferClose(buf);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn htmlSaveFile(
    mut filename: *const c_char,
    mut cur: xmlDocPtr,
) -> c_int {
    let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    let mut handler: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    let mut encoding: *const c_char = ::core::ptr::null::<c_char>();
    let mut ret: c_int = 0;
    if cur.is_null() || filename.is_null() {
        return -(1 as c_int);
    }
    xmlInitParser();
    encoding = htmlGetMetaEncoding(cur as htmlDocPtr) as *const c_char;
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as c_int != XML_CHAR_ENCODING_UTF8 as c_int {
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as c_int,
                    ::core::ptr::null_mut::<xmlNode>(),
                    encoding,
                );
            }
        }
    } else {
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"HTML\0" as *const u8 as *const c_char);
        }
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"ascii\0" as *const u8 as *const c_char);
        }
    }
    buf = xmlOutputBufferCreateFilename(filename, handler, (*cur).compression);
    if buf.is_null() {
        return 0 as c_int;
    }
    htmlDocContentDumpOutput(buf, cur, ::core::ptr::null::<c_char>());
    ret = xmlOutputBufferClose(buf);
    return ret;
}
#[inline]
pub unsafe fn htmlSaveFileFormat(
    mut filename: *const c_char,
    mut cur: xmlDocPtr,
    mut encoding: *const c_char,
    mut format: c_int,
) -> c_int {
    let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
    let mut handler: xmlCharEncodingHandlerPtr = ::core::ptr::null_mut::<xmlCharEncodingHandler>();
    let mut ret: c_int = 0;
    if cur.is_null() || filename.is_null() {
        return -(1 as c_int);
    }
    xmlInitParser();
    if !encoding.is_null() {
        let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
        enc = xmlParseCharEncoding(encoding);
        if enc as c_int != XML_CHAR_ENCODING_UTF8 as c_int {
            handler = xmlFindCharEncodingHandler(encoding);
            if handler.is_null() {
                htmlSaveErr(
                    XML_SAVE_UNKNOWN_ENCODING as c_int,
                    ::core::ptr::null_mut::<xmlNode>(),
                    encoding,
                );
            }
        }
        htmlSetMetaEncoding(cur as htmlDocPtr, encoding as *const xmlChar);
    } else {
        htmlSetMetaEncoding(
            cur as htmlDocPtr,
            b"UTF-8\0" as *const u8 as *const c_char as *const xmlChar,
        );
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"HTML\0" as *const u8 as *const c_char);
        }
        if handler.is_null() {
            handler =
                xmlFindCharEncodingHandler(b"ascii\0" as *const u8 as *const c_char);
        }
    }
    buf = xmlOutputBufferCreateFilename(filename, handler, 0 as c_int);
    if buf.is_null() {
        return 0 as c_int;
    }
    htmlDocContentDumpFormatOutput(buf, cur, encoding, format);
    ret = xmlOutputBufferClose(buf);
    return ret;
}
#[inline]
pub unsafe fn htmlSaveFileEnc(
    mut filename: *const c_char,
    mut cur: xmlDocPtr,
    mut encoding: *const c_char,
) -> c_int {
    return htmlSaveFileFormat(filename, cur, encoding, 1 as c_int);
}

