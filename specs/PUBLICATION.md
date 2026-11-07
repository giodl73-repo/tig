# Publication protocol and first local adapter

WP-003 bounded slice. Input main 3ecce389891212191ca53124e3906e1009a4ba5a.
Controls: REQ-003/004/009, SPEC-003, ARCH-002, IF-003/004, DES-003,
PKG-002/003, PROFILE-003 (PROVIDER). WP-000 choose/defer decision is recorded
in BACKEND-DECISION.md. Scope: new tig-store crate, explicit temporary stores,
streamed publication, operation replay and fault/reopen tests. No live cache,
daemon, broad census, transfer configuration or removal.

Entry: pure exact identities available; immutable intent/request and bounded
local publication contract chosen. Slice exit: truthful process-restart results,
operation conflict rejection, no-clobber verified objects, retry/reopen and
injected publication ordering faults. Full WP-003 exit remains OPEN: provider
power-loss guarantees, journal restore coverage, durable claim recovery/admission
and all provider persistence faults require further work.

```text
validate capabilities / request bound
              |
protect exact operation+owner+object+ack-level intent
              |
reuse verified final object OR stream into private partial
              |
verify exact size/hash -> sync file -> no-clobber publish
              |
verify final destination -> record exact completion -> acknowledge
```

Operation IDs bind the whole request; conflicting reuse rejects before bytes
change. Publication sources remain caller-owned until acknowledgement. A failed
attempt may dispose only its own private partial. Abrupt process death can leave
partials; recovery must not interpret them as complete or silently sweep them.
An acknowledged object missing/corrupt on replay is an explicit integrity failure,
not a normal cache miss. No API in this slice deletes completed objects/intents.

Capability levels distinguish verified-local, process-restart and provider-durable.
The local adapter supports at most process-restart, with file sync and separately
verified completion. It refuses provider-durable/power-loss requests before IO;
it does not claim directory flush, redundant custody, device/site loss survival
or removal authority. A returned result is conditional on its admitted provider
contract, not proof that any arbitrary port implementation is trustworthy.

Roots are explicitly initialized/opened by caller in owned locations, never by
repo startup. SHA-256-derived operation/object names, bounded strict metadata and
symlink/type rejection restrict paths. Concurrent trusted Tig writers use private
partials and no-clobber publication. Adversarial path replacement/TOCTOU and
unmanaged writers require later handle-based fencing; these stores cannot be
admitted as production custody for that threat model.

Per-object byte caps bound IO, not physical free-space reservation. Automatic
retry loops, stale-partial cleanup, full-disk admission, catalog recovery and
global protection coverage remain later engine work. Test fixtures must stay
small; expanding operation records/observations live outside ordinary Git.

## Implemented checkpoint and limits

The publication coordinator and provider ports live in tig-store (PKG-002 and a
bounded PKG-003 subset); tig-core remains free of filesystem IO. This allocation
does not require a separate engine crate before its responsibilities grow.
The local design-5 store has a small place marker, immutable per-operation
intent/completion files, final SHA-256 objects and private staging. Runtime
records live in its explicit root, never Git. Keys are bounded ASCII tokens;
operation filenames hash the token. Records use the shared strict canonical
JSON implementation; missing/duplicate/unknown/noncanonical fields reject.

The reference adapter caps an object at 8 MiB and streams with a 64 KiB buffer.
This bounded proof is not a configured production media provider. It reserves
no physical capacity and offers no deletion API. The later scoped-coordination
checkpoint adds shared read/publication guards and exclusive exclusion, not
complete retirement/claim authorization. Same-filesystem no-clobber hard linking publishes a closed/synced private
file, followed by exact destination verification. No writable alias is exported. The later ACCOUNTED-PUBLICATION checkpoint adds
an instance-pinned place and attributed private attempt scopes.
Synchronous failures clean only their own new private partial; abrupt death can
leave partials. Those are recovery/accounting debt and cannot be swept or allowed
to grow without admission in a live engine.

Retries compare owner, object and requested acknowledgement level. Verified
existing objects can satisfy retry without rereading producer bytes. Completion
is checked before recreating missing intent: surviving completion cannot be
rebound by another owner after partial journal restore. Malformed state blocks,
never empty protection. A stale restore losing entire operations cannot establish
complete protection coverage from this provider alone. Full restore/redundancy/
epoch policy remains open.

L0: repository/VTRACE/fmt/whitespace. L1/L2: 24 publication test entries (including
one subprocess helper), using actual isolated temporary files, ten injected
before/after faults, four abrupt process exits, stream errors, metadata corruption,
replay and concurrent trusted threads. The abrupt partial-write case proves debris
is neither complete nor swept by exact recovery. Unix symlink rejection runs on
Linux; Windows tests use type guards, not adversarial reparse-point fencing.
No actual full disk, power loss, host reboot, hostile filesystem writer or
provider-durable backend was tested.

Commands: `cargo test -p tig-store --locked` (24 entries),
`cargo test --workspace --locked` (154 total), fmt and all-target Clippy.
EVID-010 records this scope; WP-003 remains in progress, not fully verified.
