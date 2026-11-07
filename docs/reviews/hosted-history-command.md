# Producer review: one service for admitted history hosts

Input/controls: [HOSTED-HISTORY-COMMAND](../../specs/HOSTED-HISTORY-COMMAND.md).
Targets: extracted history service, thin binary, shared SDK metadata reader,
resolver imports and host fixtures. Six producer-applied [roles](../../.roles/ROLE.md),
not independent review or authority to upgrade consumers.

| Role | Finding and disposition |
| --- | --- |
| Identity | Existing parsing, pins, exact source/path and proposal preflight move intact. Optional layout/settings default to Flat/null for existing profiles; host settings remain explicit bounded data. |
| Placement | Compiled host factory supplies the backend. Requested root/layout must match a reader place; no provider discovery or new engine. Default binary rejects nonlocal layouts/settings. |
| Custody | Independent paired readback of new metadata/record precedes Git proposal creation. New readback flag is distinct from payload/independent-custody/old-release flags, which remain false. |
| Recovery | A mismatched host can publish bytes but cannot create a proposal. Exact retry after correcting enrollment reuses acknowledged operations. No deletion, rollback or old-root release follows a refusal. |
| Provider/privacy | Typed factory errors contain no provider paths. It is not called during check, stale preflight or no-op. Public reader clamps each acquisition to the existing16MiB receipt ceiling, honors smaller limits and handles usize::MAX without overflow. |
| Git/adoption | One orchestration implementation is shared with consumers. No source/claim adoption or live REEL change. Existing command operations remain tested through the thin binary; actual REEL backend qualification is next. |

Six focused tests pass, including all four prior real-command cases and two new
host scenarios: custom settings without factory calls before admission, reader
bounds, wrong-provider/readback refusal, preserved bytes/refs and corrected retry.
Strict workspace Clippy and format pass. Full workspace, existing inspector,
assets-only/standalone and hosted exact-head checks are recorded separately.

Disposition: pass_with_risk for service extraction and verified new metadata.
Provider trust/admission, actual consumer integration and T3 remain open.
