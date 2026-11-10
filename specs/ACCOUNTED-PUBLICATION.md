# Publication with restart-persistent admission accounting

Input 39c65eb1c4f97c3f7c7f07b71e3104a6eea52a25. Bounded WP-003/004/006/007/008
slice: REQ-003/004/005/006/009, SPEC-003/004/008/009, ARCH-002/007,
IF-003/004/006/012, DES-003/004/008, PKG-002/003, PROVIDER profile.
Entry: tested local publication, scoped exclusion and transactional capacity ledger.
Exit: bind admitted allocation before provider writes, serialize one exact attempt,
publish/recover verified bytes, measure a labelled bounded footprint and settle
only verified completion. No live caches, automatic cleanup or retirement.

The first profile charges a **conservative allocated-file-data footprint**, not
net volume growth. It sums allocated sizes of exact final/operation files and a
bounded private attempt scope. Existing shared data or hard-link aliases may be
charged more than once temporarily. A later coherent free-space observation clears
that conservative debit. Namespace/MFT, directory, SQLite and external-writer costs
are outside this evidence scope and require admitted headroom/measurement policy.
This is not complete physical-capacity enforcement or permission to run on E/F.

```text
fresh admitted probe -> reserve job peak -> bind publication allocation durably
                                              |
                           exact attempt fence + persistent binding
                                              |
                          bounded private write / verified publication
                                              |
                         measured scoped footprint -> durable settlement
```

The publication owner, request, place instance, job, limit and accounting basis
are pinned before bytes are written. A completed allocation cannot authorize new
writes on replay. Exact retry reuses its binding; changed requests/owners/places
reject. No DB transaction spans stream IO. Same-attempt contention is nonblocking;
other objects/attempts can proceed under their own budgets and object guards.

Private staging is attributed to one exact attempt. Unknown names, too many files,
incomplete payloads or allocation/measurement errors retain in-flight capacity.
An interrupted write is not retried into another full staging copy automatically.
Successful final publication can be recovered after a lost reply; a partial scope
without a verified final remains pending for explicit future disposition. No
broad staging census or automatic deletion of uncertain bytes is introduced.

The local prototype remains capped at 8 MiB payloads. Its budget estimate rounds
using provider allocation granularity and allows bounded metadata/staging slack;
it is not a proof against every filesystem overhead or external writer. Allocated
file-data evidence comes from fs2 0.4.3, with capability errors propagated. Source
bytes remain caller-owned until both publication and settlement acknowledge.
Provider/power-loss durability, complete restore authority, physical free-space
probing/enforcement, quota/multi-domain transactions and eviction remain open.

## Implemented scope

The local manifest advances to design-3 with a random immutable place instance;
old roots reject without automatic migration. Attempt bindings live under one
exact hashed operation directory in staging; up to 16 known files are inspected.
Only canonical binding.json and provider-generated partial names are allowed.
No full cache/archive inventory is performed. Attempt fences explicitly unlock
on scope exit, including early failures after acquiring the lock.

The ledger has distinct BeginPublication/SettleFootprint commands. A generic
allocation completion cannot relabel publication evidence. Raw publication APIs
remain low-level primitives; only publish_accounted enforces this admission
contract. No runtime consumers are switched to either path here.

fs2 preallocates the private payload and measures allocated file data. Its
[FileExt contract](https://docs.rs/fs2/0.4.3/fs2/trait.FileExt.html) is distinct
from total filesystem free space or independent custody. Namespace and ledger
overheads remain separately budgeted/unproven; no blanket no-ENOSPC claim.

20 new test entries, including a subprocess helper, cover insufficient/closed
budgets, wrong owner/store/request, exact retry after close, missing/corrupt
settled bytes, stream failures, four bridge fault points, three actual child
exits, over-budget evidence, unknown/damaged scopes and concurrent attempts.
A simulated StorageFull read error is not an actual full-disk test. Fixed probe
values are synthetic; file-data allocation is measured only in temporary stores.
The workspace has 154 entries. L0 structure/fmt/VTRACE, L1 locked workspace tests
and all-target Clippy, L2 actual temporary-store/process integration apply.
Commands: `cargo test -p tig-store --lib --locked accounted::tests` (20),
`cargo test --workspace --locked` (154). EVID-015 records this bounded scope;
full WP-007 and upstream safety/physical-admission gates remain open.

The later [local capacity probe](LOCAL-CAPACITY-PROBE.md) supplies measured
user-available free space for explicitly enrolled store/ledger pairs. Its EVID-016
scope does not close physical domain/quota discovery or complete enforcement.

[Private-attempt cleanup](PRIVATE-ATTEMPT-CLEANUP.md) adds explicit reviewed
unacknowledged-partial discard under EVID-017. Publication rechecks durable
cancellation after acquiring its attempt fence, closing the stale-status race.
Cancelled operations cannot be republished; this is not final-object retirement.

The [storage orchestration slice](STORAGE-ORCHESTRATION.md) adds optional
PublicationPlan context identity. None keeps legacy canonical bytes unchanged;
Some binds the whole orchestration plan into the existing allocation command and
private-attempt binding. Reusing an operation with changed context rejects through
existing replay. Cleanup preserves/revalidates the context. Old readers reject
context-bearing records; explicitly enroll compatible readers before use.
