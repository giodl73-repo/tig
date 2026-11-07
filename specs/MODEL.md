# Tig placement model

Status: proposed requirements, not a released schema or implemented API.

| Primitive | Meaning |
| --- | --- |
| Object | Immutable bytes, algorithm-tagged digest and exact size |
| Tree | Immutable bounded mapping of logical path entries to objects/child trees |
| Bundle snapshot | Exact tree root plus declared scope/coverage; never a mutable archive |
| Bundle reference | Stable logical name advancing to immutable snapshots with fenced updates |
| Place | Stable storage identity, capabilities, capacity and failure domain |
| Claim | Admitted retention/placement obligation on exact resolved objects/bundles |
| Plan | Input-bound proposed transfers, checks and eligible removals |
| Observation | Scoped integrity/availability/custody evidence with source and freshness |

```text
snapshot + logical path -> immutable object
objects + all active claims + policy -> desired placement constraints
constraints + observations + capacity -> bounded plan or explicit unsatisfied state
plan + fresh revalidation -> journalled provider actions -> new observations
```

Git/adapters or the optional typed-asset layer resolve moving names and selectors
before supplying an exact snapshot to placement. Under docs/UNIFIED-DESIGN.md,
Art's generic value is planned inside Tig; native media extraction and status
vocabulary remain contract/adapter-owned. Labels
only affect placement through a pinned admitted policy, never by filename magic.
Place aliases and filesystem paths do not define immutable object identity.

For a resolved assignment to place p:

```text
required(p) = union of assigned obligations from all admitted claims
missing(p)  = required(p) - verified_present(p)
surplus(p)  = observed_present(p) - required(p)
```

Claims may constrain copy count, failure-domain diversity, place eligibility,
retention and active jobs rather than name an exact place. Assignment precedes
these set equations; infeasible constraints remain unsatisfied. Protection is
conservative when inventory/consumer coverage is incomplete. A reference count is
an optimization of known claims, not a complete custody or removal authorization.

A partial planning profile can remain unresolved. An executable plan must resolve
all required destinations, limits, capabilities and authority, or block. Conflicts
need explicit precedence or an unsatisfied outcome; do not choose last writer.

Trees/indexes use bounded immutable chunks and stable partition rules. A small
edit should replace only affected manifest chunks; measure this in fixtures.
Bundle membership does not allocate a second physical copy. Objects inside packs
require exact representation maps; shared packs may prevent immediate reclamation.
Hash trees, whole-object SHA-256, LFS OIDs and torrent identities are distinct.

Transitions keep desired checkpoint, proposed plan and observed physical reality
separate. Adoption may succeed while placement is pending/degraded. Each action
is scoped, retryable and fenced against newer claims/policies. Preserve already
verified useful copies on replanning. No global distributed transaction is claimed.

Open before format freeze: canonical encoding/domains, path normalization/collision
rules, unknown sizes, index chunking, claim authority/expiry, observation freshness,
constraint solver policy, place migration and encryption/representation identity.
