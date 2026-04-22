# nom-rs — ETNA Tasks

Total tasks: 12

## Task Index

| Task | Variant | Framework | Property | Witness |
|------|---------|-----------|----------|---------|
| 001 | `count_handles_zero_sized_output_931bcf0_1` | proptest | `CountHandlesZeroSizedOutput` | `witness_count_handles_zero_sized_output_case_three_abc` |
| 002 | `count_handles_zero_sized_output_931bcf0_1` | quickcheck | `CountHandlesZeroSizedOutput` | `witness_count_handles_zero_sized_output_case_three_abc` |
| 003 | `count_handles_zero_sized_output_931bcf0_1` | crabcheck | `CountHandlesZeroSizedOutput` | `witness_count_handles_zero_sized_output_case_three_abc` |
| 004 | `count_handles_zero_sized_output_931bcf0_1` | hegel | `CountHandlesZeroSizedOutput` | `witness_count_handles_zero_sized_output_case_three_abc` |
| 005 | `float_parses_infinity_fully_63def4e_1` | proptest | `FloatParsesInfinityFully` | `witness_float_parses_infinity_fully_case_lowercase` |
| 006 | `float_parses_infinity_fully_63def4e_1` | quickcheck | `FloatParsesInfinityFully` | `witness_float_parses_infinity_fully_case_lowercase` |
| 007 | `float_parses_infinity_fully_63def4e_1` | crabcheck | `FloatParsesInfinityFully` | `witness_float_parses_infinity_fully_case_lowercase` |
| 008 | `float_parses_infinity_fully_63def4e_1` | hegel | `FloatParsesInfinityFully` | `witness_float_parses_infinity_fully_case_lowercase` |
| 009 | `multispace0_consumes_all_whitespace_51c3c4e_1` | proptest | `Multispace0ConsumesAllWhitespace` | `witness_multispace0_consumes_all_whitespace_case_newline_only` |
| 010 | `multispace0_consumes_all_whitespace_51c3c4e_1` | quickcheck | `Multispace0ConsumesAllWhitespace` | `witness_multispace0_consumes_all_whitespace_case_newline_only` |
| 011 | `multispace0_consumes_all_whitespace_51c3c4e_1` | crabcheck | `Multispace0ConsumesAllWhitespace` | `witness_multispace0_consumes_all_whitespace_case_newline_only` |
| 012 | `multispace0_consumes_all_whitespace_51c3c4e_1` | hegel | `Multispace0ConsumesAllWhitespace` | `witness_multispace0_consumes_all_whitespace_case_newline_only` |

## Witness Catalog

- `witness_count_handles_zero_sized_output_case_three_abc` — base passes, variant fails
- `witness_float_parses_infinity_fully_case_lowercase` — base passes, variant fails
- `witness_float_parses_infinity_fully_case_mixed_case` — base passes, variant fails
- `witness_multispace0_consumes_all_whitespace_case_newline_only` — base passes, variant fails
- `witness_multispace0_consumes_all_whitespace_case_mixed_ws` — base passes, variant fails
