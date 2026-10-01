#!/usr/bin/env bash
# C reference build — xxHash (empirical study, self-contained).
# Mirrors study.toml [builds.c].build_cmd. Run from repo root OR this dir.
# FAIRNESS: builds from the ORIGINAL xxhash.h (COMPILER_GUARD + ASSUME intact).
#   xxhash.h.patched_for_c2rust is provenance only (how 0_raw was transpiled) —
#   NEVER build C from it (removing the guard cripples C to IPC~0.96 = fake gap).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../bin/xxh_cref17"
mkdir -p "$HERE/../bin"
/usr/bin/clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -DXXH_VECTOR=0 \
  -DXXH_STATIC_LINKING_ONLY -I "$HERE" -o "$OUT" \
  "$HERE/xxh_bench.c" "$HERE/xxhash.c"
echo "[c_ref] built $OUT"
