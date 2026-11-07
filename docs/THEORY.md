# Tig theory

Tig separates the identity of content from its physical placement. Git can version
the authored records that choose content; Tig can reconcile copies without
rewriting those versions. Art's semantic asset/dependency value is now planned
as an optional layer of Tig under the [unified design](UNIFIED-DESIGN.md),
superseding the earlier separate-product boundary. This is a target theory with a small implemented pure
mechanism, not a released storage protocol. See [current evidence](vtrace/VERIFICATION.md).

## Identity, naming and location

```text
NAME                     IDENTITY                    LOCATION
bundle snapshot + path -> object hash + size ------> place + provider locator
                                  |                   |
                                  +-------------------+ another copy
```

The name answers which object a consumer requested. The identity answers which
exact bytes satisfy that request. The location answers where a copy can be read.
Changing a location does not change the object. Replacing bytes creates another
object, even if its display name or logical path stays the same.

A moving bundle reference is resolved to an immutable snapshot before use. The
same path can name different objects in different snapshots. A physical locator
never substitutes for an exact content identity. Knowing a location is not
permission to read it.

## Snapshots, claims and observations

A snapshot describes logical membership; a claim requires retention/placement;
an observation records evidence about an actual copy. These are independent:

```text
snapshot:       these are the objects in version V3
claim:          retain V3 at working and archive
observation:    working is verified; archive is offline/unknown
consequence:    V3 is selected, placement remains unsatisfied
```

Bundles can overlap. Claims can overlap. One object at one place can satisfy
several compatible claims. Counting references helps compute demand but does not
prove independent custody, current availability or safe removal.

## Placement algebra

Let O be immutable objects, P admitted places, and C current admitted claims.
Each claim constrains placement: particular places, eligible roles, copy counts,
retention, failure-domain diversity or active use. An assignment step must either
satisfy these constraints or report the unsatisfied obligations. It cannot hide a
conflict by processing the most recent branch last.

For an assigned place p:

```text
D(p) = union of all objects required at p by admitted claims
V(p) = objects with admissible verified-presence evidence at p
L(p) = objects observed at p within the declared inventory scope

acquire(p) = D(p) - V(p)
keep(p)    = D(p) intersect V(p)
surplus(p) = L(p) - D(p)
```

The sets are meaningful only with their declared coverage and evidence scope.
Unknown objects/consumers/places are not empty sets. Incomplete demand discovery
can still justify acquiring a known needed object, but cannot justify removing
an allegedly unused one.

Surplus is a candidate for assessment. Actual removal additionally requires
current authorization, fresh all-consumer protection, satisfied independent
retention obligations and an effective provider/object fence. An execution engine
must prove these conditions; a pure set calculation cannot.

## Three states and convergence

```text
DESIRED                  PLANNED                   OBSERVED
exact obligations -----> proposed operations ----> physical evidence
      ^                          |                       |
      |                          +---- failures ---------+
      +---------------- fresh reconciliation ------------+
```

Git/claim adoption updates desired state. A plan binds exact source, policy,
protection and observation checkpoints. Execution changes physical reality and
records observations. It does not manufacture a successful copy from a desired
claim or a successful merge.

With stable requirements, admitted capabilities, available sources, sufficient
capacity and successful operations, repeated reconciliation should satisfy all
obligations without duplicate work. Those are preconditions and an intended
property, not a proved liveness theorem. Offline places or impossible capacity can
leave a truthful pending/blocked state indefinitely. Retry work, not false success.

Acquisition can add useful copies without reducing protection. Removal needs a
fresh check because claims can increase while transfers run. Use scoped fencing
and short catalog transactions rather than a global render/cache lock. No atomic
transaction spans Git, every disk and every remote host.

## Durable truth and rebuildable acceleration

```text
Git policy / immutable manifests / durable admitted claims / operation journal
                                  |
                                  v
                       rebuildable lookup indexes
                                  |
                                  v
                  scoped provider evidence and validation
```

Objects, manifest nodes and transfer representations are immutable where their
contracts require it. Operational indexes accelerate discovery; they are not a
second authored asset list. Durable job/draft admission and operation intents
cannot be discarded as disposable cache rows. Losing protection history blocks
destructive work until recovery establishes sufficient scope.

Bounded trees let versions share unchanged metadata and payloads. Physical packs
and torrent shards are representations, not bundle identity. A logical membership
change may need no transfer; a logical deletion from a shared pack may reclaim no
physical bytes. Measure payload, metadata, scratch and allocated storage separately.

## Boundaries

| System | Responsibility |
| --- | --- |
| Git | Versions authored trees and references |
| Tig typed-asset layer, optional | Resolves semantic asset IDs, variants, dependencies and lifecycle roots; planned consolidation from Art |
| Tig byte/placement layer | Resolves exact bundle paths/objects, claims and physical placement/access |
| Provider | Performs capability-declared local or remote operations |

Tig can accept a plain LFS object request without a typed asset record. Typed
scene references resolve through the optional semantic layer to exact object/bundle requests.
No provider is allowed to choose a more convenient content revision.

## What is currently tested

The pure Rust [handoff prototype](../specs/HANDOFF-PROTOTYPE.md) exercises assigned
claims, deduplicated per-place demand, stale snapshot rejection and conservative
release candidates in 13 synthetic tests. It trusts supplied coverage/replica
assertions and performs no IO. Durable recovery, native Git/LFS integration,
provider integrity/custody and live eviction remain unimplemented. All 110 broader
scenarios remain designed.

Continue with [branches](guides/BRANCHES.md), [lookup](guides/LOOKUP-AND-LFS.md),
[bundles](guides/TREES-AND-BUNDLES.md) and [recovery](guides/PLACEMENT-AND-RECOVERY.md).
