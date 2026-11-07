# Producer review: prepared update publication

Input/controls: [HISTORY-UPDATE-PUBLICATION](../../specs/HISTORY-UPDATE-PUBLICATION.md).
Targets: `native::updates::publication`, its export, provider/CLI/Git fixtures and
README linkage. Six [role checklists](../../.roles/ROLE.md) applied by the producer;
not independent review, consumer rollout or storage-removal authority.

| Role | Finding and disposition |
| --- | --- |
| Identity | One explicit operation first binds the entire expected/candidate/ordered-object/owner/ack intent. Derived child keys cannot silently accept a different batch. Changed owner, level or candidate conflicts before child publication. |
| Placement | Every new object is capability-checked before writes. Only new metadata and the record are published; old history and media are not recopied. Working/history demand remains T3 policy. |
| Custody | Result acknowledges individual new objects at the explicitly supported provider level. Intent presence is not completion; no partial result escapes on error. Independent custody and permission to release old protection remain false. |
| Recovery | Existing publication coordinator supplies guards, verification, exact retries and durable outcomes. Tests reopen before retry after faults before/after child commit; no repair/rollback/deletion is inferred from failure. No new global lock. |
| Provider/privacy | No filesystem writes in the helper: only admitted PublicationBackend calls. Static errors preserve busy/conflict/unsupported distinctions and omit provider paths. Flat reader and actual history CLI resolve the published objects. Other provider admission remains external. |
| Git/adoption | Sparse replacement becomes available after all acknowledgements. It neither advances Git nor selects/approves an asset. The fixture explicitly proposes and cherry-picks; old source/job claim release remains unimplemented T3 work. |

Four new focused tests pass (3.99s on the final fixture set), covering normal
reader/CLI/Git handoff, no-op zero provider calls, reopen/retry, both commit-side
faults, strict conflicting reuse, corrupted acknowledged intent, unsupported ack
before writes, and a later oversized object refusing before the first intent.
Strict all-feature/all-target workspace Clippy and fmt pass. Source/structure,
full workspace/standalone checks and hosted exact-head CI are separately recorded.

Disposition: pass_with_risk for new-byte publication only. LocalStore's admitted
process-restart guarantee does not prove independent or power-loss custody.
Prepared old history still requires existing protection. This supplies the
reusable publisher seam; persistent CLI/profile ownership and T3 handoff remain.
