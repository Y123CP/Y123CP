use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_structs::*;
pub use crate::src::c_types::*;
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
    fn xmlDictReference(dict: xmlDictPtr) -> c_int;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: c_int,
    ) -> *const xmlChar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlPattern {
    pub data: *mut c_void,
    pub dict: xmlDictPtr,
    pub next: *mut _xmlPattern,
    pub pattern: *const xmlChar,
    pub flags: c_int,
    pub nbStep: c_int,
    pub maxStep: c_int,
    pub steps: xmlStepOpPtr,
    pub stream: xmlStreamCompPtr,
}
pub type xmlStreamCompPtr = *mut xmlStreamComp;
pub type xmlStreamComp = _xmlStreamComp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamComp {
    pub dict: *mut xmlDict,
    pub nbStep: c_int,
    pub maxStep: c_int,
    pub steps: xmlStreamStepPtr,
    pub flags: c_int,
}
pub type xmlStreamStepPtr = *mut xmlStreamStep;
pub type xmlStreamStep = _xmlStreamStep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamStep {
    pub flags: c_int,
    pub name: *const xmlChar,
    pub ns: *const xmlChar,
    pub nodeType: c_int,
}
pub type xmlStepOpPtr = *mut xmlStepOp;
pub type xmlStepOp = _xmlStepOp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepOp {
    pub op: xmlPatOp,
    pub value: *const xmlChar,
    pub value2: *const xmlChar,
}
pub type xmlPatOp = c_uint;
pub const XML_OP_ALL: xmlPatOp = 8;
pub const XML_OP_NS: xmlPatOp = 7;
pub const XML_OP_ANCESTOR: xmlPatOp = 6;
pub const XML_OP_PARENT: xmlPatOp = 5;
pub const XML_OP_ATTR: xmlPatOp = 4;
pub const XML_OP_CHILD: xmlPatOp = 3;
pub const XML_OP_ELEM: xmlPatOp = 2;
pub const XML_OP_ROOT: xmlPatOp = 1;
pub const XML_OP_END: xmlPatOp = 0;
pub type xmlPattern = _xmlPattern;
pub type xmlPatternPtr = *mut xmlPattern;
pub type C2RustUnnamed = c_uint;
pub const XML_PATTERN_XSFIELD: C2RustUnnamed = 4;
pub const XML_PATTERN_XSSEL: C2RustUnnamed = 2;
pub const XML_PATTERN_XPATH: C2RustUnnamed = 1;
pub const XML_PATTERN_DEFAULT: C2RustUnnamed = 0;
pub type xmlPatParserContextPtr = *mut xmlPatParserContext;
pub type xmlPatParserContext = _xmlPatParserContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlPatParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: c_int,
    pub dict: xmlDictPtr,
    pub comp: xmlPatternPtr,
    pub elem: xmlNodePtr,
    pub namespaces: *mut *const xmlChar,
    pub nb_namespaces: c_int,
}
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: c_int,
    pub nbLongRange: c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}

