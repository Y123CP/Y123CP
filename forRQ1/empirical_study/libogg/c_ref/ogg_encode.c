/* ogg_encode — C reference for the Rust harness
 *   dataset_trans_process/libogg/workloads/harness/ogg_encode/src/main.rs
 *
 * EMPIRICAL STUDY: two operations dispatched by argv[1] so ONE binary covers
 * libogg's ENCODE and DECODE hot paths. Both self-contained (input ignored);
 * C and Rust MUST print identical stdout.
 *   default  → ENCODE: init a stream, feed M packets, flush each into a page,
 *              fold the page HEADER (carries CRC). Prints "packets=M checksum=HEX".
 *   "decode" → DECODE: build the encoded byte-buffer ONCE (M_DEC packets), then
 *              loop N_DEC times through ogg_sync_buffer/wrote + ogg_sync_pageout +
 *              ogg_stream_pagein + ogg_stream_packetout, folding each packet's
 *              (packetno,bytes). Prints "decoded checksum=HEX".
 *
 * Build (fair): clang-17 -O3 -flto -march=native -DNDEBUG -Iinclude -Ibuild/include \
 *               ogg_encode.c src/bitwise.c src/framing.c -o <out>
 */
#include <ogg/ogg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define M 300000
#define M_DEC 30000
#define N_DEC 10
#define PLEN 1024

static void fill_payload(unsigned char *payload) {
    for (int i = 0; i < PLEN; i++)
        payload[i] = (unsigned char)(i * 31 + 7);
}

static void encode(void) {
    ogg_stream_state os;
    memset(&os, 0, sizeof(os));
    ogg_stream_init(&os, 0x1234);
    unsigned char payload[PLEN];
    fill_payload(payload);

    unsigned long checksum = 0;
    for (long n = 0; n < M; n++) {
        ogg_packet op;
        op.packet = payload;
        op.bytes = PLEN;
        op.b_o_s = (n == 0);
        op.e_o_s = (n == M - 1);
        op.granulepos = n;
        op.packetno = n;
        ogg_stream_packetin(&os, &op);
        ogg_page og;
        while (ogg_stream_flush(&os, &og)) {
            /* Fold ONLY the page header (carries the 4-byte CRC) — see note. */
            for (long k = 0; k < og.header_len; k++)
                checksum = ((checksum << 1) | (checksum >> 63)) ^ og.header[k];
        }
    }
    ogg_stream_clear(&os);
    printf("packets=%d checksum=%016lx\n", M, checksum);
}

/* Encode M_DEC packets, capturing full page bytes (header+body) into *pbuf. */
static unsigned char *build_encoded_buffer(long *plen) {
    ogg_stream_state os;
    memset(&os, 0, sizeof(os));
    ogg_stream_init(&os, 0x1234);
    unsigned char payload[PLEN];
    fill_payload(payload);

    size_t cap = 1 << 20, len = 0;
    unsigned char *buf = (unsigned char *)malloc(cap);
    for (long n = 0; n < M_DEC; n++) {
        ogg_packet op;
        op.packet = payload;
        op.bytes = PLEN;
        op.b_o_s = (n == 0);
        op.e_o_s = (n == M_DEC - 1);
        op.granulepos = n;
        op.packetno = n;
        ogg_stream_packetin(&os, &op);
        ogg_page og;
        while (ogg_stream_flush(&os, &og)) {
            size_t need = (size_t)(og.header_len + og.body_len);
            while (len + need > cap) { cap <<= 1; buf = (unsigned char *)realloc(buf, cap); }
            memcpy(buf + len, og.header, (size_t)og.header_len); len += og.header_len;
            memcpy(buf + len, og.body, (size_t)og.body_len);   len += og.body_len;
        }
    }
    ogg_stream_clear(&os);
    *plen = (long)len;
    return buf;
}

static void decode(void) {
    long len;
    unsigned char *buf = build_encoded_buffer(&len);
    unsigned long checksum = 0;
    for (int it = 0; it < N_DEC; it++) {
        ogg_sync_state oy;
        ogg_sync_init(&oy);
        ogg_stream_state os;
        memset(&os, 0, sizeof(os));
        ogg_stream_init(&os, 0x1234);

        char *dst = ogg_sync_buffer(&oy, len);
        memcpy(dst, buf, (size_t)len);
        ogg_sync_wrote(&oy, len);

        ogg_page og;
        while (ogg_sync_pageout(&oy, &og) == 1) {
            ogg_stream_pagein(&os, &og);
            ogg_packet op;
            while (ogg_stream_packetout(&os, &op) == 1) {
                checksum = ((checksum << 1) | (checksum >> 63))
                           ^ (unsigned long)op.packetno ^ (unsigned long)op.bytes;
            }
        }
        ogg_sync_clear(&oy);
        ogg_stream_clear(&os);
    }
    free(buf);
    printf("decoded checksum=%016lx\n", checksum);
}

/* OPERATION 3 — COV: exercise the oggpack/oggpackB bit-packing API (bitwise.c, 0%
 * before) + framing page accessors + stream/sync reset/check so fn coverage ≥85%.
 * Deterministic; C and Rust MUST print identical stdout. */
