/* C-side mirror of dataset_trans_process/heman/workloads/harness/
 * heman_pipeline/src/main.rs — must produce IDENTICAL stdout for fair
 * three-way comparison.
 *
 * Pattern: import_u8 → distance_create_df → color_create_gradient →
 *          color_apply_gradient → export_u8 → sha256(out) → stdout
 *
 * Build: clang-14 -O3 -flto -march=native -DNDEBUG -I include \
 *        -o heman_pipeline_c heman_pipeline_c.c src/*.c -lm -lcrypto
 */
#include "heman.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
/* <openssl/sha.h> removed — empirical study emits raw RGB, no sha256 in timed region */

/* OPERATION 3 — GENERATE: island heightmap from OpenSimplex noise, GEN_N times,
 * export the last. Self-contained (no seed file). Exercises generate.c + noise.c. */
#define GEN_SZ   512
#define GEN_SEED 42
#define GEN_N    6
#define PIPE_N   3   /* repeat pipeline/lighting to clear the wall-clock floor on small seeds; export LAST (digest loop-independent) */

static void generate_workload(void) {
    heman_image *last = NULL;
    for (int i = 0; i < GEN_N; i++) {
        heman_image *img = heman_generate_island_heightmap(GEN_SZ, GEN_SZ, GEN_SEED);
        if (i + 1 < GEN_N) heman_image_destroy(img);
        else last = img;
    }
    int cw = 0, ch = 0, cn = 0;
    heman_image_info(last, &cw, &ch, &cn);
    size_t nout = (size_t)cw * (size_t)ch * (size_t)cn;
    unsigned char *out = (unsigned char *)malloc(nout);
    heman_export_u8(last, 0.0f, 1.0f, out);
    fwrite(out, 1, nout, stdout);
    free(out);
    heman_image_destroy(last);
}

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: %s <seed.bin | generate> [lighting]\n", argv[0]);
        return 2;
    }
    if (strcmp(argv[1], "generate") == 0) {   /* OPERATION 3 — no seed file */
        generate_workload();
        return 0;
    }
    int lighting_mode = (argc >= 3 && strcmp(argv[2], "lighting") == 0);
    FILE *f = fopen(argv[1], "rb");
    if (!f) { fprintf(stderr, "open %s: failed\n", argv[1]); return 1; }
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    if (sz < 8) { fclose(f); return 1; }
    fseek(f, 0, SEEK_SET);
    unsigned char *buf = (unsigned char *)malloc((size_t)sz);
    if (fread(buf, 1, (size_t)sz, f) != (size_t)sz) {
        fclose(f); free(buf); return 1;
    }
    fclose(f);

    int w = (int)((uint32_t)buf[0] | ((uint32_t)buf[1] << 8)
                                  | ((uint32_t)buf[2] << 16)
                                  | ((uint32_t)buf[3] << 24));
    int h = (int)((uint32_t)buf[4] | ((uint32_t)buf[5] << 8)
                                  | ((uint32_t)buf[6] << 16)
                                  | ((uint32_t)buf[7] << 24));
    unsigned char *pixels = buf + 8;
    size_t pixn = (size_t)w * (size_t)h;
    if ((size_t)(sz - 8) != pixn) {
        fprintf(stderr, "pixel buffer size mismatch\n");
        free(buf); return 1;
    }

    /* PIPE_N repeats of the full pipeline (import→df→gradient→apply→[lighting]→export);
       export only the LAST buffer so stdout digest stays loop-independent, all
       intermediates freed each iter (no leak). Scales runtime to clear the floor. */
    unsigned char *out = NULL;
    size_t nout = 0;
    for (int _it = 0; _it < PIPE_N; _it++) {
        heman_image *src = heman_import_u8(w, h, 1, pixels, 0.0f, 1.0f);
        if (!src) { fprintf(stderr, "heman_import_u8 null\n"); free(buf); free(out); return 1; }

        heman_image *df = heman_distance_create_df(src);

        int cp_locs[5] = {0, 64, 128, 192, 255};
        heman_color cp_vals[5] = {0x000033u, 0x0055AAu, 0x33AA66u, 0xCCAA22u, 0xFFEECCu};
        heman_image *gradient = heman_color_create_gradient(256, 5, cp_locs, cp_vals);

        heman_image *colored = heman_color_apply_gradient(df, -1.0f, 1.0f, gradient);

        /* OPERATION 2 — LIGHTING: normals + occlusion + diffuse shading (lighting.c). */
        heman_image *final_img = colored;
        if (lighting_mode) {
            float light_position[3] = {-0.5f, 0.5f, 1.0f};
            final_img = heman_lighting_apply(src, colored, 1.0f, 1.0f, 0.5f, light_position);
        }

        int cw = 0, ch = 0, cn = 0;
        heman_image_info(final_img, &cw, &ch, &cn);
        nout = (size_t)cw * (size_t)ch * (size_t)cn;
        free(out);
        out = (unsigned char *)malloc(nout);
        heman_export_u8(final_img, 0.0f, 1.0f, out);

        if (lighting_mode) heman_image_destroy(final_img);
        heman_image_destroy(colored);
        heman_image_destroy(gradient);
        heman_image_destroy(df);
        heman_image_destroy(src);
    }
    /* Empirical study: emit the raw RGB buffer to stdout; the driver hashes it. */
    fwrite(out, 1, nout, stdout);

    free(out);
    free(buf);
    return 0;
}
