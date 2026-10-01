"""C10 detects a loop by what it MEANS, not by what it looks like.

The rule exists because of a measured gap. On one crate the hottest function
spent its time expanding one byte into eight, one bit per iteration. Five
consecutive pipeline runs matched III④ on it and produced five "wrap it in an
iterator" rewrites; the best measured -0.216%. Replacing the loop with one
SWAR multiply measured **-22.97%** on that operation (cv 0.24%, 8/8 paired
runs agreeing, 900 golden records still passing). No existing rule looks at
whether the *algorithm* can change — they all rewrite the representation.

The first version of this detector was written from that one instance and
keyed on `mask >>= 1`. It found the instance it was written from and nothing
else: 1 of 12 projects. Rewritten against the semantics below it finds 30
loops across 4 projects.

The semantics: a loop whose trip count is a compile-time constant smaller than
a machine word, whose iterations carry nothing to one another beyond the
induction variable, that cannot exit early, and whose body touches one
sub-word unit per step. All its work fits in a register; it pays N iterations
and N branches to do it.
"""

from __future__ import annotations

import pytest

from perf_opt.hot_probe.merged_hits import (
    _C10_MAX_STMTS,
    _c10_classify,
    _c10_const_trip,
    _c10_has_early_exit,
    _c10_has_inner_loop,
    _c10_induction_set,
    _c10_is_secondary_induction,
    _c10_reads_data,
    _c10_touches_subword,
)
from perf_opt.hot_probe.static_facts import _parse_rust_source


def _verdict(src: bytes):
    """Run the real admission sequence, in the order the scanner runs it."""
    tree = _parse_rust_source(src)
    stack = [tree.root_node]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type != "while_expression":
            continue
        trip = _c10_const_trip(n, src)
        if trip is None:
            return "reject:trip-not-constant"
        var, count = trip
        body = n.child_by_field_name("body")
        if body is None:
            return "reject:no-body"
        if _c10_has_early_exit(body):
            return "reject:early-exit"
        if _c10_has_inner_loop(body):
            return "reject:inner-loop"
        if len(body.named_children) > _C10_MAX_STMTS:
            return "reject:body-too-long"
        form = _c10_classify(body, var, src)
        if form is None:
            return "reject:carries-state"
        if form in ("split", "pack") and not _c10_touches_subword(body, src):
            return "reject:not-sub-word"
        return f"{form}:{count}"
    return "reject:no-loop"


# ───────────────────────────── the forms it must find

BIT_EXPANSION = b"""fn f(){ while j < 8 as size_t {
  if data_view[(i) as usize] as c_int & mask as c_int != 0 { *p = 1 as c_uchar; }
  else { *p = 0 as c_uchar; }
  p = p.offset(1); mask = (mask as c_int >> 1 as c_int) as c_uchar;
  j = j.wrapping_add(1); } }"""

FIXED_COPY = b"""fn f(){ while i < 6 as c_uint {
  decode_sync_buffer[i as usize] = (*ctx).decode_sync_buffer[i as usize];
  i = i.wrapping_add(1); } }"""

WORD_TO_BYTES = b"""fn f(){ while i < 4 as c_int {
  *out_buf.m_pBuf.offset(i as isize) = (c >> 24 as c_int) as mz_uint8;
  i += 1; } }"""


def test_bit_expansion_is_a_split() -> None:
    """The measured -22.97% case."""
    assert _verdict(BIT_EXPANSION) == "split:8"


def test_fixed_length_copy_is_a_copy() -> None:
    assert _verdict(FIXED_COPY) == "copy:6"


def test_word_taken_apart_into_bytes_is_a_split() -> None:
    assert _verdict(WORD_TO_BYTES) == "split:4"


# ───────────────────────────── what it must refuse

def test_a_data_dependent_trip_count_is_refused() -> None:
    """`while n < 3 && i < len` — the second clause hands the count back to
    the data, which is C4's situation, not this one."""
    src = b"""fn f(){ while n < 3 as c_int && i < len {
      igroup[n as usize] = c as c_uchar; n += 1; i += 1; } }"""
    assert _verdict(src) == "reject:trip-not-constant"


