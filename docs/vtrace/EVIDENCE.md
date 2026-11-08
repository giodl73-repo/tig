# Evidence index

EVID-082: bounded multi-asset history service candidate,
[contract](../../specs/HISTORY-BATCH-COMMAND.md) and
[producer review](../reviews/history-multi-entry.md). Eight actual temporary
Git/LocalStore cases pass: one captured source/two-file sparse proposal,
reordering and no-op replay, late invalid candidate without a provider, identity/
scope/stale-path rejection, request bounds, later-object capability preflight
before any write, interrupted publication preserving acknowledgements and exact
retry, wrong readback place/corrected retry, and the full64-item boundary. Nine
legacy/sequence command cases and strict all-feature/all-target Clippy also pass.
Final full-workspace/compatibility and exact-head hosted qualification remain
pending. No real family conversion, consumer upgrade, complete provenance/custody
or reclamation is proven by these synthetic operations.

EVID-081: ordered history preparation and explicit sequence CLI candidate,
[controlled scope/review](../reviews/history-batch-entry.md). Four pure tests
exercise image/document serial equivalence, original-base preservation, generated
metadata reuse, read-once retry, exact net no-op, late failure, cumulative byte/
document bounds, 64-operation boundary and stale/oversized input refusal. One
actual temporary Git/metadata-store fixture finishes/reopens the combined history,
creates one sparse proposal without changing HEAD/index/worktree, checks selected
history and refuses the stale original. Three real CLI tests cover combined
register/select/readback/proposal/retry/application/no-op, late failure before
provider construction, invalid sequence shape, and legacy single-step record
compatibility. All eight focused tests and strict all-target/all-feature Clippy
pass locally. Final full-workspace/compatibility and exact-head hosted qualification
remain pending at this entry. No user caches, runtime upgrades, all-family
conversion, complete producer semantics, custody or reclamation are claimed.

EVID-080: acknowledged working-owner handoff, PROTECTED-WORKING-HANDOFF.md.
Nine new actual settled-retention/journal cases pass. Explicit metadata finish/
reopen precedes guarded new/old-retained byte verification; only then do existing
abort/withdraw commands retire temporary working ownership. Four phase failures
reopen/retry without losing old current protection; completed replay is historical
and returns no fresh guards. Foreign journal, changed reuse, guard admission,
missing/weak/corrupt retention and stale successor refuse. Unrelated journal
progress and physical ingest continue; native destination identity cannot be
relabelled. Recovery reloads the retained host request/intent after volatile loss.
Tests use owned synthetic LocalStores;
their limits stay unchanged. No real media/cache enrollment, independent custody,
store-grant release, payload eviction or complete consumer acceptance is proven.

EVID-079: source-journal role placement, JOURNAL-PLACEMENT.md. Four new actual
temporary journal/archive cases pass: pending/current roots keep working demand;
adoption maps predecessors to retained-only without erasing peers; journal reopen
preserves the exact projection; explicit peer withdrawal removes its working
demand. Stable journal/source identities compose without collisions. Repeated
roots share one IO budget, missing/foreign roots refuse, and empty scope remains
incomplete. These are desired assignments, not provider protection/custody or
physical reclaim evidence. Full regression/hosted and actual adoption remain open.

EVID-078: classified demand metadata roots, PLACEMENT-ARCHIVE.md. Five new archive
cases and seven classification regressions pass (12 total). Exact assignments
survive actual SQLite finish/reopen and Git detachment; journal prepare/adopt,
historical retry and independent peer roots survive reopen. Deterministic metadata
contains payload references without copying payloads; interrupted writes preserve
old roots and retry exactly. Unknown schema, noncanonical/foreign/missing/count/
path/header and operation-wide IO-budget negatives refuse. Metadata persistence
does not establish byte protection, current policy, custody or eviction authority.
Full workspace/lint/hosted and actual consumer acceptance remain separate gates.

EVID-077: first T3 classification slice, HISTORY-PLACEMENT.md. Seven new captured
Git fixtures pass, including five-of-1,000 primary-input replacements with one
peer-job obligation (four scoped working-placement candidates), exact authorized
drafts, unchanged source selection, missing retained-copy observations, unknown
claim coverage, old/delivery witness gaps, partial/untracked/unclassified history,
conflicting sizes and cumulative repeated-closure budgets. Adapter data is
synthetic; asserted place observations are not physical custody. Strict all-feature
Clippy passes. Full workspace/standalone/hosted qualification is recorded separately;
durable claim handoff, physical reclaim/restore and consumer cutover remain open.

EVID-076: explicit retained publication subjects, WP-010/011/015. The baseline
passes 29 and fails six of 35 cases; all 35 pass after the explicit stream-policy
extension. Different/absent/equal current selections, strict default report
compatibility, two targets and actual CLI/privacy checking are covered. See
[producer review](../reviews/2026-11-01-publication-retained-subjects.md).
Workspace/lint/hosted and actual consumer acceptance remain separate gates.

EVID-076 local closeout: locked all-feature workspace tests pass (689 passed
entries including subprocess helpers), strict workspace all-target/all-feature
Clippy passes, and standalone Git/LFS check passes. Hosted source qualification,
actual consumer adoption and complete publication/custody coverage remain open.

EVID-075: explicit singleton publication rows, WP-010/011/015. The initial root
receipt regression failed at policy admission; five new singleton cases and all
existing publication cases now pass (29 tests, 0.73 s). The CLI/privacy case
includes a fourth receipt shape. Full workspace/lint/hosted and actual consumer
qualification remain separate. See [producer review](../reviews/2026-10-31-publication-singleton-rows.md).
Original receipt bytes remain pinned; no provider, payload or custody authority.
Subsequent full locked all-feature workspace tests, strict all-target/all-feature
Clippy and standalone Git/LFS check pass. Hosted and consumer gates stay open.

EVID-074: bounded exact-plan restore partial inspection, WP-005/011/015/016.
Nine new cases and all project regressions pass (42 tests, 24.37 s), including
linked parent/entry refusal, active writer preservation, entry budgets and explicit
retry retaining partial candidates. Full workspace/hosted/consumer gates remain
separate; see [producer review](../reviews/2026-10-31-project-partial-inspection.md).
This is directory metadata visibility, never cleanup or custody authority.

EVID-073: bounded declared consumer evidence and explicit selector/destination
handoff outcomes, WP-005/011/015/016. Five new cases and all existing project
regressions pass (33 tests, 20.14 s). No extra byte lookup, custody or runtime
authority follows from a link. Full workspace/hosted/consumer gates stay separate;
see [producer review](../reviews/2026-10-31-project-handoff-evidence.md).

