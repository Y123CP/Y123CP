/*
 * validation driver for lodepng — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves;
 * byte-identical across versions.
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     decode32       : PNG -> RGBA32 (inflate + unfilter + color conversion)
 *     decode_keep    : PNG -> raw color type, no conversion (state decode);
 *                      run on palette/grey16/interlaced/meta variants to hit
 *                      Adam7 / bit-depth / readChunk_* paths
 *     decode_file    : PNG file path -> RGBA32 via lodepng_decode32_file
 *     encode32       : .rgba file -> PNG (filter + deflate)
 *     encode_meta    : .rgba -> PNG with ancillary metadata (tEXt/zTXt/iTXt,
 *                      gAMA, pHYs, tIME, bKGD, sRGB, cHRM, sBIT) + auto_convert
 *     encode_settings: .rgba -> PNG over a matrix of filter strategies x
 *                      zlib btypes (0/1/2) -- use a small tile input
 *     zlib_roundtrip : zlib compress + decompress + raw inflate of input bytes
 *     convert        : RGBA -> {RGB, grey, grey16, RGBA16} conversions
 *     chunk_crc      : chunk walk/surgery + inspect(+chunk) + crc32 + misc
 *                      color-mode/error_text utilities
 *   driver gen_rgba    <input.png> <output.rgba>   u32 LE w, h + RGBA pixels
 *   driver gen_tile    <input.rgba> <output.rgba>  crop top-left 256x256 tile
 *   driver gen_variant <input.png> <out_prefix>    write <p>_palette.png,
 *                      <p>_grey16.png, <p>_interlaced.png, <p>_meta.png
 *                      (real-image-derived variants, reference encoder)
 *
 * Output: ONE digest line on stdout after the timing loop (same convention as
 * the lz4 driver): op=<op> in=<bytes> out=<bytes> iters=<N> digest=<hex>.
 * The digest is re-seeded with the running value every iteration, so no
 * iteration can be elided.
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "lodepng.h"

/* 4-lane xor-rotate digest (same as the lz4 driver): sensitive to any byte
 * change, order-sensitive, no multiplies in the hot loop so the checksum
 * stays a small fraction of self-time (gate G4). */
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

