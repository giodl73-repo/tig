# Tig roadmap

Tig connects small Git-authored references to immutable bytes and explicit storage
obligations. It remains usable for Git/LFS without typed asset schemas, and across
unrelated projects without built-in media or cache policy.

The experimental source baseline is public. Its six-role review and ongoing
release checks are recorded in [public release](docs/PUBLIC-RELEASE.md).
This is source availability, not a stable product or full storage acceptance.

## Implemented experimental slices

- Pure identity/trees, bindings, claim handoff and bounded placement arithmetic.
- Local publication, scoped coordination, capacity journals, retained claims and
  recovery primitives with synthetic and real-client fixtures.
- Exact commit/index extraction, source journals and download-only Git LFS cache
  interoperability. Stores are explicitly enrolled, never discovered.
- Optional native records, schema admission, graphs, sparse variants, production
  contexts, revision/delivery history and sparse Git proposals.
- Explicit Rust project restore, handoff/progress evidence and history inspection.
- Local catalogue: work counts, nested navigation, search, versions, guarded
  metadata expansion, coverage, copy observations, placement preferences and
  work/version/object comparisons. Movement/removal remains a held review.
  Record impact exposes admitted consumer paths, coverage gaps and exact selected
  navigation through explicit relationship selection and source-fenced paging.
  Record comparison uses an exact in-memory baseline across projects, exposing
  independent identity/field/slot differences without opening payloads.
- Agent queries: versioned JSON discovery, inspection, admission diagnostics,
  search, byte/record comparison and explained impact through admitted graphs.
  Explicit work-scoped manifest references reuse nested adapters and verified
  readers, with source/content fences and document/field provenance.
  Manifest impact joins all matching owners to one admitted graph, separating
  association evidence from conditional native consumers and coverage gaps.
  Missing semantic profiles and excluded inventories remain explicit gaps.

Current contracts and [architecture](docs/ARCHITECTURE.md) limit each claim.
[Evidence](docs/vtrace/EVIDENCE.md) records historical checkpoints rather than
certifying full requirements or every consumer.

## Next delivery sequence

Next reconciliation integration: [completed final absence](specs/PLACEMENT-FINAL-ABSENCE.md)
will allow an unrequired, validated completed-deleted history copy to remain
absent after explicit backup reacquisition and working restoration. Required
stores keep their final-reacquisition fence. The private helper now passes13
focused SDK entries/3 actual exits, strict Clippy, all15 teaching regressions and81
controls; full profiles/source/artifact/hosted checks remain. The joined public
Git-return/acquisition/restore lesson is still required.

Next bounded teaching slice: [Git return after final loss](specs/TEACHING-GIT-RETURN.md)
will admit an exact old-version snapshot under a new Git/source revision and show
actual missing bytes and final-retirement holds with public seed access disabled.
The fresh-root ninth chapter now implements this flow with independently checked
absence, useful fixed-version demotion, real native HTTP/shipped controls and23
semantic refusals. All15/81 controls/both workspace+500k/strict/standalone/fresh
artifact and exact Windows/Linux/secrets qualify runtime81ecc27. Joined
acquisition/restoration and full teaching acceptance remain required work. Parent
reacquisition PR53 and Git-return PR54 are ready for review;
derive exact-head hosted qualification from GitHub.

Public teaching parity is now a required deliverable: a portable synthetic sample
catalogue carries the same assets through Git-style init/add/status/diff/commit,
branch/worktree, publish, merge, revert/restore and retirement lessons. Every step
needs an executable fixture, checked expected results, a failure case and a
catalogue view of actual engine plans and physical copies. Reuse existing
adapters, without a demo storage engine or private assets. See the
[teaching contract](specs/TEACHING-CATALOGUE.md) and
[lesson sequence](examples/catalogue/README.md). A first schema-gated tig-sample
runner now executes seven ordinary checkpoints with actual Git/source/provider
effects, independent copy assertions and a real catalogue enrollment. Focused
CLI/catalogue tests and strict Clippy pass. Optional recorded lesson playback now
reads sealed reports and shows before/desired/observed copies; qualification is
tracked in [its contract](specs/TEACHING-PLAYBACK.md). Further command/worktree,
remote-byte/failure, final retirement/lifetime lessons and separate
browser/user qualification remain required for teaching parity.

The [local Git exchange rehearsal](specs/TEACHING-EXCHANGE.md) adds actual
push/clone/fetch/pull over a fresh bare remote, preserved dirty-pull refusal and
three existing-engine placement checkpoints in the same recorded catalogue.
It separates metadata transport from explicit source adoption and byte placement;
remote byte publication/offline backup and full course acceptance remain open.

The [worktree/index rehearsal](specs/TEACHING-WORKTREE.md) adds an independent
linked checkout and captured index, staging/unstaging, dirty-removal refusal and
job protection that survives checkout closure. Twelve actual sealed checkpoints
use the existing source and placement APIs; the associated Git commit is distinct
from contributing index/draft/job snapshots. General native writer/adoption and
complete command/failure/final-loss/expiry teaching remain open.

The [backup-outage rehearsal](specs/TEACHING-BACKUP-OUTAGE.md) adds actual held
merge/job-end states, explicit unknown M coverage, preserved old F copies and
same-plan catchup after custody revalidation with seed reads disabled. Versioned
partial reports distinguish known copies from unknown coverage in the same
read-only catalogue. Hardware faults, remote byte publication and full failure/
restart/final-loss/expiry teaching remain separate open gates.

