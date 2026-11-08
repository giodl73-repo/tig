# Tig roadmap

Tig connects small Git-authored references to immutable bytes and explicit storage
obligations. It remains usable for Git/LFS without typed asset schemas, and across
unrelated projects without built-in media or cache policy.

The experimental source baseline is public. Its six-role review and ongoing
release checks are recorded in [public release](docs/PUBLIC-RELEASE.md).
This is source availability, not a stable product or full storage acceptance.

## Implemented experimental slices

- Pure identity/trees, bindings, claim handoff and bounded placement arithmetic.
- Local publication, scoped coordination, capacity journals, retained claims and
  recovery primitives with synthetic and real-client fixtures.
- Exact commit/index extraction, source journals and download-only Git LFS cache
  interoperability. Stores are explicitly enrolled, never discovered.
- Optional native records, schema admission, graphs, sparse variants, production
  contexts, revision/delivery history and sparse Git proposals.
- Explicit Rust project restore, handoff/progress evidence and history inspection.
- Local catalogue: work counts, nested navigation, search, versions, guarded
  metadata expansion, coverage, copy observations, placement preferences and
  work/version/object comparisons. Movement/removal remains a held review.
  Record impact exposes admitted consumer paths, coverage gaps and exact selected
  navigation through explicit relationship selection and source-fenced paging.
- Agent queries: versioned JSON discovery, inspection, admission diagnostics,
  search, byte/record comparison and explained impact through admitted graphs.
  Explicit work-scoped manifest references reuse nested adapters and verified
  readers, with source/content fences and document/field provenance.
  Manifest impact joins all matching owners to one admitted graph, separating
  association evidence from conditional native consumers and coverage gaps.
  Missing semantic profiles and excluded inventories remain explicit gaps.

Current contracts and [architecture](docs/ARCHITECTURE.md) limit each claim.
[Evidence](docs/vtrace/EVIDENCE.md) records historical checkpoints rather than
certifying full requirements or every consumer.

## Remaining milestones

| Milestone | Exit condition |
| --- | --- |
| Consumer completeness | Explainable declared versus consumed inputs, opaque gaps, active jobs and exact profile/source coverage |
| Lifecycle reconciliation | Durable scoped demands and idempotent plans across Git changes, interruption and concurrent work |
| Guarded execution | Independent retained custody, provider revalidation and authority before retirement; physical reclaim measured in isolated fixtures |
| Catalogue usability | Comparable coverage, shared bytes, versions and primary subtypes; explicit enrollment and tool handoff |
| Providers/transfers | Capability/operational/license review and separate recovery/custody proof for remote or optional torrent adapters |

No deadline or completed consumer migration is implied. Catalogue work can advance
independently of deletion. Do not create a second cache engine or global cache
lock to complete a milestone. Estimates, observations, protection and authority
remain distinct.

The [17 work packages](docs/vtrace/WORK_PACKAGES.md),
[catalogue roadmap](docs/CATALOGUE-UI-PLAN.md) and
[adoption gates](docs/CONSUMER-ADOPTION.md) provide finer allocation.
