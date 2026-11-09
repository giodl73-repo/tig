# Trusted GitHub placement requests

Input 1c253ea0905310269d7d1f712e0b90ab001f9a58.
WP-004/005/007/008/009/013/015; REQ-001/002/003/004/005/006/007/008/009;
SPEC-002/003/004/005/006/007/008/009/013;
IF-002/003/004/005/006/008/009/012/013; PKG-002/003/004/007/008/009.
GIT/PROVIDER profiles. Allowed: optional GitHub authentication/source adapter,
explicit foreground request receiver, bounded coordinator over the existing
committed-ref/placement APIs, inert opt-in Action example and synthetic fixtures.
No live enrollment, service installation, private credentials, cache operations
on resume, new storage journal, final deletion or metadata expiry.
Entry: exact committed-ref and local placement contracts, immutable private
authority and provider fences. Exit: actual signed-request intake -> independently
current source -> existing selection/placement -> bounded truthful progress,
including duplicate/stale/outage and missed-event catch-up. Implementation and
verification/validation/trace/review close separately; authentication alone does
not close this exit or priority 6.

## Trust boundaries

Workflow code runs outside the storage executor's OS authority. It supplies only
a signed OIDC bearer and an exact commit request; no roots, executable paths,
credentials, policy, placement destinations or removal flags. Private operator
enrollment pins GitHub owner/repository immutable IDs and exact names, audience,
subject, allowed branch/event/workflow and optional immutable reusable workflow,
plus the exact Tig lifecycle and placement enrollment identities. All execution
permissions remain independently admitted by existing private profiles. Policy
updates need trusted adoption; a merged workflow cannot expand them.

The verifier fixes GitHub's issuer and RS256, selects only an admitted bounded
public key by kid, verifies the original JWS signing input, then validates exact
typed claims and clock bounds. Reject duplicate JSON keys, noncanonical base64url,
algorithm/key confusion, token-supplied key URLs, malformed identifiers, wrong
repository/owner/ref/workflow/event/runner, future/expired/overlong lifetimes and
request/claim commit mismatch. Unknown issuer claims may be ignored only within
the bounded authenticated payload; unknown JOSE mechanisms refuse. Public keys
are independently admitted snapshots with explicit validity, never event data.
Key refresh is a separately bounded fixed-origin HTTPS operation; unknown/expired
keys hold rather than accepting stale signatures. No bearer appears in Debug,
reports, logs, command lines or persistent receipts.

Authentication establishes request provenance, not current GitHub head, retained
custody or storage authority. Independently check immutable repository identity
and current enrolled ref through an authenticated source adapter, then obtain an
exact bounded metadata capture or explicitly admitted private Git-mirror refresh.
Unbounded incoming Git packs are not admitted by the existing metadata budget.
Match the exact commit. Never execute
checkout code/hooks/filters, event-supplied URLs or a repository-built executor.
The accepted local source still uses existing pre/post-ref and journal fences.
Older/delayed requests cannot replace a newer source or claim global convergence.

The committed-source seam is independent of storage and optional typed assets.
The local Git adapter preserves its existing listing/root/context identities.
Remote adapters must qualify complete explicit pointer scopes, hash/size/path and
source identity, cumulative request/response/tree/blob budgets and current-ref
checks. Missing/truncated coverage never becomes an empty successful selection.
A remote source contract may have a distinct listing domain and requires private
adoption; a signed event alone cannot fabricate flat payload closure.

## Bounded execution and operational interface

The explicit receiver is foreground and default disabled; it is never started by
opening a repository. A loopback receiver requires an independently configured
HTTPS/auth transport boundary for external traffic. Untrusted PR jobs get no
storage profile, OS account or provider access. Request/header/body/read-time and
connection limits are fixed before intake; malformed/unauthenticated work cannot
open source/provider roots. Static errors omit paths and tokens.

A private coordinator schedules existing bounded pages with total object/byte/
page/time bounds. Current ref and desired basis are checked between pages. A
basis change restarts an explicitly bounded pass while reusing original effects;
held/unknown work remains pending. Full convergence requires a complete same-basis
pass and a final source check, never the last successful page alone. Token expiry
stops intake/new pages; a previously admitted bounded page still uses the existing
private operation grant and current provider/namespace fences. This is not an
implementation of authority/epoch migration or instantaneous grant revocation.

Retries, missed events and explicit catch-up select the current authenticated
enrolled ref. Action completion reports request acceptance separately from storage
convergence. Accepted request provenance can be retained as bounded digests and
claim identifiers outside Git; it cannot become another expanding event journal.
No request completion grants final archival deletion or tombstone retirement.

## Operator and Action contract

