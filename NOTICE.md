# Provenance and scope

Original software and ordinary technical documentation: copyright 2026
Gio Della-Libera, MIT. No private consumer content or assets are included.

Pinned unmodified ROLES/PITFALL development checkers and their upstream licenses
are retained in tools/vendor; pins.json records exact source revisions and hashes.
They are not runtime dependencies. Only checker software and license notices are
vendored; upstream notices also describe separately licensed non-software content.
The root MIT license does not replace upstream terms. TRACKER scaffold conventions informed this repo.
Art's bundle/placement design informed the separation recorded here.
crates/tig-core/src/identity/canonical.rs and tests/canonical.rs derive from
crates/art-core/src/canonical.rs and tests/canonical.rs in giodl73-repo/art,
revision 119c66446f2e95129d8d8f143d60ec6220d2813a (MIT, copyright 2026
Gio Della-Libera). Imports/error plumbing and raw SHA-256 formatting construction were adapted; strict encoding and
legacy semantic framing remain intact. Art and Tig share the copyright holder
and the complete MIT terms retained in LICENSE. No private consumer data moved.
Backend/protocol sources are linked in the decision note.

Embedded capacity journaling uses rusqlite 0.41.0 and libsqlite3-sys 0.39.1
(MIT), with bundled SQLite; upstream SQLite is public domain. getrandom 0.4.3
(MIT OR Apache-2.0) supplies ledger instance nonces. Cargo.lock pins transitive
packages, whose own license terms remain in force; no source is vendored here.
These are library dependencies, not installed database services. Binary/package
distribution must retain the applicable upstream notices.

Allocated-file-data measurement/preallocation uses fs2 0.4.3 (MIT OR Apache-2.0),
also pinned in Cargo.lock. Its filesystem-dependent file-allocation evidence is
not a measurement of all namespace/database overhead or net device growth.

crates/tig-git adapts the CAS path mapping and disposable hard-link handoff
behavior from REEL crates/reel-assets/src/lib.rs and src/bin/reel-lfs.rs,
revision e8fb31c6b21d74f111700d72076bb2a9d2d18176 (MIT, copyright 2026
Gio Della-Libera). The complete MIT terms remain in LICENSE. No private content
or REEL runtime dependency is included. Git LFS protocol references are linked
in specs/LFS-CACHE-INTEROP.md; Git LFS is an external test client.

Git blob identity verification uses RustCrypto sha1 0.11.0 (MIT OR Apache-2.0)
for legacy Git SHA-1 interoperability only; sha2 verifies SHA-256 repositories.
SHA-1 is not a new collision-resistance/security promise. Tig/LFS payload identity
remains raw SHA-256. Dependencies remain pinned in Cargo.lock.

crates/tig-assets/src/variants.rs and tests/variants.rs adapt the sparse resolver
and 14 behavioral vectors from Art crates/art-core/src/variants.rs and tests/
variants.rs, revision 119c66446f2e95129d8d8f143d60ec6220d2813a (MIT, copyright 2026
Gio Della-Libera; full terms in LICENSE). Native slot/Object wire and subject
identity differ. Tig adds project-context admission/provenance; no private data
or legacy Art runtime/record dependency is imported.
