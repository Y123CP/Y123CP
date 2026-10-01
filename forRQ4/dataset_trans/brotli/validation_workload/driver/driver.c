/*
 * validation driver for brotli — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves.
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     compress        : one-shot BrotliEncoderCompress at q{1,5,9} (covers the
 *                       fragment / standard backward-reference kernels)
 *     compress_hq     : q11 (HQ backward references + optimal parse), lgwin 24
 *     decompress      : one-shot BrotliDecoderDecompress of a .br file
 *     stream_roundtrip: chunked CompressStream + DecompressStream with
 *                       TakeOutput/HasMoreOutput/IsFinished/IsUsed
 *     dict_roundtrip  : shared-dictionary encode + decode (Prepare/Attach)
 *     misc            : versions, MaxCompressedSize, EstimatePeakMemoryUsage,
 *                       decoder error path (GetErrorCode/ErrorString)
 *   driver gen_br   <input> <out.br>    q9 one-shot (decompress input)
 *
 * Output: ONE digest line on stdout (project convention).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <brotli/decode.h>
#include <brotli/encode.h>
#include <brotli/shared_dictionary.h>

/* project-local static-table initializers (c2rust de-globalization shim,
 * present in both the C tree and the translation) */
extern void BrotliEncoderEnsureStaticInit(void);
extern void BrotliDecoderEnsureStaticInit(void);

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

