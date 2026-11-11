# Journal continuation and eventual metadata retirement

Inputb177d4fee4ce84c0ee8a15486e731391e726fd9d. Full PRODUCT_PLAN remains active.
Priority2 lifetime is prerequisite to sustained operation and priority8 expiry.
No current root is upgraded and no current journal cap is lifted by this design.

WP-003/004/006/007/008/013/015; REQ-001/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014; IF-002/003/004/005/006/008/012/013;
PKG-001/002/003/008/009. RUST/PROVIDER/INTEGRATION profiles.

## Required end state

Continuation must support active jobs, in-flight allocations, settled publication/
retention/handoff/cleanup dependencies, closed/retired reservations, clock highs,
original exact retries and conflicting operation-ID refusal. It cannot require
quiescent-empty accounting or reset capacity/generations to make a test pass.
Working-set caps remain enforced. New reservations require a fresh handle-local
probe after reopen/activation. Pending/historical probes and timestamps never
become new permission; actual in-flight writes retain their full budget.

Keep every metadata record bounded. Canonical object/tree codecs remain capped
at64KiB per record. State, event segments and receipt indexes belong asset-side;
small control descriptors bind exact content-addressed roots/current source.
Use bounded page/read/write/byte budgets. No inventory/journal becomes a growing
Git record or a whole-domain in-memory retry map. No global cache lock or SQLite
transaction spans provider/node IO. Serialize overlapping metadata transitions
and revalidate exact current head before activation.

A working checkpoint must preserve domain/authority epoch, generation high-water,
allocation sequence, observation nonce/watermark, remaining reservations, exact
allocation limits/outcomes and all unobserved debits, including retired jobs.
It is only one input to continuation. Original per-operation results and original
checkpoints require an immutable bounded segment/receipt index; a new reply must
not pretend a current operation or fresh observation occurred. References remain
bound to the original journal genesis/operation. Distinguish authority epoch from
segment numbering; segment rotation must not invalidate active legitimate jobs or
silently re-admit old names. Core state alone is not that receipt index.

Persist/reserve intent, checkpoint/segment data, readback, activation and recovery
before reclaiming old working journal pages. Continuation must prove bounded
reopen/tail catchup and exact dependency lookup without replaying all lifetime
history. Unknown/missing indexes, unsettled outcomes or exceeded budgets hold.
Pre/post-commit process exits, concurrent old/new handles, lost replies, replaced
roots and acknowledged-prefix restoration must resolve from independently retained
minima. Older writer binaries must refuse new activated formats; current readers
must still accept legacy controls. No format decision is deployed automatically.

## Expiry is a later, separate protocol

Detailed events/receipts and per-asset deletion records can expire only after the
admitted audit window/dependencies end and a surviving current-authority/epoch
fence prevents every old decision from acting. Elapsed time and an absent label
are insufficient. New and offline/restored old peers, restored provider metadata,
same-hash new births and unknown authority coverage are required counterexamples.
An old lookup can become explicitly expired/unknown; it cannot fabricate an old
success or mint new deletion permission. A compact trusted generation/authority
fence may outlive detailed per-asset history. No permanent content-hash ban and no
claim that all replay protection can be forgotten. Actual .tig removal is explicit
source adoption; Git history need not be rewritten. No user payload is GC scratch.

## Delivery and evidence gates

First building block: [paged working state](CAPACITY-CHECKPOINT.md), with actual
existing ledger export as second adopter and pure exact-ticket recovery fixtures.
This does not implement segment activation, lazy historical replies or expiry.
Next required integrations are bounded original-output/dependency snapshots/index,
explicit durable segmented journal transition, fresh reopen/old-handle fencing,
then provider/source/witness continuation and separately authorized metadata expiry.
Do not relabel the first block as the delivered lifetime capability.

L0 pins/format/links/bounds. L1 deterministic vectors, malformed/resource cases and
locked focused/strict checks. L2 real admitted fresh ledgers and publication/
retention/retirement dependencies across more than current lifetime caps, actual
process interruption/restore/old-writer compatibility, both workspace/500k/strict,
standalone Git and exact hosts/secrets. Retained custody/current authority/time/
large-file/provider and independent/human acceptance remain separate full gates.
Implementation, verification, scoped validation, trace and six producer lenses
close separately for each block and the final joined lifecycle. No live cache,
daemon, automatic migration, deletion, browser or deployment on resume.

## Activation counterexample to retain

A legacy handle may pass its config-file guard immediately before activation and
then begin its SQL command afterward. Changing only a config/active pointer does
not fence that already-passed guard. The activation protocol must prove refusal
at a mutation authority the old writer actually checks, preserve the real final
acknowledged prefix, and recover every pre/post-intent outcome. Required fixtures
must pause a real legacy process after its guard and before its transaction,
activate/recover with another handle, then release the old process. No old writer
may append an acknowledged command to an abandoned journal. This is an OPEN
next-stage design obligation; the history archive installs no activation fence.
