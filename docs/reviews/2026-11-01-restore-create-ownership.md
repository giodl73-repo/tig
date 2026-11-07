# Restore exclusive-creation ownership

Entry main `136ba0c396ef75a65b3c0fbc69331930b15aefc3`.
WP-005/011/015/016, REQ-001/003/006/010, SPEC-004/007/011/012/015,
IF-005/008/010/013, PKG-003/004/008; RUST + INTEGRATION profiles.
Allowed scope: shared restore private-file creation and regression fixtures.
Public profiles, copy/publication behavior, provider admission and cleanup
authority stay unchanged. Synthetic fixtures only; no live cache removal.

The previous implementation constructed its path cleanup guard before opening
the private file with `create_new`. An exclusive-create refusal then dropped the
guard and attempted to unlink a path this invocation never created. The new
creator opens exclusively first and returns a cleanup guard only after success.
The caller keeps the output file binding after the guard binding so error-path
unwinding closes the writer before cleanup.

A deterministic synthetic collision reproduces loss of the existing fixture
under the original ordering. It is not evidence that a random-name collision or
data loss occurred in a consumer cache. After correction, the existing file's
path and bytes must remain; an ordinary failed copy must still remove only its
new private file and preserve unrelated files. Existing project and single-copy
CLI regressions remain required.

Producer identity/namespace and storage/custody review: the authority boundary is
successful creation, not intent, filename or an error value. Source read guards,
independent private-file verification and no-replace hard-link publication remain
unchanged. This fixes current-invocation ownership only; authenticated orphan
attribution, durable interrupted-attempt cleanup and eviction remain open.

Entry/exit: L0 structure/fmt/Clippy, L1 deterministic creation/copy failures,
L2 project/restore CLI synthetic integration. Local verification, exact-head hosted
qualification, runtime installation and consumer adoption are distinct obligations.
This is producer review, not independent or human acceptance.

Local verification passes six library tests (including the two ownership cases),
one project-CLI unit test, 42 project integration tests and eight single-restore
CLI tests. Strict scoped Clippy, formatting, repository validation and whitespace
checks pass. The original-order collision failure is retained separately from
the corrected results. Hosted qualification and runtime adoption remain pending.
