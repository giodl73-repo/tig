# Tig resume state

Active goal: deliver PRODUCT_PLAN.md, including merge-driven placement/eviction,
bulk final deletion and eventual metadata retirement. The full goal is not done.
Read AGENTS.md, CONTEXT.md, docs/DEVELOPMENT.md, the roadmap and active pulse.
Derive branch/worktree/PR status from Git/GitHub; do not trust old chat or deploy
source changes into running tools as a side effect. No startup cache operations.

## Current delivery state

- Active lane: feat/git-placement, inputa678381, implementation7432739, local placement
  reconciliation through existing store mechanisms and `tig-lifecycle reconcile`.
  specs/PLACEMENT-RECONCILIATION.md freezes controls/scope/entry/exit. EVID-093
  separately records local verification, synthetic validation, trace and all six
  producer lenses. Final focused coverage is sixteen placement entries
  (one child helper, one real process exit), two namespace-projection cases and twelve
  Git lifecycle entries, including renewed-demand cancellation, unexplained
  managed-copy refusal, byte budgets, real merge, cursor handling and completion
  of a superseded original publication. The first broad all-feature run caught
  two new-fixture/reporting failures; those were corrected, not waived. Final
  all-feature workspace/default changed-package/strict Clippy/standalone Git and
  explicit retirement CLI qualification pass. The final provider-order correction
  followed broad workspace builds; final default/all-feature store library runs
  each pass 262 active entries (two ignored), default lifecycle passes twelve,
  strict Clippy passes and both ignored cross-artifact qualifiers explicitly pass.
  The pre-pair reader was rebuilt from clean pinned inputd81cabe. Local broad
  default workspace passed before the final focused corrections. Implementation
  and evidence/review closeout are separate commits. Next: publish the stacked
  draft PR and inspect hosted current-head qualification; derive its state live.
  Private partial cleanup remains explicit; CLI requires roots open at entry;
  page success alone does not establish full selection convergence.

  Trusted-trigger research for the next contract (not implemented): GitHub's
  [OIDC discovery](https://token.actions.githubusercontent.com/.well-known/openid-configuration)
  publishes issuer/JWKS, RS256 and claims including sha, repository_id,
  repository_owner_id, workflow_ref/sha and run identifiers. Its
  [reference](https://docs.github.com/en/actions/reference/security/oidc) explains
  audience/subject and reusable-workflow claims. Pin immutable repository/owner
  IDs and independently admitted workflow/policy; require exact commit/ref and
  expiration checks. This is a proposed verifier boundary, not request-JSON trust.
  [GitHub's runner guidance](https://docs.github.com/en/actions/reference/security/secure-use)
  warns about persistent compromise from workflow code on self-hosted runners.
  Preserve the plan's separate trusted executor/profile boundary; putting workflow
  code in the storage executor's OS authority would not satisfy that boundary.

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
- PR #19, feat/managed-replica-retirement, implementationfe88c7c: opt-in
  design-7 stores add actual ordinary replica unlink, owner cancellation and explicit
  new-operation reacquisition with incarnation-bound publication/replay. Existing
  accounted/retention/handoff and all-namespace/provider fences are reused. Twenty-
  four focused entries include eight actual child exits and a main/job/replacement/
  backup/unlink/restore lifecycle. Final default/all-feature workspace, Clippy, fmt,
  standalone Git and L0 pass locally. Head3e300ca passes hosted Windows (40m56s),
  Linux (3m29s) and secrets (6s), and is ready for review.
  EVID-088 separates mechanism verification/review from implementation. No live
  stores, final-copy deletion, expiry or automatic Git action is enabled.
- Six-role roadmap re-review85ac433 records precedence, mixed-root retained-subset
  handoff and store/namespace/epoch authority handoff requirements. These remain
  implementation gates, not merely documentation acceptance.
- PR #20, feat/retirement-agent-cli, implementation19346c4, based on3e300ca: explicit tig-storage retire
  plan/status/apply/resume/cancel/reacquire uses existing managed retirement and
  private owner/context/domain/grant/role/budget admission. Eight CLI process
  cases and the separate real-child pending-intent CLI cancellation qualification
  pass locally. Final locked default/all-feature workspace suites, strict Clippy,
  fmt, standalone Git and portable L0 pass. EVID-089 separately records verification,
  synthetic validation, trace and all six producer-review lenses. Head3fd9918
  passes hosted Windows (12m39s), Linux (3m55s) and secrets (6s), and is ready for
  review. No real cache,
  initialization, final deletion or automation.
- feat/s3-publication, implementation4fc3060, based on3fd9918: optional source-preserving S3 conditional
  PUT/full GET readback and explicit CLI reuse accounted local intent/completion.
  Fifteen focused HTTP/process cases, final locked default/all-feature workspace
  suites, strict Clippy, standalone Git and retirement CLI qualification pass.
  Actual peers contend only on the exact operation; another job uploads while
  its guard is held. EVID-090 separately records verification, validation rehearsal,
  trace and six-role producer review. Head9b8a689, PR #21, passes hosted Windows
  (12m29s), Linux (2m49s) and secrets (6s), and is ready for review; derive current
  PR state live. Private access is absent from plans; receipts deny
  retained custody/removal. No real account, service or cloud store is attached.
- feat/lfs-upload, implementation2f1173b, based on9b8a689: optional standalone
  Basic upload and tig-lfs upload plan/peak/status/apply/resume reuse existing OID
  authorization, guarded Git LFS readers and shared accounted local witnesses.
  Thirteen HTTP/actual Git LFS 3.4.0/CLI cases pass, including killed-process
  recovery, peer refusal and another job's progress without re-upload. Full
  readback plus optional verify notification precede completion; readable bytes
  cannot skip unfinished verify. Private action credentials are explicitly scoped
  and absent from plans. Final missing-root assertions reran after the broad-suite
  build; the product implementation was unchanged. Locked default/all-feature
  workspace suites, all 15 S3 regressions, strict Clippy, fmt, standalone Git/
  dependency graph, explicit retirement CLI qualification and portable L0 pass.
  EVID-091 separately records verification, synthetic validation, trace and six
  producer lenses. PR #22 head d81cabe passes hosted Windows (8m18s), Linux
  (3m2s) and secrets (6s), and is ready for review; derive PR state live.
  No actual server/TLS/account, remote quota, custody or live store is attached.
- feat/git-lifecycle, implementationfc3c57f, based on d81cabe: exact enrolled
  committed-ref selection and tig-lifecycle plan/status/apply/resume compose
  finished/readback metadata with an atomic pair of existing Prepare/Adopt events.
  Source encodings and one-command callers remain compatible. Six added pair
  cases bring active admission entries to 26, including three actual child exits
  and the 16,383-event last-slot edge. Nine real Git/library/CLI cases cover
  merge/revert, stale/catch-up/no-op, active jobs, exact/Unicode refs, drift and
  offline historical status. Existing journal regressions pass all 35 entries.
  Both broad workspace suites pass; the final status/discovery correction followed
  their builds, then all nine lifecycle cases and strict all-target/all-feature
  Clippy reran. Other package implementations were unchanged. Explicit retirement
  CLI and pinned pre-pair source-reader qualifications pass. The latter requires
  TIG_BASELINE_SOURCE_CLI and an explicit ignored/exact invocation; normal-suite
  ignored output is not proof. CI separately builds pinned inputd81cabe's reader
  on both hosts. EVID-092 separates verification, validation rehearsal, trace and
  six producer lenses. Hosted current-head qualification remains separate.
  This is selection only: storage convergence is unknown, and no provider
  copy/eviction or authenticated GitHub receiver is enabled.
- Catalogue record/owner/impact/compare slices remain separate. User intends to
  test UX; do not start browser automation on resume. Viewer deployment, source
  merge, producer review, independent review and human acceptance are distinct.

## Next executor entry

Check current-head local/hosted qualification and PR state. The store binding
covers its explicit immutable first roster; it does not implement epoch migration,
external-client discovery or full restore authority. Preserve that distinction.

Check current-head qualification for the S3 lane before duplicating or restarting
work. A polling timeout is not failure. Managed-retirement and CLI qualification
and separate evidence closeout pass locally and on both hosted platforms. The cross-artifact
pending-cancellation fixture requires a freshly built binary via TIG_RETIREMENT_CLI
and an explicit --ignored --exact invocation; its ordinary-suite ignored entry is
not executed evidence. Both hosted platforms include that qualification.

Check current Git lifecycle selection branch/PR qualification before duplicating
that lane. S3 and LFS upload hosted checks pass and both PRs are ready for review.
Next compose actual copy/retention/ordinary eviction from fresh admitted namespace
demands and provider observations. Selection-only/no-op Git results cannot skip
storage work caused by job withdrawal or destination recovery. Reuse exact existing
publication/grant provenance rather than creating another claim/copy per merge.
The copy helper retains destinations: working ingest retention needs the existing
owner-authorized handoff to independent backup before working eviction. Retention
required for a keeper remains through conflicting removal. No metadata-file deletion
or accounting reset may bypass that sequence. Extend the generic three-store
merge/job/offline-backup/unlink/restore fixture through the actual new selector.

Then integrate an independently authenticated trusted trigger and configured
branch withdrawal/draft/index lifecycle. No second cache engine or automatic runner
installation. Duplicate/delayed/missed events must converge to the currently
admitted ref; preserve drafts/jobs and report pending work separately from merge
success. A Git revert selects exact earlier bytes under a new source revision,
not permission to resurrect retired operations. Read GIT-WORKFLOW.md,
GIT-LIFECYCLE-RECONCILIATION.md, mandatory admission and retirement contracts.
Preserve source and distinguish uploaded/verified availability from independently
retained custody. Loopback protocol fixtures qualify neither actual AWS/LFS TLS/
authentication/account nor remote quota/retention capabilities.
Do not install services, connect real stores or copy private credentials as startup
behavior. Native media, automatic Git lifecycle and last-copy deletion retain their
separate boundaries and remain required later roadmap work.

Then continue roadmap priorities: real-provider qualification and explicitly
enabled exact-ref lifecycle convergence; finite-backup bulk
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
separate; full provider adoption, automatic merge reconciliation, final deletion and
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
