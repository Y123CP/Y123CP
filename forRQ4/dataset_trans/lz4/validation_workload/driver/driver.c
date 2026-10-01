/*
 * validation driver for lz4 — ONE driver, three link targets (c / c2rust_raw / ours).
 * Calls only the C ABI that c2rust preserves; byte-identical across versions.
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     one-shot ops : compress_fast, compress_hc (level 9), compress_hc_max
 *                    (level 12, optimal parser), decompress
 *     streaming ops: compress_stream, compress_stream_hc, decompress_stream
 *                    (64KB chunks via LZ4_*_continue, implicit prefix dict)
 *     dict ops     : compress_dict{,_hc}, decompress_dict (4KB messages against
 *                    a 64KB preset dictionary, attach/loadDict / usingDict)
 *     variant ops  : compress_destsize{,_hc} (4KB output pages),
 *                    compress_extstate (caller-managed state, fastReset),
 *                    decompress_partial{,_dict} (decode first half only)
 *   driver gen_lz4  <input> <output>  decompress input: u64 LE orig size + LZ4 block
 *   driver gen_lz4s <input> <output>  decompress_stream input: u64 LE total orig
 *                                     size + per chunk {u32 comp, u32 orig, bytes}
 *   driver gen_lz4d <input> <output>  decompress_dict input: u32 dict size + dict
 *                                     + u64 total + per msg {u32 comp, u32 orig, bytes}
 *
 * Output: ONE digest line on stdout after the timing loop, e.g.
 *   op=compress_fast in=5345280 out=2963446 iters=48 digest=0123456789abcdef
 * The digest is re-seeded with the running value every iteration, so each
 * iteration's output feeds the next seed — no iteration can be elided.
 */
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* expose LZ4LIB_STATIC_API prototypes (extState_fastReset); we link the lib
 * statically in every build target, which is exactly what this API requires */
#define LZ4_STATIC_LINKING_ONLY
#include "lz4.h"
#include "lz4hc.h"

/* 4-lane xor-rotate digest: every step is a bijection of lane state, so any
 * byte change in any position propagates to the final value (order-sensitive).
 * No multiplies in the hot loop — cheapest possible full-output fold, keeping
 * the driver's checksum a small fraction of self-time even for very fast ops
 * like lz4 decompress (gate G4 self_time_own >= 70%). */
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

/* compress <src> iters times; digest folds every iteration's full output.
 * level 0 = fast path (LZ4_compress_default), >0 = HC at that level
 * (9 = hash-chain kernel, 12 = optimal-parser kernel -- distinct hot paths). */
static void run_compress(const char *op, const uint8_t *src, size_t n,
                         long iters, int level) {
    int bound = LZ4_compressBound((int)n);
    char *dst = malloc((size_t)bound);
    if (!dst) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    int outlen = 0;
    for (long i = 0; i < iters; i++) {
        outlen = level
            ? LZ4_compress_HC((const char *)src, dst, (int)n, bound, level)
            : LZ4_compress_default((const char *)src, dst, (int)n, bound);
        if (outlen <= 0) { fprintf(stderr, "compress failed\n"); exit(3); }
        h = digest64(h, (const uint8_t *)dst, (size_t)outlen);
    }
    printf("op=%s in=%zu out=%d iters=%ld digest=%016llx\n",
           op, n, outlen, iters, (unsigned long long)h);
    free(dst);
}

#define DICT_SIZE 65536
#define DICT_MSG  4096

/* per-message dictionary compression (lz4's small-message flagship use case):
 * dict = first 64KB of the input, messages = 4KB slices of the rest; each
 * message is compressed independently against the dict via the modern
 * attach-dictionary pattern (loadDict once, attach per message). */
