# Successor journal framing and recovery

Inputd3c331ba22f3a2f0db8e6b483b508e7d1d33662e. Design before implementation;
the private SQL fence and staged editor are prerequisites, not this delivery.
[JOURNAL-ACTIVATION](JOURNAL-ACTIVATION.md) and
[JOURNAL-CONTINUATION](JOURNAL-CONTINUATION.md) retain the complete obligations.
WP-003/004/006/007/008/013/015; REQ-001/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014; IF-002/003/004/005/006/008/012/013;
PKG-001/002/003/008/009. RUST/PROVIDER/INTEGRATION, L0/L1/L2 below.

Allowed scope: extend existing CapacityLedger, history codecs, Tree and metadata
adapters with explicit continuation, synthetic fixtures and controlled records.
No second engine, dependencies, global cache lock, cap waiver, auto migration,
payload IO or expiry. Entry: exact source, original authority/minima and separately
admitted existing metadata provider. Exit: joined prepare/seal/recover/open/write
and subsequent rotation, not a public seal-only API. Private cfg(test) staging is
permitted before the joined API qualifies; normal callers must not get partial
activation. Keep implementation, verification, scoped validation, trace and
producer review separate. Full PRODUCT_PLAN remains active.

## Stable authority and bounded records

Keep the exact original Config and SHA256 genesis. Its instance remains the core
authority epoch; segment generation is a different counter. Existing JournalRefs,
request encoding and tig.capacity-event.design-1 hashes retain their meaning.
All new records are strict canonical JSON, deny unknown fields, at most65536
bytes, and use integers0..9007199254740991 with checked arithmetic. Domain/root
paths stay provider-owned; no caller path, executable, URL or credential in records.

Frozen field names (existing Rust types retain their exact encoding):

- ActivationIdentity: {generation,digest}.
- PreparedData: {schema,config,generation,source,working_state,operations,sequences,
  activations,operation_count,clock_high,predecessor}.
- TailIdentity: {authority,generation,operation,base}.
- Preparation: {schema,operation,data,metadata,tail}.
- SealReceipt: {intent,hash}; hash is the canonical intent digest.
- ActiveControl: {schema,preparation,seal}.
- ContinuedMinimum: {schema,authority,activation,checkpoint}.
- Continued item: {schema,sequence,previous,hash,request,output}, preserving the
  existing Request/ArchivedOutput encoding.
- Ancestor: {schema,authority,generation,digest}, schema
  tig.capacity-successor-ancestor.design-1.

PreparedData (tig.capacity-successor-data.design-1) contains original Config,
positive generation, source Checkpoint, full working-state Checkpoint, global
operation/sequence/activation-ancestry tree roots, operation count equal to source
sequence, historical clock high and predecessor activation identity (null only
for original legacy authority). It contains no own hash, metadata minimum or
growing inventory. The current generation is absent from its own ancestry tree.

Write PreparedData and all referenced nodes to the admitted MetadataStore, finish,
then read back through its exact minimum. Preparation (tig.capacity-successor-
preparation.design-1) binds the operation, PreparedData Object, finished metadata
Checkpoint and exact successor identity/base. Store its canonical bytes as a
create-new provider-owned activation record. The seal intent's prepared Object
hashes Preparation, not a record containing that intent/hash. This ordering avoids
a metadata-checkpoint/self-hash cycle. An existing exact preparation is a retry;
different bytes or any missing finished dependency hold. No repair by initialization.

ActiveControl (tig.capacity-ledger.design-3) replaces domain.json only through the
controller. It contains the exact Preparation and committed seal receipt. Its
SHA256 is the activation identity. The new control does not deserialize as old
Config: an actual unmodified old library must refuse it. Old already-passed guards
are separately fenced by source SQL. No physical metadata path is encoded in the
control; an explicit caller-supplied MetadataReader must match its minimum identity.

## Tail and indexes

Each successor uses a private provider-derived segment filename from generation
and activation-operation SHA256, under the enrolled domain. Validate each directory
and regular file; do not follow symlinks or accept an imported relative path.
Prepare it explicitly, create-new, with the existing meta/events SQL shape and
original genesis. Initial global head/hash equals source; local encoded bytes0.
Normal reopen requires it to exist and validates the prepared base/schema; it
never initializes an absent journal. Preserve the sealed old database untouched.

