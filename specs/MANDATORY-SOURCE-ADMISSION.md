# Mandatory scoped source admission

Input 355aad7fc6ab964527b46492cdb6f98a395f58c1;
roadmap review adopted in 85ac433. WP-004/006/008/009;
REQ-001/002/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009; IF-002/004/005/006/008/012;
PKG-002/003, PKG-009 source-budget control. RUST/PROVIDER profiles.
Allowed: an explicitly created guarded source-journal epoch, fixed placement
policy, enrolled source classes, object-scoped admission and subject assessment;
existing Tree/NodeStore/source-journal ports; synthetic temporary roots only.
Entry: exact-root demand projection, journal replay/revision fences and local OS
locking primitives. Exit: direct mutation cannot bypass scoped admission; related
claim changes conflict with a held subject fence while unrelated changes proceed;
restart and old receipt recovery preserve exact outcomes. No payload removal.

## Managed namespace contract

A new source-claims design-2 marker contains immutable class/placement policy.
All four branch/draft/job/retention classes are explicit, with assigned namespace-
global place keys for current/pending/predecessor roots. Source Enroll records
owner and immutable class before its first Prepare. It uses the existing journal
command/replay protocol and limits. The namespace uses one authority journal for
all its admitted sources; it cannot claim coverage of other namespaces/clients.
Explicit prefer_active policy suppresses inactive predecessor proximity when an
active branch/draft/job demand exists anywhere in the namespace. Retention-class
demands remain additive. Changed raw active/history/retained buckets are fenced,
including role changes that could affect precedence even if object bytes match.

Legacy design-1 encoding, genesis and command bodies stay unchanged. New readers
continue to read/write legacy journals through their existing API. Older readers
reject the new marker/Enroll command. Existing roots are never migrated, reset or
adopted implicitly. New generic SourceJournal mutations/previews refuse guarded
roots; only the guarded namespace compiler can supply its nonserializable permit.
Inspection/acknowledgment is still read-only and does not mint a mutation permit.

Guarded mutations resolve the exact before/after flat binding roots and lifecycle
placements outside SQL transactions. Objects whose raw active/history/retained
demand buckets change acquire shared admission fences before commit, even when
their currently effective placement union stays equal. Expected source
revision and exact command/operation identity reject a stale compiler result.
A replayed command remains a historical receipt, never a fresh admission grant.
The compiler does not interpret opaque native contexts; adapters must first admit
an exact flat byte-binding root, not mistake a selected manifest for its payloads.

The epoch precreates 256 fixed zero-byte lock anchors, selected by the first
SHA-256 byte. Shared claim admission is compatible; an exclusive subject fence
blocks changes in its partition. Partitions can cause conservative contention,
not a cache-wide lock. All acquisition is ordered/nonblocking; no anchor is
unlinked to clear a lock, and guards cannot be serialized or cloned. Explicit
unlock on drop handles duplicate/inherited OS handles. Process exit releases
locks; no timeout is used as deletion or claim-withdrawal authority.

Subject assessment holds its exclusive fence and checks every registered source's
root demand under the immutable policy. It can establish scoped absence for one
object/place while unrelated journal heads change: all mutations that could change
that predicate must first take the conflicting shared fence. A global snapshot
head is not held or required unchanged through metadata IO. Missing/corrupt roots,
foreign authority, invalid sizes, limits and known rollback refuse assessment.
An absence fact borrows the live fence. It is not provider removal authority,
independent custody, protection outside this namespace, or hostile-filesystem/
full restore proof. Existing unfenced journals remain outside this mechanism.

Creating another epoch does not transfer source obligations or fence old removal
executors. Store-scope binding to all applicable namespaces and acknowledged epoch
handoff remain separate prerequisites before enabling any removal. The existing
read-only protection collector rejects a profile class conflicting with a guarded
source's recorded class; legacy sources retain caller-admitted classification.
That collector's additive placement projection does not apply prefer_active and
must not substitute for this namespace's policy assessment.

Owned local metadata, cooperative admitted clients and trusted minimum witnesses
are required. Caller identity strings are not authentication. Source/provider
adoption must independently establish unmanaged-client boundaries. Policy changes
require a new explicitly enrolled epoch, not marker replacement. Replaced markers
refuse. No automatic daemon, Git hook, cache scan or consumer upgrade is installed.

Closure work retains the projection's finite root/binding/read/byte/time budgets;
placement associations count work, not physical copies. Locks span only journal
commit, not tree traversal on the admission path. Subject assessment may hold its
partition through metadata reads; unrelated partitions remain available. Journal
completion itself is not preempted by a cooperative deadline. Bootstrap metadata
has a finite 256-anchor footprint; whole-volume accounting remains separate.

L0: validate/diff/fmt and six producer lenses. L1: locked default/all-feature suites,
strict Clippy, legacy encoding/CLI regressions and exact focused admission cases.
L2: actual temporary journals/locks, direct bypass refusal, duplicate/stale/foreign
requests, overlapping versus unrelated admissions, role changes, missing metadata,
marker replacement, actual child exits and restart/receipt recovery. No physical
retirement path is enabled until retained custody, store binding and authorized-
absence/incarnation protocols have their separate implementation evidence.
