# Recovery and durable protection

Producer-applied design walkthrough, not an executed test. Each row starts with
its stated condition, applies the next step and checks the required consequence.
All mitigations are planned; references link to the common doctrine and mitigation
register. See [learnings](LEARNINGS.md).

| ID | Starting condition | Next step | Required consequence | Pitfall / mitigation |
| --- | --- | --- | --- | --- |
| CR01 | Publication succeeds but its acknowledgement is lost | Retry the same operation after restart | Recognize the prior object publication; do not publish twice or shrink protection | TIG-F-03 / M03 |
| CR02 | Power loss occurs after destination bytes are written but before verified publication | Replay the durable ingest intent | Revalidate destination bytes before exposing completion; keep source protected | TIG-F-03 / M03 |
| CR03 | A protection journal ends with a torn record | Restart and rebuild indexes | Mark the uncertain suffix incomplete; recover authoritative state before removal | TIG-F-03 / M03 |
| CR04 | Only an older protection backup can be restored | Replay known later operations and compare source generations | Do not assume absent later job records expired; block destructive work until scope is recovered | TIG-F-03 / M03 |
| CR05 | Two workers restore the same coordinator identity from a backup | Both attempt conflicting generation advances | Fence stale authority; avoid a pair of independent destructive writers | TIG-F-04 / M04 |
| CR06 | Wall clock moves backwards while a job lease approaches expiry | Observe old and new clock values during renewal | Anomalous time cannot prove that protection ended; follow declared clock policy | TIG-F-05 / M05 |
| CR07 | A reader cannot renew while coordinator connectivity is lost | Attempt planned removal of its object | Require an effective reader fence/recovery contract; timeout alone is not permission | TIG-F-05 / M05 |
| CR08 | Claim handoff crashes between old and new checkpoint records | Recover the interrupted transition | Protect the union until durable transition outcome is known | TIG-F-12 / M12 |
| CR09 | An archive drive is detached and a different drive reuses its mount path | Resolve a pending archive operation | Reject the stale place identity; the replacement is not the verified archive | TIG-F-06 / M06 |
| CR10 | A release intent exists but its provider result was never recorded | Restart and retry the release transaction | Inspect exact object/place state; report absent versus present without inventing a complete retained copy | TIG-F-03 / M03 |
