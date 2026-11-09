# Explicit bulk execution review

Input6f8fdaa7897aa0b1c47d477348c6ee37ee921533;
implementation6b9016ef90d2add364c07bd6b2d30e780d314bd2.
Controls, scope, entry/exit and RUST/PROVIDER L0/L1/L2:
[BULK-EXECUTION](../../specs/BULK-EXECUTION.md).
WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014;
IF-002/003/004/005/006/008/012/013; PKG-002/003/004/007/008/009.
Targets: separate CLI execution profile, shared private runtime, metadata/time-free
Cancellation capability, existing inspection discovery, fresh CLI qualification,
workflow invocation and operator guide. Reviewed hashes: foundation-targets.json.

## Method and six producer lenses

One producer applies all six [.roles checklists](../../.roles/ROLE.md), without
delegation or independent/human approval. Owner of every finding: Tig maintainer
using the named lens. Exact private profile admission is an application/operator
trust boundary, not cryptographic proof or a UI approval inferred from hashing.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| BE-01 Identity and Namespace | High | A portable plan cannot approve itself. A distinct strict execution profile compares independently admitted plan digest and enrollment before provider/metadata/clock IO. Existing exact roots, roster, context, jobs and minima remain private. Wrong consent/clock/context and unknown fields refuse with reopened checkpoint/file preservation. | Actual admission service/profile protection, authority migration and native exact selectors. |
| BE-02 Placement Policy | High | Apply/resume call the existing bounded controller, preserving held/shared members and whole-window checks. An expired native window refuses source settlement; transient use refuses before settlement or holds provider actions after it. Permissions default disabled, and cancel is separate. | Native category/supersession events, UI/Action integration and full intended-use policy acceptance. |
| BE-03 Storage and Custody | High | Actual fresh CLI removes three eligible temporary bytes while shared/held siblings remain byte-exact. Completed replay appends no source/capacity/clock commands. Logical deletion and historical outcomes retain existing report meanings; no volume reclaim or universal erasure claim is added. | Large-file/real-provider custody, failure-domain qualification and measured physical space. |
| BE-04 Concurrency and Recovery | High | New Cancellation reuses the same internal runtime with no source writer/time capability; expired consent and offline metadata cannot trap cancellation. Nineteen active bulk entries cover old child-exit/race/retry paths plus metadata-offline cancellation at intent, settled, partially removed and completed boundaries. Actual CLI resumes interrupted subsets and cancels partial effects without restoring historical claims. | Whole-system restore/current authority and journal continuation; full operational cancellation debt handling. |
| BE-05 Provider and Privacy | High | Apply uses only an explicitly enrolled existing LocalClock. Cancellation opens no clock/metadata root. Preflight denials leave actual reopened journals and provider records unchanged. Errors remain static and private paths are absent; missing roots are not initialized. | Live OS trust and independent current minima, unmanaged clients, real provider/large-file acceptance. |
| BE-06 Git and Adoption | Medium | The old inspection profile and operations stay observational. Separate execute discovery advertises default-disabled permissions and does not create approvals. CI explicitly runs the new ignored qualifier against a fresh artifact. No runner, hook, migration, cache or runtime is deployed. | Exact-head hosted qualification, full operator UI/Action and human acceptance. |

## Verification

Nineteen active focused bulk entries pass; two artifact qualifiers are ignored by
ordinary selection and are not counted as executed there. The new metadata-offline
library case closes and moves the actual source metadata store at intent, settled,
partially removed and completed boundaries, then cancels without a metadata or
clock handle. Source checkpoints stay unchanged and shared/held bytes read back
exactly. Existing ten actual abrupt exits, batch-scoped process peer exclusion,
unrelated progress and independent framing cases remain in the passing group.

The explicitly executed fresh CLI qualifier passes actual enrolled OS-clock
deletion, completed no-op replay with reopened source/capacity/clock checkpoints,
interrupted source settlement resume, and cancellation after intent/settlement/
one removal while metadata and clock are unavailable. Expired native consent
cannot settle history, but separately enabled cancellation succeeds. New transient
use is challenged both before and after source settlement; all bytes survive.
Request-side synthetic clock is set to zero before actual apply, demonstrating
that the subprocess uses the independently enrolled native clock.

