# Durable journal activation and legacy mutation fencing

Inputc833add85f9a9c79d3387a78e469bc13a2570403, PR59. Full PRODUCT_PLAN and
[JOURNAL-CONTINUATION](JOURNAL-CONTINUATION.md) remain the required end state.
Parent original-history candidate is focused-qualified; full72751/40677 and exact
43107903309 Windows/Linux remain live at entry, secrets pass. No target overlap.

WP-003/004/006/007/008/013/015; REQ-001/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014; IF-002/003/004/005/006/008/012/013;
PKG-001/002/003/008/009. RUST/PROVIDER/INTEGRATION, L0/L1/L2 below.
Contract and exact subprotocol framing must precede runtime changes. Foundation
qualification uses explicit owned synthetic roots only; no live-root migration.

## Required complete delivery

Extend the existing ledger engine with explicit durable activation, recovery and
bounded tail reopen, preserving active jobs, in-flight allocations, original exact
receipts/dependencies, authority identity, generation/sequence highs and clock high.
Keep existing providers and Tree/NodeStore/MetadataStore. Do not introduce another
cache engine, require empty/quiescent accounting, reset counters or raise current
16384-event/16MiB journal and64KiB metadata-record limits. Missing roots, outcomes,
coverage or independent minima hold; no automatic empty replacement or migration.

Desired/prepared/observed state stays separate. An immutable archive is not an
active journal and a prepared successor is not permission to write. No CLI/public
activation entry point may expose a seal-only operation before the complete
activation/recovery protocol is implemented and qualified. Private qualification
fixtures may establish the fence before that integration; they are not delivery.

## Selected mutation fence

A file/config guard is insufficient: an old writer can pass it before activation,
then start its SQL transaction afterward. An appended unknown command would fence
old catch_up, but it cannot be the only mechanism because an already-full command
or byte budget must also be supported without a cap waiver.

Prepare and validate immutable metadata outside SQL. Then one immediate source
transaction revalidates the exact expected acknowledged head and installs a
bounded immutable activation-intent row plus SQL guards against ordinary writes
to the old events/meta tables. Normal old writers still execute INSERT events and
UPDATE meta inside their existing transaction; the guards must abort those writes
even when the writer passed its file guard earlier. The staged core transition
must roll back and no new receipt/token may be acknowledged. New segments never
remove, disable or bypass the old guards. Direct hostile SQL/schema edits are
outside the inherited trusted-local-storage boundary; no security claim is made.

The old event sequence/hash/encoded-byte count and original replies stay unchanged.
The fixed fence record is separate bounded control metadata, not a fake ordinary
event or a larger journal. Its physical schema/journal growth must be admitted.
If an older transaction owns the write slot first, it may finish before sealing;
revalidate its actual new head, refuse a stale plan and prepare again. Unrelated
capacity domains must progress. No SQL transaction spans node/provider IO.

## Independent activation minimum

Rotation may preserve the same acknowledged ordinary Checkpoint. Checkpoint-only
witnesses therefore cannot detect restoration of the pre-fence database/config.
Continued mode requires an independently retained minimum binding original
authority, activation generation/digest and acknowledged checkpoint. A missing,
older or mismatched activation minimum holds. Do not infer generation0 or current
authority merely from a legacy-looking restored root. Legacy APIs remain usable
for unactivated legacy enrollment; new activated formats require explicit upgraded
admission, and older writers/readers must refuse them. No old epoch label is
silently reinterpreted as a segment number.

This local activation minimum is not full provider/current-authority restore
reconciliation. Source/provider/witness continuation and offline/restored old peer
counterexamples remain required before deletion or metadata expiry. An entirely
restored world with every independent witness lost remains unprovable, not safe.

## Activation state machine

1. Legacy/active source: normal bounded metadata transactions. Explicit preparation
   pins authority/minima, current head, full working state and original outputs.
2. Prepared immutable data: finish/read back admitted metadata and the private
   successor. Source remains authoritative; peer advance invalidates preparation.
3. Source sealed: immediate transaction binds exact source head and immutable
   intent to write guards. After commit only explicit recovery may complete this
   transition; cancellation must not unseal and re-admit stale actors.
4. Active successor: validate sealed intent, metadata closure, original indexes,
   working state and private successor; atomically publish the new bounded control
   format, retain the new minimum and admit only the successor's scoped authority.
5. Unknown/uncertain: hold and inspect/reconcile retained minima. No missing tail,
   journal, index or intent can become an empty initialized replacement.

Crash/lost-reply recovery must distinguish every pre/post-commit and pre/post-
control-publication boundary. An exact retry returns the original activation
receipt; conflicting reuse or source substitution refuses. No old journal/metadata
is reclaimed merely because a successor was prepared or installed. Expiry is a
separate protocol with surviving authority/replay protection and audit policy.

## Preserved identity and bounded lookup

Logical journal authority/genesis and original references survive rotation. Global
acknowledged checkpoints are distinct from segment-local count/byte budgets.
Restore the full working model with old samples/pending probes invalidated; retain
in-flight budget and original token/owner/generation/limits. New reserve admission
requires a fresh handle-local probe. Historical probes remain non-rehydratable.
Original retries return original values/checkpoints, not current accounting.

The successor must resolve old operation IDs and typed dependencies through an
admitted persistent index. Add a sequence/checkpoint index for independent prefix
minimum lookup; scanning all prior segments or materializing all lifetime outputs
on reopen is not acceptable. Preserve publication/retention/handoff/cleanup and
retired allocation semantics through lazy lookup. Unknown or exhausted lookups
hold. Stage/index updates need shared record/read/write/byte budgets and physical
capacity admission; a naive loop producing unbounded intermediate tree nodes is
not qualified. Full and near-full current segments must both continue.

