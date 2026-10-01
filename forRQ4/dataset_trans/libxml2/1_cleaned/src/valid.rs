use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
extern "C" {
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlBuf;
    pub type _xmlRegexp;
    pub type _xmlRegExecCtxt;
    pub type _xmlAttrHashBucket;
    pub type _xmlParserNsData;
    pub type _xmlStartTag;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlLink;
    pub type _xmlList;
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: c_int,
    ) -> *const xmlChar;
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> c_int;
    fn xmlHashCreateDict(size: c_int, dict: xmlDictPtr) -> xmlHashTablePtr;
    fn xmlHashFree(hash: xmlHashTablePtr, dealloc: xmlHashDeallocator);
    fn xmlHashAddEntry(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut c_void,
    ) -> c_int;
    fn xmlHashUpdateEntry(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        userdata: *mut c_void,
        dealloc: xmlHashDeallocator,
    ) -> c_int;
    fn xmlHashAddEntry2(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        userdata: *mut c_void,
    ) -> c_int;
    fn xmlHashAddEntry3(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
        userdata: *mut c_void,
    ) -> c_int;
    fn xmlHashRemoveEntry(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        dealloc: xmlHashDeallocator,
    ) -> c_int;
    fn xmlHashRemoveEntry2(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        dealloc: xmlHashDeallocator,
    ) -> c_int;
    fn xmlHashLookup(hash: xmlHashTablePtr, name: *const xmlChar) -> *mut c_void;
    fn xmlHashLookup2(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
    ) -> *mut c_void;
    fn xmlHashLookup3(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
    ) -> *mut c_void;
    fn xmlHashCopy(hash: xmlHashTablePtr, copy: xmlHashCopier) -> xmlHashTablePtr;
    fn xmlHashScan(hash: xmlHashTablePtr, scan: xmlHashScanner, data: *mut c_void);
    fn xmlHashScan3(
        hash: xmlHashTablePtr,
        name: *const xmlChar,
        name2: *const xmlChar,
        name3: *const xmlChar,
        scan: xmlHashScanner,
        data: *mut c_void,
    );
    fn xmlBuildURI(URI: *const xmlChar, base: *const xmlChar) -> *mut xmlChar;
    fn xmlParseDTD(ExternalID: *const xmlChar, SystemID: *const xmlChar) -> xmlDtdPtr;
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlGetLineNo(node: *const xmlNode) -> c_long;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlIsBlankNode(node: *const xmlNode) -> c_int;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlNodeListGetString(
        doc: xmlDocPtr,
        list: *const xmlNode,
        inLine: c_int,
    ) -> *mut xmlChar;
    fn xmlNewAutomata() -> xmlAutomataPtr;
    fn xmlFreeAutomata(am: xmlAutomataPtr);
    fn xmlAutomataGetInitState(am: xmlAutomataPtr) -> xmlAutomataStatePtr;
    fn xmlAutomataSetFinalState(
        am: xmlAutomataPtr,
        state: xmlAutomataStatePtr,
    ) -> c_int;
    fn xmlAutomataNewState(am: xmlAutomataPtr) -> xmlAutomataStatePtr;
    fn xmlAutomataNewTransition(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
        token: *const xmlChar,
        data: *mut c_void,
    ) -> xmlAutomataStatePtr;
    fn xmlAutomataNewEpsilon(
        am: xmlAutomataPtr,
        from: xmlAutomataStatePtr,
        to: xmlAutomataStatePtr,
    ) -> xmlAutomataStatePtr;
    fn xmlAutomataCompile(am: xmlAutomataPtr) -> *mut _xmlRegexp;
    fn xmlRegFreeRegexp(regexp: xmlRegexpPtr);
    fn xmlRegexpIsDeterminist(comp: xmlRegexpPtr) -> c_int;
    fn xmlRegNewExecCtxt(
        comp: xmlRegexpPtr,
        callback: xmlRegExecCallbacks,
        data: *mut c_void,
    ) -> xmlRegExecCtxtPtr;
    fn xmlRegFreeExecCtxt(exec: xmlRegExecCtxtPtr);
    fn xmlRegExecPushString(
        exec: xmlRegExecCtxtPtr,
        value: *const xmlChar,
        data: *mut c_void,
    ) -> c_int;
    fn xmlGetDocEntity(doc: *const xmlDoc, name: *const xmlChar) -> xmlEntityPtr;
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
    fn xmlListCreate(deallocator: xmlListDeallocator, compare: xmlListDataCompare) -> xmlListPtr;
    fn xmlListDelete(l: xmlListPtr);
    fn xmlListAppend(l: xmlListPtr, data: *mut c_void) -> c_int;
    fn xmlListRemoveFirst(l: xmlListPtr, data: *mut c_void) -> c_int;
    fn xmlListEmpty(l: xmlListPtr) -> c_int;
    fn xmlListWalk(l: xmlListPtr, walker: xmlListWalker, user: *mut c_void);
    fn xmlLinkGetData(lk: xmlLinkPtr) -> *mut c_void;
}

pub type xmlDict = _xmlDict;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlHashTable = _xmlHashTable;
pub type xmlHashTablePtr = *mut xmlHashTable;

pub type C2RustUnnamed = c_uint;
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

pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
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

pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
pub type xmlRegExecCtxt = _xmlRegExecCtxt;
pub type xmlRegExecCtxtPtr = *mut xmlRegExecCtxt;
pub type xmlRegExecCallbacks = Option<
    unsafe extern "C" fn(
        xmlRegExecCtxtPtr,
        *const xmlChar,
        *mut c_void,
        *mut c_void,
    ) -> (),
>;
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
pub type xmlStartTag = _xmlStartTag;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidState {
    pub elemDecl: xmlElementPtr,
    pub node: xmlNodePtr,
    pub exec: xmlRegExecCtxtPtr,
}
pub type xmlElementPtr = *mut xmlElement;
pub type xmlElement = _xmlElement;
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
pub type xmlAttributePtr = *mut xmlAttribute;
pub type xmlAttribute = _xmlAttribute;
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

pub const XML_ATTRIBUTE_FIXED: xmlAttributeDefault = 4;
pub const XML_ATTRIBUTE_IMPLIED: xmlAttributeDefault = 3;
pub const XML_ATTRIBUTE_REQUIRED: xmlAttributeDefault = 2;
pub const XML_ATTRIBUTE_NONE: xmlAttributeDefault = 1;

pub const XML_ELEMENT_TYPE_ELEMENT: xmlElementTypeVal = 4;
pub const XML_ELEMENT_TYPE_MIXED: xmlElementTypeVal = 3;
pub const XML_ELEMENT_TYPE_ANY: xmlElementTypeVal = 2;
pub const XML_ELEMENT_TYPE_EMPTY: xmlElementTypeVal = 1;
pub const XML_ELEMENT_TYPE_UNDEFINED: xmlElementTypeVal = 0;
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
pub type C2RustUnnamed_0 = c_uint;
pub const XML_DOC_HTML: C2RustUnnamed_0 = 128;
pub const XML_DOC_INTERNAL: C2RustUnnamed_0 = 64;
pub const XML_DOC_USERBUILT: C2RustUnnamed_0 = 32;
pub const XML_DOC_XINCLUDE: C2RustUnnamed_0 = 16;
pub const XML_DOC_DTDVALID: C2RustUnnamed_0 = 8;
pub const XML_DOC_OLD10: C2RustUnnamed_0 = 4;
pub const XML_DOC_NSVALID: C2RustUnnamed_0 = 2;
pub const XML_DOC_WELLFORMED: C2RustUnnamed_0 = 1;
pub type xmlLink = _xmlLink;
pub type xmlLinkPtr = *mut xmlLink;
pub type xmlList = _xmlList;
pub type xmlListPtr = *mut xmlList;
pub type xmlListDeallocator = Option<unsafe extern "C" fn(xmlLinkPtr) -> ()>;

pub type xmlValidStatePtr = *mut xmlValidState;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlNotationTable = _xmlHashTable;
pub type xmlNotationTablePtr = *mut xmlNotationTable;
pub type xmlElementTable = _xmlHashTable;
pub type xmlElementTablePtr = *mut xmlElementTable;
pub type xmlAttributeTable = _xmlHashTable;
pub type xmlAttributeTablePtr = *mut xmlAttributeTable;
pub type xmlIDTable = _xmlHashTable;
pub type xmlIDTablePtr = *mut xmlIDTable;
pub type xmlRefTable = _xmlHashTable;
pub type xmlRefTablePtr = *mut xmlRefTable;
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: c_int,
    pub nbLongRange: c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}

pub type xmlRemoveMemo = xmlRemoveMemo_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlRemoveMemo_t {
    pub l: xmlListPtr,
    pub ap: xmlAttrPtr,
}
pub type xmlRemoveMemoPtr = *mut xmlRemoveMemo;
pub type xmlValidateMemo = xmlValidateMemo_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlValidateMemo_t {
    pub ctxt: xmlValidCtxtPtr,
    pub name: *const xmlChar,
}
pub type xmlValidateMemoPtr = *mut xmlValidateMemo;
pub type xmlEntitiesTablePtr = *mut xmlEntitiesTable;
pub type xmlEntitiesTable = _xmlHashTable;

