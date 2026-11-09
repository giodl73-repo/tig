# Explicit local clock authority

Input4aeebf8ad136c69a7cf3ad399807e7d461ca82f8.
WP-003/004/006/007/008/009/015; REQ-001/003/004/005/006/007/009;
SPEC-003/004/005/008/009/014; IF-003/004/005/006/012;
PKG-002/003/008/009. RUST/PROVIDER profiles. Allowed: fresh clock-enabled
capacity ledgers, an explicitly enrolled OS clock adapter, bounded time windows
and synthetic qualification. No live root, upgrade, bootstrap on resume, service,
bulk CLI activation or second journal engine. Entry: independently trusted OS
time boundary, immutable policy and actual existing journal/checkpoint. Exit:
accepted lower time persists before use, stale/rollback/skew/unknown time holds,
window checks use the whole interval, restart/peer/crash and legacy refusal prove
the bounded contract. Implementation and V/V/trace/six producer review close apart.

## Authority and persistence

This first profile is a proposed local-clock choice; no user preference was
recorded after the asynchronous question. Explicit enrollment admits OS wall time
as the authority. It does not discover/verify NTP, promise externally correct UTC,
resist a hostile administrator, or detect restoration of both journal and every
trusted checkpoint. Latest authority and stronger time services remain admission
gates. SystemTime is not a fallback for an unknown requested clock identity.

The policy binds authority, epoch, native-time boundary and finite sample-age,
wall/monotonic drift and invocation-duration limits. The enrollment identity binds
that policy and the actual ledger genesis; a new ledger cannot impersonate the old
clock. A fresh design-2 capacity marker includes the immutable clock policy.
Design-1 bytes/identity stay unchanged and cannot acquire clock commands. Older
strict readers refuse design-2 markers. No in-place migration exists.

ObserveClock is a normal bounded hashed command in the existing SQLite ledger.
Its returned timestamp must not go backwards; the first accepted sample of a new
adapter invocation must advance beyond the retained floor. Exact command replay
remains historical and cannot mint fresh time. A derived in-memory high-water
value is reconstructed incrementally from existing events; no new SQL journal,
global cache lock or payload IO. The existing 16,384-command/16 MiB bounds and
normal process-restart acknowledgement apply. Exhaustion holds; no ledger reset
or new genesis can preserve the old clock identity. Continuation remains required
before sustained operation.

## Sampling and decision interval

Each fresh call samples OS wall time and monotonic marks, persists its lower
timestamp, then resamples after commit. Backwards monotonic/wall movement,
excessive sampling/commit latency, drift against the invocation anchor, invocation
expiry, missing/corrupt/spent evidence or failed persistence grant no time.
No replayed journal result is returned as a fresh clock observation. Reopening
never treats out-of-format native timestamps as recordable values: both interval
ends must fit the existing canonical-number contract before any append.
Reopening
may start a new monotonic anchor but cannot reset the durable floor or weaken the
independently supplied minimum.

Native observations return an earliest/latest interval covering admitted sample
age plus drift at the final sampling point. All final-deletion, grant-release,
reacquisition-birth and initial bulk checks require the whole interval inside
[not_before, expires), with a nondecreasing accepted lower bound. Old exact Clock
implementations keep a default point interval; record encodings stay unchanged.
The persisted record time is the accepted lower bound, not proof of external UTC.

Rust documents [SystemTime](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
as non-monotonic and leaves suspend behavior of
[Instant](https://doc.rust-lang.org/std/time/struct.Instant.html) unspecified.
Use checked durations; never saturate a backwards clock into apparent success.
Suspend/drift can therefore hold an invocation. These are cooperative sampled
decision checks, not a hard real-time promise that unlink completes before a
deadline despite an arbitrary OS pause after the final check.

L0: fmt, reviewed hashes, portable validator, diff and six producer lenses.
L1: independent canonical/hash framing, legacy and negative/interval tests,
locked default/all-feature workspace, strict Clippy and standalone Git.
L2: actual OS samples and temporary durable journals, peers/reopen, injected
clock/commit boundaries and abrupt exits, final provider/reacquisition/bulk window
checks and explicit pinned older-reader refusal/legacy control. Native host/time
trust, whole-system restore, live adoption, power-loss/authority handoff and full
operational lifetime remain open. This does not complete operator execution.
