#!/usr/bin/env bash
                                             
                                                      
                                                       
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"          # .../purebin/c
PB="$(dirname "$HERE")"                          # .../purebin
REPO="$(cd "$PB/../../../.." && pwd)"            # repo root
S="$REPO/dataset_source/libopenaptx"
D="$REPO/dataset_trans/libopenaptx/validation_workload/driver/driver.c"
OUT="$PB/bin/libopenaptx_c"

clang-17 -O3 -flto -falign-functions=64 -march=native -DNDEBUG -I"$S" "$S/openaptx.c" "$D" -lm -o "$OUT"
echo "built: $OUT"
echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)   (clang 默认 prefer-vector-width=256 → 应为 0)"
