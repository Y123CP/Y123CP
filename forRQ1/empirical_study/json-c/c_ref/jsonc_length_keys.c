/* jsonc_length_keys — C reference for the Rust harness
 *   dataset_trans_process/json-c/workloads/harness/json_roundtrip/src/main.rs
 * Output format must match exactly: `length=<N> keys=<M>\n`.
 *
 *   length = strlen(json_object_to_json_string(root))
 *   keys   = json_object_object_length(root)
 *
 *   Usage: jsonc_length_keys <input.json>
 *   Build: gcc -O2 -I_build -I. -o jsonc_length_keys \
 *              jsonc_length_keys.c _build/libjson-c.a
 */
#include "json.h"
#include "json_visit.h"
#include "json_patch.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>

/* OPERATION 2 — COV: exercise the broad json-c public API (all json_object types,
 * array/object ops, iterator, pointer, patch, visit, tokener flags, deep_copy,
 * version) so fn coverage reaches ≥85%. Deterministic; C and Rust MUST match. */
static int cov_visit(struct json_object *jso, int flags, struct json_object *parent,
                     const char *key, size_t *idx, void *data) {
    (void)jso; (void)flags; (void)parent; (void)key; (void)idx; (void)data;
    return JSON_C_VISIT_RETURN_CONTINUE;
}
static int cov_arr_cmp(const void *a, const void *b) {
    struct json_object *ja = *(struct json_object *const *)a;
    struct json_object *jb = *(struct json_object *const *)b;
    return json_object_get_int(ja) - json_object_get_int(jb);
}
static int cov_ser(struct json_object *jso, struct printbuf *pb, int level, int flags) {
    (void)jso; (void)level; (void)flags;
    return printbuf_memappend(pb, "\"X\"", 3);
}

