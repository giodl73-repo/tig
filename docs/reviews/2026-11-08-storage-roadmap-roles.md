# Six-role storage roadmap and development handoff review

Input 44b7dabf3b7bd913a5d9e448e20623001697a0f6. DOCS profile;
WP-003/004/005/006/007/008/009/011/012/013/015/016;
REQ-001/002/003/004/005/006/007/008/009/010; SPEC-002..014.
Targets: PRODUCT_PLAN.md next sequence and fixture design; AGENTS.md, CONTEXT.md,
docs/DEVELOPMENT.md and context/RESUME.md for self-contained future startup.
No runtime/provider mutation or new implementation claim.

Method: producer-applied checklists from .roles/ROLE.md and all six parliament
roles, against specs/MODEL.md, specs/SAFETY.md and foundation scenarios. This is
one producer's review, not six independent agents or approval for live operations.
Entry: the copy/verify/retain/evict roadmap and existing component evidence.
Exit/L0: explicit dispositions, local links, source budget, foundation hashes and
diff checks. L1/L2: not applicable to prose; all new lifecycle tests remain planned.

| Finding / role | Severity | Required correction / design disposition | Implementation proof still required |
| --- | --- | --- | --- |
| SR-01 Identity and Namespace | High | Bind plan and retry IDs to exact object hash/length, store instance/generation, source/policy/claim basis and provider contract; conflicting reuse refuses. Added to priority 2 and contract. | Changed store root/generation, operation-ID collision and source drift refuse without destructive effects. |
| SR-02 Placement Policy | High | Eviction cannot precede complete scoped branch/draft/job/retention admission merely because automatic Git reconciliation is later. Put minimum admission in priority 2, require it in 4; incomplete scope permits copy-only progress. Add explicit precedence, infeasibility and peak reservation. | Shared branch/active job, conflicting demands, unknown coverage and insufficient peak capacity preserve protection; no anticipated reclaim funds admission. |
| SR-03 Storage and Custody | High | Verification is not retained independent custody. Require declared replica count/domain provenance and alias refusal; first executor excludes final-archive deletion. Separate logical/file/allocation and capacity observations. | Destination loss, aliases, shared packs and restore after eviction demonstrate surviving bytes and honest reclaim. |
| SR-04 Concurrency and Recovery | High | Fresh checks must remain protected through removal; a check/delete race is insufficient. Define scoped fencing, durable idempotent operations, cancellation and uncertain restart states, with unrelated writers progressing. | Claim arrival at final boundary, interrupted copy/removal, lost acknowledgement, cancellation and fresh-process recovery; no blind destructive retry. |
| SR-05 Provider and Privacy | High | Boolean capabilities alone hide acknowledgement/retention strength. Admit provider-specific guarantees; hold unsupported removal. Bound apply/default dry run, credential handling, path safety and external-writer assumptions. | Credential denial, replaced roots, path/symlink escape, unsupported fence and untrusted remote acknowledgement refuse safely; separately qualify real providers. |
| SR-06 Git and Adoption | Medium | Make startup independent of old chat/consumer checkout. Link a bounded handoff and startup path; derive live branch/PR state, use external build/temp roots, preserve other lanes and running tools. Keep journals/large plans machine-side. | Documentation links/checks now; next fresh session verifies source/PR state. No deployment, cache init or automatic hook authority follows from reading files. |

All six corrections are incorporated in the revised plan/startup documentation.
Owner: Tig maintainer, using the named role as the review lens. Disposition:
pass_with_risk for the revised design only. No unresolved design blocker within
this scope; runtime milestones remain open until their specified tests execute.
This does not close full work packages, independent review or human acceptance.

The prior roadmap checkpoint reran 20 replica entries, 16 handoff entries and six
real LFS client cases successfully. These scoped component results are unchanged;
they do not execute the proposed copy-to-evict lifecycle. No Cargo rerun is
required for this documentation-only revision. Protected CI and portable L0
checks remain distinct from those historical component results.

Startup walkthrough: resolve the actual checkout; inspect branch/dirty state/
worktrees/PRs; read resume state and ordered roadmap; select an unowned bounded
slice; configure external build/temp roots; run its checks; update handoff when
priorities change. Machine-local checkout paths are deliberately not in public
source. Canonical source can therefore be relocated without rewriting this guide.

## Lifecycle feedback follow-up

Input 43adabd5077123165a00895ef1c4ae99c9fd4a38. Scope: PRODUCT_PLAN.md,
context/RESUME.md and docs/CATALOGUE-UI-PLAN.md; planning only. Controls:
WP-003/004/006/007/008/009/013/015; REQ-001/002/003/004/005/007/009;
SPEC-002/003/004/005/006/008/009/013/014; IF-002/003/004/005/006/008/012/013;
PKG-001/002/003/004/007/008/009. No persistent format or runtime contract changes.
Entry: user's merge/placement, finite-backup and bulk-deletion requirements plus
architecture review feedback. Exit: named target capabilities, separate authority
boundaries, scoped fixtures and resumable planning context. DOCS L0: structure,
links, source budget, target hashes and diff checks. L1/L2 do not apply to prose;
implementation, verification, intended-use validation and trace closure remain
separate future obligations. No runtime evidence count is added.

