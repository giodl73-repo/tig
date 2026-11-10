# Current architecture

This describes the implemented experimental workspace. The
[engineering architecture](vtrace/ARCHITECTURE.md) also allocates target components;
its boxes are not a claim that every service/provider already exists.

## Packages and dependency direction

```text
                         tig-git
                   Git/LFS adapters + CLIs
                  /          |            \
                 v           | optional    v
             tig-store       +--------> tig-assets
                 |                          |
                 +------------+-------------+
                              v
                           tig-core
```

| Crate | Implemented responsibilities |
| --- | --- |
| tig-core | Immutable object/tree identity, canonical records, binding views, claims, placement and pure worktree operations |
| tig-store | Local publication, guarded existing-byte reads, capacity/operation ledgers, immutable metadata, source claims, replica verification and retention |
| tig-assets | Optional native records/resolver port, declared graphs, sparse variants; schema-gated contracts, revisions, deliveries, edits and publication checks |
| tig-git | Exact commit/index capture, Git identity/pointers, existing-cache LFS download and optional Basic upload, native resolution, project restore, sparse proposals, history and local catalogue |

The basic tig-git build uses tig-core and tig-store. Its **assets** feature adds
tig-assets; **schema** adds assets plus schema validation, catalogue and history
facilities. tig-assets/schema adds its JSON Schema-backed contracts. Low-level
placement and byte access do not depend on typed assets, a renderer or project
checkout. Crates are publish=false while interfaces are experimental.

The independent **upload** feature enables the explicit standalone LFS upload
agent and operator commands. It uses tig-store's network-free **upload-witness**
feature over existing accounted publication; **s3** also reuses that witness
composition without changing its wire identities. Only the optional HTTP adapters
enable the maintained TLS client. Default Git/LFS downloads remain independent
of HTTP and typed-asset features. Remote availability grants no retained custody.

The default `tig-lifecycle` CLI and `lifecycle` library connect an exact enrolled
committed Git ref to the existing guarded source namespace. Immutable metadata
finishes and is read through a fresh reader before one transactional Prepare/Adopt
pair publishes desired protection. Existing event encodings and roles are reused.
No-op duplicates append no commands; historical replay cannot replace a newer
selection. Current ref observations are separate from admitted state. This is
the selection prerequisite. `tig-store::placement` adds generic requirements
projection and bounded pages over existing accounted publication, retention,
handoff and managed retirement/reacquisition. `tig-lifecycle reconcile` composes
that executor after exact committed-ref admission, including no-op selections.
Copies retain stable original operation/job bindings across later Git revisions.
Required cache readers span conflicting source removal; existing all-namespace
and provider fences still decide removal. No new cache engine or journal exists.
Page convergence is distinct from full selection convergence. Complete lifecycle/
provider adoption, large files and operational continuation remain required.
No service starts on resume.

The independent **github** feature adds fixed-issuer RS256 request verification,
a bounded metadata-only GitHub CommittedSource adapter, foreground loopback
`tig-trigger` intake and a coordinator over the same selection/placement APIs.
Local Git implements that source seam without changing its listing/context
identities. Private enrollment supplies authority; a signed commit request cannot
choose roots or permissions. Complete same-basis passes, partial effects and
source drift remain separate observations. The inert Action example installs
nothing; external TLS ingress, workflow/executor OS separation, full issuer/
provider qualification and adoption are separate gates. See
[trusted requests](../specs/GITHUB-TRIGGER.md).

Default `lifecycle::index` and `tig-lifecycle index` protect an explicitly enrolled
worktree's staged pointer set as a Draft source. Distinct immutable contexts bind
the index listing and unresolved stage counts; the flat obligation tree deduplicates
payload identities rather than pretending conflict stages form one merged view.
Known alternatives participate in existing placement/admission. Other extraction
issues hold adoption, and source observations stay separate from durable receipts.
No Git/index write, hook or journal format is added. See
[staged-index contract](../specs/INDEX-LIFECYCLE.md).

