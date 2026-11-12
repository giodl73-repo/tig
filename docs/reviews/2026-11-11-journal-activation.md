# Journal activation design review

EVID-129 design only; inputc833add85f9a9c79d3387a78e469bc13a2570403.
[JOURNAL-ACTIVATION](../../specs/JOURNAL-ACTIVATION.md) names controls, full
required outcome, allowed fixture-only entry and L0/L1/L2 gates before code.
Method: producer inspected actual legacy guard/apply_once/catch_up and staged
rollback, current caps, original-history export/lookup and all six checklists.
These are producer lenses, not independent people or human acceptance.

| Lens / target | Finding / severity | Required disposition / evidence |
| --- | --- | --- |
| Identity / ordinary and activation minima | Critical: rotation may keep the same ordinary checkpoint; its minimum cannot detect pre-fence restore | Separate independently retained activation generation/digest minimum; preserve original authority/references, refuse missing/stale minima |
| Placement / restored accounting | Critical: an empty or summary-only successor loses active/in-flight obligations | Full working model/highs and exact typed original dependencies; fresh handle-local probe only, real nonempty multi-rotation fixtures |
| Custody / lifecycle | High: sealing alone could be advertised as permission to reclaim old metadata or bytes | No reclaim on prepare/activate, no seal-only public entry point; expiry and current provider/source/witness authority stay open |
| Concurrency / delayed legacy writer | Critical: old process can pass file guard then begin SQL after rotation | Transactional old events/meta write guards; actual pinned legacy Rust process paused at that exact boundary, older-held transaction and unrelated-domain progress |
| Provider/privacy / finite budgets | High: full journals may have no room for an unknown seal event; naive index editing can consume unbounded staging | Separate fixed bounded fence control row, no journal cap waiver; admitted physical metadata growth and bounded global index/tail proof |
| Git/adoption / proof identity | Critical: current-writer or Python model could be mislabeled legacy qualification | Pin original source/binary and any timing-only patch hashes; unmodified old-binary format refusal plus legacy acceptance, separate full activation/recovery gates |

Disposition: pass_design_with_risk. SQL/helper and active-control/index framing
must be frozen before their implementation. Initial private fence proof and full
activation have separate closure; neither is implemented by this design record.
All runtime/verification/validation/trace/full lifetime/expiry gates remain OPEN.
Parent PR59 full72751/40677 and exact43107903309 still live at entry; do not
restart or overlap those targets. Full PRODUCT_PLAN remains active.

## Supporting SQLite behavior evidence

Designd650b51 and SQL/probe amendment48f5f40 precede tools/probe_activation_sql.py.
This stdlib probe requires a new explicit synthetic root. Its hidden child accepts
only the exact fixed public fixture config; existing journal opens use SQLite
mode=rw, and normal probe entry refuses an existing destination. Separate checks
prove a missing journal stays missing and an existing root's sentinel is unchanged.

The qualified probe uses SQLite3.44.0 through Python. A real Python SQL process
passes its modeled file guard and pauses before BEGIN. The parent commits the
single bounded intent and nine fixed SQL guards; released child INSERT refuses
and the original acknowledged event/head remain unchanged. Drop/reopen retains
all guards; rollback removes the entire DDL/intent transaction and permits the
old write. A separately identified capacity domain progresses normally.

The pre-fence backup has the exact same ordinary checkpoint but no activation
row/guards. An independently retained generation/digest minimum makes the probe's
explicit status model hold that restore. An unchecked copy that ignores the
minimum can append and fork; no claim that SQL guards alone solve restore is made.
A partial guard-set copy also holds despite retaining the intent row. The probe
never unseals or changes the original sealed journal. All data are fixed public
synthetic metadata; there is no payload transfer or removal.

Initial attempt exited1 because the producer expected DELETE on an empty events
table to fire a row trigger. The corrected fixture has an actual acknowledged
event and tests preservation of that prefix; the SQL guards were not relaxed.
A later refinement gives the unrelated domain a distinct identity and compares
actual intent/guard presence against its retained minimum. The final scope flags
are legacy_rust_qualified=false and activation_implemented=false. This is support
for selecting the fence, not pinned Rust writer or complete activation proof.
The final artifact is activation-sql-probe-c61a307ba98941148c122bc453d50cc4;
use the external activation-sql-probe-final-root.txt for the authoritative root.
Implementation of the private pinned legacy-Rust qualification and complete
activation/recovery/index/minimum protocol remains required. Producer review is
still pass_design_with_risk; no independent/human acceptance or lifetime closure.

