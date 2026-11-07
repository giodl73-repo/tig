# Git LFS local-cache compatibility profile

Input c68d7fad00b7fa6da528255f9b242a42e4dcb048. WP-005 component:
REQ-006/008/009; SPEC-005/007; ARCH-003/004; IF-005/008/009;
DES-005/007; PKG-004; GIT profile. Entry: raw identity, bounded tree lookup,
durable capacity accounting and audited REEL adapter contract. Exit: standard
pointer parsing, bounded standalone transfer protocol, existing CAS/LFS layout
lookup, disposable hard-link handoff and actual Git LFS checkout/restoration.

legacy consumer's current adapter is download-only. This compatibility profile preserves
that capability boundary; it does not claim generic upload/remote support or
replace producer ingest. REEL source e8fb31c6b21d74f111700d72076bb2a9d2d18176,
crates/reel-assets/src/lib.rs and src/bin/reel-lfs.rs, supplies the audited CAS
mapping, verified ordered lookup and disposable-link behavior. Software is MIT,
Copyright (c) 2026 Gio Della-Libera; Tig's LICENSE preserves those terms. No private
consumer records or media are ported.

The standalone agent requires an explicit absolute trusted profile. It installs
no hooks/config, creates no cache roots and follows no network actions. Profiles
pin an immutable authorization tree: OID path -> exact Object. Lookup reads only
bounded metadata nodes, never a full inventory or a cache census. Unsupported
pointer extensions/transforms reject rather than losing fields. Plain current
and historical Hawser SHA-256 pointer forms are recognized; no Tig pointer dialect.

Read places have configured order and layout: REEL CAS objects/sha256/aa/OID,
standard LFS objects/aa/bb/OID, or flat objects/OID. Disabled places are untouched.
Missing/offline candidates are unavailable, never durable absence proof. A present
corrupt or linked higher-priority candidate fails instead of hiding behind a
later healthy tier. Roots/handoff directories must already be explicitly prepared.

A successful download returns a NEW disposable hard link, never the cache path.
The link is verified before response. Hard-link creation must succeed on the
same filesystem: there is no full-copy fallback into cache staging. Transfer
ownership changes before attempting the complete response, preserving ambiguous
handoffs if stdout fails. Only unexposed owned aliases are cleaned automatically.
Git LFS may move the returned name; the canonical cache entry remains intact.

Each place carries a caller-admitted capacity ledger/job and metadata window.
The adapter records allocation before link creation, then completion with zero
new file-data bytes: a hard link shares existing data. Namespace/ledger overhead
is outside that basis and needs configured headroom. Short Busy retries reopen
verified accounting; no transaction spans linking/hashing. This is not total
physical accounting, cloud custody or lifecycle ownership. Git LFS's own store
and worktree materializations remain separate allocations requiring outer job
management; no zero-copy worktree promise is made.

Requests, JSON frames, metadata reads, object size, places and session count are
explicitly bounded. Current profile supports up to 64 GiB declared object size;
small executed streams do not prove native large-media or 500,000-object readiness.
External cache mutation/pruners require their own admitted coordination; this
read-only compatibility adapter grants no retained custody or eviction authority.
No live bindings/caches, archive hydration or production producer changes.

L0 fmt/structure/VTRACE; L1 locked tests/Clippy; L2 real temporary Git/LFS repos,
layout fixtures, concurrent transfers and historical restoration. Full WP-005,
VAL-001/004, upload, exact staged/ref lifecycle and complete accounting stay OPEN.

Primary protocol references, pinned to the locally available Git LFS 3.4.0:
[custom transfers](https://raw.githubusercontent.com/git-lfs/git-lfs/v3.4.0/docs/custom-transfers.md),
[pointer specification](https://raw.githubusercontent.com/git-lfs/git-lfs/v3.4.0/docs/spec.md).
CI records its installed client version and runs the same real-client fixtures.

Executed Windows client: Git LFS 3.4.0. EVID-022: 12 component entries;
EVID-023: three actual-client tests. Workspace total: 261 Windows entries.
Linux has one additional Unix cross-filesystem entry; its physical test requires
/dev/shm and skips if unavailable. No full-size media/500,000-object run claimed.

Profiles may explicitly set authority_checkpoint to use a read-only persistent
SQLite metadata root at authority_nodes; omitted checkpoint retains loose-node
behavior. There is no backend discovery or migration. max_authority_entries
defaults to 500,000 and accepts 1..2,000,000; metadata node-read/byte budgets remain
independent. EVID-030 exercises unflushed refusal followed by persisted-root
source-claim adoption and actual LFS checkout. New fields have compatible defaults
for existing experimental profiles. No released stable profile promise is implied.

Concurrent-probe refinement: another observer may supersede a probe between
its journal steps. Admission now retries the whole bounded probe/sample sequence
on stale/unknown observations or expired probes as well as SQLite contention.
It never spends rejected samples or retries closed/corrupt/insufficient jobs as
success. A deterministic two-handle interleaving covers this race (EVID-035);
short transactions and the existing eight-attempt bound remain unchanged.
