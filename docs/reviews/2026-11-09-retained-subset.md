# Retained-subset handoff review

Input63f2c96e991553e45a55f2cab1b645a945ede496; implementation2fecfab followed
by corrective002a85042be6cda1ee35e665aad97ba09d9dd0a8. Controls, allowed scope,
entry/exit and RUST/PROVIDER L0/L1/L2 are in
[RETAINED-SUBSET](../../specs/RETAINED-SUBSET.md). Targets: that contract,
source_claims::subset, guarded atomic selection, tig-source subset, their actual
CLI/process fixtures and the pinned old-reader CI qualifier. Method: one producer
applying all six [.roles checklists](../../.roles/ROLE.md). Owner of all findings:
Tig maintainer using the named lens. No delegated/independent reviewer or approval.

## Implementation

Exact sorted binding paths identify one owner's Branch/Draft predecessor subset.
Bounded full closure reads and sparse edits produce a verified survivor root.
Existing metadata is reused and finished/read back before source adoption. A
deterministic same-owner/class derived source protects survivors as history before
conditional owner retirement of the original predecessor. Empty survivors need
only the original exact Retire. Other sources/current/pending roots are untouched.
Plan/status/apply/resume use existing private roots, with mutation disabled by
default. No payload-provider open, payload IO, last-copy authority or live enrollment.

The policy review of2fecfab found transient active-use suppression: exposing the
survivor as current before Withdraw can suppress required advisory history roles
even while the original predecessor still exists. Corrective002a850 atomically
composes Prepare/Adopt/Withdraw under the existing scoped admission and short SQL
transaction. Readers see only enrolled-empty or historical survivor state. No
source event encoding changes; the unreleased subset plan/effect moves to design-2
so earlier fixture plans cannot silently acquire different execution semantics.

## Verification

Focused source-claim qualification passes 48 active entries and two ignored
artifact qualifiers. New subset entries total fourteen: twelve active behavior
tests, one child helper and one explicit old-reader qualifier. Four new admission
cases cover atomic history observation, every internal fault boundary, duplicate
phase IDs/effective subject fencing and actual exits. Two actual CLI cases execute
both Branch and Draft profiles, disabled mutation, canonical plans, private-path
redaction, resume and metadata-offline status. The existing finite-journal edge
also refuses the three-command operation before consuming its remaining slot.

Six additional actual child exits cover enrollment/survivor/original retirement
and inside/before/after the atomic history transaction. A 4,096-binding root removes
one leaf with fewer than 128 newly persisted nodes, preserving 4,095 bindings.
Exact aliases and a foreign active job retain demand; full-root settlement keeps
an independently current selection. Corrupt/missing closure, pending metadata,
unexplained source occupancy, changed original/witness, budget/owner/class/path
refusals and later survivor retirement do not authorize destructive replay.

Both explicitly run pinned pre-pair reader qualifiers pass: the inherited
Prepare/Adopt/conditional Withdraw receipt flow and the new complete subset state
plus final conditional Retire receipt. Ordinary ignored entries are not that proof.
Corrected locked default and all-feature workspace suites pass in separate
external targets, including both 500,000-binding metadata runs. Final strict
all-feature/all-target workspace Clippy, standalone Git check, fmt, 70 Node flows
and three restore helper tests pass. The explicitly executed retirement CLI
cancellation qualifier passes with a freshly built all-feature artifact. L0
passes for the corrective implementation; final closeout hashes are checked
separately. Hosted exact-head qualification remains separate; corrective run
42558349597 has Linux/secrets success and Windows in progress at observation.

## Synthetic validation

This rehearses the claim-settlement prerequisite for VAL-002/003/004: selecting
one old binding preserves its unselected sibling and aliases; an independently
owned job remains active; a stale original can be replanned without creating
another survivor source. Cancellation/interruption leaves original or survivor
protection and may retain extra claims. Historical receipt recovery does not
recreate a survivor later deliberately retired by another authorized operation.
No age/category selection, native semantic closure, actual F/D/M custody, payload
deletion, measured capacity reclaim or user acceptance is established.

## Six producer lenses

| Finding / role | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| RS-01 Identity and Namespace | High | Exact source/predecessor/path identities and canonical plans replace mutable labels. Same-hash aliases remain in the survivor root; an old retry cannot recreate later-retired protection. | Bulk selectors must bind exact native versions and preserve unknown coverage; final deletion still needs incarnation fencing. |
| RS-02 Placement Policy | High, corrected | Same owner/class is necessary but insufficient if an intermediate current root suppresses history. Atomic history adoption now hides that intermediate state; concurrent observations and faults prove the boundary. | Native/private extraction and independently held/non-releasable claims remain separate. |
| RS-03 Storage and Custody | High | Replacement metadata finishes and independently reads back before adoption; exact acknowledged survivor state precedes original retirement. Foreign/job and shared objects remain protected. | No custody or reclaim is inferred. Final deletion must separately settle provider-retention grants and integrate deliberate absence with retained-placement floors; ordinary keeper requirements stay intact. |
| RS-04 Concurrency and Recovery | High | Original expected revision and survivor condition fence original Retire in one transaction. New atomic history has all-or-none receipts; six process exits recover. Changed witnesses hold; unrelated admission continues. | Authority/epoch handoff, metadata lifetime and rollback-resistant external minima remain open. |
| RS-05 Provider and Privacy | Medium | CLI opens only existing private metadata/namespace roots, rejects overlapping mutation roots and defaults off. Plans omit private roots; public errors stay static. Sparse edits reuse the existing provider. | One new source per partial batch consumes finite lifetime capacity; journal/source continuation and provider durability qualification precede sustained use. |
| RS-06 Git and Adoption | Medium | No authored Git mutation, trigger, service or payload deletion is installed. Old-reader state/receipt qualification is explicit; code and verification/review close separately. | Automatic retirement selection/UI, native adoption and human acceptance remain open. |

Disposition: pass_with_risk for bounded claim-settlement implementation and the
named local evidence. This is a prerequisite of roadmap priority 7, not completion
of bulk final deletion or metadata retirement. It closes no full REQ/VAL outcome,
independent review, production acceptance or whole-plan goal.
