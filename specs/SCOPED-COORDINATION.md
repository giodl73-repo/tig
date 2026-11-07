# Scoped object fences and capacity accounting

Input main e09a3d591a5c2566fff524d5055792f14b3f042c. Bounded WP-004/006
subsets: REQ-002/003/004/005, SPEC-004/008, ARCH-001/002, IF-004/006/012,
DES-004/008, PKG-001/002/003, RUST/PROVIDER profiles. Scope: pure resource
accounting and temporary-store local fences. Full WP-003 remains in progress;
this component work does not waive its prerequisites for live integration or
close either dependent package. No media/cache operations or deletion API.

## Object-scoped exclusion

The local adapter uses stable per-object OS lock anchors: shared handles for
read/publication, exclusive handles for conflicting operations. Acquisition is
nonblocking and returns Busy for that object only. Several readers coexist;
unrelated objects/places can proceed. Publication holds its shared guard through
protection, verification and completion. A returned reader keeps the guard until
it closes. Anchors are never unlinked/replaced to clear a lock; process exit
releases handles. No TTL expiry substitutes for effective exclusion.

An exclusive guard is mutual exclusion only: it grants no deletion authority,
complete claim coverage, independent custody or durable job protection. There
is still no retirement API. Hostile namespace replacement, remote filesystem
locking semantics and machine/partition recovery remain unsupported. Root
bindings must be trusted and all managed actors cooperate with this provider.
The standard-library primitive/caveats are documented in
[Rust File locking](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock).

The experimental local manifest advances to design-2 with a fences directory.
Opening an older root rejects without creating files or migrating anything.
Stable anchors consume metadata: future observation/retention budgets must count
them; broad lock-file deletion would reintroduce inode/handle ABA races.

## Capacity-domain model checkpoint

One authoritative in-memory state owns a domain/epoch. Place aliases must share
that state; independent domains do not share a mutex. The model uses &mut
transitions, bounded records, checked arithmetic and private scope/generation
tokens. It is not a multiprocess coordinator, filesystem measurement, physical
reservation or restart-persistent ledger. Trusted adapters must journal/fence
these transitions before real allocation can depend on them.

```text
reserve peak -> begin bounded write -> physical IO -> confirm actual allocation
     |                 |                                  |
remaining held     still held                      remaining becomes debit
                                                          |
capture allocation watermark BEFORE free-space sample -> reconcile observation
```

Available = max(0, sampled free - floor - remaining reservations - confirmed
allocations newer than the sample's captured watermark). IO remains outside
short coordination transitions. Allocations confirmed after the captured
watermark remain charged even if a racing sample also saw them: conservative
double counting is temporary; a gap that charges neither is forbidden.
Never sample first then attach a later watermark.

In-flight writes block reservation close. Completing with zero actual bytes is
valid only after admitted proof that no allocation remains. Unknown/dead workers
do not expire automatically. Completion events are idempotent; reused IDs with
different limits/results reject. Closed reservations retain bounded tombstones.
Observation tickets are scoped to domain/epoch and latest probe; invalidation
blocks new admission. New epochs cannot inherit old tokens. Freshness/clock,
provider visibility, durable replay, quotas and multi-domain admission remain
future work. Planned reclaim never changes sampled free space.

## Executed scope

18 core capacity tests exercise alias contention with one shared authority,
independent-domain progress, conservation/racing samples, in-flight close,
idempotency, epoch/ticket rejection and u64/bounded-history limits. History caps
are 1,024 reservation records including closed tombstones and 4,096 events.
Reaching a cap refuses work; it does not silently discard recovery history.
These limits and sample values are not measured capacity of any real volume.

12 local fence entries (11 integration plus one release unit) include one
subprocess helper. Real temporary files
and two coordinated child processes test shared readers, exclusive exclusion,
unrelated publication, process-exit release and stable anchors. Earlier 85
entries still pass; the later journal checkpoint brings the workspace to 154.
This component checkpoint does not prove real removal, claim renewal/expiry, hostile
namespace race, network filesystem, durable resource transaction or crash replay
of reservations is verified. Publication still acknowledges process restart only.

L0: structure/VTRACE/fmt/whitespace. L1: locked workspace tests and all-target
Clippy with warnings denied. L2: public API tests plus real temporary-store child
processes. Commands: `cargo test -p tig-core --test capacity --locked` (18),
`cargo test -p tig-store --test fences --locked` (11),
`cargo test --workspace --locked` (154). EVID-012/013 record this scope;
WP-004/006 remain in progress with full requirements/VAL runs open.

The later [reservation journal](RESERVATION-JOURNAL.md) adds restart-persistent
transactional coordination under EVID-014. Physical space measurement/enforcement,
durable witness recovery and full claim/custody retirement are still open.

Guard-drop follow-up: explicitly unlock before closing the owned handle. A fork
or supervisor can temporarily duplicate an OS handle; close alone can delay
release. Independent acquired guards remain protected. The unit regression keeps
a duplicate alive while dropping each shared/exclusive guard. Process death still
uses OS cleanup; stable anchors are never unlinked to resolve contention.

The later [accounted publication](ACCOUNTED-PUBLICATION.md) component connects
job reservations to bounded local attempts and labelled measured file-data charges
under EVID-015. It does not close whole-volume enforcement, restore or removal.

[Replica verification](REPLICA-VERIFICATION.md) adds exact completed-publication
sessions that retain a shared object guard across their lifetime. Expiry rejects
coverage without releasing the guard; process death still releases OS handles.
This is scoped live integrity evidence, not a durable retention promise.
