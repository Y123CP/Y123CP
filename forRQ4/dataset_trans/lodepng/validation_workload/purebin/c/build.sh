#!/usr/bin/env bash
                                                    
                                                                        
                                              
                                                       
                        
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/lodepng"
D="$HERE/driver.c"
OUT="$PB/bin/lodepng_c"
clang-17 -x c -O3 -flto -falign-functions=64 -march=native -DNDEBUG -I"$S" "$S/lodepng.cpp" "$D" -lm -o "$OUT"
echo "built: $OUT"
echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)   (应为 0)"
