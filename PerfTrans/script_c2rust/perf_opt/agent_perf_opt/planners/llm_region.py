"""Plan one fenced LLM source-region replacement."""

from __future__ import annotations

from dataclasses import dataclass
import hashlib
import json
import re

from perf_opt.agent_perf_opt.changeset.handlers.replace_region import (
    validate_region_replacement_source,
)
from perf_opt.agent_perf_opt.changeset.types import (
    ImpactScope,
    ProposedChangeSet,
    RegionRef,
    ReplaceSourceRegion,
    TriggerContext,
)
from perf_opt.agent_perf_opt.regions.model import (
    RuleCapability,
    rule_capability,
    rules_region_local_for,
)
from perf_opt.agent_perf_opt.rewrite_applier import (
    parse_applied_rules,
    parse_llm_response,
    parse_skipped_rules,
)


# greedy `.*`, not `[^\]]*` — the list ends at the LAST `]` on the line.
# See `_APPLIED_RULES_RE` in rewrite_applier for what the narrow form cost:
# a skip reason quoting a Rust array type (`[*mut Node; 2]`) closed the
# bracket early, `$` failed, and a complete correct rewrite was discarded as
# "headers missing". These two must stay in step with that module's pair —
# this one gates the response, that one parses it.
_APPLIED_HEADER = re.compile(
    r"^\s*//\s*Applied\s+rules\s*:\s*\[.*\][ \t\r]*$",
    re.IGNORECASE,
)
_SKIPPED_HEADER = re.compile(
    r"^\s*//\s*Skipped\s+rules\s*:\s*\[.*\][ \t\r]*$",
    re.IGNORECASE,
)
_EXACT_FENCE_RESPONSE = re.compile(
    r"\A```(?:rust)?[ \t]*\n(?:(?!```).)*\n```[ \t]*\Z",
    re.DOTALL,
)


# ── Contract repair ────────────────────────────────────────────────────
#
# These abstain reasons say nothing about the REWRITE — only that the reply
# was not shaped the way the contract asks. The rewrite itself was never
# evaluated. Discarding it costs whatever it was worth, and the loss is
# invisible: the run records "rule did not apply", the region's anchors are
# retired, and it is never offered again.
#
# That is not hypothetical. Two consecutive runs of one crate lost the same
# rewrite this way — the model applied three rules correctly, including a
# `qsort` → `sort_unstable_by` conversion hand-measured at -30.72% on one
# operation, and the reply was thrown away because a skip reason quoted an
# array type. Fixing that one bracket does not close the class: any future
# punctuation the parser does not anticipate lands in exactly the same place.
#
# So a reply that fails on FORM gets one more chance, with the previous reply
# and the specific violation quoted back. What is deliberately NOT here:
# `no_applied_rules` (the model declined every rule — a real judgment, and
# re-asking is pestering it) and the region-shape codes (the rewrite BODY is
# wrong, so "send the same thing again" is the wrong instruction).
_CONTRACT_REPAIRABLE: dict[str, str] = {
    "metadata_headers_missing_or_not_first":
        "the first two lines of the fenced block must be exactly "
        "`// Applied rules: [...]` then `// Skipped rules: [...]`, each "
        "complete on ONE line",
    "replacement_metadata_header":
        "the two header lines appeared again inside the replacement body; "
        "they belong only at the very top, once",
    "response_fence_contract":
        "the reply must be one fenced block and nothing else — no prose "
        "before it, no commentary after it, no second fence inside it",
    "parse_fail":
        "no ```rust fence was found in the reply",
    "empty_response":
        "the reply was empty",
    "duplicate_declared_rule":
        "a rule id was listed twice",
    "candidate_rule_coverage_incomplete":
        "every candidate rule must appear exactly once — either in "
        "`Applied rules` or in `Skipped rules` with a reason",
    "skipped_rule_reason_empty":
        "every skipped rule needs a non-empty reason after its colon",
    # Whole-function envelope failures. Same character as the ones above: the
    # rewrite inside may be entirely correct and was never judged. Documented
    # cost of discarding one — a correct rewrite of a crate's hottest function
    # (99.75% self time in its op) was thrown away because it nested a helper,
    # and the planner never returned to that function (see
    # test_single_function_validation).
    "replacement_function_count":
        "return exactly ONE top-level function; a helper must be nested "
        "inside its body, not placed beside it",
    "replacement_function_name":
        "the function must keep its original name exactly",
    "replacement_extra_source":
        "nothing may follow the function — the replacement is spliced into "
        "its span, so trailing items would be duplicated into the file",
}


