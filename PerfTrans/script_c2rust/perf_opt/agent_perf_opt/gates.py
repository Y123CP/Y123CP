"""agent_perf_opt gates — W1 (correctness) + W2 (wall-clock) thin wrappers.

All heavy lifting is done by `perf_opt.verify.*` (already used by Stage A).
This module is the agent's **policy adapter**:

  * W1: use `verify.functional.golden_specs(assets, sample_per_op=cfg.w1_sample_per_op)`
        + `verify.w1.run_w1_suite(verifier, specs)`.  v1 default sample_per_op=1
        (fastest per-rewrite check; see cfg.w1_sample_per_op docstring).
  * W2: pass through to `verify.w2.w2_gate(pre_bin, post_bin, ..., repeats)`.
        v1 scope = hf.hottest_op only (impl_plan §8.9). Caches per-op iters
        and pre_measurement so we autotune / measure pre once, not per rewrite.

The gates DO NOT own building. Agent.py must have already built pre_bin
(once at session start) + post_bin (each rewrite) and pass the paths.
"""

from __future__ import annotations

import json
import math
import time
import hashlib
import logging
import shutil
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any, Optional

from perf_opt.verify.functional import golden_specs
from perf_opt.verify.w1 import run_w1_suite
from perf_opt.verify.w2 import W2Verdict

from perf_opt.agent_perf_opt.measurement import (
    MeasurementBackend,
    PairedComparison,
    PerfStatBackend,
    compare_paired,
)
from perf_opt.agent_perf_opt.measurement import (
    _PLACEMENT_INSNS_DROP_PCT,
    _PLACEMENT_MIN_AGG_GAIN_PCT,
    did_measurably_less_work,
)

# Instruction-count noise between two builds of the same source is a few
# hundredths of a percent; 0.10% is "did more work", not jitter.
_PLACEMENT_INSNS_CREEP_PCT = 0.10


def _op_insns(result) -> dict[str, float]:
    """op -> instruction delta %, for the ops that carry one."""
    out: dict[str, float] = {}
    for op, cmp_op in (getattr(result, "per_op", None) or {}).items():
        d = getattr(cmp_op, "insns_delta_pct", None)
        if isinstance(d, (int, float)):
            out[op] = float(d)
    return out

logger = logging.getLogger(__name__)


def _sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


class BaselineTamperedError(RuntimeError):
    """A measurement binary changed on disk after it was snapshotted.

    Every W2 verdict is a comparison against one of these snapshots. They sit
    on disk, writable, for the whole run — hours, dozens of gates — so a stray
    process rewriting one silently invalidates every number that follows and
    leaves no trace in the logs. Raised instead of logged: a run that keeps
    going after this produces data nobody can tell apart from good data.
    """


@dataclass
class W1Result:
    passed: bool
    reason: str = ""              # "" on pass; "<spec.name>: <gate detail>" on fail
    failed_spec: Optional[str] = None    # spec.name of the first failure


def w1_gate(verifier: Any, assets: Any, *, sample_per_op: int = 1) -> W1Result:
    """Sample `sample_per_op` golden entries per op → run through verifier.w1
    (from `verify.cargo::Verifier`) → return W1Result. First failure wins.

    `verifier` must be `Verifier(project_dir=<crate>, binary_dir=<harness_dir>)`
    already bound; caller (agent.py) built the harness binary before calling.
    """
    specs = golden_specs(assets, sample_per_op=sample_per_op)
    # W1 is the longest silent stretch in a candidate's life — on one crate it
    # replays 1500 golden records and, together with the build, accounted for
    # ~25 of every 70 minutes with not one line of output. A stall there was
    # indistinguishable from a stall in the build, or from progress.
    logger.info("[w1] replaying %d golden record(s) ...", len(specs))
    started = time.monotonic()
    ok, detail = run_w1_suite(verifier, specs)
    elapsed = time.monotonic() - started
    if ok:
        logger.info("[w1] %d/%d pass in %.1fs", len(specs), len(specs), elapsed)
        return W1Result(passed=True)
    logger.info("[w1] FAILED in %.1fs: %s", elapsed, detail[:200])
    # detail = "<spec_name>: <gate_detail>"
    spec_name = detail.split(":", 1)[0] if ":" in detail else None
    return W1Result(passed=False, reason=detail, failed_spec=spec_name)



