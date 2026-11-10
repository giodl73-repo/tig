# Backup-outage teaching review

Runtime implementation: 73069222eaa0999b2eebb69786373917353f0162.

Input9310d0cbe48ba796f0feaa7847ed5fba10e74b48; feat/teaching-backup-outage.
[Contract](../../specs/TEACHING-BACKUP-OUTAGE.md) names controls, allowed scope,
entry/exit and L0/L1/L2 profiles. Targets: fresh-root outage sample, explicit
partial-observation protocol, verified reader/UI, independent oracle and tests.

## Implementation

Explicit rehearse-backup-outage includes normal v1/draft publication, then makes
M's configured path unavailable at merge. The existing SDK returns actual held
reports. Job end changes desired placement, while old F remains protected;
useful publication into D can proceed. Restoring the same provider and retrying
the same plan/source checkpoint completes ordinary demotion. A counted ByteSource
fixture wrapper denies seed reads during retry, proving protected local copies
suffice. It delegates normal verified reads to the existing ReadCache; it is not
a cache engine or alternate placement solver. No core execution changes.

Only a verified canonical backup child is renamed within the fresh owned root;
both source/destination boundaries and return path are checked. This models
configured-path unavailability, not actual hardware-fault qualification. Existing
roots refuse. All reports retain synthetic-domain/hardware limits. No private
cache, migration, network provider, general source adoption or final loss.

Report design2 requires explicit per-object unknown_copies arrays. Known/unknown
roles must be sorted, unique, admitted and disjoint. Partial receipt rows must
exactly cover plan objects/providers, have matching required bits and known state
or held reasons. Unknown roles require corresponding held provider evidence;
held providers with no verified copy cannot silently omit unknown coverage.
Unknown coverage cannot claim convergence. Null is not an absent versioned field.
Legacy report1 rejects any asserted unknown array/null; complete legacy books
retain exact response1 shape. Books with new reports use response2 and explicit
unknown arrays, including empty arrays on complete rows.

The shipped view accepts both versions, shows known copies plus Unknown/provider
chips in Before and Observed after, and labels partial known copy counts/bytes.
Unknown-only does not become zero copies. Malformed/conflicting/unversioned/
falsely converged coverage refuses. No route, query argument or execution bridge
is added. The same bounded authenticated metadata reader remains outside the
catalogue mutex, with16 reports/64KiB per document/2MiB reads/64KiB response.

## Verification

Initial6403 check failed E0624 on a private identity method. The fixture now
uses public Place::local validation; no SDK visibility or protection was weakened.
Native66637 passes five actual CLI/provider/catalogue cases. Final78266 passes
strict schema workspace/all-target Clippy plus the same five after explicit
non-null coverage deserialization. Actual HTTP output is passed into shipped
controls: M is unknown, F persists while held, Before preserves prior unknown,
and only verified catchup yields D+M without F. Plan/source identity stays fixed
and seed attempts during retry are zero. The public literal oracle independently
checks F/D/M copies; no observations are fabricated from desired state.

Ten repinned admitted negatives test overlap, foreign/missing coverage, legacy/
legacy-null fields, false convergence, suppressed unknowns, missing held evidence,
duplicate receipt pairs and wrong required bits. Valid evidence still loads after
refusals. Older ordinary/exchange/worktree, environment isolation and metadata
integrity/refusal cases pass. All76 shipped controller cases pass, including three
new partial/unknown-only/invalid-coverage cases. These are not browser acceptance.

Default12177 and all-feature3970 terminate zero in separate external targets:
both locked workspace suites/both500k/strict workspace profiles and standalone Git.
Only CLI usage text changed after broad starts; all-feature strict Clippy and
fresh artifact25647 qualify it. Artifact25647 terminates zero: fmt, fresh schema
bins, explicit expanded-usage refusal, nine-checkpoint outage CLI and catalogue
check (read_only=true,payloads_opened=0 during enrollment validation). The public
persistent sample is retained outside source. Initial structure/pins/diff/source
secrets pass. Final documentation pins/diff and two-commit secret scan qualify
the source pin; exact-head hosted checks remain open. Parent PR48 run42753891927 at9310d0c passes
Windows/Linux/secrets and is ready; no merge/deployment. Parent qualification is
not hosted proof of the new partial-observation boundary.

## Validation and trace

EVID-118 connects WP-004/008/009/010/015/016; REQ-001/005/006/007/009/010;
SPEC-002/005/008/010/014; IF-002/005/008/010/011/013;
PKG-002/003/004/006/008 through TEACHING-BACKUP-OUTAGE.md, existing SDK effects,
literal oracle, native/API/negative/shipped-script cases and this review.
Nineteen specified stages include three new outage/catchup stages; this mode
executes nine actual reports. The still-unavailable final-retirement stage is
not executed. No complete VAL, real hardware/remote-byte publication, general
writer/current authority, full faults/restarts/final-loss/expiry course or native/
provider/large-file/restore/lifetime/browser/classroom/full product acceptance.

## Producer role review

All six [.roles](../../.roles/ROLE.md) lenses are producer checklists, not
independent reviewers, human approval or publication authority. Owner: maintainers.
Disposition: pass_with_risk for locally qualified outage/coverage teaching; hosted
and intended-use gates remain open.

| Lens | Severity / finding | Disposition and remaining evidence |
| --- | --- | --- |
| Identity/namespace | High: unknown roles could alias providers or wrong plan rows | Exact trace role binding, canonical pinned objects and complete typed partial receipt matrix; duplicate/mismatch negatives. |
| Placement policy | Critical: job end could trigger F eviction with unavailable M | Real SDK held job-end report keeps F; same current plan retry only after custody revalidation. No label/final-loss inference. |
| Storage/custody | Critical: prior backup receipt could masquerade as current custody | Explicit unknown M coverage and partial counts, no converged claim, actual physical preservation. Hardware qualification remains open. |
| Concurrency/recovery | High: retry could grow authority or replace the source decision | Same plan/source checkpoint and zero seed attempts; namespace/provider SDK guards reused. Full crash/restart/native writer gates remain open. |
| Provider/privacy | High: outage move could affect another root | Exact canonical fresh child checks, fresh destination and verified restoration; no existing cache/data or remote URL. |
| Git/adoption | High: missing/unknown could be shown as zero or a completed lesson | Versioned boundary, actual held-provider evidence, unknown-before/after chips, known-only counts and false-convergence/legacy-null refusals. Browser/classroom and full course remain open. |
