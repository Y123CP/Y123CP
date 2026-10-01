#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
use core::ffi::*;
use ::libxml2_cleaned::src::ffi::*;
use ::libxml2_cleaned::src::c_consts::*;
use ::libxml2_cleaned::src::c_structs::*;
use ::libxml2_cleaned::src::c_types::*;
use ::libxml2_cleaned::src::c_extern_types::*;
#[allow(unused_imports)]
use ::libxml2_cleaned;
use ::libxml2_cleaned::src::globals::__xmlGenericError;
use ::libxml2_cleaned::src::globals::__xmlGenericErrorContext;
use ::libxml2_cleaned::src::globals::__xmlTreeIndentString;
use ::libxml2_cleaned::src::HTMLparser::htmlCreatePushParserCtxt;
use ::libxml2_cleaned::src::HTMLparser::htmlCtxtUseOptions;
use ::libxml2_cleaned::src::HTMLparser::htmlFreeParserCtxt;
use ::libxml2_cleaned::src::HTMLparser::htmlParseChunk;
use ::libxml2_cleaned::src::HTMLparser::htmlReadFile;
use ::libxml2_cleaned::src::HTMLparser::htmlReadMemory;
use ::libxml2_cleaned::src::HTMLtree::htmlSaveFileFormat;
use ::libxml2_cleaned::src::encoding::xmlAddEncodingAlias;
use ::libxml2_cleaned::src::c14n::xmlC14NDocDumpMemory;
use ::libxml2_cleaned::src::parserInternals::xmlCheckVersion;
use ::libxml2_cleaned::src::threads::xmlCleanupParser;
use ::libxml2_cleaned::src::parser::xmlCtxtReadIO;
use ::libxml2_cleaned::src::debugXML::xmlDebugDumpDocument;
use ::libxml2_cleaned::src::debugXML::xmlDebugDumpEntities;
use ::libxml2_cleaned::src::tree::xmlDeregisterNodeDefault;
use ::libxml2_cleaned::src::xmlsave::xmlDocDumpFormatMemory;
use ::libxml2_cleaned::src::xmlsave::xmlDocDumpFormatMemoryEnc;
use ::libxml2_cleaned::src::xmlsave::xmlDocDumpMemory;
use ::libxml2_cleaned::src::xmlsave::xmlDocDumpMemoryEnc;
use ::libxml2_cleaned::src::tree::xmlDocSetRootElement;
use ::libxml2_cleaned::src::valid::xmlFreeEnumeration;
use ::libxml2_cleaned::src::pattern::xmlFreePattern;
use ::libxml2_cleaned::src::pattern::xmlFreeStreamCtxt;
use ::libxml2_cleaned::src::xmlreader::xmlFreeTextReader;
use ::libxml2_cleaned::src::valid::xmlFreeValidCtxt;
use ::libxml2_cleaned::src::xmlIO::xmlGetExternalEntityLoader;
use ::libxml2_cleaned::src::parser::xmlHasFeature;
use ::libxml2_cleaned::src::catalog::xmlLoadCatalogs;
use ::libxml2_cleaned::src::xmlmemory::xmlMemFree;
use ::libxml2_cleaned::src::xmlmemory::xmlMemMalloc;
use ::libxml2_cleaned::src::xmlmemory::xmlMemRealloc;
use ::libxml2_cleaned::src::xmlmemory::xmlMemSetup;
use ::libxml2_cleaned::src::xmlmemory::xmlMemSize;
use ::libxml2_cleaned::src::xmlmemory::xmlMemUsed;
use ::libxml2_cleaned::src::xmlmemory::xmlMemoryStrdup;
use ::libxml2_cleaned::src::valid::xmlNewValidCtxt;
use ::libxml2_cleaned::src::xmlsave::xmlNodeDumpOutput;
use ::libxml2_cleaned::src::pattern::xmlPatternGetStreamCtxt;
use ::libxml2_cleaned::src::pattern::xmlPatterncompile;
use ::libxml2_cleaned::src::parser::xmlReadFd;
use ::libxml2_cleaned::src::parser::xmlReadIO;
use ::libxml2_cleaned::src::xmlreader::xmlReaderForFile;
use ::libxml2_cleaned::src::xmlreader::xmlReaderForMemory;
use ::libxml2_cleaned::src::xmlreader::xmlReaderWalker;
use ::libxml2_cleaned::src::tree::xmlRegisterNodeDefault;
use ::libxml2_cleaned::src::relaxng::xmlRelaxNGFree;
use ::libxml2_cleaned::src::relaxng::xmlRelaxNGFreeParserCtxt;
use ::libxml2_cleaned::src::relaxng::xmlRelaxNGFreeValidCtxt;
use ::libxml2_cleaned::src::relaxng::xmlRelaxNGNewParserCtxt;
use ::libxml2_cleaned::src::relaxng::xmlRelaxNGNewValidCtxt;
use ::libxml2_cleaned::src::relaxng::xmlRelaxNGParse;
use ::libxml2_cleaned::src::relaxng::xmlRelaxNGSetParserErrors;
use ::libxml2_cleaned::src::relaxng::xmlRelaxNGSetValidErrors;
use ::libxml2_cleaned::src::xmlsave::xmlSaveClose;
use ::libxml2_cleaned::src::xmlsave::xmlSaveDoc;
use ::libxml2_cleaned::src::xmlsave::xmlSaveFileEnc;
use ::libxml2_cleaned::src::xmlsave::xmlSaveFormatFile;
use ::libxml2_cleaned::src::xmlsave::xmlSaveFormatFileEnc;
use ::libxml2_cleaned::src::xmlsave::xmlSaveToFd;
use ::libxml2_cleaned::src::xmlsave::xmlSaveToFilename;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaFree;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaFreeParserCtxt;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaFreeValidCtxt;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaNewParserCtxt;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaNewValidCtxt;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaParse;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaSetParserErrors;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaSetValidErrors;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaValidateDoc;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaValidateSetFilename;
use ::libxml2_cleaned::src::xmlschemas::xmlSchemaValidateStream;
use ::libxml2_cleaned::src::schematron::xmlSchematronFree;
use ::libxml2_cleaned::src::schematron::xmlSchematronFreeParserCtxt;
use ::libxml2_cleaned::src::schematron::xmlSchematronFreeValidCtxt;
use ::libxml2_cleaned::src::schematron::xmlSchematronNewParserCtxt;
use ::libxml2_cleaned::src::schematron::xmlSchematronNewValidCtxt;
use ::libxml2_cleaned::src::schematron::xmlSchematronParse;
use ::libxml2_cleaned::src::schematron::xmlSchematronValidateDoc;
use ::libxml2_cleaned::src::xmlIO::xmlSetExternalEntityLoader;
use ::libxml2_cleaned::src::debugXML::xmlShell;
use ::libxml2_cleaned::src::xmlstring::xmlStrcat;
use ::libxml2_cleaned::src::xmlstring::xmlStrdup;
use ::libxml2_cleaned::src::pattern::xmlStreamPop;
use ::libxml2_cleaned::src::pattern::xmlStreamPush;
use ::libxml2_cleaned::src::xmlstring::xmlStrndup;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderConstLocalName;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderConstName;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderConstNamespaceUri;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderConstValue;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderCurrentNode;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderDepth;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderHasValue;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderIsEmptyElement;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderIsValid;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderNodeType;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderRead;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderRelaxNGValidate;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderSchemaValidate;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderSetMaxAmplification;
use ::libxml2_cleaned::src::xmlreader::xmlTextReaderSetParserProp;
use ::libxml2_cleaned::src::valid::xmlValidGetValidElements;
use ::libxml2_cleaned::src::xinclude::xmlXIncludeProcessFlags;
use ::libxml2_cleaned::src::xpath::xmlXPathIsInf;
use ::libxml2_cleaned::src::xpath::xmlXPathIsNaN;
use ::libxml2_cleaned::src::xpath::xmlXPathOrderDocElems;
pub use libxml2_cleaned::src::relaxng::_xmlRelaxNG;
pub use libxml2_cleaned::src::relaxng::_xmlRelaxNGParserCtxt;
pub use libxml2_cleaned::src::relaxng::_xmlRelaxNGValidCtxt;
pub use libxml2_cleaned::src::xmlschemas::_xmlSchema;
pub use libxml2_cleaned::src::xmlschemas::_xmlSchemaParserCtxt;
pub use libxml2_cleaned::src::xmlschemas::_xmlSchemaValidCtxt;
pub use libxml2_cleaned::src::xmlreader::_xmlTextReader;
pub use libxml2_cleaned::src::schematron::_xmlSchematron;
pub use libxml2_cleaned::src::schematron::_xmlSchematronParserCtxt;
pub use libxml2_cleaned::src::schematron::_xmlSchematronValidCtxt;
pub use libxml2_cleaned::src::pattern::_xmlPattern;
pub use libxml2_cleaned::src::pattern::_xmlStreamCtxt;
pub use libxml2_cleaned::src::xmlsave::_xmlSaveCtxt;
pub use libxml2_cleaned::src::xpath::_xmlXPathCompExpr;
pub use libxml2_cleaned::src::valid::_xmlValidState;
pub use libxml2_cleaned::src::dict::_xmlDict;
pub use libxml2_cleaned::src::hash::_xmlHashTable;
pub use libxml2_cleaned::src::buf::_xmlBuf;
pub use libxml2_cleaned::src::parser::_xmlAttrHashBucket;
pub use libxml2_cleaned::src::parser::_xmlParserNsData;
pub use libxml2_cleaned::src::parser::_xmlStartTag;
pub use libxml2_cleaned::src::xmlregexp::_xmlAutomataState;
pub use libxml2_cleaned::src::xmlregexp::_xmlAutomata;
extern "C" {
    fn vfprintf(
        __s: *mut FILE,
        __format: *const c_char,
        __arg: ::core::ffi::VaList,
    ) -> c_int;
    fn vsnprintf(
        __s: *mut c_char,
        __maxlen: size_t,
        __format: *const c_char,
        __arg: ::core::ffi::VaList,
    ) -> c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut c_void) -> c_int;
    fn __xstat(
        __ver: c_int,
        __filename: *const c_char,
        __stat_buf: *mut stat,
    ) -> c_int;
    fn write(__fd: c_int, __buf: *const c_void, __n: size_t) -> ssize_t;
    fn mmap(
        __addr: *mut c_void,
        __len: size_t,
        __prot: c_int,
        __flags: c_int,
        __fd: c_int,
        __offset: __off64_t,
    ) -> *mut c_void;
    fn munmap(__addr: *mut c_void, __len: size_t) -> c_int;
    fn xmlParserInputBufferCreateFilename(
        URI: *const c_char,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    fn xmlOutputBufferCreateFile(
        file: *mut FILE,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferWrite(
        out: xmlOutputBufferPtr,
        len: c_int,
        buf: *const c_char,
    ) -> c_int;
    fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> c_int;
    fn xmlNoNetExternalEntityLoader(
        URL: *const c_char,
        ID: *const c_char,
        ctxt: xmlParserCtxtPtr,
    ) -> xmlParserInputPtr;
    fn xmlValidateDtd(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, dtd: xmlDtdPtr) -> c_int;
    fn xmlValidateDocument(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn xmlParseDTD(ExternalID: *const xmlChar, SystemID: *const xmlChar) -> xmlDtdPtr;
    fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
    fn xmlNewSAXParserCtxt(
        sax_0: *const xmlSAXHandler,
        userData: *mut c_void,
    ) -> xmlParserCtxtPtr;
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlCreatePushParserCtxt(
        sax_0: xmlSAXHandlerPtr,
        user_data: *mut c_void,
        chunk: *const c_char,
        size: c_int,
        filename: *const c_char,
    ) -> xmlParserCtxtPtr;
    fn xmlParseChunk(
        ctxt: xmlParserCtxtPtr,
        chunk: *const c_char,
        size: c_int,
        terminate: c_int,
    ) -> c_int;
    fn xmlCtxtUseOptions(
        ctxt: xmlParserCtxtPtr,
        options_0: c_int,
    ) -> c_int;
    fn xmlCtxtSetMaxAmplification(ctxt: xmlParserCtxtPtr, maxAmpl_0: c_uint);
    fn xmlCtxtReadFile(
        ctxt: xmlParserCtxtPtr,
        filename: *const c_char,
        encoding_0: *const c_char,
        options_0: c_int,
    ) -> xmlDocPtr;
    fn xmlCtxtReadMemory(
        ctxt: xmlParserCtxtPtr,
        buffer_0: *const c_char,
        size: c_int,
        URL: *const c_char,
        encoding_0: *const c_char,
        options_0: c_int,
    ) -> xmlDocPtr;
    fn xmlGetIntSubset(doc: *const xmlDoc) -> xmlDtdPtr;
    fn xmlFreeDtd(cur: xmlDtdPtr);
    fn xmlNewDoc(version: *const xmlChar) -> xmlDocPtr;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlCopyDoc(doc: xmlDocPtr, recursive: c_int) -> xmlDocPtr;
    fn xmlNewDocNode(
        doc: xmlDocPtr,
        ns: xmlNsPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> xmlNodePtr;
    fn xmlGetNodePath(node: *const xmlNode) -> *mut xmlChar;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlNodeSetContent(cur: xmlNodePtr, content: *const xmlChar);
    fn xmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> c_int;
    fn xmlSaveFile(filename: *const c_char, cur: xmlDocPtr) -> c_int;
    fn xmlEncodeEntitiesReentrant(doc: xmlDocPtr, input: *const xmlChar) -> *mut xmlChar;
    fn htmlDocDump(f: *mut FILE, cur: xmlDocPtr) -> c_int;
    fn htmlSaveFile(filename: *const c_char, cur: xmlDocPtr) -> c_int;
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    fn xmlXPathNewContext(doc: xmlDocPtr) -> xmlXPathContextPtr;
    fn xmlXPathFreeContext(ctxt: xmlXPathContextPtr);
    fn xmlXPathEval(str: *const xmlChar, ctx: xmlXPathContextPtr) -> xmlXPathObjectPtr;
    fn xmlRelaxNGValidateDoc(ctxt: xmlRelaxNGValidCtxtPtr, doc: xmlDocPtr) -> c_int;
    fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> c_int;
}

pub type __suseconds_t = c_long;

pub type __ssize_t = c_long;

pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type xmlRegisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlDeregisterNodeFunc = Option<unsafe extern "C" fn(xmlNodePtr) -> ()>;

pub type xmlValidCtxtPtr = *mut xmlValidCtxt;

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
pub type xmlExternalEntityLoader = Option<
    unsafe extern "C" fn(
        *const c_char,
        *const c_char,
        xmlParserCtxtPtr,
    ) -> xmlParserInputPtr,
>;
pub type C2RustUnnamed_htdd24ee73 = c_uint;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_htdd24ee73 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_htdd24ee73 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_htdd24ee73 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_htdd24ee73 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_htdd24ee73 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_htdd24ee73 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_htdd24ee73 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_htdd24ee73 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_htdd24ee73 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_htdd24ee73 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_htdd24ee73 = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed_htdd24ee73 = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_htdd24ee73 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_htdd24ee73 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_htdd24ee73 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_htdd24ee73 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_htdd24ee73 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_htdd24ee73 = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_htdd24ee73 = 1;

pub type htmlParserCtxtPtr = xmlParserCtxtPtr;
pub type htmlSAXHandlerPtr = xmlSAXHandlerPtr;
pub type htmlDocPtr = xmlDocPtr;
pub const HTML_PARSE_IGNORE_ENC: C2RustUnnamed_htdd24ee73 = 2097152;
pub const HTML_PARSE_COMPACT: C2RustUnnamed_htdd24ee73 = 65536;
pub const HTML_PARSE_NOIMPLIED: C2RustUnnamed_htdd24ee73 = 8192;
pub const HTML_PARSE_NONET: C2RustUnnamed_htdd24ee73 = 2048;
pub const HTML_PARSE_NOBLANKS: C2RustUnnamed_htdd24ee73 = 256;
pub const HTML_PARSE_PEDANTIC: C2RustUnnamed_htdd24ee73 = 128;
pub const HTML_PARSE_NOWARNING: C2RustUnnamed_htdd24ee73 = 64;
pub const HTML_PARSE_NOERROR: C2RustUnnamed_htdd24ee73 = 32;
pub const HTML_PARSE_NODEFDTD: C2RustUnnamed_htdd24ee73 = 4;
pub const HTML_PARSE_RECOVER: C2RustUnnamed_htdd24ee73 = 1;
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

pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;

pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;

pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
pub const XML_PARSER_SUBST_ENTITIES: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_PARSER_VALIDATE: C2RustUnnamed_htdd24ee73 = 3;
pub const XML_PARSER_DEFAULTATTRS: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_PARSER_LOADDTD: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed_htdd24ee73 = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed_htdd24ee73 = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed_htdd24ee73 = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed_htdd24ee73 = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed_htdd24ee73 = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed_htdd24ee73 = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed_htdd24ee73 = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed_htdd24ee73 = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed_htdd24ee73 = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed_htdd24ee73 = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed_htdd24ee73 = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed_htdd24ee73 = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed_htdd24ee73 = 0;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;

pub type xmlSchematron = _xmlSchematron;
pub type xmlSchematronPtr = *mut xmlSchematron;
pub type xmlSchematronParserCtxt = _xmlSchematronParserCtxt;
pub type xmlSchematronParserCtxtPtr = *mut xmlSchematronParserCtxt;
pub type xmlSchematronValidCtxt = _xmlSchematronValidCtxt;
pub type xmlSchematronValidCtxtPtr = *mut xmlSchematronValidCtxt;
pub type xmlPattern = _xmlPattern;
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlStreamCtxt = _xmlStreamCtxt;
pub type xmlStreamCtxtPtr = *mut xmlStreamCtxt;

pub const XML_C14N_1_1: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_C14N_EXCLUSIVE_1_0: C2RustUnnamed_htdd24ee73 = 1;
pub const XML_C14N_1_0: C2RustUnnamed_htdd24ee73 = 0;
pub const XML_SAVE_WSNONSIG: C2RustUnnamed_htdd24ee73 = 128;
pub const XML_SAVE_AS_HTML: C2RustUnnamed_htdd24ee73 = 64;
pub const XML_SAVE_AS_XML: C2RustUnnamed_htdd24ee73 = 32;
pub const XML_SAVE_XHTML: C2RustUnnamed_htdd24ee73 = 16;
pub const XML_SAVE_NO_XHTML: C2RustUnnamed_htdd24ee73 = 8;
pub const XML_SAVE_NO_EMPTY: C2RustUnnamed_htdd24ee73 = 4;
pub const XML_SAVE_NO_DECL: C2RustUnnamed_htdd24ee73 = 2;
pub const XML_SAVE_FORMAT: C2RustUnnamed_htdd24ee73 = 1;
pub type xmlSaveCtxt = _xmlSaveCtxt;
pub type xmlSaveCtxtPtr = *mut xmlSaveCtxt;
pub type xmllintReturnCode = c_uint;
pub const XMLLINT_ERR_XPATH: xmllintReturnCode = 10;
pub const XMLLINT_ERR_MEM: xmllintReturnCode = 9;
pub const XMLLINT_ERR_RDREGIS: xmllintReturnCode = 8;
pub const XMLLINT_ERR_SCHEMAPAT: xmllintReturnCode = 7;
pub const XMLLINT_ERR_OUT: xmllintReturnCode = 6;
pub const XMLLINT_ERR_SCHEMACOMP: xmllintReturnCode = 5;
pub const XMLLINT_ERR_RDFILE: xmllintReturnCode = 4;
pub const XMLLINT_ERR_VALID: xmllintReturnCode = 3;
pub const XMLLINT_ERR_DTD: xmllintReturnCode = 2;
pub const XMLLINT_ERR_UNCLASS: xmllintReturnCode = 1;
pub const XMLLINT_RETURN_OK: xmllintReturnCode = 0;

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const c_char) -> c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut c_char,
        10 as c_int,
    ) as c_int;
}

