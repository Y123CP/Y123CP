"""Stage A intra_ptr — PLAN (lightweight / safe-floor form).

Turns COLLECT's isolated per-function pointer facts into lift units — the
decision + ordering layer. THIS module implements only the **safe floor**
(design doc §5.1 / P2 first milestone):

    Kind A (signature-frozen) + Immutable views only.

The safe floor is deliberately function-INDEPENDENT, so it needs no
pointer-flow family clustering and no SVF: a read-only `&[T]`/`&T` view has
no aliasing UB, and `cargo check` is a complete oracle for it. The heavier
PLAN (Kind B param lift, family clustering via SA caller_arg_id, the
same_object alias gate, mutable views) is a later milestone and will extend
this module.

Eligibility for the floor (every gate is a correctness requirement):
  · ptr_depth == 1          — no `*mut *mut T` (design §2.4.3 double-ptr)
  · not cursor_reassigned   — a reassigned `p` breaks a fixed slice view
  · not body_mutates        — IMMUTABLE only (mutable needs the alias gate)
  · typed inner             — not c_void / unknown (can't form a typed view)
  · not passed_to_calls     — a callee might mutate `p` while our shared view
                              is live (aliasing UB only SVF can rule out) →
                              conservatively deferred to the full PLAN
  · length available        — Array needs a resolved length; Scalar doesn't

Output is a per-pointer plan + a unit list, dumped to JSON for human review
before any rewrite. PLAN rewrites nothing and calls no LLM.
"""

from __future__ import annotations

import logging
from dataclasses import dataclass, field, asdict
from pathlib import Path

from .collect import PtrParamFacts, collect_crate

logger = logging.getLogger("intra_ptr.plan")

_VOID_INNERS = {"c_void", "core::ffi::c_void", "std::os::raw::c_void", "()"}

# Inner types with NO pointer fields → a `&mut T` view cannot be invalidated
# by an interior back-pointer aliasing the same object (the `s.strm.state == s`
# pattern). Mutable views are restricted to these until a struct-field scan
# can prove a struct carries no self-referential/aliasing pointer. Primitive
# scalars + c2rust's UInt64-style byte-array structs qualify; arbitrary
# `*mut SomeStruct` does not.
_PRIMITIVE_INNERS = {
    "u8", "u16", "u32", "u64", "u128", "usize",
    "i8", "i16", "i32", "i64", "i128", "isize",
    "f32", "f64", "bool", "char",
    "c_char", "c_uchar", "c_schar", "c_short", "c_ushort", "c_int", "c_uint",
    "c_long", "c_ulong", "c_longlong", "c_ulonglong", "c_float", "c_double",
    "size_t", "ssize_t", "off_t", "ptrdiff_t", "intptr_t", "uintptr_t",
    "UChar", "UInt16", "UInt32", "Int32", "Bool", "Char", "UInt64",
}


def _is_primitive_inner(inner: str) -> bool:
    t = inner.strip().lstrip("*").strip()
    t = t.rsplit("::", 1)[-1]            # std::os::raw::c_int → c_int
    return t in _PRIMITIVE_INNERS


# ─────────────────────────────────────────────────────────────────
# SVF facts (sa_engine) — replaces the airtight structural gates with a
# sound no-alias + no-mutation PROOF where the SA report is available.
# ─────────────────────────────────────────────────────────────────

def _sa_dict_from_report(rep: Path) -> dict[tuple[str, str], object]:
    """{(fn_name, param_name): SAParam} from a func_analysis_report.json."""
    try:
        from perf_opt.stage_a.sa_utils import _load_sa_facts
    except ImportError:
        return {}
    if not rep or not rep.is_file():
        return {}
    out: dict[tuple[str, str], object] = {}
    for fn, params in _load_sa_facts(rep).items():
        for p in params:
            out[(fn, p.name)] = p
    logger.info(f"[plan] SVF facts loaded from {rep.parent.parent.name}: "
                f"{len(out)} ptr-param facts")
    return out


