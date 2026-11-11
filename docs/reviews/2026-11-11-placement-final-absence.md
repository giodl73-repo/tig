# Completed final-absence candidate review

Input81ecc27eba6289f2639174c7872be3c53583f5ee; contracta36aa29 precedes code.
Branch feat/placement-final-absence; runtime candidate is pinned before commit.
[PLACEMENT-FINAL-ABSENCE](../../specs/PLACEMENT-FINAL-ABSENCE.md) names all
controls, allowed scope, entry/exit and L0/L1/L2. This is EVID-124. Full plan active.

## Implementation

A private placement helper recognizes only an unrequired final-capable provider's
validated current final-deleted birth, exact final plan/original job/capacity,
released grants, settled metadata charge and freshly observed physical absence.
It uses the existing final-provider status API and checks the current birth again.
Only that observation yields existing AuthorizedAbsence and a no-effect cache skip.
Required activation retains FinalRetirement; it cannot authorize reacquisition,
release a grant or unlink bytes. Ordinary non-final providers use their existing
observation path without this extra final lookup. No public/provider/event/ledger
format, dependency, cache engine, global lock or metadata expiry is introduced.

## Verification

| Evidence | Actual scope/result |
| --- | --- |
| 24078 | Failed fixture assertion treated historical provider bytes as new loss; API correctly preserves5 historical bytes. Corrected caller checks historical flag plus exact bytes/grants/checkpoints; no receipt contract change |
| 82177/7390 | Fixture setup failures: placement-specific retention request, fresh capacity probe and distinct immutable observation ID were required. Production admission was preserved |
| 10846 |11 of12 focused entries passed, including3 actual exits; offline test incorrectly expected whole-call error instead of per-provider held result. It now requires exact Held(Unavailable) and nonconvergence |
| 63536 | Terminal0:13 entries including existing two-provider regression and inert child helper;3 actual exit77 boundaries within the process case; strict all-feature workspace/all-target Clippy |
| 82636 | Terminal0: all15 native teaching/physical/HTTP/shipped-control regressions on SDK delta before final ordinary-provider fast path |
| 63500 | Terminal0: final fast-path13 entries/fmt and strict all-feature workspace/all-target Clippy |
| Controllers | All81 shipped controller cases pass |
| 24981/61132 | Both TERMINAL0: schema/all-feature workspace/500k/strict.24981 also passes standalone Git/fmt/fresh bins and two persisted nine-checkpoint CLI/catalogue artifacts |

Logs placement-final-absence-{native,matrix,matrix-final,process,qualified,final,
teaching,controls,default,all}.log remain outside Git. Each broad phase writes a
numeric .exit file. Parent runs6457/57936 are terminal; never overlap their reused
targets with these newer handles. No broad/new-head hosted result claimed yet.

## Synthetic validation

| Actual fixture step | Working F | History D | Backup M | Observed result |
| --- | --- | --- | --- | --- |
| Select deleted object with seed denied | absent | deliberately absent | deliberately absent | Final-retirement/protection/source holds; no implicit M acquisition |
| Explicit new M birth and exact retained grant, then ordinary reconciliation | literal asset bytes | absent | literal asset bytes | Actual5-byte M-to-F stream; D records/capacity unchanged; converged |
| Independent status and exact repeat | unchanged | unchanged | unchanged |0 streamed bytes/new effects and unchanged source/ledger checkpoints |
| Old D/M deletion retries at simulated700, beyond old expiry | preserved | absent | new retained birth preserved | Historical flag and5-byte historical provider counter; no new loss/commands |

