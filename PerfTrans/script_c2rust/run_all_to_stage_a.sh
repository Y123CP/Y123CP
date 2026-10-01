#!/usr/bin/env bash
# Run every dataset_source project through the pipeline to Stage A
# (c2rust → cleanup → stage_a) by invoking the canonical entry point
# `main.py` once per project. Per-project log goes to /tmp/run_<NAME>.log;
# overall summary printed at the end.
#
# Usage:
#   ./run_all_to_stage_a.sh                       # all 14 projects
#   ./run_all_to_stage_a.sh bzip2-1.0.8 libcsv    # specific projects
#   FROM_STAGE=cleanup ./run_all_to_stage_a.sh    # skip c2rust (reuse 0_raw)
#
# Projects whose oracle.stdout_hash is still PLACEHOLDER will fail
# Stage A — fix via the project's workloads/gen.sh first.

set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

DEFAULT_PROJECTS=(
    binn bzip2-1.0.8 heman json-c json_h libcsv libtree
    libxml2 libzahl lil lodepng optipng-0.7.7 tmux brotli
)
PROJECTS=("${@:-${DEFAULT_PROJECTS[@]}}")
if [ "$#" -gt 0 ]; then PROJECTS=("$@"); fi
FROM_STAGE="${FROM_STAGE:-cleanup}"   # default: reuse git-tracked 0_raw

PY=.venv/bin/python
declare -A STATUS ELAPSED

t_overall_start=$(date +%s)
echo "=== run_all_to_stage_a.sh — ${#PROJECTS[@]} project(s), --from $FROM_STAGE ==="
for proj in "${PROJECTS[@]}"; do
    log=/tmp/run_${proj}.log
    t0=$(date +%s)
    printf '[%(%H:%M:%S)T] %-18s … ' -1 "$proj"
    if "$PY" main.py --project "$proj" --from "$FROM_STAGE" > "$log" 2>&1; then
        STATUS[$proj]=ok
    else
        STATUS[$proj]=FAIL
    fi
    ELAPSED[$proj]=$(( $(date +%s) - t0 ))
    printf '%-6s  %4ds  %s\n' "${STATUS[$proj]}" "${ELAPSED[$proj]}" "$log"
done

echo
printf '=== done in %d s ===\n' "$(( $(date +%s) - t_overall_start ))"
echo
printf '%-18s  %-6s  %5s  %s\n' 'project' 'status' 'time' 'last log line'
echo '-----------------------------------------------------------------------'
for proj in "${PROJECTS[@]}"; do
    last=$(grep -E '\[stage_a\] (OK|skipped)|RuntimeError' "/tmp/run_${proj}.log" | tail -1 | cut -c1-110)
    printf '%-18s  %-6s  %4ds  %s\n' \
        "$proj" "${STATUS[$proj]}" "${ELAPSED[$proj]}" "$last"
done
