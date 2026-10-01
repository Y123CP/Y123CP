/* binn_roundtrip — C reference for the Rust harness
 *   dataset_trans_process/binn/workloads/harness/binn_roundtrip_raw/src/main.rs
 *
 * EMPIRICAL STUDY: two operations dispatched by argv[1] (like bzip2 -z/-d) so ONE
 * binary covers binn's WRITE and READ hot paths. Both self-contained (input
 * ignored); C and Rust MUST emit identical stdout.
 *   default → ENCODE: build a binn list of M int32, serialize, loop N_ENC times,
 *             emit the serialized bytes of the last build.
 *   "decode"→ DECODE: build+serialize once, then loop N_DEC times iterating via
 *             binn_iter_init + binn_list_next, summing each value.vint32; emit
 *             "sum=<lld>". Exercises binn's read path.
 *
 * Build (fair): clang-17 -O3 -flto -march=native -DNDEBUG -DBINN_NO_COMPRESS \
 *               -I. binn_roundtrip.c binn.c -o <out>
 */
#include "binn.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define M 100000
#define N_ENC 600
#define N_DEC 1000

static binn *build_list(void) {
    binn *list = binn_list();
    for (int i = 0; i < M; i++) {
        int v = i;
        binn_list_add(list, BINN_INT32, &v, 0);
    }
    return list;
}

static void encode(int emit) {
    binn *list = build_list();
    void *out = binn_ptr(list);
    int sz = binn_size(list);
    if (emit) fwrite(out, 1, (size_t)sz, stdout);
    binn_free(list);
}

static void decode(void) {
    binn *list = build_list();
    void *out = binn_ptr(list);
    int sz = binn_size(list);
    unsigned char *buf = (unsigned char *)malloc((size_t)sz);
    memcpy(buf, out, (size_t)sz);
    binn_free(list);

    long long acc = 0;
    for (int k = 0; k < N_DEC; k++) {
        binn_iter iter;
        binn value;
        binn_iter_init(&iter, buf, BINN_LIST);
        while (binn_list_next(&iter, &value)) {
            acc += value.vint32;
        }
    }
    printf("sum=%lld\n", acc);
    free(buf);
}

int main(int argc, char *argv[]) {
    const char *mode = (argc > 1) ? argv[1] : "";
    if (strcmp(mode, "decode") == 0) {
        decode();
    } else {
        for (int k = 0; k < N_ENC - 1; k++) encode(0);
        encode(1);   /* final build → emit serialized bytes (oracle) */
    }
    return 0;
}