Local limits remain16384 events and16MiB request bodies. Enforce head-base.sequence
and local bytes, never head<=16384 in continued mode. Sequence stays global and
monotonic; exact retries neither consume a new sequence nor manufacture a receipt.
First full legacy segment and later full continued segments must both rotate.

The operation tree maps op/SHA256(operation UTF8) to an immutable original request/
typed-output record. The sequence tree maps seq/20-digit zero-padded decimal
sequence to that same object. The activation tree maps activation/20-digit prior
generation to that prior control's digest record. Validate keys, item sequence,
original authority, exact request/event hash and dependency types/earlier sequence.
Selected operation/sequence entries must identify the same item; its previous
hash must match the indexed preceding checkpoint (genesis for sequence1). Resolve
typed dependency graphs to at most the existing four dependency edges in depth;
no recursive lifetime-prefix validation or renewal of old probes.
Reuse original-history design-1 items only within their original bounded range;
continued items require a distinct tig.capacity-successor-op.design-1 schema.
Both carry original response/checkpoint/output, never recomputed current replies.
Tree entry counts retain their u32 format bound; operation_count must fit that
bound as well as the safe-integer limit. This is finite admitted metadata, not a
claim of unlimited lifetime. Separate expiry and surviving fences remain needed.

Build/update only the bounded source segment's delta with staged tree edits. Do
not enumerate lifetime trees or walk a segment-linked list. Shared caller budgets
include source rows, working state, typed-output/retired leaves, both index edits
and ancestry update. Feed each edit the remaining logical limits; abort on failure,
without resetting counters per leaf/tree or continuing after an unmetered error.
Track logical staging work separately from provider calls and physical admission.
Initial controller ceilings:2M logical reads/1M logical writes,512MiB read and
512MiB write bytes, unchanged64KiB records; provider quota/peak admission is separate.
Ceilings are refusal limits, not preallocated memory or promises of free capacity.

Point reads have their own shared1024-call/4MiB ceiling across selected item,
bounded typed dependencies and retired-outcome lookup. Missing/corrupt nodes,
incomplete proof or budget exhaustion hold. Do not turn an unsuccessful lookup
into operation absence. Proven absence against the exact admitted operation root
can admit a new name; it supplies no custody or removal permission.

## Reopen and pure execution

Restore full working state using the existing checkpoint codec and into_domain,
preserving tokens, owner/generation, allocation limits/outcomes, all in-flight and
unobserved retired charges, generation/allocation/observation highs and clock high.
Invalidate old sample/pending probe admission. Historical ProbeEvidence cannot
become ObservationTicket; an exact old retry still returns its original receipt.
Before exposing the restored model, compare its domain, core epoch and floor to
the original Config's domain, instance and floor. Internally coherent foreign
working state is not acceptable. Add a read-only context-match seam to the codec
if needed; a domain-name check alone is insufficient. Prove rebound epoch/floor
and empty-state negatives, not only mismatched reservation tickets.

Keep only bounded tail outputs in memory. Resolve old own-operation retries and
direct typed dependencies through the global index. Audit every public ledger
method that currently accesses outputs, including allocation/publication/retention/
handoff/cleanup status and retirement enumeration. Active old reservations come
from bounded working state; never materialize all historical outputs to enumerate
them. Old retired allocation status uses the paged original archive. Missing data
is an error, not closed/unused. Exact same-operation/different-request reuse conflicts.

No NodeStore or provider IO inside a SQL transaction. Read bounded tail rows and
head/bytes under a short SQL snapshot, release cursors/transaction, resolve old
dependencies and replay pure state outside SQL. Revalidate exact admitted control
and tail head before adopting that staged result. For a new write, synchronize and
resolve dependencies first; an immediate transaction then compares exact head/
bytes/control identity and executes only the pure transition plus SQL append.
If a peer advanced, refuse/retry explicitly outside the transaction. Never lazily
resolve an archive during catch_up/execute under SQL. Poison uncertain handles.
Use the existing CapacityDomain::try_transition for staged pure state/commit
adoption; stage new outputs and clock state as well. A failed head comparison or
SQL commit must not leave any replay/new-command state adopted in memory.
Current handles retain the existing fresh-probe/invocation-clock admission rules;
reopen, rotation or historical replay cannot renew them.

