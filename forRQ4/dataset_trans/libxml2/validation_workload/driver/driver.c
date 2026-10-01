/*
 * validation driver for libxml2 — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves.
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     parse_suite   : xmlReadFile each path in the manifest, traverse the
 *                     tree, dump via xmlDocDumpFormatMemory + xmlsave
 *     html_suite    : htmlReadFile each path, traverse
 *     sax_suite     : SAX2 callback parse of each path (event counting)
 *     reader_suite  : xmlreader streaming walk of each path
 *     valid_suite   : DTD-validating parse of each path (fold validity)
 *     xpath_op      : XPath expression set over one document
 *     writer_op     : xmlwriter document generation into a buffer
 *     dict_uri_str  : dict/hash/string/URI utility surface over input words
 *     schema_suite  : XSD schema parse + instance validation (pairs manifest)
 *     relaxng_suite : RelaxNG parse + validation (pairs manifest)
 *     c14n_op       : canonicalization of one document
 *
 * Manifests hold one path per line (pairs manifests: "<schema> <instance>").
 * Inputs come from libxml2's OWN upstream test suite (real provenance).
 * Output: ONE digest line on stdout (project convention). libxml2 error
 * output goes to stderr (default handlers) and is not part of the oracle.
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <libxml/parser.h>
#include <libxml/HTMLparser.h>
#include <libxml/HTMLtree.h>
#include <libxml/tree.h>
#include <libxml/xmlsave.h>
#include <libxml/xpath.h>
#include <libxml/xmlreader.h>
#include <libxml/xmlwriter.h>
#include <libxml/dict.h>
#include <libxml/hash.h>
#include <libxml/uri.h>
#include <libxml/xmlstring.h>
#include <libxml/chvalid.h>
#include <libxml/xmlschemas.h>
#include <libxml/relaxng.h>
#include <libxml/c14n.h>
#include <libxml/xmlunicode.h>
#include <libxml/list.h>
#include <libxml/xpathInternals.h>
#include <libxml/xpointer.h>
#include <libxml/xinclude.h>
#include <libxml/xlink.h>
#include <libxml/catalog.h>
#include <libxml/xmlmemory.h>
#include <libxml/threads.h>
#include <libxml/schematron.h>
#include <libxml/debugXML.h>
#include <libxml/encoding.h>
#include <libxml/entities.h>
#include <libxml/xmlerror.h>
#include <libxml/parserInternals.h>
#include <libxml/xmlIO.h>
#include <libxml/SAX2.h>
#include <libxml/valid.h>
#include <libxml/xmlautomata.h>
#include <libxml/xmlregexp.h>
#include <libxml/pattern.h>
#include <libxml/xmlmodule.h>
#include <libxml/xmlschemastypes.h>
#include <libxml/globals.h>
#include <stdarg.h>
#include <fcntl.h>
#include <unistd.h>

/* 4-lane xor-rotate digest (project convention) */
static inline uint64_t rotl64(uint64_t x, int r) { return (x << r) | (x >> (64 - r)); }

static uint64_t digest64(uint64_t seed, const uint8_t *p, size_t n) {
    uint64_t l0 = seed ^ 0xcbf29ce484222325ULL;
    uint64_t l1 = 0x84222325cbf29ce4ULL;
    uint64_t l2 = 0x9ce484222325cbf2ULL;
    uint64_t l3 = 0x2325cbf29ce48422ULL;
    size_t i = 0;
    for (; i + 32 <= n; i += 32) {
        uint64_t w0, w1, w2, w3;
        memcpy(&w0, p + i,      8);
        memcpy(&w1, p + i + 8,  8);
        memcpy(&w2, p + i + 16, 8);
        memcpy(&w3, p + i + 24, 8);
        l0 = rotl64(l0 ^ w0, 17);
        l1 = rotl64(l1 ^ w1, 19);
        l2 = rotl64(l2 ^ w2, 23);
        l3 = rotl64(l3 ^ w3, 29);
    }
    for (; i < n; i++)
        l0 = rotl64(l0 ^ p[i], 11);
    uint64_t h = l0 * 0x9e3779b97f4a7c15ULL;
    h ^= rotl64(l1, 1); h *= 0x9e3779b97f4a7c15ULL;
    h ^= rotl64(l2, 2); h *= 0x9e3779b97f4a7c15ULL;
    h ^= rotl64(l3, 3); h *= 0x9e3779b97f4a7c15ULL;
    return h;
}

static uint8_t *read_file(const char *path, size_t *out_n) {
    FILE *f = fopen(path, "rb");
    if (!f) { fprintf(stderr, "cannot open %s\n", path); exit(2); }
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    fseek(f, 0, SEEK_SET);
    uint8_t *buf = malloc((size_t)n + 1);
    if (!buf || fread(buf, 1, (size_t)n, f) != (size_t)n) {
        fprintf(stderr, "cannot read %s\n", path); exit(2);
    }
    buf[n] = 0;
    fclose(f);
    *out_n = (size_t)n;
    return buf;
}

/* iterate manifest lines; cb returns folded digest */
typedef uint64_t (*line_fn)(uint64_t h, const char *path, const char *arg2);

static uint64_t for_each_line(const char *listpath, uint64_t h, line_fn fn,
                              uint64_t *count) {
    size_t n;
    uint8_t *buf = read_file(listpath, &n);
    char *p = (char *)buf, *end = (char *)buf + n;
    *count = 0;
    while (p < end) {
        char *nl = memchr(p, '\n', (size_t)(end - p));
        if (!nl) nl = end;
        *nl = 0;
        if (*p) {
            char *sp = strchr(p, ' ');
            if (sp) *sp = 0;
            h = fn(h, p, sp ? sp + 1 : NULL);
            (*count)++;
        }
        p = nl + 1;
        if (nl == end) break;
    }
    free(buf);
    return h;
}

/* fold a tree recursively: node types, names, attr names/values, text.
 * Only element children are descended -- entity-reference and DTD subtrees
 * hold shared/internal structures that are not part of the document walk. */
static uint64_t fold_node(uint64_t h, xmlNodePtr node) {
    for (; node; node = node->next) {
        uint64_t t = (uint64_t)node->type;
        h = digest64(h, (const uint8_t *)&t, sizeof(t));
        if (node->type == XML_ELEMENT_NODE || node->type == XML_PI_NODE ||
            node->type == XML_ENTITY_REF_NODE) {
            if (node->name)
                h = digest64(h, node->name, strlen((const char *)node->name));
        }
        if ((node->type == XML_TEXT_NODE || node->type == XML_CDATA_SECTION_NODE ||
             node->type == XML_COMMENT_NODE) && node->content)
            h = digest64(h, node->content, strlen((const char *)node->content));
        if (node->type == XML_ELEMENT_NODE) {
            for (xmlAttrPtr a = node->properties; a; a = a->next) {
                h = digest64(h, a->name, strlen((const char *)a->name));
                xmlChar *v = xmlGetProp(node, a->name);
                if (v) { h = digest64(h, v, strlen((const char *)v)); xmlFree(v); }
            }
            h = fold_node(h, node->children);
        }
    }
    return h;
}

static uint64_t parse_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    uint64_t ok = (doc != NULL);
    h = digest64(h, (const uint8_t *)&ok, sizeof(ok));
    if (!doc) return h;
    h = fold_node(h, xmlDocGetRootElement(doc));
    xmlChar *dump = NULL;
    int dump_n = 0;
    xmlDocDumpFormatMemory(doc, &dump, &dump_n, 1);
    if (dump) { h = digest64(h, dump, (size_t)dump_n); xmlFree(dump); }
    xmlBufferPtr xb = xmlBufferCreate();
    xmlSaveCtxtPtr sc = xmlSaveToBuffer(xb, "UTF-8", 0);
    if (sc) {
        xmlSaveDoc(sc, doc);
        xmlSaveClose(sc);
        h = digest64(h, xmlBufferContent(xb), (size_t)xmlBufferLength(xb));
    }
    xmlBufferFree(xb);
    xmlFreeDoc(doc);
    return h;
}

static uint64_t html_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    htmlDocPtr doc = htmlReadFile(path, NULL,
                                  HTML_PARSE_NOERROR | HTML_PARSE_NOWARNING);
    uint64_t ok = (doc != NULL);
    h = digest64(h, (const uint8_t *)&ok, sizeof(ok));
    if (!doc) return h;
    h = fold_node(h, xmlDocGetRootElement(doc));
    xmlChar *dump = NULL;
    int dump_n = 0;
    htmlDocDumpMemory(doc, &dump, &dump_n);
    if (dump) { h = digest64(h, dump, (size_t)dump_n); xmlFree(dump); }
    xmlFreeDoc(doc);
    return h;
}

/* SAX2 counting handlers */
struct sax_acc { uint64_t elems, chars, h; };
static void sax_start(void *ctx, const xmlChar *name, const xmlChar **attrs) {
    struct sax_acc *a = (struct sax_acc *)ctx;
    a->elems++;
    a->h = digest64(a->h, name, strlen((const char *)name));
    if (attrs)
        for (int i = 0; attrs[i]; i += 2)
            a->h = digest64(a->h, attrs[i], strlen((const char *)attrs[i]));
}
static void sax_end(void *ctx, const xmlChar *name) {
    (void)name;
    ((struct sax_acc *)ctx)->elems++;
}
static void sax_chars(void *ctx, const xmlChar *ch, int len) {
    struct sax_acc *a = (struct sax_acc *)ctx;
    a->chars += (uint64_t)len;
    a->h = digest64(a->h, ch, (size_t)len);
}

static uint64_t sax_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    xmlSAXHandler sh;
    memset(&sh, 0, sizeof(sh));
    sh.startElement = sax_start;
    sh.endElement = sax_end;
    sh.characters = sax_chars;
    struct sax_acc a = { 0, 0, h };
    int r = xmlSAXUserParseFile(&sh, &a, path);
    uint64_t meta[3] = { (uint64_t)r, a.elems, a.chars };
    return digest64(a.h, (const uint8_t *)meta, sizeof(meta));
}

static uint64_t reader_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    xmlTextReaderPtr rd = xmlReaderForFile(path, NULL,
                                           XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!rd) return h;
    int r;
    while ((r = xmlTextReaderRead(rd)) == 1) {
        uint64_t meta[2] = { (uint64_t)xmlTextReaderNodeType(rd),
                             (uint64_t)xmlTextReaderDepth(rd) };
        h = digest64(h, (const uint8_t *)meta, sizeof(meta));
        const xmlChar *nm = xmlTextReaderConstName(rd);
        if (nm) h = digest64(h, nm, strlen((const char *)nm));
        const xmlChar *val = xmlTextReaderConstValue(rd);
        if (val) h = digest64(h, val, strlen((const char *)val));
    }
    uint64_t rr = (uint64_t)r;
    h = digest64(h, (const uint8_t *)&rr, sizeof(rr));
    xmlFreeTextReader(rd);
    return h;
}

static uint64_t valid_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    xmlDocPtr doc = xmlReadFile(path, NULL,
        XML_PARSE_DTDVALID | XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    uint64_t ok = (doc != NULL);
    h = digest64(h, (const uint8_t *)&ok, sizeof(ok));
    if (doc) {
        h = fold_node(h, xmlDocGetRootElement(doc));
        xmlFreeDoc(doc);
    }
    return h;
}

