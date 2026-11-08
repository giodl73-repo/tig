# Tig resume state

Active goal: deliver PRODUCT_PLAN.md, including merge-driven placement/eviction,
bulk final deletion and eventual metadata retirement. The full goal is not done.
Read AGENTS.md, CONTEXT.md, docs/DEVELOPMENT.md, the roadmap and active pulse.
Derive branch/worktree/PR status from Git/GitHub; do not trust old chat or deploy
source changes into running tools as a side effect. No startup cache operations.

## Current delivery state

- PR #14: reviewed storage roadmap, finite-backup/bulk-retirement requirements
  and direct startup. Source/merge status must be checked live.
- PR #15, feat/storage-orchestration-contract, d18afba: bounded immutable storage
  plans and tig-storage plan/peak/status/apply/resume through existing accounted
  publication/retention. Local checks and hosted Windows/Linux/secrets pass;
  marked ready for review. Optional publication context preserves legacy None
  encoding; old readers refuse new context-bearing records. No removal API.
- PR #16, feat/protection-demand-snapshot, implementation55a21e4: actual read-only journal/root
  demand projection, exact selected revisions/pending references, and copy-basis
  integration. Seventeen focused fixtures plus 35 source-journal cases qualify
  this prerequisite; local default/all-feature, Clippy, fmt, standalone Git and
  L0 checks pass, recorded under EVID-085. Hosted Windows/Linux/secrets pass;
  marked ready for review. Declared flat
  binding trees only; opaque/native context roots remain gaps. The View adapter
  always sets complete_claim_coverage=false. This does not complete admission.
- PR #17 (draft), feat/mandatory-source-admission, implementation0b75664: guarded design-2 journal
  gates every generic mutation, enrolls immutable source classes and compiles
  exact before/after raw demands into live shared partition permits. Exclusive
  subject assessment tolerates unrelated head changes. Active/history precedence
  and additive retention are explicit. Final default/all-feature workspace suites,
  strict Clippy, fmt, standalone Git and L0 pass; hosted qualification remains
  separate. Twenty
  admission entries include a helper and three actual child processes. EVID-086
  separates verification/review from implementation. No removal or store binding.
- Six-role roadmap re-review85ac433 records precedence, mixed-root retained-subset
  handoff and store/namespace/epoch authority handoff requirements. These remain
  implementation gates, not merely documentation acceptance.
- Catalogue record/owner/impact/compare slices remain separate. User intends to
  test UX; do not start browser automation on resume. Viewer deployment, source
  merge, producer review, independent review and human acceptance are distinct.

## Next executor entry

Check hosted qualification/source status without treating a single guarded
namespace as provider-wide coverage. Next bind each enrolled store scope/
generation to all authorities that can protect it, define acknowledged authority/
policy epoch handoff and refuse unresolved/legacy/unmanaged boundaries. A fresh
epoch does not transfer claims or revoke an old removal executor by itself.
Preserve per-subject admission and unrelated progress; no global cache lock.

Define authorized-absence/publication recovery and incarnation fencing:
old deletion retries must not remove a new same-hash placement. Only after these
gates implement guarded ordinary eviction. Full private/provider adoption stays
separate; remote/LFS upload, automatic merge reconciliation, final deletion and
metadata retirement remain required later milestones, not waived requirements.

## Product and safety constraints to retain

Released/golden and category vocabulary is project configuration. Enabled merge
actions should put released bytes at working/backup roles, superseded unused bytes
at history/backup roles, then automatically evict eligible working copies.
Retention on backup is finite. Users need exact block/category/age selection for
final deletion, held/shared-byte explanations, resumable outcomes and honest
reclaim. Age uses admitted lifecycle events, not file mtime. Final-copy deletion
has separate authority from cache eviction; low space grants neither.

A mixed source root may contain selected and held assets. A retained-subset
replacement/root handoff must be acknowledged before withdrawing the broader
obligation; never retire a whole root just to delete one selected leaf. Exact
selection revisions distinguish same-byte historical uses. Required audit windows
and rollback-resistant replay/authority fences precede metadata forgetting.
Journal continuation and large-object qualification remain explicit gates.

Use synthetic, explicit external build/temp roots and scoped check commands.
Keep private bindings, credentials, inventories, payloads and runtime evidence
outside Git. Preserve existing consumers and running tools. .roles are producer
checklists, not independent people or live-operation authority.