The [stale-plan rehearsal](specs/TEACHING-STALE-PLAN.md) explicitly admits a new
job after planning demotion. The existing SDK rejects that stale apply before
storage effects; separately recorded status shows the changed protection basis.
Fresh plans retain the job's F copy and demote only after its explicit end.
Ten recorded checkpoints expose historical desired versus observed state in the
same read-only catalogue. General races, crash/restart and final-loss/expiry
teaching remain separate open gates.

The [final-loss teaching contract](specs/TEACHING-FINAL-RETIREMENT.md) now has
fresh final-capable enrollment, exact manual and native category/retained-age
rehearsals, separately admitted claim/final consent, actual loss, cancellation,
intent resume/process recovery and same-hash new-birth replay protection.
Distinct bounded-bulk and paged readers reconstruct actual SDK scope and show
before/selection/observed copies, exclusions, new/historical logical removal and
recorded file allocation; free-space change remains unmeasured. Fourteen teaching
cases,13 manual/17 paged report negatives,8 malformed recovery frames,79 controller
cases, both workspace/500k profiles, strict checks and a fresh eight-step artifact
pass locally. [EVID-121](docs/reviews/2026-11-10-teaching-final-retirement.md)
separates this synthetic checkpoint from complete teaching, wider crash/provider/
authority/clock/large-file qualification, metadata lifetime and human acceptance.
Exact-head hosted checks remain required. This does not complete priority7 or the
full product plan.

The [reacquisition chapter](specs/TEACHING-REACQUISITION.md) now exposes actual
same-hash ingest/retention and old deletion retry as a ninth readonly checkpoint.
It captures SDK receipt accessors and validates new-birth/source/grant/capacity/
replay bindings, with truthful copy/ack/charge observations. Fourteen teaching
cases/strict checks,30 repinned acquisition negatives and81 controller cases pass
locally; broader/fresh/hosted qualification remains pending. Git return after
final loss, complete teaching and lifetime remain open.

The [same-byte rename rehearsal](specs/TEACHING-RENAME.md) adds a real conflicting
destination refusal and successful Git record rename with unchanged blob/asset
identity. Explicit native adoption and SDK reconciliation preserve verified
copies with zero streaming/provider effects. Eight recorded checkpoints use
the same catalogue; qualification is tracked separately in EVID-120.

The [paged operator candidate](specs/PAGED-OPERATOR.md) now exposes explicit
existing-root copy-index staging, whole-selection planning, original scope pages,
witness peak/status and separately approved apply/resume/cancel through
`tig-storage paged`. It reuses the existing controller and shares metadata
budgets across finish/readback. Synthetic20-page manual and actual-clock/registry
workflows, native executable handoff, broad/default/all-feature/500k/strict Clippy
and standalone checks pass locally. Hosted qualification remains separate.
This advances the operator surface, while native writer/current authority,
agent/UI/Action, provider/restore/large-file/lifetime/metadata-expiry and intended-use
acceptance remain necessary for full delivery. See context/RESUME.md and the
[operator guide](docs/guides/PAGED-OPERATOR.md).
Originally held copies also need a fresh approved follow-up after selected
history settlement, for example when an offline provider returns or another job
ends. Preserve the original scope/receipts for discovery and recheck current
protection; neither expanding an old decision nor recreating retired history is
an acceptable substitute. This remains an explicit priority-7 delivery gate.
The [held-copy follow-up](specs/HELD-COPY-FOLLOWUP.md) now implements a new
copy-only candidate/runtime and explicit default-disabled CLI over original
provenance, current content-wide protection and new final-loss approval. Focused
recovered-provider/ended-job, cancellation/race/process and fresh CLI qualifiers
pass; local broad, final artifact and exact-head hosted qualification pass. No source
history is recreated. Native/current-authority, provider/lifetime and complete
block-retirement acceptance remain open; see [the guide](docs/guides/HELD-COPY-FOLLOWUP.md).

The indexed native selection candidate now connects multiple archived historical
roots and large block/category scopes to existing paged requests. Explicit
index-paged/select-paged commands stage metadata only, default-disabled, with
independent finished-provider readback and shared whole-call budgets. Synthetic
aged17-root execution preserves shared/unselected bytes;1000 native assets stage
all4000 bindings without an inline inventory. Read
[the entry contract](specs/NATIVE-PAGED-RETIREMENT.md) and context/RESUME.md for
local qualification and pending hosted checks. Operator/agent/UI/Action execution and actual native
writer/current authority/time/provider/restore/large-file/lifetime/metadata-expiry/
human adoption remain necessary for the full delivery.

The explicit `paged::control::aged` Rust candidate now freezes retained original
occurrence evidence, rechecks conservative clock eligibility and acknowledges
exact survivor inheritance before completing each source page. It preserves
manual framing and shares budgets through existing age/source/provider ports.
Read-only registry-aware status clarifies interrupted age acknowledgement without
writing or sampling time; cancellation preserves unresolved debt. The new
[entry/review](docs/reviews/2026-11-09-paged-age.md) records broad pre-tightening,
final changed-scope and artifact qualification; exact-head hosted checks remain.
Native selectors and operator/agent/UI/Action
composition, actual current authority/writer/time/restore/provider/large-file and
lifetime/metadata-expiry/human acceptance remain full delivery obligations.

The manual `paged::control` candidate now composes frozen prepared plans with
separate historical-claim/final-copy approvals, durable global/page witnesses,
actual receipt-checked progress and original provider copy plans. Source-metadata-
offline cancellation records stop before cleanup and reports unknown debt.
[Paged execution](specs/PAGED-EXECUTION.md) controls its synthetic qualification;
full age-aware/native/operator/UI/Action and operational acceptance stay required.

