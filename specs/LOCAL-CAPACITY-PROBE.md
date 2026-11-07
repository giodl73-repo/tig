# Explicit local capacity sampling

Input b642dd8c9f3fce6c40ae516c3cc6e76633272870. Bounded WP-006/007/008 component:
REQ-003/004/005/007, SPEC-003/004/005/008/009, ARCH-002/003/007,
IF-004/005/006/012, DES-004/005/008, PKG-003, PROVIDER profile.
Entry: restart-persistent accounting and accounted local publication tested.
Exit: an explicitly enrolled store/ledger pair samples actual user-available
filesystem space between newly committed probe start and observation, checks
store identity before/after sampling, and refuses historical probe reuse.

Enrollment is a trusted caller assertion that the store belongs to this capacity
domain. Bind exact random store instance and ledger genesis, never just labels.
It does not discover physical volume identity, mount changes with copied
manifests, quotas, shared aliases or multiple independently enrolled ledgers.
All places sharing a volume/quota must use one admitted accounting authority.
No enrollment file, cache initialization or service is installed automatically.

```text
explicit store + ledger enrollment (no IO mutation)
    -> invalidate this handle's old admission freshness
    -> commit a NEW probe ticket
    -> verify store -> fs2 filesystem statistics -> verify store
    -> validate user-available <= total and nonzero allocation granularity
    -> commit observation within existing five-second freshness limit
    -> reserve job -> accounted publication
```

Historical receipt replay is marked explicitly. A replayed probe never runs the
sampler and cannot grant a new admission. Failed, malformed or late samples do
not restore this handle's old freshness. No database transaction spans the sample;
allocation events racing the sample retain the existing conservative watermark
charge until a later coherent sample. Other handles remain independently subject
to ledger admission rules; this is not global revocation of previous tickets.

Use available space for the current caller, not privileged total free blocks.
Sampling is an observation, never a physical reservation or custody proof.
External writes may consume space immediately afterward. Namespace, ledger,
quota/volume enrollment authority, full-disk recovery, power-loss durability,
partial cleanup and eviction remain open. Only synthetic temporary roots are
used for integration verification; no live cache or runtime consumer changes.

L0: fmt, structural checks, canonical VTRACE and target hashes. L1: locked tests
and all-target Clippy. L2: actual filesystem statistics and publication/reopen
fixtures, plus injected failure/malformed/racing sampling. Full VAL-002/003/004
and upstream package exits remain open. Evidence is recorded after execution.

## Implemented evidence

EVID-016: 12 local-probe test entries, 166 workspace entries. Actual temporary
filesystem samples drive admission and publication; dynamic space values are
not asserted to be stable. Other fixtures cover explicit identity mismatch,
reopen, replay, failed/malformed/expired sampling, root mutation, shared-domain
headroom and publication during sampling. The deadline fixture waits 5.1 seconds;
it does not simulate power loss or an actual full disk. All real sampling uses
owned temporary roots only. No capacity detection is performed on resume.

The low-level manual ledger observation APIs remain available for other admitted
providers; this component does not authenticate a malicious caller's numbers.
Existing completed replies expose was_replayed without changing journal wire
bytes or acknowledged checkpoints. Empty new ledgers can enroll before their
first observation; history validation must not require a prior capacity sample.
