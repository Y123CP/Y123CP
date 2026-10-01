/*
 * validation driver for miniz — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves;
 * byte-identical across versions.
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     compress        : mz_compress2 level 6 (zlib one-shot)
 *     uncompress      : mz_uncompress of a .z file (u64 LE orig size + zlib)
 *     deflate_stream  : chunked mz_deflate (64KB in / 64KB out windows)
 *     inflate_stream  : chunked mz_inflate of a .z file
 *     tdefl_matrix    : low-level tdefl_compress_mem_to_heap over a flag
 *                       matrix (probes / greedy / RLE / raw-vs-zlib)
 *     tinfl_lowlevel  : tinfl_decompress_mem_to_heap of a .defl file
 *                       (u64 LE orig size + raw deflate)
 *     checksums       : mz_crc32 + mz_adler32 over the input
 *     zip_write       : build an in-memory ZIP from 64KB slices of the input
 *     zip_read        : stat + locate + extract every entry of a .zip file
 *     zip_file        : file-based writer + reader entry points (page-cached)
 *   driver gen_z    <input> <out.z>     u64 LE orig size + zlib stream
 *   driver gen_defl <input> <out.defl>  u64 LE orig size + raw deflate stream
 *   driver gen_zip  <input> <out.zip>   ZIP with 64KB slices of the input
 *
 * Output: ONE digest line on stdout after the timing loop (same convention
 * as the lz4/lodepng drivers).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "miniz.h"

/* 4-lane xor-rotate digest (project convention; see lz4 driver) */
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

#define SLICE 65536

/* fixed archive timestamp: miniz's add_mem stamps time(NULL) into ZIP
 * headers, which breaks cross-version byte equivalence -- always use the
 * _ex_v2 entry point with this deterministic time instead */
static MZ_TIME_T FIXED_TIME = (MZ_TIME_T)1750000000;

static int zip_add_slice(mz_zip_archive *zip, const char *name,
                         const void *data, size_t size, unsigned level) {
    return mz_zip_writer_add_mem_ex_v2(zip, name, data, size, NULL, 0, level,
                                       0, 0, &FIXED_TIME, NULL, 0, NULL, 0);
}

/* -------------------------------------------------------------------- */

static void run_compress(const uint8_t *src, size_t n, long iters) {
    mz_ulong bound = mz_compressBound((mz_ulong)n);
    unsigned char *dst = malloc(bound);
    if (!dst) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    mz_ulong outlen = 0;
    for (long i = 0; i < iters; i++) {
        outlen = bound;
        /* alternate the two equivalent one-shot entry points (same output) */
        int r = (i & 1) ? mz_compress(dst, &outlen, src, (mz_ulong)n)
                        : mz_compress2(dst, &outlen, src, (mz_ulong)n, MZ_DEFAULT_LEVEL);
        if (r != MZ_OK) { fprintf(stderr, "compress failed: %d\n", r); exit(3); }
        h = digest64(h, dst, (size_t)outlen);
    }
    printf("op=compress in=%zu out=%lu iters=%ld digest=%016llx\n",
           n, (unsigned long)outlen, iters, (unsigned long long)h);
    free(dst);
}

/* .z file: u64 LE orig size + zlib stream */
static void run_uncompress(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad .z input\n"); exit(2); }
    uint64_t orig = 0;
    memcpy(&orig, file, 8);
    unsigned char *dst = malloc((size_t)orig);
    if (!dst) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        mz_ulong dlen = (mz_ulong)orig;
        int r = mz_uncompress(dst, &dlen, file + 8, (mz_ulong)(file_n - 8));
        if (r != MZ_OK || dlen != (mz_ulong)orig) { fprintf(stderr, "uncompress failed: %d\n", r); exit(3); }
        h = digest64(h, dst, (size_t)dlen);
    }
    printf("op=uncompress in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)orig, iters, (unsigned long long)h);
    free(dst);
}

