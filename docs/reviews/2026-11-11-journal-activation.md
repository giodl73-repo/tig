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
