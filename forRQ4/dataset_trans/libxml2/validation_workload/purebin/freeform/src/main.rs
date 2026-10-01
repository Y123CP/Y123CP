#![allow(non_snake_case)]
                                                           
                                                          
                                                          
                                                               
                                                 
                                                           
                                     
                                                    
                                                             
                                 
mod libcall {
    use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
    use xmllib::src::{
        c14n, chvalid, dict, globals, hash, parser, relaxng, threads, tree, uri, xmlreader,
        xmlsave, xmlschemas, HTMLparser, HTMLtree, xpath,
    };

                                    
    pub type XmlNode = tree::_xmlNode;
    pub type XPathObject = xpath::_xmlXPathObject;
    pub type SaxHandler = parser::xmlSAXHandler;
    pub type Writer = xmlwriter::xmlTextWriterPtr;
    use xmllib::src::xmlstring;
    use xmllib::src::xmlwriter;

                          
    pub type Doc = *mut c_void;
    pub type Buf = *mut c_void;
    pub type SaveCtxt = *mut c_void;
    pub type Reader = *mut c_void;
    pub type XPathCtx = *mut c_void;
    pub type Dict = *mut c_void;
    pub type Hash = *mut c_void;
    pub type Uri = *mut c_void;
    pub type SchemaParserCtxt = *mut c_void;
    pub type Schema = *mut c_void;
    pub type SchemaValidCtxt = *mut c_void;
    pub type RngParserCtxt = *mut c_void;
    pub type Rng = *mut c_void;
    pub type RngValidCtxt = *mut c_void;

                             
    pub use xmllib::src::c14n::XML_C14N_1_0;
    pub use xmllib::src::parser::{XML_PARSE_DTDVALID, XML_PARSE_NOERROR, XML_PARSE_NOWARNING};
    pub use xmllib::src::tree::{
        XML_CDATA_SECTION_NODE, XML_COMMENT_NODE, XML_ELEMENT_NODE, XML_ENTITY_REF_NODE,
        XML_PI_NODE, XML_TEXT_NODE,
    };
    pub use xmllib::src::xpath::{XPATH_BOOLEAN, XPATH_NODESET, XPATH_NUMBER, XPATH_STRING};
    pub use xmllib::src::HTMLparser::{HTML_PARSE_NOERROR, HTML_PARSE_NOWARNING};

    // ---- init / cleanup(threads TU) ----
    #[inline]
    pub unsafe fn xmlInitParser() {
        threads::xmlInitParser()
    }
    #[inline]
    pub unsafe fn xmlCleanupParser() {
        threads::xmlCleanupParser()
    }

                                         
    #[inline]
    pub unsafe fn xmlFree(p: *mut c_void) {
        (globals::xmlFree).expect("xmlFree")(p)
    }

    // ---- parser ----
    #[inline]
    pub unsafe fn xmlReadFile(path: *const c_char, enc: *const c_char, opts: c_int) -> Doc {
        parser::xmlReadFile(path, enc, opts) as Doc
    }
    #[inline]
    pub unsafe fn xmlSAXUserParseFile(
        sax: *mut SaxHandler,
        user_data: *mut c_void,
        filename: *const c_char,
    ) -> c_int {
        parser::xmlSAXUserParseFile(sax, user_data, filename)
    }

    // ---- tree ----
    #[inline]
    pub unsafe fn xmlDocGetRootElement(doc: Doc) -> *mut XmlNode {
        tree::xmlDocGetRootElement(doc as tree::xmlDocPtr)
    }
    #[inline]
    pub unsafe fn xmlGetProp(node: *const XmlNode, name: *const u8) -> *mut u8 {
        tree::xmlGetProp(node, name)
    }
    #[inline]
    pub unsafe fn xmlFreeDoc(doc: Doc) {
        tree::xmlFreeDoc(doc as tree::xmlDocPtr)
    }
    #[inline]
    pub unsafe fn xmlBufferCreate() -> Buf {
        tree::xmlBufferCreate() as Buf
    }
    #[inline]
    pub unsafe fn xmlBufferContent(b: Buf) -> *const u8 {
        tree::xmlBufferContent(b as tree::xmlBufferPtr)
    }
    #[inline]
    pub unsafe fn xmlBufferLength(b: Buf) -> c_int {
        tree::xmlBufferLength(b as tree::xmlBufferPtr)
    }
    #[inline]
    pub unsafe fn xmlBufferFree(b: Buf) {
        tree::xmlBufferFree(b as tree::xmlBufferPtr)
    }

