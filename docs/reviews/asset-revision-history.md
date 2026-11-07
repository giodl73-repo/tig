# T1a producer review: common revision history

Public summary: consumer identifiers generalized; original notes retained privately.

Input and controls: [ASSET-REVISION-HISTORY](../../specs/ASSET-REVISION-HISTORY.md).
Targets: new `crates/tig-assets/src/revisions.rs`, its integration tests, the
two-line optional export in `lib.rs` and that contract. This is the revision
foundation of T1, not delivery-history implementation or a completed migration.

Method: producer manually applied all six [checklists](../../.roles/ROLE.md).
No independent reviewer, human creative approval or live operation is claimed.

| Role / finding | Severity | Disposition / owner and evidence |
| --- | --- | --- |
| Identity: same-byte renders collapse into one version | High | Resolved for T1a: distinct run keys/manifests, shared payload set; same-key different-manifest histories conflict. `distinct_same_byte_runs_share_payload_and_keep_both_revision_identities` and `duplicate_run_key_is_conflict_even_for_same_payload_not_last_writer_wins` pass |
| Placement: history traversal turns every old result into working demand | High | T1a exposes separate metadata/payload/provenance sets, adopts no claims; T3 owns classification/placement integration, still OPEN |
| Custody: declared-complete history mistaken for complete recoverability | Critical | Coverage explicitly concerns revision declarations only; unknown/missing history never empty; witnesses remain unverified references and eviction/complete semantic flags remain false. Missing/corrupt, cumulative-bound and produced-witness negatives pass. T3 must traverse admitted witness closures |
| Concurrency: branch union loses history or selection advances implicitly | High | Append/union pages share old nodes, exact selected manifest/representation checked; no writer/adoption added. Branch-union and unchanged-selection tests pass. T2 expected-base writes/event retry and T3 ownership handoff remain OPEN |
| Provider/privacy: unbounded reads or leaked reader/parser details | High | Expected sizes and cumulative limits checked before calls; generic error categories, no History Debug/Serialize; no payload fetches. Bounded reader and mismatch tests pass; reader must enforce acquisition limit itself |
| Git/adoption: unbounded record or premature native replacement | High | 1,000 versions use 2,000 immutable metadata documents and one payload; record size changes by less than16 bytes. Absent capability preserves legacy records with zero reads. No consumer project records or runtime pins change |

The foundation review manifest's existing `lib.rs` target is refreshed solely for
the reviewed optional module export (392 -> 438 bytes). No prior target pins,
source limits or validation requirements are relaxed. New code is separately
scoped above; the old foundation review does not certify new runtime behavior.

## Executed local verification

- `cargo test -p tig-assets --features schema --locked`: all package suites pass,
  including 16 new revision integration tests and one cycle/diamond unit test.
- `cargo clippy -p tig-assets --features schema --all-targets --locked -- -D warnings`:
  pass. `cargo fmt --all` applied; repository structural validator and diff check pass.
- The 1,000-version integration fixture reads exactly 2,000 unique metadata
  documents; payload and opaque witness bytes are deliberately absent from its
  reader. Old metadata remains byte-identical after appends.

Full workspace all-feature qualification, standalone Git/LFS compilation and
exact-head Windows/Linux CI remain separate obligations until their results are
recorded. The existing main/PR67 journal-observation SQL race is outside this pure
module; a passing history suite does not resolve it. No CLI/provider L2 is claimed
in this slice; synthetic reader integration is its declared L2 boundary.

Next: common delivery policy/event history reusing the existing publication
checker and this bounded history approach, then explicit T2 operations. Historical
claim/context evidence must still be admitted by existing validators. No automatic
selection, production freshness, custody, placement or cache reclamation follows.
