#!/usr/bin/env bash
# Per-project coverage measurement.
#
# For each project:
#   1. Rebuild C reference with -O0 -g --coverage to <proj>/_cov/<bin>
#   2. Run W1 (correctness), W2 (pipeline), W3 (experiment) inputs through it
#   3. Run gcov on .c files in library scope; aggregate line + function % via Python
#   4. Append row to script_c2rust/docs/coverage_summary.csv
#
# Output: per-project gcov .gcov files in <proj>/_cov/coverage/
#         consolidated CSV in script_c2rust/docs/coverage_summary.csv
#         markdown table in script_c2rust/docs/coverage_summary.md
set -u
REPO="/home/anonymous/artifact/PerfTrans"
SOURCE="$REPO/dataset_source"
TRANS="$REPO/dataset_trans_process"
DOCS="$REPO/script_c2rust/docs"
CSV="$DOCS/coverage_summary.csv"
MD="$DOCS/coverage_summary.md"
LOG="$TRANS/transpile_log/coverage.log"
: > "$LOG"

# libxml2 needs cmake ≥3.18; use bundled 3.27 first, fall back to system 3.16.
export PATH="$REPO/c2rust-thinking/local/src/cmake-3.27.9-linux-x86_64/bin:$PATH"

ok()   { printf "[ OK ] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }
fail() { printf "[FAIL] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }
skip() { printf "[SKIP] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }

# Reset CSV
echo "project,lib_files,total_lines,covered_lines,line_pct,total_funcs,covered_funcs,fn_pct" > "$CSV"

aggregate_gcov() {
    # Run gcov over $1 (work dir) and aggregate line/fn coverage via Python.
    # Uses gcov-12 --json-format for both line and function counts.
    local wd="$1" proj="$2"
    python3 - "$wd" "$proj" "$CSV" <<'PY'
import sys, os, glob, subprocess, gzip, json
work_dir, proj, csv_path = sys.argv[1], sys.argv[2], sys.argv[3]

gcda_files = []
for root, _, files in os.walk(work_dir):
    for f in files:
        if f.endswith('.gcda'):
            gcda_files.append(os.path.join(root, f))
if not gcda_files:
    print(f"  WARN: no .gcda files under {work_dir}", file=sys.stderr)
    with open(csv_path,'a') as f: f.write(f"{proj},0,0,0,0.0,0,0,0.0\n")
    sys.exit(0)

# Run gcov-12 in each dir with .gcda — produces .gcov.json.gz per source.
dirs = sorted({os.path.dirname(f) for f in gcda_files})
for d in dirs:
    gcdas = [f for f in os.listdir(d) if f.endswith('.gcda')]
    subprocess.run(['gcov-12', '-j', '--json-format'] + gcdas,
                   cwd=d, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

# Aggregate from JSON (line + function level).
total_lines = covered_lines = 0
fns_total = fns_covered = 0
seen_files = set()
for jgz in glob.glob(f'{work_dir}/**/*gcov.json.gz', recursive=True):
    try:
        with gzip.open(jgz) as f:
            data = json.load(f)
        for fe in data.get('files', []):
            fpath = fe.get('file', '')
            if '/test' in fpath or fpath.endswith('_test.c'): continue
            if fpath in seen_files: continue   # dedupe
            seen_files.add(fpath)
            for ln in fe.get('lines', []):
                total_lines += 1
                if ln.get('count', 0) > 0:
                    covered_lines += 1
            for fn in fe.get('functions', []):
                fns_total += 1
                if fn.get('execution_count', 0) > 0:
                    fns_covered += 1
    except Exception as e:
        print(f"  WARN: parse {jgz}: {e}", file=sys.stderr)

line_pct = (100.0 * covered_lines / total_lines) if total_lines else 0.0
fn_pct = (100.0 * fns_covered / fns_total) if fns_total else 0.0
print(f"  {proj}: {covered_lines}/{total_lines} lines ({line_pct:.1f}%), {fns_covered}/{fns_total} fns ({fn_pct:.1f}%)")
with open(csv_path, 'a') as f:
    f.write(f"{proj},{len(seen_files)},{total_lines},{covered_lines},{line_pct:.2f},{fns_total},{fns_covered},{fn_pct:.2f}\n")
PY
}

measure_one() {
    local name="$1" build="$2" bin="$3" run_w1="$4" run_w2="$5" run_w3="$6"
    local src="$SOURCE/$name"
    local wd="$TRANS/$name/workloads"
    local cov="$src/_cov"

    [ -d "$src" ] || { skip "$name" "source dir missing"; return; }
    [ -d "$wd" ]  || { skip "$name" "workloads dir missing"; return; }

    echo "==[ $name ]==" >> "$LOG"
    rm -rf "$cov"
    mkdir -p "$cov"

    # Build with coverage instrumentation
    (cd "$src" && eval "$build") >> "$LOG" 2>&1 || { fail "$name" "build failed"; return; }
    [ -x "$cov/$bin" ] || { fail "$name" "binary $cov/$bin missing"; return; }

    # Run W1
    eval "$run_w1" >> "$LOG" 2>&1 || true
    # Run W2
    eval "$run_w2" >> "$LOG" 2>&1 || true
    # Run W3
    eval "$run_w3" >> "$LOG" 2>&1 || true

    # Aggregate — search whole src tree (gcda may live outside _cov/ for autotools).
    aggregate_gcov "$src" "$name" | tee -a "$LOG"
    ok "$name" "coverage measured"
}

# CFLAGS for coverage builds
CC_FLAGS='-O0 -g -fprofile-arcs -ftest-coverage'
LD_FLAGS='--coverage'

# ===== Per-project recipes =====

# bzip2 — CLI sweep + exhaustive driver hitting the libbzip2 in-memory APIs
# (BZ2_bzBuffToBuff*, bzopen, BZ2_bzWriteOpen/Close stream variants).
measure_one bzip2-1.0.8 \
    "make clean >/dev/null 2>&1; mkdir -p _cov && \
     gcc $CC_FLAGS -c -o _cov/blocksort.o blocksort.c && \
     gcc $CC_FLAGS -c -o _cov/huffman.o huffman.c && \
     gcc $CC_FLAGS -c -o _cov/crctable.o crctable.c && \
     gcc $CC_FLAGS -c -o _cov/randtable.o randtable.c && \
     gcc $CC_FLAGS -c -o _cov/compress.o compress.c && \
     gcc $CC_FLAGS -c -o _cov/decompress.o decompress.c && \
     gcc $CC_FLAGS -c -o _cov/bzlib.o bzlib.c && \
     gcc $CC_FLAGS -c -o _cov/bzip2.o bzip2.c && \
     gcc $LD_FLAGS -o _cov/bzip2 _cov/*.o && \
     gcc $CC_FLAGS -c -o _cov/bz_drv.o bz_exhaustive_driver.c && \
     gcc $LD_FLAGS -o _cov/bz_exhaustive _cov/bz_drv.o _cov/bzlib.o _cov/blocksort.o _cov/huffman.o _cov/crctable.o _cov/randtable.o _cov/compress.o _cov/decompress.o" \
    "bzip2" \
    "BZ=$SOURCE/bzip2-1.0.8/_cov/bzip2; \
     IN=$TRANS/bzip2-1.0.8/workloads/inputs/correctness/tiny.txt; \
     \$BZ -c -z \$IN > /tmp/cov.bz2 && \$BZ -c -d /tmp/cov.bz2 > /tmp/cov.out && \
     \$BZ -c -z -1 \$IN > /tmp/cov.bz2 && \
     \$BZ -c -z -9 \$IN > /tmp/cov.bz2 && \
     \$BZ -c -d /tmp/cov.bz2 > /tmp/cov.out && \
     $SOURCE/bzip2-1.0.8/_cov/bz_exhaustive 2>/tmp/cov.out >/tmp/cov.out2 || true" \
    "BZ=$SOURCE/bzip2-1.0.8/_cov/bzip2; \
     IN=$TRANS/bzip2-1.0.8/workloads/inputs/pipeline/silesia-w2.ref; \
     for lvl in 1 5 9; do \$BZ -c -z -\$lvl \$IN > /tmp/cov.bz2 && \$BZ -c -d /tmp/cov.bz2 > /tmp/cov.out; done; \
     \$BZ -t /tmp/cov.bz2; \
     \$BZ --help 2>&1 > /tmp/cov.out || true; \
     \$BZ --version 2>&1 > /tmp/cov.out || true; \
     # File-mode operations (NOT -c) — hit bzip2.c file-utility paths
     cp \$IN /tmp/cov_in.txt && \$BZ -k -z -f /tmp/cov_in.txt && \$BZ -d -k -f /tmp/cov_in.txt.bz2 || true; \
     cp \$IN /tmp/cov_in2.txt && \$BZ -z -f /tmp/cov_in2.txt && \$BZ -d /tmp/cov_in2.txt.bz2 || true; \
     # Corrupt a compressed file and try to decompress (hits crcError / panic-recovery)
     \$BZ -c -z /tmp/cov_in2.txt > /tmp/corrupt.bz2 && \
       dd if=/dev/urandom of=/tmp/corrupt.bz2 bs=1 count=4 seek=20 conv=notrunc 2>/dev/null && \
       \$BZ -c -d /tmp/corrupt.bz2 > /tmp/cov.out 2>&1 || true; \
     # -s small mode + -v verbose mode (hits more bzip2.c paths)
     \$BZ -c -z -s -1 \$IN > /tmp/cov.bz2; \
     \$BZ -c -d -s /tmp/cov.bz2 > /tmp/cov.out; \
     \$BZ -c -z -v -1 \$IN > /tmp/cov.bz2 2>&1; \
     \$BZ -c -d -v /tmp/cov.bz2 > /tmp/cov.out 2>&1; \
     rm -f /tmp/cov_in.txt /tmp/cov_in2.txt /tmp/cov_in.txt.bz2 /tmp/cov_in2.txt.bz2" \
    "for f in $TRANS/bzip2-1.0.8/workloads/inputs/experiment/silesia-*; do \
       $SOURCE/bzip2-1.0.8/_cov/bzip2 -c -z -9 \$f > /tmp/cov.bz2 && \
       $SOURCE/bzip2-1.0.8/_cov/bzip2 -c -d /tmp/cov.bz2 > /tmp/cov.out && \
       $SOURCE/bzip2-1.0.8/_cov/bzip2 -t /tmp/cov.bz2; \
     done; \
     # Concatenate two .bz2 streams (multi-stream input)
     $SOURCE/bzip2-1.0.8/_cov/bzip2 -c -z -1 $TRANS/bzip2-1.0.8/workloads/inputs/experiment/silesia-xml > /tmp/multi.bz2 && \
     $SOURCE/bzip2-1.0.8/_cov/bzip2 -c -z -1 $TRANS/bzip2-1.0.8/workloads/inputs/experiment/silesia-dickens >> /tmp/multi.bz2 && \
     $SOURCE/bzip2-1.0.8/_cov/bzip2 -c -d /tmp/multi.bz2 > /tmp/cov.out || true"

# brotli — cmake build + exhaustive C driver exercising streaming + dictionary.
measure_one brotli \
    "rm -rf _cov && mkdir _cov && cd _cov && \
     cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_C_FLAGS='$CC_FLAGS' -DCMAKE_EXE_LINKER_FLAGS='$LD_FLAGS' .. >/dev/null && \
     cmake --build . --target brotli brotlienc brotlidec brotlicommon -j && \
     cd .. && \
     gcc $CC_FLAGS -I c/include -c brotli_exhaustive_driver.c -o _cov/brotli_drv.o && \
     gcc $LD_FLAGS -o _cov/brotli_exh _cov/brotli_drv.o -L_cov -lbrotlienc -lbrotlidec -lbrotlicommon -Wl,-rpath,$SOURCE/brotli/_cov" \
    "brotli" \
    "BR=$SOURCE/brotli/_cov/brotli; IN=$TRANS/brotli/workloads/inputs/correctness/tiny.txt; \
     \$BR -c -q 6 \$IN > /tmp/cov.br && \$BR -c -d /tmp/cov.br > /tmp/cov.out && \
     \$BR -c -q 0 \$IN > /tmp/cov.br && \$BR -c -d /tmp/cov.br > /tmp/cov.out && \
     \$BR -c -q 11 \$IN > /tmp/cov.br && \$BR -c -d /tmp/cov.br > /tmp/cov.out && \
     LD_LIBRARY_PATH=$SOURCE/brotli/_cov $SOURCE/brotli/_cov/brotli_exh 2>&1 > /tmp/cov.out || true" \
    "BR=$SOURCE/brotli/_cov/brotli; IN=$TRANS/brotli/workloads/inputs/pipeline/silesia-w2.ref; \
     for q in 0 3 6 9 11; do \$BR -c -q \$q \$IN > /tmp/cov.br && \$BR -c -d /tmp/cov.br > /tmp/cov.out; done; \
     \$BR --help 2>&1 > /tmp/cov.out || true; \
     \$BR --version 2>&1 > /tmp/cov.out || true" \
    "for f in $TRANS/brotli/workloads/inputs/experiment/silesia-*; do \
       BR=$SOURCE/brotli/_cov/brotli; \
       for q in 1 6; do \$BR -c -q \$q \$f > /tmp/cov.br && \$BR -c -d /tmp/cov.br > /tmp/cov.out; done; \
       \$BR -c -q 6 -w 16 \$f > /tmp/cov.br && \$BR -c -d /tmp/cov.br > /tmp/cov.out; \
     done; \
     # Only run q11 on the smallest input (silesia-xml at 5MB) to exercise quality-11 code path.
     BR=$SOURCE/brotli/_cov/brotli; \
     \$BR -c -q 11 $TRANS/brotli/workloads/inputs/experiment/silesia-xml > /tmp/cov.br && \$BR -c -d /tmp/cov.br > /tmp/cov.out"

# libcsv — exhaustive driver covering all 23 public APIs.
measure_one libcsv \
    "mkdir -p _cov && \
     gcc $CC_FLAGS -c -o _cov/libcsv.o libcsv.c && \
     gcc $CC_FLAGS -c -o _cov/libcsv_drv.o libcsv_exhaustive_driver.c -I. && \
     gcc $LD_FLAGS -o _cov/libcsv_exh _cov/libcsv_drv.o _cov/libcsv.o" \
    "libcsv_exh" \
    "$SOURCE/libcsv/_cov/libcsv_exh $TRANS/libcsv/workloads/inputs/correctness/tiny.csv > /tmp/cov.out 2>&1 && \
     $SOURCE/libcsv/_cov/libcsv_exh --strict $TRANS/libcsv/workloads/inputs/correctness/tiny.csv > /tmp/cov.out 2>&1" \
    "$SOURCE/libcsv/_cov/libcsv_exh $TRANS/libcsv/workloads/inputs/pipeline/medium.csv > /tmp/cov.out 2>&1" \
    "for f in $TRANS/libcsv/workloads/inputs/experiment/*.csv; do $SOURCE/libcsv/_cov/libcsv_exh \$f > /tmp/cov.out 2>&1; $SOURCE/libcsv/_cov/libcsv_exh --strict --append-null \$f > /tmp/cov.out 2>&1; done"

# libtree
measure_one libtree \
    "mkdir -p _cov && \
     gcc $CC_FLAGS -std=c99 -D_FILE_OFFSET_BITS=64 -c -o _cov/libtree.o libtree.c && \
     gcc $LD_FLAGS -o _cov/libtree _cov/libtree.o" \
    "libtree" \
    "LT=$SOURCE/libtree/_cov/libtree; \
     LIBTREE_NO_COLOR=1 \$LT -p $TRANS/libtree/workloads/inputs/correctness/tiny-elf > /tmp/cov.out 2>&1; \
     LIBTREE_NO_COLOR=1 \$LT -v $TRANS/libtree/workloads/inputs/correctness/tiny-elf > /tmp/cov.out 2>&1; \
     LIBTREE_NO_COLOR=1 \$LT -vv $TRANS/libtree/workloads/inputs/correctness/tiny-elf > /tmp/cov.out 2>&1; \
     LIBTREE_NO_COLOR=1 \$LT --max-depth 1 $TRANS/libtree/workloads/inputs/correctness/tiny-elf > /tmp/cov.out 2>&1; \
     LIBTREE_NO_COLOR=1 \$LT --version > /tmp/cov.out 2>&1; \
     LIBTREE_NO_COLOR=1 \$LT --help > /tmp/cov.out 2>&1" \
    "LIBTREE_NO_COLOR=1 $SOURCE/libtree/_cov/libtree -vvv -p $TRANS/libtree/workloads/inputs/pipeline/target-bin > /tmp/cov.out 2>&1" \
    "for f in $TRANS/libtree/workloads/inputs/experiment/*-bin; do LIBTREE_NO_COLOR=1 $SOURCE/libtree/_cov/libtree -vvv -p \$f > /tmp/cov.out 2>&1; LIBTREE_NO_COLOR=1 $SOURCE/libtree/_cov/libtree -p \$f > /tmp/cov.out 2>&1; LIBTREE_NO_COLOR=1 $SOURCE/libtree/_cov/libtree --max-depth 2 \$f > /tmp/cov.out 2>&1; done"

# libzahl — compile all sources with --coverage, then harness driver.
measure_one libzahl \
    "mkdir -p _cov && \
     for src in src/*.c; do \
       gcc $CC_FLAGS -DGOOD_RAND -D_DEFAULT_SOURCE -D_BSD_SOURCE -D_XOPEN_SOURCE=700 -I. -c \$src -o _cov/\$(basename \$src .c).o; \
     done && \
     gcc $CC_FLAGS -DGOOD_RAND -I. -c libzahl_harness.c -o _cov/harness.o && \
     gcc $LD_FLAGS -o _cov/libzahl_test _cov/*.o" \
    "libzahl_test" \
    "$SOURCE/libzahl/_cov/libzahl_test > /tmp/cov.out 2>&1" \
    "for f in $TRANS/libzahl/workloads/inputs/experiment/*.bin; do $SOURCE/libzahl/_cov/libzahl_test \$f > /tmp/cov.out 2>&1; done" \
    "$SOURCE/libzahl/_cov/libzahl_test > /tmp/cov.out 2>&1"

# lil  — primary CLI + secondary embedder binary that drives lil_* C-API.
measure_one lil \
    "mkdir -p _cov && \
     gcc $CC_FLAGS -c -o _cov/lil.o lil.c && \
     gcc $CC_FLAGS -c -o _cov/main.o main.c && \
     gcc $LD_FLAGS -o _cov/lil _cov/lil.o _cov/main.o -lm && \
     gcc $CC_FLAGS -I. -c -o _cov/lil_embedder.o lil_embedder.c && \
     gcc $LD_FLAGS -o _cov/lil_embedder _cov/lil_embedder.o _cov/lil.o -lm && \
     cp $TRANS/lil/workloads/inputs/pipeline/_inner.lil /tmp/_lil_inner.lil 2>/dev/null || true" \
    "lil" \
    "$SOURCE/lil/_cov/lil $TRANS/lil/workloads/inputs/correctness/tiny.lil > /tmp/cov.out 2>&1; \
     $SOURCE/lil/_cov/lil_embedder > /tmp/cov.out 2>&1" \
    "$SOURCE/lil/_cov/lil $TRANS/lil/workloads/inputs/pipeline/medium.lil > /tmp/cov.out 2>&1" \
    "for f in $TRANS/lil/workloads/inputs/experiment/*.lil; do timeout 10 $SOURCE/lil/_cov/lil \$f > /tmp/cov.out 2>&1 || true; done; \
     for f in $SOURCE/lil/*.lil; do timeout 10 $SOURCE/lil/_cov/lil \$f > /tmp/cov.out 2>&1 || true; done"

# lodepng
measure_one lodepng \
    "mkdir -p _cov && \
     gcc $CC_FLAGS -DLODEPNG_COMPILE_C -x c -c lodepng.cpp -o _cov/lodepng.o && \
     gcc $CC_FLAGS -c -o _cov/driver.o lodepng_perf_driver.c && \
     gcc $LD_FLAGS -o _cov/lodepng_perf_driver _cov/driver.o _cov/lodepng.o" \
    "lodepng_perf_driver" \
    "$SOURCE/lodepng/_cov/lodepng_perf_driver $TRANS/lodepng/workloads/inputs/correctness/tiny_dir/tiny.png > /tmp/cov.png 2>&1" \
    "$SOURCE/lodepng/_cov/lodepng_perf_driver $TRANS/lodepng/workloads/inputs/pipeline/medium.png > /tmp/cov.png 2>&1" \
    "for f in $TRANS/lodepng/workloads/inputs/experiment/pngsuite/*.png; do $SOURCE/lodepng/_cov/lodepng_perf_driver \$f > /tmp/cov.png 2>/dev/null; done"

# optipng  — uses ./configure + make. Rebuild with CFLAGS.
# Use 'make local-build' since optipng's configure doesn't accept --enable-static.
measure_one optipng-0.7.7 \
    "make clean >/dev/null 2>&1; \
     ./configure >/dev/null && \
     make -j CFLAGS='$CC_FLAGS' LDFLAGS='$LD_FLAGS' && \
     mkdir -p _cov && cp src/optipng/optipng _cov/optipng" \
    "optipng" \
    "OP=$SOURCE/optipng-0.7.7/_cov/optipng; \
     for lvl in 0 1 2 3 4 5 6 7; do \$OP -force -o\$lvl -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png; done; \
     for r in '-nb' '-nc' '-np' '-nx' '-no-color-reduction' '-no-bitdepth-reduction' '-no-palette-reduction' '-no-reductions' '-keep' '-fix' '-snip'; do \$OP -force \$r -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; done; \
     \$OP -force -i 0 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png; \
     \$OP -force -i 1 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png; \
     \$OP -force -strip all -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png; \
     \$OP -force -log /tmp/cov.log -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png; \
     \$OP -v $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>&1 > /tmp/cov.out || true; \
     \$OP --help 2>&1 > /tmp/cov.out || true; \
     \$OP --version 2>&1 > /tmp/cov.out || true; \
     # Corrupted PNG to trigger pngerror.c paths.
     cp $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png /tmp/cov_corrupt.png && \
       dd if=/dev/urandom of=/tmp/cov_corrupt.png bs=1 count=4 seek=10 conv=notrunc 2>/dev/null && \
       \$OP -force -fix -out /tmp/cov.png /tmp/cov_corrupt.png 2>/dev/null || true; \
     \$OP -force -out /tmp/cov.png /tmp/cov_corrupt.png 2>/dev/null || true; \
     # Reduce options + sample-depth options.
     \$OP -force -bit_depth 8 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -filter 0 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -filter 5 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -mem 4 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -nz -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -paranoid -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -full -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -simulate -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     # Read from each non-PNG format the W3 generator made.
     for f in $TRANS/optipng-0.7.7/workloads/inputs/experiment/tiny.bmp $TRANS/optipng-0.7.7/workloads/inputs/experiment/tiny.ppm $TRANS/optipng-0.7.7/workloads/inputs/experiment/tiny_p6.ppm $TRANS/optipng-0.7.7/workloads/inputs/experiment/tiny.pgm $TRANS/optipng-0.7.7/workloads/inputs/experiment/tiny.gif $TRANS/optipng-0.7.7/workloads/inputs/experiment/tiny.tif; do \
       [ -f \$f ] && \$OP -force -out /tmp/cov_fmt.png \$f 2>/dev/null || true; \
     done; \
     # Non-PNG dispatch paths (pngxtern/minitiff/pnmio/gifread + jpeg stub).
     for f in $TRANS/optipng-0.7.7/workloads/inputs/experiment/be_4x2.tif \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/with_ext.gif \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/corrupt.gif \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/indexed_1bit.bmp \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/truncated.tif \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/fake.jpg \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/rgb24.bmp \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/rle4.bmp \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/ascii.pbm \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/ascii.pgm \
              $TRANS/optipng-0.7.7/workloads/inputs/experiment/ascii.ppm; do \
       [ -f \$f ] && \$OP -force -out /tmp/cov_fmt.png \$f 2>/dev/null || true; \
     done; \
     # CLI error-path validators (optipng.c err_option_arg/check_power2/check_rangeset).
     \$OP -force -mem 7 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -mem garbage -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -o 99 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -strip nosuch -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -filter 9 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     \$OP -force -zc 11 -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     # -dir output path helpers (ioutil opng_path_replace_dir / opng_os_create_dir).
     mkdir -p /tmp/cov_optdir && \
       \$OP -force -dir /tmp/cov_optdir $TRANS/optipng-0.7.7/workloads/inputs/correctness/tiny.png 2>/dev/null || true; \
     rm -rf /tmp/cov_optdir; \
     rm -f /tmp/cov_corrupt.png /tmp/cov_fmt.png" \
    "for lvl in 0 2 4 7; do $SOURCE/optipng-0.7.7/_cov/optipng -force -o\$lvl -out /tmp/cov.png $TRANS/optipng-0.7.7/workloads/inputs/pipeline/medium.png; done" \
    "OP=$SOURCE/optipng-0.7.7/_cov/optipng; \
     for f in $TRANS/optipng-0.7.7/workloads/inputs/experiment/*.png; do for lvl in 2 4 7; do \$OP -force -o\$lvl -out /tmp/cov.png \$f; done; done; \
     for ext in bmp gif tif ppm pgm pbm; do \
       for f in $TRANS/optipng-0.7.7/workloads/inputs/experiment/*.\$ext; do \
         [ -e \$f ] || continue; \
         out=/tmp/cov_optipng_\$(basename \$f .\$ext)_\$ext.png; \
         \$OP -force -out \$out \$f 2>/dev/null || true; \
         rm -f \$out; \
       done; \
     done; \
     # tiny_p6 special-case (P6 PPM)
     [ -e $TRANS/optipng-0.7.7/workloads/inputs/experiment/tiny_p6.ppm ] && \$OP -force -out /tmp/cov_p6.png $TRANS/optipng-0.7.7/workloads/inputs/experiment/tiny_p6.ppm 2>/dev/null || true; \
     rm -f /tmp/cov_p6.png; true"

# tmux  — autotools; rebuild with --coverage CFLAGS.
measure_one tmux \
    "make clean >/dev/null 2>&1; \
     ./configure --enable-static >/dev/null && \
     make -j CFLAGS='$CC_FLAGS' LDFLAGS='$LD_FLAGS' && \
     mkdir -p _cov && cp tmux _cov/tmux" \
    "tmux" \
    "TX=$SOURCE/tmux/_cov/tmux; \
     CORR=$TRANS/tmux/workloads/inputs/correctness; \
     \$TX -L cov_w1_$$ -f \$CORR/tiny.conf -C kill-server > /tmp/cov.out 2>&1; \
     # New-session based runs exercise format/options/cmd-find/environ much more than -C alone.
     timeout 30 \$CORR/exhaustive_run.sh \$TX \$CORR/exhaustive.conf > /tmp/cov.out 2>&1 || true; \
     # Run the exhaustive script with multiple configs to vary the option paths.
     timeout 30 \$CORR/exhaustive_run.sh \$TX \$CORR/tiny.conf > /tmp/cov.out 2>&1 || true" \
    "TX=$SOURCE/tmux/_cov/tmux; \
     timeout 30 $TRANS/tmux/workloads/inputs/correctness/exhaustive_run.sh \$TX $TRANS/tmux/workloads/inputs/pipeline/large.conf > /tmp/cov.out 2>&1 || true; \
     \$TX -L cov_w2_$$ -f $TRANS/tmux/workloads/inputs/pipeline/large.conf -C kill-server > /tmp/cov.out 2>&1" \
    "TX=$SOURCE/tmux/_cov/tmux; \
     for f in $TRANS/tmux/workloads/inputs/experiment/*.conf; do \
       \$TX -L cov_w3a_$$_\$RANDOM -f \$f -C kill-server > /tmp/cov.out 2>&1; \
       timeout 20 $TRANS/tmux/workloads/inputs/correctness/exhaustive_run.sh \$TX \$f > /tmp/cov.out 2>&1 || true; \
     done"

# libxml2 — cmake build + exhaustive_driver linked against the cov-instrumented libxml2.so.
measure_one libxml2 \
    "rm -rf _cov && mkdir _cov && cd _cov && \
     cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_C_FLAGS='$CC_FLAGS' -DCMAKE_EXE_LINKER_FLAGS='$LD_FLAGS' \
       -DLIBXML2_WITH_PYTHON=OFF -DLIBXML2_WITH_ICONV=OFF -DLIBXML2_WITH_ICU=OFF \
       -DLIBXML2_WITH_LZMA=OFF -DLIBXML2_WITH_ZLIB=OFF -DLIBXML2_WITH_TESTS=OFF .. >/dev/null && \
     make -j xmllint LibXml2 >/dev/null && cd .. && \
     gcc $CC_FLAGS -I. -Iinclude -I_cov -c exhaustive_driver.c -o _cov/exh_drv.o && \
     gcc $LD_FLAGS -o _cov/libxml2_exh _cov/exh_drv.o -L_cov -lxml2 -Wl,-rpath,$SOURCE/libxml2/_cov -lm" \
    "xmllint" \
    "XL=$SOURCE/libxml2/_cov/xmllint; \
     CORR=$TRANS/libxml2/workloads/inputs/correctness; \
     \$XL --noout \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --debug \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --copy \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --push \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --pushsmall \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --sax \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --sax1 \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --stream \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --walker \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --memory \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --noent \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --noblanks \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --nocdata \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --format \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --pretty 1 \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --pretty 2 \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --encode UTF-8 \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --encode ISO-8859-1 \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --recover \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --dropdtd \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --nsclean \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --valid \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --postvalid \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --noout \$CORR/namespaced.xml > /tmp/cov.out 2>&1; \
     \$XL --debug \$CORR/namespaced.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath '//book/title' \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath '//*/@id' \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath 'count(//item)' \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath '//*[local-name()=\"item\"]' \$CORR/namespaced.xml > /tmp/cov.out 2>&1; \
     # XPath exercising attribute / text / position / string / number axes.
     \$XL --xpath '//book[1]/@id|//book[2]/@id' \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath 'string(//title)|number(//count)' \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath 'sum(//price)|floor(3.7)|ceiling(2.1)|round(2.5)' \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath 'substring-before(\"abc-def\",\"-\")|substring-after(\"abc-def\",\"-\")' \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath 'concat(\"foo\",\"-\",\"bar\")|translate(\"abc\",\"abc\",\"xyz\")' \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath '//book[position()=last()]/preceding-sibling::*' \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath '//*[contains(@id,\"X\") or starts-with(name(),\"b\")]' \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath 'normalize-space(//title)' \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     # Malformed XML — drives xmlSaveErr / parse error paths.
     printf '<root><a><b></a></b></root>' > /tmp/_xml_bad.xml; \
     \$XL --recover /tmp/_xml_bad.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --htmlout --noout /tmp/_xml_bad.xml > /tmp/cov.out 2>&1 || true; \
     rm -f /tmp/_xml_bad.xml; \
     \$XL --auto > /tmp/cov.out 2>&1; \
     \$XL --version > /tmp/cov.out 2>&1; \
     \$XL --c14n \$CORR/tiny.xml > /tmp/cov.out 2>&1; \
     \$XL --insert \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --chkregister --noout \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --debugent \$CORR/with_dtd.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --testIO \$CORR/tiny.xml < \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --pattern '//book' \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --oldxml10 \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --noenc \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --huge \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --nonet \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --maxmem 4000000 \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --quiet --noout \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     LD_LIBRARY_PATH=$SOURCE/libxml2/_cov $SOURCE/libxml2/_cov/libxml2_exh \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     LD_LIBRARY_PATH=$SOURCE/libxml2/_cov $SOURCE/libxml2/_cov/libxml2_exh \$CORR/with_dtd.xml > /tmp/cov.out 2>&1 || true; \
     LD_LIBRARY_PATH=$SOURCE/libxml2/_cov $SOURCE/libxml2/_cov/libxml2_exh \$CORR/namespaced.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --sax \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --sax1 \$CORR/with_dtd.xml > /tmp/cov.out 2>&1; \
     \$XL --sax \$CORR/namespaced.xml > /tmp/cov.out 2>&1; \
     \$XL --catalogs \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --timing --noout \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --repeat \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --load-trace \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --maxmem 1000000 \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --htmlout --noout \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --noxincludenode --noout \$CORR/tiny.xml > /tmp/cov.out 2>&1 || true; \
     \$XL --nowarning --noout not_exist.xml > /tmp/cov.out 2>&1 || true; \
     \$XL > /tmp/cov.out 2>&1 || true" \
    "XL=$SOURCE/libxml2/_cov/xmllint; \
     \$XL --noout $TRANS/libxml2/workloads/inputs/pipeline/medium.xml > /tmp/cov.out 2>&1; \
     \$XL --stream --noout $TRANS/libxml2/workloads/inputs/pipeline/medium.xml > /tmp/cov.out 2>&1; \
     \$XL --xpath 'count(/)' $TRANS/libxml2/workloads/inputs/pipeline/medium.xml > /tmp/cov.out 2>&1" \
    "for f in $TRANS/libxml2/workloads/inputs/experiment/*.xml; do \
       $SOURCE/libxml2/_cov/xmllint --noout \$f > /tmp/cov.out 2>&1; \
       $SOURCE/libxml2/_cov/xmllint --stream --noout \$f > /tmp/cov.out 2>&1; \
       $SOURCE/libxml2/_cov/xmllint --xpath '//*[1]' \$f > /tmp/cov.out 2>&1; \
     done"

# json_h — driver re-build with coverage on the single header-only header.
# W1 hits every allow_* parse flag via per-flag input files.
measure_one json_h \
    "mkdir -p _cov && \
     gcc $CC_FLAGS -c -o _cov/json_h_driver.o json_h_driver.c && \
     gcc $LD_FLAGS -o _cov/json_h_driver _cov/json_h_driver.o" \
    "json_h_driver" \
    "DRV=$SOURCE/json_h/_cov/json_h_driver; CORR=$TRANS/json_h/workloads/inputs/correctness; \
     \$DRV \$CORR/tiny.json > /tmp/cov.out 2>&1; \
     \$DRV --pretty \$CORR/tiny.json > /tmp/cov.out 2>&1; \
     \$DRV --extract \$CORR/tiny.json > /tmp/cov.out 2>&1; \
     \$DRV --mode location \$CORR/tiny.json > /tmp/cov.out 2>&1; \
     \$DRV --mode trailing_comma \$CORR/trailing_comma.json > /tmp/cov.out 2>&1; \
     \$DRV --mode unquoted_keys \$CORR/unquoted_keys.json > /tmp/cov.out 2>&1; \
     \$DRV --mode global_object \$CORR/global_object.json > /tmp/cov.out 2>&1; \
     \$DRV --mode equals_in_object \$CORR/equals_in_object.json > /tmp/cov.out 2>&1; \
     \$DRV --mode no_commas \$CORR/no_commas.json > /tmp/cov.out 2>&1; \
     \$DRV --mode c_style_comments \$CORR/comments.json > /tmp/cov.out 2>&1; \
     \$DRV --mode single_quotes \$CORR/single_quotes.json > /tmp/cov.out 2>&1; \
     \$DRV --mode hexadecimal \$CORR/hex.json > /tmp/cov.out 2>&1; \
     \$DRV --mode leading_plus \$CORR/leading_plus.json > /tmp/cov.out 2>&1; \
     \$DRV --mode decimal_point \$CORR/decimal_point.json > /tmp/cov.out 2>&1; \
     \$DRV --mode inf_nan \$CORR/inf_nan.json > /tmp/cov.out 2>&1; \
     \$DRV --mode multi_line_strings \$CORR/multi_line.json > /tmp/cov.out 2>&1; \
     \$DRV --mode simplified \$CORR/trailing_comma.json > /tmp/cov.out 2>&1; \
     \$DRV --mode json5 \$CORR/hex.json > /tmp/cov.out 2>&1" \
    "$SOURCE/json_h/_cov/json_h_driver $TRANS/json_h/workloads/inputs/pipeline/synthetic-5m.json > /tmp/cov.out" \
    "for f in $TRANS/json_h/workloads/inputs/experiment/*.json; do $SOURCE/json_h/_cov/json_h_driver \$f > /tmp/cov.out; $SOURCE/json_h/_cov/json_h_driver --pretty \$f > /tmp/cov.out; $SOURCE/json_h/_cov/json_h_driver --extract \$f > /tmp/cov.out; done"

# json-c — cmake build with coverage CFLAGS + exhaustive driver linking against libjson-c.a.
measure_one json-c \
    "rm -rf _cov && mkdir _cov && cd _cov && \
     cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_C_FLAGS='$CC_FLAGS' -DCMAKE_EXE_LINKER_FLAGS='$LD_FLAGS' \
       -DBUILD_SHARED_LIBS=OFF -DDISABLE_EXTRA_LIBS=ON -DDISABLE_THREAD_LOCAL_STORAGE=ON .. >/dev/null && \
     make -j json-c json_parse >/dev/null && \
     gcc $CC_FLAGS -I.. -I. -c ../exhaustive_driver.c -o exhaustive_driver.o && \
     gcc $LD_FLAGS -o exhaustive_driver exhaustive_driver.o libjson-c.a" \
    "exhaustive_driver" \
    "$SOURCE/json-c/_cov/exhaustive_driver $TRANS/json-c/workloads/inputs/correctness/tiny.json > /tmp/cov.out 2>&1" \
    "$SOURCE/json-c/_cov/exhaustive_driver $TRANS/json-c/workloads/inputs/pipeline/medium.json > /tmp/cov.out 2>&1" \
    "for f in $TRANS/json-c/workloads/inputs/experiment/*.json; do $SOURCE/json-c/_cov/exhaustive_driver \$f > /tmp/cov.out 2>&1; done"

# binn  — exhaustive driver covering all 14 types × {list,map,object} × {write,read,iter}.
measure_one binn \
    "cd src && mkdir -p ../_cov && \
     gcc $CC_FLAGS -DBINN_NO_COMPRESS -c binn.c -o ../_cov/binn.o && \
     gcc $CC_FLAGS -c binn_exhaustive_driver.c -o ../_cov/driver.o && \
     gcc $LD_FLAGS -o ../_cov/binn_exhaustive ../_cov/driver.o ../_cov/binn.o" \
    "binn_exhaustive" \
    "$SOURCE/binn/_cov/binn_exhaustive build > /tmp/cov.out 2>&1" \
    "$SOURCE/binn/_cov/binn_exhaustive roundtrip > /tmp/cov.out 2>&1" \
    "$SOURCE/binn/_cov/binn_exhaustive build > /tmp/cov_build.bin && $SOURCE/binn/_cov/binn_exhaustive parse < /tmp/cov_build.bin > /tmp/cov.out 2>&1"

# heman — library only. Build with coverage + harness driver.
measure_one heman \
    "rm -rf _cov && mkdir _cov && cd _cov && \
     cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_C_FLAGS='$CC_FLAGS' \
       -DCMAKE_EXE_LINKER_FLAGS='$LD_FLAGS' .. >/dev/null && \
     make -j heman >/dev/null && \
     gcc $CC_FLAGS -I../include -c ../heman_harness.c -o heman_harness.o && \
     gcc $LD_FLAGS -o heman_harness heman_harness.o libheman.a -fopenmp -lm" \
    "heman_harness" \
    "$SOURCE/heman/_cov/heman_harness > /tmp/cov.out 2>&1" \
    "$SOURCE/heman/_cov/heman_harness > /tmp/cov.out 2>&1" \
    "$SOURCE/heman/_cov/heman_harness > /tmp/cov.out 2>&1"

echo ""
echo "=== Coverage summary CSV ==="
column -t -s, "$CSV"

# Render markdown table
python3 - <<PY
import csv
with open("$CSV") as f:
    rows = list(csv.reader(f))
md = ["# Coverage measurement results\n",
      "**Date:** 2026-05-21",
      "**Method:** gcov instrumentation (-O0 -g --coverage), run W1+W2+W3 inputs,",
      "aggregate via gcov --json-format. Library scope only (tests/examples excluded).\n",
      "| Project | .c files | Total lines | Covered | Line % | Total fns | Covered | Fn % |",
      "|---|---:|---:|---:|---:|---:|---:|---:|"]
for r in rows[1:]:
    md.append("| {} | {} | {} | {} | {}% | {} | {} | {}% |".format(*r))
with open("$MD","w") as f:
    f.write("\n".join(md) + "\n")
print(f"Wrote $MD")
PY

echo ""
echo "Full log: $LOG"
