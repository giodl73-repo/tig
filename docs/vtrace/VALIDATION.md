# Intended-use validation

EVID-076 exercises retained-subject selection and separate target bindings with
synthetic API/CLI inputs. It accepts no actual consumer migration, remote
publication, source/job lifecycle, custody, runtime switch or full VAL-005 run.

EVID-075 adds synthetic singleton receipt and compiled CLI coverage. It does not
accept actual consumer records, current remote contents, independent custody,
runtime installation or the full VAL-005 workflow.

This is an acceptance plan. No complete user workflow or private consumer
cutover is accepted by the current 13 pure tests. Each run names actor, exact
source/provider/client, expectations, observations, limits and accepting owner.

| Validation ID | Need / workflow | Acceptance demonstration | Packages / actors | Status |
| --- | --- | --- | --- | --- |
| VAL-001 | NEED-001/003/005; CON-001/002 | Generic Git-only project opens a complete logical view, changes one file, seals/adopts its exact leaf pointer and reads shared unchanged bytes without mass hydration; measured work stays within pinned budgets | WP-001/002/005/008/009/015; user/CI | planned |
| VAL-002 | NEED-002; CON-003 | Interrupted publication, lost reply, stale/torn metadata and read/removal races preserve acknowledged recoverability or honestly block destructive work; independent retention assumptions are verified | WP-003/004/007/013/016; recovery/storage owner | planned |
| VAL-003 | NEED-002/003; CON-004 | Current and V-1 versions land at configured places; history/archive are recoverable; working eviction preserves all claims and yields measured physical reclaim without unbounded scratch | WP-006/007/013/014; operator | planned |
| VAL-004 | NEED-004; CON-002/005 | Two worktrees and CI publish/use shared objects concurrently; merge/rebase/missed hooks and pending replication preserve source ownership and active readers without a global pause | WP-004/005/008/009/013; creator/CI | planned |
| VAL-005 | NEED-005; CON-006 | Optional typed generic project resolves dependencies/variants/profiles, reuses unchanged consumption and respects exact evaluation/delivery authority; standalone byte access remains independent | WP-010/011/012/015/016; typed/native adapter owner | planned |

A synthetic demonstration can validate the synthetic workflow within its scope;
it does not automatically accept a private production project. Private migration
requires its owning session's approved exact mapping, byte/recovery/rollback
checks and scoped operator acceptance. No screenshots/private payloads move into
Tig to manufacture evidence. Consent/publication and storage safety stay distinct.

WP-001 supplies the pure identity/tree component of VAL-001 through EVID-007/008.
It does not change that intended-use run's planned status: Git/LFS, byte worktree,
catalog, materialization and actual storage measurements are still absent.

## Stage review

Producer-applied six-role lenses: pass_with_risk. VAL-01: a green test count could
be mistaken for acceptance of the user's multi-store lifecycle. Resolved by five
named intended-use runs and explicit owners/measurements; all remain planned.
Scope-limited accepted deferrals must name which need is not yet validated.

VAL-02 follow-up for CON-002/006: accept explicit mixed stream bindings under
the pinned definition, catch semantic/structural conflicts at merge, and permit
valid source fixes to accumulate without immediate renders. A catch-up run must
freeze exact roots/definitions/variants/producers, reuse unchanged results,
render affected targets once per admitted fingerprint and disclose fixes arriving
after the snapshot. Prove current-source, produced-result freshness and delivered
version remain separate. These runs remain planned; a recast schematic with null
selected results validates only metadata wiring, not this full behavior.


WP-002 adds pure binding-state/merge components through EVID-009. Its scene and
dataset callers validate only that synthetic API scope. VAL-001/004 remain planned:
actual Git/index transitions, durable ownership and byte providers are absent.


WP-003 supplies a process-restart publication subset under EVID-010 using
synthetic temporary stores. Four child exits and replay do not close VAL-002:
full provider persistence, independent custody, restore coverage and removal
fences are unproven. No production legacy consumer/cache acceptance is implied.


EVID-012/013 add pure capacity conservation and actual local OS exclusion
components. These do not close VAL-002/003/004: durable job/domain ownership,
capacity probing/admission/replay, complete claims and real retirement remain
absent. No native project/caches were admitted to these experimental modules.


EVID-014 validates only restart-persistent synthetic accounting: commands and
in-flight charges survive/rollback correctly within the declared SQLite/local
process model. Fixed free/allocation values are not physical disk measurements.
Full physical publishing/admission, active-worker restore reconciliation and
native project use remain unaccepted under VAL-002/003/004.

EVID-015 connects publication to restart-persistent admission in synthetic stores.
Measured allocation covers labelled file data only; probe values remain synthetic.
Interrupted partials retain in-flight capacity; settled retries cannot finance new
writes. This does not close VAL-002/003/004 or admit live cache operation.

EVID-016 exercises real available-space sampling and admitted publication in
owned temporary roots. Trusted enrollment is explicit; native volume/quota mapping,
external-writer guarantees and full-device safety remain unaccepted. This bounded
synthetic workflow does not close VAL-002/003/004.