def _sa_lookup(crate_dir: Path) -> dict[tuple[str, str], object]:
    """{(fn_name, param_name): SAParam} from the project's sa_engine report,
    or {} if unavailable. Project name = crate_dir.parent.name (works for
    `<proj>/1_cleaned` and a `<work_root=proj>/crate` copy, NOT a
    `<proj>/2_stage_a/crate` pipeline copy — there pass `sa_report=` explicitly
    via e2.find_sa_report)."""
    try:
        from Config.paths import get_path
    except ImportError:
        return {}
    base = get_path("SOURCE_PROJECT_BASE")
    if not base:
        return {}
    proj = crate_dir.parent.name
    rep = Path(base) / proj / "svf_analysis_output" / "func_analysis_report.json"
    return _sa_dict_from_report(rep)


@dataclass
class PtrLiftPlan:
    fn_name: str
    fn_file: str
    anchor: str
    current_type: str
    inner_type: str | None
    n_use_sites: int
    # decision
    eligible: bool
    kind: str                       # "A" (floor) | "skip"
    frozen: bool                    # always True in the floor
    target_view: str | None         # "&[UChar]" / "&Foo" / None
    length: str | None              # for Array views
    needs_no_alias: bool            # mutable views (not in floor) would set this
    skip_reason: str | None
    null_guarded: bool = False      # Tier 2: body null-checks p — the view must
                                    # be built INSIDE the guard (LLM-only)
    is_local: bool = False          # Tier 3a: cast-from-param local — the view
                                    # is built at the `let` decl, not body-top
    source_param: str | None = None

    def to_json(self) -> dict:
        return asdict(self)


