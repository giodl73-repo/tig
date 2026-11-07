# Preview and apply a source handoff

Build `tig-source` with `cargo build --locked -p tig-store --bin tig-source`.
These experimental commands operate on an explicitly enrolled existing journal.
They do not create a cache or journal, run Git, render, transfer, or remove bytes.
The caller admits the source/owner and already protected snapshot metadata.
Owner strings and plan self-hashes are not authentication or signed approval.

```text
PRIVATE_PROFILE --plan--> NEW_PRIVATE_PLAN (read-only preview)
                              |
                    inspect exact request and proposed state
                              |
                           --apply-->
                              | save NEW_PRIVATE_REQUEST first
                              | revalidate and commit journal transition
                              + save NEW_PRIVATE_RECEIPT

interrupted/missing receipt:
  NEW_PRIVATE_REQUEST --acknowledge--> NEW_PRIVATE_RECOVERY_REPORT
                          |
                          + only recorded exact requests succeed
                          + unknown requests never execute automatically
```

```text
tig-source plan PRIVATE_PROFILE NEW_PRIVATE_PLAN
tig-source apply PRIVATE_PLAN NEW_PRIVATE_REQUEST NEW_PRIVATE_RECEIPT
tig-source acknowledge PRIVATE_REQUEST NEW_PRIVATE_RECOVERY_REPORT
```

All new output paths must be absolute, distinct, outside the authority directory
and under existing trusted parents. Files are bounded to 64 KiB. Publication
uses synced owned temporaries and no-overwrite hard links; unsupported filesystems
refuse. Stdout contains counts/booleans, not source IDs, roots, paths or hashes.
Detailed requests, plans and receipts remain private runtime metadata outside Git.

The transition profile has this shape. Replace placeholders with real admitted
values; a fabricated digest or guessed expected revision is not a usable request.
Keys contain only ASCII letters, digits, hyphens and underscores, at most 128 bytes.

```json
{
  "schema": "tig.source-transition-profile.experimental-1",
  "journal_root": "X:/project-state/journal",
  "minimum": {
    "genesis": "<trusted authority digest>",
    "sequence": 0,
    "hash": "<trusted checkpoint digest>"
  },
  "operation": "prepare-scene-v2",
  "command": {
    "kind": "prepare",
    "source": "scene-worktree-01",
    "owner": "project-owner",
    "expected_revision": 2,
    "selection": {
      "root": {"sha256": "<snapshot metadata digest>", "bytes": 1536},
      "context": "<exact source and interpretation context digest>"
    }
  }
}
```

Preview verifies the same ownership, revision, pending-reference and growth bounds
as mutation. Its observed checkpoint and proposed before/after source come from
one replay. A proposed source is not a committed receipt or reserved revision.
An already recorded exact request is reported separately with its historical
receipt. Changed operation-ID reuse refuses even when its new state looks valid.

Apply reads the exact plan request, checks its canonical request hash, and uses
the plan's observed checkpoint as a rollback floor. It re-previews current state
and checks the proposed source against the plan before saving the acknowledgment
request. Unrelated source changes may proceed: no global-head equality or cache
lock is required. The actual write transaction checks all fences again, so a
source change racing this preflight can still refuse. No transaction spans file IO.

If applying fails, the journal may or may not contain the transition. An intent
file alone does not prove execution. If it exists, acknowledge that exact request
into a new report. An unknown request refuses and leaves claims unchanged; inspect
and reconcile the source separately before choosing another explicit application.
If committed, acknowledge recovers the original receipt without repeating it.
Do not alter the saved operation ID or expected revision. Keep stronger trusted
checkpoints: a historical receipt must not replace a later known minimum.

The request is saved before journal mutation, but receipt publication can still
fail afterward (for example disk/full-filesystem errors). Handled failures clean
owned temporary files. Process restart is the tested scope, not cross-volume
atomicity, power-loss durability or hostile-filesystem containment. Re-running
apply requires new output paths; no existing request or receipt is overwritten.

## The version lifecycle

```text
source main: current A
prepare B:   current A + pending B          (protect both)
caller completes the reviewed Git/ref handoff separately
adopt B:     current B + predecessor A      (record selected truth)
retire A:    current B                      (withdraw this old obligation)

other source/branch: current A throughout   (its obligation remains)
```

`adopt` and `abort` require the exact prepare reference from its receipt plus the
current expected source revision. Abort withdraws only that pending obligation.
Withdraw moves a source's current selection into its predecessor list and retains
the source identity/revision; it refuses unresolved pending work. Retire names one
exact predecessor revision. Each is a new, explicitly planned operation with an
independent operation ID; a retry keeps the original ID and command unchanged.

These operations record root obligations, not complete object ref counts. They
do not verify root closure, output currency, production approval, byte availability,
replica custody or all other active repositories/jobs. A retired predecessor can
still be required by another source. Placement and eviction require their own
complete claim/custody checks and current fences; this CLI grants no removal
permission. Full job recovery, compaction and automatic Git lifecycle integration
remain separate work.

## Refusal guidance

The experimental [source diagnostics](../../specs/SOURCE-DIAGNOSTICS.md) add a
static `code` and recovery hint without exposing private request or provider
details. A stale revision calls for inspection and a newly reviewed plan; a
checkpoint refusal must never be bypassed by lowering the trusted minimum.
Owner mismatch, operation conflict and unresolved pending work require their
own reconciliation. Generic failures can occur after a transition committed:
preserve any saved exact request and use `acknowledge` to inspect its historical
receipt before a separate application. A hint does not authorize cleanup or
prove the request rolled back. Existing private reports and exit codes remain.
