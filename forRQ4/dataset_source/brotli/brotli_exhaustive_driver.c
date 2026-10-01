/* brotli exhaustive driver — exercises encoder+decoder streaming APIs +
 * BrotliEncoderCompressStream + BrotliDecoderDecompressStream + custom
 * dictionary attachment + every mode/quality/window combination.
 *
 * Drives the encoder's many internal hash-table strategies (rolling /
 * forgetful_chain / longest_match / longest_match_quickly / composite).
 */
#include <brotli/encode.h>
#include <brotli/decode.h>
#include <brotli/types.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char *SAMPLE =
    "The quick brown fox jumps over the lazy dog. "
    "Pack my box with five dozen liquor jugs. "
    "The quick brown fox jumps over the lazy dog. "
    "Pack my box with five dozen liquor jugs. "
    "Sphinx of black quartz, judge my vow. "
    "How vexingly quick daft zebras jump! "
    "Brotli is a lossless compression format suitable for web content. "
    "Brotli is a lossless compression format suitable for web content. ";

static void test_one_shot(int quality, int window, BrotliEncoderMode mode,
                          const uint8_t *src, size_t src_len) {
    size_t enc_size = BrotliEncoderMaxCompressedSize(src_len);
    if (!enc_size) enc_size = src_len * 2 + 128;
    uint8_t *enc = malloc(enc_size);
    BROTLI_BOOL ok = BrotliEncoderCompress(quality, window, mode,
                                            src_len, src, &enc_size, enc);
    if (!ok) { free(enc); return; }
    /* Round-trip. */
    size_t dec_size = src_len + 16;
    uint8_t *dec = malloc(dec_size);
    BrotliDecoderResult dr = BrotliDecoderDecompress(enc_size, enc,
                                                      &dec_size, dec);
    (void)dr;
    free(enc); free(dec);
}

static void test_stream_encoder(int quality, int window, BrotliEncoderMode mode,
                                 const uint8_t *src, size_t src_len) {
    BrotliEncoderState *st = BrotliEncoderCreateInstance(NULL, NULL, NULL);
    if (!st) return;
    BrotliEncoderSetParameter(st, BROTLI_PARAM_QUALITY, quality);
    BrotliEncoderSetParameter(st, BROTLI_PARAM_LGWIN, window);
    BrotliEncoderSetParameter(st, BROTLI_PARAM_MODE, mode);
    BrotliEncoderSetParameter(st, BROTLI_PARAM_SIZE_HINT, src_len);
    BrotliEncoderSetParameter(st, BROTLI_PARAM_LARGE_WINDOW, 0);
    /* Feed input in chunks. */
    uint8_t out[8192];
    size_t avail_in = src_len;
    const uint8_t *next_in = src;
    size_t avail_out = sizeof out;
    uint8_t *next_out = out;
    while (!BrotliEncoderIsFinished(st)) {
        BROTLI_BOOL r = BrotliEncoderCompressStream(st,
            avail_in > 0 ? BROTLI_OPERATION_PROCESS : BROTLI_OPERATION_FINISH,
            &avail_in, &next_in, &avail_out, &next_out, NULL);
        if (!r) break;
        if (BrotliEncoderHasMoreOutput(st)) {
            size_t out_sz = 0;
            const uint8_t *out_buf = BrotliEncoderTakeOutput(st, &out_sz);
            (void)out_buf;
        }
        if (avail_out == 0) { avail_out = sizeof out; next_out = out; }
    }
    BrotliEncoderDestroyInstance(st);

    /* Flush operation. */
    BrotliEncoderState *fl = BrotliEncoderCreateInstance(NULL, NULL, NULL);
    if (fl) {
        BrotliEncoderSetParameter(fl, BROTLI_PARAM_QUALITY, quality);
        size_t ai = src_len > 16 ? 16 : src_len;
        const uint8_t *ni = src;
        size_t ao = sizeof out;
        uint8_t *no = out;
        BrotliEncoderCompressStream(fl, BROTLI_OPERATION_FLUSH,
                                     &ai, &ni, &ao, &no, NULL);
        BrotliEncoderDestroyInstance(fl);
    }

    /* Version + max-compressed-size getters. */
    (void)BrotliEncoderVersion();
    (void)BrotliEncoderMaxCompressedSize(src_len);
}

