# Bounded staged tree edits

Input3586d821487b4bfd0df8d86698e9a432a498dc97. This prerequisite supports
JOURNAL-ACTIVATION global indexes; it does not implement a successor journal.
Controls: WP-001/003/006/007/015; REQ-001/003/004/007/009;
SPEC-002/003/004/008/014; IF-002/003/005/013; PKG-001/002/003/009.
RUST/PROVIDER/INTEGRATION profiles: L0 source pins/canonical format/bounds,
L1 pure vectors/resource/adversarial ports and locked tests/strict Clippy,
L2 fresh physical metadata finish/reopen/readback and admitted larger deltas.

Entry: existing immutable Tree/NodeStore, exact expected root, caller-admitted
TreeLimits, owned synthetic fixtures. Allowed code: tig-core tree staging and
its tests, physical metadata adopter fixture, linked contract/evidence/context.
No journal/control/cap change, source initialization, payload IO or expiry.
Exit: same canonical result as Tree::edit, shared whole-operation limits, only
reachable newly staged metadata persisted, exact provider acknowledgements,
explicit failure semantics and physical readback. Implementation, verification,
scoped validation, trace and six producer findings close separately.

## Contract

Tree::edit_staged accepts the same inputs as edit and returns a Tree plus
StagedEditMetrics { tree: TreeMetrics, published_nodes, published_bytes }.
It runs the existing edit algorithm against a bounded in-memory NodeStore overlay.
Existing TreeLimits meter every logical read/write, including intermediate nodes,
with no reset per path and no larger defaults. Unique staged bytes/nodes are
bounded by those cumulative write limits. No provider put occurs until every
edit succeeds. Reads check staged content first and otherwise use the existing
provider. No payload object is read or written by this operation.

After staging succeeds, walk only staged directory/map nodes reachable from the
final root. Stop at unchanged provider-owned nodes; do not enumerate the base tree.
Follow directory roots, map left/right and directory-valued entries; file-valued
objects are opaque, even if their bytes resemble a staged metadata object. Flush
each reachable staged node once, children before parents. Validate each put's
exact Object acknowledgement. The final root matches ordinary sequential edit;
the existing design-1 directory/map encoding and stable identities do not change.

tree metrics describe logical work; published metrics describe provider put
calls/bytes, including content already present at that provider. They are not
new physical allocation, retained custody, finish acknowledgement or reclaim.
Traversal bookkeeping is bounded by staged nodes/bytes, with no base-node cache
or full-history map. No new dependencies, cache engine, lock or hidden IO.

Staging refusal leaves the provider without new puts. A provider failure during
flush may leave immutable children present; no root is returned and no adoption
is implied. Caller handles physical admission, finish/readback and atomic root
adoption. A successful put sequence alone is not durability or publication.

## Qualification

Compare canonical roots with sequential edit and independent Tree::build across
insertion/replacement/removal/nested paths and deterministic shuffled input.
Check exact reachable node publication, unchanged subtree reuse without scanning,
no-op, stale root/path conflict/inconsistent lengths and aggregate budget refusal
before provider puts. Exercise missing/corrupt base nodes, failed put and wrong
acknowledgement without fabricated success. Verify file values stay opaque.
Measure actual16384-entry append onto a nonempty index using explicit admitted
larger logical limits; do not raise normal defaults or old journal caps. Reopen
an existing physical MetadataStore after finish and inspect the exact resulting
tree. Full lifetime activation/old-writer/minimum/restore gates remain OPEN.

Producer lenses: identity requires canonical equivalence; placement forbids
inferring authority from index edits; custody distinguishes puts from durability;
concurrency requires exact expected root and explicit adoption outside the API;
provider/privacy requires bounded metadata-only ports; Git/adoption keeps runtime
nodes and measurements out of source. These are checklists, not independent review.
