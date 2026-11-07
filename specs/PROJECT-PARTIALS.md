# Explicit restore partial inspection

Entry `19790c58f35ca764fc66ca1f818a56bf9b75de90`; WP-005/011/015/016,
REQ-001/003/006/007/010, SPEC-004/007/011/012/015, IF-005/008/010/013,
PKG-003/004/008; RUST + INTEGRATION. Scope: bounded read-only directory metadata
observations for an exact project restore plan. No payload open, cache discovery,
cleanup, source mutation, transfer, ownership inference or global lock.
L0 format/foundation/lint; L1 strict flags/bounds/path/name/refusal tests;
L2 synthetic real Git, missing roots/source payloads, held writers, interrupted
restore/retry, linked directories/entries and actual CLI. Exit requires passing
scoped tests, producer review and separate exact-head hosted qualification.

```text
tig-project --expect-plan SHA256 --inspect-partials LOCAL_PROFILE MANIFEST_PATH
```

This mode requires an expected plan and excludes apply/progress/inspect-progress.
It captures metadata using existing exact Git planning but does not verify or
open source/destination payloads. It lists only the immediate parent directories
of explicitly planned destinations, never recursively or from an inferred cache
root. Repeated identical logical parents are scanned once. Missing parents are
reported without creating them. Existing directory ancestors must be ordinary:
links, reparses, files and unavailable ancestry refuse before directory listing.
As in restoration, this is a caller-owned trusted namespace, not a sandbox
against hostile directory replacement races.

Profile `limits.partial_directory_entries` defaults to 4,096 and admits 1..16,384.
That default and the private-name syntax are part of this experimental contract;
changing them requires contract review rather than silently reinterpreting a plan.
Every observed directory entry, including unrelated files, consumes that budget.
Excess refuses instead of returning a truncated report as complete. The bound
is part of the raw-profile plan identity. No directory/provider discovery beyond
the declared parents, file-content reads or unbounded inventories are introduced.
An entry bound is not an interruptible filesystem deadline; slow or unavailable
filesystems retain the existing explicit local-IO limitations.

Only the exact current private-name syntax `.tig-restore-<32 lowercase hex>.partial`
is reported. Other names count toward scan limits but are not treated as managed
data. Ordinary matching files are `unassociated-private-partial` with observed
logical lengths. Matching non-files are `non-file-refused`; links/reparses,
vanished or unreadable matching entries are `unverified-entry` with unknown size.
No entry is followed, hashed, selected, removed or adopted. Sorting is by exact
relative logical path. Reports contain private paths and belong asset-side.

Names do not prove Tig created a file, which plan/attempt owns it, whether a writer
is active, or whether it is disposable. A live writer or another plan can use the
same parent. The result explicitly denies ownership, active-writer knowledge,
payload verification, atomic-snapshot and cleanup/eviction authority. Logical
length totals are observations across named paths, not unique physical allocation
or reclaimable bytes; aliases/hard links and concurrent changes prevent that claim.
Physical reclaimable bytes remain unknown. No safe-to-delete classification exists.

Explicit retry still verifies completed destinations through the existing guarded
copy primitive. Inspection never replays a progress log or cleans an old partial.
This closes bounded visibility only; authenticated attempt attribution, fenced
authorized disposal, complete job/custody coverage and actual consumer adoption
remain separate open obligations. No production cache scan follows from merging.