EVID-072: explicit opaque publication locators, WP-010/011/015. Four new cases
plus the three-shape CLI/privacy case pass (24 publication tests, 0.93 s).
Exact locator spelling and a separate identity domain preserve strict logical
paths without interpreting provider addresses. Full locked all-feature workspace,
strict Clippy and standalone Git/LFS check pass. Hosted and actual receipt
qualification remain separate. See [producer review](../reviews/2026-10-31-publication-opaque-locators.md).
No provider identity/current publication, custody or eviction authority follows.

EVID-071: explicit object-member target extraction, WP-010/011/015. Four new
publication cases and the two-shape CLI/privacy case pass (20 total, 0.61 s).
Key targets remain bound to the exact row subject/length, selector and admitted
policy; no receipt is rewritten. See [producer review](../reviews/2026-10-31-publication-member-keys.md).
Full local/hosted qualification and actual consumer acceptance remain separate.

EVID-070: explicit bounded host/space preflight, WP-005/006/011/015. Eight new
project cases cover exact external files, view/render independence, path relocation,
apply revalidation, bounds, unavailable/unrelated capacity roots, observed space
thresholds, advisory estimates and progress/retry after host repair. Focused
project28 and restore8 pass, as do full locked all-feature workspace tests, strict
Clippy and standalone Git/LFS check. Exact-head hosted qualification and actual
consumer adoption remain separate. See
[contract](../../specs/PROJECT-PREFLIGHT.md) and
[producer review](../reviews/2026-10-31-project-preflight.md). No environment,
license, quota, capacity admission or custody/eviction authority is implied.

EVID-069: explicit project progress evidence, WP-005/011/015. Seven additional
project harness cases exercise synced completion/child exit, torn or unlogged
completion, exact plan/profile matching, strict bounded admission and explicit
verified retry. Final affected suites, strict Clippy and standalone check pass;
the earlier workspace pass and remaining exact-head hosted gate are scoped in
VERIFICATION.md. See
[contract](../../specs/PROJECT-PROGRESS.md) and
[producer review](../reviews/2026-10-31-project-progress.md). No custody, automatic
resume/cleanup or production adoption authority follows from a progress log.

EVID-068: first Rust project/profile restore orchestration, WP-005/011/015.
Thirteen new harness cases (including child exit) and all eight existing restore
cases pass. Full locked all-feature workspace tests, strict Clippy and standalone
Git/LFS check pass. Nested/fresh roots, exact frozen source, explicit selectors,
compact 1,000-record catalog, byte/alias budgets and retry have synthetic proof.
Consumer acceptance, persistent operator recovery, host dependencies and custody
stay open; see [the producer review](../reviews/2026-10-31-project-restore-profiles.md).

EVID-067: bounded retained publication-evidence binding, WP-010/011/015. Sixteen
new focused tests pass across invoice/font consumers, exact schema/subject/receipt
pins, strict selectors/parser limits, null/duplicate refusal, large external
metadata and actual read-only CLI. Full all-feature workspace tests, strict Clippy
and standalone Git/LFS check pass. An external owner-run candidate checks 79
consumer anchors; raw private evidence remains outside Tig. Hosted qualification,
provider identity/current remote publication, custody and adoption stay separate.
See [scope and producer review](../reviews/2026-10-31-publication-evidence.md).

EVID-066: explicit selected-object restoration extends EVID-065. All eight local
Windows restore CLI fixtures pass, including selected-only records, distinct
literal streams, exact committed selection, preview/apply/retry and planned/
corrupt/missing refusal without fallback. No private bytes are in these fixtures.
Hosted qualification and consumer acceptance remain separate; see the
[producer review](../reviews/2026-10-31-selected-restore.md).

EVID-065: explicit single-stream restoration from exact Git records through the
existing verified reader. Five Windows CLI fixtures pass (preview/application,
unchanged committed selection, identical retry, different destination, corrupt/
missing source, traversal/missing-parent and junction refusal). The all-feature
workspace run passed 595 tests across 40 suites; the final focused rerun includes
the added junction case and destination readback. Warnings-denied all-target
workspace Clippy passes. Three Python path fixtures pass. Producer review and
limitations: [explicit restore review](../reviews/2026-10-31-explicit-restore.md).
Hosted platform qualification and consumer outcomes are separate gates. This
does not close full restoration, native semantic coverage, custody or eviction.

Compact evidence only. Handoff source originates at 5cbef39; WP-001 extends the
pure core from baseline 1bbfcc6; WP-003 adds local publication from 3ecce38.
Exact reviewed hashes are in the target manifest.
No future package/provider acceptance is represented as passed evidence.

EVID-040: entry 0855e7a, native archive adapter and six new native-source cases
(plus archive replay in the existing non-media invoice case). Local Windows:
116 asset/Git entries pass, including 24 native-source cases; workspace
all-target/all-feature warnings-denied Clippy passes. A real temporary SQLite
provider is explicitly finished/reopened after Git/working metadata are moved
away; current selection, retained frozen lookup and resolved graph digest match.
The 100-record replacement fixture asserts unchanged document/tree sharing and
replay of both roots. Missing/corrupt bytes, counts/keys, limits, interrupted
construction and false provider acknowledgements refuse. No media custody,
root-owner/job adoption, opaque native payload retention or eviction is proved.

