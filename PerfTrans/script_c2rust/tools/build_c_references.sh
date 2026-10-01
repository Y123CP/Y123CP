#!/usr/bin/env bash
# Build C reference binaries for all 14 benchmarks.
# Idempotent — skips already-built binaries.
set -u
REPO="/home/anonymous/artifact/PerfTrans"
SOURCE="$REPO/dataset_source"
LOG="$REPO/dataset_trans_process/transpile_log/c_build.log"
: > "$LOG"

# Make local cmake available.
export PATH="$REPO/c2rust-thinking/local/src/cmake-3.27.9-linux-x86_64/bin:$PATH"

ok()   { printf "[ OK ] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }
fail() { printf "[FAIL] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }
skip() { printf "[SKIP] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }

build_proj() {
    local name="$1" build_cmd="$2" output_check="$3"
    cd "$SOURCE/$name" 2>/dev/null || { skip "$name" "source dir missing"; return; }
    if eval "$output_check" >/dev/null 2>&1; then
        ok "$name" "already built"
        return
    fi
    echo "==[ $name ]==" >> "$LOG"
    eval "$build_cmd" >> "$LOG" 2>&1 &&
        ok "$name" "built" ||
        fail "$name" "see log"
}

# 1. binn — library only. We build a small roundtrip driver harness.c here
#    that exercises read+write+nesting paths.
build_proj binn \
    'cd src && gcc -O3 -fPIC -DBINN_NO_COMPRESS -c binn.c -o binn.o && ar rcs libbinn.a binn.o && \
     if [ ! -f binn_roundtrip.c ]; then cat > binn_roundtrip.c <<EOF
#include "binn.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
int main(int argc, char *argv[]) {
    FILE *f = (argc>=2) ? fopen(argv[1],"rb") : stdin;
    if (!f) return 2;
    char buf[8192]; size_t cap=65536,n=0; char *p=malloc(cap);
    while (1){ size_t r=fread(buf,1,sizeof(buf),f); if(!r) break;
      if(n+r>cap){cap*=2;p=realloc(p,cap);} memcpy(p+n,buf,r); n+=r; }
    if (argc>=2) fclose(f);
    binn *obj = binn_object();
    binn_object_set_str(obj, "data", p);
    binn_object_set_int32(obj, "len", (int)n);
    binn *arr = binn_list();
    for (int i = 0; i < 64; i++) binn_list_add_int32(arr, i);
    binn_object_set_list(obj, "arr", arr);
    void *out = binn_ptr(obj);
    int sz = binn_size(obj);
    fwrite(out, 1, sz, stdout);
    binn_free(arr); binn_free(obj); free(p);
    return 0;
}
EOF
     fi && \
     gcc -O3 -o binn_roundtrip binn_roundtrip.c binn.o' \
    'test -x src/binn_roundtrip'

# 2. brotli — cmake _build exists.
build_proj brotli \
    'cd _build && cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_EXPORT_COMPILE_COMMANDS=ON .. >/dev/null && cmake --build . --target brotli -j' \
    'test -x _build/brotli'

# 3. bzip2 — already built.
build_proj bzip2-1.0.8 \
    'make CFLAGS="-O3 -fPIC -Wall" -j bzip2' \
    'test -x bzip2'

# 4. heman — cmake.
build_proj heman \
    'mkdir -p _build && cd _build && cmake -DCMAKE_BUILD_TYPE=Release .. && make -j' \
    'test -f _build/src/libheman.a'

# 5. json-c — cmake static.
build_proj json-c \
    'mkdir -p _build && cd _build && cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF .. && make -j' \
    'test -f _build/libjson-c.a -o -f _build/libjson-c.so'

# 6. json_h — driver.
build_proj json_h \
    'gcc -O3 -flto -march=native -o json_h_driver json_h_driver.c' \
    'test -x json_h_driver'

# 7. libcsv — autotools; bypass cwd-check + aclocal-1.14 timestamp regen.
build_proj libcsv \
    'if [ ! -f Makefile ]; then \
       sed -i.bak "s/as_fn_error \$? \"working directory cannot be determined\"/echo \"[patched] cwd check bypassed\"/" configure 2>/dev/null || true; \
       sed -i "s/as_fn_error \$? \"pwd does not report name of working directory\"/echo \"[patched] pwd check bypassed\"/" configure 2>/dev/null || true; \
       ./configure --enable-static --disable-shared; \
     fi && \
     touch aclocal.m4 Makefile.in configure config.status libcsv-config.in 2>/dev/null; \
     make -j AUTOCONF=true ACLOCAL=true AUTOMAKE=true AUTOHEADER=true' \
    'test -f .libs/libcsv.a -o -f libcsv.la -o -f libcsv.a'

# 8. libtree — plain Makefile at root.
build_proj libtree \
    'make CFLAGS="-O3 -std=c99 -Wall" -j && mkdir -p _build && cp libtree _build/' \
    'test -x _build/libtree'

# 9. libxml2 — cmake (xmllint binary).
build_proj libxml2 \
    'mkdir -p _build && cd _build && cmake -DCMAKE_BUILD_TYPE=Release -DLIBXML2_WITH_PYTHON=OFF -DLIBXML2_WITH_ICONV=OFF -DLIBXML2_WITH_ICU=OFF -DLIBXML2_WITH_LZMA=OFF -DLIBXML2_WITH_ZLIB=OFF -DLIBXML2_WITH_TESTS=OFF -DLIBXML2_WITH_PROGRAMS=ON .. && make -j xmllint' \
    'test -x _build/xmllint'

# 10. libzahl — Makefile builds .a + pdf docs (latex). Build just .a.
build_proj libzahl \
    'make CC=gcc CFLAGS="-O3 -fPIC -Wall" -j libzahl.a' \
    'test -f libzahl.a'

# 11. lil — sources at TOP LEVEL, has Makefile (older form). We compile
#     directly: gcc lil.c main.c -lm -O3 -o lil
build_proj lil \
    'gcc -O3 -o lil lil.c main.c -lm' \
    'test -x lil'

# 12. lodepng — write our own decode→encode roundtrip driver (lodepng_benchmark.cpp
#     needs SDL2 — skip it). Driver reads PNG path from argv, decodes, re-encodes,
#     writes to stdout.
build_proj lodepng \
    'if [ ! -f lodepng_perf_driver.c ]; then cat > lodepng_perf_driver.c <<EOF
#include "lodepng.h"
#include <stdio.h>
#include <stdlib.h>
int main(int argc, char *argv[]) {
    if (argc < 2) { fprintf(stderr, "usage: %s <png>\n", argv[0]); return 1; }
    unsigned char *image = 0; unsigned w, h;
    unsigned err = lodepng_decode32_file(&image, &w, &h, argv[1]);
    if (err) { fprintf(stderr, "decode: %s\n", lodepng_error_text(err)); return 2; }
    unsigned char *out = 0; size_t outsize = 0;
    err = lodepng_encode32(&out, &outsize, image, w, h);
    if (err) { fprintf(stderr, "encode: %s\n", lodepng_error_text(err)); return 3; }
    fwrite(out, 1, outsize, stdout);
    free(image); free(out);
    return 0;
}
EOF
     fi && \
     gcc -O3 -DLODEPNG_COMPILE_C -x c -c lodepng.cpp -o lodepng.o && \
     gcc -O3 -o lodepng_perf_driver lodepng_perf_driver.c lodepng.o' \
    'test -x lodepng_perf_driver'

# 13. optipng — already built.
build_proj optipng-0.7.7 \
    './configure && make -j' \
    'test -x src/optipng/optipng'

# 14. tmux — autotools.
build_proj tmux \
    '[ -f Makefile ] || ./configure --enable-static; make -j' \
    'test -x tmux'

echo ""
echo "=== Summary ==="
grep -E '^\[' "$LOG" | awk '{print $1}' | sort | uniq -c
echo ""
echo "Full log: $LOG"