def _plan_one(f: PtrParamFacts, airtight: bool = True,
              sa=None, allow_null_guarded: bool = False,
              allow_mutable_struct: bool = False) -> PtrLiftPlan:
    """Decide the safe-floor plan for one pointer param. Gates are checked in
    a fixed order so `skip_reason` names the FIRST disqualifier.

    `airtight=True` (option 3 — the SVF-free milestone): add gates that make
    the no-aliasing obligation tree-sitter-PROVABLE, so cargo check is a
    complete oracle and we never lean on W1 for soundness:
      · exactly one ptr param  — no sibling pointer can alias the buffer
      · no local raw pointer    — no interior-pointer extraction (the
                                  `let s = (*p).field` back-alias, e.g.
                                  handle_compress::strm)
      · function makes no call  — no callee can mutate the buffer
    Under these three, `p` is the ONLY path to its buffer and nothing in the
    call can write it → a read-only view cannot be invalidated. Coverage is
    small by design; SVF (later) replaces these three with same_object +
    cross-callee mutation facts to expand it."""
    base = dict(
        fn_name=f.fn_name, fn_file=f.fn_file, anchor=f.anchor,
        current_type=f.current_type, inner_type=f.inner_type,
        n_use_sites=len(f.use_sites),
        frozen=True, is_local=f.is_local, source_param=f.source_param,
    )

    def skip(reason: str) -> PtrLiftPlan:
        return PtrLiftPlan(eligible=False, kind="skip", target_view=None,
                           length=None, skip_reason=reason,
                           needs_no_alias=False, **base)

    if f.ptr_depth >= 2:
        return skip("double-ptr (ptr_depth>=2) — §2.4.3 defer")
    if f.cursor_reassigned:
        return skip("cursor reassigned — fixed view unsound")
    if f.inner_type is None or f.inner_type.strip() in _VOID_INNERS:
        return skip("void/untyped inner — no typed view")
    # Escape gate: any use we don't recognize as a deref ("other" = stored
    # into a struct field/global, cast to another ptr, returned, compared) is
    # a potential alias/reinterpret. e.g. BZ2_bzBuffToBuffCompress stores
    # `strm.next_out = dest` — dest is written through strm later; a view of
    # dest would alias a mutated region. Defer.
    if any(u.kind == "other" for u in f.use_sites):
        return skip("escapes via non-deref use (store/cast/return/cmp) — "
                    "aliasing/reinterpret risk")
    # Null-guard gate: if the body null-checks p (`p.is_null()`), the original
    # derefs are CONDITIONAL on non-null. A view built UNCONDITIONALLY at
    # body-top (`&*p`) would deref null on the guarded path → ADDS a deref the
    # original didn't do. The deterministic rewriter can't place the view
    # conditionally, so it skips these. Tier 2 (LLM-only): when
    # `allow_null_guarded`, KEEP the param eligible but flag it — the LLM
    # builds the view INSIDE the existing null-guard (keeping the null check),
    # which the deterministic rewriter cannot do. SVF nullability is always
    # "Nullable" (no signal), so the in-body is_null check is the only cue.
    null_guarded = any(u.kind == "is_null" for u in f.use_sites)
    if null_guarded and not allow_null_guarded:
        return skip("null-guarded (is_null in body) — top-of-body view would "
                    "deref null")
    # Must have something to rewrite: at least one read OR write deref. (Write
    # derefs are actionable for `&mut` views.) A ptr used only via `is_null()`
    # has no actionable site.
    _ACTIONABLE = {"offset_read", "offset_write", "deref_read", "deref_write",
                   "field_read", "field_write"}
    if not any(u.kind in _ACTIONABLE for u in f.use_sites):
        return skip("no actionable deref (only is_null/etc.)")

    # ── Decide view mutability + soundness ──
    # `mutable` view (`&mut T`/`&mut [T]`) needs EXCLUSIVE access for its
    # lifetime; `&T`/`&[T]` only needs no-mutation. Step 2 (2026-06-25) added
    # the mutable path on top of the immutable floor.
    mutable = f.body_mutates
    if sa is not None:
        # SVF-backed proof. Andersen over-approximates may-alias, so
        # same_object=False is a sound no-alias proof; Mutable/Owning/
        # same_object=True are the positive findings that exclude.
        if sa.ownership != "Borrowed":
            return skip(f"SVF: ownership={sa.ownership} (not Borrowed)")
        if mutable:
            # No-alias is needed ONLY for `&mut` (exclusivity). Andersen's
            # same_object is field-INSENSITIVE, so distinct struct-field arrays
            # (bzip2 EState arr1/block/ftab) all report may-alias — a false
            # positive, but for `&mut` we stay conservative. single-ptr fn has
            # no sibling (vacuous); multi-ptr needs every callsite same_object
            # =False.
            noalias_ok = (f.fn_n_ptr_params == 1) or sa.noalias_all_sites
            if not noalias_ok:
                return skip("SVF: same_object — may-alias a sibling arg (mutable)")
            # `&mut` exclusivity: the body writes through p, so p must be the
            # ONLY access path for the view's lifetime. no-alias rules out
            # sibling args; passing p to a callee would give a second live
            # access path → forbid (SVF mutability cannot prove the callee
            # doesn't TOUCH it, only whether it writes).
            if f.passed_to_calls:
                return skip("mutable: p passed to call — exclusivity unprovable")
            # Interior back-pointer guard: a `&mut Struct` can be invalidated
            # if the struct holds a pointer that aliases the same object
            # (`s.strm.state == s`). same_object is arg-only, so by default
            # restrict mutable to inner types with no pointer fields. Tier 3
            # (`allow_mutable_struct`): the dominant raw-ptr uses are mutable
            # struct handles (bzip2 DState 1495 + EState 589 uses) — blocking
            # them all is the 5% ceiling. With SA already proving Borrowed +
            # no-sibling-alias + not-passed-to-call, allow struct &mut and let
            # W1 (byte-exact) be the judge of the interior-back-alias residual.
            if not allow_mutable_struct and not _is_primitive_inner(f.inner_type):
                return skip("mutable: non-primitive inner — interior back-alias "
                            "unprovable (struct &mut deferred)")
        else:
            # immutable view: require SVF proves NO write to the pointee within
            # the call (incl. via interior back-pointers / callees). body not
            # writing p but SVF Mutable ⇒ written via an alias/callee ⇒ unsound.
            if sa.mutability != "Immutable":
                return skip("SVF: pointee Mutable (written via alias/callee)")
    else:
        # No SVF. Tree-sitter proxies only.
        if f.passed_to_calls:
            return skip(f"passed to call(s) {f.passed_to_calls} — cross-callee "
                        f"mutation aliasing (need SVF)")
        if mutable:
            return skip("mutable — needs SVF no_alias proof")
        if airtight:
            if f.fn_n_ptr_params != 1:
                return skip("airtight: fn has multiple ptr params (sibling "
                            "alias unprovable without SVF)")
            if f.fn_has_local_ptr:
                return skip("airtight: fn derives a local raw pointer "
                            "(interior-pointer back-alias)")
            if f.fn_makes_call:
                return skip("airtight: fn makes a call (callee may mutate)")

    inner = f.inner_type.strip()
    mut_kw = "mut " if mutable else ""
    if f.count == "Array":
        if f.length_source.kind == "UNKNOWN" or not f.length_source.expr:
            return skip("Array with unresolved length")
        return PtrLiftPlan(eligible=True, kind="A",
                           target_view=f"&{mut_kw}[{inner}]",
                           length=f.length_source.expr,
                           needs_no_alias=mutable, skip_reason=None,
                           null_guarded=null_guarded, **base)
    return PtrLiftPlan(eligible=True, kind="A",
                       target_view=f"&{mut_kw}{inner}",
                       length=None, needs_no_alias=mutable,
                       skip_reason=None, null_guarded=null_guarded, **base)