pub const UINT_MAX: c_uint = (__INT_MAX__ as c_uint)
    .wrapping_mul(2 as c_uint)
    .wrapping_add(1 as c_uint);

pub const PROT_READ: c_int = 0x1 as c_int;
pub const MAP_SHARED: c_int = 0x1 as c_int;

#[inline]
unsafe extern "C" fn stat(
    mut __path: *const c_char,
    mut __statbuf: *mut stat,
) -> c_int {
    return __xstat(_STAT_VER, __path, __statbuf);
}
pub const MAP_FAILED: *mut c_void =
    -(1 as c_int) as *mut c_void;

pub const XML_XML_DEFAULT_CATALOG: [c_char; 34] = unsafe {
    ::core::mem::transmute::<[u8; 34], [c_char; 34]>(
        *b"file:///usr/local/etc/xml/catalog\0",
    )
};
static mut shell: c_int = 0 as c_int;
static mut debugent: c_int = 0 as c_int;
static mut debug: c_int = 0 as c_int;
static mut maxmem: c_int = 0 as c_int;
static mut copy: c_int = 0 as c_int;
static mut recovery: c_int = 0 as c_int;
static mut noent: c_int = 0 as c_int;
static mut noenc: c_int = 0 as c_int;
static mut noblanks: c_int = 0 as c_int;
static mut noout: c_int = 0 as c_int;
static mut nowrap: c_int = 0 as c_int;
static mut format: c_int = 0 as c_int;
static mut output: *const c_char = ::core::ptr::null::<c_char>();
static mut compress: c_int = 0 as c_int;
static mut oldout: c_int = 0 as c_int;
static mut valid: c_int = 0 as c_int;
static mut postvalid: c_int = 0 as c_int;
static mut dtdvalid: *mut c_char =
    ::core::ptr::null::<c_char>() as *mut c_char;
static mut dtdvalidfpi: *mut c_char =
    ::core::ptr::null::<c_char>() as *mut c_char;
static mut relaxng: *mut c_char =
    ::core::ptr::null::<c_char>() as *mut c_char;
static mut relaxngschemas: xmlRelaxNGPtr = ::core::ptr::null::<xmlRelaxNG>() as *mut xmlRelaxNG;
static mut schema: *mut c_char =
    ::core::ptr::null::<c_char>() as *mut c_char;
static mut wxschemas: xmlSchemaPtr = ::core::ptr::null::<xmlSchema>() as *mut xmlSchema;
static mut schematron: *mut c_char =
    ::core::ptr::null::<c_char>() as *mut c_char;
static mut wxschematron: xmlSchematronPtr =
    ::core::ptr::null::<xmlSchematron>() as *mut xmlSchematron;
static mut repeat: c_int = 0 as c_int;
static mut insert: c_int = 0 as c_int;
static mut html: c_int = 0 as c_int;
static mut xmlout: c_int = 0 as c_int;
static mut htmlout: c_int = 0 as c_int;
static mut nodefdtd: c_int = 0 as c_int;
static mut push: c_int = 0 as c_int;
static mut pushsize: c_int = 4096 as c_int;
static mut memory: c_int = 0 as c_int;
static mut testIO: c_int = 0 as c_int;
static mut encoding: *mut c_char =
    ::core::ptr::null::<c_char>() as *mut c_char;
static mut xinclude: c_int = 0 as c_int;
static mut dtdattrs: c_int = 0 as c_int;
static mut loaddtd: c_int = 0 as c_int;
static mut progresult: xmllintReturnCode = XMLLINT_RETURN_OK;
static mut quiet: c_int = 0 as c_int;
static mut timing: c_int = 0 as c_int;
static mut generate: c_int = 0 as c_int;
static mut dropdtd: c_int = 0 as c_int;
static mut catalogs: c_int = 0 as c_int;
static mut nocatalogs: c_int = 0 as c_int;
static mut canonical: c_int = 0 as c_int;
static mut canonical_11: c_int = 0 as c_int;
static mut exc_canonical: c_int = 0 as c_int;
static mut stream: c_int = 0 as c_int;
static mut walker: c_int = 0 as c_int;
static mut pattern: *const c_char = ::core::ptr::null::<c_char>();
static mut patternc: xmlPatternPtr = ::core::ptr::null::<xmlPattern>() as *mut xmlPattern;
static mut patstream: xmlStreamCtxtPtr = ::core::ptr::null::<xmlStreamCtxt>() as *mut xmlStreamCtxt;
static mut chkregister: c_int = 0 as c_int;
static mut nbregister: c_int = 0 as c_int;
static mut sax1: c_int = 0 as c_int;
static mut xpathquery: *const c_char = ::core::ptr::null::<c_char>();
static mut options: c_int =
    XML_PARSE_COMPACT as c_int | XML_PARSE_BIG_LINES as c_int;
static mut sax: c_int = 0 as c_int;
static mut oldxml10: c_int = 0 as c_int;
static mut maxAmpl: c_uint = 0 as c_uint;
pub const MAX_PATHS: c_int = 64 as c_int;

static mut paths: [*mut xmlChar; 65] = [::core::ptr::null::<xmlChar>() as *mut xmlChar; 65];
static mut nbpaths: c_int = 0 as c_int;
static mut load_trace: c_int = 0 as c_int;
unsafe extern "C" fn parsePath(mut path: *const xmlChar) {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    if path.is_null() {
        return;
    }
    while *path as c_int != 0 as c_int {
        if nbpaths >= MAX_PATHS {
            fprintf(
                stderr,
                b"MAX_PATHS reached: too many paths\n\0" as *const u8 as *const c_char,
            );
            return;
        }
        cur = path;
        while *cur as c_int == ' ' as i32
            || *cur as c_int == PATH_SEPARATOR
        {
            cur = cur.offset(1);
        }
        path = cur;
        while *cur as c_int != 0 as c_int
            && *cur as c_int != ' ' as i32
            && *cur as c_int != PATH_SEPARATOR
        {
            cur = cur.offset(1);
        }
        if cur != path {
            paths[nbpaths as usize] = xmlStrndup(
                path,
                cur.offset_from(path) as c_long as c_int,
            );
            if !paths[nbpaths as usize].is_null() {
                nbpaths += 1;
            }
            path = cur;
        }
    }
}
static mut defaultEntityLoader: xmlExternalEntityLoader = None;
unsafe extern "C" fn xmllintExternalEntityLoader(
    mut URL: *const c_char,
    mut ID: *const c_char,
    mut ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    let mut ret: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut warning: warningSAXFunc = None;
    let mut err: errorSAXFunc = None;
    let mut i: c_int = 0;
    let mut lastsegment: *const c_char = URL;
    let mut iter: *const c_char = URL;
    if nbpaths > 0 as c_int && !iter.is_null() {
        while *iter as c_int != 0 as c_int {
            if *iter as c_int == '/' as i32 {
                lastsegment = iter.offset(1 as c_int as isize);
            }
            iter = iter.offset(1);
        }
    }
    if !ctxt.is_null() && !(*ctxt).sax.is_null() {
        warning = (*(*ctxt).sax).warning;
        err = (*(*ctxt).sax).error;
        (*(*ctxt).sax).warning = None;
        (*(*ctxt).sax).error = None;
    }
    if defaultEntityLoader.is_some() {
        ret = defaultEntityLoader.expect("non-null function pointer")(URL, ID, ctxt);
        if !ret.is_null() {
            if warning.is_some() {
                (*(*ctxt).sax).warning = warning;
            }
            if err.is_some() {
                (*(*ctxt).sax).error = err;
            }
            if load_trace != 0 {
                fprintf(
                    stderr,
                    b"Loaded URL=\"%s\" ID=\"%s\"\n\0" as *const u8 as *const c_char,
                    if !URL.is_null() {
                        URL
                    } else {
                        b"(null)\0" as *const u8 as *const c_char
                    },
                    if !ID.is_null() {
                        ID
                    } else {
                        b"(null)\0" as *const u8 as *const c_char
                    },
                );
            }
            return ret;
        }
    }
    i = 0 as c_int;
    while i < nbpaths {
        let mut newURL: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        newURL = xmlStrdup(paths[i as usize] as *const xmlChar);
        newURL = xmlStrcat(
            newURL,
            b"/\0" as *const u8 as *const c_char as *const xmlChar,
        );
        newURL = xmlStrcat(newURL, lastsegment as *const xmlChar);
        if !newURL.is_null() {
            ret = defaultEntityLoader.expect("non-null function pointer")(
                newURL as *const c_char,
                ID,
                ctxt,
            );
            if !ret.is_null() {
                if warning.is_some() {
                    (*(*ctxt).sax).warning = warning;
                }
                if err.is_some() {
                    (*(*ctxt).sax).error = err;
                }
                if load_trace != 0 {
                    fprintf(
                        stderr,
                        b"Loaded URL=\"%s\" ID=\"%s\"\n\0" as *const u8
                            as *const c_char,
                        newURL,
                        if !ID.is_null() {
                            ID
                        } else {
                            b"(null)\0" as *const u8 as *const c_char
                        },
                    );
                }
                xmlFree.expect("non-null function pointer")(newURL as *mut c_void);
                return ret;
            }
            xmlFree.expect("non-null function pointer")(newURL as *mut c_void);
        }
        i += 1;
    }
    if err.is_some() {
        (*(*ctxt).sax).error = err;
    }
    if warning.is_some() {
        (*(*ctxt).sax).warning = warning;
        if !URL.is_null() {
            warning.expect("non-null function pointer")(
                ctxt as *mut c_void,
                b"failed to load external entity \"%s\"\n\0" as *const u8
                    as *const c_char,
                URL,
            );
        } else if !ID.is_null() {
            warning.expect("non-null function pointer")(
                ctxt as *mut c_void,
                b"failed to load external entity \"%s\"\n\0" as *const u8
                    as *const c_char,
                ID,
            );
        }
    }
    return ::core::ptr::null_mut::<xmlParserInput>();
}
extern "C" fn OOM() { unsafe {
    fprintf(
        stderr,
        b"Ran out of memory needs > %d bytes\n\0" as *const u8 as *const c_char,
        maxmem,
    );
    progresult = XMLLINT_ERR_MEM;
} }
unsafe extern "C" fn myFreeFunc(mut mem: *mut c_void) {
    xmlMemFree(mem);
}
extern "C" fn myMallocFunc(mut size: size_t) -> *mut c_void { unsafe {
    let mut ret: *mut c_void = ::core::ptr::null_mut::<c_void>();
    ret = xmlMemMalloc(size);
    if !ret.is_null() {
        if xmlMemUsed() > maxmem {
            OOM();
            xmlMemFree(ret);
            return ::core::ptr::null_mut::<c_void>();
        }
    }
    return ret;
} }
unsafe extern "C" fn myReallocFunc(
    mut mem: *mut c_void,
    mut size: size_t,
) -> *mut c_void {
    let mut oldsize: size_t = xmlMemSize(mem);
    if (xmlMemUsed() as size_t)
        .wrapping_add(size)
        .wrapping_sub(oldsize)
        > maxmem as size_t
    {
        OOM();
        return ::core::ptr::null_mut::<c_void>();
    }
    return xmlMemRealloc(mem, size);
}
unsafe extern "C" fn myStrdupFunc(mut str: *const c_char) -> *mut c_char {
    let mut ret: *mut c_char = ::core::ptr::null_mut::<c_char>();
    ret = xmlMemoryStrdup(str);
    if !ret.is_null() {
        if xmlMemUsed() > maxmem {
            OOM();
            xmlFree.expect("non-null function pointer")(ret as *mut c_void);
            return ::core::ptr::null_mut::<c_char>();
        }
    }
    return ret;
}
static mut end: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};
static mut begin: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};
extern "C" fn startTimer() { unsafe {
    gettimeofday(&raw mut begin, NULL_0);
} }
unsafe extern "C" fn endTimer(mut fmt: *const c_char, mut args: ...) {
    let mut msec: c_long = 0;
    let mut ap: ::core::ffi::VaListImpl;
    gettimeofday(&raw mut end, NULL_0);
    msec = (end.tv_sec - begin.tv_sec) as c_long;
    msec *= 1000 as c_long;
    msec += (end.tv_usec as c_long - begin.tv_usec as c_long)
        / 1000 as c_long;
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(
        stderr,
        b" took %ld ms\n\0" as *const u8 as *const c_char,
        msec,
    );
}
static mut buffer: [c_char; 50000] = [0; 50000];
extern "C" fn xmlHTMLEncodeSend() { unsafe {
    let mut result: *mut c_char = ::core::ptr::null_mut::<c_char>();
    memset(
        (&raw mut buffer as *mut c_char).offset(
            (::core::mem::size_of::<[c_char; 50000]>() as usize)
                .wrapping_sub(4 as usize) as isize,
        ) as *mut c_char as *mut c_void,
        0 as c_int,
        4 as size_t,
    );
    result = xmlEncodeEntitiesReentrant(
        ::core::ptr::null_mut::<xmlDoc>(),
        &raw mut buffer as *mut c_char as *mut xmlChar,
    ) as *mut c_char;
    if !result.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"%s\0" as *const u8 as *const c_char,
            result,
        );
        xmlFree.expect("non-null function pointer")(result as *mut c_void);
    }
    buffer[0 as c_int as usize] = 0 as c_char;
} }
extern "C" fn xmlHTMLPrintFileInfo(mut input: xmlParserInputPtr) { unsafe {
    let mut len: c_int = 0;
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"<p>\0" as *const u8 as *const c_char,
    );
    len = strlen(&raw mut buffer as *mut c_char) as c_int;
    if !input.is_null() {
        if !(*input).filename.is_null() {
            snprintf(
                (&raw mut buffer as *mut c_char).offset(len as isize)
                    as *mut c_char,
                (::core::mem::size_of::<[c_char; 50000]>() as size_t)
                    .wrapping_sub(len as size_t),
                b"%s:%d: \0" as *const u8 as *const c_char,
                (*input).filename,
                (*input).line,
            );
        } else {
            snprintf(
                (&raw mut buffer as *mut c_char).offset(len as isize)
                    as *mut c_char,
                (::core::mem::size_of::<[c_char; 50000]>() as size_t)
                    .wrapping_sub(len as size_t),
                b"Entity: line %d: \0" as *const u8 as *const c_char,
                (*input).line,
            );
        }
    }
    xmlHTMLEncodeSend();
} }
extern "C" fn xmlHTMLPrintFileContext(mut input: xmlParserInputPtr) { unsafe {
    let mut cur: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut base: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut len: c_int = 0;
    let mut n: c_int = 0;
    if input.is_null() {
        return;
    }
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"<pre>\n\0" as *const u8 as *const c_char,
    );
    cur = (*input).cur;
    base = (*input).base;
    while cur > base
        && (*cur as c_int == '\n' as i32 || *cur as c_int == '\r' as i32)
    {
        cur = cur.offset(-1);
    }
    n = 0 as c_int;
    loop {
        let fresh0 = n;
        n = n + 1;
        if !(fresh0 < 80 as c_int
            && cur > base
            && *cur as c_int != '\n' as i32
            && *cur as c_int != '\r' as i32)
        {
            break;
        }
        cur = cur.offset(-1);
    }
    if *cur as c_int == '\n' as i32 || *cur as c_int == '\r' as i32 {
        cur = cur.offset(1);
    }
    base = cur;
    n = 0 as c_int;
    while *cur as c_int != 0 as c_int
        && *cur as c_int != '\n' as i32
        && *cur as c_int != '\r' as i32
        && n < 79 as c_int
    {
        len = strlen(&raw mut buffer as *mut c_char) as c_int;
        let fresh1 = cur;
        cur = cur.offset(1);
        snprintf(
            (&raw mut buffer as *mut c_char).offset(len as isize)
                as *mut c_char,
            (::core::mem::size_of::<[c_char; 50000]>() as size_t)
                .wrapping_sub(len as size_t),
            b"%c\0" as *const u8 as *const c_char,
            *fresh1 as c_int,
        );
        n += 1;
    }
    len = strlen(&raw mut buffer as *mut c_char) as c_int;
    snprintf(
        (&raw mut buffer as *mut c_char).offset(len as isize)
            as *mut c_char,
        (::core::mem::size_of::<[c_char; 50000]>() as size_t)
            .wrapping_sub(len as size_t),
        b"\n\0" as *const u8 as *const c_char,
    );
    cur = (*input).cur;
    while *cur as c_int == '\n' as i32 || *cur as c_int == '\r' as i32 {
        cur = cur.offset(-1);
    }
    n = 0 as c_int;
    while cur != base && {
        let fresh2 = n;
        n = n + 1;
        fresh2 < 80 as c_int
    } {
        len = strlen(&raw mut buffer as *mut c_char) as c_int;
        snprintf(
            (&raw mut buffer as *mut c_char).offset(len as isize)
                as *mut c_char,
            (::core::mem::size_of::<[c_char; 50000]>() as size_t)
                .wrapping_sub(len as size_t),
            b" \0" as *const u8 as *const c_char,
        );
        base = base.offset(1);
    }
    len = strlen(&raw mut buffer as *mut c_char) as c_int;
    snprintf(
        (&raw mut buffer as *mut c_char).offset(len as isize)
            as *mut c_char,
        (::core::mem::size_of::<[c_char; 50000]>() as size_t)
            .wrapping_sub(len as size_t),
        b"^\n\0" as *const u8 as *const c_char,
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"</pre>\0" as *const u8 as *const c_char,
    );
} }
unsafe extern "C" fn xmlHTMLError(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: c_int = 0;
    buffer[0 as c_int as usize] = 0 as c_char;
    input = (*ctxt).input;
    if !input.is_null() && (*input).filename.is_null() && (*ctxt).inputNr > 1 as c_int
    {
        input = *(*ctxt)
            .inputTab
            .offset(((*ctxt).inputNr - 2 as c_int) as isize);
    }
    xmlHTMLPrintFileInfo(input);
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"<b>error</b>: \0" as *const u8 as *const c_char,
    );
    args_0 = args.clone();
    len = strlen(&raw mut buffer as *mut c_char) as c_int;
    vsnprintf(
        (&raw mut buffer as *mut c_char).offset(len as isize)
            as *mut c_char,
        (::core::mem::size_of::<[c_char; 50000]>() as size_t)
            .wrapping_sub(len as size_t),
        msg,
        args_0.as_va_list(),
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"</p>\n\0" as *const u8 as *const c_char,
    );
    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}
