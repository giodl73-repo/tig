# Restore partial visibility: producer review

Public summary: consumer identifiers generalized; original notes retained privately.

Entry `19790c58f35ca764fc66ca1f818a56bf9b75de90`; controls/allowed scope in
[PROJECT-PARTIALS](../../specs/PROJECT-PARTIALS.md). WP-005/011/015/016,
REQ-001/003/006/007/010, RUST + INTEGRATION. Only bounded immediate-directory
metadata observations over an exact plan; no payload open or cleanup authority.

Nine new cases plus existing project regressions pass: 42 tests, 24.37 s.
They cover missing roots/source bytes, unrelated-entry budget exhaustion,
nonrecursive scope/sorted output, unchanged bytes, zero-length files, active
writers, malformed names, non-files, stale plan, linked ancestry/entries, actual
interrupted restore with explicit retry and CLI incompatible-mode refusal.
Links use Windows junctions and Unix symlinks. The Windows fixture helper first
passed a mixed-separator path to mklink, so fixture creation failed before
inspection; converting only the helper's command arguments to Windows separators
fixes setup and both refusal cases pass. An initial iterator error-type mismatch
was corrected before test execution. No admission or safety assertion was relaxed.
Full workspace/lint and exact-head hosted checks remain separate gates.

Subsequent full locked all-feature workspace tests pass, including the
500,000-binding persistence fixture. Strict all-target all-feature Clippy passes
(19.09 s), standalone Git/LFS checking passes (4.43 s), and format/foundation/
whitespace checks pass. Exact-head hosted qualification remains required.

An actual metadata-only consumer project check at main `72907b5`, using the immutable
inspector built from Tig `a57a874`, reuses the exact raw profile and expected
plan of the previous handoff preview. It reports one missing planned parent,
zero listed entries/candidates and no payload reads or destination creation.
All ownership/liveness/cleanup flags remain false and physical reclaim remains
unknown. Raw report/profile/runtime hashes stay outside Git. This proves the
missing-root route, not discovery of actual abandoned files or cache coverage;
later documentation/rebase commits do not relabel the tool used in this check.

Producer lenses:

- Identity/namespace: exact expected plan binds source/profile/scopes/budget;
  matching random filename syntax proves no plan or attempt ownership.
- Provider/privacy: only declared immediate parents are listed, all entries count
  toward the bound, links/reparses are never followed and raw operational paths
  remain private. Trusted namespace races are not advertised as sandbox safety.
- Storage/custody: logical lengths are not physical reclaim or retained custody;
  report explicitly denies ownership, liveness knowledge and removal authority.
- Concurrency/recovery: inspection adds no locks or mutation, does not block a
  held writer and leaves partials through verified retry. Observations can race.
- Git/adoption: ordinary Git holds the small contract/code; detailed listings
  stay asset-side. No scan is installed on resume, checkout, merge or startup.

Disposition pass_with_risk for read-only visibility. Owner: Tig orchestration.
No independent review or human acceptance. Authenticated attempt attribution,
fenced authorized disposal, complete source/job coverage, custody and production
adoption remain open. An unassociated candidate is never classified safe to delete.
