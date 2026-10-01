#!/usr/bin/env bash
                                                     
                                                                
                                                     
                                                       
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; PB="$(dirname "$HERE")"
REPO="$(cd "$PB/../../../.." && pwd)"
S="$REPO/dataset_source/libxml2"
D="$REPO/dataset_trans/libxml2/validation_workload/driver/driver.c"
OUT="$PB/bin/libxml2_c"
MODS='buf c14n catalog chvalid debugXML dict encoding entities error globals hash HTMLparser HTMLtree legacy list nanoftp nanohttp parserInternals parser pattern relaxng SAX2 SAX schematron threads tree uri valid xinclude xlink xmlIO xmlmemory xmlmodule xmlreader xmlregexp xmlsave xmlschemas xmlschemastypes xmlstring xmlunicode xmlwriter xpath xpointer xzlib'
clang-17 -O3 -flto -falign-functions=64 -march=native -DNDEBUG -D_REENTRANT -I"$S/_build" -I"$S/include" \
    $(for m in $MODS; do echo "$S/$m.c"; done) "$D" -lm -lz -ldl -lpthread -o "$OUT"
echo "built: $OUT"
echo "zmm count: $(objdump -d "$OUT" | grep -c zmm)   (应为 0)"
