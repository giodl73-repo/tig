# Tig mission and needs

Baseline input: Tig 0a9a522. Scope: one product with a low-level byte-worktree/
placement layer and optional generic typed-asset semantics. Design readiness is
distinct from runtime safety, release readiness and consumer acceptance.

## Users and operating context

Git/LFS users, media/document creators, CI operators and storage administrators
need small authored histories while large immutable content remains available
near active work and recoverable across retained versions. One workstation may
have many concurrent worktrees, intermittent archives and remote stores. No
background service or full hydration is implied by opening a repository.

| Need | Intended outcome | Validation basis |
| --- | --- | --- |
| NEED-001 | Small reviewable Git records and complete logical file views | CON-001/CON-002; independent leaf edits and same-name replacement |
| NEED-002 | No unintended loss of acknowledged managed bytes | CON-003; faults, custody checks and exact recoverability |
| NEED-003 | Nearby working sets without uncontrolled duplication or disk exhaustion | CON-004; peak-space and actual reclaim measurements |
| NEED-004 | Concurrent branches/jobs/CI with recoverable lifecycle handoffs | CON-002/CON-005; interrupted and racing operations |
| NEED-005 | Git-only usability plus optional asset semantics without media-specific core | CON-001/CON-006; generic and typed consumers |

## Success and constraints

Users can change one logical file, adopt its exact new bytes in Git, run CI and
merge without copying all unchanged assets. Placement and independent retention
converge under admitted policy, or report specific pending/blocked obligations.
Routine eviction never reduces required recoverability or active-use protection.

Measure metadata/worktree size, duplicate allocated bytes, peak scratch, lookup
reads/latency, recovery cost and physical reclaim on declared workloads. Numerical
performance targets require fixtures before a capability is accepted. Local
admission cannot guarantee space against unrelated external writers.

Non-goals: replace Git's history, build media renderers, create a new torrent
protocol, promise survival of all hardware/key loss, silently install hooks or
services, or publish private content. Typed features remain optional; protocols
and public wire formats are not yet frozen. Preserve existing consumer safeguards.

## Stage review

Producer-applied identity, policy, custody, recovery, privacy and Git lenses:
pass_with_risk for mission scope. Finding M-01: preservation could imply survival
of all physical failures. Resolved by explicit failure-domain assumptions and
provider/retention validation. Detailed acceptance belongs to VALIDATION.md and
work packages; no current loss-free runtime claim is made.
