#!/usr/bin/env bash
                                              
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/http-parser"
D="$REPO/dataset_trans/http-parser/validation_workload/driver/driver.c"
OUT="$PB/bin/http_parser_c"
clang-17 -O3 -flto -falign-functions=64 -march=native -DNDEBUG -I"$S" "$S/http_parser.c" "$D" -o "$OUT"
echo "built: $OUT"; echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)"
