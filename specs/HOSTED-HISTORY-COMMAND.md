# One history application service, admitted provider hosts

Input ce585dd827fbd26ae8b5961138fccd99168f1def. Extends T2
[HISTORY-COMMAND](HISTORY-COMMAND.md), with the same WP/REQ/SPEC controls plus
IF-003/004 and PKG-002/003/004/006/008. RUST/PROVIDER/GIT/INTEGRATION profiles.
Scope: extract the existing command orchestration into a schema-feature library
service and keep a thin LocalStore binary. No REEL dependency or new provider.

Entry: captured preparation, supported publication acknowledgements and explicit
source/proposal preflight. Exit: a compiled trusted host supplies its existing
PublicationBackend through a typed factory; all source/request/Git/publication
logic stays in Tig. Factory invocation happens only after validation and Git
preflight, never for check/no-op. JSON settings are data for that admitted host,
not executable hooks or automatic provider discovery.

Backward-compatible optional publication fields: layout (default flat), settings
(default null). Root, operation, owner and required ack remain unchanged. A host
must validate its own bounded settings and reject unsupported enrollment through
static publication errors. The default binary admits Flat/LocalStore with null
settings only; it never interprets other hosts' settings or initializes stores.
The requested root/layout must also be an explicit reader place.

After all new objects are acknowledged, independently read back each new metadata
object and record through that reader and compare exact bytes/pins before creating
a proposal. Readback failure can follow successful publication: preserve all
acknowledgements, return no success result, and do not create a Git proposal or
release old protection. A later explicit retry with a corrected profile is safe.
Report new-metadata readback separately from payload verification/custody.

Expose the shared bounded CacheMetadataReader for SDK/CLI use; maximum individual
metadata acquisition is16MiB (existing receipt ceiling), with smaller caller
limits honored. No arbitrary oversized media read or arithmetic overflow from a
caller-supplied maximum. Keep existing Missing/Pin/Budget/Busy/Denied/Unsupported
behavior and static private error hints. Constructor validates explicit profiles;
it never enrolls or writes any cache.

L0 role/structure/diff/fmt; L1 existing full command/inspection tests, host factory
never-called preflight cases, exact readback success/refusal and unsupported host,
locked workspace/strict Clippy and assets-only/standalone builds. L2 temporary
Git/provider misconfiguration followed by corrected retry; Git refs unchanged on
failed readback. Actual REEL host qualification and runtime adoption are separate.
