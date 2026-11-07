# Git events and copy lifecycle

Additional producer-applied design walkthroughs from the Git-to-Tig namespace and
redundant-copy requirements. All cases and mitigations remain designed/planned.
See [learnings](LEARNINGS.md) and [lifecycle guide](../guides/GIT-LIFECYCLE.md).

| ID | Starting condition | Next step | Required consequence | Pitfall / mitigation |
| --- | --- | --- | --- | --- |
| LC01 | A producer writes a new large file into an admitted Tig working directory | Seal the completed write and bind the logical path | Protect partial/new ingest; publish exact immutable bytes only after completion and validation | TIG-F-03 / M03 |
| LC02 | A new file has one local copy while backup replication is pending | Keep editing before the first commit | Admit bounded draft protection and expose pending redundancy rather than claim a complete backup | TIG-F-07 / M07 |
| LC03 | The local Git commit contains the new LFS pointer | Advance local committed source claims | Keep exact bytes available locally; commit success does not fabricate remote custody | TIG-F-12 / M12 |
| LC04 | Git push succeeds for a branch containing only small pointer changes | Reconcile remote-pushed lifecycle policy | Schedule configured replica obligations; pointer push does not prove payload upload or independent backup | TIG-F-08 / M08 |
| LC05 | A pull request requires two independent retained copies | Evaluate the exact candidate and custody evidence | Gate only the configured transition; count real failure domains, not aliases or directories | TIG-F-07 / M07 |
| LC06 | A branch merges to main with already replicated working bytes | Durably hand off draft/current/history obligations | Reuse existing copies and retain other jobs; no new physical directory or second content merge | TIG-F-12 / M12 |
| LC07 | A current version becomes previous under a two-copy retention policy | Demote its inactive hot objects | Verify required history/archive copies across independent failure domains before hot release | TIG-F-01 / M01 |
| LC08 | A user checks out an older retained version in a new branch | Create its active working-set claim | Acquire missing hot copies under budget without discarding newer retained versions | TIG-F-12 / M12 |
| LC09 | Another worktree starts on the same project and consumes existing bytes | Register its distinct claim source | Share permitted cached objects, preserve job ownership and account for editable materializations | TIG-F-02 / M02 |
| LC10 | A repository is moved locally while its Tig namespace remains enrolled | Rebind trusted local root and reconcile source identity | Preserve logical paths/owned claims without treating the new filesystem path as a new backup | TIG-F-06 / M06 |
