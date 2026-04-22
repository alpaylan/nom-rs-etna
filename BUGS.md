# nom-rs — Injected Bugs

Total mutations: 3

## Bug Index

| # | Variant | Name | Location | Injection | Fix Commit |
|---|---------|------|----------|-----------|------------|
| 1 | `count_handles_zero_sized_output_931bcf0_1` | `count_handles_zero_sized_output` | `src/multi/mod.rs` | `patch` | `931bcf0109decd100746297a0d0fa243f8c39e23` |
| 2 | `float_parses_infinity_fully_63def4e_1` | `float_parses_infinity_fully` | `src/number/complete.rs` | `patch` | `63def4e16b1273f702f1a77f19a0b61d2bcb1e18` |
| 3 | `multispace0_consumes_all_whitespace_51c3c4e_1` | `multispace0_consumes_all_whitespace` | `src/traits.rs` | `patch` | `51c3c4e44fa78a8a09b413419372b97b2cc2a787` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `count_handles_zero_sized_output_931bcf0_1` | `CountHandlesZeroSizedOutput` | `witness_count_handles_zero_sized_output_case_three_abc` |
| `float_parses_infinity_fully_63def4e_1` | `FloatParsesInfinityFully` | `witness_float_parses_infinity_fully_case_lowercase`, `witness_float_parses_infinity_fully_case_mixed_case` |
| `multispace0_consumes_all_whitespace_51c3c4e_1` | `Multispace0ConsumesAllWhitespace` | `witness_multispace0_consumes_all_whitespace_case_newline_only`, `witness_multispace0_consumes_all_whitespace_case_mixed_ws` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `CountHandlesZeroSizedOutput` | ✓ | ✓ | ✓ | ✓ |
| `FloatParsesInfinityFully` | ✓ | ✓ | ✓ | ✓ |
| `Multispace0ConsumesAllWhitespace` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. count_handles_zero_sized_output

- **Variant**: `count_handles_zero_sized_output_931bcf0_1`
- **Location**: `src/multi/mod.rs`
- **Property**: `CountHandlesZeroSizedOutput`
- **Witness(es)**:
  - `witness_count_handles_zero_sized_output_case_three_abc`
- **Source**: avoid panic when counting zero-sized outputs in count() (#1618)
  > `count`'s capacity heuristic divided a byte budget by `size_of::<O>()` without guarding the divisor, so a parser whose output is a zero-sized type hit a divide-by-zero panic; the fix reinstates `.max(1)` on the divisor.
- **Fix commit**: `931bcf0109decd100746297a0d0fa243f8c39e23` — avoid panic when counting zero-sized outputs in count() (#1618)
- **Invariant violated**: `count(parser, n)` with a parser whose output is a zero-sized type must not panic during capacity allocation.
- **How the mutation triggers**: The initial capacity divisor `MAX_INITIAL_CAPACITY_BYTES / size_of::<O>()` drops the `.max(1)` guard, so when `O = ()` (size 0) the division panics with "attempt to divide by zero".

### 2. float_parses_infinity_fully

- **Variant**: `float_parses_infinity_fully_63def4e_1`
- **Location**: `src/number/complete.rs`
- **Property**: `FloatParsesInfinityFully`
- **Witness(es)**:
  - `witness_float_parses_infinity_fully_case_lowercase`
  - `witness_float_parses_infinity_fully_case_mixed_case`
- **Source**: fix: parse "infinity" literal fully in `float` (#1673)

  > In `recognize_float_or_exceptions` the `tag_no_case("inf")` branch appeared before `tag_no_case("infinity")` inside the `alt`, so parsing "infinity" matched the shorter prefix and left "inity" unconsumed; reordering the branches lets the longer literal win.
- **Fix commit**: `63def4e16b1273f702f1a77f19a0b61d2bcb1e18` — fix: parse "infinity" literal fully in `float` (#1673)

- **Invariant violated**: `float("infinity")` must return `("", +inf)` — no unparsed remainder.
- **How the mutation triggers**: Inside `recognize_float_or_exceptions` the `alt` branches test `tag_no_case("inf")` before `tag_no_case("infinity")`, so the short tag matches first and leaves `"inity"` unconsumed.

### 3. multispace0_consumes_all_whitespace

- **Variant**: `multispace0_consumes_all_whitespace_51c3c4e_1`
- **Location**: `src/traits.rs`
- **Property**: `Multispace0ConsumesAllWhitespace`
- **Witness(es)**:
  - `witness_multispace0_consumes_all_whitespace_case_newline_only`
  - `witness_multispace0_consumes_all_whitespace_case_mixed_ws`
- **Source**: Fix for issue 1808 (#1811)
  > `multispace0` (via `split_at_position_complete`) split at offset 0 when every byte matched the predicate, so on an all-whitespace input the parser returned consumed="" / remainder=whole instead of consuming everything; the fix delegates the exhaustive case to `take_split(self.input_len())`.
- **Fix commit**: `51c3c4e44fa78a8a09b413419372b97b2cc2a787` — Fix for issue 1808 (#1811)
- **Invariant violated**: `recognize(multispace0)` on an all-whitespace `&str` input must return `(rem="", consumed=whole)`.
- **How the mutation triggers**: The `None` branch of `split_at_position_complete` for `&str` returns `self.split_at(0)` (yielding consumed="", remainder=whole at offset 0) instead of `self.take_split(self.input_len())` (yielding consumed=whole, remainder="" at offset len).