    // ---- xmlsave ----
    #[inline]
    pub unsafe fn xmlDocDumpFormatMemory(doc: Doc, mem: *mut *mut u8, size: *mut c_int, format: c_int) {
        xmlsave::xmlDocDumpFormatMemory(doc as xmlsave::xmlDocPtr, mem, size, format)
    }
    #[inline]
    pub unsafe fn xmlSaveToBuffer(b: Buf, encoding: *const c_char, options: c_int) -> SaveCtxt {
        xmlsave::xmlSaveToBuffer(b as xmlsave::xmlBufferPtr, encoding, options) as SaveCtxt
    }
    #[inline]
    pub unsafe fn xmlSaveDoc(sc: SaveCtxt, doc: Doc) -> c_long {
        xmlsave::xmlSaveDoc(sc as xmlsave::xmlSaveCtxtPtr, doc as xmlsave::xmlDocPtr)
    }
    #[inline]
    pub unsafe fn xmlSaveClose(sc: SaveCtxt) -> c_int {
        xmlsave::xmlSaveClose(sc as xmlsave::xmlSaveCtxtPtr)
    }

    // ---- HTML ----
    #[inline]
    pub unsafe fn htmlReadFile(path: *const c_char, enc: *const c_char, opts: c_int) -> Doc {
        HTMLparser::htmlReadFile(path, enc, opts) as Doc
    }
    #[inline]
    pub unsafe fn htmlDocDumpMemory(doc: Doc, mem: *mut *mut u8, size: *mut c_int) {
        HTMLtree::htmlDocDumpMemory(doc as HTMLtree::xmlDocPtr, mem, size)
    }

    // ---- xmlreader ----
    #[inline]
    pub unsafe fn xmlReaderForFile(path: *const c_char, enc: *const c_char, opts: c_int) -> Reader {
        xmlreader::xmlReaderForFile(path, enc, opts) as Reader
    }
    #[inline]
    pub unsafe fn xmlTextReaderRead(r: Reader) -> c_int {
        xmlreader::xmlTextReaderRead(r as xmlreader::xmlTextReaderPtr)
    }
    #[inline]
    pub unsafe fn xmlTextReaderNodeType(r: Reader) -> c_int {
        xmlreader::xmlTextReaderNodeType(r as xmlreader::xmlTextReaderPtr)
    }
    #[inline]
    pub unsafe fn xmlTextReaderDepth(r: Reader) -> c_int {
        xmlreader::xmlTextReaderDepth(r as xmlreader::xmlTextReaderPtr)
    }
    #[inline]
    pub unsafe fn xmlTextReaderConstName(r: Reader) -> *const u8 {
        xmlreader::xmlTextReaderConstName(r as xmlreader::xmlTextReaderPtr)
    }
    #[inline]
    pub unsafe fn xmlTextReaderConstValue(r: Reader) -> *const u8 {
        xmlreader::xmlTextReaderConstValue(r as xmlreader::xmlTextReaderPtr)
    }
    #[inline]
    pub unsafe fn xmlFreeTextReader(r: Reader) {
        xmlreader::xmlFreeTextReader(r as xmlreader::xmlTextReaderPtr)
    }

