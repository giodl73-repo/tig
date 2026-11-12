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

Scale-fixture partition entry: input090d629 plus the full-rotations candidate
qualified by46476 and all-feature strict44924 (all phase exits0). Same engineering
controls; INTEGRATION L0/L1/L2. The new actual49153 workload takes1087.57s alone
locally. Assign its exact module prefix to a fourth disjoint store-library shard
and a seventh native suite on each OS, preserving every old suite and60-minute
deadline. All names still come from the compiled executable; empty/duplicate/
unparsed inventories fail. Keep both foundation status names and require every
native matrix job to succeed. Allowed scope: workflow/runner/tests/evidence only.
Local partition and actual inventory checks precede push; hosted results remain
pending for the new exact source. No Rust implementation or test result changes.

Local verification: six runner unit checks pass, including a missing-rotations-shard
inventory refusal and native failure propagation. Actual compiled inventory535
partitions exactly store419/journal64/continuation51/rotations1. Actionlint1.7.12
passes. All prior suites/explicit cases/baseline SHAs/status names remain unchanged;
the new full fixture has its own native suite. Hosted execution on the new exact
head remains pending, and20a3865 hosted proof is not transferred to this source.

Library scheduling entry: input858595b plus legacy-preparation candidate qualified
by13356/53584. Archived090 Windows continuation log shows all small fixtures
finishing early, three maximum fixtures overlapping, two completing only after
about54minutes and the third still running when the newer push cancelled the job.
No FAILED line is logged before cancellation. New858 standalone rotations passed
Windows in18m08s. This suggests independent-fixture resource contention, not proof
of an assertion failure or a complete pass. Preserve the raw archived log externally.

Scope: CI runner/argument tests/evidence only, INTEGRATION L0/L1/L2, same controls.
Run top-level store-library batches with explicit --test-threads=1; preserve exact
names, all groups, ignored semantics, every explicit peer/process concurrency test
and60-minute job deadlines. Bound the added CLI argument in batch length checks.
Local argument/failure/inventory tests precede push; exact hosted timing/result is
still required. Workspace/non-store and explicit operator/compatibility commands
retain their existing scheduling. No Rust implementation, cap or native timeout
change. This is a scheduling qualification, not independent or human approval.

Exact858595b hosted43170309914 all17 jobs SUCCESS (native14/secrets/foundation2).
Windows continuation59m53s leaves almost no slack; standalone rotations18m08s.
The later scheduling/helper source is not covered. New local scheduling checks:
six Python tests and Actionlint pass; actual542 names still partition exactly
store419/journal64/continuation58/rotations1, with explicit one-thread batches.
New exact hosted execution/timing remains required; no deadline was increased.
