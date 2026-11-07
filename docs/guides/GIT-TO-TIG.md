# From Git files to Tig-managed bytes

This guide teaches the intended system. Tig currently has a small pure placement
prototype; the native records, merge tooling, LFS adapter, providers and render
engine shown here are not implemented yet. Hash labels are abbreviations, not
literal valid pointers. The design-1 fixture envelopes are not a released format.

## The mirror: Git branch operations become Tig placement transitions

```text
             GIT                               TIG
      authored file versions            exact byte copies / lifecycle

      branch from main ----------------> share immutable base objects
      edit a file ---------------------> private new bytes; old bytes intact
      commit --------------------------> pin bindings; protect that source view
      push ----------------------------> acquire/verify configured remote copies
      merge to main -------------------> hand off current/history obligations
      retire branch -------------------> release its claims; preserve other users
      revert / checkout old -----------> acquire old bytes near the active work
                                              |
                                 policy + observed copies + all claims
                                              |
                                 copy / verify / promote / demote
```

This is the central intended experience: work normally with Git; admitted rules
make the large-byte lifecycle follow. One Git decision drives reconciliation,
not a second manual Tig content merge. A logical transition can require zero
transfer if the right copies already exist. Physical moves are copy/verify/release
transactions and can remain pending; Git events cannot fabricate storage success.

| Git operation | Tig mirror | Example physical consequence under project rules |
| --- | --- | --- |
| Create branch/worktree | New owned working/draft view shares the base | Keep existing local objects; acquire only requested missing bytes |
| Commit replacement | Source snapshot binds new object and retains predecessor obligations | New bytes are protected; old bytes are not overwritten |
| Push branch | Remote branch/CI placement obligations become applicable | Upload missing LFS/provider objects and verify actual completion |
| Merge branch | Adopt the actual merged records and hand off source claims | Ensure current copies; retain predecessor history; assess unnecessary hot copies |
| Delete branch | Retire that branch's claims after reconciliation | Release eligible copies only if no job/other branch/retention claim needs them |
| Checkout/revert | Pin an old or different source view as active | Re-promote missing bytes from retained stores under budget |
| Rebase/force-push | Reconcile exact old/new source checkpoints | Preserve active work and retained objects; do not guess lineage from timestamps |

Rules are explicit configuration: which roots stay nearby, how many independent
copies are retained and when old versions leave working storage. The selected
bytes come from Git pointers/.tig instances; filenames and file presence do not
choose the version. Rendering remains a separate derived job, so source merges
can accumulate fixes before a later catch-up batch.

## 1. Define a type; declare an instance

```text
SMALL FILES IN GIT                         LARGE BYTES IN TIG

definitions/cel.json
  fields, stream groups/order,
  input/output relationships
              |
              | exact definition pin
              v
scene-04/cel-b.tig ----------------------> selected object/result B1
  id: cel-b                                        |
  authored: prompt/settings                       +--> nearby cache
  streams: exact byte bindings                     +--> retained archive
  selected: exact result pin
```

The .tig file is instance data. Its separate definition governs what that data
means and how to validate it. You can create the instance before generating an
image: a planned asset has no selected result yet. Physical paths, credentials
and transfer progress stay out of the record.

Git can also store standard LFS pointers directly. Those users need no typed
asset definition. Both authoring paths reach the same exact-object storage layer.

## 2. Two cels share one cue

```text
Git scene-04/
  scene.tig ---------> cel-a.tig -------> image A1
          |          cel-b.tig -------> image B1
          +--------> cue.tig ---------> recording Q1
                          ^
                     shared by both cels

Resolved working view:       A1 B1 Q1
Required working copies:     A1 B1 Q1     (not Q1 twice)
```

Scene meaning is declared by its type/native adapter. Bundle membership is
derived from those references; no second handwritten inventory is required.
Each view chooses exact source context: dirty working records, staged index,
commit or PR merge candidate. A build freezes exact resolved selections.

## 3. A job writes new bytes

```text
current view                  admitted job-private output
  cel-a.png -> A1              job-42/cel-b.png
  cel-b.png -> B1                        |
  cue.wav   -> Q1               complete / hash / verify
                                        |
                                        v
                                 immutable candidate B2
```

The job reads shared protected inputs and writes only its outputs. A bounded
private directory is sensible; copying the whole previous scene is unnecessary.
Writing a file is not adopting it. Tig protects admitted incomplete/new work,
and does not present partial bytes as a complete immutable object.

## 4. Adopt the candidate; Git sees a small edit

```text
cel-b.tig BEFORE                 cel-b.tig AFTER
  definition -> D                 definition -> D
  authored -> old settings        authored -> changed settings
  selected -> B1                  selected -> B2
                      |
                      v
             Git diff of one small record
```

