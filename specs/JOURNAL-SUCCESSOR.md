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
