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

Private reader candidate on precontract3d45e0a/inputf2c4890: canonical bounded
PreparedData/Preparation/intent/control/minimum/ancestry records and provider-derived
relative names. read_base checks expected original authority, actual metadata
identity/counters, Data/control binding, current-generation absence and declared
predecessor even when minimum is current; older activation minima get point proofs.
At/below-base checkpoints use the original global prefix index; above-base checkpoints
return an explicit unverified TailRequirement, never an active journal or admission.
Working state uses checked original domain/instance/floor consumption. IO shares the
existing history meter; no independent counter reset or provider write in read_base.

Verification:44958 initially failed compilation on a test assigning Key to the
Config.instance Digest; corrected the fixture without changing types.59486 terminal0:
seven focused cases.44927 terminal0: final nine cases,0.94s, actual finished metadata
writer drop/reopen, in-flight/invalidated sample, prior/current/future/self/missing
ancestry, foreign empty floor/instance, missing nodes, older/foreign metadata reader,
foreign authority, prefix rollback/deferred tail bounds, canonical/extra/null/rebound
records, unchanged legacy fields/cap and independent Python SHA256 literals. These
are synthetic wire/base tests, not an observed seal or actual second SQL rotation.

Owned52820 runs pinned final Rust/TOML/lock regression from external
successor-record-reader-47dfa5e009724ffd9128c8528bf42e99: records, short index,
original history, checkpoint, strict core/store all-target Clippy and fmt. Pending
until each terminal result/source pin is checked. Current broad d29 default workspace
phase is terminal0; subsequent phases and current hosted853 Windows remain pending.
Parent27e8792 exact43135967822 all hosts/secrets now pass; PR59 ready, no remote merge.

Separate closure: implementation and focused verification/scoped synthetic base
validation PASS; broader regression/trace closeout remains pending. Six producer
lenses PASS_WITH_RISK for this candidate: noncyclic canonical identities; no
placement grant; preserved in-flight accounting without custody claim; explicit
tail deferral and predecessor checks; bounded read-only provider/relative paths;
unchanged original references/legacy limits. These are producer checklists, not
independent/human approval. JC-01..03 physical admission/control recovery, actual
continued SQL/open/write/rotation, source/provider/witness, expiry and full course/
operator/UI/Action/native/authority/large-file/human gates remain OPEN.

Owned52820 is terminal1/reaped: records9/index8/original history8/checkpoint3 all
exit0; strict101 Clippy unused_unit in one empty legacy intent match arm. Removed
the redundant unit expression without suppression or semantic/type/codec changes.
Final owned13720 pins the corrected source and runs records/strict/fmt in external
successor-record-final-f2a1ac8592fa4a8f89973ee828fd9595. Pending until terminal proof.
The unchanged full32768 maximum is not repeated for private framing/visibility;
exact d29 maximum/core/physical/500k evidence remains scoped to those algorithms.

Final13720 TERMINAL0/reaped: complete/records/strict/fmt all.exit0. Nine records
pass1.61s; strict core/store all-target45.91s, formatting and all pinned source
path/hash checks pass. Only a redundant unit expression differs from prior passing
index/history/checkpoint source, so those unchanged mechanisms are not rerun.
Implementation PASS, focused verification PASS, scoped synthetic read-only base
validation PASS, trace PASS and six producer lenses PASS_WITH_RISK separately for
this private component. Full broader new-source/hosted and every named controller/
admission/expiry/product acceptance gate remain OPEN; no writable handle is exposed.

Runtime/reader0cd978e is committed/pushed on draft PR60, which now describes the
checked-context/base-reader scope and unresolved joined gates. Exact43143282830
queued; broad d29 detached44636 remains at standalone after default workspace and
strict pass. Derive final documentation head/its hosted checks independently.

Private SQL-tail entry, contractabdab3a/input401dcd6, same frozen controls/profiles:
existing meta/events shape plus exact canonical TailIdentity header and three
immutable header guards. This distinguishes even empty prepared generations.
Collect bounded SQL rows, close the transaction, resolve original typed inputs
using the shared index meter, replay with existing execute, check deferred minimum
and revalidate exact SQL header/head/bytes. Only local-tail outputs remain in RAM;
per-command dependency scratch is bounded. Historical probes have no conversion,
and final replay invalidates old free-space samples/pending tickets. This remains
cfg(test), read-only, without source seal/control freshness/physical admission.

