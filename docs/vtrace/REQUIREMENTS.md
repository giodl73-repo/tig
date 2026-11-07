# Controlled requirements

All full requirements are OPEN. REQ aliases serve VTRACE tooling; stable TIG-R
IDs remain the same obligations, not a second specification. P0 protects bytes,
authority or recovery; P1 enables efficient use. Owners are local role slugs.

| Requirement ID | Stable ID | Testable obligation | Need / CONOPS | Priority | Owner | Verification method |
| --- | --- | --- | --- | --- | --- | --- |
| REQ-001 | TIG-R01 | Resolve a logical path within one pinned snapshot to exact digest/size; reject identity, size and namespace ambiguity before use | NEED-001; CON-001/002 | P0 | identity-namespace | Hash/canonical vectors, path collision/traversal negatives, exact tree lookup and cross-platform materialization fixtures |
| REQ-002 | TIG-R02 | Union admitted owned obligations, explicitly resolve placement constraints and retain unknown coverage rather than infer zero demand | NEED-002/004; CON-002/004 | P0 | placement-policy | Shared consumer, conflict/expiry, incomplete closure and independent owner transition fixtures |
| REQ-003 | TIG-R03 | Preserve recoverability of acknowledged managed bytes through publication/replacement/recovery; release protected replicas only after fresh retention/independent-custody checks and current authority | NEED-002; CON-003/004 | P0 | storage-custody | Fault-injected byte recoverability, corrupt/offline/same-domain copies and explicit final-retirement negatives |
| REQ-004 | TIG-R04 | Reject stale/conflicting operations and protect valid readers/jobs using scoped coordination; recover retries without a global cache pause | NEED-004; CON-003/005 | P0 | concurrency-recovery | Reader/removal races, lost replies, clock/lease ambiguity, replay and unrelated-writer progress tests |
| REQ-005 | TIG-R05 | Bound metadata/IO/scratch under declared workloads and report logical versus physical effects, including duplicate copies and pack pinning | NEED-001/003; CON-004 | P0 | storage-custody | Peak-space, full-disk, large sparse edit, pack compaction and allocated-byte measurements; acceptance numbers pinned per workload |
| REQ-006 | TIG-R06 | Admit providers/paths/access explicitly and expose unsupported capabilities/denials; basic use and public artifacts contain no private consumer dependency | NEED-005; CON-001/006 | P0 | provider-privacy | Fake capability/access failures, hostile paths/config, redaction and non-media standalone consumer |
| REQ-007 | TIG-R07 | Resolve requests through bounded indexes with generation/freshness/coverage-aware observations; normal lookups do not crawl all stores | NEED-003; CON-001/005 | P1 | concurrency-recovery | Warm/cold/miss invalidation, watcher overflow, catalog restore and measured lookup/read budgets |
| REQ-008 | TIG-R08 | Interoperate with supported standard LFS pointers and pinned client transfer semantics without typed assets; account for local-store/worktree copies and reject unsupported transforms | NEED-001/005; CON-001/005 | P1 | git-adoption | Real pinned LFS client fixtures, correct OID/size/completion, skip-smudge, configured stores and coordinated-prune negatives |
| REQ-009 | TIG-R09 | Derive source-owned claims from exact Git/worktree/index checkpoints while preserving durable uncommitted admission and recovering interrupted ref/selection handoffs | NEED-001/004; CON-002/003/005 | P0 | git-adoption | Dirty/conflicted index, detached worktree, force-push/rename, seal/adoption crash and missed-event catch-up fixtures |
| REQ-010 | TIG-R10 | Offer optional generic typed records, exact dependency/variant resolution and consumed-value build/lifecycle rules without duplicating byte identity/storage or requiring that layer for Git/LFS use | NEED-005; CON-006 | P1 | identity-namespace | Art parity vectors followed by synthetic typed/text/build/review/delivery integrations; standalone low-level consumer regressions |

## Acceptance tailoring

Each package pins its supported provider/client/profile and quantitative resource
limits before implementation closes. Initial planning can report unknown or
unsatisfied assignments; executable operations require complete applicable scope.
No requirement assumes Git hooks are authoritative or all remote stores share a
transaction. Typed authority/approval, delivered state and physical custody stay
separate. Final archival deletion requires its explicit admitted policy.

## Stage review

Producer-applied six-role lenses: pass_with_risk. R-01: the earlier trace used broad
phrases without priorities/acceptance methods. Resolved with owned observable
obligations and explicit workload tailoring. Numeric latency targets and released
wire formats remain named discovery outputs in the baseline/work packages; no
unknown is represented as a passed acceptance criterion.

## Native instance and merge clarification

REQ-010 includes small native .tig instance records governed by separately stored,
exactly pinned type definitions. Definitions own shape, grouping, stream roles,
ordering/cardinality and admitted semantic checks; instances own actual values
and exact byte bindings. Definition changes cannot silently reinterpret history.

For a declared ready selection, validate the actual resolved merge candidate
before acceptance, not merely after it reaches main. A user may intentionally
choose three streams from one branch and two from another. Accept that mixture
when declared constraints hold; unusual provenance alone is not an error. Check
output/input currency only for the relationships the pinned definition declares.
Missing facts or violated constraints require explicit resolution or an admitted
draft disposition under target policy; never silently label the result current.

R-02 follow-up review: separated native instance data from definitions and made
merge-time validation a controlled obligation without prohibiting explicit mixed
bindings. Exact wire format and merge tooling remain target implementation.
