# Shared cache lookup, ingest and Git LFS integration

Status: target contract with a bounded download-only adapter subset implemented.
See [executed compatibility scope](LFS-CACHE-INTEROP.md); full index/lifecycle and
upload support remain open.
Tig must be useful to a Git/LFS project without typed assets. Art's value is now
planned as an optional Tig layer under docs/UNIFIED-DESIGN.md, not a prerequisite
for object lookup, placement or cache management. References to the prior Art
client remain relevant to legacy compatibility, not a mandatory separate product.

## One access layer, multiple places

```text
Git LFS pointer / exact Tig bundle path / direct object request
                            |
                  object identity + expected size
                            |
                  shared local location catalog
                            |
          eligible verified copy -> protected readable handle
                            |
        otherwise: bounded acquire -> verify -> publish -> handle
```

Use object-to-replica indexes, bundle/path indexes and claim-to-object indexes.
Cache immutable manifest nodes and negative observations under explicit scopes,
generations and expiry. An offline or unqueried place is unknown, not absent.
Location preference is configurable; never hard-code drive letters. Expose warm
lookup, stale-hit revalidation, cold metadata resolution and remote acquisition
separately so latency/cost is understandable. Ordinary lookup must not recursively
scan caches or load every bundle. Integrity guarantees and evidence freshness
must be explicit; a cached filename/size cannot establish verified object bytes.

An embedded transactional local catalog is a candidate, not a selected backend.
It is derived operational state outside Git, with bounded rebuild/import from
immutable manifests, trusted provenance and provider observations. Support
multiple processes/worktrees without a global cache/render reservation. Explicit
bounded recovery may enumerate a named provider scope; ordinary lookup may not.

## Creation and replacement are incremental operations

Write through Tig's ingest/publication API where possible. Stream/hash under a
bounded reservation, verify expected identity/size when supplied, then promote
immutably and publish a durable replica observation. Choose the configured final
place up front; stage on that destination when supported rather than copying a
whole output through a default hot cache first. Never overwrite an existing
verified immutable object with different bytes. Deduplicate concurrent publication
of identical bytes; interrupted ingest remains partial and cannot be returned as
complete. Producer completion alone does not establish external retained custody.

Replacing a logical path creates a new immutable bundle snapshot. The prior
object remains unchanged; update only affected path/claim/index partitions. A
successful catalog update must not silently lose a completed physical object if
the process crashes: recover through a durable scoped operation record and
provider revalidation. Storage and catalog do not form one universal transaction.

Events from Tig-managed writes update the catalog immediately. Filesystem
watchers/provider events are optional hints for externally modified caches;
duplicates, gaps, overflow and disconnects require explicit stale/unknown scope
and bounded reconciliation. Cache generation changes invalidate negative results.
Externally mutable files need provider-specific checks before trusted reads;
path reuse must not return the old object's identity for new bytes.

Return a protected handle or materialization with an explicit active-use claim.
Reads and eviction must coordinate at object/place scope. Expiry/renewal must not
permit deletion while a valid reader still uses the copy. No universal guarantee
may rely on an open file preventing removal on every operating system.

## Standard LFS compatibility

Read existing standard Git LFS pointers; do not invent a Tig pointer dialect.
Supported plain SHA-256 pointer OID and byte size identify the object. The OID
matches Tig's plain SHA-256 blob identity for those exact bytes, not a manifest
hash, torrent root or transformed representation. Reject malformed or unsupported
extension/transform semantics explicitly rather than silently misinterpret them.

Two complementary entry points are planned:

1. Read pointer files from an explicitly selected Git tree/index to derive exact
   object inventory and admitted branch/worktree retention claims. A branch name
   must be sealed to a commit; staged candidates have their own exact scope.
2. Implement an explicitly configured Git LFS custom transfer agent backed by
   Tig lookup/ingest. Speak the upstream init/transfer/progress/complete protocol;
   validate identities and bound concurrency. An upload completion means the
   configured destination contract is satisfied, not that GitHub cloud storage
   received the object. Repository config cannot select arbitrary executables
   or locations without trusted local configuration.

The existing Git LFS clean/smudge client remains responsible for its standard
pointer/worktree workflow. A transfer agent does not automatically eliminate the
LFS local object store or hydrated worktree copies. Model these as distinct places
or materializations, account for their physical bytes, and prove supported reuse
before promising zero-copy. Thin/pointer-only worktrees are an explicit mode.
Do not alter shared LFS settings or install hooks during repository startup.

Remote LFS servers and local disks are both providers with different capabilities.
An adapter may transfer into a cache, satisfy an LFS download, or upload to a
configured endpoint. Local cache eviction and remote LFS deletion are separate;
remote deletion may be unsupported. Fetching or checkout alone must not imply
permission to mirror the full history or evict unrelated consumers.

Sources: [LFS pointer specification](https://github.com/git-lfs/git-lfs/blob/main/docs/spec.md)
and [custom transfer protocol](https://github.com/git-lfs/git-lfs/blob/main/docs/custom-transfers.md).
Pin implementation fixtures to an exact upstream revision/client version before
claiming conformance; these design links are moving documentation references.

## Additional designed acceptance cases

These ten cases supplement T01-T25; none has been executed.

| ID | Trigger | Required outcome |
| --- | --- | --- |
| CL01 | Warm object lookup with many bundles/places | Bounded indexed lookup; measure reads, latency and memory; no census |
| CL02 | Cached miss followed by another writer publishing the object | Generation-aware invalidation; new verified copy becomes discoverable |
| CL03 | Watched external cache disconnects or overflows event queue | Mark affected scope stale/unknown; bounded recovery, no false absence |
| CL04 | Two worktrees ingest identical bytes concurrently | One immutable publication per place; both get accurate observations |
| CL05 | Crash between physical publication and catalog update | Recover completed bytes without false completion or orphan data loss |
| CL06 | Reader acquires object while eviction is planned | Fresh active-use protection fences removal at object/place scope |
| CL07 | Plain pointer, malformed OID, wrong size or unsupported extension | Correct exact identity or explicit rejection; no arbitrary path lookup |
| CL08 | LFS agent upload/download retries and parallel requests | Protocol-correct idempotent results; bounded concurrency and truthful completion |
| CL09 | LFS local store plus hydrated worktree plus Tig cache | Measure all allocated copies; do not claim logical dedup means zero physical cost |
| CL10 | Generic Git-only repo replaces an LFS file and merges | Tig derives old/new object claims without Art, preserves other users and plans placement |

Open measurements: warm/cold lookup percentile latency, catalog contention,
number of metadata/object reads, recovery cost, write amplification, duplicate
physical bytes and bounded scratch. Set numerical budgets from fixtures and
declared workloads; this document makes no benchmark claim.
