# Live local replica verification

Input 18910df6a1afad4cbedaf59389336ea482639985. Bounded WP-003/004/007/008:
REQ-003/004/006/007; SPEC-003/004/005/009; ARCH-002/003/007;
IF-003/004/005/006/012; DES-003/004/005/008; PKG-002/003; PROVIDER profile.
Entry: completed publication verification and object-scoped shared guards.
Exit: read-only verification of an exact completed publication, bounded live
session holding the object guard, explicit declared-domain coverage and bounded
portable historical observation. No retention installation, copy or eviction.

```text
expected publication + enrolled store instance
    -> verify completion/intent + exact bytes
    -> open verified reader and hold its shared object guard
    -> recheck completed publication / store identity
    -> live verification session (in-process freshness only)
       |                         |
       v                         v
declared-domain coverage    canonical historical observation
(live sessions only)        (never a live session or retention authority)
```

The expected object hash/length and owner/operation/ack level are authoritative
inputs; an indexed file without matching completion is insufficient. The adapter
remains capped at 8 MiB and process-restart acknowledgement. It adds no files,
repairs no markers and scans no roots. Missing anchors/bytes, damaged records or
identity changes fail. Revalidation obtains fresh byte evidence before replacing
the old session. Expiry rejects coverage; it does not unlock or authorize removal.

Coverage uses at most 16 retained sessions and 17 explicit store-to-failure-domain
enrollments. Store instances must be unique, objects identical, evidence recent
and acknowledgement adequate. Repeated/aliased stores cannot inflate counts;
several retained stores in one declared domain count as one domain. The source's
declared domain never contributes to independent retained coverage.

Enrollment labels are trusted caller assertions, not native volume, provider or
physical independence proof. A report can meet declared integrity/domain coverage
while retention remains unestablished. No coverage report or serialized observation
is eviction authority. Durable retention commitments, authoritative restore,
claim completeness, native failure-domain identity and final-removal revalidation
remain separate prerequisites. No unbudgeted retention metadata is written here.

Guards protect only cooperating local actors. Raw/hostile writes, namespace
replacement, copied manifests, network filesystems and power-loss guarantees are
outside this profile. All integration uses synthetic owned temporary roots.
No live cache or consumer changes. L0 structure/fmt/VTRACE/target hashes;
L1 locked workspace/all-target Clippy; L2 exact real temporary byte/record/fence
and restart fixtures. Full VAL-002/003/004 and package exits remain OPEN.

## Implemented evidence

EVID-018 adds 20 entries including a subprocess helper; workspace total 209.
Commands: cargo test -p tig-store --lib --locked replica::tests; locked workspace,
fmt and warnings-denied all-target Clippy. Real temporary fixtures cover exact
records/hash/length including empty and multi-buffer payloads, no-write verification, missing/corrupt evidence, bounds,
acknowledgement levels, duplicate stores/domains, stale and invalidated sessions,
reopen and a child-held guard released on process death. Fixture domain labels
are synthetic assertions, not proof of different physical disks.

Verification makes one bounded payload hash pass while holding the same guard
across pre/post record and store checks. Record UTC milliseconds are provenance
only; live freshness uses a process-local monotonic clock. A saved observation
can explicitly request new verification of its exact store; decoding alone cannot
restore a session. Failed revalidation invalidates evidence while retaining its
guard. No persistent retention claim or unbudgeted metadata write is introduced.

The later [durable retention](DURABLE-RETENTION.md) component installs budgeted
process-restart claims and verifies them with the live replica session. Plain
replica observations/coverage remain integrity-only; retention-aware coverage is
separate and still grants no final-object eviction authority.