`lifecycle::settlement`/`tig-lifecycle settle` compose owner Withdraw with verified
ref absence or adopted-successor byte coverage. Up to eight source-revision
conditions are rechecked inside the existing journal transaction, preserving its
event encoding and short SQL scope. Index/settlement share private local placement
preflight; Git observations and current admitted-source state remain independent.
Predecessor/history and retained backups are not deleted. See
[owned settlement](../specs/SOURCE-SETTLEMENT.md).

The schema-gated `tig-query` CLI and `catalogue::query` library reuse captured
native metadata, catalogue projections and the graph engine. Machine-readable
request/result schemas cover inspect/admission, find, comparison and impact.
Admission diagnostics group exact definition revisions, expose contract field
names/schema pins and reuse the graph's bounded profile validation/fingerprint.
They omit definition defaults and schema examples; instance edge evaluation
remains a separate impact query. Owner-admitted inspector fields are shared with
the UI. The fenced basis includes those field rules. No second asset registry,
query database, media scan or mutation engine is introduced. See
[agent queries](AGENT-QUERIES.md).

Explicit manifest queries reuse admitted work projections, nested adapters and
verified byte readers. A source basis fences configuration before IO; a content
token fences bounded extraction results between pages. Physical byte-source
paths are hashed into interpretation identity without being exported. Named JSON
reads yield document/pointer associations, not semantic production edges. Media
leaves remain unopened; missing roots remain unknown and grant no eviction rights.

Manifest impact uses the same extraction and a multi-root reverse traversal of
one native graph. Association owners are exact selected records; paths traverse
only admitted semantic edges. Owners and other consumers are separate, with
deterministic shortest witnesses and independent extraction/graph coverage.
Retained byte associations remain conditional evidence, never production edges.

The catalogue Impact tab uses a fixed metadata-only authenticated endpoint over
the same native impact resolver and basis framing. It pages 25 consumers, keeps
gaps/admission diagnostics visible and supplies navigation cards only for exact
selected revisions. It exposes no general agent-query or byte-read endpoint.

The fixed catalogue comparison endpoint calls the same compare_assets operation
as the agent CLI, checking both captured project snapshots and optional basis.
The frontend baseline retains exact record identity and generation-fences its
results; it does not become a retention claim or a historical-source provider.

Profile previews pass an explicit candidate set to the same impact resolver and
compare full known consumer sets and shortest-witness fingerprints. Neither
candidate nor baseline enrollment is temporarily mutated. The request must fence
the current source/interpretation basis; partial knowledge remains explicit.

## Authored state, bytes and operational state

```text
Git: small records, definitions, schema/policy references, LFS pointers
  |
  +-> exact captured snapshot -> admitted definitions / record IDs
  |                                  |
  |                           optional semantic resolution
  |                                  |
  +----------------------- exact byte object / tree
                                     |
                     explicit provider or existing-store profile
                                     |
                               guarded bytes

Outside Git: payloads, large immutable indexes/history, local enrollment,
             credentials, ledgers, claims, observations, private evidence
```

Git blob identity, canonical record identity and raw payload SHA-256 are different
domains. Resolve an ID within one exact snapshot; never substitute the newest
file in a directory. Small native records can reference larger immutable history
documents without rewriting a growing inventory on every edit.

Providers own physical layout. Existing-byte readers admit flat, Git LFS and
CAS layouts, verify the named object and retain the declared read protection.
Strict Windows protection and cooperative immutable mode have different guarantees.
See [existing-byte contract](../specs/EXISTING-BYTE-READ.md).

## Selection, production and delivery

The optional asset layer separates authored metadata, declared dependencies,
selected revisions, production contexts, result evidence and delivery history.
Changing a selection need not render immediately. A producer consumes an exact
resolved context; result currency and delivery currency are separate questions.
Project status vocabulary, language/style dimensions, accepted profiles and
production semantics come from admitted definitions/adapters.

Graphs distinguish production, composition, evidence and authority relationships.
Declared graph resolution is not complete extraction of opaque native formats
or proof of the producer's actual consumed inputs. Revision history, publication
evidence and external targets do not independently authorize storage retirement.

## Local catalogue data flow

```text
external enrollment -> resolver profiles -> captured Git metadata
                                            |
                                 admitted presentation rules
                                            |
                            project / collection / work / child
                                            |
                     versions, deliveries, search, direct byte objects
                                            |
explicit JSON expansion -> bounded nested descriptors and provenance
explicit observation   -> named file attributes / capacity observations
explicit verification  -> guarded hash-and-length check
                                            |
                       comparison / preferences / held dry-run review
```

