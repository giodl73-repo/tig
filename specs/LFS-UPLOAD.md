# Explicit Git LFS upload

Input 9b8a6890e0074a58851a8d313dcb9b841fd7d613.
WP-003/005/006/007/014/015; REQ-001/003/004/005/006/007/008/009;
SPEC-002/003/004/005/007/008/009/013; IF-002/003/004/005/006/008/009/012/013;
PKG-002/003/004/007/008. GIT/PROVIDER profiles. Scope: optional standalone upload
agent, Basic HTTP Batch/PUT/verify/download readback, existing guarded Git LFS cache
reads and accounted metadata witnesses; synthetic temporary repositories only.
Entry: exact OID authorization tree, explicit source/witness/ledger/job enrollment
and trusted remote scope. Exit: actual Git LFS push and guarded exact-byte upload,
readback, retry/lost reply, denial, bounded resource use and truthful unsupported
retention. No cache initialization, hook/config installation, live consumer or
cloud/last-copy deletion. Verification/validation/trace/review close separately.

The opt-in tig-git upload feature adds `tig-lfs --upload-profile PROFILE`; the
existing --profile download protocol/records remain compatible and network-free
without the new feature. Standard standalone upload frames must carry action:null.
Init remote must match private enrollment; uploaded paths must exactly match the
enrolled standard Git LFS cache's OID layout. Arbitrary action URLs or input files
cannot grant access. The immutable authorization tree independently admits exact
OID+size; the existing ReadCache verifies and holds source bytes through transfer.
No payload copy or disposable alias is created; upload completion has no path.

The adapter itself calls a fixed admitted Batch endpoint for one exact object and
ref. It supports basic/SHA-256 only, conditional PUT and optional verify POST,
followed by a fresh download Batch/GET and full hash/length verification. A prior
download probe may reuse exact existing remote bytes. Only an object-level 404
means missing; repository 404, denial, unsupported transfer/hash, wrong object,
corruption or unknown actions hold. Known acknowledged loss never silently
re-uploads. A fresh explicit operation scope permits a separately intended ingest.
Readable remote bytes do not skip upload Batch admission and any requested verify
notification for an unfinished operation. Otherwise a retry after failed verify
could falsely complete against a readable but unregistered content-store object.

Private profiles pin repository/operation scope, authorization snapshot, source
identity/generation, owner/context, witness/capacity authority/job, remote instance/
generation/Batch location and allowed action origin/path scopes, plus finite
object/session/HTTP/body/time budgets. Upload defaults disabled. Credentials and
dynamic actions remain outside plans/witnesses. Returned action scopes cannot
broaden private permissions; headers cannot override routing/framing. HTTPS is
required except explicitly admitted literal-loopback fixtures. No ambient proxy,
credential helper, redirect, recursive discovery or automatic retry is installed.
Batch credentials apply only to the fixed Batch endpoint. Each action scope may
also enroll private `credentials` headers for servers that expect credential
lookup (`authenticated` absent/false); there is no implicit Batch-token forwarding.
These headers must be permitted by that scope's `headers` list. Exact origin and
decoded path prefix select exactly one scope. Ambiguous matches, case-insensitive
duplicate names and conflicting returned/private header values refuse. Credentials
are excluded from target/plan identity, so refresh does not alter an operation.
Header names that control routing/framing, content encoding or the immutable PUT
condition cannot be supplied. Content-Type/Accept supplied for a fixed protocol
request must agree with its expected media type. Action expiry follows bounded
`expires_in` preferentially, otherwise uppercase RFC3339 second-precision
`expires_at`; invalid dates, unsupported leap seconds and pre-epoch dates refuse.
Elapsed operation time conservatively bounds relative expiry. Remote denial still
holds when clocks differ. The CLI installs no logger; library hosts must disable
HTTP transport trace logging, which can otherwise reveal action URLs/credentials.
The first admitted Basic provider contract requires conditional immutable CAS
publication; incompatible servers hold rather than fabricating no-clobber/custody.

