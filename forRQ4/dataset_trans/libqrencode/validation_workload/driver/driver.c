/*
 * validation driver for libqrencode — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves.
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     encode_matrix   : QRcode_encodeString over the string list x EC levels
 *                       {L,M,Q,H} x versions {auto,5} (case-sensitive 8-bit hint)
 *     encode_8bit     : QRcode_encodeString8bit + QRcode_encodeData slices
 *     encode_mqr      : Micro QR (encodeStringMQR/8bitMQR/DataMQR, short inputs)
 *     structured      : structured-append split across symbols + QRcode_List_*
 *     input_builder   : QRinput_* builder surface (append modes incl. Kanji,
 *                       version/EC setters, estimate/check/dup/getByteStream,
 *                       FNC1, ECI header, Struct split/append/parity)
 *     split_op        : Split_splitStringToQRinput mode detection
 *
 * Input: strings.txt (one payload per line: real URLs, text lines, numeric
 * and alphanumeric codes). digest folds symbol width + module bytes.
 * Output: ONE digest line on stdout (project convention).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "qrencode.h"

/* internal-but-linked headers: estimateBits / check / ECI live in qrinput.h,
 * Split_ helpers in split.h -- part of the translated library surface */
#include "qrinput.h"
#include "split.h"

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

static char **split_lines(char *buf, size_t n, size_t *out_count) {
    size_t cap = 1024, cnt = 0;
    char **lines = malloc(cap * sizeof(char *));
    char *p = buf, *end = buf + n;
    while (p < end) {
        char *nl = memchr(p, '\n', (size_t)(end - p));
        if (!nl) nl = end;
        *nl = 0;
        if (*p) {
            if (cnt == cap) { cap *= 2; lines = realloc(lines, cap * sizeof(char *)); }
            lines[cnt++] = p;
        }
        p = nl + 1;
    }
    *out_count = cnt;
    return lines;
}

static uint64_t fold_qr(uint64_t h, QRcode *qr) {
    if (!qr) return digest64(h, (const uint8_t *)"null", 4);
    uint64_t meta[2] = { (uint64_t)qr->version, (uint64_t)qr->width };
    h = digest64(h, (const uint8_t *)meta, sizeof(meta));
    h = digest64(h, qr->data, (size_t)qr->width * qr->width);
    return h;
}

static const QRecLevel LEVELS[4] = { QR_ECLEVEL_L, QR_ECLEVEL_M, QR_ECLEVEL_Q, QR_ECLEVEL_H };

