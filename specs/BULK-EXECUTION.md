# Explicit bulk operator execution

Input6f8fdaa7897aa0b1c47d477348c6ee37ee921533.
WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014;
IF-002/003/004/005/006/008/012/013; PKG-002/003/004/007/008/009.
RUST/PROVIDER profiles. Allowed: explicit bounded apply/resume/cancel CLI over
existing bulk/final-provider/controller primitives, independently admitted exact
plan and native clock, metadata-offline cancellation and synthetic fixtures.
No new journal/cache engine, provider initialization, live root, service, hook,
native category/age inference, migration or deployment. Entry: qualified bounded
bulk/inspection/local-clock ports, existing private roots/jobs and complete current
roster/minima. Exit: real CLI exact consent/permission checks, actual eligible
deletion with shared siblings preserved, restart/resume/cancel and fail-closed
clock/authority/profile/provider behavior. Separate implementation, verification,
synthetic validation, trace and six producer review closeout.

## Interface and authority

The inspection profile and tig-storage bulk describe/plan/peak/status retain
their existing meanings. The separate interface is tig-storage bulk execute
describe, or apply/resume/cancel PROFILE PLAN. Apply and resume invoke the same
idempotent bulk controller. No final reacquisition verb is introduced here.

Private schema tig.bulk-execution-profile.design-1 contains inspection (the exact
existing strict inspection profile), admitted_plan (an independently approved
canonical plan digest), permissions {apply, cancel}, and optional clock
{root, minimum, enrollment}. Missing permission fields default false. Portable
PLAN cannot supply roots, permissions, consent or trusted minima. Decode and
validate the exact plan, profile, permission and admitted digest before provider,
metadata, ledger or clock IO. Hashing a plan is not owner approval. The trusted
operator/application protects and supplies this profile outside project Git.
No command generates approval or enables its own permission.

Apply/resume require the explicit enrolled LocalClock with absolute existing
root, independently current ledger minimum and exact enrollment ID matching the
bulk clock identity. No fixed/request timestamp or SystemTime fallback. Native
clock sampling occurs only inside existing explicit controller decisions; status
and discovery never sample. Constructor/enrollment checks precede mutation.
Current complete namespace roster, provider/capacity/job/context and metadata
identity are independently checked exactly as for library execution. Existing
plan/batch/provider bounds and 64 KiB document/response bounds remain. Static
redacted errors disclose no private paths. Provider holds/partial progress remain
structured reports, not false all-or-nothing success or physical-reclaim claims.

## Cancellation and historical recovery

Cancellation is separately enabled for the exact admitted plan and owner/context.
It requires no fresh clock or source metadata writer: expired consent and offline
source metadata cannot prevent recording cancellation. A dedicated Cancellation
port reuses the same internal controller/control/report code with no source-write
capability. It cannot apply or settle historical roots. Missing source/provider
observations remain holds; missing witness/binding evidence remains an error.
It never creates a stale provider admission solely to cancel it. Source subsets
already settled stay retired; released grants stay released; completed deletion
remains historical. Cancellation prevents future work rather than claiming rollback.

Read-only inspection stays writer-free and acquires no mutation method. Existing
Execution callers retain their API and metadata writer. New optional internal
capabilities are private to the shared controller, and cancellation cannot acquire
a clock or metadata writer through a portable request.

## Qualification

L0: fmt, portable validation, reviewed hashes and diff. L1: strict discovery/
profile/permission/plan/bounds tests, existing bulk/provider regressions, locked
default/all-feature workspace, strict Clippy and standalone Git. L2: explicitly
fresh real CLI artifact against owned temporary roots, actual native clock and
hash/length-checked surviving bytes; completed/no-op replay, interrupted intent/
subset/removal resume, expired/clock-offline and metadata-offline cancellation,
changed use/authority, wrong roots/jobs/minima/clock and disabled permissions.
Assert no source/provider/clock commands on preflight denial. Execute ignored
artifact qualifiers explicitly locally and on Windows/Linux CI.

Native age/category selectors, UI/Action integration, live OS/current authority
trust and whole-system restore, operational journal continuation, large-file/real
provider qualification, metadata retirement and human acceptance remain required
full-product work. This source interface grants no live operation authority.

## Bounded implementation checkpoint

Implementation6b9016e and EVID-104 qualify the named local CLI/shared controller.
[Six-producer review](../docs/reviews/2026-11-09-bulk-execution.md) separates
verification, synthetic validation and trace: nineteen active bulk entries,
actual fresh execution/inspection/cancellation artifact tests, both full workspace
suites/both500k metadata fixtures, final strict Clippy/standalone/fmt/L0 and pinned
older-writer refusal/legacy control pass. Later additions strengthen permission/
replaced-root tests; runtime and record encodings remain unchanged. Hosted
exact-head, live authority/provider/large-file/lifetime and human acceptance stay
separate. This completes no whole product milestone or operational enrollment.
