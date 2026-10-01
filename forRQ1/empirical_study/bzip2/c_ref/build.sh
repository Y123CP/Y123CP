#!/usr/bin/env bash
# C reference build — bzip2 (empirical study, self-contained; copied from dataset_source/bzip2-1.0.8).
# Whole-program clang-17 LTO over bzip2.c + 7 lib .c (test/util mains excluded).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT="$HERE/../bin/bzip2_cref17"
mkdir -p "$HERE/../bin"
( cd "$HERE" && /usr/bin/clang-17 -O3 -flto -march=native -mno-avx512f -DNDEBUG -D_FILE_OFFSET_BITS=64 \
    -Wno-unused-result \
    bzip2.c blocksort.c huffman.c crctable.c randtable.c compress.c decompress.c bzlib.c \
    -o "$OUT" )
echo "[c_ref] built $OUT"
