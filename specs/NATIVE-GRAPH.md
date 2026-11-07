# Exact native declared-reference graph

The [explicit graph CLI](../docs/guides/GRAPH-INSPECTION.md) decodes caller-owned
profiles using strict Deserialize on the existing admitted types. Existing
Serialize spelling and admission identity are unchanged. It reuses this resolver,
production requirements and scoped primary comparison over exact local Git
snapshots. Private bounded reports preserve gaps and null unavailable comparison;
no payload access, authored-schema execution, source change or eviction follows.

Map-reference extension entry: 85e49381a52b52ee44f9a2c875bc967de95923ab.
WP-010/011 and WP-005 source seam; REQ-001/006/008/010, SPEC-007/010/011,
ARCH-004/005, IF-008/010/013, DES-009, PKG-004/006. RUST/GIT profiles:
L0 structural/fmt/Clippy/canonical VTRACE; L1 deterministic map traversal,
shape/reference/key/budget negatives and unchanged single/array behavior;
L2 synthetic non-media reuse plus real temporary Git scene/leaf revisions.
Allowed scope: pure graph extraction, source integration tests and bounded
control docs. No cache/store changes, private fixtures, new IO or consumer
runtime switch. Exit requires map-based primary change comparison with unchanged
parent records, frozen-pin behavior and output/coverage enforcement preserved.

The experimental Rust Rule API uses Collection::One, Array or ObjectValues
instead of the earlier many boolean. A mode never guesses another shape.
ObjectValues reads only the directly selected object's values, sorted by UTF-8
key bytes, and appends the escaped key to each RFC 6901 edge pointer. Empty
structural keys are valid; control characters, keys over 256 UTF-8 bytes or
derived pointers over 1,024 bytes refuse. Keys are not paths or logical targets.
Each value must independently satisfy the admitted exact/snapshot binding mode.
The admission digest includes the collection mode. Map keys appear in structural
pointers; graph output is not a sanitized public receipt. No recursive scanning,
duplicated authored dependency array, producer fingerprint or eviction authority
is introduced. Full WP-010/011 and native adoption remain open.

Entry: Tig 562837485a2131e4d2209b7dce98fb39a8bb248c. WP-010 native graph
slice and WP-011 extraction prerequisite; REQ-001/006/010, SPEC-010/011,
ARCH-005, IF-010, DES-009, PKG-006, CR-001..008. RUST profile: L0 structural,
fmt, Clippy and canonical VTRACE; L1 positive/negative bounded graph tests;
L2 synthetic scene and non-media consumers through the public native API.
Only tig-assets pure code, tests and scoped control docs may change. No IO,
cache setup, private fixtures, producer jobs or live lane changes in this package.

A trusted caller admits an exact definition pin with extraction rules. Rules
select explicit RFC 6901 pointers relative to authored fields, each one instance
reference, an ordered array, or explicit object-map values. Profiles declare coverage by edge kind
and classify instances as primary, output or container. This admission is caller
policy, never authority granted by a record's own status or untrusted definition.
Frozen references include logical ID, exact raw SHA-256 and file size. Authoring
references can contain only a logical ID, selected within the exact source view.
There is no
latest-version fallback, directory search, duplicate authored dependency list or
implicit scanning of evidence/metadata for dependencies. Generic profiles do not
know episodes, language names, golden status, file extensions or drive letters.

Rules explicitly permit exact-only, snapshot-selected-only or either form. Their
values contain exactly id/sha256/bytes or exactly id; partial pins and unsupported
extensions reject. Snapshot selection checks both ID and checkpoint, caches each
ID once per query and preserves missing/denied/unsupported/ambiguous results. It
cannot fall back to a different view or an old byte pin. Concrete source adapters
must qualify view membership; equality of a caller-trusted checkpoint is not a
cryptographic proof of provider honesty. The admitted profile set receives a deterministic digest.
Changing classification/extraction requires a new policy comparison; ordinary
payload comparison requires identical admission digests. Profile ordering alone
does not change that identity. At most 1,024 profiles/128 rules each are allowed,
and the aggregate fingerprint also obeys strict 64 KiB/4,096-node JSON limits.