def test_an_early_exit_is_refused() -> None:
    """A `break` means the count is not really fixed. This exact loop is C4's:
    a bit reservoir filled until the input runs out."""
    src = b"""fn f(){ while bits < 16 as c_uint {
      if have == 0 as c_uint { break; }
      hold = hold.wrapping_add((*next as c_ulong) << bits); bits += 8; } }"""
    assert _verdict(src) == "reject:early-exit"


def test_an_inner_loop_is_refused() -> None:
    src = b"""fn f(){ while i_1 < 4 as mz_uint {
      (*d).m_bit_buffer |= bits_7 << (*d).m_bits_in;
      (*d).m_bits_in = (*d).m_bits_in.wrapping_add(len_7);
      while (*d).m_bits_in >= 8 as mz_uint { flush(); }
      i_1 += 1; } }"""
    assert _verdict(src) == "reject:inner-loop"


def test_a_real_recurrence_is_refused() -> None:
    """The next iteration reads what this one computed FROM A LOAD. Folding
    the loop would change the result."""
    src = b"""fn f(){ while i < 4 as c_int {
      acc = acc + tbl[(acc ^ src[i as usize]) as usize];
      i += 1; } }"""
    assert _verdict(src) == "reject:carries-state"


# ───────────────────────────── the c2rust disguise

def test_a_stepped_cursor_is_an_induction_variable_not_state() -> None:
    """`p = p.offset(1)` and `mask >>= 1` depend only on the iteration count.

    Counting them as loop-carried state is what made the first version of this
    detector classify a textbook bit-expansion as a recurrence.
    """
    tree = _parse_rust_source(BIT_EXPANSION)
    stack = [tree.root_node]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type == "while_expression":
            body = n.child_by_field_name("body")
            ind = {v.decode() for v in _c10_induction_set(body, b"j")}
            assert ind == {"j", "mask", "p"}, ind
            return
    pytest.fail("no loop parsed")


def test_a_method_call_receiver_is_not_a_memory_load() -> None:
    """`p.offset(1)` parses as field_expression under call_expression. Reading
    that as a load makes every c2rust cursor step look like carried data."""
    src = b"fn f(){ p = p.offset(1); }"
    tree = _parse_rust_source(src)
    stack = [tree.root_node]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type == "assignment_expression":
            rhs = n.child_by_field_name("right")
            lhs = n.child_by_field_name("left")
            assert not _c10_reads_data(rhs)
            assert _c10_is_secondary_induction(rhs, lhs.text)
            return
    pytest.fail("no assignment parsed")


def test_a_field_load_is_still_a_load() -> None:
    """The exemption above must not swallow `(*d).field`."""
    src = b"fn f(){ x = (*d).m_bits_in; }"
    tree = _parse_rust_source(src)
    stack = [tree.root_node]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type == "assignment_expression":
            assert _c10_reads_data(n.child_by_field_name("right"))
            return
    pytest.fail("no assignment parsed")


# ───────────────────────────── the sub-word test applies where it earns its keep

def test_copy_does_not_have_to_be_sub_word() -> None:
    """`split`/`pack` fold N bit-operations into one word operation, so their
    units must be sub-word. `copy` folds N loads and stores into one move —
    six `u32`s fold as well as six bytes. Requiring it of `copy` rejected a
    textbook fixed-length array copy."""
    assert _verdict(FIXED_COPY) == "copy:6"
    tree = _parse_rust_source(FIXED_COPY)
    stack = [tree.root_node]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type == "while_expression":
            body = n.child_by_field_name("body")
            assert not _c10_touches_subword(body, FIXED_COPY)
            return
    pytest.fail("no loop parsed")


# ───────────────────────────── boundaries and wiring

def test_the_trip_count_ceiling_is_a_word_of_sub_words() -> None:
    """Above 16 units the loop cannot be describing sub-words of one word."""
    over = b"""fn f(){ while i < 64 as c_int { d[i as usize] = s[i as usize]; i += 1; } }"""
    assert _verdict(over) == "reject:trip-not-constant"


def test_the_scanner_is_called_by_the_merge_step() -> None:
    """A detector nobody calls produces no hits at all."""
    import inspect
    from perf_opt.hot_probe import merged_hits
    src = inspect.getsource(merged_hits)
    assert "civ_c10 = find_fixed_subword_loop_hits(" in src