    // ---- xpath ----
    #[inline]
    pub unsafe fn xmlXPathNewContext(doc: Doc) -> XPathCtx {
        xpath::xmlXPathNewContext(doc as xpath::xmlDocPtr) as XPathCtx
    }
    #[inline]
    pub unsafe fn xmlXPathEvalExpression(s: *const u8, ctx: XPathCtx) -> *mut XPathObject {
        xpath::xmlXPathEvalExpression(s, ctx as xpath::xmlXPathContextPtr)
    }
    #[inline]
    pub unsafe fn xmlXPathFreeObject(obj: *mut XPathObject) {
        xpath::xmlXPathFreeObject(obj)
    }
    #[inline]
    pub unsafe fn xmlXPathFreeContext(ctx: XPathCtx) {
        xpath::xmlXPathFreeContext(ctx as xpath::xmlXPathContextPtr)
    }

                                                               
    #[inline]
    pub unsafe fn xmlNewTextWriterMemory(b: Buf, compression: c_int) -> Writer {
        xmlwriter::xmlNewTextWriterMemory(b as xmlwriter::xmlBufferPtr, compression)
    }
    pub use xmlwriter::{
        xmlFreeTextWriter, xmlTextWriterEndDocument, xmlTextWriterEndElement,
        xmlTextWriterStartDocument, xmlTextWriterStartElement, xmlTextWriterWriteAttribute,
        xmlTextWriterWriteCDATA, xmlTextWriterWriteComment, xmlTextWriterWriteFormatAttribute,
        xmlTextWriterWriteFormatElement, xmlTextWriterWritePI, xmlTextWriterWriteString,
    };

    // ---- dict / hash ----
    #[inline]
    pub unsafe fn xmlDictCreate() -> Dict {
        dict::xmlDictCreate() as Dict
    }
    #[inline]
    pub unsafe fn xmlDictLookup(d: Dict, name: *const u8, len: c_int) -> *const u8 {
        dict::xmlDictLookup(d as dict::xmlDictPtr, name, len)
    }
    #[inline]
    pub unsafe fn xmlDictSize(d: Dict) -> c_int {
        dict::xmlDictSize(d as dict::xmlDictPtr)
    }
    #[inline]
    pub unsafe fn xmlDictFree(d: Dict) {
        dict::xmlDictFree(d as dict::xmlDictPtr)
    }
    #[inline]
    pub unsafe fn xmlHashCreate(size: c_int) -> Hash {
        hash::xmlHashCreate(size) as Hash
    }
    #[inline]
    pub unsafe fn xmlHashAddEntry(h: Hash, key: *const u8, payload: *mut c_void) -> c_int {
        hash::xmlHashAddEntry(h as hash::xmlHashTablePtr, key, payload)
    }
    #[inline]
    pub unsafe fn xmlHashLookup(h: Hash, key: *const u8) -> *mut c_void {
        hash::xmlHashLookup(h as hash::xmlHashTablePtr, key)
    }
    #[inline]
    pub unsafe fn xmlHashSize(h: Hash) -> c_int {
        hash::xmlHashSize(h as hash::xmlHashTablePtr)
    }
    #[inline]
    pub unsafe fn xmlHashFree(h: Hash) {
        hash::xmlHashFree(h as hash::xmlHashTablePtr, None)
    }

    // ---- uri / string / chvalid ----
    #[inline]
    pub unsafe fn xmlParseURI(s: *const c_char) -> Uri {
        uri::xmlParseURI(s) as Uri
    }
    #[inline]
    pub unsafe fn xmlSaveUri(u: Uri) -> *mut u8 {
        uri::xmlSaveUri(u as uri::xmlURIPtr)
    }
    #[inline]
    pub unsafe fn xmlFreeURI(u: Uri) {
        uri::xmlFreeURI(u as uri::xmlURIPtr)
    }
    #[inline]
    pub unsafe fn xmlBuildURI(uri_: *const u8, base: *const u8) -> *mut u8 {
        uri::xmlBuildURI(uri_, base)
    }
    pub use xmlstring::{xmlStrcat, xmlStrcmp, xmlStrdup, xmlStrlen, xmlStrstr};
                                                                            
    #[inline]
    pub unsafe fn xmlCharInRangeBaseCharGroup(v: c_uint) -> c_int {
        chvalid::xmlCharInRange(v, core::ptr::addr_of!(chvalid::xmlIsBaseCharGroup))
    }

