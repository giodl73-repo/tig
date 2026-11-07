# Project handoff links: producer review

Public summary: consumer identifiers generalized; original notes retained privately.

Entry `a35f48fb12eeedea36c430cb226f01a59762cf9d`; controls and allowed scope
are in [PROJECT-HANDOFF](../../specs/PROJECT-HANDOFF.md). WP-005/011/015/016,
REQ-001/003/006/010; RUST + INTEGRATION. Extend the existing frozen-plan result,
not a second inventory, transfer engine, journal or executable hook mechanism.

The consumer can declare sixteen bounded labeled Object pins in its local
profile. Raw profile identity already binds them to the expected plan. Reports
retain those declared links with an explicit false verification flag, alongside
each entry's selector and relative destination. This distinguishes aliases and
literal named streams from selected outputs without introducing a second list.
Links add no source requests, payload copies, byte obligations or authority.

Five new real-Git fixtures plus all existing project regressions pass: 33 tests,
20.14 s. Checks cover absent/unread evidence, exact selected/named-stream aliases,
unchanged read/object counts, plan rejection on changed evidence, count/label/
object/duplicate-key bounds, failed/interrupted reports and an actual CLI using
a frozen commit older than checkout HEAD. The first duplicate-key fixture
constructed its mutation using a different serialization field order, so its
assertion that input changed failed. The fixture now serializes the actual
profile Value before injecting a duplicate; the intended rejection passes.
No production rule or assertion was relaxed. Full workspace/lint and hosted
qualification are separate gates; no real consumer runtime is switched.

The subsequent locked all-feature workspace tests pass, including the
500,000-binding persistence fixture. Strict all-target all-feature Clippy passes
(24.13 s), as does standalone Git/LFS checking (6.00 s). Format, foundation and
whitespace checks pass. Exact-head hosted qualification remains required.

An owner-run read-only consumer project preview at `e29e355` using the immutable tool built
from Tig `1d64b6e` also passes: two selected placements of one 1,059-byte retained
text object, 2,118 logical destination bytes, one source verification, zero copies
and no destination directory. Two declared links pin a separate runtime descriptor
and the earlier comparison report. Their verification flag and all custody/
removal/completeness flags remain false. Raw profiles/results/executable hashes
stay outside Git. Later documentation does not relabel this actual run's source.

Producer-applied lenses:

- Identity/namespace: exact external pins are provenance links, independent of
  byte selection/identity; profile changes fence old plans before writes.
- Provider/privacy: no link is resolved, read or executed. Labels use portable
  relative-path admission. Detailed results remain private asset-side.
- Storage/custody: no linked approval/custody label is interpreted as authority;
  existing source-guard, custody, eviction and coverage flags remain false.
- Git/adoption: no growing report or consumer payload enters Git; old profiles
  still work. Result consumers must admit experimental-4 before upgrading.
- Concurrency/recovery: prior scoped copy/progress behavior is unchanged; links
  survive structured failure without implying completion or automatic recovery.

Disposition pass_with_risk for declared handoff linkage. Owner: Tig orchestration
and consumer integration. No independent review/human approval. Orphan
classification, link evidence verification/retention, complete environment and
producer semantics, actual consumer adoption and custody/removal remain open.

Integration base follow-up: PR57 qualified Windows/Linux and merged as `b2086a5`.
The handoff branch is rebased on that source. Conflicts were confined to review
catalog pins and additive VTRACE paragraphs; both EVID-072/073 entries remain.
Exact Git comparisons preserve the previously tested handoff Rust source/tests
and the qualified publication Rust source/tests. Format/foundation/whitespace
checks pass; combined exact-head hosted qualification remains required. The
earlier immutable handoff executable and actual preview retain their original
`1d64b6e` source rather than being relabeled as the rebased branch.
