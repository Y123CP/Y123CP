"""Safety metrics for a Rust crate (tree-sitter, read-only).

Counts the surfaces the safety lift is supposed to shrink. Comparable
across pipeline stages and across baselines (c2rust raw / Crown / ours):

  unsafe_fn        `unsafe fn` items (incl. `unsafe extern "C" fn`)
  unsafe_block     `unsafe { … }` blocks
  ptr_type         `*mut T` / `*const T` type nodes (params, fields, lets, casts)
  offset_calls     `.offset(` method calls (raw pointer arithmetic)
  raw_parts        `from_raw_parts[_mut](` view-construction sites
  deref_sites      unary `*expr` expressions — NOTE: includes derefs of
                   safe references, so its absolute value overstates raw
                   derefs; DELTAS between two variants of the same crate
                   are meaningful.
  loc              non-blank lines (context scale)

Usage:
    python -m tools.safety_metrics <crate_dir> [<crate_dir> ...] [--json]
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

from perf_opt.stage_a.intra_ptr.collect import _PARSER, _txt


def _walk(n):
    st = [n]
    while st:
        x = st.pop()
        st.extend(x.children)
        yield x


_RAW_PARTS = re.compile(r"\bfrom_raw_parts(_mut)?\s*\(")


def measure(crate_dir: Path) -> dict:
    m = dict(unsafe_fn=0, unsafe_block=0, ptr_type=0, offset_calls=0,
             raw_parts=0, deref_sites=0, loc=0, files=0)
    for f in sorted(crate_dir.rglob("*.rs")):
        if "target" in f.parts or f.name == "build.rs":
            continue
        src = f.read_bytes()
        m["files"] += 1
        m["loc"] += sum(1 for ln in src.splitlines() if ln.strip())
        m["raw_parts"] += len(_RAW_PARTS.findall(src.decode("utf-8", "replace")))
        for n in _walk(_PARSER.parse(src).root_node):
            t = n.type
            if t == "function_item":
                if any(c.type == "function_modifiers" and
                       "unsafe" in _txt(src, c) for c in n.children):
                    m["unsafe_fn"] += 1
            elif t == "unsafe_block":
                m["unsafe_block"] += 1
            elif t == "pointer_type":
                m["ptr_type"] += 1
            elif t == "call_expression":
                fn = n.child_by_field_name("function")
                if fn is not None and fn.type == "field_expression":
                    fld = fn.child_by_field_name("field")
                    if fld is not None and _txt(src, fld) == "offset":
                        m["offset_calls"] += 1
            elif t == "unary_expression" and \
                    _txt(src, n).lstrip().startswith("*"):
                m["deref_sites"] += 1
    return m


def main() -> int:
    ap = argparse.ArgumentParser(description="crate safety metrics")
    ap.add_argument("crates", nargs="+", type=Path)
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()
    rows = [(str(c), measure(c)) for c in args.crates]
    if args.json:
        print(json.dumps(dict(rows), indent=1))
        return 0
    keys = ["unsafe_fn", "unsafe_block", "ptr_type", "offset_calls",
            "raw_parts", "deref_sites", "loc"]
    print(f"{'crate':58s}" + "".join(f"{k:>13s}" for k in keys))
    for name, m in rows:
        print(f"{name[-58:]:58s}" + "".join(f"{m[k]:>13d}" for k in keys))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