    // ---- xmlschemas ----
    #[inline]
    pub unsafe fn xmlSchemaNewParserCtxt(url: *const c_char) -> SchemaParserCtxt {
        xmlschemas::xmlSchemaNewParserCtxt(url) as SchemaParserCtxt
    }
    #[inline]
    pub unsafe fn xmlSchemaParse(pc: SchemaParserCtxt) -> Schema {
        xmlschemas::xmlSchemaParse(pc as xmlschemas::xmlSchemaParserCtxtPtr) as Schema
    }
    #[inline]
    pub unsafe fn xmlSchemaFreeParserCtxt(pc: SchemaParserCtxt) {
        xmlschemas::xmlSchemaFreeParserCtxt(pc as xmlschemas::xmlSchemaParserCtxtPtr)
    }
    #[inline]
    pub unsafe fn xmlSchemaNewValidCtxt(s: Schema) -> SchemaValidCtxt {
        xmlschemas::xmlSchemaNewValidCtxt(s as xmlschemas::xmlSchemaPtr) as SchemaValidCtxt
    }
    #[inline]
    pub unsafe fn xmlSchemaValidateDoc(vc: SchemaValidCtxt, doc: Doc) -> c_int {
        xmlschemas::xmlSchemaValidateDoc(
            vc as xmlschemas::xmlSchemaValidCtxtPtr,
            doc as xmlschemas::xmlDocPtr,
        )
    }
    #[inline]
    pub unsafe fn xmlSchemaFreeValidCtxt(vc: SchemaValidCtxt) {
        xmlschemas::xmlSchemaFreeValidCtxt(vc as xmlschemas::xmlSchemaValidCtxtPtr)
    }
    #[inline]
    pub unsafe fn xmlSchemaFree(s: Schema) {
        xmlschemas::xmlSchemaFree(s as xmlschemas::xmlSchemaPtr)
    }

    // ---- relaxng ----
    #[inline]
    pub unsafe fn xmlRelaxNGNewParserCtxt(url: *const c_char) -> RngParserCtxt {
        relaxng::xmlRelaxNGNewParserCtxt(url) as RngParserCtxt
    }
    #[inline]
    pub unsafe fn xmlRelaxNGParse(pc: RngParserCtxt) -> Rng {
        relaxng::xmlRelaxNGParse(pc as relaxng::xmlRelaxNGParserCtxtPtr) as Rng
    }
    #[inline]
    pub unsafe fn xmlRelaxNGFreeParserCtxt(pc: RngParserCtxt) {
        relaxng::xmlRelaxNGFreeParserCtxt(pc as relaxng::xmlRelaxNGParserCtxtPtr)
    }
    #[inline]
    pub unsafe fn xmlRelaxNGNewValidCtxt(r: Rng) -> RngValidCtxt {
        relaxng::xmlRelaxNGNewValidCtxt(r as relaxng::xmlRelaxNGPtr) as RngValidCtxt
    }
    #[inline]
    pub unsafe fn xmlRelaxNGValidateDoc(vc: RngValidCtxt, doc: Doc) -> c_int {
        relaxng::xmlRelaxNGValidateDoc(
            vc as relaxng::xmlRelaxNGValidCtxtPtr,
            doc as relaxng::xmlDocPtr,
        )
    }
    #[inline]
    pub unsafe fn xmlRelaxNGFreeValidCtxt(vc: RngValidCtxt) {
        relaxng::xmlRelaxNGFreeValidCtxt(vc as relaxng::xmlRelaxNGValidCtxtPtr)
    }
    #[inline]
    pub unsafe fn xmlRelaxNGFree(r: Rng) {
        relaxng::xmlRelaxNGFree(r as relaxng::xmlRelaxNGPtr)
    }

    // ---- c14n ----
    #[inline]
    pub unsafe fn xmlC14NDocDumpMemory(doc: Doc, mode: c_int, out: *mut *mut u8) -> c_int {
        c14n::xmlC14NDocDumpMemory(
            doc as c14n::xmlDocPtr,
            core::ptr::null_mut(),
            mode,
            core::ptr::null_mut(),
            0,
            out,
        )
    }

                                                      
                                                        
    #[link(name = "z")]
    #[link(name = "m")]
    #[link(name = "dl")]
    #[link(name = "pthread")]
    extern "C" {}
}

include!("../../driver.rs");
