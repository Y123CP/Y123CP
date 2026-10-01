#!/usr/bin/env bash
                                                  
                                                             
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/miniz"
D="$REPO/dataset_trans/miniz/validation_workload/driver/driver.c"
OUT="$PB/bin/miniz_c"
clang-17 -O3 -flto -falign-functions=64 -march=native -DNDEBUG -I"$S" -I"$S/build" \
    "$S/miniz.c" "$S/miniz_tdef.c" "$S/miniz_tinfl.c" "$S/miniz_zip.c" "$D" \
    -o "$OUT"
echo "built: $OUT"
echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)   (应为 0)"
