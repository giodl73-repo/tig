# Observe a source before deciding what to retry

```text
tig-source inspect PRIVATE_PROFILE NEW_PRIVATE_REPORT
```

Build with `cargo build --locked -p tig-store --bin tig-source`. This experimental
command opens an existing source journal through SQLite read-only access. It
replays the bounded hash chain, verifies the caller's trusted minimum checkpoint,
and returns one consistent source/checkpoint observation. It creates no journal,
cache, owner, worker, network connection or recovery action.

The profile is a small private JSON file:

```json
{
  "schema": "tig.source-observation-profile.experimental-1",
  "journal_root": "X:/project-state/journal",
  "minimum": {
    "genesis": "<trusted authority digest>",
    "sequence": 0,
    "hash": "<trusted checkpoint digest>"
  },
  "source": "my-source"
}
```

Use a real trusted checkpoint from setup or a later acknowledged operation;
the placeholders above are not usable digests. A setup-only floor cannot detect
rollback of later changes it has never witnessed. Keep later observations and
acknowledgments as private control evidence; neither proves globally latest state.
The journal directory must already exist and be absolute. No file discovery or
database repair is attempted; a damaged/hot/unsupported database may refuse.

The report contains the observed checkpoint and named source's owner, revision,
current selection, pending prepare reference and predecessors. An absent source
is null, not a guessed revision or ownership grant. The observation is not a
lease: another writer may advance immediately. Before any later transition,
existing owner, expected-revision, exact-operation and pending-reference checks
still apply. The report does not establish worker liveness, whether a command
executed, root/payload availability, complete claims, recovery or eviction safety.

Stdout contains counts and booleans plus an observed revision or null. Raw IDs,
paths, digests and roots remain in the private report, never ordinary Git. Both
profile and report are bounded to 64 KiB. The report path must be new, absolute,
outside the authority directory, with an existing trusted parent. Publication
uses a synced owned temporary and a same-parent no-overwrite hard link. An
unsupported filesystem refuses; handled failures clean that temporary. An abrupt
crash can leave a small private temporary for explicit inspection. No power-loss
durability or hostile-filesystem containment is claimed.

## Observe several related sources together

For several related sources, use the bounded
[batch observation contract](../../specs/SOURCE-BATCH-OBSERVATION.md):

```text
tig-source inspect-many PRIVATE_PROFILE NEW_PRIVATE_REPORT
```

Use schema `tig.source-batch-observation-profile.experimental-1`, the same
`journal_root` and trusted `minimum`, and `sources` containing 1..64 distinct
explicit source IDs instead of `source`. All returned states share one verified
replay checkpoint; absent IDs remain explicit nulls. This avoids combining
separate observations taken while a writer advances. It does not discover source
IDs or prove job completion/liveness. Large histories may exceed the unchanged
64 KiB report budget and refuse; no truncated success is returned. The same
read-only authority, external private report and no-overwrite rules apply.

## Recover an acknowledgment, without executing a command

```text
tig-source acknowledge PRIVATE_PROFILE NEW_PRIVATE_REPORT
```

Use schema `tig.source-acknowledgment-profile.experimental-1`, the same
`journal_root` and trusted `minimum`, plus `operation` (the original operation
ID) and `command` (the exact original serialized prepare/adopt/abort/retire/
withdraw command). Omit `source`; it is part of the original command instead.
Never generate a new operation ID or update the old command's expected revision
to make a retry pass. Keep the original request private alongside its receipt.

This opens the existing authority read-only and returns only a verified recorded
receipt matching that exact canonical request. Unknown IDs refuse; changed reuse
refuses. No missing transition executes. This resolves the specific uncertainty
"was this exact journal command committed?" without retrying any producer.
For an unknown request, inspect the source and reconcile the interrupted job
separately; an unknown journal operation does not prove a render never ran.

The returned checkpoint and source revision describe the original event, even
if later transitions have already occurred. They are not current state or a
reason to replace a newer trusted minimum. Inspect separately for current state;
all future transitions still require owner/revision/pending-reference checks.
The report explicitly marks the receipt historical and does not verify producer
success, complete claims, payload custody or eviction permission. Private report
bounds, no-overwrite publication and redacted diagnostics are the same as inspect.

For an explicitly admitted metadata handoff, use the separate
[transition preview/application workflow](SOURCE-TRANSITIONS.md). It saves the
exact acknowledgment request before mutation. Inspect and acknowledge remain
read-only and never create missing operations.
