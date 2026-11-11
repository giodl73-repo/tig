# Bounded local placement reconciliation

Input a6783814c22c5d1de01a95fb026f05e8bd8c400b.
WP-003/004/005/006/007/008/009/015;
REQ-001/002/003/004/005/006/007/008/009;
SPEC-002/003/004/005/006/007/008/009/013;
IF-002/003/004/005/006/008/009/012/013; PKG-002/003/004/008/009.
GIT/PROVIDER profiles. Allowed: existing guarded namespaces, explicitly enrolled
local cache/retained providers, verified input readers, accounted publication,
retention/handoff and managed ordinary retirement/reacquisition. Synthetic roots
only. No new cache engine/journal, live enrollment, runner or final deletion.
Entry: exact Git desired selection and existing provider/authority contracts.
Exit: idempotent copy/retain/handoff/evict/reacquire, job-end and outage catch-up,
bounded pages, source-preserving interruption and actual old-revision restore.
Implementation and verification/validation/trace/review close separately.

## Desired state and private enrollment

Read-only namespace requirements reuse the admission compiler's exact flat-root
closure, active/history precedence and additive hard retention. Every registered
source contributes. The returned checkpoint is an observation, never a lease or
removal permission. Missing/opaque closure, conflicting size or work-budget failure
holds planning. Resolve policy within each namespace, then union physical place
assignments across the explicitly admitted namespaces.
This remains design-1 enrollment behavior. Explicit design-2 enrollment and fresh
roster-scope bindings instead use [roster precedence](ROSTER-PRECEDENCE.md),
preserving hard/additive obligations while resolving advisory history across the
whole admitted group. Mixed scopes refuse; existing enrollment is not upgraded.

An immutable private enrollment pins owner/authority, configured namespace IDs,
policy hashes, logical-to-physical place mappings, provider instance/generation,
declared failure domains, capacity authorities and existing bounded jobs. Cache
providers require the exact existing managed store roster. Every namespace policy
key is mapped exactly once; unknown namespaces/aliases/roots refuse. Retained
providers form an explicit floor for managed content; no retained provider is
automatically deleted. Inputs come from enrolled verified readers/providers,
never an event-supplied physical file path. Copy/eviction/reacquisition permissions
are independently enabled; all default disabled.

Plans bind the immutable enrollment and exact namespace checkpoint basis, bounded
object window, desired destinations, source set and budgets. Pagination is tied
to that basis; source changes invalidate continuation and require fresh planning.
Only explicit reconciliation resolves inventories; normal point lookup does not
census caches. Operational plans/cursors/receipts stay asset-side, outside Git.

## Existing mechanisms and stable effects

Publication/retention identities belong to the object/provider placement and
admitted controller scope, not each Git merge. Later runs reuse original accounted
receipts/grants under fresh byte verification. A merge/no-op/job end can change
desired placement without relabelling an old operation or duplicating payloads.
The fixed first jobs and existing history bounds remain required lifetime limits;
continuation/re-enrollment cannot silently reset original retry bindings.

Acquire missing required placements with the existing accounted bounded stream
and temporary retained ingest protection. Establish every configured retained
backup before handing off cache ingest retention through the existing owner-
authorized successor protocol. Namespace active-use protection continues after
that handoff. Verify/hold other required cache locations through conflicting
source retirement; retained keeper grants remain protected through the existing
retirement API. No directory/file deletion bypasses those APIs or accounting.

Routine cache eviction uses current all-namespace/provider fences and fresh
independent retained custody; labels or an old plan cannot authorize it alone.
New demand can block a previously eligible removal. Independent objects progress
without a cache-wide lock. An interrupted retirement resumes its original exact
intent, or owner cancellation preserves current needed bytes. Reacquisition is
an explicitly enabled fresh operation tied to the prior retired incarnation;
old copy/deletion retries cannot resurrect or remove a later placement.

