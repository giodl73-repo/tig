# Producer review: protected working-owner handoff

Public summary: consumer identifiers generalized; original notes retained privately.

Input/controls: [PROTECTED-WORKING-HANDOFF](../../specs/PROTECTED-WORKING-HANDOFF.md).
Six [roles](../../.roles/ROLE.md) applied as producer checklists; no independent
review, creative acceptance, global custody or live-operation authority.

| Role | Finding and disposition |
| --- | --- |
| Identity | Critical: identical source names/roots in a different journal could retire the wrong authority. Request binds journal genesis, exact source/owner/revision/selections and acknowledgment. Foreign-journal fixture refuses before mutation. Intent claim ID hashes the entire request; phase operation IDs bind replay through existing command bodies. |
| Placement | High: old temporary roots must not remain permanently hot. Exact abort/withdraw leaves the old source predecessor retained-only under role projection. Successor full assignments and all old retained assignments remain protected. Old store grants/physical copies are not removed here. |
| Custody | Critical: publication or metadata readback alone cannot retire working ownership. Compiled provider returns permanent retained-byte guards at the requested acknowledgment level; existing settled LocalStore grants verify actual bytes and revalidate. Missing, weak and corrupt retention refuses. This proves neither independent domains nor global removal authority. |
| Recovery | High: interruption can leak or gap source protection. Old current remains through intent prepare, guard verification and abort. Four phase failures reopen/retry; completed historical replay does not fabricate fresh guards. Changed request reuse conflicts. Another source advances while this source is prepared. |
| Provider/privacy | High: unbounded held files or a new cache engine would undermine adoption. Guard admission defaults to 4,096 with hard 65,536 before journal writes; archive reads share operation bounds. Local adapter enrolls existing stores/ledgers/grants only; its 8 MiB object limit is unchanged. No paths or inventories are printed; no live caches are enrolled. |
| Git/adoption | High: observed source stamps are not eternal leases. Exact current stamps are checked at entry and handoff boundaries; old-source commands fence its revisions. Durable provider guards preserve bytes if other source state advances afterward. Original metadata finish/independent readback is an admitted provider prerequisite, demonstrated by SQLite reopen. No Git application or status promotion occurs. |

Nine actual retained-byte/journal cases pass, including real settled grants,
source-role withdrawal, fresh versus historical acknowledgment, all four phase
failures, weak/missing/corrupt-copy refusal and repaired retry, changed requests,
foreign journal, guard admission, stale successor, independent source progress and
unrelated physical publication while successor guards remain held. Native place
identity cannot be relabelled as another protected destination. Recovery loads
the retained host operation record after volatile request values are lost. Strict Clippy
passes. Full regression/hosted qualification remains separately recorded.

Disposition pass_with_risk for protected desired working-owner withdrawal.
Real REEL-cache protection adapter, native witness interpretation, store-grant
retirement, fresh all-consumer custody/fences, actual reclaim/restore and consumer project
M1/M2/M3/M4 acceptance remain open. Do not create a LocalStore for that migration.
Existing journal lifetime bounds and multi-command growth admission remain open;
capacity refusal is conservative protection, not successful lifecycle completion.
