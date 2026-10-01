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
    pub type _xmlRelaxNG;
    pub type _xmlRelaxNGParserCtxt;
    pub type _xmlRelaxNGValidCtxt;
    pub type _xmlSchema;
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlSchemaSAXPlug;
    pub type _xmlPattern;
    pub type _xmlXIncludeCtxt;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlFindCharEncodingHandler(name: *const ::core::ffi::c_char) -> xmlCharEncodingHandlerPtr;
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc;
    fn xmlSplitQName2(name: *const xmlChar, prefix: *mut *mut xmlChar) -> *mut xmlChar;
    fn xmlBufferCreate() -> xmlBufferPtr;
    fn xmlBufferFree(buf: xmlBufferPtr);
    fn xmlBufferCat(buf: xmlBufferPtr, str: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlBufferSetAllocationScheme(buf: xmlBufferPtr, scheme: xmlBufferAllocationScheme);
    fn xmlFreeDtd(cur: xmlDtdPtr);
    fn xmlFreeNs(cur: xmlNsPtr);
    fn xmlFreeNsList(cur: xmlNsPtr);
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlCopyDtd(dtd: xmlDtdPtr) -> xmlDtdPtr;
    fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar) -> xmlNodePtr;
    fn xmlDocCopyNode(
        node: xmlNodePtr,
        doc: xmlDocPtr,
        recursive: ::core::ffi::c_int,
    ) -> xmlNodePtr;
    fn xmlGetLineNo(node: *const xmlNode) -> ::core::ffi::c_long;
    fn xmlIsBlankNode(node: *const xmlNode) -> ::core::ffi::c_int;
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
        inLine: ::core::ffi::c_int,
    ) -> *mut xmlChar;
    fn xmlBufGetNodeContent(buf: xmlBufPtr, cur: *const xmlNode) -> ::core::ffi::c_int;
    fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeGetSpacePreserve(cur: *const xmlNode) -> ::core::ffi::c_int;
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeDump(
        buf: xmlBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        level: ::core::ffi::c_int,
        format: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFilename(
        URI: *const ::core::ffi::c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFd(
        fd: ::core::ffi::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateMem(
        mem: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut ::core::ffi::c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferRead(
        in_0: xmlParserInputBufferPtr,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    fn xmlParserGetDirectory(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrlen(str: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    fn xmlStopParser(ctxt: xmlParserCtxtPtr);
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlCreatePushParserCtxt(
        sax: xmlSAXHandlerPtr,
        user_data: *mut ::core::ffi::c_void,
        chunk: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        filename: *const ::core::ffi::c_char,
    ) -> xmlParserCtxtPtr;
    fn xmlParseChunk(
        ctxt: xmlParserCtxtPtr,
        chunk: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        terminate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlByteConsumed(ctxt: xmlParserCtxtPtr) -> ::core::ffi::c_long;
    fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn xmlCtxtSetMaxAmplification(ctxt: xmlParserCtxtPtr, maxAmpl: ::core::ffi::c_uint);
    fn xmlSAXVersion(hdlr: *mut xmlSAXHandler, version: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn xmlFreeIDTable(table: xmlIDTablePtr);
    fn xmlFreeRefTable(table: xmlRefTablePtr);
    fn xmlValidatePushElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        qname: *const xmlChar,
    ) -> ::core::ffi::c_int;
    fn xmlValidatePushCData(
        ctxt: xmlValidCtxtPtr,
        data: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlValidatePopElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        qname: *const xmlChar,
    ) -> ::core::ffi::c_int;
    fn xmlDictCreate() -> xmlDictPtr;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> *const xmlChar;
    fn xmlDictQLookup(
        dict: xmlDictPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
    ) -> *const xmlChar;
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> ::core::ffi::c_int;
    fn __xmlGenericErrorContext() -> *mut *mut ::core::ffi::c_void;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn xmlParserError(ctx: *mut ::core::ffi::c_void, msg: *const ::core::ffi::c_char, ...);
    fn xmlParserWarning(ctx: *mut ::core::ffi::c_void, msg: *const ::core::ffi::c_char, ...);
    fn xmlParserValidityError(ctx: *mut ::core::ffi::c_void, msg: *const ::core::ffi::c_char, ...);
    fn xmlParserValidityWarning(
        ctx: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn xmlRelaxNGNewParserCtxt(URL: *const ::core::ffi::c_char) -> xmlRelaxNGParserCtxtPtr;
    fn xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr);
    fn xmlRelaxNGSetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut ::core::ffi::c_void,
    );
    fn xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
    fn xmlRelaxNGFree(schema: xmlRelaxNGPtr);
    fn xmlRelaxNGSetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut ::core::ffi::c_void,
    );
    fn xmlRelaxNGSetValidStructuredErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut ::core::ffi::c_void,
    );
    fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr;
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    fn xmlRelaxNGValidatePushElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::core::ffi::c_int;
    fn xmlRelaxNGValidatePushCData(
        ctxt: xmlRelaxNGValidCtxtPtr,
        data: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlRelaxNGValidatePopElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::core::ffi::c_int;
    fn xmlRelaxNGValidateFullElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> ::core::ffi::c_int;
    fn xmlSchemaNewParserCtxt(URL: *const ::core::ffi::c_char) -> xmlSchemaParserCtxtPtr;
    fn xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr);
    fn xmlSchemaSetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut ::core::ffi::c_void,
    );
    fn xmlSchemaIsValid(ctxt: xmlSchemaValidCtxtPtr) -> ::core::ffi::c_int;
    fn xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
    fn xmlSchemaFree(schema: xmlSchemaPtr);
    fn xmlSchemaSetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut ::core::ffi::c_void,
    );
    fn xmlSchemaSetValidStructuredErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut ::core::ffi::c_void,
    );
    fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    fn xmlSchemaSAXPlug(
        ctxt: xmlSchemaValidCtxtPtr,
        sax: *mut xmlSAXHandlerPtr,
        user_data: *mut *mut ::core::ffi::c_void,
    ) -> xmlSchemaSAXPlugPtr;
    fn xmlSchemaSAXUnplug(plug: xmlSchemaSAXPlugPtr) -> ::core::ffi::c_int;
    fn xmlSchemaValidateSetLocator(
        vctxt: xmlSchemaValidCtxtPtr,
        f: xmlSchemaValidityLocatorFunc,
        ctxt: *mut ::core::ffi::c_void,
    );
    fn xmlXIncludeNewContext(doc: xmlDocPtr) -> xmlXIncludeCtxtPtr;
    fn xmlXIncludeSetFlags(
        ctxt: xmlXIncludeCtxtPtr,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlXIncludeFreeContext(ctxt: xmlXIncludeCtxtPtr);
    fn xmlXIncludeProcessNode(ctxt: xmlXIncludeCtxtPtr, tree: xmlNodePtr) -> ::core::ffi::c_int;
    fn xmlSwitchToEncoding(
        ctxt: xmlParserCtxtPtr,
        handler: xmlCharEncodingHandlerPtr,
    ) -> ::core::ffi::c_int;
    fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr) -> ::core::ffi::c_int;
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    fn xmlBufCreateSize(size: size_t) -> xmlBufPtr;
    fn xmlBufSetAllocationScheme(
        buf: xmlBufPtr,
        scheme: xmlBufferAllocationScheme,
    ) -> ::core::ffi::c_int;
    fn xmlBufFree(buf: xmlBufPtr);
    fn xmlBufEmpty(buf: xmlBufPtr);
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr) -> ::core::ffi::c_int;
    static mut __xmlRegisterCallbacks: ::core::ffi::c_int;
    fn xmlXIncludeSetStreamingMode(
        ctxt: xmlXIncludeCtxtPtr,
        mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xmlFreePattern(comp: xmlPatternPtr);
    fn xmlPatterncompile(
        pattern: *const xmlChar,
        dict: *mut xmlDict,
        flags: ::core::ffi::c_int,
        namespaces: *mut *const xmlChar,
    ) -> xmlPatternPtr;
    fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type va_list = __builtin_va_list;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type xmlCharEncoding = ::core::ffi::c_int;
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
pub type xmlCharEncodingInputFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlCharEncodingOutputFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
        *const ::core::ffi::c_uchar,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut ::core::ffi::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
}
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_0: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
pub type xmlChar = ::core::ffi::c_uchar;
pub type xmlBufferAllocationScheme = ::core::ffi::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut ::core::ffi::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub rawconsumed: ::core::ffi::c_ulong,
}
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
pub type xmlInputCloseCallback =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type xmlInputReadCallback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_char,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const ::core::ffi::c_char,
    pub directory: *const ::core::ffi::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: ::core::ffi::c_int,
    pub line: ::core::ffi::c_int,
    pub col: ::core::ffi::c_int,
    pub consumed: ::core::ffi::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub flags: ::core::ffi::c_int,
    pub id: ::core::ffi::c_int,
    pub parentConsumed: ::core::ffi::c_ulong,
    pub entity: xmlEntityPtr,
}
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEntity {
    pub _private: *mut ::core::ffi::c_void,
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
    pub length: ::core::ffi::c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub expandedSize: ::core::ffi::c_ulong,
}
pub type xmlEntityType = ::core::ffi::c_uint;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *mut ::core::ffi::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: ::core::ffi::c_int,
    pub standalone: ::core::ffi::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut ::core::ffi::c_void,
    pub refs: *mut ::core::ffi::c_void,
    pub URL: *const xmlChar,
    pub charset: ::core::ffi::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut ::core::ffi::c_void,
    pub parseFlags: ::core::ffi::c_int,
    pub properties: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut ::core::ffi::c_void,
    pub context: *mut _xmlDoc,
}
pub type xmlElementType = xmlNsType;
pub type xmlNsType = ::core::ffi::c_uint;
pub const XML_XINCLUDE_END: xmlNsType = 20;
pub const XML_XINCLUDE_START: xmlNsType = 19;
pub const XML_NAMESPACE_DECL: xmlNsType = 18;
pub const XML_ENTITY_DECL: xmlNsType = 17;
pub const XML_ATTRIBUTE_DECL: xmlNsType = 16;
pub const XML_ELEMENT_DECL: xmlNsType = 15;
pub const XML_DTD_NODE: xmlNsType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlNsType = 13;
pub const XML_NOTATION_NODE: xmlNsType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlNsType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlNsType = 10;
pub const XML_DOCUMENT_NODE: xmlNsType = 9;
pub const XML_COMMENT_NODE: xmlNsType = 8;
pub const XML_PI_NODE: xmlNsType = 7;
pub const XML_ENTITY_NODE: xmlNsType = 6;
pub const XML_ENTITY_REF_NODE: xmlNsType = 5;
pub const XML_CDATA_SECTION_NODE: xmlNsType = 4;
pub const XML_TEXT_NODE: xmlNsType = 3;
pub const XML_ATTRIBUTE_NODE: xmlNsType = 2;
pub const XML_ELEMENT_NODE: xmlNsType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut ::core::ffi::c_void,
    pub elements: *mut ::core::ffi::c_void,
    pub attributes: *mut ::core::ffi::c_void,
    pub entities: *mut ::core::ffi::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut ::core::ffi::c_void,
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
    pub psvi: *mut ::core::ffi::c_void,
    pub line: ::core::ffi::c_ushort,
    pub extra: ::core::ffi::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut ::core::ffi::c_void,
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
    pub psvi: *mut ::core::ffi::c_void,
}
pub type xmlAttributeType = ::core::ffi::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlParserInputDeallocate = Option<unsafe extern "C" fn(*mut xmlChar) -> ()>;
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut ::core::ffi::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: ::core::ffi::c_int,
    pub replaceEntities: ::core::ffi::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: ::core::ffi::c_int,
    pub html: ::core::ffi::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: ::core::ffi::c_int,
    pub inputMax: ::core::ffi::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: ::core::ffi::c_int,
    pub nodeMax: ::core::ffi::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: ::core::ffi::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: ::core::ffi::c_int,
    pub hasExternalSubset: ::core::ffi::c_int,
    pub hasPErefs: ::core::ffi::c_int,
    pub external: ::core::ffi::c_int,
    pub valid: ::core::ffi::c_int,
    pub validate: ::core::ffi::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: ::core::ffi::c_int,
    pub directory: *mut ::core::ffi::c_char,
    pub name: *const xmlChar,
    pub nameNr: ::core::ffi::c_int,
    pub nameMax: ::core::ffi::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: ::core::ffi::c_long,
    pub checkIndex: ::core::ffi::c_long,
    pub keepBlanks: ::core::ffi::c_int,
    pub disableSAX: ::core::ffi::c_int,
    pub inSubset: ::core::ffi::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut ::core::ffi::c_int,
    pub spaceNr: ::core::ffi::c_int,
    pub spaceMax: ::core::ffi::c_int,
    pub spaceTab: *mut ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: ::core::ffi::c_int,
    pub nodelen: ::core::ffi::c_int,
    pub nodemem: ::core::ffi::c_int,
    pub pedantic: ::core::ffi::c_int,
    pub _private: *mut ::core::ffi::c_void,
    pub loadsubset: ::core::ffi::c_int,
    pub linenumbers: ::core::ffi::c_int,
    pub catalogs: *mut ::core::ffi::c_void,
    pub recovery: ::core::ffi::c_int,
    pub progressive: ::core::ffi::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: ::core::ffi::c_int,
    pub docdict: ::core::ffi::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: ::core::ffi::c_int,
    pub nsNr: ::core::ffi::c_int,
    pub nsMax: ::core::ffi::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut ::core::ffi::c_uint,
    pub pushTab: *mut xmlStartTag,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: ::core::ffi::c_int,
    pub options: ::core::ffi::c_int,
    pub dictNames: ::core::ffi::c_int,
    pub freeElemsNr: ::core::ffi::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: ::core::ffi::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: ::core::ffi::c_ulong,
    pub sizeentities: ::core::ffi::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: ::core::ffi::c_int,
    pub nodeInfoMax: ::core::ffi::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: ::core::ffi::c_int,
    pub sizeentcopy: ::core::ffi::c_ulong,
    pub endCheckState: ::core::ffi::c_int,
    pub nbErrors: ::core::ffi::c_ushort,
    pub nbWarnings: ::core::ffi::c_ushort,
    pub maxAmpl: ::core::ffi::c_uint,
    pub nsdb: *mut xmlParserNsData,
    pub attrHashMax: ::core::ffi::c_uint,
    pub attrHash: *mut xmlAttrHashBucket,
}
pub type xmlAttrHashBucket = _xmlAttrHashBucket;
pub type xmlParserNsData = _xmlParserNsData;
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: ::core::ffi::c_ulong,
    pub begin_line: ::core::ffi::c_ulong,
    pub end_pos: ::core::ffi::c_ulong,
    pub end_line: ::core::ffi::c_ulong,
}
pub type xmlParserMode = ::core::ffi::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub message: *mut ::core::ffi::c_char,
    pub level: xmlErrorLevel,
    pub file: *mut ::core::ffi::c_char,
    pub line: ::core::ffi::c_int,
    pub str1: *mut ::core::ffi::c_char,
    pub str2: *mut ::core::ffi::c_char,
    pub str3: *mut ::core::ffi::c_char,
    pub int1: ::core::ffi::c_int,
    pub int2: ::core::ffi::c_int,
    pub ctxt: *mut ::core::ffi::c_void,
    pub node: *mut ::core::ffi::c_void,
}
pub type xmlErrorLevel = ::core::ffi::c_uint;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlStartTag = _xmlStartTag;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlParserInputState = ::core::ffi::c_int;
pub const XML_PARSER_XML_DECL: xmlParserInputState = 17;
pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const XML_PARSER_DTD: xmlParserInputState = 3;
pub const XML_PARSER_PI: xmlParserInputState = 2;
pub const XML_PARSER_MISC: xmlParserInputState = 1;
pub const XML_PARSER_START: xmlParserInputState = 0;
pub const XML_PARSER_EOF: xmlParserInputState = -1;
pub type xmlValidCtxt = _xmlValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut ::core::ffi::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: ::core::ffi::c_int,
    pub nodeMax: ::core::ffi::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub flags: ::core::ffi::c_uint,
    pub doc: xmlDocPtr,
    pub valid: ::core::ffi::c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: ::core::ffi::c_int,
    pub vstateMax: ::core::ffi::c_int,
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
pub type xmlValidityWarningFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlValidityErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: ::core::ffi::c_ulong,
    pub length: ::core::ffi::c_ulong,
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
    pub initialized: ::core::ffi::c_uint,
    pub _private: *mut ::core::ffi::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