unsafe extern "C" fn xmlHTMLWarning(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: c_int = 0;
    buffer[0 as c_int as usize] = 0 as c_char;
    input = (*ctxt).input;
    if !input.is_null() && (*input).filename.is_null() && (*ctxt).inputNr > 1 as c_int
    {
        input = *(*ctxt)
            .inputTab
            .offset(((*ctxt).inputNr - 2 as c_int) as isize);
    }
    xmlHTMLPrintFileInfo(input);
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"<b>warning</b>: \0" as *const u8 as *const c_char,
    );
    args_0 = args.clone();
    len = strlen(&raw mut buffer as *mut c_char) as c_int;
    vsnprintf(
        (&raw mut buffer as *mut c_char).offset(len as isize)
            as *mut c_char,
        (::core::mem::size_of::<[c_char; 50000]>() as size_t)
            .wrapping_sub(len as size_t),
        msg,
        args_0.as_va_list(),
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"</p>\n\0" as *const u8 as *const c_char,
    );
    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}
unsafe extern "C" fn xmlHTMLValidityError(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: c_int = 0;
    buffer[0 as c_int as usize] = 0 as c_char;
    input = (*ctxt).input;
    if (*input).filename.is_null() && (*ctxt).inputNr > 1 as c_int {
        input = *(*ctxt)
            .inputTab
            .offset(((*ctxt).inputNr - 2 as c_int) as isize);
    }
    xmlHTMLPrintFileInfo(input);
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"<b>validity error</b>: \0" as *const u8 as *const c_char,
    );
    len = strlen(&raw mut buffer as *mut c_char) as c_int;
    args_0 = args.clone();
    vsnprintf(
        (&raw mut buffer as *mut c_char).offset(len as isize)
            as *mut c_char,
        (::core::mem::size_of::<[c_char; 50000]>() as size_t)
            .wrapping_sub(len as size_t),
        msg,
        args_0.as_va_list(),
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"</p>\n\0" as *const u8 as *const c_char,
    );
    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
    progresult = XMLLINT_ERR_VALID;
}
unsafe extern "C" fn xmlHTMLValidityWarning(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut input: xmlParserInputPtr = ::core::ptr::null_mut::<xmlParserInput>();
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: c_int = 0;
    buffer[0 as c_int as usize] = 0 as c_char;
    input = (*ctxt).input;
    if (*input).filename.is_null() && (*ctxt).inputNr > 1 as c_int {
        input = *(*ctxt)
            .inputTab
            .offset(((*ctxt).inputNr - 2 as c_int) as isize);
    }
    xmlHTMLPrintFileInfo(input);
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"<b>validity warning</b>: \0" as *const u8 as *const c_char,
    );
    args_0 = args.clone();
    len = strlen(&raw mut buffer as *mut c_char) as c_int;
    vsnprintf(
        (&raw mut buffer as *mut c_char).offset(len as isize)
            as *mut c_char,
        (::core::mem::size_of::<[c_char; 50000]>() as size_t)
            .wrapping_sub(len as size_t),
        msg,
        args_0.as_va_list(),
    );
    xmlHTMLEncodeSend();
    (*__xmlGenericError()).expect("non-null function pointer")(
        *__xmlGenericErrorContext(),
        b"</p>\n\0" as *const u8 as *const c_char,
    );
    xmlHTMLPrintFileContext(input);
    xmlHTMLEncodeSend();
}
unsafe extern "C" fn xmlShellReadline(
    mut prompt: *mut c_char,
) -> *mut c_char {
    let mut line_read: [c_char; 501] = [0; 501];
    let mut ret: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut len: c_int = 0;
    if !prompt.is_null() {
        fprintf(
            stdout,
            b"%s\0" as *const u8 as *const c_char,
            prompt,
        );
    }
    fflush(stdout);
    if fgets(
        &raw mut line_read as *mut c_char,
        500 as c_int,
        stdin,
    )
    .is_null()
    {
        return ::core::ptr::null_mut::<c_char>();
    }
    line_read[500 as c_int as usize] = 0 as c_char;
    len = strlen(&raw mut line_read as *mut c_char) as c_int;
    ret = malloc((len + 1 as c_int) as size_t) as *mut c_char;
    if !ret.is_null() {
        memcpy(
            ret as *mut c_void,
            &raw mut line_read as *mut c_char as *const c_void,
            (len + 1 as c_int) as size_t,
        );
    }
    return ret;
}
unsafe extern "C" fn myRead(
    mut f: *mut c_void,
    mut buf: *mut c_char,
    mut len: c_int,
) -> c_int {
    return fread(
        buf as *mut c_void,
        1 as size_t,
        len as size_t,
        f as *mut FILE,
    ) as c_int;
}
unsafe extern "C" fn myClose(mut context: *mut c_void) -> c_int {
    let mut f: *mut FILE = context as *mut FILE;
    if f == stdin {
        return 0 as c_int;
    }
    return fclose(f);
}
static mut emptySAXHandlerStruct: xmlSAXHandler = _xmlSAXHandler {
    internalSubset: None,
    isStandalone: None,
    hasInternalSubset: None,
    hasExternalSubset: None,
    resolveEntity: None,
    getEntity: None,
    entityDecl: None,
    notationDecl: None,
    attributeDecl: None,
    elementDecl: None,
    unparsedEntityDecl: None,
    setDocumentLocator: None,
    startDocument: None,
    endDocument: None,
    startElement: None,
    endElement: None,
    reference: None,
    characters: None,
    ignorableWhitespace: None,
    processingInstruction: None,
    comment: None,
    warning: None,
    error: None,
    fatalError: None,
    getParameterEntity: None,
    cdataBlock: None,
    externalSubset: None,
    initialized: XML_SAX2_MAGIC,
    _private: NULL_0,
    startElementNs: None,
    endElementNs: None,
    serror: None,
};
static mut emptySAXHandler: xmlSAXHandlerPtr =
    unsafe { &raw const emptySAXHandlerStruct as xmlSAXHandlerPtr };
