"""A reply the API cut short must not pass for a reply the model finished.

`finish_reason` is the API's own statement of why generation stopped. The
streaming path used to ignore it and hand the accumulated fragment back like
any other completion, so a transport failure reached the pipeline wearing the
shape of a model decision: no ```rust fence in the text, therefore
`parse_fail`, therefore `planner_abstained` — "the model judged this function
not worth rewriting". Measured on one crate: two consecutive attempts on the
same function, each cut mid-expression at ~520 chars, in a batch where
completions 35x longer came back whole.

These tests pin the three things that go wrong when the reason is ignored:
the fragment is returned, the fragment is cached, and the retry loop never
fires because nothing was raised.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from utils.llm_client import IncompleteResponseError, LLMClient


class _Delta:
    def __init__(self, content: str | None) -> None:
        self.content = content


class _Choice:
    def __init__(self, content: str | None, finish_reason: str | None) -> None:
        self.delta = _Delta(content)
        self.message = _Delta(content)
        self.finish_reason = finish_reason


class _Chunk:
    def __init__(self, content: str | None = None,
                 finish_reason: str | None = None) -> None:
        self.choices = [_Choice(content, finish_reason)]


class _Stream:
    """Minimal stand-in for the SDK's streaming iterator."""

    def __init__(self, chunks: list[_Chunk]) -> None:
        self._chunks = chunks
        self.closed = False

    def __iter__(self):
        return iter(self._chunks)

    def close(self) -> None:
        self.closed = True


def _client(tmp_path: Path, streams: list[list[_Chunk]]) -> LLMClient:
    """An LLMClient whose transport replays `streams`, one per attempt."""
    c = LLMClient.__new__(LLMClient)
    from utils.llm_client import LLMConfig
    c.cfg = LLMConfig(model_name="test-model", stream=True, max_retries=3)
    c.transcript_path = None
    c.cache_path = tmp_path / "cache.jsonl"
    c._cache = {}
    c._last_finish_reason = None

    pending = [_Stream(ch) for ch in streams]
    served: list[_Stream] = []

    class _Completions:
        def create(self, **kwargs):
            s = pending.pop(0)
            served.append(s)
            return s

    class _Chat:
        completions = _Completions()

    class _SDK:
        chat = _Chat()

    c._client = _SDK()
    c._served = served          # test-visible, for the close() assertion
    return c


# ─────────────────────────────────────────── the reason decides completeness

def test_a_stream_that_finishes_normally_returns_its_text(tmp_path) -> None:
    c = _client(tmp_path, [[_Chunk("```rust\nfn f() {}\n```"),
                            _Chunk(finish_reason="stop")]])
    assert c._stream([]) == "```rust\nfn f() {}\n```"
    assert c._last_finish_reason == "stop"


@pytest.mark.parametrize("reason", ["length", "content_filter"])
def test_a_stream_the_api_cut_short_raises_instead_of_returning_the_fragment(
    tmp_path, reason: str,
) -> None:
    c = _client(tmp_path, [[_Chunk("```rust\nlet v = from_raw_parts(p"),
                            _Chunk(finish_reason=reason)]])
    with pytest.raises(IncompleteResponseError) as exc:
        c._stream([])
    assert reason in str(exc.value)


def test_a_stream_that_never_reports_a_reason_is_treated_as_cut_short(
    tmp_path,
) -> None:
    """The observed failure: the server closes the stream mid-sentence and
    the iterator simply ends. No exception, no reason — and before this fix,
    no signal of any kind that the text was a fragment."""
    c = _client(tmp_path, [[_Chunk("```rust\nlet v = from_raw_parts(p")]])
    with pytest.raises(IncompleteResponseError):
        c._stream([])


def test_the_connection_is_released_even_when_the_reply_was_cut_short(
    tmp_path,
) -> None:
    c = _client(tmp_path, [[_Chunk("half"), _Chunk(finish_reason="length")]])
    with pytest.raises(IncompleteResponseError):
        c._stream([])
    assert c._served[0].closed, "stream must be closed or the pool leaks"


# ─────────────────────────────────────────────── what chat() does around it

def test_chat_retries_a_truncated_reply_and_returns_the_whole_one(
    tmp_path,
) -> None:
    """The retry loop already existed; it only ever caught exceptions, so a
    truncation — which raised nothing — walked straight past it."""
    c = _client(tmp_path, [
        [_Chunk("```rust\nlet v = from_raw"), _Chunk(finish_reason="length")],
        [_Chunk("```rust\nfn f() {}\n```"), _Chunk(finish_reason="stop")],
    ])
    out = c.chat("sys", "user")
    assert out == "```rust\nfn f() {}\n```"


def test_a_truncated_reply_is_never_written_to_the_cache(tmp_path) -> None:
    """Caching a fragment makes the failure permanent: the same prompt then
    replays the fragment forever without ever reaching the API again. Two
    such entries were found in the corpus."""
    c = _client(tmp_path, [
        [_Chunk("```rust\nlet v = from_raw"), _Chunk(finish_reason="length")],
        [_Chunk("```rust\nfn f() {}\n```"), _Chunk(finish_reason="stop")],
    ])
    c.chat("sys", "user")
    assert all(v.count("```") % 2 == 0 for v in c._cache.values()), (
        f"a fragment reached the cache: {c._cache}")


def test_chat_gives_up_with_an_empty_string_when_every_attempt_is_truncated(
    tmp_path,
) -> None:
    """Exhausting the retries is a failure, not an answer — chat()'s contract
    is to return "" so callers fall through to "no usable fix". What must not
    happen is a fragment being returned as though the model wrote it."""
    cut = [_Chunk("```rust\nlet v = from_raw"), _Chunk(finish_reason="length")]
    c = _client(tmp_path, [list(cut) for _ in range(3)])
    assert c.chat("sys", "user") == ""
    assert c._cache == {}


# ───────────────────────────────────────────────────── non-streaming path

def test_the_non_streaming_path_checks_the_reason_too(tmp_path) -> None:
    c = _client(tmp_path, [])

    class _Resp:
        choices = [_Choice("```rust\nlet v = from_raw", "length")]

    class _Completions:
        def create(self, **kwargs):
            return _Resp()

    class _Chat:
        completions = _Completions()

    class _SDK:
        chat = _Chat()

    c._client = _SDK()
    c.cfg.stream = False
    with pytest.raises(IncompleteResponseError):
        c._oneshot([])
