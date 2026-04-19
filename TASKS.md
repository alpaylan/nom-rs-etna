# nom — ETNA Tasks

Total tasks: 12

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task. The `<PropertyKey>` placeholder in the command column uses the PascalCase key recognised by `src/bin/etna.rs`; passing `All` runs every property for the named framework in a single invocation.

## Property keys

| Property | PropertyKey |
|----------|-------------|
| `property_multispace0_consumes_all_whitespace` | `Multispace0ConsumesAllWhitespace` |
| `property_float_parses_infinity_fully` | `FloatParsesInfinityFully` |
| `property_count_handles_zero_sized_output` | `CountHandlesZeroSizedOutput` |

## Task Index

| Task | Variant | Framework | Property | Witness | Command |
|------|---------|-----------|----------|---------|---------|
| 001 | `multispace0_consumes_all_whitespace_51c3c4e_1` | proptest    | `property_multispace0_consumes_all_whitespace` | `witness_multispace0_consumes_all_whitespace_case_newline_only` | `cargo run --release --bin etna -- proptest Multispace0ConsumesAllWhitespace` |
| 002 | `multispace0_consumes_all_whitespace_51c3c4e_1` | quickcheck  | `property_multispace0_consumes_all_whitespace` | `witness_multispace0_consumes_all_whitespace_case_mixed_ws`     | `cargo run --release --bin etna -- quickcheck Multispace0ConsumesAllWhitespace` |
| 003 | `multispace0_consumes_all_whitespace_51c3c4e_1` | crabcheck   | `property_multispace0_consumes_all_whitespace` | `witness_multispace0_consumes_all_whitespace_case_newline_only` | `cargo run --release --bin etna -- crabcheck Multispace0ConsumesAllWhitespace` |
| 004 | `multispace0_consumes_all_whitespace_51c3c4e_1` | hegel       | `property_multispace0_consumes_all_whitespace` | `witness_multispace0_consumes_all_whitespace_case_mixed_ws`     | `cargo run --release --bin etna -- hegel Multispace0ConsumesAllWhitespace` |
| 005 | `float_parses_infinity_fully_63def4e_1`         | proptest    | `property_float_parses_infinity_fully`         | `witness_float_parses_infinity_fully_case_lowercase`             | `cargo run --release --bin etna -- proptest FloatParsesInfinityFully` |
| 006 | `float_parses_infinity_fully_63def4e_1`         | quickcheck  | `property_float_parses_infinity_fully`         | `witness_float_parses_infinity_fully_case_mixed_case`            | `cargo run --release --bin etna -- quickcheck FloatParsesInfinityFully` |
| 007 | `float_parses_infinity_fully_63def4e_1`         | crabcheck   | `property_float_parses_infinity_fully`         | `witness_float_parses_infinity_fully_case_lowercase`             | `cargo run --release --bin etna -- crabcheck FloatParsesInfinityFully` |
| 008 | `float_parses_infinity_fully_63def4e_1`         | hegel       | `property_float_parses_infinity_fully`         | `witness_float_parses_infinity_fully_case_mixed_case`            | `cargo run --release --bin etna -- hegel FloatParsesInfinityFully` |
| 009 | `count_handles_zero_sized_output_931bcf0_1`     | proptest    | `property_count_handles_zero_sized_output`     | `witness_count_handles_zero_sized_output_case_three_abc`         | `cargo run --release --bin etna -- proptest CountHandlesZeroSizedOutput` |
| 010 | `count_handles_zero_sized_output_931bcf0_1`     | quickcheck  | `property_count_handles_zero_sized_output`     | `witness_count_handles_zero_sized_output_case_three_abc`         | `cargo run --release --bin etna -- quickcheck CountHandlesZeroSizedOutput` |
| 011 | `count_handles_zero_sized_output_931bcf0_1`     | crabcheck   | `property_count_handles_zero_sized_output`     | `witness_count_handles_zero_sized_output_case_three_abc`         | `cargo run --release --bin etna -- crabcheck CountHandlesZeroSizedOutput` |
| 012 | `count_handles_zero_sized_output_931bcf0_1`     | hegel       | `property_count_handles_zero_sized_output`     | `witness_count_handles_zero_sized_output_case_three_abc`         | `cargo run --release --bin etna -- hegel CountHandlesZeroSizedOutput` |

## Witness catalog

Each witness is a deterministic concrete test. Base build: passes. Variant-active build: fails.

- `witness_multispace0_consumes_all_whitespace_case_newline_only` — `property_multispace0_consumes_all_whitespace(vec![2])` → `Pass` (bytes map via `sanitize_ws` to `"\n"`, buggy branch returns `rem="", out=""`).
- `witness_multispace0_consumes_all_whitespace_case_mixed_ws` — `property_multispace0_consumes_all_whitespace(vec![0, 1, 2, 3])` → `Pass` (maps to `" \t\n\r"`; same detection path).
- `witness_float_parses_infinity_fully_case_lowercase` — `property_float_parses_infinity_fully(vec![0])` → `Pass` (seed selects `"infinity"`; buggy branch leaves remainder `"inity"`).
- `witness_float_parses_infinity_fully_case_mixed_case` — `property_float_parses_infinity_fully(vec![3])` → `Pass` (seed selects `"InFiNiTy"`; same detection path via `tag_no_case`).
- `witness_count_handles_zero_sized_output_case_three_abc` — `property_count_handles_zero_sized_output(2)` → `Pass` (parses `"abcabcabc"` with ZST output type, leaves `"def"`; buggy branch divides `MAX_INITIAL_CAPACITY_BYTES` by `size_of::<()>() == 0` → panic).