2804 TERMINAL0:17 short tests2.70s, actual original complete/finish requests produce
identical continued SQL hashes and accounting; prior cleanup cancels settlement;
old probe/type/operation reuse holds; deferred exact prefix and malformed/budget
rows hold. A real peer commits on the first provider read and final revalidation
conflicts, proving the original reader transaction was released before node IO.
One unused test import is removed. Producer inspection then found/repaired missing
exact Base/Data-object rebinding protection and character-count SQL field bounds;
new fixtures use encoded byte lengths before Rust field copies. Clock continuity
and those negatives are added; final short20 pass3.43s on that corrected source.

Owned46516 pins all Rust/TOML/lock source and runs short/full actual SQL/index8/
strict/fmt from external successor-sql-tail-da3822c7bfe44a9b81f296f5f5c208aa.
Maximum/subsequent outcomes pending. The maximum must use a real full original16384
journal plus a physically committed local SQL tail16384, not immutable records.
It does not establish an actual public continued writer or either rotation/seal.
Implementation candidate only; final verification/scoped validation/trace/producer
closeout pending. Shared metadata admission JC-02, full controller/recovery/source/
provider/witness/expiry and all full-product gates remain OPEN.

46516 TERMINAL0/reaped: short20, full actual SQL maximum1 (359.16s), index8,
strict29.98s and fmt all.exit0; source pins pass. Maximum reaches32768 global/
16384 local with351309 metadata reads/147040227 bytes,0 writes,1021129 local body
bytes. These are replay IO/body counts, not physical quota, admission or reclaim.

Producer finding ST-03, major, owner Tig maintainers: WHERE name NOT LIKE sqlite_%
uses wildcard underscore and hides a user sqlitex_intruder table. Independent
in-memory SQLite counterexample returns old=[] versus literal GLOB=[intruder].
Resolved before final qualification with NOT GLOB sqlite_* and actual Rust/SQLite
extra-schema refusal. ST-01 Base/Data-object rebinding and ST-02 encoded byte
bounds also have their focused negatives. No lint suppression or cap increase.

Final42044 TERMINAL0/reaped: every complete/short/maximum/index/strict/fmt.exit0;
all Rust/TOML/lock source paths/raw hashes checked before phases and completion.
External successor-sql-tail-final-108d7039689444c4a49b076452d7379d. Short21 pass3.44s;
maximum273.27s with identical read/body counts and explicit original reservation
still active, old allocation Some(50) and original completion sequence1. Strict
core/store all-target16.98s and fmt pass. Source has no Cargo dependency changes.

Separate closure: implementation PASS for private immutable header/SQL snapshot/
bounded typed replay; verification PASS for named vectors/negative/clock/cleanup/
peer/full-cap cases; scoped synthetic physical SQL+metadata validation PASS; trace
PASS in this record/spec/method audit. Six producer lenses PASS_WITH_RISK: exact
original authority/header/Data identities; no placement grant; held state without
custody/removal claim; released SQL transaction and changed-head conflict; bounded
metadata-only IO/no provider paths; unchanged original command hashes/limits.
These checklists are not independent/human approval. Full continued public methods,
writer CAS/poisoning, activation/recovery/control/seal/admission and all product
gates remain OPEN. Existing public-method audit names their required behaviors.

ST-04, major/open, owner Tig maintainers: joined preparation must meter replay,
archived output capture, both indexes, working state/SQL rows/ancestry together.
Separate observed maxima147040227 replay bytes plus446696246 index-append bytes
exceed512MiB if naively combined. Their workloads differ, so this comparison is
a risk finding rather than an executed joined failure. Next exact same-workload
fixture must enforce one remaining budget without repeated proofs/caches/cap
waivers. JC-02 physical metadata admission remains separately open even if it fits.

ST-04 diagnostic entry: c374775 contract precedes code, input9888618. Existing
BudgetPort now offers ledger-scoped remaining/nested logical accounting with checked
subtraction; existing Item/archive/index helpers are reused, no new provider cache.
Replay borrows the bounded SQL rows for later capture, retaining strict original-name
proofs in this first measurement. One port charges base/SQL/replay/output/index/
ancestry/working Data. Nested edit reports include hidden proof/overlay work not
seen at the underlying provider; under-reporting/budget refusal aborts unchanged.

