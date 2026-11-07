# Design controls and verification hooks

The detailed contracts remain in specs/ and linked guides. This index controls
their parentage, algorithms and required counterexamples without duplicating them.

| ID | Design decision | Parent specs / interfaces | Required proof / open boundary |
| --- | --- | --- | --- |
| DES-001 | Stable object identity and bounded immutable tree partitions; no naming/representation conflation | SPEC-002; IF-002 | Hash vectors, collisions/path mapping, cyclic/oversized tree and sparse update cost |
| DES-002 | Union admitted claims, explicitly assign constraints and preserve unknown scope | SPEC-001/008; IF-001/004/006 | Shared references, infeasible placements and unknown-as-empty negatives |
| DES-003 | Persist protection and intent before publication/adoption; acknowledge actual durability | SPEC-003; IF-003/004 | Crash at every ordering boundary; lost reply, torn log and stale restore recovery |
| DES-004 | Scoped reader/writer/removal fencing, not process-global pauses | SPEC-004/009; IF-004/006/012 | Granted read versus retirement, ABA and partition/expiry; prove effective provider fence |
| DES-005 | Indexed scoped observations with generation-aware stale/miss invalidation | SPEC-005; IF-005 | Watcher overflow/reconnect and catalog rebuild; no hidden scan per request |
| DES-006 | Frozen base plus private puts/removals; seal before expected-selection adoption | SPEC-006; IF-007/008 | Delete/modify conflicts, disjoint edits, late jobs and seal/index crash handoff |
| DES-007 | Standard pointer and exact Git source interpretation are lifecycle inputs | SPEC-007/008; IF-008/009 | Dirty/index stages, changed attributes, push timing and external-pruner coordination |
| DES-008 | Plan peak resources, copy/verify, recheck authority/protection, retire and measure | SPEC-008/009; IF-006/012 | Low space, external writers, corrupt archives, independent domains and allocated reclaim |
| DES-009 | Optional typed layer preserves canonical/graph/variant identity and consumed projections | SPEC-010/011; IF-010/011 | Art parity, complete native consumption, unrelated-change reuse and sparse missing variants |
| DES-010 | Evaluate exact subject under trusted policy; delivery records observed remote reality | SPEC-012; IF-011 | No stale pass, self-weakened policy, issue-close proof or unverified remote success |
| DES-011 | Reuse immutable representations/shards behind capability-declared ports | SPEC-013; IF-012 | Cross-store identity, key custody, real client reuse and no peer-count backup inference |
| DES-012 | UI uses the same resolved model, exposes gaps and never embeds private source | SPEC-014; IF-013 | Native/browser vectors, explicit commit/auth/provider boundaries and redacted export |

## Recovery invariants

TIG-P-09/TIG-I-01 govern every mutable surface. Uncertain protection or incomplete
coverage allows safe positive acquisition but blocks absence-based removal.
Expired rows, lost replies and successful hooks are not independent deletion
authority. Final archival deletion stays a separate admitted policy operation.
Prototype candidate plans do not implement these guarantees.

## Tradeoffs and deferred decisions

Explicit protected inputs/private outputs precede a mount. Leaf Git records
precede any claim that opaque root-only snapshots merge automatically. Local
verified copy precedes optional peer distribution. Backend discovery decides
reuse of iRODS/Rucio versus a minimal Rust implementation; no selection is implied.
Effective fencing and recovery semantics take precedence over a convenient API.

## Stage review

Producer-applied six-role lenses: pass_with_risk. D-01: TTL or an open file could
be assumed to protect readers portably. Resolved: design requires a demonstrable
provider-effective fence and explicit clock/renewal model; unsupported destructive
operations remain blocked. Detailed algorithms and budgets close package by package.