def _reason_code(reason: str) -> str:
    """The bare code, with any `: <detail>` explanation stripped.

    Planners attach a human-readable detail to some abstains
    (`"<code>: <detail>"`). Matching the whole string against the table would
    silently fail on exactly those, which are the informative ones.
    """
    return reason.split(":", 1)[0].strip()


def is_contract_repairable(reason: str | None) -> bool:
    """True when the reply failed on FORM, leaving the rewrite unjudged."""
    if not isinstance(reason, str) or not reason:
        return False
    return (
        _reason_code(reason) in _CONTRACT_REPAIRABLE
        or reason.startswith("unknown_declared_rule:")
        or reason.startswith("rule_declared_in_both:")
    )


# Post-validation findings a further turn can act on. Deliberately a
# whitelist: `operation_edit_bijection` and friends report that the machinery
# disagreed with itself, and no amount of rewriting by the model fixes that.
_POST_VALIDATION_REPAIRABLE = frozenset({"added_bounds_check_in_loop"})


def is_post_validation_repairable(code: str | None) -> bool:
    """True when a guard rejected the rewrite on a shape the model can fix.

    Such a rejection is not a verdict on whether the rewrite is faster. It is
    a mechanical finding that names the offending expression — the same class
    of feedback as a compile error, and the class the retry loop was built
    for. Routing it to ABSTAINED filed it under "the model's own judgment"
    instead, where nothing retries.

    What that cost, measured: libqrencode's `Mask_calcRunLengthH` was rejected
    in four consecutive runs and never once landed, and fzy's
    `precompute_bonus` and lil's `hm_destroy` each threw away a full LLM turn
    plus a build the same way.
    """
    return isinstance(code, str) and code in _POST_VALIDATION_REPAIRABLE


def post_validation_repair_note(code: str, detail: str, previous: str) -> str:
    """The corrective turn: the finding, and the reply that triggered it."""
    if code == "added_bounds_check_in_loop":
        what = (
            "Those expressions sit INSIDE a loop and index a slice, so they "
            "pay a bounds check on EVERY iteration. The c2rust original used "
            "raw pointer arithmetic there and paid none, so the rewrite is "
            "buying safety the original never bought — measured at 8.35% on "
            "one crate's hottest loop.\n\n"
            "Rewrite so those sites add no check: `get_unchecked` / "
            "`get_unchecked_mut` in the existing `unsafe` context with a "
            "SAFETY comment proving the index is in range, or leave that one "
            "site on the raw pointer. Slicing ONCE outside the loop is fine "
            "— only per-iteration indexing is the problem.\n\n"
            "A fixed-size lookup table is NOT an exception. The index still "
            "comes from runtime data, so the check still runs every time.\n\n"
            "Keep the access pattern you had. A strided walk stays "
            "`(a..b).map(|i| *p.get_unchecked(i * stride))` — do NOT convert "
            "it to `.iter().step_by(stride)`, which re-derives the position "
            "each step (measured: -6.35% vs -0.02% on the same function)."
        )
    else:
        what = "Correct what the finding reports, and change nothing else."
    return (
        "\n\n## Previous attempt was rejected by a mechanical check\n"
        f"check: `{code}`\n"
        f"finding: {detail[:2000]}\n\n"
        f"{what}\n\n"
        f"Your previous output was:\n{quote_previous(previous)}\n\n"
        "Output the corrected rewrite in the same format contract (keep the "
        "// Applied rules: / // Skipped rules: headers)."
    )


