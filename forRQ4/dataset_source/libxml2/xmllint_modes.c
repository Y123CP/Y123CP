/* xmllint_modes — C reference for the Rust harness at
 *   dataset_trans_process/libxml2/workloads/harness/xmllint_modes/src/main.rs
 *
 * Mirrors four libxml2 modes (parse / xpath count / xpath strlen /
 * serialize) and emits a single line:
 *   xmllint_modes_sha256=<16-hex>\n
 *
 * Fingerprint = FNV-1a 64-bit over four (tag, bytes) chunks:
 *   ("parse_size",     metadata.len() as u64 in LE bytes)
 *   ("xpath_count",    "<kind>/<val:.6f>")
 *   ("xpath_strlen",   "<kind>/<val:.6f>")
 *   ("serialize_bytes", xmlDocDumpMemory bytes)
 * Must match the Rust harness output BYTE-EXACT — including the {:.6}
 * float formatting and the LE u64 of file size.
 *
 *   Usage: xmllint_modes <input.xml>
 *   Build: gcc -O2 -I include -o xmllint_modes xmllint_modes.c \
 *              _build/libxml2.so -Wl,-rpath,_build
 */
#include <libxml/parser.h>
#include <libxml/tree.h>
#include <libxml/xpath.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <stdint.h>

#define FNV_OFFSET ((uint64_t)0xcbf29ce484222325ULL)
#define FNV_PRIME  ((uint64_t)0x100000001b3ULL)

static void fnv_update(uint64_t *h, const unsigned char *p, size_t n) {
    for (size_t i = 0; i < n; i++) {
        *h ^= (uint64_t)p[i];
        *h *= FNV_PRIME;
    }
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <input.xml>\n", argv[0]);
        return 1;
    }
    struct stat st;
    if (stat(argv[1], &st) != 0) {
        fprintf(stderr, "stat %s failed\n", argv[1]);
        return 1;
    }
    uint64_t file_size_le = (uint64_t)st.st_size;  /* host is x86_64 LE */

    /* Mode 1: parse */
    xmlDocPtr doc = xmlReadFile(argv[1], NULL, 0);
    if (!doc) { fprintf(stderr, "xmlReadFile returned NULL\n"); return 2; }

    /* Mode 2: xpath count(//*) */
    xmlXPathContextPtr ctx = xmlXPathNewContext(doc);
    xmlXPathObjectPtr r1 = xmlXPathEvalExpression((const xmlChar *)"count(//*)", ctx);
    int    r1_kind = r1 ? (int)r1->type : -1;
    double r1_val  = r1 ? r1->floatval  : -1.0;
    if (r1) xmlXPathFreeObject(r1);

    /* Mode 4: xpath string-length(string(//.)) */
    xmlXPathObjectPtr r2 = xmlXPathEvalExpression((const xmlChar *)"string-length(string(//.))", ctx);
    int    r2_kind = r2 ? (int)r2->type : -1;
    double r2_val  = r2 ? r2->floatval  : -1.0;
    if (r2) xmlXPathFreeObject(r2);
    xmlXPathFreeContext(ctx);

    /* Mode 3: serialize */
    xmlChar *mem = NULL;
    int size = 0;
    xmlDocDumpMemory(doc, &mem, &size);
    if (!mem || size <= 0) { mem = NULL; size = 0; }

    xmlFreeDoc(doc);

    /* Format the two count/strlen lines exactly like Rust's
     *   format!("{}/{:.6}", kind, val)
     * — printf %.6f for f64 matches Rust's {:.6} for f64 on x86_64. */
    char line_count[64], line_strlen[64];
    int lc = snprintf(line_count,  sizeof line_count,  "%d/%.6f", r1_kind, r1_val);
    int ls = snprintf(line_strlen, sizeof line_strlen, "%d/%.6f", r2_kind, r2_val);

    /* FNV-1a aggregation: tag bytes, then payload bytes, for each chunk. */
    uint64_t h = FNV_OFFSET;
    fnv_update(&h, (const unsigned char *)"parse_size", strlen("parse_size"));
    fnv_update(&h, (const unsigned char *)&file_size_le, sizeof file_size_le);

    fnv_update(&h, (const unsigned char *)"xpath_count", strlen("xpath_count"));
    fnv_update(&h, (const unsigned char *)line_count, (size_t)lc);

    fnv_update(&h, (const unsigned char *)"xpath_strlen", strlen("xpath_strlen"));
    fnv_update(&h, (const unsigned char *)line_strlen, (size_t)ls);

    fnv_update(&h, (const unsigned char *)"serialize_bytes", strlen("serialize_bytes"));
    if (mem && size > 0) fnv_update(&h, mem, (size_t)size);

    printf("xmllint_modes_sha256=%016lx\n", (unsigned long)h);

    /* libxml2 allocates `mem` via xmlMalloc; harness leaks like the Rust
     * version (runs once + exits). */
    return 0;
}
