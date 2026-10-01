#!/bin/bash
# Path adjustments for the script_c2rust layout:
#   - paths.conf lives at ../Config/ (one level up, sibling of sa_engine/)
#   - analyzer .cpp files live in $SCRIPT_DIR (this directory)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../Config/paths.conf"

# SVF runtime needs SVF_DIR exported to locate extapi.bc fallback path.
# Without it, SVF aborts with "Failed to find extapi.bc LLVM bitcode file".
export SVF_DIR="$SVF_ROOT"

        
show_usage() {
    echo "使用方法: $0 <SOURCE_DIR> [ANALYZE_CPP]"
    echo ""
    echo "参数说明:"
    echo "  SOURCE_DIR    源代码目录路径 (必需)"
    echo "  ANALYZE_CPP   分析文件名，默认为 analyze.cpp (可选)"
    echo ""
    echo "示例:"
    echo "  $0 /home/usr/CodeTrans_c2Rust/dataset/quadtree_0_1_0_expanded_dealed"
    echo "  $0 /home/usr/CodeTrans_c2Rust/dataset/quadtree_0_1_0_expanded_dealed analyze_advanced.cpp"
    echo ""
    exit 1
}

        
if [ $# -eq 0 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    show_usage
fi

if [ $# -lt 1 ] || [ $# -gt 2 ]; then
    echo "错误: 参数数量不正确"
    echo ""
    show_usage
fi

      
                                                   
PROJECT_DIR="$PROJECT_ROOT"
SOURCE_DIR="$1"                                        
LLVM_VERSION="14"                                 
SVF_ROOT="$PROJECT_DIR/dependencyLib/SVF-SVF-2.9"         


                     
ANALYZE_CPP="PA_func.cpp"
if [ $# -eq 2 ]; then
    ANALYZE_CPP="$2"
    echo "使用指定的分析文件: $ANALYZE_CPP"
else
    echo "使用默认分析文件: $ANALYZE_CPP"
fi

      
echo "正在验证参数..."

         
if [ ! -d "$SOURCE_DIR" ]; then
    echo "错误: 源代码目录不存在: $SOURCE_DIR"
    echo "请确保提供正确的源代码目录路径"
    exit 1
fi

            
if [ ! -f "$SCRIPT_DIR/$ANALYZE_CPP" ]; then
    echo "错误: 指定的分析文件不存在: $SCRIPT_DIR/$ANALYZE_CPP"
    echo "可用的分析文件:"
    ls -1 "$SCRIPT_DIR/"*.cpp 2>/dev/null || echo "  未找到任何 .cpp 文件"
    exit 1
fi

         
if [ ! -d "$SVF_ROOT" ]; then
    echo "错误: SVF根目录不存在: $SVF_ROOT"
    echo "请检查SVF安装路径是否正确"
    exit 1
fi

                
C_FILES_COUNT=$(find "$SOURCE_DIR" -maxdepth 1 -name "*.c" | wc -l)
if [ "$C_FILES_COUNT" -eq 0 ]; then
    echo "警告: 源代码目录中未找到任何 .c 文件: $SOURCE_DIR"
    echo "请确认目录路径是否正确"
    read -p "是否继续执行? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "操作已取消"
        exit 1
    fi
fi

echo "参数验证通过!"
echo "源代码目录: $SOURCE_DIR"
echo "分析文件: $ANALYZE_CPP"
echo "找到 $C_FILES_COUNT 个 C 源文件"
echo ""

         
SVF_LLVM_DIR="$SVF_ROOT/svf-llvm"
SVF_CORE_DIR="$SVF_ROOT/svf"
SVF_BUILD_DIR="$SVF_ROOT/Release-build"

           
if [ ! -d "$SVF_BUILD_DIR" ] || [ ! -f "$SVF_BUILD_DIR/svf/libSvfCore.a" ] || [ ! -d "$SVF_BUILD_DIR/svf-llvm" ]; then
    echo "错误: SVF构建目录无效或未编译 $SVF_BUILD_DIR"
    echo "请检查是否在 $SVF_ROOT 中运行了以下命令并确保编译成功:"
    echo "  $ cd $SVF_ROOT"
    echo "  $ ./build.sh"
    exit 1
fi

         
SVF_CORE_LIB="$SVF_BUILD_DIR/svf/libSvfCore.a"
SVF_LLVM_LIB="$SVF_BUILD_DIR/svf-llvm/libSvfLLVM.a"

                            
SOURCE_DIR_NAME=$(basename "$SOURCE_DIR")
OUTPUT_DIR="$SOURCE_DIR/svf_analysis_output"

if [ -d "$OUTPUT_DIR" ]; then
                                 
    echo "清理输出目录（保留已有的JSON文件）..."
    find "$OUTPUT_DIR" -type f ! -name "*.json" -delete 2>/dev/null
else
                 
    echo "创建输出目录: $OUTPUT_DIR"
    mkdir -p "$OUTPUT_DIR"
fi

                
echo "正在生成LLVM IR文件..."
cd "$SOURCE_DIR" || exit 1

IR_COUNT=0
for src_file in *.c; do
    if [ ! -f "$src_file" ]; then
        echo "警告: 未找到C源文件"
        continue
    fi
    echo "处理文件: $src_file"
    # Both analyzers default to -O0.
    #   PA_struct_optimized → -O0. Its ownership analysis (PtrTrans rules)
    #     runs on the SVFIR + Andersen points-to. At -O0 every struct field
    #     access is a regular `getelementptr %struct.S ... i32 N`, so field
    #     addressing is exact; at -O1 the optimiser lowers some accesses to
    #     raw byte-offset GEPs which SVF collapses to offset_0 (a write/
    #     alloc/free done only in that form would be invisible). The old
    #     per-field IR-text scans that made -O0 time out have been removed
    #     — detection is now single-pass PAG indexing + bounded BFS.
    #
    #   PA_func             → -O0 (inlining hides fn-arg pointer facts:
    #     a fn whose body is inlined into all callers shows up with NO
    #     ptr params in the IR, so PA_func emits no facts for it. Empirically
    #     bzip2 dropped from 174 → 108 fn-arg facts when PA_func ran on
    #     -O1 IR, and the walker silently skipped 27 extra fns as "no
    #     pointer facts; nothing to do". For function-level analysis
    #     PA_func tolerates O0 IR — its complexity is param-count-driven
    #     not call-edge-count-driven.)
    #
    # Override with SA_OPT_LEVEL=O0|O1.
    DEFAULT_OPT="O0"
    OPT_LEVEL="${SA_OPT_LEVEL:-$DEFAULT_OPT}"
    O0_FLAGS=""
    if [ "$OPT_LEVEL" = "O0" ]; then
        O0_FLAGS="-Xclang -disable-O0-optnone"
    fi
    # SA_CFLAGS_EXTRA: caller can inject -I, -D, -include flags for
    # projects whose .c files aren't self-contained at top-level (e.g.
    # heman includes <heman.h> from include/, kazmath/vec3.h from kazmath/;
    # lodepng.cpp gates std::cout behind LODEPNG_NO_COMPILE_CPP). Splits
    # on whitespace — quoted args not supported.
    if clang-$LLVM_VERSION -S -emit-llvm -g -$OPT_LEVEL $O0_FLAGS -fno-discard-value-names \
        -femit-all-decls \
        -D_Float128="long double" \
        ${SA_CFLAGS_EXTRA:-} \
        "$src_file" -o "$OUTPUT_DIR/${src_file%.c}.ll" 2>/dev/null; then
        ((IR_COUNT++))
    else
        # Skip a single non-compilable TU instead of failing the whole run.
        # A project often has one peripheral file that needs a build-system
        # define the flat compile lacks (e.g. fzy's options.c uses VERSION,
        # injected via `-DVERSION=...` in the Makefile, not any header). The
        # hot/liftable files still compile; partial SA facts beat none. The
        # "0 IR files" guard below still fails the run if NOTHING compiled.
        echo "生成IR失败(跳过该文件,继续其余): $src_file"
        SKIP_COUNT=$(( ${SKIP_COUNT:-0} + 1 ))
    fi
done

echo "成功生成 $IR_COUNT 个 LLVM IR 文件 (跳过 ${SKIP_COUNT:-0} 个无法编译的文件)"

                             
echo "正在链接所有 .ll 文件..."
LINKED_LL_FILE="$OUTPUT_DIR/linked_program.ll"

              
LLVM_IR_FILES=("$OUTPUT_DIR"/*.ll)
if [ ${#LLVM_IR_FILES[@]} -eq 0 ] || [ ! -f "${LLVM_IR_FILES[0]}" ]; then
    echo "错误: 未找到任何LLVM IR文件"
    exit 1
fi

# ============================================================================
                                             
                                     
                            
# ============================================================================
echo "预处理 .ll 文件：将 internal 替换为 weak 链接以保留 static 函数..."
for ll_file in "${LLVM_IR_FILES[@]}"; do
    if [ -f "$ll_file" ]; then
                                                     
                                     
        # Make every cross-TU symbol weak so llvm-link can absorb duplicates.
        # Multi-main projects (e.g. bzip2 has bzip2.c + bzip2recover.c) define
        # both `main` and statics like `progName` in two TUs; without this
        # the linker errors out with "symbol multiply defined".
        #   1. `define [internal|dso_local|weak|private]? <ret>` → `define weak <ret>`
        #   2. `declare internal` → `declare`
        #   3. global/constant `dso_local` → `weak`
        sed -E '
            /^define / s/^define (internal |dso_local |weak |private )?/define weak /
            s/declare internal /declare /g
            s/^(@[A-Za-z_][A-Za-z0-9_.]* = )dso_local (global|constant)/\1weak \2/g
            s/^(@[A-Za-z_][A-Za-z0-9_.]* = )(global|constant)/\1weak \2/g
        ' "$ll_file" > "${ll_file}.tmp"
        mv "${ll_file}.tmp" "$ll_file"
        echo "  已处理: $(basename "$ll_file")"
    fi
done
echo "预处理完成"

                          
if llvm-link-$LLVM_VERSION -S "${LLVM_IR_FILES[@]}" -o "$LINKED_LL_FILE"; then
    echo "所有 .ll 文件已链接到: $LINKED_LL_FILE"
else
    echo "链接 .ll 文件失败"
    exit 1
fi

              
echo "编译SVF分析程序..."
cd "$OUTPUT_DIR" || exit 1

                                                                       
                                                
if [ -z "$NLOHMANN_INCLUDE" ] || [ ! -f "$NLOHMANN_INCLUDE/json.hpp" ]; then
    echo "错误: 未找到 nlohmann/json (NLOHMANN_INCLUDE=$NLOHMANN_INCLUDE)"
    echo "请安装: sudo apt install nlohmann-json3-dev"
    exit 1
fi

              
SVFCORE_LIB=$(find "$SVF_BUILD_DIR" -name "libSvfCore.a")
SVFLLVM_LIB=$(find "$SVF_BUILD_DIR" -name "libSvfLLVM.a")

if [ -z "$SVFCORE_LIB" ] || [ -z "$SVFLLVM_LIB" ]; then
    echo "错误: 找不到SVF库文件，请确保SVF已正确编译"
    exit 1
fi

echo "找到 SVF Core 库: $SVFCORE_LIB"
echo "找到 SVF LLVM 库: $SVFLLVM_LIB"

                
echo "正在编译分析程序..."
if clang++-$LLVM_VERSION -std=c++17 \
    -I "$SVF_LLVM_DIR/include" \
    -I "$SVF_CORE_DIR/include" \
    -I "$SVF_BUILD_DIR/include" \
    -I "/usr/lib/llvm-$LLVM_VERSION/include" \
    -I "$(dirname "$NLOHMANN_INCLUDE")" \
    "$SCRIPT_DIR/$ANALYZE_CPP" \
    -o svf_pointer_analysis \
    -Wl,--start-group "$SVFLLVM_LIB" "$SVFCORE_LIB" -Wl,--end-group \
    -L "/usr/lib/llvm-$LLVM_VERSION/lib" \
    -lLLVM -lz -lpthread -ldl -ltinfo -lm -lz3; then
    echo "分析程序编译成功"
else
    echo "编译SVF分析程序失败"
    exit 1
fi

           
echo "运行指针分析..."
if [ ! -f "$LINKED_LL_FILE" ]; then
    echo "错误: 链接后的 .ll 文件不存在: $LINKED_LL_FILE"
    exit 1
fi

echo "正在分析 $(basename "$SOURCE_DIR") 项目..."

# Hard timeout. Whole-program Andersen on -O0 IR is slow on call-edge-heavy,
# FORCE_INLINE-heavy projects (lz4 -O0 has 8902 call edges → PA_struct ~21min;
# libcsv ~17s, bzip2 ~1min are typical). Default 1800s; override with
# SA_TIMEOUT_SEC. Exit code 124 = timeout (per coreutils `timeout`).
SA_TIMEOUT_SEC="${SA_TIMEOUT_SEC:-1800}"
echo "(timeout: ${SA_TIMEOUT_SEC}s, opt level: $OPT_LEVEL)"

LD_LIBRARY_PATH="$SVF_BUILD_DIR/lib:$LD_LIBRARY_PATH" \
    timeout -k 10 "$SA_TIMEOUT_SEC" \
    ./svf_pointer_analysis "$LINKED_LL_FILE"
RC=$?
# `timeout` exits 124 on SIGTERM; 137 on SIGKILL (after -k window).
# (No --preserve-status: that flips 124 → signal exit code 143 and
# breaks our caller-side detection.)

if [ $RC -eq 0 ]; then
    echo ""
    echo "=== 分析完成! ==="
    echo "源代码目录: $SOURCE_DIR"
    echo "分析结果保存到: $OUTPUT_DIR/analysis_result.json"
    echo "输出目录: $OUTPUT_DIR"

               
    if [ -f "$OUTPUT_DIR/analysis_result.json" ]; then
        FILE_SIZE=$(du -h "$OUTPUT_DIR/analysis_result.json" | cut -f1)
        echo "结果文件大小: $FILE_SIZE"
    fi
elif [ $RC -eq 124 ] || [ $RC -eq 137 ]; then
    # 124 = SIGTERM from `timeout`; 137 = SIGKILL (-k window). Both mean
    # the analysis exceeded the wall budget. Exit 124 unconditionally so
    # the caller can distinguish from generic SVF crashes.
    echo "指针分析超时 (>${SA_TIMEOUT_SEC}s, opt=$OPT_LEVEL)"
    exit 124
else
    echo "指针分析执行失败 (rc=$RC)"
    exit 1
fi