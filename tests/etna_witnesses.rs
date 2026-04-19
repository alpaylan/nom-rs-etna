//! Deterministic witness tests for nom ETNA variants.
//!
//! Each `witness_<name>_case_<tag>` passes on the base HEAD and fails when
//! the corresponding `etna/<variant>` branch is checked out (patch applied).
//! Witnesses call `property_<name>` directly with frozen inputs — no
//! proptest/quickcheck/RNG/clock machinery.

use nom::etna::{
    property_count_handles_zero_sized_output, property_float_parses_infinity_fully,
    property_multispace0_consumes_all_whitespace, PropertyResult,
};

fn expect_pass(r: PropertyResult, what: &str) {
    match r {
        PropertyResult::Pass => {}
        PropertyResult::Fail(m) => panic!("{what}: property failed: {m}"),
        PropertyResult::Discard => panic!("{what}: unexpected discard"),
    }
}

// Variant: multispace0_consumes_all_whitespace_51c3c4e_1
#[test]
fn witness_multispace0_consumes_all_whitespace_case_newline_only() {
    // Bytes mapped by `sanitize_ws` to `"\n"` (since 2 % 4 == 2 => '\n').
    expect_pass(
        property_multispace0_consumes_all_whitespace(vec![2u8]),
        "multispace0_consumes_all_whitespace / newline_only",
    );
}

#[test]
fn witness_multispace0_consumes_all_whitespace_case_mixed_ws() {
    // Maps to a mix of space/tab/newline/cr across four characters.
    expect_pass(
        property_multispace0_consumes_all_whitespace(vec![0u8, 1u8, 2u8, 3u8]),
        "multispace0_consumes_all_whitespace / mixed_ws",
    );
}

// Variant: float_parses_infinity_fully_63def4e_1
#[test]
fn witness_float_parses_infinity_fully_case_lowercase() {
    // seed % 6 == 0 => literal "infinity".
    expect_pass(
        property_float_parses_infinity_fully(vec![0u8]),
        "float_parses_infinity_fully / lowercase",
    );
}

#[test]
fn witness_float_parses_infinity_fully_case_mixed_case() {
    // seed % 6 == 3 => literal "InFiNiTy".
    expect_pass(
        property_float_parses_infinity_fully(vec![3u8]),
        "float_parses_infinity_fully / mixed_case",
    );
}

// Variant: count_handles_zero_sized_output_931bcf0_1
#[test]
fn witness_count_handles_zero_sized_output_case_three_abc() {
    // repeats % 4 + 1 == 3 => parses "abcabcabc" and leaves "def".
    expect_pass(
        property_count_handles_zero_sized_output(2u8),
        "count_handles_zero_sized_output / three_abc",
    );
}
