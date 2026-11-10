# Recorded teaching playback review

Input6d9a2e024a2ddaeae5409a4df3d7077a8c9b1965; feat/teaching-playback.
Controls, scope, entry/exit and L0/L1/L2 profiles are in
[TEACHING-PLAYBACK](../../specs/TEACHING-PLAYBACK.md).
Targets: optional catalogue enrollment, bounded metadata reader, authenticated
read route, sample evidence export, shipped lesson controls and fixtures.

## Implementation

The ordinary sample now exports its actual canonical reports through the existing
flat reader. Trace design2 explicitly pins provider-to-role order. The optional
teaching profile binds the exact trace and enrolled project. Older enrollments
without teaching remain valid. Old trace design1 is not silently reinterpreted.

The GET endpoint snapshots enrollment under the catalogue mutex, releases it,
then verifies bounded report metadata. It verifies object hashes, canonical JSON,
schema, plan/receipt identity, provider order, object identity/size and effect
scope. It caps16 reports,64KiB per document,2MiB reads and64KiB output. It exposes
recorded rehearsal evidence, never current custody or storage execution authority.
No browser paths, copy/delete commands, live discovery or new storage engine.

Shipped controls show previous observations, desired plan roles and subsequent
recorded observations separately. Missing rows stay unknown. The same digest
follows job-held original bytes, demotion and revert. Catalogue navigation retains
lesson selection. Late successes/errors cannot replace another selected view;
failed reloads clear prior lesson controls. Enrolled text uses text nodes.

## Verification

Focused native chain52463 passes two actual CLI/provider/catalogue tests. The
sample producer reads back all seven reports through the new reader. Independent
assertions verify F+M -> D+M -> F+M for the same original object. An owned loopback
child serves the shipped JS and authenticated API; uncredentialed/query requests
refuse. Missing/corrupt bytes refuse. Repinned synthetic negative reports test
unsupported schema, receipt/plan mismatch, foreign roles, false hardware claims
and object-size conflicts; a17-report trace refuses. Valid evidence still loads
after each refusal; no managed payload store is edited by the negative tests.

All73 shipped-script controller tests pass, including seven lesson cases covering
optional enrollment/no IO, transitions, return selection, missing observations,
stale success/error, failed reload/retry and text rendering. These use a DOM double
and literal DTO oracle; they do not prove browser layout or provider effects.
Native integration supplies the separate provider/API evidence.

Default26395 is terminal zero: locked workspace suite including500k, strict
workspace Clippy and standalone Git. Final1180 is terminal zero: fmt, actual
sample/API/HTTP -> shipped-controls bridge, all repinned negative cases, strict
schema workspace Clippy and a fresh persistent seven-checkpoint CLI followed by
catalogue check (zero payload opens during enrollment validation). The same
actual HTTP response renders job hold/demotion/revert in the shipped scripts.
All73 controller cases pass again. All-feature22256 is terminal zero, including
the second500k and strict all-feature/all-target workspace Clippy. Broad
compilation predates the final test bridge and fuller pending-course label;
final1180 qualifies these. Initial structure/pins/diff and changed-source secrets
pass. Final structure/pins/fmt/diff and changed-source secrets pass.
Exact-head hosted qualification remains pending.
Parent PR45 run42737465924 at6d9a2e0 passed Linux/secrets, failed Windows in the
inherited simultaneous age-capture test because neither peer acknowledged.
That failure remains open; it is not evidence against or proof of this reader.

## Validation and trace

EVID-115 connects WP-008/010/015/016; REQ-001/005/006/007/009/010;
SPEC-002/005/008/010/014; IF-002/005/008/010/011/013;
PKG-002/004/006/008 to the contract, implementation and native/controller cases.
This rehearses the teaching interaction with public synthetic bytes, with no
classroom/human acceptance or completed VAL. Source availability means membership
in the currently enrolled commit/ref set, not a promise to resolve all Git history.

Publish/fetch/pull, index/worktree variants, failure/restart/stale execution,
separately authorized final block/category/age loss and metadata expiry remain
course obligations. Native writer/current authority, provider/large-file/restore/
lifetime and full operator/Action acceptance remain open. Three declared domains
do not establish independent hardware. Full PRODUCT_PLAN remains open.

## Producer role review

All six [.roles](../../.roles/ROLE.md) lenses are producer-applied checklists,
not independent people, approval or publication authority. Owner: maintainers.
Disposition: pass_with_risk for locally qualified recorded playback; hosted and
intended-use acceptance remain open.

| Lens | Severity / finding | Disposition and remaining evidence |
| --- | --- | --- |
| Identity/namespace | High: role permutations or same name/new bytes could mislead | Trace2 pins provider order; hash/size and name identity checks plus negative fixtures. Broader native closure remains open. |
| Placement policy | Critical: absent observations could look like zero demand | Null/missing remains unknown; desired plan sets and observed copies are separate. Full failure/final-loss lessons remain open. |
| Storage/custody | Critical: historical reports could imply current recoverability | Explicit recorded mode and hardware limitation; current copy inspection remains separate. No mutation bridge or removal authority. |
| Concurrency/recovery | High: slow metadata or stale replies could disrupt selected view | Metadata IO outside global catalogue mutex; generation/mode fences; failed reload clears controls. Native restart lessons remain open. |
| Provider/privacy | High: arbitrary paths, unbounded metadata or executable labels | Private existing reader, fixed route, byte/count budgets, canonical hash verification, text nodes and session protections. No private assets/config copied. |
| Git/adoption | High: playback could be mistaken for complete command teaching | Actual ordinary sample, documented learner path and explicit pending course. Browser/classroom acceptance remains open. |