_LIBRARY_SHARE_FLOOR = 0.33
# Where 0.33 comes from — not a knob turned until a project passed.
#
# An op whose library share is `s` translates a library slowdown of `x` into a
# wall-clock move of about `s·x`. Read backwards: an op that moved by the
# catastrophic threshold (5%) implies the library moved by 5/s. At s=0.125
# that is a 40% library regression — no local rewrite does that, so what moved
# was the other 87.5%: the harness, and the allocator behind it. At s=0.50 the
# same 5% implies 10%, which a rewrite plausibly can do. Requiring the implied
# figure to stay under ~15% before an op may veto gives s ≥ 5/15 ≈ 0.33.
#
# The measured shares agree and leave a wide gap around it: the ops that must
# lose their veto sit at 12.5% (lil) and 16.9 / 17.2% (fzy) — 83-87% harness —
# while the lowest that must keep one is zopfli's cache_roundtrip at 49.8%,
# the very op this whole mechanism was built for. Nothing lands in 17-50%.


def _measured_library_share(opt_dir: Path | None) -> dict[str, float]:
    """{op -> fraction of this op's time spent in the crate under test}.

    Read from `hotspots.json`, which `locate` writes — NOT from a symbol-level
    profile, and not from the `crate_share` recorded once at harness-generation
    time against the untouched translation.

    The distinction is the whole point. `locate` profiles with
    `perf record --call-graph dwarf` and folds each sample with
    `perf script --inline`, so a library function the linker absorbed into the
    harness's own op function is still charged to the library. A plain
    `perf report --sort symbol` cannot do that: LTO leaves the absorbed
    function with no symbol at all, and every one of its samples reads as
    harness time.

    The gap is not small and not rare. Profiled both ways on the same binary
    and the same input, one operation came out 0.0% library by symbol and
    49.1% by inline-expanded call graph — 64% of its samples sat in a single
    harness symbol that the library had been inlined into. Taking the symbol
    number at face value would strip that operation of its veto on the grounds
    that the crate does not run in it, while half its time is exactly that.
    (The same artefact, measured earlier on a different crate: an operation
    whose timed loop contains nothing but library calls profiled at 99.1%
    harness / 0% library — see test_inline_attribution.)

    So the floor below which an op loses its single-op veto must be read from
    the inline-aware profile. Ops that are genuinely harness-bound still fall
    below it; ops that merely *look* harness-bound to the symbol table do not.

    Missing or unreadable → empty dict, and every op keeps full standing.
    """
    if opt_dir is None:
        return {}
    try:
        data = json.loads((Path(opt_dir) / "hotspots.json").read_text("utf-8"))
    except (OSError, ValueError):
        return {}
    functions = data.get("hot_functions")
    if not isinstance(functions, list):
        return {}
    # `dropped_wrappers` too. They are crate functions — they only got dropped
    # from the REWRITE list because they delegate straight to libc, which says
    # nothing about whose time they are. Leaving them out is not conservative,
    # it silently guts the number: on lil, `expr_embedded_builtins` reads 0.2%
    # from `hot_functions` alone and 12.5% with the wrappers, because its two
    # hottest entries (`lil_clone_value`, `alloc_value_len`) are both on the
    # dropped list. `locate` independently measures that op at 13%.
    out: dict[str, float] = {}
    for group in (functions, data.get("dropped_wrappers") or []):
        if not isinstance(group, list):
            continue
        for entry in group:
            if not isinstance(entry, dict):
                continue
            per_op = entry.get("per_op")
            if not isinstance(per_op, dict):
                continue
            for op, pct in per_op.items():
                if isinstance(op, str) and isinstance(pct, (int, float)) \
                        and not isinstance(pct, bool):
                    out[op] = out.get(op, 0.0) + float(pct) / 100.0
    # Still a LOWER bound: only fns at or above the hotness threshold are
    # listed at all. Note which way that cuts — a share read too LOW falls
    # below the floor and DISARMS the op. Under-reading does not preserve a
    # veto, it removes one, so the reading has to be as complete as the file
    # allows rather than merely cautious.
    return {op: min(share, 1.0) for op, share in out.items()}