/* PNG -> RGBA32 */
static void run_decode32(const uint8_t *png, size_t n, long iters) {
    uint64_t h = 0;
    unsigned w = 0, ht = 0;
    size_t outbytes = 0;
    for (long i = 0; i < iters; i++) {
        unsigned char *img = NULL;
        unsigned err = lodepng_decode32(&img, &w, &ht, png, n);
        if (err) { fprintf(stderr, "decode32 failed: %u\n", err); exit(3); }
        outbytes = (size_t)w * ht * 4;
        h = digest64(h, img, outbytes);
        free(img);
    }
    printf("op=decode32 in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, outbytes, iters, (unsigned long long)h);
}

/* PNG -> raw color type (no conversion), via state-based decoder */
static void run_decode_keep(const uint8_t *png, size_t n, long iters) {
    uint64_t h = 0;
    size_t outbytes = 0;
    for (long i = 0; i < iters; i++) {
        LodePNGState state;
        lodepng_state_init(&state);
        state.decoder.color_convert = 0;
        unsigned char *img = NULL;
        unsigned w = 0, ht = 0;
        unsigned err = lodepng_decode(&img, &w, &ht, &state, png, n);
        if (err) { fprintf(stderr, "decode_keep failed: %u\n", err); exit(3); }
        outbytes = lodepng_get_raw_size(w, ht, &state.info_raw);
        h = digest64(h, img, outbytes);
        free(img);
        lodepng_state_cleanup(&state);
    }
    printf("op=decode_keep in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, outbytes, iters, (unsigned long long)h);
}

/* .rgba file: u32 LE w, u32 LE h, then w*h*4 bytes of RGBA pixels */
static void run_encode32(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad rgba input\n"); exit(2); }
    uint32_t w = 0, ht = 0;
    memcpy(&w, file, 4);
    memcpy(&ht, file + 4, 4);
    if (file_n != 8 + (size_t)w * ht * 4) { fprintf(stderr, "rgba size mismatch\n"); exit(2); }
    const unsigned char *pixels = file + 8;
    uint64_t h = 0;
    size_t outsize = 0;
    for (long i = 0; i < iters; i++) {
        unsigned char *png = NULL;
        unsigned err = lodepng_encode32(&png, &outsize, pixels, w, ht);
        if (err) { fprintf(stderr, "encode32 failed: %u\n", err); exit(3); }
        h = digest64(h, png, outsize);
        free(png);
    }
    printf("op=encode32 in=%zu out=%zu iters=%ld digest=%016llx\n",
           file_n, outsize, iters, (unsigned long long)h);
}

/* parse utilities: chunk walk + inspect + crc32 over the whole file */
static void run_chunk_crc(const uint8_t *png, size_t n, long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        uint64_t acc[8] = {0};
        LodePNGState state;
        lodepng_state_init(&state);
        unsigned w = 0, ht = 0;
        unsigned err = lodepng_inspect(&w, &ht, &state, png, n);
        if (err) { fprintf(stderr, "inspect failed: %u\n", err); exit(3); }
        acc[0] = w; acc[1] = ht;
        acc[2] = state.info_png.color.colortype;
        acc[3] = state.info_png.color.bitdepth;
        lodepng_state_cleanup(&state);
        if (n > 8) {
            const unsigned char *chunk = png + 8;
            const unsigned char *end = png + n;
            while (chunk + 12 <= end) {
                unsigned len = lodepng_chunk_length(chunk);
                char type[5];
                lodepng_chunk_type(type, chunk);
                acc[4] += len;
                acc[5] += (uint64_t)lodepng_chunk_ancillary(chunk);
                acc[6] += (uint64_t)lodepng_chunk_check_crc(chunk);
                if (memcmp(type, "IEND", 4) == 0) break;
                if (chunk + 12 + (size_t)len > end) break;
                chunk = lodepng_chunk_next_const(chunk, end);
                if (!chunk) break;
            }
        }
        acc[7] = lodepng_crc32(png, n);
        h = digest64(h, (const uint8_t *)acc, sizeof(acc));

        /* chunk surgery + misc utilities (deterministic, folded into digest) */
        uint64_t acc2[8] = {0};
        const unsigned char *end = png + n;
        const unsigned char *ihdr = lodepng_chunk_find_const(png + 8, end, "IHDR");
        if (ihdr) {
            acc2[0] = lodepng_chunk_length(ihdr);
            acc2[1] = (uint64_t)lodepng_chunk_private(ihdr)
                    | ((uint64_t)lodepng_chunk_safetocopy(ihdr) << 1);
            LodePNGState st2;
            lodepng_state_init(&st2);
            unsigned w2 = 0, h2 = 0;
            lodepng_inspect(&w2, &h2, &st2, png, n);
            /* inspect_chunk on the first chunk after IHDR */
            const unsigned char *nxt = lodepng_chunk_next_const(ihdr, end);
            if (nxt && nxt < end) (void)lodepng_inspect_chunk(&st2, (size_t)(nxt - png), png, n);
            LodePNGState st3;
            lodepng_state_init(&st3);
            lodepng_state_copy(&st3, &st2);
            acc2[2] = st3.info_png.color.colortype;
            lodepng_state_cleanup(&st3);
            lodepng_state_cleanup(&st2);
        }
        /* build a tiny chunk buffer: create + append + find + mutate */
        {
            unsigned char *buf2 = NULL; size_t bn = 0;
            unsigned char payload[4] = { 1, 2, 3, 4 };
            if (!lodepng_chunk_create(&buf2, &bn, 4, "tEXt", payload) && buf2) {
                unsigned char *c = buf2;
                acc2[3] = lodepng_chunk_length(c);
                acc2[4] = (uint64_t)lodepng_chunk_type_equals(c, "tEXt")
                        | ((uint64_t)lodepng_chunk_ancillary(c) << 2);
                unsigned char *data = lodepng_chunk_data(c);
                if (data) acc2[5] = data[0];
                unsigned char *find = lodepng_chunk_find(buf2, buf2 + bn, "tEXt");
                acc2[6] = (find == buf2);
                if (find) { unsigned char *nx = lodepng_chunk_next(find, buf2 + bn); (void)nx; }
                lodepng_chunk_generate_crc(c);
                acc2[7] = (uint64_t)lodepng_chunk_check_crc(c);
                (void)lodepng_chunk_append(&buf2, &bn, c);
                free(buf2);
            }
        }
        h = digest64(h, (const uint8_t *)acc2, sizeof(acc2));

        /* color-mode + error-text helpers */
        uint64_t acc3[6] = {0};
        LodePNGColorMode pal = lodepng_color_mode_make(LCT_PALETTE, 8);
        for (unsigned k = 0; k < 4; k++)
            lodepng_palette_add(&pal, (unsigned char)k, (unsigned char)(k * 2),
                                (unsigned char)(k * 3), 255);
        acc3[0] = lodepng_get_bpp(&pal);
        acc3[1] = lodepng_get_channels(&pal);
        acc3[2] = (uint64_t)lodepng_is_palette_type(&pal)
                | ((uint64_t)lodepng_has_palette_alpha(&pal) << 1)
                | ((uint64_t)lodepng_is_alpha_type(&pal) << 2)
                | ((uint64_t)lodepng_is_greyscale_type(&pal) << 3);
        LodePNGColorMode palcopy;
        lodepng_color_mode_init(&palcopy);
        lodepng_color_mode_copy(&palcopy, &pal);
        acc3[3] = lodepng_can_have_alpha(&palcopy);
        lodepng_color_mode_cleanup(&palcopy);
        lodepng_color_mode_cleanup(&pal);
        uint64_t elen = 0;
        for (unsigned e = 0; e < 96; e++) elen += strlen(lodepng_error_text(e));
        acc3[4] = elen;
        h = digest64(h, (const uint8_t *)acc3, sizeof(acc3));
    }
    printf("op=chunk_crc in=%zu out=0 iters=%ld digest=%016llx\n",
           n, iters, (unsigned long long)h);
}

/* PNG file path -> RGBA32 via the _file entry points (real API surface;
 * the file is page-cached so I/O adds negligible time vs decode) */
static void run_decode_file(const char *path, long iters) {
    uint64_t h = 0;
    unsigned w = 0, ht = 0;
    size_t outbytes = 0;
    for (long i = 0; i < iters; i++) {
        unsigned char *img = NULL;
        unsigned err = lodepng_decode32_file(&img, &w, &ht, path);
        if (err) { fprintf(stderr, "decode32_file failed: %u\n", err); exit(3); }
        outbytes = (size_t)w * ht * 4;
        h = digest64(h, img, outbytes);
        free(img);
    }
    printf("op=decode_file in=0 out=%zu iters=%ld digest=%016llx\n",
           outbytes, iters, (unsigned long long)h);
}

static void set_meta(LodePNGState *state) {
    LodePNGInfo *info = &state->info_png;
    lodepng_add_text(info, "Title", "validation workload");
    lodepng_add_text(info, "Author", "reference encoder");
    lodepng_add_itext(info, "Comment", "en", "comment", "three-way perf validation");
    info->gama_defined = 1; info->gama_gamma = 45455;
    info->phys_defined = 1; info->phys_x = 2835; info->phys_y = 2835; info->phys_unit = 1;
    info->time_defined = 1;
    info->time.year = 2026; info->time.month = 7; info->time.day = 16;
    info->time.hour = 12; info->time.minute = 0; info->time.second = 0;
    info->background_defined = 1;
    info->background_r = 255; info->background_g = 255; info->background_b = 255;
    info->srgb_defined = 1; info->srgb_intent = 0;
    info->chrm_defined = 1;
    info->chrm_white_x = 31270; info->chrm_white_y = 32900;
    info->chrm_red_x = 64000; info->chrm_red_y = 33000;
    info->chrm_green_x = 30000; info->chrm_green_y = 60000;
    info->chrm_blue_x = 15000; info->chrm_blue_y = 6000;
    info->sbit_defined = 1; info->sbit_r = 8; info->sbit_g = 8; info->sbit_b = 8; info->sbit_a = 8;
}

/* .rgba -> PNG carrying the common ancillary chunks (zTXt via
 * text_compression) with auto color-model selection */
static void run_encode_meta(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad rgba input\n"); exit(2); }
    uint32_t w = 0, ht = 0;
    memcpy(&w, file, 4);
    memcpy(&ht, file + 4, 4);
    const unsigned char *pixels = file + 8;
    uint64_t h = 0;
    size_t outsize = 0;
    for (long i = 0; i < iters; i++) {
        /* two encodes: text_compression 1 -> zTXt path, 0 -> tEXt path */
        for (unsigned tc = 0; tc < 2; tc++) {
            LodePNGState state;
            lodepng_state_init(&state);
            set_meta(&state);
            state.encoder.text_compression = tc;
            state.encoder.auto_convert = 1;       /* color stats + model selection */
            state.encoder.filter_strategy = LFS_MINSUM;
            unsigned char *png = NULL;
            unsigned err = lodepng_encode(&png, &outsize, pixels, w, ht, &state);
            if (err) { fprintf(stderr, "encode_meta failed: %u\n", err); exit(3); }
            h = digest64(h, png, outsize);
            free(png);
            lodepng_clear_text(&state.info_png);
            lodepng_clear_itext(&state.info_png);
            lodepng_state_cleanup(&state);
        }
    }
    printf("op=encode_meta in=%zu out=%zu iters=%ld digest=%016llx\n",
           file_n, outsize, iters, (unsigned long long)h);
}

/* .rgba -> PNG over a settings matrix: filter strategies x zlib btypes.
 * btype 0/1 exercise the stored / fixed-huffman deflate paths. */
static void run_encode_settings(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad rgba input\n"); exit(2); }
    uint32_t w = 0, ht = 0;
    memcpy(&w, file, 4);
    memcpy(&ht, file + 4, 4);
    const unsigned char *pixels = file + 8;
    static const LodePNGFilterStrategy strategies[] =
        { LFS_ZERO, LFS_MINSUM, LFS_ENTROPY, LFS_BRUTE_FORCE };
    static const unsigned btypes[] = { 0, 1, 2 };
    uint64_t h = 0, total = 0;
    for (long i = 0; i < iters; i++) {
        total = 0;
        for (size_t s = 0; s < 4; s++) {
            for (size_t b = 0; b < 3; b++) {
                LodePNGState state;
                lodepng_state_init(&state);
                state.encoder.filter_strategy = strategies[s];
                state.encoder.zlibsettings.btype = btypes[b];
                unsigned char *png = NULL;
                size_t outsize = 0;
                unsigned err = lodepng_encode(&png, &outsize, pixels, w, ht, &state);
                if (err) { fprintf(stderr, "encode_settings failed: %u\n", err); exit(3); }
                h = digest64(h, png, outsize);
                total += outsize;
                free(png);
                lodepng_state_cleanup(&state);
            }
        }
    }
    printf("op=encode_settings in=%zu out=%llu iters=%ld digest=%016llx\n",
           file_n, (unsigned long long)total, iters, (unsigned long long)h);
}

/* zlib compress + decompress + raw-deflate inflate over the input bytes */
static void run_zlib_roundtrip(const uint8_t *buf, size_t n, long iters) {
    LodePNGCompressSettings cs;
    lodepng_compress_settings_init(&cs);
    LodePNGDecompressSettings ds;
    lodepng_decompress_settings_init(&ds);
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        unsigned char *z = NULL; size_t zn = 0;
        if (lodepng_zlib_compress(&z, &zn, buf, n, &cs)) { fprintf(stderr, "zlib compress failed\n"); exit(3); }
        h = digest64(h, z, zn);
        unsigned char *back = NULL; size_t bn = 0;
        if (lodepng_zlib_decompress(&back, &bn, z, zn, &ds) || bn != n) {
            fprintf(stderr, "zlib decompress failed\n"); exit(3);
        }
        h = digest64(h, back, bn);
        /* raw deflate stream = zlib payload without the 2-byte header */
        unsigned char *raw = NULL; size_t rn = 0;
        if (lodepng_inflate(&raw, &rn, z + 2, zn - 2, &ds) || rn != n) {
            fprintf(stderr, "inflate failed\n"); exit(3);
        }
        h = digest64(h, raw, rn);
        free(z); free(back); free(raw);
    }
    printf("op=zlib_roundtrip in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, n, iters, (unsigned long long)h);
}

/* RGBA -> RGB / grey / grey16 / RGBA16 conversions (lodepng_convert) */
static void run_convert(const uint8_t *file, size_t file_n, long iters) {
    if (file_n < 8) { fprintf(stderr, "bad rgba input\n"); exit(2); }
    uint32_t w = 0, ht = 0;
    memcpy(&w, file, 4);
    memcpy(&ht, file + 4, 4);
    const unsigned char *pixels = file + 8;
    LodePNGColorMode in_mode;
    lodepng_color_mode_init(&in_mode);
    in_mode.colortype = LCT_RGBA; in_mode.bitdepth = 8;
    static const struct { LodePNGColorType t; unsigned d; } outs[] =
        { { LCT_RGB, 8 }, { LCT_GREY, 8 }, { LCT_GREY, 16 }, { LCT_RGBA, 16 } };
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        for (size_t k = 0; k < 4; k++) {
            LodePNGColorMode out_mode;
            lodepng_color_mode_init(&out_mode);
            out_mode.colortype = outs[k].t; out_mode.bitdepth = outs[k].d;
            size_t outbytes = lodepng_get_raw_size(w, ht, &out_mode);
            unsigned char *out = malloc(outbytes);
            if (!out) { fprintf(stderr, "oom\n"); exit(2); }
            unsigned err = lodepng_convert(out, pixels, &out_mode, &in_mode, w, ht);
            if (err) { fprintf(stderr, "convert failed: %u\n", err); exit(3); }
            h = digest64(h, out, outbytes);
            free(out);
            lodepng_color_mode_cleanup(&out_mode);
        }
    }
    printf("op=convert in=%zu out=0 iters=%ld digest=%016llx\n",
           file_n, iters, (unsigned long long)h);
}