## Minimum, seal and recovery

ContinuedMinimum (tig.capacity-successor-minimum.design-1) binds original genesis,
activation generation/control digest and acknowledged global Checkpoint. It is an
explicit independently retained admission input, not inferred from the current
root. Missing/wrong-authority/older/unprovable minima hold. For an earlier activation,
prove its digest through the admitted ancestry tree; for the current one compare
the control hash. Validate a checkpoint at/below base through the sequence index
(sequence0 is genesis), or above base against bounded tail rows. No lifetime scan.

First rotation uses the frozen legacy design-1 SQL intent. Later source heads can
exceed16384, so a distinct tig.capacity-activation-intent.design-2 additionally
binds predecessor activation generation/digest and a safe-integer global source.
Its exact fields are {schema,operation,authority,generation,source,prepared,
predecessor}; predecessor is the original ActivationIdentity and generation is
exactly predecessor.generation+1. Legacy design-1 encodings have no predecessor
field and retain their original sequence bound.
The fixed one-row table/nine SQL guards remain unchanged. Do not reinterpret a
design-1 intent's original sequence bound. A shared private inspector must strictly
recognize the exact permitted schema and validate its actual source identity.

Controller preparation/adoption uses an OS lock scoped to this capacity authority,
not a cache-wide lock. Ordinary commands keep their existing per-journal SQL path.
Concurrent controllers compare exact predecessor and source head. A prior SQL
writer may commit first; stale preparation then conflicts. After preparation and
readback, seal the exact source in its immediate transaction with no node IO.
Every source, including a continued tail, gets the immutable mutation guards.

Recovery locator amendment, before controller implementation: a predecessor
digest cannot by itself recover the predecessor's operation-derived tail filename.
After sealing, write the exact prospective ActiveControl bytes to the provider's
create-new controls/DIGEST.json, where DIGEST is SHA256 of those bytes. Sync and
read back before publishing the same bytes as domain.json. Existing exact content
is a retry; missing current/predecessor content or conflicting bytes hold. No
directory enumeration or reverse lookup from filenames. First legacy source is
the fixed ledger.sqlite; later sources are located by one bounded predecessor
control read at its known digest, then its exact TailIdentity. Validate that
control's authority/generation/hash and the derived source header/head/seal.

Each successful continued activation must leave its control record available for
the next rotation/recovery. Crash after seal but before creating the new control
record can finish that exact record from the retained preparation and observed
sealed receipt; it cannot reconstruct an unknown missing predecessor or tail.
Crash after control-record readback but before domain publication is another
required process-exit fixture. These are bounded per-generation asset-side records,
not a growing domain.json. Their retention/expiry and physical quota are explicit;
no control is removed by this protocol. Reopen/minimum lookup must not walk all
old controls; the global ancestry tree still proves prior minimum digests.
Creating the prospective new control record is permitted only while the domain
still matches the known predecessor and admitted minima do not contradict that
prepublication state. An already-published control missing its archive is known
loss: hold rather than silently recreating it. Check these conditions before IO.

Physical admission is an unresolved required gate, not a caller-number shortcut.
Before preparation writes, a qualified provider must reserve/enforce peak and
lifetime growth for immutable metadata, control/preparation files, successor SQL,
rollback scratch and source-fence growth, excluding competing capacity users.
An already-full source may have no ordinary event slot for a new reservation;
do not reserve after the fact, reset its cap, silently consume its free-space
floor or require empty accounting. A distinct admitted metadata resource or
pre-existing protected admission can support full-source continuation. Shared-
resource/bootstrap cases require an explicit safe protocol; a plain numeric limit,
fresh free-space sample or the new index's bounded RAM/IO proof is insufficient.
Private format/recovery fixtures cannot close this gate or expose public activation.

