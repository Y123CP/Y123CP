"""Minimal OpenAI-compatible LLM client used by all stages.

Wraps `openai>=2.x` with retry + streaming, routed via Config/llm_config.py.

Usage:
    client = LLMClient(model_name="claude-sonnet-4-20250514")
    text = client.chat(system="...", user="...")

Optional prompt cache (`cache_path`): JSONL file storing
`{key, ok, output}` per call. On chat() we hash (model, system, user)
and short-circuit if a previous successful answer is on disk. The cache
is **per-project** by convention (Stage 2 places it under `<rust_project>/
llm_cache.jsonl`) so re-runs on the same project skip already-answered
calls. Temperature is 0.1 in our config — outputs are nearly deterministic
for the same prompt; the cache trades a small amount of variance for
large speed-ups on resume / fixpoint paths.
"""

from __future__ import annotations

import hashlib
import json
import logging
import time
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any, Optional

# openai is optional at IMPORT time so deterministic-rule paths
# (perf_opt C7/A1/D1) work in venvs without the SDK installed.
# Construction raises only when a chat() call is actually attempted
# without the SDK present.
try:
    from openai import (
        OpenAI,
        APIConnectionError, APITimeoutError, RateLimitError,
        AuthenticationError, PermissionDeniedError, BadRequestError,
    )
    _OPENAI_AVAILABLE = True
except ImportError:
    OpenAI = None  # type: ignore[assignment]
    _OPENAI_AVAILABLE = False
    class APIConnectionError(Exception): ...           # type: ignore[no-redef]
    class APITimeoutError(Exception): ...              # type: ignore[no-redef]
    class RateLimitError(Exception): ...               # type: ignore[no-redef]
    class AuthenticationError(Exception): ...         # type: ignore[no-redef]
    class PermissionDeniedError(Exception): ...       # type: ignore[no-redef]
    class BadRequestError(Exception): ...              # type: ignore[no-redef]

from Config.llm_config import get_llm_config

logger = logging.getLogger(__name__)


# Error types that are NOT worth retrying. These are caller-side problems
# (bad credentials, exhausted token, malformed request) and the API will
# keep returning the same code on retry. Catching them early saves the
# `1+2+4 = 7s` exponential-backoff wait per call — when token is fully
# exhausted (hundreds of stuck calls), this alone trims tens of minutes.
_NON_TRANSIENT_ERRORS = (AuthenticationError, PermissionDeniedError, BadRequestError)


class IncompleteResponseError(Exception):
    """The API ended the reply before the model was done.

    `finish_reason` is the API's own statement of why generation stopped.
    Only ``stop`` means "the model finished". Anything else — ``length``
    (hit the token ceiling), ``content_filter``, or a stream that closes
    without ever reporting one — means the text we hold is a fragment.

    Before this existed, `_stream` returned that fragment like any other
    reply: the retry loop below never saw a failure (it only catches
    exceptions), the fragment was cached, and downstream code read the
    missing ```rust fence as the model declining to rewrite. Measured on
    one crate: two consecutive attempts on the same function, both cut
    mid-expression at ~520 chars while completions 35x longer succeeded in
    the same batch, were recorded as `planner_abstained` — a transport
    failure filed as the model's technical judgement.

    Transient by construction: it is raised inside the retry loop's `try`,
    so a truncated reply now costs one retry instead of a wrong verdict.
    """


_COMPLETE_FINISH_REASONS = frozenset({"stop", "tool_calls", "function_call"})


@dataclass
class LLMConfig:
    model_name: str
    temperature: float = 0.1
    max_retries: int = 3
    timeout: int = 60     # Lower from 180 to avoid hour-long hangs on flaky API/network.
    stream: bool = True


