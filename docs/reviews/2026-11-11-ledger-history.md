# Original-result archive design review

EVID-128 design only; input9122ccff80f18dc79b8fafb3a58f52705ae78ac3.
[LEDGER-HISTORY](../../specs/LEDGER-HISTORY.md) names controls/scope and L0/L1/L2.
Method: the producer inspected actual ledger requests/outputs/execute/catch_up,
core private tokens, immutable Tree point lookup and parent checkpoint behavior.
All findings belong to Tig producer. These six lenses are not independent people,
publication authority or human acceptance. No implementation evidence yet.

| Lens / target | Finding / severity | Required disposition / evidence |
| --- | --- | --- |
| Identity / original event | High: rehashing only a summary loses exact request and original checkpoint | Canonical request, original sequence/previous/hash and typed result bound to exact ID/index path; literal and re-pinned mismatch vectors |
| Placement / admission | Critical: public historical probe decoder could forge a watermark and discard racing charges | Historical probe data has no conversion to live ObservationTicket; original replay only, new observation requires a newly issued handle-local probe |
| Custody / dependencies | High: response labels alone lose publication/retention/handoff/cleanup and retired allocation data | Preserve every typed original output and binding; exact old dependency inspection and cancelled/conflicting settlement cases |
| Concurrency / source export | High: archival node IO must not block peers or return a raced source as current | No SQL transaction spans metadata IO; compare source config/genesis/head, injected peer progress forces conflict |
| Provider/privacy / bounds | High: nested retired graphs or whole-lifetime output maps bypass limits | Existing Tree/NodeStore, bounded retired records, one shared export meter and separately bounded lazy point query; actual finish/reopen and counting-port evidence |
| Git/adoption / lifetime | High: archive could be mistaken for activated compaction or expiry | New additive schemas only; legacy bytes/caps unchanged. Explicit durable activation, original dependency lookup, old-writer/restore fences and expiry remain required |

Disposition: pass_design_with_risk for the stated archive scope. Exact wire fields
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