# Reasons that assert another rule already did this rule's work. The claim is
# free to make, costs a whole rule when wrong, and is never tested.
_CLAIM_LEADS_WITHIN = 80
_SUBSUMPTION_WORDS = ("subsumed", "subsumes", "covered by", "redundant with",
                      "already covered", "no longer needed", "made redundant")


def find_untested_subsumption(
    skipped: "list[tuple[str, str]]",
) -> "tuple[str, str] | None":
    """The first rule dropped on the grounds that another rule covered it.

    Measured: on one crate's hottest function (69% of its operation) the model
    declared `[III④]` and skipped C3 with "subsumed by III④". C3 alone — a
    loop-invariant load hoisted out from behind an aliasing store — measures
    -4.123% there. The III④ rewrite that supposedly covered it measures
    +3.100%, and W2 rejected the whole thing, so the function shipped
    untouched. Neither claim was ever tested; the rule simply vanished into a
    skip reason.
    """
    for rule, reason in (skipped or ()):
        low = (reason or "").lower()
        # The claim must LEAD the reason, not trail it. Swept over the
        # dataset's 54 committed skip declarations, the bare form always
        # opens the reason ("subsumed by III④" — the whole reason, 16
        # characters; "Subsumed by III④: once the parser …") while three
        # others reach it only after 200+ characters of argument about the
        # code: "these are not a forward cursor walk over a stable buffer
        # but repeated access to the dynamically changing last byte of a
        # reallocatable vector … and is largely subsumed by". That is the
        # model doing its job and then adding a remark; re-offering the rule
        # against it buys an abstain and costs a build, a W1 and a W2.
        if any(low.find(w) != -1 and low.find(w) < _CLAIM_LEADS_WITHIN
               for w in _SUBSUMPTION_WORDS):
            return rule.strip(), (reason or "").strip()
    return None


def subsumption_repair_note(rule: str, reason: str, previous: str) -> str:
    """The corrective turn: apply the discarded rule, and only it."""
    return (
        "\n\n## Previous attempt was REJECTED as slower — and it skipped a rule\n"
        f"You skipped `{rule}` with:\n> {reason[:400]}\n\n"
        "That claim was never measured, and the rewrite that was supposed to "
        "cover it has now been rejected. So the evidence points the other "
        "way: the rule you dropped may be the one that pays.\n\n"
        f"Apply `{rule}` ALONE this time. Do not apply the rules from the "
        "previous attempt, and do not reproduce its rewrite — start from the "
        "original function and make only the change `{rule}`'s card "
        "prescribes. A small, local edit is the point.\n\n"
        # Deliberately an excerpt, unlike every other repair note: this one
        # tells the model to start over from the original, so the quote is
        # there to say what to avoid, not what to correct. Quoting it whole
        # would argue for copying it.
        f"For reference, the attempt being replaced opened with:\n"
        f"```\n{previous[:1200]}\n```\n\n"
        "Output the corrected rewrite in the same format contract (keep the "
        "// Applied rules: / // Skipped rules: headers)."
    ).replace("{rule}", rule)


def w2_finding_repair_note(code: str, detail: str, previous: str) -> str:
    """One corrective turn for a W2 rejection that came with a diagnosis.

    A W2 verdict on its own is a measurement, and re-prompting on it asks the
    model to guess its way past a stopwatch. This note exists only for the
    case where a mechanical check ALSO found something the rewrite introduced,
    so there is a specific thing to undo.

    Measured: one crate's `precompute_bonus` walks a NUL-terminated string
    with `while *h.offset(i) != 0` — the loop IS the scan. The rewrite
    prefixed it with `CStr::from_ptr(h).to_bytes()`, traversing the string
    twice, and W2 read +7.818%. The contract already forbade this in prose;
    nothing had ever told the model it had done it.
    """
    if code == "introduced_scan":
        what = (
            "Building that view costs a full pass over the data before the "
            "loop even starts. If the original walked to a sentinel, ITS OWN "
            "LOOP was already the scan, so the rewrite now traverses twice.\n\n"
            "Keep the original's traversal. Apply the parts of your rewrite "
            "that do not need a length — a hoisted invariant, an inlined "
            "table lookup, a cheaper per-byte test — and leave the pointer "
            "walk alone. If a length is genuinely available without scanning "
            "(a struct field, a parameter), use that instead."
        )
    else:
        what = "Undo what the finding reports, and keep the rest."
    return (
        "\n\n## Previous attempt was REJECTED as slower, and here is why\n"
        f"check: `{code}`\n"
        f"finding: {detail[:2000]}\n\n"
        f"{what}\n\n"
        f"Your previous output was:\n{quote_previous(previous)}\n\n"
        "Output the corrected rewrite in the same format contract (keep the "
        "// Applied rules: / // Skipped rules: headers)."
    )


