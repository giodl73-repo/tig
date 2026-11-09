# Tig resume state

Active goal: deliver PRODUCT_PLAN.md, including merge-driven placement/eviction,
bulk final deletion and eventual metadata retirement. The full goal is not done.
Read AGENTS.md, CONTEXT.md, docs/DEVELOPMENT.md, the roadmap and active pulse.
Derive branch/worktree/PR status from Git/GitHub; do not trust old chat or deploy
source changes into running tools as a side effect. No startup cache operations.

## Current delivery state

- Active lane: feat/local-clock, input4aeebf8. Read specs/LOCAL-CLOCK.md.
  Proposed first local-clock profile after optional preference received no answer;
  no live trust/adoption is inferred. Fresh clock-enabled capacity ledger design-2
  reuses existing hashed commands/replay and retains a high-water timestamp.
  Legacy design-1 bytes stay unchanged. Clock identity binds actual genesis plus
  immutable OS-time policy. LocalClock samples only on explicit calls, commits
  its lower bound before use and checks monotonic/wall correlation, sample age and
  invocation duration. TimeBounds protects the entire approval window in bulk,
  grant release, unlink and reacquisition birth. No CLI activation or fallback.
  Seventeen clock-filter entries, sixteen provider entries, eighteen active bulk
  entries, strict Clippy and expanded pinned older-writer refusal pass so far.
  Three actual clock-journal child exits and full16,384-command exhaustion run.
  Final cross-sample monotonic ordering and out-of-format timestamp preflight now
  pass their regression; final strict Clippy passes. Both broad workspace suites
  and fresh CLI artifact qualifications are running; derive their live handles
  and terminal results before closeout. Five
  independent framing vectors pin legacy/new marker, policy, enrollment and event.
  Next: final focused/broad qualification and separate evidence/validation/trace/
  six-producer closeout, then stacked draft PR against feat/bulk-inspection (#33).
  Explicit OS time accuracy/trust, independently current minima, whole-system
  restore, native deployment and operational lifetime remain unqualified gates.

- Previous lane: feat/bulk-inspection, input36fc91b, implementation29e87e7.
  PR #33 stacks on feat/roster-placement (#32); evidence closeout9051538.
  Head4aeebf8 passes Validate42594434031: Windows18m20s, Linux6m8s, secrets5s;
  ready for review, unmerged. Derive current exact-head status live.
  Read specs/BULK-OPERATOR.md.
  Exact existing-root CLI describe/plan/peak/status exposes the bounded controller
  without mutation permissions. bulk::Inspection separates source metadata identity
  from a writable MetadataStore/Clock. Actual CLI qualification covers fresh,
  interrupted-intent and completed states, source metadata offline, missing copies/
  namespaces/witness, strict private admission and unchanged reopened journals and
  provider file snapshots. Seventeen focused entries and one separately executed
  ignored CLI qualifier pass. Locked default/all-feature workspace suites, both
  500k metadata runs, final strict Clippy/fmt, standalone Git, fresh ordinary CLI
  cancellation and pinned older-writer refusal/legacy control pass. EVID-102
  records separate verification/validation/trace/six-producer closeout. No runtime
  code changed after29e87e7. Next: PR #33 exact-head hosted qualification,
  then full operator execution/native
  selection and admitted clock/authority integration. Explicit local
  versus external clock preference was requested asynchronously; no answer is
  recorded. This read-only work does not depend on that choice. No fallback clock,
  live root, bulk apply, migration or deployment was introduced. Full operator
  execution, native selection, clock/authority, provider/lifetime acceptance remain.

- Previous lane: feat/roster-placement, input106f6f5 (PR #31 head at entry),
  implementationada8b21.
  PR #32 stacks on feat/bulk-retirement (#31); separate evidence closeoutfe704ea.
  Head36fc91b passes Validate42588131136: Windows18m6s, Linux5m47s, secrets6s.
  Ready for review, unmerged. Derive current exact-head status live.
  PL-01 cross-namespace advisory precedence uses fresh explicit roster-scope
  bindings and distinct design-2 placement enrollment. Existing namespace-local
  bindings keep their bytes/identity and behavior. Planning and guarded ordinary
  eviction share raw active/history/retained buckets; all subject fences hold
  remote active use through removal. Hard retention and additive policies survive.
  Read specs/ROSTER-PRECEDENCE.md. Twenty-five placement entries (one helper),
  sixteen store-protection entries, actual peer Busy77/success78, independent
  legacy/roster binding hashes and explicit pinned old-writer refusal pass.
  Locked default/all-feature workspace suites (both500k metadata fixtures),
  strict Clippy, standalone Git and fresh ordinary-retirement CLI cancellation
  pass. EVID-101 records separate verification/validation/trace/six-producer
  closeout. No implementation changed afterada8b21; no live upgrade or merge.
  Next: PR #32 exact-head hosted qualification, then bulk operator/agent/native
  selection and admitted clock/
  authority integration. Full-product/provider/lifetime/human gates remain.

- Previous lane: feat/bulk-retirement, input4478207, implementationfe073a6,
  public action binding/test correctivef999e3a.
  Draft PR #31 stacks on feat/final-provider (#30); evidence closeoute621381.
  Subsequent head106f6f5 passes Validate42584753189: Windows15m30s,
  Linux6m5s and secrets5s; PR #31 is ready for review, unmerged. The initial
  scanner download failure below is historical and superseded by that result.
  Initial hosted run42584582005: foundation jobs running at inspection; secrets
  failed while downloading the pinned scanner (HTTP504), before any scan. This
  is not a detected secret or a completed qualification. Derive new-head state
  live after this handoff; no job is successful merely because it was retried.
  Read specs/BULK-RETIREMENT.md. Exact flat namespace/source/predecessor/path
  selection resolves held/eligible objects across the complete admitted roster;
  same-source predecessor plans reuse the existing pure transition model.
  Durable accounted intent/job binding precedes source subsets and pre-admitted
  final-provider actions. Consent history is checked under final live fences,
  including prepare/abort that leaves no current membership. Cancellation stops
  future source/removal work and records honest partial effects. Alternate unused
  terminal metadata uses existing empty-review private cleanup, so the witness
  job can finish. No new cache/journal engine, source event format or CLI is added.
  Sixteen focused entries pass (one helper, ten actual abrupt exits plus a process
  peer refused by the batch fence). An unrelated batch progresses on the same
  witness. Six independent ASCII canonical/hashlib vectors pin held/eligible plan,
  enrollment, intent and terminal framing. Final-runtime locked default/all-feature
  workspace suites pass in separate targets, including both 500,000-binding runs.
  Strict all-feature/all-target Clippy, standalone Git, fresh ordinary-retirement
  CLI cancellation and explicit pinned pre-final writer refusal/legacy control
  pass. Cancellation of never-started stale actions creates no new admission;
  interrupted alternate-terminal disposal resumes its same empty review.
  Broad suites coverfe073a6; focused sixteen/Clippy/standalone also cover the
  read-only iterator/test additionf999e3a. EVID-100 records separate evidence/
  validation/trace/six-producer closeout. Next: exact-head hosted checks, then
  PL-01 cross-namespace active/history precedence correction in a separate lane.
  Full age/category/
  native adapters, operator/UI surfaces, large-file/real provider/current clock/
  authority adoption, journal/metadata lifetime and human acceptance remain open.
  Foundation fixtures only; the full plan is active and incomplete.

- Previous lane: feat/final-provider, input273e639, implementation9f5e8c0,
  evidence closeout9c0d1ca, handoff4478207, PR #30 against feat/final-retirement.
  Read specs/FINAL-PROVIDER.md and FINAL-RETIREMENT.md. Fresh explicit design-8
  LocalStore uses existing fences, capacity ledger and publication; no upgrade,
  live root or CLI activation. New immutable final intent/release/terminal records
  separate pre/post-release cancellation, deliberate absence and current unknown
  presence. Explicit library reacquisition uses a new context-bearing birth and
  accounted streaming; old deletion retry cannot remove that placement. Read-only
  status adds no commands and grants no removal authority. Canonical ASCII/Python
  vectors include final records and fresh reacquisition framing. Thirteen focused
  entries pass (one helper, nine actual child exits): same-hash/new-retention,
  late clock expiry/rollback, foreign grants, budget refusal, status and public
  placement refusal followed by explicit reacquisition/convergence. Final-runtime
  default workspace, strict all-feature/all-target Clippy and standalone Git pass.
  Final-runtime all-feature workspace and explicit fresh ordinary-retirement CLI
  cancellation also pass, including 500,000 bindings. The first all-feature run
  exposed an overstrong S3 per-round-winner assertion. Twenty unchanged races
  passed; a new held-reader fixture deterministically makes both peers Busy and
  verifies serial convergence after release. All fifteen corrected S3 tests pass;
  unexpected error codes remain failures. No S3 runtime behavior changed.
  Pre-final tig-storage artifact refusal passes with an accepted legacy control;
  CI now builds pinned273e639 and explicitly runs that ignored qualification.
  Workspace compilation exposed and fixed the new placement hold's CLI mapping.
  EVID-099 and docs/reviews/2026-11-09-final-provider.md separately record
  verification, synthetic validation, trace and six producer lenses. No runtime
  code changed after9f5e8c0. PR #30 is OPEN/ready after exact-head Validate42571000366
  at4478207 Windows/Linux/secrets SUCCESS was observed. No merge/deployment.
  The active lane implements bounded
  bulk intent/consent and exact historical selection/source settlement over these
  ports, retaining held siblings and explicit per-object/store outcomes. Budget,
  authority/native clock/provider/large-file, complete bulk selection/source
  settlement, metadata expiry/continuation/epoch migration and human acceptance
  remain required. Full plan remains active. Foundation fixtures only.

- Previous lane: feat/final-retirement, implementation89d260e, closeout273e639.
  PR #29 is OPEN and ready, observed at273e639 with exact-head Validate42564231857
  Windows/Linux/secrets SUCCESS. Content-wide live namespace membership clearance
  and trusted minimum ancestry are qualified in EVID-098. Clearance grants no
  provider release/removal; the active provider lane adds those separate effects.
  Thirteen focused entries (one helper/one actual exit), 48 active source-claim
  regressions, both old-reader qualifiers, default/all-feature workspace including
  500,000 bindings, strict Clippy/standalone/fmt/L0 and explicit ordinary-retirement
  CLI cancellation passed at that historical checkpoint. No merge or deployment.

- Previous lane: feat/retained-subset, implementation2fecfab/corrective002a850,
  closeout59174b1, PR #28 ready after exact-head run42559095758 Windows/Linux/
  secrets all succeed. It was marked ready only after those results were read.
  RETAINED-SUBSET.md and EVID-097 scope same-owner/class historical survivor
  protection before exact original predecessor retirement. Atomic history avoids
  transient active-use suppression without new source event bytes. Local locked
  default/all-feature suites, strict Clippy/standalone, focused/process/CLI,
  both old-reader qualifiers, explicit retirement CLI cancellation, Node/restore
  and L0 passed. One source per partial batch consumes finite lifetime slots.
  No payload deletion, full bulk selector, final-provider release or metadata
  expiry was implemented. Derive later merge state live; no PR is merged here.

- Previous lane: feat/source-settlement, implementation51c5acc, closeout63f2c96,
  PR #27 ready after exact-head Validate42554592615 Windows/Linux/secrets and
  issuer42554587752 succeed. It was marked ready after rechecking all four actual
  results. SOURCE-SETTLEMENT.md and EVID-096 scope owner branch/draft withdrawal,
  current-source observations and bounded transactional revision predicates.
  Local default/all-feature workspace, strict Clippy/standalone, Node/restore,
  both explicit artifact qualifiers and L0 passed. Hosted issuer remains an empty
  synthetic scope with zero payload effects. Full automatic routing, durable
  rename/controller revocation, native/private coverage and later roadmap gates
  remain open. Derive subsequent PR/merge status live; nothing is merged here.

- Previous lane: feat/index-lifecycle, input195bd0a, implementation51dd2b2,
  PR #26 ready at9c2a29f after hosted run42549837195 Windows/Linux/secrets pass. Read
  specs/INDEX-LIFECYCLE.md for GIT/PROVIDER controls/scope/entry/exit. Default
  lifecycle::index/tig-lifecycle index compose exact staged pointer obligations,
  finished/read-back metadata, existing atomic source adoption and one existing
  placement page. Conflict alternatives remain draft byte obligations, not merged
  content. Other source issues hold adoption. Fifteen focused entries pass (one
  helper, two actual abrupt process exits); linked indexes, stale/foreign/disabled
  plans, offline status and real CLI job-aware demotion/no-op catch-up execute.
  Locked default/all-feature workspace suites pass in separate external targets,
  including the large metadata case; strict all-feature/all-target workspace
  Clippy, standalone Git, fmt, 70 Node flows, three restore-helper cases, both
  explicitly executed artifact qualifiers and portable L0 pass. Do not overlap
  builds against a target under CLI tests. No source journal/event format changed.
  EVID-095 separately records verification/validation/trace/six producer lenses.
  Hosted closeout run42549837195 at9c2a29f passes Windows/Linux/secrets; PR #26
  was marked ready only after those checks passed. Derive later merge state live.
  Source settlement and the remaining full plan continue in the active lane. No live cache,
  hook or consumer is attached. Branch withdrawal/rename, automatic commit/draft
  settlement, private overlays/native closure, provider/large-file/authority/
  lifetime, final deletion, metadata expiry and user acceptance remain open.

- Previous lane: feat/github-trigger, base1c253ea, implementation503adca,
  issuer-time fixb3aced0; PR #25 is ready at195bd0a. Read
  specs/GITHUB-TRIGGER.md for input controls, bounded source/intake/coordinator
  contract and adoption gates. Optional feature github adds tig-trigger serve,
  independent RS256/OIDC verification and bounded metadata-only GitHub source;
  existing local source identities remain compatible through CommittedSource.
  Signed synthetic HTTP requests exercise actual temporary working/backup copies,
  expired/rollback clocks, source outage, paging/budget/duplicate reuse and head
  drift after physical progress. Sixteen library, eight authentication/CLI and
  thirteen Git/lifecycle cases pass at final focused scope. Default workspace,
  fresh-target strict workspace Clippy, standalone Git, fmt, 70 Node flows, three
  restore checks and both explicit artifact qualifiers pass. The isolated broad
  all-feature rerun passes before the final issuer-time correction, followed by
  final all-feature changed-package library/authentication/lifecycle suites after
  it. Do not overlap Cargo builds against a target while CLI tests execute.
  Earlier shared-target binary replacement caused
  upload failures; stale compiler metadata required fresh target directories.
  Hosted issuer run42546575041 passes atb3aced0 through the actual foreground
  binary and GitHub current API with a random empty fixture scope: zero objects,
  streamed bytes and effects. Ordinary ignored entries are not proof. Hosted
  exact closeout run42547240764 at195bd0a passes Windows/Linux/secrets, and
  issuer run42547232445 passes. PR #25 was marked ready after all checks passed.
  Node Action examples install nothing. EVID-094 separates verification,
  synthetic validation, trace and six producer lenses from implementation.
  PR #24 head1c253ea is open/ready; hosted Windows/Linux/secrets all pass.
  EVID-094 records separate local verification/validation/trace/six producer
  lenses. No complete priority-6, provider adoption, final deletion or metadata
  lifetime claim follows. Derive later merge state live; the goal stays active.

- Previous lane: feat/git-placement, inputa678381, implementation7432739, local placement
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
  and evidence/review closeout are separate commits. PR #24 is ready with passing
  hosted qualification at1c253ea; derive subsequent merge state live.
  Private partial cleanup remains explicit; CLI requires roots open at entry;
  page success alone does not establish full selection convergence.

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
