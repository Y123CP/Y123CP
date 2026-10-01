# Optimization Card Templates: Paper Terminology and Implementation Mapping

RQ3 identifies nine recurring **Performance Issues**, each with an
**Observed Pattern** and an **Optimization Direction**. The approach encodes
each pattern–direction pair as an **optimization rule**, then adds
**Rewrite Preconditions** to form an **optimization card**. Thus, cards are
the approach's representation of optimization guidance; they are not the
name of the RQ3 findings themselves.

This directory contains 18
implementation templates, identified by the IDs used in detection and rewrite
logs. Several templates implement different rewrite forms within the same
paper-level issue; the number of files is not the number of paper-level rules.

This README is a reviewer guide. The artifact's template checklists have
been revised to align with the paper's rewrite-precondition description and
figure; implementation IDs and filenames are retained. These revised files
are used for future runs and are not a claim that historical runs used this
exact prompt text. Recorded experimental outputs are unchanged. The loader
in `prompt_builder.py` loads a matching template by ID and does not include
this README in an LLM prompt.

## Paper names and template IDs

| Performance issue in the paper | Template IDs and files |
|---|---|
| Residual Bounds Checks | [C1](C1_redundant_check.md) |
| Redundant Conversion Saturation | [C2](C2_scalar_conversion.md) |
| Missed Loop Vectorization | [II_vec](II_vec_vectorization_loss.md) |
| Missed Hot-Callee Inlining | [II_inl](II_inl_uninlined.md) |
| Repeated Invariant Dispatch | [III1](III1_callback_monomorph.md), [C6](C6_dispatch_hoisting.md), [C11](C11_goto_state_dispatch.md) |
| Inefficient Buffer Management | [C7](C7_amortized_buffer_growth.md), [III2](III2_manual_heap_to_raii.md) |
| Byte-Oriented Memory Operations | [III3](III3_mem_ops_to_slice.md), [C12](C12_eager_zero_init.md) |
| Scalar Element-Wise Processing | [C4](C4_word_at_a_time.md), [C5](C5_reduction_reassociation.md), [C8](C8_crc_slicing.md), [C10](C10_fixed_subword_loop.md) |
| Obscured Memory Access Properties | [C3](C3_aliasing_gap.md), [III4](III4_raw_ptr_cursor.md), [II_const](II_const_promote.md) |

The log IDs `III①`, `III②`, `III③`, and `III④` are normalized to the file
prefixes `III1`, `III2`, `III3`, and `III4`. A suffix such as `C11.split`
selects the base template `C11`; filenames and IDs should remain stable.

`C9` and `II_iso` also occur in the implementation and attribution mapping,
but have no Markdown card in this directory. They use deterministic actions
in `agent.py`: bitfield lowering for `C9`, and inlining isolation for `II_iso`.
The attribution mapping groups these with Typed Memory Operations and
Hot-Callee Inlining Restoration, respectively. Inlining isolation is a
distinct transformation from restoring a missing inline. Other IDs can also
use deterministic paths; a listed template does not imply every accepted
rewrite was generated from an LLM prompt containing that template.

The names in `tools/facet_coverage.py::RULE_OF` describe optimization
directions, such as Memory Property Recovery and Data-Level Parallelism
Exposure. The table above uses the corresponding performance-issue names
from the paper. These are two naming levels, not additional rules.

## The three fields shown in the paper figure

The figure presents a matched card as:

1. **Observed Pattern**: the source or compiler-evidence pattern that matched
   the region. A match identifies a candidate; it does not prove a rewrite
   is legal.
2. **Optimization Direction**: the transformation to consider for that
   pattern.
3. **Rewrite Preconditions**: the semantic conditions to establish from
   the enclosing function and dependency context before applying the
   transformation.

The six templates associated with the two cards in the figure (`C3`, `III4`,
`C4`, `C5`, `C8`, `C10`) now start with the figure's three fields verbatim,
including the optimization direction and the three standalone checklist
items. These fields reproduce the matched `zrsh` example; the numbered
implementation sections then describe each template's specific scope.
Other templates retain their existing numbered sections.