unsafe extern "C" fn xmlVErrMemory(
    mut ctxt: xmlValidCtxtPtr,
    mut extra: *const c_char,
) {
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut data: *mut c_void = NULL;
    if !ctxt.is_null() {
        channel = (*ctxt).error as xmlGenericErrorFunc;
        data = (*ctxt).userData;
        if (*ctxt).flags & XML_VCTXT_USE_PCTXT != 0 {
            pctxt = (*ctxt).userData as xmlParserCtxtPtr;
        }
    }
    if !extra.is_null() {
        __xmlRaiseError(
            None,
            channel,
            data,
            pctxt as *mut c_void,
            NULL,
            XML_FROM_VALID as c_int,
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
            channel,
            data,
            pctxt as *mut c_void,
            NULL,
            XML_FROM_VALID as c_int,
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
unsafe extern "C" fn xmlErrValid(
    mut ctxt: xmlValidCtxtPtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut extra: *const c_char,
) {
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut data: *mut c_void = NULL;
    if !ctxt.is_null() {
        channel = (*ctxt).error as xmlGenericErrorFunc;
        data = (*ctxt).userData;
        if (*ctxt).flags & XML_VCTXT_USE_PCTXT != 0 {
            pctxt = (*ctxt).userData as xmlParserCtxtPtr;
        }
    }
    if !extra.is_null() {
        __xmlRaiseError(
            None,
            channel,
            data,
            pctxt as *mut c_void,
            NULL,
            XML_FROM_VALID as c_int,
            error as c_int,
            XML_ERR_ERROR,
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            extra,
            ::core::ptr::null::<c_char>(),
            ::core::ptr::null::<c_char>(),
            0 as c_int,
            0 as c_int,
            msg,
            extra,
        );
    } else {
        __xmlRaiseError(
            None,
            channel,
            data,
            pctxt as *mut c_void,
            NULL,
            XML_FROM_VALID as c_int,
            error as c_int,
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
    };
}
unsafe extern "C" fn xmlErrValidNode(
    mut ctxt: xmlValidCtxtPtr,
    mut node: xmlNodePtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut str3: *const xmlChar,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut data: *mut c_void = NULL;
    if !ctxt.is_null() {
        channel = (*ctxt).error as xmlGenericErrorFunc;
        data = (*ctxt).userData;
        if (*ctxt).flags & XML_VCTXT_USE_PCTXT != 0 {
            pctxt = (*ctxt).userData as xmlParserCtxtPtr;
        }
    }
    __xmlRaiseError(
        schannel,
        channel,
        data,
        pctxt as *mut c_void,
        node as *mut c_void,
        XML_FROM_VALID as c_int,
        error as c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        str1 as *const c_char,
        str2 as *const c_char,
        str3 as *const c_char,
        0 as c_int,
        0 as c_int,
        msg,
        str1,
        str2,
        str3,
    );
}
unsafe extern "C" fn xmlErrValidNodeNr(
    mut ctxt: xmlValidCtxtPtr,
    mut node: xmlNodePtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut str1: *const xmlChar,
    mut int2: c_int,
    mut str3: *const xmlChar,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut data: *mut c_void = NULL;
    if !ctxt.is_null() {
        channel = (*ctxt).error as xmlGenericErrorFunc;
        data = (*ctxt).userData;
        if (*ctxt).flags & XML_VCTXT_USE_PCTXT != 0 {
            pctxt = (*ctxt).userData as xmlParserCtxtPtr;
        }
    }
    __xmlRaiseError(
        schannel,
        channel,
        data,
        pctxt as *mut c_void,
        node as *mut c_void,
        XML_FROM_VALID as c_int,
        error as c_int,
        XML_ERR_ERROR,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        str1 as *const c_char,
        str3 as *const c_char,
        ::core::ptr::null::<c_char>(),
        int2,
        0 as c_int,
        msg,
        str1,
        int2,
        str3,
    );
}
unsafe extern "C" fn xmlErrValidWarning(
    mut ctxt: xmlValidCtxtPtr,
    mut node: xmlNodePtr,
    mut error: xmlParserErrors,
    mut msg: *const c_char,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
    mut str3: *const xmlChar,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut pctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    let mut data: *mut c_void = NULL;
    if !ctxt.is_null() {
        channel = (*ctxt).warning as xmlGenericErrorFunc;
        data = (*ctxt).userData;
        if (*ctxt).flags & XML_VCTXT_USE_PCTXT != 0 {
            pctxt = (*ctxt).userData as xmlParserCtxtPtr;
        }
    }
    __xmlRaiseError(
        schannel,
        channel,
        data,
        pctxt as *mut c_void,
        node as *mut c_void,
        XML_FROM_VALID as c_int,
        error as c_int,
        XML_ERR_WARNING,
        ::core::ptr::null::<c_char>(),
        0 as c_int,
        str1 as *const c_char,
        str2 as *const c_char,
        str3 as *const c_char,
        0 as c_int,
        0 as c_int,
        msg,
        str1,
        str2,
        str3,
    );
}
unsafe extern "C" fn vstateVPush(
    mut ctxt: xmlValidCtxtPtr,
    mut elemDecl: xmlElementPtr,
    mut node: xmlNodePtr,
) -> c_int {
    if (*ctxt).vstateMax == 0 as c_int || (*ctxt).vstateTab.is_null() {
        (*ctxt).vstateMax = 10 as c_int;
        (*ctxt).vstateTab = xmlMalloc.expect("non-null function pointer")(
            ((*ctxt).vstateMax as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlValidState>() as size_t),
        ) as *mut xmlValidState;
        if (*ctxt).vstateTab.is_null() {
            xmlVErrMemory(
                ctxt,
                b"malloc failed\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
    }
    if (*ctxt).vstateNr >= (*ctxt).vstateMax {
        let mut tmp: *mut xmlValidState = ::core::ptr::null_mut::<xmlValidState>();
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).vstateTab as *mut c_void,
            ((2 as c_int * (*ctxt).vstateMax) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlValidState>() as size_t),
        ) as *mut xmlValidState;
        if tmp.is_null() {
            xmlVErrMemory(
                ctxt,
                b"realloc failed\0" as *const u8 as *const c_char,
            );
            return -(1 as c_int);
        }
        (*ctxt).vstateMax *= 2 as c_int;
        (*ctxt).vstateTab = tmp;
    }
    (*ctxt).vstate = (*ctxt).vstateTab.offset((*ctxt).vstateNr as isize) as *mut xmlValidState;
    let ref mut fresh12 = (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).elemDecl;
    *fresh12 = elemDecl;
    let ref mut fresh13 = (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).node;
    *fresh13 = node;
    if !elemDecl.is_null()
        && (*elemDecl).etype as c_uint
            == XML_ELEMENT_TYPE_ELEMENT as c_int as c_uint
    {
        if (*elemDecl).contModel.is_null() {
            xmlValidBuildContentModel(ctxt, elemDecl);
        }
        if !(*elemDecl).contModel.is_null() {
            let ref mut fresh14 = (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).exec;
            *fresh14 = xmlRegNewExecCtxt((*elemDecl).contModel, None, NULL);
        } else {
            let ref mut fresh15 = (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).exec;
            *fresh15 = ::core::ptr::null_mut::<xmlRegExecCtxt>();
            xmlErrValidNode(
                ctxt,
                elemDecl as xmlNodePtr,
                XML_ERR_INTERNAL_ERROR,
                b"Failed to build content model regexp for %s\n\0" as *const u8
                    as *const c_char,
                (*node).name,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
        }
    }
    let fresh16 = (*ctxt).vstateNr;
    (*ctxt).vstateNr = (*ctxt).vstateNr + 1;
    return fresh16;
}
unsafe extern "C" fn vstateVPop(mut ctxt: xmlValidCtxtPtr) -> c_int {
    let mut elemDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    if (*ctxt).vstateNr < 1 as c_int {
        return -(1 as c_int);
    }
    (*ctxt).vstateNr -= 1;
    elemDecl = (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).elemDecl;
    let ref mut fresh17 = (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).elemDecl;
    *fresh17 = ::core::ptr::null_mut::<xmlElement>();
    let ref mut fresh18 = (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).node;
    *fresh18 = ::core::ptr::null_mut::<xmlNode>();
    if !elemDecl.is_null()
        && (*elemDecl).etype as c_uint
            == XML_ELEMENT_TYPE_ELEMENT as c_int as c_uint
    {
        xmlRegFreeExecCtxt((*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).exec);
    }
    let ref mut fresh19 = (*(*ctxt).vstateTab.offset((*ctxt).vstateNr as isize)).exec;
    *fresh19 = ::core::ptr::null_mut::<xmlRegExecCtxt>();
    if (*ctxt).vstateNr >= 1 as c_int {
        (*ctxt).vstate = (*ctxt)
            .vstateTab
            .offset(((*ctxt).vstateNr - 1 as c_int) as isize)
            as *mut xmlValidState;
    } else {
        (*ctxt).vstate = ::core::ptr::null_mut::<xmlValidState>();
    }
    return (*ctxt).vstateNr;
}
unsafe extern "C" fn nodeVPush(
    mut ctxt: xmlValidCtxtPtr,
    mut value: xmlNodePtr,
) -> c_int {
    if (*ctxt).nodeMax <= 0 as c_int {
        (*ctxt).nodeMax = 4 as c_int;
        (*ctxt).nodeTab = xmlMalloc.expect("non-null function pointer")(
            ((*ctxt).nodeMax as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if (*ctxt).nodeTab.is_null() {
            xmlVErrMemory(
                ctxt,
                b"malloc failed\0" as *const u8 as *const c_char,
            );
            (*ctxt).nodeMax = 0 as c_int;
            return 0 as c_int;
        }
    }
    if (*ctxt).nodeNr >= (*ctxt).nodeMax {
        let mut tmp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*ctxt).nodeTab as *mut c_void,
            (((*ctxt).nodeMax * 2 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if tmp.is_null() {
            xmlVErrMemory(
                ctxt,
                b"realloc failed\0" as *const u8 as *const c_char,
            );
            return 0 as c_int;
        }
        (*ctxt).nodeMax *= 2 as c_int;
        (*ctxt).nodeTab = tmp;
    }
    let ref mut fresh4 = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    *fresh4 = value;
    (*ctxt).node = value;
    let fresh5 = (*ctxt).nodeNr;
    (*ctxt).nodeNr = (*ctxt).nodeNr + 1;
    return fresh5;
}
unsafe extern "C" fn nodeVPop(mut ctxt: xmlValidCtxtPtr) -> xmlNodePtr {
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if (*ctxt).nodeNr <= 0 as c_int {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    (*ctxt).nodeNr -= 1;
    if (*ctxt).nodeNr > 0 as c_int {
        (*ctxt).node = *(*ctxt)
            .nodeTab
            .offset(((*ctxt).nodeNr - 1 as c_int) as isize);
    } else {
        (*ctxt).node = ::core::ptr::null_mut::<xmlNode>();
    }
    ret = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    let ref mut fresh3 = *(*ctxt).nodeTab.offset((*ctxt).nodeNr as isize);
    *fresh3 = ::core::ptr::null_mut::<xmlNode>();
    return ret;
}
unsafe extern "C" fn xmlValidBuildAContentModel(
    mut content: xmlElementContentPtr,
    mut ctxt: xmlValidCtxtPtr,
    mut name: *const xmlChar,
) -> c_int {
    if content.is_null() {
        xmlErrValidNode(
            ctxt,
            ::core::ptr::null_mut::<xmlNode>(),
            XML_ERR_INTERNAL_ERROR,
            b"Found NULL content in content model of %s\n\0" as *const u8
                as *const c_char,
            name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        return 0 as c_int;
    }
    match (*content).type_0 as c_uint {
        1 => {
            xmlErrValidNode(
                ctxt,
                ::core::ptr::null_mut::<xmlNode>(),
                XML_ERR_INTERNAL_ERROR,
                b"Found PCDATA in content model of %s\n\0" as *const u8
                    as *const c_char,
                name,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            return 0 as c_int;
        }
        2 => {
            let mut oldstate: xmlAutomataStatePtr = (*ctxt).state;
            let mut fn_0: [xmlChar; 50] = [0; 50];
            let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            fullname = xmlBuildQName(
                (*content).name,
                (*content).prefix,
                &raw mut fn_0 as *mut xmlChar,
                50 as c_int,
            );
            if fullname.is_null() {
                xmlVErrMemory(
                    ctxt,
                    b"Building content model\0" as *const u8 as *const c_char,
                );
                return 0 as c_int;
            }
            match (*content).ocur as c_uint {
                1 => {
                    (*ctxt).state = xmlAutomataNewTransition(
                        (*ctxt).am,
                        (*ctxt).state,
                        ::core::ptr::null_mut::<xmlAutomataState>(),
                        fullname,
                        NULL,
                    );
                }
                2 => {
                    (*ctxt).state = xmlAutomataNewTransition(
                        (*ctxt).am,
                        (*ctxt).state,
                        ::core::ptr::null_mut::<xmlAutomataState>(),
                        fullname,
                        NULL,
                    );
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate, (*ctxt).state);
                }
                4 => {
                    (*ctxt).state = xmlAutomataNewTransition(
                        (*ctxt).am,
                        (*ctxt).state,
                        ::core::ptr::null_mut::<xmlAutomataState>(),
                        fullname,
                        NULL,
                    );
                    xmlAutomataNewTransition(
                        (*ctxt).am,
                        (*ctxt).state,
                        (*ctxt).state,
                        fullname,
                        NULL,
                    );
                }
                3 => {
                    (*ctxt).state = xmlAutomataNewEpsilon(
                        (*ctxt).am,
                        (*ctxt).state,
                        ::core::ptr::null_mut::<xmlAutomataState>(),
                    );
                    xmlAutomataNewTransition(
                        (*ctxt).am,
                        (*ctxt).state,
                        (*ctxt).state,
                        fullname,
                        NULL,
                    );
                }
                _ => {}
            }
            if fullname != &raw mut fn_0 as *mut xmlChar
                && fullname != (*content).name as *mut xmlChar
            {
                xmlFree.expect("non-null function pointer")(fullname as *mut c_void);
            }
        }
        3 => {
            let mut oldstate_0: xmlAutomataStatePtr = ::core::ptr::null_mut::<xmlAutomataState>();
            let mut oldend: xmlAutomataStatePtr = ::core::ptr::null_mut::<xmlAutomataState>();
            let mut ocur: xmlElementContentOccur = 0 as xmlElementContentOccur;
            oldstate_0 = (*ctxt).state;
            ocur = (*content).ocur;
            if ocur as c_uint
                != XML_ELEMENT_CONTENT_ONCE as c_int as c_uint
            {
                (*ctxt).state = xmlAutomataNewEpsilon(
                    (*ctxt).am,
                    oldstate_0,
                    ::core::ptr::null_mut::<xmlAutomataState>(),
                );
                oldstate_0 = (*ctxt).state;
            }
            loop {
                xmlValidBuildAContentModel((*content).c1 as xmlElementContentPtr, ctxt, name);
                content = (*content).c2 as xmlElementContentPtr;
                if !((*content).type_0 as c_uint
                    == XML_ELEMENT_CONTENT_SEQ as c_int as c_uint
                    && (*content).ocur as c_uint
                        == XML_ELEMENT_CONTENT_ONCE as c_int as c_uint)
                {
                    break;
                }
            }
            xmlValidBuildAContentModel(content, ctxt, name);
            oldend = (*ctxt).state;
            (*ctxt).state = xmlAutomataNewEpsilon(
                (*ctxt).am,
                oldend,
                ::core::ptr::null_mut::<xmlAutomataState>(),
            );
            match ocur as c_uint {
                2 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_0, (*ctxt).state);
                }
                3 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_0, (*ctxt).state);
                    xmlAutomataNewEpsilon((*ctxt).am, oldend, oldstate_0);
                }
                4 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldend, oldstate_0);
                }
                1 | _ => {}
            }
        }
        4 => {
            let mut oldstate_1: xmlAutomataStatePtr = ::core::ptr::null_mut::<xmlAutomataState>();
            let mut oldend_0: xmlAutomataStatePtr = ::core::ptr::null_mut::<xmlAutomataState>();
            let mut ocur_0: xmlElementContentOccur = 0 as xmlElementContentOccur;
            ocur_0 = (*content).ocur;
            if ocur_0 as c_uint
                == XML_ELEMENT_CONTENT_PLUS as c_int as c_uint
                || ocur_0 as c_uint
                    == XML_ELEMENT_CONTENT_MULT as c_int as c_uint
            {
                (*ctxt).state = xmlAutomataNewEpsilon(
                    (*ctxt).am,
                    (*ctxt).state,
                    ::core::ptr::null_mut::<xmlAutomataState>(),
                );
            }
            oldstate_1 = (*ctxt).state;
            oldend_0 = xmlAutomataNewState((*ctxt).am);
            loop {
                (*ctxt).state = oldstate_1;
                xmlValidBuildAContentModel((*content).c1 as xmlElementContentPtr, ctxt, name);
                xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, oldend_0);
                content = (*content).c2 as xmlElementContentPtr;
                if !((*content).type_0 as c_uint
                    == XML_ELEMENT_CONTENT_OR as c_int as c_uint
                    && (*content).ocur as c_uint
                        == XML_ELEMENT_CONTENT_ONCE as c_int as c_uint)
                {
                    break;
                }
            }
            (*ctxt).state = oldstate_1;
            xmlValidBuildAContentModel(content, ctxt, name);
            xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, oldend_0);
            (*ctxt).state = xmlAutomataNewEpsilon(
                (*ctxt).am,
                oldend_0,
                ::core::ptr::null_mut::<xmlAutomataState>(),
            );
            match ocur_0 as c_uint {
                2 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_1, (*ctxt).state);
                }
                3 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldstate_1, (*ctxt).state);
                    xmlAutomataNewEpsilon((*ctxt).am, oldend_0, oldstate_1);
                }
                4 => {
                    xmlAutomataNewEpsilon((*ctxt).am, oldend_0, oldstate_1);
                }
                1 | _ => {}
            }
        }
        _ => {
            xmlErrValid(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"ContentModel broken for element %s\n\0" as *const u8
                    as *const c_char,
                name as *const c_char,
            );
            return 0 as c_int;
        }
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidBuildContentModel(
    mut ctxt: xmlValidCtxtPtr,
    mut elem: xmlElementPtr,
) -> c_int {
    if ctxt.is_null() || elem.is_null() {
        return 0 as c_int;
    }
    if (*elem).type_0 as c_uint
        != XML_ELEMENT_DECL as c_int as c_uint
    {
        return 0 as c_int;
    }
    if (*elem).etype as c_uint
        != XML_ELEMENT_TYPE_ELEMENT as c_int as c_uint
    {
        return 1 as c_int;
    }
    if !(*elem).contModel.is_null() {
        if xmlRegexpIsDeterminist((*elem).contModel) == 0 {
            (*ctxt).valid = 0 as c_int;
            return 0 as c_int;
        }
        return 1 as c_int;
    }
    (*ctxt).am = xmlNewAutomata();
    if (*ctxt).am.is_null() {
        xmlErrValidNode(
            ctxt,
            elem as xmlNodePtr,
            XML_ERR_INTERNAL_ERROR,
            b"Cannot create automata for element %s\n\0" as *const u8 as *const c_char,
            (*elem).name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        return 0 as c_int;
    }
    (*ctxt).state = xmlAutomataGetInitState((*ctxt).am);
    xmlValidBuildAContentModel((*elem).content, ctxt, (*elem).name);
    xmlAutomataSetFinalState((*ctxt).am, (*ctxt).state);
    (*elem).contModel = xmlAutomataCompile((*ctxt).am) as xmlRegexpPtr;
    if xmlRegexpIsDeterminist((*elem).contModel) != 1 as c_int {
        let mut expr: [c_char; 5000] = [0; 5000];
        expr[0 as c_int as usize] = 0 as c_char;
        xmlSnprintfElementContent(
            &raw mut expr as *mut c_char,
            5000 as c_int,
            (*elem).content,
            1 as c_int,
        );
        xmlErrValidNode(
            ctxt,
            elem as xmlNodePtr,
            XML_DTD_CONTENT_NOT_DETERMINIST,
            b"Content model of %s is not deterministic: %s\n\0" as *const u8
                as *const c_char,
            (*elem).name,
            &raw mut expr as *mut c_char as *mut xmlChar,
            ::core::ptr::null::<xmlChar>(),
        );
        (*ctxt).valid = 0 as c_int;
        (*ctxt).state = ::core::ptr::null_mut::<xmlAutomataState>();
        xmlFreeAutomata((*ctxt).am);
        (*ctxt).am = ::core::ptr::null_mut::<xmlAutomata>();
        return 0 as c_int;
    }
    (*ctxt).state = ::core::ptr::null_mut::<xmlAutomataState>();
    xmlFreeAutomata((*ctxt).am);
    (*ctxt).am = ::core::ptr::null_mut::<xmlAutomata>();
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewValidCtxt() -> xmlValidCtxtPtr {
    let mut ret: xmlValidCtxtPtr = ::core::ptr::null_mut::<xmlValidCtxt>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlValidCtxt>() as size_t
    ) as xmlValidCtxtPtr;
    if ret.is_null() {
        xmlVErrMemory(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlValidCtxt>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlValidCtxt>() as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeValidCtxt(mut cur: xmlValidCtxtPtr) {
    if cur.is_null() {
        return;
    }
    if !(*cur).vstateTab.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).vstateTab as *mut c_void);
    }
    if !(*cur).nodeTab.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).nodeTab as *mut c_void);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewDocElementContent(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut type_0: xmlElementContentType,
) -> xmlElementContentPtr {
    let mut ret: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if !doc.is_null() {
        dict = (*doc).dict as xmlDictPtr;
    }
    match type_0 as c_uint {
        2 => {
            if name.is_null() {
                xmlErrValid(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlNewElementContent : name == NULL !\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
            }
        }
        1 | 3 | 4 => {
            if !name.is_null() {
                xmlErrValid(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlNewElementContent : name != NULL !\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
            }
        }
        _ => {
            xmlErrValid(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                XML_ERR_INTERNAL_ERROR,
                b"Internal: ELEMENT content corrupted invalid type\n\0" as *const u8
                    as *const c_char,
                ::core::ptr::null::<c_char>(),
            );
            return ::core::ptr::null_mut::<xmlElementContent>();
        }
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlElementContent>() as size_t
    ) as xmlElementContentPtr;
    if ret.is_null() {
        xmlVErrMemory(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlElementContent>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlElementContent>() as size_t,
    );
    (*ret).type_0 = type_0;
    (*ret).ocur = XML_ELEMENT_CONTENT_ONCE;
    if !name.is_null() {
        let mut l: c_int = 0;
        let mut tmp: *const xmlChar = ::core::ptr::null::<xmlChar>();
        tmp = xmlSplitQName3(name, &raw mut l);
        if tmp.is_null() {
            if dict.is_null() {
                (*ret).name = xmlStrdup(name);
            } else {
                (*ret).name = xmlDictLookup(dict, name, -(1 as c_int));
            }
        } else if dict.is_null() {
            (*ret).prefix = xmlStrndup(name, l);
            (*ret).name = xmlStrdup(tmp);
        } else {
            (*ret).prefix = xmlDictLookup(dict, name, l);
            (*ret).name = xmlDictLookup(dict, tmp, -(1 as c_int));
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewElementContent(
    mut name: *const xmlChar,
    mut type_0: xmlElementContentType,
) -> xmlElementContentPtr {
    return xmlNewDocElementContent(::core::ptr::null_mut::<xmlDoc>(), name, type_0);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCopyDocElementContent(
    mut doc: xmlDocPtr,
    mut cur: xmlElementContentPtr,
) -> xmlElementContentPtr {
    let mut ret: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut prev: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut tmp: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlElementContent>();
    }
    if !doc.is_null() {
        dict = (*doc).dict as xmlDictPtr;
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlElementContent>() as size_t
    ) as xmlElementContentPtr;
    if ret.is_null() {
        xmlVErrMemory(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlElementContent>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlElementContent>() as size_t,
    );
    (*ret).type_0 = (*cur).type_0;
    (*ret).ocur = (*cur).ocur;
    if !(*cur).name.is_null() {
        if !dict.is_null() {
            (*ret).name = xmlDictLookup(dict, (*cur).name, -(1 as c_int));
        } else {
            (*ret).name = xmlStrdup((*cur).name);
        }
    }
    if !(*cur).prefix.is_null() {
        if !dict.is_null() {
            (*ret).prefix = xmlDictLookup(dict, (*cur).prefix, -(1 as c_int));
        } else {
            (*ret).prefix = xmlStrdup((*cur).prefix);
        }
    }
    if !(*cur).c1.is_null() {
        (*ret).c1 = xmlCopyDocElementContent(doc, (*cur).c1 as xmlElementContentPtr)
            as *mut _xmlElementContent;
    }
    if !(*ret).c1.is_null() {
        (*(*ret).c1).parent = ret as *mut _xmlElementContent;
    }
    if !(*cur).c2.is_null() {
        prev = ret;
        cur = (*cur).c2 as xmlElementContentPtr;
        while !cur.is_null() {
            tmp = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<
                xmlElementContent,
            >() as size_t) as xmlElementContentPtr;
            if tmp.is_null() {
                xmlVErrMemory(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    b"malloc failed\0" as *const u8 as *const c_char,
                );
                return ret;
            }
            memset(
                tmp as *mut c_void,
                0 as c_int,
                ::core::mem::size_of::<xmlElementContent>() as size_t,
            );
            (*tmp).type_0 = (*cur).type_0;
            (*tmp).ocur = (*cur).ocur;
            (*prev).c2 = tmp as *mut _xmlElementContent;
            (*tmp).parent = prev as *mut _xmlElementContent;
            if !(*cur).name.is_null() {
                if !dict.is_null() {
                    (*tmp).name = xmlDictLookup(dict, (*cur).name, -(1 as c_int));
                } else {
                    (*tmp).name = xmlStrdup((*cur).name);
                }
            }
            if !(*cur).prefix.is_null() {
                if !dict.is_null() {
                    (*tmp).prefix = xmlDictLookup(dict, (*cur).prefix, -(1 as c_int));
                } else {
                    (*tmp).prefix = xmlStrdup((*cur).prefix);
                }
            }
            if !(*cur).c1.is_null() {
                (*tmp).c1 = xmlCopyDocElementContent(doc, (*cur).c1 as xmlElementContentPtr)
                    as *mut _xmlElementContent;
            }
            if !(*tmp).c1.is_null() {
                (*(*tmp).c1).parent = tmp as *mut _xmlElementContent;
            }
            prev = tmp;
            cur = (*cur).c2 as xmlElementContentPtr;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCopyElementContent(
    mut cur: xmlElementContentPtr,
) -> xmlElementContentPtr {
    return xmlCopyDocElementContent(::core::ptr::null_mut::<xmlDoc>(), cur);
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeDocElementContent(
    mut doc: xmlDocPtr,
    mut cur: xmlElementContentPtr,
) {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    let mut depth: size_t = 0 as size_t;
    if cur.is_null() {
        return;
    }
    if !doc.is_null() {
        dict = (*doc).dict as xmlDictPtr;
    }
    loop {
        let mut parent: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
        while !(*cur).c1.is_null() || !(*cur).c2.is_null() {
            cur = (if !(*cur).c1.is_null() {
                (*cur).c1
            } else {
                (*cur).c2
            }) as xmlElementContentPtr;
            depth = (depth as c_ulong).wrapping_add(1 as c_ulong)
                as size_t as size_t;
        }
        match (*cur).type_0 as c_uint {
            1 | 2 | 3 | 4 => {}
            _ => {
                xmlErrValid(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    XML_ERR_INTERNAL_ERROR,
                    b"Internal: ELEMENT content corrupted invalid type\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
                return;
            }
        }
        if !dict.is_null() {
            if !(*cur).name.is_null() && xmlDictOwns(dict, (*cur).name) == 0 {
                xmlFree.expect("non-null function pointer")(
                    (*cur).name as *mut xmlChar as *mut c_void,
                );
            }
            if !(*cur).prefix.is_null() && xmlDictOwns(dict, (*cur).prefix) == 0 {
                xmlFree.expect("non-null function pointer")(
                    (*cur).prefix as *mut xmlChar as *mut c_void,
                );
            }
        } else {
            if !(*cur).name.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*cur).name as *mut xmlChar as *mut c_void,
                );
            }
            if !(*cur).prefix.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*cur).prefix as *mut xmlChar as *mut c_void,
                );
            }
        }
        parent = (*cur).parent as xmlElementContentPtr;
        if depth == 0 as size_t || parent.is_null() {
            xmlFree.expect("non-null function pointer")(cur as *mut c_void);
            break;
        } else {
            if cur == (*parent).c1 {
                (*parent).c1 = ::core::ptr::null_mut::<_xmlElementContent>();
            } else {
                (*parent).c2 = ::core::ptr::null_mut::<_xmlElementContent>();
            }
            xmlFree.expect("non-null function pointer")(cur as *mut c_void);
            if !(*parent).c2.is_null() {
                cur = (*parent).c2 as xmlElementContentPtr;
            } else {
                depth = (depth as c_ulong).wrapping_sub(1 as c_ulong)
                    as size_t as size_t;
                cur = parent;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeElementContent(mut cur: xmlElementContentPtr) {
    xmlFreeDocElementContent(::core::ptr::null_mut::<xmlDoc>(), cur);
}
unsafe extern "C" fn xmlDumpElementOccur(mut buf: xmlBufferPtr, mut cur: xmlElementContentPtr) {
    match (*cur).ocur as c_uint {
        2 => {
            xmlBufferWriteChar(buf, b"?\0" as *const u8 as *const c_char);
        }
        3 => {
            xmlBufferWriteChar(buf, b"*\0" as *const u8 as *const c_char);
        }
        4 => {
            xmlBufferWriteChar(buf, b"+\0" as *const u8 as *const c_char);
        }
        1 | _ => {}
    };
}
unsafe extern "C" fn xmlDumpElementContent(
    mut buf: xmlBufferPtr,
    mut content: xmlElementContentPtr,
) {
    let mut cur: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    if content.is_null() {
        return;
    }
    xmlBufferWriteChar(buf, b"(\0" as *const u8 as *const c_char);
    cur = content;
    let mut current_block_27: u64;
    loop {
        if cur.is_null() {
            return;
        }
        match (*cur).type_0 as c_uint {
            1 => {
                xmlBufferWriteChar(buf, b"#PCDATA\0" as *const u8 as *const c_char);
                current_block_27 = 7149356873433890176;
            }
            2 => {
                if !(*cur).prefix.is_null() {
                    xmlBufferWriteCHAR(buf, (*cur).prefix);
                    xmlBufferWriteChar(buf, b":\0" as *const u8 as *const c_char);
                }
                xmlBufferWriteCHAR(buf, (*cur).name);
                current_block_27 = 7149356873433890176;
            }
            3 | 4 => {
                if cur != content
                    && !(*cur).parent.is_null()
                    && ((*cur).type_0 as c_uint
                        != (*(*cur).parent).type_0 as c_uint
                        || (*cur).ocur as c_uint
                            != XML_ELEMENT_CONTENT_ONCE as c_int
                                as c_uint)
                {
                    xmlBufferWriteChar(buf, b"(\0" as *const u8 as *const c_char);
                }
                cur = (*cur).c1 as xmlElementContentPtr;
                current_block_27 = 12675440807659640239;
            }
            _ => {
                xmlErrValid(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    XML_ERR_INTERNAL_ERROR,
                    b"Internal: ELEMENT cur corrupted invalid type\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
                current_block_27 = 7149356873433890176;
            }
        }
        match current_block_27 {
            7149356873433890176 => {
                while cur != content {
                    let mut parent: xmlElementContentPtr = (*cur).parent as xmlElementContentPtr;
                    if parent.is_null() {
                        return;
                    }
                    if ((*cur).type_0 as c_uint
                        == XML_ELEMENT_CONTENT_OR as c_int as c_uint
                        || (*cur).type_0 as c_uint
                            == XML_ELEMENT_CONTENT_SEQ as c_int as c_uint)
                        && ((*cur).type_0 as c_uint
                            != (*parent).type_0 as c_uint
                            || (*cur).ocur as c_uint
                                != XML_ELEMENT_CONTENT_ONCE as c_int
                                    as c_uint)
                    {
                        xmlBufferWriteChar(buf, b")\0" as *const u8 as *const c_char);
                    }
                    xmlDumpElementOccur(buf, cur);
                    if cur == (*parent).c1 {
                        if (*parent).type_0 as c_uint
                            == XML_ELEMENT_CONTENT_SEQ as c_int as c_uint
                        {
                            xmlBufferWriteChar(
                                buf,
                                b" , \0" as *const u8 as *const c_char,
                            );
                        } else if (*parent).type_0 as c_uint
                            == XML_ELEMENT_CONTENT_OR as c_int as c_uint
                        {
                            xmlBufferWriteChar(
                                buf,
                                b" | \0" as *const u8 as *const c_char,
                            );
                        }
                        cur = (*parent).c2 as xmlElementContentPtr;
                        break;
                    } else {
                        cur = parent;
                    }
                }
            }
            _ => {}
        }
        if !(cur != content) {
            break;
        }
    }
    xmlBufferWriteChar(buf, b")\0" as *const u8 as *const c_char);
    xmlDumpElementOccur(buf, content);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSprintfElementContent(
    mut buf: *mut c_char,
    mut content: xmlElementContentPtr,
    mut englob: c_int,
) {
}
#[no_mangle]
pub unsafe extern "C" fn xmlSnprintfElementContent(
    mut buf: *mut c_char,
    mut size: c_int,
    mut content: xmlElementContentPtr,
    mut englob: c_int,
) {
    let mut len: c_int = 0;
    if content.is_null() {
        return;
    }
    len = strlen(buf) as c_int;
    if size - len < 50 as c_int {
        if size - len > 4 as c_int
            && *buf.offset((len - 1 as c_int) as isize) as c_int
                != '.' as i32
        {
            strcat(buf, b" ...\0" as *const u8 as *const c_char);
        }
        return;
    }
    if englob != 0 {
        strcat(buf, b"(\0" as *const u8 as *const c_char);
    }
    match (*content).type_0 as c_uint {
        1 => {
            strcat(buf, b"#PCDATA\0" as *const u8 as *const c_char);
        }
        2 => {
            let mut qnameLen: c_int = xmlStrlen((*content).name);
            if !(*content).prefix.is_null() {
                qnameLen += xmlStrlen((*content).prefix) + 1 as c_int;
            }
            if size - len < qnameLen + 10 as c_int {
                strcat(buf, b" ...\0" as *const u8 as *const c_char);
                return;
            }
            if !(*content).prefix.is_null() {
                strcat(buf, (*content).prefix as *mut c_char);
                strcat(buf, b":\0" as *const u8 as *const c_char);
            }
            if !(*content).name.is_null() {
                strcat(buf, (*content).name as *mut c_char);
            }
        }
        3 => {
            if (*(*content).c1).type_0 as c_uint
                == XML_ELEMENT_CONTENT_OR as c_int as c_uint
                || (*(*content).c1).type_0 as c_uint
                    == XML_ELEMENT_CONTENT_SEQ as c_int as c_uint
            {
                xmlSnprintfElementContent(
                    buf,
                    size,
                    (*content).c1 as xmlElementContentPtr,
                    1 as c_int,
                );
            } else {
                xmlSnprintfElementContent(
                    buf,
                    size,
                    (*content).c1 as xmlElementContentPtr,
                    0 as c_int,
                );
            }
            len = strlen(buf) as c_int;
            if size - len < 50 as c_int {
                if size - len > 4 as c_int
                    && *buf.offset((len - 1 as c_int) as isize) as c_int
                        != '.' as i32
                {
                    strcat(buf, b" ...\0" as *const u8 as *const c_char);
                }
                return;
            }
            strcat(buf, b" , \0" as *const u8 as *const c_char);
            if ((*(*content).c2).type_0 as c_uint
                == XML_ELEMENT_CONTENT_OR as c_int as c_uint
                || (*(*content).c2).ocur as c_uint
                    != XML_ELEMENT_CONTENT_ONCE as c_int as c_uint)
                && (*(*content).c2).type_0 as c_uint
                    != XML_ELEMENT_CONTENT_ELEMENT as c_int as c_uint
            {
                xmlSnprintfElementContent(
                    buf,
                    size,
                    (*content).c2 as xmlElementContentPtr,
                    1 as c_int,
                );
            } else {
                xmlSnprintfElementContent(
                    buf,
                    size,
                    (*content).c2 as xmlElementContentPtr,
                    0 as c_int,
                );
            }
        }
        4 => {
            if (*(*content).c1).type_0 as c_uint
                == XML_ELEMENT_CONTENT_OR as c_int as c_uint
                || (*(*content).c1).type_0 as c_uint
                    == XML_ELEMENT_CONTENT_SEQ as c_int as c_uint
            {
                xmlSnprintfElementContent(
                    buf,
                    size,
                    (*content).c1 as xmlElementContentPtr,
                    1 as c_int,
                );
            } else {
                xmlSnprintfElementContent(
                    buf,
                    size,
                    (*content).c1 as xmlElementContentPtr,
                    0 as c_int,
                );
            }
            len = strlen(buf) as c_int;
            if size - len < 50 as c_int {
                if size - len > 4 as c_int
                    && *buf.offset((len - 1 as c_int) as isize) as c_int
                        != '.' as i32
                {
                    strcat(buf, b" ...\0" as *const u8 as *const c_char);
                }
                return;
            }
            strcat(buf, b" | \0" as *const u8 as *const c_char);
            if ((*(*content).c2).type_0 as c_uint
                == XML_ELEMENT_CONTENT_SEQ as c_int as c_uint
                || (*(*content).c2).ocur as c_uint
                    != XML_ELEMENT_CONTENT_ONCE as c_int as c_uint)
                && (*(*content).c2).type_0 as c_uint
                    != XML_ELEMENT_CONTENT_ELEMENT as c_int as c_uint
            {
                xmlSnprintfElementContent(
                    buf,
                    size,
                    (*content).c2 as xmlElementContentPtr,
                    1 as c_int,
                );
            } else {
                xmlSnprintfElementContent(
                    buf,
                    size,
                    (*content).c2 as xmlElementContentPtr,
                    0 as c_int,
                );
            }
        }
        _ => {}
    }
    if (size as size_t).wrapping_sub(strlen(buf)) <= 2 as size_t {
        return;
    }
    if englob != 0 {
        strcat(buf, b")\0" as *const u8 as *const c_char);
    }
    match (*content).ocur as c_uint {
        2 => {
            strcat(buf, b"?\0" as *const u8 as *const c_char);
        }
        3 => {
            strcat(buf, b"*\0" as *const u8 as *const c_char);
        }
        4 => {
            strcat(buf, b"+\0" as *const u8 as *const c_char);
        }
        1 | _ => {}
    };
}
unsafe extern "C" fn xmlFreeElement(mut elem: xmlElementPtr) {
    if elem.is_null() {
        return;
    }
    xmlUnlinkNode(elem as xmlNodePtr);
    xmlFreeDocElementContent((*elem).doc as xmlDocPtr, (*elem).content);
    if !(*elem).name.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*elem).name as *mut xmlChar as *mut c_void,
        );
    }
    if !(*elem).prefix.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*elem).prefix as *mut xmlChar as *mut c_void,
        );
    }
    if !(*elem).contModel.is_null() {
        xmlRegFreeRegexp((*elem).contModel);
    }
    xmlFree.expect("non-null function pointer")(elem as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlAddElementDecl(
    mut ctxt: xmlValidCtxtPtr,
    mut dtd: xmlDtdPtr,
    mut name: *const xmlChar,
    mut type_0: xmlElementTypeVal,
    mut content: xmlElementContentPtr,
) -> xmlElementPtr {
    let mut ret: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    let mut table: xmlElementTablePtr = ::core::ptr::null_mut::<xmlElementTable>();
    let mut oldAttributes: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut ns: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut uqname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if dtd.is_null() {
        return ::core::ptr::null_mut::<xmlElement>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlElement>();
    }
    match type_0 as c_uint {
        1 => {
            if !content.is_null() {
                xmlErrValid(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlAddElementDecl: content != NULL for EMPTY\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
                return ::core::ptr::null_mut::<xmlElement>();
            }
        }
        2 => {
            if !content.is_null() {
                xmlErrValid(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlAddElementDecl: content != NULL for ANY\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
                return ::core::ptr::null_mut::<xmlElement>();
            }
        }
        3 => {
            if content.is_null() {
                xmlErrValid(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlAddElementDecl: content == NULL for MIXED\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
                return ::core::ptr::null_mut::<xmlElement>();
            }
        }
        4 => {
            if content.is_null() {
                xmlErrValid(
                    ctxt,
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlAddElementDecl: content == NULL for ELEMENT\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
                return ::core::ptr::null_mut::<xmlElement>();
            }
        }
        _ => {
            xmlErrValid(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"Internal: ELEMENT decl corrupted invalid type\n\0" as *const u8
                    as *const c_char,
                ::core::ptr::null::<c_char>(),
            );
            return ::core::ptr::null_mut::<xmlElement>();
        }
    }
    uqname = xmlSplitQName2(name, &raw mut ns);
    if !uqname.is_null() {
        name = uqname;
    }
    table = (*dtd).elements as xmlElementTablePtr;
    if table.is_null() {
        let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
        if !(*dtd).doc.is_null() {
            dict = (*(*dtd).doc).dict as xmlDictPtr;
        }
        table = xmlHashCreateDict(0 as c_int, dict) as xmlElementTablePtr;
        (*dtd).elements = table as *mut c_void;
    }
    if table.is_null() {
        xmlVErrMemory(
            ctxt,
            b"xmlAddElementDecl: Table creation failed!\n\0" as *const u8
                as *const c_char,
        );
        if !uqname.is_null() {
            xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
        }
        if !ns.is_null() {
            xmlFree.expect("non-null function pointer")(ns as *mut c_void);
        }
        return ::core::ptr::null_mut::<xmlElement>();
    }
    if !(*dtd).doc.is_null() && !(*(*dtd).doc).intSubset.is_null() {
        ret = xmlHashLookup2(
            (*(*(*dtd).doc).intSubset).elements as xmlHashTablePtr,
            name,
            ns,
        ) as xmlElementPtr;
        if !ret.is_null()
            && (*ret).etype as c_uint
                == XML_ELEMENT_TYPE_UNDEFINED as c_int as c_uint
        {
            oldAttributes = (*ret).attributes;
            (*ret).attributes = ::core::ptr::null_mut::<xmlAttribute>();
            xmlHashRemoveEntry2(
                (*(*(*dtd).doc).intSubset).elements as xmlHashTablePtr,
                name,
                ns,
                None,
            );
            xmlFreeElement(ret);
        }
    }
    ret = xmlHashLookup2(table as xmlHashTablePtr, name, ns) as xmlElementPtr;
    if !ret.is_null() {
        if (*ret).etype as c_uint
            != XML_ELEMENT_TYPE_UNDEFINED as c_int as c_uint
        {
            xmlErrValidNode(
                ctxt,
                dtd as xmlNodePtr,
                XML_DTD_ELEM_REDEFINED,
                b"Redefinition of element %s\n\0" as *const u8 as *const c_char,
                name,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            if !uqname.is_null() {
                xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as *mut c_void);
            }
            return ::core::ptr::null_mut::<xmlElement>();
        }
        if !ns.is_null() {
            xmlFree.expect("non-null function pointer")(ns as *mut c_void);
            ns = ::core::ptr::null_mut::<xmlChar>();
        }
    } else {
        ret = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlElement>() as size_t
        ) as xmlElementPtr;
        if ret.is_null() {
            xmlVErrMemory(
                ctxt,
                b"malloc failed\0" as *const u8 as *const c_char,
            );
            if !uqname.is_null() {
                xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as *mut c_void);
            }
            return ::core::ptr::null_mut::<xmlElement>();
        }
        memset(
            ret as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<xmlElement>() as size_t,
        );
        (*ret).type_0 = XML_ELEMENT_DECL;
        (*ret).name = xmlStrdup(name);
        if (*ret).name.is_null() {
            xmlVErrMemory(
                ctxt,
                b"malloc failed\0" as *const u8 as *const c_char,
            );
            if !uqname.is_null() {
                xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as *mut c_void);
            }
            xmlFree.expect("non-null function pointer")(ret as *mut c_void);
            return ::core::ptr::null_mut::<xmlElement>();
        }
        (*ret).prefix = ns;
        if xmlHashAddEntry2(
            table as xmlHashTablePtr,
            name,
            ns,
            ret as *mut c_void,
        ) != 0
        {
            xmlErrValidNode(
                ctxt,
                dtd as xmlNodePtr,
                XML_DTD_ELEM_REDEFINED,
                b"Redefinition of element %s\n\0" as *const u8 as *const c_char,
                name,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            xmlFreeElement(ret);
            if !uqname.is_null() {
                xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
            }
            return ::core::ptr::null_mut::<xmlElement>();
        }
        (*ret).attributes = oldAttributes;
    }
    (*ret).etype = type_0;
    if !ctxt.is_null() && (*ctxt).flags & XML_VCTXT_USE_PCTXT != 0 {
        (*ret).content = content;
        if !content.is_null() {
            (*content).parent =
                1 as c_int as xmlElementContentPtr as *mut _xmlElementContent;
        }
    } else {
        (*ret).content = xmlCopyDocElementContent((*dtd).doc as xmlDocPtr, content);
    }
    (*ret).parent = dtd as *mut _xmlDtd;
    (*ret).doc = (*dtd).doc;
    if (*dtd).last.is_null() {
        (*dtd).last = ret as xmlNodePtr as *mut _xmlNode;
        (*dtd).children = (*dtd).last;
    } else {
        (*(*dtd).last).next = ret as xmlNodePtr as *mut _xmlNode;
        (*ret).prev = (*dtd).last;
        (*dtd).last = ret as xmlNodePtr as *mut _xmlNode;
    }
    if !uqname.is_null() {
        xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
    }
    return ret;
}
unsafe extern "C" fn xmlFreeElementTableEntry(
    mut elem: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlFreeElement(elem as xmlElementPtr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeElementTable(mut table: xmlElementTablePtr) {
    xmlHashFree(
        table as xmlHashTablePtr,
        Some(
            xmlFreeElementTableEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
}
unsafe extern "C" fn xmlCopyElement(
    mut payload: *mut c_void,
    mut name: *const xmlChar,
) -> *mut c_void {
    let mut elem: xmlElementPtr = payload as xmlElementPtr;
    let mut cur: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    cur = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlElement>() as size_t
    ) as xmlElementPtr;
    if cur.is_null() {
        xmlVErrMemory(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<c_void>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlElement>() as size_t,
    );
    (*cur).type_0 = XML_ELEMENT_DECL;
    (*cur).etype = (*elem).etype;
    if !(*elem).name.is_null() {
        (*cur).name = xmlStrdup((*elem).name);
    } else {
        (*cur).name = ::core::ptr::null::<xmlChar>();
    }
    if !(*elem).prefix.is_null() {
        (*cur).prefix = xmlStrdup((*elem).prefix);
    } else {
        (*cur).prefix = ::core::ptr::null::<xmlChar>();
    }
    (*cur).content = xmlCopyElementContent((*elem).content);
    (*cur).attributes = ::core::ptr::null_mut::<xmlAttribute>();
    return cur as *mut c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCopyElementTable(mut table: xmlElementTablePtr) -> xmlElementTablePtr {
    return xmlHashCopy(
        table as xmlHashTablePtr,
        Some(
            xmlCopyElement
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                ) -> *mut c_void,
        ),
    ) as xmlElementTablePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDumpElementDecl(mut buf: xmlBufferPtr, mut elem: xmlElementPtr) {
    if buf.is_null() || elem.is_null() {
        return;
    }
    match (*elem).etype as c_uint {
        1 => {
            xmlBufferWriteChar(
                buf,
                b"<!ELEMENT \0" as *const u8 as *const c_char,
            );
            if !(*elem).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*elem).prefix);
                xmlBufferWriteChar(buf, b":\0" as *const u8 as *const c_char);
            }
            xmlBufferWriteCHAR(buf, (*elem).name);
            xmlBufferWriteChar(
                buf,
                b" EMPTY>\n\0" as *const u8 as *const c_char,
            );
        }
        2 => {
            xmlBufferWriteChar(
                buf,
                b"<!ELEMENT \0" as *const u8 as *const c_char,
            );
            if !(*elem).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*elem).prefix);
                xmlBufferWriteChar(buf, b":\0" as *const u8 as *const c_char);
            }
            xmlBufferWriteCHAR(buf, (*elem).name);
            xmlBufferWriteChar(buf, b" ANY>\n\0" as *const u8 as *const c_char);
        }
        3 => {
            xmlBufferWriteChar(
                buf,
                b"<!ELEMENT \0" as *const u8 as *const c_char,
            );
            if !(*elem).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*elem).prefix);
                xmlBufferWriteChar(buf, b":\0" as *const u8 as *const c_char);
            }
            xmlBufferWriteCHAR(buf, (*elem).name);
            xmlBufferWriteChar(buf, b" \0" as *const u8 as *const c_char);
            xmlDumpElementContent(buf, (*elem).content);
            xmlBufferWriteChar(buf, b">\n\0" as *const u8 as *const c_char);
        }
        4 => {
            xmlBufferWriteChar(
                buf,
                b"<!ELEMENT \0" as *const u8 as *const c_char,
            );
            if !(*elem).prefix.is_null() {
                xmlBufferWriteCHAR(buf, (*elem).prefix);
                xmlBufferWriteChar(buf, b":\0" as *const u8 as *const c_char);
            }
            xmlBufferWriteCHAR(buf, (*elem).name);
            xmlBufferWriteChar(buf, b" \0" as *const u8 as *const c_char);
            xmlDumpElementContent(buf, (*elem).content);
            xmlBufferWriteChar(buf, b">\n\0" as *const u8 as *const c_char);
        }
        _ => {
            xmlErrValid(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                XML_ERR_INTERNAL_ERROR,
                b"Internal: ELEMENT struct corrupted invalid type\n\0" as *const u8
                    as *const c_char,
                ::core::ptr::null::<c_char>(),
            );
        }
    };
}
unsafe extern "C" fn xmlDumpElementDeclScan(
    mut elem: *mut c_void,
    mut buf: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlDumpElementDecl(buf as xmlBufferPtr, elem as xmlElementPtr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlDumpElementTable(mut buf: xmlBufferPtr, mut table: xmlElementTablePtr) {
    if buf.is_null() || table.is_null() {
        return;
    }
    xmlHashScan(
        table as xmlHashTablePtr,
        Some(
            xmlDumpElementDeclScan
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut c_void,
                    *const xmlChar,
                ) -> (),
        ),
        buf as *mut c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlCreateEnumeration(mut name: *const xmlChar) -> xmlEnumerationPtr {
    let mut ret: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlEnumeration>() as size_t
    ) as xmlEnumerationPtr;
    if ret.is_null() {
        xmlVErrMemory(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlEnumeration>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlEnumeration>() as size_t,
    );
    if !name.is_null() {
        (*ret).name = xmlStrdup(name);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeEnumeration(mut cur: xmlEnumerationPtr) {
    if cur.is_null() {
        return;
    }
    if !(*cur).next.is_null() {
        xmlFreeEnumeration((*cur).next as xmlEnumerationPtr);
    }
    if !(*cur).name.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*cur).name as *mut xmlChar as *mut c_void,
        );
    }
    xmlFree.expect("non-null function pointer")(cur as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCopyEnumeration(mut cur: xmlEnumerationPtr) -> xmlEnumerationPtr {
    let mut ret: xmlEnumerationPtr = ::core::ptr::null_mut::<xmlEnumeration>();
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlEnumeration>();
    }
    ret = xmlCreateEnumeration((*cur).name as *mut xmlChar);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlEnumeration>();
    }
    if !(*cur).next.is_null() {
        (*ret).next = xmlCopyEnumeration((*cur).next as xmlEnumerationPtr) as *mut _xmlEnumeration;
    } else {
        (*ret).next = ::core::ptr::null_mut::<_xmlEnumeration>();
    }
    return ret;
}
unsafe extern "C" fn xmlDumpEnumeration(mut buf: xmlBufferPtr, mut cur: xmlEnumerationPtr) {
    if buf.is_null() || cur.is_null() {
        return;
    }
    xmlBufferWriteCHAR(buf, (*cur).name);
    if (*cur).next.is_null() {
        xmlBufferWriteChar(buf, b")\0" as *const u8 as *const c_char);
    } else {
        xmlBufferWriteChar(buf, b" | \0" as *const u8 as *const c_char);
        xmlDumpEnumeration(buf, (*cur).next as xmlEnumerationPtr);
    };
}
unsafe extern "C" fn xmlScanIDAttributeDecl(
    mut ctxt: xmlValidCtxtPtr,
    mut elem: xmlElementPtr,
    mut err: c_int,
) -> c_int {
    let mut cur: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut ret: c_int = 0 as c_int;
    if elem.is_null() {
        return 0 as c_int;
    }
    cur = (*elem).attributes;
    while !cur.is_null() {
        if (*cur).atype as c_uint
            == XML_ATTRIBUTE_ID as c_int as c_uint
        {
            ret += 1;
            if ret > 1 as c_int && err != 0 {
                xmlErrValidNode(
                    ctxt,
                    elem as xmlNodePtr,
                    XML_DTD_MULTIPLE_ID,
                    b"Element %s has too many ID attributes defined : %s\n\0" as *const u8
                        as *const c_char,
                    (*elem).name,
                    (*cur).name,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
        }
        cur = (*cur).nexth as xmlAttributePtr;
    }
    return ret;
}
unsafe extern "C" fn xmlFreeAttribute(mut attr: xmlAttributePtr) {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if attr.is_null() {
        return;
    }
    if !(*attr).doc.is_null() {
        dict = (*(*attr).doc).dict as xmlDictPtr;
    } else {
        dict = ::core::ptr::null_mut::<xmlDict>();
    }
    xmlUnlinkNode(attr as xmlNodePtr);
    if !(*attr).tree.is_null() {
        xmlFreeEnumeration((*attr).tree);
    }
    if !dict.is_null() {
        if !(*attr).elem.is_null() && xmlDictOwns(dict, (*attr).elem) == 0 {
            xmlFree.expect("non-null function pointer")(
                (*attr).elem as *mut xmlChar as *mut c_void,
            );
        }
        if !(*attr).name.is_null() && xmlDictOwns(dict, (*attr).name) == 0 {
            xmlFree.expect("non-null function pointer")(
                (*attr).name as *mut xmlChar as *mut c_void,
            );
        }
        if !(*attr).prefix.is_null() && xmlDictOwns(dict, (*attr).prefix) == 0 {
            xmlFree.expect("non-null function pointer")(
                (*attr).prefix as *mut xmlChar as *mut c_void,
            );
        }
        if !(*attr).defaultValue.is_null() && xmlDictOwns(dict, (*attr).defaultValue) == 0 {
            xmlFree.expect("non-null function pointer")(
                (*attr).defaultValue as *mut xmlChar as *mut c_void,
            );
        }
    } else {
        if !(*attr).elem.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*attr).elem as *mut xmlChar as *mut c_void,
            );
        }
        if !(*attr).name.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*attr).name as *mut xmlChar as *mut c_void,
            );
        }
        if !(*attr).defaultValue.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*attr).defaultValue as *mut xmlChar as *mut c_void,
            );
        }
        if !(*attr).prefix.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*attr).prefix as *mut xmlChar as *mut c_void,
            );
        }
    }
    xmlFree.expect("non-null function pointer")(attr as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlAddAttributeDecl(
    mut ctxt: xmlValidCtxtPtr,
    mut dtd: xmlDtdPtr,
    mut elem: *const xmlChar,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
    mut type_0: xmlAttributeType,
    mut def: xmlAttributeDefault,
    mut defaultValue: *const xmlChar,
    mut tree: xmlEnumerationPtr,
) -> xmlAttributePtr {
    let mut ret: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut table: xmlAttributeTablePtr = ::core::ptr::null_mut::<xmlAttributeTable>();
    let mut elemDef: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if dtd.is_null() {
        xmlFreeEnumeration(tree);
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    if name.is_null() {
        xmlFreeEnumeration(tree);
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    if elem.is_null() {
        xmlFreeEnumeration(tree);
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    if !(*dtd).doc.is_null() {
        dict = (*(*dtd).doc).dict as xmlDictPtr;
    }
    match type_0 as c_uint {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => {}
        _ => {
            xmlErrValid(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"Internal: ATTRIBUTE struct corrupted invalid type\n\0" as *const u8
                    as *const c_char,
                ::core::ptr::null::<c_char>(),
            );
            xmlFreeEnumeration(tree);
            return ::core::ptr::null_mut::<xmlAttribute>();
        }
    }
    if !defaultValue.is_null()
        && xmlValidateAttributeValueInternal((*dtd).doc as xmlDocPtr, type_0, defaultValue) == 0
    {
        xmlErrValidNode(
            ctxt,
            dtd as xmlNodePtr,
            XML_DTD_ATTRIBUTE_DEFAULT,
            b"Attribute %s of %s: invalid default value\n\0" as *const u8
                as *const c_char,
            elem,
            name,
            defaultValue,
        );
        defaultValue = ::core::ptr::null::<xmlChar>();
        if !ctxt.is_null() {
            (*ctxt).valid = 0 as c_int;
        }
    }
    if !(*dtd).doc.is_null()
        && (*(*dtd).doc).extSubset == dtd
        && !(*(*dtd).doc).intSubset.is_null()
        && !(*(*(*dtd).doc).intSubset).attributes.is_null()
    {
        ret = xmlHashLookup3(
            (*(*(*dtd).doc).intSubset).attributes as xmlHashTablePtr,
            name,
            ns,
            elem,
        ) as xmlAttributePtr;
        if !ret.is_null() {
            xmlFreeEnumeration(tree);
            return ::core::ptr::null_mut::<xmlAttribute>();
        }
    }
    table = (*dtd).attributes as xmlAttributeTablePtr;
    if table.is_null() {
        table = xmlHashCreateDict(0 as c_int, dict) as xmlAttributeTablePtr;
        (*dtd).attributes = table as *mut c_void;
    }
    if table.is_null() {
        xmlVErrMemory(
            ctxt,
            b"xmlAddAttributeDecl: Table creation failed!\n\0" as *const u8
                as *const c_char,
        );
        xmlFreeEnumeration(tree);
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlAttribute>() as size_t
    ) as xmlAttributePtr;
    if ret.is_null() {
        xmlVErrMemory(
            ctxt,
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        xmlFreeEnumeration(tree);
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlAttribute>() as size_t,
    );
    (*ret).type_0 = XML_ATTRIBUTE_DECL;
    (*ret).atype = type_0;
    (*ret).doc = (*dtd).doc as *mut _xmlDoc;
    if !dict.is_null() {
        (*ret).name = xmlDictLookup(dict, name, -(1 as c_int));
        (*ret).prefix = xmlDictLookup(dict, ns, -(1 as c_int));
        (*ret).elem = xmlDictLookup(dict, elem, -(1 as c_int));
    } else {
        (*ret).name = xmlStrdup(name);
        (*ret).prefix = xmlStrdup(ns);
        (*ret).elem = xmlStrdup(elem);
    }
    (*ret).def = def;
    (*ret).tree = tree;
    if !defaultValue.is_null() {
        if !dict.is_null() {
            (*ret).defaultValue = xmlDictLookup(dict, defaultValue, -(1 as c_int));
        } else {
            (*ret).defaultValue = xmlStrdup(defaultValue);
        }
    }
    if xmlHashAddEntry3(
        table as xmlHashTablePtr,
        (*ret).name,
        (*ret).prefix,
        (*ret).elem,
        ret as *mut c_void,
    ) < 0 as c_int
    {
        xmlErrValidWarning(
            ctxt,
            dtd as xmlNodePtr,
            XML_DTD_ATTRIBUTE_REDEFINED,
            b"Attribute %s of element %s: already defined\n\0" as *const u8
                as *const c_char,
            name,
            elem,
            ::core::ptr::null::<xmlChar>(),
        );
        xmlFreeAttribute(ret);
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    elemDef = xmlGetDtdElementDesc2(ctxt, dtd, elem, 1 as c_int);
    if !elemDef.is_null() {
        if type_0 as c_uint
            == XML_ATTRIBUTE_ID as c_int as c_uint
            && xmlScanIDAttributeDecl(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                elemDef,
                1 as c_int,
            ) != 0 as c_int
        {
            xmlErrValidNode(
                ctxt,
                dtd as xmlNodePtr,
                XML_DTD_MULTIPLE_ID,
                b"Element %s has too may ID attributes defined : %s\n\0" as *const u8
                    as *const c_char,
                elem,
                name,
                ::core::ptr::null::<xmlChar>(),
            );
            if !ctxt.is_null() {
                (*ctxt).valid = 0 as c_int;
            }
        }
        if xmlStrEqual(
            (*ret).name,
            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
        ) != 0
            || !(*ret).prefix.is_null()
                && xmlStrEqual(
                    (*ret).prefix,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                ) != 0
        {
            (*ret).nexth = (*elemDef).attributes as *mut _xmlAttribute;
            (*elemDef).attributes = ret;
        } else {
            let mut tmp: xmlAttributePtr = (*elemDef).attributes;
            while !tmp.is_null()
                && (xmlStrEqual(
                    (*tmp).name,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                ) != 0
                    || !(*ret).prefix.is_null()
                        && xmlStrEqual(
                            (*ret).prefix,
                            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                        ) != 0)
            {
                if (*tmp).nexth.is_null() {
                    break;
                }
                tmp = (*tmp).nexth as xmlAttributePtr;
            }
            if !tmp.is_null() {
                (*ret).nexth = (*tmp).nexth;
                (*tmp).nexth = ret as *mut _xmlAttribute;
            } else {
                (*ret).nexth = (*elemDef).attributes as *mut _xmlAttribute;
                (*elemDef).attributes = ret;
            }
        }
    }
    (*ret).parent = dtd as *mut _xmlDtd;
    if (*dtd).last.is_null() {
        (*dtd).last = ret as xmlNodePtr as *mut _xmlNode;
        (*dtd).children = (*dtd).last;
    } else {
        (*(*dtd).last).next = ret as xmlNodePtr as *mut _xmlNode;
        (*ret).prev = (*dtd).last;
        (*dtd).last = ret as xmlNodePtr as *mut _xmlNode;
    }
    return ret;
}
unsafe extern "C" fn xmlFreeAttributeTableEntry(
    mut attr: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlFreeAttribute(attr as xmlAttributePtr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeAttributeTable(mut table: xmlAttributeTablePtr) {
    xmlHashFree(
        table as xmlHashTablePtr,
        Some(
            xmlFreeAttributeTableEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
}
unsafe extern "C" fn xmlCopyAttribute(
    mut payload: *mut c_void,
    mut name: *const xmlChar,
) -> *mut c_void {
    let mut attr: xmlAttributePtr = payload as xmlAttributePtr;
    let mut cur: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    cur = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlAttribute>() as size_t
    ) as xmlAttributePtr;
    if cur.is_null() {
        xmlVErrMemory(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<c_void>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlAttribute>() as size_t,
    );
    (*cur).type_0 = XML_ATTRIBUTE_DECL;
    (*cur).atype = (*attr).atype;
    (*cur).def = (*attr).def;
    (*cur).tree = xmlCopyEnumeration((*attr).tree);
    if !(*attr).elem.is_null() {
        (*cur).elem = xmlStrdup((*attr).elem);
    }
    if !(*attr).name.is_null() {
        (*cur).name = xmlStrdup((*attr).name);
    }
    if !(*attr).prefix.is_null() {
        (*cur).prefix = xmlStrdup((*attr).prefix);
    }
    if !(*attr).defaultValue.is_null() {
        (*cur).defaultValue = xmlStrdup((*attr).defaultValue);
    }
    return cur as *mut c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCopyAttributeTable(
    mut table: xmlAttributeTablePtr,
) -> xmlAttributeTablePtr {
    return xmlHashCopy(
        table as xmlHashTablePtr,
        Some(
            xmlCopyAttribute
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                ) -> *mut c_void,
        ),
    ) as xmlAttributeTablePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDumpAttributeDecl(mut buf: xmlBufferPtr, mut attr: xmlAttributePtr) {
    if buf.is_null() || attr.is_null() {
        return;
    }
    xmlBufferWriteChar(
        buf,
        b"<!ATTLIST \0" as *const u8 as *const c_char,
    );
    xmlBufferWriteCHAR(buf, (*attr).elem);
    xmlBufferWriteChar(buf, b" \0" as *const u8 as *const c_char);
    if !(*attr).prefix.is_null() {
        xmlBufferWriteCHAR(buf, (*attr).prefix);
        xmlBufferWriteChar(buf, b":\0" as *const u8 as *const c_char);
    }
    xmlBufferWriteCHAR(buf, (*attr).name);
    match (*attr).atype as c_uint {
        1 => {
            xmlBufferWriteChar(buf, b" CDATA\0" as *const u8 as *const c_char);
        }
        2 => {
            xmlBufferWriteChar(buf, b" ID\0" as *const u8 as *const c_char);
        }
        3 => {
            xmlBufferWriteChar(buf, b" IDREF\0" as *const u8 as *const c_char);
        }
        4 => {
            xmlBufferWriteChar(buf, b" IDREFS\0" as *const u8 as *const c_char);
        }
        5 => {
            xmlBufferWriteChar(buf, b" ENTITY\0" as *const u8 as *const c_char);
        }
        6 => {
            xmlBufferWriteChar(
                buf,
                b" ENTITIES\0" as *const u8 as *const c_char,
            );
        }
        7 => {
            xmlBufferWriteChar(
                buf,
                b" NMTOKEN\0" as *const u8 as *const c_char,
            );
        }
        8 => {
            xmlBufferWriteChar(
                buf,
                b" NMTOKENS\0" as *const u8 as *const c_char,
            );
        }
        9 => {
            xmlBufferWriteChar(buf, b" (\0" as *const u8 as *const c_char);
            xmlDumpEnumeration(buf, (*attr).tree);
        }
        10 => {
            xmlBufferWriteChar(
                buf,
                b" NOTATION (\0" as *const u8 as *const c_char,
            );
            xmlDumpEnumeration(buf, (*attr).tree);
        }
        _ => {
            xmlErrValid(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                XML_ERR_INTERNAL_ERROR,
                b"Internal: ATTRIBUTE struct corrupted invalid type\n\0" as *const u8
                    as *const c_char,
                ::core::ptr::null::<c_char>(),
            );
        }
    }
    match (*attr).def as c_uint {
        1 => {}
        2 => {
            xmlBufferWriteChar(
                buf,
                b" #REQUIRED\0" as *const u8 as *const c_char,
            );
        }
        3 => {
            xmlBufferWriteChar(
                buf,
                b" #IMPLIED\0" as *const u8 as *const c_char,
            );
        }
        4 => {
            xmlBufferWriteChar(buf, b" #FIXED\0" as *const u8 as *const c_char);
        }
        _ => {
            xmlErrValid(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                XML_ERR_INTERNAL_ERROR,
                b"Internal: ATTRIBUTE struct corrupted invalid def\n\0" as *const u8
                    as *const c_char,
                ::core::ptr::null::<c_char>(),
            );
        }
    }
    if !(*attr).defaultValue.is_null() {
        xmlBufferWriteChar(buf, b" \0" as *const u8 as *const c_char);
        xmlBufferWriteQuotedString(buf, (*attr).defaultValue);
    }
    xmlBufferWriteChar(buf, b">\n\0" as *const u8 as *const c_char);
}
unsafe extern "C" fn xmlDumpAttributeDeclScan(
    mut attr: *mut c_void,
    mut buf: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlDumpAttributeDecl(buf as xmlBufferPtr, attr as xmlAttributePtr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlDumpAttributeTable(
    mut buf: xmlBufferPtr,
    mut table: xmlAttributeTablePtr,
) {
    if buf.is_null() || table.is_null() {
        return;
    }
    xmlHashScan(
        table as xmlHashTablePtr,
        Some(
            xmlDumpAttributeDeclScan
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut c_void,
                    *const xmlChar,
                ) -> (),
        ),
        buf as *mut c_void,
    );
}
unsafe extern "C" fn xmlFreeNotation(mut nota: xmlNotationPtr) {
    if nota.is_null() {
        return;
    }
    if !(*nota).name.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*nota).name as *mut xmlChar as *mut c_void,
        );
    }
    if !(*nota).PublicID.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*nota).PublicID as *mut xmlChar as *mut c_void,
        );
    }
    if !(*nota).SystemID.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*nota).SystemID as *mut xmlChar as *mut c_void,
        );
    }
    xmlFree.expect("non-null function pointer")(nota as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlAddNotationDecl(
    mut ctxt: xmlValidCtxtPtr,
    mut dtd: xmlDtdPtr,
    mut name: *const xmlChar,
    mut PublicID: *const xmlChar,
    mut SystemID: *const xmlChar,
) -> xmlNotationPtr {
    let mut ret: xmlNotationPtr = ::core::ptr::null_mut::<xmlNotation>();
    let mut table: xmlNotationTablePtr = ::core::ptr::null_mut::<xmlNotationTable>();
    if dtd.is_null() {
        return ::core::ptr::null_mut::<xmlNotation>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlNotation>();
    }
    if PublicID.is_null() && SystemID.is_null() {
        return ::core::ptr::null_mut::<xmlNotation>();
    }
    table = (*dtd).notations as xmlNotationTablePtr;
    if table.is_null() {
        let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
        if !(*dtd).doc.is_null() {
            dict = (*(*dtd).doc).dict as xmlDictPtr;
        }
        table = xmlHashCreateDict(0 as c_int, dict) as xmlNotationTablePtr;
        (*dtd).notations = table as *mut c_void;
    }
    if table.is_null() {
        xmlVErrMemory(
            ctxt,
            b"xmlAddNotationDecl: Table creation failed!\n\0" as *const u8
                as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlNotation>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlNotation>() as size_t
    ) as xmlNotationPtr;
    if ret.is_null() {
        xmlVErrMemory(
            ctxt,
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlNotation>();
    }
    memset(
        ret as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlNotation>() as size_t,
    );
    (*ret).name = xmlStrdup(name);
    if !SystemID.is_null() {
        (*ret).SystemID = xmlStrdup(SystemID);
    }
    if !PublicID.is_null() {
        (*ret).PublicID = xmlStrdup(PublicID);
    }
    if xmlHashAddEntry(
        table as xmlHashTablePtr,
        name,
        ret as *mut c_void,
    ) != 0
    {
        xmlErrValid(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            XML_DTD_NOTATION_REDEFINED,
            b"xmlAddNotationDecl: %s already defined\n\0" as *const u8
                as *const c_char,
            name as *const c_char,
        );
        xmlFreeNotation(ret);
        return ::core::ptr::null_mut::<xmlNotation>();
    }
    return ret;
}
unsafe extern "C" fn xmlFreeNotationTableEntry(
    mut nota: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlFreeNotation(nota as xmlNotationPtr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeNotationTable(mut table: xmlNotationTablePtr) {
    xmlHashFree(
        table as xmlHashTablePtr,
        Some(
            xmlFreeNotationTableEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
}
unsafe extern "C" fn xmlCopyNotation(
    mut payload: *mut c_void,
    mut name: *const xmlChar,
) -> *mut c_void {
    let mut nota: xmlNotationPtr = payload as xmlNotationPtr;
    let mut cur: xmlNotationPtr = ::core::ptr::null_mut::<xmlNotation>();
    cur = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlNotation>() as size_t
    ) as xmlNotationPtr;
    if cur.is_null() {
        xmlVErrMemory(
            ::core::ptr::null_mut::<xmlValidCtxt>(),
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<c_void>();
    }
    if !(*nota).name.is_null() {
        (*cur).name = xmlStrdup((*nota).name);
    } else {
        (*cur).name = ::core::ptr::null::<xmlChar>();
    }
    if !(*nota).PublicID.is_null() {
        (*cur).PublicID = xmlStrdup((*nota).PublicID);
    } else {
        (*cur).PublicID = ::core::ptr::null::<xmlChar>();
    }
    if !(*nota).SystemID.is_null() {
        (*cur).SystemID = xmlStrdup((*nota).SystemID);
    } else {
        (*cur).SystemID = ::core::ptr::null::<xmlChar>();
    }
    return cur as *mut c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlCopyNotationTable(
    mut table: xmlNotationTablePtr,
) -> xmlNotationTablePtr {
    return xmlHashCopy(
        table as xmlHashTablePtr,
        Some(
            xmlCopyNotation
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                ) -> *mut c_void,
        ),
    ) as xmlNotationTablePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDumpNotationDecl(mut buf: xmlBufferPtr, mut nota: xmlNotationPtr) {
    if buf.is_null() || nota.is_null() {
        return;
    }
    xmlBufferWriteChar(
        buf,
        b"<!NOTATION \0" as *const u8 as *const c_char,
    );
    xmlBufferWriteCHAR(buf, (*nota).name);
    if !(*nota).PublicID.is_null() {
        xmlBufferWriteChar(
            buf,
            b" PUBLIC \0" as *const u8 as *const c_char,
        );
        xmlBufferWriteQuotedString(buf, (*nota).PublicID);
        if !(*nota).SystemID.is_null() {
            xmlBufferWriteChar(buf, b" \0" as *const u8 as *const c_char);
            xmlBufferWriteQuotedString(buf, (*nota).SystemID);
        }
    } else {
        xmlBufferWriteChar(
            buf,
            b" SYSTEM \0" as *const u8 as *const c_char,
        );
        xmlBufferWriteQuotedString(buf, (*nota).SystemID);
    }
    xmlBufferWriteChar(buf, b" >\n\0" as *const u8 as *const c_char);
}
unsafe extern "C" fn xmlDumpNotationDeclScan(
    mut nota: *mut c_void,
    mut buf: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlDumpNotationDecl(buf as xmlBufferPtr, nota as xmlNotationPtr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlDumpNotationTable(
    mut buf: xmlBufferPtr,
    mut table: xmlNotationTablePtr,
) {
    if buf.is_null() || table.is_null() {
        return;
    }
    xmlHashScan(
        table as xmlHashTablePtr,
        Some(
            xmlDumpNotationDeclScan
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut c_void,
                    *const xmlChar,
                ) -> (),
        ),
        buf as *mut c_void,
    );
}
unsafe extern "C" fn xmlValidNormalizeString(mut str: *mut xmlChar) {
    let mut dst: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut src: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if str.is_null() {
        return;
    }
    src = str;
    dst = str;
    while *src as c_int == 0x20 as c_int {
        src = src.offset(1);
    }
    while *src as c_int != 0 as c_int {
        if *src as c_int == 0x20 as c_int {
            while *src as c_int == 0x20 as c_int {
                src = src.offset(1);
            }
            if *src as c_int != 0 as c_int {
                let fresh0 = dst;
                dst = dst.offset(1);
                *fresh0 = 0x20 as xmlChar;
            }
        } else {
            let fresh1 = src;
            src = src.offset(1);
            let fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = *fresh1;
        }
    }
    *dst = 0 as xmlChar;
}
unsafe extern "C" fn xmlIsStreaming(mut ctxt: xmlValidCtxtPtr) -> c_int {
    let mut pctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    if ctxt.is_null() {
        return 0 as c_int;
    }
    if (*ctxt).flags & XML_VCTXT_USE_PCTXT == 0 as c_uint {
        return 0 as c_int;
    }
    pctxt = (*ctxt).userData as xmlParserCtxtPtr;
    return ((*pctxt).parseMode as c_uint
        == XML_PARSE_READER as c_int as c_uint)
        as c_int;
}
unsafe extern "C" fn xmlFreeID(mut id: xmlIDPtr) {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if id.is_null() {
        return;
    }
    if !(*id).doc.is_null() {
        dict = (*(*id).doc).dict as xmlDictPtr;
    }
    if !(*id).value.is_null() {
        if !(*id).value.is_null()
            && (dict.is_null() || xmlDictOwns(dict, (*id).value) == 0 as c_int)
        {
            xmlFree.expect("non-null function pointer")(
                (*id).value as *mut c_char as *mut c_void,
            );
        }
    }
    if !(*id).name.is_null() {
        if !(*id).name.is_null()
            && (dict.is_null() || xmlDictOwns(dict, (*id).name) == 0 as c_int)
        {
            xmlFree.expect("non-null function pointer")(
                (*id).name as *mut c_char as *mut c_void,
            );
        }
    }
    xmlFree.expect("non-null function pointer")(id as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlAddID(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut value: *const xmlChar,
    mut attr: xmlAttrPtr,
) -> xmlIDPtr {
    let mut ret: xmlIDPtr = ::core::ptr::null_mut::<xmlID>();
    let mut table: xmlIDTablePtr = ::core::ptr::null_mut::<xmlIDTable>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlID>();
    }
    if value.is_null()
        || *value.offset(0 as c_int as isize) as c_int
            == 0 as c_int
    {
        return ::core::ptr::null_mut::<xmlID>();
    }
    if attr.is_null() {
        return ::core::ptr::null_mut::<xmlID>();
    }
    table = (*doc).ids as xmlIDTablePtr;
    if table.is_null() {
        table =
            xmlHashCreateDict(0 as c_int, (*doc).dict as xmlDictPtr) as xmlIDTablePtr;
        (*doc).ids = table as *mut c_void;
    }
    if table.is_null() {
        xmlVErrMemory(
            ctxt,
            b"xmlAddID: Table creation failed!\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlID>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlID>() as size_t)
        as xmlIDPtr;
    if ret.is_null() {
        xmlVErrMemory(
            ctxt,
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlID>();
    }
    (*ret).value = xmlStrdup(value);
    (*ret).doc = doc as *mut _xmlDoc;
    if xmlIsStreaming(ctxt) != 0 {
        if !(*doc).dict.is_null() {
            (*ret).name = xmlDictLookup(
                (*doc).dict as xmlDictPtr,
                (*attr).name,
                -(1 as c_int),
            );
        } else {
            (*ret).name = xmlStrdup((*attr).name);
        }
        (*ret).attr = ::core::ptr::null_mut::<xmlAttr>();
    } else {
        (*ret).attr = attr;
        (*ret).name = ::core::ptr::null::<xmlChar>();
    }
    (*ret).lineno = xmlGetLineNo((*attr).parent) as c_int;
    if xmlHashAddEntry(
        table as xmlHashTablePtr,
        value,
        ret as *mut c_void,
    ) < 0 as c_int
    {
        if !ctxt.is_null() {
            xmlErrValidNode(
                ctxt,
                (*attr).parent as xmlNodePtr,
                XML_DTD_ID_REDEFINED,
                b"ID %s already defined\n\0" as *const u8 as *const c_char,
                value,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
        }
        xmlFreeID(ret);
        return ::core::ptr::null_mut::<xmlID>();
    }
    if !attr.is_null() {
        (*attr).atype = XML_ATTRIBUTE_ID;
    }
    return ret;
}
unsafe extern "C" fn xmlFreeIDTableEntry(
    mut id: *mut c_void,
    mut name: *const xmlChar,
) {
    xmlFreeID(id as xmlIDPtr);
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeIDTable(mut table: xmlIDTablePtr) {
    xmlHashFree(
        table as xmlHashTablePtr,
        Some(
            xmlFreeIDTableEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsID(
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut attr: xmlAttrPtr,
) -> c_int {
    if attr.is_null() || (*attr).name.is_null() {
        return 0 as c_int;
    }
    if !(*attr).ns.is_null()
        && !(*(*attr).ns).prefix.is_null()
        && strcmp(
            (*attr).name as *mut c_char,
            b"id\0" as *const u8 as *const c_char,
        ) == 0
        && strcmp(
            (*(*attr).ns).prefix as *mut c_char,
            b"xml\0" as *const u8 as *const c_char,
        ) == 0
    {
        return 1 as c_int;
    }
    if doc.is_null() {
        return 0 as c_int;
    }
    if (*doc).intSubset.is_null()
        && (*doc).extSubset.is_null()
        && (*doc).type_0 as c_uint
            != XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    } else if (*doc).type_0 as c_uint
        == XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        if xmlStrEqual(
            b"id\0" as *const u8 as *const c_char as *mut xmlChar,
            (*attr).name,
        ) != 0
            || xmlStrEqual(
                b"name\0" as *const u8 as *const c_char as *mut xmlChar,
                (*attr).name,
            ) != 0
                && (elem.is_null()
                    || xmlStrEqual(
                        (*elem).name,
                        b"a\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) != 0)
        {
            return 1 as c_int;
        }
        return 0 as c_int;
    } else if elem.is_null() {
        return 0 as c_int;
    } else {
        let mut attrDecl: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
        let mut felem: [xmlChar; 50] = [0; 50];
        let mut fattr: [xmlChar; 50] = [0; 50];
        let mut fullelemname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut fullattrname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        fullelemname = if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
            xmlBuildQName(
                (*elem).name,
                (*(*elem).ns).prefix,
                &raw mut felem as *mut xmlChar,
                50 as c_int,
            )
        } else {
            (*elem).name as *mut xmlChar
        };
        fullattrname = if !(*attr).ns.is_null() && !(*(*attr).ns).prefix.is_null() {
            xmlBuildQName(
                (*attr).name,
                (*(*attr).ns).prefix,
                &raw mut fattr as *mut xmlChar,
                50 as c_int,
            )
        } else {
            (*attr).name as *mut xmlChar
        };
        if !fullelemname.is_null() && !fullattrname.is_null() {
            attrDecl = xmlGetDtdAttrDesc((*doc).intSubset as xmlDtdPtr, fullelemname, fullattrname);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdAttrDesc((*doc).extSubset as xmlDtdPtr, fullelemname, fullattrname);
            }
        }
        if fullattrname != &raw mut fattr as *mut xmlChar
            && fullattrname != (*attr).name as *mut xmlChar
        {
            xmlFree.expect("non-null function pointer")(fullattrname as *mut c_void);
        }
        if fullelemname != &raw mut felem as *mut xmlChar
            && fullelemname != (*elem).name as *mut xmlChar
        {
            xmlFree.expect("non-null function pointer")(fullelemname as *mut c_void);
        }
        if !attrDecl.is_null()
            && (*attrDecl).atype as c_uint
                == XML_ATTRIBUTE_ID as c_int as c_uint
        {
            return 1 as c_int;
        }
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRemoveID(
    mut doc: xmlDocPtr,
    mut attr: xmlAttrPtr,
) -> c_int {
    let mut table: xmlIDTablePtr = ::core::ptr::null_mut::<xmlIDTable>();
    let mut id: xmlIDPtr = ::core::ptr::null_mut::<xmlID>();
    let mut ID: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if doc.is_null() {
        return -(1 as c_int);
    }
    if attr.is_null() {
        return -(1 as c_int);
    }
    table = (*doc).ids as xmlIDTablePtr;
    if table.is_null() {
        return -(1 as c_int);
    }
    ID = xmlNodeListGetString(doc, (*attr).children, 1 as c_int);
    if ID.is_null() {
        return -(1 as c_int);
    }
    xmlValidNormalizeString(ID);
    id = xmlHashLookup(table as xmlHashTablePtr, ID) as xmlIDPtr;
    if id.is_null() || (*id).attr != attr {
        xmlFree.expect("non-null function pointer")(ID as *mut c_void);
        return -(1 as c_int);
    }
    xmlHashRemoveEntry(
        table as xmlHashTablePtr,
        ID,
        Some(
            xmlFreeIDTableEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
    xmlFree.expect("non-null function pointer")(ID as *mut c_void);
    (*attr).atype = 0 as xmlAttributeType;
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetID(mut doc: xmlDocPtr, mut ID: *const xmlChar) -> xmlAttrPtr {
    let mut table: xmlIDTablePtr = ::core::ptr::null_mut::<xmlIDTable>();
    let mut id: xmlIDPtr = ::core::ptr::null_mut::<xmlID>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    if ID.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    table = (*doc).ids as xmlIDTablePtr;
    if table.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    id = xmlHashLookup(table as xmlHashTablePtr, ID) as xmlIDPtr;
    if id.is_null() {
        return ::core::ptr::null_mut::<xmlAttr>();
    }
    if (*id).attr.is_null() {
        return doc as xmlAttrPtr;
    }
    return (*id).attr;
}
unsafe extern "C" fn xmlFreeRef(mut lk: xmlLinkPtr) {
    let mut ref_0: xmlRefPtr = xmlLinkGetData(lk) as xmlRefPtr;
    if ref_0.is_null() {
        return;
    }
    if !(*ref_0).value.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ref_0).value as *mut xmlChar as *mut c_void,
        );
    }
    if !(*ref_0).name.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*ref_0).name as *mut xmlChar as *mut c_void,
        );
    }
    xmlFree.expect("non-null function pointer")(ref_0 as *mut c_void);
}
unsafe extern "C" fn xmlFreeRefTableEntry(
    mut payload: *mut c_void,
    mut name: *const xmlChar,
) {
    let mut list_ref: xmlListPtr = payload as xmlListPtr;
    if list_ref.is_null() {
        return;
    }
    xmlListDelete(list_ref);
}
unsafe extern "C" fn xmlWalkRemoveRef(
    mut data: *const c_void,
    mut user: *mut c_void,
) -> c_int {
    let mut attr0: xmlAttrPtr = (*(data as xmlRefPtr)).attr;
    let mut attr1: xmlAttrPtr = (*(user as xmlRemoveMemoPtr)).ap;
    let mut ref_list: xmlListPtr = (*(user as xmlRemoveMemoPtr)).l;
    if attr0 == attr1 {
        xmlListRemoveFirst(ref_list, data as *mut c_void);
        return 0 as c_int;
    }
    return 1 as c_int;
}
unsafe extern "C" fn xmlDummyCompare(
    mut data0: *const c_void,
    mut data1: *const c_void,
) -> c_int {
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAddRef(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut value: *const xmlChar,
    mut attr: xmlAttrPtr,
) -> xmlRefPtr {
    let mut current_block: u64;
    let mut ret: xmlRefPtr = ::core::ptr::null_mut::<xmlRef>();
    let mut table: xmlRefTablePtr = ::core::ptr::null_mut::<xmlRefTable>();
    let mut ref_list: xmlListPtr = ::core::ptr::null_mut::<xmlList>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlRef>();
    }
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlRef>();
    }
    if attr.is_null() {
        return ::core::ptr::null_mut::<xmlRef>();
    }
    table = (*doc).refs as xmlRefTablePtr;
    if table.is_null() {
        table =
            xmlHashCreateDict(0 as c_int, (*doc).dict as xmlDictPtr) as xmlRefTablePtr;
        (*doc).refs = table as *mut c_void;
    }
    if table.is_null() {
        xmlVErrMemory(
            ctxt,
            b"xmlAddRef: Table creation failed!\n\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlRef>();
    }
    ret = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlRef>() as size_t)
        as xmlRefPtr;
    if ret.is_null() {
        xmlVErrMemory(
            ctxt,
            b"malloc failed\0" as *const u8 as *const c_char,
        );
        return ::core::ptr::null_mut::<xmlRef>();
    }
    (*ret).value = xmlStrdup(value);
    if xmlIsStreaming(ctxt) != 0 {
        (*ret).name = xmlStrdup((*attr).name);
        (*ret).attr = ::core::ptr::null_mut::<xmlAttr>();
    } else {
        (*ret).name = ::core::ptr::null::<xmlChar>();
        (*ret).attr = attr;
    }
    (*ret).lineno = xmlGetLineNo((*attr).parent) as c_int;
    ref_list = xmlHashLookup(table as xmlHashTablePtr, value) as xmlListPtr;
    if ref_list.is_null() {
        ref_list = xmlListCreate(
            Some(xmlFreeRef as unsafe extern "C" fn(xmlLinkPtr) -> ()),
            Some(
                xmlDummyCompare
                    as unsafe extern "C" fn(
                        *const c_void,
                        *const c_void,
                    ) -> c_int,
            ),
        );
        if ref_list.is_null() {
            xmlErrValid(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                XML_ERR_INTERNAL_ERROR,
                b"xmlAddRef: Reference list creation failed!\n\0" as *const u8
                    as *const c_char,
                ::core::ptr::null::<c_char>(),
            );
            current_block = 7356627998027237037;
        } else if xmlHashAddEntry(
            table as xmlHashTablePtr,
            value,
            ref_list as *mut c_void,
        ) < 0 as c_int
        {
            xmlListDelete(ref_list);
            xmlErrValid(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                XML_ERR_INTERNAL_ERROR,
                b"xmlAddRef: Reference list insertion failed!\n\0" as *const u8
                    as *const c_char,
                ::core::ptr::null::<c_char>(),
            );
            current_block = 7356627998027237037;
        } else {
            current_block = 14648156034262866959;
        }
    } else {
        current_block = 14648156034262866959;
    }
    match current_block {
        14648156034262866959 => {
            if xmlListAppend(ref_list, ret as *mut c_void) != 0 as c_int {
                xmlErrValid(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    XML_ERR_INTERNAL_ERROR,
                    b"xmlAddRef: Reference list insertion failed!\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
            } else {
                return ret;
            }
        }
        _ => {}
    }
    if !ret.is_null() {
        if !(*ret).value.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ret).value as *mut c_char as *mut c_void,
            );
        }
        if !(*ret).name.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ret).name as *mut c_char as *mut c_void,
            );
        }
        xmlFree.expect("non-null function pointer")(ret as *mut c_void);
    }
    return ::core::ptr::null_mut::<xmlRef>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeRefTable(mut table: xmlRefTablePtr) {
    xmlHashFree(
        table as xmlHashTablePtr,
        Some(
            xmlFreeRefTableEntry
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsRef(
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut attr: xmlAttrPtr,
) -> c_int {
    if attr.is_null() {
        return 0 as c_int;
    }
    if doc.is_null() {
        doc = (*attr).doc as xmlDocPtr;
        if doc.is_null() {
            return 0 as c_int;
        }
    }
    if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as c_int;
    } else if (*doc).type_0 as c_uint
        == XML_HTML_DOCUMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    } else {
        let mut attrDecl: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
        if elem.is_null() {
            return 0 as c_int;
        }
        attrDecl = xmlGetDtdAttrDesc((*doc).intSubset as xmlDtdPtr, (*elem).name, (*attr).name);
        if attrDecl.is_null() && !(*doc).extSubset.is_null() {
            attrDecl = xmlGetDtdAttrDesc((*doc).extSubset as xmlDtdPtr, (*elem).name, (*attr).name);
        }
        if !attrDecl.is_null()
            && ((*attrDecl).atype as c_uint
                == XML_ATTRIBUTE_IDREF as c_int as c_uint
                || (*attrDecl).atype as c_uint
                    == XML_ATTRIBUTE_IDREFS as c_int as c_uint)
        {
            return 1 as c_int;
        }
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRemoveRef(
    mut doc: xmlDocPtr,
    mut attr: xmlAttrPtr,
) -> c_int {
    let mut ref_list: xmlListPtr = ::core::ptr::null_mut::<xmlList>();
    let mut table: xmlRefTablePtr = ::core::ptr::null_mut::<xmlRefTable>();
    let mut ID: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut target: xmlRemoveMemo = xmlRemoveMemo {
        l: ::core::ptr::null_mut::<xmlList>(),
        ap: ::core::ptr::null_mut::<xmlAttr>(),
    };
    if doc.is_null() {
        return -(1 as c_int);
    }
    if attr.is_null() {
        return -(1 as c_int);
    }
    table = (*doc).refs as xmlRefTablePtr;
    if table.is_null() {
        return -(1 as c_int);
    }
    ID = xmlNodeListGetString(doc, (*attr).children, 1 as c_int);
    if ID.is_null() {
        return -(1 as c_int);
    }
    ref_list = xmlHashLookup(table as xmlHashTablePtr, ID) as xmlListPtr;
    if ref_list.is_null() {
        xmlFree.expect("non-null function pointer")(ID as *mut c_void);
        return -(1 as c_int);
    }
    target.l = ref_list;
    target.ap = attr;
    xmlListWalk(
        ref_list,
        Some(
            xmlWalkRemoveRef
                as unsafe extern "C" fn(
                    *const c_void,
                    *mut c_void,
                ) -> c_int,
        ),
        &raw mut target as *mut c_void,
    );
    if xmlListEmpty(ref_list) != 0 {
        xmlHashUpdateEntry(
            table as xmlHashTablePtr,
            ID,
            NULL,
            Some(
                xmlFreeRefTableEntry
                    as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
            ),
        );
    }
    xmlFree.expect("non-null function pointer")(ID as *mut c_void);
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetRefs(mut doc: xmlDocPtr, mut ID: *const xmlChar) -> xmlListPtr {
    let mut table: xmlRefTablePtr = ::core::ptr::null_mut::<xmlRefTable>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlList>();
    }
    if ID.is_null() {
        return ::core::ptr::null_mut::<xmlList>();
    }
    table = (*doc).refs as xmlRefTablePtr;
    if table.is_null() {
        return ::core::ptr::null_mut::<xmlList>();
    }
    return xmlHashLookup(table as xmlHashTablePtr, ID) as xmlListPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdElementDesc(
    mut dtd: xmlDtdPtr,
    mut name: *const xmlChar,
) -> xmlElementPtr {
    let mut table: xmlElementTablePtr = ::core::ptr::null_mut::<xmlElementTable>();
    let mut cur: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    let mut uqname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if dtd.is_null() || name.is_null() {
        return ::core::ptr::null_mut::<xmlElement>();
    }
    if (*dtd).elements.is_null() {
        return ::core::ptr::null_mut::<xmlElement>();
    }
    table = (*dtd).elements as xmlElementTablePtr;
    uqname = xmlSplitQName2(name, &raw mut prefix);
    if !uqname.is_null() {
        name = uqname;
    }
    cur = xmlHashLookup2(table as xmlHashTablePtr, name, prefix) as xmlElementPtr;
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
    }
    if !uqname.is_null() {
        xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
    }
    return cur;
}
unsafe extern "C" fn xmlGetDtdElementDesc2(
    mut ctxt: xmlValidCtxtPtr,
    mut dtd: xmlDtdPtr,
    mut name: *const xmlChar,
    mut create: c_int,
) -> xmlElementPtr {
    let mut table: xmlElementTablePtr = ::core::ptr::null_mut::<xmlElementTable>();
    let mut cur: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    let mut uqname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if dtd.is_null() {
        return ::core::ptr::null_mut::<xmlElement>();
    }
    if (*dtd).elements.is_null() {
        let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
        if !(*dtd).doc.is_null() {
            dict = (*(*dtd).doc).dict as xmlDictPtr;
        }
        if create == 0 {
            return ::core::ptr::null_mut::<xmlElement>();
        }
        table = (*dtd).elements as xmlElementTablePtr;
        if table.is_null() {
            table = xmlHashCreateDict(0 as c_int, dict) as xmlElementTablePtr;
            (*dtd).elements = table as *mut c_void;
        }
        if table.is_null() {
            xmlVErrMemory(
                ctxt,
                b"element table allocation failed\0" as *const u8 as *const c_char,
            );
            return ::core::ptr::null_mut::<xmlElement>();
        }
    }
    table = (*dtd).elements as xmlElementTablePtr;
    uqname = xmlSplitQName2(name, &raw mut prefix);
    if !uqname.is_null() {
        name = uqname;
    }
    cur = xmlHashLookup2(table as xmlHashTablePtr, name, prefix) as xmlElementPtr;
    if cur.is_null() && create != 0 {
        cur = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlElement>() as size_t
        ) as xmlElementPtr;
        if cur.is_null() {
            xmlVErrMemory(
                ctxt,
                b"malloc failed\0" as *const u8 as *const c_char,
            );
        } else {
            memset(
                cur as *mut c_void,
                0 as c_int,
                ::core::mem::size_of::<xmlElement>() as size_t,
            );
            (*cur).type_0 = XML_ELEMENT_DECL;
            (*cur).name = xmlStrdup(name);
            (*cur).prefix = xmlStrdup(prefix);
            (*cur).etype = XML_ELEMENT_TYPE_UNDEFINED;
            if xmlHashAddEntry2(
                table as xmlHashTablePtr,
                name,
                prefix,
                cur as *mut c_void,
            ) < 0 as c_int
            {
                xmlVErrMemory(
                    ctxt,
                    b"adding entry failed\0" as *const u8 as *const c_char,
                );
                xmlFreeElement(cur);
                cur = ::core::ptr::null_mut::<xmlElement>();
            }
        }
    }
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
    }
    if !uqname.is_null() {
        xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdQElementDesc(
    mut dtd: xmlDtdPtr,
    mut name: *const xmlChar,
    mut prefix: *const xmlChar,
) -> xmlElementPtr {
    let mut table: xmlElementTablePtr = ::core::ptr::null_mut::<xmlElementTable>();
    if dtd.is_null() {
        return ::core::ptr::null_mut::<xmlElement>();
    }
    if (*dtd).elements.is_null() {
        return ::core::ptr::null_mut::<xmlElement>();
    }
    table = (*dtd).elements as xmlElementTablePtr;
    return xmlHashLookup2(table as xmlHashTablePtr, name, prefix) as xmlElementPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdAttrDesc(
    mut dtd: xmlDtdPtr,
    mut elem: *const xmlChar,
    mut name: *const xmlChar,
) -> xmlAttributePtr {
    let mut table: xmlAttributeTablePtr = ::core::ptr::null_mut::<xmlAttributeTable>();
    let mut cur: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut uqname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if dtd.is_null() {
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    if (*dtd).attributes.is_null() {
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    table = (*dtd).attributes as xmlAttributeTablePtr;
    if table.is_null() {
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    uqname = xmlSplitQName2(name, &raw mut prefix);
    if !uqname.is_null() {
        cur = xmlHashLookup3(table as xmlHashTablePtr, uqname, prefix, elem) as xmlAttributePtr;
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as *mut c_void);
        }
        if !uqname.is_null() {
            xmlFree.expect("non-null function pointer")(uqname as *mut c_void);
        }
    } else {
        cur = xmlHashLookup3(
            table as xmlHashTablePtr,
            name,
            ::core::ptr::null::<xmlChar>(),
            elem,
        ) as xmlAttributePtr;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdQAttrDesc(
    mut dtd: xmlDtdPtr,
    mut elem: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix: *const xmlChar,
) -> xmlAttributePtr {
    let mut table: xmlAttributeTablePtr = ::core::ptr::null_mut::<xmlAttributeTable>();
    if dtd.is_null() {
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    if (*dtd).attributes.is_null() {
        return ::core::ptr::null_mut::<xmlAttribute>();
    }
    table = (*dtd).attributes as xmlAttributeTablePtr;
    return xmlHashLookup3(table as xmlHashTablePtr, name, prefix, elem) as xmlAttributePtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetDtdNotationDesc(
    mut dtd: xmlDtdPtr,
    mut name: *const xmlChar,
) -> xmlNotationPtr {
    let mut table: xmlNotationTablePtr = ::core::ptr::null_mut::<xmlNotationTable>();
    if dtd.is_null() {
        return ::core::ptr::null_mut::<xmlNotation>();
    }
    if (*dtd).notations.is_null() {
        return ::core::ptr::null_mut::<xmlNotation>();
    }
    table = (*dtd).notations as xmlNotationTablePtr;
    return xmlHashLookup(table as xmlHashTablePtr, name) as xmlNotationPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNotationUse(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut notationName: *const xmlChar,
) -> c_int {
    let mut notaDecl: xmlNotationPtr = ::core::ptr::null_mut::<xmlNotation>();
    if doc.is_null() || (*doc).intSubset.is_null() || notationName.is_null() {
        return -(1 as c_int);
    }
    notaDecl = xmlGetDtdNotationDesc((*doc).intSubset as xmlDtdPtr, notationName);
    if notaDecl.is_null() && !(*doc).extSubset.is_null() {
        notaDecl = xmlGetDtdNotationDesc((*doc).extSubset as xmlDtdPtr, notationName);
    }
    if notaDecl.is_null() && !ctxt.is_null() {
        xmlErrValidNode(
            ctxt,
            doc as xmlNodePtr,
            XML_DTD_UNKNOWN_NOTATION,
            b"NOTATION %s is not declared\n\0" as *const u8 as *const c_char,
            notationName,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        return 0 as c_int;
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsMixedElement(
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
) -> c_int {
    let mut elemDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    if doc.is_null() || (*doc).intSubset.is_null() {
        return -(1 as c_int);
    }
    elemDecl = xmlGetDtdElementDesc((*doc).intSubset as xmlDtdPtr, name);
    if elemDecl.is_null() && !(*doc).extSubset.is_null() {
        elemDecl = xmlGetDtdElementDesc((*doc).extSubset as xmlDtdPtr, name);
    }
    if elemDecl.is_null() {
        return -(1 as c_int);
    }
    match (*elemDecl).etype as c_uint {
        0 => return -(1 as c_int),
        4 => return 0 as c_int,
        1 | 2 | 3 => return 1 as c_int,
        _ => {}
    }
    return 1 as c_int;
}
unsafe extern "C" fn xmlIsDocNameStartChar(
    mut doc: xmlDocPtr,
    mut c: c_int,
) -> c_int {
    if doc.is_null()
        || (*doc).properties & XML_DOC_OLD10 as c_int == 0 as c_int
    {
        if c >= 'a' as i32 && c <= 'z' as i32
            || c >= 'A' as i32 && c <= 'Z' as i32
            || c == '_' as i32
            || c == ':' as i32
            || c >= 0xc0 as c_int && c <= 0xd6 as c_int
            || c >= 0xd8 as c_int && c <= 0xf6 as c_int
            || c >= 0xf8 as c_int && c <= 0x2ff as c_int
            || c >= 0x370 as c_int && c <= 0x37d as c_int
            || c >= 0x37f as c_int && c <= 0x1fff as c_int
            || c >= 0x200c as c_int && c <= 0x200d as c_int
            || c >= 0x2070 as c_int && c <= 0x218f as c_int
            || c >= 0x2c00 as c_int && c <= 0x2fef as c_int
            || c >= 0x3001 as c_int && c <= 0xd7ff as c_int
            || c >= 0xf900 as c_int && c <= 0xfdcf as c_int
            || c >= 0xfdf0 as c_int && c <= 0xfffd as c_int
            || c >= 0x10000 as c_int && c <= 0xeffff as c_int
        {
            return 1 as c_int;
        }
    } else if (if c < 0x100 as c_int {
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
        || c == '_' as i32
        || c == ':' as i32
    {
        return 1 as c_int;
    }
    return 0 as c_int;
}
unsafe extern "C" fn xmlIsDocNameChar(
    mut doc: xmlDocPtr,
    mut c: c_int,
) -> c_int {
    if doc.is_null()
        || (*doc).properties & XML_DOC_OLD10 as c_int == 0 as c_int
    {
        if c >= 'a' as i32 && c <= 'z' as i32
            || c >= 'A' as i32 && c <= 'Z' as i32
            || c >= '0' as i32 && c <= '9' as i32
            || c == '_' as i32
            || c == ':' as i32
            || c == '-' as i32
            || c == '.' as i32
            || c == 0xb7 as c_int
            || c >= 0xc0 as c_int && c <= 0xd6 as c_int
            || c >= 0xd8 as c_int && c <= 0xf6 as c_int
            || c >= 0xf8 as c_int && c <= 0x2ff as c_int
            || c >= 0x300 as c_int && c <= 0x36f as c_int
            || c >= 0x370 as c_int && c <= 0x37d as c_int
            || c >= 0x37f as c_int && c <= 0x1fff as c_int
            || c >= 0x200c as c_int && c <= 0x200d as c_int
            || c >= 0x203f as c_int && c <= 0x2040 as c_int
            || c >= 0x2070 as c_int && c <= 0x218f as c_int
            || c >= 0x2c00 as c_int && c <= 0x2fef as c_int
            || c >= 0x3001 as c_int && c <= 0xd7ff as c_int
            || c >= 0xf900 as c_int && c <= 0xfdcf as c_int
            || c >= 0xfdf0 as c_int && c <= 0xfffd as c_int
            || c >= 0x10000 as c_int && c <= 0xeffff as c_int
        {
            return 1 as c_int;
        }
    } else if (if c < 0x100 as c_int {
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
        }) != 0
    {
        return 1 as c_int;
    }
    return 0 as c_int;
}
unsafe extern "C" fn xmlValidateNameValueInternal(
    mut doc: xmlDocPtr,
    mut value: *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut val: c_int = 0;
    let mut len: c_int = 0;
    if value.is_null() {
        return 0 as c_int;
    }
    cur = value;
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    cur = cur.offset(len as isize);
    if xmlIsDocNameStartChar(doc, val) == 0 {
        return 0 as c_int;
    }
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    cur = cur.offset(len as isize);
    while xmlIsDocNameChar(doc, val) != 0 {
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
        cur = cur.offset(len as isize);
    }
    if val != 0 as c_int {
        return 0 as c_int;
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNameValue(mut value: *const xmlChar) -> c_int {
    return xmlValidateNameValueInternal(::core::ptr::null_mut::<xmlDoc>(), value);
}
unsafe extern "C" fn xmlValidateNamesValueInternal(
    mut doc: xmlDocPtr,
    mut value: *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut val: c_int = 0;
    let mut len: c_int = 0;
    if value.is_null() {
        return 0 as c_int;
    }
    cur = value;
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    cur = cur.offset(len as isize);
    if xmlIsDocNameStartChar(doc, val) == 0 {
        return 0 as c_int;
    }
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    cur = cur.offset(len as isize);
    while xmlIsDocNameChar(doc, val) != 0 {
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
        cur = cur.offset(len as isize);
    }
    while val == 0x20 as c_int {
        while val == 0x20 as c_int {
            val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
            cur = cur.offset(len as isize);
        }
        if xmlIsDocNameStartChar(doc, val) == 0 {
            return 0 as c_int;
        }
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
        cur = cur.offset(len as isize);
        while xmlIsDocNameChar(doc, val) != 0 {
            val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
            cur = cur.offset(len as isize);
        }
    }
    if val != 0 as c_int {
        return 0 as c_int;
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNamesValue(mut value: *const xmlChar) -> c_int {
    return xmlValidateNamesValueInternal(::core::ptr::null_mut::<xmlDoc>(), value);
}
unsafe extern "C" fn xmlValidateNmtokenValueInternal(
    mut doc: xmlDocPtr,
    mut value: *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut val: c_int = 0;
    let mut len: c_int = 0;
    if value.is_null() {
        return 0 as c_int;
    }
    cur = value;
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    cur = cur.offset(len as isize);
    if xmlIsDocNameChar(doc, val) == 0 {
        return 0 as c_int;
    }
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    cur = cur.offset(len as isize);
    while xmlIsDocNameChar(doc, val) != 0 {
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
        cur = cur.offset(len as isize);
    }
    if val != 0 as c_int {
        return 0 as c_int;
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNmtokenValue(mut value: *const xmlChar) -> c_int {
    return xmlValidateNmtokenValueInternal(::core::ptr::null_mut::<xmlDoc>(), value);
}
unsafe extern "C" fn xmlValidateNmtokensValueInternal(
    mut doc: xmlDocPtr,
    mut value: *const xmlChar,
) -> c_int {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut val: c_int = 0;
    let mut len: c_int = 0;
    if value.is_null() {
        return 0 as c_int;
    }
    cur = value;
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    cur = cur.offset(len as isize);
    while if val < 0x100 as c_int {
        (val == 0x20 as c_int
            || 0x9 as c_int <= val && val <= 0xa as c_int
            || val == 0xd as c_int) as c_int
    } else {
        0 as c_int
    } != 0
    {
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
        cur = cur.offset(len as isize);
    }
    if xmlIsDocNameChar(doc, val) == 0 {
        return 0 as c_int;
    }
    while xmlIsDocNameChar(doc, val) != 0 {
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
        cur = cur.offset(len as isize);
    }
    while val == 0x20 as c_int {
        while val == 0x20 as c_int {
            val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
            cur = cur.offset(len as isize);
        }
        if val == 0 as c_int {
            return 1 as c_int;
        }
        if xmlIsDocNameChar(doc, val) == 0 {
            return 0 as c_int;
        }
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
        cur = cur.offset(len as isize);
        while xmlIsDocNameChar(doc, val) != 0 {
            val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
            cur = cur.offset(len as isize);
        }
    }
    if val != 0 as c_int {
        return 0 as c_int;
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNmtokensValue(mut value: *const xmlChar) -> c_int {
    return xmlValidateNmtokensValueInternal(::core::ptr::null_mut::<xmlDoc>(), value);
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateNotationDecl(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut nota: xmlNotationPtr,
) -> c_int {
    let mut ret: c_int = 1 as c_int;
    return ret;
}
unsafe extern "C" fn xmlValidateAttributeValueInternal(
    mut doc: xmlDocPtr,
    mut type_0: xmlAttributeType,
    mut value: *const xmlChar,
) -> c_int {
    match type_0 as c_uint {
        6 | 4 => return xmlValidateNamesValueInternal(doc, value),
        5 | 3 | 2 | 10 => return xmlValidateNameValueInternal(doc, value),
        8 | 9 => return xmlValidateNmtokensValueInternal(doc, value),
        7 => return xmlValidateNmtokenValueInternal(doc, value),
        1 | _ => {}
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateAttributeValue(
    mut type_0: xmlAttributeType,
    mut value: *const xmlChar,
) -> c_int {
    return xmlValidateAttributeValueInternal(::core::ptr::null_mut::<xmlDoc>(), type_0, value);
}
unsafe extern "C" fn xmlValidateAttributeValue2(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut name: *const xmlChar,
    mut type_0: xmlAttributeType,
    mut value: *const xmlChar,
) -> c_int {
    let mut ret: c_int = 1 as c_int;
    match type_0 as c_uint {
        5 => {
            let mut ent: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
            ent = xmlGetDocEntity(doc as *const xmlDoc, value);
            if ent.is_null() && (*doc).standalone == 1 as c_int {
                (*doc).standalone = 0 as c_int;
                ent = xmlGetDocEntity(doc as *const xmlDoc, value);
            }
            if ent.is_null() {
                xmlErrValidNode(
                    ctxt,
                    doc as xmlNodePtr,
                    XML_DTD_UNKNOWN_ENTITY,
                    b"ENTITY attribute %s reference an unknown entity \"%s\"\n\0" as *const u8
                        as *const c_char,
                    name,
                    value,
                    ::core::ptr::null::<xmlChar>(),
                );
                ret = 0 as c_int;
            } else if (*ent).etype as c_uint
                != XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as c_int as c_uint
            {
                xmlErrValidNode(
                    ctxt,
                    doc as xmlNodePtr,
                    XML_DTD_ENTITY_TYPE,
                    b"ENTITY attribute %s reference an entity \"%s\" of wrong type\n\0" as *const u8
                        as *const c_char,
                    name,
                    value,
                    ::core::ptr::null::<xmlChar>(),
                );
                ret = 0 as c_int;
            }
        }
        6 => {
            let mut dup: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut nam: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut cur: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            let mut save: xmlChar = 0;
            let mut ent_0: xmlEntityPtr = ::core::ptr::null_mut::<xmlEntity>();
            dup = xmlStrdup(value);
            if dup.is_null() {
                return 0 as c_int;
            }
            cur = dup;
            while *cur as c_int != 0 as c_int {
                nam = cur;
                while *cur as c_int != 0 as c_int
                    && !(*cur as c_int == 0x20 as c_int
                        || 0x9 as c_int <= *cur as c_int
                            && *cur as c_int <= 0xa as c_int
                        || *cur as c_int == 0xd as c_int)
                {
                    cur = cur.offset(1);
                }
                save = *cur;
                *cur = 0 as xmlChar;
                ent_0 = xmlGetDocEntity(doc as *const xmlDoc, nam);
                if ent_0.is_null() {
                    xmlErrValidNode(
                        ctxt,
                        doc as xmlNodePtr,
                        XML_DTD_UNKNOWN_ENTITY,
                        b"ENTITIES attribute %s reference an unknown entity \"%s\"\n\0" as *const u8
                            as *const c_char,
                        name,
                        nam,
                        ::core::ptr::null::<xmlChar>(),
                    );
                    ret = 0 as c_int;
                } else if (*ent_0).etype as c_uint
                    != XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as c_int
                        as c_uint
                {
                    xmlErrValidNode(
                        ctxt,
                        doc as xmlNodePtr,
                        XML_DTD_ENTITY_TYPE,
                        b"ENTITIES attribute %s reference an entity \"%s\" of wrong type\n\0"
                            as *const u8 as *const c_char,
                        name,
                        nam,
                        ::core::ptr::null::<xmlChar>(),
                    );
                    ret = 0 as c_int;
                }
                if save as c_int == 0 as c_int {
                    break;
                }
                *cur = save;
                while *cur as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *cur as c_int
                        && *cur as c_int <= 0xa as c_int
                    || *cur as c_int == 0xd as c_int
                {
                    cur = cur.offset(1);
                }
            }
            xmlFree.expect("non-null function pointer")(dup as *mut c_void);
        }
        10 => {
            let mut nota: xmlNotationPtr = ::core::ptr::null_mut::<xmlNotation>();
            nota = xmlGetDtdNotationDesc((*doc).intSubset as xmlDtdPtr, value);
            if nota.is_null() && !(*doc).extSubset.is_null() {
                nota = xmlGetDtdNotationDesc((*doc).extSubset as xmlDtdPtr, value);
            }
            if nota.is_null() {
                xmlErrValidNode(
                    ctxt,
                    doc as xmlNodePtr,
                    XML_DTD_UNKNOWN_NOTATION,
                    b"NOTATION attribute %s reference an unknown notation \"%s\"\n\0" as *const u8
                        as *const c_char,
                    name,
                    value,
                    ::core::ptr::null::<xmlChar>(),
                );
                ret = 0 as c_int;
            }
        }
        4 | 3 | 2 | 8 | 9 | 7 | 1 | _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidCtxtNormalizeAttributeValue(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut attrDecl: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut extsubset: c_int = 0 as c_int;
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if elem.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
        let mut fn_0: [xmlChar; 50] = [0; 50];
        let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        fullname = xmlBuildQName(
            (*elem).name,
            (*(*elem).ns).prefix,
            &raw mut fn_0 as *mut xmlChar,
            50 as c_int,
        );
        if fullname.is_null() {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        attrDecl = xmlGetDtdAttrDesc((*doc).intSubset as xmlDtdPtr, fullname, name);
        if attrDecl.is_null() && !(*doc).extSubset.is_null() {
            attrDecl = xmlGetDtdAttrDesc((*doc).extSubset as xmlDtdPtr, fullname, name);
            if !attrDecl.is_null() {
                extsubset = 1 as c_int;
            }
        }
        if fullname != &raw mut fn_0 as *mut xmlChar && fullname != (*elem).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(fullname as *mut c_void);
        }
    }
    if attrDecl.is_null() && !(*doc).intSubset.is_null() {
        attrDecl = xmlGetDtdAttrDesc((*doc).intSubset as xmlDtdPtr, (*elem).name, name);
    }
    if attrDecl.is_null() && !(*doc).extSubset.is_null() {
        attrDecl = xmlGetDtdAttrDesc((*doc).extSubset as xmlDtdPtr, (*elem).name, name);
        if !attrDecl.is_null() {
            extsubset = 1 as c_int;
        }
    }
    if attrDecl.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*attrDecl).atype as c_uint
        == XML_ATTRIBUTE_CDATA as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlStrdup(value);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    xmlValidNormalizeString(ret);
    if (*doc).standalone != 0
        && extsubset == 1 as c_int
        && xmlStrEqual(value, ret) == 0
    {
        xmlErrValidNode(
            ctxt,
            elem,
            XML_DTD_NOT_STANDALONE,
            b"standalone: %s on %s value had to be normalized based on external subset declaration\n\0"
                as *const u8 as *const c_char,
            name,
            (*elem).name,
            ::core::ptr::null::<xmlChar>(),
        );
        (*ctxt).valid = 0 as c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidNormalizeAttributeValue(
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut name: *const xmlChar,
    mut value: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut attrDecl: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    if doc.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if elem.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if value.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
        let mut fn_0: [xmlChar; 50] = [0; 50];
        let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        fullname = xmlBuildQName(
            (*elem).name,
            (*(*elem).ns).prefix,
            &raw mut fn_0 as *mut xmlChar,
            50 as c_int,
        );
        if fullname.is_null() {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        if fullname != &raw mut fn_0 as *mut xmlChar && fullname != (*elem).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(fullname as *mut c_void);
        }
    }
    attrDecl = xmlGetDtdAttrDesc((*doc).intSubset as xmlDtdPtr, (*elem).name, name);
    if attrDecl.is_null() && !(*doc).extSubset.is_null() {
        attrDecl = xmlGetDtdAttrDesc((*doc).extSubset as xmlDtdPtr, (*elem).name, name);
    }
    if attrDecl.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*attrDecl).atype as c_uint
        == XML_ATTRIBUTE_CDATA as c_int as c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ret = xmlStrdup(value);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    xmlValidNormalizeString(ret);
    return ret;
}
unsafe extern "C" fn xmlValidateAttributeIdCallback(
    mut payload: *mut c_void,
    mut data: *mut c_void,
    mut name: *const xmlChar,
) {
    let mut attr: xmlAttributePtr = payload as xmlAttributePtr;
    let mut count: *mut c_int = data as *mut c_int;
    if (*attr).atype as c_uint
        == XML_ATTRIBUTE_ID as c_int as c_uint
    {
        *count += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateAttributeDecl(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut attr: xmlAttributePtr,
) -> c_int {
    let mut ret: c_int = 1 as c_int;
    let mut val: c_int = 0;
    if doc.is_null() {
        return 0 as c_int;
    } else if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as c_int;
    }
    if attr.is_null() {
        return 1 as c_int;
    }
    if !(*attr).defaultValue.is_null() {
        val = xmlValidateAttributeValueInternal(doc, (*attr).atype, (*attr).defaultValue);
        if val == 0 as c_int {
            xmlErrValidNode(
                ctxt,
                attr as xmlNodePtr,
                XML_DTD_ATTRIBUTE_DEFAULT,
                b"Syntax of default value for attribute %s of %s is not valid\n\0" as *const u8
                    as *const c_char,
                (*attr).name,
                (*attr).elem,
                ::core::ptr::null::<xmlChar>(),
            );
        }
        ret &= val;
    }
    if (*attr).atype as c_uint
        == XML_ATTRIBUTE_ID as c_int as c_uint
        && (*attr).def as c_uint
            != XML_ATTRIBUTE_IMPLIED as c_int as c_uint
        && (*attr).def as c_uint
            != XML_ATTRIBUTE_REQUIRED as c_int as c_uint
    {
        xmlErrValidNode(
            ctxt,
            attr as xmlNodePtr,
            XML_DTD_ID_FIXED,
            b"ID attribute %s of %s is not valid must be #IMPLIED or #REQUIRED\n\0" as *const u8
                as *const c_char,
            (*attr).name,
            (*attr).elem,
            ::core::ptr::null::<xmlChar>(),
        );
        ret = 0 as c_int;
    }
    if (*attr).atype as c_uint
        == XML_ATTRIBUTE_ID as c_int as c_uint
    {
        let mut nbId: c_int = 0;
        let mut elem: xmlElementPtr =
            xmlGetDtdElementDesc((*doc).intSubset as xmlDtdPtr, (*attr).elem);
        if !elem.is_null() {
            nbId = xmlScanIDAttributeDecl(
                ::core::ptr::null_mut::<xmlValidCtxt>(),
                elem,
                0 as c_int,
            );
        } else {
            let mut table: xmlAttributeTablePtr = ::core::ptr::null_mut::<xmlAttributeTable>();
            nbId = 0 as c_int;
            if !(*doc).intSubset.is_null() {
                table = (*(*doc).intSubset).attributes as xmlAttributeTablePtr;
                xmlHashScan3(
                    table as xmlHashTablePtr,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                    (*attr).elem,
                    Some(
                        xmlValidateAttributeIdCallback
                            as unsafe extern "C" fn(
                                *mut c_void,
                                *mut c_void,
                                *const xmlChar,
                            ) -> (),
                    ),
                    &raw mut nbId as *mut c_void,
                );
            }
        }
        if nbId > 1 as c_int {
            xmlErrValidNodeNr(
                ctxt,
                attr as xmlNodePtr,
                XML_DTD_ID_SUBSET,
                b"Element %s has %d ID attribute defined in the internal subset : %s\n\0"
                    as *const u8 as *const c_char,
                (*attr).elem,
                nbId,
                (*attr).name,
            );
        } else if !(*doc).extSubset.is_null() {
            let mut extId: c_int = 0 as c_int;
            elem = xmlGetDtdElementDesc((*doc).extSubset as xmlDtdPtr, (*attr).elem);
            if !elem.is_null() {
                extId = xmlScanIDAttributeDecl(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    elem,
                    0 as c_int,
                );
            }
            if extId > 1 as c_int {
                xmlErrValidNodeNr(
                    ctxt,
                    attr as xmlNodePtr,
                    XML_DTD_ID_SUBSET,
                    b"Element %s has %d ID attribute defined in the external subset : %s\n\0"
                        as *const u8 as *const c_char,
                    (*attr).elem,
                    extId,
                    (*attr).name,
                );
            } else if extId + nbId > 1 as c_int {
                xmlErrValidNode(
                    ctxt,
                    attr as xmlNodePtr,
                    XML_DTD_ID_SUBSET,
                    b"Element %s has ID attributes defined in the internal and external subset : %s\n\0"
                        as *const u8 as *const c_char,
                    (*attr).elem,
                    (*attr).name,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
        }
    }
    if !(*attr).defaultValue.is_null() && !(*attr).tree.is_null() {
        let mut tree: xmlEnumerationPtr = (*attr).tree;
        while !tree.is_null() {
            if xmlStrEqual((*tree).name, (*attr).defaultValue) != 0 {
                break;
            }
            tree = (*tree).next as xmlEnumerationPtr;
        }
        if tree.is_null() {
            xmlErrValidNode(
                ctxt,
                attr as xmlNodePtr,
                XML_DTD_ATTRIBUTE_VALUE,
                b"Default value \"%s\" for attribute %s of %s is not among the enumerated set\n\0"
                    as *const u8 as *const c_char,
                (*attr).defaultValue,
                (*attr).name,
                (*attr).elem,
            );
            ret = 0 as c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateElementDecl(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlElementPtr,
) -> c_int {
    let mut ret: c_int = 1 as c_int;
    let mut tst: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    if doc.is_null() {
        return 0 as c_int;
    } else if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as c_int;
    }
    if elem.is_null() {
        return 1 as c_int;
    }
    if (*elem).etype as c_uint
        == XML_ELEMENT_TYPE_MIXED as c_int as c_uint
    {
        let mut cur: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
        let mut next: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
        let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
        cur = (*elem).content;
        while !cur.is_null() {
            if (*cur).type_0 as c_uint
                != XML_ELEMENT_CONTENT_OR as c_int as c_uint
            {
                break;
            }
            if (*cur).c1.is_null() {
                break;
            }
            if (*(*cur).c1).type_0 as c_uint
                == XML_ELEMENT_CONTENT_ELEMENT as c_int as c_uint
            {
                name = (*(*cur).c1).name;
                next = (*cur).c2 as xmlElementContentPtr;
                while !next.is_null() {
                    if (*next).type_0 as c_uint
                        == XML_ELEMENT_CONTENT_ELEMENT as c_int as c_uint
                    {
                        if xmlStrEqual((*next).name, name) != 0
                            && xmlStrEqual((*next).prefix, (*(*cur).c1).prefix) != 0
                        {
                            if (*(*cur).c1).prefix.is_null() {
                                xmlErrValidNode(
                                    ctxt,
                                    elem as xmlNodePtr,
                                    XML_DTD_CONTENT_ERROR,
                                    b"Definition of %s has duplicate references of %s\n\0"
                                        as *const u8
                                        as *const c_char,
                                    (*elem).name,
                                    name,
                                    ::core::ptr::null::<xmlChar>(),
                                );
                            } else {
                                xmlErrValidNode(
                                    ctxt,
                                    elem as xmlNodePtr,
                                    XML_DTD_CONTENT_ERROR,
                                    b"Definition of %s has duplicate references of %s:%s\n\0"
                                        as *const u8
                                        as *const c_char,
                                    (*elem).name,
                                    (*(*cur).c1).prefix,
                                    name,
                                );
                            }
                            ret = 0 as c_int;
                        }
                        break;
                    } else {
                        if (*next).c1.is_null() {
                            break;
                        }
                        if (*(*next).c1).type_0 as c_uint
                            != XML_ELEMENT_CONTENT_ELEMENT as c_int
                                as c_uint
                        {
                            break;
                        }
                        if xmlStrEqual((*(*next).c1).name, name) != 0
                            && xmlStrEqual((*(*next).c1).prefix, (*(*cur).c1).prefix) != 0
                        {
                            if (*(*cur).c1).prefix.is_null() {
                                xmlErrValidNode(
                                    ctxt,
                                    elem as xmlNodePtr,
                                    XML_DTD_CONTENT_ERROR,
                                    b"Definition of %s has duplicate references to %s\n\0"
                                        as *const u8
                                        as *const c_char,
                                    (*elem).name,
                                    name,
                                    ::core::ptr::null::<xmlChar>(),
                                );
                            } else {
                                xmlErrValidNode(
                                    ctxt,
                                    elem as xmlNodePtr,
                                    XML_DTD_CONTENT_ERROR,
                                    b"Definition of %s has duplicate references to %s:%s\n\0"
                                        as *const u8
                                        as *const c_char,
                                    (*elem).name,
                                    (*(*cur).c1).prefix,
                                    name,
                                );
                            }
                            ret = 0 as c_int;
                        }
                        next = (*next).c2 as xmlElementContentPtr;
                    }
                }
            }
            cur = (*cur).c2 as xmlElementContentPtr;
        }
    }
    tst = xmlGetDtdElementDesc((*doc).intSubset as xmlDtdPtr, (*elem).name);
    if !tst.is_null()
        && tst != elem
        && ((*tst).prefix == (*elem).prefix || xmlStrEqual((*tst).prefix, (*elem).prefix) != 0)
        && (*tst).etype as c_uint
            != XML_ELEMENT_TYPE_UNDEFINED as c_int as c_uint
    {
        xmlErrValidNode(
            ctxt,
            elem as xmlNodePtr,
            XML_DTD_ELEM_REDEFINED,
            b"Redefinition of element %s\n\0" as *const u8 as *const c_char,
            (*elem).name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        ret = 0 as c_int;
    }
    tst = xmlGetDtdElementDesc((*doc).extSubset as xmlDtdPtr, (*elem).name);
    if !tst.is_null()
        && tst != elem
        && ((*tst).prefix == (*elem).prefix || xmlStrEqual((*tst).prefix, (*elem).prefix) != 0)
        && (*tst).etype as c_uint
            != XML_ELEMENT_TYPE_UNDEFINED as c_int as c_uint
    {
        xmlErrValidNode(
            ctxt,
            elem as xmlNodePtr,
            XML_DTD_ELEM_REDEFINED,
            b"Redefinition of element %s\n\0" as *const u8 as *const c_char,
            (*elem).name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
        ret = 0 as c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateOneAttribute(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut attr: xmlAttrPtr,
    mut value: *const xmlChar,
) -> c_int {
    let mut attrDecl: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut val: c_int = 0;
    let mut ret: c_int = 1 as c_int;
    if doc.is_null() {
        return 0 as c_int;
    } else if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as c_int;
    }
    if elem.is_null() || (*elem).name.is_null() {
        return 0 as c_int;
    }
    if attr.is_null() || (*attr).name.is_null() {
        return 0 as c_int;
    }
    if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
        let mut fn_0: [xmlChar; 50] = [0; 50];
        let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        fullname = xmlBuildQName(
            (*elem).name,
            (*(*elem).ns).prefix,
            &raw mut fn_0 as *mut xmlChar,
            50 as c_int,
        );
        if fullname.is_null() {
            return 0 as c_int;
        }
        if !(*attr).ns.is_null() {
            attrDecl = xmlGetDtdQAttrDesc(
                (*doc).intSubset as xmlDtdPtr,
                fullname,
                (*attr).name,
                (*(*attr).ns).prefix,
            );
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdQAttrDesc(
                    (*doc).extSubset as xmlDtdPtr,
                    fullname,
                    (*attr).name,
                    (*(*attr).ns).prefix,
                );
            }
        } else {
            attrDecl = xmlGetDtdAttrDesc((*doc).intSubset as xmlDtdPtr, fullname, (*attr).name);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdAttrDesc((*doc).extSubset as xmlDtdPtr, fullname, (*attr).name);
            }
        }
        if fullname != &raw mut fn_0 as *mut xmlChar && fullname != (*elem).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(fullname as *mut c_void);
        }
    }
    if attrDecl.is_null() {
        if !(*attr).ns.is_null() {
            attrDecl = xmlGetDtdQAttrDesc(
                (*doc).intSubset as xmlDtdPtr,
                (*elem).name,
                (*attr).name,
                (*(*attr).ns).prefix,
            );
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdQAttrDesc(
                    (*doc).extSubset as xmlDtdPtr,
                    (*elem).name,
                    (*attr).name,
                    (*(*attr).ns).prefix,
                );
            }
        } else {
            attrDecl = xmlGetDtdAttrDesc((*doc).intSubset as xmlDtdPtr, (*elem).name, (*attr).name);
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl =
                    xmlGetDtdAttrDesc((*doc).extSubset as xmlDtdPtr, (*elem).name, (*attr).name);
            }
        }
    }
    if attrDecl.is_null() {
        xmlErrValidNode(
            ctxt,
            elem,
            XML_DTD_UNKNOWN_ATTRIBUTE,
            b"No declaration for attribute %s of element %s\n\0" as *const u8
                as *const c_char,
            (*attr).name,
            (*elem).name,
            ::core::ptr::null::<xmlChar>(),
        );
        return 0 as c_int;
    }
    (*attr).atype = (*attrDecl).atype;
    val = xmlValidateAttributeValueInternal(doc, (*attrDecl).atype, value);
    if val == 0 as c_int {
        xmlErrValidNode(
            ctxt,
            elem,
            XML_DTD_ATTRIBUTE_VALUE,
            b"Syntax of value for attribute %s of %s is not valid\n\0" as *const u8
                as *const c_char,
            (*attr).name,
            (*elem).name,
            ::core::ptr::null::<xmlChar>(),
        );
        ret = 0 as c_int;
    }
    if (*attrDecl).def as c_uint
        == XML_ATTRIBUTE_FIXED as c_int as c_uint
    {
        if xmlStrEqual(value, (*attrDecl).defaultValue) == 0 {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_ATTRIBUTE_DEFAULT,
                b"Value for attribute %s of %s is different from default \"%s\"\n\0" as *const u8
                    as *const c_char,
                (*attr).name,
                (*elem).name,
                (*attrDecl).defaultValue,
            );
            ret = 0 as c_int;
        }
    }
    if (*attrDecl).atype as c_uint
        == XML_ATTRIBUTE_ID as c_int as c_uint
    {
        if xmlAddID(ctxt, doc, value, attr).is_null() {
            ret = 0 as c_int;
        }
    }
    if (*attrDecl).atype as c_uint
        == XML_ATTRIBUTE_IDREF as c_int as c_uint
        || (*attrDecl).atype as c_uint
            == XML_ATTRIBUTE_IDREFS as c_int as c_uint
    {
        if xmlAddRef(ctxt, doc, value, attr).is_null() {
            ret = 0 as c_int;
        }
    }
    if (*attrDecl).atype as c_uint
        == XML_ATTRIBUTE_NOTATION as c_int as c_uint
    {
        let mut tree: xmlEnumerationPtr = (*attrDecl).tree;
        let mut nota: xmlNotationPtr = ::core::ptr::null_mut::<xmlNotation>();
        nota = xmlGetDtdNotationDesc((*doc).intSubset as xmlDtdPtr, value);
        if nota.is_null() {
            nota = xmlGetDtdNotationDesc((*doc).extSubset as xmlDtdPtr, value);
        }
        if nota.is_null() {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_UNKNOWN_NOTATION,
                b"Value \"%s\" for attribute %s of %s is not a declared Notation\n\0" as *const u8
                    as *const c_char,
                value,
                (*attr).name,
                (*elem).name,
            );
            ret = 0 as c_int;
        }
        while !tree.is_null() {
            if xmlStrEqual((*tree).name, value) != 0 {
                break;
            }
            tree = (*tree).next as xmlEnumerationPtr;
        }
        if tree.is_null() {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_NOTATION_VALUE,
                b"Value \"%s\" for attribute %s of %s is not among the enumerated notations\n\0"
                    as *const u8 as *const c_char,
                value,
                (*attr).name,
                (*elem).name,
            );
            ret = 0 as c_int;
        }
    }
    if (*attrDecl).atype as c_uint
        == XML_ATTRIBUTE_ENUMERATION as c_int as c_uint
    {
        let mut tree_0: xmlEnumerationPtr = (*attrDecl).tree;
        while !tree_0.is_null() {
            if xmlStrEqual((*tree_0).name, value) != 0 {
                break;
            }
            tree_0 = (*tree_0).next as xmlEnumerationPtr;
        }
        if tree_0.is_null() {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_ATTRIBUTE_VALUE,
                b"Value \"%s\" for attribute %s of %s is not among the enumerated set\n\0"
                    as *const u8 as *const c_char,
                value,
                (*attr).name,
                (*elem).name,
            );
            ret = 0 as c_int;
        }
    }
    if (*attrDecl).def as c_uint
        == XML_ATTRIBUTE_FIXED as c_int as c_uint
        && xmlStrEqual((*attrDecl).defaultValue, value) == 0
    {
        xmlErrValidNode(
            ctxt,
            elem,
            XML_DTD_ATTRIBUTE_VALUE,
            b"Value for attribute %s of %s must be \"%s\"\n\0" as *const u8
                as *const c_char,
            (*attr).name,
            (*elem).name,
            (*attrDecl).defaultValue,
        );
        ret = 0 as c_int;
    }
    ret &= xmlValidateAttributeValue2(ctxt, doc, (*attr).name, (*attrDecl).atype, value);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateOneNamespace(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut prefix: *const xmlChar,
    mut ns: xmlNsPtr,
    mut value: *const xmlChar,
) -> c_int {
    let mut attrDecl: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut val: c_int = 0;
    let mut ret: c_int = 1 as c_int;
    if doc.is_null() {
        return 0 as c_int;
    } else if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as c_int;
    }
    if elem.is_null() || (*elem).name.is_null() {
        return 0 as c_int;
    }
    if ns.is_null() || (*ns).href.is_null() {
        return 0 as c_int;
    }
    if !prefix.is_null() {
        let mut fn_0: [xmlChar; 50] = [0; 50];
        let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        fullname = xmlBuildQName(
            (*elem).name,
            prefix,
            &raw mut fn_0 as *mut xmlChar,
            50 as c_int,
        );
        if fullname.is_null() {
            xmlVErrMemory(
                ctxt,
                b"Validating namespace\0" as *const u8 as *const c_char,
            );
            return 0 as c_int;
        }
        if !(*ns).prefix.is_null() {
            attrDecl = xmlGetDtdQAttrDesc(
                (*doc).intSubset as xmlDtdPtr,
                fullname,
                (*ns).prefix,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdQAttrDesc(
                    (*doc).extSubset as xmlDtdPtr,
                    fullname,
                    (*ns).prefix,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                );
            }
        } else {
            attrDecl = xmlGetDtdAttrDesc(
                (*doc).intSubset as xmlDtdPtr,
                fullname,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdAttrDesc(
                    (*doc).extSubset as xmlDtdPtr,
                    fullname,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                );
            }
        }
        if fullname != &raw mut fn_0 as *mut xmlChar && fullname != (*elem).name as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(fullname as *mut c_void);
        }
    }
    if attrDecl.is_null() {
        if !(*ns).prefix.is_null() {
            attrDecl = xmlGetDtdQAttrDesc(
                (*doc).intSubset as xmlDtdPtr,
                (*elem).name,
                (*ns).prefix,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdQAttrDesc(
                    (*doc).extSubset as xmlDtdPtr,
                    (*elem).name,
                    (*ns).prefix,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                );
            }
        } else {
            attrDecl = xmlGetDtdAttrDesc(
                (*doc).intSubset as xmlDtdPtr,
                (*elem).name,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            if attrDecl.is_null() && !(*doc).extSubset.is_null() {
                attrDecl = xmlGetDtdAttrDesc(
                    (*doc).extSubset as xmlDtdPtr,
                    (*elem).name,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                );
            }
        }
    }
    if attrDecl.is_null() {
        if !(*ns).prefix.is_null() {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_UNKNOWN_ATTRIBUTE,
                b"No declaration for attribute xmlns:%s of element %s\n\0" as *const u8
                    as *const c_char,
                (*ns).prefix,
                (*elem).name,
                ::core::ptr::null::<xmlChar>(),
            );
        } else {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_UNKNOWN_ATTRIBUTE,
                b"No declaration for attribute xmlns of element %s\n\0" as *const u8
                    as *const c_char,
                (*elem).name,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
        }
        return 0 as c_int;
    }
    val = xmlValidateAttributeValueInternal(doc, (*attrDecl).atype, value);
    if val == 0 as c_int {
        if !(*ns).prefix.is_null() {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_INVALID_DEFAULT,
                b"Syntax of value for attribute xmlns:%s of %s is not valid\n\0" as *const u8
                    as *const c_char,
                (*ns).prefix,
                (*elem).name,
                ::core::ptr::null::<xmlChar>(),
            );
        } else {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_INVALID_DEFAULT,
                b"Syntax of value for attribute xmlns of %s is not valid\n\0" as *const u8
                    as *const c_char,
                (*elem).name,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
        }
        ret = 0 as c_int;
    }
    if (*attrDecl).def as c_uint
        == XML_ATTRIBUTE_FIXED as c_int as c_uint
    {
        if xmlStrEqual(value, (*attrDecl).defaultValue) == 0 {
            if !(*ns).prefix.is_null() {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_ATTRIBUTE_DEFAULT,
                    b"Value for attribute xmlns:%s of %s is different from default \"%s\"\n\0"
                        as *const u8 as *const c_char,
                    (*ns).prefix,
                    (*elem).name,
                    (*attrDecl).defaultValue,
                );
            } else {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_ATTRIBUTE_DEFAULT,
                    b"Value for attribute xmlns of %s is different from default \"%s\"\n\0"
                        as *const u8 as *const c_char,
                    (*elem).name,
                    (*attrDecl).defaultValue,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            ret = 0 as c_int;
        }
    }
    if (*attrDecl).atype as c_uint
        == XML_ATTRIBUTE_NOTATION as c_int as c_uint
    {
        let mut tree: xmlEnumerationPtr = (*attrDecl).tree;
        let mut nota: xmlNotationPtr = ::core::ptr::null_mut::<xmlNotation>();
        nota = xmlGetDtdNotationDesc((*doc).intSubset as xmlDtdPtr, value);
        if nota.is_null() {
            nota = xmlGetDtdNotationDesc((*doc).extSubset as xmlDtdPtr, value);
        }
        if nota.is_null() {
            if !(*ns).prefix.is_null() {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_UNKNOWN_NOTATION,
                    b"Value \"%s\" for attribute xmlns:%s of %s is not a declared Notation\n\0"
                        as *const u8 as *const c_char,
                    value,
                    (*ns).prefix,
                    (*elem).name,
                );
            } else {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_UNKNOWN_NOTATION,
                    b"Value \"%s\" for attribute xmlns of %s is not a declared Notation\n\0"
                        as *const u8 as *const c_char,
                    value,
                    (*elem).name,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            ret = 0 as c_int;
        }
        while !tree.is_null() {
            if xmlStrEqual((*tree).name, value) != 0 {
                break;
            }
            tree = (*tree).next as xmlEnumerationPtr;
        }
        if tree.is_null() {
            if !(*ns).prefix.is_null() {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_NOTATION_VALUE,
                    b"Value \"%s\" for attribute xmlns:%s of %s is not among the enumerated notations\n\0"
                        as *const u8 as *const c_char,
                    value,
                    (*ns).prefix,
                    (*elem).name,
                );
            } else {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_NOTATION_VALUE,
                    b"Value \"%s\" for attribute xmlns of %s is not among the enumerated notations\n\0"
                        as *const u8 as *const c_char,
                    value,
                    (*elem).name,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            ret = 0 as c_int;
        }
    }
    if (*attrDecl).atype as c_uint
        == XML_ATTRIBUTE_ENUMERATION as c_int as c_uint
    {
        let mut tree_0: xmlEnumerationPtr = (*attrDecl).tree;
        while !tree_0.is_null() {
            if xmlStrEqual((*tree_0).name, value) != 0 {
                break;
            }
            tree_0 = (*tree_0).next as xmlEnumerationPtr;
        }
        if tree_0.is_null() {
            if !(*ns).prefix.is_null() {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_ATTRIBUTE_VALUE,
                    b"Value \"%s\" for attribute xmlns:%s of %s is not among the enumerated set\n\0"
                        as *const u8 as *const c_char,
                    value,
                    (*ns).prefix,
                    (*elem).name,
                );
            } else {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_ATTRIBUTE_VALUE,
                    b"Value \"%s\" for attribute xmlns of %s is not among the enumerated set\n\0"
                        as *const u8 as *const c_char,
                    value,
                    (*elem).name,
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            ret = 0 as c_int;
        }
    }
    if (*attrDecl).def as c_uint
        == XML_ATTRIBUTE_FIXED as c_int as c_uint
        && xmlStrEqual((*attrDecl).defaultValue, value) == 0
    {
        if !(*ns).prefix.is_null() {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_ELEM_NAMESPACE,
                b"Value for attribute xmlns:%s of %s must be \"%s\"\n\0" as *const u8
                    as *const c_char,
                (*ns).prefix,
                (*elem).name,
                (*attrDecl).defaultValue,
            );
        } else {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_DTD_ELEM_NAMESPACE,
                b"Value for attribute xmlns of %s must be \"%s\"\n\0" as *const u8
                    as *const c_char,
                (*elem).name,
                (*attrDecl).defaultValue,
                ::core::ptr::null::<xmlChar>(),
            );
        }
        ret = 0 as c_int;
    }
    if !(*ns).prefix.is_null() {
        ret &= xmlValidateAttributeValue2(ctxt, doc, (*ns).prefix, (*attrDecl).atype, value);
    } else {
        ret &= xmlValidateAttributeValue2(
            ctxt,
            doc,
            b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            (*attrDecl).atype,
            value,
        );
    }
    return ret;
}
unsafe extern "C" fn xmlSnprintfElements(
    mut buf: *mut c_char,
    mut size: c_int,
    mut node: xmlNodePtr,
    mut glob: c_int,
) {
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut len: c_int = 0;
    if node.is_null() {
        return;
    }
    if glob != 0 {
        strcat(buf, b"(\0" as *const u8 as *const c_char);
    }
    cur = node;
    while !cur.is_null() {
        len = strlen(buf) as c_int;
        if size - len < 50 as c_int {
            if size - len > 4 as c_int
                && *buf.offset((len - 1 as c_int) as isize) as c_int
                    != '.' as i32
            {
                strcat(buf, b" ...\0" as *const u8 as *const c_char);
            }
            return;
        }
        let mut current_block_33: u64;
        match (*cur).type_0 as c_uint {
            1 => {
                if !(*cur).ns.is_null() && !(*(*cur).ns).prefix.is_null() {
                    if size - len < xmlStrlen((*(*cur).ns).prefix) + 10 as c_int {
                        if size - len > 4 as c_int
                            && *buf.offset((len - 1 as c_int) as isize)
                                as c_int
                                != '.' as i32
                        {
                            strcat(buf, b" ...\0" as *const u8 as *const c_char);
                        }
                        return;
                    }
                    strcat(buf, (*(*cur).ns).prefix as *mut c_char);
                    strcat(buf, b":\0" as *const u8 as *const c_char);
                }
                if size - len < xmlStrlen((*cur).name) + 10 as c_int {
                    if size - len > 4 as c_int
                        && *buf.offset((len - 1 as c_int) as isize)
                            as c_int
                            != '.' as i32
                    {
                        strcat(buf, b" ...\0" as *const u8 as *const c_char);
                    }
                    return;
                }
                strcat(buf, (*cur).name as *mut c_char);
                if !(*cur).next.is_null() {
                    strcat(buf, b" \0" as *const u8 as *const c_char);
                }
                current_block_33 = 7828949454673616476;
            }
            3 => {
                if xmlIsBlankNode(cur as *const xmlNode) != 0 {
                    current_block_33 = 7828949454673616476;
                } else {
                    current_block_33 = 4061920306801842036;
                }
            }
            4 | 5 => {
                current_block_33 = 4061920306801842036;
            }
            2 | 9 | 13 | 10 | 11 | 12 | 18 => {
                strcat(buf, b"???\0" as *const u8 as *const c_char);
                if !(*cur).next.is_null() {
                    strcat(buf, b" \0" as *const u8 as *const c_char);
                }
                current_block_33 = 7828949454673616476;
            }
            6 | 7 | 14 | 8 | 15 | 16 | 17 | 19 | 20 | _ => {
                current_block_33 = 7828949454673616476;
            }
        }
        match current_block_33 {
            4061920306801842036 => {
                strcat(buf, b"CDATA\0" as *const u8 as *const c_char);
                if !(*cur).next.is_null() {
                    strcat(buf, b" \0" as *const u8 as *const c_char);
                }
            }
            _ => {}
        }
        cur = (*cur).next as xmlNodePtr;
    }
    if glob != 0 {
        strcat(buf, b")\0" as *const u8 as *const c_char);
    }
}
unsafe extern "C" fn xmlValidateElementContent(
    mut ctxt: xmlValidCtxtPtr,
    mut child: xmlNodePtr,
    mut elemDecl: xmlElementPtr,
    mut warn: c_int,
    mut parent: xmlNodePtr,
) -> c_int {
    let mut current_block: u64;
    let mut ret: c_int = 1 as c_int;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut cont: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if elemDecl.is_null() || parent.is_null() || ctxt.is_null() {
        return -(1 as c_int);
    }
    cont = (*elemDecl).content;
    name = (*elemDecl).name;
    if (*elemDecl).contModel.is_null() {
        ret = xmlValidBuildContentModel(ctxt, elemDecl);
    }
    if (*elemDecl).contModel.is_null() {
        return -(1 as c_int);
    } else {
        let mut exec: xmlRegExecCtxtPtr = ::core::ptr::null_mut::<xmlRegExecCtxt>();
        if xmlRegexpIsDeterminist((*elemDecl).contModel) == 0 {
            return -(1 as c_int);
        }
        (*ctxt).nodeMax = 0 as c_int;
        (*ctxt).nodeNr = 0 as c_int;
        (*ctxt).nodeTab = ::core::ptr::null_mut::<xmlNodePtr>();
        exec = xmlRegNewExecCtxt((*elemDecl).contModel, None, NULL);
        if !exec.is_null() {
            cur = child;
            loop {
                if cur.is_null() {
                    current_block = 14832935472441733737;
                    break;
                }
                match (*cur).type_0 as c_uint {
                    5 => {
                        if !(*cur).children.is_null() && !(*(*cur).children).children.is_null() {
                            nodeVPush(ctxt, cur);
                            cur = (*(*cur).children).children as xmlNodePtr;
                            continue;
                        }
                    }
                    3 => {
                        if !(xmlIsBlankNode(cur as *const xmlNode) != 0) {
                            ret = 0 as c_int;
                            current_block = 17486495861892298170;
                            break;
                        }
                    }
                    4 => {
                        ret = 0 as c_int;
                        current_block = 17486495861892298170;
                        break;
                    }
                    1 => {
                        if !(*cur).ns.is_null() && !(*(*cur).ns).prefix.is_null() {
                            let mut fn_0: [xmlChar; 50] = [0; 50];
                            let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                            fullname = xmlBuildQName(
                                (*cur).name,
                                (*(*cur).ns).prefix,
                                &raw mut fn_0 as *mut xmlChar,
                                50 as c_int,
                            );
                            if fullname.is_null() {
                                ret = -(1 as c_int);
                                current_block = 17486495861892298170;
                                break;
                            } else {
                                ret = xmlRegExecPushString(exec, fullname, NULL);
                                if fullname != &raw mut fn_0 as *mut xmlChar
                                    && fullname != (*cur).name as *mut xmlChar
                                {
                                    xmlFree.expect("non-null function pointer")(
                                        fullname as *mut c_void,
                                    );
                                }
                            }
                        } else {
                            ret = xmlRegExecPushString(exec, (*cur).name, NULL);
                        }
                    }
                    _ => {}
                }
                cur = (*cur).next as xmlNodePtr;
                while cur.is_null() {
                    cur = nodeVPop(ctxt);
                    if cur.is_null() {
                        break;
                    }
                    cur = (*cur).next as xmlNodePtr;
                }
            }
            match current_block {
                14832935472441733737 => {
                    ret = xmlRegExecPushString(exec, ::core::ptr::null::<xmlChar>(), NULL);
                }
                _ => {}
            }
            xmlRegFreeExecCtxt(exec);
        }
        if warn != 0 && (ret != 1 as c_int && ret != -(3 as c_int)) {
            if !ctxt.is_null() {
                let mut expr: [c_char; 5000] = [0; 5000];
                let mut list: [c_char; 5000] = [0; 5000];
                expr[0 as c_int as usize] = 0 as c_char;
                xmlSnprintfElementContent(
                    (&raw mut expr as *mut c_char)
                        .offset(0 as c_int as isize)
                        as *mut c_char,
                    5000 as c_int,
                    cont,
                    1 as c_int,
                );
                list[0 as c_int as usize] = 0 as c_char;
                xmlSnprintfElements(
                    (&raw mut list as *mut c_char)
                        .offset(0 as c_int as isize)
                        as *mut c_char,
                    5000 as c_int,
                    child,
                    1 as c_int,
                );
                if !name.is_null() {
                    xmlErrValidNode(
                        ctxt,
                        parent,
                        XML_DTD_CONTENT_MODEL,
                        b"Element %s content does not follow the DTD, expecting %s, got %s\n\0"
                            as *const u8 as *const c_char,
                        name,
                        &raw mut expr as *mut c_char as *mut xmlChar,
                        &raw mut list as *mut c_char as *mut xmlChar,
                    );
                } else {
                    xmlErrValidNode(
                        ctxt,
                        parent,
                        XML_DTD_CONTENT_MODEL,
                        b"Element content does not follow the DTD, expecting %s, got %s\n\0"
                            as *const u8 as *const c_char,
                        &raw mut expr as *mut c_char as *mut xmlChar,
                        &raw mut list as *mut c_char as *mut xmlChar,
                        ::core::ptr::null::<xmlChar>(),
                    );
                }
            } else if !name.is_null() {
                xmlErrValidNode(
                    ctxt,
                    parent,
                    XML_DTD_CONTENT_MODEL,
                    b"Element %s content does not follow the DTD\n\0" as *const u8
                        as *const c_char,
                    name,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
            } else {
                xmlErrValidNode(
                    ctxt,
                    parent,
                    XML_DTD_CONTENT_MODEL,
                    b"Element content does not follow the DTD\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            ret = 0 as c_int;
        }
        if ret == -(3 as c_int) {
            ret = 1 as c_int;
        }
        (*ctxt).nodeMax = 0 as c_int;
        (*ctxt).nodeNr = 0 as c_int;
        if !(*ctxt).nodeTab.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*ctxt).nodeTab as *mut c_void,
            );
            (*ctxt).nodeTab = ::core::ptr::null_mut::<xmlNodePtr>();
        }
        return ret;
    };
}
unsafe extern "C" fn xmlValidateOneCdataElement(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
) -> c_int {
    let mut ret: c_int = 1 as c_int;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut child: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if ctxt.is_null()
        || doc.is_null()
        || elem.is_null()
        || (*elem).type_0 as c_uint
            != XML_ELEMENT_NODE as c_int as c_uint
    {
        return 0 as c_int;
    }
    child = (*elem).children as xmlNodePtr;
    cur = child;
    while !cur.is_null() {
        match (*cur).type_0 as c_uint {
            5 => {
                if !(*cur).children.is_null() && !(*(*cur).children).children.is_null() {
                    nodeVPush(ctxt, cur);
                    cur = (*(*cur).children).children as xmlNodePtr;
                    continue;
                }
            }
            8 | 7 | 3 | 4 => {}
            _ => {
                ret = 0 as c_int;
                break;
            }
        }
        cur = (*cur).next as xmlNodePtr;
        while cur.is_null() {
            cur = nodeVPop(ctxt);
            if cur.is_null() {
                break;
            }
            cur = (*cur).next as xmlNodePtr;
        }
    }
    (*ctxt).nodeMax = 0 as c_int;
    (*ctxt).nodeNr = 0 as c_int;
    if !(*ctxt).nodeTab.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).nodeTab as *mut c_void);
        (*ctxt).nodeTab = ::core::ptr::null_mut::<xmlNodePtr>();
    }
    return ret;
}
unsafe extern "C" fn xmlValidateCheckMixed(
    mut ctxt: xmlValidCtxtPtr,
    mut cont: xmlElementContentPtr,
    mut qname: *const xmlChar,
) -> c_int {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut plen: c_int = 0;
    name = xmlSplitQName3(qname, &raw mut plen);
    if name.is_null() {
        while !cont.is_null() {
            if (*cont).type_0 as c_uint
                == XML_ELEMENT_CONTENT_ELEMENT as c_int as c_uint
            {
                if (*cont).prefix.is_null() && xmlStrEqual((*cont).name, qname) != 0 {
                    return 1 as c_int;
                }
            } else if (*cont).type_0 as c_uint
                == XML_ELEMENT_CONTENT_OR as c_int as c_uint
                && !(*cont).c1.is_null()
                && (*(*cont).c1).type_0 as c_uint
                    == XML_ELEMENT_CONTENT_ELEMENT as c_int as c_uint
            {
                if (*(*cont).c1).prefix.is_null() && xmlStrEqual((*(*cont).c1).name, qname) != 0 {
                    return 1 as c_int;
                }
            } else if (*cont).type_0 as c_uint
                != XML_ELEMENT_CONTENT_OR as c_int as c_uint
                || (*cont).c1.is_null()
                || (*(*cont).c1).type_0 as c_uint
                    != XML_ELEMENT_CONTENT_PCDATA as c_int as c_uint
            {
                xmlErrValid(
                    ::core::ptr::null_mut::<xmlValidCtxt>(),
                    XML_DTD_MIXED_CORRUPT,
                    b"Internal: MIXED struct corrupted\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
                break;
            }
            cont = (*cont).c2 as xmlElementContentPtr;
        }
    } else {
        while !cont.is_null() {
            if (*cont).type_0 as c_uint
                == XML_ELEMENT_CONTENT_ELEMENT as c_int as c_uint
            {
                if !(*cont).prefix.is_null()
                    && xmlStrncmp((*cont).prefix, qname, plen) == 0 as c_int
                    && xmlStrEqual((*cont).name, name) != 0
                {
                    return 1 as c_int;
                }
            } else if (*cont).type_0 as c_uint
                == XML_ELEMENT_CONTENT_OR as c_int as c_uint
                && !(*cont).c1.is_null()
                && (*(*cont).c1).type_0 as c_uint
                    == XML_ELEMENT_CONTENT_ELEMENT as c_int as c_uint
            {
                if !(*(*cont).c1).prefix.is_null()
                    && xmlStrncmp((*(*cont).c1).prefix, qname, plen) == 0 as c_int
                    && xmlStrEqual((*(*cont).c1).name, name) != 0
                {
                    return 1 as c_int;
                }
            } else if (*cont).type_0 as c_uint
                != XML_ELEMENT_CONTENT_OR as c_int as c_uint
                || (*cont).c1.is_null()
                || (*(*cont).c1).type_0 as c_uint
                    != XML_ELEMENT_CONTENT_PCDATA as c_int as c_uint
            {
                xmlErrValid(
                    ctxt,
                    XML_DTD_MIXED_CORRUPT,
                    b"Internal: MIXED struct corrupted\n\0" as *const u8
                        as *const c_char,
                    ::core::ptr::null::<c_char>(),
                );
                break;
            }
            cont = (*cont).c2 as xmlElementContentPtr;
        }
    }
    return 0 as c_int;
}
unsafe extern "C" fn xmlValidGetElemDecl(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut extsubset: *mut c_int,
) -> xmlElementPtr {
    let mut elemDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    let mut prefix: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if ctxt.is_null() || doc.is_null() || elem.is_null() || (*elem).name.is_null() {
        return ::core::ptr::null_mut::<xmlElement>();
    }
    if !extsubset.is_null() {
        *extsubset = 0 as c_int;
    }
    if !(*elem).ns.is_null() && !(*(*elem).ns).prefix.is_null() {
        prefix = (*(*elem).ns).prefix;
    }
    if !prefix.is_null() {
        elemDecl = xmlGetDtdQElementDesc((*doc).intSubset as xmlDtdPtr, (*elem).name, prefix);
        if elemDecl.is_null() && !(*doc).extSubset.is_null() {
            elemDecl = xmlGetDtdQElementDesc((*doc).extSubset as xmlDtdPtr, (*elem).name, prefix);
            if !elemDecl.is_null() && !extsubset.is_null() {
                *extsubset = 1 as c_int;
            }
        }
    }
    if elemDecl.is_null() {
        elemDecl = xmlGetDtdElementDesc((*doc).intSubset as xmlDtdPtr, (*elem).name);
        if elemDecl.is_null() && !(*doc).extSubset.is_null() {
            elemDecl = xmlGetDtdElementDesc((*doc).extSubset as xmlDtdPtr, (*elem).name);
            if !elemDecl.is_null() && !extsubset.is_null() {
                *extsubset = 1 as c_int;
            }
        }
    }
    if elemDecl.is_null() {
        xmlErrValidNode(
            ctxt,
            elem,
            XML_DTD_UNKNOWN_ELEM,
            b"No declaration for element %s\n\0" as *const u8 as *const c_char,
            (*elem).name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        );
    }
    return elemDecl;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidatePushElement(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut qname: *const xmlChar,
) -> c_int {
    let mut ret: c_int = 1 as c_int;
    let mut eDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    let mut extsubset: c_int = 0 as c_int;
    if ctxt.is_null() {
        return 0 as c_int;
    }
    if (*ctxt).vstateNr > 0 as c_int && !(*ctxt).vstate.is_null() {
        let mut state: xmlValidStatePtr = (*ctxt).vstate as xmlValidStatePtr;
        let mut elemDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
        if !(*state).elemDecl.is_null() {
            elemDecl = (*state).elemDecl;
            match (*elemDecl).etype as c_uint {
                0 => {
                    ret = 0 as c_int;
                }
                1 => {
                    xmlErrValidNode(
                        ctxt,
                        (*state).node,
                        XML_DTD_NOT_EMPTY,
                        b"Element %s was declared EMPTY this one has content\n\0" as *const u8
                            as *const c_char,
                        (*(*state).node).name,
                        ::core::ptr::null::<xmlChar>(),
                        ::core::ptr::null::<xmlChar>(),
                    );
                    ret = 0 as c_int;
                }
                3 => {
                    if !(*elemDecl).content.is_null()
                        && (*(*elemDecl).content).type_0 as c_uint
                            == XML_ELEMENT_CONTENT_PCDATA as c_int
                                as c_uint
                    {
                        xmlErrValidNode(
                            ctxt,
                            (*state).node,
                            XML_DTD_NOT_PCDATA,
                            b"Element %s was declared #PCDATA but contains non text nodes\n\0"
                                as *const u8
                                as *const c_char,
                            (*(*state).node).name,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                        );
                        ret = 0 as c_int;
                    } else {
                        ret = xmlValidateCheckMixed(ctxt, (*elemDecl).content, qname);
                        if ret != 1 as c_int {
                            xmlErrValidNode(
                                ctxt,
                                (*state).node,
                                XML_DTD_INVALID_CHILD,
                                b"Element %s is not declared in %s list of possible children\n\0"
                                    as *const u8
                                    as *const c_char,
                                qname,
                                (*(*state).node).name,
                                ::core::ptr::null::<xmlChar>(),
                            );
                        }
                    }
                }
                4 => {
                    if !(*state).exec.is_null() {
                        ret = xmlRegExecPushString((*state).exec, qname, NULL);
                        if ret < 0 as c_int {
                            xmlErrValidNode(
                                ctxt,
                                (*state).node,
                                XML_DTD_CONTENT_MODEL,
                                b"Element %s content does not follow the DTD, Misplaced %s\n\0"
                                    as *const u8
                                    as *const c_char,
                                (*(*state).node).name,
                                qname,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            ret = 0 as c_int;
                        } else {
                            ret = 1 as c_int;
                        }
                    }
                }
                2 | _ => {}
            }
        }
    }
    eDecl = xmlValidGetElemDecl(ctxt, doc, elem, &raw mut extsubset);
    vstateVPush(ctxt, eDecl, elem);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidatePushCData(
    mut ctxt: xmlValidCtxtPtr,
    mut data: *const xmlChar,
    mut len: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut ret: c_int = 1 as c_int;
    if ctxt.is_null() {
        return 0 as c_int;
    }
    if len <= 0 as c_int {
        return ret;
    }
    if (*ctxt).vstateNr > 0 as c_int && !(*ctxt).vstate.is_null() {
        let mut state: xmlValidStatePtr = (*ctxt).vstate as xmlValidStatePtr;
        let mut elemDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
        if !(*state).elemDecl.is_null() {
            elemDecl = (*state).elemDecl;
            match (*elemDecl).etype as c_uint {
                0 => {
                    current_block = 4496743486339921000;
                    match current_block {
                        4496743486339921000 => {
                            ret = 0 as c_int;
                        }
                        13536709405535804910 => {
                            let mut i: c_int = 0;
                            i = 0 as c_int;
                            while i < len {
                                if !(*data.offset(i as isize) as c_int
                                    == 0x20 as c_int
                                    || 0x9 as c_int
                                        <= *data.offset(i as isize) as c_int
                                        && *data.offset(i as isize) as c_int
                                            <= 0xa as c_int
                                    || *data.offset(i as isize) as c_int
                                        == 0xd as c_int)
                                {
                                    xmlErrValidNode(
                                        ctxt,
                                        (*state).node,
                                        XML_DTD_CONTENT_MODEL,
                                        b"Element %s content does not follow the DTD, Text not allowed\n\0"
                                            as *const u8 as *const c_char,
                                        (*(*state).node).name,
                                        ::core::ptr::null::<xmlChar>(),
                                        ::core::ptr::null::<xmlChar>(),
                                    );
                                    ret = 0 as c_int;
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                        }
                        _ => {
                            xmlErrValidNode(
                                ctxt,
                                (*state).node,
                                XML_DTD_NOT_EMPTY,
                                b"Element %s was declared EMPTY this one has content\n\0"
                                    as *const u8
                                    as *const c_char,
                                (*(*state).node).name,
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                            );
                            ret = 0 as c_int;
                        }
                    }
                }
                1 => {
                    current_block = 4720854468459483876;
                    match current_block {
                        4496743486339921000 => {
                            ret = 0 as c_int;
                        }
                        13536709405535804910 => {
                            let mut i: c_int = 0;
                            i = 0 as c_int;
                            while i < len {
                                if !(*data.offset(i as isize) as c_int
                                    == 0x20 as c_int
                                    || 0x9 as c_int
                                        <= *data.offset(i as isize) as c_int
                                        && *data.offset(i as isize) as c_int
                                            <= 0xa as c_int
                                    || *data.offset(i as isize) as c_int
                                        == 0xd as c_int)
                                {
                                    xmlErrValidNode(
                                        ctxt,
                                        (*state).node,
                                        XML_DTD_CONTENT_MODEL,
                                        b"Element %s content does not follow the DTD, Text not allowed\n\0"
                                            as *const u8 as *const c_char,
                                        (*(*state).node).name,
                                        ::core::ptr::null::<xmlChar>(),
                                        ::core::ptr::null::<xmlChar>(),
                                    );
                                    ret = 0 as c_int;
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                        }
                        _ => {
                            xmlErrValidNode(
                                ctxt,
                                (*state).node,
                                XML_DTD_NOT_EMPTY,
                                b"Element %s was declared EMPTY this one has content\n\0"
                                    as *const u8
                                    as *const c_char,
                                (*(*state).node).name,
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                            );
                            ret = 0 as c_int;
                        }
                    }
                }
                4 => {
                    current_block = 13536709405535804910;
                    match current_block {
                        4496743486339921000 => {
                            ret = 0 as c_int;
                        }
                        13536709405535804910 => {
                            let mut i: c_int = 0;
                            i = 0 as c_int;
                            while i < len {
                                if !(*data.offset(i as isize) as c_int
                                    == 0x20 as c_int
                                    || 0x9 as c_int
                                        <= *data.offset(i as isize) as c_int
                                        && *data.offset(i as isize) as c_int
                                            <= 0xa as c_int
                                    || *data.offset(i as isize) as c_int
                                        == 0xd as c_int)
                                {
                                    xmlErrValidNode(
                                        ctxt,
                                        (*state).node,
                                        XML_DTD_CONTENT_MODEL,
                                        b"Element %s content does not follow the DTD, Text not allowed\n\0"
                                            as *const u8 as *const c_char,
                                        (*(*state).node).name,
                                        ::core::ptr::null::<xmlChar>(),
                                        ::core::ptr::null::<xmlChar>(),
                                    );
                                    ret = 0 as c_int;
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                        }
                        _ => {
                            xmlErrValidNode(
                                ctxt,
                                (*state).node,
                                XML_DTD_NOT_EMPTY,
                                b"Element %s was declared EMPTY this one has content\n\0"
                                    as *const u8
                                    as *const c_char,
                                (*(*state).node).name,
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                            );
                            ret = 0 as c_int;
                        }
                    }
                }
                2 | 3 | _ => {}
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidatePopElement(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
    mut qname: *const xmlChar,
) -> c_int {
    let mut ret: c_int = 1 as c_int;
    if ctxt.is_null() {
        return 0 as c_int;
    }
    if (*ctxt).vstateNr > 0 as c_int && !(*ctxt).vstate.is_null() {
        let mut state: xmlValidStatePtr = (*ctxt).vstate as xmlValidStatePtr;
        let mut elemDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
        if !(*state).elemDecl.is_null() {
            elemDecl = (*state).elemDecl;
            if (*elemDecl).etype as c_uint
                == XML_ELEMENT_TYPE_ELEMENT as c_int as c_uint
            {
                if !(*state).exec.is_null() {
                    ret = xmlRegExecPushString((*state).exec, ::core::ptr::null::<xmlChar>(), NULL);
                    if ret <= 0 as c_int {
                        xmlErrValidNode(
                            ctxt,
                            (*state).node,
                            XML_DTD_CONTENT_MODEL,
                            b"Element %s content does not follow the DTD, Expecting more children\n\0"
                                as *const u8 as *const c_char,
                            (*(*state).node).name,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                        );
                        ret = 0 as c_int;
                    } else {
                        ret = 1 as c_int;
                    }
                }
            }
        }
        vstateVPop(ctxt);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateOneElement(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
) -> c_int {
    let mut current_block: u64;
    let mut elemDecl: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    let mut cont: xmlElementContentPtr = ::core::ptr::null_mut::<xmlElementContent>();
    let mut attr: xmlAttributePtr = ::core::ptr::null_mut::<xmlAttribute>();
    let mut child: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: c_int = 1 as c_int;
    let mut tmp: c_int = 0;
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut extsubset: c_int = 0 as c_int;
    if doc.is_null() {
        return 0 as c_int;
    } else if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as c_int;
    }
    if elem.is_null() {
        return 0 as c_int;
    }
    match (*elem).type_0 as c_uint {
        2 => {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_ERR_INTERNAL_ERROR,
                b"Attribute element not expected\n\0" as *const u8 as *const c_char,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            return 0 as c_int;
        }
        3 => {
            if !(*elem).children.is_null() {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_ERR_INTERNAL_ERROR,
                    b"Text element has children !\n\0" as *const u8 as *const c_char,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
                return 0 as c_int;
            }
            if !(*elem).ns.is_null() {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_ERR_INTERNAL_ERROR,
                    b"Text element has namespace !\n\0" as *const u8 as *const c_char,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
                return 0 as c_int;
            }
            if (*elem).content.is_null() {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_ERR_INTERNAL_ERROR,
                    b"Text element has no content !\n\0" as *const u8 as *const c_char,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
                return 0 as c_int;
            }
            return 1 as c_int;
        }
        19 | 20 => return 1 as c_int,
        4 | 5 | 7 | 8 => return 1 as c_int,
        6 => {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_ERR_INTERNAL_ERROR,
                b"Entity element not expected\n\0" as *const u8 as *const c_char,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            return 0 as c_int;
        }
        12 => {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_ERR_INTERNAL_ERROR,
                b"Notation element not expected\n\0" as *const u8 as *const c_char,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            return 0 as c_int;
        }
        9 | 10 | 11 => {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_ERR_INTERNAL_ERROR,
                b"Document element not expected\n\0" as *const u8 as *const c_char,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            return 0 as c_int;
        }
        13 => {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_ERR_INTERNAL_ERROR,
                b"HTML Document not expected\n\0" as *const u8 as *const c_char,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            return 0 as c_int;
        }
        1 => {}
        _ => {
            xmlErrValidNode(
                ctxt,
                elem,
                XML_ERR_INTERNAL_ERROR,
                b"unknown element type\n\0" as *const u8 as *const c_char,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            return 0 as c_int;
        }
    }
    elemDecl = xmlValidGetElemDecl(ctxt, doc, elem, &raw mut extsubset);
    if elemDecl.is_null() {
        return 0 as c_int;
    }
    if (*ctxt).vstateNr == 0 as c_int {
        match (*elemDecl).etype as c_uint {
            0 => {
                xmlErrValidNode(
                    ctxt,
                    elem,
                    XML_DTD_UNKNOWN_ELEM,
                    b"No declaration for element %s\n\0" as *const u8 as *const c_char,
                    (*elem).name,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                );
                return 0 as c_int;
            }
            1 => {
                if !(*elem).children.is_null() {
                    xmlErrValidNode(
                        ctxt,
                        elem,
                        XML_DTD_NOT_EMPTY,
                        b"Element %s was declared EMPTY this one has content\n\0" as *const u8
                            as *const c_char,
                        (*elem).name,
                        ::core::ptr::null::<xmlChar>(),
                        ::core::ptr::null::<xmlChar>(),
                    );
                    ret = 0 as c_int;
                }
            }
            3 => {
                if !(*elemDecl).content.is_null()
                    && (*(*elemDecl).content).type_0 as c_uint
                        == XML_ELEMENT_CONTENT_PCDATA as c_int as c_uint
                {
                    ret = xmlValidateOneCdataElement(ctxt, doc, elem);
                    if ret == 0 {
                        xmlErrValidNode(
                            ctxt,
                            elem,
                            XML_DTD_NOT_PCDATA,
                            b"Element %s was declared #PCDATA but contains non text nodes\n\0"
                                as *const u8
                                as *const c_char,
                            (*elem).name,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                        );
                    }
                } else {
                    child = (*elem).children as xmlNodePtr;
                    while !child.is_null() {
                        let mut current_block_66: u64;
                        if (*child).type_0 as c_uint
                            == XML_ELEMENT_NODE as c_int as c_uint
                        {
                            name = (*child).name;
                            if !(*child).ns.is_null() && !(*(*child).ns).prefix.is_null() {
                                let mut fn_0: [xmlChar; 50] = [0; 50];
                                let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                                fullname = xmlBuildQName(
                                    (*child).name,
                                    (*(*child).ns).prefix,
                                    &raw mut fn_0 as *mut xmlChar,
                                    50 as c_int,
                                );
                                if fullname.is_null() {
                                    return 0 as c_int;
                                }
                                cont = (*elemDecl).content;
                                while !cont.is_null() {
                                    if (*cont).type_0 as c_uint
                                        == XML_ELEMENT_CONTENT_ELEMENT as c_int
                                            as c_uint
                                    {
                                        if xmlStrEqual((*cont).name, fullname) != 0 {
                                            break;
                                        }
                                    } else if (*cont).type_0 as c_uint
                                        == XML_ELEMENT_CONTENT_OR as c_int
                                            as c_uint
                                        && !(*cont).c1.is_null()
                                        && (*(*cont).c1).type_0 as c_uint
                                            == XML_ELEMENT_CONTENT_ELEMENT as c_int
                                                as c_uint
                                    {
                                        if xmlStrEqual((*(*cont).c1).name, fullname) != 0 {
                                            break;
                                        }
                                    } else if (*cont).type_0 as c_uint
                                        != XML_ELEMENT_CONTENT_OR as c_int
                                            as c_uint
                                        || (*cont).c1.is_null()
                                        || (*(*cont).c1).type_0 as c_uint
                                            != XML_ELEMENT_CONTENT_PCDATA as c_int
                                                as c_uint
                                    {
                                        xmlErrValid(
                                            ::core::ptr::null_mut::<xmlValidCtxt>(),
                                            XML_DTD_MIXED_CORRUPT,
                                            b"Internal: MIXED struct corrupted\n\0" as *const u8
                                                as *const c_char,
                                            ::core::ptr::null::<c_char>(),
                                        );
                                        break;
                                    }
                                    cont = (*cont).c2 as xmlElementContentPtr;
                                }
                                if fullname != &raw mut fn_0 as *mut xmlChar
                                    && fullname != (*child).name as *mut xmlChar
                                {
                                    xmlFree.expect("non-null function pointer")(
                                        fullname as *mut c_void,
                                    );
                                }
                                if !cont.is_null() {
                                    current_block_66 = 3237041207979566912;
                                } else {
                                    current_block_66 = 3580086814630675314;
                                }
                            } else {
                                current_block_66 = 3580086814630675314;
                            }
                            match current_block_66 {
                                3237041207979566912 => {}
                                _ => {
                                    cont = (*elemDecl).content;
                                    while !cont.is_null() {
                                        if (*cont).type_0 as c_uint
                                            == XML_ELEMENT_CONTENT_ELEMENT as c_int
                                                as c_uint
                                        {
                                            if xmlStrEqual((*cont).name, name) != 0 {
                                                break;
                                            }
                                        } else if (*cont).type_0 as c_uint
                                            == XML_ELEMENT_CONTENT_OR as c_int
                                                as c_uint
                                            && !(*cont).c1.is_null()
                                            && (*(*cont).c1).type_0 as c_uint
                                                == XML_ELEMENT_CONTENT_ELEMENT as c_int
                                                    as c_uint
                                        {
                                            if xmlStrEqual((*(*cont).c1).name, name) != 0 {
                                                break;
                                            }
                                        } else if (*cont).type_0 as c_uint
                                            != XML_ELEMENT_CONTENT_OR as c_int
                                                as c_uint
                                            || (*cont).c1.is_null()
                                            || (*(*cont).c1).type_0 as c_uint
                                                != XML_ELEMENT_CONTENT_PCDATA as c_int
                                                    as c_uint
                                        {
                                            xmlErrValid(
                                                ctxt,
                                                XML_DTD_MIXED_CORRUPT,
                                                b"Internal: MIXED struct corrupted\n\0" as *const u8
                                                    as *const c_char,
                                                ::core::ptr::null::<c_char>(),
                                            );
                                            break;
                                        }
                                        cont = (*cont).c2 as xmlElementContentPtr;
                                    }
                                    if cont.is_null() {
                                        xmlErrValidNode(
                                            ctxt,
                                            elem,
                                            XML_DTD_INVALID_CHILD,
                                            b"Element %s is not declared in %s list of possible children\n\0"
                                                as *const u8 as *const c_char,
                                            name,
                                            (*elem).name,
                                            ::core::ptr::null::<xmlChar>(),
                                        );
                                        ret = 0 as c_int;
                                    }
                                }
                            }
                        }
                        child = (*child).next as xmlNodePtr;
                    }
                }
            }
            4 => {
                if (*doc).standalone == 1 as c_int
                    && extsubset == 1 as c_int
                {
                    child = (*elem).children as xmlNodePtr;
                    while !child.is_null() {
                        if (*child).type_0 as c_uint
                            == XML_TEXT_NODE as c_int as c_uint
                        {
                            let mut content: *const xmlChar = (*child).content;
                            while *content as c_int == 0x20 as c_int
                                || 0x9 as c_int <= *content as c_int
                                    && *content as c_int <= 0xa as c_int
                                || *content as c_int == 0xd as c_int
                            {
                                content = content.offset(1);
                            }
                            if *content as c_int == 0 as c_int {
                                xmlErrValidNode(
                                    ctxt,
                                    elem,
                                    XML_DTD_STANDALONE_WHITE_SPACE,
                                    b"standalone: %s declared in the external subset contains white spaces nodes\n\0"
                                        as *const u8 as *const c_char,
                                    (*elem).name,
                                    ::core::ptr::null::<xmlChar>(),
                                    ::core::ptr::null::<xmlChar>(),
                                );
                                ret = 0 as c_int;
                                break;
                            }
                        }
                        child = (*child).next as xmlNodePtr;
                    }
                }
                child = (*elem).children as xmlNodePtr;
                cont = (*elemDecl).content;
                tmp =
                    xmlValidateElementContent(ctxt, child, elemDecl, 1 as c_int, elem);
                if tmp <= 0 as c_int {
                    ret = tmp;
                }
            }
            2 | _ => {}
        }
    }
    attr = (*elemDecl).attributes;
    while !attr.is_null() {
        if (*attr).def as c_uint
            == XML_ATTRIBUTE_REQUIRED as c_int as c_uint
        {
            let mut qualified: c_int = -(1 as c_int);
            if (*attr).prefix.is_null()
                && xmlStrEqual(
                    (*attr).name,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                ) != 0
            {
                let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                ns = (*elem).nsDef as xmlNsPtr;
                loop {
                    if ns.is_null() {
                        current_block = 17336970397495664729;
                        break;
                    }
                    if (*ns).prefix.is_null() {
                        current_block = 1543413795638927773;
                        break;
                    }
                    ns = (*ns).next as xmlNsPtr;
                }
            } else if xmlStrEqual(
                (*attr).prefix,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                let mut ns_0: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                ns_0 = (*elem).nsDef as xmlNsPtr;
                loop {
                    if ns_0.is_null() {
                        current_block = 17336970397495664729;
                        break;
                    }
                    if xmlStrEqual((*attr).name, (*ns_0).prefix) != 0 {
                        current_block = 1543413795638927773;
                        break;
                    }
                    ns_0 = (*ns_0).next as xmlNsPtr;
                }
            } else {
                let mut attrib: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
                attrib = (*elem).properties as xmlAttrPtr;
                loop {
                    if attrib.is_null() {
                        current_block = 17336970397495664729;
                        break;
                    }
                    if xmlStrEqual((*attrib).name, (*attr).name) != 0 {
                        if (*attr).prefix.is_null() {
                            current_block = 1543413795638927773;
                            break;
                        }
                        let mut nameSpace: xmlNsPtr = (*attrib).ns as xmlNsPtr;
                        if nameSpace.is_null() {
                            nameSpace = (*elem).ns as xmlNsPtr;
                        }
                        if nameSpace.is_null() {
                            if qualified < 0 as c_int {
                                qualified = 0 as c_int;
                            }
                        } else {
                            if !(xmlStrEqual((*nameSpace).prefix, (*attr).prefix) == 0) {
                                current_block = 1543413795638927773;
                                break;
                            }
                            if qualified < 1 as c_int {
                                qualified = 1 as c_int;
                            }
                        }
                    }
                    attrib = (*attrib).next as xmlAttrPtr;
                }
            }
            match current_block {
                1543413795638927773 => {}
                _ => {
                    if qualified == -(1 as c_int) {
                        if (*attr).prefix.is_null() {
                            xmlErrValidNode(
                                ctxt,
                                elem,
                                XML_DTD_MISSING_ATTRIBUTE,
                                b"Element %s does not carry attribute %s\n\0" as *const u8
                                    as *const c_char,
                                (*elem).name,
                                (*attr).name,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            ret = 0 as c_int;
                        } else {
                            xmlErrValidNode(
                                ctxt,
                                elem,
                                XML_DTD_MISSING_ATTRIBUTE,
                                b"Element %s does not carry attribute %s:%s\n\0" as *const u8
                                    as *const c_char,
                                (*elem).name,
                                (*attr).prefix,
                                (*attr).name,
                            );
                            ret = 0 as c_int;
                        }
                    } else if qualified == 0 as c_int {
                        xmlErrValidWarning(
                            ctxt,
                            elem,
                            XML_DTD_NO_PREFIX,
                            b"Element %s required attribute %s:%s has no prefix\n\0" as *const u8
                                as *const c_char,
                            (*elem).name,
                            (*attr).prefix,
                            (*attr).name,
                        );
                    } else if qualified == 1 as c_int {
                        xmlErrValidWarning(
                            ctxt,
                            elem,
                            XML_DTD_DIFFERENT_PREFIX,
                            b"Element %s required attribute %s:%s has different prefix\n\0"
                                as *const u8
                                as *const c_char,
                            (*elem).name,
                            (*attr).prefix,
                            (*attr).name,
                        );
                    }
                }
            }
        } else if (*attr).def as c_uint
            == XML_ATTRIBUTE_FIXED as c_int as c_uint
        {
            if (*attr).prefix.is_null()
                && xmlStrEqual(
                    (*attr).name,
                    b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
                ) != 0
            {
                let mut ns_1: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                ns_1 = (*elem).nsDef as xmlNsPtr;
                while !ns_1.is_null() {
                    if (*ns_1).prefix.is_null() {
                        if xmlStrEqual((*attr).defaultValue, (*ns_1).href) == 0 {
                            xmlErrValidNode(
                                ctxt,
                                elem,
                                XML_DTD_ELEM_DEFAULT_NAMESPACE,
                                b"Element %s namespace name for default namespace does not match the DTD\n\0"
                                    as *const u8 as *const c_char,
                                (*elem).name,
                                ::core::ptr::null::<xmlChar>(),
                                ::core::ptr::null::<xmlChar>(),
                            );
                            ret = 0 as c_int;
                        }
                        break;
                    } else {
                        ns_1 = (*ns_1).next as xmlNsPtr;
                    }
                }
            } else if xmlStrEqual(
                (*attr).prefix,
                b"xmlns\0" as *const u8 as *const c_char as *mut xmlChar,
            ) != 0
            {
                let mut ns_2: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
                ns_2 = (*elem).nsDef as xmlNsPtr;
                while !ns_2.is_null() {
                    if xmlStrEqual((*attr).name, (*ns_2).prefix) != 0 {
                        if xmlStrEqual((*attr).defaultValue, (*ns_2).href) == 0 {
                            xmlErrValidNode(
                                ctxt,
                                elem,
                                XML_DTD_ELEM_NAMESPACE,
                                b"Element %s namespace name for %s does not match the DTD\n\0"
                                    as *const u8
                                    as *const c_char,
                                (*elem).name,
                                (*ns_2).prefix,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            ret = 0 as c_int;
                        }
                        break;
                    } else {
                        ns_2 = (*ns_2).next as xmlNsPtr;
                    }
                }
            }
        }
        attr = (*attr).nexth as xmlAttributePtr;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateRoot(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
) -> c_int {
    let mut current_block: u64;
    let mut root: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ret: c_int = 0;
    if doc.is_null() {
        return 0 as c_int;
    }
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() || (*root).name.is_null() {
        xmlErrValid(
            ctxt,
            XML_DTD_NO_ROOT,
            b"no root element\n\0" as *const u8 as *const c_char,
            ::core::ptr::null::<c_char>(),
        );
        return 0 as c_int;
    }
    if !(*doc).intSubset.is_null() && !(*(*doc).intSubset).name.is_null() {
        if xmlStrEqual((*(*doc).intSubset).name, (*root).name) == 0 {
            if !(*root).ns.is_null() && !(*(*root).ns).prefix.is_null() {
                let mut fn_0: [xmlChar; 50] = [0; 50];
                let mut fullname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                fullname = xmlBuildQName(
                    (*root).name,
                    (*(*root).ns).prefix,
                    &raw mut fn_0 as *mut xmlChar,
                    50 as c_int,
                );
                if fullname.is_null() {
                    xmlVErrMemory(ctxt, ::core::ptr::null::<c_char>());
                    return 0 as c_int;
                }
                ret = xmlStrEqual((*(*doc).intSubset).name, fullname);
                if fullname != &raw mut fn_0 as *mut xmlChar
                    && fullname != (*root).name as *mut xmlChar
                {
                    xmlFree.expect("non-null function pointer")(
                        fullname as *mut c_void,
                    );
                }
                if ret == 1 as c_int {
                    current_block = 9298655658625806653;
                } else {
                    current_block = 17860125682698302841;
                }
            } else {
                current_block = 17860125682698302841;
            }
            match current_block {
                9298655658625806653 => {}
                _ => {
                    if !(xmlStrEqual(
                        (*(*doc).intSubset).name,
                        b"HTML\0" as *const u8 as *const c_char as *mut xmlChar,
                    ) != 0
                        && xmlStrEqual(
                            (*root).name,
                            b"html\0" as *const u8 as *const c_char as *mut xmlChar,
                        ) != 0)
                    {
                        xmlErrValidNode(
                            ctxt,
                            root,
                            XML_DTD_ROOT_NAME,
                            b"root and DTD name do not match '%s' and '%s'\n\0" as *const u8
                                as *const c_char,
                            (*root).name,
                            (*(*doc).intSubset).name,
                            ::core::ptr::null::<xmlChar>(),
                        );
                        return 0 as c_int;
                    }
                }
            }
        }
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateElement(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut root: xmlNodePtr,
) -> c_int {
    let mut elem: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut value: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: c_int = 1 as c_int;
    if root.is_null() {
        return 0 as c_int;
    }
    if doc.is_null() {
        return 0 as c_int;
    } else if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as c_int;
    }
    elem = root;
    's_39: loop {
        ret &= xmlValidateOneElement(ctxt, doc, elem);
        if (*elem).type_0 as c_uint
            == XML_ELEMENT_NODE as c_int as c_uint
        {
            attr = (*elem).properties as xmlAttrPtr;
            while !attr.is_null() {
                value = xmlNodeListGetString(doc, (*attr).children, 0 as c_int);
                ret &= xmlValidateOneAttribute(ctxt, doc, elem, attr, value);
                if !value.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        value as *mut c_char as *mut c_void,
                    );
                }
                attr = (*attr).next as xmlAttrPtr;
            }
            ns = (*elem).nsDef as xmlNsPtr;
            while !ns.is_null() {
                if (*elem).ns.is_null() {
                    ret &= xmlValidateOneNamespace(
                        ctxt,
                        doc,
                        elem,
                        ::core::ptr::null::<xmlChar>(),
                        ns,
                        (*ns).href,
                    );
                } else {
                    ret &= xmlValidateOneNamespace(
                        ctxt,
                        doc,
                        elem,
                        (*(*elem).ns).prefix,
                        ns,
                        (*ns).href,
                    );
                }
                ns = (*ns).next as xmlNsPtr;
            }
            if !(*elem).children.is_null() {
                elem = (*elem).children as xmlNodePtr;
                continue;
            }
        }
        loop {
            if elem == root {
                break 's_39;
            }
            if !(*elem).next.is_null() {
                break;
            }
            elem = (*elem).parent as xmlNodePtr;
        }
        elem = (*elem).next as xmlNodePtr;
    }
    return ret;
}
unsafe extern "C" fn xmlValidateRef(
    mut ref_0: xmlRefPtr,
    mut ctxt: xmlValidCtxtPtr,
    mut name: *const xmlChar,
) {
    let mut id: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if ref_0.is_null() {
        return;
    }
    if (*ref_0).attr.is_null() && (*ref_0).name.is_null() {
        return;
    }
    attr = (*ref_0).attr;
    if attr.is_null() {
        let mut dup: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut str: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut cur: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut save: xmlChar = 0;
        dup = xmlStrdup(name);
        if dup.is_null() {
            (*ctxt).valid = 0 as c_int;
            return;
        }
        cur = dup;
        while *cur as c_int != 0 as c_int {
            str = cur;
            while *cur as c_int != 0 as c_int
                && !(*cur as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *cur as c_int
                        && *cur as c_int <= 0xa as c_int
                    || *cur as c_int == 0xd as c_int)
            {
                cur = cur.offset(1);
            }
            save = *cur;
            *cur = 0 as xmlChar;
            id = xmlGetID((*ctxt).doc, str);
            if id.is_null() {
                xmlErrValidNodeNr(
                    ctxt,
                    ::core::ptr::null_mut::<xmlNode>(),
                    XML_DTD_UNKNOWN_ID,
                    b"attribute %s line %d references an unknown ID \"%s\"\n\0" as *const u8
                        as *const c_char,
                    (*ref_0).name,
                    (*ref_0).lineno,
                    str,
                );
                (*ctxt).valid = 0 as c_int;
            }
            if save as c_int == 0 as c_int {
                break;
            }
            *cur = save;
            while *cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *cur as c_int
                    && *cur as c_int <= 0xa as c_int
                || *cur as c_int == 0xd as c_int
            {
                cur = cur.offset(1);
            }
        }
        xmlFree.expect("non-null function pointer")(dup as *mut c_void);
    } else if (*attr).atype as c_uint
        == XML_ATTRIBUTE_IDREF as c_int as c_uint
    {
        id = xmlGetID((*ctxt).doc, name);
        if id.is_null() {
            xmlErrValidNode(
                ctxt,
                (*attr).parent as xmlNodePtr,
                XML_DTD_UNKNOWN_ID,
                b"IDREF attribute %s references an unknown ID \"%s\"\n\0" as *const u8
                    as *const c_char,
                (*attr).name,
                name,
                ::core::ptr::null::<xmlChar>(),
            );
            (*ctxt).valid = 0 as c_int;
        }
    } else if (*attr).atype as c_uint
        == XML_ATTRIBUTE_IDREFS as c_int as c_uint
    {
        let mut dup_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut str_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut cur_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut save_0: xmlChar = 0;
        dup_0 = xmlStrdup(name);
        if dup_0.is_null() {
            xmlVErrMemory(
                ctxt,
                b"IDREFS split\0" as *const u8 as *const c_char,
            );
            (*ctxt).valid = 0 as c_int;
            return;
        }
        cur_0 = dup_0;
        while *cur_0 as c_int != 0 as c_int {
            str_0 = cur_0;
            while *cur_0 as c_int != 0 as c_int
                && !(*cur_0 as c_int == 0x20 as c_int
                    || 0x9 as c_int <= *cur_0 as c_int
                        && *cur_0 as c_int <= 0xa as c_int
                    || *cur_0 as c_int == 0xd as c_int)
            {
                cur_0 = cur_0.offset(1);
            }
            save_0 = *cur_0;
            *cur_0 = 0 as xmlChar;
            id = xmlGetID((*ctxt).doc, str_0);
            if id.is_null() {
                xmlErrValidNode(
                    ctxt,
                    (*attr).parent as xmlNodePtr,
                    XML_DTD_UNKNOWN_ID,
                    b"IDREFS attribute %s references an unknown ID \"%s\"\n\0" as *const u8
                        as *const c_char,
                    (*attr).name,
                    str_0,
                    ::core::ptr::null::<xmlChar>(),
                );
                (*ctxt).valid = 0 as c_int;
            }
            if save_0 as c_int == 0 as c_int {
                break;
            }
            *cur_0 = save_0;
            while *cur_0 as c_int == 0x20 as c_int
                || 0x9 as c_int <= *cur_0 as c_int
                    && *cur_0 as c_int <= 0xa as c_int
                || *cur_0 as c_int == 0xd as c_int
            {
                cur_0 = cur_0.offset(1);
            }
        }
        xmlFree.expect("non-null function pointer")(dup_0 as *mut c_void);
    }
}
unsafe extern "C" fn xmlWalkValidateList(
    mut data: *const c_void,
    mut user: *mut c_void,
) -> c_int {
    let mut memo: xmlValidateMemoPtr = user as xmlValidateMemoPtr;
    xmlValidateRef(data as xmlRefPtr, (*memo).ctxt, (*memo).name);
    return 1 as c_int;
}
unsafe extern "C" fn xmlValidateCheckRefCallback(
    mut payload: *mut c_void,
    mut data: *mut c_void,
    mut name: *const xmlChar,
) {
    let mut ref_list: xmlListPtr = payload as xmlListPtr;
    let mut ctxt: xmlValidCtxtPtr = data as xmlValidCtxtPtr;
    let mut memo: xmlValidateMemo = xmlValidateMemo {
        ctxt: ::core::ptr::null_mut::<xmlValidCtxt>(),
        name: ::core::ptr::null::<xmlChar>(),
    };
    if ref_list.is_null() {
        return;
    }
    memo.ctxt = ctxt;
    memo.name = name;
    xmlListWalk(
        ref_list,
        Some(
            xmlWalkValidateList
                as unsafe extern "C" fn(
                    *const c_void,
                    *mut c_void,
                ) -> c_int,
        ),
        &raw mut memo as *mut c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateDocumentFinal(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
) -> c_int {
    let mut table: xmlRefTablePtr = ::core::ptr::null_mut::<xmlRefTable>();
    let mut save: c_uint = 0;
    if ctxt.is_null() {
        return 0 as c_int;
    }
    if doc.is_null() {
        xmlErrValid(
            ctxt,
            XML_DTD_NO_DOC,
            b"xmlValidateDocumentFinal: doc == NULL\n\0" as *const u8 as *const c_char,
            ::core::ptr::null::<c_char>(),
        );
        return 0 as c_int;
    }
    save = (*ctxt).flags;
    (*ctxt).flags &= !XML_VCTXT_USE_PCTXT;
    table = (*doc).refs as xmlRefTablePtr;
    (*ctxt).doc = doc;
    (*ctxt).valid = 1 as c_int;
    xmlHashScan(
        table as xmlHashTablePtr,
        Some(
            xmlValidateCheckRefCallback
                as unsafe extern "C" fn(
                    *mut c_void,
                    *mut c_void,
                    *const xmlChar,
                ) -> (),
        ),
        ctxt as *mut c_void,
    );
    (*ctxt).flags = save;
    return (*ctxt).valid;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateDtd(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut dtd: xmlDtdPtr,
) -> c_int {
    let mut ret: c_int = 0;
    let mut oldExt: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut oldInt: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut root: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if dtd.is_null() {
        return 0 as c_int;
    }
    if doc.is_null() {
        return 0 as c_int;
    }
    oldExt = (*doc).extSubset as xmlDtdPtr;
    oldInt = (*doc).intSubset as xmlDtdPtr;
    (*doc).extSubset = dtd as *mut _xmlDtd;
    (*doc).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
    ret = xmlValidateRoot(ctxt, doc);
    if ret == 0 as c_int {
        (*doc).extSubset = oldExt as *mut _xmlDtd;
        (*doc).intSubset = oldInt as *mut _xmlDtd;
        return ret;
    }
    if !(*doc).ids.is_null() {
        xmlFreeIDTable((*doc).ids as xmlIDTablePtr);
        (*doc).ids = NULL;
    }
    if !(*doc).refs.is_null() {
        xmlFreeRefTable((*doc).refs as xmlRefTablePtr);
        (*doc).refs = NULL;
    }
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    ret = xmlValidateElement(ctxt, doc, root);
    ret &= xmlValidateDocumentFinal(ctxt, doc);
    (*doc).extSubset = oldExt as *mut _xmlDtd;
    (*doc).intSubset = oldInt as *mut _xmlDtd;
    return ret;
}
unsafe extern "C" fn xmlValidateNotationCallback(
    mut payload: *mut c_void,
    mut data: *mut c_void,
    mut name: *const xmlChar,
) {
    let mut cur: xmlEntityPtr = payload as xmlEntityPtr;
    let mut ctxt: xmlValidCtxtPtr = data as xmlValidCtxtPtr;
    if cur.is_null() {
        return;
    }
    if (*cur).etype as c_uint
        == XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as c_int as c_uint
    {
        let mut notation: *mut xmlChar = (*cur).content;
        if !notation.is_null() {
            let mut ret: c_int = 0;
            ret = xmlValidateNotationUse(ctxt, (*cur).doc as xmlDocPtr, notation);
            if ret != 1 as c_int {
                (*ctxt).valid = 0 as c_int;
            }
        }
    }
}
unsafe extern "C" fn xmlValidateAttributeCallback(
    mut payload: *mut c_void,
    mut data: *mut c_void,
    mut name: *const xmlChar,
) {
    let mut cur: xmlAttributePtr = payload as xmlAttributePtr;
    let mut ctxt: xmlValidCtxtPtr = data as xmlValidCtxtPtr;
    let mut ret: c_int = 0;
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut elem: xmlElementPtr = ::core::ptr::null_mut::<xmlElement>();
    if cur.is_null() {
        return;
    }
    match (*cur).atype as c_uint {
        5 | 6 | 10 => {
            if !(*cur).defaultValue.is_null() {
                ret = xmlValidateAttributeValue2(
                    ctxt,
                    (*ctxt).doc,
                    (*cur).name,
                    (*cur).atype,
                    (*cur).defaultValue,
                );
                if ret == 0 as c_int && (*ctxt).valid == 1 as c_int {
                    (*ctxt).valid = 0 as c_int;
                }
            }
            if !(*cur).tree.is_null() {
                let mut tree: xmlEnumerationPtr = (*cur).tree;
                while !tree.is_null() {
                    ret = xmlValidateAttributeValue2(
                        ctxt,
                        (*ctxt).doc,
                        (*cur).name,
                        (*cur).atype,
                        (*tree).name,
                    );
                    if ret == 0 as c_int && (*ctxt).valid == 1 as c_int {
                        (*ctxt).valid = 0 as c_int;
                    }
                    tree = (*tree).next as xmlEnumerationPtr;
                }
            }
        }
        1 | 2 | 3 | 4 | 7 | 8 | 9 | _ => {}
    }
    if (*cur).atype as c_uint
        == XML_ATTRIBUTE_NOTATION as c_int as c_uint
    {
        doc = (*cur).doc as xmlDocPtr;
        if (*cur).elem.is_null() {
            xmlErrValid(
                ctxt,
                XML_ERR_INTERNAL_ERROR,
                b"xmlValidateAttributeCallback(%s): internal error\n\0" as *const u8
                    as *const c_char,
                (*cur).name as *const c_char,
            );
            return;
        }
        if !doc.is_null() {
            elem = xmlGetDtdElementDesc((*doc).intSubset as xmlDtdPtr, (*cur).elem);
        }
        if elem.is_null() && !doc.is_null() {
            elem = xmlGetDtdElementDesc((*doc).extSubset as xmlDtdPtr, (*cur).elem);
        }
        if elem.is_null()
            && !(*cur).parent.is_null()
            && (*(*cur).parent).type_0 as c_uint
                == XML_DTD_NODE as c_int as c_uint
        {
            elem = xmlGetDtdElementDesc((*cur).parent as xmlDtdPtr, (*cur).elem);
        }
        if elem.is_null() {
            xmlErrValidNode(
                ctxt,
                ::core::ptr::null_mut::<xmlNode>(),
                XML_DTD_UNKNOWN_ELEM,
                b"attribute %s: could not find decl for element %s\n\0" as *const u8
                    as *const c_char,
                (*cur).name,
                (*cur).elem,
                ::core::ptr::null::<xmlChar>(),
            );
            return;
        }
        if (*elem).etype as c_uint
            == XML_ELEMENT_TYPE_EMPTY as c_int as c_uint
        {
            xmlErrValidNode(
                ctxt,
                ::core::ptr::null_mut::<xmlNode>(),
                XML_DTD_EMPTY_NOTATION,
                b"NOTATION attribute %s declared for EMPTY element %s\n\0" as *const u8
                    as *const c_char,
                (*cur).name,
                (*cur).elem,
                ::core::ptr::null::<xmlChar>(),
            );
            (*ctxt).valid = 0 as c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateDtdFinal(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
) -> c_int {
    let mut dtd: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
    let mut table: xmlAttributeTablePtr = ::core::ptr::null_mut::<xmlAttributeTable>();
    let mut entities: xmlEntitiesTablePtr = ::core::ptr::null_mut::<xmlEntitiesTable>();
    if doc.is_null() || ctxt.is_null() {
        return 0 as c_int;
    }
    if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        return 0 as c_int;
    }
    (*ctxt).doc = doc;
    (*ctxt).valid = 1 as c_int;
    dtd = (*doc).intSubset as xmlDtdPtr;
    if !dtd.is_null() && !(*dtd).attributes.is_null() {
        table = (*dtd).attributes as xmlAttributeTablePtr;
        xmlHashScan(
            table as xmlHashTablePtr,
            Some(
                xmlValidateAttributeCallback
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *mut c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            ctxt as *mut c_void,
        );
    }
    if !dtd.is_null() && !(*dtd).entities.is_null() {
        entities = (*dtd).entities as xmlEntitiesTablePtr;
        xmlHashScan(
            entities as xmlHashTablePtr,
            Some(
                xmlValidateNotationCallback
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *mut c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            ctxt as *mut c_void,
        );
    }
    dtd = (*doc).extSubset as xmlDtdPtr;
    if !dtd.is_null() && !(*dtd).attributes.is_null() {
        table = (*dtd).attributes as xmlAttributeTablePtr;
        xmlHashScan(
            table as xmlHashTablePtr,
            Some(
                xmlValidateAttributeCallback
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *mut c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            ctxt as *mut c_void,
        );
    }
    if !dtd.is_null() && !(*dtd).entities.is_null() {
        entities = (*dtd).entities as xmlEntitiesTablePtr;
        xmlHashScan(
            entities as xmlHashTablePtr,
            Some(
                xmlValidateNotationCallback
                    as unsafe extern "C" fn(
                        *mut c_void,
                        *mut c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            ctxt as *mut c_void,
        );
    }
    return (*ctxt).valid;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidateDocument(
    mut ctxt: xmlValidCtxtPtr,
    mut doc: xmlDocPtr,
) -> c_int {
    let mut ret: c_int = 0;
    let mut root: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if doc.is_null() {
        return 0 as c_int;
    }
    if (*doc).intSubset.is_null() && (*doc).extSubset.is_null() {
        xmlErrValid(
            ctxt,
            XML_DTD_NO_DTD,
            b"no DTD found!\n\0" as *const u8 as *const c_char,
            ::core::ptr::null::<c_char>(),
        );
        return 0 as c_int;
    }
    if !(*doc).intSubset.is_null()
        && (!(*(*doc).intSubset).SystemID.is_null() || !(*(*doc).intSubset).ExternalID.is_null())
        && (*doc).extSubset.is_null()
    {
        let mut sysID: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        if !(*(*doc).intSubset).SystemID.is_null() {
            sysID = xmlBuildURI((*(*doc).intSubset).SystemID, (*doc).URL);
            if sysID.is_null() {
                xmlErrValid(
                    ctxt,
                    XML_DTD_LOAD_ERROR,
                    b"Could not build URI for external subset \"%s\"\n\0" as *const u8
                        as *const c_char,
                    (*(*doc).intSubset).SystemID as *const c_char,
                );
                return 0 as c_int;
            }
        } else {
            sysID = ::core::ptr::null_mut::<xmlChar>();
        }
        (*doc).extSubset =
            xmlParseDTD((*(*doc).intSubset).ExternalID, sysID as *const xmlChar) as *mut _xmlDtd;
        if !sysID.is_null() {
            xmlFree.expect("non-null function pointer")(sysID as *mut c_void);
        }
        if (*doc).extSubset.is_null() {
            if !(*(*doc).intSubset).SystemID.is_null() {
                xmlErrValid(
                    ctxt,
                    XML_DTD_LOAD_ERROR,
                    b"Could not load the external subset \"%s\"\n\0" as *const u8
                        as *const c_char,
                    (*(*doc).intSubset).SystemID as *const c_char,
                );
            } else {
                xmlErrValid(
                    ctxt,
                    XML_DTD_LOAD_ERROR,
                    b"Could not load the external subset \"%s\"\n\0" as *const u8
                        as *const c_char,
                    (*(*doc).intSubset).ExternalID as *const c_char,
                );
            }
            return 0 as c_int;
        }
    }
    if !(*doc).ids.is_null() {
        xmlFreeIDTable((*doc).ids as xmlIDTablePtr);
        (*doc).ids = NULL;
    }
    if !(*doc).refs.is_null() {
        xmlFreeRefTable((*doc).refs as xmlRefTablePtr);
        (*doc).refs = NULL;
    }
    ret = xmlValidateDtdFinal(ctxt, doc);
    if xmlValidateRoot(ctxt, doc) == 0 {
        return 0 as c_int;
    }
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    ret &= xmlValidateElement(ctxt, doc, root);
    ret &= xmlValidateDocumentFinal(ctxt, doc);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidGetPotentialChildren(
    mut ctree: *mut xmlElementContent,
    mut names: *mut *const xmlChar,
    mut len: *mut c_int,
    mut max: c_int,
) -> c_int {
    let mut i: c_int = 0;
    if ctree.is_null() || names.is_null() || len.is_null() {
        return -(1 as c_int);
    }
    if *len >= max {
        return *len;
    }
    match (*ctree).type_0 as c_uint {
        1 => {
            i = 0 as c_int;
            while i < *len {
                if xmlStrEqual(
                    b"#PCDATA\0" as *const u8 as *const c_char as *mut xmlChar,
                    *names.offset(i as isize),
                ) != 0
                {
                    return *len;
                }
                i += 1;
            }
            let fresh6 = *len;
            *len = *len + 1;
            let ref mut fresh7 = *names.offset(fresh6 as isize);
            *fresh7 = b"#PCDATA\0" as *const u8 as *const c_char as *mut xmlChar;
        }
        2 => {
            i = 0 as c_int;
            while i < *len {
                if xmlStrEqual((*ctree).name, *names.offset(i as isize)) != 0 {
                    return *len;
                }
                i += 1;
            }
            let fresh8 = *len;
            *len = *len + 1;
            let ref mut fresh9 = *names.offset(fresh8 as isize);
            *fresh9 = (*ctree).name;
        }
        3 => {
            xmlValidGetPotentialChildren((*ctree).c1 as *mut xmlElementContent, names, len, max);
            xmlValidGetPotentialChildren((*ctree).c2 as *mut xmlElementContent, names, len, max);
        }
        4 => {
            xmlValidGetPotentialChildren((*ctree).c1 as *mut xmlElementContent, names, len, max);
            xmlValidGetPotentialChildren((*ctree).c2 as *mut xmlElementContent, names, len, max);
        }
        _ => {}
    }
    return *len;
}
unsafe extern "C" fn xmlNoValidityErr(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
}
#[no_mangle]
pub unsafe extern "C" fn xmlValidGetValidElements(
    mut prev: *mut xmlNode,
    mut next: *mut xmlNode,
    mut names: *mut *const xmlChar,
    mut max: c_int,
) -> c_int {
    let mut vctxt: xmlValidCtxt = xmlValidCtxt {
        userData: ::core::ptr::null_mut::<c_void>(),
        error: None,
        warning: None,
        node: ::core::ptr::null_mut::<xmlNode>(),
        nodeNr: 0,
        nodeMax: 0,
        nodeTab: ::core::ptr::null_mut::<xmlNodePtr>(),
        flags: 0,
        doc: ::core::ptr::null_mut::<xmlDoc>(),
        valid: 0,
        vstate: ::core::ptr::null_mut::<xmlValidState>(),
        vstateNr: 0,
        vstateMax: 0,
        vstateTab: ::core::ptr::null_mut::<xmlValidState>(),
        am: ::core::ptr::null_mut::<xmlAutomata>(),
        state: ::core::ptr::null_mut::<xmlAutomataState>(),
    };
    let mut nb_valid_elements: c_int = 0 as c_int;
    let mut elements: [*const xmlChar; 256] = [
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
    ];
    let mut nb_elements: c_int = 0 as c_int;
    let mut i: c_int = 0;
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ref_node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
    let mut parent: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
    let mut test_node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
    let mut prev_next: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
    let mut next_prev: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
    let mut parent_childs: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
    let mut parent_last: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
    let mut element_desc: *mut xmlElement = ::core::ptr::null_mut::<xmlElement>();
    if prev.is_null() && next.is_null() {
        return -(1 as c_int);
    }
    if names.is_null() {
        return -(1 as c_int);
    }
    if max <= 0 as c_int {
        return -(1 as c_int);
    }
    memset(
        &raw mut vctxt as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlValidCtxt>() as size_t,
    );
    vctxt.error = Some(
        xmlNoValidityErr
            as unsafe extern "C" fn(
                *mut c_void,
                *const c_char,
                ...
            ) -> (),
    ) as xmlValidityErrorFunc;
    nb_valid_elements = 0 as c_int;
    ref_node = if !prev.is_null() { prev } else { next };
    parent = (*ref_node).parent as *mut xmlNode;
    element_desc = xmlGetDtdElementDesc((*(*parent).doc).intSubset as xmlDtdPtr, (*parent).name)
        as *mut xmlElement;
    if element_desc.is_null() && !(*(*parent).doc).extSubset.is_null() {
        element_desc = xmlGetDtdElementDesc((*(*parent).doc).extSubset as xmlDtdPtr, (*parent).name)
            as *mut xmlElement;
    }
    if element_desc.is_null() {
        return -(1 as c_int);
    }
    prev_next = (if !prev.is_null() {
        (*prev).next
    } else {
        ::core::ptr::null_mut::<_xmlNode>()
    }) as *mut xmlNode;
    next_prev = (if !next.is_null() {
        (*next).prev
    } else {
        ::core::ptr::null_mut::<_xmlNode>()
    }) as *mut xmlNode;
    parent_childs = (*parent).children as *mut xmlNode;
    parent_last = (*parent).last as *mut xmlNode;
    test_node = xmlNewDocNode(
        (*ref_node).doc as xmlDocPtr,
        ::core::ptr::null_mut::<xmlNs>(),
        b"<!dummy?>\0" as *const u8 as *const c_char as *mut xmlChar,
        ::core::ptr::null::<xmlChar>(),
    ) as *mut xmlNode;
    if test_node.is_null() {
        return -(1 as c_int);
    }
    (*test_node).parent = parent as *mut _xmlNode;
    (*test_node).prev = prev as *mut _xmlNode;
    (*test_node).next = next as *mut _xmlNode;
    name = (*test_node).name;
    if !prev.is_null() {
        (*prev).next = test_node as *mut _xmlNode;
    } else {
        (*parent).children = test_node as *mut _xmlNode;
    }
    if !next.is_null() {
        (*next).prev = test_node as *mut _xmlNode;
    } else {
        (*parent).last = test_node as *mut _xmlNode;
    }
    nb_elements = xmlValidGetPotentialChildren(
        (*element_desc).content as *mut xmlElementContent,
        &raw mut elements as *mut *const xmlChar,
        &raw mut nb_elements,
        256 as c_int,
    );
    i = 0 as c_int;
    while i < nb_elements {
        (*test_node).name = elements[i as usize];
        if xmlValidateOneElement(
            &raw mut vctxt,
            (*parent).doc as xmlDocPtr,
            parent as xmlNodePtr,
        ) != 0
        {
            let mut j: c_int = 0;
            j = 0 as c_int;
            while j < nb_valid_elements {
                if xmlStrEqual(elements[i as usize], *names.offset(j as isize)) != 0 {
                    break;
                }
                j += 1;
            }
            let fresh10 = nb_valid_elements;
            nb_valid_elements = nb_valid_elements + 1;
            let ref mut fresh11 = *names.offset(fresh10 as isize);
            *fresh11 = elements[i as usize];
            if nb_valid_elements >= max {
                break;
            }
        }
        i += 1;
    }
    if !prev.is_null() {
        (*prev).next = prev_next as *mut _xmlNode;
    }
    if !next.is_null() {
        (*next).prev = next_prev as *mut _xmlNode;
    }
    (*parent).children = parent_childs as *mut _xmlNode;
    (*parent).last = parent_last as *mut _xmlNode;
    (*test_node).name = name;
    xmlFreeNode(test_node as xmlNodePtr);
    return nb_valid_elements;
}

