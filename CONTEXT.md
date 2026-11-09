# Tig maintainer context

This repository is the development source of truth. A new session can start here
without an older consumer-repository conversation. Follow docs/DEVELOPMENT.md and
context/RESUME.md; PRODUCT_PLAN.md orders the next product slices. Git/PR checks
determine what actually merged. Source availability does not deploy a running UI.

Tig is generic experimental Rust infrastructure. Machine-local enrollment,
cache paths, credentials, producer bindings and private acceptance evidence belong
outside this repository. docs/ARCHITECTURE.md maps the implementation;
PRODUCT_PLAN.md tracks remaining work. Historical test totals and target diagrams
are not current implementation claims.

The workspace contains tig-core, tig-store, tig-git and optional tig-assets.
Basic Git/LFS does not depend on typed assets. Captured Git metadata and admitted
schemas drive optional resolution, graphs, contexts, revisions and deliveries.
The catalogue provides works, search, children, coverage and storage comparison.
Project meaning remains external configuration.

The explicit tig-lifecycle CLI/library connects an enrolled exact committed ref
to verified persistent metadata and atomically adopted desired claims. Duplicate,
stale, revert and active-job/history behavior have synthetic Git fixtures. Source
journals reuse existing Prepare/Adopt event bytes in one transaction; status
preserves historical receipts independently of Git/metadata availability.
The explicit tig-lifecycle reconcile command now composes bounded local copy,
retention/handoff, ordinary eviction, cancellation and exact-byte reacquisition
through tig-store::placement. EVID-093 scopes temporary Git/store qualification;
page success does not establish complete selection convergence. See
specs/PLACEMENT-RECONCILIATION.md. Authenticated triggers, full lifecycle/native/
provider adoption, unattended partial recovery, large files, bulk final deletion,
authority handoff and lifetime remain required plan work.

Optional github feature now adds strict OIDC verification, bounded independent
GitHub metadata capture, foreground tig-trigger and multi-page coordination over
the same selection/placement APIs. EVID-094 scopes synthetic signed-request
working/backup copies and a separate actual hosted-issuer/CLI empty-scope test.
The latter performs zero payload operations. An inert Action client installs
nothing; external ingress/OS isolation, full lifecycle/provider/large-file/lifetime
adoption and later final-deletion/metadata milestones remain open.

The bounded staged-index lane adds explicit draft-only plan/status/apply/
resume/reconcile over existing guarded metadata/placement. Conflict-stage pointer
alternatives are unique byte obligations, never a merged source selection. Fifteen
focused entries (one child helper/two abrupt exits) pass; EVID-095 separately records
passing local broad qualification, synthetic validation, trace and producer review.
Hosted current-head qualification remains separate. No hook or live cache is attached.

Owned source settlement is bounded under SOURCE-SETTLEMENT.md: local branch
absence and an adopted committed successor can authorize withdrawal of only the
owned current source, preserving predecessors. Bounded source-revision conditions
are checked in the existing transaction; event bytes are unchanged. Shared local
storage preflight and fresh admitted-source observations prevent false convergence.
EVID-096 separates passing local broad/focused checks, synthetic validation, trace
and producer review from pending hosted current-head/adoption gates.

Existing-byte readers and explicit local publication/restore/retention primitives
have bounded fixture evidence. The first tig-store orchestration slice composes
input-bound plans and source-preserving copy/retention with an explicit tig-storage
CLI; specs/STORAGE-ORCHESTRATION.md limits its scope. Protection completeness remains
caller-declared for that copy-only CLI. The opt-in managed local library now adds
bounded ordinary replica retirement/cancellation and explicit reacquisition under
specs/MANAGED-REPLICA-RETIREMENT.md. It requires scoped protection, retained
survivors and incarnation-bound recovery in synthetic stores. Full live-project
custody, native dependency coverage and production eviction remain open. Catalogue
alignment/older-copy actions are held previews. No cache initialization, scan,
hydration, daemon or hook installation on resume.

