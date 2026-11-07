# T1b producer review: common delivery history

Public summary: consumer identifiers generalized; original notes retained privately.

Entry/scope/controls: [ASSET-DELIVERY-HISTORY](../../specs/ASSET-DELIVERY-HISTORY.md).
Targets: `tig-assets::delivery`, shared revision Budget/read-cache helpers, explicit
publication ReceiptChecker/ParsedBinding extraction, delivery/receipt integration
tests and the optional module export. Native types, status, payload selections,
provider operations and consumer project records remain unchanged.

Method: producer applies all six [role checklists](../../.roles/ROLE.md), inspecting
identity, bounded closure, desired/observed separation and old receipt API parity.
This is not independent or human creative approval.

| Finding / role | Severity | Disposition / owner and remaining work |
| --- | --- | --- |
| Identity: target key rename loses history or two aliases pretend independence | High | Exact declared target identity groups the frontier across key renames; duplicate identities within one policy refuse. Retargeting cannot inherit old claims. Named tests pass; real provider identity still adapter-owned |
| Identity/concurrency: a retry silently changes subject or authorization | High | Event key reuse conflicts; operation binds exact policy/key/identity/revision/representation/subject/profile. Separate real publications remain distinct. Idempotence and changed-gate-policy tests pass |
| Placement: every receipt/history dependency becomes hot-cache demand | High | Report separates checked metadata, receipt bytes, payloads and opaque witnesses; no claims adopted. T3 owns working/history placement and complete opaque closure interpretation |
| Custody: acknowledged/readback/absent labels become actual remote/custody proof | Critical | These remain retained claims only, explicit current-remote/semantic/eviction flags false. Absent observations remain opaque references. T2/provider adapters must establish genuine outcomes; no removal authority |
| Concurrency: latest timestamp/Git parent erases conflicting publications | High | Causal claim frontier preserves incomparable claims; attempted/failed descendants do not erase prior publication. Same-identity parent check; partial history stays unknown. Tests pass; remote conditional writes remain T2/adapters |
| Provider/privacy: new parser bypasses limits or leaks locators | High | Reuse existing receipt parser/row/selector engine, exact pins and per-binding limits. Combined revision/evidence budget and parsed-receipt reuse retain strict checking. New independent binding tests and existing35 publication tests pass; no raw provider text emitted |
| Git/adoption: receipt-specific asset records or ledger growth | High | Evidence bindings/mappings and ledgers remain immutable asset-side blobs, not new .tig asset types. Thousand-publication test keeps record bounded and reads one shared receipt once. Private mapping/consumer adoption still M1/T2 work |

Review tightened two details before qualification: operation identity includes its
exact gate policy, and target frontier groups by declared endpoint identity rather
than mutable policy labels. Reports expose a retained claim's original profile/
policy, so newer desired resolution cannot be mistaken for what was delivered.
Existing revision witness references join the opaque set; they do not disappear
from the consumer's remaining traversal obligations.

## Local evidence and limits

Seventeen delivery tests and five explicit receipt-binding tests pass. They cover
three domain shapes, independent targets/retained versions, causal union/conflict,
failed attempts, duplicate/retried events, immutable policies, key rename/retarget,
shared raw receipt rows, wrong subject/language/profile/target, strict schema and
missing/corrupt evidence, unregistered revisions and cumulative prefetch limits.
The 1,000-publication fixture uses2,006 metadata documents, one parsed receipt and
one binding; the local selected revision remains unchanged and Git size changes
by less than16 bytes. Existing35 publication and16 revision integration cases
also pass after extraction; package strict Clippy passes.

Receipt parsing/binding proves only the pinned evidence relationship, never remote
authentication/currentness, actual payload readback, rights, creative approval,
production freshness or independent custody. Full workspace/standalone/hosted
qualification is recorded separately; designed lifecycle/storage cases do not
become accepted just because these pure checks pass.

An additional existing-recipe integration test passes:
`delivery_history_updates_do_not_dirty_a_selected_payload_recipe_but_new_cue_bytes_do`.
Under explicit selected-payload admission, adding/updating delivery streams changes
binding provenance without changing the recipe; replacing recorded cue bytes does
change it. This is not a global exclusion of history from every producer's inputs:
each consumer must still admit its actual consumed fields/slots. The declaration
and delivery reader tests address their own separate contracts.

Refresh only the existing lib export, publication source and recipe-test review
pins for the actual changes; do not alter unrelated targets or size guards.
Build outputs/logs remain outside Git. Local D build writes hit insufficient disk
space; the bounded replacement target is on F with debug symbols/incremental
disabled for these invocations. No asset-cache cleanup or binding change follows.

Next: T2 explicit operations over these contracts using existing sparse proposal
and publication seams; then T3 placement/custody integration and actual consumer project M1.
