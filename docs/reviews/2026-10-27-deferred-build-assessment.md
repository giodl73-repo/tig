# Deferred-build assessment producer review

Entry7adb6e529e0374e0f41d561a04fe0a838296ef0d; controls, boundaries and profiles
are in specs/DEFERRED-BUILD-ASSESSMENT.md. Scope: pure optional Rust API, tests
and bounded docs. No new dependency, provider, executable, daemon or live cache IO.

Pre-code six-role review is recorded in that specification. Post-code findings:

- Identity/namespace: one exact desired checkpoint and explicit asset/selector
  keys; duplicate/invalid keys refuse. Foreign current subjects block.
- Git/adoption: source merges remain independent from rendering; all dependency
  resolution uses existing Plans. No new authored dependency list or Git writes.
- Concurrency/recovery: immutable borrowed capabilities only. Reassess when source
  changes; the report cannot enroll a job or survive as executable authority.
- Placement policy: unknown desired plan remains blocked; unproven existing
  masters require investigation rather than automatic regeneration.
- Storage/custody: missing/stale states are advice only, never eviction evidence.
- Provider/privacy: no provider calls; Debug reports counts, explicit entries
  remain private. No raw errors, prompts or claim contents in diagnostics.

Initial focused run: six pass, one fixture fails because its scene deliberately
froze old leaf pins. Corrected the fixture to logical-ID references before its
original plan, then advanced the synthetic checkpoint after two primary edits.
All seven focused cases pass; frozen semantics and the comparison code were not
changed to force invalidation. This is fixture correction, not a producer defect.
The real-Git/archive replay fixture additionally assesses stale/blocked outcomes
with Git, records and definitions physically unavailable.

Remaining risks: caller must admit current source subjects and replay original
claims; this API cannot establish native consumption, full schema/variant meaning,
complete output discovery, producer execution, approval or custody. Typed reports
are private runtime data, not growing Git inventories. Review is producer-applied,
not independent or human acceptance. Full local/hosted qualification follows.

Post-code evidence review tightened Observation to Absent/Present/Unavailable for
both current output and claim. An IO/parse/replay failure must remain Unavailable,
not None silently converted to an Unbuilt candidate or Unproven history. A focused
negative fixture covers each unavailable position. Final source qualification
must include this change; the earlier seven-case run alone does not qualify it.

Final local verification: all98 tig-assets entries pass (including all eight new
assessment cases); all42 native_source entries pass, including actual detached
Git/archive assessment replay. Workspace all-target/all-feature warnings-denied
Clippy, standalone no-feature tig-git check and format pass. These final targeted
runs qualify the Observation refinement after the earlier491-entry workspace
run; they are not described as a second full workspace run. Exact committed-head
Windows/Linux hosted validation remains mandatory before merge. No full intended-
use validation or private producer/custody acceptance is closed by these tests.
