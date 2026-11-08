# Bounded storage plan and local copy orchestration

Input bb51c5a13e1277c1b76e1bcdd2d865800595e733; WP-003/004/006/007/008;
REQ-001/002/003/004/005/007/009; SPEC-002/003/004/005/008/009;
IF-002/003/004/005/006/012/013; PKG-002/003/008. RUST/PROVIDER profiles.
Scope: tig-store plan and source-preserving local copy composition using existing
publication/capacity/retention primitives, with no optional asset dependency.
Entry: enrolled owned temporary stores, existing generation/replay fences and
explicit caller admission. Exit: bounded immutable plan identities, exact retry
conflict refusal, source-preserving copy/retention and fresh restart inspection.
This is the first priority-2/3 slice, not full protection admission or eviction.

## Contract frozen for this slice

The versioned plan binds exact source/destination store instances, enrollment
generations, provider contract identities, capacity authority, source checkpoint,
policy/claim/authority basis and declared protection coverage. These are trusted
caller assertions, not authentication, native volume discovery or complete-claim
proof. The first provider profile is local process-restart only. All plans expose
removal unavailable as a hold even when coverage is declared complete. Unknown
coverage does not block source-preserving copy. No removal API exists here.

At most 32 distinct objects/operations and 64 KiB canonical JSON; exact hash/size
conflicts, duplicate target objects/operations, aliasing source/destination and
invalid/overflowing budgets refuse. Object order is canonicalized by operation;
changes to the selected set or basis change plan identity. Per-action operation
keys are caller-supplied stable identities, distinct from content hashes.

Explicit local apply validates current admitted basis/enrollments before IO,
checks the destination capacity ledger instance, recomputes publication/retention
peak and uses a caller-admitted job. The existing reference provider remains
capped at 8 MiB per object. Copy holds verified source access through publication,
then installs and verifies destination retention. No source removal or automatic
job finish/cleanup occurs. Batches can partially complete; cancellation between
actions preserves completed copies and pending allocations. Mid-stream private
partials require existing explicit cleanup; resume never sweeps them.

Accounted PublicationPlan gains an optional context digest. Legacy None encoding
and old allocation binding bytes are unchanged. New context-bearing records bind
the orchestration plan into existing exact-operation ledger replay; old readers
reject the unknown field and must be deliberately upgraded. Changed context/job
under the same operation conflicts, including after restart. No new journal
command/schema or independent cache engine is introduced. Existing explicit
private-cleanup plans retain and validate the context.

Read-only status synchronizes existing journal history and reports not-started,
partial or freshly verified retained for an exact action/job. It creates no
publication/retention/repair command. Invalid/missing acknowledged bytes and
foreign/conflicting history remain errors, never absence or permission to retry
blindly. Returned live retention holds guards; serialized plans/status do not.

L0: validate.ps1, diff and fmt. L1: focused plan/retry/negative tests, locked
workspace default/all-feature suites and strict all-target Clippy. L2: isolated
actual local stores and SQLite, multi-object partial progress/restart, source
preservation, wrong roots/ledger/basis, missing/corrupt bytes and capacity refusal.
Pin commands/results in evidence after running. Stronger durability, hostile
filesystem mutation, complete claim registry, effective removal authority,
large-object scale, cloud/upload, lifecycle events, final deletion and metadata
retirement remain open and retain the roadmap's separate gates.

## Explicit CLI and agent surface

`tig-storage describe` lists the versioned bounded interface without provider IO.
`tig-storage plan REQUEST` emits canonical `tig.storage-plan.design-1` bytes from
a strict PlanRequest, with no provider enrollment or mutation. Store those exact
bytes outside Git. The plan contains identities, not machine root paths.
`tig-storage peak PROFILE PLAN` recomputes the conservative file-data peak.
`tig-storage status PROFILE PLAN INDEX` freshly inspects one exact action.
`tig-storage apply PROFILE PLAN INDEX` and `resume` invoke the same idempotent
copy/retention path. Each action is explicit, so a caller can stop scheduling the
remaining batch; there is no implicit worker, background retry or partial sweep.

PROFILE is machine-local strict JSON with schema `tig.storage-profile.design-1`,
absolute source_root/destination_root/ledger_root, trusted minimum ledger
checkpoint, existing job JournalRef, current admitted source/destination Place
and Basis. Bootstrap/job admission remains with the existing explicit library
caller; the CLI never creates these authorities. Do not derive current admission
from an untrusted plan. Errors are bounded response codes without private paths.
CLI status opens existing SQLite via the existing ledger API, synchronizes history
and adds no journal commands; it is not an OS-enforced read-only connection.
Process exit releases live read guards; durable retention stays in existing store
records. The catalogue can reuse this library plan later, but no UI apply/removal
binding or provider deployment is added by this slice.

The separate [retirement namespace](RETIREMENT-AGENT-CLI.md) now uses the opt-in
managed provider and independently admitted private policy. Its existence does
not grant removal to these original copy commands or change their plan encoding.

## Explicitly bound local stores

The later [store protection boundary](STORE-PROTECTION-BOUNDARY.md) adds an opt-in
local design-6 provider marker for fresh synthetic stores. Copy/retention behavior
and acknowledgement limits remain those of the existing local adapter. Place
descriptors use a distinct design-6 contract identity and must match the actual
bound enrollment generation; actual provider contract/instance/generation are
rechecked before execution and destination peak computation. Old design-5 plan
identities remain unchanged. Older clients reject the unknown provider contract.
This supported copy profile does not grant removal or implement namespace/epoch
migration. Machine-local paths and authority witnesses remain caller-owned.

Fresh design-7 stores expose a distinct managed-incarnation copy contract. Initial
copy/retention uses existing accounted primitives with additional bounded lifecycle
metadata budget. A previously retired hash requires the explicit library
[reacquisition protocol](MANAGED-REPLICA-RETIREMENT.md); ordinary copy never silently
revives it. tig-storage errors distinguish authorized_absence, reacquisition_required
and needs_recovery. The copy CLI still exposes no removal/reacquisition command;
the new guarded retirement executor is a separate explicit library interface.
