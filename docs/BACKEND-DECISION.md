# Existing placement system or separate Tig engine?

Status: bounded WP-000 discovery recorded; first local publication adapter chosen
below. Distributed placement/backend acceptance remains OPEN. No service installed.

Tig names the product/interface independently of its implementation. Art consumes
resolved bundle/claim and observation/plan contracts, not a backend's mutable
catalog as a second semantic asset authority.

| Candidate | Documented capability | Questions requiring fixtures |
| --- | --- | --- |
| iRODS | Logical namespace, metadata, policy-driven tiering | Deployment/Windows fit, offline mode, catalog recovery, bounded IO and claim adapters |
| Rucio | File/dataset identifiers, replica rules and metadata subscriptions | Operational footprint, existing-data relabel reconciliation, local stores, custody/deletion semantics |
| Tig Rust engine | Proposed minimal local-first implementation | Every integrity, concurrency, recovery, capacity and provider obligation remains to be built |

Sources: [iRODS capabilities](https://irods.org/documentation/),
[iRODS namespace/catalog](https://irods.org/faq/),
[Rucio replica rules](https://rucio.github.io/documentation/started/concepts/replica_management/),
[Rucio subscriptions](https://rucio.github.io/documentation/started/concepts/subscriptions/).
Metadata subscriptions for newly registered data do not by themselves prove that
changing an existing label triggers the reconciliation Art needs.

Transport is a separate choice. Prefer verified local copy on one workstation;
optional BitTorrent can acquire immutable objects/shards from admitted peers.
[BEP 52](https://www.bittorrent.org/beps/bep_0052.html) defines v2 per-file hash trees.
A bundle checkpoint can reuse shards; no monolithic torrent per asset revision.
Cross-torrent local reuse and scratch consumption require actual client fixtures.
Private torrents alone do not establish confidentiality or retained backup.

Acceptance matrix: one local workstation; disconnected archive; authenticated
remote place; two simultaneous writers; interrupted copy; corrupted destination;
shared object; expired claim; stale removal; rename/path collision; full disk;
small change in a large bundle. Compare operational cost and measured behavior.
No benchmark, portability or compatibility result is claimed from this survey.

## WP-000 decision checkpoint, 2026-10-25

Input main 3ecce389891212191ca53124e3906e1009a4ba5a. Scope: local-first
publication with independent provider/coordination ports, before placement or
removal. This is a documented choose/defer decision, not a comparative benchmark.

| Capability | Disposition and basis |
| --- | --- |
| Initial payload publication | Minimal Rust local adapter in explicit isolated roots; no daemon or server prerequisite |
| Shared protection/publication protocol | Immutable owned intent before final bytes, completion after verification; retries keyed by exact operation/request |
| Persistence | Process-restart acknowledgement first; refuse provider/power-loss durability until directory/metadata persistence is established |
| iRODS | Defer optional adapter: documented metadata/tiering/replication fit; Windows/offline footprint, recovery and Tig claim semantics unmeasured |
| Rucio | Defer optional adapter: replication-rule union maps well; server/database deployment and local/offline integration not admitted by this workstation slice |
| Distributed catalog / locks | Defer; no inferred replacement of proven deployments, no global cache lock |
| Remote providers / BitTorrent | Defer to provider capability/privacy and transport fixtures |
| Eviction / final deletion | Unsupported; require WP-004/006/007 and actual custody/physical accounting |

iRODS documents policy-based synchronization, tiering and replication patterns;
that supports future adapter investigation, not a Windows/offline compatibility
claim. [Official capability overview](https://irods.org/documentation/).
Rucio's rules require minimum replicas and derive necessary transfers; its
installation guide describes a server and configured database. Treat that as a
separate deployment choice. [Rule model](https://rucio.github.io/documentation/started/concepts/replica_management/),
[server setup](https://rucio.github.io/documentation/operator/installing_server/).
The server guide includes dated Python prerequisites; no version/platform support
claim is taken from that section. Neither product was installed or benchmarked.

REEL source e8fb31c6b21d74f111700d72076bb2a9d2d18176 was read as a design
donor: bounded streamed hashing, private create-new partials, file sync, no-clobber
hard-link publication and exact destination verification are useful patterns.
No REEL runtime code/dependency, native registry or cache configuration is copied.
Its broader guards/admission cannot be assumed present in the new adapter.

Rust documents file synchronization and no-clobber hard-link behavior separately;
neither alone establishes directory-entry persistence or independent backup.
[File synchronization](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all),
[hard links](https://doc.rust-lang.org/std/fs/fn.hard_link.html).
The adapter must expose unsupported guarantees honestly and use trusted owned
roots; adversarial filesystem mutation needs later handle/identity fences.