## Parent qualification correction

Parent PR59 sourcec833add exact43107903309 now passes Windows/Linux/secrets.
Initial local default72751/all40677 ended101 in native-source Budget refusals:
64-item proposal in both and17-root age in all. These failures are retained.
Focus4352 passes those cases sequentially with unchanged production budgets
(total fixture times36.43s/99.38s). Complete profiles now run sequentially under
session37691, RUST_TEST_THREADS=1; explicit in-test concurrency stays enabled.
Default then all use the same two reserved external targets. Do not overlap or
restart; follow ledger-history-serial phase.exit/logs and poll the exact handle.
PR59 remains draft pending this full local qualification. This is no blocker to
the supporting probe, and no reason to raise production budgets or ignore tests.

## Pinned timing-instrumented Rust writer

Harness contract32d9c36 precedes tools/qualify_activation_legacy.py and its external
Rust fixture. The harness requires a clean detached linked checkout at exact
c833add85f9a9c79d3387a78e469bc13a2570403. Preparation refuses attached branches and
active source checkout paths before patching; a negative check verifies the
parent production branch bytes remain unchanged. Evidence/target roots are outside
source checkouts and are never reset, cleared or reused by the harness.

One matching site adds only a bounded20s timing barrier after original self.guard()
and before Request/SQL transaction. No legacy request, transaction, rollback or
admission algorithm is replaced. Original ledger SHA is
98a043d49d73d193a07a65cfb06c05f8b672ee8a0e385f30b8f3ef3ffda435ed;
patched SHA is1f8760d6bea94440bdd4da24dde2ff64a93858213922ac3ba036b0d405cbb991.
Exact patch, generated test, Cargo.lock and helper hashes are retained in prepared
metadata and checked before build. Initial preparation was superseded before any
execution to add the detached-branch safeguard; old artifacts are preserved.

The selected old Rust fixture creates a real legacy journal through original
public APIs: sample,1000-byte reservation,25-byte completion and200-byte in-flight
allocation. The writer opens that six-command prefix, passes its real original
file guard and pauses at the recorded barrier. The Python controller commits the
frozen SQL intent/guards, then releases the old Rust transaction. The old writer
returns the specific sealed-source SQL refusal, poisons its handle, acknowledges
no new event and preserves the exact original checkpoint. A fresh old reader
still sees975 reserved remaining/25 unobserved charges, refuses finish with
InFlight and requires a fresh probe for new reservation admission.

Run76934 TERMINAL0 used an explicit new external target, locked/offline Cargo and
one build/test job. It was admitted after the parent sequential native-source
suite completed157 cases/1 existing ignored qualification-only case; the earlier
proposal/age deadline cases had passed. Parent normal targets remained untouched.
The controller used Python SQLite3.44.0; the actual old Rust writer used bundled
SQLite3.54.1. Binary SHA is
bf6846e501c841631461c984cce7b256ee662af313b0a52837aee129e324dc1e.
Strict pinned-fixture Clippy28344 TERMINAL0; Rust fixture formatting and Python
syntax checks pass. No dependency or production Rust module is added here.

Prepared evidence activation-legacy-evidence-66a09d34b4ba4318971df47b28918a5c/run
contains actual build stdout/stderr, fixture preparation, writer result, exact
hashes and report. This qualifies the named timing-instrumented pinned Rust writer
against the frozen guards; it is not an unmodified-binary proof. Report fields
unmodified_legacy_binary_qualified=false,
rust_activation_controller_implemented=false and activation_implemented=false
remain explicit. An unmodified old binary's real activated-format refusal, old
already-held transaction ordering, native Rust seal controller, full activation/
recovery/global indexes/minima and provider/source/witness continuation remain
required. No sustainable lifetime, expiry or human approval is claimed.

