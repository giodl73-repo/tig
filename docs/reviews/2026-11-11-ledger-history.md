# Original-result archive design review

EVID-128 candidate; input9122ccff80f18dc79b8fafb3a58f52705ae78ac3.
[LEDGER-HISTORY](../../specs/LEDGER-HISTORY.md) names controls/scope and L0/L1/L2.
Method: the producer inspected actual ledger requests/outputs/execute/catch_up,
core private tokens, immutable Tree point lookup and parent checkpoint behavior.
All findings belong to Tig producer. These six lenses are not independent people,
publication authority or human acceptance. Entry was design only; implementation and scoped evidence follow.

| Lens / target | Finding / severity | Required disposition / evidence |
| --- | --- | --- |
| Identity / original event | High: rehashing only a summary loses exact request and original checkpoint | Canonical request, original sequence/previous/hash and typed result bound to exact ID/index path; literal and re-pinned mismatch vectors |
| Placement / admission | Critical: public historical probe decoder could forge a watermark and discard racing charges | Historical probe data has no conversion to live ObservationTicket; original replay only, new observation requires a newly issued handle-local probe |
| Custody / dependencies | High: response labels alone lose publication/retention/handoff/cleanup and retired allocation data | Preserve every typed original output and binding; exact old dependency inspection and cancelled/conflicting settlement cases |
| Concurrency / source export | High: archival node IO must not block peers or return a raced source as current | No SQL transaction spans metadata IO; compare source config/genesis/head, injected peer progress forces conflict |
| Provider/privacy / bounds | High: nested retired graphs or whole-lifetime output maps bypass limits | Existing Tree/NodeStore, bounded retired records, one shared export meter and separately bounded lazy point query; actual finish/reopen and counting-port evidence |
| Git/adoption / lifetime | High: archive could be mistaken for activated compaction or expiry | New additive schemas only; legacy bytes/caps unchanged. Explicit durable activation, original dependency lookup, old-writer/restore fences and expiry remain required |

Design-entry disposition: pass_design_with_risk for the stated archive scope. Exact wire fields
and API mapping must be frozen before code. Parent PR58 full local/hosted checks
remain live at entry. All implementation/verification/validation/trace and full
continuation gates remain OPEN. No root migration, metadata expiry or payload IO.

## Core primitives: intermediate implementation

Contractc4280c6 and exact wire7ac2c21 precede code. Additive capacity::history
records preserve reservation/allocation matching identity. ProbeEvidence captures
historical fields, with no live-ticket conversion. Positive documentation and a
compile-fail conversion example check that boundary. Public pure matching data
still confers no caller authorization, current observation or provider custody.

RetiredArchive stores one header and separate completed event leaves through the
existing Tree/NodeStore. Full read restores only the existing read-only history
type, validating canonical framing/path/count/tokens, completed limits/charges,
unique original sequences and aggregate charge<=peak. Point inspection binds the
exact caller ticket and admitted root, returns original charge/sequence and does
not claim complete coverage. The existing checkpoint meter is reused through
capacity-module-private visibility; its algorithm and old bytes are unchanged.
One budget spans leaf and Tree phases. Partial export installs no active root.

Core22592: initial five history cases pass. Core28528: all16 unit cases (nine
prior checkpoint/seven new history), positive and compile-fail probe docs, and
strict core/all-target Clippy TERMINAL0. A literal retired header/digest is
independently pinned. Thirteen re-pinned malformed groups, altered provider bytes,
missing nodes, source-preserving budget failures, same-name new generation and
zero/original sequence cases pass. Maximum4096-event/128-byte token fixture stays
within frozen record/IO limits; point inspection asserts fewer than100 calls and
less than1/100 of the full read's calls. This does not establish provider durability.

The Identity lens has core literal/framing evidence; Placement has the critical
probe non-rehydration evidence; Custody has retired exact-token/outcome evidence;
Provider/privacy has bounded pure NodeStore evidence. Concurrency and complete
ledger dependency/adoption findings are still OPEN. Historical output export,
lazy original-request/reply lookup, actual metadata finish/reopen/peer races and
all ledger output-kind/binding tests remain required. No implementation closure
for the full archive or journal continuation is claimed.

Broader regression35189 is TERMINAL0: all core tests,36 actual-ledger cases
and strict schema workspace/all-target Clippy pass. Both targets are free. Full candidate profiles/source/hosted gates remain required
after original-output integration. Parent PR58 runtime9122ccf now passes both
full profiles/500k/strict/standalone/fresh14-step artifact and exact43100104758
Windows/Linux/secrets; ready for review, no merge. Parent artifact is
capacity-checkpoint-qualified-5f5d8bef786b44cb97f466d49698bd8f. Full PRODUCT_PLAN
remains active; no sustainable-lifetime/expiry/independent/human acceptance.

## Original-output exporter and reader

The existing ledger now explicitly exports exact config/source checkpoint,
working-state root, operation index/count/encoded bytes and historical clock high.
It preserves actual original outputs; it does not derive old replies from current
accounting. Every output kind has strict new framing; no old command, event,
marker, receipt, dependency or capacity limit changes. The SQL row cursor is gone
before all immutable node IO. Before/after source sync rejects peer progress;
partial nodes are orphaned and no root is installed. Caller staging admission,
metadata finish/readback and independently admitted descriptor remain required.

