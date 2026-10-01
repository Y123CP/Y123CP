/* lil_embedder.c — exercises lil's C-API entry points that the standalone
 * lil CLI doesn't reach. Compile alongside lil.c (NOT main.c).
 *
 * Hits:
 *   lil_new / lil_free
 *   lil_parse / lil_parse_value
 *   lil_call / lil_callback / lil_register
 *   lil_set_error / lil_error
 *   lil_to_string / lil_to_double / lil_to_integer / lil_to_boolean
 *   lil_alloc_string / lil_alloc_string_len / lil_alloc_double / lil_alloc_integer
 *   lil_append_char / lil_append_string / lil_append_string_len / lil_append_val
 *   lil_clone_value / lil_free_value
 *   lil_alloc_list / lil_free_list / lil_list_append / lil_list_size /
 *     lil_list_get / lil_list_to_value
 *   lil_subst_to_list / lil_subst_to_value
 *   lil_alloc_env / lil_free_env / lil_push_env / lil_pop_env
 *   lil_set_var / lil_get_var / lil_get_var_or
 *   lil_eval_expr
 *   lil_unused_name
 *   lil_arg
 *   lil_set_data / lil_get_data
 *   lil_embedded / lil_freemem
 *   lil_write
 *
 * Reads /dev/null or ignores argv (no-op when no path).
 */
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lil.h"

/* Forward-declare a few functions defined in lil.c but missing from lil.h. */
extern lil_value_t lil_alloc_string_len(const char *str, size_t len);
extern int lil_append_string_len(lil_value_t val, const char *s, size_t len);

static int callback_seen = 0;

/* lil callback that just counts invocations. */
static void my_write_cb(lil_t lil, const char *msg) {
	(void)lil; (void)msg;
	callback_seen++;
}

static void my_error_cb(lil_t lil, size_t pos, const char *msg) {
	(void)lil; (void)pos; (void)msg;
	callback_seen++;
}

/* Custom registered function: returns argc as integer. */
static lil_value_t fn_argcount(lil_t lil, size_t argc, lil_value_t *argv) {
	(void)argv;
	return lil_alloc_integer((lilint_t)argc);
}

/* Custom registered function: returns first arg (uses lil_arg). */
static lil_value_t fn_first(lil_t lil, size_t argc, lil_value_t *argv) {
	(void)lil; (void)argc;
	return lil_clone_value(lil_arg(argv, 0));
}

