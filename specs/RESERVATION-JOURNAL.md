# Restart-persistent reservation ledger

Input a24d962cd0b47ed2045414dd67e1c17d37394fed. Bounded WP-003/004/006/008
slice: REQ-003/004/005/007/009, SPEC-003/004/005/008, ARCH-002/003,
IF-003/004/005/006, DES-003/004/005/008, PKG-003 in tig-store, PROVIDER profile.
Entry: tested capacity model/local exclusion and explicit trusted temporary roots.
Exit: per-domain transactional commands, idempotent replies, bounded incremental
replay, process-restart recovery and concurrent admissions. Full packages remain
in progress; no production cache admission, allocation measurement or eviction.

## Storage and transaction boundary

One embedded SQLite database per capacity domain, outside Git. SQLite handles
transaction commit/rollback and local multiprocess writer coordination. No database
service is installed. An immediate transaction spans only metadata sync, capacity
transition and commit; actual writes, rendering and free-space probes are outside
it. Zero busy timeout returns Busy rather than pausing unrelated sessions/domains.
Rollback journal and FULL synchronization are configured and checked. This slice
claims process-restart behavior only, not universal filesystem/power-loss safety.
[SQLite commit assumptions](https://www.sqlite.org/atomiccommit.html),
[synchronization modes](https://www.sqlite.org/pragma.html#pragma_synchronous),
[rusqlite](https://docs.rs/rusqlite/0.41.0/rusqlite/).

Commands are small canonical JSON events with unique operation IDs and an exact
previous hash/sequence chain. SQLite atomically commits an event and its head.
An open handle replays only unseen events; first open performs bounded indexed
replay. No asset/cache directory census or monolithic manifest rewrite is used.
Limits: 16,384 events, 16 MiB aggregate encoded journal payload, 65,536 bytes per
event; core reservation/history limits also apply. Capacity metadata consumes its
own storage and needs a future compaction/retention policy. A full ledger refuses
growth instead of silently discarding closed-operation history.

Root creation/open are explicit. Missing/corrupt databases and inconsistent tails
block, never initialize an empty replacement. A caller supplies a trusted minimum
checkpoint; a missing/rewritten acknowledged prefix rejects. A consistent rollback
older than every available witness is fundamentally undetectable here. Production
restore must reconcile durable witnesses and active workers before admission;
the minimum checkpoint is not complete distributed protection authority.

## Safety and replay

References bind domain, epoch, genesis and operation. Dependencies must have the
right result kind. Identical retries reproduce the original result; conflicting
reuse rejects. The checked core transition is staged and adopted only after SQL
commit. An uncertain database/hook result poisons that handle until explicit
reopen; no potentially stale cached model continues admitting work.

Reopen retains reservations/in-flight writes but never treats an old free-space
sample as fresh permission. New reservation admission requires a newly issued and
completed probe on that handle, within five seconds, consumed by one new reserve.
Existing idempotent retries do not consume new capacity. Probes capture the core
allocation watermark before the caller samples; no DB transaction spans a probe.
Logical allocation results must come from admitted provider-visible byte evidence.
Zero disposition is not permission to forget uncertain writes. No TTL releases
worker reservations or removes bytes.

This ledger still trusts local filesystem/database integrity and admitted caller
authority, probe coverage and allocation facts. Hostile namespace replacement,
network filesystems, quota/multi-domain coordination, physical space enforcement,
compaction and complete restore/final retirement remain future work.

## Implemented checkpoint

rusqlite 0.41.0 with bundled libsqlite3-sys 0.39.1 provides the embedded engine;
getrandom 0.4.3 supplies a unique ledger instance identity. Cargo.lock pins the
complete graph. This changes no local publisher root format or runtime binding.
Core try_transition stages bounded accounting and adopts after successful metadata
commit; staged tokens must never drive physical work before successful return.
The journal records commands, not a full serialized capacity snapshot per edit.

References include the unique genesis, so independently initialized roots with
identical domain/epoch labels cannot exchange references. Replay validates exact
canonical bytes, request identity, row sequence/hash chain, committed head and
aggregate byte count. SQLite integers are checked before conversion; request
JSON retains the strict safe-integer subset (not the core's entire u64 range).
Initial open is bounded full journal replay. An existing handle queries only the
indexed tail; last_replay_count counts replayed events, not physical disk reads.
Historical idempotent replies keep their original accounting/checkpoint, distinct
from the current accounting view. Neither an old reply nor reporting available
bytes is new admission authority.

19 journal test entries (including a subprocess helper) and one additional core
staging test raise the workspace total to 134. Tests use actual temporary SQLite
databases: three injected commit-stage failures, process exits before/after commit
and with in-flight work, a child-held domain transaction with another domain
progressing, concurrent connections, corruption/gaps/oversize records, stale backup
witness rejection, probe freshness and indexed-tail replay. Fixed free/allocation
values are synthetic, not measurements of physical capacity. No real full-disk,
power-loss, network filesystem, hostile writer or unwitnessed-restore proof exists.

Commands: `cargo test -p tig-store --lib --locked ledger::tests` (19 entries),
`cargo test -p tig-core --test capacity --locked` (18),
`cargo test --workspace --locked` (134), fmt and all-target Clippy. EVID-014
records this scope. Full WP-003/004/006/008 and intended-use gates remain open.

CI follow-up: explicit guard unlock on drop prevents duplicated OS handles from
delaying release. A deterministic duplicated-handle regression adds one fence
entry, included in the current total; the journal-specific filter still runs 19.

The later [accounted publication](ACCOUNTED-PUBLICATION.md) component connects
job reservations to bounded local attempts and labelled measured file-data charges
under EVID-015. It does not close whole-volume enforcement, restore or removal.

The [private cleanup](PRIVATE-ATTEMPT-CLEANUP.md) extension adds canonical
BeginCleanup/CompleteCleanup events for exact plan-bound cancellation and terminal
remaining-footprint disposition. Old command bytes are unchanged; older readers
cannot interpret new commands. Cancelled operations remain tombstoned and cannot
settle as normal publication. Full restore authority and compaction remain open.

[Durable retention](DURABLE-RETENTION.md) adds distinct BeginRetention and
SettleRetention commands without changing old command encodings. Generic and
publication completion cannot release or relabel these allocations. Metadata
admission precedes provider writes; no SQL transaction spans verification/IO.

[Owned handoff](RETENTION-HANDOFF.md) adds BeginHandoff/SettleHandoff without
changing prior command encodings. Metadata is admitted before marker writes and
charged after exact old-claim retirement; generic completion cannot relabel this
allocation. Settled replies are historical and never current custody proof.


[Closed working-set retirement](CAPACITY-RETIREMENT.md) adds a canonical
RetireClosed command and immutable historic dispositions. It frees only finished
model slots, preserves all journal rows and old command replay, and changes no
accounting or payload. Older readers reject the new command; compatible software
must be explicitly enrolled. Journal size/command limits and restore witnesses
remain unchanged; this does not implement journal compaction.