Backup-only demand with all copy/eviction/reacquisition/cancellation permissions
false also converges with zero effects/ledger/source changes. A later D requirement
keeps exact Held(FinalRetirement). New explicit D birth never inherits old absence;
old deletion retry preserves its bytes/grant and records. Present/cancelled,
partial/unsettled, corrupt/missing proof, wrong job/capacity and offline provider
cases remain held. An actual older ledger backup fails the acknowledged settled
minimum. Child exit77 at Unlinked/Terminal/Settled plus independent reopen proves
only settled final completion is recognized. Clock samples/domains/consent remain
synthetic and establish no hardware or native-time/current-authority acceptance.

## Trace and separate closure

PLACEMENT-FINAL-ABSENCE -> existing validated head/final status + no-effect skip
-> actual three-provider source/copy/final/reacquire/replay -> independent literal
bytes/grants/checkpoints/record snapshots -> refusals +3 actual exits -> EVID-124.
Implementation and focused verification/scoped validation pass; broader/source/
artifact/hosted verification remains pending. Trace is scoped to those facts.
Joined public Git-return/acquisition/restore export, complete course, actual
provider/current-authority/large-file/lifetime/metadata expiry and human/full-
product acceptance remain OPEN. No live root, daemon, viewer/browser or deployment.

Parent runtime81ecc27 PR54 passes local both workspace/500k/strict, standalone,
fresh nine-checkpoint artifact and exact Windows/Linux/secrets43059041378. Parent
documentation-only closeout71d7028 has its own pending CI. Parent qualification
is not evidence for this SDK delta. PR53/ad64f03 passes exact43057224855.

## Six producer lenses

One producer uses all six [.roles](../../.roles/ROLE.md) checklists. These are not
independent reviewers, people, human approval or storage/publication authority.

| Lens / severity / owner | Focused disposition | Evidence remaining |
| --- | --- | --- |
| Identity / critical / WP-003/007 | Bind validated current birth, exact immutable final plan and original admitted capacity/job/allocation; never overlay old terminal on a new birth | 13 focused entries pass; full profiles/hosted/whole-store restore remain |
| Placement / critical / WP-006/009 | Absence satisfies only no-copy requirements; required backup/cache still needs explicit final reacquisition | Actual F/M restore with D absent, required-D and legacy case pass; broader gates remain |
| Storage / critical / WP-003/007/013 | No path-only proof, retained floor waiver or hidden final removal; already-absent skip has zero effects | Literal bytes/absence/grants/checkpoints, all-disabled no-effect and old replay pass; actual providers remain |
| Recovery / critical / WP-004/008/009 | Incomplete/unsettled/present/unknown states hold; old final retries never touch new retained bytes | Injected boundaries,3 actual exits/reopen, expired old replay and actual older-ledger refusal pass; wider authority/lifetime remain |
| Provider/privacy / high / WP-007/015 | Reuse final status, bounded current provider checks and scoped admission; no global lock/private root/new engine | Wrong job/capacity/offline/corrupt cases pass; broader scoped-concurrency/provider adoption remain |
| Git/adoption / high / WP-009/015/016 | Git/source acceptance remains separate from payload restoration; missing-byte teaching must still hold | All15 teaching/HTTP/shipped cases and81 controls pass; standalone/baseline/joined lesson remain |

Disposition: pass_with_risk for focused synthetic behavior. One producer applies
all six lenses, not independent people or human approval. Implementation,
verification, scoped validation, trace and review close separately; remaining
contract/product/current authority/provider/lifetime/human gates are not waived.

## Local broad closeout at runtime923a40c / source8e948d4

All broad phase exits are0; both original handles are terminal0. Fresh final-git-
return and final-reacquire artifacts each record9 checkpoints, complete_course=false;
readonly catalogue validation opens0 payloads and reports automatic_eviction=false.
Runtime code is frozen. Parent documentation closeout71d7028 passes exact
43066253510 on both hosts/secrets. PR55 exact-head43069646518 passes
Windows/Linux/secrets; ready for review. Runtime hosted qualification passes.
Final documentation pins/L0 and latest-source state remain separate gates. No live
root/viewer/browser/deployment/PR merge or complete product acceptance follows.