## Private Rust mutation-fence component

Contract822d777 (inputfb2ba3ba6156f679bd94c9d85ac5f388eed62c38) precedes the
cfg(test)-only ledger::activation component. There is no public seal/activation
operation and no production ledger path invokes it. Intent framing and fixed SQL
are unchanged from the frozen protocol. Immediate transaction rechecks actual
source checkpoint/encoded bytes, installs one immutable row/nine guards, validates
exact control schema/row/hash/head, and commits atomically. Matching retry is
read-only and returns the original receipt; another intent conflicts. Sealed or
uncertain handles are poisoned, preserving the old ordinary checkpoint/accounting.

Inspection is read-only and never creates/repairs missing control state. It reads
at most11 prefixed schema objects/two rows, checks name/type/SQL lengths before
copying (128/16/4096 characters), body<=64KiB and hash length64, and refuses
partial/extra schema or canonical/hash/head inconsistencies. No NodeStore/provider
IO spans SQL. Physical metadata admission and active-control publication remain
complete-controller obligations; a private fixture is not normal admission.

Rust2927: initial seven cases pass, including real process exits before/after
commit, injected rollback/lost reply, exact/conflicting retry, partial state,
source advance, older write transaction Busy and unrelated-domain progress.
Rust65307: eight default cases pass/one external-input test ignored, then that
exact ignored test explicitly passes with the hashed c833add legacy executable
and Rust sealer. No ignored case is counted as qualification without its run.
Rust37874: ten pass/one ignored, including independent intent literal/digest and
actual full16384-command source with active/in-flight work; explicit pinned legacy
case and strict lib/tests Clippy pass. The full source's ordinary head/encoded
bytes stay unchanged; no extra event slot or cap waiver is required.
Rust85873: length-bound refinement passes nine short cases/one ignored, explicit
pinned legacy case and strict Clippy. Full-cap case had passed before this bounded
read-only refinement. Helpers are included in numeric harness totals, not treated
as extra protocol scenarios. The separate external target is now free.

The composed Rust-sealer/pinned-writer fixture verifies executable/source hashes,
creates the source through the original Rust API, pauses at the exact after-guard
barrier, seals in Rust and releases the old writer. It proves the specific SQL
refusal, no new event, preserved exact six-event prefix/in-flight accounting and
fresh-probe requirement. This qualifies a private Rust mutation fence; it does
not qualify an unmodified old binary against an implemented active-control format.

Separate closure: private primitive implementation/focused verification/scoped
synthetic validation/trace pass. Identity/framing, held accounting, transactional
race/rollback and bounded local metadata findings have scoped evidence. Custody,
new active format, independent activation minimum, global indexes, bounded tail
reopen, full handover/recovery, provider/source/witness continuation, expiry and
independent/human acceptance remain OPEN. No public runtime activation is added.

Parent serial37691 was stopped at the proven tests/fences stdout-readiness
mismatch; this is a test harness failure, not success. Parent contract7a1ebe6 and
fix1c1f3ca move readiness to bounded stderr. Focus25433 passes all11 fence entries
under1/4 threads plus strict Clippy; production algorithms/caps/deadlines unchanged.
New parent source1c1f3cad448e5bf5578d734ea9cdd43a565c7740 is pushed on PR59;
full controlled pipeline88738 is LIVE (default then all), both normal targets
reserved. New exact43119762645 Windows/Linux are live, secrets pass. Old failed/
interrupted runs remain recorded. Do not overlap/restart; full PRODUCT_PLAN active.

Local parent integration brings1c1f3ca fixture readiness/spec/EVID-128 into this
owned activation branch. Private Rust component bytes remain unchanged; parent
fence fixture bytes match the separately qualified correction. Context retains
the active lane and controlled88738 process/target ownership. No PR merge occurs.


## Staged index-write prerequisite