The adopted result must correspond to its declared production provenance.
Expected-selection checks keep a late job from silently replacing a newer choice.
For direct LFS authoring, the standard OID/size pointer changes at the tracked
file instead. In both cases B1 remains immutable; policy determines its retention.

## 5. Commit, push, PR and merge

```text
LOCAL GIT                 REMOTE GIT / CI                 TIG STORAGE

new .tig + bytes ---------------------------------------> draft/job protection
      |
git add / commit C1 ------------------------------------> exact source claim
      |
push branch ------------> small records/pointers
      |                         |
      |                    configured LFS transfer -----> ensure required bytes
      |                         |
open/update PR ----------> exact merge candidate
                                |
                         definition + semantic checks
                                |
                         merge accepted M1 ------------> current/history handoff
                                                              |
                                                 copy -> verify -> eligible release
```

Git lifecycle changes desired obligations. It does not prove transfers completed.
CI can use its own local cache and acquire missing objects through the configured
LFS/provider path. Active CI reads have protection independent of branch names.
Required server checks enforce target policy even if a local hook is bypassed.

## 6. Two branches fix independent inputs

```text
                    base: A1 B1 Q1
                      /         \
audio branch                         cel branch
  cue.tig: Q1 -> Q2                    cel-b.tig: B1 -> B2
                      \         /
                       merged M2
                       A1 B2 Q2
```

Independent leaf records can merge without central-list contention. Editing the
same record requires resolution where choices conflict. A .tig record can contain
several stream bindings, and the user can explicitly compose them:

```text
resolved record
  stream-1 <- ours
  stream-2 <- ours
  stream-3 <- ours
  stream-4 <- theirs
  stream-5 <- theirs
       |
       v
validate pinned shape / groups / order / declared dependency rules
       |
       +--> valid selection, including unusual combinations
       +--> conflict / unknown / explicitly admitted draft disposition
```

Tig does not reject a mixture merely because streams came from different jobs.
It checks only declared constraints. Nor does successful JSON text merging prove
those constraints hold. A merge tool should explain semantic conflicts before
the candidate is accepted, not surprise the user after merging.

## 7. Merge source truth now; render next week

```text
MERGED SOURCE M2                    LAST PRODUCED EPISODE
  cel-a -> A1                         R1 built from A1 B1 Q1
  cel-b -> B2                                |
  cue   -> Q2                                |
       |                                     |
       +---------- compare consumed fingerprints --------+
                              |
                    source: valid
                    episode output: stale
                    last delivery: previous R1
```

That is a legitimate state. Source-consistency and ready-output/delivery gates
are different. A project can accumulate bug fixes without rendering every merge.
The old result retains its true old inputs; its provenance is not rewritten.

When requested, a catch-up run freezes one chosen source snapshot:

```text
requested deliverables + source M9 + definitions/profiles/producers
                              |
                      resolved exact build view
                              |
                    current results? reuse them
                    stale results? explain why
                              |
                    render affected targets only
                              |
                    verify -> adopt -> deliver
```

Fixes after M9 belong to later demand, not a running job's mutable inputs. Queued
requests can coalesce before admission; already-started work cannot be undone
by calling it a single batch. Included-fix reporting needs exact source/issue
links, not a guess from filenames or dates.

## 8. Current, history and replicas

Illustrative project policy, not a built-in cloud configuration:

```text
current selected scene V2 -> A1 B2 Q2
previous scene V1         -> A1 B1 Q1

local working:  required active inputs, shared across branches/jobs
remote LFS:     admitted branch/CI objects
Cloudflare:     configured current selected content
Amazon S3:     configured retained history
```

The scope of current is explicit: current source inputs and current delivered
episode can differ while rendering is pending. Do not publish R1 as if it were
rendered from M2. Copies can satisfy several compatible claims, but aliases on
one disk do not count as independent backups.

```text
V1-only hot copy is no longer needed
             |
     required history/backup copies already verified?
             |
       yes --+--> recheck jobs/claims/authority -> eligible hot release
       no  --+--> keep source; acquire retained copies or report blocked
```

The configured number of independent copies governs safety. Moving bytes to one
history location alone does not satisfy a two-independent-copy policy. Releasing
a cache replica is different from authorizing final deletion of retained content.

## The three authorities

```text
definition says:        what an instance means and what combinations are valid
Git .tig record says:   which instance values and byte bindings were chosen
Tig observations say:  where verified copies are actually available/retained
```

Rules connect lifecycle to placement. File presence alone decides none of these.
See [native records](../../specs/NATIVE-RECORDS.md), [byte worktrees](../../specs/BYTE-WORKTREE.md),
[safety](../SAFETY-PROMISE.md) and [current evidence](../vtrace/EVIDENCE.md).