static void gen_rgba(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *png = read_file(in_path, &n);
    unsigned char *img = NULL;
    unsigned w = 0, ht = 0;
    unsigned err = lodepng_decode32(&img, &w, &ht, png, n);
    if (err) { fprintf(stderr, "decode failed: %u\n", err); exit(3); }
    FILE *f = fopen(out_path, "wb");
    if (!f) { fprintf(stderr, "cannot open %s\n", out_path); exit(2); }
    uint32_t w32 = w, h32 = ht;
    fwrite(&w32, 1, 4, f);
    fwrite(&h32, 1, 4, f);
    fwrite(img, 1, (size_t)w * ht * 4, f);
    fclose(f);
    fprintf(stderr, "wrote %s: %ux%u RGBA\n", out_path, w, ht);
    free(img); free(png);
}

/* crop the top-left 256x256 tile of an .rgba file (encode_settings input) */
static void gen_tile(const char *in_path, const char *out_path) {
    size_t n;
    uint8_t *file = read_file(in_path, &n);
    uint32_t w = 0, ht = 0;
    memcpy(&w, file, 4);
    memcpy(&ht, file + 4, 4);
    uint32_t tw = w < 256 ? w : 256, th = ht < 256 ? ht : 256;
    FILE *f = fopen(out_path, "wb");
    if (!f) { fprintf(stderr, "cannot open %s\n", out_path); exit(2); }
    fwrite(&tw, 1, 4, f);
    fwrite(&th, 1, 4, f);
    for (uint32_t y = 0; y < th; y++)
        fwrite(file + 8 + (size_t)y * w * 4, 1, (size_t)tw * 4, f);
    fclose(f);
    fprintf(stderr, "wrote %s: %ux%u tile\n", out_path, tw, th);
    free(file);
}

