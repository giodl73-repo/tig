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