After seal commit there is no cancellation/unseal. Explicit recovery inspects the
known source and exact preparation: validate the sealed receipt, admitted roots,
bounded working state/source delta, private tail base and predecessor without
enumerating lifetime indexes; then atomically replace the bounded
active control using a synced private same-directory file. Never publish a
different preparation over an already sealed source. Exact active retry returns
the original activation result; a conflicting operation refuses. Missing or
partial preparation, tail, seal/guards or control must hold without reconstruction.

Activation returns the new minimum and no implicitly writable successor handle.
The caller must independently retain/admit that minimum before opening continued
mode. This API obligation is not proof of an external witness: qualify retained
minimum/reconciliation separately. Ordinary Checkpoint-only open remains for
explicit unactivated legacy enrollment and cannot admit design-3. A restored
legacy-looking root cannot override an admitted continued minimum. A wholly
restored world without independent minima/current authority remains unproven.

Already-open source/new handles must refuse after a later rotation, including a
writer paused after its file guard. The exact SQL guards fence that last race;
control identity detects stale observations before normal mutation. No physical
provider action gains authority merely from the journal transition. All source/
provider/witness continuation and expiry protocols remain required.

## Qualification and producer findings

L0: format/source pins, bounded records, no self-hash/path leak, exact controls and
diff checks. L1: independent literals/hashes, malformed/rebound/budget/absence,
typed outputs/dependencies, both full segments, lazy read counts and strict Rust.
L2: real finished metadata and SQL, two or more rotations beyond old lifetime caps
with active/in-flight/retired outcomes and all public method kinds, bounded reopen,
before/after prepare/seal/control process exits/lost replies, concurrent controllers/
old/new handles and another domain, actual unmodified old-library refusal plus
accepted legacy control, stale/restored prefixes/minima and missing nodes/tails.
Then both workspace/500k/strict/standalone/teaching artifacts and exact hosts.

Producer checklists, owner Tig maintainers, design pass_with_risk: identity requires
noncyclic exact authority/record proofs; placement retains unknown/protection
semantics; custody requires independent minima/physical admission and grants no
reclaim; concurrency requires SQL and controller fencing plus every recovery exit;
provider/privacy requires explicit existing adapters and bounded metadata-only IO;
Git/adoption requires original references/outcomes and synthetic old-library
fixtures. Implementation and all named integration gates are OPEN. These lenses
are not independent people, human acceptance or publication authority.


## Private record-reader implementation entry

Inputf2c48901451d84a82e0892ab9ff55d3bc732e258, after bound-context seam. The next
allowed component is cfg(test)-only ledger::continuation records and read-only
base/minimum validation, reusing private history index/meter adapters. Same frozen
WP/REQ/SPEC/IF/PKG and RUST/PROVIDER/INTEGRATION controls above. No public activation,
new engine, node/source writes, source SQL claim, physical admission or expiry.

Strict record codecs use the exact fields/schemas above. Intent wire adapters keep
legacy fields/sequence bounds and add predecessor only for design-2; a legacy
predecessor:null must not round-trip as a permitted extra field. Match intent hash,
Preparation object, operation/authority/generation/source/base and Data identities.
Metadata minimum must match the actual admitted MetadataReader identity and not
exceed its counters. Only that reader supplies bounded verified nodes.

read_base validates the independent expected authority, control/metadata/data and
activation minimum using a shared existing64MiB/262144-call meter. Prior activation
lookup is one path activation/20-digit generation with strict ancestor record;
no scans. Check current generation absent from its ancestry index. For a checkpoint
at/below base, use the bounded global sequence proof. A checkpoint above base is
explicit TailRequirement::AtLeast(Checkpoint), NOT a verified prefix or writable
journal admission. Restore working state through into_domain_for_context; samples
stay invalid. Return private base state plus unresolved tail requirement/metrics.
The eventual controller must verify source SQL seals, actual tail prefix, control
archive, physical admission and current authority before exposing any handle.