# The previous reply is quoted so the model corrects THAT rewrite rather than
# proposing a new and unmeasured one. Quoting it in part defeats the purpose:
# a reply cut mid-expression is not valid Rust, and "fix the compile error in
# this" then asks about code the model cannot see the end of. The cap here is
# a runaway guard, not a budget — the budget is `max_llm_tokens_per_fn`, which
# is checked before the turn is spent. Over the corpus the longest reply ever
# quoted was 11554 chars, against the 2000 that used to be hardcoded here.
_PREVIOUS_REPLY_CHARS = 24000


def quote_previous(previous: str) -> str:
    """The previous reply, whole, as a fenced block for a repair note."""
    if len(previous) <= _PREVIOUS_REPLY_CHARS:
        return f"```\n{previous}\n```"
    return (
        f"```\n{previous[:_PREVIOUS_REPLY_CHARS]}\n```\n"
        "**The quote above was cut off — it is not the whole reply and its "
        "last line is incomplete.** Reproduce the rest from the target code "
        "rather than closing it where the quote stops."
    )


_W1_CRASH_HEADLINE = (
    "## Previous attempt was REJECTED: the program CRASHED\n"
)
_W1_MISMATCH_HEADLINE = (
    "## Previous attempt was REJECTED: it changed the program's output\n"
)
_W1_TIMEOUT_HEADLINE = (
    "## Previous attempt was REJECTED: the program never finished\n"
)

# A crash is a memory-safety fault, not a disagreement about bytes. The two
# need different questions asked of them, and asking the wrong one wastes the
# whole turn — measured: on a rewrite that died with `exit -11`, the model was
# handed the byte-equivalence checklist below, worked through it, found
# nothing wrong (correctly — none of it applied), and returned the previous
# reply byte for byte. Twice, in two independent runs.
_W1_CRASH_CHECKS = (
    "The rewrite reads or writes memory the original never touched. Check in "
    "this order — the first question is usually the answer:\n"
    "1. **List every early return and guard in the ORIGINAL function, then "
    "find each one in your rewrite.** A null test, a zero-length return, a "
    "bounds test. c2rust code is full of them, they read like noise, and a "
    "rewrite that tidies the body drops one without noticing — a dropped "
    "null guard is exactly this crash. Say where each one now lives. If one "
    "is gone, that is the bug: put it back and stop.\n"
    "2. **Where each length came from** — a `from_raw_parts` length, a copy "
    "count, an offset. It must be a quantity the original already used to "
    "bound itself: its loop condition, a struct field, a parameter. A length "
    "inferred from what the buffer *ought* to hold is the next commonest "
    "cause.\n"
    "3. **A view built before something that reallocates.** If what follows "
    "can grow, `realloc`, or free that buffer, every slice taken beforehand "
    "dangles.\n"
    "4. **Only if none of those explain it:** a new pointer interrogation — "
    "`malloc_usable_size`, an alignment or capacity probe — asks of that "
    "pointer what the original never did. Before blaming it, trace where the "
    "pointer is assigned: if every assignment comes from the matching "
    "allocator, the probe is sound and the fault is elsewhere. Do not abandon "
    "the optimisation over a provenance worry you have not traced.\n"
)

