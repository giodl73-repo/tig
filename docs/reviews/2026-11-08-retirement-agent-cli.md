# Retirement operator/agent interface review

Input 3e300ca0dea0c3e36944e2f6743ecdbc939dd89a;
implementation 19346c45930b575c1bbda5a7ee618fb4101d9d98.
WP-003/004/006/007/008/009/015; REQ-001/002/003/004/005/006/007/008/009;
SPEC-002/003/004/005/006/007/008/009/013;
IF-002/003/004/005/006/008/012/013; PKG-002/003/008, with PKG-009 CI qualification.
RUST/PROVIDER profiles. Scope: tig-storage retirement adapter, canonical library
plan serialization/status, synthetic process fixtures and qualification workflow.
Entry/exit and L0/L1/L2: [contract](../../specs/RETIREMENT-AGENT-CLI.md).

Method: one producer applies .roles/ROLE.md and every parliament checklist against
model/safety, source admission/store boundary/managed retirement contracts and
foundation T02/03/08/09/10/12/14/15/16/20/24/25. Owner for all findings is the Tig
maintainer through the named lens. This is neither independent nor human approval.

| Finding / lens | Severity, disposition and evidence obligation |
| --- | --- |
| RC-01 Identity | High: portable JSON must not become a live guard. Strict canonical plan parsing and actual enrollment/capacity checks precede execution. Source publication/object, action incarnation and immutable library bindings remain authoritative. New-operation restore and old-retry process fixtures check that a restored copy survives. |
| RC-02 Placement | High: a plan cannot grant itself a lower survivor floor or different domains/context. Private profiles construct requests and independently compare imported plans against exact grants, owner/context, domains, byte/peak limits and all namespace role keys. Every configured survivor is mandatory. Unknown mappings refuse. This fixed conservative policy is not a general lifecycle optimizer. |
| RC-03 Custody | High: keeper availability alone is insufficient. Apply reuses fresh library retained-domain verification; restore verifies an exact configured keeper grant before streaming. Reader/claim/missing-keeper process cases preserve bytes. Historical retirement and present integrity are distinct; no whole-volume reclaim is inferred. |
| RC-04 Recovery | High: status must not initialize/repair/admit work, or report acknowledged damage as authorized absence. It reads existing bound allocations and immutable intent/completion, reports missing settled evidence as recovery_required, and freshly verifies bytes separately. Review added lifecycle-allocation validation to reacquisition status and shared payload-shape validation. Exact source/keeper command counts, corruption, restored-prefix, restart and cancellation checks cover these branches. |
| RC-05 Provider/privacy | High: execution is disabled by default, roots are private absolute existing roots, unknown fields refuse, and errors expose static codes. Bounds precede IO. Existing provider path/contract checks remain, without broader hostile-FS/power-loss promises. No real caches, credentials or assets participate. |
| RC-06 Git/adoption | Medium: discovery links the new namespace while original copy commands stay compatible. Enabled eviction, cancellation and reacquisition are separate private flags. No automatic runner/workflow, catalogue action or consumer switch is installed. CI explicitly builds the binary and executes pending-cancellation qualification on both OS targets. |

Pending-cancellation is a deliberate cross-artifact L2 fixture: a real child exits
after durable intent, then actual CLI processes inspect/cancel/retry while keeper,
tree and namespace roots are offline. It is ignored in library-only suites because
the CLI artifact is not available there; the explicit command is required in CI
and local qualification. Never count that ordinary skipped entry as executed proof.

Disposition: pass_with_risk for this bounded explicit local interface, subject to
the separately recorded checks. Runtime verification, synthetic intended-use
rehearsal, trace and producer review close separately. Full requirements, all 110
designed workflows, independent review, live adoption and user acceptance remain
open. Next work remains remote/LFS upload and trusted Git event convergence,
followed by bulk final deletion and metadata retirement. Dynamic authority handoff,
large-file/native-provider qualification and journal continuation retain their gates.

Failure memory: the first new claim fixture hard-coded revision zero after
enrollment; the guarded journal correctly refused it as stale. It now reads the
actual revision before prepare/adopt. No runtime invariant was weakened.

EVID-089 records local default/all-feature workspace, strict Clippy, standalone
Git, fmt/L0 and executed process qualification separately from this review.
Hosted qualification and intended-use acceptance remain separate open gates.
