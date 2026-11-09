# Paged preparation and inherited-age review

Input47af6dc7d2f6d88fd54df54b97e537d7b608d122;
implementation2dd9990d22cdffa41442d9b168813e8c6598838d.
Controls, entry/exit, scope and RUST/PROVIDER L0/L1/L2:
[PAGED-EXECUTION](../../specs/PAGED-EXECUTION.md), retaining the full
[paged protocol](../../specs/PAGED-RETIREMENT.md). Targets: actual-state pure
conditional source planning, asset-side prepared plans/readback, append-only age
proofs and shared existing accounted witness construction. No new SQL/cache/event
engine, live roots, controller completion or deployment.

## Producer method and dispositions

One producer applies all six [.roles](../../.roles/ROLE.md) checklists. Owner of
each finding: Tig maintainer through the named lens. These are not independent
reviewers, human acceptance or deletion authority. Compare actual source/model
transitions, metadata effects, immutable framing and old executable behavior;
preserve the difference between prediction, acknowledgement and current protection.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| PE-01 Identity and Namespace | High | Prepared steps bind exact original page identities and separate canonical subset plans. Readback reconstructs original pages, checks source/predecessor-to-survivor lineage and unchanged objects, and recomputes each projection. Different-occurrence plan substitution and false no-survivor summaries refuse. Two independent Python framing cases pin new records. | A prepared digest cannot authenticate global claim/final-copy consent or actual progress. Independently current initial reconstruction remains mandatory. |
| PE-02 Placement Policy | High | Actual17-root fixtures settle all34 prepared pages after reopen, with every actual retirement receipt matching its prediction. An unselected alias remains protected after all selected history settles. Two namespaces use independent checkpoint sequences despite reordered runtime handles. | Durable global decision and original held/eligible provider copy set; no later expansion when protection disappears. Current/job/foreign/hard retention gates remain. |
| PE-03 Storage and Custody | High | Preparation stages only immutable metadata; actual journal state/slot/event/byte usage is cloned for pure simulation. No source mutation occurs before explicit subset apply in tests. Shared witness extraction preserves existing bulk operation/document identities and accounted capacity rules; fresh bulk CLI execution passes. | Admit global/page/action recovery capacity and durable intent before source effects; compose original provider births/grants and final live fences. No paged payload removal or physical reclaim is proved. |
| PE-04 Concurrency and Recovery | High | Actual peer admission while conditional plans are persisted invalidates the whole preparation without retiring original history. Duplicate bindings and occupied survivor slots refuse. Truncated/reordered/changed page records hold. A younger survivor capture cannot suppress a verified older inheritance; facts and parent/plan dependencies remain immutable. Historical inheritance after a later page retires its leaf does not recreate that claim. | Durable page completion must follow actual receipt/checkpoint and age acknowledgement, never simulation or cursor movement. Exact owned-command checks, cancellation, restore fencing and process-exit coverage for the new controller remain. |
| PE-05 Provider and Privacy | High | Existing Tree/NodeStore/MetadataStore ports retain bounded asset-side indexes and separate plans. One outer metadata/time meter spans nested reconstruction, paging, simulation and readback. Small records remain64KiB. Aggregate budget refusal preserves source state. Source/docs cap rises from6 to8MiB because ordinary code exceeded remaining headroom; runtime limits and128KiB per-file rule stay unchanged. | Actual providers/large files, independent custody, current authority/host trust, dependency-aware expiry and journal continuation require separate qualification. No private roots or payloads are admitted by preparation. |
| PE-06 Git and Adoption | Medium | Existing age event/document encoding and enrollment identities remain unchanged. A pinned pre-alternate CLI reads the younger primary fact and its capture retry preserves stronger alternate evidence without clock-journal changes. Current readers still select the proved older fact afterward. Six independent age framing cases include the new domain-separated inheritance key. | Native/agent/UI/Action composition, actual writer/authority/time adoption, hosted exact-head checks and independent/human acceptance remain open. Preparation has no new mutation CLI. |

Review strengthened readback after initial tests: matching original page bindings
alone did not prove that its attached plan belonged to that occurrence. Final
checks now validate the original-to-survivor chain and recompute projections;
regressions reject a different occurrence's same-path plan and a false survivor
summary. Old-age qualification initially used an incomplete clock profile and
misread the capture response wrapper; the corrected actual executable test passes.
Those failed fixture attempts are not passing evidence.

## Verification

- Final17 paged entries, all15 age entries including the pinned older executable,
  and strict store all-target Clippy pass in97302 (terminal zero).
- Initial15 subset entries pass, with the pre-pair source artifact qualifier
  ignored;19 bulk entries pass with two explicit artifact entries initially ignored.
- Freshly rebuilt CLI then passes both explicit bulk inspection/execution entries
  and ordinary pending-cancellation qualification (83737 terminal zero).
- Default locked workspace tests including the500,000-binding metadata fixture,
  standalone Git check and strict default workspace/all-target Clippy pass (9715
  terminal zero). Test compilation preceded final readback tightening; the final
  focused run above qualifies that tightening and the added namespace regression.
- All-feature workspace/second500k/strict workspace Clippy chain78449 terminates
  zero. Its tests also compiled before final readback tightening; final17-entry
  all-feature paged focus passes afterward (70165 terminal zero). Hosted
  exact-head qualification remains required.
- Fmt, portable validator/reviewed hashes and diff checks pass before the
  implementation commit; repeat L0 after this review record is added.

Commands use explicit external build/temp roots, separate targets and bounded
test threads. Broad logs stay outside source. The old executable is preserved in
its pinned target; no new build overwrites it.

```text
cargo test -p tig-store --lib --locked final_retirement::paged
cargo test -p tig-store --lib --locked source_claims::age::tests -- --include-ignored
cargo test -p tig-store --lib --locked source_claims::subset::tests
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo check -p tig-git --no-default-features --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo build -p tig-store --bin tig-storage --locked
cargo test -p tig-store --lib --locked explicit_cli_bulk -- --ignored
cargo test -p tig-store --lib --locked explicit_cli_pending_cancellation_qualification -- --ignored
pwsh -NoProfile -File tools/validate.ps1
git diff --check
```

## Synthetic validation

Parts of VAL-002/003/004 are rehearsed: the original whole historical selection
survives durable metadata reopening, all prepared pages can settle through actual
source commands, unselected shared content remains protected and age provenance
survives later pages and another observer. Two-namespace fixtures prove that each
actual local journal checkpoint matches its frozen prediction. These are direct
subset applications for qualification, not a durable paged operation/controller.
No final-copy loss, real F/D/M workflow, full user validation or human acceptance
follows from them. All full VALs remain open.

## Trace and disposition

WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009 ->
SPEC-002/003/004/005/006/008/009/013/014,
IF-002/004/005/006/008/010/012/013, PKG-002/003/004/006/008/009 ->
PAGED-EXECUTION -> subset::plan_pages, paged::preparation,
age alternate inheritance and bulk::witness shared construction -> EVID-108.

Disposition: pass_with_risk for these bounded prerequisites. NB-07 operational
block retirement remains open. Durable exact global/page intent, authenticated
separate claim/final-copy authority, actual completion/age witnesses, original
provider copy identity, final fences, cancellation/status and crash/restore tests
still need composition and proof. Full native/UI/agent/Action, provider/large-file,
authority/time/writer/lifetime/metadata-expiry and human acceptance stay required.