Provider status separates unknown/unavailable, integrity incidents, partial
publication/retention, active retained custody, released ingest protection and
authorized absence. Known missing acknowledged data or settled evidence holds
recovery; reconciliation does not silently repair/reset it. Offline backup permits
safe copy-only progress, preserves temporary protection and blocks source eviction.
All effects reserve existing ledger metadata/payload/scratch budgets before IO;
prospective reclaim is never admitted space. Reports expose streamed copy bytes,
logical namespace unlink and pre-unlink file allocation separately. Repeated hash
verification IO is not aggregated by this first report; it must not be presented
as zero or included in a claimed total IO measurement. No measured free-space
increase or secure-erasure claim follows from a namespace unlink.

Final convergence requires a matching fresh desired basis and verified required
placements; held/uncertain/excess protected caches remain explained pending work.
A no-op Git selection never skips provider reconciliation. The Git adapter must
keep admitted selection, latest observed ref and physical convergence separate.
External trigger authentication, configured branch/draft/index integration,
native closure, actual provider/large-file qualification, final deletion, retained-
subset handoff, authority migration and operational lifetime remain required plan
work. This local composition grants no cloud retention or live-operation acceptance.

L0: fmt/validator/target hashes/diff and six producer lenses. L1: focused demand/
plan/admission/retry/refusal tests, locked default/all-feature workspace, strict
Clippy and standalone Git. L2: actual temporary Git selector -> three stores ->
job end/history demotion -> ordinary unlink -> revert/reacquisition; shared bytes,
offline backup, corrupt/missing evidence, fresh claims/readers, cancellation,
child exit and unrelated progress. Designed cases close only with exact evidence.

## Explicit committed-ref operator composition

`tig-lifecycle reconcile PROFILE REQUEST PLACEMENT_PROFILE [CURSOR]` composes
the existing committed-ref selector and one actual placement page. PROFILE and
REQUEST keep their existing strict schemas. PLACEMENT_PROFILE is private strict
`tig.placement-profile.design-1`: config, ordered providers (root, ledger_root,
minimum capacity checkpoint), namespaces (root, minimum source checkpoint),
byte_source and independently default-false allow_copy, allow_eviction,
allow_reacquisition and allow_cancellation. All file/path arguments are absolute;
documents are bounded regular JSON, never repository-selected physical paths.
Config owner/authority and one namespace/policy must match the Git enrollment.
Config seed is the canonical JSON digest of the exact byte_source profile.

The original plan/status/apply/resume operations remain selection-only. Describe
now advertises physical reconciliation separately. A reconcile call validates
private bindings before adopting selection, finishes/readbacks metadata, then
resolves current all-source requirements even after a no-op Git selection.
It checks the enrolled Git ref after selection and after provider work. Ref drift
requests catch-up without conflating historical selection or page observations
with the latest ref. This is explicit trusted operator invocation, not an
authenticated GitHub receiver, runner installation or production adoption.

Plans bind ordered provider IDs and exact run budgets. Enrollment/copy identity
normalizes provider/mapping order and excludes run budgets so changing page size
cannot mint another copy operation. Original provider jobs stay pinned; changing
enrollment scope still requires the separate authority/lifetime handoff contract.
CURSOR is the previous response's exact bounded cursor, saved outside Git. Source
changes invalidate it. Each call schedules at most 32 objects; no implicit loop
or background retry runs. page_converged covers only the returned page. When a
page has a predecessor/successor cursor, storage_converged remains unknown even
when that page succeeds. A caller needs a complete same-basis pass before it can
claim convergence of the entire selection.

All roots/ledgers must open at CLI entry. If an enrolled store is already
unavailable then, this first command reports provider_unavailable without
initializing or replacing it. The library can preserve copy-only progress when
an opened backup becomes unavailable; either path can catch up on explicit retry
after recovery. General offline enrollment/observation is not implemented here.
Missing acknowledged data/evidence holds; never reinterpret it as a new miss.
Interrupted private partials use the existing separately authorized cleanup API;
this controller does not sweep or discard them. Full unattended partial recovery,
provider qualification and aggregate verification-IO reporting remain open.

Completed final-absence refinement is controlled separately by
[PLACEMENT-FINAL-ABSENCE](PLACEMENT-FINAL-ABSENCE.md). It permits only read-only
recognition and no-effect handling of an unrequired, validated completed/settled
absent cache birth. Required destinations keep explicit final reacquisition;
provider formats and removal authority remain unchanged. Contract-only at entry.
