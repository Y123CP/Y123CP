/* xxh_bench — C reference for the Rust harness
 *   dataset_trans_process/xxHash/workloads/harness/xxh_bench/src/main.rs
 *
 * EMPIRICAL STUDY (2026-07-01): same workload as the Rust harness — XXH64 over a
 * fixed 64 KiB buffer, N times with varying seed, XORing the digests. Built from
 * the SAME (patched) xxhash.h as the c2rust side: XXH_ASSUME + XXH_COMPILER_GUARD
 * are no-ops (c2rust can't handle __builtin_assume / the barrier asm), and
 * XXH_VECTOR=0 forces the scalar path (c2rust can't do SIMD) — all applied to
 * BOTH sides, so the comparison is fair. Input ignored. Must print the same
 * "hashes=N acc=HEX" as the Rust harness.
 *
 * Build (fair): clang-17 -O3 -flto -march=native -DNDEBUG -DXXH_VECTOR=0 \
 *               -DXXH_STATIC_LINKING_ONLY -I. xxh_bench.c xxhash.c -o <out>
 */
#define XXH_STATIC_LINKING_ONLY
#include "xxhash.h"
#include <stdio.h>
#include <string.h>

#define BUFSIZE 65536
#define N 50000

/* OPERATION 2 — COV: exercise XXH32/XXH64/XXH3-64/XXH3-128 one-shot + streaming +
 * canonical + secret/seed variants so fn coverage reaches ≥85%. Deterministic. */
static void cov_workload(void) {
    unsigned char buf[512];
    for (int i = 0; i < 512; i++) buf[i] = (unsigned char)(i * 31 + 7);
    unsigned long long acc = 0;

    /* XXH32 one-shot + streaming + canonical */
    acc ^= XXH32(buf, 512, 0);
    XXH32_state_t *s32 = XXH32_createState();
    XXH32_reset(s32, 1);
    XXH32_update(s32, buf, 256);
    XXH32_update(s32, buf + 256, 256);
    acc ^= XXH32_digest(s32);
    XXH32_freeState(s32);
    XXH32_canonical_t c32;
    XXH32_canonicalFromHash(&c32, 0x12345678u);
    acc ^= XXH32_hashFromCanonical(&c32);

    /* XXH64 one-shot + streaming + canonical */
    acc ^= XXH64(buf, 512, 0);
    XXH64_state_t *s64 = XXH64_createState();
    XXH64_reset(s64, 1);
    XXH64_update(s64, buf, 512);
    acc ^= XXH64_digest(s64);
    XXH64_freeState(s64);
    XXH64_canonical_t c64;
    XXH64_canonicalFromHash(&c64, 0x123456789ABCDEFULL);
    acc ^= XXH64_hashFromCanonical(&c64);

    /* XXH3-64 one-shot + seed + secret + streaming */
    acc ^= XXH3_64bits(buf, 512);
    acc ^= XXH3_64bits_withSeed(buf, 512, 42);
    acc ^= XXH3_64bits_withSecret(buf, 512, buf, 192);
    XXH3_state_t *s3 = XXH3_createState();
    XXH3_64bits_reset(s3);
    XXH3_64bits_update(s3, buf, 512);
    acc ^= XXH3_64bits_digest(s3);
    XXH3_64bits_reset_withSeed(s3, 42);
    XXH3_64bits_update(s3, buf, 512);
    acc ^= XXH3_64bits_digest(s3);
    XXH3_64bits_reset_withSecret(s3, buf, 192);
    XXH3_64bits_update(s3, buf, 512);
    acc ^= XXH3_64bits_digest(s3);
    XXH3_freeState(s3);

    /* XXH3-128 one-shot + seed + streaming + canonical */
    XXH128_hash_t h = XXH3_128bits(buf, 512);
    acc ^= h.low64 ^ h.high64;
    h = XXH3_128bits_withSeed(buf, 512, 42);
    acc ^= h.low64;
    h = XXH3_128bits_withSecret(buf, 512, buf, 192);
    acc ^= h.high64;
    XXH3_state_t *s128 = XXH3_createState();
    XXH3_128bits_reset(s128);
    XXH3_128bits_update(s128, buf, 512);
    h = XXH3_128bits_digest(s128);
    acc ^= h.low64;
    XXH3_freeState(s128);
    XXH128_canonical_t c128;
    XXH128_canonicalFromHash(&c128, h);
    XXH128_hash_t h2 = XXH128_hashFromCanonical(&c128);
    acc ^= h2.high64;
    acc ^= (unsigned long long)XXH128_isEqual(h, h2);
    acc ^= (unsigned long long)XXH128_cmp(&h, &h2);

    /* batch 2: large input (long scrambler path) + secret-gen + withSecretandSeed */
    unsigned char big[4096];
    for (int i = 0; i < 4096; i++) big[i] = (unsigned char)(i * 17 + 5);
    acc ^= XXH3_64bits(big, 4096);
    XXH128_hash_t hb = XXH3_128bits(big, 4096);
    acc ^= hb.low64;
    unsigned char secret[192];
    XXH3_generateSecret(secret, sizeof secret, buf, 64);
    XXH3_generateSecret_fromSeed(secret, 42);
    acc ^= XXH3_64bits_withSecret(big, 4096, secret, sizeof secret);
    acc ^= XXH3_64bits_withSecretandSeed(buf, 512, secret, sizeof secret, 42);
    XXH128_hash_t hss = XXH3_128bits_withSecretandSeed(buf, 512, secret, sizeof secret, 42);
    acc ^= hss.low64;
    XXH3_state_t *ss = XXH3_createState();
    XXH3_64bits_reset_withSecretandSeed(ss, secret, sizeof secret, 42);
    XXH3_64bits_update(ss, big, 4096);
    acc ^= XXH3_64bits_digest(ss);
    XXH3_128bits_reset_withSeed(ss, 7);
    XXH3_128bits_update(ss, big, 4096);
    acc ^= XXH3_128bits_digest(ss).low64;
    XXH3_128bits_reset_withSecret(ss, secret, sizeof secret);
    XXH3_128bits_update(ss, big, 4096);
    acc ^= XXH3_128bits_digest(ss).high64;
    XXH3_freeState(ss);

    acc ^= XXH_versionNumber();
    printf("cov acc=%016llx\n", acc);
}

int main(int argc, char *argv[]) {
    if (argc > 1 && strcmp(argv[1], "cov") == 0) { cov_workload(); return 0; }
    /* argv[1] selects the hash variant: xxh64 (default) | xxh3 | xxh32.
       Any other value (e.g. an ignored input path) falls through to xxh64. */
    const char *mode = (argc > 1) ? argv[1] : "xxh64";
    unsigned char buf[BUFSIZE];
    for (int i = 0; i < BUFSIZE; i++)
        buf[i] = (unsigned char)(i * 31 + 7);

    unsigned long long acc = 0;
    if (strcmp(mode, "xxh3") == 0) {
        for (unsigned long long n = 0; n < N; n++)
            acc ^= (unsigned long long)XXH3_64bits_withSeed(buf, BUFSIZE, n);
    } else if (strcmp(mode, "xxh32") == 0) {
        for (unsigned long long n = 0; n < N; n++)
            acc ^= (unsigned long long)XXH32(buf, BUFSIZE, (XXH32_hash_t)n);
    } else {
        for (unsigned long long n = 0; n < N; n++)
            acc ^= (unsigned long long)XXH64(buf, BUFSIZE, n);
    }
    printf("hashes=%d acc=%016llx\n", N, acc);
    return 0;
}