class PerformanceSession:
    """Own pristine/current-parent binary snapshots for one optimization run."""

    def __init__(
        self,
        pristine_bin: Path | None = None,
        assets: Any = None,
        *,
        opt_dir: Path | None = None,
        backend: MeasurementBackend | None = None,
        checkpoints: tuple[int, ...] = (10, 20, 40),
        per_op_upper_limit_pct: float = 1.0,
        bystander_op_upper_limit_pct: float = 3.0,
        primary_op_share_pct: float = 10.0,
        regress_tolerance_pct: float = 0.3,
        catastrophic_regress_pct: float = 5.0,
        cumulative_op_share_pct: float = 50.0,
        cumulative_op_limit_pct: float = 2.0,
        cumulative_op_limit_strict_pct: float = 1.0,
        min_total_gain_pct: float = 0.1,
        pin_cpu: int | None = None,
        target_wall: float = 2.0,
        pre_bin: Path | None = None,
        repeats: int | None = None,
        all_ops: tuple[str, ...] | list[str] = (),
    ) -> None:
        del repeats  # legacy constructor argument; paired checkpoints supersede it
        source = pristine_bin if pristine_bin is not None else pre_bin
        if source is None:
            raise ValueError("pristine_bin is required")
        source = Path(source).resolve()
        if not source.is_file():
            raise FileNotFoundError(source)
        self.assets = assets
        self.opt_dir = Path(opt_dir or source.parent).resolve()
        bins_dir = self.opt_dir / ".agent_bins"
        bins_dir.mkdir(parents=True, exist_ok=True)
        self._bin_digests: dict[Path, str] = {}
        self.pristine_bin = bins_dir / "pristine-harness"
        self.parent_bin = bins_dir / "parent-harness"
        self._snapshot_bin(source, self.pristine_bin)
        self._snapshot_bin(source, self.parent_bin)
        self.backend = backend or PerfStatBackend(pin_cpu=pin_cpu)
        self.checkpoints = checkpoints
        self.per_op_upper_limit_pct = per_op_upper_limit_pct
        self.library_share = _measured_library_share(opt_dir)
        self.bystander_op_upper_limit_pct = bystander_op_upper_limit_pct
        self.primary_op_share_pct = primary_op_share_pct
        self.regress_tolerance_pct = regress_tolerance_pct
        self.cumulative_op_share_pct = cumulative_op_share_pct
        self.cumulative_op_limit_pct = cumulative_op_limit_pct
        self.cumulative_op_limit_strict_pct = cumulative_op_limit_strict_pct
        self.min_total_gain_pct = min_total_gain_pct
        self.catastrophic_regress_pct = catastrophic_regress_pct
        self.pin_cpu = pin_cpu
        self.target_wall = target_wall
        self.parent_generation = 0
        self.parent_commit: str | None = None
        # Where the crate stands against pristine as of the last commit, and
        # where the candidate under judgement would put it. The net gain gate
        # is the difference; `promote_candidate` is what makes a pending
        # figure the parent's, so a rejected candidate never moves the line.
        self._parent_total_pct: float | None = 0.0
        self._pending_total_pct: float | None = None
        # Per-op instruction delta against pristine, as of the last commit and
        # for the candidate under judgement — same lifecycle as the two above.
        # The cumulative gate's placement exemption reads the committed one:
        # past the first commit, "fewer instructions than base" is judged
        # against where the op stood when it was last accepted, so a drop that
        # one commit banked cannot be spent again to hide later work.
        self._committed_op_insns: dict[str, float] | None = None
        self._pending_op_insns: dict[str, float] | None = None
        # Every perf-measurable op, not just the ones a candidate touches. The
        # total gate needs this: a rewrite changes the whole binary's layout,
        # so ops it never mentions move too. Measured on optipng — one commit
        # touching only `src/optipng/bitset.rs` moved three untouched ops by
        # +1.37% / -1.73% / -0.10%, and the per-op limit is 1.0%. Judging such
        # a commit on its own ops alone reads layout drift as its verdict.
        self.all_ops: tuple[str, ...] = tuple(dict.fromkeys(all_ops))

    def _compare(
        self,
        base_binary: Path,
        candidate_binary: Path,
        ops: tuple[str, ...] | list[str],
        *,
        weights: dict[str, float] | None = None,
        high_risk_ops: tuple[str, ...] | list[str] = (),
        aggregate_only: bool = False,
        per_op_limits: dict[str, float] | None = None,
    ) -> PairedComparison:
        self._assert_intact(base_binary)
        return compare_paired(
            self.backend,
            base_binary,
            Path(candidate_binary).resolve(),
            ops=ops,
            assets=self.assets,
            checkpoints=self.checkpoints,
            weights=weights,
            per_op_upper_limit_pct=self.per_op_upper_limit_pct,
            per_op_limits=per_op_limits,
            regress_tolerance_pct=self.regress_tolerance_pct,
            catastrophic_regress_pct=self.catastrophic_regress_pct,
            target_wall=self.target_wall,
            pin_cpu=self.pin_cpu,
            high_risk_ops=high_risk_ops,
            aggregate_only=aggregate_only,
        )

    def _snapshot_bin(self, src: Path, dst: Path) -> None:
        """Copy `src` → `dst`, freeze it read-only, and record its digest.

        The digest comes from reading `dst` back after the write, not from the
        bytes handed to `copy2`, so a truncated or intercepted copy is caught
        here instead of surfacing hours later as an unexplained delta. The mode
        is 0o555 rather than 0o444 because these are executed to be measured.
        """
        src, dst = Path(src), Path(dst)
        if dst.exists():
            dst.chmod(0o755)        # frozen at 0o555; unfreeze to overwrite
        shutil.copy2(src, dst)
        dst.chmod(0o755)
        written, source_digest = _sha256_file(dst), _sha256_file(src)
        if written != source_digest:
            raise BaselineTamperedError(
                f"snapshot {src} -> {dst} does not match its source "
                f"(wrote {written[:16]}, source {source_digest[:16]})")
        self._bin_digests[dst] = written
        dst.chmod(0o555)

    def _assert_intact(self, path: Path) -> None:
        """Stop the run if a snapshotted binary changed since it was taken."""
        path = Path(path)
        expected = self._bin_digests.get(path)
        if expected is None:
            return
        actual = _sha256_file(path)
        if actual != expected:
            raise BaselineTamperedError(
                f"{path} changed on disk after it was snapshotted "
                f"(expected {expected[:16]}, found {actual[:16]}); every W2 "
                "comparison made against it is invalid")

    def compare_candidate(
        self,
        candidate_binary: Path,
        ops: tuple[str, ...] | list[str],
        *,
        high_risk_ops: tuple[str, ...] | list[str] = (),
    ) -> PairedComparison:
        return self._compare(
            self.parent_bin,
            candidate_binary,
            ops,
            high_risk_ops=high_risk_ops,
        )

    def promote_candidate(self, candidate_binary: Path, commit_sha: str) -> None:
        """Advance parent only after the source ChangeSet commit succeeded."""
        self._snapshot_bin(Path(candidate_binary), self.parent_bin)
        self.parent_generation += 1
        self.parent_commit = commit_sha
        self._parent_total_pct = self._pending_total_pct
        self._pending_total_pct = None
        self._committed_op_insns = self._pending_op_insns
        self._pending_op_insns = None

    def measure_final(
        self, final_binary: Path, ops: tuple[str, ...] | list[str]
    ) -> PairedComparison:
        report = self._compare(self.pristine_bin, final_binary, ops)
        output = asdict(report)
        output["per_op"] = {
            op: asdict(comparison) for op, comparison in report.per_op.items()
        }
        output.update(self._library_view(report))
        self._write_final_measurement(output)
        return report

    def _library_view(self, report: "PairedComparison") -> dict[str, Any]:
        """Report the aggregate a second time over only the ops that run the library.

        An op whose library share is ~0 cannot register an optimization: its
        time is allocator, syscall and process setup, and whatever number it
        contributes is relink noise. It still belongs in the headline aggregate
        — that time is real time the workload spends — but reading the run's
        effect off a mean that includes it understates the effect by however
        many such ops the workload happens to have.

        Measured on optipng, where four of ten ops sit below 1% library share:
        the headline reads -0.413% and the same commits over the six ops that
        do run the library read -0.756%. On the other ten projects in the set
        no op falls below the floor and the two numbers are identical, so this
        is reported alongside the headline rather than replacing it — the
        verdict, and every historical number, stays on the same definition.
        """
        floor = 0.01                      # 1% library share
        shares = {op: self.library_share.get(op, 0.0) for op in report.per_op}
        deltas = {
            op: cmp_op.mean_delta_pct for op, cmp_op in report.per_op.items()
            if isinstance(getattr(cmp_op, "mean_delta_pct", None), (int, float))
        }
        live = [d for op, d in deltas.items() if shares.get(op, 0.0) >= floor]
        dead = sorted(op for op in deltas if shares.get(op, 0.0) < floor)
        return {
            "library_share_by_op": {op: round(v, 4) for op, v in shares.items()},
            "library_share_floor": floor,
            "ops_below_library_floor": dead,
            "aggregate_library_ops_pct": (
                sum(live) / len(live) if live else None),
        }

    def _write_final_measurement(self, output: dict[str, Any]) -> None:
        (self.opt_dir / "final_measurement.json").write_text(
            json.dumps(output, indent=2, sort_keys=True), encoding="utf-8"
        )

    def maybe_skip_final(
        self, committed_attempts: int
    ) -> dict[str, Any] | None:
        if committed_attempts != 0 or self.parent_generation != 0:
            return None
        pristine_hash = _sha256_file(self.pristine_bin)
        parent_hash = _sha256_file(self.parent_bin)
        if pristine_hash != parent_hash:
            logger.warning(
                "[w2] final skip invariant mismatch: zero commits but "
                "pristine/parent hashes differ"
            )
            return None
        output = {
            "status": "skipped",
            "reason": "no_accepted_commits",
            "committed_attempts": 0,
            "binaries_identical": True,
        }
        self._write_final_measurement(output)
        logger.info("[w2] final measurement skipped: no accepted commits")
        return output

    def gate(self, post_bin: Path, op: str) -> W2Verdict:
        verdict, _ = self.gate_ops(post_bin, [op])
        return verdict


    def _cumulative_op_regress(
        self, total: "PairedComparison"
    ) -> Optional[tuple[str, float, float]]:
        """The op-level question neither existing gate asks, run against base.

        The step gate resets at every commit, so an op can give up a little
        under each of six candidates and trip nothing; the aggregate hides the
        same six because other ops gained more. Measured on optipng: an op at
        75.6% library share reached +3.34% against pristine, CI [3.23, 3.44],
        across six commits that each passed both gates, while the run as a
        whole reported -0.41%.

        Only ops the library genuinely dominates are checked. Below that share
        a relink moves an op by more than any ceiling worth setting — the
        reason `_op_limits` relaxes bystanders and the reason the total gate
        takes no per-op limits at all. On lil an op at 12.3% share read
        +22.55% while the aggregate was -7.6%; rejecting that would have been
        wrong, and this check must not.

        Returns (op, delta, ceiling) for the worst offender, or None.
        """
        floor = self.cumulative_op_share_pct / 100.0
        worst: Optional[tuple[str, float, float]] = None
        for op, cmp_op in (total.per_op or {}).items():
            share = self.library_share.get(op)
            if share is None or share < floor:
                continue
            delta = getattr(cmp_op, "mean_delta_pct", None)
            if not isinstance(delta, (int, float)):
                continue
            # A tighter ceiling where the library is nearly the whole op: the
            # share of the movement that layout can account for shrinks with it.
            ceiling = (self.cumulative_op_limit_strict_pct if share >= 0.90
                       else self.cumulative_op_limit_pct)
            if delta <= ceiling:
                continue
            # The placement exemption — but the STRICT half only, and that
            # asymmetry against the step gate is deliberate. This check reads
            # mean wall against pristine, so it was the next veto in line
            # after the step gate: lz4's C11 split left streaming_decode
            # (90% library, strict 1.0% ceiling) at +2.5% wall on -1.22%
            # instructions against pristine, with the crate at -1.94%.
            # Creep this gate exists for is work — an op retiring MORE
            # instructions each commit. An op retiring fewer is not creeping.
            #
            # The step gate also exempts an op whose instructions are FLAT
            # (see `is_placement_noise`), because against one parent a flat
            # count means the rewrite did not touch that op. Here it would be
            # the opposite: creep is precisely a run of flat-or-slightly-up
            # steps, every one of them small enough to clear the step gate,
            # and this is the only gate positioned to add them up. So flat
            # gets no pass here — `did_measurably_less_work`, not
            # `is_placement_noise`.
            insns = getattr(cmp_op, "insns_delta_pct", None)
            agg_hi = getattr(total, "aggregate_ci_high_pct", None)
            if (isinstance(insns, (int, float))
                    and did_measurably_less_work(insns)
                    and isinstance(agg_hi, (int, float))
                    and agg_hi < _PLACEMENT_MIN_AGG_GAIN_PCT
                    and self._no_new_work_since_commit(op, insns)):
                logger.info(
                    "[w2] 累计门豁免:%s 相对 base wall %+.2f%% 越限 %.2f%%,"
                    "但 insns %+.2f%%(判为代码摆放);聚合 ci_high %+.3f%%",
                    op, delta, ceiling, insns, agg_hi)
                continue
            if worst is None or delta > worst[1]:
                worst = (op, float(delta), ceiling)
        return worst

    def _no_new_work_since_commit(self, op: str, insns: float) -> bool:
        """The op retired no more instructions than when it was last accepted.

        Measured on lz4: the C11 split committed with streaming_decode at
        -1.22% instructions against pristine, and every later total gate read
        that. Against pristine, a later candidate could add ~0.95% of real
        work to the op and still pass as "fewer instructions than base".
        Before the first commit there is no accepted state; the check falls
        back to pristine, which is what `insns` is measured against.
        """
        committed = (self._committed_op_insns or {}).get(op)
        return committed is None or insns <= committed + _PLACEMENT_INSNS_CREEP_PCT

    def _net_contribution(
        self, total_pct: Optional[float]
    ) -> Optional[tuple[float, float]]:
        """Whether the candidate earns its place, asked of the whole crate.

        Both gates above answer "is this candidate harmless" — the step gate
        against the parent over the ops the candidate's function was profiled
        into, the total gate against pristine over every op. Neither asks the
        question the run is actually judged on: does keeping this move the
        crate forward.

        The two are not the same, because committing anything relinks the
        image and moves ops the edit never touched. Measured across the run
        history on disk: http-parser's `http_message_needs_eof` passed the step
        gate at -0.168% and moved the crate BACKWARD by 1.615%, taking
        `parse_url_and_meta` from -15.427% to -5.716% at an instruction delta
        of +0.036% — pure placement. lil committed three candidates whose net
        contributions were +0.151%, -0.031% and +0.054%, and the -0.031% one
        is what pushed `expr_embedded_builtins` from -0.083% to +3.633%.

        The step gate cannot see this. Its op list comes from `hf.per_op`,
        which `locate` fills from sampled self time with no floor, so a single
        perf sample — 0.05% — decides whether an op is watched at all. Across
        two runs of lil five functions lost an op that way and libqrencode two.
        A gate whose coverage is decided by sampling noise cannot be the last
        word; this one does not depend on coverage at all.

        So a candidate has to pay for the relink it causes. Anything whose net
        contribution sits inside measurement noise is not free — it is a fresh
        layout draw with no compensation. Replaying every committed changeset
        from all 24 runs on disk: at a 0.1% floor the set comes out 8.19 points
        ahead, 7 runs better and 2 worse, the largest single gain being
        optipng's 3.57 points and the largest single loss +0.33 on a run where
        nothing was rejected at all — the noise between the last total gate and
        the final re-measurement, not this gate's doing.

        The floor sits near zero on purpose. The question is whether a
        candidate earns anything, not whether it earns a lot: raising it to
        0.3% throws away real 0.16% and 0.25% gains and costs the set 1.4
        points.

        Returns (net, required) when the contribution falls short, else None.
        A missing baseline — the first commit, whose total gate is skipped
        because it would duplicate the step gate — waives the check rather
        than guessing.
        """
        if self._parent_total_pct is None or total_pct is None:
            return None
        net = total_pct - self._parent_total_pct
        required = -abs(self.min_total_gain_pct)
        return (net, required) if net > required else None

    def _op_limits(
        self, ops: list[str], op_weights: Optional[dict[str, float]]
    ) -> Optional[dict[str, float]]:
        """Looser ceilings for the ops this candidate is a bystander in.

        `op_weights` is the rewritten function's share of each op's self time.
        Where that share is small the function cannot plausibly move the op by
        the amounts a relink does on its own — measured up to 4.12% on an op
        whose source no commit had touched — so holding it to the tight
        ceiling rejects on layout, not on the edit.

        Returns None when there is nothing to relax, so the default path is
        byte-for-byte what it was.
        """
        if not op_weights:
            return None
        relaxed = {
            op: self.bystander_op_upper_limit_pct
            for op in ops
            if op_weights.get(op, 0.0) < self.primary_op_share_pct
        }
        # An op that barely runs the library cannot speak for it: it keeps its
        # place in the aggregate (its time is real time) but loses the single-op
        # veto, because what moved is allocator behaviour and code layout.
        #
        # The share MUST come from the inline-expanded profile — see
        # `_measured_library_share`. An earlier version of this relaxation read
        # a symbol-level number instead and disarmed an op measuring 0.0%
        # library by symbol and 49.1% by call graph: LTO had inlined the
        # library into the harness's own op function, leaving it no symbol.
        # Reading the wrong profile does not make this rule conservative, it
        # inverts it.
        # `inf`, not the catastrophic ceiling. "Loses the single-op veto" has
        # to mean it, and the catastrophic value does not: the collapse check
        # reads that SAME number, so handing it out here left the op vetoing at
        # exactly the threshold it was supposed to be exempt from. The
        # aggregate still holds these ops to account — their time is real time
        # — so disarming the single-op veto does not let a genuine collapse
        # through here: an op that is a quarter of the workload cannot fall
        # apart without moving the aggregate past its own tolerance.
        #
        # These limits go to the STEP gate only, and that is deliberate — see
        # `gate_ops`. An earlier note here credited this relaxation with
        # unblocking lil's -7.48% rewrite. It could not have: that rejection
        # came from the TOTAL gate, which is not handed these limits, and the
        # run that later reached -13.13% did so on a different rewrite. The
        # reasoning below stands on its own; the lil number was not its.
        #
        # An op ABSENT from a non-empty map is not an unknown. `locate`
        # profiles every op in the list and writes a `per_op` entry for each
        # hot function it charges; an op that appears nowhere in that file
        # carried no hot function at all, which is the strongest form of the
        # condition this rule tests. Defaulting it to 1.0 gave the ops with
        # the least standing the most, exactly inverted. An EMPTY map still
        # means "could not measure" — hotspots.json missing or unreadable —
        # and there every op keeps full standing, as the docstring of
        # `_measured_library_share` promises.
        unknown = 1.0 if not self.library_share else 0.0
        for op in ops:
            if self.library_share.get(op, unknown) < _LIBRARY_SHARE_FLOOR:
                relaxed[op] = math.inf
        return relaxed or None

    def gate_ops(
        self,
        post_bin: Path,
        ops: list[str],
        op_weights: Optional[dict[str, float]] = None,
        high_risk_ops: Optional[list[str]] = None,
    ) -> tuple[W2Verdict, str]:
        ""               
                                                     
                                                         
                                                           
                              
           
        hr = tuple(high_risk_ops or ())
        # A verdict only becomes the parent's through `promote_candidate`;
        # clearing here keeps a rejected candidate's figure from being read as
        # the next one's baseline.
        self._pending_total_pct = None
        self._pending_op_insns = None

        def _rows(result: PairedComparison) -> list[dict]:
            """One serializable row per op, from the comparison just made."""
            def _num(item, name):
                value = getattr(item, name, None)
                return round(value, 4) if isinstance(value, (int, float)) else None

            return [
                {
                    "op": op,
                    "mean_delta_pct": _num(item, "mean_delta_pct"),
                    "ci_low_pct": _num(item, "ci_low_pct"),
                    "ci_high_pct": _num(item, "ci_high_pct"),
                    # Recorded, never judged. Wall time alone cannot say
                    # whether an op got slower because the candidate does more
                    # work or because relinking moved it: measured on this
                    # gate's own binaries, optipng's `png_palette_roundtrip`
                    # read +3.16% wall at -0.99% instructions and libxml2's
                    # `buf_strings_chars` +6.04% wall at -0.06%, both pure
                    # placement, while a genuine -34.8% win carried -26.2%
                    # instructions with it. Reading a rejection without this
                    # number means re-measuring to find out which kind it was.
                    "insns_delta_pct": _num(item, "insns_delta_pct"),
                    "samples": getattr(item, "sample_count", 0),
                    "state": getattr(item, "state", None),
                    "stop_reason": getattr(item, "stop_reason", None),
                }
                for op, item in sorted((result.per_op or {}).items())
            ]

        def _mk(result: PairedComparison, *, reason: Optional[str] = None,
                gate: str = "step",
                also: tuple[tuple[PairedComparison, str], ...] = ()):
            """Build the verdict, carrying EVERY per-op number that was paid for.

            The aggregate alone cannot answer the two questions that come up
            every time a candidate is judged: which op moved, and by how much.
            Without them a rejection reads `per_op_regress, triggering_op=X`
            with no magnitude, so "X collapsed" and "X drifted 5.1% on code
            layout" are indistinguishable — and the only way to tell them
            apart afterwards is to re-measure, which costs as much as the run.
            Attribution of a committed gain has the same problem in reverse: a
            bundle commits at -9.37% aggregate and nothing on disk says how
            much of that was any one rule.

            `also` carries the gate that ran but did not produce the verdict,
            so a candidate that passes BOTH gates still records both sets of
            numbers rather than only the step gate's.
            """
            measurable = list(result.per_op.items())
            worst = (
                max(measurable, key=lambda kv: kv[1].mean_delta_pct)[0]
                if measurable else ""
            )
            def _block(cmp_result: PairedComparison, name: str) -> dict:
                """Descriptive fields are read with a default on purpose.

                This is bookkeeping running inside the decision path. Every
                gate verdict passes through here, so an AttributeError while
                *recording* a measurement would reject the candidate that was
                just measured — the observation killing the thing observed.
                The fields the verdict itself is built from stay direct; only
                the ones that merely describe it are forgiving.
                """
                return {
                    "gate": name,
                    "reference": "parent" if name == "step" else "pristine",
                    "aggregate_pct": cmp_result.aggregate_mean_pct,
                    "aggregate_ci_low_pct": getattr(
                        cmp_result, "aggregate_ci_low_pct", None),
                    "aggregate_ci_high_pct": getattr(
                        cmp_result, "aggregate_ci_high_pct", None),
                    "checkpoint": getattr(cmp_result, "checkpoint", 0),
                    "accepted": cmp_result.accepted,
                    "reason": cmp_result.reason,
                    "unmeasurable_ops": list(
                        getattr(cmp_result, "unmeasurable_ops", ()) or ()),
                    "ops": _rows(cmp_result),
                }

            measurements = [_block(result, gate)]
            for other, other_gate in also:
                measurements.append(_block(other, other_gate))
            return W2Verdict(
                ok=(reason is None and result.accepted),
                reason=reason or result.reason,
                delta_pct=result.aggregate_mean_pct,
                effective_tol_pct=self.per_op_upper_limit_pct,
                measured_cv=0.0,
                detail={"measurements": measurements},
            ), worst

                                           
        step = self._compare(
            self.parent_bin, post_bin, ops,
            weights=op_weights, high_risk_ops=hr,
            per_op_limits=self._op_limits(ops, op_weights),
        )
        if not step.accepted:
            return _mk(step, gate="step")

                                            
        #
                                                
                                               
                                                               
                                                         
                                          
                                 
        #
                                                       
                                                  
                                                 
        total_ops = self.all_ops or ops
        if self.parent_generation > 0 or tuple(total_ops) != tuple(ops):
                                                       
                                                                
            #
                                                         
                                                      
                                                   
                                               
                                                 
                                                                  
                                                       
                                                        
                                               
                                               
            total = self._compare(
                self.pristine_bin, post_bin, total_ops,
                weights=op_weights, high_risk_ops=hr, aggregate_only=True,
            )
            if not total.accepted:
                logger.info(
                    "[w2] total gate (vs base, %d op) 拒:整个 crate 相对原始 "
                    "base 累积退化 (%s, aggregate=%+.3f%%)",
                    len(total_ops), total.reason, total.aggregate_mean_pct or 0.0,
                )
                return _mk(total, reason="total_regress", gate="total",
                           also=((step, "step"),))
            # The aggregate passed. Ask separately whether any library-dominated
            # op has drifted away from base — six small concessions read the same
            # as one big one to every check above this line.
            creep = self._cumulative_op_regress(total)
            if creep is not None:
                op, delta, ceiling = creep
                logger.info(
                    "[w2] total gate 拒:%s 相对原始 base 累积退化 %+.3f%% "
                    "(上限 %.2f%%,库占比 %.1f%%);聚合 %+.3f%% 掩盖了它",
                    op, delta, ceiling, self.library_share.get(op, 0.0) * 100,
                    total.aggregate_mean_pct or 0.0,
                )
                return _mk(total, reason="cumulative_op_regress", gate="total",
                           also=((step, "step"),))
            # Both "is it harmless" questions passed. Now the one the run is
            # scored on: does keeping this move the crate forward at all.
            short = self._net_contribution(total.aggregate_mean_pct)
            if short is not None:
                net, required = short
                logger.info(
                    "[w2] total gate 拒:这一步对整个 crate 的净贡献 %+.3f%% "
                    "(要求 ≤ %+.3f%%);上一个 commit 在 %+.3f%%,这一版 %+.3f%% "
                    "—— 收益进不了噪音带,却要付一次重新布局的代价",
                    net, required, self._parent_total_pct or 0.0,
                    total.aggregate_mean_pct or 0.0,
                )
                return _mk(total, reason="insufficient_net_gain", gate="total",
                           also=((step, "step"),))
            self._pending_total_pct = total.aggregate_mean_pct
            self._pending_op_insns = _op_insns(total)
            return _mk(step, gate="step", also=((total, "total"),))

        # The total gate was skipped because parent is still pristine and the
        # step gate already covered every op — so its aggregate IS the crate's
        # position against pristine, and the next candidate gets a baseline.
        self._pending_total_pct = step.aggregate_mean_pct
        self._pending_op_insns = _op_insns(step)
        return _mk(step, gate="step")


# Temporary import compatibility while downstream callers migrate.
W2Session = PerformanceSession
