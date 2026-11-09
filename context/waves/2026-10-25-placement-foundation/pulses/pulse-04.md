# Pulse 04: Fake-provider safety

Goal: Concurrency, recovery, custody, scratch and measured reclaim fixtures.

Status: in progress. WP-000 choose/defer discovery is recorded. WP-003 adds a
bounded local publication subset: 24 test entries including a child helper,
ten injected protocol boundaries and four actual abrupt process exits. Full
persistence, restore coverage, admission, fencing and eviction remain open.
See specs/PUBLICATION.md and EVID-010/011. Workspace total: 85 test entries.

Validation: pwsh -NoProfile -File tools/validate.ps1; git diff --check.
When implementation exists, add fmt/locked Rust tests/Clippy and exact focused
failure fixtures before closing the pulse. Update docs/vtrace/REVIEW.md with
producer/independent method and exact evidence scope.


WP-004/006 checkpoint: object-scoped local OS guards and a pure single-authority
capacity model add 28 entries (113 total). Cross-process exclusion/release and
unrelated progress are tested; durable resource service, complete protection and
retirement remain open. See SCOPED-COORDINATION.md and EVID-012/013.


Reservation journal checkpoint: per-domain SQLite persists canonical commands
and capacity state transitions through replay, with no full-state manifest rewrite.
19 journal entries and one core rollback case bring the workspace to 134. Actual
process/commit/corruption tests are scoped in RESERVATION-JOURNAL.md/EVID-014;
physical writer integration, complete restore authority and live admission remain
open. No service or cache binding enabled.

Accounted publication checkpoint: EVID-015 adds 20 entries (154 workspace total).
Durable job admission precedes scoped writes; verified publication settles a
labelled conservative file-data footprint. Three real process exits and concurrent
attempts are tested. Actual volume probes/enforcement, cleanup and retirement
remain open. No live cache/service binding enabled.

Local sampling checkpoint: EVID-016 adds 12 entries (166 total). Real temporary
filesystem samples drive admission; explicit store/ledger enrollment, fresh-only
starts, failures/deadlines and racing writes are exercised. No live root is
probed on resume. Native domain/quota authority and full safety gates stay open.

Private-attempt cleanup checkpoint: EVID-017 adds 23 entries (189 total).
Read-only previews, exact explicit authorization, durable cancellation before
partial removal and remaining-metadata disposition are exercised, including
three child exits. No final-object eviction or live cache cleanup is enabled.

Replica verification checkpoint: EVID-018 adds 20 entries (209 total). Exact
completed bytes, held guards, failed revalidation, historical records and declared
domain coverage are exercised. A child exit demonstrates that a live guard is
not durable retention. No final-object removal or live cache operations added.

Durable retention checkpoint: EVID-019 adds 21 entries (230 total). Immutable
budgeted local claims survive process restart; exact grants, metadata accounting,
concurrent last-slot writers and three child exits are exercised. No expiry,
release/handoff, final-object eviction or live cache binding is enabled.

Owned handoff checkpoint: EVID-020 adds 16 entries (246 total), including reciprocal
guard overlap, onward custody and three child exits. EVID-021 records actual
legacy consumer adoption gaps and bounded source growth. No payload eviction, cache
migration or live runtime switch. Full claim/recovery authority remains open.

## Download-only LFS compatibility checkpoint

Input c68d7fad00b7fa6da528255f9b242a42e4dcb048; WP-005, REQ-006/008/009,
SPEC-005/007, IF-005/008/009, PKG-004. Controlled scope and entry/exit:
specs/LFS-CACHE-INTEROP.md. EVID-022 adds 12 Windows component entries;
EVID-023 adds three actual-client temporary-repo fixtures. Windows total 261;
Linux adds a Unix cross-filesystem entry, conditional on /dev/shm availability.
L0 structure/fmt/VTRACE; L1 locked workspace tests/all-target Clippy; L2 current
and historical checkout, two simultaneous clients, corruption and repaired retry.
No private caches or consumer configuration changed. Full WP-005, requirements,
validation, staged/ref adoption, upload and complete accounting remain OPEN.

