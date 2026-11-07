# Authorized private-attempt cleanup

Input 41a19b51b38b694cf1855ec42363cf66280203d5. Bounded WP-003/004/007/008:
REQ-003/004/005/007/009; SPEC-003/004/005/009; ARCH-002/003/007;
IF-003/004/005/006/012; DES-003/004/005/008; PKG-002/003; PROVIDER profile.
Entry: admitted publication, scoped attempt exclusion, persistent ledger and
local sampling. Exit: read-only exact-scope preview, explicit authorization,
durable cancellation before any partial removal, restart-safe retry and measured
remaining footprint before disposition. Full custody/retirement remains open.

Only unacknowledged private partials of one existing accounted attempt qualify.
No root census, recursive deletion, final-object eviction or automatic cleanup.
If a final object or completion exists, prefer publication recovery or integrity
incident handling; this cleanup refuses. Binding and intent records stay intact.

```text
inspect existing attempt under its nonblocking fence (no new files)
    -> bounded exact partial names / lengths / hashes / allocated sizes
    -> user/application explicitly authorizes discarding these unacknowledged bytes
    -> reacquire fence + recheck full preview + commit cancellation
    -> remove ONLY reviewed partials -> verify none remain
    -> measure retained metadata -> commit terminal footprint disposition
```

Publication rechecks cancellation after acquiring its attempt fence. A pending
writer cannot act on a pre-cancellation status. Cancellation is permanent for
this operation: retry uses a new publication operation and fresh budget. Failure
before terminal disposition preserves in-flight capacity. A retry with the same
reviewed plan can tolerate already removed files only after matching durable
cancellation. Changed/new files, damaged binding or different authorization fail
closed. A completed disposition cannot fund new writes.

Plans/authorizations are bounded canonical records, intended for runtime storage
outside Git. Identity and private filenames are not media/source evidence. A
cancelled attempt still charges retained metadata; unused job reservation is
released only when its caller explicitly finishes the job. Removed scoped file
allocation is not measured volume reclaim: hard-link aliases and filesystem
overhead may remain. A later admitted free-space sample measures availability.

This authority is an explicit discard decision for unacknowledged staging. It is
not independent retained custody or permission to discard acknowledged objects.
Trusted owned local namespace and cooperating accounted writers only; hostile
mutation, stale restored ledger authority, power loss, actual ENOSPC and full
retirement remain open. Legacy raw publisher APIs are not a cancellation-aware
runtime path. No live roots or consumers are admitted by these fixtures.

L0 structure/fmt/VTRACE/target hashes; L1 locked tests and all-target Clippy;
L2 real temporary partial removal, concurrent fences and process interruption.
Full VAL-002/003/004 and upstream package prerequisites remain OPEN.

## Implemented evidence

EVID-017: 23 new entries including a subprocess helper; workspace total 189.
Commands: cargo test -p tig-store --lib --locked accounted::cleanup::tests;
cargo test --workspace --locked; fmt/all-target Clippy. Temporary-store tests
cover exact preview/authorization, file/read limits, changed/missing/new files,
damaged binding/intent, final-object refusal, same-attempt contention, stale
publisher status, readonly terminal replay and four actual child exits: one producer interruption and three after
cancellation, first removal and disposition. No actual full disk/power loss,
hostile namespace, stale-backup restore authority or live cleanup is proven.

New BeginCleanup/CompleteCleanup journal commands bind the approved plan digest
and retain cancelled-operation tombstones. Generic allocation completion and
normal publication settlement cannot override cancellation. Existing command
wire encodings and local manifest remain unchanged. PublicationPlan now decodes
strict fields for bounded persisted cleanup plans; execution recomputes the
publication contract and matches current provider-enumerated files before IO.
