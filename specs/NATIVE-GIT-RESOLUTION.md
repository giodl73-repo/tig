# Native asset IDs from exact Git views

Entry 6da872a7fa7e807ca5a5b079c2105d836d3745fa. WP-005/010 and WP-011
consumer-resolution prerequisite; REQ-001/006/008/009/010, SPEC-007/010/011,
ARCH-004/005, IF-008/010/013, DES-007/009, PKG-004/006/008. GIT/INTEGRATION
profiles: L0 structural/fmt/Clippy/VTRACE, L1 malformed/bounded resolution,
L2 real temporary Git commits/index/branches and an explicit CLI consumer.
Scope: optional tig-git native source, generic tig-assets resolver port and tests.
No REEL renderer, cache operation, hook/config installation or private test data.

REEL should consume AssetResolver, not infer cache paths, newest files or Git
branch state. A resolved asset exposes exact record/checkpoint, selected object
and named stream object pins. It verifies metadata identity/binding, not payload
availability, freshness, custody, human approval or full schema/producer semantics.

The Git adapter captures explicit lower-case .tig record and supporting .json
definition scopes from one immutable listing. Blob reads use captured Git IDs,
the existing bounded/no-network plumbing and raw Git-object hash verification.
Never read mutable working files, execute filters/hooks, follow symlinks or hydrate
LFS media. Missing, unsupported, conflicted, malformed, duplicate-ID or incomplete
scoped metadata refuses the namespace. No timestamp/latest/path fallback.

Capture is explicitly invoked and bounded by document count, per-file 64 KiB,
aggregate metadata bytes and wall time, in bounded batches. The in-memory catalog
supports repeated ID resolution and graph callbacks without subsequent Git IO.
This first capture is not sparse incremental indexing or native 500k scale proof.
Frozen old metadata can be supplied from explicitly captured retained snapshots;
their bytes never replace the current snapshot's ID selections. No retention/
restore durability follows merely from an in-memory metadata capture.

Exit: real Git fixtures show day-two ID selection with unchanged parent, old
commit/index restoration, ambiguity/conflict/budget failures and a generic consumer
resolving metadata pins. Full opaque-native REEL extraction, producer currency,
global source/job protection, custody and reversible production adoption stay OPEN.

## Consumer contract

REEL can receive &dyn tig_assets::resolve::AssetResolver. resolve_id(id) returns
an immutable metadata view: record/checkpoint, exact definition pin, authored
fields, selected object and ordered stream bindings (slot/role/group/object).
Authored values stay accessible in-process; Debug and the CLI omit them. Stream
order is preserved rather than alphabetically sorted. Consumers must admit the
expected definition before interpreting fields; payload verification/opening is
still the byte-provider boundary. A selected object may itself be a manifest.

Consumer-status extension entry: 12757c21e9f896946b21a6a0dca653bf61177e5b,
with the same WP/REQ/SPEC/IF/PKG controls and L0/L1/L2 profiles above. Allow only
an immutable status-map accessor and its snapshot/redaction fixtures. Status
vocabulary and eligibility remain consumer-owned; reading a value is not an
approval, selected-output freshness, retained-custody or removal acknowledgement.
Debug and CLI output continue omitting authored/status values. Exit requires old
and new Git views to retain their own status despite identical payload identity.

Example (inside a caller returning Result):

```rust
use tig_assets::resolve::AssetResolver;
let cel = assets.resolve_id("scene/04/cel/b")?;
let image = cel.stream("image")?.clone();
// Check cel.definition(), then give image to the existing verified byte provider.
```

The Git implementation is opt-in with tig-git feature assets. Low-level Git/LFS
builds remain usable without that dependency. CI tests/lints all features so the
optional adapter cannot silently skip tests. The explicit metadata-only CLI is:

```text
tig-resolve LOCAL_PROFILE ASSET_ID [STREAM_SLOT]
```

The local profile is machine configuration, not a second authored asset inventory:

```json
{
  "schema": "tig.asset-resolver-profile.experimental-1",
  "git_executable": "/absolute/path/to/git",
  "repository": "/absolute/project",
  "commit": "FULL_GIT_COMMIT_ID",
  "records": ["episodes", "videos"],
  "definitions": ["definitions"]
}
```

Limits default to 16,384 documents / 16 MiB / 30 seconds; hard maxima are 65,536
/ 128 MiB / 300 seconds. The separate Repository listing/command limits also
apply. Each file is at most 64 KiB; batches contain at most 256 blobs. The clock
is checked between batches/records; existing per-command deadlines bound active
Git calls. Raw byte accounting is not peak RAM/physical IO/capacity admission.
Initial capture must contain the used definitions/schema bytes; retain() supplies
historical instance bodies after a valid capture, preserves current selections,
and refuses aggregate growth atomically. Retain is not durable storage.

Execution: 13 real-Git/CLI tests include a 1,000-record capture followed by ID
lookup with the Git directory detached, a non-media ordered-stream consumer,
frozen references, index capture, staged conflicts, symlink modes, duplicate IDs,
metadata budgets and redaction. No native 500k/REEL production claim follows.

## Native claim metadata

Resolved.metadata() exposes private immutable metadata from the same exact record
as authored/status/streams; Debug and CLI omit its values. The optional native
output assessment adapter reads only an explicitly admitted bounded claim location,
then replays original metadata and compares the desired recipe. See
[NATIVE-OUTPUT-ASSESSMENT.md](NATIVE-OUTPUT-ASSESSMENT.md). No field name, approval,
producer execution, payload integrity or storage authority is inferred.
