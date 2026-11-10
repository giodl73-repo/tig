# Staged-index lifecycle producer review

Input195bd0a, implementation51dd2b2. Controls, allowed scope and GIT/PROVIDER
entry/exit/L0/L1/L2 are in specs/INDEX-LIFECYCLE.md. Targets: lifecycle::index,
tig-lifecycle index, shared placement error mapping, strict profile/plan/status
documents and fifteen focused Git/CLI/store entries. No new event encoding,
provider, hook, enrollment discovery or final-deletion capability is introduced.
Method: one producer applying all six .roles checklists; this is not independent
review or human approval. Owner of every row: Tig maintainer using the named lens.

| Finding / role | Severity and disposition | Evidence and remaining limitation |
| --- | --- | --- |
| IL-01 Identity / namespace | High: conflict stages cannot collapse into a fabricated merged path map. Build a deduplicated payload-identity obligation tree with distinct index context, listing, association and unresolved-stage counts; require an independently enrolled Draft source/worktree. | Real conflicted index retains base/ours/theirs; linked worktrees have independent sources. Existing merged extraction is unchanged. Assigned worktree IDs/private root enrollment are trusted; copied/relocated Git directories are not automatically authoritative. |
| IL-02 Placement policy | High: unsupported capture must not shrink acknowledged obligations. Only Unmerged issues are admitted as known protection alternatives; every other issue holds. Preserve current/predecessor policy and all-source demand. | Invalid pointer/stale index holds leave the old draft protected. Job demand keeps old working bytes; job-end/no-op reconciliation demotes through the same policy. Automatic commit/draft settlement, branch rename/withdrawal and unknown native/unstaged producers remain separate required work. |
| IL-03 Storage / custody | Critical retained boundary: index adoption is metadata, not custody. Compose existing accounted copy/retention/handoff and all-namespace/keeper retirement, with independently default-false permissions. | Actual CLI/three temporary stores verify copies and old-byte readback after ordinary demotion. Retained backup is never deleted. Real provider/domain, large-file and physical free-space qualification remain open. |
| IL-04 Concurrency / recovery | High: bind exact index listing and prior source revision/state; finish/read back metadata before atomic adoption; recheck before and observe after. Durable acknowledgement and current-source truth stay separate. | Drift before adoption refuses; drift after adoption reports false. Two actual child exits recover verified metadata/adoption and original receipts; offline status and no-op replay add no commands. A past capture does not protect future unmanaged Git/producer writes; no Git/SQLite transaction or global cache lock is claimed. |
| IL-05 Provider / privacy | High: validate private owner/authority/policy/seed/provider roster and canonical namespace root before selection. Missing roots refuse without initialization. Later provider failures preserve selected truth in a held result. | Wrong owner, missing provider and default-disabled copy fixtures pass; profiles contain only synthetic machine bindings. Hostile filesystem, restored authority, dynamic roster migration and live adoption remain unqualified. |
| IL-06 Git / adoption | Medium: expose index operations separately and keep committed-ref/GitHub behavior unchanged. A draft byte set always reports merged_selection:false and unknown storage in selection-only results. | Actual CLI plan/apply/resume/status and physical reconcile execute. No filters/hooks/Git writes or working payload reads are added. Existing reader/event compatibility and whole-workspace regressions remain separately qualified; no full priority-6 closure follows. |

The first fixture failures were corrected, not accepted: the protection helper
mistook an absent-claim result for an error; it now explicitly unwraps and tests
protected membership. The seed fixture initially bypassed the existing flat
layout's objects directory; it now uses Layout::object_path. Byte readback uses
the actual protected reader. None required weakening provider/admission behavior.

Disposition: pass_with_risk for bounded implementation and local qualification
under EVID-095. Default/all-feature workspace, strict Clippy, standalone Git,
shipped Node/restore, explicit artifact qualifiers and L0 pass. Hosted Linux/
secrets pass at51dd2b2; Windows/current-head checks remain pending. Source history
and journals retain finite bounds; sustained unattended staging requires explicit
owner settlement/continuation rather than dropping old obligations. No full
requirement, intended-use VAL, independent/human acceptance or live-operation
authority is closed by this producer review.