| Evidence ID | Source / Command | Status | Scope / limitation |
| --- | --- | --- | --- |
| EVID-001 | crates/tig-core/tests/handoff.rs; cargo test --workspace --locked | passed | 13 synthetic tests of assigned claims and snapshot/candidate algebra; no IO/custody/fencing execution |
| EVID-002 | cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked -- -D warnings | passed | Current core/store static checks; not future provider assurance |
| EVID-003 | tests/scenarios/catalog.json, integrations.json and learning-links.json | designed | 110 intended cases with doctrine/mitigation links; not executed workflow acceptance |
| EVID-004 | .roles/parliament and stage-specific VTRACE review sections | inspected | Producer-applied local role lenses; no independent reviewer or human approval |
| EVID-005 | docs/SAFETY-PROMISE.md, specs/SAFETY.md and specs/BYTE-WORKTREE.md | inspected | Controlled safety/overlay target obligations, not proof of no-loss execution |
| EVID-006 | tools/vendor/pins.json and preserved upstream notices | inspected | Pinned development-only ROLES/PITFALL source custody; no runtime dependency |
| EVID-007 | crates/tig-core/tests/canonical.rs; cargo test --test canonical --locked | passed | 10 Art donor canonical/hash vectors, unchanged legacy framing; no typed graph/record parity claim |
| EVID-008 | crates/tig-core/tests/tree.rs; cargo test --test tree --locked | passed | 16 public-API pure tests: pinned wire vectors, exact paths, malformed/missing nodes, bounded sparse edits, old-root preservation and injected metadata write failure; no durable store |
| EVID-009 | crates/tig-core/tests/worktree.rs; cargo test --test worktree --locked | passed | 22 pure public-API tests for isolated bindings, exact candidates, three-way conflicts, shared budgets and retry; no payload publication, Git integration or durable ownership |
| EVID-010 | crates/tig-store/tests/publication.rs; cargo test -p tig-store --locked | passed | 24 entries including child helper; real temporary stores, ten protocol faults, four abrupt process exits, threaded writers; process restart only, no power-loss/capacity/fence acceptance |
| EVID-011 | docs/BACKEND-DECISION.md; official sources and REEL source audit | inspected | WP-000 choose/defer scope: local first slice, service/remote providers deferred; not installation, comparative benchmark or deployment approval |
| EVID-012 | crates/tig-core/tests/capacity.rs; cargo test -p tig-core --test capacity --locked | passed | 18 pure single-authority tests for reserved/in-flight/allocated handoff, sample watermarks, epochs and bounds; no persistent or physical reservation service |
| EVID-013 | crates/tig-store/tests/fences.rs and src/local.rs release_tests; cargo test -p tig-store --test fences --locked; cargo test -p tig-store --lib --locked local::release_tests | passed | 12 entries including helper/release unit: real local object OS fences and two child processes; no full claims, hostile namespace, network provider or removal authorization |
| EVID-014 | crates/tig-store/src/ledger_tests.rs; cargo test -p tig-store --lib --locked ledger::tests | passed | 19 entries including helper: temporary SQLite replay/commit faults, four child processes, concurrency, witness rollback and freshness; no physical admission/full restore/power-loss proof |
| EVID-015 | crates/tig-store/src/accounted_tests.rs; cargo test -p tig-store --lib --locked accounted::tests | passed | 20 entries including helper: actual temporary publication, four injected bridge boundaries, three child exits and measured scoped file data; no full-volume/free-space enforcement/retirement proof |
| EVID-016 | crates/tig-store/src/local_capacity_tests.rs; cargo test -p tig-store --lib --locked local_capacity::tests | passed | 12 entries: actual temporary-root available-space sampling, publication, freshness/replay/failure, shared-domain and racing allocation; no native volume/quota enrollment authority or full-device enforcement |
| EVID-017 | crates/tig-store/src/accounted/cleanup_tests.rs; cargo test -p tig-store --lib --locked accounted::cleanup::tests | passed | 23 entries including helper: bounded read-only preview, explicit discard, durable cancellation, real private partial removal, stale-writer/fault/retry, actual producer interruption and three cleanup child exits; no final-object eviction/full reclaim/restore authority |
| EVID-018 | crates/tig-store/src/replica_tests.rs; cargo test -p tig-store --lib --locked replica::tests | passed | 20 entries including helper: exact completed-byte verification, live object guard/freshness, declared-domain coverage, historical record bounds and process exit; no durable retention, native independence or eviction authority |
| EVID-019 | crates/tig-store/src/retention_tests.rs; cargo test -p tig-store --lib --locked retention::tests | passed | 21 entries including helper: budgeted immutable owner-scoped local claims, restart enforcement, concurrent last-slot race, exact grants/accounting and three child exits; no release/handoff, native independence, restore authority or final-object eviction |
| EVID-020 | crates/tig-store/src/retention/handoff_tests.rs; cargo test -p tig-store --lib --locked retention::handoff::tests | passed | 16 entries including helper: exact owner handoff, preserved payloads, six fault boundaries, reciprocal guards, onward custody and three child exits; no payload eviction/full authority/native independence |
| EVID-022 | crates/tig-git/tests/codec.rs; cargo test -p tig-git --test codec --locked | passed | 12 Windows entries: strict pointers/protocol, three layouts, exact authorization, corruption, admission, bounded state and ownership failures; Linux adds a Unix cross-filesystem entry (requires /dev/shm) |
| EVID-023 | crates/tig-git/tests/client.rs; cargo test -p tig-git --test client --locked | passed | Three real Git LFS temporary-repo tests: current/prior checkout, parallel clients, corruption then repair; Windows client 3.4.0; CI records its client; no upload, native enrollment or private consumer acceptance |
| EVID-024 | crates/tig-git/tests/source.rs; cargo test -p tig-git --test source --locked | passed | 16 temporary Git fixtures: exact staged/commit views, partial staging, conflicts, linked/bare repos, SHA-1/SHA-256, strict scope, missing/corrupt objects, no promisor hydration and budget refusal; no durable claim adoption or global attribute-aware coverage |
| EVID-025 | crates/tig-git/tests/client.rs; cargo test -p tig-git --test client --locked exact_git_selection | passed | One real-client source-to-transfer fixture: current commit authorizes current bytes, existing historical cache bytes remain unauthorized until explicit old-commit selection, then checkout succeeds; no retention/eviction or native adoption proof |
| EVID-026 | crates/tig-store/tests/source_claims.rs; cargo test -p tig-store --test source_claims --locked | passed | 19 entries including helper: durable root roles, owner/revision fences, abort/withdraw/retire, concurrent writers, 15 injected command boundaries, six actual child exits, corruption/known rollback, source/history limits and a 16,384-command replay/cap fixture; no root closure, global coverage, physical custody or eviction proof |
| EVID-027 | crates/tig-git/tests/source.rs; cargo test -p tig-git --test source --locked | passed | Two added fixtures: deterministic exact source/scope context and Git commit-to-durable-root handoff with metadata outside Git, unchanged inputs reused, predecessor retirement and another branch retaining old inputs; no automated native hook/cutover |
| EVID-028 | crates/tig-store/tests/metadata.rs; cargo test -p tig-store --test metadata --locked | passed | 14 bounded provider/reader entries including helper: flush, dedup, restart, three actual child exits, faults/concurrency, logical/page bounds, corrupt/missing data, known rollback and read-only access; no complete custody/admission proof |
| EVID-029 | crates/tig-store/tests/metadata.rs; cargo test -p tig-store --test metadata --locked half_million -- --nocapture | passed | One actual disk-backed 500,000-binding synthetic build; reopen uses no node scan, bounded point reads, 13 sparse replacements, old/new roots preserved; 391 new nodes and 212,992 database-growth bytes in local Windows run; not native media/graph scale or eviction evidence |
| EVID-030 | crates/tig-core/tests/tree.rs and crates/tig-git/tests/client.rs; cargo test --workspace --locked | passed | Two new cases: explicit inventory limits classify as Budget, plus persisted read-only metadata -> source journal -> actual Git LFS; unflushed roots refuse access, flushed roots work; legacy profiles retain defaults; no native cutover |
| EVID-031 | crates/tig-assets/tests/native.rs; cargo test -p tig-assets --test native --locked | passed | 12 pure native intake cases: separate exact pins, raw whitespace identity, changed definitions/schema/ID, declared fields/defaults, ordered mixed streams, duplicate slots/conflicting sizes, planned null, strict malformed/optional-null/budget refusals; no semantic rule execution or complete graph authority |
| EVID-032 | crates/tig-assets/tests/cli.rs; cargo test -p tig-assets --test cli --locked | passed | Three actual read-only CLI fixtures with scene and invoice consumers, redacted summaries, exact slot lookup, missing/changed inputs and bounded file reads; no private media, cache attachment or native producer integration |
| EVID-034 | crates/tig-git/tests/native_source.rs; cargo test -p tig-git --features assets --test native_source --locked | passed | 13 real Git/CLI cases: exact commits/indexes, unchanged-parent cel update, explicit retained metadata, duplicate/conflict/mode/budget refusal, generic ordered-stream consumer, redaction and 1,000-record multi-batch capture followed by lookup with Git directory detached; metadata only, no payload or renderer/custody proof |
| EVID-036 | crates/tig-store/tests/existing.rs; cargo test -p tig-store --test existing --locked | passed | Ten Windows entries including a helper and an actual child exit: three existing layouts, held read/seek, corrupt/size refusal, no creation, budgets/permit release, Windows object/ancestor sharing protection and cooperative aliases; Linux has an additional symlink case; no custody/eviction proof |
| EVID-037 | crates/tig-git/tests/native_source.rs and tests/client.rs; cargo test -p tig-git --all-features --test native_source --test client --locked | passed | Two new ID-to-byte/explicit-CLI cases plus real LFS handoff while a cooperative verified reader holds the same object; metadata-only lookup remains independent; no native renderer or global source/job qualification |
| EVID-035 | crates/tig-git/src/cache.rs; cargo test -p tig-git --locked | passed | Forced second-handle probe/sample supersedes the first; admission resamples and succeeds on attempt two, without global locking or spending rejected observations; five actual LFS client regressions also pass |
| EVID-033 | crates/tig-assets/tests/graph.rs; cargo test -p tig-assets --test graph --locked | passed | 34 native graph cases: exact/snapshot-selected references, unchanged parents after a cel edit, historical view/frozen pin behavior, missing/ambiguous/checkpoint refusals, typed edges/cycles, caller-admitted coverage, shared-use primary delta, explicit payload selectors, binding/policy digests and a 1,011-node/1,000-primary synthetic hierarchy; no source/job completeness, private producer, native 500k scale or eviction authority |
| EVID-021 | historical consumer-readiness note at its pinned tree (generic successor: docs/CONSUMER-ADOPTION.md), docs/SOURCE-BUDGET.md | inspected | Read-only consumer/donor contract audit and source-only budget control; 500,000 is a configured inventory ceiling, not executed scale proof or cutover acceptance |