static void run_compress(const uint8_t *src, size_t n, long iters) {
    static const int qs[5] = { 0, 1, 3, 5, 9 };
    size_t cap = BrotliEncoderMaxCompressedSize(n);
    uint8_t *out = malloc(cap);
    if (!out) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0, total = 0;
    for (long i = 0; i < iters; i++) {
        total = 0;
        for (int q = 0; q < 5; q++) {
            size_t outlen = cap;
            if (!BrotliEncoderCompress(qs[q], 22, BROTLI_MODE_GENERIC,
                                       n, src, &outlen, out)) {
                fprintf(stderr, "compress q%d failed\n", qs[q]); exit(3);
            }
            h = digest64(h, out, outlen);
            total += outlen;
        }
    }
    printf("op=compress in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)total, iters, (unsigned long long)h);
    free(out);
}

static void run_compress_hq(const uint8_t *src, size_t n, long iters) {
    size_t cap = BrotliEncoderMaxCompressedSize(n);
    uint8_t *out = malloc(cap);
    if (!out) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    size_t outlen = 0;
    for (long i = 0; i < iters; i++) {
        for (int q = 10; q <= 11; q++) {   /* zopfli + hq-zopfli parsers */
            outlen = cap;
            if (!BrotliEncoderCompress(q, 24, BROTLI_MODE_TEXT, n, src, &outlen, out)) {
                fprintf(stderr, "compress q%d failed\n", q); exit(3);
            }
            h = digest64(h, out, outlen);
        }
    }
    printf("op=compress_hq in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, outlen, iters, (unsigned long long)h);
    free(out);
}

/* .br file: u64 LE orig size + brotli stream */
static void run_decompress(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad .br input\n"); exit(2); }
    uint64_t orig = 0;
    memcpy(&orig, file, 8);
    uint8_t *out = malloc((size_t)orig);
    if (!out) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        size_t outlen = (size_t)orig;
        if (BrotliDecoderDecompress(file_n - 8, file + 8, &outlen, out)
                != BROTLI_DECODER_RESULT_SUCCESS || outlen != orig) {
            fprintf(stderr, "decompress failed\n"); exit(3);
        }
        h = digest64(h, out, outlen);
    }
    printf("op=decompress in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)orig, iters, (unsigned long long)h);
    free(out);
}

/* metadata callbacks: fold sizes/bytes into the accumulator */
static uint64_t g_meta_h;
static void meta_start(void *opaque, size_t size) {
    (void)opaque;
    uint64_t s = (uint64_t)size;
    g_meta_h = digest64(g_meta_h, (const uint8_t *)&s, sizeof(s));
}
static void meta_chunk(void *opaque, const uint8_t *data, size_t size) {
    (void)opaque;
    g_meta_h = digest64(g_meta_h, data, size);
}

static void run_stream_roundtrip(const uint8_t *src, size_t n, long iters) {
    enum { IN_CHUNK = 65536, OUT_WIN = 4096, DEC_CHUNK = 7 };
    static const uint8_t metadata[] = "vw-metadata-block";
    size_t cap = BrotliEncoderMaxCompressedSize(n) + 4096;
    uint8_t *comp = malloc(cap);
    uint8_t *back = malloc(n);
    uint8_t outwin[OUT_WIN];
    if (!comp || !back) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0, comp_n = 0;
    for (long i = 0; i < iters; i++) {
        /* --- chunked encode via small window + TakeOutput drain --- */
        BrotliEncoderState *es = BrotliEncoderCreateInstance(NULL, NULL, NULL);
        BrotliEncoderSetParameter(es, BROTLI_PARAM_QUALITY, 5);
        BrotliEncoderSetParameter(es, BROTLI_PARAM_LGWIN, 22);
        comp_n = 0;
        size_t pos = 0;
        int meta_sent = 0;
        while (1) {
            size_t c = (n - pos < IN_CHUNK) ? (n - pos) : IN_CHUNK;
            const uint8_t *next_in;
            size_t avail_in;
            BrotliEncoderOperation eop;
            if (!meta_sent && pos >= n / 2) {
                next_in = metadata;
                avail_in = sizeof(metadata) - 1;
                eop = BROTLI_OPERATION_EMIT_METADATA;
                meta_sent = 1;
            } else {
                next_in = src + pos;
                avail_in = c;
                pos += c;
                eop = (pos >= n) ? BROTLI_OPERATION_FINISH : BROTLI_OPERATION_PROCESS;
            }
            do {
                uint8_t *next_out = outwin;
                size_t avail_out = OUT_WIN;
                if (!BrotliEncoderCompressStream(es, eop, &avail_in, &next_in,
                                                 &avail_out, &next_out, NULL)) {
                    fprintf(stderr, "stream compress failed\n"); exit(3);
                }
                size_t got = OUT_WIN - avail_out;
                memcpy(comp + comp_n, outwin, got);
                comp_n += got;
                while (BrotliEncoderHasMoreOutput(es)) {
                    size_t take = 0;
                    const uint8_t *tp = BrotliEncoderTakeOutput(es, &take);
                    if (!tp || !take) break;
                    memcpy(comp + comp_n, tp, take);
                    comp_n += take;
                }
            } while (avail_in > 0);
            if (BrotliEncoderIsFinished(es)) break;
        }
        h = digest64(h, comp, comp_n);
        BrotliEncoderDestroyInstance(es);

        /* --- tiny-chunk decode (safe bit-reader paths) + metadata cbs --- */
        BrotliDecoderState *ds = BrotliDecoderCreateInstance(NULL, NULL, NULL);
        BrotliDecoderSetParameter(ds, BROTLI_DECODER_PARAM_LARGE_WINDOW, 0);
        g_meta_h = 0;
        BrotliDecoderSetMetadataCallbacks(ds, meta_start, meta_chunk, NULL);
        size_t back_n = 0, dpos = 0;
        size_t din = 0;
        const uint8_t *dnext_in = comp;
        BrotliDecoderResult r = BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT;
        while (r != BROTLI_DECODER_RESULT_SUCCESS) {
            if (r == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT) {
                if (dpos >= comp_n) { fprintf(stderr, "stream decode starved\n"); exit(3); }
                size_t c = (comp_n - dpos < DEC_CHUNK) ? (comp_n - dpos) : DEC_CHUNK;
                dnext_in = comp + dpos;
                din = c;
                dpos += c;
            }
            uint8_t *dnext_out = outwin;
            size_t dout = OUT_WIN;
            r = BrotliDecoderDecompressStream(ds, &din, &dnext_in, &dout, &dnext_out, NULL);
            if (r == BROTLI_DECODER_RESULT_ERROR) {
                fprintf(stderr, "stream decode failed: %s\n",
                        BrotliDecoderErrorString(BrotliDecoderGetErrorCode(ds)));
                exit(3);
            }
            size_t got = OUT_WIN - dout;
            memcpy(back + back_n, outwin, got);
            back_n += got;
            while (BrotliDecoderHasMoreOutput(ds)) {
                size_t take = 0;
                const uint8_t *tp = BrotliDecoderTakeOutput(ds, &take);
                if (!tp || !take) break;
                memcpy(back + back_n, tp, take);
                back_n += take;
            }
        }
        uint64_t used[3] = { (uint64_t)BrotliDecoderIsUsed(ds),
                             (uint64_t)BrotliDecoderIsFinished(ds), g_meta_h };
        h = digest64(h, (const uint8_t *)used, sizeof(used));
        if (back_n != n || memcmp(back, src, n) != 0) {
            fprintf(stderr, "stream roundtrip mismatch (%zu/%zu)\n", back_n, n); exit(3);
        }
        h = digest64(h, back, back_n);
        BrotliDecoderDestroyInstance(ds);
    }
    printf("op=stream_roundtrip in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)comp_n, iters, (unsigned long long)h);
    free(comp); free(back);
}

/* shared raw dictionary: first 64KB is the dict, rest is the payload */
static void run_dict_roundtrip(const uint8_t *src, size_t n, long iters) {
    enum { DICT = 65536 };
    if (n <= DICT + 4096) { fprintf(stderr, "input too small\n"); exit(2); }
    const uint8_t *dict = src;
    const uint8_t *data = src + DICT;
    size_t dn = n - DICT;
    size_t cap = BrotliEncoderMaxCompressedSize(dn);
    uint8_t *comp = malloc(cap);
    uint8_t *back = malloc(dn);
    if (!comp || !back) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        BrotliEncoderPreparedDictionary *pd = BrotliEncoderPrepareDictionary(
            BROTLI_SHARED_DICTIONARY_RAW, DICT, dict, 5, NULL, NULL, NULL);
        if (!pd) { fprintf(stderr, "prepare dict failed\n"); exit(3); }
        uint64_t pds = (uint64_t)BrotliEncoderGetPreparedDictionarySize(pd);
        h = digest64(h, (const uint8_t *)&pds, sizeof(pds));
        BrotliEncoderState *es = BrotliEncoderCreateInstance(NULL, NULL, NULL);
        BrotliEncoderSetParameter(es, BROTLI_PARAM_QUALITY, 5);
        if (!BrotliEncoderAttachPreparedDictionary(es, pd)) {
            fprintf(stderr, "attach dict failed\n"); exit(3);
        }
        size_t avail_in = dn, avail_out = cap;
        const uint8_t *next_in = data;
        uint8_t *next_out = comp;
        if (!BrotliEncoderCompressStream(es, BROTLI_OPERATION_FINISH, &avail_in,
                                         &next_in, &avail_out, &next_out, NULL)
                || !BrotliEncoderIsFinished(es)) {
            fprintf(stderr, "dict compress failed\n"); exit(3);
        }
        size_t comp_n = cap - avail_out;
        h = digest64(h, comp, comp_n);
        BrotliEncoderDestroyInstance(es);
        BrotliEncoderDestroyPreparedDictionary(pd);

        BrotliDecoderState *ds = BrotliDecoderCreateInstance(NULL, NULL, NULL);
        if (!BrotliDecoderAttachDictionary(ds, BROTLI_SHARED_DICTIONARY_RAW, DICT, dict)) {
            fprintf(stderr, "decoder attach failed\n"); exit(3);
        }
        size_t din = comp_n, dout = dn;
        const uint8_t *dnext_in = comp;
        uint8_t *dnext_out = back;
        BrotliDecoderResult dr =
            BrotliDecoderDecompressStream(ds, &din, &dnext_in, &dout, &dnext_out, NULL);
        if (dr != BROTLI_DECODER_RESULT_SUCCESS || dout != 0
                || memcmp(back, data, dn) != 0) {
            fprintf(stderr, "dict roundtrip mismatch (r=%d)\n", (int)dr); exit(3);
        }
        h = digest64(h, back, dn);
        BrotliDecoderDestroyInstance(ds);
    }
    printf("op=dict_roundtrip in=%zu out=0 iters=%ld digest=%016llx\n",
           n, iters, (unsigned long long)h);
    free(comp); free(back);
}

static void run_misc(const uint8_t *src, size_t n, long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        uint64_t acc[4] = {
            (uint64_t)BrotliEncoderVersion(),
            (uint64_t)BrotliDecoderVersion(),
            (uint64_t)BrotliEncoderMaxCompressedSize(1 << 20),
            (uint64_t)BrotliEncoderEstimatePeakMemoryUsage(5, 22, 1 << 20),
        };
        h = digest64(h, (const uint8_t *)acc, sizeof(acc));
        /* decoder error surface on deliberately corrupt input */
        uint8_t bad[64];
        size_t bn = n < sizeof(bad) ? n : sizeof(bad);
        memcpy(bad, src, bn);
        for (size_t k = 0; k < bn; k++) bad[k] ^= 0xFF;
        uint8_t out[256];
        size_t outlen = sizeof(out);
        BrotliDecoderResult r = BrotliDecoderDecompress(bn, bad, &outlen, out);
        uint64_t e = (uint64_t)r;
        h = digest64(h, (const uint8_t *)&e, sizeof(e));
        BrotliDecoderState *ds = BrotliDecoderCreateInstance(NULL, NULL, NULL);
        size_t din = bn, dout = sizeof(out);
        const uint8_t *ni = bad;
        uint8_t *no = out;
        BrotliDecoderDecompressStream(ds, &din, &ni, &dout, &no, NULL);
        const char *es = BrotliDecoderErrorString(BrotliDecoderGetErrorCode(ds));
        if (es) h = digest64(h, (const uint8_t *)es, strlen(es));
        BrotliDecoderDestroyInstance(ds);
    }
    printf("op=misc in=%zu out=0 iters=%ld digest=%016llx\n",
           n, iters, (unsigned long long)h);
}

static void gen_br(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *src = read_file(in_path, &n);
    size_t cap = BrotliEncoderMaxCompressedSize(n), outlen = cap;
    uint8_t *out = malloc(cap);
    if (!BrotliEncoderCompress(9, 22, BROTLI_MODE_GENERIC, n, src, &outlen, out)) {
        fprintf(stderr, "compress failed\n"); exit(3);
    }
    FILE *f = fopen(out_path, "wb");
    uint64_t orig = (uint64_t)n;
    fwrite(&orig, 1, 8, f);
    fwrite(out, 1, outlen, f);
    fclose(f);
    fprintf(stderr, "wrote %s: %llu -> %zu\n", out_path,
            (unsigned long long)orig, outlen);
    free(src); free(out);
}

int main(int argc, char **argv) {
    BrotliEncoderEnsureStaticInit();
    BrotliDecoderEnsureStaticInit();
    if (argc == 4 && strcmp(argv[1], "gen_br") == 0) { gen_br(argv[2], argv[3]); return 0; }
    if (argc != 4) {
        fprintf(stderr, "usage: %s <op> <input> <iters>\n", argv[0]);
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }
    size_t n;
    uint8_t *buf = read_file(argv[2], &n);

    if (strcmp(op, "compress") == 0)              run_compress(buf, n, iters);
    else if (strcmp(op, "compress_hq") == 0)      run_compress_hq(buf, n, iters);
    else if (strcmp(op, "decompress") == 0)       run_decompress(buf, n, iters);
    else if (strcmp(op, "stream_roundtrip") == 0) run_stream_roundtrip(buf, n, iters);
    else if (strcmp(op, "dict_roundtrip") == 0)   run_dict_roundtrip(buf, n, iters);
    else if (strcmp(op, "misc") == 0)             run_misc(buf, n, iters);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    free(buf);
    return 0;
}
