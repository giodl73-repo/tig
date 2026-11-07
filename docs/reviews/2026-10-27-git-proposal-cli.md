# Verified-cache Git proposal CLI

Public summary: consumer identifiers generalized; original notes retained privately.

Entry759af7a, parentPR30 pending qualification. WP-005/008/009/010/011/016;
REQ-001/004/006/007/008/009/010, SPEC-005/006/007/009/010/011,
IF-005/007/008/009, PKG-004/005/008. GIT/INTEGRATION profiles, pulse05.
Allowed: tig-git proposal preflight split, explicit CLI/Cargo target, synthetic
CLI fixtures and scoped spec/guide/VTRACE records. No live consumer switch.

Provide tig-propose check PROFILE and tig-propose create PROFILE. No arguments
means usage/no action. Both use the SAME library preflight against exact base
and old raw-byte pins; check must write zero Git objects/refs or worktree bytes.
Create consumes a non-deserializable checked request, reusing parentPR30's
verified immutable commit/ref publication and ordinary Git application boundary.

The strict bounded private profile names Git executable/repo/base, explicit
identity/message, existing byte-source roots/protection, and path/old/new Object
pins. No unpinned ambient candidate filename or fallback to worktree bytes.
Use existing ReadCache, bound reads to64KiB and128 entries, verify actual buffers,
deduplicate identical new objects and hold guards until completion. Existing
place order/corruption/protection limits remain authoritative. No copying media,
cache initialization, scan, setup, daemon, upload, checkout, staging or approval.

L0 structure/roles/trace/fmt. L1 bounded strict profile/arguments, zero-write check,
same preparation as create, explicit output statuses. L2 actual CLI with three
existing layouts, corruption/missing/stale source/duplicate/oversize rejection,
same-input retry, shared new objects, unchanged HEAD/index/worktree and standard
Git application. Full locked all-feature workspace/Clippy, standalone Git check,
fmt/structure and final-head Windows/Linux qualification before merge. Parent30
must qualify/merge before final-head consumer qualification. Producer role review
only; native semantics, full claim/custody/adoption/readiness stay OPEN.

## Implemented seam and producer review

PreparedProposal borrows immutable caller bytes and exact source snapshot; only
the library preflight constructs it, and create consumes it under the same bounded
deadline. The existing propose wrapper uses check then create. CLI check serializes
only a report, not this capability. Each independent invocation rechecks.

The CLI accepts exact old/new Object pins and uses existing ReadCache protection.
New objects are deduplicated, actual buffers rehashed and guards held until reply.
It emits private changes/pins and an optional proposal commit/ref; current selection,
complete semantic coverage and payload custody remain false. No native claim is
manufactured by deserializing a report. Failed static/cache/source checks emit no
success JSON and create no Git objects or refs.

Method: producer Git/adoption, concurrency/recovery and provider/privacy lenses;
exact target is this preflight split, CLI and synthetic fixtures. No independent
review, human approval or production acceptance is asserted.

| Lens | Finding / disposition | Remaining boundary |
| --- | --- | --- |
| Git/adoption | Scoped pass: check/create share exact preflight; caller explicitly chooses a write command, normal Git handles application | Native schema/freshness and source/claim handoff are not inferred |
| Concurrency/recovery | Scoped pass: borrowed immutable request, create-only ref retries, protected byte reads and no global cache/index lock | A saved check cannot authorize create; lost replies retry fixed profile; owner recovery remains open |
| Provider/privacy | Scoped pass: exact enrolled roots/layouts/protection, bounded actual buffer checks, no ambient filenames or missing-root initialization; private text omitted from result | Cooperative mode needs cooperating writers; private paths/hashes stay private and do not prove retained custody |

Five new actual CLI fixtures pass17.53s and all13 parent library entries pass8.34s.
They execute all three layouts, zero-write check, same-input create retry, ordinary
Git application, unchanged HEAD/index/worktree, one-reader/request shared-object
deduplication, corruption/missing/stale/duplicate/oversize refusal, ordered roots,
strict argument/profile bounds and no private text/path leakage on refusal.
Initial workspace all-target/all-feature warnings-denied Clippy passes. Overall disposition
pass_with_risk for this seam, not closure of full VAL or legacy consumer readiness.

## Final local qualification

The all-feature workspace suite passes484 entries, separately confirmed by the
Cargo test-list census. The500,000-binding metadata suite passes221.58s. Five CLI
and thirteen proposal entries additionally pass with no default features (17.61s
and19.25s). All-target/all-feature warnings-denied Clippy, standalone Git/LFS check
and fmt check pass. Structural reviewed-file pins are refreshed after producer
inspection; repository validation remains required before commit. Hosted final-
head qualification follows qualified parent30; no native consumer cutover follows.