pub type xmlStepStatePtr = *mut xmlStepState;
pub type xmlStepState = _xmlStepState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepState {
    pub step: c_int,
    pub node: xmlNodePtr,
}
pub type xmlStepStates = _xmlStepStates;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepStates {
    pub nbstates: c_int,
    pub maxstates: c_int,
    pub states: xmlStepStatePtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamCtxt {
    pub next: *mut _xmlStreamCtxt,
    pub comp: xmlStreamCompPtr,
    pub nbState: c_int,
    pub maxState: c_int,
    pub level: c_int,
    pub states: *mut c_int,
    pub flags: c_int,
    pub blockLevel: c_int,
}
pub type xmlStreamCtxt = _xmlStreamCtxt;
pub type xmlStreamCtxtPtr = *mut xmlStreamCtxt;

pub const XML_STREAM_STEP_DESC: c_int = 1 as c_int;
pub const XML_STREAM_STEP_FINAL: c_int = 2 as c_int;
pub const XML_STREAM_STEP_ROOT: c_int = 4 as c_int;
pub const XML_STREAM_STEP_ATTR: c_int = 8 as c_int;
pub const XML_STREAM_STEP_NODE: c_int = 16 as c_int;
pub const XML_STREAM_STEP_IN_SET: c_int = 32 as c_int;
pub const XML_STREAM_FINAL_IS_ANY_NODE: c_int =
    (1 as c_int) << 14 as c_int;
pub const XML_STREAM_FROM_ROOT: c_int =
    (1 as c_int) << 15 as c_int;
pub const XML_STREAM_DESC: c_int =
    (1 as c_int) << 16 as c_int;
pub const XML_STREAM_ANY_NODE: c_int = 100 as c_int;
pub const PAT_FROM_ROOT: c_int = (1 as c_int) << 8 as c_int;
pub const PAT_FROM_CUR: c_int = (1 as c_int) << 9 as c_int;
unsafe extern "C" fn xmlNewPattern() -> xmlPatternPtr {
    let mut cur: xmlPatternPtr = ::core::ptr::null_mut::<xmlPattern>();
    cur = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlPattern>() as size_t
    ) as xmlPatternPtr;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlPattern>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlPattern>() as size_t,
    );
    (*cur).maxStep = 10 as c_int;
    (*cur).steps = xmlMalloc.expect("non-null function pointer")(
        ((*cur).maxStep as size_t).wrapping_mul(::core::mem::size_of::<xmlStepOp>() as size_t),
    ) as xmlStepOpPtr;
    if (*cur).steps.is_null() {
        xmlFree.expect("non-null function pointer")(cur as *mut c_void);
        return ::core::ptr::null_mut::<xmlPattern>();
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreePattern(mut comp: xmlPatternPtr) {
    xmlFreePatternList(comp);
}
unsafe extern "C" fn xmlFreePatternInternal(mut comp: xmlPatternPtr) {
    let mut op: xmlStepOpPtr = ::core::ptr::null_mut::<xmlStepOp>();
    let mut i: c_int = 0;
    if comp.is_null() {
        return;
    }
    if !(*comp).stream.is_null() {
        xmlFreeStreamComp((*comp).stream);
    }
    if !(*comp).pattern.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*comp).pattern as *mut xmlChar as *mut c_void,
        );
    }
    if !(*comp).steps.is_null() {
        if (*comp).dict.is_null() {
            i = 0 as c_int;
            while i < (*comp).nbStep {
                op = (*comp).steps.offset(i as isize) as *mut xmlStepOp as xmlStepOpPtr;
                if !(*op).value.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*op).value as *mut xmlChar as *mut c_void,
                    );
                }
                if !(*op).value2.is_null() {
                    xmlFree.expect("non-null function pointer")(
                        (*op).value2 as *mut xmlChar as *mut c_void,
                    );
                }
                i += 1;
            }
        }
        xmlFree.expect("non-null function pointer")((*comp).steps as *mut c_void);
    }
    if !(*comp).dict.is_null() {
        xmlDictFree((*comp).dict);
    }
    memset(
        comp as *mut c_void,
        -(1 as c_int),
        ::core::mem::size_of::<xmlPattern>() as size_t,
    );
    xmlFree.expect("non-null function pointer")(comp as *mut c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreePatternList(mut comp: xmlPatternPtr) {
    let mut cur: xmlPatternPtr = ::core::ptr::null_mut::<xmlPattern>();
    while !comp.is_null() {
        cur = comp;
        comp = (*comp).next as xmlPatternPtr;
        (*cur).next = ::core::ptr::null_mut::<_xmlPattern>();
        xmlFreePatternInternal(cur);
    }
}
unsafe extern "C" fn xmlNewPatParserContext(
    mut pattern: *const xmlChar,
    mut dict: xmlDictPtr,
    mut namespaces: *mut *const xmlChar,
) -> xmlPatParserContextPtr {
    let mut cur: xmlPatParserContextPtr = ::core::ptr::null_mut::<xmlPatParserContext>();
    if pattern.is_null() {
        return ::core::ptr::null_mut::<xmlPatParserContext>();
    }
    cur = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlPatParserContext>() as size_t,
    ) as xmlPatParserContextPtr;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlPatParserContext>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlPatParserContext>() as size_t,
    );
    (*cur).dict = dict;
    (*cur).cur = pattern;
    (*cur).base = pattern;
    if !namespaces.is_null() {
        let mut i: c_int = 0;
        i = 0 as c_int;
        while !(*namespaces.offset((2 as c_int * i) as isize)).is_null() {
            i += 1;
        }
        (*cur).nb_namespaces = i;
    } else {
        (*cur).nb_namespaces = 0 as c_int;
    }
    (*cur).namespaces = namespaces;
    return cur;
}
unsafe extern "C" fn xmlFreePatParserContext(mut ctxt: xmlPatParserContextPtr) {
    if ctxt.is_null() {
        return;
    }
    memset(
        ctxt as *mut c_void,
        -(1 as c_int),
        ::core::mem::size_of::<xmlPatParserContext>() as size_t,
    );
    xmlFree.expect("non-null function pointer")(ctxt as *mut c_void);
}
unsafe extern "C" fn xmlPatternAdd(
    mut ctxt: xmlPatParserContextPtr,
    mut comp: xmlPatternPtr,
    mut op: xmlPatOp,
    mut value: *mut xmlChar,
    mut value2: *mut xmlChar,
) -> c_int {
    if (*comp).nbStep >= (*comp).maxStep {
        let mut temp: xmlStepOpPtr = ::core::ptr::null_mut::<xmlStepOp>();
        temp = xmlRealloc.expect("non-null function pointer")(
            (*comp).steps as *mut c_void,
            (((*comp).maxStep * 2 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlStepOp>() as size_t),
        ) as xmlStepOpPtr;
        if temp.is_null() {
            return -(1 as c_int);
        }
        (*comp).steps = temp;
        (*comp).maxStep *= 2 as c_int;
    }
    (*(*comp).steps.offset((*comp).nbStep as isize)).op = op;
    let ref mut fresh10 = (*(*comp).steps.offset((*comp).nbStep as isize)).value;
    *fresh10 = value;
    let ref mut fresh11 = (*(*comp).steps.offset((*comp).nbStep as isize)).value2;
    *fresh11 = value2;
    (*comp).nbStep += 1;
    return 0 as c_int;
}
unsafe extern "C" fn xmlReversePattern(mut comp: xmlPatternPtr) -> c_int {
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    if (*comp).nbStep > 0 as c_int
        && (*(*comp).steps.offset(0 as c_int as isize)).op as c_uint
            == XML_OP_ANCESTOR as c_int as c_uint
    {
        i = 0 as c_int;
        j = 1 as c_int;
        while j < (*comp).nbStep {
            let ref mut fresh0 = (*(*comp).steps.offset(i as isize)).value;
            *fresh0 = (*(*comp).steps.offset(j as isize)).value;
            let ref mut fresh1 = (*(*comp).steps.offset(i as isize)).value2;
            *fresh1 = (*(*comp).steps.offset(j as isize)).value2;
            (*(*comp).steps.offset(i as isize)).op = (*(*comp).steps.offset(j as isize)).op;
            i += 1;
            j += 1;
        }
        (*comp).nbStep -= 1;
    }
    if (*comp).nbStep >= (*comp).maxStep {
        let mut temp: xmlStepOpPtr = ::core::ptr::null_mut::<xmlStepOp>();
        temp = xmlRealloc.expect("non-null function pointer")(
            (*comp).steps as *mut c_void,
            (((*comp).maxStep * 2 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlStepOp>() as size_t),
        ) as xmlStepOpPtr;
        if temp.is_null() {
            return -(1 as c_int);
        }
        (*comp).steps = temp;
        (*comp).maxStep *= 2 as c_int;
    }
    i = 0 as c_int;
    j = (*comp).nbStep - 1 as c_int;
    while j > i {
        let mut tmp: *const xmlChar = ::core::ptr::null::<xmlChar>();
        let mut op: xmlPatOp = XML_OP_END;
        tmp = (*(*comp).steps.offset(i as isize)).value;
        let ref mut fresh2 = (*(*comp).steps.offset(i as isize)).value;
        *fresh2 = (*(*comp).steps.offset(j as isize)).value;
        let ref mut fresh3 = (*(*comp).steps.offset(j as isize)).value;
        *fresh3 = tmp;
        tmp = (*(*comp).steps.offset(i as isize)).value2;
        let ref mut fresh4 = (*(*comp).steps.offset(i as isize)).value2;
        *fresh4 = (*(*comp).steps.offset(j as isize)).value2;
        let ref mut fresh5 = (*(*comp).steps.offset(j as isize)).value2;
        *fresh5 = tmp;
        op = (*(*comp).steps.offset(i as isize)).op;
        (*(*comp).steps.offset(i as isize)).op = (*(*comp).steps.offset(j as isize)).op;
        (*(*comp).steps.offset(j as isize)).op = op;
        j -= 1;
        i += 1;
    }
    let ref mut fresh6 = (*(*comp).steps.offset((*comp).nbStep as isize)).value;
    *fresh6 = ::core::ptr::null::<xmlChar>();
    let ref mut fresh7 = (*(*comp).steps.offset((*comp).nbStep as isize)).value2;
    *fresh7 = ::core::ptr::null::<xmlChar>();
    let fresh8 = (*comp).nbStep;
    (*comp).nbStep = (*comp).nbStep + 1;
    (*(*comp).steps.offset(fresh8 as isize)).op = XML_OP_END;
    return 0 as c_int;
}
unsafe extern "C" fn xmlPatPushState(
    mut states: *mut xmlStepStates,
    mut step: c_int,
    mut node: xmlNodePtr,
) -> c_int {
    if (*states).states.is_null() || (*states).maxstates <= 0 as c_int {
        (*states).maxstates = 4 as c_int;
        (*states).nbstates = 0 as c_int;
        (*states).states = xmlMalloc.expect("non-null function pointer")(
            (4 as size_t).wrapping_mul(::core::mem::size_of::<xmlStepState>() as size_t),
        ) as xmlStepStatePtr;
    } else if (*states).maxstates <= (*states).nbstates {
        let mut tmp: *mut xmlStepState = ::core::ptr::null_mut::<xmlStepState>();
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*states).states as *mut c_void,
            ((2 as c_int * (*states).maxstates) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlStepState>() as size_t),
        ) as xmlStepStatePtr as *mut xmlStepState;
        if tmp.is_null() {
            return -(1 as c_int);
        }
        (*states).states = tmp as xmlStepStatePtr;
        (*states).maxstates *= 2 as c_int;
    }
    (*(*states).states.offset((*states).nbstates as isize)).step = step;
    let fresh12 = (*states).nbstates;
    (*states).nbstates = (*states).nbstates + 1;
    let ref mut fresh13 = (*(*states).states.offset(fresh12 as isize)).node;
    *fresh13 = node;
    return 0 as c_int;
}
unsafe extern "C" fn xmlPatMatch(
    mut comp: xmlPatternPtr,
    mut node: xmlNodePtr,
) -> c_int {
    let mut current_block: u64;
    let mut i: c_int = 0;
    let mut step: xmlStepOpPtr = ::core::ptr::null_mut::<xmlStepOp>();
    let mut states: xmlStepStates = _xmlStepStates {
        nbstates: 0 as c_int,
        maxstates: 0 as c_int,
        states: ::core::ptr::null_mut::<xmlStepState>(),
    };
    if comp.is_null() || node.is_null() {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < (*comp).nbStep {
        step = (*comp).steps.offset(i as isize) as *mut xmlStepOp as xmlStepOpPtr;
        match (*step).op as c_uint {
            0 => {
                break;
            }
            1 => {
                if (*node).type_0 as c_uint
                    == XML_NAMESPACE_DECL as c_int as c_uint
                {
                    current_block = 7062278877704567555;
                } else {
                    node = (*node).parent as xmlNodePtr;
                    if (*node).type_0 as c_uint
                        == XML_DOCUMENT_NODE as c_int as c_uint
                        || (*node).type_0 as c_uint
                            == XML_HTML_DOCUMENT_NODE as c_int as c_uint
                    {
                        current_block = 4644295000439058019;
                    } else {
                        current_block = 7062278877704567555;
                    }
                }
            }
            2 => {
                if (*node).type_0 as c_uint
                    != XML_ELEMENT_NODE as c_int as c_uint
                {
                    current_block = 7062278877704567555;
                } else if (*step).value.is_null() {
                    current_block = 4644295000439058019;
                } else if *(*step).value.offset(0 as c_int as isize)
                    as c_int
                    != *(*node).name.offset(0 as c_int as isize) as c_int
                {
                    current_block = 7062278877704567555;
                } else if xmlStrEqual((*step).value, (*node).name) == 0 {
                    current_block = 7062278877704567555;
                } else if (*node).ns.is_null() {
                    if !(*step).value2.is_null() {
                        current_block = 7062278877704567555;
                    } else {
                        current_block = 4644295000439058019;
                    }
                } else if !(*(*node).ns).href.is_null() {
                    if (*step).value2.is_null() {
                        current_block = 7062278877704567555;
                    } else if xmlStrEqual((*step).value2, (*(*node).ns).href) == 0 {
                        current_block = 7062278877704567555;
                    } else {
                        current_block = 4644295000439058019;
                    }
                } else {
                    current_block = 4644295000439058019;
                }
            }
            3 => {
                let mut lst: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                if (*node).type_0 as c_uint
                    != XML_ELEMENT_NODE as c_int as c_uint
                    && (*node).type_0 as c_uint
                        != XML_DOCUMENT_NODE as c_int as c_uint
                    && (*node).type_0 as c_uint
                        != XML_HTML_DOCUMENT_NODE as c_int as c_uint
                {
                    current_block = 7062278877704567555;
                } else {
                    lst = (*node).children as xmlNodePtr;
                    if !(*step).value.is_null() {
                        while !lst.is_null() {
                            if (*lst).type_0 as c_uint
                                == XML_ELEMENT_NODE as c_int as c_uint
                                && *(*step).value.offset(0 as c_int as isize)
                                    as c_int
                                    == *(*lst).name.offset(0 as c_int as isize)
                                        as c_int
                                && xmlStrEqual((*step).value, (*lst).name) != 0
                            {
                                break;
                            }
                            lst = (*lst).next as xmlNodePtr;
                        }
                        if !lst.is_null() {
                            current_block = 4644295000439058019;
                        } else {
                            current_block = 7062278877704567555;
                        }
                    } else {
                        current_block = 7062278877704567555;
                    }
                }
            }
            4 => {
                if (*node).type_0 as c_uint
                    != XML_ATTRIBUTE_NODE as c_int as c_uint
                {
                    current_block = 7062278877704567555;
                } else {
                    if !(*step).value.is_null() {
                        if *(*step).value.offset(0 as c_int as isize)
                            as c_int
                            != *(*node).name.offset(0 as c_int as isize)
                                as c_int
                        {
                            current_block = 7062278877704567555;
                        } else if xmlStrEqual((*step).value, (*node).name) == 0 {
                            current_block = 7062278877704567555;
                        } else {
                            current_block = 1118134448028020070;
                        }
                    } else {
                        current_block = 1118134448028020070;
                    }
                    match current_block {
                        7062278877704567555 => {}
                        _ => {
                            if (*node).ns.is_null() {
                                if !(*step).value2.is_null() {
                                    current_block = 7062278877704567555;
                                } else {
                                    current_block = 4644295000439058019;
                                }
                            } else if !(*step).value2.is_null() {
                                if xmlStrEqual((*step).value2, (*(*node).ns).href) == 0 {
                                    current_block = 7062278877704567555;
                                } else {
                                    current_block = 4644295000439058019;
                                }
                            } else {
                                current_block = 4644295000439058019;
                            }
                        }
                    }
                }
            }
            5 => {
                if (*node).type_0 as c_uint
                    == XML_DOCUMENT_NODE as c_int as c_uint
                    || (*node).type_0 as c_uint
                        == XML_HTML_DOCUMENT_NODE as c_int as c_uint
                    || (*node).type_0 as c_uint
                        == XML_NAMESPACE_DECL as c_int as c_uint
                {
                    current_block = 7062278877704567555;
                } else {
                    node = (*node).parent as xmlNodePtr;
                    if node.is_null() {
                        current_block = 7062278877704567555;
                    } else if (*step).value.is_null() {
                        current_block = 4644295000439058019;
                    } else if *(*step).value.offset(0 as c_int as isize)
                        as c_int
                        != *(*node).name.offset(0 as c_int as isize)
                            as c_int
                    {
                        current_block = 7062278877704567555;
                    } else if xmlStrEqual((*step).value, (*node).name) == 0 {
                        current_block = 7062278877704567555;
                    } else if (*node).ns.is_null() {
                        if !(*step).value2.is_null() {
                            current_block = 7062278877704567555;
                        } else {
                            current_block = 4644295000439058019;
                        }
                    } else if !(*(*node).ns).href.is_null() {
                        if (*step).value2.is_null() {
                            current_block = 7062278877704567555;
                        } else if xmlStrEqual((*step).value2, (*(*node).ns).href) == 0 {
                            current_block = 7062278877704567555;
                        } else {
                            current_block = 4644295000439058019;
                        }
                    } else {
                        current_block = 4644295000439058019;
                    }
                }
            }
            6 => {
                if (*step).value.is_null() {
                    i += 1;
                    step = (*comp).steps.offset(i as isize) as *mut xmlStepOp as xmlStepOpPtr;
                    if (*step).op as c_uint
                        == XML_OP_ROOT as c_int as c_uint
                    {
                        break;
                    }
                    if (*step).op as c_uint
                        != XML_OP_ELEM as c_int as c_uint
                    {
                        current_block = 7062278877704567555;
                    } else {
                        if (*step).value.is_null() {
                            return -(1 as c_int);
                        }
                        current_block = 5159818223158340697;
                    }
                } else {
                    current_block = 5159818223158340697;
                }
                match current_block {
                    7062278877704567555 => {}
                    _ => {
                        if node.is_null() {
                            current_block = 7062278877704567555;
                        } else if (*node).type_0 as c_uint
                            == XML_DOCUMENT_NODE as c_int as c_uint
                            || (*node).type_0 as c_uint
                                == XML_HTML_DOCUMENT_NODE as c_int
                                    as c_uint
                            || (*node).type_0 as c_uint
                                == XML_NAMESPACE_DECL as c_int as c_uint
                        {
                            current_block = 7062278877704567555;
                        } else {
                            node = (*node).parent as xmlNodePtr;
                            while !node.is_null() {
                                if (*node).type_0 as c_uint
                                    == XML_ELEMENT_NODE as c_int as c_uint
                                    && *(*step).value.offset(0 as c_int as isize)
                                        as c_int
                                        == *(*node).name.offset(0 as c_int as isize)
                                            as c_int
                                    && xmlStrEqual((*step).value, (*node).name) != 0
                                {
                                    if (*node).ns.is_null() {
                                        if (*step).value2.is_null() {
                                            break;
                                        }
                                    } else if !(*(*node).ns).href.is_null() {
                                        if !(*step).value2.is_null()
                                            && xmlStrEqual((*step).value2, (*(*node).ns).href) != 0
                                        {
                                            break;
                                        }
                                    }
                                }
                                node = (*node).parent as xmlNodePtr;
                            }
                            if node.is_null() {
                                current_block = 7062278877704567555;
                            } else {
                                if (*step).op as c_uint
                                    == XML_OP_ANCESTOR as c_int as c_uint
                                {
                                    xmlPatPushState(&raw mut states, i, node);
                                } else {
                                    xmlPatPushState(
                                        &raw mut states,
                                        i - 1 as c_int,
                                        node,
                                    );
                                }
                                current_block = 4644295000439058019;
                            }
                        }
                    }
                }
            }
            7 => {
                if (*node).type_0 as c_uint
                    != XML_ELEMENT_NODE as c_int as c_uint
                {
                    current_block = 7062278877704567555;
                } else if (*node).ns.is_null() {
                    if !(*step).value.is_null() {
                        current_block = 7062278877704567555;
                    } else {
                        current_block = 4644295000439058019;
                    }
                } else if !(*(*node).ns).href.is_null() {
                    if (*step).value.is_null() {
                        current_block = 7062278877704567555;
                    } else if xmlStrEqual((*step).value, (*(*node).ns).href) == 0 {
                        current_block = 7062278877704567555;
                    } else {
                        current_block = 4644295000439058019;
                    }
                } else {
                    current_block = 4644295000439058019;
                }
            }
            8 => {
                if (*node).type_0 as c_uint
                    != XML_ELEMENT_NODE as c_int as c_uint
                {
                    current_block = 7062278877704567555;
                } else {
                    current_block = 4644295000439058019;
                }
            }
            _ => {
                current_block = 4644295000439058019;
            }
        }
        match current_block {
            4644295000439058019 => {
                i += 1;
            }
            _ => {
                if states.states.is_null() {
                    return 0 as c_int;
                }
                if states.nbstates <= 0 as c_int {
                    xmlFree.expect("non-null function pointer")(
                        states.states as *mut c_void,
                    );
                    return 0 as c_int;
                }
                states.nbstates -= 1;
                i = (*states.states.offset(states.nbstates as isize)).step;
                node = (*states.states.offset(states.nbstates as isize)).node;
            }
        }
    }
    if !states.states.is_null() {
        xmlFree.expect("non-null function pointer")(states.states as *mut c_void);
    }
    return 1 as c_int;
}
unsafe extern "C" fn xmlPatScanName(mut ctxt: xmlPatParserContextPtr) -> *mut xmlChar {
    let mut q: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut val: c_int = 0;
    let mut len: c_int = 0;
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
    q = (*ctxt).cur;
    cur = q;
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    if !((if val < 0x100 as c_int {
        (0x41 as c_int <= val && val <= 0x5a as c_int
            || 0x61 as c_int <= val && val <= 0x7a as c_int
            || 0xc0 as c_int <= val && val <= 0xd6 as c_int
            || 0xd8 as c_int <= val && val <= 0xf6 as c_int
            || 0xf8 as c_int <= val) as c_int
    } else {
        xmlCharInRange(val as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if val < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= val && val <= 0x9fa5 as c_int
                || val == 0x3007 as c_int
                || 0x3021 as c_int <= val && val <= 0x3029 as c_int)
                as c_int
        }) != 0)
        && val != '_' as i32
        && val != ':' as i32
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while (if val < 0x100 as c_int {
        (0x41 as c_int <= val && val <= 0x5a as c_int
            || 0x61 as c_int <= val && val <= 0x7a as c_int
            || 0xc0 as c_int <= val && val <= 0xd6 as c_int
            || 0xd8 as c_int <= val && val <= 0xf6 as c_int
            || 0xf8 as c_int <= val) as c_int
    } else {
        xmlCharInRange(val as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if val < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= val && val <= 0x9fa5 as c_int
                || val == 0x3007 as c_int
                || 0x3021 as c_int <= val && val <= 0x3029 as c_int)
                as c_int
        }) != 0
        || (if val < 0x100 as c_int {
            (0x30 as c_int <= val && val <= 0x39 as c_int)
                as c_int
        } else {
            xmlCharInRange(val as c_uint, &raw const xmlIsDigitGroup)
        }) != 0
        || val == '.' as i32
        || val == '-' as i32
        || val == '_' as i32
        || (if val < 0x100 as c_int {
            0 as c_int
        } else {
            xmlCharInRange(val as c_uint, &raw const xmlIsCombiningGroup)
        }) != 0
        || (if val < 0x100 as c_int {
            (val == 0xb7 as c_int) as c_int
        } else {
            xmlCharInRange(val as c_uint, &raw const xmlIsExtenderGroup)
        }) != 0
    {
        cur = cur.offset(len as isize);
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    }
    if !(*ctxt).dict.is_null() {
        ret = xmlDictLookup(
            (*ctxt).dict,
            q,
            cur.offset_from(q) as c_long as c_int,
        ) as *mut xmlChar;
    } else {
        ret = xmlStrndup(
            q,
            cur.offset_from(q) as c_long as c_int,
        );
    }
    (*ctxt).cur = cur;
    return ret;
}
unsafe extern "C" fn xmlPatScanNCName(mut ctxt: xmlPatParserContextPtr) -> *mut xmlChar {
    let mut q: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut val: c_int = 0;
    let mut len: c_int = 0;
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
    q = (*ctxt).cur;
    cur = q;
    val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    if !((if val < 0x100 as c_int {
        (0x41 as c_int <= val && val <= 0x5a as c_int
            || 0x61 as c_int <= val && val <= 0x7a as c_int
            || 0xc0 as c_int <= val && val <= 0xd6 as c_int
            || 0xd8 as c_int <= val && val <= 0xf6 as c_int
            || 0xf8 as c_int <= val) as c_int
    } else {
        xmlCharInRange(val as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if val < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= val && val <= 0x9fa5 as c_int
                || val == 0x3007 as c_int
                || 0x3021 as c_int <= val && val <= 0x3029 as c_int)
                as c_int
        }) != 0)
        && val != '_' as i32
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    while (if val < 0x100 as c_int {
        (0x41 as c_int <= val && val <= 0x5a as c_int
            || 0x61 as c_int <= val && val <= 0x7a as c_int
            || 0xc0 as c_int <= val && val <= 0xd6 as c_int
            || 0xd8 as c_int <= val && val <= 0xf6 as c_int
            || 0xf8 as c_int <= val) as c_int
    } else {
        xmlCharInRange(val as c_uint, &raw const xmlIsBaseCharGroup)
    }) != 0
        || (if val < 0x100 as c_int {
            0 as c_int
        } else {
            (0x4e00 as c_int <= val && val <= 0x9fa5 as c_int
                || val == 0x3007 as c_int
                || 0x3021 as c_int <= val && val <= 0x3029 as c_int)
                as c_int
        }) != 0
        || (if val < 0x100 as c_int {
            (0x30 as c_int <= val && val <= 0x39 as c_int)
                as c_int
        } else {
            xmlCharInRange(val as c_uint, &raw const xmlIsDigitGroup)
        }) != 0
        || val == '.' as i32
        || val == '-' as i32
        || val == '_' as i32
        || (if val < 0x100 as c_int {
            0 as c_int
        } else {
            xmlCharInRange(val as c_uint, &raw const xmlIsCombiningGroup)
        }) != 0
        || (if val < 0x100 as c_int {
            (val == 0xb7 as c_int) as c_int
        } else {
            xmlCharInRange(val as c_uint, &raw const xmlIsExtenderGroup)
        }) != 0
    {
        cur = cur.offset(len as isize);
        val = xmlStringCurrentChar(::core::ptr::null_mut::<xmlParserCtxt>(), cur, &raw mut len);
    }
    if !(*ctxt).dict.is_null() {
        ret = xmlDictLookup(
            (*ctxt).dict,
            q,
            cur.offset_from(q) as c_long as c_int,
        ) as *mut xmlChar;
    } else {
        ret = xmlStrndup(
            q,
            cur.offset_from(q) as c_long as c_int,
        );
    }
    (*ctxt).cur = cur;
    return ret;
}
unsafe extern "C" fn xmlCompileAttributeTest(mut ctxt: xmlPatParserContextPtr) {
    let mut current_block: u64;
    let mut token: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut URL: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
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
    name = xmlPatScanNCName(ctxt);
    if name.is_null() {
        if *(*ctxt).cur as c_int == '*' as i32 {
            if xmlPatternAdd(
                ctxt,
                (*ctxt).comp,
                XML_OP_ATTR,
                ::core::ptr::null_mut::<xmlChar>(),
                ::core::ptr::null_mut::<xmlChar>(),
            ) != 0
            {
                current_block = 1001200768659047971;
            } else {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
                current_block = 3640593987805443782;
            }
        } else {
            (*ctxt).error = 1 as c_int;
            current_block = 3640593987805443782;
        }
        match current_block {
            1001200768659047971 => {}
            _ => return,
        }
    } else {
        if *(*ctxt).cur as c_int == ':' as i32 {
            let mut i: c_int = 0;
            let mut prefix: *mut xmlChar = name;
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
            if *(*ctxt).cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *(*ctxt).cur as c_int
                    && *(*ctxt).cur as c_int <= 0xa as c_int
                || *(*ctxt).cur as c_int == 0xd as c_int
            {
                (*ctxt).error = 1 as c_int;
                current_block = 1001200768659047971;
            } else {
                token = xmlPatScanName(ctxt);
                if *prefix.offset(0 as c_int as isize) as c_int
                    == 'x' as i32
                    && *prefix.offset(1 as c_int as isize) as c_int
                        == 'm' as i32
                    && *prefix.offset(2 as c_int as isize) as c_int
                        == 'l' as i32
                    && *prefix.offset(3 as c_int as isize) as c_int
                        == 0 as c_int
                {
                    if !(*(*ctxt).comp).dict.is_null() {
                        URL = xmlDictLookup(
                            (*(*ctxt).comp).dict,
                            b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                as *const c_char
                                as *const xmlChar as *mut xmlChar,
                            -(1 as c_int),
                        ) as *mut xmlChar;
                    } else {
                        URL = xmlStrdup(
                            b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                as *const c_char
                                as *const xmlChar as *mut xmlChar,
                        );
                    }
                    current_block = 11459959175219260272;
                } else {
                    i = 0 as c_int;
                    while i < (*ctxt).nb_namespaces {
                        if xmlStrEqual(
                            *(*ctxt).namespaces.offset(
                                (2 as c_int * i + 1 as c_int) as isize,
                            ),
                            prefix,
                        ) != 0
                        {
                            if !(*(*ctxt).comp).dict.is_null() {
                                URL = xmlDictLookup(
                                    (*(*ctxt).comp).dict,
                                    *(*ctxt)
                                        .namespaces
                                        .offset((2 as c_int * i) as isize)
                                        as *mut xmlChar,
                                    -(1 as c_int),
                                ) as *mut xmlChar;
                            } else {
                                URL = xmlStrdup(
                                    *(*ctxt)
                                        .namespaces
                                        .offset((2 as c_int * i) as isize)
                                        as *mut xmlChar,
                                );
                            }
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    if i >= (*ctxt).nb_namespaces {
                        (*ctxt).error = 1 as c_int;
                        current_block = 1001200768659047971;
                    } else {
                        current_block = 11459959175219260272;
                    }
                }
                match current_block {
                    1001200768659047971 => {}
                    _ => {
                        if (*(*ctxt).comp).dict.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                name as *mut c_void,
                            );
                        }
                        name = ::core::ptr::null_mut::<xmlChar>();
                        if token.is_null() {
                            if *(*ctxt).cur as c_int == '*' as i32 {
                                if *(*ctxt).cur as c_int != 0 {
                                    (*ctxt).cur = (*ctxt).cur.offset(1);
                                } else {
                                };
                                if xmlPatternAdd(
                                    ctxt,
                                    (*ctxt).comp,
                                    XML_OP_ATTR,
                                    ::core::ptr::null_mut::<xmlChar>(),
                                    URL,
                                ) != 0
                                {
                                    current_block = 1001200768659047971;
                                } else {
                                    current_block = 12381812505308290051;
                                }
                            } else {
                                (*ctxt).error = 1 as c_int;
                                current_block = 1001200768659047971;
                            }
                        } else if xmlPatternAdd(ctxt, (*ctxt).comp, XML_OP_ATTR, token, URL) != 0 {
                            current_block = 1001200768659047971;
                        } else {
                            current_block = 12381812505308290051;
                        }
                    }
                }
            }
        } else if xmlPatternAdd(
            ctxt,
            (*ctxt).comp,
            XML_OP_ATTR,
            name,
            ::core::ptr::null_mut::<xmlChar>(),
        ) != 0
        {
            current_block = 1001200768659047971;
        } else {
            current_block = 12381812505308290051;
        }
        match current_block {
            1001200768659047971 => {}
            _ => return,
        }
    }
    if !name.is_null() {
        if (*(*ctxt).comp).dict.is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
        }
    }
    if !URL.is_null() {
        if (*(*ctxt).comp).dict.is_null() {
            xmlFree.expect("non-null function pointer")(URL as *mut c_void);
        }
    }
    if !token.is_null() {
        if (*(*ctxt).comp).dict.is_null() {
            xmlFree.expect("non-null function pointer")(token as *mut c_void);
        }
    }
}
unsafe extern "C" fn xmlCompileStepPattern(mut ctxt: xmlPatParserContextPtr) {
    let mut current_block: u64;
    let mut token: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut name: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut URL: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut hasBlanks: c_int = 0 as c_int;
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
    if *(*ctxt).cur as c_int == '.' as i32 {
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        if !(xmlPatternAdd(
            ctxt,
            (*ctxt).comp,
            XML_OP_ELEM,
            ::core::ptr::null_mut::<xmlChar>(),
            ::core::ptr::null_mut::<xmlChar>(),
        ) != 0)
        {
            return;
        }
    } else if *(*ctxt).cur as c_int == '@' as i32 {
        if (*(*ctxt).comp).flags & XML_PATTERN_XSSEL as c_int != 0 {
            (*ctxt).error = 1 as c_int;
            return;
        }
        if *(*ctxt).cur as c_int != 0 {
            (*ctxt).cur = (*ctxt).cur.offset(1);
        } else {
        };
        xmlCompileAttributeTest(ctxt);
        if !((*ctxt).error != 0 as c_int) {
            return;
        }
    } else {
        name = xmlPatScanNCName(ctxt);
        if name.is_null() {
            if *(*ctxt).cur as c_int == '*' as i32 {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
                if !(xmlPatternAdd(
                    ctxt,
                    (*ctxt).comp,
                    XML_OP_ALL,
                    ::core::ptr::null_mut::<xmlChar>(),
                    ::core::ptr::null_mut::<xmlChar>(),
                ) != 0)
                {
                    return;
                }
            } else {
                (*ctxt).error = 1 as c_int;
                return;
            }
        } else {
            if *(*ctxt).cur as c_int == 0x20 as c_int
                || 0x9 as c_int <= *(*ctxt).cur as c_int
                    && *(*ctxt).cur as c_int <= 0xa as c_int
                || *(*ctxt).cur as c_int == 0xd as c_int
            {
                hasBlanks = 1 as c_int;
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
            if *(*ctxt).cur as c_int == ':' as i32 {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
                if *(*ctxt).cur as c_int != ':' as i32 {
                    let mut prefix: *mut xmlChar = name;
                    let mut i: c_int = 0;
                    if hasBlanks != 0
                        || (*(*ctxt).cur as c_int == 0x20 as c_int
                            || 0x9 as c_int <= *(*ctxt).cur as c_int
                                && *(*ctxt).cur as c_int <= 0xa as c_int
                            || *(*ctxt).cur as c_int == 0xd as c_int)
                    {
                        (*ctxt).error = 1 as c_int;
                        current_block = 3808131046653677698;
                    } else {
                        token = xmlPatScanName(ctxt);
                        if *prefix.offset(0 as c_int as isize) as c_int
                            == 'x' as i32
                            && *prefix.offset(1 as c_int as isize)
                                as c_int
                                == 'm' as i32
                            && *prefix.offset(2 as c_int as isize)
                                as c_int
                                == 'l' as i32
                            && *prefix.offset(3 as c_int as isize)
                                as c_int
                                == 0 as c_int
                        {
                            if !(*(*ctxt).comp).dict.is_null() {
                                URL = xmlDictLookup(
                                    (*(*ctxt).comp).dict,
                                    b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                        as *const c_char
                                        as *const xmlChar
                                        as *mut xmlChar,
                                    -(1 as c_int),
                                ) as *mut xmlChar;
                            } else {
                                URL = xmlStrdup(
                                    b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                        as *const c_char
                                        as *const xmlChar
                                        as *mut xmlChar,
                                );
                            }
                            current_block = 1345366029464561491;
                        } else {
                            i = 0 as c_int;
                            while i < (*ctxt).nb_namespaces {
                                if xmlStrEqual(
                                    *(*ctxt).namespaces.offset(
                                        (2 as c_int * i + 1 as c_int)
                                            as isize,
                                    ),
                                    prefix,
                                ) != 0
                                {
                                    if !(*(*ctxt).comp).dict.is_null() {
                                        URL = xmlDictLookup(
                                            (*(*ctxt).comp).dict,
                                            *(*ctxt)
                                                .namespaces
                                                .offset((2 as c_int * i) as isize)
                                                as *mut xmlChar,
                                            -(1 as c_int),
                                        )
                                            as *mut xmlChar;
                                    } else {
                                        URL = xmlStrdup(
                                            *(*ctxt)
                                                .namespaces
                                                .offset((2 as c_int * i) as isize)
                                                as *mut xmlChar,
                                        );
                                    }
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                            if i >= (*ctxt).nb_namespaces {
                                (*ctxt).error = 1 as c_int;
                                current_block = 3808131046653677698;
                            } else {
                                current_block = 1345366029464561491;
                            }
                        }
                        match current_block {
                            3808131046653677698 => {}
                            _ => {
                                if (*(*ctxt).comp).dict.is_null() {
                                    xmlFree.expect("non-null function pointer")(
                                        prefix as *mut c_void,
                                    );
                                }
                                name = ::core::ptr::null_mut::<xmlChar>();
                                if token.is_null() {
                                    if *(*ctxt).cur as c_int == '*' as i32 {
                                        if *(*ctxt).cur as c_int != 0 {
                                            (*ctxt).cur = (*ctxt).cur.offset(1);
                                        } else {
                                        };
                                        if xmlPatternAdd(
                                            ctxt,
                                            (*ctxt).comp,
                                            XML_OP_NS,
                                            URL,
                                            ::core::ptr::null_mut::<xmlChar>(),
                                        ) != 0
                                        {
                                            current_block = 3808131046653677698;
                                        } else {
                                            current_block = 16910810822589621899;
                                        }
                                    } else {
                                        (*ctxt).error = 1 as c_int;
                                        current_block = 3808131046653677698;
                                    }
                                } else if xmlPatternAdd(ctxt, (*ctxt).comp, XML_OP_ELEM, token, URL)
                                    != 0
                                {
                                    current_block = 3808131046653677698;
                                } else {
                                    current_block = 16910810822589621899;
                                }
                            }
                        }
                    }
                } else {
                    if *(*ctxt).cur as c_int != 0 {
                        (*ctxt).cur = (*ctxt).cur.offset(1);
                    } else {
                    };
                    if xmlStrEqual(
                        name,
                        b"child\0" as *const u8 as *const c_char as *const xmlChar,
                    ) != 0
                    {
                        if (*(*ctxt).comp).dict.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                name as *mut c_void,
                            );
                        }
                        name = xmlPatScanName(ctxt);
                        if name.is_null() {
                            if *(*ctxt).cur as c_int == '*' as i32 {
                                if *(*ctxt).cur as c_int != 0 {
                                    (*ctxt).cur = (*ctxt).cur.offset(1);
                                } else {
                                };
                                if !(xmlPatternAdd(
                                    ctxt,
                                    (*ctxt).comp,
                                    XML_OP_ALL,
                                    ::core::ptr::null_mut::<xmlChar>(),
                                    ::core::ptr::null_mut::<xmlChar>(),
                                ) != 0)
                                {
                                    return;
                                }
                            } else {
                                (*ctxt).error = 1 as c_int;
                            }
                        } else {
                            if *(*ctxt).cur as c_int == ':' as i32 {
                                let mut prefix_0: *mut xmlChar = name;
                                let mut i_0: c_int = 0;
                                if *(*ctxt).cur as c_int != 0 {
                                    (*ctxt).cur = (*ctxt).cur.offset(1);
                                } else {
                                };
                                if *(*ctxt).cur as c_int == 0x20 as c_int
                                    || 0x9 as c_int
                                        <= *(*ctxt).cur as c_int
                                        && *(*ctxt).cur as c_int
                                            <= 0xa as c_int
                                    || *(*ctxt).cur as c_int
                                        == 0xd as c_int
                                {
                                    (*ctxt).error = 1 as c_int;
                                    current_block = 3808131046653677698;
                                } else {
                                    token = xmlPatScanName(ctxt);
                                    if *prefix_0.offset(0 as c_int as isize)
                                        as c_int
                                        == 'x' as i32
                                        && *prefix_0.offset(1 as c_int as isize)
                                            as c_int
                                            == 'm' as i32
                                        && *prefix_0.offset(2 as c_int as isize)
                                            as c_int
                                            == 'l' as i32
                                        && *prefix_0.offset(3 as c_int as isize)
                                            as c_int
                                            == 0 as c_int
                                    {
                                        if !(*(*ctxt).comp).dict.is_null() {
                                            URL = xmlDictLookup(
                                                (*(*ctxt).comp).dict,
                                                b"http://www.w3.org/XML/1998/namespace\0"
                                                    as *const u8
                                                    as *const c_char
                                                    as *const xmlChar
                                                    as *mut xmlChar,
                                                -(1 as c_int),
                                            )
                                                as *mut xmlChar;
                                        } else {
                                            URL = xmlStrdup(
                                                b"http://www.w3.org/XML/1998/namespace\0"
                                                    as *const u8
                                                    as *const c_char
                                                    as *const xmlChar
                                                    as *mut xmlChar,
                                            );
                                        }
                                        current_block = 9505035279996566320;
                                    } else {
                                        i_0 = 0 as c_int;
                                        while i_0 < (*ctxt).nb_namespaces {
                                            if xmlStrEqual(
                                                *(*ctxt).namespaces.offset(
                                                    (2 as c_int * i_0
                                                        + 1 as c_int)
                                                        as isize,
                                                ),
                                                prefix_0,
                                            ) != 0
                                            {
                                                if !(*(*ctxt).comp).dict.is_null() {
                                                    URL = xmlDictLookup(
                                                        (*(*ctxt).comp).dict,
                                                        *(*ctxt).namespaces.offset(
                                                            (2 as c_int * i_0)
                                                                as isize,
                                                        )
                                                            as *mut xmlChar,
                                                        -(1 as c_int),
                                                    )
                                                        as *mut xmlChar;
                                                } else {
                                                    URL = xmlStrdup(*(*ctxt).namespaces.offset(
                                                        (2 as c_int * i_0) as isize,
                                                    )
                                                        as *mut xmlChar);
                                                }
                                                break;
                                            } else {
                                                i_0 += 1;
                                            }
                                        }
                                        if i_0 >= (*ctxt).nb_namespaces {
                                            (*ctxt).error = 1 as c_int;
                                            current_block = 3808131046653677698;
                                        } else {
                                            current_block = 9505035279996566320;
                                        }
                                    }
                                    match current_block {
                                        3808131046653677698 => {}
                                        _ => {
                                            if (*(*ctxt).comp).dict.is_null() {
                                                xmlFree.expect("non-null function pointer")(
                                                    prefix_0 as *mut c_void,
                                                );
                                            }
                                            name = ::core::ptr::null_mut::<xmlChar>();
                                            if token.is_null() {
                                                if *(*ctxt).cur as c_int == '*' as i32
                                                {
                                                    if *(*ctxt).cur as c_int != 0 {
                                                        (*ctxt).cur = (*ctxt).cur.offset(1);
                                                    } else {
                                                    };
                                                    if xmlPatternAdd(
                                                        ctxt,
                                                        (*ctxt).comp,
                                                        XML_OP_NS,
                                                        URL,
                                                        ::core::ptr::null_mut::<xmlChar>(),
                                                    ) != 0
                                                    {
                                                        current_block = 3808131046653677698;
                                                    } else {
                                                        current_block = 5265702136860997526;
                                                    }
                                                } else {
                                                    (*ctxt).error = 1 as c_int;
                                                    current_block = 3808131046653677698;
                                                }
                                            } else if xmlPatternAdd(
                                                ctxt,
                                                (*ctxt).comp,
                                                XML_OP_CHILD,
                                                token,
                                                URL,
                                            ) != 0
                                            {
                                                current_block = 3808131046653677698;
                                            } else {
                                                current_block = 5265702136860997526;
                                            }
                                        }
                                    }
                                }
                            } else if xmlPatternAdd(
                                ctxt,
                                (*ctxt).comp,
                                XML_OP_CHILD,
                                name,
                                ::core::ptr::null_mut::<xmlChar>(),
                            ) != 0
                            {
                                current_block = 3808131046653677698;
                            } else {
                                current_block = 5265702136860997526;
                            }
                            match current_block {
                                3808131046653677698 => {}
                                _ => return,
                            }
                        }
                    } else if xmlStrEqual(
                        name,
                        b"attribute\0" as *const u8 as *const c_char as *const xmlChar,
                    ) != 0
                    {
                        if (*(*ctxt).comp).dict.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                name as *mut c_void,
                            );
                        }
                        name = ::core::ptr::null_mut::<xmlChar>();
                        if (*(*ctxt).comp).flags & XML_PATTERN_XSSEL as c_int != 0 {
                            (*ctxt).error = 1 as c_int;
                        } else {
                            xmlCompileAttributeTest(ctxt);
                            if !((*ctxt).error != 0 as c_int) {
                                return;
                            }
                        }
                    } else {
                        (*ctxt).error = 1 as c_int;
                    }
                    current_block = 3808131046653677698;
                }
            } else if *(*ctxt).cur as c_int == '*' as i32 {
                if !name.is_null() {
                    (*ctxt).error = 1 as c_int;
                    current_block = 3808131046653677698;
                } else {
                    if *(*ctxt).cur as c_int != 0 {
                        (*ctxt).cur = (*ctxt).cur.offset(1);
                    } else {
                    };
                    if xmlPatternAdd(
                        ctxt,
                        (*ctxt).comp,
                        XML_OP_ALL,
                        token,
                        ::core::ptr::null_mut::<xmlChar>(),
                    ) != 0
                    {
                        current_block = 3808131046653677698;
                    } else {
                        current_block = 16910810822589621899;
                    }
                }
            } else if xmlPatternAdd(
                ctxt,
                (*ctxt).comp,
                XML_OP_ELEM,
                name,
                ::core::ptr::null_mut::<xmlChar>(),
            ) != 0
            {
                current_block = 3808131046653677698;
            } else {
                current_block = 16910810822589621899;
            }
            match current_block {
                3808131046653677698 => {}
                _ => return,
            }
        }
    }
    if !URL.is_null() {
        if (*(*ctxt).comp).dict.is_null() {
            xmlFree.expect("non-null function pointer")(URL as *mut c_void);
        }
    }
    if !token.is_null() {
        if (*(*ctxt).comp).dict.is_null() {
            xmlFree.expect("non-null function pointer")(token as *mut c_void);
        }
    }
    if !name.is_null() {
        if (*(*ctxt).comp).dict.is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut c_void);
        }
    }
}
unsafe extern "C" fn xmlCompilePathPattern(mut ctxt: xmlPatParserContextPtr) {
    let mut current_block: u64;
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
    if *(*ctxt).cur as c_int == '/' as i32 {
        (*(*ctxt).comp).flags |= PAT_FROM_ROOT;
    } else if *(*ctxt).cur as c_int == '.' as i32
        || (*(*ctxt).comp).flags
            & (XML_PATTERN_XPATH as c_int
                | XML_PATTERN_XSSEL as c_int
                | XML_PATTERN_XSFIELD as c_int)
            != 0
    {
        (*(*ctxt).comp).flags |= PAT_FROM_CUR;
    }
    if *(*ctxt).cur as c_int == '/' as i32
        && *(*ctxt).cur.offset(1 as c_int as isize) as c_int == '/' as i32
    {
        if xmlPatternAdd(
            ctxt,
            (*ctxt).comp,
            XML_OP_ANCESTOR,
            ::core::ptr::null_mut::<xmlChar>(),
            ::core::ptr::null_mut::<xmlChar>(),
        ) != 0
        {
            current_block = 11107567411178369866;
        } else {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
            current_block = 4808432441040389987;
        }
    } else if *(*ctxt).cur as c_int == '.' as i32
        && *(*ctxt).cur.offset(1 as c_int as isize) as c_int == '/' as i32
        && *(*ctxt).cur.offset(2 as c_int as isize) as c_int == '/' as i32
    {
        if xmlPatternAdd(
            ctxt,
            (*ctxt).comp,
            XML_OP_ANCESTOR,
            ::core::ptr::null_mut::<xmlChar>(),
            ::core::ptr::null_mut::<xmlChar>(),
        ) != 0
        {
            current_block = 11107567411178369866;
        } else {
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
            if *(*ctxt).cur as c_int != 0 {
                (*ctxt).cur = (*ctxt).cur.offset(1);
            } else {
            };
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
            if *(*ctxt).cur as c_int == 0 as c_int {
                (*ctxt).error = 1 as c_int;
                current_block = 11107567411178369866;
            } else {
                current_block = 4808432441040389987;
            }
        }
    } else {
        current_block = 4808432441040389987;
    }
    match current_block {
        4808432441040389987 => {
            if *(*ctxt).cur as c_int == '@' as i32 {
                if *(*ctxt).cur as c_int != 0 {
                    (*ctxt).cur = (*ctxt).cur.offset(1);
                } else {
                };
                xmlCompileAttributeTest(ctxt);
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
                if *(*ctxt).cur as c_int != 0 as c_int {
                    xmlCompileStepPattern(ctxt);
                    if (*ctxt).error != 0 as c_int {
                        current_block = 11107567411178369866;
                    } else {
                        current_block = 5159818223158340697;
                    }
                } else {
                    current_block = 5159818223158340697;
                }
            } else {
                if *(*ctxt).cur as c_int == '/' as i32 {
                    if xmlPatternAdd(
                        ctxt,
                        (*ctxt).comp,
                        XML_OP_ROOT,
                        ::core::ptr::null_mut::<xmlChar>(),
                        ::core::ptr::null_mut::<xmlChar>(),
                    ) != 0
                    {
                        current_block = 11107567411178369866;
                    } else {
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
                        if *(*ctxt).cur as c_int == 0 as c_int {
                            (*ctxt).error = 1 as c_int;
                            current_block = 11107567411178369866;
                        } else {
                            current_block = 7828949454673616476;
                        }
                    }
                } else {
                    current_block = 7828949454673616476;
                }
                match current_block {
                    11107567411178369866 => {}
                    _ => {
                        xmlCompileStepPattern(ctxt);
                        if (*ctxt).error != 0 as c_int {
                            current_block = 11107567411178369866;
                        } else {
                            while *(*ctxt).cur as c_int == 0x20 as c_int
                                || 0x9 as c_int <= *(*ctxt).cur as c_int
                                    && *(*ctxt).cur as c_int
                                        <= 0xa as c_int
                                || *(*ctxt).cur as c_int == 0xd as c_int
                            {
                                if *(*ctxt).cur as c_int != 0 {
                                    (*ctxt).cur = (*ctxt).cur.offset(1);
                                } else {
                                };
                            }
                            loop {
                                if !(*(*ctxt).cur as c_int == '/' as i32) {
                                    current_block = 5159818223158340697;
                                    break;
                                }
                                if *(*ctxt).cur.offset(1 as c_int as isize)
                                    as c_int
                                    == '/' as i32
                                {
                                    if xmlPatternAdd(
                                        ctxt,
                                        (*ctxt).comp,
                                        XML_OP_ANCESTOR,
                                        ::core::ptr::null_mut::<xmlChar>(),
                                        ::core::ptr::null_mut::<xmlChar>(),
                                    ) != 0
                                    {
                                        current_block = 11107567411178369866;
                                        break;
                                    }
                                    if *(*ctxt).cur as c_int != 0 {
                                        (*ctxt).cur = (*ctxt).cur.offset(1);
                                    } else {
                                    };
                                    if *(*ctxt).cur as c_int != 0 {
                                        (*ctxt).cur = (*ctxt).cur.offset(1);
                                    } else {
                                    };
                                    while *(*ctxt).cur as c_int
                                        == 0x20 as c_int
                                        || 0x9 as c_int
                                            <= *(*ctxt).cur as c_int
                                            && *(*ctxt).cur as c_int
                                                <= 0xa as c_int
                                        || *(*ctxt).cur as c_int
                                            == 0xd as c_int
                                    {
                                        if *(*ctxt).cur as c_int != 0 {
                                            (*ctxt).cur = (*ctxt).cur.offset(1);
                                        } else {
                                        };
                                    }
                                    xmlCompileStepPattern(ctxt);
                                    if (*ctxt).error != 0 as c_int {
                                        current_block = 11107567411178369866;
                                        break;
                                    }
                                } else {
                                    if xmlPatternAdd(
                                        ctxt,
                                        (*ctxt).comp,
                                        XML_OP_PARENT,
                                        ::core::ptr::null_mut::<xmlChar>(),
                                        ::core::ptr::null_mut::<xmlChar>(),
                                    ) != 0
                                    {
                                        current_block = 11107567411178369866;
                                        break;
                                    }
                                    if *(*ctxt).cur as c_int != 0 {
                                        (*ctxt).cur = (*ctxt).cur.offset(1);
                                    } else {
                                    };
                                    while *(*ctxt).cur as c_int
                                        == 0x20 as c_int
                                        || 0x9 as c_int
                                            <= *(*ctxt).cur as c_int
                                            && *(*ctxt).cur as c_int
                                                <= 0xa as c_int
                                        || *(*ctxt).cur as c_int
                                            == 0xd as c_int
                                    {
                                        if *(*ctxt).cur as c_int != 0 {
                                            (*ctxt).cur = (*ctxt).cur.offset(1);
                                        } else {
                                        };
                                    }
                                    if *(*ctxt).cur as c_int == 0 as c_int
                                    {
                                        (*ctxt).error = 1 as c_int;
                                        current_block = 11107567411178369866;
                                        break;
                                    } else {
                                        xmlCompileStepPattern(ctxt);
                                        if (*ctxt).error != 0 as c_int {
                                            current_block = 11107567411178369866;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                11107567411178369866 => {}
                _ => {
                    if *(*ctxt).cur as c_int != 0 as c_int {
                        (*ctxt).error = 1 as c_int;
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn xmlCompileIDCXPathPath(mut ctxt: xmlPatParserContextPtr) {
    let mut current_block: u64;
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
    if !(*(*ctxt).cur as c_int == '/' as i32) {
        (*(*ctxt).comp).flags |= PAT_FROM_CUR;
        if *(*ctxt).cur as c_int == '.' as i32 {
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
            if *(*ctxt).cur as c_int == 0 as c_int {
                if xmlPatternAdd(
                    ctxt,
                    (*ctxt).comp,
                    XML_OP_ELEM,
                    ::core::ptr::null_mut::<xmlChar>(),
                    ::core::ptr::null_mut::<xmlChar>(),
                ) != 0
                {
                    current_block = 14193453886395082364;
                } else {
                    return;
                }
            } else if *(*ctxt).cur as c_int != '/' as i32 {
                current_block = 14193453886395082364;
            } else {
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
                if *(*ctxt).cur as c_int == '/' as i32 {
                    if *(*ctxt).cur.offset(-(1 as c_int) as isize)
                        as c_int
                        == 0x20 as c_int
                        || 0x9 as c_int
                            <= *(*ctxt).cur.offset(-(1 as c_int) as isize)
                                as c_int
                            && *(*ctxt).cur.offset(-(1 as c_int) as isize)
                                as c_int
                                <= 0xa as c_int
                        || *(*ctxt).cur.offset(-(1 as c_int) as isize)
                            as c_int
                            == 0xd as c_int
                    {
                        current_block = 14193453886395082364;
                    } else if xmlPatternAdd(
                        ctxt,
                        (*ctxt).comp,
                        XML_OP_ANCESTOR,
                        ::core::ptr::null_mut::<xmlChar>(),
                        ::core::ptr::null_mut::<xmlChar>(),
                    ) != 0
                    {
                        current_block = 14193453886395082364;
                    } else {
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
                        current_block = 11042950489265723346;
                    }
                } else {
                    current_block = 11042950489265723346;
                }
                match current_block {
                    14193453886395082364 => {}
                    _ => {
                        if *(*ctxt).cur as c_int == 0 as c_int {
                            current_block = 2955554247575894110;
                        } else {
                            current_block = 2719512138335094285;
                        }
                    }
                }
            }
        } else {
            current_block = 2719512138335094285;
        }
        match current_block {
            14193453886395082364 => {}
            _ => {
                loop {
                    match current_block {
                        2955554247575894110 => {
                            (*ctxt).error = 1 as c_int;
                            return;
                        }
                        _ => {
                            xmlCompileStepPattern(ctxt);
                            if (*ctxt).error != 0 as c_int {
                                current_block = 14193453886395082364;
                                break;
                            }
                            while *(*ctxt).cur as c_int == 0x20 as c_int
                                || 0x9 as c_int <= *(*ctxt).cur as c_int
                                    && *(*ctxt).cur as c_int
                                        <= 0xa as c_int
                                || *(*ctxt).cur as c_int == 0xd as c_int
                            {
                                if *(*ctxt).cur as c_int != 0 {
                                    (*ctxt).cur = (*ctxt).cur.offset(1);
                                } else {
                                };
                            }
                            if *(*ctxt).cur as c_int != '/' as i32 {
                                current_block = 8180496224585318153;
                                break;
                            }
                            if xmlPatternAdd(
                                ctxt,
                                (*ctxt).comp,
                                XML_OP_PARENT,
                                ::core::ptr::null_mut::<xmlChar>(),
                                ::core::ptr::null_mut::<xmlChar>(),
                            ) != 0
                            {
                                current_block = 14193453886395082364;
                                break;
                            }
                            if *(*ctxt).cur as c_int != 0 {
                                (*ctxt).cur = (*ctxt).cur.offset(1);
                            } else {
                            };
                            while *(*ctxt).cur as c_int == 0x20 as c_int
                                || 0x9 as c_int <= *(*ctxt).cur as c_int
                                    && *(*ctxt).cur as c_int
                                        <= 0xa as c_int
                                || *(*ctxt).cur as c_int == 0xd as c_int
                            {
                                if *(*ctxt).cur as c_int != 0 {
                                    (*ctxt).cur = (*ctxt).cur.offset(1);
                                } else {
                                };
                            }
                            if *(*ctxt).cur as c_int == '/' as i32 {
                                current_block = 14193453886395082364;
                                break;
                            }
                            if *(*ctxt).cur as c_int == 0 as c_int {
                                current_block = 2955554247575894110;
                                continue;
                            }
                            if *(*ctxt).cur as c_int != 0 as c_int {
                                current_block = 2719512138335094285;
                            } else {
                                current_block = 8180496224585318153;
                                break;
                            }
                        }
                    }
                }
                match current_block {
                    14193453886395082364 => {}
                    _ => {
                        if *(*ctxt).cur as c_int != 0 as c_int {
                            (*ctxt).error = 1 as c_int;
                        }
                        return;
                    }
                }
            }
        }
    }
    (*ctxt).error = 1 as c_int;
}
unsafe extern "C" fn xmlNewStreamComp(mut size: c_int) -> xmlStreamCompPtr {
    let mut cur: xmlStreamCompPtr = ::core::ptr::null_mut::<xmlStreamComp>();
    if size < 4 as c_int {
        size = 4 as c_int;
    }
    cur = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlStreamComp>() as size_t
    ) as xmlStreamCompPtr;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlStreamComp>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlStreamComp>() as size_t,
    );
    (*cur).steps = xmlMalloc.expect("non-null function pointer")(
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlStreamStep>() as size_t),
    ) as xmlStreamStepPtr;
    if (*cur).steps.is_null() {
        xmlFree.expect("non-null function pointer")(cur as *mut c_void);
        return ::core::ptr::null_mut::<xmlStreamComp>();
    }
    (*cur).nbStep = 0 as c_int;
    (*cur).maxStep = size;
    return cur;
}
unsafe extern "C" fn xmlFreeStreamComp(mut comp: xmlStreamCompPtr) {
    if !comp.is_null() {
        if !(*comp).steps.is_null() {
            xmlFree.expect("non-null function pointer")((*comp).steps as *mut c_void);
        }
        if !(*comp).dict.is_null() {
            xmlDictFree((*comp).dict as xmlDictPtr);
        }
        xmlFree.expect("non-null function pointer")(comp as *mut c_void);
    }
}
unsafe extern "C" fn xmlStreamCompAddStep(
    mut comp: xmlStreamCompPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
    mut nodeType: c_int,
    mut flags: c_int,
) -> c_int {
    let mut cur: xmlStreamStepPtr = ::core::ptr::null_mut::<xmlStreamStep>();
    if (*comp).nbStep >= (*comp).maxStep {
        cur = xmlRealloc.expect("non-null function pointer")(
            (*comp).steps as *mut c_void,
            (((*comp).maxStep * 2 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlStreamStep>() as size_t),
        ) as xmlStreamStepPtr;
        if cur.is_null() {
            return -(1 as c_int);
        }
        (*comp).steps = cur;
        (*comp).maxStep *= 2 as c_int;
    }
    let fresh9 = (*comp).nbStep;
    (*comp).nbStep = (*comp).nbStep + 1;
    cur = (*comp).steps.offset(fresh9 as isize) as *mut xmlStreamStep as xmlStreamStepPtr;
    (*cur).flags = flags;
    (*cur).name = name;
    (*cur).ns = ns;
    (*cur).nodeType = nodeType;
    return (*comp).nbStep - 1 as c_int;
}
unsafe extern "C" fn xmlStreamCompile(mut comp: xmlPatternPtr) -> c_int {
    let mut current_block: u64;
    let mut stream: xmlStreamCompPtr = ::core::ptr::null_mut::<xmlStreamComp>();
    let mut i: c_int = 0;
    let mut s: c_int = 0 as c_int;
    let mut root: c_int = 0 as c_int;
    let mut flags: c_int = 0 as c_int;
    let mut prevs: c_int = -(1 as c_int);
    let mut step: xmlStepOp = xmlStepOp {
        op: XML_OP_END,
        value: ::core::ptr::null::<xmlChar>(),
        value2: ::core::ptr::null::<xmlChar>(),
    };
    if comp.is_null() || (*comp).steps.is_null() {
        return -(1 as c_int);
    }
    if (*comp).nbStep == 1 as c_int
        && (*(*comp).steps.offset(0 as c_int as isize)).op as c_uint
            == XML_OP_ELEM as c_int as c_uint
        && (*(*comp).steps.offset(0 as c_int as isize))
            .value
            .is_null()
        && (*(*comp).steps.offset(0 as c_int as isize))
            .value2
            .is_null()
    {
        stream = xmlNewStreamComp(0 as c_int);
        if stream.is_null() {
            return -(1 as c_int);
        }
        (*stream).flags |= XML_STREAM_FINAL_IS_ANY_NODE;
        (*comp).stream = stream;
        return 0 as c_int;
    }
    stream = xmlNewStreamComp((*comp).nbStep / 2 as c_int + 1 as c_int);
    if stream.is_null() {
        return -(1 as c_int);
    }
    if !(*comp).dict.is_null() {
        (*stream).dict = (*comp).dict as *mut xmlDict;
        xmlDictReference((*stream).dict as xmlDictPtr);
    }
    i = 0 as c_int;
    if (*comp).flags & PAT_FROM_ROOT != 0 {
        (*stream).flags |= XML_STREAM_FROM_ROOT;
    }
    loop {
        if !(i < (*comp).nbStep) {
            current_block = 10380409671385728102;
            break;
        }
        step = *(*comp).steps.offset(i as isize);
        match step.op as c_uint {
            1 => {
                if i != 0 as c_int {
                    current_block = 11812692070449465682;
                    break;
                }
                root = 1 as c_int;
            }
            7 => {
                s = xmlStreamCompAddStep(
                    stream,
                    ::core::ptr::null::<xmlChar>(),
                    step.value,
                    XML_ELEMENT_NODE as c_int,
                    flags,
                );
                if s < 0 as c_int {
                    current_block = 11812692070449465682;
                    break;
                }
                prevs = s;
                flags = 0 as c_int;
            }
            4 => {
                flags |= XML_STREAM_STEP_ATTR;
                prevs = -(1 as c_int);
                s = xmlStreamCompAddStep(
                    stream,
                    step.value,
                    step.value2,
                    XML_ATTRIBUTE_NODE as c_int,
                    flags,
                );
                flags = 0 as c_int;
                if s < 0 as c_int {
                    current_block = 11812692070449465682;
                    break;
                }
            }
            2 => {
                if step.value.is_null() && step.value2.is_null() {
                    if (*comp).nbStep == i + 1 as c_int
                        && flags & XML_STREAM_STEP_DESC != 0
                    {
                        if (*comp).nbStep == i + 1 as c_int {
                            (*stream).flags |= XML_STREAM_FINAL_IS_ANY_NODE;
                        }
                        flags |= XML_STREAM_STEP_NODE;
                        s = xmlStreamCompAddStep(
                            stream,
                            ::core::ptr::null::<xmlChar>(),
                            ::core::ptr::null::<xmlChar>(),
                            XML_STREAM_ANY_NODE,
                            flags,
                        );
                        if s < 0 as c_int {
                            current_block = 11812692070449465682;
                            break;
                        }
                        flags = 0 as c_int;
                        if prevs != -(1 as c_int) {
                            (*(*stream).steps.offset(prevs as isize)).flags |=
                                XML_STREAM_STEP_IN_SET;
                            prevs = -(1 as c_int);
                        }
                    }
                } else {
                    s = xmlStreamCompAddStep(
                        stream,
                        step.value,
                        step.value2,
                        XML_ELEMENT_NODE as c_int,
                        flags,
                    );
                    if s < 0 as c_int {
                        current_block = 11812692070449465682;
                        break;
                    }
                    prevs = s;
                    flags = 0 as c_int;
                }
            }
            3 => {
                s = xmlStreamCompAddStep(
                    stream,
                    step.value,
                    step.value2,
                    XML_ELEMENT_NODE as c_int,
                    flags,
                );
                if s < 0 as c_int {
                    current_block = 11812692070449465682;
                    break;
                }
                prevs = s;
                flags = 0 as c_int;
            }
            8 => {
                s = xmlStreamCompAddStep(
                    stream,
                    ::core::ptr::null::<xmlChar>(),
                    ::core::ptr::null::<xmlChar>(),
                    XML_ELEMENT_NODE as c_int,
                    flags,
                );
                if s < 0 as c_int {
                    current_block = 11812692070449465682;
                    break;
                }
                prevs = s;
                flags = 0 as c_int;
            }
            6 => {
                if !(flags & XML_STREAM_STEP_DESC != 0) {
                    flags |= XML_STREAM_STEP_DESC;
                    if (*stream).flags & XML_STREAM_DESC == 0 as c_int {
                        (*stream).flags |= XML_STREAM_DESC;
                    }
                }
            }
            0 | 5 | _ => {}
        }
        i += 1;
    }
    match current_block {
        10380409671385728102 => {
            if root == 0
                && (*comp).flags
                    & (XML_PATTERN_XPATH as c_int
                        | XML_PATTERN_XSSEL as c_int
                        | XML_PATTERN_XSFIELD as c_int)
                    == 0 as c_int
            {
                if (*stream).flags & XML_STREAM_DESC == 0 as c_int {
                    (*stream).flags |= XML_STREAM_DESC;
                }
                if (*stream).nbStep > 0 as c_int {
                    if (*(*stream).steps.offset(0 as c_int as isize)).flags
                        & XML_STREAM_STEP_DESC
                        == 0 as c_int
                    {
                        (*(*stream).steps.offset(0 as c_int as isize)).flags |=
                            XML_STREAM_STEP_DESC;
                    }
                }
            }
            if !((*stream).nbStep <= s) {
                (*(*stream).steps.offset(s as isize)).flags |= XML_STREAM_STEP_FINAL;
                if root != 0 {
                    (*(*stream).steps.offset(0 as c_int as isize)).flags |=
                        XML_STREAM_STEP_ROOT;
                }
                (*comp).stream = stream;
                return 0 as c_int;
            }
        }
        _ => {}
    }
    xmlFreeStreamComp(stream);
    return 0 as c_int;
}
unsafe extern "C" fn xmlNewStreamCtxt(mut stream: xmlStreamCompPtr) -> xmlStreamCtxtPtr {
    let mut cur: xmlStreamCtxtPtr = ::core::ptr::null_mut::<xmlStreamCtxt>();
    cur = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlStreamCtxt>() as size_t
    ) as xmlStreamCtxtPtr;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlStreamCtxt>();
    }
    memset(
        cur as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<xmlStreamCtxt>() as size_t,
    );
    (*cur).states = xmlMalloc.expect("non-null function pointer")(
        ((4 as c_int * 2 as c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
    ) as *mut c_int;
    if (*cur).states.is_null() {
        xmlFree.expect("non-null function pointer")(cur as *mut c_void);
        return ::core::ptr::null_mut::<xmlStreamCtxt>();
    }
    (*cur).nbState = 0 as c_int;
    (*cur).maxState = 4 as c_int;
    (*cur).level = 0 as c_int;
    (*cur).comp = stream;
    (*cur).blockLevel = -(1 as c_int);
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeStreamCtxt(mut stream: xmlStreamCtxtPtr) {
    let mut next: xmlStreamCtxtPtr = ::core::ptr::null_mut::<xmlStreamCtxt>();
    while !stream.is_null() {
        next = (*stream).next as xmlStreamCtxtPtr;
        if !(*stream).states.is_null() {
            xmlFree.expect("non-null function pointer")(
                (*stream).states as *mut c_void,
            );
        }
        xmlFree.expect("non-null function pointer")(stream as *mut c_void);
        stream = next;
    }
}
unsafe extern "C" fn xmlStreamCtxtAddState(
    mut comp: xmlStreamCtxtPtr,
    mut idx: c_int,
    mut level: c_int,
) -> c_int {
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < (*comp).nbState {
        if *(*comp)
            .states
            .offset((2 as c_int * i) as isize)
            < 0 as c_int
        {
            *(*comp)
                .states
                .offset((2 as c_int * i) as isize) = idx;
            *(*comp)
                .states
                .offset((2 as c_int * i + 1 as c_int) as isize) = level;
            return i;
        }
        i += 1;
    }
    if (*comp).nbState >= (*comp).maxState {
        let mut cur: *mut c_int = ::core::ptr::null_mut::<c_int>();
        cur = xmlRealloc.expect("non-null function pointer")(
            (*comp).states as *mut c_void,
            (((*comp).maxState * 4 as c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
        ) as *mut c_int;
        if cur.is_null() {
            return -(1 as c_int);
        }
        (*comp).states = cur;
        (*comp).maxState *= 2 as c_int;
    }
    *(*comp)
        .states
        .offset((2 as c_int * (*comp).nbState) as isize) = idx;
    let fresh14 = (*comp).nbState;
    (*comp).nbState = (*comp).nbState + 1;
    *(*comp)
        .states
        .offset((2 as c_int * fresh14 + 1 as c_int) as isize) = level;
    return (*comp).nbState - 1 as c_int;
}
unsafe extern "C" fn xmlStreamPushInternal(
    mut stream: xmlStreamCtxtPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
    mut nodeType: c_int,
) -> c_int {
    let mut current_block: u64;
    let mut ret: c_int = 0 as c_int;
    let mut err: c_int = 0 as c_int;
    let mut final_0: c_int = 0 as c_int;
    let mut tmp: c_int = 0;
    let mut i: c_int = 0;
    let mut m: c_int = 0;
    let mut match_0: c_int = 0;
    let mut stepNr: c_int = 0;
    let mut desc: c_int = 0;
    let mut comp: xmlStreamCompPtr = ::core::ptr::null_mut::<xmlStreamComp>();
    let mut step: xmlStreamStep = xmlStreamStep {
        flags: 0,
        name: ::core::ptr::null::<xmlChar>(),
        ns: ::core::ptr::null::<xmlChar>(),
        nodeType: 0,
    };
    if stream.is_null() || (*stream).nbState < 0 as c_int {
        return -(1 as c_int);
    }
    while !stream.is_null() {
        comp = (*stream).comp;
        if nodeType == XML_ELEMENT_NODE as c_int && name.is_null() && ns.is_null() {
            (*stream).nbState = 0 as c_int;
            (*stream).level = 0 as c_int;
            (*stream).blockLevel = -(1 as c_int);
            if (*comp).flags & XML_STREAM_FROM_ROOT != 0 {
                if (*comp).nbStep == 0 as c_int {
                    ret = 1 as c_int;
                } else if (*comp).nbStep == 1 as c_int
                    && (*(*comp).steps.offset(0 as c_int as isize)).nodeType
                        == XML_STREAM_ANY_NODE
                    && (*(*comp).steps.offset(0 as c_int as isize)).flags
                        & XML_STREAM_STEP_DESC
                        != 0
                {
                    ret = 1 as c_int;
                } else if (*(*comp).steps.offset(0 as c_int as isize)).flags
                    & XML_STREAM_STEP_ROOT
                    != 0
                {
                    tmp = xmlStreamCtxtAddState(
                        stream,
                        0 as c_int,
                        0 as c_int,
                    );
                    if tmp < 0 as c_int {
                        err += 1;
                    }
                }
            }
            stream = (*stream).next as xmlStreamCtxtPtr;
        } else {
            if (*comp).nbStep == 0 as c_int {
                if (*stream).flags & XML_PATTERN_XPATH as c_int != 0 {
                    stream = (*stream).next as xmlStreamCtxtPtr;
                    continue;
                } else {
                    if nodeType != XML_ATTRIBUTE_NODE as c_int
                        && ((*stream).flags
                            & (XML_PATTERN_XPATH as c_int
                                | XML_PATTERN_XSSEL as c_int
                                | XML_PATTERN_XSFIELD as c_int)
                            == 0 as c_int
                            || (*stream).level == 0 as c_int)
                    {
                        ret = 1 as c_int;
                    }
                    (*stream).level += 1;
                }
            } else if (*stream).blockLevel != -(1 as c_int) {
                (*stream).level += 1;
            } else if nodeType != XML_ELEMENT_NODE as c_int
                && nodeType != XML_ATTRIBUTE_NODE as c_int
                && (*comp).flags & XML_STREAM_FINAL_IS_ANY_NODE == 0 as c_int
            {
                (*stream).level += 1;
            } else {
                i = 0 as c_int;
                m = (*stream).nbState;
                while i < m {
                    if (*comp).flags & XML_STREAM_DESC == 0 as c_int {
                        stepNr = *(*stream).states.offset(
                            (2 as c_int
                                * ((*stream).nbState - 1 as c_int))
                                as isize,
                        );
                        if *(*stream).states.offset(
                            (2 as c_int
                                * ((*stream).nbState - 1 as c_int)
                                + 1 as c_int) as isize,
                        ) < (*stream).level
                        {
                            return -(1 as c_int);
                        }
                        desc = 0 as c_int;
                        i = m;
                        current_block = 10758786907990354186;
                    } else {
                        stepNr = *(*stream)
                            .states
                            .offset((2 as c_int * i) as isize);
                        if stepNr < 0 as c_int {
                            current_block = 3741760658858474606;
                        } else {
                            tmp = *(*stream).states.offset(
                                (2 as c_int * i + 1 as c_int) as isize,
                            );
                            if tmp > (*stream).level {
                                current_block = 3741760658858474606;
                            } else {
                                desc = (*(*comp).steps.offset(stepNr as isize)).flags
                                    & XML_STREAM_STEP_DESC;
                                if tmp < (*stream).level && desc == 0 {
                                    current_block = 3741760658858474606;
                                } else {
                                    current_block = 10758786907990354186;
                                }
                            }
                        }
                    }
                    match current_block {
                        10758786907990354186 => {
                            step = *(*comp).steps.offset(stepNr as isize);
                            if step.nodeType != nodeType {
                                if step.nodeType == XML_ATTRIBUTE_NODE as c_int {
                                    if (*comp).flags & XML_STREAM_DESC == 0 as c_int {
                                        (*stream).blockLevel =
                                            (*stream).level + 1 as c_int;
                                    }
                                    current_block = 3741760658858474606;
                                } else if step.nodeType != XML_STREAM_ANY_NODE {
                                    current_block = 3741760658858474606;
                                } else {
                                    current_block = 16738040538446813684;
                                }
                            } else {
                                current_block = 16738040538446813684;
                            }
                            match current_block {
                                3741760658858474606 => {}
                                _ => {
                                    match_0 = 0 as c_int;
                                    if step.nodeType == XML_STREAM_ANY_NODE {
                                        match_0 = 1 as c_int;
                                    } else if step.name.is_null() {
                                        if step.ns.is_null() {
                                            match_0 = 1 as c_int;
                                        } else if !ns.is_null() {
                                            match_0 = xmlStrEqual(step.ns, ns);
                                        }
                                    } else if (step.ns != NULL as *const xmlChar)
                                        as c_int
                                        == (ns != NULL as *const xmlChar) as c_int
                                        && !name.is_null()
                                        && *step.name.offset(0 as c_int as isize)
                                            as c_int
                                            == *name.offset(0 as c_int as isize)
                                                as c_int
                                        && xmlStrEqual(step.name, name) != 0
                                        && (step.ns == ns || xmlStrEqual(step.ns, ns) != 0)
                                    {
                                        match_0 = 1 as c_int;
                                    }
                                    if match_0 != 0 {
                                        final_0 = step.flags & XML_STREAM_STEP_FINAL;
                                        if final_0 != 0 {
                                            ret = 1 as c_int;
                                        } else {
                                            xmlStreamCtxtAddState(
                                                stream,
                                                stepNr + 1 as c_int,
                                                (*stream).level + 1 as c_int,
                                            );
                                        }
                                        if ret != 1 as c_int
                                            && step.flags & XML_STREAM_STEP_IN_SET != 0
                                        {
                                            ret = 1 as c_int;
                                        }
                                    }
                                    if (*comp).flags & XML_STREAM_DESC == 0 as c_int
                                        && (match_0 == 0 || final_0 != 0)
                                    {
                                        (*stream).blockLevel =
                                            (*stream).level + 1 as c_int;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                (*stream).level += 1;
                step = *(*comp).steps.offset(0 as c_int as isize);
                if !(step.flags & XML_STREAM_STEP_ROOT != 0) {
                    desc = step.flags & XML_STREAM_STEP_DESC;
                    if (*stream).flags
                        & (XML_PATTERN_XPATH as c_int
                            | XML_PATTERN_XSSEL as c_int
                            | XML_PATTERN_XSFIELD as c_int)
                        != 0
                    {
                        if (*stream).level == 1 as c_int {
                            if (*stream).flags
                                & (XML_PATTERN_XSSEL as c_int
                                    | XML_PATTERN_XSFIELD as c_int)
                                != 0
                            {
                                current_block = 17236156349205036676;
                            } else {
                                current_block = 12700875449501569761;
                            }
                        } else if desc != 0 {
                            current_block = 12700875449501569761;
                        } else if (*stream).level == 2 as c_int
                            && (*stream).flags
                                & (XML_PATTERN_XSSEL as c_int
                                    | XML_PATTERN_XSFIELD as c_int)
                                != 0
                        {
                            current_block = 12700875449501569761;
                        } else {
                            current_block = 17236156349205036676;
                        }
                    } else {
                        current_block = 12700875449501569761;
                    }
                    match current_block {
                        17236156349205036676 => {}
                        _ => {
                            if step.nodeType != nodeType {
                                if nodeType == XML_ATTRIBUTE_NODE as c_int {
                                    current_block = 17236156349205036676;
                                } else if step.nodeType != XML_STREAM_ANY_NODE {
                                    current_block = 17236156349205036676;
                                } else {
                                    current_block = 16789764818708874114;
                                }
                            } else {
                                current_block = 16789764818708874114;
                            }
                            match current_block {
                                17236156349205036676 => {}
                                _ => {
                                    match_0 = 0 as c_int;
                                    if step.nodeType == XML_STREAM_ANY_NODE {
                                        match_0 = 1 as c_int;
                                    } else if step.name.is_null() {
                                        if step.ns.is_null() {
                                            match_0 = 1 as c_int;
                                        } else if !ns.is_null() {
                                            match_0 = xmlStrEqual(step.ns, ns);
                                        }
                                    } else if (step.ns != NULL as *const xmlChar)
                                        as c_int
                                        == (ns != NULL as *const xmlChar) as c_int
                                        && !name.is_null()
                                        && *step.name.offset(0 as c_int as isize)
                                            as c_int
                                            == *name.offset(0 as c_int as isize)
                                                as c_int
                                        && xmlStrEqual(step.name, name) != 0
                                        && (step.ns == ns || xmlStrEqual(step.ns, ns) != 0)
                                    {
                                        match_0 = 1 as c_int;
                                    }
                                    final_0 = step.flags & XML_STREAM_STEP_FINAL;
                                    if match_0 != 0 {
                                        if final_0 != 0 {
                                            ret = 1 as c_int;
                                        } else {
                                            xmlStreamCtxtAddState(
                                                stream,
                                                1 as c_int,
                                                (*stream).level,
                                            );
                                        }
                                        if ret != 1 as c_int
                                            && step.flags & XML_STREAM_STEP_IN_SET != 0
                                        {
                                            ret = 1 as c_int;
                                        }
                                    }
                                    if (*comp).flags & XML_STREAM_DESC == 0 as c_int
                                        && (match_0 == 0 || final_0 != 0)
                                    {
                                        (*stream).blockLevel = (*stream).level;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            stream = (*stream).next as xmlStreamCtxtPtr;
        }
    }
    if err > 0 as c_int {
        ret = -(1 as c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamPush(
    mut stream: xmlStreamCtxtPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
) -> c_int {
    return xmlStreamPushInternal(stream, name, ns, XML_ELEMENT_NODE as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamPushNode(
    mut stream: xmlStreamCtxtPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
    mut nodeType: c_int,
) -> c_int {
    return xmlStreamPushInternal(stream, name, ns, nodeType);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamPushAttr(
    mut stream: xmlStreamCtxtPtr,
    mut name: *const xmlChar,
    mut ns: *const xmlChar,
) -> c_int {
    return xmlStreamPushInternal(stream, name, ns, XML_ATTRIBUTE_NODE as c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamPop(mut stream: xmlStreamCtxtPtr) -> c_int {
    let mut i: c_int = 0;
    let mut lev: c_int = 0;
    if stream.is_null() {
        return -(1 as c_int);
    }
    while !stream.is_null() {
        if (*stream).blockLevel == (*stream).level {
            (*stream).blockLevel = -(1 as c_int);
        }
        if (*stream).level != 0 {
            (*stream).level -= 1;
        }
        i = (*stream).nbState - 1 as c_int;
        while i >= 0 as c_int {
            lev = *(*stream)
                .states
                .offset((2 as c_int * i + 1 as c_int) as isize);
            if lev > (*stream).level {
                (*stream).nbState -= 1;
            }
            if lev <= (*stream).level {
                break;
            }
            i -= 1;
        }
        stream = (*stream).next as xmlStreamCtxtPtr;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlStreamWantsAnyNode(
    mut streamCtxt: xmlStreamCtxtPtr,
) -> c_int {
    if streamCtxt.is_null() {
        return -(1 as c_int);
    }
    while !streamCtxt.is_null() {
        if (*(*streamCtxt).comp).flags & XML_STREAM_FINAL_IS_ANY_NODE != 0 {
            return 1 as c_int;
        }
        streamCtxt = (*streamCtxt).next as xmlStreamCtxtPtr;
    }
    return 0 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatterncompile(
    mut pattern: *const xmlChar,
    mut dict: *mut xmlDict,
    mut flags: c_int,
    mut namespaces: *mut *const xmlChar,
) -> xmlPatternPtr {
    let mut current_block: u64;
    let mut ret: xmlPatternPtr = ::core::ptr::null_mut::<xmlPattern>();
    let mut cur: xmlPatternPtr = ::core::ptr::null_mut::<xmlPattern>();
    let mut ctxt: xmlPatParserContextPtr = ::core::ptr::null_mut::<xmlPatParserContext>();
    let mut or: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut start: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut type_0: c_int = 0 as c_int;
    let mut streamable: c_int = 1 as c_int;
    if pattern.is_null() {
        return ::core::ptr::null_mut::<xmlPattern>();
    }
    start = pattern;
    or = start;
    loop {
        if !(*or as c_int != 0 as c_int) {
            current_block = 313581471991351815;
            break;
        }
        tmp = ::core::ptr::null_mut::<xmlChar>();
        while *or as c_int != 0 as c_int
            && *or as c_int != '|' as i32
        {
            or = or.offset(1);
        }
        if *or as c_int == 0 as c_int {
            ctxt = xmlNewPatParserContext(start, dict as xmlDictPtr, namespaces);
        } else {
            tmp = xmlStrndup(
                start,
                or.offset_from(start) as c_long as c_int,
            );
            if !tmp.is_null() {
                ctxt = xmlNewPatParserContext(tmp, dict as xmlDictPtr, namespaces);
            }
            or = or.offset(1);
        }
        if ctxt.is_null() {
            current_block = 12319523405637821788;
            break;
        }
        cur = xmlNewPattern();
        if cur.is_null() {
            current_block = 12319523405637821788;
            break;
        }
        if !dict.is_null() {
            (*cur).dict = dict as xmlDictPtr;
            xmlDictReference(dict as xmlDictPtr);
        }
        if ret.is_null() {
            ret = cur;
        } else {
            (*cur).next = (*ret).next;
            (*ret).next = cur as *mut _xmlPattern;
        }
        (*cur).flags = flags;
        (*ctxt).comp = cur;
        if (*cur).flags
            & (XML_PATTERN_XSSEL as c_int | XML_PATTERN_XSFIELD as c_int)
            != 0
        {
            xmlCompileIDCXPathPath(ctxt);
        } else {
            xmlCompilePathPattern(ctxt);
        }
        if (*ctxt).error != 0 as c_int {
            current_block = 12319523405637821788;
            break;
        }
        xmlFreePatParserContext(ctxt);
        ctxt = ::core::ptr::null_mut::<xmlPatParserContext>();
        if streamable != 0 {
            if type_0 == 0 as c_int {
                type_0 = (*cur).flags & (PAT_FROM_ROOT | PAT_FROM_CUR);
            } else if type_0 == PAT_FROM_ROOT {
                if (*cur).flags & PAT_FROM_CUR != 0 {
                    streamable = 0 as c_int;
                }
            } else if type_0 == PAT_FROM_CUR {
                if (*cur).flags & PAT_FROM_ROOT != 0 {
                    streamable = 0 as c_int;
                }
            }
        }
        if streamable != 0 {
            xmlStreamCompile(cur);
        }
        if xmlReversePattern(cur) < 0 as c_int {
            current_block = 12319523405637821788;
            break;
        }
        if !tmp.is_null() {
            xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
            tmp = ::core::ptr::null_mut::<xmlChar>();
        }
        start = or;
    }
    match current_block {
        12319523405637821788 => {
            if !ctxt.is_null() {
                xmlFreePatParserContext(ctxt);
            }
            if !ret.is_null() {
                xmlFreePattern(ret);
            }
            if !tmp.is_null() {
                xmlFree.expect("non-null function pointer")(tmp as *mut c_void);
            }
            return ::core::ptr::null_mut::<xmlPattern>();
        }
        _ => {
            if streamable == 0 as c_int {
                cur = ret;
                while !cur.is_null() {
                    if !(*cur).stream.is_null() {
                        xmlFreeStreamComp((*cur).stream);
                        (*cur).stream = ::core::ptr::null_mut::<xmlStreamComp>();
                    }
                    cur = (*cur).next as xmlPatternPtr;
                }
            }
            return ret;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternMatch(
    mut comp: xmlPatternPtr,
    mut node: xmlNodePtr,
) -> c_int {
    let mut ret: c_int = 0 as c_int;
    if comp.is_null() || node.is_null() {
        return -(1 as c_int);
    }
    while !comp.is_null() {
        ret = xmlPatMatch(comp, node);
        if ret != 0 as c_int {
            return ret;
        }
        comp = (*comp).next as xmlPatternPtr;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternGetStreamCtxt(mut comp: xmlPatternPtr) -> xmlStreamCtxtPtr {
    let mut current_block: u64;
    let mut ret: xmlStreamCtxtPtr = ::core::ptr::null_mut::<xmlStreamCtxt>();
    let mut cur: xmlStreamCtxtPtr = ::core::ptr::null_mut::<xmlStreamCtxt>();
    if comp.is_null() || (*comp).stream.is_null() {
        return ::core::ptr::null_mut::<xmlStreamCtxt>();
    }
    loop {
        if comp.is_null() {
            current_block = 11650488183268122163;
            break;
        }
        if (*comp).stream.is_null() {
            current_block = 4436216492381406444;
            break;
        }
        cur = xmlNewStreamCtxt((*comp).stream);
        if cur.is_null() {
            current_block = 4436216492381406444;
            break;
        }
        if ret.is_null() {
            ret = cur;
        } else {
            (*cur).next = (*ret).next;
            (*ret).next = cur as *mut _xmlStreamCtxt;
        }
        (*cur).flags = (*comp).flags;
        comp = (*comp).next as xmlPatternPtr;
    }
    match current_block {
        11650488183268122163 => return ret,
        _ => {
            xmlFreeStreamCtxt(ret);
            return ::core::ptr::null_mut::<xmlStreamCtxt>();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternStreamable(mut comp: xmlPatternPtr) -> c_int {
    if comp.is_null() {
        return -(1 as c_int);
    }
    while !comp.is_null() {
        if (*comp).stream.is_null() {
            return 0 as c_int;
        }
        comp = (*comp).next as xmlPatternPtr;
    }
    return 1 as c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternMaxDepth(mut comp: xmlPatternPtr) -> c_int {
    let mut ret: c_int = 0 as c_int;
    let mut i: c_int = 0;
    if comp.is_null() {
        return -(1 as c_int);
    }
    while !comp.is_null() {
        if (*comp).stream.is_null() {
            return -(1 as c_int);
        }
        i = 0 as c_int;
        while i < (*(*comp).stream).nbStep {
            if (*(*(*comp).stream).steps.offset(i as isize)).flags & XML_STREAM_STEP_DESC != 0 {
                return -(2 as c_int);
            }
            i += 1;
        }
        if (*(*comp).stream).nbStep > ret {
            ret = (*(*comp).stream).nbStep;
        }
        comp = (*comp).next as xmlPatternPtr;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternMinDepth(mut comp: xmlPatternPtr) -> c_int {
    let mut ret: c_int = 12345678 as c_int;
    if comp.is_null() {
        return -(1 as c_int);
    }
    while !comp.is_null() {
        if (*comp).stream.is_null() {
            return -(1 as c_int);
        }
        if (*(*comp).stream).nbStep < ret {
            ret = (*(*comp).stream).nbStep;
        }
        if ret == 0 as c_int {
            return 0 as c_int;
        }
        comp = (*comp).next as xmlPatternPtr;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlPatternFromRoot(mut comp: xmlPatternPtr) -> c_int {
    if comp.is_null() {
        return -(1 as c_int);
    }
    while !comp.is_null() {
        if (*comp).stream.is_null() {
            return -(1 as c_int);
        }
        if (*comp).flags & PAT_FROM_ROOT != 0 {
            return 1 as c_int;
        }
        comp = (*comp).next as xmlPatternPtr;
    }
    return 0 as c_int;
}