Reopen reads the admitted base plus bounded new tail, not all lifetime history.
Warm handles observe current activation/source fences before mutation; outdated
handles refuse or explicitly reopen under new admission. Provider jobs already
in flight remain accounted and require their own current provider/source fences.
An old journal seal alone does not fence physical deletion or prove custody.

## Initial SQL-fence qualification framing

The first private proof uses one source SQL control row with canonical intent:
{schema:"tig.capacity-activation-intent.design-1",operation:Key,authority:Digest,
generation:u64,source:Checkpoint,prepared:Object}. generation is positive and
bounded by canonical safe integers. authority equals source.genesis; prepared is
an exact already-admitted bounded object, not an arbitrary path or executable.
Intent hash is SHA256 of these exact canonical bytes, with no self-hash field.
The SQL row has id1, body<=65536 and exact digest. No growing inventory is added.

Install fixed-name BEFORE INSERT/UPDATE/DELETE guards for old events/meta, and
immutable intent-row guards, in the same transaction as that row. Fixed SQL only;
no caller SQL fragments. Validate expected schema/trigger identities and canonical
row/hash on recovery; partial/conflicting/missing fences hold. Initial retry is
read-only for the same intent and conflicting intent refuses. Exact SQL statements
and helper API must be frozen in a committed amendment before implementation.

A source checkpoint mismatch or failed transaction leaves no committed intent or
guards. Real pre/post-commit process exits and a different domain's concurrent
progress are required. This private primitive must remain unavailable to normal
operators until the complete successor/recovery protocol above exists.

## Entry, exit and evidence

Entry: startup/router/active wave/pulses, MODEL/SAFETY/RESERVATION-JOURNAL,
LOCAL-CLOCK, CAPACITY-CHECKPOINT, LEDGER-HISTORY and JOURNAL-CONTINUATION;
actual guard/apply_once/catch_up/staging/receipt code, immutable tree/provider
contracts, pitfalls and all six producer checklists. Preserve input source SHA.

L0: source pins/bounds/links, fmt/diff and secret scans. L1: independent canonical
intent/hash/control vectors; exact/conflicting retry; head/authority/generation/
prepared-root/schema tampering; source preservation; bounded shared budgets and
no fresh observation from recovery. Locked focused tests/strict Clippy.

L2 fence proof must execute an actual pinned legacy Rust writer against the real
SQLite database, paused after its file guard and before its transaction. Resume
it after sealing and prove SQL refusal/no new event/no acknowledged token. If a
pinned source checkout needs a timing-only instrumentation patch, record exact
original SHA, exact patch and patched file hashes; label it instrumented. A model
or Python driver is supporting evidence, not that legacy-writer qualification.
Also qualify an unmodified pinned old binary's activated-format refusal and legacy
control acceptance. Do not silently substitute the current updated writer.

L2 complete activation: real active/in-flight legacy and clock-enabled journals,
full command/byte boundaries, multiple rotations beyond lifetime caps, exact old
receipt/dependency lookup without lifetime scans, actual process exits at every
intent/control boundary, warm old/new handles, restored acknowledged prefixes and
activation minima, missing/offline/corrupt metadata and source/intent substitution.
Original provider/publication/retention/retirement composition, both workspace/
500k/strict profiles, standalone Git, fresh teaching artifacts and exact Windows/
Linux/secrets are required. Provider/source/witness/current-authority/large-file/
clock/quota and independent/human acceptance remain separate full gates.

Close implementation, verification, scoped validation, trace and six producer
findings separately for the primitive and complete activation. Do not report a
private seal proof as durable activation, sustainable lifetime, .tig removal or
metadata expiry. Full PRODUCT_PLAN/course/operator/UI/Action remains active.
No live cache, daemon, browser/viewer, transfer, deletion, deployment or merge.

## Frozen initial SQL/probe amendment

Private qualification names are fixed. Control table:
`CREATE TABLE tig_activation(id INTEGER PRIMARY KEY CHECK(id=1),body BLOB NOT NULL CHECK(length(body)<=65536),hash TEXT NOT NULL)`.
After inserting the single canonical intent, create these nine guards:
for each table events, meta, tig_activation and action INSERT, UPDATE, DELETE,
`CREATE TRIGGER tig_activation_TABLE_ACTION BEFORE ACTION ON TABLE BEGIN SELECT RAISE(ABORT,'tig:sealed-source'); END`.
TABLE/ACTION substitutions come from that fixed literal list, never caller input.
The intent insert precedes its own INSERT guard. All DDL/insert/guard creation
commits or rolls back together. A guard abort is an error, never an acknowledgement.

The supporting stdlib Python probe is tools/probe_activation_sql.py, invoked only
with --root pointing to a new explicit synthetic directory. It creates original
legacy-format empty journals from fixed public config/bytes, commits these exact
DDL guards, pauses a Python SQL writer before BEGIN, releases it after sealing,
checks refusal/unchanged prefix, rollback/reopen, unrelated-domain progress and
why a pre-fence restore with the same ordinary checkpoint needs a new minimum.
It records its driver/version and labels legacy_rust_qualified=false and
activation_implemented=false. This is supporting SQLite behavior evidence only;
it cannot replace the pinned Rust/legacy-process and complete activation gates.
No runtime production migration or public seal API is added by this probe.
