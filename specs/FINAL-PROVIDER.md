# Bounded final-capable local provider

Input273e639cd9515f0860140c926240b18e4ca69a1b.
WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014;
IF-002/003/004/005/006/008/012/013; PKG-002/003/004/007/008/009.
RUST/PROVIDER profiles. Allowed: explicit local final-retirement library,
versioned fresh provider, existing ledgers/fences/publication, bounded canonical
records and synthetic fixtures. No live roots, upgrade, daemon, CLI activation,
bulk selection or full production qualification. Entry: complete privately
admitted claim roster, owner/policy/context and clock authority, exact existing
publication and grant set, reserved metadata/job. Exit: durable final intent,
release, unlink/completion/cancel with honest partial recovery and legacy refusal.
Implementation and verification/validation/trace/six producer reviews close apart.

This in-progress slice implements the bounded local layer of [FINAL-RETIREMENT](FINAL-RETIREMENT.md),
not its whole bulk/provider/lifetime contract. Ordinary eviction keeps its
nonempty independently retained keepers. Owner-approved final loss is a different
operation and cannot be inferred from missing demand or from a successful merge.

## Format and authority

`LocalStore::create_final` creates only a fresh explicit design-8 root. Existing
design-5/6/7 formats are unchanged; no implicit upgrade or root census exists.
Design-8 adds required final_policy to the existing manifest: schema
tig.final-policy.design-1, authority, epoch, content scope digest, clock policy,
1..16 sorted eligible owners and max_window in milliseconds (1..604,800,000).
Removing that policy or using an unknown/mixed manifest refuses. Old writers
must demonstrably refuse the profile. Existing-handle/live migration and latest-
authority rollback resistance remain separate acceptance gates.

One plan binds schema tig.final-retirement-plan.design-1, store/binding generation,
the immutable policy, exact current Birth, Request, sorted active RetentionGrants
and metadata limit. Request names operation, owner, original publication,
capacity authority, private context and not_before/expires window. The initial
local profile admits a single consenting owner; another owner's active grant
holds rather than being released. Unknown/unsettled grant evidence also holds.
The private application explicitly authorizes this exact plan; a cookie/hash is
not human approval or authentication. Runtime context, store policy and admitted
content roster are independently compared before effects. Concrete roster minima
come from trusted admission, never from weaker request-selected witnesses.

Clock is a required private Clock port, not request data or SystemTime fallback.
It supplies current time for the immutable enrolled clock policy and must provide
its admitted anti-rollback/freshness guarantee across operations. Local execution
also rejects backwards time within the operation and outside its consent window
before release/unlink. Unknown clock holds. Synthetic clock fixtures qualify the
port/phase behavior; they do not qualify native OS time, remote clock authority,
live epoch changes or offline policy adoption. Cancellation is separately
owner-authorized and remains possible after deletion consent expires.

Clock::bounds now permits explicit earliest/latest observations; the entire
interval must fit the window before source settlement, release, unlink or a new
reacquisition birth. Exact existing clocks keep point-interval defaults and record
bytes stay unchanged. [LOCAL-CLOCK](LOCAL-CLOCK.md) adds a fresh explicit bounded
OS/journal adapter; declared OS trust, current independent minima and full native
restore/adoption remain separate, not inferred from a native clock API existing.

Portable Place descriptors use the separate contract identity
tig.local-store.design-8/process-restart/final-incarnation/v1. A final-capable
root cannot masquerade as design-7 in copy/placement plans. Actual root instance,
binding generation and contract are compared by existing executors. An older
descriptor reader rejects that unknown contract independently of manifest refusal.

## Immutable records and capacity

The existing lifecycle directory and per-object/attempt fences are reused.
Each birth may have final-intent, final-release and final-terminal JSON files;
ordinary intent/done/abort cannot coexist with final intent at that birth.
All records are strict canonical JSON bounded to 65,536 bytes. Independent ASCII
canonical/hashlib vectors pin manifest, policy/plan, intent, release, deletion
terminal and pre/post-release cancellation framing before runtime qualification.

Intent (tig.final-retirement-intent.design-1) contains plan, job/allocation refs,
the actual admitted roster/checked checkpoints, mode, optional time and measured
payload allocation. Delete mode requires a verified time and all checked members;
cancel_only has no membership-clearance assertion/time and cannot release grants.
Release (tig.final-retirement-release.design-1) binds the exact intent digest and
fresh in-window nondecreasing time. Terminal (tig.final-retirement-terminal.design-1)
binds intent, optional exact release digest and outcome deleted/cancelled. A
deleted terminal requires release evidence. Pre-release cancellation cannot later
accept release evidence; old released grants remain released after cancellation.