Lazy inspection uses Tree.get, verifies the selected original request/hash/kind
and bounded typed dependency closure, returns replayed original reference/response/
checkpoint and exact request bytes, and refuses conflicting request bytes. It
checks original reservation generation/owner, ticket event/limit, publication/
retention/handoff binding, cleanup review and ordered dependency kinds. Cycles
and budgets refuse rather than expanding into whole-history replay. Probe output
is evidence only. Retired outcomes preserve original charge and sequence.

Explicit full validation checks all bounded segment paths/counts/chain, original
responses/dependencies, retired graphs, clock high and working-state root through
the existing pure transitions. It writes deterministic proof metadata to private
memory through a fork of the same shared meter; the supplied metadata port stays
read-only. Full validation is separate from point inspection and is not the
future activated-journal reopen path. Source-body staging and all node phases
share bounded encoded-byte/call admission. No physical IO/free-space claim is made.

| Recorded attempt | Outcome |
| --- | --- |
| Export20889 / dependency34408 | Both compile checks TERMINAL0 |
| Real83075 | Two pass; third reached all receipt comparisons but producer expected17 legacy result kinds instead of16. Corrected to an explicit literal kind set (17 with clock), no engine change |
| Refusals74749 | Four real-journal cases pass, including legacy/clock physical metadata finish/reopen and18 re-pinned local/dependency cases; request mutations rebind existing event hashes |
| Max39341 | Six cases pass, including actual16384-command journal with live/in-flight work,128-byte operation IDs and bounded point queries; original cap still refuses another command |
| Focus80830 | All16 core and eight ledger archive cases pass, including source staging budget, independent operation literal/digest, five cleanup/kind/cycle mutations, full validation and maximal segment. Strict Clippy then flags six style issues; this was not a strict pass |
| Style62542 | Six style issues corrected without suppressions. Strict schema workspace/all-target Clippy and seven short archive cases TERMINAL0; maximal case already passed before style-only edits and remains in both full profiles |

The actual full segment is built through the real SQLite command API. It keeps a
live500-byte reservation and200-byte in-flight allocation throughout export and
requires no empty/quiescent state. Counting ports prove each selected original
receipt stays within256 reads/1MiB; a one-call budget makes exactly one provider
read, and an undersized first-read byte budget makes none. All records stay64KiB.
Retired inspection shares512 calls/4MiB. Full validation detects altered aggregate
request bytes and clock high. Wrong put acknowledgements, missing nodes and
corrupt bytes refuse. Source commands/accounting remain unchanged by export.

Both legacy and clock-enabled journals physically finish/drop/reopen actual
MetadataStore/MetadataReader. All17 typed result kinds (16 without clock), exact
original receipts and old summaries survive; pending probe admission is stripped
and in-flight work remains held. A peer commits inside the first node put and
forces conflict, proving no SQL cursor/transaction spans that IO. These are
synthetic local component facts, not provider/power-loss or current authority proof.

## Separate current closure

Implementation and focused verification pass for the stated archive candidate.
Scoped validation covers bounded original-segment preservation and admitted
read-only historical lookup. Trace: frozen c4280c6/wire7ac2c21 -> core historical
records -> actual original ledger outputs -> immutable source/state/index roots ->
physical reopen -> exact receipts/dependency refusals/full validation/point budgets.
All six producer findings have focused dispositions within this scope; pass_with_risk.
No independent review or human acceptance is claimed. Full profiles/default72751
and all40677 are LIVE in distinct external targets, followed by strict/500k/
standalone/fmt/fresh14-step artifact as applicable. Exact-source hosted checks and
source gate remain pending at this checkpoint; do not overlap/restart live targets.

Next activation review retains a critical race: a legacy writer may have passed
its file guard before activation but enter SQL afterward. A pointer/config change
alone is insufficient; mutation-authority fencing, final acknowledged prefix and
real paused-legacy-process evidence are required. This remains OPEN under
JOURNAL-CONTINUATION, alongside durable segment activation/recovery and lifetime
index composition. This archive does not implement pruning, .tig removal, expiry
or sustainable lifetime. Full product/course/operator/UI/Action/provider/current
authority/time/large-file and independent/human acceptance remain OPEN.

## Single-thread process-readiness fixture correction

Contract7a1ebe6 precedes a test-only correction in tests/fences.rs. Serial37691
completed the native-source suite157/1 ignored and later reached the11-entry fence
integration target, where its stdout READY parser blocked: single-thread libtest
prefixes the child line with its test name. Source inspection identified the exact
parent read/child write mismatch. The owned default runner9020 and its descendant
Cargo/test/helper processes were terminated; outer37691 is terminal1. It was not
restarted on an observation timeout. Old logs and prior native Budget failures
remain preserved; queued all-feature phase never started.

The fixture now sends a distinct readiness marker through stderr, separate from
libtest stdout, with8192-byte/16-line input and15s readiness bounds. Child lifetime
cleanup still kills/waits only its own process. Cross-process object exclusion,
shared readers, independent object progress, actual exit77 and old-root refusal
assertions remain unchanged. No production source/provider/ledger algorithm,
record/cap/deadline or operation behavior changes.

Focus25433 is TERMINAL0: all11 fence entries pass with RUST_TEST_THREADS=1 and4;
strict fence-target Clippy passes. This fixes the observed fixture deadlock; it
does not convert the interrupted whole run into a successful workspace pass.
New-source full profiles/500k/strict/standalone/fresh teaching artifact and hosted
qualification remain required. The c833add source itself passes exact43107903309
Windows/Linux/secrets. This correction needs its own exact-head CI.
