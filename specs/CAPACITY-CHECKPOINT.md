# Paged capacity working-state checkpoint

Inputb177d4fee4ce84c0ee8a15486e731391e726fd9d, PR57. Parent local both profiles,
500k/strict/standalone/fresh14-step artifact pass; exact43093171430 Linux/secrets
pass, Windows live at entry. Contract precedes code. Full PRODUCT_PLAN active.

WP-003/004/006/007/013/015; REQ-001/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014; IF-002/003/004/005/006/008/012/013;
PKG-001/002/003/009. RUST/PROVIDER/INTEGRATION profiles.

Allowed: additive pure tig-core capacity checkpoint/tree codec, observed watermark
tracking with unchanged existing transitions, explicit read-only existing ledger
export through an admitted metadata NodeStore, and synthetic recovery/corruption/
budget/concurrency vectors. No dependency, current journal/event/receipt format
change, old-root upgrade, cap increase, activation, historical receipt pruning,
provider initialization, payload transfer/removal, CLI action or expiry.

Entry: MODEL/SAFETY/RESERVATION-JOURNAL/capacity arithmetic/Tree/identity codecs,
engineering router/wave/pulse/pitfalls and all six roles. The existing Core history
retirement and ledger caps do not qualify lifetime continuation. Exit: actual live
and closed jobs, incomplete/completed allocations, retired unobserved charges and
generation/nonce highs survive exact paged round trip; new probe remains required.
The existing persistent ledger is a second adopter, exporting exact head + state
without a transaction across metadata writes. Full continuation stays OPEN under
[JOURNAL-CONTINUATION](JOURNAL-CONTINUATION.md).

## Framing frozen before implementation

Reuse public Tree/NodeStore and strict canonical JSON, not a new storage engine or
large serialized state record. Each leaf is a deny-unknown-fields envelope
`{schema:"tig.capacity-state.design-1", entry:...}`. Entry variants:

- header: domain, epoch, floor, next_generation, allocation_sequence,
  observation_nonce, observed_through, pending_observation, sampled_free,
  reservation_count, event_count and debit_count;
- reservation: id, owner, generation, peak, remaining, closed;
- allocation: reservation generation, event ID, limit and optional completed
  {charge,sequence};
- debit: positive sequence/bytes for unobserved allocation, including retired jobs.

Paths are `header`, `r/GENERATION`, `a/GENERATION/SHA256(EVENT_UTF8)` and
`d/SEQUENCE`, using canonical decimal counters. Root is the existing exact Tree
Object; no self-hash field. Event-path hashing is this new framing rule, not any
existing operation-key recipe. Token strings remain existing bounded ASCII IDs.

Budget: existing1024 reservations/4096 events/4096 debits; at most9217 leaf entries,
64KiB each metadata object,65536 total metadata get/put calls and16MiB aggregate
read/write bytes per operation. Meter Tree nodes and leaf IO together; no separate
per-phase reset. Caller may lower budgets, never raise these caps. No recursive
unbounded graph walk, duplicate/aliased paths or silent dropped state. Partial
export can leave unreferenced immutable metadata but returns no valid descriptor;
caller owns staging/resource admission. It does not mutate the source model.

Decoder verifies content hash/length, exact canonical bytes, schema/path binding,
counts and all accounting invariants. Check nonzero unique reservation generations
bounded by high-water, owner/ID tokens, positive limits/peaks, remaining and closed
state, in-flight sums, completed charge/sequence bounds and uniqueness, observed
watermark monotonic range, positive unique unobserved debits and exact active-event
charge/debit correspondence after that watermark. Retired debit provenance remains
bound by the admitted root; this is not reconstruction of all historical receipts.
Reject invalid arithmetic/overflow, unknown fields, malformed/missing nodes,
conflicting references and exhausted budgets. A different self-consistent root
is a different input, not proof of current authority. No IO or permission arises
from a core decoded object.

Restoration keeps original token/owner/epoch/generation and allocation results,
including zero charge and incomplete writes. It invalidates sampled-free admission
and every pending observation; preserve nonce/allocation/generation/observed-through
high-water so old tickets stay stale and new tickets cannot alias. Existing live
reservation/allocation replay remains exact, finish still holds incomplete work,
and next admitted probe accounts racing later allocations conservatively. Expose
recorded accounting as historical data only; never return a writable model that
implicitly carries fresh free-space permission. No serialized monotonic Instant.

## Existing ledger adopter

An explicit export API returns a small typed design-1 descriptor binding exact
original journal Checkpoint and state Tree Object; state nodes go to the caller's
bounded admitted metadata port. Sync/revalidate the source before/after node IO,
with no SQL write transaction across that IO. If a peer advances the journal,
return conflict, not a falsely current export. No source event, accounting reset,
probe, root creation or metadata activation by export. Original old replies and
current accounting remain unchanged. The descriptor is an observation, not a
migration, new journal genesis or current protection/deletion authority. Legacy
writer/reader formats remain untouched. Future continuation must add exact
original-output/index/activation/fencing evidence separately.

L0 pins/links/source bounds/fmt/diff. L1 independent literal canonical/hash vector,
active/in-flight/zero-charge/retired-debit/generation recovery, stale tickets/new
probe, re-pinned malformed/path/count/arithmetic cases and shared budget failures,
strict Clippy. L2 existing fresh ledger + second immutable metadata store, exact
head/root readback/reopen/export-no-command and peer-progress conflict; all old
core/ledger/publication tests, both workspace/500k/strict/standalone, exact Windows/
Linux/secrets. Store/metadata side effects use owned synthetic roots only.

Close implementation, verification, scoped validation, trace and six producer
checklists separately. This does not deliver segmented lifetime, original-reply
index, old-handle/writer activation fences, tombstone expiry, real authority/time/
provider/large-file or independent/human acceptance. No live operations/merge.