# glibc aborts when it finds its own bookkeeping damaged, which means the
# write that did the damage already happened, somewhere earlier.
_W1_ABORT_EXTRA = (
    "\nThis is an **abort**, not a segfault: the allocator found its "
    "metadata corrupted. The offending write happened before this point and "
    "went past the end of a heap block, or freed something twice. Look for a "
    "destination length larger than what was actually allocated.\n"
)

_W1_MISMATCH_CHECKS = (
    "The rewrite is not equivalent to the code it replaced. Find the "
    "difference and fix it — do not abandon the optimisation and do not "
    "start over with a different one.\n\n"
    "Check these first, in this order:\n"
    "1. **Any constant you derived by hand.** Re-derive it over the whole "
    "input domain (256 values for a byte) and state the check. A "
    "multi-byte mask or multiplier written in hex is easy to get "
    "backwards.\n"
    "2. **Byte order.** `to_le_bytes` / `from_le_bytes` versus the `be` "
    "forms — the first byte out must be the first byte the original "
    "produced.\n"
    "3. **The tail.** A loop widened to N units at a time still owes the "
    "remaining `len % N` the original's exact behaviour.\n"
    "4. **Off-by-one at the boundaries** of every range you sliced.\n"
)

_W1_TIMEOUT_CHECKS = (
    "The rewrite does not terminate on an input the original finished. "
    "Check in this order:\n"
    "1. **A loop counter that stopped advancing.** Folding a cursor into an "
    "iterator can leave the original `i += 1` behind, or drop it.\n"
    "2. **A bound that now recomputes each pass** and grows as fast as the "
    "index does.\n"
    "3. **A `while` whose exit condition depended on a value the rewrite "
    "hoisted** out of the loop, so it never changes again.\n"
)


def classify_w1_failure(reason: str) -> str:
    """Which kind of W1 failure this is: crash / abort / timeout / mismatch.

    The detail comes from `Verifier.w1` and has four shapes — `exit N ≠ M`,
    `W1 timeout`, `sha256 … ≠ …`, and `missing <path>`. Over the corpus the
    exit-code form is the majority of all W1 failures and almost all of it is
    a signal death: 16 segfaults and 6 aborts against 19 byte mismatches.
    Telling them apart is what lets the repair turn ask the right question.
    """
    head = reason.split("\n", 1)[0]
    if "W1 timeout" in reason:
        return "timeout"
    if "sha256" in head:
        return "mismatch"
    marker = "exit "
    if marker in head:
        tail = head.split(marker, 1)[1].split()[0]
        try:
            code = int(tail)
        except ValueError:
            return "mismatch"
        # Negative: killed by signal N. 128+N and 101 (Rust panic) reach us
        # through a shell or a wrapper that already folded the signal in.
        if code == -6 or code == 134:
            return "abort"
        if code < 0 or code == 101 or code > 128:
            return "crash"
    return "mismatch"


def build_repair_note(stderr: str, previous: str) -> str:
    """One corrective turn for a region rewrite that did not compile.

    Region mode had a repair turn for every failure class that carries a
    definite signal — post-validation findings, W1, W2 findings, untested
    subsumption — except the one the compiler diagnoses precisely. A build
    failure ended the candidate on the spot, and `_should_decompose` does not
    catch it either: it only splits bundles rejected by W2.

    Measured, zopfli `EncodeTree`: the III④ rewrite converted a variable to a
    slice and left four `.offset()` calls behind it, `no method named offset
    found for reference &[u32]` four times over, at four named lines. The
    candidate died there. Region mode is where the hottest functions go —
    that run sent its five hottest through it — so this is the path where a
    dead candidate costs the most.
    """
    return (
        "\n\n## Previous attempt did not compile\n"
        f"cargo stderr:\n```\n{stderr[:2000]}\n```\n\n"
        "Fix exactly what the compiler objected to. A rewrite that converts a "
        "raw pointer to a slice has to convert every use of it in the same "
        "region — a leftover `.offset()` or `.add()` on the new binding is "
        "the usual cause. Do not abandon the optimisation and do not "
        "substitute a different one.\n\n"
        f"Your previous output was:\n{quote_previous(previous)}\n\n"
        "Output the corrected rewrite in the same format contract (keep the "
        "// Applied rules: / // Skipped rules: headers)."
    )


