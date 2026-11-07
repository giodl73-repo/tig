# Metadata budget inspection

Entry a378bf6495df742942744ecb10e670caf3bfd320. WP-003/006/016,
REQ-003/004/007/010, persistent-metadata provider, caller admission interface.
Allowed scope: expose validated opening Limits by value on writer/reader; no
new provider format, capacity reservation, cache operation or mutable policy API.

The committed-source capture caller must inspect database bounds before writes
without parsing private provider configuration. The existing readonly open still
validates identity, minimum checkpoint, configured bounds and database size.
An accessor returns that admitted snapshot; it is not live free space or a quota.
Caller must recheck the writer's limits/identity before its admitted write phase.

L0 structure/fmt/Clippy. L1 reader/writer agreement across reopen, detached-copy
mutation, unchanged metadata bytes/checkpoint/metrics and no new provider files.
L2 isolated synthetic caller inspection followed by an ordinary bounded write;
native media/custody/adoption are not this API's evidence.

Producer storage/custody, concurrency and privacy lenses: read-only inspection
does not reserve or lock a cache, modify bounds or authorize eviction. The API
exposes numeric limits only. Existing corruption/foreign/rollback checks remain
the authority boundary. No independent or human acceptance is claimed.

Closeout: EVID-057. SPEC-003/005, IF-005/012, PKG-003. Locked metadata
integration suite: 16 passed (124.15 s), including sparse 500,000-binding history
and recovery coverage. Locked all-target/all-feature workspace Clippy passed
(17.02 s); fmt passed. Producer review found no new provider configuration format,
mutable policy reference or storage authority. Full native workflow, capacity
admission and intended-use acceptance remain open.

Hosted Linux Clippy found the larger reader crossing the large-enum-variant
threshold in the existing LFS authority adapter. Follow-up scope: IF-009, PKG-004,
box that private enum's SQLite reader; preserve open/authority/read-only behavior
and wire format. Local Windows had passed lint; the hosted finding is retained
and requires exact-head cross-platform requalification. Existing client tests
exercise the persistent-metadata authority backend.
