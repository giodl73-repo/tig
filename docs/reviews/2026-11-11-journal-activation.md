# Journal activation design review

EVID-129 design only; inputc833add85f9a9c79d3387a78e469bc13a2570403.
[JOURNAL-ACTIVATION](../../specs/JOURNAL-ACTIVATION.md) names controls, full
required outcome, allowed fixture-only entry and L0/L1/L2 gates before code.
Method: producer inspected actual legacy guard/apply_once/catch_up and staged
rollback, current caps, original-history export/lookup and all six checklists.
These are producer lenses, not independent people or human acceptance.

| Lens / target | Finding / severity | Required disposition / evidence |
| --- | --- | --- |
| Identity / ordinary and activation minima | Critical: rotation may keep the same ordinary checkpoint; its minimum cannot detect pre-fence restore | Separate independently retained activation generation/digest minimum; preserve original authority/references, refuse missing/stale minima |
| Placement / restored accounting | Critical: an empty or summary-only successor loses active/in-flight obligations | Full working model/highs and exact typed original dependencies; fresh handle-local probe only, real nonempty multi-rotation fixtures |
| Custody / lifecycle | High: sealing alone could be advertised as permission to reclaim old metadata or bytes | No reclaim on prepare/activate, no seal-only public entry point; expiry and current provider/source/witness authority stay open |
| Concurrency / delayed legacy writer | Critical: old process can pass file guard then begin SQL after rotation | Transactional old events/meta write guards; actual pinned legacy Rust process paused at that exact boundary, older-held transaction and unrelated-domain progress |
| Provider/privacy / finite budgets | High: full journals may have no room for an unknown seal event; naive index editing can consume unbounded staging | Separate fixed bounded fence control row, no journal cap waiver; admitted physical metadata growth and bounded global index/tail proof |
| Git/adoption / proof identity | Critical: current-writer or Python model could be mislabeled legacy qualification | Pin original source/binary and any timing-only patch hashes; unmodified old-binary format refusal plus legacy acceptance, separate full activation/recovery gates |

Disposition: pass_design_with_risk. SQL/helper and active-control/index framing
must be frozen before their implementation. Initial private fence proof and full
activation have separate closure; neither is implemented by this design record.
All runtime/verification/validation/trace/full lifetime/expiry gates remain OPEN.
Parent PR59 full72751/40677 and exact43107903309 still live at entry; do not
restart or overlap those targets. Full PRODUCT_PLAN remains active.
