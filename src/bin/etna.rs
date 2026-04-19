// ETNA workload runner for nom.
//
// Usage: cargo run --release --bin etna -- <tool> <property>
//   tool:     etna | proptest | quickcheck | crabcheck | hegel
//   property: Multispace0ConsumesAllWhitespace
//           | FloatParsesInfinityFully
//           | CountHandlesZeroSizedOutput
//           | All
//
// Each invocation emits a single JSON line on stdout and exits 0 (usage
// errors exit 2). Framework adapters wrap the run loop in
// `std::panic::catch_unwind` with a silenced panic hook so panics from the
// library-under-test translate into counterexamples rather than aborts.

use crabcheck::quickcheck as crabcheck_qc;
use hegel::{generators as hgen, Hegel, Settings as HegelSettings};
use nom::etna::{
    property_count_handles_zero_sized_output, property_float_parses_infinity_fully,
    property_multispace0_consumes_all_whitespace, PropertyResult,
};
use proptest::prelude::*;
use proptest::test_runner::{Config as ProptestConfig, TestCaseError};
use quickcheck::{QuickCheck, ResultStatus, TestResult};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Default, Clone, Copy)]
struct Metrics {
    inputs: u64,
    elapsed_us: u128,
}

impl Metrics {
    fn combine(self, other: Metrics) -> Metrics {
        Metrics {
            inputs: self.inputs + other.inputs,
            elapsed_us: self.elapsed_us + other.elapsed_us,
        }
    }
}

type Outcome = (Result<(), String>, Metrics);

fn to_err(r: PropertyResult) -> Result<(), String> {
    match r {
        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
        PropertyResult::Fail(m) => Err(m),
    }
}

const ALL_PROPERTIES: &[&str] = &[
    "Multispace0ConsumesAllWhitespace",
    "FloatParsesInfinityFully",
    "CountHandlesZeroSizedOutput",
];

fn run_all<F: FnMut(&str) -> Outcome>(mut f: F) -> Outcome {
    let mut total = Metrics::default();
    for p in ALL_PROPERTIES {
        let (r, m) = f(p);
        total = total.combine(m);
        if let Err(e) = r {
            return (Err(e), total);
        }
    }
    (Ok(()), total)
}

