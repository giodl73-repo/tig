# Pulse 02: Contract and backend evaluation

Goal: Freeze exact object/tree/claim identities and evaluate existing systems.

Status: in progress: Git/LFS and state-ownership design pressure-reviewed.
All 110 cases remain designed, including 60 additional recovery/concurrency/LFS/
space/bundle/lifecycle walkthroughs with doctrine and owned mitigation links.
Backend evaluation and contract freeze remain open; no runtime behavior claimed.

Unified-product decision: incorporate Art's generic value in an optional Tig
typed-asset layer. docs/UNIFIED-DESIGN.md records boundaries, byte worktrees,
pointer/merge granularity and parity-first extraction. TIG-R10 remains OPEN;
additional semantic/parity fixtures are required before any implementation claim.

Byte-worktree design: specs/BYTE-WORKTREE.md defines base/private/sealed/Git-selected
states, leaf pointer ownership, removals and expected-selection adoption. Planned
overlays/root-tree merging need their own executable fixtures; no released format
or mount requirement has been introduced.

Full-baseline closeout: docs/vtrace/README.md and STAGE_EXECUTION.md route the
one-deliverable-at-a-time producer role reviews. Specification/package design is
pass_with_risk; all runtime requirements remain OPEN. Seventeen proposed packages
have explicit parent IDs, allowed boundaries, entry/exit, dependency and L0/L1/L2
obligations. Backend discovery/identity vectors are next; no live-operation gate
or private consumer acceptance was passed.

Validation: pwsh -NoProfile -File tools/validate.ps1; git diff --check.
When implementation exists, add fmt/locked Rust tests/Clippy and exact focused
failure fixtures before closing the pulse. Update docs/vtrace/REVIEW.md with
producer/independent method and exact evidence scope.