EVID-017 validates bounded synthetic private-staging discard only. Explicit
authorization precedes durable cancellation and removal; final objects and
records remain preserved. Scope/path evidence is not independent retained custody
and removed allocation is not measured volume reclaim. VAL-002/003/004 stay planned.

EVID-018 validates only bounded live replica integrity/declared-domain coverage
in synthetic temporary stores. A guard ends at process death; a historical
observation does not establish retained custody. Durable retention, native domain
identity and safe final-object removal remain unaccepted under VAL-002/003/004.

EVID-019 validates bounded synthetic process-restart retention: installed claims
continue blocking exclusive access after sessions exit. It does not validate
native independent custody, stale-backup recovery, complete claims, owned release
or final-source retirement. VAL-002/003/004 remain planned for full intended use.

EVID-020 validates bounded synthetic owned claim handoff with payload preservation,
not complete production ownership/recovery or final eviction. EVID-021 identifies
actual cache/LFS, metadata-scale, semantic/build and reversible-consumer gaps.
Neither an audited configuration ceiling nor green component tests close intended
legacy consumer use under VAL-002/003/004/005.

## Download-only LFS compatibility checkpoint

Input c68d7fad00b7fa6da528255f9b242a42e4dcb048; WP-005, REQ-006/008/009,
SPEC-005/007, IF-005/008/009, PKG-004. Controlled scope and entry/exit:
specs/LFS-CACHE-INTEROP.md. EVID-022 adds 12 Windows component entries;
EVID-023 adds three actual-client temporary-repo fixtures. Windows total 261;
Linux adds a Unix cross-filesystem entry, conditional on /dev/shm availability.
L0 structure/fmt/VTRACE; L1 locked workspace tests/all-target Clippy; L2 current
and historical checkout, two simultaneous clients, corruption and repaired retry.
No private caches or consumer configuration changed. Full WP-005, requirements,
validation, staged/ref adoption, upload and complete accounting remain OPEN.

## Exact Git source checkpoint

Input 07cfa97ce60a0d74f5923aaa96cdad8aad0e994b; WP-005, REQ-001/006/008/009,
SPEC-006/007, IF-008/009, PKG-004. GIT-SOURCE-EXTRACTION.md controls scope.
EVID-024 adds 16 temporary Git fixtures; EVID-025 connects exact source selection
through a complete OID tree to actual LFS transfer authorization. L0 structure/
fmt/VTRACE; L1 locked workspace/Clippy; L2 partial/conflicted staging, linked/bare
repos, missing/corrupt Git objects, disabled promisor hydration and historical
checkout. Windows total 278; Linux adds its Unix cross-filesystem entry. Complete
coverage is limited to explicit prefixes. Durable old/new claim handoff, global
attribute-aware extraction, large metadata scale and native adoption stay OPEN.

## Persistent source-root checkpoint

Input 5411e2be0521aef0e3722e508f31ff6355b8a66f; WP-008/009, REQ-002/004/007/009,
SPEC-005/006/007/008, IF-004/006/008, PKG-003/004. SOURCE-CLAIM-JOURNAL.md
controls scope and L0/L1/L2: structure/fmt/VTRACE, locked tests/Clippy, actual Git
root handoff plus six child exits. EVID-026 adds 19 journal entries including
helper; EVID-027 adds two Git/context fixtures. Full command cap replay executes,
not inventory-scale proof. Total 299 Windows / 300 Linux entries. Owner/revision
transitions preserve old/new roots; exact retries remain historical. Returned
obligations deny global completeness. Root closure, full source/job coverage,
final-removal fencing, restore authority, compaction and native adoption stay OPEN.

## Persistent metadata and scale checkpoint

Input 29e21c7673c4a50acade32228612ed849ac0ca3f; WP-001/008/009,
REQ-001/005/007/009, SPEC-002/005/006/008, IF-002/005/008/009, PKG-002/003/004.
PERSISTENT-METADATA.md controls L0 structure/fmt/VTRACE, L1 locked tests/Clippy,
and L2 persistent 500,000-binding synthetic scale plus actual LFS integration.
EVID-028: 14 provider entries including helper/three child exits. EVID-029: one
scale case, 13 edits adding 391 nodes / 212,992 database bytes, old root retained.
EVID-030: two core/integration cases. Total 316 Windows / 317 Linux entries.
Bounds are explicit; large-tree refusal is Budget. Full custody/admission,
compaction, complete native graph/source/job coverage and cutover stay OPEN.

## Native intake intended-use boundary

VAL-005 remains OPEN. Synthetic scene/invoice CLI fixtures validate bounded
inspection and exact stream pin lookup only. A private consumer project delivery fixture
is being registered separately; its readback does not establish complete native
primary-input graph extraction, REEL production parity or consumer cutover.

## Native graph intended-use boundary

