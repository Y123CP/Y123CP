#!/usr/bin/env bash
                                                                
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/libqrencode"
D="$REPO/dataset_trans/libqrencode/validation_workload/driver/driver.c"
OUT="$PB/bin/libqrencode_c"
DEFS='-DSTATIC_IN_RELEASE= -DMAJOR_VERSION=4 -DMINOR_VERSION=1 -DMICRO_VERSION=1 -DVERSION="4.1.1"'
clang-17 -O3 -flto -falign-functions=64 -march=native -DNDEBUG $DEFS -I"$S" \
  "$S"/bitstream.c "$S"/mask.c "$S"/mmask.c "$S"/mqrspec.c "$S"/qrencode.c \
  "$S"/qrinput.c "$S"/qrspec.c "$S"/rsecc.c "$S"/split.c "$D" -lpthread -lm -o "$OUT"
echo "built: $OUT"; echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)"
