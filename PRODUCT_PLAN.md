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
  Record comparison uses an exact in-memory baseline across projects, exposing
  independent identity/field/slot differences without opening payloads.
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

## Next delivery sequence

Planning input f742abf7f62b3c4c9c65149a404626679a004f71. This sequence
prioritizes usable operations over additional catalogue decoration. Estimates
and provider capabilities must be measured per admitted scope. No live storage
action is authorized by this roadmap; fixture work uses isolated temporary roots.

| Priority | Deliverable | Demonstrable exit |
| --- | --- | --- |
| 1 | Finish current catalogue changes and user UX test | Protected checks pass; user can find a work, compare versions, trace an owner and return without losing context; task failures become regression fixtures |
| 2 | Unified storage plan and agent contract | Explicit source/target scopes, immutable object set, policy/source basis, copy/retain/remove dependencies, budgets, reasons and holds; same plan data powers CLI and UI |
| 3 | Recoverable local copy orchestration | Plan/apply/status/resume in temporary stores; bounded streaming, destination staging, hash/length readback, retained destination and restart-safe acknowledgement; source preserved |
| 4 | Guarded local eviction | Recheck current claims, active use, source/policy and retained destination before removing an exact copy; concurrent claim or stale evidence blocks that object; actual file/allocation results reported separately |
| 5 | Explicit cloud and Git LFS upload adapters | One selected remote provider first, then LFS upload; retry, failed/lost acknowledgement, corrupt/missing readback, credentials and unsupported retention capabilities exercised; no remote retirement from mere upload success |
| 6 | Git-driven lifecycle reconciliation | Commit/branch/merge/revert produce idempotent desired-placement deltas; protect drafts/jobs and retained revisions; event replay and missed events converge without global locks |
| 7 | Scale, onboarding and release | Derived incremental indexes keyed to exact sources/rules, measured warm lookup and sparse updates, generic sample enrollment and operator/agent examples; rebuild survives deleted derived state |

UX testing can run in parallel with priorities 2-3. Storage selection should show
unique content, copy bytes, scratch peak, retained obligations, estimated eligible
bytes and measured results separately. Add grouping/filter changes only when they
help answer a concrete task such as why one work grew or what blocks reclamation.
Never count prospective eviction as free space for an admitted job.

### Storage orchestration contract

```text
Git snapshot + drafts/jobs + placement/retention policy
                          |
                      explicit plan
                          |
              copy -> verify -> retain destination
                          |
                  recheck scoped protection
                          |
                authorized source-copy removal
                          |
               fresh capacity/copy observation
```

These are target operator concepts, not newly implemented CLI commands. Use
existing identity, publication, ledger, retention, source-claim and provider
primitives. The adapter declares capabilities: readable, writable, resumable,
verifiable, retainable, removable, and enforceable protection where supported.
Unsupported guarantees produce explicit holds. Keep operational journals and
large plans outside Git; bounded source definitions and policies remain in Git.

Cache, remote object store and Git LFS endpoint are separate provider roles.
Publishing a Git pointer does not prove its bytes reached a remote; successful
transfer does not establish an independently retained backup. Removing one cache
copy differs from deleting an object's final archive. Both use exact byte
identity, but have different obligations and authority.

### Fixtures already present and the missing end-to-end proof

| Existing evidence | What it covers | What it does not establish |
| --- | --- | --- |
| tig-store publication/accounted tests | Local immutable publication, bounded resource accounting, interruption and private partial cleanup | Arbitrary final-object eviction or cloud transfer |
| tig-store replica tests | Completed hash/length verification, object guards, alias/domain refusals and stale evidence | Durable independent backup by itself |
| tig-store retention and handoff tests | Durable scoped claims, restart and acknowledged ownership handoff | Source payload deletion or complete project coverage |
| tig-store metadata_copy_cli tests | Explicit plan/apply and fresh-process metadata-store verification with source preserved | General media/payload transfer |
| tig-git client tests | Real Git LFS current/prior checkout, parallel clients, corruption/retry and selected-source authorization | Remote upload, cloud custody or eviction |
| Catalogue storage tests | Placement/version/shared-byte classification and stale held previews | Executed cache movement/removal |

The next end-to-end fixture is a synthetic work with two unchanged inputs and one
replaced input, a delivered output, two branches and an active reader. Use three
temporary stores as hot/history/archive roles supplied by configuration. Advance
main, compute the delta, copy and retain required history, remove only eligible
hot copies, then restore the previous revision from retained storage. Compare
predicted unique/copy/peak bytes with observed results. No real drive names,
project vocabulary or user assets belong in this generic fixture.

Before enabling that removal path, test: interruption during copy; full target;
wrong hash/length; lost completion reply; duplicate retry; missing retained copy;
same-location aliases; new claim after planning; active reader; concurrent unrelated
writer; changed Git/policy basis; reverted selection; partial/unknown dependency
coverage; partial batch progress; retained destination going offline; and restart
after removal but before completion acknowledgement. Each case names the exact
permitted effects and proves surviving bytes, not only a reported success flag.
Then repeat the same contract suite against a local remote-provider test server
and real LFS client fixtures. Explicit opt-in provider qualification follows;
synthetic remote success alone does not qualify a cloud service.

Trace: priorities 2-4 map to WP-003/004/006/007/008, priority 5 to WP-005/013,
priority 6 to WP-009/011/012 and priorities 1/7 to WP-008/015/016. Each slice
requires exact-source controls, focused failures and protected checks. Optional
torrent transport, broad WASM parity and additional presentation features follow
the working local orchestration slice; they are not prerequisites for it.

## Full outstanding obligations

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