The 2026-10-26 local Windows Rust 1.98.0 rerun passed EVID-001/002/007/008/009/010/012/013/014/015/016/017/018/019/020
(392 local Windows all-feature test entries, with two additional Linux-specific entries and current-head CI pending). [WP-001 scope](../../specs/IDENTITY-AND-TREES.md) records sparse-work
measurements and missing proof. Growing raw logs, inventories and provider receipts
belong outside ordinary Git under admitted storage; public fixtures remain
synthetic. No provider-durable/power-loss or private consumer acceptance run exists.
Real Git LFS download integration is limited to EVID-023/025/030 synthetic temporary repos. The local store has scoped process-restart tests only.

## Stage review

EVID-038: crates/tig-git/tests/native_source.rs,
consumer_status_is_snapshot_pinned_and_never_default_debug_output. Passed locally:
old/new committed views preserve draft/selected project values for identical
payload bytes, change exact record identity, and omit status values from Debug.
The API does not interpret those values as authority; CLI output is unchanged.

Producer-applied six-role lenses: pass_with_risk. E-01: planned scenarios or an
Art donor test count could inflate Tig evidence. Resolved by explicit designed/
inspected/passed scopes and no invented future evidence rows. Requirement closure
still needs actual package-specific integration and validation records.

EVID-039: map-reference collection extension, entry 85e4938. Six new public pure
graph cases and two real-Git source cases pass: deterministic escaped map keys,
explicit shape/binding interpretation, shared resolution, bounded refusal,
output/primary separation, 1,000 non-media attachments, one-cel-only Git edits
and explicitly retained frozen revisions. Forty graph and eighteen native-source
cases pass within the 110-entry Windows asset/Git component rerun. No private
fixture or cache/removal authority is imported; full current-head CI is pending.

EVID-041: CI follow-up entry 9bb91a5, scoped cooperative guard destruction.
The earlier push workflow failed immediate Linux writer acquisition after Drop
(WouldBlock, run 40443726783); the same-head PR workflow passed. The new guard
explicitly releases only its cooperative lock. Windows local store library and
existing-byte suite: 132 + 11 entries pass; workspace Clippy passes. A Unix-only
duplicate-descriptor regression directly exercises the inherited open-description
lifetime and requires current-head Linux CI. The portable second-reader case
proves one Drop does not release another independently opened reader. This is
neither a global lock nor media-removal authority; Windows sharing remains intact.

EVID-042: entry 5a35f5a, six new graph cases and one real-Git/native archive/job-
journal integration. 123 Windows asset/Git entries pass (46 graph, 25 native
source); workspace all-target/all-feature warnings-denied Clippy passes. The
query derives exact primary, control and instance/definition/schema objects,
refuses missing/unsupported controls, output-root payloads, query gaps and size
conflicts, and keeps control-only changes separate from primary diffs. A real
SQLite metadata/source-journal reopen with Git/working metadata moved aside
preserves an old render-job root after main advances/retires its own predecessor.
No payload bytes are present in that fixture: metadata replay/obligations do not
prove custody or complete source/job enrollment. Full current-head CI required.

EVID-043: entry e7d2bde, frozen native production context adapter plus seven new
native-source cases (32 native-source entries), extended admission-byte checks
and the revised actual source/journal fixture. 130 Windows asset/Git entries pass;
workspace all-target/all-feature warnings-denied Clippy passes. Exact metadata
finish/reopen/readback precedes owner journal adoption; an old job replays after
main advances/retires its predecessor and Git/working metadata are moved aside.
Current/frozen revisions, idempotent/sparse metadata sharing, descriptor identity,
interrupted writes and missing/corrupt/substituted policy/source/descriptor/roots/
roles/headers/bounds are checked. No payload bytes are stored or verified here.
This is not complete job enrollment, byte custody or producer execution proof.

EVID-044: entry e504cf3, optional declared consumed-input recipes. Fifteen new
pure cases plus two temporary Git/archive integrations execute; 147 Windows
asset/Git test entries pass (61 graph, 34 native-source), with workspace all-
feature/all-target warnings-denied Clippy. Annotations/status/unconsumed changes
preserve recipes while consumed properties/defaults/payloads/slots/producer/wiring
changes invalidate them. Same-ID frozen root reordering, recursive wiring, gaps,
provider substitution, bounds and a 1,001-node report DAG are checked. Four Git
views replay exact recipe/provenance from archives after Git/working metadata
are unavailable. No payload IO, producer execution, output correspondence, full
model/variant parity, custody or runtime adoption follows; full CI is required.

