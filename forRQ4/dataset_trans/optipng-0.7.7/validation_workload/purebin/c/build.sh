#!/usr/bin/env bash
                                                           
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/optipng-0.7.7/src"
OUT="$PB/bin/optipng_c"
clang-17 -O3 -flto -falign-functions=64 -march=native -DNDEBUG -Wno-deprecated-non-prototype \
  -I"$S/cexcept" -I"$S/libpng" -I"$S/zlib" -I"$S/opngreduc" -I"$S/pngxtern" \
  -I"$S/pnmio" -I"$S/minitiff" -I"$S/gifread" \
  "$S"/optipng/*.c "$S"/opngreduc/*.c "$S"/pngxtern/*.c "$S"/pnmio/*.c \
  "$S"/minitiff/*.c "$S"/gifread/*.c $(ls "$S"/libpng/*.c | grep -v pngtest) \
  "$S"/zlib/*.c -lm -o "$OUT"
echo "built: $OUT"; echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)"