Input3586d821487b4bfd0df8d86698e9a432a498dc97; contractd6dc4c6 precedes code.
[STAGED-TREE-EDIT](../../specs/STAGED-TREE-EDIT.md) freezes controls, allowed scope,
entry/exit and L0/L1/L2. Existing Tree::edit runs against a transient bounded
NodeStore overlay; only final reachable staged metadata nodes flush children first.
No new tree encoding, journal cap, activation/controller or authority is added.
Logical metrics include intermediate writes; published metrics count provider
put calls/bytes, not newly allocated storage, durability or reclaim. Caller still
owns physical admission, finish/readback and atomic root adoption.

Producer inspection of tree/staged.rs and named fixtures, owner Tig maintainers:

| Lens | Finding / disposition |
| --- | --- |
| Identity | Canonical roots must match unchanged edit and independent bulk build; preserve original design-1 vector. Eight pure entries pass, including full delta and the independent vector. |
| Placement | Index edits convey no placement/removal authority; absent data still unknown. No new placement path. |
| Custody | Distinguish logical staging, provider puts and physical finished metadata. Full-delta finish/reopen passes; no custody or reclaim claim. |
| Concurrency | Expected-root check is local to supplied Tree. Caller must fence/adopt the root; partial flush is immutable orphan metadata, never a returned success. |
| Provider/privacy | Whole-edit read/write/byte limits bound staging. Flush walks staged metadata only, skipping unchanged base subtrees and file-valued edges. No private/runtime data in Git. |
| Git/adoption | Public synthetic fixtures only; measurements/builds remain external. No merge, production migration or live-store activation. |

Initial qualification7999 ended101 at fixture compile: ambiguous iterator sum
with serde_json PartialEq. Explicit sum::<u64> fixes the fixture; no algorithm or
limit change. Corrected82881 is running eight named pure entries (including a
4096-base/16384-delta case), the physical full-delta adopter and strict core/store
all-target Clippy. No pass is claimed before actual terminal completion.
Full exact-source workspace/hosted qualification and complete continuation,
independent activation minimum/restore, old-writer and provider/source/witness/
expiry gates remain required. Producer review is not independent acceptance.


Continuation process audit: write_stdin82881 and88738 both report unknown IDs.
Windows process inspection finds neither exact test/target executables, Cargo nor
controlled runner; original phase exit files are absent. The first staged log
proves eight pure passes but physical run has no completion; parent default log
stops mid bulk paged followup and all phase never starts. These are interrupted/
unknown-exit pipelines, not inferred pass/fail. Original logs remain unchanged.
With the separate target authoritatively free, new89281 explicitly runs physical
v2, strict Clippy and core/metadata regression with separate logs/exit markers.
No process restart was based solely on timeout; no other process was stopped.


Final staged prerequisite:89281 TERMINAL0, reaped. Physical-v2.exit,
Clippy-v2.exit and regression.exit are all0. The physical fixture appends16384
entries to4096 old entries, finishes/reopens and verifies all20480 plus the exact
old root. Metadata rows4099->20492; logical stored bytes1262810->6315322.
The235279 logical writes/78349532 bytes become16394 provider puts/5052712 bytes;
one put deduplicates an existing node. No allocated-file-byte/reclaim claim.
Pure82881 eight entries pass in161.11s before its later interruption; physical
v2 passes in102.20s. Both share reads542003/logical read bytes193703803 under
explicit2M reads/1M writes/512MiB read/256MiB write limits, unchanged defaults.
The pure reported provider-get total40980 includes final inspection, not edit IO
alone. Strict core/store all-target Clippy and fmt pass. Full core regression
passes (only the already-qualified full staged delta skipped); original metadata
suite16 passes including actual500000 bindings/thirteen edits and child exits.
No production code changed during these qualifying runs.

Separate closure: implementation, focused verification, scoped synthetic validation
and trace pass for staged edits. Six producer lenses above are pass_with_risk for
this metadata-only API and retain caller finish/admission/root-adoption boundaries.
Full exact-source workspace/hosted qualification and complete successor/minimum/
restore/old-writer/provider/source/witness/expiry gates remain OPEN.

Parent full qualification restarted explicitly only after authoritative absence of
old runner/test/Cargo processes. Owned hidden pwsh PID44168 runs source1c1f3ca
from ledger-history-owned-b787ba2789be482c8bf5b596fdb6d43e/run.ps1 externally.
It preserves old logs, validates clean exact source before each profile, uses
jobs1/threads1 and sequential default/all with phase logs/exit markers, standalone,
strict/fmt and fresh14-step artifact. Both normal targets are RESERVED by that
runner. It remains live; no full pass claimed. Current branch's separate target
is free after89281. PR59 remains draft; no PR merge or deployment occurred.