static long pack_roundtrip(int big) {
    oggpack_buffer w;
    if (big) oggpackB_writeinit(&w); else oggpack_writeinit(&w);
    for (int i = 0; i < 50; i++) {
        if (big) oggpackB_write(&w, (unsigned long)(i * 2654435761u), (i % 24) + 1);
        else oggpack_write(&w, (unsigned long)(i * 2654435761u), (i % 24) + 1);
    }
    if (big) { oggpackB_writealign(&w); } else { oggpack_writealign(&w); }
    long chk = big ? oggpackB_writecheck(&w) : oggpack_writecheck(&w);
    long bits = big ? oggpackB_bits(&w) : oggpack_bits(&w);
    long bytes = big ? oggpackB_bytes(&w) : oggpack_bytes(&w);
    unsigned char *buf = big ? oggpackB_get_buffer(&w) : oggpack_get_buffer(&w);

    oggpack_buffer c;
    if (big) { oggpackB_writeinit(&c); oggpackB_writecopy(&c, buf, bits); }
    else { oggpack_writeinit(&c); oggpack_writecopy(&c, buf, bits); }
    if (big) oggpackB_writetrunc(&c, 16); else oggpack_writetrunc(&c, 16);
    if (big) oggpackB_writeclear(&c); else oggpack_writeclear(&c);

    oggpack_buffer r;
    if (big) oggpackB_readinit(&r, buf, (int)bytes); else oggpack_readinit(&r, buf, (int)bytes);
    long acc = 0;
    acc += big ? oggpackB_read(&r, 5) : oggpack_read(&r, 5);
    acc += big ? oggpackB_look(&r, 8) : oggpack_look(&r, 8);
    if (big) oggpackB_adv(&r, 8); else oggpack_adv(&r, 8);
    acc += big ? oggpackB_read1(&r) : oggpack_read1(&r);
    acc += big ? oggpackB_look1(&r) : oggpack_look1(&r);
    if (big) oggpackB_adv1(&r); else oggpack_adv1(&r);
    if (big) oggpackB_reset(&w); else oggpack_reset(&w);
    if (big) oggpackB_writeclear(&w); else oggpack_writeclear(&w);
    return chk + bits + bytes + acc;
}

static void cov_workload(void) {
    long p1 = pack_roundtrip(0);
    long p2 = pack_roundtrip(1);

    /* framing accessors on a real page */
    ogg_stream_state os;
    memset(&os, 0, sizeof(os));
    ogg_stream_init(&os, 0x1234);
    unsigned char payload[64];
    for (int i = 0; i < 64; i++) payload[i] = (unsigned char)(i * 7 + 3);
    ogg_packet op;
    op.packet = payload; op.bytes = 64; op.b_o_s = 1; op.e_o_s = 0;
    op.granulepos = 12345; op.packetno = 0;
    ogg_stream_packetin(&os, &op);
    ogg_packet peek;
    int havepeek = ogg_stream_packetpeek(&os, &peek);
    ogg_page og;
    int flushed = ogg_stream_flush(&os, &og);
    int bos = ogg_page_bos(&og), eos = ogg_page_eos(&og), cont = ogg_page_continued(&og);
    long sn = ogg_page_serialno(&og), pn = ogg_page_pageno(&og);
    int ver = ogg_page_version(&og), pk = ogg_page_packets(&og);
    ogg_int64_t gp = ogg_page_granulepos(&og);
    int chk = ogg_stream_check(&os), streos = ogg_stream_eos(&os);

    /* sync side — must read og.header/body BEFORE resetting/clearing os */
    ogg_sync_state oy;
    ogg_sync_init(&oy);
    int syc = ogg_sync_check(&oy);
    ogg_sync_reset(&oy);
    char *sb = ogg_sync_buffer(&oy, (long)(og.header_len + og.body_len));
    memcpy(sb, og.header, og.header_len);
    memcpy(sb + og.header_len, og.body, og.body_len);
    ogg_sync_wrote(&oy, og.header_len + og.body_len);
    ogg_page sog;
    long seek = ogg_sync_pageseek(&oy, &sog);
    ogg_sync_clear(&oy);

    ogg_stream_reset(&os);
    ogg_stream_reset_serialno(&os, 0x5678);
    ogg_stream_clear(&os);

    /* ogg_packet_clear frees op->packet — use a heap-allocated packet, not peek */
    ogg_packet pc;
    memset(&pc, 0, sizeof(pc));
    pc.packet = (unsigned char *)malloc(8);
    pc.bytes = 8;
    ogg_packet_clear(&pc);
    (void)peek;

    printf("cov p1=%ld p2=%ld peek=%d flush=%d bos=%d eos=%d cont=%d sn=%ld pn=%ld ver=%d "
           "pk=%d gp=%lld chk=%d streos=%d syc=%d seek=%ld\n",
           p1, p2, havepeek, flushed, bos, eos, cont, sn, pn, ver, pk, (long long)gp,
           chk, streos, syc, seek);
}

int main(int argc, char *argv[]) {
    const char *mode = (argc > 1) ? argv[1] : "";
    if (strcmp(mode, "decode") == 0) decode();
    else if (strcmp(mode, "cov") == 0) cov_workload();
    else encode();
    return 0;
}
