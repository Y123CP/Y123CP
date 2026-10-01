#!/usr/bin/env bash
# C reference build — libzahl (empirical study, self-contained).
# Mirrors study.toml [builds.c].build_cmd (copied from dataset_source/libzahl).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../bin/libzahl_cref17"
mkdir -p "$HERE/../bin"
( cd "$HERE" && /usr/bin/clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -I . -I src \
    -o "$OUT" libzahl_opsuite.c src/*.c )
echo "[c_ref] built $OUT"