The [paged-execution entry](specs/PAGED-EXECUTION.md) now includes persisted
conditional preparation through actual source-state simulation and append-only
inherited age proofs. EVID-108 separates source receipt/projection/compatibility
qualification from the durable global/page controller still to be composed.
Prepared metadata and predicted checkpoints grant no approval or execution proof.

The [paged-retirement contract](specs/PAGED-RETIREMENT.md) addresses NB-07's large
shared-root selection. Its first layer freezes exact asset-side selection and
hypothetical protection across the whole scope, then emits deterministic original-
binding pages. Separate durable claim/final-copy decisions, survivor/age lineage,
page recovery and actual final execution remain required; a manifest or cursor
grants no authority and does not close the block-retirement milestone.

The [age-witness contract](specs/AGE-WITNESSES.md) and
[operator guide](docs/guides/AGE-WITNESSES.md) add conservative observations for
exact historical occurrences, verified age inheritance through owned subsets and
separate native age-aware candidates. EVID-106 records local/hosted qualification;
the mechanism grants no final consent and does not close paged shared-claim settlement,
authority/writer/time adoption, UI/Action, provider or operational lifetime gates.

The [native binding adapter](specs/NATIVE-RETIREMENT-BINDINGS.md) compiles exact
declared native storage closure into independently aliased flat bindings. The
metadata-only bind/select CLI maps exact blocks and admitted definition categories
to existing bulk selections. EVID-105 records local qualification. Recorded age
is a separate admitted protocol under EVID-106; no input timestamp or Git date grants it.

The [bulk execution interface](specs/BULK-EXECUTION.md) composes exact private
plan approval, explicit enrolled clock and existing apply/resume/cancel. Inspection
permissions remain separate; cancellation does not need source metadata or fresh
time. EVID-104 records local qualification; hosted checks remain separate.
Native selectors/UI/Action, current authority,
large files/providers, lifetime and eventual metadata retirement remain required.

The explicit [local clock profile](specs/LOCAL-CLOCK.md) is the first candidate
for final-deletion time admission. It reuses bounded journals and checks the whole
time interval. Native OS trust, current authority/minima, restoration/lifetime and
operator mutation integration require separate qualification and adoption.

The [bulk inspection commands](docs/guides/BULK-RETIREMENT.md) expose exact bounded
block planning, metadata peak and writer-free historical status. They add no bulk
apply permission. Final operator execution, native selectors, current clock/
authority, provider/large-file and operational-lifetime gates remain open.

Planning input f742abf7f62b3c4c9c65149a404626679a004f71. This sequence
prioritizes usable operations over additional catalogue decoration. Estimates
and provider capabilities must be measured per admitted scope. No live storage
action is authorized by this roadmap; fixture work uses isolated temporary roots.

| Priority | Deliverable | Demonstrable exit |
| --- | --- | --- |
| 1 | Finish current catalogue changes and user UX test | Protected checks pass; user can find a work, compare versions, trace an owner and return without losing context; task failures become regression fixtures |
| 2 | Unified storage plan and agent contract | Immutable plan/operation identities, enrolled place generations, object set, source/policy/claim basis, capabilities, budgets, authority and holds; explicit complete protection scope before removal; same plan powers CLI and UI |
| 3 | Recoverable local copy orchestration | Plan/apply/status/resume in temporary stores; bounded streaming, destination staging, hash/length readback, retained destination and restart-safe acknowledgement; source preserved |
| 4 | Guarded local eviction | Require complete scoped source/job/draft claims from priority 2; hold overlapping protection through revalidation/removal, retain required independent domains, and prove exact-copy recovery; report file/allocation results separately |
| 5 | Explicit cloud and Git LFS upload adapters | One selected remote provider first, then LFS upload; retry, failed/lost acknowledgement, corrupt/missing readback, credentials and unsupported retention capabilities exercised; no remote retirement from mere upload success |
| 6 | Git-driven lifecycle reconciliation | Commit/branch/merge/revert produce idempotent desired-placement deltas; protect drafts/jobs and retained revisions; event replay and missed events converge without global locks |
| 7 | Bulk asset retirement and final deletion | Block/age/category selection of exact superseded versions; preview shared/protected content and affected stores; separately authorize eligible last-copy deletion; resume batches and measure reclaim |
| 8 | Metadata retirement and operational lifetime | Bounded deletion receipts and retention windows; authority/replay fences permit eventual record retirement; journal continuation preserves witnesses, generations and original retry outcomes |
| 9 | Scale, onboarding and release | Derived incremental indexes keyed to exact sources/rules, measured warm lookup and sparse updates, generic sample enrollment and operator/agent examples; rebuild survives deleted derived state |

UX testing can run in parallel with priorities 2-3. Storage selection should show
unique content, copy bytes, scratch peak, retained obligations, estimated eligible
bytes and measured results separately. Add grouping/filter changes only when they
help answer a concrete task such as why one work grew or what blocks reclamation.
Never count prospective eviction as free space for an admitted job.

### Merge-driven placement and automatic eviction

Target: a merge to main selects a new released version and an explicitly enabled
GitHub Action requests reconciliation of that exact commit and admitted policy.
Project terms such as golden are configuration, not built-in lifecycle enums.
Abstract roles map to enrolled stores outside Git:

| Admitted lifecycle state | Example required roles |
| --- | --- |
| Current released selection | Working + retained backup |
| Superseded with no remaining active use | History + retained backup |
| Required by another selection, branch, draft or job | Union of all admitted placement/retention demands |