static void run_deflate_stream(const uint8_t *src, size_t n, long iters) {
    unsigned char *out = malloc(SLICE);
    if (!out) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0, total = 0;
    mz_stream s;
    memset(&s, 0, sizeof(s));
    if (mz_deflateInit(&s, MZ_DEFAULT_LEVEL) != MZ_OK) { fprintf(stderr, "deflateInit failed\n"); exit(3); }
    for (long i = 0; i < iters; i++) {
        if (mz_deflateReset(&s) != MZ_OK) { fprintf(stderr, "deflateReset failed\n"); exit(3); }
        total = 0;
        size_t pos = 0;
        int flush = MZ_NO_FLUSH;
        do {
            size_t chunk = (n - pos < SLICE) ? (n - pos) : SLICE;
            s.next_in = src + pos;
            s.avail_in = (unsigned)chunk;
            pos += chunk;
            flush = (pos >= n) ? MZ_FINISH : MZ_NO_FLUSH;
            do {
                s.next_out = out;
                s.avail_out = SLICE;
                int r = mz_deflate(&s, flush);
                if (r != MZ_OK && r != MZ_STREAM_END) { fprintf(stderr, "deflate failed: %d\n", r); exit(3); }
                size_t got = SLICE - s.avail_out;
                h = digest64(h, out, got);
                total += got;
            } while (s.avail_out == 0);
        } while (flush != MZ_FINISH);
    }
    mz_deflateEnd(&s);
    printf("op=deflate_stream in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)total, iters, (unsigned long long)h);
    free(out);
}

static void run_inflate_stream(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad .z input\n"); exit(2); }
    uint64_t orig = 0;
    memcpy(&orig, file, 8);
    unsigned char *out = malloc(SLICE);
    if (!out) { fprintf(stderr, "oom\n"); exit(2); }
    uint64_t h = 0, total = 0;
    mz_stream s;
    memset(&s, 0, sizeof(s));
    if (mz_inflateInit(&s) != MZ_OK) { fprintf(stderr, "inflateInit failed\n"); exit(3); }
    for (long i = 0; i < iters; i++) {
        if (mz_inflateReset(&s) != MZ_OK) { fprintf(stderr, "inflateReset failed\n"); exit(3); }
        total = 0;
        s.next_in = file + 8;
        s.avail_in = (unsigned)(file_n - 8);
        int r;
        do {
            s.next_out = out;
            s.avail_out = SLICE;
            r = mz_inflate(&s, MZ_NO_FLUSH);
            if (r != MZ_OK && r != MZ_STREAM_END) { fprintf(stderr, "inflate failed: %d\n", r); exit(3); }
            size_t got = SLICE - s.avail_out;
            h = digest64(h, out, got);
            total += got;
        } while (r != MZ_STREAM_END);
        if (total != orig) { fprintf(stderr, "inflate size mismatch\n"); exit(3); }
    }
    mz_inflateEnd(&s);
    printf("op=inflate_stream in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)total, iters, (unsigned long long)h);
    free(out);
}

/* low-level tdefl over a flag matrix: probe counts, greedy, RLE, zlib header */
static void run_tdefl_matrix(const uint8_t *src, size_t n, long iters) {
    static const int flag_sets[] = {
        1,                                            /* 1 probe, fastest */
        128 | TDEFL_GREEDY_PARSING_FLAG,              /* greedy, mid probes */
        TDEFL_WRITE_ZLIB_HEADER | 256,                /* zlib wrapper */
        TDEFL_RLE_MATCHES | 1,                        /* RLE-only matches */
        TDEFL_FORCE_ALL_STATIC_BLOCKS | 64,           /* static-huffman blocks */
    };
    uint64_t h = 0, total = 0;
    for (long i = 0; i < iters; i++) {
        total = 0;
        for (size_t k = 0; k < 5; k++) {
            size_t outlen = 0;
            void *out = tdefl_compress_mem_to_heap(src, n, &outlen, (int)flag_sets[k]);
            if (!out) { fprintf(stderr, "tdefl failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)out, outlen);
            total += outlen;
            mz_free(out);
        }
        /* fixed-buffer variant (same first flag set, output must fit) */
        {
            unsigned char *fixed = malloc(n + 1024);
            size_t got = tdefl_compress_mem_to_mem(fixed, n + 1024, src, n, 1);
            if (!got) { fprintf(stderr, "tdefl mem_to_mem failed\n"); exit(3); }
            h = digest64(h, fixed, got);
            free(fixed);
        }
        /* heap compressor lifecycle + status query */
        {
            tdefl_compressor *c = tdefl_compressor_alloc();
            if (!c) { fprintf(stderr, "tdefl alloc failed\n"); exit(3); }
            tdefl_init(c, NULL, NULL, 1);
            uint64_t st = (uint64_t)tdefl_get_prev_return_status(c);
            h = digest64(h, (const uint8_t *)&st, sizeof(st));
            tdefl_compressor_free(c);
        }
        /* PNG writer (tdefl-based, deterministic from input bytes) */
        {
            size_t png_n = 0;
            void *png = tdefl_write_image_to_png_file_in_memory_ex(src, 64, 64, 3, &png_n, 6, 0);
            if (!png) { fprintf(stderr, "tdefl png failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)png, png_n);
            mz_free(png);
            size_t png2_n = 0;
            void *png2 = tdefl_write_image_to_png_file_in_memory(src, 64, 64, 3, &png2_n);
            if (!png2) { fprintf(stderr, "tdefl png2 failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)png2, png2_n);
            mz_free(png2);
        }
    }
    printf("op=tdefl_matrix in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)total, iters, (unsigned long long)h);
}

static int tinfl_count_cb(const void *pBuf, int len, void *pUser) {
    (void)pBuf;
    *(uint64_t *)pUser += (uint64_t)len;
    return 1;
}

/* mz_file_write_func-shaped counter for the extract_to_callback variants */
static size_t zip_count_cb(void *pUser, mz_uint64 ofs, const void *pBuf, size_t n) {
    (void)ofs; (void)pBuf;
    *(uint64_t *)pUser += (uint64_t)n;
    return n;
}

/* mz_file_read_func-shaped memory reader for add_read_buf_callback */
struct mem_reader { const uint8_t *p; size_t n; };
static size_t zip_read_cb(void *pUser, mz_uint64 ofs, void *pBuf, size_t n) {
    struct mem_reader *m = (struct mem_reader *)pUser;
    if (ofs >= m->n) return 0;
    size_t avail = m->n - (size_t)ofs;
    if (n > avail) n = avail;
    memcpy(pBuf, m->p + ofs, n);
    return n;
}

/* .defl file: u64 LE orig size + raw deflate stream */
static void run_tinfl_lowlevel(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad .defl input\n"); exit(2); }
    uint64_t orig = 0;
    memcpy(&orig, file, 8);
    uint64_t h = 0;
    unsigned char *fixed = malloc((size_t)orig);
    if (!fixed) { fprintf(stderr, "oom\n"); exit(2); }
    for (long i = 0; i < iters; i++) {
        size_t outlen = 0;
        void *out = tinfl_decompress_mem_to_heap(file + 8, file_n - 8, &outlen, 0);
        if (!out || outlen != orig) { fprintf(stderr, "tinfl failed\n"); exit(3); }
        h = digest64(h, (const uint8_t *)out, outlen);
        mz_free(out);
        /* fixed-buffer variant */
        size_t got = tinfl_decompress_mem_to_mem(fixed, (size_t)orig, file + 8, file_n - 8, 0);
        if (got != (size_t)orig) { fprintf(stderr, "tinfl mem_to_mem failed\n"); exit(3); }
        h = digest64(h, fixed, got);
    }
    /* callback variant + decompressor lifecycle (once; folds via counter) */
    {
        uint64_t cb_total = 0;
        size_t in_n = file_n - 8;
        if (!tinfl_decompress_mem_to_callback(file + 8, &in_n, tinfl_count_cb, &cb_total, 0)) {
            fprintf(stderr, "tinfl callback failed\n"); exit(3);
        }
        h = digest64(h, (const uint8_t *)&cb_total, sizeof(cb_total));
        tinfl_decompressor *d = tinfl_decompressor_alloc();
        if (!d) { fprintf(stderr, "tinfl alloc failed\n"); exit(3); }
        tinfl_decompressor_free(d);
    }
    free(fixed);
    printf("op=tinfl_lowlevel in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)orig, iters, (unsigned long long)h);
}

static void run_checksums(const uint8_t *src, size_t n, long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        uint64_t acc[3];
        acc[0] = mz_crc32(MZ_CRC32_INIT, src, n);
        acc[1] = mz_adler32(MZ_ADLER32_INIT, src, n);
        acc[2] = strlen(mz_error(MZ_MEM_ERROR));
        h = digest64(h, (const uint8_t *)acc, sizeof(acc));
    }
    printf("op=checksums in=%zu out=0 iters=%ld digest=%016llx\n",
           n, iters, (unsigned long long)h);
}

/* build an in-memory ZIP from 64KB slices of the input, then re-read it */
static void run_zip_write(const uint8_t *src, size_t n, long iters) {
    uint64_t h = 0;
    size_t zipsize = 0;
    for (long i = 0; i < iters; i++) {
        mz_zip_archive zip;
        mz_zip_zero_struct(&zip);
        if (!mz_zip_writer_init_heap(&zip, 0, 1 << 16)) { fprintf(stderr, "zip init failed\n"); exit(3); }
        char name[64];
        unsigned idx = 0;
        for (size_t off = 0; off < n; off += SLICE, idx++) {
            size_t chunk = (n - off < SLICE) ? (n - off) : SLICE;
            snprintf(name, sizeof(name), "slice_%04u.bin", idx);
            if (!zip_add_slice(&zip, name, src + off, chunk, MZ_DEFAULT_LEVEL)) {
                fprintf(stderr, "zip add failed\n"); exit(3);
            }
        }
        void *buf = NULL;
        if (!mz_zip_writer_finalize_heap_archive(&zip, &buf, &zipsize)) { fprintf(stderr, "zip finalize failed\n"); exit(3); }
        mz_zip_writer_end(&zip);
        h = digest64(h, (const uint8_t *)buf, zipsize);
        mz_free(buf);
    }
    printf("op=zip_write in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, zipsize, iters, (unsigned long long)h);
}

/* .zip file: stat + locate + extract every entry */
static void run_zip_read(const uint8_t *file, size_t file_n, long iters) {
    uint64_t h = 0, total = 0;
    for (long i = 0; i < iters; i++) {
        mz_zip_archive zip;
        mz_zip_zero_struct(&zip);
        if (!mz_zip_reader_init_mem(&zip, file, file_n, 0)) { fprintf(stderr, "zip reader init failed\n"); exit(3); }
        unsigned nfiles = mz_zip_reader_get_num_files(&zip);
        total = 0;
        for (unsigned f = 0; f < nfiles; f++) {
            mz_zip_archive_file_stat st;
            if (!mz_zip_reader_file_stat(&zip, f, &st)) { fprintf(stderr, "zip stat failed\n"); exit(3); }
            uint64_t meta[3] = { st.m_uncomp_size, st.m_comp_size, st.m_crc32 };
            h = digest64(h, (const uint8_t *)meta, sizeof(meta));
            if (mz_zip_reader_is_file_a_directory(&zip, f)) continue;
            int loc = mz_zip_reader_locate_file(&zip, st.m_filename, NULL, 0);
            if (loc != (int)f) { fprintf(stderr, "zip locate mismatch\n"); exit(3); }
            size_t outlen = 0;
            void *out = mz_zip_reader_extract_to_heap(&zip, f, &outlen, 0);
            if (!out) { fprintf(stderr, "zip extract failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)out, outlen);
            total += outlen;
            mz_free(out);
        }

        /* archive-level queries + error surface */
        {
            uint64_t q[8];
            q[0] = mz_zip_get_archive_size(&zip);
            q[1] = mz_zip_get_central_dir_size(&zip);
            q[2] = mz_zip_get_archive_file_start_offset(&zip);
            q[3] = (uint64_t)mz_zip_is_zip64(&zip);
            q[4] = (uint64_t)mz_zip_get_mode(&zip) | ((uint64_t)mz_zip_get_type(&zip) << 8);
            q[5] = (uint64_t)mz_zip_peek_last_error(&zip);
            q[6] = (uint64_t)mz_zip_clear_last_error(&zip);
            q[7] = (uint64_t)mz_zip_get_last_error(&zip)
                 + strlen(mz_zip_get_error_string(MZ_ZIP_UNDEFINED_ERROR));
            h = digest64(h, (const uint8_t *)q, sizeof(q));
            (void)mz_zip_get_cfile(&zip);
            unsigned char hdr[64];
            size_t got = mz_zip_read_archive_data(&zip, 0, hdr, sizeof(hdr));
            h = digest64(h, hdr, got);
            if (!mz_zip_validate_archive(&zip, 0)) { fprintf(stderr, "zip validate failed\n"); exit(3); }
        }

        /* entry-0 extract variants (all must agree with extract_to_heap) */
        if (nfiles > 0) {
            mz_zip_archive_file_stat st0;
            mz_zip_reader_file_stat(&zip, 0, &st0);
            char fname[260];
            mz_zip_reader_get_filename(&zip, 0, fname, sizeof(fname));
            size_t sz0 = (size_t)st0.m_uncomp_size;
            unsigned char *buf0 = malloc(sz0);
            if (!mz_zip_reader_extract_to_mem(&zip, 0, buf0, sz0, 0)) { fprintf(stderr, "extract_to_mem failed\n"); exit(3); }
            h = digest64(h, buf0, sz0);
            if (!mz_zip_reader_extract_file_to_mem(&zip, fname, buf0, sz0, 0)) { fprintf(stderr, "extract_file_to_mem failed\n"); exit(3); }
            h = digest64(h, buf0, sz0);
            if (!mz_zip_reader_extract_to_mem_no_alloc(&zip, 0, buf0, sz0, 0, NULL, 0)) { fprintf(stderr, "no_alloc failed\n"); exit(3); }
            if (!mz_zip_reader_extract_file_to_mem_no_alloc(&zip, fname, buf0, sz0, 0, NULL, 0)) { fprintf(stderr, "file no_alloc failed\n"); exit(3); }
            size_t hn = 0;
            void *hp = mz_zip_reader_extract_file_to_heap(&zip, fname, &hn, 0);
            if (!hp) { fprintf(stderr, "extract_file_to_heap failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)hp, hn);
            mz_free(hp);
            uint64_t cbt = 0;
            if (!mz_zip_reader_extract_to_callback(&zip, 0, zip_count_cb, &cbt, 0)) { fprintf(stderr, "extract cb failed\n"); exit(3); }
            if (!mz_zip_reader_extract_file_to_callback(&zip, fname, zip_count_cb, &cbt, 0)) { fprintf(stderr, "extract file cb failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)&cbt, sizeof(cbt));
            mz_zip_reader_extract_iter_state *it = mz_zip_reader_extract_iter_new(&zip, 0, 0);
            if (!it) { fprintf(stderr, "iter new failed\n"); exit(3); }
            uint64_t itot = 0;
            unsigned char ib[4096];
            size_t r;
            while ((r = mz_zip_reader_extract_iter_read(it, ib, sizeof(ib))) > 0) {
                h = digest64(h, ib, r);
                itot += r;
            }
            mz_zip_reader_extract_iter_free(it);
            mz_zip_reader_extract_iter_state *it2 = mz_zip_reader_extract_file_iter_new(&zip, fname, 0);
            if (it2) { (void)mz_zip_reader_extract_iter_read(it2, ib, sizeof(ib)); mz_zip_reader_extract_iter_free(it2); }
            h = digest64(h, (const uint8_t *)&itot, sizeof(itot));
            free(buf0);
        }
        mz_zip_end(&zip);   /* generic end (reader mode) */
    }
    /* standalone validate helper over the raw archive bytes */
    {
        mz_zip_error zerr = MZ_ZIP_NO_ERROR;
        if (!mz_zip_validate_mem_archive(file, file_n, 0, &zerr)) { fprintf(stderr, "validate_mem failed\n"); exit(3); }
    }
    printf("op=zip_read in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)total, iters, (unsigned long long)h);
}

/* file/cfile/in-place/append entry points (page-cached scratch files).
 * DIGEST RULE: fold only extracted payloads and counts, never raw archive
 * bytes -- file-based writers stamp source mtimes into entry headers. */
static void run_zip_file(const uint8_t *src, size_t n, long iters,
                         const char *input_path) {
    const char *scratch  = "/tmp/miniz_vw_scratch.zip";
    const char *scratch2 = "/tmp/miniz_vw_scratch2.zip";
    const char *scratch3 = "/tmp/miniz_vw_scratch3.bin";
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        /* 1. file-based writer: slices + whole input file + read-buf callback */
        mz_zip_archive w;
        mz_zip_zero_struct(&w);
        if (!mz_zip_writer_init_file(&w, scratch, 0)) { fprintf(stderr, "zip file init failed\n"); exit(3); }
        size_t part = n / 4;
        for (unsigned k = 0; k < 4; k++) {
            char name[32];
            snprintf(name, sizeof(name), "f%u.bin", k);
            if (!zip_add_slice(&w, name, src + (size_t)k * part, part, MZ_BEST_SPEED)) {
                fprintf(stderr, "zip file add failed\n"); exit(3);
            }
        }
        if (!mz_zip_writer_add_file(&w, "whole.bin", input_path, NULL, 0, MZ_BEST_SPEED)) {
            fprintf(stderr, "zip add_file failed\n"); exit(3);
        }
        struct mem_reader mr = { src, n / 8 };
        if (!mz_zip_writer_add_read_buf_callback(&w, "cb.bin", zip_read_cb, &mr,
                                                 (mz_uint64)(n / 8), &FIXED_TIME, NULL, 0,
                                                 MZ_BEST_SPEED, NULL, 0, NULL, 0)) {
            fprintf(stderr, "zip add_read_buf failed\n"); exit(3);
        }
        if (!mz_zip_writer_finalize_archive(&w)) { fprintf(stderr, "zip file finalize failed\n"); exit(3); }
        mz_zip_writer_end(&w);

        /* 2. in-place append helpers */
        if (!mz_zip_add_mem_to_archive_file_in_place(scratch, "extra1.bin", src, 1024, NULL, 0, MZ_BEST_SPEED)) {
            fprintf(stderr, "in_place failed\n"); exit(3);
        }
        mz_zip_error zerr = MZ_ZIP_NO_ERROR;
        if (!mz_zip_add_mem_to_archive_file_in_place_v2(scratch, "extra2.bin", src, 1024, NULL, 0, MZ_BEST_SPEED, &zerr)) {
            fprintf(stderr, "in_place v2 failed\n"); exit(3);
        }

        /* 3. reader -> writer append pattern */
        {
            mz_zip_archive z;
            mz_zip_zero_struct(&z);
            if (!mz_zip_reader_init_file(&z, scratch, 0)) { fprintf(stderr, "reader init failed\n"); exit(3); }
            if (!mz_zip_writer_init_from_reader_v2(&z, scratch, 0)) { fprintf(stderr, "init_from_reader failed\n"); exit(3); }
            if (!zip_add_slice(&z, "appended.bin", src, 2048, MZ_BEST_SPEED)) { fprintf(stderr, "append failed\n"); exit(3); }
            if (!mz_zip_writer_finalize_archive(&z)) { fprintf(stderr, "append finalize failed\n"); exit(3); }
            mz_zip_writer_end(&z);
        }

        /* 4. read back: file reader + extract to file / heap-by-name helpers */
        {
            mz_zip_archive r;
            mz_zip_zero_struct(&r);
            if (!mz_zip_reader_init_file(&r, scratch, 0)) { fprintf(stderr, "zip file reader failed\n"); exit(3); }
            unsigned nf = mz_zip_reader_get_num_files(&r);
            for (unsigned f = 0; f < nf; f++) {
                size_t outlen = 0;
                void *out = mz_zip_reader_extract_to_heap(&r, f, &outlen, 0);
                if (!out) { fprintf(stderr, "zip file extract failed\n"); exit(3); }
                h = digest64(h, (const uint8_t *)out, outlen);
                mz_free(out);
            }
            if (!mz_zip_reader_extract_to_file(&r, 0, scratch3, 0)) { fprintf(stderr, "extract_to_file failed\n"); exit(3); }
            if (!mz_zip_reader_extract_file_to_file(&r, "f1.bin", scratch3, 0)) { fprintf(stderr, "extract_file_to_file failed\n"); exit(3); }
            size_t rn = 0;
            uint8_t *rb = read_file(scratch3, &rn);
            h = digest64(h, rb, rn);
            free(rb);
            FILE *devnull = fopen("/dev/null", "wb");
            if (devnull) {
                if (!mz_zip_reader_extract_to_cfile(&r, 0, devnull, 0)) { fprintf(stderr, "extract_to_cfile failed\n"); exit(3); }
                if (!mz_zip_reader_extract_file_to_cfile(&r, "f1.bin", devnull, 0)) { fprintf(stderr, "extract_file_to_cfile failed\n"); exit(3); }
                fclose(devnull);
            }
            mz_zip_reader_end(&r);
        }
        {
            size_t hn = 0;
            void *hp = mz_zip_extract_archive_file_to_heap(scratch, "f2.bin", &hn, 0);
            if (!hp) { fprintf(stderr, "archive_file_to_heap failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)hp, hn);
            mz_free(hp);
            mz_zip_error e2 = MZ_ZIP_NO_ERROR;
            void *hp2 = mz_zip_extract_archive_file_to_heap_v2(scratch, "f3.bin", NULL, &hn, 0, &e2);
            if (!hp2) { fprintf(stderr, "archive_file_to_heap_v2 failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)hp2, hn);
            mz_free(hp2);
            if (!mz_zip_validate_file_archive(scratch, 0, &e2)) { fprintf(stderr, "validate_file failed\n"); exit(3); }
        }

        /* 5. cfile writer + reader */
        {
            FILE *fo = fopen(scratch2, "w+b");
            FILE *fi = fopen(input_path, "rb");
            if (!fo || !fi) { fprintf(stderr, "cfile open failed\n"); exit(3); }
            mz_zip_archive cw;
            mz_zip_zero_struct(&cw);
            if (!mz_zip_writer_init_cfile(&cw, fo, 0)) { fprintf(stderr, "writer_init_cfile failed\n"); exit(3); }
            rewind(fi);
            if (!mz_zip_writer_add_cfile(&cw, "c.bin", fi, (mz_uint64)n, &FIXED_TIME, NULL, 0,
                                         MZ_BEST_SPEED, NULL, 0, NULL, 0)) {
                fprintf(stderr, "add_cfile failed: %s\n",
                        mz_zip_get_error_string(mz_zip_get_last_error(&cw)));
                exit(3);
            }
            if (!mz_zip_writer_finalize_archive(&cw)) { fprintf(stderr, "cfile finalize failed\n"); exit(3); }
            mz_uint64 written = mz_zip_get_archive_size(&cw);
            mz_zip_writer_end(&cw);
            fclose(fi);
            fseek(fo, 0, SEEK_SET);
            mz_zip_archive cr;
            mz_zip_zero_struct(&cr);
            if (!mz_zip_reader_init_cfile(&cr, fo, written, 0)) { fprintf(stderr, "reader_init_cfile failed\n"); exit(3); }
            size_t cn = 0;
            void *cp = mz_zip_reader_extract_to_heap(&cr, 0, &cn, 0);
            if (!cp) { fprintf(stderr, "cfile extract failed\n"); exit(3); }
            h = digest64(h, (const uint8_t *)cp, cn);
            mz_free(cp);
            mz_zip_reader_end(&cr);
            fclose(fo);
        }
    }
    remove(scratch); remove(scratch2); remove(scratch3);
    printf("op=zip_file in=%zu out=0 iters=%ld digest=%016llx\n",
           n, iters, (unsigned long long)h);
}

/* -------------------------------------------------------------------- */

static void gen_z(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *src = read_file(in_path, &n);
    mz_ulong bound = mz_compressBound((mz_ulong)n), outlen = 0;
    unsigned char *dst = malloc(bound);
    outlen = bound;
    if (mz_compress2(dst, &outlen, src, (mz_ulong)n, MZ_DEFAULT_LEVEL) != MZ_OK) {
        fprintf(stderr, "compress failed\n"); exit(3);
    }
    FILE *f = fopen(out_path, "wb");
    uint64_t orig = (uint64_t)n;
    fwrite(&orig, 1, 8, f);
    fwrite(dst, 1, (size_t)outlen, f);
    fclose(f);
    fprintf(stderr, "wrote %s: %llu -> %lu\n", out_path,
            (unsigned long long)orig, (unsigned long)outlen);
    free(src); free(dst);
}

static void gen_defl(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *src = read_file(in_path, &n);
    size_t outlen = 0;
    void *out = tdefl_compress_mem_to_heap(src, n, &outlen, 128);
    if (!out) { fprintf(stderr, "tdefl failed\n"); exit(3); }
    FILE *f = fopen(out_path, "wb");
    uint64_t orig = (uint64_t)n;
    fwrite(&orig, 1, 8, f);
    fwrite(out, 1, outlen, f);
    fclose(f);
    fprintf(stderr, "wrote %s: %llu -> %zu (raw deflate)\n", out_path,
            (unsigned long long)orig, outlen);
    free(src); mz_free(out);
}

static void gen_zip(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *src = read_file(in_path, &n);
    mz_zip_archive zip;
    mz_zip_zero_struct(&zip);
    if (!mz_zip_writer_init_file(&zip, out_path, 0)) { fprintf(stderr, "zip init failed\n"); exit(3); }
    char name[64];
    unsigned idx = 0;
    for (size_t off = 0; off < n; off += SLICE, idx++) {
        size_t chunk = (n - off < SLICE) ? (n - off) : SLICE;
        snprintf(name, sizeof(name), "slice_%04u.bin", idx);
        if (!zip_add_slice(&zip, name, src + off, chunk, MZ_DEFAULT_LEVEL)) {
            fprintf(stderr, "zip add failed\n"); exit(3);
        }
    }
    if (!mz_zip_writer_finalize_archive(&zip)) { fprintf(stderr, "zip finalize failed\n"); exit(3); }
    mz_zip_writer_end(&zip);
    fprintf(stderr, "wrote %s (%u slices)\n", out_path, idx);
    free(src);
}

int main(int argc, char **argv) {
    /* provenance sanity check (stderr only, stdout unaffected) */
    if (mz_version() == NULL) { fprintf(stderr, "bad miniz\n"); return 1; }
    if (argc == 4 && strcmp(argv[1], "gen_z") == 0)    { gen_z(argv[2], argv[3]); return 0; }
    if (argc == 4 && strcmp(argv[1], "gen_defl") == 0) { gen_defl(argv[2], argv[3]); return 0; }
    if (argc == 4 && strcmp(argv[1], "gen_zip") == 0)  { gen_zip(argv[2], argv[3]); return 0; }
    if (argc != 4) {
        fprintf(stderr, "usage: %s <op> <input> <iters>  |  %s gen_{z,defl,zip} <in> <out>\n",
                argv[0], argv[0]);
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }
    size_t n;
    uint8_t *buf = read_file(argv[2], &n);

    if (strcmp(op, "compress") == 0)            run_compress(buf, n, iters);
    else if (strcmp(op, "uncompress") == 0)     run_uncompress(buf, n, iters);
    else if (strcmp(op, "deflate_stream") == 0) run_deflate_stream(buf, n, iters);
    else if (strcmp(op, "inflate_stream") == 0) run_inflate_stream(buf, n, iters);
    else if (strcmp(op, "tdefl_matrix") == 0)   run_tdefl_matrix(buf, n, iters);
    else if (strcmp(op, "tinfl_lowlevel") == 0) run_tinfl_lowlevel(buf, n, iters);
    else if (strcmp(op, "checksums") == 0)      run_checksums(buf, n, iters);
    else if (strcmp(op, "zip_write") == 0)      run_zip_write(buf, n, iters);
    else if (strcmp(op, "zip_read") == 0)       run_zip_read(buf, n, iters);
    else if (strcmp(op, "zip_file") == 0)       run_zip_file(buf, n, iters, argv[2]);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    free(buf);
    return 0;
}