Tail filenames are segments/20-digit-generation-SHA256(operation UTF8).sqlite;
preparation names activations/SHA256(operation UTF8).json and controls use their
canonical digest. These are provider-derived relative names, never imported paths.
No IO merely by deriving them. Require exact canonical/hash vectors, unknown/extra/
rebound fields, self/predecessor/range/budget errors, coherent foreign state, missing
nodes, mismatched metadata resource and physical read-only reopen with ancestor/
current minima and deferred tail minima. Design-only framing cannot prove a seal.

## Private bounded SQL tail replay entry

Input401dcd674bff050264ac16c79fd4283438922edc. Same WP/REQ/SPEC/IF/PKG controls
and RUST/PROVIDER/INTEGRATION L0/L1/L2 above. Allowed next scope is cfg(test)-only
continuation/tail adapter over the existing SQL shape, pure execute and history
index; no public activation/open/write, actual metadata admission or expiry.
Entry is the checked Base and exact TailIdentity; exit is a read-only replayed
snapshot with acknowledged tail prefix checked, never a writable journal handle.

An empty tail cannot be bound by its head alone. Preserve existing meta/events
columns and add tig_tail(id INTEGER PRIMARY KEY CHECK(id=1),body BLOB NOT NULL
CHECK(length(body)<=65536),hash TEXT NOT NULL). Exactly one row stores canonical
TailIdentity bytes and SHA256. After its initial insertion, three fixed SQL guards
reject INSERT/UPDATE/DELETE on tig_tail; readback requires exact table/guard SQL.
The nine activation guards remain separate and unchanged. A future real tail
creator must establish and read back this header under admitted capacity before
any publication. This entry permits only explicit synthetic SQL fixture creation.

Collect one bounded SQL snapshot: exact schema/header, singleton meta row, all
local events ordered by global sequence. Enforce at most16384 rows,16MiB body total,
64KiB body,128-byte operation and64-byte hashes before copying individual fields.
SQL head-base.sequence, aggregate body bytes and final chain must agree. Close
all cursors/transaction before any node-provider IO or dependency replay. Never
initialize an absent DB or fabricate missing prefixes. A continued sequence can
exceed16384; only its local count is capped, global counts stay within u32.

Replay a consumed checked Base outside SQL with the existing pure execute helper.
Prove every tail operation absent in the original index; reject duplicate local
names and any original-name reuse. Resolve only a command's bounded direct typed
dependencies into per-command scratch; retain only bounded local-tail outputs.
Share the existing index build meter2M/1M calls and512MiB each across all old
operation/dependency proofs, with each point proof still1024/4MiB. No lifetime
output map, inventory scan, new provider cache or fresh budget per command.

Original Reservation/Allocation/Publication/Retention/Handoff/Cleanup records may
supply exact matching execution dependencies after typed/index checks. Historical
ProbeEvidence has no ObservationTicket conversion; a new Observe cannot consume
an old probe. Historical replies stay historical, and a previously recorded cleanup
must still cancel a later settlement. Final replay invalidates observations and
retains the original clock high, active/in-flight state and local original outputs.
AtLeast minimum requires the exact row prefix, not merely an equal/higher head.

Before returning the read-only snapshot, re-read exact header/head/bytes under
another short SQL snapshot; a peer advance or header change conflicts and adopts
no state. Control-file/current-authority freshness, seal observation, physical
admission and committed writer CAS/poisoning remain separate joined-controller
requirements. Returning this snapshot grants none of them.

L0 exact source/pins/format/roles. L1 malformed/header/schema/count/byte/hash/
operation/type/prefix/budget/old-probe/cleanup cancellation negatives and no cap
waiver. L2 real SQLite tail after actual original full-segment history, global
sequence above16384, active/in-flight completion across the boundary, reader
transaction released before actual peer commit, changed-head refusal, read-only
physical metadata reopen and independent prefix vectors. Both full segments and
joined write/rotation/recovery/old-writer/source-provider-witness remain owed.
Producer lenses pass_with_risk for this entry; all implementation/checks pending.

## Existing-ledger method audit for joined integration

Audited ledger.rs at401dcd6; private tail replay is not these method integrations.
Do not route only new writes while leaving old-result status on the tail map.