The Rust application bundles static HTML/CSS/JavaScript and serves loopback HTTP.
There is no build-time web service or installed daemon. Requests are session-bound;
configured display fields are projected rather than exposing raw authored records.
The loopback service is a local tool, not a remotely hosted multi-user application.

Work grouping is a presentation projection, not another asset registry. Counters
use project-admitted selectors/status interpretation. Different IDs or languages
do not automatically become extra deliverables. Nested descriptors inherit the
admitted work memberships of their referencing records; shared bytes remain shared.

Coverage means configured paths were expanded, not complete production closure.
Comparison distinguishes unique declared content from observed copy bytes.
Work totals can overlap. No observed copy is not proof of absence in unprobed
stores. Removed version members do not establish zero demand or deprecation.

Reads, selectors, expansion depth, associations and responses are bounded.
Metadata IO for expansion/comparison runs outside the catalogue mutex. Snapshot
and coverage-basis checks reject stale results/mixed pagination. Loaded expansion
and observations are process memory, cleared on restart; no hidden metadata read
repopulates them. See [catalogue contract](../specs/CATALOGUE.md).

## Storage safety boundary

Desired placement, operational plans, observations, retained protection and
removal authority remain separate. There is no cross-system transaction spanning
Git, local files and remote providers. Bounded local protocols use scoped intent,
safe acknowledgement ordering and restart recovery. No global shared-cache lock
is a prerequisite for unrelated writers.

The library contains local provider/ledger/retention primitives; the catalogue
does not attach a generic eviction executor. Full active-job/dependency coverage,
independent custody, fresh provider fences and authorization remain prerequisites
for safe retirement. Complete remote transport and automatic Git-hook lifecycle
management are future integration, not hidden behavior in the viewer.

## Entry points

| Binary | Gate | Purpose |
| --- | --- | --- |
| tig-storage | tig-store basic | Explicit bounded plan/peak/status/apply/resume for source-preserving local copy and retention |
| tig-source | tig-store basic | Explicit source-authority inspection and durable source-claim transitions |
| tig-lfs | basic | Explicit existing-cache Git LFS transfer agent |
| tig-propose | basic | Prepare bounded verified byte-to-Git proposals |
| tig-resolve / tig-graph | assets | Resolve IDs / inspect admitted declared graphs |
| tig-restore / tig-project | assets | Explicit named/project restore, preview and guarded application |
| tig-history | schema | Inspect/prepare/publish history through an explicit local host profile |
| tig-catalogue | schema | Validate enrollment or run the local viewer |
| tig-record | tig-assets basic | Inspect an explicitly supplied native record/stream; schema options use schema feature |
| tig-publication | tig-assets/schema | Inspect explicitly supplied publication evidence |

Flags and authority differ by command; follow the relevant spec rather than
treating the catalogue's read-only boundary as a promise for every CLI.
The older Python restore helper is compatibility orchestration; new integrations
use the Rust project API. Python checker scripts are development tooling.

The [roadmap](../PRODUCT_PLAN.md), [adoption gates](CONSUMER-ADOPTION.md) and
[evidence register](vtrace/EVIDENCE.md) describe outstanding system obligations.

## Storage orchestration entry

The experimental tig-store::orchestration module binds a bounded exact plan to
existing accounted publication and retention. `tig-storage` supplies explicit
plan/peak/status/apply/resume JSON operations; source-preserving local copying
uses an already admitted capacity job and has no optional typed-asset dependency.
It adds no cache engine, source/job registry, eviction, automatic Git event
installation or final deletion. See [scope and protocol](../specs/STORAGE-ORCHESTRATION.md).

The tig-store::protection projection observes all registered sources in explicitly
enrolled journals and resolves admitted flat byte-binding roots through existing
Tree/NodeStore ports. It joins current/pending/predecessor demands with owner and
kind provenance; missing/opaque/changing scope remains a bounded gap. Its View
adapter keeps complete protection false. See [scope and limits](../specs/PROTECTION-DEMAND-SNAPSHOT.md).