Method: one producer applying all six checklists, not independent reviewers.
Owner for every row: Tig maintainer using the named lens. Findings below are
incorporated in the plan; the executable proof remains required.

| Finding / role | Severity | Design disposition | Required implementation proof |
| --- | --- | --- | --- |
| LF-01 Identity and Namespace | High | Distinguish authorized absence from corruption and old publication replay; separately gate metadata forgetting with authority/epoch fencing. | Old retry, restored backup and historical pointer cannot revive retired operations or falsely promise availability. |
| LF-02 Placement Policy | High | Merge-triggered released/history/backup roles are configured policy; bulk age/category selection binds exact versions and checks all surviving demands. | Shared/current/branch/job-protected bytes survive; unknown age and stale selection hold; reactivation invalidates eligibility. |
| LF-03 Storage and Custody | High | Separate replica eviction from authorized final-copy loss; prevent two stores retiring against each other; show honest reclaim. | Simultaneous retirement preserves required custody; eligible final deletion truthfully reports intentional loss; mixed batches preserve held items. |
| LF-04 Concurrency and Recovery | High | Name coverage authority/invalidation and resumable partial outcomes; require journal continuation before sustained use. | Authority outage, lost replies, cancellation, continuation faults and old-peer retries retain fencing and historical receipts. |
| LF-05 Provider and Privacy | High | Qualify large objects beyond the 8 MiB reference proof; respect retention locks/offline scope; keep all places/categories generic. | Bounded memory/peak IO, disk full, cancellation, locked/offline stores and measured physical effects; no private bindings in source. |
| LF-06 Git and Adoption | Medium | Enabled Action targets an exact merged commit via an enrolled executor; routine eviction needs no per-copy confirmation under admitted policy. | Duplicate/delayed/missed events converge, newer selections prevail, offline backup holds safe progress, no implicit runner deployment. |

Disposition: pass_with_risk for planning only. Final deletion and metadata
retirement are explicit later milestones; the first executor stays copy-only,
then guarded replica eviction. No implementation, full requirement verification,
consumer validation, independent review or live-operation acceptance is closed.

Follow-up L0 result: tools/validate.ps1 passes structure, local links, budgets,
reviewed target hashes and synthetic arithmetic; git diff --check passes.
No Rust/controller suite rerun for these documentation-only edits. Hosted CI,
executable lifecycle fixtures and independent acceptance remain unclaimed.

## Second six-role pass after lifecycle feedback

Input 79791904f5490ede00f5ac9811538cc0d5992fae. Targets: PRODUCT_PLAN.md,
context/RESUME.md and their consistency with specs/MODEL.md, specs/SAFETY.md,
specs/SOURCE-CLAIM-JOURNAL.md, docs/SAFETY-PROMISE.md and foundation scenarios.
Same DOCS controls as the lifecycle follow-up, with REQ-006 for provider/privacy
and adoption. Scope excludes runtime code, formats, live stores and deployment.
Entry: revised merge/eviction/final-deletion/metadata roadmap. Exit: challenge
counterexamples, correct planning gaps, preserve explicit implementation gates.
Method: one producer reapplying all six .roles checklists; no delegated or
independent reviewers. Owner of each finding: Tig maintainer, with its named lens.

| Finding / role | Severity | Counterexample and correction | Remaining proof |
| --- | --- | --- | --- |
| RR-01 Identity and Namespace | High | A hash-wide tombstone could reject legitimate new identical content, or an old deletion could remove it. Bind retirement to operations/authority generations and fence new placement against stale removal. | Same-hash new ingest survives old deletion retry; revert reports absent exact bytes honestly. |
| RR-02 Placement Policy | High | Selected history may protect itself forever, or a batch might withdraw another owner's claim. Add explicit recoverable owner-authorized settlement of selected historical obligations; other/non-releasable protection remains. | Own eligible claims can retire; foreign/shared/held claims refuse; crashes between claim and payload steps preserve truthful state. |
| RR-03 Storage and Custody | Medium | Namespace unlink can leave provider versions/trash or free zero bytes. Separate managed removal, retained representations and measured reclaim; do not imply secure erasure. | Provider versions/locks and shared representations remain visible; no universal or secure-erasure claim. |
| RR-04 Concurrency and Recovery | High | A full journal after removal could prevent durable evidence. Require admitted metadata budget and durable authorized intent before destructive IO, with completion/recovery capacity. | Exhaustion holds before removal; child exit after unlink recovers the exact intended action; new claims stay fenced. |
| RR-05 Provider and Privacy | High | Merged workflow/policy content could attempt to grant itself storage authority. Authenticate the trigger and independently admit policy/credentials; untrusted PR execution receives neither. | Wrong repo/ref/event, unadmitted policy upgrade and untrusted workflow cannot authorize provider IO. |
| RR-06 Git and Adoption | Medium | Listing lifetime at priority 8 could defer safe exhaustion, and forgetting tombstones could conflict with required audit retention. Bring budget design into priority 2; require audit-window and persistent aggregate-fence compatibility before metadata retirement. | Sustained operation/continuation retains witnesses and old receipts; expired asset detail cannot revive old peers; audit requirements remain satisfied. |