| Existing method family | Continued-mode obligation |
| --- | --- |
| create/create_clock/open | Preserve explicit legacy enrollment; continued admission needs its independent minimum and qualified metadata/control/source/tail checks. No automatic initialization, format migration or public partial activation. |
| identity/checkpoint | Original genesis and cached checkpoint remain stable references, not proof of current control, authority or freshness. |
| reserve/begin_allocation/complete_allocation/begin_probe/observe/finish/invalidate/retire_closed | Route every command through bounded tail synchronization, exact old own-operation retry or proven absence, bounded typed inputs and pure SQL CAS; preserve original response/hash and conflicting-request refusal. |
| begin_publication/settle_footprint/begin_retention/settle_retention/begin_handoff/settle_handoff/begin_cleanup/complete_cleanup/observe_clock | Same command route; original typed allocation/binding/cleanup/clock checks apply even when their dependencies predate the current tail. |
| publication_status/retention_status/handoff_status | Resolve original allocation kind/ticket/binding, then current core outcome or a selected paged retired outcome. Some(0), unknown in-flight and absent/missing proof remain distinct. |
| publication_status/cleanup_status | Lookup original cancellation operation even when outside the tail; proven absent cleanup is different from missing/corrupt metadata. Preserve Cancelled and exact reviewed binding checks. |
| allocation_reference/handoff_reference/publication_reference | Resolve original or tail operation and preserve each API's existing wrong-kind/absence errors; do not treat a failed point proof as None. |
| retirable_reservations | Use bounded current closed tokens, verify selected original reservation identity/owner/generation, and return exact JournalRefs. Never scan the lifetime output index or rebuild a lifetime output map. |
| allocation_state | Select exact retirement key and paged original outcome when retired; do not fully materialize old RetiredReservation histories. Validate the original allocation ticket before reporting any old zero. |
| clock_enrollment/clock_high_water/observe_clock | Original Config/clock policy and monotonic high persist across generations; exact old replies do not create fresh native-time authority. Stage clock high together with model and output adoption. |
| accounting/check_history/last_replay_count | Bounded unseen tail replay and exact current head; accounting samples remain invalid after reopen, replay count is local unseen work. No archive IO inside an SQL transaction. |
| begin_fresh_probe | Clear existing handle freshness first; an exact old BeginProbe retry remains NeedsProbe. Do not convert historical ProbeEvidence to a live ticket. |

The current read-only adapter invalidates observations after full reopen replay.
An eventual already-open writer must synchronize only unseen tail work against its
current staged model, retaining the existing invocation freshness semantics rather
than resetting every command or renewing a peer's sample. Stage model/outputs/clock
and checkpoint/bytes consistently; changed head/control or uncertain commit adopts
none and poisons uncertain handles. The fixed SQL source guards must fence rotation
after the caller's last filesystem/control guard. This audit is design evidence;
every family still needs actual public-method cross-generation fixtures, including
retired allocation status and provider/source/witness callers, before integration.

Joined preparation budget gate ST-04: individually qualified components are not a
qualified sum. Observed separate full-case replay147040227 read bytes plus index
append446696246 exceed512MiB if naively combined; their workloads differ, so this
is not an executed joined failure. Before controller implementation, run the exact
same real tail through replay, archived-output capture, both index edits, ancestry
and working-state/Data preparation with one remaining logical budget. Include
SQL row/body and base-read work; abort on refusal rather than resetting counters.
Do not enlarge ceilings or hide repeated reads in a new provider cache. A later
design may avoid duplicate absence/type proofs using bound private staged results,
but such results cannot expose a writable handle or provider permission before
all source/head/index/state proofs succeed. Freeze that design before code.

## Joined-budget diagnostic implementation entry

Input9888618c0fb72a36fd20c2f8157dbb82262bc9d6. Same WP-003/004/006/007/008/013/015,
REQ-001/003/004/005/006/007/009, SPEC-002/003/004/005/006/008/009/013/014,
IF-002/003/004/005/006/008/012/013, PKG-001/002/003/008/009 and
RUST/PROVIDER/INTEGRATION L0/L1/L2. Allowed scope is a private joined synthetic
preparation fixture plus scoped reuse of existing meters/index/archive adapters.
No activation/controller grant, native admission, public writer or cap change.

