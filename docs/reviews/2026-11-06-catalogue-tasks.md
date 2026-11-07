# Catalogue task review

Input 6a96618e4bfb5280e373c7a658de46aeac6e0051. Controls: WP-010/015,
REQ-006/007/010, SPEC-010/014, IF-010/011/013, PKG-004/006/008.
UI + INTEGRATION L0/L1/L2. Scope: generic navigation and task testing;
no enrollment changes, producer changes, new cache, payload writes or removal.
This is producer-run task testing, not an independent user study.
Private project identities, screenshots and run results remain operator-side.

## Task expectations

| Task | Expected behavior |
| --- | --- |
| 01 Current work count | Project-admitted status counts match cards. |
| 02 First season | Facet narrows count. |
| 03 Second season | Facet replaces prior selection. |
| 04 Third season | Facet replaces prior selection. |
| 05 Small collection | All five fixture works visible together. |
| 06 Versions | Versions belong to one work. |
| 07 Deliveries | Registration differs from remote verification. |
| 08 Children | Parent opens indexed scenes. |
| 09 Ancestry | Child can return to parent. |
| 10 Search | Work, scene and primary records are discoverable. |
| 11 Search type | Type filter changes results. |
| 12 Cross-project | Results preserve project identity. |
| 13 Dashboard | All enrolled projects appear. |
| 14 Objects | Dashboard reaches project objects. |
| 15 Capacity | Objects reach cross-project drives. |
| 16 Scope | Capacity retains project filter. |
| 17 Compare | Visible action reaches storage by work. |
| 18 Storage type | Admitted category changes object rows. |
| 19 Selection | Selection survives filter changes. |
| 20 Review | Older-copy review remains held. |
| 21 Execution | Review offers no deletion executor. |
| 22 Pilot | No adapter differs from complete coverage. |
| 23 Coverage | Loaded paths differ from complete dependencies. |
| 24 Return | Coverage/object round trip retains search. |
| 25 Errors | No browser exceptions. |
| 26 Entry | Compare works opens project storage. |
| 27 Family | Small collection shows all work totals together. |
| 28 Meaning | Incomplete coverage and overlap remain explicit. |
| 29 Drill-down | Ranked work narrows observed objects. |
| 30 Broaden | All families clears prior work scope. |
| 31 Pivots | Version grouping and cache colors are independent. |
| 32 Return | Project storage reaches named object management. |
| 33 Drives | Global capacity returns to project work comparison. |

## Findings and dispositions

Initial run: storage-by-work was unreachable from visible controls. Add
Compare works beside object management in capacity/project views; provide
a return to project objects. Reuse existing scoped reader and pivots.
Three initial failures used abbreviated rather than displayed facet labels;
one expected a domain subtype where enrollment exposes a broader category.
Correct test assumptions; do not change project semantics to satisfy a harness.

Open usability gap: admitted storage categories may group several primary
subtypes. Search can find individual subtypes when catalog rules expose them;
the storage filter cannot promise the same taxonomy. A future external mapping
must preserve shared-object classification and unknowns.

Work sizes compare loaded declarations/observations, not equal production
coverage. Shared references overlap. History classification is not deprecation;
no known other references is not zero demand. Physical reclaim and independently
retained custody remain unknown. These tasks cannot qualify eviction execution.

Identity/privacy: generic controls only; no private fixture copied into source.
Placement/custody: comparison changes no retention or removal authority.
Concurrency: existing scoped observation and result fences remain in force;
no global lock. Adoption: keep external enrollment and producer bindings intact.
A deeper live task found grouping leaked owners from other families into a
filtered work ranking. Match family and work against the same owner pair and
restrict group keys to matching owners. Synthetic shared-package tests cover
filtered totals, impossible cross-family pairs and broadening back to all owners.
Named observations also attributed nested bytes to package record IDs rather
than their enclosing admitted works. Carry the native record-to-work membership
into nested observations; preserve shared owners and unknown lifecycle status.
An adapter integration test requires a nested leaf in its enclosing work's
filtered storage result without granting removal authority.
Final local results: 25 catalogue tasks plus eight work/capacity tasks passed
after fixes. Private scripts use UI clicks and filter events; no hidden function
substitutes for the Compare works path. Ten catalogue integration tests and six
storage unit tests passed, as did warnings-denied workspace Clippy, standalone
check, Rust fmt and JS syntax. Hosted CI is a separate pending gate.

Version pivots use recorded version tags; untagged nested leaves are not
assigned a version by inference. Compare declared version manifests separately.
