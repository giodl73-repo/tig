# Exact-base small-file Git proposals

Experimental Rust API: `tig_git::source::Repository::propose`, independent of the
optional typed-asset feature. WP-002/005/009; SPEC-006/007. This is an explicit
authoring operation, not a side effect of inspection or rendering.

`Repository::check_proposal` performs the identical source/bounds preflight with
zero Git writes and returns a non-deserializable `PreparedProposal` borrowing
immutable caller bytes. It exposes exact old/new changes; explicit `create`
consumes it under the same operation deadline. The `propose` convenience method
checks then creates. The [verified-cache CLI](GIT-PROPOSAL-CLI.md) uses this seam.

Inputs are one immutable base commit, existing portable logical paths, expected
raw-byte Object pins, small replacement byte strings, explicit Git author identity
and a bounded message. A caller that handles native `.tig` records must separately
validate their definitions, schemas, claims and production context. The generic
operation accepts small binary or text files without interpreting their meaning.

Before writes, all paths must exist as ordinary nonexecutable Git blobs in that
base, expected bytes must match, paths must be unique and limits must hold. No
working file, current index or moving branch supplies an old-byte substitute.
Limits: 1..128 files, 64 KiB old/new bytes per file, 128 distinct changed ancestor
trees including root, 4 KiB message, 256-byte identity fields, 30-second overall
checks and at most1024 wrapped Git commands. A command already in progress also
has Repository's separately bounded deadline (at most30seconds). Existing Git
source listing/count limits still apply. Partial unreachable Git objects may
remain after a failed write; no worktree rollback or payload retirement is needed.

The operation hashes replacement blobs without path filters, rewrites affected
Git tree ancestors, creates a deterministic unsigned commit with the exact base
parent, and reads back both commit bytes and the full bounded leaf listing.
Unchanged blob IDs/modes and submodule/symlink entries must match. It does not
rewrite every payload or use a private index copy. Repository's existing trusted
Git executable, environment isolation, no-replace and no-lazy-fetch rules apply.
Proposal commands disable configured hooks with Git's documented per-command
[`core.hooksPath=/dev/null`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-corehooksPath),
including reference-transaction hooks during ref creation.

Only after readback, create `refs/tig/proposals/<commit-object-id>` using Git's
create-only CAS with `--no-deref`. Same-input retries return the same commit/ref,
including concurrent retries. A foreign or symbolic ref refuses; it is never
overwritten or used to advance another ref. The proposal ref protects this small
Git commit from ordinary GC while present. Another writer can still change/delete
Git refs; this is not a durable Tig owner/payload-custody acknowledgement.

```text
frozen base B + retained candidate record bytes
                    |
        exact old-byte/path/size checks
                    |
         immutable proposal P (parent B)
         refs/tig/proposals/P -> P
                    |
             review with git show
                    |
       ordinary git cherry-pick P / merge
              /                  \
    disjoint changes          same-file conflict
    normal Git commit         Git stops for resolution
              |
    separate source/claim reconciliation
```

Proposing leaves HEAD, active branches, index and working files untouched, even
when dirty. Applying is a separate caller action using Git, with normal dirty-file
checks and three-way conflict handling. Generic Git may textually merge compatible
edits in one record; native schema/semantic/build gates must still validate the
result. A conflict-free merge is not proof of fresh outputs or valid native rules.
The proposal is not automatically the latest selected version. No hooks, external
transport, cache/media IO, approvals or managed source-journal transitions run.

The retained job/source roots remain obligations after proposal creation and Git
application. Removing a proposal ref is not permission to retire those roots or
evict bytes. Canonical native handoff integration, lost-ack owner recovery, lifecycle
claim convergence and complete consumer project adoption remain open.
