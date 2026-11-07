# Branches, scratch, merge and history

Target walkthrough. No Tig CLI/hook or real Git adapter is implemented yet.

## Branch without a second asset directory

```text
Git main C1                 Git feature C2
  logical objects A B         logical objects A C
          |                           |
     current claim               scratch claim
          |                           |
          +-------------+-------------+
                        v
              working place: A B C
              A exists once, used by both
```

The scratch bundle describes the branch's exact object set. Its claim protects
draft content under configured policy. It can use the same working place as main;
it need not allocate a directory or another copy of A. A separate scratch device
is a placement choice, not a different identity for those bytes.

## New files before a commit

```text
admit job/draft -> write bounded output -> hash/verify -> publish immutable object
      |                                                       |
      +--------------- durable protection --------------------+
                                                              |
                                             stage -> commit -> branch claim
```

New bytes need protection before they appear in a committed pointer inventory.
External LFS writes and staged pointers require explicit source reconciliation;
Tig cannot infer their branch from an OID-only transfer. Active jobs and unstaged
work are separate claims. Committing does not silently release them.

## Merge is one content decision

```text
Git merges feature -> main C3
               |
               v
Art, or Git-only pointer adapter, resolves exact C3
               |
               v
desired handoff:
  main-current = A C
  previous     = B
  scratch      = retired only as its obligations are handed off durably
               |
               v
Tig reconciles physical placement; no second manual content merge
```

If A and C are already in the required working place, merging needs no new working
copies. B moves out only after required history/archive obligations are verified
and no other branch, draft or job needs it there.

```text
main: A C        history: B         another branch/job: B
     |                |                         |
     v                v                         v
 working A C     history B              working B still protected
```

Desired C3 can coexist with pending transfers. A configured pre-merge gate can
require particular availability; otherwise the merge succeeds and placement
progress remains visible. A missed hook is recovered by explicit catch-up.

## Revert and branch deletion

A revert resolves another exact snapshot, requesting old objects if necessary.
It preserves newer retained history. A deleted branch retires only its admitted
claim; deleting a name is not permission to delete its bytes. Branch rename,
force push, conflicts and detached worktrees have explicit source identities and
coverage rules in [the workflow contract](../../specs/GIT-WORKFLOW.md).

See [handoff tests](../../crates/tig-core/tests/handoff.rs) for the implemented
pure subset, including the 1,000-object/five-replacement example. The diagrams do
not prove actual Git event capture, durable commit or physical storage behavior.