Optional tig-git feature upload adds an explicit standalone Git LFS Basic upload
agent and tig-lfs upload plan/peak/status/apply/resume over existing OID authority,
guarded source readers and accounted local witnesses. Full remote hash/length
readback and optional verify notification precede completion. Private action
credentials are explicitly scoped and excluded from plans; upload defaults off.
See specs/LFS-UPLOAD.md. This is bounded 8 MiB source-preserving transfer, not
retained backup custody, remote quota or automatic Git lifecycle reconciliation.

The explicit `tig-storage retire` interface now composes that library over existing
managed stores and private policy profiles: plan/apply/status/resume/cancel and
new-operation reacquisition. See specs/RETIREMENT-AGENT-CLI.md. Survivor grants,
domains, role assignments and finite budgets are admitted independently from
portable plans; all three execution permissions default to disabled. Status
separates historical outcomes from freshly verified present/absent/unproven bytes.
This does not attach a catalogue action or enable automatic lifecycle execution.

Optional tig-store feature s3 adds a source-preserving conditional S3 PUT/full GET
readback adapter and explicit tig-storage s3 plan/peak/status/apply/resume. Small
intent/completion witnesses reuse existing accounted local publication/ledger;
presigned URLs remain private and absent from portable plans. The synthetic HTTP
fixture covers collisions, lost replies, corrupt/missing readback, denied access,
bounded cancellation and source preservation. It grants no retained custody or
removal authority. See specs/S3-PUBLICATION.md; real cloud/IAM/TLS/provider and
remote quota qualification, LFS upload and remaining roadmap gates stay separate.

Read AGENTS.md and the active wave/pulse. docs/vtrace/README.md routes controls
and evidence. Preserve stable IDs and distinguish historical results, current
code and target behavior. Roles are producer checklists, not independent reviewers.

The public repository began with a clean source snapshot. Private consumer notes,
original history and hosting context remain outside this repository. Historical
evidence SHAs in documentation identify original checkpoints; they are not
promises that those commits exist in public history. Port reviewed generic
patches rather than importing private ancestry. See docs/PUBLIC-RELEASE.md.

Source/docs remain below 6 MiB, individual files below 128 KiB. No media, builds,
runtime databases, inventories, raw receipts or private fixtures in Git.
Use explicit build/temp roots. No portfolio/renderer checkout is a runtime
dependency. Vendored checkers retain source pins/licenses; see docs/INTAKE.md
and NOTICE.md. Run tools/validate.ps1 and git diff --check before commits.
Rust changes also require fmt, appropriate locked tests, strict Clippy and
integration cases. Hosted Windows/Linux CI is a separate qualification.

Historical subset prerequisite: source_claims::subset and explicit tig-source
subset plan/status/apply/resume protect unselected flat bindings with a same-owner/
class survivor predecessor before conditional original retirement. Atomic
Prepare/Adopt/Withdraw exposes no artificial active use. See RETAINED-SUBSET.md
and EVID-097. These are claim transitions, never payload deletion or full bulk/
age/native selection. Source/journal lifetime and all later full-plan gates remain.

Final-retirement prerequisite: final_retirement::claims guards object membership
across a separately admitted content-scope namespace/policy/minimum roster,
independent of target-place requirements. Shared ancestry checks refuse weaker
handle minima/restored known history. Roster summaries grant no removal or release.
FINAL-RETIREMENT.md and EVID-098 separate this bounded implementation from the
remaining provider format, grant-release/intent/unlink/cancellation executor,
intentional-absence placement integration, bulk selection and metadata lifetime.

The active final-provider lane implements the bounded fresh design-8 local layer
under specs/FINAL-PROVIDER.md: immutable intent/release/terminal, exact owner grant
release, unlink/cancellation, explicit fresh same-hash acquisition and read-only
status, using existing capacity/publication/fences. It is under qualification,
not live enabled or production accepted. Bulk selection/consent/source settlement,
complete age/native closure, large-file/provider/authority qualification, operator
interfaces and metadata lifetime remain required. Do not infer goal completion
from this library layer; derive current checks from the active lane and PR.
