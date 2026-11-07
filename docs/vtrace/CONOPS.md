# Concept of operations

Target workflows, not evidence that providers/hooks exist. Actors may be local
users, producers, CI workers or authorized storage operators. Every flow uses
explicit enrollment, exact source identity, access scope and bounded work.

| Scenario | Actor / trigger | Inputs | Normal path and output | Degraded path / handoff |
| --- | --- | --- | --- | --- |
| CON-001 | Git/LFS user enrolls a project | Declared Git scope, trusted place bindings and policy | Resolve exact pointers; produce indexed availability/placement plan; no Art schema required | Unsupported pointers/access or incomplete scope stays explicit; operator resolves |
| CON-002 | Creator changes one cel/file and merges | Frozen scene/base, two cels and one shared cue; job admission | Private B2 output; seal; adopt leaf pointer; push/CI/merge; hand off current/history claims | Stale job or same-leaf conflict remains unselected; pending placement is not false success |
| CON-003 | Process crashes during copy/handoff/removal | Durable intent, admitted claims, exact objects and provider evidence | Recover/reconcile; preserve acknowledged bytes and required independent custody | Lost protection scope or insufficient fencing blocks destructive work; recovery owner investigates |
| CON-004 | Operator demotes an inactive version | All admitted consumers, retention, capacity and observed copies | Copy missing retained bytes; verify; recheck; release eligible replicas; measure actual reclaim | Archive offline/corrupt or peak-space shortage keeps source protected; no global pause |
| CON-005 | CI and several worktrees run concurrently | Exact branch/candidate/index views and independent job claims | Share eligible immutable bytes; protect active readers; reconcile duplicate/missed Git events | Partition/lease ambiguity cannot justify removal; denied authority remains distinct from absence |
| CON-006 | Advanced user enables typed assets | Generic definitions, variants, bindings and admitted native producers | Resolve semantic refs into exact bytes, budgets/build/review/delivery plans | Native coverage/approval/remote publication gaps block affected action; plain LFS remains usable |

The worked lifecycle is documented in [Git lifecycle](../guides/GIT-LIFECYCLE.md)
and [byte worktree](../guides/BYTE-WORKTREE.md). Current/source work stays near
its active jobs; previous versions retain configured copies in independent
domains. Cloudflare current/S3 history is a consumer policy example, not a core
drive/provider rule. Pushing pointers does not prove cloud payload durability.

## Stage review

Producer-applied six-role lenses, pass_with_risk for workflow design. C-01:
unclear handoff when a merge succeeds but replication is unavailable. Resolved:
source adoption and observed placement are separate states; configured gates
control only their named transition. User acceptance/fault runs remain deferred
to VALIDATION.md and integration packages. No implicit live operation authorized.
