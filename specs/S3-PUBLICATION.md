# Source-preserving S3 publication

Input 3fd9918c30295e9a8ca0cf943e076408bde5e3b1.
WP-003/005/006/007/014/015; REQ-001/003/004/005/006/007/009;
SPEC-002/003/004/007/008/009/013; IF-002/003/004/005/006/012/013;
PKG-002/003/004/008. RUST/PROVIDER profiles. Allowed scope: optional S3 conditional
PUT/full GET readback, existing local publication/capacity witnesses and explicit
operator/agent interface, using synthetic protocol fixtures only. Entry: exact
local guarded source and existing admitted witness store/ledger/job. Exit: actual
bounded HTTP transfer, immutable retry identity, lost-reply recovery, fresh status,
source preservation and explicit unsupported custody/removal. No cloud account,
service installation, consumer deployment or production qualification.

## Protocol and authority

The first remote backend is Amazon S3 general-purpose conditional PutObject with
independently admitted per-object presigned PUT and GET URLs. Signatures/credentials
are supplied outside Tig, remain private, and can be refreshed without changing
operation identity. No discovery, credential helper, ambient proxy, redirect,
automatic retry, bucket listing, multipart or DELETE is available in this slice.
Query validation admits only SigV4 authentication/operation annotations (and a
loopback-fixture credential). Version selectors, multipart selectors, unknown or
duplicate query keys refuse; credential refresh cannot silently change the object
representation while preserving a location identity.
Nonfixture URLs require HTTPS and normal certificate verification. Explicit
fixture HTTP is restricted to literal loopback addresses; it grants no remote HTTP.

Private admission binds target instance/generation and a digest of the exact
scheme/authority/path, excluding credential query strings. PUT/GET must name the
same location, ending with the exact object SHA-256. Portable plans never contain
URLs, credentials or roots. Object identity remains hash+length; endpoint identity
is distinct. The conditional header If-None-Match:* and expected SHA-256 checksum
are mandatory. A conflicting existing object is never overwritten. 403 is denial,
not absence; only an authenticated GET 404 is treated as absent. Redirects, content
encoding and partial responses refuse rather than changing target/representation.

The presigner must admit the PUT headers used here: If-None-Match:*,
x-amz-checksum-sha256 containing base64 of the raw SHA-256 digest, Content-Length
equal to the object bytes and Content-Type:application/octet-stream. The GET URL
must authorize full readback of that same key. Other metadata, ACL, encryption-key
headers and redirect-based endpoint discovery are not synthesized. Expired access
can be replaced in the private profile without changing target location/plan.
Tig emits neither URLs nor transport errors. The CLI installs no HTTP logger;
library hosts must keep transport trace logging disabled because ureq trace mode
can include URL queries. A redacted Client Debug implementation is not protection
against an application explicitly enabling sensitive transport tracing.

The source is a freshly verified completed local publication under its existing
held object reader. The first reference profile retains the 8 MiB object bound
and streams through bounded buffers. A fresh full GET verifies actual hash/length;
ETag, successful PUT and checksum headers alone are not readback proof. An existing
exact object can satisfy retry without another PUT. A lost PUT reply leaves intent
and source intact; explicit resume GETs before considering another conditional PUT.
412 triggers readback of the winner; 409/5xx and transport uncertainty return for
explicit retry. No automatic destructive fallback occurs.

Presigned access does not establish durable retention, failure-domain independence,
Object Lock or permission for source eviction. The receipt explicitly reports
verified remote availability and denies retained custody/removal. Provider-durable
and retained-copy requests are unsupported. Actual AWS IAM/TLS/account/provider
qualification remains separate from loopback protocol evidence.

## Reuse and bounded recovery

Small exact intent/completion documents are published through the existing local
accounted publication path into an explicitly existing witness store. This is not
a second cache/journal engine and does not cache the uploaded payload. Both metadata
allocations/bindings are admitted before remote IO. Intent is acknowledged before
PUT; completion follows verified readback. Fixed derived operation IDs bind the
whole plan/context and existing reserved job. Conflicting reuse refuses before
HTTP. Witness loss/corruption and capacity exhaustion hold instead of resetting.
An existing completion-attempt guard coordinates only that exact operation through
remote IO. Completion is acquired before intent; another job/operation can still
upload. The guard is released for the existing accounted completion publisher,
whose exact binding handles a competing finisher. No SQL transaction spans remote
IO. Status samples provider evidence before the ledger and rechecks after a fresh
settlement witness, so concurrent completion does not fabricate missing metadata.