class LLMClient:
    def __init__(self, model_name: str,
                 transcript_path: Path | None = None,
                 cache_path:      Path | None = None,
                 **kwargs):
        cfg = get_llm_config(model_name) if _OPENAI_AVAILABLE else None
        # When the openai SDK isn't installed we still allow LLMClient to
        # be constructed — deterministic rules (C7/A1/D1) never reach
        # chat() so they don't need a live client. chat() itself raises.
        self._client = (
            OpenAI(api_key=cfg["api_key"], base_url=cfg["base_url"])
            if _OPENAI_AVAILABLE else None
        )
        self.cfg = LLMConfig(model_name=model_name, **kwargs)
        # The API's own account of why the last generation stopped. Recorded
        # into the transcript so a truncated reply is diagnosable after the
        # fact instead of guessed at from the text's shape.
        self._last_finish_reason: str | None = None
        self.transcript_path = Path(transcript_path) if transcript_path else None
        if self.transcript_path:
            self.transcript_path.parent.mkdir(parents=True, exist_ok=True)
            self.transcript_path.touch(exist_ok=True)
            logger.info(f"[LLMClient] transcript → {self.transcript_path}")
        # Optional prompt-hash cache. Loaded on init so repeated runs that
        # ask the same question don't burn API calls.
        self.cache_path = Path(cache_path) if cache_path else None
        self._cache: dict[str, str] = self._load_cache()
        if self.cache_path:
            logger.info(f"[LLMClient] cache → {self.cache_path} ({len(self._cache)} entries)")
        base_url = cfg["base_url"] if cfg else "<openai SDK not installed>"
        logger.info(f"[LLMClient] model={model_name} base_url={base_url}")

    # ----- prompt cache ----------------------------------------------------

    def _cache_key(self, system: str, user: str) -> str:
        h = hashlib.sha256()
        h.update(self.cfg.model_name.encode())
        h.update(b"\x00")
        h.update(system.encode("utf-8", errors="replace"))
        h.update(b"\x00")
        h.update(user.encode("utf-8", errors="replace"))
        return h.hexdigest()

    def _load_cache(self) -> dict[str, str]:
        if not self.cache_path or not self.cache_path.exists():
            return {}
        out: dict[str, str] = {}
        for ln in self.cache_path.read_text(encoding="utf-8").splitlines():
            ln = ln.strip()
            if not ln:
                continue
            try:
                row = json.loads(ln)
            except json.JSONDecodeError:
                continue
            if row.get("ok") and "key" in row and "output" in row:
                out[row["key"]] = row["output"]
        return out

    def _cache_put(self, key: str, output: str) -> None:
        if not self.cache_path:
            return
        self._cache[key] = output
        self.cache_path.parent.mkdir(parents=True, exist_ok=True)
        with self.cache_path.open("a", encoding="utf-8") as f:
            f.write(json.dumps({"key": key, "ok": True, "output": output}) + "\n")

    def chat(self, system: str, user: str, meta: dict[str, Any] | None = None) -> str:
        """Returns LLM completion text. Returns empty string on persistent
        failures so callers (rewriters) can fall through to "no usable fix"
        without crashing the whole Stage 2 pipeline.

        `meta` is an optional caller-supplied tag (e.g. {"cluster_id": 7,
        "function": "foo"}) that gets recorded into the transcript JSONL
        for later forensics.

        **Transcript invariant**: every successful chat() call MUST emit
        exactly one transcript row before returning. The `try/finally`
        block enforces this even when intermediate paths (`_record` raised,
        an unforeseen exception, etc.) would otherwise leave the call
        silent — that was Stage 2's blind spot in the previous run.
        """
        if not _OPENAI_AVAILABLE or self._client is None:
            # Return "" rather than raising — matches the docstring contract
            # ("Returns empty string on persistent failures so callers […]
            # can fall through to 'no usable fix' without crashing"). This
            # lets perf_opt.orchestrator's degrade path skip LLM-backed
            # rules cleanly when the SDK isn't installed and continue with
            # deterministic rules (C7/A1/D1).
            if not getattr(self, "_warned_missing_sdk", False):
                logger.warning(
                    "[LLMClient] chat() requested but openai SDK is not "
                    "installed; returning empty completion. LLM-backed "
                    "rules will degrade; deterministic rules continue."
                )
                self._warned_missing_sdk = True
            return ""
        # Cache hit: short-circuit the API call entirely. Still record a
        # transcript row (with ATTEMPTS=0) for forensic continuity.
        cache_key = self._cache_key(system, user) if self.cache_path else ""
        if cache_key and cache_key in self._cache:
            cached = self._cache[cache_key]
            self._record(system, user, cached, ok=True, attempts=0,
                         error="cache hit", meta=meta)
            return cached

        messages = [{"role": "system", "content": system},
                    {"role": "user",   "content": user}]
        last_err: Optional[Exception] = None
        attempts_used = 0
        output = ""
        recorded = False
        try:
            for attempt in range(self.cfg.max_retries):
                attempts_used = attempt + 1
                try:
                    output = self._stream(messages) if self.cfg.stream else self._oneshot(messages)
                    self._record(system, user, output, ok=True, attempts=attempts_used,
                                 error=None, meta=meta)
                    recorded = True
                    if cache_key:
                        self._cache_put(cache_key, output)
                    return output
                except _NON_TRANSIENT_ERRORS as e:
                    # 401 / 403 / 400: caller-side; retrying is pointless.
                    # Token exhaustion (403 "Token is exhausted") is the
                    # canonical case — without this fast-fail we'd waste
                    # ~7s of backoff per call.
                    last_err = e
                    logger.error(f"[LLMClient] non-transient {type(e).__name__}: {e}; "
                                 f"will not retry")
                    break
                except Exception as e:
                    # Cast wide: APIConnectionError, APITimeoutError, RateLimitError,
                    # httpx.RemoteProtocolError, json decode errors, etc. all map
                    # to "transient — retry then fall through".
                    last_err = e
                    wait = 2 ** attempt
                    logger.warning(f"[LLMClient] {type(e).__name__} on attempt "
                                   f"{attempt + 1}/{self.cfg.max_retries}: {e}; retry in {wait}s")
                    if attempt + 1 < self.cfg.max_retries:
                        time.sleep(wait)
            logger.error(f"[LLMClient] giving up after {attempts_used} attempt(s): {last_err}")
            self._record(system, user, "", ok=False, attempts=attempts_used,
                         error=f"{type(last_err).__name__}: {last_err}", meta=meta)
            recorded = True
            return ""  # caller treats empty as "no usable fix"
        finally:
            if not recorded:
                # Belt-and-suspenders: a successful chat() that didn't emit
                # a transcript row indicates _record itself raised. Capture
                # what we can so the next debug session can find the cause.
                try:
                    self._record(system, user, output,
                                 ok=False, attempts=attempts_used,
                                 error=f"unrecorded path; last_err={last_err!r}",
                                 meta=meta)
                except Exception as e:
                    logger.error(f"[LLMClient] failsafe record also raised: {e!r}")

    # Top-level + section dividers chosen to be visually distinct in any
    # text editor: `=` for "new entry", `-` for "section within entry".
    _ENTRY_BAR   = "=" * 80
    _SECTION_BAR = "-" * 80

    def _record(self, system: str, user: str, output: str, *,
                ok: bool, attempts: int, error: str | None,
                meta: dict[str, Any] | None) -> None:
        """Append one human-readable entry to the transcript log.

        Format (one entry):
            ============================================================
            TIMESTAMP : 2026-04-29 04:49:47
            MODEL     : gpt-4o-2024-11-20
            PHASE     : 3
            CLUSTER   : 148
            FUNCTION  : foo
            TARGET    : Option<&mut T>
            FILE      : src/blocksort.rs
            ATTEMPTS  : 1
            OK        : true
            (ERROR    : ...   ← only on failure)

            ----- SYSTEM PROMPT -----
            <system>
            ----- USER PROMPT -----
            <user>
            ----- OUTPUT -----
            <output>

        Designed to grep / scroll comfortably. JSON was the prior format
        (llm_transcript.jsonl) — switched away because long prompts/outputs
        in a single line make it unreadable without external tooling.
        """
        if not self.transcript_path:
            return
        try:
            meta = meta or {}
            header_fields = [
                ("TIMESTAMP", datetime.now().strftime("%Y-%m-%d %H:%M:%S")),
                ("MODEL",     self.cfg.model_name),
                ("PHASE",     str(meta.get("phase",       "-"))),
                ("CLUSTER",   str(meta.get("cluster_id",  "-"))),
                ("FUNCTION",  str(meta.get("function",    "-"))),
                ("TARGET",    str(meta.get("target",      "-"))),
                ("RULE",      str(meta.get("rule_id",     "-"))),
                ("FILE",      str(meta.get("file",        "-"))),
                ("ERRORS",    str(meta.get("error_count", "-"))),
                ("ATTEMPTS",  str(attempts)),
                ("OK",        "true" if ok else "false"),
                ("OUT_BYTES", str(len(output))),
                ("FINISH",    str(self._last_finish_reason or "-")),
            ]
            if error:
                header_fields.append(("ERROR", str(error)))
            # Right-pad keys to a fixed width so the colons line up.
            kw = max(len(k) for k, _ in header_fields)
            header = "\n".join(f"{k:<{kw}} : {v}" for k, v in header_fields)

            entry = (
                f"{self._ENTRY_BAR}\n"
                f"{header}\n"
                f"\n{self._SECTION_BAR}\n"
                f"----- SYSTEM PROMPT -----\n"
                f"{system.rstrip()}\n"
                f"\n----- USER PROMPT -----\n"
                f"{user.rstrip()}\n"
                f"\n----- OUTPUT -----\n"
                f"{output.rstrip() if output else '<empty>'}\n"
                f"{self._ENTRY_BAR}\n\n"
            )

            with self.transcript_path.open("a", encoding="utf-8") as f:
                f.write(entry)
                f.flush()
            logger.info(f"[LLMClient] recorded transcript: cluster={meta.get('cluster_id', '-')} "
                        f"fn={meta.get('function', '-')} ok={ok} len(output)={len(output)}")
        except Exception as e:
            logger.warning(f"[LLMClient] failed to write transcript "
                           f"(meta={meta!r}): {type(e).__name__}: {e}")

    def _accepts_temperature(self) -> bool:
        """Newer Anthropic / OpenAI models reject the `temperature` param
        with a 400 BadRequest (`temperature is deprecated for this model`).
        Probe by model name — claude-opus-4-7 / claude-sonnet-4-6+ refuse it.
        """
        name = self.cfg.model_name.lower()
        rejecting_prefixes = ("claude-opus-4-7", "claude-sonnet-4-6",
                              "claude-haiku-4-5")
        return not any(name.startswith(p) for p in rejecting_prefixes)

    def _stream(self, messages) -> str:
        chunks = []
        finish_reason = None
        kwargs = {"model": self.cfg.model_name, "messages": messages,
                  "stream": True, "timeout": self.cfg.timeout}
        if self._accepts_temperature():
            kwargs["temperature"] = self.cfg.temperature
        stream = self._client.chat.completions.create(**kwargs)
        try:
            for resp in stream:
                if not resp.choices:
                    continue
                # Carried out of the loop: the reason arrives on a late chunk
                # (usually the last one, whose delta holds no content).
                if resp.choices[0].finish_reason:
                    finish_reason = resp.choices[0].finish_reason
                # gpt-5.4 emits chunks whose `delta` (or `delta.content`) is
                # None (role-only / trailing chunks). The old `delta.content`
                # access raised AttributeError MID-STREAM, which left the httpx
                # stream unclosed → connection leak → pool exhaustion → the
                # next request blocked forever (deadlock). Guard + always close.
                delta = resp.choices[0].delta
                if delta is not None and delta.content:
                    chunks.append(delta.content)
        finally:
            try:
                stream.close()
            except Exception:      # noqa: BLE001 — best-effort connection release
                pass
        self._last_finish_reason = finish_reason
        if finish_reason not in _COMPLETE_FINISH_REASONS:
            raise IncompleteResponseError(
                f"stream ended with finish_reason={finish_reason!r} after "
                f"{sum(len(c) for c in chunks)} chars")
        return "".join(chunks)

    def _oneshot(self, messages) -> str:
        kwargs = {"model": self.cfg.model_name, "messages": messages,
                  "stream": False, "timeout": self.cfg.timeout}
        if self._accepts_temperature():
            kwargs["temperature"] = self.cfg.temperature
        resp = self._client.chat.completions.create(**kwargs)
        if not resp.choices:
            raise IncompleteResponseError("response carried no choices")
        choice = resp.choices[0]
        self._last_finish_reason = choice.finish_reason
        msg = choice.message
        text = (msg.content or "") if msg is not None else ""
        if choice.finish_reason not in _COMPLETE_FINISH_REASONS:
            raise IncompleteResponseError(
                f"finish_reason={choice.finish_reason!r} after {len(text)} chars")
        return text
