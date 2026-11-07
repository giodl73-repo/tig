# Git-first operation and state ownership

Status: full-system target with bounded experimental source/LFS components.
Commands below illustrate an intended general interface; no production Tig CLI,
hook or complete catalog exists. Explicit source extraction is implemented in
[GIT-SOURCE-EXTRACTION](GIT-SOURCE-EXTRACTION.md), and download-only transfer in
[LFS-CACHE-INTEROP](LFS-CACHE-INTEROP.md). Git-only onboarding does not need Art.

## Four kinds of state

| State | Authority and location | Recovery rule |
| --- | --- | --- |
| Authored policy | Small reviewable rules in Git; trusted local config binds place IDs to roots/credentials | Restore exact Git policy; do not execute arbitrary repo-provided commands |
| Logical snapshots | Exact Git commit/tree plus LFS pointers, or explicit immutable Tig manifests | Rebuild bounded indexes from pinned source; missing source is unknown |
| Protection and work | Durable admitted claims, active-use records, operation journals and provider retention commitments outside Git | Recover before destructive work; lost authoritative protection state blocks removals |
| Derived catalog | Object/replica/path/claim indexes and scoped provider observations outside Git | Rebuild incrementally from the above and named providers; stale data is not complete truth |

Do not call every local database row disposable. Cached indexes are derived;
uncommitted draft admission, active work and pending publication are durable
protection records with explicit owners/recovery. Losing their journal must never
silently make an object surplus. No single append-only Git file coordinates jobs.
Share machine-local catalog/protection storage across admitted worktrees; do not
copy it into every worktree. Backend and on-disk schema remain undecided.

## Minimal Git/LFS onboarding

Conceptual commands:

```text
tig init                 # opt-in local registration; no hydration or census
tig status               # desired / available / missing / surplus / unknown
tig plan                 # bounded proposed acquisitions and eligible releases
tig apply <plan-id>      # explicit admitted execution with fresh checks
```

Register the repository using its common Git directory and a durable repo ID;
track individual worktrees separately. Paths, remote URLs and branch names can
change and are not stable repository or claim identities. Do not silently combine
repositories just because they share a remote. Different projects may share
content bytes while retaining independent access and retention obligations.

Configuration separates portable policy (which refs/paths and lifecycle classes
need which place roles) from machine-local mappings and credentials. Validate a
small initial enrollment plan with declared roots, read limits and disk budget.
Never crawl every historical commit or every mounted drive by default. Hook and
LFS-agent installation are separate explicit configuration steps, with inspection
and removal paths; ordinary commands may also reconcile without hooks.

## Branches are claim sources, not extra physical directories

```text
Git main at C1       -> exact pointer set S1 -> current claim
Git feature at C2    -> exact pointer set S2 -> admitted draft claim
active worktree/job  -> exact object uses    -> transient protection
                                        |
                                 union of obligations
                                        |
                          one placement plan per scope
```

Before publishing a new claim generation, resolve the exact candidate view and
persist its protection. During a ref transition, protect the old and new relevant
sets until the new checkpoint is durable. Incomplete pointer extraction prevents
claim shrink; it need not prevent safe acquisitions of known positive members.
A source's absence from one worktree is not evidence that its branch, another
worktree or a job no longer needs it. Only an authorized claim owner retires it.

Different policies may track heads, tags, recent accepted releases or explicit
drafts. Do not interpret every Git commit as a new delivered version or keep all
history in a hot place. Art may supply its selected-delivery lineage; Git-only
projects declare a release/tag policy. Git reflog expiry is not a storage rule.

Branch rename preserves source identity; deletion retires only the applicable
claim after reconciliation. Force pushes, rebases and remote pruning need explicit
old/new checkpoint handling, never guessed predecessor order. Offline remote
tracking refs are an observed source snapshot, not universal remote knowledge.

## Creation, staging and merge

| Event | Tig responsibility | What must not happen |
| --- | --- | --- |
| Producer starts | Admit bounded destination/scratch and active job protection | No full duplicate bundle staging or global render lock |
| Bytes complete | Hash/verify and publish immutable object with durable ingest protection | No dependency on a later Git hook to prevent immediate eviction |
| LFS clean/staging | Reconcile selected index pointers; retain uncommitted object admission | Do not assume a committed branch already protects new data |
| Commit succeeds | Adopt exact committed pointer view under configured policy | Do not release index/job/draft protection prematurely |
| Merge succeeds | Resolve actual merged tree and advance main desired checkpoint | No second manual Tig content merge or false placement-complete claim |
| User aborts or resets | Release only explicitly retired claims, report remaining objects | No deleting uncommitted files or shared objects as automatic cleanup |

External Git LFS clean may initially write to LFS's local store rather than Tig.
Account for that store as an admitted source, reconcile staged pointers, and
require explicit draft/ingest protection before managing its removal. Git-only
compatibility cannot depend on every producer using a Tig API. A transfer request
often lacks branch/lifecycle context: it can satisfy object access and admit
bounded ingest, but cannot infer a current/history claim solely from an OID.

