/* libxml2 exhaustive driver — exercises the parser / tree / XPath / XPointer /
 * reader / save / encoding / list / hash / dict / valid / SAX / IO / chvalid /
 * threads / error APIs.
 *
 * Used to push libxml2 in-scope coverage to ≥80% fn / ≥75% line.
 *
 * Usage:
 *   exhaustive_driver <file.xml>
 *
 * Runs every API call cluster regardless of input (uses inline literal XML
 * for branches that need specific shapes).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <libxml/parser.h>
#include <libxml/parserInternals.h>
#include <libxml/tree.h>
#include <libxml/xmlsave.h>
#include <libxml/xmlIO.h>
#include <libxml/xmlreader.h>
#include <libxml/xpath.h>
#include <libxml/xpathInternals.h>
#include <libxml/xpointer.h>
#include <libxml/encoding.h>
#include <libxml/entities.h>
#include <libxml/valid.h>
#include <libxml/SAX2.h>
#include <libxml/uri.h>
#include <libxml/xmlmemory.h>
#include <libxml/xmlstring.h>
#include <libxml/hash.h>
#include <libxml/dict.h>
#include <libxml/list.h>
#include <libxml/chvalid.h>
#include <libxml/globals.h>
#include <libxml/threads.h>
#include <libxml/xmlerror.h>
#include <stdint.h>

static const char *INLINE_XML =
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
    "<!DOCTYPE root [\n"
    "  <!ELEMENT root (child+)>\n"
    "  <!ELEMENT child (#PCDATA)>\n"
    "  <!ATTLIST child id ID #IMPLIED>\n"
    "  <!ENTITY copyright \"&#169; 2026\">\n"
    "]>\n"
    "<root xmlns:ns=\"http://example.org/ns\" attr=\"v\">\n"
    "  <child id=\"a\">first &copyright;</child>\n"
    "  <ns:child id=\"b\">second</ns:child>\n"
    "  <child id=\"c\"><![CDATA[<raw>data</raw>]]></child>\n"
    "  <!-- comment -->\n"
    "  <?pi target?>\n"
    "</root>\n";

static int err_count = 0;
static void structured_err(void *ctx, const xmlError *err) {
    err_count++;
    (void)ctx; (void)err;
}
static void generic_err(void *ctx, const char *msg, ...) {
    err_count++;
    (void)ctx; (void)msg;
}

/* -------------- parser.c / parserInternals.c -------------- */
static void test_parser(const char *input) {
    /* xmlReadMemory (in-memory parse). */
    xmlDocPtr d1 = xmlReadMemory(input, (int)strlen(input), "in.xml", NULL,
        XML_PARSE_DTDLOAD | XML_PARSE_DTDATTR | XML_PARSE_NOENT |
        XML_PARSE_PEDANTIC | XML_PARSE_NONET);
    if (d1) xmlFreeDoc(d1);

    /* xmlReadDoc with different parse options. */
    xmlDocPtr d2 = xmlReadDoc((const xmlChar *)input, "in.xml", NULL,
        XML_PARSE_RECOVER | XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (d2) xmlFreeDoc(d2);

    /* xmlCreatePushParserCtxt + xmlParseChunk (push parser). */
    xmlParserCtxtPtr push = xmlCreatePushParserCtxt(NULL, NULL,
        input, 64, "push.xml");
    if (push) {
        xmlParseChunk(push, input + 64, (int)strlen(input) - 64, 1);
        if (push->myDoc) xmlFreeDoc(push->myDoc);
        xmlFreeParserCtxt(push);
    }

    /* xmlCreateMemoryParserCtxt + xmlParseDocument (low-level). */
    xmlParserCtxtPtr mem = xmlCreateMemoryParserCtxt(input, (int)strlen(input));
    if (mem) {
        xmlParseDocument(mem);
        if (mem->myDoc) xmlFreeDoc(mem->myDoc);
        xmlFreeParserCtxt(mem);
    }

    /* xmlCreateIOParserCtxt — read from FILE via xmlReadFd path. */
    xmlDocPtr fd_doc = xmlReadFd(0, NULL, NULL, 0);
    if (fd_doc) xmlFreeDoc(fd_doc);

    /* xmlSAXUserParseMemory — exercise SAX. */
    xmlSAXHandler sax;
    memset(&sax, 0, sizeof(sax));
    xmlSAX2InitDefaultSAXHandler(&sax, 0);
    xmlSAXUserParseMemory(&sax, NULL, input, (int)strlen(input));

    /* xmlNewParserCtxt + reset + read variants. */
    xmlParserCtxtPtr ctxt = xmlNewParserCtxt();
    if (ctxt) {
        xmlCtxtReset(ctxt);
        xmlCtxtResetPush(ctxt, input, (int)strlen(input), "ctxt.xml", NULL);
        xmlCtxtUseOptions(ctxt, XML_PARSE_NOENT | XML_PARSE_NOCDATA);
        /* xmlCtxtReadDoc / xmlCtxtReadMemory. */
        xmlDocPtr rd = xmlCtxtReadDoc(ctxt, (const xmlChar *)input,
                                      "ctxt.xml", NULL, 0);
        if (rd) xmlFreeDoc(rd);
        xmlDocPtr rm = xmlCtxtReadMemory(ctxt, input, (int)strlen(input),
                                         "ctxt.xml", NULL, 0);
        if (rm) xmlFreeDoc(rm);
        if (ctxt->myDoc) xmlFreeDoc(ctxt->myDoc);
        xmlFreeParserCtxt(ctxt);
    }

    /* Misc parser internals: name validation, language id. */
    xmlIsLetter('A');
    xmlIsLetter(0x4E2D);          
    /* Look up an entity from the predefined table. */
    xmlEntityPtr lt = xmlGetPredefinedEntity(BAD_CAST "lt");
    xmlEntityPtr gt = xmlGetPredefinedEntity(BAD_CAST "gt");
    xmlEntityPtr apos = xmlGetPredefinedEntity(BAD_CAST "apos");
    xmlEntityPtr quot = xmlGetPredefinedEntity(BAD_CAST "quot");
    (void)lt;(void)gt;(void)apos;(void)quot;

    /* Parse a doc with public+system ID DTD reference (won't load externally
     * because nonet, but exercises the SAX path for DOCTYPE handling). */
    static const char nota_xml[] =
        "<?xml version=\"1.0\"?>\n"
        "<!DOCTYPE root [\n"
        "  <!NOTATION gif PUBLIC \"-//IETF//NOTATION GIF//EN\" \"gif\">\n"
        "  <!ENTITY pic SYSTEM \"pic.gif\" NDATA gif>\n"
        "  <!ELEMENT root (item*)>\n"
        "  <!ATTLIST root pic ENTITY #IMPLIED>\n"
        "  <!ELEMENT item EMPTY>\n"
        "  <!ATTLIST item id ID #REQUIRED>\n"
        "]>\n"
        "<root pic=\"pic\"><item id=\"x\"/></root>\n";
    xmlDocPtr nd = xmlReadMemory(nota_xml, sizeof nota_xml - 1,
                                 "nota.xml", NULL,
                                 XML_PARSE_DTDLOAD | XML_PARSE_DTDATTR);
    if (nd) xmlFreeDoc(nd);

    /* Legacy parse APIs (lots of small wrappers). */
    xmlDocPtr ld = xmlParseDoc((xmlChar *)input);
    if (ld) xmlFreeDoc(ld);
    xmlDocPtr lm = xmlParseMemory(input, (int)strlen(input));
    if (lm) xmlFreeDoc(lm);
    xmlDocPtr rm = xmlRecoverMemory(input, (int)strlen(input));
    if (rm) xmlFreeDoc(rm);
    xmlDocPtr rd = xmlRecoverDoc((xmlChar *)input);
    if (rd) xmlFreeDoc(rd);

    /* Write input to a tmpfile + parse-from-file APIs. */
    char tmpl[] = "/tmp/libxml2_drv.XXXXXX";
    int fd = mkstemp(tmpl);
    if (fd >= 0) {
        write(fd, input, strlen(input));
        close(fd);
        xmlDocPtr fd1 = xmlReadFile(tmpl, NULL, 0);
        if (fd1) xmlFreeDoc(fd1);
        xmlDocPtr fd2 = xmlParseFile(tmpl);
        if (fd2) xmlFreeDoc(fd2);
        xmlDocPtr fd3 = xmlRecoverFile(tmpl);
        if (fd3) xmlFreeDoc(fd3);
        xmlDocPtr fd4 = xmlSAXParseFile(NULL, tmpl, 0);
        if (fd4) xmlFreeDoc(fd4);
        xmlDocPtr fd5 = xmlSAXParseFileWithData(NULL, tmpl, 0, NULL);
        if (fd5) xmlFreeDoc(fd5);
        /* xmlSAXUserParseFile + xmlSAXParseDoc + xmlSAXParseMemory. */
        xmlSAXHandler sax;
        memset(&sax, 0, sizeof(sax));
        xmlSAX2InitDefaultSAXHandler(&sax, 0);
        xmlSAXUserParseFile(&sax, NULL, tmpl);
        xmlDocPtr sd = xmlSAXParseDoc(&sax, (xmlChar *)input, 0);
        if (sd) xmlFreeDoc(sd);
        xmlDocPtr sm = xmlSAXParseMemory(&sax, input, (int)strlen(input), 0);
        if (sm) xmlFreeDoc(sm);
        xmlDocPtr smw = xmlSAXParseMemoryWithData(&sax, input,
                                                   (int)strlen(input), 0, NULL);
        if (smw) xmlFreeDoc(smw);

        /* xmlReadFile + xmlCtxtReadFile via Ctxt. */
        xmlParserCtxtPtr fc = xmlCreateFileParserCtxt(tmpl);
        if (fc) {
            xmlParseDocument(fc);
            if (fc->myDoc) xmlFreeDoc(fc->myDoc);
            xmlFreeParserCtxt(fc);
        }
        xmlParserCtxtPtr uc = xmlCreateURLParserCtxt(tmpl, 0);
        if (uc) {
            xmlParseDocument(uc);
            if (uc->myDoc) xmlFreeDoc(uc->myDoc);
            xmlFreeParserCtxt(uc);
        }
        /* xmlCtxtReadFd. */
        xmlParserCtxtPtr fdc = xmlNewParserCtxt();
        if (fdc) {
            int fdr = open(tmpl, 0);
            if (fdr >= 0) {
                xmlDocPtr fdd = xmlCtxtReadFd(fdc, fdr, NULL, NULL, 0);
                if (fdd) xmlFreeDoc(fdd);
                close(fdr);
            }
            xmlFreeParserCtxt(fdc);
        }
        unlink(tmpl);
    }

    /* xmlParseBalancedChunkMemory + xmlParseInNodeContext. */
    {
        xmlNodePtr res = NULL;
        xmlParseBalancedChunkMemory(NULL, NULL, NULL, 0,
                                    BAD_CAST "<a/><b/>", &res);
        if (res) xmlFreeNodeList(res);
    }
    {
        xmlDocPtr root_doc = xmlReadMemory("<r/>", 4, "r.xml", NULL, 0);
        if (root_doc) {
            xmlNodePtr resnode = NULL;
            xmlParseInNodeContext(xmlDocGetRootElement(root_doc),
                                  "<a/>", 4, 0, &resnode);
            if (resnode) xmlFreeNodeList(resnode);
            xmlFreeDoc(root_doc);
        }
    }

    /* parserInternals helpers. */
    xmlKeepBlanksDefault(0);
    xmlKeepBlanksDefault(1);
    xmlLineNumbersDefault(0);
    xmlLineNumbersDefault(1);
    xmlPedanticParserDefault(0);
    xmlSubstituteEntitiesDefault(0);
    /* xmlClearParserCtxt + xmlInitParserCtxt + xmlNewStringInputStream. */
    xmlParserCtxtPtr pci = xmlNewParserCtxt();
    if (pci) {
        xmlClearParserCtxt(pci);
        xmlInitParserCtxt(pci);
        xmlParserInputPtr inp = xmlNewStringInputStream(pci, BAD_CAST "<a/>");
        if (inp) {
            xmlParserInputGrow(inp, 64);
            xmlParserInputRead(inp, 4);
            xmlFreeInputStream(inp);
        }
        xmlFreeParserCtxt(pci);
    }
    /* xmlSwitchEncoding on a fresh context. */
    xmlParserCtxtPtr se = xmlNewParserCtxt();
    if (se) {
        xmlSwitchEncoding(se, XML_CHAR_ENCODING_UTF8);
        xmlCtxtSetMaxAmplification(se, 10);
        xmlFreeParserCtxt(se);
    }

    /* xmlIsNameStartChar/xmlIsNameChar are private in this build; skip. */
    /* Language ID validation. */
    xmlCheckLanguageID(BAD_CAST "en");
    xmlCheckLanguageID(BAD_CAST "en-US");
    xmlCheckLanguageID(BAD_CAST "fr-CA");
    xmlCheckLanguageID(BAD_CAST "i-default");
    xmlCheckLanguageID(BAD_CAST "x-private");

    /* xmlPushInput / xmlPopInput on a freshly-built parser context. */
    xmlParserCtxtPtr pi = xmlNewParserCtxt();
    if (pi) {
        xmlParserInputPtr inp = xmlNewStringInputStream(pi, BAD_CAST "<a/>");
        if (inp) {
            xmlPushInput(pi, inp);
            xmlPopInput(pi);
        }
        xmlFreeParserCtxt(pi);
    }

    /* Parse XML that exercises xmlParseCharRef, xmlParseCharData,
     * xmlParseCommentComplex, xmlParseConditionalSections. */
    static const char rich_xml[] =
        "<?xml version='1.0'?>"
        "<root><e>&#65;&#x42;&amp;&lt;&gt;</e>"
        "<!-- multi line\n--><!-- nested-style hyphen -->"
        "<!--*comment with stars*-->"
        "<![CDATA[ raw <data> & entities not parsed ]]>"
        "</root>";
    xmlDocPtr rdoc = xmlReadMemory(rich_xml, sizeof rich_xml - 1,
                                   "rich.xml", NULL, XML_PARSE_NOENT);
    if (rdoc) xmlFreeDoc(rdoc);

    /* xmlReadDoc with extras: Nmtoken, enumeration types in DTD. */
    static const char enum_xml[] =
        "<?xml version='1.0'?>"
        "<!DOCTYPE root [\n"
        "  <!ELEMENT root EMPTY>\n"
        "  <!ATTLIST root mode (a|b|c) 'a'>\n"
        "  <!ATTLIST root tokens NMTOKENS #IMPLIED>\n"
        "]>"
        "<root mode='b' tokens='x y z'/>";
    xmlDocPtr edoc = xmlReadMemory(enum_xml, sizeof enum_xml - 1,
                                   "enum.xml", NULL,
                                   XML_PARSE_DTDATTR | XML_PARSE_DTDVALID);
    if (edoc) xmlFreeDoc(edoc);
}

/* -------------- tree.c -------------- */
static void test_tree(void) {
    /* Build a document programmatically. */
    xmlDocPtr doc = xmlNewDoc(BAD_CAST "1.0");
    xmlNodePtr root = xmlNewNode(NULL, BAD_CAST "root");
    xmlDocSetRootElement(doc, root);
    xmlNsPtr ns = xmlNewNs(root, BAD_CAST "http://example.org", BAD_CAST "ex");

    /* Children, attrs, text. */
    for (int i = 0; i < 5; i++) {
        char buf[64];
        snprintf(buf, sizeof buf, "item-%d", i);
        xmlNodePtr c = xmlNewChild(root, ns, BAD_CAST "item", BAD_CAST buf);
        xmlNewProp(c, BAD_CAST "id", BAD_CAST buf);
        xmlSetProp(c, BAD_CAST "lang", BAD_CAST "en");
        xmlSetNsProp(c, ns, BAD_CAST "key", BAD_CAST "v");
        xmlChar *got = xmlGetProp(c, BAD_CAST "id");
        if (got) xmlFree(got);
        xmlChar *gotns = xmlGetNsProp(c, BAD_CAST "key", ns->href);
        if (gotns) xmlFree(gotns);
    }
    xmlNodePtr text = xmlNewText(BAD_CAST "raw text");
    xmlAddChild(root, text);
    xmlNodePtr cdata = xmlNewCDataBlock(doc, BAD_CAST "<raw>data</raw>", 15);
    xmlAddChild(root, cdata);
    xmlNodePtr comment = xmlNewComment(BAD_CAST "a comment");
    xmlAddChild(root, comment);
    xmlNodePtr pi = xmlNewPI(BAD_CAST "target", BAD_CAST "data");
    xmlAddChild(root, pi);
    xmlNodePtr ref = xmlNewReference(doc, BAD_CAST "amp");
    if (ref) xmlAddChild(root, ref);

    /* Navigation. */
    xmlNodePtr first = xmlFirstElementChild(root);
    if (first) {
        xmlNodePtr next = xmlNextElementSibling(first);
        xmlNodePtr prev = xmlPreviousElementSibling(next);
        xmlNodePtr last = xmlLastElementChild(root);
        (void)next; (void)prev; (void)last;
    }
    unsigned long count = xmlChildElementCount(root);
    (void)count;

    /* Copy + replace + unlink. */
    xmlNodePtr clone = xmlCopyNode(root, 1);
    if (clone) xmlFreeNode(clone);
    xmlNodePtr clone_props = xmlCopyPropList(root, root->properties);
    if (clone_props) xmlFreePropList(clone_props);

    /* Node string content getters/setters. */
    xmlNodeSetContent(text, BAD_CAST "replaced");
    xmlChar *got = xmlNodeGetContent(text);
    if (got) xmlFree(got);
    xmlNodeAddContent(text, BAD_CAST " more");
    xmlNodeAddContentLen(text, BAD_CAST " more2", 6);

    /* xmlBuffer. */
    xmlBufferPtr buf = xmlBufferCreate();
    xmlNodeDump(buf, doc, root, 0, 1);
    xmlBufferFree(buf);

    /* xmlDoc copy + dump. */
    xmlDocPtr dup = xmlCopyDoc(doc, 1);
    if (dup) {
        xmlChar *dumped = NULL;
        int dlen = 0;
        xmlDocDumpMemory(dup, &dumped, &dlen);
        if (dumped) xmlFree(dumped);
        xmlDocDumpFormatMemory(dup, &dumped, &dlen, 1);
        if (dumped) xmlFree(dumped);
        xmlFreeDoc(dup);
    }

    /* xmlSearchNs / xmlSearchNsByHref. */
    xmlNsPtr found = xmlSearchNs(doc, root, BAD_CAST "ex");
    xmlNsPtr foundh = xmlSearchNsByHref(doc, root, BAD_CAST "http://example.org");
    (void)found; (void)foundh;

    /* More tree manipulation: AddPrevSibling / AddNextSibling / ReplaceNode / TextMerge. */
    xmlNodePtr sibling = xmlNewNode(NULL, BAD_CAST "sib");
    xmlAddNextSibling(text, sibling);
    xmlNodePtr prevsib = xmlNewNode(NULL, BAD_CAST "prevsib");
    xmlAddPrevSibling(sibling, prevsib);
    xmlNodePtr repl = xmlNewNode(NULL, BAD_CAST "replaced");
    xmlNodePtr was = xmlReplaceNode(prevsib, repl);
    if (was) xmlFreeNode(was);
    /* xmlTextMerge. */
    xmlNodePtr t1 = xmlNewText(BAD_CAST "abc");
    xmlNodePtr t2 = xmlNewText(BAD_CAST "def");
    xmlAddChild(root, t1);
    xmlNodePtr merged = xmlTextMerge(t1, t2);
    (void)merged;
    /* xmlAddSibling. */
    xmlNodePtr sib2 = xmlNewText(BAD_CAST "sib-text");
    xmlAddSibling(t1, sib2);
    /* xmlNodeIsText / xmlIsBlankNode. */
    xmlNodeIsText(t1);
    xmlIsBlankNode(t1);
    /* xmlValidateName / xmlValidateNCName / xmlValidateQName. */
    xmlValidateName(BAD_CAST "valid-name", 0);
    xmlValidateNCName(BAD_CAST "valid-ncname", 0);
    xmlValidateQName(BAD_CAST "ns:name", 0);
    xmlValidateNMToken(BAD_CAST "token", 0);
    /* xmlSplitQName. */
    xmlChar *prefix = NULL;
    xmlChar *localname = xmlSplitQName2(BAD_CAST "ns:name", &prefix);
    if (localname) xmlFree(localname);
    if (prefix) xmlFree(prefix);
    /* xmlBuildQName. */
    xmlChar mem[64];
    xmlChar *qn = xmlBuildQName(BAD_CAST "name", BAD_CAST "ns", mem, sizeof mem);
    if (qn && qn != mem) xmlFree(qn);
    /* xmlNodeListGetString / xmlNodeListGetRawString. */
    xmlChar *ls = xmlNodeListGetString(doc, root->children, 1);
    if (ls) xmlFree(ls);
    xmlChar *lsr = xmlNodeListGetRawString(doc, root->children, 1);
    if (lsr) xmlFree(lsr);
    /* xmlGetNodePath. */
    xmlChar *path = xmlGetNodePath(text);
    if (path) xmlFree(path);
    /* xmlGetLineNo. */
    xmlGetLineNo(root);
    /* xmlBuffer ops. */
    xmlBufferPtr b = xmlBufferCreate();
    xmlBufferAdd(b, BAD_CAST "hello", -1);
    xmlBufferCCat(b, " world");
    xmlBufferGrow(b, 16);
    xmlBufferContent(b);
    xmlBufferLength(b);
    xmlBufferShrink(b, 3);
    xmlBufferEmpty(b);
    xmlBufferAdd(b, BAD_CAST "again", -1);
    xmlBufferResize(b, 1024);
    xmlBufferFree(b);

    /* Additional tree.c functions. */
    xmlNodePtr child_list = xmlNewNode(NULL, BAD_CAST "list_root");
    xmlNodePtr cl_a = xmlNewNode(NULL, BAD_CAST "ca");
    xmlNodePtr cl_b = xmlNewNode(NULL, BAD_CAST "cb");
    cl_a->next = cl_b;
    cl_b->prev = cl_a;
    xmlAddChildList(child_list, cl_a);
    xmlFreeNode(child_list);

    /* Buffer create-size + create-static + add-head + detach + dump. */
    xmlBufferPtr bcs = xmlBufferCreateSize(64);
    xmlBufferAdd(bcs, BAD_CAST "world", 5);
    xmlBufferAddHead(bcs, BAD_CAST "hello-", 6);
    xmlChar *detached = xmlBufferDetach(bcs);
    if (detached) xmlFree(detached);
    xmlBufferFree(bcs);
    FILE *dt = tmpfile();
    if (dt) {
        xmlBufferPtr b = xmlBufferCreate();
        xmlBufferAdd(b, BAD_CAST "abcde", 5);
        xmlBufferDump(dt, b);
        xmlBufferFree(b);
        fclose(dt);
    }
    /* Buffer-create-static (read-only view). */
    xmlBufferPtr bst = xmlBufferCreateStatic((void *)"static", 6);
    if (bst) xmlBufferFree(bst);

    /* xmlCopyNodeList. */
    xmlNodePtr nl = xmlNewNode(NULL, BAD_CAST "a");
    xmlAddSibling(nl, xmlNewNode(NULL, BAD_CAST "b"));
    xmlNodePtr cnl = xmlCopyNodeList(nl);
    if (cnl) xmlFreeNodeList(cnl);
    xmlFreeNodeList(nl);

    /* More tree.c functions. */
    xmlNodePtr docfrag = xmlNewDocFragment(doc);
    if (docfrag) xmlFreeNode(docfrag);
    xmlNodePtr cr = xmlNewCharRef(doc, BAD_CAST "amp");
    if (cr) xmlFreeNode(cr);
    xmlNodePtr drn = xmlNewDocRawNode(doc, NULL, BAD_CAST "raw", BAD_CAST "value");
    if (drn) xmlFreeNode(drn);
    xmlNodePtr dtl = xmlNewDocTextLen(doc, BAD_CAST "hello", 5);
    if (dtl) xmlFreeNode(dtl);
    /* xmlNewNsProp attaches to root; ownership stays with root. */
    (void)xmlNewNsProp(root, ns, BAD_CAST "nsprop", BAD_CAST "v");
    /* xmlGetLastChild / xmlGetNoNsProp / xmlHasNsProp / xmlGetNsList. */
    xmlGetLastChild(root);
    xmlChar *nonsprop = xmlGetNoNsProp(root, BAD_CAST "id");
    if (nonsprop) xmlFree(nonsprop);
    xmlHasNsProp(root, BAD_CAST "id", NULL);
    xmlNsPtr *nslist = xmlGetNsList(doc, root);
    if (nslist) xmlFree(nslist);
    /* Compression scheme + buffer alloc scheme. */
    xmlGetBufferAllocationScheme();
    xmlGetCompressMode();
    xmlGetDocCompressMode(doc);
    xmlSetCompressMode(0);
    xmlSetDocCompressMode(doc, 0);
    /* xmlBufGetNodeContent: xmlBufPtr API is internal; skip. */
    /* xmlDocCopyNodeList. */
    xmlNodePtr docnl = xmlDocCopyNodeList(doc, root->children);
    if (docnl) xmlFreeNodeList(docnl);
    /* xmlAddPropSibling not exported in this build; skip. */

    /* Additional tree.c functions. */
    xmlNodePtr ext = xmlNewTextChild(root, NULL, BAD_CAST "txt", BAD_CAST "v");
    if (ext) {
        xmlNodeSetBase(ext, BAD_CAST "http://base.example.com/");
        xmlNodeSetLang(ext, BAD_CAST "en");
        xmlNodeSetName(ext, BAD_CAST "renamed");
        xmlNodeSetSpacePreserve(ext, 1);
        xmlNodeSetContentLen(ext, BAD_CAST "hello world", 11);
        xmlUnsetNsProp(ext, ns, BAD_CAST "nonexist");
        xmlRemoveProp(xmlSetProp(ext, BAD_CAST "del", BAD_CAST "v"));
    }
    /* xmlSearchNsByPrefixStrict is not exported in this build. */
    /* xmlSetNs - explicitly change a node's namespace. */
    xmlNsPtr ns2 = xmlNewNs(root, BAD_CAST "http://other.example.com", BAD_CAST "ot");
    xmlSetNs(ext, ns2);
    /* xmlReconciliateNs. */
    xmlReconciliateNs(doc, root);
    /* xmlGetParameterEntityFromDtd is not exported in this build. */
    /* xmlSetBufferAllocationScheme. */
    xmlSetBufferAllocationScheme(XML_BUFFER_ALLOC_DOUBLEIT);

    /* xmlUnsetProp. */
    xmlAttrPtr unset_target = xmlSetProp(root, BAD_CAST "tmp_attr", BAD_CAST "v");
    if (unset_target) xmlUnsetProp(root, BAD_CAST "tmp_attr");

    /* xmlDOMWrap adopt-attr branch. */
    xmlDOMWrapCtxtPtr wctxt2 = xmlDOMWrapNewCtxt();
    if (wctxt2) {
        xmlDocPtr src2 = xmlReadMemory("<root attr='v'/>", 16, "src.xml", NULL, 0);
        xmlDocPtr dst2 = xmlNewDoc(BAD_CAST "1.0");
        xmlNodePtr d2root = xmlNewNode(NULL, BAD_CAST "dst");
        xmlDocSetRootElement(dst2, d2root);
        if (src2 && dst2) {
            xmlNodePtr srcroot = xmlDocGetRootElement(src2);
            if (srcroot && srcroot->properties) {
                /* AdoptAttr: move an attr from src into dst. */
                xmlDOMWrapAdoptNode(wctxt2, src2, (xmlNodePtr)srcroot->properties,
                                    dst2, d2root, 0);
            }
            xmlDOMWrapRemoveNode(wctxt2, dst2, d2root, 0);
        }
        if (src2) xmlFreeDoc(src2);
        if (dst2) xmlFreeDoc(dst2);
        xmlDOMWrapFreeCtxt(wctxt2);
    }

    /* xmlDOMWrap* helpers (DOM normalization helpers). */
    xmlDOMWrapCtxtPtr wctxt = xmlDOMWrapNewCtxt();
    if (wctxt) {
        xmlDocPtr src = xmlReadMemory(
            "<root xmlns:a='http://a'><a:c/></root>", 37,
            "src.xml", NULL, 0);
        xmlDocPtr dst = xmlNewDoc(BAD_CAST "1.0");
        xmlNodePtr dstroot = xmlNewNode(NULL, BAD_CAST "destroot");
        xmlDocSetRootElement(dst, dstroot);
        if (src && dst) {
            xmlNodePtr cl = NULL;
            xmlDOMWrapCloneNode(wctxt,
                                src,
                                xmlDocGetRootElement(src),
                                &cl,
                                dst, dstroot, 1, 0);
            if (cl) xmlFreeNode(cl);
            xmlDOMWrapAdoptNode(wctxt, src,
                                xmlDocGetRootElement(src),
                                dst, dstroot, 0);
            xmlDOMWrapReconcileNamespaces(wctxt, dstroot, 0);
        }
        if (src) xmlFreeDoc(src);
        if (dst) xmlFreeDoc(dst);
        xmlDOMWrapFreeCtxt(wctxt);
    }

    xmlFreeDoc(doc);
}

/* -------------- xmlsave.c + xmlIO.c -------------- */
static void test_save(const char *input) {
    xmlDocPtr d = xmlReadMemory(input, (int)strlen(input), "in.xml", NULL, 0);
    if (!d) return;

    /* Save to a memory buffer via xmlSaveToBuffer. */
    xmlBufferPtr buf = xmlBufferCreate();
    xmlSaveCtxtPtr c1 = xmlSaveToBuffer(buf, NULL, XML_SAVE_FORMAT);
    if (c1) {
        xmlSaveDoc(c1, d);
        xmlSaveTree(c1, xmlDocGetRootElement(d));
        xmlSaveFlush(c1);
        xmlSaveClose(c1);
    }
    xmlBufferFree(buf);

    /* xmlSaveToFd. */
    FILE *t1 = tmpfile();
    if (t1) {
        xmlSaveCtxtPtr c2 = xmlSaveToFd(fileno(t1), NULL,
                                         XML_SAVE_NO_DECL | XML_SAVE_NO_EMPTY);
        if (c2) {
            xmlSaveSetEscape(c2, NULL);
            xmlSaveSetAttrEscape(c2, NULL);
            xmlSaveDoc(c2, d);
            xmlSaveClose(c2);
        }
        fclose(t1);
    }

    /* Various Dump forms. */
    FILE *t2 = tmpfile();
    if (t2) {
        xmlDocDump(t2, d);
        xmlDocFormatDump(t2, d, 1);
        xmlDocDumpFormatMemoryEnc(d, NULL, NULL, "UTF-8", 1);
        xmlNodeDumpOutput(xmlOutputBufferCreateFile(t2, NULL),
                          d, xmlDocGetRootElement(d), 0, 0, NULL);
        fclose(t2);
    }

    /* Direct chunk-level xmlNodeDump into a buffer. */
    xmlBufferPtr nbuf = xmlBufferCreate();
    xmlNodeDump(nbuf, d, xmlDocGetRootElement(d), 0, 1);
    xmlBufferFree(nbuf);

    /* xmlOutputBuffer roundtrip. */
    xmlBufferPtr ob = xmlBufferCreate();
    xmlOutputBufferPtr outb = xmlOutputBufferCreateBuffer(ob, NULL);
    if (outb) {
        xmlOutputBufferWrite(outb, 5, "hello");
        xmlOutputBufferWriteString(outb, " world");
        xmlOutputBufferFlush(outb);
        xmlOutputBufferClose(outb);
    }
    xmlBufferFree(ob);

    /* xmlParserInputBuffer roundtrip — feed bytes into a parser input buffer. */
    xmlParserInputBufferPtr pib = xmlParserInputBufferCreateMem(
        input, (int)strlen(input), XML_CHAR_ENCODING_NONE);
    if (pib) {
        xmlParserInputBufferGrow(pib, 4096);
        xmlParserInputBufferRead(pib, 16);
        xmlFreeParserInputBuffer(pib);
    }
    /* From static (read-only) memory. */
    xmlParserInputBufferPtr pib2 = xmlParserInputBufferCreateStatic(
        input, (int)strlen(input), XML_CHAR_ENCODING_NONE);
    if (pib2) {
        xmlParserInputBufferPush(pib2, 4, "<a/>");
        xmlFreeParserInputBuffer(pib2);
    }

    /* xmlCheckFilename / xmlCheckHTTPInput etc. */
    xmlCheckFilename("./input.xml");

    /* xmlSaveFile / xmlSaveFileEnc / xmlSaveFileTo / xmlSaveFormatFile. */
    char stmpl[] = "/tmp/libxml2_save.XXXXXX";
    int sfd = mkstemp(stmpl);
    if (sfd >= 0) {
        close(sfd);
        xmlSaveFile(stmpl, d);
        xmlSaveFileEnc(stmpl, d, "UTF-8");
        xmlSaveFormatFile(stmpl, d, 1);
        xmlSaveFormatFileEnc(stmpl, d, "UTF-8", 1);
        FILE *fp = fopen(stmpl, "wb");
        if (fp) {
            xmlOutputBufferPtr ob = xmlOutputBufferCreateFile(fp, NULL);
            if (ob) xmlSaveFileTo(ob, d, NULL);
            fclose(fp);
        }
        /* xmlSaveToFilename + xmlSaveDoc. */
        xmlSaveCtxtPtr stf = xmlSaveToFilename(stmpl, NULL, XML_SAVE_FORMAT);
        if (stf) {
            xmlSaveDoc(stf, d);
            xmlSaveClose(stf);
        }
        unlink(stmpl);
    }
    /* xmlDocDumpMemoryEnc. */
    xmlChar *enc = NULL;
    int elen = 0;
    xmlDocDumpMemoryEnc(d, &enc, &elen, "UTF-8");
    if (enc) xmlFree(enc);

    /* xmlElemDump (deprecated but exported). */
    FILE *tmpd = tmpfile();
    if (tmpd) {
        xmlElemDump(tmpd, d, xmlDocGetRootElement(d));
        fclose(tmpd);
    }

    /* XHTML save option — triggers xhtmlNodeDumpOutput / xhtmlAttrListDumpOutput. */
    xmlBufferPtr xhbuf = xmlBufferCreate();
    xmlSaveCtxtPtr xhc = xmlSaveToBuffer(xhbuf, "UTF-8",
                                          XML_SAVE_XHTML | XML_SAVE_FORMAT);
    if (xhc) {
        xmlSaveDoc(xhc, d);
        xmlSaveClose(xhc);
    }
    xmlBufferFree(xhbuf);
    /* xmlSaveAs HTML — touches htmlNodeDumpOutputInternal. */
    xmlBufferPtr hbuf = xmlBufferCreate();
    xmlSaveCtxtPtr hc = xmlSaveToBuffer(hbuf, "UTF-8",
                                         XML_SAVE_AS_HTML | XML_SAVE_FORMAT);
    if (hc) {
        xmlSaveDoc(hc, d);
        xmlSaveClose(hc);
    }
    xmlBufferFree(hbuf);
    /* XML_SAVE_FORMAT to xmlSaveFormatFileTo (a variant we haven't used). */
    FILE *tf3 = tmpfile();
    if (tf3) {
        xmlOutputBufferPtr ob3 = xmlOutputBufferCreateFile(tf3, NULL);
        if (ob3) xmlSaveFormatFileTo(ob3, d, "UTF-8", 1);
        fclose(tf3);
    }

    /* xmlOutputBufferGetContent / xmlOutputBufferGetSize. */
    xmlBufferPtr obuf = xmlBufferCreate();
    xmlOutputBufferPtr ob2 = xmlOutputBufferCreateBuffer(obuf, NULL);
    if (ob2) {
        xmlOutputBufferWriteString(ob2, "hello");
        xmlOutputBufferFlush(ob2);
        const xmlChar *content = xmlOutputBufferGetContent(ob2);
        size_t size = xmlOutputBufferGetSize(ob2);
        (void)content; (void)size;
        xmlOutputBufferClose(ob2);
    }
    xmlBufferFree(obuf);
    /* xmlPopInput/Output Callbacks (test stack pop). */
    xmlPopInputCallbacks();
    xmlPopOutputCallbacks();
    /* Re-register default callbacks (so subsequent IO still works). */
    xmlRegisterDefaultInputCallbacks();
    xmlRegisterDefaultOutputCallbacks();

    /* xmlParserInputBufferCreateFile. */
    char ftpl[] = "/tmp/libxml2_io.XXXXXX";
    int ftf = mkstemp(ftpl);
    if (ftf >= 0) {
        write(ftf, "<a/>", 4);
        close(ftf);
        FILE *fp = fopen(ftpl, "rb");
        if (fp) {
            xmlParserInputBufferPtr fpib = xmlParserInputBufferCreateFile(fp,
                XML_CHAR_ENCODING_NONE);
            if (fpib) xmlFreeParserInputBuffer(fpib);
        }
        unlink(ftpl);
    }

    xmlFreeDoc(d);
}

/* -------------- xpath.c / xpointer.c -------------- */
static void test_xpath(const char *input) {
    xmlDocPtr d = xmlReadMemory(input, (int)strlen(input), "in.xml", NULL, 0);
    if (!d) return;
    xmlXPathContextPtr xpc = xmlXPathNewContext(d);
    if (!xpc) { xmlFreeDoc(d); return; }
    xmlXPathRegisterNs(xpc, BAD_CAST "ns", BAD_CAST "http://example.org/ns");

    const char *queries[] = {
        "/root",
        "//child",
        "//child[@id='a']",
        "/root/ns:child",
        "//child/text()",
        "count(//child)",
        "string(//child[1])",
        "//child[position()<3]",
        "//child[last()]",
        "//comment()",
        "//processing-instruction()",
        "//@*",
        "//*[name()='child']",
        "//*[local-name()='child']",
        "//ancestor::*",
        "//child[1]/following-sibling::*",
        "//child[2]/preceding-sibling::*",
        "//child/descendant::*",
        "boolean(//child)",
        "true() and false()",
        "1 + 2 * 3",
        "sum(//child/@id != '')",
        "starts-with('hello', 'he')",
        "contains('hello', 'ell')",
        "substring('hello', 2, 3)",
        "translate('abc', 'abc', 'ABC')",
        "concat('a', 'b', 'c')",
        "normalize-space('  a   b  ')",
        "string-length('xyz')",
        "round(1.7)",
        "ceiling(1.2)",
        "floor(1.8)",
    };
    for (size_t i = 0; i < sizeof(queries)/sizeof(*queries); i++) {
        xmlXPathObjectPtr r = xmlXPathEvalExpression(BAD_CAST queries[i], xpc);
        if (r) xmlXPathFreeObject(r);
    }

    /* xmlXPathCompile + xmlXPathCompiledEval (compiled variant). */
    xmlXPathCompExprPtr expr = xmlXPathCompile(BAD_CAST "//child");
    if (expr) {
        xmlXPathObjectPtr r = xmlXPathCompiledEval(expr, xpc);
        if (r) xmlXPathFreeObject(r);
        xmlXPathFreeCompExpr(expr);
    }

    /* Register an XPath variable + custom function. */
    xmlXPathRegisterVariable(xpc, BAD_CAST "x",
                             xmlXPathNewFloat(42.0));
    xmlXPathObjectPtr v = xmlXPathVariableLookup(xpc, BAD_CAST "x");
    if (v) xmlXPathFreeObject(v);

    /* Convert XPath objects between types. */
    xmlXPathObjectPtr s = xmlXPathNewString(BAD_CAST "abc");
    if (s) {
        xmlXPathBooleanFunction(NULL, 0);  /* invalid: no ctxt; just touches it */
        xmlXPathFreeObject(s);
    }
    xmlXPathObjectPtr b = xmlXPathNewBoolean(1);
    if (b) xmlXPathFreeObject(b);
    xmlXPathObjectPtr nl = xmlXPathNewNodeSet(NULL);
    if (nl) xmlXPathFreeObject(nl);
    xmlXPathObjectPtr cs = xmlXPathNewCString("hello");
    if (cs) xmlXPathFreeObject(cs);

    /* xmlXPathEval shorthand. */
    xmlXPathObjectPtr e2 = xmlXPathEval(BAD_CAST "//child[@id]", xpc);
    if (e2) xmlXPathFreeObject(e2);

    /* Cast helpers. */
    xmlXPathCastNumberToString(3.14);
    xmlXPathCastBooleanToString(1);
    xmlXPathCastStringToBoolean(BAD_CAST "true");
    xmlXPathCastStringToNumber(BAD_CAST "42");
    xmlXPathCastNumberToBoolean(0.0);
    xmlXPathCastBooleanToNumber(1);

    /* Object cast helpers (need objects to cast). */
    xmlXPathObjectPtr ob_num = xmlXPathNewFloat(42.0);
    xmlXPathObjectPtr ob_str = xmlXPathNewString(BAD_CAST "abc");
    xmlXPathObjectPtr ob_bool = xmlXPathNewBoolean(0);
    if (ob_num) {
        xmlChar *s = xmlXPathCastToString(ob_num);
        if (s) xmlFree(s);
        xmlXPathCastToBoolean(ob_num);
        xmlXPathCastToNumber(ob_num);
        xmlXPathFreeObject(ob_num);
    }
    if (ob_str) {
        xmlXPathCastToNumber(ob_str);
        xmlXPathCastToBoolean(ob_str);
        xmlXPathFreeObject(ob_str);
    }
    if (ob_bool) {
        xmlChar *s = xmlXPathCastToString(ob_bool);
        if (s) xmlFree(s);
        xmlXPathCastToNumber(ob_bool);
        xmlXPathFreeObject(ob_bool);
    }
    /* Convert in-place. */
    xmlXPathObjectPtr ob = xmlXPathNewFloat(7.5);
    xmlXPathObjectPtr cn = xmlXPathConvertNumber(ob);
    if (cn) xmlXPathFreeObject(cn);
    xmlXPathObjectPtr ob2 = xmlXPathNewFloat(7.5);
    xmlXPathObjectPtr cs2 = xmlXPathConvertString(ob2);
    if (cs2) xmlXPathFreeObject(cs2);
    xmlXPathObjectPtr ob3 = xmlXPathNewFloat(7.5);
    xmlXPathObjectPtr cb3 = xmlXPathConvertBoolean(ob3);
    if (cb3) xmlXPathFreeObject(cb3);

    /* NaN/Inf helpers. */
    xmlXPathNAN;
    xmlXPathPINF;
    xmlXPathNINF;
    xmlXPathIsNaN(xmlXPathNAN);
    xmlXPathIsInf(xmlXPathPINF);

    /* Context cache + ordering helpers. */
    xmlXPathContextSetCache(xpc, 1, -1, 0);

    /* xmlXPathOrderDocElems. */
    xmlXPathOrderDocElems(d);

    /* Explicit init. */
    xmlXPathInit();

    /* XPath queries that hit specific built-in functions. */
    const char *more_queries[] = {
        "substring-before('hello,world', ',')",
        "substring-after('hello,world', ',')",
        "id('a')",
        "lang('en')",
        "namespace-uri(//ns:child)",
        "name(//child)",
        "local-name(//ns:child)",
        "not(false())",
        "number('3.14')",
        "boolean(1)",
        "position()",
        "last()",
        "1 + 1",
        "1 - 1",
        "2 * 3",
        "6 div 2",
        "5 mod 3",
        "1 = 1",
        "1 != 2",
        "1 < 2",
        "2 <= 2",
        "3 > 2",
        "3 >= 3",
        "(//child) | (//root)",  /* node-set union */
    };
    for (size_t i = 0; i < sizeof(more_queries)/sizeof(*more_queries); i++) {
        xmlXPathObjectPtr r = xmlXPathEvalExpression(BAD_CAST more_queries[i], xpc);
        if (r) xmlXPathFreeObject(r);
    }

    /* Predicates with positional filters (trigger CompOpEvalFirst/Last/Filter). */
    const char *filter_queries[] = {
        "//child[1]",
        "//child[last()]",
        "//child[3]",
        "(//child)[2]",
        "//child[@id='a'][1]",
        "//*[name()='child'][last()]",
        "//child[position() mod 2 = 0]",
        "//child/following::*[1]",
        "//child/preceding::*[1]",
        "//child[count(@*) > 0]",
    };
    for (size_t i = 0; i < sizeof(filter_queries)/sizeof(*filter_queries); i++) {
        xmlXPathObjectPtr r = xmlXPathEvalExpression(BAD_CAST filter_queries[i], xpc);
        if (r) xmlXPathFreeObject(r);
    }

    /* Set context node + node-relative eval. */
    xmlNodePtr ctxnode = xmlDocGetRootElement(d);
    xmlXPathSetContextNode(ctxnode, xpc);
    xmlXPathObjectPtr nr = xmlXPathNodeEval(ctxnode, BAD_CAST "child", xpc);
    if (nr) xmlXPathFreeObject(nr);

    /* Compiled boolean eval. */
    xmlXPathCompExprPtr ce = xmlXPathCompile(BAD_CAST "count(//child) > 0");
    if (ce) {
        xmlXPathCompiledEvalToBoolean(ce, xpc);
        xmlXPathFreeCompExpr(ce);
    }

    /* Eval predicate. */
    xmlXPathParserContextPtr ppc = xmlXPathNewParserContext(BAD_CAST "true()", xpc);
    if (ppc) {
        xmlXPathFreeParserContext(ppc);
    }

    /* libxml2-specific XPath extensions (set operations). */
    const char *ext_queries[] = {
        "escape-uri('hello world', 1)",   /* xmlXPathEscapeUriFunction */
    };
    for (size_t i = 0; i < sizeof(ext_queries)/sizeof(*ext_queries); i++) {
        xmlXPathObjectPtr r = xmlXPathEvalExpression(BAD_CAST ext_queries[i], xpc);
        if (r) xmlXPathFreeObject(r);
    }

    /* Predicate evaluation directly. */
    xmlXPathParserContextPtr pep = xmlXPathNewParserContext(BAD_CAST "1", xpc);
    if (pep) {
        xmlXPathFreeParserContext(pep);
    }

    /* Node set explicit ops: build a set + merge + add + remove + clear. */
    xmlXPathObjectPtr ns_a = xmlXPathEvalExpression(BAD_CAST "//child", xpc);
    xmlXPathObjectPtr ns_b = xmlXPathEvalExpression(BAD_CAST "//*[@id]", xpc);
    if (ns_a && ns_b && ns_a->nodesetval && ns_b->nodesetval) {
        xmlNodeSetPtr s = xmlXPathNodeSetCreate(ctxnode);
        xmlXPathNodeSetMerge(s, ns_a->nodesetval);
        xmlXPathNodeSetAdd(s, ctxnode);
        xmlXPathNodeSetContains(s, ctxnode);
        xmlXPathNodeSetDel(s, ctxnode);
        xmlXPathFreeNodeSet(s);
    }
    if (ns_a) xmlXPathFreeObject(ns_a);
    if (ns_b) xmlXPathFreeObject(ns_b);

    /* XPointer */
    xmlXPathContextPtr xpt = xmlXPtrNewContext(d, NULL, NULL);
    if (xpt) {
        const char *ptrs[] = {
            "xpointer(/root)",
            "xpointer(//child[1])",
            "element(/1/2)",
        };
        for (size_t i = 0; i < sizeof(ptrs)/sizeof(*ptrs); i++) {
            xmlXPathObjectPtr r = xmlXPtrEval(BAD_CAST ptrs[i], xpt);
            if (r) xmlXPathFreeObject(r);
        }
        xmlXPathFreeContext(xpt);
    }

    xmlXPathFreeContext(xpc);
    xmlFreeDoc(d);
}

/* -------------- xmlreader.c -------------- */
static void test_reader(const char *input) {
    xmlTextReaderPtr r = xmlReaderForMemory(input, (int)strlen(input),
        "in.xml", NULL, XML_PARSE_DTDATTR | XML_PARSE_NOENT);
    if (!r) return;
    while (xmlTextReaderRead(r) == 1) {
        int type = xmlTextReaderNodeType(r);
        const xmlChar *name = xmlTextReaderConstName(r);
        const xmlChar *value = xmlTextReaderConstValue(r);
        int depth = xmlTextReaderDepth(r);
        int empty = xmlTextReaderIsEmptyElement(r);
        int hasA = xmlTextReaderHasAttributes(r);
        int hasV = xmlTextReaderHasValue(r);
        int attr_count = xmlTextReaderAttributeCount(r);
        (void)type;(void)name;(void)value;(void)depth;
        (void)empty;(void)hasA;(void)hasV;(void)attr_count;
        /* xmlTextReaderMoveToAttribute* */
        if (hasA) {
            xmlTextReaderMoveToFirstAttribute(r);
            do {
                const xmlChar *aname = xmlTextReaderConstLocalName(r);
                const xmlChar *aval = xmlTextReaderConstValue(r);
                (void)aname; (void)aval;
            } while (xmlTextReaderMoveToNextAttribute(r) == 1);
            /* Also try positional attribute access. */
            xmlChar *byidx0 = xmlTextReaderGetAttributeNo(r, 0);
            if (byidx0) xmlFree(byidx0);
            xmlChar *byname = xmlTextReaderGetAttribute(r, BAD_CAST "id");
            if (byname) xmlFree(byname);
            xmlChar *byns = xmlTextReaderGetAttributeNs(r, BAD_CAST "id", NULL);
            if (byns) xmlFree(byns);
            xmlTextReaderMoveToAttributeNo(r, 0);
            xmlTextReaderMoveToAttribute(r, BAD_CAST "id");
            xmlTextReaderMoveToAttributeNs(r, BAD_CAST "id", NULL);
            xmlTextReaderMoveToElement(r);
        }
        /* Get prefix / namespace URI / base / xml:lang / xml:base. */
        const xmlChar *pfx = xmlTextReaderConstPrefix(r);
        const xmlChar *uri = xmlTextReaderConstNamespaceUri(r);
        const xmlChar *base = xmlTextReaderConstBaseUri(r);
        const xmlChar *xmllang = xmlTextReaderConstXmlLang(r);
        const xmlChar *xmlver = xmlTextReaderConstXmlVersion(r);
        const xmlChar *encname = xmlTextReaderConstEncoding(r);
        const xmlChar *strval = xmlTextReaderConstString(r, BAD_CAST "hello");
        (void)pfx;(void)uri;(void)base;(void)xmllang;(void)xmlver;(void)encname;(void)strval;
        /* Misc inspection. */
        xmlTextReaderIsDefault(r);
        xmlTextReaderIsNamespaceDecl(r);
        xmlTextReaderIsValid(r);
        xmlTextReaderReadState(r);
        xmlTextReaderQuoteChar(r);
        xmlTextReaderStandalone(r);
        xmlTextReaderByteConsumed(r);
        xmlTextReaderGetParserLineNumber(r);
        xmlTextReaderGetParserColumnNumber(r);
        xmlTextReaderCurrentNode(r);
        xmlTextReaderCurrentDoc(r);
        /* Read variants. */
        xmlChar *iv = xmlTextReaderReadInnerXml(r);
        if (iv) xmlFree(iv);
        xmlChar *ov = xmlTextReaderReadOuterXml(r);
        if (ov) xmlFree(ov);
        xmlChar *sv = xmlTextReaderReadString(r);
        if (sv) xmlFree(sv);
    }
    /* Reset + close. */
    xmlTextReaderClose(r);
    xmlFreeTextReader(r);

    /* Second reader: walk into expand mode. */
    xmlTextReaderPtr r2 = xmlReaderForMemory(input, (int)strlen(input),
        "in.xml", NULL, 0);
    if (r2) {
        xmlTextReaderSetParserProp(r2, XML_PARSER_LOADDTD, 1);
        xmlTextReaderGetParserProp(r2, XML_PARSER_LOADDTD);
        while (xmlTextReaderRead(r2) == 1) {
            if (xmlTextReaderNodeType(r2) == XML_READER_TYPE_ELEMENT) {
                xmlNodePtr expanded = xmlTextReaderExpand(r2);
                (void)expanded;
            }
        }
        xmlFreeTextReader(r2);
    }

    /* Walker variant: drive a reader over an in-memory tree. */
    xmlDocPtr d = xmlReadMemory(input, (int)strlen(input), "in.xml", NULL, 0);
    if (d) {
        xmlTextReaderPtr w = xmlReaderWalker(d);
        if (w) {
            while (xmlTextReaderRead(w) == 1) {
                /* Use non-Const variants (allocates copies). */
                xmlChar *n = xmlTextReaderName(w);
                if (n) xmlFree(n);
                xmlChar *ln = xmlTextReaderLocalName(w);
                if (ln) xmlFree(ln);
                xmlChar *nu = xmlTextReaderNamespaceUri(w);
                if (nu) xmlFree(nu);
                xmlChar *pfx = xmlTextReaderPrefix(w);
                if (pfx) xmlFree(pfx);
                xmlChar *base = xmlTextReaderBaseUri(w);
                if (base) xmlFree(base);
                xmlChar *val = xmlTextReaderValue(w);
                if (val) xmlFree(val);
                xmlChar *lookup = xmlTextReaderLookupNamespace(w, NULL);
                if (lookup) xmlFree(lookup);
            }
            xmlFreeTextReader(w);
        }
        xmlFreeDoc(d);
    }

    /* xmlReaderForDoc / xmlReaderForFd / xmlReaderForIO + xmlReaderNew* variants */
    xmlTextReaderPtr r3 = xmlReaderForDoc(BAD_CAST input, "doc.xml", NULL, 0);
    if (r3) {
        while (xmlTextReaderRead(r3) == 1) {}
        xmlFreeTextReader(r3);
    }
    /* xmlReaderNewMemory: reset an existing reader to new input. */
    xmlTextReaderPtr r4 = xmlReaderForMemory("<a/>", 4, NULL, NULL, 0);
    if (r4) {
        xmlReaderNewMemory(r4, "<b/>", 4, NULL, NULL, 0);
        xmlReaderNewDoc(r4, BAD_CAST "<c/>", NULL, NULL, 0);
        while (xmlTextReaderRead(r4) == 1) {}
        xmlFreeTextReader(r4);
    }

    /* Reader from FILE descriptor via tmpfile. */
    FILE *tf = tmpfile();
    if (tf) {
        fwrite(input, 1, strlen(input), tf);
        rewind(tf);
        int fd = fileno(tf);
        xmlTextReaderPtr r5 = xmlReaderForFd(fd, "fd.xml", NULL, 0);
        if (r5) {
            while (xmlTextReaderRead(r5) == 1) {}
            xmlFreeTextReader(r5);
        }
        fclose(tf);
    }

    /* More reader operations + advanced navigation. */
    xmlTextReaderPtr r6 = xmlReaderForMemory(input, (int)strlen(input),
        "in.xml", NULL, 0);
    if (r6) {
        while (xmlTextReaderRead(r6) == 1) {
            xmlTextReaderNormalization(r6);
            /* Use Next / NextSibling occasionally. */
            if (xmlTextReaderNodeType(r6) == XML_READER_TYPE_ELEMENT) {
                xmlTextReaderNext(r6);
                xmlTextReaderNextSibling(r6);
            }
        }
        xmlFreeTextReader(r6);
    }

    /* Reader from a temp file path (xmlReaderForFile + xmlNewTextReaderFilename). */
    char rtmpl[] = "/tmp/libxml2_rdr.XXXXXX";
    int rfd = mkstemp(rtmpl);
    if (rfd >= 0) {
        write(rfd, input, strlen(input));
        close(rfd);
        xmlTextReaderPtr rf = xmlReaderForFile(rtmpl, NULL, 0);
        if (rf) {
            while (xmlTextReaderRead(rf) == 1) {}
            xmlFreeTextReader(rf);
        }
        xmlTextReaderPtr rfn = xmlNewTextReaderFilename(rtmpl);
        if (rfn) {
            xmlTextReaderRead(rfn);
            xmlFreeTextReader(rfn);
        }
        /* xmlReaderNewFile (reuse existing reader). */
        xmlTextReaderPtr rr = xmlReaderForMemory("<a/>", 4, NULL, NULL, 0);
        if (rr) {
            xmlReaderNewFile(rr, rtmpl, NULL, 0);
            while (xmlTextReaderRead(rr) == 1) {}
            xmlFreeTextReader(rr);
        }
        unlink(rtmpl);
    }
    /* xmlReaderNewWalker reuses an existing reader. */
    xmlDocPtr wd = xmlReadMemory(input, (int)strlen(input), "w.xml", NULL, 0);
    if (wd) {
        xmlTextReaderPtr wr = xmlReaderForMemory("<x/>", 4, NULL, NULL, 0);
        if (wr) {
            xmlReaderNewWalker(wr, wd);
            while (xmlTextReaderRead(wr) == 1) {}
            xmlFreeTextReader(wr);
        }
        xmlFreeDoc(wd);
    }

    /* Error handlers + locator. */
    xmlTextReaderPtr re = xmlReaderForMemory(input, (int)strlen(input), "e.xml", NULL, 0);
    if (re) {
        xmlTextReaderSetErrorHandler(re, NULL, NULL);
        xmlTextReaderErrorFunc fn = NULL;
        void *arg = NULL;
        xmlTextReaderGetErrorHandler(re, &fn, &arg);
        xmlTextReaderSetStructuredErrorHandler(re, NULL, NULL);
        xmlTextReaderSetMaxAmplification(re, 10);
        while (xmlTextReaderRead(re) == 1) {
            /* xmlTextReaderPreserve / PreservePattern. */
            xmlTextReaderPreserve(re);
            xmlTextReaderXmlLang(re);
            xmlTextReaderReadAttributeValue(re);
        }
        xmlFreeTextReader(re);
    }

    /* xmlReaderForIO + xmlReaderNewIO with simple input callbacks. */
    {
        struct ReadCtx { const char *buf; size_t pos, len; };
        struct ReadCtx rc = {input, 0, strlen(input)};
        static int (*rd)(void*, char*, int) = NULL;
        (void)rd;
        /* Use libxml2's xmlIOHTTPMatch with a non-http url to skip — easier to
         * exercise: create a memory IO buffer through xmlParserInputBufferCreateMem. */
        xmlParserInputBufferPtr ib = xmlParserInputBufferCreateMem(input,
            (int)strlen(input), XML_CHAR_ENCODING_NONE);
        xmlOutputBufferPtr ob = xmlAllocOutputBuffer(NULL);
        if (ib && ob) {
            xmlTextReaderPtr rio = xmlNewTextReader(ib, "io.xml");
            if (rio) {
                while (xmlTextReaderRead(rio) == 1) {}
                xmlFreeTextReader(rio);
                ib = NULL; /* freed by reader */
            }
        }
        if (ib) xmlFreeParserInputBuffer(ib);
        if (ob) xmlOutputBufferClose(ob);
    }
}

/* -------------- encoding.c -------------- */
static void test_encoding(void) {
    /* Walk every named encoding libxml2 knows about. */
    const char *encs[] = {
        "UTF-8", "UTF-16", "UTF-16LE", "UTF-16BE", "ASCII",
        "ISO-8859-1", "ISO-8859-2", "ISO-8859-3", "ISO-8859-4",
        "ISO-8859-5", "ISO-8859-6", "ISO-8859-7", "ISO-8859-8",
        "ISO-8859-9", "ISO-8859-10", "ISO-8859-11", "ISO-8859-13",
        "ISO-8859-14", "ISO-8859-15", "ISO-8859-16",
    };
    /* Round-trip "ABCabc!@#" through each encoding's output then input handler. */
    const xmlChar *sample = BAD_CAST "ABCabc!@#~";
    for (size_t i = 0; i < sizeof(encs)/sizeof(*encs); i++) {
        xmlCharEncoding e = xmlParseCharEncoding(encs[i]);
        const char *en = xmlGetCharEncodingName(e);
        xmlCharEncodingHandlerPtr h = xmlFindCharEncodingHandler(encs[i]);
        if (!h) h = xmlGetCharEncodingHandler(e);
        (void)en;
        if (h && h->output) {
            unsigned char obuf[64] = {0};
            int olen = sizeof obuf;
            int ilen = (int)xmlStrlen(sample);
            h->output(obuf, &olen, sample, &ilen);
            if (h->input) {
                xmlChar ibuf[64] = {0};
                int xlen = sizeof ibuf;
                int rilen = olen;
                h->input(ibuf, &xlen, obuf, &rilen);
            }
        }
    }
    /* Aliases. */
    xmlAddEncodingAlias("UTF8-LIKE", "UTF-8");
    const char *resolved = xmlGetEncodingAlias("UTF8-LIKE");
    (void)resolved;
    xmlDelEncodingAlias("UTF8-LIKE");
    xmlCleanupEncodingAliases();

    /* BOM detection branch coverage. */
    const unsigned char bom_utf16le[] = {0xFF, 0xFE};
    const unsigned char bom_utf16be[] = {0xFE, 0xFF};
    const unsigned char bom_utf8[]    = {0xEF, 0xBB, 0xBF};
    const unsigned char ebcdic[]      = {0x4C, 0x6F, 0xA7, 0x94};
    const unsigned char ucs4_be[]     = {0x00, 0x00, 0x00, 0x3C};
    const unsigned char ucs4_le[]     = {0x3C, 0x00, 0x00, 0x00};
    xmlDetectCharEncoding(bom_utf16le, sizeof bom_utf16le);
    xmlDetectCharEncoding(bom_utf16be, sizeof bom_utf16be);
    xmlDetectCharEncoding(bom_utf8,    sizeof bom_utf8);
    xmlDetectCharEncoding(ebcdic,      sizeof ebcdic);
    xmlDetectCharEncoding(ucs4_be,     sizeof ucs4_be);
    xmlDetectCharEncoding(ucs4_le,     sizeof ucs4_le);

    /* Parse a document that DECLARES ISO-8859-1 — forces the encoding path. */
    static const char iso8859_xml[] =
        "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n"
        "<root attr=\"\xe9\xeb\xf1\">caf\xe9</root>\n";
    xmlDocPtr d = xmlReadMemory(iso8859_xml, sizeof iso8859_xml - 1, "iso.xml", NULL, 0);
    if (d) xmlFreeDoc(d);
}

/* -------------- entities.c -------------- */
static void test_entities(void) {
    xmlDocPtr d = xmlNewDoc(BAD_CAST "1.0");
    xmlNodePtr root = xmlNewNode(NULL, BAD_CAST "root");
    xmlDocSetRootElement(d, root);
    xmlAddDocEntity(d, BAD_CAST "myent",
        XML_INTERNAL_GENERAL_ENTITY, NULL, NULL, BAD_CAST "REPLACEMENT");
    xmlAddDocEntity(d, BAD_CAST "ext",
        XML_EXTERNAL_GENERAL_PARSED_ENTITY,
        BAD_CAST "//pub", BAD_CAST "http://sys", NULL);
    xmlAddDocEntity(d, BAD_CAST "pe",
        XML_INTERNAL_PARAMETER_ENTITY, NULL, NULL,
        BAD_CAST "<!ELEMENT a EMPTY>");
    xmlEntityPtr got = xmlGetDocEntity(d, BAD_CAST "myent");
    xmlEntityPtr amp = xmlGetPredefinedEntity(BAD_CAST "amp");
    xmlEntityPtr pe  = xmlGetParameterEntity(d, BAD_CAST "pe");
    (void)got;(void)amp;(void)pe;
    /* xmlEncodeEntitiesReentrant + xmlEncodeSpecialChars + xmlCopyEntity. */
    xmlChar *enc = xmlEncodeEntitiesReentrant(d, BAD_CAST "a&b<c>d");
    if (enc) xmlFree(enc);
    xmlChar *spec = xmlEncodeSpecialChars(d, BAD_CAST "with\nnewline");
    if (spec) xmlFree(spec);
    /* SAX2 default handler init + simple inspection. */
    xmlSAXDefaultVersion(2);
    xmlSAXHandler sax2;
    memset(&sax2, 0, sizeof(sax2));
    xmlDefaultSAXHandlerInit();
    xmlSAX2InitDefaultSAXHandler(&sax2, 0);
    /* Parse a doc so SAX2 ctx-inspection funcs have a populated parser. */
    xmlParserCtxtPtr sp = xmlCreateMemoryParserCtxt(
        "<?xml version='1.0' standalone='yes'?><root/>", 47);
    if (sp) {
        xmlParseDocument(sp);
        xmlSAX2GetColumnNumber(sp);
        xmlSAX2GetLineNumber(sp);
        xmlSAX2GetParameterEntity(sp, BAD_CAST "amp");
        xmlSAX2GetPublicId(sp);
        xmlSAX2GetSystemId(sp);
        xmlSAX2HasExternalSubset(sp);
        xmlSAX2HasInternalSubset(sp);
        xmlSAX2IsStandalone(sp);
        if (sp->myDoc) xmlFreeDoc(sp->myDoc);
        xmlFreeParserCtxt(sp);
    }
    /* xmlDumpEntityDecl. */
    xmlBufferPtr b = xmlBufferCreate();
    if (got) xmlDumpEntityDecl(b, got);
    xmlBufferFree(b);
    xmlFreeDoc(d);
}

/* -------------- valid.c -------------- */
static void test_valid(const char *input) {
    xmlDocPtr d = xmlReadMemory(input, (int)strlen(input), "in.xml", NULL,
        XML_PARSE_DTDLOAD | XML_PARSE_DTDATTR | XML_PARSE_DTDVALID);
    if (!d) return;
    xmlValidCtxt ctxt;
    memset(&ctxt, 0, sizeof(ctxt));
    ctxt.error = generic_err;
    ctxt.warning = generic_err;
    xmlValidateDocument(&ctxt, d);
    xmlValidateDtdFinal(&ctxt, d);
    xmlValidateOneElement(&ctxt, d, xmlDocGetRootElement(d));
    xmlValidateRoot(&ctxt, d);
    xmlValidateOneNamespace(&ctxt, d, xmlDocGetRootElement(d),
                            NULL, NULL, BAD_CAST "v");
    xmlValidateDocumentFinal(&ctxt, d);
    xmlGetID(d, BAD_CAST "a");
    xmlIsID(d, NULL, NULL);
    xmlGetRefs(d, BAD_CAST "x");
    xmlIsRef(d, NULL, NULL);
    xmlIsMixedElement(d, BAD_CAST "root");

    /* AddRef + RemoveRef explicit. */
    xmlNodePtr root = xmlDocGetRootElement(d);
    if (root && root->properties) {
        xmlAttrPtr attr = root->properties;
        xmlAddRef(&ctxt, d, BAD_CAST "myref", attr);
        xmlRemoveRef(d, attr);
    }
    /* DTD notation lookup. */
    if (d->intSubset) {
        xmlGetDtdNotationDesc(d->intSubset, BAD_CAST "gif");
    }
    /* xmlSprintfElementContent (deprecated but exported). */
    xmlElementContentPtr ec2 = xmlNewDocElementContent(d, BAD_CAST "elem", XML_ELEMENT_CONTENT_ELEMENT);
    if (ec2) {
        char buf2[256] = {0};
        xmlSprintfElementContent(buf2, ec2, 0);
        xmlFreeDocElementContent(d, ec2);
    }
    /* xmlValidateAttributeValue type combos. */
    xmlValidateAttributeValue(XML_ATTRIBUTE_CDATA, BAD_CAST "value");
    xmlValidateAttributeValue(XML_ATTRIBUTE_ID, BAD_CAST "myid");
    xmlValidateAttributeValue(XML_ATTRIBUTE_NMTOKEN, BAD_CAST "tok");
    xmlValidateAttributeValue(XML_ATTRIBUTE_NMTOKENS, BAD_CAST "a b c");
    xmlValidateAttributeValue(XML_ATTRIBUTE_ENTITY, BAD_CAST "pic");
    /* xmlValidNormalizeAttributeValue. */
    xmlValidNormalizeAttributeValue(d, root, BAD_CAST "id", BAD_CAST " trimmed ");
    /* xmlIsID + variants on real attr. */
    if (root && root->properties) {
        xmlIsID(d, root, root->properties);
    }
    /* String-form validators. */
    xmlValidateNameValue(BAD_CAST "valid-name");
    xmlValidateNamesValue(BAD_CAST "name1 name2");
    xmlValidateNmtokenValue(BAD_CAST "token");
    xmlValidateNmtokensValue(BAD_CAST "tok1 tok2");
    /* Element content tree helpers. */
    xmlElementContentPtr ne1 = xmlNewElementContent(BAD_CAST "child",
                                                     XML_ELEMENT_CONTENT_ELEMENT);
    if (ne1) xmlFreeElementContent(ne1);
    /* Walk-style validate the doc. */
    xmlValidateDtd(&ctxt, d, d->intSubset);
    /* xmlValidatePushElement / xmlValidatePushCData / PopElement (read API).*/
    if (root) {
        xmlValidatePushElement(&ctxt, d, root, root->name);
        xmlValidatePushCData(&ctxt, BAD_CAST "data", 4);
        xmlValidatePopElement(&ctxt, d, root, root->name);
    }
    /* xmlValidateNotationDecl + xmlValidateRef. */
    if (d->intSubset && d->intSubset->notations) {
        xmlNotationPtr nt = xmlHashLookup(d->intSubset->notations, BAD_CAST "gif");
        if (nt) xmlValidateNotationDecl(&ctxt, d, nt);
    }
    /* xmlNewValidCtxt + xmlFreeValidCtxt. */
    xmlValidCtxtPtr vc = xmlNewValidCtxt();
    if (vc) xmlFreeValidCtxt(vc);

    /* xmlValidNormalizeString is private; skip. */
    /* xmlCopyNotation / xmlFreeNotation / xmlDumpEnumeration / xmlDumpNotationDecl
     * are private symbols in this libxml2 build; skip. */
    /* xmlValidateNotationUse. */
    xmlValidateNotationUse(&ctxt, d, BAD_CAST "gif");
    /* Notation decl + enumeration + element content. */
    xmlEnumerationPtr en = xmlCreateEnumeration(BAD_CAST "val");
    if (en) xmlFreeEnumeration(en);
    xmlElementContentPtr ec = xmlNewDocElementContent(d, BAD_CAST "elem", XML_ELEMENT_CONTENT_ELEMENT);
    if (ec) {
        xmlChar buf[128] = {0};
        xmlSnprintfElementContent(buf, sizeof buf, ec, 0);
        xmlFreeDocElementContent(d, ec);
    }
    /* Dump tables. */
    xmlBufferPtr b = xmlBufferCreate();
    if (d->intSubset) {
        xmlDumpElementTable(b, d->intSubset->elements);
        xmlDumpAttributeTable(b, d->intSubset->attributes);
        xmlDumpNotationTable(b, d->intSubset->notations);
    }
    xmlBufferFree(b);
    xmlFreeDoc(d);
}

/* -------------- list.c -------------- */
static int list_walker(const void *data, void *user) {
    (void)data; (void)user; return 1;
}
static void test_list(void) {
    xmlListPtr l = xmlListCreate(NULL, NULL);
    if (l) {
        long ids[] = {3, 1, 4, 1, 5, 9, 2, 6, 5, 3};
        for (size_t i = 0; i < sizeof(ids)/sizeof(*ids); i++) {
            xmlListInsert(l, (void *)ids[i]);
        }
        xmlListSize(l);
        xmlListEmpty(l);
        for (size_t i = 0; i < sizeof(ids)/sizeof(*ids); i++) {
            xmlListAppend(l, (void *)ids[i]);
        }
        xmlListWalk(l, list_walker, NULL);
        xmlListReverseWalk(l, list_walker, NULL);
        xmlListReverse(l);
        xmlListSort(l);
        xmlListPushFront(l, (void *)42L);
        xmlListPushBack(l, (void *)84L);
        xmlListFront(l);
        xmlListEnd(l);
        /* Search variants (link-search not exported in this libxml2). */
        xmlListSearch(l, (void *)4L);
        xmlListReverseSearch(l, (void *)5L);
        xmlListRemoveFirst(l, (void *)1L);
        xmlListRemoveLast(l, (void *)5L);
        xmlListRemoveAll(l, (void *)3L);
        xmlListPopFront(l);
        xmlListPopBack(l);
        xmlListClear(l);
        xmlListDelete(l);
    }
}

/* -------------- hash.c / dict.c -------------- */
static void hash_walker(void *payload, void *data, const xmlChar *name) {
    (void)payload; (void)data; (void)name;
}
static void test_hash_dict(void) {
    xmlHashTablePtr h = xmlHashCreate(16);
    if (h) {
        const char *keys[] = {"a", "b", "c", "d", "e"};
        for (size_t i = 0; i < 5; i++) {
            xmlHashAddEntry(h, BAD_CAST keys[i], (void*)(uintptr_t)(i+1));
            xmlHashUpdateEntry(h, BAD_CAST keys[i], (void*)(uintptr_t)i, NULL);
        }
        xmlHashSize(h);
        xmlHashLookup(h, BAD_CAST "a");
        xmlHashScan(h, hash_walker, NULL);
        xmlHashRemoveEntry(h, BAD_CAST "a", NULL);
        xmlHashFree(h, NULL);
    }

    xmlDictPtr d = xmlDictCreate();
    if (d) {
        const xmlChar *interned = xmlDictLookup(d, BAD_CAST "hello", -1);
        const xmlChar *interned2 = xmlDictLookup(d, BAD_CAST "hello", -1);
        (void)interned; (void)interned2;
        xmlDictSize(d);
        xmlDictOwns(d, interned);
        xmlDictReference(d);
        xmlDictFree(d);
        xmlDictFree(d); /* second free decrements ref, hits the ref-counted path */
    }
}

/* -------------- xmlstring.c -------------- */
static void test_xmlstring(void) {
    xmlChar *dup = xmlStrdup(BAD_CAST "hello");
    xmlChar *part = xmlStrndup(BAD_CAST "hello", 3);
    int len = xmlStrlen(dup);
    int cmp = xmlStrcmp(dup, BAD_CAST "hello");
    int ncmp = xmlStrncmp(dup, BAD_CAST "hel", 3);
    int caseEq = xmlStrcasecmp(dup, BAD_CAST "HELLO");
    int chreq = xmlStrEqual(dup, BAD_CAST "hello");
    const xmlChar *chr = xmlStrchr(dup, 'l');
    const xmlChar *str = xmlStrstr(dup, BAD_CAST "ell");
    xmlChar *cat = xmlStrcat(dup, BAD_CAST " world");
    xmlChar *ncat = xmlStrncatNew(BAD_CAST "x", BAD_CAST "yz", 2);
    /* Substring extraction + case-insensitive search. */
    xmlChar *sub = xmlStrsub(BAD_CAST "abcdef", 1, 3);
    const xmlChar *ci = xmlStrcasestr(BAD_CAST "Hello World", BAD_CAST "WORLD");
    /* UTF-8 helpers. */
    xmlCheckUTF8((const unsigned char *)"héllo");
    xmlUTF8Size(BAD_CAST "é");
    xmlUTF8Charcmp(BAD_CAST "é", BAD_CAST "é");
    xmlUTF8Strlen(BAD_CAST "héllo");
    xmlUTF8Strsize(BAD_CAST "héllo", 5);
    /* sprintf-style. */
    xmlChar pbuf[64];
    xmlStrPrintf(pbuf, sizeof pbuf, "n=%d", 42);
    /* xmlEscapeFormatString not exported in this libxml2 build; skip. */
    (void)len; (void)cmp; (void)ncmp; (void)caseEq; (void)chreq; (void)chr; (void)str;
    (void)ci;
    xmlFree(cat); xmlFree(part); xmlFree(ncat); if (sub) xmlFree(sub);
}

/* -------------- uri.c -------------- */
static void test_uri(void) {
    xmlURIPtr u = xmlParseURI("http://user:pass@example.com:8080/path?q=v#frag");
    if (u) {
        xmlChar *back = xmlSaveUri(u);
        if (back) xmlFree(back);
        xmlFreeURI(u);
    }
    xmlChar *resolved = xmlBuildURI(BAD_CAST "rel/path", BAD_CAST "http://base/dir/");
    if (resolved) xmlFree(resolved);
    xmlChar *abs = xmlBuildRelativeURI(BAD_CAST "http://a/b/c", BAD_CAST "http://a/b/");
    if (abs) xmlFree(abs);
    xmlChar *unescaped = xmlURIUnescapeString("hello%20world", -1, NULL);
    if (unescaped) xmlFree(unescaped);
    xmlChar *escaped = xmlURIEscape(BAD_CAST "/path with space");
    if (escaped) xmlFree(escaped);
    xmlChar *escapeurl = xmlURIEscapeStr(BAD_CAST "abc def", BAD_CAST "/");
    if (escapeurl) xmlFree(escapeurl);
}

/* -------------- chvalid.c -------------- */
static void test_chvalid(void) {
    /* Walk a few code points through each char-class table. */
    for (unsigned int cp = 0; cp < 256; cp++) {
        xmlIsBaseCharQ(cp);
        xmlIsCharQ(cp);
        xmlIsCombiningQ(cp);
        xmlIsDigitQ(cp);
        xmlIsExtenderQ(cp);
        xmlIsIdeographicQ(cp);
        xmlIsPubidCharQ(cp);
    }
    /* Direct table tests against group descriptors — hit many code points
     * to ensure both short-range and long-range searches execute. */
    for (unsigned int cp = 0x20; cp < 0x10000; cp += 17) {
        xmlCharInRange(cp, &xmlIsBaseCharGroup);
        xmlCharInRange(cp, &xmlIsCharGroup);
        xmlCharInRange(cp, &xmlIsCombiningGroup);
        xmlCharInRange(cp, &xmlIsDigitGroup);
        xmlCharInRange(cp, &xmlIsExtenderGroup);
        xmlCharInRange(cp, &xmlIsIdeographicGroup);
    }
    /* Also call the predicate functions directly. */
    for (unsigned int cp = 0x20; cp < 0x10000; cp += 31) {
        xmlIsBaseCharQ(cp);
        xmlIsCharQ(cp);
        xmlIsCombiningQ(cp);
        xmlIsDigitQ(cp);
        xmlIsExtenderQ(cp);
        xmlIsIdeographicQ(cp);
        xmlIsPubidCharQ(cp);
    }
    xmlIsBaseChar(0x41);
    xmlIsBlank(0x20);
    xmlIsChar(0xA);
    xmlIsCombining(0x300);
    xmlIsDigit(0x39);
    xmlIsExtender(0xB7);
    xmlIsIdeographic(0x4E00);
    xmlIsPubidChar(0x41);
}

/* -------------- threads.c + globals.c -------------- */
static void test_threads_globals(void) {
    xmlMutexPtr m = xmlNewMutex();
    if (m) {
        xmlMutexLock(m);
        xmlMutexUnlock(m);
        xmlFreeMutex(m);
    }
    xmlRMutexPtr rm = xmlNewRMutex();
    if (rm) {
        xmlRMutexLock(rm);
        xmlRMutexLock(rm); /* recursive */
        xmlRMutexUnlock(rm);
        xmlRMutexUnlock(rm);
        xmlFreeRMutex(rm);
    }
    /* globals state. */
    xmlInitializeGlobalState(xmlGetGlobalState());
    xmlIsMainThread();
    /* Touch every xmlThrDef* setter (thread-default versions). */
    xmlThrDefBufferAllocScheme(XML_BUFFER_ALLOC_DOUBLEIT);
    xmlThrDefDefaultBufferSize(4096);
    xmlThrDefDoValidityCheckingDefaultValue(0);
    xmlThrDefGetWarningsDefaultValue(1);
    xmlThrDefIndentTreeOutput(1);
    xmlThrDefKeepBlanksDefaultValue(1);
    xmlThrDefLineNumbersDefaultValue(0);
    xmlThrDefLoadExtDtdDefaultValue(0);
    xmlThrDefParserDebugEntities(0);
    xmlThrDefPedanticParserDefaultValue(0);
    xmlThrDefSaveNoEmptyTags(0);
    xmlThrDefSubstituteEntitiesDefaultValue(0);
    xmlThrDefTreeIndentString("  ");
    xmlThrDefSetGenericErrorFunc(NULL, generic_err);
    xmlThrDefSetStructuredErrorFunc(NULL, structured_err);
    xmlThrDefRegisterNodeDefault(NULL);
    xmlThrDefDeregisterNodeDefault(NULL);
}

/* -------------- error.c -------------- */
static void test_error(void) {
    xmlSetStructuredErrorFunc(NULL, structured_err);
    xmlSetGenericErrorFunc(NULL, generic_err);
    xmlGetLastError();
    xmlResetLastError();
    xmlCtxtResetLastError(NULL);

    /* Trigger errors via deliberate parse failures. */
    xmlDocPtr d1 = xmlReadMemory("not xml", 7, "bad.xml", NULL,
                                 XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (d1) xmlFreeDoc(d1);
    xmlDocPtr d2 = xmlReadMemory("<a><b/>", 8, "bad.xml", NULL,
                                 XML_PARSE_RECOVER | XML_PARSE_NOERROR);
    if (d2) xmlFreeDoc(d2);
    /* xmlCopyError / xmlResetError. */
    xmlErrorPtr last = xmlGetLastError();
    if (last) {
        xmlError tgt;
        memset(&tgt, 0, sizeof(tgt));
        xmlCopyError(last, &tgt);
        xmlResetError(&tgt);
    }
}

/* -------------- xmlmemory.c -------------- */
static void test_memory(void) {
    /* Allocate via the libxml2 allocator hooks. */
    void *p = xmlMalloc(64);
    p = xmlRealloc(p, 128);
    xmlChar *s = xmlMemStrdup("hello");
    xmlFree(p);
    xmlFree(s);
    int mused = xmlMemUsed();
    int mblocks = xmlMemBlocks();
    (void)mused; (void)mblocks;
    /* Atomic / per-thread + memory contract. */
    xmlMemoryDump();
    xmlMemSize(NULL);
    void *p2 = xmlMallocAtomic(32);
    p2 = xmlMemRealloc(p2, 64);
    void *p3 = xmlMallocLoc(16, "drv", 0);
    void *p4 = xmlReallocLoc(p3, 32, "drv", 0);
    char *p5 = xmlMemoryStrdup("dup");
    xmlMemFree(p2);
    xmlMemFree(p4);
    xmlMemFree(p5);

    /* Memory display + Init/Cleanup + GC hooks. */
    FILE *mf = tmpfile();
    if (mf) {
        xmlMemDisplay(mf);
        xmlMemDisplayLast(mf, 1024);
        xmlMemShow(mf, 16);
        fclose(mf);
    }
    /* MallocAtomicLoc / xmlMemGet (read current allocators). */
    void *p6 = xmlMallocAtomicLoc(16, "drv", 0);
    xmlMemFree(p6);
    xmlMallocFunc gm = NULL;
    xmlReallocFunc gr = NULL;
    xmlFreeFunc gf = NULL;
    xmlStrdupFunc gs = NULL;
    xmlMemGet(&gm, &gr, &gf, &gs);
    /* xmlGcMemGet (debugging variant). */
    xmlMallocFunc gma = NULL;
    xmlMemGet(&gma, NULL, NULL, NULL);
    /* xmlInitMemory + xmlCleanupMemory (safe to call again). */
    xmlInitMemory();
}

/* Cover several error.c reporting helpers via a controlled parse failure. */
static void test_more_error(const char *input) {
    /* xmlCtxtGetLastError requires a context. */
    xmlParserCtxtPtr c = xmlNewParserCtxt();
    if (c) {
        xmlReadMemory(input, (int)strlen(input), "in.xml", NULL, 0);
        xmlCtxtGetLastError(c);
        xmlFreeParserCtxt(c);
    }
    /* xmlParserWarning / xmlParserError / xmlParserValidityError / Warning:
     * These need an xmlParserCtxtPtr and a format string. Run a deliberately
     * broken parse that hits the warning paths. */
    xmlDocPtr bad = xmlReadMemory("<a></b>", 7, "bad.xml", NULL,
                                  XML_PARSE_RECOVER | XML_PARSE_NOERROR);
    if (bad) xmlFreeDoc(bad);
    /* xmlParserPrintFileInfo / xmlParserPrintFileContext: these expect a
     * non-null xmlParserInput. Safe to skip; they're already partially
     * triggered by parser errors above. */
}

int main(int argc, char *argv[]) {
    xmlInitParser();
    xmlInitGlobals();
    xmlInitThreads();
    LIBXML_TEST_VERSION;

    test_error();
    test_memory();
    test_more_error(INLINE_XML);
    test_chvalid();
    test_xmlstring();
    test_uri();
    test_list();
    test_hash_dict();
    test_encoding();
    test_entities();
    test_threads_globals();

    /* Read input if provided, otherwise use INLINE_XML for every API cluster. */
    const char *input = INLINE_XML;
    char *malloc_input = NULL;
    size_t input_len = 0;
    if (argc >= 2) {
        FILE *f = fopen(argv[1], "rb");
        if (f) {
            fseek(f, 0, SEEK_END);
            long sz = ftell(f);
            fseek(f, 0, SEEK_SET);
            malloc_input = malloc(sz + 1);
            input_len = fread(malloc_input, 1, sz, f);
            malloc_input[input_len] = 0;
            fclose(f);
            input = malloc_input;
        }
    }

    test_parser(input);
    test_tree();
    test_save(input);
    test_xpath(input);
    test_reader(input);
    test_valid(INLINE_XML);  /* needs the DTD-bearing inline doc */

    if (malloc_input) free(malloc_input);

    xmlCleanupParser();
    xmlCleanupThreads();
    xmlCleanupGlobals();
    return 0;
}
