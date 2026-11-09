# Explicit local clock review

Input4aeebf8ad136c69a7cf3ad399807e7d461ca82f8;
implementation98021662b2cb85d21804c4e2a8fee6bed229ca53;
portable fixture pin/spec clarification43ad0f4.
Controls, scope, entry/exit and RUST/PROVIDER L0/L1/L2:
[LOCAL-CLOCK](../../specs/LOCAL-CLOCK.md).
WP-003/004/006/007/008/009/015; REQ-001/003/004/005/006/007/009;
SPEC-003/004/005/008/009/014; IF-003/004/005/006/012;
PKG-002/003/008/009. Targets: clock adapter, existing capacity ledger marker/
command replay, final-provider/reacquisition/bulk window checks, vectors and
expanded pinned older-writer qualifier. Reviewed hashes: foundation-targets.json.

## Method and six producer lenses

One producer applies all six [.roles checklists](../../.roles/ROLE.md), without
delegation or independent/human approval. Owner of every finding: Tig maintainer
using the named lens. The proposed local-clock implementation does not establish
user adoption of OS time or enroll live roots.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| LC-01 Identity and Namespace | High | Enrollment binds immutable policy and actual clock-enabled ledger genesis. Legacy design-1 bytes remain; a fresh design-2 marker refuses old readers. Wrong enrollment refuses without sampling. Independent vectors pin five canonical identities. | Live authority/epoch handoff and independently current checkpoint admission. |
| LC-02 Placement Policy | High | The whole earliest/latest interval must fit the consent window and prior lower bound. Provider release/unlink, new reacquisition birth and initial bulk admission use the same check. Expired intervals hold real temporary bytes; completed historical replay grants no fresh time. | Native age/category events and separately admitted final-deletion policy. |
| LC-03 Storage and Custody | High | Clock status has no current-time authority. A time observation grants neither custody nor deletion permission; existing live membership/provider/grant/birth guards remain mandatory. Actual enrolled OS samples drive a synthetic final-retirement fixture. | Large-file/real-provider recovery, custody and measured physical reclaim. |
| LC-04 Concurrency and Recovery | High | Persist the high-water floor before return; new invocations must advance it. Peer races, stale prefixes, full 16,384-command exhaustion and three actual abrupt exits exercise the existing ledger. Checked cross-sample monotonic ordering rejects regression; out-of-format interval bounds refuse before append. | Whole-system unseen restore, power-loss guarantees, operational continuation and authority migration. |
| LC-05 Provider and Privacy | High | Explicit enrollment admits the OS time boundary; no NTP discovery, fallback, daemon or private binding enters portable records. Sample/commit age, drift and invocation limits fail closed. Constructor/status grant no fresh observation. | Host accuracy/trust qualification; cooperative sampling cannot guarantee unlink completion before a deadline under an arbitrary OS pause. |
| LC-06 Git and Adoption | Medium | No mutation CLI, Action permission, live migration or deployment is enabled. Existing pinned pre-final executable accepts its legacy control and refuses the clock-enabled ledger in addition to final/roster profiles. | Full operator/agent/UI mutation integration, exact-head hosted qualification and intended-use acceptance. |

## Verification

Seven clock adapter entries and seven ledger-clock entries include one child
helper. Seventeen clock-filter entries pass, overlapping the native provider and
older clock-named regressions; these are not disjoint totals. Sixteen provider
entries and eighteen active bulk entries pass. Three actual clock-journal child
exits distinguish validated, written and committed boundaries. The full command
budget holds without resetting the floor, and a restored known prefix refuses an
independently retained newer minimum. Five Python/json/hashlib framing cases pin
legacy/new markers, policy, enrollment and event; framing is not time authority.

Negative sampling covers wall rollback, in-sample and cross-sample monotonic
regression, stale commit, excessive drift, session expiry, stale peer observation
and canonical-number overflow. Failed post-commit validation may conservatively
advance the persisted floor but returns no usable time. A failed invocation does
not silently heal; reopening preserves its floor. Capacity invalidation and
working-set retirement do not reset clock history.

The first provider test incorrectly expected ordinary retention verification to
accept a pending final intent. It now checks the exact encoded active grants
before release and their absence after release; no product guard was weakened.
Two additional adversarial cases found missing between-sample monotonic ordering
and pre-append canonical bounds checks. Both are fixed in9802166; their final
regression and strict all-feature/all-target Clippy pass. The vector's portable
outer LF bytes and manifest pin were corrected in43ad0f4 without changing its
canonical cases or runtime code.

Full default workspace and standalone Git checks pass. All-feature workspace and
fresh artifact qualification remain pending until terminal results are observed.

```text
cargo fmt --all -- --check
cargo test -p tig-store --lib clock --locked
cargo test -p tig-store --lib final_retirement::provider --locked
cargo test -p tig-store --lib final_retirement::bulk --locked
cargo test --workspace --locked
cargo build --workspace --all-features --bins --locked
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo check -p tig-git --no-default-features --locked
cargo test -p tig-store --lib --locked retirement::tests::explicit_cli_pending_cancellation_qualification -- --ignored --exact
cargo test -p tig-store --lib --locked final_retirement::bulk::tests::explicit_cli_bulk_inspection_qualification -- --ignored --exact
cargo test -p tig-store --test retirement_cli --locked pre_final_writer_refuses_new_manifest_and_accepts_legacy_control -- --ignored --exact
```

TIG_RETIREMENT_CLI/TIG_BULK_CLI name the fresh external all-feature artifact;
TIG_PRE_FINAL_CLI names pinned273e639. Ordinary ignored output is not explicit
artifact qualification. Source pre-pair artifact, frontend and restore evidence
is historical unless rerun. External target/temp roots contain all build/runtime
fixtures; no media, private roots or operational journals are added to Git.

## Synthetic validation, trace and residual scope

EVID-103 contributes bounded VAL-002/003/004 rehearsal: deletion and reacquisition
can consume explicit persisted native time bounds instead of request-supplied
timestamps. Expired/uncertain intervals preserve bytes and source obligations;
historical status remains separate from current permission. This is a bounded
library prerequisite, not full F/D/M workflow or native age selection acceptance.

Trace: controls above -> LOCAL-CLOCK.md -> clock::LocalClock/Policy/Enrollment,
CapacityLedger clock marker/command/high-water replay, Clock::bounds and
Window::check_clock -> adapter/ledger/provider/bulk and pinned artifact cases ->
EVID-103. Implementation, verification, synthetic validation and this producer
review close separately. Disposition: pass_with_risk for the bounded synthetic
scope only; broad/hosted qualification status must be derived from actual results.

Explicit OS accuracy/trust, independently current minima/authority, globally
unseen whole-system restore, arbitrary pause semantics, operational continuation,
live provider/large-file qualification, native selectors, mutation surfaces,
metadata retirement and human acceptance remain open. No live store, cache
initialization, transfer, runner, hook, source merge or deployment occurred.
The full PRODUCT_PLAN goal remains active and incomplete.