int main(int argc, char **argv) {
	(void)argc; (void)argv;

	/* 1) lil_new + register callbacks */
	lil_t lil = lil_new();
	if (!lil) return 1;
	lil_callback(lil, LIL_CALLBACK_WRITE, (lil_callback_proc_t)my_write_cb);
	lil_callback(lil, LIL_CALLBACK_ERROR, (lil_callback_proc_t)my_error_cb);

	/* user data round-trip */
	int my_data = 99;
	lil_set_data(lil, &my_data);
	void *got = lil_get_data(lil);
	if (got != &my_data) fprintf(stderr, "set/get_data mismatch\n");

	/* 2) lil_register: add custom funcs */
	lil_register(lil, "argcount", fn_argcount);
	lil_register(lil, "first", fn_first);

	/* 3) lil_parse: run some lil code */
	const char *code =
		"set greeting \"hello\"\n"
		"print $greeting\n"
		"set n [argcount 1 2 3 4 5]\n"
		"set fst [first alpha beta]\n"
		"set sum [expr 1 + 2 + 3]\n";
	lil_value_t r = lil_parse(lil, code, strlen(code), 0);
	lil_free_value(r);

	/* 4) lil_call with explicit argv */
	lil_value_t cargv[3];
	cargv[0] = lil_alloc_string("a");
	cargv[1] = lil_alloc_string("b");
	cargv[2] = lil_alloc_integer(7);
	lil_value_t cres = lil_call(lil, "argcount", 3, cargv);
	lil_free_value(cres);
	for (int i = 0; i < 3; ++i) lil_free_value(cargv[i]);

	/* 5) lil_get_var / lil_set_var / lil_get_var_or */
	lil_value_t v = lil_get_var(lil, "greeting");
	const char *s = lil_to_string(v);
	(void)s;
	lil_value_t defv = lil_alloc_string("fallback");
	lil_value_t v2 = lil_get_var_or(lil, "nonexistent", defv);
	(void)v2;
	lil_free_value(defv);
	lil_value_t newv = lil_alloc_integer(42);
	lil_set_var(lil, "answer", newv, LIL_SETVAR_GLOBAL);
	lil_free_value(newv);

	/* 6) lil_alloc_* */
	lil_value_t s1 = lil_alloc_string("hello");
	lil_value_t s2 = lil_alloc_string_len("world\0skip", 5);
	lil_value_t d1 = lil_alloc_double(3.14);
	lil_value_t i1 = lil_alloc_integer(123);

	/* 7) lil_append_* */
	lil_append_char(s1, '!');
	lil_append_string(s1, " more");
	lil_append_string_len(s1, "1234567890", 5);
	lil_append_val(s1, s2);

	/* 8) lil_to_* */
	(void)lil_to_string(s1);
	(void)lil_to_double(d1);
	(void)lil_to_integer(i1);
	(void)lil_to_boolean(s1);

	/* 9) clone */
	lil_value_t clone = lil_clone_value(s1);
	lil_free_value(clone);

	/* 10) list ops */
	lil_list_t lst = lil_alloc_list();
	lil_list_append(lst, lil_alloc_string("zero"));
	lil_list_append(lst, lil_alloc_integer(1));
	lil_list_append(lst, lil_alloc_double(2.5));
	size_t sz = lil_list_size(lst);
	for (size_t i = 0; i < sz; ++i) {
		lil_value_t e = lil_list_get(lst, i);
		(void)e;
	}
	lil_value_t list_as_value = lil_list_to_value(lst, 1);
	(void)list_as_value;

	/* lil_subst_to_list / lil_subst_to_value: take a code value, substitute
	 * variables, return list/value. */
	lil_value_t code_val = lil_alloc_string("$greeting world");
	lil_list_t substl = lil_subst_to_list(lil, code_val);
	lil_value_t substv = lil_subst_to_value(lil, code_val);
	if (substl) lil_free_list(substl);
	lil_free_value(substv);
	lil_free_value(code_val);

	/* 11) environment ops */
	lil_env_t saved = lil_push_env(lil);
	lil_set_var(lil, "in_inner", lil_alloc_string("yes"), LIL_SETVAR_LOCAL_NEW);
	lil_pop_env(lil);
	(void)saved;
	lil_env_t freestanding = lil_alloc_env(NULL);
	lil_free_env(freestanding);

	/* 12) lil_eval_expr */
	lil_value_t expr_code = lil_alloc_string("3 + 4 * 5");
	lil_value_t expr_result = lil_eval_expr(lil, expr_code);
	(void)expr_result;
	lil_free_value(expr_code);
	lil_free_value(expr_result);

	/* 13) lil_unused_name */
	lil_value_t un = lil_unused_name(lil, "mypfx");
	(void)un;
	lil_free_value(un);

	/* 14) lil_set_error / lil_error */
	lil_set_error(lil, "synthetic error message");
	const char *errmsg = NULL;
	size_t errpos = 0;
	int has_err = lil_error(lil, &errmsg, &errpos);
	(void)has_err;
	lil_set_error_at(lil, 5, "at-position error");
	(void)lil_error(lil, &errmsg, &errpos);

	/* 15) lil_parse_value */
	lil_value_t cv = lil_alloc_string("print embedded-parse-value\nset xyz 99\n");
	lil_value_t pv = lil_parse_value(lil, cv, 0);
	lil_free_value(cv);
	lil_free_value(pv);

	/* 16) lil_embedded - runs a code string in a transient interpreter */
	char *embedded_result = lil_embedded(lil, "expr 7 + 8", LIL_EMBED_NOFLAGS);
	if (embedded_result) lil_freemem(embedded_result);

	/* 17) lil_write */
	lil_write(lil, "direct lil_write hello\n");

	/* Cleanup */
	lil_free_value(s1);
	lil_free_value(s2);
	lil_free_value(d1);
	lil_free_value(i1);
	lil_free_list(lst);
	lil_free(lil);

	fprintf(stderr, "callback_seen=%d\n", callback_seen);
	return 0;
}
