#!/usr/bin/env bash
                                             
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/zopfli/src/zopfli"
D="$REPO/dataset_trans/zopfli/validation_workload/driver/driver.c"
OUT="$PB/bin/zopfli_c"
clang-17 -O3 -flto -falign-functions=64 -march=native -DNDEBUG -I"$S" \
  "$S"/blocksplitter.c "$S"/cache.c "$S"/deflate.c "$S"/gzip_container.c "$S"/hash.c \
  "$S"/katajainen.c "$S"/lz77.c "$S"/squeeze.c "$S"/tree.c "$S"/util.c \
  "$S"/zlib_container.c "$S"/zopfli_lib.c "$D" -lm -o "$OUT"
echo "built: $OUT"; echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)"
