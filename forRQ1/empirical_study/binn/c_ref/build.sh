#!/usr/bin/env bash
# C reference build — binn (empirical study, self-contained; copied from dataset_source/binn/src).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../bin/binn_cref17"
mkdir -p "$HERE/../bin"
( cd "$HERE" && /usr/bin/clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -DBINN_NO_COMPRESS -I . \
    -o "$OUT" binn_roundtrip.c binn.c )
echo "[c_ref] built $OUT"