Queries bind an immutable source checkpoint, exact roots, requested edge kinds
and finite node/edge/byte/depth limits. The source must honor the requested byte
budget and return instance, definition and authored-schema bytes from that view.
The engine verifies raw pins and checkpoint itself. Missing, unavailable, denied,
unsupported, corrupt, unadmitted, uncovered and resource-limited results are gaps,
not empty dependencies. Results omit authored values, status and provenance;
IDs and admitted structural map keys remain names in graph results.
Parsed-file limits remain those of NATIVE-INTAKE.md. Breadth-first traversal counts
depth by first discovery; shared targets are loaded once per exact revision.
Hard query ceilings are 500,000 nodes, 2,000,000 edges, depth 128 and 256 MiB
of returned metadata. The source callback is trusted to bound its own IO and
allocation; returned bytes are checked/counted even on refused data/checkpoints.
This logical response counter is not measured physical IO or capacity admission.
Selection-reference metadata also consumes this budget; edge attempts are bounded
even when a selection cannot resolve. Production/composition cycles are gaps;
evidence/authority/publication relationship cycles are permitted.

Production edges may traverse containers and primary instances, but an output
target is an explicit gap. Composition, evidence, authority and publication edges
are distinct; projects may admit additional stricter policy later. Retained source
inventories can declare evidence-only coverage; a production query then remains
incomplete, even when every candidate byte has previously been verified.

Payload meaning is explicit: selected-object, streams matching declared roles/
groups with minimum cardinality, no external payload, or unsupported. A selected
result manifest is never implicitly expanded or mistaken for every media input.
Unknown payload meaning/missing required selections blocks payload comparison.

Primary-payload comparison accepts only complete declared Production queries and
returns added/shared/unreferenced-by-new-query primary objects. Shared uses are
deduplicated. These are scoped candidates, never global ref counts or deletion
authority: other branches, jobs, drafts, histories and custody remain separate.
Complete declared coverage does not prove full authored-schema/producer semantics,
freshness, approval, delivery, retained custody or complete project/job coverage.
Art graph concepts informed this design; no legacy model/graph parity is claimed.
This slice extracts from authored fields only. Opaque native documents/stream
manifests need separately qualified adapters that read their single declarative
reference authority; do not copy their dependencies into a second authored list.

Exit: positive two-cel/shared-cue revision comparison, non-media reuse, exact pin/
checkpoint and primary/output enforcement, coverage and bounded-failure vectors.
Full WP-010/011, provider integration and native consumer cutover remain OPEN.

Current execution: 40 graph cases include an actual 1,011-node/1,010-edge synthetic
hierarchy with 1,000 primary payloads and one lookup per revision. This does not
claim 500,000 native graph scale, private primary-input adoption or custody.

## Small authoring changes and frozen builds

```text
scene.tig: deps=[{id:cel/b}]       scene file stays unchanged
                   |
Git snapshot A: cel/b -> B1        binding digest A
Git snapshot B: cel/b -> B2        binding digest B
frozen reference: cel/b + B1 pin   remains B1 in either snapshot
```

Only the cel leaf needs editing. The graph materializes exact dependency pins
from the chosen view, so a parent file hash alone is not a build identity.
A binding digest covers admitted interpretation, requested kinds, ordered roots,
resolved nodes and edges. It uses SHA-256 with TIG-NUL-native-binding-NUL-
experimental-1-NUL framing, length-prefixed canonical items and collection counts.
Items are admission digest, sorted requested kinds, sorted roots, nodes by exact
reference, and breadth-first resolved edges in rule/array or UTF-8 map-key order; each collection
has its own unsigned 64-bit big-endian count and each item a byte-length prefix.
The source checkpoint is recorded separately: an unrelated Git commit leaves
the binding digest unchanged. Missing coverage yields no digest. This full-source
binding identity includes metadata edits; it is not the future consumed-property
recipe fingerprint and does not prove output freshness or human approval.

The 1,001-node/1,000-edge non-media map case separately proves bounded object
value traversal. Maps may be empty when the admitted project schema permits it;
an array, null, nested non-reference or malformed target cannot stand in for a
map. Native producer schemas may impose stricter cardinality. This experimental
Rule API change leaves native instance/definition envelope schemas unchanged.

## Complete declared production object requirements

The additive Named selector and Graph::production_requirements API are controlled
by [PRODUCTION-REQUIREMENTS](PRODUCTION-REQUIREMENTS.md), entry 5a35f5a. The query
includes admitted container controls and raw interpretation documents alongside
primary payloads. It refuses unknown/missing payload meaning and output bytes as
build inputs, including output roots. Existing primary_change remains a primary-
only candidate comparison; existing binding framing and native envelopes remain
unchanged. Full opaque producer semantics and all storage authority remain open.