Resolve lifecycle preferences before taking that union. Under the example policy,
history proximity applies only when an object has no active use: unchanged bytes
shared with the new release, or old bytes still needed by a job, do not acquire a
history-store requirement merely because a predecessor root mentions them.
Explicit hard retention/location obligations remain additive and cannot be
suppressed by active-use precedence. Configure and explain this distinction;
another project may deliberately choose additive history placement instead.
The explicit [roster-precedence profile](specs/ROSTER-PRECEDENCE.md) addresses
active use held in another enrolled namespace. It requires fresh roster-scope
store bindings and design-2 placement enrollment; legacy namespace-local policy
remains unchanged. Planner and final ordinary-eviction assessment must agree on
this group, without suppressing hard retention or explicitly additive history.

Acquire and verify new copies, establish required retention, then recheck current
protection and evict eligible old working copies. Routine eviction can execute
without per-copy confirmation under an explicitly enabled bounded policy. Merge
success and storage convergence are separate: unavailable stores produce pending
or held work while preserving protection. Retries resume stable operation IDs.
The configured executor must reach enrolled stores; a hosted Action does not
itself gain access to local disks. No runner, hook or daemon is installed by this
plan. Duplicate, delayed and missed events must converge; an old event cannot
override a newer admitted selection. This is a target capability, not implemented
automatic reconciliation. The executor authenticates the repository/ref/event
and applies independently admitted policy/authority. Merged project content or
workflow code cannot grant itself broader deletion permissions; untrusted PR jobs
receive no storage credentials or execution authority. Policy upgrades require
explicit trusted adoption, including when authored policy lives in Git.

### Bulk final deletion and eventual metadata retirement

Backup capacity is finite. Distinguish replica eviction, asset/version retirement,
final content deletion and metadata retirement. Superseding a selection does not
end another consumer's demand. Final deletion deliberately ends recoverability
for authorized exact content; it is not demotion or a consequence of low space.

Users can select a block of old assets/versions in the catalogue, or supply the
same bounded selection through CLI/agent operations. Categories and lifecycle
states come from admitted definitions. Age filters use a recorded supersession/
retirement event and explicit clock policy, never file modification time. Unknown
age holds age-based eligibility; restored/current use invalidates eligibility,
and retention deadlines follow explicit policy. Age permits review, not deletion.
The retirement plan explicitly settles the selected owners' exact historical
retention claims under their authority before payload deletion becomes eligible.
It cannot withdraw another owner's protection or override non-releasable holds.
A pointer remaining in Git history is not itself a permanent byte-retention claim;
which historical revisions stay protected is explicit admitted policy. These
claim/source transitions and deletion are recoverable steps, not one transaction.

A selected block can be only part of a protected root. Before retiring that root's
obligation, resolve and acknowledge a replacement that protects its unselected or
held members, with exact source-revision fencing. Never withdraw a whole mixed
root to make one selected leaf eligible. Interruption may leave extra protection;
it must not leave surviving members unprotected. This subset handoff needs its
own implementation before bulk retirement can use whole-root journal transitions.

Preview binds exact versions, object identities and source/policy/claim basis.
Show eligible and held items with reasons, sharing outside the batch, affected
stores, loss of historical recoverability, and estimated versus measured reclaim.
Separate asset counts, unique bytes, copy bytes and allocated bytes. Shared objects
remain while any surviving admitted obligation protects them. Incomplete scope
holds deletion. A capacity shortfall can propose retirement but cannot silently
shorten retention or grant final-deletion authority.

Explicit batch authorization, or a separately enabled bounded final-deletion
policy, permits only the eligible exact scope, including last retained backup
copies. Revalidate authority/protection under effective final fences per action.
Partial progress, held/failed/uncertain outcomes, cancellation and restart remain
visible; no implicit all-or-nothing batch promise. Offline or unenrolled stores
remain unresolved, not reported deleted. Provider retention locks can hold removal
even after authorization. The first eviction executor still excludes final deletion;
priority 7 requires separate entry/exit and qualification. Persist exact authorized
retirement intent before destructive IO and reserve metadata capacity for intent,
completion and recovery first. New protection admission and removal share the
same effective fence; a crash cannot leave a deletion with no recoverable intent.
If evidence cannot be durably recorded at the required level, hold removal.
Before that executor is implemented, freeze cancellation after historical-claim
settlement: either restore the admitted protection through owner-authorized
recovery or explicitly report retired-but-present content and its remaining
protection. Never report an unqualified rollback. Batch approval binds its exact
basis and validity window; changed selection/use, policy or authority invalidates
affected pending removals. Revalidate each action, preserve historical completed
effects, and test cancellation and invalidation on both sides of claim settlement.
Report removal from the admitted namespace separately from provider-version,
snapshot or trash retention and actual capacity release. Unlinking is not secure
erasure and does not prove deletion of unknown copies outside managed scope.

Current authored .tig records can be removed through explicit source adoption;
old Git commits may retain pointers without retaining payloads forever. Keep a
compact exact deletion receipt/tombstone for the admitted audit/recovery window;
detailed evidence stays outside Git under its own budget. Historical lookup reports
deliberate retirement when evidence supports it, otherwise unknown/unavailable;
a pointer never promises recoverability. Metadata retirement is separately
permitted after dependencies and required retention end and stale operations can
no longer act. Define an enforceable authority/epoch or equivalent replay fence
before forgetting tombstones; elapsed time alone is insufficient. Old peers and
restored backups must not revive retired writes. Permanent per-asset records are
not required, and Git history/old receipts are not rewritten. The final protocol
and encoding remain to be designed and verified. Tombstones fence exact retired
operations/authority generations, not a permanent ban on a content hash. Fresh
explicit ingest of identical bytes may create new protection; old deletion retries
must not remove that new placement. A Git revert after final deletion reports
unavailable content until explicit reacquisition succeeds, never silently choosing
other bytes. Define the required audit window against SAFETY-PROMISE.md before
metadata retirement is implemented. A compact trusted authority/epoch fence may
outlive per-asset records; forgetting all replay protection is not promised.

