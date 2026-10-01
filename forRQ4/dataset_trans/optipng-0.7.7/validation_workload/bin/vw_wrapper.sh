#!/bin/bash
# shared wrapper: vw_wrapper.sh <real_bin> <mode> <input> <iters> [optipng args...]
BIN="$1"; MODE="$2"; INPUT="$3"; ITERS="$4"; shift 4
case "$MODE" in
  out)
    OUT="/tmp/vw_optipng_out.png"
    for ((i=0; i<ITERS; i++)); do
      rm -f "$OUT"
      "$BIN" "$@" -quiet -force -out "$OUT" "$INPUT" || exit 3
    done
    sha256sum < "$OUT"; rm -f "$OUT" ;;
  report)                       # non-quiet: fold the size/ratio report too
    OUT="/tmp/vw_optipng_out.png"
    for ((i=0; i<ITERS; i++)); do
      rm -f "$OUT"
      "$BIN" "$@" -force -out "$OUT" "$INPUT" 2>/dev/null || exit 3
    done
    sha256sum < "$OUT"; rm -f "$OUT" ;;
  inplace)                      # real in-place + -keep backup path
    D=/tmp/vw_optipng_ip
    for ((i=0; i<ITERS; i++)); do
      rm -rf $D; mkdir -p $D
      cp "$INPUT" $D/x.png
      "$BIN" "$@" -quiet -keep $D/x.png || exit 3
    done
    sha256sum < $D/x.png
    ls $D | sort | tr "\n" " "; echo
    rm -rf $D ;;
  expectfail)                   # error-path surface: fold exit codes
    for ((i=0; i<ITERS; i++)); do
      "$BIN" "$@" -quiet -force -out /tmp/vw_optipng_bad.png "$INPUT" >/dev/null 2>&1
      echo "exit=$?"
    done
    rm -f /tmp/vw_optipng_bad.png ;;
esac