def test_every_scanner_result_reaches_the_written_hits() -> None:
    """The merge has two lists, and putting a scanner in only one is silent.

    `all_fns` decides which functions get an entry; the `hits` concatenation
    decides what that entry contains. C10 was added to the first and not the
    second: the log reported "1 hot fn(s), 1 loop(s)", `fn_hits.json` came back
    with zero C10 hits, and nothing anywhere said they had been dropped. An
    assertion naming C10 alone would have missed it just as easily — compare
    the two lists mechanically instead.
    """
    import inspect, re
    from perf_opt.hot_probe import merged_hits
    src = inspect.getsource(merged_hits)
    in_fns = set(re.findall(r"set\((civ_\w+|ci|cii|ciii)\)", src))
    concat = set(re.findall(r"(civ_\w+|ci|cii|ciii)\.get\(fn, \[\]\)", src))
    missing = in_fns - concat
    assert not missing, (
        "these scanners contribute functions to `all_fns` but their hits are "
        f"never concatenated, so every hit is silently dropped: {sorted(missing)}")


def _card_text() -> str:
    """Card text with runs of whitespace flattened.

    The assertions below are about what the card SAYS; matching raw text makes
    them fail on a reflowed paragraph, which is a formatting change, not a
    content one.
    """
    import re
    from perf_opt.agent_perf_opt.prompt_builder import load_card
    return re.sub(r"\s+", " ", load_card("C10"))


def test_the_card_separates_this_from_c4() -> None:
    """The two rules both fold byte loops; only the trip count tells them
    apart, and the model has to be told which one it is looking at."""
    card = _card_text()
    assert "Not to be confused with C4" in card
    assert "break" in card


def test_the_card_requires_verifying_swar_constants() -> None:
    """A wrong magic constant is silently wrong on most inputs — W1 may pass
    on the corpus and fail in the field."""
    card = _card_text()
    assert "enumerate" in card and "256" in card


def test_the_card_documents_the_cursor_disguise() -> None:
    card = _card_text()
    assert "self-advancing locals" in card


def test_the_execution_fingerprint_covers_every_form() -> None:
    """Each form's rewrite leaves a different trace; a fingerprint that only
    knew SWAR would read a correct `copy` rewrite as an empty declaration."""
    from perf_opt.agent_perf_opt.rewrite_applier import _RULE_EXEC_FINGERPRINTS
    fp = _RULE_EXEC_FINGERPRINTS["C10"]
    assert "copy_from_slice" in fp          # copy
    assert "from_le_bytes" in fp            # pack
    assert "to_le_bytes" in fp              # split
    assert "wrapping_mul" in fp             # split via SWAR


# ───────────────────────────── accumulation that folds vs. one that cannot

SHIFT_ACCUMULATE = b"""fn f(){ while j < 8 as size_t {
  v = ((v as c_int) << 1 as c_int) as c_uchar;
  v = (v as c_int | *p as c_int) as c_uchar;
  p = p.offset(1); j = j.wrapping_add(1); } }"""

BYTES_INTO_WORD = b"""fn f(){ while i < 4 as c_int {
  ecinum = ecinum << 8 as c_int;
  ecinum |= *data.offset((3 as c_int - i) as isize) as c_uint;
  i += 1; } }"""

TRUE_RECURRENCE = b"""fn f(){ while i < 4 as c_int {
  acc = acc + tbl[(acc ^ src[i as usize]) as usize]; i += 1; } }"""

CRC_RECURRENCE = b"""fn f(){ while i < 8 as c_int {
  crc = crc >> 8 as c_int ^ table[((crc ^ *b as c_ulong) & 0xff) as usize];
  b = b.offset(1); i += 1; } }"""


def test_shift_accumulation_is_a_pack() -> None:
    """`v = (v << 1) | *p` — the previous value is only shifted along; `p`
    alone picks the address, so the eight steps fold into one word read and
    one multiply. Reading this as state left `pack` with zero hits across the
    dataset while exactly this loop sat in a hot function."""
    assert _verdict(SHIFT_ACCUMULATE) == "pack:8"


