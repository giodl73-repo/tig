# Read-only source-journal observation

Entry 36f277cada879ded677a396bdcf8d8bbc9ac3e1c. WP-008/009/015;
REQ-002/004/007/009, SPEC-005/006/008, IF-004/006/008, PKG-003.
Scope: an explicit existing journal, trusted minimum and source key produce a
single replayed source/checkpoint observation. Reuse existing bounded canonical
replay and witness verification. No new persistent format or mutation command.

Add a genuinely SQLite read-only open path, exposing only copied observation
data rather than a mutable journal handle. Source state and head must come from
the same replay; do not separately refresh one of them. No create, recovery,
adopt/abort/retire, liveness inference, source-absence eviction authority or
claim-completeness assertion. An observation can become stale immediately;
existing expected-revision and ownership checks still govern all later commands.

L0 repository/fmt/locked Clippy. L1 empty/current/pending/predecessor state,
read-only-file access, unchanged DB/marker, foreign/rollback/missing/corrupt
refusals and concurrent writer snapshot coherence. L2 existing real-Git/source
journal and process-exit regressions. Producer roles: concurrency/recovery,
provider/privacy and Git/adoption; independent review remains unclaimed.

The explicit `tig-source inspect` CLI keeps this capability usable without REEL
or typed assets. It reads an explicit bounded private profile, emits only counts
and an observed revision/null, and writes detailed roots/owner/checkpoint state
to a bounded private report outside the authority directory. Synced owned temp
plus same-parent no-overwrite hard-link publication avoids replacing existing
files; handled failures clean the temp. Unsupported filesystems refuse. No
source IDs, raw digests or paths appear in successful stdout/refusal diagnostics.

Local outcome: the 23-entry source-journal suite passes (260.74 seconds), including
the three new read-only/coherence cases and existing restart/exit/limit regressions.
An additional explicit before-commit isolation case passes separately: observation
returns committed absence while the writer waits, then sees its committed root.
Three actual CLI cases pass with PATH empty, unchanged authority bytes/counts,
absent-source null revision, exact checkpoint, overwrite/inside-authority refusal,
foreign/missing authority and redacted diagnostics. Locked all-target/all-feature
workspace Clippy passes; no new dependencies or journal format are introduced.
Final structure and exact-head hosted qualification remain required.

Role disposition: producer-applied pass_with_risk for observation only.
Concurrency/recovery confirms no automatic adopt, abort, owner takeover or timeout
expiry; an observation never bypasses later CAS/owner checks. Provider/privacy
confirms read-only SQLite flags, bounded private output and scoped error handling.
Git/adoption confirms no hook installation or runtime changes. Open high,
integration owner: source/job enrollment, durable checkpoint management, actual
recovery/compaction and complete custody remain outside this implementation.