60174 terminal0:23 short cases2.49s. Two-row joined preparation finishes/reopens
actual metadata, preserves original/prefix replies, ancestor and closed/50-byte
working outcome. Measured252 reads/325095 bytes,52 writes/12901 bytes including
declared conservative SQL/base charges. Nested work negatives preserve spent
counters. An unused mut in the test is removed; source otherwise unchanged.
Owned50172 pins final diagnostic source in external
joined-preparation-diagnostic-99b730a6c2ec48d0af6ed9198a0ce67a; full same-workload
16384+16384 preparation outcome pending. This is a required-success diagnostic,
not authority to treat a Budget failure as product completion. All joined controller,
physical metadata admission, rotations and full-product gates remain open.

50172 TERMINAL1/reaped: full joined required-success fixture refuses Budget424.19s.
After base/SQL/replay367726 reads/152653060 bytes, only384217852 read bytes remain.
Index operation edit reaches302037614 bytes; sequence edit refuses. This confirms
ST-04 on the same actual full original/tail workload. Exact failed source/logs are
preserved, no caps increased. Pre-optimization amendment is frozen in the spec and
external staging-amendment-before-code.json with spec/old code hashes and input988.

Candidate fix: a private non-Clone Staged owns original Index/frame/head/body-count
and pure state, exposing only metrics. Preparing replay defers duplicate old-name
proofs to existing conditional index insertion. Consuming finish validates exact
same rows and both indexes/shared work before it releases state for working Data.
Normal strict replay keeps per-row proofs. Conflicts may leave bounded immutable
orphans but no control/root/model adoption or provider authority. Frame/end/body/
raw operation-column mutation refuses, including index event/hash/type checks.

95475 terminal1:24 pass and one fixture NeedsRecovery because its intentional
orphan write was not finished before a second independent reader. Corrected the
fixture with explicit finish, without changing reader recovery behavior. Final
59288 short25 pass3.04s, exact pinned root joined-preparation-owned-
907450afd81a4ecca5606b7fc5820871. Full joined/strict maximum/index8/strict/fmt remain
pending. This candidate does not yet close ST-04 or any public controller/admission.

59288 joined.exit0,357.72s: exact same actual full original16384+SQLtail16384
prepares both indexes, ancestor, working state/Data and finishes/reopens metadata.
Original/prefix readbacks and old50-byte working outcome pass. One shared meter
includes base/SQL/typed capture, physical provider calls and hidden logical work:
1123418 reads447596663 bytes,668057 writes258673542 bytes. All unchanged ceilings
fit. Before index, preparing replay uses16570 reads5675112 bytes rather than the
failed strict367726/152653060. No provider cache, cap waiver or public unchecked
mode. Original strict maximum/index/strict/fmt still pending before final closure.

Older broad44636 TERMINAL0/reaped, every phase.exit0: exact clean d29 detached
default/schema and all-feature workspace (both half-million/thirteen-edits cases),
strict Clippy, standalone Git, fmt/bins and fresh14 artifact/catalogue. Artifact
journal-index-d29-qualified-179d88bc6af84274ad887514071b8213:14 checkpoints,
complete_coursefalse;2PUT54 bytes,28 requests270 GET bytes; custody/removalfalse.
Catalogue read-only0payloads, automatic_evictionfalse. This closes the earlier
local broad d29 obligation only, not new source/hosted/controller/product gates.

Final59288 TERMINAL0/reaped: complete/short/joined/maximum/index/strict/fmt all0,
all Rust/TOML/lock paths/raw hashes checked before phases/end. Short25 pass3.04s;
joined357.72s; ordinary strict maximum191.84s still351309 reads147040227 bytes,
0 writes; index8 pass; strict core/store all-target24.17s and fmt pass. No dependency,
format, journal/tree limit or native deadline changed. The staged private path's
full physical finish/readback preserves both prefix indexes, ancestor and working
old50-byte outcome without duplicate name reads or model release before insertion.

