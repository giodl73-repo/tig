# Explicit history command

Input b594974e25b6616458d9b77c368c2d79b284bec1, T2 of
[version/delivery landing](../docs/VERSION-DELIVERY-LANDING.md).
WP-003/009/010/011/012/015, REQ-001/003/004/006/009/010,
SPEC-003/007/010/011/012/014/015, IF-003/004/007/008/010/011/013,
PKG-002/003/004/006/008. RUST/PROVIDER/GIT/INTEGRATION profiles.

Scope: schema-feature `tig-history check|publish|propose PROFILE REQUEST` over
existing captured preparation, update publication and sparse proposals. Share
the bounded existing history reader with tig-resolve. No provider initialization,
daemon, network, implicit branch/index/worktree change or cleanup.

Entry: exact source, pure updates and admitted publication contracts. Exit: real
commands register revisions, explicitly select, configure targets and record
delivery through the same native contracts. Strict bounded private input files,
raw document pins, expected source and argument checking precede writes. Unknown
or denied reads refuse without success output. Check performs no writes.

`publish` uses an explicitly already-enrolled LocalStore. Its Flat object root
must also occur explicitly in the reader profile; no second private unreadable
metadata store. This prototype backend is not enrolled automatically for consumer project.
REEL's existing CAS PublicationBackend can host the same library operations;
consumer glue and qualified runtime adoption remain separate obligations.
No fallback from an unsupported backend/ack level to a weaker one.

The strict profile schema is tig.history-command.experimental-1. Fields:
git_executable, repository, base, records, definitions, asset, expected, dialect;
optional byte_source, limits (existing native capture limits), publication
{root,operation,owner,required}, proposal {path,identity,message}.
[Admitted hosts](HOSTED-HISTORY-COMMAND.md) may additionally use optional publication
layout/settings; defaults preserve the local command profile. All paths are
explicit; document paths resolve relative to the request file. Profile, request
and raw document are each at most64KiB. Raw change documents require exact
{path,expected} pins. Fixed common history limits apply (4096 documents/64MiB/
65536 references). No credentials, locators, authored bodies or local paths in
stdout/errors. Output contains pins/counts and explicitly false authority flags.

Request operation is configure-delivery (document), register-revision (document,
coverage), record-delivery (document,coverage), or select-revision (revision,
representation,subject,status,streams). Selection never copies approval/status;
the supplied complete status map is an explicit caller decision.
The optional `sequence` operation takes `changes`, an ordered list of one to64
of those existing operations. Nested/empty/excessive sequences refuse before
document reads or provider construction. Each document retains its own exact pin
and64KiB ceiling (at most4MiB aggregate document input); the existing fixed
metadata byte/document limits apply across the sequence. Every intermediate
candidate is validated in memory, then only the final original-base result is
published/read back and proposed once. A failed later step publishes nothing.
This does not accept multiple asset IDs or implicitly apply a proposal. Original
single-operation requests retain their existing preparation/publication ordering.
When a proposal path is supplied it must be inside the declared record scope;
capture that exact native file and require its ID/expected pin, rather than
editing an unrelated file merely because it contains equal raw bytes.

Check validates preparation and any supplied proposal path/base/identity, without
opening a publisher. It does not establish publication readiness. Publish
requires publication configuration. Propose additionally requires proposal
configuration, performs Git preflight before publication, then repeats preflight
and creates a separate immutable proposal after acknowledgement. It never applies
that proposal. No-op writes nothing, including no proposal. Retrying checks all
inputs again; saved stdout is not a capability. If proposal creation fails after
publication, acknowledged bytes remain and exact retry is safe. Old source/job
protection remains required, with no release inferred from this command.

L0 structure/diff/fmt and six-role review; L1 focused negative/positive real CLI,
locked full workspace/strict Clippy/standalone and assets-only build; L2 temporary
Git and existing Flat provider: check unchanged files/refs/store, publish/reopen,
propose/retry/application, old selected history, missing/denied/corrupt sources,
stale/wrong document pin, unknown fields, invalid mode, no-op and unsupported
ack. Full consumer project producer/claim integration and T3 remain open.