static void run_encode_matrix(char *buf, size_t n, long iters) {
    size_t cnt;
    char **lines = split_lines(buf, n, &cnt);
    uint64_t h = 0, symbols = 0;
    for (long i = 0; i < iters; i++) {
        symbols = 0;
        for (size_t j = 0; j < cnt; j++) {
            for (int lv = 0; lv < 4; lv++) {
                int version = (j % 2) ? 5 : 0;   /* alternate fixed / auto */
                QRcode *qr = QRcode_encodeString(lines[j], version, LEVELS[lv],
                                                 QR_MODE_8, 1);
                if (!qr) { fprintf(stderr, "encodeString failed on line %zu\n", j); exit(3); }
                h = fold_qr(h, qr);
                QRcode_free(qr);
                symbols++;
            }
        }
    }
    printf("op=encode_matrix in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)symbols, iters, (unsigned long long)h);
    free(lines);
}

static void run_encode_8bit(char *buf, size_t n, long iters) {
    size_t cnt;
    char **lines = split_lines(buf, n, &cnt);
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        for (size_t j = 0; j < cnt; j++) {
            QRcode *qr = QRcode_encodeString8bit(lines[j], 0, LEVELS[j % 4]);
            if (!qr) { fprintf(stderr, "encodeString8bit failed\n"); exit(3); }
            h = fold_qr(h, qr);
            QRcode_free(qr);
            size_t len = strlen(lines[j]);
            QRcode *qd = QRcode_encodeData((int)len, (unsigned char *)lines[j], 0, QR_ECLEVEL_M);
            if (!qd) { fprintf(stderr, "encodeData failed\n"); exit(3); }
            h = fold_qr(h, qd);
            QRcode_free(qd);
        }
    }
    printf("op=encode_8bit in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, cnt, iters, (unsigned long long)h);
    free(lines);
}

static void run_encode_mqr(char *buf, size_t n, long iters) {
    size_t cnt;
    char **lines = split_lines(buf, n, &cnt);
    uint64_t h = 0, done = 0;
    for (long i = 0; i < iters; i++) {
        done = 0;
        for (size_t j = 0; j < cnt; j++) {
            char shorty[16];
            strncpy(shorty, lines[j], 10);      /* MQR capacity is tiny */
            shorty[10] = 0;
            QRcode *qr = QRcode_encodeStringMQR(shorty, 3, QR_ECLEVEL_L, QR_MODE_8, 1);
            if (qr) { h = fold_qr(h, qr); QRcode_free(qr); done++; }
            QRcode *q8 = QRcode_encodeString8bitMQR(shorty, 4, QR_ECLEVEL_L);
            if (q8) { h = fold_qr(h, q8); QRcode_free(q8); done++; }
            QRcode *qd = QRcode_encodeDataMQR(6, (unsigned char *)shorty, 4, QR_ECLEVEL_L);
            if (qd) { h = fold_qr(h, qd); QRcode_free(qd); done++; }
        }
    }
    printf("op=encode_mqr in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)done, iters, (unsigned long long)h);
    free(lines);
}

static void run_structured(char *buf, size_t n, long iters) {
    size_t cnt;
    char **lines = split_lines(buf, n, &cnt);
    /* one long payload: concatenate lines up to ~1.5KB (forces multi-symbol) */
    /* structured append allows at most 16 symbols: ~800B at version 6-L
     * splits into a handful of symbols */
    char big[1024];
    size_t off = 0;
    for (size_t j = 0; j < cnt && off < 800; j++) {
        size_t l = strlen(lines[j]);
        if (off + l + 1 >= 800) break;
        memcpy(big + off, lines[j], l);
        off += l;
        big[off++] = ' ';
    }
    big[off] = 0;
    uint64_t h = 0, syms = 0;
    for (long i = 0; i < iters; i++) {
        QRcode_List *list = QRcode_encodeString8bitStructured(big, 6, QR_ECLEVEL_L);
        if (!list) { fprintf(stderr, "structured failed\n"); exit(3); }
        syms = (uint64_t)QRcode_List_size(list);
        for (QRcode_List *e = list; e; e = e->next)
            h = fold_qr(h, e->code);
        QRcode_List_free(list);
        QRcode_List *l2 = QRcode_encodeStringStructured(big, 7, QR_ECLEVEL_M, QR_MODE_8, 1);
        if (l2) {
            uint64_t s2 = (uint64_t)QRcode_List_size(l2);
            h = digest64(h, (const uint8_t *)&s2, sizeof(s2));
            for (QRcode_List *e = l2; e; e = e->next) h = fold_qr(h, e->code);
            QRcode_List_free(l2);
        }
        QRcode_List *l3 = QRcode_encodeDataStructured((int)off, (unsigned char *)big, 6, QR_ECLEVEL_L);
        if (l3) {
            for (QRcode_List *e = l3; e; e = e->next) h = fold_qr(h, e->code);
            QRcode_List_free(l3);
        }
    }
    printf("op=structured in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)syms, iters, (unsigned long long)h);
    free(lines);
}

static void run_input_builder(long iters) {
    static const unsigned char kanji[4] = { 0x93, 0x5f, 0x93, 0x5f };  /* SJIS */
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        QRinput *in = QRinput_new2(0, QR_ECLEVEL_Q);
        if (!in) { fprintf(stderr, "QRinput_new2 failed\n"); exit(3); }
        QRinput_append(in, QR_MODE_NUM, 10, (const unsigned char *)"0123456789");
        QRinput_append(in, QR_MODE_AN, 8, (const unsigned char *)"AC-42$/+");
        QRinput_append(in, QR_MODE_8, 12, (const unsigned char *)"binary\x01\x02:-)#");
        QRinput_append(in, QR_MODE_KANJI, 4, kanji);
        uint64_t est[5] = {
            (uint64_t)QRinput_estimateBitsModeNum(10),
            (uint64_t)QRinput_estimateBitsModeAn(8),
            (uint64_t)QRinput_estimateBitsMode8(12),
            (uint64_t)QRinput_estimateBitsModeKanji(4),
            (uint64_t)QRinput_check(QR_MODE_AN, 8, (const unsigned char *)"AC-42$/+"),
        };
        h = digest64(h, (const uint8_t *)est, sizeof(est));
        QRinput_setVersionAndErrorCorrectionLevel(in, 10, QR_ECLEVEL_H);
        uint64_t got[2] = { (uint64_t)QRinput_getVersion(in),
                            (uint64_t)QRinput_getErrorCorrectionLevel(in) };
        h = digest64(h, (const uint8_t *)got, sizeof(got));
        QRinput *dup = QRinput_dup(in);
        QRcode *qr = QRcode_encodeInput(dup);
        if (qr) { h = fold_qr(h, qr); QRcode_free(qr); }
        QRinput_free(dup);

        /* structured-append via the builder */
        QRinput *big = QRinput_new2(0, QR_ECLEVEL_L);
        static unsigned char blob[800];
        for (size_t b = 0; b < sizeof(blob); b++) blob[b] = (unsigned char)(b * 7 + 1);
        QRinput_append(big, QR_MODE_8, (int)sizeof(blob), blob);
        QRinput_setVersion(big, 5);
        QRinput_setErrorCorrectionLevel(big, QR_ECLEVEL_L);
        QRinput_Struct *st = QRinput_splitQRinputToStruct(big);
        if (st) {
            QRinput_Struct_setParity(st, 0x42);
            QRinput_Struct_insertStructuredAppendHeaders(st);
            QRcode_List *list = QRcode_encodeInputStructured(st);
            if (list) {
                uint64_t sz = (uint64_t)QRcode_List_size(list);
                h = digest64(h, (const uint8_t *)&sz, sizeof(sz));
                for (QRcode_List *e = list; e; e = e->next) h = fold_qr(h, e->code);
                QRcode_List_free(list);
            }
            QRinput_Struct_free(st);
        }
        QRinput_free(big);

        /* FNC1 + ECI + MQR input + misc */
        QRinput *f1 = QRinput_new();
        QRinput_setFNC1First(f1);
        QRinput_append(f1, QR_MODE_NUM, 4, (const unsigned char *)"1234");
        QRcode *qf = QRcode_encodeInput(f1);
        if (qf) { h = fold_qr(h, qf); QRcode_free(qf); }
        QRinput_free(f1);
        QRinput *f2 = QRinput_new2(0, QR_ECLEVEL_M);
        QRinput_setFNC1Second(f2, 'A');
        QRinput_appendECIheader(f2, 26);
        QRinput_append(f2, QR_MODE_8, 5, (const unsigned char *)"hello");
        QRcode *qe = QRcode_encodeInput(f2);
        if (qe) { h = fold_qr(h, qe); QRcode_free(qe); }
        QRinput_free(f2);
        QRinput *mq = QRinput_newMQR(3, QR_ECLEVEL_L);
        if (mq) {
            QRinput_append(mq, QR_MODE_NUM, 6, (const unsigned char *)"424242");
            uint64_t sp = (uint64_t)QRinput_isSplittableMode(QR_MODE_8);
            h = digest64(h, (const uint8_t *)&sp, sizeof(sp));
            unsigned char *bs = NULL;
            /* getByteStream on the main input */
            bs = QRinput_getByteStream(in);
            if (bs) { free(bs); }
            QRinput_free(mq);
        }
        QRinput_free(in);
        int vmaj = 0, vmin = 0, vmic = 0;
        QRcode_APIVersion(&vmaj, &vmin, &vmic);
        uint64_t api[2] = { (uint64_t)vmaj << 16 | (uint64_t)vmin << 8 | (uint64_t)vmic,
                            (uint64_t)strlen(QRcode_APIVersionString()) };
        h = digest64(h, (const uint8_t *)api, sizeof(api));
    }
    QRcode_clearCache();
    printf("op=input_builder in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

static void run_split_op(char *buf, size_t n, long iters) {
    size_t cnt;
    char **lines = split_lines(buf, n, &cnt);
    /* SJIS sample so the Kanji-eating path in split.c is reachable */
    static const char sjis_line[] = "QR\x93\x5f\x93\x5f 123ABC";
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        for (size_t j = 0; j < cnt; j++) {
            /* case-sensitive 8-bit hint + case-folding Kanji hint (both real) */
            for (int mode = 0; mode < 2; mode++) {
                QRinput *in = QRinput_new2(0, QR_ECLEVEL_M);
                int r = mode
                    ? Split_splitStringToQRinput(lines[j], in, QR_MODE_KANJI, 0)
                    : Split_splitStringToQRinput(lines[j], in, QR_MODE_8, 1);
                uint64_t rr = (uint64_t)r;
                h = digest64(h, (const uint8_t *)&rr, sizeof(rr));
                if (r == 0) {
                    QRcode *qr = QRcode_encodeInput(in);
                    if (qr) { h = fold_qr(h, qr); QRcode_free(qr); }
                }
                QRinput_free(in);
            }
        }
        QRinput *ki = QRinput_new2(0, QR_ECLEVEL_M);
        if (Split_splitStringToQRinput(sjis_line, ki, QR_MODE_KANJI, 0) == 0) {
            QRcode *qk = QRcode_encodeInput(ki);
            if (qk) { h = fold_qr(h, qk); QRcode_free(qk); }
        }
        QRinput_free(ki);

        /* struct split WITHOUT explicit parity -> auto calcParity path */
        QRinput *auto_in = QRinput_new2(0, QR_ECLEVEL_L);
        static unsigned char blob2[600];
        for (size_t b = 0; b < sizeof(blob2); b++) blob2[b] = (unsigned char)(b * 13 + 5);
        QRinput_append(auto_in, QR_MODE_8, (int)sizeof(blob2), blob2);
        QRinput_setVersion(auto_in, 6);
        QRinput_Struct *st2 = QRinput_splitQRinputToStruct(auto_in);
        if (st2) {
            QRinput_Struct_insertStructuredAppendHeaders(st2);
            QRcode_List *lst = QRcode_encodeInputStructured(st2);
            if (lst) {
                for (QRcode_List *e = lst; e; e = e->next) h = fold_qr(h, e->code);
                QRcode_List_free(lst);
            }
            QRinput_Struct_free(st2);
        }
        QRinput_free(auto_in);
    }
    printf("op=split_op in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, cnt, iters, (unsigned long long)h);
    free(lines);
}

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "usage: %s <op> <input> <iters>\n", argv[0]);
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }

    if (strcmp(op, "input_builder") == 0) { run_input_builder(iters); return 0; }

    size_t n;
    uint8_t *buf = read_file(argv[2], &n);
    if (strcmp(op, "encode_matrix") == 0)      run_encode_matrix((char *)buf, n, iters);
    else if (strcmp(op, "encode_8bit") == 0)   run_encode_8bit((char *)buf, n, iters);
    else if (strcmp(op, "encode_mqr") == 0)    run_encode_mqr((char *)buf, n, iters);
    else if (strcmp(op, "structured") == 0)    run_structured((char *)buf, n, iters);
    else if (strcmp(op, "split_op") == 0)      run_split_op((char *)buf, n, iters);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    free(buf);
    return 0;
}
