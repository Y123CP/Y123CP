#!/bin/bash
                                                
                                          
#
                               
                                                          
                                                                               
                                                                                
                                                                      
                                
#
     
#   bash perf_run.sh <command...>
#     e.g.  bash perf_run.sh ./target/release/bench input.dat
#           bash perf_run.sh script_c2rust/.venv/bin/python -m some.measure
             
                                  
                                             
#
                                                                        
#   anon ALL=(root) NOPASSWD: /usr/bin/cpupower, \
#     /usr/bin/tee /sys/devices/system/cpu/intel_pstate/no_turbo, \
#     /usr/bin/bash /home/anonymous/artifact/PerfTrans/dataset_trans/rdt_isolate.sh
set -uo pipefail

CORES=16,34
PIN_CORE=16
NOTURBO=/sys/devices/system/cpu/intel_pstate/no_turbo
HERE="$(cd "$(dirname "$0")" && pwd)"
RDT_SH="$HERE/dataset_trans/rdt_isolate.sh"
PIN=${PIN:-1}
NO_RDT=${NO_RDT:-0}

RDT_ON=0                                          

_restore() {
    sudo /usr/bin/cpupower -c "$CORES" frequency-set -g powersave >/dev/null 2>&1
    echo 0 | sudo /usr/bin/tee "$NOTURBO" >/dev/null 2>&1
    [ "$RDT_ON" = 1 ] && sudo /usr/bin/bash "$RDT_SH" teardown >/dev/null 2>&1
    echo "[perf_run] restored -> powersave on $CORES + turbo ON$([ "$RDT_ON" = 1 ] && echo ' + RDT teardown')"
}
trap _restore EXIT INT TERM

sudo /usr/bin/cpupower -c "$CORES" frequency-set -g performance >/dev/null
echo 1 | sudo /usr/bin/tee "$NOTURBO" >/dev/null
if [ "$NO_RDT" != 1 ]; then
    if sudo /usr/bin/bash "$RDT_SH" setup >/dev/null 2>&1; then
        RDT_ON=1
    else
        echo "[perf_run] ⚠️ RDT setup 失败 — 检查 sudoers 是否含 'rdt_isolate.sh *';本次无 L3/带宽隔离" >&2
    fi
fi
echo "[perf_run] pinned -> performance on $CORES + turbo OFF$([ "$RDT_ON" = 1 ] && echo ' + RDT isolated' || echo ' (RDT OFF)')" \
     "(cpu16 now $(cat /sys/devices/system/cpu/cpu16/cpufreq/scaling_cur_freq 2>/dev/null) kHz)"

                                                          
                                                      
                                                 
                                                         
                                   
                                            
if [ "${NO_CORE_CHECK:-0}" != 1 ] && command -v perf >/dev/null 2>&1; then
    _cc=$(mktemp -u /tmp/perf_run_corecheck.XXXXXX).data
    if perf record -C "$PIN_CORE" -F 99 -o "$_cc" -- sleep 2 >/dev/null 2>&1; then
                                                
                                                         
        _top=$(perf report -i "$_cc" --stdio --sort comm 2>/dev/null \
               | grep -E '^[[:space:]]+[0-9]+\.[0-9]+%' \
               | sed 's/[[:space:]]*$//' \
               | grep -vE '(swapper|perf)$' | head -1)
        _pct=$(printf '%s' "$_top" | awk '{print int($1)}')
        if [ -n "$_pct" ] && [ "$_pct" -gt 10 ]; then
            echo "[perf_run] ❌ 测量核 $PIN_CORE 被外部进程占用:$_top" >&2
            echo "[perf_run]    绝对墙钟会被稀释,拒绝启动。查占用者:" >&2
            echo "[perf_run]    perf record -C $PIN_CORE -- sleep 3 && perf report --sort comm" >&2
            echo "[perf_run]    定位到 PID 后:cat /proc/<pid>/{comm,cmdline,status}" >&2
            echo "[perf_run]    确认无害可用 NO_CORE_CHECK=1 跳过本检查。" >&2
            rm -f "$_cc"; exit 1
        fi
    fi
    rm -f "$_cc"
fi

                                           
                                                          
                                        
export PERF_RUN_ENV=1

                                                
if [ "$PIN" = 1 ]; then
    taskset -c "$PIN_CORE" "$@"
else
    "$@"
fi
rc=$?
exit $rc