Separate closure: implementation PASS (private bound staging and nested meter),
verification PASS (short binding/conflict/accounting plus full same-workload),
scoped synthetic physical SQLite/metadata preparation validation PASS, trace PASS.
Six producer lenses PASS_WITH_RISK: exact bound snapshot/original identity; no
placement/removal permission; retained active/in-flight accounting; released SQL
transactions/consumed failed state and unchanged strict path; one bounded logical
budget/no new provider cache; exact original replies and both original/new prefix
readbacks. ST-04 closes only for this named preparation profile. This is not
independent/human approval, native metadata admission, public controller/write/
activation/recovery, full provider/source/witness/expiry/course/product acceptance.
The earlier measured Budget failure and fixture failure remain preserved above.

Existing-engine entrybd0fabd/input7caf201 freezes method routing before code.
Private cfg(test) continued state is part of CapacityLedger, with existing
apply/execute, shared typed status/reference lookup and bounded current closed
token enumeration. Old Probe/Retired outputs are explicit historical variants;
old probes never rehydrate and old retired zeros remain paged. Own retries return
original receipts, conflicting requests hold, cached outputs are local-tail only.
Actual observed legacy seal/guards, exact control/archive/header, finished metadata
and independent minimum are required; a wire receipt alone never admits a writer.
Missing resources hold without creation. General continued-source seal/controller/
second rotation and native physical admission are still required.

Before engine code, the reopen amendment and source/spec hashes were recorded at
continued-engine-entry-0332bb80928d4d18a617d8ae0525ae1f. Internal recorded tail
observation/pending state is needed to replay a peer's legitimate later Observe/
Reserve; invocation Instant/probe maps stay empty after reopen, old evidence has no
ticket conversion, and visible accounting stays unknown until own fresh probe.
Read-only/preparation snapshots retain observation invalidation. Generic working
checkpoint export remains read-only/no authority; original-history export refuses
continued mode before node IO. Extra SQL count/prefix control rows are charged
conservatively as rowcount+20 in joined preparation; ceilings remain unchanged.

Initial parser/import errors were corrected.17947 five tests pass with private
visibility/dead-code warnings;51286 seven pass after narrowing visibility/reusing
strict replay.90801 adding private Guarded stage exposed a non-exhaustive legacy
clock-test match, corrected explicitly.63454 final focused12 pass3.53s: real old/
new public-method command families, historical/new statuses/retries/zero, actual
seal versus wire-only assertion, no missing replacement, both reopened-peer probe
cases, catch-up/write head races, lost commit reply/poison/reopen and export scopes.
35892 terminal1/reaped: short37,legacy26,clock7 all0; strict101 needless references
in two lookup calls. Removed without suppression/behavior change;50008 strict0.

Final owned35364 pins all Rust/TOML/lock source at continued-engine-final-
d4a74510c61142e29b9eb59b8e8fc631. Short/legacy pass, remaining clock/strict/full
method-written tail/joined/strict replay/archive/checkpoint/index/fence/fmt pending.
No implementation/verification/validation/trace/producer full closure yet. This
private integration supplies no activation, native admission, recovery or cutover.

35364 engine-maximum.exit0,522.46s: an actual full original16384 journal is sealed;
existing CapacityLedger methods commit another16384 commands to the derived tail,
global32768/local971987 encoded bytes. The cached output map contains exactly the
16384 local results, not lifetime outputs. Exact original reserve and new completion
retry remain available at the full cap; new writes/probes refuse. Actual finished
metadata/tail reopen under the independent minimum preserves old50-byte outcome,
exact original/new receipts and no fresh reservation permission. This is a real
method-written tail, not raw SQL insertion or immutable synthetic index records.
Final short37/legacy26/clock7/strict-entry all.exit0. Remaining joined/strict replay/
archive/checkpoint/index/fence/final strict/fmt are still pending; no full closure.

Final35364 TERMINAL0/reaped; every phase and complete exit0, all Rust/TOML/lock
paths/raw hashes checked before phases/end. Short37, legacy26, clock7; actual
full method tail522.46s; joined479.09s; strict read-only maximum231.43s; original
history8/checkpoint3/index8; fence10pass1explicit-old-writer-ignored; both strict
entries and fmt pass. Joined totals1123422reads447596663B,668057writes258673542B;
only four additional logical control reads versus7caf; ceilings unchanged.