def test_bytes_assembled_into_a_word_is_a_pack() -> None:
    """`ecinum = ecinum << 8 | data[3 - i]` is `u32::from_be_bytes` spelled
    out."""
    assert _verdict(BYTES_INTO_WORD) == "pack:4"


def test_an_address_computed_from_the_accumulator_is_still_refused() -> None:
    """The line between the two: does the previous value decide WHERE this
    iteration reads. Here it does, so folding changes the result."""
    assert _verdict(TRUE_RECURRENCE) == "reject:carries-state"


def test_crc_recurrence_stays_with_c8() -> None:
    """A CRC table step indexes with the running value. C8 owns this shape and
    rewrites it as slicing-by-N; C10 must not claim it."""
    assert _verdict(CRC_RECURRENCE) == "reject:carries-state"


def test_an_accumulator_is_never_mistaken_for_a_counter() -> None:
    """`v = v << 1` in isolation reads exactly like a cursor step. Admitting it
    to the induction set makes the `v = v | *p` beside it look like an
    induction step too, and the body then classifies as nothing at all."""
    tree = _parse_rust_source(SHIFT_ACCUMULATE)
    stack = [tree.root_node]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type == "while_expression":
            body = n.child_by_field_name("body")
            ind = {v.decode() for v in _c10_induction_set(body, b"j")}
            assert ind == {"j", "p"}, ind
            return
    pytest.fail("no loop parsed")


def test_an_indexed_load_reveals_its_subscript() -> None:
    """tree-sitter-rust gives `a[i]` no `index` field. Asking for one returns
    None, which answers "no address depends on this" for every indexed load —
    and every true recurrence would be admitted."""
    from perf_opt.hot_probe.merged_hits import _c10_addresses_depend_on
    tree = _parse_rust_source(b"fn f(){ x = tbl[(acc ^ y) as usize]; }")
    stack = [tree.root_node]
    while stack:
        n = stack.pop()
        stack.extend(n.children)
        if n.type == "assignment_expression":
            rhs = n.child_by_field_name("right")
            assert _c10_addresses_depend_on(rhs, b"acc")
            assert not _c10_addresses_depend_on(rhs, b"zzz")
            return
    pytest.fail("no assignment parsed")


def test_the_card_covers_bit_level_packing() -> None:
    """The `pack` section's byte-conversion example does not reach the shape
    that actually occurs.

    Measured: given `v = (v << 1) | *p` over eight 0/1 bytes, the model
    declared C10 and wrote `chunk.iter().fold(0, |acc, &b| acc << 1 | b)` —
    eight dependent steps wearing an iterator, which is III④. The execution
    fingerprint stripped C10, correctly, and the loop stayed. `from_le_bytes`
    cannot express this; it needs the multiply from §2.1 run backwards.
    """
    card = _card_text()
    assert "contributes one BIT" in card
    assert "wrapping_mul(GATHER)" in card
    assert "fold" in card, "the card must name the non-answer it saw"


def test_the_lane_constants_are_written_as_bytes_not_hex() -> None:
    """A 64-bit mask written in hex hides its byte order.

    Measured: the model reproduced the split rewrite with the selection mask
    reversed — `0x8040_2010_0804_0201` for a transform that needs
    `0x0102_0408_1020_4080`. It compiled, it looked right, and W1 caught it in
    0.0s. The array form makes element `i` the mask for output byte `i`, in
    the same order `to_le_bytes` writes them.
    """
    card = _card_text()
    assert "u64::from_le_bytes([0x80,0x40,0x20,0x10,0x08,0x04,0x02,0x01])" in card
    for hexed in ("0x0102_0408_1020_4080", "0x0101_0101_0101_0101",
                  "0x7f7f_7f7f_7f7f_7f7f"):
        assert hexed not in card, f"{hexed} still written as a hex literal"


def test_the_card_names_the_lane_constants_it_uses() -> None:
    """Every constant the templates reference must be defined on the page —
    a `const` the model has to invent is a constant it can invent wrong."""
    card = _card_text()
    for name in ("LANES", "HALF", "SELECT", "GATHER"):
        assert f"const {name}" in card or f"`{name}` is" in card, name


def test_the_card_says_a_surviving_loop_means_the_rule_did_not_apply() -> None:
    card = _card_text()
    assert "did not apply" in card
