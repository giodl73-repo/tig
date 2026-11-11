//! Copied only into a pinned, timing-instrumented legacy checkout by the probe.
use serde_json::{Value, json};
use std::{env, fs, path::PathBuf};
use tig_store::{
    Key,
    ledger::{CapacityLedger, Checkpoint, JournalRef, LedgerError},
};

const PIN: &str = "c833add85f9a9c79d3387a78e469bc13a2570403";

fn paths() -> (PathBuf, PathBuf) {
    let evidence = PathBuf::from(env::var("TIG_ACTIVATION_CASE").expect("explicit fixture case"));
    let owner: Value =
        serde_json::from_slice(&fs::read(evidence.join("owner.json")).unwrap()).unwrap();
    assert_eq!(owner["schema"], "tig.activation-legacy-case.design-1");
    assert_eq!(owner["legacy_source"], PIN);
    let root = evidence.join("source");
    (evidence, root)
}

fn key(s: &str) -> Key {
    Key::parse(s).unwrap()
}

#[test]
fn prepare_legacy_source() {
    let (evidence, root) = paths();
    assert!(!root.exists(), "fixture never reuses an existing source");
    let mut ledger =
        CapacityLedger::create(&root, key("synthetic-activation"), key("epoch"), 0).unwrap();
    let probe = ledger.begin_probe(key("probe")).unwrap();
    ledger
        .observe(key("sample"), probe.reference(), 1_000_000)
        .unwrap();
    let job = ledger
        .reserve(key("active-job"), key("owner"), 1000)
        .unwrap();
    let done = ledger
        .begin_allocation(key("completed"), job.reference(), key("event-done"), 100)
        .unwrap();
    ledger
        .complete_allocation(key("completion"), done.reference(), 25)
        .unwrap();
    ledger
        .begin_allocation(key("inflight"), job.reference(), key("event-pending"), 200)
        .unwrap();
    let record = json!({
        "minimum":ledger.checkpoint(), "job":job.reference(),
        "config":serde_json::from_slice::<Value>(&fs::read(root.join("domain.json")).unwrap()).unwrap(),
        "sqlite_version":rusqlite::version()
    });
    fs::write(
        evidence.join("prepared-source.json"),
        serde_json::to_vec(&record).unwrap(),
    )
    .unwrap();
}

#[test]
fn delayed_legacy_writer() {
    let (evidence, root) = paths();
    let prepared: Value =
        serde_json::from_slice(&fs::read(evidence.join("prepared-source.json")).unwrap()).unwrap();
    let config: Value =
        serde_json::from_slice(&fs::read(root.join("domain.json")).unwrap()).unwrap();
    assert_eq!(config, prepared["config"]);
    assert_eq!(config["domain"], "synthetic-activation");
    let minimum: Checkpoint = serde_json::from_value(prepared["minimum"].clone()).unwrap();
    let job: JournalRef = serde_json::from_value(prepared["job"].clone()).unwrap();
    let mut ledger = CapacityLedger::open(&root, &minimum).unwrap();
    // The timing-only baseline patch pauses inside this exact old call after guard.
    let failure = ledger.invalidate(key("after-guard-write")).unwrap_err();
    let LedgerError::Sql(message) = failure else {
        panic!("wrong refusal: {failure:?}")
    };
    assert!(message.contains("tig:sealed-source"), "{message}");
    assert_eq!(ledger.checkpoint(), minimum);
    assert!(matches!(
        ledger.accounting(),
        Err(LedgerError::NeedsRecovery)
    ));
    let mut reopened = CapacityLedger::open(&root, &minimum).unwrap();
    let summary = reopened.accounting().unwrap();
    assert_eq!(summary.reserved_remaining, 975);
    assert_eq!(summary.allocated_after_watermark, 25);
    assert!(matches!(
        reopened.finish(key("cannot-finish"), &job),
        Err(LedgerError::Capacity(
            tig_core::capacity::CapacityError::InFlight
        ))
    ));
    assert!(matches!(
        reopened.reserve(key("no-fresh-admission"), key("owner"), 1),
        Err(LedgerError::NeedsProbe)
    ));
    assert_eq!(reopened.checkpoint(), minimum);
    fs::write(
        evidence.join("writer-result.json"),
        serde_json::to_vec(&json!({
            "schema":"tig.instrumented-legacy-writer.design-1", "legacy_source":PIN,
            "timing_instrumented":true, "sqlite_version":rusqlite::version(),
            "sql_guard_refused":true, "ordinary_prefix_unchanged":true,
            "inflight_preserved":true, "fresh_probe_required":true,
            "activation_implemented":false
        }))
        .unwrap(),
    )
    .unwrap();
}
