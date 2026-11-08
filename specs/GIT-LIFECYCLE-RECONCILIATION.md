# Explicit Git lifecycle reconciliation

Input d81cabe662e0efb976f9ac1c230a1de2ace8e23f.
WP-003/004/005/006/008/009/011/012/015;
REQ-001/002/003/004/005/006/007/008/009;
SPEC-002/003/004/005/006/007/008/009/013;
IF-002/003/004/005/006/008/009/012/013; PKG-002/003/004/008/009.
GIT/PROVIDER profiles. Allowed scope: exact enrolled Git ref capture, bounded
selection plans, existing guarded source journals/metadata, and composition of
existing copy/retention/ordinary-retirement primitives in synthetic stores.
Entry: complete explicit pointer scopes, enrolled source/owner/class, immutable
namespace/store policy and existing retained-survivor/removal contracts.
Exit: exact-ref catch-up, duplicate/stale/revert handling, source-preserving copy,
retained history and job-aware ordinary eviction with restart-safe results.
Implementation, verification, validation, trace and review close separately.

This contract is under implementation. No GitHub receiver, runner installation,
live cache, final deletion or production adoption is enabled by this document.
Golden/released terms and working/history/backup places remain configuration.

## First implementation checkpoint

The first checkpoint implements exact committed-ref selection and atomic desired
protection through the existing namespace, not the full provider orchestration
exit above. `tig-lifecycle describe` advertises `selection_only:true` and
`storage_converged:null` (unknown). The same distinction applies to library results.
Copy/retention/eviction integration, a trusted GitHub trigger boundary, branch
withdrawal and draft/index lifecycle integration remain required subsequent work.
This checkpoint does not close priority 6 or replace those deliverables.

```text
tig-lifecycle plan PROFILE REQUEST
tig-lifecycle status PROFILE PLAN
tig-lifecycle apply PROFILE PLAN
tig-lifecycle resume PROFILE PLAN
```

All arguments are absolute bounded regular JSON files. `plan` emits a canonical
plan directly; save exact bytes as PLAN without a trailing newline. Failures
return a static versioned code and nonzero status without Git/provider stderr.
No command initializes/enrolls a Git repository, namespace or metadata provider.
`status` needs only the admitted namespace and does not open Git/metadata stores
or append commands. Historical receipts remain distinct from current selection.
Fresh apply also reports `observed_ref_current`; null means not sampled/unknown,
not agreement. Drift before selection holds; drift after selection requests later
catch-up without claiming that the admitted root is the latest Git ref. Duplicate
selection of an unchanged exact checkpoint is a no-op without new metadata/events.

The strict `tig.git-lifecycle-profile.design-1` has `enrollment`, absolute
`git_executable`/`repository_root`, existing `namespace_root`/`namespace_minimum`,
existing `metadata_root`/`metadata_minimum`, `source_limits`, and default-false
`allow_selection`. Enrollment pins repository/authority/namespace/policy digests,
source/owner/class, full ref, sorted explicit logical scopes and finite budget
(entries, nodes, node_bytes, millis). Policy digest covers the actual immutable
namespace policy. Source limits reuse the existing Git listing/entry/pointer/
command bounds. All existing provider and journal lifetime bounds remain in force;
elapsed limits are cooperative and do not preempt filesystem/provider code.
This first profile caps each selection at 32,768 bindings, 500,000 metadata nodes
and 32 MiB of node work, with at most 600,000 ms cooperative elapsed budget.
These caps are not native/large-source scale qualification. Source Git-command
limits remain independently bounded by the existing extractor.

REQUEST is `tig.git-lifecycle-request.design-1` with independently matched
`repository`, `ref_name` and optional `commit`; null/missing commit requests the
currently observed enrolled ref. A non-null commit must match that ref. Complete
explicit pointer extraction is required. Native/opaque roots need their own
admitted flat-byte closure; selected manifests do not imply payload coverage.
Public library enrollment is trusted caller configuration, not authentication.

Compatibility qualification builds the pre-pair `tig-source` at pinned input
d81cabe662e0efb976f9ac1c230a1de2ace8e23f into a separate external target directory.
Set `TIG_BASELINE_SOURCE_CLI` to that artifact, then explicitly run
`cargo test -p tig-store --lib --locked source_claims::admission::tests::baseline_cli_replays_atomic_pairs_without_format_migration -- --ignored --exact`.
The ordinary ignored entry is not executed evidence. Both hosted targets require
this qualification; old source inspects new pairs and recovers both original
command receipts. No other checkout/cache/runtime is changed by the fixture.

