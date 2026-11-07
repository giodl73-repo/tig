# Exact-base Git record proposals

Public summary: consumer identifiers generalized; original notes retained privately.

Entry823e15c. WP-002/005/009/010/011/016; REQ-001/004/006/009/010;
SPEC-006/007/010/011; IF-007/008; PKG-004/005. Pulse05, GIT profile.
Allowed: tig-git source proposal module/tests, scoped spec/guide/VTRACE closure.
Existing read-only extraction and standalone LFS remain usable independently.

Entry: qualified native producers can retain small candidate records and exact
old-template pins; no generic Git handoff exists. Exit for this slice: caller-
initiated exact-base proposal commit/ref, ordinary Git conflict handling, bounded
small-file changes and no active HEAD/index/worktree mutation. Not full adoption.

Use Git's immutable commit/tree objects and create-only refs/tig/proposals namespace
instead of inventing multi-file worktree transactions. Caller supplies exact base
commit, portable existing paths, old raw-byte pins, replacement bytes and explicit
commit identity/message. Validate all targets before writing Git objects; preserve
unmodified Git tree entries, rewrite affected ancestors only, independently read
back the proposed tree and then pin it with a create-only proposal ref. Retries
with identical inputs converge. Never overwrite a foreign proposal ref.

The caller reviews and cherry-picks/merges with ordinary Git. Git handles dirty
worktrees, disjoint changes and same-file conflicts. Creating a proposal is not
selection, renderer approval, recipe currency, payload custody or source-journal
adoption. Source/job retained roots remain obligations until separately reconciled.
No automatic hooks, checkout, ref advancement, staging, remote transport or caches.

L0: repository validation, fmt, diff/roles/trace. L1: exact old bytes/path/count/
size/identity negatives; idempotent refs; foreign ref refusal. L2: real temporary
Git repositories, dirty worktree/index unchanged, standard disjoint cherry-pick,
same-file conflict before selection, linked worktree and preservation of other
tree entries. Workspace locked default/all-feature tests, Clippy and standalone
Git/LFS check; hosted Windows/Linux final-head checks before merge. Synthetic only.

Producer Git/adoption and concurrency/recovery role lenses apply. Full legacy consumer
readiness stays open, including canonical native schema validation and handoff
integration, owner recovery, all-family/variant/global claim/accounting/custody/
eviction and reversible consumer acceptance.

## Producer review findings

Method: producer-applied Git/adoption and concurrency/recovery checklists, with
MODEL.md, SAFETY.md and FOUNDATION.md. Exact target: source/proposal.rs and the
temporary real-Git proposal fixtures; no independent/human acceptance asserted.

| Lens | Finding / disposition | Remaining scope |
| --- | --- | --- |
| Git/adoption | Scoped pass: all old pins checked against immutable base before object writes, exact commit/tree readback, distinct proposal ref; Git performs application/conflicts | Native candidate/claim integration and post-merge schema/build gates remain open; generic text merge is not semantic validation |
| Concurrency/recovery | Scoped pass: create-only no-dereference CAS, deterministic retry identity, no index or worktree lock, foreign/symbolic refs refused | Lost replies retain small Git objects/ref for retry; no managed source-owner crash recovery or lifecycle handoff is claimed |
| Identity/privacy | Scoped pass: raw-byte identity separate from Git blob/commit identity; explicit bounded identity; inherited Git routing removed by existing runner | Private record content remains in its caller-selected Git repo; this is not an anonymizing/export command |
| Storage | Scoped pass for metadata only: at most128 small files/128 changed tree ancestors; no payload materialization or cache access | Proposal GC protection is not media custody; existing job/source retention and global coverage remain separate |

Review caught a symbolic-ref redirection hazard: update-ref now uses --no-deref,
create-only expected-zero CAS and verifies that the resulting ref is direct. The
fixture tries both a foreign direct ref and a symbolic ref targeting a branch.
Neither is overwritten or followed. Failure after partial object writes can leave
unreachable Git metadata, but cannot alter the active branch/index/worktree. This
is not a cross-store transaction or power-loss acknowledgement.

Disposition: pass_with_risk for this seam, conditional on executed checks below
and hosted final-head qualification. Full VAL-001/002/003/004/005 remain OPEN.

## Executed qualification

Final Windows all-feature workspace suite passes479 test entries, including13
proposal entries (twelve primary fixtures plus one process helper). A separate
Cargo --list census confirms479; it is not substituted for execution. The final
proposal run passes7.58s alone and9.36s inside the workspace suite. The unchanged
500,000-binding persistence/sparse-history case passes in its210.06s metadata
suite. Workspace all-target/all-feature warnings-denied Clippy, standalone
no-default-features tig-git check and fmt check pass after the hook amendment.
An earlier default workspace run also passed before that private proposal-only
amendment. No donor/media/private cache/runtime changes were made.

Repository structural validation additionally checks exact reviewed-file pins;
those pins are refreshed for these producer-reviewed authored changes. Final-head
Windows/Linux hosted qualification and consumer integration remain required.
