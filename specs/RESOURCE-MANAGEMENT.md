# Resource budgets, reservations and observations

Status: target contract with tested capacity accounting and a bounded local
restart-persistent command ledger. Explicit local sampling/publication integration
is implemented; native domain/quota authority, whole-device enforcement and a
complete durable resource service remain open.
Parents: TIG-R02/R03/R04/R05/R07/R09; SPEC-003/004/005/008/009. Existing pure
handoff tests do not prove concurrent space reservation or measured allocation.

## Model independent of projects, drives and media

| Entity | Responsibility |
| --- | --- |
| Project policy | Named worksets/lifecycle, proximity, retention and resource profiles |
| Bundle | Exact object membership; overlapping bundles share immutable objects |
| Place | Admitted storage endpoint and capabilities; local root, remote service or archive |
| Capacity domain | Physical volume or provider quota shared by one or more places |
| Worktree | Small authored control view and independent source/working-set claims |
| Job | Bounded admitted input acquisition, output, materialization and scratch demand |
| Reservation | Durable owned unallocated headroom, scoped to a capacity domain |
| Observation | Actual scoped measurements, freshness, attribution and uncertainty |

Roles such as working, retained and archive are configurable names, not a fixed
tiering enum. Local bindings map roles to places outside portable authored
policy. A source schema/profile can supply expected asset/job sizes; users need
not set a quota on every object or enumerate another handwritten bundle list.

Several places can share one capacity domain. Do not reserve the same free disk
independently under different path aliases. Failure domains for backup and
capacity domains for admission are related but not interchangeable: distinct
quotas may still live on one underlying failure domain.

## Separate actual bytes and remaining reservation

For capacity domain v, use a fresh admitted observation and durable reservations:

```text
F(v) = physically reported free bytes, or admitted provider quota availability
H(v) = required safety headroom
R(v) = sum of still-unallocated portions of admitted reservations

A(v) = max(0, F(v) - H(v) - R(v))

new peak demand(v) = missing input copies + new outputs + additional scratch
                    + materialization + publication/representation overhead

admit only if new peak demand(v) <= A(v), and all applicable job/project limits fit
```

Allocated scratch/output is already reflected in observed free space. Subtract
only the remaining reservation; subtracting the original peak again double-charges
work and can unnecessarily block all rendering. Observation and allocation
watermarks must reconcile atomically enough to prevent a gap that counts neither
reservation nor physical allocation. A naive daily sample plus an unrelated
counter is not a safe admission protocol.

Reserve scoped domain headroom before job start, with generation checks,
idempotency and bounded retries. Multi-domain jobs need recoverable partial
reservation acquisition and may start only when their required scopes are
admitted. Transfer IO is outside short catalog transactions. No laptop-wide
render/cache lock or full-job-duration catalog lock is permitted.

Idle worktrees do not automatically reserve every configured scratch maximum.
Per-worktree limits bound its potential jobs; active jobs obtain actual scoped
reservations. Existing protected jobs keep their obligations. A request that
cannot fit is deferred/rejected with reasons; unrelated places/jobs may proceed.
An external application can consume disk after admission, so providers still
need safe full-disk handling. Reservations are not guarantees about foreign IO.

## Budgets and estimates

Track controlling profile settings separately from warnings/estimates. Distinguish
expected, conservative bound, observed and unknown. Include immutable payloads,
metadata, LFS local-store copies, editable materializations, private partials,
retry/pack staging and retained versions. Final encoded output size is not the
peak production workspace requirement. A large input already present at the
chosen place adds zero acquisition bytes there; a missing input must be admitted.

Project quotas/chargeback need an explicit attribution rule. Physical totals
deduplicate an object per allocated location; two project reference lists can
have overlapping logical bytes. Do not sum logical bundle/project sizes and call
that physical use. Object, pack, hard-link/reflink and provider billing effects
require truthful capability-specific accounting.

## Incremental and periodic observations

Managed publication/transfer/release events update observations and reservation
watermarks incrementally. Explicit periodic sampling reconciles free space,
known allocated data, scratch, materialization, operational metadata and unexplained
delta. A routine sample is not a full cache crawl or full byte rehash. Providers
declare cost/coverage and unknown states; offline archives are not probed by
default or treated as absent. Daily frequency is configurable, not a daemon
installed on repository open.

Growing observations/deltas live outside ordinary Git in bounded catalog/journal
storage and immutable report chunks. They have their own retention and overhead
budget. Git holds authored policy and optional reviewed checkpoint pins, not
an ever-growing daily spreadsheet or duplicate editable asset inventory.

Report change attribution and drift: new unique objects, promoted replicas,
historical copies, active/completed/unknown scratch, materializations, metadata,
external changes and unaccounted differences. Actual free space is useful even
when attribution is incomplete; incomplete attribution cannot justify deletion.
Trend/projection ranges are planning aids, not admission or custody proof.

Completed job cleanup applies only to its declared disposable scope after
required outputs/protection are settled. Unknown files and uncommitted data are
not automatically disposable. Eligible-cache and compaction debt can be planned,
but anticipated reclaim cannot fund a job until execution and a fresh observation
establish the space. Final archival deletion remains a separate policy action.

## Required proof

WP-003/004/006/007/008 must exercise reservation/allocation handoff races, shared
capacity aliases, multiple projects, idle versus active worktrees, stale samples,
external writers, full disk, partial admission recovery, pack pinning and unknown
scratch ownership. Quantitative acceptance pins workload/provider limits before
closeout. The worked guide and JSON fixture check arithmetic only.

The later [scoped coordination checkpoint](SCOPED-COORDINATION.md) adds 18
executable model tests for reservations, in-flight writes and sample watermarks.
It does not turn the original JSON walkthrough into runtime admission evidence.
Between coherent observations, additionally charge confirmed allocations newer
than the sample's captured watermark. Capture that watermark before probing;
the model permits conservative temporary double charging, never an uncharged gap.

[Reservation journaling](RESERVATION-JOURNAL.md) adds transactional persistence,
incremental replay and same-handle fresh-probe admission. Actual physical probing,
allocation enforcement, journal compaction and full restore authority remain open.

## Local sampling component

[LOCAL-CAPACITY-PROBE](LOCAL-CAPACITY-PROBE.md) implements actual user-available
filesystem sampling under a freshly committed ticket. Explicit enrollment binds
random store/ledger instances; it does not discover physical domain/quota identity.
No DB transaction spans sampling. Full resource enforcement and retirement remain
open; EVID-016 proves only bounded temporary-root integration.