EVID-044 parameter amendment, entry c9348e4: Operation now requires an opaque
parameter Object and Plan retains its immutable operation identity. Two pure
cases cover eight language/profile/encoding combinations and unsafe/conflicting
pins before source IO. A second native integration freezes both language
parameter descriptors into an actual SQLite metadata store, finishes/reopens it,
moves Git/working metadata aside and derives different recipes with the same
input graph/checkpoint. Final scoped suite147 Windows entries (61graph/34native),
Clippy/fmt/standalone check pass; current-head full CI must requalify this change.
This neither executes a producer nor proves parameter interpretation/custody.

EVID-045: entry77a7e14, bounded unsigned produced claims and native metadata replay.
Fourteen new pure cases plus two temporary Git/context/provider cases execute;
163 Windows asset/Git entries pass (75 graph,36 native-source), with workspace
all-target/all-feature warnings-denied Clippy and standalone Git check. Exact
current subjects, original-plan provenance, parameters, multiple candidates,
unbuilt/unproven history, stale inputs, strict parser/bounds/size conflicts and
self-cycle-free claim pointers are covered. Actual SQLite finish/reopen and
Git/working-metadata unavailability replay old context and distinguish annotation
reuse from consumed-input changes. No output payload is read or producer executed.
Full current-head CI remains required, with upstream PR26 qualification first.
EVID-046: entry84136b5, canonical Windows existing-cache root correction.
All 13 Windows existing-reader cases pass, including raw/canonical aliases across
REEL CAS, Git LFS and flat layouts, missing objects, held mutation/rename denial
and unrelated writer progress. Six real Git LFS client and 18 source-view cases
pass; standalone Git check, workspace all-target/all-feature warnings-denied
Clippy and fmt pass. Reject incomplete drive/extended prefixes and UNC roots.
No provider writes, live cache access, custody, deletion or adoption is proved.

EVID-047: entry9617787, frozen declared job resource roots. Six new Windows
native cases pass, within a 42-entry native-source suite; workspace all-feature/
all-target warnings-denied Clippy, fmt and standalone Git check pass. Actual Git,
SQLite finish/reopen, source metadata unavailable and journal restart preserve
independent main/job/peer-job obligations and typed original inputs plus claimed
outputs/evidence. A 1,001-primary source uses a five-entry job envelope and fewer
than 64 metadata puts. Missing/corrupt/extra/substituted metadata, cross-role size
conflicts, unsafe lengths, prefix collisions, bounds and interrupted new writes
refuse/recover. No payload IO, producer execution, complete coverage, custody,
capacity admission, retirement or native managed-job adoption is established.

EVID-047 concurrency amendment, entry561cc1a: PR29 Windows real concurrent LFS
clients exposed a generic Capacity error; matrix fail-fast cancelled its Linux
companion after tests passed, not a second feature failure. Adapter contention
was capped at eight synchronized retries and reported as full capacity. It now
uses 32 bounded op-specific capped waits (at most 80ms each), fresh observation
attempts, Busy on exhaustion and State for non-capacity ledger failures. Two new
pure/temporary-ledger cases plus the existing superseded-probe case pass, as do
all six actual LFS client cases and workspace warnings-denied Clippy. No capacity,
freshness, corruption or ownership check is relaxed; exact-head CI must requalify.

EVID-047 authority amendment, entryfd23a18: SourceJournal exposes its existing
stable genesis identity for consumers to bind job capabilities/fences to actual
journal authority. A new temporary-journal case passes: identity persists after
mutation/reopen and differs for separate same-project/epoch journals. Workspace
all-target/all-feature warnings-denied Clippy passes. This getter is not a live
revision fence, latest-witness proof or distributed authority service.

EVID-048: entry823e15c, exact-base small-file Git proposal commits. The public
Repository::propose API is independent of optional assets. Thirteen Windows
test entries (twelve primary fixtures plus one child helper) pass on Git
2.42.0.windows.2/Rust1.98.0: exact source/bounds/identity refusals, unchanged dirty
index/worktree/HEAD, same-input and concurrent retries, foreign/symbolic ref
refusal, SHA-256 Git, linked worktrees, unrelated symlink/gitlink/executable modes,
multi-record commits, normal disjoint cherry-pick and same-file conflict. A
one-record proposal creates exactly six small Git objects and retry adds none.
Inherited Git routing cannot redirect writes; an actually rejecting reference-
transaction hook is bypassed only for proposal commands. No media is read or
copied, no native semantic claim is made and no source/claim lifecycle is adopted.
Full workspace/final-head qualification is recorded in the package review.
Final local all-feature workspace suite passes479 entries; all-target/all-feature
warnings-denied Clippy, standalone Git/LFS check and fmt pass. The final metadata
suite including500,000 bindings passes210.06s. Hosted final-head Windows/Linux
and private consumer acceptance remain separate requirements.

EVID-049: entry759af7a, verified-cache Git proposal CLI/shared preflight. Five new
actual CLI fixtures pass across flat/REEL CAS/LFS layouts and ordered existing
roots: zero-write check, explicit repeatable create, same-byte deduplication under
one reader/request, normal Git application and corruption/missing/stale/duplicate/
oversize/strict-profile refusals. The13 parent library entries still pass after
the preflight/create split. Final Windows all-feature workspace484 entries pass;
the500,000-binding metadata suite passes221.58s. Standalone no-feature CLI/library
entries, workspace warnings-denied Clippy, Git/LFS check and fmt pass. This is
generic small-record handoff, not native semantic validation, approval, source/
claim adoption, media custody or private consumer project readiness.

EVID-050: entry7adb6e5, bounded pure deferred-build assessment. Seven initial
focused cases pass after correcting frozen-versus-logical fixture references.
The workspace all-feature run passes491 entries, including the500,000-binding
metadata fixture (suite309.54s). Post-review Observation refinement adds an eighth
negative case for unavailable current/claim evidence; final changed-package and
native-source replay qualification is recorded separately in the package review.
No media IO, renderer, queue execution, approval, custody or adoption is inferred.

EVID-051: entry36286fb, native current-output/retained-claim assessment. Ten
focused real-Git/synthetic-store cases pass, including current/original metadata
provider reopen with Git, record and definition directories unavailable. Exact
checkpoint/definition and malformed/bounded pins refuse before evidence IO;
legacy/planned outputs need no claim-store read; missing/corrupt/failed reads,
foreign subjects and unadmitted original semantics block. Named streams and
literal metadata keys work; an admitted old producer can be stale under a new
desired producer. Read-only providers forbid writes; diagnostics omit raw private
metadata/provider errors. Full workspace qualification follows in the package
review; no execution, payload integrity, custody or private cutover is inferred.
Final EVID-051 local qualification: all502 workspace all-feature entries pass,
including52 native-source cases and the500,000-binding persistence suite
(171.74s). Workspace all-target/all-feature warnings-denied Clippy, standalone
Git/LFS check, fmt and repository/source-budget validation pass. Exact-head hosted
Windows/Linux and parent33 qualification remain mandatory; all full acceptance
and private adoption requirements remain open.