Successor framing entry: inputd3c331ba22f3a2f0db8e6b483b508e7d1d33662e is the
qualified staged editor. specs/JOURNAL-SUCCESSOR.md freezes the next joined
controller format before code: noncyclic PreparedData/finished metadata minimum/
Preparation/intent/ActiveControl, stable original authority, global operation/
sequence/ancestor indexes, bounded tail/local unchanged caps, explicit independent
minimum and source SQL plus per-authority controller fences. Legacy intents keep
their original bound; subsequent global heads require distinct design-2 intents.
Metadata IO stays outside SQL; staged core/output/clock adoption must fail together.
No giant lifetime map, source reset, missing-tail initialization or public seal-only
shortcut. Six producer design lenses are pass_with_risk with every successor L1/L2
execution, old-library/new-format, restore/minimum and source/provider/witness/
expiry obligation OPEN. Tree u32 entry bounds are explicit finite metadata, not
an unlimited lifetime claim. No implementation evidence for this new protocol.


## Global receipt-index implementation entry

Input7ef73d328f5c2a28477d1f2a7993a8ed288f9978; frozen successor contract precedes
code. WP-003/004/006/007/008/013/015, REQ-001/003/004/005/006/007/009,
SPEC-002/003/004/005/006/008/009/013/014, IF-002/003/004/005/006/008/012/013,
PKG-001/002/003/008/009; RUST/PROVIDER/INTEGRATION L0/L1/L2 as frozen there.
Allowed scope is existing history/core/metadata ports and private global operation/
sequence indexes, bounded source/delta fixtures and controlled records. Entry uses
owned synthetic roots; exit for this component is exact old replies/dependencies,
append/prefix/absence/integrity/shared-budget evidence. Joined active-control/tail/
minimum/recovery/old-writer and complete continuation remain separately required.

Implementation is private cfg(test) ledger::history::index. Shared DependencyView
keeps original Snapshot typed checks/range/default caps intact. Index pairs exact
operation and sequence roots; selected records check both mappings and preceding
checkpoint. It imports a fully validated original segment and stages bounded
continued-schema deltas, checking original context/request/hash/types and earlier
references. Deltas use a bounded input view rather than a lifetime output map.
Original receipts/probes stay historical; old retired outcomes use existing pages.
Index::extend accepts controller-supplied acknowledged rows; it is not independent
source-authority admission and has no public runtime API or successor journal.

The whole-build BudgetPort counts provider IO and private proof-memory IO against
shared read/write/byte counters before calls. Snapshot::validate_all retains its
public signature/format/caps and private default MemoryNodes proof; a private
helper lets the index share the proof port's counters. Staged overlays feed each
next tree remaining logical limits and charge hidden overlay work; on failure the
operation stops. No per-tree reset, deadline/cap waiver, payload IO or new engine.

Initial57423 TERMINAL1: fixture used u64 directly as SQLite parameter. Existing
checked sql_number fixes that helper; no runtime algorithm change. Corrected65184
TERMINAL0/reaped: four short index entries pass, including real cross-snapshot
completion/retirement, physical metadata finish/reopen, all legacy/clock receipts,
point budgets, rebound roots and exact aggregate staging ceilings. Subsequent
source review replaces post-hoc proof-write accounting with the shared proof port;
14508 currently runs six short entries, adding exact proof quotas and corrupt/
wrong-ack providers. No later pass or maximum-delta result is claimed yet.

Producer findings, owner Tig maintainers: identity requires original/continued
schema separation and paired sequence/operation proofs; placement grants no new
use/removal authority; custody retains external source/minimum/admission gates;
concurrency requires exact source/controller CAS beyond this immutable component;
provider/privacy requires the shared proof/overlay budget and no payload access;
Git/adoption keeps real source fixtures synthetic and .tig/history expiry open.
These are producer lenses with qualification pending, not independent approval.