Separate closure: implementation PASS for private existing-engine integration;
verification PASS for named focused/full/regression source; scoped validation
PASS for synthetic real filesystem/SQLite method writes and process reopen;
trace PASS against the before-code method audit. Six producer lenses
PASS_WITH_RISK: exact original/local identities; no policy/deletion permission;
retained active/in-flight/zero outcomes; staged peer conflict and poison/reopen;
no metadata provider IO inside SQL or implicit creation; original receipt/retry
and legacy regressions preserved. This is no independent/human approval.
Full public entry/controller, continued-source seals, two rotations, native
physical metadata admission, provider/source/witness/expiry/course acceptance
and new-source broad/exact hosted qualification remain OPEN. Prior pending
paragraphs describe historical observations, superseded by this terminal result.

Continued-source SQL seal entry input20a3865 is frozen in JOURNAL-SUCCESSOR.md
before implementation. External continued-seal-entry-451e5aee2d16462fbc898fcb254d5479
records raw source/spec hashes. Same named controls and RUST/PROVIDER/INTEGRATION
profiles apply. Shared canonical SealIntent preserves original design-1 fields/
bytes/cap and admits distinct design-2 predecessor/global head. Actual inspector
requires the exact tail schema plus all ten original activation objects, immutable
header generation, bounded local count/bytes and exact source. The sealer compares
predecessor with the checked active Mode control outside SQL, synchronizes existing
methods and rechecks header/head/bytes in its immediate transaction before guards.
No provider IO inside SQL, ordinary event slot, public seal API or cap waiver.

First42724 TERMINAL0/reaped: four source seal/rollback/lost-reply/paused-writer/
predecessor cases pass1.06s. Extended41344 TERMINAL101: malformed-header fixture
called nonexistent TailIdentity.to_json; corrected to existing canonical bytes,
not a new codec. Retest58016 TERMINAL0/reaped: seven cases pass1.68s, including
actual child exits at Written/Committed, strict extra/partial schema and rebound
header refusal, all nine mutation guards and sealed-tail open refusal. Unconfigured
child helper is a harness entry; two actual exit77 executions provide exit proof.

Final3708 owns continued-seal target and all Rust/TOML/lock path/hash pins before
phases/end at continued-seal-final-135cef7a939c42268db68fbfb0a81573. Focused44 pass;
legacy/clock/original-seal/strict/full32768-seal/joined/strict maximum/archive/index/
fmt results remain pending; derive exact exits. Implementation candidate only;
full verification/scoped validation/trace/producer closure is not yet claimed.
Actual controller/source locator/minimum publication/two rotations/native physical
metadata admission and full product/expiry/teaching/adoption gates remain OPEN.

Pinned-seal interim evidence: focused44/legacy26/clock7/original-seal10 all0;
strict-entry0. The explicitly ignored pinned c833add old-writer test also passes
0.77s through the current compiled test binary; its original executable SHA
bf6846e501c841631461c984cce7b256ee662af313b0a52837aee129e324dc1e was
checked before/after. This is the existing timing-only instrumented qualification,
not an unmodified-binary claim. Full-tail-seal/joined/strict maximum and final
regressions remain pending on live3708. Do not edit its pinned Rust source.

Next bounded source-locator/open design is appended before its implementation.
Six producer lenses (owner Tig maintainers), design only PASS_WITH_RISK: identity
checks both digest identity and complete prior tail header; policy adds no
placement permission; custody keeps expiry/removal a separate required protocol;
recovery refuses known missing archives and does not recursively repair history;
provider paths derive from one validated known control with no census; Git/adoption
preserves exact original/intermediate replies and independent minima. The relaxed
original-file prerequisite after generation1 is only a bounded dependency, never
an authorized cleanup. Actual multi-generation/source-rebind/minimum/stale-handle
fixtures, exact input freeze, implementation and all runtime closure remain owed.

Full-tail-seal.exit0 on pinned3708,467.35s: actual full original16384 plus actual
method-written local16384 reaches global32768,971987 local encoded bytes. Physical
metadata/tail reopen preserves exact original/new receipts and old50-byte outcome,
local output map16384, no freshness renewal and cap refusal. The same full source
then seals with observed design-2 intent/unchanged nine guards without consuming
an ordinary event slot; checkpoint32768 and local outputs remain unchanged.
Joined/strict maximum/archive/index/final strict/fmt remain pending. This is not
second activation/controller publication, complete rotation or physical admission.

