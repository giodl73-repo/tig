# Historical age witness review

Input0dbc41f318b5acffacb3ed6562df839ae1ffe9d9;
implementation1a344fcd27ba5114b1853eb337d59e7abe3ade1e.
Scope/controls/entry/exit and RUST/PROVIDER/GIT L0/L1/L2:
[AGE-WITNESSES](../../specs/AGE-WITNESSES.md).
Targets: generic source age registry, historical subset proof helper, explicit
private operator CLI, separate optional native age-aware candidate selection,
synthetic process/clock/source/metadata fixtures and independent framing vectors.
Reviewed file hashes: foundation-targets.json. No live roots or payload operations
are authorized by this review.

## Method and producer findings

One producer applies all six [.roles](../../.roles/ROLE.md) checklists. These are
technical lenses, not delegated people, independent review or human acceptance.
Owner of every finding: Tig maintainer using the named lens. Read the safety/model
contracts and existing native/bulk/source/clock boundaries; compare new behavior
with exact-source tests. Do not extend these results to a complete native writer,
trusted global clock, current authority or whole-product deletion guarantee.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| AW-01 Identity and Namespace | High | Enrollment pins registry/metadata/clock/observer scope and authority/epoch. Each observation binds exact namespace/source/predecessor/root/context/owner/class and source checkpoint. Reusing identical bytes creates a separate occurrence. Independent framing covers enrollment, occurrence, identity and both record origins. | Admit actual source writers and preserve identities/checkpoint ancestry through lifetime continuation; no inferred global authority. |
| AW-02 Placement Policy | High | Age is a conservative observation upper bound. Whole intervals and checked canonical arithmetic decide age; unknown, recent and missing occurrences remain distinct. Current/recent/shared use stays independently protected by the existing final controller. Partial subsets inherit age only through exact executed receipts and recomputed unchanged projection; two-generation inheritance is exercised. | NB-07 global selection/consent and separately authorized historical-claim settlement; age alone does not make repeated bounded final pages converge. |
| AW-03 Storage and Custody | High | Registry entries never release claims or grant removal. Existing source/metadata ports retain immutable facts; independently finished-provider record/root readback precedes adoption. Root cleanup proves every old binding survives before retiring metadata history. | Final live protection/custody/consent remains mandatory. Metadata GC must retain record/parent/plan/source dependencies or hold expiry; real providers/large files and measured volume reclaim stay open. |
| AW-04 Concurrency and Recovery | High | Same-occurrence retries preserve acknowledged dates with no clock sample. Actual exits at six capture boundaries recover original acknowledged evidence or a later conservative unacknowledged retry. Interleaved and simultaneous peers converge without overwriting the winner. Source-prefix restore, unavailable records, full registry and phase-spanning read/time exhaustion refuse. Review found per-phase counter renewal; implementation now shares one operation meter across cleanup, writes, readback, source tree admission and status/lineage. | Current authority/minimum distribution, globally unseen restoration, journal continuation and full operational lifetime remain unqualified. Exhaustion may leave honest partial metadata effects; no automatic reset/sweep. |
| AW-05 Provider and Privacy | High | Explicit private existing-root profiles; mutations default independently disabled. Native clock is enrolled and lazily opened only if sampled, so acknowledged retries/unknown assessments need no clock IO. Actual CLI denies timestamps and disabled mutations before root IO; status is writer/time-free. No provider paths or authored private text are exported by the tested flows. | Qualify actual OS/registry writer custody and native time trust. A document/hash or owner string does not authenticate provenance; admitted generic APIs remain trusted substrate. |
| AW-06 Git and Adoption | Medium | Optional select-aged verifies the existing native header/path/source proof and same exact age occurrence; unknown/recent age yields null selection, eligible age emits an existing bulk candidate. Actual CLI works after Git is detached and leaves source/registry/metadata checkpoints unchanged. Plain select remains age-free; low-level age infrastructure has no typed-asset dependency. | Full native lifecycle/semantic/writer adoption, UI/Action, independently admitted final consent, hosted exact-head checks and human acceptance. |

## Verification

Twelve focused library entries pass, including one subprocess helper and one
independent framing entry with five Python/json/hashlib cases. The parent process
actually exits six children at Observed, Sampled, MetadataFinished, RegistryAdopted,
BeforeRetire and Retired. Acknowledged retries return original evidence without
sampling a clock; pre-adoption retries can only observe later. Registry cleanup
retains all facts. Separate peer handles/threads exercise optimistic contention.

Three actual age CLI integration entries pass over owned synthetic existing
metadata/source/clock roots: independently disabled permissions before root IO,
timestamp rejection, native-clock capture, metadata/read-only status, time-offline
acknowledged replay, unknown-age assessment without clock IO, executed subset
inheritance and idempotent retry. Five existing storage CLI entries also pass
after discovery exposes the new age interface.

All nine native-retirement integration entries pass; one new age-aware CLI case
uses actual LocalClock observation, holds unknown/recent candidates, waits for its
conservative interval and emits only the exact four-binding asset selection.
Git is actually detached before eligible selection. Header/timestamp injection
refuses; no source/registry/metadata checkpoint advances. Existing native bulk
fixtures remain regression evidence for their prior bounded scope.

Final focused/native/strict workspace all-feature/all-target Clippy chain24427
terminated zero. The strengthened identity-helper framing assertion and strict
Clippy chain53407 terminated zero. Final age/storage CLI and strict Clippy pass
after the discovery-only addition. Fmt, validator and diff checks pass before the
implementation commit. No runtime changes follow implementation1a344fc.

Earlier default/all-feature workspace chains64389/1159 terminated zero, including
both500k fixtures, but compiled the pre-aggregate-meter implementation. They are
not final-runtime broad qualification. Final default chain68634 (includes final
meter, predates the separately tested discovery-only addition) terminated zero,
including its500k fixture and standalone Git check. Exact-runtime all-feature/
Clippy chain87689 remains pending. External logs and current process
handles, not this paragraph, determine their eventual result. Fresh ordinary CLI
cancellation, bulk inspection/execution and pinned pre-final older-writer refusal
with accepted legacy control pass (chain89964 terminal zero). Historical
frontend/pre-pair/other-consumer evidence is not newly rerun by these tests.

Commands (external build/temp roots and finite test threads required):

```text
cargo test -p tig-store --lib --locked source_claims::age::tests
cargo test -p tig-store --test age_cli --test storage_cli --locked
cargo test -p tig-git --features schema --test native_source --locked retirement_cases
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo check -p tig-git --no-default-features --locked
pwsh -NoProfile -File tools/validate.ps1
git diff --check
```

## Synthetic validation, trace and disposition

This rehearses parts of VAL-002/003/004: exact aged historical selection can retain
meaning across retry and partial owned settlement without guessing timestamps or
claiming final authority. Complete intended-use validations stay open. Trace:
WP-003/004/006/007/008/009/010/013/015;
REQ-001/002/003/004/005/006/007/009/010 ->
SPEC-002/003/004/005/008/009/010/013/014,
IF-002/004/005/006/008/010/012/013,
PKG-002/003/004/006/008/009 -> AGE-WITNESSES -> source_claims::age,
subset::historical_survivor, tig-storage age, tig-retirement select-aged -> EVID-106.

Disposition: pass_with_risk for the bounded mechanism under independently admitted
private writers/clock/minima; final all-feature broad and hosted qualification remain
pending. No full requirement, VAL, deployment or independent/human approval closes.
NB-07 paging, live native/writer adoption, current authority/host trust/unseen
restore, UI/Action, real providers/large files, journal continuation, eventual
metadata retirement and full user acceptance remain required. Full goal active.
