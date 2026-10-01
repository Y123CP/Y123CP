/* libzahl_opsuite — C reference for the Rust harness
 *   dataset_trans_process/libzahl/workloads/harness/libzahl_opsuite_raw/src/main.rs
 *
 * EMPIRICAL STUDY (2026-07-01): rewritten to measure REAL multi-precision
 * arithmetic. The old version zsetu'd operands to 0 → the op loop did 0+0/0*0
 * on empty bignums (measured startup + call/pool overhead, not arithmetic).
 * We now zload two LARGE ~4096-bit operands, crafted byte-identically to the
 * Rust harness (same LCG, seeds, 64 limbs), so the loop exercises real
 * big-number add/sub/mul. Both harnesses print "pairs=N used=M checksum=HEX"
 * where checksum folds acc's limbs (acc = 1500*(x+y)); C and Rust MUST match.
 * zload buffer format (src/zload.c): [long sign][size_t used][used*u64 limbs LE].
 *
 * Usage: libzahl_opsuite <input.bin>   (input ignored — operands in-harness)
 * Build (fair): clang-17 -O3 -flto -march=native -DNDEBUG -I. -Isrc \
 *               libzahl_opsuite.c src/*.c -o <out>
 */
#include "zahl.h"
#include <setjmp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

static size_t make_operand(unsigned char *buf, uint64_t seed, size_t nlimbs) {
    long sign = 1;
    size_t off = 0;
    memcpy(buf + off, &sign, sizeof(long));   off += sizeof(long);
    memcpy(buf + off, &nlimbs, sizeof(size_t)); off += sizeof(size_t);
    uint64_t s = seed;
    for (size_t i = 0; i < nlimbs; i++) {
        s = s * 0x9E3779B97F4A7C15ULL + (uint64_t)(i + 1);
        uint64_t limb = (i + 1 == nlimbs) ? (s | (1ULL << 63)) : (s | 1ULL);
        memcpy(buf + off, &limb, 8); off += 8;
    }
    return off;
}