Small prior intent and later completion reuse shared upload_witness composition
over existing accounted local publication. S3 keeps its exact existing schemas,
bytes, keys and bindings when adopting that helper. Completion-attempt guards
span remote IO; unrelated work proceeds without a cache-wide lock or SQL
transaction across IO. Missing settled evidence holds, not repair. Read-only
status adds no commands and needs no source payload. Receipts deny retained
custody, provider durability and removal; remote quota remains unqualified.
The receipt's `payload_io_bytes` reports application request/response bodies,
including Batch/verify JSON and full readbacks as well as upload bytes; it is
neither network-interface traffic nor provider billing/quota accounting.

The first upload cap is 8 MiB, not qualification for native large media. Existing
download bounds are unchanged. Linux protection is explicitly cooperative; the
adapter does not claim to coordinate arbitrary external cache pruners. Local
witness growth, original retry receipts and lifetime/epoch continuation retain
their existing gates. LFS upload does not make a Git merge a backup acknowledgement.

## Operator and agent interface

With the optional `upload` feature, `tig-lfs upload describe` reports the versioned
interface and default-disabled capability. Commands are:

```text
tig-lfs upload plan PROFILE OBJECT
tig-lfs upload peak PROFILE PLAN
tig-lfs upload status PROFILE PLAN
tig-lfs upload apply PROFILE PLAN
tig-lfs upload resume PROFILE PLAN
tig-lfs --upload-profile PROFILE
```

PROFILE/OBJECT/PLAN arguments are absolute paths to bounded regular JSON files;
no command creates a store, capacity authority, job or cache. OBJECT is the
existing `{sha256,bytes}` object document. `plan` emits a canonical plan document
which can be saved directly as PLAN; plan parsing requires exact canonical bytes,
without a trailing newline. Successful CLI output is canonical JSON; failures
return a static versioned code and nonzero status without remote response text.
`peak` reports local witness admission only, never reserved remote quota. `status`
performs fresh bounded readback and separates historical completion/accounting
from present availability; it creates no journal commands or source payload IO.
Apply/resume require `allow_upload:true`; discovery, planning and status grant no
write permission. Git LFS standalone configuration must be explicitly installed
by its operator with the upload direction and correct private profile; the command
does not configure Git or install hooks.

The strict `tig.lfs-upload.design-1` profile groups are:

| Group | Required enrollment |
| --- | --- |
| Source | `source_root` (existing Git LFS root), `source_identity`, positive `source_generation`, `source_protection` |
| Authorization | Exact `authority_root`, `authority_nodes`, optional `authority_checkpoint`, finite `max_authority_entries` |
| Local witness | Existing `witness_root`, exact `witness` place, `ledger_root`, `minimum` checkpoint, `capacity`, reserved `job`, `owner` |
| Operation | `context`, `operation_scope`, exact init `expected_remote`, full `ref_name` |
| Remote | `remote.instance`, positive `generation`, fixed `batch_url`, private `headers`, `scopes` with origin/prefix/header names and optional private credentials |
| Budgets | `budget.object_bytes`, `total_source_bytes`, `objects`, per-object `http_requests`, `transfer_bytes`, `millis`; explicit `allow_upload` |

Private roots and secrets remain outside Git and portable plans. The loopback
HTTP exception additionally requires `remote.loopback_fixture:true`. Source and
witness identities must differ; the witness is an ordinary local publication
store, not a managed-retirement store. Actual source cache bytes are authorized
by OID/size and verified under the reader guard; a private source identity is
enrollment provenance rather than discovery of a Git LFS marker.

L0: format/validate/diff, target pins and six role lenses. L1: focused protocol/
negative tests, default/all-feature workspace, strict Clippy and standalone Git.
L2: real temporary Git LFS push to an actual HTTP fixture; lost PUT/verify/reply,
existing/shared objects, corruption/missing/denied readback, unsafe scopes/headers,
wrong OID/size/path/ref, source preservation and commandless status, concurrent
peers/unrelated work and unchanged S3 witness/retry behavior. No real credentials,
assets, remote account or deployed consumer participates.

Primary contracts: [custom transfer protocol](https://raw.githubusercontent.com/git-lfs/git-lfs/v3.4.0/docs/custom-transfers.md),
[Basic transfers](https://raw.githubusercontent.com/git-lfs/git-lfs/main/docs/api/basic-transfers.md),
[Batch API](https://raw.githubusercontent.com/git-lfs/git-lfs/main/docs/api/batch.md).
Local installed client is Git LFS 3.4.0; CI records its actual client version.
