# Manual paged controller review

Input88225ff0e7e2ed8082f660eedcd54f4781588834;
implementation4bcd433f8a3e89dc1819aef1def5dcba68dd1199.
Controls, allowed scope, entry/exit and RUST/PROVIDER L0/L1/L2:
[PAGED-EXECUTION](../../specs/PAGED-EXECUTION.md). Targets: manual control
decision/copy/subject/action indexes, separate exact permissions, existing
accounted global/page witnesses, actual receipt-checked subset progress and final
provider composition, metered metadata ports, inspection and cancellation.
No new SQL/cache/event engine, mutation CLI, live roots, deployment or migration.
Reviewed target hashes are in foundation-targets.json.

## Producer method and findings

One producer applies all six [.roles](../../.roles/ROLE.md) checklists. Owner of
each finding: Tig maintainer through the named lens. This is neither independent
review nor human acceptance or deletion authority. Challenge actual source and
provider effects, metadata identity, partial outcomes and old/new operation
binding; compare persisted decisions with actual receipts and process restarts.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| PC-01 Identity and Namespace | High | Small decisions bind private enrollment, exact whole selection, conditional preparation and asset-side subject/copy/action indexes. Initial apply reconstructs the exact decision against current sources/providers. Separate claim/final permissions cannot mix decisions. A changed window cannot rebind an acknowledged operation. Concrete metadata identity refuses before witness admission. Fourteen independent canonical frames plus key/control-binding derivations pin new encodings. Original exclusions and exact provider plans can be inspected before approval. | Actual private writer/current authority and epoch handoff/adoption remain external gates; candidate hashes authenticate no permission. |
| PC-02 Placement Policy | High | Claim settlement deliberately covers exact selected history independently of final-copy eligibility. Unselected siblings obtain owned survivor protection first. Twenty pages over17 historical roots finish across bounded calls; only three original eligible copies disappear. Current job/shared and unselected bytes survive. Removing a later protection claim does not expand original copy actions. | Native block/category/age admission, reactivation and complete real consumer/job/dependency coverage remain required. This mode admits exact manual selections only. |
| PC-03 Storage and Custody | High | Every original provider plan pins actual birth/grants and capacity/context. Recovery metadata is admitted before source effects; complete witness peak is checked before settlement. Physical actions follow all completed source pages and existing final live clearance. Original copy receipts describe historical deletion, not current whole-object absence, secure erasure or free volume space. Ordinary keeper-based eviction is unchanged. | Multiple actual providers, large files beyond the8MiB reference cap, native volume/quota authority, measured reclaim and operational custody qualification remain. |
| PC-04 Concurrency and Recovery | High | Scoped batch/page attempts reuse existing witness ports. Page intent precedes source IO; completion requires actual retirement checkpoint equality. Known completed-page/source-prefix restore refuses. Unknown transient command history blocks physical work under final live fences. Fourteen actual process exits cover global/page/source and provider intent/release/grant-removal/unlink/terminal/settlement boundaries. Cancel acknowledges stop before metadata/provider cleanup and never resumes selected history afterward. | Globally unseen restore, current authority/native writer adoption, age acknowledgement before page completion and broader cancellation/provider matrices remain separate. |
| PC-05 Provider and Privacy | High | One call-owned node budget spans readback, reconstruction, successive subset application/admission and final clearance. Failed reads and new views consume the same allowance. A two-page regression proves the second page cannot reset it. Deadline also covers final owned-history checks. Fixed records remain64KiB; listed holds cap64 and provider attempts cap32. Metadata-offline stop reports null observations/unknown debt, then resolves admitted cleanup when metadata returns. | Sustained journal/source/witness continuation and dependency-aware metadata expiry must preserve receipts/fences; no reset or implicit sweep is provided. |
| PC-06 Git and Adoption | Medium | Runtime data/indexes remain asset-side and portable records contain identities rather than machine paths. Existing publication/source/provider encodings are unchanged. A crate-private metered subset path and guarded provider fault hook compose existing primitives; old APIs retain their behavior. Fresh ordinary/bulk CLI and pinned old-writer refusal/accepted legacy control pass. | Operator/agent/UI/Action interfaces, exact admitted aged evidence, actual F/D/M adoption and independent/human acceptance remain open. No hook/runner is installed. |

Review strengthened the candidate before commitment: unknown observations became
null rather than zero; historical source retirement became separate from page
completion/cancellation; a concrete wrong metadata provider now refuses before
admission; the whole witness peak is checked; metadata budgets persist across
protocol phases; completed provider prefixes no longer expand returned effects;
and an inspectable original exclusion/copy set is available before approval.
The initial readonly helper shadowed the manifest inspector; qualifying that
call corrected the compile error before final checks. Strict Clippy findings
were corrected, not suppressed. None of those failed checks is passing evidence.