static mut callbacks: c_int = 0;
unsafe extern "C" fn isStandaloneDebug(mut ctx: *mut c_void) -> c_int {
    callbacks += 1;
    if noout != 0 {
        return 0 as c_int;
    }
    fprintf(
        stdout,
        b"SAX.isStandalone()\n\0" as *const u8 as *const c_char,
    );
    return 0 as c_int;
}
unsafe extern "C" fn hasInternalSubsetDebug(
    mut ctx: *mut c_void,
) -> c_int {
    callbacks += 1;
    if noout != 0 {
        return 0 as c_int;
    }
    fprintf(
        stdout,
        b"SAX.hasInternalSubset()\n\0" as *const u8 as *const c_char,
    );
    return 0 as c_int;
}
unsafe extern "C" fn hasExternalSubsetDebug(
    mut ctx: *mut c_void,
) -> c_int {
    callbacks += 1;
    if noout != 0 {
        return 0 as c_int;
    }
    fprintf(
        stdout,
        b"SAX.hasExternalSubset()\n\0" as *const u8 as *const c_char,
    );
    return 0 as c_int;
}
unsafe extern "C" fn internalSubsetDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.internalSubset(%s,\0" as *const u8 as *const c_char,
        name,
    );
    if ExternalID.is_null() {
        fprintf(stdout, b" ,\0" as *const u8 as *const c_char);
    } else {
        fprintf(
            stdout,
            b" %s,\0" as *const u8 as *const c_char,
            ExternalID,
        );
    }
    if SystemID.is_null() {
        fprintf(stdout, b" )\n\0" as *const u8 as *const c_char);
    } else {
        fprintf(
            stdout,
            b" %s)\n\0" as *const u8 as *const c_char,
            SystemID,
        );
    };
}
unsafe extern "C" fn externalSubsetDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.externalSubset(%s,\0" as *const u8 as *const c_char,
        name,
    );
    if ExternalID.is_null() {
        fprintf(stdout, b" ,\0" as *const u8 as *const c_char);
    } else {
        fprintf(
            stdout,
            b" %s,\0" as *const u8 as *const c_char,
            ExternalID,
        );
    }
    if SystemID.is_null() {
        fprintf(stdout, b" )\n\0" as *const u8 as *const c_char);
    } else {
        fprintf(
            stdout,
            b" %s)\n\0" as *const u8 as *const c_char,
            SystemID,
        );
    };
}
unsafe extern "C" fn resolveEntityDebug(
    mut ctx: *mut c_void,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) -> xmlParserInputPtr {
    callbacks += 1;
    if noout != 0 {
        return ::core::ptr::null_mut::<xmlParserInput>();
    }
    fprintf(
        stdout,
        b"SAX.resolveEntity(\0" as *const u8 as *const c_char,
    );
    if !publicId.is_null() {
        fprintf(
            stdout,
            b"%s\0" as *const u8 as *const c_char,
            publicId as *mut c_char,
        );
    } else {
        fprintf(stdout, b" \0" as *const u8 as *const c_char);
    }
    if !systemId.is_null() {
        fprintf(
            stdout,
            b", %s)\n\0" as *const u8 as *const c_char,
            systemId as *mut c_char,
        );
    } else {
        fprintf(
            stdout,
            b", )\n\0" as *const u8 as *const c_char,
        );
    }
    return ::core::ptr::null_mut::<xmlParserInput>();
}
unsafe extern "C" fn getEntityDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    callbacks += 1;
    if noout != 0 {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    fprintf(
        stdout,
        b"SAX.getEntity(%s)\n\0" as *const u8 as *const c_char,
        name,
    );
    return ::core::ptr::null_mut::<xmlEntity>();
}
unsafe extern "C" fn getParameterEntityDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    callbacks += 1;
    if noout != 0 {
        return ::core::ptr::null_mut::<xmlEntity>();
    }
    fprintf(
        stdout,
        b"SAX.getParameterEntity(%s)\n\0" as *const u8 as *const c_char,
        name,
    );
    return ::core::ptr::null_mut::<xmlEntity>();
}
unsafe extern "C" fn entityDeclDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut type_0: c_int,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut content: *mut xmlChar,
) {
    let mut nullstr: *const xmlChar =
        b"(null)\0" as *const u8 as *const c_char as *mut xmlChar;
    if publicId.is_null() {
        publicId = nullstr;
    }
    if systemId.is_null() {
        systemId = nullstr;
    }
    if content.is_null() {
        content = nullstr as *mut xmlChar;
    }
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.entityDecl(%s, %d, %s, %s, %s)\n\0" as *const u8 as *const c_char,
        name,
        type_0,
        publicId,
        systemId,
        content,
    );
}
unsafe extern "C" fn attributeDeclDebug(
    mut ctx: *mut c_void,
    mut elem: *const xmlChar,
    mut name: *const xmlChar,
    mut type_0: c_int,
    mut def: c_int,
    mut defaultValue: *const xmlChar,
    mut tree: xmlEnumerationPtr,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    if defaultValue.is_null() {
        fprintf(
            stdout,
            b"SAX.attributeDecl(%s, %s, %d, %d, NULL, ...)\n\0" as *const u8
                as *const c_char,
            elem,
            name,
            type_0,
            def,
        );
    } else {
        fprintf(
            stdout,
            b"SAX.attributeDecl(%s, %s, %d, %d, %s, ...)\n\0" as *const u8
                as *const c_char,
            elem,
            name,
            type_0,
            def,
            defaultValue,
        );
    }
    xmlFreeEnumeration(tree);
}
unsafe extern "C" fn elementDeclDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut type_0: c_int,
    mut content: xmlElementContentPtr,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.elementDecl(%s, %d, ...)\n\0" as *const u8 as *const c_char,
        name,
        type_0,
    );
}
unsafe extern "C" fn notationDeclDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.notationDecl(%s, %s, %s)\n\0" as *const u8 as *const c_char,
        name as *mut c_char,
        publicId as *mut c_char,
        systemId as *mut c_char,
    );
}
unsafe extern "C" fn unparsedEntityDeclDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut notationName: *const xmlChar,
) {
    let mut nullstr: *const xmlChar =
        b"(null)\0" as *const u8 as *const c_char as *mut xmlChar;
    if publicId.is_null() {
        publicId = nullstr;
    }
    if systemId.is_null() {
        systemId = nullstr;
    }
    if notationName.is_null() {
        notationName = nullstr;
    }
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.unparsedEntityDecl(%s, %s, %s, %s)\n\0" as *const u8 as *const c_char,
        name as *mut c_char,
        publicId as *mut c_char,
        systemId as *mut c_char,
        notationName as *mut c_char,
    );
}
unsafe extern "C" fn setDocumentLocatorDebug(
    mut ctx: *mut c_void,
    mut loc: xmlSAXLocatorPtr,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.setDocumentLocator()\n\0" as *const u8 as *const c_char,
    );
}
unsafe extern "C" fn startDocumentDebug(mut ctx: *mut c_void) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.startDocument()\n\0" as *const u8 as *const c_char,
    );
}
unsafe extern "C" fn endDocumentDebug(mut ctx: *mut c_void) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.endDocument()\n\0" as *const u8 as *const c_char,
    );
}
unsafe extern "C" fn startElementDebug(
    mut ctx: *mut c_void,
    mut name: *const xmlChar,
    mut atts: *mut *const xmlChar,
) {
    let mut i: c_int = 0;
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.startElement(%s\0" as *const u8 as *const c_char,
        name as *mut c_char,
    );
    if !atts.is_null() {
        i = 0 as c_int;
        while !(*atts.offset(i as isize)).is_null() {
            let fresh3 = i;
            i = i + 1;
            fprintf(
                stdout,
                b", %s='\0" as *const u8 as *const c_char,
                *atts.offset(fresh3 as isize),
            );
            if !(*atts.offset(i as isize)).is_null() {
                fprintf(
                    stdout,
                    b"%s'\0" as *const u8 as *const c_char,
                    *atts.offset(i as isize),
                );
            }
            i += 1;
        }
    }
    fprintf(stdout, b")\n\0" as *const u8 as *const c_char);
}
unsafe extern "C" fn endElementDebug(mut ctx: *mut c_void, mut name: *const xmlChar) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.endElement(%s)\n\0" as *const u8 as *const c_char,
        name as *mut c_char,
    );
}
unsafe extern "C" fn charactersDebug(
    mut ctx: *mut c_void,
    mut ch: *const xmlChar,
    mut len: c_int,
) {
    let mut out: [c_char; 40] = [0; 40];
    let mut i: c_int = 0;
    callbacks += 1;
    if noout != 0 {
        return;
    }
    i = 0 as c_int;
    while i < len && i < 30 as c_int {
        out[i as usize] = *ch.offset(i as isize) as c_char;
        i += 1;
    }
    out[i as usize] = 0 as c_char;
    fprintf(
        stdout,
        b"SAX.characters(%s, %d)\n\0" as *const u8 as *const c_char,
        &raw mut out as *mut c_char,
        len,
    );
}
unsafe extern "C" fn referenceDebug(mut ctx: *mut c_void, mut name: *const xmlChar) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.reference(%s)\n\0" as *const u8 as *const c_char,
        name,
    );
}
unsafe extern "C" fn ignorableWhitespaceDebug(
    mut ctx: *mut c_void,
    mut ch: *const xmlChar,
    mut len: c_int,
) {
    let mut out: [c_char; 40] = [0; 40];
    let mut i: c_int = 0;
    callbacks += 1;
    if noout != 0 {
        return;
    }
    i = 0 as c_int;
    while i < len && i < 30 as c_int {
        out[i as usize] = *ch.offset(i as isize) as c_char;
        i += 1;
    }
    out[i as usize] = 0 as c_char;
    fprintf(
        stdout,
        b"SAX.ignorableWhitespace(%s, %d)\n\0" as *const u8 as *const c_char,
        &raw mut out as *mut c_char,
        len,
    );
}
unsafe extern "C" fn processingInstructionDebug(
    mut ctx: *mut c_void,
    mut target: *const xmlChar,
    mut data: *const xmlChar,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    if !data.is_null() {
        fprintf(
            stdout,
            b"SAX.processingInstruction(%s, %s)\n\0" as *const u8 as *const c_char,
            target as *mut c_char,
            data as *mut c_char,
        );
    } else {
        fprintf(
            stdout,
            b"SAX.processingInstruction(%s, NULL)\n\0" as *const u8 as *const c_char,
            target as *mut c_char,
        );
    };
}
unsafe extern "C" fn cdataBlockDebug(
    mut ctx: *mut c_void,
    mut value: *const xmlChar,
    mut len: c_int,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.pcdata(%.20s, %d)\n\0" as *const u8 as *const c_char,
        value as *mut c_char,
        len,
    );
}
unsafe extern "C" fn commentDebug(mut ctx: *mut c_void, mut value: *const xmlChar) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.comment(%s)\n\0" as *const u8 as *const c_char,
        value,
    );
}
unsafe extern "C" fn warningDebug(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    callbacks += 1;
    if noout != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(
        stdout,
        b"SAX.warning: \0" as *const u8 as *const c_char,
    );
    vfprintf(stdout, msg, args_0.as_va_list());
}
unsafe extern "C" fn errorDebug(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    callbacks += 1;
    if noout != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(
        stdout,
        b"SAX.error: \0" as *const u8 as *const c_char,
    );
    vfprintf(stdout, msg, args_0.as_va_list());
}
unsafe extern "C" fn fatalErrorDebug(
    mut ctx: *mut c_void,
    mut msg: *const c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    callbacks += 1;
    if noout != 0 {
        return;
    }
    args_0 = args.clone();
    fprintf(
        stdout,
        b"SAX.fatalError: \0" as *const u8 as *const c_char,
    );
    vfprintf(stdout, msg, args_0.as_va_list());
}
static mut debugSAXHandlerStruct: xmlSAXHandler = unsafe {
    _xmlSAXHandler {
        internalSubset: Some(
            internalSubsetDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        isStandalone: Some(
            isStandaloneDebug
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        hasInternalSubset: Some(
            hasInternalSubsetDebug
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        hasExternalSubset: Some(
            hasExternalSubsetDebug
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        resolveEntity: Some(
            resolveEntityDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> xmlParserInputPtr,
        ),
        getEntity: Some(
            getEntityDebug
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: Some(
            entityDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                    *const xmlChar,
                    *const xmlChar,
                    *mut xmlChar,
                ) -> (),
        ),
        notationDecl: Some(
            notationDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        attributeDecl: Some(
            attributeDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    c_int,
                    c_int,
                    *const xmlChar,
                    xmlEnumerationPtr,
                ) -> (),
        ),
        elementDecl: Some(
            elementDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                    xmlElementContentPtr,
                ) -> (),
        ),
        unparsedEntityDecl: Some(
            unparsedEntityDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        setDocumentLocator: Some(
            setDocumentLocatorDebug
                as unsafe extern "C" fn(*mut c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            startDocumentDebug as unsafe extern "C" fn(*mut c_void) -> (),
        ),
        endDocument: Some(endDocumentDebug as unsafe extern "C" fn(*mut c_void) -> ()),
        startElement: Some(
            startElementDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *mut *const xmlChar,
                ) -> (),
        ),
        endElement: Some(
            endElementDebug as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        reference: Some(
            referenceDebug as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        characters: Some(
            charactersDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            ignorableWhitespaceDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        processingInstruction: Some(
            processingInstructionDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        comment: Some(
            commentDebug as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            warningDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            errorDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            fatalErrorDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: Some(
            getParameterEntityDebug
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        cdataBlock: Some(
            cdataBlockDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        externalSubset: Some(
            externalSubsetDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        initialized: 1 as c_uint,
        _private: NULL_0,
        startElementNs: None,
        endElementNs: None,
        serror: None,
    }
};
#[no_mangle]
pub static mut debugSAXHandler: xmlSAXHandlerPtr =
    unsafe { &raw const debugSAXHandlerStruct as xmlSAXHandlerPtr };
unsafe extern "C" fn startElementNsDebug(
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
    let mut i: c_int = 0;
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.startElementNs(%s\0" as *const u8 as *const c_char,
        localname as *mut c_char,
    );
    if prefix.is_null() {
        fprintf(
            stdout,
            b", NULL\0" as *const u8 as *const c_char,
        );
    } else {
        fprintf(
            stdout,
            b", %s\0" as *const u8 as *const c_char,
            prefix as *mut c_char,
        );
    }
    if URI.is_null() {
        fprintf(
            stdout,
            b", NULL\0" as *const u8 as *const c_char,
        );
    } else {
        fprintf(
            stdout,
            b", '%s'\0" as *const u8 as *const c_char,
            URI as *mut c_char,
        );
    }
    fprintf(
        stdout,
        b", %d\0" as *const u8 as *const c_char,
        nb_namespaces,
    );
    if !namespaces.is_null() {
        i = 0 as c_int;
        while i < nb_namespaces * 2 as c_int {
            fprintf(
                stdout,
                b", xmlns\0" as *const u8 as *const c_char,
            );
            if !(*namespaces.offset(i as isize)).is_null() {
                fprintf(
                    stdout,
                    b":%s\0" as *const u8 as *const c_char,
                    *namespaces.offset(i as isize),
                );
            }
            i += 1;
            fprintf(
                stdout,
                b"='%s'\0" as *const u8 as *const c_char,
                *namespaces.offset(i as isize),
            );
            i += 1;
        }
    }
    fprintf(
        stdout,
        b", %d, %d\0" as *const u8 as *const c_char,
        nb_attributes,
        nb_defaulted,
    );
    if !attributes.is_null() {
        i = 0 as c_int;
        while i < nb_attributes * 5 as c_int {
            if !(*attributes.offset((i + 1 as c_int) as isize)).is_null() {
                fprintf(
                    stdout,
                    b", %s:%s='\0" as *const u8 as *const c_char,
                    *attributes.offset((i + 1 as c_int) as isize),
                    *attributes.offset(i as isize),
                );
            } else {
                fprintf(
                    stdout,
                    b", %s='\0" as *const u8 as *const c_char,
                    *attributes.offset(i as isize),
                );
            }
            fprintf(
                stdout,
                b"%.4s...', %d\0" as *const u8 as *const c_char,
                *attributes.offset((i + 3 as c_int) as isize),
                (*attributes.offset((i + 4 as c_int) as isize))
                    .offset_from(*attributes.offset((i + 3 as c_int) as isize))
                    as c_long as c_int,
            );
            i += 5 as c_int;
        }
    }
    fprintf(stdout, b")\n\0" as *const u8 as *const c_char);
}
unsafe extern "C" fn endElementNsDebug(
    mut ctx: *mut c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
) {
    callbacks += 1;
    if noout != 0 {
        return;
    }
    fprintf(
        stdout,
        b"SAX.endElementNs(%s\0" as *const u8 as *const c_char,
        localname as *mut c_char,
    );
    if prefix.is_null() {
        fprintf(
            stdout,
            b", NULL\0" as *const u8 as *const c_char,
        );
    } else {
        fprintf(
            stdout,
            b", %s\0" as *const u8 as *const c_char,
            prefix as *mut c_char,
        );
    }
    if URI.is_null() {
        fprintf(
            stdout,
            b", NULL)\n\0" as *const u8 as *const c_char,
        );
    } else {
        fprintf(
            stdout,
            b", '%s')\n\0" as *const u8 as *const c_char,
            URI as *mut c_char,
        );
    };
}
static mut debugSAX2HandlerStruct: xmlSAXHandler = unsafe {
    _xmlSAXHandler {
        internalSubset: Some(
            internalSubsetDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        isStandalone: Some(
            isStandaloneDebug
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        hasInternalSubset: Some(
            hasInternalSubsetDebug
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        hasExternalSubset: Some(
            hasExternalSubsetDebug
                as unsafe extern "C" fn(*mut c_void) -> c_int,
        ),
        resolveEntity: Some(
            resolveEntityDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> xmlParserInputPtr,
        ),
        getEntity: Some(
            getEntityDebug
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        entityDecl: Some(
            entityDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                    *const xmlChar,
                    *const xmlChar,
                    *mut xmlChar,
                ) -> (),
        ),
        notationDecl: Some(
            notationDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        attributeDecl: Some(
            attributeDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    c_int,
                    c_int,
                    *const xmlChar,
                    xmlEnumerationPtr,
                ) -> (),
        ),
        elementDecl: Some(
            elementDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                    xmlElementContentPtr,
                ) -> (),
        ),
        unparsedEntityDecl: Some(
            unparsedEntityDeclDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        setDocumentLocator: Some(
            setDocumentLocatorDebug
                as unsafe extern "C" fn(*mut c_void, xmlSAXLocatorPtr) -> (),
        ),
        startDocument: Some(
            startDocumentDebug as unsafe extern "C" fn(*mut c_void) -> (),
        ),
        endDocument: Some(endDocumentDebug as unsafe extern "C" fn(*mut c_void) -> ()),
        startElement: None,
        endElement: None,
        reference: Some(
            referenceDebug as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        characters: Some(
            charactersDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        ignorableWhitespace: Some(
            ignorableWhitespaceDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        processingInstruction: Some(
            processingInstructionDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        comment: Some(
            commentDebug as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> (),
        ),
        warning: Some(
            warningDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        error: Some(
            errorDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        fatalError: Some(
            fatalErrorDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ),
        getParameterEntity: Some(
            getParameterEntityDebug
                as unsafe extern "C" fn(*mut c_void, *const xmlChar) -> xmlEntityPtr,
        ),
        cdataBlock: Some(
            cdataBlockDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    c_int,
                ) -> (),
        ),
        externalSubset: Some(
            externalSubsetDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        initialized: XML_SAX2_MAGIC,
        _private: NULL_0,
        startElementNs: Some(
            startElementNsDebug
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
        ),
        endElementNs: Some(
            endElementNsDebug
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        serror: None,
    }
};
static mut debugSAX2Handler: xmlSAXHandlerPtr =
    unsafe { &raw const debugSAX2HandlerStruct as xmlSAXHandlerPtr };
unsafe extern "C" fn testSAX(mut filename: *const c_char) {
    let mut handler: xmlSAXHandlerPtr = ::core::ptr::null_mut::<xmlSAXHandler>();
    let mut user_data: *const c_char =
        b"user_data\0" as *const u8 as *const c_char;
    callbacks = 0 as c_int;
    if noout != 0 {
        handler = emptySAXHandler;
    } else if sax1 != 0 {
        handler = debugSAXHandler;
    } else {
        handler = debugSAX2Handler;
    }
    if !wxschemas.is_null() {
        let mut ret: c_int = 0;
        let mut vctxt: xmlSchemaValidCtxtPtr = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
        let mut buf: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
        buf = xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
        if buf.is_null() {
            return;
        }
        vctxt = xmlSchemaNewValidCtxt(wxschemas);
        if vctxt.is_null() {
            progresult = XMLLINT_ERR_MEM;
            xmlFreeParserInputBuffer(buf);
            return;
        }
        xmlSchemaSetValidErrors(vctxt, *__xmlGenericError(), *__xmlGenericError(), NULL_0);
        xmlSchemaValidateSetFilename(vctxt, filename);
        ret = xmlSchemaValidateStream(
            vctxt,
            buf,
            XML_CHAR_ENCODING_NONE,
            handler,
            user_data as *mut c_void,
        );
        if repeat == 0 as c_int {
            if ret == 0 as c_int {
                if quiet == 0 {
                    fprintf(
                        stderr,
                        b"%s validates\n\0" as *const u8 as *const c_char,
                        filename,
                    );
                }
            } else if ret > 0 as c_int {
                fprintf(
                    stderr,
                    b"%s fails to validate\n\0" as *const u8 as *const c_char,
                    filename,
                );
                progresult = XMLLINT_ERR_VALID;
            } else {
                fprintf(
                    stderr,
                    b"%s validation generated an internal error\n\0" as *const u8
                        as *const c_char,
                    filename,
                );
                progresult = XMLLINT_ERR_VALID;
            }
        }
        xmlSchemaFreeValidCtxt(vctxt);
    } else {
        let mut ctxt: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
        ctxt = xmlNewSAXParserCtxt(
            handler as *const xmlSAXHandler,
            user_data as *mut c_void,
        );
        if ctxt.is_null() {
            progresult = XMLLINT_ERR_MEM;
            return;
        }
        if maxAmpl > 0 as c_uint {
            xmlCtxtSetMaxAmplification(ctxt, maxAmpl);
        }
        xmlCtxtReadFile(
            ctxt,
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        );
        if !(*ctxt).myDoc.is_null() {
            fprintf(
                stderr,
                b"SAX generated a doc !\n\0" as *const u8 as *const c_char,
            );
            xmlFreeDoc((*ctxt).myDoc);
            (*ctxt).myDoc = ::core::ptr::null_mut::<xmlDoc>();
        }
        xmlFreeParserCtxt(ctxt);
    };
}
extern "C" fn processNode(mut reader: xmlTextReaderPtr) { unsafe {
    let mut name: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut value: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut type_0: c_int = 0;
    let mut empty: c_int = 0;
    type_0 = xmlTextReaderNodeType(reader);
    empty = xmlTextReaderIsEmptyElement(reader);
    if debug != 0 {
        name = xmlTextReaderConstName(reader);
        if name.is_null() {
            name = b"--\0" as *const u8 as *const c_char as *mut xmlChar;
        }
        value = xmlTextReaderConstValue(reader);
        printf(
            b"%d %d %s %d %d\0" as *const u8 as *const c_char,
            xmlTextReaderDepth(reader),
            type_0,
            name,
            empty,
            xmlTextReaderHasValue(reader),
        );
        if value.is_null() {
            printf(b"\n\0" as *const u8 as *const c_char);
        } else {
            printf(b" %s\n\0" as *const u8 as *const c_char, value);
        }
    }
    if !patternc.is_null() {
        let mut path: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut match_0: c_int = -(1 as c_int);
        if type_0 == XML_READER_TYPE_ELEMENT as c_int {
            match_0 = xmlPatternMatch(patternc, xmlTextReaderCurrentNode(reader));
            if match_0 != 0 {
                path = xmlGetNodePath(xmlTextReaderCurrentNode(reader) as *const xmlNode);
                printf(
                    b"Node %s matches pattern %s\n\0" as *const u8 as *const c_char,
                    path,
                    pattern,
                );
            }
        }
        if !patstream.is_null() {
            let mut ret: c_int = 0;
            if type_0 == XML_READER_TYPE_ELEMENT as c_int {
                ret = xmlStreamPush(
                    patstream,
                    xmlTextReaderConstLocalName(reader),
                    xmlTextReaderConstNamespaceUri(reader),
                );
                if ret < 0 as c_int {
                    fprintf(
                        stderr,
                        b"xmlStreamPush() failure\n\0" as *const u8 as *const c_char,
                    );
                    xmlFreeStreamCtxt(patstream);
                    patstream = ::core::ptr::null_mut::<xmlStreamCtxt>();
                } else if ret != match_0 {
                    if path.is_null() {
                        path = xmlGetNodePath(xmlTextReaderCurrentNode(reader) as *const xmlNode);
                    }
                    fprintf(
                        stderr,
                        b"xmlPatternMatch and xmlStreamPush disagree\n\0" as *const u8
                            as *const c_char,
                    );
                    if !path.is_null() {
                        fprintf(
                            stderr,
                            b"  pattern %s node %s\n\0" as *const u8 as *const c_char,
                            pattern,
                            path,
                        );
                    } else {
                        fprintf(
                            stderr,
                            b"  pattern %s node %s\n\0" as *const u8 as *const c_char,
                            pattern,
                            xmlTextReaderConstName(reader),
                        );
                    }
                }
            }
            if type_0 == XML_READER_TYPE_END_ELEMENT as c_int
                || type_0 == XML_READER_TYPE_ELEMENT as c_int && empty != 0
            {
                ret = xmlStreamPop(patstream);
                if ret < 0 as c_int {
                    fprintf(
                        stderr,
                        b"xmlStreamPop() failure\n\0" as *const u8 as *const c_char,
                    );
                    xmlFreeStreamCtxt(patstream);
                    patstream = ::core::ptr::null_mut::<xmlStreamCtxt>();
                }
            }
        }
        if !path.is_null() {
            xmlFree.expect("non-null function pointer")(path as *mut c_void);
        }
    }
} }
unsafe extern "C" fn streamFile(mut filename: *mut c_char) {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    let mut ret: c_int = 0;
    let mut fd: c_int = -(1 as c_int);
    let mut info: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut base: *const c_char = ::core::ptr::null::<c_char>();
    let mut input: xmlParserInputBufferPtr = ::core::ptr::null_mut::<xmlParserInputBuffer>();
    if memory != 0 {
        if stat(filename, &raw mut info) < 0 as c_int {
            return;
        }
        fd = open(filename, O_RDONLY);
        if fd < 0 as c_int {
            return;
        }
        base = mmap(
            NULL_0,
            info.st_size as size_t,
            PROT_READ,
            MAP_SHARED,
            fd,
            0 as __off64_t,
        ) as *const c_char;
        if base == MAP_FAILED as *const c_char {
            close(fd);
            fprintf(
                stderr,
                b"mmap failure for file %s\n\0" as *const u8 as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_RDFILE;
            return;
        }
        reader = xmlReaderForMemory(
            base,
            info.st_size as c_int,
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        );
    } else {
        reader = xmlReaderForFile(
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        );
    }
    if !patternc.is_null() {
        patstream = xmlPatternGetStreamCtxt(patternc);
        if !patstream.is_null() {
            ret = xmlStreamPush(
                patstream,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            if ret < 0 as c_int {
                fprintf(
                    stderr,
                    b"xmlStreamPush() failure\n\0" as *const u8 as *const c_char,
                );
                xmlFreeStreamCtxt(patstream);
                patstream = ::core::ptr::null_mut::<xmlStreamCtxt>();
            }
        }
    }
    if !reader.is_null() {
        if maxAmpl > 0 as c_uint {
            xmlTextReaderSetMaxAmplification(reader, maxAmpl);
        }
        if valid != 0 {
            xmlTextReaderSetParserProp(
                reader,
                XML_PARSER_VALIDATE as c_int,
                1 as c_int,
            );
        } else if loaddtd != 0 {
            xmlTextReaderSetParserProp(
                reader,
                XML_PARSER_LOADDTD as c_int,
                1 as c_int,
            );
        }
        if !relaxng.is_null() {
            if timing != 0 && repeat == 0 {
                startTimer();
            }
            ret = xmlTextReaderRelaxNGValidate(reader, relaxng);
            if ret < 0 as c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Relax-NG schema %s failed to compile\n\0" as *const u8
                        as *const c_char,
                    relaxng,
                );
                progresult = XMLLINT_ERR_SCHEMACOMP;
                relaxng = ::core::ptr::null_mut::<c_char>();
            }
            if timing != 0 && repeat == 0 {
                endTimer(b"Compiling the schemas\0" as *const u8 as *const c_char);
            }
        }
        if !schema.is_null() {
            if timing != 0 && repeat == 0 {
                startTimer();
            }
            ret = xmlTextReaderSchemaValidate(reader, schema);
            if ret < 0 as c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"XSD schema %s failed to compile\n\0" as *const u8
                        as *const c_char,
                    schema,
                );
                progresult = XMLLINT_ERR_SCHEMACOMP;
                schema = ::core::ptr::null_mut::<c_char>();
            }
            if timing != 0 && repeat == 0 {
                endTimer(b"Compiling the schemas\0" as *const u8 as *const c_char);
            }
        }
        if timing != 0 && repeat == 0 {
            startTimer();
        }
        ret = xmlTextReaderRead(reader);
        while ret == 1 as c_int {
            if debug != 0 || !patternc.is_null() {
                processNode(reader);
            }
            ret = xmlTextReaderRead(reader);
        }
        if timing != 0 && repeat == 0 {
            if !relaxng.is_null() {
                endTimer(b"Parsing and validating\0" as *const u8 as *const c_char);
            } else if valid != 0 {
                endTimer(b"Parsing and validating\0" as *const u8 as *const c_char);
            } else {
                endTimer(b"Parsing\0" as *const u8 as *const c_char);
            }
        }
        if valid != 0 {
            if xmlTextReaderIsValid(reader) != 1 as c_int {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Document %s does not validate\n\0" as *const u8 as *const c_char,
                    filename,
                );
                progresult = XMLLINT_ERR_VALID;
            }
        }
        if !relaxng.is_null() || !schema.is_null() {
            if xmlTextReaderIsValid(reader) != 1 as c_int {
                fprintf(
                    stderr,
                    b"%s fails to validate\n\0" as *const u8 as *const c_char,
                    filename,
                );
                progresult = XMLLINT_ERR_VALID;
            } else if quiet == 0 {
                fprintf(
                    stderr,
                    b"%s validates\n\0" as *const u8 as *const c_char,
                    filename,
                );
            }
        }
        xmlFreeTextReader(reader);
        if ret != 0 as c_int {
            fprintf(
                stderr,
                b"%s : failed to parse\n\0" as *const u8 as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_UNCLASS;
        }
    } else {
        fprintf(
            stderr,
            b"Unable to open %s\n\0" as *const u8 as *const c_char,
            filename,
        );
        progresult = XMLLINT_ERR_UNCLASS;
    }
    if !patstream.is_null() {
        xmlFreeStreamCtxt(patstream);
        patstream = ::core::ptr::null_mut::<xmlStreamCtxt>();
    }
    if memory != 0 {
        xmlFreeParserInputBuffer(input);
        munmap(
            base as *mut c_char as *mut c_void,
            info.st_size as size_t,
        );
        close(fd);
    }
}
extern "C" fn walkDoc(mut doc: xmlDocPtr) { unsafe {
    let mut reader: xmlTextReaderPtr = ::core::ptr::null_mut::<xmlTextReader>();
    let mut ret: c_int = 0;
    let mut root: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
    let mut namespaces: [*const xmlChar; 22] = [::core::ptr::null::<xmlChar>(); 22];
    let mut i: c_int = 0;
    let mut ns: xmlNsPtr = ::core::ptr::null_mut::<xmlNs>();
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"Document does not have a root element\0" as *const u8 as *const c_char,
        );
        progresult = XMLLINT_ERR_UNCLASS;
        return;
    }
    ns = (*root).nsDef as xmlNsPtr;
    i = 0 as c_int;
    while !ns.is_null() && i < 20 as c_int {
        let fresh4 = i;
        i = i + 1;
        namespaces[fresh4 as usize] = (*ns).href;
        let fresh5 = i;
        i = i + 1;
        namespaces[fresh5 as usize] = (*ns).prefix;
        ns = (*ns).next as xmlNsPtr;
    }
    let fresh6 = i;
    i = i + 1;
    namespaces[fresh6 as usize] = ::core::ptr::null::<xmlChar>();
    namespaces[i as usize] = ::core::ptr::null::<xmlChar>();
    if !pattern.is_null() {
        patternc = xmlPatterncompile(
            pattern as *const xmlChar,
            (*doc).dict as *mut xmlDict,
            0 as c_int,
            (&raw mut namespaces as *mut *const xmlChar).offset(0 as c_int as isize)
                as *mut *const xmlChar,
        );
        if patternc.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Pattern %s failed to compile\n\0" as *const u8 as *const c_char,
                pattern,
            );
            progresult = XMLLINT_ERR_SCHEMAPAT;
            pattern = ::core::ptr::null::<c_char>();
        }
    }
    if !patternc.is_null() {
        patstream = xmlPatternGetStreamCtxt(patternc);
        if !patstream.is_null() {
            ret = xmlStreamPush(
                patstream,
                ::core::ptr::null::<xmlChar>(),
                ::core::ptr::null::<xmlChar>(),
            );
            if ret < 0 as c_int {
                fprintf(
                    stderr,
                    b"xmlStreamPush() failure\n\0" as *const u8 as *const c_char,
                );
                xmlFreeStreamCtxt(patstream);
                patstream = ::core::ptr::null_mut::<xmlStreamCtxt>();
            }
        }
    }
    reader = xmlReaderWalker(doc);
    if !reader.is_null() {
        if timing != 0 && repeat == 0 {
            startTimer();
        }
        ret = xmlTextReaderRead(reader);
        while ret == 1 as c_int {
            if debug != 0 || !patternc.is_null() {
                processNode(reader);
            }
            ret = xmlTextReaderRead(reader);
        }
        if timing != 0 && repeat == 0 {
            endTimer(b"walking through the doc\0" as *const u8 as *const c_char);
        }
        xmlFreeTextReader(reader);
        if ret != 0 as c_int {
            fprintf(
                stderr,
                b"failed to walk through the doc\n\0" as *const u8 as *const c_char,
            );
            progresult = XMLLINT_ERR_UNCLASS;
        }
    } else {
        fprintf(
            stderr,
            b"Failed to crate a reader from the document\n\0" as *const u8
                as *const c_char,
        );
        progresult = XMLLINT_ERR_UNCLASS;
    }
    if !patstream.is_null() {
        xmlFreeStreamCtxt(patstream);
        patstream = ::core::ptr::null_mut::<xmlStreamCtxt>();
    }
} }
extern "C" fn doXPathDump(mut cur: xmlXPathObjectPtr) { unsafe {
    match (*cur).type_0 as c_uint {
        1 => {
            let mut i: c_int = 0;
            let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            let mut buf: xmlOutputBufferPtr = ::core::ptr::null_mut::<xmlOutputBuffer>();
            if (*cur).nodesetval.is_null() || (*(*cur).nodesetval).nodeNr <= 0 as c_int
            {
                if quiet == 0 {
                    fprintf(
                        stderr,
                        b"XPath set is empty\n\0" as *const u8 as *const c_char,
                    );
                }
            } else {
                buf = xmlOutputBufferCreateFile(
                    stdout,
                    ::core::ptr::null_mut::<xmlCharEncodingHandler>(),
                );
                if buf.is_null() {
                    fprintf(
                        stderr,
                        b"Out of memory for XPath\n\0" as *const u8 as *const c_char,
                    );
                    progresult = XMLLINT_ERR_MEM;
                    return;
                }
                i = 0 as c_int;
                while i < (*(*cur).nodesetval).nodeNr {
                    node = *(*(*cur).nodesetval).nodeTab.offset(i as isize);
                    xmlNodeDumpOutput(
                        buf,
                        ::core::ptr::null_mut::<xmlDoc>(),
                        node,
                        0 as c_int,
                        0 as c_int,
                        ::core::ptr::null::<c_char>(),
                    );
                    xmlOutputBufferWrite(
                        buf,
                        1 as c_int,
                        b"\n\0" as *const u8 as *const c_char,
                    );
                    i += 1;
                }
                xmlOutputBufferClose(buf);
            }
        }
        2 => {
            if (*cur).boolval != 0 {
                printf(b"true\n\0" as *const u8 as *const c_char);
            } else {
                printf(b"false\n\0" as *const u8 as *const c_char);
            }
        }
        3 => match xmlXPathIsInf((*cur).floatval) {
            1 => {
                printf(b"Infinity\n\0" as *const u8 as *const c_char);
            }
            -1 => {
                printf(b"-Infinity\n\0" as *const u8 as *const c_char);
            }
            _ => {
                if xmlXPathIsNaN((*cur).floatval) != 0 {
                    printf(b"NaN\n\0" as *const u8 as *const c_char);
                } else {
                    printf(
                        b"%0g\n\0" as *const u8 as *const c_char,
                        (*cur).floatval,
                    );
                }
            }
        },
        4 => {
            printf(
                b"%s\n\0" as *const u8 as *const c_char,
                (*cur).stringval as *const c_char,
            );
        }
        0 => {
            fprintf(
                stderr,
                b"XPath Object is uninitialized\n\0" as *const u8 as *const c_char,
            );
            progresult = XMLLINT_ERR_XPATH;
        }
        _ => {
            fprintf(
                stderr,
                b"XPath object of unexpected type\n\0" as *const u8 as *const c_char,
            );
            progresult = XMLLINT_ERR_XPATH;
        }
    };
} }
unsafe extern "C" fn doXPathQuery(mut doc: xmlDocPtr, mut query: *const c_char) {
    let mut ctxt: xmlXPathContextPtr = ::core::ptr::null_mut::<xmlXPathContext>();
    let mut res: xmlXPathObjectPtr = ::core::ptr::null_mut::<xmlXPathObject>();
    ctxt = xmlXPathNewContext(doc);
    if ctxt.is_null() {
        fprintf(
            stderr,
            b"Out of memory for XPath\n\0" as *const u8 as *const c_char,
        );
        progresult = XMLLINT_ERR_MEM;
        return;
    }
    (*ctxt).node = doc as xmlNodePtr;
    res = xmlXPathEval(query as *mut xmlChar, ctxt);
    xmlXPathFreeContext(ctxt);
    if res.is_null() {
        fprintf(
            stderr,
            b"XPath evaluation failure\n\0" as *const u8 as *const c_char,
        );
        progresult = XMLLINT_ERR_XPATH;
        return;
    }
    doXPathDump(res);
    xmlXPathFreeObject(res);
}
unsafe extern "C" fn parseAndPrintFile(
    mut filename: *mut c_char,
    mut rectxt: xmlParserCtxtPtr,
) {
    let mut doc: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    let mut tmp: xmlDocPtr = ::core::ptr::null_mut::<xmlDoc>();
    if timing != 0 && repeat == 0 {
        startTimer();
    }
    if filename.is_null() {
        if generate != 0 {
            let mut n: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
            doc = xmlNewDoc(b"1.0\0" as *const u8 as *const c_char as *mut xmlChar);
            n = xmlNewDocNode(
                doc,
                ::core::ptr::null_mut::<xmlNs>(),
                b"info\0" as *const u8 as *const c_char as *mut xmlChar,
                ::core::ptr::null::<xmlChar>(),
            );
            xmlNodeSetContent(
                n,
                b"abc\0" as *const u8 as *const c_char as *mut xmlChar,
            );
            xmlDocSetRootElement(doc, n);
        }
    } else if html != 0 && push != 0 {
        let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut res: c_int = 0;
        let mut chars: [c_char; 4096] = [0; 4096];
        let mut ctxt: htmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
        if *filename.offset(0 as c_int as isize) as c_int == '-' as i32
            && *filename.offset(1 as c_int as isize) as c_int
                == 0 as c_int
        {
            f = stdin;
        } else {
            f = fopen(filename, b"rb\0" as *const u8 as *const c_char);
            if f.is_null() {
                fprintf(
                    stderr,
                    b"Can't open %s\n\0" as *const u8 as *const c_char,
                    filename,
                );
                progresult = XMLLINT_ERR_UNCLASS;
                return;
            }
        }
        res = fread(
            &raw mut chars as *mut c_char as *mut c_void,
            1 as size_t,
            4 as size_t,
            f,
        ) as c_int;
        ctxt = htmlCreatePushParserCtxt(
            ::core::ptr::null_mut::<xmlSAXHandler>(),
            NULL_0,
            &raw mut chars as *mut c_char,
            res,
            filename,
            XML_CHAR_ENCODING_NONE,
        );
        if ctxt.is_null() {
            progresult = XMLLINT_ERR_MEM;
            if f != stdin {
                fclose(f);
            }
            return;
        }
        htmlCtxtUseOptions(ctxt, options);
        loop {
            res = fread(
                &raw mut chars as *mut c_char as *mut c_void,
                1 as size_t,
                pushsize as size_t,
                f,
            ) as c_int;
            if !(res > 0 as c_int) {
                break;
            }
            htmlParseChunk(
                ctxt,
                &raw mut chars as *mut c_char,
                res,
                0 as c_int,
            );
        }
        htmlParseChunk(
            ctxt,
            &raw mut chars as *mut c_char,
            0 as c_int,
            1 as c_int,
        );
        doc = (*ctxt).myDoc;
        htmlFreeParserCtxt(ctxt);
        if f != stdin {
            fclose(f);
        }
    } else if html != 0 && memory != 0 {
        let mut fd: c_int = 0;
        let mut info: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut base: *const c_char = ::core::ptr::null::<c_char>();
        if stat(filename, &raw mut info) < 0 as c_int {
            return;
        }
        fd = open(filename, O_RDONLY);
        if fd < 0 as c_int {
            return;
        }
        base = mmap(
            NULL_0,
            info.st_size as size_t,
            PROT_READ,
            MAP_SHARED,
            fd,
            0 as __off64_t,
        ) as *const c_char;
        if base == MAP_FAILED as *const c_char {
            close(fd);
            fprintf(
                stderr,
                b"mmap failure for file %s\n\0" as *const u8 as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_RDFILE;
            return;
        }
        doc = htmlReadMemory(
            base as *mut c_char,
            info.st_size as c_int,
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        ) as xmlDocPtr;
        munmap(
            base as *mut c_char as *mut c_void,
            info.st_size as size_t,
        );
        close(fd);
    } else if html != 0 {
        doc = htmlReadFile(
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        ) as xmlDocPtr;
    } else if push != 0 {
        let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut ret: c_int = 0;
        let mut res_0: c_int = 0;
        let mut size: c_int = 1024 as c_int;
        let mut chars_0: [c_char; 1024] = [0; 1024];
        let mut ctxt_0: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
        if *filename.offset(0 as c_int as isize) as c_int == '-' as i32
            && *filename.offset(1 as c_int as isize) as c_int
                == 0 as c_int
        {
            f_0 = stdin;
        } else {
            f_0 = fopen(filename, b"rb\0" as *const u8 as *const c_char);
            if f_0.is_null() {
                fprintf(
                    stderr,
                    b"Can't open %s\n\0" as *const u8 as *const c_char,
                    filename,
                );
                progresult = XMLLINT_ERR_UNCLASS;
                return;
            }
        }
        res_0 = fread(
            &raw mut chars_0 as *mut c_char as *mut c_void,
            1 as size_t,
            4 as size_t,
            f_0,
        ) as c_int;
        ctxt_0 = xmlCreatePushParserCtxt(
            ::core::ptr::null_mut::<xmlSAXHandler>(),
            NULL_0,
            &raw mut chars_0 as *mut c_char,
            res_0,
            filename,
        );
        if ctxt_0.is_null() {
            progresult = XMLLINT_ERR_MEM;
            if f_0 != stdin {
                fclose(f_0);
            }
            return;
        }
        xmlCtxtUseOptions(ctxt_0, options);
        if maxAmpl > 0 as c_uint {
            xmlCtxtSetMaxAmplification(ctxt_0, maxAmpl);
        }
        loop {
            res_0 = fread(
                &raw mut chars_0 as *mut c_char as *mut c_void,
                1 as size_t,
                size as size_t,
                f_0,
            ) as c_int;
            if !(res_0 > 0 as c_int) {
                break;
            }
            xmlParseChunk(
                ctxt_0,
                &raw mut chars_0 as *mut c_char,
                res_0,
                0 as c_int,
            );
        }
        xmlParseChunk(
            ctxt_0,
            &raw mut chars_0 as *mut c_char,
            0 as c_int,
            1 as c_int,
        );
        doc = (*ctxt_0).myDoc;
        ret = (*ctxt_0).wellFormed;
        xmlFreeParserCtxt(ctxt_0);
        if ret == 0 && recovery == 0 {
            xmlFreeDoc(doc);
            doc = ::core::ptr::null_mut::<xmlDoc>();
        }
        if f_0 != stdin {
            fclose(f_0);
        }
    } else if testIO != 0 {
        if *filename.offset(0 as c_int as isize) as c_int == '-' as i32
            && *filename.offset(1 as c_int as isize) as c_int
                == 0 as c_int
        {
            doc = xmlReadFd(
                0 as c_int,
                ::core::ptr::null::<c_char>(),
                ::core::ptr::null::<c_char>(),
                options,
            );
        } else {
            let mut f_1: *mut FILE = ::core::ptr::null_mut::<FILE>();
            f_1 = fopen(filename, b"rb\0" as *const u8 as *const c_char);
            if !f_1.is_null() {
                if rectxt.is_null() {
                    doc = xmlReadIO(
                        Some(
                            myRead
                                as unsafe extern "C" fn(
                                    *mut c_void,
                                    *mut c_char,
                                    c_int,
                                )
                                    -> c_int,
                        ),
                        Some(
                            myClose
                                as unsafe extern "C" fn(
                                    *mut c_void,
                                )
                                    -> c_int,
                        ),
                        f_1 as *mut c_void,
                        filename,
                        ::core::ptr::null::<c_char>(),
                        options,
                    );
                } else {
                    doc = xmlCtxtReadIO(
                        rectxt,
                        Some(
                            myRead
                                as unsafe extern "C" fn(
                                    *mut c_void,
                                    *mut c_char,
                                    c_int,
                                )
                                    -> c_int,
                        ),
                        Some(
                            myClose
                                as unsafe extern "C" fn(
                                    *mut c_void,
                                )
                                    -> c_int,
                        ),
                        f_1 as *mut c_void,
                        filename,
                        ::core::ptr::null::<c_char>(),
                        options,
                    );
                }
            } else {
                doc = ::core::ptr::null_mut::<xmlDoc>();
            }
        }
    } else if htmlout != 0 {
        let mut ctxt_1: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
        if rectxt.is_null() {
            ctxt_1 = xmlNewParserCtxt();
            if ctxt_1.is_null() {
                progresult = XMLLINT_ERR_MEM;
                return;
            }
            if maxAmpl > 0 as c_uint {
                xmlCtxtSetMaxAmplification(ctxt_1, maxAmpl);
            }
        } else {
            ctxt_1 = rectxt;
        }
        (*(*ctxt_1).sax).error = Some(
            xmlHTMLError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as errorSAXFunc;
        (*(*ctxt_1).sax).warning = Some(
            xmlHTMLWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as warningSAXFunc;
        (*ctxt_1).vctxt.error = Some(
            xmlHTMLValidityError
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityErrorFunc;
        (*ctxt_1).vctxt.warning = Some(
            xmlHTMLValidityWarning
                as unsafe extern "C" fn(
                    *mut c_void,
                    *const c_char,
                    ...
                ) -> (),
        ) as xmlValidityWarningFunc;
        doc = xmlCtxtReadFile(
            ctxt_1,
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        );
        if rectxt.is_null() {
            xmlFreeParserCtxt(ctxt_1);
        }
    } else if memory != 0 {
        let mut fd_0: c_int = 0;
        let mut info_0: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut base_0: *const c_char = ::core::ptr::null::<c_char>();
        if stat(filename, &raw mut info_0) < 0 as c_int {
            return;
        }
        fd_0 = open(filename, O_RDONLY);
        if fd_0 < 0 as c_int {
            return;
        }
        base_0 = mmap(
            NULL_0,
            info_0.st_size as size_t,
            PROT_READ,
            MAP_SHARED,
            fd_0,
            0 as __off64_t,
        ) as *const c_char;
        if base_0 == MAP_FAILED as *const c_char {
            close(fd_0);
            fprintf(
                stderr,
                b"mmap failure for file %s\n\0" as *const u8 as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_RDFILE;
            return;
        }
        if rectxt.is_null() {
            let mut ctxt_2: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
            ctxt_2 = xmlNewParserCtxt();
            if ctxt_2.is_null() {
                fprintf(
                    stderr,
                    b"out of memory\n\0" as *const u8 as *const c_char,
                );
                progresult = XMLLINT_ERR_MEM;
                return;
            }
            if maxAmpl > 0 as c_uint {
                xmlCtxtSetMaxAmplification(ctxt_2, maxAmpl);
            }
            doc = xmlCtxtReadMemory(
                ctxt_2,
                base_0,
                info_0.st_size as c_int,
                filename,
                ::core::ptr::null::<c_char>(),
                options,
            );
            xmlFreeParserCtxt(ctxt_2);
        } else {
            doc = xmlCtxtReadMemory(
                rectxt,
                base_0 as *mut c_char,
                info_0.st_size as c_int,
                filename,
                ::core::ptr::null::<c_char>(),
                options,
            );
        }
        munmap(
            base_0 as *mut c_char as *mut c_void,
            info_0.st_size as size_t,
        );
        close(fd_0);
    } else if valid != 0 {
        let mut ctxt_3: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
        if rectxt.is_null() {
            ctxt_3 = xmlNewParserCtxt();
            if ctxt_3.is_null() {
                progresult = XMLLINT_ERR_MEM;
                return;
            }
        } else {
            ctxt_3 = rectxt;
        }
        if maxAmpl > 0 as c_uint {
            xmlCtxtSetMaxAmplification(ctxt_3, maxAmpl);
        }
        doc = xmlCtxtReadFile(
            ctxt_3,
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        );
        if (*ctxt_3).valid == 0 as c_int {
            progresult = XMLLINT_ERR_RDFILE;
        }
        if rectxt.is_null() {
            xmlFreeParserCtxt(ctxt_3);
        }
    } else if !rectxt.is_null() {
        doc = xmlCtxtReadFile(
            rectxt,
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        );
    } else {
        let mut ctxt_4: xmlParserCtxtPtr = ::core::ptr::null_mut::<xmlParserCtxt>();
        ctxt_4 = xmlNewParserCtxt();
        if ctxt_4.is_null() {
            fprintf(
                stderr,
                b"out of memory\n\0" as *const u8 as *const c_char,
            );
            progresult = XMLLINT_ERR_MEM;
            return;
        }
        if maxAmpl > 0 as c_uint {
            xmlCtxtSetMaxAmplification(ctxt_4, maxAmpl);
        }
        doc = xmlCtxtReadFile(
            ctxt_4,
            filename,
            ::core::ptr::null::<c_char>(),
            options,
        );
        xmlFreeParserCtxt(ctxt_4);
    }
    if doc.is_null() {
        progresult = XMLLINT_ERR_UNCLASS;
        return;
    }
    if timing != 0 && repeat == 0 {
        endTimer(b"Parsing\0" as *const u8 as *const c_char);
    }
    if dropdtd != 0 {
        let mut dtd: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
        dtd = xmlGetIntSubset(doc as *const xmlDoc);
        if !dtd.is_null() {
            xmlUnlinkNode(dtd as xmlNodePtr);
            (*doc).intSubset = ::core::ptr::null_mut::<_xmlDtd>();
            xmlFreeDtd(dtd);
        }
    }
    if xinclude != 0 {
        if timing != 0 && repeat == 0 {
            startTimer();
        }
        if xmlXIncludeProcessFlags(doc, options) < 0 as c_int {
            progresult = XMLLINT_ERR_UNCLASS;
        }
        if timing != 0 && repeat == 0 {
            endTimer(b"Xinclude processing\0" as *const u8 as *const c_char);
        }
    }
    if !xpathquery.is_null() {
        doXPathQuery(doc, xpathquery);
    }
    if shell != 0 {
        xmlXPathOrderDocElems(doc);
        xmlShell(
            doc,
            filename,
            Some(
                xmlShellReadline
                    as unsafe extern "C" fn(*mut c_char) -> *mut c_char,
            ),
            stdout,
        );
    }
    if copy != 0 {
        tmp = doc;
        if timing != 0 {
            startTimer();
        }
        doc = xmlCopyDoc(doc, 1 as c_int);
        if doc.is_null() {
            progresult = XMLLINT_ERR_MEM;
            xmlFreeDoc(tmp);
            return;
        }
        if timing != 0 {
            endTimer(b"Copying\0" as *const u8 as *const c_char);
        }
        if timing != 0 {
            startTimer();
        }
        xmlFreeDoc(tmp);
        if timing != 0 {
            endTimer(b"Freeing original\0" as *const u8 as *const c_char);
        }
    }
    if insert != 0 && html == 0 {
        let mut list: [*const xmlChar; 256] = [::core::ptr::null::<xmlChar>(); 256];
        let mut nb: c_int = 0;
        let mut i: c_int = 0;
        let mut node: xmlNodePtr = ::core::ptr::null_mut::<xmlNode>();
        if !(*doc).children.is_null() {
            node = (*doc).children as xmlNodePtr;
            while !node.is_null() && (*node).last.is_null() {
                node = (*node).next as xmlNodePtr;
            }
            if !node.is_null() {
                nb = xmlValidGetValidElements(
                    (*node).last as *mut xmlNode,
                    ::core::ptr::null_mut::<xmlNode>(),
                    &raw mut list as *mut *const xmlChar,
                    256 as c_int,
                );
                if nb < 0 as c_int {
                    fprintf(
                        stderr,
                        b"could not get valid list of elements\n\0" as *const u8
                            as *const c_char,
                    );
                } else if nb == 0 as c_int {
                    fprintf(
                        stderr,
                        b"No element can be inserted under root\n\0" as *const u8
                            as *const c_char,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"%d element types can be inserted under root:\n\0" as *const u8
                            as *const c_char,
                        nb,
                    );
                    i = 0 as c_int;
                    while i < nb {
                        fprintf(
                            stderr,
                            b"%s\n\0" as *const u8 as *const c_char,
                            list[i as usize] as *mut c_char,
                        );
                        i += 1;
                    }
                }
            }
        }
    } else if walker != 0 {
        walkDoc(doc);
    }
    if noout == 0 as c_int {
        let mut ret_0: c_int = 0;
        if debug == 0 {
            if timing != 0 && repeat == 0 {
                startTimer();
            }
            if html != 0 && xmlout == 0 {
                if compress != 0 {
                    htmlSaveFile(
                        if !output.is_null() {
                            output
                        } else {
                            b"-\0" as *const u8 as *const c_char
                        },
                        doc,
                    );
                } else if !encoding.is_null() {
                    if format == 1 as c_int {
                        htmlSaveFileFormat(
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const c_char
                            },
                            doc,
                            encoding,
                            1 as c_int,
                        );
                    } else {
                        htmlSaveFileFormat(
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const c_char
                            },
                            doc,
                            encoding,
                            0 as c_int,
                        );
                    }
                } else if format == 1 as c_int {
                    htmlSaveFileFormat(
                        if !output.is_null() {
                            output
                        } else {
                            b"-\0" as *const u8 as *const c_char
                        },
                        doc,
                        ::core::ptr::null::<c_char>(),
                        1 as c_int,
                    );
                } else {
                    let mut out: *mut FILE = ::core::ptr::null_mut::<FILE>();
                    if output.is_null() {
                        out = stdout;
                    } else {
                        out = fopen(output, b"wb\0" as *const u8 as *const c_char);
                    }
                    if !out.is_null() {
                        if htmlDocDump(out, doc) < 0 as c_int {
                            progresult = XMLLINT_ERR_OUT;
                        }
                        if !output.is_null() {
                            fclose(out);
                        }
                    } else {
                        fprintf(
                            stderr,
                            b"failed to open %s\n\0" as *const u8 as *const c_char,
                            output,
                        );
                        progresult = XMLLINT_ERR_OUT;
                    }
                }
                if timing != 0 && repeat == 0 {
                    endTimer(b"Saving\0" as *const u8 as *const c_char);
                }
            } else if canonical != 0 {
                let mut result: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                let mut size_0: c_int = 0;
                size_0 = xmlC14NDocDumpMemory(
                    doc,
                    ::core::ptr::null_mut::<xmlNodeSet>(),
                    XML_C14N_1_0 as c_int,
                    ::core::ptr::null_mut::<*mut xmlChar>(),
                    1 as c_int,
                    &raw mut result,
                );
                if size_0 >= 0 as c_int {
                    if write(
                        1 as c_int,
                        result as *const c_void,
                        size_0 as size_t,
                    ) == -(1 as c_int) as c_long
                    {
                        fprintf(
                            stderr,
                            b"Can't write data\n\0" as *const u8 as *const c_char,
                        );
                    }
                    xmlFree.expect("non-null function pointer")(result as *mut c_void);
                } else {
                    fprintf(
                        stderr,
                        b"Failed to canonicalize\n\0" as *const u8 as *const c_char,
                    );
                    progresult = XMLLINT_ERR_OUT;
                }
            } else if canonical_11 != 0 {
                let mut result_0: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                let mut size_1: c_int = 0;
                size_1 = xmlC14NDocDumpMemory(
                    doc,
                    ::core::ptr::null_mut::<xmlNodeSet>(),
                    XML_C14N_1_1 as c_int,
                    ::core::ptr::null_mut::<*mut xmlChar>(),
                    1 as c_int,
                    &raw mut result_0,
                );
                if size_1 >= 0 as c_int {
                    if write(
                        1 as c_int,
                        result_0 as *const c_void,
                        size_1 as size_t,
                    ) == -(1 as c_int) as c_long
                    {
                        fprintf(
                            stderr,
                            b"Can't write data\n\0" as *const u8 as *const c_char,
                        );
                    }
                    xmlFree.expect("non-null function pointer")(
                        result_0 as *mut c_void,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"Failed to canonicalize\n\0" as *const u8 as *const c_char,
                    );
                    progresult = XMLLINT_ERR_OUT;
                }
            } else if exc_canonical != 0 {
                let mut result_1: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                let mut size_2: c_int = 0;
                size_2 = xmlC14NDocDumpMemory(
                    doc,
                    ::core::ptr::null_mut::<xmlNodeSet>(),
                    XML_C14N_EXCLUSIVE_1_0 as c_int,
                    ::core::ptr::null_mut::<*mut xmlChar>(),
                    1 as c_int,
                    &raw mut result_1,
                );
                if size_2 >= 0 as c_int {
                    if write(
                        1 as c_int,
                        result_1 as *const c_void,
                        size_2 as size_t,
                    ) == -(1 as c_int) as c_long
                    {
                        fprintf(
                            stderr,
                            b"Can't write data\n\0" as *const u8 as *const c_char,
                        );
                    }
                    xmlFree.expect("non-null function pointer")(
                        result_1 as *mut c_void,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"Failed to canonicalize\n\0" as *const u8 as *const c_char,
                    );
                    progresult = XMLLINT_ERR_OUT;
                }
            } else if memory != 0 {
                let mut result_2: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
                let mut len: c_int = 0;
                if !encoding.is_null() {
                    if format == 1 as c_int {
                        xmlDocDumpFormatMemoryEnc(
                            doc,
                            &raw mut result_2,
                            &raw mut len,
                            encoding,
                            1 as c_int,
                        );
                    } else {
                        xmlDocDumpMemoryEnc(doc, &raw mut result_2, &raw mut len, encoding);
                    }
                } else if format == 1 as c_int {
                    xmlDocDumpFormatMemory(
                        doc,
                        &raw mut result_2,
                        &raw mut len,
                        1 as c_int,
                    );
                } else {
                    xmlDocDumpMemory(doc, &raw mut result_2, &raw mut len);
                }
                if result_2.is_null() {
                    fprintf(
                        stderr,
                        b"Failed to save\n\0" as *const u8 as *const c_char,
                    );
                    progresult = XMLLINT_ERR_OUT;
                } else {
                    if write(
                        1 as c_int,
                        result_2 as *const c_void,
                        len as size_t,
                    ) == -(1 as c_int) as c_long
                    {
                        fprintf(
                            stderr,
                            b"Can't write data\n\0" as *const u8 as *const c_char,
                        );
                    }
                    xmlFree.expect("non-null function pointer")(
                        result_2 as *mut c_void,
                    );
                }
            } else if compress != 0 {
                xmlSaveFile(
                    if !output.is_null() {
                        output
                    } else {
                        b"-\0" as *const u8 as *const c_char
                    },
                    doc,
                );
            } else if oldout != 0 {
                if !encoding.is_null() {
                    if format == 1 as c_int {
                        ret_0 = xmlSaveFormatFileEnc(
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const c_char
                            },
                            doc,
                            encoding,
                            1 as c_int,
                        );
                    } else {
                        ret_0 = xmlSaveFileEnc(
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const c_char
                            },
                            doc,
                            encoding,
                        );
                    }
                    if ret_0 < 0 as c_int {
                        fprintf(
                            stderr,
                            b"failed save to %s\n\0" as *const u8 as *const c_char,
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const c_char
                            },
                        );
                        progresult = XMLLINT_ERR_OUT;
                    }
                } else if format == 1 as c_int {
                    ret_0 = xmlSaveFormatFile(
                        if !output.is_null() {
                            output
                        } else {
                            b"-\0" as *const u8 as *const c_char
                        },
                        doc,
                        1 as c_int,
                    );
                    if ret_0 < 0 as c_int {
                        fprintf(
                            stderr,
                            b"failed save to %s\n\0" as *const u8 as *const c_char,
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const c_char
                            },
                        );
                        progresult = XMLLINT_ERR_OUT;
                    }
                } else {
                    let mut out_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
                    if output.is_null() {
                        out_0 = stdout;
                    } else {
                        out_0 = fopen(output, b"wb\0" as *const u8 as *const c_char);
                    }
                    if !out_0.is_null() {
                        if xmlDocDump(out_0, doc) < 0 as c_int {
                            progresult = XMLLINT_ERR_OUT;
                        }
                        if !output.is_null() {
                            fclose(out_0);
                        }
                    } else {
                        fprintf(
                            stderr,
                            b"failed to open %s\n\0" as *const u8 as *const c_char,
                            output,
                        );
                        progresult = XMLLINT_ERR_OUT;
                    }
                }
            } else {
                let mut ctxt_5: xmlSaveCtxtPtr = ::core::ptr::null_mut::<xmlSaveCtxt>();
                let mut saveOpts: c_int = 0 as c_int;
                if format == 1 as c_int {
                    saveOpts |= XML_SAVE_FORMAT as c_int;
                } else if format == 2 as c_int {
                    saveOpts |= XML_SAVE_WSNONSIG as c_int;
                }
                if xmlout != 0 {
                    saveOpts |= XML_SAVE_AS_XML as c_int;
                }
                if output.is_null() {
                    ctxt_5 = xmlSaveToFd(1 as c_int, encoding, saveOpts);
                } else {
                    ctxt_5 = xmlSaveToFilename(output, encoding, saveOpts);
                }
                if !ctxt_5.is_null() {
                    if xmlSaveDoc(ctxt_5, doc) < 0 as c_long {
                        fprintf(
                            stderr,
                            b"failed save to %s\n\0" as *const u8 as *const c_char,
                            if !output.is_null() {
                                output
                            } else {
                                b"-\0" as *const u8 as *const c_char
                            },
                        );
                        progresult = XMLLINT_ERR_OUT;
                    }
                    xmlSaveClose(ctxt_5);
                } else {
                    progresult = XMLLINT_ERR_OUT;
                }
            }
            if timing != 0 && repeat == 0 {
                endTimer(b"Saving\0" as *const u8 as *const c_char);
            }
        } else {
            let mut out_1: *mut FILE = ::core::ptr::null_mut::<FILE>();
            if output.is_null() {
                out_1 = stdout;
            } else {
                out_1 = fopen(output, b"wb\0" as *const u8 as *const c_char);
            }
            if !out_1.is_null() {
                xmlDebugDumpDocument(out_1, doc);
                if !output.is_null() {
                    fclose(out_1);
                }
            } else {
                fprintf(
                    stderr,
                    b"failed to open %s\n\0" as *const u8 as *const c_char,
                    output,
                );
                progresult = XMLLINT_ERR_OUT;
            }
        }
    }
    if !dtdvalid.is_null() || !dtdvalidfpi.is_null() {
        let mut dtd_0: xmlDtdPtr = ::core::ptr::null_mut::<xmlDtd>();
        if timing != 0 && repeat == 0 {
            startTimer();
        }
        if !dtdvalid.is_null() {
            dtd_0 = xmlParseDTD(::core::ptr::null::<xmlChar>(), dtdvalid as *const xmlChar);
        } else {
            dtd_0 = xmlParseDTD(
                dtdvalidfpi as *const xmlChar,
                ::core::ptr::null::<xmlChar>(),
            );
        }
        if timing != 0 && repeat == 0 {
            endTimer(b"Parsing DTD\0" as *const u8 as *const c_char);
        }
        if dtd_0.is_null() {
            if !dtdvalid.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Could not parse DTD %s\n\0" as *const u8 as *const c_char,
                    dtdvalid,
                );
            } else {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Could not parse DTD %s\n\0" as *const u8 as *const c_char,
                    dtdvalidfpi,
                );
            }
            progresult = XMLLINT_ERR_DTD;
        } else {
            let mut cvp: xmlValidCtxtPtr = ::core::ptr::null_mut::<xmlValidCtxt>();
            cvp = xmlNewValidCtxt();
            if cvp.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Couldn't allocate validation context\n\0" as *const u8
                        as *const c_char,
                );
                progresult = XMLLINT_ERR_MEM;
                xmlFreeDtd(dtd_0);
                return;
            }
            (*cvp).error = *__xmlGenericError() as xmlValidityErrorFunc;
            (*cvp).warning = *__xmlGenericError() as xmlValidityWarningFunc;
            if timing != 0 && repeat == 0 {
                startTimer();
            }
            if xmlValidateDtd(cvp, doc, dtd_0) == 0 {
                if !dtdvalid.is_null() {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Document %s does not validate against %s\n\0" as *const u8
                            as *const c_char,
                        filename,
                        dtdvalid,
                    );
                } else {
                    (*__xmlGenericError()).expect("non-null function pointer")(
                        *__xmlGenericErrorContext(),
                        b"Document %s does not validate against %s\n\0" as *const u8
                            as *const c_char,
                        filename,
                        dtdvalidfpi,
                    );
                }
                progresult = XMLLINT_ERR_VALID;
            }
            if timing != 0 && repeat == 0 {
                endTimer(b"Validating against DTD\0" as *const u8 as *const c_char);
            }
            xmlFreeValidCtxt(cvp);
            xmlFreeDtd(dtd_0);
        }
    } else if postvalid != 0 {
        let mut cvp_0: xmlValidCtxtPtr = ::core::ptr::null_mut::<xmlValidCtxt>();
        cvp_0 = xmlNewValidCtxt();
        if cvp_0.is_null() {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Couldn't allocate validation context\n\0" as *const u8
                    as *const c_char,
            );
            progresult = XMLLINT_ERR_MEM;
            xmlFreeDoc(doc);
            return;
        }
        if timing != 0 && repeat == 0 {
            startTimer();
        }
        (*cvp_0).error = *__xmlGenericError() as xmlValidityErrorFunc;
        (*cvp_0).warning = *__xmlGenericError() as xmlValidityWarningFunc;
        if xmlValidateDocument(cvp_0, doc) == 0 {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Document %s does not validate\n\0" as *const u8 as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_VALID;
        }
        if timing != 0 && repeat == 0 {
            endTimer(b"Validating\0" as *const u8 as *const c_char);
        }
        xmlFreeValidCtxt(cvp_0);
    }
    if !wxschematron.is_null() {
        let mut ctxt_6: xmlSchematronValidCtxtPtr =
            ::core::ptr::null_mut::<xmlSchematronValidCtxt>();
        let mut ret_1: c_int = 0;
        let mut flag: c_int = 0;
        if timing != 0 && repeat == 0 {
            startTimer();
        }
        if debug != 0 {
            flag = XML_SCHEMATRON_OUT_XML as c_int;
        } else {
            flag = XML_SCHEMATRON_OUT_TEXT as c_int;
        }
        if noout != 0 {
            flag |= XML_SCHEMATRON_OUT_QUIET as c_int;
        }
        ctxt_6 = xmlSchematronNewValidCtxt(wxschematron, flag);
        if ctxt_6.is_null() {
            progresult = XMLLINT_ERR_MEM;
            xmlFreeDoc(doc);
            return;
        }
        ret_1 = xmlSchematronValidateDoc(ctxt_6, doc);
        if ret_1 == 0 as c_int {
            if quiet == 0 {
                fprintf(
                    stderr,
                    b"%s validates\n\0" as *const u8 as *const c_char,
                    filename,
                );
            }
        } else if ret_1 > 0 as c_int {
            fprintf(
                stderr,
                b"%s fails to validate\n\0" as *const u8 as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_VALID;
        } else {
            fprintf(
                stderr,
                b"%s validation generated an internal error\n\0" as *const u8
                    as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_VALID;
        }
        xmlSchematronFreeValidCtxt(ctxt_6);
        if timing != 0 && repeat == 0 {
            endTimer(b"Validating\0" as *const u8 as *const c_char);
        }
    }
    if !relaxngschemas.is_null() {
        let mut ctxt_7: xmlRelaxNGValidCtxtPtr = ::core::ptr::null_mut::<xmlRelaxNGValidCtxt>();
        let mut ret_2: c_int = 0;
        if timing != 0 && repeat == 0 {
            startTimer();
        }
        ctxt_7 = xmlRelaxNGNewValidCtxt(relaxngschemas);
        if ctxt_7.is_null() {
            progresult = XMLLINT_ERR_MEM;
            xmlFreeDoc(doc);
            return;
        }
        xmlRelaxNGSetValidErrors(ctxt_7, *__xmlGenericError(), *__xmlGenericError(), NULL_0);
        ret_2 = xmlRelaxNGValidateDoc(ctxt_7, doc);
        if ret_2 == 0 as c_int {
            if quiet == 0 {
                fprintf(
                    stderr,
                    b"%s validates\n\0" as *const u8 as *const c_char,
                    filename,
                );
            }
        } else if ret_2 > 0 as c_int {
            fprintf(
                stderr,
                b"%s fails to validate\n\0" as *const u8 as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_VALID;
        } else {
            fprintf(
                stderr,
                b"%s validation generated an internal error\n\0" as *const u8
                    as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_VALID;
        }
        xmlRelaxNGFreeValidCtxt(ctxt_7);
        if timing != 0 && repeat == 0 {
            endTimer(b"Validating\0" as *const u8 as *const c_char);
        }
    } else if !wxschemas.is_null() {
        let mut ctxt_8: xmlSchemaValidCtxtPtr = ::core::ptr::null_mut::<xmlSchemaValidCtxt>();
        let mut ret_3: c_int = 0;
        if timing != 0 && repeat == 0 {
            startTimer();
        }
        ctxt_8 = xmlSchemaNewValidCtxt(wxschemas);
        if ctxt_8.is_null() {
            progresult = XMLLINT_ERR_MEM;
            xmlFreeDoc(doc);
            return;
        }
        xmlSchemaSetValidErrors(ctxt_8, *__xmlGenericError(), *__xmlGenericError(), NULL_0);
        ret_3 = xmlSchemaValidateDoc(ctxt_8, doc);
        if ret_3 == 0 as c_int {
            if quiet == 0 {
                fprintf(
                    stderr,
                    b"%s validates\n\0" as *const u8 as *const c_char,
                    filename,
                );
            }
        } else if ret_3 > 0 as c_int {
            fprintf(
                stderr,
                b"%s fails to validate\n\0" as *const u8 as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_VALID;
        } else {
            fprintf(
                stderr,
                b"%s validation generated an internal error\n\0" as *const u8
                    as *const c_char,
                filename,
            );
            progresult = XMLLINT_ERR_VALID;
        }
        xmlSchemaFreeValidCtxt(ctxt_8);
        if timing != 0 && repeat == 0 {
            endTimer(b"Validating\0" as *const u8 as *const c_char);
        }
    }
    if debugent != 0 && html == 0 {
        xmlDebugDumpEntities(stderr, doc);
    }
    if timing != 0 && repeat == 0 {
        startTimer();
    }
    xmlFreeDoc(doc);
    if timing != 0 && repeat == 0 {
        endTimer(b"Freeing\0" as *const u8 as *const c_char);
    }
}
unsafe extern "C" fn showVersion(mut name: *const c_char) {
    fprintf(
        stderr,
        b"%s: using libxml version %s\n\0" as *const u8 as *const c_char,
        name,
        xmlParserVersion,
    );
    fprintf(
        stderr,
        b"   compiled with: \0" as *const u8 as *const c_char,
    );
    if xmlHasFeature(XML_WITH_THREAD) != 0 {
        fprintf(
            stderr,
            b"Threads \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_TREE) != 0 {
        fprintf(
            stderr,
            b"Tree \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_OUTPUT) != 0 {
        fprintf(
            stderr,
            b"Output \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_PUSH) != 0 {
        fprintf(
            stderr,
            b"Push \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_READER) != 0 {
        fprintf(
            stderr,
            b"Reader \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_PATTERN) != 0 {
        fprintf(
            stderr,
            b"Patterns \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_WRITER) != 0 {
        fprintf(
            stderr,
            b"Writer \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_SAX1) != 0 {
        fprintf(
            stderr,
            b"SAXv1 \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_FTP) != 0 {
        fprintf(stderr, b"FTP \0" as *const u8 as *const c_char);
    }
    if xmlHasFeature(XML_WITH_HTTP) != 0 {
        fprintf(
            stderr,
            b"HTTP \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_VALID) != 0 {
        fprintf(
            stderr,
            b"DTDValid \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_HTML) != 0 {
        fprintf(
            stderr,
            b"HTML \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_LEGACY) != 0 {
        fprintf(
            stderr,
            b"Legacy \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_C14N) != 0 {
        fprintf(
            stderr,
            b"C14N \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_CATALOG) != 0 {
        fprintf(
            stderr,
            b"Catalog \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_XPATH) != 0 {
        fprintf(
            stderr,
            b"XPath \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_XPTR) != 0 {
        fprintf(
            stderr,
            b"XPointer \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_XINCLUDE) != 0 {
        fprintf(
            stderr,
            b"XInclude \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_ICONV) != 0 {
        fprintf(
            stderr,
            b"Iconv \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_ICU) != 0 {
        fprintf(stderr, b"ICU \0" as *const u8 as *const c_char);
    }
    if xmlHasFeature(XML_WITH_ISO8859X) != 0 {
        fprintf(
            stderr,
            b"ISO8859X \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_UNICODE) != 0 {
        fprintf(
            stderr,
            b"Unicode \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_REGEXP) != 0 {
        fprintf(
            stderr,
            b"Regexps \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_AUTOMATA) != 0 {
        fprintf(
            stderr,
            b"Automata \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_EXPR) != 0 {
        fprintf(
            stderr,
            b"Expr \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_SCHEMAS) != 0 {
        fprintf(
            stderr,
            b"Schemas \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_SCHEMATRON) != 0 {
        fprintf(
            stderr,
            b"Schematron \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_MODULES) != 0 {
        fprintf(
            stderr,
            b"Modules \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_DEBUG) != 0 {
        fprintf(
            stderr,
            b"Debug \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_DEBUG_MEM) != 0 {
        fprintf(
            stderr,
            b"MemDebug \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_DEBUG_RUN) != 0 {
        fprintf(
            stderr,
            b"RunDebug \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_ZLIB) != 0 {
        fprintf(
            stderr,
            b"Zlib \0" as *const u8 as *const c_char,
        );
    }
    if xmlHasFeature(XML_WITH_LZMA) != 0 {
        fprintf(
            stderr,
            b"Lzma \0" as *const u8 as *const c_char,
        );
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const c_char);
}
unsafe extern "C" fn usage(mut f: *mut FILE, mut name: *const c_char) {
    fprintf(
        f,
        b"Usage : %s [options] XMLfiles ...\n\0" as *const u8 as *const c_char,
        name,
    );
    fprintf(
        f,
        b"\tParse the XML files and output the result of the parsing\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--version : display the version of the XML library used\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--debug : dump a debug tree of the in-memory document\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--shell : run a navigating shell\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--debugent : debug the entities defined in the document\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--copy : used to test the internal copy implementation\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--recover : output what was parsable on broken XML documents\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--huge : remove any internal arbitrary parser limits\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--noent : substitute entity references by their value\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--noenc : ignore any encoding specified inside the document\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--noout : don't output the result tree\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--path 'paths': provide a set of paths for resources\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--load-trace : print trace of all external entities loaded\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--nonet : refuse to fetch DTDs or entities over network\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--nocompact : do not generate compact text nodes\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--htmlout : output results as HTML\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--nowrap : do not put HTML doc wrapper\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--valid : validate the document in addition to std well-formed check\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--postvalid : do a posteriori validation, i.e after parsing\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--dtdvalid URL : do a posteriori validation against a given DTD\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--dtdvalidfpi FPI : same but name the DTD with a Public Identifier\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--quiet : be quiet when succeeded\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--timing : print some timings\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--output file or -o file: save to a given file\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--repeat : repeat 100 times, for timing or profiling\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--insert : ad-hoc test for valid insertions\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--html : use the HTML parser\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--xmlout : force to use the XML serializer when using --html\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--nodefdtd : do not default HTML doctype\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--push : use the push mode of the parser\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--pushsmall : use the push mode of the parser using tiny increments\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--memory : parse from memory\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--maxmem nbbytes : limits memory allocation to nbbytes bytes\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--nowarning : do not emit warnings from parser/validator\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--noblanks : drop (ignorable?) blanks spaces\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--nocdata : replace cdata section with text nodes\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--format : reformat/reindent the output\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--encode encoding : output in the given encoding\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--dropdtd : remove the DOCTYPE of the input docs\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--pretty STYLE : pretty-print in a particular style\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t                 0 Do not pretty print\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t                 1 Format the XML content, as --format\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t                 2 Add whitespace inside tags, preserving content\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--c14n : save in W3C canonical format v1.0 (with comments)\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--c14n11 : save in W3C canonical format v1.1 (with comments)\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--exc-c14n : save in W3C exclusive canonical format (with comments)\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--nsclean : remove redundant namespace declarations\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--testIO : test user I/O support\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--catalogs : use SGML catalogs from $SGML_CATALOG_FILES\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t             otherwise XML Catalogs starting from \n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t         %s are activated by default\n\0" as *const u8 as *const c_char,
        XML_XML_DEFAULT_CATALOG.as_ptr(),
    );
    fprintf(
        f,
        b"\t--nocatalogs: deactivate all catalogs\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--auto : generate a small doc on the fly\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--xinclude : do XInclude processing\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--noxincludenode : same but do not generate XInclude nodes\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--nofixup-base-uris : do not fixup xml:base uris\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--loaddtd : fetch external DTD\n\0" as *const u8 as *const c_char,
    );
    fprintf(
        f,
        b"\t--dtdattr : loaddtd + populate the tree with inherited attributes \n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--stream : use the streaming interface to process very large files\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--walker : create a reader and walk though the resulting doc\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--pattern pattern_value : test the pattern support\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--chkregister : verify the node registration code\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--relaxng schema : do RelaxNG validation against the schema\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--schema schema : do validation against the WXS schema\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--schematron schema : do validation against a schematron\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--sax1: use the old SAX1 interfaces for processing\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--sax: do not build a tree but work just at the SAX level\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--oldxml10: use XML-1.0 parsing rules before the 5th edition\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--xpath expr: evaluate the XPath expression, imply --noout\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\t--max-ampl value: set maximum amplification factor\n\0" as *const u8
            as *const c_char,
    );
    fprintf(
        f,
        b"\nLibxml project home page: https://gitlab.gnome.org/GNOME/libxml2\n\0" as *const u8
            as *const c_char,
    );
}
extern "C" fn registerNode(mut node: xmlNodePtr) { unsafe {
    (*node)._private = malloc(::core::mem::size_of::<c_long>() as size_t);
    if (*node)._private.is_null() {
        fprintf(
            stderr,
            b"Out of memory in xmllint:registerNode()\n\0" as *const u8
                as *const c_char,
        );
        exit(XMLLINT_ERR_MEM as c_int);
    }
    *((*node)._private as *mut c_long) =
        0x81726354 as c_uint as c_long;
    nbregister += 1;
} }
extern "C" fn deregisterNode(mut node: xmlNodePtr) { unsafe {
    free((*node)._private);
    nbregister -= 1;
} }
unsafe extern "C" fn parseInteger(
    mut ctxt: *const c_char,
    mut str: *const c_char,
    mut min: c_ulong,
    mut max: c_ulong,
) -> c_ulong {
    let mut strEnd: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut val: c_ulong = 0;
    *__errno_location() = 0 as c_int;
    val = strtoul(str, &raw mut strEnd, 10 as c_int);
    if *__errno_location() == EINVAL || *strEnd as c_int != 0 as c_int {
        fprintf(
            stderr,
            b"%s: invalid integer: %s\n\0" as *const u8 as *const c_char,
            ctxt,
            str,
        );
        exit(XMLLINT_ERR_UNCLASS as c_int);
    }
    if *__errno_location() != 0 as c_int || val < min || val > max {
        fprintf(
            stderr,
            b"%s: integer out of range: %s\n\0" as *const u8 as *const c_char,
            ctxt,
            str,
        );
        exit(XMLLINT_ERR_UNCLASS as c_int);
    }
    return val;
}
unsafe fn main_0(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) -> c_int {
    let mut current_block: u64;
    let mut i: c_int = 0;
    let mut acount: c_int = 0;
    let mut files: c_int = 0 as c_int;
    let mut version: c_int = 0 as c_int;
    if argc <= 1 as c_int {
        usage(stderr, *argv.offset(0 as c_int as isize));
        return XMLLINT_ERR_UNCLASS as c_int;
    }
    i = 1 as c_int;
    while i < argc {
        if !(*(*argv.offset(i as isize)).offset(0 as c_int as isize)
            as c_int
            != '-' as i32)
        {
            if strcmp(
                *argv.offset(i as isize),
                b"-maxmem\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--maxmem\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                if i >= argc {
                    fprintf(
                        stderr,
                        b"maxmem: missing integer value\n\0" as *const u8
                            as *const c_char,
                    );
                    return XMLLINT_ERR_UNCLASS as c_int;
                }
                *__errno_location() = 0 as c_int;
                maxmem = parseInteger(
                    b"maxmem\0" as *const u8 as *const c_char,
                    *argv.offset(i as isize),
                    0 as c_ulong,
                    INT_MAX as c_ulong,
                ) as c_int;
            }
        }
        i += 1;
    }
    if maxmem != 0 as c_int {
        xmlMemSetup(
            Some(myFreeFunc as unsafe extern "C" fn(*mut c_void) -> ()),
            Some(myMallocFunc as unsafe extern "C" fn(size_t) -> *mut c_void),
            Some(
                myReallocFunc
                    as unsafe extern "C" fn(
                        *mut c_void,
                        size_t,
                    ) -> *mut c_void,
            ),
            Some(
                myStrdupFunc
                    as unsafe extern "C" fn(*const c_char) -> *mut c_char,
            ),
        );
    }
    xmlCheckVersion(21205 as c_int);
    i = 1 as c_int;
    while i < argc {
        if !(*(*argv.offset(i as isize)).offset(0 as c_int as isize)
            as c_int
            != '-' as i32
            || *(*argv.offset(i as isize)).offset(1 as c_int as isize)
                as c_int
                == 0 as c_int)
        {
            if strcmp(
                *argv.offset(i as isize),
                b"-debug\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--debug\0" as *const u8 as *const c_char,
                ) == 0
            {
                debug += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-shell\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--shell\0" as *const u8 as *const c_char,
                ) == 0
            {
                shell += 1;
                noout = 1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-copy\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--copy\0" as *const u8 as *const c_char,
                ) == 0
            {
                copy += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-recover\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--recover\0" as *const u8 as *const c_char,
                ) == 0
            {
                recovery += 1;
                options |= XML_PARSE_RECOVER as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-huge\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--huge\0" as *const u8 as *const c_char,
                ) == 0
            {
                options |= XML_PARSE_HUGE as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noent\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noent\0" as *const u8 as *const c_char,
                ) == 0
            {
                noent = 1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noenc\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noenc\0" as *const u8 as *const c_char,
                ) == 0
            {
                noenc += 1;
                options |= XML_PARSE_IGNORE_ENC as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nsclean\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nsclean\0" as *const u8 as *const c_char,
                ) == 0
            {
                options |= XML_PARSE_NSCLEAN as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nocdata\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nocdata\0" as *const u8 as *const c_char,
                ) == 0
            {
                options |= XML_PARSE_NOCDATA as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nodict\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nodict\0" as *const u8 as *const c_char,
                ) == 0
            {
                options |= XML_PARSE_NODICT as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-version\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--version\0" as *const u8 as *const c_char,
                ) == 0
            {
                showVersion(*argv.offset(0 as c_int as isize));
                version = 1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noout\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noout\0" as *const u8 as *const c_char,
                ) == 0
            {
                noout += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-o\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"-output\0" as *const u8 as *const c_char,
                ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--output\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                output = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-htmlout\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--htmlout\0" as *const u8 as *const c_char,
                ) == 0
            {
                htmlout += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nowrap\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nowrap\0" as *const u8 as *const c_char,
                ) == 0
            {
                nowrap += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-html\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--html\0" as *const u8 as *const c_char,
                ) == 0
            {
                html += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-xmlout\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--xmlout\0" as *const u8 as *const c_char,
                ) == 0
            {
                xmlout += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nodefdtd\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nodefdtd\0" as *const u8 as *const c_char,
                ) == 0
            {
                nodefdtd += 1;
                options |= HTML_PARSE_NODEFDTD as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-loaddtd\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--loaddtd\0" as *const u8 as *const c_char,
                ) == 0
            {
                loaddtd += 1;
                options |= XML_PARSE_DTDLOAD as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-dtdattr\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dtdattr\0" as *const u8 as *const c_char,
                ) == 0
            {
                loaddtd += 1;
                dtdattrs += 1;
                options |= XML_PARSE_DTDATTR as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-valid\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--valid\0" as *const u8 as *const c_char,
                ) == 0
            {
                valid += 1;
                options |= XML_PARSE_DTDVALID as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-postvalid\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--postvalid\0" as *const u8 as *const c_char,
                ) == 0
            {
                postvalid += 1;
                loaddtd += 1;
                options |= XML_PARSE_DTDLOAD as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-dtdvalid\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dtdvalid\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                dtdvalid = *argv.offset(i as isize);
                loaddtd += 1;
                options |= XML_PARSE_DTDLOAD as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-dtdvalidfpi\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dtdvalidfpi\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                dtdvalidfpi = *argv.offset(i as isize);
                loaddtd += 1;
                options |= XML_PARSE_DTDLOAD as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-dropdtd\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--dropdtd\0" as *const u8 as *const c_char,
                ) == 0
            {
                dropdtd += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-insert\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--insert\0" as *const u8 as *const c_char,
                ) == 0
            {
                insert += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-quiet\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--quiet\0" as *const u8 as *const c_char,
                ) == 0
            {
                quiet += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-timing\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--timing\0" as *const u8 as *const c_char,
                ) == 0
            {
                timing += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-auto\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--auto\0" as *const u8 as *const c_char,
                ) == 0
            {
                generate += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-repeat\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--repeat\0" as *const u8 as *const c_char,
                ) == 0
            {
                if repeat != 0 {
                    repeat *= 10 as c_int;
                } else {
                    repeat = 100 as c_int;
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-push\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--push\0" as *const u8 as *const c_char,
                ) == 0
            {
                push += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-pushsmall\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pushsmall\0" as *const u8 as *const c_char,
                ) == 0
            {
                push += 1;
                pushsize = 10 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-memory\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--memory\0" as *const u8 as *const c_char,
                ) == 0
            {
                memory += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-testIO\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--testIO\0" as *const u8 as *const c_char,
                ) == 0
            {
                testIO += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-xinclude\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--xinclude\0" as *const u8 as *const c_char,
                ) == 0
            {
                xinclude += 1;
                options |= XML_PARSE_XINCLUDE as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noxincludenode\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noxincludenode\0" as *const u8 as *const c_char,
                ) == 0
            {
                xinclude += 1;
                options |= XML_PARSE_XINCLUDE as c_int;
                options |= XML_PARSE_NOXINCNODE as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nofixup-base-uris\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nofixup-base-uris\0" as *const u8 as *const c_char,
                ) == 0
            {
                xinclude += 1;
                options |= XML_PARSE_XINCLUDE as c_int;
                options |= XML_PARSE_NOBASEFIX as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nowarning\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nowarning\0" as *const u8 as *const c_char,
                ) == 0
            {
                options |= XML_PARSE_NOWARNING as c_int;
                options &= !(XML_PARSE_PEDANTIC as c_int);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-pedantic\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pedantic\0" as *const u8 as *const c_char,
                ) == 0
            {
                options |= XML_PARSE_PEDANTIC as c_int;
                options &= XML_PARSE_NOWARNING as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-debugent\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--debugent\0" as *const u8 as *const c_char,
                ) == 0
            {
                debugent += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-c14n\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--c14n\0" as *const u8 as *const c_char,
                ) == 0
            {
                canonical += 1;
                options |= XML_PARSE_NOENT as c_int
                    | XML_PARSE_DTDATTR as c_int
                    | XML_PARSE_DTDLOAD as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-c14n11\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--c14n11\0" as *const u8 as *const c_char,
                ) == 0
            {
                canonical_11 += 1;
                options |= XML_PARSE_NOENT as c_int
                    | XML_PARSE_DTDATTR as c_int
                    | XML_PARSE_DTDLOAD as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-exc-c14n\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--exc-c14n\0" as *const u8 as *const c_char,
                ) == 0
            {
                exc_canonical += 1;
                options |= XML_PARSE_NOENT as c_int
                    | XML_PARSE_DTDATTR as c_int
                    | XML_PARSE_DTDLOAD as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-catalogs\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--catalogs\0" as *const u8 as *const c_char,
                ) == 0
            {
                catalogs += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nocatalogs\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nocatalogs\0" as *const u8 as *const c_char,
                ) == 0
            {
                nocatalogs += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-encode\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--encode\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                encoding = *argv.offset(i as isize);
                xmlAddEncodingAlias(
                    b"UTF-8\0" as *const u8 as *const c_char,
                    b"DVEnc\0" as *const u8 as *const c_char,
                );
            } else if strcmp(
                *argv.offset(i as isize),
                b"-noblanks\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--noblanks\0" as *const u8 as *const c_char,
                ) == 0
            {
                noblanks = 1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-maxmem\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--maxmem\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-format\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--format\0" as *const u8 as *const c_char,
                ) == 0
            {
                format = 1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-pretty\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pretty\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                if !(*argv.offset(i as isize)).is_null() {
                    format = atoi(*argv.offset(i as isize));
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-stream\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--stream\0" as *const u8 as *const c_char,
                ) == 0
            {
                stream += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-walker\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--walker\0" as *const u8 as *const c_char,
                ) == 0
            {
                walker += 1;
                noout += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-pattern\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--pattern\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                pattern = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-sax1\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--sax1\0" as *const u8 as *const c_char,
                ) == 0
            {
                sax1 += 1;
                options |= XML_PARSE_SAX1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-sax\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--sax\0" as *const u8 as *const c_char,
                ) == 0
            {
                sax += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-chkregister\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--chkregister\0" as *const u8 as *const c_char,
                ) == 0
            {
                chkregister += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-relaxng\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--relaxng\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                relaxng = *argv.offset(i as isize);
                noent = 1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-schema\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--schema\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                schema = *argv.offset(i as isize);
                noent = 1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-schematron\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--schematron\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                schematron = *argv.offset(i as isize);
                noent = 1 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nonet\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nonet\0" as *const u8 as *const c_char,
                ) == 0
            {
                options |= XML_PARSE_NONET as c_int;
                xmlSetExternalEntityLoader(Some(
                    xmlNoNetExternalEntityLoader
                        as unsafe extern "C" fn(
                            *const c_char,
                            *const c_char,
                            xmlParserCtxtPtr,
                        ) -> xmlParserInputPtr,
                ));
            } else if strcmp(
                *argv.offset(i as isize),
                b"-nocompact\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--nocompact\0" as *const u8 as *const c_char,
                ) == 0
            {
                options &= !(XML_PARSE_COMPACT as c_int);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-load-trace\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--load-trace\0" as *const u8 as *const c_char,
                ) == 0
            {
                load_trace += 1;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-path\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--path\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                parsePath(*argv.offset(i as isize) as *mut xmlChar);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-xpath\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--xpath\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                noout += 1;
                xpathquery = *argv.offset(i as isize);
            } else if strcmp(
                *argv.offset(i as isize),
                b"-oldxml10\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--oldxml10\0" as *const u8 as *const c_char,
                ) == 0
            {
                oldxml10 += 1;
                options |= XML_PARSE_OLD10 as c_int;
            } else if strcmp(
                *argv.offset(i as isize),
                b"-max-ampl\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    b"--max-ampl\0" as *const u8 as *const c_char,
                ) == 0
            {
                i += 1;
                if i >= argc {
                    fprintf(
                        stderr,
                        b"max-ampl: missing integer value\n\0" as *const u8
                            as *const c_char,
                    );
                    return XMLLINT_ERR_UNCLASS as c_int;
                }
                maxAmpl = parseInteger(
                    b"max-ampl\0" as *const u8 as *const c_char,
                    *argv.offset(i as isize),
                    1 as c_ulong,
                    UINT_MAX as c_ulong,
                ) as c_uint;
            } else {
                fprintf(
                    stderr,
                    b"Unknown option %s\n\0" as *const u8 as *const c_char,
                    *argv.offset(i as isize),
                );
                usage(stderr, *argv.offset(0 as c_int as isize));
                return XMLLINT_ERR_UNCLASS as c_int;
            }
        }
        i += 1;
    }
    if nocatalogs == 0 as c_int {
        if catalogs != 0 {
            let mut catal: *const c_char = ::core::ptr::null::<c_char>();
            catal = getenv(b"SGML_CATALOG_FILES\0" as *const u8 as *const c_char);
            if !catal.is_null() {
                xmlLoadCatalogs(catal);
            } else {
                fprintf(
                    stderr,
                    b"Variable $SGML_CATALOG_FILES not set\n\0" as *const u8
                        as *const c_char,
                );
            }
        }
    }
    if chkregister != 0 {
        xmlRegisterNodeDefault(Some(registerNode as unsafe extern "C" fn(xmlNodePtr) -> ()));
        xmlDeregisterNodeDefault(Some(
            deregisterNode as unsafe extern "C" fn(xmlNodePtr) -> (),
        ));
    }
    let mut indent: *const c_char =
        getenv(b"XMLLINT_INDENT\0" as *const u8 as *const c_char);
    if !indent.is_null() {
        let ref mut fresh7 = *__xmlTreeIndentString();
        *fresh7 = indent;
    }
    defaultEntityLoader = xmlGetExternalEntityLoader();
    xmlSetExternalEntityLoader(Some(
        xmllintExternalEntityLoader
            as unsafe extern "C" fn(
                *const c_char,
                *const c_char,
                xmlParserCtxtPtr,
            ) -> xmlParserInputPtr,
    ));
    if noent != 0 as c_int {
        options |= XML_PARSE_NOENT as c_int;
    }
    if noblanks != 0 as c_int || format == 1 as c_int {
        options |= XML_PARSE_NOBLANKS as c_int;
    }
    if htmlout != 0 && nowrap == 0 {
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0 Transitional//EN\"\n\0" as *const u8
                as *const c_char,
        );
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"\t\"http://www.w3.org/TR/REC-html40/loose.dtd\">\n\0" as *const u8
                as *const c_char,
        );
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"<html><head><title>%s output</title></head>\n\0" as *const u8
                as *const c_char,
            *argv.offset(0 as c_int as isize),
        );
        (*__xmlGenericError()).expect("non-null function pointer")(
            *__xmlGenericErrorContext(),
            b"<body bgcolor=\"#ffffff\"><h1 align=\"center\">%s output</h1>\n\0" as *const u8
                as *const c_char,
            *argv.offset(0 as c_int as isize),
        );
    }
    if !schematron.is_null() && sax == 0 as c_int && stream == 0 as c_int
    {
        let mut ctxt: xmlSchematronParserCtxtPtr =
            ::core::ptr::null_mut::<xmlSchematronParserCtxt>();
        options |= XML_PARSE_DTDLOAD as c_int;
        if timing != 0 {
            startTimer();
        }
        ctxt = xmlSchematronNewParserCtxt(schematron);
        if ctxt.is_null() {
            progresult = XMLLINT_ERR_MEM;
            current_block = 13714557355923613615;
        } else {
            wxschematron = xmlSchematronParse(ctxt);
            if wxschematron.is_null() {
                (*__xmlGenericError()).expect("non-null function pointer")(
                    *__xmlGenericErrorContext(),
                    b"Schematron schema %s failed to compile\n\0" as *const u8
                        as *const c_char,
                    schematron,
                );
                progresult = XMLLINT_ERR_SCHEMACOMP;
                schematron = ::core::ptr::null_mut::<c_char>();
            }
            xmlSchematronFreeParserCtxt(ctxt);
            if timing != 0 {
                endTimer(b"Compiling the schemas\0" as *const u8 as *const c_char);
            }
            current_block = 6838189980997685602;
        }
    } else {
        current_block = 6838189980997685602;
    }
    match current_block {
        6838189980997685602 => {
            if !relaxng.is_null()
                && sax == 0 as c_int
                && stream == 0 as c_int
            {
                let mut ctxt_0: xmlRelaxNGParserCtxtPtr =
                    ::core::ptr::null_mut::<xmlRelaxNGParserCtxt>();
                options |= XML_PARSE_DTDLOAD as c_int;
                if timing != 0 {
                    startTimer();
                }
                ctxt_0 = xmlRelaxNGNewParserCtxt(relaxng);
                if ctxt_0.is_null() {
                    progresult = XMLLINT_ERR_MEM;
                    current_block = 13714557355923613615;
                } else {
                    xmlRelaxNGSetParserErrors(
                        ctxt_0,
                        *__xmlGenericError(),
                        *__xmlGenericError(),
                        NULL_0,
                    );
                    relaxngschemas = xmlRelaxNGParse(ctxt_0);
                    if relaxngschemas.is_null() {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"Relax-NG schema %s failed to compile\n\0" as *const u8
                                as *const c_char,
                            relaxng,
                        );
                        progresult = XMLLINT_ERR_SCHEMACOMP;
                        relaxng = ::core::ptr::null_mut::<c_char>();
                    }
                    xmlRelaxNGFreeParserCtxt(ctxt_0);
                    if timing != 0 {
                        endTimer(
                            b"Compiling the schemas\0" as *const u8 as *const c_char,
                        );
                    }
                    current_block = 8483315232868171348;
                }
            } else if !schema.is_null() && stream == 0 as c_int {
                let mut ctxt_1: xmlSchemaParserCtxtPtr =
                    ::core::ptr::null_mut::<xmlSchemaParserCtxt>();
                if timing != 0 {
                    startTimer();
                }
                ctxt_1 = xmlSchemaNewParserCtxt(schema);
                if ctxt_1.is_null() {
                    progresult = XMLLINT_ERR_MEM;
                    current_block = 13714557355923613615;
                } else {
                    xmlSchemaSetParserErrors(
                        ctxt_1,
                        *__xmlGenericError(),
                        *__xmlGenericError(),
                        NULL_0,
                    );
                    wxschemas = xmlSchemaParse(ctxt_1);
                    if wxschemas.is_null() {
                        (*__xmlGenericError()).expect("non-null function pointer")(
                            *__xmlGenericErrorContext(),
                            b"WXS schema %s failed to compile\n\0" as *const u8
                                as *const c_char,
                            schema,
                        );
                        progresult = XMLLINT_ERR_SCHEMACOMP;
                        schema = ::core::ptr::null_mut::<c_char>();
                    }
                    xmlSchemaFreeParserCtxt(ctxt_1);
                    if timing != 0 {
                        endTimer(
                            b"Compiling the schemas\0" as *const u8 as *const c_char,
                        );
                    }
                    current_block = 8483315232868171348;
                }
            } else {
                current_block = 8483315232868171348;
            }
            match current_block {
                13714557355923613615 => {}
                _ => {
                    if !pattern.is_null() && walker == 0 as c_int {
                        patternc = xmlPatterncompile(
                            pattern as *const xmlChar,
                            ::core::ptr::null_mut::<xmlDict>(),
                            0 as c_int,
                            ::core::ptr::null_mut::<*const xmlChar>(),
                        );
                        if patternc.is_null() {
                            (*__xmlGenericError()).expect("non-null function pointer")(
                                *__xmlGenericErrorContext(),
                                b"Pattern %s failed to compile\n\0" as *const u8
                                    as *const c_char,
                                pattern,
                            );
                            progresult = XMLLINT_ERR_SCHEMAPAT;
                            pattern = ::core::ptr::null::<c_char>();
                        }
                    }
                    i = 1 as c_int;
                    loop {
                        if !(i < argc) {
                            current_block = 7235894238071384538;
                            break;
                        }
                        if strcmp(
                            *argv.offset(i as isize),
                            b"-encode\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--encode\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-o\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"-output\0" as *const u8 as *const c_char,
                            ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--output\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-dtdvalid\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--dtdvalid\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-path\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--path\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-dtdvalidfpi\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--dtdvalidfpi\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-relaxng\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--relaxng\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-maxmem\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--maxmem\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-pretty\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--pretty\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-schema\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--schema\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-schematron\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--schematron\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-pattern\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--pattern\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-xpath\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--xpath\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else if strcmp(
                            *argv.offset(i as isize),
                            b"-max-ampl\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--max-ampl\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            i += 1;
                        } else {
                            if timing != 0 && repeat != 0 {
                                startTimer();
                            }
                            if *(*argv.offset(i as isize)).offset(0 as c_int as isize)
                                as c_int
                                != '-' as i32
                                || strcmp(
                                    *argv.offset(i as isize),
                                    b"-\0" as *const u8 as *const c_char,
                                ) == 0 as c_int
                            {
                                if repeat != 0 {
                                    let mut ctxt_2: xmlParserCtxtPtr =
                                        ::core::ptr::null_mut::<xmlParserCtxt>();
                                    ctxt_2 = xmlNewParserCtxt();
                                    if ctxt_2.is_null() {
                                        progresult = XMLLINT_ERR_MEM;
                                        current_block = 13714557355923613615;
                                        break;
                                    } else {
                                        if maxAmpl > 0 as c_uint {
                                            xmlCtxtSetMaxAmplification(ctxt_2, maxAmpl);
                                        }
                                        acount = 0 as c_int;
                                        while acount < repeat {
                                            if stream != 0 as c_int {
                                                streamFile(*argv.offset(i as isize));
                                            } else if sax != 0 {
                                                testSAX(*argv.offset(i as isize));
                                            } else {
                                                parseAndPrintFile(*argv.offset(i as isize), ctxt_2);
                                            }
                                            acount += 1;
                                        }
                                        xmlFreeParserCtxt(ctxt_2);
                                    }
                                } else {
                                    nbregister = 0 as c_int;
                                    if stream != 0 as c_int {
                                        streamFile(*argv.offset(i as isize));
                                    } else if sax != 0 {
                                        testSAX(*argv.offset(i as isize));
                                    } else {
                                        parseAndPrintFile(
                                            *argv.offset(i as isize),
                                            ::core::ptr::null_mut::<xmlParserCtxt>(),
                                        );
                                    }
                                    if chkregister != 0 && nbregister != 0 as c_int {
                                        fprintf(
                                            stderr,
                                            b"Registration count off: %d\n\0" as *const u8
                                                as *const c_char,
                                            nbregister,
                                        );
                                        progresult = XMLLINT_ERR_RDREGIS;
                                    }
                                }
                                files += 1;
                                if timing != 0 && repeat != 0 {
                                    endTimer(
                                        b"%d iterations\0" as *const u8
                                            as *const c_char,
                                        repeat,
                                    );
                                }
                            }
                        }
                        i += 1;
                    }
                    match current_block {
                        13714557355923613615 => {}
                        _ => {
                            if generate != 0 {
                                parseAndPrintFile(
                                    ::core::ptr::null_mut::<c_char>(),
                                    ::core::ptr::null_mut::<xmlParserCtxt>(),
                                );
                            }
                            if htmlout != 0 && nowrap == 0 {
                                (*__xmlGenericError()).expect("non-null function pointer")(
                                    *__xmlGenericErrorContext(),
                                    b"</body></html>\n\0" as *const u8
                                        as *const c_char,
                                );
                            }
                            if files == 0 as c_int
                                && generate == 0
                                && version == 0 as c_int
                            {
                                usage(stderr, *argv.offset(0 as c_int as isize));
                                progresult = XMLLINT_ERR_UNCLASS;
                            }
                            if !wxschematron.is_null() {
                                xmlSchematronFree(wxschematron);
                            }
                            if !relaxngschemas.is_null() {
                                xmlRelaxNGFree(relaxngschemas);
                            }
                            if !wxschemas.is_null() {
                                xmlSchemaFree(wxschemas);
                            }
                            if !patternc.is_null() {
                                xmlFreePattern(patternc);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    xmlCleanupParser();
    return progresult as c_int;
}

pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as c_int,
            args_ptrs.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}
