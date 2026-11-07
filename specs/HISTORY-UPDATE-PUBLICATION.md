# Publish prepared history through an admitted provider

Input 33f0dd76ab5ac939656d5ed76f4122be7eb3c23a; T2 of
[version/delivery landing](../docs/VERSION-DELIVERY-LANDING.md).
WP-003/009/010/011/012/015; REQ-001/003/004/006/009/010;
SPEC-003/007/010/011/012/014/015; IF-003/004/007/008/010/011/013;
PKG-002/003/004/006/008. RUST/PROVIDER/GIT/INTEGRATION profiles.

Scope: optional native helper over existing `tig_store::publish` and
`PublicationBackend`, bounded synthetic provider/Git fixtures. No new cache
engine, provider, file-copy logic, root discovery, live IO or automatic retries.
Entry: an opaque pure `edits::Prepared`, explicit operation/owner/ack level and
an independently admitted backend. Exit: every newly prepared metadata object
and candidate record has its own supported publication acknowledgement before
the helper returns a sparse Git replacement. A no-op performs no provider calls.

Publish a bounded immutable batch-intent descriptor first. It binds the expected
record, candidate, ordered new metadata, owner, operation and required ack level.
Its operation key depends only on the explicit operation namespace; different
reuse therefore conflicts before child publication, including owner/level/list
changes. Child operation keys are fixed ordinals in that same namespace. Source
bytes stay caller-owned until all acknowledgements return. Preflight every
object against provider capabilities before any intent or object write.

The descriptor acknowledges its own bytes, not batch completion. Interrupted
batches return no successful result. Caller retries the same operation through
the existing publication protocol, which re-verifies acknowledged objects and
recovers exact outcomes. Do not infer success merely from the descriptor's
presence. No implicit rollback, cleanup, removal or old-root release on failure.

Only new metadata and the record are published. Old history, payloads, receipts
and witnesses are not recopied or freshly verified by this helper. The batch is
not an archive of the full graph and does not establish independent custody,
domain validity, delivery, selection, source adoption or claim handoff. No-op
results acknowledge no publication. T3 still owns replacement protection and
release of temporary roots. Provider trust and actual reader layout remain
explicit: normal readers must resolve these exact objects through the enrolled
provider; the fixture tests the existing Flat layout, not a private second store.

L0 structure/diff/fmt and six-role review. L1 locked package/workspace tests,
strict Clippy and standalone Git compile. L2 actual temporary LocalStore, reopen,
ReadCache/normal history CLI compatibility, same-operation retry/conflicting
reuse, unsupported admission before writes, partial failure/retry and no-op.
Finish with real sparse Git proposal/application and frozen prior lookup.