Existing CapacityLedger BeginRetention/SettleRetention allocates metadata under
a domain-separated (final plan, job) binding; no new journal/event engine or
negative reclaim credit. Existing LocalAttempt binding/admission precedes writes.
Metadata limit conservatively rounds 65,536 bytes by the store allocation quantum
for 16 attempt files + 113 lifecycle files + 20 recovery/claim allowances. Objects
remain capped at 8 MiB, grants at 16, births/action slots at 16; final planning
requires a spare continuation slot. Ordinary stores retain their 65-file bound.
Known partial attempts, exhausted budgets/history or missing settled evidence hold.
No prospectively freed payload space finances intent or recovery.

## Execution and recovery

After metadata admission/binding, inspect exact existing state. Historical terminal
replay does not repeat a decision or repair missing evidence under spent budget.
Fresh execution holds content-wide namespace clearance and the existing provider
object-exclusive fence together. Verify exact current bytes/birth, complete active
grant set, owner and settled grant allocations. Persist synced immutable intent
before grant effects. Revalidate clock/claim markers; persist release before
removing only matching active grant files. Release is the exact grant invalidation
point even if file cleanup is interrupted. Another grant or changed record holds.
Revalidate bytes, consent time, content clearance and provider protection through
exact unlink, then persist terminal and settle measured metadata footprint.

Exact intent + release + already absent original placement permits historical
absence recovery without fresh deletion/clock permission. Missing bytes without
release evidence remains recovery failure. A later active claim cannot undo
historical absence; it is now protected-but-unavailable until explicit acquisition.
A different birth is never removed by old retry. Completed absence and unknown
damage are different; receipts separate historical outcome, current unverified
presence/absence, original logical/file allocation and charged metadata. Unlink
does not promise secure erasure, snapshot/trash removal or volume-free-space gain.

Cancellation before starting writes cancel_only intent then cancellation, preserving
grants/bytes. Cancellation after delete intent but before release preserves grants.
After release it preserves bytes and reports released/retired-but-present; original
grants never revive. Fresh owner grants are required to restore custody. A cancelled
action advances a fenced lifecycle slot without changing physical placement, so
old publication receipts survive and the cancelled delete cannot rearm. If bytes
are already absent under release evidence, recover deletion rather than claim
rollback. Actual reader/provider locks remain effective; no global cache lock.

Ordinary placement refuses a pending/completed final lifecycle and cannot use
ordinary reacquisition authority to recreate it. Once all source memberships are
settled, the existing source-derived retained floor has no row for that object;
stale pages are invalidated by source basis. Fresh reverted demand is held for
explicit final reacquisition. Complete user/agent final-reacquisition, bulk intent/
selection, native closure, audit/metadata expiry, large-file/native provider/clock,
authority migration and live adoption remain separate required product work.

L0: fmt/portable validator/reviewed hashes/diff. L1: locked focused/default/all-
feature workspace, strict Clippy, standalone Git and actual pinned legacy refusal.
L2: temporary accounted stores and exact bytes, grant/reader/source/clock/budget
refusals, cancellation on both sides of release, abrupt exits around release/
unlink/completion, missing evidence and stale retry versus later state. No live
cache/user asset or full REQ/VAL acceptance is implied.

## Explicit final reacquisition and status

The library reacquire::Plan binds the old completed final plan/terminal, a fresh
publication operation, next birth/placement, capacity authority, private context
and its own consent window. Canonical vectors pin the context-bearing birth and
plan framing. An old deletion cookie is insufficient. Metadata and payload
admission plus exact attempt bindings precede birth. Fresh birth requires current
in-window clock authority. Once birth is durable, source-preserving copy may
resume after expiry; this is completion of that exact admitted acquisition,
never authority for another birth. Existing accounted streaming verifies exact
hash/length. Original released grants stay released; new custody needs new grants.
Historical deletion replay cannot remove the new incarnation. No CLI/UI or
automatic final-reacquisition permission is enabled by this library port.

status::inspect synchronizes the existing ledger without adding commands or
repairing records. It separates not_started/admitted/intent/released/deleted/
cancelled/recovery_required, original grant-release evidence, metadata settlement
and current unverified presence/absence/unknown. An absent file after release but
before completion stays released until authorized historical recovery records
completion. Every status reports removal_authority false. Observations are not
byte-integrity or current-claim/authority proof and may change after inspection.