VAL-005 remains OPEN. Scene/cel/cue and invoice/account fixtures use the public
pure API. Native Git snapshot membership, REEL schema/producer extraction and
private asset adoption still need real adapters. The legacy candidate-input
inventory must not be relabelled as selected production closure. Query-scoped
unreferenced objects are candidates only; other roots/jobs/custody still protect.

## Native resolver intended-use boundary

A private metadata lookup rehearsal resolves consumer project's three current delivery IDs
and two historical stream slots from its exact commit, without consulting the
old legacy consumer checkout. This is lookup evidence, not native REEL render acceptance,
source-independent production, fresh payload verification or full cutover.
VAL-005 stays OPEN. Concrete opaque scene consumers and custody remain needed.

## Verified-byte intended-use boundary

A private rehearsal resolves consumer project's three current delivery IDs and freshly
hashes 509,760,461 bytes in existing enrolled cache objects with Windows sharing
protection. This does not establish native REEL production parity, backup custody,
independent domains, global source/job protection or final eviction authority.
Synthetic package proof remains separate; full VAL-002/003/005 stay OPEN.

## Exact-base proposal intended-use boundary

EVID-048 exercises temporary real Git repos, linked worktrees, dirty work and
ordinary conflict resolution for small authored files. This contributes the
metadata proposal/application component of VAL-001/004. The tests do not enroll
or reconcile payload claims, accept a native schema, catch every semantic merge
conflict or demonstrate multi-place storage convergence. All five full intended-
use validations and private consumer project acceptance remain OPEN.

EVID-049 adds the explicit verified-cache CLI path to this component boundary:
synthetic projects can check and create proposals from three existing layouts
without typed assets, cache initialization or worktree changes. Full VAL-001/004
still require managed source/byte lifecycle reconciliation; all other full
validations and private acceptance remain OPEN.

## Deferred catch-up intended-use boundary

EVID-050 contributes scoped post-edit triage to VAL-001/004: two logical primary
fixes can produce one stale output candidate without rendering intermediate
versions. Frozen references intentionally retain their older bytes. A 1,000-key
fixture verifies bounded batching, not complete project discovery or native scale.
Actual consumer consumption, execution/adoption and all full validations stay OPEN.

EVID-051 adds native read-only current-output assessment to the scoped VAL-001/004
triage path. Reopened metadata can reproduce matching/stale outcomes without Git
or working metadata, while legacy evidence stays Unproven and unavailable evidence
stays Blocked. This is synthetic metadata-only replay, not media recoverability,
producer truth, storage convergence or private consumer cutover. Full VAL gates
remain OPEN.

EVID-052 contributes an authored-schema check to VAL-005 without declaring full
native type/variant/producer meaning. Synthetic scene/invoice CLI cases and native
Git/archive cases prove the admitted offline schema interpreter only. Optional
engine admission, exact check identity and source immutability remain separate
from approval, capacity/custody, actual family migration and full intended-use
acceptance. All complete VAL gates and private cutover remain OPEN.

EVID-053 contributes local definition-driven contract checks to VAL-005 using
synthetic media/nonmedia CLI, mixed-version arrays and native Git/archive cases.
These assertions do not resolve external records, authenticate approvals, prove
actual consumption/production or establish storage convergence. Source merging
remains separate from rendering and selection. All full VAL gates remain OPEN.

EVID-054 contributes sparse native selection to VAL-005 with synthetic cue/cel
and non-media-neutral contexts. Temporary Git/recipe fixtures qualify selected
language projection, not actual private scene compilation/consumption. A retained
metadata provider can reopen captured documents without Git; this is not full
managed variant/job control-root admission or media recoverability. All complete
intended-use validations/private adoption remain OPEN.

EVID-055 contributes declared variant interpretation retention to VAL-005. A real
temporary Git source and existing persistent metadata/journal providers preserve
an explicitly enrolled job root and replay its exact language/style choices with
Git unavailable. This is not live producer integration, actual media protection,
complete source/job enrollment or full private-family/history migration. All full
VAL gates and reversible runtime adoption remain OPEN.

EVID-056 contributes consumed-identity controls to VAL-005: one generic synthetic
control/primary/output graph retains its full job requirements while an admitted
producer excludes raw control bytes from its recipe. This does not qualify a
real native semantic projector or actual media production. Full intended-use
validation, global source/job protection and reversible private adoption stay OPEN.

EVID-084 contributes bounded source-preserving copy/restart scenarios to VAL-002/
003: generic CAD/report-like byte fixtures, shared plan/CLI identity, actual child
exits, retained destination inspection and explicit partial-batch resumption. It
is mechanism verification, not an owner-run live storage workflow or independent
acceptance. Complete source/job coverage, catalogue application, merge automation,
eviction, large media/provider scale and final deletion remain unvalidated.

EVID-085 contributes exact source-root demand to VAL-003: branch/draft/job/
retention roots retain old/current/pending declared bytes, shared obligations
combine and missing coverage holds the pure planner. One temporary-store copy
uses the actual snapshot basis and refuses a changed-context retry. This is not
actual producer-consumption closure, enforced source admission, physical custody,
Git-triggered orchestration, live adoption or final-deletion acceptance.
