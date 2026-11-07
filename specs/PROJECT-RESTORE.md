# Explicit Rust project restore profiles

The optional [handoff extension](PROJECT-HANDOFF.md) adds bounded declared
consumer-evidence pins and explicit selector/destination outcomes. Links remain
unverified and confer no runtime, custody, approval or removal authority.

Entry source `e184ef74142f2b1464122be5c0c577d069be0389`; WP-005/011/015,
REQ-001/003/006/010, SPEC-004/007/011/012/015, IF-005/008/010/013,
PKG-003/004/008, RUST + INTEGRATION profiles. Scope: shared existing guarded
restore primitive, bounded exact-Git project planning and explicit diagnostic
CLI with synthetic fixtures. No new provider/cache engine, daemon, lifecycle
claim, quota, render or automatic runtime/source adoption.

```text
tig-project [--apply] [--expect-plan SHA256] LOCAL_PROFILE MANIFEST_PATH
```

The explicit local profile supplies Git executable/repository/exact commit,
existing byte-source configuration, absolute owned destination root, native
metadata limits and hard request/destination-byte budgets. The manifest is a
bounded ordinary file read from that exact committed snapshot, never a dirty
working file. It declares nested record/definition scopes, allowed relative
destination prefixes and ordered entries with an ID, explicit selected/stream
selector and relative destination. An alternative compact catalog selection
uses all selected records in the declared Git namespace and an explicit authored
destination field; it does not expand a thousand-entry list into the manifest.
No broad directory/provider discovery or implicit dependency-closure traversal.

Capture the Git listing once, read that manifest through the same snapshot,
capture/bind the native namespace once and resolve all entries before any
destination directories or copies. Enforce 1..1024 entries, a caller budget no
larger than 8 GiB, scoped portable paths, case-conservative destination alias
and file/parent collisions, and exact object lengths. Record aliases, unique
objects/bytes and destination bytes separately. View and render manifests are
independent selections; undeclared host/render dependencies are not discovered.

Preview performs complete bounded source verification and existing-destination
checks without creating a run root. Apply performs that same preflight first,
then creates explicit owned parents and calls the same restore primitive used
by `tig-restore`. Reopen/reverify each source at copy time and hold its effective
guard throughout copy. Preserve synced private sibling, independent readback,
atomic no-replace publication and verified identical-target retry. No global lock.

The plan identity binds source, manifest, captured metadata, resolved entries and
the exact local profile. Optional `--expect-plan` refuses changed context before
verification/copy. Machine relocation can change a plan without changing object
identity. A deliberately new invocation can select a new source; this is not an
implicit replay of an earlier receipt. Frozen source need not equal checkout HEAD.

Local results distinguish entry ordinals, requested/resolved identities,
preflight/operation phase and verified actions, with source/profile/manifest/plan
pins and explicit unknown custody/held-protection state. A failed invocation may
retain earlier successful files; explicit retry verifies them. No multi-file
atomicity, durable interruption journal, automatic orphan cleanup or deletion
authority is promised. Detailed results are local operational evidence and belong
asset-side; bounded stdout alone does not enroll or retain them.

L0 fmt/structure/Clippy; L1 shared primitive plus malformed/scoped/collision/budget
fixtures; L2 synthetic real-Git nested view/render and non-media consumers,
fresh roots, no-write preview, retries, stale-plan refusal, working-file drift,
missing/corrupt source and symlink/parent conflicts. Existing single restore and
standalone Git/LFS regressions remain mandatory. Full operator recovery, host
dependency reporting, actual consumers and custody stay separate/open.

## Profile and manifest example

The ignored local profile schema is `tig.project-restore-profile.experimental-1`.
It requires `git_executable`, `repository`, an exact `commit`, `destination_root`,
the existing `byte_source` profile and `limits` with `entries` and
`destination_bytes`. Optional `metadata_limits` uses the native resolver's
document/byte/time limits. Native documents and manifests retain their ordinary
64 KiB strict JSON bounds. The caller must budget source reads for both unique
preflight objects and each applied destination; insufficient reader limits refuse
before creating any destination. This is logical workload admission, not quota
or a filesystem free-space guarantee.

```json
{
  "schema": "tig.project-restore.experimental-2",
  "records": ["assets/view", "assets/render"],
  "definitions": ["types/shared", "types/scene"],
  "destination_prefixes": ["inputs"],
  "selection": {
    "kind": "entries",
    "entries": [
      { "id": "picture", "selector": { "kind": "selected" }, "destination": "inputs/picture.bin" },
      { "id": "cue", "selector": { "kind": "stream", "slot": "selected" }, "destination": "inputs/cue.bin" }
    ]
  }
}
```

A literal slot named `selected` remains a stream. It does not select the root
Object. A large scoped catalog instead uses:

```json
"selection": {
  "kind": "catalog",
  "selector": { "kind": "selected" },
  "destination_field": "restore_path"
}
```

Each selected record must explicitly author that named string property. Missing
paths/selections refuse; there is no inferred ID-to-filename mapping or profile
label filtering. Catalog ordering is stable by asset ID. A streamed, length-framed
entry digest keeps plan identity bounded without serializing a large aggregate
as one native identity document. No entry list is written back into Git.

The optional library progress observer can persist consumer-owned results or
request a stop between entries. It is trusted Rust code, never an executable
hook loaded from a manifest. The CLI does not install a persistent progress
journal. A killed process can leave verified completed files or private orphan
partials; only explicit retry is supported, with no automatic resume or cleanup.
Detailed reports include private record identifiers and belong in local asset
storage, not public logs or ordinary Git.

The opt-in [progress evidence extension](PROJECT-PROGRESS.md) now defines a
bounded synced per-entry local log and explicit inspection. It does not turn
restore into an atomic transaction or automatic resume/cleanup service.

The optional [host/space extension](PROJECT-PREFLIGHT.md) admits declared external
file requirements and advisory capacity observations before source verification
and copying. It does not discover tools or grant render capacity admission.
