/* libcsv harness — counts rows and fields in a CSV file using libcsv. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "csv.h"

static unsigned long g_fields = 0;
static unsigned long g_rows = 0;

static void cb_field(void *s, size_t len, void *data) {
    (void)s; (void)len; (void)data;
    g_fields++;
}
static void cb_row(int c, void *data) {
    (void)c; (void)data;
    g_rows++;
}

/* OPERATION 3 — COV: exercise the remaining public API (opts/delim/quote getters+
 * setters, blk_size, buffer_size, space/term/realloc/free func setters, error,
 * strerror, fwrite/fwrite2). Deterministic; C and Rust MUST print identical stdout. */
static int cov_space(unsigned char c) { return c == ' ' || c == '\t'; }
static int cov_term(unsigned char c) { return c == '\n'; }

static void cov_workload(void) {
    struct csv_parser p;
    csv_init(&p, 0);
    csv_set_opts(&p, CSV_STRICT | CSV_APPEND_NULL);
    int opts = csv_get_opts(&p);
    csv_set_delim(&p, ';');
    unsigned char d = csv_get_delim(&p);
    csv_set_quote(&p, '\'');
    unsigned char q = csv_get_quote(&p);
    csv_set_blk_size(&p, 2048);
    size_t bufsz = csv_get_buffer_size(&p);
    csv_set_space_func(&p, cov_space);
    csv_set_term_func(&p, cov_term);
    csv_set_realloc_func(&p, realloc);
    csv_set_free_func(&p, free);
    const char *data = "a;b;'c;d'\nx;y;z\n";
    csv_parse(&p, data, strlen(data), NULL, NULL, NULL);
    int err = csv_error(&p);
    const char *es = csv_strerror(err);
    csv_fini(&p, NULL, NULL, NULL);
    csv_free(&p);
    printf("opts=%d delim=%u quote=%u bufsz=%zu err=%d es=%s\n",
           opts, d, q, bufsz, err, es);
}

/* OPERATION 2 — WRITE: build K fixed fields once (each forced to contain a comma
 * and a quote so csv_write must quote + double-escape), loop N_W times escaping
 * every field via csv_write, folding each result's (len, first, last) byte.
 * Self-contained (argv[1]=="write"); C and Rust MUST print identical stdout. */
#define CSV_K    10000
#define CSV_NW   300
#define CSV_FLEN 40
#define CSV_DEST 256

static void write_workload(void) {
    unsigned char *fields = (unsigned char *)malloc((size_t)CSV_K * CSV_FLEN);
    for (int i = 0; i < CSV_K; i++) {
        for (int j = 0; j < CSV_FLEN; j++)
            fields[i * CSV_FLEN + j] = (unsigned char)(i * 31 + j * 7);
        fields[i * CSV_FLEN + 5] = ',';
        fields[i * CSV_FLEN + 10] = '"';
    }
    unsigned char dest[CSV_DEST];
    unsigned long checksum = 0, total = 0;
    for (int it = 0; it < CSV_NW; it++) {
        for (int i = 0; i < CSV_K; i++) {
            size_t sz = csv_write(dest, CSV_DEST, fields + i * CSV_FLEN, CSV_FLEN);
            unsigned char last = sz > 0 ? dest[sz - 1] : 0;
            checksum = ((checksum << 1) | (checksum >> 63))
                       ^ (unsigned long)sz ^ (unsigned long)dest[0] ^ (unsigned long)last;
            total += sz;
        }
    }
    printf("written=%lu checksum=%016lx\n", total, checksum);
    free(fields);
}

int main(int argc, char **argv) {
    if (argc == 2 && strcmp(argv[1], "write") == 0) {
        write_workload();
        return 0;
    }
    if (argc == 2 && strcmp(argv[1], "cov") == 0) {
        cov_workload();
        return 0;
    }
    if (argc != 2) {
        fprintf(stderr, "usage: %s <file.csv | write>\n", argv[0]);
        return 1;
    }
    FILE *fp = fopen(argv[1], "rb");
    if (!fp) { perror(argv[1]); return 1; }

    struct csv_parser p;
    if (csv_init(&p, 0) != 0) {
        fprintf(stderr, "csv_init failed\n");
        return 1;
    }

    char buf[65536];
    size_t n;
    while ((n = fread(buf, 1, sizeof(buf), fp)) > 0) {
        if (csv_parse(&p, buf, n, cb_field, cb_row, NULL) != n) {
            fprintf(stderr, "parse error: %s\n", csv_strerror(csv_error(&p)));
            csv_free(&p);
            fclose(fp);
            return 1;
        }
    }
    csv_fini(&p, cb_field, cb_row, NULL);
    csv_free(&p);
    fclose(fp);

    printf("rows=%lu fields=%lu\n", g_rows, g_fields);
    return 0;
}