// ───────────── etna tool: replays frozen witness inputs. ─────────────
fn run_etna_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_etna_property);
    }
    let t0 = Instant::now();
    let result = match property {
        "Multispace0ConsumesAllWhitespace" => {
            to_err(property_multispace0_consumes_all_whitespace(vec![2u8]))
        }
        "FloatParsesInfinityFully" => to_err(property_float_parses_infinity_fully(vec![0u8])),
        "CountHandlesZeroSizedOutput" => to_err(property_count_handles_zero_sized_output(2u8)),
        _ => {
            return (
                Err(format!("Unknown property: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    (result, Metrics { inputs: 1, elapsed_us })
}

// ───────────── proptest ─────────────
fn bytes_strategy() -> BoxedStrategy<Vec<u8>> {
    prop::collection::vec(any::<u8>(), 0..16).boxed()
}

fn run_proptest_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_proptest_property);
    }
    let counter = Arc::new(AtomicU64::new(0));
    let t0 = Instant::now();
    let mut runner = proptest::test_runner::TestRunner::new(ProptestConfig::default());
    let result: Result<(), String> = match property {
        "Multispace0ConsumesAllWhitespace" => {
            let c = counter.clone();
            runner
                .run(&bytes_strategy(), move |v| {
                    c.fetch_add(1, Ordering::Relaxed);
                    match property_multispace0_consumes_all_whitespace(v) {
                        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
                        PropertyResult::Fail(m) => Err(TestCaseError::fail(m)),
                    }
                })
                .map_err(|e| e.to_string())
        }
        "FloatParsesInfinityFully" => {
            let c = counter.clone();
            runner
                .run(&bytes_strategy(), move |v| {
                    c.fetch_add(1, Ordering::Relaxed);
                    match property_float_parses_infinity_fully(v) {
                        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
                        PropertyResult::Fail(m) => Err(TestCaseError::fail(m)),
                    }
                })
                .map_err(|e| e.to_string())
        }
        "CountHandlesZeroSizedOutput" => {
            let c = counter.clone();
            runner
                .run(&any::<u8>(), move |v| {
                    c.fetch_add(1, Ordering::Relaxed);
                    match property_count_handles_zero_sized_output(v) {
                        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
                        PropertyResult::Fail(m) => Err(TestCaseError::fail(m)),
                    }
                })
                .map_err(|e| e.to_string())
        }
        _ => {
            return (
                Err(format!("Unknown property for proptest: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = counter.load(Ordering::Relaxed);
    (result, Metrics { inputs, elapsed_us })
}

// ───────────── quickcheck (fork with `etna` feature) ─────────────
//
// The fork requires `Display` on every property argument. `Vec<u8>` does not
// implement `Display`, so we take scalar seeds (u64 / u8) and expand them
// deterministically into the byte vectors the property functions expect.
static QC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn seed_to_bytes(seed: u64) -> Vec<u8> {
    let len = ((seed >> 60) as usize) % 9 + 1;
    let mut out = Vec::with_capacity(len);
    let mut s = seed;
    for _ in 0..len {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        out.push((s >> 33) as u8);
    }
    out
}

fn qc_multispace0_consumes_all_whitespace(seed: u64) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_multispace0_consumes_all_whitespace(seed_to_bytes(seed)) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_float_parses_infinity_fully(seed: u64) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_float_parses_infinity_fully(seed_to_bytes(seed)) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn qc_count_handles_zero_sized_output(seed: u8) -> TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_count_handles_zero_sized_output(seed) {
        PropertyResult::Pass => TestResult::passed(),
        PropertyResult::Discard => TestResult::discard(),
        PropertyResult::Fail(_) => TestResult::failed(),
    }
}

fn run_quickcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_quickcheck_property);
    }
    QC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let mut qc = QuickCheck::new().tests(200).max_tests(2000);
    let result = match property {
        "Multispace0ConsumesAllWhitespace" => {
            qc.quicktest(qc_multispace0_consumes_all_whitespace as fn(u64) -> TestResult)
        }
        "FloatParsesInfinityFully" => {
            qc.quicktest(qc_float_parses_infinity_fully as fn(u64) -> TestResult)
        }
        "CountHandlesZeroSizedOutput" => {
            qc.quicktest(qc_count_handles_zero_sized_output as fn(u8) -> TestResult)
        }
        _ => {
            return (
                Err(format!("Unknown property for quickcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = QC_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match result.status {
        ResultStatus::Finished => Ok(()),
        ResultStatus::Failed { arguments } => Err(format!(
            "quickcheck counterexample: ({})",
            arguments.join(" ")
        )),
        ResultStatus::Aborted { err } => Err(format!("quickcheck aborted: {err:?}")),
        ResultStatus::TimedOut => Err("quickcheck timed out".into()),
        ResultStatus::GaveUp => Err(format!(
            "quickcheck gave up: passed={}, discarded={}",
            result.n_tests_passed, result.n_tests_discarded
        )),
    };
    (status, metrics)
}

// ───────────── crabcheck ─────────────
static CC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn usize_to_u8(x: usize) -> u8 {
    // crabcheck's Arbitrary<usize> skews toward small values; a multiplicative
    // hash spreads small usizes across the full byte range so the property
    // doesn't always discard.
    let h = (x as u32).wrapping_mul(2654435761);
    (h >> 24) as u8
}

fn usize_vec_to_u8_vec(v: Vec<usize>) -> Vec<u8> {
    v.into_iter().map(usize_to_u8).collect()
}

fn cc_multispace0_consumes_all_whitespace(v: Vec<usize>) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_multispace0_consumes_all_whitespace(usize_vec_to_u8_vec(v)) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_float_parses_infinity_fully(v: Vec<usize>) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_float_parses_infinity_fully(usize_vec_to_u8_vec(v)) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_count_handles_zero_sized_output(v: usize) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_count_handles_zero_sized_output(usize_to_u8(v)) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn run_crabcheck_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_crabcheck_property);
    }
    CC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let result = match property {
        "Multispace0ConsumesAllWhitespace" => {
            crabcheck_qc::quickcheck(cc_multispace0_consumes_all_whitespace)
        }
        "FloatParsesInfinityFully" => crabcheck_qc::quickcheck(cc_float_parses_infinity_fully),
        "CountHandlesZeroSizedOutput" => {
            crabcheck_qc::quickcheck(cc_count_handles_zero_sized_output)
        }
        _ => {
            return (
                Err(format!("Unknown property for crabcheck: {property}")),
                Metrics::default(),
            )
        }
    };
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = CC_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match result.status {
        crabcheck_qc::ResultStatus::Finished => Ok(()),
        crabcheck_qc::ResultStatus::Failed { arguments } => Err(format!(
            "crabcheck counterexample: ({})",
            arguments.join(" ")
        )),
        crabcheck_qc::ResultStatus::TimedOut => Err("crabcheck timed out".into()),
        crabcheck_qc::ResultStatus::GaveUp => Err(format!(
            "crabcheck gave up: passed={}, discarded={}",
            result.passed, result.discarded
        )),
        crabcheck_qc::ResultStatus::Aborted { error } => {
            Err(format!("crabcheck aborted: {error}"))
        }
    };
    (status, metrics)
}

// ───────────── hegel (hegeltest 0.3.7) ─────────────
static HG_COUNTER: AtomicU64 = AtomicU64::new(0);

fn hegel_settings() -> HegelSettings {
    HegelSettings::new().test_cases(200).seed(Some(0x0C5F_A7E7))
}

fn run_hegel_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_hegel_property);
    }
    HG_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let settings = hegel_settings();
    let run_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match property {
        "Multispace0ConsumesAllWhitespace" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let v = tc.draw(hgen::vecs(hgen::integers::<u8>()).max_size(16));
                if let PropertyResult::Fail(m) = property_multispace0_consumes_all_whitespace(v) {
                    panic!("{m}");
                }
            })
            .settings(settings.clone())
            .run();
        }
        "FloatParsesInfinityFully" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let v = tc.draw(hgen::vecs(hgen::integers::<u8>()).max_size(16));
                if let PropertyResult::Fail(m) = property_float_parses_infinity_fully(v) {
                    panic!("{m}");
                }
            })
            .settings(settings.clone())
            .run();
        }
        "CountHandlesZeroSizedOutput" => {
            Hegel::new(|tc: hegel::TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let v = tc.draw(hgen::integers::<u8>());
                if let PropertyResult::Fail(m) = property_count_handles_zero_sized_output(v) {
                    panic!("{m}");
                }
            })
            .settings(settings.clone())
            .run();
        }
        _ => panic!("__unknown_property:{property}"),
    }));
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = HG_COUNTER.load(Ordering::Relaxed);
    let metrics = Metrics { inputs, elapsed_us };
    let status = match run_result {
        Ok(()) => Ok(()),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "hegel panicked with non-string payload".to_string()
            };
            if let Some(rest) = msg.strip_prefix("__unknown_property:") {
                return (
                    Err(format!("Unknown property for hegel: {rest}")),
                    Metrics::default(),
                );
            }
            Err(format!("hegel found counterexample: {msg}"))
        }
    };
    (status, metrics)
}

