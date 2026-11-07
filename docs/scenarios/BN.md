# Bundles, transport and scoped knowledge

Producer-applied design walkthrough, not an executed test. Each row starts with
its stated condition, applies the next step and checks the required consequence.
All mitigations are planned; references link to the common doctrine and mitigation
register. See [learnings](LEARNINGS.md).

| ID | Starting condition | Next step | Required consequence | Pitfall / mitigation |
| --- | --- | --- | --- | --- |
| BN01 | A bundle manifest is small but expands into excessive descendants | Resolve it within declared read budgets | Return explicit incomplete scope when limits are exhausted; no unbounded traversal | TIG-F-10 / M10 |
| BN02 | Imported manifests contain a cycle or inconsistent child identity | Resolve exact bundle paths | Reject malformed graph/identity before materialization; do not trust root naming alone | TIG-F-10 / M10 |
| BN03 | A moving bundle reference advances while a reader opens a file | Resolve and acquire the requested snapshot | Pin one exact snapshot; never mix old metadata with new byte selection | TIG-F-06 / M06 |
| BN04 | A partial place profile leaves two eligible destinations with no tie-break policy | Build an executable placement plan | Keep the assignment unresolved or use an explicitly pinned deterministic rule | TIG-F-12 / M12 |
| BN05 | One digest is advertised with conflicting sizes in two inventories | Combine branch bundle membership | Reject the inconsistency instead of choosing whichever report arrived last | TIG-F-06 / M06 |
| BN06 | Torrent metadata would expose confidential file names to unadmitted peers | Prepare a distribution descriptor | Use an admitted opaque/encrypted mapping and privacy review; private discovery flags alone are insufficient | TIG-F-09 / M09 |
| BN07 | All seeds disappear halfway through a download | Retry with verified partial pieces | Keep partial state, report unavailable sources, and do not count an independent complete replica | TIG-F-07 / M07 |
| BN08 | A file root is known but supporting piece data are unavailable | Try to materialize the file | Identity knowledge does not imply byte availability; acquisition stays pending | TIG-F-07 / M07 |
| BN09 | A minor edit causes a compressed output to share almost no byte chunks | Estimate version storage using content similarity | Use measured byte-level reuse rather than visual similarity or an assumed delta ratio | TIG-F-11 / M11 |
| BN10 | A backup client without Art submits overlapping immutable bundles | Reconcile with existing Git-project claims | Apply the same object/place rules and independent claim ownership without media assumptions | TIG-F-12 / M12 |
