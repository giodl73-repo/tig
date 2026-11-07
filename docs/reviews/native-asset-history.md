# Producer review: usable native history inspection

Entry/controls/scope: [NATIVE-ASSET-HISTORY](../../specs/NATIVE-ASSET-HISTORY.md).
Targets: `tig-git::source::native::lifecycle`, existing `tig-resolve`, delivery
aggregate helpers, new native lifecycle tests and the schema-disabled refusal
case. No native object, provider or journal format changes.

Method: manually apply all six [role checklists](../../.roles/ROLE.md). Findings
below concern producer verification, not independent review or creative approval.

| Role / finding | Severity | Disposition and remaining owner obligations |
| --- | --- | --- |
| Identity: inspect dirty or newer data instead of the requested version | High | Capture exact Git commit/scopes and pin-check requested record/definition/schema; old commit and unavailable-Git SDK tests pass. T2 write/proposal base fencing remains separate work |
| Placement: missing profile/history becomes zero demand | High | Unconfigured reports unknown coverage; configured missing/failed reads refuse without stdout. No claim adoption or working/history placement; T3 still owns that integration |
| Custody: verified receipt bytes imply output availability/remote success | Critical | No media/opaque-witness reads. Payload, remote, custody, eviction and complete-semantic flags false; local domain contract execution also explicitly false. Real consumer/custody acceptance remains open |
| Concurrency: inspection reserves/locks an entire cache or changes source | High | Existing per-object held reads only; no new lease, writer, journal or global lock. Record/store byte comparisons pass; explicit read-capture is not a later mutation fence |
| Provider/privacy: collapse denial into absence or echo provider paths | High | CLI preserves fixed denied/unsupported failures separately; generic errors and bounded aggregate output omit target/asset keys, locators and authoring. Denied Windows and unsupported non-Windows tests cover declared capabilities |
| Git/adoption: new schema mode breaks standalone Git/LFS or enables itself | High | Optional schema feature; assets-only mode refuses history before profile IO. Existing lookup/stream/verify stay unchanged; no runtime binding or private asset conversion |

## Executed focused evidence

Ten schema-enabled native lifecycle tests pass, using real temporary Git plus
existing Reel CAS and Git LFS layouts. They cover two distinct native definitions,
exact committed/old source, absent Git after capture, strict schema failure before
byte IO, planned/revision-only assets, aggregate ambiguity/absence/unknown states,
no-write/no-media operation, missing/profile/budget/corrupt access and privacy.
The schema-disabled history refusal test passes under `assets` only; standalone
Git/LFS compilation passes. Strict all-feature/all-target workspace Clippy and
repository fmt/structural/diff checks pass. Refresh only the three existing review
pins for the real resolver/export/test changes; no old target or budget relaxation.

The inspector executes the requested record's authored schema and common history
contracts. It does not execute every domain stream/record rule, validate every
other asset's semantics or imply a previously published item is currently remote.
An absent claim has its own count; it is not counted as a publication claim.

Full workspace/hosted exact-head qualification remains separately recorded. No
real project cache read, upload, render, tool installation or source-lane switch
occurred. Compiler outputs use the explicit F temporary target; raw logs stay
outside Git. T2 register/select/record-delivery operations and T3 lifecycle claims
are still required before claiming complete native asset-management readiness.
