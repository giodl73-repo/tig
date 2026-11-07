# Checked metadata provider copy

Input `0ceade2a16de1c1ed6cc27e48675e7945d7b61df`. WP-001/008/009;
REQ-001/005/007/009; SPEC-002/005/006/008; IF-002/005/008; PKG-002/003/008.
Scope/exit: additive API/CLI copying one exact metadata snapshot into an empty
provider. Entry: immutable nodes and trusted checkpoints. Experimental.

Read-only source, bounded keyset pages/point reads, SHA-256/length for every node,
no transfer-long transaction/global lock. Source drift refuses completion.
Distinct nonnested target and explicit limits; source configuration/nodes stay.
Failure leaves an unselected prefix; no automatic retry/cleanup. Caller admits
physical capacity. Observation binds checkpoints/limits/counts/bytes/set digest;
fresh target readback verifies all nodes. No journal/cache setup, source adoption,
discovery, selection, removal or custody/latest-source assertion.

CLI: `tig-metadata plan PROFILE NEW_PLAN`, `apply PROFILE PLAN NEW_REPORT`,
`verify REPORT NEW_VERIFICATION`. Bounded strict JSON; new-only reports outside
providers. Schema `tig.metadata-provider-copy-profile.experimental-1`: absolute
source_root, source_minimum, missing absolute target_root, target_limits, budget
(nodes/bytes/millis). Plan pins raw profile, exact source checkpoint/limits and
canonical paths; creates/verifies nothing. Apply rechecks plan; API also demands
exact checkpoint. Verify uses admitted report; source may be offline. Private
reports keep paths; summaries omit paths/errors. CLI reserves no capacity.

PROFILE-003: L0 structure/fmt/trace; L1 copy/corruption/budget/drift/failure tests,
workspace locked tests/strict Clippy; L2 synthetic plan/apply/process verification.
Producer role review is not independent/human approval. Hosted qualification,
physical admission and private adoption remain separate gates.