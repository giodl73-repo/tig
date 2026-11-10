# Public Git exchange review

Runtime implementation: 8f17aa9a711fcb6d372fb2f267f67ef2ac313692.

Input618628a1571082474576061de4510ee54a59f1ad; feat/teaching-exchange.
[Contract](../../specs/TEACHING-EXCHANGE.md) names controls, allowed scope,
entry/exit and L0/L1/L2 profiles. Targets: the public oracle, fresh-root sample
runner, native/API/shipped-controls tests and learner documentation.

## Implementation

Explicit rehearse-exchange reuses the ordinary seven-checkpoint runner, then
creates a fresh local bare Git remote and observer checkout. It actually pushes,
clones without local hardlinks, fetches and fast-forward pulls. Three additional
checkpoints reuse existing source adoption and placement/provider APIs; no new
placement engine, transport provider, storage command or browser bridge.

The observer first sees v2; fetch advances its remote ref but leaves HEAD intact.
An exact synthetic overlapping edit makes pull refuse. The fixture checks the
English overwrite diagnostic, unchanged dirty bytes/HEAD and unchanged hash-checked
managed copies. Only that fixture edit is repaired. Successful pull selects exact
v1 records before explicit source adoption and reconciliation; a second exchange
returns to v2. The observer is a metadata reader, not an admitted asset writer/job.
This is not general live-source adoption or an automatic pull hook.

The new file-transport capability is private to fixed relative paths generated
inside the fresh root. Ordinary Git calls still deny all transport; inherited
GIT_* routing/configuration, global config, attributes, templates, hooks, signing,
lazy fetch and prompts remain disabled. C locale makes the refusal classification
portable. No URL or Git arguments are accepted from the browser or CLI caller.

The independent public oracle has eleven specified stages: seven ordinary,
three exchange and one still-unavailable final-retirement target. Ten actual
sealed reports appear in the same existing catalogue lesson reader. The publish
checkpoint streams zero payload bytes. Clone's tracked tree excludes selected
payload seeds; metadata transport does not establish backup custody or hardware
independence. Existing reports retain the explicit synthetic-domain limitation.
The bounded exchange.json CLI evidence pins actual commits/refusal/copy snapshot;
the CLI result supplies its exact object hash/size. Ordinary result shape, trace2,
existing report schema and read-only API remain compatible.

## Verification

Initial65714 passes three actual CLI/provider/catalogue cases. Final45494 passes
the same three after tightening refusal classification to the actual overwrite
diagnostic with C locale, then strict schema workspace/all-target Clippy.
The new case uses paths with spaces and poisoned GIT_ALLOW_PROTOCOL=ssh. It
checks evidence integrity, distinct exact commits, unchanged fetch HEAD, refusal,
literal original-object copy states, zero publish streaming and existing-root
preservation. Actual authenticated HTTP output is passed into the shipped scripts
and all ten checkpoints render, including the three new role transitions with the
same object hash. The older ordinary/negative/HTTP and Git-environment-isolation
cases pass unchanged. All73 controller cases pass.

Default43090 and all-feature54984 terminate zero in separate external targets:
both locked workspace suites/both500k/strict workspace profiles and standalone Git.
No Rust edits followed their start. Fresh artifact91098 also terminates zero:
fmt check, explicit schema bin build, ten-checkpoint exchange CLI and catalogue
check (read_only=true,payloads_opened=0 during enrollment validation). The public
persistent sample is retained outside source. Structural/pin/diff and changed-source
secret checks pass. Final documentation pins/diff and the two-commit secret scan
qualify the source pin. Exact-head run42748580854 at27a6eb4f3ebd11743d2772694ec2ea2b3242732f
passes Windows, Linux and secrets; PR47 is ready for review, no merge/deployment.
Browser/layout/keyboard acceptance is separate from the shipped-script fixtures.
Parent PR46 exact-head run42744449594 at618628a terminates SUCCESS for Windows,
Linux and secrets; PR46 is ready for review. No merge/deployment. This is parent
qualification, not hosted proof of the new exchange code.

## Validation and trace

EVID-116 connects WP-004/008/010/015/016; REQ-001/005/006/007/009/010;
SPEC-002/005/008/010/014; IF-002/005/008/010/011/013;
PKG-002/004/006/008 through TEACHING-EXCHANGE.md, real Git/source/provider exercise,
the independent oracle, native/API/controller assertions and this review.
This rehearses metadata exchange and explicit byte reconciliation on public data.
It closes no full VAL or command curriculum. Remote byte publication, offline
backup, general worktree/index/current-writer adoption, full faults/restart,
separately authorized final loss and metadata expiry remain course obligations.
Browser/classroom, native/provider/large-file/restore/lifetime and whole product
acceptance remain open. No live storage, source merge or deployment.

## Producer role review

All six [.roles](../../.roles/ROLE.md) lenses are producer checklists, not
independent reviewers, human approval or publication authority. Owner: maintainers.
Disposition: pass_with_risk for locally qualified exchange; hosted and intended-use
qualification remain open.

| Lens | Severity / finding | Disposition and remaining evidence |
| --- | --- | --- |
| Identity/namespace | High: fetched ref could be mistaken for selected checkout | Exact HEAD/ref/record checks; fetch does not advance HEAD. Full native writer/source authority remains open. |
| Placement policy | Critical: Git exchange could imply eviction permission | Only explicit adoption/reconciliation invokes placement; dirty refusal invokes no Tig action. Literal copy oracle and old/new/shared obligations are checked. |
| Storage/custody | Critical: local Git remote could be presented as payload backup | Tracked-tree seed exclusion, zero publish streaming and existing synthetic-domain labels; real remote byte publication/offline backup remain open. |
| Concurrency/recovery | High: failed pull could lose uncommitted bytes or reuse stale state | Exact dirty bytes/HEAD/copy preservation before repair; exact commit selected before admission. Full concurrent writers/restart lessons remain open. |
| Provider/privacy | High: local transport could enable private/external access | Fresh root, fixed generated relative paths, file-only capability at selected calls, isolated config and no user URLs. Existing roots refuse; no credentials/private assets. |
| Git/adoption | High: a metadata observer could be confused with an enrolled writer | Explicit reader-only observer limitation and separate source-adoption step; ten actual recorded checkpoints, with full course/browser/classroom gates open. |