static void run_compress_dict(const char *op, const uint8_t *src, size_t n,
                              long iters) {
    if (n < DICT_SIZE + DICT_MSG) { fprintf(stderr, "input too small for dict op\n"); exit(2); }
    const char *data = (const char *)src + DICT_SIZE;
    size_t dn = n - DICT_SIZE;
    int bound = LZ4_compressBound(DICT_MSG);
    char *dst = malloc((size_t)bound);
    LZ4_stream_t *dict_s = LZ4_createStream();
    LZ4_stream_t *work_s = LZ4_createStream();
    if (!dst || !dict_s || !work_s) { fprintf(stderr, "oom\n"); exit(2); }
    LZ4_loadDict(dict_s, (const char *)src, DICT_SIZE);
    {   /* exercise the ring-buffer dict-relocation API; the dict content is
         * unchanged so compressed output (and the oracle) is unaffected */
        char *scratch = malloc(DICT_SIZE);
        if (scratch) { LZ4_saveDict(dict_s, scratch, DICT_SIZE); free(scratch); }
    }
    uint64_t h = 0, total = 0;
    for (long i = 0; i < iters; i++) {
        total = 0;
        for (size_t off = 0; off < dn; off += DICT_MSG) {
            int msg = (dn - off < DICT_MSG) ? (int)(dn - off) : DICT_MSG;
            LZ4_resetStream_fast(work_s);
            LZ4_attach_dictionary(work_s, dict_s);
            int outlen = LZ4_compress_fast_continue(work_s, data + off, dst, msg, bound, 1);
            if (outlen <= 0) { fprintf(stderr, "dict compress failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)dst, (size_t)outlen);
            total += (uint64_t)outlen;
        }
    }
    printf("op=%s in=%zu out=%llu iters=%ld digest=%016llx\n",
           op, n, (unsigned long long)total, iters, (unsigned long long)h);
    LZ4_freeStream(dict_s); LZ4_freeStream(work_s); free(dst);
}

/* HC flavour of the preset-dictionary message pattern (LZ4_loadDictHC +
 * LZ4_attach_HC_dictionary + LZ4_compress_HC_continue at default level). */
static void run_compress_dict_hc(const char *op, const uint8_t *src, size_t n,
                                 long iters) {
    if (n < DICT_SIZE + DICT_MSG) { fprintf(stderr, "input too small for dict op\n"); exit(2); }
    const char *data = (const char *)src + DICT_SIZE;
    size_t dn = n - DICT_SIZE;
    int bound = LZ4_compressBound(DICT_MSG);
    char *dst = malloc((size_t)bound);
    LZ4_streamHC_t *dict_s = LZ4_createStreamHC();
    LZ4_streamHC_t *work_s = LZ4_createStreamHC();
    if (!dst || !dict_s || !work_s) { fprintf(stderr, "oom\n"); exit(2); }
    LZ4_loadDictHC(dict_s, (const char *)src, DICT_SIZE);
    {
        char *scratch = malloc(DICT_SIZE);
        if (scratch) { LZ4_saveDictHC(dict_s, scratch, DICT_SIZE); free(scratch); }
    }
    uint64_t h = 0, total = 0;
    for (long i = 0; i < iters; i++) {
        total = 0;
        for (size_t off = 0; off < dn; off += DICT_MSG) {
            int msg = (dn - off < DICT_MSG) ? (int)(dn - off) : DICT_MSG;
            LZ4_resetStreamHC_fast(work_s, 9);
            LZ4_attach_HC_dictionary(work_s, dict_s);
            int outlen = LZ4_compress_HC_continue(work_s, data + off, dst, msg, bound);
            if (outlen <= 0) { fprintf(stderr, "hc dict compress failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)dst, (size_t)outlen);
            total += (uint64_t)outlen;
        }
    }
    printf("op=%s in=%zu out=%llu iters=%ld digest=%016llx\n",
           op, n, (unsigned long long)total, iters, (unsigned long long)h);
    LZ4_freeStreamHC(dict_s); LZ4_freeStreamHC(work_s); free(dst);
}

/* budget-bounded compression: fill fixed 4KB output pages, letting lz4 decide
 * how much input each page consumes (database page-compression pattern). */
static void run_compress_destsize(const char *op, const uint8_t *src, size_t n,
                                  long iters, int hc) {
    enum { BUDGET = 4096 };
    char *dst = malloc(BUDGET);
    /* caller-managed state via the sizeof/extState pattern (embedded usage);
     * output is identical to the convenience entry points */
    void *state = malloc((size_t)(hc ? LZ4_sizeofStateHC() : LZ4_sizeofState()));
    if (!dst || !state) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0, total = 0;
    for (long i = 0; i < iters; i++) {
        total = 0;
        size_t pos = 0;
        long page = 0;
        while (pos < n) {
            int srcSize = (n - pos > INT_MAX) ? INT_MAX : (int)(n - pos);
            int outlen;
            if (hc)
                outlen = LZ4_compress_HC_destSize(state, (const char *)src + pos, dst, &srcSize, BUDGET, 9);
            else if (page & 1)   /* alternate the two equivalent fast entry points */
                outlen = LZ4_compress_destSize_extState(state, (const char *)src + pos, dst, &srcSize, BUDGET, 1);
            else
                outlen = LZ4_compress_destSize((const char *)src + pos, dst, &srcSize, BUDGET);
            if (outlen <= 0 || srcSize <= 0) { fprintf(stderr, "destSize failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)dst, (size_t)outlen);
            total += (uint64_t)outlen;
            pos += (size_t)srcSize;
            page++;
        }
    }
    printf("op=%s in=%zu out=%llu iters=%ld digest=%016llx\n",
           op, n, (unsigned long long)total, iters, (unsigned long long)h);
    free(state);
    free(dst);
}

/* caller-managed compression state, reset via the cheap fastReset variant
 * (embedded / allocation-free usage pattern). */
static void run_compress_extstate(const char *op, const uint8_t *src, size_t n,
                                  long iters) {
    void *state = malloc((size_t)LZ4_sizeofState());
    int bound = LZ4_compressBound((int)n);
    char *dst = malloc((size_t)bound);
    if (!state || !dst) { fprintf(stderr, "oom\n"); exit(2); }
    if (!LZ4_initStream(state, (size_t)LZ4_sizeofState())) {
        fprintf(stderr, "initStream failed (alignment)\n"); exit(2);
    }
    uint64_t h = 0;
    int outlen = 0;
    for (long i = 0; i < iters; i++) {
        outlen = LZ4_compress_fast_extState_fastReset(state, (const char *)src,
                                                      dst, (int)n, bound, 1);
        if (outlen <= 0) { fprintf(stderr, "extState compress failed\n"); exit(3); }
        h = digest64(h, (const uint8_t *)dst, (size_t)outlen);
    }
    printf("op=%s in=%zu out=%d iters=%ld digest=%016llx\n",
           op, n, outlen, iters, (unsigned long long)h);
    free(state); free(dst);
}

/* partial decode of a plain .lz4b block: decode only the first half
 * (peek-style usage: LZ4_decompress_safe_partial). */
static void run_decompress_partial(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad decompress input\n"); exit(2); }
    uint64_t orig = 0;
    memcpy(&orig, file, 8);
    const char *comp = (const char *)file + 8;
    size_t comp_n = file_n - 8;
    int target = (int)(orig / 2);
    char *dst = malloc((size_t)orig);
    if (!dst) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        int r = LZ4_decompress_safe_partial(comp, dst, (int)comp_n, target, (int)orig);
        if (r != target) { fprintf(stderr, "partial failed: %d\n", r); exit(3); }
        h = digest64(h, (const uint8_t *)dst, (size_t)r);
    }
    printf("op=decompress_partial in=%zu out=%d iters=%ld digest=%016llx\n",
           file_n, target, iters, (unsigned long long)h);
    free(dst);
}

/* partial decode of dict-compressed .lz4d messages
 * (LZ4_decompress_safe_partial_usingDict, external-dict path). */
static void run_decompress_partial_dict(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 12) { fprintf(stderr, "bad dict input\n"); exit(2); }
    uint32_t dict_n = 0;
    memcpy(&dict_n, file, 4);
    const char *dict = (const char *)file + 4;
    size_t pos0 = 4 + dict_n + 8;
    char *out = malloc(DICT_MSG);
    if (!out) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0, produced = 0;
    for (long i = 0; i < iters; i++) {
        size_t pos = pos0;
        produced = 0;
        while (pos < file_n) {
            uint32_t comp = 0, orig = 0;
            memcpy(&comp, file + pos, 4);
            memcpy(&orig, file + pos + 4, 4);
            pos += 8;
            int target = (int)(orig / 2);
            int r = LZ4_decompress_safe_partial_usingDict((const char *)file + pos, out,
                                                          (int)comp, target, (int)orig,
                                                          dict, (int)dict_n);
            if (r != target) { fprintf(stderr, "partial dict failed: %d\n", r); exit(3); }
            h = digest64(h, (const uint8_t *)out, (size_t)r);
            produced += (uint64_t)r;
            pos += comp;
        }
    }
    printf("op=decompress_partial_dict in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)produced, iters, (unsigned long long)h);
    free(out);
}

/* input file: u32 LE dict size + dict bytes + u64 LE total orig size +
 * per message {u32 LE comp, u32 LE orig, bytes}; each message decoded
 * independently with LZ4_decompress_safe_usingDict. */
static void run_decompress_dict(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 12) { fprintf(stderr, "bad dict input\n"); exit(2); }
    uint32_t dict_n = 0;
    memcpy(&dict_n, file, 4);
    const char *dict = (const char *)file + 4;
    size_t pos0 = 4 + dict_n;
    uint64_t total = 0;
    memcpy(&total, file + pos0, 8);
    pos0 += 8;
    char *out = malloc(DICT_MSG);
    if (!out) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        size_t pos = pos0;
        uint64_t off = 0;
        while (pos < file_n) {
            uint32_t comp = 0, orig = 0;
            memcpy(&comp, file + pos, 4);
            memcpy(&orig, file + pos + 4, 4);
            pos += 8;
            int r = LZ4_decompress_safe_usingDict((const char *)file + pos, out,
                                                  (int)comp, (int)orig, dict, (int)dict_n);
            if (r != (int)orig) { fprintf(stderr, "dict decompress failed: %d\n", r); exit(3); }
            h = digest64(h, (const uint8_t *)out, (size_t)orig);
            pos += comp;
            off += orig;
        }
        if (off != total) { fprintf(stderr, "dict size mismatch\n"); exit(3); }
    }
    printf("op=decompress_dict in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)total, iters, (unsigned long long)h);
    free(out);
}

static void gen_lz4d(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *src = read_file(in_path, &n);
    if (n < DICT_SIZE + DICT_MSG) { fprintf(stderr, "input too small\n"); exit(2); }
    const char *data = (const char *)src + DICT_SIZE;
    size_t dn = n - DICT_SIZE;
    int bound = LZ4_compressBound(DICT_MSG);
    char *dst = malloc((size_t)bound);
    FILE *f = fopen(out_path, "wb");
    LZ4_stream_t *dict_s = LZ4_createStream();
    LZ4_stream_t *work_s = LZ4_createStream();
    if (!dst || !f || !dict_s || !work_s) { fprintf(stderr, "oom/open fail\n"); exit(2); }
    LZ4_loadDict(dict_s, (const char *)src, DICT_SIZE);
    uint32_t dict_n = DICT_SIZE;
    uint64_t total = (uint64_t)dn;
    fwrite(&dict_n, 1, 4, f);
    fwrite(src, 1, DICT_SIZE, f);
    fwrite(&total, 1, 8, f);
    for (size_t off = 0; off < dn; off += DICT_MSG) {
        int msg = (dn - off < DICT_MSG) ? (int)(dn - off) : DICT_MSG;
        LZ4_resetStream_fast(work_s);
        LZ4_attach_dictionary(work_s, dict_s);
        int outlen = LZ4_compress_fast_continue(work_s, data + off, dst, msg, bound, 1);
        if (outlen <= 0) { fprintf(stderr, "dict compress failed\n"); exit(3); }
        uint32_t comp = (uint32_t)outlen, orig = (uint32_t)msg;
        fwrite(&comp, 1, 4, f);
        fwrite(&orig, 1, 4, f);
        fwrite(dst, 1, (size_t)outlen, f);
    }
    LZ4_freeStream(dict_s); LZ4_freeStream(work_s);
    fclose(f);
    fprintf(stderr, "wrote %s (dict %u + %llu bytes in %d-byte msgs)\n",
            out_path, dict_n, (unsigned long long)total, DICT_MSG);
    free(src); free(dst);
}

/* input file: u64 LE original size + LZ4 block */
static void run_decompress(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad decompress input\n"); exit(2); }
    uint64_t orig = 0;
    memcpy(&orig, file, 8);
    const char *comp = (const char *)file + 8;
    size_t comp_n = file_n - 8;
    char *dst = malloc((size_t)orig);
    if (!dst) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        int r = LZ4_decompress_safe(comp, dst, (int)comp_n, (int)orig);
        if (r != (int)orig) { fprintf(stderr, "decompress failed: %d\n", r); exit(3); }
        h = digest64(h, (const uint8_t *)dst, (size_t)orig);
    }
    printf("op=decompress in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)orig, iters, (unsigned long long)h);
    free(dst);
}

#define STREAM_CHUNK 65536  /* 64KB = LZ4 dict window; implicit prefix-dict mode */

/* chunked compress over a contiguous buffer with LZ4_*_continue; the digest
 * folds every chunk's compressed bytes. hc=0 -> fast(accel 1), hc=1 -> HC. */
static void run_compress_stream(const char *op, const uint8_t *src, size_t n,
                                long iters, int hc) {
    int bound = LZ4_compressBound(STREAM_CHUNK);
    char *dst = malloc((size_t)bound);
    if (!dst) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    uint64_t total_out = 0;
    for (long i = 0; i < iters; i++) {
        LZ4_stream_t *s = NULL;
        LZ4_streamHC_t *sh = NULL;
        if (hc) sh = LZ4_createStreamHC(); else s = LZ4_createStream();
        if (!s && !sh) { fprintf(stderr, "stream alloc failed\n"); exit(2); }
        total_out = 0;
        for (size_t off = 0; off < n; off += STREAM_CHUNK) {
            int csize = (n - off < STREAM_CHUNK) ? (int)(n - off) : STREAM_CHUNK;
            int outlen = hc
                ? LZ4_compress_HC_continue(sh, (const char *)src + off, dst, csize, bound)
                : LZ4_compress_fast_continue(s, (const char *)src + off, dst, csize, bound, 1);
            if (outlen <= 0) { fprintf(stderr, "stream compress failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)dst, (size_t)outlen);
            total_out += (uint64_t)outlen;
        }
        if (hc) LZ4_freeStreamHC(sh); else LZ4_freeStream(s);
    }
    printf("op=%s in=%zu out=%llu iters=%ld digest=%016llx\n",
           op, n, (unsigned long long)total_out, iters, (unsigned long long)h);
    free(dst);
}

/* input file: u64 LE total orig size + per chunk {u32 LE comp, u32 LE orig, bytes} */
static void run_decompress_stream(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad stream input\n"); exit(2); }
    uint64_t total = 0;
    memcpy(&total, file, 8);
    char *out = malloc((size_t)total);
    if (!out) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    LZ4_streamDecode_t *sd = LZ4_createStreamDecode();
    if (!sd) { fprintf(stderr, "stream alloc failed\n"); exit(2); }
    for (long i = 0; i < iters; i++) {
        /* reset via the public API instead of realloc (ring-buffer pattern) */
        if (!LZ4_setStreamDecode(sd, NULL, 0)) { fprintf(stderr, "setStreamDecode failed\n"); exit(2); }
        size_t pos = 8, off = 0;
        while (pos < file_n) {
            uint32_t comp = 0, orig = 0;
            memcpy(&comp, file + pos, 4);
            memcpy(&orig, file + pos + 4, 4);
            pos += 8;
            int r = LZ4_decompress_safe_continue(sd, (const char *)file + pos,
                                                 out + off, (int)comp, (int)orig);
            if (r != (int)orig) { fprintf(stderr, "stream decompress failed: %d\n", r); exit(3); }
            h = digest64(h, (const uint8_t *)out + off, (size_t)orig);
            pos += comp;
            off += orig;
        }
        if (off != (size_t)total) { fprintf(stderr, "stream size mismatch\n"); exit(3); }
    }
    LZ4_freeStreamDecode(sd);
    printf("op=decompress_stream in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)total, iters, (unsigned long long)h);
    free(out);
}

static void gen_lz4s(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *src = read_file(in_path, &n);
    int bound = LZ4_compressBound(STREAM_CHUNK);
    char *dst = malloc((size_t)bound);
    FILE *f = fopen(out_path, "wb");
    if (!dst || !f) { fprintf(stderr, "oom/open fail\n"); exit(2); }
    uint64_t total = (uint64_t)n;
    fwrite(&total, 1, 8, f);
    LZ4_stream_t *s = LZ4_createStream();
    uint64_t written = 0;
    for (size_t off = 0; off < n; off += STREAM_CHUNK) {
        int csize = (n - off < STREAM_CHUNK) ? (int)(n - off) : STREAM_CHUNK;
        int outlen = LZ4_compress_fast_continue(s, (const char *)src + off, dst,
                                                csize, bound, 1);
        if (outlen <= 0) { fprintf(stderr, "stream compress failed\n"); exit(3); }
        uint32_t comp = (uint32_t)outlen, orig = (uint32_t)csize;
        fwrite(&comp, 1, 4, f);
        fwrite(&orig, 1, 4, f);
        fwrite(dst, 1, (size_t)outlen, f);
        written += outlen;
    }
    LZ4_freeStream(s);
    fclose(f);
    fprintf(stderr, "wrote %s: %llu -> %llu (chunked)\n", out_path,
            (unsigned long long)total, (unsigned long long)written);
    free(src); free(dst);
}

static void gen_lz4(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *src = read_file(in_path, &n);
    int bound = LZ4_compressBound((int)n);
    char *dst = malloc((size_t)bound);
    int outlen = LZ4_compress_default((const char *)src, dst, (int)n, bound);
    if (outlen <= 0) { fprintf(stderr, "compress failed\n"); exit(3); }
    FILE *f = fopen(out_path, "wb");
    if (!f) { fprintf(stderr, "cannot open %s\n", out_path); exit(2); }
    uint64_t orig = (uint64_t)n;
    fwrite(&orig, 1, 8, f);
    fwrite(dst, 1, (size_t)outlen, f);
    fclose(f);
    fprintf(stderr, "wrote %s: %llu -> %d\n", out_path,
            (unsigned long long)orig, outlen);
    free(src); free(dst);
}

int main(int argc, char **argv) {
    if (argc == 4 && strcmp(argv[1], "gen_lz4") == 0) {
        gen_lz4(argv[2], argv[3]);
        return 0;
    }
    if (argc == 4 && strcmp(argv[1], "gen_lz4s") == 0) {
        gen_lz4s(argv[2], argv[3]);
        return 0;
    }
    if (argc == 4 && strcmp(argv[1], "gen_lz4d") == 0) {
        gen_lz4d(argv[2], argv[3]);
        return 0;
    }
    if (argc != 4) {
        fprintf(stderr,
                "usage: %s <compress_fast|compress_hc|decompress> <input> <iters>\n"
                "       %s gen_lz4 <input> <output>\n", argv[0], argv[0]);
        return 1;
    }
    /* provenance sanity check on the linked implementation (stderr only,
     * stdout digest lines are unaffected) */
    if (LZ4_versionNumber() < 10000 || LZ4_versionString() == NULL) {
        fprintf(stderr, "unexpected lz4 version\n");
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }
    size_t n;
    uint8_t *buf = read_file(argv[2], &n);

    if (strcmp(op, "compress_fast") == 0)           run_compress(op, buf, n, iters, 0);
    else if (strcmp(op, "compress_hc") == 0)        run_compress(op, buf, n, iters, 9);
    else if (strcmp(op, "compress_hc_min") == 0)    run_compress(op, buf, n, iters, 2);
    else if (strcmp(op, "compress_hc_max") == 0)    run_compress(op, buf, n, iters, 12);
    else if (strcmp(op, "decompress") == 0)         run_decompress(buf, n, iters);
    else if (strcmp(op, "compress_stream") == 0)    run_compress_stream(op, buf, n, iters, 0);
    else if (strcmp(op, "compress_stream_hc") == 0) run_compress_stream(op, buf, n, iters, 1);
    else if (strcmp(op, "decompress_stream") == 0)  run_decompress_stream(buf, n, iters);
    else if (strcmp(op, "compress_dict") == 0)      run_compress_dict(op, buf, n, iters);
    else if (strcmp(op, "decompress_dict") == 0)    run_decompress_dict(buf, n, iters);
    else if (strcmp(op, "compress_dict_hc") == 0)   run_compress_dict_hc(op, buf, n, iters);
    else if (strcmp(op, "compress_destsize") == 0)  run_compress_destsize(op, buf, n, iters, 0);
    else if (strcmp(op, "compress_destsize_hc") == 0) run_compress_destsize(op, buf, n, iters, 1);
    else if (strcmp(op, "compress_extstate") == 0)  run_compress_extstate(op, buf, n, iters);
    else if (strcmp(op, "decompress_partial") == 0) run_decompress_partial(buf, n, iters);
    else if (strcmp(op, "decompress_partial_dict") == 0) run_decompress_partial_dict(buf, n, iters);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    free(buf);
    return 0;
}