### Storage orchestration contract

```text
Git snapshot + drafts/jobs + placement/retention policy
                          |
                      explicit plan
                          |
              copy -> verify -> retain destination
                          |
                  recheck scoped protection
                          |
                authorized source-copy removal
                          |
               fresh capacity/copy observation
```

These are target operator concepts, not newly implemented CLI commands. Use
existing identity, publication, ledger, retention, source-claim and provider
primitives. The adapter declares capabilities: readable, writable, resumable,
verifiable, retainable, removable, and enforceable protection where supported.
Unsupported guarantees produce explicit holds. Keep operational journals and
large plans outside Git; bounded source definitions and policies remain in Git.

Plan identity binds exact enrolled store instances/generations, object hash and
length, source snapshots, policy/claim checkpoints, provider contract and finite
budgets. Stable operation IDs bind retries to their admitted action; conflicting
reuse refuses. Changed inputs require a new plan without discarding verified
useful copies. Capability admission records acknowledgement strength, integrity
proof, retention enforceability and failure-domain provenance, not just booleans.

Priority 2 must establish bounded complete protection scope for the first local
executor: all admitted branches, drafts, jobs and retention claims, with explicit
handling of unmanaged writers/pruners. Manual explicit enrollment can provide
that scope before automatic Git reconciliation in priority 6. Gaps hold removal;
copy-only progress can continue. Claim publication and removal must share an
effective scoped fence, including the final destructive step; a check followed
by an unprotected delete is insufficient. Unrelated additions remain concurrent.
Admission names who can register new demand, what invalidates complete coverage,
and how unavailable authorities block removal. Simultaneous retirements in two
stores cannot each rely on the other's copy: required surviving custody remains
protected through the destructive step under effective provider-specific fences.

Bind each removable store scope/generation to every authority namespace that can
protect it; absence in one guarded journal is insufficient when another namespace
can admit demand for the same copy. Define authority/policy epoch handoff before
enabling removal: fence old removal permissions, preserve acknowledged demands
through the successor's adoption, and refuse deletion during an unresolved handoff.
Creating a fresh epoch does not transfer protection or invalidate old executors
by itself. Qualify the transition without a global cache lock, including old-peer
replay and a new claim arriving through either authority at the boundary.

Define precedence for conflicting proximity/retention demands and report
unsatisfied constraints. Reserve source/destination metadata, payload and scratch
peak without borrowing prospective reclaim. Retain the required replica count
across admitted independent failure domains; aliases cannot satisfy redundancy.
Verified availability alone does not qualify retained custody.

Cache, remote object store and Git LFS endpoint are separate provider roles.
Publishing a Git pointer does not prove its bytes reached a remote; successful
transfer does not establish an independently retained backup. Removing one cache
copy differs from deleting an object's final archive. Both use exact byte
identity, but have different obligations and authority.
The first executor excludes final-archive deletion. Operator-enabled apply is
bounded to the admitted plan/provider policy, defaults to dry run, and supports
status/cancellation/recovery. Cancellation stops future destructive work while
preserving valid copies and unsettled protection. Credentials and private root
paths stay in machine-local profiles; portable exports omit secrets. Reject
symlink/path escapes and replaced store roots before provider IO.

### Replay, large-object and journal lifetime gates

Before eviction implementation, reconcile publication replay with intentional
absence: the existing publication contract treats missing acknowledged objects
as integrity failures. Durable authorized retirement evidence must distinguish
absence from damage, fence stale retries and prevent silent resurrection. Specify
explicit reacquisition/new-operation behavior separately.

The reference publication adapter's 8 MiB cap is bounded component proof. Before
large-file provider adoption, qualify representative large synthetic objects with
finite memory, throughput/verification budgets, cancellation and disk-full
recovery. Raising a cap alone is insufficient. Before sustained operation, design
journal continuation/checkpoint compatibility: closed working-set retirement does
not remove command/byte lifetime bounds. Preserve authority, witnesses, generations,
original receipts and conflicting-retry fences across continuation, including
interrupted transitions and old-peer refusal. Lifetime budgets and safe exhaustion
behavior belong in priority 2; implement continuation before sustained operation
even though metadata retirement is listed later. A full journal must never force
an unrecorded destructive step or a ledger reset.

### Fixtures already present and the missing end-to-end proof

| Existing evidence | What it covers | What it does not establish |
| --- | --- | --- |
| tig-store publication/accounted tests | Local immutable publication, bounded resource accounting, interruption and private partial cleanup | Arbitrary final-object eviction or cloud transfer |
| tig-store replica tests | Completed hash/length verification, object guards, alias/domain refusals and stale evidence | Durable independent backup by itself |
| tig-store retention and handoff tests | Durable scoped claims, restart and acknowledged ownership handoff | Source payload deletion or complete project coverage |
| tig-store metadata_copy_cli tests | Explicit plan/apply and fresh-process metadata-store verification with source preserved | General media/payload transfer |
| tig-git client tests | Real Git LFS current/prior checkout, parallel clients, corruption/retry and selected-source authorization | Remote upload, cloud custody or eviction |
| Catalogue storage tests | Placement/version/shared-byte classification and stale held previews | Executed cache movement/removal |

