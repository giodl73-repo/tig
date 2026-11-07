# Product implementation plan

Planning baseline only. No package closes merely because this plan exists.
Implement a bounded product capability, then record verification and validation;
do not create runtime work-package/status/evidence commands as process tooling.

## Delivery order and dependencies

```text
WP-000 backend discovery    WP-001 identity/tree vectors
          \                 /          |
          WP-003 publication        WP-002 private views
                    |
               WP-004 fences
                    |
       +------------+----------------+
       |            |                |
  WP-005 Git/LFS WP-006 placement WP-008 catalog
                    |
               WP-007 local execution
       +------------+----------------+
       |                             |
 WP-009 ref lifecycle          WP-013 remote stores
       |                             |
       |                       WP-014 optional transport
       |
 WP-010 typed extraction -> WP-011 bindings -> WP-012 gates/delivery
       |                                         |
       +------- WP-015 inspector ----------------+
                                                 |
                                  WP-016 private migration rehearsal
```

The diagram shows capability flow, not every prerequisite; the work-package
records are authoritative for exact dependency sets. Typed extraction can follow
identity independently of storage execution. Optional transport/UI never becomes
a prerequisite for basic Git/LFS use or local safe placement.

## Requirement and specification disposition

| Requirement | Specification allocation | Work packages | Current evidence / remaining gap |
| --- | --- | --- | --- |
| REQ-001 / TIG-R01 | SPEC-002/006/010 | WP-001/002/010 | Prototype identity fields are asserted; real hashing/tree/record parity open |
| REQ-002 / TIG-R02 | SPEC-001/004/008 | WP-004/006/009 | Partial pure union/candidates; authority, constraints and durable source claims open |
| REQ-003 / TIG-R03 | SPEC-003/004/008/009/013 | WP-003/004/006/007/013/014 | No-loss/retained custody and live execution unverified |
| REQ-004 / TIG-R04 | SPEC-001/003/004/009 | WP-003/004/007/009 | Snapshot rejection tested; durable fencing/recovery open |
| REQ-005 / TIG-R05 | SPEC-001/002/005/008/009/011/013 | WP-001/006/007/008/011/013/014 | Logical count subset tested; physical resources/reclaim unverified |
| REQ-006 / TIG-R06 | SPEC-002/007/010/012/013/014/015 | WP-000/001/005/010/012/013/014/015/016 | Structural/private-source boundary inspection only; providers/adoption open |
| REQ-007 / TIG-R07 | SPEC-005/014 | WP-008/015 | Shared catalog and fast lookup not implemented |
| REQ-008 / TIG-R08 | SPEC-007/013 | WP-005/013 | Pinned real-client integration not implemented |
| REQ-009 / TIG-R09 | SPEC-001/003/006/007/008/012 | WP-002/003/005/009/012 | Pure desired projection only; Git/index/job recovery open |
| REQ-010 / TIG-R10 | SPEC-010/011/012/014 | WP-010/011/012/015/016 | Canonical donor subset ported under WP-001; full semantics/adoption open |

SPEC-001 is retained/regression-checked by pure and integration packages;
SPEC-015 is ongoing development verification in every package. All other specs
have allocations above; no orphan spec or silently waived requirement is intended.

## Git, worktree and closeout control

One writer per package scope in an isolated branch/worktree based on a recorded
main SHA. Worktrees contain small code/records; shared Cargo and managed asset
stores are explicitly configured. Do not move private media or mutate live
runtime/cache settings to prove a package. Rebase/reconcile unrelated changes
without discarding another lane's work. No default global lock or paused render.

Before code: freeze entry contracts, package IDs, allowed dependencies, fixture
commands, profile-specific L0/L1/L2 and stop conditions. Stop destructive apply
when custody/authority/coverage/fencing is unknown; do not weaken safety to close
a package. Commit/push after relevant checks; PR describes the concrete behavior,
source/evidence and limitations. Merge readiness never implies live-operation
approval. VTRACE-only closeout updates evidence, trace, reviews and pulse status.

No independent role review or user acceptance is implied by producer review.
Backend/format choices become controlled changes with compatibility tests and
named owners. Unknowns and accepted risks stay attached to their implementation.

## Stage review

Producer-applied six-role lenses: pass_with_risk. IP-01: optional semantics/UI
could block basic storage delivery, or a package could hide unresolved custody.
Resolved with independent tracks and explicit entry/closeout gates. No live
provider work starts solely from this plan; each package supplies its bounded
right-side proof and affected consumer validation before adoption.
