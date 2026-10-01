/* C-side mirror of dataset_trans_process/lodepng/workloads/harness/
 * lodepng_roundtrip/src/main.rs — must produce IDENTICAL stdout for fair
 * three-way comparison. Pattern: decode → encode → decode → byte-compare.
 *
 * Build: clang-14 -O3 -flto -march=native -DNDEBUG -o lodepng_roundtrip_c \
 *        lodepng_roundtrip_c.c lodepng.cpp -x c++ -lm
 */
#include "lodepng.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static unsigned char *slurp(const char *path, size_t *outsize) {
    FILE *f = fopen(path, "rb");
    if (!f) return NULL;
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    if (sz <= 0) { fclose(f); return NULL; }
    fseek(f, 0, SEEK_SET);
    unsigned char *buf = (unsigned char *)malloc((size_t)sz);
    if (!buf) { fclose(f); return NULL; }
    size_t r = fread(buf, 1, (size_t)sz, f);
    fclose(f);
    if (r != (size_t)sz) { free(buf); return NULL; }
    *outsize = (size_t)sz;
    return buf;
}

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <input.png>\n", argv[0]);
        return 1;
    }
    size_t insz = 0;
    unsigned char *png = slurp(argv[1], &insz);
    if (!png) { fprintf(stderr, "read %s: failed\n", argv[1]); return 1; }

    unsigned char *px1 = NULL;
    unsigned w1 = 0, h1 = 0;
    if (lodepng_decode32(&px1, &w1, &h1, png, insz) != 0 || !px1) {
        printf("lodepng roundtrip: FAILED\n");
        return 1;
    }
    size_t n1 = (size_t)w1 * h1 * 4;

    unsigned char *png2 = NULL;
    size_t png2sz = 0;
    if (lodepng_encode32(&png2, &png2sz, px1, w1, h1) != 0 || !png2) {
        printf("lodepng roundtrip: FAILED\n");
        return 1;
    }

    unsigned char *px2 = NULL;
    unsigned w2 = 0, h2 = 0;
    if (lodepng_decode32(&px2, &w2, &h2, png2, png2sz) != 0 || !px2) {
        printf("lodepng roundtrip: FAILED\n");
        return 1;
    }
    size_t n2 = (size_t)w2 * h2 * 4;

    if (w1 == w2 && h1 == h2 && n1 == n2 && memcmp(px1, px2, n1) == 0) {
        printf("lodepng roundtrip: ok (pixels=%zu)\n", n1);
        return 0;
    } else {
        printf("lodepng roundtrip: FAILED\n");
        return 1;
    }
}