static void test_stream_decoder(const uint8_t *enc, size_t enc_size,
                                 size_t orig_size) {
    BrotliDecoderState *st = BrotliDecoderCreateInstance(NULL, NULL, NULL);
    if (!st) return;
    BrotliDecoderSetParameter(st, BROTLI_DECODER_PARAM_LARGE_WINDOW, 0);
    BrotliDecoderSetParameter(st, BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION, 0);
    size_t avail_in = enc_size;
    const uint8_t *next_in = enc;
    uint8_t *outbuf = malloc(orig_size + 64);
    size_t avail_out = orig_size + 64;
    uint8_t *next_out = outbuf;
    BrotliDecoderResult r;
    while ((r = BrotliDecoderDecompressStream(st, &avail_in, &next_in,
                                               &avail_out, &next_out, NULL))
           == BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT) {
        if (BrotliDecoderHasMoreOutput(st)) {
            size_t sz = 0;
            const uint8_t *p = BrotliDecoderTakeOutput(st, &sz);
            (void)p;
        }
        avail_out = orig_size + 64;
        next_out = outbuf;
    }
    (void)BrotliDecoderIsUsed(st);
    (void)BrotliDecoderIsFinished(st);
    (void)BrotliDecoderGetErrorCode(st);
    (void)BrotliDecoderErrorString(BrotliDecoderGetErrorCode(st));
    (void)BrotliDecoderVersion();
    BrotliDecoderDestroyInstance(st);
    free(outbuf);
}

static void test_with_dictionary(int quality, int window,
                                  const uint8_t *src, size_t src_len) {
    /* Raw LZ77 dictionary. */
    const uint8_t dict[] =
        "The quick brown fox jumps over the lazy dog. "
        "Brotli is a lossless compression format suitable for web content. ";
    size_t dict_len = sizeof dict - 1;

    BrotliEncoderPreparedDictionary *pd = BrotliEncoderPrepareDictionary(
        BROTLI_SHARED_DICTIONARY_RAW, dict_len, dict, quality, NULL, NULL, NULL);
    if (!pd) return;
    BrotliEncoderState *st = BrotliEncoderCreateInstance(NULL, NULL, NULL);
    if (!st) { BrotliEncoderDestroyPreparedDictionary(pd); return; }
    BrotliEncoderSetParameter(st, BROTLI_PARAM_QUALITY, quality);
    BrotliEncoderSetParameter(st, BROTLI_PARAM_LGWIN, window);
    BrotliEncoderAttachPreparedDictionary(st, pd);
    size_t out_size = BrotliEncoderMaxCompressedSize(src_len) + 256;
    uint8_t *out = malloc(out_size);
    size_t avail_in = src_len;
    const uint8_t *next_in = src;
    size_t avail_out = out_size;
    uint8_t *next_out = out;
    while (!BrotliEncoderIsFinished(st)) {
        if (!BrotliEncoderCompressStream(st,
            avail_in > 0 ? BROTLI_OPERATION_PROCESS : BROTLI_OPERATION_FINISH,
            &avail_in, &next_in, &avail_out, &next_out, NULL)) break;
    }
    BrotliEncoderDestroyInstance(st);
    BrotliEncoderDestroyPreparedDictionary(pd);
    free(out);
}

