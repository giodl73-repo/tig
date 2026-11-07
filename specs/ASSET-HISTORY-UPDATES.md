# Prepare explicit asset history changes

T2 mutation preparation of [the landing plan](../docs/VERSION-DELIVERY-LANDING.md),
input `27846e51261e871502b5848c3fd245147dc18baf`. WP-009/010/011/012/015,
REQ-001/004/006/009/010, SPEC-007/010/011/012/014/015, IF-007/008/010/011/013,
PKG-004/005/006/008. RUST/GIT/INTEGRATION profiles. Allowed: pure tig-assets update
preparation over existing contracts, focused tests and bounded review/specs.
No IO writers, cache initialization, provider engine, automatic Git operation,
approval inference or live consumer adoption. Existing sparse Git proposals and
metadata publication seams remain the later explicit handoff, not duplicated here.

Entry: exact instance/definition/schema binding, common revision/delivery readers
and expected-source semantics. Exit: prepare bounded new immutable metadata and
one native record; preserve old history/identity/authoring; reject stale input,
conflicts, unclassified replacement and invalid candidates; deterministic retry
and exact repeat no-op. Full T2 CLI/publication/proposal handoff and T3 ownership
remain open; a preparation result is not retained custody or a committed change.

## Operations

Every operation takes an exact expected raw record Object and explicit pinned
definition/schema bytes/dialect plus reader limits. Stale raw bytes refuse before
schema/metadata IO. Validate source, prepare the edit in memory, then execute the
candidate's authored schema and common history contracts. Preserve the stable ID,
definition, authored data, unrelated metadata/streams and project fields. Existing
history stream labels are used literally; fresh reserved slots must not overwrite
unowned streams, even when roles happen to match. Existing pin annotations stay
intact when updating a history pointer.

- Configure delivery: explicitly supply the policy bytes; preserve any existing
  delivery history and current selection. No inferred targets or upload.
- Register revision: supply the immutable manifest bytes and explicit declared
  coverage for the new page. Append a page whose parent is the current history
  root. Leave selection/status unchanged. A draft does not become selected.
- Record delivery: supply immutable event bytes and explicit page coverage.
  Delivery must already be configured; append over the current root. Validate
  exact policy/revision/evidence through existing readers. Never select the
  delivered revision or rewrite another target's history.
- Select revision: explicitly supply the registered manifest, representation,
  exact native subject Pin and complete desired project status map. Never copy a
  status/approval implicitly. Caller explicitly names any domain payload streams
  that must follow selection; do not infer them or alter historical aliases.
  Named streams must already mirror the old/new subject and cannot be common
  history/policy streams. Crosscheck subject and selected representation. If
  previous selected bytes are unclassified, permit only classifying those same
  bytes first, not silently replacing them and losing their history.

Coverage is caller-declared revision/event-list coverage, not global knowledge or
custody. Partial ancestors stay partial. A repeated already-recorded exact fact
is a no-op; changed reuse conflicts under existing common contracts. Selection
with identical desired fields is a no-op. Retry over the same expected source
and request yields identical bytes; retry over a different source must resupply
and revalidate its base, never blindly replay a patch.

The result contains expected source, candidate record bytes/Object and only new
immutable metadata blobs. It holds no provider locations, expanded copies of old
ledgers or media. Preserve source raw bytes exactly for a no-op; changed records
use existing canonical JSON (one stable asset file, no sibling churn). Semantic
JSON values/text/order remain intact except the requested fields. A bounded
memoizing overlay reads old objects once across source/candidate validation and
charges new and old metadata against one byte/document limit. Native/ref checks
remain bounded by existing contracts. New objects remain unretained until an
explicit admitted publisher/claim handoff finishes; preparation cannot authorize
Git adoption, public delivery, cleanup, creative approval or byte deletion.

L0 structure/fmt/diff and six-role review. L1 old/new/collision/stale/selection/
policy/receipt/limits negatives, deterministic retries and sparse growth; locked
workspace tests/strict Clippy/standalone build. L2 synthetic episode-like and
non-media workflows with multiple revisions/targets and exact no-write readers.
Actual sparse Git/cache/owner handoff is a separately qualified T2 integration.

## Ordered preparation without intermediate writes

`edits::sequence::prepare` composes one to 64 explicit existing operations on one
exact original source. Each step executes the same validators; new metadata feeds
later steps through an in-memory verified overlay. Unique acquired and generated
documents share one aggregate byte/document budget. Existing reference limits
apply per operation, with the hard operation ceiling bounding repeated validation
work. The result uses the original expected source pin and final record plus
deduplicated new immutable metadata. A semantic net no-op preserves original raw
bytes and returns no new metadata. Failure returns no partial candidate and never
writes to the reader. This is not a transaction across Git and storage: callers
still publish, independently read back, and base-fence the final proposal through
existing APIs. Snapshot-level multi-asset batching and the compiled host/CLI entry
remain separate integration work; see [the entry](../docs/reviews/history-batch-entry.md).
`AssetSnapshot::prepare_history_sequence` uses the same expected-selected-record
extraction as a single edit, so callers can reuse one captured Git snapshot across
assets without changing the original selection or reading dirty working files.