Preflight negatives include omitted/empty permissions, unapproved plan, missing/
wrong clock, unavailable clock root, missing metadata, wrong context and unknown
fields, independent cancel/resume permission denial and replaced provider roots.
Actual newly reopened checkpoints and exact provider/witness files remain
unchanged across these denials. Unknown roots remain absent and error JSON is
redacted. Discovery truthfully reports cancellation needs neither clock nor
metadata writer. Initial test compilation incorrectly compared ObjectRead to a
byte slice; tests now consume the guarded reader and compare actual bytes.
An initial stale-use test expected a report before settlement, where the existing
controller correctly returns stale_basis. It now asserts that error and separately
tests post-settlement structured holds; no product guard was weakened.

The private Runtime refactor preserves existing Execution public fields/API and
record encodings. Its metadata writer is optional internally, required explicitly
for non-cancelling paths. Cancellation always enters the cancelling path with
no writer and a rejecting time port. Inspection has no new mutation method.
Final strict all-feature/all-target workspace Clippy passes after removing
needless borrows; fmt and portable L0/diff pass. Full locked default workspace,
its500k metadata fixture and standalone Git pass (chain3144 terminated zero).
The expanded fresh execution CLI qualifier passes after the additional independent
permission/replaced-root negatives (chain70528 terminated zero). Full locked
all-feature workspace and its500k metadata fixture pass. Final artifact chain40830
terminated zero: explicit ordinary cancellation, bulk inspection, expanded bulk
execution and pinned older-writer refusal with accepted legacy control all pass.
Both broad suites cover the final runtime; final Clippy and explicit execution
qualifiers also cover the later negative test assertions. No runtime changed
after6b9016e. PR #35 is draft against #34 pending exact-head hosted qualification.

```text
cargo fmt --all -- --check
cargo test -p tig-store --lib --locked final_retirement::bulk
cargo build -p tig-store --bin tig-storage --locked
cargo test -p tig-store --lib --locked final_retirement::bulk::tests::operator::explicit_cli_bulk_execution_qualification -- --ignored --exact
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo check -p tig-git --no-default-features --locked
cargo build --workspace --all-features --bins --locked
cargo test --workspace --all-features --locked
cargo test -p tig-store --lib --locked retirement::tests::explicit_cli_pending_cancellation_qualification -- --ignored --exact
cargo test -p tig-store --lib --locked final_retirement::bulk::tests::explicit_cli_bulk_inspection_qualification -- --ignored --exact
cargo test -p tig-store --test retirement_cli --locked pre_final_writer_refuses_new_manifest_and_accepts_legacy_control -- --ignored --exact
```

TIG_BULK_CLI/TIG_RETIREMENT_CLI identify fresh external artifacts;
TIG_PRE_FINAL_CLI identifies pinned273e639. All builds, logs and temporary
synthetic stores remain external. Frontend/restore and source pre-pair artifact
evidence is historical unless explicitly rerun; hosted CI remains separate.

## Synthetic validation, trace and residual scope

EVID-104 contributes bounded VAL-002/003/004 rehearsal: an operator/application
can execute an exact admitted historical block and recover/cancel partial effects
through the CLI using the same library controller as inspection. Held siblings
survive, expired/changed consent holds removal, and cancellation remains possible
when fresh time/source metadata is unavailable. No native category or age meaning
is inferred, and profile decoding does not establish current global authority.

Trace: controls above -> BULK-EXECUTION.md -> separate execute adapter, immutable
Plan::enrollment accessor, Runtime and Cancellation -> nineteen active bulk cases
plus explicit actual fresh CLI qualifier -> EVID-104. Implementation, verification,
synthetic validation and six producer review close separately. Disposition:
pass_with_risk for bounded synthetic scope; broad/exact-head hosted qualification,
independent review and human acceptance remain separate.

Native age/category/UI/Action, real large files/providers, current authority and
host trust, whole-system restore, epoch handoff, journal continuation, eventual
metadata retirement and full intended-use acceptance remain open. This advances
the complete product goal without closing it or authorizing live storage use.