static void cov_workload(void) {
    const char *ver = json_c_version();
    int vn = json_c_version_num();

    struct json_object *obj = json_object_new_object();
    json_object_object_add(obj, "i", json_object_new_int(42));
    json_object_object_add(obj, "i64", json_object_new_int64(1234567890123LL));
    json_object_object_add(obj, "u64", json_object_new_uint64(9876543210ULL));
    json_object_object_add(obj, "d", json_object_new_double(3.14159));
    json_object_object_add(obj, "b", json_object_new_boolean(1));
    json_object_object_add(obj, "s", json_object_new_string("hello"));
    json_object_object_add(obj, "sl", json_object_new_string_len("world", 5));
    json_object_object_add(obj, "nul", NULL);

    struct json_object *arr = json_object_new_array();
    for (int i = 0; i < 5; i++) json_object_array_add(arr, json_object_new_int(i));
    json_object_array_put_idx(arr, 5, json_object_new_int(99)); /* contiguous, no hole */
    json_object_object_add(obj, "arr", arr);

    int keys = json_object_object_length(obj);
    size_t alen = json_object_array_length(arr);
    struct json_object *got = NULL;
    json_object_object_get_ex(obj, "i", &got);
    int iv = json_object_get_int(got);
    int64_t i64v = json_object_get_int64(json_object_object_get(obj, "i64"));
    uint64_t u64v = json_object_get_uint64(json_object_object_get(obj, "u64"));
    double dv = json_object_get_double(json_object_object_get(obj, "d"));
    const char *sv = json_object_get_string(json_object_object_get(obj, "s"));
    json_bool bv = json_object_get_boolean(json_object_object_get(obj, "b"));
    enum json_type t = json_object_get_type(obj);
    const char *tn = json_type_to_name(t);
    int isarr = json_object_is_type(arr, json_type_array);

    const char *s1 = json_object_to_json_string(obj);
    const char *s2 = json_object_to_json_string_ext(obj, JSON_C_TO_STRING_PRETTY | JSON_C_TO_STRING_SPACED);
    size_t l1 = s1 ? strlen(s1) : 0, l2 = s2 ? strlen(s2) : 0;

    struct json_object_iterator it = json_object_iter_begin(obj);
    struct json_object_iterator ie = json_object_iter_end(obj);
    int itcount = 0;
    while (!json_object_iter_equal(&it, &ie)) {
        (void)json_object_iter_peek_name(&it);
        (void)json_object_iter_peek_value(&it);
        json_object_iter_next(&it);
        itcount++;
    }

    struct json_object *pget = NULL;
    int prc = json_pointer_get(obj, "/arr/0", &pget);
    json_pointer_set(&obj, "/newkey", json_object_new_int(7));

    int visited = json_c_visit(obj, 0, cov_visit, NULL);

    struct json_object *cp = NULL;
    int dcrc = json_object_deep_copy(obj, &cp, json_c_shallow_copy_default);
    int eq = json_object_equal(obj, cp);

    struct json_tokener *tok = json_tokener_new();
    json_tokener_set_flags(tok, JSON_TOKENER_STRICT);
    struct json_object *parsed = json_tokener_parse_ex(tok, "{\"a\":1}", 7);
    enum json_tokener_error terr = json_tokener_get_error(tok);
    const char *tdesc = json_tokener_error_desc(terr);
    (void)tdesc;
    json_object_put(parsed);
    json_tokener_free(tok);

    struct json_object *patch = json_tokener_parse("[{\"op\":\"add\",\"path\":\"/pz\",\"value\":5}]");
    struct json_object *patched = NULL;
    int patchrc = json_patch_apply(obj, patch, &patched, NULL);

    struct json_object *ref = json_object_get(obj);
    json_object_put(ref);

    /* --- batch 2: setters, array ops, file I/O, patch ops, pointer printf --- */
    struct json_object *sv2 = json_object_new_int(1);
    json_object_set_int(sv2, 100);
    json_object_set_int64(sv2, 200);
    json_object_set_uint64(sv2, 300);
    json_object_set_double(sv2, 2.5);
    json_object_set_boolean(sv2, 0);
    json_object_set_string(sv2, "changed");
    json_object_put(sv2);
    json_object_object_add_ex(obj, "ex", json_object_new_int(9), 0);
    json_object_object_del(obj, "ex");
    (void)json_object_get_object(obj);
    (void)json_object_get_array(arr);
    json_object_array_del_idx(arr, 0, 1);
    json_object_to_file((char *)"/tmp/jc_cov.json", obj);
    json_object_to_file_ext((char *)"/tmp/jc_cov2.json", obj, JSON_C_TO_STRING_PRETTY);
    struct json_object *fromf = json_object_from_file("/tmp/jc_cov.json");
    if (fromf) json_object_put(fromf);
    struct json_object *bad = json_object_from_file("/nonexistent_dir_xyz/none.json");
    if (bad) json_object_put(bad); /* error path → _json_c_strerror */
    struct json_object *pg2 = NULL;
    json_pointer_getf(obj, &pg2, "/arr/%d", 0);
    json_pointer_setf(&obj, json_object_new_int(1), "/kf%d", 5);
    struct json_object *patch2 = json_tokener_parse(
        "[{\"op\":\"replace\",\"path\":\"/i\",\"value\":2},"
        "{\"op\":\"copy\",\"from\":\"/i\",\"path\":\"/ic\"},"
        "{\"op\":\"move\",\"from\":\"/ic\",\"path\":\"/im\"},"
        "{\"op\":\"test\",\"path\":\"/i\",\"value\":2},"
        "{\"op\":\"remove\",\"path\":\"/im\"}]");
    struct json_object *patched2 = NULL;
    json_patch_apply(obj, patch2, &patched2, NULL);
    json_object_put(patch2);
    json_object_put(patched2);
    enum json_tokener_error jerr2;
    struct json_object *tv = json_tokener_parse_verbose("{\"x\":1}", &jerr2);
    if (tv) json_object_put(tv);
    /* --- batch 3: array sort/bsearch/insert, serialization, int_inc --- */
    json_object_array_sort(arr, cov_arr_cmp);
    struct json_object *skey = json_object_new_int(2);
    (void)json_object_array_bsearch(skey, arr, cov_arr_cmp);
    json_object_put(skey);
    json_object_array_insert_idx(arr, 1, json_object_new_int(55));
    struct json_object *ii = json_object_new_int(10);
    json_object_int_inc(ii, 5);
    json_object_put(ii);
    (void)json_object_get_string_len(json_object_object_get(obj, "s"));
    /* --- batch 4: fd-based file I/O + debug toggles --- */
    mc_set_debug(1);
    (void)mc_get_debug();
    mc_set_debug(0);
    int fd = open("/tmp/jc_cov.json", O_RDONLY);
    if (fd >= 0) {
        struct json_object *ff = json_object_from_fd(fd);
        if (ff) json_object_put(ff);
        close(fd);
    }
    /* --- batch 5: custom serializer + userdata, double_s, array_ext --- */
    struct json_object *so = json_object_new_int(1);
    json_object_set_serializer(so, cov_ser, strdup("ud"), json_object_free_userdata);
    (void)json_object_get_userdata(so);
    (void)json_object_to_json_string(so);
    json_object_put(so);
    struct json_object *ds = json_object_new_double_s(1.5, "1.5");
    json_object_put(ds);
    struct json_object *ae = json_object_new_array_ext(4);
    json_object_put(ae);

    printf("cov ver=%s vn=%d keys=%d alen=%zu iv=%d i64=%lld u64=%llu dv=%.3f sv=%s bv=%d "
           "tn=%s isarr=%d l1=%zu l2=%zu itc=%d prc=%d vis=%d dc=%d eq=%d patch=%d terr=%d\n",
           ver, vn, keys, alen, iv, (long long)i64v, (unsigned long long)u64v, dv, sv, bv,
           tn, isarr, l1, l2, itcount, prc, visited, dcrc, eq, patchrc, (int)terr);

    json_object_put(cp);
    json_object_put(patch);
    json_object_put(patched);
    json_object_put(obj);
}