Parent exact43119762645 is now all three SUCCESS (Windows/Linux/secrets). Local
owned PID44168 remains live on exact1c1f3ca; full local/teaching qualification is
still pending and PR59 remains draft. Neither old interrupted run is called pass.


Global maximum failure: owned49184 is terminal/absent, maximum.exit101 and
complete.exit1. Real full legacy import passes; full index delta refuses Budget
at Index::extend (test line528) after517.86s. Original-history/strict phases did
not execute. Preserve successor-index-owned-7a1949ba902c42408cfad16a826e7793 logs
and source pins; no full pass or cap waiver. The target is free after process audit.
Tree leaf read-reduction amendment precedes optimization, with exact old tree
SHA256 and unchanged protocol limits. Combine find/change leaf traversal while
retaining the old BST/heap path checks, directory conflicts/no-ops and canonical
roots. Requalify malformed paths and exact independent roots; no base-node cache.


Read reduction candidate: shared MapRoute preserves the old find BST/heap checks
on mutation's single leaf traversal; unchanged children return their old links.
No new encoding/API/cache or larger limit.2570 TERMINAL0/reaped: existing tree17
and staged9 (full staged maximum explicitly skipped) pass, including rehashed
BST and heap refusals, two-read single-leaf update, zero-write missing deletion,
directory conflict and independent bulk roots. Global short-v3 seven pass, adding
independent Python canonical/hashlib continued-item vector17001 and strict legacy
schema rejection above16384. Runtime and journal caps remain unchanged.

Owned hidden9348 now runs source-pinned successor-index-leaf-owned-
f50df6133837474d8dcdcf346c02ec93/run.ps1: maximum, full core, physical metadata
staged maximum, original history eight, strict core/store all-target and fmt.
All Rust/TOML/Cargo.lock source pins are checked before phases; source must remain
unchanged while live. Separate activation-legacy target RESERVED. The maximum is
live; no maximum/full/strict pass is claimed. First failed maximum remains preserved.
No commit of this candidate before required qualification; producer findings and
joined controller/minimum/tail/recovery/old-writer/expiry gates remain OPEN.


Joined-controller producer review while immutable source qualification runs:
JC-01 critical, owner Tig maintainers, design disposition resolved / implementation
OPEN. Previous-generation digest alone cannot derive its operation-derived tail
filename after domain.json replacement. Successor contract now requires immutable
controls/DIGEST.json readback before current-control publication; recover the
previous tail through exactly one known predecessor control, never a history scan.
Missing predecessor/current record holds; seal-to-archive-to-publish exits and two
rotations remain required fixtures. No new code or passing runtime claim follows.

JC-02 critical, owner Tig maintainers, OPEN. An already-full source cannot be
assumed to have another ordinary reservation slot. Controller metadata admission
must precede preparation writes and exclude competing capacity users, including
source-fence/rollback growth and cumulative control/tail files. Numeric ceilings
and index RAM/IO bounds alone do not admit physical growth. Do not consume the
existing free-space floor or reset counters. Distinct admitted metadata resources/
pre-existing protected capacity and the shared-resource bootstrap case need actual
provider qualification before a public activation API. This is part of the full
plan, not delegated to an unverified caller assertion or quiescent-only shortcut.

First original-history parent fresh default-tests.exit is0 on exact1c1f3ca;
default Clippy/fmt also completed and owned44168 is in standalone/bins sequence
at the last observed checkpoint. All-feature/artifact outcomes remain pending.
This parent evidence does not qualify the new dirty index/core candidate.


Second maximum9348 TERMINAL/absent: maximum.exit101, complete.exit1,525.44s.
Leaf optimization passes operation-tree stage but sequence stage exhausts remaining
read budget. Before operation insertion:350923 reads/146832332 bytes; before
sequence insertion:1072847 reads/453644890 bytes, leaving83226022 bytes. Preserve
successor-index-leaf-owned-f50df6133837474d8dcdcf346c02ec93. Later core/physical/
original-history/strict phases did not run. No cap waiver or maximum pass.

