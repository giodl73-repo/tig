# Paged working-state design review

EVID-127 design only; inputb177d4fee4ce84c0ee8a15486e731391e726fd9d.
[CAPACITY-CHECKPOINT](../../specs/CAPACITY-CHECKPOINT.md) freezes controls,
allowed scope, framing/budgets and L0/L1/L2 before code. The
[full continuation target](../../specs/JOURNAL-CONTINUATION.md) remains owed;
working state alone cannot replace historical results or permit metadata expiry.

Method: one producer inspected core accounting, ledger replay/output dependencies,
canonical64KiB/4096-node budgets, existing Tree/NodeStore and source/minimum rules.
All findings owned by Tig producer; these six lenses are not independent people,
permission or human acceptance. No implementation/runtime evidence yet.

| Lens / target | Finding / severity | Contract disposition / required evidence |
| --- | --- | --- |
| Identity / paged format | High: a giant state JSON blob would bypass established record limits | Existing Tree and strict canonical leaf framing; exact schema/path/hash/count vectors, budget and duplicate/unknown cases |
| Placement / restoration | High: a saved free sample or pending probe could become new admission | Invalidate both on restore, retain counters and held debits; old ticket rejection, new probe/racing charge fixtures |
| Custody / active and retired state | High: restoring only summary totals loses in-flight work or reuses generations | Preserve full bounded working state and retired unobserved debits; original ticket/zero-charge/finish/conflicting completion tests |
| Concurrency / ledger adopter | High: metadata export could hold SQL across IO or race head changes | Peer progress inside NodeStore callback must succeed; final head mismatch refuses descriptor. No activation/replay-fence claim |
| Provider/privacy / metadata budgets | High: per-phase budgets could multiply or source silently mutate | Shared metadata meter, immutable caller-owned port, failure leaves source unchanged; physical fresh-store export/readback and bounded negatives |
| Git/adoption / full lifetime boundary | High: codec could be reported as finished continuation/expiry | Separate required historical reply index, segment activation/old writer/restore fences and audit expiry. No current format/root migration or .tig deletion |

Disposition: pass_design_with_risk for implementing this building block. All
implementation/verification/validation/trace/full-continuation gates remain OPEN.
No journal cap is increased and no current root is migrated or discovered.