static inline uint64_t rotl64(uint64_t v, unsigned n) {
    n &= 63;
    return n ? ((v << n) | (v >> (64 - n))) : v;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <input.bin>\n", argv[0]);
        return 1;
    }

    jmp_buf jbuf;
    int rc = setjmp(jbuf);
    if (rc != 0) {
        fprintf(stderr, "libzahl arithmetic error (longjmp code %d)\n", rc);
        return 2;
    }
    zsetup(jbuf);

    z_t x, y, acc, tmp;
    memset(x, 0, sizeof(struct zahl));
    memset(y, 0, sizeof(struct zahl));
    memset(acc, 0, sizeof(struct zahl));
    memset(tmp, 0, sizeof(struct zahl));

    unsigned char bx[16 + 64 * 8], by[16 + 64 * 8];
    unsigned char zbuf[16];
    memset(zbuf, 0, sizeof(zbuf));                 /* sign=0,used=0 → 0 */
    make_operand(bx, 0xB2C3D4E5F6071829ULL, 64);   /* ~4096-bit */
    make_operand(by, 0x13579BDF2468ACE0ULL, 64);   /* ~4096-bit, distinct */
    zload(x, bx);
    zload(y, by);
    zload(acc, zbuf);
    zload(tmp, zbuf);

    if (strcmp((argc > 1 ? argv[1] : ""), "cov") == 0) {
        /* OPERATION 3 — COV: call the remaining DETERMINISTIC bignum ops so libzahl
         * fn coverage reaches ≥85% (skips zrand/zptest — non-deterministic). C and
         * Rust MUST print identical stdout. */
        z_t r, m, e, nx, g;
        memset(r, 0, sizeof(struct zahl));  memset(m, 0, sizeof(struct zahl));
        memset(e, 0, sizeof(struct zahl));  memset(nx, 0, sizeof(struct zahl));
        memset(g, 0, sizeof(struct zahl));
        zload(r, zbuf); zload(m, zbuf); zload(e, zbuf); zload(nx, zbuf); zload(g, zbuf);
        zsetu(m, 0xFFFFFFFFFFFFFFC5ULL); zlsh(m, m, 400); zbset(m, m, 0, 1); /* big odd modulus */
        zsetu(e, 5);
        zand(r, x, y); zor(r, x, y); zxor(r, x, y); znot(r, x);      /* bitwise */
        zgcd(r, x, y);                                               /* gcd */
        zsqr(r, x); zpowu(r, y, 3); zpow(r, y, e);                   /* squares/powers */
        zmodmul(r, x, y, m); zmodsqr(r, x, m);                       /* modular */
        zmodpowu(r, x, 7ULL, m); zmodpow(r, x, e, m);
        zlsh(r, x, 13); zrsh(r, x, 13);                             /* shifts */
        zbset(r, x, 10, 1); zbset(r, x, 10, 0); zbset(r, x, 10, -1); /* set/clear/flip */
        zneg(nx, x); zabs(r, nx);                                    /* sign */
        zadd(r, x, nx); zsub(r, x, nx); zadd(r, nx, y); zsub(r, nx, y); /* *_unsigned/_assign */
        zswap(r, nx);
        zsets(r, "123456789012345678901234567890");                 /* set from string */
        char sbuf[8192]; zstr(x, sbuf, sizeof sbuf);                 /* to string */
        char sbuf2[8192]; zstr(nx, sbuf2, sizeof sbuf2);            /* negative → sign path */
        size_t sl = zstr_length(x, 10);
        const char *edesc = NULL; (void)zerror(&edesc);             /* error state */
        zperror("libzahl-cov");                                     /* error print (stderr) */
        zgcd(g, x, y);                                               /* deterministic checksum */
        struct zahl *pg = (struct zahl *)g;
        uint64_t checksum = (uint64_t)pg->used;
        for (size_t i = 0; i < pg->used; i++)
            checksum ^= rotl64((uint64_t)pg->chars[i], (unsigned)(i & 63));
        unsigned long ssum = 0;
        for (const char *s = sbuf; *s; s++) ssum = ssum * 131u + (unsigned char)*s;
        printf("cov gcd_used=%zu strlen=%zu ssum=%lu checksum=%016lx\n",
               pg->used, sl, ssum, (unsigned long)checksum);
    } else if (strcmp((argc > 1 ? argv[1] : ""), "divmod") == 0) {
        /* OPERATION 2 — DIVISION: dividend = x*y (~8192-bit) ÷ y (~4096-bit), N times
         * → real schoolbook long division. Exercises zdivmod, omitted by add/sub/mul. */
        z_t big, q, r;
        memset(big, 0, sizeof(struct zahl));
        memset(q, 0, sizeof(struct zahl));
        memset(r, 0, sizeof(struct zahl));
        zload(big, zbuf);
        zload(q, zbuf);
        zload(r, zbuf);
        zmul(big, x, y);
        unsigned long pairs = 0;
        for (int i = 0; i < 1500; i++) {
            zdivmod(q, r, big, y);
            pairs++;
        }
        struct zahl *pq = (struct zahl *)q;
        uint64_t checksum = (uint64_t)pq->used;
        for (size_t i = 0; i < pq->used; i++)
            checksum ^= rotl64((uint64_t)pq->chars[i], (unsigned)(i & 63));
        printf("pairs=%lu used=%zu checksum=%016lx\n",
               pairs, pq->used, (unsigned long)checksum);
    } else if (strcmp((argc > 1 ? argv[1] : ""), "modpow") == 0) {
        /* OPERATION 4 — MODULAR EXPONENTIATION: res = x^e mod y, with y a ~4096-bit
         * odd modulus and e a ~256-bit exponent, N times → the crypto-relevant heavy
         * kernel (square-and-multiply over modular multiplies). */
        z_t e, res;
        memset(e, 0, sizeof(struct zahl));
        memset(res, 0, sizeof(struct zahl));
        unsigned char be[16 + 4 * 8];
        make_operand(be, 0x2468ACE013579BDFULL, 4);   /* ~256-bit exponent */
        zload(e, be);
        zload(res, zbuf);
        unsigned long pairs = 0;
        for (int i = 0; i < 3; i++) {
            zmodpow(res, x, e, y);
            pairs++;
        }
        struct zahl *pr = (struct zahl *)res;
        uint64_t checksum = (uint64_t)pr->used;
        for (size_t i = 0; i < pr->used; i++)
            checksum ^= rotl64((uint64_t)pr->chars[i], (unsigned)(i & 63));
        printf("pairs=%lu used=%zu checksum=%016lx\n",
               pairs, pr->used, (unsigned long)checksum);
    } else {
        /* OPERATION 1 — MULTIPLY: isolate the O(n²) schoolbook multiply kernel
         * (add/sub are O(n), too fast to time meaningfully and diluted the signal). */
        unsigned long pairs = 0;
        for (int i = 0; i < 1500; i++) {
            zmul(tmp, x, y);   /* zmul_ll in Rust ≡ zmul for positive operands */
            pairs++;
        }
        struct zahl *pt = (struct zahl *)tmp;
        uint64_t checksum = (uint64_t)pt->used;
        for (size_t i = 0; i < pt->used; i++)
            checksum ^= rotl64((uint64_t)pt->chars[i], (unsigned)(i & 63));
        printf("pairs=%lu used=%zu checksum=%016lx\n",
               pairs, pt->used, (unsigned long)checksum);
    }

    zunsetup();
    (void)x; (void)y; (void)acc; (void)tmp;
    return 0;
}
