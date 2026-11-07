# Explicit single-stream restoration

Entry: 9252a8a1822e99155e5d2bc9bdd4a96476cfe22f. WP-005/011/015,
REQ-001/003/006/010, SPEC-004/007/011, IF-005/010/013, PKG-003/004/008.
Scope: one explicit CLI using exact Git native resolution and the existing
verified reader. No new provider, ingest, discovery, hooks, removal or runtime switch.
L0: portable checks, Rust fmt/locked tests/Clippy. L1/L2: synthetic real Git/cache
fixtures, dry run, restored hash, retry, conflicting destination, corrupt/missing
source and destination path refusal. Consumer rehearsals stay in consumer repos.

```
tig-restore PROFILE ID STREAM_SLOT ABSOLUTE_DESTINATION
tig-restore --apply PROFILE ID STREAM_SLOT ABSOLUTE_DESTINATION
tig-restore --selected PROFILE ID ABSOLUTE_DESTINATION
tig-restore --apply --selected PROFILE ID ABSOLUTE_DESTINATION
```

`--selected` chooses the exact selected Object instead of a stream. The flags
may precede the profile in either order; duplicate flags and wrong arity refuse.
It works for a primary whose streams array is empty. A literal stream named
`selected` remains a stream when passed as STREAM_SLOT without the flag.
An unselected/planned record refuses; it never falls back to another stream.
The shared verified copy, destination and publication rules below are unchanged.

The resolver profile is the same experimental-1 profile used by tig-resolve,
with required byte_source. Default verifies source and previews without writing.
Apply holds the verified source reader through copy, rechecks SHA-256/size while
writing a private sibling, syncs it, and publishes by an atomic no-replace hard
link. All destination parents must already exist. Existing identical files are
verified and reported already-present; different files and link/reparse paths
refuse. Unsupported hard links refuse rather than weakening publication.

Normal failures clean only the invocation's private partial. Abrupt termination
may leave a .tig-restore-*.partial; it is never a selected object or automatic
cleanup target. The source and Git state are unchanged. This is not a journalled
multi-file transaction, quota reservation, portable crash-durability guarantee,
or custody/eviction authority. A lost reply can retry; a verified existing target
is success. The caller owns destination capacity and explicit path authorization.
Roots and destination namespace are trusted local filesystems, not hostile
mutation sandboxes. Parent checks do not fence adversarial directory swaps.
Reader verification limits remain those of EXISTING-BYTE-READ.md; copy has the
exact pinned byte bound, but no interruptible filesystem deadline.

Before publication the synced private file is independently reread and checked.
`tools/restore_project.py` is explicit operator orchestration, not a provider or
installed runtime. It reads `tig/project.json` and each listed record from one
exact Git commit, admits 1..1024 records and at most 8 GiB, checks portable paths
and declared destination prefixes, and invokes tig-resolve for all requested
objects before creating any consumer directories. Apply then delegates every
copy to tig-restore. Failure midway leaves earlier verified files available for
retry; no multi-file rollback, complete dependency-closure or consumer semantic
validation is claimed. Definitions/records remain under tig/definitions and
tig/records. Machine paths and the temporary resolver profile stay outside Git.
On Unix cooperating immutable writers are required; strict Windows protection
is used on Windows. Git, Python 3.10+ and explicitly supplied CLI binaries are
prerequisites. No automatic install, transfer, Git adoption or deletion occurs.

Exit: scoped fixtures pass and limitations are retained. Full restoration,
consumer runtime adoption and storage safety remain OPEN. Producer review is
recorded separately; it is not independent or human acceptance.