EVID-052: entry1de926c, explicit authored-schema engine/CLI/native resolver. Initial
six pure cases and three native cases pass: five explicit dialects, nested/local
references and composition, formats, exact pins/default immutability/redaction,
regex failure under negation,1,001 native records sharing two exact definitions,
invalid envelope-valid authored values and retained history, and metadata archive
reopen with Git unavailable. Full qualification adds actual CLI/no-feature and
live external-resource refusal cases; results are recorded in the package review.
No complete native semantic/producer/custody or adoption acceptance is inferred.
Final EVID-052 local qualification:513 all-feature workspace entries and99 default
asset entries pass. After an equivalent Clippy dialect-guard cleanup,106 asset
and55 native-source entries pass; workspace warnings-denied Clippy, standalone
Git/LFS build/dependency isolation, format and structure pass. The final explicit
external-ref fixture uses an existing file and a live synthetic HTTP listener;
validation refuses both without connecting. Exact-head hosted qualification and
full native semantic/private acceptance remain separate open obligations.

EVID-053: entryd1f8712, explicit local record contracts. Five focused pure cases
and three native cases pass: mixed attempts permitted under declared roles/groups/
cardinality/order, local selected/authored conditions without status mutation,
strict declaration/dialect/pins/private errors/offline policy, legacy refusal,
authored-only distinction, invalid retained stream/record history and actual
metadata reopen with Git unavailable. Full/CLI qualification follows in the
package review. No native reference/producer/approval/custody acceptance follows.
Final EVID-053 local qualification:522 full workspace all-feature and99 default
asset entries pass; workspace warnings-denied Clippy, standalone Git/LFS check
and format pass. CLI proves media/nonmedia contract checks, legacy refusal,
privacy and no mutation. Source did not change after these runs. Exact-head
hosted qualification, complete native semantics and private adoption stay open.

EVID-054: entry2f0c2dc, native sparse variant resolver. Initial15 pure cases
exercise14 adapted Art behaviors plus project-context extension; two real native
Git/recipe/archive cases pass. English primary closure/recipe stay unchanged after
Spanish bytes change while record/checkpoint/binding provenance changes. Index
entries refer to existing stream slots, not duplicate byte pins. Further context
provenance/control admission and final qualification are recorded in the review;
no private producer, freshness, approval, payload or custody proof is inferred.
Final EVID-054 scope:539 full workspace entries pass before the final retained
context-document refinement; default assets114 pass. Final127 asset/60 native
entries, workspace warnings-denied Clippy, standalone check and fmt pass after the
refinement. Source/docs now use a reviewed3MiB aggregate ceiling with unchanged
128KiB/file and runtime exclusions. Exact-head hosted qualification remains open.

EVID-055: entry1095b6f, retained native variant interpretation and optional job
version2 linkage. Nine focused native cases pass, including exact offline
Git/store/job replay and explicit journal adoption/reopen, sparse per-record
coverage, strict primary-language equality, frozen prior record pins, resource
refusals, read-only restore and foreign/corrupt/missing evidence. Version1 calls
remain unchanged. Full qualification and role findings are in the package review;
no producer truth, live cache protection, complete claims or custody is inferred.

Final EVID-055 local qualification:548 workspace all-feature entries pass (69
native-source), plus warnings-denied all-target Clippy, standalone Git/LFS check,
format and structural/role checks. The full run terminates zero; source remains
unchanged afterward. Exact-head Windows/Linux qualification is required for merge.

EVID-056: entry81965c7, opt-in consumed payload policies and explicit native claim
replay. Initial five pure cases and one actual Git/provider/context/job case pass:
raw control-only edits preserve a projected recipe while captured controls remain
required; consumed primary/parameter/slot/order/role changes do not disappear.
Same-byte uncaptured streams refuse. Exact original claim replay survives Git
unavailability. Final extensions and full qualification are in the package review;
no native producer execution, live adoption or custody is inferred.

Final EVID-056 full regression:555 all-feature workspace entries pass, including
70 native source entries, six new pure cases and explicit legacy/projected mode
refusal in both directions. Original controls remain in job requirements;
output-as-primary cannot be erased by an empty consumption policy. Final lint,
structure and exact-head hosted qualification are recorded separately.

EVID-057 metadata budget inspection: validated opening Limits are exposed by value
on MetadataReader and MetadataStore. All 16 metadata integration tests pass,
including unchanged header/database bytes, file count, checkpoint and metrics
under readonly inspection, detached-copy mutation and a subsequent bounded write.
Locked all-target/all-feature workspace Clippy passes. This is inspection evidence,
not capacity admission, independent custody or native workflow acceptance.

EVID-058 source observation: SQLite read-only journal replay returns a named
source and checkpoint from one snapshot. The 23-entry source suite passes;
an additional explicit uncommitted-writer isolation case passes separately.
Three portable CLI cases verify bounded private reporting, null absent revision,
unchanged authority, no overwrite and foreign/missing refusals. Locked workspace
all-target/all-feature Clippy passes. This is point-in-time observation, not
execution/liveness, recovery authorization, global absence, custody or eviction.
See docs/reviews/2026-10-28-source-observation.md for entry/scope and open gates.

EVID-059 exact acknowledgment recovery: two API cases and four CLI cases pass,
including historical receipt recovery after adoption, unchanged authority,
missing/changed request refusal, missing/foreign authority, rollback minimum,
private output and no-overwrite publication. Locked all-target/all-feature
workspace Clippy passes. This reuses read-only replay; it never executes an
unknown command or reruns a producer. Complete job recovery, custody and consumer
adoption remain open. See docs/reviews/2026-10-28-source-acknowledgment.md.

EVID-060 source transition preview/CLI: two new API cases pass (1.04 seconds)
and six real CLI cases pass (3.35 seconds). Cases distinguish read-only proposals
from historical receipts, preserve another branch's old root, reject stale/
altered plans, allow unrelated progress, refuse overwrites and inside-authority
outputs, and retain the exact acknowledgment request across busy refusal and
post-commit receipt-publication failure. Rollback uses the observed plan floor,
not a weaker original witness. Four existing inspector/acknowledgment CLI cases
also pass; locked all-target/all-feature Clippy passes. Full source/workspace and
hosted qualification remain required. This is metadata handoff, not complete
source/job coverage, payload custody, Git hook installation or removal authority.

