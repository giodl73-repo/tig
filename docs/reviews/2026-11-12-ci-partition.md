# CI partition entry

Input: bd0fabd8fc75f4e29f99c7a99f3028c0b46d81c4 with the separately pinned,
uncommitted continued-engine candidate. Hosted run43151515936 on7caf201 passed
Linux/secrets; Windows job130961642989 was cancelled after60m15s in the Rust
step. This is incomplete Windows verification, not a test pass.

Controls: WP-003/004/006/007/008/013/015; REQ-001/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014; IF-002/003/004/005/006/008/012/013;
PKG-001/002/003/008/009. INTEGRATION profile; L0 inventory and source validation,
L1 partition/argument/failure tests, L2 hosted native Windows/Linux results.

Allowed scope: workflow and portable Python test inventory/runner plus evidence.
No Rust/TOML/lock edits while the engine qualifier owns those hashes. Preserve
60-minute test-job deadlines and every current default/explicit ignored check.
Partition store library tests by exact names from the compiled test executable:
continuation prefix, other ledger prefix, all remaining store tests. Reject empty,
duplicate or malformed inventories. Execute bounded batches of exact names;
default ignored semantics remain unchanged. Workspace integration/doc tests,
other crates, operator qualifications and historical-reader compatibility remain
separate explicit jobs on both operating systems. Keep foundation status names
through an always-evaluated aggregate that fails unless all test jobs succeeded.

Entry: exact source and incomplete hosted run identified, multiple exact libtest
filters locally confirmed with two named passing tests. Exit: partition unit and
real-inventory checks, workflow coverage audit, L0/diff check; hosted native
results are independently pending until the next exact-source run finishes.
Implementation, verification, scoped validation, trace and producer review must
be recorded separately; producer checklists provide no independent approval.

Implementation: six native suites per OS replace the monolithic job. Store binary
unit harnesses, integration tests and doctests are explicit workspace steps; all
seven ignored operator qualifications and three compatibility qualifications are
retained verbatim. No test filter is removed and no timeout is raised.

Verification: six Python checks pass, covering disjoint complete partitioning,
invalid/duplicate/empty inventory, bounded exact batches, failed compilation,
missing artifacts and propagation of native test failure. Actual compiled Windows
inventory has523 names: store419, journal64, continuation40; batches7/1/1.
Two actual exact-filter names ran together and passed. Actionlint1.7.12 passes;
its release archive SHA256 was verified against the GitHub release asset digest.

Scoped validation: local inventory/runner/workflow structure only. The new hosted
Windows/Linux matrix has not run. Trace: every prior cargo workspace test class,
explicit ignored case, baseline SHA, feature profile, Clippy and standalone check
has a destination above. Foundation aggregates require all native suites to pass.

Producer review (six .roles lenses, owner Tig maintainers): exact test identities
and exhaustive partition checked; policy/custody claims remain unchanged; failed
or skipped suites cannot authorize success; no provider enrollment/private asset
copy; existing required foundation names retained. PASS_WITH_RISK for this scope;
hosted execution remains OPEN. This is no independent or human approval.

Hosted L2: exact20a38656a58e256b2826bc4d92737fb16d3e4427 run43159604450 is
COMPLETED/SUCCESS. All twelve native suites, secrets and both preserved foundation
aggregates succeed. Windows journal22m47s, continuation21m47s and workspace20m25s
finish inside the unchanged60-minute test-job deadline. This closes the hosted
partition execution obligation for20a3865 only. Dirty continued-seal integration
is a later source and is not covered by this run; full product gates remain open.
