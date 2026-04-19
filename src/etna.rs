//! ETNA framework-neutral property functions for nom.
//!
//! Each `property_<name>` is a pure function taking concrete, owned inputs and
//! returning `PropertyResult`. Framework adapters (proptest/quickcheck/crabcheck/hegel)
//! in `src/bin/etna.rs` and the deterministic witness tests in
//! `tests/etna_witnesses.rs` both call these functions directly — there is no
//! re-implementation of the invariant inside any adapter.

#![allow(missing_docs)]

use crate::bytes::complete::tag;
use crate::character::complete::multispace0;
use crate::combinator::{map, recognize};
use crate::multi::count;
use crate::number::complete::float;
use crate::Parser;

pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

fn sanitize_ws(payload: &[u8]) -> String {
    // Map bytes to whitespace characters so `multispace0` matches the entire
    // output. Bytes are modulo-classed into space/tab/newline/CR so the
    // property exercises all four whitespace kinds across a run.
    let mut out = String::with_capacity(payload.len().min(16));
    for &b in payload.iter().take(16) {
        let c = match b % 4 {
            0 => ' ',
            1 => '\t',
            2 => '\n',
            _ => '\r',
        };
        out.push(c);
    }
    if out.is_empty() {
        out.push(' ');
    }
    out
}

// Property 1: `split_at_position_complete` on `&str` returns the whole input
// when the predicate never matches — exercised through `multispace0`.
//
// Regression for 51c3c4e (issue #1808). The buggy `None` branch of the `&str`
// impl returned `Ok(self.split_at(0))`, which typechecks but yields a tuple
// in the wrong order — `(prefix="", suffix=whole)` — so `IResult` was built
// as `(remainder=whole, consumed="")`. The fix swaps to
// `Ok(self.take_split(self.input_len()))`, returning `(remainder="",
// consumed=whole)`.
pub fn property_multispace0_consumes_all_whitespace(payload: Vec<u8>) -> PropertyResult {
    let input = sanitize_ws(&payload);
    if input.is_empty() {
        return PropertyResult::Discard;
    }

    let parsed: Result<(&str, &str), _> =
        recognize::<_, crate::error::Error<_>, _>(multispace0).parse(input.as_str());

    match parsed {
        Ok((rem, out)) => {
            if rem.is_empty() && out == input.as_str() {
                PropertyResult::Pass
            } else {
                PropertyResult::Fail(format!(
                    "recognize(multispace0) on {:?}: got rem={:?} out={:?}, expected rem=\"\" out=whole",
                    input, rem, out
                ))
            }
        }
        Err(e) => PropertyResult::Fail(format!(
            "recognize(multispace0) errored on {:?}: {:?}",
            input, e
        )),
    }
}

// Property 2: `float("infinity")` consumes the whole literal.
//
// Regression for 63def4e. Inside `recognize_float_or_exceptions` the `alt`
// tested `tag_no_case("inf")` before `tag_no_case("infinity")`. The short
// tag matched first, so `float("infinity")` returned `("inity", +inf)`
// instead of `("", +inf)`. The fix swapped the branches.
fn pick_infinity_casing(seed: u8) -> &'static str {
    match seed % 6 {
        0 => "infinity",
        1 => "Infinity",
        2 => "INFINITY",
        3 => "InFiNiTy",
        4 => "iNFINITY",
        _ => "infiNITY",
    }
}

pub fn property_float_parses_infinity_fully(payload: Vec<u8>) -> PropertyResult {
    let seed = payload.first().copied().unwrap_or(0);
    let literal = pick_infinity_casing(seed);

    match float::<_, crate::error::Error<_>>(literal) {
        Ok((rem, value)) => {
            if !value.is_infinite() {
                return PropertyResult::Fail(format!(
                    "float({:?}) parsed value {} is not infinite",
                    literal, value
                ));
            }
            if !rem.is_empty() {
                return PropertyResult::Fail(format!(
                    "float({:?}) left remainder {:?} (expected empty)",
                    literal, rem
                ));
            }
            PropertyResult::Pass
        }
        Err(e) => PropertyResult::Fail(format!("float({:?}) errored: {:?}", literal, e)),
    }
}

// Property 3: `count` handles zero-sized parser outputs without panicking.
//
// Regression for 931bcf01. The capacity cap was
// `MAX_INITIAL_CAPACITY_BYTES / size_of::<O>()`. For `O = ()` that divides by
// zero and panics during parsing. The fix adds `.max(1)` to the divisor.
pub fn property_count_handles_zero_sized_output(repeats: u8) -> PropertyResult {
    let n = (repeats as usize) % 4 + 1;
    let mut input = String::with_capacity(3 * n + 3);
    for _ in 0..n {
        input.push_str("abc");
    }
    input.push_str("def");

    let parser = map(tag::<_, _, crate::error::Error<_>>("abc"), |_| ());
    let mut counted = count(parser, n);

    match counted.parse(input.as_str()) {
        Ok((rem, out)) => {
            if rem != "def" {
                return PropertyResult::Fail(format!(
                    "count returned rem={:?}, expected \"def\"",
                    rem
                ));
            }
            if out.len() != n {
                return PropertyResult::Fail(format!(
                    "count returned {} elements, expected {}",
                    out.len(),
                    n
                ));
            }
            PropertyResult::Pass
        }
        Err(e) => PropertyResult::Fail(format!("count errored: {:?}", e)),
    }
}
