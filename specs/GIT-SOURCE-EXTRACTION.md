# Exact Git source extraction

Input 07cfa97ce60a0d74f5923aaa96cdad8aad0e994b. WP-005; REQ-001/006/008/009;
SPEC-006/007; ARCH-004; IF-008/009; DES-007; PKG-004; GIT profile.
Entry: strict standard pointers and immutable Tig trees. Exit: real temporary Git
repos establish exact commit/index extraction, partial staging, conflicts,
historical reads, linked worktrees and failures without reading working payloads.
L0 structure/fmt/VTRACE; L1 locked workspace/Clippy; L2 real Git plumbing fixtures.

An explicit caller-enrolled absolute repository and trusted Git executable are
required. Commit inputs are full lowercase SHA-1 or SHA-256 Git object IDs, not
moving branch names. Index capture reads one `ls-files --stage -z` listing;
its content checkpoint hashes the exact listing. It does not assert that the
index remains current. Callers must compare a fresh checkpoint at adoption and
durably protect old/new sources before retiring claims. No refs/index are written.

Pointer scope is explicit, sorted logical directory prefixes. This subset does
not infer scope through attributes or assume every Git blob is an LFS pointer.
All selected entries must be supported regular-file pointers for a complete
scope. Empty scopes are valid only by explicit caller policy; no whole-repository
or global claim completeness is implied. Unsupported paths/modes, extensions,
missing objects, conflicting sizes and nonzero index stages prevent publication.
Known positive alternatives remain available for acquisition/protection only.
Submodules and symlinks are not followed. Files outside enrolled prefixes remain
outside coverage, including unknown uncommitted producer outputs.

Captured source entries name immutable Git blob IDs. Content reads use those IDs,
never mutable worktree paths. A bounded `cat-file --batch` reads pointer-sized
blobs, not hydrated payloads; repeated blob IDs reuse a per-extraction cache.
Git SHA-1/SHA-256 identify Git objects; Tig/LFS SHA-256 identifies payload bytes.
They are never interchangeable. Binding-tree emission is allowed only after
complete conflict-free extraction and uses the caller's metadata NodeStore.

Plumbing disables replacement objects, filesystem-monitor execution, optional
locks, inherited Git routing/config overrides, global/system config and all Git
transport protocols. GIT_NO_LAZY_FETCH is also set, but Git 2.42 does not promise
that newer switch; the empty protocol allowlist blocks transport independently.
No filters, hooks, checkout, network actions, daemon or cache initialization.
The enrolled local repository/executable and local filesystem remain trusted;
this is not a sandbox against malicious executables or hostile filesystem changes.

Listings, entry counts, blob reads, output sizes and subprocess wall time have
caller-visible upper bounds and hard ceilings. Budget exhaustion is an error,
never a truncated complete snapshot. Large native inventory performance and
incremental attribute-aware extraction remain separate proof obligations.
An extraction checkpoint is derived source evidence, not a durable retention
claim, branch owner ID, provider custody proof or permission to evict bytes.

Primary plumbing contracts: [index output](https://git-scm.com/docs/git-ls-files),
[tree output](https://git-scm.com/docs/git-ls-tree),
[batch objects](https://git-scm.com/docs/git-cat-file), and
[Git environment/transport controls](https://git-scm.com/docs/git).

## Executed binding and transfer path

```text
full commit ID -> bounded listing -> scoped pointer blobs -> complete extraction
                                                         |       |
                                             path tree <-+       +-> OID tree
                                                                     |
                                                    explicit LFS profile root
                                                                     |
                                                     verified disposable link
                                                                     |
                                                     Git LFS checkout bytes

index stages 1/2/3 -> known alternatives + unresolved issues -> NO emitted tree
```

A single explicit Repository enrollment owns its private captures; another
handle cannot silently consume them. Worktree roots are required for index reads;
bare repositories support commit reads only. A subdirectory is not admitted as
an apparently complete repository index. Git blob hashes are recomputed from the
Git blob header plus content before pointer interpretation. sha1 0.11.0 supports
legacy Git interoperability, not a new collision-resistance claim. Tig payloads
and binding metadata retain SHA-256 identities.

EVID-024 executes 16 Git fixtures; EVID-025 executes the complete source-to-LFS
path, including historical bytes present in cache but unauthorized by the current
commit selection. Local Git: 2.42.0.windows.2; LFS: 3.4.0. CI records its versions.
Existing EVID-023 fixtures remain three additional real-client cases. Current
workspace total: 278 Windows / 279 Linux entries, including subprocess helpers.
No attribute-aware/global coverage, large-inventory or durable source-claim proof.

EVID-027 adds deterministic selection-context hashes and a Git-to-source-journal
fixture. Context binds view, listing, explicit scope and this interpretation
schema. Consumer policy context must be combined explicitly where required.
Root metadata is persisted outside Git before journal preparation; a separate
branch retains old input obligations after main retires its own predecessor.
