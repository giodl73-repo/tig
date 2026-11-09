# Owned branch and staged-draft settlement

Input9c2a29f90499b901c352b7ebc9e77997787d2265.
WP-003/004/005/006/007/008/009/015; REQ-001/002/003/004/005/007/008/009;
SPEC-002/004/005/006/007/008/009; IF-002/004/005/006/008/013;
PKG-002/003/004/008/009. GIT/PROVIDER profiles. Allowed: exact local ref absence,
owned branch/draft withdrawal, bounded conditional admission in the existing
journal, private operator CLI, existing placement and synthetic fixtures. No live
enrollment, hook/service, Git mutation, new journal encoding, final deletion or
implicit predecessor retirement. Entry: guarded source owner/class, exact source
capture, finished metadata and current provider fences. Exit: plan/status/apply/
resume/reconcile, stale-source refusal, partial observations and restart recovery.
Implementation and verification/validation/trace/six producer lenses close apart.

Branch settlement requires an independently enrolled Branch source and complete
successful exact local ref enumeration proving that its configured ref is absent.
Git failure, unsupported target, an offline mirror or missing repository is not
absence. This proves local observed absence only, not global remote deletion.
Private allow_withdraw defaults false. Withdraw moves current into predecessor
under the existing policy, preserving historical obligations and all other owners.
It never deletes payloads or silently retires history.

Draft settlement requires a registered Draft index source with a currently adopted
conflict-free index capture and a separately enrolled Branch successor in the
same namespace/policy/authority. The successor must already match its exact current
Git ref and durably adopted binding selection. Its verified flat metadata closure
must cover every exact draft object (hash and length). This is a byte-protection
handoff, not proof that Git paths or native asset semantics are equivalent.
Unmerged, unsupported, missing metadata or unadopted source holds settlement.

Plans bind private mode/enrollment, owner/source, expected revision/state,
exact index/ref evidence and successor source revision. Before withdrawal,
revalidate evidence, finish no new source implicitly, and recheck Git after work.
The existing journal's short transaction checks bounded source-revision conditions
after catch-up and before appending the unchanged Withdraw event. This prevents
a successor changing between metadata inspection and commit without a global
cache lock or SQL held through Git/tree IO. Unrelated source changes can proceed.
Conditions are ephemeral admission predicates, not new durable proof fields;
controller operation IDs bind the whole immutable plan. Historical retries return
the original command receipt, never a renewed condition grant. The generic API
accepts at most eight unique sorted source/revision conditions.

Status needs only the admitted namespace and immutable plan. Reports distinguish
historical withdrawal, current settled state, current Git evidence and physical
page convergence. A source can reactivate later; an old retry cannot withdraw
its new revision. Cancellation does not unrecord a completed owner withdrawal.
Source drift/outage after acknowledgement remains visible rather than undoing
history or claiming current convergence.

Explicit physical reconciliation uses existing private placement permissions and
one bounded page after settlement, including no-op settlement. Validate private
storage/namespace bindings before source mutation. All payload evictions still
require current all-namespace demands and independently retained custody; other
jobs/branches remain active protection. Retained backup and predecessor roots
are never deleted by this contract.

Git rename preserves a logical source only through explicitly adopted enrollment;
revoking old controllers/dynamic alias or epoch migration is separate required
work. This first contract does not infer rename from absence, reset identity,
drop staged/private data, install automatic hooks or make a Git/SQLite transaction.
Future private writes still require ingest/job protection. Finite journal/history
bounds and eventual owner subset/final-retirement gates remain in force.

L0: fmt/validator/pins/diff. L1: locked default/all-feature workspace, strict Clippy,
standalone Git, exact condition/replay/negative cases. L2: actual temporary Git
ref/index/namespace and provider CLI flows, changed successor during tree IO,
unrelated progress, pre/post-commit abrupt exits, stale source/reactivation,
missing/corrupt metadata and held jobs. Full native/provider/large-file/lifetime,
authority migration, final deletion, metadata expiry and user adoption stay open.

## Operator interface

`tig-lifecycle settle` provides plan PROFILE, status/apply/resume PROFILE PLAN
and reconcile PROFILE PLACEMENT_PROFILE [CURSOR]. Describe advertises these
separately. Strict `tig.source-settlement-profile.design-1` contains enrollment,
git_executable, repository_root, source_limits, namespace_root, namespace_minimum,
metadata_root, metadata_minimum and default-false allow_withdraw. Enrollment kind
branch contains branch (committed-ref enrollment); kind draft contains draft
(index enrollment) and successor (committed-ref enrollment). Both draft/successor
belong to one repository/authority/namespace/policy. Plans are bounded canonical
JSON without the output newline. Missing roots and sources are never initialized.
Post-work settlement_current, observed_evidence_current and conditions_current
are separate; all must be true before reporting page/storage convergence.
Committed-ref/index reports also expose selection_current separately from Git
observations. GitHub coordination rechecks the admitted selection before pages
and final convergence; an unchanged ref cannot resurrect a settled source.