# ─────────────────────────────────────────────────────────────────
# Crate-level plan
# ─────────────────────────────────────────────────────────────────

@dataclass
class LiftUnit:
    """One atomic rewrite unit. In the floor, a unit = one function with its
    eligible (frozen-sig, immutable) pointer views; the whole function is
    verified/rolled-back atomically."""
    fn_name: str
    fn_file: str
    kind: str                       # "A"
    pointers: list[PtrLiftPlan]
    priority: int                   # higher = do earlier (more derefs cleaned)


@dataclass
class CratePlan:
    plans: list[PtrLiftPlan] = field(default_factory=list)   # every ptr param
    units: list[LiftUnit] = field(default_factory=list)      # eligible, grouped


def plan_crate(crate_dir: Path,
               only_fns: set[str] | None = None,
               airtight: bool = True,
               use_svf: bool = True,
               sa_lookup_dir: Path | None = None,
               sa_report: Path | None = None,
               allow_null_guarded: bool = False,
               allow_mutable_struct: bool = False) -> CratePlan:
    """`sa_report` (preferred when known): direct path to the project's
    func_analysis_report.json. `sa_lookup_dir`: a `<proj>/1_cleaned`-shaped
    dir to derive the project name from, when scanning a relocated work copy.
    If neither is given, derive from `crate_dir.parent.name`."""
    facts = collect_crate(crate_dir, only_fns=only_fns)
    if not use_svf:
        sa = {}
    elif sa_report is not None:
        sa = _sa_dict_from_report(sa_report)
    else:
        sa = _sa_lookup(sa_lookup_dir or crate_dir)
    plans = [_plan_one(
                f, airtight=airtight,
                # Tier 3a: a cast-from-param local inherits its SOURCE param's
                # SA fact (Andersen preserves points-to across the cast).
                sa=sa.get((f.fn_name, f.source_param if f.is_local else f.anchor)),
                allow_null_guarded=allow_null_guarded,
                allow_mutable_struct=allow_mutable_struct)
             for f in facts]

    # Group eligible plans by function into atomic units.
    by_fn: dict[tuple[str, str], list[PtrLiftPlan]] = {}
    for p in plans:
        if p.eligible:
            by_fn.setdefault((p.fn_file, p.fn_name), []).append(p)
    units: list[LiftUnit] = []
    for (fn_file, fn_name), ps in by_fn.items():
        # priority = total deref/offset use-sites recovered (safety yield)
        prio = sum(p.n_use_sites for p in ps)
        units.append(LiftUnit(fn_name=fn_name, fn_file=fn_file, kind="A",
                              pointers=ps, priority=prio))
    # Floor order: highest-yield functions first (purely safety-yield —
    # NOT hotness; that is Stage B's axis).
    units.sort(key=lambda u: (-u.priority, u.fn_file, u.fn_name))
    return CratePlan(plans=plans, units=units)