Remote transfer budgets bound object bytes, total application payload IO bytes,
HTTP calls and elapsed time per invocation. Network timeouts are cooperative, not
an arbitrary user-reader interrupt or whole-system deadline. There are no retry
loops. Repeated invocations still consume external bandwidth/provider billing;
this per-invocation budget is not a lifetime billing or remote-quota reservation.
Only local witness allocation is charged to the existing ledger. Remote quota and
device/site/key-loss guarantees remain unqualified; never borrow expected reclaim.
The receipt's payload_io_bytes counts source bytes consumed for upload plus bytes
read from successful GET bodies, not raw NIC traffic, headers, billing, remotely
acknowledged PUT bytes or independently retained space. Error bodies are not
materialized; response headers and client buffers have separate finite bounds.

Status performs explicit fresh GET and verifies existing witnesses without adding
commands or repairing anything. Historical completion and observed availability
are separate. Missing/corrupt acknowledged remote bytes are an incident, never an
ordinary miss that silently triggers re-upload. Source can remain offline during
status. Runtime records and detailed receipts stay asset-side, never in Git.

## CLI and agent contract

Build with `cargo build -p tig-store --features s3 --bin tig-storage --locked`.
The default standalone Git/LFS build has no S3 network dependency. Root discovery
reports whether S3 was compiled. `tig-storage s3 describe` opens no provider.

    tig-storage s3 plan PROFILE REQUEST
    tig-storage s3 peak PROFILE PLAN
    tig-storage s3 status PROFILE PLAN
    tig-storage s3 apply PROFILE PLAN
    tig-storage s3 resume PROFILE PLAN

REQUEST is strict `tig.s3-upload-request.design-1` JSON with operation and the
existing exact source publication Request. It cannot supply URLs, owner, target,
budgets, policy context or an enable flag. Plan output is canonical
`tig.s3-publication-plan.design-1` (at most 16 KiB); retain exact stdout bytes.
Imported documents/profile have a 64 KiB outer bound and unknown fields refuse.

The machine-local `tig.s3-profile.design-1` has absolute source_root/witness_root/
ledger_root, independently admitted source/witness Place values, minimum capacity
Checkpoint, existing reserved job JournalRef, target, owner, context, budget,
put_url/get_url, allow_upload (default false) and loopback_fixture (default false).
Target pins instance, positive generation and location digest; construct it using
the library Client::presigned().target() from admitted access. Profile edits cannot
silently alter an imported plan: all declared admission fields must match.
The witness is an existing ordinary local store, never a managed-retirement root;
source and witness cannot alias. peak reports conservative local witness headroom,
not remote quota. No source/witness/ledger/job is initialized by these commands.

Status opens only the witness and ledger, then performs explicit GET; source can
be offline. It returns historical_completion, intent_recorded, accounting_settled,
observed (absent/verified), requires_recovery and false retained_custody/
removal_authority. Corruption and denied/unavailable access return bounded static
error codes with nonzero exit. Apply/resume preserves source and denies custody/
removal even on success. Intent/completion and metadata settlement are distinct.
Cooperative library cancellation checks before work, after intent and throughout
payload reads; it preserves partial outcomes for exact explicit retry. CLI process
termination uses the same durable intent/retry boundary, with no implicit sweep.

L0: validate/diff/fmt, target pins and all six role lenses. L1: focused negative/
protocol cases, locked default/all-feature workspace tests, strict Clippy and
standalone Git. L2: actual temporary source/witness stores and HTTP server, exact
headers/body, conditional collision, lost response, readback corruption/missing,
denied/expired access, redirect refusal, budget/timeout/cancellation boundaries,
restart/retry and source preservation. Implementation, verification, validation,
trace and review close separately. Priority 5 and full requirements remain open
until the declared remote/LFS/provider/adoption obligations are qualified.

Primary protocol sources (reviewed 2026-11-08):
[conditional writes](https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes.html),
[PutObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html),
[presigned URLs](https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-presigned-url.html).
The optional synchronous client is pinned ureq 3.5.1; its maintained TLS/HTTP stack
is reused rather than implementing TLS/signatures in Tig. See its
[configuration](https://docs.rs/ureq/3.5.1/ureq/config/struct.ConfigBuilder.html).
