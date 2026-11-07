# Bounded source CLI refusal guidance

Entry source `ad330ddf9e5635bc66be715f7684dc40396efba5`;
WP-008/009/015/016, REQ-003/006/007/010, SPEC-005/008/014/015,
IF-004/006/013, PKG-003; RUST + INTEGRATION profiles.

Scope: `tig-source` stderr only. Preserve existing exit codes, report formats,
read-only observations, owner/revision fencing, planned transitions and recovery
semantics. No new persistent data, operation, discovery, retry or cleanup.

Refusals retain the existing generic line and add `code=CODE; STATIC_HINT`.
Direct typed source-journal errors distinguish stale revisions, owner mismatch,
operation conflict, checkpoint refusal, unresolved pending transition, unavailable
or invalid references, budget exhaustion, busy authority and required recovery.
JSON parsing errors receive profile guidance. Other errors remain generic.
Classification never interpolates error text or traverses provider error chains.
Unknown errors and strings that resemble journal errors cannot select a category.
Codes are experimental check categories, not a complete root-cause diagnosis.

All hints are static and bounded. No paths, source/owner IDs, request bodies,
credentials or control characters from an error reach public output. Detailed
requests and reports remain private. A refusal can occur after a transition
committed but before receipt publication; guidance must never promise rollback.
Preserve and reconcile the exact saved request through `acknowledge` before a
separate application. Do not weaken trusted minima, reset authority, substitute
owners, remove locks or discard retained obligations to bypass a refusal.

L0: foundation, format, scoped Clippy and diff. L1: typed categories, private
OS/JSON messages, decorated lookalikes and safe fallback. L2: actual CLI stale
apply refuses before intent/report creation with unchanged journal; existing
observation, acknowledgment and transition regressions. Exit: scoped local
evidence and producer privacy/recovery review, then exact-head hosted tests.
Consumer runtime adoption, independent review and full recovery stay separate.
