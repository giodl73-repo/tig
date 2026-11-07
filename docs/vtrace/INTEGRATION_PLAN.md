# Integration and adoption controls

Each seam advances through pure contract, fake provider/producer, isolated real
temporary integration and then scoped owner-run adoption. No live cache census,
transfer, deletion, daemon or consumer runtime switch is authorized by this plan.

| Seam | Packages | Boundary / required evidence |
| --- | --- | --- |
| Identity -> publication -> claims | WP-001/003/004 | Exact raw/semantic identities, acknowledgement ordering, active-use/removal fencing and recoverable retry |
| Private view -> Git/LFS -> ref lifecycle | WP-002/005/009 | One authored source, precise index/commit view and crash-safe selection/claim handoff |
| Planner/catalog -> local/remote provider | WP-006/007/008/013 | Capability/failure-domain evidence, full-disk/corrupt/partial outcomes and physical-byte accounting |
| Art donor -> optional typed layer | WP-010/011 | Pinned source/license/lock and old/new canonical/model/graph/variant parity; no private source copied |
| Typed build -> gates/delivery | WP-011/012 | Consumed projections and authority/remote observation are separate; fake host replay/drift/privacy |
| Provider -> optional shard transport | WP-013/014 | Representation mapping, client pin, confidentiality/access, resumability and measured local reuse |
| Native -> CLI/WASM | WP-015 | Shared resolution vectors; authenticated exact Git views and blocked missing-object state |
| Generic product -> private consumer | WP-016 | Exact old/new map, dry-run storage delta, recovery/rollback and owner-run acceptance |

Integrations must name supported client/provider versions and unsupported
capabilities. A standard LFS agent does not automatically control external prune;
an opaque root pointer does not automatically gain leaf-level merge semantics.
Live access remains under explicit admitted scope/authority. Test stores and logs
must not expose private data or become retained production caches.

## Stage review

Producer-applied six-role lenses: pass_with_risk. INT-01: a passing pure donor
suite could imply accepted consumer behavior. Resolved by explicit old/new and
owner-run integration acceptance before cutover; rollback protects both byte
versions until acceptance and retention policy allow retirement.
