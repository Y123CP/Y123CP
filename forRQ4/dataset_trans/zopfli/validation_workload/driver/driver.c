/*
 * validation driver for zopfli — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves.
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     compress_gzip    : ZopfliCompress, GZIP container, default options
 *     compress_zlib    : ZLIB container
 *     compress_deflate : raw DEFLATE container
 *     options_matrix   : numiterations {5,15} x blocksplitting {0,1}
 *                        (real tuning knobs), GZIP container
 *
 * zopfli is compress-only and ~100x slower than gzip: keep inputs moderate
 * (tens to hundreds of KB). Output: ONE digest line on stdout (project
 * convention, see the lz4 driver).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "zopfli.h"

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
    uint8_t *buf = malloc((size_t)n);
    if (!buf || fread(buf, 1, (size_t)n, f) != (size_t)n) {
        fprintf(stderr, "cannot read %s\n", path); exit(2);
    }
    fclose(f);
    *out_n = (size_t)n;
    return buf;
}

static void run_container(const char *op, ZopfliFormat fmt,
                          const uint8_t *src, size_t n, long iters) {
    ZopfliOptions opt;
    ZopfliInitOptions(&opt);
    uint64_t h = 0;
    size_t outsize = 0;
    for (long i = 0; i < iters; i++) {
        unsigned char *out = NULL;
        outsize = 0;
        ZopfliCompress(&opt, fmt, src, n, &out, &outsize);
        if (!out || !outsize) { fprintf(stderr, "compress failed\n"); exit(3); }
        h = digest64(h, out, outsize);
        free(out);
    }
    printf("op=%s in=%zu out=%zu iters=%ld digest=%016llx\n",
           op, n, outsize, iters, (unsigned long long)h);
}

static void run_options_matrix(const uint8_t *src, size_t n, long iters) {
    static const int its[]  = { 5, 15 };
    static const int bsp[]  = { 0, 1 };
    uint64_t h = 0, total = 0;
    for (long i = 0; i < iters; i++) {
        total = 0;
        for (size_t a = 0; a < 2; a++) {
            for (size_t b = 0; b < 2; b++) {
                ZopfliOptions opt;
                ZopfliInitOptions(&opt);
                opt.numiterations = its[a];
                opt.blocksplitting = bsp[b];
                unsigned char *out = NULL;
                size_t outsize = 0;
                ZopfliCompress(&opt, ZOPFLI_FORMAT_GZIP, src, n, &out, &outsize);
                if (!out || !outsize) { fprintf(stderr, "compress failed\n"); exit(3); }
                h = digest64(h, out, outsize);
                total += outsize;
                free(out);
            }
        }
    }
    printf("op=options_matrix in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)total, iters, (unsigned long long)h);
}

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "usage: %s <compress_gzip|compress_zlib|compress_deflate|options_matrix> <input> <iters>\n",
                argv[0]);
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }
    size_t n;
    uint8_t *buf = read_file(argv[2], &n);

    if (strcmp(op, "compress_gzip") == 0)         run_container(op, ZOPFLI_FORMAT_GZIP, buf, n, iters);
    else if (strcmp(op, "compress_zlib") == 0)    run_container(op, ZOPFLI_FORMAT_ZLIB, buf, n, iters);
    else if (strcmp(op, "compress_deflate") == 0) run_container(op, ZOPFLI_FORMAT_DEFLATE, buf, n, iters);
    else if (strcmp(op, "options_matrix") == 0)   run_options_matrix(buf, n, iters);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    free(buf);
    return 0;
}