EVID-061 graph inspection: four real-Git CLI cases pass, including independent
source-text replacement without translation/parent edits, old snapshot replay,
explicit current-payload selection excluding history, missing/null/output gaps,
incompatible-policy refusal with complete graphs, strict profile decoding,
bounded private output, no overwrite/temp leaks and stable admission encoding.
All-target/all-feature Clippy and standalone no-feature Git/LFS checks pass.
Reports explicitly leave schema execution, semantics, payload verification,
global claims and eviction authority unproven. Full qualification and producer
review are recorded in docs/reviews/2026-10-28-graph-inspector.md.

EVID-062: explicit graph byte-verification adapter for WP-004/005/007/011/015.
Five new real-Git CLI cases (nine total graph CLI cases) cover primary/control
scope, exact graph-profile admission, deduplicated current objects excluding
history, missing/corrupt outcomes, no corruption fallback, whole-plan/report
limits, no overwrite, released guards and output-only/non-Production refusal.
The reader/provider implementation and byte identity remain unchanged. Reports
keep schema/semantics, independent custody, global claims and removal authority
unproven; each object observation ends before report publication. Component
results and producer review are in docs/reviews/2026-10-28-graph-byte-verification.md;
full local/hosted qualification and consumer adoption remain separate gates.

EVID-063: explicit native job evidence entry budget for WP-004/008/010/011.
Seven focused native job cases pass, including a new real-Git 1,000-evidence
boundary/sparse-history case. Total envelope entry overflow refuses before any
NodeStore IO on freeze/restore; runtime count and prefix-conflict checks remain.
Evidence stays distinct from primary/claimed-output roles and no global coverage,
availability or custody is granted. Scope, producer review and remaining hosted/
consumer gates are in docs/reviews/2026-10-28-job-evidence-budget.md.

The private EVID-062 rehearsal verifies 748 exact declared objects / 1,507,903,378
bytes using a frozen candidate executable and existing Windows-sharing readers.
All selected objects match; report guards/independent-custody/eviction flags remain
false. This is a read-only batch observation, not a simultaneous or durable
custody receipt, decoded comparison or runtime installation.

EVID-064: native job pre-execution composition/evidence references for
WP-004/008/009/010/011. Six new dependency tests, ten variant tests and all78
native-source cases pass: v1/v2 root identity, distinct roles/shared byte aliases,
exact map/role admission, whole-entry/size/prefix zero-IO refusal, 1,000 refs,
interruption/retry, source-offline metadata reopen and independently owned journal
restart/withdraw/retire. The initial withdrawal assertion was corrected to preserve
the engine's safer predecessor behavior. Scope, source, role review and remaining
hosted/consumer gates are in
[the review](../reviews/2026-10-28-job-composition-dependencies.md).


Historical qualification from capacity branch 712639b; integration qualification is recorded separately.



EVID-KRY-01: entry 84136b516a3f784c51398d819af7b44826236be1, bounded closed
capacity reservation retirement. Three new pure cases and seven ledger cases
pass: 21 capacity / 26 ledger entries. Actual temporary SQLite tests fill the
1,024-record window, retire a bounded subset, admit new work and reopen while
preserving original event bytes/hash prefixes, identity, receipts and accounting.
Two actual child exits and three injected commit boundaries exercise retirement
recovery; live/foreign/colliding references refuse. Publication, cleanup,
retention and handoff disposition queries survive retirement. Debit history stays
bounded and a new generation never revives an old token. Full default workspace
and all-feature suites pass (465 listed Windows all-feature entries); fmt,
all-target all-feature warnings-denied Clippy and standalone Git check pass.
Limits are unchanged. Native journal compaction, application deployment, payload
custody and full-system acceptance are not claimed; current-head CI is pending.

EVID-083: checked metadata copy, input0ceade2a16de1c1ed6cc27e48675e7945d7b61df.
Eight API and four separate-process CLI tests pass on Windows: 2,300-node paged
copy/readback, old-root reopen, target growth/source preservation, drift without
long transactions, partial failure, corruption, path/plan/budget refusal and
source-offline verification. Initial CLI borrow compile failure corrected.
Locked all-feature workspace tests, strict Clippy, fmt, structure and standalone
Git check passed locally. Final CLI cases pass again after temp-owner fix.
Hosted qualification, physical admission, private adoption and custody stay open.
Synthetic providers only; no real provider copy. See METADATA-PROVIDER-COPY.md.

EVID-084: first storage orchestration slice, inputbb51c5a13e1277c1b76e1bcdd2d865800595e733.
Fifteen focused library entries (including a child helper) and five separate-process
CLI fixtures pass on Windows/Rust 1.98.0 using explicitly configured external
build/temp roots. Two abrupt child exits exercise publication/retention recovery.
Three-object partial batches preserve source, recover exact destination retention,
and inspect without new journal commands. Changed plan context, root/ledger/basis,
operation conflicts, insufficient peak/reservation, missing/corrupt bytes and
explicit private-cleanup cancellation refuse safely. A held source blocks its
exclusive scope while another object remains accessible. CLI plan/discovery are
provider-free, errors omit root paths and resume is exact. Legacy None context
encoding stays unchanged; existing cleanup retains the optional new context.
An initial fixture failed NeedsProbe after reopening; corrected by a fresh explicit
probe, not by weakening admission. All-feature workspace regression, strict
all-target/all-feature Clippy, fmt, standalone Git check and locked default
workspace regression pass. No full WP/VAL closure,
complete protection registry, UI execution, eviction, large-object qualification,
remote upload, automatic lifecycle or final-deletion/metadata-retirement claim.
See [controlled scope](../../specs/STORAGE-ORCHESTRATION.md).

EVID-084 implementation source: 390ac4a. Verification/validation/trace and
producer review are recorded separately from that implementation commit.

EVID-085: protection demand snapshot, inputd18afba336f48f8b982ffe83c5b609d1f903c9f7.
Seventeen focused Windows entries pass: actual temporary source journals enumerate
all registered sources, all current/pending/predecessor roots contribute, and
shared placement demand retains exact owner/kind/root provenance. Missing,
unexpected, foreign-owner, opaque, changed and rolled-back scope remains unknown;
corrupt/missing binding roots contribute no partial demand. Aggregate time/read/byte/
root/association budgets refuse excess; a 33,000-binding fixture resolves demand
but the existing pure View budget refuses without truncation. Bounded summaries
sample gaps while retaining their total count. Explicit owner retirement leaves
an empty registered opaque source without inventing a payload obligation; other
jobs remain protected. Snapshot-derived Basis drives actual copy/retention, and
changed claims reject operation reuse. All 35 existing source-journal fixtures
pass. Final default/all-feature workspace regressions and portable L0 pass. Strict all-feature/
all-target Clippy and standalone Git check pass. No effective admission fence,
complete claim registry, byte custody, eviction or full-system acceptance follows.