All six design corrections are incorporated in the roadmap and resume handoff.
Disposition: pass_with_risk for revised planning only. Before final-deletion code,
freeze owner claim-settlement and intentional-absence semantics; before metadata
retirement, freeze audit retention and rollback-resistant authority fencing under
controlled specs. These implementation entry gates remain open. No new runtime
tests or evidence counts, full requirement closure, independent review, human
acceptance or live-operation authority are claimed. L0 structure/link/budget/hash
and diff checks apply; L1/L2 are not applicable to this prose-only revision.

Second-pass L0 result: tools/validate.ps1 and git diff --check pass on the
corrected prose, including reviewed hashes, local links and source budgets.
Rust/controller suites were not rerun; hosted CI remains a separate check.

## Current architecture re-review

Input 355aad7fc6ab964527b46492cdb6f98a395f58c1. Targets: PRODUCT_PLAN.md,
specs/MODEL.md, specs/SAFETY.md, specs/SOURCE-CLAIM-JOURNAL.md,
specs/PROTECTION-DEMAND-SNAPSHOT.md and docs/SAFETY-PROMISE.md. Method:
one producer reapplying all six parliament checklists in .roles/ROLE.md;
no delegation, independent approval or live-operation authority.
Controls: WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014;
IF-002/003/004/005/006/008/012/013; PKG-001/002/003/004/007/008/009.
DOCS profile: allowed scope is roadmap clarification and this review record;
no code, persistent format, provider operation or implementation closeout.
Entry: existing lifecycle roadmap and scoped copy/protection evidence.
Exit: explicit counterexamples, dispositions and required future tests.
L0: portable validation, reviewed target hashes and diff check. L1/L2 are not
required for prose; the runtime cases below remain unexecuted design obligations.
Owner of every finding: Tig maintainer using the named checklist.

| Finding / role | Severity | Counterexample and disposition | Required proof |
| --- | --- | --- | --- |
| AR-01 Identity and Namespace | High | A store copy can be protected by more than one namespace. A single journal's absence fact cannot cover the others. Added store-scope/generation binding to every applicable protection authority. | Another namespace's claim blocks removal; unknown enrollment holds. Exact bytes and incarnation remain bound to the authorized action. |
| AR-02 Placement Policy | Medium | Unioning current and predecessor placements can send every unchanged byte to history, defeating the user's inactive-only history policy. Added configurable active-use precedence over advisory history proximity; hard retention/location obligations remain additive. | Shared unchanged bytes avoid unnecessary history requirements; a job retains old working bytes; explicit hard history retention survives precedence. |
| AR-03 Storage and Custody | High | Whole-root retirement for one selected leaf can drop protection for unselected/held siblings. Added acknowledged retained-subset replacement before broad-root retirement. | Mixed eligible/held/shared block and crashes before/after replacement adoption preserve every surviving obligation. Payload removal remains independently authorized. |
| AR-04 Concurrency and Recovery | High | A fresh authority epoch has different fences; an old executor may miss new demand even when both epochs are internally correct. Added explicit handoff, old-removal fencing and conservative refusal while unresolved. | Old/new authorities race claim admission and removal, restart mid-handoff and replay old peers without losing protection; unrelated work continues. |
| AR-05 Provider and Privacy | High, retained gate | Drive letters, upload acknowledgements and namespace unlink still establish neither independent custody nor physical reclaim. Existing provider/custody gates remain mandatory; no new correction needed. | Alias/offline/locked/versioned providers, actual restore, large-object resource limits and measured allocation results. |
| AR-06 Git and Adoption | Medium, retained gate | A successful merge cannot prove storage convergence or grant workflow-authored deletion authority. Existing trusted-event/policy/executor separation remains appropriate. | Duplicate/delayed/missed events, wrong ref/repository, offline backup and policy escalation preserve bounded authority and report pending work. |

Disposition: pass_with_risk for the corrected roadmap. AR-01/03/04 remain
implementation entry gates for the affected removal paths; this prose closes
neither those mechanisms nor full requirements. Scoped copy and read-only
protection projection do not prove eviction readiness. Final-copy deletion and
metadata forgetting retain their separate authority, audit and replay gates.
The in-progress admission implementation in another worktree was not modified
or qualified by this plan review. No runtime evidence count is added.
