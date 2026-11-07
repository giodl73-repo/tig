# Catalogue collection selection

Input e51e62213e21f2239129e9a3d73dffd43339b1d2. WP-015; REQ-001/006/007/010;
SPEC-014; IF-010/011/013; PKG-004/006/008; PROFILE-006 UI.
Entry: existing pinned-snapshot catalogue and synthetic work fixtures.
Allowed scope: summary-card/collection toggles, bounded native union listing,
snapshot explanation, synthetic tests and this contract/review. No storage IO,
new project rules, source checkout, fetch, provider or lifecycle behavior.
Exit: ordinary click selects/toggles; Ctrl/Meta-click unions; all-off clears;
each total keeps its configured state; paging/search and snapshot checks stay
bounded; unknown/duplicate selectors refuse. Source snapshots remain explicit.
L0: foundation/format/hash/privacy; L1: native union and frontend checks, Clippy;
L2: synthetic browser flows including paging, reset, keyboard and stale results.
Producer checklists: identity, provider/privacy, Git/adoption and concurrency.
No independent review, custody, eviction or production acceptance is implied.

Extension before implementation: WP-010/015, REQ-001/006/010, SPEC-010/014,
IF-010/011/013, PKG-004/006/008. Add a metadata-only shared-selected-object
CLI with exact pinned commits, category filtering, bounded private report,
same-hash/different-length refusal and explicit unindexed-content uncertainty.
Load only the two requested projects. L1/L2 synthetic fixtures cover duplicate
references, disjoint selections, missing selections, bad selectors and CLI
create-new output. No payload hashing, provider observation or filesystem scan.
The source budget increases from 3.5 to 3.6 MiB to accommodate these generic
operations and tests; the 128 KiB per-file and runtime/media exclusions remain.

Continuation: same controls/input SHA. Expose the same shared-object calculation
in a paged two-project UI, plus selected/named-stream object-reference lookup in
the CLI. Both report exact commits and excluded scopes. Tests must distinguish
byte bindings from transitive semantic consumers, reject malformed hashes and
oversized pages, and fence stale UI responses. Browser interaction requires
fresh user permission; automated controller/API fixtures remain in scope.

Producer review findings: identity keeps SHA-256 plus length and refuses
contradictory lengths; privacy exports admitted IDs/definitions/categories, not
raw authored text or filesystem paths; Git/adoption loads explicit pins and
keeps reports outside Git; concurrency retains UI generation/snapshot fences,
create-new reports and no mutation/removal authority. No independent review.
Remaining scope: nested/LFS adapters and transitive consumers are unindexed;
metadata agreement does not verify payload presence or decoded media equality.

Verification: full locked workspace tests passed before the continuation;
continuation adds focused Rust catalogue/CLI tests and five automated frontend
controller tests for toggles, combined states, shared links, stale responses,
coverage and zero results. Strict all-target/all-feature Clippy and standalone
no-default-features checks are required on this final patch. Browser validation
remains pending permission; no claim of final visual acceptance is made here.

Permission subsequently granted: producer-run browser tasks at 5bb0c39 passed
single-total filtering, project-pair selection, exact commit labels, category
filtering, shared-byte totals and opening linked record inspectors. Testing found
summary redraw lost keyboard focus. This follow-up restores focus to the newly
rendered total and adds a controller assertion; keyboard validation follows
against the rebuilt candidate. No private project data is retained in source.
