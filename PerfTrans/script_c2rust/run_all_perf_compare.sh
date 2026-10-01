#!/usr/bin/env bash
# Run perf_compare on every project whose experiment.toml has
# `[c_reference]` filled. Uses 1_cleaned (not 0_raw) as the c2rust_raw
# baseline because raw c2rust 0.22.1 output frequently fails to compile
# under current rustc (the c2rust_compat patches in Stage 1 are needed
# just to build it). This makes the comparison "Stage 1 cleanup baseline
# vs Stage A lift" — the cleanest measurement of Stage A's contribution.
#
# Output: <repo_root>/report/<proj>_stage_a_perf_compare.md (per project)
# Per-project log: /tmp/perfcmp_<proj>.log
#
# Usage:
#   ./run_all_perf_compare.sh                       # default 7 ready projects
#   ./run_all_perf_compare.sh bzip2-1.0.8 libcsv    # specific projects
#   PIN_CPU=0 ./run_all_perf_compare.sh             # different CPU pin

set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

DEFAULT_PROJECTS=(
    bzip2-1.0.8 heman libcsv lil lodepng json_h libtree
)
PROJECTS=("${@:-${DEFAULT_PROJECTS[@]}}")
if [ "$#" -gt 0 ]; then PROJECTS=("$@"); fi
PIN_CPU="${PIN_CPU:-0}"

PY=.venv/bin/python
DT=../dataset_trans_process
declare -A STATUS ELAPSED LIFT_VS_C LIFT_VS_RAW

echo "=== run_all_perf_compare.sh — ${#PROJECTS[@]} project(s), pin_cpu=$PIN_CPU ==="
t_overall=$(date +%s)
for proj in "${PROJECTS[@]}"; do
    log=/tmp/perfcmp_${proj}.log
    t0=$(date +%s)
    printf '[%(%H:%M:%S)T] %-15s … ' -1 "$proj"
    if "$PY" perf_compare.py \
        --c2rust-project "$DT/$proj/1_cleaned" \
        --ours-project   "$DT/$proj/2_stage_a/crate" \
        --manifest       "$DT/$proj/workloads/experiment.toml" \
        --label          "${proj}_stage_a" \
        --pin-cpu        "$PIN_CPU" \
        > "$log" 2>&1; then
        STATUS[$proj]=ok
    else
        STATUS[$proj]=FAIL
    fi
    ELAPSED[$proj]=$(( $(date +%s) - t0 ))
    # Pluck the two headline numbers from the report if it was produced.
    report="../report/${proj}_stage_a_perf_compare.md"
    if [ -f "$report" ]; then
        LIFT_VS_C[$proj]=$(grep -oE 'ours vs C gap: \*\*[+-][0-9.]+%' "$report" | head -1 | grep -oE '[+-][0-9.]+%')
        LIFT_VS_RAW[$proj]=$(grep -oE 'perf-opt lift over c2rust_raw: \*\*[+-][0-9.]+%' "$report" | head -1 | grep -oE '[+-][0-9.]+%')
    fi
    printf '%-6s  %4ds  vs_C=%-8s  vs_raw=%s\n' \
        "${STATUS[$proj]}" "${ELAPSED[$proj]}" \
        "${LIFT_VS_C[$proj]:--}" "${LIFT_VS_RAW[$proj]:--}"
done

echo
printf '=== done in %d s ===\n' "$(( $(date +%s) - t_overall ))"
echo
printf '%-15s  %-6s  %5s  %-10s  %-10s  %s\n' \
    'project' 'status' 'time' 'vs C' 'vs c2rust' 'report'
echo '------------------------------------------------------------------------------------'
for proj in "${PROJECTS[@]}"; do
    printf '%-15s  %-6s  %4ds  %-10s  %-10s  %s\n' \
        "$proj" "${STATUS[$proj]}" "${ELAPSED[$proj]}" \
        "${LIFT_VS_C[$proj]:--}" "${LIFT_VS_RAW[$proj]:--}" \
        "report/${proj}_stage_a_perf_compare.md"
done