/* real-image-derived variants via the reference encoder:
 * <p>_palette.png (posterized to <=64 colors), <p>_grey16.png,
 * <p>_interlaced.png (Adam7), <p>_meta.png (ancillary chunks) */
static void gen_variant(const char *in_path, const char *prefix) {
    size_t n;
    uint8_t *png = read_file(in_path, &n);
    unsigned char *img = NULL;
    unsigned w = 0, ht = 0;
    if (lodepng_decode32(&img, &w, &ht, png, n)) { fprintf(stderr, "decode failed\n"); exit(3); }
    char path[1024];
    size_t npix = (size_t)w * ht;

    /* palette: posterize to 2 bits/channel (<=64 colors), opaque */
    unsigned char *post = malloc(npix * 4);
    for (size_t i = 0; i < npix * 4; i++)
        post[i] = (i % 4 == 3) ? 255 : (unsigned char)(img[i] & 0xC0);
    {
        LodePNGState st;
        lodepng_state_init(&st);
        st.encoder.auto_convert = 1;  /* stats will pick palette */
        unsigned char *out = NULL; size_t on = 0;
        if (lodepng_encode(&out, &on, post, w, ht, &st)) { fprintf(stderr, "palette enc failed\n"); exit(3); }
        snprintf(path, sizeof(path), "%s_palette.png", prefix);
        lodepng_save_file(out, on, path);
        fprintf(stderr, "wrote %s (%zu bytes)\n", path, on);
        free(out);
        lodepng_state_cleanup(&st);
    }
    /* grey16 */
    {
        LodePNGColorMode in_mode = lodepng_color_mode_make(LCT_RGBA, 8);
        LodePNGColorMode g16 = lodepng_color_mode_make(LCT_GREY, 16);
        size_t gb = lodepng_get_raw_size(w, ht, &g16);
        unsigned char *grey = malloc(gb);
        if (lodepng_convert(grey, img, &g16, &in_mode, w, ht)) { fprintf(stderr, "convert failed\n"); exit(3); }
        LodePNGState st;
        lodepng_state_init(&st);
        st.info_raw.colortype = LCT_GREY; st.info_raw.bitdepth = 16;
        st.info_png.color.colortype = LCT_GREY; st.info_png.color.bitdepth = 16;
        st.encoder.auto_convert = 0;
        unsigned char *out = NULL; size_t on = 0;
        if (lodepng_encode(&out, &on, grey, w, ht, &st)) { fprintf(stderr, "grey16 enc failed\n"); exit(3); }
        snprintf(path, sizeof(path), "%s_grey16.png", prefix);
        lodepng_save_file(out, on, path);
        fprintf(stderr, "wrote %s (%zu bytes)\n", path, on);
        free(out); free(grey);
        lodepng_state_cleanup(&st);
    }
    /* interlaced (Adam7) */
    {
        LodePNGState st;
        lodepng_state_init(&st);
        st.info_png.interlace_method = 1;
        st.encoder.auto_convert = 0;
        unsigned char *out = NULL; size_t on = 0;
        if (lodepng_encode(&out, &on, img, w, ht, &st)) { fprintf(stderr, "interlace enc failed\n"); exit(3); }
        snprintf(path, sizeof(path), "%s_interlaced.png", prefix);
        lodepng_save_file(out, on, path);
        fprintf(stderr, "wrote %s (%zu bytes)\n", path, on);
        free(out);
        lodepng_state_cleanup(&st);
    }
    /* meta: ancillary chunks for the readChunk_* decode paths */
    {
        LodePNGState st;
        lodepng_state_init(&st);
        set_meta(&st);
        st.encoder.text_compression = 1;
        unsigned char *out = NULL; size_t on = 0;
        if (lodepng_encode(&out, &on, img, w, ht, &st)) { fprintf(stderr, "meta enc failed\n"); exit(3); }
        snprintf(path, sizeof(path), "%s_meta.png", prefix);
        lodepng_save_file(out, on, path);
        fprintf(stderr, "wrote %s (%zu bytes)\n", path, on);
        free(out);
        lodepng_state_cleanup(&st);
    }
    free(post); free(img); free(png);
}

