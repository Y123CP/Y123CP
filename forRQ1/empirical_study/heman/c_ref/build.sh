#!/usr/bin/env bash
# C reference build — heman (empirical study, self-contained; copied from dataset_source/heman).
# omp.h stub lives in ../shim (4-fn OpenMP stub); -lm for kazmath.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../bin/heman_cref17"
mkdir -p "$HERE/../bin"
( cd "$HERE" && /usr/bin/clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -Wno-unknown-pragmas \
    -I "$HERE/../shim" -I include -I . \
    -o "$OUT" \
    heman_pipeline_c.c src/*.c kazmath/*.c -lm )
echo "[c_ref] built $OUT"
