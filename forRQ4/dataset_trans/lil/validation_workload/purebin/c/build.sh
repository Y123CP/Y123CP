#!/usr/bin/env bash
                                                                
                                                       
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/lil"
D="$REPO/dataset_trans/lil/validation_workload/driver/driver.c"
OUT="$PB/bin/lil_c"
clang-17 -O3 -flto -falign-functions=64 -march=native -DNDEBUG -I"$S" \
    "$S/lil.c" "$D" -lm -o "$OUT"
echo "built: $OUT"
echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)   (应为 0)"
