# Journal roles to classified placement

Input d4c33ab193a461392357640e43ae4fe2f50b2357 (PR78 candidate).
WP-008/009/010/012; REQ-002/004/006/007/009;
SPEC-005/006/007/008/010/012; IF-001/002/004/006/010;
PKG-001/003/004/006, RUST/INTEGRATION/PROVIDER profiles.
Scope: read-only ObligationSet-to-existing-View adapter, stable checkpoint identity
accessor, read-only View claim iteration and temporary journal/archive fixtures. No new journal/format/provider,
claim release, automatic source adoption, transfer or removal.
Entry: classified immutable demand roots; dependency qualification before landing.
Exit: prepared/current demands keep working placements; predecessor roots retain
only retained placements. Independent sources/jobs still compose. Missing roots
refuse; all roots share operation-wide IO/placement bounds. L0 structure/fmt/
diff/roles/trace; L1 locked all-feature tests/Clippy + standalone compatibility;
L2 actual temporary journal prepare/adopt/withdraw/reopen and detached archives.

Before adoption: current A + pending B require both working closures. After
adoption: current B remains working, predecessor A requires retained places only.
A different current job/branch still requiring A keeps its working places.
Withdrawing that job leaves its predecessor retained, never implicitly absent.
This prevents historical journal roots from holding every old revision hot.

Coalesce placements per journal/source/owner, not per historical root. Claim IDs
hash the stable journal genesis, source key and owner; independent sources sharing
one archived Demand never overwrite each other. Preserve the exact observation
checkpoint with View generation equal to its sequence. Neither is a live lease or
global cache lock. The checkpoint identity accessor changes no persisted format.

Bound roots to an explicit maximum (default 4,096; hard ceiling 65,536), coalesced
sources to the existing 1,024 and total assigned placements to 65,536. One read-only
NodeStore wrapper charges every attempted lookup and all returned bytes across
all roots, including repeated roots. No per-root IO-budget reset or partial result.
Underlying stores remain responsible for bounded acquisition. Missing/corrupt/
foreign-format roots are errors, never empty claims. Desired constraints may be
unsatisfied until providers finish placement. Do not silently omit another source
to fit a budget; an explicit admitted partition still needs complete union coverage
before destructive actions.

Projection always has complete_claim_coverage=false. It covers only the supplied
verified journal obligations, not unregistered drafts, other journals or native
opaque semantics. It neither protects bytes nor releases old provider claims.
Actual successor acknowledgment, owner-authorized recovery, fresh independent
custody and scoped physical reclaim/restore remain required later T3 work.
