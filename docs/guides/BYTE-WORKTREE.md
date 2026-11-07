# A Tig worktree for bytes

Target user experience with a tested pure binding-state subset: 22 worktree API
tests cover isolated edits, candidates and conflicts. Mounts, payload publication,
durable ownership and real Git pointer/index operations remain unimplemented.
See the [implementation checkpoint](../../specs/BYTE-WORKTREE.md).

```text
OPEN V1                 EDIT ONLY CEL B             SEAL / ADOPT V2
cel-a -> A1             base: A1 B1 Q1               cel-a -> A1
cel-b -> B1             changes: cel-b -> B2         cel-b -> B2
cue   -> Q1                                         cue   -> Q1
```

You see one complete scene view. Unchanged bytes remain shared in admitted caches.
The renderer writes only new outputs in its private bounded job area. Sealing
makes those bytes immutable; adoption updates the corresponding small Git record
or standard LFS pointer. It does not automatically commit, approve or publish.

## Git carries the choices

```text
Small Git records                 Tig byte worktree
  scene definition -----------+      base snapshot
  cel-a selection ------------+----> shared A1
  cel-b selection ------------+----> B1, then adopted B2
  cue selection --------------+----> shared Q1
```

For ordinary Git LFS, the tracked leaf files contain standard pointers in Git.
For optional typed assets, small records carry useful properties and exact result
bindings. Both lead to the same byte/cache layer. A single scene-wide root pointer
is optional; it must not become unavoidable contention for every independent edit.

## What status should explain

Illustrative information, not current CLI output:

```text
base:             exact V1 snapshot
logical changes:  modified cel-b; unchanged cel-a and cue
byte candidates:  B2 sealed; B1 retained by base/history
Git selection:    B2 proposed, not yet adopted
placement:        working verified; archive pending
```

These are separate dimensions. Dirty authored parameters can make a produced
result stale; a sealed result can be unselected; a selected result can still have
pending replication. One status word cannot represent all of them.

Deleting a path hides it in the new view; it does not immediately delete bytes.
A rename can reuse the same object. Two jobs changing the same cel need explicit
adoption if they disagree; a late job cannot silently overwrite a newer selection.

## Reviewable Git proposals

The experimental Rust `Repository::propose` API accepts small replacements and
their expected old-byte pins against one exact base commit. It writes a proposal
commit/ref while leaving HEAD, index and working files untouched:

```text
base B -------- current branch (possibly with other edits)
   \
    P ---------- refs/tig/proposals/<P>

git show refs/tig/proposals/<P>     review exact record changes
git cherry-pick <P>                ordinary three-way Git application
                                   (conflicts stop for resolution)
```

The Rust API also backs the explicit `tig-propose check PROFILE` and
`tig-propose create PROFILE` commands, with verified existing-cache byte pins.
It is not an installed hook. Actual commit IDs
replace the illustrative `<P>`. The proposal's base is frozen; a late render never
substitutes today's working files for its expected old records. Git performs
application and conflict handling. Native schema/build gates and source/claim
reconciliation remain separate. See the [proposal contract](../../specs/GIT-RECORD-PROPOSALS.md).
The [CLI guide](../../specs/GIT-PROPOSAL-CLI.md) defines its bounded private profile.

Read the [byte-worktree contract](../../specs/BYTE-WORKTREE.md) for state ownership,
merge rules and recovery, and [unified design](../UNIFIED-DESIGN.md) for optional
asset semantics. A future mount provides transparent paths; explicit protected
inputs plus private outputs can prove the model first.