Partial staging, merge conflicts and a detached HEAD are separate source views.
Unmerged index stages retain known competing objects until resolved; they do not
define a successful merged bundle. Unstaged content and newly created files need
separate job/draft protection; branch inventory alone is not complete coverage.
Tig never overwrites a user's modified worktree file to satisfy an object request.

## Access and materialization

Support two user experiences behind one object/provider API:

1. Standard Git LFS: its client owns clean/smudge and worktree materialization;
   Tig supplies/acquires exact objects through an adapter. All extra physical
   copies, including LFS's local object store, stay visible in space accounting.
2. Thin worktree: pointers remain in Git checkout; a Tig-aware consumer opens
   a protected immutable object handle or requests an explicit materialization.
   Art is one such consumer, not the only one.

A filesystem mount for transparent virtual paths is an optional later adapter,
not a prerequisite for the minimal Rust library/CLI. Zero-copy and hard-linking
are not default promises: editable worktree writes must not mutate immutable CAS.
Hydration requires explicit scope and budgets; an ordinary branch checkout must
not unexpectedly replicate all archives. Catalog lookup can report a known
location without granting access or reading payloads. Restricted source knowledge
must not leak through shared-catalog responses.

## Moving a version safely

```text
Git commit C3 replaces pointer OLD with NEW
  -> persist new current claim at C3 and configured predecessor retention
  -> ensure NEW in its required places
  -> ensure OLD in its retained places, if not already satisfied
  -> refresh all branch/worktree/job claims and provider custody
  -> retire eligible hot OLD copy
  -> report measured physical reclaim and remaining obligations
```

The merge need not wait for archival transfer unless a configured gate requires
it. Desired C3 / observed C2+partial C3 is a normal pending state, not another
branch version. Copy before removal can require temporary headroom; if unavailable,
use bounded per-object batches or report blocked capacity. Do not weaken custody
to make a full disk appear solved. A plan cannot promise net savings as its peak
space requirement. Reverting C3 reacquires OLD if required and available; it does
not overwrite NEW's retained history or silently download into an offline place.

## Incremental scale and operational modes

Cache pointer inventories by exact immutable tree and attribute/interpretation
context. Apply bounded Git diffs only with a complete known baseline; otherwise
read a named tree scope under budget. Track renames, deletions, attributes changes,
submodule boundaries and extraction coverage explicitly. A tracked submodule is
not automatically enrolled. Unsupported pointer extensions block affected claims.

Routine operation is an explicit short-lived invocation. An optional later worker
can drain durable admitted plans; filesystem watches and Git hooks accelerate
discovery but are never required for correctness. No daemon starts on resume.
Use per-object/place fencing and short catalog transactions; transfer IO occurs
outside catalog transactions. Retry bounded contention rather than hold one lock
through an entire copy. Numerical latency/size targets require workload fixtures.

## Fifteen designed workflow cases

These supplement T01-T25 and CL01-CL10. All remain designed, not executed.

| ID | Trigger | Required outcome |
| --- | --- | --- |
| GW01 | Enroll a Git/LFS repo without Art | Plan from exact pointers and local policy; no full hydration/census |
| GW02 | Same object requested by two unrelated repos | Share permitted bytes; retain independent claims and access scope |
| GW03 | Catalog lost but durable job/claim records remain | Rebuild conservatively; no removal before protection recovery |
| GW04 | Both catalog and an authoritative protection journal are missing | Block removal; do not invent empty protection from branch heads |
| GW05 | Producer completes before staging or committing | Durable ingest/job protection covers the gap; new bytes are not garbage |
| GW06 | LFS clean runs outside Tig and creates a new local LFS object | Reconcile explicit source/index and admit protection; no assumed Tig write event |
| GW07 | Transfer-agent request supplies only OID and size | Resolve exact object, not an invented branch or lifecycle status |
| GW08 | Merge conflicts leave three index stages | Protect known alternatives; unresolved state is not a merged snapshot |
| GW09 | Partial staging, detached HEAD and dirty worktree coexist | Separate views and protections; never overwrite user edits |
| GW10 | Branch rename/delete/rebase and remote prune occur | Authorized source transition; preserve other claims and unknown remote scope |
| GW11 | Attributes change which tracked paths represent LFS content | Invalidate affected extraction context; no stale inventory shrink |
| GW12 | Git merge succeeds while archive is offline | Desired checkpoint advances; placement visibly pending; retain needed local bytes |
| GW13 | Thin and hydrated worktrees consume the same object | Same byte identity; distinct materialization cost and mutation safety |
| GW14 | Demotion needs headroom on a full destination | Budget peak and batch safely or block; no unsafe source deletion |
| GW15 | Hook is disabled and a new ref arrives | Explicit catch-up converges from exact refs without a daemon or duplicate claims |

Before implementation freeze source/claim identity, active-use lifecycle, durable
protection recovery, trusted policy adoption, LFS completion semantics and the
provider's final removal fence. No wire syntax or catalog implementation is frozen.

Bounded source-root persistence is now implemented in
[SOURCE-CLAIM-JOURNAL](SOURCE-CLAIM-JOURNAL.md). It does not make Git and metadata
transactions atomic, discover every source/job, or prove root closure availability.
The complete workflow cases above remain designed until their full exits execute.
