# Paged working-state checkpoint review

EVID-127 candidate; inputb177d4fee4ce84c0ee8a15486e731391e726fd9d.
[CAPACITY-CHECKPOINT](../../specs/CAPACITY-CHECKPOINT.md) freezes controls,
allowed scope, framing/budgets and L0/L1/L2 before code. The
[full continuation target](../../specs/JOURNAL-CONTINUATION.md) remains owed;
working state alone cannot replace historical results or permit metadata expiry.

Method: one producer inspected core accounting, ledger replay/output dependencies,
canonical64KiB/4096-node budgets, existing Tree/NodeStore and source/minimum rules.
All findings owned by Tig producer; these six lenses are not independent people,
permission or human acceptance. Contract3afb135ff77b60bd8d77e112d24b43f66cb1f88e precedes implementation.
Focused implementation evidence follows; full gates are recorded separately.

| Lens / target | Finding / severity | Contract disposition / required evidence |
| --- | --- | --- |
| Identity / paged format | High: a giant state JSON blob would bypass established record limits | Existing Tree and strict canonical leaf framing; exact schema/path/hash/count vectors, budget and duplicate/unknown cases |
| Placement / restoration | High: a saved free sample or pending probe could become new admission | Invalidate both on restore, retain counters and held debits; old ticket rejection, new probe/racing charge fixtures |
| Custody / active and retired state | High: restoring only summary totals loses in-flight work or reuses generations | Preserve full bounded working state and retired unobserved debits; original ticket/zero-charge/finish/conflicting completion tests |
| Concurrency / ledger adopter | High: metadata export could hold SQL across IO or race head changes | Peer progress inside NodeStore callback must succeed; final head mismatch refuses descriptor. No activation/replay-fence claim |
| Provider/privacy / metadata budgets | High: per-phase budgets could multiply or source silently mutate | Shared metadata meter, immutable caller-owned port, failure leaves source unchanged; physical fresh-store export/readback and bounded negatives |
| Git/adoption / full lifetime boundary | High: codec could be reported as finished continuation/expiry | Separate required historical reply index, segment activation/old writer/restore fences and audit expiry. No current format/root migration or .tig deletion |

Design-entry disposition: pass_design_with_risk for implementing this block.
Implementation and evidence at that entry were OPEN; current outcomes follow.
No journal cap is increased and no current root is migrated or discovered.

## Implementation and focused evidence

The additive core module exports strict canonical leaves through the existing
Tree/NodeStore, with a single shared call/byte meter across tree and leaf phases.
The decoder checks hash/length, exact framing/path/counts, generation/owner tokens,
remaining/in-flight/charged arithmetic, sequence uniqueness and active-debit
correspondence. It preserves retired charges and high counters. Historical
accounting is inspectable; converting to a writable model invalidates both free
sample and pending probe. Existing transitions/receipts/formats remain unchanged;
only the observation watermark is tracked as additional private working state.
Canonical numbers retain the existing safe-integer subset, not all core u64s.
Values outside that subset refuse export rather than truncate or reset counters.

The existing persistent ledger is the second adopter: sync, export into the
caller-admitted metadata port, sync and compare exact journal head. No source
command or SQL transaction spans metadata IO. A changed head returns Conflict;
partial immutable nodes never install a root. The caller must finish/read back
its port to establish durability. There is no activation or automatic migration.

| Recorded attempt | Outcome |
| --- | --- |
| Initial core compile | Private canonical MAX_BYTES access refused; local frozen64KiB limit corrected, no visibility change |
| Initial test compile | Malformed match guard and comma corrected; no runtime claim |
| Combined17240 | Core/store check TERMINAL0 |
| Core3686 | Initial7 core cases pass |
| Store94789 | Compile101: MetadataLimits has no Default; test corrected to explicit admitted budgets |
| Focus17175 | Eight core and three real-ledger cases plus strict schema workspace/all-target Clippy TERMINAL0 |
| Boundary96011 | Nine core cases TERMINAL0, including128-byte IDs/owners/events at full1024/4096/4096 working set and safe-integer counter highs |

The maximum-counter case starts from synthetic high-water marks and then executes
real core transitions. It does not claim trillions of historical commands ran.
The literal empty header and digest are independently pinned; sixteen re-pinned
malformed groups include lost debit, path/count/generation/owner/arithmetic/schema
and sequence refusals. Missing/corrupt graphs never produce empty state. Meter
failure preserves the source model. A retired-only checkpoint preserves old-token
staleness and the next generation. Fresh observation racing a completion preserves
the later charge; zero-byte completion keeps its original sequence.

L2 fresh SQLite ledger export finishes actual MetadataStore, drops both handles,
reopens MetadataReader and recovers exact working state. Original receipt replay
on the original journal preserves checkpoint/reference and requires a new probe
for new admission. A peer commits from inside metadata.put; it progresses without
a held SQL transaction and forces the exporter to refuse the stale descriptor.
This is physical metadata reopen evidence, not journal segment recovery.

## Separate closure

Implementation and focused verification pass for this building block. Scoped
synthetic validation covers bounded working-state preservation and read-only
ledger observation. Trace: frozen contract -> canonical leaf/tree vectors ->
core accounting recovery -> existing ledger/metadata export/reopen -> peer race.
All six producer findings are addressed within this scope; pass_with_risk.
This is producer review, not independent or human approval.

Broader default75122 and all-feature13738 workspace/500k/strict regressions are
LIVE in distinct external targets. Default continues standalone/fmt/fresh schema
binaries; all continues a fresh persisted14-step publication teaching artifact.
Do not overlap or restart these targets on an observation timeout. Source pins,
L0 and exact-source hosted qualification remain pending. Full continuation,
original-output/dependency index, durable activation/recovery/old-writer fences,
metadata expiry and full PRODUCT_PLAN acceptance remain OPEN.

Parent PR57/sourceb177d4f now passes exact43093171430 Windows/Linux/secrets;
marked ready for review. Parent body records final gates. No merge or deployment.
