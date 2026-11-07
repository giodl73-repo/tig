# Tig foundation scenarios

This ledger remains design-only; none of its rows is closed by component tests
alone. Scoped runtime tests are recorded separately in
[the evidence register](../vtrace/EVIDENCE.md). Full-system acceptance remains open.

| ID | Trigger | Expected outcome |
| --- | --- | --- |
| T01 | Two bundles share one object | One verified copy per required place; no per-bundle duplication |
| T02 | Five objects are replaced in current | Acquire new, retain required history, assess surplus only after fresh checks |
| T03 | Old object remains used by another current bundle | Protect its working-place obligation |
| T04 | Same path differs across snapshots | Resolve by exact snapshot and path; never latest-path substitution |
| T05 | Logical path renamed with identical bytes | Manifest update can require zero payload transfer |
| T06 | Admitted lifecycle label changes | Pinned policy changes desired placement; descriptive labels do nothing |
| T07 | Required archive already verified present | No redundant transfer; revalidate admissible custody before eviction |
| T08 | Only alleged archive is offline | Required custody remains unknown/blocked unless explicit admissible evidence policy applies |
| T09 | Destination has right name/size but wrong bytes | Reject complete observation; preserve good source |
| T10 | Two aliases resolve to the same disk | Do not count as independent retained copies |
| T11 | Merge event missed or duplicated | Catch-up exact refs; idempotent convergent reconciliation |
| T12 | New protection claim arrives after planning | Fence/replan conflicting removal; preserve valid completed copies |
| T13 | Two sessions add content while eviction runs | Scoped coordination; no global pause; deduplicate safe promotion |
| T14 | Crash during cross-volume copy | Partial destination not complete; resume/retry with source protected |
| T15 | Scratch/capacity exhausted | Bounded refusal/recovery; no full duplicate staging |
| T16 | Provider cannot fence or verify retention | Expose unsupported action rather than a fabricated pass |
| T17 | One entry changes in a huge index | Reuse unaffected chunks; measure metadata and physical write amplification |
| T18 | Dropped object shares a pack with a protected one | May reclaim zero; explicit budgeted compaction is separate |
| T19 | Destination constraints partially specified | Plan stays unresolved until defaults/assignment are explicit |
| T20 | Two policies impose incompatible locations/budgets | Declared precedence or unsatisfied obligations; no last-writer choice |
| T21 | Malicious tree or torrent path escapes root | Reject before IO; normalize with explicit cross-platform rules |
| T22 | New checkpoint reuses most shard torrents | Acquire missing shards only; prove client local reuse/no duplicate staging |
| T23 | Seeder leaves after a successful download | Availability observation changes; retained custody independently assessed |
| T24 | Draft claim expires while a job uses its objects | Active admitted job remains protected; expiry does not erase other claims |
| T25 | Synthetic document bundles use remote and local stores | No episode/language/golden assumptions; same generic placement rules |

The canonical ledger is tests/scenarios/catalog.json. Execution evidence must
identify source, provider/runtime, inputs, outcome and limitations before status
changes. In particular T17/T22 must measure physical bytes, not just logical sets.