int main(int argc, char **argv) {
    /* provenance sanity check on the linked implementation (stderr only) */
    if (LODEPNG_VERSION_STRING == NULL) { fprintf(stderr, "bad lodepng\n"); return 1; }
    if (argc == 4 && strcmp(argv[1], "gen_rgba") == 0) {
        gen_rgba(argv[2], argv[3]);
        return 0;
    }
    if (argc == 4 && strcmp(argv[1], "gen_tile") == 0) {
        gen_tile(argv[2], argv[3]);
        return 0;
    }
    if (argc == 4 && strcmp(argv[1], "gen_variant") == 0) {
        gen_variant(argv[2], argv[3]);
        return 0;
    }
    if (argc != 4) {
        fprintf(stderr,
                "usage: %s <decode32|decode_keep|encode32|chunk_crc> <input> <iters>\n"
                "       %s gen_rgba <input.png> <output.rgba>\n", argv[0], argv[0]);
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }
    size_t n;
    uint8_t *buf = read_file(argv[2], &n);

    if (strcmp(op, "decode32") == 0)              run_decode32(buf, n, iters);
    else if (strcmp(op, "decode_keep") == 0)      run_decode_keep(buf, n, iters);
    else if (strcmp(op, "decode_file") == 0)      run_decode_file(argv[2], iters);
    else if (strcmp(op, "encode32") == 0)         run_encode32(buf, n, iters);
    else if (strcmp(op, "encode_meta") == 0)      run_encode_meta(buf, n, iters);
    else if (strcmp(op, "encode_settings") == 0)  run_encode_settings(buf, n, iters);
    else if (strcmp(op, "zlib_roundtrip") == 0)   run_zlib_roundtrip(buf, n, iters);
    else if (strcmp(op, "convert") == 0)          run_convert(buf, n, iters);
    else if (strcmp(op, "chunk_crc") == 0)        run_chunk_crc(buf, n, iters);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    free(buf);
    return 0;
}