For the six figure-aligned templates:

| Figure field | Where to find the content in an existing template |
|---|---|
| Observed Pattern | Opening `Observed Pattern`; concrete template scope in `Implementation-specific pattern` |
| Optimization Direction | Opening `Optimization Direction` uses the figure wording; `Template strategy` and rewrite examples give implementation details |
| Rewrite Preconditions | Opening `Rewrite Preconditions` uses the figure wording; numbered `Implementation-specific preconditions` supplies supporting checks |

Post-rewrite checks and performance gates are additional validation steps.
They do not replace establishing semantic preconditions. The same requirement
applies to every class, including control-flow and type/API rewrites.

## Shared context and matched instances

`Source Region`, `Enclosing Function`, and `Dependency Context` in the figure
are per-instance program context. The opening fields of the six aligned
templates reproduce the figure instance for reference, while the region
being optimized is supplied by the prompt builder. The region prompt builder supplies the editable region, surrounding
function body, anchor-hit details, candidate cards, and available dependency
context. This grounds a generic pattern in a concrete source location.

The two card numbers in the figure are local labels within that example;
they are not implementation IDs `C1` and `C2`.

## Reading the `zrsh` example

The figure's **Obscured Memory Access Properties** card corresponds to the
memory-property family, including `C3` and `III4`. Its mutable-slice example
uses these figure-level checklist items, repeated in `C3` and `III4`:

- [ ] The buffer extent can be established.
- [ ] All accesses remain within this buffer extent.
- [ ] No conflicting access occurs while the mutable slice is live.

These are followed by the API- and template-specific checks. The figure is an
abbreviated checklist; it does not replace the full validity obligations of
the chosen Rust API.

The **Scalar Element-Wise Processing** family (`C4`, `C5`, `C8`, `C10`)
uses the figure's core checklist, followed by operation-specific checks:

- [ ] The recurrence admits an equivalent block-wise formulation.
- [ ] Required inputs can be read before overlapping outputs are written.
- [ ] Boundary cases preserve the original semantics.

For independent fixed-length operations, the first item establishes the
equivalence of grouping their scalar steps; it does not assert that every
operation has a loop-carried recurrence. The adjacent-limb shift loop is not
automatically an instance of every template in this family:

- `C4` targets byte-wise comparisons and scans, not arbitrary `u64` updates.
- `C5` targets reduction recurrences, such as a running sum of a running sum.
- `C8` targets a CRC table recurrence.
- `C10` targets fixed-length sub-word operations.

The figure's in-place `zrsh` transformation needs its own argument that a
block-wise formulation is equivalent, that required old values are read
before overlapping outputs overwrite them, and that boundary cases preserve
the original behavior. Matching adjacent accesses alone does not establish
these conditions. This README does not add a new detector, rewrite template,
or recorded experiment for that example.

## Validation contract

Apply a template only when its relevant preconditions are established from
the supplied program context. If the conditions are unresolved, abstain from
that rewrite. Introduced unsafe operations require a specific `SAFETY`
justification. Candidate rewrites still pass the implementation's build,
functional, and performance gates; passing workload checks alone is not a
proof of correctness for all possible inputs.

The nine RQ3 findings are presented separately in
`forRQ3/performance_issues/`. This directory contains the approach's
implementation card templates, including rewrite preconditions.

## API references for the detailed checks

The additional validity checks follow the APIs introduced by each rewrite,
including Rust's [mutable-slice construction requirements](https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html)
and [unchecked float-to-integer conversion requirements](https://doc.rust-lang.org/std/primitive.f64.html#method.to_int_unchecked).
For allocation queries, the [Linux `malloc_usable_size` contract](https://man7.org/linux/man-pages/man3/malloc_usable_size.3.html)
does not permit treating surplus reported space as writable requested capacity.