Final3708 TERMINAL0/reaped. Every phase/complete exit0, plus explicit pinned legacy
exit0; all Rust/TOML/lock paths/raw hashes checked before phases/end. Focused44,
legacy26, clock7, original-seal10pass1externalignored; explicit pinned legacy1pass;
full-tail-seal467.35s, joined448.96s, strict maximum239.46s; history8/checkpoint3/
index8, strict-entry/final strict core/store all-targets and fmt pass. Joined metrics
1123422reads447596663B,668057writes258673542B; ordinary strict replay unchanged
351309reads147040227B,0writes. No limit/dependency/format/native deadline changed.

Separate closure for this named private source-seal seam: implementation PASS
(shared exact intent codec, actual SQL source inspector and existing-engine sealer);
verification PASS (named focused/full/regression and hash-checked old-writer case);
scoped validation PASS (synthetic real filesystem/SQLite writes, full32768 source
and actual process exits/reopen); trace PASS against input20a3865/before-code entry.
Six producer lenses PASS_WITH_RISK: exact predecessor and header identities;
no placement permission; retained old accounting/replies and no cleanup; source
head CAS/paused writer/rollback/poison/lost reply; bounded exact schema and no
provider IO inside SQL; legacy canonical vector/cap/old-writer compatibility.
No independent/human approval. General source locator/open, complete controller/
publication/recovery, independently retained minima, full multiple rotations,
genuine physical metadata admission JC-02 and source/provider/witness/expiry/course
or full product acceptance remain OPEN. Earlier pending paragraphs are historical
observations superseded by this terminal result; later source needs its own hosts.

General source-locator entry uses qualified private-seal input090d629; the exact
before-code source/spec hashes are external at general-source-entry-
a0060baabe79468fb1a06248dc1c0f5f. Existing Mode guard/open now resolves a bounded
Source descriptor: legacy fixed source only for generation1; otherwise one known
predecessor digest archive, checked canonical identity/authority and derived tail.
Actual seal inspection is followed by complete prior TailIdentity comparison;
final resource/control checks follow replay before returning the existing engine.
No recursive control walk, directory census or original-file dependency aftergen1.
No source/provider IO inside SQL, implicit creation, public API or quota/cap change.

57244 TERMINAL0/reaped: three focused cases pass1.32s.33856 TERMINAL: expanded
four cases pass1.65s; strict101 collapsible_if at the header comparison. Collapsed
that branch without suppression; strengthened missing-source no-creation assertion.
The four cases use actual existing-method writes through three generations, exact
original/intermediate/latest replies, inherited in-flight completion at0, ancestor/
current/forged minima, no probe renewal and stale-handle refusal. Generation2 opens
with original ledger file held aside. Generation3 still opens with ancestor archive
held aside, proving one predecessor lookup rather than recursive history reads.
Missing/rebound prior archive and missing source hold without repair. A fully
matching observed seal can coexist with a rebound operation-only tail header;
the new exact prior header check independently refuses it. A wire-only receipt
without actual source guards also refuses. Publication is manual synthetic fixture
setup, NOT the durable activation controller or metadata expiry authority.

Final16268 LIVE at last check, root general-source-final-
7f4390d09c2d4509a23c256d4d5f9d31, reserved continued-seal target. Pins all Rust/
TOML/lock paths/raw hashes before phases/end. Runs continuation excluding the three
previous maximum workloads, legacy/clock/seal/strict/archive/checkpoint/index/fmt.
New-source full-cap multiple rotations and broad/hosted remain separate required
gates; no implementation/verification/validation/trace/producer final closure yet.
090d629 hosted43163826567 remains pending and excludes this dirty source candidate.

Final16268 TERMINAL0/reaped: every phase/complete exit0 with all Rust/TOML/lock
path/hash checks before phases/end. Focused48, legacy26, clock7, original seal10
(one external case ignored in that command), history8/checkpoint3/index8; strict
entry16.24s/final strict and fmt pass. Explicit hash-checked pinned legacy1 also
passes0.31s from this compiled source. The named fixture/short regression scope
is verified; the three prior maximum workloads were explicitly excluded here.
Do not credit090/20a3865 hosts or3708 maxima to the dirty new source. Full-cap
multi-generation writes/preparation/reopen under unchanged budgets remain REQUIRED
before claiming the joined operational lifetime capability. General controller,
physical admission and other full-product gates are unchanged and OPEN.

