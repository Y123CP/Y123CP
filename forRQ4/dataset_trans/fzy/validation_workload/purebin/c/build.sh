#!/usr/bin/env bash
                                           
                                                                    
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/fzy/src"
D="$REPO/dataset_trans/fzy/validation_workload/driver/driver.c"
OUT="$PB/bin/fzy_c"
clang-17 -O3 -flto -falign-functions=64 -march=native -ffp-contract=off -DNDEBUG -DVERSION='"1.0"' -I"$S" \
    "$S/match.c" "$S/choices.c" "$S/options.c" "$S/tty.c" "$D" -lpthread -lm -o "$OUT"
echo "built: $OUT"
echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)   (应为 0)"