Build `tig-git` with optional feature `github`. `tig-trigger describe` is static;
`tig-trigger serve PROFILE` explicitly starts one foreground loopback listener.
The strict private profile schema is `tig.github-receiver.design-1`: trust,
enrollment, namespace (root/minimum), metadata_root, metadata_minimum,
placement_profile, listen, budget, max_connections, allow_receive and
allow_selection. Receive/selection default false. The placement profile reuses
`tig.placement-profile.design-1`, including its four independent default-false
copy/eviction/reacquisition/cancellation permissions. Root paths are absolute;
all provider/ledger roots must already exist. Missing roots are not initialized.
API credentials use either api_token_file or an explicitly named api_token_env,
never both. Neither mechanism accepts credentials from incoming requests.

Trust pins issuer-independent repository/owner IDs, exact repository/ref,
audience/subject/workflow/event and the two enrollment digests. The optional
reusable workflow pin includes an immutable commit. The source uses GitHub's
fixed HTTPS API, verifies immutable repository identity and current head, then
reads nonrecursive commit/tree and pointer-sized blob metadata within explicit
scopes. GitHub's TLS/API mapping of commit to tree is admitted source authority;
this adapter does not reconstruct raw Git commits/trees to independently prove
that mapping. Selected blob Git hashes are verified locally. Truncated trees,
unsupported selected entries and oversized controls hold complete selection;
excluded payload blobs are never opened. No Git fetch, checkout or hydration runs.

POST `/v1/reconcile` requires HTTP/1.1, Bearer authorization, application/json and
one canonical Content-Length. The strict body contains schema
`tig.github-request.design-1`, commit and optional existing placement cursor.
Limits: 24 KiB headers, 2 KiB body, 16 KiB token, five seconds total intake,
1 MiB response and 1..4096 serial connections per invocation. The receiver emits
one bounded ready line; HTTP responses separate authenticated, ok and result.
Malformed/unauthenticated requests never dispatch source/provider work. A stale
request reports stale; acceptance is not a global convergence acknowledgement.
The first receiver has no automatic restart or unbounded background scheduling.

Coordinator limits are 1..16384 pages, at most 500000 objects, 1 TiB streamed
copy bytes and 600000 ms cooperative run time. Source enrollment separately
bounds each capture's requests/tree entries, aggregate bytes and elapsed time;
current-head HTTP observations have a 30-second deadline. A bounded in-flight
provider/source call can finish after a coordinator deadline; there is no hard
preemption or aggregate verification-IO claim. Holds retain partial page effects
and the original desired-selection acknowledgement. Resumed suffix success keeps
global convergence unknown; a fresh complete same-basis pass can prove it.

The inert [Action example](../examples/github/placement.yml) and
[request client](../examples/github/request.mjs) install nothing. Independently
enroll the repository/ref/workflow, configure an HTTPS ingress to the loopback
receiver and isolate its OS/provider authority from workflow execution before
adoption. The client requests a runtime OIDC token for the enrolled audience,
sends only commit/cursor-free initial request and redacts token/network errors.
Pending convergence exits nonzero; workflow_dispatch can request explicit catch-up
when that event is admitted. A workflow run alone does not repair missed events
without another explicitly scheduled or requested invocation.

The dedicated hosted qualification invokes the actual foreground binary with a
fresh real issuer token and temporary stores under a random absent metadata scope.
It qualifies issuer/intake/API compatibility with zero payload actions only when
explicitly executed successfully. The ordinary ignored test entry is not proof.
Nonempty signed HTTP/store behavior uses separate synthetic RSA/source fixtures.
External TLS ingress, rate admission and OS separation remain adoption gates.

## Evidence and remaining gates

L0: fmt, portable validator/pins, diff and six producer lenses. L1: locked default/
all-feature workspace, strict Clippy, standalone Git without network/typed features,
and adversarial signed-token/source/coordinator vectors. L2: independently generated
synthetic RSA signatures, actual foreground HTTP/CLI -> temporary Git/stores,
source drift, replay, expiry/key rotation, bounded paging, lost replies and process
restart; unrelated work must progress without global cache locks. Real-account,
TLS ingress/OS isolation, native-provider/large-file/lifetime and user adoption
remain separate qualification. A verifier or fake claim alone is not that proof.

Primary references, checked 2026-11-08:
[GitHub discovery](https://token.actions.githubusercontent.com/.well-known/openid-configuration)
lists issuer/JWKS, signing algorithm and claims including sha and immutable IDs.
[OIDC reference](https://docs.github.com/en/actions/reference/security/oidc)
describes audience/subject/workflow claims; pin the admitted subject rather than
assuming one default subject format. [Runner guidance](https://docs.github.com/en/actions/reference/security/secure-use)
explains why workflow code and the executor need separate authority.
[ring RSA verification](https://docs.rs/ring/0.17.14/ring/signature/struct.RsaPublicKeyComponents.html)
provides the maintained signature primitive; Tig does not implement RSA.
