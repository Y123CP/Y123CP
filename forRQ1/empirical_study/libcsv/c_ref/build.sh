#!/usr/bin/env bash
# C reference build — libcsv (empirical study, self-contained).
# main.c = the csv_count driver (was draft/gap_analyze/harness/libcsv/c/main.c);
# libcsv.c + csv.h copied from dataset_source/libcsv.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../bin/csv_count_cref17"
mkdir -p "$HERE/../bin"
( cd "$HERE" && /usr/bin/clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -I . \
    -o "$OUT" main.c libcsv.c )
echo "[c_ref] built $OUT"
