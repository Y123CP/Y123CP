/*
 * validation driver for fzy — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves.
 *
 * NOTE: the C function `match()` is renamed `match_0` by c2rust (Rust keyword
 * collision), so its exported symbol differs across versions. The driver
 * therefore drives the same kernel through match_positions() (match() is a
 * thin wrapper around it) and has_match().
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     match_batch      : has_match filter + match_positions scoring of every
 *                        candidate line against a fixed needle set
 *     match_positions_full : scoring with position arrays (first 2000 lines)
 *     choices_ops      : choices_init/add/search/get/getscore/next/prev
 *                        lifecycle over the candidate list (workers=1 for
 *                        determinism -- fzy search is threaded by default)
 *     choices_fread_op : choices_fread parsing of the candidate buffer
 *     options_matrix   : options_parse over real flag combinations
 *     tty_ops          : full tty_* surface against a PTY pair (fzy is a
 *                        terminal program; escape output folded from master)
 *
 * Input: candidates.txt = sorted real file listing (one path per line).
 * Output: ONE digest line on stdout (project convention).
 */
#define _XOPEN_SOURCE 600
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <termios.h>
#include <unistd.h>

#include "match.h"
#include "choices.h"
#include "options.h"
#include "tty.h"

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

/* split the input buffer into NUL-terminated lines (mutates buf) */
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

static const char *NEEDLES[] = { "a", "src", "main", "test", "make", "cfg.h" };
#define N_NEEDLES 6

static void run_match_batch(char *buf, size_t n, long iters) {
    size_t cnt;
    char **lines = split_lines(buf, n, &cnt);
    uint64_t h = 0, matched = 0;
    for (long i = 0; i < iters; i++) {
        matched = 0;
        for (size_t k = 0; k < N_NEEDLES; k++) {
            for (size_t j = 0; j < cnt; j++) {
                if (has_match(NEEDLES[k], lines[j])) {
                    score_t s = match_positions(NEEDLES[k], lines[j], NULL);
                    h = digest64(h, (const uint8_t *)&s, sizeof(s));
                    matched++;
                }
            }
        }
    }
    printf("op=match_batch in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)matched, iters, (unsigned long long)h);
    free(lines);
}

static void run_match_positions_full(char *buf, size_t n, long iters) {
    size_t cnt;
    char **lines = split_lines(buf, n, &cnt);
    if (cnt > 2000) cnt = 2000;
    size_t pos[MATCH_MAX_LEN];
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        for (size_t k = 0; k < N_NEEDLES; k++) {
            size_t nl = strlen(NEEDLES[k]);
            for (size_t j = 0; j < cnt; j++) {
                if (!has_match(NEEDLES[k], lines[j])) continue;
                memset(pos, 0, nl * sizeof(size_t));
                score_t s = match_positions(NEEDLES[k], lines[j], pos);
                h = digest64(h, (const uint8_t *)&s, sizeof(s));
                h = digest64(h, (const uint8_t *)pos, nl * sizeof(size_t));
            }
        }
    }
    printf("op=match_positions_full in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, cnt, iters, (unsigned long long)h);
    free(lines);
}

static void run_choices_ops(char *buf, size_t n, long iters) {
    size_t cnt;
    char **lines = split_lines(buf, n, &cnt);
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        options_t opt;
        options_init(&opt);
        opt.workers = 1;              /* deterministic single-threaded search */
        choices_t c;
        choices_init(&c, &opt);
        for (size_t j = 0; j < cnt; j++)
            choices_add(&c, lines[j]);
        for (size_t k = 0; k < N_NEEDLES; k++) {
            choices_search(&c, NEEDLES[k]);
            uint64_t avail = (uint64_t)choices_available(&c);
            h = digest64(h, (const uint8_t *)&avail, sizeof(avail));
            size_t top = avail < 20 ? (size_t)avail : 20;
            for (size_t t = 0; t < top; t++) {
                const char *s = choices_get(&c, t);
                score_t sc = choices_getscore(&c, t);
                if (s) h = digest64(h, (const uint8_t *)s, strlen(s));
                h = digest64(h, (const uint8_t *)&sc, sizeof(sc));
            }
            choices_next(&c);
            choices_next(&c);
            choices_prev(&c);
            uint64_t sel = (uint64_t)c.selection;
            h = digest64(h, (const uint8_t *)&sel, sizeof(sel));
        }
        choices_destroy(&c);

        /* threaded search path (worker merge); fzy's merge is deterministic
         * and the cross-version equivalence gate verifies it */
        options_t opt2;
        options_init(&opt2);
        opt2.workers = 2;
        choices_t c2;
        choices_init(&c2, &opt2);
        for (size_t j = 0; j < cnt; j++)
            choices_add(&c2, lines[j]);
        choices_search(&c2, "src");
        uint64_t avail2 = (uint64_t)choices_available(&c2);
        h = digest64(h, (const uint8_t *)&avail2, sizeof(avail2));
        for (size_t t = 0; t < (avail2 < 10 ? avail2 : 10); t++) {
            const char *s = choices_get(&c2, t);
            score_t sc = choices_getscore(&c2, t);
            if (s) h = digest64(h, (const uint8_t *)s, strlen(s));
            h = digest64(h, (const uint8_t *)&sc, sizeof(sc));
        }
        choices_destroy(&c2);
    }
    printf("op=choices_ops in=%zu out=%zu iters=%ld digest=%016llx\n",
           n, cnt, iters, (unsigned long long)h);
    free(lines);
}

