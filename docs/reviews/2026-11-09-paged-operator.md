# Paged retirement operator review

Input: cec0217f5bae71b43f5ca43a3e3027b4c2fbb426; branch
feat/paged-operator. Controls, entry/exit and RUST/PROVIDER L0/L1/L2 are in
[the contract](../../specs/PAGED-OPERATOR.md). Targets: existing-root operator CLI,
shared metadata Session, copy-index extension/readback and synthetic qualifications.
No new cache/journal engine, live roots, deployment or source merge.
Implementation: 8f5d39edb25129ad26f750e257faf2584b72cbbd. No runtime changes follow
this pin; subsequent review/checkpoint updates are documentation only.

## Implementation

Implemented bounded candidate: tig-storage paged stages copy chunks and exact whole-selection
decisions, explains original scope with bounded cursors, reports witness peak
and clock-free status, and composes separately approved apply/resume/cancel with
the existing controller. Default permissions are false. Only the independently
admitted private profile supplies roots, minima and the two approval digests.
Witness-only stop precedes source/provider/metadata cleanup. A witnessed stop is
preserved if later cleanup fails; uncertain observations stay unknown.

The existing copy-index schema remains canonical. Exact retries write nothing;
extension reuses original request documents, with sparse existing-Tree edits when
few ordinal positions change. One Session preserves counters and deadline across
writer finish, replacement reader and independent scope readback.

## Verification

Initial22 paged library entries pass, including stable manual/aged decision
frames. Initial fresh-executable20-page manual and aged CLI workflows pass,
including separate consent, bounded resume, shared-copy preservation, actual
enrolled clock/registry and offline cancellation. A repeated aged test exposed
an insufficient fixture wait: the clock's conservative bound includes5 s sample
and1 s drift, so3 s was not enough. The fixture now waits7 s without weakening
production eligibility. A subsequent resume correctly refused an execution
window that had started before the capture/wait setup; the fixture now freezes
its window immediately before planning and never changes an approved window.
A copy extension test's64-write allowance also excluded
the rebuilt101-entry Tree; reuse is now tested under128 writes, and a separate
sparse-append case keeps the tighter64-write allowance.

Pagination passes200-row fixtures with both tiny and10 KiB rows. Final four
copy-index regressions and fresh manual/actual-clock-aged20-page CLI qualifiers
pass after the fixes above (chain42894). Default workspace (including500k),
strict workspace Clippy and standalone Git chain42894 terminate zero.
Native artifact/schema Clippy chain48425 also terminates zero: both native
index/select executables hand off the exact request to the storage operator,
which settles more than16 pages, deletes the six-byte eligible unique copy and
retains verified shared bytes. That run takes87.77 s. Its initial100 s execution
window refused a slow resume; the fixture now admits a fixed240 s window under
a300 s policy/session. No approved window is refreshed. Representative latency
and scale remain release gates.

The initial native fixture used a private helper; public verified reads and
deletion receipts now assert the cross-package outcome. Its initial plain-name
filter also refused; it now supplies exact admitted record References.
All-feature72525 stopped on the older compiled64-write fixture, and45832 stopped
on the private-helper compilation. Their replacements qualify changed source;
they are not restarts for observation silence. Final all-feature workspace and
strict all-feature/all-target Clippy chain44984 terminate zero, including the
second500k metadata run. Its test compilation precedes only the ignored native
qualifier's final window constants;48425 executes those constants and final Clippy
covers current source. L0/target pins/fmt/diff and changed-source Gitleaks pass.
Hosted exact-head qualification remains pending. Ordinary ignored entries are
not executed evidence; CI invokes each explicit CLI qualifier with a fresh executable.

## Validation and trace

Synthetic operator rehearsal is scoped to temporary enrolled namespaces,
metadata, providers, ledgers and actual LocalClock. It does not establish native
writer authority, project deprecation/inactivity semantics, real provider custody,
large-file/restore/lifetime behavior or F/D/M intended-use acceptance. Full
agent/UI/Action integration and metadata expiry remain required. No requirement,
invariant or full roadmap milestone closes from this component alone.

## Producer role review

One producer applied all six [.roles](../../.roles/ROLE.md) checklists; no
independent or human approval is claimed. Owner: Tig maintainers through the
named lens. Disposition: pass_with_risk for the locally qualified operator
candidate, pending hosted checks. PO-07 and full product acceptance remain open.

| Finding / lens | Severity | Disposition | Remaining gate |
| --- | --- | --- | --- |
| PO-01 Identity/namespace | High | Portable decision binds exact original selection and copy indexes; original-scope cursors bind decision and kind. Canonical framing is preserved. Both native executables hand off an exact request to actual operator execution in48425. | Independently current source/native writer admission. |
| PO-02 Placement policy | High | Historical held subjects and final eligible actions stay fixed; approval is checked separately for claim release and final loss. | Complete current/branch/job/native coverage; project continuous-inactivity/reset policy is not inferred from an old occurrence timestamp. |
| PO-03 Storage/custody | High | Three actual eligible copies disappear in the manual/aged rehearsal; shared/current bytes survive. Witness peak has a named limited basis. | Provider/large-file/physical-volume qualification; no secure-erasure or whole-volume reclaim claim. |
| PO-04 Concurrency/recovery | Critical | Witness stop precedes optional cleanup; clock/registry availability is not required. Shared Session cannot reset consumed IO. Existing controller owns source/provider fences and receipts. | Final fault/restore/child-exit/artifact scope and surviving authority/lifetime evidence; bounded fixtures are not independent latest authority. |
| PO-05 Provider/privacy | High | Existing absolute private roots/minima only; strict documents, bounded reports, no credentials or discovery. Unused clock configuration cannot block read-only status or cancellation. | Actual private profile/OS isolation and provider capability enrollment. |
| PO-06 Git/adoption | Medium | Explicit opt-in commands and CI artifact qualifiers; ordinary bulk profile encodings remain unchanged. | Full native/agent/UI/Action, released operator workflow and human acceptance. |
| PO-07 Held-copy follow-up | High | Paged preparation settles the expressly approved original binding pages even when an original provider copy is held. Its old action set must never grow later. Unlike ordinary bulk's protected survivor projection, that historical occurrence can therefore be unavailable to a fresh subset selector (`subset::original_source` requires a retained predecessor). | WP-007/009: provide a fresh exact-copy follow-up over retained original scope/receipts and independently current protection/provider authority. It needs a new decision/approval and must not recreate history or revive the old operation. Exercise offline-provider recovery and a foreign job ending after claim settlement. Full block-retirement acceptance remains open until this path exists. |