Next concrete scale gate: one full original16384, then two actual method-written
16384 continued tails and two prepared/observed source seals, reaching49152 before
a new generation3 command. Retain original/intermediate/latest exact replies,
active reservation plus inherited in-flight allocation, zero completion, ancestor/
current minima, local-only output maps and no probe renewal. Reuse the actual
joined preparation helper and existing providers, with one unchanged shared build
budget per preparation (2M reads/512MiB,1M writes/512MiB), unchanged point proofs,
16384/16MiB local bounds and global monotonic source/clock/instance identity.
An explicit larger finite synthetic metadata profile may support the workload;
it is not native physical admission or a runtime cap waiver. A measured Budget
failure must be preserved and fixed within the original logical limits. This
full-cap fixture and controller are not implemented by the small four cases above.

Full multiple-generation entry is frozen before code in JOURNAL-SUCCESSOR.md;
external full-rotations-entry-f22e0456b3344022a266527d899eb2d4 contains exact source/
spec hashes and input090d629 plus qualified16268 general-source candidate. Normal
fixture profile remains unchanged; explicit full fixture requests1048576 metadata
nodes/512MiB bytes/768MiB DB/64KiB floor within existing provider-supported bounds.
This finite synthetic profile is NOT genuine native physical metadata admission.

Owned46476 TERMINAL0/reaped: all phase/complete exits0, all Rust/TOML/lock paths/
raw hashes checked before phases/end. Focused48 pass10.88s, default strict-entry
15.57s; actual full multiple-generation workload1087.57s; history8/checkpoint3/
index8/final strict/fmt pass. Original actual16384 + method tail16384 + method tail
16384 reaches49152, then new generation3 completion reaches49153. Exact original/
intermediate/latest receipts survive reopen, inherited pending allocation becomes
Some(0), original authority/config stays equal, ancestor/current minima work and
new reservation still needs its own fresh probe. Local cached outputs reset to0
at each activation and1 after the new command, never a lifetime output map.
Both full old handles refuse; neither local16384 cap is waived.

First preparation1151198reads460836597B,681672writes265144865B; second preparation
1135541reads452776080B,673999writes261228062B. Each includes base/SQL/staging/index/
ancestry/working work under the unchanged2M/512MiB reads and1M/512MiB writes.
The two local body totals are1037522 and1053921 bytes; source checkpoint sequences
stay global32768/49152 with no epoch reset. No failed diagnostic was hidden; this
first declared full workload passed its original profile and logical limits.

Independent build target44924 TERMINAL0/reaped also verifies identical pinned
Rust/TOML/lock source: workspace all-feature/all-target strict Clippy and fmt0.
Implementation PASS for private general source descriptor/guard/open; verification
PASS for named source negatives/short regressions/full49153 workload; scoped
validation PASS for actual synthetic SQLite/metadata/method writes and reopen;
trace PASS to the before-code entries and controls. Six producer lenses
PASS_WITH_RISK: exact prior digest AND complete header identity, no new placement
permission, active/in-flight/zero protection, immutable source seals/stale refusal,
finite unchanged logical budgets/no SQL provider IO/no implicit initialization,
exact original replies/minima/legacy regression preservation. This is no independent
or human approval. Manual fixture control publication is NOT qualified atomic
controller publication/recovery. New-source broad/hosts, genuine metadata admission
JC-02, source/provider/witness authority, expiry and complete product gates OPEN.

Older41952 exact7caf201 broad TERMINAL0/reaped: every default/all-feature workspace/
500k/strict/standalone/fmt/bins/artifact/catalogue phase0. Fresh external artifact
joined-preparation-qualified-96c070573b47400990e3ccf0193f09cf reports14 checkpoints,
complete_coursefalse,2PUT54B,28requests270GETB, retained_custody/removal_authorityfalse;
catalogue read-only0payloads/automatic_evictionfalse. Both normal targets released;
teaching-playback-all was then reserved/released by44924. These older checks do
not qualify the new source. Exact20a3865 broad60988 still live, now all-tests.
