#!/usr/bin/env bash
# ============================================================================
                                                         
                                                        
                                                     
#
                     
                                                 
                                          
                                                    
                                                     
                                               
#
                           
                                                      
                                            
# ============================================================================
set -uo pipefail

RESCTRL=/sys/fs/resctrl
MEAS_GROUP="$RESCTRL/measure"
MEAS_CPUS="16,34"                                     
DEFAULT_MB=50                                                  

need_root() { [ "$(id -u)" = 0 ] || { echo "需 root: sudo bash $0 ${1:-}"; exit 1; }; }
has_l3() { [ -d "$RESCTRL/info/L3" ]; }
has_mb() { [ -d "$RESCTRL/info/MB" ]; }

mount_resctrl() {
    [ -f "$RESCTRL/schemata" ] || { echo "挂载 resctrl ..."; mount -t resctrl resctrl "$RESCTRL"; }
}

                               
                                              
schemata_line() {
    local sch=""
    if has_l3 && [ -n "$1" ]; then sch="L3:0=$1"; fi
    if has_mb; then [ -n "$sch" ] && sch="$sch"$'\n'; sch="${sch}MB:0=$2"; fi
    printf '%s\n' "$sch"
}

                                                  
                                                             
                            
evict_intruders() {
    local other tid n=0
    other=$(seq 0 $(($(nproc --all)-1)) | grep -vxE '16|34' | paste -sd,)
    for tid in $(ps -eLo tid,psr --no-headers | awk '$2==16||$2==34{print $1}'); do
        if taskset -pc "$other" "$tid" >/dev/null 2>&1; then n=$((n+1)); fi
    done
    echo "清场: 已把 16/34 上 $n 个可迁移线程赶到其余核(内核线程略过)"
}

setup() {
    need_root setup
    mount_resctrl

    local meas_l3="" def_l3=""
    if has_l3; then
        local cbm full n x half def_mask meas_mask
        cbm=$(cat "$RESCTRL/info/L3/cbm_mask")
        full=$((16#$cbm))
        n=0; x=$full; while ((x)); do ((n += x & 1)); ((x >>= 1)); done
        half=$((n / 2)); ((half < 1)) && half=1
        def_mask=$(((1 << half) - 1))
        meas_mask=$((full & ~def_mask))
        meas_l3=$(printf '%x' "$meas_mask"); def_l3=$(printf '%x' "$def_mask")
        printf 'L3 CAT: 共 %d way; measure 高 %d way=0x%s; default 低 %d way=0x%s\n' \
            "$n" "$((n - half))" "$meas_l3" "$half" "$def_l3"
    else
        echo "⚠️ L3 CAT 不可用(内核只暴露 MBA)→ 仅 MBA 限带宽,无 L3 空间隔离;务必配测量 gate 兜底"
    fi
    has_mb && echo "MBA: measure 100% / default $DEFAULT_MB%"

    mkdir -p "$MEAS_GROUP"
    echo "$MEAS_CPUS" > "$MEAS_GROUP/cpus_list"
    schemata_line "$meas_l3" 100          > "$MEAS_GROUP/schemata"
    schemata_line "$def_l3"  "$DEFAULT_MB" > "$RESCTRL/schemata"

    evict_intruders

    echo "==== 配置完成 ===="; status
}

status() {
    [ -f "$RESCTRL/schemata" ] || { echo "resctrl 未挂载(先 sudo bash $0 setup)"; return; }
    echo "== 资源: $(ls "$RESCTRL/info" 2>/dev/null | tr '\n' ' ')"
    echo "== default(其他进程) =="; cat "$RESCTRL/schemata"; echo "  cpus: $(cat "$RESCTRL/cpus_list")"
    if [ -d "$MEAS_GROUP" ]; then
        echo "== measure(核 $MEAS_CPUS) =="; cat "$MEAS_GROUP/schemata"; echo "  cpus: $(cat "$MEAS_GROUP/cpus_list")"
    else
        echo "(measure group 未建)"
    fi
}

teardown() {
    need_root teardown
    [ -d "$MEAS_GROUP" ] && { rmdir "$MEAS_GROUP"; echo "已删 measure group"; }
    if [ -f "$RESCTRL/schemata" ]; then
        local full_l3=""
        has_l3 && full_l3=$(cat "$RESCTRL/info/L3/cbm_mask")
        schemata_line "$full_l3" 100 > "$RESCTRL/schemata"
        echo "default 已恢复: 全 L3 + 带宽不限"
    fi
    echo "(如需彻底卸载: sudo umount $RESCTRL)"
}

case "${1:-}" in
    setup)    setup ;;
    status)   status ;;
    evict)    need_root evict; evict_intruders ;;
    teardown) teardown ;;
    *) echo "用法: sudo bash $0 {setup|status|evict|teardown}"; exit 1 ;;
esac