## Selection and trusted execution

An event requests an exact enrolled repository/ref/commit. It grants no authority.
Private operator enrollment independently pins repository identity, ref, pointer
scope, source owner/class, namespace/policy, metadata provider and finite budgets.
Writes default disabled. A trusted executor or operator invokes the preinstalled
binary using profiles outside the authored repository; untrusted PR/workflow code
gets no profile, credentials or provider rights. Authenticating an external event
belongs to that trusted boundary, never to an asserted JSON field. Initial CLI
input is explicit trusted operator invocation, not an authenticated webhook API.

The exact locally enrolled ref is resolved before capture and checked again
before selection admission. Local observations do not claim global GitHub-head
knowledge. Duplicate/missed events can request catch-up from the enrolled ref.
A stale event cannot replace a newer admitted source revision. Git revert creates
a new source revision selecting the exact earlier bytes; old retirement retries
remain historical and unavailable bytes require explicit reacquisition.

Plans bind exact source revision/state, selected commit/listing/binding root,
repository/ref, policy/namespace/enrollment and budgets. Incomplete extraction
holds claim shrink. Immutable metadata must finish and read back before selection
admission. A journal acknowledgement establishes desired protection, not byte
availability, retained custody or storage convergence.

## Atomic use of existing journal commands

Git-selected desired roots need no externally visible half-adopted selection.
The guarded namespace may append one exact Prepare/Adopt pair in a single existing
SQLite transaction. Before/after demand compilation and scoped fences cover the
whole pair; no SQL transaction spans Git/tree/provider IO. Readers see either the
old source or the new current plus retained predecessors. Concurrent sources and
unrelated providers remain independent. Existing source roles and history limits
stay unchanged; no new persistent command/schema/hash domain is introduced.

Both existing event bodies, sequence numbers, hash chaining and operation IDs
retain their current encoding. Both exact receipts replay historically; a partial
recorded pair or mismatched operation binding refuses rather than completing an
unrelated half transition. Failed second-command validation, limits or pre-commit
failure rolls back both events. After-commit failure is recovered from the original
receipts. Existing one-command callers preserve their behavior. Old readers can
replay the resulting existing commands without a format migration.

This composes admitted desired-state changes only. It is not a transaction over
Git refs and storage providers. A ref can advance while work runs; observations
and later catch-up expose that separately. All byte removals still require current
all-namespace protection, scoped provider fences and independently retained custody.
Unenrolled writers, incomplete native closure and unresolved authority handoff
remain holds, not inferred absence.

## Placement and outcomes

Resolve effective policy before union: current/active demand can suppress advisory
history proximity, while hard retention remains additive. Unchanged/shared bytes
do not automatically require a history copy. An old job can retain working bytes.
Copy only missing required placements, verify/retain successors, then use the
existing managed retirement API for eligible working copies. Transfer/upload
success alone never qualifies a retained keeper. Offline destinations leave pending
work. No anticipated reclaim funds admission. Expose desired checkpoint, plan,
historical outcome, current observations, holds and measured effects separately.

A no-op Git selection does not imply a no-op storage reconciliation: job/draft
withdrawal, provider availability or policy obligations can change independently.
Continue from fresh admitted demands and provider observations even when no new
selection commands are needed. Do not mint another physical copy or retention
claim for every merge; reuse exact existing publication/grant provenance through
fresh verification. Preserve original retry bindings rather than relabelling old
copy plans with a new source basis.

The existing copy helper deliberately retains its destination. When used for a
working cache, hand that temporary ingest retention to an independently retained
backup with the existing owner-authorized handoff before treating the working
copy as eligible. Namespace active-use protection remains in force. History-cache
retention needed as a keeper must remain through conflicting removal; later release
also uses authorized handoff. Never erase retention files or reset their accounting
to force an eviction. Final backup-claim release remains a separate final-deletion
contract, not an ordinary handoff to nowhere.

Keep plans and expanding runtime records outside Git. Preserve existing bounded
journal/history limits and hold safe exhaustion. Continuation, retained-subset
handoff for bulk retirement, authority migration, large-file qualification and
metadata expiry remain separate required work; this slice does not reset prefixes.

L0: fmt/validator/target hashes/diff and all six producer lenses. L1: focused
selection/journal failures, default/all-feature locked suites, strict Clippy and
standalone Git. L2: actual temporary Git merge/revert/duplicate/missed/stale events,
old/new readers, journal pre/post-commit faults and child exit; three-store copy/
retention/ordinary unlink/restore with active job, backup outage and unrelated
progress. These remain obligations until exact-source results are recorded.
