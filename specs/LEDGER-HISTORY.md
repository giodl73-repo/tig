# Original ledger replies and dependency archive

Input9122ccff80f18dc79b8fafb3a58f52705ae78ac3 (PR58). Full PRODUCT_PLAN is
active. Parent working-state checkpoint is implemented/focused-tested; full
profiles75122/13738 and exact43100104758 remain live at this design entry.
Do not overlap their targets or treat timeout as terminal. No runtime changes
are authorized by this design checkpoint until its contract is committed.

WP-003/004/006/007/008/013/015; REQ-001/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009/013/014; IF-002/003/004/005/006/008/012/013;
PKG-001/002/003/008/009. RUST/PROVIDER/INTEGRATION, L0/L1/L2 below.

## Required result and allowed scope

Complete the next required input to [journal continuation](JOURNAL-CONTINUATION.md):
a bounded immutable original-request/result/dependency archive exported from the
existing CapacityLedger. Use its actual validated journal and original outputs;
never recompute old responses from current accounting. The existing
[working-state checkpoint](CAPACITY-CHECKPOINT.md) is necessary but insufficient.

Allowed: additive core historical token/retired-record framing; additive existing
ledger archive exporter and lazy reader over existing Tree/NodeStore; synthetic
exact-reply, dependency, clock, reopen, corruption, resource and race fixtures.
No dependencies, active journal/config/event/receipt format changes, migration,
segment activation, journal pruning, cap increase, CLI action or metadata expiry.
There is no new cache engine, daemon, discovery or provider/payload operation.

The archive must preserve all original operation IDs, exact canonical requests,
original sequence/previous/hash/checkpoint and response values. Preserve domain,
authority epoch/genesis, original config/clock policy and clock high-water. Each
output keeps its exact kind and dependency data: reservation token; generic,
publication, retention and handoff allocation tickets; bound digests; cleanup
review; settled footprint/sequence values; original observation summary; finished,
invalidated and retired results. Retired records retain their exact closed token,
events/limits and original completed outcomes for historical allocation queries.
Do not replace typed dependencies with response labels or summary totals.

Historical lookup returns an explicitly replayed original Receipt and exact
request bytes, never a new command/current-head checkpoint. Conflicting reuse of
an operation ID with different canonical request bytes refuses. Wrong genesis,
result kind, binding, owner/generation or allocation limit refuses. An absent,
corrupt or budget-exhausted lookup is unknown/error, never an invented success,
new reservation or permission to rerun a destructive command.

## Probe and authority boundary

Historical probes remain historical. Do not add a public decoder that constructs
a live ObservationTicket from caller-controlled nonce/watermark fields: observe
currently compares private ticket identity to pending state, and an invented
watermark could erase racing charges. Archive probe fields for exact historical
evidence but expose no conversion to a live observation ticket. Historical
ProbeStarted replies remain replayable; future activated ledgers must reject new
Observe dependencies on archived probes and require newly issued handle-local
probes. Do not serialize Instant or restore the old handle's probes/fresh maps.

Reservation/allocation token decoding must validate the same bounded token,
nonzero generation and positive limit rules. Such tokens are pure matching data,
not caller authorization or custody. Retired data remains read-only. Decoding a
new self-consistent archive root is a different admitted input, not proof of
current authority or historical provenance. Current protection/deletion authority
is not derived from an archived response, timestamp or accounting sample.

## Bounded immutable framing

Reuse strict canonical JSON and existing content-addressed Tree/NodeStore. Freeze
new schema names and exact leaf paths before implementation; keep all old codec
bytes unchanged. Small descriptor binds original config/source Checkpoint,
working-state root, original-result index root, counts and clock high-water.
Each operation leaf binds its exact request/event chain fields and typed original
output. Large retired event maps are separate immutable indexed records, never
embedded whole in an operation leaf or repeated in every dependent result.
Operation paths bind the exact operation ID through its SHA256 UTF8 digest; a
leaf also carries and checks the original ID. Retired event paths similarly bind
exact event identity. No self-hash field or new private operation-key recipe.

A segment export is bounded by the existing16384-command/16MiB journal limits;
these limits stay unchanged. All metadata records remain at most64KiB. One shared
meter covers working state, operation leaves/index, retired subgraphs and all
metadata reads/writes: at most262144 calls and64MiB encoded bytes per export or
complete bounded validation. These are new archive-operation limits, not raised
journal/provider/Tree record limits. Lowerable budgets never restart per phase.

Lazy lookup must use existing Tree.get against the exact admitted root, without
entries() over the archive or constructing all outputs in memory. Freeze a
separate lowerable point-query budget at256 calls/1MiB for one ordinary original
receipt; retired allocation inspection may use512 calls/4MiB. Budget checks occur
before provider calls. Historical lookup returns only the requested bounded data.
Future activation must compose persistent indexes without replaying/materializing
all lifetime history; a linked list that searches every segment is insufficient.

The source is already bounded and validated by the existing engine. Export may
read its current segment rows and outputs, checking exact sequence/hash/count/
request/output associations. No SQL transaction spans NodeStore IO. Revalidate
exact config/genesis/head before and after export; peer advance refuses a falsely
current descriptor. Partial immutable metadata may remain orphaned but installs
no root. The caller admits staging/capacity and must finish and read back its
metadata port before relying on durability. Export issues no source command,
probe or accounting reset. A returned descriptor is a historical observation.

## Entry and exit evidence

Entry: required startup/router/wave/pulses, MODEL/SAFETY/RESERVATION-JOURNAL,
LOCAL-CLOCK, CAPACITY-CHECKPOINT/JOURNAL-CONTINUATION, actual ledger Request/
Output/execute/catch_up/receipt code, core ticket privacy and Tree/metadata ports,
pitfalls and all six producer lenses. Freeze exact new wire fields/paths in an
amendment before implementation, including historical probe non-rehydration.

L0: exact input/contract/source pins, links/bounds, fmt/diff and secret scan.
L1: independent literal/hash vectors; every output kind and exact request/result;
original zero/completed/in-flight/retired dependencies, cleanup cancellation and
settled binding mismatch; original response after later accounting changes;
clock policy/high preserved but no fresh authority; re-pinned malformed/path/
kind/token/count/hash/unknown cases; missing/corrupt nodes and shared/point budget
failure; maximal legal segment and128-byte tokens within frozen limits. Strict
locked focused tests and workspace/all-target Clippy.
L2: actual fresh legacy and clock-enabled SQLite journals -> explicit admitted
MetadataStore -> finish/drop/reopen reader -> lazy exact receipts/dependency
inspection. A peer commits from inside node IO and forces conflict without a
held SQL transaction. Failed export changes no source commands/accounting.
A counting NodeStore proves point lookups do not enumerate unrelated receipts.
Both workspace/500k/strict profiles, standalone Git, previous14-step teaching
artifact and exact Windows/Linux/secrets remain required. No private/live roots.

Close implementation, verification, scoped synthetic validation, trace and six
producer findings separately. Original-result archive alone does not deliver
segmented activation, recovery/old-handle/writer/restore fencing, provider/source/
witness continuation, sustainable lifetime or metadata expiry. Those remain
required next integrations; no quiescent-empty workaround or larger journal cap.
Full product/course/current-authority/time/provider/large-file/operator/UI/Action
and independent/human acceptance remain OPEN. No merge or deployment.
