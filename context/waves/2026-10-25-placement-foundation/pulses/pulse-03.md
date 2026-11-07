# Pulse 03: Pure Rust planning

Goal: Positive/negative synthetic plan fixtures and constraint/coverage behavior.

Status: in progress: 13 pure handoff mechanism tests pass locally. Parent
TIG-R02/R03/R04/R05/R09, specs/HANDOFF-PROTOTYPE.md. L0: structural checks;
L1: fmt, locked tests and all-target Clippy. L2 durable/provider safety is open.
No IO or actual eviction is implemented; full designed cases remain unexecuted.

WP-001 checkpoint: 39 total pure tests (13 handoff, 10 canonical, 16 tree),
including bounded sparse edits and fault-injected metadata ports. Pure package
scope verified; durable/provider safety and the broader pulse remain open.
See specs/IDENTITY-AND-TREES.md and EVID-007/008. WP-002 byte views are next.

WP-002 checkpoint: 22 additional public-API tests (61 total) verify isolated
binding intents, exact seals, three-way conflicts, shared budgets and retry.
See specs/BYTE-WORKTREE.md and EVID-009. Durable provider/Git coupling is open;
WP-000 discovery and WP-003/004 publication/fencing are the next prerequisites.

Documentation checkpoint: docs/THEORY.md and four docs/guides ASCII walkthroughs
connect branch scratch/main/history, exact-object lookup/LFS, immutable bundles
and transport, and safe demotion/recovery. README provides the entry path and
seven-concept glossary. This changes explanation, not prototype capability.

Validation: pwsh -NoProfile -File tools/validate.ps1; git diff --check.
When implementation exists, add fmt/locked Rust tests/Clippy and exact focused
failure fixtures before closing the pulse. Update docs/vtrace/REVIEW.md with
producer/independent method and exact evidence scope.