int main(void) {
    /* Build a richer corpus to trigger more hash strategies. */
    size_t corpus_len = 256 * 1024;
    uint8_t *corpus = malloc(corpus_len);
    size_t sl = strlen(SAMPLE);
    for (size_t i = 0; i < corpus_len; i++)
        corpus[i] = SAMPLE[i % sl];

    /* One-shot at each quality x mode x window combination. */
    int qualities[] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11};
    BrotliEncoderMode modes[] = {BROTLI_MODE_GENERIC, BROTLI_MODE_TEXT, BROTLI_MODE_FONT};
    int windows[] = {10, 16, 22, 24};
    for (size_t q = 0; q < sizeof(qualities)/sizeof(*qualities); q++) {
        for (size_t m = 0; m < sizeof(modes)/sizeof(*modes); m++) {
            for (size_t w = 0; w < sizeof(windows)/sizeof(*windows); w++) {
                test_one_shot(qualities[q], windows[w], modes[m],
                              corpus, corpus_len);
            }
        }
    }

    /* Streaming encoder/decoder at representative qualities. */
    for (size_t q = 0; q < sizeof(qualities)/sizeof(*qualities); q++) {
        test_stream_encoder(qualities[q], 22, BROTLI_MODE_TEXT,
                            corpus, corpus_len);
    }

    /* Round-trip via streaming. */
    size_t enc_size = BrotliEncoderMaxCompressedSize(corpus_len);
    uint8_t *enc = malloc(enc_size);
    BROTLI_BOOL ok = BrotliEncoderCompress(11, 22, BROTLI_MODE_TEXT,
                                            corpus_len, corpus,
                                            &enc_size, enc);
    if (ok) test_stream_decoder(enc, enc_size, corpus_len);
    free(enc);

    /* Custom dictionary (RAW). */
    test_with_dictionary(6, 22, corpus, corpus_len);
    test_with_dictionary(11, 22, corpus, corpus_len);

    /* Encode a binary noise corpus — triggers different hash selection
     * (e.g., FONT mode + small NDIRECT + lower contextual modeling). */
    size_t noise_len = 256 * 1024;
    uint8_t *noise = malloc(noise_len);
    uint32_t seed = 12345;
    for (size_t i = 0; i < noise_len; i++) {
        seed = seed * 1103515245 + 12345;
        noise[i] = (seed >> 16) & 0xff;
    }
    for (size_t q = 0; q < sizeof(qualities)/sizeof(*qualities); q++) {
        for (size_t w = 0; w < sizeof(windows)/sizeof(*windows); w++) {
            test_one_shot(qualities[q], windows[w], BROTLI_MODE_FONT,
                          noise, noise_len);
        }
    }

    /* Encode at extreme low + extreme high size with various NPOSTFIX/NDIRECT. */
    for (size_t q = 0; q < sizeof(qualities)/sizeof(*qualities); q++) {
        for (int npostfix = 0; npostfix <= 3; npostfix++) {
            for (int ndirect = 0; ndirect <= 15; ndirect += 5) {
                BrotliEncoderState *st = BrotliEncoderCreateInstance(NULL, NULL, NULL);
                if (!st) continue;
                BrotliEncoderSetParameter(st, BROTLI_PARAM_QUALITY, qualities[q]);
                BrotliEncoderSetParameter(st, BROTLI_PARAM_LGWIN, 22);
                BrotliEncoderSetParameter(st, BROTLI_PARAM_LGBLOCK, 16);
                BrotliEncoderSetParameter(st, BROTLI_PARAM_NPOSTFIX, npostfix);
                BrotliEncoderSetParameter(st, BROTLI_PARAM_NDIRECT, ndirect);
                BrotliEncoderSetParameter(st, BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING, 1);
                size_t avail_in = noise_len;
                const uint8_t *next_in = noise;
                uint8_t out[8192];
                size_t avail_out = sizeof out;
                uint8_t *next_out = out;
                while (!BrotliEncoderIsFinished(st)) {
                    if (!BrotliEncoderCompressStream(st,
                        avail_in > 0 ? BROTLI_OPERATION_PROCESS : BROTLI_OPERATION_FINISH,
                        &avail_in, &next_in, &avail_out, &next_out, NULL)) break;
                    if (avail_out == 0) { avail_out = sizeof out; next_out = out; }
                }
                BrotliEncoderDestroyInstance(st);
            }
        }
    }

    /* Force-pick H_ROLLING + H_FORGETFUL_CHAIN by using q=11 + LARGE_WINDOW + 512KB input. */
    size_t huge_len = 512 * 1024;
    uint8_t *huge = malloc(huge_len);
    /* Mix repetitive + noisy patterns so different hashers help. */
    for (size_t i = 0; i < huge_len; i++) {
        if ((i / 64) & 1) huge[i] = SAMPLE[i % sl];
        else { seed = seed * 1103515245 + 12345; huge[i] = (seed >> 16) & 0xff; }
    }
    /* q=11 with various LGWIN values + LARGE_WINDOW. */
    for (int lgwin = 16; lgwin <= 24; lgwin += 2) {
        BrotliEncoderState *st = BrotliEncoderCreateInstance(NULL, NULL, NULL);
        if (!st) continue;
        BrotliEncoderSetParameter(st, BROTLI_PARAM_QUALITY, 11);
        BrotliEncoderSetParameter(st, BROTLI_PARAM_LGWIN, lgwin);
        BrotliEncoderSetParameter(st, BROTLI_PARAM_LARGE_WINDOW, 1);
        BrotliEncoderSetParameter(st, BROTLI_PARAM_SIZE_HINT, huge_len);
        size_t avail_in = huge_len;
        const uint8_t *next_in = huge;
        uint8_t out[16384];
        size_t avail_out = sizeof out;
        uint8_t *next_out = out;
        while (!BrotliEncoderIsFinished(st)) {
            if (!BrotliEncoderCompressStream(st,
                avail_in > 0 ? BROTLI_OPERATION_PROCESS : BROTLI_OPERATION_FINISH,
                &avail_in, &next_in, &avail_out, &next_out, NULL)) break;
            if (avail_out == 0) { avail_out = sizeof out; next_out = out; }
        }
        BrotliEncoderDestroyInstance(st);
    }
    /* q=4 with various NPOSTFIX (touches longest_match_quickly variants). */
    for (int npostfix = 0; npostfix <= 3; npostfix++) {
        BrotliEncoderState *st = BrotliEncoderCreateInstance(NULL, NULL, NULL);
        if (!st) continue;
        BrotliEncoderSetParameter(st, BROTLI_PARAM_QUALITY, 4);
        BrotliEncoderSetParameter(st, BROTLI_PARAM_LGWIN, 22);
        BrotliEncoderSetParameter(st, BROTLI_PARAM_NPOSTFIX, npostfix);
        size_t avail_in = huge_len;
        const uint8_t *next_in = huge;
        uint8_t out[16384];
        size_t avail_out = sizeof out;
        uint8_t *next_out = out;
        while (!BrotliEncoderIsFinished(st)) {
            if (!BrotliEncoderCompressStream(st,
                avail_in > 0 ? BROTLI_OPERATION_PROCESS : BROTLI_OPERATION_FINISH,
                &avail_in, &next_in, &avail_out, &next_out, NULL)) break;
            if (avail_out == 0) { avail_out = sizeof out; next_out = out; }
        }
        BrotliEncoderDestroyInstance(st);
    }
    free(huge);

    /* BrotliEncoderEstimatePeakMemoryUsage + GetPreparedDictionarySize
     * are exported in newer versions only; skip in this build. */
    /* Encoder EMIT_METADATA op (touches ProcessMetadata / WriteMetadataHeader). */
    {
        BrotliEncoderState *st = BrotliEncoderCreateInstance(NULL, NULL, NULL);
        if (st) {
            BrotliEncoderSetParameter(st, BROTLI_PARAM_QUALITY, 6);
            uint8_t meta[] = "metadata-content";
            size_t avail_in = sizeof meta - 1;
            const uint8_t *next_in = meta;
            uint8_t out[1024];
            size_t avail_out = sizeof out;
            uint8_t *next_out = out;
            BrotliEncoderCompressStream(st, BROTLI_OPERATION_EMIT_METADATA,
                                         &avail_in, &next_in, &avail_out, &next_out, NULL);
            BrotliEncoderCompressStream(st, BROTLI_OPERATION_FINISH,
                                         &avail_in, &next_in, &avail_out, &next_out, NULL);
            BrotliEncoderDestroyInstance(st);
        }
    }
    /* Decoder: attach RAW dictionary + metadata callbacks. */
    {
        const uint8_t dict[] = "shared-dict-content";
        /* Build a small compressed stream first. */
        size_t enc_sz = 1024;
        uint8_t enc_buf[1024];
        BROTLI_BOOL ok = BrotliEncoderCompress(11, 22, BROTLI_MODE_TEXT,
            sizeof dict - 1, dict, &enc_sz, enc_buf);
        if (ok) {
            BrotliDecoderState *ds = BrotliDecoderCreateInstance(NULL, NULL, NULL);
            if (ds) {
                BrotliDecoderAttachDictionary(ds, BROTLI_SHARED_DICTIONARY_RAW,
                                               sizeof dict - 1, dict);
                BrotliDecoderSetMetadataCallbacks(ds, NULL, NULL, NULL);
                size_t avail_in = enc_sz;
                const uint8_t *next_in = enc_buf;
                uint8_t dec[256];
                size_t avail_out = sizeof dec;
                uint8_t *next_out = dec;
                BrotliDecoderDecompressStream(ds, &avail_in, &next_in,
                                               &avail_out, &next_out, NULL);
                /* Explicit TakeOutput. */
                size_t to = 0;
                (void)BrotliDecoderTakeOutput(ds, &to);
                BrotliDecoderDestroyInstance(ds);
            }
        }
    }

    free(noise);
    free(corpus);
    return 0;
}
