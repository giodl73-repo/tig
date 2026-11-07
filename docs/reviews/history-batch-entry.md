# Bounded history composition entry

Public summary: consumer identifiers generalized; original notes retained privately.

Input `dfdf87013ab5f75e585d3256fe86bc139e0e45ec`. WP-009/010/012/015,
REQ-001/004/006/009/010, SPEC-007/010/012/014/015, IF-007/008/010/011/013,
PKG-002/003/004/005/006/008; RUST/PROVIDER/GIT/INTEGRATION profiles. Allowed scope: pure
`tig-assets::edits::sequence`, its existing captured-source bridge in
`tig-git::source::native::updates`, an explicit ordered-sequence request in the
existing history service/CLI, synthetic tests and controlled documentation.
Also applies existing WP-003, REQ-003, SPEC-003, IF-003/004 publication controls.
No existing identity/format change, new publisher/cache, implicit Git mutation or
consumer upgrade. Existing single-operation publication bytes remain unchanged.

Entry: qualified single-edit/history/publication contracts. Exit: compose existing
validated operations against one original record; one final expected-base result,
bounded shared old/new metadata, exact semantic no-op preservation, failure before
any candidate is returned. Maximum 64 operations; existing reference-work limits
apply per operation, aggregate unique metadata byte/document limits across all
operations. The operation limit bounds repeated validation work; this is not an
unbounded history importer.

L0 validator/fmt/diff. L1 focused composition/stale/failure/aggregate-budget/no-op
tests, full locked all-feature workspace and strict Clippy, standalone Git check.
L2 synthetic media and document callers exercise the same pure interface without
IO; an actual temporary Git/metadata-store fixture finishes/reopens the combined
metadata and creates one unchanged-head/index/worktree sparse proposal, then
checks the selected revision and stale-original refusal.
Six producer lenses: identity preserves original expected pin; concurrency exposes
no intermediate write/approval; custody grants no persistence/removal; placement
keeps aggregate bounds; privacy adds no authored-data diagnostics; Git adoption
requires existing independently verified publication and sparse proposal fences.
These are entry findings, not independent review or executed evidence.

## Producer review and focused execution

Eight focused cases now pass: four pure, one actual temporary Git/metadata-store,
and three actual CLI cases. Strict all-feature/all-target Clippy passes. Methods
are named in EVID-081; full final workspace/compatibility and exact-head hosted
qualification are still pending. No production or independent review is implied.

- Identity: serial-equivalent final records retain original expected pins and
  old aliases; legacy request preparation stays on its original code path, so a
  sequence's metadata ordering cannot change old operation replay identities.
- Concurrency/recovery: no intermediate publication; later failure returns no
  candidate and never constructs a publisher. Check/no-op do not open providers;
  successful proposals still require explicit caller application.
- Custody/placement: fixed aggregate acquired/generated metadata limits and hard
  operation ceiling; oversized source inputs refuse before hashing/cloning.
  Existing source protection and independent custody obligations are unchanged.
- Privacy/adoption: real CLI output retains existing redaction/false-authority
  checks. Synthetic media and documents share the mechanism. The compiled REEL
  host must intake the qualified source before real migration uses this operation.

Remaining product work: multi-asset orchestration over one captured snapshot,
qualified REEL intake of the updated shared service, exact-head Windows/Linux qualification
and actual consumer project family conversion. This helper alone does not finish M2 or T3.
