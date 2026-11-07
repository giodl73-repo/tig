# Producer review: explicit history command

Public summary: consumer identifiers generalized; original notes retained privately.

Input/controls: [HISTORY-COMMAND](../../specs/HISTORY-COMMAND.md). Targets: schema
binary, Cargo feature gate, shared history reader and existing resolver refactor,
command fixtures and guide. Six [role checklists](../../.roles/ROLE.md) applied
by the producer, not independent review or runtime rollout acceptance.

| Role | Finding and disposition |
| --- | --- |
| Identity | Strict small profile/request/raw pins; selected captured record must match expected bytes. Proposal path selects the exact native file within declared scopes. Wrong document, stale record, unknown/oversized inputs and out-of-scope paths refuse. |
| Placement | Explicit already-enrolled publisher also appears as a Flat reader place. No root creation, store discovery, policy weakening or media copying. Check performs preparation only, not publication admission. |
| Custody | Only new metadata receives provider acknowledgements. No success stdout after refusal. Old-protection release/custody/semantic/approval flags remain false. No-op performs no writes or proposal creation. |
| Recovery | Propose preflights before publication and rechecks after acknowledgements. Exact retries reuse the proposal. Publication may survive a failed later Git step; it is never rolled back or treated as cleanup authority. Existing library fault/reopen tests cover that seam. |
| Provider/privacy | The reader is shared with existing history inspection. Lock-induced Busy, missing and corrupt objects remain distinct; a test initially expected Denied for a Windows sharing lock and was corrected to its actual Busy classification. Static messages and pin-only output omit private bodies/paths. |
| Git/adoption | Check/publish/propose leave HEAD/worktree/index untouched; application is explicit. Register/select/configure/record attempts use one stable asset. A recorded attempt does not become a remote publication claim. Backend enrollment and actual consumer project consumer migration remain open. |

Four new real-command tests pass on the final fixture set, covering all operations,
publish/propose/retry/no-op, exact application, status/authoring preservation,
unsupported acknowledgements before writes, bad inputs and blocked history.
Strict workspace Clippy and format pass. Existing history-inspection regressions,
full workspace, standalone/asset-only and exact-head hosted checks are recorded
separately. Source budget is measured; no guard is relaxed.

Disposition: pass_with_risk for the explicit LocalStore command surface. It does
not make that backend the consumer project cache or replace REEL's existing CAS publisher.
The generic SDK host seam is shared; consumer glue and T3 protection stay open.
