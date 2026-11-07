# Races, claims and shared work

Producer-applied design walkthrough, not an executed test. Each row starts with
its stated condition, applies the next step and checks the required consequence.
All mitigations are planned; references link to the common doctrine and mitigation
register. See [learnings](LEARNINGS.md).

| ID | Starting condition | Next step | Required consequence | Pitfall / mitigation |
| --- | --- | --- | --- | --- |
| RW01 | A reader arrives after a plan marks an object surplus | Acquire a read while removal tries to start | Serialize the conflicting scope so a granted read remains protected | TIG-F-04 / M04 |
| RW02 | An object path is removed and reused between validation and deletion | Apply the earlier removal intent | Fence exact object identity and provider generation, not the reused path | TIG-F-04 / M04 |
| RW03 | Two publishers reserve the same remaining free space | Attempt two bounded ingests concurrently | Admission accounts for both reservations; do not oversubscribe capacity | TIG-F-02 / M02 |
| RW04 | A branch is deleted while its renderer is still using old objects | Retire branch claims and continue the job | The independent admitted job claim continues to protect its inputs | TIG-F-01 / M01 |
| RW05 | Two projects share bytes but require incompatible placement/privacy rules | Reconcile both claim sets | Share only permitted copies and report unsatisfied constraints; no last-writer rule | TIG-F-09 / M09 |
| RW06 | A user loses removal authority after producing a dry run | Attempt apply under the old token | Recheck current authority; refuse the stale destructive action | TIG-F-05 / M05 |
| RW07 | A claim generation counter would overflow | Attempt the next transition | Reject explicitly rather than wrap and admit stale work | TIG-F-04 / M04 |
| RW08 | A remote provider cannot enforce the required stale-writer fence | Plan an automatic removal | Leave the operation unsupported/blocked until an adequate backend protocol exists | TIG-F-09 / M09 |
| RW09 | An idempotency key is reused for a different object payload | Retry the altered request | Reject conflicting operation identity; never replay a success for different bytes | TIG-F-04 / M04 |
| RW10 | A shelved draft expires while another branch adopts one of its objects | Release the draft claim | Preserve adoption/active claims; expire only the owning draft obligation | TIG-F-05 / M05 |