def w1_repair_note(reason: str, previous: str) -> str:
    """One corrective turn for a rewrite that failed the equivalence oracle.

    W1 replays golden records and compares stdout byte for byte, so a failure
    is a mechanical finding, like a compile error, and unlike a W2 verdict,
    which is a judgement about speed and not something the model can be told
    to fix. It is also the cheapest failure in a candidate's life — the first
    record disagrees in well under a second, against the minutes W2 spends.

    It was nevertheless the only failure class with a definite error signal
    that earned no retry at all. Measured: one crate's sub-word split rewrite
    failed W1 because the model wrote the 64-bit selection mask with its bytes
    reversed. The rest of the rewrite was the one that had measured -7.555% a
    run earlier, and it was discarded on the spot.

    The note is written to what actually failed. See `classify_w1_failure`
    for why one checklist could not cover all of it.
    """
    kind = classify_w1_failure(reason)
    if kind == "timeout":
        headline, checks = _W1_TIMEOUT_HEADLINE, _W1_TIMEOUT_CHECKS
        preamble = "The workload was killed after the time limit. It reported:"
    elif kind in ("crash", "abort"):
        headline = _W1_CRASH_HEADLINE
        checks = _W1_CRASH_CHECKS + (_W1_ABORT_EXTRA if kind == "abort" else "")
        preamble = (
            "The workload died on a signal instead of running to completion. "
            "It reported:"
        )
    else:
        headline, checks = _W1_MISMATCH_HEADLINE, _W1_MISMATCH_CHECKS
        preamble = (
            "W1 replays recorded runs and compares stdout byte for byte. It "
            "reported:"
        )
    return (
        "\n\n" + headline
        + f"{preamble}\n```\n{reason[:1200]}\n```\n\n"
        + checks
        + "\nDo not abandon the optimisation and do not start over with a "
        "different one — correct this rewrite.\n\n"
        f"Your previous output was:\n{quote_previous(previous)}\n\n"
        "Output the corrected rewrite in the same format contract (keep the "
        "// Applied rules: / // Skipped rules: headers)."
    )


def contract_repair_note(
    reason: str, rule_ids: "tuple[str, ...] | list[str]", previous: str
) -> str:
    """The corrective turn: what was wrong, and the reply to re-shape.

    The previous reply is quoted in full on purpose. Without it the model
    writes a NEW rewrite, which is a different (and unmeasured) proposal; the
    point of the repair is to recover the one it already produced.
    """
    if reason.startswith("unknown_declared_rule:"):
        detail = ("a rule id was declared that is not among the candidates "
                  f"{list(rule_ids)!r} — ids must be copied exactly")
    elif reason.startswith("rule_declared_in_both:"):
        detail = "a rule was listed as both applied and skipped"
    else:
        detail = _CONTRACT_REPAIRABLE[_reason_code(reason)]
    return (
        "## Your previous reply was not accepted — FORM only\n"
        "The rewrite below was never evaluated. Nothing is wrong with it as "
        "far as this check is concerned; the reply did not match the required "
        "shape, and the specific violation was:\n\n"
        f"    {reason} — {detail}\n\n"
        "Send **the same rewrite again**, changing only what the violation "
        "names. Do not re-derive it, do not drop rules you had applied, and "
        "do not add new ones. If a skip reason has to quote Rust that "
        "contains `]` or `;`, keep it inside backticks and keep the whole "
        "`// Skipped rules: [...]` list on a single line. If a list is "
        "empty, write it as `[]` — an empty list is NOT the word `none`, "
        "which reads as a rule id and voids the reply a second time.\n\n"
        f"Candidate rule ids, verbatim: {list(rule_ids)!r}\n\n"
        "### Your previous reply\n"
        f"{previous}"
    )


@dataclass(frozen=True)
class RegionAttempt:
    """One region turn, including the repair turn when there was one."""
    plan: "LLMRegionPlan"
    response: str
    tokens_in: int
    tokens_out: int
    repaired_from: str | None = None


