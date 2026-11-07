# Producer review: captured history to sparse Git

Input and controls: [NATIVE-HISTORY-UPDATES](../../specs/NATIVE-HISTORY-UPDATES.md).
Targets: optional `native::updates` adapter/export, native test entry and new
update fixtures. Six [role checklists](../../.roles/ROLE.md) are producer-applied;
this is not independent review or consumer acceptance.

| Role | Finding and disposition |
| --- | --- |
| Identity | Expected raw pin is compared to the captured ID selection before metadata IO. Retained old records cannot substitute. Git-absent, dirty/newer source and stale/missing-ID fixtures exercise this boundary. |
| Placement | Revision registration leaves existing selection/status unchanged; explicit selection supplies the project status map. No working/history assignment or remote action is inferred. |
| Custody | Preparation and proposal are not protection. The caller fixture persists new immutable metadata, finishes, reopens and reads back exact bytes before proposal creation. Neither that check nor the SDK asserts owner adoption or independent custody. T3 remains open. |
| Concurrency/recovery | Same-input proposal identity repeats exactly. Independent asset edits merge; same-asset history proposals conflict during Git application, with both candidates retained. Stale expected bytes against a new base refuse without changing proposal refs. |
| Provider/privacy | Existing SQLite metadata port and bounded supplied reader only. Synthetic fixtures contain no media or credentials; no provider/root discovery or automatic initialization exists in the SDK. |
| Git/adoption | Proposal creation leaves HEAD, index and worktree unchanged. Only the caller cherry-picks. Git diffs change one asset file; original Git record and immutable revision remain readable after B replaces A. Optional schema feature preserves the standalone path. |

Four focused real-Git tests pass, including two independent native definitions,
actual metadata finish/reopen, deterministic proposal retries, two competing
proposals and explicit A/draft-B/selected-B Git revisions. The initial fixture
compile needed explicit existing metadata limits; strict Clippy also identified
an unnecessary test clone, corrected without changing contracts.

Final structure/format, full locked workspace, strict Clippy, standalone check
and exact-head hosted qualification are recorded separately as executed. Pure
update/common-history tests remain the detailed operation validation baseline.
This proves a caller-composed path over existing ports, not a finished persistent
CLI, automatic claim handoff, complete delivery migration or safe eviction.