## Exact Git source checkpoint

Input 07cfa97ce60a0d74f5923aaa96cdad8aad0e994b; WP-005, REQ-001/006/008/009,
SPEC-006/007, IF-008/009, PKG-004. GIT-SOURCE-EXTRACTION.md controls scope.
EVID-024 adds 16 temporary Git fixtures; EVID-025 connects exact source selection
through a complete OID tree to actual LFS transfer authorization. L0 structure/
fmt/VTRACE; L1 locked workspace/Clippy; L2 partial/conflicted staging, linked/bare
repos, missing/corrupt Git objects, disabled promisor hydration and historical
checkout. Windows total 278; Linux adds its Unix cross-filesystem entry. Complete
coverage is limited to explicit prefixes. Durable old/new claim handoff, global
attribute-aware extraction, large metadata scale and native adoption stay OPEN.

## Persistent source-root checkpoint

Input 5411e2be0521aef0e3722e508f31ff6355b8a66f; WP-008/009, REQ-002/004/007/009,
SPEC-005/006/007/008, IF-004/006/008, PKG-003/004. SOURCE-CLAIM-JOURNAL.md
controls scope and L0/L1/L2: structure/fmt/VTRACE, locked tests/Clippy, actual Git
root handoff plus six child exits. EVID-026 adds 19 journal entries including
helper; EVID-027 adds two Git/context fixtures. Full command cap replay executes,
not inventory-scale proof. Total 299 Windows / 300 Linux entries. Owner/revision
transitions preserve old/new roots; exact retries remain historical. Returned
obligations deny global completeness. Root closure, full source/job coverage,
final-removal fencing, restore authority, compaction and native adoption stay OPEN.

## Persistent metadata and scale checkpoint

Input 29e21c7673c4a50acade32228612ed849ac0ca3f; WP-001/008/009,
REQ-001/005/007/009, SPEC-002/005/006/008, IF-002/005/008/009, PKG-002/003/004.
PERSISTENT-METADATA.md controls L0 structure/fmt/VTRACE, L1 locked tests/Clippy,
and L2 persistent 500,000-binding synthetic scale plus actual LFS integration.
EVID-028: 14 provider entries including helper/three child exits. EVID-029: one
scale case, 13 edits adding 391 nodes / 212,992 database bytes, old root retained.
EVID-030: two core/integration cases. Total 316 Windows / 317 Linux entries.
Bounds are explicit; large-tree refusal is Budget. Full custody/admission,
compaction, complete native graph/source/job coverage and cutover stay OPEN.


Closed working-set retirement checkpoint: entry84136b; WP-003/006/008,
REQ-003/004/005/007/009, SPEC-003/004/005/008, IF-003/004/005/006, PKG-001/003.
CAPACITY-RETIREMENT.md sets entry/exit and RUST/PROVIDER L0/L1/L2.
EVID-KRY-01 records ten new pure/SQLite/process cases; 21 capacity and 26 ledger
entries pass, with full local suites/Clippy/format. Metadata-only retirement
preserves old events/receipts and accounting while admitting past a filled
working window. No payload IO, service deployment, compaction or complete
restore authority is claimed; current-head CI remains pending.

Storage orchestration checkpoint: inputbb51c5a; WP-003/004/006/007/008,
REQ-001/002/003/004/005/007/009, SPEC-002/003/004/005/008/009,
IF-002/003/004/005/006/012, PKG-002/003 and CLI PKG-008. The first shared
plan/local copy slice composes accounted publication and retention; EVID-084
records focused process/retry/source-preservation checks. Full protection registry,
eviction, cloud/upload, lifecycle events, bulk final deletion and metadata retirement
remain open. See specs/STORAGE-ORCHESTRATION.md for RUST/PROVIDER scope and levels.

Protection demand checkpoint: inputd18afba, WP-004/006/008/009;
REQ-001/002/004/005/007/009, SPEC-002/004/005/006/008;
IF-002/004/005/006/008, PKG-001/003. EVID-085 records bounded actual journal/root
projection and snapshot-to-copy integration. The View adapter still holds removal;
effective mandatory admission and intentional-absence recovery remain open.