Conditional-insert contract amendment precedes code and is independently hashed
externally in successor-index-insert-amendment.json with exact old core/index SHAs.
Use one checked traversal to prove file absence and insert, retaining conflict on
an identical existing object and all BST/heap checks. Eliminate the index's
separate old-name scan. Full retry authority still comes from original receipts;
immutable construction is not current source admission. Requalify whole maximum.


Conditional candidate48753 TERMINAL0/reaped: tree17 and staged12 (full max
explicitly skipped) plus index short8 pass. Tree insert refuses an identical
existing object, overlapping/mid-batch files and missing/corrupt routes before
flushing; independent roots and bounded single traversal pass. Added a correctly
rehashed new record reusing old invalidation/job operation IDs: both conflict and
original requests/checkpoints remain historical. No new retry receipt is minted.
Fault fixture was then separated so corrupt-read mode actually acknowledges writes
and reaches a bad base read, whereas wrong-ack mode corrupts acknowledgement.
16485 strict TERMINAL101 reported one collapsible-if in shared dependency checks;
collapse preserves short-circuit behavior, no suppression.8896 strict TERMINAL0,
41.07s, core/store all-target. Subsequent source-pinned61404 repeats final checks.

Current61404 runs successor-index-insert-owned-17d0c4e105c646cf900aae8707bcecdf,
all Rust/TOML/lock file sets/hashes pinned before phases. Short8 pass7.04s;
maximum PASS415.87s, source original full16384 plus synthetic continued index
full16384 ->32768. This is immutable index proof, NOT actual continued SQL.
Original results/typed dependencies and bounded selected prefix/absence queries
pass; first old bound still refuses another ordinary event. Legacy import work:
98343 reads/34293585 bytes;16415 writes/5269213 bytes. Append:
1116746 logical reads/446696246 bytes;672718 logical writes/260951223 bytes.
Both retain the frozen2M reads/1M writes/512MiB per-direction ceilings. Separate
name preflight is eliminated, not omitted as a safety check: conditional mutation
proves absence and refuses conflicts. Logical staging bytes are not physical
allocation, retained custody or reclaimed space. No journal/tree encoding changes.
Full core phase.exit0. Physical, original-history, final strict/fmt remain in
progress/queued; no terminal whole-pipeline pass claimed at this checkpoint.
Original metadata500k regression and exact new-source broader profiles remain owed.


Final conditional-index local checkpoint: owned61404 TERMINAL/absent,
complete.exit0 and short/maximum/core/physical/original-history/strict/fmt exits0.
Physical full staged16384 append onto4096 base finishes/reopens20480 entries and
exact old root;4099->20492 rows,1262810->6315322 stored logical bytes. Same final
16394 puts/5052712 bytes; fewer logical reads388642/132979848 bytes, unchanged
235279 logical writes/78349532 bytes.123.60s; no allocated-byte/reclaim inference.
Original archive8 pass70.89s, including actual full source, all original result
kinds, repinned negatives, peer progress and read-only validation. Final strict
is0.26s on unchanged source; no suppressions.18823 TERMINAL0/reaped adds original
metadata16 in135.96s including actual500000 bindings/thirteen edits, child exits,
concurrency, page quotas and restored minimum refusals. No later Rust changes.

Separate scoped closure: implementation PASS for conditional staged insertion,
shared typed/proof budgets and private appendable operation/sequence index.
Verification PASS for the named local source-pinned checks; broader exact-source
workspace/hosted qualification is pending. Synthetic validation PASS for immutable
index/readback/receipt preservation, not actual continued SQL or live admission.
Trace points to frozen contracts/controls and exact external candidate pins.
Six producer lenses PASS_WITH_RISK for this component: identity roots and format
vectors pass; placement grants no new authority; custody keeps physical admission/
reclaim out of scope; concurrency retains expected-root/external adoption and source
CAS requirements; provider/privacy shares all budgets/uses synthetic metadata only;
Git/adoption preserves original request/checkpoint/probe semantics. Independent/
human review and the complete plan remain OPEN. No public activation is exposed.

