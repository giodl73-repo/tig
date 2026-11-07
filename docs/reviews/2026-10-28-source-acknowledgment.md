# Exact recorded-operation acknowledgment

Entry 81d9224964491dc498653a0663a687d88b747cce. WP-008/009/015;
REQ-002/004/007/009, SPEC-005/006/008, IF-004/006/008, PKG-003.
Allowed scope: existing journal read-only acknowledgment of an exact operation
ID and original command. Reuse verified replay, canonical request comparison and
historical receipt construction. An unknown ID must refuse, never execute it.
This is recovery of an acknowledgment, not rerunning a producer or completing
an unrecorded transition. Keep all roots and current journal state unchanged.

Entry: bounded read-only observation exists. Exit: exact historical receipt
after later transitions, changed/missing requests and foreign/rollback witnesses
refuse, CLI remains private and cannot overwrite reports. No writable authority
handle escapes. L0 structure/fmt/locked all-feature Clippy; L1 new API/CLI cases;
L2 existing restart/source integration suite and hosted platform qualification.

Producer role lenses: concurrency/recovery, provider/privacy, Git/adoption.
Review risks: typo IDs becoming new commands, confusing old receipt revision
with current source state, and acknowledgment being mistaken for render success
or permission to retry an unknown job. Independent acceptance remains open.

Initial focused qualification: two API cases pass (0.67 seconds), covering
historical acknowledgment after adoption, unchanged database/count/current roots,
unknown/changed requests, missing/foreign authority and rollback witness refusal.
The first foreign-authority assertion expected Conflict; existing authority
validation correctly returns InvalidReference. The test was corrected without
changing the contract. Four CLI cases pass (0.95 seconds), including the new
acknowledgment case with PATH empty, private output, unchanged authority and
unknown/changed/overwrite/inside-authority refusals. Locked all-target/all-feature
workspace Clippy passes. Full regression and exact-head hosted checks pending.

Producer disposition: pass_with_risk for exact recorded acknowledgments only.
Concurrency/recovery: read-only deferred transactions, no new transition for
missing IDs, original receipt revisions explicitly historical. Provider/privacy:
no writable handle exposed, unchanged report publication bounds/redaction.
Git/adoption: no hooks installed, no runtime binding changes or asset migration.
High/open, integration owner: recover unrecorded job outcomes, source coverage,
compaction, complete custody and the all-family production workflow.