static void run_choices_fread(const char *path, long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        FILE *f = fopen(path, "rb");
        if (!f) { fprintf(stderr, "cannot open %s\n", path); exit(2); }
        options_t opt;
        options_init(&opt);
        opt.workers = 1;
        choices_t c;
        choices_init(&c, &opt);
        choices_fread(&c, f, '\n');
        fclose(f);
        choices_search(&c, "src");
        uint64_t acc[2] = { c.size, choices_available(&c) };
        h = digest64(h, (const uint8_t *)acc, sizeof(acc));
        if (choices_available(&c) > 0) {
            const char *s = choices_get(&c, 0);
            if (s) h = digest64(h, (const uint8_t *)s, strlen(s));
        }
        choices_destroy(&c);
    }
    printf("op=choices_fread_op in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

static void run_options_matrix(long iters) {
    static char *argvs[4][8] = {
        { "fzy", "-l", "20", "-s", NULL },
        { "fzy", "-e", "needle", "-q", "init", NULL },
        { "fzy", "-p", "seek> ", "-j", "1", NULL },
        { "fzy", "-i", "-0", NULL },
    };
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        for (size_t k = 0; k < 4; k++) {
            int argc = 0;
            while (argvs[k][argc]) argc++;
            options_t opt;
            options_init(&opt);
            optind = 0;   /* glibc: 0 = full getopt state re-initialization */
            options_parse(&opt, argc, argvs[k]);
            uint64_t acc[6] = {
                (uint64_t)opt.show_scores, (uint64_t)opt.num_lines,
                (uint64_t)opt.workers, (uint64_t)(unsigned char)opt.input_delimiter,
                opt.filter ? strlen(opt.filter) : 0,
                opt.prompt ? strlen(opt.prompt) : 0,
            };
            h = digest64(h, (const uint8_t *)acc, sizeof(acc));
        }
    }
    printf("op=options_matrix in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* drive the tty_* surface against a PTY pair; fold the escape stream that
 * fzy writes to the terminal (read back from the master side) */
static void run_tty_ops(long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        int master = posix_openpt(O_RDWR | O_NOCTTY);
        if (master < 0 || grantpt(master) || unlockpt(master)) {
            fprintf(stderr, "pty setup failed\n"); exit(2);
        }
        struct winsize ws = { 24, 80, 0, 0 };
        ioctl(master, TIOCSWINSZ, &ws);
        const char *slave = ptsname(master);
        if (!slave) { fprintf(stderr, "ptsname failed\n"); exit(2); }

        tty_t tty;
        tty_init(&tty, slave);
        ioctl(master, TIOCSWINSZ, &ws);   /* re-assert after termios churn */
        tty_getwinsz(&tty);
        uint64_t dims[2] = { tty_getwidth(&tty), tty_getheight(&tty) };
        h = digest64(h, (const uint8_t *)dims, sizeof(dims));

        tty_setfg(&tty, 2);
        tty_setinvert(&tty);
        tty_setunderline(&tty);
        tty_setnormal(&tty);
        tty_setnowrap(&tty);
        tty_printf(&tty, "score %d %s", 42, "line");
        tty_putc(&tty, 'X');
        tty_setcol(&tty, 5);
        tty_clearline(&tty);
        tty_newline(&tty);
        tty_moveup(&tty, 2);
        tty_setwrap(&tty);
        tty_flush(&tty);

        /* input path: feed a char through the master, read via tty_getchar */
        if (write(master, "k", 1) == 1) {
            int ready = tty_input_ready(&tty, 100, 0);
            char ch = tty_getchar(&tty);
            uint64_t in[2] = { (uint64_t)ready, (uint64_t)(unsigned char)ch };
            h = digest64(h, (const uint8_t *)in, sizeof(in));
        }

        /* fold everything fzy wrote to the terminal */
        unsigned char out[4096];
        ssize_t got = read(master, out, sizeof(out));
        if (got > 0) h = digest64(h, out, (size_t)got);
        uint64_t g = (uint64_t)got;
        h = digest64(h, (const uint8_t *)&g, sizeof(g));

        tty_reset(&tty);
        tty_close(&tty);
        close(master);
    }
    printf("op=tty_ops in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "usage: %s <op> <input> <iters>\n", argv[0]);
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }

    if (strcmp(op, "options_matrix") == 0) { run_options_matrix(iters); return 0; }
    if (strcmp(op, "tty_ops") == 0)        { run_tty_ops(iters); return 0; }
    if (strcmp(op, "choices_fread_op") == 0) { run_choices_fread(argv[2], iters); return 0; }

    size_t n;
    uint8_t *buf = read_file(argv[2], &n);
    if (strcmp(op, "match_batch") == 0)              run_match_batch((char *)buf, n, iters);
    else if (strcmp(op, "match_positions_full") == 0) run_match_positions_full((char *)buf, n, iters);
    else if (strcmp(op, "choices_ops") == 0)         run_choices_ops((char *)buf, n, iters);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    free(buf);
    return 0;
}
