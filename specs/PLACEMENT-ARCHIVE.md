# Persistent classified demand roots

Input b935170772bc4ce5af7043c6255516b52bc3d32a (PR77 candidate).
WP-008/009/010/012; REQ-001/002/004/006/007/009;
SPEC-005/006/007/008/010/012; IF-002/004/006/010;
PKG-001/003/004/006, RUST/INTEGRATION/PROVIDER profiles.
Scope: immutable classified-demand encoding over existing Tree/NodeStore ports,
bounded readback and temporary SQLite/SourceJournal fixtures. No new journal,
cache engine, transfer/removal, source adoption automation or protection claim.
Entry: classified history candidate; qualify its dependency before landing.
Exit: exact assignments restore after provider finish/reopen without Git or
history replay; malformed/missing/substituted roots refuse. L0 validation/fmt/
diff/review; L1 locked all-feature tests/Clippy and standalone Git; L2 actual
temporary metadata finish/reopen plus independent journal roots/restart.

Use existing immutable Tree nodes. Entry identity references a canonical bounded
header, schema tig.native-placement.experimental-1. Header binds source checkpoint,
record pin, policy generation, claim ID, working/retained place sets and counts.
The header object's raw SHA-256 is Selection.context. The exact tree root is
Selection.root. Other entries are working/SHA256 or retained/SHA256 with exact
Object values; each object occurs once. Working membership implies retained
membership. This avoids a second copy of shared working/retained inventory.

Readback traverses the complete bounded tree, verifies raw header identity/context,
canonical encoding/schema/counts, exact path/object identity, unique membership,
record membership, size consistency and existing planner bounds. A caller supplies
the expected source checkpoint/record/policy pins when comparing adoption intent;
an old root must never silently become a newer policy or current Git view.
Restoration means declared desired assignments, not renewed semantic validation,
availability, independent custody, current global coverage or active protection.

Write returns a candidate Selection only. The provider must finish and independently
read back before any SourceJournal prepare/adopt acknowledgment. Journal pending,
current and predecessor obligations remain explicit; another branch/job keeps its
own root. A caller may not retire an old source/job obligation merely because the
new metadata restores. Protected successor closure acknowledgment and exact owner
handoff remain required next T3 work. No new mutable inventory file goes into Git.