The next end-to-end fixture is a synthetic work with two unchanged inputs and one
replaced input, a delivered output, two branches and an active reader. Use three
temporary stores as hot/history/archive roles supplied by configuration. Advance
main, compute the delta, copy and retain required history, remove only eligible
hot copies, then restore the previous revision from retained storage. Compare
predicted unique/copy/peak bytes with observed results. No real drive names,
project vocabulary or user assets belong in this generic fixture.

Before enabling that removal path, test: interruption during copy; full target;
wrong hash/length; lost completion reply; duplicate retry; missing retained copy;
same-location aliases; new claim after planning; active reader; concurrent unrelated
writer; changed Git/policy basis; reverted selection; partial/unknown dependency
coverage; partial batch progress; retained destination going offline; and restart
after removal but before completion acknowledgement. Each case names the exact
permitted effects and proves surviving bytes, not only a reported success flag.
Also test conflicting policy/claim precedence, duplicate operation-ID mismatch,
stale or replaced store identity, credential denial, path/symlink escape,
understated scratch peak, shared-pack zero reclaim, cancellation between steps
and retention-domain aliasing. Pin finite workload/latency/peak-byte limits before
each implementation slice. Keep an append-only operation trail outside Git; a
fresh process must distinguish done, partial, held and uncertain outcomes without
blindly repeating destructive actions.
Then repeat the same contract suite against a local remote-provider test server
and real LFS client fixtures. Explicit opt-in provider qualification follows;
synthetic remote success alone does not qualify a cloud service.

Extend the planned suite with simultaneous cross-store retirement, protection
authority outage and coverage invalidation; duplicate/out-of-order merge events
and merge while backup is offline; historical lookup/retry after authorized
eviction and final deletion; mixed eligible/held/shared objects in a bulk batch;
unknown age, reactivated versions, retention lock and interrupted last-copy
removal; metadata expiry with old peers or restored backups; and journal
continuation pre/post-commit failures. Prove surviving protection for held content
and truthful intentional loss for authorized final deletion. Pin representative
large-object memory, latency and peak-byte limits. Also test same-hash re-ingest
against an old deletion retry, unauthorized claim release, journal exhaustion
before removal, trusted-trigger/policy denial and retained provider versions.
Implement the lifecycle fixture in stages: source-preserving copy first, guarded
eviction only after its gates, final deletion only under its separate contract.
All remain designed fixtures.

Additional review cases: unchanged objects shared by current/predecessor roots
avoid unnecessary history placement under the configured precedence; a live job
keeps its old bytes active while hard history retention remains additive; mixed-
root subset handoff survives interruption without dropping held members; and
namespace/epoch migration refuses stale removal while preserving new admissions.

Trace: priorities 2-4 map to WP-003/004/006/007/008, priority 5 to WP-005/013,
priority 6 to WP-009/011/012, priorities 7-8 to WP-003/004/006/007/008/009/013/015,
and priorities 1/9 to WP-008/015/016. Each slice
requires exact-source controls, focused failures and protected checks. Optional
torrent transport, broad WASM parity and additional presentation features follow
the working local orchestration slice; they are not prerequisites for it.

The [six-role review](docs/reviews/2026-11-08-storage-roadmap-roles.md) records
design corrections and the implementation proof still required.

Native selection review adds NB-07 in the
[native binding review](docs/reviews/2026-11-09-native-retirement-bindings.md):
when shared content appears in more historical roots than one bounded batch can
select, every page remains held by the others. Larger-block delivery therefore
needs exact global consent and a separately admitted historical-claim-only
settlement phase before final byte removal. Repeating current bounded final
batches is insufficient. Preserve held/current/foreign obligations and original
deletion consent; qualify age inheritance and interruption across those pages.

## Full outstanding obligations

| Milestone | Exit condition |
| --- | --- |
| Consumer completeness | Explainable declared versus consumed inputs, opaque gaps, active jobs and exact profile/source coverage |
| Lifecycle reconciliation | Durable scoped demands and idempotent plans across Git changes, interruption and concurrent work |
| Guarded execution | Independent retained custody, provider revalidation and authority before retirement; physical reclaim measured in isolated fixtures |
| Final deletion and metadata lifetime | Exact bulk scope, protected shared content, explicit last-copy authority, resumable outcomes and eventual metadata retirement with stale-replay fencing |
| Catalogue usability | Comparable coverage, shared bytes, versions and primary subtypes; explicit enrollment and tool handoff |
| Providers/transfers | Capability/operational/license review and separate recovery/custody proof for remote or optional torrent adapters |

No deadline or completed consumer migration is implied. Catalogue work can advance
independently of deletion. Do not create a second cache engine or global cache
lock to complete a milestone. Estimates, observations, protection and authority
remain distinct.

The [17 work packages](docs/vtrace/WORK_PACKAGES.md),
[catalogue roadmap](docs/CATALOGUE-UI-PLAN.md) and
[adoption gates](docs/CONSUMER-ADOPTION.md) provide finer allocation.

## Storage implementation checkpoint

The [first orchestration slice](specs/STORAGE-ORCHESTRATION.md) adds the shared
bounded plan and explicit local CLI/library copy/retention path, built on existing
publication/ledger/retention. Exact source/policy/claim/authority basis and caller
admission are bound into replay. Protection completeness remains an assertion;
all removal stays held. This advances parts of priorities 2-3, not their full
exit or the roadmap as a whole. Catalogue apply, complete claim admission,
guarded eviction, remote upload, automatic lifecycle reconciliation, bulk final
deletion and metadata retirement remain required subsequent work. Journal lifetime
and representative large-file qualification retain their earlier entry gates.