Local placement composition checkpoint: inputa678381, implementation7432739;
controls and GIT/PROVIDER L0/L1/L2 in specs/PLACEMENT-RECONCILIATION.md. EVID-093
separately records sixteen placement entries, two projection cases, twelve Git/
CLI entries, broad/final-focused qualification and six producer lenses. Existing
mechanisms now compose bounded copy/retain/handoff/ordinary eviction/cancellation/
reacquisition, including job-end/no-op/outage catch-up. The pulse remains open:
trusted triggers, full lifecycle/native/provider adoption, large files, unattended
partial recovery, authority/lifetime, final deletion and metadata expiry are not
closed by this temporary-store checkpoint.

Trusted-request checkpoint: input1c253ea, implementation503adca/fixb3aced0;
GITHUB-TRIGGER.md freezes controls and GIT/PROVIDER L0/L1/L2. EVID-094 separates
sixteen library/eight authentication/thirteen Git cases, broader verification,
signed nonempty temporary-store rehearsal, hosted real issuer empty-scope proof
and six producer lenses. No service is installed. Full source/native/provider
adoption, large files, authority/lifetime, final deletion and metadata expiry
remain required; the pulse and whole goal stay open.

Staged-index checkpoint: input195bd0a, implementation51dd2b2. INDEX-LIFECYCLE.md
freezes controls and GIT/PROVIDER L0/L1/L2. EVID-095 records fifteen focused
entries (one child helper/two actual exits), real CLI/job-aware ordinary placement,
passing default/all-feature workspace/Clippy/artifact/L0 checks, synthetic
validation, trace and six producer lenses. Conflict alternatives are draft byte
obligations, never a merged selection. Hosted current-head checks and full
branch/commit/private/native/provider/authority/lifetime/final-deletion/metadata
acceptance remain open. No hook, consumer or live cache is attached.

Owned settlement checkpoint: input9c2a29f, implementation51c5acc;
SOURCE-SETTLEMENT.md freezes controls/L0/L1/L2. EVID-096 records three condition,
twelve settlement and one signed-source-withdrawal additions, broader regression/
old-reader/artifact checks, synthetic validation and six producer lenses. Existing
Withdraw/event bytes are unchanged. Source/docs ceiling is 6 MiB, with all runtime
limits and exclusions preserved. Full workflow/native/provider/authority/lifetime,
final deletion, metadata expiry, hosted current-head and user adoption stay open.


Roster precedence entry: input106f6f5; WP-004/006/007/009, REQ-002/003/004/005/009,
SPEC-004/008/009, IF-004/006/012, PKG-002/003/008. ROSTER-PRECEDENCE.md
controls fresh explicit group enrollment and shared planning/live assessment.
Legacy namespace behavior remains; synthetic qualification/closeout in progress.
No live upgrade, cache operation, source merge or full acceptance.


Bulk operator entry: input36fc91b; WP-003/004/007/008/009/015,
REQ-001/003/004/005/007/009, SPEC-003/004/005/008/009/014,
IF-003/004/005/006/008/012, PKG-002/003/008/009. BULK-OPERATOR.md controls
existing-root plan/peak/status and writer-free Inspection. Explicit CLI artifact
qualification and full regressions close apart from implementation. No bulk
mutation permission, native clock fallback, live stores or deployment.

Local-clock checkpoint: input4aeebf8, implementation9802166, portable fixture
pin43ad0f4. LOCAL-CLOCK.md names controls and RUST/PROVIDER L0/L1/L2.
EVID-103 and the six-producer review separate implementation from verification
and synthetic validation. Persisted high-water/native samples, complete-window
checks, independent vectors, three actual exits, full command exhaustion and
negative/provider/bulk fixtures pass. Default workspace/standalone/strict Clippy
pass; all-feature/fresh-artifact checks remain pending. No live trust/adoption,
mutation CLI, authority migration, lifetime or full pulse acceptance is implied.
