# Bounded final-provider review

Input273e639; implementation9f5e8c0b1c5e0a7dcbd5fa7647e51571e1d9af8e.
Targets: FINAL-PROVIDER.md, final_retirement::provider/reacquire/status,
LocalStore final state/incarnation, accounted/ordinary retirement peaks,
portable Place contracts, placement hold/CLI error mapping, canonical vectors,
old-writer qualification and the S3 contention regression fixture. Exact source
targets/hashes are recorded in foundation-targets.json.

Controls: WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009; SPEC-002/003/004/005/006/008/009/013/014;
IF-002/003/004/005/006/008/012/013; PKG-002/003/004/007/008/009.
The inherited S3 test clarification also uses WP-005/014 and SPEC-007.
RUST/PROVIDER profiles, with Git regression coverage for the CLI hold mapping.
Entry: explicit fresh synthetic provider and privately admitted scope/owner/
context/clock, exact existing publication/grants and reserved capacity.
Exit: bounded local intent/release/unlink/cancel/reacquire/status qualification;
no live adoption, format upgrade, bulk workflow or full requirement acceptance.

## Method and producer findings

One producer applies all six .roles checklists; no delegation, independent review,
human approval or publication/deletion authority is implied. Owner of every row:
Tig maintainer using the named lens. Severity describes the counterexample before
the disposition; retained adoption gates remain open.

| Finding / role | Severity | Disposition and evidence | Remaining gate |
| --- | --- | --- | --- |
| FP-01 Identity and Namespace | High | New birth/context and separate design-8 portable contract prevent stale placement interpretation. Old deletion cookies cannot authorize final reacquisition; same-hash new bytes/new retention survive old deletion/grant replay. Eleven independent ASCII/Python canonical/hashlib cases pin record and reacquisition framing. | Live migration, known/global restore authority and metadata forgetting. |
| FP-02 Placement Policy | High | Public placement executor has no retained-floor row after source settlement; reverted demand holds final absence even with ordinary copy/reacquire enabled. Explicit final reacquisition then permits convergence. Content-wide clearance remains independent of place-local surplus. | Bulk age/category/native selection, batch consent and full source settlement integration. |
| FP-03 Storage and Custody | High | Exact owned grants are released before unlink; foreign/new grants and readers hold. Cancellation before release preserves old grants; after release preserves bytes without reviving grants. Receipt separates historical removal, unverified presence, file allocation and metadata charge. | Real custody/provider/version/trash behavior, large objects and measured volume reclaim. |
| FP-04 Concurrency and Recovery | High | Metadata admission/attempt binding precede immutable intent. Nine actual child exits cover admitted/bound/intent/release/grant cleanup/pre-unlink/unlink/terminal/settled boundaries. Injected cancellation/reacquisition boundaries and missing-evidence refusals retain exact outcomes. | Full capacity/journal exhaustion matrix, source-settlement cancellation, continuation and authority handoff. |
| FP-05 Provider and Privacy | High, retained gate | Fresh design-8 roots only; actual pre-final CLI refuses them and accepts a legacy control. Clock port is independently admitted, with synthetic late-expiry/rollback refusal. Portable records carry identities, not credentials/root paths. | Native clock freshness, latest authority, OS/provider durability, old already-open writers and live adoption. |
| FP-06 Git and Adoption | Medium | CLI reports the new placement hold; ordinary keepers remain. No final CLI/Action permission is enabled. Default/all-feature Git regressions pass. S3 contention now tests both-Busy plus explicit serial convergence, without runtime changes or automatic retry. | Full user/agent bulk interface, trusted executor deployment and human intended-use acceptance. |

## Verification

Final-runtime locked default and all-feature workspace suites pass in separate
external targets, including both 500,000-binding metadata runs. Thirteen focused
provider entries pass (one child helper; nine actual abrupt exits). They also
cover source membership outside the store binding, reader/new-grant protection,
foreign owner, insufficient metadata reservation, wrong context/cookie, expiry,
late rollback, cancellation before/after release, missing evidence, exact replay,
fresh same-hash grants, read-only status and ordinary placement integration.

The explicitly executed ignored pre-final artifact test passes against the
parent-built tig-storage: legacy planning succeeds and final-capable manifest
planning refuses, with source/ledger preservation. CI separately builds the
exact pinned273e639 artifact on Windows/Linux; current-head CI is not local proof.
Fresh all-feature CLI ordinary-retirement cancellation qualification also passes.
Strict workspace all-feature/all-target Clippy, standalone Git without optional
features, fmt, portable L0/reviewed hashes and diff checks pass. No runtime code
changed after9f5e8c0; this closeout is documentation only.

The first all-feature run failed an S3 assertion requiring a winner in every
racing round. Twenty unchanged isolated two-process races passed. Inspection
showed nonblocking ledger admission; a new held-SQLite-reader fixture proves
both peers may return Busy, then serial resume converges after release. The test
still rejects all unexpected failure codes and checks source bytes, remote bytes
and exact no-reupload retry. All fifteen corrected S3 entries pass separately;
the full final-runtime all-feature rerun passes as well. This does not prove the
unrecorded original response codes or add a fairness/automatic-retry guarantee.

Source-journal event bytes are unchanged. Existing pre-pair artifact qualification
is historical evidence; it was not rerun locally for this provider slice. No
frontend changed, and no new local Node/restore or human UX result is claimed.

## Synthetic validation and residual scope

Actual temporary payload unlink intentionally ends the admitted last copy.
Cancellation preserves observed bytes while retaining honest released-grant
state. Fresh exact-byte reacquisition and subsequent ordinary reconciliation
restore new custody without allowing old deletion replay to affect it. This is
synthetic validation of the bounded library intent, not the full F/D/M workflow.

The 8 MiB/process-restart reference limit, sixteen owners/grants/birth slots,
bounded journal/file records and safe exhaustion remain. No capacity credit
comes from predicted reclaim. Full bulk selection/source consent, source-aware
age provenance, native closure, actual providers/large files, authority/clock/
epoch adoption, metadata/audit lifetime and user acceptance remain open.
Disposition: pass_with_risk for the named bounded local scope. Full requirements,
independent acceptance and the complete PRODUCT_PLAN goal are not closed.
