# Exact native source metadata archives

Entry 0855e7a97a1b51ded9e31c40d89dc9c051abd06f. WP-008/010/011;
REQ-001/006/007/008/010; SPEC-005/007/010/011; ARCH-003/004/005;
IF-002/005/008/010; DES-001/005/007/009; PKG-002/004/006.
RUST/GIT/PROVIDER profiles: L0 fmt/Clippy/structural/canonical VTRACE;
L1 bounded, corrupt, missing and untrusted-provider refusals; L2 real temporary
Git capture -> metadata provider -> reopen without Git, with frozen/current
selection replay and sparse-update sharing. Allowed scope: native source archive
adapter, synthetic tests and control docs. No media IO, cache configuration,
daemon, global locks, retention-claim adoption or runtime switch.

An in-memory captured Git view does not make its interpretation available after
the process exits or Git history is compacted. An explicit metadata archive must
preserve exact raw instance, definition and authored-schema bytes, all selected
ID bindings and explicitly retained frozen records. It must restore those same
selections without consulting a worktree, repository or host-global latest view.
It must not silently promote a frozen record into the current ID selection.

Use the existing bounded immutable NodeStore/Tree ports. Raw source documents
are individual objects, not a monolithic inventory; a small archive descriptor
pins the source checkpoint and an indexed tree. Index entries use hashed keys,
not authored filesystem paths. Identical document bytes and unchanged tree nodes
can be shared between archives. Building an index still visits captured metadata;
this is not an incremental native source index or 500,000-record scale claim.

Experimental descriptor schema: `tig.native-source-archive.experimental-1`, with
`checkpoint`, `tree` (exact Object), `blobs`, `records`, `selected` and `bytes`.
The latter is the total unique raw source-document size. Index namespaces:

| Path | Value |
| --- | --- |
| blobs/SHA256 | Exact raw source document Object |
| records/SHA256 | An instance Object present in blobs; includes retained revisions |
| selected/SHA256-of-UTF8-ID | Current instance Object present in records |

These are metadata tree entries, never physical directory layouts. Restore binds
every instance to its exact definition and authored schema again. Counts and all
namespaces/keys must agree with the complete index. Raw documents/descriptor are
at most 65,536 bytes each. The caller supplies existing native document/aggregate/
elapsed-time bounds and separate TreeLimits for index reads/writes/entries/bytes;
the raw-document counter does not include tree-node/provider allocation overhead.
NodeStore implementations must bound their own allocation and IO; elapsed-time
checks cannot preempt a blocking provider. There is no full-volume reservation.

Archive creation returns a candidate root only. The owner must explicitly finish
its provider, verify reopen/readback and adopt/protect that root using the existing
source/job lifecycle. NodeStore success alone is not a durability acknowledgement.
Failure can leave unreferenced metadata prefixes but must not alter any old root.
No archive GC/removal API is introduced.

Restore verifies every descriptor/index/document identity, finite document/byte/
time and tree budgets, namespace/key shape, counts, current-ID uniqueness and
definition/schema bindings. Missing or corrupt data refuse; no empty fallback.
The caller admits the exact archive root and origin checkpoint. These pins do
not independently authenticate source enrollment or author-admitted schemas.

This preserves native source interpretation only. Opaque selected scene JSON,
control streams, media and other payload bytes remain separately managed objects;
their transitive meaning needs admitted consumer extraction and source/job claims.
Archive contents are private authored data; archive roots/diagnostics are not
sanitized public receipts. Full metadata retention/custody/eviction and production
adoption stay open. Exit requires actual detached-source replay and negative
coverage, rather than treating a list of metadata hashes as retained bytes.

## CI follow-up entry 9bb91a5

Run 40443726783 failed the existing Linux cooperative reader's immediate lock
release assertion (existing.rs:189, WouldBlock); the PR workflow on the identical
head passed Linux. Do not dismiss this as archive success. Expand this package's
allowed scope to the scoped reader destructor and regression fixtures under
WP-004/007, REQ-003/004/007/010, SPEC-004/009, IF-004/012, DES-004, PKG-002.
L1 must prove ending one guard preserves a second independent reader; L2 Linux
must prove a duplicated file descriptor cannot prolong an ended guard's lock.
No new admission, global lock, provider or removal capability is allowed.

Linux flock locks belong to open file descriptions; duplicate/inherited handles
can outlive one File close. Explicitly unlock the cooperative lock on guard Drop,
before closing the File, while leaving other independently opened readers intact.
An unlock error cannot be reported from Drop: ordinary close still follows, and
there is no successful-release/custody acknowledgement to infer. A lingering busy
result remains busy. Windows mandatory sharing protection keeps its existing
close-based lifetime. Reference: [Linux flock documentation](https://man7.org/linux/man-pages/man2/flock.2.html).
