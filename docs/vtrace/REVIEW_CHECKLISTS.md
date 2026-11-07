# Scoped review checklist

Gate: documentation/specification/work-package design only. These decisions do
not pass provider readiness, archival deletion or private adoption gates.

| Gate | Item | Required | Decision | Evidence / follow-up |
| --- | --- | --- | --- | --- |
| Design baseline | Mission/CONOPS and owned observable requirements exist | yes | pass_with_risk | MISSION, CONOPS and REQUIREMENTS; quantitative workload choices are package entries |
| Design baseline | Every requirement/spec has implementation allocation or disposition | yes | pass_with_risk | SPECIFICATION_BASELINE, IMPLEMENTATION_PLAN, WORK_PACKAGES and exact TRACE |
| Design baseline | Current mechanism evidence is distinct from target safety | yes | pass_with_risk | EVIDENCE/VERIFICATION; 13 pure tests, no provider durability claim |
| Design baseline | Public/private/package boundaries and compatibility are controlled | yes | pass_with_risk | INTERFACES, PACKAGE_BOUNDARIES, CHANGE_CONTROL and INTEGRATION_PLAN |
| Design baseline | Each package has entry/exit, allowed surfaces and L0/L1/L2 | yes | pass_with_risk | WORK_PACKAGES and LANGUAGE_PROFILES; feature fixture names frozen before closeout |
| Design baseline | Required review lanes and open critical risks have owners | yes | pass_with_risk | REVIEW/RISKS; no waiver for future live action |
| Design baseline | Intended-use validation remains distinct from unit tests | yes | pass_with_risk | Five planned VAL runs; no private acceptance inferred |
| Design baseline | Canonical assessment and portable checks have scoped meaning | yes | pass_with_risk | Exact-ID/profile correction and successful structural assessment; no certification |

Before each implementation closeout, reopen the applicable checklist with its
actual code/source/provider scope and executed evidence. All full requirements,
invariants and mitigation targets remain OPEN/planned until their named proof.

WP-001 reopened checkpoint: L0 structural/fmt/whitespace, L1 39 locked tests and
all-target Clippy, and L2 public API synthetic caller/fault/reference tests pass.
The eight-lane implementation review is in REVIEW.md; EVID-007/008 and
IDENTITY-AND-TREES.md define exact scope. Full VAL/provider gates remain open.

## Stage review

Producer-applied six-role lenses: pass_with_risk. CK-01: a closed checklist could
be reused outside the design gate. Resolved by explicit gate scope and mandatory
reopening for implementation/adoption; readiness dispositions stay in REVIEW.md.


WP-002 reopened checkpoint: L0 structural/fmt/whitespace, L1 locked tests/Clippy,
and L2 synthetic API callers pass within the scope in BYTE-WORKTREE.md. EVID-009
and the eight-lane producer review cover this package. No provider, durable
ownership, live storage or real Git/index gate is passed.


WP-003 reopened checkpoint: component/isolated local restart proof passes within
PUBLICATION.md. EVID-010/011 and producer eight-lane review leave provider-durable,
complete restore, capacity/admission and hostile path-race proof OPEN. Status is
in progress; structural checks cannot promote this store to live custody.


WP-004/006 reopened checkpoint: L0 structure/fmt, L1 model/port regressions and
L2 local child-process fences pass within SCOPED-COORDINATION.md. EVID-012/013
are partial; durable coordination, complete claims and actual custody/removal
gates stay OPEN. Both packages remain in progress; no prerequisite waived.


Reservation-journal checkpoint: L0 source/structure/fmt, L1 staged/core and journal
cases, L2 actual temporary DB/process tests pass in EVID-014 scope. Full durable
protection, witnessed restore, physical admission and cache catalog gates remain
OPEN. No service installs/live bindings or full work-package closure occurred.

Accounted-publication checkpoint: EVID-015 exercises admission-before-write,
exact attempt binding, conservative allocated-file-data measurement, restart and
read-only settled replay. L0/L1/L2 remain scoped to ACCOUNTED-PUBLICATION.md.
Physical volume enforcement, witnessed restore, partial cleanup and removal are
OPEN; producer role review is not independent approval.

Local sampling checkpoint: verify explicit random-instance enrollment, fresh-only
probe start, caller-available space, failure/expiry without restored freshness,
no database transaction across sampling and retained racing charges. EVID-016
covers this component; native domain/quota detection and full enforcement remain
OPEN. Producer role review is not independent approval.

Private cleanup checkpoint: review exact dry-run evidence and explicit discard
authority; commit cancellation before removal; recheck publication under its fence;
retain in-flight capacity on uncertainty and measure remaining metadata before
terminal disposition. EVID-017 does not authorize acknowledged-object eviction,
final archival deletion, live paths or restore/whole-volume safety claims.

Replica verification checkpoint: match exact publication and bytes, retain the
same object guard, reject stale/invalidated evidence, deduplicate declared failure
domains and preserve honest acknowledgement. EVID-018 reports retention and
eviction authority as false. Saved records cannot stand in for live sessions.

Durable retention checkpoint: reserve before metadata writes, verify target bytes,
publish immutable owner/operation claim under shared object and scoped metadata
fences, settle measured metadata, and check retained records before granting
exclusive access. EVID-019 covers normal restart only; no automatic expiry,
owned release, stale-restore authority or final-object eviction is admitted.

Owned handoff checkpoint: explicit exact owner decision, fresh protected successor,
source exclusive scope, marker before old claim removal, no old-grant resurrection
and read-only settled replay. EVID-020 is claim metadata only. EVID-021/source
budget controls do not admit runtime files, payload removal or consumer cutover.

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

## Native intake review checks

Producer-applied: exact raw pins/definition ID, missing versus null, immutable
parsed identity, duplicate slots/sizes, field/default declarations, bounded IO,
no private authored/provenance/status output and no cache/daemon initialization.
Confirm complete_semantic_coverage remains false for opaque semantic rules;
standalone tig-core/store/git cannot depend on the optional asset crate.

## Native graph checks

Check admitted profile identity, exact view membership, snapshot-selected versus
frozen references, unchanged parent files, binding versus recipe identity,
production/output exclusion, build versus evidence cycles, explicit payload
selectors, unknown coverage, bounded callbacks/response bytes and edge attempts.
A provider must qualify membership rather than simply echo a requested checkpoint.

## Native resolver review checks

Check captured Git-object reads, native namespace uniqueness, complete explicit
scope, definition/schema bindings, no filters or working-file fallback, retained
metadata not changing current selections, bounded batch/time/growth behavior,
ordered stream metadata, authored access with default redaction, and optional
feature CI coverage. Native capture is an in-memory index, not durable custody.

## Existing-reader checks

Check exact identity/size, no corrupt fallback, explicit roots/layouts/protection,
no cache/alias writes, held handles/permit lifetime, no mandatory downgrade,
parent namespace stability for path consumers, live limits and actual child exit.
Verify cooperative alias use alongside the real Git LFS client and preservation
of the public cache Layout path. Unsupported protection cannot become a pass.