Replay may borrow its bounded collected rows so original requests can subsequently
be archived without copying the whole input. Its strict old-name checks stay in
place for this first diagnostic. Capture each exact replayed output/request into
the existing continued Item format; conditionally extend both original indexes,
insert the old control's ancestry entry, export working state and encode next Data.
Return no result until current tail/header/head are revalidated; no SQL/provider IO
overlap. Actual seal/control publication and admission remain out of scope.

One existing BudgetPort owns total work. Charge checked Base.metrics first and
conservatively charge SQL snapshots as local row count+16 reads, body bytes plus
264 bytes per row plus256KiB fixed framing. These are declared conservative logical
charges, not SQLite page IO or physical allocation. Metadata calls/bytes charge
that same port. Feed nested index edits its remaining limits, then account their
reported logical work minus already-observed provider calls. Checked subtraction
must refuse under-reporting; abort on any error, never continue with reset counters.
This accounts overlay/proof-memory work invisible to the underlying provider.

Entry exact real full original journal/physical tail and checked Base; exit one
same-workload preparation under the unchanged2M/1M and512MiB each ceilings, with
old originals/prefixes and active state retained. A diagnostic failure is evidence
to preserve and fix, not a passing controller or reason to increase budgets.
L0 pins/format/diff; L1 nested-work/underflow/budget and typed/binding negatives;
L2 full16384+16384 physical metadata/SQL, shared-meter phase evidence, independent
old/new prefix and working-state readback, then strict and original regressions.
Producer role findings remain pass_with_risk; implementation/checks pending.

### ST-04 measured failure and bound staging amendment

Diagnostic50172 on c374775/input9888618 is terminal101/complete1,424.19s.
Base/SQL/strict replay uses367726 reads/152653060 bytes; index entry receives
384217852 remaining bytes. Its operation edit reaches302037614 read bytes;
the sequence edit refuses Budget. Preserve that failed run, never increase caps.

Before optimization code: add a private non-Clone staged replay type. It stores
its own original Index, exact TailIdentity/end/body-count binding and pure replayed
model/outputs. Preparation replay may omit only old own-operation absence lookups;
it still verifies local uniqueness, canonical requests, global sequence/hash chain,
minimum, every typed dependency, clock and accounting. It cannot return Replayed,
working state, tickets, Data or provider permission to its caller.

The consuming finish method must match the same snapshot binding, archive exact
replayed outputs/requests, and conditionally extend both stored original indexes
under the same remaining global budget. Existing conditional insertion proves
every old name/sequence absent; index validation proves the same rows and typed
outputs. Only after both edits and logical-accounting success may finish return
the replayed state and new Index for ancestry/working Data construction. Any
conflict/error drops the private state; bounded immutable orphan metadata may
remain, requiring separately admitted physical capacity. No root/control is adopted.

Normal read-only replay retains its original per-row old-name proofs. Do not add
a public boolean/unchecked mode, caller-asserted proof token, lifetime cache or
second engine. Share one internal pure replay loop; expose only strict replay or
the bound private staged constructor/consuming finish. Negative fixtures require
old-name conflict, rebound snapshot/end/body, unchanged strict refusal and spent
meter preservation. Repeat the exact full joined fixture, original strict maximum,
short/index/strict/fmt before closing the scoped budget gate. Full activation,
all-method writer, physical admission, source/provider/witness and expiry stay open.

Scoped ST-04 preparation outcome: final59288 passes same real16384+16384, physical
finish/readback and retained old outcomes under one unchanged meter1123418 reads/
447596663 bytes,668057 writes/258673542 bytes. Ordinary strict maximum and index
regressions/short25/strict/fmt also pass. EVID-129 retains the failed diagnostic and
before-code amendment hashes. This closes this fixture's joined logical budget,
not physical admission, actual controller/writer/rotation/recovery or all-method/
provider/source/witness/expiry/full product acceptance.

## Existing-engine continued method integration entry

