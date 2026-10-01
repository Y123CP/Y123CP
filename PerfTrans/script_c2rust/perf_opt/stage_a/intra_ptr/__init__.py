"""Stage A intra-procedural pointer-view recovery (`e_intra_ptr`).

Signature-frozen body safety lift: re-express c2rust raw-pointer derefs /
`.offset()` as slice / reference views *in place*, without touching the fn
signature. See `docs/stage_a_safety_lift_design.md`.

P1 (this milestone) = the read-only INFORMATION layer only:
  · collect.py — tree-sitter per-fn raw-pointer fact collection (§4.1)
  · (later) plan.py — pointer-flow family + Kind A/B + order
No code rewriting, no LLM. Output is a JSON dump for human review of
collected accuracy before any rewrite is attempted.
"""
