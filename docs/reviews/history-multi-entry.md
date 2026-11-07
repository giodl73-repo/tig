# Multi-asset history service entry

Input `89606502149bace28a6bd55e2daa73285afca7b4`, dependent on PR81,
including its fixture-only fresh-probe correction.
WP-003/009/010/012/015, REQ-001/003/004/006/009/010,
SPEC-003/007/010/012/014/015, IF-003/004/007/008/010/011/013,
PKG-002/003/004/006/008. RUST/PROVIDER/GIT/INTEGRATION profiles.

Allowed: reuse the existing history service's request loader/preparer; explicit
batch-check/batch-publish/batch-propose over one captured source, existing
publication backend and one sparse proposal; temporary synthetic fixtures/docs.
No consumer upgrade, user cache, daemon, transport, new store or automatic apply.
Old single-asset operations and publication identities remain unchanged.

Entry: bounded ordered preparation, existing backend admission/readback and Git
proposal APIs. Exit: maximum64 distinct asset/path pairs and256 operations per
batch; each request/profile/document64KiB; aggregate prepared bytes64MiB and4096
blobs, with clamped existing-reader bytes/requests. Validate all items, aggregate
limits and complete Git write set before opening the publisher. Check the admitted
backend's capabilities for every item before the first publication write.
Publish through the existing protocol, independently read back all
changed records/metadata, then create one proposal. A partial publication failure
keeps acknowledged bytes, creates no proposal, and permits exact retry; never
promise storage/Git atomicity. Asset operation keys bind batch namespace and
stable asset ID so reordering or changing a payload cannot hijack another item.

L0 structure/fmt/diff; L1 all existing sequence/legacy tests plus multi-asset
preflight/no-op/late failure/duplicate/scope/limits/retry/capability cases, strict
Clippy and full locked workspace/standalone/assets-only checks. L2 actual temporary
Git and LocalStore, one final sparse proposal with two independently editable
assets, application/reopen/history and retained old selections. Six producer
lenses require exact identities, scoped source/Git fences, bounded resource use,
honest partial publication, no privacy leakage and no inferred removal authority.
Implementation and qualification remain pending at entry; no readiness claim.

## Focused execution and producer findings

Eight temporary Git/LocalStore batch tests pass, including the64-item boundary,
along with all nine legacy/sequence command cases and strict Clippy. Every new
case runs the real command or shared compiled-host seam. A wrong provider place
retains acknowledged objects but fails independent readback and creates no Git
proposal; correcting enrollment safely recovers exact operations. Later-object
capability failure is checked across the full batch before any intent write.

Identity and concurrency lenses found that publish-only paths also need exact
binding: an ID resolving elsewhere in the captured scope is insufficient. Each
declared path is now read from the same pinned Git listing and hash-checked before
preparation, even without proposal settings; the swapped-path regression passes.
The stable per-ID operation namespace preserves exact replay under item reordering.
Placement/custody lenses preserve explicit aggregate bounds and false removal/
custody flags; synthetic acknowledgements do not assert independent protection.
Privacy/adoption lenses retain CLI redaction checks and leave existing single-
operation publication serialization unchanged. These are producer checks, not
independent approval. Full current workspace/compatibility, hosted qualification,
REEL intake and real record acceptance remain separate.