@dataclass(frozen=True)
class LLMRegionPlan:
    proposal: ProposedChangeSet | None
    abstain_reason: str | None = None


def _abstain(reason: str) -> LLMRegionPlan:
    return LLMRegionPlan(None, reason)


def _sha256_canonical(value: object) -> str:
    encoded = json.dumps(
        value,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def _region_identity(region: RegionRef) -> dict[str, object]:
    return {
        "target_function": {
            "qualified_name": region.target_function.qualified_name,
            "symbol_kind": region.target_function.symbol_kind.value,
            "file_hint": region.target_function.file_hint,
            "declaration_hash": region.target_function.declaration_hash,
        },
        "relative_path": region.relative_path,
        "region_kind": region.region_kind.value,
        "parent_kind": region.parent_kind,
        "start_byte": region.start_byte,
        "end_byte": region.end_byte,
        "expected_region_hash": region.expected_region_hash,
        "anchor_hit_ids": list(region.anchor_hit_ids),
        "anchor_lines": list(region.anchor_lines),
    }


def _parse_metadata(code: str) -> tuple[list[str], list[tuple[str, str]], str] | str:
    lines = code.splitlines()
    if (
        len(lines) < 2
        or not _APPLIED_HEADER.fullmatch(lines[0])
        or not _SKIPPED_HEADER.fullmatch(lines[1])
    ):
        return "metadata_headers_missing_or_not_first"
    applied = parse_applied_rules(lines[0])
    skipped = parse_skipped_rules(lines[1])
    replacement = "\n".join(lines[2:])
    return applied, skipped, replacement


def plan_llm_region_change(
    *,
    response: str,
    region: RegionRef,
    rule_ids: tuple[str, ...],
    base_head: str,
    candidate_id: str,
    hits: tuple | list | None = None,
) -> LLMRegionPlan:
    """Convert one region response into a deterministic local ChangeSet."""

    try:
        if not isinstance(response, str):
            return _abstain("invalid_response")
        if not isinstance(region, RegionRef):
            return _abstain("invalid_region")
        if not isinstance(rule_ids, tuple) or not rule_ids:
            return _abstain("invalid_candidate_rules")
        if any(not isinstance(rule_id, str) or not rule_id for rule_id in rule_ids):
            return _abstain("invalid_candidate_rules")
        if len(set(rule_ids)) != len(rule_ids):
            return _abstain("duplicate_candidate_rules")
        # Per-hit, not per-rule: `III①` form E is a body-local rewrite that
        # shares its rule id with the cross-function forms. Judging by rule id
        # alone rejected the candidate the router had just accepted.
        if rules_region_local_for(rule_ids, hits):
            return _abstain("invalid_non_region_local_candidate_rule")
        if not isinstance(base_head, str) or not base_head:
            return _abstain("invalid_base_head")
        if not isinstance(candidate_id, str) or not candidate_id:
            return _abstain("invalid_candidate_id")

        code, parse_reason = parse_llm_response(response)
        if parse_reason is not None or code is None:
            return _abstain(parse_reason or "parse_fail")
        if _EXACT_FENCE_RESPONSE.fullmatch(response.strip()) is None:
            return _abstain("response_fence_contract")
        metadata = _parse_metadata(code)
        if isinstance(metadata, str):
            return _abstain(metadata)
        applied, skipped, replacement = metadata
        if any(
            _APPLIED_HEADER.fullmatch(line) or _SKIPPED_HEADER.fullmatch(line)
            for line in replacement.splitlines()
        ):
            return _abstain("replacement_metadata_header")

        applied_duplicates = len(applied) != len(set(applied))
        skipped_rules = [rule_id for rule_id, _ in skipped]
        skipped_duplicates = len(skipped_rules) != len(set(skipped_rules))
        if applied_duplicates or skipped_duplicates:
            return _abstain("duplicate_declared_rule")
        candidates = set(rule_ids)
        declared = set(applied) | set(skipped_rules)
        unknown = declared - candidates
        if unknown:
            return _abstain(f"unknown_declared_rule:{sorted(unknown)!r}")
        overlap = set(applied) & set(skipped_rules)
        if overlap:
            return _abstain(f"rule_declared_in_both:{sorted(overlap)!r}")
        if declared != candidates:
            return _abstain("candidate_rule_coverage_incomplete")
        if any(not reason.strip() for _, reason in skipped):
            return _abstain("skipped_rule_reason_empty")
        if not applied:
            return _abstain("no_applied_rules")
        applied_set = set(applied)
        applied = [rule_id for rule_id in rule_ids if rule_id in applied_set]

        shape = validate_region_replacement_source(region.region_kind, replacement)
        if not shape.ok:
            return _abstain(shape.as_reason())

        rule_id = ",".join(applied)
        region_identity = _region_identity(region)
        operation_id = "replace-region-" + _sha256_canonical(
            {
                "candidate_id": candidate_id,
                "region": region_identity,
                "applied_rules": applied,
                "replacement": replacement,
            }
        )[:16]
        changeset_id = "region-" + _sha256_canonical(
            {
                "base_head": base_head,
                "candidate_id": candidate_id,
                "operation_id": operation_id,
                "region": region_identity,
            }
        )[:16]
        operation = ReplaceSourceRegion(
            operation_id=operation_id,
            rule_id=rule_id,
            evidence_hit_ids=region.anchor_hit_ids,
            region=region,
            replacement_region_source=replacement,
        )
        trigger = TriggerContext(
            rule_id=rule_id,
            candidate_id=candidate_id,
            hot_function=region.target_function.qualified_name.rsplit("::", 1)[-1],
            hit_ids=region.anchor_hit_ids,
            evidence_artifacts=(),
        )
        return LLMRegionPlan(
            ProposedChangeSet(
                changeset_id=changeset_id,
                base_head=base_head,
                trigger=trigger,
                operations=(operation,),
                impact_scope=ImpactScope.LOCAL_FUNCTION,
            )
        )
    except (MemoryError, SystemExit):
        raise
    except Exception as exc:
        return _abstain(f"invalid_planner_input:{type(exc).__name__}")



def plan_region_with_contract_repair(
    *,
    chat,
    system_prompt: str,
    user_prompt: str,
    candidate_id_for,
    token_estimate,
    budget_remaining: int,
    region: RegionRef,
    rule_ids: tuple[str, ...],
    base_head: str,
    hits=None,
) -> RegionAttempt:
    """Ask once; on a FORM failure, ask exactly once more with the reason.

    `chat(system, user) -> str` and `token_estimate(text) -> int` are injected
    so this stays a pure function of the reply text — the caller owns the LLM
    client and the accounting. The repair is skipped when it would not fit in
    `budget_remaining`, so the per-function token cap still binds.
    """
    tokens_in = token_estimate(system_prompt) + token_estimate(user_prompt)
    response = chat(system_prompt, user_prompt)
    tokens_out = token_estimate(response)
    plan = plan_llm_region_change(
        response=response, region=region, rule_ids=rule_ids,
        base_head=base_head, candidate_id=candidate_id_for(response), hits=hits,
    )
    if plan.proposal is not None or not is_contract_repairable(
        plan.abstain_reason
    ):
        return RegionAttempt(plan, response, tokens_in, tokens_out)

    repair_prompt = user_prompt + "\n\n" + contract_repair_note(
        plan.abstain_reason, rule_ids, response
    )
    repair_in = token_estimate(system_prompt) + token_estimate(repair_prompt)
    if tokens_in + tokens_out + repair_in > budget_remaining:
        return RegionAttempt(plan, response, tokens_in, tokens_out)

    repaired = chat(system_prompt, repair_prompt)
    repair_out = token_estimate(repaired)
    repaired_plan = plan_llm_region_change(
        response=repaired, region=region, rule_ids=rule_ids,
        base_head=base_head, candidate_id=candidate_id_for(repaired), hits=hits,
    )
    return RegionAttempt(
        repaired_plan, repaired,
        tokens_in + repair_in, tokens_out + repair_out,
        repaired_from=plan.abstain_reason,
    )
