#!/usr/bin/env bash
# Run gen.sh + fetch.sh for every project; record hashes for oracle update.
set -u
REPO="/home/anonymous/artifact/PerfTrans"
TRANS="$REPO/dataset_trans_process"
LOG="$TRANS/transpile_log/inputs_populate.log"
HASHES="$TRANS/transpile_log/oracle_hashes.txt"
: > "$LOG"
: > "$HASHES"

PROJECTS=(binn brotli bzip2-1.0.8 heman json-c json_h libcsv libtree libxml2 libzahl lil lodepng optipng-0.7.7 tmux)

ok()   { printf "[ OK ] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }
fail() { printf "[FAIL] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }
skip() { printf "[SKIP] %-22s %s\n" "$1" "$2" | tee -a "$LOG"; }

for p in "${PROJECTS[@]}"; do
    wd="$TRANS/$p/workloads"
    echo "==[ $p ]==" | tee -a "$LOG"
    [ -d "$wd" ] || { skip "$p" "no workloads/"; continue; }

    if [ -x "$wd/gen.sh" ]; then
        bash "$wd/gen.sh" >> "$LOG" 2>&1 \
            && ok "$p" "gen.sh ok" \
            || fail "$p" "gen.sh failed"
    fi
    if [ -x "$wd/fetch.sh" ]; then
        bash "$wd/fetch.sh" >> "$LOG" 2>&1 \
            && ok "$p" "fetch.sh ok" \
            || fail "$p" "fetch.sh failed (likely network or missing tool)"
    fi
done

echo "" | tee -a "$LOG"
echo "=== Final input populations ===" | tee -a "$LOG"
for p in "${PROJECTS[@]}"; do
    wd="$TRANS/$p/workloads"
    pc=$(ls "$wd/inputs/pipeline" 2>/dev/null | wc -l)
    cc=$(ls "$wd/inputs/correctness" 2>/dev/null | wc -l)
    ec=$(ls "$wd/inputs/experiment" 2>/dev/null | wc -l)
    printf "%-22s W1=%-2s  W2=%-2s  W3=%-2s\n" "$p" "$cc" "$pc" "$ec" | tee -a "$LOG"
done

echo "" | tee -a "$LOG"
echo "Full log: $LOG"
