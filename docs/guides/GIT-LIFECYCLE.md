# Git file versions, Tig byte-copy lifecycle

Target design, not an installed Git integration. Git versions file trees; Tig
manages the bytes behind those versions and the lifecycle of their copies.
Tig does not create a competing semantic version history or choose new content.

## One project namespace, several physical places

```text
project namespace + exact snapshot + assets/arrival.png
                             |
                       object hash + size
                             |
                 +-----------+-----------+
                 v                       v
         nearby working copy      independent retained copy
         for active branches      for recovery / long-term use
```

An admitted Tig directory associates project-relative paths with this logical
namespace. It is not permission to put arbitrary absolute paths into manifests.
A producer can write into a managed staging/working area and seal completed bytes;
an import can adopt an existing file under explicit policy. A write watcher may
help discover work, but an event alone does not prove that writing has finished.
Hash/validate completion before publishing an immutable object and path binding.

Git can then store the standard LFS pointer rather than the large payload. The
logical namespace follows the exact file versions; provider-owned physical
locations can change independently. Editable worktree materializations remain
distinct from immutable cache objects and are accounted for as real copies.

## V2 replaces V1 at the same logical name

```text
logical path: assets/arrival.png
Git snapshot C1 -> LFS pointer OLD -> immutable object OLD

job-42/arrival.png -> complete + hash/verify -> immutable object NEW
                                              |
                       explicit/admitted selection with expected C1
                                              |
Git snapshot C2 -> LFS pointer NEW -> same logical path, new exact bytes
```

A render writes only its new outputs into a unique bounded job area, preferably
on the configured destination device. It reads shared inputs through protected
handles; it does not copy the entire previous artifact directory. The job area
and logical path need not have the same physical spelling. A new Git branch or
worktree per render is optional, not a Tig requirement.

Seal the completed bytes before adopting the new path binding. The old object
is never overwritten in place. Validate the expected source/selection when
adopting so a stale job cannot replace a newer accepted result. Art can perform
that semantic adoption; a Git-only workflow stages the new standard LFS pointer.
Neither job completion nor byte registration implies approval or delivery.

Git snapshots keep the historical path-to-pointer mapping. Tig keeps its bytes
under admitted retention; Git history alone cannot recover bytes that every store
deleted. If two attempts produce identical bytes, they share one Tig object.
Separate attempt/provenance records can still distinguish the productions; no
duplicate payload is needed just to give a render another version label.

After adoption, policy changes current/history claims. Existing replicas can
satisfy those obligations; copy only what is missing. Do not remove a replaced
working copy until required retained custody and every other consumer permit it.

## Follow Git events without inventing storage success

```text
local new -> staged -> local commit -> pushed branch -> PR -> merged selection
     |          |           |                |           |          |
     +----------+-----------+----------------+-----------+----------+
                            event/source reconciliation
                                        |
                           durable desired Tig claims
                                        |
                    acquire missing -> verify -> observe
                                        |
                      later supersession / retirement
                                        |
                    retained custody -> eligible hot release
```

| Source transition | Intended Tig lifecycle response |
| --- | --- |
| New uncommitted file | Protect bounded ingest/draft and close access; expose pending replication |
| Staging/local commit | Bind exact index/commit objects and retain independently active jobs/drafts |
| Push | Apply configured pushed-branch retention; verify payload placement separately from pointer acceptance |
| PR | Evaluate exact candidate and any required availability/replication gate |
| Merge | Hand off to selected main/current obligations; retain configured predecessor versions |
| Superseded version | Release unnecessary hot copies only after all required retained copies and other consumers are satisfied |
| Revert/old-version checkout | Re-promote required exact objects while preserving newer retained history |

These are policy triggers, not one universal linear state machine. A commit can
exist on several branches; events can be skipped, duplicated or observed late.
Separate claim sources compose across that history. A Git push, accepted PR or
merge cannot itself assert that bytes reached an archive. Reconcile from exact
source checkpoints, including explicit catch-up when hooks were missed.

## Replication and proximity are separate requirements

The project usually wants more than one retained copy of valuable content. Set
that as explicit count/failure-domain obligations, independently of which working
set should be fast. One working copy can count toward redundancy if the policy
admits its retention; two directories or aliases on one disk are not independent.
An archive must be recoverable, including required representation keys.

Illustrative policy, not a built-in default:

```text
draft:
  nearby: active work location
  retention target: nearby + independent archive
  initial ingest may be redundancy-pending within admitted budget

current selected version:
  nearby: configured active working sets
  retention target: working + independent archive

previous retained version:
  nearby: only if another active branch/job requires it
  retention target: history + independent archive

archival-only version:
  nearby: none unless explicitly requested
  retention target: two admitted archive failure domains
```

Counts, places and gate points are configurable. Do not hydrate every object on
every branch to satisfy a proximity rule: declared active roots and requested
materializations bound the working set. Art can supply selected-delivery roots;
a Git-only client supplies its own path/ref policy. A file shared across current
and previous versions remains one object, with the union of relevant obligations.

The first protected local copy is never evicted merely because replication is
pending. Backup delay can block a configured merge/release gate while allowing
unrelated rendering or admitted additions to continue; no global cache pause.

## What this adds around Git LFS

Standard pointers and transfer interfaces are the integration surface. Tig adds
explicit source-owned retention/proximity claims, indexed cross-place lookup,
replication observations and lifecycle reconciliation around them. Its custom
transfer agent cannot infer those facts from an OID alone and cannot by itself
coordinate an external LFS pruner. Those integrations need pinned-client fixtures.

See [LC01-LC10](../../docs/scenarios/LC.md), [Git workflow](../../specs/GIT-WORKFLOW.md)
and [cache/LFS contract](../../specs/CACHE-AND-LFS.md). None of these diagrams is
evidence that hooks, backups or transfers are already implemented.
