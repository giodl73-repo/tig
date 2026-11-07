# Bounded source batch observation

Entry source `7b315b244db5a92fe30a29edb9ebccd434b7c2bd`;
WP-008/009/015/016, REQ-003/006/007/010, SPEC-005/008/014/015,
IF-004/006/013, PKG-003; RUST + INTEGRATION profiles.

Scope: read an explicit set of 1..64 distinct source IDs from one existing
source-journal replay. No authority creation, source discovery, recovery,
mutation, process inspection, byte access, cache scan or removal.

`SourceJournal::observe_many(root, minimum, sources)` validates the bounded
selection before opening the journal. It opens the same read-only authority as
single-source observation, replays once and returns one checkpoint plus a sorted
map of the requested IDs to source states or explicit nulls. Each state belongs
to that checkpoint. Existing single-source behavior remains unchanged.

```text
tig-source inspect-many PRIVATE_PROFILE NEW_PRIVATE_REPORT
```

The strict profile schema is
`tig.source-batch-observation-profile.experimental-1`, with `journal_root`,
`minimum` and `sources` (an array of IDs). Existing 64 KiB profile/report limits
and create-only external report publication apply. A too-large result refuses;
there is no truncation or silent subset. The report schema is
`tig.source-batch-observation.experimental-1`; `sources` is an ID-keyed map.
Stdout exposes bounded aggregate counts and journal sequence, never IDs, roots,
owners, digests or local paths. Detailed reports stay private asset-side.

`single_checkpoint=true` means only that all returned source states share the
verified replay checkpoint. It is not a live lease, a snapshot of files/processes,
current producer success, complete claim coverage or recovery/eviction authority.
Absent IDs are absent only in that journal snapshot. Writers can advance after
the observation; modifying commands still need fresh owner/revision checks.

L0: format, scoped Clippy, foundation and diff checks. L1: empty/duplicate/over-limit
sets, explicit absent sources, rollback/foreign/missing authority, read-only file
preservation. L2: concurrent two-source writer with checkpoint/revision coherence;
CLI redaction, refusal, output no-replace, bounded report and old CLI regressions.
Exit: passing scoped checks and producer privacy/recovery review, followed by
separate exact-head hosted qualification. Actual consumer use is separate.
