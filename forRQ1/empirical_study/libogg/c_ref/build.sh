#!/usr/bin/env bash
# C reference build — libogg (empirical study, self-contained; copied from dataset_source/libogg).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../bin/ogg_cref17"
mkdir -p "$HERE/../bin"
( cd "$HERE" && /usr/bin/clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -I include -I build/include \
    -o "$OUT" ogg_encode.c src/bitwise.c src/framing.c )
echo "[c_ref] built $OUT"