static uint64_t schema_one(uint64_t h, const char *xsd, const char *xml) {
    if (!xml) return h;
    xmlSchemaParserCtxtPtr pc = xmlSchemaNewParserCtxt(xsd);
    if (!pc) return h;
    xmlSchemaPtr schema = xmlSchemaParse(pc);
    xmlSchemaFreeParserCtxt(pc);
    uint64_t got = (schema != NULL);
    h = digest64(h, (const uint8_t *)&got, sizeof(got));
    if (!schema) return h;
    xmlDocPtr doc = xmlReadFile(xml, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (doc) {
        xmlSchemaValidCtxtPtr vc = xmlSchemaNewValidCtxt(schema);
        if (vc) {
            uint64_t v = (uint64_t)(int64_t)xmlSchemaValidateDoc(vc, doc);
            h = digest64(h, (const uint8_t *)&v, sizeof(v));
            xmlSchemaFreeValidCtxt(vc);
        }
        xmlFreeDoc(doc);
    }
    xmlSchemaFree(schema);
    return h;
}

static uint64_t relaxng_one(uint64_t h, const char *rng, const char *xml) {
    if (!xml) return h;
    xmlRelaxNGParserCtxtPtr pc = xmlRelaxNGNewParserCtxt(rng);
    if (!pc) return h;
    xmlRelaxNGPtr rg = xmlRelaxNGParse(pc);
    xmlRelaxNGFreeParserCtxt(pc);
    uint64_t got = (rg != NULL);
    h = digest64(h, (const uint8_t *)&got, sizeof(got));
    if (!rg) return h;
    xmlDocPtr doc = xmlReadFile(xml, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (doc) {
        xmlRelaxNGValidCtxtPtr vc = xmlRelaxNGNewValidCtxt(rg);
        if (vc) {
            uint64_t v = (uint64_t)(int64_t)xmlRelaxNGValidateDoc(vc, doc);
            h = digest64(h, (const uint8_t *)&v, sizeof(v));
            xmlRelaxNGFreeValidCtxt(vc);
        }
        xmlFreeDoc(doc);
    }
    xmlRelaxNGFree(rg);
    return h;
}

static void run_suite(const char *op, const char *listpath, long iters, line_fn fn) {
    uint64_t h = 0, count = 0;
    for (long i = 0; i < iters; i++)
        h = for_each_line(listpath, h, fn, &count);
    printf("op=%s in=0 out=%llu iters=%ld digest=%016llx\n",
           op, (unsigned long long)count, iters, (unsigned long long)h);
}

static void run_xpath(const char *path, long iters) {
    static const char *exprs[] = {
        "//*", "count(//*)", "//@*", "string(/*)", "//*[position() mod 7 = 0]",
        "//*[@*]", "concat(name(/*), '-', count(//text()))",
        "sum(//*[string-length(name()) > 3]/string-length(name()))",
        "boolean(//comment())", "normalize-space(string(/*))",
    };
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!doc) { fprintf(stderr, "xpath parse failed\n"); exit(3); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlXPathContextPtr ctx = xmlXPathNewContext(doc);
        for (size_t e = 0; e < sizeof(exprs) / sizeof(exprs[0]); e++) {
            xmlXPathObjectPtr obj =
                xmlXPathEvalExpression((const xmlChar *)exprs[e], ctx);
            if (!obj) continue;
            uint64_t t = (uint64_t)obj->type;
            h = digest64(h, (const uint8_t *)&t, sizeof(t));
            switch (obj->type) {
            case XPATH_NODESET: {
                uint64_t sz = obj->nodesetval ? (uint64_t)obj->nodesetval->nodeNr : 0;
                h = digest64(h, (const uint8_t *)&sz, sizeof(sz));
                break;
            }
            case XPATH_NUMBER: {
                h = digest64(h, (const uint8_t *)&obj->floatval, sizeof(double));
                break;
            }
            case XPATH_BOOLEAN: {
                uint64_t b = (uint64_t)obj->boolval;
                h = digest64(h, (const uint8_t *)&b, sizeof(b));
                break;
            }
            case XPATH_STRING:
                if (obj->stringval)
                    h = digest64(h, obj->stringval,
                                 strlen((const char *)obj->stringval));
                break;
            default: break;
            }
            xmlXPathFreeObject(obj);
        }
        xmlXPathFreeContext(ctx);
    }
    xmlFreeDoc(doc);
    printf("op=xpath_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

static void run_writer(long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlBufferPtr xb = xmlBufferCreate();
        xmlTextWriterPtr w = xmlNewTextWriterMemory(xb, 0);
        if (!w) { fprintf(stderr, "writer failed\n"); exit(3); }
        xmlTextWriterStartDocument(w, "1.0", "UTF-8", NULL);
        xmlTextWriterStartElement(w, BAD_CAST "catalog");
        xmlTextWriterWriteAttribute(w, BAD_CAST "version", BAD_CAST "2.1");
        xmlTextWriterWriteComment(w, BAD_CAST "generated by validation driver");
        for (int k = 0; k < 40; k++) {
            xmlTextWriterStartElement(w, BAD_CAST "item");
            xmlTextWriterWriteFormatAttribute(w, BAD_CAST "id", "i%04d", k);
            xmlTextWriterWriteFormatElement(w, BAD_CAST "price", "%d.%02d", k, k % 100);
            xmlTextWriterStartElement(w, BAD_CAST "desc");
            xmlTextWriterWriteString(w, BAD_CAST "plain & <escaped> text");
            xmlTextWriterEndElement(w);
            if (k % 5 == 0) {
                xmlTextWriterWriteCDATA(w, BAD_CAST "raw <cdata> content");
                xmlTextWriterWritePI(w, BAD_CAST "proc", BAD_CAST "inst");
            }
            xmlTextWriterEndElement(w);
        }
        xmlTextWriterEndElement(w);
        xmlTextWriterEndDocument(w);
        xmlFreeTextWriter(w);
        h = digest64(h, xmlBufferContent(xb), (size_t)xmlBufferLength(xb));
        xmlBufferFree(xb);
    }
    printf("op=writer_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

static void run_dict_uri_str(const char *path, long iters) {
    size_t n;
    uint8_t *buf = read_file(path, &n);
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        /* dict + hash over whitespace-separated words of the input */
        xmlDictPtr dict = xmlDictCreate();
        xmlHashTablePtr ht = xmlHashCreate(64);
        char *p = (char *)buf, *end = (char *)buf + n;
        uint64_t words = 0;
        while (p < end && words < 5000) {
            while (p < end && (*p == ' ' || *p == '\n' || *p == '\t' || *p == '\r')) p++;
            char *w = p;
            while (p < end && *p != ' ' && *p != '\n' && *p != '\t' && *p != '\r') p++;
            if (p > w) {
                const xmlChar *interned = xmlDictLookup(dict, (const xmlChar *)w, (int)(p - w));
                if (interned) {
                    xmlHashAddEntry(ht, interned, (void *)(uintptr_t)(words + 1));
                    void *found = xmlHashLookup(ht, interned);
                    h = digest64(h, (const uint8_t *)&found, sizeof(found) > 8 ? 8 : sizeof(found));
                }
                words++;
            }
        }
        uint64_t meta[3] = { words, (uint64_t)xmlDictSize(dict), (uint64_t)xmlHashSize(ht) };
        h = digest64(h, (const uint8_t *)meta, sizeof(meta));
        xmlHashFree(ht, NULL);
        xmlDictFree(dict);

        /* URI surface */
        static const char *uris[] = {
            "http://user:pw@www.example.com:8080/a/b/c?x=1&y=2#frag",
            "../relative/path/file.xml", "urn:isbn:0451450523",
            "https://[2001:db8::1]/ipv6", "file:///tmp/x.xml",
        };
        for (size_t u = 0; u < sizeof(uris) / sizeof(uris[0]); u++) {
            xmlURIPtr pu = xmlParseURI(uris[u]);
            if (pu) {
                xmlChar *s = xmlSaveUri(pu);
                if (s) { h = digest64(h, s, strlen((const char *)s)); xmlFree(s); }
                xmlFreeURI(pu);
            }
            xmlChar *built = xmlBuildURI((const xmlChar *)uris[u],
                                         (const xmlChar *)"http://base.example.com/dir/");
            if (built) { h = digest64(h, built, strlen((const char *)built)); xmlFree(built); }
        }

        /* string + chvalid surface */
        xmlChar *dup = xmlStrdup(BAD_CAST "hello libxml2 world");
        xmlChar *cat = xmlStrcat(dup, BAD_CAST " & more");
        uint64_t acc[6] = {
            (uint64_t)xmlStrlen(cat),
            (uint64_t)xmlStrcmp(cat, BAD_CAST "x"),
            (uint64_t)(uintptr_t)(void *)xmlStrstr(cat, BAD_CAST "libxml") ? 1u : 0u,
            (uint64_t)xmlIsChar_ch('A') + (uint64_t)xmlIsBlank_ch(' '),
            (uint64_t)xmlIsDigit_ch('7') + (uint64_t)xmlIsBaseChar_ch('z'),
            (uint64_t)xmlCharInRange(0x4E2D, &xmlIsBaseCharGroup),
        };
        h = digest64(h, (const uint8_t *)acc, sizeof(acc));
        xmlFree(cat);
    }
    free(buf);
    printf("op=dict_uri_str in=%zu out=0 iters=%ld digest=%016llx\n",
           n, iters, (unsigned long long)h);
}

static void run_c14n(const char *path, long iters) {
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!doc) { fprintf(stderr, "c14n parse failed\n"); exit(3); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlChar *out = NULL;
        int r = xmlC14NDocDumpMemory(doc, NULL, XML_C14N_1_0, NULL, 0, &out);
        if (r >= 0 && out) { h = digest64(h, out, (size_t)r); xmlFree(out); }
        uint64_t rr = (uint64_t)(int64_t)r;
        h = digest64(h, (const uint8_t *)&rr, sizeof(rr));
    }
    xmlFreeDoc(doc);
    printf("op=c14n_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* ------- coverage-expansion ops (see study.toml header) ---------------- */

/* the xmlUCSIsBlock/Cat name tables dispatch to every generated predicate */
static void run_unicode_tables(long iters) {
    static const char *blocks[] = {
        "AegeanNumbers","AlphabeticPresentationForms","Arabic","ArabicPresentationForms-A",
        "ArabicPresentationForms-B","Armenian","Arrows","BasicLatin","Bengali","Bopomofo",
        "BopomofoExtended","BoxDrawing","BraillePatterns","Buhid","ByzantineMusicalSymbols",
        "CJKCompatibility","CJKCompatibilityForms","CJKCompatibilityIdeographs",
        "CJKCompatibilityIdeographsSupplement","CJKRadicalsSupplement","CJKSymbolsandPunctuation",
        "CJKUnifiedIdeographs","CJKUnifiedIdeographsExtensionA","CJKUnifiedIdeographsExtensionB",
        "Cherokee","CombiningDiacriticalMarks","CombiningDiacriticalMarksforSymbols",
        "CombiningHalfMarks","ControlPictures","CurrencySymbols","CypriotSyllabary",
        "Cyrillic","CyrillicSupplement","Deseret","Devanagari","Dingbats","EnclosedAlphanumerics",
        "EnclosedCJKLettersandMonths","Ethiopic","GeneralPunctuation","GeometricShapes",
        "Georgian","Gothic","Greek","GreekExtended","Gujarati","Gurmukhi","HalfwidthandFullwidthForms",
        "HangulCompatibilityJamo","HangulJamo","HangulSyllables","Hanunoo","Hebrew","Hiragana",
        "IPAExtensions","IdeographicDescriptionCharacters","Kanbun","KangxiRadicals","Kannada",
        "Katakana","KatakanaPhoneticExtensions","Khmer","KhmerSymbols","Lao","Latin-1Supplement",
        "LatinExtended-A","LatinExtended-B","LatinExtendedAdditional","LetterlikeSymbols","Limbu",
        "LinearBIdeograms","LinearBSyllabary","Malayalam","MathematicalAlphanumericSymbols",
        "MathematicalOperators","MiscellaneousMathematicalSymbols-A","MiscellaneousMathematicalSymbols-B",
        "MiscellaneousSymbols","MiscellaneousSymbolsandArrows","MiscellaneousTechnical","Mongolian",
        "MusicalSymbols","Myanmar","NumberForms","Ogham","OldItalic","OpticalCharacterRecognition",
        "Oriya","Osmanya","PhoneticExtensions","PrivateUseArea","Runic","Shavian","Sinhala",
        "SmallFormVariants","SpacingModifierLetters","Specials","SuperscriptsandSubscripts",
        "SupplementalArrows-A","SupplementalArrows-B","SupplementalMathematicalOperators",
        "SupplementaryPrivateUseArea-A","SupplementaryPrivateUseArea-B","Syriac","Tagalog",
        "Tagbanwa","Tags","TaiLe","Tamil","Telugu","Thaana","Thai","Tibetan","Ugaritic",
        "UnifiedCanadianAboriginalSyllabics","VariationSelectors","VariationSelectorsSupplement",
        "YiRadicals","YiSyllables","YijingHexagramSymbols",
    };
    static const char *cats[] = {
        "C","Cc","Cf","Co","Cs","L","Ll","Lm","Lo","Lt","Lu","M","Mc","Me","Mn",
        "N","Nd","Nl","No","P","Pc","Pd","Pe","Pf","Pi","Po","Ps","S","Sc","Sk",
        "Sm","So","Z","Zl","Zp","Zs",
    };
    static const int cps[] = { 0x41, 0xE9, 0x391, 0x5D0, 0x4E2D, 0x3042, 0x1F600,
                               0x0660, 0x20AC, 0x2200, 0x2028, 0x10330 };
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        uint64_t acc = 0;
        for (size_t b = 0; b < sizeof(blocks) / sizeof(blocks[0]); b++)
            for (size_t c = 0; c < sizeof(cps) / sizeof(cps[0]); c++)
                acc = acc * 3 + (uint64_t)xmlUCSIsBlock(cps[c], blocks[b]);
        for (size_t k = 0; k < sizeof(cats) / sizeof(cats[0]); k++)
            for (size_t c = 0; c < sizeof(cps) / sizeof(cps[0]); c++)
                acc = acc * 3 + (uint64_t)xmlUCSIsCat(cps[c], cats[k]);
        h = digest64(h, (const uint8_t *)&acc, sizeof(acc));
    }
    printf("op=unicode_tables in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* full xmlreader accessor surface + walker + in-reader RelaxNG validation */
static uint64_t reader_full_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    xmlTextReaderPtr rd = xmlReaderForFile(path, NULL,
        XML_PARSE_NOERROR | XML_PARSE_NOWARNING | XML_PARSE_NONET);
    if (!rd) return h;
    xmlTextReaderSetParserProp(rd, XML_PARSER_LOADDTD, 1);
    int r;
    while ((r = xmlTextReaderRead(rd)) == 1) {
        uint64_t acc[8] = {
            (uint64_t)xmlTextReaderNodeType(rd), (uint64_t)xmlTextReaderDepth(rd),
            (uint64_t)xmlTextReaderHasAttributes(rd), (uint64_t)xmlTextReaderAttributeCount(rd),
            (uint64_t)xmlTextReaderIsEmptyElement(rd), (uint64_t)xmlTextReaderHasValue(rd),
            (uint64_t)xmlTextReaderReadState(rd), (uint64_t)xmlTextReaderIsDefault(rd),
        };
        h = digest64(h, (const uint8_t *)acc, sizeof(acc));
        const xmlChar *s;
        if ((s = xmlTextReaderConstLocalName(rd))) h = digest64(h, s, strlen((const char *)s));
        if ((s = xmlTextReaderConstPrefix(rd)))    h = digest64(h, s, strlen((const char *)s));
        if ((s = xmlTextReaderConstNamespaceUri(rd))) h = digest64(h, s, strlen((const char *)s));
        if ((s = xmlTextReaderConstBaseUri(rd)))   h = digest64(h, s, strlen((const char *)s));
        if ((s = xmlTextReaderConstXmlLang(rd)))   h = digest64(h, s, strlen((const char *)s));
        if (xmlTextReaderNodeType(rd) == 1 && xmlTextReaderHasAttributes(rd)) {
            if (xmlTextReaderMoveToFirstAttribute(rd) == 1) {
                do {
                    const xmlChar *an = xmlTextReaderConstName(rd);
                    const xmlChar *av = xmlTextReaderConstValue(rd);
                    if (an) h = digest64(h, an, strlen((const char *)an));
                    if (av) h = digest64(h, av, strlen((const char *)av));
                } while (xmlTextReaderMoveToNextAttribute(rd) == 1);
                xmlTextReaderMoveToElement(rd);
            }
            xmlChar *a0 = xmlTextReaderGetAttributeNo(rd, 0);
            if (a0) { h = digest64(h, a0, strlen((const char *)a0)); xmlFree(a0); }
        }
    }
    /* ByteConsumed depends on internal buffering state -- exercise the API
     * but keep it out of the digest */
    (void)xmlTextReaderByteConsumed(rd);
    uint64_t tail[2] = { (uint64_t)r, (uint64_t)xmlTextReaderIsValid(rd) };
    h = digest64(h, (const uint8_t *)tail, sizeof(tail));
    xmlFreeTextReader(rd);

    /* walker over a parsed doc + ReadInner/OuterXml + Expand + ReadString */
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (doc) {
        xmlTextReaderPtr w = xmlReaderWalker(doc);
        int steps = 0;
        while (w && xmlTextReaderRead(w) == 1 && steps < 8) {
            steps++;
            if (xmlTextReaderNodeType(w) == 1) {
                xmlChar *inner = xmlTextReaderReadInnerXml(w);
                if (inner) { h = digest64(h, inner, strlen((const char *)inner)); xmlFree(inner); }
                xmlChar *outer = xmlTextReaderReadOuterXml(w);
                if (outer) { h = digest64(h, outer, strlen((const char *)outer)); xmlFree(outer); }
                xmlChar *str = xmlTextReaderReadString(w);
                if (str) { h = digest64(h, str, strlen((const char *)str)); xmlFree(str); }
                xmlNodePtr ex = xmlTextReaderExpand(w);
                uint64_t got = (ex != NULL);
                h = digest64(h, (const uint8_t *)&got, sizeof(got));
                (void)xmlTextReaderNext(w);
            }
        }
        if (w) xmlFreeTextReader(w);
        xmlFreeDoc(doc);
    }
    return h;
}

/* DOM construction / manipulation / query surface (tree.c) */
static void run_dom_build(long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlDocPtr doc = xmlNewDoc(BAD_CAST "1.0");
        xmlNodePtr root = xmlNewNode(NULL, BAD_CAST "inventory");
        xmlDocSetRootElement(doc, root);
        xmlNsPtr ns = xmlNewNs(root, BAD_CAST "http://example.com/inv", BAD_CAST "inv");
        xmlSetProp(root, BAD_CAST "version", BAD_CAST "3");
        for (int k = 0; k < 24; k++) {
            xmlNodePtr item = xmlNewChild(root, ns, BAD_CAST "item", NULL);
            char idv[16];
            snprintf(idv, sizeof(idv), "id-%03d", k);
            xmlNewProp(item, BAD_CAST "id", BAD_CAST idv);
            xmlNewTextChild(item, NULL, BAD_CAST "name", BAD_CAST "widget & part");
            xmlNodePtr price = xmlNewChild(item, NULL, BAD_CAST "price", BAD_CAST "9.99");
            xmlAddChild(item, xmlNewComment(BAD_CAST "audit trail"));
            xmlAddChild(item, xmlNewCDataBlock(doc, BAD_CAST "<raw/>", 6));
            xmlAddSibling(price, xmlNewPI(BAD_CAST "check", BAD_CAST "true"));
            if (k % 4 == 0) xmlUnsetProp(item, BAD_CAST "id");
        }
        /* queries + copies + surgery */
        xmlNodePtr first = xmlFirstElementChild(root);
        xmlNodePtr last = xmlLastElementChild(root);
        uint64_t counts[3] = { (uint64_t)xmlChildElementCount(root),
                               (uint64_t)xmlGetLineNo(last),
                               (uint64_t)(xmlNextElementSibling(first) != NULL) };
        h = digest64(h, (const uint8_t *)counts, sizeof(counts));
        xmlChar *pathstr = xmlGetNodePath(last);
        if (pathstr) { h = digest64(h, pathstr, strlen((const char *)pathstr)); xmlFree(pathstr); }
        xmlChar *content = xmlNodeGetContent(first);
        if (content) { h = digest64(h, content, strlen((const char *)content)); xmlFree(content); }
        xmlNodePtr copy = xmlCopyNode(first, 1);
        xmlAddChild(root, copy);
        xmlNodePtr repl = xmlNewNode(NULL, BAD_CAST "replaced");
        xmlReplaceNode(xmlPreviousElementSibling(last), repl);
        xmlUnlinkNode(first);
        xmlFreeNode(first);
        xmlNodeSetContent(repl, BAD_CAST "new content");
        xmlNodeAddContent(repl, BAD_CAST " + appended");
        xmlChar *lang = xmlNodeGetLang(repl);
        if (lang) xmlFree(lang);
        xmlNodeSetLang(repl, BAD_CAST "en");
        xmlNodeSetBase(repl, BAD_CAST "http://example.com/base");
        xmlChar *base = xmlNodeGetBase(doc, repl);
        if (base) { h = digest64(h, base, strlen((const char *)base)); xmlFree(base); }
        xmlDocPtr dcopy = xmlCopyDoc(doc, 1);
        /* serialize both, fold */
        xmlChar *dump = NULL;
        int dn = 0;
        xmlDocDumpMemory(dcopy, &dump, &dn);
        if (dump) { h = digest64(h, dump, (size_t)dn); xmlFree(dump); }
        xmlBufferPtr nb = xmlBufferCreate();
        xmlNodeDump(nb, doc, xmlDocGetRootElement(doc), 0, 1);
        h = digest64(h, xmlBufferContent(nb), (size_t)xmlBufferLength(nb));
        xmlBufferFree(nb);
        xmlFreeDoc(dcopy);
        xmlFreeDoc(doc);
    }
    printf("op=dom_build in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* extended XPath: axes, string/number fns, unions, compiled exprs, variables */
static void xp_custom(xmlXPathParserContextPtr ctxt, int nargs) {
    (void)nargs;
    xmlXPathReturnNumber(ctxt, 42.0);
}
static void run_xpath_full(const char *path, long iters) {
    static const char *exprs[] = {
        "//item/ancestor::*", "//*/following-sibling::*[1]", "//*/preceding-sibling::*",
        "/descendant-or-self::node()/child::*", "//@* | //comment()",
        "substring(name(/*), 2, 3)", "translate('abc', 'abc', 'xyz')",
        "contains(string(/*), 'e')", "starts-with(name(/*), 's')",
        "string-length(normalize-space(string(/*)))",
        "count(//*[last()])", "//*[position() < 3]",
        "number('12.5') + floor(3.7) + ceiling(1.2) + round(2.5)",
        "not(false()) and true()", "$vwvar + 1", "vwfunc()",
        "//*[lang('en')]", "id('nosuch')", "local-name(//*[2]) != ''",
        "namespace-uri(/*)",
    };
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!doc) { fprintf(stderr, "xpath_full parse failed\n"); exit(3); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlXPathContextPtr ctx = xmlXPathNewContext(doc);
        xmlXPathRegisterVariable(ctx, BAD_CAST "vwvar", xmlXPathNewFloat(41.0));
        xmlXPathRegisterFunc(ctx, BAD_CAST "vwfunc", xp_custom);
        for (size_t e = 0; e < sizeof(exprs) / sizeof(exprs[0]); e++) {
            xmlXPathCompExprPtr comp = xmlXPathCompile(BAD_CAST exprs[e]);
            if (!comp) continue;
            xmlXPathObjectPtr obj = xmlXPathCompiledEval(comp, ctx);
            if (obj) {
                uint64_t t = (uint64_t)obj->type;
                h = digest64(h, (const uint8_t *)&t, sizeof(t));
                if (obj->type == XPATH_NODESET && obj->nodesetval) {
                    uint64_t sz = (uint64_t)obj->nodesetval->nodeNr;
                    h = digest64(h, (const uint8_t *)&sz, sizeof(sz));
                } else if (obj->type == XPATH_NUMBER) {
                    h = digest64(h, (const uint8_t *)&obj->floatval, sizeof(double));
                } else if (obj->type == XPATH_STRING && obj->stringval) {
                    h = digest64(h, obj->stringval, strlen((const char *)obj->stringval));
                } else if (obj->type == XPATH_BOOLEAN) {
                    uint64_t b = (uint64_t)obj->boolval;
                    h = digest64(h, (const uint8_t *)&b, sizeof(b));
                }
                xmlXPathFreeObject(obj);
            }
            xmlXPathFreeCompExpr(comp);
        }
        xmlXPathFreeContext(ctx);
    }
    xmlFreeDoc(doc);
    printf("op=xpath_full in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* extended writer: DTD parts, attribute staging, raw/base64, indentation */
static void run_writer_full(long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlBufferPtr xb = xmlBufferCreate();
        xmlTextWriterPtr w = xmlNewTextWriterMemory(xb, 0);
        xmlTextWriterSetIndent(w, 1);
        xmlTextWriterSetIndentString(w, BAD_CAST "  ");
        xmlTextWriterSetQuoteChar(w, '\x27');
        xmlTextWriterStartDocument(w, "1.0", "UTF-8", "yes");
        xmlTextWriterStartDTD(w, BAD_CAST "doc", NULL, BAD_CAST "doc.dtd");
        xmlTextWriterWriteDTDElement(w, BAD_CAST "doc", BAD_CAST "(item*)");
        xmlTextWriterWriteDTDAttlist(w, BAD_CAST "item", BAD_CAST "id CDATA #REQUIRED");
        xmlTextWriterWriteDTDEntity(w, 0, BAD_CAST "vw", NULL, NULL, NULL,
                                    BAD_CAST "validation");
        xmlTextWriterEndDTD(w);
        xmlTextWriterStartElement(w, BAD_CAST "doc");
        xmlTextWriterStartAttribute(w, BAD_CAST "staged");
        xmlTextWriterWriteString(w, BAD_CAST "attr-value");
        xmlTextWriterEndAttribute(w);
        xmlTextWriterWriteAttributeNS(w, BAD_CAST "v", BAD_CAST "n",
                                      BAD_CAST "http://example.com/ns", BAD_CAST "x");
        xmlTextWriterStartElementNS(w, BAD_CAST "v", BAD_CAST "child",
                                    BAD_CAST "http://example.com/ns");
        xmlTextWriterWriteRaw(w, BAD_CAST "<rawline/>");
        xmlTextWriterWriteBase64(w, "binary!", 0, 7);
        xmlTextWriterEndElement(w);
        xmlTextWriterStartComment(w);
        xmlTextWriterWriteString(w, BAD_CAST "streamed comment");
        xmlTextWriterEndComment(w);
        xmlTextWriterStartCDATA(w);
        xmlTextWriterWriteString(w, BAD_CAST "cdata body");
        xmlTextWriterEndCDATA(w);
        xmlTextWriterStartPI(w, BAD_CAST "target");
        xmlTextWriterWriteString(w, BAD_CAST "data");
        xmlTextWriterEndPI(w);
        xmlTextWriterEndDocument(w);
        xmlFreeTextWriter(w);
        h = digest64(h, xmlBufferContent(xb), (size_t)xmlBufferLength(xb));
        xmlBufferFree(xb);
    }
    printf("op=writer_full in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* push-mode parsing (xml + html) + html table queries */
static void run_push_parse(const char *path, long iters) {
    size_t n;
    uint8_t *buf = read_file(path, &n);
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlParserCtxtPtr ctxt = xmlCreatePushParserCtxt(NULL, NULL,
                                                        (const char *)buf,
                                                        n > 4 ? 4 : (int)n, "push.xml");
        for (size_t pos = 4; pos < n; pos += 512)
            xmlParseChunk(ctxt, (const char *)buf + pos,
                          (int)(n - pos < 512 ? n - pos : 512), 0);
        xmlParseChunk(ctxt, NULL, 0, 1);
        uint64_t ok = (ctxt->myDoc != NULL && ctxt->wellFormed);
        h = digest64(h, (const uint8_t *)&ok, sizeof(ok));
        if (ctxt->myDoc) {
            h = fold_node(h, xmlDocGetRootElement(ctxt->myDoc));
            xmlFreeDoc(ctxt->myDoc);
        }
        xmlFreeParserCtxt(ctxt);

        htmlParserCtxtPtr hc = htmlCreatePushParserCtxt(NULL, NULL, "<html><bo", 9,
                                                        "push.html", XML_CHAR_ENCODING_UTF8);
        htmlParseChunk(hc, "dy><p>hi<br>there</p></body></html>", 36, 0);
        htmlParseChunk(hc, NULL, 0, 1);
        if (hc->myDoc) {
            h = fold_node(h, xmlDocGetRootElement(hc->myDoc));
            xmlFreeDoc(hc->myDoc);
        }
        htmlFreeParserCtxt(hc);

        const htmlElemDesc *ed = htmlTagLookup(BAD_CAST "table");
        const htmlEntityDesc *en = htmlEntityLookup(BAD_CAST "amp");
        uint64_t tbl[3] = { ed ? (uint64_t)ed->isinline : 9,
                            en ? (uint64_t)en->value : 9,
                            (uint64_t)htmlIsScriptAttribute(BAD_CAST "onclick") };
        h = digest64(h, (const uint8_t *)tbl, sizeof(tbl));
    }
    free(buf);
    printf("op=push_parse in=%zu out=0 iters=%ld digest=%016llx\n",
           n, iters, (unsigned long long)h);
}

/* encodings: charset handlers, buffer conversion, ISO-8859/UTF-16 file parses */
static uint64_t enc_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    uint64_t ok = (doc != NULL);
    h = digest64(h, (const uint8_t *)&ok, sizeof(ok));
    if (doc) {
        xmlChar *dump = NULL;
        int dn = 0;
        xmlDocDumpFormatMemoryEnc(doc, &dump, &dn, "ISO-8859-1", 0);
        if (dump) { h = digest64(h, dump, (size_t)dn); xmlFree(dump); }
        xmlFreeDoc(doc);
    }
    return h;
}
static void run_encoding_op(const char *listpath, long iters) {
    uint64_t h = 0, count = 0;
    for (long i = 0; i < iters; i++) {
        h = for_each_line(listpath, h, enc_one, &count);
        static const char *encs[] = { "UTF-8", "UTF-16LE", "UTF-16BE",
                                      "ISO-8859-1", "ASCII", "ISO-8859-5" };
        for (size_t e = 0; e < sizeof(encs) / sizeof(encs[0]); e++) {
            xmlCharEncodingHandlerPtr hd = xmlFindCharEncodingHandler(encs[e]);
            uint64_t got = (hd != NULL);
            h = digest64(h, (const uint8_t *)&got, sizeof(got));
        }
        uint64_t enc[3] = {
            (uint64_t)xmlParseCharEncoding("UTF-16"),
            (uint64_t)xmlDetectCharEncoding((const unsigned char *)"\xFF\xFE<\0", 4),
            (uint64_t)(uintptr_t)(xmlGetCharEncodingName(XML_CHAR_ENCODING_8859_1) != NULL),
        };
        h = digest64(h, (const uint8_t *)enc, sizeof(enc));
    }
    printf("op=encoding_op in=0 out=%llu iters=%ld digest=%016llx\n",
           (unsigned long long)count, iters, (unsigned long long)h);
}

/* xmlList generic list surface */
static int list_walker(const void *data, void *user) {
    *(uint64_t *)user += (uint64_t)(uintptr_t)data;
    return 1;
}
static int list_cmp(const void *a, const void *b) {
    return (int)((intptr_t)a - (intptr_t)b);
}
static void run_list_op(long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlListPtr l = xmlListCreate(NULL, list_cmp);
        for (int k = 0; k < 40; k++)
            xmlListPushBack(l, (void *)(uintptr_t)((k * 37) % 41 + 1));
        xmlListPushFront(l, (void *)(uintptr_t)99);
        uint64_t acc = 0;
        xmlListWalk(l, list_walker, &acc);
        xmlListSort(l);
        xmlListReverse(l);
        uint64_t meta[4] = { acc, (uint64_t)xmlListSize(l),
                             (uint64_t)(xmlListFront(l) != NULL),
                             (uint64_t)(uintptr_t)xmlLinkGetData(xmlListFront(l)) };
        xmlListPopFront(l);
        xmlListPopBack(l);
        xmlListRemoveFirst(l, (void *)(uintptr_t)7);
        meta[1] += (uint64_t)xmlListEmpty(l);
        h = digest64(h, (const uint8_t *)meta, sizeof(meta));
        xmlListPtr dup = xmlListDup(l);
        xmlListMerge(dup, l);
        xmlListClear(dup);
        xmlListDelete(dup);
        xmlListDelete(l);
    }
    printf("op=list_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* structured error surface via a deliberately malformed parse */
static void run_error_op(long iters) {
    static const char bad[] = "<root><unclosed></root";
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlResetLastError();
        xmlDocPtr doc = xmlReadMemory(bad, sizeof(bad) - 1, "bad.xml", NULL,
                                      XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
        if (doc) xmlFreeDoc(doc);
        const xmlError *err = xmlGetLastError();
        if (err) {
            uint64_t meta[3] = { (uint64_t)err->code, (uint64_t)err->line,
                                 (uint64_t)err->domain };
            h = digest64(h, (const uint8_t *)meta, sizeof(meta));
            if (err->message)
                h = digest64(h, (const uint8_t *)err->message, strlen(err->message));
            xmlErrorPtr copy = malloc(sizeof(xmlError));
            memset(copy, 0, sizeof(xmlError));
            xmlCopyError((xmlErrorPtr)err, copy);
            uint64_t cc = (uint64_t)copy->code;
            h = digest64(h, (const uint8_t *)&cc, sizeof(cc));
            xmlResetError(copy);
            free(copy);
        }
    }
    printf("op=error_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* XInclude processing over upstream test docs */
static uint64_t xinclude_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!doc) return h;
    int subs = xmlXIncludeProcessFlags(doc, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    uint64_t s = (uint64_t)(int64_t)subs;
    h = digest64(h, (const uint8_t *)&s, sizeof(s));
    if (subs >= 0)
        h = fold_node(h, xmlDocGetRootElement(doc));
    xmlFreeDoc(doc);
    return h;
}

/* XPointer evaluation over one document */
static void run_xpointer_op(const char *path, long iters) {
    static const char *ptrs[] = {
        "xpointer(//*)", "xpointer(/child::*[1])", "xpointer(id('nope'))",
        "element(/1)", "element(/1/1)",
    };
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!doc) { fprintf(stderr, "xpointer parse failed\n"); exit(3); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlXPathContextPtr ctx = xmlXPtrNewContext(doc, NULL, NULL);
        for (size_t p = 0; p < sizeof(ptrs) / sizeof(ptrs[0]); p++) {
            xmlXPathObjectPtr obj = xmlXPtrEval(BAD_CAST ptrs[p], ctx);
            uint64_t got = obj ? (uint64_t)obj->type + 1 : 0;
            h = digest64(h, (const uint8_t *)&got, sizeof(got));
            if (obj) xmlXPathFreeObject(obj);
        }
        xmlXPathFreeContext(ctx);
    }
    xmlFreeDoc(doc);
    printf("op=xpointer_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* schematron validation over upstream pairs */
static uint64_t schematron_one(uint64_t h, const char *sct, const char *xml) {
    if (!xml) return h;
    xmlSchematronParserCtxtPtr pc = xmlSchematronNewParserCtxt(sct);
    if (!pc) return h;
    xmlSchematronPtr s = xmlSchematronParse(pc);
    xmlSchematronFreeParserCtxt(pc);
    uint64_t got = (s != NULL);
    h = digest64(h, (const uint8_t *)&got, sizeof(got));
    if (!s) return h;
    xmlDocPtr doc = xmlReadFile(xml, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (doc) {
        xmlSchematronValidCtxtPtr vc =
            xmlSchematronNewValidCtxt(s, XML_SCHEMATRON_OUT_QUIET);
        if (vc) {
            uint64_t v = (uint64_t)(int64_t)xmlSchematronValidateDoc(vc, doc);
            h = digest64(h, (const uint8_t *)&v, sizeof(v));
            xmlSchematronFreeValidCtxt(vc);
        }
        xmlFreeDoc(doc);
    }
    xmlSchematronFree(s);
    return h;
}

/* gzip-compressed XML I/O (xzlib) */
static uint64_t gz_one(uint64_t h, const char *path, const char *unused) {
    (void)unused;
    return parse_one(h, path, unused);
}

/* debugXML dumps into a scratch file, bytes folded (content is symbolic --
 * names/types/values -- no addresses) */
static void run_debug_op(const char *path, long iters) {
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!doc) { fprintf(stderr, "debug parse failed\n"); exit(3); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        const char *tmp = "/tmp/vw_libxml2_debug.txt";
        FILE *f = fopen(tmp, "w");
        if (!f) { fprintf(stderr, "cannot open scratch\n"); exit(2); }
        xmlDebugDumpDocument(f, doc);
        xmlDebugDumpDocumentHead(f, doc);
        xmlNodePtr root = xmlDocGetRootElement(doc);
        xmlDebugDumpNode(f, root, 2);
        xmlDebugDumpNodeList(f, root->children, 1);
        xmlDebugDumpOneNode(f, root, 1);
        if (root->properties) xmlDebugDumpAttr(f, root->properties, 1);
        xmlDebugDumpEntities(f, doc);
        xmlDebugDumpString(f, BAD_CAST "debug string probe");
        xmlDebugDumpDTD(f, doc->intSubset);
        fclose(f);
        size_t n;
        uint8_t *out = read_file(tmp, &n);
        h = digest64(h, out, n);
        free(out);
        uint64_t chk[2] = { (uint64_t)xmlDebugCheckDocument(NULL, doc),
                            (uint64_t)xmlLsCountNode(root) };
        h = digest64(h, (const uint8_t *)chk, sizeof(chk));
    }
    remove("/tmp/vw_libxml2_debug.txt");
    xmlFreeDoc(doc);
    printf("op=debug_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* XML catalog surface over the upstream test catalog */
static void run_catalog_op(const char *path, long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        xmlCatalogPtr cat = xmlNewCatalog(0);
        if (cat) {
            xmlACatalogAdd(cat, BAD_CAST "public",
                           BAD_CAST "-//VW//DTD test//EN", BAD_CAST "file:///tmp/vw.dtd");
            xmlACatalogAdd(cat, BAD_CAST "system",
                           BAD_CAST "http://example.com/s.dtd", BAD_CAST "file:///tmp/s.dtd");
            xmlACatalogAdd(cat, BAD_CAST "rewriteURI",
                           BAD_CAST "http://example.com/", BAD_CAST "file:///tmp/");
            xmlChar *r1 = xmlACatalogResolvePublic(cat, BAD_CAST "-//VW//DTD test//EN");
            xmlChar *r2 = xmlACatalogResolveSystem(cat, BAD_CAST "http://example.com/s.dtd");
            xmlChar *r3 = xmlACatalogResolveURI(cat, BAD_CAST "http://example.com/doc.xml");
            xmlChar *r4 = xmlACatalogResolve(cat, BAD_CAST "-//VW//DTD test//EN", NULL);
            xmlChar *rs[4] = { r1, r2, r3, r4 };
            for (int k = 0; k < 4; k++)
                if (rs[k]) { h = digest64(h, rs[k], strlen((const char *)rs[k])); xmlFree(rs[k]); }
            xmlACatalogRemove(cat, BAD_CAST "http://example.com/s.dtd");
            uint64_t e = (uint64_t)xmlCatalogIsEmpty(cat);
            h = digest64(h, (const uint8_t *)&e, sizeof(e));
            const char *tmp = "/tmp/vw_catalog_dump.xml";
            FILE *f = fopen(tmp, "w");
            if (f) { xmlACatalogDump(cat, f); fclose(f); }
            size_t n;
            uint8_t *out = read_file(tmp, &n);
            h = digest64(h, out, n);
            free(out);
            remove(tmp);
            xmlFreeCatalog(cat);
        }
        /* file-based load of the upstream test catalog */
        xmlCatalogPtr fc = xmlLoadACatalog(path);
        uint64_t got = (fc != NULL);
        h = digest64(h, (const uint8_t *)&got, sizeof(got));
        if (fc) {
            xmlChar *r = xmlACatalogResolveURI(fc, BAD_CAST "http://nosuch.example/x");
            if (r) { h = digest64(h, r, strlen((const char *)r)); xmlFree(r); }
            xmlFreeCatalog(fc);
        }
        (void)xmlCatalogGetDefaults();
        xmlCatalogSetDefaults(XML_CATA_ALLOW_ALL);
    }
    printf("op=catalog_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* direct public-API storm: xpath/tree utilities, validation API, memory,
 * threads, xlink defaults, entities, html helpers, encoding funcs */
static void run_api_storm(const char *path, long iters) {
    xmlDocPtr doc = xmlReadFile(path, NULL, XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!doc) { fprintf(stderr, "api_storm parse failed\n"); exit(3); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        /* --- xpath object/nodeset utilities --- */
        xmlNodePtr root = xmlDocGetRootElement(doc);
        xmlNodeSetPtr ns = xmlXPathNodeSetCreate(root);
        xmlXPathNodeSetAdd(ns, (xmlNodePtr)doc);
        xmlXPathNodeSetAddUnique(ns, root);
        uint64_t nsl[2] = { (uint64_t)xmlXPathNodeSetGetLength(ns),
                            (uint64_t)xmlXPathNodeSetContains(ns, root) };
        h = digest64(h, (const uint8_t *)nsl, sizeof(nsl));
        xmlXPathObjectPtr o1 = xmlXPathNewNodeSet(root);
        xmlXPathObjectPtr o2 = xmlXPathNewString(BAD_CAST "12.75");
        xmlXPathObjectPtr o3 = xmlXPathNewBoolean(1);
        xmlXPathObjectPtr o4 = xmlXPathNewFloat(2.5);
        xmlXPathObjectPtr copy = xmlXPathObjectCopy(o2);
        uint64_t casts[4] = {
            (uint64_t)(int64_t)(xmlXPathCastToNumber(o2) * 100),
            (uint64_t)xmlXPathCastToBoolean(o4),
            (uint64_t)xmlXPathCastNumberToBoolean(0.0),
            0,
        };
        xmlChar *cs = xmlXPathCastToString(o4);
        if (cs) { casts[3] = strlen((const char *)cs); xmlFree(cs); }
        h = digest64(h, (const uint8_t *)casts, sizeof(casts));
        xmlChar *ns2str = xmlXPathCastNodeToString(root);
        if (ns2str) { h = digest64(h, ns2str, strlen((const char *)ns2str)); xmlFree(ns2str); }
        xmlXPathObjectPtr objs[5] = { o1, o2, o3, o4, copy };
        for (int k = 0; k < 5; k++)
            xmlXPathFreeObject(objs[k]);
        xmlXPathFreeNodeSet(ns);
        uint64_t nan[3] = { (uint64_t)xmlXPathIsNaN(xmlXPathNAN),
                            (uint64_t)xmlXPathIsInf(xmlXPathPINF),
                            (uint64_t)(int64_t)xmlXPathIsInf(xmlXPathNINF) };
        h = digest64(h, (const uint8_t *)nan, sizeof(nan));

        /* --- tree utilities --- */
        xmlChar *prefix = NULL;
        xmlChar *local = xmlSplitQName2(BAD_CAST "inv:item", &prefix);
        if (local) { h = digest64(h, local, strlen((const char *)local)); xmlFree(local); }
        if (prefix) xmlFree(prefix);
        xmlChar *qn = xmlBuildQName(BAD_CAST "item", BAD_CAST "inv", NULL, 0);
        if (qn) { h = digest64(h, qn, strlen((const char *)qn)); xmlFree(qn); }
        uint64_t vld[4] = {
            (uint64_t)xmlValidateNCName(BAD_CAST "good-name", 0),
            (uint64_t)xmlValidateQName(BAD_CAST "p:n", 0),
            (uint64_t)xmlValidateName(BAD_CAST "name", 0),
            (uint64_t)xmlValidateNMToken(BAD_CAST "tok", 0),
        };
        h = digest64(h, (const uint8_t *)vld, sizeof(vld));
        xmlNodePtr frag = xmlNewDocFragment(doc);
        xmlNodePtr t1 = xmlNewDocText(doc, BAD_CAST "left");
        xmlNodePtr t2 = xmlNewDocText(doc, BAD_CAST "right");
        xmlAddChild(frag, t1);
        xmlTextConcat(t1, BAD_CAST "-mid-", 5);
        xmlAddChild(frag, t2);   /* merges adjacent text nodes */
        xmlChar *fc = xmlNodeGetContent(frag);
        if (fc) { h = digest64(h, fc, strlen((const char *)fc)); xmlFree(fc); }
        xmlFreeNode(frag);
        uint64_t misc[3] = { (uint64_t)xmlIsBlankNode(root),
                             (uint64_t)xmlGetCompressMode(),
                             (uint64_t)(uintptr_t)(xmlGetIntSubset(doc) != NULL) };
        xmlSetCompressMode(0);
        h = digest64(h, (const uint8_t *)misc, sizeof(misc));

        /* --- validation API --- */
        uint64_t vv[4] = {
            (uint64_t)xmlValidateNameValue(BAD_CAST "elem"),
            (uint64_t)xmlValidateNamesValue(BAD_CAST "a b c"),
            (uint64_t)xmlValidateNmtokenValue(BAD_CAST "tok1"),
            (uint64_t)xmlValidateNmtokensValue(BAD_CAST "t1 t2"),
        };
        h = digest64(h, (const uint8_t *)vv, sizeof(vv));
        xmlValidCtxtPtr vc = xmlNewValidCtxt();
        if (vc) {
            uint64_t dv = (uint64_t)xmlValidateDocumentFinal(vc, doc);
            h = digest64(h, (const uint8_t *)&dv, sizeof(dv));
            uint64_t isid = (uint64_t)(int64_t)xmlIsID(doc, root, root->properties);
            h = digest64(h, (const uint8_t *)&isid, sizeof(isid));
            xmlFreeValidCtxt(vc);
        }

        /* --- memory API --- */
        void *m = xmlMemMalloc(64);
        m = xmlMemRealloc(m, 128);
        char *sd = xmlMemoryStrdup("memdup");
        uint64_t mem[2] = { (uint64_t)(m != NULL && sd != NULL),
                            (uint64_t)(xmlMemUsed() >= 0) };
        h = digest64(h, (const uint8_t *)mem, sizeof(mem));
        (void)xmlMemBlocks();
        xmlMemFree(m);
        xmlMemFree(sd);

        /* --- threads / xlink / entities / html / encoding helpers --- */
        xmlMutexPtr mx = xmlNewMutex();
        xmlMutexLock(mx);
        xmlMutexUnlock(mx);
        xmlFreeMutex(mx);
        xmlRMutexPtr rmx = xmlNewRMutex();
        xmlRMutexLock(rmx);
        xmlRMutexUnlock(rmx);
        xmlFreeRMutex(rmx);
        uint64_t thr = (uint64_t)xmlIsMainThread();
        h = digest64(h, (const uint8_t *)&thr, sizeof(thr));
        xlinkNodeDetectFunc old = xlinkGetDefaultDetect();
        xlinkSetDefaultDetect(old);
        xlinkHandlerPtr oldh = xlinkGetDefaultHandler();
        xlinkSetDefaultHandler(oldh);

        xmlChar *enc1 = xmlEncodeEntitiesReentrant(doc, BAD_CAST "a<b>&c\"d");
        if (enc1) { h = digest64(h, enc1, strlen((const char *)enc1)); xmlFree(enc1); }
        xmlChar *enc2 = xmlEncodeSpecialChars(doc, BAD_CAST "x<y&z");
        if (enc2) { h = digest64(h, enc2, strlen((const char *)enc2)); xmlFree(enc2); }
        xmlEntityPtr ent = xmlGetPredefinedEntity(BAD_CAST "amp");
        uint64_t ep = (ent != NULL);
        h = digest64(h, (const uint8_t *)&ep, sizeof(ep));

        unsigned char htmlout[64];
        int outlen = sizeof(htmlout), inlen = 7;
        UTF8ToHtml(htmlout, &outlen, (const unsigned char *)"caf\xC3\xA9!x", &inlen);
        h = digest64(h, htmlout, (size_t)outlen);
        htmlDocPtr hd = htmlNewDoc(NULL, NULL);
        if (hd) {
            htmlSetMetaEncoding(hd, BAD_CAST "UTF-8");
            const xmlChar *me = htmlGetMetaEncoding(hd);
            if (me) h = digest64(h, me, strlen((const char *)me));
            xmlFreeDoc(hd);
        }
        uint64_t utf[4] = {
            (uint64_t)xmlUTF8Strlen(BAD_CAST "caf\xC3\xA9"),
            (uint64_t)xmlCheckUTF8((const unsigned char *)"ok"),
            (uint64_t)xmlUTF8Size(BAD_CAST "\xC3\xA9"),
            (uint64_t)xmlUTF8Charcmp(BAD_CAST "a", BAD_CAST "b"),
        };
        h = digest64(h, (const uint8_t *)utf, sizeof(utf));
        int glen = 4;
        uint64_t g1 = (uint64_t)xmlGetUTF8Char((const unsigned char *)"\xC3\xA9xx", &glen);
        h = digest64(h, (const uint8_t *)&g1, sizeof(g1));

        /* c14n exclusive + 1.1 modes */
        xmlChar *c14nout = NULL;
        int cr = xmlC14NDocDumpMemory(doc, NULL, XML_C14N_EXCLUSIVE_1_0, NULL, 1, &c14nout);
        if (cr >= 0 && c14nout) { h = digest64(h, c14nout, (size_t)cr); xmlFree(c14nout); }
        c14nout = NULL;
        cr = xmlC14NDocDumpMemory(doc, NULL, XML_C14N_1_1, NULL, 0, &c14nout);
        if (cr >= 0 && c14nout) { h = digest64(h, c14nout, (size_t)cr); xmlFree(c14nout); }
    }
    xmlFreeDoc(doc);
    printf("op=api_storm in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* ------- api_variants: variant entry points of already-exercised kernels --
 * (coverage_only). Same parse/serialize kernels reached through every
 * exported wrapper: Doc/Fd/File/IO/Memory x xml/html x ctxt/no-ctxt, the
 * WriteFormat + WriteVFormat writer family, thread-default and global
 * accessors, dump/save variants. Deprecated wrappers are called on purpose. */
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"

static const char av_doc[] =
    "<?xml version=\"1.0\"?>\n"
    "<!DOCTYPE doc [\n"
    "<!NOTATION gif SYSTEM \"image/gif\">\n"
    "<!ELEMENT doc (item*)>\n"
    "<!ELEMENT item (#PCDATA)>\n"
    "<!ATTLIST item id ID #IMPLIED>\n"
    "<!ENTITY vw \"vee\">\n"
    "]>\n"
    "<doc><item id=\"a1\">t&vw;</item><item>u</item></doc>\n";

/* memory-reader callbacks for the *IO entry points */
struct av_io { const char *p; size_t n, off; };
static int av_ioread(void *ctx, char *buf, int len) {
    struct av_io *io = (struct av_io *)ctx;
    size_t left = io->n - io->off;
    size_t take = (size_t)len < left ? (size_t)len : left;
    memcpy(buf, io->p + io->off, take);
    io->off += take;
    return (int)take;
}
static int av_ioclose(void *ctx) { (void)ctx; return 0; }
static int av_iowrite(void *ctx, const char *buf, int len) {
    *(uint64_t *)ctx = digest64(*(uint64_t *)ctx, (const uint8_t *)buf, (size_t)len);
    return len;
}

/* fold parse result: well-formedness + tree walk (heap addresses never) */
static uint64_t av_fold_doc(uint64_t h, xmlDocPtr doc) {
    uint64_t ok = (doc != NULL);
    h = digest64(h, (const uint8_t *)&ok, sizeof(ok));
    if (doc) {
        h = fold_node(h, xmlDocGetRootElement(doc));
        xmlFreeDoc(doc);
    }
    return h;
}
static uint64_t av_b(uint64_t h, uint64_t v) {
    return digest64(h, (const uint8_t *)&v, sizeof(v));
}
static uint64_t av_s(uint64_t h, const xmlChar *s) {
    return s ? digest64(h, s, strlen((const char *)s)) : av_b(h, 0);
}

static uint64_t av_parser(uint64_t h, const char *path) {
    size_t n = sizeof(av_doc) - 1;
    int opt = XML_PARSE_NOERROR | XML_PARSE_NOWARNING;

    h = av_fold_doc(h, xmlReadDoc(BAD_CAST av_doc, "av.xml", NULL, opt));
    int fd = open(path, O_RDONLY);
    if (fd >= 0) { h = av_fold_doc(h, xmlReadFd(fd, "av.xml", NULL, opt)); close(fd); }
    struct av_io io = { av_doc, n, 0 };
    h = av_fold_doc(h, xmlReadIO(av_ioread, av_ioclose, &io, "av.xml", NULL, opt));

    xmlParserCtxtPtr c = xmlNewParserCtxt();
    h = av_fold_doc(h, xmlCtxtReadDoc(c, BAD_CAST av_doc, "av.xml", NULL, opt));
    h = av_fold_doc(h, xmlCtxtReadMemory(c, av_doc, (int)n, "av.xml", NULL, opt));
    fd = open(path, O_RDONLY);
    if (fd >= 0) { h = av_fold_doc(h, xmlCtxtReadFd(c, fd, "av.xml", NULL, opt)); close(fd); }
    io.off = 0;
    h = av_fold_doc(h, xmlCtxtReadIO(c, av_ioread, av_ioclose, &io, "av.xml", NULL, opt));
    xmlCtxtSetMaxAmplification(c, 100.0f);
    xmlCtxtResetPush(c, av_doc, 4, "av.xml", NULL);
    xmlParseChunk(c, av_doc + 4, (int)(n - 4), 1);
    h = av_b(h, (uint64_t)c->wellFormed);
    if (c->myDoc) { xmlFreeDoc(c->myDoc); c->myDoc = NULL; }
    xmlClearParserCtxt(c);
    xmlFreeParserCtxt(c);
    h = av_b(h, (uint64_t)xmlHasFeature(XML_WITH_TREE));

    h = av_fold_doc(h, xmlParseDoc(BAD_CAST av_doc));
    h = av_fold_doc(h, xmlParseFile(path));
    h = av_fold_doc(h, xmlParseMemory(av_doc, (int)n));
    h = av_fold_doc(h, xmlRecoverDoc(BAD_CAST "<r><broken></r>"));
    h = av_fold_doc(h, xmlRecoverFile(path));
    h = av_fold_doc(h, xmlRecoverMemory(av_doc, (int)n));
    h = av_fold_doc(h, xmlSAXParseDoc(NULL, BAD_CAST av_doc, 0));
    h = av_fold_doc(h, xmlSAXParseFile(NULL, path, 0));
    h = av_fold_doc(h, xmlSAXParseMemory(NULL, av_doc, (int)n, 0));
    h = av_fold_doc(h, xmlSAXParseFileWithData(NULL, path, 0, NULL));
    h = av_fold_doc(h, xmlSAXParseMemoryWithData(NULL, av_doc, (int)n, 0, NULL));
    h = av_b(h, (uint64_t)(int64_t)xmlSAXUserParseMemory(NULL, NULL, av_doc, (int)n));

    /* balanced chunks + in-node context */
    xmlDocPtr doc = xmlReadDoc(BAD_CAST av_doc, "av.xml", NULL, opt);
    if (doc) {
        xmlNodePtr lst = NULL;
        h = av_b(h, (uint64_t)(int64_t)xmlParseBalancedChunkMemory(
                     doc, NULL, NULL, 0, BAD_CAST "<item>b1</item>", &lst));
        if (lst) xmlFreeNodeList(lst);
        lst = NULL;
        h = av_b(h, (uint64_t)(int64_t)xmlParseBalancedChunkMemoryRecover(
                     doc, NULL, NULL, 0, BAD_CAST "<item>b2</broken>", &lst, 1));
        if (lst) xmlFreeNodeList(lst);
        lst = NULL;
        h = av_b(h, (uint64_t)(int64_t)xmlParseInNodeContext(
                     xmlDocGetRootElement(doc), "<item>ctx</item>", 15, opt, &lst));
        if (lst) xmlFreeNodeList(lst);
        xmlFreeDoc(doc);
    }
    h = av_b(h, (uint64_t)xmlCheckLanguageID(BAD_CAST "en-US"));
    return h;
}

static const char av_html[] =
    "<html><head><title>vw</title></head>"
    "<body><p class=\"x\">hi<br>there</p><table><tr><td>1</td></tr></table>"
    "</body></html>";

static uint64_t av_htmlp(uint64_t h, const char *path) {
    size_t n = sizeof(av_html) - 1;
    int opt = HTML_PARSE_NOERROR | HTML_PARSE_NOWARNING;
    (void)path;

    h = av_fold_doc(h, htmlReadDoc(BAD_CAST av_html, "av.html", NULL, opt));
    h = av_fold_doc(h, htmlReadMemory(av_html, (int)n, "av.html", NULL, opt));
    int fds[2];
    if (pipe(fds) == 0) {
        (void)!write(fds[1], av_html, n);
        close(fds[1]);
        h = av_fold_doc(h, htmlReadFd(fds[0], "av.html", NULL, opt));
        close(fds[0]);
    }
    struct av_io io = { av_html, n, 0 };
    h = av_fold_doc(h, htmlReadIO(av_ioread, av_ioclose, &io, "av.html", NULL, opt));

    htmlParserCtxtPtr c = htmlCreateMemoryParserCtxt(av_html, (int)n);
    if (c) {
        h = av_fold_doc(h, htmlCtxtReadDoc(c, BAD_CAST av_html, "av.html", NULL, opt));
        h = av_fold_doc(h, htmlCtxtReadMemory(c, av_html, (int)n, "av.html", NULL, opt));
        io.off = 0;
        h = av_fold_doc(h, htmlCtxtReadIO(c, av_ioread, av_ioclose, &io, "av.html", NULL, opt));
        if (pipe(fds) == 0) {
            (void)!write(fds[1], av_html, n);
            close(fds[1]);
            h = av_fold_doc(h, htmlCtxtReadFd(c, fds[0], "av.html", NULL, opt));
            close(fds[0]);
        }
        htmlCtxtReset(c);
        htmlFreeParserCtxt(c);
    }
    h = av_fold_doc(h, htmlParseDoc(BAD_CAST av_html, NULL));
    h = av_fold_doc(h, htmlSAXParseDoc(BAD_CAST av_html, NULL, NULL, NULL));

    /* element/attr status helpers on a parsed doc */
    htmlDocPtr hd = htmlReadDoc(BAD_CAST av_html, "av.html", NULL, opt);
    if (hd) {
        xmlNodePtr root = xmlDocGetRootElement(hd);
        xmlNodePtr body = root ? root->children : NULL;
        while (body && !xmlStrEqual(body->name, BAD_CAST "body")) body = body->next;
        const htmlElemDesc *pdesc = htmlTagLookup(BAD_CAST "p");
        const htmlElemDesc *tddesc = htmlTagLookup(BAD_CAST "td");
        if (pdesc && body) {
            h = av_b(h, (uint64_t)htmlAttrAllowed(pdesc, BAD_CAST "class", 0));
            h = av_b(h, (uint64_t)htmlElementAllowedHere(pdesc, BAD_CAST "em"));
            h = av_b(h, (uint64_t)(int64_t)htmlElementStatusHere(pdesc, tddesc));
            h = av_b(h, (uint64_t)(int64_t)htmlNodeStatus(body->children, 0));
            h = av_b(h, (uint64_t)htmlAutoCloseTag(hd, BAD_CAST "p", body->children));
            h = av_b(h, (uint64_t)htmlIsAutoClosed(hd, body->children));
        }
        xmlFreeDoc(hd);
    }
    htmlInitAutoClose();
    int old = htmlHandleOmittedElem(1);
    htmlHandleOmittedElem(old);

    unsigned char out[64];
    int outlen = (int)sizeof(out), inlen = 4;
    htmlEncodeEntities(out, &outlen, (const unsigned char *)"a<b\xA9", &inlen, '\x27');
    h = digest64(h, out, (size_t)outlen);
    return h;
}

/* walk one reader folding the accessor battery at each node */
static uint64_t av_reader_walk(uint64_t h, xmlTextReaderPtr rd) {
    if (!rd) return av_b(h, 0);
    int r;
    while ((r = xmlTextReaderRead(rd)) == 1) {
        h = av_s(h, xmlTextReaderConstName(rd));
        xmlChar *s;
        if ((s = xmlTextReaderName(rd)))      { h = av_s(h, s); xmlFree(s); }
        if ((s = xmlTextReaderLocalName(rd))) { h = av_s(h, s); xmlFree(s); }
        if ((s = xmlTextReaderPrefix(rd)))    { h = av_s(h, s); xmlFree(s); }
        if ((s = xmlTextReaderNamespaceUri(rd))) { h = av_s(h, s); xmlFree(s); }
        if ((s = xmlTextReaderBaseUri(rd)))   { h = av_s(h, s); xmlFree(s); }
        if ((s = xmlTextReaderValue(rd)))     { h = av_s(h, s); xmlFree(s); }
        if ((s = xmlTextReaderXmlLang(rd)))   { h = av_s(h, s); xmlFree(s); }
        h = av_b(h, (uint64_t)xmlTextReaderIsNamespaceDecl(rd));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderQuoteChar(rd));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderStandalone(rd));
        h = av_b(h, (uint64_t)xmlTextReaderNormalization(rd));
        h = av_b(h, (uint64_t)xmlTextReaderGetParserLineNumber(rd));
        h = av_b(h, (uint64_t)xmlTextReaderGetParserColumnNumber(rd));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderGetParserProp(rd, XML_PARSER_LOADDTD));
        if (xmlTextReaderNodeType(rd) == 1) {
            if ((s = xmlTextReaderGetAttribute(rd, BAD_CAST "id"))) { h = av_s(h, s); xmlFree(s); }
            if ((s = xmlTextReaderGetAttributeNs(rd, BAD_CAST "id", NULL))) { h = av_s(h, s); xmlFree(s); }
            if ((s = xmlTextReaderLookupNamespace(rd, NULL))) { h = av_s(h, s); xmlFree(s); }
            if (xmlTextReaderMoveToAttribute(rd, BAD_CAST "id") == 1) {
                h = av_b(h, (uint64_t)xmlTextReaderReadAttributeValue(rd));
                xmlTextReaderMoveToElement(rd);
            }
            if (xmlTextReaderMoveToAttributeNo(rd, 0) == 1)
                xmlTextReaderMoveToElement(rd);
            if (xmlTextReaderMoveToAttributeNs(rd, BAD_CAST "id", NULL) == 1)
                xmlTextReaderMoveToElement(rd);
        }
    }
    h = av_b(h, (uint64_t)r);
    h = av_s(h, xmlTextReaderConstEncoding(rd));
    h = av_s(h, xmlTextReaderConstXmlVersion(rd));
    h = av_s(h, xmlTextReaderConstString(rd, BAD_CAST "vw-probe"));
    return h;
}

static uint64_t av_reader(uint64_t h, const char *path) {
    size_t n = sizeof(av_doc) - 1;
    int opt = XML_PARSE_NOERROR | XML_PARSE_NOWARNING | XML_PARSE_NONET;

    h = av_reader_walk(h, NULL);
    xmlTextReaderPtr rd = xmlReaderForDoc(BAD_CAST av_doc, "av.xml", NULL, opt);
    h = av_reader_walk(h, rd);
    if (rd) {
        /* xmlReaderNew* reuse the same reader object */
        if (xmlReaderNewDoc(rd, BAD_CAST av_doc, "av.xml", NULL, opt) == 0)
            h = av_reader_walk(h, rd);
        if (xmlReaderNewMemory(rd, av_doc, (int)n, "av.xml", NULL, opt) == 0)
            h = av_reader_walk(h, rd);
        if (xmlReaderNewFile(rd, path, NULL, opt) == 0) {
            h = av_b(h, (uint64_t)xmlTextReaderRead(rd));
            h = av_b(h, (uint64_t)xmlTextReaderNext(rd));
        }
        struct av_io io = { av_doc, n, 0 };
        if (xmlReaderNewIO(rd, av_ioread, av_ioclose, &io, "av.xml", NULL, opt) == 0)
            h = av_reader_walk(h, rd);
        int fds[2];
        if (pipe(fds) == 0) {
            (void)!write(fds[1], av_doc, n);
            close(fds[1]);
            if (xmlReaderNewFd(rd, fds[0], "av.xml", NULL, opt) == 0)
                h = av_reader_walk(h, rd);
            close(fds[0]);
        }
        xmlFreeTextReader(rd);
    }
    struct av_io io2 = { av_doc, n, 0 };
    rd = xmlReaderForIO(av_ioread, av_ioclose, &io2, "av.xml", NULL, opt);
    if (rd) { h = av_b(h, (uint64_t)xmlTextReaderRead(rd)); xmlFreeTextReader(rd); }
    int fds[2];
    if (pipe(fds) == 0) {
        (void)!write(fds[1], av_doc, n);
        close(fds[1]);
        rd = xmlReaderForFd(fds[0], "av.xml", NULL, opt);
        if (rd) { h = av_b(h, (uint64_t)xmlTextReaderRead(rd)); xmlFreeTextReader(rd); }
        close(fds[0]);
    }
    rd = xmlReaderForMemory(av_doc, (int)n, "av.xml", NULL, opt);
    if (rd) {
        /* NULL deactivates: covers the set/validate entry paths */
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderSetSchema(rd, NULL));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderRelaxNGSetSchema(rd, NULL));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderRelaxNGValidate(rd, NULL));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderRelaxNGValidateCtxt(rd, NULL, 0));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderSchemaValidate(rd, NULL));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderSchemaValidateCtxt(rd, NULL, 0));
        xmlTextReaderSetErrorHandler(rd, NULL, NULL);
        xmlTextReaderSetStructuredErrorHandler(rd, NULL, NULL);
        xmlTextReaderErrorFunc ef = NULL; void *ea = NULL;
        xmlTextReaderGetErrorHandler(rd, &ef, &ea);
        h = av_b(h, (uint64_t)(ef == NULL));
        xmlTextReaderSetMaxAmplification(rd, 100.0f);
        h = av_b(h, (uint64_t)xmlTextReaderRead(rd));
        h = av_b(h, (uint64_t)(xmlTextReaderPreserve(rd) != NULL));
        h = av_b(h, (uint64_t)(int64_t)xmlTextReaderPreservePattern(rd, BAD_CAST "//item", NULL));
        xmlDocPtr cur = xmlTextReaderCurrentDoc(rd);
        h = av_b(h, (uint64_t)(cur != NULL));
        xmlNodePtr cn = xmlTextReaderCurrentNode(rd);
        h = av_b(h, (uint64_t)(cn != NULL));
        xmlChar *rem_probe = NULL; (void)rem_probe;
        xmlParserInputBufferPtr remb = xmlTextReaderGetRemainder(rd);
        h = av_b(h, (uint64_t)(remb != NULL));
        if (remb) xmlFreeParserInputBuffer(remb);
        xmlFreeTextReader(rd);
        if (cur) xmlFreeDoc(cur);
    }
    /* walker: NextSibling only works on walker readers */
    xmlDocPtr doc = xmlReadDoc(BAD_CAST av_doc, "av.xml", NULL, opt);
    if (doc) {
        rd = xmlReaderWalker(doc);
        if (rd) {
            if (xmlReaderNewWalker(rd, doc) == 0) {
                while (xmlTextReaderRead(rd) == 1) {
                    if (xmlTextReaderNodeType(rd) == 1) {
                        h = av_b(h, (uint64_t)xmlTextReaderNextSibling(rd));
                        break;
                    }
                }
            }
            xmlFreeTextReader(rd);
        }
        xmlFreeDoc(doc);
    }
    return h;
}

static uint64_t av_writer(uint64_t h) {
    /* WriteFormat* delegate to WriteVFormat*, so both families are covered */
    xmlBufferPtr xb = xmlBufferCreate();
    xmlTextWriterPtr w = xmlNewTextWriterMemory(xb, 0);
    if (w) {
        xmlTextWriterStartDocument(w, "1.0", "UTF-8", NULL);
        xmlTextWriterWriteFormatDTD(w, BAD_CAST "doc", NULL, BAD_CAST "%s.dtd", "doc");
        xmlTextWriterStartElement(w, BAD_CAST "doc");
        xmlTextWriterWriteFormatComment(w, "comment %d", 7);
        xmlTextWriterWriteFormatPI(w, BAD_CAST "pi", "v%d", 2);
        xmlTextWriterWriteFormatElementNS(w, BAD_CAST "v", BAD_CAST "e",
                                          BAD_CAST "http://example.com/ns", "%03d", 5);
        xmlTextWriterStartElement(w, BAD_CAST "a");
        xmlTextWriterWriteFormatAttributeNS(w, BAD_CAST "v", BAD_CAST "at",
                                            NULL, "x%c", 'y');
        xmlTextWriterWriteFormatString(w, "str-%s", "body");
        xmlTextWriterWriteFormatCDATA(w, "cd-%d", 3);
        xmlTextWriterWriteFormatRaw(w, "<raw%d/>", 1);
        xmlTextWriterFullEndElement(w);
        xmlTextWriterWriteElementNS(w, BAD_CAST "v", BAD_CAST "el",
                                    BAD_CAST "http://example.com/ns2", BAD_CAST "t");
        xmlTextWriterWriteBinHex(w, "hexpayload", 0, 10);
        xmlTextWriterEndElement(w);
        xmlTextWriterEndDocument(w);
        xmlFreeTextWriter(w);
        h = digest64(h, xmlBufferContent(xb), (size_t)xmlBufferLength(xb));
    }
    xmlBufferFree(xb);

    /* DTD content parts through a second writer */
    xb = xmlBufferCreate();
    w = xmlNewTextWriterMemory(xb, 0);
    if (w) {
        xmlTextWriterStartDocument(w, "1.0", "UTF-8", NULL);
        xmlTextWriterWriteDTD(w, BAD_CAST "doc", NULL, NULL,
                              BAD_CAST "<!ELEMENT doc (#PCDATA)>");
        xmlTextWriterStartElement(w, BAD_CAST "doc");
        xmlTextWriterEndElement(w);
        xmlTextWriterEndDocument(w);
        xmlFreeTextWriter(w);
        h = digest64(h, xmlBufferContent(xb), (size_t)xmlBufferLength(xb));
    }
    xmlBufferFree(xb);
    xb = xmlBufferCreate();
    w = xmlNewTextWriterMemory(xb, 0);
    if (w) {
        xmlTextWriterStartDocument(w, "1.0", "UTF-8", NULL);
        xmlTextWriterStartDTD(w, BAD_CAST "doc", NULL, NULL);
        xmlTextWriterWriteFormatDTDElement(w, BAD_CAST "doc", "(%s*)", "item");
        xmlTextWriterWriteFormatDTDAttlist(w, BAD_CAST "item", "id %s #IMPLIED", "CDATA");
        xmlTextWriterWriteFormatDTDInternalEntity(w, 0, BAD_CAST "fe", "%d", 9);
        xmlTextWriterWriteDTDExternalEntity(w, 0, BAD_CAST "xe", NULL,
                                            BAD_CAST "sys.ent", NULL);
        xmlTextWriterWriteDTDExternalEntityContents(w, NULL, BAD_CAST "sys2.ent", NULL);
        xmlTextWriterWriteDTDNotation(w, BAD_CAST "gif", NULL, BAD_CAST "image/gif");
        xmlTextWriterEndDTD(w);
        xmlTextWriterStartElement(w, BAD_CAST "doc");
        xmlTextWriterEndElement(w);
        xmlTextWriterEndDocument(w);
        xmlFreeTextWriter(w);
        h = digest64(h, xmlBufferContent(xb), (size_t)xmlBufferLength(xb));
    }
    xmlBufferFree(xb);

    /* variant constructors: doc-backed, tree-backed, push-parser, filename */
    xmlDocPtr wd = NULL;
    w = xmlNewTextWriterDoc(&wd, 0);
    if (w) {
        xmlTextWriterStartDocument(w, "1.0", "UTF-8", NULL);
        xmlTextWriterStartElement(w, BAD_CAST "root");
        xmlTextWriterWriteString(w, BAD_CAST "docwriter");
        xmlTextWriterEndElement(w);
        xmlTextWriterEndDocument(w);
        xmlFreeTextWriter(w);
        if (wd) {
            h = fold_node(h, xmlDocGetRootElement(wd));
            xmlNodePtr r2 = xmlDocGetRootElement(wd);
            xmlTextWriterPtr tw = xmlNewTextWriterTree(wd, r2, 0);
            if (tw) {
                xmlTextWriterStartElement(tw, BAD_CAST "sub");
                xmlTextWriterEndElement(tw);
                xmlFreeTextWriter(tw);
            }
            h = fold_node(h, xmlDocGetRootElement(wd));
            xmlFreeDoc(wd);
        }
    }
    xmlParserCtxtPtr pc = xmlCreatePushParserCtxt(NULL, NULL, NULL, 0, "w.xml");
    if (pc) {
        xmlTextWriterPtr pw = xmlNewTextWriterPushParser(pc, 0);
        if (pw) {
            xmlTextWriterStartDocument(pw, "1.0", "UTF-8", NULL);
            xmlTextWriterStartElement(pw, BAD_CAST "p");
            xmlTextWriterEndElement(pw);
            xmlTextWriterEndDocument(pw);
            if (pc->myDoc)
                h = fold_node(h, xmlDocGetRootElement(pc->myDoc));
            xmlFreeTextWriter(pw);   /* frees the ctxt AND its doc */
        } else {
            xmlFreeParserCtxt(pc);
        }
    }
    const char *scratch = "/tmp/vw_lx_writer.xml";
    w = xmlNewTextWriterFilename(scratch, 0);
    if (w) {
        xmlTextWriterStartDocument(w, "1.0", "UTF-8", NULL);
        xmlTextWriterStartElement(w, BAD_CAST "f");
        xmlTextWriterEndElement(w);
        xmlTextWriterEndDocument(w);
        xmlFreeTextWriter(w);
        size_t fn2;
        uint8_t *fb = read_file(scratch, &fn2);
        h = digest64(h, fb, fn2);
        free(fb);
        remove(scratch);
    }
    return h;
}

static uint64_t av_xpath(uint64_t h) {
    int opt = XML_PARSE_NOERROR | XML_PARSE_NOWARNING;
    xmlDocPtr doc = xmlReadDoc(BAD_CAST av_doc, "av.xml", NULL, opt);
    if (!doc) return av_b(h, 0);
    xmlNodePtr root = xmlDocGetRootElement(doc);
    xmlNodePtr first = xmlFirstElementChild(root);
    xmlNodePtr last = xmlLastElementChild(root);

    xmlXPathInit();
    xmlXPathOrderDocElems(doc);
    xmlXPathContextPtr ctx = xmlXPathNewContext(doc);
    xmlXPathContextSetCache(ctx, 1, 16, 0);
    xmlXPathRegisterNs(ctx, BAD_CAST "vw", BAD_CAST "http://example.com/vw");
    h = av_s(h, xmlXPathNsLookup(ctx, BAD_CAST "vw"));
    xmlXPathRegisterFuncLookup(ctx, NULL, NULL);
    xmlXPathRegisterVariableLookup(ctx, NULL, NULL);
    h = av_b(h, (uint64_t)(int64_t)xmlXPathSetContextNode(root, ctx));
    xmlXPathObjectPtr ne = xmlXPathNodeEval(root, BAD_CAST "count(item)", ctx);
    if (ne) { h = av_b(h, (uint64_t)(int64_t)ne->floatval); xmlXPathFreeObject(ne); }
    xmlXPathCompExprPtr comp = xmlXPathCompile(BAD_CAST "count(//item) > 0");
    if (comp) {
        h = av_b(h, (uint64_t)xmlXPathCompiledEvalToBoolean(comp, ctx));
        FILE *dn = fopen("/dev/null", "w");
        if (dn) { xmlXPathDebugDumpCompExpr(dn, comp, 0); fclose(dn); }
        xmlXPathFreeCompExpr(comp);
    }

    /* node-set algebra (s1 needs >= 2 nodes: the Leading/Trailing helpers
     * return nodes1 ITSELF -- an alias, not a copy -- when Item(nodes2,1)
     * is NULL, and aliased sets must not be double-freed) */
    xmlNodeSetPtr s1 = xmlXPathNodeSetCreate(first);
    xmlXPathNodeSetAdd(s1, last);
    xmlNodeSetPtr s2 = xmlXPathNodeSetCreate(last);
    xmlXPathNodeSetAdd(s2, first);
    xmlNodeSetPtr mrg = xmlXPathNodeSetMerge(NULL, s1);
    mrg = xmlXPathNodeSetMerge(mrg, s2);
    h = av_b(h, (uint64_t)xmlXPathNodeSetGetLength(mrg));
    xmlNodeSetPtr diff = xmlXPathDifference(s2, s1);
    h = av_b(h, diff ? (uint64_t)xmlXPathNodeSetGetLength(diff) : 99);
    xmlNodeSetPtr inter = xmlXPathIntersection(s1, s2);
    h = av_b(h, inter ? (uint64_t)xmlXPathNodeSetGetLength(inter) : 99);
    h = av_b(h, (uint64_t)xmlXPathHasSameNodes(s1, s2));
    xmlNodeSetPtr dis = xmlXPathDistinct(s2);
    h = av_b(h, dis ? (uint64_t)xmlXPathNodeSetGetLength(dis) : 99);
    xmlNodeSetPtr diss = xmlXPathDistinctSorted(s2);
    xmlNodeSetPtr lead = xmlXPathLeading(s2, s1);
    xmlNodeSetPtr leads = xmlXPathLeadingSorted(s2, s1);
    xmlNodeSetPtr trail = xmlXPathTrailing(s2, s1);
    xmlNodeSetPtr trails = xmlXPathTrailingSorted(s2, s1);
    xmlNodeSetPtr nlead = xmlXPathNodeLeading(s2, first);
    xmlNodeSetPtr nleads = xmlXPathNodeLeadingSorted(s2, first);
    xmlNodeSetPtr ntrail = xmlXPathNodeTrailing(s2, first);
    xmlNodeSetPtr ntrails = xmlXPathNodeTrailingSorted(s2, first);
    h = av_b(h, (uint64_t)(int64_t)xmlXPathCmpNodes(first, last));
    xmlXPathNodeSetDel(mrg, first);
    if (xmlXPathNodeSetGetLength(mrg) > 0) xmlXPathNodeSetRemove(mrg, 0);
    xmlNsPtr nsdef = xmlSearchNs(doc, root, NULL);
    if (nsdef) xmlXPathNodeSetAddNs(s1, root, nsdef);
    xmlXPathObjectPtr setobj = xmlXPathNewNodeSetList(s2);
    if (setobj) xmlXPathFreeNodeSetList(setobj);   /* frees obj, keeps nodes */
    xmlNodeSetPtr sets[15] = { s1, s2, mrg, diff, inter, dis, diss, lead, leads,
                               trail, trails, nlead, nleads, ntrail, ntrails };
    for (int i = 0; i < 15; i++) {
        int dup = 0;
        for (int j = 0; j < i; j++) if (sets[j] == sets[i]) dup = 1;
        if (sets[i] && !dup) xmlXPathFreeNodeSet(sets[i]);
    }

    /* object constructors / casts / conversions */
    xmlXPathObjectPtr oc = xmlXPathNewCString("cstr");
    h = av_s(h, oc->stringval);
    xmlXPathObjectPtr wc = xmlXPathWrapCString((char *)xmlMemStrdup("wrapped"));
    h = av_s(h, wc->stringval);
    static int av_ext;
    xmlXPathObjectPtr we = xmlXPathWrapExternal(&av_ext);
    h = av_b(h, (uint64_t)(we->type == XPATH_USERS));
    xmlNodePtr vt = xmlCopyNode(first, 1);
    xmlXPathObjectPtr tv = xmlXPathNewValueTree(vt);
    h = av_b(h, tv ? (uint64_t)tv->type : 99);
    h = av_b(h, (uint64_t)(int64_t)xmlXPathCastBooleanToNumber(1));
    xmlChar *bs = xmlXPathCastBooleanToString(1);
    h = av_s(h, bs); xmlFree(bs);
    xmlXPathObjectPtr cb = xmlXPathConvertBoolean(xmlXPathNewFloat(0.0));
    h = av_b(h, (uint64_t)cb->boolval);
    xmlXPathObjectPtr cn2 = xmlXPathConvertNumber(xmlXPathNewCString("7"));
    h = av_b(h, (uint64_t)(int64_t)cn2->floatval);
    xmlXPathObjectPtr cs2 = xmlXPathConvertString(xmlXPathNewFloat(3.5));
    h = av_s(h, cs2->stringval);
    FILE *dn2 = fopen("/dev/null", "w");
    if (dn2) { xmlXPathDebugDumpObject(dn2, cs2, 0); fclose(dn2); }
    xmlXPathFreeObject(oc); xmlXPathFreeObject(wc); xmlXPathFreeObject(we);
    if (tv) xmlXPathFreeObject(tv);
    xmlXPathFreeObject(cb); xmlXPathFreeObject(cn2); xmlXPathFreeObject(cs2);

    /* parser-context stack ops + axis iterators */
    xmlXPathParserContextPtr p = xmlXPathNewParserContext(BAD_CAST "1", ctx);
    if (p) {
        /* upstream quirk: a fresh external parser context has valueMax==0 and
         * valuePush's doubling realloc can't grow from 0 -- pre-size the stack */
        if (p->valueMax == 0) {
            p->valueTab = (xmlXPathObjectPtr *)
                xmlMalloc(16 * sizeof(xmlXPathObjectPtr));
            if (!p->valueTab) { xmlXPathFreeParserContext(p); p = NULL; }
            else { p->valueMax = 16; p->valueNr = 0; p->value = NULL; }
        }
    }
    if (p) {
        valuePush(p, xmlXPathNewFloat(8.0));
        valuePush(p, xmlXPathNewFloat(2.0));
        xmlXPathDivValues(p);
        h = av_b(h, (uint64_t)(int64_t)xmlXPathPopNumber(p));
        valuePush(p, xmlXPathNewFloat(3.0));
        valuePush(p, xmlXPathNewFloat(4.0));
        xmlXPathMultValues(p);
        valuePush(p, xmlXPathNewFloat(2.0));
        xmlXPathSubValues(p);
        xmlXPathValueFlipSign(p);
        h = av_b(h, (uint64_t)(int64_t)xmlXPathPopNumber(p));
        valuePush(p, xmlXPathNewBoolean(1));
        h = av_b(h, (uint64_t)xmlXPathPopBoolean(p));
        valuePush(p, xmlXPathNewCString("popped"));
        xmlChar *ps = xmlXPathPopString(p);
        h = av_s(h, ps); xmlFree(ps);
        valuePush(p, xmlXPathNewNodeSet(root));
        xmlNodeSetPtr pns = xmlXPathPopNodeSet(p);
        h = av_b(h, pns ? (uint64_t)xmlXPathNodeSetGetLength(pns) : 99);
        if (pns) xmlXPathFreeNodeSet(pns);
        valuePush(p, xmlXPathWrapExternal(&av_ext));
        void *pe = xmlXPathPopExternal(p);
        h = av_b(h, (uint64_t)(pe == &av_ext));
        valuePush(p, xmlXPathNewCString("haystack"));
        valuePush(p, xmlXPathNewCString("st"));
        xmlXPathSubstringBeforeFunction(p, 2);
        xmlChar *sb = xmlXPathPopString(p);
        h = av_s(h, sb); xmlFree(sb);
        valuePush(p, xmlXPathNewCString("haystack"));
        valuePush(p, xmlXPathNewCString("st"));
        xmlXPathSubstringAfterFunction(p, 2);
        xmlChar *sa = xmlXPathPopString(p);
        h = av_s(h, sa); xmlFree(sa);
        ctx->node = first;
        h = av_b(h, (uint64_t)(xmlXPathNextSelf(p, NULL) != NULL));
        h = av_b(h, (uint64_t)(xmlXPathNextAncestor(p, NULL) != NULL));
        h = av_b(h, (uint64_t)(xmlXPathNextAncestorOrSelf(p, NULL) != NULL));
        h = av_b(h, (uint64_t)(xmlXPathNextFollowing(p, NULL) != NULL));
        h = av_b(h, (uint64_t)(xmlXPathNextPreceding(p, last) != NULL));
        h = av_b(h, (uint64_t)(xmlXPathNextNamespace(p, NULL) != NULL));
        xmlXPathObjectPtr pred = xmlXPathNewBoolean(1);
        h = av_b(h, (uint64_t)xmlXPathEvalPredicate(ctx, pred));
        xmlXPathFreeObject(pred);
        xmlXPatherror(p, __FILE__, __LINE__, XPATH_EXPR_ERROR);
        xmlXPathFreeParserContext(p);
    }
    xmlXPathFreeContext(ctx);
    xmlFreeDoc(doc);
    return h;
}

static uint64_t av_valid(uint64_t h) {
    xmlDocPtr doc = xmlReadDoc(BAD_CAST av_doc, "av.xml", NULL,
        XML_PARSE_DTDLOAD | XML_PARSE_NOERROR | XML_PARSE_NOWARNING);
    if (!doc || !doc->intSubset) { if (doc) xmlFreeDoc(doc); return av_b(h, 0); }
    xmlNodePtr root = xmlDocGetRootElement(doc);
    xmlDtdPtr dtd = doc->intSubset;

    xmlValidCtxtPtr vc = xmlNewValidCtxt();
    if (vc) {
        h = av_b(h, (uint64_t)xmlValidateDocument(vc, doc));
        h = av_b(h, (uint64_t)xmlValidateDtd(vc, doc, dtd));
        h = av_b(h, (uint64_t)xmlValidatePushElement(vc, doc, root, BAD_CAST "doc"));
        h = av_b(h, (uint64_t)xmlValidatePushCData(vc, BAD_CAST "tx", 2));
        h = av_b(h, (uint64_t)xmlValidatePopElement(vc, doc, root, BAD_CAST "doc"));
        xmlNotationPtr nota = xmlGetDtdNotationDesc(dtd, BAD_CAST "gif");
        if (nota) {
            h = av_b(h, (uint64_t)xmlValidateNotationDecl(vc, doc, nota));
            xmlBufferPtr nb = xmlBufferCreate();
            xmlDumpNotationDecl(nb, nota);
            h = digest64(h, xmlBufferContent(nb), (size_t)xmlBufferLength(nb));
            xmlBufferFree(nb);
        }
        h = av_b(h, (uint64_t)xmlValidateNotationUse(vc, doc, BAD_CAST "gif"));
        xmlFreeValidCtxt(vc);
    }
    h = av_b(h, (uint64_t)xmlIsMixedElement(doc, BAD_CAST "item"));

    /* element content model build / copy / print */
    xmlElementContentPtr ec = xmlNewElementContent(BAD_CAST "item",
                                                   XML_ELEMENT_CONTENT_ELEMENT);
    if (ec) {
        ec->ocur = XML_ELEMENT_CONTENT_MULT;
        xmlElementContentPtr ecc = xmlCopyElementContent(ec);
        xmlElementContentPtr ecd = xmlCopyDocElementContent(doc, ec);
        char sbuf[256];
        sbuf[0] = 0;
        xmlSprintfElementContent(sbuf, ec, 1);
        h = digest64(h, (const uint8_t *)sbuf, strlen(sbuf));
        const xmlChar *names[16];
        int nn = xmlValidGetPotentialChildren(ec, names, &(int){0}, 16);
        (void)nn;
        if (ecc) xmlFreeElementContent(ecc);
        if (ecd) xmlFreeDocElementContent(doc, ecd);
        xmlFreeElementContent(ec);
    }
    {
        const xmlChar *names[16];
        int len = 0;
        xmlNodePtr first = xmlFirstElementChild(root);
        len = xmlValidGetValidElements(first, NULL, names, 16);
        h = av_b(h, (uint64_t)(int64_t)len);
    }

    /* dtd tables: copy + dump */
    if (dtd->elements) {
        xmlElementTablePtr et = xmlCopyElementTable((xmlElementTablePtr)dtd->elements);
        xmlBufferPtr b = xmlBufferCreate();
        xmlDumpElementTable(b, (xmlElementTablePtr)dtd->elements);
        /* hash-scan order is seed-dependent: fold length only */
        h = av_b(h, (uint64_t)xmlBufferLength(b));
        xmlBufferFree(b);
        if (et) xmlFreeElementTable(et);
    }
    if (dtd->attributes) {
        xmlAttributeTablePtr at = xmlCopyAttributeTable((xmlAttributeTablePtr)dtd->attributes);
        xmlBufferPtr b = xmlBufferCreate();
        xmlDumpAttributeTable(b, (xmlAttributeTablePtr)dtd->attributes);
        /* hash-scan order is seed-dependent: fold length only */
        h = av_b(h, (uint64_t)xmlBufferLength(b));
        xmlBufferFree(b);
        if (at) xmlFreeAttributeTable(at);
    }
    if (dtd->notations) {
        xmlNotationTablePtr nt = xmlCopyNotationTable((xmlNotationTablePtr)dtd->notations);
        xmlBufferPtr b = xmlBufferCreate();
        xmlDumpNotationTable(b, (xmlNotationTablePtr)dtd->notations);
        /* hash-scan order is seed-dependent: fold length only */
        h = av_b(h, (uint64_t)xmlBufferLength(b));
        xmlBufferFree(b);
        if (nt) xmlFreeNotationTable(nt);
    }
    xmlNodePtr first = xmlFirstElementChild(root);
    xmlChar *nv = xmlValidNormalizeAttributeValue(doc, first, BAD_CAST "id",
                                                  BAD_CAST "  a1  ");
    if (nv) { h = av_s(h, nv); xmlFree(nv); }
    h = av_b(h, (uint64_t)xmlValidateAttributeValue(XML_ATTRIBUTE_NMTOKEN, BAD_CAST "tok"));
    h = av_b(h, (uint64_t)(xmlGetRefs(doc, BAD_CAST "a1") != NULL));
    h = av_b(h, (uint64_t)(int64_t)xmlRemoveRef(doc, first ? first->properties : NULL));
    /* xmlAddNotationDecl into a fresh dtd-less table via validation ctxt */
    xmlNotationPtr added = xmlAddNotationDecl(NULL, dtd, BAD_CAST "vwnota",
                                              BAD_CAST "pub-id", NULL);
    h = av_b(h, (uint64_t)(added != NULL));
    xmlFreeDoc(doc);
    return h;
}

static uint64_t av_tree(uint64_t h) {
    int opt = XML_PARSE_NOERROR | XML_PARSE_NOWARNING;
    xmlDocPtr doc = xmlReadDoc(BAD_CAST av_doc, "av.xml", NULL, opt);
    if (!doc) return av_b(h, 0);
    xmlNodePtr root = xmlDocGetRootElement(doc);
    xmlNodePtr first = xmlFirstElementChild(root);

    /* buffers */
    xmlBufferPtr b = xmlBufferCreateSize(8);
    xmlBufferSetAllocationScheme(b, XML_BUFFER_ALLOC_DOUBLEIT);
    xmlBufferAdd(b, BAD_CAST "middle", 6);
    xmlBufferAddHead(b, BAD_CAST "head-", 5);
    h = av_b(h, (uint64_t)xmlBufferGrow(b, 256));
    xmlBufferShrink(b, 2);
    h = digest64(h, xmlBufferContent(b), (size_t)xmlBufferLength(b));
    FILE *dn = fopen("/dev/null", "w");
    if (dn) { xmlBufferDump(dn, b); fclose(dn); }
    xmlChar *det = xmlBufferDetach(b);
    h = av_s(h, det); xmlFree(det);
    xmlBufferEmpty(b);
    xmlBufferFree(b);
    xmlBufferPtr bs = xmlBufferCreateStatic((void *)"staticmem", 9);
    if (bs) { h = av_b(h, (uint64_t)xmlBufferLength(bs)); xmlBufferFree(bs); }
    xmlNodeBufGetContent((b = xmlBufferCreate()), root);
    h = digest64(h, xmlBufferContent(b), (size_t)xmlBufferLength(b));
    xmlBufferFree(b);

    /* allocation scheme + compression getters/setters (restored) */
    xmlBufferAllocationScheme oldsch = xmlGetBufferAllocationScheme();
    xmlSetBufferAllocationScheme(oldsch);
    int oldcm = xmlGetDocCompressMode(doc);
    xmlSetDocCompressMode(doc, oldcm);

    /* ns props + qname surgery */
    xmlNsPtr ns = xmlNewNs(root, BAD_CAST "http://example.com/t", BAD_CAST "t");
    xmlAttrPtr np = xmlNewNsProp(first, ns, BAD_CAST "np", BAD_CAST "v1");
    h = av_b(h, (uint64_t)(np != NULL));
    xmlAttrPtr hp = xmlHasNsProp(first, BAD_CAST "np", BAD_CAST "http://example.com/t");
    h = av_b(h, (uint64_t)(hp != NULL));
    h = av_b(h, (uint64_t)(int64_t)xmlUnsetNsProp(first, ns, BAD_CAST "np"));
    xmlAttrPtr rp = xmlSetProp(first, BAD_CAST "toremove", BAD_CAST "x");
    h = av_b(h, (uint64_t)(int64_t)xmlRemoveProp(rp));
    xmlNodePtr sub = xmlNewChild(root, NULL, BAD_CAST "sub", NULL);
    xmlSetNs(sub, ns);
    h = av_b(h, (uint64_t)(int64_t)xmlReconciliateNs(doc, root));

    /* node builders / mutators */
    xmlNodePtr cr = xmlNewCharRef(doc, BAD_CAST "&#65;");
    if (cr) { xmlAddChild(sub, cr); }
    xmlNodeSetName(sub, BAD_CAST "renamed");
    xmlNodeSetContentLen(sub, BAD_CAST "len-content", 11);
    xmlNodeSetSpacePreserve(sub, 1);
    xmlChar *raw = xmlNodeListGetRawString(doc, first->children, 1);
    h = av_s(h, raw); if (raw) xmlFree(raw);
    xmlNodePtr t1 = xmlNewDocText(doc, BAD_CAST "left");
    xmlNodePtr t2 = xmlNewDocText(doc, BAD_CAST "right");
    xmlNodePtr merged = xmlTextMerge(t1, t2);
    if (merged) { h = av_s(h, merged->content); xmlFreeNode(merged); }

    /* list copies */
    xmlNodePtr cl = xmlCopyNodeList(root->children);
    if (cl) {
        xmlNodePtr fresh = xmlNewNode(NULL, BAD_CAST "holder");
        h = av_b(h, (uint64_t)(xmlAddChildList(fresh, cl) != NULL));
        xmlFreeNode(fresh);
    }
    xmlDocPtr doc2 = xmlNewDoc(BAD_CAST "1.0");
    xmlNodePtr dcl = xmlDocCopyNodeList(doc2, root->children);
    if (dcl) xmlFreeNodeList(dcl);
    xmlDtdPtr dtdc = xmlCopyDtd(doc->intSubset);
    if (dtdc) xmlFreeDtd(dtdc);

    /* DOMWrap */
    xmlDOMWrapCtxtPtr wc2 = xmlDOMWrapNewCtxt();
    if (wc2) {
        h = av_b(h, (uint64_t)(int64_t)xmlDOMWrapReconcileNamespaces(wc2, root, 0));
        xmlNodePtr clone = NULL;
        h = av_b(h, (uint64_t)(int64_t)xmlDOMWrapCloneNode(wc2, doc, first, &clone,
                                                           doc, NULL, 1, 0));
        if (clone) {
            h = av_b(h, (uint64_t)(int64_t)xmlDOMWrapAdoptNode(wc2, doc, clone,
                                                               doc2, NULL, 0));
            xmlDocSetRootElement(doc2, clone);
        }
        xmlNodePtr rm = xmlNewChild(root, NULL, BAD_CAST "togo", NULL);
        h = av_b(h, (uint64_t)(int64_t)xmlDOMWrapRemoveNode(wc2, doc, rm, 0));
        xmlFreeNode(rm);
        xmlDOMWrapFreeCtxt(wc2);
    }
    xmlFreeDoc(doc2);

    /* node-callback registration hooks (registered as NULL, then cleared) */
    xmlRegisterNodeFunc oldreg = xmlRegisterNodeDefault(NULL);
    xmlRegisterNodeDefault(oldreg);
    xmlDeregisterNodeFunc olddereg = xmlDeregisterNodeDefault(NULL);
    xmlDeregisterNodeDefault(olddereg);
    xmlFreeDoc(doc);
    return h;
}

static uint64_t av_saveio(uint64_t h) {
    int opt = XML_PARSE_NOERROR | XML_PARSE_NOWARNING;
    const char *scratch = "/tmp/vw_lx_save.xml";
    xmlDocPtr doc = xmlReadDoc(BAD_CAST av_doc, "av.xml", NULL, opt);
    if (!doc) return av_b(h, 0);
    xmlNodePtr root = xmlDocGetRootElement(doc);

    /* xmlsave: every sink variant; scratch-file bytes are folded */
    uint64_t iodig = 0;
    xmlSaveCtxtPtr sc = xmlSaveToIO(av_iowrite, NULL, &iodig, "UTF-8", 0);
    if (sc) {
        xmlSaveSetEscape(sc, NULL);
        xmlSaveSetAttrEscape(sc, NULL);
        xmlSaveDoc(sc, doc);
        xmlSaveTree(sc, root);
        xmlSaveClose(sc);
        h = av_b(h, iodig);
    }
    sc = xmlSaveToFilename(scratch, "UTF-8", 0);
    if (sc) {
        xmlSaveDoc(sc, doc);
        xmlSaveClose(sc);
        size_t sn; uint8_t *sb = read_file(scratch, &sn);
        h = digest64(h, sb, sn); free(sb);
    }
    int sfd = open(scratch, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (sfd >= 0) {
        sc = xmlSaveToFd(sfd, "UTF-8", 0);
        if (sc) { xmlSaveDoc(sc, doc); xmlSaveClose(sc); }
        close(sfd);
        size_t sn; uint8_t *sb = read_file(scratch, &sn);
        h = digest64(h, sb, sn); free(sb);
    }
    h = av_b(h, (uint64_t)(int64_t)xmlSaveFile(scratch, doc));
    h = av_b(h, (uint64_t)(int64_t)xmlSaveFileEnc(scratch, doc, "UTF-8"));
    h = av_b(h, (uint64_t)(int64_t)xmlSaveFormatFile(scratch, doc, 1));
    h = av_b(h, (uint64_t)(int64_t)xmlSaveFormatFileEnc(scratch, doc, "UTF-8", 1));
    {
        size_t sn; uint8_t *sb = read_file(scratch, &sn);
        h = digest64(h, sb, sn); free(sb);
    }
    xmlOutputBufferPtr ob = xmlOutputBufferCreateIO(av_iowrite, NULL, &iodig, NULL);
    if (ob) h = av_b(h, (uint64_t)(int64_t)xmlSaveFileTo(ob, doc, NULL));
    FILE *df = fopen(scratch, "w");
    if (df) {
        xmlDocDump(df, doc);
        xmlDocFormatDump(df, doc, 1);
        xmlElemDump(df, doc, root);
        fclose(df);
        size_t sn; uint8_t *sb = read_file(scratch, &sn);
        h = digest64(h, sb, sn); free(sb);
    }
    xmlChar *encdump = NULL; int encn = 0;
    xmlDocDumpMemoryEnc(doc, &encdump, &encn, "UTF-8");
    if (encdump) { h = digest64(h, encdump, (size_t)encn); xmlFree(encdump); }
    xmlBufferPtr ab = xmlBufferCreate();
    xmlAttrPtr idattr = xmlFirstElementChild(root)->properties;
    xmlAttrSerializeTxtContent(ab, doc, idattr, BAD_CAST "a<b&c\"d");
    h = digest64(h, xmlBufferContent(ab), (size_t)xmlBufferLength(ab));
    xmlBufferFree(ab);
    iodig = 0;
    ob = xmlOutputBufferCreateIO(av_iowrite, NULL, &iodig, NULL);
    if (ob) {
        xmlOutputBufferWriteString(ob, "probe");
        xmlOutputBufferFlush(ob);
        h = av_b(h, (uint64_t)(xmlOutputBufferGetContent(ob) != NULL));
        h = av_b(h, (uint64_t)(int64_t)xmlOutputBufferGetSize(ob));
        xmlOutputBufferClose(ob);
    }

    /* html save family */
    htmlDocPtr hd = htmlReadDoc(BAD_CAST av_html, "av.html", NULL,
                                HTML_PARSE_NOERROR | HTML_PARSE_NOWARNING);
    if (hd) {
        xmlNodePtr hroot = xmlDocGetRootElement(hd);
        FILE *hf = fopen(scratch, "w");
        if (hf) {
            htmlDocDump(hf, hd);
            htmlNodeDumpFile(hf, hd, hroot);
            htmlNodeDumpFileFormat(hf, hd, hroot, "UTF-8", 1);
            fclose(hf);
            size_t sn; uint8_t *sb = read_file(scratch, &sn);
            h = digest64(h, sb, sn); free(sb);
        }
        h = av_b(h, (uint64_t)(int64_t)htmlSaveFile(scratch, hd));
        h = av_b(h, (uint64_t)(int64_t)htmlSaveFileEnc(scratch, hd, "UTF-8"));
        h = av_b(h, (uint64_t)(int64_t)htmlSaveFileFormat(scratch, hd, "UTF-8", 1));
        {
            size_t sn; uint8_t *sb = read_file(scratch, &sn);
            h = digest64(h, sb, sn); free(sb);
        }
        xmlBufferPtr hb = xmlBufferCreate();
        h = av_b(h, (uint64_t)(int64_t)htmlNodeDump(hb, hd, hroot));
        h = digest64(h, xmlBufferContent(hb), (size_t)xmlBufferLength(hb));
        xmlBufferFree(hb);
        iodig = 0;
        ob = xmlOutputBufferCreateIO(av_iowrite, NULL, &iodig, NULL);
        if (ob) {
            htmlNodeDumpOutput(ob, hd, hroot, NULL);
            htmlDocContentDumpOutput(ob, hd, NULL);
            xmlOutputBufferClose(ob);
            h = av_b(h, iodig);
        }
        xmlFreeDoc(hd);
    }
    remove(scratch);

    /* xmlIO input/output constructors */
    xmlOutputBufferPtr o2 = xmlOutputBufferCreateFilename(scratch, NULL, 0);
    if (o2) { xmlOutputBufferWrite(o2, 3, "abc"); xmlOutputBufferClose(o2); remove(scratch); }
    o2 = __xmlOutputBufferCreateFilename(scratch, NULL, 0);
    if (o2) { xmlOutputBufferClose(o2); remove(scratch); }
    xmlOutputBufferCreateFilenameFunc oldout = xmlOutputBufferCreateFilenameDefault(NULL);
    xmlOutputBufferCreateFilenameDefault(oldout);
    xmlParserInputBufferCreateFilenameFunc oldin = xmlParserInputBufferCreateFilenameDefault(NULL);
    xmlParserInputBufferCreateFilenameDefault(oldin);
    int nfd = open("/dev/null", O_WRONLY);
    if (nfd >= 0) {
        xmlOutputBufferPtr o3 = xmlOutputBufferCreateFd(nfd, NULL);
        if (o3) xmlOutputBufferClose(o3);
        close(nfd);
    }
    struct av_io rio = { av_doc, sizeof(av_doc) - 1, 0 };
    xmlParserInputBufferPtr ib = xmlParserInputBufferCreateIO(av_ioread, av_ioclose,
                                                              &rio, XML_CHAR_ENCODING_UTF8);
    xmlParserCtxtPtr pctxt = xmlNewParserCtxt();
    if (ib && pctxt) {
        xmlParserInputPtr st = xmlNewIOInputStream(pctxt, ib, XML_CHAR_ENCODING_UTF8);
        if (st) {
            h = av_b(h, (uint64_t)(int64_t)xmlParserInputGrow(st, 16));
            h = av_b(h, (uint64_t)(int64_t)xmlParserInputRead(st, 16));
            xmlParserInputShrink(st);
            xmlFreeInputStream(st);
        } else {
            xmlFreeParserInputBuffer(ib);
        }
    } else if (ib) {
        xmlFreeParserInputBuffer(ib);
    }
    if (pctxt) {
        xmlParserInputPtr ss = xmlNewStringInputStream(pctxt, BAD_CAST "<s/>");
        if (ss) xmlFreeInputStream(ss);
        xmlFreeParserCtxt(pctxt);
    }
    FILE *rf = fopen("/dev/null", "r");
    if (rf) {
        xmlParserInputBufferPtr fb = xmlParserInputBufferCreateFile(rf, XML_CHAR_ENCODING_NONE);
        if (fb) xmlFreeParserInputBuffer(fb);
        fclose(rf);
    }
    int rfd = open("/dev/null", O_RDONLY);
    if (rfd >= 0) {
        xmlParserInputBufferPtr fdb = xmlParserInputBufferCreateFd(rfd, XML_CHAR_ENCODING_NONE);
        if (fdb) xmlFreeParserInputBuffer(fdb);
        close(rfd);
    }
    xmlChar *norm = xmlNormalizeWindowsPath(BAD_CAST "C:\\dir\\file.xml");
    h = av_s(h, norm); if (norm) xmlFree(norm);
    xmlExternalEntityLoader oldld = xmlGetExternalEntityLoader();
    xmlSetExternalEntityLoader(oldld);
    /* pop the callback stacks, then restore the defaults */
    h = av_b(h, (uint64_t)(int64_t)xmlPopInputCallbacks());
    h = av_b(h, (uint64_t)(int64_t)xmlPopOutputCallbacks());
    xmlRegisterDefaultInputCallbacks();
    xmlRegisterDefaultOutputCallbacks();
    xmlFreeDoc(doc);
    return h;
}

static uint64_t av_globals(uint64_t h) {
    /* thread-default setters return the previous value: set, then restore */
    xmlThrDefBufferAllocScheme(xmlThrDefBufferAllocScheme(XML_BUFFER_ALLOC_DOUBLEIT));
    xmlThrDefDefaultBufferSize(xmlThrDefDefaultBufferSize(4096));
    xmlThrDefDoValidityCheckingDefaultValue(xmlThrDefDoValidityCheckingDefaultValue(0));
    xmlThrDefGetWarningsDefaultValue(xmlThrDefGetWarningsDefaultValue(1));
    xmlThrDefIndentTreeOutput(xmlThrDefIndentTreeOutput(1));
    xmlThrDefKeepBlanksDefaultValue(xmlThrDefKeepBlanksDefaultValue(1));
    xmlThrDefLineNumbersDefaultValue(xmlThrDefLineNumbersDefaultValue(0));
    xmlThrDefLoadExtDtdDefaultValue(xmlThrDefLoadExtDtdDefaultValue(0));
    xmlThrDefParserDebugEntities(xmlThrDefParserDebugEntities(0));
    xmlThrDefPedanticParserDefaultValue(xmlThrDefPedanticParserDefaultValue(0));
    xmlThrDefSaveNoEmptyTags(xmlThrDefSaveNoEmptyTags(0));
    xmlThrDefSubstituteEntitiesDefaultValue(xmlThrDefSubstituteEntitiesDefaultValue(0));
    const char *oldind = xmlThrDefTreeIndentString("  ");
    xmlThrDefTreeIndentString(oldind);
    xmlThrDefSetGenericErrorFunc(NULL, NULL);
    xmlThrDefSetStructuredErrorFunc(NULL, NULL);
    xmlThrDefRegisterNodeDefault(xmlThrDefRegisterNodeDefault(NULL));
    xmlThrDefDeregisterNodeDefault(xmlThrDefDeregisterNodeDefault(NULL));
    xmlThrDefOutputBufferCreateFilenameDefault(
        xmlThrDefOutputBufferCreateFilenameDefault(NULL));
    xmlThrDefParserInputBufferCreateFilenameDefault(
        xmlThrDefParserInputBufferCreateFilenameDefault(NULL));

    /* per-thread global-state accessors */
    h = av_s(h, BAD_CAST *__xmlParserVersion());
    h = av_b(h, (uint64_t)*__oldXMLWDcompatibility());
    h = av_b(h, (uint64_t)(*__xmlRegisterNodeDefaultValue() == NULL));
    h = av_b(h, (uint64_t)(*__xmlDeregisterNodeDefaultValue() == NULL));
    h = av_b(h, (uint64_t)(*__xmlOutputBufferCreateFilenameValue() == NULL));
    h = av_b(h, (uint64_t)(*__xmlStructuredErrorContext() == NULL));
    h = av_b(h, (uint64_t)(__htmlDefaultSAXHandler() != NULL));
    h = av_b(h, (uint64_t)(int64_t)xmlCheckThreadLocalStorage());
    xmlGlobalStatePtr gs = xmlGetGlobalState();
    h = av_b(h, (uint64_t)(gs != NULL));
    xmlInitGlobals();
    xmlCleanupGlobals();

    /* threads shims */
    xmlInitThreads();
    xmlLockLibrary();
    xmlUnlockLibrary();
    h = av_b(h, (uint64_t)(xmlGetThreadId() == xmlGetThreadId()));
    xmlCleanupThreads();
    return h;
}

static uint64_t av_sax2err(uint64_t h) {
    /* a fully-parsed ctxt gives the SAX2 getters valid state */
    xmlParserCtxtPtr c = xmlCreateDocParserCtxt(BAD_CAST av_doc);
    if (!c) return av_b(h, 0);
    int pr = xmlParseDocument(c);
    h = av_b(h, (uint64_t)(int64_t)pr);
    h = av_b(h, (uint64_t)(int64_t)xmlSAX2GetLineNumber(c));
    h = av_b(h, (uint64_t)(int64_t)xmlSAX2GetColumnNumber(c));
    h = av_s(h, (const xmlChar *)xmlSAX2GetPublicId(c));
    h = av_b(h, (uint64_t)(xmlSAX2GetSystemId(c) != NULL));
    h = av_b(h, (uint64_t)(int64_t)xmlSAX2IsStandalone(c));
    h = av_b(h, (uint64_t)(int64_t)xmlSAX2HasInternalSubset(c));
    h = av_b(h, (uint64_t)(int64_t)xmlSAX2HasExternalSubset(c));
    xmlSAX2IgnorableWhitespace(c, BAD_CAST "  ", 2);
    if (c->myDoc && c->myDoc->intSubset) {
        xmlSAX2NotationDecl(c, BAD_CAST "vwn", NULL, BAD_CAST "sys-n");
        xmlSAX2UnparsedEntityDecl(c, BAD_CAST "vwe", NULL, BAD_CAST "sys-e",
                                  BAD_CAST "gif");
    }
    /* error channel: printf-style reporters write to stderr (not the oracle) */
    xmlParserError(c, "%s probe\n", "generic");
    xmlParserWarning(c, "%s probe\n", "warning");
    xmlParserValidityError(c, "%s probe\n", "validity");
    xmlParserValidityWarning(c, "%s probe\n", "vwarn");
    xmlParserPrintFileInfo(c->input);
    xmlParserPrintFileContext(c->input);
    const xmlError *le = xmlCtxtGetLastError(c);
    h = av_b(h, le ? (uint64_t)le->code : 0);
    xmlCtxtResetLastError(c);
    if (c->myDoc) { xmlFreeDoc(c->myDoc); c->myDoc = NULL; }
    xmlFreeParserCtxt(c);

    xmlGenericErrorFunc gef = NULL;
    initGenericErrorDefaultFunc(&gef);
    h = av_b(h, (uint64_t)(gef != NULL));
    xmlSetGenericErrorFunc(NULL, NULL);
    xmlSetStructuredErrorFunc(NULL, NULL);

    /* default SAX handler initializers */
    xmlSAXHandler sh;
    memset(&sh, 0, sizeof(sh));
    xmlSAX2InitDefaultSAXHandler(&sh, 1);
    h = av_b(h, (uint64_t)(sh.startElement != NULL));
    xmlDefaultSAXHandlerInit();
    htmlDefaultSAXHandlerInit();
    int oldv = xmlSAXDefaultVersion(2);
    xmlSAXDefaultVersion(oldv);
    return h;
}

static const char av_xsd[] =
    "<xs:schema xmlns:xs=\"http://www.w3.org/2001/XMLSchema\">"
    "<xs:element name=\"doc\" type=\"xs:string\"/></xs:schema>";
static const char av_rng[] =
    "<element name=\"doc\" xmlns=\"http://relaxng.org/ns/structure/1.0\">"
    "<text/></element>";
static const char av_sct[] =
    "<schema xmlns=\"http://purl.oclc.org/dsdl/schematron\">"
    "<pattern><rule context=\"doc\"><assert test=\"true()\">ok</assert>"
    "</rule></pattern></schema>";
static const char av_inst[] = "<doc>x</doc>";

static uint64_t av_schemas(uint64_t h) {
    int opt = XML_PARSE_NOERROR | XML_PARSE_NOWARNING;

    /* XSD: mem + doc parser ctxts, valid ctxt accessor battery */
    xmlSchemaParserCtxtPtr sp = xmlSchemaNewMemParserCtxt(av_xsd, (int)sizeof(av_xsd) - 1);
    if (sp) {
        xmlSchemaPtr sch = xmlSchemaParse(sp);
        h = av_b(h, (uint64_t)(sch != NULL));
        if (sch) {
            FILE *dn = fopen("/dev/null", "w");
            if (dn) { xmlSchemaDump(dn, sch); fclose(dn); }
            xmlSchemaValidCtxtPtr sv = xmlSchemaNewValidCtxt(sch);
            if (sv) {
                xmlSchemaSetValidOptions(sv, XML_SCHEMA_VAL_VC_I_CREATE);
                h = av_b(h, (uint64_t)xmlSchemaValidCtxtGetOptions(sv));
                h = av_b(h, (uint64_t)(xmlSchemaValidCtxtGetParserCtxt(sv) == NULL));
                xmlSchemaValidateSetFilename(sv, "av-inst.xml");
                xmlSchemaValidateSetLocator(sv, NULL, NULL);
                xmlSchemaGetValidErrors(sv, &(xmlSchemaValidityErrorFunc){NULL},
                                        &(xmlSchemaValidityWarningFunc){NULL},
                                        &(void *){NULL});
                xmlDocPtr inst = xmlReadDoc(BAD_CAST av_inst, "i.xml", NULL, opt);
                if (inst) {
                    h = av_b(h, (uint64_t)(int64_t)xmlSchemaValidateDoc(sv, inst));
                    h = av_b(h, (uint64_t)(int64_t)xmlSchemaIsValid(sv));
                    h = av_b(h, (uint64_t)(int64_t)xmlSchemaValidateOneElement(
                                 sv, xmlDocGetRootElement(inst)));
                    xmlFreeDoc(inst);
                }
                struct av_io sio = { av_inst, sizeof(av_inst) - 1, 0 };
                xmlParserInputBufferPtr sib = xmlParserInputBufferCreateIO(
                    av_ioread, av_ioclose, &sio, XML_CHAR_ENCODING_UTF8);
                if (sib)
                    h = av_b(h, (uint64_t)(int64_t)xmlSchemaValidateStream(
                                 sv, sib, XML_CHAR_ENCODING_UTF8, NULL, NULL));
                xmlSchemaFreeValidCtxt(sv);
            }
            /* SAX plug/unplug on a plain SAX parse */
            xmlSchemaValidCtxtPtr sv2 = xmlSchemaNewValidCtxt(sch);
            if (sv2) {
                xmlSAXHandler sh; memset(&sh, 0, sizeof(sh));
                xmlSAX2InitDefaultSAXHandler(&sh, 0);
                xmlSAXHandlerPtr shp = &sh;
                void *ud = NULL;
                xmlSchemaSAXPlugPtr plug = xmlSchemaSAXPlug(sv2, &shp, &ud);
                h = av_b(h, (uint64_t)(plug != NULL));
                if (plug) h = av_b(h, (uint64_t)(int64_t)xmlSchemaSAXUnplug(plug));
                xmlSchemaFreeValidCtxt(sv2);
            }
            xmlSchemaFree(sch);
        }
        xmlSchemaGetParserErrors(sp, &(xmlSchemaValidityErrorFunc){NULL},
                                 &(xmlSchemaValidityWarningFunc){NULL}, &(void *){NULL});
        xmlSchemaFreeParserCtxt(sp);
    }
    xmlDocPtr xsdd = xmlReadDoc(BAD_CAST av_xsd, "s.xsd", NULL, opt);
    if (xsdd) {
        xmlSchemaParserCtxtPtr dp = xmlSchemaNewDocParserCtxt(xsdd);
        if (dp) {
            xmlSchemaPtr s2 = xmlSchemaParse(dp);
            h = av_b(h, (uint64_t)(s2 != NULL));
            if (s2) xmlSchemaFree(s2);
            xmlSchemaFreeParserCtxt(dp);
        }
        xmlFreeDoc(xsdd);
    }
    h = av_b(h, (uint64_t)(int64_t)xmlSchemaValidateFile(NULL, "nosuch.xml", 0));

    /* schemastypes values */
    xmlSchemaTypePtr strt = xmlSchemaGetPredefinedType(
        BAD_CAST "string", BAD_CAST "http://www.w3.org/2001/XMLSchema");
    h = av_b(h, (uint64_t)(strt != NULL));
    xmlSchemaValPtr v1 = xmlSchemaNewStringValue(XML_SCHEMAS_STRING,
                                                 xmlStrdup(BAD_CAST "sval"));
    xmlSchemaValPtr vq = xmlSchemaNewQNameValue(xmlStrdup(BAD_CAST "ns"),
                                                xmlStrdup(BAD_CAST "ln"));
    xmlSchemaValPtr vn = xmlSchemaNewNOTATIONValue(xmlStrdup(BAD_CAST "nota"),
                                                   xmlStrdup(BAD_CAST "nsn"));
    if (v1) {
        xmlSchemaValPtr vc = xmlSchemaCopyValue(v1);
        h = av_b(h, (uint64_t)(vc != NULL));
        h = av_b(h, (uint64_t)(int64_t)xmlSchemaValueGetAsBoolean(v1));
        const xmlChar *canon = NULL;
        h = av_b(h, (uint64_t)(int64_t)xmlSchemaGetCanonValueWhtsp(
                     v1, &canon, XML_SCHEMA_WHITESPACE_COLLAPSE));
        if (canon) { h = av_s(h, canon); xmlFree((void *)canon); }
        if (vq) h = av_b(h, (uint64_t)(int64_t)xmlSchemaValueAppend(v1, vq));
        if (vc) xmlSchemaFreeValue(vc);
        xmlSchemaFreeValue(v1);   /* frees appended vq too */
    } else if (vq) {
        xmlSchemaFreeValue(vq);
    }
    if (vn) xmlSchemaFreeValue(vn);
    xmlChar *ws = xmlSchemaWhiteSpaceReplace(BAD_CAST "a\tb\nc");
    h = av_s(h, ws); if (ws) xmlFree(ws);
    xmlSchemaTypePtr lst = xmlSchemaGetBuiltInType(XML_SCHEMAS_NMTOKENS);
    if (lst)
        h = av_b(h, (uint64_t)(xmlSchemaGetBuiltInListSimpleTypeItemType(lst) != NULL));

    /* RelaxNG: mem + doc ctxts, error accessors, push validation */
    xmlRelaxNGParserCtxtPtr rp = xmlRelaxNGNewMemParserCtxt(av_rng, (int)sizeof(av_rng) - 1);
    if (rp) {
        xmlRelaxNGSetParserErrors(rp, NULL, NULL, NULL);
        xmlRelaxNGSetParserStructuredErrors(rp, NULL, NULL);
        h = av_b(h, (uint64_t)(int64_t)xmlRelaxParserSetFlag(rp, 0));
        xmlRelaxNGValidityErrorFunc re = NULL;
        xmlRelaxNGValidityWarningFunc rw = NULL;
        void *ra = NULL;
        xmlRelaxNGGetParserErrors(rp, &re, &rw, &ra);
        xmlRelaxNGPtr rg = xmlRelaxNGParse(rp);
        h = av_b(h, (uint64_t)(rg != NULL));
        if (rg) {
            FILE *dn = fopen("/dev/null", "w");
            if (dn) { xmlRelaxNGDump(dn, rg); xmlRelaxNGDumpTree(dn, rg); fclose(dn); }
            xmlRelaxNGValidCtxtPtr rv = xmlRelaxNGNewValidCtxt(rg);
            if (rv) {
                xmlRelaxNGSetValidErrors(rv, NULL, NULL, NULL);
                xmlRelaxNGSetValidStructuredErrors(rv, NULL, NULL);
                xmlRelaxNGGetValidErrors(rv, &re, &rw, &ra);
                xmlDocPtr inst = xmlReadDoc(BAD_CAST av_inst, "i.xml", NULL, opt);
                if (inst) {
                    xmlNodePtr iroot = xmlDocGetRootElement(inst);
                    h = av_b(h, (uint64_t)(int64_t)xmlRelaxNGValidatePushElement(rv, inst, iroot));
                    h = av_b(h, (uint64_t)(int64_t)xmlRelaxNGValidatePushCData(rv, BAD_CAST "x", 1));
                    h = av_b(h, (uint64_t)(int64_t)xmlRelaxNGValidatePopElement(rv, inst, iroot));
                    h = av_b(h, (uint64_t)(int64_t)xmlRelaxNGValidateFullElement(rv, inst, iroot));
                    xmlFreeDoc(inst);
                }
                xmlRelaxNGFreeValidCtxt(rv);
            }
            xmlRelaxNGFree(rg);
        }
        xmlRelaxNGFreeParserCtxt(rp);
    }
    xmlDocPtr rngd = xmlReadDoc(BAD_CAST av_rng, "r.rng", NULL, opt);
    if (rngd) {
        xmlRelaxNGParserCtxtPtr rdp = xmlRelaxNGNewDocParserCtxt(rngd);
        if (rdp) {
            xmlRelaxNGPtr rg2 = xmlRelaxNGParse(rdp);
            if (rg2) xmlRelaxNGFree(rg2);
            xmlRelaxNGFreeParserCtxt(rdp);
        }
        xmlFreeDoc(rngd);
    }

    /* schematron: mem + doc ctxts */
    xmlSchematronParserCtxtPtr tp =
        xmlSchematronNewMemParserCtxt(av_sct, (int)sizeof(av_sct) - 1);
    if (tp) {
        xmlSchematronPtr st = xmlSchematronParse(tp);
        h = av_b(h, (uint64_t)(st != NULL));
        if (st) {
            xmlSchematronValidCtxtPtr tv =
                xmlSchematronNewValidCtxt(st, XML_SCHEMATRON_OUT_QUIET);
            if (tv) {
                xmlSchematronSetValidStructuredErrors(tv, NULL, NULL);
                xmlDocPtr inst = xmlReadDoc(BAD_CAST av_inst, "i.xml", NULL, opt);
                if (inst) {
                    h = av_b(h, (uint64_t)(int64_t)xmlSchematronValidateDoc(tv, inst));
                    xmlFreeDoc(inst);
                }
                xmlSchematronFreeValidCtxt(tv);
            }
            xmlSchematronFree(st);
        }
        xmlSchematronFreeParserCtxt(tp);
    }
    xmlDocPtr sctd = xmlReadDoc(BAD_CAST av_sct, "s.sct", NULL, opt);
    if (sctd) {
        xmlSchematronParserCtxtPtr tdp = xmlSchematronNewDocParserCtxt(sctd);
        if (tdp) {
            xmlSchematronPtr st2 = xmlSchematronParse(tdp);
            if (st2) xmlSchematronFree(st2);
            xmlSchematronFreeParserCtxt(tdp);
        }
        /* doc parser ctxt takes ownership of the doc? it does not: free it */
        xmlFreeDoc(sctd);
    }
    return h;
}

/* shell input: xmlShell takes a readline callback, not a FILE */
static const char *av_shell_cmds[] = {
    "pwd", "ls", "dir /", "base", "cat /", "du", "cd doc", "pwd",
    "xpath //item", "validate", "write /tmp/vw_lx_shell_w.xml",
    "save /tmp/vw_lx_shell_s.xml", "grep item", "quit",
};
static size_t av_shell_idx;
static char *av_shell_readline(char *prompt) {
    (void)prompt;
    if (av_shell_idx >= sizeof(av_shell_cmds) / sizeof(av_shell_cmds[0]))
        return NULL;
    return strdup(av_shell_cmds[av_shell_idx++]);   /* xmlShell frees it */
}
static void *hash_copier(void *payload, const xmlChar *name) {
    (void)name;
    return payload;
}
static int av_strvprintf(xmlChar *buf, int len, const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    int r = xmlStrVPrintf(buf, len, fmt, ap);
    va_end(ap);
    return r;
}

static uint64_t av_misc(uint64_t h, const char *path) {
    int opt = XML_PARSE_NOERROR | XML_PARSE_NOWARNING;
    FILE *dn = fopen("/dev/null", "w");
    xmlDocPtr doc = xmlReadDoc(BAD_CAST av_doc, "av.xml", NULL, opt);
    if (!doc || !dn) { if (doc) xmlFreeDoc(doc); if (dn) fclose(dn); return av_b(h, 0); }
    xmlNodePtr root = xmlDocGetRootElement(doc);

    /* debug shell: one scripted session covers the xmlShell* command set */
    av_shell_idx = 0;
    xmlShell(doc, (char *)"av.xml", av_shell_readline, dn);
    remove("/tmp/vw_lx_shell_w.xml");
    remove("/tmp/vw_lx_shell_s.xml");
    h = av_s(h, BAD_CAST xmlBoolToText(1));
    xmlLsOneNode(dn, root);
    if (xmlFirstElementChild(root)->properties)
        xmlDebugDumpAttrList(dn, xmlFirstElementChild(root)->properties, 1);

    /* global catalog surface (default catalog; state identical on both sides) */
    xmlCatalogSetDebug(0);
    xmlCatalogSetDefaultPrefer(XML_CATA_PREFER_PUBLIC);
    h = av_b(h, (uint64_t)(int64_t)xmlLoadCatalog(path));
    xmlLoadCatalogs("/tmp/vw_nocat1.xml:/tmp/vw_nocat2.xml");
    h = av_b(h, (uint64_t)(int64_t)xmlCatalogAdd(BAD_CAST "public",
                 BAD_CAST "-//VW//GLOB//EN", BAD_CAST "file:///tmp/g.dtd"));
    xmlChar *cr1 = xmlCatalogResolvePublic(BAD_CAST "-//VW//GLOB//EN");
    h = av_s(h, cr1); if (cr1) xmlFree(cr1);
    xmlChar *cr2 = xmlCatalogResolveSystem(BAD_CAST "http://nosuch/s.dtd");
    h = av_b(h, (uint64_t)(cr2 != NULL)); if (cr2) xmlFree(cr2);
    h = av_b(h, (uint64_t)(xmlCatalogGetPublic(BAD_CAST "-//VW//GLOB//EN") != NULL));
    h = av_b(h, (uint64_t)(xmlCatalogGetSystem(BAD_CAST "http://nosuch/s.dtd") != NULL));
    h = av_b(h, (uint64_t)(int64_t)xmlCatalogRemove(BAD_CAST "-//VW//GLOB//EN"));
    xmlCatalogDump(dn);
    h = av_b(h, (uint64_t)(int64_t)xmlCatalogConvert());
    h = av_b(h, (uint64_t)(xmlLoadSGMLSuperCatalog("/tmp/vw_nosuch.sgml") != NULL));
    void *loc = xmlCatalogAddLocal(NULL, BAD_CAST path);
    h = av_b(h, (uint64_t)(loc != NULL));
    xmlChar *lr = xmlCatalogLocalResolve(loc, BAD_CAST "-//X//Y//EN", NULL);
    h = av_b(h, (uint64_t)(lr != NULL)); if (lr) xmlFree(lr);
    lr = xmlCatalogLocalResolveURI(loc, BAD_CAST "http://nosuch/u");
    h = av_b(h, (uint64_t)(lr != NULL)); if (lr) xmlFree(lr);
    xmlCatalogFreeLocal(loc);

    /* memory hooks: fetch current, install back (no-op swap) */
    xmlFreeFunc ff; xmlMallocFunc mf, mfa; xmlReallocFunc rf2; xmlStrdupFunc sf;
    if (xmlMemGet(&ff, &mf, &rf2, &sf) == 0)
        h = av_b(h, (uint64_t)(int64_t)xmlMemSetup(ff, mf, rf2, sf));
    if (xmlGcMemGet(&ff, &mf, &mfa, &rf2, &sf) == 0)
        h = av_b(h, (uint64_t)(int64_t)xmlGcMemSetup(ff, mf, mfa, rf2, sf));
    xmlMemDisplay(dn);
    xmlMemDisplayLast(dn, 64);
    xmlMemShow(dn, 4);
    xmlMemoryDump();
    void *ml = xmlMallocAtomicLoc(24, "driver.c", __LINE__);
    h = av_b(h, (uint64_t)(ml != NULL));
    h = av_b(h, (uint64_t)(xmlMemSize(ml) >= 0));
    if (ml) xmlMemFree(ml);   /* *Loc allocs carry a debug header */
    xmlInitMemory();

    /* encoding aliases + buffer conversion funcs */
    h = av_b(h, (uint64_t)(int64_t)xmlAddEncodingAlias("UTF-8", "vw-alias"));
    h = av_b(h, (uint64_t)(int64_t)xmlDelEncodingAlias("vw-alias"));
    xmlInitCharEncodingHandlers();
    xmlCharEncodingHandlerPtr u16 = xmlGetCharEncodingHandler(XML_CHAR_ENCODING_UTF16LE);
    if (u16) {
        xmlBufferPtr in = xmlBufferCreate();
        xmlBufferAdd(in, (const xmlChar *)"h\0i\0", 4);
        xmlBufferPtr out = xmlBufferCreate();
        h = av_b(h, (uint64_t)(int64_t)xmlCharEncFirstLine(u16, out, in));
        h = av_b(h, (uint64_t)(int64_t)xmlCharEncInFunc(u16, out, in));
        h = digest64(h, xmlBufferContent(out), (size_t)xmlBufferLength(out));
        xmlBufferPtr back = xmlBufferCreate();
        h = av_b(h, (uint64_t)(int64_t)xmlCharEncOutFunc(u16, back, out));
        xmlBufferFree(in); xmlBufferFree(out); xmlBufferFree(back);
    }
    /* xmlNewCharEncodingHandler registers the handler itself (and calls
     * xmlRegisterCharEncodingHandler internally): registering twice would
     * double-free at xmlCleanupParser */
    xmlCharEncodingHandlerPtr nh = xmlNewCharEncodingHandler("vw-null-enc", NULL, NULL);
    h = av_b(h, (uint64_t)(nh != NULL));

    /* chvalid deprecated function forms + unicode leftovers */
    uint64_t cv = 0;
    cv = cv * 2 + (uint64_t)xmlIsBaseChar(0x41);
    cv = cv * 2 + (uint64_t)xmlIsBlank(0x20);
    cv = cv * 2 + (uint64_t)xmlIsChar(0x42);
    cv = cv * 2 + (uint64_t)xmlIsCombining(0x300);
    cv = cv * 2 + (uint64_t)xmlIsDigit(0x37);
    cv = cv * 2 + (uint64_t)xmlIsExtender(0xB7);
    cv = cv * 2 + (uint64_t)xmlIsIdeographic(0x4E2D);
    cv = cv * 2 + (uint64_t)xmlIsPubidChar(0x41);
    cv = cv * 2 + (uint64_t)xmlUCSIsBlockElements(0x2588);
    cv = cv * 2 + (uint64_t)xmlUCSIsCombiningMarksforSymbols(0x20D0);
    cv = cv * 2 + (uint64_t)xmlUCSIsGreekandCoptic(0x3B1);
    cv = cv * 2 + (uint64_t)xmlUCSIsHighPrivateUseSurrogates(0xDB80);
    cv = cv * 2 + (uint64_t)xmlUCSIsHighSurrogates(0xD800);
    cv = cv * 2 + (uint64_t)xmlUCSIsLowSurrogates(0xDC00);
    cv = cv * 2 + (uint64_t)xmlUCSIsPrivateUse(0xE000);
    cv = cv * 2 + (uint64_t)xmlUCSIsTaiXuanJingSymbols(0x1D300);
    h = av_b(h, cv);

    /* xinclude variants over a doc with one xi element */
    static const char av_xi[] =
        "<d xmlns:xi=\"http://www.w3.org/2001/XInclude\">"
        "<xi:include href=\"nosuch.xml\"><xi:fallback>fb</xi:fallback></xi:include></d>";
    xmlDocPtr xid = xmlReadDoc(BAD_CAST av_xi, "xi.xml", NULL, opt);
    if (xid) {
        h = av_b(h, (uint64_t)(int64_t)xmlXIncludeProcess(xid));
        xmlFreeDoc(xid);
    }
    xid = xmlReadDoc(BAD_CAST av_xi, "xi.xml", NULL, opt);
    if (xid) {
        h = av_b(h, (uint64_t)(int64_t)xmlXIncludeProcessTree(xmlDocGetRootElement(xid)));
        xmlFreeDoc(xid);
    }
    xid = xmlReadDoc(BAD_CAST av_xi, "xi.xml", NULL, opt);
    if (xid) {
        h = av_b(h, (uint64_t)(int64_t)xmlXIncludeProcessTreeFlags(
                     xmlDocGetRootElement(xid), opt));
        xmlFreeDoc(xid);
    }
    xid = xmlReadDoc(BAD_CAST av_xi, "xi.xml", NULL, opt);
    if (xid) {
        xmlXIncludeCtxtPtr xc = xmlXIncludeNewContext(xid);
        if (xc) {
            h = av_b(h, (uint64_t)(int64_t)xmlXIncludeProcessNode(
                         xc, xmlDocGetRootElement(xid)));
            xmlXIncludeFreeContext(xc);
        }
        xmlFreeDoc(xid);
    }

    /* list + hash leftovers */
    xmlListPtr l = xmlListCreate(NULL, list_cmp);
    for (int k = 0; k < 8; k++) xmlListPushBack(l, (void *)(uintptr_t)(k + 1));
    h = av_b(h, (uint64_t)(xmlListEnd(l) != NULL));
    h = av_b(h, (uint64_t)(xmlListReverseSearch(l, (void *)(uintptr_t)3) != NULL));
    uint64_t acc = 0;
    xmlListReverseWalk(l, list_walker, &acc);
    h = av_b(h, acc);
    h = av_b(h, (uint64_t)(int64_t)xmlListRemoveLast(l, (void *)(uintptr_t)5));
    h = av_b(h, (uint64_t)(int64_t)xmlListRemoveAll(l, (void *)(uintptr_t)2));
    xmlListDelete(l);
    xmlHashTablePtr ht = xmlHashCreate(8);
    xmlHashAddEntry(ht, BAD_CAST "k1", (void *)(uintptr_t)1);
    h = av_b(h, (uint64_t)(int64_t)xmlHashUpdateEntry(ht, BAD_CAST "k1",
                                                      (void *)(uintptr_t)2, NULL));
    h = av_b(h, (uint64_t)(int64_t)xmlHashUpdateEntry3(ht, BAD_CAST "k2", NULL, NULL,
                                                       (void *)(uintptr_t)3, NULL));
    h = av_b(h, (uint64_t)(uintptr_t)xmlHashQLookup(ht, NULL, BAD_CAST "k1"));
    xmlHashTablePtr htc = xmlHashCopy(ht, hash_copier);
    h = av_b(h, (uint64_t)(htc != NULL));
    if (htc) xmlHashFree(htc, NULL);
    h = av_b(h, (uint64_t)(int64_t)xmlHashRemoveEntry(ht, BAD_CAST "k2", NULL));
    xmlHashFree(ht, NULL);

    /* entities */
    xmlEntityPtr ne = xmlNewEntity(doc, BAD_CAST "vwent",
                                   XML_INTERNAL_GENERAL_ENTITY, NULL, NULL,
                                   BAD_CAST "content");
    h = av_b(h, (uint64_t)(ne != NULL));
    h = av_b(h, (uint64_t)(xmlGetDtdEntity(doc, BAD_CAST "vw") != NULL));
    xmlEntitiesTablePtr et = xmlCreateEntitiesTable();
    h = av_b(h, (uint64_t)(et != NULL));
    if (doc->intSubset && doc->intSubset->entities) {
        xmlEntitiesTablePtr etc2 =
            xmlCopyEntitiesTable((xmlEntitiesTablePtr)doc->intSubset->entities);
        if (etc2) {
            xmlBufferPtr eb = xmlBufferCreate();
            xmlDumpEntitiesTable(eb, etc2);
            /* hash-scan order is seed-dependent: fold length only */
            h = av_b(h, (uint64_t)xmlBufferLength(eb));
            xmlBufferFree(eb);
            xmlFreeEntitiesTable(etc2);
        }
    }
    if (et) xmlFreeEntitiesTable(et);

    /* dict */
    xmlInitializeDict();
    xmlDictPtr d1 = xmlDictCreate();
    const xmlChar *w1 = xmlDictLookup(d1, BAD_CAST "word", -1);
    h = av_b(h, (uint64_t)xmlDictExists(d1, BAD_CAST "word", -1) != 0 ? 1 : 0);
    h = av_b(h, (uint64_t)(xmlDictGetUsage(d1) > 0));
    xmlDictPtr d2 = xmlDictCreateSub(d1);
    if (d2) {
        h = av_b(h, (uint64_t)(xmlDictLookup(d2, BAD_CAST "word", -1) == w1));
        xmlDictFree(d2);
    }
    xmlDictFree(d1);
    xmlDictCleanup();

    /* module (same .so on both sides; booleans only) */
    xmlModulePtr mod = xmlModuleOpen("libm.so.6", 0);
    h = av_b(h, (uint64_t)(mod != NULL));
    if (mod) {
        void *sym = NULL;
        h = av_b(h, (uint64_t)(int64_t)xmlModuleSymbol(mod, "cos", &sym));
        h = av_b(h, (uint64_t)(sym != NULL));
        xmlModuleClose(mod);
    }
    xmlModulePtr mod2 = xmlModuleOpen("libm.so.6", 0);
    if (mod2) xmlModuleFree(mod2);

    /* strings */
    xmlChar sbuf[64];
    h = av_b(h, (uint64_t)(int64_t)xmlStrPrintf(sbuf, (int)sizeof(sbuf),
                                                "%s-%d", "fmt", 42));
    h = av_s(h, sbuf);
    h = av_b(h, (uint64_t)(int64_t)av_strvprintf(sbuf, (int)sizeof(sbuf),
                                                 "%s:%d", "vfmt", 7));
    h = av_s(h, sbuf);
    xmlChar *ss = xmlStrsub(BAD_CAST "substring", 3, 3);
    h = av_s(h, ss); if (ss) xmlFree(ss);

    /* regexp + automata leftovers */
    xmlRegexpPtr rx = xmlRegexpCompile(BAD_CAST "(a|b)c");
    if (rx) {
        /* xmlRegexpPrint reads atom strings the compacting pass freed: skip */
        h = av_b(h, (uint64_t)xmlRegexpExec(rx, BAD_CAST "ac"));
        xmlRegFreeRegexp(rx);
    }
    xmlAutomataPtr am = xmlNewAutomata();
    if (am) {
        xmlAutomataStatePtr s0 = xmlAutomataGetInitState(am);
        xmlAutomataStatePtr s1 = xmlAutomataNewCountTrans(am, s0, NULL,
                                                          BAD_CAST "x", 1, 3, NULL);
        xmlAutomataStatePtr s2 = xmlAutomataNewOnceTrans(am, s1, NULL,
                                                         BAD_CAST "y", 1, 1, NULL);
        if (s2) xmlAutomataSetFinalState(am, s2);
        xmlRegexpPtr ar = xmlAutomataCompile(am);
        h = av_b(h, (uint64_t)(ar != NULL));
        if (ar) xmlRegFreeRegexp(ar);
        xmlFreeAutomata(am);
    }

    /* uri leftovers */
    xmlURIPtr u = xmlParseURIRaw("http://host/a%20b", 1);
    if (u) { xmlPrintURI(dn, u); xmlFreeURI(u); }
    xmlChar *esc = xmlURIEscape(BAD_CAST "http://h/a b?q=1");
    h = av_s(h, esc); if (esc) xmlFree(esc);

    /* pattern streaming */
    xmlPatternPtr pat = xmlPatterncompile(BAD_CAST "//item", NULL, 0, NULL);
    if (pat) {
        xmlStreamCtxtPtr sctx = xmlPatternGetStreamCtxt(pat);
        if (sctx) {
            h = av_b(h, (uint64_t)(int64_t)xmlStreamPush(sctx, BAD_CAST "doc", NULL));
            h = av_b(h, (uint64_t)(int64_t)xmlStreamPushNode(sctx, BAD_CAST "item",
                                                             NULL, XML_ELEMENT_NODE));
            xmlFreeStreamCtxt(sctx);
        }
        xmlFreePattern(pat);
    }

    /* c14n save-to-file variant */
    const char *c14nf = "/tmp/vw_lx_c14n.xml";
    h = av_b(h, (uint64_t)(int64_t)xmlC14NDocSave(doc, NULL, XML_C14N_1_0,
                                                  NULL, 0, c14nf, 0));
    {
        size_t cn; uint8_t *cb = read_file(c14nf, &cn);
        h = digest64(h, cb, cn); free(cb);
        remove(c14nf);
    }

    /* parserInternals leftovers */
    xmlCheckVersion(LIBXML_VERSION);
    h = av_b(h, (uint64_t)xmlIsLetter(0x41));
    xmlKeepBlanksDefault(xmlKeepBlanksDefault(1));
    xmlLineNumbersDefault(xmlLineNumbersDefault(0));
    xmlPedanticParserDefault(xmlPedanticParserDefault(0));
    xmlSubstituteEntitiesDefault(xmlSubstituteEntitiesDefault(0));
    xmlParserCtxtPtr ic = xmlNewParserCtxt();
    if (ic) {
        xmlInitParserCtxt(ic);
        namePush(ic, BAD_CAST "pushed");
        h = av_s(h, namePop(ic));
        xmlParserNodeInfoSeq seq;
        xmlInitNodeInfoSeq(&seq);
        xmlClearNodeInfoSeq(&seq);
        xmlParserNodeInfo info;
        memset(&info, 0, sizeof(info));
        info.node = root;
        xmlParserAddNodeInfo(ic, &info);
        h = av_b(h, (uint64_t)(xmlParserFindNodeInfo(ic, root) != NULL));
        h = av_b(h, (uint64_t)xmlParserFindNodeInfoIndex(&ic->node_seq, root));
        xmlClearParserCtxt(ic);
        xmlFreeParserCtxt(ic);
    }
    fclose(dn);
    xmlFreeDoc(doc);
    return h;
}

/* coverage_only: variant entry points; input = a small standalone xml file */
static void run_api_variants(const char *path, long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        h = av_parser(h, path);
        h = av_htmlp(h, path);
        h = av_reader(h, path);
        h = av_writer(h);
        h = av_xpath(h);
        h = av_valid(h);
        h = av_tree(h);
        h = av_saveio(h);
        h = av_globals(h);
        h = av_sax2err(h);
        h = av_schemas(h);
        h = av_misc(h, path);
    }
    printf("op=api_variants in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "usage: %s <op> <input> <iters>\n", argv[0]);
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }
    xmlInitParser();

    if (strcmp(op, "parse_suite") == 0)        run_suite(op, argv[2], iters, parse_one);
    else if (strcmp(op, "html_suite") == 0)    run_suite(op, argv[2], iters, html_one);
    else if (strcmp(op, "sax_suite") == 0)     run_suite(op, argv[2], iters, sax_one);
    else if (strcmp(op, "reader_suite") == 0)  run_suite(op, argv[2], iters, reader_one);
    else if (strcmp(op, "valid_suite") == 0)   run_suite(op, argv[2], iters, valid_one);
    else if (strcmp(op, "schema_suite") == 0)  run_suite(op, argv[2], iters, schema_one);
    else if (strcmp(op, "relaxng_suite") == 0) run_suite(op, argv[2], iters, relaxng_one);
    else if (strcmp(op, "xpath_op") == 0)      run_xpath(argv[2], iters);
    else if (strcmp(op, "writer_op") == 0)     run_writer(iters);
    else if (strcmp(op, "dict_uri_str") == 0)  run_dict_uri_str(argv[2], iters);
    else if (strcmp(op, "c14n_op") == 0)       run_c14n(argv[2], iters);
    else if (strcmp(op, "unicode_tables") == 0) run_unicode_tables(iters);
    else if (strcmp(op, "reader_full") == 0)   run_suite(op, argv[2], iters, reader_full_one);
    else if (strcmp(op, "dom_build") == 0)     run_dom_build(iters);
    else if (strcmp(op, "xpath_full") == 0)    run_xpath_full(argv[2], iters);
    else if (strcmp(op, "writer_full") == 0)   run_writer_full(iters);
    else if (strcmp(op, "push_parse") == 0)    run_push_parse(argv[2], iters);
    else if (strcmp(op, "encoding_op") == 0)   run_encoding_op(argv[2], iters);
    else if (strcmp(op, "list_op") == 0)       run_list_op(iters);
    else if (strcmp(op, "error_op") == 0)      run_error_op(iters);
    else if (strcmp(op, "xinclude_suite") == 0) run_suite(op, argv[2], iters, xinclude_one);
    else if (strcmp(op, "xpointer_op") == 0)   run_xpointer_op(argv[2], iters);
    else if (strcmp(op, "schematron_suite") == 0) run_suite(op, argv[2], iters, schematron_one);
    else if (strcmp(op, "gz_suite") == 0)      run_suite(op, argv[2], iters, gz_one);
    else if (strcmp(op, "debug_op") == 0)      run_debug_op(argv[2], iters);
    else if (strcmp(op, "catalog_op") == 0)    run_catalog_op(argv[2], iters);
    else if (strcmp(op, "api_storm") == 0)     run_api_storm(argv[2], iters);
    else if (strcmp(op, "api_variants") == 0)  run_api_variants(argv[2], iters);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    xmlCleanupParser();
    return 0;
}