/* OPERATION split: PARSE (text→tree) and SERIALIZE (tree→text) are separate hot
 * paths — measured independently. Each loops N times so runtime ≥ ~1s (also fixes
 * the citm/twitter <100ms CV warnings). C and Rust MUST print identical stdout. */
#define JC_N 20
#define JC_N_SER 80   /* serialize ~3x faster/iter than parse -> scale to clear 100ms floor even on twitter (digest=length, loop-independent) */
static void parse_workload(const char *buf) {
    long keys = 0;
    for (int i = 0; i < JC_N; i++) {
        struct json_object *root = json_tokener_parse(buf);
        if (!root) { fprintf(stderr, "parse NULL\n"); exit(2); }
        keys = json_object_object_length(root);
        json_object_put(root);
    }
    printf("keys=%ld\n", keys);
}
static void serialize_workload(const char *buf) {
    struct json_object *root = json_tokener_parse(buf);
    if (!root) { fprintf(stderr, "parse NULL\n"); exit(2); }
    size_t len = 0;
    for (int i = 0; i < JC_N_SER; i++) {
        const char *s = json_object_to_json_string(root);
        len = s ? strlen(s) : 0;
    }
    printf("length=%zu\n", len);
    json_object_put(root);
}

int main(int argc, char *argv[]) {
    if (argc >= 2 && strcmp(argv[1], "cov") == 0) {
        cov_workload();
        return 0;
    }
    if (argc < 2) {
        fprintf(stderr, "usage: %s <input.json> [parse|serialize] | cov\n", argv[0]);
        return 1;
    }
    const char *mode = (argc > 2) ? argv[2] : "roundtrip";
    if (strcmp(mode, "parse") == 0 || strcmp(mode, "serialize") == 0) {
        FILE *ff = fopen(argv[1], "rb");
        if (!ff) { fprintf(stderr, "open %s\n", argv[1]); return 1; }
        fseek(ff, 0, SEEK_END); long nn = ftell(ff); fseek(ff, 0, SEEK_SET);
        char *bb = malloc((size_t)nn + 1);
        if (fread(bb, 1, (size_t)nn, ff) != (size_t)nn) { fclose(ff); free(bb); return 1; }
        bb[nn] = '\0'; fclose(ff);
        if (strcmp(mode, "parse") == 0) parse_workload(bb);
        else serialize_workload(bb);
        free(bb);
        return 0;
    }
    FILE *f = fopen(argv[1], "rb");
    if (!f) { fprintf(stderr, "open %s: %s\n", argv[1], strerror(errno)); return 1; }
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    if (n < 0) { fclose(f); return 1; }
    fseek(f, 0, SEEK_SET);
    char *buf = malloc((size_t)n + 1);
    if (!buf) { fclose(f); return 1; }
    if (fread(buf, 1, (size_t)n, f) != (size_t)n) { fclose(f); free(buf); return 1; }
    buf[n] = '\0';
    fclose(f);

    struct json_object *root = json_tokener_parse(buf);
    if (!root) {
        fprintf(stderr, "json_tokener_parse returned NULL\n");
        free(buf);
        return 2;
    }
    int keys = json_object_object_length(root);
    const char *s = json_object_to_json_string(root);
    size_t slen = s ? strlen(s) : 0;
    printf("length=%zu keys=%d\n", slen, keys);
    json_object_put(root);
    free(buf);
    return 0;
}
