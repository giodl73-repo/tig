# Scenario review: 110 cases and doctrine feedback

Method: one producer applied the six local role lenses and walked each new case
from starting condition through next action to expected consequence. These are
60 additional design cases, making 110 total, not 110 executed integration tests.
The existing 13 Rust tests remain separately scoped mechanism evidence.

| Suite | Cases | Focus |
| --- | --- | --- |
| [Foundation](FOUNDATION.md) | 25 | Membership, identity, custody, capacity and genericity |
| [Cache/LFS](../../specs/CACHE-AND-LFS.md) | 10 | Lookup, publication, pointers and copies |
| [Git workflow](../../specs/GIT-WORKFLOW.md) | 15 | Staging, branch transitions and claim ownership |
| [Recovery and durable protection](CR.md) | 10 | Adversarial state transitions |
| [Races, claims and shared work](RW.md) | 10 | Adversarial state transitions |
| [Git and LFS integration](LF.md) | 10 | Adversarial state transitions |
| [Space, representations and place identity](SP.md) | 10 | Adversarial state transitions |
| [Bundles, transport and scoped knowledge](BN.md) | 10 | Adversarial state transitions |
| [Git events and copy lifecycle](LC.md) | 10 | Namespace enrollment, nearby access and independent redundancy |

## Learnings that change the contract obligations

1. Recovery is part of deletion safety. Torn journals, restored backups and lost
   acknowledgements require durable operation identity and explicit uncertain
   protection scope (P05, I05/I06, F03, M03). A catalog rebuild is not enough.
2. A plan generation is not a reader/removal fence. Read grants, path reuse and
   lost renewal require provider-effective coordination (P06, I03/I07, F04/F05,
   M04/M05). A timeout or global process lock is not a complete solution.
3. Capacity admission must include both other Tig writers and external disk
   consumers. It bounds Tig's work but cannot guarantee a future write succeeds
   (P08, I10, F02, M02). Full-disk recovery remains required.
4. LFS interoperability includes the surrounding client lifecycle: configured
   local stores, prune, attributes and unsupported extensions (P07/P08, I11/I12,
   F08, M08). A transfer agent alone is not safe space management.
5. Identity must survive aliases, mounts, materialization collisions and key
   changes without conflating object bytes with representations (P02, I08/I09,
   F06/F11, M06/M11). Refuse silent substitutions.
6. Event completeness and policy authority govern claim shrink. Hooks, ref order
   or reflog expiry cannot erase another owner's obligations (P03/P05, I02/I11,
   F12, M12). Keep interrupted handoffs conservatively protected.
7. Proximity and redundancy are independent axes. Draft/current/previous/archive
   policy can change where a working copy belongs while retaining multiple
   independent copies (P01/P03/P04, I01/I11/I12, F01/F07/F08, M01/M07/M08).
   Git lifecycle events advance desired obligations, not verified-copy facts.

## The feedback loop

```text
scenario -> expected consequence -> pitfall -> mitigation + owner
                                       |              |
                                  principle       invariant
                                       |              |
                                       +-------+------+
                                               v
                           executable counterexample / recovery proof
                                               |
                               scoped evidence and doctrine update
```

The [mitigation register](MITIGATIONS.md) and JSON learning links cover every
scenario, including the earlier 50. The validator checks IDs, coverage, role and
requirement references, and doctrine link existence. It does not execute these
transitions. A new scenario must add or reuse doctrine deliberately; new evidence
can refine a principle or invariant without erasing stable IDs or past failures.

Next fixture order: interrupted claim handoff and lost publication reply;
reader/removal fence and lease uncertainty; bounded capacity/partial writes;
then standard-client LFS integration. Backend choice remains open.

Byte-worktree follow-up refines P02/I08/F06 and M06: explicit removals must hide
base entries rather than resurrect them on fallback. P02/F12/M12 also require
independent authored leaf changes or an explicitly tested aggregate-tree merge.
Sealed versus Git-selected states and stale same-path adoption remain F03/F04/F08
obligations. These are further design refinements, not additional executed cases.

Implemented mechanism follow-up (EVID-045, P01/P04): an unsigned recipe claim is
not execution or selected-source authority; independent BoundSubject construction
and explicit Unproven history preserve those distinctions. Adversarial parsing
also showed an internally tagged unit enum can ignore extra fields despite
serde deny_unknown_fields. An empty struct variant and explicit unknown-field
regression fix this new selector. Other tagged-unit consumer profiles require a
separate audit; this finding does not claim they have all been qualified.
