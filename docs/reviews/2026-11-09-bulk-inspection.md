# Bulk operator inspection review

Input36fc91bdd4161917e7ab5c1e9f2a078041c7033d;
implementation29e87e74ad50134e53f915a90d17e636e58ae60b.
Controls, entry/exit and RUST/PROVIDER L0/L1/L2:
[BULK-OPERATOR](../../specs/BULK-OPERATOR.md).
WP-003/004/007/008/009/015; REQ-001/003/004/005/007/009;
SPEC-003/004/005/008/009/014; IF-003/004/005/006/008/012;
PKG-002/003/008/009.
Targets: bulk::Inspection/shared control/report, tig-storage bulk adapter and
discovery, ledger checkpoint identity accessor, explicit CLI qualifier, workflow
invocation and operator guide. Reviewed hashes: foundation-targets.json.

## Method and six producer lenses

One producer applies all six [.roles checklists](../../.roles/ROLE.md), without
delegation or independent/human acceptance. Owner of all findings: Tig maintainer
using the named lens. This is operator inspection only. Full apply/resume/cancel,
native selection, clock/current authority and lifetime requirements stay open.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| BI-01 Identity and Namespace | High | Exact block request reproduces the library's canonical plan. Private profile supplies actual roster/minima, context, metadata identity, store/capacity/job bindings; requests cannot choose physical roots or substitute capacity authority. Namespace floors must equal their admitted roster floors. | Native version/category/age identities and current authority adoption. |
| BI-02 Placement Policy | Medium | Missing copy preview keeps every selected subject held; unknown source closure refuses planning. Status reports unavailable sources/actions as holds, without inferring zero protection or deletion. Existing shared/alias/foreign-job qualification remains. | Native grouping/filtering and operator action integration; no full preview UX acceptance. |
| BI-03 Storage and Custody | High | Inspection has no MetadataStore/Clock/cookie and no mutation methods. Actual CLI preserves source/ledger checkpoints verified through newly reopened handles, plus exact provider/witness file snapshots. Status distinguishes new removal zero, historical effects, original allocations and metadata peak. | Real providers/large files, native custody and measured volume reclaim. |
| BI-04 Concurrency and Recovery | High | Writer-free status works with source metadata unavailable across fresh, interrupted intent and completed batches. Changed private metadata identity refuses existing intent/completion binding. Missing witness errors instead of inventing not-started; status never repairs or admits an operation. Execution delegates to the same control/report logic. | Full recovery/authority restoration, mutation cancellation interface and operational lifetime. |
| BI-05 Provider and Privacy | High | Existing roots only; available provider identity/contract mismatch refuses. Input/output bounds, strict fields and explicit copy authority mappings hold. Errors use existing flat static envelopes; JSON-escaped private root prefixes are absent from reports. Missing roots are never created. | Live private enrollment, unmanaged-client boundaries and full provider qualification. |
| BI-06 Git and Adoption | Medium | Discovery advertises inspection only and rejects mutation verbs. No source/provider record encoding changes; ledger accessor is observation only. CI explicitly runs the ignored qualifier against its fresh built artifact. Guide keeps profiles/reports outside Git. | Actual deployment, bulk execution/agent/UI/native adapters and human intended-use acceptance. |

## Verification

Seventeen focused bulk entries pass; the new ignored CLI qualifier is executed
separately rather than counted from ordinary ignored output. A fresh tig-storage
artifact reproduces the exact plan, previews missing copies, computes metadata
peak and reports fresh/intent/completed states with source metadata unavailable.
It exercises unavailable source/provider/witness, wrong context/capacity/provider,
unknown profile fields, oversized input and all mutation-verb refusals. Newly
reopened source/capacity handles prove no appended commands, and exact synthetic
provider/witness file snapshots prove no new object/claim/control records.

The library-only case actually closes and moves the metadata directory before
constructing Inspection, without any Clock. Historical binding mismatch refuses;
source/provider journals and original metadata path remain unchanged. Strict
all-feature/all-target Clippy passes after the shared-report refactor. Source and
workflow use explicit external target/temp roots. Implementation L0 and fmt pass.

Full locked default and all-feature workspace suites pass in separate external
targets, including both 500,000-binding metadata fixtures. Standalone Git without
optional features passes. The fresh all-feature CLI passes both explicit bulk
inspection and ordinary-retirement pending-cancellation qualifiers. The pinned
pre-change writer qualifier accepts the legacy control and refuses final-capable/
roster bindings. Final fmt and strict all-feature/all-target Clippy pass at29e87e7;
no implementation changed after that commit. Source pre-pair artifact, frontend/
restore, native/provider/time evidence remain historical unless explicitly rerun.
The hosted workflow includes the new explicit bulk qualifier on both platforms;
exact-head hosted success remains separate and pending at local closeout.

```text
cargo fmt --all -- --check
cargo test -p tig-store --lib final_retirement::bulk --locked
cargo test --workspace --locked
cargo build --workspace --all-features --bins --locked
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo check -p tig-git --no-default-features --locked
cargo test -p tig-store --lib --locked final_retirement::bulk::tests::explicit_cli_bulk_inspection_qualification -- --ignored --exact
cargo test -p tig-store --lib --locked retirement::tests::explicit_cli_pending_cancellation_qualification -- --ignored --exact
cargo test -p tig-store --test retirement_cli --locked pre_final_writer_refuses_new_manifest_and_accepts_legacy_control -- --ignored --exact
```

TIG_BULK_CLI and TIG_RETIREMENT_CLI name the fresh external all-feature artifact;
TIG_PRE_FINAL_CLI names pinned273e639's external artifact. Ignored entries in the
ordinary suite are not these passing explicit qualification invocations.

An initial CLI test expected a nested error code; the existing interface uses a
flat code field. The assertion was corrected without changing product errors.
The final preservation assertions reopen journals after the subprocess; comparing
only the original handles' cached checkpoints would have been insufficient.

## Synthetic validation and residual scope

This contributes bounded VAL-002/003/004 rehearsal: an operator can review exact
eligible/held content and inspect interrupted history without opening a metadata
writer or weakening authority. It does not authorize or perform final deletion,
source settlement or cancellation through the CLI. The existing library remains
the separately qualified mutation path, requiring independently admitted Clock.

The first-clock preference was requested asynchronously; no answer is recorded
at this slice. Inspection does not depend on that decision and adds no fallback
or native clock assumption. Full native selectors, execution permissions, current
clock/authority, actual providers/large files, journal/metadata lifetime and human
acceptance remain required. No live store, runner, hook, upgrade or deployment.
Disposition: pass_with_risk for bounded synthetic inspection; exact-head hosted
qualification remains pending. The full PRODUCT_PLAN goal is incomplete.
