# Bounded asset revision history

T1a of [the landing plan](../docs/VERSION-DELIVERY-LANDING.md), input
`c44acdd90907a7c4fff2dd37909956f6b584ee72`. WP-010/011/012/015,
REQ-001/006/010, SPEC-010/011/012/014/015, IF-010/011/013, PKG-006/008.
RUST/INTEGRATION profiles. Allowed: pure optional `tig-assets::revisions`, synthetic
integration tests, this contract and bounded review evidence. No CLI/provider,
claim adoption, delivery implementation or consumer project record conversion in T1a.

Entry: existing exact native/schema binding, produced-claim/context contracts and
the reviewed landing plan. Exit: optional common revision attachment validated
over synthetic image/audio/document assets, bounded append/union DAG traversal,
selected-byte agreement, same-byte distinct runs, strict malformed/conflict/budget
negatives and retained revision lookup. T1 delivery events/target histories remain
the next slice; T1 is not complete merely because T1a passes.

## Wire contracts

All documents use the existing strict native JSON subset: 64 KiB, 4,096 nodes,
depth32, safe integers, no floats or duplicate/unknown fields. Exact raw bytes
determine object identity; parsing never rewrites them. No schema defaults.

An optional `metadata.revisions` has schema
`tig.revision-state.experimental-1`, required `history_slot`, optional
`selection: {slot, representation}`. The named history stream has role
`history/revisions`; the optional selection stream has role `revision/selected`.
Both are ordinary bounded instance streams. Absence of the attachment is valid
legacy/planned state, with history unknown rather than asserted empty. A record
without selected bytes must not attach a revision selection. A selected legacy
output can have history with no classified selected revision.

A history page has schema `tig.revision-history.experimental-1`, the exact asset
ID in `asset`, `coverage` (`complete` or `partial`), `parents` (0..16 exact history
page Objects) and `revisions` (0..64 exact revision-manifest Objects). Append a
page pointing to the previous root; merge by a new page pointing to both roots.
No ledger rewrite or second mutable inventory is needed. Repeated references
within one list reject; sharing between pages is normal and read once per check.
Complete means declared revision-list coverage only, never storage/claim coverage.
Any partial page keeps the combined report partial. Missing/corrupt objects fail;
they do not produce an empty history or a usable removal decision.

A revision manifest has schema `tig.asset-revision.experimental-1`, exact `asset`,
stable `revision` run/import key, `parents` (0..16 revision-manifest Objects),
`representations` (1..256 unique `{key, object}` entries) and `provenance`:

```text
{kind: imported, source: Object}
or
{kind: produced, claim: Object, context: Object}
```

The produced claim/context reuse existing contracts; these are retained references,
not another recipe/dependency list or verified producer assertions. Imported
history may retain unknown correspondence explicitly. Two run keys can reference
identical payload bytes without becoming one revision. The same run key cannot
identify two different manifest Objects within the resolved history. Parent
manifests belong to the same asset and are included in traversal. Both page and
revision ancestry must be acyclic. Selection must resolve to a manifest in this
history and its explicit representation must exactly equal `instance.selected`.
No project status/approval interpretation, selection advancement or staleness
claim follows. Domain/schema validation and recipe checking remain separate.

## API and bounds

The caller supplies an exact `BoundInstance`, its successful authored-schema
`Checked`, admitted limits and a metadata-only reader. A mismatched Checked
rejects. The reader receives the expected Object and per-object maximum; it must
enforce bounded fetching itself. The checker rechecks bytes, hash, size and strict
JSON before parsing. Pins share a global digest/length consistency table with
the instance/definition/schema. No filesystem/network discovery is performed.

Hard traversal ceilings: 4,096 distinct pages/manifests, 64 MiB cumulative metadata
bytes and 65,536 references. Callers may tighten, never broaden them. Admission
limits and expected pin size are checked before reader calls. Iterative traversal
and cycle checks avoid attacker-controlled recursion. Payload/context/claim bytes
are not fetched; their references are exposed separately from checked metadata.
Opaque context/claim closures still require existing admitted adapters before any
retention/removal decision. Selected history does not pin all predecessors hot.

Reports separate verified metadata documents, payload Objects and provenance
references; expose partial/complete declared history and optional exact selection.
They grant no production proof, creative approval, independent custody, complete
reference/semantic coverage or eviction authority. Reports omit authored text,
asset/run IDs and raw reader/parser errors; explicit lookup by caller-known run
key is available without emitting an inventory by default.

L0: repository validator, diff check, fmt. L1: named revisions tests, locked
all-feature workspace tests/strict Clippy, standalone Git/LFS check. L2 for this
pure slice: synthetic reader integration across three asset shapes and 1,000
versions; real CLI integration belongs to T2. No live user-store test or new
producer execution is necessary or claimed.