pub type xmlStructuredErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> ()>;
pub type endElementNsSAX2Func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type startElementNsSAX2Func = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        ::core::ffi::c_int,
        *mut *const xmlChar,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *mut *const xmlChar,
    ) -> (),
>;
pub type externalSubsetSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type cdataBlockSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type getParameterEntitySAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
pub type fatalErrorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type errorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type warningSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type commentSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type processingInstructionSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar) -> ()>;
pub type ignorableWhitespaceSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type charactersSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
>;
pub type referenceSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type endElementSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type startElementSAXFunc = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *mut *const xmlChar) -> (),
>;
pub type endDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type startDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> ()>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
    pub getSystemId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
    pub getLineNumber: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub getColumnNumber:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}
pub type unparsedEntityDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type elementDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
        xmlElementContentPtr,
    ) -> (),
>;
pub type xmlElementContentPtr = *mut xmlElementContent;
pub type xmlElementContent = _xmlElementContent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElementContent {
    pub type_0: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
pub type xmlElementContentOccur = ::core::ffi::c_uint;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = ::core::ffi::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type attributeDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *const xmlChar,
        xmlEnumerationPtr,
    ) -> (),
>;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type notationDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type entityDeclSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        ::core::ffi::c_int,
        *const xmlChar,
        *const xmlChar,
        *mut xmlChar,
    ) -> (),
>;
pub type getEntitySAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
pub type resolveEntitySAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlParserInputPtr,
>;
pub type hasExternalSubsetSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type hasInternalSubsetSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type isStandaloneSAXFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
pub type internalSubsetSAXFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type xmlDeregisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlErrorPtr = *mut xmlError;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlIDTable = _xmlHashTable;
pub type xmlIDTablePtr = *mut xmlIDTable;
pub type xmlRefTable = _xmlHashTable;
pub type xmlRefTablePtr = *mut xmlRefTable;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed = 1;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGValidityErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlRelaxNGValidityWarningFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaValidityErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlSchemaValidityWarningFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
pub type xmlSchemaValidityLocatorFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut *const ::core::ffi::c_char,
        *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int,
>;
pub type xmlSchemaSAXPlugStruct = _xmlSchemaSAXPlug;
pub type xmlSchemaSAXPlugPtr = *mut xmlSchemaSAXPlugStruct;
pub type xmlParserSeverities = ::core::ffi::c_uint;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const XML_TEXTREADER_MODE_READING: C2RustUnnamed_0 = 5;
pub const XML_TEXTREADER_MODE_CLOSED: C2RustUnnamed_0 = 4;
pub const XML_TEXTREADER_MODE_EOF: C2RustUnnamed_0 = 3;
pub const XML_TEXTREADER_MODE_ERROR: C2RustUnnamed_0 = 2;
pub const XML_TEXTREADER_MODE_INTERACTIVE: C2RustUnnamed_0 = 1;
pub const XML_TEXTREADER_MODE_INITIAL: C2RustUnnamed_0 = 0;
pub type xmlParserProperties = ::core::ffi::c_uint;
pub const XML_PARSER_SUBST_ENTITIES: xmlParserProperties = 4;
pub const XML_PARSER_VALIDATE: xmlParserProperties = 3;
pub const XML_PARSER_DEFAULTATTRS: xmlParserProperties = 2;
pub const XML_PARSER_LOADDTD: xmlParserProperties = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed_1 = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed_1 = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed_1 = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed_1 = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed_1 = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed_1 = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed_1 = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed_1 = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed_1 = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed_1 = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed_1 = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed_1 = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed_1 = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed_1 = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed_1 = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed_1 = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed_1 = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextReader {
    pub mode: ::core::ffi::c_int,
    pub doc: xmlDocPtr,
    pub validate: xmlTextReaderValidate,
    pub allocs: ::core::ffi::c_int,
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
    pub base: ::core::ffi::c_uint,
    pub cur: ::core::ffi::c_uint,
    pub node: xmlNodePtr,
    pub curnode: xmlNodePtr,
    pub depth: ::core::ffi::c_int,
    pub faketext: xmlNodePtr,
    pub preserve: ::core::ffi::c_int,
    pub buffer: xmlBufPtr,
    pub dict: xmlDictPtr,
    pub ent: xmlNodePtr,
    pub entNr: ::core::ffi::c_int,
    pub entMax: ::core::ffi::c_int,
    pub entTab: *mut xmlNodePtr,
    pub errorFunc: xmlTextReaderErrorFunc,
    pub errorFuncArg: *mut ::core::ffi::c_void,
    pub rngSchemas: xmlRelaxNGPtr,
    pub rngValidCtxt: xmlRelaxNGValidCtxtPtr,
    pub rngPreserveCtxt: ::core::ffi::c_int,
    pub rngValidErrors: ::core::ffi::c_int,
    pub rngFullNode: xmlNodePtr,
    pub xsdSchemas: xmlSchemaPtr,
    pub xsdValidCtxt: xmlSchemaValidCtxtPtr,
    pub xsdPreserveCtxt: ::core::ffi::c_int,
    pub xsdValidErrors: ::core::ffi::c_int,
    pub xsdPlug: xmlSchemaSAXPlugPtr,
    pub xinclude: ::core::ffi::c_int,
    pub xinclude_name: *const xmlChar,
    pub xincctxt: xmlXIncludeCtxtPtr,
    pub in_xinclude: ::core::ffi::c_int,
    pub patternNr: ::core::ffi::c_int,
    pub patternMax: ::core::ffi::c_int,
    pub patternTab: *mut xmlPatternPtr,
    pub preserves: ::core::ffi::c_int,
    pub parserFlags: ::core::ffi::c_int,
    pub sErrorFunc: xmlStructuredErrorFunc,
}
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlPattern = _xmlPattern;
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
pub type xmlTextReaderErrorFunc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        xmlParserSeverities,
        xmlTextReaderLocatorPtr,
    ) -> (),
