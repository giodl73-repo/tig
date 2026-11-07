# Native variant retention producer review

Entry1095b6fc591e7ac9155989188081a06d803bce96 (merged PR37).
Controls, scope, entry/exit and L0/L1/L2 were recorded before code in
specs/NATIVE-VARIANT-RETENTION.md. Six-role producer review, not independent or
human acceptance. Owner: Tig producer. No new dependency or production-cache IO.

- Identity: external exact definition/space/project-space/semantics/axes policies;
  re-admit context and resolve the exact source record, including retained frozen
  records at the current checkpoint. Compare complete generated witnesses/tree
  on restore; supplied stored policies never grant admission.
- Privacy/provider: no Debug for packets/admissions or emitted private contents;
  bounded exact NodeStore reads reuse existing metadata ports. Missing/corrupt
  controls refuse. No media reads, cache creation or private source copied to Git.
- Git/adoption: one bounded witness per record plus deduplicated controls in an
  asset-side Tree; job gets one link. Legacy version1 calls/bytes are unchanged;
  optional version2 rejects missing/wrong packet and foreign source context.
- Recovery/concurrency: immutable candidate roots, no global lock. Existing
  persistent metadata finish/reopen/readback precedes explicit owner journal
  adoption in the fixture. A packet is not automatically enrolled or durable.
- Placement: require per-record coverage even when two records share one payload;
  compare each selected primary set exactly, never silently retain all languages.
  Packet references join Interpretation, not Primary. Counts are logical references,
  not allocated disk bytes or a complete provider tree-node footprint.
- Custody: no producer execution, actual byte protection, independent retained
  copies, complete global claims or removal authority. Production context/claim
  replay alone does not yet replay attached variant packets; callers explicitly
  restore the packet and optional job under independent admission.

Nine focused native cases pass. Early compile-only fixtures used nonexistent
graph defaults and private helper types; corrected to the actual public bounded
APIs without changing production contracts. Packet replay with zero write budget
passes; write-side and read-side limits remain distinct. Partial candidate tree
failure can leave unadopted metadata, never an adopted successful packet.

Learning: checking only a deduplicated global primary set misses an omitted
witness when two assets share bytes. Validate owners individually. Full unused
style changes alter packet/job provenance but preserve projected audio subject
and declared recipe. Historical exact references must not select newer ID bytes.

Residuals: dynamic per-owner graph projection, actual private bilingual producer,
generated-input chains, produced-claim packet replay, global source/job recovery,
custody/compaction/eviction and all-family/history reversible adoption remain open.
Disposition: pass_with_risk for this explicit metadata seam pending full local
and exact-head hosted qualification; no complete intended-use gate is closed.

Final local qualification:548 full workspace all-feature entries pass, including
all69 native-source entries and the existing500,000-binding persistent-tree
fixture. Workspace all-feature/all-target warnings-denied Clippy, standalone
Git/LFS check, format and structural/role checks pass. No Rust changed after
these runs. An earlier full-run observation handle disappeared without terminal
output; process inspection confirmed no remaining job and its partial log is not
counted as a pass. The recorded final run has a terminal zero exit. Exact-head
Windows/Linux hosted checks remain required before merge.
