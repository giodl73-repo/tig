# Same-byte rename teaching review

Input21a70f5abc4340582cda3a075a3c5fe1cee13e40; feat/teaching-rename.
[Contract](../../specs/TEACHING-RENAME.md) names controls, scope, entry/exit and
L0/L1/L2 profiles. Owner: maintainers. Producer review is not independent review
or human approval. No live data, merge or deployment is authorized here.

## Implementation

Explicit rehearse-rename adds one actual checkpoint to the ordinary seven.
A conflicting git mv into the existing reference record must refuse; exact
source/destination bytes, HEAD, index tree and managed copy snapshot remain
unchanged. Successful git mv changes the authored path and creates a commit while
preserving exact record bytes and Git blob identity. Native record IDs and
payload objects remain stable. Explicit source adoption reuses the existing
native adapter; actual SDK plan/apply must converge on the current basis with
zero streamed bytes, no provider effects/holds/continuation and unchanged copies.
The independent literal stage checks F/M shared+fixed and D/M original.

Canonical rename.json pins commits, old/new paths, blob, record object and actual
copy snapshots. The CLI result pins its exact hash/length. Catalogue enrollment
uses the renamed commit. Existing protocol1 records the actual plan/receipt;
no new reader/UI protocol, alternate placement solver or cache engine is added.
Git's authored path is distinct from provider-owned payload paths. The fixture
does not establish that every possible rename in a native application is inert.

## Verification

Terminal95832 passes the focused locked native CLI/provider/catalogue HTTP test
and strict schema all-target Clippy. The test checks pinned companion bytes,
changed commit, stable record identity, unchanged copies and zero actual effects.
Actual authenticated HTTP evidence flows through shipped scripts. A subsequent
renderer assertion explicitly visits the rename lesson, checks all three objects'
recorded roles and the no-new-provider-effects message; broad runs qualify it.
All78 controller cases pass. Structure/pins/diff passed before this review.

Default/schema15009 and all-feature80546 terminate zero in separate external
targets: both workspace suites/both500k/strict workspace-all-target profiles,
all seven teaching cases and standalone Git. Terminal artifact3407 passes fmt, fresh schema binaries,
eight-checkpoint persistent sample and catalogue check with read_only=true and
payloads_opened=0. Its separate external teaching-rename-artifact target avoids
overlapping the active suites. Final suite outcomes/pins/secrets/commit and
exact-head hosted results remain pending. All local jobs are terminal; none was
restarted due to timeout. Initial source-secret scan passed; final pins/commit
secrets remain separate. Structural validation passes after shortening a historical
review-index summary under128KiB while retaining its full detailed review link.

## Validation

Eight actual reports teach source path change without redundant byte transfer.
The conflict is a real Git refusal, not an invented DTO outcome. Public synthetic
seeds and declared fixture domains do not prove independent hardware backup.
Browser/classroom acceptance remains open; no browser/viewer was launched.
Twenty-three specified stages still need bounded chapters/paging for a complete
course. Remote bytes, remaining fault/restart/final-loss/expiry lessons and full
native/current-authority/provider/large-file/lifetime/product acceptance remain.

## Trace

EVID-120 connects WP-004/008/010/015/016; REQ-001/005/006/007/009/010;
SPEC-002/005/008/010/014; IF-002/005/008/010/011/013;
PKG-002/004/006/008 through TEACHING-RENAME.md, existing APIs, literal oracle,
native/API/shipped-controls assertions and this review. Full goal remains open.

## Producer role review

Method: apply all six [.roles](../../.roles/ROLE.md) checklists to the named input
and rename runner/oracle/test targets. Owner: maintainers. Disposition:
pass_with_risk for locally qualified teaching; hosted/human gates pending.

| Lens | Severity / concern | Disposition and remaining evidence |
| --- | --- | --- |
| Identity/namespace | High: path change mistaken for byte identity change | Exact record/Git blob and asset identities preserved; native snapshot and literal copy checks. |
| Placement policy | High: rename could duplicate or withdraw demand | Existing native/source SDK adoption and actual zero-effect placement; no filename policy inference. |
| Storage/custody | Critical: zero effects could conceal lost custody | Actual before/after verified copy snapshots plus independent F/D/M oracle; hardware independence remains unproven. |
| Concurrency/recovery | High: failed rename could partly modify source | Real conflicting destination checks exact bytes/HEAD/index/copies; this is not general crash/race qualification. |
| Provider/privacy | High: sample could rename real payloads | Only fixed authored paths inside a verified fresh root; provider APIs remain unchanged. No remote/private roots. |
| Git/adoption | High: UI could show a generic success without native evidence | Actual HTTP feeds shipped controls and explicitly selects new lesson; no browser acceptance claim. |

Implementation, verification, validation, trace and review stay separate. This
review grants no deletion authority and does not close PRODUCT_PLAN.md.
