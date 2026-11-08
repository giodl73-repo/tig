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
- PR #17, feat/mandatory-source-admission, implementation0b75664: guarded design-2 journal
  gates every generic mutation, enrolls immutable source classes and compiles
  exact before/after raw demands into live shared partition permits. Exclusive
  subject assessment tolerates unrelated head changes. Active/history precedence
  and additive retention are explicit. Final default/all-feature workspace suites,
  strict Clippy, fmt, standalone Git and L0 pass; hosted Windows/Linux/secrets
  pass and it is ready for review. Twenty
  admission entries include a helper and three actual child processes. EVID-086
  separates verification/review from implementation. No removal or store binding.
- PR #18, feat/store-protection-boundary, implementationbe6f946: fresh opt-in local
  design-6 stores pin immutable namespace/place roster, boundary identity and
  generation. Assessment requires every enrolled guarded namespace plus the
  provider's existing exclusive object guard and shared aggregate work budget.
  Fifteen focused entries include one real child process; actual copy/accounting/
  retention works with the new contract. Final default/all-feature workspace,
  strict Clippy, fmt, standalone Git and L0 pass. Hosted Windows/Linux/secrets
  pass and it is ready for review. EVID-087
  keeps verification/review separate. No physical removal or live enrollment.
- PR #19 (draft), feat/managed-replica-retirement, implementationfe88c7c: opt-in
  design-7 stores add actual ordinary replica unlink, owner cancellation and explicit
  new-operation reacquisition with incarnation-bound publication/replay. Existing
  accounted/retention/handoff and all-namespace/provider fences are reused. Twenty-
  four focused entries include eight actual child exits and a main/job/replacement/
  backup/unlink/restore lifecycle. Final default/all-feature workspace, Clippy, fmt,
  standalone Git and L0 pass locally; hosted qualification remains separate.
  EVID-088 separates mechanism verification/review from implementation. No live
  stores, final-copy deletion, expiry or automatic Git action is enabled.
- Six-role roadmap re-review85ac433 records precedence, mixed-root retained-subset
  handoff and store/namespace/epoch authority handoff requirements. These remain
  implementation gates, not merely documentation acceptance.
- feat/retirement-agent-cli, implementation19346c4, based on3e300ca: explicit tig-storage retire
  plan/status/apply/resume/cancel/reacquire uses existing managed retirement and
  private owner/context/domain/grant/role/budget admission. Eight CLI process
  cases and the separate real-child pending-intent CLI cancellation qualification
  pass locally. Final locked default/all-feature workspace suites, strict Clippy,
  fmt, standalone Git and portable L0 pass. EVID-089 separately records verification,
  synthetic validation, trace and all six producer-review lenses. Hosted current-
  head qualification remains separate; derive PR state live. No real cache,
  initialization, final deletion or automation.
- Catalogue record/owner/impact/compare slices remain separate. User intends to
  test UX; do not start browser automation on resume. Viewer deployment, source
  merge, producer review, independent review and human acceptance are distinct.

## Next executor entry

Check current-head local/hosted qualification and PR state. The store binding
covers its explicit immutable first roster; it does not implement epoch migration,
external-client discovery or full restore authority. Preserve that distinction.

Check current-head hosted qualification for the managed-retirement and CLI lanes;
inspect actual jobs before retrying. A polling timeout is not failure. Local CLI
qualification and separate evidence closeout are complete. The cross-artifact
pending-cancellation fixture requires a freshly built binary via TIG_RETIREMENT_CLI
and an explicit --ignored --exact invocation; its ordinary-suite ignored entry is
not executed evidence. Both hosted platforms include that qualification.

Continue the first remote/upload provider slice from the existing publication/
orchestration ports. Select one concrete backend, freeze its admitted capability,
acknowledgement, retry/readback and credential boundaries, then use isolated
protocol/provider fixtures. Preserve source and distinguish uploaded/verified
availability from independent retained custody. Do not install services, connect
real stores or copy private credentials as startup behavior. Keep native media,
automatic Git lifecycle and last-copy deletion out of the generic provider port.

Then continue roadmap priorities: one qualified remote/upload adapter, Git/LFS
upload and explicitly enabled exact-ref lifecycle convergence; finite-backup bulk
retirement/final deletion and eventual metadata forgetting remain required. Large-
file/native-provider qualification and journal/history continuation are prerequisites
for sustained/private use. The reference profile remains 8 MiB, sixteen object
action slots and cooperative process restart only. Unexplained bootstrap anchors,
partial metadata and full-store/authority restore need explicit recovery/adoption;
never silently repair/reset their prefixes.
Define/implement acknowledged authority/policy epoch handoff before enabling roster
changes: freeze old removal permissions, adopt surviving demand and refuse unresolved
legacy/unmanaged boundaries. A fresh epoch does not transfer claims or revoke an
old executor by itself. Preserve unrelated progress; no global cache lock.
Full private/provider adoption stays
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
