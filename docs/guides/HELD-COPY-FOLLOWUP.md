# Finish held copies after historical settlement

Use a fresh follow-up when an earlier paged retirement has finished or been
cancelled, and a held provider returns or a protecting job ends. The original
operation keeps its original action set. This experimental surface creates a
new exact-copy decision and requires a new final-loss approval. It makes no
source-history changes. See [the contract](../../specs/HELD-COPY-FOLLOWUP.md).

```text
tig-storage paged followup describe
tig-storage paged followup plan PROFILE REQUEST
tig-storage paged followup inspect PROFILE DECISION
tig-storage paged followup subjects PROFILE DECISION [CURSOR]
tig-storage paged followup actions PROFILE DECISION [CURSOR]
tig-storage paged followup peak PROFILE DECISION
tig-storage paged followup status PROFILE DECISION
tig-storage paged followup apply PROFILE DECISION
tig-storage paged followup resume PROFILE DECISION
tig-storage paged followup cancel PROFILE DECISION
```

The private `tig.held-copy-followup-profile.design-1` contains:

- `operator`: the existing [paged operator profile](PAGED-OPERATOR.md), with
  independently current complete roster/minima, existing roots, jobs, context,
  clock and optional original age registry. Its witness job and store jobs fund
  the new execution. Old claim-release/final-loss approval fields grant no follow-up authority.
- `original_witness_job` and sorted `original_jobs`: exact bindings of the
  original operation's acknowledged witness. They are separate from new jobs.
- `permissions`: independent `stage`, `apply`, `cancel` flags, all false by default.
- `final_loss`: the freshly reviewed follow-up decision digest. The original
  decision digest cannot substitute. `resume` needs apply permission.
- `run_limits.actions`: default16, allowed1..32 attempted actions per call.

Keep this profile outside Git. Allocate new jobs through the existing enrolled
capacity authority with a fresh space probe when needed. Old outcomes, debits and
spent reservations remain intact. Provider metadata is admitted only for attempted
actions, and prospective reclaim cannot finance admission.

Stage requested copies using the existing `tig-storage paged copies` command
with a separately enabled operator profile. The indexed request shape is unchanged.
A `tig.held-copy-followup-request.design-1` contains `operation`, `original` (the
exact original paged decision), `copies` (that asset-side index) and a new bounded
`window`. Every copy must belong to the original exact object digest/size scope
and consenting owner. A different placement/birth needs review in the new decision.

Plan verifies an actual completed/cancelled original witness and retained scope,
then freezes current protection, provider births/grants and held/eligible rows.
An original intent still running must finish or be cancelled first. Where an
earlier provider attempt remains pending, finish/cancel it before planning another
attempt for that placement. Missing scope, evidence, coverage or authority holds.
Age-aware origins keep their original pinned policy/evidence; a timestamp alone
does not establish project inactivity after reactivation.

The plan response includes `decision`, `decision_id` and a finished/read-back
metadata `minimum`. Save the decision as canonical JSON, update the private
metadata minimum, review its exact rows and admit its new `final_loss` digest.
Staging grants no removal permission. No command approves its own output.

Inspect/subject/action pages read metadata only and reuse decision/kind-bound
bounded cursors. Status and peak need no clock or metadata writer. Peak names
only witness metadata, not complete provider capacity or free disk space.
Apply/resume use a metadata reader, current content-wide protection and the
actual enrolled clock. The approved window stays fixed. A transient new use can
invalidate remaining consent even if it is later aborted; refresh the decision
and approval rather than enlarging the old one.

Cancellation records its own witness stop before source/provider/metadata/
registry/clock cleanup access. It can acknowledge stop with unavailable metadata
and report null counts plus `cleanup_unknown`; repeat explicit cancel when
cleanup ports return. It preserves completed irreversible effects. Cancellation
does not restore deleted bytes or retired history. Completed provider outcomes
are historical and cannot authorize deleting a later same-hash acquisition.
A retry of a cancelled operation can finish cancellation cleanup only. Deleting
remaining copies requires another fresh decision and authorization.

All documents/responses stay at most64KiB and growing inventories remain asset-side.
Real native/current-authority, realistic large-file/provider/latency/restore,
metadata lifetime, agent/UI/Action and intended-use acceptance remain separate gates.