Priority-2 prerequisite: [protection demand snapshots](specs/PROTECTION-DEMAND-SNAPSHOT.md)
now compare all registered versus admitted sources and derive exact assigned demand
from declared flat binding trees. This replaces no source authority or provider;
unknown native/opaque closure remains a gap. Snapshot freshness is not a lease,
and the View adapter holds removal. Effective source-admission/object-removal
fencing and intentional-absence recovery remain the next executor gates.

[Mandatory source admission](specs/MANDATORY-SOURCE-ADMISSION.md) now provides an
explicit guarded-journal mode with immutable source classes/policy and compatible
scoped claim/subject fences. This is a cooperative single-namespace prerequisite;
it does not cover other authorities or attach a byte provider. Store-scope binding
and authority handoff, intentional-absence/incarnation recovery and retained
custody remain required before guarded eviction. Full workflow delivery remains
open; see the separate EVID-086 verification and producer-review scope.

[Store protection binding](specs/STORE-PROTECTION-BOUNDARY.md) now pins an explicit
namespace/place roster and generation in fresh local stores. Assessment requires
every enrolled namespace, shares aggregate budgets, and holds all subject/provider
fences while respecting readers and local retained claims. It grants no removal
authority. The first roster is immutable; acknowledged epoch/roster handoff,
authorized-absence/incarnation recovery and retained custody remain necessary
execution gates. Existing stores are not migrated or enrolled implicitly.

The [managed local retirement profile](specs/MANAGED-REPLICA-RETIREMENT.md) now
executes bounded ordinary replica unlink under all-namespace/provider fences and
fresh retained-survivor verification. Durable intent, cancellation, exact retries
and explicit new-operation reacquisition fence later same-hash placements. The
synthetic lifecycle publishes replacement/backup, holds old active job use, then
evicts/restores exact prior bytes. This advances the local mechanism in priority 4;
its CLI/agent/UI contract, large-file/native qualification and live adoption are
still required. No automatic merge action, remote upload, final-copy deletion,
metadata expiry or dynamic epoch adoption follows from this component.

The [operator/agent retirement interface](specs/RETIREMENT-AGENT-CLI.md) now
exposes plan/status/apply/resume/cancel/reacquire over that local mechanism.
Private profiles independently pin owner/context, survivor grants/domains, role
assignments and finite budgets. Execution permissions default independently to
disabled. The first policy conservatively retains every configured survivor;
it is not the automatic Git lifecycle policy selector. Fresh status and historical
outcomes remain separate, including after same-hash reacquisition. Catalogue apply,
remote/LFS upload, trusted exact-ref event convergence, bulk final deletion,
metadata expiry and operational lifetime gates remain required roadmap work.

Priority-5 first backend: the optional [S3 publication adapter](specs/S3-PUBLICATION.md)
now composes existing guarded local reads and accounted intent/completion witnesses
with conditional PUT/full readback. The explicit CLI uses independently admitted
private access and bounded operation identities. Actual loopback protocol/process
fixtures exercise retry, lost reply, denial, collisions, corrupt/missing readback,
budget/cancellation and source preservation. This establishes the bounded protocol
adapter, not real-cloud custody/retention, remote-quota enforcement, large-file
qualification or the full priority-5 exit. Git LFS upload and the remaining exact-ref
lifecycle, bulk final-deletion and metadata-lifetime requirements remain open.

Priority-5 Git LFS adapter: optional [standalone upload](specs/LFS-UPLOAD.md)
now composes the existing exact OID authorization and verified reader with Basic
Batch/conditional PUT/optional verify/full download readback. Explicit operator
commands and real Git LFS push fixtures exercise bounded private admission,
lost replies, failed verify, credential scopes, interrupted processes and source
preservation. Shared accounted witnesses retain S3 wire compatibility. This is
an 8 MiB experimental protocol adapter; real-server/provider qualification,
remote quota, custody and large-file/lifetime gates remain open. Next required
product slice is trusted exact-ref lifecycle convergence, followed by bulk final
deletion with retained-subset handoff and eventual metadata retirement.

Priority-6 selection prerequisite: [Git lifecycle reconciliation](specs/GIT-LIFECYCLE-RECONCILIATION.md)
now binds an exact enrolled committed ref, policy and expected source revision to
verified persistent metadata and atomic desired-root adoption. Existing
Prepare/Adopt commands commit together with no format change. Explicit library/
CLI plan/status/apply/resume handles no-op duplicates, stale events, reverts,
active jobs and historical receipts; observations do not assert provider convergence.
The remaining priority-6 exit still requires automatic copy/retention/ordinary
eviction composition, independently trusted trigger integration, configured branch
withdrawal/draft/index lifecycle and end-to-end provider failure fixtures. This
prerequisite does not replace those deliverables or complete the milestone.

Priority-6 local composition: [placement reconciliation](specs/PLACEMENT-RECONCILIATION.md)
now reuses exact namespace requirements and existing accounted/retention/handoff/
retirement/reacquisition adapters. Explicit `tig-lifecycle reconcile` adopts the
enrolled committed ref, then reconciles one bounded page even for a no-op selection.
Stable copy identities survive later source changes and run-budget adjustments.
Temporary-store fixtures exercise replacement, job-end demotion, shared bytes,
backup outage/catch-up, reader and late-claim holds, process exit, exact old-byte
reacquisition, pagination and CLI defaults. EVID-093 separately records local
qualification, synthetic validation, trace and six-role producer review; hosted
current-head checks remain separate. Trusted external triggers, branch/draft/index/native integration,
unattended private-partial recovery, offline enrollment, aggregate verification IO,
large-file/provider qualification and lifetime remain required. This does not
complete priority 6 or enable live storage operations.

