#!/usr/bin/env bash
# C reference build — json-c (empirical study, self-contained; copied from dataset_source/json-c).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../bin/jsonc_cref17"
mkdir -p "$HERE/../bin"
( cd "$HERE" && /usr/bin/clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -D_GNU_SOURCE -I _build -I . \
    -o "$OUT" \
    jsonc_length_keys.c arraylist.c debug.c json_c_version.c json_object.c \
    json_object_iterator.c json_patch.c json_pointer.c json_tokener.c json_util.c \
    json_visit.c libjson.c linkhash.c printbuf.c random_seed.c strerror_override.c -lm )
echo "[c_ref] built $OUT"
