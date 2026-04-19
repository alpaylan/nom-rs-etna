# nom — Injected Bugs

Total mutations: 3

## Bug Index

| # | Name | Variant | File | Injection | Fix Commit |
|---|------|---------|------|-----------|------------|
| 1 | `multispace0_consumes_all_whitespace` | `multispace0_consumes_all_whitespace_51c3c4e_1` | `patches/multispace0_consumes_all_whitespace_51c3c4e_1.patch` | `patch` | `51c3c4e44fa78a8a09b413419372b97b2cc2a787` |
| 2 | `float_parses_infinity_fully` | `float_parses_infinity_fully_63def4e_1` | `patches/float_parses_infinity_fully_63def4e_1.patch` | `patch` | `63def4e16b1273f702f1a77f19a0b61d2bcb1e18` |
| 3 | `count_handles_zero_sized_output` | `count_handles_zero_sized_output_931bcf0_1` | `patches/count_handles_zero_sized_output_931bcf0_1.patch` | `patch` | `931bcf0109decd100746297a0d0fa243f8c39e23` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `multispace0_consumes_all_whitespace_51c3c4e_1` | `property_multispace0_consumes_all_whitespace` | `witness_multispace0_consumes_all_whitespace_case_newline_only`, `witness_multispace0_consumes_all_whitespace_case_mixed_ws` |
| `float_parses_infinity_fully_63def4e_1` | `property_float_parses_infinity_fully` | `witness_float_parses_infinity_fully_case_lowercase`, `witness_float_parses_infinity_fully_case_mixed_case` |
| `count_handles_zero_sized_output_931bcf0_1` | `property_count_handles_zero_sized_output` | `witness_count_handles_zero_sized_output_case_three_abc` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `property_multispace0_consumes_all_whitespace` | ✓ | ✓ | ✓ | ✓ |
| `property_float_parses_infinity_fully` | ✓ | ✓ | ✓ | ✓ |
| `property_count_handles_zero_sized_output` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. multispace0_consumes_all_whitespace
- **Variant**: `multispace0_consumes_all_whitespace_51c3c4e_1`
- **Location**: `patches/multispace0_consumes_all_whitespace_51c3c4e_1.patch` (target `src/traits.rs`)
- **Property**: `property_multispace0_consumes_all_whitespace`
- **Witness(es)**: `witness_multispace0_consumes_all_whitespace_case_newline_only`, `witness_multispace0_consumes_all_whitespace_case_mixed_ws`
- **Fix commit**: `51c3c4e44fa78a8a09b413419372b97b2cc2a787` — Fix for issue 1808 (#1811)
- **Invariant violated**: `recognize(multispace0)` on an all-whitespace `&str` input must return `(rem="", consumed=whole)`.
- **How the mutation triggers**: The `None` branch of `split_at_position_complete` for `&str` returns `self.split_at(0)` (yielding consumed="", remainder=whole at offset 0) instead of `self.take_split(self.input_len())` (yielding consumed=whole, remainder="" at offset len).

### 2. float_parses_infinity_fully
- **Variant**: `float_parses_infinity_fully_63def4e_1`
- **Location**: `patches/float_parses_infinity_fully_63def4e_1.patch` (target `src/number/complete.rs`)
- **Property**: `property_float_parses_infinity_fully`
- **Witness(es)**: `witness_float_parses_infinity_fully_case_lowercase`, `witness_float_parses_infinity_fully_case_mixed_case`
- **Fix commit**: `63def4e16b1273f702f1a77f19a0b61d2bcb1e18` — fix: parse "infinity" literal fully in `float` (#1673)
- **Invariant violated**: `float("infinity")` must return `("", +inf)` — no unparsed remainder.
- **How the mutation triggers**: Inside `recognize_float_or_exceptions` the `alt` branches test `tag_no_case("inf")` before `tag_no_case("infinity")`, so the short tag matches first and leaves `"inity"` unconsumed.

### 3. count_handles_zero_sized_output
- **Variant**: `count_handles_zero_sized_output_931bcf0_1`
- **Location**: `patches/count_handles_zero_sized_output_931bcf0_1.patch` (target `src/multi/mod.rs`)
- **Property**: `property_count_handles_zero_sized_output`
- **Witness(es)**: `witness_count_handles_zero_sized_output_case_three_abc`
- **Fix commit**: `931bcf0109decd100746297a0d0fa243f8c39e23` — avoid panic when counting zero-sized outputs in count() (#1618)
- **Invariant violated**: `count(parser, n)` with a parser whose output is a zero-sized type must not panic during capacity allocation.
- **How the mutation triggers**: The initial capacity divisor `MAX_INITIAL_CAPACITY_BYTES / size_of::<O>()` drops the `.max(1)` guard, so when `O = ()` (size 0) the division panics with "attempt to divide by zero".