JC-03 high, owner Tig maintainers, design clarified / implementation OPEN:
prepublication recovery can finish only its known prospective archive. Missing an
already-published archive is known loss, not an initialization opportunity; minima
and predecessor checks precede IO. JC-04 high, same owner, OPEN: restored working
state must match original domain/instance-as-core-epoch/floor, including an empty
model. Current codec exposes domain only; a pure context-match seam and rebound
negatives are required before continued-mode admission. No caller epoch label or
internally coherent foreign model may substitute for the original authority.


Implementation checkpoint d29a57625179e7d3013bd6e2283a8a5f49825aa7 commits exact
qualified source (333 Rust/TOML/lock hashes match owned61404 pins). L0/diff/staged
redacted Gitleaks pass. Current context/resume are compact routing notes; superseded
process declarations remain in Git history and linked original evidence records.
No Rust changed after qualification. Broader d29 source now runs detached/clean in
journal-index-d29-qualification through owned44636, external journal-index-d29-owned-
429faab9cef4417cb3718ef3264ca967. Both normal targets RESERVED; separate activation
target free. Do not conflate these pending profiles with parent1c1f3ca's results.

Parent archive qualification is now terminal0 for full local profiles/500k/strict/
standalone/fresh14, plus exact1c hosted all3. Documentation27e8792 is locally
integrated without a remote PR merge; EVID-128 carries its complete outcomes and
prior failed/interrupted runs. Its new exact43135967822 remains pending, PR59 draft.
Integration resolves only active context/resume/review/pin records, preserving
current d29 runtime bytes and full remaining plan. No production capability added.


Next code entry853db3c12aa667206dfe59e673138b1d6c4d4c05: CAPACITY-CHECKPOINT
amendment freezes decoded context matching/checked consumption and bounded current
closed-reservation enumeration before code. Same WP/REQ/SPEC/IF/PKG and RUST/
PROVIDER/INTEGRATION controls are explicit there. JC-04 requires domain/original
instance/floor matching even for an empty foreign model; expected context must
come from original authority. Enumeration is current bounded accounting, not a
scan of historical receipts or removal permission. Core/physical-adopter evidence
is required before scoped closure; implementation and all these checks pending.
No controller/minimum/admission/expiry gate closes with this seam alone.


Bound-context seam: contract3a67bb0 precedes additive Decoded::matches_context /
into_domain_for_context and CapacityDomain::closed_reservations(1..256).33728
TERMINAL0/reaped: checkpoint12 (three new plus existing max/negative/vector cases)
and physical ledger adopter3 pass. The adopter uses actual original Config.instance,
rejects its caller epoch label/wrong floor and a coherent foreign empty model after
finished metadata reopen. Matching is read-only with unchanged provider/decoded
metrics. Checked conversion preserves exact tickets/outcomes/highs/unobserved debits
and invalidates old samples/pending probes. Closed enumeration excludes active/
in-flight/retired records, preserves owner/generation/order, refuses invalid bounds
and leaves the exported root/accounting unchanged; old token after reuse is stale.

81049 TERMINAL0/reaped: core regression and strict core/store all-target Clippy/fmt
pass. Only the unchanged full staged-tree delta was skipped because d29 qualified
that exact algorithm; no tree/format/cap/accounting transition changed here.
Separate closure: implementation/focused verification/scoped synthetic validation/
trace PASS for these pure interfaces and existing physical adopter. Six producer
lenses PASS_WITH_RISK within that scope: exact context/vector identity; no placement
or deletion grant; held accounting preserved; no IO or new mutation/admission;
bounded read-only working-set query; original source/probe semantics retained.
JC-04 pure seam resolved; actual continued-mode adoption must use it and remains
OPEN. Broader new-source/hosted checks and JC-01..03/metadata admission/full controller/
provider-source-witness/expiry and human acceptance remain required.


Next private record-reader entryf2c4890: JOURNAL-SUCCESSOR amendment precedes code.
Framing/read-only base/minimum proof uses existing index/meter/provider, explicit
expected original authority/context, single-path ancestry and source prefix checks.
Above-base minimum is deferred as a typed tail requirement, never declared proved.
This component cannot validate an SQL seal, reserve physical metadata or grant a
writable journal. Those joined controller gates remain required. Existing meter/
index visibility may widen only within ledger to permit reuse, not public ingress.