Input7caf201d785a03125173eb3a8301f49283838f95. Same frozen WP/REQ/SPEC/IF/PKG
controls above; RUST/PROVIDER/INTEGRATION L0/L1/L2. Extend CapacityLedger itself
with cfg(test)-only continued state and private explicit open over existing enrolled
metadata/current control/SQL tail. No second engine, public activation/open, native
capacity claim, cache discovery, format migration, cap waiver or expiry.

Private first-generation entry must observe the actual legacy SQL seal/guards,
matching canonical intent/hash, exact current control and digest archive, existing
derived tail file/header, admitted metadata and independent minimum. No wire-only
seal assertion or missing-file creation. General continued-source seal/controller
recovery and second rotation remain mandatory subsequent integration exits.

Route every ordinary command through existing apply/execute and every audited
status/reference/retirement method through shared lookup helpers. Retain only local
tail outputs. Original own-operation retries compare exact canonical requests and
return original receipts; historical probes stay non-rehydratable. Typed original
reservation/allocation/binding inputs may reconstruct matching tickets. Probe and
paged retired outputs use explicit historical variants, never a fabricated live
ticket or full historical output map. Preserve wrong-kind/absence/cancellation and
Some(0) versus unknown semantics. Enumerate bounded current closed tokens, verifying
their original reservation identity, rather than scanning the lifetime index.

Incremental catch-up collects only unseen rows plus bounded SQL header/head/count
and cached-prefix hash checks. Total local count/bytes remain16384/16MiB; global
sequence stays monotonic. Preserve already-open invocation freshness; reopen clears
old samples/pending probes. Because collection adds count/prefix control reads,
joined SQL framing charges conservatively increase from rowcount+16 to rowcount+20
(bytes and ceilings unchanged). No inventory/census or unbounded prefix scan.

Resolve original operations/dependencies outside SQL under shared point1024/4MiB
or bounded batch2M/512MiB limits. Stage incremental model/outputs/clock together.
Revalidate filesystem control/archive, then exact SQL header/head/bytes under a
short transaction before adopting catch-up. New writes use an immediate transaction
that compares the cached head/bytes/header and only executes pure transition+SQL.
No NodeStore/provider IO inside SQL. Peer advance conflicts without partial state;
uncertain SQL/commit outcomes poison the handle. Source SQL guards must fence a
rotation after the final filesystem guard. No minimum/authority renewal from replay.

L0 source/pins/format/roles; L1 all command/status kinds, exact/conflicting old and
tail retries, stale probe/clock, paged old zero, closed-token enumeration, malformed/
missing/budget and atomic staged failure. L2 real sealed source/finished metadata/
tail, peer catch-up/conflict, lost commit reply/reopen, full original plus real public
method tail beyond16384, legacy/new-format refusal and both original regressions.
Then both broad profiles and exact hosts. This is an intermediate engine integration;
complete controller prepare/seal/control publication/recovery, two rotations, native
admission/source/provider/witness/expiry and full product gates remain OPEN.

Legacy export audit: export_history design-1 covers a complete original bounded
segment and must refuse continued mode before NodeStore IO. export_checkpoint
remains a generic read-only working-state descriptor: it binds the current global
Checkpoint and exact state, revalidates after provider IO and grants no authority.
Reading its codec still invalidates observations. Qualify this distinction; do not
pretend a local tail is a complete original-history export.

Continued-engine reopen amendment before engine code: a peer may still have a
valid handle-local probe from the already-recorded tail when another handle opens.
The new handle must replay that peer's later Reserve without gaining its permission.
Keep recorded tail observation/pending state only inside the private engine's
pure replay model; initialize its invocation fresh/probe maps empty. Normal
read-only/staged preparation snapshots still invalidate observations. No immutable
ProbeEvidence conversion is added. Public observe/reserve/begin_fresh_probe retain
their current invocation checks; visible continued accounting stays unknown until
the handle completes its own fresh probe. Reopen/catch-up never create an Instant
or native-time ticket. Qualify two real handles with peer Reserve/Observe after
reopen, old-probe refusal on the new handle, and unchanged independent held state.