>;
pub type xmlTextReaderLocatorPtr = *mut ::core::ffi::c_void;
pub type xmlTextReaderState = ::core::ffi::c_int;
pub const XML_TEXTREADER_ERROR: xmlTextReaderState = 6;
pub const XML_TEXTREADER_DONE: xmlTextReaderState = 5;
pub const XML_TEXTREADER_BACKTRACK: xmlTextReaderState = 4;
pub const XML_TEXTREADER_EMPTY: xmlTextReaderState = 3;
pub const XML_TEXTREADER_END: xmlTextReaderState = 2;
pub const XML_TEXTREADER_ELEMENT: xmlTextReaderState = 1;
pub const XML_TEXTREADER_START: xmlTextReaderState = 0;
pub const XML_TEXTREADER_NONE: xmlTextReaderState = -1;
pub type xmlTextReaderValidate = ::core::ffi::c_uint;
pub const XML_TEXTREADER_VALIDATE_XSD: xmlTextReaderValidate = 4;
pub const XML_TEXTREADER_VALIDATE_RNG: xmlTextReaderValidate = 2;
pub const XML_TEXTREADER_VALIDATE_DTD: xmlTextReaderValidate = 1;
pub const XML_TEXTREADER_NOT_VALIDATE: xmlTextReaderValidate = 0;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const XML_DETECT_IDS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const XML_COMPLETE_ATTRS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const XML_SAX2_MAGIC: ::core::ffi::c_uint = 0xdeedbeaf as ::core::ffi::c_uint;
pub const MAX_ERR_MSG_SIZE: ::core::ffi::c_int = 64000 as ::core::ffi::c_int;
pub const MAX_FREE_NODES: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const CHUNK_SIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const XML_TEXTREADER_INPUT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const XML_TEXTREADER_CTXT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NODE_IS_EMPTY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const NODE_IS_PRESERVED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const NODE_IS_SPRESERVED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
unsafe extern "C" fn xmlTextReaderFreeProp(mut reader: xmlTextReaderPtr, mut cur: xmlAttrPtr) {
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
        && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as ::core::ffi::c_int)
    {
        xmlFree.expect("non-null function pointer")(
            (*cur).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
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
        xmlFree.expect("non-null function pointer")(cur as *mut ::core::ffi::c_void);
    };
}
unsafe extern "C" fn xmlTextReaderFreePropList(mut reader: xmlTextReaderPtr, mut cur: xmlAttrPtr) {
    let mut next: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    while !cur.is_null() {
        next = (*cur).next as xmlAttrPtr;
        xmlTextReaderFreeProp(reader, cur);
        cur = next;
    }
}
unsafe extern "C" fn xmlTextReaderFreeNodeList(mut reader: xmlTextReaderPtr, mut cur: xmlNodePtr) {
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
    if (*cur).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlFreeNsList(cur as xmlNsPtr);
        return;
    }
    if (*cur).type_0 as ::core::ffi::c_uint
        == XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cur).type_0 as ::core::ffi::c_uint
            == XML_HTML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlFreeDoc(cur as xmlDocPtr);
        return;
    }
    loop {
        while (*cur).type_0 as ::core::ffi::c_uint
            != XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*cur).type_0 as ::core::ffi::c_uint
                != XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            && !(*cur).children.is_null()
            && (*(*cur).children).parent == cur
        {
            cur = (*cur).children as xmlNodePtr;
            depth = (depth as ::core::ffi::c_ulong).wrapping_add(1 as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        next = (*cur).next as xmlNodePtr;
        parent = (*cur).parent as xmlNodePtr;
        if (*cur).type_0 as ::core::ffi::c_uint
            != XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
                (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
            }
            if ((*cur).type_0 as ::core::ffi::c_uint
                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cur).type_0 as ::core::ffi::c_uint
                    == XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cur).type_0 as ::core::ffi::c_uint
                    == XML_XINCLUDE_END as ::core::ffi::c_int as ::core::ffi::c_uint)
                && !(*cur).properties.is_null()
            {
                xmlTextReaderFreePropList(reader, (*cur).properties as xmlAttrPtr);
            }
            if (*cur).content != &raw mut (*cur).properties as *mut xmlChar
                && (*cur).type_0 as ::core::ffi::c_uint
                    != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*cur).type_0 as ::core::ffi::c_uint
                    != XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*cur).type_0 as ::core::ffi::c_uint
                    != XML_XINCLUDE_END as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*cur).type_0 as ::core::ffi::c_uint
                    != XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if !(*cur).content.is_null()
                    && (dict.is_null()
                        || xmlDictOwns(dict, (*cur).content as *const xmlChar)
                            == 0 as ::core::ffi::c_int)
                {
                    xmlFree.expect("non-null function pointer")(
                        (*cur).content as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    );
                }
            }
            if ((*cur).type_0 as ::core::ffi::c_uint
                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cur).type_0 as ::core::ffi::c_uint
                    == XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cur).type_0 as ::core::ffi::c_uint
                    == XML_XINCLUDE_END as ::core::ffi::c_int as ::core::ffi::c_uint)
                && !(*cur).nsDef.is_null()
            {
                xmlFreeNsList((*cur).nsDef as xmlNsPtr);
            }
            if (*cur).type_0 as ::core::ffi::c_uint
                != XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*cur).type_0 as ::core::ffi::c_uint
                    != XML_COMMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if !(*cur).name.is_null()
                    && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as ::core::ffi::c_int)
                {
                    xmlFree.expect("non-null function pointer")(
                        (*cur).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    );
                }
            }
            if ((*cur).type_0 as ::core::ffi::c_uint
                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cur).type_0 as ::core::ffi::c_uint
                    == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint)
                && !reader.is_null()
                && !(*reader).ctxt.is_null()
                && (*(*reader).ctxt).freeElemsNr < MAX_FREE_NODES
            {
                (*cur).next = (*(*reader).ctxt).freeElems as *mut _xmlNode;
                (*(*reader).ctxt).freeElems = cur;
                (*(*reader).ctxt).freeElemsNr += 1;
            } else {
                xmlFree.expect("non-null function pointer")(cur as *mut ::core::ffi::c_void);
            }
        }
        if !next.is_null() {
            cur = next;
        } else {
            if depth == 0 as size_t || parent.is_null() {
                break;
            }
            depth = (depth as ::core::ffi::c_ulong).wrapping_sub(1 as ::core::ffi::c_ulong)
                as size_t as size_t;
            cur = parent;
            (*cur).children = ::core::ptr::null_mut::<_xmlNode>();
        }
    }
}
unsafe extern "C" fn xmlTextReaderFreeNode(mut reader: xmlTextReaderPtr, mut cur: xmlNodePtr) {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if !reader.is_null() && !(*reader).ctxt.is_null() {
        dict = (*(*reader).ctxt).dict;
    } else {
        dict = ::core::ptr::null_mut::<xmlDict>();
    }
    if (*cur).type_0 as ::core::ffi::c_uint
        == XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlFreeDtd(cur as xmlDtdPtr);
        return;
    }
    if (*cur).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlFreeNs(cur as xmlNsPtr);
        return;
    }
    if (*cur).type_0 as ::core::ffi::c_uint
        == XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xmlTextReaderFreeProp(reader, cur as xmlAttrPtr);
        return;
    }
    if !(*cur).children.is_null()
        && (*cur).type_0 as ::core::ffi::c_uint
            != XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*(*cur).children).parent == cur {
            xmlTextReaderFreeNodeList(reader, (*cur).children as xmlNodePtr);
        }
        (*cur).children = ::core::ptr::null_mut::<_xmlNode>();
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    if ((*cur).type_0 as ::core::ffi::c_uint
        == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cur).type_0 as ::core::ffi::c_uint
            == XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cur).type_0 as ::core::ffi::c_uint
            == XML_XINCLUDE_END as ::core::ffi::c_int as ::core::ffi::c_uint)
        && !(*cur).properties.is_null()
    {
        xmlTextReaderFreePropList(reader, (*cur).properties as xmlAttrPtr);
    }
    if (*cur).content != &raw mut (*cur).properties as *mut xmlChar
        && (*cur).type_0 as ::core::ffi::c_uint
            != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cur).type_0 as ::core::ffi::c_uint
            != XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cur).type_0 as ::core::ffi::c_uint
            != XML_XINCLUDE_END as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cur).type_0 as ::core::ffi::c_uint
            != XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !(*cur).content.is_null()
            && (dict.is_null()
                || xmlDictOwns(dict, (*cur).content as *const xmlChar) == 0 as ::core::ffi::c_int)
        {
            xmlFree.expect("non-null function pointer")(
                (*cur).content as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
        }
    }
    if ((*cur).type_0 as ::core::ffi::c_uint
        == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cur).type_0 as ::core::ffi::c_uint
            == XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cur).type_0 as ::core::ffi::c_uint
            == XML_XINCLUDE_END as ::core::ffi::c_int as ::core::ffi::c_uint)
        && !(*cur).nsDef.is_null()
    {
        xmlFreeNsList((*cur).nsDef as xmlNsPtr);
    }
    if (*cur).type_0 as ::core::ffi::c_uint
        != XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cur).type_0 as ::core::ffi::c_uint
            != XML_COMMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !(*cur).name.is_null()
            && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as ::core::ffi::c_int)
        {
            xmlFree.expect("non-null function pointer")(
                (*cur).name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
        }
    }
    if ((*cur).type_0 as ::core::ffi::c_uint
        == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cur).type_0 as ::core::ffi::c_uint
            == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint)
        && !reader.is_null()
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).freeElemsNr < MAX_FREE_NODES
    {
        (*cur).next = (*(*reader).ctxt).freeElems as *mut _xmlNode;
        (*(*reader).ctxt).freeElems = cur;
        (*(*reader).ctxt).freeElemsNr += 1;
    } else {
        xmlFree.expect("non-null function pointer")(cur as *mut ::core::ffi::c_void);
    };
}
unsafe extern "C" fn xmlTextReaderFreeDoc(mut reader: xmlTextReaderPtr, mut cur: xmlDocPtr) {
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
            (*cur).version as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
    }
    if !(*cur).name.is_null() {
        xmlFree.expect("non-null function pointer")((*cur).name as *mut ::core::ffi::c_void);
    }
    if !(*cur).encoding.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*cur).encoding as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
    }
    if !(*cur).oldNs.is_null() {
        xmlFreeNsList((*cur).oldNs as xmlNsPtr);
    }
    if !(*cur).URL.is_null() {
        xmlFree.expect("non-null function pointer")(
            (*cur).URL as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        );
    }
    if !(*cur).dict.is_null() {
        xmlDictFree((*cur).dict as xmlDictPtr);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlTextReaderEntPush(
    mut reader: xmlTextReaderPtr,
    mut value: xmlNodePtr,
) -> ::core::ffi::c_int {
    if (*reader).entNr >= (*reader).entMax {
        let mut newSize: size_t = (if (*reader).entMax == 0 as ::core::ffi::c_int {
            10 as ::core::ffi::c_int
        } else {
            (*reader).entMax * 2 as ::core::ffi::c_int
        }) as size_t;
        let mut tmp: *mut xmlNodePtr = ::core::ptr::null_mut::<xmlNodePtr>();
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*reader).entTab as *mut ::core::ffi::c_void,
            newSize.wrapping_mul(::core::mem::size_of::<xmlNodePtr>() as size_t),
        ) as *mut xmlNodePtr;
        if tmp.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*reader).entTab = tmp;
        (*reader).entMax = newSize as ::core::ffi::c_int;
    }
    let ref mut fresh2 = *(*reader).entTab.offset((*reader).entNr as isize);
    *fresh2 = value;
    (*reader).ent = value;
    let fresh3 = (*reader).entNr;
    (*reader).entNr = (*reader).entNr + 1;
    return fresh3;
}
unsafe extern "C" fn xmlTextReaderEntPop(mut reader: xmlTextReaderPtr) -> xmlNodePtr {
    let mut ret: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if (*reader).entNr <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    (*reader).entNr -= 1;
    if (*reader).entNr > 0 as ::core::ffi::c_int {
        (*reader).ent = *(*reader)
            .entTab
            .offset(((*reader).entNr - 1 as ::core::ffi::c_int) as isize);
    } else {
        (*reader).ent = ::core::ptr::null_mut::<xmlNode>();
    }
    ret = *(*reader).entTab.offset((*reader).entNr as isize);
    let ref mut fresh1 = *(*reader).entTab.offset((*reader).entNr as isize);
    *fresh1 = ::core::ptr::null_mut::<xmlNode>();
    return ret;
}
unsafe extern "C" fn xmlTextReaderStartElement(
    mut ctx: *mut ::core::ffi::c_void,
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
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '>' as i32
        {
            (*(*ctxt).node).extra = NODE_IS_EMPTY as ::core::ffi::c_ushort;
        }
    }
    if !reader.is_null() {
        (*reader).state = XML_TEXTREADER_ELEMENT;
    }
}
unsafe extern "C" fn xmlTextReaderEndElement(
    mut ctx: *mut ::core::ffi::c_void,
    mut fullname: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).endElement.is_some() {
        (*reader).endElement.expect("non-null function pointer")(ctx, fullname);
    }
}
unsafe extern "C" fn xmlTextReaderStartElementNs(
    mut ctx: *mut ::core::ffi::c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
    mut nb_namespaces: ::core::ffi::c_int,
    mut namespaces: *mut *const xmlChar,
    mut nb_attributes: ::core::ffi::c_int,
    mut nb_defaulted: ::core::ffi::c_int,
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
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
            && *(*(*ctxt).input)
                .cur
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '>' as i32
        {
            (*(*ctxt).node).extra = NODE_IS_EMPTY as ::core::ffi::c_ushort;
        }
    }
    if !reader.is_null() {
        (*reader).state = XML_TEXTREADER_ELEMENT;
    }
}
unsafe extern "C" fn xmlTextReaderEndElementNs(
    mut ctx: *mut ::core::ffi::c_void,
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
    mut ctx: *mut ::core::ffi::c_void,
    mut ch: *const xmlChar,
    mut len: ::core::ffi::c_int,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).characters.is_some() {
        (*reader).characters.expect("non-null function pointer")(ctx, ch, len);
    }
}
unsafe extern "C" fn xmlTextReaderCDataBlock(
    mut ctx: *mut ::core::ffi::c_void,
    mut ch: *const xmlChar,
    mut len: ::core::ffi::c_int,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && (*reader).cdataBlock.is_some() {
        (*reader).cdataBlock.expect("non-null function pointer")(ctx, ch, len);
    }
}
unsafe extern "C" fn xmlTextReaderPushData(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    let mut inbuf: xmlBufPtr = ::core::ptr::null_mut::<xmlBuf>();
    let mut val: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut oldstate: xmlTextReaderState = XML_TEXTREADER_START;
    if (*reader).input.is_null() || (*(*reader).input).buffer.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    oldstate = (*reader).state;
    (*reader).state = XML_TEXTREADER_NONE;
    inbuf = (*(*reader).input).buffer;
    while (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_NONE as ::core::ffi::c_int {
        if xmlBufUse(inbuf)
            < (*reader)
                .cur
                .wrapping_add(CHUNK_SIZE as ::core::ffi::c_uint) as size_t
        {
            if !((*reader).mode != XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int) {
                break;
            }
            val = xmlParserInputBufferRead((*reader).input, 4096 as ::core::ffi::c_int);
            if val == 0 as ::core::ffi::c_int {
                if xmlBufUse(inbuf) == (*reader).cur as size_t {
                    (*reader).mode = XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int;
                    break;
                }
            } else if val < 0 as ::core::ffi::c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"xmlParserInputBufferRead failed\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*reader).mode = XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int;
                (*reader).state = oldstate;
                return val;
            }
        }
        if xmlBufUse(inbuf)
            >= (*reader)
                .cur
                .wrapping_add(CHUNK_SIZE as ::core::ffi::c_uint) as size_t
        {
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const ::core::ffi::c_char)
                    .offset((*reader).cur as isize),
                CHUNK_SIZE,
                0 as ::core::ffi::c_int,
            );
            (*reader).cur = (*reader)
                .cur
                .wrapping_add(CHUNK_SIZE as ::core::ffi::c_uint);
            if val != 0 as ::core::ffi::c_int {
                (*(*reader).ctxt).wellFormed = 0 as ::core::ffi::c_int;
            }
            if (*(*reader).ctxt).wellFormed == 0 as ::core::ffi::c_int {
                break;
            }
        } else {
            s = xmlBufUse(inbuf).wrapping_sub((*reader).cur as size_t) as ::core::ffi::c_int;
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const ::core::ffi::c_char)
                    .offset((*reader).cur as isize),
                s,
                0 as ::core::ffi::c_int,
            );
            (*reader).cur = (*reader).cur.wrapping_add(s as ::core::ffi::c_uint);
            if val != 0 as ::core::ffi::c_int {
                (*(*reader).ctxt).wellFormed = 0 as ::core::ffi::c_int;
            }
            break;
        }
    }
    (*reader).state = oldstate;
    if (*reader).mode == XML_TEXTREADER_MODE_INTERACTIVE as ::core::ffi::c_int {
        if (*(*reader).input).readcallback.is_some() {
            if (*reader).cur >= 4096 as ::core::ffi::c_uint
                && xmlBufUse(inbuf).wrapping_sub((*reader).cur as size_t) <= CHUNK_SIZE as size_t
            {
                val = xmlBufShrink(inbuf, (*reader).cur as size_t) as ::core::ffi::c_int;
                if val >= 0 as ::core::ffi::c_int {
                    (*reader).cur = (*reader).cur.wrapping_sub(val as ::core::ffi::c_uint);
                }
            }
        }
    } else if (*reader).mode == XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int {
        if (*reader).state as ::core::ffi::c_int != XML_TEXTREADER_DONE as ::core::ffi::c_int {
            s = xmlBufUse(inbuf).wrapping_sub((*reader).cur as size_t) as ::core::ffi::c_int;
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const ::core::ffi::c_char)
                    .offset((*reader).cur as isize),
                s,
                1 as ::core::ffi::c_int,
            );
            (*reader).cur = xmlBufUse(inbuf) as ::core::ffi::c_uint;
            (*reader).state = XML_TEXTREADER_DONE;
            if val != 0 as ::core::ffi::c_int {
                if (*(*reader).ctxt).wellFormed != 0 {
                    (*(*reader).ctxt).wellFormed = 0 as ::core::ffi::c_int;
                } else {
                    return -(1 as ::core::ffi::c_int);
                }
            }
        }
    }
    if (*(*reader).ctxt).wellFormed == 0 as ::core::ffi::c_int {
        (*reader).mode = XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int;
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlTextReaderValidatePush(mut reader: xmlTextReaderPtr) {
    let mut node: xmlNodePtr = (*reader).node;
    if (*reader).validate as ::core::ffi::c_uint
        == XML_TEXTREADER_VALIDATE_DTD as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).validate == 1 as ::core::ffi::c_int
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
                b":\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
            qname = xmlStrcat(qname, (*node).name);
            (*(*reader).ctxt).valid &= xmlValidatePushElement(
                &raw mut (*(*reader).ctxt).vctxt,
                (*(*reader).ctxt).myDoc,
                node,
                qname,
            );
            if !qname.is_null() {
                xmlFree.expect("non-null function pointer")(qname as *mut ::core::ffi::c_void);
            }
        }
    }
    if (*reader).validate as ::core::ffi::c_uint
        == XML_TEXTREADER_VALIDATE_RNG as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*reader).rngValidCtxt.is_null()
    {
        let mut ret: ::core::ffi::c_int = 0;
        if !(*reader).rngFullNode.is_null() {
            return;
        }
        ret = xmlRelaxNGValidatePushElement((*reader).rngValidCtxt, (*(*reader).ctxt).myDoc, node);
        if ret == 0 as ::core::ffi::c_int {
            node = xmlTextReaderExpand(reader);
            if node.is_null() {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                ret = xmlRelaxNGValidateFullElement(
                    (*reader).rngValidCtxt,
                    (*(*reader).ctxt).myDoc,
                    node,
                );
                (*reader).rngFullNode = node;
            }
        }
        if ret != 1 as ::core::ffi::c_int {
            (*reader).rngValidErrors += 1;
        }
    }
}
unsafe extern "C" fn xmlTextReaderValidateCData(
    mut reader: xmlTextReaderPtr,
    mut data: *const xmlChar,
    mut len: ::core::ffi::c_int,
) {
    if (*reader).validate as ::core::ffi::c_uint
        == XML_TEXTREADER_VALIDATE_DTD as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).validate == 1 as ::core::ffi::c_int
    {
        (*(*reader).ctxt).valid &=
            xmlValidatePushCData(&raw mut (*(*reader).ctxt).vctxt, data, len);
    }
    if (*reader).validate as ::core::ffi::c_uint
        == XML_TEXTREADER_VALIDATE_RNG as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*reader).rngValidCtxt.is_null()
    {
        let mut ret: ::core::ffi::c_int = 0;
        if !(*reader).rngFullNode.is_null() {
            return;
        }
        ret = xmlRelaxNGValidatePushCData((*reader).rngValidCtxt, data, len);
        if ret != 1 as ::core::ffi::c_int {
            (*reader).rngValidErrors += 1;
        }
    }
}
unsafe extern "C" fn xmlTextReaderValidatePop(mut reader: xmlTextReaderPtr) {
    let mut node: xmlNodePtr = (*reader).node;
    if (*reader).validate as ::core::ffi::c_uint
        == XML_TEXTREADER_VALIDATE_DTD as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*reader).ctxt.is_null()
        && (*(*reader).ctxt).validate == 1 as ::core::ffi::c_int
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
                b":\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
            qname = xmlStrcat(qname, (*node).name);
            (*(*reader).ctxt).valid &= xmlValidatePopElement(
                &raw mut (*(*reader).ctxt).vctxt,
                (*(*reader).ctxt).myDoc,
                node,
                qname,
            );
            if !qname.is_null() {
                xmlFree.expect("non-null function pointer")(qname as *mut ::core::ffi::c_void);
            }
        }
    }
    if (*reader).validate as ::core::ffi::c_uint
        == XML_TEXTREADER_VALIDATE_RNG as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*reader).rngValidCtxt.is_null()
    {
        let mut ret: ::core::ffi::c_int = 0;
        if !(*reader).rngFullNode.is_null() {
            if node == (*reader).rngFullNode {
                (*reader).rngFullNode = ::core::ptr::null_mut::<xmlNode>();
            }
            return;
        }
        ret = xmlRelaxNGValidatePopElement((*reader).rngValidCtxt, (*(*reader).ctxt).myDoc, node);
        if ret != 1 as ::core::ffi::c_int {
            (*reader).rngValidErrors += 1;
        }
    }
}
unsafe extern "C" fn xmlTextReaderValidateEntity(mut reader: xmlTextReaderPtr) {
    let mut current_block: u64;
    let mut oldnode: xmlNodePtr = (*reader).node;
    let mut node: xmlNodePtr = (*reader).node;
    loop {
        if (*node).type_0 as ::core::ffi::c_uint
            == XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if !(*node).children.is_null()
                && (*(*node).children).type_0 as ::core::ffi::c_uint
                    == XML_ENTITY_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
                && !(*(*node).children).children.is_null()
            {
                if xmlTextReaderEntPush(reader, node) < 0 as ::core::ffi::c_int {
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
            if (*node).type_0 as ::core::ffi::c_uint
                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*reader).node = node;
                xmlTextReaderValidatePush(reader);
            } else if (*node).type_0 as ::core::ffi::c_uint
                == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*node).type_0 as ::core::ffi::c_uint
                    == XML_CDATA_SECTION_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                xmlTextReaderValidateCData(reader, (*node).content, xmlStrlen((*node).content));
            }
            if !(*node).children.is_null() {
                node = (*node).children as xmlNodePtr;
                current_block = 11174649648027449784;
            } else {
                if (*node).type_0 as ::core::ffi::c_uint
                    == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
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
                        if (*node).type_0 as ::core::ffi::c_uint
                            == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let mut tmp: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
                            if (*reader).entNr == 0 as ::core::ffi::c_int {
                                loop {
                                    tmp = (*node).last as xmlNodePtr;
                                    if tmp.is_null() {
                                        break;
                                    }
                                    if !((*tmp).extra as ::core::ffi::c_int & NODE_IS_PRESERVED
                                        == 0 as ::core::ffi::c_int)
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
                        if (*node).type_0 as ::core::ffi::c_uint
                            == XML_ENTITY_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
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
}
unsafe extern "C" fn xmlTextReaderGetSuccessor(mut cur: xmlNodePtr) -> xmlNodePtr {
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
}
unsafe extern "C" fn xmlTextReaderDoExpand(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    let mut val: ::core::ffi::c_int = 0;
    if reader.is_null() || (*reader).node.is_null() || (*reader).ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        if (*(*reader).ctxt).instate as ::core::ffi::c_int == XML_PARSER_EOF as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        if !xmlTextReaderGetSuccessor((*reader).node).is_null() {
            return 1 as ::core::ffi::c_int;
        }
        if (*(*reader).ctxt).nodeNr < (*reader).depth {
            return 1 as ::core::ffi::c_int;
        }
        if (*reader).mode == XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        val = xmlTextReaderPushData(reader);
        if val < 0 as ::core::ffi::c_int {
            (*reader).mode = XML_TEXTREADER_MODE_ERROR as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        if !((*reader).mode != XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int) {
            break;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlTextReaderCollectSiblings(mut node: xmlNodePtr) -> *mut xmlChar {
    let mut buffer: xmlBufferPtr = ::core::ptr::null_mut::<xmlBuffer>();
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if node.is_null()
        || (*node).type_0 as ::core::ffi::c_uint
            == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    buffer = xmlBufferCreate();
    if buffer.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    xmlBufferSetAllocationScheme(buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    while !node.is_null() {
        match (*node).type_0 as ::core::ffi::c_uint {
            3 | 4 => {
                xmlBufferCat(buffer, (*node).content);
            }
            1 => {
                let mut tmp: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                tmp = xmlTextReaderCollectSiblings((*node).children as xmlNodePtr);
                xmlBufferCat(buffer, tmp);
                xmlFree.expect("non-null function pointer")(tmp as *mut ::core::ffi::c_void);
            }
            _ => {}
        }
        node = (*node).next as xmlNodePtr;
    }
    ret = (*buffer).content;
    (*buffer).content = ::core::ptr::null_mut::<xmlChar>();
    xmlBufferFree(buffer);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRead(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut val: ::core::ffi::c_int = 0;
    let mut olddepth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut oldstate: xmlTextReaderState = XML_TEXTREADER_START;
    let mut oldnode: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    if !(*reader).doc.is_null() {
        return xmlTextReaderReadTree(reader);
    }
    if (*reader).ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).mode == XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int {
        (*reader).mode = XML_TEXTREADER_MODE_INTERACTIVE as ::core::ffi::c_int;
        loop {
            val = xmlTextReaderPushData(reader);
            if val < 0 as ::core::ffi::c_int {
                (*reader).mode = XML_TEXTREADER_MODE_ERROR as ::core::ffi::c_int;
                (*reader).state = XML_TEXTREADER_ERROR;
                return -(1 as ::core::ffi::c_int);
            }
            if !((*(*reader).ctxt).node.is_null()
                && ((*reader).mode != XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int
                    && (*reader).state as ::core::ffi::c_int
                        != XML_TEXTREADER_DONE as ::core::ffi::c_int))
            {
                break;
            }
        }
        if (*(*reader).ctxt).node.is_null() {
            if !(*(*reader).ctxt).myDoc.is_null() {
                (*reader).node = (*(*(*reader).ctxt).myDoc).children as xmlNodePtr;
            }
            if (*reader).node.is_null() {
                (*reader).mode = XML_TEXTREADER_MODE_ERROR as ::core::ffi::c_int;
                (*reader).state = XML_TEXTREADER_ERROR;
                return -(1 as ::core::ffi::c_int);
            }
            (*reader).state = XML_TEXTREADER_ELEMENT;
        } else {
            if !(*(*reader).ctxt).myDoc.is_null() {
                (*reader).node = (*(*(*reader).ctxt).myDoc).children as xmlNodePtr;
            }
            if (*reader).node.is_null() {
                (*reader).node = *(*(*reader).ctxt)
                    .nodeTab
                    .offset(0 as ::core::ffi::c_int as isize);
            }
            (*reader).state = XML_TEXTREADER_ELEMENT;
        }
        (*reader).depth = 0 as ::core::ffi::c_int;
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
                    if (*reader).mode == XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    } else {
                        return -(1 as ::core::ffi::c_int);
                    }
                }
                while !(*reader).node.is_null()
                    && (*(*reader).node).next.is_null()
                    && (*(*reader).ctxt).nodeNr == olddepth
                    && (oldstate as ::core::ffi::c_int
                        == XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int
                        || (*(*reader).node).children.is_null()
                        || (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        || !(*(*reader).node).children.is_null()
                            && (*(*(*reader).node).children).type_0 as ::core::ffi::c_uint
                                == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*(*(*reader).node).children).next.is_null()
                        || (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_HTML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint)
                    && ((*(*reader).ctxt).node.is_null()
                        || (*(*reader).ctxt).node == (*reader).node
                        || (*(*reader).ctxt).node == (*(*reader).node).parent)
                    && (*(*reader).ctxt).instate as ::core::ffi::c_int
                        != XML_PARSER_EOF as ::core::ffi::c_int
                {
                    val = xmlTextReaderPushData(reader);
                    if val < 0 as ::core::ffi::c_int {
                        (*reader).mode = XML_TEXTREADER_MODE_ERROR as ::core::ffi::c_int;
                        (*reader).state = XML_TEXTREADER_ERROR;
                        return -(1 as ::core::ffi::c_int);
                    }
                    if (*reader).node.is_null() {
                        break '_get_next_node;
                    }
                }
                if oldstate as ::core::ffi::c_int != XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int
                {
                    if !(*(*reader).node).children.is_null()
                        && (*(*reader).node).type_0 as ::core::ffi::c_uint
                            != XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        && (*(*reader).node).type_0 as ::core::ffi::c_uint
                            != XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
                        && (*(*reader).node).type_0 as ::core::ffi::c_uint
                            != XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (*reader).node = (*(*reader).node).children as xmlNodePtr;
                        (*reader).depth += 1;
                        (*reader).state = XML_TEXTREADER_ELEMENT;
                        current_block = 1022895542554905014;
                        continue;
                    }
                }
                if !(*(*reader).node).next.is_null() {
                    if oldstate as ::core::ffi::c_int
                        == XML_TEXTREADER_ELEMENT as ::core::ffi::c_int
                        && (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        && (*(*reader).node).children.is_null()
                        && (*(*reader).node).extra as ::core::ffi::c_int & NODE_IS_EMPTY
                            == 0 as ::core::ffi::c_int
                        && (*reader).in_xinclude <= 0 as ::core::ffi::c_int
                    {
                        (*reader).state = XML_TEXTREADER_END;
                        current_block = 1022895542554905014;
                    } else {
                        if (*reader).validate as ::core::ffi::c_uint != 0
                            && (*(*reader).node).type_0 as ::core::ffi::c_uint
                                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            xmlTextReaderValidatePop(reader);
                        }
                        if (*reader).preserves > 0 as ::core::ffi::c_int
                            && (*(*reader).node).extra as ::core::ffi::c_int & NODE_IS_SPRESERVED
                                != 0
                        {
                            (*reader).preserves -= 1;
                        }
                        (*reader).node = (*(*reader).node).next as xmlNodePtr;
                        (*reader).state = XML_TEXTREADER_ELEMENT;
                        if (*reader).preserves == 0 as ::core::ffi::c_int
                            && (*reader).in_xinclude == 0 as ::core::ffi::c_int
                            && (*reader).entNr == 0 as ::core::ffi::c_int
                            && !(*(*reader).node).prev.is_null()
                            && (*(*(*reader).node).prev).type_0 as ::core::ffi::c_uint
                                != XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let mut tmp: xmlNodePtr = (*(*reader).node).prev as xmlNodePtr;
                            if (*tmp).extra as ::core::ffi::c_int & NODE_IS_PRESERVED
                                == 0 as ::core::ffi::c_int
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
                } else if oldstate as ::core::ffi::c_int
                    == XML_TEXTREADER_ELEMENT as ::core::ffi::c_int
                    && (*(*reader).node).type_0 as ::core::ffi::c_uint
                        == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*(*reader).node).children.is_null()
                    && (*(*reader).node).extra as ::core::ffi::c_int & NODE_IS_EMPTY
                        == 0 as ::core::ffi::c_int
                {
                    (*reader).state = XML_TEXTREADER_END;
                    current_block = 1022895542554905014;
                } else {
                    if (*reader).validate as ::core::ffi::c_uint
                        != XML_TEXTREADER_NOT_VALIDATE as ::core::ffi::c_int as ::core::ffi::c_uint
                        && (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        xmlTextReaderValidatePop(reader);
                    }
                    if (*reader).preserves > 0 as ::core::ffi::c_int
                        && (*(*reader).node).extra as ::core::ffi::c_int & NODE_IS_SPRESERVED != 0
                    {
                        (*reader).preserves -= 1;
                    }
                    (*reader).node = (*(*reader).node).parent as xmlNodePtr;
                    if (*reader).node.is_null()
                        || (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_HTML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        if (*reader).mode != XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int {
                            val = xmlParseChunk(
                                (*reader).ctxt,
                                b"\0" as *const u8 as *const ::core::ffi::c_char,
                                0 as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            );
                            (*reader).state = XML_TEXTREADER_DONE;
                            if val != 0 as ::core::ffi::c_int {
                                return -(1 as ::core::ffi::c_int);
                            }
                        }
                        (*reader).node = ::core::ptr::null_mut::<xmlNode>();
                        (*reader).depth = -(1 as ::core::ffi::c_int);
                        if !oldnode.is_null()
                            && (*reader).preserves == 0 as ::core::ffi::c_int
                            && (*reader).in_xinclude == 0 as ::core::ffi::c_int
                            && (*reader).entNr == 0 as ::core::ffi::c_int
                            && (*oldnode).type_0 as ::core::ffi::c_uint
                                != XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                            && (*oldnode).extra as ::core::ffi::c_int & NODE_IS_PRESERVED
                                == 0 as ::core::ffi::c_int
                        {
                            xmlUnlinkNode(oldnode);
                            xmlTextReaderFreeNode(reader, oldnode);
                        }
                        break;
                    } else {
                        if (*reader).preserves == 0 as ::core::ffi::c_int
                            && (*reader).in_xinclude == 0 as ::core::ffi::c_int
                            && (*reader).entNr == 0 as ::core::ffi::c_int
                            && !(*(*reader).node).last.is_null()
                            && (*(*(*reader).node).last).extra as ::core::ffi::c_int
                                & NODE_IS_PRESERVED
                                == 0 as ::core::ffi::c_int
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
                    && ((*(*reader).node).type_0 as ::core::ffi::c_uint
                        == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_CDATA_SECTION_NODE as ::core::ffi::c_int as ::core::ffi::c_uint)
                {
                    if xmlTextReaderExpand(reader).is_null() {
                        return -(1 as ::core::ffi::c_int);
                    }
                }
                if (*reader).xinclude != 0
                    && (*reader).in_xinclude == 0 as ::core::ffi::c_int
                    && (*reader).state as ::core::ffi::c_int
                        != XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int
                    && !(*reader).node.is_null()
                    && (*(*reader).node).type_0 as ::core::ffi::c_uint
                        == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !(*(*reader).node).ns.is_null()
                    && (xmlStrEqual((*(*(*reader).node).ns).href, XINCLUDE_NS) != 0
                        || xmlStrEqual((*(*(*reader).node).ns).href, XINCLUDE_OLD_NS) != 0)
                {
                    if (*reader).xincctxt.is_null() {
                        (*reader).xincctxt = xmlXIncludeNewContext((*(*reader).ctxt).myDoc);
                        xmlXIncludeSetFlags(
                            (*reader).xincctxt,
                            (*reader).parserFlags & !(XML_PARSE_NOXINCNODE as ::core::ffi::c_int),
                        );
                        xmlXIncludeSetStreamingMode((*reader).xincctxt, 1 as ::core::ffi::c_int);
                    }
                    if xmlTextReaderExpand(reader).is_null() {
                        return -(1 as ::core::ffi::c_int);
                    }
                    xmlXIncludeProcessNode((*reader).xincctxt, (*reader).node);
                }
                if !(*reader).node.is_null()
                    && (*(*reader).node).type_0 as ::core::ffi::c_uint
                        == XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*reader).in_xinclude += 1;
                    current_block = 14230955220427894555;
                } else if !(*reader).node.is_null()
                    && (*(*reader).node).type_0 as ::core::ffi::c_uint
                        == XML_XINCLUDE_END as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*reader).in_xinclude -= 1;
                    current_block = 14230955220427894555;
                } else {
                    if !(*reader).node.is_null()
                        && (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !(*reader).ctxt.is_null()
                        && (*(*reader).ctxt).replaceEntities == 1 as ::core::ffi::c_int
                    {
                        if !(*(*reader).node).children.is_null()
                            && (*(*(*reader).node).children).type_0 as ::core::ffi::c_uint
                                == XML_ENTITY_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
                            && !(*(*(*reader).node).children).children.is_null()
                        {
                            if xmlTextReaderEntPush(reader, (*reader).node)
                                < 0 as ::core::ffi::c_int
                            {
                                current_block = 14230955220427894555;
                                continue;
                            }
                            (*reader).node = (*(*(*reader).node).children).children as xmlNodePtr;
                        }
                    } else if !(*reader).node.is_null()
                        && (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !(*reader).ctxt.is_null()
                        && (*reader).validate as ::core::ffi::c_uint != 0
                    {
                        xmlTextReaderValidateEntity(reader);
                    }
                    if !(*reader).node.is_null()
                        && (*(*reader).node).type_0 as ::core::ffi::c_uint
                            == XML_ENTITY_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !(*reader).ent.is_null()
                        && (*(*reader).ent).children == (*reader).node
                    {
                        (*reader).node = xmlTextReaderEntPop(reader);
                        (*reader).depth += 1;
                        current_block = 14230955220427894555;
                    } else {
                        if (*reader).validate as ::core::ffi::c_uint
                            != XML_TEXTREADER_NOT_VALIDATE as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            && !(*reader).node.is_null()
                        {
                            let mut node: xmlNodePtr = (*reader).node;
                            if (*node).type_0 as ::core::ffi::c_uint
                                == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                                && ((*reader).state as ::core::ffi::c_int
                                    != XML_TEXTREADER_END as ::core::ffi::c_int
                                    && (*reader).state as ::core::ffi::c_int
                                        != XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int)
                            {
                                xmlTextReaderValidatePush(reader);
                            } else if (*node).type_0 as ::core::ffi::c_uint
                                == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                                || (*node).type_0 as ::core::ffi::c_uint
                                    == XML_CDATA_SECTION_NODE as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                            {
                                xmlTextReaderValidateCData(
                                    reader,
                                    (*node).content,
                                    xmlStrlen((*node).content),
                                );
                            }
                        }
                        if (*reader).patternNr > 0 as ::core::ffi::c_int
                            && (*reader).state as ::core::ffi::c_int
                                != XML_TEXTREADER_END as ::core::ffi::c_int
                            && (*reader).state as ::core::ffi::c_int
                                != XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int
                        {
                            let mut i: ::core::ffi::c_int = 0;
                            i = 0 as ::core::ffi::c_int;
                            while i < (*reader).patternNr {
                                if xmlPatternMatch(
                                    *(*reader).patternTab.offset(i as isize),
                                    (*reader).node,
                                ) == 1 as ::core::ffi::c_int
                                {
                                    xmlTextReaderPreserve(reader);
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                        }
                        if (*reader).validate as ::core::ffi::c_uint
                            == XML_TEXTREADER_VALIDATE_XSD as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            && (*reader).xsdValidErrors == 0 as ::core::ffi::c_int
                            && !(*reader).xsdValidCtxt.is_null()
                        {
                            (*reader).xsdValidErrors = (xmlSchemaIsValid((*reader).xsdValidCtxt)
                                == 0)
                                as ::core::ffi::c_int;
                        }
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    (*reader).state = XML_TEXTREADER_DONE;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadState(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return (*reader).mode;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderExpand(mut reader: xmlTextReaderPtr) -> xmlNodePtr {
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*reader).doc.is_null() {
        return (*reader).node;
    }
    if (*reader).ctxt.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if xmlTextReaderDoExpand(reader) < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    return (*reader).node;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNext(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut cur: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*reader).doc.is_null() {
        return xmlTextReaderNextTree(reader);
    }
    cur = (*reader).node;
    if cur.is_null()
        || (*cur).type_0 as ::core::ffi::c_uint
            != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return xmlTextReaderRead(reader);
    }
    if (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_END as ::core::ffi::c_int
        || (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int
    {
        return xmlTextReaderRead(reader);
    }
    if (*cur).extra as ::core::ffi::c_int & NODE_IS_EMPTY != 0 {
        return xmlTextReaderRead(reader);
    }
    loop {
        ret = xmlTextReaderRead(reader);
        if ret != 1 as ::core::ffi::c_int {
            return ret;
        }
        if !((*reader).node != cur) {
            break;
        }
    }
    return xmlTextReaderRead(reader);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadInnerXml(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
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
        node = xmlDocCopyNode(cur_node, doc, 1 as ::core::ffi::c_int);
        buff2 = xmlBufferCreate();
        xmlBufferSetAllocationScheme(buff2, XML_BUFFER_ALLOC_DOUBLEIT);
        if xmlNodeDump(
            buff2,
            doc,
            node,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == -(1 as ::core::ffi::c_int)
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
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadOuterXml(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
    let mut resbuf: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut buff: xmlBufferPtr = ::core::ptr::null_mut::<xmlBuffer>();
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if xmlTextReaderExpand(reader).is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    node = (*reader).node;
    doc = (*node).doc as xmlDocPtr;
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        node = xmlCopyDtd(node as xmlDtdPtr) as xmlNodePtr;
    } else {
        node = xmlDocCopyNode(node, doc, 1 as ::core::ffi::c_int);
    }
    buff = xmlBufferCreate();
    xmlBufferSetAllocationScheme(buff, XML_BUFFER_ALLOC_DOUBLEIT);
    if xmlNodeDump(
        buff,
        doc,
        node,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) == -(1 as ::core::ffi::c_int)
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
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadString(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    node = if !(*reader).curnode.is_null() {
        (*reader).curnode
    } else {
        (*reader).node
    };
    match (*node).type_0 as ::core::ffi::c_uint {
        3 => {
            if !(*node).content.is_null() {
                return xmlStrdup((*node).content);
            }
        }
        1 => {
            if xmlTextReaderDoExpand(reader) != -(1 as ::core::ffi::c_int) {
                return xmlTextReaderCollectSiblings((*node).children as xmlNodePtr);
            }
        }
        2 => {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlreader.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1677 as ::core::ffi::c_int,
            );
        }
        _ => {}
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
unsafe extern "C" fn xmlTextReaderNextTree(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_END as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*reader).node.is_null() {
        if (*(*reader).doc).children.is_null() {
            (*reader).state = XML_TEXTREADER_END;
            return 0 as ::core::ffi::c_int;
        }
        (*reader).node = (*(*reader).doc).children as xmlNodePtr;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as ::core::ffi::c_int;
    }
    if (*reader).state as ::core::ffi::c_int != XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int {
        if !(*(*reader).node).next.is_null() {
            (*reader).node = (*(*reader).node).next as xmlNodePtr;
            (*reader).state = XML_TEXTREADER_START;
            return 1 as ::core::ffi::c_int;
        }
        (*reader).state = XML_TEXTREADER_BACKTRACK;
        xmlTextReaderRead(reader);
    }
    if !(*(*reader).node).next.is_null() {
        (*reader).node = (*(*reader).node).next as xmlNodePtr;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as ::core::ffi::c_int;
    }
    if !(*(*reader).node).parent.is_null() {
        if (*(*(*reader).node).parent).type_0 as ::core::ffi::c_uint
            == XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*reader).state = XML_TEXTREADER_END;
            return 0 as ::core::ffi::c_int;
        }
        (*reader).node = (*(*reader).node).parent as xmlNodePtr;
        (*reader).depth -= 1;
        (*reader).state = XML_TEXTREADER_BACKTRACK;
        xmlTextReaderNextTree(reader);
    }
    (*reader).state = XML_TEXTREADER_END;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlTextReaderReadTree(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    let mut current_block: u64;
    if (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_END as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        if (*reader).node.is_null() {
            if (*(*reader).doc).children.is_null() {
                (*reader).state = XML_TEXTREADER_END;
                return 0 as ::core::ffi::c_int;
            }
            (*reader).node = (*(*reader).doc).children as xmlNodePtr;
            (*reader).state = XML_TEXTREADER_START;
        } else {
            if (*reader).state as ::core::ffi::c_int
                != XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int
                && (*(*reader).node).type_0 as ::core::ffi::c_uint
                    != XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*(*reader).node).type_0 as ::core::ffi::c_uint
                    != XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*(*reader).node).type_0 as ::core::ffi::c_uint
                    != XML_ENTITY_REF_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if !(*(*reader).node).children.is_null() {
                    (*reader).node = (*(*reader).node).children as xmlNodePtr;
                    (*reader).depth += 1;
                    (*reader).state = XML_TEXTREADER_START;
                    current_block = 66159954028994333;
                } else if (*(*reader).node).type_0 as ::core::ffi::c_uint
                    == XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
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
                        if (*(*(*reader).node).parent).type_0 as ::core::ffi::c_uint
                            == XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
                            || (*(*(*reader).node).parent).type_0 as ::core::ffi::c_uint
                                == XML_HTML_DOCUMENT_NODE as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                        {
                            (*reader).state = XML_TEXTREADER_END;
                            return 0 as ::core::ffi::c_int;
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
        if !((*(*reader).node).type_0 as ::core::ffi::c_uint
            == XML_XINCLUDE_START as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*(*reader).node).type_0 as ::core::ffi::c_uint
                == XML_XINCLUDE_END as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            break;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNextSibling(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).doc.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_END as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if (*reader).node.is_null() {
        return xmlTextReaderNextTree(reader);
    }
    if !(*(*reader).node).next.is_null() {
        (*reader).node = (*(*reader).node).next as xmlNodePtr;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextReader(
    mut input: xmlParserInputBufferPtr,
    mut URI: *const ::core::ffi::c_char,
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
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlTextReader>() as size_t,
    );
    (*ret).doc = ::core::ptr::null_mut::<xmlDoc>();
    (*ret).entTab = ::core::ptr::null_mut::<xmlNodePtr>();
    (*ret).entMax = 0 as ::core::ffi::c_int;
    (*ret).entNr = 0 as ::core::ffi::c_int;
    (*ret).input = input;
    (*ret).buffer = xmlBufCreateSize(100 as size_t);
    if (*ret).buffer.is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    (*ret).sax = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlSAXHandler>() as size_t,
    ) as *mut xmlSAXHandler as xmlSAXHandlerPtr;
    if (*ret).sax.is_null() {
        xmlBufFree((*ret).buffer);
        xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    xmlSAXVersion((*ret).sax as *mut xmlSAXHandler, 2 as ::core::ffi::c_int);
    (*ret).startElement = (*(*ret).sax).startElement;
    (*(*ret).sax).startElement = Some(
        xmlTextReaderStartElement
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*ret).endElement = (*(*ret).sax).endElement;
    (*(*ret).sax).endElement = Some(
        xmlTextReaderEndElement
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    if (*(*ret).sax).initialized == XML_SAX2_MAGIC {
        (*ret).startElementNs = (*(*ret).sax).startElementNs;
        (*(*ret).sax).startElementNs = Some(
            xmlTextReaderStartElementNs
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    *mut *const xmlChar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *const xmlChar,
                ) -> (),
        ) as startElementNsSAX2Func;
        (*ret).endElementNs = (*(*ret).sax).endElementNs;
        (*(*ret).sax).endElementNs = Some(
            xmlTextReaderEndElementNs
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
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
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*(*ret).sax).ignorableWhitespace = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*ret).cdataBlock = (*(*ret).sax).cdataBlock;
    (*(*ret).sax).cdataBlock = Some(
        xmlTextReaderCDataBlock
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as cdataBlockSAXFunc;
    (*ret).mode = XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int;
    (*ret).node = ::core::ptr::null_mut::<xmlNode>();
    (*ret).curnode = ::core::ptr::null_mut::<xmlNode>();
    if xmlBufUse((*(*ret).input).buffer) < 4 as size_t {
        xmlParserInputBufferRead(input, 4 as ::core::ffi::c_int);
    }
    if xmlBufUse((*(*ret).input).buffer) >= 4 as size_t {
        (*ret).ctxt = xmlCreatePushParserCtxt(
            (*ret).sax,
            NULL,
            xmlBufContent((*(*ret).input).buffer as *const xmlBuf) as *const ::core::ffi::c_char,
            4 as ::core::ffi::c_int,
            URI,
        );
        (*ret).base = 0 as ::core::ffi::c_uint;
        (*ret).cur = 4 as ::core::ffi::c_uint;
    } else {
        (*ret).ctxt = xmlCreatePushParserCtxt(
            (*ret).sax,
            NULL,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            URI,
        );
        (*ret).base = 0 as ::core::ffi::c_uint;
        (*ret).cur = 0 as ::core::ffi::c_uint;
    }
    if (*ret).ctxt.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        xmlBufFree((*ret).buffer);
        xmlFree.expect("non-null function pointer")((*ret).sax as *mut ::core::ffi::c_void);
        xmlFree.expect("non-null function pointer")(ret as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    (*(*ret).ctxt).parseMode = XML_PARSE_READER;
    (*(*ret).ctxt)._private = ret as *mut ::core::ffi::c_void;
    (*(*ret).ctxt).linenumbers = 1 as ::core::ffi::c_int;
    (*(*ret).ctxt).dictNames = 1 as ::core::ffi::c_int;
    (*ret).allocs = XML_TEXTREADER_CTXT;
    (*(*ret).ctxt).docdict = 1 as ::core::ffi::c_int;
    (*ret).dict = (*(*ret).ctxt).dict;
    (*ret).xinclude = 0 as ::core::ffi::c_int;
    (*ret).patternMax = 0 as ::core::ffi::c_int;
    (*ret).patternTab = ::core::ptr::null_mut::<xmlPatternPtr>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextReaderFilename(
    mut URI: *const ::core::ffi::c_char,
) -> xmlTextReaderPtr {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    let mut ret: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    let mut directory: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
        (*(*ret).ctxt).directory = xmlStrdup(directory as *mut xmlChar) as *mut ::core::ffi::c_char;
    }
    if !directory.is_null() {
        xmlFree.expect("non-null function pointer")(directory as *mut ::core::ffi::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeTextReader(mut reader: xmlTextReaderPtr) {
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
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*reader).patternNr {
            if !(*(*reader).patternTab.offset(i as isize)).is_null() {
                xmlFreePattern(*(*reader).patternTab.offset(i as isize));
            }
            i += 1;
        }
        xmlFree.expect("non-null function pointer")(
            (*reader).patternTab as *mut ::core::ffi::c_void,
        );
    }
    if (*reader).mode != XML_TEXTREADER_MODE_CLOSED as ::core::ffi::c_int {
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
        xmlFree.expect("non-null function pointer")((*reader).sax as *mut ::core::ffi::c_void);
    }
    if !(*reader).buffer.is_null() {
        xmlBufFree((*reader).buffer);
    }
    if !(*reader).entTab.is_null() {
        xmlFree.expect("non-null function pointer")((*reader).entTab as *mut ::core::ffi::c_void);
    }
    if !(*reader).dict.is_null() {
        xmlDictFree((*reader).dict);
    }
    xmlFree.expect("non-null function pointer")(reader as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderClose(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*reader).node = ::core::ptr::null_mut::<xmlNode>();
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).mode = XML_TEXTREADER_MODE_CLOSED as ::core::ffi::c_int;
    if !(*reader).faketext.is_null() {
        xmlFreeNode((*reader).faketext);
        (*reader).faketext = ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*reader).ctxt.is_null() {
        if !(*(*reader).ctxt).vctxt.vstateTab.is_null()
            && (*(*reader).ctxt).vctxt.vstateMax > 0 as ::core::ffi::c_int
        {
            while (*(*reader).ctxt).vctxt.vstateNr > 0 as ::core::ffi::c_int {
                xmlValidatePopElement(
                    &raw mut (*(*reader).ctxt).vctxt,
                    ::core::ptr::null_mut::<xmlDoc>(),
                    ::core::ptr::null_mut::<xmlNode>(),
                    ::core::ptr::null::<xmlChar>(),
                );
            }
            xmlFree.expect("non-null function pointer")(
                (*(*reader).ctxt).vctxt.vstateTab as *mut ::core::ffi::c_void,
            );
            (*(*reader).ctxt).vctxt.vstateTab = ::core::ptr::null_mut::<xmlValidState>();
            (*(*reader).ctxt).vctxt.vstateMax = 0 as ::core::ffi::c_int;
        }
        xmlStopParser((*reader).ctxt);
        if !(*(*reader).ctxt).myDoc.is_null() {
            if (*reader).preserve == 0 as ::core::ffi::c_int {
                xmlTextReaderFreeDoc(reader, (*(*reader).ctxt).myDoc);
            }
            (*(*reader).ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
        }
    }
    if !(*reader).input.is_null() && (*reader).allocs & XML_TEXTREADER_INPUT != 0 {
        xmlFreeParserInputBuffer((*reader).input);
        (*reader).allocs -= XML_TEXTREADER_INPUT;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttributeNo(
    mut reader: xmlTextReaderPtr,
    mut no: ::core::ffi::c_int,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut i: ::core::ffi::c_int = 0;
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
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    ns = (*(*reader).node).nsDef as xmlNsPtr;
    i = 0 as ::core::ffi::c_int;
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
        1 as ::core::ffi::c_int,
    );
    if ret.is_null() {
        return xmlStrdup(b"\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttribute(
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
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    localname = xmlSplitQName2(name, &raw mut prefix);
    if localname.is_null() {
        if xmlStrEqual(
            name,
            b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
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
        b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
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
    xmlFree.expect("non-null function pointer")(localname as *mut ::core::ffi::c_void);
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut ::core::ffi::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttributeNs(
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
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if xmlStrEqual(
        namespaceURI,
        b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const ::core::ffi::c_char
            as *mut xmlChar,
    ) != 0
    {
        if xmlStrEqual(
            localName,
            b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
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
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetRemainder(
    mut reader: xmlTextReaderPtr,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    (*reader).node = ::core::ptr::null_mut::<xmlNode>();
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).mode = XML_TEXTREADER_MODE_EOF as ::core::ffi::c_int;
    if !(*reader).ctxt.is_null() {
        xmlStopParser((*reader).ctxt);
        if !(*(*reader).ctxt).myDoc.is_null() {
            if (*reader).preserve == 0 as ::core::ffi::c_int {
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
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/anonymous/artifact/PerfTrans/dataset_source/libxml2/xmlreader.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2405 as ::core::ffi::c_int,
        );
        return ::core::ptr::null_mut::<xmlParserInputBuffer>();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLookupNamespace(
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
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttributeNo(
    mut reader: xmlTextReaderPtr,
    mut no: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut cur: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    ns = (*(*reader).node).nsDef as xmlNsPtr;
    i = 0 as ::core::ffi::c_int;
    while i < no && !ns.is_null() {
        ns = (*ns).next as xmlNsPtr;
        i += 1;
    }
    if !ns.is_null() {
        (*reader).curnode = ns as xmlNodePtr;
        return 1 as ::core::ffi::c_int;
    }
    cur = (*(*reader).node).properties as xmlAttrPtr;
    if cur.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    while i < no {
        cur = (*cur).next as xmlAttrPtr;
        if cur.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    (*reader).curnode = cur as xmlNodePtr;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttribute(
    mut reader: xmlTextReaderPtr,
    mut name: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut localname: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    if reader.is_null() || name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    localname = xmlSplitQName2(name, &raw mut prefix);
    if localname.is_null() {
        if xmlStrEqual(
            name,
            b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
        ) != 0
        {
            ns = (*(*reader).node).nsDef as xmlNsPtr;
            while !ns.is_null() {
                if (*ns).prefix.is_null() {
                    (*reader).curnode = ns as xmlNodePtr;
                    return 1 as ::core::ffi::c_int;
                }
                ns = (*ns).next as xmlNsPtr;
            }
            return 0 as ::core::ffi::c_int;
        }
        prop = (*(*reader).node).properties as xmlAttrPtr;
        while !prop.is_null() {
            if xmlStrEqual((*prop).name, name) != 0
                && ((*prop).ns.is_null() || (*(*prop).ns).prefix.is_null())
            {
                (*reader).curnode = prop as xmlNodePtr;
                return 1 as ::core::ffi::c_int;
            }
            prop = (*prop).next as xmlAttrPtr;
        }
        return 0 as ::core::ffi::c_int;
    }
    if xmlStrEqual(
        prefix,
        b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
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
                xmlFree.expect("non-null function pointer")(localname as *mut ::core::ffi::c_void);
            }
            if !prefix.is_null() {
                xmlFree.expect("non-null function pointer")(prefix as *mut ::core::ffi::c_void);
            }
            return 1 as ::core::ffi::c_int;
        }
        _ => {
            if !localname.is_null() {
                xmlFree.expect("non-null function pointer")(localname as *mut ::core::ffi::c_void);
            }
            if !prefix.is_null() {
                xmlFree.expect("non-null function pointer")(prefix as *mut ::core::ffi::c_void);
            }
            return 0 as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttributeNs(
    mut reader: xmlTextReaderPtr,
    mut localName: *const xmlChar,
    mut namespaceURI: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut prop: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut prefix: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    if reader.is_null() || localName.is_null() || namespaceURI.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    node = (*reader).node;
    if xmlStrEqual(
        namespaceURI,
        b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const ::core::ffi::c_char
            as *mut xmlChar,
    ) != 0
    {
        if xmlStrEqual(
            localName,
            b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
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
                return 1 as ::core::ffi::c_int;
            }
            ns = (*ns).next as xmlNsPtr;
        }
        return 0 as ::core::ffi::c_int;
    }
    prop = (*node).properties as xmlAttrPtr;
    while !prop.is_null() {
        if xmlStrEqual((*prop).name, localName) != 0
            && (!(*prop).ns.is_null() && xmlStrEqual((*(*prop).ns).href, namespaceURI) != 0)
        {
            (*reader).curnode = prop as xmlNodePtr;
            return 1 as ::core::ffi::c_int;
        }
        prop = (*prop).next as xmlAttrPtr;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToFirstAttribute(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*(*reader).node).nsDef.is_null() {
        (*reader).curnode = (*(*reader).node).nsDef as xmlNodePtr;
        return 1 as ::core::ffi::c_int;
    }
    if !(*(*reader).node).properties.is_null() {
        (*reader).curnode = (*(*reader).node).properties as xmlNodePtr;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToNextAttribute(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*reader).curnode.is_null() {
        return xmlTextReaderMoveToFirstAttribute(reader);
    }
    if (*(*reader).curnode).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ns: xmlNsPtr = (*reader).curnode as xmlNsPtr;
        if !(*ns).next.is_null() {
            (*reader).curnode = (*ns).next as xmlNodePtr;
            return 1 as ::core::ffi::c_int;
        }
        if !(*(*reader).node).properties.is_null() {
            (*reader).curnode = (*(*reader).node).properties as xmlNodePtr;
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    } else if (*(*reader).curnode).type_0 as ::core::ffi::c_uint
        == XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*(*reader).curnode).next.is_null()
    {
        (*reader).curnode = (*(*reader).curnode).next as xmlNodePtr;
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToElement(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*reader).curnode.is_null() {
        (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadAttributeValue(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).curnode.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*(*reader).curnode).type_0 as ::core::ffi::c_uint
        == XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*(*reader).curnode).children.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*reader).curnode = (*(*reader).curnode).children as xmlNodePtr;
    } else if (*(*reader).curnode).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
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
                    (*(*reader).faketext).content as *mut ::core::ffi::c_void,
                );
            }
            (*(*reader).faketext).content = xmlStrdup((*ns).href);
        }
        (*reader).curnode = (*reader).faketext;
    } else {
        if (*(*reader).curnode).next.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*reader).curnode = (*(*reader).curnode).next as xmlNodePtr;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstEncoding(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
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
        return xmlDictLookup((*reader).dict, (*doc).encoding, -(1 as ::core::ffi::c_int));
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderAttributeCount(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut attr: xmlAttrPtr = ::core::ptr::null_mut::<xmlAttr>();
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_END as ::core::ffi::c_int
        || (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    ret = 0 as ::core::ffi::c_int;
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
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNodeType(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return XML_READER_TYPE_NONE as ::core::ffi::c_int;
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as ::core::ffi::c_uint {
        1 => {
            if (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_END as ::core::ffi::c_int
                || (*reader).state as ::core::ffi::c_int
                    == XML_TEXTREADER_BACKTRACK as ::core::ffi::c_int
            {
                return XML_READER_TYPE_END_ELEMENT as ::core::ffi::c_int;
            }
            return XML_READER_TYPE_ELEMENT as ::core::ffi::c_int;
        }
        18 | 2 => return XML_READER_TYPE_ATTRIBUTE as ::core::ffi::c_int,
        3 => {
            if xmlIsBlankNode((*reader).node as *const xmlNode) != 0 {
                if xmlNodeGetSpacePreserve((*reader).node as *const xmlNode) != 0 {
                    return XML_READER_TYPE_SIGNIFICANT_WHITESPACE as ::core::ffi::c_int;
                } else {
                    return XML_READER_TYPE_WHITESPACE as ::core::ffi::c_int;
                }
            } else {
                return XML_READER_TYPE_TEXT as ::core::ffi::c_int;
            }
        }
        4 => return XML_READER_TYPE_CDATA as ::core::ffi::c_int,
        5 => return XML_READER_TYPE_ENTITY_REFERENCE as ::core::ffi::c_int,
        6 => return XML_READER_TYPE_ENTITY as ::core::ffi::c_int,
        7 => return XML_READER_TYPE_PROCESSING_INSTRUCTION as ::core::ffi::c_int,
        8 => return XML_READER_TYPE_COMMENT as ::core::ffi::c_int,
        9 | 13 => return XML_READER_TYPE_DOCUMENT as ::core::ffi::c_int,
        11 => return XML_READER_TYPE_DOCUMENT_FRAGMENT as ::core::ffi::c_int,
        12 => return XML_READER_TYPE_NOTATION as ::core::ffi::c_int,
        10 | 14 => return XML_READER_TYPE_DOCUMENT_TYPE as ::core::ffi::c_int,
        15 | 16 | 17 | 19 | 20 => return XML_READER_TYPE_NONE as ::core::ffi::c_int,
        _ => {}
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsEmptyElement(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() || (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*reader).node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*reader).curnode.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*(*reader).node).children.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*reader).state as ::core::ffi::c_int == XML_TEXTREADER_END as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !(*reader).doc.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if (*reader).in_xinclude > 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    return ((*(*reader).node).extra as ::core::ffi::c_int & NODE_IS_EMPTY
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocalName(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return xmlStrdup(
                b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
        } else {
            return xmlStrdup((*ns).prefix);
        }
    }
    if (*node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return xmlTextReaderName(reader);
    }
    return xmlStrdup((*node).name);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstLocalName(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return xmlDictLookup(
                (*reader).dict,
                b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                -(1 as ::core::ffi::c_int),
            );
        } else {
            return (*ns).prefix;
        }
    }
    if (*node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return xmlTextReaderConstName(reader);
    }
    return (*node).name;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderName(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
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
    match (*node).type_0 as ::core::ffi::c_uint {
        1 | 2 => {
            if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
                return xmlStrdup((*node).name);
            }
            ret = xmlStrdup((*(*node).ns).prefix);
            ret = xmlStrcat(
                ret,
                b":\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
            ret = xmlStrcat(ret, (*node).name);
            return ret;
        }
        3 => {
            return xmlStrdup(
                b"#text\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
        }
        4 => {
            return xmlStrdup(
                b"#cdata-section\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
        }
        6 | 5 => return xmlStrdup((*node).name),
        7 => return xmlStrdup((*node).name),
        8 => {
            return xmlStrdup(
                b"#comment\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
        }
        9 | 13 => {
            return xmlStrdup(
                b"#document\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
        }
        11 => {
            return xmlStrdup(
                b"#document-fragment\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
        }
        12 => return xmlStrdup((*node).name),
        10 | 14 => return xmlStrdup((*node).name),
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            ret = xmlStrdup(b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
            if (*ns).prefix.is_null() {
                return ret;
            }
            ret = xmlStrcat(
                ret,
                b":\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            );
            ret = xmlStrcat(ret, (*ns).prefix);
            return ret;
        }
        15 | 16 | 17 | 19 | 20 => return ::core::ptr::null_mut::<xmlChar>(),
        _ => {}
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstName(mut reader: xmlTextReaderPtr) -> *const xmlChar {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as ::core::ffi::c_uint {
        1 | 2 => {
            if (*node).ns.is_null() || (*(*node).ns).prefix.is_null() {
                return (*node).name;
            }
            return xmlDictQLookup((*reader).dict, (*(*node).ns).prefix, (*node).name);
        }
        3 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#text\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                -(1 as ::core::ffi::c_int),
            );
        }
        4 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#cdata-section\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                -(1 as ::core::ffi::c_int),
            );
        }
        6 | 5 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as ::core::ffi::c_int));
        }
        7 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as ::core::ffi::c_int));
        }
        8 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#comment\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                -(1 as ::core::ffi::c_int),
            );
        }
        9 | 13 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#document\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                -(1 as ::core::ffi::c_int),
            );
        }
        11 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#document-fragment\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                -(1 as ::core::ffi::c_int),
            );
        }
        12 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as ::core::ffi::c_int));
        }
        10 | 14 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as ::core::ffi::c_int));
        }
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            if (*ns).prefix.is_null() {
                return xmlDictLookup(
                    (*reader).dict,
                    b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                    -(1 as ::core::ffi::c_int),
                );
            }
            return xmlDictQLookup(
                (*reader).dict,
                b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
                (*ns).prefix,
            );
        }
        15 | 16 | 17 | 19 | 20 => return ::core::ptr::null::<xmlChar>(),
        _ => {}
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPrefix(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return ::core::ptr::null_mut::<xmlChar>();
        }
        return xmlStrdup(b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar);
    }
    if (*node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
        return xmlStrdup((*(*node).ns).prefix);
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstPrefix(mut reader: xmlTextReaderPtr) -> *const xmlChar {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if (*ns).prefix.is_null() {
            return ::core::ptr::null::<xmlChar>();
        }
        return xmlDictLookup(
            (*reader).dict,
            b"xmlns\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar,
            -(1 as ::core::ffi::c_int),
        );
    }
    if (*node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*node).ns.is_null() && !(*(*node).ns).prefix.is_null() {
        return xmlDictLookup(
            (*reader).dict,
            (*(*node).ns).prefix,
            -(1 as ::core::ffi::c_int),
        );
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNamespaceUri(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return xmlStrdup(
            b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const ::core::ffi::c_char
                as *mut xmlChar,
        );
    }
    if (*node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if !(*node).ns.is_null() {
        return xmlStrdup((*(*node).ns).href);
    }
    return ::core::ptr::null_mut::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstNamespaceUri(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return xmlDictLookup(
            (*reader).dict,
            b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const ::core::ffi::c_char
                as *mut xmlChar,
            -(1 as ::core::ffi::c_int),
        );
    }
    if (*node).type_0 as ::core::ffi::c_uint
        != XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*node).type_0 as ::core::ffi::c_uint
            != XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<xmlChar>();
    }
    if !(*node).ns.is_null() {
        return xmlDictLookup(
            (*reader).dict,
            (*(*node).ns).href,
            -(1 as ::core::ffi::c_int),
        );
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderBaseUri(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
    if reader.is_null() || (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlNodeGetBase(
        ::core::ptr::null::<xmlDoc>(),
        (*reader).node as *const xmlNode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstBaseUri(mut reader: xmlTextReaderPtr) -> *const xmlChar {
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
    ret = xmlDictLookup((*reader).dict, tmp, -(1 as ::core::ffi::c_int));
    xmlFree.expect("non-null function pointer")(tmp as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderDepth(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*reader).curnode.is_null() {
        if (*(*reader).curnode).type_0 as ::core::ffi::c_uint
            == XML_ATTRIBUTE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*(*reader).curnode).type_0 as ::core::ffi::c_uint
                == XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return (*reader).depth + 1 as ::core::ffi::c_int;
        }
        return (*reader).depth + 2 as ::core::ffi::c_int;
    }
    return (*reader).depth;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderHasAttributes(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as ::core::ffi::c_uint
        == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (!(*node).properties.is_null() || !(*node).nsDef.is_null())
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderHasValue(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as ::core::ffi::c_uint {
        2 | 3 | 4 | 7 | 8 | 18 => return 1 as ::core::ffi::c_int,
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderValue(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
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
    match (*node).type_0 as ::core::ffi::c_uint {
        18 => return xmlStrdup((*(node as xmlNsPtr)).href),
        2 => {
            let mut attr: xmlAttrPtr = node as xmlAttrPtr;
            if !(*attr).parent.is_null() {
                return xmlNodeListGetString(
                    (*(*attr).parent).doc as xmlDocPtr,
                    (*attr).children,
                    1 as ::core::ffi::c_int,
                );
            } else {
                return xmlNodeListGetString(
                    ::core::ptr::null_mut::<xmlDoc>(),
                    (*attr).children,
                    1 as ::core::ffi::c_int,
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
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstValue(mut reader: xmlTextReaderPtr) -> *const xmlChar {
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
    match (*node).type_0 as ::core::ffi::c_uint {
        18 => return (*(node as xmlNsPtr)).href,
        2 => {
            let mut attr: xmlAttrPtr = node as xmlAttrPtr;
            let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
            if !(*attr).children.is_null()
                && (*(*attr).children).type_0 as ::core::ffi::c_uint
                    == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
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
                                as *const ::core::ffi::c_char,
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
                    ret = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut xmlChar;
                }
                return ret;
            }
        }
        3 | 4 | 7 | 8 => return (*node).content,
        _ => {}
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsDefault(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderQuoteChar(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return '"' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderXmlLang(mut reader: xmlTextReaderPtr) -> *mut xmlChar {
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    if (*reader).node.is_null() {
        return ::core::ptr::null_mut::<xmlChar>();
    }
    return xmlNodeGetLang((*reader).node as *const xmlNode);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstXmlLang(mut reader: xmlTextReaderPtr) -> *const xmlChar {
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
    ret = xmlDictLookup((*reader).dict, tmp, -(1 as ::core::ffi::c_int));
    xmlFree.expect("non-null function pointer")(tmp as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstString(
    mut reader: xmlTextReaderPtr,
    mut str: *const xmlChar,
) -> *const xmlChar {
    if reader.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    return xmlDictLookup((*reader).dict, str, -(1 as ::core::ffi::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNormalization(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetParserProp(
    mut reader: xmlTextReaderPtr,
    mut prop: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: xmlParserProperties = prop as xmlParserProperties;
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ctxt = (*reader).ctxt;
    match p as ::core::ffi::c_uint {
        1 => {
            if value != 0 as ::core::ffi::c_int {
                if (*ctxt).loadsubset == 0 as ::core::ffi::c_int {
                    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int {
                        return -(1 as ::core::ffi::c_int);
                    }
                    (*ctxt).loadsubset = XML_DETECT_IDS;
                }
            } else {
                (*ctxt).loadsubset = 0 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        2 => {
            if value != 0 as ::core::ffi::c_int {
                (*ctxt).loadsubset |= XML_COMPLETE_ATTRS;
            } else if (*ctxt).loadsubset & XML_COMPLETE_ATTRS != 0 {
                (*ctxt).loadsubset -= XML_COMPLETE_ATTRS;
            }
            return 0 as ::core::ffi::c_int;
        }
        3 => {
            if value != 0 as ::core::ffi::c_int {
                (*ctxt).options |= XML_PARSE_DTDVALID as ::core::ffi::c_int;
                (*ctxt).validate = 1 as ::core::ffi::c_int;
                (*reader).validate = XML_TEXTREADER_VALIDATE_DTD;
            } else {
                (*ctxt).options &= !(XML_PARSE_DTDVALID as ::core::ffi::c_int);
                (*ctxt).validate = 0 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        4 => {
            if value != 0 as ::core::ffi::c_int {
                (*ctxt).options |= XML_PARSE_NOENT as ::core::ffi::c_int;
                (*ctxt).replaceEntities = 1 as ::core::ffi::c_int;
            } else {
                (*ctxt).options &= !(XML_PARSE_NOENT as ::core::ffi::c_int);
                (*ctxt).replaceEntities = 0 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserProp(
    mut reader: xmlTextReaderPtr,
    mut prop: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: xmlParserProperties = prop as xmlParserProperties;
    let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ctxt = (*reader).ctxt;
    match p as ::core::ffi::c_uint {
        1 => {
            if (*ctxt).loadsubset != 0 as ::core::ffi::c_int
                || (*ctxt).validate != 0 as ::core::ffi::c_int
            {
                return 1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        2 => {
            if (*ctxt).loadsubset & XML_COMPLETE_ATTRS != 0 {
                return 1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        3 => return (*reader).validate as ::core::ffi::c_int,
        4 => return (*ctxt).replaceEntities,
        _ => {}
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserLineNumber(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() || (*reader).ctxt.is_null() || (*(*reader).ctxt).input.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return (*(*(*reader).ctxt).input).line;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserColumnNumber(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() || (*reader).ctxt.is_null() || (*(*reader).ctxt).input.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return (*(*(*reader).ctxt).input).col;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderCurrentNode(mut reader: xmlTextReaderPtr) -> xmlNodePtr {
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlNode>();
    }
    if !(*reader).curnode.is_null() {
        return (*reader).curnode;
    }
    return (*reader).node;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPreserve(mut reader: xmlTextReaderPtr) -> xmlNodePtr {
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
    if (*cur).type_0 as ::core::ffi::c_uint
        != XML_DOCUMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cur).type_0 as ::core::ffi::c_uint
            != XML_DTD_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cur).extra =
            ((*cur).extra as ::core::ffi::c_int | NODE_IS_PRESERVED) as ::core::ffi::c_ushort;
        (*cur).extra =
            ((*cur).extra as ::core::ffi::c_int | NODE_IS_SPRESERVED) as ::core::ffi::c_ushort;
    }
    (*reader).preserves += 1;
    parent = (*cur).parent as xmlNodePtr;
    while !parent.is_null() {
        if (*parent).type_0 as ::core::ffi::c_uint
            == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*parent).extra = ((*parent).extra as ::core::ffi::c_int | NODE_IS_PRESERVED)
                as ::core::ffi::c_ushort;
        }
        parent = (*parent).parent as xmlNodePtr;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPreservePattern(
    mut reader: xmlTextReaderPtr,
    mut pattern: *const xmlChar,
    mut namespaces: *mut *const xmlChar,
) -> ::core::ffi::c_int {
    let mut comp: xmlPatternPtr = ::core::ptr::null_mut::<xmlPattern>();
    if reader.is_null() || pattern.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    comp = xmlPatterncompile(
        pattern,
        (*reader).dict as *mut xmlDict,
        0 as ::core::ffi::c_int,
        namespaces,
    );
    if comp.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).patternMax <= 0 as ::core::ffi::c_int {
        (*reader).patternMax = 4 as ::core::ffi::c_int;
        (*reader).patternTab = xmlMalloc.expect("non-null function pointer")(
            ((*reader).patternMax as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlPatternPtr>() as size_t),
        ) as *mut xmlPatternPtr;
        if (*reader).patternTab.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlMalloc failed !\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    if (*reader).patternNr >= (*reader).patternMax {
        let mut tmp: *mut xmlPatternPtr = ::core::ptr::null_mut::<xmlPatternPtr>();
        (*reader).patternMax *= 2 as ::core::ffi::c_int;
        tmp = xmlRealloc.expect("non-null function pointer")(
            (*reader).patternTab as *mut ::core::ffi::c_void,
            ((*reader).patternMax as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlPatternPtr>() as size_t),
        ) as *mut xmlPatternPtr;
        if tmp.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            (*reader).patternMax /= 2 as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
        (*reader).patternTab = tmp;
    }
    let ref mut fresh4 = *(*reader).patternTab.offset((*reader).patternNr as isize);
    *fresh4 = comp;
    let fresh5 = (*reader).patternNr;
    (*reader).patternNr = (*reader).patternNr + 1;
    return fresh5;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderCurrentDoc(mut reader: xmlTextReaderPtr) -> xmlDocPtr {
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    if !(*reader).doc.is_null() {
        return (*reader).doc;
    }
    if (*reader).ctxt.is_null() || (*(*reader).ctxt).myDoc.is_null() {
        return ::core::ptr::null_mut::<xmlDoc>();
    }
    (*reader).preserve = 1 as ::core::ffi::c_int;
    return (*(*reader).ctxt).myDoc;
}
unsafe extern "C" fn xmlTextReaderValidityErrorRelay(
    mut ctx: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut reader: xmlTextReaderPtr = ctx as xmlTextReaderPtr;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    str = xmlTextReaderBuildMessage(msg, ap.as_va_list());
    if (*reader).errorFunc.is_none() {
        xmlTextReaderValidityError(ctx, b"%s\0" as *const u8 as *const ::core::ffi::c_char, str);
    } else {
        (*reader).errorFunc.expect("non-null function pointer")(
            (*reader).errorFuncArg,
            str,
            XML_PARSER_SEVERITY_VALIDITY_ERROR,
            NULL,
        );
    }
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn xmlTextReaderValidityWarningRelay(
    mut ctx: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut reader: xmlTextReaderPtr = ctx as xmlTextReaderPtr;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    str = xmlTextReaderBuildMessage(msg, ap.as_va_list());
    if (*reader).errorFunc.is_none() {
        xmlTextReaderValidityWarning(ctx, b"%s\0" as *const u8 as *const ::core::ffi::c_char, str);
    } else {
        (*reader).errorFunc.expect("non-null function pointer")(
            (*reader).errorFuncArg,
            str,
            XML_PARSER_SEVERITY_VALIDITY_WARNING,
            NULL,
        );
    }
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn xmlTextReaderValidityStructuredRelay(
    mut userData: *mut ::core::ffi::c_void,
    mut error: *const xmlError,
) {
    let mut reader: xmlTextReaderPtr = userData as xmlTextReaderPtr;
    if (*reader).sErrorFunc.is_some() {
        (*reader).sErrorFunc.expect("non-null function pointer")((*reader).errorFuncArg, error);
    } else {
        xmlTextReaderStructuredError(reader as *mut ::core::ffi::c_void, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGSetSchema(
    mut reader: xmlTextReaderPtr,
    mut schema: xmlRelaxNGPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
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
        (*reader).rngPreserveCtxt = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
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
    (*reader).rngPreserveCtxt = 0 as ::core::ffi::c_int;
    (*reader).rngValidCtxt = xmlRelaxNGNewValidCtxt(schema);
    if (*reader).rngValidCtxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).errorFunc.is_some() {
        xmlRelaxNGSetValidErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
            reader as *mut ::core::ffi::c_void,
        );
    }
    if (*reader).sErrorFunc.is_some() {
        xmlRelaxNGSetValidStructuredErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> (),
            ),
            reader as *mut ::core::ffi::c_void,
        );
    }
    (*reader).rngValidErrors = 0 as ::core::ffi::c_int;
    (*reader).rngFullNode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).validate = XML_TEXTREADER_VALIDATE_RNG;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlTextReaderLocator(
    mut ctx: *mut ::core::ffi::c_void,
    mut file: *mut *const ::core::ffi::c_char,
    mut line: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    if ctx.is_null() || file.is_null() && line.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !file.is_null() {
        *file = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !line.is_null() {
        *line = 0 as ::core::ffi::c_ulong;
    }
    reader = ctx as xmlTextReaderPtr;
    if !(*reader).ctxt.is_null() && !(*(*reader).ctxt).input.is_null() {
        if !file.is_null() {
            *file = (*(*(*reader).ctxt).input).filename;
        }
        if !line.is_null() {
            *line = (*(*(*reader).ctxt).input).line as ::core::ffi::c_ulong;
        }
        return 0 as ::core::ffi::c_int;
    }
    if !(*reader).node.is_null() {
        let mut res: ::core::ffi::c_long = 0;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !line.is_null() {
            res = xmlGetLineNo((*reader).node as *const xmlNode);
            if res > 0 as ::core::ffi::c_long {
                *line = res as ::core::ffi::c_ulong;
            } else {
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        if !file.is_null() {
            let mut doc: xmlDocPtr = (*(*reader).node).doc as xmlDocPtr;
            if !doc.is_null() && !(*doc).URL.is_null() {
                *file = (*doc).URL as *const ::core::ffi::c_char;
            } else {
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        return ret;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetSchema(
    mut reader: xmlTextReaderPtr,
    mut schema: xmlSchemaPtr,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
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
        (*reader).xsdPreserveCtxt = 0 as ::core::ffi::c_int;
        if !(*reader).xsdSchemas.is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
        }
        return 0 as ::core::ffi::c_int;
    }
    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
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
    (*reader).xsdPreserveCtxt = 0 as ::core::ffi::c_int;
    if !(*reader).xsdSchemas.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
    }
    (*reader).xsdValidCtxt = xmlSchemaNewValidCtxt(schema);
    if (*reader).xsdValidCtxt.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
        return -(1 as ::core::ffi::c_int);
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
        return -(1 as ::core::ffi::c_int);
    }
    xmlSchemaValidateSetLocator(
        (*reader).xsdValidCtxt,
        Some(
            xmlTextReaderLocator
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_ulong,
                ) -> ::core::ffi::c_int,
        ),
        reader as *mut ::core::ffi::c_void,
    );
    if (*reader).errorFunc.is_some() {
        xmlSchemaSetValidErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
            reader as *mut ::core::ffi::c_void,
        );
    }
    if (*reader).sErrorFunc.is_some() {
        xmlSchemaSetValidStructuredErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> (),
            ),
            reader as *mut ::core::ffi::c_void,
        );
    }
    (*reader).xsdValidErrors = 0 as ::core::ffi::c_int;
    (*reader).validate = XML_TEXTREADER_VALIDATE_XSD;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlTextReaderRelaxNGValidateInternal(
    mut reader: xmlTextReaderPtr,
    mut rng: *const ::core::ffi::c_char,
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !rng.is_null() && !ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (!rng.is_null() || !ctxt.is_null())
        && ((*reader).mode != XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int
            || (*reader).ctxt.is_null())
    {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*reader).rngValidCtxt.is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        (*reader).rngValidCtxt = ::core::ptr::null_mut::<xmlRelaxNGValidCtxt>();
    }
    (*reader).rngPreserveCtxt = 0 as ::core::ffi::c_int;
    if !(*reader).rngSchemas.is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        (*reader).rngSchemas = ::core::ptr::null_mut::<xmlRelaxNG>();
    }
    if rng.is_null() && ctxt.is_null() {
        return 0 as ::core::ffi::c_int;
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
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ...
                        ) -> (),
                ),
                reader as *mut ::core::ffi::c_void,
            );
        }
        if (*reader).sErrorFunc.is_some() {
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> (),
                ),
                reader as *mut ::core::ffi::c_void,
            );
        }
        (*reader).rngSchemas = xmlRelaxNGParse(pctxt);
        xmlRelaxNGFreeParserCtxt(pctxt);
        if (*reader).rngSchemas.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        (*reader).rngValidCtxt = xmlRelaxNGNewValidCtxt((*reader).rngSchemas);
        if (*reader).rngValidCtxt.is_null() {
            xmlRelaxNGFree((*reader).rngSchemas);
            (*reader).rngSchemas = ::core::ptr::null_mut::<xmlRelaxNG>();
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        (*reader).rngValidCtxt = ctxt;
        (*reader).rngPreserveCtxt = 1 as ::core::ffi::c_int;
    }
    if (*reader).errorFunc.is_some() {
        xmlRelaxNGSetValidErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
            reader as *mut ::core::ffi::c_void,
        );
    }
    if (*reader).sErrorFunc.is_some() {
        xmlRelaxNGSetValidStructuredErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> (),
            ),
            reader as *mut ::core::ffi::c_void,
        );
    }
    (*reader).rngValidErrors = 0 as ::core::ffi::c_int;
    (*reader).rngFullNode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).validate = XML_TEXTREADER_VALIDATE_RNG;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlTextReaderSchemaValidateInternal(
    mut reader: xmlTextReaderPtr,
    mut xsd: *const ::core::ffi::c_char,
    mut ctxt: xmlSchemaValidCtxtPtr,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !xsd.is_null() && !ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (!xsd.is_null() || !ctxt.is_null())
        && ((*reader).mode != XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int
            || (*reader).ctxt.is_null())
    {
        return -(1 as ::core::ffi::c_int);
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
    (*reader).xsdPreserveCtxt = 0 as ::core::ffi::c_int;
    if !(*reader).xsdSchemas.is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
    }
    if xsd.is_null() && ctxt.is_null() {
        return 0 as ::core::ffi::c_int;
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
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ...
                        ) -> (),
                ),
                reader as *mut ::core::ffi::c_void,
            );
        }
        (*reader).xsdSchemas = xmlSchemaParse(pctxt);
        xmlSchemaFreeParserCtxt(pctxt);
        if (*reader).xsdSchemas.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        (*reader).xsdValidCtxt = xmlSchemaNewValidCtxt((*reader).xsdSchemas);
        if (*reader).xsdValidCtxt.is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            (*reader).xsdSchemas = ::core::ptr::null_mut::<xmlSchema>();
            return -(1 as ::core::ffi::c_int);
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
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        (*reader).xsdValidCtxt = ctxt;
        (*reader).xsdPreserveCtxt = 1 as ::core::ffi::c_int;
        (*reader).xsdPlug = xmlSchemaSAXPlug(
            (*reader).xsdValidCtxt,
            &raw mut (*(*reader).ctxt).sax,
            &raw mut (*(*reader).ctxt).userData,
        );
        if (*reader).xsdPlug.is_null() {
            (*reader).xsdValidCtxt = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
            (*reader).xsdPreserveCtxt = 0 as ::core::ffi::c_int;
            return -(1 as ::core::ffi::c_int);
        }
    }
    xmlSchemaValidateSetLocator(
        (*reader).xsdValidCtxt,
        Some(
            xmlTextReaderLocator
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_ulong,
                ) -> ::core::ffi::c_int,
        ),
        reader as *mut ::core::ffi::c_void,
    );
    if (*reader).errorFunc.is_some() {
        xmlSchemaSetValidErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
            reader as *mut ::core::ffi::c_void,
        );
    }
    if (*reader).sErrorFunc.is_some() {
        xmlSchemaSetValidStructuredErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> (),
            ),
            reader as *mut ::core::ffi::c_void,
        );
    }
    (*reader).xsdValidErrors = 0 as ::core::ffi::c_int;
    (*reader).validate = XML_TEXTREADER_VALIDATE_XSD;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSchemaValidateCtxt(
    mut reader: xmlTextReaderPtr,
    mut ctxt: xmlSchemaValidCtxtPtr,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return xmlTextReaderSchemaValidateInternal(
        reader,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ctxt,
        options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSchemaValidate(
    mut reader: xmlTextReaderPtr,
    mut xsd: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return xmlTextReaderSchemaValidateInternal(
        reader,
        xsd,
        ::core::ptr::null_mut::<xmlSchemaValidCtxt>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGValidateCtxt(
    mut reader: xmlTextReaderPtr,
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return xmlTextReaderRelaxNGValidateInternal(
        reader,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ctxt,
        options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGValidate(
    mut reader: xmlTextReaderPtr,
    mut rng: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return xmlTextReaderRelaxNGValidateInternal(
        reader,
        rng,
        ::core::ptr::null_mut::<xmlRelaxNGValidCtxt>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsNamespaceDecl(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).node.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*reader).curnode.is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if XML_NAMESPACE_DECL as ::core::ffi::c_int as ::core::ffi::c_uint
        == (*node).type_0 as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstXmlVersion(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
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
        return xmlDictLookup((*reader).dict, (*doc).version, -(1 as ::core::ffi::c_int));
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderStandalone(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_int {
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*reader).doc.is_null() {
        doc = (*reader).doc;
    } else if !(*reader).ctxt.is_null() {
        doc = (*(*reader).ctxt).myDoc;
    }
    if doc.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return (*doc).standalone;
}
unsafe extern "C" fn xmlTextReaderBuildMessage(
    mut msg: *const ::core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) -> *mut ::core::ffi::c_char {
    let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut chars: ::core::ffi::c_int = 0;
    let mut larger: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut aq: ::core::ffi::VaListImpl;
    loop {
        aq = ap.clone();
        chars = vsnprintf(str, size as size_t, msg, aq.as_va_list());
        if chars < 0 as ::core::ffi::c_int {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"vsnprintf failed !\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !str.is_null() {
                xmlFree.expect("non-null function pointer")(str as *mut ::core::ffi::c_void);
            }
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if chars < size || size == MAX_ERR_MSG_SIZE {
            break;
        }
        if chars < MAX_ERR_MSG_SIZE {
            size = chars + 1 as ::core::ffi::c_int;
        } else {
            size = MAX_ERR_MSG_SIZE;
        }
        larger = xmlRealloc.expect("non-null function pointer")(
            str as *mut ::core::ffi::c_void,
            size as size_t,
        ) as *mut ::core::ffi::c_char;
        if larger.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !str.is_null() {
                xmlFree.expect("non-null function pointer")(str as *mut ::core::ffi::c_void);
            }
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        str = larger;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocatorLineNumber(
    mut locator: xmlTextReaderLocatorPtr,
) -> ::core::ffi::c_int {
    let mut ctx: xmlParserCtxtPtr = locator as xmlParserCtxtPtr;
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if locator.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*ctx).node.is_null() {
        ret = xmlGetLineNo((*ctx).node as *const xmlNode) as ::core::ffi::c_int;
    } else {
        let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
        input = (*ctx).input;
        if (*input).filename.is_null() && (*ctx).inputNr > 1 as ::core::ffi::c_int {
            input = *(*ctx)
                .inputTab
                .offset(((*ctx).inputNr - 2 as ::core::ffi::c_int) as isize);
        }
        if !input.is_null() {
            ret = (*input).line;
        } else {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocatorBaseURI(
    mut locator: xmlTextReaderLocatorPtr,
) -> *mut xmlChar {
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
        if (*input).filename.is_null() && (*ctx).inputNr > 1 as ::core::ffi::c_int {
            input = *(*ctx)
                .inputTab
                .offset(((*ctx).inputNr - 2 as ::core::ffi::c_int) as isize);
        }
        if !input.is_null() {
            ret = xmlStrdup((*input).filename as *mut xmlChar);
        } else {
            ret = ::core::ptr::null_mut::<xmlChar>();
        }
    }
    return ret;
}
unsafe extern "C" fn xmlTextReaderGenericError(
    mut ctxt: *mut ::core::ffi::c_void,
    mut severity: xmlParserSeverities,
    mut str: *mut ::core::ffi::c_char,
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
        xmlFree.expect("non-null function pointer")(str as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn xmlTextReaderStructuredError(
    mut ctxt: *mut ::core::ffi::c_void,
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
    mut ctxt: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
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
    mut ctxt: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
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
    mut ctxt: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut len: ::core::ffi::c_int = xmlStrlen(msg as *const xmlChar);
    if len > 1 as ::core::ffi::c_int
        && *msg.offset((len - 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ':' as i32
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
    mut ctxt: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut len: ::core::ffi::c_int = xmlStrlen(msg as *const xmlChar);
    if len != 0 as ::core::ffi::c_int
        && *msg.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != ':' as i32
    {
        ap = args.clone();
        xmlTextReaderGenericError(
            ctxt,
            XML_PARSER_SEVERITY_VALIDITY_WARNING,
            xmlTextReaderBuildMessage(msg, ap.as_va_list()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: xmlTextReaderErrorFunc,
    mut arg: *mut ::core::ffi::c_void,
) {
    if f.is_some() {
        (*(*(*reader).ctxt).sax).error = Some(
            xmlTextReaderError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as errorSAXFunc;
        (*(*(*reader).ctxt).sax).serror = None;
        (*(*reader).ctxt).vctxt.error = Some(
            xmlTextReaderValidityError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*(*(*reader).ctxt).sax).warning = Some(
            xmlTextReaderWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*(*reader).ctxt).vctxt.warning = Some(
            xmlTextReaderValidityWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
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
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ...
                        ) -> (),
                ),
                reader as *mut ::core::ffi::c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut ::core::ffi::c_void,
            );
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                            ...
                        ) -> (),
                ),
                reader as *mut ::core::ffi::c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut ::core::ffi::c_void,
            );
        }
    } else {
        (*(*(*reader).ctxt).sax).error = Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as errorSAXFunc;
        (*(*reader).ctxt).vctxt.error = Some(
            xmlParserValidityError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*(*(*reader).ctxt).sax).warning = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*(*reader).ctxt).vctxt.warning = Some(
            xmlParserValidityWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
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
                reader as *mut ::core::ffi::c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut ::core::ffi::c_void,
            );
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut ::core::ffi::c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut ::core::ffi::c_void,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetStructuredErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: xmlStructuredErrorFunc,
    mut arg: *mut ::core::ffi::c_void,
) {
    if f.is_some() {
        (*(*(*reader).ctxt).sax).error = None;
        (*(*(*reader).ctxt).sax).serror = Some(
            xmlTextReaderStructuredError
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> (),
        ) as xmlStructuredErrorFunc;
        (*(*reader).ctxt).vctxt.error = Some(
            xmlTextReaderValidityError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*(*(*reader).ctxt).sax).warning = Some(
            xmlTextReaderWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*(*reader).ctxt).vctxt.warning = Some(
            xmlTextReaderValidityWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
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
                reader as *mut ::core::ffi::c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> (),
                ),
                reader as *mut ::core::ffi::c_void,
            );
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut ::core::ffi::c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> (),
                ),
                reader as *mut ::core::ffi::c_void,
            );
        }
    } else {
        (*(*(*reader).ctxt).sax).error = Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as errorSAXFunc;
        (*(*(*reader).ctxt).sax).serror = None;
        (*(*reader).ctxt).vctxt.error = Some(
            xmlParserValidityError
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*(*(*reader).ctxt).sax).warning = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*(*reader).ctxt).vctxt.warning = Some(
            xmlParserValidityWarning
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
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
                reader as *mut ::core::ffi::c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut ::core::ffi::c_void,
            );
        }
        if !(*reader).xsdValidCtxt.is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut ::core::ffi::c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut ::core::ffi::c_void,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsValid(mut reader: xmlTextReaderPtr) -> ::core::ffi::c_int {
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*reader).validate as ::core::ffi::c_uint
        == XML_TEXTREADER_VALIDATE_RNG as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ((*reader).rngValidErrors == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    if (*reader).validate as ::core::ffi::c_uint
        == XML_TEXTREADER_VALIDATE_XSD as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return ((*reader).xsdValidErrors == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    if !(*reader).ctxt.is_null() && (*(*reader).ctxt).validate == 1 as ::core::ffi::c_int {
        return (*(*reader).ctxt).valid;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: *mut xmlTextReaderErrorFunc,
    mut arg: *mut *mut ::core::ffi::c_void,
) {
    if !f.is_null() {
        *f = (*reader).errorFunc;
    }
    if !arg.is_null() {
        *arg = (*reader).errorFuncArg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetup(
    mut reader: xmlTextReaderPtr,
    mut input: xmlParserInputBufferPtr,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if reader.is_null() {
        if !input.is_null() {
            xmlFreeParserInputBuffer(input);
        }
        return -(1 as ::core::ffi::c_int);
    }
    options |= XML_PARSE_COMPACT as ::core::ffi::c_int;
    (*reader).doc = ::core::ptr::null_mut::<xmlDoc>();
    (*reader).entNr = 0 as ::core::ffi::c_int;
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
            b"xmlTextReaderSetup : malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
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
            b"xmlTextReaderSetup : malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    xmlSAXVersion((*reader).sax as *mut xmlSAXHandler, 2 as ::core::ffi::c_int);
    (*reader).startElement = (*(*reader).sax).startElement;
    (*(*reader).sax).startElement = Some(
        xmlTextReaderStartElement
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    ) as startElementSAXFunc;
    (*reader).endElement = (*(*reader).sax).endElement;
    (*(*reader).sax).endElement = Some(
        xmlTextReaderEndElement
            as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> (),
    ) as endElementSAXFunc;
    if (*(*reader).sax).initialized == XML_SAX2_MAGIC {
        (*reader).startElementNs = (*(*reader).sax).startElementNs;
        (*(*reader).sax).startElementNs = Some(
            xmlTextReaderStartElementNs
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    ::core::ffi::c_int,
                    *mut *const xmlChar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *const xmlChar,
                ) -> (),
        ) as startElementNsSAX2Func;
        (*reader).endElementNs = (*(*reader).sax).endElementNs;
        (*(*reader).sax).endElementNs = Some(
            xmlTextReaderEndElementNs
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
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
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as charactersSAXFunc;
    (*(*reader).sax).ignorableWhitespace = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as ignorableWhitespaceSAXFunc;
    (*reader).cdataBlock = (*(*reader).sax).cdataBlock;
    (*(*reader).sax).cdataBlock = Some(
        xmlTextReaderCDataBlock
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const xmlChar,
                ::core::ffi::c_int,
            ) -> (),
    ) as cdataBlockSAXFunc;
    (*reader).mode = XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int;
    (*reader).node = ::core::ptr::null_mut::<xmlNode>();
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    if !input.is_null() {
        if xmlBufUse((*(*reader).input).buffer) < 4 as size_t {
            xmlParserInputBufferRead(input, 4 as ::core::ffi::c_int);
        }
        if (*reader).ctxt.is_null() {
            if xmlBufUse((*(*reader).input).buffer) >= 4 as size_t {
                (*reader).ctxt = xmlCreatePushParserCtxt(
                    (*reader).sax,
                    NULL,
                    xmlBufContent((*(*reader).input).buffer as *const xmlBuf)
                        as *const ::core::ffi::c_char,
                    4 as ::core::ffi::c_int,
                    URL,
                );
                (*reader).base = 0 as ::core::ffi::c_uint;
                (*reader).cur = 4 as ::core::ffi::c_uint;
            } else {
                (*reader).ctxt = xmlCreatePushParserCtxt(
                    (*reader).sax,
                    NULL,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                    URL,
                );
                (*reader).base = 0 as ::core::ffi::c_uint;
                (*reader).cur = 0 as ::core::ffi::c_uint;
            }
        } else {
            let mut inputStream: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
            let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
            let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
            xmlCtxtReset((*reader).ctxt);
            buf = xmlAllocParserInputBuffer(enc);
            if buf.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
            inputStream = xmlNewInputStream((*reader).ctxt);
            if inputStream.is_null() {
                xmlFreeParserInputBuffer(buf);
                return -(1 as ::core::ffi::c_int);
            }
            if URL.is_null() {
                (*inputStream).filename = ::core::ptr::null::<::core::ffi::c_char>();
            } else {
                (*inputStream).filename =
                    xmlCanonicPath(URL as *const xmlChar) as *mut ::core::ffi::c_char;
            }
            (*inputStream).buf = buf;
            xmlBufResetInput((*buf).buffer, inputStream);
            inputPush((*reader).ctxt, inputStream);
            (*reader).cur = 0 as ::core::ffi::c_uint;
        }
        if (*reader).ctxt.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"xmlTextReaderSetup : malloc failed\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
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
    (*(*reader).ctxt)._private = reader as *mut ::core::ffi::c_void;
    (*(*reader).ctxt).linenumbers = 1 as ::core::ffi::c_int;
    (*(*reader).ctxt).dictNames = 1 as ::core::ffi::c_int;
    (*(*reader).ctxt).docdict = 1 as ::core::ffi::c_int;
    (*(*reader).ctxt).parseMode = XML_PARSE_READER;
    if !(*reader).xincctxt.is_null() {
        xmlXIncludeFreeContext((*reader).xincctxt);
        (*reader).xincctxt = ::core::ptr::null_mut::<xmlXIncludeCtxt>();
    }
    if options & XML_PARSE_XINCLUDE as ::core::ffi::c_int != 0 {
        (*reader).xinclude = 1 as ::core::ffi::c_int;
        (*reader).xinclude_name =
            xmlDictLookup((*reader).dict, XINCLUDE_NODE, -(1 as ::core::ffi::c_int));
        options -= XML_PARSE_XINCLUDE as ::core::ffi::c_int;
    } else {
        (*reader).xinclude = 0 as ::core::ffi::c_int;
    }
    (*reader).in_xinclude = 0 as ::core::ffi::c_int;
    if (*reader).patternTab.is_null() {
        (*reader).patternNr = 0 as ::core::ffi::c_int;
        (*reader).patternMax = 0 as ::core::ffi::c_int;
    }
    while (*reader).patternNr > 0 as ::core::ffi::c_int {
        (*reader).patternNr -= 1;
        if !(*(*reader).patternTab.offset((*reader).patternNr as isize)).is_null() {
            xmlFreePattern(*(*reader).patternTab.offset((*reader).patternNr as isize));
            let ref mut fresh0 = *(*reader).patternTab.offset((*reader).patternNr as isize);
            *fresh0 = ::core::ptr::null_mut::<xmlPattern>();
        }
    }
    if options & XML_PARSE_DTDVALID as ::core::ffi::c_int != 0 {
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
            xmlStrdup(URL as *const xmlChar) as *mut ::core::ffi::c_char;
    }
    (*reader).doc = ::core::ptr::null_mut::<xmlDoc>();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetMaxAmplification(
    mut reader: xmlTextReaderPtr,
    mut maxAmpl: ::core::ffi::c_uint,
) {
    xmlCtxtSetMaxAmplification((*reader).ctxt, maxAmpl);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderByteConsumed(
    mut reader: xmlTextReaderPtr,
) -> ::core::ffi::c_long {
    if reader.is_null() || (*reader).ctxt.is_null() {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    return xmlByteConsumed((*reader).ctxt);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderWalker(mut doc: xmlDocPtr) -> xmlTextReaderPtr {
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
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlTextReader>() as size_t,
    );
    (*ret).entNr = 0 as ::core::ffi::c_int;
    (*ret).input = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    (*ret).mode = XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int;
    (*ret).node = ::core::ptr::null_mut::<xmlNode>();
    (*ret).curnode = ::core::ptr::null_mut::<xmlNode>();
    (*ret).base = 0 as ::core::ffi::c_uint;
    (*ret).cur = 0 as ::core::ffi::c_uint;
    (*ret).allocs = XML_TEXTREADER_CTXT;
    (*ret).doc = doc;
    (*ret).state = XML_TEXTREADER_START;
    (*ret).dict = xmlDictCreate();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForDoc(
    mut cur: *const xmlChar,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlTextReaderPtr {
    let mut len: ::core::ffi::c_int = 0;
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    len = xmlStrlen(cur);
    return xmlReaderForMemory(
        cur as *const ::core::ffi::c_char,
        len,
        URL,
        encoding,
        options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForFile(
    mut filename: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    reader = xmlNewTextReaderFilename(filename);
    if reader.is_null() {
        return ::core::ptr::null_mut::<xmlTextReader>();
    }
    xmlTextReaderSetup(
        reader,
        ::core::ptr::null_mut::<xmlParserInputBuffer>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        encoding,
        options,
    );
    return reader;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForMemory(
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForFd(
    mut fd: ::core::ffi::c_int,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if fd < 0 as ::core::ffi::c_int {
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
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForIO(
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut ::core::ffi::c_void,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewWalker(
    mut reader: xmlTextReaderPtr,
    mut doc: xmlDocPtr,
) -> ::core::ffi::c_int {
    if doc.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*reader).input.is_null() {
        xmlFreeParserInputBuffer((*reader).input);
    }
    if !(*reader).ctxt.is_null() {
        xmlCtxtReset((*reader).ctxt);
    }
    (*reader).entNr = 0 as ::core::ffi::c_int;
    (*reader).input = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    (*reader).mode = XML_TEXTREADER_MODE_INITIAL as ::core::ffi::c_int;
    (*reader).node = ::core::ptr::null_mut::<xmlNode>();
    (*reader).curnode = ::core::ptr::null_mut::<xmlNode>();
    (*reader).base = 0 as ::core::ffi::c_uint;
    (*reader).cur = 0 as ::core::ffi::c_uint;
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
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewDoc(
    mut reader: xmlTextReaderPtr,
    mut cur: *const xmlChar,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    if cur.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    len = xmlStrlen(cur);
    return xmlReaderNewMemory(
        reader,
        cur as *const ::core::ffi::c_char,
        len,
        URL,
        encoding,
        options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewFile(
    mut reader: xmlTextReaderPtr,
    mut filename: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if filename.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    input = xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return xmlTextReaderSetup(reader, input, filename, encoding, options);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewMemory(
    mut reader: xmlTextReaderPtr,
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if buffer.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    input = xmlParserInputBufferCreateMem(buffer, size, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewFd(
    mut reader: xmlTextReaderPtr,
    mut fd: ::core::ffi::c_int,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if fd < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*input).closecallback = None;
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewIO(
    mut reader: xmlTextReaderPtr,
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut ::core::ffi::c_void,
    mut URL: *const ::core::ffi::c_char,
    mut encoding: *const ::core::ffi::c_char,
    mut options: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if ioread.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if reader.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    input = xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return -(1 as ::core::ffi::c_int);
    }
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
pub const XINCLUDE_NS: *const xmlChar = b"http://www.w3.org/2003/XInclude\0" as *const u8
    as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_OLD_NS: *const xmlChar = b"http://www.w3.org/2001/XInclude\0" as *const u8
    as *const ::core::ffi::c_char as *const xmlChar;
pub const XINCLUDE_NODE: *const xmlChar =
    b"include\0" as *const u8 as *const ::core::ffi::c_char as *const xmlChar;
