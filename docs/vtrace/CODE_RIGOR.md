# Code rigor controls

Applies to new implementation packages. Development conventions do not create
product commands or certification claims. Any exception names scope, rationale,
owner and compensating evidence before package closeout.

| ID | Constraint | Verification |
| --- | --- | --- |
| CR-001 | Pure core remains deterministic and IO-free; runtime dependencies follow package direction | Dependency/source inspection plus standalone pure tests |
| CR-002 | Parse/traverse/copy/queue operations have explicit byte/node/depth/time/concurrency limits; size arithmetic is checked | Boundary/overflow/resource-exhaustion tests; no unbounded recursive traversal |
| CR-003 | Public runtime paths return typed errors for expected failures; no panic/unwrap/expect as ordinary recovery | Negative/error-path tests and all-target Clippy with warnings denied |
| CR-004 | Critical state transitions are small reviewable units; target at most 80 logical lines per function, otherwise record focused justification | Review and source-size inspection; test helpers may exceed with rationale, not hidden runtime logic |
| CR-005 | Unsafe code is absent by default; any provider/FFI exception has an explicit safety contract and targeted review | Unsafe/dependency inventory, platform fixtures and recorded exception |
| CR-006 | Publication, replay, fencing and removal have fault/race counterexamples before live execution | Fake-provider crash matrices and isolated temporary-store integration tests |
| CR-007 | Logs/errors/exports minimize private paths/payloads and exclude credentials; tracing is bounded | Redaction/permission negatives and source inspection |
| CR-008 | Format, locked tests and Clippy pass; lockfile/source/provider pins are retained | Commands in LANGUAGE_PROFILES.md, CI matrix and evidence ledger |

Do not add ad hoc production Python or per-artifact scripts. Existing vendored
Python checkers and PowerShell repository checks are development-only tools.
Do not build a project-management framework to record these packages; bounded
Markdown/JSON control records suffice. A coherent large change must deliver a
named product capability or shared infrastructure with a second adopter.

## Stage review

Producer-applied identity, recovery, privacy and Git lenses: pass_with_risk.
CR-01: unchecked IO claims could survive clean static checks. Resolved by making
fault/race evidence mandatory in addition to lint; these constraints do not
retroactively prove current prototype durability or a real-power-loss guarantee.