## Verification

Thirteen controller entries include the child helper and twelve integration
fixtures. The process-exit fixture actually reaches fourteen named boundaries.
The independent framing entry covers enrollment, decision, copy index, five
subject/hold cases and six global/page intent/terminal documents, with separate
key/control-binding checks. These pin framing, not authority or semantic closure.

- Default workspace/500k/standalone Git/strict workspace Clippy chain17838 and
  all-feature/second500k/strict workspace Clippy chain41582 terminate zero.
- Those test compilations preceded the last readonly explanation and composed
  guarded fault-hook additions. Final79 active retirement entries under default
  and all features (two explicit artifact entries ignored) pass afterward, with
  strict default store and all-feature workspace Clippy (84350/95356 terminal zero).
- Sixteen active subset entries, one baseline source-artifact entry ignored,
  pass with the new shared-page-budget regression. The independent failed-read/
  changed-view/write-budget regression also passes.
- Freshly rebuilt bulk inspection/execution and ordinary pending-cancellation
  CLI qualifiers pass (90958 terminal zero). Pinned pre-final writer refusal with
  accepted legacy control passes (direct exec terminal zero).
- Review additionally pins copy-index and all subject/hold frames without a
  runtime change. The strengthened exact vector entry and strict store Clippy
  pass (41297 terminal zero).
- Fmt, portable structure/local links/budgets/reviewed hashes and diff checks pass
  before implementation; refresh L0 after this review. Hosted exact-head Windows/
  Linux/secrets qualification remains required.

First hosted run42665505108 flagged two generic-api-key matches in historical
vector lines20/35 of implementation4bcd433. Both are public operation digests
recomputed by independent framing; no credential is present. The two exact
commit/path/rule/line [fingerprints](../../.gitleaksignore) are excluded, with no
file-wide or rule-wide suppression. Pinned Gitleaks8.30.1 (official Windows
release archive SHA-256 verified) then scans all92 reachable commits clean locally.
Resubmitted hosted qualification must still succeed at the new exact head.

Commands use explicit external build/temp roots and finite test threads. The
default/all-feature/final-focus logs remain outside source; pinned old binaries
are preserved in separate targets.

```text
cargo test -p tig-store --lib --locked final_retirement
cargo test -p tig-store --all-features --lib --locked final_retirement
cargo test -p tig-store --lib --locked source_claims::subset::tests
cargo test -p tig-store --lib --locked protection::budget
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo check -p tig-git --no-default-features --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo build -p tig-store --bin tig-storage --locked
cargo test -p tig-store --lib --locked explicit_cli_bulk -- --ignored
cargo test -p tig-store --lib --locked explicit_cli_pending_cancellation_qualification -- --ignored
cargo test -p tig-store --test retirement_cli --locked pre_final_writer_refuses_new_manifest_and_accepts_legacy_control -- --ignored
pwsh -NoProfile -File tools/validate.ps1
git diff --check
gitleaks git . --redact --log-opts=--all
```

## Synthetic validation

Parts of VAL-002/003/004 now exercise actual manual block execution beyond the
old16-root bound, not only metadata or direct subset applications. Temporary
journals/stores witness20 pages across17 original roots and deletion of three
one-byte eligible objects, while a live job/shared object and unselected object
stay readable. Partial provider-attempt limits converge without repeating loss.
Metadata-offline cancellation records stop, reports unknown debt and resolves it
after metadata restoration. Known source-prefix restore and stale protection
history hold work. This is bounded local synthetic mechanism evidence, not a
representative large-file/workload result or real F/D/M/human acceptance. All full
VALs remain open.

## Trace and disposition

WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009 ->
SPEC-002/003/004/005/006/008/009/013/014,
IF-002/004/005/006/008/010/012/013, PKG-002/003/004/006/008/009 ->
PAGED-EXECUTION -> paged::control, protection::budget, subset::apply_metered and
guarded provider composition -> EVID-109.

Disposition: pass_with_risk for this manual Rust candidate. It advances NB-07 to
actual paged source/provider execution; full aged/native operational acceptance
is still owed. Freeze and verify admitted age evidence, preserve acknowledged
survivor inheritance before page completion, compose native/operator/agent/UI/
Action surfaces and qualify actual current authority/writer/time/restore/provider/
large-file/lifetime/metadata-expiry behavior. All full requirements, independent
review, human acceptance and live-operation authority remain open.
