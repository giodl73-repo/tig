# Bounded persistent Merkle metadata

Input 29e21c7673c4a50acade32228612ed849ac0ca3f. WP-001/008/009;
REQ-001/005/007/009; SPEC-002/005/006/008; ARCH-001/003; IF-002/005/008;
DES-001/005/007; PKG-002/003; PROVIDER/GIT profiles. Entry: immutable directory
trees and durable root obligations. Exit: bounded disk-backed NodeStore with
restart/corruption/concurrency/fault tests, actual 500,000-binding synthetic
construction and small-edit/history sharing measured without a Git inventory.
L0 structure/fmt/VTRACE; L1 locked tests/Clippy; L2 source-root and scale fixtures.

Runtime metadata resides in an explicitly owned directory outside ordinary Git.
This provider stores small immutable tree nodes, not media payloads. It implements
the existing NodeStore seam: exact SHA-256 plus size on every returned node;
missing nodes remain missing, never empty-tree success. Opening performs bounded
manifest/counter checks, not a node scan or media/cache census. Physical paths
are provider-owned; no daemon or automatic enrollment/configuration is installed.

Writes buffer at most 1,024 nodes / 1 MiB. Flush uses a nonblocking short SQLite
transaction, deduplicates exact existing bytes and atomically updates counters.
Auto-flush may persist a construction prefix. Only explicit finish acknowledges
the pending nodes; publish/protect the new root after successful finish. Old nodes
are immutable and there is no prune/eviction API. Failed/uncertain writes require
verified reopen; dropped buffers and interrupted builds do not remove old roots.
Other handles cannot read unflushed private nodes. No transaction spans a whole
500,000-node build, media hash or transfer.

Configured logical node/byte bounds and an enforced SQLite database page ceiling
limit growth. A free-space guard runs before each flush, reserving conservative
database/journal headroom in the check. This is not an atomic multi-session volume
reservation or measurement of every SQLite temporary/namespace allocation. Caller
metadata job admission and full capacity accounting remain OPEN. Tests create
only admitted-size synthetic temporary metadata; no native cache is attached.

Minimum checkpoints pin store identity and monotonic generations/counters. They
detect known rollback but cannot prove a globally latest backup, full database
integrity or availability of all tree closures. Counters are declared metadata,
not object custody or a complete inventory scan. Actual node reads verify hashes;
root/source owners must handle missing/corrupt dependencies conservatively.
Process restart is the scoped acknowledgment; no power-loss/provider-durable
or final-retirement authority is implied.

Limits: nodes <= 2,000,000; logical bytes <= 2 GiB; database <= 4 GiB in 4 KiB
pages; individual nodes <= 65,536 bytes. Root metadata and journal compaction,
backup providers, native adoption, global source/job coverage and media payload
retirement remain separate obligations. API failures must refuse growth without
silently dropping nodes or weakening an existing root.

SQLite controls: [max_page_count](https://www.sqlite.org/pragma.html#pragma_max_page_count)
limits database pages; [temporary file documentation](https://www.sqlite.org/tempfiles.html)
describes additional journal/temporary allocations, whose implementation details
are not a promise of total storage use. No journal is disabled to improve results.

## Executed scale and integration

Windows Rust 1.98.0 debug build; bundled SQLite; synthetic ASCII names/payload
identities and owned temporary metadata. EVID-028 covers 14 provider entries
including helper and three actual child exits. EVID-029 is one scale case.
EVID-030 adds one core limit case and one real Git LFS/claim integration case.
All growing raw logs/databases remain temporary, outside ordinary Git.

| Measurement | Recorded Windows result |
| --- | --- |
| Logical bindings | 500,000 |
| Persisted node count | 500,003 |
| Node bytes | 155,601,465 |
| Database file bytes | 247,840,768 |
| Initial generation/build/finish | 156,629 ms |
| Replacement bindings | 13 |
| New nodes / new node bytes | 391 / 153,237 |
| Database growth | 212,992 bytes |
| Edit node reads / writes | 1,108 / 391 |
| Edit plus finish | 451 ms |

These are one synthetic debug run's measurements, not product latency promises.
Tests enforce generous upper bounds, not incidental exact timing/counts. Five
post-reopen lookups require fewer than 512 SQL node reads; opening performs zero
node reads. Node-read counters count point queries, including dedup checks, not
physical disk IO. The old root remains readable after new-root persistence.

The first scale run exposed a 65,536-entry default at lookup. Clients now carry
an explicit larger inventory limit (LFS default 500,000), separately from bounded
per-path node reads. Limit refusal reports Budget; it does not relabel a valid
large tree as corrupt. No wire identity changed to pass the workload.

MetadataReader opens the existing database read-only and refuses writes. The
LFS profile explicitly chooses it with a minimum checkpoint; otherwise existing
loose-node behavior remains. The real client rejects unflushed metadata, then
uses the finished root with persistent source claims and preserves canonical
payload cache names. Complete native source/graph scale, custody, all physical
allocation costs, shared admission and backup/recovery authority remain OPEN.
Current workspace: 316 Windows / 317 Linux entries.

The final writer uses a separate small OID index with append-oriented node body
rows and a 64 MiB SQLite cache ceiling; read-only connections retain an 8 MiB
ceiling. The earlier clustered-body layout measured 229,410 ms and 239,968,256
database bytes locally. The final layout measured 156,629 ms and 247,840,768 bytes
under the same 500,000-binding debug fixture. This trades about 7.5 MiB for a
roughly 32% shorter observed initial build; these are observations, not guarantees.
Node content identities, logical bytes and 391-node sparse edit size are unchanged.
SQLite's [layout guidance](https://www.sqlite.org/withoutrowid.html) explains why
larger rows may benefit from an ordinary rowid layout. Both original Windows CI
and local scale runs stayed live until completion; performance changes were
tested against the same workload, rather than reducing its size.