Priority-6 bounded trusted intake: optional
[GitHub request execution](specs/GITHUB-TRIGGER.md) connects signed OIDC intake,
independent current GitHub source metadata and a bounded placement pass. Its
foreground loopback receiver and inert Action client reuse private placement
permissions; workflow code receives no executor policy or OS authority. Signed
synthetic HTTP fixtures cover nonempty three-store placement, expiry/clock holds,
source outage and post-copy head drift. EVID-094 separately records broad/final-
focused local qualification, synthetic validation, trace and six producer lenses.
A real hosted-issuer/foreground-CLI fixture passes with zero payload effects;
hosted current-head checks remain separate. Full branch/draft/index/native
integration, external ingress/OS isolation, provider/large-file/lifetime and later
final-deletion/metadata milestones remain required. No service or live cache is attached.

Priority-6 bounded staged-index slice:
[index lifecycle](specs/INDEX-LIFECYCLE.md) derives a draft obligation set from exact
bounded Git index pointers, including every known conflict alternative without
publishing a merged content selection. It reuses metadata finish/readback, guarded
atomic adoption and one ordinary placement page. Fifteen focused entries include
one child helper, two actual abrupt exits, linked worktrees and a real CLI/store
job-end demotion flow. EVID-095 separately records passing local default/all-feature
workspace, strict Clippy/artifact qualification, synthetic validation, trace and
six producer lenses. Hosted current-head checks remain separate.
Branch rename/withdrawal, automatic commit/draft settlement, arbitrary private
overlay/native coverage and the later full roadmap obligations remain required.

Priority-6 owned settlement: [source settlement](specs/SOURCE-SETTLEMENT.md)
adds exact local branch-absence and adopted-successor draft withdrawal with
transactional source-revision conditions. Ordinary placement can catch up after
no-op settlement; history and other jobs remain protected. Focused tests include
two abrupt exits and actual staged -> committed -> branch-end -> job-end demotion.
EVID-096 records local default/all-feature, condition/old-reader/CLI qualification,
synthetic validation, trace and producer review; hosted current-head is separate. Automatic
event routing, durable rename/controller revocation, native/private coverage and
later final-deletion/metadata/authority/large-file/lifetime gates remain open.

Priority-7 retained-subset prerequisite: [partial history settlement](specs/RETAINED-SUBSET.md)
now composes sparse edits, finished/read-back metadata and same-owner/class
survivor protection before exact original predecessor retirement. Atomic
Prepare/Adopt/Withdraw avoids transient active-use suppression; original revision
and survivor conditions fence the final claim transition. The default-disabled
tig-source subset CLI supports plan/status/apply/resume over existing private
roots. EVID-097 records focused/process/CLI/old-reader qualification and six
producer lenses separately from implementation. This releases owned obligations,
not payloads. Bounded bulk age/category/native selection, final-deletion intent/
authorization/execution, cancellation recovery, actual reclaim, metadata expiry,
authority/lifetime and remaining full lifecycle/adoption gates remain required.
Before the last-copy executor, freeze owner-authorized provider-retention grant
release as well as source settlement, and integrate deliberate absence with the
retained-placement floor. Ordinary keeper/handoff requirements must remain intact;
lost source demand alone neither releases retained backups nor permits old
reconciliation to resurrect explicitly retired content.

Priority-7 final-retirement boundary: [the final-retirement contract](specs/FINAL-RETIREMENT.md)
now separates source settlement, provider grant release, final unlink/cancellation
and eventual metadata retirement. Its first implementation is content-wide live
claim clearance across admitted namespace/policy/minimum pins, independent of
place-local surplus. A job on F can protect the only recovery bytes on M even
without an M assignment. Exact stronger minima, aggregate metadata bounds and
scoped fences have synthetic/process/restore/vector evidence in EVID-098.
The live guard grants no provider release or removal. Canonical final-provider
formats and old-writer refusal, exact owner consents, budgeted durable intent/
release/completion/cancellation, deliberate-absence placement, bulk selectors,
actual reclaim and all lifetime/adoption gates remain required, not implemented
by this prerequisite. FR-01..15 distinguishes current fixtures from later gates.

The next bounded layer, [FINAL-PROVIDER](specs/FINAL-PROVIDER.md), is in progress:
fresh explicit design-8 LocalStore, exact owned grant release, durable final
intent/terminal, cancellation, deliberate absence, separately authorized fresh
same-hash acquisition and read-only status. It reuses existing scoped guards,
capacity and accounted streaming. Ordinary placement holds final-retirement
states; its copy permission cannot silently recreate them. Format vectors and
actual old-writer refusal have separate qualification. This library slice does
not complete priority 7: batch consent/source settlement, age/category/native
selection, operator surfaces, authority/provider/large-file qualification and
metadata lifetime remain required before full intended-use acceptance.

[BULK-RETIREMENT](specs/BULK-RETIREMENT.md) now composes the bounded Rust path:
exact historical flat bindings, held/eligible projection, sequential same-source
subsets, durable accounted consent and final-provider actions. Current source
history is checked under final live fences; a prepared/aborted new use invalidates
pending approval. Cancellation preserves honest source and byte effects, and
unused terminal metadata reservations close through existing private cleanup.
Status distinguishes exact roots/bindings, unique/copy bytes and historical versus
new removal. This remains a bounded library slice: native age/category selection,
operator/UI/Action surfaces, larger paged scopes and all authority/provider/
large-file/lifetime/human acceptance gates remain required for full priority 7.