EVID-085 implementation source: 55a21e4. Verification, intended-use validation,
trace and producer review closeout are separate from the implementation commit.

EVID-086: mandatory scoped source admission, implementation
0b75664e63ca3807096fb9e1dfdafd7faac275ce; input355aad7, reviewed roadmap85ac433.
Twenty admission entries (including one subprocess helper) pass within the locked
default Windows workspace suite. Three actual child processes exercise held
exclusive subject locks, parent termination, and abrupt exit before/after SQL
commit. Restart distinguishes uncommitted intent from a committed historical
receipt; exact replay requires no old tree nodes. Direct generic mutation and
preview refuse guarded journals; legacy journals keep their existing encoding.
Changed owner/class/operation, stale compilation, foreign fences, missing closure,
resource limits and replaced markers/anchors refuse. Unrelated partitions and
registry-only enrollment progress under a held subject fence. Active/history
precedence shares unchanged bytes while explicit retained demand stays additive.
The read-only projection rejects conflicting guarded source classification.
Final default/all-feature workspace suites, strict all-feature/all-target Clippy,
fmt, standalone no-feature Git and portable L0 pass. Logs and generated
fixtures remain outside source. No provider removal, namespace/store adoption,
epoch handoff, independent custody, hostile/full-restore fencing or power-loss
guarantee is established. Full requirements and all 110 workflows remain open.
Hosted Windows/Linux/secrets qualification of PR #17 remains separate; the
previous snapshot PR #16 has green hosted checks and is ready for review.

EVID-086 hosted follow-up: PR #17 head83d3ec9 passes Windows (22m39s), Linux
(2m42s) and secrets (6s), and is ready for review. This qualifies the admitted
local component on those hosts; it does not close full workflow/adoption gates.

EVID-087: store-bound protection assessment, implementation
be6f946cc48c0ca72da641a72b55a86190985bba; input83d3ec9. Fifteen focused Windows
entries (including one subprocess helper) pass. Explicit fresh design-6 stores
pin exact namespace/place rosters and enrollment generation. Both namespaces and
every mapped alias contribute; omitted/duplicate/foreign rosters, wrong generation,
unknown place, conflicting size, missing closure/anchor and replaced markers
refuse. One aggregate resource/deadline budget spans namespaces. Existing readers
and persistent retention block assessment; failure releases acquired guards, while
failed revalidation preserves an already returned assessment's guards. A real
child process holds both namespace fences and the provider fence, blocks claim
admission/readers, allows unrelated enrollment, then actual termination releases
them and fresh handles reassess. Existing accounted copy/retention works with the
new provider contract; stale actual contract refuses before ledger commands.
Legacy marker encoding remains unchanged. Final default/all-feature workspace
suites, strict all-feature/all-target Clippy, standalone no-feature Git, fmt and
L0 pass. No live stores, credentials or consumer adoption.
The roster/boundary is explicit trusted enrollment, not external-client discovery
or authentication. Epoch handoff, complete restore authority, custody and
authorized-absence/incarnation removal recovery remain open. No deletion API.
PR #18 remains draft for current-head hosted qualification; implementationbe6f946
has green hosted Linux/secrets and a running Windows job at this checkpoint.

EVID-087 hosted follow-up: PR #18 headfcb7009 passes Windows (40m43s), Linux
(3m47s) and secrets (6s), and is ready for review. Component scope is unchanged.

EVID-088: managed replica retirement, implementation
fe88c7cb9277d5e2d1c93935cd98932a0d153984; inputfcb7009. Twenty-four focused Windows
entries (one helper) pass, including eight actual child exits at binding, intent,
unlink, completion, settlement, reacquisition birth and both cancellation phases.
Actual temporary-store unlink preserves independently retained bytes under declared
domain enrollment; explicit new-operation restore survives old deletion/publication
retries. A two-namespace main/job fixture publishes a replacement and backup,
preserves two unchanged inputs, holds old working bytes while the job needs them,
then evicts and restores the previous revision. Existing retention handoff composes
with managed eviction. Empty/multibuffer, active reader/claim/retention, stale context,
wrong authority/authorization, alias/domain/custody, missing evidence and known
provider-prefix rollback cases refuse unsafe actions. Concurrent unrelated actual
publication and source admission progress while target fences remain held.
Cancellation preserves original publication identity; partial metadata and history
exhaustion refuse silent reset. Final locked default/all-feature workspace suites,
strict all-feature/all-target Clippy, fmt, standalone no-feature Git and portable
L0 pass. No live assets/providers were used. Native independence,
full-store restore authority, power loss, large media, dynamic epoch adoption,
CLI/agent retirement, Git automation, last-copy deletion and metadata expiry remain
unqualified. All 110 full workflows and full requirements remain open.
PR #19 remains draft for hosted current-head Windows/Linux/secrets qualification.

EVID-089: explicit retirement CLI/agent interface, implementation
19346c45930b575c1bbda5a7ee618fb4101d9d98; input3e300ca. Eight actual CLI process
cases pass in 1.65s over synthetic managed stores, retained backups, persistent
metadata and capacity/source journals. They cover plan/status/apply/resume,
physical unlink, exact-byte new-operation reacquisition and old eviction replay
preserving the new copy. Private owner/context/grants/domains/roles and byte/peak
limits independently admit execution; disabled defaults, stale/forged plans,
unmapped roles, missing keeper, active reader/claim, strict/oversized documents
and missing roots refuse. Status appends no source capacity commands, needs no
keeper/tree/namespace availability, and separates historical retirement from
present integrity. Corrupt/missing acknowledged restored bytes remain unproven/
recovery_required without repair or authorized-absence fabrication.

The 24 existing retirement kernel entries pass with new status assertions at
real-child interruption boundaries, cancellation and known settled-provider-prefix
rollback. A separate explicit cross-artifact L2 test passes in 0.55s: a real child
exits after durable intent, then actual CLI status/cancel/retry processes preserve
bytes and original publication identity with keeper/tree/namespace roots offline.
That test is intentionally ignored in a library-only suite and explicitly run
with a freshly built TIG_RETIREMENT_CLI artifact; both CI platforms require it.
Do not count the ordinary ignored entry as executed evidence.

Final locked default/all-feature workspace suites, strict all-feature/all-target
Clippy, fmt, standalone no-feature Git and portable L0 pass on Windows using
external build/temp roots. Verification, synthetic intended-use rehearsal, trace
and producer review are separate records. Hosted current-head qualification
remains pending publication. No real cache or consumer was attached, initialized,
transferred or removed. This closes the explicit bounded local interface, not
dynamic epoch/restore authority, large-file/native qualification, automatic Git
reconciliation, remote/LFS upload, bulk final deletion or metadata expiry. All
full requirements and 110 designed workflows retain their separate open gates.
