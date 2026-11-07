# Experimental identity and persistent trees

WP-001 implementation checkpoint. Input: main
1bbfcc6cb3c94388bcd90795bdf99637d65dd8f4. Parents: REQ-001/005/006,
SPEC-002, ARCH-001, IF-002, DES-001, PKG-001, PROFILE-002 (RUST).
Scope: pure Rust, synthetic in-memory metadata only. No runtime schema release,
provider selection, payload store, materialization, acknowledgement or deletion.

Entry: existing 13 handoff tests, separate raw and semantic hash contracts, Art
donor pinned at 119c66446f2e95129d8d8f143d60ec6220d2813a. Exit:
canonical/raw vectors, path/size negatives, bounded lookup, reproducible tree
roots and measured sparse metadata edits. Full requirement acceptance stays open.

## One identity implementation

`Object` pairs exact raw SHA-256 with a u64 byte length. `Digest::blob` hashes
unchanged bytes, matching standard Git LFS SHA-256 identity; it does not add a
semantic prefix. Metadata JSON is the strict Art prototype subset: duplicate
keys reject, UTF-8 key order, array order preserved, no Unicode normalization,
safe integers only, no floats/exponents/negative zero. Budgets are 65,536 bytes,
4,096 JSON value nodes and depth 32. This is not an RFC 8785 implementation.

The copied Art canonicalizer retains its `ART\0prototype-1\0` semantic frame
and its known vectors intentionally. Renaming the product does not silently
change identities. Semantic framing and raw payload identities stay distinct.
The schema-bearing tree nodes below use raw SHA-256 of their canonical bytes.

`LogicalPath` is exact UTF-8, at most 1,024 bytes and 32 components. Empty,
absolute, dot, parent, backslash, drive/stream colon, control, wildcard, reserved
`.git`/`.tig`, Windows device stems and trailing-dot/space components reject.
Case and Unicode normalization remain distinct logical names. Provider-specific
case/normalization/device collisions must still reject before materialization;
this logical parser alone is not a safe filesystem extraction implementation.

## Tree representation (design-1, not released)

```text
Tree root -> directory {schema, root: optional entry-map link}
                                  |
                         {object: hash+size, entries}
                                  |
                     entry-map {schema, name, value, left, right}
                                  |
                    file object OR child directory object
```

Schemas are `tig.directory.design-1` and `tig.entry-map.design-1`. An entry-map
is a persistent binary search tree ordered by exact name. Its max-heap priority
is `(SHA256(name UTF-8), name)`; the name breaks hash ties. This deterministic
treap gives the same root for the same bindings regardless of edit order.
Adversarial names are bounded by explicit depth/work limits, not a promise of
worst-case logarithmic cost. Do not treat this representation as a torrent ID.

Construction builds reachable nodes once. Edits create immutable nodes only on
affected paths; unchanged directories/subtrees stay shared. Removing a leaf
prunes empty logical directories. It never deletes stored bytes. Restore can
recover the original root. File/directory replacement conflicts are explicit.

Default per-operation budgets: 65,536 reads, writes and map entries; 32 MiB read
and write metadata bytes; entry-map depth 64. Full enumeration also bounds total
returned file bindings to 65,536. Sparse edits bound their change count and each
touched map; they do not assert the entire snapshot is within enumeration limits.

`NodeStore` is an explicit metadata get/put port. `MemoryNodes` is a synthetic
implementation. Every read checks declared length, raw hash, strict canonical
bytes, exact typed shape and traversed node constraints. Writes verify the
returned identity. A real provider must enforce bounded allocation inside its
port implementation too; returning a Vec is not a provider memory sandbox.

`Tree::from_root` admits an unverified pin. `get` verifies its traversed route,
not siblings or payloads. `entries` verifies the bounded reachable metadata
closure, sorted/heap/count invariants and conflicting payload size pins; it
still does not verify payload bytes or custody. Sparse edits need not discover
unvisited malformed subtrees or conflicting size pins elsewhere. Complete
adoption must require the corresponding full validation/verified-root authority.
An empty edit preserves a pin and is not a validation operation.

Missing/corrupt/unsupported nodes fail explicitly. Budget or store failure may
leave unreferenced immutable metadata; no new root is returned/adopted and the
old root stays readable. Durable orphan accounting, crash recovery, protection
and admission belong to WP-003/004/007/008, not this in-memory API. Expected-root
checks compare this immutable view; they are not a concurrent shared-ref fence.

## Verification scope

L0: repository/link/role/trace checks, fmt and whitespace. L1: 10 canonical
vectors, 16 tree tests, 13 existing handoff regressions; all-target Clippy with
warnings denied. L2: external Rust tests call the public API through in-memory
and fault-injected ports, compare 120 deterministic edits against a reference
map and bulk reconstruction, and preserve old snapshots through replacements.
No IO/provider acceptance is implied.

Two independently composed .NET SHA-256 wire vectors pin empty and one-file
roots. Synthetic 1,000-binding tests measure metadata work, not media IO:

| Edit | Reads | Writes | Read bytes | Write bytes |
| --- | ---: | ---: | ---: | ---: |
| One replacement | 40 | 15 | 15,286 | 5,460 |
| Five replacements | 170 | 65 | 61,239 | 22,223 |

The tests enforce upper bounds rather than these incidental exact counts.
Only 15 new stored metadata nodes appear for the single replacement; unchanged
payloads are never read/copied by this module. Initial bulk construction writes
1,003 reachable nodes with zero reads. No physical disk savings or production
throughput has been measured.

Run `cargo test --workspace --locked`; use `--test tree -- --nocapture` to see
the sparse-edit metrics. Source provenance is recorded in [NOTICE](../NOTICE.md).

EVID-029 now adds persistent synthetic scale measurements in
[PERSISTENT-METADATA](PERSISTENT-METADATA.md). Declared inventory bounds remain
explicit for lookup/edit; exceeding them now returns Budget rather than labeling
otherwise valid nodes corrupt. Wire encodings and all historical hash vectors
remain unchanged. Native media throughput/custody are still unproven.
