/*
 * validation driver for lil — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves.
 *
 * NOTE (ours): lil's 2_stage_a lifted the API to safe Rust signatures and
 * dropped the C ABI. Plan: shim_ours carries thin #[no_mangle] extern "C"
 * wrappers around the lifted functions (inlined under fat LTO), so this
 * driver stays byte-identical across all three targets.
 *
 * CLI:
 *   driver run_script  <script.lil> <iters>   parse+run a real LIL script;
 *                                             script writes are folded via a
 *                                             registered WRITE callback
 *   driver api_surface <ignored>    <iters>   value/list/env/var/call/expr
 *                                             builder API surface
 *
 * Output: ONE digest line on stdout (project convention).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "lil.h"

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

/* WRITE callback: fold script output into the accumulator attached via
 * lil_set_data (the callback carries no user pointer of its own) */
struct acc { uint64_t h; uint64_t writes; };

static void write_cb(lil_t lil, const char *msg) {
    struct acc *a = (struct acc *)lil_get_data(lil);
    if (a && msg) {
        a->h = digest64(a->h, (const uint8_t *)msg, strlen(msg));
        a->writes++;
    }
}

static void run_script(const char *path, long iters) {
    size_t n;
    uint8_t *code = read_file(path, &n);
    uint64_t h = 0, writes = 0;
    for (long i = 0; i < iters; i++) {
        struct acc a = {0};
        lil_t lil = lil_new();
        if (!lil) { fprintf(stderr, "lil_new failed\n"); exit(3); }
        lil_set_data(lil, &a);
        lil_callback(lil, LIL_CALLBACK_WRITE, (lil_callback_proc_t)write_cb);
        lil_value_t v = lil_parse(lil, (const char *)code, n, 0);
        const char *s = lil_to_string(v);
        if (s) h = digest64(h, (const uint8_t *)s, strlen(s));
        lil_free_value(v);
        const char *errmsg = NULL;
        size_t errpos = 0;
        uint64_t haderr = (uint64_t)lil_error(lil, &errmsg, &errpos);
        h = digest64(h, (const uint8_t *)&haderr, sizeof(haderr));
        if (haderr && errmsg) h = digest64(h, (const uint8_t *)errmsg, strlen(errmsg));
        h = digest64(h, (const uint8_t *)&a.h, sizeof(a.h));
        writes = a.writes;
        lil_free(lil);
    }
    printf("op=run_script in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)writes, iters, (unsigned long long)h);
    free(code);
}

/* run every script listed in a manifest (one path per line), each in a
 * FRESH interpreter -- demo scripts may end with top-level `return` or
 * install catchers, so isolation is required for full execution */
static void run_suite(const char *listpath, long iters) {
    size_t ln;
    uint8_t *list = read_file(listpath, &ln);
    uint64_t h = 0, scripts = 0;
    for (long i = 0; i < iters; i++) {
        scripts = 0;
        char *p = (char *)list, *end = (char *)list + ln;
        while (p < end) {
            char *nl = memchr(p, '\n', (size_t)(end - p));
            if (!nl) nl = end;
            *nl = 0;
            if (*p) {
                size_t n;
                uint8_t *code = read_file(p, &n);
                struct acc a = {0};
                lil_t lil = lil_new();
                lil_set_data(lil, &a);
                lil_callback(lil, LIL_CALLBACK_WRITE, (lil_callback_proc_t)write_cb);
                lil_value_t v = lil_parse(lil, (const char *)code, n, 0);
                const char *s = lil_to_string(v);
                if (s) h = digest64(h, (const uint8_t *)s, strlen(s));
                lil_free_value(v);
                h = digest64(h, (const uint8_t *)&a.h, sizeof(a.h));
                lil_free(lil);
                free(code);
                scripts++;
            }
            *nl = '\n';           /* restore for next iteration */
            p = nl + 1;
            if (nl == end) break;
        }
    }
    printf("op=run_suite in=%zu out=%llu iters=%ld digest=%016llx\n",
           ln, (unsigned long long)scripts, iters, (unsigned long long)h);
    free(list);
}

/* custom native function for lil_register + lil_call */
static lil_value_t native_sum(lil_t lil, size_t argc, lil_value_t *argv) {
    (void)lil;
    lilint_t sum = 0;
    for (size_t i = 0; i < argc; i++) sum += lil_to_integer(lil_arg(argv, i));
    return lil_alloc_integer(sum);
}

static void run_api_surface(long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        struct acc a = {0};
        lil_t lil = lil_new();
        lil_set_data(lil, &a);
        lil_callback(lil, LIL_CALLBACK_WRITE, (lil_callback_proc_t)write_cb);

        /* values */
        lil_value_t vs = lil_alloc_string("hello lil");
        lil_value_t vi = lil_alloc_integer(424242);
        lil_value_t vd = lil_alloc_double(3.5);
        lil_value_t vc = lil_clone_value(vs);
        lil_append_char(vc, '!');
        lil_append_string(vc, " world");
        lil_append_val(vc, vi);
        uint64_t nums[3] = { (uint64_t)lil_to_integer(vi),
                             (uint64_t)lil_to_boolean(vd),
                             (uint64_t)(lil_to_double(vd) * 1000) };
        h = digest64(h, (const uint8_t *)nums, sizeof(nums));
        h = digest64(h, (const uint8_t *)lil_to_string(vc), strlen(lil_to_string(vc)));

        /* lists */
        lil_list_t list = lil_alloc_list();
        lil_list_append(list, lil_clone_value(vs));
        lil_list_append(list, lil_clone_value(vi));
        lil_list_append(list, lil_clone_value(vd));
        uint64_t lsz = (uint64_t)lil_list_size(list);
        h = digest64(h, (const uint8_t *)&lsz, sizeof(lsz));
        lil_value_t item = lil_list_get(list, 1);
        h = digest64(h, (const uint8_t *)lil_to_string(item), strlen(lil_to_string(item)));
        lil_value_t joined = lil_list_to_value(list, 1);
        h = digest64(h, (const uint8_t *)lil_to_string(joined), strlen(lil_to_string(joined)));
        lil_free_value(joined);
        lil_free_list(list);

        /* variables + env */
        lil_set_var(lil, "answer", vi, LIL_SETVAR_GLOBAL);
        lil_value_t got = lil_get_var(lil, "answer");
        h = digest64(h, (const uint8_t *)lil_to_string(got), strlen(lil_to_string(got)));
        lil_value_t dflt = lil_get_var_or(lil, "missing", vd);
        h = digest64(h, (const uint8_t *)lil_to_string(dflt), strlen(lil_to_string(dflt)));
        lil_env_t pushed = lil_push_env(lil);
        (void)pushed;
        lil_set_var(lil, "inner", vs, LIL_SETVAR_LOCAL_NEW);
        lil_pop_env(lil);
        lil_env_t standalone = lil_alloc_env(NULL);
        lil_free_env(standalone);

        /* register + call native, expression eval, substitution */
        lil_register(lil, "natsum", native_sum);
        lil_value_t args[2] = { lil_alloc_integer(40), lil_alloc_integer(2) };
        lil_value_t called = lil_call(lil, "natsum", 2, args);
        h = digest64(h, (const uint8_t *)lil_to_string(called), strlen(lil_to_string(called)));
        lil_free_value(called);
        lil_free_value(args[0]);
        lil_free_value(args[1]);
        lil_value_t exprsrc = lil_alloc_string("(3 + 4) * 5 - 1");
        lil_value_t exprv = lil_eval_expr(lil, exprsrc);
        if (exprv) {
            h = digest64(h, (const uint8_t *)lil_to_string(exprv), strlen(lil_to_string(exprv)));
            lil_free_value(exprv);
        }
        lil_free_value(exprsrc);
        lil_value_t pv = lil_parse_value(lil, vs, 0);
        lil_free_value(pv);
        lil_value_t substsrc = lil_alloc_string("val is $answer");
        lil_value_t sv = lil_subst_to_value(lil, substsrc);
        if (sv) {
            h = digest64(h, (const uint8_t *)lil_to_string(sv), strlen(lil_to_string(sv)));
            lil_free_value(sv);
        }
        lil_list_t sl = lil_subst_to_list(lil, substsrc);
        if (sl) {
            uint64_t ssz = (uint64_t)lil_list_size(sl);
            h = digest64(h, (const uint8_t *)&ssz, sizeof(ssz));
            lil_free_list(sl);
        }
        lil_free_value(substsrc);

        /* error surface + misc */
        lil_set_error(lil, "synthetic error");
        const char *errmsg = NULL;
        size_t errpos = 0;
        uint64_t hade = (uint64_t)lil_error(lil, &errmsg, &errpos);
        h = digest64(h, (const uint8_t *)&hade, sizeof(hade));
        lil_set_error_at(lil, 7, "positioned error");
        (void)lil_error(lil, &errmsg, &errpos);
        lil_value_t un = lil_unused_name(lil, "tmp");
        if (un) {
            h = digest64(h, (const uint8_t *)lil_to_string(un), strlen(lil_to_string(un)));
            lil_free_value(un);
        }
        lil_write(lil, "written via lil_write");
        h = digest64(h, (const uint8_t *)&a.h, sizeof(a.h));

        lil_free_value(vs); lil_free_value(vi);
        lil_free_value(vd); lil_free_value(vc);
        lil_free(lil);
        lil_freemem(strdup("x"));   /* allocator-matched free helper */
    }
    printf("op=api_surface in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "usage: %s <run_script|api_surface> <input> <iters>\n", argv[0]);
        return 1;
    }
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }
    if (strcmp(argv[1], "run_script") == 0)       run_script(argv[2], iters);
    else if (strcmp(argv[1], "run_suite") == 0)   run_suite(argv[2], iters);
    else if (strcmp(argv[1], "api_surface") == 0) run_api_surface(iters);
    else { fprintf(stderr, "unknown op %s\n", argv[1]); return 1; }
    return 0;
}