fn run(tool: &str, property: &str) -> Outcome {
    match tool {
        "etna" => run_etna_property(property),
        "proptest" => run_proptest_property(property),
        "quickcheck" => run_quickcheck_property(property),
        "crabcheck" => run_crabcheck_property(property),
        "hegel" => run_hegel_property(property),
        _ => (Err(format!("Unknown tool: {tool}")), Metrics::default()),
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit_json(
    tool: &str,
    property: &str,
    status: &str,
    metrics: Metrics,
    counterexample: Option<&str>,
    error: Option<&str>,
) {
    let cex = counterexample.map_or("null".to_string(), json_str);
    let err = error.map_or("null".to_string(), json_str);
    println!(
        "{{\"status\":{},\"tests\":{},\"discards\":0,\"time\":{},\"counterexample\":{},\"error\":{},\"tool\":{},\"property\":{}}}",
        json_str(status),
        metrics.inputs,
        json_str(&format!("{}us", metrics.elapsed_us)),
        cex,
        err,
        json_str(tool),
        json_str(property),
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property>", args[0]);
        eprintln!("Tools: etna | proptest | quickcheck | crabcheck | hegel");
        eprintln!(
            "Properties: Multispace0ConsumesAllWhitespace | FloatParsesInfinityFully | CountHandlesZeroSizedOutput | All"
        );
        std::process::exit(2);
    }
    let (tool, property) = (args[1].as_str(), args[2].as_str());

    // Silence library-under-test panic noise; frameworks catch panics
    // internally but the default hook still prints to stderr.
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(tool, property)));
    std::panic::set_hook(previous_hook);

    let (result, metrics) = match caught {
        Ok(outcome) => outcome,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "panic with non-string payload".to_string()
            };
            emit_json(
                tool,
                property,
                "aborted",
                Metrics::default(),
                None,
                Some(&format!("adapter panic: {msg}")),
            );
            return;
        }
    };

    match result {
        Ok(()) => emit_json(tool, property, "passed", metrics, None, None),
        Err(msg) => emit_json(tool, property, "failed", metrics, Some(&msg), None),
    }
}
